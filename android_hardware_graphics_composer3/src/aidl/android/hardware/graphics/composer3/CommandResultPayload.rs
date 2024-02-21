#![forbid(unsafe_code)]
#![rustfmt::skip]
#[derive(Debug)]
pub enum r#CommandResultPayload {
  Error(crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_CommandError),
  ChangedCompositionTypes(crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ChangedCompositionTypes),
  DisplayRequest(crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayRequest),
  PresentFence(crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_PresentFence),
  ReleaseFences(crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_13_ReleaseFences),
  PresentOrValidateResult(crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_PresentOrValidate),
  ClientTargetProperty(crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_34_ClientTargetPropertyWithBrightness),
}
impl Default for r#CommandResultPayload {
  fn default() -> Self {
    Self::Error(Default::default())
  }
}
impl binder::Parcelable for r#CommandResultPayload {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    match self {
      Self::Error(v) => {
        parcel.write(&0i32)?;
        parcel.write(v)
      }
      Self::ChangedCompositionTypes(v) => {
        parcel.write(&1i32)?;
        parcel.write(v)
      }
      Self::DisplayRequest(v) => {
        parcel.write(&2i32)?;
        parcel.write(v)
      }
      Self::PresentFence(v) => {
        parcel.write(&3i32)?;
        parcel.write(v)
      }
      Self::ReleaseFences(v) => {
        parcel.write(&4i32)?;
        parcel.write(v)
      }
      Self::PresentOrValidateResult(v) => {
        parcel.write(&5i32)?;
        parcel.write(v)
      }
      Self::ClientTargetProperty(v) => {
        parcel.write(&6i32)?;
        parcel.write(v)
      }
    }
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    let tag: i32 = parcel.read()?;
    match tag {
      0 => {
        let value: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_CommandError = parcel.read()?;
        *self = Self::Error(value);
        Ok(())
      }
      1 => {
        let value: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_23_ChangedCompositionTypes = parcel.read()?;
        *self = Self::ChangedCompositionTypes(value);
        Ok(())
      }
      2 => {
        let value: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_14_DisplayRequest = parcel.read()?;
        *self = Self::DisplayRequest(value);
        Ok(())
      }
      3 => {
        let value: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_12_PresentFence = parcel.read()?;
        *self = Self::PresentFence(value);
        Ok(())
      }
      4 => {
        let value: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_13_ReleaseFences = parcel.read()?;
        *self = Self::ReleaseFences(value);
        Ok(())
      }
      5 => {
        let value: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_17_PresentOrValidate = parcel.read()?;
        *self = Self::PresentOrValidateResult(value);
        Ok(())
      }
      6 => {
        let value: crate::mangled::_7_android_8_hardware_8_graphics_9_composer3_34_ClientTargetPropertyWithBrightness = parcel.read()?;
        *self = Self::ClientTargetProperty(value);
        Ok(())
      }
      _ => {
        Err(binder::StatusCode::BAD_VALUE)
      }
    }
  }
}
binder::impl_serialize_for_parcelable!(r#CommandResultPayload);
binder::impl_deserialize_for_parcelable!(r#CommandResultPayload);
impl binder::binder_impl::ParcelableMetadata for r#CommandResultPayload {
  fn get_descriptor() -> &'static str { "android.hardware.graphics.composer3.CommandResultPayload" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub mod r#Tag {
  #![allow(non_upper_case_globals)]
  use binder::declare_binder_enum;
  declare_binder_enum! {
    r#Tag : [i32; 7] {
      r#error = 0,
      r#changedCompositionTypes = 1,
      r#displayRequest = 2,
      r#presentFence = 3,
      r#releaseFences = 4,
      r#presentOrValidateResult = 5,
      r#clientTargetProperty = 6,
    }
  }
}
pub(crate) mod mangled {
 pub use super::r#CommandResultPayload as _7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload;
 pub use super::r#Tag::r#Tag as _7_android_8_hardware_8_graphics_9_composer3_20_CommandResultPayload_3_Tag;
}
