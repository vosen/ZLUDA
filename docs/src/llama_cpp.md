# llama.cpp

llama.cpp works at native speeds if compiled for CUDA architecture 86 and with cuBLAS enabled, like so:
```
cmake -B build -DGGML_CUDA=ON -DCMAKE_CUDA_ARCHITECTURES="86" -DGGML_CUDA_FORCE_CUBLAS=true
```

Compiling for multiple CUDA architectures should be fine as long as one of the architectures is 80, 86 or 89.  
Compiling with cuBLAS disabled might lead to performance degradation.