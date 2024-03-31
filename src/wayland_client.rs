use std::collections::HashMap;
use std::ffi::CString;
use std::fmt;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsRawFd, BorrowedFd, FromRawFd, OwnedFd, RawFd};
use std::process::exit;
use std::sync::{Arc, Mutex};

use android_hardware_graphics_common::aidl::android::hardware::graphics::common::PixelFormat::PixelFormat;
use nix::libc::{EAGAIN, POLLERR, POLLNVAL};
use tokio::io::unix::AsyncFd;
use tokio::io::{Empty, Interest};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex as AsyncMutex;
use tokio::task::JoinSet;
use wayrs_client::core::ObjectId;
use wayrs_client::global::GlobalsExt;
use wayrs_client::object::Proxy;
use wayrs_client::protocol::{
    wl_output, wl_subcompositor, WlBuffer, WlSubcompositor, WlSubsurface,
};
use wayrs_client::ClientTransport;
use wayrs_client::{
    protocol::{WlCompositor, WlSurface},
    Connection, EventCtx,
};
use wayrs_client_transport_virtgpu::VirtgpuChannel;
use wayrs_egl::Fourcc;
use wayrs_protocols::linux_dmabuf_unstable_v1::zwp_linux_buffer_params_v1;
use wayrs_protocols::single_pixel_buffer_v1::WpSinglePixelBufferManagerV1;
use wayrs_protocols::viewporter::{WpViewport, WpViewporter};
use wayrs_protocols::{
    linux_dmabuf_unstable_v1::{
        zwp_linux_dmabuf_feedback_v1, ZwpLinuxDmabufFeedbackV1, ZwpLinuxDmabufV1,
    },
    xdg_shell::{xdg_surface, xdg_toplevel, xdg_wm_base, XdgSurface, XdgToplevel, XdgWmBase},
};
use wayrs_utils::dmabuf_feedback::{DmabufFeedback, DmabufFeedbackHandler};

use crate::bindings::sw_sync_timeline_inc;
use crate::composer_client::{
    ClientBuffer, ClientRect, ComposerClientState, HwcEvent, LayerUpdatePayload,
};

type WaylandChannel = VirtgpuChannel;

pub async fn start_wayland_client(
    mut rx: Receiver<HwcEvent>,
    composer_client_state: Arc<Mutex<ComposerClientState>>,
) {
    println!("starting wayland client");

    let (mut conn, globals) =
        Connection::<WaylandState, WaylandChannel>::async_connect_and_collect_globals()
            .await
            .unwrap();
    let linux_dmabuf: ZwpLinuxDmabufV1 = globals.bind(&mut conn, 4).unwrap();
    let wl_compositor: WlCompositor = globals.bind(&mut conn, ..=6).unwrap();
    let wl_subcompositor: WlSubcompositor = globals.bind(&mut conn, 1).unwrap();
    let wp_single_pixel_buffer_manager_v1: WpSinglePixelBufferManagerV1 =
        globals.bind(&mut conn, 1).unwrap();
    let xdg_wm_base: XdgWmBase = globals
        .bind_with_cb(&mut conn, ..=4, xdg_wm_base_cb)
        .unwrap();
    let wp_viewporter: WpViewporter = globals.bind(&mut conn, 1).unwrap();

    let mut state = WaylandState {
        composer_client_state,
        linux_dmabuf,
        wl_compositor,
        wl_subcompositor,
        xdg_wm_base,
        wp_single_pixel_buffer_manager_v1,
        wp_viewporter,
        windows: HashMap::new(),
        display_to_window: HashMap::new(),
    };

    loop {
        tokio::select!(
            _ = conn.async_recv_events() => {
                //println!("recv_events done");
                conn.dispatch_events(&mut state);
                //println!("disptach_events done");
                conn.async_flush().await.unwrap();
                //println!("flushed");
            },
            evt = rx.recv() => {
                if evt.is_none() {
                    continue;
                }
                state.process_hwc_events(&mut conn, evt.unwrap()).await;
                conn.dispatch_events(&mut state);
                //println!("disptach_events done");
                conn.async_flush().await.unwrap();
                //println!("flushed");
            },
        );
        //conn.flush(IoMode::Blocking).unwrap();
        //conn.recv_events(IoMode::Blocking).unwrap();
    }
}

