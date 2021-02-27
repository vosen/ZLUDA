LLVM-C interfaces has a decent coverage, but it does not expose everything,
hence this project to fill in the gaps.
Compilation order:
* CMake generate llvm-project
* Compile llvm-config and build subset of LLVM components
* Link llvm-sys rust wrapper with LLVM componets
* Compile C++ code in zluda_llvm
* Link zluda_llvm Rust + zluda_llvm C++ + llvm-sys
