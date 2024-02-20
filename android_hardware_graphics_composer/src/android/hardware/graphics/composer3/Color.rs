#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#Color {
  pub r#r: f32,
  pub r#g: f32,
  pub r#b: f32,
  pub r#a: f32,
}
impl Default for r#Color {
  fn default() -> Self {
    Self {
      r#r: 0.000000f32,
      r#g: 0.000000f32,
      r#b: 0.000000f32,
      r#a: 0.000000f32,
    }
  }
}
impl binder::Parcelable for r#Color {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#r)?;
      subparcel.write(&self.r#g)?;
      subparcel.write(&self.r#b)?;
      subparcel.write(&self.r#a)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#r = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#g = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#b = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#a = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#Color);
binder::impl_deserialize_for_parcelable!(r#Color);
impl binder::binder_impl::ParcelableMetadata for r#Color {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.Color" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#Color as _7_android_8_hardware_8_graphics_9_composer3_5_Color;
}
