#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#HardwareBufferDescription {
  pub r#width: i32,
  pub r#height: i32,
  pub r#layers: i32,
  pub r#format: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat,
  pub r#usage: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_BufferUsage,
  pub r#stride: i32,
}
impl Default for r#HardwareBufferDescription {
  fn default() -> Self {
    Self {
      r#width: 0,
      r#height: 0,
      r#layers: 0,
      r#format: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat::UNSPECIFIED,
      r#usage: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_BufferUsage::CPU_READ_NEVER,
      r#stride: 0,
    }
  }
}
impl binder::Parcelable for r#HardwareBufferDescription {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#width)?;
      subparcel.write(&self.r#height)?;
      subparcel.write(&self.r#layers)?;
      subparcel.write(&self.r#format)?;
      subparcel.write(&self.r#usage)?;
      subparcel.write(&self.r#stride)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#width = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#height = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#layers = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#format = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#usage = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#stride = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#HardwareBufferDescription);
binder::impl_deserialize_for_parcelable!(r#HardwareBufferDescription);
impl binder::binder_impl::ParcelableMetadata for r#HardwareBufferDescription {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.common.HardwareBufferDescription" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#HardwareBufferDescription as _7_android_8_hardware_8_graphics_6_common_25_HardwareBufferDescription;
}
