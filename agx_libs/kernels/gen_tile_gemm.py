#!/usr/bin/env python3
"""Generates the tile GEMM kernels of sgemm.mlir and hgemm.mlir (written out, as MLIR has no loop over SSA names):
each SIMD-group's 32 x 32 tile is 4 x 4 air 8x8 matrices accumulated along K in registers. The kernels are
row-major D = A B with variants that read A or B transposed (`_ta`, `_tb`, `_tatb`): A[r, k] at a[k * lda + r]
and B[k, c] at b[c * ldb + k]. Run it after changing the kernel shape:

  gen_tile_gemm.py sgemm.mlir hgemm.mlir

Each gpu.func @agx_[sh]gemm_tile64* of the files is replaced in place (the comments before it stay); missing
variants are appended at the end of the module.
"""

import re
import sys

KERNELS = {  # file: [(name, A and B type, C type)]
    "sgemm.mlir": [("agx_sgemm_tile64", "f32", "f32")],
    "hgemm.mlir": [("agx_hgemm_tile64_cf16", "f16", "f16"), ("agx_hgemm_tile64_cf32", "f16", "f32")],
}
VARIANTS = [("", False, False), ("_ta", True, False), ("_tb", False, True), ("_tatb", True, True)]
# `_u` kernels make no alignment assumption (odd leading dimensions or strides, matrices not 8-byte aligned)
ALIGNMENTS = [("", True), ("_u", False)]


