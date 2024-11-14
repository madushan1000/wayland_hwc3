/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/mnishant/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl --lang=rust aidl/android/hardware/graphics/common/AlphaInterpretation.aidl aidl/android/hardware/graphics/common/BlendMode.aidl aidl/android/hardware/graphics/common/BufferUsage.aidl aidl/android/hardware/graphics/common/ChromaSiting.aidl aidl/android/hardware/graphics/common/ColorTransform.aidl aidl/android/hardware/graphics/common/Compression.aidl aidl/android/hardware/graphics/common/Cta861_3.aidl aidl/android/hardware/graphics/common/Dataspace.aidl aidl/android/hardware/graphics/common/DisplayDecorationSupport.aidl aidl/android/hardware/graphics/common/ExtendableType.aidl aidl/android/hardware/graphics/common/FRect.aidl aidl/android/hardware/graphics/common/HardwareBuffer.aidl aidl/android/hardware/graphics/common/HardwareBufferDescription.aidl aidl/android/hardware/graphics/common/Hdr.aidl aidl/android/hardware/graphics/common/HdrConversionCapability.aidl aidl/android/hardware/graphics/common/HdrConversionStrategy.aidl aidl/android/hardware/graphics/common/Interlaced.aidl aidl/android/hardware/graphics/common/PixelFormat.aidl aidl/android/hardware/graphics/common/PlaneLayout.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponent.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponentType.aidl aidl/android/hardware/graphics/common/Point.aidl aidl/android/hardware/graphics/common/Rect.aidl aidl/android/hardware/graphics/common/Smpte2086.aidl aidl/android/hardware/graphics/common/StandardMetadataType.aidl aidl/android/hardware/graphics/common/Transform.aidl aidl/android/hardware/graphics/common/XyColor.aidl -I aidl/ --stability=vintf --structured -o android_hardware_graphics_common/src/aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct r#Cta861_3 {
  pub r#maxContentLightLevel: f32,
  pub r#maxFrameAverageLightLevel: f32,
}
impl Default for r#Cta861_3 {
  fn default() -> Self {
    Self {
      r#maxContentLightLevel: 0.000000f32,
      r#maxFrameAverageLightLevel: 0.000000f32,
    }
  }
}
impl binder::Parcelable for r#Cta861_3 {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#maxContentLightLevel)?;
      subparcel.write(&self.r#maxFrameAverageLightLevel)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#maxContentLightLevel = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#maxFrameAverageLightLevel = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#Cta861_3);
binder::impl_deserialize_for_parcelable!(r#Cta861_3);
impl binder::binder_impl::ParcelableMetadata for r#Cta861_3 {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.common.Cta861_3" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#Cta861_3 as _7_android_8_hardware_8_graphics_6_common_8_Cta861_3;
}
