#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#PowerMode : [i32; 5] {
    r#OFF = 0,
    r#DOZE = 1,
    r#DOZE_SUSPEND = 3,
    r#ON = 2,
    r#ON_SUSPEND = 4,
  }
}
pub(crate) mod mangled {
 pub use super::r#PowerMode as _7_android_8_hardware_8_graphics_9_composer3_9_PowerMode;
}
