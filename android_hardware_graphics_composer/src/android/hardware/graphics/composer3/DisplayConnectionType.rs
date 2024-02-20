#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#DisplayConnectionType : [i32; 2] {
    r#INTERNAL = 0,
    r#EXTERNAL = 1,
  }
}
pub(crate) mod mangled {
 pub use super::r#DisplayConnectionType as _7_android_8_hardware_8_graphics_9_composer3_21_DisplayConnectionType;
}
