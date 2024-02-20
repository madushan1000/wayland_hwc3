#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#CommandError {
  pub r#commandIndex: i32,
  pub r#errorCode: i32,
}
impl Default for r#CommandError {
  fn default() -> Self {
    Self {
      r#commandIndex: 0,
      r#errorCode: 0,
    }
  }
}
impl binder::Parcelable for r#CommandError {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#commandIndex)?;
      subparcel.write(&self.r#errorCode)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#commandIndex = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#errorCode = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#CommandError);
binder::impl_deserialize_for_parcelable!(r#CommandError);
impl binder::binder_impl::ParcelableMetadata for r#CommandError {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.CommandError" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#CommandError as _7_android_8_hardware_8_graphics_9_composer3_12_CommandError;
}
