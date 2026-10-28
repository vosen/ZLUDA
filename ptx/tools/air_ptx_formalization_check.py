#!/usr/bin/env python3
"""Check ZLUDA's PTX -> AIR path on a real Apple GPU against the PTX-formalization test programs.

Like agx_ptx_formalization_check.py, but instead of running AGX code on the simulator, each module is compiled to
upstream AIR (`ZLUDA_AGX_VIA_AIR=1 zoc --target agx`, which writes NAME.air), packaged for Metal by the AIR target
(`llc -mtriple=air -filetype=obj -air-output=air`) and run on the GPU behind the AIR service (/air, a JSON file
queue; see /air/client/client.py). The .expect directives are checked bit-exactly, like `krun` does.

Memory: the harness buffers live in one device buffer (the heap), laid out as in agx_ptx_formalization_check.py
from BASE; every value that is the address of a harness symbol (a pointer kernel argument, a pointer in a buffer
initializer or .patch) is written by the service as an address patch, so the kernel sees real device addresses.
Kernel parameters are constant buffers, one per parameter, bound at their AIR location index. Consecutive launches
run as one service job (in order, on the same buffers), so module-scope .global variables keep their values between
them; a job boundary (any host-side directive between launches) starts them again from their initializers.

Module-scope .global variables are one buffer in the .air (zoc writes NAME.air.globals: its size, initial image,
relocations and symbols); it is placed in the heap like a harness buffer, its relocations written as address patches,
and bound at each kernel's zluda_globals argument, so .expect / .patch of globals work and globals keep their values
between launches.

Not supported here (reported UNSUPPORTED): textures, surfaces, samplers, tensor maps, multimem, fabric, clusters and
.address_size 32 modules.

    python3 ptx/tools/air_ptx_formalization_check.py --zoc target/debug/zoc --llc $LLVM/bin/llc \\
        --llvm-dis $LLVM/bin/llvm-dis PTX-formalization/tests/*.ptx
"""

import argparse
import base64
import json
import os
import re
import subprocess
import sys
import tempfile
from concurrent.futures import ThreadPoolExecutor

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import agx_ptx_formalization_check as h  # noqa: E402
from agx_ptx_formalization_check import SIZES, Unsupported  # noqa: E402

BASE = h.BASE
UNSUPPORTED_DIRECTIVES = ('.tensormap', '.texture', '.sampler', '.surface', '.multimem', '.fabric', '.cooplaunch')
TRANSIENT = ('XPC_ERROR_CONNECTION_INTERRUPTED', 'did not reply')


# Every tool sending requests to the AIR service takes this lock around each one, so that requests never overlap (a
# request the Mac never finishes freezes its display; two at once make that worse and hide which one it was).
DEVICE_LOCK = os.environ.get('AIR_DEVICE_LOCK', '/scratch/air-device.lock')


# Per-request limits the service enforces (capabilities: gpu_timeout_ms, compile_timeout_ms). A kernel that runs past
# GPU_TIMEOUT_MS is reported (and the service refuses GPU work until it ends) instead of freezing the Mac's display.
GPU_TIMEOUT_MS = 2000
COMPILE_TIMEOUT_MS = 5000
# Error texts of a request the service gave up on: a hang to record, never to retry.
TIMED_OUT = ('gpu_timeout_ms=', 'compile_timeout_ms=', 'Refusing GPU work', 'timed out after')


def is_hang(error):
    return any(t in (error or '') for t in TIMED_OUT)


def submit(job, queue, tries=4, transient=TRANSIENT):
    """Send one request to the service holding the device lock, retrying the failures listed in transient. A request
    that timed out is not retried: the lock is held until the service has finished the timed-out kernel."""
    import fcntl
    import time
    sys.path.insert(0, os.environ.get('AIR_CLIENT', '/air/client'))
    from client import submit as service_submit
    if job.get('op') == 'execute':
        job.setdefault('gpu_timeout_ms', GPU_TIMEOUT_MS)
        job.setdefault('compile_timeout_ms', COMPILE_TIMEOUT_MS)
    reply = {}
    with open(DEVICE_LOCK, 'a') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        for _ in range(tries):
            try:
                reply = service_submit(job, queue, timeout=180)
            except TimeoutError as e:
                reply = {'ok': False, 'error': str(e)}
            if is_hang(reply.get('error')):
                deadline = time.monotonic() + 60
                while time.monotonic() < deadline:
                    caps = service_submit({'op': 'capabilities'}, queue, timeout=60)
                    if not (caps.get('result') or {}).get('gpu_busy_with_timed_out_kernel'):
                        break
                    time.sleep(0.5)
                break
            if reply.get('ok') or not any(t in reply.get('error', '') for t in transient):
                break
    return reply


