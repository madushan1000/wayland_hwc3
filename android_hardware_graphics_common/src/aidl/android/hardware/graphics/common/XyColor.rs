#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#XyColor {
  pub r#x: f32,
  pub r#y: f32,
}
impl Default for r#XyColor {
  fn default() -> Self {
    Self {
      r#x: 0.000000f32,
      r#y: 0.000000f32,
    }
  }
}
impl binder::Parcelable for r#XyColor {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#x)?;
      subparcel.write(&self.r#y)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#x = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#y = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#XyColor);
binder::impl_deserialize_for_parcelable!(r#XyColor);
impl binder::binder_impl::ParcelableMetadata for r#XyColor {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.common.XyColor" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#XyColor as _7_android_8_hardware_8_graphics_6_common_7_XyColor;
}
