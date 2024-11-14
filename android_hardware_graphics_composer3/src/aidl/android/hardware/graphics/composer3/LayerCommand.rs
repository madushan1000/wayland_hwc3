/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/mnishant/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl --lang=rust aidl/android/hardware/graphics/composer3/Buffer.aidl aidl/android/hardware/graphics/composer3/Capability.aidl aidl/android/hardware/graphics/composer3/ChangedCompositionLayer.aidl aidl/android/hardware/graphics/composer3/ChangedCompositionTypes.aidl aidl/android/hardware/graphics/composer3/ClientTarget.aidl aidl/android/hardware/graphics/composer3/ClientTargetProperty.aidl aidl/android/hardware/graphics/composer3/ClientTargetPropertyWithBrightness.aidl aidl/android/hardware/graphics/composer3/ClockMonotonicTimestamp.aidl aidl/android/hardware/graphics/composer3/Color.aidl aidl/android/hardware/graphics/composer3/ColorMode.aidl aidl/android/hardware/graphics/composer3/CommandError.aidl aidl/android/hardware/graphics/composer3/CommandResultPayload.aidl aidl/android/hardware/graphics/composer3/Composition.aidl aidl/android/hardware/graphics/composer3/ContentType.aidl aidl/android/hardware/graphics/composer3/DimmingStage.aidl aidl/android/hardware/graphics/composer3/DisplayAttribute.aidl aidl/android/hardware/graphics/composer3/DisplayBrightness.aidl aidl/android/hardware/graphics/composer3/DisplayCapability.aidl aidl/android/hardware/graphics/composer3/DisplayCommand.aidl aidl/android/hardware/graphics/composer3/DisplayConnectionType.aidl aidl/android/hardware/graphics/composer3/DisplayContentSample.aidl aidl/android/hardware/graphics/composer3/DisplayContentSamplingAttributes.aidl aidl/android/hardware/graphics/composer3/DisplayIdentification.aidl aidl/android/hardware/graphics/composer3/DisplayRequest.aidl aidl/android/hardware/graphics/composer3/FormatColorComponent.aidl aidl/android/hardware/graphics/composer3/HdrCapabilities.aidl aidl/android/hardware/graphics/composer3/IComposer.aidl aidl/android/hardware/graphics/composer3/IComposerCallback.aidl aidl/android/hardware/graphics/composer3/IComposerClient.aidl aidl/android/hardware/graphics/composer3/LayerBrightness.aidl aidl/android/hardware/graphics/composer3/LayerCommand.aidl aidl/android/hardware/graphics/composer3/OverlayProperties.aidl aidl/android/hardware/graphics/composer3/ParcelableBlendMode.aidl aidl/android/hardware/graphics/composer3/ParcelableComposition.aidl aidl/android/hardware/graphics/composer3/ParcelableDataspace.aidl aidl/android/hardware/graphics/composer3/ParcelableTransform.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadata.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadataBlob.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadataKey.aidl aidl/android/hardware/graphics/composer3/PlaneAlpha.aidl aidl/android/hardware/graphics/composer3/PowerMode.aidl aidl/android/hardware/graphics/composer3/PresentFence.aidl aidl/android/hardware/graphics/composer3/PresentOrValidate.aidl aidl/android/hardware/graphics/composer3/ReadbackBufferAttributes.aidl aidl/android/hardware/graphics/composer3/RefreshRateChangedDebugData.aidl aidl/android/hardware/graphics/composer3/ReleaseFences.aidl aidl/android/hardware/graphics/composer3/RenderIntent.aidl aidl/android/hardware/graphics/composer3/VirtualDisplay.aidl aidl/android/hardware/graphics/composer3/VsyncPeriodChangeConstraints.aidl aidl/android/hardware/graphics/composer3/VsyncPeriodChangeTimeline.aidl aidl/android/hardware/graphics/composer3/ZOrder.aidl -I aidl/ --stability=vintf --structured -o android_hardware_graphics_composer3/src/aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct r#LayerCommand {
  pub r#layer: i64,
  pub r#cursorPosition: Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_5_Point>,
  pub r#buffer: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_6_Buffer>,
  pub r#damage: Option<Vec<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_4_Rect>>>,
  pub r#blendMode: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_ParcelableBlendMode>,
  pub r#color: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_5_Color>,
  pub r#composition: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_21_ParcelableComposition>,
  pub r#dataspace: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_ParcelableDataspace>,
  pub r#displayFrame: Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_4_Rect>,
  pub r#planeAlpha: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_10_PlaneAlpha>,
  pub r#sidebandStream: Option<crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle>,
  pub r#sourceCrop: Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_5_FRect>,
  pub r#transform: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_ParcelableTransform>,
  pub r#visibleRegion: Option<Vec<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_4_Rect>>>,
  pub r#z: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_6_ZOrder>,
  pub r#colorTransform: Option<Vec<f32>>,
  pub r#brightness: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_15_LayerBrightness>,
  pub r#perFrameMetadata: Option<Vec<Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_16_PerFrameMetadata>>>,
  pub r#perFrameMetadataBlob: Option<Vec<Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_PerFrameMetadataBlob>>>,
  pub r#blockingRegion: Option<Vec<Option<crate::mangled::_7_android_8_hardware_8_graphics_6_common_4_Rect>>>,
  pub r#bufferSlotsToClear: Option<Vec<i32>>,
}
impl Default for r#LayerCommand {
  fn default() -> Self {
    Self {
      r#layer: 0,
      r#cursorPosition: Default::default(),
      r#buffer: Default::default(),
      r#damage: Default::default(),
      r#blendMode: Default::default(),
      r#color: Default::default(),
      r#composition: Default::default(),
      r#dataspace: Default::default(),
      r#displayFrame: Default::default(),
      r#planeAlpha: Default::default(),
      r#sidebandStream: Default::default(),
      r#sourceCrop: Default::default(),
      r#transform: Default::default(),
      r#visibleRegion: Default::default(),
      r#z: Default::default(),
      r#colorTransform: Default::default(),
      r#brightness: Default::default(),
      r#perFrameMetadata: Default::default(),
      r#perFrameMetadataBlob: Default::default(),
      r#blockingRegion: Default::default(),
      r#bufferSlotsToClear: Default::default(),
    }
  }
}
impl binder::Parcelable for r#LayerCommand {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#layer)?;
      subparcel.write(&self.r#cursorPosition)?;
      subparcel.write(&self.r#buffer)?;
      subparcel.write(&self.r#damage)?;
      subparcel.write(&self.r#blendMode)?;
      subparcel.write(&self.r#color)?;
      subparcel.write(&self.r#composition)?;
      subparcel.write(&self.r#dataspace)?;
      subparcel.write(&self.r#displayFrame)?;
      subparcel.write(&self.r#planeAlpha)?;
      subparcel.write(&self.r#sidebandStream)?;
      subparcel.write(&self.r#sourceCrop)?;
      subparcel.write(&self.r#transform)?;
      subparcel.write(&self.r#visibleRegion)?;
      subparcel.write(&self.r#z)?;
      subparcel.write(&self.r#colorTransform)?;
      subparcel.write(&self.r#brightness)?;
      subparcel.write(&self.r#perFrameMetadata)?;
      subparcel.write(&self.r#perFrameMetadataBlob)?;
      subparcel.write(&self.r#blockingRegion)?;
      subparcel.write(&self.r#bufferSlotsToClear)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#layer = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#cursorPosition = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#buffer = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#damage = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#blendMode = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#color = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#composition = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#dataspace = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#displayFrame = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#planeAlpha = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#sidebandStream = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#sourceCrop = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#transform = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#visibleRegion = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#z = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#colorTransform = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#brightness = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#perFrameMetadata = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#perFrameMetadataBlob = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#blockingRegion = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#bufferSlotsToClear = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#LayerCommand);
binder::impl_deserialize_for_parcelable!(r#LayerCommand);
impl binder::binder_impl::ParcelableMetadata for r#LayerCommand {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.LayerCommand" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#LayerCommand as _7_android_8_hardware_8_graphics_9_composer3_12_LayerCommand;
}
