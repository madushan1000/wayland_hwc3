#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#Composition : [i32; 8] {
    r#INVALID = 0,
    r#CLIENT = 1,
    r#DEVICE = 2,
    r#SOLID_COLOR = 3,
    r#CURSOR = 4,
    r#SIDEBAND = 5,
    r#DISPLAY_DECORATION = 6,
    r#REFRESH_RATE_INDICATOR = 7,
  }
}
pub(crate) mod mangled {
 pub use super::r#Composition as _7_android_8_hardware_8_graphics_9_composer3_11_Composition;
}
