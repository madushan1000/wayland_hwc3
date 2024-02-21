#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#ChangedCompositionLayer {
  pub r#layer: i64,
  pub r#composition: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_11_Composition,
}
impl Default for r#ChangedCompositionLayer {
  fn default() -> Self {
    Self {
      r#layer: 0,
      r#composition: Default::default(),
    }
  }
}
impl binder::Parcelable for r#ChangedCompositionLayer {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#layer)?;
      subparcel.write(&self.r#composition)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#layer = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#composition = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#ChangedCompositionLayer);
binder::impl_deserialize_for_parcelable!(r#ChangedCompositionLayer);
impl binder::binder_impl::ParcelableMetadata for r#ChangedCompositionLayer {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ChangedCompositionLayer" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#ChangedCompositionLayer as _7_android_8_hardware_8_graphics_9_composer3_23_ChangedCompositionLayer;
}
