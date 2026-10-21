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
  threadgroup otherwise. `isspacep` is decided at compile time where it can
  be, and at run time otherwise (see below). `prmt` takes every mode
  (`.f4e`, `.b4e`, `.rc8`, `.ecl`, `.ecr`, `.rc16`), and `szext` and `fns`
  are helpers (AGX only: the AMD path reports them as not implemented). The
  3-input `min`/`max` of `f32`, `f16`, `f16x2`, `bf16` and `bf16x2` (sm_100)
  is two 2-input operations.
  `add`, `sub`, `mul` and `fma` of `f32x2` (sm_100, two f32 in a 64-bit
  register) are `<2 x float>` operations that follow the f32 rounding and
  denormal modes, directed rounding included. `fma.relu` of `f16`, `f16x2`,
  `bf16` and `bf16x2` clamps negative results to +0 and keeps NaN (with
  round to nearest; a directed `.relu` fma is not implemented).
  `lop3` (any lookup table, also in a register, and the predicate output
  form `lop3.and`/`lop3.or`) is the OR of the table's minterms.
  `set` of packed integer lanes (`u8x4`, `s8x4`, `u16x2`, `s16x2`) gives
  all-ones lanes where the comparison holds.
  `ptx/src/test/agx_sim/run_sim.py` checks these instructions and `cuLink` on
  the simulator against reference models.
* Not supported yet: the `f32x2` forms of
  instructions other than `add`/`sub`/`mul`/`fma` and `.ftz`/`.sat` of `bf16`
  arithmetic (which PTX does not have).
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
* The mixed-precision forms (`add`/`sub`/`mul`/`fma` of `.f32.{f16,bf16}`,
  `.f32x2.{f16x2,bf16x2}.f32x2`, `.{f16x2,bf16x2}.f32x2.f32x2` and
  `mul.bf16x2.bf16x2.f16x2`) convert their operands exactly and compute in
  `f32` (in `.rz`, then rounded with `.rz`, for 16-bit pair results).
