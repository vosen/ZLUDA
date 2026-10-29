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
UNSUPPORTED = ()

SIZES = {'b8': 1, 'u8': 1, 's8': 1, 'b16': 2, 'u16': 2, 's16': 2, 'f16': 2, 'bf16': 2,
         'b32': 4, 'u32': 4, 's32': 4, 'f32': 4, 'f16x2': 4, 'bf16x2': 4,
         'b64': 8, 'u64': 8, 's64': 8, 'f64': 8, 'pred': 1, 'e4m3': 1, 'e5m2': 1}

BASE = 0x10000000          # first harness buffer
PARAM_BASE = 0x08000000    # kernel parameter memory
GLOBALS_BASE = 0x04000000  # global data buffer (.agx.globals)
# the multimem window table of zluda_ptx_impl_agx_cvt.c: window k at MM_BASE + (k + 1) * MM_WINDOW, entry k (64
# bytes: the count, then the device buffers) at MM_BASE + 64 k
MM_BASE = 0x700000000000
MM_WINDOW = 1 << 36
# the fabric logical endpoint table: entry le (64 bytes: the count, the size, the device buffers) at FAB_TABLE + 64 le
FAB_TABLE = 0x6ff000000000
TEX_HEAP = 0x6fe000000000      # hardware texture descriptors (zluda_agx's texture heap)
SAMPLER_HEAP = 0x6fe100000000  # hardware samplers


class Unsupported(Exception):
    pass


# tensormap.replace.elemtype numbering (PTX-formalization's tensor-map encoding)
TM_ELEMTYPE = {'u8': 0, 'u16': 1, 'u32': 2, 's32': 3, 'u64': 4, 's64': 5, 'f16': 6, 'f32': 7,
               'f64': 9, 'bf16': 10, 'tf32': 11}


# descriptor type codes of PTX-formalization's .texture (tmCode of ptx-harness.k)
TEX_CODE = {'u8': 0, 'u16': 1, 'u32': 2, 's32': 3, 'u64': 4, 's64': 5, 'f16': 6, 'f32': 7, 's8': 20, 's16': 21,
            'f64': 8, 'bf16': 9, 'tf32': 11, 'b8': 0, 'b16': 1, 'b32': 2, 'b64': 4}


def object_words(kind, t, addr, attrs, dims):
    """The descriptor of a .texture / .sampler / .surface directive (objBlob / smBlob of
    semantics/ptx-harness.k), which zluda_agx's cuTexObjectCreate / cuSurfObjectCreate also write."""
    nth = lambda l, i, d=0: l[i] if i < len(l) else d
    if kind == 'sampler':
        return [0x5343504d, nth(attrs, 0), nth(attrs, 1), nth(attrs, 2), nth(attrs, 3), nth(attrs, 4),
                nth(attrs, 5, 1048576)] + [0] * 9
    if kind == 'texture':
        if t not in TEX_CODE:
            raise Unsupported('texture type .' + t)
        return [0x54585854, addr, TEX_CODE[t]] + [nth(attrs, i) for i in range(6)] + [nth(dims, i) for i in range(3)] + \
               [nth(attrs, 6, 1), nth(attrs, 7) | nth(attrs, 9) << 8, 0, nth(attrs, 8)]
    return [0x53545446, addr, SIZES[t], nth(attrs, 0), nth(dims, 0), nth(dims, 1), nth(dims, 2), nth(attrs, 1, 11),
            nth(attrs, 2, 1), nth(attrs, 3), nth(attrs, 4), nth(attrs, 5, 1), nth(attrs, 6, int(nth(attrs, 0) > 1)),
            0, 0, 0]


