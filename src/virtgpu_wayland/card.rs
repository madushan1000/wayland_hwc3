use std::{
    collections::{HashMap, VecDeque},
    ffi::{c_void, OsString},
    io::{self, ErrorKind, Read, Write},
    mem::{size_of, size_of_val},
    os::{
        fd::{AsRawFd, FromRawFd, OwnedFd},
        unix::{ffi::OsStringExt, fs::OpenOptionsExt},
    },
    ptr::null_mut,
    slice,
};

use nix::{
    ioctl_readwrite, ioctl_write_ptr,
    libc::{EAGAIN, O_CLOEXEC, O_RDWR},
    sys::mman::{MapFlags, ProtFlags},
};
use wayrs_client::IoMode;

use crate::virtgpu_wayland::card::bindings::{
    CrossDomainCapabilities, CrossDomainHeader, CrossDomainImageRequirements,
    CROSS_DOMAIN_CHANNEL_TYPE_WAYLAND, CROSS_DOMAIN_CMD_READ, CROSS_DOMAIN_CMD_RECEIVE,
    CROSS_DOMAIN_ID_TYPE_WRITE_PIPE, CROSS_DOMAIN_QUERY_RING, DRM_COMMAND_BASE,
    DRM_VIRTGPU_CONTEXT_INIT, DRM_VIRTGPU_EXECBUFFER, DRM_VIRTGPU_GET_CAPS, DRM_VIRTGPU_MAP,
    DRM_VIRTGPU_RESOURCE_CREATE_BLOB, DRM_VIRTGPU_RESOURCE_INFO, DRM_VIRTGPU_WAIT,
    VIRTGPU_EVENT_FENCE_SIGNALED,
};

pub use self::bindings::drm_version;
use self::bindings::{
    drm_event, drm_gem_close, drm_prime_handle, drm_virtgpu_3d_wait, drm_virtgpu_context_init,
    drm_virtgpu_context_set_param, drm_virtgpu_execbuffer, drm_virtgpu_get_caps, drm_virtgpu_map,
    drm_virtgpu_resource_create_blob, drm_virtgpu_resource_info, CrossDomainGetImageRequirements,
    CrossDomainInit, CrossDomainPoll, CrossDomainSendReceive, CROSS_DOMAIN_CHANNEL_RING,
    CROSS_DOMAIN_CMD_GET_IMAGE_REQUIREMENTS, CROSS_DOMAIN_CMD_INIT, CROSS_DOMAIN_CMD_POLL,
    CROSS_DOMAIN_CMD_SEND, CROSS_DOMAIN_ID_TYPE_VIRTGPU_BLOB, CROSS_DOMAIN_RING_NONE,
    DRM_IOCTL_BASE, VIRTGPU_BLOB_FLAG_USE_MAPPABLE, VIRTGPU_BLOB_FLAG_USE_SHAREABLE,
    VIRTGPU_BLOB_MEM_GUEST, VIRTGPU_BLOB_MEM_HOST3D, VIRTGPU_CONTEXT_PARAM_CAPSET_ID,
    VIRTGPU_CONTEXT_PARAM_NUM_RINGS, VIRTGPU_CONTEXT_PARAM_POLL_RINGS_MASK,
    VIRTGPU_EXECBUF_RING_IDX,
};

#[allow(non_camel_case_types, non_snake_case, unused)]
mod bindings;

#[allow(unused)]
const VIRTIO_GPU_CAPSET_VIRGL: u32 = 1;
#[allow(unused)]
const VIRTIO_GPU_CAPSET_VIRGL2: u32 = 2;
#[allow(unused)]
const VIRTIO_GPU_CAPSET_GFXSTREAM: u32 = 3;
#[allow(unused)]
const VIRTIO_GPU_CAPSET_VENUS: u32 = 4;
const VIRTIO_GPU_CAPSET_CROSS_DOMAIN: u32 = 5;