* The PTX ISA 9.x conversions (`.pzo`, `.scaled::n1::ue8m0` and
  `.scaled::n2::ue8m0`, `ue5m3x2` and `s2f6x2`) and `spcompress` /
  `spdecompress` run in software (`ptx/lib/zluda_ptx_impl_agx_cvt.c`), with
  every value held as an integer significand and a power of two.
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
  same step rewrites generic addresses of threadgroup and private memory that
  went through integer arithmetic into pointer arithmetic, and turns those
  only used as integers (compared, masked) into their 32-bit offsets, which
  avoids the tagged form where it is not needed.
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
* `__assertfail` writes CUDA's message (`file:line: function: block: [x,y,z],
  thread: [x,y,z] Assertion `+"`expr`"+` failed.`) as a printf record, sets
  `__zluda_agx_trap_flag` (which `agx_runtime` reports as
  `CUDA_ERROR_ASSERT`), orders its stores with a release fence and executes
  `trap`. On the M2 Max an executed `trap` does nothing (AGX Performance
  Simulator `tasks/ptx-re-blockers.md`, B21): the stores after it land and the
  command ends normally, so the flag word is what reports the assertion, and
  the kernel keeps running after a failed `assert`.
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
rounding mode other than `.rn`, and register selectors of block scaled and
sparse `mma`.

### Fifth-generation tensor core (`tcgen05`)

The tensor memory of a CTA (128 lanes of 512 32-bit columns) is a 256 KiB slab
of device memory: a pool of 4 slabs is a module global
(`ptx/lib/zluda_ptx_impl_agx_cvt.c`). A CTA claims a free slab at its first
`tcgen05.alloc` (the slab's owner word holds the CTA's linear index + 1, found
again by every later instruction), allocates columns first fit from the slab's
map of 32-column granules, and gives the slab back with its last
`tcgen05.dealloc`. A CTA that finds no free slab waits for one, so at most four
CTAs that use tensor memory run at once. The wait cannot deadlock: a CTA that
holds a slab is running, and a resident CTA is not preempted by the ones
spinning (B30), so it reaches its `dealloc` unless it waits on a CTA that has
not started. `alloc` and `dealloc` are done by the
lowest active lane of the warp; `relinquish_alloc_permit`, `fence` and `wait`
do nothing (every operation completes before the next instruction).

* `tcgen05.ld` / `st` (shapes `32x32b`, `16x64b`, `16x128b`, `16x256b`,
  `16x32bx2`, `.pack::16b` / `.unpack::16b`), `ld.red` (min / max of f32 with
  `.abs` / `.NaN`, u32, s32) and `ld.spcompress` are loads and stores of the
  slab words.
* `tcgen05.mma` (kinds `f16`, `tf32`, `i8`, `f8f6f4`, `mxf8f6f4`, `mxf4`,
  `mxf4nvf4`; `.sp`, `.ws` with its zero-column mask, `.ashift`, the
  disable-output-lane mask, `scale-input-d`, `.block_scale` with every scale
  vector size, `.cta_group::2`) is computed by the issuing thread: a loop over
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

All 33 PTX-formalization `tcgen05` programs but `tcg2-1` match the formal
semantics. `tcg2-1` needs 2 x 19 KiB of shared memory per cluster, more than
the 32 KiB of a threadgroup the cluster runs in. A shared memory address
written as a constant in a cluster kernel is the address of the first CTA's
window (addresses taken from shared variables are right in every CTA).

### Warpgroup MMA (`wgmma`)

`wgmma.mma_async` (dense and `.sp`, f16, bf16, tf32, fp8 and integer types) runs
as `mma.sync` tiles. Warp `w % 4` of the warpgroup computes rows
`16w..16w+15` of D as `n / 8` tiles of `m16n8kK`, through one tile helper
that is inlined at each use (AGX has no calls). A comes from the registers or,
like B, from shared memory through its matrix descriptor (start address, LBO,
SBO and swizzle modes 0 to 6, following `ptx-exec-wg.k`), K-major or
transposed with `.trans`. Sparse A in shared memory is the packed
`M x K/2` tile, read in the element order of the `mma.sp` fragment.
`imm-scale-a/b` = -1 negate the elements, and `scale-d` is either an
immediate or a predicate. Every `wgmma` is synchronous on AGX, so
`wgmma.fence`, `commit_group` and `wait_group` compile to nothing. D can have
up to 128 registers (`n256` with `f32`). A vector operand can now hold up to
256 elements. All 34 `wg-*` and `wgsp-*` programs of PTX-formalization
match the formal semantics on agx-g13g and agx-g14c.

### Tensor memory accelerator (`cp.async.bulk.tensor`)

The TMA copies are emulated by the issuing thread, element by element, and
complete at once (the mbarrier transaction completes when the copy is
issued). Supported:

* tile loads, stores and reductions (`add`, `min`, `max`, `inc`, `dec`,
  `and`, `or`, `xor`; signedness and float from the tensor map's element
  type), 1D to 5D, with element strides, out-of-bounds zero or NaN fill and
  the shared-memory swizzles (32B, 64B, 128B, and 128B with 32B, 32B + 8-byte
  flip and 64B atomicity), by the CTA-relative shared address;
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
corners), and the emulation reads the same one. All 28 `tma*` and `tma-*`
programs of PTX-formalization match the formal semantics on agx-g13g and
agx-g14c. The interleaved layouts are plain copies of the dims, as in the
model. `cp.async.bulk.prefetch.tensor` and `.tile::gather4` / `scatter4` are
not supported yet.

### Textures and surfaces

Textures and surfaces are sampled in software (`ptx/lib/zluda_ptx_impl_agx_tex.c`, compiled to the generated
part of `zluda_ptx_impl_agx.ll` by `ptx/lib/gen_agx_tex.py`). AGX's texture unit is not used yet: its descriptors
and sampling rules have not been measured (simulator task B29). The rules are those of PTX-formalization's model:
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
- `tex` (`.base`, `.level`, `.grad`, separate samplers, the residency predicate);
- `tld4`, `txq`;
- `suld.b`, `sust.b` and `sust.p` (formatted), `sured.b` and `sured.p`, each with `.trap` (an out-of-bounds
  access reports `CUDA_ERROR_ASSERT`), `.clamp` and `.zero`;
- `suq`, `istypep`.

The runtime supports:
- CUDA arrays (`cuArrayCreate`, `cuArray3DCreate`, layered and cubemap; `cuMipmappedArray*`);
- copies to and from arrays (`cuMemcpy2D`, `cuMemcpy3D`, `cuMemcpyHtoA` / `AtoH`);
- `cuTexObjectCreate` over arrays, mipmapped arrays, linear and pitch-linear memory;
- `cuSurfObjectCreate` over arrays.

Module `.texref`, `.samplerref` and `.surfref` variables compile: on AGX each holds the 64-bit handle of
a texture, sampler or surface object, which the instructions load. The runtime does not fill them yet.

Not supported:
- the legacy texture reference API (`cuTexRef*`, `cuModuleGetTexRef`), which would fill those variables;
- arbitrary border colours (the border is zero);
- the min/max mipmap level clamps and the level bias of `CUDA_TEXTURE_DESC`;
- per-dimension address modes (the first one applies).

All PTX-formalization texture and surface programs (`tex`, `tex-mip`, `surf`, `istypep`) match the formal
semantics. A texture fetch runs a few thousand instructions, and kernels with many `tex` instructions take
minutes to compile.

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

Limitations: an mbarrier counts at most 4095 pending arrivals and
transaction counts between -2^18 and 2^18 - 1 bytes; phases are only
distinguished by their parity; the `.multicast::cluster::32b` mbarrier
operations (`expect_tx`, `complete_tx`, `arrive` and `arrive.expect_tx`)
apply to the mbarrier at the same offset of each CTA in the mask. Tensor
copies (`cp.async.bulk.tensor`) are not supported. `ptx/tools/agx_tests/async.ptx` checks the bulk copies, the
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

Limitations: at most 1024 threads per cluster and 32 KiB of shared memory per
cluster; no dynamic shared memory (`extern .shared`) in cluster kernels; the
grid must be a multiple of the cluster (else `CUDA_ERROR_INVALID_CLUSTER_SIZE`);
a function that reads thread ids and is not inlined cannot be shared by kernels
of different cluster shapes; cluster dimensions set at launch
(`cuLaunchKernelEx` attributes) are not supported. `st.async` and
`red.async` store (or update atomically) and complete their bytes on the
destination's mbarrier; `clusterlaunchcontrol.try_cancel` never cancels a
cluster (a valid outcome: the response is 0 and `query_cancel.is_canceled`
false); `st.bulk` is a memset and `setmaxnreg` a no-op. PTX-formalization
`cluster.ptx`, `cluster-exit.ptx`, `cluster-generic.ptx`,
`mbarrier-cluster.ptx`, `mbar-mc.ptx`, `misc-async.ptx` and `bulk.ptx` pass. On AMD GPUs clusters still have a
single CTA.

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
* `atom.add.f32` returns the canonical NaN, not the operand's payload. These
  last two wait on hardware answers (AGX-Performance-Simulator
  `tasks/ptx-re-blockers.md`, B3 and B11).

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

## Mesa ISA mode

`zoc --mesa-isa` (and `ZLUDA_AGX_MESA_ISA=1` for `zluda_agx`) compiles for M1 and M2 using only the instruction forms
that Mesa's asahi compiler emits for the same GPU: the LLVM backend's `mesa-isa` feature (`llvm/docs/AGXUsage.rst` in
the LLVM fork, "Mesa ISA mode"). Code then relies only on encodings that a production driver uses, and the backend
reports any other instruction as an error. Nothing changes in the PTX translation: every AGX intrinsic ZLUDA calls has a
Mesa form. What the backend does differently:

- float atomic adds are compare-and-swap loops, not the M2's native float add;
- 64-bit unsigned atomic min/max use the lock table, not the M2's native form;
- device and threadgroup loads, stores and atomics are always encoded in their long form, as Mesa packs them.

The mode is part of the compilation cache key. The library kernels (`agx_libs`) carry a `CHIP+mesa-isa` build of every
M1 and M2 entry, compiled with `#agxir.target<..., features = "+mesa-isa">` so that the MLIR `agx` operations take
their portable forms, and `zluda_agx` loads those entries in this mode. The AGX Performance Simulator's
`--mesa-isa GPU` (or `AGX_MESA_ISA=GPU`) runs code with the same check, so a test fails on any instruction Mesa would
not emit; `agx_ptx_formalization_check.py --mesa-isa` and `test_kernels.py` (for `+mesa-isa` entries) use it.

