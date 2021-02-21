# ZLUDA

ZLUDA is a drop-in replacement for CUDA on Intel GPU. ZLUDA allows to run unmodified CUDA applications using Intel GPUs with near-native performance (more below). It works with current integrated Intel UHD GPUs and will work with future Intel Xe GPUs

## Performance

ZLUDA performance has been measured with GeekBench 5.2.3 on Intel UHD 630.\
One measurement has been done using OpenCL and another measurement has been done using CUDA with Intel GPU masquerading as a (relatively slow) NVIDIA GPU with the help of ZLUDA. Both measurements use the same GPU.

Performance below is normalized to OpenCL performance. 110% means that ZLUDA-implemented CUDA is 10% faster on Intel UHD 630.

![Performance graph](GeekBench_5_2_3.svg)

[ZLUDA - detailed results on Geekbench.com](https://browser.geekbench.com/v5/compute/2305009)

[OpenCL - detailed results on Geekbench.com](https://browser.geekbench.com/v5/compute/2304997)

Overall, ZLUDA is slower in GeekBench by roughly 2%.

### Explanation of the results
 * Why is ZLUDA faster in some benchmarks?\
   This has not been precisely pinpointed to one thing or another but it's likely a combination of things:
   * ZLUDA uses [Level 0](https://spec.oneapi.com/level-zero/latest/index.html), which in general is a more low level, high performance API than OpenCL
   * Tying to the previous point, currently ZLUDA does not support asynchronous execution. This gives us an unfair advantage in a benchmark like GeekBench. GeekBench exclusively uses CUDA synchronous APIs
   * There is a set of GPU instructions which are available on both NVIDIA hardware and Intel hardware, but are not exposed through OpenCL. We are comparing NVIDIA GPU optimized code with the more general OpenCL code. It's a lucky coincidence (and a credit to the underlying Intel Graphics Compiler) that this code also works well on an Intel GPU
 * Why is OpenCL faster in Canny and Horizon Detection?\
   Authors of CUDA benchmarks used CUDA functions `atomicInc` and `atomicDec` which have direct hardware support on NVIDIA cards, but no hardware support on Intel cards. They have to be emulated in software, which limits performance
 * Why is ZLUDA slower in the remaining benchmarks?\
   The reason is unknown. Most likely, in some tests we compile from suboptimal NVIDIA GPU code and in other tests ZLUDA itself is emitting suboptimal Intel GPU code. For example, SFFT used to be even slower before PR [#22](https://github.com/vosen/ZLUDA/pull/22)
   

## Details

 * Is ZLUDA a drop-in replacement for CUDA?\
   Yes, but certain applications use CUDA in ways which make it incompatible with  ZLUDA
 * What is the status of the project?\
   This project is a Proof of Concept. About the only thing that works currently is  Geekbench. It's amazingly buggy and incomplete. You  should not rely on it for anything serious
 * Is it an Intel project? Is it an NVIDIA project?\
   No, it's a private project
 * What is the performance?\
   Performance can be close to the performance of similarly written OpenCL code (see  GeekBench results in the previous section).  NVIDIA GPUs and Intel GPUs have  different architecture and feature set. Consequently, certain NVIDIA features have  to be emulated in ZLUDA with performance penalty. Additionally, performance of  ZLUDA will be always lower than the performance of code specifically optimized for Intel GPUs
 * How it's different from AMD HIP or Intel DPC++ Compatibility toolkit?\
   Both are porting toolkits which require programmer's effort to port applications  to the API in question. With ZLUDA existing applications "just work" on an Intel  GPU (if you are lucky and ZLUDA supports the particular subset of CUDA)
 * Which Intel GPU are supported?\
   Intel Gen9 and newer (Skylake and newer) which are supported by Intel Level 0
 * Does ZLUDA support AMD GPUs?\
   Certainly not currently, but it might be technically possible


## Usage
**Warning**: this is a very incomplete Proof of Concept. It's probably not going to work with your application. ZLUDA currently works only with applications which use CUDA Driver API. Linux builds also work with applications which use statically-linked CUDA Runtime API

### Windows
You should have the most recent Intel GPU drivers installed.\
Run your application like this:
```
<ZLUDA_DIRECTORY>\zluda_with.exe -- <APPLICATION> <APPLICATIONS_ARGUMENTS>
```

### Linux
You should install most recent run-time driver packages as outlined here: https://dgpu-docs.intel.com/installation-guides/index.html.  
Run your application like this:
```
LD_LIBRARY_PATH=<ZLUDA_DIRECTORY> <APPLICATION> <APPLICATIONS_ARGUMENTS>
```

## Building
You should have a relatively recent version of Rust installed, then you just do:

```
cargo build --release
```
in the main directory of the project.  
### Linux
You should install most recent run-time an developer driver packages as outlined here: https://dgpu-docs.intel.com/installation-guides/index.html. Additionally, you should have `ocl-icd-opencl-dev` (or equivalent) installed.  
If you are building on Linux you must also symlink (or rename) the ZLUDA output binaries after ZLUDA build finishes:
```
ln -s libnvcuda.so target/release/libcuda.so
ln -s libcuda.so target/release/libcuda.so.1
```

## Contributing

If you want to develop ZLUDA itself, read [CONTRIBUTING.md](CONTRIBUTING.md), it contains instructions how to set up dependencies and run tests


## License

This software is dual-licensed under either the Apache 2.0 license or the MIT license. See [LICENSE-APACHE](LICENSE-APACHE) or [LICENSE-MIT](LICENSE-MIT) for details
