#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#HardwareBuffer {
  pub r#description: crate::mangled::_7_android_8_hardware_8_graphics_6_common_25_HardwareBufferDescription,
  pub r#handle: crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle,
}
impl Default for r#HardwareBuffer {
  fn default() -> Self {
    Self {
      r#description: Default::default(),
      r#handle: Default::default(),
    }
  }
}
impl binder::Parcelable for r#HardwareBuffer {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#description)?;
      subparcel.write(&self.r#handle)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#description = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#handle = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#HardwareBuffer);
binder::impl_deserialize_for_parcelable!(r#HardwareBuffer);
impl binder::binder_impl::ParcelableMetadata for r#HardwareBuffer {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.common.HardwareBuffer" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#HardwareBuffer as _7_android_8_hardware_8_graphics_6_common_14_HardwareBuffer;
}