#[allow(unused)]
const RUTABAGA_GRALLOC_USE_SCANOUT: u32 = 1 << 0;
#[allow(unused)]
const RUTABAGA_GRALLOC_USE_RENDERING: u32 = 1 << 2;
#[allow(unused)]
const RUTABAGA_GRALLOC_USE_LINEAR: u32 = 1 << 4;
const RUTABAGA_GRALLOC_USE_TEXTURING: u32 = 1 << 5;
#[allow(unused)]
const RUTABAGA_GRALLOC_USE_CAMERA_WRITE: u32 = 1 << 6;
#[allow(unused)]
const RUTABAGA_GRALLOC_USE_CAMERA_READ: u32 = 1 << 7;
#[allow(unused)]
const RUTABAGA_GRALLOC_USE_PROTECTED: u32 = 1 << 8;
#[allow(unused)]
const RUTABAGA_GRALLOC_USE_SW_READ_OFTEN: u32 = 1 << 9;
#[allow(unused)]
const RUTABAGA_GRALLOC_USE_SW_WRITE_OFTEN: u32 = 1 << 11;

ioctl_readwrite!(drm_get_version, DRM_IOCTL_BASE, 0x00, drm_version);
ioctl_readwrite!(
    drm_prime_fd_to_handle,
    DRM_IOCTL_BASE,
    0x2e,
    drm_prime_handle
);
ioctl_readwrite!(
    drm_prime_handle_to_fd,
    DRM_IOCTL_BASE,
    0x2d,
    drm_prime_handle
);
ioctl_write_ptr!(drm_gem_close, DRM_IOCTL_BASE, 0x09, drm_gem_close);
ioctl_readwrite!(
    virtgpu_context_init,
    DRM_IOCTL_BASE,
    DRM_COMMAND_BASE + DRM_VIRTGPU_CONTEXT_INIT,
    drm_virtgpu_context_init
);
ioctl_readwrite!(
    virtgpu_resource_create_blob,
    DRM_IOCTL_BASE,
    DRM_COMMAND_BASE + DRM_VIRTGPU_RESOURCE_CREATE_BLOB,
    drm_virtgpu_resource_create_blob
);
ioctl_readwrite!(
    virtgpu_map,
    DRM_IOCTL_BASE,
    DRM_COMMAND_BASE + DRM_VIRTGPU_MAP,
    drm_virtgpu_map
);

ioctl_readwrite!(
    virtgpu_execbuffer,
    DRM_IOCTL_BASE,
    DRM_COMMAND_BASE + DRM_VIRTGPU_EXECBUFFER,
    drm_virtgpu_execbuffer
);

ioctl_readwrite!(
    virtgpu_wait,
    DRM_IOCTL_BASE,
    DRM_COMMAND_BASE + DRM_VIRTGPU_WAIT,
    drm_virtgpu_3d_wait
);

ioctl_readwrite!(
    virtgpu_resource_info,
    DRM_IOCTL_BASE,
    DRM_COMMAND_BASE + DRM_VIRTGPU_RESOURCE_INFO,
    drm_virtgpu_resource_info
);

ioctl_readwrite!(
    virtgpu_get_caps,
    DRM_IOCTL_BASE,
    DRM_COMMAND_BASE + DRM_VIRTGPU_GET_CAPS,
    drm_virtgpu_get_caps
);

trait CrossDomainCmd {
    fn size(&self) -> usize {
        size_of_val(self)
    }
}

impl CrossDomainCmd for CrossDomainInit {}
impl CrossDomainCmd for CrossDomainPoll {}
impl CrossDomainCmd for CrossDomainSendReceive {}
impl CrossDomainCmd for CrossDomainGetImageRequirements {}

pub struct Card {
    pub card_fd: std::fs::File,
    channel_ring_addr: *mut c_void,
    channel_ring_handle: u32,
    query_ring_addr: *mut c_void,
    query_ring_handle: u32,
    image_query_cache: HashMap<(u32, u32, u32), CrossDomainImageRequirements>,
}

