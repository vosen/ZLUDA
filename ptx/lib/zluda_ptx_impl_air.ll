; ZLUDA's primitives for the Air target (ptx::Target::Air), in upstream AIR.
;
; The Apple GPU helpers (zluda_ptx_impl_agx.ll) and the front end use a few
; operations of the GPU: system values, SIMD-group functions, barriers and the
; hardware's approximations. For the Agx target they are AGX intrinsics
; (llvm.agx.*); for the Air target ptx/build.rs renames them to the
; __zluda_air.* primitives defined here, in AIR's own terms: AIR's system
; values and SIMD-group functions, and LLVM math that the AIR target's portable
; passes write as AIR (and that the AGX backend compiles back to its
; instructions). Nothing here is AGX IR.
;
; Address spaces are ZLUDA's (generic 0, global 1, shared 3, constant 4,
; private 5); llvm_zluda's AIR compilation maps them to AIR's.
;
; Not yet defined (a module using them does not link): texture sampling and
; the 8x8 SIMD-group matrix multiply-add. The floating point mode register
; does not exist in AIR: __zluda_air.get.fpmode reads round to nearest and
; __zluda_air.set.fpmode does nothing; the Air front end is to round per
; instruction instead.

; System values
declare <3 x i32> @air.thread_position_in_threadgroup.v3i32()
declare <3 x i32> @air.threadgroup_position_in_grid.v3i32()
declare <3 x i32> @air.threads_per_threadgroup.v3i32()
declare <3 x i32> @air.threadgroups_per_grid.v3i32()
declare <3 x i32> @air.thread_position_in_grid.v3i32()
declare i32 @air.thread_index_in_simdgroup()
declare i32 @air.thread_index_in_threadgroup()
declare i32 @air.simdgroup_index_in_threadgroup()

define i32 @__zluda_air.thread.position.in.threadgroup.x() #0 {
  %v = call <3 x i32> @air.thread_position_in_threadgroup.v3i32()
  %e = extractelement <3 x i32> %v, i32 0
  ret i32 %e
}
define i32 @__zluda_air.thread.position.in.threadgroup.y() #0 {
  %v = call <3 x i32> @air.thread_position_in_threadgroup.v3i32()
  %e = extractelement <3 x i32> %v, i32 1
  ret i32 %e
}
define i32 @__zluda_air.thread.position.in.threadgroup.z() #0 {
  %v = call <3 x i32> @air.thread_position_in_threadgroup.v3i32()
  %e = extractelement <3 x i32> %v, i32 2
  ret i32 %e
}
define i32 @__zluda_air.threadgroup.position.in.grid.x() #0 {
  %v = call <3 x i32> @air.threadgroup_position_in_grid.v3i32()
  %e = extractelement <3 x i32> %v, i32 0
  ret i32 %e
}
define i32 @__zluda_air.threadgroup.position.in.grid.y() #0 {
  %v = call <3 x i32> @air.threadgroup_position_in_grid.v3i32()
  %e = extractelement <3 x i32> %v, i32 1
  ret i32 %e
}
define i32 @__zluda_air.threadgroup.position.in.grid.z() #0 {
  %v = call <3 x i32> @air.threadgroup_position_in_grid.v3i32()
  %e = extractelement <3 x i32> %v, i32 2
  ret i32 %e
}
define i32 @__zluda_air.threads.per.threadgroup.x() #0 {
  %v = call <3 x i32> @air.threads_per_threadgroup.v3i32()
  %e = extractelement <3 x i32> %v, i32 0
  ret i32 %e
}
define i32 @__zluda_air.threads.per.threadgroup.y() #0 {
  %v = call <3 x i32> @air.threads_per_threadgroup.v3i32()
  %e = extractelement <3 x i32> %v, i32 1
  ret i32 %e
}
define i32 @__zluda_air.threads.per.threadgroup.z() #0 {
  %v = call <3 x i32> @air.threads_per_threadgroup.v3i32()
  %e = extractelement <3 x i32> %v, i32 2
  ret i32 %e
}
define i32 @__zluda_air.threadgroups.per.grid.x() #0 {
  %v = call <3 x i32> @air.threadgroups_per_grid.v3i32()
  %e = extractelement <3 x i32> %v, i32 0
  ret i32 %e
}
define i32 @__zluda_air.threadgroups.per.grid.y() #0 {
  %v = call <3 x i32> @air.threadgroups_per_grid.v3i32()
  %e = extractelement <3 x i32> %v, i32 1
  ret i32 %e
}
define i32 @__zluda_air.threadgroups.per.grid.z() #0 {
  %v = call <3 x i32> @air.threadgroups_per_grid.v3i32()
  %e = extractelement <3 x i32> %v, i32 2
  ret i32 %e
}
define i32 @__zluda_air.thread.position.in.grid.x() #0 {
  %v = call <3 x i32> @air.thread_position_in_grid.v3i32()
  %e = extractelement <3 x i32> %v, i32 0
  ret i32 %e
}
define i32 @__zluda_air.thread.position.in.grid.y() #0 {
  %v = call <3 x i32> @air.thread_position_in_grid.v3i32()
  %e = extractelement <3 x i32> %v, i32 1
  ret i32 %e
}
define i32 @__zluda_air.thread.position.in.grid.z() #0 {
  %v = call <3 x i32> @air.thread_position_in_grid.v3i32()
  %e = extractelement <3 x i32> %v, i32 2
  ret i32 %e
}
define i32 @__zluda_air.thread.index.in.simdgroup() #0 {
  %v = call i32 @air.thread_index_in_simdgroup()
  ret i32 %v
}
define i32 @__zluda_air.thread.index.in.threadgroup() #0 {
  %v = call i32 @air.thread_index_in_threadgroup()
  ret i32 %v
}
define i32 @__zluda_air.simdgroup.index.in.threadgroup() #0 {
  %v = call i32 @air.simdgroup_index_in_threadgroup()
  ret i32 %v
}
; No core numbering in AIR (%smid is any value in [0, %nsmid)).
define i32 @__zluda_air.core.id() #0 {
  ret i32 0
}

