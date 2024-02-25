use async_trait::async_trait;
use binder_tokio::*;
use std::{borrow::Borrow, io, string};
use tokio::task;
use wayrs_client::{
    self,
    global::GlobalsExt,
    protocol::{wl_shm, WlCompositor, WlSurface},
    EventCtx,
};
use wayrs_protocols::xdg_shell::{
    xdg_surface, xdg_toplevel, xdg_wm_base, XdgSurface, XdgToplevel, XdgWmBase,
};
use wayrs_utils::shm_alloc::{self, ShmAlloc};

#[allow(non_snake_case)]
use android_hardware_graphics_composer3::aidl::android::hardware::graphics::composer3::IComposer;
use android_hardware_graphics_composer3::aidl::android::hardware::graphics::composer3::{
    Capability,
    IComposer::{BnComposer, IComposerAsync, IComposerAsyncServer},
    IComposerClient,
};
use binder;

pub struct WaylandHwc3Service;

impl binder::Interface for WaylandHwc3Service {}

#[async_trait]
impl IComposerAsyncServer for WaylandHwc3Service {
    async fn r#createClient(
        &self,
    ) -> binder::Result<binder::Strong<dyn IComposerClient::IComposerClient>> {
        panic!("not implimented createClient")
    }

    async fn r#getCapabilities(&self) -> binder::Result<Vec<Capability::Capability>> {
        panic!("not implimented getCapabilities")
    }
}

const SERVICE_IDENTIFIER: &str = "android.hardware.graphics.composer3.IComposer/default";

async fn start_hwc3() {
    binder::ProcessState::start_thread_pool();

    let mut _a = String::new();
    io::stdin().read_line(&mut _a);

    let wayland_hwc3_service = WaylandHwc3Service;
    let wayland_hwc3_service_binder = BnComposer::new_async_binder(
        wayland_hwc3_service,
        TokioRuntime(tokio::runtime::Handle::current()),
        binder::BinderFeatures::default(),
    );

    binder::add_service(SERVICE_IDENTIFIER, wayland_hwc3_service_binder.as_binder())
        .expect("Failed to register wayland hwc3 service");

    task::block_in_place(move || {
        binder::ProcessState::join_thread_pool();
    });
}

struct WaylandClientState {
}

async fn start_wayland_client() {
    println!("starting wayland client");

    let (mut conn, globals) =
        wayrs_client::Connection::<WaylandClientState>::async_connect_and_collect_globals()
            .await
            .expect("can't connect to wayland socket");

    let wl_compositor: WlCompositor = globals.bind(&mut conn, 1..=5).expect("no compositor api");

    let mut shm_alloc = ShmAlloc::bind(&mut conn, &globals).expect("ShmAlloc error");

    let shm = shm_alloc
        .alloc_buffer(
            &mut conn,
            shm_alloc::BufferSpec {
                width: 100,
                height: 200,
                stride: 100 * 4,
                format: wl_shm::Format::Argb8888,
            },
        )
        .expect("can't allocate shm");
    let xdg_wm_base: XdgWmBase = globals
        .bind_with_cb(&mut conn, ..=4, xdg_wm_base_cb)
        .unwrap();

    let wl_surface = wl_compositor.create_surface(&mut conn);
    let xdg_surface = xdg_wm_base.get_xdg_surface(&mut conn, wl_surface);
    let xdg_toplevel = xdg_surface.get_toplevel(&mut conn);

    conn.set_callback_for(xdg_surface, |ctx| {
        if let xdg_surface::Event::Configure(serial) = ctx.event {
            ctx.proxy.ack_configure(ctx.conn, serial);
        }
    });

    conn.set_callback_for(xdg_toplevel, |ctx| match ctx.event {
        xdg_toplevel::Event::Configure(args) => {
            //todo
        }
        xdg_toplevel::Event::Close => {
            ctx.conn.break_dispatch_loop();
        }
        _ => (),
    });

    xdg_toplevel.set_app_id(&mut conn, wayrs_client::cstr!("test").into());
    xdg_toplevel.set_title(&mut conn, wayrs_client::cstr!("TEST").into());
    
    for b in shm.1.iter_mut() {
        *b = 255;
    }
    
    wl_surface.attach(&mut conn, Some(shm.0.into_wl_buffer()), 0, 0);
    wl_surface.damage(&mut conn, 0, 0, i32::MAX, i32::MAX);

    wl_surface.commit(&mut conn);

    let mut wcstate = WaylandClientState {
    };

    loop {
        conn.async_flush().await.expect("can't flush");
        conn.async_recv_events().await.expect("can't recv events");
        conn.dispatch_events(&mut wcstate)
    }
}

fn xdg_wm_base_cb(ctx: wayrs_client::EventCtx<WaylandClientState, XdgWmBase>) {
    if let xdg_wm_base::Event::Ping(serial) = ctx.event {
        ctx.proxy.pong(ctx.conn, serial);
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    println!("starting wayland_hwc3 {}", std::process::id());
    //start_hwc3().await;
    start_wayland_client().await;
}
