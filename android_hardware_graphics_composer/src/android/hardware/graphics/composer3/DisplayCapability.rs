#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#DisplayCapability : [i32; 8] {
    r#INVALID = 0,
    r#SKIP_CLIENT_COLOR_TRANSFORM = 1,
    r#DOZE = 2,
    r#BRIGHTNESS = 3,
    r#PROTECTED_CONTENTS = 4,
    r#AUTO_LOW_LATENCY_MODE = 5,
    r#SUSPEND = 6,
    r#DISPLAY_IDLE_TIMER = 7,
  }
}
pub(crate) mod mangled {
 pub use super::r#DisplayCapability as _7_android_8_hardware_8_graphics_9_composer3_17_DisplayCapability;
}
