#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#DimmingStage : [i32; 3] {
    r#NONE = 0,
    r#LINEAR = 1,
    r#GAMMA_OETF = 2,
  }
}
pub(crate) mod mangled {
 pub use super::r#DimmingStage as _7_android_8_hardware_8_graphics_9_composer3_12_DimmingStage;
}
