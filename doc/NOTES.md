I'm convinced nobody actually uses parser generators in Rust:
* pomelo can't generate lexer (understandable, as it is a port of lemon and lemon can't do this either)
* pest can't do parse actions, you have to convert your parse tree to ast manually
* lalrpop can't do comments
    * and the day I wrote the line above it can
    * reports parsing errors as byte offsets
    * if you want to skip parsing one of the alternatives, functional design gets quite awkward
* antlr4rust is untried and requires java to build
* no library supports island grammars

What to emit?
* SPIR-V
    * Better library support, easier to emit
    * Can by optimized by IGC
    * Can't do some things (not sure what exactly yet)
    * But we can work around things with inline VISA
* VISA
    * Quicker compilation

A64 vs BTS
* How to force A64: -cl-intel-greater-than-4GB-buffer-required
* PTX made a baffling desing choice: global pointers are represented as untyped 64bit integers
* Consequently, there's no 100% certain way to know which argument is a surface and which is a scalar
    * It seems that NVidia guys realized what a horrible idea that was and emit `cvta.to.global` as a marker for global pointers?
        * But it's only emitted in a recent release build, can't rely on it
        * Maybe debug builds emit debug metadata to detect surfaces?
        * Might add this as an optimization later
    * `cuLaunchKernel` docs say this: "The number of kernel parameters and their offsets and sizes do not need to be specified as that information is retrieved directly from the kernel's image", note the wording: _offsets_ and _sizes_ and not _types_
    * Wait, you can mark an argument as a pointer with `.ptr`: https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#kernel-parameter-attribute-ptr, but it's useless with NV compiler  not emitting it
* Potential solution: compile only during the dispatch, when type of arguments is known?
    * Can't do, the set of arguments passed to cuLaunchKernel is untyped
* Solution: treat all arguments as untyped integers and say goodbye to BTS access
