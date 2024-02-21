#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#ChromaSiting : [i64; 4] {
    r#NONE = 0,
    r#UNKNOWN = 1,
    r#SITED_INTERSTITIAL = 2,
    r#COSITED_HORIZONTAL = 3,
  }
}
pub(crate) mod mangled {
 pub use super::r#ChromaSiting as _7_android_8_hardware_8_graphics_6_common_12_ChromaSiting;
}
