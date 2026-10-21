; llc -mtriple=agx -mcpu=agx-g14c -mattr=+agx-checked -filetype=obj checked.ll -o checked.o
;
; A checked kernel with a pointer argument and a pointer inside a by-reference
; argument (as ZLUDA passes PTX .param structs), with a check site table.

%pair = type { i32, ptr addrspace(1) }

define agx_kernel void @put(ptr addrspace(1) %out, i32 %i, ptr byref(%pair) align 8 %s) !dbg !5 {
  %n = load i32, ptr %s, align 8
  %pp = getelementptr inbounds i8, ptr %s, i64 8
  %q = load ptr addrspace(1), ptr %pp, align 8
  %p = getelementptr i32, ptr addrspace(1) %out, i32 %i, !dbg !8
  store i32 %n, ptr addrspace(1) %p, align 4, !dbg !8
  %r = getelementptr i32, ptr addrspace(1) %q, i32 %i, !dbg !9
  store i32 7, ptr addrspace(1) %r, align 4, !dbg !9
  ret void
}

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!3}
!0 = distinct !DICompileUnit(language: DW_LANG_C99, file: !1, emissionKind: FullDebug)
!1 = !DIFile(filename: "put.cu", directory: "/tmp")
!3 = !{i32 2, !"Debug Info Version", i32 3}
!5 = distinct !DISubprogram(name: "put", scope: !1, file: !1, line: 1, type: !6, unit: !0, spFlags: DISPFlagDefinition)
!6 = !DISubroutineType(types: !7)
!7 = !{null}
!8 = !DILocation(line: 4, column: 3, scope: !5)
!9 = !DILocation(line: 5, column: 3, scope: !5)
