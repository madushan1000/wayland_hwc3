#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#ParcelableDataspace {
  pub r#dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace,
}
impl Default for r#ParcelableDataspace {
  fn default() -> Self {
    Self {
      r#dataspace: Default::default(),
    }
  }
}
impl binder::Parcelable for r#ParcelableDataspace {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#dataspace)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#dataspace = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#ParcelableDataspace);
binder::impl_deserialize_for_parcelable!(r#ParcelableDataspace);
impl binder::binder_impl::ParcelableMetadata for r#ParcelableDataspace {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ParcelableDataspace" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#ParcelableDataspace as _7_android_8_hardware_8_graphics_9_composer3_19_ParcelableDataspace;
}
