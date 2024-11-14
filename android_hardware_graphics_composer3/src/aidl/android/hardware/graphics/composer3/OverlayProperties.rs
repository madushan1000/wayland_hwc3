/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/mnishant/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl --lang=rust aidl/android/hardware/graphics/composer3/Buffer.aidl aidl/android/hardware/graphics/composer3/Capability.aidl aidl/android/hardware/graphics/composer3/ChangedCompositionLayer.aidl aidl/android/hardware/graphics/composer3/ChangedCompositionTypes.aidl aidl/android/hardware/graphics/composer3/ClientTarget.aidl aidl/android/hardware/graphics/composer3/ClientTargetProperty.aidl aidl/android/hardware/graphics/composer3/ClientTargetPropertyWithBrightness.aidl aidl/android/hardware/graphics/composer3/ClockMonotonicTimestamp.aidl aidl/android/hardware/graphics/composer3/Color.aidl aidl/android/hardware/graphics/composer3/ColorMode.aidl aidl/android/hardware/graphics/composer3/CommandError.aidl aidl/android/hardware/graphics/composer3/CommandResultPayload.aidl aidl/android/hardware/graphics/composer3/Composition.aidl aidl/android/hardware/graphics/composer3/ContentType.aidl aidl/android/hardware/graphics/composer3/DimmingStage.aidl aidl/android/hardware/graphics/composer3/DisplayAttribute.aidl aidl/android/hardware/graphics/composer3/DisplayBrightness.aidl aidl/android/hardware/graphics/composer3/DisplayCapability.aidl aidl/android/hardware/graphics/composer3/DisplayCommand.aidl aidl/android/hardware/graphics/composer3/DisplayConnectionType.aidl aidl/android/hardware/graphics/composer3/DisplayContentSample.aidl aidl/android/hardware/graphics/composer3/DisplayContentSamplingAttributes.aidl aidl/android/hardware/graphics/composer3/DisplayIdentification.aidl aidl/android/hardware/graphics/composer3/DisplayRequest.aidl aidl/android/hardware/graphics/composer3/FormatColorComponent.aidl aidl/android/hardware/graphics/composer3/HdrCapabilities.aidl aidl/android/hardware/graphics/composer3/IComposer.aidl aidl/android/hardware/graphics/composer3/IComposerCallback.aidl aidl/android/hardware/graphics/composer3/IComposerClient.aidl aidl/android/hardware/graphics/composer3/LayerBrightness.aidl aidl/android/hardware/graphics/composer3/LayerCommand.aidl aidl/android/hardware/graphics/composer3/OverlayProperties.aidl aidl/android/hardware/graphics/composer3/ParcelableBlendMode.aidl aidl/android/hardware/graphics/composer3/ParcelableComposition.aidl aidl/android/hardware/graphics/composer3/ParcelableDataspace.aidl aidl/android/hardware/graphics/composer3/ParcelableTransform.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadata.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadataBlob.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadataKey.aidl aidl/android/hardware/graphics/composer3/PlaneAlpha.aidl aidl/android/hardware/graphics/composer3/PowerMode.aidl aidl/android/hardware/graphics/composer3/PresentFence.aidl aidl/android/hardware/graphics/composer3/PresentOrValidate.aidl aidl/android/hardware/graphics/composer3/ReadbackBufferAttributes.aidl aidl/android/hardware/graphics/composer3/RefreshRateChangedDebugData.aidl aidl/android/hardware/graphics/composer3/ReleaseFences.aidl aidl/android/hardware/graphics/composer3/RenderIntent.aidl aidl/android/hardware/graphics/composer3/VirtualDisplay.aidl aidl/android/hardware/graphics/composer3/VsyncPeriodChangeConstraints.aidl aidl/android/hardware/graphics/composer3/VsyncPeriodChangeTimeline.aidl aidl/android/hardware/graphics/composer3/ZOrder.aidl -I aidl/ --stability=vintf --structured -o android_hardware_graphics_composer3/src/aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct r#OverlayProperties {
  pub r#combinations: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties_27_SupportedBufferCombinations>,
  pub r#supportMixedColorSpaces: bool,
}
impl Default for r#OverlayProperties {
  fn default() -> Self {
    Self {
      r#combinations: Default::default(),
      r#supportMixedColorSpaces: false,
    }
  }
}
impl binder::Parcelable for r#OverlayProperties {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#combinations)?;
      subparcel.write(&self.r#supportMixedColorSpaces)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#combinations = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#supportMixedColorSpaces = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#OverlayProperties);
binder::impl_deserialize_for_parcelable!(r#OverlayProperties);
impl binder::binder_impl::ParcelableMetadata for r#OverlayProperties {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.OverlayProperties" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub mod r#SupportedBufferCombinations {
  #[derive(Debug)]
  pub struct r#SupportedBufferCombinations {
    pub r#pixelFormats: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat>,
    pub r#standards: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace>,
    pub r#transfers: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace>,
    pub r#ranges: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace>,
  }
  impl Default for r#SupportedBufferCombinations {
    fn default() -> Self {
      Self {
        r#pixelFormats: Default::default(),
        r#standards: Default::default(),
        r#transfers: Default::default(),
        r#ranges: Default::default(),
      }
    }
  }
  impl binder::Parcelable for r#SupportedBufferCombinations {
    fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
      parcel.sized_write(|subparcel| {
        subparcel.write(&self.r#pixelFormats)?;
        subparcel.write(&self.r#standards)?;
        subparcel.write(&self.r#transfers)?;
        subparcel.write(&self.r#ranges)?;
        Ok(())
      })
    }
    fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
      parcel.sized_read(|subparcel| {
        if subparcel.has_more_data() {
          self.r#pixelFormats = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.r#standards = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.r#transfers = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.r#ranges = subparcel.read()?;
        }
        Ok(())
      })
    }
  }
  binder::impl_serialize_for_parcelable!(r#SupportedBufferCombinations);
  binder::impl_deserialize_for_parcelable!(r#SupportedBufferCombinations);
  impl binder::binder_impl::ParcelableMetadata for r#SupportedBufferCombinations {
    fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.OverlayProperties.SupportedBufferCombinations" }
    fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
  }
}
pub(crate) mod mangled {
 pub use super::r#OverlayProperties as _7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties;
 pub use super::r#SupportedBufferCombinations::r#SupportedBufferCombinations as _7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties_27_SupportedBufferCombinations;
}