struct WaylandState {
    composer_client_state: Arc<Mutex<ComposerClientState>>,
    linux_dmabuf: ZwpLinuxDmabufV1,
    wl_compositor: WlCompositor,
    wl_subcompositor: WlSubcompositor,
    xdg_wm_base: XdgWmBase,
    wp_single_pixel_buffer_manager_v1: WpSinglePixelBufferManagerV1,
    wp_viewporter: WpViewporter,
    windows: HashMap<u32, WaylandWindow>,
    display_to_window: HashMap<i64, u32>,
}

impl WaylandState {
    async fn process_hwc_events(
        &mut self,
        conn: &mut Connection<WaylandState, WaylandChannel>,
        evt: HwcEvent,
    ) {
        println!("{:?}", evt);
        match evt {
            HwcEvent::CreateDisplay { display } => {
                let window = WaylandWindow::new(
                    conn,
                    self.composer_client_state.clone(),
                    self.wl_compositor,
                    self.wl_subcompositor,
                    self.xdg_wm_base,
                    self.linux_dmabuf,
                    self.wp_single_pixel_buffer_manager_v1,
                    self.wp_viewporter,
                    display,
                );
                let window_id = window.xdg_surface.id().as_u32();
                self.display_to_window.insert(display, window_id);
                self.windows.insert(window_id, window);
            }
            HwcEvent::PresentDisplay {display} => {
                let window_id = self.display_to_window.get(&display).unwrap();
                let window = self.windows.get_mut(window_id).unwrap();
                window.present_display(conn);
            }
            HwcEvent::CreateLayer { display, layer } => {
                let window_id = self.display_to_window.get(&display).unwrap();
                let window = self.windows.get_mut(window_id).unwrap();
                window.create_layer(conn, layer);
            }
            HwcEvent::DestroyLayer { display, layer } => {
                let window_id = self.display_to_window.get_mut(&display).unwrap();
                let window = self.windows.get_mut(window_id).unwrap();
                window.destory_layer(conn, layer);
            }
            HwcEvent::PresentLayer { display, layer } => {
                let window_id = self.display_to_window.get_mut(&display).unwrap();
                let window = self.windows.get_mut(window_id).unwrap();
                window
                    .subsurfaces
                    .get_mut(&layer)
                    .unwrap()
                    .present_layer(conn)
                    .await;
            }
        }
    }
}

fn xdg_wm_base_cb(ctx: EventCtx<WaylandState, XdgWmBase, WaylandChannel>) {
    if let xdg_wm_base::Event::Ping(serial) = ctx.event {
        ctx.proxy.pong(ctx.conn, serial);
    }
}

#[derive(Debug)]
pub struct WaylandWindow {
    composer_client_state: Arc<Mutex<ComposerClientState>>,
    surface: WlSurface,
    wl_compositor: WlCompositor,
    wl_subcompositor: WlSubcompositor,
    xdg_surface: XdgSurface,
    xdg_toplevel: XdgToplevel,
    linux_dmabuf: ZwpLinuxDmabufV1,
    wp_viewporter: WpViewporter,
    should_close: bool,
    buffer: WlBuffer,
    configured: bool,
    subsurfaces: HashMap<i64, WaylandSubSurface>,
}

