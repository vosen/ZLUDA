#!/usr/bin/env python3
"""Generates the generic and dot GEMM kernels of sgemm.mlir and hgemm.mlir. The generic kernels compute C = alpha op(A)
op(B) + beta C with arbitrary element strides, a thread per element of C, its sum four steps of K at a time (four
loads of each operand in flight). The dot kernels take the same arguments, with a SIMD-group per
element of C instead of a thread. Its lanes split K (four elements a lane at a time, then one) and sum by a SIMD-group
reduction, so a row of op(A) or a column of op(B) that is contiguous along K is read by consecutive lanes. The plan
uses them for C with a side of at most 8 (GEMV-shaped products, as in decoding). Run it after changing the kernel:

  gen_small_gemm.py sgemm.mlir hgemm.mlir

Each gpu.func @agx_[sdh]gemm_{generic,dot}* of the files is replaced in place (the comments before it stay); missing kernels
are appended at the end of the module.
"""

import re
import sys

KERNELS = {  # file: [(name, A and B type, C type, accumulator and scalar type)]
    "sgemm.mlir": [("agx_sgemm_generic", "f32", "f32", "f32"), ("agx_dgemm_generic", "f64", "f64", "f64"),
                   ("agx_sgemm_dot", "f32", "f32", "f32"), ("agx_dgemm_dot", "f64", "f64", "f64")],
    "hgemm.mlir": [("agx_hgemm_generic_cf16", "f16", "f16", "f32"), ("agx_hgemm_generic_cf32", "f16", "f32", "f32"),
                   ("agx_hgemm_dot_cf16", "f16", "f16", "f32"), ("agx_hgemm_dot_cf32", "f16", "f32", "f32")],
}


