#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#Hdr : [i32; 6] {
    r#INVALID = 0,
    r#DOLBY_VISION = 1,
    r#HDR10 = 2,
    r#HLG = 3,
    r#HDR10_PLUS = 4,
    r#DOLBY_VISION_4K30 = 5,
  }
}
pub(crate) mod mangled {
 pub use super::r#Hdr as _7_android_8_hardware_8_graphics_6_common_3_Hdr;
}
