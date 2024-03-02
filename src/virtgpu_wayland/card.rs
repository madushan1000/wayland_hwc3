use std::{
    borrow::Borrow,
    ffi::{c_void, OsString},
    io::{IoSlice, IoSliceMut, Read, Write},
    mem::{size_of, size_of_val},
    os::{
        fd::{AsFd, AsRawFd, BorrowedFd, RawFd},
        unix::{ffi::OsStringExt, fs::OpenOptionsExt},
    },
    ptr::{addr_of, null_mut, NonNull},
    slice,
};

use nix::{
    ioctl_readwrite,
    libc::EAGAIN,
    sys::{
        mman::{MapFlags, ProtFlags},
        socket::ControlMessage,
    },
};

use crate::virtgpu_wayland::card::bindings::{
    CrossDomainHeader, CROSS_DOMAIN_CHANNEL_TYPE_WAYLAND, CROSS_DOMAIN_CMD_READ,
    CROSS_DOMAIN_CMD_RECEIVE, DRM_COMMAND_BASE, DRM_VIRTGPU_CONTEXT_INIT, DRM_VIRTGPU_EXECBUFFER,
    DRM_VIRTGPU_MAP, DRM_VIRTGPU_RESOURCE_CREATE_BLOB, DRM_VIRTGPU_RESOURCE_INFO, DRM_VIRTGPU_WAIT,
    VIRTGPU_EVENT_FENCE_SIGNALED,
};

pub use self::bindings::drm_version;
use self::bindings::{
    drm_event, drm_prime_handle, drm_virtgpu_3d_wait, drm_virtgpu_context_init,
    drm_virtgpu_context_set_param, drm_virtgpu_execbuffer, drm_virtgpu_map,
    drm_virtgpu_resource_create_blob, drm_virtgpu_resource_info, CrossDomainInit, CrossDomainPoll,
    CrossDomainSendReceive, CROSS_DOMAIN_CHANNEL_RING, CROSS_DOMAIN_CMD_INIT,
    CROSS_DOMAIN_CMD_POLL, CROSS_DOMAIN_CMD_SEND, CROSS_DOMAIN_ID_TYPE_VIRTGPU_BLOB,
    CROSS_DOMAIN_RING_NONE, DRM_IOCTL_BASE, VIRTGPU_BLOB_FLAG_USE_MAPPABLE, VIRTGPU_BLOB_MEM_GUEST,
    VIRTGPU_CONTEXT_PARAM_CAPSET_ID, VIRTGPU_CONTEXT_PARAM_NUM_RINGS,
    VIRTGPU_CONTEXT_PARAM_POLL_RINGS_MASK, VIRTGPU_EXECBUF_RING_IDX,
};

mod bindings;

pub const VIRTIO_GPU_CAPSET_VIRGL: u32 = 1;
pub const VIRTIO_GPU_CAPSET_VIRGL2: u32 = 2;
pub const VIRTIO_GPU_CAPSET_GFXSTREAM: u32 = 3;
pub const VIRTIO_GPU_CAPSET_VENUS: u32 = 4;
pub const VIRTIO_GPU_CAPSET_CROSS_DOMAIN: u32 = 5;

ioctl_readwrite!(drm_get_version, DRM_IOCTL_BASE, 0x00, drm_version);
ioctl_readwrite!(
    drm_prime_fd_to_handle,
    DRM_IOCTL_BASE,
    0x2e,
    drm_prime_handle
);
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

trait CrossDomainCmd {
    fn size(&self) -> usize {
        size_of_val(self)
    }
}

impl CrossDomainCmd for CrossDomainInit {}
impl CrossDomainCmd for CrossDomainPoll {}
impl CrossDomainCmd for CrossDomainSendReceive {}

pub struct Card {
    card_fd: std::fs::File,
    channel_ring_addr: *mut c_void,
    channel_ring_handle: u32,
    query_ring_addr: *mut c_void,
    query_ring_handle: u32,
}

impl Card {
    pub fn open(path: &str) -> Self {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        options.write(true);
        //options.custom_flags(nix::libc::O_NONBLOCK);
        let file = options.open(path).unwrap();
        //let flags = nix::fcntl::fcntl(file.as_raw_fd(), nix::fcntl::FcntlArg::F_GETFL).unwrap();
        //let flags = nix::fcntl::OFlag::O_RDONLY | nix::fcntl::OFlag::O_NONBLOCK;
        //nix::fcntl::fcntl(file.as_raw_fd(), nix::fcntl::FcntlArg::F_SETFL(flags)).unwrap();
        Card {
            card_fd: file,
            channel_ring_addr: null_mut(),
            channel_ring_handle: 0,
            query_ring_addr: null_mut(),
            query_ring_handle: 0,
        }
    }

