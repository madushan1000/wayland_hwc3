#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#PerFrameMetadata {
  pub r#key: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey,
  pub r#value: f32,
}
impl Default for r#PerFrameMetadata {
  fn default() -> Self {
    Self {
      r#key: Default::default(),
      r#value: 0.000000f32,
    }
  }
}
impl binder::Parcelable for r#PerFrameMetadata {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#key)?;
      subparcel.write(&self.r#value)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#key = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#value = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#PerFrameMetadata);
binder::impl_deserialize_for_parcelable!(r#PerFrameMetadata);
impl binder::binder_impl::ParcelableMetadata for r#PerFrameMetadata {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.PerFrameMetadata" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#PerFrameMetadata as _7_android_8_hardware_8_graphics_9_composer3_16_PerFrameMetadata;
}
