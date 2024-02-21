#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#ExtendableType {
  pub r#name: String,
  pub r#value: i64,
}
impl Default for r#ExtendableType {
  fn default() -> Self {
    Self {
      r#name: Default::default(),
      r#value: 0,
    }
  }
}
impl binder::Parcelable for r#ExtendableType {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#name)?;
      subparcel.write(&self.r#value)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#name = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#value = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#ExtendableType);
binder::impl_deserialize_for_parcelable!(r#ExtendableType);
impl binder::binder_impl::ParcelableMetadata for r#ExtendableType {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.common.ExtendableType" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#ExtendableType as _7_android_8_hardware_8_graphics_6_common_14_ExtendableType;
}
