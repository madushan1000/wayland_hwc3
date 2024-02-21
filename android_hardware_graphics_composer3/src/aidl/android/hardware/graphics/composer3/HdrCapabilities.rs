#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#HdrCapabilities {
  pub r#types: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_3_Hdr>,
  pub r#maxLuminance: f32,
  pub r#maxAverageLuminance: f32,
  pub r#minLuminance: f32,
}
impl Default for r#HdrCapabilities {
  fn default() -> Self {
    Self {
      r#types: Default::default(),
      r#maxLuminance: 0.000000f32,
      r#maxAverageLuminance: 0.000000f32,
      r#minLuminance: 0.000000f32,
    }
  }
}
impl binder::Parcelable for r#HdrCapabilities {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#types)?;
      subparcel.write(&self.r#maxLuminance)?;
      subparcel.write(&self.r#maxAverageLuminance)?;
      subparcel.write(&self.r#minLuminance)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#types = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#maxLuminance = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#maxAverageLuminance = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#minLuminance = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#HdrCapabilities);
binder::impl_deserialize_for_parcelable!(r#HdrCapabilities);
impl binder::binder_impl::ParcelableMetadata for r#HdrCapabilities {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.HdrCapabilities" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#HdrCapabilities as _7_android_8_hardware_8_graphics_9_composer3_15_HdrCapabilities;
}
