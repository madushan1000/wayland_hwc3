#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#DisplayContentSample {
  pub r#frameCount: i64,
  pub r#sampleComponent0: Vec<i64>,
  pub r#sampleComponent1: Vec<i64>,
  pub r#sampleComponent2: Vec<i64>,
  pub r#sampleComponent3: Vec<i64>,
}
impl Default for r#DisplayContentSample {
  fn default() -> Self {
    Self {
      r#frameCount: 0,
      r#sampleComponent0: Default::default(),
      r#sampleComponent1: Default::default(),
      r#sampleComponent2: Default::default(),
      r#sampleComponent3: Default::default(),
    }
  }
}
impl binder::Parcelable for r#DisplayContentSample {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#frameCount)?;
      subparcel.write(&self.r#sampleComponent0)?;
      subparcel.write(&self.r#sampleComponent1)?;
      subparcel.write(&self.r#sampleComponent2)?;
      subparcel.write(&self.r#sampleComponent3)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#frameCount = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#sampleComponent0 = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#sampleComponent1 = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#sampleComponent2 = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#sampleComponent3 = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#DisplayContentSample);
binder::impl_deserialize_for_parcelable!(r#DisplayContentSample);
impl binder::binder_impl::ParcelableMetadata for r#DisplayContentSample {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.DisplayContentSample" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#DisplayContentSample as _7_android_8_hardware_8_graphics_9_composer3_20_DisplayContentSample;
}
