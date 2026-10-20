#!/usr/bin/env python3
"""Run the GEMM kernels of a fat AGX object (agx-build-fatbin.py output) on the AGX Performance Simulator's agx_run
and compare them with a Python reference, for each chip in the object (or --chips).

  test_kernels.py --agx-run AGX_RUN --objcopy LLVM_OBJCOPY --readelf LLVM_READELF sgemm.agxfat [--chips agx-g14c]

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
    uniforms, dumps, bases = {}, [], [OUT, IN_A, IN_B]
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
            uniforms[u], uniforms[u + 1] = value & 0xFFFFFFFF, value >> 32
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
                  ("i64", sc[1]), ("f32", alpha), ("f32", beta)],
                 (-(-N // 16) * 16, -(-M // 8) * 8), (16, 8), tmp)
    want = reference(M, N, K, A, sa, B, sb, C, sc, alpha, beta)
    return close(unpack(got, c), want, [i * sc[0] + j * sc[1] for i in range(M) for j in range(N)],
                 1e-4 if c == "f" else 2e-3)


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
            records = kernel_records(obj, a, tmp)
            if "agx_epilogue_f32" in records:
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