def native_texture(words):
    """The hardware descriptors (Mesa's cmdbuf.xml Texture and Sampler) that zluda_agx's native_descriptors makes for a
    texture of these descriptor words, as (24-byte texture, 8-byte sampler, word 16 without the heap offset and
    sampler index), or None for a texture the software sampler reads."""
    code, ch, filt, addr, norm, rnorm, layers = words[2:9]
    dims, levels, flags = words[9:12], words[12], words[13]
    fmt = {0: (1, 'u'), 1: (2, 'u'), 2: (4, 'u'), 3: (4, 's'), 6: (2, 'f'), 7: (4, 'f'), 20: (1, 's'), 21: (2, 's')}
    if code not in fmt or ch not in (1, 2, 4):
        return None
    nbytes, kind = fmt[code]
    rnorm = rnorm and kind != 'f' and nbytes < 4
    is_float = kind == 'f' or rnorm
    rank = 2 if dims[1] > 1 else 1
    pitch = max(dims[0], 1) * nbytes * ch
    w = {0: 1, 1: 0, 2: 3, 3: 2}.get(addr, 0)
    if dims[2] > 1 or layers > 1 or levels > 1 or flags or words[1] % 16 or pitch % 16 or dims[0] > 16384 \
            or dims[1] > 16384 or (filt and not is_float) or (not norm and w in (1, 2)) or (ch != 4 and w == 3):
        return None
    chan = {(1, 1): 0x00, (1, 2): 0x0a, (1, 4): 0x28, (2, 1): 0x09, (2, 2): 0x23, (2, 4): 0x32, (4, 1): 0x21,
            (4, 2): 0x31, (4, 4): 0x38}[(nbytes, ch)]
    ty = 4 if kind == 'f' else (int(kind == 's') if rnorm else 2 + int(kind == 's'))
    swz = {1: [0, 5, 5, 4], 2: [0, 1, 5, 4], 4: [0, 1, 2, 3]}[ch]
    t = (0 if rank == 1 else 2) | chan << 6 | ty << 13 | sum(v << (16 + 3 * i) for i, v in enumerate(swz))
    t |= (dims[0] - 1) << 28 | (0 if rank == 1 else dims[1] - 1) << 42 | (words[1] >> 4) << 66
    t |= ((max(pitch, 16) - 16) >> 4) << 110
    smp = filt << 23 | filt << 25 | w << 29 | w << 32 | w << 35 | int(not norm) << 38
    tag = 1 << 63 | int(not is_float) << 62 | int(rank == 2) << 61
    return t.to_bytes(24, 'little'), smp.to_bytes(8, 'little'), tag


def tensormap_words(t, addr, dims, strides, box, swizzle, im2col=None):
    """The 16 64-bit words of a tensor map in the encoding of PTX-formalization (tmBlob /
    tmBlobI of semantics/ptx-harness.k), which is also the one ZLUDA's AGX runtime uses:
    0 magic | 1 element type | 2 rank | 3 global address | 4..8 dims | 9..12 byte strides of
    dims 1..4 | 13 box dims 0..3 (16 bits each) | 14 box dim 4, swizzle_mode << 16,
    fill << 24, interleave << 32, swizzle_atomicity << 48 | 15 element strides (8 bits each)."""
    if t not in TM_ELEMTYPE:
        raise Unsupported('tensormap type .' + t)
    nth = lambda l, i: l[i] if i < len(l) else 0
    z = swizzle & 255
    mode = 3 if 3 <= z <= 6 else z
    atom = z - 3 if 4 <= z <= 6 else 0
    w = [0x54484150, TM_ELEMTYPE[t], len(dims), addr] + [nth(dims, i) for i in range(5)]
    w += [nth(strides, i) for i in range(4)]
    w.append(nth(box, 0) | nth(box, 1) << 16 | nth(box, 2) << 32 | nth(box, 3) << 48)
    w.append(nth(box, 4) | mode << 16 | (swizzle >> 8) << 24 | atom << 48)
    w.append(0x0101010101010101)
    if im2col is not None:
        lower, upper, estrides = im2col
        pack = lambda vs: sum((v + 32768) << (16 * i) for i, v in enumerate(vs))
        w[1] |= pack(lower) << 8
        w[2] |= pack(upper) << 8
        w[14] |= 1 << 40
        w[15] = sum(v << (8 * i) for i, v in enumerate(estrides))
    return [x & (2**64 - 1) for x in w]


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


