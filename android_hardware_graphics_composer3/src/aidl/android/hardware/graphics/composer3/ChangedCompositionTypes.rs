#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#ChangedCompositionTypes {
  pub r#display: i64,
  pub r#layers: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ChangedCompositionLayer>,
}
impl Default for r#ChangedCompositionTypes {
  fn default() -> Self {
    Self {
      r#display: 0,
      r#layers: Default::default(),
    }
  }
}
impl binder::Parcelable for r#ChangedCompositionTypes {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#display)?;
      subparcel.write(&self.r#layers)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#display = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#layers = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#ChangedCompositionTypes);
binder::impl_deserialize_for_parcelable!(r#ChangedCompositionTypes);
impl binder::binder_impl::ParcelableMetadata for r#ChangedCompositionTypes {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ChangedCompositionTypes" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#ChangedCompositionTypes as _7_android_8_hardware_8_graphics_9_composer3_23_ChangedCompositionTypes;
}
