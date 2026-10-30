# .ptx → .air (→ .agx): experiments and dead ends

A log of what was tried on the Air target (`compile_air.rs`, `lib.cpp`, the AIR/AGX backends it drives) and did
not work, or worked differently than expected, so that later attempts start from it. Newest first within each
section. GPU times are from the M2 Max behind the AIR service; "suite" is PTX-formalization's tests through
`ptx/tools/air_ptx_formalization_check.py` (default mode baseline 245–247 PASS, strict subset 18/18).

## Open problems

- **Blender Cycles to AGX code does not fit in ~7.9 GB**, on either route. Whole-module: the old route
  (`--target agx`) reached 5.7 GB and the AIR route 6.5 GB before the watchdog stopped them (2026-10-30, current
  trees). The user recalls AGX code generation working at one point; no record of a Cycles AGX build was found in
  /scratch or the commit history (the per-kernel commit 3435e7a already says shade_shadow needs over 6.5 GB), so
  the working build may have been an earlier commit, another machine, or smaller programs. If it was an earlier
  commit, it is a regression to bisect (candidates: the AGX always-inline policy, the texture and soft-f64
  libraries, the AIR inliner's keep-function changes of 2026-10-29) rather than a limit to work around. A kernel at a time (`ZLUDA_AGX_PER_KERNEL=1`), 12 kernels compile, then
  `kernel_gpu_integrator_shade_shadow` (about 5M instructions with everything inlined, as the AGX backend inlines
  every non-kernel function) runs out. Measured on that kernel alone with `llc`: straight-line strength reduction's
  scalar evolution took it from 0.9 to 4.4 GB (now skipped above 200k instructions,
  `-agx-straight-line-sr-max-instructions`; 2.8 GB without it), the load/store vectorizer adds ~1.2 GB, and
  StructurizeCFG then ran for over 25 minutes. Inside `zoc`, RSS only grew across the per-kernel loop (1 → 5.6 GB
  over the first 12 kernels) although each kernel has its own context: being checked with `malloc_trim` after each
  kernel.
- Six Cycles kernels exceed the AIR service worker's 60 s Metal compile cap (the client allows 120 s; the worker
  was not restarted).

## Dead ends and surprises

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
