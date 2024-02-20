#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#OverlayProperties {
  pub r#combinations: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties_27_SupportedBufferCombinations>,
  pub r#supportMixedColorSpaces: bool,
}
impl Default for r#OverlayProperties {
  fn default() -> Self {
    Self {
      r#combinations: Default::default(),
      r#supportMixedColorSpaces: false,
    }
  }
}
impl binder::Parcelable for r#OverlayProperties {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#combinations)?;
      subparcel.write(&self.r#supportMixedColorSpaces)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#combinations = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#supportMixedColorSpaces = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#OverlayProperties);
binder::impl_deserialize_for_parcelable!(r#OverlayProperties);
impl binder::binder_impl::ParcelableMetadata for r#OverlayProperties {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.OverlayProperties" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub mod r#SupportedBufferCombinations {
  #[derive(Debug)]
  pub struct r#SupportedBufferCombinations {
    pub r#pixelFormats: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_11_PixelFormat>,
    pub r#standards: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace>,
    pub r#transfers: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace>,
    pub r#ranges: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_9_Dataspace>,
  }
  impl Default for r#SupportedBufferCombinations {
    fn default() -> Self {
      Self {
        r#pixelFormats: Default::default(),
        r#standards: Default::default(),
        r#transfers: Default::default(),
        r#ranges: Default::default(),
      }
    }
  }
  impl binder::Parcelable for r#SupportedBufferCombinations {
    fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
      parcel.sized_write(|subparcel| {
        subparcel.write(&self.r#pixelFormats)?;
        subparcel.write(&self.r#standards)?;
        subparcel.write(&self.r#transfers)?;
        subparcel.write(&self.r#ranges)?;
        Ok(())
      })
    }
    fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
      parcel.sized_read(|subparcel| {
        if subparcel.has_more_data() {
          self.r#pixelFormats = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.r#standards = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.r#transfers = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.r#ranges = subparcel.read()?;
        }
        Ok(())
      })
    }
  }
  binder::impl_serialize_for_parcelable!(r#SupportedBufferCombinations);
  binder::impl_deserialize_for_parcelable!(r#SupportedBufferCombinations);
  impl binder::binder_impl::ParcelableMetadata for r#SupportedBufferCombinations {
    fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.OverlayProperties.SupportedBufferCombinations" }
    fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
  }
}
pub(crate) mod mangled {
 pub use super::r#OverlayProperties as _7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties;
 pub use super::r#SupportedBufferCombinations::r#SupportedBufferCombinations as _7_android_8_hardware_8_graphics_9_composer3_17_OverlayProperties_27_SupportedBufferCombinations;
}
