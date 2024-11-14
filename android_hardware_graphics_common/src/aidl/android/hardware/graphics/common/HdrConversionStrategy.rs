/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/mnishant/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl --lang=rust aidl/android/hardware/graphics/common/AlphaInterpretation.aidl aidl/android/hardware/graphics/common/BlendMode.aidl aidl/android/hardware/graphics/common/BufferUsage.aidl aidl/android/hardware/graphics/common/ChromaSiting.aidl aidl/android/hardware/graphics/common/ColorTransform.aidl aidl/android/hardware/graphics/common/Compression.aidl aidl/android/hardware/graphics/common/Cta861_3.aidl aidl/android/hardware/graphics/common/Dataspace.aidl aidl/android/hardware/graphics/common/DisplayDecorationSupport.aidl aidl/android/hardware/graphics/common/ExtendableType.aidl aidl/android/hardware/graphics/common/FRect.aidl aidl/android/hardware/graphics/common/HardwareBuffer.aidl aidl/android/hardware/graphics/common/HardwareBufferDescription.aidl aidl/android/hardware/graphics/common/Hdr.aidl aidl/android/hardware/graphics/common/HdrConversionCapability.aidl aidl/android/hardware/graphics/common/HdrConversionStrategy.aidl aidl/android/hardware/graphics/common/Interlaced.aidl aidl/android/hardware/graphics/common/PixelFormat.aidl aidl/android/hardware/graphics/common/PlaneLayout.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponent.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponentType.aidl aidl/android/hardware/graphics/common/Point.aidl aidl/android/hardware/graphics/common/Rect.aidl aidl/android/hardware/graphics/common/Smpte2086.aidl aidl/android/hardware/graphics/common/StandardMetadataType.aidl aidl/android/hardware/graphics/common/Transform.aidl aidl/android/hardware/graphics/common/XyColor.aidl -I aidl/ --stability=vintf --structured -o android_hardware_graphics_common/src/aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub enum r#HdrConversionStrategy {
  Passthrough(bool),
  AutoAllowedHdrTypes(Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr>),
  ForceHdrConversion(crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr),
}
impl Default for r#HdrConversionStrategy {
  fn default() -> Self {
    Self::Passthrough(true)
  }
}
impl binder::Parcelable for r#HdrConversionStrategy {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    match self {
      Self::Passthrough(v) => {
        parcel.write(&0i32)?;
        parcel.write(v)
      }
      Self::AutoAllowedHdrTypes(v) => {
        parcel.write(&1i32)?;
        parcel.write(v)
      }
      Self::ForceHdrConversion(v) => {
        parcel.write(&2i32)?;
        parcel.write(v)
      }
    }
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    let tag: i32 = parcel.read()?;
    match tag {
      0 => {
        let value: bool = parcel.read()?;
        *self = Self::Passthrough(value);
        Ok(())
      }
      1 => {
        let value: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr> = parcel.read()?;
        *self = Self::AutoAllowedHdrTypes(value);
        Ok(())
      }
      2 => {
        let value: crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr = parcel.read()?;
        *self = Self::ForceHdrConversion(value);
        Ok(())
      }
      _ => {
        Err(binder::StatusCode::BAD_VALUE)
      }
    }
  }
}
binder::impl_serialize_for_parcelable!(r#HdrConversionStrategy);
binder::impl_deserialize_for_parcelable!(r#HdrConversionStrategy);
impl binder::binder_impl::ParcelableMetadata for r#HdrConversionStrategy {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.common.HdrConversionStrategy" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub mod r#Tag {
  #![allow(non_upper_case_globals)]
  use binder::declare_binder_enum;
  declare_binder_enum! {
    r#Tag : [i32; 3] {
      r#passthrough = 0,
      r#autoAllowedHdrTypes = 1,
      r#forceHdrConversion = 2,
    }
  }
}
pub(crate) mod mangled {
 pub use super::r#HdrConversionStrategy as _7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy;
 pub use super::r#Tag::r#Tag as _7_android_8_hardware_8_graphics_6_common_21_HdrConversionStrategy_3_Tag;
}
