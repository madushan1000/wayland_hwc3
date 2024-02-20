#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#PresentOrValidate {
  pub r#display: i64,
  pub r#result: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_PresentOrValidate_6_Result,
}
impl Default for r#PresentOrValidate {
  fn default() -> Self {
    Self {
      r#display: 0,
      r#result: Default::default(),
    }
  }
}
impl binder::Parcelable for r#PresentOrValidate {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#display)?;
      subparcel.write(&self.r#result)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#display = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#result = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#PresentOrValidate);
binder::impl_deserialize_for_parcelable!(r#PresentOrValidate);
impl binder::binder_impl::ParcelableMetadata for r#PresentOrValidate {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.PresentOrValidate" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub mod r#Result {
  #![allow(non_upper_case_globals)]
  use binder::declare_binder_enum;
  declare_binder_enum! {
    r#Result : [i8; 2] {
      r#Validated = 0,
      r#Presented = 1,
    }
  }
}
pub(crate) mod mangled {
 pub use super::r#PresentOrValidate as _7_android_8_hardware_8_graphics_9_composer3_17_PresentOrValidate;
 pub use super::r#Result::r#Result as _7_android_8_hardware_8_graphics_9_composer3_17_PresentOrValidate_6_Result;
}
