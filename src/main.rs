use async_trait::async_trait;
use binder_tokio::*;
use std::{io, string};
use tokio::task;

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

async fn start_wayland_client() {
    println!("starting wayland client");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    println!("starting wayland_hwc3 {}", std::process::id());
    start_hwc3().await;
    start_wayland_client().await;
}
