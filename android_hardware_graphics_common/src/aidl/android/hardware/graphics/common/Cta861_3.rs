#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#Cta861_3 {
  pub r#maxContentLightLevel: f32,
  pub r#maxFrameAverageLightLevel: f32,
}
impl Default for r#Cta861_3 {
  fn default() -> Self {
    Self {
      r#maxContentLightLevel: 0.000000f32,
      r#maxFrameAverageLightLevel: 0.000000f32,
    }
  }
}
impl binder::Parcelable for r#Cta861_3 {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#maxContentLightLevel)?;
      subparcel.write(&self.r#maxFrameAverageLightLevel)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#maxContentLightLevel = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#maxFrameAverageLightLevel = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#Cta861_3);
binder::impl_deserialize_for_parcelable!(r#Cta861_3);
impl binder::binder_impl::ParcelableMetadata for r#Cta861_3 {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.common.Cta861_3" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#Cta861_3 as _7_android_8_hardware_8_graphics_6_common_8_Cta861_3;
}
