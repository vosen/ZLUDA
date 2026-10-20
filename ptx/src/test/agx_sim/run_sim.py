#!/usr/bin/env python3
"""Compiles the PTX here with zoc for AGX and runs it on the AGX Performance Simulator's agx_run, checking each
result against a reference model: prmt modes, szext and fns, the 3-input min/max, f32x2 arithmetic (default and
--strict-fp), fma.relu of the 16-bit types and two modules linked with zoc (cuLink).

  run_sim.py --zoc ZOC --agx-run AGX_RUN --objcopy LLVM_OBJCOPY --readelf LLVM_READELF [--chip agx-g14c]

Exits with 1 if any case is wrong.
"""

import argparse
import json
import os
import struct
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "../../../../agx_libs/kernels"))
import test_kernels as t  # noqa: E402


def f32(x):
    return struct.unpack("<f", struct.pack("<f", x))[0]


def compile_ptx(a, tmp, *names, strict=False):
    out = os.path.join(tmp, "out")
    cmd = [a.zoc, "--target", "agx", "--arch", a.chip, "--output-dir", out] + (["--strict-fp"] if strict else [])
    r = subprocess.run(cmd + [os.path.join(HERE, n) for n in names], capture_output=True, text=True)
    if r.returncode:
        raise RuntimeError("zoc: " + r.stderr[-400:])
    return open(os.path.join(out, os.path.basename(names[0]).replace(".ptx", ".o")), "rb").read()


def prmt(a, tmp):
    tables = {"f4e": [0x3210, 0x4321, 0x5432, 0x6543], "b4e": [0x5670, 0x6701, 0x7012, 0x0123],
              "rc8": [0, 0x1111, 0x2222, 0x3333], "ecl": [0x3210, 0x3211, 0x3222, 0x3333],
              "ecr": [0, 0x1110, 0x2210, 0x3210], "rc16": [0x1010, 0x3232, 0x1010, 0x3232]}
    (out,) = t.run(a, compile_ptx(a, tmp, "prmt_modes.ptx"), "prmt_modes", [("buf", bytes(96), True)], (1, 1), (1, 1), tmp)
    got = struct.unpack("<24I", out)
    want = [sum(((tab[i] >> (4 * j)) & 7) * 0x11 << (8 * j) for j in range(4)) for i in range(4) for tab in tables.values()]
    return [k for k in range(24) if got[k] != want[k]]


def szext_fns(a, tmp):
    sz, fn = json.load(open(os.path.join(HERE, "szext_fns.json")))

    def szext(m, ty, x, b):
        n = b % 32 if m == "wrap" else min(b, 32)
        if n == 0:
            return 0
        v = x & ((1 << n) - 1) if n < 32 else x
        if ty == "s32" and n < 32 and v >> (n - 1) & 1:
            v |= (0xffffffff << n) & 0xffffffff
        return v

    def fns(mask, base, off):
        base &= 31
        if off == 0:
            return base if mask >> base & 1 else 0xffffffff
        step, left, i = (1 if off > 0 else -1), abs(off), base
        while 0 <= i < 32:
            if mask >> i & 1:
                left -= 1
                if left == 0:
                    return i
            i += step
        return 0xffffffff

    n = len(sz) + len(fn)
    (out,) = t.run(a, compile_ptx(a, tmp, "szext_fns.ptx"), "k", [("buf", bytes(4 * n), True)], (1, 1), (1, 1), tmp)
    got = struct.unpack("<%dI" % n, out)
    want = [szext(*c) for c in sz] + [fns(*c) for c in fn]
    return [k for k in range(n) if got[k] != want[k]]


def min3(a, tmp):
    (out,) = t.run(a, compile_ptx(a, tmp, "min3.ptx"), "min3", [("buf", bytes(20), True)], (1, 1), (1, 1), tmp)
    got = struct.unpack("<5f", out)
    want = [-1.0, 3.0, 2.0, None, 3.0]
    return [k for k in range(5) if not (got[k] != got[k] if want[k] is None else got[k] == want[k])]