; Barriers: an execution barrier; the fences around it stay LLVM fences.
declare void @air.wg.barrier(i32, i32) #1
define void @__zluda_air.threadgroup.barrier() #2 {
  call void @air.wg.barrier(i32 0, i32 1)
  ret void
}

; SIMD-group functions. AGX's shuffles take an i32 lane or delta, AIR's an i16.
declare i64 @air.simd_ballot.i64(i1) #1
declare i32 @air.simd_shuffle.u.i32(i32, i16) #1
declare i32 @air.simd_shuffle_xor.u.i32(i32, i16) #1
declare i32 @air.simd_shuffle_up.u.i32(i32, i16) #1
declare i32 @air.simd_shuffle_down.u.i32(i32, i16) #1
declare i32 @air.quad_shuffle.u.i32(i32, i16) #1

define i32 @__zluda_air.simd.ballot(i1 %p) #2 {
  %b = call i64 @air.simd_ballot.i64(i1 %p)
  %t = trunc i64 %b to i32
  ret i32 %t
}
define i32 @__zluda_air.simd.shuffle.i32(i32 %v, i32 %l) #2 {
  %l16 = trunc i32 %l to i16
  %r = call i32 @air.simd_shuffle.u.i32(i32 %v, i16 %l16)
  ret i32 %r
}
define i32 @__zluda_air.simd.shuffle.xor.i32(i32 %v, i32 %l) #2 {
  %l16 = trunc i32 %l to i16
  %r = call i32 @air.simd_shuffle_xor.u.i32(i32 %v, i16 %l16)
  ret i32 %r
}
define i32 @__zluda_air.simd.shuffle.up.i32(i32 %v, i32 %l) #2 {
  %l16 = trunc i32 %l to i16
  %r = call i32 @air.simd_shuffle_up.u.i32(i32 %v, i16 %l16)
  ret i32 %r
}
define i32 @__zluda_air.simd.shuffle.down.i32(i32 %v, i32 %l) #2 {
  %l16 = trunc i32 %l to i16
  %r = call i32 @air.simd_shuffle_down.u.i32(i32 %v, i16 %l16)
  ret i32 %r
}
define i32 @__zluda_air.quad.shuffle.i32(i32 %v, i32 %l) #2 {
  %l16 = trunc i32 %l to i16
  %r = call i32 @air.quad_shuffle.u.i32(i32 %v, i16 %l16)
  ret i32 %r
}

