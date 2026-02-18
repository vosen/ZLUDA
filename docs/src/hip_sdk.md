# Installing HIP SDK

On Windows, in addition to installing the GPU driver, you need to install the HIP SDK. Choose one of the two options below:

<table width="100%">
<tr >
<th> Official HIP SDK </th>
<th> Unofficial HIP SDK builds </th>
</tr>
<tr/>
<tr>
<td style="width: 50%;border:none;vertical-align: top">

✅ Automatic installation\
✅ Stable and supported by AMD\
❌ Older code\
❌ No Machine Learning support (PyTorch and TensorFlow will not work)

### Installation

1. Visit the [AMD HIP SDK for Windows](https://www.amd.com/en/developer/resources/rocm-hub/hip-sdk.html) website
2. Download the latest version for your OS
3. Run the installer and follow the on-screen instructions

</td>
<td style="width: 50%;border:none;vertical-align: top">

❌ Manual installation\
❌ Unstable and unsupported by AMD\
✅ Newer code\
✅ Machine Learning support (required for PyTorch, TensorFlow, etc.)

### Installation

1. Visit the [ROCm SDK nightly tarballs](https://therock-nightly-tarball.s3.amazonaws.com/index.html) website
2. Download a recent `therock-dist-windows-gfx<GPUARCH>...tar.gz` file:
   - If you don't know your `<GPUARCH>`, check the TechPowerUp GPU database. Example: for [AMD Radeon RX 9070](https://www.techpowerup.com/gpu-specs/radeon-rx-9070.c4250), the `<GPUARCH>` (Shader ISA) is 1201
   - Alternatively, download any `tar.gz` file, unpack it and run the included `hipInfo.exe` to determine your `<GPUARCH>` (gcnArchName)
3. Extract the `.tar.gz` file to a directory of your choice (you can use [7-Zip](https://www.7-zip.org/))
   - Note: You may need to extract twice (first the `.tar.gz`, then the `.tar` file)
4. Set the `HIP_PATH` environment variable to the extracted directory path ([instructions here](https://www.tenforums.com/tutorials/121855-edit-user-system-environment-variables-windows.html))
   - The directory must contain a `bin` subdirectory
   - The `bin` directory must contain various `.dll` files, including `rocblas.dll`

</td>
</tr>
</table>

## Verification

ZLUDA includes a small CUDA application that tests the loading and initialization of all performance libraries. Run it using:

```
zluda.exe -- cuda_check.exe
```

The output should look like this:

```
nvcuda    : OK (C:\hip_sdk\bin\amdhip64_7.dll)
nvml      : OK
cufft11   : OK
cudnn9    : OK (C:\hip_sdk\bin\MIOpen.dll)
cudnn8    : OK (C:\hip_sdk\bin\MIOpen.dll)
cublaslt13: OK (C:\hip_sdk\bin\libhipblaslt.dll)
cusparse12: OK
cufft12   : OK
cublas13  : OK (C:\hip_sdk\bin\rocblas.dll)
cublaslt12: OK (C:\hip_sdk\bin\libhipblaslt.dll)
cublas12  : OK (C:\hip_sdk\bin\rocblas.dll)
cusparse11: OK
```

The path in parentheses is the path to the underlying HIP SDK library.

**Caveats and Known Issues:**
- The path shown in parentheses indicates the underlying HIP SDK library, but is not guaranteed to be used. If an application loads the library from a different path before loading ZLUDA, ZLUDA will use the already-loaded library instead
- `cuda_check.exe` may sometimes hang and not close due to a bug in MIOpen
- When using the official HIP SDK, `cuda_check.exe` will fail to load `cudnn8` and `cudnn9` because the official SDK does not include MIOpen

