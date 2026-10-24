# llama.cpp

## Apple GPUs on Asahi Linux

The AGX backend is experimental. GPU inference and benchmarking with the
prebuilt applications below have **not** succeeded yet. The AMD instructions
in the next section do not establish AGX compatibility.

Build `zluda_agx` and the AGX-enabled `compiler` as described in [AGX](agx.md).
Create a driver directory with `libcuda.so.1` pointing to the built
`libnvcuda_agx.so`, then set both paths explicitly:

```bash
export LD_LIBRARY_PATH="/path/to/driver${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
export ZLUDA_AGX_ZOC=/absolute/path/to/zoc
export ZLUDA_AGX_LOG=1
```

For the bundled application, use:

```bash
/path/to/llama-app help all
/path/to/llama-app bench --list-devices
/path/to/llama-app completion --list-devices
# Only after an AGX CUDA device is listed:
/path/to/llama-app bench -m /path/to/model.gguf -ngl 99 -p 128 -n 32 -r 1
/path/to/llama-app completion -m /path/to/model.gguf -ngl 99 -n 32 -p 'Once upon a time'
```

For a standalone distribution, the equivalent commands are `llama-bench`
and `llama-completion`. A successful command exit or a CPU-only benchmark
does not verify CUDA offload: check that a CUDA device is listed and that
the model reports layers offloaded to that device.

### Local validation, 2026-10-24

On the M2 Max under Asahi Linux:

* `/scratch/llama-app-inspect/llama-app` (b11706, per the workspace setup
  notes) starts and lists its commands with the AGX driver. Its runtime
  initialization previously failed with "integrity checks failed" because
  the driver reported CUDA 12.4 while its runtime compatibility implementation
  used `cuda_types::cuda::CUDA_VERSION` (13.0). The driver now uses that
  constant for its reported driver and context API versions.
* With `ZLUDA_AGX_ZOC` set, `bench --list-devices` still fails in
  `cuLibraryLoadData` with `CUDA_ERROR_NO_BINARY_FOR_GPU` (209). No usable
  PTX was selected from that image. Native CUDA GPU images cannot execute
  on AGX. This is a remaining blocker, not successful GPU initialization.
* `/scratch/llama-b11838/llama-bench` starts with CPU backends, but its
  `libggml-cuda.so` requires `libcudart.so.13`, which is absent locally.
  ZLUDA provides the driver and custom cuBLAS, not a CUDA runtime library.
  The existing `libcudart_linkcheck_only.so` is not a working runtime and
  must not be used to claim inference support.
* The rebuilt driver's `agx_cuda_smoke` passes on the GPU: 50,000 elements,
  zero incorrect results, with graph, unified memory, and 2D-copy checks
  also passing. This verifies basic PTX compilation and execution, not
  llama kernels.

No GGUF model was found in scratch. Model execution remains untested.
Further work needs a distribution with usable PTX for every required GPU
module (or an open-source build providing it), plus a local GGUF model.
Do not download or inspect proprietary NVIDIA components to resolve these
dependencies; follow the workspace's licensing restrictions.

## AMD GPUs

llama.cpp runs at native speed when compiled for CUDA architecture 86 and with cuBLAS enabled:
```
cmake -B build -DGGML_CUDA=ON -DCMAKE_CUDA_ARCHITECTURES="86" -DGGML_CUDA_FORCE_CUBLAS=true
```

Compiling for multiple CUDA architectures should be fine as long as one of the architectures is 80, 86 or 89.  
Compiling with cuBLAS disabled might lead to performance degradation.

## Windows
You need to install [HIP SDK](hip_sdk.html) to have access to rocBLAS
