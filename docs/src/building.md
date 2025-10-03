# Building

## Dependencies

* Git
* CMake
* Python 3
* Rust compiler (recent version)
* C++ compiler
* (Linux only) HIP ([instructions here](https://rocm.docs.amd.com/projects/HIP/en/latest/install/install.html))
* (Optional, but recommended) [Ninja build system](https://ninja-build.org/)

## Build steps

* Git clone the repo (make sure to use `--recursive` option to fetch submodules):\
`git clone --recursive https://github.com/vosen/ZLUDA.git`
* Enter freshly cloned `ZLUDA` directory and build with cargo (this takes a while):
  * `cargo xtask --release` for Release build
  * `cargo xtask` for Debug build
