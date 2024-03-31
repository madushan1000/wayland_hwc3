#![allow(unused_imports)]
use async_trait::async_trait;
use binder_tokio::*;
use composer_client::{ComposerClientState, HwcEvent};
use std::{
    os::fd::{AsFd, BorrowedFd, FromRawFd, OwnedFd},
    sync::{Arc, Mutex, RwLock},
};
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    task,
};

#[allow(non_camel_case_types, non_snake_case, unused_imports)]
use android_hardware_graphics_composer3::aidl::android::hardware::graphics::composer3::IComposer;
use android_hardware_graphics_composer3::aidl::android::hardware::graphics::composer3::{
    Capability,
    IComposer::{BnComposer, IComposerAsyncServer},
    IComposerClient,
};
use binder::{self, BinderFeatures};

#[allow(non_camel_case_types, non_snake_case, unused, non_upper_case_globals)]
pub mod bindings;
mod composer_client;
mod wayland_client;

use wayland_client::start_wayland_client;

use bindings::sw_sync_timeline_create;

pub struct WaylandHwc3Service {
    hwc_client: Mutex<Option<binder::Strong<dyn IComposerClient::IComposerClient>>>,
    channel: Sender<HwcEvent>,
    client_state: Arc<Mutex<ComposerClientState>>,
}

const SERVICE_IDENTIFIER: &str = "android.hardware.graphics.composer3.IComposer/default";

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    println!("starting wayland_hwc3 {}", std::process::id());

    //let mut _a = String::new();
    //io::stdin().read_line(&mut _a);
    let (tx, rx) = mpsc::channel(32);
    let composer_client_state = Arc::new(Mutex::new(composer_client::ComposerClientState::new()));
    tokio::spawn(start_wayland_client(rx, composer_client_state.clone()));
    tokio::join!(start_hwc3_server(tx, composer_client_state));
}

async fn start_hwc3_server(tx: Sender<HwcEvent>, client_state: Arc<Mutex<ComposerClientState>>) {
    binder::ProcessState::start_thread_pool();
    let wayland_hwc3_service = WaylandHwc3Service {
        hwc_client: Mutex::new(None),
        channel: tx,
        client_state,
    };
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

impl binder::Interface for WaylandHwc3Service {}

#[async_trait]
impl IComposerAsyncServer for WaylandHwc3Service {
    async fn r#createClient(
        &self,
    ) -> binder::Result<binder::Strong<dyn IComposerClient::IComposerClient>> {
        let mut client = self.hwc_client.lock().unwrap();
        if client.is_some() {
            panic!("hwc client already exists");
        }

        let new_composer_client = composer_client::ComposerClient::new(
            self.channel.clone(),
            self.client_state.clone(),
        );

        let new_composer_client_binder = IComposerClient::BnComposerClient::new_async_binder(
            new_composer_client,
            TokioRuntime(tokio::runtime::Handle::current()),
            BinderFeatures::default(),
        );

        client.replace(new_composer_client_binder);
        Ok(client.as_mut().unwrap().to_owned())
    }

    async fn r#getCapabilities(&self) -> binder::Result<Vec<Capability::Capability>> {
        Ok(vec![])
    }
}
