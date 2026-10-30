# .ptx → .air (→ .agx): experiments and dead ends

A log of what was tried on the Air target (`compile_air.rs`, `lib.cpp`, the AIR/AGX backends it drives) and did
not work, or worked differently than expected, so that later attempts start from it. Newest first within each
section. GPU times are from the M2 Max behind the AIR service; "suite" is PTX-formalization's tests through
`ptx/tools/air_ptx_formalization_check.py` (default mode baseline 245–247 PASS, strict subset 18/18).

## Open problems

- **(Solved with real calls, below.) Blender Cycles to AGX code fully inlined does not fit in ~7.9 GB**, on
  either route. Whole-module: the old route
  (`--target agx`) reached 5.7 GB and the AIR route 6.5 GB before the watchdog stopped them (2026-10-30, current
  trees). The user recalls AGX code generation working at one point; no record of a Cycles AGX build was found in
  /scratch or the commit history (the per-kernel commit 3435e7a already says shade_shadow needs over 6.5 GB), so
  the working build may have been an earlier commit, another machine, or smaller programs. If it was an earlier
  commit, it is a regression to bisect (candidates: the AGX always-inline policy, the texture and soft-f64
  libraries, the AIR inliner's keep-function changes of 2026-10-29) rather than a limit to work around.
  Rerun alone (nothing else on the container, 2026-10-30): the old route still stops at 6.4 GB within 73 s,
  with or without `--object-only` (the IR text dumps are not the difference). Cycles' PTX only reached /scratch on
  2026-10-28 23:14, and the first AGX attempt after it (3435e7a) already ran out, so in this container Cycles has
  never compiled to AGX; a working build would have been on a machine with more memory. A kernel at a time (`ZLUDA_AGX_PER_KERNEL=1`), 12 kernels compile, then
  `kernel_gpu_integrator_shade_shadow` (about 5M instructions with everything inlined, as the AGX backend inlines
  every non-kernel function) runs out. Measured on that kernel alone with `llc`: straight-line strength reduction's
  scalar evolution took it from 0.9 to 4.4 GB (now skipped above 200k instructions,
  `-agx-straight-line-sr-max-instructions`; 2.8 GB without it), the load/store vectorizer adds ~1.2 GB, and
  StructurizeCFG then ran for over 25 minutes. Inside `zoc`, RSS only grew across the per-kernel loop (1 → 5.6 GB
  over the first 12 kernels) although each kernel has its own context. Not the allocator keeping freed memory:
  `malloc_trim(0)` after each kernel (with its module, diagnostics and context dropped first) left the curve the
  same (6.6 GB at shade_shadow); reverted. Nothing accumulates either: traced per kernel, RSS falls back to
  0.2–0.5 GB between kernels (shade_background peaks at 2.2 GB, shade_light_* at 1.2 GB). shade_shadow alone grows
  in steps: 0.5 → 1.9 GB in its first minute (IR passes), flat for 7 minutes (StructurizeCFG on one 5M-instruction
  function), 4.2 → 5.0 GB at ~8 minutes (instruction selection, by the pass order), 5.6 → 6.4 GB at ~17 minutes
  (the machine passes). It is the size of one fully inlined function in machine IR; pass tweaks will not bring
  it under 7.9 GB. What would: not inlining everything into the kernel (real calls in the AGX backend, now only
  `-agx-experimental-leaf-calls`), or a machine with more memory.
- One Cycles kernel, `kernel_gpu_integrator_shade_surface_mnee` (3.7 MB of AIR packaged alone), exceeds the AIR
  service's 120 s Metal compile cap. With the .ftz and texture fixes the other five of the six that used to time
  out now build (2026-10-30, wall time with the queue: shade_background 129 s, shade_surface 63 s,
  shade_surface_raytrace 110 s, shade_volume_ray_marching 38 s, shader_eval_curve_shadow_transparency 19 s; their
  AIR 40-55% smaller), so 74 of Cycles' 75 pipelines build. Metal's compile time for these kernels, not ZLUDA's,
  is now the slow part. Not over-inlining: shade_surface_mnee packaged alone keeps 162 functions, and
  `-air-keep-function-cost` 20000 / 5000 / 2000 left it byte-identical (its large functions are kept already).

## Real calls in the AGX backend (`-agx-experimental-leaf-calls`), 2026-10-30

The way out of Cycles' AGX memory: shade_shadow with its large helpers kept as called functions compiles in 14 s at
156 MB (fully inlined: 6.4 GB and over 25 minutes in StructurizeCFG without finishing). What was in the way, in
the order it was found:

- **A crash at once on Cycles**: AGXAlwaysInline's call-depth computation held a DenseMap iterator across an insert
  into the same map (fine for the small lit tests, not for 120 functions).
- **63 of the 71 refused helpers read global data** (`kernel_params`, lookup tables, `switch.table.*`), which only a
  kernel can address (a uniform register). AGXGlobalDataArgs now passes the buffer's address as an extra i64
  parameter to the kept functions that need it and their callers; their global references become address + the
  variable's offset in computeGlobalDataLayout's layout. (A first test module made with `llvm-extract --recursive`
  had every global as a declaration, which gives the same "unsupported external global" error: extract with
  `--rglob='.*'`.)
- **Callees before the kernel in .text**: modules from AIR list functions before kernels, and a kernel's program
  runs from its symbol on, so a call jumped backwards out of it (indirect.ptx: "targets -52 outside the program").
  Called functions are now moved after every kernel.
- **The runtime and the harness loaded only the kernel symbol's own bytes**; with called functions in the module,
  a kernel's code now extends to the end of .text in agx_runtime and agx_ptx_formalization_check.py.
- **The simulator rejects `ret`** unless `AGX_EXPERIMENTAL_RET=buffer-pop2` (the return model measured on 570
  hardware cases); call timing is not calibrated. Most PTX suite programs keep no calls (the AIR inliner inlines
  functions under 100 instructions), so passing tests there say little about calls: indirect.ptx is the one that
  exercised them.

- **Stack frames over the fixed 4 KB per call depth** (whole-module Cycles: shade_volume's kernel frame 6.3 KB;
  per kernel: integrate_surface's callee frame 6.9 KB). Frame offsets are 15-bit, so all depths must fit in 32 KB.
  `-agx-measured-call-frames` (a module of one kernel, so per-kernel compilation): the kernel's region is its
  frame, each depth's is the largest frame at that depth, recorded as called functions compile in increasing
  depth after the kernel; the kernel reserves all 32 KB. A larger kernel region alone, with fixed 4 KB depths,
  was not enough (callee frames over 4 KB too).
- **AGXGlobalDataArgs appended the rewritten functions to the module**, which undid the depth order measured frames
  rely on ("called functions out of depth order"): each now takes its original's place.
- **The per-kernel object merger refused duplicate function names**: each kernel's object now carries its own copy
  of what it calls (local symbols, calls resolved within the object). It accepts duplicate locals and writes them
  with the other local symbols.
- Result, Cycles per kernel with `-agx-experimental-leaf-calls -agx-measured-call-frames`: all 75 kernels, a
  28.9 MB object (1338 local called functions), 1.0 GB peak, 119 s in all (AGX stage 83 s). Fully inlined it ran
  out at 6.4 GB on shade_shadow. Call timing on the GPU is not calibrated; no GPU time measured yet.

- **The AGX object is mostly duplicates**: Cycles' 28.9 MB holds 0.7 MB of kernels and 28.0 MB of called
  functions, 1338 copies of 213 functions (5.3 MB with one copy each). 151 of the 213 are byte-identical in
  every kernel; the others differ by their call offsets and (measured) frame offsets. Sharing one copy needs
  the backend to emit relocations for calls to local functions (they are resolved PC-relative when each
  kernel's object is written), so that the merger can point every kernel's calls at the one copy; and one
  frame layout for the shared functions. Done (ceafb92, `-agx-call-relocations`): 28.9 -> 16.7 MB, 516 copies
  kept; the remaining duplicates differ by their kernels' measured frame offsets, so a frame layout common to
  the kernels would be the next step (one copy each: about 6 MB).
- **A single-kernel module never reaches the merger**, so with `-agx-call-relocations` its calls stayed unresolved
  and the simulator crashed decoding them (17 tests, "index out of bounds" in fsem.rs). Any object with call
  relocations now goes through the merger, on both routes.
- **Small modules forced through the per-kernel path** (`ZLUDA_AGX_PER_KERNEL=1`): tma-red.ptx fails to merge,
  "the kernels' objects have different global data", with or without calls; Cycles (and the default, over 4 MB)
  does not hit it.
- **zoc was not deterministic** (until 649b737's parent): the Metal copy's functions came out in heap-address
  order (LLVMZludaAirGlobalsToBuffer), almost a million lines of difference between two runs of Cycles. Compare
  outputs of two runs (`llvm-dis`, or md5sum now that they match) before attributing a difference to a change.
- **Device-flaky PTX tests**: ldm-0, ldm2-st0 and occupancy pass or fail from one run to the next with the same
  zoc; rerun them with the previous build before calling a change a regression.

## Dead ends and surprises

- **The scratch `guard.py` (compile a kernel's pipeline without running it) assumed a named entry block**; zoc's
  current Metal AIR has unnamed ones. Run `opt -passes=instnamer` before it.

- **"Cannot select: load (s64) ... addrspace 2" from AGX instruction selection** after regenerating the texture
  library was not about the loads: the newer clang made switch lookup tables (`@switch.table.*`), globals AIR and
  AGX do not have, and the first error (`global switch.table... has a type AIR does not have`) is easy to miss
  above it. `gen_agx_tex.py` now passes `-fno-jump-tables`. Same class of failure as the first pipeline's
  `simplifycfg` lookup tables: keep `switch-to-lookup` away from anything AIR or AGX compiles.
- **Regenerating `zluda_ptx_impl_agx.ll` with a different clang rewrites every generated section** (the library was
  made with Ubuntu clang 18.1.3; the tree's clang 24 changes cvt, heap and math too, and `llvm.lifetime.*` lost its
  size argument). Only the texture section was regenerated and spliced in; the others keep clang 18's output.
- **Soft-f64 was blamed on Cycles' 113 `.rz` f32 operations; wrong.** Directed-rounding operations are already
  outlined into shared `__air_hint.*` functions (one call site each). All of Cycles' soft-f64 (720 multiplies, 40
  divides) came from the software texture sampler, which filters in double to match the formal model exactly and
  is inlined at every `tex`. Count the callers of `__air_f64_*` before assuming where doubles come from: Cycles'
  PTX has no `.f64` at all.
- **Software subnormal flushing for `.ftz` round-to-nearest f32 operations doubled Cycles' IR** (1.46M vs 0.53M
  instructions; 49 s vs 18 s .ptx → .air). Metal's f32 hardware already flushes outside strict mode; flush in
  software only in strict mode and for directed or unknown roundings.
- **Release `zoc` discards value names**, so anything identified by a value name breaks only in release builds:
  the globals buffer's and the atomics lock table's `air.arg_name` were empty (the harness then reports a flood of
  `UNSUPPORTED: more AIR buffers than PTX parameters`, and the lock table would get an ordinary location index).
  Interface names now go through an `air-arg-name` attribute. Earlier, parsing the soft-f64 library in a
  name-discarding context failed the same way. Test release builds, not only debug ones.
- **Regenerating the soft-f64 library with clang 24 plus a branch-free `fma` / `clz128`** (no branches in
  `__air_f64_fma`, 282 → 258 instructions, host-tested on 2M cases): fma -4.8% GPU time, but mul +7.9%, div +4.6%
  (the new clang's code for the rest). Reverted; the library stays as generated before. `-mlzcnt` did not remove
  `fma`'s branches either (they came from `clz128` branching between two counts, not from the cost model).
- **Pipeline choice does not change Metal GPU time**: O3 vs light vs light+, the old route vs the direct one, GEP vs
  integer address arithmetic all measured equal within noise; Metal's compiler re-optimizes. Choose pipelines for
  compile time and memory. (The GEP-addressing change to AIRLegalizeUnsupportedIR was reverted for this reason.)
- **A light first pipeline crashed Metal's compiler** (`XPC_ERROR_CONNECTION_INTERRUPTED`) on 125 SIMD tests: SIMD
  operations left inside loops and under branches. The first stage needs full unrolling, lower-constant-intrinsics
  and tailcallelim (recursion in calls.ptx). Check new pipelines offline before sending them to the device; a
  crashing or hung request can freeze the Mac running the service.
- **InstCombine's fixpoint verification aborts** in the light pipelines: use `instcombine<no-verify-fixpoint>`.
- **Metal's pipeline cache ignores metadata**: a per-build salt in metadata did not defeat it; an unused volatile
  stack array per kernel does (`ZLUDA_AIR_SALT`).
- **The globals buffer as a `ptr` parameter of non-kernel functions was rejected by Metal**; it is passed as i64.
- **Per-kernel AGX compilation** first copied every function into each kernel's module (reachable functions only
  now) and shared one context (a fresh context per kernel now: the shared one held Cycles at 4 GB).
- **`llvm-extract` of one kernel from portable AIR**: `llc` rejects its bitcode ("Invalid call record") and the
  other kernels' `!air.kernel` entries remain ("an !air.kernel entry names no function"). Round-trip through
  `.ll` and prune `!air.kernel` to the kept kernel.
- **Never run Geekbench's BVH kernels with random (gbgen) inputs**: they hang the GPU.
