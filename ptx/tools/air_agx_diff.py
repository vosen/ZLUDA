#!/usr/bin/env python3
"""Differential test of ZLUDA's two Apple GPU paths on the PTX-formalization test programs.

Each program is compiled once with ZLUDA_AGX_VIA_AIR=1 (zoc --target agx), which writes NAME.air (portable AIR) and
NAME.o (AGX code compiled from that AIR by agx-lower-air). Every launch of the harness then runs, with identical
inputs, on
  air    the AIR, packaged for Metal (llc -mtriple=air -air-output=air), on the Apple GPU behind the AIR service
         (as air_ptx_formalization_check.py runs it), and
  agx    NAME.o on the functional AGX simulator agx_run (as agx_ptx_formalization_check.py runs it),
and, with --direct, also on
  direct the AGX code of the direct PTX -> AGX path (zoc without ZLUDA_AGX_VIA_AIR) on agx_run,
which separates bugs of the AIR raise / AGX lowering from bugs of the PTX front end.

Consecutive launches run as one service job on the AIR side; after each job (or each launch, with --per-launch)
ALL harness buffers and the module's global variables are compared bit for bit between the sides, whatever the
.expect directives check. Harness buffers have the same addresses on every side (the AIR harness's layout); values
that are addresses are compared as (symbol, offset): the device address of the AIR heap is learnt with an extra
address-patched probe buffer in each job, global variables by their symbols on each side.

Every differing element is classified:
  ftz    one side flushed an f32 subnormal: the values are equal once f32 subnormals are flushed to zero
  nan    both values are NaNs (only the payload or sign differs)
  ulp    the program uses an approximate function (rcp/rsqrt/sqrt/ex2/lg2/sin/cos/tanh .approx, div.approx/full)
         and the values are within --ulps ULPs (the distance is reported)
  real   anything else
Float interpretations are taken from the buffer's (or its .expect's) type, else from the float types the module
uses (most buffers are .u32 holding float bits); an element is an address match before anything else.
A program is SAME, or DIFF-<worst class> (real > ulp > nan > ftz), with its first differing elements, and the
.expect results of each side (which side matches the formal expectation).

The AIR service rules of air_ptx_formalization_check.py apply: one request at a time under the device lock, every
request named in --progress before it is sent, never a kernel of the hang list; a request that times out stops all
device work and is added to the hang list (it is never resent).

    python3 ptx/tools/air_agx_diff.py --tools /scratch/diff-tools --agx-run .../agx_run --direct \\
        --progress progress.txt --json diff.json PTX-formalization/tests/*.ptx
"""

import argparse
import base64
import json
import math
import os
import re
import struct
import sys
import tempfile
import threading
from concurrent.futures import ThreadPoolExecutor

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import agx_ptx_formalization_check as h  # noqa: E402
import air_ptx_formalization_check as air  # noqa: E402
from agx_ptx_formalization_check import SIZES, BASE, GLOBALS_BASE, Unsupported  # noqa: E402

APPROX = re.compile(r'\b(?:(?:rcp|rsqrt|sqrt|ex2|lg2|sin|cos|tanh)\.approx|div\.(?:approx|full))\b')
SEVERITY = ['ftz', 'nan', 'ulp', 'real']
# reply errors meaning the request did not finish on the Mac: never resent, device work stops
HANG_ERRORS = ('did not reply', 'timed out', 'timeout', 'time out')
STOP = threading.Event()
STOP_REASON = []
HANG_LOCK = threading.Lock()
SEND_LOCK = threading.Lock()


# ------------------------------------------------------------------ classification

FLOAT_FMT = {'f16': ('<e', '<H', 2), 'f32': ('<f', '<I', 4), 'f64': ('<d', '<Q', 8)}


def is_nan(bits, t):
    if t == 'bf16':
        return (bits & 0x7f80) == 0x7f80 and bits & 0x7f
    exp, man = {'f16': (0x7c00, 0x3ff), 'f32': (0x7f800000, 0x7fffff), 'f64': (0x7ff << 52, (1 << 52) - 1)}[t]
    return (bits & exp) == exp and bits & man


