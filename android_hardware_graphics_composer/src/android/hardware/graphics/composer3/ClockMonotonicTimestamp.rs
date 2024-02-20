#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#ClockMonotonicTimestamp {
  pub r#timestampNanos: i64,
}
impl Default for r#ClockMonotonicTimestamp {
  fn default() -> Self {
    Self {
      r#timestampNanos: 0,
    }
  }
}
impl binder::Parcelable for r#ClockMonotonicTimestamp {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#timestampNanos)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#timestampNanos = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#ClockMonotonicTimestamp);
binder::impl_deserialize_for_parcelable!(r#ClockMonotonicTimestamp);
impl binder::binder_impl::ParcelableMetadata for r#ClockMonotonicTimestamp {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ClockMonotonicTimestamp" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#ClockMonotonicTimestamp as _7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp;
}