impl Card {
    pub fn open(path: &str) -> Self {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        options.write(true);
        options.custom_flags(nix::libc::O_NONBLOCK);
        let file = options.open(path).unwrap();

        Card {
            card_fd: file,
            channel_ring_addr: null_mut(),
            channel_ring_handle: 0,
            query_ring_addr: null_mut(),
            query_ring_handle: 0,
            image_query_cache: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> OsString {
        let mut data = drm_version::default();
        unsafe { drm_get_version(self.card_fd.as_raw_fd(), &mut data) }.unwrap();

        let mut name_buf: Vec<i8> = Vec::with_capacity(data.name_len as usize);

        let mut new_data = drm_version {
            name_len: data.name_len,
            name: name_buf.as_mut_ptr(),
            ..Default::default()
        };

        unsafe { drm_get_version(self.card_fd.as_raw_fd(), &mut new_data) }.unwrap();

        unsafe {
            name_buf.set_len(data.name_len as usize);
        }
        OsString::from_vec(unsafe { transmute_vec(name_buf) })
    }

    pub fn fix_metadata(
        &mut self,
        plane_idx: usize,
        width: u32,
        height: u32,
        format: u32,
    ) -> Option<(u32, u32, u64)> {
        println!("width: {}, height: {}, format: {}", width, height, format);
        if let Some(val) = self.image_query_cache.get(&(width, height, format)) {
            return Some((val.offsets[plane_idx], val.strides[plane_idx], val.modifier));
        }

        let mut cmd_get_reqs: CrossDomainGetImageRequirements = Default::default();
        cmd_get_reqs.hdr.cmd = CROSS_DOMAIN_CMD_GET_IMAGE_REQUIREMENTS as u8;
        cmd_get_reqs.hdr.cmd_size = size_of::<CrossDomainGetImageRequirements>() as u16;
        cmd_get_reqs.width = width;
        cmd_get_reqs.height = height;
        cmd_get_reqs.drm_format = format;
        cmd_get_reqs.flags = RUTABAGA_GRALLOC_USE_TEXTURING;

        self.submit_cmd(
            &cmd_get_reqs,
            &vec![],
            CROSS_DOMAIN_QUERY_RING,
            self.query_ring_handle,
            true,
        );

        let response: &CrossDomainImageRequirements = unsafe {
            (self.query_ring_addr as *const CrossDomainImageRequirements)
                .as_ref()
                .unwrap()
        };

        println!("{:?}", response);

        self.image_query_cache
            .insert((width, height, format), response.clone());

        //Have to allocate a host blob when image query is done. see
        //rutabaga_gfx/src/rutabaga_gralloc/minigbm.rs:get_image_memory_requirements()
        //so just allocate it and discard, maybe there is a way to destroy it?
        self.create_host_blob(response.blob_id, response.size, &mut 0);

        Some((
            response.offsets[plane_idx],
            response.strides[plane_idx],
            response.modifier,
        ))
    }

    pub fn create_context(&mut self) {
        let mut cross_domain_caps: CrossDomainCapabilities = Default::default();
        let mut args: drm_virtgpu_get_caps = Default::default();
        args.cap_set_id = VIRTIO_GPU_CAPSET_CROSS_DOMAIN;
        args.size = size_of::<CrossDomainCapabilities>() as u32;
        args.addr = &mut cross_domain_caps as *const _ as u64;

        let _ret = unsafe { virtgpu_get_caps(self.card_fd.as_raw_fd(), &mut args) }.unwrap();
        println!("{:?}", cross_domain_caps);

        let mut init: drm_virtgpu_context_init = Default::default();
        let mut ctx_set_params: [drm_virtgpu_context_set_param; 3] = Default::default();

        ctx_set_params[0].param = VIRTGPU_CONTEXT_PARAM_CAPSET_ID as u64;
        ctx_set_params[0].value = VIRTIO_GPU_CAPSET_CROSS_DOMAIN as u64;
        ctx_set_params[1].param = VIRTGPU_CONTEXT_PARAM_NUM_RINGS as u64;
        ctx_set_params[1].value = 2;
        ctx_set_params[2].param = VIRTGPU_CONTEXT_PARAM_POLL_RINGS_MASK as u64;
        ctx_set_params[2].value = 1 << CROSS_DOMAIN_CHANNEL_RING;

        init.ctx_set_params = ctx_set_params.as_mut_ptr() as u64;
        init.num_params = 3;

        let _ret = unsafe { virtgpu_context_init(self.card_fd.as_raw_fd(), &mut init) }.unwrap();

        let (query_ring_handle, query_ring_resource_id, query_ring_addr) = self.create_ring();
        let (channel_ring_handle, channel_ring_resource_id, channel_ring_addr) = self.create_ring();

        self.query_ring_addr = query_ring_addr;
        self.query_ring_handle = query_ring_handle;
        self.channel_ring_addr = channel_ring_addr;
        self.channel_ring_handle = channel_ring_handle;

        let mut cmd_init: CrossDomainInit = Default::default();

        cmd_init.hdr.cmd = CROSS_DOMAIN_CMD_INIT as u8;
        cmd_init.hdr.cmd_size = size_of::<CrossDomainInit>() as u16;
        cmd_init.query_ring_id = query_ring_resource_id;
        cmd_init.channel_ring_id = channel_ring_resource_id;
        cmd_init.channel_type = CROSS_DOMAIN_CHANNEL_TYPE_WAYLAND;

        println!("cmd_init: {:?}", cmd_init);

        self.submit_cmd(&cmd_init, &vec![], CROSS_DOMAIN_RING_NONE, 0, false);
        self.channel_poll();
    }

    pub fn send(&self, iov: &[std::io::IoSlice<'_>], fds: &[OwnedFd], _mode: IoMode) -> usize {
        const DEFAULT_BUFFER_SIZE: usize = 4096;

        let mut cmd_send: CrossDomainSendReceive = Default::default();
        cmd_send.hdr.cmd = CROSS_DOMAIN_CMD_SEND as u8;

        let opaque_data_size = iov.iter().map(|x| x.len()).sum::<usize>();
        assert!(
            opaque_data_size < DEFAULT_BUFFER_SIZE - size_of::<CrossDomainSendReceive>(),
            "data packet too big"
        );
        cmd_send.hdr.cmd_size =
            size_of::<CrossDomainSendReceive>() as u16 + opaque_data_size as u16;

        cmd_send.opaque_data_size = opaque_data_size as u32;

        for i in 0..fds.len() {
            self.fd_analysis(
                &fds[i],
                &mut cmd_send.identifiers[i],
                &mut cmd_send.identifier_types[i],
            )
        }

        cmd_send.num_identifiers = fds.len() as u32;

        let mut data: Vec<u8> = vec![];

        data.write_vectored(iov).unwrap();

        self.submit_cmd(&cmd_send, &data, CROSS_DOMAIN_RING_NONE, 0, false);
        opaque_data_size
    }

    fn fd_analysis(&self, fd: &OwnedFd, idx: &mut u32, idx_type: &mut u32) {
        let mut gem_handle: drm_prime_handle = Default::default();

        gem_handle.fd = fd.as_raw_fd();

        let _ret =
            unsafe { drm_prime_fd_to_handle(self.card_fd.as_raw_fd(), &mut gem_handle) }.unwrap();

        let mut res_info: drm_virtgpu_resource_info = Default::default();

        res_info.bo_handle = gem_handle.handle;

        let _ret =
            unsafe { virtgpu_resource_info(self.card_fd.as_raw_fd(), &mut res_info) }.unwrap();

        *idx = res_info.res_handle;
        *idx_type = CROSS_DOMAIN_ID_TYPE_VIRTGPU_BLOB;

        println!(
            "fd: {}, idx: {} idx_type: {}, gem_handle: {:?}, res_info: {:?}",
            fd.as_raw_fd(),
            idx,
            idx_type,
            gem_handle,
            res_info,
        );
    }

    pub fn channel_poll(&self) {
        let mut cmd_poll: CrossDomainPoll = Default::default();
        cmd_poll.hdr.cmd = CROSS_DOMAIN_CMD_POLL as u8;
        cmd_poll.hdr.cmd_size = size_of::<CrossDomainPoll>() as u16;
        self.submit_cmd(&cmd_poll, &vec![], CROSS_DOMAIN_CHANNEL_RING, 0, false);
    }

    pub fn handle_channel_event(
        &self,
        iov: &mut [std::io::IoSliceMut<'_>],
        fds: &mut VecDeque<OwnedFd>,
        mode: IoMode,
    ) -> io::Result<usize> {
        let cmd_hdr: &CrossDomainHeader = unsafe {
            (self.channel_ring_addr as *const CrossDomainHeader)
                .as_ref()
                .unwrap()
        };
        let mut drm_event_buf = vec![0; size_of::<drm_event>()];
        let mut bytes_read = 0;
        loop {
            let res = nix::unistd::read(self.card_fd.as_raw_fd(), &mut drm_event_buf[..]);

            if Some(nix::errno::Errno::EAGAIN) != res.err() {
                bytes_read = res.unwrap();
                print!("{:?}, ", res);
                break;
            }
            if mode == IoMode::NonBlocking {
                return Err(io::Error::from(ErrorKind::WouldBlock));
            }
        }
        assert!(bytes_read == size_of::<drm_event>(), "invalid size");

        let (head, body, _tail) = unsafe { drm_event_buf.align_to_mut::<drm_event>() };
        assert!(head.is_empty(), "Data was not aligned");
        let dummy_event = &body[0];
        assert!(
            dummy_event.type_ == VIRTGPU_EVENT_FENCE_SIGNALED,
            "invalid event type"
        );

        let ret = match cmd_hdr.cmd as u32 {
            CROSS_DOMAIN_CMD_RECEIVE => self.handle_receive(iov, fds),
            CROSS_DOMAIN_CMD_READ => self.handle_read(),
            x => panic!("shouldn't get this command {}", x),
        };
        self.channel_poll();
        println!("bytes_read: {}", ret);
        Ok(ret)
    }

    fn handle_receive(
        &self,
        iov: &mut [std::io::IoSliceMut<'_>],
        fds_in: &mut VecDeque<OwnedFd>,
    ) -> usize {
        let cmd_receive: &CrossDomainSendReceive = unsafe {
            (self.channel_ring_addr as *const CrossDomainSendReceive)
                .as_ref()
                .unwrap()
        };

        let recv_data_addr = self.channel_ring_addr as usize + size_of::<CrossDomainSendReceive>();
        for i in 0..cmd_receive.num_identifiers as usize {
            let mut fd = 0;
            self.create_fd(
                cmd_receive.identifiers[i],
                cmd_receive.identifier_types[i],
                cmd_receive.identifier_sizes[i].into(),
                &mut fd,
            );
            fds_in.push_back(unsafe { OwnedFd::from_raw_fd(fd) });
            if cmd_receive.identifier_types[i] == CROSS_DOMAIN_ID_TYPE_WRITE_PIPE {
                todo!()
            }
        }

        let mut opaque_data = unsafe {
            slice::from_raw_parts(
                recv_data_addr as *mut u8,
                cmd_receive.opaque_data_size as usize,
            )
        };

        //println!("{:?}", cmd_receive);

        //println!("opaque_data: {:?}", opaque_data);
        opaque_data.read_vectored(iov).unwrap();

        cmd_receive.opaque_data_size as usize
    }

    fn create_fd(&self, idx: u32, idx_type: u32, idx_size: u64, fd: &mut i32) {
        if idx_type == CROSS_DOMAIN_ID_TYPE_VIRTGPU_BLOB {
            self.create_host_blob(idx, idx_size, fd);
        } else {
            todo!()
        }
    }

    fn create_host_blob(&self, idx: u32, size: u64, fd: &mut i32) {
        let mut drm_rc_blob: drm_virtgpu_resource_create_blob = Default::default();

        drm_rc_blob.size = size;
        drm_rc_blob.blob_mem = VIRTGPU_BLOB_MEM_HOST3D;
        drm_rc_blob.blob_flags = VIRTGPU_BLOB_FLAG_USE_MAPPABLE | VIRTGPU_BLOB_FLAG_USE_SHAREABLE;
        drm_rc_blob.blob_id = idx as u64;

        let _ret =
            unsafe { virtgpu_resource_create_blob(self.card_fd.as_raw_fd(), &mut drm_rc_blob) }
                .unwrap();

        let mut gem_handle: drm_prime_handle = Default::default();

        gem_handle.handle = drm_rc_blob.bo_handle;
        gem_handle.flags = O_CLOEXEC as u32 | O_RDWR as u32;

        let _ret =
            unsafe { drm_prime_handle_to_fd(self.card_fd.as_raw_fd(), &mut gem_handle) }.unwrap();

        *fd = gem_handle.fd;

        self.close_gem_handle(drm_rc_blob.bo_handle);
    }

    fn handle_read(&self) -> usize {
        todo!()
    }

    fn close_gem_handle(&self, gem_handle: u32) {
        let mut gem_close: drm_gem_close = Default::default();

        gem_close.handle = gem_handle;

        let _ret = unsafe { drm_gem_close(self.card_fd.as_raw_fd(), &gem_close) }.unwrap();
    }

    fn submit_cmd(
        &self,
        cmd: &impl CrossDomainCmd,
        data: &[u8],
        ring_idx: u32,
        ring_handle: u32,
        wait: bool,
    ) {
        let mut wait_3d: drm_virtgpu_3d_wait = Default::default();
        let mut exec: drm_virtgpu_execbuffer = Default::default();

        let mut cmd_buf = unsafe { any_as_u8_slice(cmd) }.to_vec();
        cmd_buf.write(data).unwrap();
        //println!("cmd_buf: {:?}", cmd_buf);

        //exec.command = (cmd as *const _) as u64;
        exec.command = cmd_buf.as_ptr() as u64;
        exec.size = (cmd.size() + data.len() * size_of::<u8>()) as u32;

        //println!("size: {}", size_of_val(&exec));

        if ring_idx != CROSS_DOMAIN_RING_NONE {
            exec.flags = VIRTGPU_EXECBUF_RING_IDX;
            exec.ring_idx = ring_idx;
        }

        if ring_handle != 0 {
            exec.bo_handles = &ring_handle as *const _ as u64;
            exec.num_bo_handles = 1;
        }

        let _ret = unsafe { virtgpu_execbuffer(self.card_fd.as_raw_fd(), &mut exec) }.unwrap();

        if wait {
            let mut ret = -EAGAIN;
            while ret == -EAGAIN {
                wait_3d.handle = ring_handle as u32;
                println!("waiting {:?}", wait_3d);
                ret = unsafe { virtgpu_wait(self.card_fd.as_raw_fd(), &mut wait_3d) }.unwrap();
                println!("waiting {:?}", wait_3d);
            }
        }
    }

    fn create_ring(&self) -> (u32, u32, *mut c_void) {
        let page_size = nix::unistd::sysconf(nix::unistd::SysconfVar::PAGE_SIZE)
            .unwrap()
            .unwrap() as u64;

        let mut drm_rc_blob: drm_virtgpu_resource_create_blob = Default::default();
        let mut map: drm_virtgpu_map = Default::default();

        drm_rc_blob.size = page_size;
        drm_rc_blob.blob_mem = VIRTGPU_BLOB_MEM_GUEST;
        drm_rc_blob.blob_flags = VIRTGPU_BLOB_FLAG_USE_MAPPABLE;

        let _ret =
            unsafe { virtgpu_resource_create_blob(self.card_fd.as_raw_fd(), &mut drm_rc_blob) }
                .unwrap();

        map.handle = drm_rc_blob.bo_handle;

        let _ret = unsafe { virtgpu_map(self.card_fd.as_raw_fd(), &mut map) }.unwrap();

        let out_addr = unsafe {
            nix::sys::mman::mmap(
                None,
                (page_size as usize).try_into().unwrap(),
                ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                MapFlags::MAP_SHARED,
                Some(&self.card_fd),
                map.offset as i64,
            )
        }
        .unwrap();

        (drm_rc_blob.bo_handle, drm_rc_blob.res_handle, out_addr)
    }
}

pub unsafe fn transmute_vec<T, U>(from: Vec<T>) -> Vec<U> {
    let mut from = std::mem::ManuallyDrop::new(from);

    Vec::from_raw_parts(from.as_mut_ptr() as *mut U, from.len(), from.capacity())
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}
