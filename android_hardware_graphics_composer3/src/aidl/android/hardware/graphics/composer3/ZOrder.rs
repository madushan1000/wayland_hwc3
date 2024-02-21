#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#ZOrder {
  pub r#z: i32,
}
impl Default for r#ZOrder {
  fn default() -> Self {
    Self {
      r#z: 0,
    }
  }
}
impl binder::Parcelable for r#ZOrder {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#z)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#z = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#ZOrder);
binder::impl_deserialize_for_parcelable!(r#ZOrder);
impl binder::binder_impl::ParcelableMetadata for r#ZOrder {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ZOrder" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#ZOrder as _7_android_8_hardware_8_graphics_9_composer3_6_ZOrder;
}
