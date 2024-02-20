#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#DisplayDecorationSupport {
  pub r#format: crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat,
  pub r#alphaInterpretation: crate::mangled::_7_android_8_hardware_8_graphics_6_common_19_AlphaInterpretation,
}
impl Default for r#DisplayDecorationSupport {
  fn default() -> Self {
    Self {
      r#format: Default::default(),
      r#alphaInterpretation: Default::default(),
    }
  }
}
impl binder::Parcelable for r#DisplayDecorationSupport {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#format)?;
      subparcel.write(&self.r#alphaInterpretation)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#format = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#alphaInterpretation = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#DisplayDecorationSupport);
binder::impl_deserialize_for_parcelable!(r#DisplayDecorationSupport);
impl binder::binder_impl::ParcelableMetadata for r#DisplayDecorationSupport {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.common.DisplayDecorationSupport" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#DisplayDecorationSupport as _7_android_8_hardware_8_graphics_6_common_24_DisplayDecorationSupport;
}
