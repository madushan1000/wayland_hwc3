use std::{
    io::{IoSlice, IoSliceMut},
    os::fd::{AsFd, AsRawFd, RawFd},
};

use nix::sys::socket::{ControlMessage, MsgFlags, RecvMsg, SockaddrLike};
use wayrs_client::{
    ArrayBuffer, RingBuffer, WaylandSocket, BYTES_IN_LEN, BYTES_OUT_LEN, FDS_IN_LEN, FDS_OUT_LEN,
};

use card::Card;

mod card;

pub struct VirtgpuWaylandChannel {
    card: Card,
}

impl AsRawFd for VirtgpuWaylandChannel {
    fn as_raw_fd(&self) -> RawFd {
        self.card.as_fd().as_raw_fd()
    }
}

impl WaylandSocket for VirtgpuWaylandChannel {
    fn connect() -> Result<Self, wayrs_client::ConnectError>
    where
        Self: Sized,
    {
        let mut card = Card::open("/dev/dri/card0");
        if card.get_name() != "virtio_gpu" {
            panic!("not a virtio gpu");
        }

        card.create_context();

        Ok(Self { card })
    }

    fn sendmsg(
        &self,
        bytes_out: &mut RingBuffer<BYTES_OUT_LEN>,
        fds: &[RawFd],
        _flags: MsgFlags,
    ) -> nix::Result<usize> {
        println!("sendmsg");
        let mut iov_buf = [IoSlice::new(&[]), IoSlice::new(&[])];
        let iov = bytes_out.get_readable_iov(&mut iov_buf);
        Ok(self.card.send(iov, fds))
    }

    fn recvmsg(
        &self,
        bytes_in: &mut RingBuffer<BYTES_IN_LEN>,
        fds_in: &mut ArrayBuffer<RawFd, FDS_IN_LEN>,
        _flags: MsgFlags,
    ) -> nix::Result<usize> {
        let mut iov_buf = [IoSliceMut::new(&mut []), IoSliceMut::new(&mut [])];
        let iov = bytes_in.get_writeable_iov(&mut iov_buf);
        let mut cmsgs = vec![];
        //let msg = socket::recvmsg::<()>(self.as_raw_fd(), iov, Some(&mut cmsg), flags)?;
        let recv = self.card.handle_channel_event(iov, &mut cmsgs);

        fds_in.extend(&cmsgs);

        Ok(recv)
    }
}