def lower_coll_test(module):
    """The model's internal coll_test.{sum,first}.u32 d, a, mask (semantics/ptx-exec-coll.k) as PTX: redux.sync.add,
    or a shuffle from the lowest lane of the mask."""
    def one(m):
        kind, d, a, mask = m.group(1), m.group(2), m.group(3), m.group(4)
        if kind == 'sum':
            return f'redux.sync.add.u32 {d}, {a}, {mask};'
        return (f'{{ .reg .b32 %coll_t<2>; brev.b32 %coll_t0, {mask}; bfind.shiftamt.u32 %coll_t1, %coll_t0; '
                f'shfl.sync.idx.b32 {d}, {a}, %coll_t1, 0x1f, {mask}; }}')
    return re.sub(r'coll_test\.(sum|first)\.u32\s+([^,;]+?)\s*,\s*([^,;]+?)\s*,\s*([^,;]+?)\s*;', one, module)


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
        # "Not yet implemented: <what>: <statement>" names the statement after the last ": "
        if stmt.startswith('Not yet implemented:') and stmt.rstrip().endswith(';'):
            stmt = stmt.rsplit(': ', 1)[-1]
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


def launch_value(d, kind):
    """Value `kind` of .agx.kernels extension 7 for the 32-bit argument word d, a divisor (|d| of a signed one for
    kinds 3 to 5; of max(d, 1) for kinds 6 to 8), as agx_runtime computes it: with l = ceil(log2(d)), the multiplier 2^32 (2^l - d) / d + 1 and the
    shifts min(l, 1) and max(l, 1) - 1; all 0 for d = 0."""
    d &= 0xFFFFFFFF
    if kind >= 6:  # of max(d, 1), a guarded unsigned divisor
        d, kind = max(d, 1), kind - 6
    if kind >= 3 and d >> 31:
        d = (1 << 32) - d
    if d == 0:
        return 0
    l = (d - 1).bit_length()
    return [((1 << 32) * ((1 << l) - d)) // d + 1, min(l, 1), max(l, 1) - 1][kind % 3] & 0xFFFFFFFF

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
    # the kernel records (.agx.kernels): uniform layout, arguments and resources, as agx_runtime reads them
    records = {}
    sec = os.path.join(tmp, 'kernels.bin')
    if subprocess.run([objcopy, '--dump-section', '.agx.kernels=' + sec, obj, os.path.join(tmp, 'unused.o')],
                      capture_output=True).returncode == 0:
        d = open(sec, 'rb').read()
        pos = 0
        while pos + 12 <= len(d):
            size, _version, nlen = struct.unpack_from('<III', d, pos)
            if size == 0:
                break
            name = d[pos + 12:pos + 12 + nlen].decode()
            q = pos + 12 + (nlen + 3) // 4 * 4
            halves, tg_half, glob, _gprs, tg_bytes, _stack, nargs = struct.unpack_from('<7I', d, q)
            args = [struct.unpack_from('<3I', d, q + 28 + 12 * i) for i in range(nargs)]
            # tagged extensions up to the record's end: 1 = cluster dimensions, 4 = the hidden threadgroup memory
            # size argument, 5 = the texture heap, 6 = 32-bit constants to preload, 7 = values computed from argument
            # words (2 and 3 are checked mode's, which the harness does not run)
            cluster, tgb_half, tex_half, consts, derived, e = (1, 1, 1), None, None, [], [], q + 28 + 12 * nargs
            while e + 4 <= pos + size:
                tag, = struct.unpack_from('<I', d, e)
                if tag == 1 and e + 16 <= pos + size:
                    cluster = struct.unpack_from('<3I', d, e + 4)
                    e += 16
                elif tag == 4 and e + 8 <= pos + size:
                    tgb_half, = struct.unpack_from('<I', d, e + 4)
                    e += 8
                elif tag == 5 and e + 8 <= pos + size:
                    tex_half, = struct.unpack_from('<I', d, e + 4)
                    e += 8
                elif tag == 6 and e + 8 <= pos + size:
                    n, = struct.unpack_from('<I', d, e + 4)
                    consts = [struct.unpack_from('<2I', d, e + 8 + 8 * i) for i in range(n)]
                    e += 8 + 8 * n
                elif tag == 7 and e + 8 <= pos + size:
                    n, = struct.unpack_from('<I', d, e + 4)
                    derived = [struct.unpack_from('<3I', d, e + 8 + 12 * i) for i in range(n)]
                    e += 8 + 12 * n
                else:
                    break
            records[name] = (halves, tg_half, glob, tg_bytes, args, cluster, tgb_half, tex_half, consts, derived)
            pos += size
    kernels = {}
    for name, (halves, tg_half, glob, tg_bytes, args, cluster, tgb_half, tex_half, consts, derived) in records.items():
        if name not in syms or syms[name][2] != 'FUNC':
            continue
        off, size = syms[name][0], syms[name][1]
        kernels[name] = {
            'code': text[off:off + size],
            'halves': halves,
            'grid_half': tg_half,
            'globals_half': None if glob == 0xffffffff else glob,
            'shared': tg_bytes,
            # (by pointer, bytes, first 16-bit half) per source-level argument
            'args': [(bp == 1, nb, h) for bp, nb, h in args],
            # CTAs per cluster: a cluster runs as one threadgroup, thread (x, y, z) of CTA r at (x, y, z * C + r)
            'cluster': cluster,
            # the hidden argument set to the threadgroup memory of the launch, static and dynamic bytes
            'tgb_half': None if tgb_half in (None, 0xffffffff) else tgb_half,
            # the texture heap address (extension 5, a module that samples with the texture unit)
            'tex_half': tex_half,
            # (half, value): 32-bit constants the launch preloads (extension 6, llc -agx-uniform-constants)
            'consts': consts,
            # (half, source half, kind): values the launch computes from argument words (extension 7,
            # llc -agx-launch-divisors), see launch_value
            'derived': derived,
        }
    gsyms = {n: v[0] for n, v in syms.items() if v[2] == 'OBJECT' and v[3] == gsec}
    gsizes = {n: v[1] for n, v in syms.items() if v[2] == 'OBJECT' and v[3] == gsec}
    # the relocations of address-valued initializers (.rela.agx.globals, AGXUsage.rst): (offset, size, symbol
    # offset in .agx.globals + addend), of symbols defined in .agx.globals
    relocs = []
    rela, symtab = os.path.join(tmp, 'rela.bin'), os.path.join(tmp, 'symtab.bin')
    if subprocess.run([objcopy, '--dump-section', '.rela.agx.globals=' + rela, '--dump-section',
                       '.symtab=' + symtab, obj, os.path.join(tmp, 'unused.o')], capture_output=True).returncode == 0:
        r, st = open(rela, 'rb').read(), open(symtab, 'rb').read()
        for i in range(0, len(r) - 23, 24):
            offset, info, addend = struct.unpack_from('<QQq', r, i)
            _name, _info, _other, shndx, value, _size = struct.unpack_from('<IBBHQQ', st, 24 * (info >> 32))
            if shndx != gsec:
                raise Unsupported('relocation against a symbol outside .agx.globals')
            relocs.append((offset, {1: 8, 2: 4}[info & 0xffffffff], value + addend))
    return kernels, gdata, gsyms, gsizes, relocs


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


def globals_image(gdata, gsyms, gsizes, relocs):
    """The global data buffer at GLOBALS_BASE as agx_runtime loads it: .agx.globals with its relocations applied."""
    globals_mem = bytearray(gdata) + bytearray(max(0, 16 - len(gdata)))
    for off, n, target in relocs:
        globals_mem[off:off + n] = ((GLOBALS_BASE + target) & ((1 << (8 * n)) - 1)).to_bytes(n, 'little')
    # the mask(name + offset) initializer elements (agx_runtime MASKED_INIT_SYMBOL)
    if '__zluda_agx_masked_init' in gsyms:
        t = gsyms['__zluda_agx_masked_init']
        words = lambda a: int.from_bytes(globals_mem[a:a + 8], 'little')
        for e in range(t, t + gsizes.get('__zluda_agx_masked_init', 0), 24):
            slot, address, desc = words(e) - GLOBALS_BASE, words(e + 8), words(e + 16)
            n = desc >> 8
            globals_mem[slot:slot + n] = ((address >> (desc & 63)) & 0xff).to_bytes(n, 'little')
    return globals_mem


def simulate(args, tmp, name, k, ps, ops, grid, block, dyn, syms, mem, globals_mem, tag='k', address32=False,
             extra=()):
    """Run one launch of kernel `name` (record k of kernel_info) on agx_run: harness memory mem at BASE, the global
    data globals_mem at GLOBALS_BASE (both updated in place), parameters ps (kernel_params) with launch operands ops.
    extra: more agx_run arguments (maps of the multimem / fabric / texture tables). Returns None or an error."""
    if len(ops) != len(ps):
        raise Unsupported('launch argument count')
    pmem = bytearray()
    halves = {}

    def put(half, value, nbytes):
        for j in range((nbytes + 1) // 2):
            halves[half + j] = (value >> (16 * j)) & 0xffff

    ps_all = list(zip(ps, ops))
    if address32:
        # 32-bit modules take the 64-bit base of their 32-bit addresses as a last parameter; the harness
        # buffers are below 4 GiB, so it is 0.
        ps_all.append((('u64', 1, 8), '0'))
    if len(k['args']) != len(ps_all):
        raise Unsupported('kernel record argument count')
    for ((t, n, al), o), (by_pointer, nbytes, half) in zip(ps_all, k['args']):
        if half == 0xffffffff:
            continue  # unused by the kernel
        size = SIZES[t] * n
        if n != 1:
            # an aggregate (.b8 a[16]): the launch value fills its bytes, little-endian
            value = (syms[o] if o in syms else int(o, 0)) & ((1 << (8 * size)) - 1)
            if not by_pointer and size > nbytes:
                raise Unsupported('aggregate kernel parameter')
        else:
            value = lit_bits(o, t, syms)
        if by_pointer:
            off = (len(pmem) + 15) & ~15
            pmem.extend(b'\0' * (off - len(pmem)))
            pmem.extend(value.to_bytes(size, 'little'))
            put(half, PARAM_BASE + off, 8)
        else:
            put(half, value, nbytes)
    c = k['cluster']
    if any(grid[j] % c[j] for j in range(3)):
        raise Unsupported('grid not a multiple of the cluster dimensions')
    hw_grid = [grid[j] // c[j] for j in range(3)]
    hw_block = [block[0], block[1], block[2] * c[0] * c[1] * c[2]]
    for j in range(3):
        put(k['grid_half'] + 2 * j, hw_grid[j], 4)
    if k['globals_half'] is not None:
        put(k['globals_half'], GLOBALS_BASE, 8)
    if k['tgb_half'] is not None:
        put(k['tgb_half'], k['shared'] + dyn, 4)
    if k['tex_half'] is not None:
        put(k['tex_half'], TEX_HEAP, 8)
    for h, v in k['consts']:
        put(h, v, 4)
    for h, src, kind in k['derived']:
        put(h, launch_value(halves.get(src, 0) | halves.get(src + 1, 0) << 16, kind), 4)
    # agx_run takes 32-bit uniforms
    uniforms = sorted({(h // 2, halves.get(h // 2 * 2, 0) | halves.get(h // 2 * 2 + 1, 0) << 16)
                       for h in halves})
    f_code = os.path.join(tmp, tag + '.bin')
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
           '--grid', ','.join(str(hw_grid[j] * hw_block[j]) for j in range(3)),
           '--wg', ','.join(map(str, hw_block)),
           '--shared', str(k['shared'] + dyn),
           '--map', '%#x=%s' % (BASE, f_mem), '--map', '%#x=%s' % (PARAM_BASE, f_par),
           '--map', '%#x=%s' % (GLOBALS_BASE, f_glob),
           '--dump', '%#x=%d:%s' % (BASE, len(mem.buf), f_mem),
           '--dump', '%#x=%d:%s' % (GLOBALS_BASE, len(globals_mem), f_glob)] + list(extra)
    for u, v in uniforms:
        cmd += ['--uniform', '%d=%d' % (u, v)]
    if getattr(args, 'agx_strict', False):
        cmd += ['--agx-strict', args.arch]
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=args.timeout)
    except subprocess.TimeoutExpired:
        return 'simulation timed out (%s)' % name
    if r.returncode != 0:
        try:
            err = json.loads(r.stdout.strip().splitlines()[-1])['error']
        except Exception:
            err = (r.stdout + r.stderr).strip()[-300:]
        return '%s: %s' % (name, err[:300])
    with open(f_mem, 'rb') as f:
        mem.buf = bytearray(f.read())
    with open(f_glob, 'rb') as f:
        g = f.read()
        globals_mem[:] = g[:len(globals_mem)]
    return None


def compile_ptx(args, module, tmp, extra=(), env=None, product='m.o', target='agx'):
    """Compile module (PTX text) with zoc --target agx into tmp, dropping the statements the front end rejects.
    Returns (status, detail) of a failure, or (None, (zoc output, rejected statements, tainted functions or None
    when nothing can be attributed))."""
    src = os.path.join(tmp, 'm.ptx')
    with open(src, 'w') as f:
        f.write(module)

    def zoc(*more):
        flags = list(extra) + list(more)
        if getattr(args, 'strict_fp', False):
            flags.append('--strict-fp')
        if getattr(args, 'agx_strict', False):
            flags.append('--agx-strict')
        return subprocess.run([args.zoc, *flags, '--target', target, '--arch', args.arch, '--output-dir', tmp, src],
                              capture_output=True, text=True, timeout=args.timeout, env=env)
    try:
        # The statements the front end rejects (printed only without --ignore-errors); then compile the rest.
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
    if r.returncode != 0 or not os.path.exists(os.path.join(tmp, product)):
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
    return None, (out, rejected, tainted)


def run_test(path, args):
    with open(path) as f:
        module, dirs = split_program(f.read())
    module = lower_coll_test(module)
    for d in dirs:
        if d.split()[0] in UNSUPPORTED:
            return 'UNSUPPORTED', 'harness directive ' + d.split()[0]
    with tempfile.TemporaryDirectory() as tmp:
        status, detail = compile_ptx(args, module, tmp, target=getattr(args, 'target', 'agx'))
        if status:
            return status, detail
        out, rejected, tainted = detail
        with open(os.path.join(tmp, 'm.asm')) as f:
            asm = f.read()
        kernels, gdata, gsyms, gsizes, relocs = kernel_info(asm, os.path.join(tmp, 'm.o'), args.readelf, args.objcopy, tmp)
        params = kernel_params(module)

        mem = Memory()
        syms, bufs, last = {}, {}, None
        mm_entries = []
        fab_entries = {}
        textures, samplers = [], []
        globals_mem = globals_image(gdata, gsyms, gsizes, relocs)
        for n, off in gsyms.items():
            syms[n] = GLOBALS_BASE + off
        failures, launches, skipped, passed, skipped_launches = [], 0, 0, 0, []
        # buffers a skipped launch could have written (its arguments), and what later launches derive from them
        tainted_bufs = set()

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
            elif kw == '.multimem':
                m = re.fullmatch(r'\.multimem ([\w$.]+) \.size (\d+) \{(.*?)\}\s*;?', d)
                if not m:
                    raise Unsupported(d)
                members = [syms[b.strip()] for b in m.group(3).split(',') if b.strip()]
                if len(members) > 7:
                    raise Unsupported('more than 7 multimem devices')
                mm_entries.append(members)
                syms[m.group(1)] = MM_BASE + len(mm_entries) * MM_WINDOW
            elif kw == '.fabric':
                m = re.fullmatch(r'\.fabric (\d+) \.size (\d+) \{(.*?)\}\s*;?', d)
                if not m or int(m.group(1)) > 255:
                    raise Unsupported(d)
                members = [syms[b.strip()] for b in m.group(3).split(',') if b.strip()]
                if len(members) > 7:
                    raise Unsupported('more than 7 fabric devices')
                fab_entries[int(m.group(1))] = (int(m.group(2)), members)
            elif kw == '.tensormap':
                m = re.fullmatch(r'\.tensormap ([\w$.]+) \.(\w+) ([\w$.]+) \.dims \{(.*?)\} \.strides \{(.*?)\} '
                                 r'\.box \{(.*?)\} \.swizzle (\d+)(?: \.im2col \{(.*?)\} \{(.*?)\} \.estrides \{(.*?)\})?\s*;?', d)
                if not m:
                    raise Unsupported(d)
                ints = lambda g: [int(v, 0) for v in split_vals(g)] if g and g.strip() else []
                im2col = None if m.group(8) is None else (ints(m.group(8)), ints(m.group(9)), ints(m.group(10)))
                words = tensormap_words(m.group(2), syms[m.group(3)], ints(m.group(4)), ints(m.group(5)),
                                        ints(m.group(6)), int(m.group(7)), im2col)
                addr = BASE + ((mem.top + 255) & ~255)
                mem.top = (addr - BASE) + 128
                syms[m.group(1)] = addr
                bufs[m.group(1)] = [addr, 'u64', 16]
                for k, v in enumerate(words):
                    mem.write(addr + 8 * k, 8, v)
            elif kw in ('.texture', '.sampler', '.surface'):
                m = re.fullmatch(r'\.(texture|surface) ([\w$.]+) \.(\w+) ([\w$.]+) \.dims \{(.*?)\} \.attrs \{(.*?)\}\s*;?', d) \
                    or re.fullmatch(r'\.(sampler) ([\w$.]+) \.attrs \{(.*?)\}\s*;?', d)
                if not m:
                    raise Unsupported(d)
                ints = lambda g: [int(v, 0) for v in split_vals(g)] if g and g.strip() else []
                if m.group(1) == 'sampler':
                    words = object_words('sampler', None, 0, ints(m.group(3)), [])
                else:
                    words = object_words(m.group(1), m.group(3), syms[m.group(4)], ints(m.group(6)), ints(m.group(5)))
                    native = native_texture(words) if m.group(1) == 'texture' else None
                    if native:
                        # word 16 (zluda_agx texture.rs): the heap offset and the sampler index of its hardware
                        # descriptors, samplers shared when equal
                        tex, smp, tag = native
                        if smp not in samplers:
                            samplers.append(smp)
                        # texture_sample's index of heap sampler k is k + 28 (zluda_agx texture.rs SAMPLER_HEAP_BASE)
                        words = words + [tag | (28 + samplers.index(smp)) << 32 | 24 * len(textures)]
                        textures.append(tex)
                addr = BASE + ((mem.top + 255) & ~255)
                mem.top = (addr - BASE) + 8 * len(words)
                syms[m.group(2)] = addr
                bufs[m.group(2)] = [addr, 'u64', len(words)]
                for k, v in enumerate(words):
                    mem.write(addr + 8 * k, 8, v & (2**64 - 1))
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
                    tainted_bufs.update(o.strip() for o in m.group(6).split(','))
                    continue
                if any(o.strip() in tainted_bufs for o in m.group(6).split(',')):
                    tainted_bufs.update(o.strip() for o in m.group(6).split(','))
                k = kernels[name]
                grid, block = parse_dim(m.group(2)), parse_dim(m.group(3))
                dyn = int(m.group(5)) if m.group(5) else 0
                ops = [o.strip() for o in m.group(6).split(',') if o.strip()]
                ps = params.get(name, [])
                extra = []
                if mm_entries:
                    f_mm = os.path.join(tmp, 'mm.bin')
                    with open(f_mm, 'wb') as f:
                        for members in mm_entries:
                            row = len(members).to_bytes(8, 'little') + b''.join(a.to_bytes(8, 'little') for a in members)
                            f.write(row + b'\0' * (64 - len(row)))
                    extra += ['--map', '%#x=%s' % (MM_BASE, f_mm)]
                if fab_entries:
                    f_fab = os.path.join(tmp, 'fab.bin')
                    table = bytearray(64 * (max(fab_entries) + 1))
                    for le, (size, members) in fab_entries.items():
                        row = len(members).to_bytes(4, 'little') + size.to_bytes(4, 'little') + \
                            b''.join(a.to_bytes(8, 'little') for a in members)
                        table[64 * le:64 * le + len(row)] = row
                    with open(f_fab, 'wb') as f:
                        f.write(table)
                    extra += ['--map', '%#x=%s' % (FAB_TABLE, f_fab)]
                if k['tex_half'] is not None and textures:
                    f_tex, f_smp = os.path.join(tmp, 'tex.bin'), os.path.join(tmp, 'smp.bin')
                    with open(f_tex, 'wb') as f:
                        f.write(b''.join(textures))
                    with open(f_smp, 'wb') as f:
                        f.write(b''.join(samplers))
                    extra += ['--map', '%#x=%s' % (TEX_HEAP, f_tex), '--map', '%#x=%s' % (SAMPLER_HEAP, f_smp),
                              '--sampler-heap', '%#x=%d' % (SAMPLER_HEAP, len(samplers))]
                address32 = bool(re.search(r'^\s*\.address_size\s+32\b', module, re.M))
                err = simulate(args, tmp, name, k, ps, ops, grid, block, dyn, syms, mem, globals_mem,
                               'k%d' % launches, address32, extra)
                if err:
                    return 'SIM-ERROR', err
                launches += 1
            elif kw == '.expect':
                m = re.fullmatch(r'\.expect ([\w$.]+) \.(\w+)\s*(@\s*(\d+))?\s*=\s*\{(.*)\}', d)
                t = ty_of(m.group(2))
                name = m.group(1)
                if name not in syms:
                    raise Unsupported('.expect of unknown symbol ' + name)
                off = int(m.group(4)) if m.group(4) else 0
                vals = split_vals(m.group(5))
                if skipped_launches and (name in tainted_bufs or name not in bufs):
                    # the buffer may have been written by a skipped kernel (globals: by any kernel)
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
            n = args.max_failures
            return 'FAIL', '; '.join(failures[:n]) + (' (+%d)' % (len(failures) - n) if len(failures) > n else '')
        if skipped_launches:
            return 'PARTIAL', '%d check(s) pass, %d skipped after rejected kernels %s; rejected: %s' % (
                passed, skipped, ','.join(sorted(set(skipped_launches))), ' | '.join(rejected)[:200])
        return 'PASS', '%d launch(es), %d check(s)' % (launches, passed)


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('tests', nargs='+', help='PTX-formalization test programs (tests/*.ptx)')
    ap.add_argument('--zoc', default='zoc')
    ap.add_argument('--agx-run', dest='agx_run', default='agx_run')
    ap.add_argument('--target', default='agx', choices=('agx', 'air'),
                    help='zoc --target: agx (ZLUDA\'s AGX IR) or air (PTX to portable AIR, then the AGX backend)')
    ap.add_argument('--readelf', default='llvm-readelf')
    ap.add_argument('--objcopy', default='llvm-objcopy')
    ap.add_argument('--arch', default='agx-g14g')
    ap.add_argument('--strict-fp', action='store_true',
                    help='compile with zoc --strict-fp (software IEEE binary32, canonical NaNs)')
    ap.add_argument('--agx-strict', action='store_true',
                    help='compile with zoc --agx-strict (only the instructions Mesa emits) and run agx_run '
                         '--agx-strict for --arch, so that any other instruction fails the test')
    ap.add_argument('--timeout', type=int, default=120)
    ap.add_argument('--max-failures', type=int, default=4, help='failing elements listed per test')
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