def kernel(name, ab, c, ta, tb, aligned=True):
    M = lambda t: f"vector<64x{t}>"
    acc, mab = M("f32"), M(ab)
    mem = lambda t: f"memref<?x?x{t}, strided<[?, 1], offset: ?>, 1>"
    o = []
    w = o.append
    w(f"  gpu.func @{name}(%a0: memref<1x{ab}, 1>, %b0: memref<1x{ab}, 1>,")
    w(f"      %c0: memref<1x{c}, 1>, %M: index, %N: index, %K: index, %lda: index,")
    w(f"      %ldb: index, %ldc: index, %bsa: index, %bsb: index, %bsc: index, %alpha: f32,")
    w(f"      %beta: f32) kernel")
    w("      attributes {known_block_size = array<i32: 128, 1, 1>} {")
    for v in [0, 1, 2, 8, 16, 24, 32, 64, 128]:
        w(f"    %i{v} = arith.constant {v} : index")
    # The plan launches the aligned kernel with 8-byte aligned matrices and even leading dimensions and batch
    # strides: said here, the two elements of a matrix row a lane loads are one aligned 32-bit access even for
    # 16-bit types. The `_u` kernel takes them as they are.
    e = "_e" if aligned else ""  # the values the kernel uses: masked even, or as passed
    mat = (lambda n: f"%{n}0a") if aligned else (lambda n: f"%{n}0")
    if aligned:
        w("    %even = arith.constant -2 : index")
        for n in ["a", "b", "c"]:
            w(f"    %{n}0a = memref.assume_alignment %{n}0, 8 : memref<1x{ab if n != 'c' else c}, 1>")
        for v in ["lda", "ldb", "ldc", "bsa", "bsb", "bsc"]:
            w(f"    %{v}_e = arith.andi %{v}, %even : index")
    # the matrices of batch entry z (grid y) are bsa, bsb and bsc elements on
    w("    %z = gpu.block_id y")
    for n in ["a", "b", "c"]:
        w(f"    %o{n} = arith.muli %z, %bs{n}{e} : index")
    # a transposed operand is stored as its transpose, row-major
    for n, t, sz, ld in [("A", ab, "%K, %M" if ta else "%M, %K", "%lda" + e),
                         ("B", ab, "%N, %K" if tb else "%K, %N", "%ldb" + e), ("C", c, "%M, %N", "%ldc" + e)]:
        w(f"    %{n} = memref.reinterpret_cast {mat(n.lower())} to offset: [%o{n.lower()}], sizes: [{sz}], strides: [{ld}, 1]")
        w(f"        : memref<1x{t}, 1> to {mem(t)}")
    w("""
    // Grouped tile order: groups of 8 tile rows, column-major inside a group.
    %tiles_m = arith.divui %M, %i64 : index
    %tiles_n = arith.divui %N, %i64 : index
    %id = gpu.block_id x
    %per_group = arith.muli %tiles_n, %i8 : index
    %group = arith.divui %id, %per_group : index
    %first_m = arith.muli %group, %i8 : index
    %left_m = arith.subi %tiles_m, %first_m : index
    %group_m = arith.minui %left_m, %i8 : index
    %in_group = arith.remui %id, %per_group : index
    %rem_m = arith.remui %in_group, %group_m : index
    %tm = arith.addi %first_m, %rem_m : index
    %tn = arith.divui %in_group, %group_m : index

    // This SIMD-group's 32 x 32 tile: 4 x 4 matrices.
    %sg = gpu.subgroup_id : index
    %sr = arith.divui %sg, %i2 : index
    %sc = arith.remui %sg, %i2 : index
    %tm64 = arith.muli %tm, %i64 : index
    %tn64 = arith.muli %tn, %i64 : index
    %sr32 = arith.muli %sr, %i32 : index
    %sc32 = arith.muli %sc, %i32 : index
    %row = arith.addi %tm64, %sr32 : index
    %col = arith.addi %tn64, %sc32 : index""")
    for i in range(1, 4):
        w(f"    %row{i} = arith.addi %row, %i{8 * i} : index")
        w(f"    %col{i} = arith.addi %col, %i{8 * i} : index")
    rows = ["%row"] + [f"%row{i}" for i in range(1, 4)]
    cols = ["%col"] + [f"%col{i}" for i in range(1, 4)]
    w("    %fz = arith.constant 0.0 : f32")
    w(f"    %zero = air.simdgroup_matrix_fill %fz : {acc}")
    ts = ", ".join([acc] * 16)
    w("    %acc:16 = scf.for %kk = %i0 to %K step %i128")
    w("        iter_args(" + ", ".join(f"%o{i}{j} = %zero" for i in range(4) for j in range(4)) + f") -> ({ts}) {{")
    w("      gpu.barrier memfence []")
    w("      %kk_end0 = arith.addi %kk, %i128 : index")
    w("      %kk_end = arith.minui %kk_end0, %K : index")
    w("      %in:16 = scf.for %k = %kk to %kk_end step %i8")
    w("          iter_args(" + ", ".join(f"%a{i}{j} = %o{i}{j}" for i in range(4) for j in range(4))
      + f") -> ({ts}) {{")
    # With B transposed (and A not) in 16-bit, each A fragment is instead loaded just before its four products
    # (agx_cost: hgemm _tb -9..-13% on G14G; G13G moves both ways, sgemm _tb loses).
    a_late = tb and not ta and ab == "f16"
    if not a_late:
        for i in range(4):
            idx = f"%k, {rows[i]}] transpose" if ta else f"{rows[i]}, %k]"
            w(f"        %ma{i} = air.simdgroup_matrix_load %A[{idx} : {mem(ab)} -> {mab}")
    tys = acc if ab == "f32" else f"{mab}, {acc}"
    # With A transposed (and B not), each B fragment is loaded just before its four products: fewer fragments live at
    # once (agx_cost at K = 256: -8% on G14G, -19% on G13G for agx_sgemm_tile64_ta); the other variants lose or
    # move both ways with it, so they load all of B first.
    interleave = ta and not tb
    for j in range(4):
        idx = f"{cols[j]}, %k] transpose" if tb else f"%k, {cols[j]}]"
        w(f"        %mb{j} = air.simdgroup_matrix_load %B[{idx} : {mem(ab)} -> {mab}")
        if interleave:
            for i in range(4):
                w(f"        %n{i}{j} = air.simdgroup_matrix_multiply_accumulate %ma{i}, %mb{j}, %a{i}{j} : {tys}")
    if not interleave:
        for i in range(4):
            if a_late:
                idx = f"%k, {rows[i]}] transpose" if ta else f"{rows[i]}, %k]"
                w(f"        %ma{i} = air.simdgroup_matrix_load %A[{idx} : {mem(ab)} -> {mab}")
            for j in range(4):
                w(f"        %n{i}{j} = air.simdgroup_matrix_multiply_accumulate %ma{i}, %mb{j}, %a{i}{j} : {tys}")
    w("        scf.yield " + ", ".join(f"%n{i}{j}" for i in range(4) for j in range(4)) + f" : {ts}")
    w("      }")
    w("      scf.yield " + ", ".join(f"%in#{k}" for k in range(16)) + f" : {ts}")
    w("    }")
    w("")
    w("    // C = alpha * acc + beta * C, C not read when beta is 0 (one branch for the tile).")
    w("    %no_c = arith.cmpf oeq, %beta, %fz : f32")
    w(f"    %malpha = air.simdgroup_matrix_fill %alpha : {acc}")
    w(f"    %mbeta = air.simdgroup_matrix_fill %beta : {acc}")
    mc = M(c)

    def store(v, i, j, s, ind):
        if c == "f32":
            w(f"{ind}air.simdgroup_matrix_store {v}, %C[{rows[i]}, {cols[j]}] : {acc} -> {mem(c)}")
        else:
            w(f"{ind}%h{s} = arith.truncf {v} : {acc} to {mc}")
            w(f"{ind}air.simdgroup_matrix_store %h{s}, %C[{rows[i]}, {cols[j]}] : {mc} -> {mem(c)}")

    w("    scf.if %no_c {")
    for i in range(4):
        for j in range(4):
            s = f"_{i}{j}"
            w(f"      %s{s} = arith.mulf %acc#{4 * i + j}, %malpha : {acc}")
            store(f"%s{s}", i, j, s, "      ")
    w("    } else {")
    for i in range(4):
        for j in range(4):
            s = f"_{i}{j}"
            w(f"      %s{s} = arith.mulf %acc#{4 * i + j}, %malpha : {acc}")
            if c == "f32":
                w(f"      %old{s} = air.simdgroup_matrix_load %C[{rows[i]}, {cols[j]}] : {mem(c)} -> {mc}")
            else:
                w(f"      %oh{s} = air.simdgroup_matrix_load %C[{rows[i]}, {cols[j]}] : {mem(c)} -> {mc}")
                w(f"      %old{s} = arith.extf %oh{s} : {mc} to {acc}")
            w(f"      %r{s} = math.fma %old{s}, %mbeta, %s{s} : {acc}")
            store(f"%r{s}", i, j, s, "      ")
    w("    }")
    w("    gpu.return")
    w("  }")
    return o


