#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#ClientTarget {
  pub r#buffer: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_6_Buffer,
  pub r#dataspace: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace,
  pub r#damage: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_4_Rect>,
}
impl Default for r#ClientTarget {
  fn default() -> Self {
    Self {
      r#buffer: Default::default(),
      r#dataspace: Default::default(),
      r#damage: Default::default(),
    }
  }
}
impl binder::Parcelable for r#ClientTarget {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#buffer)?;
      subparcel.write(&self.r#dataspace)?;
      subparcel.write(&self.r#damage)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#buffer = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#dataspace = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#damage = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#ClientTarget);
binder::impl_deserialize_for_parcelable!(r#ClientTarget);
impl binder::binder_impl::ParcelableMetadata for r#ClientTarget {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ClientTarget" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#ClientTarget as _7_android_8_hardware_8_graphics_9_composer3_12_ClientTarget;
}
