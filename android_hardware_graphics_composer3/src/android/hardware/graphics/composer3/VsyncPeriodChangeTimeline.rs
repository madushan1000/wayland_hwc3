#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#VsyncPeriodChangeTimeline {
  pub r#newVsyncAppliedTimeNanos: i64,
  pub r#refreshRequired: bool,
  pub r#refreshTimeNanos: i64,
}
impl Default for r#VsyncPeriodChangeTimeline {
  fn default() -> Self {
    Self {
      r#newVsyncAppliedTimeNanos: 0,
      r#refreshRequired: false,
      r#refreshTimeNanos: 0,
    }
  }
}
impl binder::Parcelable for r#VsyncPeriodChangeTimeline {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#newVsyncAppliedTimeNanos)?;
      subparcel.write(&self.r#refreshRequired)?;
      subparcel.write(&self.r#refreshTimeNanos)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#newVsyncAppliedTimeNanos = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#refreshRequired = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#refreshTimeNanos = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#VsyncPeriodChangeTimeline);
binder::impl_deserialize_for_parcelable!(r#VsyncPeriodChangeTimeline);
impl binder::binder_impl::ParcelableMetadata for r#VsyncPeriodChangeTimeline {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.VsyncPeriodChangeTimeline" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#VsyncPeriodChangeTimeline as _7_android_8_hardware_8_graphics_9_composer3_25_VsyncPeriodChangeTimeline;
}
