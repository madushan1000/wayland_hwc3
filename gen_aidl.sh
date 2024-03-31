#!/bin/env zsh
AOSP_PATH=../aosp
AIDL_PATH=aidl

$AOSP_PATH/out/host/linux-x86/bin/aidl \
    --lang=rust \
    $AIDL_PATH/android/hardware/graphics/composer3/*.aidl \
    -I $AIDL_PATH/ \
    --stability=vintf \
    --structured \
    -o android_hardware_graphics_composer3/src/aidl


$AOSP_PATH/out/host/linux-x86/bin/aidl \
    --lang=rust \
    $AIDL_PATH/android/hardware/graphics/common/*.aidl \
    -I $AIDL_PATH/ \
    --stability=vintf \
    --structured \
    -o android_hardware_graphics_common/src/aidl

$AOSP_PATH/out/host/linux-x86/bin/aidl \
    $AIDL_PATH/android/hardware/common/*.aidl \
    --lang=rust \
    -I $AIDL_PATH/ \
    --stability=vintf \
    --structured \
    -o android_hardware_common/src/aidl
