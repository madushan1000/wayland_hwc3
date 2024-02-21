#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#ReleaseFences {
  pub r#display: i64,
  pub r#layers: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_13_ReleaseFences_5_Layer>,
}
impl Default for r#ReleaseFences {
  fn default() -> Self {
    Self {
      r#display: 0,
      r#layers: Default::default(),
    }
  }
}
impl binder::Parcelable for r#ReleaseFences {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#display)?;
      subparcel.write(&self.r#layers)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#display = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#layers = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#ReleaseFences);
binder::impl_deserialize_for_parcelable!(r#ReleaseFences);
impl binder::binder_impl::ParcelableMetadata for r#ReleaseFences {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ReleaseFences" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub mod r#Layer {
  #[derive(Debug)]
  pub struct r#Layer {
    pub r#layer: i64,
    pub r#fence: Option<binder::ParcelFileDescriptor>,
  }
  impl Default for r#Layer {
    fn default() -> Self {
      Self {
        r#layer: 0,
        r#fence: Default::default(),
      }
    }
  }
  impl binder::Parcelable for r#Layer {
    fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
      parcel.sized_write(|subparcel| {
        subparcel.write(&self.r#layer)?;
        let __field_ref = self.r#fence.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
        subparcel.write(__field_ref)?;
        Ok(())
      })
    }
    fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
      parcel.sized_read(|subparcel| {
        if subparcel.has_more_data() {
          self.r#layer = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.r#fence = Some(subparcel.read()?);
        }
        Ok(())
      })
    }
  }
  binder::impl_serialize_for_parcelable!(r#Layer);
  binder::impl_deserialize_for_parcelable!(r#Layer);
  impl binder::binder_impl::ParcelableMetadata for r#Layer {
    fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.ReleaseFences.Layer" }
    fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
  }
}
pub(crate) mod mangled {
 pub use super::r#ReleaseFences as _7_android_8_hardware_8_graphics_9_composer3_13_ReleaseFences;
 pub use super::r#Layer::r#Layer as _7_android_8_hardware_8_graphics_9_composer3_13_ReleaseFences_5_Layer;
}
