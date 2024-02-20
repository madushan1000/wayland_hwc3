#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#RefreshRateChangedDebugData {
  pub r#display: i64,
  pub r#vsyncPeriodNanos: i32,
}
impl Default for r#RefreshRateChangedDebugData {
  fn default() -> Self {
    Self {
      r#display: 0,
      r#vsyncPeriodNanos: 0,
    }
  }
}
impl binder::Parcelable for r#RefreshRateChangedDebugData {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#display)?;
      subparcel.write(&self.r#vsyncPeriodNanos)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#display = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#vsyncPeriodNanos = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#RefreshRateChangedDebugData);
binder::impl_deserialize_for_parcelable!(r#RefreshRateChangedDebugData);
impl binder::binder_impl::ParcelableMetadata for r#RefreshRateChangedDebugData {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.RefreshRateChangedDebugData" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#RefreshRateChangedDebugData as _7_android_8_hardware_8_graphics_9_composer3_27_RefreshRateChangedDebugData;
}
