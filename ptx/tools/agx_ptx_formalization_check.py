#!/usr/bin/env python3
"""Check the AGX compiler against the PTX-formalization test programs.

Each test of PTX-formalization (https://github.com/Jianwen-Ding/PTX-formalization,
tests/*.ptx) is a PTX module followed by a host harness (.buffer / .launch /
.expect ...) whose .expect values are the results of the K semantics of PTX.
This script compiles the module with `zoc --target agx`, runs every launch on
the functional AGX simulator (`agx_run` of
https://github.com/Jianwen-Ding/AGX-Performance-Simulator) and checks the
.expect directives bit-exactly, like `krun` does.

Statements the front end rejects are dropped with --ignore-errors; launches of
kernels that contain one (or call a function that does) are skipped, and so
are the checks after them. Kernel parameters are passed as pointers to
parameter memory, like Metal buffer bindings (see docs/src/agx.md).

    python3 ptx/tools/agx_ptx_formalization_check.py --zoc target/debug/zoc \
        --agx-run .../agx_run PTX-formalization/tests/*.ptx
"""

import argparse
import json
import math
import os
import re
import struct
import subprocess
import sys
import tempfile
from concurrent.futures import ThreadPoolExecutor

HARNESS = ('.buffer', '.launch', '.cooplaunch', '.print', '.expect', '.patch', '.append',
           '.tensormap', '.texture', '.sampler', '.surface', '.multimem', '.fabric',
           '.memmodel', '.ub', '.occupancy', '.convergence', '.isacheck')
IGNORED = ('.print', '.memmodel', '.ub', '.occupancy', '.convergence', '.isacheck')
UNSUPPORTED = ('.tensormap', '.texture', '.sampler', '.surface', '.multimem', '.fabric')

SIZES = {'b8': 1, 'u8': 1, 's8': 1, 'b16': 2, 'u16': 2, 's16': 2, 'f16': 2, 'bf16': 2,
         'b32': 4, 'u32': 4, 's32': 4, 'f32': 4, 'f16x2': 4, 'bf16x2': 4,
         'b64': 8, 'u64': 8, 's64': 8, 'f64': 8, 'pred': 1, 'e4m3': 1, 'e5m2': 1}

BASE = 0x10000000          # first harness buffer
PARAM_BASE = 0x08000000    # kernel parameter memory
GLOBALS_BASE = 0x04000000  # global data buffer (.agx.globals)


class Unsupported(Exception):
    pass


# ------------------------------------------------------------------ parsing

def strip_comments(text):
    text = re.sub(r'/\*.*?\*/', ' ', text, flags=re.S)
    return re.sub(r'//[^\n]*', '', text)


def split_program(text):
    """Return (module text, list of harness statements in order)."""
    text = strip_comments(text)
    module, dirs = [], []
    i, depth, n = 0, 0, len(text)
    while i < n:
        c = text[i]
        if c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
        elif c == '.' and depth == 0 and (i == 0 or text[i - 1] in ' \t\n;}'):
            m = re.match(r'\.[a-z]+', text[i:])
            if m and m.group(0) in HARNESS:
                j, d = i, 0
                while j < n and not (text[j] == ';' and d == 0):
                    d += {'{': 1, '}': -1}.get(text[j], 0)
                    j += 1
                dirs.append(' '.join(text[i:j].split()))
                module.append('\n')
                i = j + 1
                continue
        module.append(c)
        i += 1
    return ''.join(module), dirs


def split_vals(s):
    """Flatten a (possibly nested) brace initializer list."""
    s = s.replace('{', ' ').replace('}', ' ')
    return [v.strip() for v in s.split(',') if v.strip()]


def ty_of(t):
    t = t.lstrip('.')
    if t not in SIZES:
        raise Unsupported('type .' + t)
    return t


def is_float(t):
    return t in ('f16', 'bf16', 'f32', 'f64')


def width(t):
    return SIZES[t] * 8


