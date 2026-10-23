# Apple GPUs (AGX, experimental)

ZLUDA can compile PTX for the GPUs of Apple silicon with the experimental
AGX LLVM backend, and run it on M1 and M2 GPUs under Asahi Linux with
`zluda_agx` (the CUDA driver API, see "The CUDA driver on Asahi") and the
`*_agx` libraries. The compiler also runs on its own (`zoc`), for inspecting
and testing code generation.

## Building

The `compiler` crate (and so `zoc`) needs the HiGHS submodule:
`git submodule update --init ext/HiGHS`.

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
section). Variables initialized with addresses of other variables have
`R_AGX_ABS64` / `R_AGX_ABS32` relocations in `.rela.agx.globals`, which
`agx_runtime` applies once it has placed the buffer. See `llvm/docs/AGXUsage.rst` in the LLVM fork for the kernel ABI.

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
* Variable initializers may hold addresses (`name`, `generic(name)`, plus an
  offset), which the object relocates (`R_AGX_ABS64` / `R_AGX_ABS32` in
  `.rela.agx.globals`), and `mask(expr)` bytes (PTX ISA 5.4.4): of a
  constant at compile time, of an address `mask(name + offset)` through the
  table `__zluda_agx_masked_init` (per element its address, the address and
  the shift with the element size), which the loader applies after the
  relocations (`agx_runtime::Module::relocated_globals`).

## Limitations

