#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#PlaneLayoutComponentType : [i64; 8] {
    r#Y = 1,
    r#CB = 2,
    r#CR = 4,
    r#R = 1024,
    r#G = 2048,
    r#B = 4096,
    r#RAW = 1048576,
    r#A = 1073741824,
  }
}
pub(crate) mod mangled {
 pub use super::r#PlaneLayoutComponentType as _7_android_8_hardware_8_graphics_6_common_24_PlaneLayoutComponentType;
}