def kernel(name, ab, c, acc):
    dot = "_dot" in name
    mem = lambda t: f"memref<?x{t}, strided<[1], offset: ?>, 1>"
    o = []
    w = o.append
    w(f"  gpu.func @{name}(%a0: memref<1x{ab}, 1>, %b0: memref<1x{ab}, 1>,")
    w(f"      %c0: memref<1x{c}, 1>, %M: index, %N: index, %K: index, %sar: index, %sac: index,")
    w(f"      %sbr: index, %sbc: index, %scr: index, %scc: index, %sa: index, %sb: index, %sc: index,")
    w(f"      %alpha: {acc}, %beta: {acc}) kernel {{")
    for v in [0, 1, 3, 4, 32, 96, 127, 128]:
        w(f"    %i{v} = arith.constant {v} : index")
    w("    %big = arith.constant 4611686018427387904 : index")
    # the matrices of batch entry z (grid y for dot, z for generic) are sa, sb and sc elements on
    w(f"    %z = gpu.block_id {'y' if dot else 'z'}")
    for n in ["a", "b", "c"]:
        w(f"    %o{n} = arith.muli %z, %s{n} : index")
    for n, t in [("A", ab), ("B", ab), ("C", c)]:
        w(f"    %{n} = memref.reinterpret_cast %{n.lower()}0 to offset: [%o{n.lower()}], sizes: [%big], strides: [1]")
        w(f"        : memref<1x{t}, 1> to {mem(t)}")
    w(f"    %f0 = arith.constant 0.0 : {acc}")
    if dot:
        w("    %g = gpu.global_id x")
        w("    %s = arith.divui %g, %i32 : index")
        w("    %lane = gpu.lane_id")
        w("    %MN = arith.muli %M, %N : index")
        w("    // the same for the whole SIMD-group, which reduces together")
        w("    %live = arith.cmpi ult, %s, %MN : index")
        w("    scf.if %live {")
        w("      %i = arith.divui %s, %N : index")
        w("      %j = arith.remui %s, %N : index")
    else:
        w("    %j = gpu.global_id x")
        w("    %i = gpu.global_id y")
        w("    %in_m = arith.cmpi ult, %i, %M : index")
        w("    %in_n = arith.cmpi ult, %j, %N : index")
        w("    %live = arith.andi %in_m, %in_n : i1")
        w("    scf.if %live {")
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

    # dot: lane l takes k = l, l + 32, ... (four at a time while k + 96 < K); generic: k = 0, 1, ... (four at a time
    # while k + 3 < K)
    first, step, lead, stride = ("%lane", "%i128", "%i96", "%i32") if dot else ("%i0", "%i4", "%i3", "%i1")
    w("      // four steps at a time (four loads of each operand in flight), then one")
    w(f"      %Kx = arith.maxui %K, {lead} : index")
    w(f"      %ub4 = arith.subi %Kx, {lead} : index")
    w(f"      %s4:4 = scf.for %k = {first} to %ub4 step {step}")
    w(f"          iter_args(%c0_ = %f0, %c1_ = %f0, %c2_ = %f0, %c3_ = %f0) -> ({acc}, {acc}, {acc}, {acc}) {{")
    ks = ["%k"]
    for q in range(1, 4):
        w(f"        %k{q} = arith.addi {ks[-1]}, {stride} : index")
        ks.append(f"%k{q}")
    for q in range(4):
        term(ks[q], f"%c{q}_", f"%d{q}", "        ")
    w(f"        scf.yield %d0, %d1, %d2, %d3 : {acc}, {acc}, {acc}, {acc}")
    w("      }")
    w("      // the rest from the first k the loop above did not reach")
    w(f"      %after = arith.cmpi ugt, %ub4, {first} : index")
    w(f"      %span = arith.subi %ub4, {first} : index")
    w(f"      %span1 = arith.addi %span, {'%i127' if dot else '%i3'} : index")
    w(f"      %trips = arith.divui %span1, {step} : index")
    w(f"      %done = arith.muli %trips, {step} : index")
    w(f"      %krest0 = arith.addi {first}, %done : index")
    w(f"      %krest = arith.select %after, %krest0, {first} : index")
    w(f"      %s01 = arith.addf %s4#0, %s4#1 : {acc}")
    w(f"      %s23 = arith.addf %s4#2, %s4#3 : {acc}")
    w(f"      %sum0 = arith.addf %s01, %s23 : {acc}")
    w(f"      %part = scf.for %k = %krest to %K step {stride} iter_args(%a = %sum0) -> {acc} {{")
    term("%k", "%a", "%a1", "        ")
    w(f"        scf.yield %a1 : {acc}")
    w("      }")
    # dot: the sum of the lanes, written by lane 0
    if dot:
        w(f"      %sum = gpu.subgroup_reduce add %part : ({acc}) -> {acc}")
        w("      %first = arith.cmpi eq, %lane, %i0 : index")
        w("      scf.if %first {")
    ind = "        " if dot else "      "
    sum_ = "%sum" if dot else "%part"
    w(f"{ind}%ic = arith.muli %i, %scr : index")
    w(f"{ind}%jc = arith.muli %j, %scc : index")
    w(f"{ind}%pc = arith.addi %ic, %jc : index")
    w(f"{ind}%scaled = arith.mulf {sum_}, %alpha : {acc}")
    w(f"{ind}%no_c = arith.cmpf oeq, %beta, %f0 : {acc}")
    w(f"{ind}%out = scf.if %no_c -> {acc} {{")
    w(f"{ind}  scf.yield %scaled : {acc}")
    w(f"{ind}}} else {{")
    if c == acc:
        w(f"{ind}  %old = memref.load %C[%pc] : {mem(c)}")
    else:
        w(f"{ind}  %oldn = memref.load %C[%pc] : {mem(c)}")
        w(f"{ind}  %old = arith.extf %oldn : {c} to {acc}")
    w(f"{ind}  %r = math.fma %old, %beta, %scaled : {acc}")
    w(f"{ind}  scf.yield %r : {acc}")
    w(f"{ind}}}")
    if c == acc:
        w(f"{ind}memref.store %out, %C[%pc] : {mem(c)}")
    else:
        w(f"{ind}%outn = arith.truncf %out : {acc} to {c}")
        w(f"{ind}memref.store %outn, %C[%pc] : {mem(c)}")
    if dot:
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
                       "  // (gen_small_gemm.py): for C with a side of at most 8, A or B contiguous along K."]
                L[end:end] = doc + body
            else:
                end = next(i for i in range(start, len(L)) if L[i] == "  }")
                L[start:end + 1] = body
        open(path, "w").write("\n".join(L))


if __name__ == "__main__":
    main()
