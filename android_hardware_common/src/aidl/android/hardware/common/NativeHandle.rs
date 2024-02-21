#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#NativeHandle {
  pub r#fds: Vec<binder::ParcelFileDescriptor>,
  pub r#ints: Vec<i32>,
}
impl Default for r#NativeHandle {
  fn default() -> Self {
    Self {
      r#fds: Default::default(),
      r#ints: Default::default(),
    }
  }
}
impl binder::Parcelable for r#NativeHandle {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#fds)?;
      subparcel.write(&self.r#ints)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#fds = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#ints = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#NativeHandle);
binder::impl_deserialize_for_parcelable!(r#NativeHandle);
impl binder::binder_impl::ParcelableMetadata for r#NativeHandle {
  fn get_descriptor() -> &'static str { "android.hardware.common.NativeHandle" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#NativeHandle as _7_android_8_hardware_6_common_12_NativeHandle;
}
