/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/mnishant/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl --lang=rust aidl/android/hardware/graphics/composer3/Buffer.aidl aidl/android/hardware/graphics/composer3/Capability.aidl aidl/android/hardware/graphics/composer3/ChangedCompositionLayer.aidl aidl/android/hardware/graphics/composer3/ChangedCompositionTypes.aidl aidl/android/hardware/graphics/composer3/ClientTarget.aidl aidl/android/hardware/graphics/composer3/ClientTargetProperty.aidl aidl/android/hardware/graphics/composer3/ClientTargetPropertyWithBrightness.aidl aidl/android/hardware/graphics/composer3/ClockMonotonicTimestamp.aidl aidl/android/hardware/graphics/composer3/Color.aidl aidl/android/hardware/graphics/composer3/ColorMode.aidl aidl/android/hardware/graphics/composer3/CommandError.aidl aidl/android/hardware/graphics/composer3/CommandResultPayload.aidl aidl/android/hardware/graphics/composer3/Composition.aidl aidl/android/hardware/graphics/composer3/ContentType.aidl aidl/android/hardware/graphics/composer3/DimmingStage.aidl aidl/android/hardware/graphics/composer3/DisplayAttribute.aidl aidl/android/hardware/graphics/composer3/DisplayBrightness.aidl aidl/android/hardware/graphics/composer3/DisplayCapability.aidl aidl/android/hardware/graphics/composer3/DisplayCommand.aidl aidl/android/hardware/graphics/composer3/DisplayConnectionType.aidl aidl/android/hardware/graphics/composer3/DisplayContentSample.aidl aidl/android/hardware/graphics/composer3/DisplayContentSamplingAttributes.aidl aidl/android/hardware/graphics/composer3/DisplayIdentification.aidl aidl/android/hardware/graphics/composer3/DisplayRequest.aidl aidl/android/hardware/graphics/composer3/FormatColorComponent.aidl aidl/android/hardware/graphics/composer3/HdrCapabilities.aidl aidl/android/hardware/graphics/composer3/IComposer.aidl aidl/android/hardware/graphics/composer3/IComposerCallback.aidl aidl/android/hardware/graphics/composer3/IComposerClient.aidl aidl/android/hardware/graphics/composer3/LayerBrightness.aidl aidl/android/hardware/graphics/composer3/LayerCommand.aidl aidl/android/hardware/graphics/composer3/OverlayProperties.aidl aidl/android/hardware/graphics/composer3/ParcelableBlendMode.aidl aidl/android/hardware/graphics/composer3/ParcelableComposition.aidl aidl/android/hardware/graphics/composer3/ParcelableDataspace.aidl aidl/android/hardware/graphics/composer3/ParcelableTransform.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadata.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadataBlob.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadataKey.aidl aidl/android/hardware/graphics/composer3/PlaneAlpha.aidl aidl/android/hardware/graphics/composer3/PowerMode.aidl aidl/android/hardware/graphics/composer3/PresentFence.aidl aidl/android/hardware/graphics/composer3/PresentOrValidate.aidl aidl/android/hardware/graphics/composer3/ReadbackBufferAttributes.aidl aidl/android/hardware/graphics/composer3/RefreshRateChangedDebugData.aidl aidl/android/hardware/graphics/composer3/ReleaseFences.aidl aidl/android/hardware/graphics/composer3/RenderIntent.aidl aidl/android/hardware/graphics/composer3/VirtualDisplay.aidl aidl/android/hardware/graphics/composer3/VsyncPeriodChangeConstraints.aidl aidl/android/hardware/graphics/composer3/VsyncPeriodChangeTimeline.aidl aidl/android/hardware/graphics/composer3/ZOrder.aidl -I aidl/ --stability=vintf --structured -o android_hardware_graphics_composer3/src/aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
declare_binder_interface! {
  IComposerClient["android.hardware.graphics.composer3.IComposerClient"] {
    native: BnComposerClient(on_transact),
    proxy: BpComposerClient {
    },
    async: IComposerClientAsync,
    stability: binder::binder_impl::Stability::Vintf,
  }
}
pub trait IComposerClient: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposerClient" }
  fn r#createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::Result<i64>;
  fn r#createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay>;
  fn r#destroyLayer(&self, _arg_display: i64, _arg_layer: i64) -> binder::Result<()>;
  fn r#destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<()>;
  fn r#executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>>;
  fn r#getActiveConfig(&self, _arg_display: i64) -> binder::Result<i32>;
  fn r#getColorModes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>>;
  fn r#getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::Result<Vec<f32>>;
  fn r#getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::Result<i32>;
  fn r#getDisplayCapabilities(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>>;
  fn r#getDisplayConfigs(&self, _arg_display: i64) -> binder::Result<Vec<i32>>;
  fn r#getDisplayConnectionType(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType>;
  fn r#getDisplayIdentificationData(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification>;
  fn r#getDisplayName(&self, _arg_display: i64) -> binder::Result<String>;
  fn r#getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<i32>;
  fn r#getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample>;
  fn r#getDisplayedContentSamplingAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes>;
  fn r#getDisplayPhysicalOrientation(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform>;
  fn r#getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities>;
  fn r#getMaxVirtualDisplayCount(&self) -> binder::Result<i32>;
  fn r#getPerFrameMetadataKeys(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>>;
  fn r#getReadbackBufferAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes>;
  fn r#getReadbackBufferFence(&self, _arg_display: i64) -> binder::Result<Option<binder::ParcelFileDescriptor>>;
  fn r#getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>>;
  fn r#getSupportedContentTypes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>>;
  fn r#getDisplayDecorationSupport(&self, _arg_display: i64) -> binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>>;
  fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::Result<()>;
  fn r#setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()>;
  fn r#setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline>;
  fn r#setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()>;
  fn r#clearBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<()>;
  fn r#getPreferredBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<i32>;
  fn r#setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool) -> binder::Result<()>;
  fn r#setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::Result<()>;
  fn r#setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::Result<()>;
  fn r#setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::Result<()>;
  fn r#setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::Result<()>;
  fn r#setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::Result<()>;
  fn r#setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>) -> binder::Result<()>;
  fn r#setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()>;
  fn r#setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::Result<()>;
  fn r#getOverlaySupport(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties>;
  fn r#getHdrConversionCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>>;
  fn r#setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr>;
  fn r#setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()>;
  fn r#setLayerName(&self, _arg_display: i64, _arg_layer: i64, _arg_name: &str) -> binder::Result<()>;
  fn getDefaultImpl() -> IComposerClientDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: IComposerClientDefaultRef) -> IComposerClientDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait IComposerClientAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposerClient" }
  fn r#createLayer<'a>(&'a self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::BoxFuture<'a, binder::Result<i64>>;
  fn r#createVirtualDisplay<'a>(&'a self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay>>;
  fn r#destroyLayer<'a>(&'a self, _arg_display: i64, _arg_layer: i64) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#destroyVirtualDisplay<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#executeCommands<'a>(&'a self, _arg_commands: &'a [crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>>>;
  fn r#getActiveConfig<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<i32>>;
  fn r#getColorModes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>>>;
  fn r#getDataspaceSaturationMatrix<'a>(&'a self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::BoxFuture<'a, binder::Result<Vec<f32>>>;
  fn r#getDisplayAttribute<'a>(&'a self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::BoxFuture<'a, binder::Result<i32>>;
  fn r#getDisplayCapabilities<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>>>;
  fn r#getDisplayConfigs<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<i32>>>;
  fn r#getDisplayConnectionType<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType>>;
  fn r#getDisplayIdentificationData<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification>>;
  fn r#getDisplayName<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<String>>;
  fn r#getDisplayVsyncPeriod<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<i32>>;
  fn r#getDisplayedContentSample<'a>(&'a self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample>>;
  fn r#getDisplayedContentSamplingAttributes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes>>;
  fn r#getDisplayPhysicalOrientation<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform>>;
  fn r#getHdrCapabilities<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities>>;
  fn r#getMaxVirtualDisplayCount<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>>;
  fn r#getPerFrameMetadataKeys<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>>>;
  fn r#getReadbackBufferAttributes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes>>;
  fn r#getReadbackBufferFence<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Option<binder::ParcelFileDescriptor>>>;
  fn r#getRenderIntents<'a>(&'a self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>>>;
  fn r#getSupportedContentTypes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>>>;
  fn r#getDisplayDecorationSupport<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>>>;
  fn r#registerCallback<'a>(&'a self, _arg_callback: &'a binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#setActiveConfig<'a>(&'a self, _arg_display: i64, _arg_config: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#setActiveConfigWithConstraints<'a>(&'a self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &'a crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline>>;
  fn r#setBootDisplayConfig<'a>(&'a self, _arg_display: i64, _arg_config: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#clearBootDisplayConfig<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#getPreferredBootDisplayConfig<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<i32>>;
  fn r#setAutoLowLatencyMode<'a>(&'a self, _arg_display: i64, _arg_on: bool) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#setClientTargetSlotCount<'a>(&'a self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#setColorMode<'a>(&'a self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#setContentType<'a>(&'a self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#setDisplayedContentSamplingEnabled<'a>(&'a self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#setPowerMode<'a>(&'a self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#setReadbackBuffer<'a>(&'a self, _arg_display: i64, _arg_buffer: &'a crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&'a binder::ParcelFileDescriptor>) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#setVsyncEnabled<'a>(&'a self, _arg_display: i64, _arg_enabled: bool) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#setIdleTimerEnabled<'a>(&'a self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#getOverlaySupport<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties>>;
  fn r#getHdrConversionCapabilities<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>>>;
  fn r#setHdrConversionStrategy<'a>(&'a self, _arg_conversionStrategy: &'a crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr>>;
  fn r#setRefreshRateChangedCallbackDebugEnabled<'a>(&'a self, _arg_display: i64, _arg_enabled: bool) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#setLayerName<'a>(&'a self, _arg_display: i64, _arg_layer: i64, _arg_name: &'a str) -> binder::BoxFuture<'a, binder::Result<()>>;
}
#[::async_trait::async_trait]
pub trait IComposerClientAsyncServer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.graphics.composer3.IComposerClient" }
  async fn r#createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::Result<i64>;
  async fn r#createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay>;
  async fn r#destroyLayer(&self, _arg_display: i64, _arg_layer: i64) -> binder::Result<()>;
  async fn r#destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<()>;
  async fn r#executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>>;
  async fn r#getActiveConfig(&self, _arg_display: i64) -> binder::Result<i32>;
  async fn r#getColorModes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>>;
  async fn r#getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::Result<Vec<f32>>;
  async fn r#getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::Result<i32>;
  async fn r#getDisplayCapabilities(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>>;
  async fn r#getDisplayConfigs(&self, _arg_display: i64) -> binder::Result<Vec<i32>>;
  async fn r#getDisplayConnectionType(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType>;
  async fn r#getDisplayIdentificationData(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification>;
  async fn r#getDisplayName(&self, _arg_display: i64) -> binder::Result<String>;
  async fn r#getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<i32>;
  async fn r#getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample>;
  async fn r#getDisplayedContentSamplingAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes>;
  async fn r#getDisplayPhysicalOrientation(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform>;
  async fn r#getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities>;
  async fn r#getMaxVirtualDisplayCount(&self) -> binder::Result<i32>;
  async fn r#getPerFrameMetadataKeys(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>>;
  async fn r#getReadbackBufferAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes>;
  async fn r#getReadbackBufferFence(&self, _arg_display: i64) -> binder::Result<Option<binder::ParcelFileDescriptor>>;
  async fn r#getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>>;
  async fn r#getSupportedContentTypes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>>;
  async fn r#getDisplayDecorationSupport(&self, _arg_display: i64) -> binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>>;
  async fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::Result<()>;
  async fn r#setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()>;
  async fn r#setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline>;
  async fn r#setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()>;
  async fn r#clearBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<()>;
  async fn r#getPreferredBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<i32>;
  async fn r#setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool) -> binder::Result<()>;
  async fn r#setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::Result<()>;
  async fn r#setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::Result<()>;
  async fn r#setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::Result<()>;
  async fn r#setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::Result<()>;
  async fn r#setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::Result<()>;
  async fn r#setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>) -> binder::Result<()>;
  async fn r#setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()>;
  async fn r#setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::Result<()>;
  async fn r#getOverlaySupport(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties>;
  async fn r#getHdrConversionCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>>;
  async fn r#setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr>;
  async fn r#setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()>;
  async fn r#setLayerName(&self, _arg_display: i64, _arg_layer: i64, _arg_name: &str) -> binder::Result<()>;
}
impl BnComposerClient {
  /// Create a new async binder service.
  pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IComposerClient>
  where
    T: IComposerClientAsyncServer + binder::Interface + Send + Sync + 'static,
    R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
  {
    struct Wrapper<T, R> {
      _inner: T,
      _rt: R,
    }
    impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync + 'static {
      fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
      fn dump(&self, _writer: &mut dyn std::io::Write, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_writer, _args) }
    }
    impl<T, R> IComposerClient for Wrapper<T, R>
    where
      T: IComposerClientAsyncServer + Send + Sync + 'static,
      R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
      fn r#createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::Result<i64> {
        self._rt.block_on(self._inner.r#createLayer(_arg_display, _arg_bufferSlotCount))
      }
      fn r#createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay> {
        self._rt.block_on(self._inner.r#createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount))
      }
      fn r#destroyLayer(&self, _arg_display: i64, _arg_layer: i64) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#destroyLayer(_arg_display, _arg_layer))
      }
      fn r#destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#destroyVirtualDisplay(_arg_display))
      }
      fn r#executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>> {
        self._rt.block_on(self._inner.r#executeCommands(_arg_commands))
      }
      fn r#getActiveConfig(&self, _arg_display: i64) -> binder::Result<i32> {
        self._rt.block_on(self._inner.r#getActiveConfig(_arg_display))
      }
      fn r#getColorModes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>> {
        self._rt.block_on(self._inner.r#getColorModes(_arg_display))
      }
      fn r#getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::Result<Vec<f32>> {
        self._rt.block_on(self._inner.r#getDataspaceSaturationMatrix(_arg_dataspace))
      }
      fn r#getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::Result<i32> {
        self._rt.block_on(self._inner.r#getDisplayAttribute(_arg_display, _arg_config, _arg_attribute))
      }
      fn r#getDisplayCapabilities(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>> {
        self._rt.block_on(self._inner.r#getDisplayCapabilities(_arg_display))
      }
      fn r#getDisplayConfigs(&self, _arg_display: i64) -> binder::Result<Vec<i32>> {
        self._rt.block_on(self._inner.r#getDisplayConfigs(_arg_display))
      }
      fn r#getDisplayConnectionType(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType> {
        self._rt.block_on(self._inner.r#getDisplayConnectionType(_arg_display))
      }
      fn r#getDisplayIdentificationData(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification> {
        self._rt.block_on(self._inner.r#getDisplayIdentificationData(_arg_display))
      }
      fn r#getDisplayName(&self, _arg_display: i64) -> binder::Result<String> {
        self._rt.block_on(self._inner.r#getDisplayName(_arg_display))
      }
      fn r#getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<i32> {
        self._rt.block_on(self._inner.r#getDisplayVsyncPeriod(_arg_display))
      }
      fn r#getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample> {
        self._rt.block_on(self._inner.r#getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp))
      }
      fn r#getDisplayedContentSamplingAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes> {
        self._rt.block_on(self._inner.r#getDisplayedContentSamplingAttributes(_arg_display))
      }
      fn r#getDisplayPhysicalOrientation(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform> {
        self._rt.block_on(self._inner.r#getDisplayPhysicalOrientation(_arg_display))
      }
      fn r#getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities> {
        self._rt.block_on(self._inner.r#getHdrCapabilities(_arg_display))
      }
      fn r#getMaxVirtualDisplayCount(&self) -> binder::Result<i32> {
        self._rt.block_on(self._inner.r#getMaxVirtualDisplayCount())
      }
      fn r#getPerFrameMetadataKeys(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>> {
        self._rt.block_on(self._inner.r#getPerFrameMetadataKeys(_arg_display))
      }
      fn r#getReadbackBufferAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes> {
        self._rt.block_on(self._inner.r#getReadbackBufferAttributes(_arg_display))
      }
      fn r#getReadbackBufferFence(&self, _arg_display: i64) -> binder::Result<Option<binder::ParcelFileDescriptor>> {
        self._rt.block_on(self._inner.r#getReadbackBufferFence(_arg_display))
      }
      fn r#getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>> {
        self._rt.block_on(self._inner.r#getRenderIntents(_arg_display, _arg_mode))
      }
      fn r#getSupportedContentTypes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>> {
        self._rt.block_on(self._inner.r#getSupportedContentTypes(_arg_display))
      }
      fn r#getDisplayDecorationSupport(&self, _arg_display: i64) -> binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>> {
        self._rt.block_on(self._inner.r#getDisplayDecorationSupport(_arg_display))
      }
      fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#registerCallback(_arg_callback))
      }
      fn r#setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#setActiveConfig(_arg_display, _arg_config))
      }
      fn r#setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline> {
        self._rt.block_on(self._inner.r#setActiveConfigWithConstraints(_arg_display, _arg_config, _arg_vsyncPeriodChangeConstraints))
      }
      fn r#setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#setBootDisplayConfig(_arg_display, _arg_config))
      }
      fn r#clearBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#clearBootDisplayConfig(_arg_display))
      }
      fn r#getPreferredBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<i32> {
        self._rt.block_on(self._inner.r#getPreferredBootDisplayConfig(_arg_display))
      }
      fn r#setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#setAutoLowLatencyMode(_arg_display, _arg_on))
      }
      fn r#setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount))
      }
      fn r#setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#setColorMode(_arg_display, _arg_mode, _arg_intent))
      }
      fn r#setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#setContentType(_arg_display, _arg_type))
      }
      fn r#setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames))
      }
      fn r#setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#setPowerMode(_arg_display, _arg_mode))
      }
      fn r#setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#setReadbackBuffer(_arg_display, _arg_buffer, _arg_releaseFence))
      }
      fn r#setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#setVsyncEnabled(_arg_display, _arg_enabled))
      }
      fn r#setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#setIdleTimerEnabled(_arg_display, _arg_timeoutMs))
      }
      fn r#getOverlaySupport(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties> {
        self._rt.block_on(self._inner.r#getOverlaySupport())
      }
      fn r#getHdrConversionCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>> {
        self._rt.block_on(self._inner.r#getHdrConversionCapabilities())
      }
      fn r#setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr> {
        self._rt.block_on(self._inner.r#setHdrConversionStrategy(_arg_conversionStrategy))
      }
      fn r#setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled))
      }
      fn r#setLayerName(&self, _arg_display: i64, _arg_layer: i64, _arg_name: &str) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#setLayerName(_arg_display, _arg_layer, _arg_name))
      }
    }
    let wrapped = Wrapper { _inner: inner, _rt: rt };
    Self::new_binder(wrapped, features)
  }
}
pub trait IComposerClientDefault: Send + Sync {
  fn r#createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::Result<i64> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#destroyLayer(&self, _arg_display: i64, _arg_layer: i64) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getActiveConfig(&self, _arg_display: i64) -> binder::Result<i32> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getColorModes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::Result<Vec<f32>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::Result<i32> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getDisplayCapabilities(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getDisplayConfigs(&self, _arg_display: i64) -> binder::Result<Vec<i32>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getDisplayConnectionType(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getDisplayIdentificationData(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getDisplayName(&self, _arg_display: i64) -> binder::Result<String> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<i32> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getDisplayedContentSamplingAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getDisplayPhysicalOrientation(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getMaxVirtualDisplayCount(&self) -> binder::Result<i32> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getPerFrameMetadataKeys(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getReadbackBufferAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getReadbackBufferFence(&self, _arg_display: i64) -> binder::Result<Option<binder::ParcelFileDescriptor>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getSupportedContentTypes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getDisplayDecorationSupport(&self, _arg_display: i64) -> binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#clearBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getPreferredBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<i32> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getOverlaySupport(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getHdrConversionCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#setLayerName(&self, _arg_display: i64, _arg_layer: i64, _arg_name: &str) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const r#createLayer: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
  pub const r#createVirtualDisplay: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
  pub const r#destroyLayer: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
  pub const r#destroyVirtualDisplay: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
  pub const r#executeCommands: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
  pub const r#getActiveConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
  pub const r#getColorModes: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
  pub const r#getDataspaceSaturationMatrix: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
  pub const r#getDisplayAttribute: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
  pub const r#getDisplayCapabilities: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
  pub const r#getDisplayConfigs: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
  pub const r#getDisplayConnectionType: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
  pub const r#getDisplayIdentificationData: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
  pub const r#getDisplayName: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
  pub const r#getDisplayVsyncPeriod: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
  pub const r#getDisplayedContentSample: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
  pub const r#getDisplayedContentSamplingAttributes: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
  pub const r#getDisplayPhysicalOrientation: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 17;
  pub const r#getHdrCapabilities: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 18;
  pub const r#getMaxVirtualDisplayCount: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 19;
  pub const r#getPerFrameMetadataKeys: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 20;
  pub const r#getReadbackBufferAttributes: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 21;
  pub const r#getReadbackBufferFence: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 22;
  pub const r#getRenderIntents: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 23;
  pub const r#getSupportedContentTypes: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 24;
  pub const r#getDisplayDecorationSupport: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 25;
  pub const r#registerCallback: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 26;
  pub const r#setActiveConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 27;
  pub const r#setActiveConfigWithConstraints: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 28;
  pub const r#setBootDisplayConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 29;
  pub const r#clearBootDisplayConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 30;
  pub const r#getPreferredBootDisplayConfig: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 31;
  pub const r#setAutoLowLatencyMode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 32;
  pub const r#setClientTargetSlotCount: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 33;
  pub const r#setColorMode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 34;
  pub const r#setContentType: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 35;
  pub const r#setDisplayedContentSamplingEnabled: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 36;
  pub const r#setPowerMode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 37;
  pub const r#setReadbackBuffer: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 38;
  pub const r#setVsyncEnabled: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 39;
  pub const r#setIdleTimerEnabled: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 40;
  pub const r#getOverlaySupport: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 41;
  pub const r#getHdrConversionCapabilities: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 42;
  pub const r#setHdrConversionStrategy: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 43;
  pub const r#setRefreshRateChangedCallbackDebugEnabled: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 44;
  pub const r#setLayerName: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 45;
}
pub type IComposerClientDefaultRef = Option<std::sync::Arc<dyn IComposerClientDefault>>;
static DEFAULT_IMPL: std::sync::Mutex<IComposerClientDefaultRef> = std::sync::Mutex::new(None);
pub const r#EX_BAD_CONFIG: i32 = 1;
pub const r#EX_BAD_DISPLAY: i32 = 2;
pub const r#EX_BAD_LAYER: i32 = 3;
pub const r#EX_BAD_PARAMETER: i32 = 4;
pub const r#EX_RESERVED: i32 = 5;
pub const r#EX_NO_RESOURCES: i32 = 6;
pub const r#EX_NOT_VALIDATED: i32 = 7;
pub const r#EX_UNSUPPORTED: i32 = 8;
pub const r#EX_SEAMLESS_NOT_ALLOWED: i32 = 9;
pub const r#EX_SEAMLESS_NOT_POSSIBLE: i32 = 10;
pub const r#INVALID_CONFIGURATION: i32 = 2147483647;
impl BpComposerClient {
  fn build_parcel_createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_bufferSlotCount)?;
    Ok(aidl_data)
  }
  fn read_response_createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i64> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#createLayer(_arg_display, _arg_bufferSlotCount);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: i64 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_width)?;
    aidl_data.write(&_arg_height)?;
    aidl_data.write(&_arg_formatHint)?;
    aidl_data.write(&_arg_outputBufferSlotCount)?;
    Ok(aidl_data)
  }
  fn read_response_createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_destroyLayer(&self, _arg_display: i64, _arg_layer: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_layer)?;
    Ok(aidl_data)
  }
  fn read_response_destroyLayer(&self, _arg_display: i64, _arg_layer: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#destroyLayer(_arg_display, _arg_layer);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_destroyVirtualDisplay(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#destroyVirtualDisplay(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_commands)?;
    Ok(aidl_data)
  }
  fn read_response_executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#executeCommands(_arg_commands);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getActiveConfig(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getActiveConfig(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getActiveConfig(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: i32 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getColorModes(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getColorModes(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getColorModes(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_dataspace)?;
    Ok(aidl_data)
  }
  fn read_response_getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<f32>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getDataspaceSaturationMatrix(_arg_dataspace);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<f32> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_config)?;
    aidl_data.write(&_arg_attribute)?;
    Ok(aidl_data)
  }
  fn read_response_getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getDisplayAttribute(_arg_display, _arg_config, _arg_attribute);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: i32 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getDisplayCapabilities(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getDisplayCapabilities(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getDisplayCapabilities(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getDisplayConfigs(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getDisplayConfigs(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<i32>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getDisplayConfigs(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<i32> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getDisplayConnectionType(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getDisplayConnectionType(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getDisplayConnectionType(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getDisplayIdentificationData(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getDisplayIdentificationData(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getDisplayIdentificationData(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getDisplayName(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getDisplayName(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getDisplayName(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: String = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getDisplayVsyncPeriod(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getDisplayVsyncPeriod(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: i32 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_maxFrames)?;
    aidl_data.write(&_arg_timestamp)?;
    Ok(aidl_data)
  }
  fn read_response_getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getDisplayedContentSamplingAttributes(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getDisplayedContentSamplingAttributes(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getDisplayedContentSamplingAttributes(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getDisplayPhysicalOrientation(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getDisplayPhysicalOrientation(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getDisplayPhysicalOrientation(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getHdrCapabilities(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getHdrCapabilities(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getMaxVirtualDisplayCount(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getMaxVirtualDisplayCount(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getMaxVirtualDisplayCount();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: i32 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getPerFrameMetadataKeys(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getPerFrameMetadataKeys(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getPerFrameMetadataKeys(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getReadbackBufferAttributes(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getReadbackBufferAttributes(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getReadbackBufferAttributes(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getReadbackBufferFence(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getReadbackBufferFence(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Option<binder::ParcelFileDescriptor>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getReadbackBufferFence(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<binder::ParcelFileDescriptor> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_mode)?;
    Ok(aidl_data)
  }
  fn read_response_getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getRenderIntents(_arg_display, _arg_mode);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getSupportedContentTypes(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getSupportedContentTypes(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getSupportedContentTypes(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getDisplayDecorationSupport(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getDisplayDecorationSupport(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getDisplayDecorationSupport(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_callback)?;
    Ok(aidl_data)
  }
  fn read_response_registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#registerCallback(_arg_callback);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_config)?;
    Ok(aidl_data)
  }
  fn read_response_setActiveConfig(&self, _arg_display: i64, _arg_config: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#setActiveConfig(_arg_display, _arg_config);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_config)?;
    aidl_data.write(_arg_vsyncPeriodChangeConstraints)?;
    Ok(aidl_data)
  }
  fn read_response_setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#setActiveConfigWithConstraints(_arg_display, _arg_config, _arg_vsyncPeriodChangeConstraints);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_config)?;
    Ok(aidl_data)
  }
  fn read_response_setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#setBootDisplayConfig(_arg_display, _arg_config);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_clearBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_clearBootDisplayConfig(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#clearBootDisplayConfig(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_getPreferredBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    Ok(aidl_data)
  }
  fn read_response_getPreferredBootDisplayConfig(&self, _arg_display: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getPreferredBootDisplayConfig(_arg_display);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: i32 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_on)?;
    Ok(aidl_data)
  }
  fn read_response_setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#setAutoLowLatencyMode(_arg_display, _arg_on);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_clientTargetSlotCount)?;
    Ok(aidl_data)
  }
  fn read_response_setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_mode)?;
    aidl_data.write(&_arg_intent)?;
    Ok(aidl_data)
  }
  fn read_response_setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#setColorMode(_arg_display, _arg_mode, _arg_intent);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_type)?;
    Ok(aidl_data)
  }
  fn read_response_setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#setContentType(_arg_display, _arg_type);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_enable)?;
    aidl_data.write(&_arg_componentMask)?;
    aidl_data.write(&_arg_maxFrames)?;
    Ok(aidl_data)
  }
  fn read_response_setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_mode)?;
    Ok(aidl_data)
  }
  fn read_response_setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#setPowerMode(_arg_display, _arg_mode);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(_arg_buffer)?;
    aidl_data.write(&_arg_releaseFence)?;
    Ok(aidl_data)
  }
  fn read_response_setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#setReadbackBuffer(_arg_display, _arg_buffer, _arg_releaseFence);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_enabled)?;
    Ok(aidl_data)
  }
  fn read_response_setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#setVsyncEnabled(_arg_display, _arg_enabled);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_timeoutMs)?;
    Ok(aidl_data)
  }
  fn read_response_setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#setIdleTimerEnabled(_arg_display, _arg_timeoutMs);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_getOverlaySupport(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getOverlaySupport(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getOverlaySupport();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getHdrConversionCapabilities(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getHdrConversionCapabilities(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#getHdrConversionCapabilities();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_conversionStrategy)?;
    Ok(aidl_data)
  }
  fn read_response_setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#setHdrConversionStrategy(_arg_conversionStrategy);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_enabled)?;
    Ok(aidl_data)
  }
  fn read_response_setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_setLayerName(&self, _arg_display: i64, _arg_layer: i64, _arg_name: &str) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_display)?;
    aidl_data.write(&_arg_layer)?;
    aidl_data.write(_arg_name)?;
    Ok(aidl_data)
  }
  fn read_response_setLayerName(&self, _arg_display: i64, _arg_layer: i64, _arg_name: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IComposerClient>::getDefaultImpl() {
        return _aidl_default_impl.r#setLayerName(_arg_display, _arg_layer, _arg_name);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
}
impl IComposerClient for BpComposerClient {
  fn r#createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::Result<i64> {
    let _aidl_data = self.build_parcel_createLayer(_arg_display, _arg_bufferSlotCount)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#createLayer, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_createLayer(_arg_display, _arg_bufferSlotCount, _aidl_reply)
  }
  fn r#createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay> {
    let _aidl_data = self.build_parcel_createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#createVirtualDisplay, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount, _aidl_reply)
  }
  fn r#destroyLayer(&self, _arg_display: i64, _arg_layer: i64) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_destroyLayer(_arg_display, _arg_layer)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#destroyLayer, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_destroyLayer(_arg_display, _arg_layer, _aidl_reply)
  }
  fn r#destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_destroyVirtualDisplay(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#destroyVirtualDisplay, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_destroyVirtualDisplay(_arg_display, _aidl_reply)
  }
  fn r#executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>> {
    let _aidl_data = self.build_parcel_executeCommands(_arg_commands)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#executeCommands, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_executeCommands(_arg_commands, _aidl_reply)
  }
  fn r#getActiveConfig(&self, _arg_display: i64) -> binder::Result<i32> {
    let _aidl_data = self.build_parcel_getActiveConfig(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getActiveConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getActiveConfig(_arg_display, _aidl_reply)
  }
  fn r#getColorModes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>> {
    let _aidl_data = self.build_parcel_getColorModes(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getColorModes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getColorModes(_arg_display, _aidl_reply)
  }
  fn r#getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::Result<Vec<f32>> {
    let _aidl_data = self.build_parcel_getDataspaceSaturationMatrix(_arg_dataspace)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getDataspaceSaturationMatrix, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getDataspaceSaturationMatrix(_arg_dataspace, _aidl_reply)
  }
  fn r#getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::Result<i32> {
    let _aidl_data = self.build_parcel_getDisplayAttribute(_arg_display, _arg_config, _arg_attribute)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayAttribute, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getDisplayAttribute(_arg_display, _arg_config, _arg_attribute, _aidl_reply)
  }
  fn r#getDisplayCapabilities(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>> {
    let _aidl_data = self.build_parcel_getDisplayCapabilities(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getDisplayCapabilities(_arg_display, _aidl_reply)
  }
  fn r#getDisplayConfigs(&self, _arg_display: i64) -> binder::Result<Vec<i32>> {
    let _aidl_data = self.build_parcel_getDisplayConfigs(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayConfigs, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getDisplayConfigs(_arg_display, _aidl_reply)
  }
  fn r#getDisplayConnectionType(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType> {
    let _aidl_data = self.build_parcel_getDisplayConnectionType(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayConnectionType, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getDisplayConnectionType(_arg_display, _aidl_reply)
  }
  fn r#getDisplayIdentificationData(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification> {
    let _aidl_data = self.build_parcel_getDisplayIdentificationData(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayIdentificationData, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getDisplayIdentificationData(_arg_display, _aidl_reply)
  }
  fn r#getDisplayName(&self, _arg_display: i64) -> binder::Result<String> {
    let _aidl_data = self.build_parcel_getDisplayName(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayName, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getDisplayName(_arg_display, _aidl_reply)
  }
  fn r#getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<i32> {
    let _aidl_data = self.build_parcel_getDisplayVsyncPeriod(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayVsyncPeriod, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getDisplayVsyncPeriod(_arg_display, _aidl_reply)
  }
  fn r#getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample> {
    let _aidl_data = self.build_parcel_getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayedContentSample, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp, _aidl_reply)
  }
  fn r#getDisplayedContentSamplingAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes> {
    let _aidl_data = self.build_parcel_getDisplayedContentSamplingAttributes(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayedContentSamplingAttributes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getDisplayedContentSamplingAttributes(_arg_display, _aidl_reply)
  }
  fn r#getDisplayPhysicalOrientation(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform> {
    let _aidl_data = self.build_parcel_getDisplayPhysicalOrientation(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayPhysicalOrientation, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getDisplayPhysicalOrientation(_arg_display, _aidl_reply)
  }
  fn r#getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities> {
    let _aidl_data = self.build_parcel_getHdrCapabilities(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getHdrCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getHdrCapabilities(_arg_display, _aidl_reply)
  }
  fn r#getMaxVirtualDisplayCount(&self) -> binder::Result<i32> {
    let _aidl_data = self.build_parcel_getMaxVirtualDisplayCount()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getMaxVirtualDisplayCount, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getMaxVirtualDisplayCount(_aidl_reply)
  }
  fn r#getPerFrameMetadataKeys(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>> {
    let _aidl_data = self.build_parcel_getPerFrameMetadataKeys(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getPerFrameMetadataKeys, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getPerFrameMetadataKeys(_arg_display, _aidl_reply)
  }
  fn r#getReadbackBufferAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes> {
    let _aidl_data = self.build_parcel_getReadbackBufferAttributes(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getReadbackBufferAttributes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getReadbackBufferAttributes(_arg_display, _aidl_reply)
  }
  fn r#getReadbackBufferFence(&self, _arg_display: i64) -> binder::Result<Option<binder::ParcelFileDescriptor>> {
    let _aidl_data = self.build_parcel_getReadbackBufferFence(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getReadbackBufferFence, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getReadbackBufferFence(_arg_display, _aidl_reply)
  }
  fn r#getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>> {
    let _aidl_data = self.build_parcel_getRenderIntents(_arg_display, _arg_mode)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getRenderIntents, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getRenderIntents(_arg_display, _arg_mode, _aidl_reply)
  }
  fn r#getSupportedContentTypes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>> {
    let _aidl_data = self.build_parcel_getSupportedContentTypes(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getSupportedContentTypes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getSupportedContentTypes(_arg_display, _aidl_reply)
  }
  fn r#getDisplayDecorationSupport(&self, _arg_display: i64) -> binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>> {
    let _aidl_data = self.build_parcel_getDisplayDecorationSupport(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getDisplayDecorationSupport, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getDisplayDecorationSupport(_arg_display, _aidl_reply)
  }
  fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_registerCallback(_arg_callback)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#registerCallback, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_registerCallback(_arg_callback, _aidl_reply)
  }
  fn r#setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_setActiveConfig(_arg_display, _arg_config)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setActiveConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setActiveConfig(_arg_display, _arg_config, _aidl_reply)
  }
  fn r#setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline> {
    let _aidl_data = self.build_parcel_setActiveConfigWithConstraints(_arg_display, _arg_config, _arg_vsyncPeriodChangeConstraints)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setActiveConfigWithConstraints, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setActiveConfigWithConstraints(_arg_display, _arg_config, _arg_vsyncPeriodChangeConstraints, _aidl_reply)
  }
  fn r#setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_setBootDisplayConfig(_arg_display, _arg_config)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setBootDisplayConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setBootDisplayConfig(_arg_display, _arg_config, _aidl_reply)
  }
  fn r#clearBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_clearBootDisplayConfig(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#clearBootDisplayConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_clearBootDisplayConfig(_arg_display, _aidl_reply)
  }
  fn r#getPreferredBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<i32> {
    let _aidl_data = self.build_parcel_getPreferredBootDisplayConfig(_arg_display)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getPreferredBootDisplayConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getPreferredBootDisplayConfig(_arg_display, _aidl_reply)
  }
  fn r#setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_setAutoLowLatencyMode(_arg_display, _arg_on)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setAutoLowLatencyMode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setAutoLowLatencyMode(_arg_display, _arg_on, _aidl_reply)
  }
  fn r#setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setClientTargetSlotCount, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount, _aidl_reply)
  }
  fn r#setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_setColorMode(_arg_display, _arg_mode, _arg_intent)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setColorMode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setColorMode(_arg_display, _arg_mode, _arg_intent, _aidl_reply)
  }
  fn r#setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_setContentType(_arg_display, _arg_type)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setContentType, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setContentType(_arg_display, _arg_type, _aidl_reply)
  }
  fn r#setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setDisplayedContentSamplingEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames, _aidl_reply)
  }
  fn r#setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_setPowerMode(_arg_display, _arg_mode)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setPowerMode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setPowerMode(_arg_display, _arg_mode, _aidl_reply)
  }
  fn r#setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_setReadbackBuffer(_arg_display, _arg_buffer, _arg_releaseFence)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setReadbackBuffer, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setReadbackBuffer(_arg_display, _arg_buffer, _arg_releaseFence, _aidl_reply)
  }
  fn r#setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_setVsyncEnabled(_arg_display, _arg_enabled)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setVsyncEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setVsyncEnabled(_arg_display, _arg_enabled, _aidl_reply)
  }
  fn r#setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_setIdleTimerEnabled(_arg_display, _arg_timeoutMs)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setIdleTimerEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setIdleTimerEnabled(_arg_display, _arg_timeoutMs, _aidl_reply)
  }
  fn r#getOverlaySupport(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties> {
    let _aidl_data = self.build_parcel_getOverlaySupport()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getOverlaySupport, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getOverlaySupport(_aidl_reply)
  }
  fn r#getHdrConversionCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>> {
    let _aidl_data = self.build_parcel_getHdrConversionCapabilities()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getHdrConversionCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getHdrConversionCapabilities(_aidl_reply)
  }
  fn r#setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr> {
    let _aidl_data = self.build_parcel_setHdrConversionStrategy(_arg_conversionStrategy)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setHdrConversionStrategy, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setHdrConversionStrategy(_arg_conversionStrategy, _aidl_reply)
  }
  fn r#setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setRefreshRateChangedCallbackDebugEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled, _aidl_reply)
  }
  fn r#setLayerName(&self, _arg_display: i64, _arg_layer: i64, _arg_name: &str) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_setLayerName(_arg_display, _arg_layer, _arg_name)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#setLayerName, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_setLayerName(_arg_display, _arg_layer, _arg_name, _aidl_reply)
  }
}
impl<P: binder::BinderAsyncPool> IComposerClientAsync<P> for BpComposerClient {
  fn r#createLayer<'a>(&'a self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::BoxFuture<'a, binder::Result<i64>> {
    let _aidl_data = match self.build_parcel_createLayer(_arg_display, _arg_bufferSlotCount) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#createLayer, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_createLayer(_arg_display, _arg_bufferSlotCount, _aidl_reply)
      }
    )
  }
  fn r#createVirtualDisplay<'a>(&'a self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay>> {
    let _aidl_data = match self.build_parcel_createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#createVirtualDisplay, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount, _aidl_reply)
      }
    )
  }
  fn r#destroyLayer<'a>(&'a self, _arg_display: i64, _arg_layer: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_destroyLayer(_arg_display, _arg_layer) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#destroyLayer, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_destroyLayer(_arg_display, _arg_layer, _aidl_reply)
      }
    )
  }
  fn r#destroyVirtualDisplay<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_destroyVirtualDisplay(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#destroyVirtualDisplay, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_destroyVirtualDisplay(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#executeCommands<'a>(&'a self, _arg_commands: &'a [crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>>> {
    let _aidl_data = match self.build_parcel_executeCommands(_arg_commands) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#executeCommands, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_executeCommands(_arg_commands, _aidl_reply)
      }
    )
  }
  fn r#getActiveConfig<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<i32>> {
    let _aidl_data = match self.build_parcel_getActiveConfig(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getActiveConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getActiveConfig(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getColorModes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>>> {
    let _aidl_data = match self.build_parcel_getColorModes(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getColorModes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getColorModes(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getDataspaceSaturationMatrix<'a>(&'a self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::BoxFuture<'a, binder::Result<Vec<f32>>> {
    let _aidl_data = match self.build_parcel_getDataspaceSaturationMatrix(_arg_dataspace) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getDataspaceSaturationMatrix, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getDataspaceSaturationMatrix(_arg_dataspace, _aidl_reply)
      }
    )
  }
  fn r#getDisplayAttribute<'a>(&'a self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::BoxFuture<'a, binder::Result<i32>> {
    let _aidl_data = match self.build_parcel_getDisplayAttribute(_arg_display, _arg_config, _arg_attribute) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getDisplayAttribute, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getDisplayAttribute(_arg_display, _arg_config, _arg_attribute, _aidl_reply)
      }
    )
  }
  fn r#getDisplayCapabilities<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>>> {
    let _aidl_data = match self.build_parcel_getDisplayCapabilities(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getDisplayCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getDisplayCapabilities(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getDisplayConfigs<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<i32>>> {
    let _aidl_data = match self.build_parcel_getDisplayConfigs(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getDisplayConfigs, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getDisplayConfigs(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getDisplayConnectionType<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType>> {
    let _aidl_data = match self.build_parcel_getDisplayConnectionType(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getDisplayConnectionType, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getDisplayConnectionType(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getDisplayIdentificationData<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification>> {
    let _aidl_data = match self.build_parcel_getDisplayIdentificationData(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getDisplayIdentificationData, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getDisplayIdentificationData(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getDisplayName<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<String>> {
    let _aidl_data = match self.build_parcel_getDisplayName(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getDisplayName, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getDisplayName(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getDisplayVsyncPeriod<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<i32>> {
    let _aidl_data = match self.build_parcel_getDisplayVsyncPeriod(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getDisplayVsyncPeriod, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getDisplayVsyncPeriod(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getDisplayedContentSample<'a>(&'a self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample>> {
    let _aidl_data = match self.build_parcel_getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getDisplayedContentSample, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp, _aidl_reply)
      }
    )
  }
  fn r#getDisplayedContentSamplingAttributes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes>> {
    let _aidl_data = match self.build_parcel_getDisplayedContentSamplingAttributes(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getDisplayedContentSamplingAttributes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getDisplayedContentSamplingAttributes(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getDisplayPhysicalOrientation<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform>> {
    let _aidl_data = match self.build_parcel_getDisplayPhysicalOrientation(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getDisplayPhysicalOrientation, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getDisplayPhysicalOrientation(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getHdrCapabilities<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities>> {
    let _aidl_data = match self.build_parcel_getHdrCapabilities(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getHdrCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getHdrCapabilities(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getMaxVirtualDisplayCount<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
    let _aidl_data = match self.build_parcel_getMaxVirtualDisplayCount() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getMaxVirtualDisplayCount, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getMaxVirtualDisplayCount(_aidl_reply)
      }
    )
  }
  fn r#getPerFrameMetadataKeys<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>>> {
    let _aidl_data = match self.build_parcel_getPerFrameMetadataKeys(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getPerFrameMetadataKeys, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getPerFrameMetadataKeys(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getReadbackBufferAttributes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes>> {
    let _aidl_data = match self.build_parcel_getReadbackBufferAttributes(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getReadbackBufferAttributes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getReadbackBufferAttributes(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getReadbackBufferFence<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Option<binder::ParcelFileDescriptor>>> {
    let _aidl_data = match self.build_parcel_getReadbackBufferFence(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getReadbackBufferFence, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getReadbackBufferFence(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getRenderIntents<'a>(&'a self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>>> {
    let _aidl_data = match self.build_parcel_getRenderIntents(_arg_display, _arg_mode) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getRenderIntents, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getRenderIntents(_arg_display, _arg_mode, _aidl_reply)
      }
    )
  }
  fn r#getSupportedContentTypes<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>>> {
    let _aidl_data = match self.build_parcel_getSupportedContentTypes(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getSupportedContentTypes, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getSupportedContentTypes(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getDisplayDecorationSupport<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>>> {
    let _aidl_data = match self.build_parcel_getDisplayDecorationSupport(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getDisplayDecorationSupport, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getDisplayDecorationSupport(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#registerCallback<'a>(&'a self, _arg_callback: &'a binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_registerCallback(_arg_callback) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#registerCallback, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_registerCallback(_arg_callback, _aidl_reply)
      }
    )
  }
  fn r#setActiveConfig<'a>(&'a self, _arg_display: i64, _arg_config: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_setActiveConfig(_arg_display, _arg_config) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setActiveConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setActiveConfig(_arg_display, _arg_config, _aidl_reply)
      }
    )
  }
  fn r#setActiveConfigWithConstraints<'a>(&'a self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &'a crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline>> {
    let _aidl_data = match self.build_parcel_setActiveConfigWithConstraints(_arg_display, _arg_config, _arg_vsyncPeriodChangeConstraints) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setActiveConfigWithConstraints, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setActiveConfigWithConstraints(_arg_display, _arg_config, _arg_vsyncPeriodChangeConstraints, _aidl_reply)
      }
    )
  }
  fn r#setBootDisplayConfig<'a>(&'a self, _arg_display: i64, _arg_config: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_setBootDisplayConfig(_arg_display, _arg_config) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setBootDisplayConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setBootDisplayConfig(_arg_display, _arg_config, _aidl_reply)
      }
    )
  }
  fn r#clearBootDisplayConfig<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_clearBootDisplayConfig(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#clearBootDisplayConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_clearBootDisplayConfig(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#getPreferredBootDisplayConfig<'a>(&'a self, _arg_display: i64) -> binder::BoxFuture<'a, binder::Result<i32>> {
    let _aidl_data = match self.build_parcel_getPreferredBootDisplayConfig(_arg_display) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getPreferredBootDisplayConfig, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getPreferredBootDisplayConfig(_arg_display, _aidl_reply)
      }
    )
  }
  fn r#setAutoLowLatencyMode<'a>(&'a self, _arg_display: i64, _arg_on: bool) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_setAutoLowLatencyMode(_arg_display, _arg_on) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setAutoLowLatencyMode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setAutoLowLatencyMode(_arg_display, _arg_on, _aidl_reply)
      }
    )
  }
  fn r#setClientTargetSlotCount<'a>(&'a self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setClientTargetSlotCount, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount, _aidl_reply)
      }
    )
  }
  fn r#setColorMode<'a>(&'a self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_setColorMode(_arg_display, _arg_mode, _arg_intent) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setColorMode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setColorMode(_arg_display, _arg_mode, _arg_intent, _aidl_reply)
      }
    )
  }
  fn r#setContentType<'a>(&'a self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_setContentType(_arg_display, _arg_type) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setContentType, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setContentType(_arg_display, _arg_type, _aidl_reply)
      }
    )
  }
  fn r#setDisplayedContentSamplingEnabled<'a>(&'a self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setDisplayedContentSamplingEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames, _aidl_reply)
      }
    )
  }
  fn r#setPowerMode<'a>(&'a self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_setPowerMode(_arg_display, _arg_mode) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setPowerMode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setPowerMode(_arg_display, _arg_mode, _aidl_reply)
      }
    )
  }
  fn r#setReadbackBuffer<'a>(&'a self, _arg_display: i64, _arg_buffer: &'a crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&'a binder::ParcelFileDescriptor>) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_setReadbackBuffer(_arg_display, _arg_buffer, _arg_releaseFence) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setReadbackBuffer, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setReadbackBuffer(_arg_display, _arg_buffer, _arg_releaseFence, _aidl_reply)
      }
    )
  }
  fn r#setVsyncEnabled<'a>(&'a self, _arg_display: i64, _arg_enabled: bool) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_setVsyncEnabled(_arg_display, _arg_enabled) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setVsyncEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setVsyncEnabled(_arg_display, _arg_enabled, _aidl_reply)
      }
    )
  }
  fn r#setIdleTimerEnabled<'a>(&'a self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_setIdleTimerEnabled(_arg_display, _arg_timeoutMs) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setIdleTimerEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setIdleTimerEnabled(_arg_display, _arg_timeoutMs, _aidl_reply)
      }
    )
  }
  fn r#getOverlaySupport<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties>> {
    let _aidl_data = match self.build_parcel_getOverlaySupport() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getOverlaySupport, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getOverlaySupport(_aidl_reply)
      }
    )
  }
  fn r#getHdrConversionCapabilities<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>>> {
    let _aidl_data = match self.build_parcel_getHdrConversionCapabilities() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getHdrConversionCapabilities, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getHdrConversionCapabilities(_aidl_reply)
      }
    )
  }
  fn r#setHdrConversionStrategy<'a>(&'a self, _arg_conversionStrategy: &'a crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr>> {
    let _aidl_data = match self.build_parcel_setHdrConversionStrategy(_arg_conversionStrategy) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setHdrConversionStrategy, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setHdrConversionStrategy(_arg_conversionStrategy, _aidl_reply)
      }
    )
  }
  fn r#setRefreshRateChangedCallbackDebugEnabled<'a>(&'a self, _arg_display: i64, _arg_enabled: bool) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setRefreshRateChangedCallbackDebugEnabled, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled, _aidl_reply)
      }
    )
  }
  fn r#setLayerName<'a>(&'a self, _arg_display: i64, _arg_layer: i64, _arg_name: &'a str) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_setLayerName(_arg_display, _arg_layer, _arg_name) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#setLayerName, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_setLayerName(_arg_display, _arg_layer, _arg_name, _aidl_reply)
      }
    )
  }
}
impl IComposerClient for binder::binder_impl::Binder<BnComposerClient> {
  fn r#createLayer(&self, _arg_display: i64, _arg_bufferSlotCount: i32) -> binder::Result<i64> { self.0.r#createLayer(_arg_display, _arg_bufferSlotCount) }
  fn r#createVirtualDisplay(&self, _arg_width: i32, _arg_height: i32, _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat, _arg_outputBufferSlotCount: i32) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay> { self.0.r#createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount) }
  fn r#destroyLayer(&self, _arg_display: i64, _arg_layer: i64) -> binder::Result<()> { self.0.r#destroyLayer(_arg_display, _arg_layer) }
  fn r#destroyVirtualDisplay(&self, _arg_display: i64) -> binder::Result<()> { self.0.r#destroyVirtualDisplay(_arg_display) }
  fn r#executeCommands(&self, _arg_commands: &[crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload>> { self.0.r#executeCommands(_arg_commands) }
  fn r#getActiveConfig(&self, _arg_display: i64) -> binder::Result<i32> { self.0.r#getActiveConfig(_arg_display) }
  fn r#getColorModes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode>> { self.0.r#getColorModes(_arg_display) }
  fn r#getDataspaceSaturationMatrix(&self, _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace) -> binder::Result<Vec<f32>> { self.0.r#getDataspaceSaturationMatrix(_arg_dataspace) }
  fn r#getDisplayAttribute(&self, _arg_display: i64, _arg_config: i32, _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute) -> binder::Result<i32> { self.0.r#getDisplayAttribute(_arg_display, _arg_config, _arg_attribute) }
  fn r#getDisplayCapabilities(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability>> { self.0.r#getDisplayCapabilities(_arg_display) }
  fn r#getDisplayConfigs(&self, _arg_display: i64) -> binder::Result<Vec<i32>> { self.0.r#getDisplayConfigs(_arg_display) }
  fn r#getDisplayConnectionType(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType> { self.0.r#getDisplayConnectionType(_arg_display) }
  fn r#getDisplayIdentificationData(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_DisplayIdentification> { self.0.r#getDisplayIdentificationData(_arg_display) }
  fn r#getDisplayName(&self, _arg_display: i64) -> binder::Result<String> { self.0.r#getDisplayName(_arg_display) }
  fn r#getDisplayVsyncPeriod(&self, _arg_display: i64) -> binder::Result<i32> { self.0.r#getDisplayVsyncPeriod(_arg_display) }
  fn r#getDisplayedContentSample(&self, _arg_display: i64, _arg_maxFrames: i64, _arg_timestamp: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample> { self.0.r#getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp) }
  fn r#getDisplayedContentSamplingAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_32_DisplayContentSamplingAttributes> { self.0.r#getDisplayedContentSamplingAttributes(_arg_display) }
  fn r#getDisplayPhysicalOrientation(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform> { self.0.r#getDisplayPhysicalOrientation(_arg_display) }
  fn r#getHdrCapabilities(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities> { self.0.r#getHdrCapabilities(_arg_display) }
  fn r#getMaxVirtualDisplayCount(&self) -> binder::Result<i32> { self.0.r#getMaxVirtualDisplayCount() }
  fn r#getPerFrameMetadataKeys(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey>> { self.0.r#getPerFrameMetadataKeys(_arg_display) }
  fn r#getReadbackBufferAttributes(&self, _arg_display: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes> { self.0.r#getReadbackBufferAttributes(_arg_display) }
  fn r#getReadbackBufferFence(&self, _arg_display: i64) -> binder::Result<Option<binder::ParcelFileDescriptor>> { self.0.r#getReadbackBufferFence(_arg_display) }
  fn r#getRenderIntents(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent>> { self.0.r#getRenderIntents(_arg_display, _arg_mode) }
  fn r#getSupportedContentTypes(&self, _arg_display: i64) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType>> { self.0.r#getSupportedContentTypes(_arg_display) }
  fn r#getDisplayDecorationSupport(&self, _arg_display: i64) -> binder::Result<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport>> { self.0.r#getDisplayDecorationSupport(_arg_display) }
  fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback>) -> binder::Result<()> { self.0.r#registerCallback(_arg_callback) }
  fn r#setActiveConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> { self.0.r#setActiveConfig(_arg_display, _arg_config) }
  fn r#setActiveConfigWithConstraints(&self, _arg_display: i64, _arg_config: i32, _arg_vsyncPeriodChangeConstraints: &crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline> { self.0.r#setActiveConfigWithConstraints(_arg_display, _arg_config, _arg_vsyncPeriodChangeConstraints) }
  fn r#setBootDisplayConfig(&self, _arg_display: i64, _arg_config: i32) -> binder::Result<()> { self.0.r#setBootDisplayConfig(_arg_display, _arg_config) }
  fn r#clearBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<()> { self.0.r#clearBootDisplayConfig(_arg_display) }
  fn r#getPreferredBootDisplayConfig(&self, _arg_display: i64) -> binder::Result<i32> { self.0.r#getPreferredBootDisplayConfig(_arg_display) }
  fn r#setAutoLowLatencyMode(&self, _arg_display: i64, _arg_on: bool) -> binder::Result<()> { self.0.r#setAutoLowLatencyMode(_arg_display, _arg_on) }
  fn r#setClientTargetSlotCount(&self, _arg_display: i64, _arg_clientTargetSlotCount: i32) -> binder::Result<()> { self.0.r#setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount) }
  fn r#setColorMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode, _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent) -> binder::Result<()> { self.0.r#setColorMode(_arg_display, _arg_mode, _arg_intent) }
  fn r#setContentType(&self, _arg_display: i64, _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType) -> binder::Result<()> { self.0.r#setContentType(_arg_display, _arg_type) }
  fn r#setDisplayedContentSamplingEnabled(&self, _arg_display: i64, _arg_enable: bool, _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent, _arg_maxFrames: i64) -> binder::Result<()> { self.0.r#setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames) }
  fn r#setPowerMode(&self, _arg_display: i64, _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode) -> binder::Result<()> { self.0.r#setPowerMode(_arg_display, _arg_mode) }
  fn r#setReadbackBuffer(&self, _arg_display: i64, _arg_buffer: &crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle, _arg_releaseFence: Option<&binder::ParcelFileDescriptor>) -> binder::Result<()> { self.0.r#setReadbackBuffer(_arg_display, _arg_buffer, _arg_releaseFence) }
  fn r#setVsyncEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> { self.0.r#setVsyncEnabled(_arg_display, _arg_enabled) }
  fn r#setIdleTimerEnabled(&self, _arg_display: i64, _arg_timeoutMs: i32) -> binder::Result<()> { self.0.r#setIdleTimerEnabled(_arg_display, _arg_timeoutMs) }
  fn r#getOverlaySupport(&self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties> { self.0.r#getOverlaySupport() }
  fn r#getHdrConversionCapabilities(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability>> { self.0.r#getHdrConversionCapabilities() }
  fn r#setHdrConversionStrategy(&self, _arg_conversionStrategy: &crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy) -> binder::Result<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr> { self.0.r#setHdrConversionStrategy(_arg_conversionStrategy) }
  fn r#setRefreshRateChangedCallbackDebugEnabled(&self, _arg_display: i64, _arg_enabled: bool) -> binder::Result<()> { self.0.r#setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled) }
  fn r#setLayerName(&self, _arg_display: i64, _arg_layer: i64, _arg_name: &str) -> binder::Result<()> { self.0.r#setLayerName(_arg_display, _arg_layer, _arg_name) }
}
fn on_transact(_aidl_service: &dyn IComposerClient, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#createLayer => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_bufferSlotCount: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#createLayer(_arg_display, _arg_bufferSlotCount);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#createVirtualDisplay => {
      let _arg_width: i32 = _aidl_data.read()?;
      let _arg_height: i32 = _aidl_data.read()?;
      let _arg_formatHint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat = _aidl_data.read()?;
      let _arg_outputBufferSlotCount: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#createVirtualDisplay(_arg_width, _arg_height, _arg_formatHint, _arg_outputBufferSlotCount);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#destroyLayer => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_layer: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#destroyLayer(_arg_display, _arg_layer);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#destroyVirtualDisplay => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#destroyVirtualDisplay(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#executeCommands => {
      let _arg_commands: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#executeCommands(&_arg_commands);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getActiveConfig => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getActiveConfig(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getColorModes => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getColorModes(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getDataspaceSaturationMatrix => {
      let _arg_dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getDataspaceSaturationMatrix(_arg_dataspace);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getDisplayAttribute => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_config: i32 = _aidl_data.read()?;
      let _arg_attribute: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getDisplayAttribute(_arg_display, _arg_config, _arg_attribute);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getDisplayCapabilities => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getDisplayCapabilities(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getDisplayConfigs => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getDisplayConfigs(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getDisplayConnectionType => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getDisplayConnectionType(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getDisplayIdentificationData => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getDisplayIdentificationData(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getDisplayName => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getDisplayName(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getDisplayVsyncPeriod => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getDisplayVsyncPeriod(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getDisplayedContentSample => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_maxFrames: i64 = _aidl_data.read()?;
      let _arg_timestamp: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getDisplayedContentSample(_arg_display, _arg_maxFrames, _arg_timestamp);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getDisplayedContentSamplingAttributes => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getDisplayedContentSamplingAttributes(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getDisplayPhysicalOrientation => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getDisplayPhysicalOrientation(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getHdrCapabilities => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getHdrCapabilities(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getMaxVirtualDisplayCount => {
      let _aidl_return = _aidl_service.r#getMaxVirtualDisplayCount();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getPerFrameMetadataKeys => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getPerFrameMetadataKeys(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getReadbackBufferAttributes => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getReadbackBufferAttributes(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getReadbackBufferFence => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getReadbackBufferFence(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getRenderIntents => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getRenderIntents(_arg_display, _arg_mode);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getSupportedContentTypes => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getSupportedContentTypes(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getDisplayDecorationSupport => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getDisplayDecorationSupport(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#registerCallback => {
      let _arg_callback: binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_IComposerCallback> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#registerCallback(&_arg_callback);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setActiveConfig => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_config: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setActiveConfig(_arg_display, _arg_config);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setActiveConfigWithConstraints => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_config: i32 = _aidl_data.read()?;
      let _arg_vsyncPeriodChangeConstraints: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setActiveConfigWithConstraints(_arg_display, _arg_config, &_arg_vsyncPeriodChangeConstraints);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setBootDisplayConfig => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_config: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setBootDisplayConfig(_arg_display, _arg_config);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#clearBootDisplayConfig => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#clearBootDisplayConfig(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getPreferredBootDisplayConfig => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getPreferredBootDisplayConfig(_arg_display);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setAutoLowLatencyMode => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_on: bool = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setAutoLowLatencyMode(_arg_display, _arg_on);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setClientTargetSlotCount => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_clientTargetSlotCount: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setClientTargetSlotCount(_arg_display, _arg_clientTargetSlotCount);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setColorMode => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_ColorMode = _aidl_data.read()?;
      let _arg_intent: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setColorMode(_arg_display, _arg_mode, _arg_intent);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setContentType => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_type: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_ContentType = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setContentType(_arg_display, _arg_type);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setDisplayedContentSamplingEnabled => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_enable: bool = _aidl_data.read()?;
      let _arg_componentMask: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent = _aidl_data.read()?;
      let _arg_maxFrames: i64 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setDisplayedContentSamplingEnabled(_arg_display, _arg_enable, _arg_componentMask, _arg_maxFrames);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setPowerMode => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_mode: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_9_PowerMode = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setPowerMode(_arg_display, _arg_mode);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setReadbackBuffer => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_buffer: crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle = _aidl_data.read()?;
      let _arg_releaseFence: Option<binder::ParcelFileDescriptor> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setReadbackBuffer(_arg_display, &_arg_buffer, _arg_releaseFence.as_ref());
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setVsyncEnabled => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_enabled: bool = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setVsyncEnabled(_arg_display, _arg_enabled);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setIdleTimerEnabled => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_timeoutMs: i32 = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setIdleTimerEnabled(_arg_display, _arg_timeoutMs);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getOverlaySupport => {
      let _aidl_return = _aidl_service.r#getOverlaySupport();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getHdrConversionCapabilities => {
      let _aidl_return = _aidl_service.r#getHdrConversionCapabilities();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setHdrConversionStrategy => {
      let _arg_conversionStrategy: crate::mangled::_7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setHdrConversionStrategy(&_arg_conversionStrategy);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setRefreshRateChangedCallbackDebugEnabled => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_enabled: bool = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setRefreshRateChangedCallbackDebugEnabled(_arg_display, _arg_enabled);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#setLayerName => {
      let _arg_display: i64 = _aidl_data.read()?;
      let _arg_layer: i64 = _aidl_data.read()?;
      let _arg_name: String = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#setLayerName(_arg_display, _arg_layer, &_arg_name);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::r#IComposerClient as _7_android_8_hardware_8_graphics_9_composer3_15_IComposerClient;
}