impl WaylandWindow {
    fn new(
        conn: &mut Connection<WaylandState, WaylandChannel>,
        composer_client_state: Arc<Mutex<ComposerClientState>>,
        wl_compositor: WlCompositor,
        wl_subcompositor: WlSubcompositor,
        xdg_wm_base: XdgWmBase,
        linux_dmabuf: ZwpLinuxDmabufV1,
        wp_single_pixel_buffer_manager_v1: WpSinglePixelBufferManagerV1,
        wp_viewporter: WpViewporter,
        id: i64,
    ) -> Self {
        let surface = wl_compositor.create_surface(conn);
        let xdg_surface = xdg_wm_base.get_xdg_surface(conn, surface);
        let xdg_toplevel = xdg_surface.get_toplevel(conn);

        // DMABUFs have origin at top-left corner, but OpenGL has origin at bottom-left. This
        // results in a y-flipped image.
        //surface.set_buffer_transform(conn, wl_output::Transform::_180);

        conn.set_callback_for(xdg_surface, |ctx| {
            if let xdg_surface::Event::Configure(serial) = ctx.event {
                ctx.state
                    .windows
                    .get_mut(&ctx.proxy.id().as_u32())
                    .and_then(|x| Some(x.configured = true));
                ctx.proxy.ack_configure(ctx.conn, serial);
            }
        });

        conn.set_callback_for(xdg_toplevel, |ctx| match ctx.event {
            xdg_toplevel::Event::Configure(args) => {
                if args.width != 0 {
                    //ctx.state.surf.width = args.width.try_into().unwrap();
                }
                if args.height != 0 {
                    //ctx.state.surf.height = args.height.try_into().unwrap();
                }
            }
            xdg_toplevel::Event::Close => {
                //ctx.state.surf.should_close = true;
                ctx.conn.break_dispatch_loop();
            }
            _ => (),
        });

        xdg_toplevel.set_app_id(conn, wayrs_client::cstr!("vmdroid").into());
        let id = format!("vmdroid-{id}");
        xdg_toplevel.set_title(conn, CString::new(id).unwrap());

        let buffer = wp_single_pixel_buffer_manager_v1.create_u32_rgba_buffer(conn, 0, 0, 0, 0);

        surface.commit(conn);

        Self {
            composer_client_state,
            surface,
            wl_compositor,
            wl_subcompositor,
            xdg_surface,
            xdg_toplevel,
            linux_dmabuf,
            wp_viewporter,
            should_close: false,
            buffer,
            configured: false,
            subsurfaces: HashMap::new(),
        }
    }
    fn create_layer(&mut self, conn: &mut Connection<WaylandState, WaylandChannel>, layer: i64) {
        let surface_for_subsurface = self.wl_compositor.create_surface(conn);
        let viewport = self
            .wp_viewporter
            .get_viewport(conn, surface_for_subsurface);
        let subsurface = WaylandSubSurface::new(
            conn,
            self.composer_client_state.clone(),
            self.wl_subcompositor,
            self.linux_dmabuf,
            self.surface,
            surface_for_subsurface,
            viewport,
            layer,
        );
        self.subsurfaces.insert(layer, subsurface);
    }

    fn destory_layer(&mut self, conn: &mut Connection<WaylandState, WaylandChannel>, layer: i64) {
        let subsurface = self.subsurfaces.remove(&layer).unwrap();
        subsurface.subsurface.destroy(conn);
        subsurface.surface.destroy(conn);
    }

    fn get_layer(&mut self, id: i64) -> &mut WaylandSubSurface {
        self.subsurfaces.get_mut(&id).unwrap()
    }

    fn present_display(&mut self, conn: &mut Connection<WaylandState, WaylandChannel>) {
        if self.configured {
            self.surface.attach(conn, Some(self.buffer), 0, 0);
            self.subsurfaces
                .iter_mut()
                .for_each(|(_, x)| x.surface.commit(conn));
            self.surface.commit(conn);
        }
    }

    fn get_parent_for_z(&self, z: i32) -> WlSurface {
        let mut parent = self.surface;

        for (_, surf) in self.subsurfaces.iter() {
            if surf.z == z - 1 {
                parent = surf.surface
            }
        }
        parent
    }
}

#[derive(Debug)]
struct WaylandSubSurface {
    composer_client_state: Arc<Mutex<ComposerClientState>>,
    surface: WlSurface,
    subsurface: WlSubsurface,
    linux_dmabuf: ZwpLinuxDmabufV1,
    viewport: WpViewport,
    buffer: Option<WlBuffer>,
    z: i32,
    id: i64,
}

