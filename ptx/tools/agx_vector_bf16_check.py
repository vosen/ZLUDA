#!/usr/bin/env python3
"""Check the strict floating point mode of the AGX backend on vector float
and on bfloat arithmetic, bit by bit.

Vector float operations (<2 x float>, plain with +ieee-fp32 and constrained
with a directed rounding) and bfloat operations are compiled, run on the
functional AGX simulator over generated inputs (with denormals) and compared
lane by lane with an exact reference (see agx_strict_fp_check.py and
agx_rounding_check.py).

    python3 ptx/tools/agx_vector_bf16_check.py --llc $LLVM/bin/llc \\
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
import agx_rounding_check as rc  # noqa: E402

F32_REF = {'add': sc.ref_add, 'sub': sc.ref_sub, 'mul': sc.ref_mul,
           'div': sc.ref_div, 'fma': sc.ref_fma, 'sqrt': sc.ref_sqrt}
INS = {'add': 'fadd', 'sub': 'fsub', 'mul': 'fmul', 'div': 'fdiv'}
ARITY = {'add': 2, 'sub': 2, 'mul': 2, 'div': 2, 'fma': 3, 'sqrt': 1,
         'minnum': 2, 'olt': 2}


def vector_kernel(op, mode):
    if mode == 'rn':
        if op in INS:
            e = f'{INS[op]} <2 x float> %x, %y'
        elif op == 'fma':
            e = 'call <2 x float> @llvm.fma.v2f32(<2 x float> %x, <2 x float> %y, <2 x float> %z)'
        else:
            e = 'call <2 x float> @llvm.sqrt.v2f32(<2 x float> %x)'
    else:
        r = f'metadata !"{rc.MODES[mode][0]}", metadata !"fpexcept.ignore"'
        name = {'add': 'fadd', 'sub': 'fsub', 'mul': 'fmul', 'div': 'fdiv',
                'fma': 'fma', 'sqrt': 'sqrt'}[op]
        args = {1: '<2 x float> %x', 2: '<2 x float> %x, <2 x float> %y',
                3: '<2 x float> %x, <2 x float> %y, <2 x float> %z'}[ARITY[op]]
        e = f'call <2 x float> @llvm.experimental.constrained.{name}.v2f32({args}, {r}) #0'
    return f'''declare i32 @llvm.agx.thread.position.in.grid.x()
define agx_kernel void @k(ptr addrspace(1) %a, ptr addrspace(1) %b, ptr addrspace(1) %c, ptr addrspace(1) %o) #0 {{
  %t = call i32 @llvm.agx.thread.position.in.grid.x()
  %pa = getelementptr <2 x float>, ptr addrspace(1) %a, i32 %t
  %pb = getelementptr <2 x float>, ptr addrspace(1) %b, i32 %t
  %pc = getelementptr <2 x float>, ptr addrspace(1) %c, i32 %t
  %x = load <2 x float>, ptr addrspace(1) %pa
  %y = load <2 x float>, ptr addrspace(1) %pb
  %z = load <2 x float>, ptr addrspace(1) %pc
  %r = {e}
  %po = getelementptr <2 x float>, ptr addrspace(1) %o, i32 %t
  store <2 x float> %r, ptr addrspace(1) %po
  ret void
}}
attributes #0 = {{ strictfp }}
'''


def bf16_kernel(op):
    if op in INS:
        e = f'%r = {INS[op]} bfloat %x, %y'
    elif op == 'fma':
        e = '%r = call bfloat @llvm.fma.bf16(bfloat %x, bfloat %y, bfloat %z)'
    elif op == 'sqrt':
        e = '%r = call bfloat @llvm.sqrt.bf16(bfloat %x)'
    elif op == 'minnum':
        e = '%r = call bfloat @llvm.minnum.bf16(bfloat %x, bfloat %y)'
    else:
        e = '%c1 = fcmp olt bfloat %x, %y\n  %r = select i1 %c1, bfloat 1.0, bfloat 0.0'
    return f'''declare i32 @llvm.agx.thread.position.in.grid.x()
define agx_kernel void @k(ptr addrspace(1) %a, ptr addrspace(1) %b, ptr addrspace(1) %c, ptr addrspace(1) %o) {{
  %t = call i32 @llvm.agx.thread.position.in.grid.x()
  %pa = getelementptr i32, ptr addrspace(1) %a, i32 %t
  %pb = getelementptr i32, ptr addrspace(1) %b, i32 %t
  %pc = getelementptr i32, ptr addrspace(1) %c, i32 %t
  %xi = load i32, ptr addrspace(1) %pa
  %yi = load i32, ptr addrspace(1) %pb
  %zi = load i32, ptr addrspace(1) %pc
  %xh = trunc i32 %xi to i16
  %yh = trunc i32 %yi to i16
  %zh = trunc i32 %zi to i16
  %x = bitcast i16 %xh to bfloat
  %y = bitcast i16 %yh to bfloat
  %z = bitcast i16 %zh to bfloat
  {e}
  %ri = bitcast bfloat %r to i16
  %rw = zext i16 %ri to i32
  %po = getelementptr i32, ptr addrspace(1) %o, i32 %t
  store i32 %rw, ptr addrspace(1) %po
  ret void
}}
'''


def bf16_ref(op, row):
    """Exact bfloat reference: the operands as float, the float reference for
    special cases, and an exact rounding to bfloat otherwise."""
    xs = [v << 16 for v in row]
    if op == 'olt':
        a, b = xs[:2]
        if sc.is_nan(a) or sc.is_nan(b):
            return 0
        return 0x3f80 if sc.exact(a) < sc.exact(b) else 0
    if op == 'minnum':
        a, b = xs[:2]
        if sc.is_nan(a) and sc.is_nan(b):
            return 0x7fff
        if sc.is_nan(a):
            return row[1]
        if sc.is_nan(b):
            return row[0]
        ka, kb = (sc.exact(a), -sc.sign(a)), (sc.exact(b), -sc.sign(b))
        return row[0] if ka <= kb else row[1]
    f = F32_REF[op](*xs[:ARITY[op]])
    if sc.is_nan(f):
        return 0x7fff
    if any(sc.is_nan(x) or sc.is_inf(x) for x in xs[:ARITY[op]]) or (f & 0x7fffffff) == 0 \
            or sc.is_inf(f):
        return f >> 16
    v = [sc.exact(x) for x in xs]
    if op == 'add':
        e = v[0] + v[1]
    elif op == 'sub':
        e = v[0] - v[1]
    elif op == 'mul':
        e = v[0] * v[1]
    elif op == 'div':
        e = v[0] / v[1]
    elif op == 'fma':
        e = v[0] * v[1] + v[2]
    else:
        # sqrt of a bfloat is never a bfloat midpoint: round the float root.
        e = sc.exact(sc.ref_sqrt(xs[0]))
    return rc.round_fmt(e, 'rn', 8, 127, 16)


def bf16_inputs(rng, n):
    special = [0, 0x8000, 0x0001, 0x8001, 0x007f, 0x0080, 0x3f80, 0xbf80, 0x7f7f,
               0x7f80, 0xff80, 0x7fc0, 0x7f81, 0x0040, 0x1f80, 0x2000]
    rows = []
    for _ in range(n):
        row = []
        for _ in range(3):
            k = rng.random()
            if k < 0.15:
                row.append(rng.choice(special))
            elif k < 0.45:
                row.append(rng.getrandbits(1) << 15 | rng.randint(0, 6) << 7 | rng.getrandbits(7))
            else:
                row.append(rng.getrandbits(16))
        rows.append(row)
    return rows


def simulate(args, tmp, src, rows, words_in, words_out, n):
    obj = os.path.join(tmp, 'k.o')
    r = subprocess.run([args.llc, '-mtriple=agx', '-mcpu=' + args.cpu, '-mattr=' + args.features,
                        '-filetype=obj', src, '-o', obj], capture_output=True, text=True)
    if r.returncode:
        lines = [l for l in r.stderr.splitlines() if 'error' in l.lower() or 'Assertion' in l]
        return 'compile error: ' + (lines[0] if lines else r.stderr[-300:])
    code = os.path.join(tmp, 'k.bin')
    subprocess.run([args.objcopy, '-O', 'binary', '--only-section=.text', obj, code], check=True)
    base = [0x100000, 0x200000, 0x300000, 0x400000]
    cmd = [args.agx_run, code, '--grid', str(n), '--wg', '64']
    for i in range(3):
        f = os.path.join(tmp, 'in%d.bin' % i)
        open(f, 'wb').write(b''.join(struct.pack('<%dI' % words_in, *w[i]) for w in rows))
        cmd += ['--map', '%#x=%s' % (base[i], f)]
    size = 4 * words_out * n
    out = os.path.join(tmp, 'out.bin')
    cmd += ['--alloc', '%#x=%d' % (base[3], size), '--dump', '%#x=%d:%s' % (base[3], size, out)]
    for i in range(4):
        cmd += ['--uniform', '%d=%d' % (2 * i, base[i]), '--uniform', '%d=0' % (2 * i + 1)]
    cmd += ['--uniform', '8=%d' % (n // 64), '--uniform', '9=1', '--uniform', '10=1']
    r = subprocess.run(cmd, capture_output=True, text=True)
    if r.returncode:
        return 'simulator error: ' + (r.stdout + r.stderr)[-300:]
    return struct.unpack('<%dI' % (words_out * n), open(out, 'rb').read())


def run_vector(op, mode, args, tmp):
    n = args.n
    rng = random.Random(args.seed)
    lanes = [sc.gen(rng, n, 3, op) for _ in range(2)]
    # rows[t][operand] = (lane0, lane1)
    rows = [[(lanes[0][t][k], lanes[1][t][k]) for k in range(3)] for t in range(n)]
    src = os.path.join(tmp, 'k.ll')
    open(src, 'w').write(vector_kernel(op, mode))
    got = simulate(args, tmp, src, rows, 2, 2, n)
    if isinstance(got, str):
        return got
    bad = []
    for t in range(n):
        for lane in range(2):
            ops = [rows[t][k][lane] for k in range(ARITY[op])]
            w = F32_REF[op](*ops) if mode == 'rn' else rc.reference(op, mode, ops)
            g = got[2 * t + lane]
            if w != g:
                bad.append((ops, w, g))
    return bad


def run_bf16(op, args, tmp):
    n = args.n
    rows = bf16_inputs(random.Random(args.seed), n)
    src = os.path.join(tmp, 'k.ll')
    open(src, 'w').write(bf16_kernel(op))
    got = simulate(args, tmp, src, [[[x] for x in r] for r in rows], 1, 1, n)
    if isinstance(got, str):
        return got
    return [(r[:ARITY[op]], w, g) for r, g in zip(rows, got)
            for w in [bf16_ref(op, r)] if w != g]


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('-n', type=int, default=2048)
    ap.add_argument('--features', default='+ieee-fp32,+canonical-nan')
    ap.add_argument('--cpu', default='agx-g13g')
    ap.add_argument('--seed', type=int, default=1)
    ap.add_argument('--llc', default='llc')
    ap.add_argument('--objcopy', default='llvm-objcopy')
    ap.add_argument('--agx-run', dest='agx_run', default='agx_run')
    args = ap.parse_args()
    ok = True
    with tempfile.TemporaryDirectory() as tmp:
        tests = [('v2f32.%s.%s' % (op, mode), lambda op=op, mode=mode: run_vector(op, mode, args, tmp))
                 for op in ('add', 'sub', 'mul', 'div', 'fma', 'sqrt') for mode in ('rn', 'rz', 'ru', 'rd')]
        tests += [('bf16.%s' % op, lambda op=op: run_bf16(op, args, tmp))
                  for op in ('add', 'sub', 'mul', 'div', 'fma', 'sqrt', 'minnum', 'olt')]
        for name, fn in tests:
            res = fn()
            if isinstance(res, str):
                print('%-18s %s' % (name, res))
                ok = False
                continue
            total = args.n * (2 if name.startswith('v2') else 1)
            print('%-18s %d/%d bit-exact' % (name, total - len(res), total))
            for ops, w, g in res[:4]:
                print('    in=%s expected %08x got %08x' % (' '.join('%08x' % x for x in ops), w, g))
            ok &= not res
    return 0 if ok else 1


if __name__ == '__main__':
    sys.exit(main())
