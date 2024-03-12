use std::{
    collections::VecDeque,
    io::{self, IoSlice, IoSliceMut},
    os::fd::{AsFd, AsRawFd, OwnedFd, RawFd},
};

use card::Card;
use wayrs_client::{ClientTransport, IoMode, Transport};

#[allow(non_camel_case_types, non_snake_case, unused, non_upper_case_globals)]
mod card;

pub struct VirtgpuWaylandChannel {
    card: Card,
}

impl AsRawFd for VirtgpuWaylandChannel {
    fn as_raw_fd(&self) -> RawFd {
        self.card.as_fd().as_raw_fd()
    }
}

impl ClientTransport for VirtgpuWaylandChannel {
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

    fn fix_metadata(&mut self, plane_idx: usize,width: u32, height: u32, format: u32) -> Option<(u32, u32, u64)> {
        self.card.fix_metadata(plane_idx, width, height, format)
    }
}

impl Transport for VirtgpuWaylandChannel {
    fn send(&mut self, bytes: &[IoSlice], fds: &[OwnedFd], _mode: IoMode) -> io::Result<usize> {
        println!("sendmsg");
        Ok(self.card.send(bytes, fds))
    }

    fn recv(
        &mut self,
        bytes: &mut [IoSliceMut],
        fds: &mut VecDeque<OwnedFd>,
        mode: IoMode,
    ) -> io::Result<usize> {
        println!("recvmsg {:?}", mode);
        let recv = self.card.handle_channel_event(bytes, fds, mode);

        Ok(recv?)
    }
}
