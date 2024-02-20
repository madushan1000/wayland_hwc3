#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#PlaneLayoutComponent {
  pub r#type: crate::mangled::_7_android_8_hardware_8_graphics_6_common_14_ExtendableType,
  pub r#offsetInBits: i64,
  pub r#sizeInBits: i64,
}
impl Default for r#PlaneLayoutComponent {
  fn default() -> Self {
    Self {
      r#type: Default::default(),
      r#offsetInBits: 0,
      r#sizeInBits: 0,
    }
  }
}
impl binder::Parcelable for r#PlaneLayoutComponent {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#type)?;
      subparcel.write(&self.r#offsetInBits)?;
      subparcel.write(&self.r#sizeInBits)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#type = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#offsetInBits = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#sizeInBits = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#PlaneLayoutComponent);
binder::impl_deserialize_for_parcelable!(r#PlaneLayoutComponent);
impl binder::binder_impl::ParcelableMetadata for r#PlaneLayoutComponent {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.common.PlaneLayoutComponent" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#PlaneLayoutComponent as _7_android_8_hardware_8_graphics_6_common_20_PlaneLayoutComponent;
}
