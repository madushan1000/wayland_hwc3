#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#PresentFence {
  pub r#display: i64,
  pub r#fence: Option<binder::ParcelFileDescriptor>,
}
impl Default for r#PresentFence {
  fn default() -> Self {
    Self {
      r#display: 0,
      r#fence: Default::default(),
    }
  }
}
impl binder::Parcelable for r#PresentFence {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#display)?;
      let __field_ref = self.r#fence.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
      subparcel.write(__field_ref)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#display = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#fence = Some(subparcel.read()?);
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#PresentFence);
binder::impl_deserialize_for_parcelable!(r#PresentFence);
impl binder::binder_impl::ParcelableMetadata for r#PresentFence {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.PresentFence" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#PresentFence as _7_android_8_hardware_8_graphics_9_composer3_12_PresentFence;
}