* No calls to external functions (the tensor core instructions are emulated,
  see [Warp-level matrix instructions](#warp-level-matrix-instructions)). `f64` is emulated in software by the backend (the hardware has
  no double precision): correctly rounded IEEE 754 with denormals, in all
  rounding modes, checked by `ptx/tools/agx_f64_check.py`; it is slow
  (hundreds of instructions per operation). 64- and 128-bit atomics are
  emulated with locks by the backend, and indirect calls are dispatched over
  the module's address-taken functions. Recursive functions run over an
  explicit stack of frames in each thread's private memory (four times
  `CU_LIMIT_STACK_SIZE`, 4 KiB by default; see the
  backend's `AGXUsage.rst`, *Recursion*); a thread that runs out of it ends and
  the launch fails with `CUDA_ERROR_LAUNCH_FAILED`.
* Device-side `malloc` / `free` allocate from one heap per context of
  `CU_LIMIT_MALLOC_HEAP_SIZE` bytes (8 MiB by default, fixed once a module that
  uses `malloc` is loaded), shared by its modules: power-of-two blocks from 32
  bytes with lock-free free lists per size, reused but never merged
  (`ptx/lib/zluda_ptx_impl_agx_heap.c`). `malloc` returns NULL when no block
  fits. In checked mode the heap counts as one allocation.
* `alloca` / `stacksave` / `stackrestore` work on a per-thread stack in private
  memory that grows down, as in PTX. Allocations of a constant size are
  reserved in full; those of a dynamic size share `CU_LIMIT_STACK_SIZE` bytes
  (1024 by default; `zoc --stack-size`, applied to the modules loaded after
  `cuCtxSetLimit`). A thread whose allocation does not fit ends and the
  launch fails with `CUDA_ERROR_LAUNCH_FAILED`.
  `bar.red` with a thread count reduces over the SIMD-group when the count
  is 32 (threads arrive at barriers in whole warps) and over the whole
  threadgroup otherwise. `isspacep` is decided at compile time where it can
  be, and at run time otherwise (see below). `prmt` takes every mode
  (`.f4e`, `.b4e`, `.rc8`, `.ecl`, `.ecr`, `.rc16`), and `szext` and `fns`
  are helpers (AGX only: the AMD path reports them as not implemented). The
  3-input `min`/`max` of `f32`, `f16`, `f16x2`, `bf16` and `bf16x2` (sm_100)
  is two 2-input operations.
  `add`, `sub`, `mul` and `fma` of `f32x2` (sm_100, two f32 in a 64-bit
  register) are `<2 x float>` operations that follow the f32 rounding and
  denormal modes, directed rounding included. `fma.relu` of `f16`, `f16x2`,
  `bf16` and `bf16x2` clamps negative results to +0 and keeps NaN (PTX gives
  it only `.rn`; another rounding mode is reported as an error).
  `lop3` (any lookup table, also in a register, and the predicate output
  form `lop3.and`/`lop3.or`) is the OR of the table's minterms.
  `set` of packed integer lanes (`u8x4`, `s8x4`, `u16x2`, `s16x2`) gives
  all-ones lanes where the comparison holds. `set` and `setp` of `f16x2` and
  `bf16x2` take every destination type (`.u32`, `.s32`, `.f16x2`, `.bf16x2`)
  and the `BoolOp` forms, scalar `set` also `.u16`, `.s16`, `.f16` and
  `.bf16` destinations; a `setp` destination may be the sink `_`. `min` and
  `max` with `.abs` and no `.xorsign` take the magnitudes of their inputs.
  Their NaN results, and those of `.relu`, are the canonical NaN.
  `.ftz` of comparisons, `min`/`max` and `neg`/`abs` in a function whose
  instructions take several denormal modes (a mode register switched at run
  time) flushes with bit operations.
  `ptx/src/test/agx_sim/run_sim.py` checks these instructions and `cuLink` on
  the simulator against reference models.
* Every `f32x2` instruction PTX has (`add`, `sub`, `mul`, `fma`, also with
  `f16x2`/`bf16x2` operands) is supported. `.ftz`/`.sat` of `bf16` arithmetic
  is not, as PTX does not have it either.
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
* No special register is a clock (ptx-re-blockers B35). When a module that
  reads `%clock`, `%clock64` or `%globaltimer` or calls `nanosleep` is loaded,
  `zluda_agx` starts a CPU thread that keeps writing the CPU's generic timer
  (24 MHz on Apple chips) into a small buffer of the context, which the kernel
  reads with a volatile load (the invalidating barrier and a coherent load,
  about 1 us): `%globaltimer` is in nanoseconds, `%clock64` in cycles of the
  reported clock rate, both in steps of 41.7 ns, and `nanosleep` waits for the
  time to pass (at most 2^16 reads). The thread keeps a CPU core busy;
  `ZLUDA_AGX_TIMER=0` turns it off, and without it (also in the simulator) the
  clocks count reads of a device-wide counter, later at each read of a thread
  but not measuring time, and `nanosleep` returns at once.
* Modules for `.target sm_1x` get its rules: single-precision instructions flush subnormals with or without
  `.ftz`, and `mad.f32` without a rounding mode truncates the product to 24 bits before the addition (as on
  those GPUs; `mad.rn.f32` and later targets fuse). The deprecated `vote.{all,any,uni}.pred` and
  `vote.ballot.b32` without `.sync` vote over the active threads.
* `%dynamic_smem_size`, `%total_smem_size` (static and dynamic bytes in
  128-byte units), `%aggr_smem_size` and `%reserved_smem_offset_begin`,
  `_end` and `_cap` (a reserved region of 1 KiB after the CTA's shared memory,
  which nothing uses and no memory backs) read the shared memory of the
  launch: the kernels of a module that reads them take it in a hidden argument
  that the driver sets (the backend's `"agx-hidden"="threadgroup-bytes"`,
  `.agx.kernels` extension 4). In a cluster kernel they are the
  threadgroup's.
* Modules with `.address_size 32` take kernels only, and their `.global` and
  `.const` variables only as byte arrays; their shared and local memory has
  no 32-bit generic addresses.

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
* The mixed-precision forms (`add`/`sub`/`mul`/`fma` of `.f32.{f16,bf16}`,
  `.f32x2.{f16x2,bf16x2}.f32x2`, `.{f16x2,bf16x2}.f32x2.f32x2` and
  `mul.bf16x2.bf16x2.f16x2`) convert their operands exactly and compute in
  `f32` (in `.rz`, then rounded with `.rz`, for 16-bit pair results; with
  `.ftz` an `.f16x2` result's subnormal halves are flushed too).
* The PTX ISA 9.x conversions (`.pzo`, also on `f16`, `bf16`, `tf32`,
  `f16x2` and `bf16x2` results, `.scaled::n1::ue8m0` and
  `.scaled::n2::ue8m0`, `ue5m3x2` and `s2f6x2`, `.rp` / `.rm` to the mini
  formats (toward positive / negative infinity on the signed value), `.rn`
  to `ue8m0x2` (the nearest power of two, ties to the even code), the
  `.relu` / `.satfinite` up-conversions, stochastic rounding `cvt.rs` to
  `f16x2` / `bf16x2` and to the x4 formats), the alternate floating point x4
  arithmetic (`add` / `sub` / `mul` / `fma` of `e4m3x4`, `e5m2x4`, `e2m3x4`,
  `e3m2x4`, `e2m1x4`, `e2m1p4x4` and `ue8m0x4`, each element exact and
  rounded once) and `spcompress` / `spdecompress` run in software
  (`ptx/lib/zluda_ptx_impl_agx_cvt.c`), with every value held as an integer
  significand and a power of two. `fma.oob` gives 0 for an element whose
  operand is the OOB NaN of the tensor memory accelerator (`0x7e00` for f16,
  `0x7fc0` for bf16).
* The conversions to and from `e4m3x2`, `e5m2x2`, `e2m3x2`, `e3m2x2`,
  `e2m1x2` and `ue8m0x2`, `cvt` to `tf32`, `.relu`/`.satfinite`, `bfind`,
  `bmsk`, `clmad`, the packed byte arithmetic (`.u8x4`, `.s8x4`) and
  `min`/`max` `.xorsign.abs` are integer arithmetic on the bit patterns.
* Generic addresses: AGX has no generic address space; device, threadgroup
  and private memory are reached by different instructions. The LLVM AGX
  backend (`AGXLowerGenericAddresses`) represents a generic pointer to
  threadgroup memory as its offset with bit 63 set and one to private memory
  as its offset with bit 62 set (device addresses are below 2^48), and a
  load, store or atomic through a generic pointer whose memory is not known
  at compile time tests those bits and branches to the right instruction.
  `cvta`, `ld`/`st`/`atom` on generic addresses and `ldmatrix`/`stmatrix` on
  generic shared addresses therefore work wherever the addresses come from.
  A private address is only meaningful in the thread that made it, as in
  PTX.
* `isspacep`: the front end emits calls to `@__zluda_agx_isspacep_<space>`,
  which `llvm_zluda` replaces after optimization by a constant when the
  address's memory is known, and otherwise by a test of the tag bits. The
  address of a kernel parameter (`cvta.param`) is in the `.param` and
  `.global` windows, not the `.const` one; a device address of unknown
  origin is `.const` when it is in one of the module's `.const` variables. The
  same step rewrites generic addresses of threadgroup and private memory that
  went through integer arithmetic into pointer arithmetic, and turns those
  only used as integers (compared, masked) into their 32-bit offsets, which
  avoids the tagged form where it is not needed.
* `alloca`, `stacksave` and `stackrestore` use a per-function stack (a
  private array and the offset of its top), so that allocations made after
  a `stackrestore` reuse the freed addresses as in PTX.
* `sub.cc` / `subc.cc` leave CC.CF as the carry of `a + ~b + 1` (1 when there
  is no borrow), as NVIDIA GPUs do (`ptx/src/test/spirv_run/subc_cc_s32`,
  recorded on one), and `subc` computes `a + ~b + CC.CF`; PTX-formalization's
  `sub-borrow` follows the PTX ISA's wording instead (CC.CF as the borrow),
  which only differs when the flag is read by `addc`.
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
* `__assertfail` writes CUDA's message (`file:line: function: block: [x,y,z],
  thread: [x,y,z] Assertion `+"`expr`"+` failed.`) as a printf record, sets
  `__zluda_agx_trap_flag` (which `agx_runtime` reports as
  `CUDA_ERROR_ASSERT`), orders its stores with a release fence and executes
  `trap`. On the M2 Max an executed `trap` does nothing (AGX Performance
  Simulator `tasks/ptx-re-blockers.md`, B21), so the flag word is what reports
  the assertion, and the LLVM backend makes a thread that reaches `trap`
  return there: the failing thread stops, the others run on (the threadgroup
  barrier does not wait for it, B28). `CUDA_ERROR_ASSERT` is sticky, as in
  CUDA: every call returns it until the context is destroyed.
* Lanes of one warp run one side of a divergent branch to completion before
  the other side (B4), so a lane that spins waiting for a lane of its own warp
  on the other side of a branch never finishes. This is the pre-Volta (`sm_6x`)
  behaviour; independent thread scheduling is not available.

Register use limits the threadgroup size: a threadgroup runs on one core, and
a core holds 2 * floor(3328 / (2 * H)) SIMD-groups of a kernel that uses H
16-bit registers (H rounded up to a multiple of 8), so 52 32-bit registers
allow 1024 threads and 128 allow 384. `.maxntid` / `.reqntid` (times
`.minnctapersm`) and `.maxnreg` make the backend spill instead of exceeding
the registers that many threads need. `cuFuncGetAttribute(MAX_THREADS_PER_BLOCK)`
reports the limit, and a launch with a larger block fails with
`CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES`, as on NVIDIA GPUs. The per-core budget is
measured on the M2 Max (AGX Performance Simulator `tasks/safe-11`) and agrees
with Mesa's occupancy table. That a single threadgroup must fit in it, and
that the Registers word encodes 256 halves as 0, are measured too
(`tasks/ptx-re-blockers.md`, B26): one granule over the budget, a 1024-thread
threadgroup with a barrier hangs.

Launches take at most 1024 threads in a block, along any of x, y and z (B22;
a 2048-thread threadgroup runs nothing). The launch words count threads, so
`gridDim * blockDim` must also be below 2^32 in each dimension; the grid is at
most 2^31 - 1 CTAs along x and 65535 along y and z, as on NVIDIA GPUs. A launch
outside these limits fails with `CUDA_ERROR_INVALID_VALUE`.

`cuOccupancyMaxActiveBlocksPerMultiprocessor` reports the CTAs per core that
are resident together: min(15, floor(S / g) - 1) for a kernel of which a core
holds S SIMD-groups and g per CTA, and less with shared memory (B5). Only a
grid of at most that many CTAs times the core count is resident as a whole:
beyond it, later CTAs start when earlier ones exit (B5, B30), so a software
grid barrier or a CTA that spins until a later CTA writes needs a grid within
the limit. Cooperative launches are not supported.

`cargo test -p ptx --no-default-features --features agx agx` compiles all PTX
run tests for AGX: 194 of 195 compile on their own. The other, `extern_func`,
calls a function defined in another module: it compiles when linked with one
(`cuLink*`, below).

### Linking (`cuLink*`)

`cuLinkCreate` / `cuLinkAddData` / `cuLinkAddFile` / `cuLinkComplete` take
PTX (or the PTX of a fat binary) and link it into one AGX object, which
`cuModuleLoadData` then loads: `zoc` translates each module, links them as
LLVM IR (the `.extern` declarations of one resolve to the `.visible`
definitions of another) and makes every non-kernel function internal, so that
the calls are inlined as AGX requires. On AGX, `.visible` functions are kept
even when their own module does not call them. An AGX object can be "linked"
only on its own, and NVIDIA objects or libraries (`CU_JIT_INPUT_CUBIN`,
`OBJECT`, `LIBRARY` holding SASS) return `CUDA_ERROR_NO_BINARY_FOR_GPU`.
`zoc` links the same way when given several PTX files. `cuModuleLoad` (a
file) and `cuModuleLoadDataEx` (JIT options ignored) are supported too.

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
  point accumulator of 120 bits below the top of the largest term,
  normalized and rounded with denormals, overflow to infinity and canonical
  NaNs. Terms that fall below the window only contribute a sticky bit, so a
  sum is exact unless its terms span more than about 100 bits and cancel.
* `f64` elements are what the PTX ISA specifies instead: the chain of `f64`
  fused multiply-adds along k starting from C, rounded in the direction of
  the instruction's `.rn`, `.rz`, `.rm` or `.rp`.
* `mma.m8n8k4` with `f16` A and B is four independent 8x8x4 products, one
  per quad pair (threads 4q to 4q + 3 and 4q + 16 to 4q + 19), with every
  `.row`/`.col` layout of A and B and `f16` or `f32` C and D: each element of
  D gathers its row of A, its column of B (and, when C and D have different
  types and so different layouts, its element of C) from the quad pair with
  SIMD-group shuffles. The products of `f16`,
  fp8, fp6 and fp4 elements have bounded exponents and are first summed in
  fixed point. An exact zero sum is -0 only if all terms are -0 (for `mma.sp`
  the omitted zeros of A are not counted).
* Block scaled `mma` (`kind::mxf8f6f4`, `mxf4`, `mxf4nvf4`) multiplies each
  block's sum exactly by its `ue8m0` or `ue4m3` scale factors (NaN factors,
  `ue8m0` 0xff and `ue4m3` 0x7f, make the products NaN); the
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
`mma`, 80 for `m8n8k128.b1`, 4,300 for
`m16n8k4.tf32`, 6,600 for `m16n8k8.f32.f16`, 12,000 for `m16n8k16.f32.f16`
(the elements of `f64` mma are computed in a loop of software fused
multiply-adds), 7,000 to 15,000 for fp8,
10,000 to 20,000 for block scaled fp8 and fp4, and 23,000 for a
`wmma.load`/`mma`/`store` of `m16n16k16.f32.f16`.

Not supported: `.satfinite` and `.relu` of floating point `mma`, `wmma` with a
rounding mode other than `.rn`, and register selectors of block scaled and
sparse `mma`.

The metadata of `mma.sp` with two chunks of rows per thread follows the
ISA's figures: for `f16` / `bf16` `m16n8k32` and `tf32` `m16n8k16`, thread
`2 sel + c / 4` of the quad holds chunks `c` of rows `g` (bits 0-15) and
`g + 8` (bits 16-31); for 8-bit `m16n8k32`, thread `2 sel + h` holds the 8
chunks of row `g + 8h`.

`zoc --fast-mma` (`ZLUDA_AGX_FAST_MMA=1` for `zluda_agx`) computes `mma` and
`wmma` with `f16`, `bf16` or `tf32` A and B and `f16` or `f32` C and D as
8x8x8 `llvm.agx.simdgroup.matrix.fma` products in `f32`: the matrix unit
(`simd_matrix_fmadd32`) on G14, the same chain of fused multiply-adds with
shuffles on other chips. The operands are gathered from the PTX fragments
with SIMD-group shuffles (whose lanes are fitted at compile time as linear
functions of the lane bits; layouts that do not fit keep the exact form), A
and B are converted to `f32` exactly, `k` is padded with zeros to a multiple
of 8, and an `f16` D is rounded once from `f32`. It is not the exact sum of
the default emulation: the products are summed by rounded fused
multiply-adds in `k` order and `f32` denormals are flushed (the `mma-*`
programs of PTX-formalization then differ from the model by a few units in
the last place). A kernel with an `m16n8k16.f32.f16` mma has 298 instead of
1,237 instructions on G14 (4 `simd_matrix_fmadd32`), 728 on G13.

### Fifth-generation tensor core (`tcgen05`)

The tensor memory of a CTA (128 lanes of 512 32-bit columns, 576 for an
exclusive allocation, in rows of 1024 words) is a 512 KiB slab of device
memory: a pool of 4 slabs is a module global
(`ptx/lib/zluda_ptx_impl_agx_cvt.c`). A CTA claims a free slab at its first
`tcgen05.alloc` (the slab's owner word holds the CTA's linear index + 1, found
again by every later instruction), allocates columns first fit from the slab's
map of 32-column granules, and gives the slab back with its last
`tcgen05.dealloc`. A CTA that finds no free slab waits for one, so at most four
CTAs that use tensor memory run at once. The wait cannot deadlock: a CTA that
holds a slab is running, and a resident CTA is not preempted by the ones
spinning (B30), so it reaches its `dealloc` unless it waits on a CTA that has
not started. `alloc` and `dealloc` are done by the
lowest active lane of the warp, and the warp then zeroes the allocated columns
(the tensor memory of a new CTA reads as zero). `alloc` writes the address
through a shared or (without `.shared::cta`) generic pointer, as `commit`
takes its mbarrier. `.exclusive` (PTX 9.4) allocates any multiple of 32
columns up to 576 from column 0, waiting until the CTA has no other live
allocation; other allocations wait while it is live. `relinquish_alloc_permit`,
`fence` and `wait` do nothing (every operation completes before the next
instruction).

* `tcgen05.ld` / `st` (shapes `32x32b`, `16x64b`, `16x128b`, `16x256b`,
  `16x32bx2`, `.pack::16b` / `.unpack::16b`), `ld.red` (min / max of f32 with
  `.abs` / `.NaN`, u32, s32) and `ld.spcompress` are loads and stores of the
  slab words. In the 16-lane shapes thread t covers lanes t / 4 and t / 4 + 8
  (ISA Figures 184-186); with `.pack::16b` a 32-bit cell takes two adjacent
  16-bit columns.
* `tcgen05.mma` (kinds `f16`, `tf32`, `i8`, `ti16`, `f8f6f4`, `mxf8f6f4`,
  `mxf4`, `mxf4nvf4`; `.sp` (with the SparsityVersion of the `mxf4` kinds),
  `.ws` with its zero-column mask, `.ashift`, the disable-output-lane mask,
  `scale-input-d`, `.block_scale` with every scale vector size and the UE8M0,
  UE4M3 and UE5M3 scale types (scale_A in the 32- or 128-lane layout),
  `.decompress::lut::b`, `.cta_group::2`; the `.collector` qualifiers have no
  effect) is computed by the issuing thread: a loop over
  the elements of D, reading A and B through their shared memory descriptors
  (or A from tensor memory), each element the exactly rounded sum of its
  products and of C (the helper is generated by `agx_matrix.rs`). The
  instruction descriptor is an operand; it is a constant in practice and the
  code for the other types folds away.
* `tcgen05.cp` (every shape, the `.b8x16.b6x16_p32` / `.b4x16_p64`
  decompression), `shift` and `commit` (an arrive on the mbarrier) are
  library functions.
* `.cta_group::2`: the issuing thread also does the work of the peer CTA of
  the pair (its rows of A and D, the odd CTA's half of B), through the
  cluster's shared memory windows and the peer's slab.

All 56 PTX-formalization `tcgen05` programs match the formal semantics
(`tcg2-1`, with 2 x 19 KiB of shared memory per cluster, keeps it in device
memory: see thread block clusters). A shared memory address
written as a constant in a cluster kernel is the address of the first CTA's
window (addresses taken from shared variables are right in every CTA).

### Warpgroup MMA (`wgmma`)

`wgmma.mma_async` (dense and `.sp`, f16, bf16, tf32, fp8 and integer types) runs
as `mma.sync` tiles. Warp `w % 4` of the warpgroup computes rows
`16w..16w+15` of D as `n / 8` tiles of `m16n8kK`, through one tile helper
that is inlined at each use (AGX has no calls). A comes from the registers or,
like B, from shared memory through its matrix descriptor (start address, LBO,
SBO, swizzle modes 0 to 6 and the base offset of bits 49-51, which starts the
swizzle pattern that many 128-byte lines before the address, following
`ptx-exec-wg.k`), K-major or
transposed with `.trans`. Sparse A in shared memory is the packed
`M x K/2` tile, read in the element order of the `mma.sp` fragment.
`imm-scale-a/b` = -1 negate the elements, and `scale-d` is either an
immediate or a predicate. Every `wgmma` is synchronous on AGX, so
`wgmma.fence`, `commit_group` and `wait_group` compile to nothing. D can have
up to 128 registers (`n256` with `f32`). A vector operand can now hold up to
256 elements. All `wg-*`, `wg-baseoff-*` and `wgsp-*` programs of
PTX-formalization match the formal semantics.

### Tensor memory accelerator (`cp.async.bulk.tensor`)

The TMA copies are emulated by the issuing thread, element by element, and
complete at once (the mbarrier transaction completes when the copy is
issued). Supported:

* tile loads, stores and reductions (`add`, `min`, `max`, `inc`, `dec`,
  `and`, `or`, `xor`; signedness and float from the tensor map's element
  type), 1D to 5D, with element strides (a box dimension of `n` elements with
  stride `s` holds `ceil(n / s)` of them), out-of-bounds zero or NaN fill and
  the shared-memory swizzles (32B, 64B, 128B, 96B, which has the pattern of
  32B, and 128B with 32B, 32B + 8-byte flip and 64B atomicity), by the
  CTA-relative shared address;
* `.multicast::cluster` (16- and 32-bit masks) and `.cta_group::2`;
* the im2col modes `.im2col`, `.im2col::w`, `.im2col::w::128` and stores with
  `.im2col_no_offs::w`;
* `.override::global_address` and `.override::global_dim{_stride}` (the
  patched map is a private copy);
* the packed sub-byte types (`16U4_ALIGN8B`, `16U4_ALIGN16B`,
  `16U6_ALIGN16B`);
* `tensormap.replace` (every field).

The 128-byte tensor map is opaque to PTX. `cuTensorMapEncodeTiled`,
`cuTensorMapEncodeIm2col` and `cuTensorMapReplaceAddress` write the encoding
of PTX-formalization's model (16 64-bit words: element type, rank, address,
dims, strides, box, swizzle, fill, interleave, element strides, im2col
corners), and the emulation reads the same one. Tensor maps are read with
ordinary loads, so `fence.proxy.tensormap::generic` is a fence of its scope.
The `tma*` and `tma-*` programs of PTX-formalization match the formal
semantics, except `.report_mechanism` (`tma-report`). The interleaved layouts are plain copies of the dims, as in the
model. `cp.async.bulk.prefetch.tensor` and `.tile::gather4` / `scatter4` are
not supported yet.

### Textures and surfaces

Textures and surfaces are sampled in software (`ptx/lib/zluda_ptx_impl_agx_tex.c`, compiled to the generated
part of `zluda_ptx_impl_agx.ll` by `ptx/lib/gen_agx_tex.py`), except where the texture unit can sample as the
software does (below). The software follows PTX-formalization's model:
- coordinates scale by the extent when normalized;
- point and linear filtering, with weights quantised to 1/256 and the sum computed exactly (in software fp64)
  and rounded once;
- wrap, clamp, mirror and border (zeros) address modes;
- read-normalized 8- and 16-bit integers;
- mipmaps: `.level`, `.grad`, nearest or linear between levels;
- 1D / 2D / 3D, layered, cube and layered cube textures;
- depth compare (LEQUAL);
- texel offsets.

Supported instructions:
- `tex` (`.base`, `.level`, `.grad`, separate samplers, the residency predicate, `.f16` and `.f16x2`
  destinations, multi-sample `2dms` and `a2dms`);
- `tld4`, `txq` (also `txq.level`), with PTX's encodings of the address modes;
- `suld.b`, `sust.b` and `sust.p` (formatted), `sured.b` and `sured.p`, each with `.trap` (an out-of-bounds
  access reports `CUDA_ERROR_ASSERT`), `.clamp` and `.zero`;
- `suq`, `istypep`.

The runtime supports:
- CUDA arrays (`cuArrayCreate`, `cuArray3DCreate`, layered and cubemap; `cuMipmappedArray*`);
- copies to and from arrays (`cuMemcpy2D`, `cuMemcpy3D`, `cuMemcpyHtoA` / `AtoH`);
- `cuTexObjectCreate` over arrays, mipmapped arrays, linear and pitch-linear memory;
- `cuSurfObjectCreate` over arrays.

**The texture unit.** `tex.1d` and `tex.2d` of a texture object with float coordinates, automatic LOD and no
separate sampler, offsets or depth compare sample with the hardware's `texture_sample`
(`llvm.agx.texture.sample`, bindless: LLVM's AGXUsage.rst, "Textures"), outside `--strict-fp`, on a texture that
`cuTexObjectCreate` (or a texture reference) gave hardware descriptors. Those are the 24-byte texture and 8-byte
sampler descriptors of Mesa's `cmdbuf.xml` layout (read for their bit layout only), which `agx_runtime` keeps per
address space: the texture heap, whose address a module that samples receives in `u0_u1` (`.agx.kernels`
extension 5), and the sampler heap of every compute command (`drm_asahi_cmd_compute.sampler_heap`, equal samplers
shared, at most 1024). A texture gets them when it is:
- a linear 1D or 2D image (an array or pitch-linear memory) of one level and one layer, not a cubemap, at most
  16384 texels wide and high, its address and row pitch multiples of 16 bytes;
- read as floats (float or half texels, or normalized 8- and 16-bit integers; linear filtering allowed) or as
  integers with point filtering, the PTX destination type matching;
- addressed with clamp or border (border only for four channels), or with wrap or mirror on normalized
  coordinates;
- not sRGB.

Word 16 of the software descriptor names the hardware ones (bit 63; bit 62 integer results, bit 61 2D, the
sampler index at bit 32, the texture heap offset in the low 32 bits); any other texture, form or the
`ZLUDA_AGX_SOFTWARE_TEXTURES` environment variable keeps the software path. Point sampling gives the same
results; linear filtering uses the hardware's weights rather than the model's 1/256 steps, and the f16 forms' results
are rounded once more, which is why `--strict-fp` does not use the unit. The
formalization harness builds the same descriptors (`agx_run --sampler-heap`), and the texture tests pass both
ways. On the GPU the texture unit first read zeros: `texture_sample`'s register sampler index of heap sampler k is
k + 28 (the lower indices are sampler state slots; Mesa's Honeykrisp stores heap index + 28), which word 16 now holds.
With it, `agx_texture_check` (a random RGBA8 1024 x 512 array at 2^20 coordinates, normalized, wrap) gives the
software sampler's results on the M2 Max: identical integers, and normalized floats with linear filtering within
1.3e-4. Until more forms are checked on the GPU, the texture unit is only used when `ZLUDA_AGX_HARDWARE_TEXTURES` is
set; by default every texture takes the software path.

Textures read as integers (`CU_TRSF_READ_AS_INTEGER` on 8- and 16-bit formats, and 32-bit integer formats) are point
sampled even when linear filtering is asked for: CUDA documents linear filtering only for textures that return
floating-point data (and as unsupported with `CU_TRSF_READ_AS_INTEGER`). Geekbench 7's path tracer creates its 8-bit
environment map that way.

Module `.texref`, `.samplerref` and `.surfref` variables compile: on AGX each holds the 64-bit handle of
a texture, sampler or surface object, which the instructions load. The legacy texture reference API
(`cuModuleGetTexRef`, `cuTexRefSetArray`, `SetMipmappedArray`, `SetAddress`, `SetAddress2D`, `SetFormat`,
`SetAddressMode`, `SetFilterMode`, `SetMipmapFilterMode`, `SetFlags`, `GetAddress`) fills a `.texref`
variable with a texture object made from the reference's settings, again after each change.

Not supported:
- surface and sampler references (`cuModuleGetSurfRef`, `cuSurfRefSetArray`);
- arbitrary border colours (the border is zero);
- the min/max mipmap level clamps and the level bias of `CUDA_TEXTURE_DESC`;
- per-dimension address modes (the first one applies).

All PTX-formalization texture and surface programs (`tex`, `tex-mip`, `surf`, `istypep`) match the formal
semantics. A texture fetch runs a few thousand instructions; each `tex` is inlined, so kernels with many `tex`
instructions take minutes to compile (`tex-mip`, 33 of them: 3 minutes).

## Asynchronous copies and mbarriers

The hardware has no asynchronous copy engine and no transaction barriers, so
these are emulated with ordinary instructions (clusters: see the next
section; cluster scope fences are GPU scope ones):

* `cp.async` (with `src-size` or `ignore-src` zero filling), `cp.async.bulk`
  (global to shared, shared to shared, shared to global, also with
  `.cp_mask`) and `cp.reduce.async.bulk` copy synchronously, with loads and
  stores (atomic ones for the reductions) at the point of the instruction.
  `commit_group`, `wait_group` and `wait_all` are no-ops: the copies of a
  thread are complete when it waits, but waiting does not synchronize
  threads, as in PTX. `.multicast::cluster` copies write to the same offset
  of each CTA in the mask and complete the transaction on each one's
  mbarrier. Cache hints and policies are ignored. `fence.proxy.async` is a
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
* `mbarrier.init.layout::v1` sets bit 24 of word 1 (bits 0-23 hold the
  expected count), and `.layout::v1` objects record more state there: bit 25
  enables the report of the last completed phase, which `test_wait` and
  `try_wait` with a report destination (a predicate or a value) read from
  bit 26, and bit 27 holds the parity of the conditional phase that
  `.phase::conditional` waits test. `mbarrier.arrive_drop` with
  `.multicast::cluster` drops on each CTA in the mask.
* `cp.async.bulk` and `cp.async.bulk.tensor` with a `.report` mechanism
  (PTX 9.4) record the outcome of the copy in the destination mbarrier
  before its transaction completes: `.report::mbarrier::inspect` writes the
  report code to the mbarrier's report bits, which the waits then return.
  The PTX 9.4 cache hints (`prefetch` with `.l1_size` and `.valid_addr`,
  `ld` with `.proxy::readonly`) are accepted and ignored, as are the
  `fence.proxy.alias` and `fence.proxy.async::generic...sync_restrict`
  variants, which are CTA fences.

Limitations: an mbarrier counts at most 4095 pending arrivals and
transaction counts between -2^18 and 2^18 - 1 bytes; phases are only
distinguished by their parity; the `.multicast::cluster::32b` mbarrier
operations (`expect_tx`, `complete_tx`, `arrive` and `arrive.expect_tx`)
apply to the mbarrier at the same offset of each CTA in the mask.
`ptx/tools/agx_tests/async.ptx` checks the bulk copies, the
reductions and the mbarrier operations with two SIMD-groups.

## Thread block clusters

AGX threadgroups cannot see each other's threadgroup memory, so a kernel with
`.reqnctapercluster x, y, z` runs each cluster of C = x * y * z CTAs as one
threadgroup of C times the CTA's threads. The backend records the cluster
shape in the kernel's `.agx.kernels` record (extension 1) and the launch
(`agx_runtime`, `zluda_agx`, the conformance harness) divides the grid by
(x, y, z) and multiplies the threadgroup's z by C. Thread (x, y, z) of CTA r
of the cluster is thread (x, y, z * C + r) of the threadgroup, so with CTAs of
depth 1 each CTA's threads are consecutive, in whole SIMD-groups when the CTA
has a multiple of 32 threads. After inlining, `LLVMZludaAgxLowerClusters`
(llvm_zluda) rewrites each cluster kernel:

* `%tid`, `%ntid`, `%ctaid`, `%nctaid` and the linear thread index are the
  CTA's; `%clusterid`, `%nclusterid`, `%cluster_ctaid`, `%cluster_nctaid`,
  `%cluster_ctarank`, `%cluster_nctarank` and `%is_explicit_cluster` are
  derived from the hardware ids (kernels without clusters get the values of a
  single-CTA cluster).
* The shared variables the kernel uses are packed into one array of C
  windows, CTA r using window r. A `.shared::cluster` address is a
  threadgroup address: `mapa` adds (r - rank) times the window size,
  `getctarank` divides by it, `isspacep.shared::cluster` is any window and
  `isspacep.shared(::cta)` the CTA's own.
* `barrier.cluster.wait` is the threadgroup barrier (with release and
  acquire fences); `barrier.cluster.arrive` only orders memory. A CTA's
  `bar.sync` is also the threadgroup barrier: correct when the CTAs of a
  cluster execute the same sequence of CTA barriers, or have exited. That a
  barrier does not wait for SIMD-groups that have ended is what the
  functional simulator does; the hardware question is ptx-re-blockers B28.

A cluster whose shared memory is more than the 32 KiB of a threadgroup keeps
it in device memory (`LLVMZludaAgxSpillClusterShared`, after the address
spaces are lowered): each such kernel has a pool of 8 slabs of its windows'
size, the first thread of a threadgroup claims a free one (the others wait at
a barrier) and the last thread to return frees it. Every threadgroup memory
access of the kernel, generic ones included, is then a device memory access
at the slab plus its window offset, with device scope fences and atomics; a
threadgroup that finds no free slab waits for one, as with the tensor memory
slabs.

Limitations: at most 1024 threads per cluster; no dynamic shared memory (`extern .shared`) in cluster kernels; the
grid must be a multiple of the cluster (else `CUDA_ERROR_INVALID_CLUSTER_SIZE`);
a function that reads thread ids and is not inlined cannot be shared by kernels
of different cluster shapes; cluster dimensions set at launch
(`cuLaunchKernelEx` attributes) are not supported. `st.async` and
`red.async` store (or update atomically) and complete their bytes on the
destination's mbarrier; `clusterlaunchcontrol.try_cancel` never cancels a
cluster (a valid outcome: the response is 0 and `query_cancel.is_canceled`
false; `.multicast::cluster::all` writes it and completes its 16 bytes in each
CTA of the cluster); `st.bulk` is a memset and `setmaxnreg` a no-op. PTX-formalization
`cluster.ptx`, `cluster-exit.ptx`, `cluster-generic.ptx`,
`mbarrier-cluster.ptx`, `mbar-mc.ptx`, `misc-async.ptx` and `bulk.ptx` pass. On AMD GPUs clusters still have a
single CTA.

## Multicast memory (`multimem`)

There is one GPU, so a multicast object (`cuMulticastCreate`) has a team of
one device (`numDevices` 1, else `CUDA_ERROR_NOT_SUPPORTED`): binding memory
to it (`cuMulticastBindMem`, `cuMulticastBindAddr`) records the physical
allocation, and `cuMemMap` of the multicast handle maps the bound memory at
the range, so that its addresses are that memory's.
`CU_DEVICE_ATTRIBUTE_MULTICAST_SUPPORTED` is 1.

`multimem.ld_reduce`, `multimem.st` and `multimem.red` (every type of the
PTX ISA tables, `.vec`, `.acc::f32` / `.acc::f16`) run in software
(`ptx/lib/zluda_ptx_impl_agx_cvt.c`) over the devices of an address: an
address in a window of the table at 0x700000000000 (window k from
0x700000000000 + (k + 1) * 2^36, entry k of 64 bytes holding the count and up
to 7 buffers) is the same offset in each of its buffers, any other address
the memory of one device. `ld_reduce` loads each device's element and folds
them in device order (each step rounded in the type, or in the `.acc_prec`
type, the fp8 types through f16), `st` stores on each device and `red`
updates each one atomically, in device order. The conformance harness
fills the table for its `.multimem` directive.

## Fabric endpoints (`fabric`)

`fabric.*` runs in software over logical endpoints on the one GPU: entry `le`
(64 bytes) of the table at 0x6ff000000000 holds the device count, the size of
the endpoint's memory and up to 7 device buffers. `try_get`, `try_put`
(`.counted::bytes`, `.cp_mask`), `try_red`, `try_pullred` and `try_atom`
reach the first device, the `.multimem` forms each device in order, and
offsets past the endpoint's size trap. The `.tensor` forms (tile mode,
`.override::global_address`, `.override::global_dim`) copy as
`cp.async.bulk.tensor` with the tensor map's global address replaced by the
endpoint's memory at the offset. Every transfer completes at once, on its
mbarrier: `try_get` and `try_pullred` count bytes, `try_put` and `try_red`
16-byte units, `try_atom` one unit. `fabric.submit` and `fabric.wait` are
no-ops. The conformance harness fills the table for its `.fabric` directive.

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
  `bf16`), also for the operations the optimizer evaluates at compile time:
  their folded NaNs (LLVM's default NaN) are rewritten after optimization, so
  a NaN literal with exactly that default payload (`0f7FC00000`) that is only
  moved, never computed with, is stored as the canonical NaN too.
* Conversions from floating point to integers saturate (NaN gives 0) in both
  modes.
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
approximations; `ex2.approx` and `tanh.approx` also take `.f16x2`, `.bf16` and
`.bf16x2` (`tanh` through f32).

Known differences from the Sail PTX model (`sail/difftest.sh` in
PTX-formalization):
* `.ftz` with `.sat` flushes a negative subnormal result to -0 before
  saturating, so it gives -0 where the model gives +0;
* `min.ftz.f32` of a negative and a positive subnormal gives +0, not -0;
* `atom.add.f32` stores and returns 0x7fc00000 for every NaN result, not the
  operand's payload: the M2 Max's native float atomic add does
  (AGX-Performance-Simulator `tasks/ptx-re-blockers.md`, B24). `min.ftz.f32`
  waits on B11.

`f32` atomic additions without `.ftz` semantics (`atom.add.noftz.f32`, and
`atom.add.f32` and `red.add.f32` on shared memory, which PTX defines without
flushing) keep subnormals: they are compare-and-swap loops whose sum is
computed in `f64` and rounded once to `f32`, as are the `f32` additions of
`cp.reduce.async.bulk`. Vector atomics (`.v2`, `.v4`, `.v8`) apply the
operation per element. `redux.sync` takes `.f32` with `.min` and `.max`, with
`.abs` (of each operand) and `.nan` (a NaN operand gives the canonical NaN),
and `ld.global.nc` takes `.v8`.

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
without strict FP.

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

## Strict instruction mode (`agx-strict`)

`zoc --agx-strict` (and `ZLUDA_AGX_STRICT=1` for `zluda_agx`) compiles for M1 and M2 using only the instruction forms
that Mesa's asahi compiler emits for the same GPU: the LLVM backend's `agx-strict` feature (`llvm/docs/AGXUsage.rst` in
the LLVM fork, "Strict mode"). Code then relies only on encodings that a production driver uses, and the backend
reports any other instruction as an error. Nothing changes in the PTX translation: every AGX intrinsic ZLUDA calls has a
Mesa form. What the backend does differently:

- float atomic adds are compare-and-swap loops, not the M2's native float add;
- 64-bit unsigned atomic min/max use the lock table, not the M2's native form;
- device and threadgroup loads, stores and atomics are always encoded in their long form, as Mesa packs them.

The mode is part of the compilation cache key. The library kernels (`agx_libs`) carry a `CHIP+agx-strict` build of every
M1 and M2 entry, compiled with `#agxir.target<..., features = "+agx-strict">` so that the MLIR `agx` operations take
their portable forms, and `zluda_agx` loads those entries in this mode. The AGX Performance Simulator's
`--agx-strict GPU` (or `AGX_STRICT=GPU`) runs code with the same check, so a test fails on any instruction Mesa would
not emit; `agx_ptx_formalization_check.py --agx-strict` and `test_kernels.py` (for `+agx-strict` entries) use it.

Results (2026-10-21, simulator):
- PTX-formalization with `--agx-strict`, run under `agx_run --agx-strict`: on `agx-g14c` and `agx-g13g`, 282 programs pass
  and all 1,728 checks match with `--strict-fp`, and 1,553 checks match without it. That is the same as without the
  mode, and no program used an instruction outside Mesa's forms.
- `cargo test -p ptx --no-default-features --features agx agx`: 388 of 388 `agx-strict` compilations, the 194 run tests
  that compile, on `agx-g13g` and `agx-g14c`.
- Cost (agx-bench in the LLVM fork): 6.0% more code from the long memory forms, and 0.4% more estimated cycles, all of
  it the float atomic add on M2.

## Checked mode (`agx-checked`)

`zoc --agx-checked` (and `ZLUDA_AGX_CHECKED=1` for `zluda_agx`) builds kernels that check their own memory accesses, as
compute-sanitizer's memcheck does on NVIDIA GPUs: the LLVM backend's `agx-checked` feature (`llvm/docs/AGXUsage.rst`
in the LLVM fork, "Checked mode"). Before each access that could go wrong, the kernel checks

- device memory: that the address is inside the allocation its pointer argument points into, and aligned; a pointer
  loaded from memory (arrays of pointers, linked structures) must point into some live allocation;
- `.shared` memory, static and dynamic: that the address is below the block's threadgroup memory size;
- `.local` memory with a variable offset: that the offset is inside the variable.

Accesses at constant offsets from the same base in the same block share a check. The first bad access of any thread
fills the module's 64-byte error record (the kind, the address and size, a check site, the thread and block) and that
thread returns; every later checked launch of the module returns at once. The kernel does not touch the bad address, so
nothing faults: without checked mode, the same bug reads or writes whatever is mapped there, or faults the command.

`zluda_agx` passes what the kernel needs as hidden arguments after the PTX ones (the `.agx.kernels` record tells
where): the error record, the threadgroup memory size, and for each pointer the kernel bounds-checks (a pointer
parameter, or a pointer inside a `.param` struct) the start and end of the allocation it points into, found with
`cuMemGetAddressRange`'s table or among the modules' global data (`cuModuleGetGlobal`). A pointer into neither gets an
empty range, so any access through it is reported. For pointers loaded from memory, each launch also gets a sorted
table of every live allocation and global data buffer, copied into the launch's staging memory, which the kernel
binary-searches. At the next synchronisation the record is printed, with the launch it came from:

```
zluda_agx: out of bounds store of 4 bytes at 0x10000000080 by thread (32, 0, 0) of block (0, 0, 0)
    at k k.cu:42:7
  in kernel k grid (1, 1, 1) block (33, 1, 1) shared 0 bytes, stream 0, command 7, arguments [0x10000000000, ...]
```

and the context is left with `CUDA_ERROR_ILLEGAL_ADDRESS` (`CUDA_ERROR_MISALIGNED_ADDRESS` for a misaligned access),
returned by every call until `cuCtxDestroy`, which also clears the records. A site names the function, file, line and column
of each frame, innermost (inlined) first, from the PTX `.file` and `.loc` directives (`nvcc -lineinfo` emits them),
which checked mode turns into LLVM line-table debug locations; without them a site names only the kernel. The line
information changes no code: the 194 PTX run tests annotated with a `.loc` before every instruction compile to the
same machine code with and without it. The mode is part of the compilation cache key; a library fat object
is used in its `CHIP+agx-checked` build if it has one and unchecked otherwise.

Cost (agx-bench in the LLVM fork): about 1.5 times the estimated cycles and 5 times the code; most of it is a fixed part
per kernel (the report and the entry check). Validation (simulator):
`llvm/utils/agx-checked/zluda_tests.py` compiles PTX kernels with `zoc --agx-checked` on `agx-g14c` and on `agx-g13g`
with `--agx-strict`, and checks that each bug is reported with the right kind, address, thread and block, that correct
kernels report nothing and compute what their unchecked builds compute, and that unchecked builds miss the bugs (32
cases, among them a store inside an inlined `.func` reported as `put put.h:3:7` called from `k inl.cu:12:5`, and 7
controls pass). It is the `zchecked` stage of `llvm/utils/agx-mesa-isa/tools/check-all.sh`.

### Fault dumps

A command that faults (both of its timestamps still 0) is reported with its launch record (kernel, grid, block, dynamic
shared memory, stream, command and argument bytes), and the error (`CUDA_ERROR_LAUNCH_FAILED`) is sticky until
`cuCtxDestroy`, as in CUDA. Faults in earlier commands of the same queue are found too, not just in the one waited for.
With `ZLUDA_AGX_DUMP=DIR`, the launch is also saved to `DIR/<pid>-<command>` for replaying in the AGX Performance
Simulator:

- `code.bin`, the kernel's code;
- `m<i>.bin`, the memory the launch can reach, listed with its GPU address in `regions.txt`: the allocations its
  arguments point into, the module's global data, the parameter block and the error record. It is the memory as the
  fault left it, not as the launch found it;
- `replay.sh`, which runs the launch in `agx_run` (`$AGX_RUN` or `agx_run` on the `PATH`) with the same uniforms,
  grid and threadgroup memory, and leaves each region's final contents in `m<i>.out`.

Kernels that use stack memory replay too: `agx_run` gives each thread its own stack, as the driver's scratch memory does (the stack size is noted in `replay.sh`).

When the module has a DWARF line table (checked mode, or `ZLUDA_AGX_LINEINFO=1`, which compiles with `zoc
--lineinfo` to keep the PTX `.loc` positions as nvcc's `-lineinfo` does), the dump also holds `module.o` and
`replay.sh` passes `--lines module.o`: a fault in the replay then prints the source line of the faulting
instruction (`"source":"file:line:column"`). The objects also work with `llvm-objdump -d -l`, `llvm-dwarfdump` and
`llvm-addr2line`.

### Launch-time specialization

A kernel of a PTX module launched 3 times with the same scalar arguments (4 bytes or smaller) and block size is
compiled once more in the background with them as constants, and later launches with those arguments run that code
(`specialize.rs`): `ld.param` of a `.u32`/`.s32`/`.b32`/`.f32` parameter becomes a `mov` of its value and
`mov.u32 %r, %ntid.*` a `mov` of the block's size, in a copy of the module that holds only that kernel. Divisions by
the arguments become multiplications or shifts, loops over them get a known trip count, bounds checks against them
fold: on Geekbench 7's GPU workloads the GPU time drops 17% (gemm -33%, im2col -62%, depthwise convolutions -55%).

The specialized code computes what the generic code computes, bit for bit: `div.approx.f32` uses the hardware's
reciprocal (`llvm.agx.rcp.f32`, as `rcp.approx`), which LLVM does not fold. Modules with global or constant data, a
`malloc` heap or the clock are not specialized (their copy would have its own), nor are launches in checked mode or
captured ones. Compilations go through the compilation cache. `ZLUDA_AGX_SPECIALIZE=0` turns this off;
`ZLUDA_AGX_LOG=1` prints each specialization when it is ready. `agx_specialize PTX KERNEL --block x,y,z ARG...` prints
the PTX that a launch with these argument bytes (hex) would compile.

### Launch profiles and captures

`ZLUDA_AGX_PROFILE=FILE` appends one JSON line per finished launch: the kernel, grid, block, dynamic shared memory,
stream, command number, argument bytes (hex), the GPU time from the firmware timestamps (`ns`) and the Unix time in
milliseconds when the launch was seen to finish (`t`). Launches are seen when their command is waited for.

`ZLUDA_AGX_CAPTURE=DIR` saves launches as they are about to run, after waiting for the earlier work: the files of a
fault dump (above), with the memory as the launch finds it, the register allocation (`regs.txt`, in 16-bit halves) and
the command number (`command.txt`, the launch's line in a profile of the same run), in `DIR/<pid>-cap<n>-<kernel>`.
`ZLUDA_AGX_CAPTURE_KERNELS` names the kernels (comma separated, `all` by default), `ZLUDA_AGX_CAPTURE_SKIP` how many
launches of each pass first (the first ones may run at a low GPU clock) and `ZLUDA_AGX_CAPTURE_MAX` how many are then
saved (1). With both variables set, the AGX Performance Simulator's `agx_dispatch` can be compared with the hardware on
real launches with their real data.

`ZLUDA_AGX_LLVM_ARGS` passes further options to LLVM in `zoc` and in the driver (part of the compilation cache key):
backend experiments such as `-agx-threadgroup-unroll=4` or `-agx-fold-mode-phis=false`.

`agx_bench_kernel MODULE KERNEL --grid ... --block ... ARG...` times one kernel the way an application keeps the GPU
busy (launches queued back to back between two events) and prints a hash of each buffer afterwards, so that two builds
of a kernel, for example an edited and reassembled `.asm`, can be checked to compute the same.

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

Kernels with statements that ZLUDA's front end does not support are
skipped, together with the checks that follow their launch. Of the 1729 checks of the 286 programs (2026-10-20,
agx-g14c), 1728 run. With `--strict-fp` all 1728 match the formal semantics, and 282 programs pass completely.
Without it 1553 match. The other 175 are the floating point differences listed above:
- denormal `f32` operands and results;
- NaN bit patterns;
- `-0.0` under `.sat`;
- `div` and `sqrt` precision.

The harness lowers the model's internal `coll_test.sum` / `coll_test.first` (used by `collective.ptx`) to
`redux.sync.add` and a shuffle from the lowest lane of the mask.

A full run takes about 50 minutes with `-j 4` (most of it compiling the texture and `tcgen05` tests).

All 74 programs of `ldmatrix`, `stmatrix`, `mma`, `mma.sp` and `wmma`
(`ldm-*`, `ldm2-*`, `mma-*`, `mma-bs-*`, `sp-*`, `wmma-*`) match the formal
semantics, with and without `--strict-fp`.

## The CUDA driver on Asahi (`zluda_agx`)

`zluda_agx` builds `libnvcuda_agx.so`, the CUDA driver API on Apple GPUs. It
exports every CUDA driver function (unimplemented ones return
`CUDA_ERROR_NOT_SUPPORTED`) and needs no ROCm: it uses `agx_runtime` for
direct DRM submission, and `cuda_types` without its `rocm` feature.

```
ZLUDA_LLVM_BUILD_DIR=/path/to/build cargo build -p compiler --no-default-features --features agx
cargo build -p zluda_agx
ZLUDA_AGX_ZOC=target/debug/zoc target/debug/agx_cuda_smoke
```

* Memory: the GPU shares system memory, and allocations are mapped on the CPU
  at their GPU address when that range is free (it normally is: GPU data
  addresses start above 64 GiB). So `cuMemAllocManaged` and `cuMemHostAlloc`
  return one pointer for both sides (`cuMemHostGetDevicePointer` returns it
  unchanged); if the range is taken, managed allocation fails with
  `CUDA_ERROR_NOT_SUPPORTED` and host allocations fall back to ordinary
  memory the GPU cannot address. The CPU must not touch managed memory while
  kernels using it run (`CONCURRENT_MANAGED_ACCESS` is 0); prefetch and advice
  are no-ops. `cuMemAllocPitch` aligns rows to 128 bytes; 2D copies and fills,
  `cuMemGetAddressRange`, peer copies and stream-ordered allocation
  (`cuMemAllocAsync`, which allocates at once, and `cuMemFreeAsync`, which
  waits for all work) are supported. `cuMemHostRegister` is not: Asahi cannot
  map existing user memory into the GPU.
* Scratch: kernels that spill or use `.local` memory get per-thread stack
  memory as Mesa sets it up (AGX Performance Simulator
  `tasks/ptx-re-blockers.md`, B1, measured on the M2 Max): each command that
  needs it carries Mesa's helper program (`libagx_helper`, compiled by Mesa
  25.3.6 and shipped as `agx_runtime/src/data/libagx_helper.bin`), which
  hands out blocks of one scratch buffer per address space laid out like
  Mesa's (128 SIMD-groups per core), and the kernel's Registers word carries
  its scratch size bucket. The buffer grows to the largest kernel's stack
  size (for example 57 MiB for 320 bytes per thread on 38 cores, 912 MiB for
  4 KiB) and is kept until the context is destroyed.
* `cuMemGetInfo` reports the system's available memory (MemAvailable) as free and its memory as total, both
  within the process's cgroup limit (`memory.max`), since the GPU shares system memory and the UAPI reports no
  figures of its own (ptx-re-blockers B39). Streams have one priority (`cuCtxGetStreamPriorityRange` gives [0, 0]:
  the queue priorities change nothing, B38).
* IPC and host memory (dma-buf, ptx-re-blockers B36): `cuIpcGetMemHandle` exports the allocation's object as a
  dma-buf and puts the process id and the descriptor in the handle; `cuIpcOpenMemHandle` in another process takes
  the descriptor with `pidfd_getfd` (the same user, and `kernel.yama.ptrace_scope` 0 or a parent process) and binds
  the object; `cuIpcCloseMemHandle`. `cuMemExportToShareableHandle` / `cuMemImportFromShareableHandle` give and take
  POSIX file descriptors of physical allocations. `cuMemHostRegister` maps a GPU object over the registered pages
  with their contents (the UAPI cannot import a user pointer), so the CPU keeps its pointer and
  `cuMemHostGetDevicePointer` gives the GPU address; `cuMemHostUnregister` puts ordinary memory back. Whole 16 KiB
  pages are registered. IPC events (`cuIpcGetEventHandle`) are not supported.
* Virtual memory management: `cuMemAddressReserve` / `Free` reserve GPU addresses (the address hint is not
  followed), `cuMemCreate` makes a physical allocation (an unbound GEM object), `cuMemMap` / `Unmap` bind parts
  of it in a reserved range (one allocation may be mapped at several addresses), `cuMemRelease`,
  `cuMemRetainAllocationHandle` and `cuMemGetAllocationPropertiesFromHandle`; the granularity is the 16 KiB GPU
  page, and `cuMemSetAccess` accepts any access (mappings are always readable and writable).
* Modules: `cuModuleLoadData` takes PTX text, a fat binary (its PTX for the
  newest architecture is used, portable PTX before architecture-specific PTX such as `sm_90a` or `sm_100f`; SASS-only fat binaries cannot load) or an AGX
  object. PTX is compiled by `zoc` (found through `ZLUDA_AGX_ZOC`, next to the
  executable or on `PATH`) and the result is cached in `ZLUDA_CACHE_DIR`
  (default `$XDG_CACHE_HOME/zluda/agx` or `~/.cache/zluda/agx`; disabled by
  `ZLUDA_CACHE_DISABLE`). Entries are keyed by the PTX, the chip and the `zoc`
  binary, so a rebuilt compiler does not reuse old code.
* Launches: `cuLaunchKernel` packs `kernelParams` with the argument layout of
  the object's `.agx.kernels` section. Most parameters travel in uniform
  registers; the others go in a parameter block.
* Streams are in-order queues on timeline syncobjs. Events record a queue's
  last fence and time with the firmware's command timestamps. The firmware
  signals a faulted compute command with success but writes neither timestamp,
  so a command whose timestamps are both still 0 is returned as
  `CUDA_ERROR_LAUNCH_FAILED`, with the launch that faulted printed; the error
  is sticky until `cuCtxDestroy` (see "Fault dumps").
* Every launch's command stream invalidates the USC (shader) cache, as Mesa's
  does: each launch stages its pipeline words in the USC ring, and with 45 or
  more launches queued on a stream the GPU can run stale words cached while
  earlier commands ran (simulator `tasks/zluda-issues.md`, Z2, which corrects
  B8). The invalidate costs about 1.8 us per launch; `ZLUDA_AGX_USC_BARRIER=0`
  emits it only on a queue's first command after new code was loaded, which
  is safe only with short queues.
* `cuLaunchCooperativeKernel` and `cuLaunchKernelEx`'s cooperative attribute
  take grids up to the threadgroups resident together on the device (the
  co-residency limit `cuOccupancyMaxActiveBlocksPerMultiprocessor` reports,
  times the cores; B5), and return `CUDA_ERROR_COOPERATIVE_LAUNCH_TOO_LARGE`
  beyond. `cuLaunchKernelEx`'s cluster dimension must be the kernel's compiled
  `.reqnctapercluster` (or 1); the scheduling preference is ignored.
* Kernels have no run time limit (`CU_DEVICE_ATTRIBUTE_KERNEL_EXEC_TIMEOUT` is
  0): one command ran 79.9 s correctly on the M2 Max (simulator
  `tasks/ptx-re-blockers.md`, B6), and a synchronisation waits as long as the
  kernel runs (`ZLUDA_AGX_WAIT_MS` sets a limit). The firmware ends a
  deadlocked command after about 0.5 s, which is reported as a launch failure.
* At every synchronisation, device `printf` output is printed, and a failed
  device `assert` is returned as `CUDA_ERROR_ASSERT`.
* There is one device (the render node is `$ZLUDA_AGX_NODE` or the first one
  that answers the Asahi parameter query). Contexts are real: each one
  `cuCtxCreate` makes has its own GPU address space (VM) and owns its memory,
  modules, streams, events, graphs, limits and sticky error, all freed by
  `cuCtxDestroy`; every thread has its own stack of current contexts
  (`cuCtxPushCurrent` / `cuCtxPopCurrent` / `cuCtxSetCurrent`). The primary
  context (`cuDevicePrimaryCtxRetain`, the CUDA runtime's) is used by a thread
  with no current context; `cuDevicePrimaryCtxReset` (`cudaDeviceReset`) waits
  for its work, frees everything in it and starts it afresh. How the firmware
  schedules several VMs is not measured yet (simulator
  `tasks/ptx-re-blockers.md`, B38, B42-B44).

## cuBLAS (`zluda_blas_agx`)

`libcublas` for zluda_agx, on the `agx_libs` engine (shared with cuBLASLt).
Its kernels are MLIR (`agx_libs/kernels/*.mlir`:
standard dialects and `agx` operations, no chip names), compiled at build time
for every G13/G14 chip by the LLVM fork's
`mlir/utils/agx-kernels/agx-build-fatbin.py` into one fat AGX object
(`ZAGXFAT1`). `cuModuleLoadData` picks the entry for the device's chip
(`CUDA_ERROR_NO_BINARY_FOR_GPU` if there is none). The library calls the
driver API of the process's `libcuda.so.1` (`ZLUDA_AGX_LIBCUDA` overrides
it), as NVIDIA's does.

* Handles, streams, pointer modes (host and device alpha / beta), math modes
  (all compute in f32) and workspaces (unused).
* `cublasSgemm`, `cublasHgemm`, their strided batched forms, and
  `cublasGemmEx` / `cublasGemmStridedBatchedEx` with f32 A, B and C or f16 A
  and B with f16 or f32 C. All accumulate in f32, including the 16-bit
  computation types (alpha and beta are then f16, as in cuBLAS); f16 results
  are rounded once at the end. Without transposes, with m and n multiples of 64, k a multiple
  of 8, even leading dimensions and 8-byte aligned matrices, the column-major
  product runs on the row-major tile kernel `agx_sgemm_tile64` (A and B
  swapped): 64 x 64 tiles per threadgroup of 2 x 2 SIMD-groups built on
  `agx.mma_tile` (`agx_libs/kernels/sgemm.mlir`; the f16 forms are in
  `hgemm.mlir`). Everything else runs on a generic kernel, one
  thread per element with arbitrary strides.
* `cublasDgemm`, `cublasDgemmStridedBatched`, and `cublasGemmEx` /
  `cublasGemmStridedBatchedEx` with `CUDA_R_64F` A, B and C and
  `CUBLAS_COMPUTE_64F` (or `_PEDANTIC`): always on the generic kernel
  `agx_dgemm_generic`, accumulating in f64. AGX has no f64 unit, so the
  arithmetic is software double precision and much slower than f32.
* Level 1 and 2 in single precision, with their `_64` forms
  (`agx_libs/kernels/blas.mlir`): `cublasSaxpy`, `Sscal`, `Scopy`, `Sswap`,
  `Sdot`, `Sasum`, `Snrm2`, `Isamax`, `Isamin`, `Sgemv` and `Sger`, with
  negative increments as BLAS defines them. Reductions are deterministic:
  1024 threads keep fixed partials that one thread combines in order. `nrm2`
  keeps a scaled sum of squares (LAPACK's `slassq`), so it does not overflow
  when the squares would. In host pointer mode a reduction waits for the
  handle's stream.
* Level 3 beyond GEMM in single precision, with their `_64` forms:
  `cublasSgeam`, `Sdgmm`, `Ssyrk`, `Ssymm`, `Strmm` (in place when C is B,
  through a copy) and `Strsm` (one thread per column of B, or per row for the
  right side, solving by substitution), and `cublasSgemmBatched` (the pointer
  arrays are read back once the stream is idle, then one GEMM per entry).
* All kernels are tested on the simulator for agx-g13g and agx-g14c
  (`agx_libs/kernels/test_kernels.py`: tile sizes, all four transpose
  combinations, `beta = 0` with NaN in C, f16 and f32 C).

Building needs `ZLUDA_LLVM_BUILD_DIR` (mlir-opt, mlir-translate, opt and llc
of the LLVM fork). The translated kernels go through `opt -O3` before `llc`,
which takes the 64 x 64 GEMM tile from about 11,800 to 6,400 cycles in the
simulator's cost model.

## cuBLASLt (`zluda_blaslt_agx`)

`libcublasLt` on the same engine. `cublasLtMatmul` computes
`D = alpha op(A) op(B) + beta C` for column-major layouts with the type
combinations of cuBLAS above, strided batches (a batch count of 1 on an
operand broadcasts it), host or device alpha / beta, and the epilogues bias,
ReLU, ReLU + bias, GELU and GELU + bias (`agx_libs/kernels/epilogue.mlir`,
bias of D's type, optionally with a batch stride). When C and D are
different buffers, C is first copied into D on the stream (they must then
have the same layout). `cublasLtMatmulAlgoGetHeuristic` returns one
algorithm, needing no workspace, or `CUBLAS_STATUS_NOT_SUPPORTED` for what
the kernels do not do (row-major or tiled orders, fp8 / int8 / bf16, scale
pointers, auxiliary epilogue outputs and gradients); preferences are
accepted and ignored.

## cuDNN (`zluda_dnn_agx`)

`libcudnn.so.9` with the legacy convolution API, on the convolution kernels
in `agx_libs/kernels/conv.mlir` (one source for every chip, compiled into a
fat AGX object like the GEMM kernels). Supported: handles and streams; 4-D
tensor descriptors with any strides (NCHW or NHWC packing, or explicit
strides); packed NCHW filters; 2-D convolution descriptors with padding,
strides, dilations, groups and both modes (cross-correlation and the flipped
convolution); and `cudnnConvolutionForward`, `cudnnConvolutionBackwardData`
and `cudnnConvolutionBackwardFilter` for f32 and f16 tensors, accumulated in
f32, with `alpha` / `beta`. The algorithm queries (`Get*_v7`, `Find*`,
`Find*Ex`) report one deterministic algorithm per direction that needs no
workspace. The kernels compute one output element per thread; they are
tested on the simulator for agx-g13g and agx-g14c (36 cases per chip in
`test_kernels.py`).

The other common legacy operations run on `agx_libs/kernels/dnn.mlir`, for
tensors of up to 4 dimensions with any strides, f32 directly and f16 or bf16
through packed f32 copies (converted in and out by `agx_dnn_to_f32` /
`agx_dnn_from_f32`):

* activations (`cudnnActivationForward` / `Backward`: sigmoid, ReLU, tanh,
  clipped ReLU, ELU, identity and swish, with their descriptors);
* `cudnnAddTensor` (broadcasting dimensions of 1), `cudnnSetTensor` and
  `cudnnScaleTensor`;
* softmax forward and backward (fast, accurate and log; instance and channel
  modes), one thread per softmax row;
* 2-D pooling forward and backward (max, average including or excluding
  padding). Max-pooling gradients go to the first maximum of each window, the
  deterministic choice, for both max modes;
* batch normalization (`ForwardInference`, `ForwardTraining`, `Backward`,
  `cudnnDeriveBNTensorDescriptor`), spatial and per-activation. The batch
  statistics take two passes (the mean, then the variance), and the running
  variance is the unbiased one, as in cuDNN.

These kernels are tested on the simulator for agx-g13g and agx-g14c.

The backend (graph) API (`cudnnBackend*`, what cudnn-frontend and PyTorch
use) runs operation graphs of 2-D convolutions (forward, backward data and
backward filter, any group count, symmetric padding), matmuls (2-D or batched
3-D, broadcast batches), pointwise operations (every mode but atan2 and index
generation: arithmetic, math functions, activations and their backward forms,
comparisons, logic and binary select, with broadcasting) and reductions, on
tensors of up to 4 dimensions in f32, f16 or bf16 (`zluda_dnn_agx/src/imp/graph.rs`).
Finalizing a graph takes a snapshot of it; heuristics return one engine
configuration when every operation is supported and none otherwise, so
cudnn-frontend falls back or reports the graph unsupported. An execution plan
runs the operations in dependency order in f32 on packed tensors: packed f32
tensors are used in place, everything else (f16 and bf16 tensors, other
strides, virtual tensors, by-value scalars) through f32 copies in the
workspace the plan reports. The pointwise and reduction kernels are tested
for every mode and operator against Python references. Normalization,
attention (SDPA), resampling and the statistics operations of the graph API,
RNNs, LRN and dropout are not implemented yet.

## cuFFT (`zluda_fft_agx`)

`libcufft` on `agx_libs/kernels/fft.mlir` (one source for every chip).
Single and double precision (C2C / Z2Z, R2C / D2Z, C2R / Z2D): complex to
complex transforms of rank 1 to 3 (rank 1 also
strided and batched through `cufftPlanMany`, with the same input and output
layout), and real to complex / complex to real transforms of rank 1,
batched, in or out of place. Power-of-two lengths use radix-2 Stockham passes
alternating two work buffers; other lengths use a direct transform. The
twiddles are computed on the host in double precision and uploaded with the
plan. Double precision runs copies of the same kernels built for `f64`
(generated by `agx_libs/build.rs`, named `<kernel>_f64`), in the backend's
software double precision, with double precision twiddles. The plan
allocates its own work buffers (`cufftSetWorkArea` is accepted and unused).
Like cuFFT, inverse transforms are not normalized. Multi-dimensional real
transforms, callbacks and the Xt API return `CUFFT_NOT_SUPPORTED`. The kernels are tested on the simulator
for agx-g13g and agx-g14c, and the plans' launch sequences (ping-pong
buffers, in-place copies, the passes of each dimension) are checked in unit
tests against a direct DFT.

## cuSPARSE (`zluda_sparse_agx`)

`libcusparse` on `agx_libs/kernels/sparse.mlir` and `sparse_ix.mlir` (one
source for every chip; `agx_libs/build.rs` instantiates `sparse_ix.mlir` for
32- and 64-bit indices and adds f64 copies of every kernel). The generic API
with f32 or f64 values (all operands of one type) and 32- or 64-bit indices
(the row offsets and the columns of one type), zero- or one-based. CSC
matrices (`cusparseCreateCsc`) work wherever CSR ones do, as the CSR matrix
of their transpose:

- `cusparseSpMV` on CSR and COO matrices, non-transposed or transposed, and
  `cusparseSpMM` on CSR matrices with `opA` non-transposed and `opB` either
  way, on column- or row-major dense matrices. Non-transposed CSR SpMV and
  SpMM use one thread per output element; the transposed and COO products
  scale `y` by `beta` first and then add with atomic adds.
- Sparse vectors (`cusparseCreateSpVec`): `cusparseSpVV` (256 partial sums in
  its workspace, added in order, so the result is deterministic),
  `cusparseAxpby`, `cusparseGather`, `cusparseScatter` and `cusparseRot`.
- Conversions: `cusparseSparseToDense` (CSR and COO), `cusparseDenseToSparse`
  to CSR (the analysis counts the nonzeros of each row, scans them into the
  row offsets and sets the matrix's nnz; `cusparseCsrSetPointers` then gives
  the columns and values), and `cusparseCsr2cscEx2` (32-bit indices; any value
  type of 4, 8 or 16 bytes, or `CUSPARSE_ACTION_SYMBOLIC`): columns are counted
  and filled with atomics, then each column is sorted, so the row indices of a
  column are in order like cuSPARSE's.
- `cusparseSpGEMM` (workEstimation, compute, copy) for `C = alpha A B` with
  CSR matrices, non-transposed, `beta` 0: the work estimation counts each
  row's products, the compute step forms them sorted by column with equal
  columns added (one thread per row) and sets C's nnz, and the copy writes C.
- `cusparseSDDMM` on CSR output, `opA` and `opB` either way, one thread per row.
- `cusparseSpSV` and `cusparseSpSM` on CSR matrices with the fill mode and
  diagonal type of `cusparseSpMatSetAttribute`, `opA` non-transposed or
  transposed (and `opB` either way for SpSM): one thread per right-hand side
  substitutes row after row, so a single SpSV runs on one thread.

Only SpVV, Csr2cscEx2 and SpGEMM need external buffers. Handles, streams, host and
device pointer modes and the legacy matrix descriptor are supported. Other
functions (SpGEMM reuse, BSR and the blocked formats, the legacy level 2 and 3 routines,
complex and f16 values) return `CUSPARSE_STATUS_NOT_SUPPORTED`.

## cuRAND (`zluda_rand_agx`)

`libcurand`'s host API on `agx_libs/kernels/rand.mlir`. Every pseudo-random
generator type is Philox4x32-10 (block `b` of a sequence is
`philox(counter (b, 0), key seed)`); each call starts at the next whole block
after the words earlier calls used, so results are reproducible for a seed and
a sequence of calls, but they are not NVIDIA's sequences. `curandGenerate`,
`curandGenerateUniform(Double)`, `curandGenerateNormal(Double)`,
`curandGenerateLogNormal(Double)` and `curandGeneratePoisson` use cuRAND's
documented conversions: uniform values in (0, 1], Box-Muller normals (an even
count), and Poisson values from exponential arrivals below lambda 64 and the
normal approximation above. The f64 kernels compute their logarithm, sine,
cosine and exponential with polynomials (the backend has no f64
transcendentals); the f32 ones use the hardware's. Host generators
(`curandCreateGeneratorHost`) compute the same words on the CPU. Quasi-random
generators (Sobol) and the discrete distributions of the device API return
`CURAND_STATUS_TYPE_ERROR`; the device API (`curand_kernel.h`) is header code
in the application's PTX and needs no library.

## cuSOLVER (`zluda_solver_agx`)

`libcusolver`'s dense LU, Cholesky and QR routines on
`agx_libs/kernels/solver.mlir`, in single and double precision:
`cusolverDn{S,D}getrf` / `getrs` (partial pivoting; `devIpiv` is required),
`potrf` / `potrs` (either triangle), `geqrf`, `ormqr` (either side, `N` or `T`),
`orgqr`, `gesvd` (m >= n; `jobu` `A` only for square matrices) and `syevd`, with
their `_bufferSize` queries. The factorizations run one
column at a time — a single-thread pivot or reflector step, then a parallel
update of the rest of the matrix — and the solves run one thread per
right-hand side. `devInfo` reports the first zero pivot (getrf) or the first
non-positive leading minor (potrf) as LAPACK does. `gesvd` and `syevd` use
one-sided Jacobi: each round rotates disjoint column pairs in parallel, and
sweeps repeat until one rotates nothing (`devInfo` 1 if 60 sweeps do not get
there); `syevd` runs it on `A + |A|_F I`, whose singular vectors are A's
eigenvectors, and takes the eigenvalues as Rayleigh quotients. The columns of U
for zero singular values are left zero. The other eigen routines, the 64-bit (`cusolverDnX*`) API, batched and sparse cuSOLVER are not
exported.

## NVRTC and nvJitLink (`zluda_rtc_agx`)

`libnvrtc` compiles CUDA C++ to PTX with the system's clang (`clang` on `PATH`,
or `ZLUDA_NVRTC_CLANG`; it needs the NVPTX target, which distribution builds
have), and zluda_agx compiles that PTX like any other module. Instead of the
CUDA toolkit's headers, clang's CUDA headers are completed by a prelude
(`zluda_rtc_agx/src/include`): the qualifiers and built-in variables, vector
types, atomics, warp intrinsics, and libdevice's `__nv_*` math functions
written in portable C++. Those are tested against the host's libm
(`cargo test -p zluda_rtc_agx`: 1–4 ulp for the elementary functions in single
and double precision) and on the GPU through the simulator
(`zluda_rtc_agx/tests/agx_math.py`); the `__sinf`-style intrinsics and
`--use_fast_math` use the hardware's approximations. Bessel, gamma, `erfinv`
and the SIMD video functions are not implemented: a program that calls one
fails to compile and the log names it. Headers passed to
`nvrtcCreateProgram`, name expressions and lowered names (read from the PTX),
and the usual options (`--gpu-architecture`, `-D`, `-I`, `--std`,
`--use_fast_math`, `-ftz`, `-fmad`, `-default-device`, `-lineinfo`, `-rdc`) are
supported; others are ignored with a note in the log. `nvrtcGetCUBIN` returns
the PTX, which zluda_agx loads wherever a cubin is expected.

`libnvJitLink` passes its inputs (PTX, fat binaries, AGX objects) to the
driver's `cuLink*`, which links several PTX modules into one AGX object; the
versioned entry points of CUDA 12.0 to 12.9 (`__nvJitLinkCreate_12_5`, ...) are
exported too. LTO IR is not supported.

## NCCL (`zluda_nccl_agx`)

`libnccl` for the one GPU of an Asahi machine: every communicator has one rank
(`ncclCommInitRank` with `nranks` 1, `ncclCommInitAll` with one device,
`ncclCommSplit`; NCCL itself refuses two ranks on one device, and other counts
return `ncclInvalidUsage`). On one rank each collective (AllReduce, Reduce,
ReduceScatter, Broadcast, Bcast, AllGather) copies the send buffer to the
receive buffer on the stream, nothing when they are the same; `ncclAvg`
divides by one, and `ncclRedOpCreatePreMulSum` operators scale the result (f32
and f64). Sends and receives to rank 0 pair up in order. `ncclMemAlloc`,
buffer registration, groups and the asynchronous-error query are supported.

## Building a release for Asahi

```
ZLUDA_LLVM_BUILD_DIR=/path/to/llvm-agx-build cargo build --release -p compiler --no-default-features --features agx
ZLUDA_XTASK_AGX=1 cargo xtask --release      # zluda_agx and the *_agx libraries (NVML, cuBLAS, cuBLASLt, cuDNN, cuFFT, cuSPARSE, cuRAND, cuSOLVER, NVRTC, nvJitLink, NCCL), with their library symlinks
```

`ZLUDA_XTASK_AGX` selects the packages marked `agx = true` in their `[package.metadata.zluda]` instead of the
HIP-based ones; the two sets ship libraries with the same names. Put `target/release/zoc` next to the application
or point `ZLUDA_AGX_ZOC` at it, and run the application with `LD_LIBRARY_PATH=target/release`.

## Support matrix (Asahi Linux, M1 / M2 family)

| Area | Status |
| --- | --- |
| Chips | G13 (M1 family) and G14 (M2 family) code generation; G14C (M2 Max) is the measured target. Apple9 (M3 and later) is not supported by the backend. |
| PTX | 194 of 195 ZLUDA run tests compile on their own, and `extern_func` compiles when linked (`cuLink*`). PTX-formalization (PTX ISA 9.4 suite, 2026-10-22, agx-g14c): 268 of 413 files pass and 1677 of 1920 checks match the formal semantics with `--strict-fp` (251 files, 1546 checks without). The rest are mostly sm_90a / sm_100a features (tcgen05 and wgmma 9.4 forms, new low-precision formats, TMA and mbarrier additions), f64 NaN payloads and newer syntax. |
| Warp-level and async | `mma`, `mma.sp`, `wmma`, `ldmatrix` / `stmatrix`, `wgmma`, `tcgen05` and the tensor memory accelerator (`cp.async.bulk.tensor`) are emulated; `cp.async`, mbarriers and thread block clusters (up to 1024 threads) run on ordinary instructions and threadgroup memory |
| Textures and surfaces | sampled in software, `tex.1d` / `tex.2d` of linear one-level texture objects on the texture unit outside `--strict-fp` (simulator only): `tex`, `tld4`, `txq`, `suld` / `sust` / `sured`, `suq`, `istypep`; CUDA arrays, mipmapped arrays, texture and surface objects. No legacy texture references, border colours or per-dimension address modes |
| Floating point | f32 hardware semantics by default (denormals flushed); `--strict-fp` (strict FP) for exact PTX semantics; f64 in software; f16 / bf16 / fp8 conversions |
| Atomics | 32-bit native; f32 add and u64 min/max without result native on G14; other 64-bit and 128-bit through a lock table |
| Memory model | fences of every scope lower to measured barrier sequences |
| Driver API | devices, contexts, memory, copies, modules (PTX, fat binaries, AGX objects), linking (`cuLink*`), CUDA arrays, launches, streams, events, graphs (stream capture), occupancy, `cuGetProcAddress`, export tables. Other functions return `CUDA_ERROR_NOT_SUPPORTED` (see `ZLUDA_AGX_LOG`). |
| Device code features | `printf`, `assert` (`CUDA_ERROR_ASSERT`) |
| Checked mode | `zoc --agx-checked` / `ZLUDA_AGX_CHECKED=1`: every memory access bounds- and alignment-checked, the first bad one reported with its thread and launch (see "Checked mode"); faulted launches saved for `agx_run` with `ZLUDA_AGX_DUMP` |
| Libraries | cuBLAS: f32, f16 and f64 GEMM (f64 in software), f32 and f64 level 1, gemv / ger, geam / dgmm / syrk / syr2k / symm / trmm / trsm and batched GEMM, and complex (C / Z) level 1, gemv / hemv / ger / her / her2, gemm / hemm / symm / herk / her2k / syrk / syr2k (`zluda_blas_agx`); cuBLASLt matmul with bias / ReLU / GELU epilogues (`zluda_blaslt_agx`); cuDNN 2-D convolutions, activations, softmax, pooling and batch normalization (`zluda_dnn_agx`); cuFFT single and double precision (`zluda_fft_agx`); cuSPARSE SpMV, SpMM, SpGEMM, sparse vectors, conversions, SDDMM, SpSV and SpSM in f32 and f64 (`zluda_sparse_agx`); cuRAND's host API on Philox (`zluda_rand_agx`); cuSOLVER dense LU, Cholesky, QR, SVD and symmetric eigenvalues (`zluda_solver_agx`); NVRTC on clang and nvJitLink (`zluda_rtc_agx`); single-rank NCCL (`zluda_nccl_agx`) |
| Not yet | peer access, IPC events, multicast objects, cuSPARSE's blocked formats, cuRAND's quasi-random generators |
| NVML | device count, name, UUID, memory, compute capability; the maximum clock (`nvmlDeviceGetMaxClockInfo`, 1,398 MHz on the M2 Max) and GPU utilisation (`nvmlDeviceGetUtilizationRates`: the share of time since the last call this process's kernels ran, from the firmware's command timestamps; other processes are not seen). Asahi exposes no current clock, temperature or power without root (B25), so those return `NVML_ERROR_NOT_SUPPORTED` |
| Strict instruction mode | `zoc --agx-strict` / `ZLUDA_AGX_STRICT=1`: only the instructions Mesa's asahi compiler emits, on M1 and M2 (see "Strict instruction mode"); PTX-formalization results identical, library kernels built for it |
| Hardware validation | M2 Max (G14C, Asahi), 2026-10-20 and 21, in the AGX Performance Simulator's `tasks/ptx-re-blockers.md` and `tasks/ptx-re-blockers-tracker.md`: the direct-DRM runner, `agx_cuda_smoke` and CUDA vectorAdd pass (B2); ZLUDA's scratch path runs on the GPU (B1, 262,144 threads); the `agx_libs` kernels give the right results in 817 of 819 cases on the GPU (B40; the 2 others are the test runner's missing scratch, Z6); checked mode on the GPU matches the simulator (B37); context teardown leaks nothing (B42, Z1); partial workgroups, the no-shared-memory word and 7-bit shift amounts (B2a, B2c, B47); forward progress and co-residency (B4, B5, B30), command duration (B6), CPU visibility (B7), the whole uniform file (B9), memory ordering per scope (B10), source modifiers and other numeric corners (B11), calls and returns (B14), `simd_matrix_fmadd` (B15), barriers (B16, B28), lane-offset fmadd (B18), fault reporting (B20), `trap` (B21), launch limits (B22), atomic corners (B3, B24), telemetry (B25), register and threadgroup limits (B26), conversions (B27), native texture corners (B29), masked instruction cost (B31), submit cost (B32), vector alignment (B33), stack cost (B34), the absence of a shader clock (B35), dma-buf IPC (B36, B45), VMs and queues (B38, B43), allocation limits (B39), fault isolation (B44) and `rcp` / `rsqrt` error (B46). Open: async copies (B17), a real CUDA program (B23, B41) and the M2 Ultra / G13X items (B19, B24-7) |
