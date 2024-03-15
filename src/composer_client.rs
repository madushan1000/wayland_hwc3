use android_hardware_common::aidl::android::hardware::common::NativeHandle::NativeHandle;
use android_hardware_graphics_common::aidl::android::hardware::graphics::common::{
    Dataspace::Dataspace, DisplayDecorationSupport::DisplayDecorationSupport, Hdr::Hdr,
    HdrConversionCapability::HdrConversionCapability, HdrConversionStrategy::HdrConversionStrategy,
    PixelFormat::PixelFormat, Transform::Transform,
};
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

pub struct ComposerClient {}

impl binder::Interface for ComposerClient {}

#[async_trait]
impl IComposerClientAsyncServer for ComposerClient {
    fn get_descriptor() -> &'static str
    where
        Self: Sized,
    {
        "android.hardware.graphics.composer3.IComposerClient"
    }
    async fn r#createLayer(
        &self,
        _arg_display: i64,
        _arg_bufferSlotCount: i32,
    ) -> binder::Result<i64> {
        todo!()
    }
    async fn r#createVirtualDisplay(
        &self,
        _arg_width: i32,
        _arg_height: i32,
        _arg_formatHint: PixelFormat,
        _arg_outputBufferSlotCount: i32,
    ) -> binder::Result<VirtualDisplay> {
        todo!()
    }
    async fn r#destroyLayer(&self, _arg_display: i64, _arg_layer: i64) -> binder::Result<()> {
        todo!()
    }
    async fn r#destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<()> {
        todo!()
    }
    async fn r#executeCommands(
        &self,
        _arg_commands: &[DisplayCommand],
    ) -> binder::Result<Vec<CommandResultPayload>> {
        todo!()
    }
    async fn r#getActiveConfig(&self, _arg_display: i64) -> binder::Result<i32> {
        todo!()
    }
    async fn r#getColorModes(&self, _arg_display: i64) -> binder::Result<Vec<ColorMode>> {
        todo!()
    }
    async fn r#getDataspaceSaturationMatrix(
        &self,
        _arg_dataspace: Dataspace,
    ) -> binder::Result<Vec<f32>> {
        todo!()
    }
    async fn r#getDisplayAttribute(
        &self,
        _arg_display: i64,
        _arg_config: i32,
        _arg_attribute: DisplayAttribute,
    ) -> binder::Result<i32> {
        todo!()
    }
    async fn r#getDisplayCapabilities(
        &self,
        _arg_display: i64,
    ) -> binder::Result<Vec<DisplayCapability>> {
        todo!()
    }
    async fn r#getDisplayConfigs(&self, _arg_display: i64) -> binder::Result<Vec<i32>> {
        todo!()
    }
    async fn r#getDisplayConnectionType(
        &self,
        _arg_display: i64,
    ) -> binder::Result<DisplayConnectionType> {
        todo!()
    }
    async fn r#getDisplayIdentificationData(
        &self,
        _arg_display: i64,
    ) -> binder::Result<DisplayIdentification> {
        Err(IComposerClient::EX_UNSUPPORTED.into())
    }
    async fn r#getDisplayName(&self, _arg_display: i64) -> binder::Result<String> {
        todo!()
    }
    async fn r#getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<i32> {
        todo!()
    }
    async fn r#getDisplayedContentSample(
        &self,
        _arg_display: i64,
        _arg_maxFrames: i64,
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
        todo!()
    }
    async fn r#getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<HdrCapabilities> {
        todo!()
    }
    async fn r#getMaxVirtualDisplayCount(&self) -> binder::Result<i32> {
        todo!()
    }
    async fn r#getPerFrameMetadataKeys(
        &self,
        _arg_display: i64,
    ) -> binder::Result<Vec<PerFrameMetadataKey>> {
        todo!()
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
        todo!()
    }
    async fn r#getSupportedContentTypes(
        &self,
        _arg_display: i64,
    ) -> binder::Result<Vec<ContentType>> {
        todo!()
    }
    async fn r#getDisplayDecorationSupport(
        &self,
        _arg_display: i64,
    ) -> binder::Result<Option<DisplayDecorationSupport>> {
        todo!()
    }
    async fn r#registerCallback(
        &self,
        callback: &binder::Strong<dyn IComposerCallback>,
    ) -> binder::Result<()> {
        callback.onHotplug( 1, true).unwrap();
        Ok(())
    }
    async fn r#setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> {
        todo!()
    }
    async fn r#setActiveConfigWithConstraints(
        &self,
        _arg_display: i64,
        _arg_config: i32,
        _arg_vsyncPeriodChangeConstraints: &VsyncPeriodChangeConstraints,
    ) -> binder::Result<VsyncPeriodChangeTimeline> {
        todo!()
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
        _arg_componentMask: FormatColorComponent,
        _arg_maxFrames: i64,
    ) -> binder::Result<()> {
        todo!()
    }
    async fn r#setPowerMode(&self, _arg_display: i64, _arg_mode: PowerMode) -> binder::Result<()> {
        todo!()
    }
    async fn r#setReadbackBuffer(
        &self,
        _arg_display: i64,
        _arg_buffer: &NativeHandle,
        _arg_releaseFence: Option<&binder::ParcelFileDescriptor>,
    ) -> binder::Result<()> {
        todo!()
    }
    async fn r#setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> {
        todo!()
    }
    async fn r#setIdleTimerEnabled(
        &self,
        _arg_display: i64,
        _arg_timeoutMs: i32,
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
        _arg_conversionStrategy: &HdrConversionStrategy,
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
}
