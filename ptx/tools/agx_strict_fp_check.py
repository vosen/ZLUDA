#!/usr/bin/env python3
"""Check the strict floating point mode of the AGX backend bit by bit.

Compiles one kernel per operation with llc (-mattr=+ieee-fp32,+canonical-nan
by default), runs it on the functional AGX simulator (`agx_run` of
https://github.com/Jianwen-Ding/AGX-Performance-Simulator) over generated
inputs (denormals, boundaries, cancellations, rounding ties, quotients and
roots next to rounding midpoints, specials and random values), and compares
the results with an exact IEEE 754 binary32 reference computed with rational
arithmetic, with NaN results expected to be PTX's canonical NaN.

    python3 ptx/tools/agx_strict_fp_check.py --llc $LLVM/bin/llc \
        --objcopy $LLVM/bin/llvm-objcopy --agx-run .../agx_run -n 8192
"""

import os
import random
import struct
import subprocess
import sys
import tempfile
from fractions import Fraction

LLC = 'llc'
OBJCOPY = 'llvm-objcopy'
AGX_RUN = 'agx_run'
CANON = 0x7fffffff

# ------------------------------------------------------------ reference


def f2b(x):
    return struct.unpack('<I', struct.pack('<f', x))[0]


def b2f(b):
    return struct.unpack('<f', struct.pack('<I', b & 0xffffffff))[0]


def is_nan(b):
    return (b & 0x7f800000) == 0x7f800000 and (b & 0x7fffff) != 0


def is_inf(b):
    return (b & 0x7fffffff) == 0x7f800000


def sign(b):
    return b >> 31


def exact(b):
    """Finite value as a Fraction."""
    e = (b >> 23) & 0xff
    m = b & 0x7fffff
    v = Fraction(m, 1 << 23) * Fraction(2) ** -126 if e == 0 else \
        Fraction(m | 0x800000, 1 << 23) * Fraction(2) ** (e - 127)
    return -v if sign(b) else v


def round_f32(v, zero_sign=0):
    """Round a Fraction to nearest even binary32 bits."""
    if v == 0:
        return zero_sign << 31
    s = 1 if v < 0 else 0
    a = -v if s else v
    # Find exponent so that 2^23 <= a / 2^(e-23) < 2^24, clamped to denormals.
    e = a.numerator.bit_length() - a.denominator.bit_length()
    while Fraction(2) ** e > a:
        e -= 1
    while Fraction(2) ** (e + 1) <= a:
        e += 1
    e = max(e, -126)
    q = a / Fraction(2) ** (e - 23)
    n = q.numerator // q.denominator
    r = q - n
    if r > Fraction(1, 2) or (r == Fraction(1, 2) and n & 1):
        n += 1
    if n == 1 << 24:
        n >>= 1
        e += 1
    if e > 127:
        return (s << 31) | 0x7f800000
    if n < 1 << 23:  # denormal
        return (s << 31) | n
    return (s << 31) | ((e + 127) << 23) | (n - (1 << 23))


def ref_add(a, b):
    if is_nan(a) or is_nan(b):
        return CANON
    if is_inf(a) or is_inf(b):
        if is_inf(a) and is_inf(b) and sign(a) != sign(b):
            return CANON
        return a if is_inf(a) else b
    v = exact(a) + exact(b)
    return round_f32(v, sign(a) & sign(b))


def ref_sub(a, b):
    return ref_add(a, b ^ 0x80000000)


def ref_mul(a, b):
    if is_nan(a) or is_nan(b):
        return CANON
    s = sign(a) ^ sign(b)
    za, zb = (a & 0x7fffffff) == 0, (b & 0x7fffffff) == 0
    if is_inf(a) or is_inf(b):
        if za or zb:
            return CANON
        return (s << 31) | 0x7f800000
    return round_f32(exact(a) * exact(b), s)


def ref_fma(a, b, c):
    if is_nan(a) or is_nan(b) or is_nan(c):
        return CANON
    s = sign(a) ^ sign(b)
    za, zb = (a & 0x7fffffff) == 0, (b & 0x7fffffff) == 0
    if is_inf(a) or is_inf(b):
        if za or zb:
            return CANON
        if is_inf(c) and sign(c) != s:
            return CANON
        return (s << 31) | 0x7f800000
    if is_inf(c):
        return c
    p = exact(a) * exact(b)
    v = p + exact(c)
    pzero = p == 0
    zs = (s & sign(c)) if pzero and (c & 0x7fffffff) == 0 else 0
    return round_f32(v, zs)