def enc_float(x, t):
    if t == 'f64':
        return struct.unpack('<Q', struct.pack('<d', x))[0]
    if t == 'f32':
        try:
            return struct.unpack('<I', struct.pack('<f', x))[0]
        except OverflowError:
            return 0x7f800000 if x > 0 else 0xff800000
    if t == 'f16':
        try:
            return struct.unpack('<H', struct.pack('<e', x))[0]
        except OverflowError:
            return 0x7c00 if x > 0 else 0xfc00
    if t == 'bf16':
        if math.isnan(x):
            return 0x7fc0
        b = struct.unpack('<Q', struct.pack('<d', x))[0]
        # round the double to bf16 (8 mantissa bits) to nearest even
        f = struct.unpack('<I', struct.pack('<f', x))[0] if abs(x) < 3.4e38 else (0x7f800000 | (b >> 32 & 0x80000000))
        lsb = (f >> 16) & 1
        return ((f + 0x7fff + lsb) >> 16) & 0xffff
    raise Unsupported('float type ' + t)


def dec_float(bits, t):
    if t == 'f32':
        return struct.unpack('<f', struct.pack('<I', bits))[0]
    if t == 'f64':
        return struct.unpack('<d', struct.pack('<Q', bits))[0]
    if t == 'f16':
        return struct.unpack('<e', struct.pack('<H', bits))[0]
    raise Unsupported(t)


def lit_bits(v, t, syms):
    """initBits of the K harness: the bits of literal v at type t."""
    w = width(t)
    mask = (1 << w) - 1
    if re.fullmatch(r'0[xXfFdDbB][0-9a-fA-F]+[uU]?', v):
        p, d = v[1].lower(), v[2:].rstrip('uU')
        if p == 'f' and is_float(t) and t != 'f32':
            return enc_float(dec_float(int(d, 16), 'f32'), t)
        if p == 'd' and is_float(t) and t != 'f64':
            return enc_float(dec_float(int(d, 16), 'f64'), t)
        if p == 'b':
            return int(d, 2) & mask
        return int(d, 16) & mask
    if re.fullmatch(r'[-+]?\d+', v):
        i = int(v)
        return enc_float(float(i), t) if is_float(t) else i & mask
    if re.fullmatch(r'[-+]?(\d+\.?\d*([eE][-+]?\d+)?|\.\d+([eE][-+]?\d+)?|inf|nan|Infinity|NaN)[fF]?', v):
        x = float(v.rstrip('fF'))
        if is_float(t):
            return enc_float(x, t)
        return int(x) & mask
    if v in syms:
        return syms[v] & mask
    raise Unsupported('initializer ' + v)


def parse_dim(s):
    s = s.strip()
    if s.startswith('('):
        v = [int(x) for x in s.strip('()').split(',')]
    else:
        v = [int(s)]
    return (v + [1, 1])[:3]


def kernel_params(module):
    """Map kernel name -> list of (type, bytes) of its .param declarations."""
    out = {}
    for m in re.finditer(r'\.entry\s+([\w$.]+)\s*(\((.*?)\))?\s*[{.]', module, re.S):
        params = []
        if m.group(3):
            for p in m.group(3).split(','):
                p = p.strip()
                if not p:
                    continue
                tm = re.search(r'\.(b8|u8|s8|b16|u16|s16|f16|bf16|b32|u32|s32|f32|b64|u64|s64|f64|pred)\b', p)
                am = re.search(r'\[(\d+)\]', p)
                al = re.search(r'\.align\s+(\d+)', p)
                t = tm.group(1) if tm else 'b8'
                n = int(am.group(1)) if am else 1
                params.append((t, n, int(al.group(1)) if al else SIZES[t]))
        out[m.group(1)] = params
    return out


def function_bodies(module):
    """Map function name -> body text, for .entry and .func definitions."""
    out = {}
    for m in re.finditer(r'\.(entry|func)\s+(\([^)]*\)\s*)?([\w$.]+)', module):
        i = module.find('{', m.end())
        semi = module.find(';', m.end())
        if i < 0 or (0 <= semi < i and '(' not in module[m.end():semi + 1]):
            continue  # prototype
        depth, j = 0, i
        while j < len(module):
            depth += {'{': 1, '}': -1}.get(module[j], 0)
            if depth == 0:
                break
            j += 1
        out[m.group(3)] = module[i:j + 1]
    return out


