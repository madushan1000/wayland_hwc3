#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#BlendMode : [i32; 4] {
    r#INVALID = 0,
    r#NONE = 1,
    r#PREMULTIPLIED = 2,
    r#COVERAGE = 3,
  }
}
pub(crate) mod mangled {
 pub use super::r#BlendMode as _7_android_8_hardware_8_graphics_6_common_9_BlendMode;
}