def b64(data):
    return base64.b64encode(bytes(data)).decode()


def split_params(sig):
    """The parameters of a define line's parameter list, split at top-level commas."""
    out, depth, cur = [], 0, ''
    for c in sig:
        if c in '([{<':
            depth += 1
        elif c in ')]}>':
            depth -= 1
        if c == ',' and depth == 0:
            out.append(cur.strip())
            cur = ''
        else:
            cur += c
    if cur.strip():
        out.append(cur.strip())
    return out


def air_kernels(text):
    """Kernel name -> list of argument descriptions from a disassembled AIR module's !air.kernel metadata:
    ('buffer', location index, size) / ('threadgroup', location index) / ('system', name)."""
    nodes = {}
    for m in re.finditer(r'^!(\d+) = (?:distinct )?!\{(.*)\}$', text, re.M):
        nodes[m.group(1)] = m.group(2)
    km = re.search(r'^!air\.kernel = !\{(.*)\}$', text, re.M)
    kernels = {}
    if not km:
        return kernels
    for ref in re.findall(r'!(\d+)', km.group(1)):
        node = nodes[ref]
        name = re.match(r'ptr @"?([^",]+)"?', node).group(1)
        arg_list = re.findall(r'!(\d+)', node)[-1]
        args = []
        for a in re.findall(r'!(\d+)', nodes[arg_list]):
            n = nodes[a]
            loc = re.search(r'!"air\.location_index", i32 (\d+)', n)
            size = re.search(r'!"air\.arg_type_size", i32 (\d+)', n)
            if '"air.buffer"' in n and '!"air.arg_name", !"zluda_globals"' in n:
                # the module's .global variables (NAME.air.globals)
                args.append(('globals', int(loc.group(1))))
            elif '"air.buffer"' in n and '!"air.address_space", i32 3' in n:
                # dynamic shared memory: a threadgroup buffer whose length the dispatch sets
                args.append(('threadgroup', int(loc.group(1)) if loc else 0))
            elif '"air.buffer"' in n:
                args.append(('buffer', int(loc.group(1)), int(size.group(1)) if size else 8))
            elif '"air.threadgroup"' in n:
                args.append(('threadgroup', int(loc.group(1)) if loc else 0))
            else:
                args.append(('system', re.search(r'!"(air\.[\w.]+)"', n).group(1)))
        kernels[name] = args
    return kernels


def hidden_roles(text):
    """Kernel name -> list (per parameter) of its "agx-hidden" role or None, from the portable AIR."""
    out = {}
    for m in re.finditer(r'^define [^@]*@"?([^"(]+)"?\((.*)\)[^)]*\{$', text, re.M):
        out[m.group(1)] = [(re.search(r'"agx-hidden"="([^"]+)"', p) or [None, None])[1]
                           for p in split_params(m.group(2))]
    return out


class Heap:
    """The harness memory from BASE, with the address patches of the values that are symbol addresses."""

    def __init__(self):
        self.buf = bytearray()
        self.top = 0
        self.patches = {}  # heap offset -> target heap offset (an 8-byte address)

    def ensure(self, end):
        if len(self.buf) < end:
            self.buf.extend(b'\0' * (end - len(self.buf)))

    def write(self, addr, n, bits, target=None):
        o = addr - BASE
        self.ensure(o + n)
        self.buf[o:o + n] = (bits & ((1 << (8 * n)) - 1)).to_bytes(n, 'little')
        for k in range(o - 7, o + n):
            self.patches.pop(k, None)
        if target is not None:
            if n != 8:
                raise Unsupported('a symbol address stored in %d bytes' % n)
            self.patches[o] = target - BASE

    def read(self, addr, n):
        o = addr - BASE
        self.ensure(o + n)
        return int.from_bytes(self.buf[o:o + n], 'little')


