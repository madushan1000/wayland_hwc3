use std::ffi::CString;
use std::fs::File;
use std::os::fd::{BorrowedFd, FromRawFd, OwnedFd};
use std::os::unix::io::AsRawFd;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::time::UNIX_EPOCH;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use android_hardware_graphics_common::aidl::android::hardware::graphics::common::Rect::Rect as AidlRect;
use android_hardware_graphics_common::aidl::android::hardware::graphics::common::FRect::FRect as AidlFRect;
use android_hardware_graphics_composer3::aidl::android::hardware::graphics::composer3::ChangedCompositionLayer::ChangedCompositionLayer;
use android_hardware_graphics_composer3::aidl::android::hardware::graphics::composer3::ChangedCompositionTypes::ChangedCompositionTypes;
use android_hardware_graphics_composer3::aidl::android::hardware::graphics::composer3::ClientTarget::ClientTarget;
use android_hardware_graphics_composer3::aidl::android::hardware::graphics::composer3::Composition;
use android_hardware_graphics_composer3::aidl::android::hardware::graphics::composer3::DisplayRequest::{DisplayRequest, LayerRequest};
use android_hardware_graphics_composer3::aidl::android::hardware::graphics::composer3::PresentFence::PresentFence;
use android_hardware_graphics_composer3::aidl::android::hardware::graphics::composer3::ReleaseFences;
use binder::ParcelFileDescriptor;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex as AsyncMutex;

use android_hardware_common::aidl::android::hardware::common::NativeHandle::NativeHandle;
use android_hardware_graphics_common::aidl::android::hardware::graphics::common::{
    Dataspace::Dataspace, DisplayDecorationSupport::DisplayDecorationSupport, Hdr::Hdr,
    HdrConversionCapability::HdrConversionCapability, HdrConversionStrategy::HdrConversionStrategy,
    PixelFormat::PixelFormat, Transform::Transform,
};
use android_hardware_graphics_composer3::aidl::android::hardware::graphics::composer3::Buffer::Buffer as AidlBuffer;
use android_hardware_graphics_composer3::aidl::android::hardware::graphics::composer3::{
    ColorMode::ColorMode,
    CommandResultPayload::CommandResultPayload,
    ContentType::ContentType,
    DisplayAttribute::DisplayAttribute,
    DisplayCapability::DisplayCapability,
    DisplayCommand::DisplayCommand,
    DisplayConnectionType::DisplayConnectionType,
    DisplayContentSample::DisplayContentSample,
    DisplayContentSamplingAttributes::DisplayContentSamplingAttributes,
    DisplayIdentification::DisplayIdentification,
    FormatColorComponent::FormatColorComponent,
    HdrCapabilities::HdrCapabilities,
    IComposerCallback::IComposerCallback,
    IComposerClient::{self, IComposerClientAsyncServer},
    LayerCommand::LayerCommand,
    OverlayProperties::OverlayProperties,
    PerFrameMetadataKey::PerFrameMetadataKey,
    PowerMode::PowerMode,
    ReadbackBufferAttributes::ReadbackBufferAttributes,
    RenderIntent::RenderIntent,
    VirtualDisplay::VirtualDisplay,
    VsyncPeriodChangeConstraints::VsyncPeriodChangeConstraints,
    VsyncPeriodChangeTimeline::VsyncPeriodChangeTimeline,
};
use async_trait::async_trait;
use tokio::time::interval;

use crate::bindings::{sw_sync_fence_create, sw_sync_timeline_create, sw_sync_timeline_inc};

#[derive(Debug)]
pub enum HwcEvent {
    CreateDisplay { display: i64 },
    PresentDisplay { display: i64 },
    CreateLayer { display: i64, layer: i64 },
    DestroyLayer { display: i64, layer: i64 },
    PresentLayer { display: i64, layer: i64 },
}

pub struct ComposerClient {
    state: Arc<Mutex<ComposerClientState>>,
    channel: Sender<HwcEvent>,
    callback: Arc<Mutex<Option<binder::Strong<dyn IComposerCallback>>>>,
    sync_timeline: OwnedFd,
    next_sync_point: AtomicU32,
}

