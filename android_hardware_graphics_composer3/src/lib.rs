#[allow(soft_unstable)]
#[allow(non_camel_case_types, non_snake_case, unused)]
pub mod aidl {
    pub mod android {
        pub mod hardware {
            pub mod graphics {
                pub mod composer3 {
                    pub mod Buffer;
                    pub mod Capability;
                    pub mod ChangedCompositionLayer;
                    pub mod ChangedCompositionTypes;
                    pub mod ClientTarget;
                    pub mod ClientTargetProperty;
                    pub mod ClientTargetPropertyWithBrightness;
                    pub mod ClockMonotonicTimestamp;
                    pub mod Color;
                    pub mod ColorMode;
                    pub mod CommandError;
                    pub mod CommandResultPayload;
                    pub mod Composition;
                    pub mod ContentType;
                    pub mod DimmingStage;
                    pub mod DisplayAttribute;
                    pub mod DisplayBrightness;
                    pub mod DisplayCapability;
                    pub mod DisplayCommand;
                    pub mod DisplayConnectionType;
                    pub mod DisplayContentSample;
                    pub mod DisplayContentSamplingAttributes;
                    pub mod DisplayIdentification;
                    pub mod DisplayRequest;
                    pub mod FormatColorComponent;
                    pub mod HdrCapabilities;
                    pub mod IComposer;
                    pub mod IComposerCallback;
                    pub mod IComposerClient;
                    pub mod LayerBrightness;
                    pub mod LayerCommand;
                    pub mod OverlayProperties;
                    pub mod ParcelableBlendMode;
                    pub mod ParcelableComposition;
                    pub mod ParcelableDataspace;
                    pub mod ParcelableTransform;
                    pub mod PerFrameMetadata;
                    pub mod PerFrameMetadataBlob;
                    pub mod PerFrameMetadataKey;
                    pub mod PlaneAlpha;
                    pub mod PowerMode;
                    pub mod PresentFence;
                    pub mod PresentOrValidate;
                    pub mod ReadbackBufferAttributes;
                    pub mod RefreshRateChangedDebugData;
                    pub mod ReleaseFences;
                    pub mod RenderIntent;
                    pub mod VirtualDisplay;
                    pub mod VsyncPeriodChangeConstraints;
                    pub mod VsyncPeriodChangeTimeline;
                    pub mod ZOrder;
                }
            }
        }
    }
}
#[allow(non_camel_case_types, non_snake_case, unused)]
pub(crate) mod mangled {
    pub(crate) use android_hardware_common::mangled::*;
    pub(crate) use android_hardware_graphics_common::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::Buffer::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::Capability::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::ChangedCompositionLayer::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::ChangedCompositionTypes::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::ClientTarget::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::ClientTargetProperty::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::ClientTargetPropertyWithBrightness::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::ClockMonotonicTimestamp::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::Color::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::ColorMode::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::CommandError::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::CommandResultPayload::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::Composition::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::ContentType::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::DimmingStage::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::DisplayAttribute::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::DisplayBrightness::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::DisplayCapability::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::DisplayCommand::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::DisplayConnectionType::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::DisplayContentSample::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::DisplayContentSamplingAttributes::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::DisplayIdentification::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::DisplayRequest::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::FormatColorComponent::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::HdrCapabilities::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::IComposer::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::IComposerCallback::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::IComposerClient::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::LayerBrightness::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::LayerCommand::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::OverlayProperties::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::ParcelableBlendMode::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::ParcelableComposition::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::ParcelableDataspace::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::ParcelableTransform::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::PerFrameMetadata::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::PerFrameMetadataBlob::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::PerFrameMetadataKey::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::PlaneAlpha::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::PowerMode::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::PresentFence::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::PresentOrValidate::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::ReadbackBufferAttributes::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::RefreshRateChangedDebugData::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::ReleaseFences::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::RenderIntent::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::VirtualDisplay::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::VsyncPeriodChangeConstraints::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::VsyncPeriodChangeTimeline::mangled::*;
    pub use crate::aidl::android::hardware::graphics::composer3::ZOrder::mangled::*;
}
