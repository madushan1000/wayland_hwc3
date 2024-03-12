set -e
cargo build
ssh 192.168.10.2 pkill -9 wayland_hwc3 || :
scp target/debug/wayland_hwc3 192.168.10.2:~/wayland_hwc3/
scp target/debug/build/binder_ndk_sys-98e2e69e014ef4eb/out/x86_64-unknown-linux-gnu/debug/libbinder_ndk.so 192.168.10.2:~/wayland_hwc3/
#ssh 192.168.10.2 LD_LIBRARY_PATH=~/wayland_hwc3 WAYLAND_DEBUG=1 strace -qq -X verbose -e trace=ioctl,read wayland_hwc3/wayland_hwc3
#ssh 192.168.10.2 LD_LIBRARY_PATH=~/wayland_hwc3 strace -qq wayland_hwc3/wayland_hwc3
ssh 192.168.10.2 LD_LIBRARY_PATH=~/wayland_hwc3 WAYLAND_DEBUG=1 wayland_hwc3/wayland_hwc3
#ssh 192.168.10.2 LD_LIBRARY_PATH=~/wayland_hwc3 wayland_hwc3/wayland_hwc3
