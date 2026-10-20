#!/usr/bin/env python3
"""Check directed rounding on AGX bit by bit.

Compiles kernels that use constrained floating point intrinsics with static
rounding (round.towardzero, round.upward, round.downward) and with dynamic
rounding set through llvm.agx.set.fpmode, runs them on the functional AGX
simulator and compares with an exact reference (see agx_strict_fp_check.py,
whose input generation and reference arithmetic this reuses).

    python3 ptx/tools/agx_rounding_check.py --llc $LLVM/bin/llc \\
        --objcopy $LLVM/bin/llvm-objcopy --agx-run .../agx_run
"""

import argparse
import os
import random
import struct
import subprocess
import sys
import tempfile
from fractions import Fraction

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import agx_strict_fp_check as sc  # noqa: E402

MODES = {'rz': ('round.towardzero', 0), 'rn': ('round.tonearest', 1),
         'ru': ('round.upward', 2), 'rd': ('round.downward', 3)}


def round_fmt(v, mode, prec=24, bias=127, width=32, zero_sign=0):
    """Round a Fraction in a direction: bits of the binary format."""
    sbit = 1 << (width - 1)
    infb = ((2 * bias + 1) << (prec - 1))
    if v == 0:
        return sbit if zero_sign else 0
    s = 1 if v < 0 else 0
    a = -v if s else v
    e = a.numerator.bit_length() - a.denominator.bit_length()
    while Fraction(2) ** e > a:
        e -= 1
    while Fraction(2) ** (e + 1) <= a:
        e += 1
    e = max(e, 1 - bias)
    q = a / Fraction(2) ** (e - (prec - 1))
    n = q.numerator // q.denominator
    r = q - n
    if mode == 'rn':
        up = r > Fraction(1, 2) or (r == Fraction(1, 2) and n & 1)
    elif mode == 'rz':
        up = False
    elif mode == 'ru':
        up = r != 0 and not s
    else:
        up = r != 0 and s
    if up:
        n += 1
    if n == 1 << prec:
        n >>= 1
        e += 1
    sign = sbit if s else 0
    if e > bias:
        over_max = mode == 'rz' or (mode == 'ru' and s) or (mode == 'rd' and not s)
        return sign | (infb - 1 if over_max else infb)
    if n < 1 << (prec - 1):
        return sign | n
    return sign | ((e + bias) << (prec - 1)) | (n - (1 << (prec - 1)))


def specials(op, row):
    """The result for NaN / infinity operands (rounding does not matter), or
    None."""
    if op in ('add', 'sub', 'mul', 'fma', 'div', 'sqrt'):
        f = {'add': sc.ref_add, 'sub': sc.ref_sub, 'mul': sc.ref_mul,
             'fma': sc.ref_fma, 'div': sc.ref_div, 'sqrt': sc.ref_sqrt}[op]
        if any(sc.is_nan(x) or sc.is_inf(x) for x in row) or \
                (op == 'div' and (row[1] & 0x7fffffff) == 0) or \
                (op == 'sqrt' and sc.sign(row[0]) and (row[0] & 0x7fffffff)):
            return f(*row)
    return None


