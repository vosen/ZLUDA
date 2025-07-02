#!/bin/bash
set -ex
ROCM_VERSION=$0

DEBIAN_FRONTEND=noninteractive apt install -y --no-install-recommends gpg
# Source: https://rocm.docs.amd.com/projects/install-on-linux/en/latest/install/install-methods/package-manager/package-manager-ubuntu.html
mkdir --parents --mode=0755 /etc/apt/keyrings
wget https://repo.radeon.com/rocm/rocm.gpg.key -O - | \
    gpg --dearmor | tee /etc/apt/keyrings/rocm.gpg > /dev/null
echo deb [arch=amd64 signed-by=/etc/apt/keyrings/rocm.gpg] https://repo.radeon.com/rocm/apt/$ROCM_VERSION noble main | tee /etc/apt/sources.list.d/rocm.list
echo -e 'Package: *\nPin: release o=repo.radeon.com\nPin-Priority: 600' \
    | tee /etc/apt/preferences.d/rocm-pin-600
DEBIAN_FRONTEND=noninteractive apt update -y
DEBIAN_FRONTEND=noninteractive apt install -y --no-install-recommends rocm-smi-lib rocm-llvm-dev hip-runtime-amd hip-dev
echo 'export PATH="$PATH:/opt/rocm/bin"' |  tee /etc/profile.d/rocm.sh
echo "/opt/rocm/lib" | tee /etc/ld.so.conf.d/rocm.conf
ldconfig