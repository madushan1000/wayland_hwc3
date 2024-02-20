#[allow(soft_unstable)]
#[allow(non_snake_case)]
pub mod android {
    pub mod hardware {
        pub mod graphics {
            pub mod common {
                pub mod AlphaInterpretation;
                pub mod BlendMode;
                pub mod BufferUsage;
                pub mod ChromaSiting;
                pub mod ColorTransform;
                pub mod Compression;
                pub mod Cta861_3;
                pub mod Dataspace;
                pub mod DisplayDecorationSupport;
                pub mod ExtendableType;
                pub mod FRect;
                pub mod HardwareBuffer;
                pub mod HardwareBufferDescription;
                pub mod Hdr;
                pub mod HdrConversionCapability;
                pub mod HdrConversionStrategy;
                pub mod Interlaced;
                pub mod PixelFormat;
                pub mod PlaneLayout;
                pub mod PlaneLayoutComponent;
                pub mod PlaneLayoutComponentType;
                pub mod Point;
                pub mod Rect;
                pub mod Smpte2086;
                pub mod StandardMetadataType;
                pub mod Transform;
                pub mod XyColor;
            }
        }
    }
}

pub mod mangled {
    pub(crate) use android_hardware_common::mangled::*;
    pub use crate::android::hardware::graphics::common::AlphaInterpretation::mangled::*;
    pub use crate::android::hardware::graphics::common::BlendMode::mangled::*;
    pub use crate::android::hardware::graphics::common::BufferUsage::mangled::*;
    pub use crate::android::hardware::graphics::common::ChromaSiting::mangled::*;
    pub use crate::android::hardware::graphics::common::ColorTransform::mangled::*;
    pub use crate::android::hardware::graphics::common::Compression::mangled::*;
    pub use crate::android::hardware::graphics::common::Cta861_3::mangled::*;
    pub use crate::android::hardware::graphics::common::Dataspace::mangled::*;
    pub use crate::android::hardware::graphics::common::DisplayDecorationSupport::mangled::*;
    pub use crate::android::hardware::graphics::common::ExtendableType::mangled::*;
    pub use crate::android::hardware::graphics::common::FRect::mangled::*;
    pub use crate::android::hardware::graphics::common::HardwareBuffer::mangled::*;
    pub use crate::android::hardware::graphics::common::HardwareBufferDescription::mangled::*;
    pub use crate::android::hardware::graphics::common::Hdr::mangled::*;
    pub use crate::android::hardware::graphics::common::HdrConversionCapability::mangled::*;
    pub use crate::android::hardware::graphics::common::HdrConversionStrategy::mangled::*;
    pub use crate::android::hardware::graphics::common::Interlaced::mangled::*;
    pub use crate::android::hardware::graphics::common::PixelFormat::mangled::*;
    pub use crate::android::hardware::graphics::common::PlaneLayout::mangled::*;
    pub use crate::android::hardware::graphics::common::PlaneLayoutComponent::mangled::*;
    pub use crate::android::hardware::graphics::common::PlaneLayoutComponentType::mangled::*;
    pub use crate::android::hardware::graphics::common::Point::mangled::*;
    pub use crate::android::hardware::graphics::common::Rect::mangled::*;
    pub use crate::android::hardware::graphics::common::Smpte2086::mangled::*;
    pub use crate::android::hardware::graphics::common::StandardMetadataType::mangled::*;
    pub use crate::android::hardware::graphics::common::Transform::mangled::*;
    pub use crate::android::hardware::graphics::common::XyColor::mangled::*;
}
