#!/usr/bin/env python3
"""NVRTC's math on the GPU: compiles a CUDA program with the prelude as zluda_rtc_agx does, then the PTX with zoc,
runs it in AGX-Performance-Simulator's agx_run and reports each function's worst error in ulp against Python's
math module (agx_libs/kernels/test_kernels.py runs the kernels)."""

import argparse
import math
import os
import random
import struct
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "../../agx_libs/kernels"))
import test_kernels as tk  # noqa: E402

SOURCE = r"""
extern "C" __global__ void math32(const float *x, float *out, int n) {
  int i = blockIdx.x * blockDim.x + threadIdx.x;
  if (i < n) {
    float v = x[i];
    float *o = out + 12 * i;
    o[0] = sinf(v); o[1] = cosf(v); o[2] = expf(v); o[3] = logf(fabsf(v) + 0.5f);
    o[4] = powf(fabsf(v) + 0.1f, 1.7f); o[5] = atanf(v); o[6] = tanhf(v); o[7] = erff(v);
    o[8] = sqrtf(fabsf(v)); o[9] = __sinf(v); o[10] = exp2f(v); o[11] = log1pf(fabsf(v));
  }
}
extern "C" __global__ void math64(const double *x, double *out, int n) {
  int i = blockIdx.x * blockDim.x + threadIdx.x;
  if (i < n) {
    double v = x[i];
    double *o = out + 8 * i;
    o[0] = sin(v); o[1] = cos(v); o[2] = exp(v); o[3] = log(fabs(v) + 0.5);
    o[4] = pow(fabs(v) + 0.1, 1.7); o[5] = atan(v); o[6] = tanh(v); o[7] = erf(v);
  }
}
"""


def f32(v):
    return struct.unpack("<f", struct.pack("<f", v))[0]


def ulp_err(got, want, single):
    if single:
        w = f32(want)
        bits = struct.unpack("<I", struct.pack("<f", w))[0]
        u = abs(struct.unpack("<f", struct.pack("<I", bits + 1))[0] - w)
    else:
        u = math.ulp(want)
    return abs(got - want) / u


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--zoc", required=True)
    ap.add_argument("--agx-run", required=True)
    ap.add_argument("--llvm-bin", required=True, help="the bin directory of the LLVM AGX build (llvm-objcopy, llvm-readelf)")
    ap.add_argument("--clang", default="clang")
    ap.add_argument("--chip", default="agx-g14g")
    a = ap.parse_args()
    a.objcopy, a.readelf = os.path.join(a.llvm_bin, "llvm-objcopy"), os.path.join(a.llvm_bin, "llvm-readelf")
    include = os.path.join(HERE, "../src/include")
    limits32 = [2, 2, 2, 2, 2, 3, 3, 4, 2, 1e9, 2, 3]
    limits64 = [2, 2, 2, 2, 2, 3, 3, 100]
    failed = 0
    with tempfile.TemporaryDirectory() as tmp:
        src = os.path.join(tmp, "m.cu")
        open(src, "w").write(SOURCE)
        ptx = os.path.join(tmp, "m.ptx")
        subprocess.run([a.clang, "-x", "cuda", "--cuda-device-only", "-nocudainc", "-nocudalib", "--cuda-gpu-arch=sm_80",
                        "-Xclang", "-target-feature", "-Xclang", "+ptx70", "-O3", "-std=c++17", "-I", include,
                        "-include", "agx_nvrtc_prelude.h", "-S", "-o", ptx, src], check=True)
        subprocess.run([a.zoc, "--arch=" + a.chip, "--output-dir=" + tmp, ptx], check=True, capture_output=True)
        obj = open(os.path.join(tmp, "m.o"), "rb").read()
        rnd = random.Random(3)
        n = 64
        xs = [rnd.uniform(-4, 4) for _ in range(n)]
        xf = [f32(v) for v in xs]
        (out,) = tk.run(a, obj, "math32", [("buf", tk.pack(xf, "f")), ("buf", tk.pack([0.0] * 12 * n, "f"), True), ("i32", n)],
                        (64, 1), (64, 1), tmp)
        got = tk.unpack(out, "f")
        refs = [("sinf", math.sin), ("cosf", math.cos), ("expf", math.exp), ("logf", lambda v: math.log(f32(abs(v) + f32(0.5)))),
                ("powf", lambda v: f32(abs(v) + f32(0.1)) ** f32(1.7)), ("atanf", math.atan), ("tanhf", math.tanh),
                ("erff", math.erf), ("sqrtf", lambda v: math.sqrt(abs(v))), ("__sinf", math.sin), ("exp2f", lambda v: 2 ** v),
                ("log1pf", lambda v: math.log1p(abs(v)))]
        for k, ((name, ref), limit) in enumerate(zip(refs, limits32)):
            worst = max(ulp_err(got[12 * i + k], ref(xf[i]), True) for i in range(n))
            bad = worst > limit
            failed += bad
            print("%-8s %10.2f ulp%s" % (name, worst, "  FAIL" if bad else ""))
        (out,) = tk.run(a, obj, "math64", [("buf", tk.pack(xs, "d")), ("buf", tk.pack([0.0] * 8 * n, "d"), True), ("i32", n)],
                        (64, 1), (64, 1), tmp)
        got = tk.unpack(out, "d")
        refs = [("sin", math.sin), ("cos", math.cos), ("exp", math.exp), ("log", lambda v: math.log(abs(v) + 0.5)),
                ("pow", lambda v: (abs(v) + 0.1) ** 1.7), ("atan", math.atan), ("tanh", math.tanh), ("erf", math.erf)]
        for k, ((name, ref), limit) in enumerate(zip(refs, limits64)):
            worst = max(ulp_err(got[8 * i + k], ref(xs[i]), False) for i in range(n))
            bad = worst > limit
            failed += bad
            print("%-8s %10.2f ulp%s" % (name, worst, "  FAIL" if bad else ""))
    print("%d failures" % failed)
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
