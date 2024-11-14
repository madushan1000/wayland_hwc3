/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/mnishant/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl --lang=rust aidl/android/hardware/graphics/common/AlphaInterpretation.aidl aidl/android/hardware/graphics/common/BlendMode.aidl aidl/android/hardware/graphics/common/BufferUsage.aidl aidl/android/hardware/graphics/common/ChromaSiting.aidl aidl/android/hardware/graphics/common/ColorTransform.aidl aidl/android/hardware/graphics/common/Compression.aidl aidl/android/hardware/graphics/common/Cta861_3.aidl aidl/android/hardware/graphics/common/Dataspace.aidl aidl/android/hardware/graphics/common/DisplayDecorationSupport.aidl aidl/android/hardware/graphics/common/ExtendableType.aidl aidl/android/hardware/graphics/common/FRect.aidl aidl/android/hardware/graphics/common/HardwareBuffer.aidl aidl/android/hardware/graphics/common/HardwareBufferDescription.aidl aidl/android/hardware/graphics/common/Hdr.aidl aidl/android/hardware/graphics/common/HdrConversionCapability.aidl aidl/android/hardware/graphics/common/HdrConversionStrategy.aidl aidl/android/hardware/graphics/common/Interlaced.aidl aidl/android/hardware/graphics/common/PixelFormat.aidl aidl/android/hardware/graphics/common/PlaneLayout.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponent.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponentType.aidl aidl/android/hardware/graphics/common/Point.aidl aidl/android/hardware/graphics/common/Rect.aidl aidl/android/hardware/graphics/common/Smpte2086.aidl aidl/android/hardware/graphics/common/StandardMetadataType.aidl aidl/android/hardware/graphics/common/Transform.aidl aidl/android/hardware/graphics/common/XyColor.aidl -I aidl/ --stability=vintf --structured -o android_hardware_graphics_common/src/aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#PixelFormat : [i32; 33] {
    r#UNSPECIFIED = 0,
    r#RGBA_8888 = 1,
    r#RGBX_8888 = 2,
    r#RGB_888 = 3,
    r#RGB_565 = 4,
    r#BGRA_8888 = 5,
    r#YCBCR_422_SP = 16,
    r#YCRCB_420_SP = 17,
    r#YCBCR_422_I = 20,
    r#RGBA_FP16 = 22,
    r#RAW16 = 32,
    r#BLOB = 33,
    r#IMPLEMENTATION_DEFINED = 34,
    r#YCBCR_420_888 = 35,
    r#RAW_OPAQUE = 36,
    r#RAW10 = 37,
    r#RAW12 = 38,
    r#RGBA_1010102 = 43,
    r#Y8 = 538982489,
    r#Y16 = 540422489,
    r#YV12 = 842094169,
    r#DEPTH_16 = 48,
    r#DEPTH_24 = 49,
    r#DEPTH_24_STENCIL_8 = 50,
    r#DEPTH_32F = 51,
    r#DEPTH_32F_STENCIL_8 = 52,
    r#STENCIL_8 = 53,
    r#YCBCR_P010 = 54,
    r#HSV_888 = 55,
    r#R_8 = 56,
    r#R_16_UINT = 57,
    r#RG_1616_UINT = 58,
    r#RGBA_10101010 = 59,
  }
}
pub(crate) mod mangled {
 pub use super::r#PixelFormat as _7_android_8_hardware_8_graphics_6_common_11_PixelFormat;
}
