set -x

mkdir include
mkdir include/android
mkdir include/ndk
mkdir include/linux

curl -s https://android.googlesource.com/platform/system/core/+/refs/heads/main/libsync/include/android/sync.h?format=TEXT | base64 -d > include/android/sync.h
curl -s https://android.googlesource.com/platform/system/core/+/refs/heads/main/libsync/include/ndk/sync.h?format=TEXT | base64 -d > include/ndk/sync.h
curl -s https://android.googlesource.com/platform/system/core/+/refs/heads/main/libsync/sw_sync.h?format=TEXT | base64 -d > include/sw_sync.h
curl -s -o include/linux/sync_file.h https://raw.githubusercontent.com/torvalds/linux/master/include/uapi/linux/sync_file.h

cat <<EOF > include/bindings.h
#define __INTRODUCED_IN(x) 
#include "android/sync.h"
#include "sw_sync.h"
EOF

BINDGEN_EXTRA_CLANG_ARGS="-D __BEGIN_DECLS= -D __END_DECLS= " bindgen --with-derive-default --no-layout-tests include/bindings.h -o src/bindings.rs
