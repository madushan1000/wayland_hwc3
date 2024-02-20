#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#Rect {
  pub r#left: i32,
  pub r#top: i32,
  pub r#right: i32,
  pub r#bottom: i32,
}
impl Default for r#Rect {
  fn default() -> Self {
    Self {
      r#left: 0,
      r#top: 0,
      r#right: 0,
      r#bottom: 0,
    }
  }
}
impl binder::Parcelable for r#Rect {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#left)?;
      subparcel.write(&self.r#top)?;
      subparcel.write(&self.r#right)?;
      subparcel.write(&self.r#bottom)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#left = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#top = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#right = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#bottom = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#Rect);
binder::impl_deserialize_for_parcelable!(r#Rect);
impl binder::binder_impl::ParcelableMetadata for r#Rect {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.common.Rect" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#Rect as _7_android_8_hardware_8_graphics_6_common_4_Rect;
}
