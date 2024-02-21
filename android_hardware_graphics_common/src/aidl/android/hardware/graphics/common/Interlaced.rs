#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#Interlaced : [i64; 3] {
    r#NONE = 0,
    r#TOP_BOTTOM = 1,
    r#RIGHT_LEFT = 2,
  }
}
pub(crate) mod mangled {
 pub use super::r#Interlaced as _7_android_8_hardware_8_graphics_6_common_10_Interlaced;
}
