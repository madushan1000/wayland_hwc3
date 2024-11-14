/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/mnishant/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl --lang=rust aidl/android/hardware/graphics/common/AlphaInterpretation.aidl aidl/android/hardware/graphics/common/BlendMode.aidl aidl/android/hardware/graphics/common/BufferUsage.aidl aidl/android/hardware/graphics/common/ChromaSiting.aidl aidl/android/hardware/graphics/common/ColorTransform.aidl aidl/android/hardware/graphics/common/Compression.aidl aidl/android/hardware/graphics/common/Cta861_3.aidl aidl/android/hardware/graphics/common/Dataspace.aidl aidl/android/hardware/graphics/common/DisplayDecorationSupport.aidl aidl/android/hardware/graphics/common/ExtendableType.aidl aidl/android/hardware/graphics/common/FRect.aidl aidl/android/hardware/graphics/common/HardwareBuffer.aidl aidl/android/hardware/graphics/common/HardwareBufferDescription.aidl aidl/android/hardware/graphics/common/Hdr.aidl aidl/android/hardware/graphics/common/HdrConversionCapability.aidl aidl/android/hardware/graphics/common/HdrConversionStrategy.aidl aidl/android/hardware/graphics/common/Interlaced.aidl aidl/android/hardware/graphics/common/PixelFormat.aidl aidl/android/hardware/graphics/common/PlaneLayout.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponent.aidl aidl/android/hardware/graphics/common/PlaneLayoutComponentType.aidl aidl/android/hardware/graphics/common/Point.aidl aidl/android/hardware/graphics/common/Rect.aidl aidl/android/hardware/graphics/common/Smpte2086.aidl aidl/android/hardware/graphics/common/StandardMetadataType.aidl aidl/android/hardware/graphics/common/Transform.aidl aidl/android/hardware/graphics/common/XyColor.aidl -I aidl/ --stability=vintf --structured -o android_hardware_graphics_common/src/aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  r#Dataspace : [i32; 61] {
    r#UNKNOWN = 0,
    r#ARBITRARY = 1,
    r#STANDARD_SHIFT = 16,
    r#STANDARD_MASK = 4128768,
    r#STANDARD_UNSPECIFIED = 0,
    r#STANDARD_BT709 = 65536,
    r#STANDARD_BT601_625 = 131072,
    r#STANDARD_BT601_625_UNADJUSTED = 196608,
    r#STANDARD_BT601_525 = 262144,
    r#STANDARD_BT601_525_UNADJUSTED = 327680,
    r#STANDARD_BT2020 = 393216,
    r#STANDARD_BT2020_CONSTANT_LUMINANCE = 458752,
    r#STANDARD_BT470M = 524288,
    r#STANDARD_FILM = 589824,
    r#STANDARD_DCI_P3 = 655360,
    r#STANDARD_ADOBE_RGB = 720896,
    r#TRANSFER_SHIFT = 22,
    r#TRANSFER_MASK = 130023424,
    r#TRANSFER_UNSPECIFIED = 0,
    r#TRANSFER_LINEAR = 4194304,
    r#TRANSFER_SRGB = 8388608,
    r#TRANSFER_SMPTE_170M = 12582912,
    r#TRANSFER_GAMMA2_2 = 16777216,
    r#TRANSFER_GAMMA2_6 = 20971520,
    r#TRANSFER_GAMMA2_8 = 25165824,
    r#TRANSFER_ST2084 = 29360128,
    r#TRANSFER_HLG = 33554432,
    r#RANGE_SHIFT = 27,
    r#RANGE_MASK = 939524096,
    r#RANGE_UNSPECIFIED = 0,
    r#RANGE_FULL = 134217728,
    r#RANGE_LIMITED = 268435456,
    r#RANGE_EXTENDED = 402653184,
    r#SRGB_LINEAR = 138477568,
    r#SCRGB_LINEAR = 406913024,
    r#SRGB = 142671872,
    r#SCRGB = 411107328,
    r#JFIF = 146931712,
    r#BT601_625 = 281149440,
    r#BT601_525 = 281280512,
    r#BT709 = 281083904,
    r#DCI_P3_LINEAR = 139067392,
    r#DCI_P3 = 155844608,
    r#DISPLAY_P3_LINEAR = 139067392,
    r#DISPLAY_P3 = 143261696,
    r#ADOBE_RGB = 151715840,
    r#BT2020_LINEAR = 138805248,
    r#BT2020 = 147193856,
    r#BT2020_PQ = 163971072,
    r#DEPTH = 4096,
    r#SENSOR = 4097,
    r#BT2020_ITU = 281411584,
    r#BT2020_ITU_PQ = 298188800,
    r#BT2020_ITU_HLG = 302383104,
    r#BT2020_HLG = 168165376,
    r#DISPLAY_BT2020 = 142999552,
    r#DYNAMIC_DEPTH = 4098,
    r#JPEG_APP_SEGMENTS = 4099,
    r#HEIF = 4100,
    r#JPEG_R = 4101,
    r#BT709_FULL_RANGE = 146866176,
  }
}
pub(crate) mod mangled {
 pub use super::r#Dataspace as _7_android_8_hardware_8_graphics_6_common_9_Dataspace;
}