def ordered(bits, t):
    """The float encoding as an integer that is monotonic in the value (for ULP distances)."""
    n = {'f16': 16, 'bf16': 16, 'f32': 32, 'f64': 64}[t]
    sign = 1 << (n - 1)
    return -(bits & (sign - 1)) if bits & sign else bits


def ftz32(bits):
    return bits & 0x80000000 if (bits & 0x7f800000) == 0 else bits


def classify(a, b, size, ftypes, approx, max_ulps):
    """(class, ULP distance or None) of two differing element encodings."""
    interps = [t for t in ftypes if (SIZES[t] if t != 'bf16' else 2) == size]
    for t in interps:
        if is_nan(a, t) and is_nan(b, t):
            return 'nan', None
    for t in interps:
        if t == 'f32' and (ftz32(a) == ftz32(b) or (ftz32(a) | ftz32(b)) & 0x7fffffff == 0) \
                and (ftz32(a) != a or ftz32(b) != b):
            return 'ftz', None
    if approx:
        for t in interps:
            if is_nan(a, t) or is_nan(b, t):
                continue
            d = abs(ordered(a, t) - ordered(b, t))
            if d <= max_ulps:
                return 'ulp', d
    return 'real', None


def float_types(module):
    return [t for t in ('f32', 'f64', 'f16', 'bf16') if re.search(r'\.' + t + r'(x2)?\b', module)]


# ------------------------------------------------------------------ the sides

class AgxSide:
    """AGX code on agx_run: harness memory at BASE mirroring the AIR heap's layout, global data at GLOBALS_BASE."""

    def __init__(self, label, tmp, args, module, harness_syms):
        self.label, self.tmp, self.args = label, tmp, args
        self.kernels, gdata, gsyms, self.gsizes, relocs = h.kernel_info(
            None, os.path.join(tmp, 'm.o'), args.readelf, args.objcopy, tmp)
        self.goff = gsyms
        self.globals_mem = h.globals_image(gdata, gsyms, self.gsizes, relocs)
        self.syms = harness_syms  # shared dict; globals resolved by sym()
        self.gaddr = {n: GLOBALS_BASE + o for n, o in gsyms.items()}
        self.mem = h.Memory()
        self.error = None
        self.launches = 0
        self.address32 = bool(re.search(r'^\s*\.address_size\s+32\b', module, re.M))

    def all_syms(self):
        return {**self.syms, **self.gaddr}

    def write(self, addr, n, bits):
        if GLOBALS_BASE <= addr < GLOBALS_BASE + len(self.globals_mem):
            o = addr - GLOBALS_BASE
            self.globals_mem[o:o + n] = bits.to_bytes(n, 'little')
        else:
            self.mem.write(addr, n, bits)

    def read(self, addr, n):
        if GLOBALS_BASE <= addr < GLOBALS_BASE + len(self.globals_mem):
            o = addr - GLOBALS_BASE
            return int.from_bytes(self.globals_mem[o:o + n], 'little')
        return self.mem.read(addr, n)

    def addr(self, name):
        return self.gaddr[name] if name in self.gaddr else self.syms[name]

    def run(self, name, ps, ops, grid, block, dyn):
        if self.error:
            return
        if self.args.dry_run:
            return
        err = h.simulate(self.args, self.tmp, name, self.kernels[name], ps, ops, grid, block, dyn, self.all_syms(),
                         self.mem, self.globals_mem, 'k%d' % self.launches, self.address32)
        self.launches += 1
        if err:
            self.error = 'SIM-ERROR ' + err

    def canon(self, v, gnames):
        """An address value as (symbol, offset), or None."""
        if GLOBALS_BASE <= v < GLOBALS_BASE + len(self.globals_mem):
            o = v - GLOBALS_BASE
            best = max(((n, g) for n, g in self.goff.items() if g <= o and n in gnames), key=lambda x: x[1],
                       default=None)
            return ('G', best[0], o - best[1]) if best else None
        return None


def canon_harness(v, bufs):
    for n, (addr, t, count) in bufs.items():
        if addr <= v < addr + max(count * SIZES[t], 1):
            return ('B', n, v - addr)
    return None


