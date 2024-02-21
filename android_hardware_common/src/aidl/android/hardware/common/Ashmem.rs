#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#Ashmem {
  pub r#fd: Option<binder::ParcelFileDescriptor>,
  pub r#size: i64,
}
impl Default for r#Ashmem {
  fn default() -> Self {
    Self {
      r#fd: Default::default(),
      r#size: 0,
    }
  }
}
impl binder::Parcelable for r#Ashmem {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      let __field_ref = self.r#fd.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
      subparcel.write(__field_ref)?;
      subparcel.write(&self.r#size)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#fd = Some(subparcel.read()?);
      }
      if subparcel.has_more_data() {
        self.r#size = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#Ashmem);
binder::impl_deserialize_for_parcelable!(r#Ashmem);
impl binder::binder_impl::ParcelableMetadata for r#Ashmem {
  fn get_descriptor() -> &'static str { "android.hardware.common.Ashmem" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#Ashmem as _7_android_8_hardware_6_common_6_Ashmem;
}
