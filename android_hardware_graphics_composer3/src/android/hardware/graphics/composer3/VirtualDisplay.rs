#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#VirtualDisplay {
  pub r#display: i64,
  pub r#format: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat,
}
impl Default for r#VirtualDisplay {
  fn default() -> Self {
    Self {
      r#display: 0,
      r#format: Default::default(),
    }
  }
}
impl binder::Parcelable for r#VirtualDisplay {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#display)?;
      subparcel.write(&self.r#format)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#display = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#format = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#VirtualDisplay);
binder::impl_deserialize_for_parcelable!(r#VirtualDisplay);
impl binder::binder_impl::ParcelableMetadata for r#VirtualDisplay {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.VirtualDisplay" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#VirtualDisplay as _7_android_8_hardware_8_graphics_9_composer3_14_VirtualDisplay;
}
