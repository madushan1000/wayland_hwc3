#![forbid(unsafe_code)]
#![rustfmt::skip]
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
