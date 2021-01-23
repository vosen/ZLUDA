# Dependencies

Development builds of ZLUDA requires following dependencies:

* CMake
* Python 3

Additionally the repository has to be cloned with Git submodules initalized. If you cloned the repo without initalizing submodules, do this:
```
git submodule update --init --recursive
```

# Tests

Tests should be executed with `--workspace` option to test non-default targets:
```
cargo test --workspace
```

# Debugging

## Debuggging CUDA applications

When running an application with ZLUDA quite often you will run into subtle bugs or incompatibilities in the generated GPU code. The best way to debug an application's GPU CUDA code is to use ZLUDA dumper.

Library `zluda_dump` can be injected into a CUDA application and produce a trace which, for every launched GPU function contains:
* PTX source
* Launch arguments (block size, grid size, shared memory)
* Memory dump of global meory used by the function. Both after and before

Example use with GeekBench:
```
set ZLUDA_DUMP_KERNEL=knn_match
set ZLUDA_DUMP_DIR=C:\temp\zluda_dump
"<ZLUDA_PATH>\zluda_with.exe" "<ZLUDA_PATH>\zluda_dump.dll" -- "geekbench_x86_64.exe" --compute CUDA
```

The example above, for every execution of GPU function `knn_match`, will save its details into the directory `C:\temp\zluda_dump`

This dump can be replayed with `replay.py` script from `zluda_dump` source directory. Use it like this:
```
python replay.py "C:\temp\zluda_dump\geekbench_x86_64.exe"
```
You must copy (or symlink) ZLUDA nvcuda.dll into pyCUDA directory, so it will run using ZLUDA. This will print similar information to stdout:
```
Intel(R) Graphics [0x3e92] [github.com/vosen/ZLUDA]
C:\temp\zluda_dump\geekbench_x86_64.exe\4140_scale_pyramid
C:\temp\zluda_dump\geekbench_x86_64.exe\4345_convolve_1d_vertical_grayscale
    Skipping, launch block size (512) bigger than maximum block size (256)
C:\temp\zluda_dump\geekbench_x86_64.exe\4480_scale_pyramid
6: 
Arrays are not equal

Mismatched elements: 1200 / 19989588 (0.006%)
Max absolute difference: 255
Max relative difference: 255.
 x: array([  7,   6,   8, ..., 193, 195, 193], dtype=uint8)
 y: array([  7,   6,   8, ..., 193, 195, 193], dtype=uint8)
```
From this output one can observe that in kernel launch 4480, 6th argument to function `scale_pyramid` differs between what was executed on an NVIDIA GPU and Intel GPU using CUDA.  
__Important__: It's impossible to infer what was the type (and semantics) of argument passed to a GPU function. At our level it's a buffer of bytes and by default `replay.py` simply checks if two buffers are byte-equal. That means you will have a ton of false negatives when running  `replay.py`. You should override them for your particular case in `replay.py` - it already contains some overrides for GeekBench kernels