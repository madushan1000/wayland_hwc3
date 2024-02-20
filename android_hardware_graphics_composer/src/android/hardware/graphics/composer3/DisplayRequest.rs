#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#DisplayRequest {
  pub r#display: i64,
  pub r#mask: i32,
  pub r#layerRequests: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayRequest_12_LayerRequest>,
}
pub const r#FLIP_CLIENT_TARGET: i32 = 1;
pub const r#WRITE_CLIENT_TARGET_TO_OUTPUT: i32 = 2;
impl Default for r#DisplayRequest {
  fn default() -> Self {
    Self {
      r#display: 0,
      r#mask: 0,
      r#layerRequests: Default::default(),
    }
  }
}
impl binder::Parcelable for r#DisplayRequest {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#display)?;
      subparcel.write(&self.r#mask)?;
      subparcel.write(&self.r#layerRequests)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#display = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#mask = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#layerRequests = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#DisplayRequest);
binder::impl_deserialize_for_parcelable!(r#DisplayRequest);
impl binder::binder_impl::ParcelableMetadata for r#DisplayRequest {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.DisplayRequest" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub mod r#LayerRequest {
  #[derive(Debug)]
  pub struct r#LayerRequest {
    pub r#layer: i64,
    pub r#mask: i32,
  }
  pub const r#CLEAR_CLIENT_TARGET: i32 = 1;
  impl Default for r#LayerRequest {
    fn default() -> Self {
      Self {
        r#layer: 0,
        r#mask: 0,
      }
    }
  }
  impl binder::Parcelable for r#LayerRequest {
    fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
      parcel.sized_write(|subparcel| {
        subparcel.write(&self.r#layer)?;
        subparcel.write(&self.r#mask)?;
        Ok(())
      })
    }
    fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
      parcel.sized_read(|subparcel| {
        if subparcel.has_more_data() {
          self.r#layer = subparcel.read()?;
        }
        if subparcel.has_more_data() {
          self.r#mask = subparcel.read()?;
        }
        Ok(())
      })
    }
  }
  binder::impl_serialize_for_parcelable!(r#LayerRequest);
  binder::impl_deserialize_for_parcelable!(r#LayerRequest);
  impl binder::binder_impl::ParcelableMetadata for r#LayerRequest {
    fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.DisplayRequest.LayerRequest" }
    fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
  }
}
pub(crate) mod mangled {
 pub use super::r#DisplayRequest as _7_android_8_hardware_8_graphics_9_composer3_14_DisplayRequest;
 pub use super::r#LayerRequest::r#LayerRequest as _7_android_8_hardware_8_graphics_9_composer3_14_DisplayRequest_12_LayerRequest;
}