def reference(op, mode, row):
    s = specials(op, row)
    if s is not None:
        return s
    if op == 'add':
        a, b = row
        v = sc.exact(a) + sc.exact(b)
        both_zero_same = (a | b) & 0x7fffffff == 0 and sc.sign(a) == sc.sign(b)
        zs = sc.sign(a) if both_zero_same else (1 if mode == 'rd' else 0)
        return round_fmt(v, mode, zero_sign=zs)
    if op == 'sub':
        return reference('add', mode, [row[0], row[1] ^ 0x80000000])
    if op == 'mul':
        a, b = row
        return round_fmt(sc.exact(a) * sc.exact(b), mode, zero_sign=sc.sign(a) ^ sc.sign(b))
    if op == 'fma':
        a, b, c = row
        p = sc.exact(a) * sc.exact(b)
        v = p + sc.exact(c)
        ps = sc.sign(a) ^ sc.sign(b)
        if v == 0:
            if p == 0 and (c & 0x7fffffff) == 0 and ps == sc.sign(c):
                zs = ps
            else:
                zs = 1 if mode == 'rd' else 0
        else:
            zs = 0
        return round_fmt(v, mode, zero_sign=zs)
    if op == 'div':
        a, b = row
        if (a & 0x7fffffff) == 0:
            return (sc.sign(a) ^ sc.sign(b)) << 31
        return round_fmt(sc.exact(a) / sc.exact(b), mode, zero_sign=sc.sign(a) ^ sc.sign(b))
    if op == 'sqrt':
        a = row[0]
        if (a & 0x7fffffff) == 0:
            return a
        v = sc.exact(a)
        # sqrt rounded in a direction: bracket with the exact root's square.
        rn = sc.isqrt_frac(v)
        x = sc.exact(rn)
        if x * x == v:
            return rn
        below = x * x > v  # rn above the exact root
        if mode == 'rn':
            return rn
        if mode in ('rz', 'rd'):
            return rn - 1 if below else rn
        return rn if below else rn + 1
    if op in ('sitofp', 'uitofp'):
        v = row[0]
        if op == 'sitofp' and v >= 1 << 31:
            v -= 1 << 32
        return round_fmt(Fraction(v), mode)
    if op == 'fptrunc':
        a = row[0]
        if sc.is_nan(a):
            return 0x7fff
        if sc.is_inf(a):
            return 0xfc00 if sc.sign(a) else 0x7c00
        return round_fmt(sc.exact(a), mode, 11, 15, 16, zero_sign=sc.sign(a))
    raise KeyError(op)


def kernel(op, mode, dynamic):
    md, code = MODES[mode]
    rnd = 'round.dynamic' if dynamic else md
    exc = 'metadata !"fpexcept.ignore"'
    r = f'metadata !"{rnd}", {exc}'
    body = {
        'add': f'%r = call float @llvm.experimental.constrained.fadd.f32(float %x, float %y, {r})',
        'sub': f'%r = call float @llvm.experimental.constrained.fsub.f32(float %x, float %y, {r})',
        'mul': f'%r = call float @llvm.experimental.constrained.fmul.f32(float %x, float %y, {r})',
        'div': f'%r = call float @llvm.experimental.constrained.fdiv.f32(float %x, float %y, {r})',
        'fma': f'%r = call float @llvm.experimental.constrained.fma.f32(float %x, float %y, float %z, {r})',
        'sqrt': f'%r = call float @llvm.experimental.constrained.sqrt.f32(float %x, {r})',
        'sitofp': f'%xi = bitcast float %x to i32\n  %r = call float @llvm.experimental.constrained.sitofp.f32.i32(i32 %xi, {r})',
        'uitofp': f'%xi = bitcast float %x to i32\n  %r = call float @llvm.experimental.constrained.uitofp.f32.i32(i32 %xi, {r})',
        'fptrunc': f'%h = call half @llvm.experimental.constrained.fptrunc.f16.f32(float %x, {r})\n'
                   f'  %hi = bitcast half %h to i16\n  %r = zext i16 %hi to i32',
    }[op]
    rty = 'i32' if op == 'fptrunc' else 'float'
    setmode = f'call void @llvm.agx.set.fpmode(i32 {code | (code << 2)})' if dynamic else ''
    return f'''declare i32 @llvm.agx.thread.position.in.grid.x()
declare void @llvm.agx.set.fpmode(i32)
declare float @llvm.experimental.constrained.fadd.f32(float, float, metadata, metadata)
declare float @llvm.experimental.constrained.fsub.f32(float, float, metadata, metadata)
declare float @llvm.experimental.constrained.fmul.f32(float, float, metadata, metadata)
declare float @llvm.experimental.constrained.fdiv.f32(float, float, metadata, metadata)
declare float @llvm.experimental.constrained.fma.f32(float, float, float, metadata, metadata)
declare float @llvm.experimental.constrained.sqrt.f32(float, metadata, metadata)
declare float @llvm.experimental.constrained.sitofp.f32.i32(i32, metadata, metadata)
declare float @llvm.experimental.constrained.uitofp.f32.i32(i32, metadata, metadata)
declare half @llvm.experimental.constrained.fptrunc.f16.f32(float, metadata, metadata)
define agx_kernel void @k(ptr addrspace(1) %a, ptr addrspace(1) %b, ptr addrspace(1) %c, ptr addrspace(1) %o) #0 {{
  %t = call i32 @llvm.agx.thread.position.in.grid.x()
  {setmode}
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
attributes #0 = {{ strictfp }}
'''