def ref_div(a, b):
    if is_nan(a) or is_nan(b):
        return CANON
    s = sign(a) ^ sign(b)
    za, zb = (a & 0x7fffffff) == 0, (b & 0x7fffffff) == 0
    if (za and zb) or (is_inf(a) and is_inf(b)):
        return CANON
    if is_inf(a) or zb:
        return (s << 31) | 0x7f800000
    if za or is_inf(b):
        return s << 31
    return round_f32(exact(a) / exact(b), s)


def isqrt_frac(v):
    """Round sqrt of a positive Fraction to binary32 (exact rounding)."""
    # Find e with 2^23 <= sqrt(v)/2^(e-23) < 2^24; compare squares exactly.
    e = (v.numerator.bit_length() - v.denominator.bit_length()) // 2
    while Fraction(2) ** (2 * e) > v:
        e -= 1
    while Fraction(2) ** (2 * e + 2) <= v:
        e += 1
    scale = Fraction(2) ** (e - 23)
    t = v / (scale * scale)  # sqrt(t) in [2^23, 2^24)
    n = int(t.numerator // t.denominator)
    lo, hi = 1 << 23, 1 << 24
    while lo + 1 < hi:  # floor(sqrt(t))
        mid = (lo + hi) // 2
        if mid * mid <= t:
            lo = mid
        else:
            hi = mid
    n = lo
    # round: compare t with (n + 1/2)^2
    half = (Fraction(2 * n + 1, 2)) ** 2
    if t > half or (t == half and n & 1):
        n += 1
    if n == 1 << 24:
        n >>= 1
        e += 1
    return ((e + 127) << 23) | (n - (1 << 23))


def ref_sqrt(a, b=None):
    if is_nan(a):
        return CANON
    if (a & 0x7fffffff) == 0:
        return a
    if sign(a):
        return CANON
    if is_inf(a):
        return a
    return isqrt_frac(exact(a))


def cmpv(a, b):
    """-1, 0, 1 or None (unordered)."""
    if is_nan(a) or is_nan(b):
        return None
    x = float('inf') if a == 0x7f800000 else float('-inf') if a == 0xff800000 else exact(a)
    y = float('inf') if b == 0x7f800000 else float('-inf') if b == 0xff800000 else exact(b)
    return (x > y) - (x < y)


def ref_cmp(pred):
    def f(a, b):
        c = cmpv(a, b)
        u = pred.startswith('u')
        if c is None:
            return int(u)
        p = pred[1:]
        return int({'eq': c == 0, 'ne': c != 0, 'lt': c < 0, 'le': c <= 0,
                    'gt': c > 0, 'ge': c >= 0}[p])
    return f


def ref_minmax(is_min, propagate):
    def f(a, b):
        na, nb = is_nan(a), is_nan(b)
        if na and nb:
            return CANON
        if na or nb:
            return CANON if propagate else (b if na else a)
        c = cmpv(a, b)
        if c == 0:
            return (a | b) if is_min else (a & b)
        if is_min:
            return a if c < 0 else b
        return a if c > 0 else b
    return f


def ref_round(kind):
    import math

    def f(a, b=None):
        if is_nan(a):
            return CANON
        if is_inf(a) or (a & 0x7fffffff) == 0:
            return a
        v = exact(a)
        if kind == 'floor':
            r = math.floor(v)
        elif kind == 'ceil':
            r = math.ceil(v)
        elif kind == 'trunc':
            r = int(v)
        else:  # rint: nearest even
            fl = math.floor(v)
            d = v - fl
            r = fl + 1 if d > Fraction(1, 2) or (d == Fraction(1, 2) and fl % 2) else fl
        return round_f32(Fraction(r), sign(a))
    return f


# ------------------------------------------------------------ inputs

SPECIAL = [0, 0x80000000, 1, 0x80000001, 2, 3, 0x7fffff, 0x807fffff, 0x400000,
           0x00800000, 0x80800000, 0x00800001, 0x00ffffff, 0x01000000, 0x3f800000,
           0xbf800000, 0x3fc00000, 0x7f7fffff, 0xff7fffff, 0x7f800000, 0xff800000,
           0x7fc00000, 0x7f800001, 0xffc12345, 0x0c800000, 0x0d000000, 0x26800000,
           0x1a800000, 0x34000000, 0x3f800001, 0x00000005, 0x00123456, 0x80654321]


def rand_bits(rng):
    k = rng.random()
    if k < 0.25:
        return rng.getrandbits(32)
    if k < 0.55:  # zero exponent or tiny exponents
        return (rng.getrandbits(1) << 31) | (rng.randint(0, 30) << 23) | rng.getrandbits(23)
    if k < 0.75:  # around 2^-64 (products into the denormal range)
        return (rng.getrandbits(1) << 31) | (rng.randint(40, 90) << 23) | rng.getrandbits(23)
    if k < 0.85:  # low-precision values: exact ties
        return (rng.getrandbits(1) << 31) | (rng.randint(0, 254) << 23) | (rng.getrandbits(4) << 19)
    return rng.choice(SPECIAL)


def gen(rng, n, arity, op):
    rows = []
    for x in SPECIAL:
        for y in SPECIAL[:12]:
            rows.append([x, y, rng.choice(SPECIAL)][:arity])
    while len(rows) < n:
        r = [rand_bits(rng) for _ in range(arity)]
        if op in ('add', 'sub', 'fma') and rng.random() < 0.3:
            # cancellation: b close to -a (or c close to -a*b)
            if op == 'fma':
                p = b2f(r[0]) * b2f(r[1])
                if p == p and abs(p) < 3.4e38:
                    r[2] = (f2b(-p) + rng.randint(-3, 3)) & 0xffffffff
            else:
                r[1] = ((r[0] ^ (0 if op == 'sub' else 0x80000000)) + rng.randint(-5, 5)) & 0xffffffff
        if op == 'div' and rng.random() < 0.4:
            # quotient next to a rounding midpoint: a = RN(mid * b)
            k = rng.getrandbits(23)
            mid = Fraction(2 * ((1 << 23) | k) + 1, 1 << 24)
            bb = (rng.randint(1, 254) << 23) | rng.getrandbits(23)
            v = mid * exact(bb) * Fraction(2) ** rng.randint(-20, 20)
            r = [round_f32(v), bb, 0][:arity]
        if op == 'sqrt' and rng.random() < 0.4:
            k = rng.getrandbits(23)
            mid = Fraction(2 * ((1 << 23) | k) + 1, 1 << 24)
            v = mid * mid * Fraction(4) ** rng.randint(-60, 60)
            r = [round_f32(v), 0, 0][:arity]
        rows.append(r)
    return rows[:n]


# ------------------------------------------------------------ kernels

def kernel(op):
    """(IR body computing %r from %x, %y, %z, result type)."""
    if op in ('add', 'sub', 'mul', 'div'):
        ins = {'add': 'fadd', 'sub': 'fsub', 'mul': 'fmul', 'div': 'fdiv'}[op]
        return f'%r = {ins} float %x, %y', 'float', 2
    if op == 'fma':
        return '%r = call float @llvm.fma.f32(float %x, float %y, float %z)', 'float', 3
    if op == 'sqrt':
        return '%r = call float @llvm.sqrt.f32(float %x)', 'float', 1
    if op.startswith('cmp_'):
        p = op[4:]
        return f'%cc = fcmp {p} float %x, %y\n  %r = zext i1 %cc to i32', 'i32', 2
    if op in ('minimumnum', 'maximumnum', 'minimum', 'maximum', 'minnum', 'maxnum'):
        return f'%r = call float @llvm.{op}.f32(float %x, float %y)', 'float', 2
    if op in ('floor', 'ceil', 'trunc', 'rint'):
        return f'%r = call float @llvm.{op}.f32(float %x)', 'float', 1
    raise KeyError(op)


def reference(op):
    if op.startswith('cmp_'):
        return ref_cmp(op[4:])
    if op in ('minimumnum', 'minnum'):
        return ref_minmax(True, False)
    if op in ('maximumnum', 'maxnum'):
        return ref_minmax(False, False)
    if op == 'minimum':
        return ref_minmax(True, True)
    if op == 'maximum':
        return ref_minmax(False, True)
    if op in ('floor', 'ceil', 'trunc', 'rint'):
        return ref_round(op)
    return {'add': ref_add, 'sub': ref_sub, 'mul': ref_mul, 'fma': ref_fma,
            'div': ref_div, 'sqrt': ref_sqrt}[op]


def run(op, n, features, cpu, seed, tmp):
    body, rty, arity = kernel(op)
    ir = f'''declare i32 @llvm.agx.thread.position.in.grid.x()
declare float @llvm.fma.f32(float, float, float)
declare float @llvm.sqrt.f32(float)
declare float @llvm.{op}.f32(float, float)
declare float @llvm.floor.f32(float)
declare float @llvm.ceil.f32(float)
declare float @llvm.trunc.f32(float)
declare float @llvm.rint.f32(float)
define agx_kernel void @k(ptr addrspace(1) %a, ptr addrspace(1) %b, ptr addrspace(1) %c, ptr addrspace(1) %o) {{
  %t = call i32 @llvm.agx.thread.position.in.grid.x()
  %pa = getelementptr float, ptr addrspace(1) %a, i32 %t
  %pb = getelementptr float, ptr addrspace(1) %b, i32 %t
  %pc = getelementptr float, ptr addrspace(1) %c, i32 %t
  %x = load float, ptr addrspace(1) %pa
  %y = load float, ptr addrspace(1) %pb
  %z = load float, ptr addrspace(1) %pc
  {body}
  %po = getelementptr {rty}, ptr addrspace(1) %o, i32 %t
  store {rty} %r, ptr addrspace(1) %po
  ret void
}}
'''
    if op in ('floor', 'ceil', 'trunc', 'rint', 'sqrt', 'fma'):
        ir = ir.replace(f'declare float @llvm.{op}.f32(float, float)\n', '')
    src = os.path.join(tmp, 'k.ll')
    open(src, 'w').write(ir)
    obj = os.path.join(tmp, 'k.o')
    r = subprocess.run([LLC, '-mtriple=agx', '-mcpu=' + cpu, '-mattr=' + features,
                        '-filetype=obj', src, '-o', obj], capture_output=True, text=True)
    if r.returncode:
        return 'compile error: ' + r.stderr[-400:]
    code = os.path.join(tmp, 'k.bin')
    subprocess.run([OBJCOPY, '-O', 'binary', '--only-section=.text', obj, code], check=True)
    rng = random.Random(seed)
    rows = gen(rng, n, arity, op)
    bufs = []
    for i in range(3):
        bufs.append(b''.join(struct.pack('<I', row[i] if i < len(row) else 0) for row in rows))
    base = [0x100000, 0x200000, 0x300000, 0x400000]
    cmd = [AGX_RUN, code, '--grid', str(n), '--wg', '64']
    for i in range(3):
        f = os.path.join(tmp, 'in%d.bin' % i)
        open(f, 'wb').write(bufs[i])
        cmd += ['--map', '%#x=%s' % (base[i], f)]
    out = os.path.join(tmp, 'out.bin')
    cmd += ['--alloc', '%#x=%d' % (base[3], 4 * n), '--dump', '%#x=%d:%s' % (base[3], 4 * n, out)]
    for i in range(4):
        cmd += ['--uniform', '%d=%d' % (2 * i, base[i]), '--uniform', '%d=0' % (2 * i + 1)]
    cmd += ['--uniform', '8=%d' % (n // 64), '--uniform', '9=1', '--uniform', '10=1']
    r = subprocess.run(cmd, capture_output=True, text=True)
    if r.returncode:
        return 'simulator error: ' + r.stdout[-300:]
    got = struct.unpack('<%dI' % n, open(out, 'rb').read())
    ref = reference(op)
    bad = []
    for row, g in zip(rows, got):
        w = ref(*row)
        if 'canonical-nan' not in features and rty == 'float' and is_nan(w) and is_nan(g):
            continue
        if w != g:
            bad.append((row, w, g))
    return bad


def main():
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument('ops', nargs='*')
    ap.add_argument('-n', type=int, default=4096)
    ap.add_argument('--features', default='+ieee-fp32,+canonical-nan')
    ap.add_argument('--cpu', default='agx-g13g')
    ap.add_argument('--seed', type=int, default=1)
    ap.add_argument('--llc', default=LLC)
    ap.add_argument('--objcopy', default=OBJCOPY)
    ap.add_argument('--agx-run', dest='agx_run', default=AGX_RUN)
    args = ap.parse_args()
    globals().update(LLC=args.llc, OBJCOPY=args.objcopy, AGX_RUN=args.agx_run)
    ops = args.ops or ['add', 'sub', 'mul', 'fma', 'div', 'sqrt', 'cmp_olt', 'cmp_oeq',
                       'cmp_ule', 'cmp_une', 'cmp_ogt', 'cmp_uge', 'minimumnum', 'maximumnum',
                       'minimum', 'maximum', 'floor', 'ceil', 'trunc', 'rint']
    ok = True
    with tempfile.TemporaryDirectory() as tmp:
        for op in ops:
            res = run(op, args.n, args.features, args.cpu, args.seed, tmp)
            if isinstance(res, str):
                print('%-12s %s' % (op, res))
                ok = False
                continue
            print('%-12s %d/%d bit-exact' % (op, args.n - len(res), args.n))
            for row, w, g in res[:6]:
                print('    in=%s expected %08x got %08x' % (' '.join('%08x' % x for x in row), w, g))
            ok &= not res
    return 0 if ok else 1


if __name__ == '__main__':
    sys.exit(main())
