#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#PerFrameMetadataKey : [i32; 13] {
    r#DISPLAY_RED_PRIMARY_X = 0,
    r#DISPLAY_RED_PRIMARY_Y = 1,
    r#DISPLAY_GREEN_PRIMARY_X = 2,
    r#DISPLAY_GREEN_PRIMARY_Y = 3,
    r#DISPLAY_BLUE_PRIMARY_X = 4,
    r#DISPLAY_BLUE_PRIMARY_Y = 5,
    r#WHITE_POINT_X = 6,
    r#WHITE_POINT_Y = 7,
    r#MAX_LUMINANCE = 8,
    r#MIN_LUMINANCE = 9,
    r#MAX_CONTENT_LIGHT_LEVEL = 10,
    r#MAX_FRAME_AVERAGE_LIGHT_LEVEL = 11,
    r#HDR10_PLUS_SEI = 12,
  }
}
pub(crate) mod mangled {
 pub use super::r#PerFrameMetadataKey as _7_android_8_hardware_8_graphics_9_composer3_19_PerFrameMetadataKey;
}
