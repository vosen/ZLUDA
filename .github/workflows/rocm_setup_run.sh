#!/bin/bash
set -ex
ROCM_VERSION=$1

DEBIAN_FRONTEND=noninteractive apt install -y --no-install-recommends gpg zstd unzip "linux-headers-$(uname -r)" "linux-modules-extra-$(uname -r)"
# Source: https://rocm.docs.amd.com/projects/install-on-linux/en/latest/install/install-methods/package-manager/package-manager-ubuntu.html
mkdir --parents --mode=0755 /etc/apt/keyrings
wget https://repo.radeon.com/rocm/rocm.gpg.key -O - | \
    gpg --dearmor | tee /etc/apt/keyrings/rocm.gpg > /dev/null
echo deb [arch=amd64 signed-by=/etc/apt/keyrings/rocm.gpg] https://repo.radeon.com/rocm/apt/$ROCM_VERSION noble main | tee /etc/apt/sources.list.d/rocm.list
echo deb [arch=amd64 signed-by=/etc/apt/keyrings/rocm.gpg] https://repo.radeon.com/amdgpu/$ROCM_VERSION/ubuntu noble main | tee /etc/apt/sources.list.d/amdgpu.list
echo -e 'Package: *\nPin: release o=repo.radeon.com\nPin-Priority: 600' \
    | tee /etc/apt/preferences.d/rocm-pin-600
DEBIAN_FRONTEND=noninteractive apt update -y
DEBIAN_FRONTEND=noninteractive apt install -y --no-install-recommends amdgpu-dkms hip-runtime-amd
echo 'export PATH="$PATH:/opt/rocm/bin"' |  tee /etc/profile.d/rocm.sh
echo "/opt/rocm/lib" | tee /etc/ld.so.conf.d/rocm.conf
ldconfig

#Grant access to GPUs to all users via udev rules
cat <<'EOF' > /etc/udev/rules.d/70-amdgpu.rules
KERNEL=="kfd", MODE="0666"
SUBSYSTEM=="drm", KERNEL=="renderD*", MODE="0666"
EOF
udevadm control --reload-rules && udevadm trigger
modprobe amdgpu