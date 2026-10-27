#!/usr/bin/env python3
"""Generates the dot GEMM kernels of sgemm.mlir and hgemm.mlir: agx_sgemm_generic's arguments, with a SIMD-group per
element of C instead of a thread. Its lanes split K (four elements a lane at a time, then one) and sum by a SIMD-group
reduction, so a row of op(A) or a column of op(B) that is contiguous along K is read by consecutive lanes. The plan
uses them for C with a side of at most 8 (GEMV-shaped products, as in decoding). Run it after changing the kernel:

  gen_dot_gemm.py sgemm.mlir hgemm.mlir

Each gpu.func @agx_[sdh]gemm_dot* of the files is replaced in place (the comments before it stay); missing kernels
are appended at the end of the module.
"""

import re
import sys

KERNELS = {  # file: [(name, A and B type, C type, accumulator and scalar type)]
    "sgemm.mlir": [("agx_sgemm_dot", "f32", "f32", "f32"), ("agx_dgemm_dot", "f64", "f64", "f64")],
    "hgemm.mlir": [("agx_hgemm_dot_cf16", "f16", "f16", "f32"), ("agx_hgemm_dot_cf32", "f16", "f32", "f32")],
}


def kernel(name, ab, c, acc):
    mem = lambda t: f"memref<?x{t}, strided<[1]>, 1>"
    o = []
    w = o.append
    w(f"  gpu.func @{name}(%a0: memref<1x{ab}, 1>, %b0: memref<1x{ab}, 1>,")
    w(f"      %c0: memref<1x{c}, 1>, %M: index, %N: index, %K: index, %sar: index, %sac: index,")
    w(f"      %sbr: index, %sbc: index, %scr: index, %scc: index, %alpha: {acc}, %beta: {acc}) kernel {{")
    for v in [0, 1, 32, 96, 127, 128]:
        w(f"    %i{v} = arith.constant {v} : index")
    w("    %big = arith.constant 4611686018427387904 : index")
    for n, t in [("A", ab), ("B", ab), ("C", c)]:
        w(f"    %{n} = memref.reinterpret_cast %{n.lower()}0 to offset: [0], sizes: [%big], strides: [1]")
        w(f"        : memref<1x{t}, 1> to {mem(t)}")
    w(f"    %f0 = arith.constant 0.0 : {acc}")
    w("    %g = gpu.global_id x")
    w("    %s = arith.divui %g, %i32 : index")
    w("    %lane = gpu.lane_id")
    w("    %MN = arith.muli %M, %N : index")
    w("    // the same for the whole SIMD-group, which reduces together")
    w("    %live = arith.cmpi ult, %s, %MN : index")
    w("    scf.if %live {")
    w("      %i = arith.divui %s, %N : index")
    w("      %j = arith.remui %s, %N : index")
    w("      %ia = arith.muli %i, %sar : index")
    w("      %jb = arith.muli %j, %sbc : index")

    def term(k, sum_in, out, ind):
        t = out[1:]
        w(f"{ind}%ka_{t} = arith.muli {k}, %sac : index")
        w(f"{ind}%kb_{t} = arith.muli {k}, %sbr : index")
        w(f"{ind}%pa_{t} = arith.addi %ia, %ka_{t} : index")
        w(f"{ind}%pb_{t} = arith.addi %kb_{t}, %jb : index")
        x, y = (f"%x_{t}", f"%y_{t}") if ab == acc else (f"%xn_{t}", f"%yn_{t}")
        w(f"{ind}{x} = memref.load %A[%pa_{t}] : {mem(ab)}")
        w(f"{ind}{y} = memref.load %B[%pb_{t}] : {mem(ab)}")
        if ab != acc:
            w(f"{ind}%x_{t} = arith.extf {x} : {ab} to {acc}")
            w(f"{ind}%y_{t} = arith.extf {y} : {ab} to {acc}")
        w(f"{ind}{out} = math.fma %x_{t}, %y_{t}, {sum_in} : {acc}")

    w("      // four elements a lane at a time (four loads of each operand in flight), while k + 96 < K")
    w("      %K96 = arith.maxui %K, %i96 : index")
    w("      %ub4 = arith.subi %K96, %i96 : index")
    w("      %s4:4 = scf.for %k = %lane to %ub4 step %i128")
    w(f"          iter_args(%c0_ = %f0, %c1_ = %f0, %c2_ = %f0, %c3_ = %f0) -> ({acc}, {acc}, {acc}, {acc}) {{")
    ks = ["%k"]
    for q in range(1, 4):
        w(f"        %k{q} = arith.addi {ks[-1]}, %i32 : index")
        ks.append(f"%k{q}")
    for q in range(4):
        term(ks[q], f"%c{q}_", f"%d{q}", "        ")
    w(f"        scf.yield %d0, %d1, %d2, %d3 : {acc}, {acc}, {acc}, {acc}")
    w("      }")
    w("      // the rest, one element a lane at a time, from the first k the loop above did not reach")
    w("      %after = arith.cmpi ugt, %ub4, %lane : index")
    w("      %span = arith.subi %ub4, %lane : index")
    w("      %span1 = arith.addi %span, %i127 : index")
    w("      %trips = arith.divui %span1, %i128 : index")
    w("      %done = arith.muli %trips, %i128 : index")
    w("      %krest0 = arith.addi %lane, %done : index")
    w("      %krest = arith.select %after, %krest0, %lane : index")
    w(f"      %s01 = arith.addf %s4#0, %s4#1 : {acc}")
    w(f"      %s23 = arith.addf %s4#2, %s4#3 : {acc}")
    w(f"      %sum0 = arith.addf %s01, %s23 : {acc}")
    w(f"      %part = scf.for %k = %krest to %K step %i32 iter_args(%a = %sum0) -> {acc} {{")
    term("%k", "%a", "%a1", "        ")
    w(f"        scf.yield %a1 : {acc}")
    w("      }")
    w(f"      %sum = gpu.subgroup_reduce add %part : ({acc}) -> {acc}")
    w("      %first = arith.cmpi eq, %lane, %i0 : index")
    w("      scf.if %first {")
    w("        %ic = arith.muli %i, %scr : index")
    w("        %jc = arith.muli %j, %scc : index")
    w("        %pc = arith.addi %ic, %jc : index")
    w(f"        %scaled = arith.mulf %sum, %alpha : {acc}")
    w(f"        %no_c = arith.cmpf oeq, %beta, %f0 : {acc}")
    w(f"        %out = scf.if %no_c -> {acc} {{")
    w(f"          scf.yield %scaled : {acc}")
    w("        } else {")
    if c == acc:
        w(f"          %old = memref.load %C[%pc] : {mem(c)}")
    else:
        w(f"          %oldn = memref.load %C[%pc] : {mem(c)}")
        w(f"          %old = arith.extf %oldn : {c} to {acc}")
    w(f"          %r = math.fma %old, %beta, %scaled : {acc}")
    w(f"          scf.yield %r : {acc}")
    w("        }")
    if c == acc:
        w(f"        memref.store %out, %C[%pc] : {mem(c)}")
    else:
        w(f"        %outn = arith.truncf %out : {acc} to {c}")
        w(f"        memref.store %outn, %C[%pc] : {mem(c)}")
    w("      }")
    w("    }")
    w("    gpu.return")
    w("  }")
    return o


def main():
    for path in sys.argv[1:]:
        base = path.rsplit("/", 1)[-1]
        L = open(path).read().split("\n")
        for name, ab, c, acc in KERNELS[base]:
            body = kernel(name, ab, c, acc)
            start = next((i for i, l in enumerate(L) if re.match(r"  gpu\.func @%s\(" % name, l)), None)
            if start is None:
                end = max(i for i, l in enumerate(L) if l == "}")  # the module's end
                doc = ["", f"  // As agx_{name.split('_')[1]}_generic{name[len('agx_xgemm_dot'):]}, with a SIMD-group per "
                           "element of C whose lanes split K",
                       "  // (gen_dot_gemm.py): for C with a side of at most 8, A or B contiguous along K."]
                L[end:end] = doc + body
            else:
                end = next(i for i in range(start, len(L)) if L[i] == "  }")
                L[start:end + 1] = body
        open(path, "w").write("\n".join(L))


if __name__ == "__main__":
    main()
