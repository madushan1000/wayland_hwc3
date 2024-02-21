#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#FormatColorComponent : [i8; 4] {
    r#FORMAT_COMPONENT_0 = 1,
    r#FORMAT_COMPONENT_1 = 2,
    r#FORMAT_COMPONENT_2 = 4,
    r#FORMAT_COMPONENT_3 = 8,
  }
}
pub(crate) mod mangled {
 pub use super::r#FormatColorComponent as _7_android_8_hardware_8_graphics_9_composer3_20_FormatColorComponent;
}