def load_hang_list(path):
    """The (test, kernel) pairs of a hang list file ("TEST KERNEL" lines, # comments)."""
    out = set()
    if path and os.path.exists(path):
        for line in open(path):
            line = line.split('#')[0].split()
            if len(line) == 2:
                out.add(tuple(line))
    return out


def compile_module(args, module, tmp):
    """zoc with ZLUDA_AGX_VIA_AIR=1 (NAME.air, and the AGX code compiled from it), then Metal's AIR (llc).
    Returns (status, detail) of a failure, or (None, info): info has 'air' (Metal AIR bytes), 'kernels'
    (air_kernels), 'roles' (hidden_roles), 'agx' (kernel_info's kernels of m.o), 'tainted', 'rejected'."""
    env = dict(os.environ, ZLUDA_AGX_VIA_AIR='1')
    status, detail = h.compile_ptx(args, module, tmp, env=env, product='m.air')
    if status:
        return status, detail
    _, rejected, tainted = detail
    air = os.path.join(tmp, 'm.air')
    # Metal's AIR, from the portable AIR zoc wrote.
    metal = os.path.join(tmp, 'm.metal.air')
    r = subprocess.run([args.llc, '-mtriple=air', '-filetype=obj', '-air-output=air', air, '-o', metal],
                       capture_output=True, text=True, timeout=args.timeout)
    if r.returncode != 0:
        return 'LLC-ERROR', (r.stderr.strip().splitlines() or ['?'])[-1][:300]
    dis = lambda p: subprocess.run([args.llvm_dis, p, '-o', '-'], capture_output=True, text=True,
                                   timeout=args.timeout).stdout
    portable, metal_text = dis(air), dis(metal)
    # static threadgroup bytes and the cluster shape, from the AGX compilation of the same AIR
    with open(os.path.join(tmp, 'm.asm')) as f:
        asm = f.read()
    agx, _, gsyms, _, _ = h.kernel_info(asm, os.path.join(tmp, 'm.o'), args.readelf, args.objcopy, tmp)
    return None, {'air': open(metal, 'rb').read(), 'kernels': air_kernels(metal_text),
                  'roles': hidden_roles(portable), 'agx': agx, 'tainted': tainted, 'rejected': rejected}


def load_globals(tmp, heap, syms, bufs):
    """Place the module's .global variables (NAME.air.globals) at the start of the heap; their symbols go to syms
    and bufs. Returns the heap offset of the globals buffer, or None for a module without one."""
    globals_desc = os.path.join(tmp, 'm.air.globals')
    if not os.path.exists(globals_desc):
        return None
    g = json.load(open(globals_desc))
    heap.top = max(g['size'], 1)
    heap.ensure(heap.top)
    heap.buf[:len(bytes.fromhex(g['image']))] = bytes.fromhex(g['image'])
    for off, target in g['relocs']:
        heap.patches[off] = target
    for name, off in g['symbols'].items():
        syms[name] = BASE + off
    # the kernel writing the mask() initializers from the buffer's address, which each job's heap has anew
    heap.init_kernel = g.get('init_kernel') or None
    ordered = sorted(g['symbols'].items(), key=lambda kv: kv[1])
    for i, (name, off) in enumerate(ordered):
        end = ordered[i + 1][1] if i + 1 < len(ordered) else g['size']
        bufs[name] = [BASE + off, 'b8', end - off]
    return 0


