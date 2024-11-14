/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/mnishant/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl --lang=rust aidl/android/hardware/graphics/composer3/Buffer.aidl aidl/android/hardware/graphics/composer3/Capability.aidl aidl/android/hardware/graphics/composer3/ChangedCompositionLayer.aidl aidl/android/hardware/graphics/composer3/ChangedCompositionTypes.aidl aidl/android/hardware/graphics/composer3/ClientTarget.aidl aidl/android/hardware/graphics/composer3/ClientTargetProperty.aidl aidl/android/hardware/graphics/composer3/ClientTargetPropertyWithBrightness.aidl aidl/android/hardware/graphics/composer3/ClockMonotonicTimestamp.aidl aidl/android/hardware/graphics/composer3/Color.aidl aidl/android/hardware/graphics/composer3/ColorMode.aidl aidl/android/hardware/graphics/composer3/CommandError.aidl aidl/android/hardware/graphics/composer3/CommandResultPayload.aidl aidl/android/hardware/graphics/composer3/Composition.aidl aidl/android/hardware/graphics/composer3/ContentType.aidl aidl/android/hardware/graphics/composer3/DimmingStage.aidl aidl/android/hardware/graphics/composer3/DisplayAttribute.aidl aidl/android/hardware/graphics/composer3/DisplayBrightness.aidl aidl/android/hardware/graphics/composer3/DisplayCapability.aidl aidl/android/hardware/graphics/composer3/DisplayCommand.aidl aidl/android/hardware/graphics/composer3/DisplayConnectionType.aidl aidl/android/hardware/graphics/composer3/DisplayContentSample.aidl aidl/android/hardware/graphics/composer3/DisplayContentSamplingAttributes.aidl aidl/android/hardware/graphics/composer3/DisplayIdentification.aidl aidl/android/hardware/graphics/composer3/DisplayRequest.aidl aidl/android/hardware/graphics/composer3/FormatColorComponent.aidl aidl/android/hardware/graphics/composer3/HdrCapabilities.aidl aidl/android/hardware/graphics/composer3/IComposer.aidl aidl/android/hardware/graphics/composer3/IComposerCallback.aidl aidl/android/hardware/graphics/composer3/IComposerClient.aidl aidl/android/hardware/graphics/composer3/LayerBrightness.aidl aidl/android/hardware/graphics/composer3/LayerCommand.aidl aidl/android/hardware/graphics/composer3/OverlayProperties.aidl aidl/android/hardware/graphics/composer3/ParcelableBlendMode.aidl aidl/android/hardware/graphics/composer3/ParcelableComposition.aidl aidl/android/hardware/graphics/composer3/ParcelableDataspace.aidl aidl/android/hardware/graphics/composer3/ParcelableTransform.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadata.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadataBlob.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadataKey.aidl aidl/android/hardware/graphics/composer3/PlaneAlpha.aidl aidl/android/hardware/graphics/composer3/PowerMode.aidl aidl/android/hardware/graphics/composer3/PresentFence.aidl aidl/android/hardware/graphics/composer3/PresentOrValidate.aidl aidl/android/hardware/graphics/composer3/ReadbackBufferAttributes.aidl aidl/android/hardware/graphics/composer3/RefreshRateChangedDebugData.aidl aidl/android/hardware/graphics/composer3/ReleaseFences.aidl aidl/android/hardware/graphics/composer3/RenderIntent.aidl aidl/android/hardware/graphics/composer3/VirtualDisplay.aidl aidl/android/hardware/graphics/composer3/VsyncPeriodChangeConstraints.aidl aidl/android/hardware/graphics/composer3/VsyncPeriodChangeTimeline.aidl aidl/android/hardware/graphics/composer3/ZOrder.aidl -I aidl/ --stability=vintf --structured -o android_hardware_graphics_composer3/src/aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct r#DisplayCommand {
  pub r#display: i64,
  pub r#layers: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_LayerCommand>,
  pub r#colorTransformMatrix: Option<Vec<f32>>,
  pub r#brightness: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayBrightness>,
  pub r#clientTarget: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_ClientTarget>,
  pub r#virtualDisplayOutputBuffer: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_6_Buffer>,
  pub r#expectedPresentTime: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp>,
  pub r#validateDisplay: bool,
  pub r#acceptDisplayChanges: bool,
  pub r#presentDisplay: bool,
  pub r#presentOrValidateDisplay: bool,
}
impl Default for r#DisplayCommand {
  fn default() -> Self {
    Self {
      r#display: 0,
      r#layers: Default::default(),
      r#colorTransformMatrix: Default::default(),
      r#brightness: Default::default(),
      r#clientTarget: Default::default(),
      r#virtualDisplayOutputBuffer: Default::default(),
      r#expectedPresentTime: Default::default(),
      r#validateDisplay: false,
      r#acceptDisplayChanges: false,
      r#presentDisplay: false,
      r#presentOrValidateDisplay: false,
    }
  }
}
impl binder::Parcelable for r#DisplayCommand {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#display)?;
      subparcel.write(&self.r#layers)?;
      subparcel.write(&self.r#colorTransformMatrix)?;
      subparcel.write(&self.r#brightness)?;
      subparcel.write(&self.r#clientTarget)?;
      subparcel.write(&self.r#virtualDisplayOutputBuffer)?;
      subparcel.write(&self.r#expectedPresentTime)?;
      subparcel.write(&self.r#validateDisplay)?;
      subparcel.write(&self.r#acceptDisplayChanges)?;
      subparcel.write(&self.r#presentDisplay)?;
      subparcel.write(&self.r#presentOrValidateDisplay)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#display = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#layers = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#colorTransformMatrix = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#brightness = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#clientTarget = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#virtualDisplayOutputBuffer = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#expectedPresentTime = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#validateDisplay = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#acceptDisplayChanges = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#presentDisplay = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#presentOrValidateDisplay = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#DisplayCommand);
binder::impl_deserialize_for_parcelable!(r#DisplayCommand);
impl binder::binder_impl::ParcelableMetadata for r#DisplayCommand {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.DisplayCommand" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#DisplayCommand as _7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand;
}
