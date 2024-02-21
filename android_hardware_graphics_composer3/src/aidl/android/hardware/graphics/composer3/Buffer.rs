#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#Buffer {
  pub r#slot: i32,
  pub r#handle: Option<crate::mangled::_7_android_8_hardware_6_common_12_NativeHandle>,
  pub r#fence: Option<binder::ParcelFileDescriptor>,
}
impl Default for r#Buffer {
  fn default() -> Self {
    Self {
      r#slot: 0,
      r#handle: Default::default(),
      r#fence: Default::default(),
    }
  }
}
impl binder::Parcelable for r#Buffer {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#slot)?;
      subparcel.write(&self.r#handle)?;
      subparcel.write(&self.r#fence)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#slot = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#handle = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#fence = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#Buffer);
binder::impl_deserialize_for_parcelable!(r#Buffer);
impl binder::binder_impl::ParcelableMetadata for r#Buffer {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.Buffer" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#Buffer as _7_android_8_hardware_8_graphics_9_composer3_6_Buffer;
}
