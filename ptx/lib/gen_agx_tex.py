#!/usr/bin/env python3
"""Compile zluda_ptx_impl_agx_tex.c, zluda_ptx_impl_agx_cvt.c, zluda_ptx_impl_agx_heap.c and
zluda_ptx_impl_agx_math.c to LLVM IR
and splice them into the generated section at the end of zluda_ptx_impl_agx.ll.

    python3 ptx/lib/gen_agx_tex.py [clang]

The C is compiled for nvptx64, whose IR is target neutral here (address space 1
is device memory, as on AGX). Internal functions get a unique prefix,
declarations that the hand-written part already has are dropped, and
attribute groups are replaced by alwaysinline and metadata is removed."""

import re
import subprocess
import sys
from pathlib import Path

LIB = Path(__file__).resolve().parent
# source, prefix of its internal functions, first metadata number
SOURCES = [('zluda_ptx_impl_agx_tex.c', '__zluda_agx_tex_', 10000), ('zluda_ptx_impl_agx_cvt.c', '__zluda_agx_cvt_', 20000),
           ('zluda_ptx_impl_agx_heap.c', '__zluda_agx_heap_', 30000),
           ('zluda_ptx_impl_agx_math.c', '__zluda_agx_math_', 40000)]
TARGET = LIB / 'zluda_ptx_impl_agx.ll'
BEGIN = '; ---------------------------------------------------------------- BEGIN GENERATED (gen_agx_tex.py)\n'
END = '; ---------------------------------------------------------------- END GENERATED (gen_agx_tex.py)\n'


def convert(clang, source, prefix, metadata_base, declared, defined):
    ir = subprocess.run([clang, '-O2', '-S', '-emit-llvm', '-target', 'nvptx64-nvidia-cuda', '-std=c11', '-ffreestanding',
                         '-fno-builtin', '-ffp-contract=off', '-Wall', '-Werror', '-o', '-', str(LIB / source)],
                        check=True, capture_output=True, text=True).stdout
    internal = set(re.findall(r'^define internal [^@]*@([\w.$]+)\(', ir, re.M))
    out = []
    for line in ir.splitlines():
        if line.startswith(('source_filename', 'target datalayout', 'target triple', 'attributes #', '; ModuleID')):
            continue
        if 'llvm.experimental.noalias.scope.decl' in line:
            continue
        # globals of the hand-written part (the heap pointer)
        m = re.match(r'^@([\w.$]+) = external ', line)
        if m and m.group(1) in defined:
            continue
        m = re.match(r'^declare [^@]*@([\w.$]+)\(', line)
        if m:
            if m.group(1) in declared or m.group(1) in defined:
                continue
            declared.add(m.group(1))
        line = line.replace('spir_func ', '')
        # every helper is always inlined (#3 of the hand-written part), so that the constant
        # arguments fold and the local arrays become registers
        if line.startswith('define '):
            line = re.sub(r'\)(\s+(?:local_unnamed_addr|unnamed_addr))?\s+#\d+ \{$', r')\1 #3 {', line)
        else:
            line = re.sub(r'\)(\s+(?:local_unnamed_addr|unnamed_addr))?\s+#\d+', r')\1', line)
        line = re.sub(r' #\d+$', '', line)
        # loop metadata (unroll disable) stays, renumbered after any of the hand-written part's
        if line.startswith('!'):
            if not re.match(r'^!\d+ = ', line):
                continue
            line = re.sub(r'!(\d+)', lambda m: '!' + str(int(m.group(1)) + metadata_base), line)
            out.append(line)
            continue
        line = re.sub(r',\s*!(?!llvm\.loop)[a-z][\w.]*\s+!\d+', '', line)
        line = re.sub(r'!llvm\.loop !(\d+)', lambda m: '!llvm.loop !' + str(int(m.group(1)) + metadata_base), line)
        for name in internal:
            line = re.sub(r'@' + re.escape(name) + r'\b', '@' + prefix + name, line)
        out.append(line)
    return '; from ' + source + '\n\n' + '\n'.join(out).strip() + '\n'


def main():
    clang = sys.argv[1] if len(sys.argv) > 1 else 'clang'
    text = TARGET.read_text()
    if BEGIN in text:
        head = text[:text.index(BEGIN)]
        tail = text[text.index(END) + len(END):]
    else:
        head, tail = text, ''
    declared = set(re.findall(r'^declare [^@]*@([\w.$]+)\(', head, re.M))
    defined = set(re.findall(r'^define [^@]*@([\w.$]+)\(', head, re.M)) | set(re.findall(r'^@([\w.$]+) = ', head, re.M))
    bodies = [convert(clang, *source, declared, defined) for source in SOURCES]
    TARGET.write_text(head + BEGIN + '; regenerate with ptx/lib/gen_agx_tex.py\n\n' + '\n'.join(bodies) + END + tail)


if __name__ == '__main__':
    main()
