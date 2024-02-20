#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#MappableFile {
  pub r#length: i64,
  pub r#prot: i32,
  pub r#fd: Option<binder::ParcelFileDescriptor>,
  pub r#offset: i64,
}
impl Default for r#MappableFile {
  fn default() -> Self {
    Self {
      r#length: 0,
      r#prot: 0,
      r#fd: Default::default(),
      r#offset: 0,
    }
  }
}
impl binder::Parcelable for r#MappableFile {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#length)?;
      subparcel.write(&self.r#prot)?;
      let __field_ref = self.r#fd.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
      subparcel.write(__field_ref)?;
      subparcel.write(&self.r#offset)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#length = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#prot = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#fd = Some(subparcel.read()?);
      }
      if subparcel.has_more_data() {
        self.r#offset = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#MappableFile);
binder::impl_deserialize_for_parcelable!(r#MappableFile);
impl binder::binder_impl::ParcelableMetadata for r#MappableFile {
  fn get_descriptor() -> &'static str { "android.hardware.common.MappableFile" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#MappableFile as _7_android_8_hardware_6_common_12_MappableFile;
}
