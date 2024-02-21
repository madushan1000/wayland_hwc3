#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub struct r#DisplayCommand {
  pub r#display: i64,
  pub r#layers: Vec<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_LayerCommand>,
  pub r#colorTransformMatrix: Option<Vec<f32>>,
  pub r#brightness: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_DisplayBrightness>,
  pub r#clientTarget: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_ClientTarget>,
  pub r#virtualDisplayOutputBuffer: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_6_Buffer>,
  pub r#expectedPresentTime: Option<crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ClockMonotonicTimestamp>,
  pub r#validateDisplay: bool,
  pub r#acceptDisplayChanges: bool,
  pub r#presentDisplay: bool,
  pub r#presentOrValidateDisplay: bool,
}
impl Default for r#DisplayCommand {
  fn default() -> Self {
    Self {
      r#display: 0,
      r#layers: Default::default(),
      r#colorTransformMatrix: Default::default(),
      r#brightness: Default::default(),
      r#clientTarget: Default::default(),
      r#virtualDisplayOutputBuffer: Default::default(),
      r#expectedPresentTime: Default::default(),
      r#validateDisplay: false,
      r#acceptDisplayChanges: false,
      r#presentDisplay: false,
      r#presentOrValidateDisplay: false,
    }
  }
}
impl binder::Parcelable for r#DisplayCommand {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#display)?;
      subparcel.write(&self.r#layers)?;
      subparcel.write(&self.r#colorTransformMatrix)?;
      subparcel.write(&self.r#brightness)?;
      subparcel.write(&self.r#clientTarget)?;
      subparcel.write(&self.r#virtualDisplayOutputBuffer)?;
      subparcel.write(&self.r#expectedPresentTime)?;
      subparcel.write(&self.r#validateDisplay)?;
      subparcel.write(&self.r#acceptDisplayChanges)?;
      subparcel.write(&self.r#presentDisplay)?;
      subparcel.write(&self.r#presentOrValidateDisplay)?;
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
      if subparcel.has_more_data() {
        self.r#colorTransformMatrix = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#brightness = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#clientTarget = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#virtualDisplayOutputBuffer = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#expectedPresentTime = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#validateDisplay = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#acceptDisplayChanges = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#presentDisplay = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#presentOrValidateDisplay = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#DisplayCommand);
binder::impl_deserialize_for_parcelable!(r#DisplayCommand);
impl binder::binder_impl::ParcelableMetadata for r#DisplayCommand {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.DisplayCommand" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#DisplayCommand as _7_android_8_hardware_8_graphics_9_composer3_14_DisplayCommand;
}
