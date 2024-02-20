#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#ParcelableBlendMode {
  pub r#blendMode: crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_BlendMode,
}
impl Default for r#ParcelableBlendMode {
  fn default() -> Self {
    Self {
      r#blendMode: Default::default(),
    }
  }
}
impl binder::Parcelable for r#ParcelableBlendMode {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#blendMode)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#blendMode = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#ParcelableBlendMode);
binder::impl_deserialize_for_parcelable!(r#ParcelableBlendMode);
impl binder::binder_impl::ParcelableMetadata for r#ParcelableBlendMode {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ParcelableBlendMode" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#ParcelableBlendMode as _7_android_8_hardware_8_graphics_9_composer3_19_ParcelableBlendMode;
}
