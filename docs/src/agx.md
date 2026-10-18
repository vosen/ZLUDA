# Apple GPUs (AGX, experimental)

ZLUDA can compile PTX for the GPUs of Apple silicon with the experimental
AGX LLVM backend. This covers the compiler only: there is no runtime yet, so
CUDA applications cannot run on Apple GPUs. The output is useful for
inspecting and testing code generation, and as the starting point of a
runtime.

## Building

The AGX backend is not part of upstream LLVM. It lives in a fork of
llvm-project, branch `claude/bold-davinci-46xi9q` of
<https://github.com/Jianwen-Ding/llvm-project>. Two ways to use it:

* Build the fork as part of the cargo build:

  ```
  ZLUDA_LLVM_SOURCE_DIR=/path/to/llvm-project \
      cargo build -p compiler --no-default-features --features agx
  ```

* Or point the build at an existing LLVM build of the fork, configured with
  `-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AGX` (and `AMDGPU` in
  `LLVM_TARGETS_TO_BUILD` when building both backends):

  ```
  ninja -C /path/to/build llvm-config LLVMAGXCodeGen LLVMAGXAsmParser \
      LLVMBitWriter LLVMBitReader LLVMPasses LLVMOption LLVMLTO
  ZLUDA_LLVM_BUILD_DIR=/path/to/build \
      cargo build -p compiler --no-default-features --features agx
  ```

The `amdgpu` cargo feature (on by default) selects the AMDGPU backend and
LLD; `agx` selects the AGX backend. Both can be enabled together.

## Compiling

```
zoc --target agx --arch agx-g14g kernel.ptx
```

`--arch` takes any processor of the backend (`agx-g13g` for M1 to
`agx-g15d` for M3 Ultra; an `agx-` architecture implies `--target agx`).
`zoc` writes the translated LLVM IR (`.ll`), the IR linked with the helper
functions and optimized (`.linked.ll`, `.opt.ll`), the assembly (`.asm`) and
an ELF object (`.o`) with each kernel's machine code in `.text`.

The assembly lists the resources of each kernel: uniform registers,
general purpose registers, threadgroup memory, stack and, when the module has
`.global` or `.const` variables, the uniform registers holding the address of
the global data buffer (whose initial contents are the `.agx.globals`
section). See `llvm/docs/AGXUsage.rst` in the LLVM fork for the kernel ABI.

M4, M5 and A18 (`agx-g16*`, `agx-g17*`) use an instruction encoding that is
not documented well enough to generate code; compiling for them fails with
an error.

## How it works

* `ptx::to_llvm_module_for_target(..., ptx::Target::Agx, ...)` translates
  PTX like for AMDGPU, with the AGX kernel calling convention and no
  `llvm.amdgcn.*` intrinsics: approximate math uses `llvm.agx.*`
  intrinsics, `mul24` and `dp4a` are expanded, and `bar.warp.sync` is a
  no-op (SIMD-groups execute in lockstep). Kernel parameters stay pointers
  to parameter memory, which matches Metal's buffer bindings.
* The `__zluda_ptx_impl_*` helper functions are implemented in LLVM IR in
  `ptx/lib/zluda_ptx_impl_agx.ll` on top of the `llvm.agx.*` intrinsics
  (clang has no AGX target, so the C++ implementation cannot be reused).
* `llvm_zluda::compile_agx` links the helpers, optimizes and generates code;
  backend errors are returned as compilation errors.

## Limitations

* No `f64` (the hardware has no double precision), no 128-bit atomics, no
  tensor core instructions (`mma`, `ldmatrix`), no fp8 conversions, no calls
  to external functions and no function pointers.
* Only round to nearest even: other rounding modes of `.rz`/`.rm`/`.rp`
  instructions that need a mode change are rejected. Denormal modes are
  ignored.
* `%clock` reads as zero. `nanosleep` does not sleep.
* `sqrt.rn` and `div.rn` are not correctly rounded.

`cargo test -p ptx --no-default-features --features agx agx` compiles all PTX
run tests for AGX: 176 of 195 compile, and the remaining ones use the
features above.
