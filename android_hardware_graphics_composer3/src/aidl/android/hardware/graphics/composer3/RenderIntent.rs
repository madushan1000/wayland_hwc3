#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#RenderIntent : [i32; 4] {
    r#COLORIMETRIC = 0,
    r#ENHANCE = 1,
    r#TONE_MAP_COLORIMETRIC = 2,
    r#TONE_MAP_ENHANCE = 3,
  }
}
pub(crate) mod mangled {
 pub use super::r#RenderIntent as _7_android_8_hardware_8_graphics_9_composer3_12_RenderIntent;
}