def launch_buffers(info, name, ps, ops, dyn, value, syms):
    """The parameter buffers of a launch of kernel `name`: (location index, bytes, [(offset, heap target)])."""
    kernels, roles, agx = info['kernels'], info['roles'], info['agx']
    if len(ops) != len(ps):
        raise Unsupported('launch argument count')
    buf_args = [a for a in kernels[name] if a[0] == 'buffer']
    kroles = roles.get(name, [])
    pbufs = []
    k = 0
    for i, a in enumerate(buf_args):
        role = kroles[i] if i < len(kroles) else None
        if role == 'threadgroup-bytes':
            data = (agx[name]['shared'] + dyn).to_bytes(4, 'little')
            pbufs.append((a[1], data + b'\0' * 12, []))
            continue
        if role is not None:
            raise Unsupported('hidden argument ' + role)
        if k >= len(ps):
            raise Unsupported('more AIR buffers than PTX parameters')
        (t, n, _), o = ps[k], ops[k]
        k += 1
        size = SIZES[t] * n
        target = None
        if n != 1:
            bits = (syms[o] if o in syms else int(o, 0)) & ((1 << (8 * size)) - 1)
            target = syms.get(o) if size == 8 else None
        else:
            bits, target = value(o, t)
        data = bytearray(bits.to_bytes(size, 'little'))
        data += b'\0' * ((-len(data)) % 16 or (16 if not data else 0))
        pbufs.append((a[1], data, [(0, target - BASE)] if target is not None else []))
    if k != len(ps):
        raise Unsupported('fewer AIR buffers than PTX parameters')
    return pbufs


