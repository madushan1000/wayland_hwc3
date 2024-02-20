#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#AlphaInterpretation : [i32; 2] {
    r#COVERAGE = 0,
    r#MASK = 1,
  }
}
pub(crate) mod mangled {
 pub use super::r#AlphaInterpretation as _7_android_8_hardware_8_graphics_6_common_19_AlphaInterpretation;
}
