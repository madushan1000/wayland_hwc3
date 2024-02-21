#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#Point {
  pub r#x: i32,
  pub r#y: i32,
}
impl Default for r#Point {
  fn default() -> Self {
    Self {
      r#x: 0,
      r#y: 0,
    }
  }
}
impl binder::Parcelable for r#Point {
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
binder::impl_serialize_for_parcelable!(r#Point);
binder::impl_deserialize_for_parcelable!(r#Point);
impl binder::binder_impl::ParcelableMetadata for r#Point {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.common.Point" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#Point as _7_android_8_hardware_8_graphics_6_common_5_Point;
}
