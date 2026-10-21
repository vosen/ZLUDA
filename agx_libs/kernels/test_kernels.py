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
        _halves, tg_half, _glob, _gprs, _tg, _stack, nargs = struct.unpack_from("<7I", d, q)
        q += 28
        args = [struct.unpack_from("<3I", d, q + 12 * i) for i in range(nargs)]
        out[name] = (tg_half // 2, args)
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


def run(a, obj, name, args, grid, wg, tmp):
    """args: ('buf', bytes, dump) | ('i64', v) | ('f32', v). Returns the dumped buffers."""
    tg_uniform, slots = kernel_records(obj, a, tmp)[name]
    code = os.path.join(tmp, name + ".bin")
    open(code, "wb").write(kernel_code(obj, name, a, tmp))
    cmd = [a.agx_run, code, "--grid", "%d,%d,1" % grid, "--wg", "%d,%d,1" % wg]
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


def case_tile(a, obj, tmp, M, N, K, alpha, beta, rnd, kernel="agx_sgemm_tile64", ab="f", c="f"):
    lda, ldb, ldc = K + 2, N + 2, N + 4
    A = rounded([rnd.uniform(-1, 1) for _ in range(M * lda)], ab)
    B = rounded([rnd.uniform(-1, 1) for _ in range(K * ldb)], ab)
    C = rounded([rnd.uniform(-1, 1) if beta != 0 else math.nan for _ in range(M * ldc)], c)
    (got,) = run(a, obj, kernel,
                 [("buf", pack(A, ab)), ("buf", pack(B, ab)), ("buf", pack(C, c), True), ("i64", M), ("i64", N),
                  ("i64", K), ("i64", lda), ("i64", ldb), ("i64", ldc), ("f32", alpha), ("f32", beta)],
                 ((M // 64) * (N // 64) * 128, 1), (128, 1), tmp)
    want = reference(M, N, K, A, (lda, 1), B, (ldb, 1), C, (ldc, 1), alpha, beta)
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
                  ("i64", sc[1]), ("f64" if c == "d" else "f32", alpha), ("f64" if c == "d" else "f32", beta)],
                 (-(-N // 16) * 16, -(-M // 8) * 8), (16, 8), tmp)
    want = reference(M, N, K, A, sa, B, sb, C, sc, alpha, beta)
    return close(unpack(got, c), want, [i * sc[0] + j * sc[1] for i in range(M) for j in range(N)],
                 {"f": 1e-4, "d": 1e-13}.get(c, 2e-3))


def gelu(x):
    return 0.5 * x * (1 + math.tanh(0.7978845608 * (x + 0.044715 * x ** 3)))


def case_epilogue(a, obj, tmp, M, N, has_bias, act, rnd, t="f"):
    ld = M + 3
    D = rounded([rnd.uniform(-3, 3) for _ in range(ld * N)], t)
    bias = rounded([rnd.uniform(-1, 1) for _ in range(M)], t)
    kernel = "agx_epilogue_f32" if t == "f" else "agx_epilogue_f16"
    (got,) = run(a, obj, kernel,
                 [("buf", pack(D, t), True), ("buf", pack(bias, t)), ("i64", M), ("i64", N), ("i64", ld),
                  ("i64", has_bias), ("i64", act)],
                 (-(-M // 16) * 16, -(-N // 8) * 8), (16, 8), tmp)
    want = list(D)
    for j in range(N):
        for i in range(M):
            x = D[j * ld + i] + (bias[i] if has_bias else 0.0)
            want[j * ld + i] = max(x, 0.0) if act == 1 else gelu(x) if act == 2 else x
    return close(unpack(got, t), want, [j * ld + i for j in range(N) for i in range(M)], 2e-5 if t == "f" else 2e-3)


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
        ns = 1
        while ns < n:
            (buf,) = run(a, obj, "agx_fft_radix2" + sfx,
                         [("buf", buf), ("buf", pack([0.0] * (2 * total), t), True), ("buf", tw), ("i32", n), ("i32", ns),
                          ("i32", batch), ("i32", stride), ("i32", dist), ("i32", inverse)],
                         (n // 2, batch), (min(n // 2, 64), 1), tmp)
            ns *= 2
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
    return rowptr, col, rounded(val, "f")


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
        x = rounded([rnd.uniform(-1, 1) for _ in range(xn)], "f")
        y = rounded([rnd.uniform(-1, 1) for _ in range(yn)], "f")
        want = [alpha * sum(dense.get((j, i) if trans else (i, j), 0.0) * x[j] for j in range(xn)) + beta * y[i]
                for i in range(yn)]
        if what == "csr":
            (got,) = run(a, obj, "agx_csr_spmv_" + I,
                         [("buf", rp), ("buf", ci), ("buf", pack(val)), ("buf", pack(x)), ("buf", pack(y), True),
                          ("i64", rows), ("i64", base), ("f32", alpha), ("f32", beta)], (-(-rows // 32) * 32, 1), (32, 1), tmp)
        else:
            (scaled,) = run(a, obj, "agx_sparse_scale", [("buf", pack(y), True), ("i64", yn), ("i64", 1), ("f32", beta)],
                            (-(-yn // 32) * 32, 1), (32, 1), tmp)
            if what == "csr_t":
                (got,) = run(a, obj, "agx_csr_spmv_t_" + I,
                             [("buf", rp), ("buf", ci), ("buf", pack(val)), ("buf", pack(x)), ("buf", scaled, True),
                              ("i64", rows), ("i64", base), ("f32", alpha)], (-(-rows // 32) * 32, 1), (32, 1), tmp)
            else:
                ri = pack([i + base for i in range(rows) for _ in range(rowptr[i], rowptr[i + 1])], it)
                (got,) = run(a, obj, "agx_coo_spmv_" + I,
                             [("buf", ri), ("buf", ci), ("buf", pack(val)), ("buf", pack(x)), ("buf", scaled, True),
                              ("i64", nnz), ("i64", base), ("i64", int(trans)), ("f32", alpha)],
                             (-(-nnz // 32) * 32, 1), (32, 1), tmp)
        return close(unpack(got), want, range(yn), 1e-5)
    # SpMM: C (rows x n) = alpha A B + beta C, B (cols x n); column-major or row-major dense matrices
    n = 5
    col_major = what == "spmm_col"
    ldb, ldc = (cols + 2, rows + 1) if col_major else (n + 2, n + 1)
    bs = (1, ldb) if col_major else (ldb, 1)
    cs = (1, ldc) if col_major else (ldc, 1)
    B = rounded([rnd.uniform(-1, 1) for _ in range(ldb * (n if col_major else cols))], "f")
    C = rounded([rnd.uniform(-1, 1) for _ in range(ldc * (n if col_major else rows))], "f")
    (got,) = run(a, obj, "agx_csr_spmm_" + I,
                 [("buf", rp), ("buf", ci), ("buf", pack(val)), ("buf", pack(B)), ("buf", pack(C), True),
                  ("i64", rows), ("i64", n), ("i64", base), ("i64", bs[0]), ("i64", bs[1]), ("i64", cs[0]), ("i64", cs[1]),
                  ("f32", alpha), ("f32", beta)], (-(-rows // 16) * 16, -(-n // 4) * 4), (16, 4), tmp)
    want = list(C)
    idx = []
    for i in range(rows):
        for j in range(n):
            o = i * cs[0] + j * cs[1]
            idx.append(o)
            want[o] = alpha * sum(dense.get((i, k), 0.0) * B[k * bs[0] + j * bs[1]] for k in range(cols)) + beta * C[o]
    return close(unpack(got), want, idx, 1e-5)


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
    (got,) = run(a, obj, K("agx_sgemv"), [("buf", pack(A, BT)), ("buf", pack(x, BT)), ("buf", pack(y, BT), True), ("i64", m), ("i64", n),
                                       ("i64", lda), ("i64", incx), ("i64", incy), ("i64", trans), (S(), alpha),
                                       (S(), beta)], (-(-ylen // 32) * 32, 1), (32, 1), tmp)
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


def case_softmax(a, obj, tmp, instance, algo, bwd, rnd):
    dims, alpha, beta = (2, 5, 3, 2), 1.0, 0.25
    N, C, H, W = dims
    x, y = T4(dims, rnd, 1, lo=-4, hi=4), T4(dims, rnd, 0, nhwc=True)
    groups = ([[(n, c, h, w) for c in range(C) for h in range(H) for w in range(W)] for n in range(N)] if instance else
              [[(n, c, h, w) for c in range(C)] for n in range(N) for h in range(H) for w in range(W)])
    rows, K = (N, C * H * W) if instance else (N * H * W, C)
    tail = [("i64", instance), ("i64", algo), ("f32", alpha), ("f32", beta)]
    if not bwd:
        (got,) = run(a, obj, "agx_dnn_softmax_fwd", [("buf", pack(x.data)), ("buf", pack(y.data), True), ("i64", rows),
                                                     ("i64", K)] + dnn_shape(x) + dnn_strides(x) + dnn_strides(y) + tail,
                     (32, 1), (32, 1), tmp)
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
                 + dnn_strides(dy) + dnn_strides(dx) + tail, (32, 1), (32, 1), tmp)
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


def case_bn(a, obj, tmp, spatial, what, rnd):
    dims = (3, 4, 2, 3)
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
                             (32, 1), (32, 1), tmp)
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
                       + [("i64", spatial), ("f32", alpha), ("f32", beta)], (32, 1), (32, 1), tmp)
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
                                           ("i64", ldc), ("i64", upper), ("i64", trans), (S(), alpha), (S(), beta)],
                     (16, 8), (16, 4), tmp)
        want = list(C)
        for j in range(n):
            for i in range(n):
                if (i <= j) if upper else (i >= j):
                    s = sum(at(A, i, l, lda, trans) * at(A, j, l, lda, trans) for l in range(k))
                    want[i + j * ldc] = alpha * s + beta * C[i + j * ldc]
        return close(unpack(got, BT), want, range(len(C)), TOL(1e-5))
    right, upper = opts["right"], opts["upper"]
    na = n if right else m
    lda, ldb = na + 1, m + 2
    A, B = mat(0, na, lda, rnd), mat(0, n, ldb, rnd)
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
                for op, label in ((0, "add bias"), (1, "set"), (2, "scale")):
                    cases.append(("dnn %s" % label, lambda op=op: case_pointwise(a, obj, tmp, op, rnd)))
                for inst in (0, 1):
                    for algo in (0, 1, 2):
                        for bwd in (0, 1):
                            cases.append(("dnn softmax %s algo %d %s" % ("instance" if inst else "channel", algo, "bwd" if bwd else "fwd"),
                                          lambda i=inst, al=algo, b=bwd: case_softmax(a, obj, tmp, i, al, b, rnd)))
                for mode in (0, 1, 2):
                    for bwd in (0, 1):
                        for R, S, pad, st in ((3, 3, (1, 1), (2, 2)), (2, 3, (0, 1), (1, 2))):
                            cases.append(("dnn pool mode %d %s %dx%d pad %s stride %s" % (mode, "bwd" if bwd else "fwd", R, S, pad, st),
                                          lambda m=mode, b=bwd, R=R, S=S, p=pad, s=st: case_pool(a, obj, tmp, m, b, R, S, p, s, rnd)))
                for sp in (1, 0):
                    for what in ("stats", "infer", "apply_inv", "bwd"):
                        cases.append(("dnn batchnorm %s %s" % ("spatial" if sp else "per-activation", what),
                                      lambda sp=sp, w=what: case_bn(a, obj, tmp, sp, w, rnd)))
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
                for ta in (0, 1):
                    for tb in (0, 1):
                        cases.append(("sgeam ta %d tb %d" % (ta, tb), lambda ta=ta, tb=tb: case_level3(a, obj, tmp, "geam", {"ta": ta, "tb": tb}, rnd)))
                for right in (0, 1):
                    cases.append(("sdgmm right %d inc -2" % right, lambda r=right: case_level3(a, obj, tmp, "dgmm", {"right": r, "incx": -2}, rnd)))
                for upper in (0, 1):
                    for trans in (0, 1):
                        cases.append(("ssyrk upper %d trans %d" % (upper, trans),
                                      lambda u=upper, t=trans: case_level3(a, obj, tmp, "syrk", {"upper": u, "trans": t}, rnd)))
                    for right in (0, 1):
                        cases.append(("ssymm right %d upper %d" % (right, upper),
                                      lambda u=upper, r=right: case_level3(a, obj, tmp, "symm", {"upper": u, "right": r}, rnd)))
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
            elif "agx_csr_spmv_i32" in records:
                cases = []
                for I in ("i32", "i64"):
                    for base in (0, 1):
                        for what in ("csr", "csr_t", "coo", "coo_t", "spmm_col", "spmm_row"):
                            cases.append(("sparse %s %s base %d" % (what, I, base),
                                          lambda w=what, I=I, b=base: case_sparse(a, obj, tmp, w, I, b, rnd)))
            elif "agx_fft_radix2" in records:
                cases = []
                for t in ("f", "d") if "agx_fft_radix2_f64" in records else ("f",):
                    p = " f64" if t == "d" else ""
                    for inv in (0, 1):
                        cases += [
                            ("fft%s radix2 n=16 x3 inverse=%d" % (p, inv), lambda inv=inv, t=t: case_fft(a, obj, tmp, 16, 3, 1, 16, inv, True, rnd, t)),
                            ("fft%s radix2 n=64 x2 inverse=%d" % (p, inv), lambda inv=inv, t=t: case_fft(a, obj, tmp, 64, 2, 1, 64, inv, True, rnd, t)),
                            ("fft%s radix2 n=8 columns of 8x4 inverse=%d" % (p, inv), lambda inv=inv, t=t: case_fft(a, obj, tmp, 8, 4, 4, 1, inv, True, rnd, t)),
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
            elif "agx_epilogue_f32" in records:
                cases = []
                for t in ("f", "e"):
                    for has_bias, act, label in ((1, 0, "bias"), (0, 1, "relu"), (1, 1, "bias+relu"), (1, 2, "bias+gelu"), (0, 2, "gelu")):
                        cases.append(("epilogue %s %s 37x11" % ("f32" if t == "f" else "f16", label),
                                      lambda t=t, hb=has_bias, ac=act: case_epilogue(a, obj, tmp, 37, 11, hb, ac, rnd, t)))
            elif "agx_sgemm_tile64" in records:
                cases = [
                    ("tile64 128x64x40 a=1.5 b=0.5", lambda: case_tile(a, obj, tmp, 128, 64, 40, 1.5, 0.5, rnd)),
                    ("tile64 64x128x16 a=1 b=0 (C NaN)", lambda: case_tile(a, obj, tmp, 64, 128, 16, 1.0, 0.0, rnd)),
                    ("generic 37x29x13 NN b=0.25", lambda: case_generic(a, obj, tmp, 37, 29, 13, 0, 0, 1.0, 0.25, rnd)),
                    ("generic 20x33x7 TN", lambda: case_generic(a, obj, tmp, 20, 33, 7, 1, 0, -2.0, 1.0, rnd)),
                    ("generic 17x9x31 NT b=0", lambda: case_generic(a, obj, tmp, 17, 9, 31, 0, 1, 0.5, 0.0, rnd)),
                    ("generic 5x40x3 TT", lambda: case_generic(a, obj, tmp, 5, 40, 3, 1, 1, 1.0, -1.0, rnd)),
                ]
                dk = "agx_dgemm_generic"
                cases += [
                    ("d generic 13x11x9 NN b=0.25",
                     lambda: case_generic(a, obj, tmp, 13, 11, 9, 0, 0, 1.0, 0.25, rnd, dk, "d", "d")),
                    ("d generic 9x14x5 TN", lambda: case_generic(a, obj, tmp, 9, 14, 5, 1, 0, -2.0, 1.0, rnd, dk, "d", "d")),
                    ("d generic 7x6x11 NT b=0 (C NaN)",
                     lambda: case_generic(a, obj, tmp, 7, 6, 11, 0, 1, 0.5, 0.0, rnd, dk, "d", "d")),
                    ("d generic 4x10x3 TT", lambda: case_generic(a, obj, tmp, 4, 10, 3, 1, 1, 1.0, -1.0, rnd, dk, "d", "d")),
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
                    ]
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
