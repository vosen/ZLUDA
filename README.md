[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?style=for-the-badge&logo=discord&logoColor=white)](https://discord.gg/sg6BNzXuc7)

# ZLUDA

ZLUDA is a drop-in replacement for CUDA on non-NVIDIA GPU. ZLUDA allows to run unmodified CUDA applications using non-NVIDIA GPUs with near-native performance.

ZLUDA is work in progress. Follow development here and say hi on [Discord](https://discord.gg/sg6BNzXuc7). For more details see the announcement: https://vosen.github.io/ZLUDA/blog/zludas-third-life/


## Usage
**Warning**: ZLUDA is under heavy development (see news [here](https://vosen.github.io/ZLUDA/blog/zludas-third-life/)). Instructions below might not work.

### Windows
You should have the most recent ROCm  installed.\
Run your application like this:
```
<ZLUDA_DIRECTORY>\zluda_with.exe -- <APPLICATION> <APPLICATIONS_ARGUMENTS>
```

### Linux

Run your application like this:
```
LD_LIBRARY_PATH=<ZLUDA_DIRECTORY> <APPLICATION> <APPLICATIONS_ARGUMENTS>
```

### MacOS

Not supported

## Building
**Warning**: ZLUDA is under heavy development (see news [here](https://vosen.github.io/ZLUDA/blog/zludas-third-life/)). Instructions below might not work.

_Note_: This repo has submodules. Make sure to recurse submodules when cloning this repo, e.g.: `git clone --recursive --depth=1 https://github.com/vosen/ZLUDA.git`

 You should have a relatively recent version of Rust installed, then you just do:

```
cargo build --release
```
in the main directory of the project.  
### Linux

If you are building on Linux you must also symlink (or rename) the ZLUDA output binaries after ZLUDA build finishes:
```
ln -s libnvcuda.so target/release/libcuda.so
ln -s libnvcuda.so target/release/libcuda.so.1
ln -s libnvml.so target/release/libnvidia-ml.so
```

## Contributing

If you want to develop ZLUDA itself, read [CONTRIBUTING.md](CONTRIBUTING.md), it contains instructions how to set up dependencies and run tests


## License

This software is dual-licensed under either the Apache 2.0 license or the MIT license. See [LICENSE-APACHE](LICENSE-APACHE) or [LICENSE-MIT](LICENSE-MIT) for details
