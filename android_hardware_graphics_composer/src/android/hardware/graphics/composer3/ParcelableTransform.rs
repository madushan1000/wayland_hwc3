#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#ParcelableTransform {
  pub r#transform: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Transform,
}
impl Default for r#ParcelableTransform {
  fn default() -> Self {
    Self {
      r#transform: Default::default(),
    }
  }
}
impl binder::Parcelable for r#ParcelableTransform {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#transform)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#transform = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#ParcelableTransform);
binder::impl_deserialize_for_parcelable!(r#ParcelableTransform);
impl binder::binder_impl::ParcelableMetadata for r#ParcelableTransform {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ParcelableTransform" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#ParcelableTransform as _7_android_8_hardware_8_graphics_9_composer3_19_ParcelableTransform;
}
