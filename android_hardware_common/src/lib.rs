#[allow(soft_unstable)]
#[allow(non_snake_case)]
pub mod aidl {
    pub mod android {
        pub mod hardware {
            pub mod common {
                pub mod Ashmem;
                pub mod MappableFile;
                pub mod NativeHandle;
            }
        }
    }
}

pub mod mangled {
    pub use crate::aidl::android::hardware::common::Ashmem::mangled::*;
    pub use crate::aidl::android::hardware::common::MappableFile::mangled::*;
    pub use crate::aidl::android::hardware::common::NativeHandle::mangled::*;
}
