#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#HdrConversionCapability {
  pub r#sourceType: crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr,
  pub r#outputType: crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr,
  pub r#addsLatency: bool,
}
impl Default for r#HdrConversionCapability {
  fn default() -> Self {
    Self {
      r#sourceType: Default::default(),
      r#outputType: Default::default(),
      r#addsLatency: false,
    }
  }
}
impl binder::Parcelable for r#HdrConversionCapability {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#sourceType)?;
      subparcel.write(&self.r#outputType)?;
      subparcel.write(&self.r#addsLatency)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#sourceType = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#outputType = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#addsLatency = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#HdrConversionCapability);
binder::impl_deserialize_for_parcelable!(r#HdrConversionCapability);
impl binder::binder_impl::ParcelableMetadata for r#HdrConversionCapability {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.common.HdrConversionCapability" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#HdrConversionCapability as _7_android_8_hardware_8_graphics_6_common_23_HdrConversionCapability;
}