def tainted_functions(module, rejected):
    """Functions containing a statement the compiler rejected, and their callers."""
    bodies = function_bodies(module)
    norm = {n: ' '.join(b.split()) for n, b in bodies.items()}
    tainted = set()
    for stmt in rejected:
        key = ' '.join(stmt.split()).rstrip(';')
        hits = [n for n, b in norm.items() if key and key in b]
        if not hits:
            return None  # cannot attribute it: everything is suspect
        tainted.update(hits)
    changed = True
    while changed:
        changed = False
        for n, b in norm.items():
            if n not in tainted and any(re.search(r'\b' + re.escape(t) + r'\b', b) for t in tainted):
                tainted.add(n)
                changed = True
    return tainted


# ------------------------------------------------------------------ object file

def kernel_info(asm, obj, readelf, objcopy, tmp):
    """Per-kernel code bytes, uniform layout and resources from zoc's output."""
    syms = {}
    out = subprocess.run([readelf, '-sSW', obj], capture_output=True, text=True, check=True).stdout
    sections = {}
    for m in re.finditer(r'\[\s*(\d+)\]\s+(\S+)\s+\S+\s+[0-9a-f]+\s+([0-9a-f]+)\s+([0-9a-f]+)', out):
        sections[int(m.group(1))] = (m.group(2), int(m.group(3), 16), int(m.group(4), 16))
    for m in re.finditer(r'^\s*\d+:\s+([0-9a-f]+)\s+(\d+)\s+(FUNC|OBJECT)\s+\S+\s+\S+\s+(\d+)\s+(\S+)$', out, re.M):
        syms[m.group(5)] = (int(m.group(1), 16), int(m.group(2)), m.group(3), int(m.group(4)))
    blobs = {}
    for idx, (name, off, size) in sections.items():
        if name in ('.text', '.agx.globals'):
            path = os.path.join(tmp, name.strip('.') + '.bin')
            subprocess.run([objcopy, '-O', 'binary', '--only-section=' + name, obj, path], check=True)
            with open(path, 'rb') as f:
                blobs[idx] = (name, f.read())
    text = next((b for n, b in blobs.values() if n == '.text'), b'')
    gdata = next((b for n, b in blobs.values() if n == '.agx.globals'), b'')
    gsec = next((i for i, (n, _) in blobs.items() if n == '.agx.globals'), None)
    kernels = {}
    for m in re.finditer(r'^([\w$.]+):\s*(?://.*)?\n((?:\s*//[^\n]*\n)*)', asm, re.M):
        name, comments = m.group(1), m.group(2)
        if name not in syms or syms[name][2] != 'FUNC':
            continue
        u = re.search(r'threadgroups_per_grid at u(\d+)', comments)
        g = re.search(r'global data buffer at u(\d+)_u\d+', comments)
        tg = re.search(r'threadgroup memory: (\d+) bytes', comments)
        off, size = syms[name][0], syms[name][1]
        kernels[name] = {
            'code': text[off:off + size],
            'grid_uniform': int(u.group(1)) if u else None,
            'globals_uniform': int(g.group(1)) if g else None,
            'shared': int(tg.group(1)) if tg else 0,
        }
    gsyms = {n: v[0] for n, v in syms.items() if v[2] == 'OBJECT' and v[3] == gsec}
    return kernels, gdata, gsyms


# ------------------------------------------------------------------ running

class Memory:
    def __init__(self):
        self.buf = bytearray()
        self.top = 0

    def ensure(self, end):
        if end > len(self.buf):
            self.buf.extend(b'\0' * (end - len(self.buf)))

    def write(self, addr, n, bits):
        off = addr - BASE
        self.ensure(off + n)
        self.buf[off:off + n] = bits.to_bytes(n, 'little')

    def read(self, addr, n):
        off = addr - BASE
        self.ensure(off + n)
        return int.from_bytes(self.buf[off:off + n], 'little')


