#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#ColorTransform : [i32; 7] {
    r#IDENTITY = 0,
    r#ARBITRARY_MATRIX = 1,
    r#VALUE_INVERSE = 2,
    r#GRAYSCALE = 3,
    r#CORRECT_PROTANOPIA = 4,
    r#CORRECT_DEUTERANOPIA = 5,
    r#CORRECT_TRITANOPIA = 6,
  }
}
pub(crate) mod mangled {
 pub use super::r#ColorTransform as _7_android_8_hardware_8_graphics_6_common_14_ColorTransform;
}
