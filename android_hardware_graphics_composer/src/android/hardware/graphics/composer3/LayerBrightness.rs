#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#LayerBrightness {
  pub r#brightness: f32,
}
impl Default for r#LayerBrightness {
  fn default() -> Self {
    Self {
      r#brightness: 0.000000f32,
    }
  }
}
impl binder::Parcelable for r#LayerBrightness {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#brightness)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#brightness = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#LayerBrightness);
binder::impl_deserialize_for_parcelable!(r#LayerBrightness);
impl binder::binder_impl::ParcelableMetadata for r#LayerBrightness {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.LayerBrightness" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#LayerBrightness as _7_android_8_hardware_8_graphics_9_composer3_15_LayerBrightness;
}