def run_test(path, args):
    with open(path) as f:
        module, dirs = split_program(f.read())
    for d in dirs:
        if d.split()[0] in UNSUPPORTED:
            return 'UNSUPPORTED', 'harness directive ' + d.split()[0]
    with tempfile.TemporaryDirectory() as tmp:
        src = os.path.join(tmp, 'm.ptx')
        with open(src, 'w') as f:
            f.write(module)
        def zoc(*extra):
            if args.strict_fp:
                extra = extra + ('--strict-fp',)
            return subprocess.run([args.zoc, *extra, '--target', 'agx', '--arch', args.arch,
                                   '--output-dir', tmp, src],
                                  capture_output=True, text=True, timeout=args.timeout)
        try:
            # The statements the front end rejects (printed only without
            # --ignore-errors); then compile the rest.
            r = zoc()
            out = r.stderr + r.stdout
            rejected = sorted(set(re.findall(r'PtxError::\w+: ([^\n]*)', out)))
            if rejected or 'PtxError::' in out:
                r = zoc('--ignore-errors')
                out = out + r.stderr + r.stdout
        except subprocess.TimeoutExpired:
            return 'COMPILE-ERROR', 'zoc timed out'
        if 'PtxError::Parser' in out or 'UnrecognizedDirective' in out:
            tainted = None
        else:
            tainted = tainted_functions(module, [x for x in rejected if x.strip()])
        if r.returncode != 0 or not os.path.exists(os.path.join(tmp, 'm.o')):
            panic = re.search(r"panicked at [^\n]*\n([^\n]*)", out)
            if panic and rejected:
                # dropping the rejected statements broke the rest of the module
                return 'REJECTED', ' | '.join(rejected)[:300]
            if panic:
                return 'CRASH', panic.group(0).replace('\n', ' ')[:300]
            if rejected:
                return 'REJECTED', ' | '.join(rejected)[:300]
            msg = [l for l in out.splitlines() if l.strip() and not l.startswith(('Pass "', 'Wrote to'))]
            return 'COMPILE-ERROR', ' | '.join(msg[:3])[:300]
        with open(os.path.join(tmp, 'm.asm')) as f:
            asm = f.read()
        kernels, gdata, gsyms = kernel_info(asm, os.path.join(tmp, 'm.o'), args.readelf, args.objcopy, tmp)
        params = kernel_params(module)

        mem = Memory()
        syms, bufs, last = {}, {}, None
        globals_mem = bytearray(gdata) + bytearray(max(0, 16 - len(gdata)))
        for n, off in gsyms.items():
            syms[n] = GLOBALS_BASE + off
        failures, launches, skipped, passed, skipped_launches = [], 0, 0, 0, []

        def gwrite(addr, n, bits):
            if GLOBALS_BASE <= addr < GLOBALS_BASE + len(globals_mem):
                o = addr - GLOBALS_BASE
                globals_mem[o:o + n] = bits.to_bytes(n, 'little')
            else:
                mem.write(addr, n, bits)

        def gread(addr, n):
            if GLOBALS_BASE <= addr < GLOBALS_BASE + len(globals_mem):
                o = addr - GLOBALS_BASE
                return int.from_bytes(globals_mem[o:o + n], 'little')
            return mem.read(addr, n)

        for d in dirs:
            kw = d.split()[0]
            if kw in IGNORED:
                continue
            if kw == '.buffer':
                m = re.fullmatch(r'\.buffer ([\w$.]+) \.(\w+)\s*(\[\s*(\d+)\s*\])?\s*(=\s*\{(.*)\})?', d)
                if not m:
                    raise Unsupported(d)
                t = ty_of(m.group(2))
                vals = split_vals(m.group(6)) if m.group(6) is not None else []
                count = int(m.group(4)) if m.group(4) else len(vals)
                addr = BASE + ((mem.top + 255) & ~255)
                mem.top = (addr - BASE) + max(count * SIZES[t], 1)
                mem.ensure(mem.top)
                syms[m.group(1)] = addr
                bufs[m.group(1)] = [addr, t, count]
                last = m.group(1)
                for k, v in enumerate(vals):
                    mem.write(addr + k * SIZES[t], SIZES[t], lit_bits(v, t, syms))
            elif kw in ('.patch', '.append'):
                m = re.fullmatch(r'\.(patch|append) ([\w$.]+) \.(\w+)\s*(@\s*(\d+))?\s*=\s*\{(.*)\}', d)
                t = ty_of(m.group(3))
                vals = split_vals(m.group(6))
                name = m.group(2)
                if m.group(1) == 'patch':
                    base = syms[name] + int(m.group(5)) * SIZES[t]
                else:
                    b = bufs[name]
                    base = b[0] + b[2] * SIZES[b[1]]
                    b[2] += len(vals)
                    mem.top = max(mem.top, base - BASE + len(vals) * SIZES[t])
                for k, v in enumerate(vals):
                    gwrite(base + k * SIZES[t], SIZES[t], lit_bits(v, t, syms))
            elif kw in ('.launch', '.cooplaunch'):
                m = re.fullmatch(r'\.\w+ ([\w$.]+) <<<\s*(\([^)]*\)|\d+)\s*,\s*(\([^)]*\)|\d+)\s*(,\s*(\d+))?\s*>>>\s*\((.*)\)', d)
                if not m:
                    raise Unsupported(d)
                name = m.group(1)
                if tainted is None or name in tainted or name not in kernels:
                    # rejected by the compiler (an unsupported statement)
                    skipped_launches.append(name)
                    continue
                k = kernels[name]
                grid, block = parse_dim(m.group(2)), parse_dim(m.group(3))
                dyn = int(m.group(5)) if m.group(5) else 0
                ops = [o.strip() for o in m.group(6).split(',') if o.strip()]
                ps = params.get(name, [])
                if len(ops) != len(ps):
                    raise Unsupported('launch argument count')
                pmem = bytearray()
                uniforms = []
                for i, ((t, n, al), o) in enumerate(zip(ps, ops)):
                    if n != 1:
                        raise Unsupported('aggregate kernel parameter')
                    off = (len(pmem) + 15) & ~15
                    pmem.extend(b'\0' * (off - len(pmem)))
                    pmem.extend(lit_bits(o, t, syms).to_bytes(SIZES[t], 'little'))
                    addr = PARAM_BASE + off
                    uniforms += [(2 * i, addr & 0xffffffff), (2 * i + 1, addr >> 32)]
                if re.search(r'^\s*\.address_size\s+32\b', module, re.M):
                    # 32-bit modules take the 64-bit base of their 32-bit
                    # addresses as a last parameter; the harness buffers are
                    # below 4 GiB, so it is 0.
                    off = (len(pmem) + 15) & ~15
                    pmem.extend(b'\0' * (off + 8 - len(pmem)))
                    addr = PARAM_BASE + off
                    i = len(ps)
                    uniforms += [(2 * i, addr & 0xffffffff), (2 * i + 1, addr >> 32)]
                if k['grid_uniform'] is not None:
                    for j in range(3):
                        uniforms.append((k['grid_uniform'] + j, grid[j]))
                if k['globals_uniform'] is not None:
                    uniforms += [(k['globals_uniform'], GLOBALS_BASE), (k['globals_uniform'] + 1, 0)]
                f_code = os.path.join(tmp, 'k%d.bin' % launches)
                f_mem = os.path.join(tmp, 'mem.bin')
                f_par = os.path.join(tmp, 'par.bin')
                f_glob = os.path.join(tmp, 'glob.bin')
                with open(f_code, 'wb') as f:
                    f.write(k['code'])
                mem.ensure(max(mem.top, 16))
                with open(f_mem, 'wb') as f:
                    f.write(mem.buf)
                with open(f_par, 'wb') as f:
                    f.write(pmem or b'\0' * 16)
                with open(f_glob, 'wb') as f:
                    f.write(globals_mem or b'\0' * 16)
                cmd = [args.agx_run, f_code,
                       '--grid', ','.join(str(grid[j] * block[j]) for j in range(3)),
                       '--wg', ','.join(map(str, block)),
                       '--shared', str(k['shared'] + dyn),
                       '--map', '%#x=%s' % (BASE, f_mem), '--map', '%#x=%s' % (PARAM_BASE, f_par),
                       '--map', '%#x=%s' % (GLOBALS_BASE, f_glob),
                       '--dump', '%#x=%d:%s' % (BASE, len(mem.buf), f_mem),
                       '--dump', '%#x=%d:%s' % (GLOBALS_BASE, len(globals_mem), f_glob)]
                for u, v in uniforms:
                    cmd += ['--uniform', '%d=%d' % (u, v)]
                try:
                    r = subprocess.run(cmd, capture_output=True, text=True, timeout=args.timeout)
                except subprocess.TimeoutExpired:
                    return 'SIM-ERROR', 'simulation timed out (%s)' % name
                if r.returncode != 0:
                    try:
                        err = json.loads(r.stdout.strip().splitlines()[-1])['error']
                    except Exception:
                        err = (r.stdout + r.stderr).strip()[-300:]
                    return 'SIM-ERROR', '%s: %s' % (name, err[:300])
                with open(f_mem, 'rb') as f:
                    mem.buf = bytearray(f.read())
                with open(f_glob, 'rb') as f:
                    g = f.read()
                    globals_mem[:] = g[:len(globals_mem)]
                launches += 1
            elif kw == '.expect':
                m = re.fullmatch(r'\.expect ([\w$.]+) \.(\w+)\s*(@\s*(\d+))?\s*=\s*\{(.*)\}', d)
                t = ty_of(m.group(2))
                name = m.group(1)
                if name not in syms:
                    raise Unsupported('.expect of unknown symbol ' + name)
                off = int(m.group(4)) if m.group(4) else 0
                vals = split_vals(m.group(5))
                if skipped_launches:
                    skipped += 1
                    continue
                bad = []
                for i, v in enumerate(vals):
                    want = lit_bits(v, t, syms)
                    got = gread(syms[name] + (off + i) * SIZES[t], SIZES[t])
                    if want != got:
                        bad.append('%s[%d] expected %#x, got %#x' % (name, off + i, want, got))
                if bad:
                    failures += bad
                    args.counts['fail'] += 1
                else:
                    passed += 1
        args.counts['pass'] += passed
        args.counts['skip'] += skipped
        if failures:
            return 'FAIL', '; '.join(failures[:4]) + (' (+%d)' % (len(failures) - 4) if len(failures) > 4 else '')
        if skipped_launches:
            return 'PARTIAL', '%d check(s) pass, %d skipped after rejected kernels %s; rejected: %s' % (
                passed, skipped, ','.join(sorted(set(skipped_launches))), ' | '.join(rejected)[:200])
        return 'PASS', '%d launch(es), %d check(s)' % (launches, passed)


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('tests', nargs='+', help='PTX-formalization test programs (tests/*.ptx)')
    ap.add_argument('--zoc', default='zoc')
    ap.add_argument('--agx-run', dest='agx_run', default='agx_run')
    ap.add_argument('--readelf', default='llvm-readelf')
    ap.add_argument('--objcopy', default='llvm-objcopy')
    ap.add_argument('--arch', default='agx-g14g')
    ap.add_argument('--strict-fp', action='store_true',
                    help='compile with zoc --strict-fp (software IEEE binary32, canonical NaNs)')
    ap.add_argument('--timeout', type=int, default=120)
    ap.add_argument('-j', type=int, default=os.cpu_count())
    ap.add_argument('--json', help='write the results to this file')
    args = ap.parse_args()
    args.counts = {'pass': 0, 'fail': 0, 'skip': 0}

    def one(p):
        try:
            return p, run_test(p, args)
        except Unsupported as e:
            return p, ('UNSUPPORTED', str(e))
        except Exception as e:
            return p, ('HARNESS-ERROR', '%s: %s' % (type(e).__name__, e))

    with ThreadPoolExecutor(args.j) as ex:
        results = list(ex.map(one, sorted(args.tests)))
    counts = {}
    for p, (status, detail) in results:
        counts[status] = counts.get(status, 0) + 1
        print('%-14s %-40s %s' % (status, os.path.basename(p), detail))
    print('---- files:', ', '.join('%d %s' % (v, k) for k, v in sorted(counts.items())))
    print('---- .expect checks: %(pass)d pass, %(fail)d fail, %(skip)d skipped (after a rejected kernel)' % args.counts)
    if args.json:
        with open(args.json, 'w') as f:
            json.dump({os.path.basename(p): {'status': s, 'detail': d} for p, (s, d) in results}, f, indent=1)
    return 0 if all(s in ('PASS', 'PARTIAL', 'UNSUPPORTED', 'REJECTED') for _, (s, _) in results) else 1


if __name__ == '__main__':
    sys.exit(main())
