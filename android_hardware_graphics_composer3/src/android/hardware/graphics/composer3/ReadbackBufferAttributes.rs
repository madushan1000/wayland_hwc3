#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#ReadbackBufferAttributes {
  pub r#format: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat,
  pub r#dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace,
}
impl Default for r#ReadbackBufferAttributes {
  fn default() -> Self {
    Self {
      r#format: Default::default(),
      r#dataspace: Default::default(),
    }
  }
}
impl binder::Parcelable for r#ReadbackBufferAttributes {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#format)?;
      subparcel.write(&self.r#dataspace)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#format = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#dataspace = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#ReadbackBufferAttributes);
binder::impl_deserialize_for_parcelable!(r#ReadbackBufferAttributes);
impl binder::binder_impl::ParcelableMetadata for r#ReadbackBufferAttributes {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ReadbackBufferAttributes" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#ReadbackBufferAttributes as _7_android_8_hardware_8_graphics_9_composer3_24_ReadbackBufferAttributes;
}
