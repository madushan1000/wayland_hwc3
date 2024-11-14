/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/mnishant/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl --lang=rust aidl/android/hardware/graphics/common/AlphaInterpretation.aidl aidl/android/hardware/graphics/common/BlendMode.aidl aidl/android/hardware/graphics/common/BufferUsage.aidl aidl/android/hardware/graphics/common/ChromaSiting.aidl aidl/android/hardware/graphics/common/ColorTransform.aidl aidl/android/hardware/graphics/common/Compression.aidl aidl/android/hardware/graphics/common/Cta861_3.aidl aidl/android/hardware/graphics/common/Dataspace.aidl aidl/android/hardware/graphics/common/DisplayDecorationSupport.aidl aidl/android/hardware/graphics/common/ExtendableType.aidl aidl/android/hardware/graphics/common/FRect.aidl aidl/android/hardware/graphics/common/HardwareBuffer.aidl aidl/android/hardware/graphics/common/HardwareBufferDescription.aidl aidl/android/hardware/graphics/common/Hdr.aidl aidl/android/hardware/graphics/common/HdrConversionCapability.aidl aidl/android/hardware/graphics/common/HdrConversionStrategy.aidl aidl/android/hardware/graphics/common/Interlaced.aidl aidl/android/hardware/graphics/common/PixelFormat.aidl aidl/android/hardware/graphics/common/PlaneLayout.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponent.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponentType.aidl aidl/android/hardware/graphics/common/Point.aidl aidl/android/hardware/graphics/common/Rect.aidl aidl/android/hardware/graphics/common/Smpte2086.aidl aidl/android/hardware/graphics/common/StandardMetadataType.aidl aidl/android/hardware/graphics/common/Transform.aidl aidl/android/hardware/graphics/common/XyColor.aidl -I aidl/ --stability=vintf --structured -o android_hardware_graphics_common/src/aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
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
