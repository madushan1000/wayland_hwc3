#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#PlaneLayout {
  pub r#components: Vec<crate::mangled::_7_android_8_hardware_8_graphics_6_common_20_PlaneLayoutComponent>,
  pub r#offsetInBytes: i64,
  pub r#sampleIncrementInBits: i64,
  pub r#strideInBytes: i64,
  pub r#widthInSamples: i64,
  pub r#heightInSamples: i64,
  pub r#totalSizeInBytes: i64,
  pub r#horizontalSubsampling: i64,
  pub r#verticalSubsampling: i64,
}
impl Default for r#PlaneLayout {
  fn default() -> Self {
    Self {
      r#components: Default::default(),
      r#offsetInBytes: 0,
      r#sampleIncrementInBits: 0,
      r#strideInBytes: 0,
      r#widthInSamples: 0,
      r#heightInSamples: 0,
      r#totalSizeInBytes: 0,
      r#horizontalSubsampling: 0,
      r#verticalSubsampling: 0,
    }
  }
}
impl binder::Parcelable for r#PlaneLayout {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#components)?;
      subparcel.write(&self.r#offsetInBytes)?;
      subparcel.write(&self.r#sampleIncrementInBits)?;
      subparcel.write(&self.r#strideInBytes)?;
      subparcel.write(&self.r#widthInSamples)?;
      subparcel.write(&self.r#heightInSamples)?;
      subparcel.write(&self.r#totalSizeInBytes)?;
      subparcel.write(&self.r#horizontalSubsampling)?;
      subparcel.write(&self.r#verticalSubsampling)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#components = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#offsetInBytes = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#sampleIncrementInBits = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#strideInBytes = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#widthInSamples = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#heightInSamples = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#totalSizeInBytes = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#horizontalSubsampling = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#verticalSubsampling = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#PlaneLayout);
binder::impl_deserialize_for_parcelable!(r#PlaneLayout);
impl binder::binder_impl::ParcelableMetadata for r#PlaneLayout {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.common.PlaneLayout" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#PlaneLayout as _7_android_8_hardware_8_graphics_6_common_11_PlaneLayout;
}
