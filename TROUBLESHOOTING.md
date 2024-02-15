# Troubleshooting

While we do our  best effort to support as many applications as possible, realistically, you will run into cases where things do not work. Here's a list of troubleshooting (or debugging) steps you can take, roughly in the order of increasing difficulty and power.

## Check for the presence of ROCm/HIP libraries

At the bare minimum ZLUDA needs to load the HIP runtime and the ROCm compiler support library. You can confirm that the HIP runtime is loaded by running your applications with environment variable:
```
AMD_LOG_LEVEL=3
```
you should see additional HIP debug output in the console. It will look like this:
```
:3:rocdevice.cpp            :442 : 15186552182 us: [pid:27431 tid:0x7f21a7842000] Initializing HSA stack.
:3:comgrctx.cpp             :33  : 15186578605 us: [pid:27431 tid:0x7f21a7842000] Loading COMGR library.
:3:rocdevice.cpp            :208 : 15186578634 us: [pid:27431 tid:0x7f21a7842000] Numa selects cpu agent[0]=0x5617fc899660(fine=0x5617fc921230,coarse=0x5617fc0dafe0) for gpu agent=0x5617fc8bdbd0 CPU<->GPU XGMI=0
:3:rocdevice.cpp            :1680: 15186578901 us: [pid:27431 tid:0x7f21a7842000] Gfx Major/Minor/Stepping: 10/3/0
:3:rocdevice.cpp            :1682: 15186578904 us: [pid:27431 tid:0x7f21a7842000] HMM support: 1, XNACK: 0, Direct host access: 0
:3:rocdevice.cpp            :1684: 15186578905 us: [pid:27431 tid:0x7f21a7842000] Max SDMA Read Mask: 0xfc8b6cb8, Max SDMA Write Mask: 0x5617
:3:hip_context.cpp          :48  : 15186579248 us: [pid:27431 tid:0x7f21a7842000] Direct Dispatch: 1
:3:hip_context.cpp          :153 : 15186581653 us: [pid:27431 tid:0x7f21a7842000]  hipInit ( 0 ) 
:3:hip_context.cpp          :159 : 15186581657 us: [pid:27431 tid:0x7f21a7842000] hipInit: Returned hipSuccess : 
:3:hip_device_runtime.cpp   :546 : 15186581662 us: [pid:27431 tid:0x7f21a7842000]  hipGetDeviceCount ( 0x7ffd227fc848 ) 
:3:hip_device_runtime.cpp   :548 : 15186581664 us: [pid:27431 tid:0x7f21a7842000] hipGetDeviceCount: Returned hipSuccess : 
:3:hip_device.cpp           :381 : 15186581669 us: [pid:27431 tid:0x7f21a7842000]  hipGetDeviceProperties ( 0x7ffd227fab78, 0 ) 
:3:hip_device.cpp           :383 : 15186581671 us: [pid:27431 tid:0x7f21a7842000] hipGetDeviceProperties: Returned hipSuccess : 
:3:hip_device_runtime.cpp   :141 : 15186581678 us: [pid:27431 tid:0x7f21a7842000]  hipDeviceGetAttribute ( 0x7ffd227fb5ac, 87, 0 ) 
:3:hip_device_runtime.cpp   :351 : 15186581679 us: [pid:27431 tid:0x7f21a7842000] hipDeviceGetAttribute: Returned hipSuccess : 
:3:hip_device_runtime.cpp   :546 : 15186582123 us: [pid:27431 tid:0x7f21a7842000]  hipGetDeviceCount ( 0x5617fca66870 ) 
:3:hip_device_runtime.cpp   :548 : 15186582128 us: [pid:27431 tid:0x7f21a7842000] hipGetDeviceCount: Returned hipSuccess : 
:3:hip_device.cpp           :169 : 15186582131 us: [pid:27431 tid:0x7f21a7842000]  hipDeviceGet ( 0x7ffd227fce68, 0 ) 
:3:hip_device.cpp           :171 : 15186582132 us: [pid:27431 tid:0x7f21a7842000] hipDeviceGet: Returned hipSuccess : 

```
If you are on Windows and are trying to run a GUI application that does not use the console then you can try to edit the subsystem of the application's exe from Windows GUI to Windows console using [CFF Explorer](https://ntcore.com/?page_id=388) or a similar tool.