ARITY = {'add': 2, 'sub': 2, 'mul': 2, 'div': 2, 'fma': 3, 'sqrt': 1,
         'sitofp': 1, 'uitofp': 1, 'fptrunc': 1}


def run(op, mode, dynamic, n, args, tmp):
    src = os.path.join(tmp, 'k.ll')
    open(src, 'w').write(kernel(op, mode, dynamic))
    obj = os.path.join(tmp, 'k.o')
    r = subprocess.run([args.llc, '-mtriple=agx', '-mcpu=' + args.cpu, '-mattr=' + args.features,
                        '-filetype=obj', src, '-o', obj], capture_output=True, text=True)
    if r.returncode:
        return 'compile error: ' + r.stderr[-600:]
    code = os.path.join(tmp, 'k.bin')
    subprocess.run([args.objcopy, '-O', 'binary', '--only-section=.text', obj, code], check=True)
    rng = random.Random(args.seed)
    arity = ARITY[op]
    base_op = op if op in ('add', 'sub', 'mul', 'fma', 'div', 'sqrt') else 'add'
    rows = [r[:arity] for r in sc.gen(rng, n, 3, base_op)]
    if op in ('sitofp', 'uitofp'):
        rows = [[rng.getrandbits(rng.choice([8, 24, 25, 31, 32]))] for _ in range(n)]
    bufs = [b''.join(struct.pack('<I', row[i] if i < len(row) else 0) for row in rows) for i in range(3)]
    base = [0x100000, 0x200000, 0x300000, 0x400000]
    cmd = [args.agx_run, code, '--grid', str(n), '--wg', '64']
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
    bad = []
    canon = 'canonical-nan' in args.features
    for row, g in zip(rows, got):
        w = reference(op, mode, row)
        nan32 = op != 'fptrunc' and sc.is_nan(w)
        nan16 = op == 'fptrunc' and (w & 0x7c00) == 0x7c00 and w & 0x3ff
        if not canon and (nan32 and sc.is_nan(g) or nan16 and (g & 0x7c00) == 0x7c00 and g & 0x3ff):
            continue
        if w != g:
            bad.append((row, w, g))
    return bad


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('ops', nargs='*')
    ap.add_argument('-n', type=int, default=4096)
    ap.add_argument('--features', default='+canonical-nan')
    ap.add_argument('--cpu', default='agx-g13g')
    ap.add_argument('--seed', type=int, default=1)
    ap.add_argument('--llc', default='llc')
    ap.add_argument('--objcopy', default='llvm-objcopy')
    ap.add_argument('--agx-run', dest='agx_run', default='agx_run')
    args = ap.parse_args()
    ops = args.ops or list(ARITY)
    ok = True
    with tempfile.TemporaryDirectory() as tmp:
        for op in ops:
            for mode in ('rz', 'ru', 'rd'):
                for dynamic in (False, True):
                    res = run(op, mode, dynamic, args.n, args, tmp)
                    name = '%s.%s%s' % (op, mode, ' (dynamic)' if dynamic else '')
                    if isinstance(res, str):
                        print('%-20s %s' % (name, res))
                        ok = False
                        continue
                    print('%-20s %d/%d bit-exact' % (name, args.n - len(res), args.n))
                    for row, w, g in res[:4]:
                        print('    in=%s expected %08x got %08x' % (' '.join('%08x' % x for x in row), w, g))
                    ok &= not res
    return 0 if ok else 1


if __name__ == '__main__':
    sys.exit(main())
