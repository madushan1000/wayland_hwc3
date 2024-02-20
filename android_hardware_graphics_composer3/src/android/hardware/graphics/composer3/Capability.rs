#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#Capability : [i32; 8] {
    r#INVALID = 0,
    r#SIDEBAND_STREAM = 1,
    r#SKIP_CLIENT_COLOR_TRANSFORM = 2,
    r#PRESENT_FENCE_IS_NOT_RELIABLE = 3,
    #[deprecated = "- enabled by default."]
    r#SKIP_VALIDATE = 4,
    r#BOOT_DISPLAY_CONFIG = 5,
    r#HDR_OUTPUT_CONVERSION_CONFIG = 6,
    r#REFRESH_RATE_CHANGED_CALLBACK_DEBUG = 7,
  }
}
pub(crate) mod mangled {
 pub use super::r#Capability as _7_android_8_hardware_8_graphics_9_composer3_10_Capability;
}