If there is no HIP logging output, it most likely means that ZLUDA could not find the runtime libraries.

#### Linux

On Linux, ZLUDA depends on the presence of `libamdhip64.so` and `libamd_comgr.so.2` being present in the system library search paths. If ZLUDA can't find either of them, it it's usually a case of not adding `/opt/rocm/lib` to the system linker paths as instructed [here](https://rocm.docs.amd.com/projects/install-on-linux/en/latest/how-to/native-install/post-install.html). As a last resort, you can use the  `LD_DEBUG=libs` environment variable to debug the library loading process.

#### Windows

On Windows, ZLUDA depends on the rpesence of `amdhip64.dll` and `amd_comgr.dll` being present in the system library search paths (usually `C:\Windows\System32`). If they are not present consider reinstalling your Radeon GPU driver.

## (Linux only) Trace CUDA calls

If an application using ZLUDA crashes or fails to run, then an easy way to check which function is failing is to run it under ltrace:

```
ltrace -f -x "cu*@*-cuda*@*" -L <APPLICATION> <APPLICATION_ARGUMENTS>
```
This will produce extra output to the console that looks like this:
```
[pid 34267] cuMemcpyHtoDAsync_v2@libcuda.so.1(0x7f82a6a00000, 0x7f82a6000000, 0x8000, 0x55ccafdde260)                                             = 0
[pid 34267] cuMemcpyHtoDAsync_v2@libcuda.so.1(0x7f82a6a08000, 0x7f82ac200800, 8, 0x55ccafdde260)                                                  = 0
[pid 34267] cuMemcpyHtoDAsync_v2@libcuda.so.1(0x7f82a6a08080, 0x7f82ac201000, 256, 0x55ccafdde260)                                                = 0
[pid 34267] cuCtxGetDevice@libcuda.so.1(0x7ffce2e3d0bc, 0x7f84a22326e0, 0, 0x7f82a56ff010)                                                        = 0
[pid 34267] cuCtxPushCurrent@libcuda.so.1(0x55ccafb4bf88, 0x7ffce2e3cdbf, 10, 0x55ccafb1a160)                                                     = 0
[pid 34267] cuCtxPopCurrent@libcuda.so.1(0x7ffce2e3cd38, 0, 3, 0)                                                                                 = 0
[pid 34267] cuModuleGetFunction@libcuda.so.1(0x55ccafd0e558, 0x55ccafdee210, 0x55ccace18090, 0x55ccafb4bf88)                                      = 500
```
Look for the call with a non-zero return value. In our case `cuModuleGetFunction` with a return value of 500. You can check what that error code means in NVIDIA's documentation. Search [here](https://docs.nvidia.com/cuda/cuda-driver-api/group__CUDA__TYPES.html) for `enum CUresult`. Below you will find a list of error codes. In our case it is `CUDA_ERROR_NOT_FOUND`. The error code alone is rarely useful for the ZLUDA developers. If you are interested in a more precise CUDA trace, you can try the ZLUDA dumper as described in the section below.

## ZLUDA dumper

In addition to the "normal" implementation of CUDA API, ZLUDA ships with debugging implementation (sometimes called ZLUDA dumper). This implementation does the following:
- Intercept any call to CUDA APIs.
- Log imortant information: function name, arguments to console output and log file.
- On GPU code load, saves the GPU code input (PTX assembly or compiled binary code).
- Passes the CUDA API call for the execution to a real implementation (either ZLUDA or original CUDA).

First set by setting two environment variables:
* `ZLUDA_DUMP_DIR`: directory path where ZLUDA dumper will create a subdirectory with all the relevant information for you run. I usually set it to `/tmp/zluda` on Linux and `C:\temp\zluda` on Windows. The ZLUDA dumper will create the directory if it does not exist.
* `ZLUDA_CUDA_LIB`: path to the real CUDA library implementation that actually executes CUDA code. If this is not set, the ZLUDA dumper will try to load NVIDIA CUDA by default.

Once you have set the environment variables, you can start ZLUDA dumper:

### Windows

The ZLUDA loader (`zluda.exe`) can load `nvcuda.dll` from any arbitrary path with the `--nvcuda` argument. You should also use `--nvml` to select the correct `nvml.dll`.

If you are dumping ZLUDA use:
```
<ZLUDA_DIRECTORY>\zluda.exe --nvcuda <ZLUDA_DIRECTORY>\zluda_dump.dll -- <APPLICATION> <APPLICATION_ARGUMENTS>
```

If you are dumping original CUDA use:
```
<ZLUDA_DIRECTORY>\zluda.exe --nvcuda <ZLUDA_DIRECTORY>\zluda_dump.dll --nvml "C:\Windows\System32\nvml.dll" -- <APPLICATION> <APPLICATION_ARGUMENTS>
```

### Linux

Known bug: when dumping from original CUDA you should remove (or rename) all the files in `<ZLUDA_DIRECTORY>/dump` except `libcuda.so` and `libcuda.so.1`.

Use it like this:
```
LD_LIBRARY_PATH="<ZLUDA_DIRECTORY>/dump:$LD_LIBRARY_PATH" <APPLICATION> <APPLICATION_ARGUMENTS>
```

### Result

If all went well you should see lines like this in the console output and in the log file specified by `ZLUDA_DUMP_DIR`:

```
cuGetProcAddress(symbol: "cuGetProcAddress", pfn: 0x7f8b9eb19b70, cudaVersion: 11030, flags: 0) -> CUDA_SUCCESS
    Created dump directory /tmp/zluda/main
cuGetProcAddress(symbol: "cuInit", pfn: 0x7f8b9e9fb480, cudaVersion: 2000, flags: 0) -> CUDA_SUCCESS
cuGetProcAddress(symbol: "cuGetProcAddress", pfn: 0x7f8b9eb19b70, cudaVersion: 11030, flags: 0) -> CUDA_SUCCESS
cuGetProcAddress(symbol: "cuDeviceGet", pfn: 0x7f8b9e9fc4f0, cudaVersion: 2000, flags: 0) -> CUDA_SUCCESS
cuGetProcAddress(symbol: "cuDeviceGetCount", pfn: 0x7f8b9e9fccb0, cudaVersion: 2000, flags: 0) -> CUDA_SUCCESS
cuGetProcAddress(symbol: "cuDeviceGetName", pfn: 0x7f8b9e9fd4f0, cudaVersion: 2000, flags: 0) -> CUDA_SUCCESS
```

## Debugging

### Building ZLUDA with debug information

You can build ZLUDA with debugging information by running:
```
cargo xtask
```

Aside from than the usual effects on the code it has two consequences:
* Most CUDA API functions will abort and print backtrace on failure. In release mode, ZLUDA tries to proceed gracefully and returns an error so that the application can handle the situation.
* GPU code is compiled with debug information (`-g`). Since ZLUDA does not emit CUDA debug information this only adds backtraces to the GPU code.
* The set of projects being built is slightly different.


### (Linux only) ROCgdb

To debug GPU code you can use ROCgdb. It's a fork of gdb that comes with ROCm. There are multiple articles out there explaining how to use gdb, so we won't go into the detail here. Some ZLUDA-specific notes:

* ZLUDA does not emit GPU debug information. If you are lucky you might get a partial stack trace.
* Consider using [gef](https://github.com/hugsy/gef). Among other things, it adds an `xinfo` command that can help you understand what a pointer is pointing to (device memory? function pointer? host memory?).
* https://github.com/vosen/amdgpu_debug has ROCgdb scripts that may be helpful.

### ZLUDA offline compiler (zoc)

If you are working with the PTX compiler, you should use zoc. Zoc is built by default when you build in debug mode. You use it like this:
```
<BUILD_DIRECTORY>/zoc <PATH_TO_PTX_FILE>
```
It will generate the final GPU binary and most of the intermediate files (LLVM IR, custom ELF section). It has several options: compilation mode, GPU ISA, etc.. For details, run it with `--help` argument.

Zoc does not use the compiler cache and will always do a full build.

For the best effect, run it with the ROCm compiler library debugging environment variables. see the details [here](https://github.com/ROCm/llvm-project/blob/amd-staging/amd/comgr/README.md#environment-variables).