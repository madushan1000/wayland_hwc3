#[allow(soft_unstable)]
#[allow(non_snake_case)]
pub mod aidl {
    pub mod android {
        pub mod hardware {
            pub mod input {
                pub mod IInputManager;
            }
        }
    }
}

pub mod mangled {
    pub use crate::aidl::android::hardware::input::IInputManager::mangled::*;
}
