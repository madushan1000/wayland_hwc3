set -e
REMOTE_PATH=/data/local/tmp
export ANDROID_NDK_HOME=/home/mnishant/Dev/my/android/android-ndk-r26c
export RUSTFLAGS="-L$PWD/libs --C link-arg=-lsync"
cargo ndk -t x86_64 -p 34 --no-strip --bindgen build
adb root
adb -e push target/x86_64-linux-android/debug/wayland_hwc3 $REMOTE_PATH/
#adb shell $REMOTE_PATH/wayland_hwc3 
#adb shell WAYLAND_DEBUG=1 $REMOTE_PATH/wayland_hwc3 
#adb shell strace -f $REMOTE_PATH/wayland_hwc3 
adb shell $@ $REMOTE_PATH/wayland_hwc3 
