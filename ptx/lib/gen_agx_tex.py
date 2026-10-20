#!/usr/bin/env python3
"""Compile zluda_ptx_impl_agx_tex.c to LLVM IR and splice it into the
generated section at the end of zluda_ptx_impl_agx.ll.

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
SOURCE = LIB / 'zluda_ptx_impl_agx_tex.c'
TARGET = LIB / 'zluda_ptx_impl_agx.ll'
BEGIN = '; ---------------------------------------------------------------- BEGIN GENERATED (gen_agx_tex.py)\n'
END = '; ---------------------------------------------------------------- END GENERATED (gen_agx_tex.py)\n'
PREFIX = '__zluda_agx_tex_'
METADATA_BASE = 10000


def main():
    clang = sys.argv[1] if len(sys.argv) > 1 else 'clang'
    ir = subprocess.run([clang, '-O2', '-S', '-emit-llvm', '-target', 'nvptx64-nvidia-cuda', '-std=c11', '-ffreestanding',
                         '-fno-builtin', '-ffp-contract=off', '-Wall', '-Werror', '-o', '-', str(SOURCE)],
                        check=True, capture_output=True, text=True).stdout
    text = TARGET.read_text()
    if BEGIN in text:
        head = text[:text.index(BEGIN)]
        tail = text[text.index(END) + len(END):]
    else:
        head, tail = text, ''
    declared = set(re.findall(r'^declare [^@]*@([\w.$]+)\(', head, re.M))
    defined = set(re.findall(r'^define [^@]*@([\w.$]+)\(', head, re.M))
    internal = set(re.findall(r'^define internal [^@]*@([\w.$]+)\(', ir, re.M))
    out = []
    for line in ir.splitlines():
        if line.startswith(('source_filename', 'target datalayout', 'target triple', 'attributes #', '; ModuleID')):
            continue
        if 'llvm.experimental.noalias.scope.decl' in line:
            continue
        m = re.match(r'^declare [^@]*@([\w.$]+)\(', line)
        if m and (m.group(1) in declared or m.group(1) in defined):
            continue
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
            line = re.sub(r'!(\d+)', lambda m: '!' + str(int(m.group(1)) + METADATA_BASE), line)
            out.append(line)
            continue
        line = re.sub(r',\s*!(?!llvm\.loop)[a-z][\w.]*\s+!\d+', '', line)
        line = re.sub(r'!llvm\.loop !(\d+)', lambda m: '!llvm.loop !' + str(int(m.group(1)) + METADATA_BASE), line)
        for name in internal:
            line = re.sub(r'@' + re.escape(name) + r'\b', '@' + PREFIX + name, line)
        out.append(line)
    body = '\n'.join(out).strip() + '\n'
    TARGET.write_text(head + BEGIN + '; from zluda_ptx_impl_agx_tex.c; regenerate with ptx/lib/gen_agx_tex.py\n\n' + body + END + tail)


if __name__ == '__main__':
    main()
