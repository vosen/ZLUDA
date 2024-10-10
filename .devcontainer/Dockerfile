FROM nvidia/cuda:12.4.1-base-ubuntu22.04

RUN DEBIAN_FRONTEND=noninteractive apt-get update -y && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    wget \
    build-essential \
    cmake \
    ninja-build \
    python3 \
    ripgrep \
    git \
    ltrace

# Feel free to change to a newer version if you have a newer verison on your host
ARG CUDA_VERSION=12-4
# Docker <-> host  driver version compatiblity is newer host <-> older docker
# We don't care about a specific driver version, so pick oldest 5XX
ARG CUDA_DRIVER=515
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    nvidia-utils-${CUDA_DRIVER} \
    cuda-cudart-${CUDA_VERSION}

ARG ROCM_VERSION=6.2.2
RUN mkdir --parents --mode=0755 /etc/apt/keyrings && \
    wget https://repo.radeon.com/rocm/rocm.gpg.key -O - | \
    gpg --dearmor | tee /etc/apt/keyrings/rocm.gpg > /dev/null && \
    echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/rocm.gpg] https://repo.radeon.com/rocm/apt/${ROCM_VERSION} jammy main" > /etc/apt/sources.list.d/rocm.list && \
    echo 'Package: *\nPin: release o=repo.radeon.com\nPin-Priority: 600' > /etc/apt/preferences.d/rocm-pin-600 && \
    DEBIAN_FRONTEND=noninteractive apt update -y && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    rocminfo \
    rocm-gdb \
    rocm-smi-lib \
    hip-runtime-amd &&  \
    echo '/opt/rocm/lib' > /etc/ld.so.conf.d/rocm.conf && \
    ldconfig

ENV PATH=$PATH:/opt/rocm-6.2.2/bin