Results (2026-10-21, simulator):
- PTX-formalization with `--mesa-isa`, run under `agx_run --mesa-isa`: on `agx-g14c` and `agx-g13g`, 282 programs pass
  and all 1,728 checks match with `--strict-fp`, and 1,553 checks match without it. That is the same as without the
  mode, and no program used an instruction outside Mesa's forms.
- `cargo test -p ptx --no-default-features --features agx agx`: 388 of 388 Mesa ISA compilations, the 194 run tests
  that compile, on `agx-g13g` and `agx-g14c`.
- Cost (agx-bench in the LLVM fork): 6.0% more code from the long memory forms, and 0.4% more estimated cycles, all of
  it the float atomic add on M2.

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

The one skipped check follows the non-PTX `coll_test` of `collective.ptx`; `tcg2-1.ptx` does not compile (its
cluster needs more than 32 KiB of shared memory) and the harness has no `.multimem` and `.fabric` directives.

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
* Modules: `cuModuleLoadData` takes PTX text, a fat binary (its PTX for the
  newest architecture is used; SASS-only fat binaries cannot load) or an AGX
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
  `CUDA_ERROR_LAUNCH_FAILED`.
* At every synchronisation, device `printf` output is printed, and a failed
  device `assert` is returned as `CUDA_ERROR_ASSERT`.
