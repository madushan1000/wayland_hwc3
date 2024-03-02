set -e
cargo build
LD_LIBRARY_PATH=target/debug/build/binder_ndk_sys-98e2e69e014ef4eb/out/x86_64-unknown-linux-gnu/debug/ target/debug/wayland_hwc3
