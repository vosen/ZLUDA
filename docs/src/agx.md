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

* No calls to external functions (the tensor core instructions are emulated,
  see [Warp-level matrix instructions](#warp-level-matrix-instructions)). `f64` is emulated in software by the backend (the hardware has
  no double precision): correctly rounded IEEE 754 with denormals, in all
  rounding modes, checked by `ptx/tools/agx_f64_check.py`; it is slow
  (hundreds of instructions per operation). 64- and 128-bit atomics are
  emulated with locks by the backend, and indirect calls are dispatched over
  the module's address-taken functions.
* `alloca` takes a constant size (the AGX stack has a fixed size).
  `bar.red` with a thread count reduces over the SIMD-group when the count
  is 32 (threads arrive at barriers in whole warps) and over the whole
  threadgroup otherwise. `isspacep` is decided at compile time (see below).
* Not supported yet: `fma.relu` of `f16x2`/`bf16x2`, `add.f32x2` and the
  other `f32x2` instructions, `set` of packed integer types, `prmt` modes,
  `szext`, `fns`, the 3-input `min`/`max`, `cvt` with `.pzo`, `.scaled`,
  `s2f6x2` or `ue5m3x2`, and `.ftz`/`.sat` of `bf16` arithmetic (which PTX
  does not have).
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

### Instructions

* `add`, `sub`, `mul` and `fma` of `f16`, `f16x2`, `bf16` and `bf16x2` with
  `.rz`, `.rm` or `.rp` are computed in `f32` in the same rounding mode and
  rounded to the 16-bit type (rounding twice in the same direction is
  exact); conversions to `bf16` with a directed rounding mode round the
  `f32` bit pattern, since the backend rounds them to nearest only.
* The video instructions (`vadd`, `vsub`, `vabsdiff`, `vmin`, `vmax`,
  `vshl`, `vshr`, `vset`, `vmad` and the SIMD `vadd2` ... `vset4`) follow the
  PTX ISA: the selected byte or half-word parts are extended and computed in
  64 bits (32 bits per SIMD lane), with saturation, secondary operations and
  merges.
* The conversions to and from `e4m3x2`, `e5m2x2`, `e2m3x2`, `e3m2x2`,
  `e2m1x2` and `ue8m0x2`, `cvt` to `tf32`, `.relu`/`.satfinite`, `bfind`,
  `bmsk`, `clmad`, the packed byte arithmetic (`.u8x4`, `.s8x4`) and
  `min`/`max` `.xorsign.abs` are integer arithmetic on the bit patterns.
* `isspacep`: AGX has no generic addressing of threadgroup and private
  memory, so a generic address of them must be resolved at compile time.
  The front end emits calls to `@__zluda_agx_isspacep_<space>`, which
  `llvm_zluda` replaces after optimization by whether the address comes from
  that space (addresses of unknown origin are device memory). The same step
  rewrites generic addresses of those memories that went through integer
  arithmetic into pointer arithmetic, and turns those only used as integers
  (compared, masked) into their 32-bit offsets.
* `alloca`, `stacksave` and `stackrestore` use a per-function stack (a
  private array and the offset of its top), so that allocations made after
  a `stackrestore` reuse the freed addresses as in PTX.
* Registers start as zero, and a register used without a declaration is
  declared with the type of its first use (ptxas rejects such code, the K
  semantics of PTX accept it); `.ftz` in a `cvt` without an `f32` type is
  ignored.
* `printf` (`vprintf`) appends a record to the device buffer
  `__zluda_agx_printf_buffer` (64 KiB, in the global data buffer), for a
  host runtime to print. Its first 32-bit word counts the bytes of records
  written (updated atomically; a record that would not fit is dropped, but
  still counted); the records start at byte 8. A record is a `u32` with its
  size (a multiple of 8), a `u32` with the size of its arguments, the `u64`
  device address of the format string and the argument bytes as the caller
  laid them out (each argument aligned to its size: 4 bytes for `%c`, `%d`,
  `%i`, `%o`, `%u`, `%x`, `%X` and `*`, 8 bytes for their `l`/`ll`/`z`/`j`/`t`
  forms, `%p`, `%s` and the floating point conversions). The argument size is
  found by scanning the format string; `vprintf` returns the number of
  conversions.

`cargo test -p ptx --no-default-features --features agx agx` compiles all PTX
run tests for AGX: 176 of 195 compile, and the remaining ones use the
features above.

## Warp-level matrix instructions

AGX has no tensor cores (the SIMD-group matrix instructions of Apple GPUs are
8x8 and round every step). `ldmatrix`, `stmatrix`, `movmatrix`, `mma.sync`,
`mma.sp` and `wmma.load`/`store`/`mma` are computed by the 32 threads of the
SIMD-group, which is the warp, with SIMD-group shuffles and integer
arithmetic. Each variant becomes a call to a helper function named after it
(`__zluda_ptx_impl_agx_mma_m16n8k16_f32_f16_f16_f32`, ...) whose LLVM IR is
generated when the module is emitted (`ptx/src/pass/agx_matrix.rs`) and
inlined.

* Fragments have the layouts of the PTX ISA, as transcribed by the K
  semantics of PTX-formalization: thread `lane` holds elements whose row and
  column are linear in `g = lane / 4` and `t = lane % 4` (checked for all 32
  threads when the helper is generated). The metadata of `mma.sp` and the
  scale factor selectors of `.block_scale` follow PTX-formalization's
  reading of the ISA's figures.
* `ldmatrix` and `stmatrix` shuffle the row addresses of the matrices, and
  each thread loads or stores its own elements (16-bit, or 32-bit when they
  are adjacent); `stmatrix.m16n8` gathers each row and stores it whole.
  `.b8x16.b6x16_p32`, `.b8x16.b4x16_p64` and `.s8.s4` expand the packed
  elements of each 16-byte (8-byte) row into 8-bit containers.
* `mma.sync` gathers the rows of A held by the thread's quad with quad
  shuffles of constant index and the columns of B it needs with SIMD-group
  shuffles, then computes its elements of D. Integer products are summed in
  32 bits (64 bits and clamped with `.satfinite`), `.b1` with `popc` of the
  `xor`/`and` of whole registers.
* Floating point dot products are exact, rounded once to nearest even into
  the type of D, like the model of PTX-formalization (tensor cores round in
  implementation-defined ways, which PTX does not specify): the products and
  C are decoded into integer significands and exponents, added in a fixed
  point accumulator of 120 bits below the top of the largest term (248 bits,
  in two 128-bit limbs, for `f64`), normalized and rounded with denormals,
  overflow to infinity and canonical NaNs. Terms that fall below the window
  only contribute a sticky bit, so a sum is exact unless its terms span more
  than about 100 bits (200 for `f64`) and cancel. The products of `f16`,
  fp8, fp6 and fp4 elements have bounded exponents and are first summed in
  fixed point. An exact zero sum is -0 only if all terms are -0 (for `mma.sp`
  the omitted zeros of A are not counted).
* Block scaled `mma` (`kind::mxf8f6f4`, `mxf4`, `mxf4nvf4`) multiplies each
  block's sum exactly by its `ue8m0` or `ue4m3` scale factors; the
  `{byte-id, thread-id}` selectors must be immediates, and so must the
  sparsity selector of `mma.sp`.
* `wmma` fragments, whose layout PTX leaves unspecified, are tiles of
  `mma.sync` fragments: A is made of the A fragments of its 16-row (8-row)
  tiles, B of the B fragments of its 8-column tiles, C and D of the C
  fragments of the tiles in row major order, repeated where the ISA gives a
  fragment more registers than it needs (`f16` A and B). `wmma.mma` runs
  `mma.sync` on each tile, `wmma.load` and `wmma.store` read and write each
  element (whole registers when their elements are adjacent in memory). The
  layouts are the same for row and column major memory, so fragments can be
  combined freely.

The emulation is large: in instructions per thread (whole test kernels),
130 for `ldmatrix.x2` with `stmatrix.x2`, 170 to 570 for 8-bit and 4-bit integer
`mma`, 80 for `m8n8k128.b1`, 3,000 for `m8n8k4.f64`, 4,300 for
`m16n8k4.tf32`, 6,600 for `m16n8k8.f32.f16`, 12,000 for `m16n8k16.f32.f16`,
11,000 for `m16n8k16.f64` (whose elements are computed in a loop, unrolled
they need more registers than the hardware has), 7,000 to 15,000 for fp8,
10,000 to 20,000 for block scaled fp8 and fp4, and 23,000 for a
`wmma.load`/`mma`/`store` of `m16n16k16.f32.f16`.

Not supported: `mma.m8n8k4` with `f16` A and B (its four quad-pair
products), `.satfinite` and `.relu` of floating point `mma`, `wmma` with a
rounding mode other than `.rn`, register selectors of block scaled and
sparse `mma`, and `wgmma`/`tcgen05`. `ldmatrix` and `stmatrix` need shared
memory addresses: AGX has no generic addressing of threadgroup memory, so
shared memory addresses converted with `cvta.shared` cannot be used.

## Asynchronous copies and mbarriers

The hardware has no asynchronous copy engine and no transaction barriers, so
these are emulated with ordinary instructions, and clusters have a single CTA
(`.shared::cluster` addresses are those of `.shared::cta`, cluster scope
fences are GPU scope ones):

* `cp.async` (with `src-size` or `ignore-src` zero filling), `cp.async.bulk`
  (global to shared, shared to shared, shared to global, also with
  `.cp_mask`) and `cp.reduce.async.bulk` copy synchronously, with loads and
  stores (atomic ones for the reductions) at the point of the instruction.
  `commit_group`, `wait_group` and `wait_all` are no-ops: the copies of a
  thread are complete when it waits, but waiting does not synchronize
  threads, as in PTX. `.multicast::cluster` copies write to the issuing CTA
  only, and cache hints and policies are ignored. `fence.proxy.async` is a
  CTA fence; `fence.sc`/`fence.acq_rel` and `fence.mbarrier_init` are
  ordinary fences.
* An mbarrier is its 8 bytes of shared memory, updated with 32-bit
  threadgroup atomics. Word 0 holds the parity of the current phase in bit 31
  and, in bits 0-30, the pending arrivals times 2^19 plus the pending
  transaction bytes (a 19-bit signed count), modulo 2^31, which is zero
  exactly when the phase is complete. Word 1 holds the expected arrival
  count. `arrive` (with a count, or once with `.expect_tx` bytes),
  `expect_tx`, `complete_tx` and the copies completed on the object update
  word 0 with a compare-and-swap; the update that brings the counts to zero
  flips the parity and reloads the expected arrivals in the same operation.
  `arrive_drop` also decrements the expected count. The state an arrive
  returns is the parity of its phase (bit 0) and the pending arrivals before
  it (bits 32-51, read by `pending_count`); `test_wait` and `try_wait`
  compare that parity (or the given one) with the current one.
  `cp.async.mbarrier.arrive.noinc` is an arrival; without `.noinc` the
  increment and the arrival cancel out.
* `try_wait` waits at a threadgroup barrier between polls, so that the other
  SIMD-groups run until they wait or exit, and returns false after 4096 polls
  (PTX programs retry). This is needed on the functional simulator, which
  switches SIMD-groups only at barriers. On hardware the barriers of
  waiting SIMD-groups meet each other and those of `bar.sync`, so a
  `try_wait` can release a `bar.sync` of another SIMD-group early.
  `test_wait` only polls.

Limitations: an mbarrier counts at most 4095 pending arrivals and
transaction counts between -2^18 and 2^18 - 1 bytes; phases are only
distinguished by their parity; the `.multicast::cluster::32b` mbarrier
operations, tensor copies (`cp.async.bulk.tensor`) and `elect.sync` are not
supported. `ptx/tools/agx_tests/async.ptx` checks the bulk copies, the
reductions and the mbarrier operations with two SIMD-groups.

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
`agx_ptx_formalization_check.py --strict-fp`, checks this). Approximate
instructions (`ex2.approx`, `rcp.approx`, ...) keep the hardware's
approximations.

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
tests use sm_90 and sm_100 features such as `wgmma`, `tcgen05`, TMA and
clusters of several CTAs) are skipped, together with the checks that follow
their launch, and programs ZLUDA rejects as a whole are not counted. Of the
1349 checks of the 285 programs, 664 run: 560 match the formal semantics and
the other 104 are the floating point differences listed above (denormal
`f32` operands and results, NaN bit patterns, `-0.0` under `.sat`, `div` and
`sqrt` precision). With `--strict-fp` (the `--strict-fp` option of the
script), all 664 match. Nine of the `cvt-*.ptx` programs do not compile
because of a backend error on conversions of sign-extended bytes.

All 74 programs of `ldmatrix`, `stmatrix`, `mma`, `mma.sp` and `wmma`
(`ldm-*`, `ldm2-*`, `mma-*`, `mma-bs-*`, `sp-*`, `wmma-*`) match the formal
semantics, with and without `--strict-fp`.