impl WaylandSubSurface {
    fn new(
        conn: &mut Connection<WaylandState, WaylandChannel>,
        composer_client_state: Arc<Mutex<ComposerClientState>>,
        wl_subcompositor: WlSubcompositor,
        linux_dmabuf: ZwpLinuxDmabufV1,
        window: WlSurface,
        surface: WlSurface,
        viewport: WpViewport,
        id: i64,
    ) -> Self {
        let subsurface = wl_subcompositor.get_subsurface(conn, surface, window);
        subsurface.place_above(conn, window);
        subsurface.set_desync(conn);
        Self {
            composer_client_state,
            surface,
            subsurface,
            linux_dmabuf,
            viewport,
            id,
            buffer: None,
            z: 0,
        }
    }
    async fn present_layer(&mut self, conn: &mut Connection<WaylandState, WaylandChannel>) {
        println!("waiting for fence");
        let fence = {
            let mut state = self.composer_client_state.lock().unwrap();
            let layer = &mut state.layers.get_mut(&(self.id)).unwrap();
            let current_slot = layer.slot;
            let buffer = &mut layer.buffers[current_slot as usize];
            if buffer.is_none() {
                return;
            }
            buffer.as_mut().unwrap().fence.take()
        };

        //this could be a race condition, may need to convert composer_client_state to AsyncMutex
        if fence.is_some() {
            let async_fd = AsyncFd::with_interest(fence.unwrap(), Interest::READABLE).unwrap();
            let mut read_gaurd = async_fd.readable().await.unwrap();
            //read_gaurd.retain_ready();
            read_gaurd.clear_ready();
            //sync_wait(client_buffer.fence.unwrap());
            //let ret = unsafe {crate::bindings::sync_wait(client_buffer.fence.unwrap().as_raw_fd(), 3000)};
            //println!("sync_wait ret: {}", ret);
        }
        println!("got fence");

        let mut state = self.composer_client_state.lock().unwrap();
        let layer = &mut state.layers.get_mut(&(self.id)).unwrap();
        let current_slot = layer.slot;
        let buffer = &layer.buffers[current_slot as usize].as_ref().unwrap();

        //let fourcc = pixel_format_to_drm_format(client_buffer.ints[4]);
        //println!("format: {}, fourcc: {:?}", client_buffer.ints[4], fourcc);
        let wl_buffer_params = self.linux_dmabuf.create_params(conn);

        let extra_fd_len = DRV_MAX_FDS - buffer.fds.len();
        if extra_fd_len == DRV_MAX_FDS {
            println!("{:#?}", layer);
        }
        let gralloc_handle_ptr = &buffer.ints[extra_fd_len..];
        let (head, body, _tail) = unsafe { gralloc_handle_ptr.align_to::<CrosGrallocHandle>() };
        //println!("elen: {}, ptr: {:?}, {:?}, {:?}, {:?}",extra_fd_len, gralloc_handle_ptr, head, body, _tail);
        assert!(head.is_empty(), "Data was not alligned");
        let gralloc_handle = &body[0];

        println!("{:?}", gralloc_handle);
        let width = gralloc_handle.width;
        let height = gralloc_handle.height;
        //let width = client_buffer.ints[16];
        //let height = client_buffer.ints[17];

        for (i, fd) in buffer
            .fds
            .iter()
            .take(gralloc_handle.num_planes as usize)
            .enumerate()
        {
            //let (_offset, stride, modifier) = conn
            //    .transport_mut()
            //    .fix_metadata(i, width as u32, height as u32, gralloc_handle.format)
            //    .unwrap();
            wl_buffer_params.add(
                conn,
                fd.try_clone().unwrap(),
                i as u32,
                0,
                gralloc_handle.strides[i],
                (gralloc_handle.format_modifier >> 32) as u32,
                (gralloc_handle.format_modifier & 0xFFFF_FFFF) as u32,
            );
        }

        let wl_buffer = wl_buffer_params.create_immed(
            conn,
            width as i32,
            height as i32,
            //fourcc.0,
            gralloc_handle.format,
            //client_buffer.ints[18] as u32,
            zwp_linux_buffer_params_v1::Flags::empty(),
        );
        wl_buffer_params.destroy(conn);

        conn.set_callback_for(wl_buffer, |_ctx| {
            println!("signaling the fence");
            //unsafe { sw_sync_timeline_inc(ctx.state.sync_timeline.as_raw_fd(), 1) };
        });
        self.surface.attach(conn, Some(wl_buffer), 0, 0);
        self.surface.damage(conn, 0, 0, i32::MAX, i32::MAX);
        self.buffer.replace(wl_buffer);

        let display_frame = layer.display_frame.as_ref().unwrap();
        let source_crop = layer.source_crop.as_ref().unwrap();
        self.subsurface
            .set_position(conn, display_frame.left, display_frame.top);
        self.viewport.set_source(
            conn,
            source_crop.left.into(),
            source_crop.top.into(),
            (source_crop.right - source_crop.left).into(),
            (source_crop.bottom - source_crop.top).into(),
        );
        self.viewport.set_destination(
            conn,
            display_frame.right - display_frame.left,
            display_frame.bottom - display_frame.top,
        );
    }
}

const DRV_MAX_PLANES: usize = 4;
const DRV_MAX_FDS: usize = DRV_MAX_PLANES + 1;

#[repr(C, packed)]
#[derive(Debug)]
struct CrosGrallocHandle {
    strides: [u32; DRV_MAX_PLANES],
    offsets: [u32; DRV_MAX_PLANES],
    sizes: [u32; DRV_MAX_PLANES],
    id: u32,
    width: u32,
    height: u32,
    format: u32, /* DRM format */
    tiling: u32,
    format_modifier: u64,
    use_flags: u64, /* Buffer creation flags */
    magic: u32,
    pixel_stride: u32,
    droid_format: u32,
    usage: i64, /* Android usage. */
    num_planes: u32,
    reserved_region_size: u64,
    total_size: u64, /* Total allocation size */
}