    pub fn get_name(&self) -> OsString {
        let mut data = drm_version::default();
        unsafe {
            drm_get_version(self.as_fd().as_raw_fd(), &mut data);
        }
        let mut name_buf: Vec<i8> = Vec::with_capacity(data.name_len as usize);

        let mut new_data = drm_version {
            name_len: data.name_len,
            name: name_buf.as_mut_ptr(),
            ..Default::default()
        };

        unsafe {
            drm_get_version(self.as_fd().as_raw_fd(), &mut new_data);
        }

        unsafe {
            name_buf.set_len(data.name_len as usize);
        }
        OsString::from_vec(unsafe { transmute_vec(name_buf) })
    }

    pub fn create_context(&mut self) {
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

        let _ret = unsafe { virtgpu_context_init(self.as_fd().as_raw_fd(), &mut init) }.unwrap();

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

    pub fn send(&self, iov: &[std::io::IoSlice<'_>], fds_out: &[RawFd]) -> usize {
        const DEFAULT_BUFFER_SIZE: usize = 4096;

        let mut cmd_send: CrossDomainSendReceive = Default::default();
        cmd_send.hdr.cmd = CROSS_DOMAIN_CMD_SEND as u8;

        let opaque_data_size = iov.iter().map(|x| x.len()).sum::<usize>();
        cmd_send.hdr.cmd_size =
            size_of::<CrossDomainSendReceive>() as u16 + opaque_data_size as u16;

        cmd_send.opaque_data_size = opaque_data_size as u32;

        for i in 0..fds_out.len() {
            self.fd_analysis(
                fds_out[i],
                &mut cmd_send.identifiers[i],
                &mut cmd_send.identifier_types[i],
            )
        }

        cmd_send.num_identifiers = fds_out.len() as u32;

        let mut data: Vec<u8> = vec![];

        data.write_vectored(iov).unwrap();

        println!("iov {:?}", iov);

        self.submit_cmd(&cmd_send, &data, CROSS_DOMAIN_RING_NONE, 0, false);
        opaque_data_size
    }

    fn fd_analysis(&self, fd: i32, idx: &mut u32, idx_type: &mut u32) {
        let mut gem_handle: drm_prime_handle = Default::default();

        println!("fd: {}, idx: {} idx_type: {}", fd, idx, idx_type);

        gem_handle.fd = fd;

        let _ret =
            unsafe { drm_prime_fd_to_handle(self.as_fd().as_raw_fd(), &mut gem_handle) }.unwrap();

        let mut res_info: drm_virtgpu_resource_info = Default::default();

        res_info.bo_handle = gem_handle.handle;

        let _ret =
            unsafe { virtgpu_resource_info(self.as_fd().as_raw_fd(), &mut res_info) }.unwrap();

        *idx = res_info.res_handle;
        *idx_type = CROSS_DOMAIN_ID_TYPE_VIRTGPU_BLOB;
    }

    fn channel_poll(&self) {
        let mut cmd_poll: CrossDomainPoll = Default::default();

        cmd_poll.hdr.cmd = CROSS_DOMAIN_CMD_POLL as u8;
        cmd_poll.hdr.cmd_size = size_of::<CrossDomainPoll>() as u16;

        self.submit_cmd(&cmd_poll, &vec![], CROSS_DOMAIN_CHANNEL_RING, 0, false);
    }

    pub fn handle_channel_event(
        &self,
        iov: &mut [std::io::IoSliceMut<'_>],
        fds_out: &mut Vec<RawFd>,
    ) -> usize {
        let cmd_hdr: &CrossDomainHeader = unsafe {
            (self.channel_ring_addr as *const CrossDomainHeader)
                .as_ref()
                .unwrap()
        };
        let mut drm_event_buf = vec![0; size_of::<drm_event>()];
        let bytes_read =
            nix::unistd::read(self.as_fd().as_raw_fd(), &mut drm_event_buf[..]).unwrap();

        assert!(bytes_read == size_of::<drm_event>(), "invalid size");

        let (head, body, _tail) = unsafe { drm_event_buf.align_to_mut::<drm_event>() };
        assert!(head.is_empty(), "Data was not aligned");
        let dummy_event = &body[0];
        assert!(
            dummy_event.type_ == VIRTGPU_EVENT_FENCE_SIGNALED,
            "invalid event type"
        );

        match cmd_hdr.cmd as u32 {
            CROSS_DOMAIN_CMD_RECEIVE => return self.handle_receive(iov, fds_out),
            CROSS_DOMAIN_CMD_READ => self.handle_read(),
            x => panic!("shouldn't get this command {}", x),
        }
        todo!()
    }

    fn handle_receive(&self, iov: &mut [std::io::IoSliceMut<'_>], fds_out: &mut Vec<RawFd>) -> usize {
        let cmd_receive: &CrossDomainSendReceive = unsafe {
            (self.channel_ring_addr as *const CrossDomainSendReceive)
                .as_ref()
                .unwrap()
        };

        let mut fds = vec![];

        let recv_data_addr = self.channel_ring_addr as usize + size_of::<CrossDomainSendReceive>();
        for i in 0..cmd_receive.num_identifiers as usize {
            self.create_fd(
                cmd_receive.identifiers[i],
                cmd_receive.identifier_types[i],
                cmd_receive.identifier_sizes[i],
                &fds[i],
            );
        }

        let mut opaque_data = unsafe {
            slice::from_raw_parts(
                recv_data_addr as *mut u8,
                cmd_receive.opaque_data_size as usize,
            )
        };

        println!("{:?}", cmd_receive);

        println!("opaque_data: {:?}", opaque_data);
        opaque_data.read_vectored(iov).unwrap();
        cmd_receive.opaque_data_size as usize
    }

    fn create_fd(&self, idx: u32, idx_type: u32, idx_size: u32, fd: &u32) {}

    fn handle_read(&self) {
        todo!()
    }

    fn submit_cmd(
        &self,
        cmd: &impl CrossDomainCmd,
        data: &[u8],
        ring_idx: u32,
        ring_handle: u64,
        wait: bool,
    ) {
        let mut wait_3d: drm_virtgpu_3d_wait = Default::default();
        let mut exec: drm_virtgpu_execbuffer = Default::default();

        let mut cmd_buf = unsafe { any_as_u8_slice(cmd) }.to_vec();
        cmd_buf.write(data).unwrap();
        println!("cmd_buf: {:?}", cmd_buf);

        //exec.command = (cmd as *const _) as u64;
        exec.command = cmd_buf.as_ptr() as u64;
        exec.size = (cmd.size() + data.len() * size_of::<u8>()) as u32;

        //println!("size: {}", size_of_val(&exec));

        if ring_idx != CROSS_DOMAIN_RING_NONE {
            exec.flags = VIRTGPU_EXECBUF_RING_IDX;
            exec.ring_idx = ring_idx;
        }

        if ring_handle != 0 {
            exec.bo_handles = ring_handle;
            exec.num_bo_handles = 1;
        }

        let _ret = unsafe { virtgpu_execbuffer(self.as_fd().as_raw_fd(), &mut exec) }.unwrap();

        if wait {
            let mut ret = -EAGAIN;
            while ret == -EAGAIN {
                wait_3d.handle = ring_handle as u32;
                ret = unsafe { virtgpu_wait(self.as_fd().as_raw_fd(), &mut wait_3d) }.unwrap();
            }
        }
    }

    fn create_ring(&self) -> (u32, u32, *mut c_void) {
        let PAGE_SIZE = nix::unistd::sysconf(nix::unistd::SysconfVar::PAGE_SIZE)
            .unwrap()
            .unwrap() as u64;

        let mut drm_rc_blob: drm_virtgpu_resource_create_blob = Default::default();
        let mut map: drm_virtgpu_map = Default::default();

        drm_rc_blob.size = PAGE_SIZE;
        drm_rc_blob.blob_mem = VIRTGPU_BLOB_MEM_GUEST;
        drm_rc_blob.blob_flags = VIRTGPU_BLOB_FLAG_USE_MAPPABLE;

        let _ret =
            unsafe { virtgpu_resource_create_blob(self.as_fd().as_raw_fd(), &mut drm_rc_blob) }
                .unwrap();

        map.handle = drm_rc_blob.bo_handle;

        let _ret = unsafe { virtgpu_map(self.as_fd().as_raw_fd(), &mut map) }.unwrap();

        let out_addr = unsafe {
            nix::sys::mman::mmap(
                None,
                (PAGE_SIZE as usize).try_into().unwrap(),
                ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                MapFlags::MAP_SHARED,
                Some(self),
                map.offset as i64,
            )
        }
        .unwrap();

        (drm_rc_blob.bo_handle, drm_rc_blob.res_handle, out_addr)
    }
}

impl AsFd for Card {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.card_fd.as_fd()
    }
}

pub unsafe fn transmute_vec<T, U>(from: Vec<T>) -> Vec<U> {
    let mut from = std::mem::ManuallyDrop::new(from);

    Vec::from_raw_parts(from.as_mut_ptr() as *mut U, from.len(), from.capacity())
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}
