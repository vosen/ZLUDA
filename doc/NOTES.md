Parser generators in Rust:
--------------------------
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
-------------
* SPIR-V
    * Better library support, easier to emit
    * Can by optimized by IGC
    * Can't do some things (not sure what exactly yet)
        * But we can work around with inline VISA
* VISA
    * Quicker compilation

A64 vs BTS
----------
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

Implicit conversions
--------------------
* PTX support for implicit conversions is completely degenerate, docs say:  
_For convenience, ld, st, and cvt instructions permit source and destination data operands to be wider than the instruction-type size, so that narrow values may be loaded, stored, and converted using regular-width registers. For example, 8-bit or 16-bit values may be held directly in 32-bit or 64-bit registers when being loaded, stored, or converted to other types and sizes_  
Which is sensible, but completely untrue. In reality ptxas compiles silly code like this:
    ```
    param.f32       param_1
    ...
    .reg.s32        %r1
    ld.param.b16 	%r1, [param_1];
    ```
* Surprise, surprise, there's two kind of implicit conversions at play in the example above:
    * "Relaxed type-checking rules": this is the conversion of b16 operation type to s32 dst register
    * Undocumented type coercion when dereferencing param_1. The PTX behaviour is to coerce **every** type. It's something to the effect of `[param_1] = *(b16*)param_1`

PTX grammar
-----------
* PTX grammar rules are atrocious, keywords can be freely reused as ids without escaping
* Modifiers can be applied to instructions in any arbitrary order. We don't support it and hope we will never have to


Rust debugging
--------------
* Nothing works 100% well on vscode/Windows:
    * MSVC/lldb - always garbage (simple enums are fubar)
    * MSVC/cppvsdbg - sometimes garbage (nested enums are fubar)
    * GNU/lldb - mostly fine, but can't follow child processes
    * GNU/gdb - always garbage (I don't have the patience to manually QA rust-gdb on Windows) and doesn't quite understand file paths for break points
* Neither on vscode/Linux:
    * lldb - mostly fine, but can't follow child processes
    * gdb - visualizes variables somewhat awkardly (shows all possible variants of an enum)
* CLion could be the solution, but intellij-rust can't load this project

CUDA <-> L0
-----------
* device ~= device
* stream ~= command queue
* context ~= context (1.0+)
* graph ~= command list
* module ~= module

IGC
---
* IGC is extremely brittle and segfaults on fairly innocent code:
    * OpBitcast of pointer to uint
    * OpCopyMemory of alloca'd variable
