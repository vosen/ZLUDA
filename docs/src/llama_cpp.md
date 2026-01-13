# llama.cpp

llama.cpp runs at native speed when compiled for CUDA architecture 86 and with cuBLAS enabled:
```
cmake -B build -DGGML_CUDA=ON -DCMAKE_CUDA_ARCHITECTURES="86" -DGGML_CUDA_FORCE_CUBLAS=true
```

Compiling for multiple CUDA architectures should be fine as long as one of the architectures is 80, 86 or 89.  
Compiling with cuBLAS disabled might lead to performance degradation.

## Windows
You should provide rocBLAS by either installing HIP SDK (https://www.amd.com/en/developer/resources/rocm-hub/hip-sdk.html) or copying rocblas.dll into the directory with zluda.exe