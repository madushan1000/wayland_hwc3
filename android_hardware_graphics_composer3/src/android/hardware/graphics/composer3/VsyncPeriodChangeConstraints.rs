#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#VsyncPeriodChangeConstraints {
  pub r#desiredTimeNanos: i64,
  pub r#seamlessRequired: bool,
}
impl Default for r#VsyncPeriodChangeConstraints {
  fn default() -> Self {
    Self {
      r#desiredTimeNanos: 0,
      r#seamlessRequired: false,
    }
  }
}
impl binder::Parcelable for r#VsyncPeriodChangeConstraints {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#desiredTimeNanos)?;
      subparcel.write(&self.r#seamlessRequired)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#desiredTimeNanos = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#seamlessRequired = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#VsyncPeriodChangeConstraints);
binder::impl_deserialize_for_parcelable!(r#VsyncPeriodChangeConstraints);
impl binder::binder_impl::ParcelableMetadata for r#VsyncPeriodChangeConstraints {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.VsyncPeriodChangeConstraints" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#VsyncPeriodChangeConstraints as _7_android_8_hardware_8_graphics_9_composer3_28_VsyncPeriodChangeConstraints;
}