def make_job(info, heap, pending, globals_off):
    """The service job running the pending launches ((kernel, grid, block, dyn, launch_buffers)) in order on the
    heap (buffer 0, read back)."""
    kernels = info['kernels']
    heap.ensure(max(heap.top, 16))
    buffers = [{'size': len(heap.buf), 'data': b64(heap.buf)}]
    patches = [{'buffer': 0, 'offset': o, 'target_buffer': 0, 'target_offset': t}
               for o, t in sorted(heap.patches.items())]
    commands = []
    init = getattr(heap, 'init_kernel', None)
    if init:
        loc = [a[1] for a in kernels[init] if a[0] == 'globals'][0]
        commands.append({'kernel': init, 'bindings': [{'index': loc, 'buffer': 0, 'offset': globals_off}],
                         'grid': [1, 1, 1], 'group': [1, 1, 1]})
    for name, grid, block, dyn, pbufs in pending:
        bindings = []
        for loc, data, ppatches in pbufs:
            bindings.append({'index': loc, 'buffer': len(buffers)})
            for off, t in ppatches:
                patches.append({'buffer': len(buffers), 'offset': off, 'target_buffer': 0,
                                'target_offset': t})
            buffers.append({'size': len(data), 'data': b64(data)})
        for a in kernels[name]:
            if a[0] == 'globals':
                bindings.append({'index': a[1], 'buffer': 0, 'offset': globals_off})
        cmd = {'kernel': name, 'bindings': bindings,
               'grid': [grid[j] * block[j] for j in range(3)], 'group': list(block)}
        tg = [a for a in kernels[name] if a[0] == 'threadgroup']
        if tg:
            cmd['threadgroup_memory'] = [{'index': tg[0][1], 'length': max(16, (dyn + 15) // 16 * 16)}]
        commands.append(cmd)
    job = {'op': 'execute', 'air': b64(info['air']), 'buffers': buffers, 'commands': commands, 'readback': [0]}
    if patches:
        job['address_patches'] = patches
    return job


def hung_kernels(path, job, skip_hang):
    """The kernels of job on the hang list (the job must not be sent)."""
    return [c['kernel'] for c in job['commands'] if (os.path.basename(path), c['kernel']) in skip_hang]


def log_progress(progress, path, job):
    """Before a request: name it in the progress file, so that a request that hangs the Mac is known."""
    if progress:
        with open(progress, 'a') as f:
            f.write('%s %s\n' % (os.path.basename(path), ','.join(c['kernel'] for c in job['commands'])))
            f.flush()
            os.fsync(f.fileno())


def read_heap(reply, heap):
    """The heap read back by a job, with the patched addresses back at their symbolic values."""
    got = bytearray(base64.b64decode(next(b for b in reply['result']['buffers'] if b['buffer'] == 0)['data']))
    for o, t in heap.patches.items():
        got[o:o + 8] = (BASE + t).to_bytes(8, 'little')
    heap.buf[:] = got[:len(heap.buf)]


def run_test(path, args):
    with open(path) as f:
        module, dirs = h.split_program(f.read())
    module = h.lower_coll_test(module)
    for d in dirs:
        if d.split()[0] in UNSUPPORTED_DIRECTIVES:
            return 'UNSUPPORTED', 'harness directive ' + d.split()[0]
    if re.search(r'^\s*\.address_size\s+32\b', module, re.M):
        return 'UNSUPPORTED', '.address_size 32'
    with tempfile.TemporaryDirectory() as tmp:
        status, info = compile_module(args, module, tmp)
        if status:
            return status, info
        kernels, agx, tainted = info['kernels'], info['agx'], info['tainted']
        params = h.kernel_params(module)

        heap = Heap()
        syms, bufs = {}, {}
        globals_off = load_globals(tmp, heap, syms, bufs)
        failures, passed, skipped, unsupported = [], 0, 0, []
        skipped_launches, tainted_bufs = [], set()
        pending = []  # launches of the next job: (kernel, grid, block, dyn, launch_buffers)
        jobs = 0

        def value(o, t):
            """A literal or symbol at type t: (bits, heap target or None)."""
            if o in syms:
                return syms[o], syms[o]
            return h.lit_bits(o, t, syms), None

        def flush():
            nonlocal jobs
            if not pending:
                return None
            job = make_job(info, heap, pending, globals_off)
            pending.clear()
            hung = hung_kernels(path, job, args.skip_hang)
            if hung:
                return 'SKIPPED-HANG: ' + ','.join(hung)
            log_progress(args.progress, path, job)
            reply = submit(job, args.queue)
            jobs += 1
            if not reply.get('ok'):
                return reply.get('error', 'service error')[:300]
            read_heap(reply, heap)
            return None

        for d in dirs:
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
                syms[m.group(1)] = addr
                bufs[m.group(1)] = [addr, t, count]
                for k, v in enumerate(vals):
                    bits, target = value(v, t)
                    heap.write(addr + k * SIZES[t], SIZES[t], bits, target)
            elif kw in ('.patch', '.append'):
                err = flush()
                if err:
                    return ('SKIPPED-HANG', err[14:]) if err.startswith('SKIPPED-HANG') else ('HANG' if is_hang(err) else 'GPU-ERROR', err)
                m = re.fullmatch(r'\.(patch|append) ([\w$.]+) \.(\w+)\s*(@\s*(\d+))?\s*=\s*\{(.*)\}', d)
                t = h.ty_of(m.group(3))
                vals = h.split_vals(m.group(6))
                name = m.group(2)
                if m.group(1) == 'patch':
                    base = syms[name] + int(m.group(5) or 0) * SIZES[t]
                else:
                    b = bufs[name]
                    base = b[0] + b[2] * SIZES[b[1]]
                    b[2] += len(vals)
                    heap.top = max(heap.top, base - BASE + len(vals) * SIZES[t])
                for k, v in enumerate(vals):
                    bits, target = value(v, t)
                    heap.write(base + k * SIZES[t], SIZES[t], bits, target)
            elif kw == '.launch':
                m = re.fullmatch(r'\.launch ([\w$.]+) <<<\s*(\([^)]*\)|\d+)\s*,\s*(\([^)]*\)|\d+)\s*(,\s*(\d+))?\s*>>>'
                                 r'\s*\((.*)\)', d)
                if not m:
                    raise Unsupported(d)
                name = m.group(1)
                ops = [o.strip() for o in m.group(6).split(',') if o.strip()]
                if tainted is None or name in tainted or name not in kernels or name not in agx:
                    skipped_launches.append(name)
                    tainted_bufs.update(ops)
                    continue
                if any(o in tainted_bufs for o in ops):
                    tainted_bufs.update(ops)
                if agx[name]['cluster'] != (1, 1, 1) and list(agx[name]['cluster']) != [1, 1, 1]:
                    raise Unsupported('cluster launch')
                grid, block = h.parse_dim(m.group(2)), h.parse_dim(m.group(3))
                dyn = int(m.group(5)) if m.group(5) else 0
                ps = params.get(name, [])
                pbufs = launch_buffers(info, name, ps, ops, dyn, value, syms)
                pending.append((name, grid, block, dyn, pbufs))
            elif kw == '.expect':
                err = flush()
                if err:
                    return ('SKIPPED-HANG', err[14:]) if err.startswith('SKIPPED-HANG') else ('HANG' if is_hang(err) else 'GPU-ERROR', err)
                m = re.fullmatch(r'\.expect ([\w$.]+) \.(\w+)\s*(@\s*(\d+))?\s*=\s*\{(.*)\}', d)
                t = h.ty_of(m.group(2))
                name = m.group(1)
                if name not in bufs:
                    unsupported.append(name)
                    skipped += 1
                    continue
                off = int(m.group(4)) if m.group(4) else 0
                vals = h.split_vals(m.group(5))
                if skipped_launches and name in tainted_bufs:
                    skipped += 1
                    continue
                bad = []
                for i, v in enumerate(vals):
                    want = h.lit_bits(v, t, syms)
                    got = heap.read(syms[name] + (off + i) * SIZES[t], SIZES[t])
                    if want != got:
                        bad.append('%s[%d] expected %#x, got %#x' % (name, off + i, want, got))
                if bad:
                    failures += bad
                    args.counts['fail'] += 1
                else:
                    passed += 1
        err = flush()
        if err:
            return ('SKIPPED-HANG', err[14:]) if err.startswith('SKIPPED-HANG') else ('HANG' if is_hang(err) else 'GPU-ERROR', err)
        args.counts['pass'] += passed
        args.counts['skip'] += skipped
        if failures:
            n = args.max_failures
            return 'FAIL', '; '.join(failures[:n]) + (' (+%d)' % (len(failures) - n) if len(failures) > n else '')
        if not passed and unsupported:
            raise Unsupported('.expect of module-scope .global ' + ', '.join(sorted(set(unsupported))))
        if skipped_launches or unsupported:
            return 'PARTIAL', '%d check(s) pass, %d skipped (rejected kernels %s; globals %s)' % (
                passed, skipped, ','.join(sorted(set(skipped_launches))) or '-',
                ','.join(sorted(set(unsupported))) or '-')
        return 'PASS', '%d job(s), %d check(s)' % (jobs, passed)




def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('tests', nargs='+', help='PTX-formalization test programs (tests/*.ptx)')
    ap.add_argument('--zoc', default='zoc')
    ap.add_argument('--llc', default='llc', help="llc with the AIR target")
    ap.add_argument('--llvm-dis', dest='llvm_dis', default='llvm-dis')
    ap.add_argument('--readelf', default='llvm-readelf')
    ap.add_argument('--objcopy', default='llvm-objcopy')
    ap.add_argument('--arch', default='agx-g14g')
    ap.add_argument('--queue', default=os.environ.get('AIR_QUEUE', '/air'))
    ap.add_argument('--timeout', type=int, default=120)
    ap.add_argument('--max-failures', type=int, default=4, help='failing elements listed per test')
    ap.add_argument('-j', type=int, default=4)
    ap.add_argument('--json', help='write the results to this file')
    ap.add_argument('--progress', help='append each request\'s test and kernels to this file before sending it')
    ap.add_argument('--skip-hang', default=os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                                         'air_ptx_formalization_hangs.txt'),
                    help='file of "TEST KERNEL" lines (# comments) whose requests hang the device; never sent')
    args = ap.parse_args()
    args.counts = {'pass': 0, 'fail': 0, 'skip': 0}
    args.skip_hang = load_hang_list(args.skip_hang)

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
    print('---- .expect checks: %(pass)d pass, %(fail)d fail, %(skip)d skipped' % args.counts)
    if args.json:
        with open(args.json, 'w') as f:
            json.dump({os.path.basename(p): {'status': s, 'detail': d} for p, (s, d) in results}, f, indent=1)
    return 0 if all(s in ('PASS', 'PARTIAL', 'UNSUPPORTED', 'REJECTED', 'SKIPPED-HANG') for _, (s, _) in results) else 1


if __name__ == '__main__':
    sys.exit(main())
