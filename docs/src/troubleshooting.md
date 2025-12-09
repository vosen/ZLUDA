# Troubleshooting

## Introduction

zluda_trace is a [shim](https://en.wikipedia.org/wiki/Shim_(computing))
for the CUDA API which traces application's CUDA usage. If your application is
encountering issues with ZLUDA, you should use zluda_trace to find out where and
why ZLUDA fails.

## Quick Start

### Linux

Run your application like this:

#### AMD GPU:

```bash
ZLUDA_CUDA_LIB=<ZLUDA_DIRECTORY>/libcuda.so LD_LIBRARY_PATH=<ZLUDA_DIRECTORY>/trace/ \
ZLUDA_LOG_DIR=<LOG_DIRECTORY> <APPLICATION> <APPLICATION_ARGUMENTS>
```

#### NVIDIA GPU:

```bash
LD_LIBRARY_PATH=<ZLUDA_DIRECTORY>/trace/ \
ZLUDA_LOG_DIR=<LOG_DIRECTORY> <APPLICATION> <APPLICATION_ARGUMENTS>
```

If you're [filing a GitHub
issue](https://github.com/vosen/ZLUDA/issues/new/choose), please create
an archive with your logs from `<LOG_DIRECTORY>` and attach it to the issue:


```bash
tar -cvf logs.tar.gz -C <LOG_DIRECTORY> .
```

### Windows

Run your application like this:

#### AMD GPU:

```bash
zluda.exe --zluda-trace -- <APPLICATION> <APPLICATION_ARGUMENTS>
```

#### NVIDIA GPU:

```bash
zluda.exe --nvidia-trace -- <APPLICATION> <APPLICATION_ARGUMENTS>
```

This will create a new directory with executable name with the log output in `C:\Users\%USERNAME%\AppData\Local\Temp\zluda` (`%TEMP%\zluda`).

If you're [filing a GitHub
issue](https://github.com/vosen/ZLUDA/issues/new/choose), please create
a .zip file with your logs from `C:\Users\%USERNAME%\AppData\Local\Temp\zluda` to attach to the
issue. In Windows Explorer, right click on the directory  and select
"Send to"/"Compressed (zipped) folder". Exact steps may vary between
Windows versions.

### Explanation

#### `LD_LIBRARY_PATH=<ZLUDA_DIRECTORY>/trace/`

`<ZLUDA_DIRECTORY>` is the directory that contains the ZLUDA driver (`libcuda.so`
and various other libraries). It will be `target/release` if you built from
source, or `zluda` if you downloaded one of the release packages. `<ZLUDA_DIRECTORY>/trace`
contains tracing shims for `libcuda.so` (zluda_trace) and other CUDA libraries.

> [!NOTE]
> `LD_LIBRARY_PATH` is an environment variable used by `ld`, Linux's dynamic linker. It tells `ld` that when it's looking for a shared library – for example, `libcuda.so` – it should first look in a specific list of directories, before system paths. It's just like `PATH`, but for shared libraries instead of executables.

#### `ZLUDA_CUDA_LIB=<ZLUDA_DIRECTORY>/libcuda.so`

By default, zluda_trace will log all calls and then redirect them to an actual
CUDA driver (`libcuda.so`). In order to use ZLUDA, `ZLUDA_CUDA_LIB` must be set
to the `libcuda.so` provided by ZLUDA. If `ZLUDA_CUDA_LIB` is not set,
zluda_trace will use NVIDIA’s `libcuda.so`.

#### `ZLUDA_LOG_DIR=<LOG_DIRECTORY>`

By default, zluda_trace prints logs to stderr. In order to save them to a
file, as well as save other useful information, you must provide a
directory that they should be saved in – for example, `/tmp/zluda`.

## Understanding the zluda_trace output

Let's look at the zluda_trace output for a simple application. Here's a
CUDA program that adds two numbers on the GPU:

```cpp,linenos
#include <iostream>

__global__ void add(int a, int b, int *out) {
    *out = a + b;
}

int main() {
    int *result;
    cudaMallocManaged(&result, sizeof(int));
    add<<<1, 1>>>(1, 2, result);
    cudaDeviceSynchronize();
    std::cout << "result: " << *result << std::endl;
    cudaFree(result);
    return 0;
}
```

I've saved this file as `add.cu`. ZLUDA doesn't successfully run this
application yet, so I'll compile it and run it using zluda_trace and CUDA
in order to demonstrate all of zluda_trace's features.

```bash
nvcc add.cu -o add -arch sm_80
LD_LIBRARY_PATH=~/ZLUDA/target/release/trace/ ZLUDA_LOG_DIR=/tmp/zluda ./add
```

The last few lines should look something like:

```
[ZLUDA_TRACE] cuCtxSynchronize() -> CUDA_SUCCESS
result: 3
[ZLUDA_TRACE] {CONTEXT_LOCAL_STORAGE_INTERFACE_V0301}::context_local_storage_get(value: 0x562c764a73c0, cu_ctx: 0x0, key: 0x562c764ba130) -> CUDA_SUCCESS
[ZLUDA_TRACE] cuMemFree_v2(dptr: 0x7f3ca2000000) -> CUDA_SUCCESS
[ZLUDA_TRACE] {CONTEXT_LOCAL_STORAGE_INTERFACE_V0301}::context_local_storage_delete(context: 0x562c764ba760, key: 0x562c764ba130) -> CUDA_ERROR_DEINITIALIZED
[ZLUDA_TRACE] cuLibraryUnload(library: 0x562c773ffb10) -> CUDA_ERROR_DEINITIALIZED
[ZLUDA_TRACE] cuDevicePrimaryCtxRelease(dev: 0) -> CUDA_ERROR_DEINITIALIZED
```

Now, let's take a look at our log directory:

```bash
ls /tmp/zluda
add
```

zluda_trace creates a new directory for each run, based on the name of
the command. If the `add` directory already existed, it'd create an `add_1`
directory, and so on. Next, let's look at that newly-created directory:

```bash
ls /tmp/zluda/add/
log.txt  module_0001_01.elf  module_0001_02.ptx
```

Let's take a look at each of these files.

### log.txt

```bash
#no_wrap
cat /tmp/zluda/add/log.txt
# ...
# cuModuleGetFunction(hfunc: 0x55ee94d645d0, hmod: 0x55ee94d63c40, name: "_Z3addiiPi") -> CUDA_SUCCESS
# cuLaunchKernel(f: 0x55ee94d645d0, gridDimX: 1, gridDimY: 1, gridDimZ: 1, blockDimX: 1, blockDimY: 1, blockDimZ: 1, sharedMemBytes: 0, hStream: 0x0, kernelParams: 0x7fffe0fa193c, extra: NULL) -> CUDA_SUCCESS
# {CONTEXT_LOCAL_STORAGE_INTERFACE_V0301}::context_local_storage_get(value: 0x55ee93e083c0, cu_ctx: 0x0, key: 0x55ee93e1b130) -> CUDA_SUCCESS
# cuCtxSynchronize() -> CUDA_SUCCESS
# {CONTEXT_LOCAL_STORAGE_INTERFACE_V0301}::context_local_storage_get(value: 0x55ee93e083c0, cu_ctx: 0x0, key: 0x55ee93e1b130) -> CUDA_SUCCESS
# cuMemFree_v2(dptr: 0x7fbde6000000) -> CUDA_SUCCESS
# {CONTEXT_LOCAL_STORAGE_INTERFACE_V0301}::context_local_storage_delete(context: 0x55ee93e1b760, key: 0x55ee93e1b130) -> CUDA_ERROR_DEINITIALIZED
# cuLibraryUnload(library: 0x55ee94d60ae0) -> CUDA_ERROR_DEINITIALIZED
# cuDevicePrimaryCtxRelease(dev: 0) -> CUDA_ERROR_DEINITIALIZED
```

As you can see, this is the same log that was written to stderr. It
records each call made to a CUDA library, the arguments it was passed,
and the status code returned. Most of these will be calls that you can
find in the NVIDIA documentation – for example,
[`cuModuleGetFunction`](https://docs.nvidia.com/cuda/cuda-driver-api/group__CUDA__MODULE.html#group__CUDA__MODULE_1ga52be009b0d4045811b30c965e1cb2cf)
– but some of them aren't publicly documented.

<!-- I've used a zero-width space around :: below. -->
For example, look at the calls to
`{CONTEXT_LOCAL_STORAGE_INTERFACE_V0301}​::​context_local_storage_get`.
Calls with this format are to what we call NVIDIA's Dark API. We'll
write more documentation for this later, but for now all you need to
know are that these are from function pointer tables returned by
`cuGetExportTable`.

We're looking at a very simple example, so it doesn't use any
performance libraries. If you use zluda_trace for code calling one of
NVIDIA's performance libraries, zluda_trace will log both the call to
that library, and then all of the calls made by that library call. That
looks like:

```
cublasCreate_v2(handle: 0x55e502373120) -> CUBLAS_STATUS_SUCCESS
    cuGetProcAddress_v2(symbol: "", pfn: 0x0, cudaVersion: 0, flags: 0, symbolStatus: NULL) -> CUDA_ERROR_NOT_FOUND
```

The call to `cublasCreate_v2` is making a call to `cuGetProcAddress_v2`.

### module_0001_01.elf

This is precompiled SASS assembly for a single GPU architecture.

### module_0001_02.ptx

This is PTX assembly that is portable across many NVIDIA GPUs.

```bash
cat /tmp/zluda/add/module_0001_02.ptx
# //
# //
# //
# //
# //
# //
#
# .version 8.7
# .target sm_80
# .address_size 64
#
# //
#
# .visible .entry _Z3addiiPi(
# .param .u32 _Z3addiiPi_param_0,
# .param .u32 _Z3addiiPi_param_1,
# .param .u64 _Z3addiiPi_param_2
# )
# {
# .reg .b32 %r<4>;
# .reg .b64 %rd<3>;
#
#
# ld.param.u32 %r1, [_Z3addiiPi_param_0];
# ld.param.u32 %r2, [_Z3addiiPi_param_1];
# ld.param.u64 %rd1, [_Z3addiiPi_param_2];
# cvta.to.global.u64 %rd2, %rd1;
# add.s32 %r3, %r2, %r1;
# st.global.u32 [%rd2], %r3;
# ret;
#
# }
```

This is the `add` function from `add.cu`. `_Z3addiiPi` is the `add(int, int, int*)` after [C++ name mangling](https://en.wikipedia.org/wiki/Name_mangling).

### Compiler logs

There's one more kind of file zluda_trace might produce: a compiler error
log file. When zluda_trace encounters a PTX module, it tries to compile
it with ZLUDA's PTX compiler. Any errors produced will be saved into a
`module_NNNN_NN.log` file. For example, it might look like

```
Unrecognized statement "nanosleep.u32 %r101;"
```

We use this information to discover which PTX instructions are used by the
application and not supported by ZLUDA.
