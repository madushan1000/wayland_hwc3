#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#StandardMetadataType : [i64; 24] {
    r#INVALID = 0,
    r#BUFFER_ID = 1,
    r#NAME = 2,
    r#WIDTH = 3,
    r#HEIGHT = 4,
    r#LAYER_COUNT = 5,
    r#PIXEL_FORMAT_REQUESTED = 6,
    r#PIXEL_FORMAT_FOURCC = 7,
    r#PIXEL_FORMAT_MODIFIER = 8,
    r#USAGE = 9,
    r#ALLOCATION_SIZE = 10,
    r#PROTECTED_CONTENT = 11,
    r#COMPRESSION = 12,
    r#INTERLACED = 13,
    r#CHROMA_SITING = 14,
    r#PLANE_LAYOUTS = 15,
    r#CROP = 16,
    r#DATASPACE = 17,
    r#BLEND_MODE = 18,
    r#SMPTE2086 = 19,
    r#CTA861_3 = 20,
    r#SMPTE2094_40 = 21,
    r#SMPTE2094_10 = 22,
    r#STRIDE = 23,
  }
}
pub(crate) mod mangled {
 pub use super::r#StandardMetadataType as _7_android_8_hardware_8_graphics_6_common_20_StandardMetadataType;
}