; Reductions: the operation is AGX::SIMDOp (and = 0, or = 2, xor = 4,
; iadd = 16, smin = 20, smax = 22, umin = 28, umax = 30), a constant at every
; use, so all but one case folds away once inlined.
declare i32 @air.simd_and.u.i32(i32) #1
declare i32 @air.simd_or.u.i32(i32) #1
declare i32 @air.simd_xor.u.i32(i32) #1
declare i32 @air.simd_sum.s.i32(i32) #1
declare i32 @air.simd_min.s.i32(i32) #1
declare i32 @air.simd_max.s.i32(i32) #1
declare i32 @air.simd_min.u.i32(i32) #1
declare i32 @air.simd_max.u.i32(i32) #1
define i32 @__zluda_air.simd.reduce.i32(i32 %v, i32 %op) #2 {
entry:
  switch i32 %op, label %umax [ i32 0, label %and
                                i32 2, label %or
                                i32 4, label %xor
                                i32 16, label %add
                                i32 20, label %smin
                                i32 22, label %smax
                                i32 28, label %umin ]
and:
  %a = call i32 @air.simd_and.u.i32(i32 %v)
  ret i32 %a
or:
  %o = call i32 @air.simd_or.u.i32(i32 %v)
  ret i32 %o
xor:
  %x = call i32 @air.simd_xor.u.i32(i32 %v)
  ret i32 %x
add:
  %s = call i32 @air.simd_sum.s.i32(i32 %v)
  ret i32 %s
smin:
  %mi = call i32 @air.simd_min.s.i32(i32 %v)
  ret i32 %mi
smax:
  %ma = call i32 @air.simd_max.s.i32(i32 %v)
  ret i32 %ma
umin:
  %ui = call i32 @air.simd_min.u.i32(i32 %v)
  ret i32 %ui
umax:
  %ua = call i32 @air.simd_max.u.i32(i32 %v)
  ret i32 %ua
}

; The hardware's approximations, as portable LLVM math with afn (which the AIR
; target writes as AIR's fast functions and the AGX backend compiles to its
; rcp, rsqrt, exp2, log2 and sin instructions).
declare float @llvm.sqrt.f32(float)
declare float @llvm.exp2.f32(float)
declare float @llvm.log2.f32(float)
declare float @llvm.sin.f32(float)
declare float @llvm.cos.f32(float)
define float @__zluda_air.rcp.f32(float %x) #0 {
  %r = fdiv afn float 1.0, %x
  ret float %r
}
define float @__zluda_air.rsqrt.f32(float %x) #0 {
  %s = call afn float @llvm.sqrt.f32(float %x)
  %r = fdiv afn float 1.0, %s
  ret float %r
}
; rsqrt such that rsqrt_special(x) * x == sqrt(x): 0 for 0, +inf for +inf.
define float @__zluda_air.rsqrt.special.f32(float %x) #0 {
  %r = call float @__zluda_air.rsqrt.f32(float %x)
  %zero = fcmp oeq float %x, 0.0
  %inf = fcmp oeq float %x, 0x7FF0000000000000
  %r1 = select i1 %zero, float 0.0, float %r
  %r2 = select i1 %inf, float 0x7FF0000000000000, float %r1
  ret float %r2
}
define float @__zluda_air.exp2.f32(float %x) #0 {
  %r = call afn float @llvm.exp2.f32(float %x)
  ret float %r
}
define float @__zluda_air.log2.f32(float %x) #0 {
  %r = call afn float @llvm.log2.f32(float %x)
  ret float %r
}
; AGX's sin and cos take the angle in quarter turns.
define float @__zluda_air.sin.f32(float %x) #0 {
  %a = fmul float %x, 0x3FF921FB60000000
  %r = call afn float @llvm.sin.f32(float %a)
  ret float %r
}
define float @__zluda_air.cos.f32(float %x) #0 {
  %a = fmul float %x, 0x3FF921FB60000000
  %r = call afn float @llvm.cos.f32(float %a)
  ret float %r
}

; The floating point mode: AIR has none (see above).
define i32 @__zluda_air.get.fpmode() #0 {
  ret i32 0
}
define void @__zluda_air.set.fpmode(i32 %m) #0 {
  ret void
}

attributes #0 = { alwaysinline nounwind willreturn memory(none) }
attributes #1 = { convergent nounwind willreturn }
attributes #2 = { alwaysinline convergent nounwind willreturn }
