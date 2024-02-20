#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#ColorMode : [i32; 14] {
    r#NATIVE = 0,
    r#STANDARD_BT601_625 = 1,
    r#STANDARD_BT601_625_UNADJUSTED = 2,
    r#STANDARD_BT601_525 = 3,
    r#STANDARD_BT601_525_UNADJUSTED = 4,
    r#STANDARD_BT709 = 5,
    r#DCI_P3 = 6,
    r#SRGB = 7,
    r#ADOBE_RGB = 8,
    r#DISPLAY_P3 = 9,
    r#BT2020 = 10,
    r#BT2100_PQ = 11,
    r#BT2100_HLG = 12,
    r#DISPLAY_BT2020 = 13,
  }
}
pub(crate) mod mangled {
 pub use super::r#ColorMode as _7_android_8_hardware_8_graphics_9_composer3_9_ColorMode;
}
