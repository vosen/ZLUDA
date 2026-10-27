#!/usr/bin/env python3
"""Run the GEMM kernels of a fat AGX object (agx-build-fatbin.py output) on the AGX Performance Simulator's agx_run
and compare them with a Python reference, for each chip in the object (or --chips).

  test_kernels.py --agx-run AGX_RUN --objcopy LLVM_OBJCOPY --readelf LLVM_READELF sgemm.agxfat [--chips agx-g14c]

Entries named CHIP+agx-strict (agx-build-fatbin.py's strict builds) run under agx_run's strict check
(AGX_STRICT=CHIP).

Exits with 1 if any case is wrong.
"""

import argparse
import math
import os
import re
import random
import struct
import subprocess
import sys
import tempfile

OUT, IN_A, IN_B = 0x1000000, 0x2000000, 0x3000000


def fat_entries(data):
    if data[:8] != b"ZAGXFAT1":
        sys.exit("not a fat AGX object")
    (n,) = struct.unpack_from("<I", data, 8)
    out = {}
    for i in range(n):
        name, off, size = struct.unpack_from("<32sQQ", data, 16 + 48 * i)
        out[name.rstrip(b"\0").decode()] = data[off:off + size]
    return out


def kernel_records(obj, a, tmp):
    """name -> (threadgroups_per_grid uniform, [(kind, bytes, first half) per argument]) from .agx.kernels."""
    p = os.path.join(tmp, "o.o")
    open(p, "wb").write(obj)
    sec = os.path.join(tmp, "k.bin")
    subprocess.run([a.objcopy, "--dump-section", ".agx.kernels=" + sec, p, os.path.join(tmp, "unused.o")],
                   check=True)
    d = open(sec, "rb").read()
    out, pos = {}, 0
    while pos + 8 <= len(d):
        size, version = struct.unpack_from("<II", d, pos)
        if size == 0:
            break
        (nlen,) = struct.unpack_from("<I", d, pos + 8)
        name = d[pos + 12:pos + 12 + nlen].decode()
        q = pos + 12 + (nlen + 3) // 4 * 4
        _halves, tg_half, glob, _gprs, _tg, stack, nargs = struct.unpack_from("<7I", d, q)
        q += 28
        args = [struct.unpack_from("<3I", d, q + 12 * i) for i in range(nargs)]
        out[name] = (tg_half // 2, args, glob, stack)
        pos += size
    return out


def kernel_code(obj, name, a, tmp):
    p = os.path.join(tmp, "o.o")
    open(p, "wb").write(obj)
    text = os.path.join(tmp, "t.bin")
    subprocess.run([a.objcopy, "-O", "binary", "--only-section=.text", p, text], check=True)
    code = open(text, "rb").read()
    syms = subprocess.run([a.readelf, "-sW", p], capture_output=True, text=True, check=True).stdout
    for line in syms.splitlines():
        f = line.split()
        if len(f) >= 8 and f[3] == "FUNC" and f[7] == name:
            start = int(f[1], 16)
            return code[start:]
    sys.exit("no kernel " + name)


HEAP_VA = 0x50000000


def run(a, obj, name, args, grid, wg, tmp, heap=0):
    """args: ('buf', bytes, dump) | ('i64', v) | ('f32', v). heap: the device heap of malloc / free, its size or
    its contents (a heap returned by an earlier run); its address goes into the module's __zluda_agx_heap, as
    zluda_agx does, and its contents after the run follow the dumped buffers. Returns the dumped buffers."""
    tg_uniform, slots, glob, stack = kernel_records(obj, a, tmp)[name]
    code = os.path.join(tmp, name + ".bin")
    open(code, "wb").write(kernel_code(obj, name, a, tmp))
    grid3 = tuple(grid) + (1,) * (3 - len(grid))  # a third dimension for batched launches
    cmd = [a.agx_run, code, "--grid", "%d,%d,%d" % grid3, "--wg", "%d,%d,1" % wg]
    # on the GPU, a kernel that uses the stack needs its scratch (the simulator gives every thread one)
    if stack and os.path.basename(a.agx_run).startswith("agx_run_hw"):
        cmd += ["--stack", str(stack)]
    uniforms, dumps, bases = {}, [], [0x1000000 * (k + 1) for k in range(16)][::-1]
    for (kind, value, *rest), (_k, nbytes, half) in zip(args, slots):
        u = half // 2
        if kind == "buf":
            base = bases.pop()
            f = os.path.join(tmp, "b%x.bin" % base)
            open(f, "wb").write(value)
            cmd += ["--map", "%#x=%s" % (base, f)]
            if rest and rest[0]:
                o = f + ".out"
                cmd += ["--dump", "%#x=%d:%s" % (base, len(value), o)]
                dumps.append(o)
            uniforms[u], uniforms[u + 1] = base, 0
        elif kind == "i64":
            uniforms[u], uniforms[u + 1] = value & 0xFFFFFFFF, (value >> 32) & 0xFFFFFFFF
        elif kind == "i32":
            uniforms[u] = value & 0xFFFFFFFF
        elif kind == "f64":
            bits = struct.unpack("<Q", struct.pack("<d", value))[0]
            uniforms[u], uniforms[u + 1] = bits & 0xFFFFFFFF, bits >> 32
        else:
            uniforms[u] = struct.unpack("<I", struct.pack("<f", value))[0]
    if glob != 0xFFFFFFFF:
        # the module's global data (e.g. the lock table of 64-bit float atomics)
        p = os.path.join(tmp, "o.o")
        open(p, "wb").write(obj)
        gfile = os.path.join(tmp, "globals.bin")
        subprocess.run([a.objcopy, "-O", "binary", "--only-section=.agx.globals", p, gfile], check=True)
        g = bytearray(open(gfile, "rb").read())
        if heap:
            syms = subprocess.run([a.readelf, "-s", p], capture_output=True, text=True, check=True).stdout
            m = re.search(r"^\s*\d+:\s+([0-9a-f]+)\s+\d+\s+OBJECT\s+\S+\s+\S+\s+\d+\s+__zluda_agx_heap$", syms, re.M)
            if m:
                off = int(m.group(1), 16)
                g[off:off + 8] = struct.pack("<Q", HEAP_VA)
                hfile = os.path.join(tmp, "heap.bin")
                data = heap if isinstance(heap, bytes) else struct.pack("<II", 0, heap).ljust(heap, b"\0")
                open(hfile, "wb").write(data)
                cmd += ["--map", "%#x=%s" % (HEAP_VA, hfile), "--dump", "%#x=%d:%s" % (HEAP_VA, len(data), hfile + ".out")]
                dumps.append(hfile + ".out")
        open(gfile, "wb").write(bytes(g).ljust(16, b"\0"))
        cmd += ["--map", "%#x=%s" % (0x40000000, gfile)]
        uniforms[glob // 2], uniforms[glob // 2 + 1] = 0x40000000, 0
    for d in range(3):
        g = (grid[d] if d < 2 else 1)
        w = (wg[d] if d < 2 else 1)
        uniforms[tg_uniform + d] = -(-g // w)
    for u, v in sorted(uniforms.items()):
        cmd += ["--uniform", "%d=%d" % (u, v)]
    r = subprocess.run(cmd, capture_output=True, text=True, timeout=1800)
    if r.returncode:
        raise RuntimeError("agx_run: " + (r.stdout + r.stderr)[-400:])
    return [open(o, "rb").read() for o in dumps]


def pack(xs, t="f"):
    return struct.pack("<%d%s" % (len(xs), t), *xs)


def unpack(b, t="f"):
    return struct.unpack("<%d%s" % (len(b) // struct.calcsize(t), t), b)


def rounded(xs, t):
    """The values as stored in type t ("f" f32, "e" f16)."""
    return list(unpack(pack(xs, t), t))


def reference(M, N, K, A, sa, B, sb, C, sc, alpha, beta):
    out = list(C)
    for i in range(M):
        for j in range(N):
            s = sum(A[i * sa[0] + k * sa[1]] * B[k * sb[0] + j * sb[1]] for k in range(K))
            p = i * sc[0] + j * sc[1]
            out[p] = alpha * s + (beta * C[p] if beta != 0 else 0.0)
    return out


def close(got, want, idx, tol=1e-4):
    bad = [p for p in idx if not (abs(got[p] - want[p]) <= tol * (1 + abs(want[p])))]
    return bad


def case_tile(a, obj, tmp, M, N, K, alpha, beta, rnd, kernel="agx_sgemm_tile64", ab="f", c="f", ta=False, tb=False):
    # the _ta / _tb variants read A[r, k] at a[k lda + r] and B[k, c] at b[c ldb + k]
    kernel += {(0, 0): "", (1, 0): "_ta", (0, 1): "_tb", (1, 1): "_tatb"}[(int(ta), int(tb))]
    lda, ldb, ldc = (M if ta else K) + 2, (K if tb else N) + 2, N + 4
    sa, sb = ((1, lda) if ta else (lda, 1)), ((1, ldb) if tb else (ldb, 1))
    A = rounded([rnd.uniform(-1, 1) for _ in range((K if ta else M) * lda)], ab)
    B = rounded([rnd.uniform(-1, 1) for _ in range((N if tb else K) * ldb)], ab)
    C = rounded([rnd.uniform(-1, 1) if beta != 0 else math.nan for _ in range(M * ldc)], c)
    (got,) = run(a, obj, kernel,
                 [("buf", pack(A, ab)), ("buf", pack(B, ab)), ("buf", pack(C, c), True), ("i64", M), ("i64", N),
                  ("i64", K), ("i64", lda), ("i64", ldb), ("i64", ldc), ("i64", 0), ("i64", 0), ("i64", 0),
                  ("f32", alpha), ("f32", beta)],
                 ((M // 64) * (N // 64) * 128, 1), (128, 1), tmp)
    want = reference(M, N, K, A, sa, B, sb, C, (ldc, 1), alpha, beta)
    return close(unpack(got, c), want, [i * ldc + j for i in range(M) for j in range(N)], 1e-4 if c == "f" else 2e-3)


def case_generic(a, obj, tmp, M, N, K, ta, tb, alpha, beta, rnd, kernel="agx_sgemm_generic", ab="f", c="f"):
    # column-major storage as cuBLAS has it, with op(X) = X or X^T
    lda = (K if ta else M) + 1
    ldb = (N if tb else K) + 3
    ldc = M + 2
    sa = (lda, 1) if ta else (1, lda)  # (row stride, column stride) of op(A)
    sb = (ldb, 1) if tb else (1, ldb)
    sc = (1, ldc)
    A = rounded([rnd.uniform(-1, 1) for _ in range(lda * (M if ta else K))], ab)
    B = rounded([rnd.uniform(-1, 1) for _ in range(ldb * (K if tb else N))], ab)
    C = rounded([rnd.uniform(-1, 1) if beta != 0 else math.nan for _ in range(ldc * N)], c)
    (got,) = run(a, obj, kernel,
                 [("buf", pack(A, ab)), ("buf", pack(B, ab)), ("buf", pack(C, c), True), ("i64", M), ("i64", N),
                  ("i64", K), ("i64", sa[0]), ("i64", sa[1]), ("i64", sb[0]), ("i64", sb[1]), ("i64", sc[0]),
                  ("i64", sc[1]), ("i64", 0), ("i64", 0), ("i64", 0), ("f64" if c == "d" else "f32", alpha),
                  ("f64" if c == "d" else "f32", beta)],
                 *gemm_launch(kernel, M, N), tmp)
    want = reference(M, N, K, A, sa, B, sb, C, sc, alpha, beta)
    return close(unpack(got, c), want, [i * sc[0] + j * sc[1] for i in range(M) for j in range(N)],
                 {"f": 1e-4, "d": 1e-13}.get(c, 2e-3))


DOT_SIDE = 8


def use_dot(m, n, k, ta, tb):
    """agx_libs' choice of the dot kernel over the generic one (generic in agx_libs/src/lib.rs)."""
    return k >= 32 and ((n <= DOT_SIDE and ta) or (m <= DOT_SIDE and not tb))


def gemm_launch(kernel, M, N):
    """Grid and threadgroup of the generic kernel (a thread per element of C) or the dot kernel (a SIMD-group)."""
    if "_dot" in kernel:
        return (-(-M * N * 32 // 128) * 128, 1), (128, 1)
    return (-(-N // 16) * 16, -(-M // 8) * 8), (16, 8)


def split_plan(M, N, K, lda, ldb, ldc, ta=False, tb=False):
    """zluda_blas_agx's plan for column-major C = alpha op(A) op(B) + beta C (agx_libs/src/lib.rs, plan): the tile
    kernel on the largest block of C with sides multiples of 64 along the largest multiple of 8 of K, the generic
    kernel on the columns right of it, the rows below it, then the remaining steps of K added to all of C. Each part:
    (tile, (m, n, k), element offsets of A, B and C, accumulate)."""
    m64, n64, k8 = M // 64 * 64, N // 64 * 64, K // 8 * 8
    if not (m64 and n64 and k8):
        return [(False, (M, N, K), (0, 0, 0), False)]
    col_b = lambda j: j if tb else j * ldb
    row_a = lambda i: i * lda if ta else i
    step_a, step_b = (lambda l: l if ta else l * lda), (lambda l: l * ldb if tb else l)
    parts = [(True, (m64, n64, k8), (0, 0, 0), False)]
    if n64 < N:
        parts.append((False, (M, N - n64, k8), (0, col_b(n64), n64 * ldc), False))
    if m64 < M:
        parts.append((False, (M - m64, n64, k8), (row_a(m64), 0, m64), False))
    if k8 < K:
        parts.append((False, (M, N, K - k8), (step_a(k8), step_b(k8), 0), True))
    return parts


def run_gemm_plan(a, obj, tmp, M, N, K, A, lda, B, ldb, C, ldc, alpha, beta, tile, gen, ab, c, ta, tb,
                  batch=1, strides=(0, 0, 0)):
    """Column-major C = alpha op(A) op(B) + beta C as zluda_blas_agx runs it (split_plan's parts, each launched once
    for `batch` matrices strides (A, B, C) elements apart); returns C."""
    sa = (lda, 1) if ta else (1, lda)  # (row stride, column stride) of op(A)
    sb = (ldb, 1) if tb else (1, ldb)
    C = list(C)
    for is_tile, (m, n, k), (oa, ob, oc), acc in split_plan(M, N, K, lda, ldb, ldc, ta, tb):
        be = 1.0 if acc else beta
        if is_tile:
            # row-major C^T = op(B)^T op(A)^T: A' = B, read transposed when op(B) is, B' = A likewise
            # the _u kernel unless the leading dimensions and strides are even (the buffers here are aligned)
            aligned = all(v % 2 == 0 for v in (lda, ldb, ldc) + tuple(strides))
            kernel = tile + ("" if aligned else "_u") + {(0, 0): "", (1, 0): "_ta", (0, 1): "_tb",
                                                          (1, 1): "_tatb"}[(int(tb), int(ta))]
            args = [("buf", pack(B[ob:], ab)), ("buf", pack(A[oa:], ab)), ("buf", pack(C[oc:], c), True),
                    ("i64", n), ("i64", m), ("i64", k), ("i64", ldb), ("i64", lda), ("i64", ldc),
                    ("i64", strides[1]), ("i64", strides[0]), ("i64", strides[2]), ("f32", alpha), ("f32", be)]
            (got,) = run(a, obj, kernel, args, ((n // 64) * (m // 64) * 128, batch), (128, 1), tmp)
        else:
            args = [("buf", pack(A[oa:], ab)), ("buf", pack(B[ob:], ab)), ("buf", pack(C[oc:], c), True),
                    ("i64", m), ("i64", n), ("i64", k), ("i64", sa[0]), ("i64", sa[1]), ("i64", sb[0]),
                    ("i64", sb[1]), ("i64", 1), ("i64", ldc), ("i64", strides[0]), ("i64", strides[1]),
                    ("i64", strides[2]), ("f32", alpha), ("f32", be)]
            kernel = gen.replace("_generic", "_dot") if use_dot(m, n, k, ta, tb) else gen
            (gx, gy), wg = gemm_launch(kernel, m, n)
            grid = (gx, batch) if "_dot" in kernel else (gx, gy, batch)
            (got,) = run(a, obj, kernel, args, grid, wg, tmp)
        C[oc:] = unpack(got, c)
    return C


def case_batched(a, obj, tmp, M, N, K, batch, alpha, beta, rnd, tile="agx_sgemm_tile64", gen="agx_sgemm_generic",
                 ab="f", c="f", ta=False, tb=False):
    """A strided batched GEMM, each part of its plan launched once for the whole batch."""
    ra, rb = (K if ta else M), (N if tb else K)
    lda, ldb, ldc = ra + 2 + ra % 2, rb + 2 + rb % 2, M + 4 + M % 2
    na, nb, nc = lda * (M if ta else K), ldb * (K if tb else N), ldc * N
    strides = (na + 6, nb + 4, nc + 2)  # even, with gaps between the matrices
    A = rounded([rnd.uniform(-1, 1) for _ in range(strides[0] * batch)], ab)
    B = rounded([rnd.uniform(-1, 1) for _ in range(strides[1] * batch)], ab)
    C0 = rounded([rnd.uniform(-1, 1) if beta != 0 else math.nan for _ in range(strides[2] * batch)], c)
    C = run_gemm_plan(a, obj, tmp, M, N, K, A, lda, B, ldb, C0, ldc, alpha, beta, tile, gen, ab, c, ta, tb, batch,
                      strides)
    sa = (lda, 1) if ta else (1, lda)
    sb = (ldb, 1) if tb else (1, ldb)
    got, want = [], []
    for e in range(batch):
        w = reference(M, N, K, A[e * strides[0]:], sa, B[e * strides[1]:], sb, C0[e * strides[2]:], (1, ldc), alpha,
                      beta)
        idx = [i + j * ldc for i in range(M) for j in range(N)]
        got += [C[e * strides[2] + i] for i in idx]
        want += [w[i] for i in idx]
    return close(got, want, range(len(got)), 1e-4 if c == "f" else 2e-3)


def case_split(a, obj, tmp, M, N, K, alpha, beta, rnd, tile="agx_sgemm_tile64", gen="agx_sgemm_generic", ab="f",
               c="f", ta=False, tb=False, odd=False):
    # column-major op(A) and op(B), the leading dimensions even (the aligned tile kernel), or odd (the _u one)
    ra, rb = (K if ta else M), (N if tb else K)
    lda, ldb, ldc = ra + 2 + ra % 2, rb + 2 + rb % 2, M + 4 + M % 2
    if odd:
        lda, ldb, ldc = lda + 1, ldb + 1, ldc + 1
    sa = (lda, 1) if ta else (1, lda)  # (row stride, column stride) of op(A)
    sb = (ldb, 1) if tb else (1, ldb)
    A = rounded([rnd.uniform(-1, 1) for _ in range(lda * (M if ta else K))], ab)
    B = rounded([rnd.uniform(-1, 1) for _ in range(ldb * (K if tb else N))], ab)
    C0 = rounded([rnd.uniform(-1, 1) if beta != 0 else math.nan for _ in range(ldc * N)], c)
    C = run_gemm_plan(a, obj, tmp, M, N, K, A, lda, B, ldb, C0, ldc, alpha, beta, tile, gen, ab, c, ta, tb)
    want = reference(M, N, K, A, sa, B, sb, C0, (1, ldc), alpha, beta)
    return close(C, want, [i + j * ldc for i in range(M) for j in range(N)], 1e-4 if c == "f" else 2e-3)


def case_conv_pointwise(a, obj, tmp, n, ch, h, w, k, alpha, beta, rnd, tile="agx_sgemm_tile64",
                        gen="agx_sgemm_generic", t="f"):
    """A 1x1 NCHW convolution as zluda_dnn_agx runs it (pointwise_gemm): per image, y^T (HW x K, ld HW) =
    x^T (HW x C, ld HW) w^T (C x K, ld C), against the direct sum."""
    hw = h * w
    X = rounded([rnd.uniform(-1, 1) for _ in range(n * ch * hw)], t)
    W = rounded([rnd.uniform(-1, 1) for _ in range(k * ch)], t)
    Y0 = rounded([rnd.uniform(-1, 1) if beta != 0 else math.nan for _ in range(n * k * hw)], t)
    got, want = [], []
    for b in range(n):
        xb, yb = X[b * ch * hw:(b + 1) * ch * hw], Y0[b * k * hw:(b + 1) * k * hw]
        got += run_gemm_plan(a, obj, tmp, hw, k, ch, xb, hw, W, ch, yb, hw, alpha, beta, tile, gen, t, t, False, False)
        for kk in range(k):
            for p in range(hw):
                v = alpha * sum(W[kk * ch + cc] * xb[cc * hw + p] for cc in range(ch))
                want.append(v + (beta * yb[kk * hw + p] if beta != 0 else 0.0))
    return close(got, want, range(len(want)), 1e-4 if t == "f" else 2e-3)


def gelu(x):
    return 0.5 * x * (1 + math.tanh(0.7978845608 * (x + 0.044715 * x ** 3)))


def case_epilogue(a, obj, tmp, M, N, has_bias, act, rnd, t="f", batch=1):
    """batch > 1: that many matrices (and biases) with gaps between them, in one launch."""
    ld = M + 3
    stride, bstride = ld * N + 5, M + 2
    D = rounded([rnd.uniform(-3, 3) for _ in range(stride * batch)], t)
    bias = rounded([rnd.uniform(-1, 1) for _ in range(bstride * batch)], t)
    kernel = "agx_epilogue_f32" if t == "f" else "agx_epilogue_f16"
    (got,) = run(a, obj, kernel,
                 [("buf", pack(D, t), True), ("buf", pack(bias, t)), ("i64", M), ("i64", N), ("i64", ld),
                  ("i64", has_bias), ("i64", act), ("i64", stride if batch > 1 else 0),
                  ("i64", bstride if batch > 1 else 0)],
                 (-(-M // 16) * 16, -(-N // 8) * 8, batch), (16, 8), tmp)
    want = list(D)
    idx = []
    for e in range(batch):
        for j in range(N):
            for i in range(M):
                p = e * stride + j * ld + i
                x = D[p] + (bias[e * bstride + i] if has_bias else 0.0)
                want[p] = max(x, 0.0) if act == 1 else gelu(x) if act == 2 else x
                idx.append(p)
    return close(unpack(got, t), want, idx, 2e-5 if t == "f" else 2e-3)


class Conv:
    """A convolution problem with NCHW x (strides xs) and y (strides ys) and a packed K x C/G x R x S filter."""

    def __init__(self, N, C, H, W, K, R, S, pad, stride, dil, G, flip, x_w_pad=0):
        self.__dict__.update(locals())
        self.P = (H + 2 * pad[0] - dil[0] * (R - 1) - 1) // stride[0] + 1
        self.Q = (W + 2 * pad[1] - dil[1] * (S - 1) - 1) // stride[1] + 1
        Wx = W + x_w_pad  # x rows padded to show that strides are honoured
        self.xs = (C * H * Wx, H * Wx, Wx, 1)
        self.ys = (K * self.P * self.Q, self.P * self.Q, self.Q, 1)
        self.xlen, self.ylen, self.wlen = N * C * H * Wx, N * K * self.P * self.Q, K * (C // G) * R * S

    def args(self):
        c = self
        return [("i32", v) for v in (c.N, c.C, c.H, c.W, c.K, c.R, c.S, c.P, c.Q, c.pad[0], c.pad[1], c.stride[0],
                                     c.stride[1], c.dil[0], c.dil[1], c.G, c.flip) + c.xs + c.ys]

    def taps(self):
        """(n, k, p, q, c, cc, r, s, h, w, wi) of every product term of the forward convolution."""
        c, cpg, kpg = self, self.C // self.G, self.K // self.G
        for n in range(c.N):
            for k in range(c.K):
                g = k // kpg
                for p in range(c.P):
                    for q in range(c.Q):
                        for cc in range(cpg):
                            for r in range(c.R):
                                for s_ in range(c.S):
                                    h = p * c.stride[0] - c.pad[0] + r * c.dil[0]
                                    w = q * c.stride[1] - c.pad[1] + s_ * c.dil[1]
                                    if 0 <= h < c.H and 0 <= w < c.W:
                                        rf, sf = (c.R - 1 - r, c.S - 1 - s_) if c.flip else (r, s_)
                                        wi = ((k * cpg + cc) * c.R + rf) * c.S + sf
                                        yield n, k, p, q, g * cpg + cc, r, s_, h, w, wi

    def xi(self, n, c, h, w):
        return n * self.xs[0] + c * self.xs[1] + h * self.xs[2] + w * self.xs[3]

    def yi(self, n, k, p, q):
        return n * self.ys[0] + k * self.ys[1] + p * self.ys[2] + q * self.ys[3]


def case_conv(a, obj, tmp, which, conv, alpha, beta, rnd, t="f"):
    c = conv
    X = rounded([rnd.uniform(-1, 1) for _ in range(c.xlen)], t)
    Wt = rounded([rnd.uniform(-1, 1) for _ in range(c.wlen)], t)
    Y = rounded([rnd.uniform(-1, 1) for _ in range(c.ylen)], t)
    T = "f32" if t == "f" else "f16"
    out_idx = {"fwd": 2, "bwd_data": 0, "bwd_filter": 1}[which]
    bufs = [X, Wt, Y]
    if beta == 0:
        bufs[out_idx] = [math.nan] * len(bufs[out_idx])
    total = {"fwd": c.ylen, "bwd_data": c.N * c.C * c.H * c.W, "bwd_filter": c.wlen}[which]
    args = [("buf", pack(b, t), i == out_idx) for i, b in enumerate(bufs)] + c.args() + [("f32", alpha), ("f32", beta)]
    (got,) = run(a, obj, "agx_conv2d_%s_%s" % (which, T), args, (-(-total // 256) * 256, 1), (256, 1), tmp)
    want = list(bufs[out_idx])
    acc = {}
    for n, k, p, q, ch, r, s_, h, w, wi in c.taps():
        if which == "fwd":
            key, v = c.yi(n, k, p, q), X[c.xi(n, ch, h, w)] * Wt[wi]
        elif which == "bwd_data":
            key, v = c.xi(n, ch, h, w), Y[c.yi(n, k, p, q)] * Wt[wi]
        else:
            key, v = wi, Y[c.yi(n, k, p, q)] * X[c.xi(n, ch, h, w)]
        acc[key] = acc.get(key, 0.0) + v
    if which == "fwd":
        idx = [c.yi(n, k, p, q) for n in range(c.N) for k in range(c.K) for p in range(c.P) for q in range(c.Q)]
    elif which == "bwd_data":
        idx = [c.xi(n, ch, h, w) for n in range(c.N) for ch in range(c.C) for h in range(c.H) for w in range(c.W)]
    else:
        idx = list(range(c.wlen))
    for i in idx:
        want[i] = alpha * acc.get(i, 0.0) + (beta * bufs[out_idx][i] if beta != 0 else 0.0)
    return close(unpack(got, t), want, idx, 1e-4 if t == "f" else 3e-3)


def case_conv_im2col(a, obj, tmp, conv, rnd, t="f", filter_grad=False):
    """The im2col matrix of each image (agx_conv2d_im2col_*), multiplied here as zluda_dnn_agx's GEMMs do: y[n] =
    w col (forward), or dw = sum over n of dy[n] col^T (filter gradient), against the direct sums (one group)."""
    c = conv
    X = rounded([rnd.uniform(-1, 1) for _ in range(c.xlen)], t)
    Wt = rounded([rnd.uniform(-1, 1) for _ in range(c.wlen)], t)
    Dy = rounded([rnd.uniform(-1, 1) for _ in range(c.ylen)], t)
    crs, pq = c.C * c.R * c.S, c.P * c.Q
    got = [0.0] * (c.wlen if filter_grad else c.ylen)
    for n in range(c.N):
        args = [("buf", pack(X, t)), ("buf", pack([math.nan] * (crs * pq), t), True)] + c.args()[:21] + [("i32", n)]
        (col,) = run(a, obj, "agx_conv2d_im2col_%s" % ("f32" if t == "f" else "f16"), args,
                     (-(-crs * pq // 256) * 256, 1), (256, 1), tmp)
        col = unpack(col, t)
        for k in range(c.K):
            if filter_grad:
                for j in range(crs):
                    got[k * crs + j] += sum(Dy[c.yi(n, k, i // c.Q, i % c.Q)] * col[j * pq + i] for i in range(pq))
            else:
                for i in range(pq):
                    got[c.yi(n, k, i // c.Q, i % c.Q)] = sum(Wt[k * crs + j] * col[j * pq + i] for j in range(crs))
    want = [0.0] * len(got)
    for n, k, p, q, ch, r, s_, h, w, wi in c.taps():
        if filter_grad:
            want[wi] += Dy[c.yi(n, k, p, q)] * X[c.xi(n, ch, h, w)]
        else:
            want[c.yi(n, k, p, q)] += X[c.xi(n, ch, h, w)] * Wt[wi]
    return close(got, want, range(len(got)), 1e-5)


def case_conv_col2im(a, obj, tmp, conv, alpha, beta, rnd, t="f"):
    """The data gradient as zluda_dnn_agx runs it for one group: col = w^T dy[n] (f32, formed here as its GEMM
    does), then agx_conv2d_col2im_* gathers dx[n] = alpha col2im(col) + beta dx[n]; against the direct sums."""
    c = conv
    Wt = rounded([rnd.uniform(-1, 1) for _ in range(c.wlen)], t)
    Dy = rounded([rnd.uniform(-1, 1) for _ in range(c.ylen)], t)
    Dx = rounded([rnd.uniform(-1, 1) if beta != 0 else math.nan for _ in range(c.xlen)], t)
    crs, pq = c.C * c.R * c.S, c.P * c.Q
    for n in range(c.N):
        col = [sum(Wt[k * crs + j] * Dy[c.yi(n, k, i // c.Q, i % c.Q)] for k in range(c.K))
               for j in range(crs) for i in range(pq)]
        args = ([("buf", pack(col, "f")), ("buf", pack(Dx, t), True)] + c.args()[:21] +
                [("i32", n), ("f32", alpha), ("f32", beta)])
        (dx,) = run(a, obj, "agx_conv2d_col2im_%s" % ("f32" if t == "f" else "f16"), args,
                    (-(-c.C * c.H * c.W // 256) * 256, 1), (256, 1), tmp)
        Dx_n = unpack(dx, t)
        if n == 0:
            got = list(Dx_n)
        else:
            lo, hi = n * c.xs[0], (n + 1) * c.xs[0]
            got[lo:hi] = Dx_n[lo:hi]
    acc = {}
    for n, k, p, q, ch, r, s_, h, w, wi in c.taps():
        key = c.xi(n, ch, h, w)
        acc[key] = acc.get(key, 0.0) + Dy[c.yi(n, k, p, q)] * Wt[wi]
    idx = [c.xi(n, ch, h, w) for n in range(c.N) for ch in range(c.C) for h in range(c.H) for w in range(c.W)]
    want = list(Dx)
    for i in idx:
        want[i] = alpha * acc.get(i, 0.0) + (beta * Dx[i] if beta != 0 else 0.0)
    return close(got, want, idx, 1e-5 if t == "f" else 3e-3)


def twiddles(n):
    out = []
    for t in range(n):
        a = -2 * math.pi * t / n
        out += [math.cos(a), math.sin(a)]
    return out


def dft(x, n, inverse):
    sign = 1 if inverse else -1
    return [sum(x[j] * complex(math.cos(sign * 2 * math.pi * j * k / n), math.sin(sign * 2 * math.pi * j * k / n))
                for j in range(n)) for k in range(n)]


def interleave(zs):
    out = []
    for z in zs:
        out += [z.real, z.imag]
    return out


def case_fft(a, obj, tmp, n, batch, stride, dist, inverse, radix2, rnd, t="f"):
    sfx, tol = ("_f64", 1e-12) if t == "d" else ("", 1e-4)
    total = max((b * dist + (n - 1) * stride for b in range(batch)), default=0) + 1
    data = [complex(rnd.uniform(-1, 1), rnd.uniform(-1, 1)) for _ in range(total)]
    tw = pack(twiddles(n), t)
    buf = pack(interleave(data), t)
    if radix2:
        # zluda_fft_agx's passes: radix 2 when log2(n) is odd, then radix 4 (radix2 == "r2": radix 2 only)
        stages = n.bit_length() - 1
        radices = [2] * stages if radix2 == "r2" else [2] * (stages % 2) + [4] * (stages // 2)
        ns = 1
        for r in radices:
            (buf,) = run(a, obj, "agx_fft_radix%d" % r + sfx,
                         [("buf", buf), ("buf", pack([0.0] * (2 * total), t), True), ("buf", tw), ("i32", n), ("i32", ns),
                          ("i32", batch), ("i32", stride), ("i32", dist), ("i32", inverse)],
                         (n // r, batch), (min(n // r, 64), 1), tmp)
            ns *= r
    else:
        (buf,) = run(a, obj, "agx_dft" + sfx,
                     [("buf", buf), ("buf", pack([0.0] * (2 * total), t), True), ("buf", tw), ("i32", n), ("i32", batch),
                      ("i32", stride), ("i32", dist), ("i32", inverse)],
                     (-(-n // 16) * 16, batch), (16, 1), tmp)
    got = unpack(buf, t)
    bad = []
    for b in range(batch):
        idx = [b * dist + i * stride for i in range(n)]
        want = dft([data[i] for i in idx], n, inverse)
        for i, w in zip(idx, want):
            g = complex(got[2 * i], got[2 * i + 1])
            if abs(g - w) > tol * (1 + abs(w)) * max(1, math.log2(n)):
                bad.append(i)
    return bad


def case_fft_packing(a, obj, tmp, n, batch, rnd, t="f"):
    """The real / complex packing kernels around the complex transforms."""
    sfx = "_f64" if t == "d" else ""
    bad = []
    idist = n + 3
    real = rounded([rnd.uniform(-1, 1) for _ in range(batch * idist)], t)
    (c,) = run(a, obj, "agx_fft_real_to_complex" + sfx,
               [("buf", pack(real, t)), ("buf", pack([math.nan] * (2 * n * batch), t), True), ("i32", n), ("i32", batch),
                ("i32", idist)], (-(-n // 16) * 16, batch), (16, 1), tmp)
    c = unpack(c, t)
    bad += [i for i in range(n * batch) if c[2 * i] != real[(i // n) * idist + i % n] or c[2 * i + 1] != 0.0]
    m, odist = n // 2 + 1, n // 2 + 3
    src = rounded([rnd.uniform(-1, 1) for _ in range(2 * n * batch)], t)
    ostride, codist = 2, 2 * m + 1
    size = codist * batch
    (cp,) = run(a, obj, "agx_fft_copy" + sfx,
                [("buf", pack(src, t)), ("buf", pack([0.0] * (2 * size), t), True), ("i32", m), ("i32", batch),
                 ("i32", 1), ("i32", n), ("i32", ostride), ("i32", codist)], (-(-m // 16) * 16, batch), (16, 1), tmp)
    cp = unpack(cp, t)
    want = [0.0] * (2 * size)
    for b in range(batch):
        for i in range(m):
            d, s_ = b * codist + i * ostride, b * n + i
            want[2 * d], want[2 * d + 1] = src[2 * s_], src[2 * s_ + 1]
    bad += ["copy%d" % k for k in range(2 * size) if cp[k] != want[k]]
    half = rounded([rnd.uniform(-1, 1) for _ in range(2 * odist * batch)], t)
    (full,) = run(a, obj, "agx_fft_hermitian" + sfx,
                  [("buf", pack(half, t)), ("buf", pack([0.0] * (2 * n * batch), t), True), ("i32", n), ("i32", batch),
                   ("i32", odist)], (-(-n // 16) * 16, batch), (16, 1), tmp)
    full = unpack(full, t)
    for b in range(batch):
        for k in range(n):
            j = k if k <= n // 2 else n - k
            sign = 1 if k <= n // 2 else -1
            if full[2 * (b * n + k)] != half[2 * (b * odist + j)] or full[2 * (b * n + k) + 1] != sign * half[2 * (b * odist + j) + 1]:
                bad.append("herm%d" % k)
    (re_,) = run(a, obj, "agx_fft_real_part" + sfx,
                 [("buf", pack(src, t)), ("buf", pack([0.0] * (idist * batch), t), True), ("i32", n), ("i32", batch),
                  ("i32", idist)], (-(-n // 16) * 16, batch), (16, 1), tmp)
    re_ = unpack(re_, t)
    bad += ["re%d" % i for b in range(batch) for i in range(n) if re_[b * idist + i] != src[2 * (b * n + i)]]
    return bad


def random_csr(rows, cols, rnd, density=0.3):
    rowptr, col, val = [0], [], []
    for _ in range(rows):
        for c in range(cols):
            if rnd.random() < density:
                col.append(c)
                val.append(rnd.uniform(-1, 1))
        rowptr.append(len(col))
    return rowptr, col, rounded(val, BT)


def case_sparse(a, obj, tmp, what, I, base, rnd):
    it = "i" if I == "i32" else "q"
    rows, cols = 13, 9
    rowptr, col, val = random_csr(rows, cols, rnd)
    nnz = len(col)
    rp = pack([r + base for r in rowptr], it)
    ci = pack([c + base for c in col], it)
    dense = {(i, col[k]): val[k] for i in range(rows) for k in range(rowptr[i], rowptr[i + 1])}
    alpha, beta = 1.5, 0.5
    if what in ("csr", "csr_t", "coo", "coo_t"):
        trans = what.endswith("_t")
        xn, yn = (rows, cols) if trans else (cols, rows)
        x = rounded([rnd.uniform(-1, 1) for _ in range(xn)], BT)
        y = rounded([rnd.uniform(-1, 1) for _ in range(yn)], BT)
        want = [alpha * sum(dense.get((j, i) if trans else (i, j), 0.0) * x[j] for j in range(xn)) + beta * y[i]
                for i in range(yn)]
        if what == "csr":
            (got,) = run(a, obj, K("agx_csr_spmv_" + I),
                         [("buf", rp), ("buf", ci), ("buf", pack(val, BT)), ("buf", pack(x, BT)), ("buf", pack(y, BT), True),
                          ("i64", rows), ("i64", base), (S(), alpha), (S(), beta)], (-(-rows // 32) * 32, 1), (32, 1), tmp)
        else:
            (scaled,) = run(a, obj, K("agx_sparse_scale"), [("buf", pack(y, BT), True), ("i64", yn), ("i64", 1), (S(), beta)],
                            (-(-yn // 32) * 32, 1), (32, 1), tmp)
            if what == "csr_t":
                (got,) = run(a, obj, K("agx_csr_spmv_t_" + I),
                             [("buf", rp), ("buf", ci), ("buf", pack(val, BT)), ("buf", pack(x, BT)), ("buf", scaled, True),
                              ("i64", rows), ("i64", base), (S(), alpha)], (-(-rows // 32) * 32, 1), (32, 1), tmp)
            else:
                ri = pack([i + base for i in range(rows) for _ in range(rowptr[i], rowptr[i + 1])], it)
                (got,) = run(a, obj, K("agx_coo_spmv_" + I),
                             [("buf", ri), ("buf", ci), ("buf", pack(val, BT)), ("buf", pack(x, BT)), ("buf", scaled, True),
                              ("i64", nnz), ("i64", base), ("i64", int(trans)), (S(), alpha)],
                             (-(-nnz // 32) * 32, 1), (32, 1), tmp)
        return close(unpack(got, BT), want, range(yn), TOL(1e-5))
    # SpMM: C (rows x n) = alpha A B + beta C, B (cols x n); column-major or row-major dense matrices
    n = 5
    col_major = what == "spmm_col"
    ldb, ldc = (cols + 2, rows + 1) if col_major else (n + 2, n + 1)
    bs = (1, ldb) if col_major else (ldb, 1)
    cs = (1, ldc) if col_major else (ldc, 1)
    B = rounded([rnd.uniform(-1, 1) for _ in range(ldb * (n if col_major else cols))], BT)
    C = rounded([rnd.uniform(-1, 1) for _ in range(ldc * (n if col_major else rows))], BT)
    (got,) = run(a, obj, K("agx_csr_spmm_" + I),
                 [("buf", rp), ("buf", ci), ("buf", pack(val, BT)), ("buf", pack(B, BT)), ("buf", pack(C, BT), True),
                  ("i64", rows), ("i64", n), ("i64", base), ("i64", bs[0]), ("i64", bs[1]), ("i64", cs[0]), ("i64", cs[1]),
                  (S(), alpha), (S(), beta)], (-(-rows // 16) * 16, -(-n // 4) * 4), (16, 4), tmp)
    want = list(C)
    idx = []
    for i in range(rows):
        for j in range(n):
            o = i * cs[0] + j * cs[1]
            idx.append(o)
            want[o] = alpha * sum(dense.get((i, k), 0.0) * B[k * bs[0] + j * bs[1]] for k in range(cols)) + beta * C[o]
    return close(unpack(got, BT), want, idx, TOL(1e-5))


def grid1(n, w=32):
    return (max(1, -(-n // w)) * w, 1)


def case_sparse_ops(a, obj, tmp, what, I, base, rnd):
    """The kernels of sparse_ix.mlir and the index-free ones of sparse.mlir."""
    it = "i" if I == "i32" else "q"
    rows, cols = 13, 9
    if what in ("spvv", "axpby", "gather", "scatter", "rot"):
        size, nnz = 23, 9
        idx = sorted(rnd.sample(range(size), nnz))
        ip = pack([i + base for i in idx], it)
        xv = rounded([rnd.uniform(-1, 1) for _ in range(nnz)], BT)
        y = rounded([rnd.uniform(-1, 1) for _ in range(size)], BT)
        if what == "spvv":
            threads = 4
            (part,) = run(a, obj, K("agx_spvv_" + I), [("buf", ip), ("buf", pack(xv, BT)), ("buf", pack(y, BT)),
                                                    ("buf", pack([7.0] * threads, BT), True), ("i64", nnz), ("i64", base),
                                                    ("i64", threads)], grid1(threads), (32, 1), tmp)
            (out,) = run(a, obj, K("agx_sparse_sum"), [("buf", part), ("buf", pack([7.0], BT), True), ("i64", threads)],
                         (32, 1), (32, 1), tmp)
            return close(unpack(out, BT), [sum(xv[k] * y[idx[k]] for k in range(nnz))], [0], TOL(1e-5))
        op = ("axpby", "gather", "scatter", "rot").index(what)
        al, be = (1.5, 0.0) if op == 0 else (0.6, 0.8)
        gx, gy = run(a, obj, K("agx_spvec_" + I), [("buf", ip), ("buf", pack(xv, BT), True), ("buf", pack(y, BT), True),
                                                  ("i64", nnz), ("i64", base), ("i64", op), (S(), al), (S(), be)],
                     grid1(nnz), (32, 1), tmp)
        wx, wy = list(xv), list(y)
        for k, i in enumerate(idx):
            if op == 0:
                wy[i] = al * xv[k] + y[i]
            elif op == 1:
                wx[k] = y[i]
            elif op == 2:
                wy[i] = xv[k]
            else:
                wx[k], wy[i] = al * xv[k] + be * y[i], al * y[i] - be * xv[k]
        return close(unpack(gx, BT) + unpack(gy, BT), wx + wy, range(nnz + size), TOL(1e-5))
    rowptr, col, val = random_csr(rows, cols, rnd)
    nnz = len(col)
    rp = pack([r + base for r in rowptr], it)
    ci = pack([c + base for c in col], it)
    dense = {(i, col[k]): val[k] for i in range(rows) for k in range(rowptr[i], rowptr[i + 1])}
    if what in ("csr2dense", "coo2dense"):
        ld = rows + 3
        D = pack([5.0] * (ld * cols), BT)
        (z,) = run(a, obj, K("agx_dense_zero"), [("buf", D, True), ("i64", rows), ("i64", cols), ("i64", 1), ("i64", ld)],
                   (16, 16), (16, 4), tmp)
        coo = what == "coo2dense"
        ai = pack([i + base for i in range(rows) for _ in range(rowptr[i], rowptr[i + 1])], it) if coo else rp
        (got,) = run(a, obj, K("agx_sp_to_dense_" + I), [("buf", ai), ("buf", ci), ("buf", pack(val, BT)), ("buf", z, True),
                                                        ("i64", nnz if coo else rows), ("i64", base), ("i64", int(coo)),
                                                        ("i64", 1), ("i64", ld)], grid1(nnz if coo else rows), (32, 1), tmp)
        want = [5.0] * (ld * cols)
        idxs = []
        for i in range(rows):
            for j in range(cols):
                want[i + j * ld] = dense.get((i, j), 0.0)
                idxs.append(i + j * ld)
        return close(unpack(got, BT), want, range(ld * cols), 0)
    if what == "dense2csr":
        ld = cols + 2
        D = [0.0] * (ld * rows)
        for (i, j), v in dense.items():
            D[i * ld + j] = v
        D = pack(D, BT)
        common = [("i64", rows), ("i64", cols), ("i64", ld), ("i64", 1), ("i64", base)]
        (cnt,) = run(a, obj, K("agx_dense_to_csr_" + I), [("buf", D), ("buf", pack([99] * (rows + 1), it), True),
                                                         ("buf", pack([0], it)), ("buf", pack([0.0], BT))] + common + [("i64", 0)],
                     grid1(rows), (32, 1), tmp)
        (ptr,) = run(a, obj, K("agx_sparse_scan_" + I), [("buf", cnt, True), ("i64", rows), ("i64", base)], (32, 1), (32, 1), tmp)
        got_ptr = list(unpack(ptr, it))
        if got_ptr != [r + base for r in rowptr]:
            return ["rowptr %s, want %s" % (got_ptr, [r + base for r in rowptr])]
        gc, gv = run(a, obj, K("agx_dense_to_csr_" + I), [("buf", D), ("buf", ptr), ("buf", pack([-1] * nnz, it), True),
                                                         ("buf", pack([9.0] * nnz, BT), True)] + common + [("i64", 1)],
                     grid1(rows), (32, 1), tmp)
        if list(unpack(gc, it)) != [c + base for c in col]:
            return ["columns %s, want %s" % (list(unpack(gc, it)), col)]
        return close(unpack(gv, BT), val, range(nnz), 0)
    if what == "sddmm":
        K_, alpha, beta = 4, 1.5, 0.5
        lda, ldb = K_ + 1, cols + 2  # A row-major rows x K, B row-major K x cols
        A = rounded([rnd.uniform(-1, 1) for _ in range(rows * lda)], BT)
        B = rounded([rnd.uniform(-1, 1) for _ in range(K_ * ldb)], BT)
        (got,) = run(a, obj, K("agx_sddmm_" + I), [("buf", rp), ("buf", ci), ("buf", pack(val, BT), True), ("buf", pack(A, BT)),
                                                  ("buf", pack(B, BT)), ("i64", rows), ("i64", K_), ("i64", base),
                                                  ("i64", lda), ("i64", 1), ("i64", ldb), ("i64", 1), (S(), alpha), (S(), beta)],
                     grid1(rows), (32, 1), tmp)
        want = [alpha * sum(A[i * lda + t] * B[t * ldb + col[k]] for t in range(K_)) + beta * val[k]
                for i in range(rows) for k in range(rowptr[i], rowptr[i + 1])]
        return close(unpack(got, BT), want, range(nnz), TOL(1e-5))
    if what.startswith("spsm"):
        # what = spsm-<upper><unit><trans>
        upper, unit, trans = (int(c) for c in what[5:])
        n = 3
        m = 11
        # a well-conditioned square matrix with entries on both sides of the diagonal
        rowptr, col, val = [0], [], []
        for i in range(m):
            for j in range(m):
                if i == j or rnd.random() < 0.3:
                    col.append(j)
                    val.append(rnd.uniform(2, 3) * rnd.choice((-1, 1)) if i == j else rnd.uniform(-0.5, 0.5))
            rowptr.append(len(col))
        val = rounded(val, BT)
        nnz = len(col)
        dense = {(i, col[k]): val[k] for i in range(m) for k in range(rowptr[i], rowptr[i + 1])}
        ldb, ldc = m + 1, m + 2
        Bm = rounded([rnd.uniform(-1, 1) for _ in range(ldb * n)], BT)
        alpha = 1.5
        (got,) = run(a, obj, K("agx_spsm_" + I), [("buf", pack([r + base for r in rowptr], it)), ("buf", pack([c + base for c in col], it)),
                                                 ("buf", pack(val, BT)), ("buf", pack(Bm, BT)), ("buf", pack([0.0] * (ldc * n), BT), True),
                                                 ("i64", m), ("i64", n), ("i64", base), ("i64", 1), ("i64", ldb),
                                                 ("i64", 1), ("i64", ldc), ("i64", upper), ("i64", unit), ("i64", trans),
                                                 (S(), alpha)], (32, 1), (32, 1), tmp)

        def tri(i, j):
            if i == j:
                return 1.0 if unit else dense.get((i, i), 0.0)
            if (j > i) if upper else (j < i):
                return dense.get((i, j), 0.0)
            return 0.0
        # the level-scheduled solve (zluda_sparse_agx's spsv_schedule): op(A)'s rows with the triangle's and the
        # diagonal's entries, the rows by level; agx_spsv_level once per level, in place in C
        op_rows = [[] for _ in range(m)]
        for r in range(m):
            for k in range(rowptr[r], rowptr[r + 1]):
                c = col[k]
                if (c >= r) if upper else (c <= r):
                    (op_rows[c] if trans else op_rows[r]).append((r if trans else c, k))
        level = [0] * m
        for i in (range(m - 1, -1, -1) if upper ^ trans else range(m)):
            level[i] = max([level[j] + 1 for j, _ in op_rows[i] if j != i], default=0)
        order = sorted(range(m), key=lambda i: (level[i], i))
        tptr, tcol, tmap = [0], [], []
        for row in op_rows:
            tcol += [j for j, _ in row]
            tmap += [k for _, k in row]
            tptr.append(len(tcol))
        cl = pack([0.0] * (ldc * n), BT)
        first = 0
        for lvl in range(max(level) + 1):
            count = level.count(lvl)
            (cl,) = run(a, obj, K("agx_spsv_level"), [("buf", pack(tptr, "q")), ("buf", pack(tcol or [0], "q")), ("buf", pack(tmap or [0], "q")),
                                                     ("buf", pack(order, "q")), ("buf", pack(val, BT)), ("buf", pack(Bm, BT)), ("buf", cl, True),
                                                     ("i64", first), ("i64", count), ("i64", n), ("i64", 1), ("i64", ldb), ("i64", 1), ("i64", ldc),
                                                     ("i64", unit), (S(), alpha)], (32, 4), (32, 1), tmp)
            first += count
        # check op(T) X = alpha B
        errs = []
        for name, X in (("spsm", unpack(got, BT)), ("levels", unpack(cl, BT))):
            for j in range(n):
                for i in range(m):
                    lhs = sum((tri(k, i) if trans else tri(i, k)) * X[k + j * ldc] for k in range(m))
                    if abs(lhs - alpha * Bm[i + j * ldb]) > TOL(1e-4) * 10:
                        errs.append("%s residual at %d,%d: %g vs %g" % (name, i, j, lhs, alpha * Bm[i + j * ldb]))
        return errs[:5]
    raise ValueError(what)


def case_csr2csc(a, obj, tmp, base, words, rnd):
    """cusparseCsr2cscEx2 in its four steps and the value permutation (words of the value type per value)."""
    rows, cols = 13, 9
    rowptr, col, val = random_csr(rows, cols, rnd, 0.35)
    nnz = len(col)
    rp, ci = pack([r + base for r in rowptr], "i"), pack([c + base for c in col], "i")
    (cnt,) = run(a, obj, "agx_csr2csc_count", [("buf", rp), ("buf", ci), ("buf", pack([0] * cols, "i"), True),
                                               ("i64", rows), ("i64", base)], grid1(rows), (32, 1), tmp)
    nxt, ptr = run(a, obj, "agx_csr2csc_scan", [("buf", cnt, True), ("buf", pack([0] * (cols + 1), "i"), True),
                                                ("i64", cols), ("i64", base)], (32, 1), (32, 1), tmp)
    ri, perm = run(a, obj, "agx_csr2csc_fill", [("buf", rp), ("buf", ci), ("buf", nxt), ("buf", pack([0] * nnz, "i"), True),
                                                ("buf", pack([0] * nnz, "i"), True), ("i64", rows), ("i64", base)],
                   grid1(rows), (32, 1), tmp)
    ri, perm = run(a, obj, "agx_csr2csc_sort", [("buf", ptr), ("buf", ri, True), ("buf", perm, True), ("i64", cols), ("i64", base)],
                   grid1(cols), (32, 1), tmp)
    want_ptr, want_ri, want_perm = [base], [], []
    for j in range(cols):
        for i in range(rows):
            for k in range(rowptr[i], rowptr[i + 1]):
                if col[k] == j:
                    want_ri.append(i + base)
                    want_perm.append(k)
        want_ptr.append(len(want_ri) + base)
    errs = []
    for label, got, want in (("ptr", unpack(ptr, "i"), want_ptr), ("rowind", unpack(ri, "i"), want_ri), ("perm", unpack(perm, "i"), want_perm)):
        if list(got) != want:
            errs.append("%s %s, want %s" % (label, list(got), want))
    if errs:
        return errs
    src = [rnd.uniform(-1, 1) for _ in range(nnz * words)]
    src = rounded(src, BT)
    (dst,) = run(a, obj, K("agx_sparse_permute"), [("buf", pack(src, BT)), ("buf", pack([0.0] * (nnz * words), BT), True),
                                                 ("buf", perm), ("i64", nnz), ("i64", words)], grid1(nnz), (32, 1), tmp)
    want = [src[want_perm[p] * words + w] for p in range(nnz) for w in range(words)]
    return close(unpack(dst, BT), want, range(nnz * words), 0)


def case_spgemm(a, obj, tmp, I, base, rnd):
    """C = alpha A B in CSR: product bounds, scan, sorted row products, scan, copy."""
    it = "i" if I == "i32" else "q"
    m, k, n = 9, 7, 8
    arp, acol, aval = random_csr(m, k, rnd, 0.35)
    brp, bcol, bval = random_csr(k, n, rnd, 0.35)
    alpha = 1.5
    sh = lambda xs: pack([x + base for x in xs], it)
    (ub,) = run(a, obj, "agx_spgemm_ub_" + I, [("buf", sh(arp)), ("buf", sh(acol)), ("buf", sh(brp)),
                                               ("buf", pack([0] * (m + 1), "q"), True), ("i64", m), ("i64", base)],
                grid1(m), (32, 1), tmp)
    (ub,) = run(a, obj, "agx_sparse_scan_i64", [("buf", ub, True), ("i64", m), ("i64", 0)], (32, 1), (32, 1), tmp)
    ubl = unpack(ub, "q")
    want_ub = [0]
    for i in range(m):
        want_ub.append(want_ub[-1] + sum(brp[acol[t] + 1] - brp[acol[t]] for t in range(arp[i], arp[i + 1])))
    if list(ubl) != want_ub:
        return ["ub %s, want %s" % (list(ubl), want_ub)]
    total = max(1, ubl[-1])
    tcol, tval, cnt = run(a, obj, K("agx_spgemm_row_" + I),
                          [("buf", sh(arp)), ("buf", sh(acol)), ("buf", pack(aval, BT)), ("buf", sh(brp)), ("buf", sh(bcol)),
                           ("buf", pack(bval, BT)), ("buf", ub), ("buf", pack([0] * total, "q"), True),
                           ("buf", pack([0.0] * total, BT), True), ("buf", pack([0] * (m + 1), "q"), True),
                           ("i64", m), ("i64", base), (S(), alpha)], grid1(m), (32, 1), tmp)
    (cnt,) = run(a, obj, "agx_sparse_scan_i64", [("buf", cnt, True), ("i64", m), ("i64", 0)], (32, 1), (32, 1), tmp)
    want = []
    for i in range(m):
        row = {}
        for t in range(arp[i], arp[i + 1]):
            r = acol[t]
            for u in range(brp[r], brp[r + 1]):
                row[bcol[u]] = row.get(bcol[u], 0.0) + alpha * aval[t] * bval[u]
        want.append(sorted(row.items()))
    nnz = sum(len(r) for r in want)
    crp, ccol, cval = run(a, obj, K("agx_spgemm_copy_" + I),
                          [("buf", ub), ("buf", tcol), ("buf", tval), ("buf", cnt), ("buf", pack([-1] * (m + 1), it), True),
                           ("buf", pack([-1] * max(1, nnz), it), True), ("buf", pack([0.0] * max(1, nnz), BT), True),
                           ("i64", m), ("i64", base)], grid1(m), (32, 1), tmp)
    want_rp = [base]
    for r in want:
        want_rp.append(want_rp[-1] + len(r))
    if list(unpack(crp, it)) != want_rp:
        return ["rowptr %s, want %s" % (list(unpack(crp, it)), want_rp)]
    flat = [e for r in want for e in r]
    if list(unpack(ccol, it))[:nnz] != [c + base for c, _ in flat]:
        return ["columns %s, want %s" % (list(unpack(ccol, it)), [c for c, _ in flat])]
    return close(unpack(cval, BT), [v for _, v in flat], range(nnz), TOL(1e-5))


def philox(c, k):
    """Philox4x32-10 of the counter words c with the key words k (Random123)."""
    M = 0xFFFFFFFF
    c, k = list(c), list(k)
    for _ in range(10):
        p0, p1 = 0xD2511F53 * c[0], 0xCD9E8D57 * c[2]
        c = [((p1 >> 32) ^ c[1] ^ k[0]) & M, p1 & M, ((p0 >> 32) ^ c[3] ^ k[1]) & M, p0 & M]
        k = [(k[0] + 0x9E3779B9) & M, (k[1] + 0xBB67AE85) & M]
    return c


def f32(x):
    return struct.unpack("<f", struct.pack("<f", x))[0]


def case_rand(a, obj, tmp, what, rnd):
    seed, start, n = rnd.getrandbits(64), rnd.getrandbits(40), 23
    blocks = lambda count: [philox([(b & 0xFFFFFFFF), b >> 32, 0, 0], [seed & 0xFFFFFFFF, seed >> 32]) for b in range(start, start + count)]
    if what == "poisson":
        bad = []
        for lam in (0.5, 7.25, 150.0):
            (got,) = run(a, obj, "agx_rand_poisson", [("buf", pack([0] * n, "I"), True), ("i64", n), ("i64", start), ("i64", seed),
                                                     ("f32", lam)], grid1(n), (32, 1), tmp)
            got = unpack(got, "I")
            want = []
            for j in range(n):
                g = start + j
                if lam < 64:
                    it, total, k = 0, 0.0, 0
                    while True:
                        w = philox([g & 0xFFFFFFFF, g >> 32, it, 1], [seed & 0xFFFFFFFF, seed >> 32])
                        it += 1
                        for x in w:
                            total -= math.log(f32(x * 2.0 ** -32 + 2.0 ** -33))
                            if total > lam:
                                break
                            k += 1
                        else:
                            continue
                        break
                    want.append(k)
                else:
                    w = philox([g & 0xFFFFFFFF, g >> 32, 0, 1], [seed & 0xFFFFFFFF, seed >> 32])
                    u, v = f32(w[0] * 2.0 ** -32 + 2.0 ** -33), w[1] * 2.0 ** -32 * 2 * math.pi
                    z = math.sqrt(-2 * math.log(u)) * math.sin(v)
                    want.append(max(0, math.floor(lam + math.sqrt(lam) * z + 0.5)))
            # f32 logs: allow an arrival on the boundary to differ by one
            bad += ["lambda %g at %d: %d, want %d" % (lam, j, g_, w_) for j, (g_, w_) in enumerate(zip(got, want)) if abs(g_ - w_) > 1]
        return bad
    if what.startswith("r64"):
        mode = int(what[3:])
        mean, sd = 0.25, 2.0
        (got,) = run(a, obj, "agx_rand64", [("buf", pack([0.0] * n, "d"), True), ("i64", n), ("i64", start), ("i64", seed),
                                            ("i64", mode), ("f64", mean), ("f64", sd)], grid1(-(-n // 2)), (32, 1), tmp)
        got = unpack(got, "d")
        want = []
        for w in blocks(-(-n // 2)):
            u0 = (w[0] ^ ((w[1] << 21) & 0xFFFFFFFFFFFFFFFF)) * 2.0 ** -53 + 2.0 ** -54
            u1 = (w[2] ^ ((w[3] << 21) & 0xFFFFFFFFFFFFFFFF)) * 2.0 ** -53 + 2.0 ** -54
            if mode == 1:
                want += [u0, u1]
            else:
                s_ = math.sqrt(-2 * math.log(u0))
                z = [mean + sd * s_ * math.sin(2 * math.pi * u1), mean + sd * s_ * math.cos(2 * math.pi * u1)]
                want += [math.exp(x) for x in z] if mode == 3 else z
        bad = close(got, want, range(n), 1e-14)
        if bad and os.environ.get("RAND_DEBUG"):
            print([(got[i], want[i]) for i in bad[:4]])
        return bad
    mode = int(what[3:])
    mean, sd = -1.0, 0.5
    (got,) = run(a, obj, "agx_rand32", [("buf", pack([0] * n, "I"), True), ("i64", n), ("i64", start), ("i64", seed),
                                        ("i64", mode), ("f32", mean), ("f32", sd)], grid1(-(-n // 4)), (32, 1), tmp)
    words = [x for w in blocks(-(-n // 4)) for x in w]
    if mode == 0:
        return [i for i in range(n) if unpack(got, "I")[i] != words[i]]
    got = unpack(got, "f")
    uni = [f32(f32(float(x)) * f32(2.0 ** -32) + f32(2.0 ** -33)) for x in words]
    if mode == 1:
        return close(got, uni, range(n), 1e-7)
    want = []
    for i in range(0, n, 2):
        u = uni[i]
        v = words[i + 1] * 2.0 ** -32 * 2 * math.pi + 2.0 ** -33 * 2 * math.pi
        s_ = math.sqrt(-2 * math.log(u))
        z = [mean + sd * s_ * math.sin(v), mean + sd * s_ * math.cos(v)]
        want += [math.exp(x) for x in z] if mode == 3 else z
    return close(got, want[:n], range(n), 2e-5)


def case_qr_wy(a, obj, tmp, rnd):
    """agx_qr_v and agx_qr_t of the blocked geqrf: V from a column panel, T from S = V^T V and tau."""
    t = BT
    m, lda, j0, jb = 11, 13, 2, 4
    A = rounded([rnd.uniform(-1, 1) for _ in range(lda * (j0 + jb))], t)
    mv = m - j0
    (v,) = run(a, obj, K("agx_qr_v"), [("buf", pack(A, t)), ("buf", pack([math.nan] * (mv * jb), t), True), ("i64", m), ("i64", j0),
                                      ("i64", jb), ("i64", lda)], (16, 8), (16, 4), tmp)
    want = [1.0 if i == c else (A[(j0 + i) + (j0 + c) * lda] if i > c else 0.0) for c in range(jb) for i in range(mv)]
    bad = ["V %d" % p for p, (g, w) in enumerate(zip(unpack(v, t), want)) if g != w]
    S = rounded([rnd.uniform(-1, 1) for _ in range(jb * jb)], t)
    tau = rounded([rnd.uniform(0.5, 1.5) for _ in range(j0 + jb)], t)
    (tb,) = run(a, obj, K("agx_qr_t"), [("buf", pack(S, t)), ("buf", pack(tau, t)), ("buf", pack([math.nan] * (jb * jb), t), True),
                                       ("i64", j0), ("i64", jb)], (1, 1), (1, 1), tmp)
    T = [0.0] * (jb * jb)
    for i in range(jb):
        T[i + i * jb] = tau[j0 + i]
        for r in range(i):
            T[r + i * jb] = -tau[j0 + i] * sum(T[r + q * jb] * S[q + i * jb] for q in range(r, i))
    got = unpack(tb, t)
    bad += ["T %d" % p for p in range(jb * jb) if abs(got[p] - T[p]) > TOL(1e-5) * (1 + abs(T[p]))]
    return bad


def case_solver(a, obj, tmp, what, rnd):
    """The column steps of getrf / potrf / geqrf, then getrs / potrs / ormqr / orgqr, against Python references."""
    t = BT
    n, lda, nrhs, ldb = 6, 8, 3, 7
    tol = TOL(1e-4)

    def matmul(A, B):
        return [[sum(A[i][k] * B[k][j] for k in range(len(B))) for j in range(len(B[0]))] for i in range(len(A))]

    def col_major(M, ld):
        flat = [0.0] * (ld * len(M[0]))
        for i in range(len(M)):
            for j in range(len(M[0])):
                flat[i + j * ld] = M[i][j]
        return flat

    def from_col(flat, rows, cols, ld):
        return [[flat[i + j * ld] for j in range(cols)] for i in range(rows)]

    B = [[rnd.uniform(-1, 1) for _ in range(nrhs)] for _ in range(n)]
    bbuf = pack(rounded(col_major(B, ldb), t), t)
    if what.startswith("lu"):
        m = n
        A = [[rnd.uniform(-1, 1) for _ in range(n)] for _ in range(m)]
        A = [[rounded([x], t)[0] for x in r] for r in A]
        buf, ipiv, info = pack(col_major(A, lda), t), pack([0] * n, "i"), pack([77], "i")
        for k in range(n):
            ipiv, info = run(a, obj, K("agx_lu_pivot"), [("buf", buf), ("i64", m), ("i64", k), ("i64", lda), ("buf", ipiv, True),
                                                       ("buf", info, True)], (1, 1), (1, 1), tmp)
            (buf,) = run(a, obj, K("agx_lu_swap"), [("buf", buf, True), ("i64", n), ("i64", lda), ("i64", k), ("buf", ipiv)],
                         grid1(n), (32, 1), tmp)
            (buf,) = run(a, obj, K("agx_lu_scale"), [("buf", buf, True), ("i64", m), ("i64", lda), ("i64", k)], grid1(m), (32, 1), tmp)
            (buf,) = run(a, obj, K("agx_lu_update"), [("buf", buf, True), ("i64", m), ("i64", n), ("i64", lda), ("i64", k)],
                         (16, 16), (16, 4), tmp)
        if unpack(info, "i")[0] != 0:
            return ["info %d" % unpack(info, "i")[0]]
        # P A = L U
        F = from_col(unpack(buf, t), m, n, lda)
        L = [[1.0 if i == j else (F[i][j] if i > j else 0.0) for j in range(n)] for i in range(m)]
        U = [[F[i][j] if i <= j else 0.0 for j in range(n)] for i in range(n)]
        PA = [r[:] for r in A]
        for k, p in enumerate(unpack(ipiv, "i")):
            PA[k], PA[p - 1] = PA[p - 1], PA[k]
        LU = matmul(L, U)
        bad = ["PA != LU at %d,%d" % (i, j) for i in range(n) for j in range(n) if abs(LU[i][j] - PA[i][j]) > tol * 10]
        if bad:
            return bad[:4]
        trans = what == "lu_t"
        # the interchanges alone (the blocked getrs): in order, then in reverse back to B
        (pb,) = run(a, obj, K("agx_getrs_pivots"), [("buf", ipiv), ("buf", bbuf, True), ("i64", n), ("i64", nrhs), ("i64", ldb),
                                                  ("i64", 0)], (32, 1), (32, 1), tmp)
        PB = from_col(unpack(bbuf, t), n, nrhs, ldb)
        for k, p in enumerate(unpack(ipiv, "i")):
            PB[k], PB[p - 1] = PB[p - 1], PB[k]
        if from_col(unpack(pb, t), n, nrhs, ldb) != PB:
            return ["agx_getrs_pivots"]
        (back,) = run(a, obj, K("agx_getrs_pivots"), [("buf", ipiv), ("buf", pb, True), ("i64", n), ("i64", nrhs), ("i64", ldb),
                                                    ("i64", 1)], (32, 1), (32, 1), tmp)
        if back != bbuf:
            return ["agx_getrs_pivots reverse"]
        (x,) = run(a, obj, K("agx_getrs"), [("buf", buf), ("i64", n), ("i64", lda), ("buf", ipiv), ("buf", bbuf, True),
                                           ("i64", nrhs), ("i64", ldb), ("i64", int(trans))], (32, 1), (32, 1), tmp)
        X = from_col(unpack(x, t), n, nrhs, ldb)
        opA = [list(r) for r in zip(*A)] if trans else A
        AX = matmul(opA, X)
        return ["residual at %d,%d" % (i, j) for i in range(n) for j in range(nrhs) if abs(AX[i][j] - B[i][j]) > tol * 100][:4]
    if what.startswith("chol"):
        upper = what == "chol_u"
        M = [[rnd.uniform(-1, 1) for _ in range(n)] for _ in range(n)]
        A = matmul(M, [list(r) for r in zip(*M)])
        for i in range(n):
            A[i][i] += n
        A = [[rounded([x], t)[0] for x in r] for r in A]
        # only the used triangle is meaningful: poison the other one
        stored = [[A[i][j] if ((j >= i) if upper else (j <= i)) else 1e3 for j in range(n)] for i in range(n)]
        rs, cs = (lda, 1) if upper else (1, lda)
        buf, info = pack(col_major(stored, lda), t), pack([77], "i")
        for k in range(n):
            (buf, info) = run(a, obj, K("agx_chol_diag"), [("buf", buf, True), ("i64", k), ("i64", rs), ("i64", cs), ("buf", info, True)],
                              (1, 1), (1, 1), tmp)
            (buf,) = run(a, obj, K("agx_chol_scale"), [("buf", buf, True), ("i64", n), ("i64", k), ("i64", rs), ("i64", cs), ("buf", info)],
                         grid1(n), (32, 1), tmp)
            (buf,) = run(a, obj, K("agx_chol_update"), [("buf", buf, True), ("i64", n), ("i64", k), ("i64", rs), ("i64", cs), ("buf", info),
                                                        ("i64", n)],
                         (16, 16), (16, 4), tmp)
        if unpack(info, "i")[0] != 0:
            return ["info %d" % unpack(info, "i")[0]]
        (x,) = run(a, obj, K("agx_potrs"), [("buf", buf), ("i64", n), ("i64", lda), ("buf", bbuf, True), ("i64", nrhs), ("i64", ldb),
                                           ("i64", int(upper))], (32, 1), (32, 1), tmp)
        F = from_col(unpack(buf, t), n, n, lda)
        bad = ["other triangle written at %d,%d" % (i, j) for i in range(n) for j in range(n)
               if ((j < i) if upper else (j > i)) and F[i][j] != 1e3]
        X = from_col(unpack(x, t), n, nrhs, ldb)
        AX = matmul(A, X)
        return (bad + ["residual at %d,%d" % (i, j) for i in range(n) for j in range(nrhs) if abs(AX[i][j] - B[i][j]) > tol * 100])[:4]
    if what == "chol_fail":
        A = [[1.0 if i == j else 0.0 for j in range(n)] for i in range(n)]
        A[3][3] = -1.0
        buf, info = pack(col_major(A, lda), t), pack([77], "i")
        for k in range(n):
            (buf, info) = run(a, obj, K("agx_chol_diag"), [("buf", buf, True), ("i64", k), ("i64", 1), ("i64", lda), ("buf", info, True)],
                              (1, 1), (1, 1), tmp)
        return [] if unpack(info, "i")[0] == 4 else ["info %d, want 4" % unpack(info, "i")[0]]
    # QR of an m x n matrix (m >= n), then Q and op(Q) C
    m = 8
    A = [[rounded([rnd.uniform(-1, 1)], t)[0] for _ in range(n)] for _ in range(m)]
    buf, tau = pack(col_major(A, m + 1), t), pack([0.0] * n, t)
    for k in range(n):
        (buf, tau) = run(a, obj, K("agx_qr_house"), [("buf", buf, True), ("i64", m), ("i64", k), ("i64", m + 1), ("buf", tau, True)],
                         (1, 1), (1, 1), tmp)
        (buf,) = run(a, obj, K("agx_qr_apply"), [("buf", buf, True), ("i64", m), ("i64", n), ("i64", k), ("i64", m + 1), ("buf", tau)],
                     grid1(n), (32, 1), tmp)
    F = from_col(unpack(buf, t), m, n, m + 1)
    R = [[F[i][j] if i <= j else 0.0 for j in range(n)] for i in range(m)]
    # orgqr: Q's first n columns from the identity
    (eye,) = run(a, obj, K("agx_sv_copy"), [("buf", pack([0.0], t)), ("i64", 1), ("buf", pack([9.0] * (m * n), t), True), ("i64", m),
                                          ("i64", m), ("i64", n), ("i64", 1)], (16, 16), (16, 4), tmp)
    (q,) = run(a, obj, K("agx_ormqr"), [("buf", buf), ("i64", m + 1), ("buf", tau), ("buf", eye, True), ("i64", m), ("i64", n),
                                       ("i64", n), ("i64", m), ("i64", 1), ("i64", 0)], (32, 1), (32, 1), tmp)
    Q = from_col(unpack(q, t), m, n, m)
    QR = matmul(Q, R[:n])
    bad = ["QR != A at %d,%d" % (i, j) for i in range(m) for j in range(n) if abs(QR[i][j] - A[i][j]) > tol * 10]
    QtQ = matmul([list(r) for r in zip(*Q)], Q)
    bad += ["Q^T Q at %d,%d" % (i, j) for i in range(n) for j in range(n) if abs(QtQ[i][j] - (i == j)) > tol * 10]
    if bad:
        return bad[:4]
    # the full Q (m x m) for the ormqr checks
    (eye,) = run(a, obj, K("agx_sv_copy"), [("buf", pack([0.0], t)), ("i64", 1), ("buf", pack([9.0] * (m * m), t), True), ("i64", m),
                                          ("i64", m), ("i64", m), ("i64", 1)], (16, 16), (16, 4), tmp)
    (qf,) = run(a, obj, K("agx_ormqr"), [("buf", buf), ("i64", m + 1), ("buf", tau), ("buf", eye, True), ("i64", m), ("i64", m),
                                        ("i64", n), ("i64", m), ("i64", 1), ("i64", 0)], (32, 1), (32, 1), tmp)
    Qf = from_col(unpack(qf, t), m, m, m)
    Qt = [list(r) for r in zip(*Qf)]
    for left in (1, 0):
        for trans in (0, 1):
            rows, cols = (m, 5) if left else (5, m)
            ldc = rows + 2
            C = [[rounded([rnd.uniform(-1, 1)], t)[0] for _ in range(cols)] for _ in range(rows)]
            (got,) = run(a, obj, K("agx_ormqr"), [("buf", buf), ("i64", m + 1), ("buf", tau), ("buf", pack(col_major(C, ldc), t), True),
                                                 ("i64", rows), ("i64", cols), ("i64", n), ("i64", ldc), ("i64", left), ("i64", trans)],
                         (32, 1), (32, 1), tmp)
            G = from_col(unpack(got, t), rows, cols, ldc)
            op = Qt if trans else Qf
            W = matmul(op, C) if left else matmul(C, op)
            bad += ["ormqr left %d trans %d at %d,%d" % (left, trans, i, j) for i in range(rows) for j in range(cols)
                    if abs(G[i][j] - W[i][j]) > tol * 10]
    return bad[:4]


def jacobi(a, obj, tmp, abuf, m, n, lda, vbuf, ldv, tol):
    """Sweeps of agx_jacobi_round until a sweep rotates nothing; returns A, V and the sweep count."""
    t = BT
    for sweep in range(40):
        count = pack([0], "i")
        nn = n + n % 2
        for r in range(nn - 1):
            abuf, vbuf, count = run(a, obj, K("agx_jacobi_round"), [("buf", abuf, True), ("i64", m), ("i64", n), ("i64", lda),
                                                                    ("buf", vbuf, True), ("i64", n), ("i64", ldv), ("i64", r),
                                                                    ("buf", count, True), (S(), tol)], (32, 1), (32, 1), tmp)
        if unpack(count, "i")[0] == 0:
            return abuf, vbuf, sweep
    return abuf, vbuf, None


def case_eigsvd(a, obj, tmp, what, rnd):
    t = BT
    tol = 1e-6 if t == "f" else 1e-14
    check = TOL(1e-4) * 10

    def matmul(A, B):
        return [[sum(A[i][k] * B[k][j] for k in range(len(B))) for j in range(len(B[0]))] for i in range(len(A))]

    def col(flat, rows, cols, ld):
        return [[flat[i + j * ld] for j in range(cols)] for i in range(rows)]

    if what == "svd":
        m, n, lda = 8, 5, 9
        A = [[rounded([rnd.uniform(-1, 1)], t)[0] for _ in range(n)] for _ in range(m)]
        flat = [0.0] * (lda * n)
        for i in range(m):
            for j in range(n):
                flat[i + j * lda] = A[i][j]
        eye = [1.0 if i == j else 0.0 for j in range(n) for i in range(n)]
        abuf, vbuf, sweeps = jacobi(a, obj, tmp, pack(flat, t), m, n, lda, pack(eye, t), n, tol)
        if sweeps is None:
            return ["no convergence"]
        abuf, sbuf = run(a, obj, K("agx_jacobi_norms"), [("buf", abuf, True), ("i64", m), ("i64", n), ("i64", lda),
                                                        ("buf", pack([0.0] * n, t), True)], (32, 1), (32, 1), tmp)
        sbuf, abuf, vbuf = run(a, obj, K("agx_jacobi_sort"), [("buf", sbuf, True), ("i64", n), ("buf", abuf, True), ("i64", m),
                                                             ("i64", lda), ("buf", vbuf, True), ("i64", n), ("i64", n), ("i64", 0)],
                               (1, 1), (1, 1), tmp)
        S_, U, V = unpack(sbuf, t), col(unpack(abuf, t), m, n, lda), col(unpack(vbuf, t), n, n, n)
        if list(S_) != sorted(S_, reverse=True):
            return ["not sorted: %s" % (S_,)]
        USVt = matmul([[U[i][j] * S_[j] for j in range(n)] for i in range(m)], [list(r) for r in zip(*V)])
        bad = ["U S V^T != A at %d,%d" % (i, j) for i in range(m) for j in range(n) if abs(USVt[i][j] - A[i][j]) > check]
        UtU = matmul([list(r) for r in zip(*U)], U)
        bad += ["U^T U at %d,%d" % (i, j) for i in range(n) for j in range(n) if abs(UtU[i][j] - (i == j)) > check]
        return bad[:4]
    # symmetric eigenvalues of the triangle what[-1] ("l" or "u")
    upper = what.endswith("u")
    n, lda = 6, 7
    M = [[rnd.uniform(-1, 1) for _ in range(n)] for _ in range(n)]
    A = [[rounded([(M[i][j] + M[j][i]) / 2], t)[0] for j in range(n)] for i in range(n)]
    flat = [5e3] * (lda * n)
    for i in range(n):
        for j in range(n):
            if (i <= j) if upper else (i >= j):
                flat[i + j * lda] = A[i][j]
    abuf = pack(flat, t)
    (slot,) = run(a, obj, K("agx_eig_shift"), [("buf", abuf), ("i64", lda), ("i64", n), ("i64", int(upper)), ("buf", pack([0.0], t), True)],
                  (1, 1), (1, 1), tmp)
    w, v = run(a, obj, K("agx_eig_prep"), [("buf", abuf), ("i64", lda), ("i64", n), ("i64", int(upper)), ("buf", pack([0.0] * n * n, t), True),
                                          ("buf", pack([0.0] * n * n, t), True), ("buf", slot)], (16, 16), (16, 4), tmp)
    w, v, sweeps = jacobi(a, obj, tmp, w, n, n, n, v, n, tol)
    if sweeps is None:
        return ["no convergence"]
    (lam,) = run(a, obj, K("agx_eig_values"), [("buf", abuf), ("i64", lda), ("i64", n), ("i64", int(upper)), ("buf", v),
                                              ("buf", pack([0.0] * n, t), True)], (32, 1), (32, 1), tmp)
    lam, v = run(a, obj, K("agx_jacobi_sort"), [("buf", lam, True), ("i64", n), ("buf", pack([0.0], t)), ("i64", 0), ("i64", 1),
                                               ("buf", v, True), ("i64", n), ("i64", n), ("i64", 1)], (1, 1), (1, 1), tmp)
    L, V = unpack(lam, t), col(unpack(v, t), n, n, n)
    if list(L) != sorted(L):
        return ["not ascending: %s" % (L,)]
    AV = matmul(A, V)
    bad = ["A v != lambda v at %d,%d" % (i, j) for i in range(n) for j in range(n) if abs(AV[i][j] - L[j] * V[i][j]) > check]
    VtV = matmul([list(r) for r in zip(*V)], V)
    bad += ["V^T V at %d,%d" % (i, j) for i in range(n) for j in range(n) if abs(VtV[i][j] - (i == j)) > check]
    return bad[:4]


def bf16_bits(v):
    b = struct.unpack("<I", struct.pack("<f", v))[0]
    if v != v:
        return 0x7FC0
    return ((b + 0x7FFF + ((b >> 16) & 1)) >> 16) & 0xFFFF


def case_dnn_convert(a, obj, tmp, kind, rnd):
    """agx_dnn_to_f32 / agx_dnn_from_f32 for f16 (kind 0) and bf16 (kind 1), strided both ways."""
    N, C, H, W = 2, 3, 2, 5
    total = N * C * H * W
    ss = (C * H * W * 2, H * W * 2, W * 2, 2)  # a padded source
    ds = (1, N, N * C, N * C * H)  # a transposed destination
    vals = [rnd.uniform(-70000, 70000) if i % 7 == 0 else rnd.uniform(-3, 3) for i in range(total)] + [float("inf"), -0.0]
    vals = vals[:total]
    to_bits = (lambda v: struct.unpack("<H", struct.pack("<e", v))[0] if abs(v) < 65504 or v != v else (0x7C00 | (0x8000 if v < 0 else 0))) if kind == 0 else bf16_bits
    def from_bits(b):
        return struct.unpack("<e", struct.pack("<H", b))[0] if kind == 0 else struct.unpack("<f", struct.pack("<I", b << 16))[0]
    src = [0] * (ss[0] * N)
    for i in range(total):
        n, c, h, w = i // (C * H * W), i // (H * W) % C, i // W % H, i % W
        src[n * ss[0] + c * ss[1] + h * ss[2] + w * ss[3]] = to_bits(vals[i])
    common = [("i64", total), ("i64", C), ("i64", H), ("i64", W)]
    (f,) = run(a, obj, "agx_dnn_to_f32", [("buf", pack(src, "H")), ("buf", pack([0.0] * total, "f"), True)] + common +
               [("i64", x) for x in ss] + [("i64", x) for x in ds] + [("i64", kind)], grid1(total), (32, 1), tmp)
    got = unpack(f, "f")
    bad = []
    for i in range(total):
        n, c, h, w = i // (C * H * W), i // (H * W) % C, i // W % H, i % W
        o = n * ds[0] + c * ds[1] + h * ds[2] + w * ds[3]
        want = from_bits(src[n * ss[0] + c * ss[1] + h * ss[2] + w * ss[3]])
        if got[o] != want and not (got[o] != got[o] and want != want):
            bad.append("to_f32 %d: %r want %r" % (i, got[o], want))
    # back from the original f32 values (rounding)
    f32s = rounded(vals, "f")
    fsrc = [0.0] * total
    for i in range(total):
        n, c, h, w = i // (C * H * W), i // (H * W) % C, i // W % H, i % W
        fsrc[n * ds[0] + c * ds[1] + h * ds[2] + w * ds[3]] = f32s[i]
    (hb,) = run(a, obj, "agx_dnn_from_f32", [("buf", pack(fsrc, "f")), ("buf", pack([0] * (ss[0] * N), "H"), True)] + common +
                [("i64", x) for x in ds] + [("i64", x) for x in ss] + [("i64", kind)], grid1(total), (32, 1), tmp)
    hb = unpack(hb, "H")
    for i in range(total):
        n, c, h, w = i // (C * H * W), i // (H * W) % C, i // W % H, i % W
        o = n * ss[0] + c * ss[1] + h * ss[2] + w * ss[3]
        if hb[o] != to_bits(f32s[i]):
            bad.append("from_f32 %d: %#x want %#x (%r)" % (i, hb[o], to_bits(f32s[i]), f32s[i]))
    return bad[:4]


def pw_ref(mode, x, b, t, p0, p1, p2):
    """cuDNN's pointwise modes (zluda_dnn_agx's graph API)."""
    sig = lambda v: 1 / (1 + math.exp(-v))
    erf = math.erf
    k = math.sqrt(2 / math.pi)
    def gelu_tanh_d(v):
        u = k * (v + 0.044715 * v ** 3)
        th = math.tanh(u)
        return 0.5 * (1 + th) + 0.5 * v * (1 - th * th) * k * (1 + 3 * 0.044715 * v * v)
    f = {
        0: lambda: x + b, 1: lambda: x * b, 2: lambda: min(x, b), 3: lambda: max(x, b), 4: lambda: math.sqrt(x),
        5: lambda: x + b * b, 6: lambda: x / b, 7: lambda: math.fmod(x, b), 8: lambda: x ** b, 9: lambda: x - b,
        10: lambda: abs(x), 11: lambda: math.ceil(x), 12: lambda: math.cos(x), 13: lambda: math.exp(x),
        14: lambda: math.floor(x), 15: lambda: math.log(x), 16: lambda: -x, 17: lambda: 1 / math.sqrt(x),
        18: lambda: math.sin(x), 19: lambda: math.tan(x), 20: lambda: erf(x), 21: lambda: x, 22: lambda: 1 / x,
        100: lambda: p0 + p2 * (x - p0) if x <= p0 else (p1 if x >= p1 else x), 101: lambda: math.tanh(x),
        102: lambda: sig(x), 103: lambda: x if x > 0 else p0 * math.expm1(x), 104: lambda: 0.5 * x * (1 + erf(x / math.sqrt(2))),
        105: lambda: math.log1p(math.exp(p0 * x)) / p0, 106: lambda: x * sig(p0 * x),
        107: lambda: 0.5 * x * (1 + math.tanh(k * (x + 0.044715 * x ** 3))),
        200: lambda: b * (p2 if x <= p0 else (0.0 if x >= p1 else 1.0)), 201: lambda: b * (1 - math.tanh(x) ** 2),
        202: lambda: b * sig(x) * (1 - sig(x)), 203: lambda: b * (1.0 if x > 0 else p0 * math.exp(x)),
        204: lambda: b * (0.5 * (1 + erf(x / math.sqrt(2))) + x * math.exp(-x * x / 2) / math.sqrt(2 * math.pi)),
        205: lambda: b * sig(p0 * x), 206: lambda: b * (sig(p0 * x) + p0 * x * sig(p0 * x) * (1 - sig(p0 * x))),
        207: lambda: b * gelu_tanh_d(x),
        300: lambda: float(x == b), 301: lambda: float(x != b), 302: lambda: float(x > b), 303: lambda: float(x >= b),
        304: lambda: float(x < b), 305: lambda: float(x <= b), 400: lambda: float(x != 0 and b != 0),
        401: lambda: float(x != 0 or b != 0), 402: lambda: float(x == 0), 601: lambda: x if t != 0 else b,
    }
    return f[mode]()


PW_MODES = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 100, 101, 102, 103, 104, 105, 106,
            107, 200, 201, 202, 203, 204, 205, 206, 207, 300, 301, 302, 303, 304, 305, 400, 401, 402, 601]


def case_pw_graph(a, obj, tmp, modes, rnd):
    N, C, H, W = 2, 3, 2, 3
    total = N * C * H * W
    positive = {4, 8, 15, 17}
    bad = []
    for mode in modes:
        lo = 0.1 if mode in positive else -2.5
        xs = rounded([rnd.uniform(lo, 2.5) for _ in range(total)], "f")
        if mode >= 100 and mode not in (105, 205):
            xs[3] = 0.0
        bc = rounded([rnd.uniform(0.5, 2.0) * rnd.choice((-1, 1)) for _ in range(C)], "f")  # b broadcast over channels
        if mode in (300, 301, 302, 303, 304, 305):
            bc[0] = xs[0]
        ts = [float(i % 2) for i in range(total)]
        p0, p1, p2 = (-0.5, 1.5, 0.1) if mode in (100, 200) else (1.3, 0.0, 0.0)
        alpha1, alpha2 = 1.0, 1.0
        ys = (C * H * W, H * W, W, 1)
        (got,) = run(a, obj, "agx_dnn_pw_graph", [("buf", pack(xs, "f")), ("buf", pack(bc, "f")), ("buf", pack(ts, "f")),
                                                  ("buf", pack([7.0] * total, "f"), True), ("i64", total), ("i64", C), ("i64", H), ("i64", W)] +
                     [("i64", v) for v in ys] + [("i64", 0), ("i64", 1), ("i64", 0), ("i64", 0)] + [("i64", v) for v in ys] +
                     [("i64", v) for v in ys] + [("i64", mode), ("f32", p0), ("f32", p1), ("f32", p2), ("f32", alpha1), ("f32", alpha2)],
                     grid1(total), (32, 1), tmp)
        got = unpack(got, "f")
        for i in range(total):
            c = i // (H * W) % C
            want = pw_ref(mode, xs[i], bc[c], ts[i], f32_(p0), f32_(p1), f32_(p2))
            if not (abs(got[i] - want) <= 2e-5 * (1 + abs(want))) and not (math.isinf(want) and got[i] == want):
                bad.append("mode %d at %d (x %r b %r): %r want %r" % (mode, i, xs[i], bc[c], got[i], want))
                break
    return bad[:6]


def f32_(v):
    return struct.unpack("<f", struct.pack("<f", v))[0]


def case_reduce_graph(a, obj, tmp, op, rnd):
    N, C, H, W = 2, 3, 4, 5
    xs = rounded([rnd.uniform(-2, 2) for _ in range(N * C * H * W)], "f")
    xs[5] = 0.0
    xst = (C * H * W, H * W, W, 1)
    bad = []
    # reduce over C and W, then over everything
    for yd in ((N, 1, H, 1), (1, 1, 1, 1)):
        e = [N if yd[0] == 1 else 1, C if yd[1] == 1 else 1, H if yd[2] == 1 else 1, W if yd[3] == 1 else 1]
        ytot = yd[0] * yd[1] * yd[2] * yd[3]
        yst = (yd[1] * yd[2] * yd[3], yd[2] * yd[3], yd[3], 1)
        (got,) = run(a, obj, "agx_dnn_reduce_graph", [("buf", pack(xs, "f")), ("buf", pack([9.0] * ytot, "f"), True), ("i64", ytot),
                                                      ("i64", yd[1]), ("i64", yd[2]), ("i64", yd[3])] + [("i64", v) for v in xst] +
                     [("i64", v) for v in yst] + [("i64", v) for v in e] + [("i64", op)], grid1(ytot), (32, 1), tmp)
        got = unpack(got, "f")
        for j in range(ytot):
            n, c, h, w = j // (yd[1] * yd[2] * yd[3]), j // (yd[2] * yd[3]) % yd[1], j // yd[3] % yd[2], j % yd[3]
            vals = [xs[(n + dn) * xst[0] + (c + dc) * xst[1] + (h + dh) * xst[2] + (w + dw) * xst[3]]
                    for dn in range(e[0]) for dc in range(e[1]) for dh in range(e[2]) for dw in range(e[3])]
            want = [sum(vals), math.prod(vals), min(vals), max(vals), max(abs(v) for v in vals), sum(vals) / len(vals),
                    sum(abs(v) for v in vals), math.sqrt(sum(v * v for v in vals)), math.prod(v for v in vals if v != 0)][op]
            if not abs(got[j] - want) <= 1e-4 * (1 + abs(want)):
                bad.append("op %d y %s at %d: %r want %r" % (op, yd, j, got[j], want))
    return bad[:4]


def blas_pos(i, n, inc):
    return i * inc if inc >= 0 else (n - 1 - i) * -inc


# The precision of the BLAS cases: "f" runs the f32 kernels, "d" their f64 copies (`<kernel>_f64`, generated by
# agx_libs/build.rs), with tighter tolerances.
BT = "f"


def K(name):
    return name + "_f64" if BT == "d" else name


def S():
    return "f64" if BT == "d" else "f32"


def TOL(t):
    return 1e-12 if BT == "d" else t


def with_t(t, f):
    global BT
    BT = t
    try:
        return f()
    finally:
        BT = "f"


def case_blas_vec(a, obj, tmp, op, n, incx, incy, rnd):
    alpha = 1.75
    lx, ly = 1 + (n - 1) * abs(incx), 1 + (n - 1) * abs(incy)
    x = rounded([rnd.uniform(-1, 1) for _ in range(lx)], BT)
    y = rounded([rnd.uniform(-1, 1) for _ in range(ly)], BT)
    gx, gy = run(a, obj, K("agx_blas_vec"), [("buf", pack(x, BT), True), ("buf", pack(y, BT), True), ("i64", n), ("i64", incx),
                                          ("i64", incy), ("i64", op), (S(), alpha)], (-(-n // 32) * 32, 1), (32, 1), tmp)
    wx, wy = list(x), list(y)
    for i in range(n):
        px, py = blas_pos(i, n, incx), blas_pos(i, n, incy)
        if op == 0:
            wy[py] = alpha * x[px] + y[py]
        elif op == 1:
            wx[px] = alpha * x[px]
        elif op == 2:
            wy[py] = x[px]
        else:
            wx[px], wy[py] = y[py], x[px]
    return close(unpack(gx, BT), wx, range(lx), 1e-6) + close(unpack(gy, BT), wy, range(ly), TOL(1e-6))


def case_blas_reduce(a, obj, tmp, op, n, incx, incy, threads, rnd):
    lx, ly = 1 + max(n - 1, 0) * abs(incx), 1 + max(n - 1, 0) * abs(incy)
    x = rounded([rnd.uniform(-4, 4) for _ in range(lx)], BT)
    if op >= 3 and n > 3:
        x[blas_pos(n - 2, n, incx)] = 9.5   # the extreme twice: the first index wins
        x[blas_pos(n - 1, n, incx)] = -9.5
        x[blas_pos(1, n, incx)] = 0.0
        x[blas_pos(2, n, incx)] = -0.0
    y = rounded([rnd.uniform(-1, 1) for _ in range(ly)], BT)
    (part,) = run(a, obj, K("agx_blas_reduce"), [("buf", pack(x, BT)), ("buf", pack(y, BT)), ("buf", bytes((16 if BT == "d" else 8) * threads), True),
                                              ("i64", n), ("i64", incx), ("i64", incy), ("i64", op), ("i64", threads)],
                  (-(-threads // 32) * 32, 1), (32, 1), tmp)
    (res,) = run(a, obj, K("agx_blas_reduce_final"), [("buf", part), ("buf", bytes(8 if BT == "d" else 4), True), ("i64", threads), ("i64", op)],
                 (32, 1), (32, 1), tmp)
    xs = [x[blas_pos(i, n, incx)] for i in range(n)]
    if op >= 3:
        got = unpack(res, "q" if BT == "d" else "i")[0]
        if n == 0:
            want = 0
        else:
            ext = max(map(abs, xs)) if op == 3 else min(map(abs, xs))
            want = 1 + [abs(v) for v in xs].index(ext)
        return [] if got == want else [("index", got, want)]
    got = unpack(res, BT)[0]
    if op == 0:
        want = sum(v * y[blas_pos(i, n, incy)] for i, v in enumerate(xs))
    elif op == 1:
        want = sum(map(abs, xs))
    else:
        want = math.sqrt(sum(v * v for v in xs))
    return close([got], [want], [0], TOL(1e-5))


def case_blas_nrm2_scaled(a, obj, tmp, rnd):
    # values whose squares overflow f32: the scaled sum keeps the norm finite
    n = 50
    x = rounded([rnd.uniform(1, 2) * (1e160 if BT == "d" else 1e25) for _ in range(n)], BT)
    (part,) = run(a, obj, K("agx_blas_reduce"), [("buf", pack(x, BT)), ("buf", pack(x, BT)), ("buf", bytes((16 if BT == "d" else 8) * 7), True),
                                              ("i64", n), ("i64", 1), ("i64", 1), ("i64", 2), ("i64", 7)], (32, 1), (32, 1), tmp)
    (res,) = run(a, obj, K("agx_blas_reduce_final"), [("buf", part), ("buf", bytes(8 if BT == "d" else 4), True), ("i64", 7), ("i64", 2)],
                 (32, 1), (32, 1), tmp)
    return close(unpack(res, BT), [math.sqrt(sum(v * v for v in x))], [0], TOL(1e-5))


def case_sgemv(a, obj, tmp, m, n, trans, incx, incy, beta, rnd):
    lda, alpha = m + 3, -1.25
    xlen, ylen = (m, n) if trans else (n, m)
    A = rounded([rnd.uniform(-1, 1) for _ in range(lda * n)], BT)
    x = rounded([rnd.uniform(-1, 1) for _ in range(1 + (xlen - 1) * abs(incx))], BT)
    y = rounded([rnd.uniform(-1, 1) for _ in range(1 + (ylen - 1) * abs(incy))], BT)
    if trans:  # a SIMD-group per element of y, as zluda_blas_agx launches it
        (got,) = run(a, obj, K("agx_sgemv_t"), [("buf", pack(A, BT)), ("buf", pack(x, BT)), ("buf", pack(y, BT), True),
                                             ("i64", m), ("i64", n), ("i64", lda), ("i64", incx), ("i64", incy),
                                             (S(), alpha), (S(), beta)], (-(-ylen * 32 // 128) * 128, 1), (128, 1), tmp)
    else:
        (got,) = run(a, obj, K("agx_sgemv"), [("buf", pack(A, BT)), ("buf", pack(x, BT)), ("buf", pack(y, BT), True),
                                           ("i64", m), ("i64", n), ("i64", lda), ("i64", incx), ("i64", incy),
                                           ("i64", trans), (S(), alpha), (S(), beta)], (-(-ylen // 32) * 32, 1), (32, 1), tmp)
    want = list(y)
    for i in range(ylen):
        s = sum((A[k + i * lda] if trans else A[i + k * lda]) * x[blas_pos(k, xlen, incx)] for k in range(xlen))
        p = blas_pos(i, ylen, incy)
        want[p] = alpha * s + (beta * y[p] if beta != 0 else 0.0)
    return close(unpack(got, BT), want, range(len(y)), TOL(1e-5))


def case_sger(a, obj, tmp, m, n, incx, incy, rnd):
    lda, alpha = m + 2, 0.75
    A = rounded([rnd.uniform(-1, 1) for _ in range(lda * n)], BT)
    x = rounded([rnd.uniform(-1, 1) for _ in range(1 + (m - 1) * abs(incx))], BT)
    y = rounded([rnd.uniform(-1, 1) for _ in range(1 + (n - 1) * abs(incy))], BT)
    (got,) = run(a, obj, K("agx_sger"), [("buf", pack(A, BT), True), ("buf", pack(x, BT)), ("buf", pack(y, BT)), ("i64", m), ("i64", n),
                                      ("i64", lda), ("i64", incx), ("i64", incy), (S(), alpha)],
                 (-(-m // 16) * 16, -(-n // 4) * 4), (16, 4), tmp)
    want = list(A)
    for j in range(n):
        for i in range(m):
            want[i + j * lda] += alpha * x[blas_pos(i, m, incx)] * y[blas_pos(j, n, incy)]
    return close(unpack(got, BT), want, range(len(A)), TOL(1e-5))


class T4:
    """A 4-D f32 tensor N C H W with padded (or permuted) strides."""

    def __init__(self, dims, rnd, pad=1, nhwc=False, lo=-2, hi=2):
        N, C, H, W = dims
        self.dims = dims
        if nhwc:
            self.strides = (H * W * C + pad, 1, W * C, C)
        else:
            self.strides = ((C * H * (W + pad)), H * (W + pad), W + pad, 1)
        self.len = sum((d - 1) * s for d, s in zip(dims, self.strides)) + 1
        self.data = rounded([rnd.uniform(lo, hi) for _ in range(self.len)], "f")

    def off(self, n, c, h, w):
        s = self.strides
        return n * s[0] + c * s[1] + h * s[2] + w * s[3]

    def idx(self):
        N, C, H, W = self.dims
        return [(n, c, h, w) for n in range(N) for c in range(C) for h in range(H) for w in range(W)]

    def positions(self):
        return [self.off(*i) for i in self.idx()]


def dnn_shape(t):
    return [("i64", t.dims[1]), ("i64", t.dims[2]), ("i64", t.dims[3])]


def dnn_strides(t):
    return [("i64", v) for v in t.strides]


def act_ref(x, mode, coef):
    if mode == 0:
        return 1 / (1 + math.exp(-x))
    if mode == 1:
        return max(x, 0.0)
    if mode == 2:
        return math.tanh(x)
    if mode == 3:
        return min(max(x, 0.0), coef)
    if mode == 4:
        return x if x > 0 else coef * math.expm1(x)
    if mode == 5:
        return x
    return x / (1 + math.exp(-coef * x))


def act_grad_ref(x, y, dy, mode, coef):
    if mode == 0:
        return dy * y * (1 - y)
    if mode == 1:
        return dy if x > 0 else 0.0
    if mode == 2:
        return dy * (1 - y * y)
    if mode == 3:
        return dy if 0 < x < coef else 0.0
    if mode == 4:
        return dy if x > 0 else dy * (y + coef)
    if mode == 5:
        return dy
    s = 1 / (1 + math.exp(-coef * x))
    return dy * (s + coef * x * s * (1 - s))


def blend(r, old, alpha, beta):
    return alpha * r + (beta * old if beta != 0 else 0.0)


def case_act(a, obj, tmp, mode, bwd, beta, rnd):
    dims, coef, alpha = (2, 3, 4, 5), 1.5, 1.25
    x, y = T4(dims, rnd, 1), T4(dims, rnd, 0, nhwc=True)
    # values near 0 too, for the series of tanh and elu
    for k in range(0, x.len, 7):
        x.data[k] = rounded([x.data[k] * 1e-3], "f")[0]
    total = 2 * 3 * 4 * 5
    if not bwd:
        (got,) = run(a, obj, "agx_dnn_act_fwd", [("buf", pack(x.data)), ("buf", pack(y.data), True), ("i64", total)]
                     + dnn_shape(x) + dnn_strides(x) + dnn_strides(y) + [("i64", mode), ("f32", coef), ("f32", alpha),
                                                                         ("f32", beta)], (-(-total // 32) * 32, 1), (32, 1), tmp)
        want = list(y.data)
        for i in x.idx():
            want[y.off(*i)] = blend(act_ref(x.data[x.off(*i)], mode, coef), y.data[y.off(*i)], alpha, beta)
        return close(unpack(got), want, y.positions(), 2e-6)
    yv = T4(dims, rnd, 2)
    for i in x.idx():
        yv.data[yv.off(*i)] = act_ref(x.data[x.off(*i)], mode, coef)
    yv.data = rounded(yv.data, "f")
    dy, dx = T4(dims, rnd, 0), T4(dims, rnd, 3)
    (got,) = run(a, obj, "agx_dnn_act_bwd", [("buf", pack(yv.data)), ("buf", pack(dy.data)), ("buf", pack(x.data)),
                                             ("buf", pack(dx.data), True), ("i64", total)] + dnn_shape(x)
                 + dnn_strides(yv) + dnn_strides(dy) + dnn_strides(x) + dnn_strides(dx)
                 + [("i64", mode), ("f32", coef), ("f32", alpha), ("f32", beta)], (-(-total // 32) * 32, 1), (32, 1), tmp)
    want = list(dx.data)
    for i in x.idx():
        g = act_grad_ref(x.data[x.off(*i)], yv.data[yv.off(*i)], dy.data[dy.off(*i)], mode, coef)
        want[dx.off(*i)] = blend(g, dx.data[dx.off(*i)], alpha, beta)
    return close(unpack(got), want, dx.positions(), 2e-5)


def case_pointwise(a, obj, tmp, op, rnd):
    dims = (2, 3, 4, 5)
    c = T4(dims, rnd, 1)
    bias = T4((1, 3, 1, 1), rnd, 0)
    astr = (0, bias.strides[1], 0, 0)
    alpha, beta = 0.75, 0.5
    total = 120
    (got,) = run(a, obj, "agx_dnn_pointwise", [("buf", pack(bias.data)), ("buf", pack(c.data), True), ("i64", total)]
                 + dnn_shape(c) + [("i64", v) for v in astr] + dnn_strides(c) + [("i64", op), ("f32", alpha), ("f32", beta)],
                 (128, 1), (32, 1), tmp)
    want = list(c.data)
    for (n, ch, h, w) in c.idx():
        o = c.off(n, ch, h, w)
        want[o] = [alpha * bias.data[ch * astr[1]] + beta * c.data[o], alpha, alpha * c.data[o]][op]
    return close(unpack(got), want, c.positions(), 1e-6)


def case_softmax(a, obj, tmp, instance, algo, bwd, rnd, dims=(2, 5, 3, 2)):
    alpha, beta = 1.0, 0.25
    N, C, H, W = dims
    x, y = T4(dims, rnd, 1, lo=-4, hi=4), T4(dims, rnd, 0, nhwc=True)
    groups = ([[(n, c, h, w) for c in range(C) for h in range(H) for w in range(W)] for n in range(N)] if instance else
              [[(n, c, h, w) for c in range(C)] for n in range(N) for h in range(H) for w in range(W)])
    rows, K = (N, C * H * W) if instance else (N * H * W, C)
    tail = [("i64", instance), ("i64", algo), ("f32", alpha), ("f32", beta)]
    if not bwd:
        (got,) = run(a, obj, "agx_dnn_softmax_fwd", [("buf", pack(x.data)), ("buf", pack(y.data), True), ("i64", rows),
                                                     ("i64", K)] + dnn_shape(x) + dnn_strides(x) + dnn_strides(y) + tail,
                     (32 * rows, 1), (32, 1), tmp)  # a SIMD-group per row
        want = list(y.data)
        for g in groups:
            vs = [x.data[x.off(*i)] for i in g]
            m = max(vs)
            s = sum(math.exp(v - m) for v in vs)
            for i, v in zip(g, vs):
                r = (v - m) - math.log(s) if algo == 2 else math.exp(v - m) / s
                want[y.off(*i)] = blend(r, y.data[y.off(*i)], alpha, beta)
        return close(unpack(got), want, y.positions(), 1e-5)
    yv, dy, dx = T4(dims, rnd, 2), T4(dims, rnd, 0), T4(dims, rnd, 1, nhwc=True)
    for g in groups:
        vs = [x.data[x.off(*i)] for i in g]
        m, s = max(vs), sum(math.exp(v - max(vs)) for v in vs)
        for i, v in zip(g, vs):
            yv.data[yv.off(*i)] = (v - m) - math.log(s) if algo == 2 else math.exp(v - m) / s
    yv.data = rounded(yv.data, "f")
    (got,) = run(a, obj, "agx_dnn_softmax_bwd", [("buf", pack(yv.data)), ("buf", pack(dy.data)), ("buf", pack(dx.data), True),
                                                 ("i64", rows), ("i64", K)] + dnn_shape(x) + dnn_strides(yv)
                 + dnn_strides(dy) + dnn_strides(dx) + tail, (32 * rows, 1), (32, 1), tmp)  # a SIMD-group per row
    want = list(dx.data)
    for g in groups:
        ys = [yv.data[yv.off(*i)] for i in g]
        ds = [dy.data[dy.off(*i)] for i in g]
        if algo == 2:
            s = sum(ds)
            rs = [d - math.exp(v) * s for v, d in zip(ys, ds)]
        else:
            s = sum(v * d for v, d in zip(ys, ds))
            rs = [v * (d - s) for v, d in zip(ys, ds)]
        for i, r in zip(g, rs):
            want[dx.off(*i)] = blend(r, dx.data[dx.off(*i)], alpha, beta)
    return close(unpack(got), want, dx.positions(), 1e-5)


def pool_ref(x, dims, P, Q, R, S, pad, st, mode):
    N, C, H, W = dims
    out, arg = {}, {}
    for n in range(N):
        for c in range(C):
            for p in range(P):
                for q in range(Q):
                    cells = [(p * st[0] - pad[0] + r, q * st[1] - pad[1] + s) for r in range(R) for s in range(S)]
                    inside = [(h, w) for h, w in cells if 0 <= h < H and 0 <= w < W]
                    if mode in (0, 3):
                        best = None
                        for h, w in inside:
                            if best is None or x.data[x.off(n, c, h, w)] > x.data[x.off(n, c, *best)]:
                                best = (h, w)
                        arg[(n, c, p, q)] = best
                        out[(n, c, p, q)] = x.data[x.off(n, c, *best)] if best else 0.0
                    else:
                        cnt = R * S if mode == 1 else max(len(inside), 1)
                        arg[(n, c, p, q)] = cnt
                        out[(n, c, p, q)] = sum(x.data[x.off(n, c, h, w)] for h, w in inside) / cnt
    return out, arg


def case_pool(a, obj, tmp, mode, bwd, R, S, pad, st, rnd):
    dims, alpha, beta = (2, 2, 7, 6), 1.0, 0.5
    N, C, H, W = dims
    P = (H + 2 * pad[0] - R) // st[0] + 1
    Q = (W + 2 * pad[1] - S) // st[1] + 1
    x = T4(dims, rnd, 1)
    win = [("i64", v) for v in (C, H, W, P, Q, R, S, pad[0], pad[1], st[0], st[1])]
    out, arg = pool_ref(x, dims, P, Q, R, S, pad, st, mode)
    if not bwd:
        y = T4((N, C, P, Q), rnd, 2)
        total = N * C * P * Q
        (got,) = run(a, obj, "agx_dnn_pool_fwd", [("buf", pack(x.data)), ("buf", pack(y.data), True), ("i64", total)] + win
                     + dnn_strides(x) + dnn_strides(y) + [("i64", mode), ("f32", alpha), ("f32", beta)],
                     (-(-total // 32) * 32, 1), (32, 1), tmp)
        want = list(y.data)
        for i, v in out.items():
            want[y.off(*i)] = blend(v, y.data[y.off(*i)], alpha, beta)
        return close(unpack(got), want, y.positions(), 1e-6)
    dy, dx = T4((N, C, P, Q), rnd, 0), T4(dims, rnd, 2, nhwc=True)
    total = N * C * H * W
    (got,) = run(a, obj, "agx_dnn_pool_bwd", [("buf", pack(dy.data)), ("buf", pack(x.data)), ("buf", pack(dx.data), True),
                                              ("i64", total)] + win + dnn_strides(x) + dnn_strides(dy) + dnn_strides(dx)
                 + [("i64", mode), ("f32", alpha), ("f32", beta)], (-(-total // 32) * 32, 1), (32, 1), tmp)
    acc = {i: 0.0 for i in x.idx()}
    for (n, c, p, q), v in arg.items():
        g = dy.data[dy.off(n, c, p, q)]
        if mode in (0, 3):
            if v:
                acc[(n, c) + v] += g
        else:
            for r in range(R):
                for s in range(S):
                    h, w = p * st[0] - pad[0] + r, q * st[1] - pad[1] + s
                    if 0 <= h < H and 0 <= w < W:
                        acc[(n, c, h, w)] += g / v
    want = list(dx.data)
    for i, v in acc.items():
        want[dx.off(*i)] = blend(v, dx.data[dx.off(*i)], alpha, beta)
    return close(unpack(got), want, dx.positions(), 1e-5)


def case_bn(a, obj, tmp, spatial, what, rnd, dims=(3, 4, 2, 3)):
    N, C, H, W = dims
    params = C if spatial else C * H * W
    M = N * H * W if spatial else N
    x = T4(dims, rnd, 1, lo=-3, hi=5)
    pidx = lambda n, c, h, w: c if spatial else (c * H + h) * W + w
    groups = {}
    for i in x.idx():
        groups.setdefault(pidx(*i), []).append(i)
    mean = [sum(x.data[x.off(*i)] for i in groups[p]) / M for p in range(params)]
    var = [sum((x.data[x.off(*i)] - mean[p]) ** 2 for i in groups[p]) / M for p in range(params)]
    eps = 1e-3
    inv = [1 / math.sqrt(v + eps) for v in var]
    P = lambda lo, hi: rounded([rnd.uniform(lo, hi) for _ in range(params)], "f")
    if what == "stats":
        rmean, rvar, factor = P(-1, 1), P(0.5, 2), 0.25
        rm, rv, sm, si = run(a, obj, "agx_dnn_bn_stats",
                             [("buf", pack(x.data)), ("buf", pack(rmean), True), ("buf", pack(rvar), True),
                              ("buf", bytes(4 * params), True), ("buf", bytes(4 * params), True), ("i64", params), ("i64", M)]
                             + dnn_shape(x) + dnn_strides(x) + [("i64", spatial), ("i64", 1), ("f32", factor), ("f32", eps)],
                             (32 * params, 1), (32, 1), tmp)  # a SIMD-group per parameter
        wrm = [(1 - factor) * o + factor * m for o, m in zip(rmean, mean)]
        wrv = [(1 - factor) * o + factor * v * M / (M - 1) for o, v in zip(rvar, var)]
        r = range(params)
        return (close(unpack(rm), wrm, r, 1e-5) + close(unpack(rv), wrv, r, 1e-5) + close(unpack(sm), mean, r, 1e-5)
                + close(unpack(si), inv, r, 1e-4))
    scale, bias = P(0.5, 1.5), P(-1, 1)
    alpha, beta = 1.0, 0.5
    total = N * C * H * W
    if what in ("infer", "apply_inv"):
        given = what == "apply_inv"
        y = T4(dims, rnd, 0, nhwc=True)
        mv = rounded(mean, "f")
        vv = rounded(inv if given else var, "f")
        (got,) = run(a, obj, "agx_dnn_bn_apply", [("buf", pack(x.data)), ("buf", pack(y.data), True), ("buf", pack(scale)),
                                                  ("buf", pack(bias)), ("buf", pack(mv)), ("buf", pack(vv)), ("i64", total)]
                     + dnn_shape(x) + dnn_strides(x) + dnn_strides(y)
                     + [("i64", spatial), ("i64", int(given)), ("f32", eps), ("f32", alpha), ("f32", beta)],
                     (-(-total // 32) * 32, 1), (32, 1), tmp)
        want = list(y.data)
        for i in x.idx():
            p = pidx(*i)
            istd = vv[p] if given else 1 / math.sqrt(vv[p] + eps)
            r = scale[p] * (x.data[x.off(*i)] - mv[p]) * istd + bias[p]
            want[y.off(*i)] = blend(r, y.data[y.off(*i)], alpha, beta)
        return close(unpack(got), want, y.positions(), 1e-5)
    # backward
    dy, dx = T4(dims, rnd, 2), T4(dims, rnd, 1, nhwc=True)
    mv, iv = rounded(mean, "f"), rounded(inv, "f")
    ds0, db0 = P(-1, 1), P(-1, 1)
    ds, db, sums = run(a, obj, "agx_dnn_bn_bwd_params",
                       [("buf", pack(x.data)), ("buf", pack(dy.data)), ("buf", pack(mv)), ("buf", pack(iv)),
                        ("buf", pack(ds0), True), ("buf", pack(db0), True), ("buf", bytes(8 * params), True),
                        ("i64", params), ("i64", M)] + dnn_shape(x) + dnn_strides(x) + dnn_strides(dy)
                       + [("i64", spatial), ("f32", alpha), ("f32", beta)], (32 * params, 1), (32, 1), tmp)
    sdy = [sum(dy.data[dy.off(*i)] for i in groups[p]) for p in range(params)]
    sdyx = [sum(dy.data[dy.off(*i)] * (x.data[x.off(*i)] - mv[p]) * iv[p] for i in groups[p]) for p in range(params)]
    r = range(params)
    bad = (close(unpack(ds), [blend(v, o, alpha, beta) for v, o in zip(sdyx, ds0)], r, 1e-4)
           + close(unpack(db), [blend(v, o, alpha, beta) for v, o in zip(sdy, db0)], r, 1e-4))
    (got,) = run(a, obj, "agx_dnn_bn_bwd_data",
                 [("buf", pack(x.data)), ("buf", pack(dy.data)), ("buf", pack(dx.data), True), ("buf", pack(scale)),
                  ("buf", pack(mv)), ("buf", pack(iv)), ("buf", sums), ("i64", total), ("i64", M)] + dnn_shape(x)
                 + dnn_strides(x) + dnn_strides(dy) + dnn_strides(dx) + [("i64", spatial), ("f32", alpha), ("f32", beta)],
                 (-(-total // 32) * 32, 1), (32, 1), tmp)
    sv = unpack(sums)
    want = list(dx.data)
    for i in x.idx():
        p = pidx(*i)
        xh = (x.data[x.off(*i)] - mv[p]) * iv[p]
        g = scale[p] * iv[p] / M * (M * dy.data[dy.off(*i)] - sv[2 * p] - xh * sv[2 * p + 1])
        want[dx.off(*i)] = blend(g, dx.data[dx.off(*i)], alpha, beta)
    return bad + close(unpack(got), want, dx.positions(), 1e-4)


def mat(rows, cols, ld, rnd, lo=-1, hi=1):
    return rounded([rnd.uniform(lo, hi) for _ in range(ld * cols)], BT)


def at(A, r, c, ld, t=False):
    return A[c + r * ld] if t else A[r + c * ld]


def tri(A, r, c, ld, upper, sym, unit):
    inside = r <= c if upper else r >= c
    if sym:
        return at(A, r, c, ld) if inside else at(A, c, r, ld)
    if unit and r == c:
        return 1.0
    return at(A, r, c, ld) if inside else 0.0


def case_level3(a, obj, tmp, what, opts, rnd):
    m, n, k = 7, 5, 4
    alpha, beta = 1.5, 0.5 if opts.get("beta", True) else 0.0
    if what == "geam":
        ta, tb = opts["ta"], opts["tb"]
        lda, ldb, ldc = (n if ta else m) + 1, (n if tb else m) + 2, m + 3
        A, B = mat(0, m if ta else n, lda, rnd), mat(0, m if tb else n, ldb, rnd)
        C = mat(0, n, ldc, rnd)
        (got,) = run(a, obj, K("agx_sgeam"), [("buf", pack(A, BT)), ("buf", pack(B, BT)), ("buf", pack(C, BT), True), ("i64", m), ("i64", n),
                                           ("i64", lda), ("i64", ldb), ("i64", ldc), ("i64", ta), ("i64", tb), (S(), alpha),
                                           (S(), beta)], (16, 8), (16, 4), tmp)
        want = list(C)
        for j in range(n):
            for i in range(m):
                want[i + j * ldc] = alpha * at(A, i, j, lda, ta) + beta * at(B, i, j, ldb, tb)
        return close(unpack(got, BT), want, range(len(C)), TOL(1e-6))
    if what == "dgmm":
        right, incx = opts["right"], opts["incx"]
        lda, ldc = m + 1, m + 2
        A, C = mat(0, n, lda, rnd), mat(0, n, ldc, rnd)
        L = n if right else m
        x = rounded([rnd.uniform(-1, 1) for _ in range(1 + (L - 1) * abs(incx))], BT)
        (got,) = run(a, obj, K("agx_sdgmm"), [("buf", pack(A, BT)), ("buf", pack(x, BT)), ("buf", pack(C, BT), True), ("i64", m), ("i64", n),
                                           ("i64", lda), ("i64", incx), ("i64", ldc), ("i64", right)], (16, 8), (16, 4), tmp)
        want = list(C)
        for j in range(n):
            for i in range(m):
                want[i + j * ldc] = A[i + j * lda] * x[blas_pos(j if right else i, L, incx)]
        return close(unpack(got, BT), want, range(len(C)), TOL(1e-6))
    if what == "syrk":
        upper, trans = opts["upper"], opts["trans"]
        lda, ldc = (k if trans else n) + 1, n + 2
        A, C = mat(0, n if trans else k, lda, rnd), mat(0, n, ldc, rnd)
        (got,) = run(a, obj, K("agx_ssyrk"), [("buf", pack(A, BT)), ("buf", pack(C, BT), True), ("i64", n), ("i64", k), ("i64", lda),
                                           ("i64", ldc), ("i64", upper), ("i64", trans), (S(), alpha), (S(), beta),
                                           ("i64", opts["nb"])], (16, 8), (16, 4), tmp)
        want = list(C)
        nb = opts["nb"]
        for j in range(n):
            for i in range(n):
                if ((i <= j) if upper else (i >= j)) and (nb == 0 or i // nb == j // nb):
                    s = sum(at(A, i, l, lda, trans) * at(A, j, l, lda, trans) for l in range(k))
                    want[i + j * ldc] = alpha * s + beta * C[i + j * ldc]
        return close(unpack(got, BT), want, range(len(C)), TOL(1e-5))
    if what == "syr2k":
        upper, trans = opts["upper"], opts["trans"]
        lda, ldb, ldc = (k if trans else n) + 1, (k if trans else n) + 3, n + 2
        A, B = mat(0, n if trans else k, lda, rnd), mat(0, n if trans else k, ldb, rnd)
        C = mat(0, n, ldc, rnd)
        (got,) = run(a, obj, K("agx_ssyr2k"), [("buf", pack(A, BT)), ("buf", pack(B, BT)), ("buf", pack(C, BT), True), ("i64", n),
                                              ("i64", k), ("i64", lda), ("i64", ldb), ("i64", ldc), ("i64", upper),
                                              ("i64", trans), (S(), alpha), (S(), beta), ("i64", opts["nb"])],
                     (16, 8), (16, 4), tmp)
        want = list(C)
        nb = opts["nb"]
        for j in range(n):
            for i in range(n):
                if ((i <= j) if upper else (i >= j)) and (nb == 0 or i // nb == j // nb):
                    s = sum(at(A, i, l, lda, trans) * at(B, j, l, ldb, trans) + at(B, i, l, ldb, trans) * at(A, j, l, lda, trans)
                            for l in range(k))
                    want[i + j * ldc] = alpha * s + beta * C[i + j * ldc]
        return close(unpack(got, BT), want, range(len(C)), TOL(1e-5))
    right, upper = opts["right"], opts["upper"]
    na = n if right else m
    lda, ldb = na + 1, m + 2
    A, B = mat(0, na, lda, rnd), mat(0, n, ldb, rnd)
    if what == "full":
        sym, unit = opts["sym"], opts["unit"]
        F = [math.nan] * (na * na)
        (got,) = run(a, obj, K("agx_sfull"), [("buf", pack(A, BT)), ("buf", pack(F, BT), True), ("i64", na), ("i64", lda),
                                           ("i64", upper), ("i64", unit), ("i64", sym)], (16, 8), (16, 4), tmp)
        want = [tri(A, i, j, lda, upper, sym, unit) for j in range(na) for i in range(na)]
        return close(unpack(got, BT), want, range(na * na), TOL(0))
    if what in ("symm", "trmm"):
        sym = what == "symm"
        trans, unit = (0, 0) if sym else (opts["trans"], opts["unit"])
        ldc = m + 3
        C = mat(0, n, ldc, rnd)
        be = beta if sym else 0.0
        (got,) = run(a, obj, K("agx_ssymm_trmm"), [("buf", pack(A, BT)), ("buf", pack(B, BT)), ("buf", pack(C, BT), True), ("i64", m), ("i64", n),
                                                ("i64", lda), ("i64", ldb), ("i64", ldc), ("i64", right), ("i64", upper),
                                                ("i64", trans), ("i64", unit), ("i64", int(sym)), (S(), alpha), (S(), be)],
                     (16, 8), (16, 4), tmp)
        opA = lambda r, c: tri(A, c, r, lda, upper, sym, unit) if trans else tri(A, r, c, lda, upper, sym, unit)
        want = list(C)
        for j in range(n):
            for i in range(m):
                if right:
                    s = sum(B[i + l * ldb] * opA(l, j) for l in range(n))
                else:
                    s = sum(opA(i, l) * B[l + j * ldb] for l in range(m))
                want[i + j * ldc] = alpha * s + (be * C[i + j * ldc] if be else 0.0)
        return close(unpack(got, BT), want, range(len(C)), TOL(1e-5))
    # trsm: a well-conditioned triangle
    trans, unit = opts["trans"], opts["unit"]
    for d in range(na):
        A[d + d * lda] = 2.0 + d * 0.25
    opA = lambda r, c: tri(A, c, r, lda, upper, False, unit) if trans else tri(A, r, c, lda, upper, False, unit)
    lower = int((not upper) ^ bool(trans) ^ bool(right))
    (got,) = run(a, obj, K("agx_strsm"), [("buf", pack(A, BT)), ("buf", pack(B, BT), True), ("i64", m), ("i64", n), ("i64", lda),
                                       ("i64", ldb), ("i64", right), ("i64", upper), ("i64", trans), ("i64", unit),
                                       ("i64", lower), (S(), alpha)], (32, 1), (32, 1), tmp)
    X = unpack(got, BT)
    # check op(A) X = alpha B (left) or X op(A) = alpha B (right)
    bad = []
    for j in range(n):
        for i in range(m):
            if right:
                s = sum(X[i + l * ldb] * opA(l, j) for l in range(n))
            else:
                s = sum(opA(i, l) * X[l + j * ldb] for l in range(m))
            if abs(s - alpha * B[i + j * ldb]) > TOL(1e-4) * (1 + abs(alpha * B[i + j * ldb])):
                bad.append(i + j * ldb)
    return bad


# ---- complex BLAS (blasc.mlir; BT selects the C kernels or their f64 Z copies)

def crand(n, rnd):
    zs = [complex(rnd.uniform(-1, 1), rnd.uniform(-1, 1)) for _ in range(n)]
    re = rounded(interleave(zs), BT)
    return [complex(re[2 * i], re[2 * i + 1]) for i in range(n)]


def cpack(zs):
    return pack(interleave(zs), BT)


def cunpack(b):
    v = unpack(b, BT)
    return [complex(v[2 * i], v[2 * i + 1]) for i in range(len(v) // 2)]


def cclose(got, want, idx, tol):
    bad = []
    for i in idx:
        if abs(got[i] - want[i]) > tol * (1 + abs(want[i])):
            bad.append((i, got[i], want[i]))
    return bad


def cop(A, r, c, ld, op):
    if op == 0:
        return A[r + c * ld]
    v = A[c + r * ld]
    return v.conjugate() if op == 2 else v


def cherm(A, r, c, ld, upper, herm):
    inside = r <= c if upper else r >= c
    if inside:
        v = A[r + c * ld]
    else:
        v = A[c + r * ld]
        if herm:
            v = v.conjugate()
    if herm and r == c:
        v = complex(v.real, 0)
    return v


def cs(z):
    return [(S(), z.real), (S(), z.imag)]


def case_cblas(a, obj, tmp, what, o, rnd):
    alpha, beta = complex(1.5, -0.5), complex(0.5, 0.25) if o.get("beta", True) else 0j
    tol = TOL(2e-5)
    if what == "gemm":
        m, n, k = 5, 4, 3
        opa, opb = o["opa"], o["opb"]
        lda, ldb, ldc = (k if opa else m) + 1, (n if opb else k) + 2, m + 1
        A, B, C = crand(lda * (m if opa else k), rnd), crand(ldb * (k if opb else n), rnd), crand(ldc * n, rnd)
        (got,) = run(a, obj, K("agx_cgemm"), [("buf", cpack(A)), ("buf", cpack(B)), ("buf", cpack(C), True), ("i64", m), ("i64", n),
                                              ("i64", k), ("i64", lda), ("i64", ldb), ("i64", ldc), ("i64", opa), ("i64", opb)]
                     + cs(alpha) + cs(beta), (16, 8), (16, 4), tmp)
        want = list(C)
        for j in range(n):
            for i in range(m):
                want[i + j * ldc] = alpha * sum(cop(A, i, l, lda, opa) * cop(B, l, j, ldb, opb) for l in range(k)) + beta * C[i + j * ldc]
        bad = cclose(cunpack(got), want, range(len(C)), tol)
        # on the real GEMM (zluda_blas_agx): the expansion of alpha op(A) (2m x 2k, leading dimension 2m), op(B) as
        # it is or copied (real 2k x n), C scaled by beta first as beta is not real; the real GEMM here
        (xa,) = run(a, obj, K("agx_cgemm_expand_a"), [("buf", cpack(A)), ("buf", pack([math.nan] * (4 * m * k), BT), True), ("i64", m),
                                                     ("i64", k), ("i64", lda), ("i64", opa)] + cs(alpha), (16, 8), (16, 4), tmp)
        if opb:
            (xb,) = run(a, obj, K("agx_cgemm_copy_b"), [("buf", cpack(B)), ("buf", pack([math.nan] * (2 * k * n), BT), True), ("i64", k),
                                                       ("i64", n), ("i64", ldb), ("i64", opb)], (16, 8), (16, 4), tmp)
            rb, ldrb = unpack(xb, BT), 2 * k
        else:
            rb, ldrb = interleave(B), 2 * ldb
        (sc,) = run(a, obj, K("agx_cgemm_scale_c"), [("buf", cpack(C), True), ("i64", m), ("i64", n), ("i64", ldc)] + cs(beta),
                    (16, 8), (16, 4), tmp)
        ra, rc = unpack(xa, BT), list(unpack(sc, BT))
        for j in range(n):
            for i in range(2 * m):
                rc[i + j * 2 * ldc] += sum(ra[i + l * 2 * m] * rb[l + j * ldrb] for l in range(2 * k))
        bad += cclose([complex(rc[2 * i], rc[2 * i + 1]) for i in range(len(C))], want, range(len(C)), tol)
        return bad
    if what == "hemm":
        m, n = 5, 4
        right, upper, herm = o["right"], o["upper"], o["herm"]
        na = n if right else m
        lda, ldb, ldc = na + 1, m + 2, m + 1
        A, B, C = crand(lda * na, rnd), crand(ldb * n, rnd), crand(ldc * n, rnd)
        (got,) = run(a, obj, K("agx_chemm"), [("buf", cpack(A)), ("buf", cpack(B)), ("buf", cpack(C), True), ("i64", m), ("i64", n),
                                              ("i64", lda), ("i64", ldb), ("i64", ldc), ("i64", right), ("i64", upper), ("i64", herm)]
                     + cs(alpha) + cs(beta), (16, 8), (16, 4), tmp)
        want = list(C)
        for j in range(n):
            for i in range(m):
                if right:
                    v = sum(B[i + l * ldb] * cherm(A, l, j, lda, upper, herm) for l in range(n))
                else:
                    v = sum(cherm(A, i, l, lda, upper, herm) * B[l + j * ldb] for l in range(m))
                want[i + j * ldc] = alpha * v + beta * C[i + j * ldc]
        bad = cclose(cunpack(got), want, range(len(C)), tol)
        # on the real GEMM (zluda_blas_agx): left, alpha A expanded times B's real view; right, alpha B expanded
        # times A made full (its real view); beta C added here
        (full,) = run(a, obj, K("agx_chemm_full"), [("buf", cpack(A)), ("buf", pack([math.nan] * ((8 if not right else 2) * na * na), BT), True),
                                                   ("i64", na), ("i64", lda), ("i64", upper), ("i64", herm), ("i64", int(not right))] + cs(alpha),
                      (16, 8), (16, 4), tmp)
        if right:
            (xb,) = run(a, obj, K("agx_cgemm_expand_a"), [("buf", cpack(B)), ("buf", pack([math.nan] * (4 * m * n), BT), True), ("i64", m),
                                                         ("i64", n), ("i64", ldb), ("i64", 0)] + cs(alpha), (16, 8), (16, 4), tmp)
            ra, lra, rb, lrb, kk = unpack(xb, BT), 2 * m, unpack(full, BT), 2 * n, 2 * n
        else:
            ra, lra, rb, lrb, kk = unpack(full, BT), 2 * m, interleave(B), 2 * ldb, 2 * m
        rc = [0.0] * (2 * ldc * n)
        for j in range(n):
            for i in range(2 * m):
                rc[i + j * 2 * ldc] = sum(ra[i + l * lra] * rb[l + j * lrb] for l in range(kk))
        bad += cclose([complex(rc[2 * i], rc[2 * i + 1]) + beta * C[i] for i in range(len(C))], want,
                      [i + j * ldc for j in range(n) for i in range(m)], tol)
        return bad
    if what == "rankk":
        n, k = 5, 3
        upper, op, herm, two = o["upper"], o["op"], o["herm"], o["two"]
        a1 = complex(alpha.real, 0) if herm and not two else alpha
        a2 = (alpha.conjugate() if herm else alpha) if two else 0j
        b = complex(beta.real, 0) if herm else beta
        lda, ldb, ldc = (k if op else n) + 1, (k if op else n) + 2, n + 1
        A, B, C = crand(lda * (n if op else k), rnd), crand(ldb * (n if op else k), rnd), crand(ldc * n, rnd)
        if not two:
            B, ldb = A, lda
        (got,) = run(a, obj, K("agx_crankk"), [("buf", cpack(A)), ("buf", cpack(B)), ("buf", cpack(C), True), ("i64", n), ("i64", k),
                                               ("i64", lda), ("i64", ldb), ("i64", ldc), ("i64", upper), ("i64", op), ("i64", herm),
                                               ("i64", two)] + cs(a1) + cs(a2) + cs(b) + [("i64", o["nb"])], (16, 8), (16, 4), tmp)
        cj = (lambda z: z.conjugate()) if herm else (lambda z: z)
        want = list(C)
        nb = o["nb"]
        for j in range(n):
            for i in range(n):
                if ((i <= j) if upper else (i >= j)) and (nb == 0 or i // nb == j // nb):
                    v = a1 * sum(cop(A, i, l, lda, op) * cj(cop(B, j, l, ldb, op)) for l in range(k))
                    if two:
                        v += a2 * sum(cop(B, i, l, ldb, op) * cj(cop(A, j, l, lda, op)) for l in range(k))
                    old = C[i + j * ldc]
                    if herm and i == j:
                        old = complex(old.real, 0)
                    r = v + b * old
                    want[i + j * ldc] = complex(r.real, 0) if herm and i == j else r
        return cclose(cunpack(got), want, range(len(C)), tol)
    if what == "gemv":
        m, n, op, hmode, incx, incy = o.get("m", 6), 4, o["op"], o.get("hmode", 0), o["incx"], o["incy"]
        if hmode:
            m = n
        lda = m + 2
        ylen, xlen = (n, n) if hmode else ((n, m) if op else (m, n))
        A = crand(lda * n, rnd)
        x, y = crand(1 + (xlen - 1) * abs(incx), rnd), crand(1 + (ylen - 1) * abs(incy), rnd)
        if op and not hmode:  # a SIMD-group per element of y, as zluda_blas_agx launches it
            (got,) = run(a, obj, K("agx_cgemv_t"), [("buf", cpack(A)), ("buf", cpack(x)), ("buf", cpack(y), True),
                                                    ("i64", m), ("i64", n), ("i64", lda), ("i64", incx), ("i64", incy),
                                                    ("i64", op)] + cs(alpha) + cs(beta), (32 * ylen, 1), (32, 1), tmp)
        else:
            (got,) = run(a, obj, K("agx_cgemv"), [("buf", cpack(A)), ("buf", cpack(x)), ("buf", cpack(y), True), ("i64", m),
                                                  ("i64", n), ("i64", lda), ("i64", incx), ("i64", incy), ("i64", op),
                                                  ("i64", hmode)] + cs(alpha) + cs(beta), grid1(ylen), (32, 1), tmp)
        want = list(y)
        for i in range(ylen):
            elem = (lambda l: cherm(A, i, l, lda, hmode == 1, True)) if hmode else (lambda l: cop(A, i, l, lda, op))
            v = sum(elem(l) * x[blas_pos(l, xlen, incx)] for l in range(xlen))
            p = blas_pos(i, ylen, incy)
            want[p] = alpha * v + beta * y[p]
        return cclose(cunpack(got), want, range(len(y)), tol)
    if what == "ger":
        m, n, conj, tri, herm, two, incx, incy = 5, 5 if o["tri"] else 4, o["conj"], o["tri"], o["herm"], o["two"], o["incx"], o["incy"]
        lda = m + 1
        A = crand(lda * n, rnd)
        x = crand(1 + (max(m, n) - 1) * abs(incx), rnd)
        y = crand(1 + (max(m, n) - 1) * abs(incy), rnd)
        a1 = complex(alpha.real, 0) if herm and not two else alpha
        a2 = alpha.conjugate() if two else 0j
        if herm and not two:
            y, incy = x, incx
        (got,) = run(a, obj, K("agx_cger"), [("buf", cpack(A), True), ("buf", cpack(x)), ("buf", cpack(y)), ("i64", m), ("i64", n),
                                             ("i64", lda), ("i64", incx), ("i64", incy), ("i64", conj), ("i64", tri), ("i64", herm)]
                     + cs(a1) + cs(a2), (16, 8), (16, 4), tmp)
        cj = (lambda z: z.conjugate()) if conj else (lambda z: z)
        want = list(A)
        for j in range(n):
            for i in range(m):
                if tri == 1 and i > j or tri == 2 and i < j:
                    continue
                v = a1 * x[blas_pos(i, m, incx)] * cj(y[blas_pos(j, n, incy)])
                if two:
                    v += a2 * y[blas_pos(i, m, incy)] * cj(x[blas_pos(j, n, incx)])
                r = A[i + j * lda] + v
                want[i + j * lda] = complex(r.real, 0) if herm and i == j else r
        return cclose(cunpack(got), want, range(len(A)), tol)
    if what == "vec":
        op, n, incx, incy = o["op"], 9, o["incx"], o["incy"]
        x, y = crand(1 + (n - 1) * abs(incx), rnd), crand(1 + (n - 1) * abs(incy), rnd)
        gx, gy = run(a, obj, K("agx_cblas_vec"), [("buf", cpack(x), True), ("buf", cpack(y), True), ("i64", n), ("i64", incx),
                                                 ("i64", incy), ("i64", op)] + cs(alpha), (32, 1), (32, 1), tmp)
        wx, wy = list(x), list(y)
        for i in range(n):
            px, py = blas_pos(i, n, incx), blas_pos(i, n, incy)
            if op == 0:
                wy[py] = alpha * x[px] + y[py]
            elif op == 1:
                wx[px] = alpha * x[px]
            elif op == 2:
                wy[py] = x[px]
            else:
                wx[px], wy[py] = y[py], x[px]
        return cclose(cunpack(gx), wx, range(len(x)), tol) + cclose(cunpack(gy), wy, range(len(y)), tol)
    if what in ("trmm", "trsm"):
        m, n = 5, 4
        right, upper, op, unit = o["right"], o["upper"], o["op"], o["unit"]
        na = n if right else m
        lda, ldb, ldc = na + 1, m + 2, m + 1
        A, B = crand(lda * na, rnd), crand(ldb * n, rnd)

        def tri(r, c):
            sr, sc = (c, r) if op else (r, c)
            inside = sr <= sc if upper else sr >= sc
            if unit and sr == sc:
                return 1 + 0j
            return cop(A, r, c, lda, op) if inside else 0j
        if what == "trmm":
            C = crand(ldc * n, rnd)
            (got,) = run(a, obj, K("agx_ctrmm"), [("buf", cpack(A)), ("buf", cpack(B)), ("buf", cpack(C), True), ("i64", m), ("i64", n),
                                                  ("i64", lda), ("i64", ldb), ("i64", ldc), ("i64", right), ("i64", upper), ("i64", op),
                                                  ("i64", unit)] + cs(alpha), (16, 8), (16, 4), tmp)
            want = list(C)
            for j in range(n):
                for i in range(m):
                    if right:
                        v = sum(B[i + l * ldb] * tri(l, j) for l in range(n))
                    else:
                        v = sum(tri(i, l) * B[l + j * ldb] for l in range(m))
                    want[i + j * ldc] = alpha * v
            bad = cclose(cunpack(got), want, range(len(C)), tol)
            # the triangle made full (zluda_blas_agx's trmm on the GEMM): op of it is tri
            (full,) = run(a, obj, K("agx_ctri_full"), [("buf", cpack(A)), ("buf", pack([math.nan] * (2 * na * na), BT), True), ("i64", na),
                                                      ("i64", lda), ("i64", upper), ("i64", unit)], (16, 8), (16, 4), tmp)
            F = cunpack(full)
            bad += cclose([cop(F, r, c, na, op) for c in range(na) for r in range(na)], [tri(r, c) for c in range(na) for r in range(na)],
                          range(na * na), 0)
            return bad
        for d in range(na):
            A[d + d * lda] = complex(2.0 + 0.25 * d, 0.5)
        lower = int((not upper) ^ bool(op) ^ bool(right))
        (got,) = run(a, obj, K("agx_ctrsm"), [("buf", cpack(A)), ("buf", cpack(B), True), ("i64", m), ("i64", n), ("i64", lda),
                                              ("i64", ldb), ("i64", right), ("i64", upper), ("i64", op), ("i64", unit), ("i64", lower)]
                     + cs(alpha), (32, 1), (32, 1), tmp)
        X = cunpack(got)
        bad = []
        for j in range(n):
            for i in range(m):
                if right:
                    v = sum(X[i + l * ldb] * tri(l, j) for l in range(n))
                else:
                    v = sum(tri(i, l) * X[l + j * ldb] for l in range(m))
                w = alpha * B[i + j * ldb]
                if abs(v - w) > TOL(1e-4) * (1 + abs(w)):
                    bad.append((i, j, v, w))
        return bad
    if what == "geam":
        m, n, opa, opb = 5, 4, o["opa"], o["opb"]
        lda, ldb, ldc = (n if opa else m) + 1, (n if opb else m) + 2, m + 1
        A, B, C = crand(lda * (m if opa else n), rnd), crand(ldb * (m if opb else n), rnd), crand(ldc * n, rnd)
        (got,) = run(a, obj, K("agx_cgeam"), [("buf", cpack(A)), ("buf", cpack(B)), ("buf", cpack(C), True), ("i64", m), ("i64", n),
                                              ("i64", lda), ("i64", ldb), ("i64", ldc), ("i64", opa), ("i64", opb)]
                     + cs(alpha) + cs(beta), (16, 8), (16, 4), tmp)
        want = list(C)
        for j in range(n):
            for i in range(m):
                want[i + j * ldc] = alpha * cop(A, i, j, lda, opa) + beta * cop(B, i, j, ldb, opb)
        return cclose(cunpack(got), want, range(len(C)), tol)
    if what == "dgmm":
        m, n, right, incx = 5, 4, o["right"], -2
        lda, ldc = m + 1, m + 2
        A, C = crand(lda * n, rnd), crand(ldc * n, rnd)
        L = n if right else m
        x = crand(1 + (L - 1) * 2, rnd)
        (got,) = run(a, obj, K("agx_cdgmm"), [("buf", cpack(A)), ("buf", cpack(x)), ("buf", cpack(C), True), ("i64", m), ("i64", n),
                                              ("i64", lda), ("i64", incx), ("i64", ldc), ("i64", right)], (16, 8), (16, 4), tmp)
        want = list(C)
        for j in range(n):
            for i in range(m):
                want[i + j * ldc] = A[i + j * lda] * x[blas_pos(j if right else i, L, incx)]
        return cclose(cunpack(got), want, range(len(C)), tol)
    # reductions
    op, n, incx, incy, threads = o["op"], o["n"], o["incx"], o["incy"], o["threads"]
    x = crand(1 + max(n - 1, 0) * abs(incx), rnd)
    y = crand(1 + max(n - 1, 0) * abs(incy), rnd)
    if op >= 4 and n > 3:
        x[blas_pos(n - 2, n, incx)] = complex(3.0, -2.0)   # the extreme twice: the first index wins
        x[blas_pos(n - 1, n, incx)] = complex(-2.0, 3.0)
    width = 16 if BT == "d" else 8
    (part,) = run(a, obj, K("agx_cblas_reduce"), [("buf", cpack(x)), ("buf", cpack(y)), ("buf", bytes(width * threads), True),
                                                 ("i64", n), ("i64", incx), ("i64", incy), ("i64", op), ("i64", threads)],
                  (-(-threads // 32) * 32, 1), (32, 1), tmp)
    (res,) = run(a, obj, K("agx_cblas_reduce_final"), [("buf", part), ("buf", bytes(width), True), ("i64", threads), ("i64", op)],
                 (32, 1), (32, 1), tmp)
    xs = [x[blas_pos(i, n, incx)] for i in range(n)]
    if op >= 4:
        got = unpack(res, "q" if BT == "d" else "i")[0]
        mags = [abs(v.real) + abs(v.imag) for v in xs]
        want = 0 if n == 0 else 1 + mags.index(max(mags) if op == 4 else min(mags))
        return [] if got == want else [("index", got, want)]
    got = cunpack(res)[0]
    if op in (0, 1):
        want = sum((v.conjugate() if op == 1 else v) * y[blas_pos(i, n, incy)] for i, v in enumerate(xs))
    elif op == 2:
        want = complex(sum(abs(v.real) + abs(v.imag) for v in xs), 0)
    else:
        want = complex(math.sqrt(sum(v.real ** 2 + v.imag ** 2 for v in xs)), 0)
    return cclose([got], [want], [0], tol)


def cblas_cases(a, obj, tmp, rnd):
    cases = []
    for opa in (0, 1, 2):
        for opb in (0, 1, 2):
            cases.append(("cgemm op %d,%d" % (opa, opb), lambda o={"opa": opa, "opb": opb}: case_cblas(a, obj, tmp, "gemm", o, rnd)))
    cases.append(("cgemm beta 0", lambda: case_cblas(a, obj, tmp, "gemm", {"opa": 2, "opb": 0, "beta": False}, rnd)))
    for right in (0, 1):
        for upper in (0, 1):
            for herm in (0, 1):
                cases.append(("c%smm right %d upper %d" % ("he" if herm else "sy", right, upper),
                              lambda o={"right": right, "upper": upper, "herm": herm}: case_cblas(a, obj, tmp, "hemm", o, rnd)))
    for upper in (0, 1):
        for name, op, herm, two in (("herk N", 0, 1, 0), ("herk C", 2, 1, 0), ("syrk N", 0, 0, 0), ("syrk T", 1, 0, 0),
                                    ("her2k N", 0, 1, 1), ("her2k C", 2, 1, 1), ("syr2k N", 0, 0, 1), ("syr2k T", 1, 0, 1)):
            for nb in (0, 2):  # nb 2: the diagonal blocks only
                cases.append(("c%s upper %d nb %d" % (name, upper, nb),
                              lambda o={"upper": upper, "op": op, "herm": herm, "two": two, "nb": nb}: case_cblas(a, obj, tmp, "rankk", o, rnd)))
    for op in (0, 1, 2):
        cases.append(("cgemv op %d inc -2,3" % op, lambda o={"op": op, "incx": -2, "incy": 3}: case_cblas(a, obj, tmp, "gemv", o, rnd)))
        cases.append(("cgemv op %d m=70 (several elements a lane)" % op,
                      lambda o={"op": op, "incx": 1, "incy": 1, "m": 70}: case_cblas(a, obj, tmp, "gemv", o, rnd)))
    for hmode in (1, 2):
        cases.append(("chemv %s inc 1,-1" % ("upper" if hmode == 1 else "lower"),
                      lambda o={"op": 0, "hmode": hmode, "incx": 1, "incy": -1}: case_cblas(a, obj, tmp, "gemv", o, rnd)))
    for name, conj, tri, herm, two in (("geru", 0, 0, 0, 0), ("gerc", 1, 0, 0, 0), ("her upper", 1, 1, 1, 0),
                                       ("her lower", 1, 2, 1, 0), ("her2 upper", 1, 1, 1, 1), ("her2 lower", 1, 2, 1, 1)):
        cases.append(("c%s inc 2,-1" % name, lambda o={"conj": conj, "tri": tri, "herm": herm, "two": two, "incx": 2, "incy": -1}:
                      case_cblas(a, obj, tmp, "ger", o, rnd)))
    for op, name in ((0, "axpy"), (1, "scal"), (2, "copy"), (3, "swap")):
        cases.append(("c%s inc 2,-3" % name, lambda o={"op": op, "incx": 2, "incy": -3}: case_cblas(a, obj, tmp, "vec", o, rnd)))
    for op, name in ((0, "dotu"), (1, "dotc"), (2, "asum"), (3, "nrm2"), (4, "amax"), (5, "amin")):
        for n, incx, incy, threads in ((40, 1, 1, 7), (9, -2, 3, 4), (0, 1, 1, 2)):
            cases.append(("c%s n=%d inc %d,%d threads %d" % (name, n, incx, incy, threads),
                          lambda o={"op": op, "n": n, "incx": incx, "incy": incy, "threads": threads}: case_cblas(a, obj, tmp, "reduce", o, rnd)))
    for right in (0, 1):
        for upper in (0, 1):
            for op in (0, 1, 2):
                for unit in (0, 1):
                    o = {"right": right, "upper": upper, "op": op, "unit": unit}
                    cases.append(("ctrmm %s" % o, lambda o=o: case_cblas(a, obj, tmp, "trmm", o, rnd)))
                    cases.append(("ctrsm %s" % o, lambda o=o: case_cblas(a, obj, tmp, "trsm", o, rnd)))
    for opa in (0, 2):
        for opb in (0, 1):
            cases.append(("cgeam op %d,%d" % (opa, opb), lambda o={"opa": opa, "opb": opb}: case_cblas(a, obj, tmp, "geam", o, rnd)))
    for right in (0, 1):
        cases.append(("cdgmm right %d" % right, lambda o={"right": right}: case_cblas(a, obj, tmp, "dgmm", o, rnd)))
    if "agx_cgemm_f64" in kernel_records(obj, a, tmp):
        cases += [("z" + label[1:], lambda f=f: with_t("d", f)) for label, f in list(cases)]
    return cases


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--agx-run", required=True)
    ap.add_argument("--objcopy", required=True)
    ap.add_argument("--readelf", required=True)
    ap.add_argument("--chips", default=None)
    ap.add_argument("fat")
    a = ap.parse_args()
    entries = fat_entries(open(a.fat, "rb").read())
    chips = a.chips.split(",") if a.chips else sorted(entries)
    failed = 0
    rnd = random.Random(1)
    with tempfile.TemporaryDirectory() as tmp:
        for chip in chips:
            obj = entries[chip]
            # A CHIP+agx-strict entry runs with agx_run's strict check: any instruction Mesa's compiler would not
            # emit for that GPU fails the case.
            if chip.endswith("+agx-strict"):
                os.environ["AGX_STRICT"] = chip[:-len("+agx-strict")]
            else:
                os.environ.pop("AGX_STRICT", None)
            records = kernel_records(obj, a, tmp)
            if "agx_dnn_act_fwd" in records:
                cases = []
                for mode in range(7):
                    for bwd in (0, 1):
                        for beta in (0.0, 0.5):
                            cases.append(("dnn act mode %d %s beta %g" % (mode, "bwd" if bwd else "fwd", beta),
                                          lambda m=mode, b=bwd, be=beta: case_act(a, obj, tmp, m, b, be, rnd)))
                for kind in (0, 1):
                    cases.append(("dnn convert %s" % ("f16", "bf16")[kind], lambda k=kind: case_dnn_convert(a, obj, tmp, k, rnd)))
                for start in range(0, len(PW_MODES), 8):
                    modes = PW_MODES[start:start + 8]
                    cases.append(("dnn graph pointwise %d-%d" % (modes[0], modes[-1]), lambda m=modes: case_pw_graph(a, obj, tmp, m, rnd)))
                for op in range(9):
                    cases.append(("dnn graph reduction op %d" % op, lambda o=op: case_reduce_graph(a, obj, tmp, o, rnd)))
                for op, label in ((0, "add bias"), (1, "set"), (2, "scale")):
                    cases.append(("dnn %s" % label, lambda op=op: case_pointwise(a, obj, tmp, op, rnd)))
                for inst in (0, 1):
                    for algo in (0, 1, 2):
                        for bwd in (0, 1):
                            cases.append(("dnn softmax %s algo %d %s" % ("instance" if inst else "channel", algo, "bwd" if bwd else "fwd"),
                                          lambda i=inst, al=algo, b=bwd: case_softmax(a, obj, tmp, i, al, b, rnd)))
                for algo in (1, 2):  # rows longer than a SIMD-group
                    for bwd in (0, 1):
                        cases.append(("dnn softmax instance algo %d %s 2x4x5x3" % (algo, "bwd" if bwd else "fwd"),
                                      lambda al=algo, b=bwd: case_softmax(a, obj, tmp, 1, al, b, rnd, (2, 4, 5, 3))))
                for mode in (0, 1, 2):
                    for bwd in (0, 1):
                        for R, S, pad, st in ((3, 3, (1, 1), (2, 2)), (2, 3, (0, 1), (1, 2))):
                            cases.append(("dnn pool mode %d %s %dx%d pad %s stride %s" % (mode, "bwd" if bwd else "fwd", R, S, pad, st),
                                          lambda m=mode, b=bwd, R=R, S=S, p=pad, s=st: case_pool(a, obj, tmp, m, b, R, S, p, s, rnd)))
                for sp in (1, 0):
                    for what in ("stats", "infer", "apply_inv", "bwd"):
                        cases.append(("dnn batchnorm %s %s" % ("spatial" if sp else "per-activation", what),
                                      lambda sp=sp, w=what: case_bn(a, obj, tmp, sp, w, rnd)))
                for what in ("stats", "bwd"):  # several elements a lane
                    cases.append(("dnn batchnorm spatial %s 2x3x6x7" % what,
                                  lambda w=what: case_bn(a, obj, tmp, 1, w, rnd, (2, 3, 6, 7))))
            elif "agx_blas_vec" in records:
                cases = []
                for op, label in ((0, "axpy"), (1, "scal"), (2, "copy"), (3, "swap")):
                    for n, incx, incy in ((37, 1, 1), (20, 3, -2), (9, -1, 4)):
                        cases.append(("blas %s n=%d inc %d,%d" % (label, n, incx, incy),
                                      lambda op=op, n=n, ix=incx, iy=incy: case_blas_vec(a, obj, tmp, op, n, ix, iy, rnd)))
                for op, label in ((0, "dot"), (1, "asum"), (2, "nrm2"), (3, "amax"), (4, "amin")):
                    for n, incx, incy, threads in ((100, 1, 1, 7), (33, 2, -3, 64), (5, -1, 1, 3), (0, 1, 1, 4)):
                        cases.append(("blas %s n=%d inc %d,%d threads %d" % (label, n, incx, incy, threads),
                                      lambda op=op, n=n, ix=incx, iy=incy, t=threads:
                                      case_blas_reduce(a, obj, tmp, op, n, ix, iy, t, rnd)))
                cases.append(("blas nrm2 without overflow", lambda: case_blas_nrm2_scaled(a, obj, tmp, rnd)))
                for trans in (0, 1):
                    for beta in (0.0, 0.5):
                        cases.append(("sgemv 13x9 trans %d beta %g inc -2,3" % (trans, beta),
                                      lambda t=trans, b=beta: case_sgemv(a, obj, tmp, 13, 9, t, -2, 3, b, rnd)))
                    cases.append(("sgemv 40x5 trans %d inc 1,1" % trans,
                                  lambda t=trans: case_sgemv(a, obj, tmp, 40, 5, t, 1, 1, 1.0, rnd)))
                    cases.append(("sgemv 100x7 trans %d beta 0.5 inc -1,1" % trans,
                                  lambda t=trans: case_sgemv(a, obj, tmp, 100, 7, t, -1, 1, 0.5, rnd)))
                for ta in (0, 1):
                    for tb in (0, 1):
                        cases.append(("sgeam ta %d tb %d" % (ta, tb), lambda ta=ta, tb=tb: case_level3(a, obj, tmp, "geam", {"ta": ta, "tb": tb}, rnd)))
                for right in (0, 1):
                    cases.append(("sdgmm right %d inc -2" % right, lambda r=right: case_level3(a, obj, tmp, "dgmm", {"right": r, "incx": -2}, rnd)))
                for upper in (0, 1):
                    for trans in (0, 1):
                        for nb in (0, 2):  # nb 2: the diagonal blocks only
                            cases.append(("ssyrk upper %d trans %d nb %d" % (upper, trans, nb),
                                          lambda u=upper, t=trans, nb=nb: case_level3(a, obj, tmp, "syrk", {"upper": u, "trans": t, "nb": nb}, rnd)))
                        for nb in (0, 2):
                            cases.append(("ssyr2k upper %d trans %d nb %d" % (upper, trans, nb),
                                          lambda u=upper, t=trans, nb=nb: case_level3(a, obj, tmp, "syr2k", {"upper": u, "trans": t, "nb": nb}, rnd)))
                    for right in (0, 1):
                        cases.append(("ssymm right %d upper %d" % (right, upper),
                                      lambda u=upper, r=right: case_level3(a, obj, tmp, "symm", {"upper": u, "right": r}, rnd)))
                        for sym, unit in ((1, 0), (0, 0), (0, 1)):
                            o = {"upper": upper, "right": right, "sym": sym, "unit": unit}
                            cases.append(("sfull %s" % o, lambda o=o: case_level3(a, obj, tmp, "full", o, rnd)))
                        for trans in (0, 1):
                            for unit in (0, 1):
                                o = {"upper": upper, "right": right, "trans": trans, "unit": unit}
                                cases.append(("strmm %s" % o, lambda o=o: case_level3(a, obj, tmp, "trmm", o, rnd)))
                                cases.append(("strsm %s" % o, lambda o=o: case_level3(a, obj, tmp, "trsm", o, rnd)))
                cases += [("sger 11x7 inc 1,1", lambda: case_sger(a, obj, tmp, 11, 7, 1, 1, rnd)),
                          ("sger 6x13 inc -3,2", lambda: case_sger(a, obj, tmp, 6, 13, -3, 2, rnd))]
                if "agx_blas_vec_f64" in records:
                    # the same cases on the f64 copies
                    cases += [("f64 " + label, lambda f=f: with_t("d", f)) for label, f in list(cases)]
            elif "agx_cgemm" in records:
                cases = cblas_cases(a, obj, tmp, rnd)
            elif "agx_lu_pivot" in records:
                cases = [("solver %s" % w, lambda w=w: case_solver(a, obj, tmp, w, rnd))
                         for w in ("lu", "lu_t", "chol_l", "chol_u", "chol_fail", "qr")]
                cases.append(("solver qr_wy", lambda: case_qr_wy(a, obj, tmp, rnd)))
                cases += [("solver %s" % w, lambda w=w: case_eigsvd(a, obj, tmp, w, rnd)) for w in ("svd", "eig_l", "eig_u")]
                if "agx_lu_pivot_f64" in records:
                    cases += [("f64 " + label, lambda f=f: with_t("d", f)) for label, f in list(cases)]
            elif "agx_rand32" in records:
                cases = [("rand %s" % w, lambda w=w: case_rand(a, obj, tmp, w, rnd))
                         for w in ("r320", "r321", "r322", "r323", "r641", "r642", "r643", "poisson")]
            elif "agx_csr_spmv_i32" in records:
                cases = []
                for I in ("i32", "i64"):
                    for base in (0, 1):
                        for what in ("csr", "csr_t", "coo", "coo_t", "spmm_col", "spmm_row"):
                            cases.append(("sparse %s %s base %d" % (what, I, base),
                                          lambda w=what, I=I, b=base: case_sparse(a, obj, tmp, w, I, b, rnd)))
                        ops = ["spvv", "axpby", "gather", "scatter", "rot", "csr2dense", "coo2dense", "dense2csr", "sddmm"]
                        ops += ["spsm-%d%d%d" % (u, d, t) for u in (0, 1) for d in (0, 1) for t in (0, 1)]
                        for what in ops:
                            cases.append(("sparse %s %s base %d" % (what, I, base),
                                          lambda w=what, I=I, b=base: case_sparse_ops(a, obj, tmp, w, I, b, rnd)))
                        cases.append(("sparse spgemm %s base %d" % (I, base), lambda I=I, b=base: case_spgemm(a, obj, tmp, I, b, rnd)))
                for base in (0, 1):
                    for words in (1, 2):
                        cases.append(("sparse csr2csc base %d %d words" % (base, words),
                                      lambda b=base, w=words: case_csr2csc(a, obj, tmp, b, w, rnd)))
                if "agx_csr_spmv_i32_f64" in records:
                    cases += [("f64 " + label, lambda f=f: with_t("d", f)) for label, f in list(cases)]

            elif "agx_fft_radix2" in records:
                cases = []
                for t in ("f", "d") if "agx_fft_radix2_f64" in records else ("f",):
                    p = " f64" if t == "d" else ""
                    for inv in (0, 1):
                        cases += [
                            ("fft%s radix2 n=16 x3 inverse=%d" % (p, inv), lambda inv=inv, t=t: case_fft(a, obj, tmp, 16, 3, 1, 16, inv, True, rnd, t)),
                            ("fft%s radix2 n=64 x2 inverse=%d" % (p, inv), lambda inv=inv, t=t: case_fft(a, obj, tmp, 64, 2, 1, 64, inv, True, rnd, t)),
                            ("fft%s radix2 n=8 columns of 8x4 inverse=%d" % (p, inv), lambda inv=inv, t=t: case_fft(a, obj, tmp, 8, 4, 4, 1, inv, True, rnd, t)),
                            ("fft%s radix2 only n=32 x2 inverse=%d" % (p, inv), lambda inv=inv, t=t: case_fft(a, obj, tmp, 32, 2, 1, 32, inv, "r2", rnd, t)),
                            ("fft%s n=256 x1 inverse=%d" % (p, inv), lambda inv=inv, t=t: case_fft(a, obj, tmp, 256, 1, 1, 256, inv, True, rnd, t)),
                            ("fft%s n=32 columns of 32x3 inverse=%d" % (p, inv), lambda inv=inv, t=t: case_fft(a, obj, tmp, 32, 3, 3, 1, inv, True, rnd, t)),
                            ("dft%s n=12 x2 inverse=%d" % (p, inv), lambda inv=inv, t=t: case_fft(a, obj, tmp, 12, 2, 1, 12, inv, False, rnd, t)),
                            ("dft%s n=7 columns of 7x3 inverse=%d" % (p, inv), lambda inv=inv, t=t: case_fft(a, obj, tmp, 7, 3, 3, 1, inv, False, rnd, t)),
                        ]
                    cases += [("fft%s packing n=10 x2" % p, lambda t=t: case_fft_packing(a, obj, tmp, 10, 2, rnd, t)),
                              ("fft%s packing n=9 x1" % p, lambda t=t: case_fft_packing(a, obj, tmp, 9, 1, rnd, t))]
            elif "agx_conv2d_fwd_f32" in records:
                convs = [
                    ("3x2 pad 1,0 stride 2,1 dil 1,2 G2", Conv(2, 4, 7, 6, 6, 3, 2, (1, 0), (2, 1), (1, 2), 2, 0, x_w_pad=3)),
                    ("3x3 pad 1 conv mode", Conv(1, 3, 5, 5, 2, 3, 3, (1, 1), (1, 1), (1, 1), 1, 1)),
                    ("1x1 stride 2 depthwise", Conv(2, 4, 6, 5, 4, 1, 1, (0, 0), (2, 2), (1, 1), 4, 0)),
                ]
                cases = []
                for t in ("f", "e"):
                    for which in ("fwd", "bwd_data", "bwd_filter"):
                        for label, cv in convs:
                            for alpha, beta in ((1.0, 0.0), (1.5, 0.5)):
                                cases.append(("conv %s %s %s a=%g b=%g" % (which, "f32" if t == "f" else "f16", label, alpha, beta),
                                              lambda t=t, w=which, cv=cv, al=alpha, be=beta: case_conv(a, obj, tmp, w, cv, al, be, rnd, t)))
                    for label, cv in convs[1:2] + [
                            ("3x2 pad 1,0 stride 2,1 dil 1,2", Conv(2, 4, 7, 6, 6, 3, 2, (1, 0), (2, 1), (1, 2), 1, 0, x_w_pad=3)),
                            ("5x5 pad 2 stride 2 conv mode", Conv(1, 2, 9, 8, 3, 5, 5, (2, 2), (2, 2), (1, 1), 1, 1))]:
                        for fg in (False, True):
                            cases.append(("conv im2col %s %s %s" % ("dw" if fg else "fwd", "f32" if t == "f" else "f16",
                                                                    label),
                                          lambda t=t, cv=cv, fg=fg: case_conv_im2col(a, obj, tmp, cv, rnd, t, fg)))
                        for alpha, beta in ((1.0, 0.0), (1.5, 0.5)):
                            cases.append(("conv col2im dx %s %s a=%g b=%g" % ("f32" if t == "f" else "f16", label, alpha,
                                                                              beta),
                                          lambda t=t, cv=cv, al=alpha, be=beta: case_conv_col2im(a, obj, tmp, cv, al, be,
                                                                                                  rnd, t)))
            elif "agx_epilogue_f32" in records:
                cases = []
                for t in ("f", "e"):
                    for has_bias, act, label in ((1, 0, "bias"), (0, 1, "relu"), (1, 1, "bias+relu"), (1, 2, "bias+gelu"), (0, 2, "gelu")):
                        cases.append(("epilogue %s %s 37x11" % ("f32" if t == "f" else "f16", label),
                                      lambda t=t, hb=has_bias, ac=act: case_epilogue(a, obj, tmp, 37, 11, hb, ac, rnd, t)))
                    cases.append(("epilogue %s batched 3 bias relu" % ("f32" if t == "f" else "f16"),
                                  lambda t=t: case_epilogue(a, obj, tmp, 21, 9, 1, 1, rnd, t, batch=3)))
            elif "agx_sgemm_tile64" in records:
                cases = [
                    ("tile64 128x64x40 a=1.5 b=0.5", lambda: case_tile(a, obj, tmp, 128, 64, 40, 1.5, 0.5, rnd)),
                    ("tile64 64x128x16 a=1 b=0 (C NaN)", lambda: case_tile(a, obj, tmp, 64, 128, 16, 1.0, 0.0, rnd)),
                    ("generic 37x29x13 NN b=0.25", lambda: case_generic(a, obj, tmp, 37, 29, 13, 0, 0, 1.0, 0.25, rnd)),
                    ("generic 20x33x7 TN", lambda: case_generic(a, obj, tmp, 20, 33, 7, 1, 0, -2.0, 1.0, rnd)),
                    ("generic 17x9x31 NT b=0", lambda: case_generic(a, obj, tmp, 17, 9, 31, 0, 1, 0.5, 0.0, rnd)),
                    ("generic 5x40x3 TT", lambda: case_generic(a, obj, tmp, 5, 40, 3, 1, 1, 1.0, -1.0, rnd)),
                    ("dot 300x1x200 TN", lambda: case_generic(a, obj, tmp, 300, 1, 200, 1, 0, 1.0, 0.0, rnd,
                                                              "agx_sgemm_dot")),
                    ("dot 3x70x150 NN b=0.5", lambda: case_generic(a, obj, tmp, 3, 70, 150, 0, 0, -1.5, 0.5, rnd,
                                                                   "agx_sgemm_dot")),
                    ("dot 50x8x33 TT b=1", lambda: case_generic(a, obj, tmp, 50, 8, 33, 1, 1, 0.5, 1.0, rnd,
                                                                "agx_sgemm_dot")),
                    ("split 70x130x21 b=0.5", lambda: case_split(a, obj, tmp, 70, 130, 21, 1.5, 0.5, rnd)),
                    ("split 128x77x8 b=0 (C NaN)", lambda: case_split(a, obj, tmp, 128, 77, 8, 1.0, 0.0, rnd)),
                    ("split 100x64x19", lambda: case_split(a, obj, tmp, 100, 64, 19, -1.0, 1.0, rnd)),
                    ("tile64 ta 64x128x24", lambda: case_tile(a, obj, tmp, 64, 128, 24, 1.0, 0.5, rnd, ta=True)),
                    ("tile64 tb 128x64x16", lambda: case_tile(a, obj, tmp, 128, 64, 16, 1.5, 0.0, rnd, tb=True)),
                    ("tile64 tatb 64x64x40", lambda: case_tile(a, obj, tmp, 64, 64, 40, -1.0, 1.0, rnd, ta=True, tb=True)),
                    ("split TN 77x130x21", lambda: case_split(a, obj, tmp, 77, 130, 21, 1.0, 0.5, rnd, ta=True)),
                    ("split NT 130x70x13 b=0", lambda: case_split(a, obj, tmp, 130, 70, 13, 1.5, 0.0, rnd, tb=True)),
                    ("split TT 64x100x8", lambda: case_split(a, obj, tmp, 64, 100, 8, -1.0, 1.0, rnd, ta=True, tb=True)),
                    ("batched 3 x 70x65x40 TN", lambda: case_batched(a, obj, tmp, 70, 65, 40, 3, 1.0, 0.5, rnd,
                                                                     ta=True)),
                    ("batched 2 x 64x128x16 NT b=0", lambda: case_batched(a, obj, tmp, 64, 128, 16, 2, -1.0, 0.0, rnd,
                                                                          tb=True)),
                    ("batched 4 x 5x1x50 NN (dot)", lambda: case_batched(a, obj, tmp, 5, 1, 50, 4, 1.0, 0.0, rnd)),
                    ("conv 1x1 as GEMM 2x40x8x9 to 72", lambda: case_conv_pointwise(a, obj, tmp, 2, 40, 8, 9, 72,
                                                                                    1.0, 0.5, rnd)),
                    ("conv 1x1 as GEMM 1x3x5x5 to 4 b=0", lambda: case_conv_pointwise(a, obj, tmp, 1, 3, 5, 5, 4,
                                                                                      -0.5, 0.0, rnd)),
                    # strips on the dot kernel: one column of C beside the tile (TN), five rows below it (NN)
                    ("split 70x130x21 odd ld (unaligned tile)",
                     lambda: case_split(a, obj, tmp, 70, 130, 21, 1.5, 0.5, rnd, odd=True)),
                    ("split TT 77x64x13 odd ld (unaligned tile)",
                     lambda: case_split(a, obj, tmp, 77, 64, 13, 1.0, 0.0, rnd, ta=True, tb=True, odd=True)),
                    ("split TN 128x65x64 (dot strip)",
                     lambda: case_split(a, obj, tmp, 128, 65, 64, 1.0, 0.5, rnd, ta=True)),
                    ("split NN 69x128x40 (dot strip)", lambda: case_split(a, obj, tmp, 69, 128, 40, -1.0, 0.0, rnd)),
                ]
                dk = "agx_dgemm_generic"
                cases += [
                    ("d generic 13x11x9 NN b=0.25",
                     lambda: case_generic(a, obj, tmp, 13, 11, 9, 0, 0, 1.0, 0.25, rnd, dk, "d", "d")),
                    ("d generic 9x14x5 TN", lambda: case_generic(a, obj, tmp, 9, 14, 5, 1, 0, -2.0, 1.0, rnd, dk, "d", "d")),
                    ("d generic 7x6x11 NT b=0 (C NaN)",
                     lambda: case_generic(a, obj, tmp, 7, 6, 11, 0, 1, 0.5, 0.0, rnd, dk, "d", "d")),
                    ("d generic 4x10x3 TT", lambda: case_generic(a, obj, tmp, 4, 10, 3, 1, 1, 1.0, -1.0, rnd, dk, "d", "d")),
                    ("d dot 40x2x100 TN b=0.25",
                     lambda: case_generic(a, obj, tmp, 40, 2, 100, 1, 0, 1.0, 0.25, rnd, "agx_dgemm_dot", "d", "d")),
                ]
            else:
                cases = []
                for c in ("e", "f"):
                    ct = "f16" if c == "e" else "f32"
                    tk, gk = "agx_hgemm_tile64_c" + ct, "agx_hgemm_generic_c" + ct
                    cases += [
                        ("h tile64 C %s 64x128x24 b=0.5" % ct,
                         lambda tk=tk, c=c: case_tile(a, obj, tmp, 64, 128, 24, 1.0, 0.5, rnd, tk, "e", c)),
                        ("h tile64 C %s 128x64x8 b=0" % ct,
                         lambda tk=tk, c=c: case_tile(a, obj, tmp, 128, 64, 8, 2.0, 0.0, rnd, tk, "e", c)),
                        ("h generic C %s 21x13x9 TN" % ct,
                         lambda gk=gk, c=c: case_generic(a, obj, tmp, 21, 13, 9, 1, 0, 1.0, 0.5, rnd, gk, "e", c)),
                        ("h generic C %s 6x30x17 NT b=0" % ct,
                         lambda gk=gk, c=c: case_generic(a, obj, tmp, 6, 30, 17, 0, 1, -1.0, 0.0, rnd, gk, "e", c)),
                        ("h dot C %s 64x1x96 TN b=0.5" % ct,
                         lambda gk=gk, c=c: case_generic(a, obj, tmp, 64, 1, 96, 1, 0, 1.0, 0.5, rnd,
                                                         gk.replace("_generic", "_dot"), "e", c)),
                        ("h split C %s 70x130x21 b=0.5" % ct,
                         lambda tk=tk, gk=gk, c=c: case_split(a, obj, tmp, 70, 130, 21, 1.0, 0.5, rnd, tk, gk, "e", c)),
                        ("h tile64 ta C %s 64x64x16" % ct,
                         lambda tk=tk, c=c: case_tile(a, obj, tmp, 64, 64, 16, 1.0, 0.5, rnd, tk, "e", c, ta=True)),
                        ("h split C %s 70x130x21 odd ld (unaligned tile)" % ct,
                         lambda tk=tk, gk=gk, c=c: case_split(a, obj, tmp, 70, 130, 21, 1.0, 0.5, rnd, tk, gk, "e", c,
                                                              odd=True)),
                        ("h split TN C %s 77x130x21" % ct,
                         lambda tk=tk, gk=gk, c=c: case_split(a, obj, tmp, 77, 130, 21, 1.0, 0.5, rnd, tk, gk, "e", c,
                                                              ta=True)),
                    ]
                    cases.append(("h batched C %s 3 x 70x65x24 TN" % ct,
                                  lambda tk=tk, gk=gk, c=c: case_batched(a, obj, tmp, 70, 65, 24, 3, 1.0, 0.5, rnd, tk, gk,
                                                                         "e", c, ta=True)))
                    if ct == "f16":  # f16 convolutions keep f16 data
                        cases.append(("h conv 1x1 as GEMM 2x24x8x8 to 64",
                                      lambda tk=tk, gk=gk: case_conv_pointwise(a, obj, tmp, 2, 24, 8, 8, 64, 1.0, 0.5,
                                                                               rnd, tk, gk, "e")))
            for label, f in cases:
                try:
                    bad = f()
                    status = "ok" if not bad else "WRONG at %d elements, first %s" % (len(bad), bad[:4])
                except RuntimeError as e:
                    bad, status = True, str(e)
                failed += bool(bad)
                print("%-9s %-36s %s" % (chip, label, status))
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
