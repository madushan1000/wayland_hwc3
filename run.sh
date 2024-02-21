REMOTE_PATH=/data/local/tmp
adb root
adb -e push target/x86_64-linux-android/debug/wayland_hwc3 $REMOTE_PATH/
adb shell RUST_BACKTRACE=full $REMOTE_PATH/wayland_hwc3 