#[derive(Debug)]
pub struct ComposerClientState {
    pub display_config: Vec<i32>,
    pub layers: HashMap<i64, ClientLayer>,
    pub layer_count: i64,
}

impl ComposerClientState {
    pub fn new() -> Self {
        Self {
            display_config: vec![
                /*invalid*/ 0,
                /*width*/ 1080,
                /*height*/ 720,
                /*vsync_period*/ 1000 * 1000 * 1000 / 60,
                /*dpi_x*/ 100,
                /*dpi_y*/ 100,
                /*pad*/ 0,
                /*configGroup*/ 1,
            ],
            layers: HashMap::new(),
            layer_count: 0,
        }
    }
}

#[derive(Debug)]
pub struct LayerUpdatePayload {
    pub buffer: ClientBuffer,
    pub display_frame: ClientRect,
    pub source_crop: ClientFRect,
    pub z: i32,
}

#[derive(Debug, Default, Clone)]
pub struct ClientRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl From<&AidlRect> for ClientRect {
    fn from(value: &AidlRect) -> Self {
        Self {
            left: value.left,
            top: value.top,
            right: value.right,
            bottom: value.bottom,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ClientFRect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl From<&AidlFRect> for ClientFRect {
    fn from(value: &AidlFRect) -> Self {
        Self {
            left: value.left,
            top: value.top,
            right: value.right,
            bottom: value.bottom,
        }
    }
}

#[derive(Debug)]
pub struct ClientBuffer {
    pub fds: Vec<OwnedFd>,
    pub ints: Vec<i32>,
    pub fence: Option<OwnedFd>,
}

impl ClientBuffer {
    pub fn new() -> Self {
        Self {
            fds: vec![],
            ints: vec![],
            fence: None,
        }
    }
}

impl From<&AidlBuffer> for ClientBuffer {
    fn from(value: &AidlBuffer) -> Self {
        let (fds, ints) = match value.handle {
            Some(NativeHandle { ref fds, ref ints }) => (fds, ints),
            None => panic!("no fds"),
        };

        let fds = fds
            .into_iter()
            .map(|x| {
                unsafe { BorrowedFd::borrow_raw((x).as_raw_fd()) }
                    .try_clone_to_owned()
                    .unwrap()
            })
            .collect();

        Self {
            fds,
            ints: ints.clone(),
            fence: value.fence.as_ref().and_then(|x| {
                Some(unsafe {
                    BorrowedFd::borrow_raw(x.as_raw_fd())
                        .try_clone_to_owned()
                        .unwrap()
                })
            }),
        }
    }
}

impl From<&ClientBuffer> for ClientBuffer {
    fn from(value: &ClientBuffer) -> Self {
        Self {
            fds: value.fds.iter().map(|x| x.try_clone().unwrap()).collect(),
            ints: value.ints.clone(),
            fence: value
                .fence
                .as_ref()
                .and_then(|x| Some(x.try_clone().unwrap())),
        }
    }
}

#[derive(Debug, Default)]
pub struct ClientLayer {
    pub buffer_slot_count: i32,
    pub name: String,
    pub layer: i64,
    pub slot: i32,
    pub cursor_position: Option<()>, //TODO
    pub buffers: Vec<Option<ClientBuffer>>,
    pub damage: Vec<ClientRect>,
    pub blend_mode: Option<()>, //TODO
    pub color: Option<()>,      //TODO
    pub display_frame: Option<ClientRect>,
    pub source_crop: Option<ClientFRect>,
    pub transform: Option<()>,           //TODO
    pub visible_region: Vec<ClientRect>, //TODO
    pub z: Option<i32>,
    pub color_transform: Vec<f32>,        //TODO
    pub blocking_region: Vec<ClientRect>, //TODO
    pub buffer_slots_to_clear: Vec<i32>,  //TODO
}
impl ClientLayer {
    pub fn new(buffer_slot_count: i32, layer: i64) -> Self {
        let mut buffers = vec![];
        for _ in 0..buffer_slot_count {
            buffers.push(None);
        }
        Self {
            buffer_slot_count,
            layer,
            buffers,
            ..Default::default()
        }
    }
    pub fn merge(&mut self, command: &LayerCommand) {
        if let Some(ref cursor_position) = command.cursorPosition {
            println!("cursor_position todo {:?}", cursor_position);
        }
        if let Some(AidlBuffer {
            ref slot,
            ref handle,
            ref fence,
        }) = command.buffer
        {
            if handle.is_some() {
                self.buffers[*slot as usize] =
                    Some(ClientBuffer::from(command.buffer.as_ref().unwrap()));
            }
            if handle.is_none() {
                self.buffers[*slot as usize].as_mut().unwrap().fence = fence.as_ref().map(|fd| {
                    unsafe { BorrowedFd::borrow_raw(fd.as_raw_fd()) }
                        .try_clone_to_owned()
                        .unwrap()
                });
            }
            self.slot = *slot;
        }
        if let Some(ref damage) = command.damage {
            self.damage = damage
                .iter()
                .map(|x| ClientRect::from(x.as_ref().unwrap()))
                .collect::<Vec<_>>();
        }
        if let Some(ref blend_mode) = command.blendMode {
            println!("blend_mode todo {:?}", blend_mode);
        }
        if let Some(ref color) = command.color {
            println!("color todo {:?}", color);
        }
        if let Some(ref display_frame) = command.displayFrame {
            self.display_frame = Some(ClientRect::from(display_frame));
        }
        if let Some(ref source_crop) = command.sourceCrop {
            self.source_crop = Some(ClientFRect::from(source_crop));
        }
        if let Some(ref transform) = command.transform {
            println!("transform todo {:?}", transform);
        }
        if let Some(ref visible_region) = command.visibleRegion {
            self.visible_region = visible_region
                .iter()
                .map(|x| ClientRect::from(x.as_ref().unwrap()))
                .collect::<Vec<_>>();
        }
        if let Some(ref z) = command.z {
            self.z = Some(z.z);
        }
        if let Some(ref color_transform) = command.colorTransform {
            self.color_transform = color_transform.clone();
        }
        if let Some(ref blocking_region) = command.blockingRegion {
            self.blocking_region = blocking_region
                .iter()
                .map(|x| ClientRect::from(x.as_ref().unwrap()))
                .collect::<Vec<_>>();
        }
    }
}

impl ComposerClient {
    pub fn new(tx: Sender<HwcEvent>, state: Arc<Mutex<ComposerClientState>>) -> Self {
        let sync_timeline = unsafe { OwnedFd::from_raw_fd(sw_sync_timeline_create()) };
        Self {
            state,
            channel: tx,
            callback: Arc::new(Mutex::new(None)),
            sync_timeline,
            next_sync_point: Default::default(),
        }
    }

    async fn execute_layer_command(
        &self,
        display: i64,
        command: &LayerCommand,
    ) -> Vec<CommandResultPayload> {
        let mut results = vec![];

        {
            let mut state = self.state.lock().unwrap();
            let layer = state.layers.get_mut(&command.layer).unwrap();

            layer.merge(command);

            if command.buffer.is_some() {
                let sync_point = self.next_sync_point.fetch_add(1, Ordering::Relaxed);
                let name = CString::new("hwc release").unwrap();
                let release_fence = unsafe {
                    sw_sync_fence_create(
                        self.sync_timeline.as_raw_fd(),
                        name.as_ptr(),
                        sync_point + 1,
                    )
                };
                let res = CommandResultPayload::ReleaseFences(ReleaseFences::ReleaseFences {
                    display,
                    layers: vec![ReleaseFences::Layer::Layer {
                        layer: command.layer,
                        fence: Some(ParcelFileDescriptor::new(unsafe {
                            File::from_raw_fd(release_fence)
                        })),
                    }],
                });
                results.push(res);
            }
        }

        self.channel
            .send(HwcEvent::PresentLayer {
                display,
                layer: command.layer,
            })
            .await
            .unwrap();

        results.push(CommandResultPayload::ChangedCompositionTypes(
            ChangedCompositionTypes {
                display,
                layers: vec![ChangedCompositionLayer {
                    layer: command.layer,
                    composition: Composition::Composition::DEVICE,
                }],
            },
        ));
        results
    }
}

impl binder::Interface for ComposerClient {}

#[async_trait]
impl IComposerClientAsyncServer for ComposerClient {
    fn get_descriptor() -> &'static str
    where
        Self: Sized,
    {
        "android.hardware.graphics.composer3.IComposerClient"
    }
    async fn r#createLayer(&self, display: i64, buffer_slot_count: i32) -> binder::Result<i64> {
        let layer_id;
        {
            let mut state = self.state.lock().unwrap();
            state.layer_count += 1;
            layer_id = state.layer_count;
            state
                .layers
                .insert(layer_id, ClientLayer::new(buffer_slot_count, layer_id));
        }
        self.channel
            .send(HwcEvent::CreateLayer {
                display,
                layer: layer_id,
            })
            .await
            .unwrap();
        println!("createLayer {}", layer_id);
        Ok(layer_id)
    }
    async fn r#createVirtualDisplay(
        &self,
        _arg_width: i32,
        _arg_height: i32,
        _arg_format_hint: PixelFormat,
        _arg_output_buffer_slot_count: i32,
    ) -> binder::Result<VirtualDisplay> {
        todo!()
    }
    async fn r#destroyLayer(&self, display: i64, layer: i64) -> binder::Result<()> {
        {
            let mut state = self.state.lock().unwrap();
            state.layers.remove(&layer);
        }
        self.channel
            .send(HwcEvent::DestroyLayer { display, layer })
            .await
            .unwrap();
        println!("destroyLayer {}", layer);
        Ok(())
    }
    async fn r#destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<()> {
        todo!()
    }
    async fn r#executeCommands(
        &self,
        commands: &[DisplayCommand],
    ) -> binder::Result<Vec<CommandResultPayload>> {
        let mut results = vec![];
        println!("executeCommands");

        for command in commands.iter() {
            //println!("got display command");
            println!("{:#?}\n", command);
            for layer_command in command.layers.iter() {
                results.push(
                    self.execute_layer_command(command.display, layer_command)
                        .await,
                )
            }

            match command {
                DisplayCommand {
                    display,
                    presentDisplay: true,
                    //clientTarget: Some(ct),
                    ..
                } => {
                    //self.client_target(ct).await;
                    self.channel
                        .send(HwcEvent::PresentDisplay { display: *display })
                        .await
                        .unwrap();
                }
                DisplayCommand { .. } => {}
            }
        }

        println!("{:?}", results);
        Ok(results.into_iter().flatten().collect::<Vec<_>>())
    }
    async fn r#getActiveConfig(&self, _arg_display: i64) -> binder::Result<i32> {
        Ok(1)
    }
    async fn r#getColorModes(&self, _arg_display: i64) -> binder::Result<Vec<ColorMode>> {
        //TODO
        Ok(vec![ColorMode::NATIVE])
    }
    async fn r#getDataspaceSaturationMatrix(
        &self,
        _arg_dataspace: Dataspace,
    ) -> binder::Result<Vec<f32>> {
        todo!()
    }
    async fn r#getDisplayAttribute(
        &self,
        _display: i64,
        _config: i32,
        attribute: DisplayAttribute,
    ) -> binder::Result<i32> {
        Ok(self.state.lock().unwrap().display_config[attribute.0 as usize])
    }
    async fn r#getDisplayCapabilities(
        &self,
        _arg_display: i64,
    ) -> binder::Result<Vec<DisplayCapability>> {
        Ok(vec![])
    }
    async fn r#getDisplayConfigs(&self, _display: i64) -> binder::Result<Vec<i32>> {
        Ok(self.state.lock().unwrap().display_config.clone())
    }
    async fn r#getDisplayConnectionType(
        &self,
        _arg_display: i64,
    ) -> binder::Result<DisplayConnectionType> {
        Ok(DisplayConnectionType::INTERNAL)
    }
    async fn r#getDisplayIdentificationData(
        &self,
        _arg_display: i64,
    ) -> binder::Result<DisplayIdentification> {
        let edid = [
            0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x1c, 0xec, 0x01, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x1b, 0x10, 0x01, 0x03, 0x80, 0x50, 0x2d, 0x78, 0x0a, 0x0d, 0xc9, 0xa0,
            0x57, 0x47, 0x98, 0x27, 0x12, 0x48, 0x4c, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01,
            0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x3a,
            0x80, 0x18, 0x71, 0x38, 0x2d, 0x40, 0x58, 0x2c, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfc,
            0x00, 0x45, 0x4d, 0x55, 0x5f, 0x64, 0x69, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x5f, 0x30,
            0x00, 0x4b,
        ];
        Ok(DisplayIdentification {
            port: 0,
            data: edid.to_vec(),
        })
    }
    async fn r#getDisplayName(&self, _arg_display: i64) -> binder::Result<String> {
        todo!()
    }
    async fn r#getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<i32> {
        Ok(self.state.lock().unwrap().display_config[3])
    }
    async fn r#getDisplayedContentSample(
        &self,
        _arg_display: i64,
        _arg_max_frames: i64,
        _arg_timestamp: i64,
    ) -> binder::Result<DisplayContentSample> {
        todo!()
    }
    async fn r#getDisplayedContentSamplingAttributes(
        &self,
        _arg_display: i64,
    ) -> binder::Result<DisplayContentSamplingAttributes> {
        todo!()
    }
    async fn r#getDisplayPhysicalOrientation(
        &self,
        _arg_display: i64,
    ) -> binder::Result<Transform> {
        Ok(Transform::NONE)
    }
    async fn r#getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<HdrCapabilities> {
        Err(IComposerClient::EX_UNSUPPORTED.into())
    }
    async fn r#getMaxVirtualDisplayCount(&self) -> binder::Result<i32> {
        todo!()
    }
    async fn r#getPerFrameMetadataKeys(
        &self,
        _arg_display: i64,
    ) -> binder::Result<Vec<PerFrameMetadataKey>> {
        Err(IComposerClient::EX_UNSUPPORTED.into())
    }
    async fn r#getReadbackBufferAttributes(
        &self,
        _arg_display: i64,
    ) -> binder::Result<ReadbackBufferAttributes> {
        todo!()
    }
    async fn r#getReadbackBufferFence(
        &self,
        _arg_display: i64,
    ) -> binder::Result<Option<binder::ParcelFileDescriptor>> {
        todo!()
    }
    async fn r#getRenderIntents(
        &self,
        _arg_display: i64,
        _arg_mode: ColorMode,
    ) -> binder::Result<Vec<RenderIntent>> {
        Ok(vec![RenderIntent::COLORIMETRIC])
    }
    async fn r#getSupportedContentTypes(
        &self,
        _arg_display: i64,
    ) -> binder::Result<Vec<ContentType>> {
        Ok(vec![])
    }
    async fn r#getDisplayDecorationSupport(
        &self,
        _arg_display: i64,
    ) -> binder::Result<Option<DisplayDecorationSupport>> {
        Err(IComposerClient::EX_UNSUPPORTED.into())
    }
    async fn r#registerCallback(
        &self,
        callback: &binder::Strong<dyn IComposerCallback>,
    ) -> binder::Result<()> {
        self.callback.lock().unwrap().replace(callback.clone());
        callback.onHotplug(1, true).unwrap();
        self.channel
            .send(HwcEvent::CreateDisplay { display: 1 })
            .await
            .unwrap();
        Ok(())
    }
    async fn r#setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> {
        todo!()
    }
    async fn r#setActiveConfigWithConstraints(
        &self,
        _arg_display: i64,
        _arg_config: i32,
        _arg_vsync_period_change_constraints: &VsyncPeriodChangeConstraints,
    ) -> binder::Result<VsyncPeriodChangeTimeline> {
        Ok(VsyncPeriodChangeTimeline {
            newVsyncAppliedTimeNanos: 0,
            refreshRequired: false,
            refreshTimeNanos: 0,
        })
    }
    async fn r#setBootDisplayConfig(
        &self,
        _arg_display: i64,
        _arg_config: i32,
    ) -> binder::Result<()> {
        todo!()
    }
    async fn r#clearBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<()> {
        todo!()
    }
    async fn r#getPreferredBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<i32> {
        todo!()
    }
    async fn r#setAutoLowLatencyMode(
        &self,
        _arg_display: i64,
        _arg_on: bool,
    ) -> binder::Result<()> {
        todo!()
    }
    async fn r#setClientTargetSlotCount(
        &self,
        display: i64,
        slot_count: i32,
    ) -> binder::Result<()> {
        println!("display: {}, slotcount: {}", display, slot_count);
        Ok(())
    }
    async fn r#setColorMode(
        &self,
        _arg_display: i64,
        _arg_mode: ColorMode,
        _arg_intent: RenderIntent,
    ) -> binder::Result<()> {
        todo!()
    }
    async fn r#setContentType(
        &self,
        _arg_display: i64,
        _arg_type: ContentType,
    ) -> binder::Result<()> {
        todo!()
    }
    async fn r#setDisplayedContentSamplingEnabled(
        &self,
        _arg_display: i64,
        _arg_enable: bool,
        _arg_component_mask: FormatColorComponent,
        _arg_max_frames: i64,
    ) -> binder::Result<()> {
        todo!()
    }
    async fn r#setPowerMode(&self, _arg_display: i64, _arg_mode: PowerMode) -> binder::Result<()> {
        //TODO
        Ok(())
    }
    async fn r#setReadbackBuffer(
        &self,
        _arg_display: i64,
        _arg_buffer: &NativeHandle,
        _arg_release_fence: Option<&binder::ParcelFileDescriptor>,
    ) -> binder::Result<()> {
        todo!()
    }
    async fn r#setVsyncEnabled(&self, display: i64, _arg_enabled: bool) -> binder::Result<()> {
        let vsync_period = self.state.lock().unwrap().display_config[3];
        let callback = self.callback.lock().unwrap().as_ref().unwrap().clone();
        let mut vsync_interval =
            tokio::time::interval(tokio::time::Duration::from_nanos(vsync_period as u64));
        tokio::spawn(async move {
            loop {
                vsync_interval.tick().await;
                let time = std::time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap();
                //println!("vsync");
                callback
                    .clone()
                    .onVsync(display, time.as_nanos() as i64, vsync_period)
                    .unwrap();
            }
        });
        Ok(())
    }
    async fn r#setIdleTimerEnabled(
        &self,
        _arg_display: i64,
        _arg_timeout_ms: i32,
    ) -> binder::Result<()> {
        todo!()
    }
    async fn r#getOverlaySupport(&self) -> binder::Result<OverlayProperties> {
        Err(IComposerClient::EX_UNSUPPORTED.into())
    }
    async fn r#getHdrConversionCapabilities(&self) -> binder::Result<Vec<HdrConversionCapability>> {
        Err(IComposerClient::EX_UNSUPPORTED.into())
    }
    async fn r#setHdrConversionStrategy(
        &self,
        _arg_conversion_strategy: &HdrConversionStrategy,
    ) -> binder::Result<Hdr> {
        todo!()
    }
    async fn r#setRefreshRateChangedCallbackDebugEnabled(
        &self,
        _arg_display: i64,
        _arg_enabled: bool,
    ) -> binder::Result<()> {
        todo!()
    }
    async fn r#setLayerName(&self, display: i64, layer_id: i64, name: &str) -> binder::Result<()> {
        let mut state = self.state.lock().unwrap();
        state.layers.get_mut(&layer_id).unwrap().name = name.into();
        println!("setLayerName: {} {} {}", display, layer_id, name);
        Ok(())
    }
}
