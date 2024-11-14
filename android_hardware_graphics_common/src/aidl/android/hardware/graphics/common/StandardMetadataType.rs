/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/mnishant/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl --lang=rust aidl/android/hardware/graphics/common/AlphaInterpretation.aidl aidl/android/hardware/graphics/common/BlendMode.aidl aidl/android/hardware/graphics/common/BufferUsage.aidl aidl/android/hardware/graphics/common/ChromaSiting.aidl aidl/android/hardware/graphics/common/ColorTransform.aidl aidl/android/hardware/graphics/common/Compression.aidl aidl/android/hardware/graphics/common/Cta861_3.aidl aidl/android/hardware/graphics/common/Dataspace.aidl aidl/android/hardware/graphics/common/DisplayDecorationSupport.aidl aidl/android/hardware/graphics/common/ExtendableType.aidl aidl/android/hardware/graphics/common/FRect.aidl aidl/android/hardware/graphics/common/HardwareBuffer.aidl aidl/android/hardware/graphics/common/HardwareBufferDescription.aidl aidl/android/hardware/graphics/common/Hdr.aidl aidl/android/hardware/graphics/common/HdrConversionCapability.aidl aidl/android/hardware/graphics/common/HdrConversionStrategy.aidl aidl/android/hardware/graphics/common/Interlaced.aidl aidl/android/hardware/graphics/common/PixelFormat.aidl aidl/android/hardware/graphics/common/PlaneLayout.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponent.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponentType.aidl aidl/android/hardware/graphics/common/Point.aidl aidl/android/hardware/graphics/common/Rect.aidl aidl/android/hardware/graphics/common/Smpte2086.aidl aidl/android/hardware/graphics/common/StandardMetadataType.aidl aidl/android/hardware/graphics/common/Transform.aidl aidl/android/hardware/graphics/common/XyColor.aidl -I aidl/ --stability=vintf --structured -o android_hardware_graphics_common/src/aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
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
