#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#Transform : [i32; 6] {
    r#NONE = 0,
    r#FLIP_H = 1,
    r#FLIP_V = 2,
    r#ROT_90 = 4,
    r#ROT_180 = 3,
    r#ROT_270 = 7,
  }
}
pub(crate) mod mangled {
 pub use super::r#Transform as _7_android_8_hardware_8_graphics_6_common_9_Transform;
}
