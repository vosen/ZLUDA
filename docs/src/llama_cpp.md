# llama.cpp

## Apple GPUs on Asahi Linux

The bundled `/scratch/llama-app-inspect/llama-app` (b11706) runs Qwen
inference and CUDA benchmarks on the M2 Max with the AGX driver. This is
experimental compatibility, with the settings and limits below.

### Run on this machine

`/scratch/run-llama-agx` selects the rebuilt driver and compiler and sets
`CUDA_MODULE_LOADING=LAZY`, `ZLUDA_AGX_LAZY_KERNELS=1`, and
`ZLUDA_AGX_GRAPH_BATCH=0`. It forwards its arguments to `llama-app`.

```bash
/scratch/run-llama-agx completion \
  -m /scratch/qwen2.5-0.5b-instruct-q5_k_m.gguf \
  -ngl 99 -dev CUDA0 -fa off -c 256 -b 8 -ub 8 \
  -n 32 --temp 0 --no-warmup \
  -p 'List the integers from 1 to 5, separated by commas.'

/scratch/run-llama-agx bench \
  -m /scratch/qwen2.5-0.5b-instruct-q5_k_m.gguf \
  -ngl 99 -dev CUDA0 -fa off -b 8 -ub 8 \
  -p 128 -n 32 -r 1 -t 4 --no-warmup
```

For another machine, build `zluda_agx` and the AGX-enabled `compiler` as
in [AGX](agx.md), then reproduce the launcher's environment:

```bash
export LD_LIBRARY_PATH="/path/to/driver${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
# The driver directory must contain libcuda.so.1 -> libnvcuda_agx.so.
export ZLUDA_AGX_ZOC=/absolute/path/to/zoc
export CUDA_MODULE_LOADING=LAZY
export ZLUDA_AGX_LAZY_KERNELS=1
export ZLUDA_AGX_GRAPH_BATCH=0
# Optional loader/compiler/cache diagnostics:
export ZLUDA_AGX_LOG=1
```

Use your `llama-app` and model paths with the same command arguments.
`-dev CUDA0 -ngl 99` explicitly requests GPU offload. Benchmark output
must name the CUDA backend and CUDA0; a CPU-only result does not verify it.

### Required settings and remaining limits

* Keep flash attention off and microbatches at 8 (`-fa off -b 8 -ub 8`).
  A completion with microbatches of 32 reaches `cublasCreate_v2`, which
  requests a module with no usable PTX and fails with error 209. The
  bundled application's cuBLAS path is not supported by this validation.
* CUDA library registration must be lazy. Registration now owns the chosen
  PTX until its first device lookup. Valid empty libraries are supported;
  libraries without usable PTX can register in LAZY mode, but attempting
  to obtain their device module/kernel still returns error 209. This follows
  the distinction between registration and device loading described in
  [CUDA library management](https://docs.nvidia.com/cuda/cuda-driver-api/cuda_driver_api/group__CUDA__LIBRARY.html).
* Graph batching is disabled by default. It produced repeated `G` tokens
  for the counting prompt; disabling specialization alone did not fix it.
  With batching disabled, both enabled and disabled specialization produce
  the correct response. `ZLUDA_AGX_GRAPH_BATCH=1` remains experimental.
* The separate `/scratch/llama-b11838/llama-bench` distribution's CUDA
  backend needs the absent `libcudart.so.13`. That is a separate build;
  it is not the shared-library requirement of the working bundled app.
* Results below cover this Qwen GGUF and these settings. Other models,
  larger microbatches and flash attention have not been validated.

### Validation, 2026-10-24

Model: `/scratch/qwen2.5-0.5b-instruct-q5_k_m.gguf` (630.17 M parameters,
492.32 MiB as reported by llama.cpp). Both CPU and GPU comparisons use the
same b11706 executable, greedy sampling, microbatches of 8, and flash
attention off. These prompts return exactly matching stdout and exit 0:

| Prompt | Response |
| --- | --- |
| List the integers from 1 to 5, separated by commas. | `1, 2, 3, 4, 5` |
| What is 2 + 3? Answer with just the number. | `5` |
| Name the first three days of the week. | `The first three days of the week are Monday, Tuesday, and Wednesday.` |
| Write a short sentence about the ocean. | `The ocean is a vast body of saltwater that covers much of the Earth's surface and is home to a diverse array of marine life.` |

The benchmark command above exits 0 and reports `backend=CUDA`,
`dev=CUDA0`, `ngl=99`, with 67.61 tokens/s for `pp128` and 32.38 tokens/s
for `tg32` in one local repetition without warmup. These are smoke-test
measurements, not a performance evaluation. The default graph setting
(without an environment override) also produces the CPU-matching counting
response and exits 0.

The driver's fixes also cover adjacent virtual-memory mappings, updates
of captured linear graphs, null frees during library initialization, and
library unload after thread-local context-stack destruction. Regression
tests exercise actual memory mappings, updated graph parameters, and a
lazily registered PTX kernel executing on the GPU after its original
registration buffer has been overwritten.

Validation passes all 33 `zluda_agx` library tests (serially, with
`ZLUDA_AGX_ZOC` set) and all 44 `agx_runtime` library tests.

Diagnostic artifacts in scratch include `qwen-agx-correctness/report.json`,
`qwen-agx-count-nobatch.txt`, and their stderr files. Compiler diagnostics
report compilation wall time and cache hits. Early flash-attention queries
compiled individual variants in approximately 5–46 seconds; the model
kernels used with flash attention off typically compiled in about 0.1
seconds in these runs. These are compilation times, not GPU kernel times.

## AMD GPUs

llama.cpp runs at native speed when compiled for CUDA architecture 86 and with cuBLAS enabled:
```
cmake -B build -DGGML_CUDA=ON -DCMAKE_CUDA_ARCHITECTURES="86" -DGGML_CUDA_FORCE_CUBLAS=true
```

Compiling for multiple CUDA architectures should be fine as long as one of the architectures is 80, 86 or 89.  
Compiling with cuBLAS disabled might lead to performance degradation.

## Windows
You need to install [HIP SDK](hip_sdk.html) to have access to rocBLAS
