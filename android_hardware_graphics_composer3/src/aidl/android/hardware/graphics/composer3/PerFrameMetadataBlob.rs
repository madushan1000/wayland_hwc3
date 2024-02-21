#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#PerFrameMetadataBlob {
  pub r#key: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey,
  pub r#blob: Vec<u8>,
}
impl Default for r#PerFrameMetadataBlob {
  fn default() -> Self {
    Self {
      r#key: Default::default(),
      r#blob: Default::default(),
    }
  }
}
impl binder::Parcelable for r#PerFrameMetadataBlob {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#key)?;
      subparcel.write(&self.r#blob)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#key = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#blob = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#PerFrameMetadataBlob);
binder::impl_deserialize_for_parcelable!(r#PerFrameMetadataBlob);
impl binder::binder_impl::ParcelableMetadata for r#PerFrameMetadataBlob {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.PerFrameMetadataBlob" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#PerFrameMetadataBlob as _7_android_8_hardware_8_graphics_9_composer3_20_PerFrameMetadataBlob;
}
