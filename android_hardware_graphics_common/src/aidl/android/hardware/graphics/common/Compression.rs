#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#Compression : [i64; 2] {
    r#NONE = 0,
    r#DISPLAY_STREAM_COMPRESSION = 1,
  }
}
pub(crate) mod mangled {
 pub use super::r#Compression as _7_android_8_hardware_8_graphics_6_common_11_Compression;
}