def main():
    for path in sys.argv[1:]:
        base = path.rsplit("/", 1)[-1]
        L = open(path).read().split("\n")
        for name, ab, c in KERNELS[base]:
            for (asuffix, aligned), (suffix, ta, tb) in [(al, v) for al in ALIGNMENTS for v in VARIANTS]:
                fn = name + asuffix + suffix
                body = kernel(fn, ab, c, ta, tb, aligned)
                start = next((i for i, l in enumerate(L) if re.match(r"  gpu\.func @%s\(" % fn, l)), None)
                if start is None:
                    end = max(i for i, l in enumerate(L) if l == "}")  # the module's end
                    which = {(False, False): "A and B as they are",
                             (True, False): "A[r, k] at a[k * lda + r]", (False, True): "B[k, c] at b[c * ldb + k]",
                             (True, True): "A[r, k] at a[k * lda + r] and B[k, c] at b[c * ldb + k]"}[(ta, tb)]
                    doc = ["", f"  // {name} with {which} (transposed)" + ("." if aligned else ", without alignment.")]
                    L[end:end] = doc + body
                else:
                    end = next(i for i in range(start, len(L)) if L[i] == "  }")
                    L[start:end + 1] = body
        open(path, "w").write("\n".join(L))


if __name__ == "__main__":
    main()