def f32x2(a, tmp, strict):
    from fractions import Fraction as F
    x, y, z = [f32(0.75), f32(1.0)], [f32(0.5), f32(-3.25)], [f32(1e-8), f32(-2.0 ** -30)]

    def directed(v, toward):
        # the f32 next to the exact value v toward zero (rz) or -inf (rm)
        r = f32(float(v))
        if (toward == "rz" and abs(F(r)) > abs(v)) or (toward == "rm" and F(r) > v):
            bits = struct.unpack("<I", struct.pack("<f", r))[0]
            # sign and magnitude: one step down in magnitude, except rm of a negative value (up in magnitude)
            bits += 1 if toward == "rm" and r < 0 else -1
            r = struct.unpack("<f", struct.pack("<I", bits))[0]
        return r

    want = []
    for op in ("add", "sub", "mul", "fma", "sat", "rz", "rm", "ftz"):
        for i in range(2):
            want.append({
                "add": lambda: f32(x[i] + y[i]), "sub": lambda: f32(x[i] - y[i]), "mul": lambda: f32(x[i] * y[i]),
                "fma": lambda: f32(float(F(x[i]) * F(y[i]) + F(z[i]))),
                "sat": lambda: min(max(f32(x[i] + y[i]), 0.0), 1.0),
                "rz": lambda: directed(F(x[i]) + F(z[i]), "rz"), "rm": lambda: directed(F(x[i]) + F(z[i]), "rm"),
                "ftz": lambda: f32(z[i] + z[i]),
            }[op]())
    inp = struct.pack("<6f", *(x + y + z))
    (out,) = t.run(a, compile_ptx(a, tmp, "f32x2.ptx", strict=strict), "f32x2", [("buf", inp), ("buf", bytes(64), True)],
                   (1, 1), (1, 1), tmp)
    got = struct.unpack("<16f", out)
    return [k for k in range(16) if got[k] != want[k]]


def fma_relu(a, tmp):
    (out,) = t.run(a, compile_ptx(a, tmp, "fma_relu.ptx"), "fma_relu", [("buf", bytes(20), True)], (1, 1), (1, 1), tmp)
    w = struct.unpack("<3I4H", out)
    bad = []
    if w[0] != 0x40000000:
        bad.append("f16x2")
    hi = w[1] >> 16
    if w[1] & 0xffff != 0 or (hi & 0x7c00) != 0x7c00 or (hi & 0x3ff) == 0:
        bad.append("f16x2 NaN")
    if w[2] != 0x40000000:
        bad.append("bf16x2")
    if w[3:] != (0x4000, 0x0000, 0x4000, 0xC900):
        bad.append("scalar %s" % [hex(v) for v in w[3:]])
    return bad


def link(a, tmp):
    obj = compile_ptx(a, tmp, "../spirv_run/extern_func.ptx", "link_lib.ptx")
    (out,) = t.run(a, obj, "extern_func", [("buf", struct.pack("<Q", 7)), ("buf", bytes(8), True)], (1, 1), (1, 1), tmp)
    (v,) = t.run(a, obj, "double_it", [("buf", struct.pack("<Q", 21), True)], (1, 1), (1, 1), tmp)
    return [] if struct.unpack("<Q", out)[0] == 8 and struct.unpack("<Q", v)[0] == 42 else ["link"]


def main():
    ap = argparse.ArgumentParser()
    for opt in ("--zoc", "--agx-run", "--objcopy", "--readelf"):
        ap.add_argument(opt, required=True)
    ap.add_argument("--chip", default="agx-g14c")
    a = ap.parse_args()
    failed = 0
    cases = [("prmt modes", prmt), ("szext and fns", szext_fns), ("3-input min/max", min3),
             ("f32x2", lambda a, tmp: f32x2(a, tmp, False)), ("f32x2 strict", lambda a, tmp: f32x2(a, tmp, True)),
             ("fma.relu", fma_relu), ("cuLink extern_func", link)]
    for label, f in cases:
        with tempfile.TemporaryDirectory() as tmp:
            try:
                bad = f(a, tmp)
                status = "ok" if not bad else "WRONG at %s" % bad
            except RuntimeError as e:
                bad, status = True, str(e)
        failed += bool(bad)
        print("%-9s %-22s %s" % (a.chip, label, status))
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
