set -x
HEADER_FILES="drm.h drm_mode.h virtgpu_drm.h"

mkdir drm

for file in ${HEADER_FILES}
do
    curl -s -o drm/${file} https://raw.githubusercontent.com/torvalds/linux/master/include/uapi/drm/${file}
done

curl -s https://chromium.googlesource.com/chromiumos/platform2/+/refs/heads/main/vm_tools/sommelier/virtualization/virtgpu_cross_domain_protocol.h?format=TEXT | base64 -d > drm/virtgpu_cross_domain_protocol.h 
curl -s https://chromium.googlesource.com/chromiumos/platform2/+/refs/heads/main/vm_tools/sommelier/virtualization/linux-headers/virtgpu_drm.h?format=TEXT | base64 -d > drm/virtgpu_drm.h

cat <<EOF > drm/bindings.h
#include "drm.h"
#include "virtgpu_drm.h"
#include "virtgpu_cross_domain_protocol.h"
EOF

BINDGEN_EXTRA_CLANG_ARGS="-D __user= " bindgen --with-derive-default --no-layout-tests drm/bindings.h -o src/virtgpu_wayland/card/bindings.rs
