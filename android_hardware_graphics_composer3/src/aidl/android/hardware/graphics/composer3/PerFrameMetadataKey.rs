/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: /home/mnishant/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl --lang=rust aidl/android/hardware/graphics/composer3/Buffer.aidl aidl/android/hardware/graphics/composer3/Capability.aidl aidl/android/hardware/graphics/composer3/ChangedCompositionLayer.aidl aidl/android/hardware/graphics/composer3/ChangedCompositionTypes.aidl aidl/android/hardware/graphics/composer3/ClientTarget.aidl aidl/android/hardware/graphics/composer3/ClientTargetProperty.aidl aidl/android/hardware/graphics/composer3/ClientTargetPropertyWithBrightness.aidl aidl/android/hardware/graphics/composer3/ClockMonotonicTimestamp.aidl aidl/android/hardware/graphics/composer3/Color.aidl aidl/android/hardware/graphics/composer3/ColorMode.aidl aidl/android/hardware/graphics/composer3/CommandError.aidl aidl/android/hardware/graphics/composer3/CommandResultPayload.aidl aidl/android/hardware/graphics/composer3/Composition.aidl aidl/android/hardware/graphics/composer3/ContentType.aidl aidl/android/hardware/graphics/composer3/DimmingStage.aidl aidl/android/hardware/graphics/composer3/DisplayAttribute.aidl aidl/android/hardware/graphics/composer3/DisplayBrightness.aidl aidl/android/hardware/graphics/composer3/DisplayCapability.aidl aidl/android/hardware/graphics/composer3/DisplayCommand.aidl aidl/android/hardware/graphics/composer3/DisplayConnectionType.aidl aidl/android/hardware/graphics/composer3/DisplayContentSample.aidl aidl/android/hardware/graphics/composer3/DisplayContentSamplingAttributes.aidl aidl/android/hardware/graphics/composer3/DisplayIdentification.aidl aidl/android/hardware/graphics/composer3/DisplayRequest.aidl aidl/android/hardware/graphics/composer3/FormatColorComponent.aidl aidl/android/hardware/graphics/composer3/HdrCapabilities.aidl aidl/android/hardware/graphics/composer3/IComposer.aidl aidl/android/hardware/graphics/composer3/IComposerCallback.aidl aidl/android/hardware/graphics/composer3/IComposerClient.aidl aidl/android/hardware/graphics/composer3/LayerBrightness.aidl aidl/android/hardware/graphics/composer3/LayerCommand.aidl aidl/android/hardware/graphics/composer3/OverlayProperties.aidl aidl/android/hardware/graphics/composer3/ParcelableBlendMode.aidl aidl/android/hardware/graphics/composer3/ParcelableComposition.aidl aidl/android/hardware/graphics/composer3/ParcelableDataspace.aidl aidl/android/hardware/graphics/composer3/ParcelableTransform.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadata.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadataBlob.aidl aidl/android/hardware/graphics/composer3/PerFrameMetadataKey.aidl aidl/android/hardware/graphics/composer3/PlaneAlpha.aidl aidl/android/hardware/graphics/composer3/PowerMode.aidl aidl/android/hardware/graphics/composer3/PresentFence.aidl aidl/android/hardware/graphics/composer3/PresentOrValidate.aidl aidl/android/hardware/graphics/composer3/ReadbackBufferAttributes.aidl aidl/android/hardware/graphics/composer3/RefreshRateChangedDebugData.aidl aidl/android/hardware/graphics/composer3/ReleaseFences.aidl aidl/android/hardware/graphics/composer3/RenderIntent.aidl aidl/android/hardware/graphics/composer3/VirtualDisplay.aidl aidl/android/hardware/graphics/composer3/VsyncPeriodChangeConstraints.aidl aidl/android/hardware/graphics/composer3/VsyncPeriodChangeTimeline.aidl aidl/android/hardware/graphics/composer3/ZOrder.aidl -I aidl/ --stability=vintf --structured -o android_hardware_graphics_composer3/src/aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
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
