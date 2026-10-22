#!/usr/bin/env python3
"""Check the software double precision of the AGX backend bit by bit.

AGX has no double precision hardware; the backend computes double precision
operations with integer arithmetic (AGXSoftF64.cpp in the LLVM fork). This
script compiles one kernel per operation (and rounding direction), runs it
on the functional AGX simulator over generated inputs (zeros, denormals,
boundaries of the exponent range, ties, cancellations, infinities and NaNs)
and compares the results with an exact reference.

    python3 ptx/tools/agx_f64_check.py --llc $LLVM/bin/llc \\
        --objcopy $LLVM/bin/llvm-objcopy --agx-run .../agx_run
"""

import argparse
import math
import os
import random
import struct
import subprocess
import sys
import tempfile
from fractions import Fraction

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from agx_rounding_check import round_fmt  # noqa: E402

SIGN = 1 << 63
MAG = SIGN - 1
INF = 0x7ff << 52
QUIET = 1 << 51
CANON = MAG
MODES = {'rz': ('round.towardzero', 0), 'rn': ('round.tonearest', 1),
         'ru': ('round.upward', 2), 'rd': ('round.downward', 3)}


def is_nan(b):
    return (b & MAG) > INF


def is_inf(b):
    return (b & MAG) == INF


def is_zero(b):
    return (b & MAG) == 0


def sgn(b):
    return b >> 63


def exact(b):
    e = (b >> 52) & 0x7ff
    m = b & ((1 << 52) - 1)
    if e == 0:
        v = Fraction(m, 1 << 1074)
    else:
        v = Fraction(m | (1 << 52)) * Fraction(2) ** (e - 1075)
    return -v if sgn(b) else v


def rnd(v, mode, zero_sign=0):
    return round_fmt(v, mode, 53, 1023, 64, zero_sign)


def f2d(b):
    return struct.unpack('<d', struct.pack('<Q', b))[0]


def inf(s):
    return INF | (SIGN if s else 0)


