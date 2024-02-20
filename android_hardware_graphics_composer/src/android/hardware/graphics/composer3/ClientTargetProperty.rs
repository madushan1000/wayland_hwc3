#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#ClientTargetProperty {
  pub r#pixelFormat: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat,
  pub r#dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace,
}
impl Default for r#ClientTargetProperty {
  fn default() -> Self {
    Self {
      r#pixelFormat: Default::default(),
      r#dataspace: Default::default(),
    }
  }
}
impl binder::Parcelable for r#ClientTargetProperty {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#pixelFormat)?;
      subparcel.write(&self.r#dataspace)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#pixelFormat = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#dataspace = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#ClientTargetProperty);
binder::impl_deserialize_for_parcelable!(r#ClientTargetProperty);
impl binder::binder_impl::ParcelableMetadata for r#ClientTargetProperty {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ClientTargetProperty" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#ClientTargetProperty as _7_android_8_hardware_8_graphics_9_composer3_20_ClientTargetProperty;
}
