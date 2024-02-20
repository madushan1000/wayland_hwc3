#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#DisplayBrightness {
  pub r#brightness: f32,
  pub r#brightnessNits: f32,
}
impl Default for r#DisplayBrightness {
  fn default() -> Self {
    Self {
      r#brightness: 0.000000f32,
      r#brightnessNits: 0.000000f32,
    }
  }
}
impl binder::Parcelable for r#DisplayBrightness {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#brightness)?;
      subparcel.write(&self.r#brightnessNits)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#brightness = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#brightnessNits = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#DisplayBrightness);
binder::impl_deserialize_for_parcelable!(r#DisplayBrightness);
impl binder::binder_impl::ParcelableMetadata for r#DisplayBrightness {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.DisplayBrightness" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#DisplayBrightness as _7_android_8_hardware_8_graphics_9_composer3_17_DisplayBrightness;
}