def ref(op, mode, row):
    """The expected bits; None where any NaN is accepted is not needed: NaN
    results are canonical."""
    a = row[0]
    b = row[1] if len(row) > 1 else 0
    c = row[2] if len(row) > 2 else 0
    if op in ('add', 'sub'):
        if op == 'sub':
            b ^= SIGN
        if is_nan(a) or is_nan(b) or (is_inf(a) and is_inf(b) and a != b):
            return CANON
        if is_inf(a) or is_inf(b):
            return a if is_inf(a) else b
        v = exact(a) + exact(b)
        if v == 0:
            if is_zero(a) and is_zero(b):
                zs = sgn(a) | sgn(b) if mode == 'rd' else sgn(a) & sgn(b)
            else:
                zs = 1 if mode == 'rd' else 0
            return rnd(v, mode, zs)
        return rnd(v, mode)
    if op == 'mul':
        s = sgn(a) ^ sgn(b)
        if is_nan(a) or is_nan(b) or (is_inf(a) and is_zero(b)) or (is_inf(b) and is_zero(a)):
            return CANON
        if is_inf(a) or is_inf(b):
            return inf(s)
        return rnd(exact(a) * exact(b), mode, s)
    if op == 'fma':
        ps = sgn(a) ^ sgn(b)
        pinf = is_inf(a) or is_inf(b)
        if is_nan(a) or is_nan(b) or is_nan(c) or (is_inf(a) and is_zero(b)) or \
                (is_inf(b) and is_zero(a)) or (pinf and is_inf(c) and ps != sgn(c)):
            return CANON
        if pinf:
            return inf(ps)
        if is_inf(c):
            return c
        p = exact(a) * exact(b)
        v = p + exact(c)
        if v == 0:
            if p == 0 and is_zero(c):
                zs = ps | sgn(c) if mode == 'rd' else ps & sgn(c)
            else:
                zs = 1 if mode == 'rd' else 0
            return rnd(v, mode, zs)
        return rnd(v, mode)
    if op == 'div':
        s = sgn(a) ^ sgn(b)
        if is_nan(a) or is_nan(b) or (is_zero(a) and is_zero(b)) or (is_inf(a) and is_inf(b)):
            return CANON
        if is_inf(a) or is_zero(b):
            return inf(s)
        if is_zero(a) or is_inf(b):
            return SIGN if s else 0
        return rnd(exact(a) / exact(b), mode, s)
    if op == 'sqrt':
        if is_nan(a) or (sgn(a) and not is_zero(a)):
            return CANON
        if is_zero(a) or is_inf(a):
            return a
        v = exact(a)
        # Scale so that the integer root has about 200 bits.
        k = max(0, 200 - (v.numerator.bit_length() - v.denominator.bit_length()) // 2)
        n = v * (1 << (2 * k))
        s = math.isqrt(n.numerator // n.denominator)
        if Fraction(s * s) == n:
            return rnd(Fraction(s, 1 << k), mode)
        # The root lies strictly between s and s + 1 (scaled), far closer
        # than any rounding boundary: round a point inside.
        return rnd(Fraction(2 * s + 1, 1 << (k + 1)), mode)
    if op == 'fptrunc32':
        if is_nan(a):
            return 0x7fffffff
        if is_inf(a):
            return 0xff800000 if sgn(a) else 0x7f800000
        return round_fmt(exact(a), mode, 24, 127, 32, sgn(a))
    if op == 'fptrunc16':
        if is_nan(a):
            return 0x7fff
        if is_inf(a):
            return 0xfc00 if sgn(a) else 0x7c00
        return round_fmt(exact(a), mode, 11, 15, 16, sgn(a))
    if op == 'fpext':
        x = a & 0xffffffff
        if (x & 0x7fffffff) > 0x7f800000:
            return CANON
        if (x & 0x7fffffff) == 0x7f800000:
            return inf(x >> 31)
        e = (x >> 23) & 0xff
        m = x & 0x7fffff
        v = Fraction(m, 1 << 149) if e == 0 else Fraction(m | 1 << 23) * Fraction(2) ** (e - 150)
        return rnd(-v if x >> 31 else v, 'rn', x >> 31)
    if op in ('sitofp', 'uitofp'):
        v = a
        if op == 'sitofp' and v >= 1 << 63:
            v -= 1 << 64
        return rnd(Fraction(v), mode)
    if op in ('fptosi', 'fptoui'):
        if is_nan(a):
            return 0
        if is_inf(a):
            t = -(1 << 70) if sgn(a) else 1 << 70
        else:
            v = exact(a)
            t = math.floor(v) if v >= 0 else -math.floor(-v)
        lo, hi = (-(1 << 63), (1 << 63) - 1) if op == 'fptosi' else (0, (1 << 64) - 1)
        return min(max(t, lo), hi) & ((1 << 64) - 1)
    if op in ('floor', 'ceil', 'trunc', 'rint', 'round'):
        if is_nan(a):
            return CANON
        if is_inf(a) or (a & MAG) >= 1075 << 52:
            return a
        v = exact(a)
        if op == 'floor':
            t = math.floor(v)
        elif op == 'ceil':
            t = math.ceil(v)
        elif op == 'trunc':
            t = math.floor(v) if v >= 0 else -math.floor(-v)
        elif op == 'round':
            t = math.floor(abs(v) + Fraction(1, 2)) * (1 if v >= 0 else -1)
        else:
            fl = math.floor(v)
            r = v - fl
            if mode == 'rn':
                t = fl + (1 if r > Fraction(1, 2) or (r == Fraction(1, 2) and fl & 1) else 0)
            elif mode == 'rz':
                t = math.floor(v) if v >= 0 else -math.floor(-v)
            elif mode == 'ru':
                t = math.ceil(v)
            else:
                t = fl
        return rnd(Fraction(t), 'rn', sgn(a))
    if op in ('minnum', 'maxnum', 'minimum', 'maximum'):
        prop = op in ('minimum', 'maximum')
        if is_nan(a) or is_nan(b):
            if prop or (is_nan(a) and is_nan(b)):
                return CANON
            return b if is_nan(a) else a
        ka = (exact(a), -1 if sgn(a) else 0)
        kb = (exact(b), -1 if sgn(b) else 0)
        if op in ('minnum', 'minimum'):
            return a if ka <= kb else b
        return a if ka >= kb else b
    if op.startswith('fcmp_'):
        p = op[5:]
        uno = is_nan(a) or is_nan(b)
        if uno:
            return 1 if p[0] == 'u' else 0
        x, y = exact(a), exact(b)
        return int({'eq': x == y, 'ne': x != y, 'lt': x < y, 'le': x <= y,
                    'gt': x > y, 'ge': x >= y}[p[1:]])
    raise KeyError(op)


def rand_double(rng):
    k = rng.random()
    s = rng.getrandbits(1) << 63
    if k < 0.05:
        return s | rng.choice([0, INF, INF | 1, INF | (1 << 51), MAG])
    if k < 0.15:
        return s | rng.getrandbits(rng.choice([1, 8, 30, 52]))  # denormal
    if k < 0.25:
        return s | (rng.choice([1, 2, 3, 0x7fd, 0x7fe, 1022, 1023, 1024]) << 52) | rng.getrandbits(52)
    if k < 0.35:
        # few significant bits: exact results, ties
        e = rng.randrange(900, 1150)
        return s | (e << 52) | (rng.getrandbits(4) << 48)
    return s | (rng.randrange(1, 0x7ff) << 52) | rng.getrandbits(52)


def gen(rng, n, op):
    rows = []
    for _ in range(n):
        a, b, c = rand_double(rng), rand_double(rng), rand_double(rng)
        k = rng.random()
        if op in ('add', 'sub', 'fma') and k < 0.3 and not is_nan(a) and not is_inf(a):
            # cancellation and ties: b close to -a or to a shifted
            d = rng.choice([0, 1, 2, 52, 53, 54, 60])
            e = (a >> 52) & 0x7ff
            if e > d:
                b = (a & ~(0x7ff << 52)) | ((e - d) << 52)
                b ^= rng.getrandbits(1) << 63
                b ^= rng.getrandbits(2)
        if op == 'fma' and k < 0.5 and not is_inf(a) and not is_nan(a) and not is_nan(b) and not is_inf(b):
            p = exact(a) * exact(b)
            if p != 0 and abs(p) < 2 ** 1000 and abs(p) > 2 ** -1000:
                c = rnd(-p, 'rn') ^ rng.getrandbits(1)
        if op in ('sitofp', 'uitofp'):
            a = rng.getrandbits(rng.choice([8, 52, 53, 54, 63, 64]))
        if op in ('fptosi', 'fptoui') and k < 0.6:
            a = rng.getrandbits(1) << 63 | (rng.randrange(1000, 1090) << 52) | rng.getrandbits(52)
        if op == 'fpext':
            a = rng.getrandbits(32) if k < 0.8 else rng.choice([0, 1, 0x7fffff, 0x7f800000, 0x7fc00000, 0x80000001])
        if op in ('floor', 'ceil', 'trunc', 'rint', 'round') and k < 0.6:
            a = rng.getrandbits(1) << 63 | (rng.randrange(1015, 1080) << 52) | rng.getrandbits(52)
            if k < 0.2:
                a &= ~((1 << 40) - 1)  # halves and integers
        if op.startswith('fcmp_') and k < 0.3:
            b = a ^ rng.choice([0, 1, SIGN])
        rows.append([a, b, c])
    return rows


ARITH = ['add', 'sub', 'mul', 'div', 'fma', 'sqrt']
ROUNDED = ARITH + ['fptrunc32', 'fptrunc16', 'sitofp', 'uitofp', 'rint']
OTHER = ['fpext', 'fptosi', 'fptoui', 'floor', 'ceil', 'trunc', 'round', 'minnum',
         'maxnum', 'minimum', 'maximum'] + ['fcmp_' + p for p in
                                            ('oeq', 'one', 'olt', 'ole', 'ogt', 'oge',
                                             'ueq', 'une', 'ult', 'ule', 'ugt', 'uge')]


# the operations from double to double, with their number of operands
F64_ARITY = {'add': 2, 'sub': 2, 'mul': 2, 'div': 2, 'fma': 3, 'sqrt': 1, 'rint': 1, 'floor': 1, 'ceil': 1,
             'trunc': 1, 'round': 1, 'minnum': 2, 'maxnum': 2, 'minimum': 2, 'maximum': 2}


def body(op, mode, dynamic):
    """IR computing %r (stored as i64) from double %x, %y, %z."""
    exc = 'metadata !"fpexcept.ignore"'
    rm = 'round.dynamic' if dynamic else MODES[mode][0]
    r = f'metadata !"{rm}", {exc}'
    cons = mode != 'rn' or dynamic
    plain = {
        'add': 'fadd double %x, %y', 'sub': 'fsub double %x, %y',
        'mul': 'fmul double %x, %y', 'div': 'fdiv double %x, %y',
        'fma': 'call double @llvm.fma.f64(double %x, double %y, double %z)',
        'sqrt': 'call double @llvm.sqrt.f64(double %x)',
    }
    constrained = {
        'add': f'call double @llvm.experimental.constrained.fadd.f64(double %x, double %y, {r})',
        'sub': f'call double @llvm.experimental.constrained.fsub.f64(double %x, double %y, {r})',
        'mul': f'call double @llvm.experimental.constrained.fmul.f64(double %x, double %y, {r})',
        'div': f'call double @llvm.experimental.constrained.fdiv.f64(double %x, double %y, {r})',
        'fma': f'call double @llvm.experimental.constrained.fma.f64(double %x, double %y, double %z, {r})',
        'sqrt': f'call double @llvm.experimental.constrained.sqrt.f64(double %x, {r})',
    }
    if op in ARITH:
        e = constrained[op] if cons else plain[op]
        return f'%d = {e}\n  %r = bitcast double %d to i64'
    if op == 'fptrunc32':
        e = (f'call float @llvm.experimental.constrained.fptrunc.f32.f64(double %x, {r})'
             if cons else 'fptrunc double %x to float')
        return f'%f = {e}\n  %fi = bitcast float %f to i32\n  %r = zext i32 %fi to i64'
    if op == 'fptrunc16':
        e = (f'call half @llvm.experimental.constrained.fptrunc.f16.f64(double %x, {r})'
             if cons else 'fptrunc double %x to half')
        return f'%f = {e}\n  %fi = bitcast half %f to i16\n  %r = zext i16 %fi to i64'
    if op in ('sitofp', 'uitofp'):
        e = (f'call double @llvm.experimental.constrained.{op}.f64.i64(i64 %xi, {r})'
             if cons else f'{op} i64 %xi to double')
        return f'%d = {e}\n  %r = bitcast double %d to i64'
    if op == 'rint':
        e = (f'call double @llvm.experimental.constrained.rint.f64(double %x, {r})'
             if cons else 'call double @llvm.rint.f64(double %x)')
        return f'%d = {e}\n  %r = bitcast double %d to i64'
    if op == 'fpext':
        return ('%lo = trunc i64 %xi to i32\n  %f = bitcast i32 %lo to float\n'
                '  %d = fpext float %f to double\n  %r = bitcast double %d to i64')
    if op in ('fptosi', 'fptoui'):
        return f'%r = call i64 @llvm.{op}.sat.i64.f64(double %x)'
    if op in ('floor', 'ceil', 'trunc', 'round', 'minnum', 'maxnum', 'minimum', 'maximum'):
        args = 'double %x' if op in ('floor', 'ceil', 'trunc', 'round') else 'double %x, double %y'
        return f'%d = call double @llvm.{op}.f64({args})\n  %r = bitcast double %d to i64'
    if op.startswith('fcmp_'):
        return f'%cc = fcmp {op[5:]} double %x, %y\n  %r = zext i1 %cc to i64'
    raise KeyError(op)


def kernel(op, mode, dynamic):
    code = MODES[mode][1]
    # bits 0-1: f32 rounding (fptrunc to float), bits 2-3: f16/f64.
    setmode = f'call void @llvm.agx.set.fpmode(i32 {code | (code << 2)})' if dynamic else ''
    return f'''declare i32 @llvm.agx.thread.position.in.grid.x()
declare void @llvm.agx.set.fpmode(i32)
define agx_kernel void @k(ptr addrspace(1) %a, ptr addrspace(1) %b, ptr addrspace(1) %c, ptr addrspace(1) %o) #0 {{
  %t = call i32 @llvm.agx.thread.position.in.grid.x()
  {setmode}
  %pa = getelementptr i64, ptr addrspace(1) %a, i32 %t
  %pb = getelementptr i64, ptr addrspace(1) %b, i32 %t
  %pc = getelementptr i64, ptr addrspace(1) %c, i32 %t
  %xi = load i64, ptr addrspace(1) %pa
  %yi = load i64, ptr addrspace(1) %pb
  %zi = load i64, ptr addrspace(1) %pc
  %x = bitcast i64 %xi to double
  %y = bitcast i64 %yi to double
  %z = bitcast i64 %zi to double
  {body(op, mode, dynamic)}
  %po = getelementptr i64, ptr addrspace(1) %o, i32 %t
  store i64 %r, ptr addrspace(1) %po
  ret void
}}
attributes #0 = {{ strictfp }}
'''


def run(op, mode, dynamic, n, args, tmp):
    src = os.path.join(tmp, 'k.ll')
    open(src, 'w').write(kernel(op, mode, dynamic))
    obj = os.path.join(tmp, 'k.o')
    r = subprocess.run([args.llc, '-mtriple=agx', '-mcpu=' + args.cpu, '-mattr=' + args.features,
                        '-filetype=obj', src, '-o', obj], capture_output=True, text=True)
    if r.returncode:
        lines = [l for l in r.stderr.splitlines() if 'error' in l.lower() or 'Assertion' in l]
        return 'compile error: ' + (lines[0] if lines else r.stderr[-300:])
    code = os.path.join(tmp, 'k.bin')
    subprocess.run([args.objcopy, '-O', 'binary', '--only-section=.text', obj, code], check=True)
    rng = random.Random(args.seed + hash(op) % 1000)
    rows = gen(rng, n, op)
    base = [0x100000, 0x200000, 0x300000, 0x400000]
    cmd = [args.agx_run, code, '--grid', str(n), '--wg', '64']
    for i in range(3):
        f = os.path.join(tmp, 'in%d.bin' % i)
        open(f, 'wb').write(b''.join(struct.pack('<Q', row[i]) for row in rows))
        cmd += ['--map', '%#x=%s' % (base[i], f)]
    out = os.path.join(tmp, 'out.bin')
    cmd += ['--alloc', '%#x=%d' % (base[3], 8 * n), '--dump', '%#x=%d:%s' % (base[3], 8 * n, out)]
    for i in range(4):
        cmd += ['--uniform', '%d=%d' % (2 * i, base[i]), '--uniform', '%d=0' % (2 * i + 1)]
    cmd += ['--uniform', '8=%d' % (n // 64), '--uniform', '9=1', '--uniform', '10=1']
    r = subprocess.run(cmd, capture_output=True, text=True, timeout=args.timeout)
    if r.returncode:
        return 'simulator error: ' + (r.stdout + r.stderr)[-300:]
    got = struct.unpack('<%dQ' % n, open(out, 'rb').read())
    canon = '+canonical-nan' in args.features
    bad = []
    for row, g in zip(rows, got):
        w = ref(op, mode, row)
        # double precision operations keep NaN payloads (PTX ISA 9.7.3): the first NaN operand, quieted
        if canon and w == CANON and op in F64_ARITY:
            nans = [x for x in row[:F64_ARITY[op]] if is_nan(x)]
            if nans:
                w = nans[0] | QUIET
        if not canon and w == CANON and op not in ('fptrunc32', 'fptrunc16') and is_nan(g):
            continue
        if not canon and op == 'fptrunc32' and w == 0x7fffffff and (g & 0x7fffffff) > 0x7f800000:
            continue
        if not canon and op == 'fptrunc16' and w == 0x7fff and (g & 0x7fff) > 0x7c00:
            continue
        if w != g:
            bad.append((row, w, g))
    return bad


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('ops', nargs='*')
    ap.add_argument('-n', type=int, default=2048)
    ap.add_argument('--features', default='+canonical-nan')
    ap.add_argument('--cpu', default='agx-g13g')
    ap.add_argument('--seed', type=int, default=1)
    ap.add_argument('--timeout', type=int, default=600)
    ap.add_argument('--llc', default='llc')
    ap.add_argument('--objcopy', default='llvm-objcopy')
    ap.add_argument('--agx-run', dest='agx_run', default='agx_run')
    args = ap.parse_args()
    ops = args.ops or ROUNDED + OTHER
    ok = True
    with tempfile.TemporaryDirectory() as tmp:
        for op in ops:
            configs = [('rn', False)]
            if op in ROUNDED:
                configs += [(m, d) for m in ('rz', 'ru', 'rd') for d in (False, True)]
            for mode, dynamic in configs:
                res = run(op, mode, dynamic, args.n, args, tmp)
                name = '%s.%s%s' % (op, mode, ' (dynamic)' if dynamic else '')
                if isinstance(res, str):
                    print('%-22s %s' % (name, res))
                    ok = False
                    continue
                print('%-22s %d/%d bit-exact' % (name, args.n - len(res), args.n))
                for row, w, g in res[:4]:
                    print('    in=%s expected %016x got %016x' %
                          (' '.join('%016x' % x for x in row), w, g))
                ok &= not res
    return 0 if ok else 1


if __name__ == '__main__':
    sys.exit(main())
