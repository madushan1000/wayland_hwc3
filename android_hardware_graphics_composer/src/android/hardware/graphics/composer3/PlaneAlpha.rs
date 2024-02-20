#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#PlaneAlpha {
  pub r#alpha: f32,
}
impl Default for r#PlaneAlpha {
  fn default() -> Self {
    Self {
      r#alpha: 0.000000f32,
    }
  }
}
impl binder::Parcelable for r#PlaneAlpha {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#alpha)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#alpha = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#PlaneAlpha);
binder::impl_deserialize_for_parcelable!(r#PlaneAlpha);
impl binder::binder_impl::ParcelableMetadata for r#PlaneAlpha {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.PlaneAlpha" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#PlaneAlpha as _7_android_8_hardware_8_graphics_9_composer3_10_PlaneAlpha;
}
