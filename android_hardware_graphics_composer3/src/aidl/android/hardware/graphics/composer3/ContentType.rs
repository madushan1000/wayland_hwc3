#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#ContentType : [i32; 5] {
    r#NONE = 0,
    r#GRAPHICS = 1,
    r#PHOTO = 2,
    r#CINEMA = 3,
    r#GAME = 4,
  }
}
pub(crate) mod mangled {
 pub use super::r#ContentType as _7_android_8_hardware_8_graphics_9_composer3_11_ContentType;
}
