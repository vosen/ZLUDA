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
* `llvm_zluda::compile_agx` links the helpers (internal to the module, so
  unused ones are removed), optimizes and generates code; backend errors are
  returned as compilation errors.
* Integer division and remainder give the results of NVIDIA GPUs where PTX
  leaves them unspecified: division by zero gives all ones, the remainder of
  a division by zero is the dividend, and the minimum signed value divided by
  -1 wraps.

## Limitations

* No `f64` (the hardware has no double precision), no 128-bit atomics, no
  tensor core instructions (`mma`, `ldmatrix`), no fp8 conversions, no calls
  to external functions and no function pointers.
* The directed rounding modes of `.rz`/`.rm`/`.rp` instructions (and of
  kernels whose instructions all use one) are computed in software, since
  the hardware only rounds to nearest even: an emulated mode register
  (`llvm.agx.set.fpmode`/`llvm.agx.get.fpmode`, see below) holds ZLUDA's
  global modes, and the backend rounds each operation in that mode with
  integer arithmetic. `add`, `sub`, `mul`, `fma`, `div`, `sqrt`, conversions
  from integers and `cvt.f16.f32` are correctly rounded in all four modes.
* 32-bit floating point operations always flush denormal inputs and results
  to zero (the hardware has no other mode), with or without `.ftz`. 16-bit
  operations keep denormals in hardware; `.ftz` 16-bit operations flush them
  in software.
* NaN results have the hardware's bit pattern (for example `0x7e00` for
  `f16`), not PTX's canonical NaN (`0x7fff`, `0x7fffffff`), and saturation
  (`.sat`) turns `-0.0` into `+0.0`.
* `sqrt.rn` and `div.rn` are not correctly rounded.

The [strict floating point mode](#strict-floating-point-mode) removes these
three differences.
* `%clock` reads as zero. `nanosleep` does not sleep.

`cargo test -p ptx --no-default-features --features agx agx` compiles all PTX
run tests for AGX: 176 of 195 compile, and the remaining ones use the
features above.

## Strict floating point mode

`zoc --strict-fp` (`ptx::Options::strict_fp` with
`llvm_zluda::compile_agx(.., strict_fp: true, ..)`) follows the floating
point semantics of PTX where the hardware differs, by emulating them in
software. It enables the `ieee-fp32` and `canonical-nan` features of the AGX
backend (see `llvm/docs/AGXUsage.rst` in the LLVM fork) and lowers `.sat`
exactly:

* 32-bit denormals are kept in kernels without `.ftz`: additions,
  multiplications and fused multiply-adds use the hardware and recompute the
  result with integer arithmetic only where the hardware result can be wrong;
  comparisons, `min`/`max` and rounding handle denormals explicitly.
* `div.rn.f32` and `sqrt.rn.f32` are correctly rounded (refined hardware
  estimates corrected with exact remainders, integer arithmetic outside the
  normal range).
* NaN results are PTX's canonical NaN (`0x7fffffff`, `0x7fff` for `f16` and
  `bf16`).
* `.sat` keeps `-0.0`.

The cost depends on the operation: an addition gets about 10 more
instructions, a fused multiply-add about 24, a division about 67 (see the
AGX backend documentation). The denormal mode is per instruction: in a
kernel that mixes `.ftz` and other `f32` instructions, the instructions
whose mode differs from the kernel's flush their operands and results
explicitly, or keep denormals with the software paths
(`ptx/tools/agx_tests/mixed-ftz.ptx`, run with
`agx_ptx_formalization_check.py --strict-fp`, checks this). `f64` remains unsupported, and
approximate instructions (`ex2.approx`, `rcp.approx`, ...) keep the
hardware's approximations.

`ptx/tools/agx_strict_fp_check.py` checks the mode bit by bit: it compiles
one kernel per operation, runs it on the functional simulator over generated
inputs (denormals, boundaries, cancellations, ties, quotients and roots next
to rounding midpoints, specials) and compares with an exact IEEE 754
reference. All 20 operations it covers (add, sub, mul, fma, div, sqrt,
comparisons, minimum and maximum variants, floor, ceil, trunc, rint) are
bit-exact. `ptx/tools/agx_rounding_check.py` does the same for the directed
rounding modes: `add`, `sub`, `mul`, `div`, `fma`, `sqrt`, `cvt.rn.f32.s32`,
`cvt.rn.f32.u32` and `cvt.f16.f32` in `.rz`, `.rp` and `.rm`, with a static
rounding mode and with the mode set at run time, are bit-exact, with and
without the strict mode.

### The emulated mode register

ZLUDA's mode statements become `llvm.agx.set.fpmode(i32)`, read and written
with `llvm.agx.get.fpmode()` so that a denormal mode change keeps the
rounding modes and the other way round. Bits 0-1 hold the `f32` rounding
mode and bits 2-3 the `f16`/`f64` one (0 toward zero, 1 to nearest even, 2
up, 3 down); bit 4 is set when `f32` denormals are flushed and bit 5 when
`f16`/`f64` ones are. The kernel's modes are its initial value. In a module
with mode changes, or whose kernels use a directed rounding mode, the
floating point operations are constrained intrinsics with dynamic rounding,
which the backend's `agx-lower-fp-env` pass lowers with the register's value
at each operation.

## Checking against PTX-formalization

`ptx/tools/agx_ptx_formalization_check.py` runs the test programs of
[PTX-formalization](https://github.com/Jianwen-Ding/PTX-formalization), whose
expected values are those of its K semantics of PTX, through `zoc --target
agx` and the functional simulator `agx_run` of
[AGX-Performance-Simulator](https://github.com/Jianwen-Ding/AGX-Performance-Simulator),
and compares the results bit by bit:

```
python3 ptx/tools/agx_ptx_formalization_check.py --zoc target/debug/zoc \
    --agx-run AGX-Performance-Simulator/rust/target/release/agx_run \
    --readelf $LLVM/bin/llvm-readelf --objcopy $LLVM/bin/llvm-objcopy \
    PTX-formalization/tests/*.ptx
```

Kernels with statements that ZLUDA's front end does not support (many of the
tests use sm_90 and sm_100 features such as `wgmma`, `tcgen05`, TMA,
clusters and `mbarrier`) are skipped, together with the checks that follow
their launch, and programs ZLUDA rejects as a whole are not counted. Of the
1125 checks of the 220 programs that compile, 213 run: 191 match the formal
semantics and the other 22 are the floating point differences listed above
(denormal `f32` comparisons, NaN bit patterns, `-0.0` under `.sat`, `div`
and `sqrt` precision). With `--strict-fp` (the `--strict-fp` option of the
script), all 213 match.