# ------------------------------------------------------------------ one program

def run_program(path, args):
    base = os.path.basename(path)
    with open(path) as f:
        module, dirs = h.split_program(f.read())
    module = h.lower_coll_test(module)
    for d in dirs:
        if d.split()[0] in air.UNSUPPORTED_DIRECTIVES:
            return {'status': 'UNSUPPORTED', 'detail': 'harness directive ' + d.split()[0]}
    if re.search(r'^\s*\.address_size\s+32\b', module, re.M):
        return {'status': 'UNSUPPORTED', 'detail': '.address_size 32'}
    with tempfile.TemporaryDirectory() as tmp:
        tmp_air, tmp_direct = os.path.join(tmp, 'air'), os.path.join(tmp, 'direct')
        os.makedirs(tmp_air)
        status, info = air.compile_module(args, module, tmp_air)
        if status:
            return {'status': status, 'detail': info}
        res = {'status': None, 'notes': [], 'jobs': 0, 'pairs': {}}
        params = h.kernel_params(module)
        bodies = h.function_bodies(module)
        ftypes_module = float_types(module)

        heap = air.Heap()
        air_syms, bufs = {}, {}
        globals_off = air.load_globals(tmp_air, heap, air_syms, bufs)
        air_gnames = dict(bufs)  # global name -> [addr, 'b8', size] in the heap
        harness_syms = {}
        sides = [AgxSide('agx', tmp_air, args, module, harness_syms)]
        if args.direct:
            os.makedirs(tmp_direct)
            st, detail = h.compile_ptx(args, module, tmp_direct)
            if st:
                res['notes'].append('direct: %s %s' % (st, detail[:200]))
                direct_tainted = set()
            else:
                d = AgxSide('direct', tmp_direct, args, module, harness_syms)
                direct_tainted = detail[2]
                sides.append(d)
        gtypes = {}
        for m in re.finditer(r'\.(?:global|const)\s+(?:\.align\s+\d+\s+)?(?:\.v\d\s+)?\.(\w+)\s+([\w$]+)', module):
            if m.group(1) in SIZES:
                gtypes[m.group(2)] = m.group(1)
        expect_types = {}
        for d in dirs:
            m = re.match(r'\.expect ([\w$.]+) \.(\w+)', d)
            if m and m.group(2) in ('f16', 'bf16', 'f32', 'f64'):
                expect_types.setdefault(m.group(1), m.group(2))

        pending, job_kernels = [], []
        skipped_launches, tainted_bufs = [], set()
        expect = {s: [0, 0] for s in ['air'] + [x.label for x in sides]}
        expect_diffs = []
        tainted = info['tainted']
        state = {'stopped': None}

        def air_value(o, t):
            if o in air_syms:
                return air_syms[o], air_syms[o]
            return h.lit_bits(o, t, air_syms), None

        def ftypes_of(name):
            t = gtypes.get(name, 'b8') if name in air_gnames else bufs[name][1]
            if t in ('f16', 'bf16', 'f32', 'f64'):
                return [t]
            if name in expect_types:
                return [expect_types[name]]
            return ftypes_module

        def approx_used(kernels):
            seen, todo, text = set(), list(kernels), ''
            while todo:
                k = todo.pop()
                if k in seen or k not in bodies:
                    continue
                seen.add(k)
                text += bodies[k]
                todo += [f for f in bodies if f not in seen and re.search(r'\b' + re.escape(f) + r'\b', bodies[k])]
            return bool(APPROX.search(text))

        def regions():
            """name -> (kind, element type, AIR address, size) of every harness buffer and global variable."""
            out = {}
            for n, (addr, t, count) in bufs.items():
                if n in air_gnames:
                    t = gtypes.get(n, 'b8')
                    out[n] = ('global', t, addr, air_gnames[n][2])
                else:
                    out[n] = ('buffer', t, addr, max(count * SIZES[t], 0))
            return out

        def compare(dev_base, when):
            approx = approx_used(job_kernels)
            regs = regions()
            reads = {'air': lambda a, n: heap.read(a, n)}
            for s in sides:
                reads[s.label] = s.read
            cols = ['air'] + [s.label for s in sides if not s.error]
            pairs = [('air', 'agx')] + ([('air', 'direct'), ('agx', 'direct')] if 'direct' in cols else [])
            for p, q in pairs:
                if p not in cols or q not in cols:
                    continue
                key = '%s-%s' % (p, q)
                pr = res['pairs'].setdefault(key, {'classes': {}, 'first': [], 'max_ulps': 0, 'elements': 0})
                for n, (kind, t, aaddr, size) in regs.items():
                    esz = SIZES[t] if t != 'bf16' else 2
                    if size == 0:
                        continue

                    def addr_of(side):
                        if side == 'air':
                            return aaddr, size
                        s = next(x for x in sides if x.label == side)
                        if kind == 'global':
                            if n not in s.goff:
                                return None, 0
                            return s.gaddr[n], min(size, s.gsizes.get(n, size))
                        return aaddr, size
                    pa, psz = addr_of(p)
                    qa, qsz = addr_of(q)
                    if pa is None or qa is None:
                        continue
                    nbytes = min(psz, qsz)
                    if kind == 'global' and nbytes % esz:
                        esz = 1
                    for i in range(nbytes // esz):
                        x, y = reads[p](pa + i * esz, esz), reads[q](qa + i * esz, esz)
                        if x == y:
                            continue
                        if esz == 8 and canon(p, x) is not None and canon(p, x) == canon(q, y):
                            continue  # the same address
                        cls, ulps = classify(x, y, esz, ftypes_of(n), approx, args.ulps)
                        pr['classes'][cls] = pr['classes'].get(cls, 0) + 1
                        pr['elements'] += 1
                        if ulps is not None:
                            pr['max_ulps'] = max(pr['max_ulps'], ulps)
                        if len(pr['first']) < args.max_diffs:
                            pr['first'].append('%s %s[%d] %s=%#x %s=%#x %s%s' % (
                                when, n, i, p, x, q, y, cls, '(%d ulp)' % ulps if ulps is not None else ''))

        def canon(side, v):
            if side == 'air':
                if state.get('dev_base') is not None and state['dev_base'] <= v < state['dev_base'] + len(heap.buf):
                    v = BASE + (v - state['dev_base'])
                for n, (addr, _, size) in air_gnames.items():
                    if addr <= v < addr + size:
                        return ('G', n, v - addr)
                return canon_harness(v, {n: b for n, b in bufs.items() if n not in air_gnames})
            s = next(x for x in sides if x.label == side)
            return s.canon(v, air_gnames) or canon_harness(v, {n: b for n, b in bufs.items() if n not in air_gnames})

        def flush(when):
            """Run the pending launches on the device, then compare. Returns a final status or None."""
            if not pending:
                return None
            job = air.make_job(info, heap, pending, globals_off)
            pending.clear()
            hung = air.hung_kernels(path, job, args.skip_hang)
            if hung:
                return 'SKIPPED-HANG', ','.join(hung)
            if STOP.is_set():
                return 'NOT-RUN', 'device work stopped: ' + '; '.join(STOP_REASON)
            # the device address of the heap: a probe buffer holding the address of heap offset 0
            probe = len(job['buffers'])
            job['buffers'].append({'size': 16, 'data': air.b64(bytes(16))})
            job.setdefault('address_patches', []).append(
                {'buffer': probe, 'offset': 0, 'target_buffer': 0, 'target_offset': 0})
            job['readback'] = [0, probe]
            res['jobs'] += 1
            with SEND_LOCK:
                # my worker threads send one at a time, and none after a request hung
                if STOP.is_set():
                    return 'NOT-RUN', 'device work stopped: ' + '; '.join(STOP_REASON)
                air.log_progress(args.progress, path, job)
                if args.dry_run:
                    return None
                reply = air.submit(job, args.queue, transient=('XPC_ERROR_CONNECTION_INTERRUPTED',))
                if not reply.get('ok') and any(e in reply.get('error', '').lower() for e in HANG_ERRORS):
                    STOP.set()
            if not reply.get('ok'):
                err = reply.get('error', 'service error')
                if any(e in err.lower() for e in HANG_ERRORS):
                    STOP.set()
                    kernels = [c['kernel'] for c in job['commands']]
                    STOP_REASON.append('%s %s: %s' % (base, ','.join(kernels), err[:200]))
                    with HANG_LOCK, open(args.skip_hang_file, 'a') as f:
                        f.write('# %s: air_agx_diff.py request timed out: %s\n' % (
                            __import__('datetime').date.today(), err[:120].replace('\n', ' ')))
                        for k in kernels:
                            f.write('%s %s\n' % (base, k))
                    return 'HANG', '%s: %s' % (','.join(kernels), err[:300])
                return 'AIR-ERROR', err[:300]
            air.read_heap(reply, heap)
            pb = next((b for b in reply['result']['buffers'] if b['buffer'] == probe), None)
            state['dev_base'] = int.from_bytes(base64.b64decode(pb['data'])[:8], 'little') if pb else None
            compare(state['dev_base'], when)
            job_kernels.clear()
            return None

        def finish(st, detail=''):
            res['status'], res['detail'] = st, detail
            return res

        for di, d in enumerate(dirs):
            kw = d.split()[0]
            if kw in h.IGNORED:
                continue
            if kw == '.buffer':
                m = re.fullmatch(r'\.buffer ([\w$.]+) \.(\w+)\s*(\[\s*(\d+)\s*\])?\s*(=\s*\{(.*)\})?', d)
                if not m:
                    raise Unsupported(d)
                t = h.ty_of(m.group(2))
                vals = h.split_vals(m.group(6)) if m.group(6) is not None else []
                count = int(m.group(4)) if m.group(4) else len(vals)
                addr = BASE + ((heap.top + 255) & ~255)
                heap.top = (addr - BASE) + max(count * SIZES[t], 1)
                heap.ensure(heap.top)
                air_syms[m.group(1)] = addr
                harness_syms[m.group(1)] = addr
                bufs[m.group(1)] = [addr, t, count]
                for s in sides:
                    s.mem.ensure(heap.top)
                for k, v in enumerate(vals):
                    bits, target = air_value(v, t)
                    heap.write(addr + k * SIZES[t], SIZES[t], bits, target)
                    for s in sides:
                        s.write(addr + k * SIZES[t], SIZES[t], h.lit_bits(v, t, s.all_syms()))
            elif kw in ('.patch', '.append'):
                st = flush('before #%d %s' % (di, kw))
                if st:
                    return finish(*st)
                m = re.fullmatch(r'\.(patch|append) ([\w$.]+) \.(\w+)\s*(@\s*(\d+))?\s*=\s*\{(.*)\}', d)
                t = h.ty_of(m.group(3))
                vals = h.split_vals(m.group(6))
                name = m.group(2)
                if m.group(1) == 'patch':
                    off = int(m.group(5) or 0) * SIZES[t]
                    abase = air_syms[name] + off
                    sbase = [s.addr(name) + off for s in sides]
                else:
                    b = bufs[name]
                    abase = b[0] + b[2] * SIZES[b[1]]
                    sbase = [abase for _ in sides]
                    b[2] += len(vals)
                    heap.top = max(heap.top, abase - BASE + len(vals) * SIZES[t])
                for k, v in enumerate(vals):
                    bits, target = air_value(v, t)
                    heap.write(abase + k * SIZES[t], SIZES[t], bits, target)
                    for s, sb in zip(sides, sbase):
                        s.write(sb + k * SIZES[t], SIZES[t], h.lit_bits(v, t, s.all_syms()))
            elif kw == '.launch':
                m = re.fullmatch(r'\.launch ([\w$.]+) <<<\s*(\([^)]*\)|\d+)\s*,\s*(\([^)]*\)|\d+)\s*(,\s*(\d+))?\s*>>>'
                                 r'\s*\((.*)\)', d)
                if not m:
                    raise Unsupported(d)
                name = m.group(1)
                ops = [o.strip() for o in m.group(6).split(',') if o.strip()]
                agx = info['agx']
                if tainted is None or name in tainted or name not in info['kernels'] or name not in agx:
                    # not compiled on the AIR path: skipped on every side, so that they stay comparable
                    skipped_launches.append(name)
                    tainted_bufs.update(ops)
                    continue
                if any(o in tainted_bufs for o in ops):
                    tainted_bufs.update(ops)
                if list(agx[name]['cluster']) != [1, 1, 1]:
                    raise Unsupported('cluster launch')
                grid, block = h.parse_dim(m.group(2)), h.parse_dim(m.group(3))
                dyn = int(m.group(5)) if m.group(5) else 0
                ps = params.get(name, [])
                pbufs = air.launch_buffers(info, name, ps, ops, dyn, air_value, air_syms)
                pending.append((name, grid, block, dyn, pbufs))
                job_kernels.append(name)
                for s in sides:
                    if s.label == 'direct' and (direct_tainted is None or name in direct_tainted
                                                or name not in s.kernels):
                        if not s.error:
                            s.error = 'kernel %s not compiled on the direct path' % name
                        continue
                    s.run(name, ps, ops, grid, block, dyn)
                if args.per_launch:
                    st = flush('after #%d %s' % (di, name))
                    if st:
                        return finish(*st)
            elif kw == '.expect':
                st = flush('before #%d .expect' % di)
                if st:
                    return finish(*st)
                m = re.fullmatch(r'\.expect ([\w$.]+) \.(\w+)\s*(@\s*(\d+))?\s*=\s*\{(.*)\}', d)
                t = h.ty_of(m.group(2))
                name = m.group(1)
                if name not in bufs or (skipped_launches and name in tainted_bufs) or args.dry_run:
                    continue
                off = int(m.group(4)) if m.group(4) else 0
                vals = h.split_vals(m.group(5))
                getters = [('air', lambda a, n: heap.read(air_syms[name] + a, n))]
                for s in sides:
                    if not s.error and (name not in air_gnames or name in s.gaddr):
                        getters.append((s.label, lambda a, n, s=s: s.read(s.addr(name) + a, n)))
                ok = {lab: True for lab, _ in getters}
                for i, v in enumerate(vals):
                    want = h.lit_bits(v, t, air_syms)
                    got = {lab: g((off + i) * SIZES[t], SIZES[t]) for lab, g in getters}
                    for lab, x in got.items():
                        if x != want:
                            ok[lab] = False
                    if len(set(got.values())) > 1 and len(expect_diffs) < args.max_diffs:
                        expect_diffs.append('%s[%d] expect %#x: %s' % (name, off + i, want, ' '.join(
                            '%s=%#x%s' % (lab, x, '(ok)' if x == want else '') for lab, x in got.items())))
                for lab, good in ok.items():
                    expect[lab][0 if good else 1] += 1
        st = flush('at end')
        if st:
            return finish(*st)
        for s in sides:
            if s.error:
                res['notes'].append('%s: %s' % (s.label, s.error))
        if skipped_launches:
            res['notes'].append('skipped launches (not compiled on the AIR path): ' +
                                ','.join(sorted(set(skipped_launches))))
        res['expect'] = {lab: '%d pass, %d fail' % tuple(v) for lab, v in expect.items()}
        if expect_diffs:
            res['expect_diffs'] = expect_diffs
        ab = res['pairs'].get('air-agx')
        if args.dry_run:
            return finish('DRY', '%d job(s)' % res['jobs'])
        if any(s.error and s.label == 'agx' for s in sides):
            return finish('AGX-ERROR', sides[0].error)
        if not res['jobs']:
            return finish('NO-LAUNCH', 'no launch ran')
        if not ab or not ab['classes']:
            return finish('SAME', '%d job(s)' % res['jobs'])
        worst = max(ab['classes'], key=SEVERITY.index)
        return finish('DIFF-' + worst, ', '.join('%s %d' % kv for kv in sorted(ab['classes'].items())))


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('tests', nargs='+', help='PTX-formalization test programs (tests/*.ptx)')
    ap.add_argument('--tools', help='directory of zoc, llc, llvm-dis, llvm-readelf, llvm-objcopy')
    ap.add_argument('--zoc', default='zoc')
    ap.add_argument('--llc', default='llc', help='llc with the AIR target')
    ap.add_argument('--llvm-dis', dest='llvm_dis', default='llvm-dis')
    ap.add_argument('--readelf', default='llvm-readelf')
    ap.add_argument('--objcopy', default='llvm-objcopy')
    ap.add_argument('--agx-run', dest='agx_run', default='agx_run')
    ap.add_argument('--arch', default='agx-g14g')
    ap.add_argument('--queue', default=os.environ.get('AIR_QUEUE', '/air'))
    ap.add_argument('--timeout', type=int, default=120)
    ap.add_argument('--direct', action='store_true', help='also run the direct PTX -> AGX code (a third column)')
    ap.add_argument('--per-launch', action='store_true', help='one service job, and one comparison, per launch')
    ap.add_argument('--ulps', type=int, default=16, help='largest ULP distance of class ulp')
    ap.add_argument('--max-diffs', type=int, default=4, help='differing elements listed per pair')
    ap.add_argument('-j', type=int, default=2, help='programs compiled / simulated at once (at most 2)')
    ap.add_argument('--json', help='write the results to this file')
    ap.add_argument('--progress', help="append each request's test and kernels to this file before sending it")
    ap.add_argument('--skip-hang', default=os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                                         'air_ptx_formalization_hangs.txt'),
                    help='hang list ("TEST KERNEL" lines): never sent; a request that times out is added to it')
    ap.add_argument('--dry-run', action='store_true',
                    help='compile and build every request (logged to --progress) without sending or simulating')
    args = ap.parse_args()
    if args.tools:
        for attr, tool in (('zoc', 'zoc'), ('llc', 'llc'), ('llvm_dis', 'llvm-dis'), ('readelf', 'llvm-readelf'),
                           ('objcopy', 'llvm-objcopy')):
            setattr(args, attr, os.path.join(args.tools, tool))
    args.j = max(1, min(args.j, 2))
    args.skip_hang_file = args.skip_hang
    args.skip_hang = air.load_hang_list(args.skip_hang)

    def one(p):
        if STOP.is_set():
            return p, {'status': 'NOT-RUN', 'detail': 'device work stopped: ' + '; '.join(STOP_REASON)}
        try:
            r = run_program(p, args)
        except Unsupported as e:
            r = {'status': 'UNSUPPORTED', 'detail': str(e)}
        except Exception as e:
            r = {'status': 'HARNESS-ERROR', 'detail': '%s: %s' % (type(e).__name__, e)}
        line = '%-14s %-36s %s' % (r['status'], os.path.basename(p), r.get('detail', ''))
        for key, pr in sorted(r.get('pairs', {}).items()):
            if pr['classes']:
                line += '\n    %s: %s%s' % (key, ', '.join('%s %d' % kv for kv in sorted(pr['classes'].items())),
                                           ' max %d ulp' % pr['max_ulps'] if pr['max_ulps'] else '')
                line += ''.join('\n      ' + x for x in pr['first'])
        for x in r.get('expect_diffs', []):
            line += '\n    ' + x
        for x in r.get('notes', []):
            line += '\n    note: ' + x
        print(line, flush=True)
        return p, r

    with ThreadPoolExecutor(args.j) as ex:
        results = list(ex.map(one, sorted(args.tests)))
    counts = {}
    for _, r in results:
        counts[r['status']] = counts.get(r['status'], 0) + 1
    print('---- programs:', ', '.join('%d %s' % (v, k) for k, v in sorted(counts.items())))
    if STOP.is_set():
        print('---- DEVICE WORK STOPPED: ' + '; '.join(STOP_REASON))
    if args.json:
        with open(args.json, 'w') as f:
            json.dump({os.path.basename(p): r for p, r in results}, f, indent=1)
    return 2 if STOP.is_set() else 0


if __name__ == '__main__':
    sys.exit(main())
