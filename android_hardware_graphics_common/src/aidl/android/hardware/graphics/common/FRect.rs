/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/mnishant/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl --lang=rust aidl/android/hardware/graphics/common/AlphaInterpretation.aidl aidl/android/hardware/graphics/common/BlendMode.aidl aidl/android/hardware/graphics/common/BufferUsage.aidl aidl/android/hardware/graphics/common/ChromaSiting.aidl aidl/android/hardware/graphics/common/ColorTransform.aidl aidl/android/hardware/graphics/common/Compression.aidl aidl/android/hardware/graphics/common/Cta861_3.aidl aidl/android/hardware/graphics/common/Dataspace.aidl aidl/android/hardware/graphics/common/DisplayDecorationSupport.aidl aidl/android/hardware/graphics/common/ExtendableType.aidl aidl/android/hardware/graphics/common/FRect.aidl aidl/android/hardware/graphics/common/HardwareBuffer.aidl aidl/android/hardware/graphics/common/HardwareBufferDescription.aidl aidl/android/hardware/graphics/common/Hdr.aidl aidl/android/hardware/graphics/common/HdrConversionCapability.aidl aidl/android/hardware/graphics/common/HdrConversionStrategy.aidl aidl/android/hardware/graphics/common/Interlaced.aidl aidl/android/hardware/graphics/common/PixelFormat.aidl aidl/android/hardware/graphics/common/PlaneLayout.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponent.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponentType.aidl aidl/android/hardware/graphics/common/Point.aidl aidl/android/hardware/graphics/common/Rect.aidl aidl/android/hardware/graphics/common/Smpte2086.aidl aidl/android/hardware/graphics/common/StandardMetadataType.aidl aidl/android/hardware/graphics/common/Transform.aidl aidl/android/hardware/graphics/common/XyColor.aidl -I aidl/ --stability=vintf --structured -o android_hardware_graphics_common/src/aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct r#FRect {
  pub r#left: f32,
  pub r#top: f32,
  pub r#right: f32,
  pub r#bottom: f32,
}
impl Default for r#FRect {
  fn default() -> Self {
    Self {
      r#left: 0.000000f32,
      r#top: 0.000000f32,
      r#right: 0.000000f32,
      r#bottom: 0.000000f32,
    }
  }
}
impl binder::Parcelable for r#FRect {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#left)?;
      subparcel.write(&self.r#top)?;
      subparcel.write(&self.r#right)?;
      subparcel.write(&self.r#bottom)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#left = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#top = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#right = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#bottom = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#FRect);
binder::impl_deserialize_for_parcelable!(r#FRect);
impl binder::binder_impl::ParcelableMetadata for r#FRect {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.common.FRect" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#FRect as _7_android_8_hardware_8_graphics_6_common_5_FRect;
}
