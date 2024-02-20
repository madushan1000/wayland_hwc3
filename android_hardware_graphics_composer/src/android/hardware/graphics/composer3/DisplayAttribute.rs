#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#DisplayAttribute : [i32; 7] {
    r#INVALID = 0,
    r#WIDTH = 1,
    r#HEIGHT = 2,
    r#VSYNC_PERIOD = 3,
    r#DPI_X = 4,
    r#DPI_Y = 5,
    r#CONFIG_GROUP = 7,
  }
}
pub(crate) mod mangled {
 pub use super::r#DisplayAttribute as _7_android_8_hardware_8_graphics_9_composer3_16_DisplayAttribute;
}