* There is one device and one context. The render node is `$ZLUDA_AGX_NODE` or
  the first one that answers the Asahi parameter query.

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
f32 tensors of up to 4 dimensions with any strides:

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

These kernels are tested on the simulator for agx-g13g and agx-g14c (63
cases per chip). The backend (graph) API, f16 for these operations, RNNs,
LRN, dropout and the fused operations are not implemented yet.

## cuFFT (`zluda_fft_agx`)

`libcufft` on `agx_libs/kernels/fft.mlir` (one source for every chip).
Single precision: complex to complex transforms of rank 1 to 3 (rank 1 also
strided and batched through `cufftPlanMany`, with the same input and output
layout), and real to complex / complex to real transforms of rank 1,
batched, in or out of place. Power-of-two lengths use radix-2 Stockham passes
alternating two work buffers; other lengths use a direct transform. The
twiddles are computed on the host in double precision and uploaded with the
plan. The plan allocates its own work buffers (`cufftSetWorkArea` is
accepted and unused). Like cuFFT, inverse transforms are not normalized.
Double precision, multi-dimensional real transforms, callbacks and the Xt
API return `CUFFT_NOT_SUPPORTED`. The kernels are tested on the simulator
for agx-g13g and agx-g14c, and the plans' launch sequences (ping-pong
buffers, in-place copies, the passes of each dimension) are checked in unit
tests against a direct DFT.

## cuSPARSE (`zluda_sparse_agx`)

`libcusparse` on `agx_libs/kernels/sparse.mlir` (one source for every chip).
The generic API with f32 values and 32- or 64-bit indices (the row offsets and
the columns of one type), zero- or one-based: `cusparseSpMV` on CSR and COO
matrices, non-transposed or transposed, and `cusparseSpMM` on CSR matrices with
`opA` non-transposed and `opB` either way, on column- or row-major dense
matrices. Non-transposed CSR SpMV and SpMM use one thread per output element;
the transposed and COO products scale `y` by `beta` first and then add with
atomic f32 adds. No external buffer is needed (`*_bufferSize` returns 0).
Handles, streams, host and device pointer modes and the legacy matrix
descriptor are supported. Other functions (sparse formats beyond CSR and COO,
SpGEMM, SpSV, the legacy level 2 and 3 routines, other value types) return
`CUSPARSE_STATUS_NOT_SUPPORTED`.

## Building a release for Asahi

