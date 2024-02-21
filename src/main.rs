use std::{io, string};

#[allow(non_snake_case)]
use android_hardware_graphics_composer3::aidl::android::hardware::graphics::composer3::IComposer;
use android_hardware_graphics_composer3::aidl::android::hardware::graphics::composer3::{
    IComposer::BnComposer,
    IComposerClient, Capability,
};
use binder;

pub struct WaylandHwc3Service;

impl binder::Interface for WaylandHwc3Service {}

impl IComposer::IComposer for WaylandHwc3Service {
    fn r#createClient(
        &self,
    ) -> binder::Result<binder::Strong<dyn IComposerClient::IComposerClient>> {
        panic!("not implimented createClient")
    }

    fn r#getCapabilities(&self) -> binder::Result<Vec<Capability::Capability>>{
        panic!("not implimented getCapabilities")
    }
}

const SERVICE_IDENTIFIER: &str = "android.hardware.graphics.composer3.IComposer/default";

fn main() {
    println!("starting wayland_hwc3 {}", std::process::id());

    let mut _a = String::new();
    io::stdin().read_line(&mut _a);

    let wayland_hwc3_service = WaylandHwc3Service;
    let wayland_hwc3_service_binder =
        BnComposer::new_binder(wayland_hwc3_service, binder::BinderFeatures::default());
    binder::add_service(SERVICE_IDENTIFIER, wayland_hwc3_service_binder.as_binder())
        .expect("Failed to register wayland hwc3 service");
    binder::ProcessState::join_thread_pool()
}
