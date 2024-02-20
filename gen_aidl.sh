#!/bin/env zsh
AOSP_PATH=../aosp

$AOSP_PATH/out/host/linux-x86/bin/aidl \
    --lang=rust \
    $AOSP_PATH/hardware/interfaces/graphics/composer/aidl/**/**.aidl \
    -I $AOSP_PATH/hardware/interfaces/graphics/common/aidl  \
    -I $AOSP_PATH/hardware/interfaces/common/aidl \
    -I $AOSP_PATH/hardware/interfaces/graphics/composer/aidl \
    --stability=vintf \
    --structured \
    -o android_hardware_graphics_composer/src


$AOSP_PATH/out/host/linux-x86/bin/aidl \
    --lang=rust \
    $AOSP_PATH/hardware/interfaces/graphics/common/aidl/**/**.aidl \
    -I $AOSP_PATH/hardware/interfaces/common/aidl \
    -I $AOSP_PATH/hardware/interfaces/graphics/common/aidl \
    --stability=vintf \
    --structured \
    -o android_hardware_graphics_common/src

$AOSP_PATH/out/host/linux-x86/bin/aidl \
    $AOSP_PATH/hardware/interfaces/common/aidl/**/**.aidl \
    --lang=rust \
    -I $AOSP_PATH/hardware/interfaces/common/aidl \
    --stability=vintf \
    --structured \
    -o android_hardware_common/src