```
ZLUDA_LLVM_BUILD_DIR=/path/to/llvm-agx-build cargo build --release -p compiler --no-default-features --features agx
ZLUDA_XTASK_AGX=1 cargo xtask --release      # zluda_agx and the *_agx libraries (NVML, cuBLAS, cuBLASLt, cuDNN, cuFFT, cuSPARSE), with their library symlinks
```

`ZLUDA_XTASK_AGX` selects the packages marked `agx = true` in their `[package.metadata.zluda]` instead of the
HIP-based ones; the two sets ship libraries with the same names. Put `target/release/zoc` next to the application
or point `ZLUDA_AGX_ZOC` at it, and run the application with `LD_LIBRARY_PATH=target/release`.

## Support matrix (Asahi Linux, M1 / M2 family)

| Area | Status |
| --- | --- |
| Chips | G13 (M1 family) and G14 (M2 family) code generation; G14C (M2 Max) is the measured target. Apple9 (M3 and later) is not supported by the backend. |
| PTX | 194 of 195 ZLUDA run tests compile on their own, and `extern_func` compiles when linked (`cuLink*`). Of the 1729 PTX-formalization checks on agx-g14c, 1728 run and all of them match the formal semantics with `--strict-fp` (1553 without). Not supported: `.multimem` and `.fabric`, `f32x2` forms other than `add` / `sub` / `mul` / `fma`, clusters with more than 32 KiB of shared memory |
| Warp-level and async | `mma`, `mma.sp`, `wmma`, `ldmatrix` / `stmatrix`, `wgmma`, `tcgen05` and the tensor memory accelerator (`cp.async.bulk.tensor`) are emulated; `cp.async`, mbarriers and thread block clusters (up to 1024 threads) run on ordinary instructions and threadgroup memory |
| Textures and surfaces | sampled in software (the texture unit is not used yet, simulator task B29): `tex`, `tld4`, `txq`, `suld` / `sust` / `sured`, `suq`, `istypep`; CUDA arrays, mipmapped arrays, texture and surface objects. No legacy texture references, border colours or per-dimension address modes |
| Floating point | f32 hardware semantics by default (denormals flushed); `--strict-fp` / strict mode for exact PTX semantics; f64 in software; f16 / bf16 / fp8 conversions |
| Atomics | 32-bit native; f32 add and u64 min/max without result native on G14; other 64-bit and 128-bit through a lock table |
| Memory model | fences of every scope lower to measured barrier sequences |
| Driver API | devices, contexts, memory, copies, modules (PTX, fat binaries, AGX objects), linking (`cuLink*`), CUDA arrays, launches, streams, events, graphs (stream capture), occupancy, `cuGetProcAddress`, export tables. Other functions return `CUDA_ERROR_NOT_SUPPORTED` (see `ZLUDA_AGX_LOG`). |
| Device code features | `printf`, `assert` (`CUDA_ERROR_ASSERT`) |
| Libraries | cuBLAS: f32, f16 and f64 GEMM (f64 in software), f32 level 1, gemv / ger, geam / dgmm / syrk / symm / trmm / trsm and batched GEMM (`zluda_blas_agx`); cuBLASLt matmul with bias / ReLU / GELU epilogues (`zluda_blaslt_agx`); cuDNN 2-D convolutions, activations, softmax, pooling and batch normalization (`zluda_dnn_agx`); cuFFT single precision (`zluda_fft_agx`); cuSPARSE SpMV and SpMM in f32 (`zluda_sparse_agx`) |
| Not yet | per-thread scratch on hardware (`.local`, spills, recursion; simulator task B1), cooperative launches, peer access, IPC, cuBLAS double precision beyond GEMM, complex types and syr2k / her*, the cuDNN graph API and f16 beyond convolutions, double precision cuFFT, cuSPARSE beyond SpMV and SpMM |
| NVML | device count, name, UUID, memory, compute capability; no telemetry |
| Mesa ISA mode | `zoc --mesa-isa` / `ZLUDA_AGX_MESA_ISA=1`: only the instructions Mesa's asahi compiler emits, on M1 and M2 (see "Mesa ISA mode"); PTX-formalization results identical, library kernels built for it |
| Hardware validation | pending: the open questions are in the AGX Performance Simulator's `tasks/ptx-re-blockers.md` |
