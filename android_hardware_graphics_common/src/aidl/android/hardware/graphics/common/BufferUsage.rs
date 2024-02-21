#![forbid(unsafe_code)]
#![rustfmt::skip]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#BufferUsage : [i64; 27] {
    r#CPU_READ_MASK = 15,
    r#CPU_READ_NEVER = 0,
    r#CPU_READ_RARELY = 2,
    r#CPU_READ_OFTEN = 3,
    r#CPU_WRITE_MASK = 240,
    r#CPU_WRITE_NEVER = 0,
    r#CPU_WRITE_RARELY = 32,
    r#CPU_WRITE_OFTEN = 48,
    r#GPU_TEXTURE = 256,
    r#GPU_RENDER_TARGET = 512,
    r#COMPOSER_OVERLAY = 2048,
    r#COMPOSER_CLIENT_TARGET = 4096,
    r#PROTECTED = 16384,
    r#COMPOSER_CURSOR = 32768,
    r#VIDEO_ENCODER = 65536,
    r#CAMERA_OUTPUT = 131072,
    r#CAMERA_INPUT = 262144,
    r#RENDERSCRIPT = 1048576,
    r#VIDEO_DECODER = 4194304,
    r#SENSOR_DIRECT_DATA = 8388608,
    r#GPU_DATA_BUFFER = 16777216,
    r#GPU_CUBE_MAP = 33554432,
    r#GPU_MIPMAP_COMPLETE = 67108864,
    r#HW_IMAGE_ENCODER = 134217728,
    r#FRONT_BUFFER = 4294967296,
    r#VENDOR_MASK = -268435456,
    r#VENDOR_MASK_HI = -281474976710656,
  }
}
pub(crate) mod mangled {
 pub use super::r#BufferUsage as _7_android_8_hardware_8_graphics_6_common_11_BufferUsage;
}
