/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/mnishant/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl --lang=rust aidl/android/hardware/graphics/common/AlphaInterpretation.aidl aidl/android/hardware/graphics/common/BlendMode.aidl aidl/android/hardware/graphics/common/BufferUsage.aidl aidl/android/hardware/graphics/common/ChromaSiting.aidl aidl/android/hardware/graphics/common/ColorTransform.aidl aidl/android/hardware/graphics/common/Compression.aidl aidl/android/hardware/graphics/common/Cta861_3.aidl aidl/android/hardware/graphics/common/Dataspace.aidl aidl/android/hardware/graphics/common/DisplayDecorationSupport.aidl aidl/android/hardware/graphics/common/ExtendableType.aidl aidl/android/hardware/graphics/common/FRect.aidl aidl/android/hardware/graphics/common/HardwareBuffer.aidl aidl/android/hardware/graphics/common/HardwareBufferDescription.aidl aidl/android/hardware/graphics/common/Hdr.aidl aidl/android/hardware/graphics/common/HdrConversionCapability.aidl aidl/android/hardware/graphics/common/HdrConversionStrategy.aidl aidl/android/hardware/graphics/common/Interlaced.aidl aidl/android/hardware/graphics/common/PixelFormat.aidl aidl/android/hardware/graphics/common/PlaneLayout.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponent.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponentType.aidl aidl/android/hardware/graphics/common/Point.aidl aidl/android/hardware/graphics/common/Rect.aidl aidl/android/hardware/graphics/common/Smpte2086.aidl aidl/android/hardware/graphics/common/StandardMetadataType.aidl aidl/android/hardware/graphics/common/Transform.aidl aidl/android/hardware/graphics/common/XyColor.aidl -I aidl/ --stability=vintf --structured -o android_hardware_graphics_common/src/aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct r#Smpte2086 {
  pub r#primaryRed: crate::mangled::_7_android_8_hardware_8_graphics_6_common_7_XyColor,
  pub r#primaryGreen: crate::mangled::_7_android_8_hardware_8_graphics_6_common_7_XyColor,
  pub r#primaryBlue: crate::mangled::_7_android_8_hardware_8_graphics_6_common_7_XyColor,
  pub r#whitePoint: crate::mangled::_7_android_8_hardware_8_graphics_6_common_7_XyColor,
  pub r#maxLuminance: f32,
  pub r#minLuminance: f32,
}
impl Default for r#Smpte2086 {
  fn default() -> Self {
    Self {
      r#primaryRed: Default::default(),
      r#primaryGreen: Default::default(),
      r#primaryBlue: Default::default(),
      r#whitePoint: Default::default(),
      r#maxLuminance: 0.000000f32,
      r#minLuminance: 0.000000f32,
    }
  }
}
impl binder::Parcelable for r#Smpte2086 {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#primaryRed)?;
      subparcel.write(&self.r#primaryGreen)?;
      subparcel.write(&self.r#primaryBlue)?;
      subparcel.write(&self.r#whitePoint)?;
      subparcel.write(&self.r#maxLuminance)?;
      subparcel.write(&self.r#minLuminance)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#primaryRed = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#primaryGreen = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#primaryBlue = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#whitePoint = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#maxLuminance = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#minLuminance = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#Smpte2086);
binder::impl_deserialize_for_parcelable!(r#Smpte2086);
impl binder::binder_impl::ParcelableMetadata for r#Smpte2086 {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.common.Smpte2086" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#Smpte2086 as _7_android_8_hardware_8_graphics_6_common_9_Smpte2086;
}
