#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#ClientTargetPropertyWithBrightness {
  pub r#display: i64,
  pub r#clientTargetProperty: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_20_ClientTargetProperty,
  pub r#brightness: f32,
  pub r#dimmingStage: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_DimmingStage,
}
impl Default for r#ClientTargetPropertyWithBrightness {
  fn default() -> Self {
    Self {
      r#display: 0,
      r#clientTargetProperty: Default::default(),
      r#brightness: 0.000000f32,
      r#dimmingStage: Default::default(),
    }
  }
}
impl binder::Parcelable for r#ClientTargetPropertyWithBrightness {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#display)?;
      subparcel.write(&self.r#clientTargetProperty)?;
      subparcel.write(&self.r#brightness)?;
      subparcel.write(&self.r#dimmingStage)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#display = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#clientTargetProperty = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#brightness = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#dimmingStage = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#ClientTargetPropertyWithBrightness);
binder::impl_deserialize_for_parcelable!(r#ClientTargetPropertyWithBrightness);
impl binder::binder_impl::ParcelableMetadata for r#ClientTargetPropertyWithBrightness {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ClientTargetPropertyWithBrightness" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#ClientTargetPropertyWithBrightness as _7_android_8_hardware_8_graphics_9_composer3_34_ClientTargetPropertyWithBrightness;
}
