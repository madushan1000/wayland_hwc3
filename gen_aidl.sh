#!/bin/env zsh
AIDL_BIN=~/Dev/my/android/build-tools-main/android-VanillaIceCream/aidl
AIDL_PATH=aidl

$AIDL_BIN \
    --lang=rust \
    $AIDL_PATH/android/hardware/graphics/composer3/*.aidl \
    -I $AIDL_PATH/ \
    --stability=vintf \
    --structured \
    -o android_hardware_graphics_composer3/src/aidl


$AIDL_BIN \
    --lang=rust \
    $AIDL_PATH/android/hardware/graphics/common/*.aidl \
    -I $AIDL_PATH/ \
    --stability=vintf \
    --structured \
    -o android_hardware_graphics_common/src/aidl

$AIDL_BIN \
    $AIDL_PATH/android/hardware/common/*.aidl \
    --lang=rust \
    -I $AIDL_PATH/ \
    --stability=vintf \
    --structured \
    -o android_hardware_common/src/aidl

$AIDL_BIN \
    --lang=rust \
    $AIDL_PATH/android/hardware/input/*.aidl \
    -I $AIDL_PATH/ \
    --structured \
    -o android_hardware_input/src/aidl

