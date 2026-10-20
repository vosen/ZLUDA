; Implementation of ZLUDA's PTX helper functions (__zluda_ptx_impl_*) for
; Apple GPUs, linked into every module compiled with the AGX backend.
;
; The AMDGPU implementation (zluda_ptx_impl.cpp) is compiled with clang, which
; has no AGX target, so this one is written directly in LLVM IR on top of the
; llvm.agx.* intrinsics. It is embedded as text and parsed at compile time.
;
; Functions missing here are reported as undefined when linking a module
; that uses them.

target triple = "agx-apple-macosx"

; Special registers. The member index selects x (0), y (1) or z (2).
define i32 @__zluda_ptx_impl_sreg_tid(i8 zeroext %member) #0 {
  switch i8 %member, label %x [i8 1, label %y
                                i8 2, label %z]
x:
  %vx = call i32 @llvm.agx.thread.position.in.threadgroup.x()
  ret i32 %vx
y:
  %vy = call i32 @llvm.agx.thread.position.in.threadgroup.y()
  ret i32 %vy
z:
  %vz = call i32 @llvm.agx.thread.position.in.threadgroup.z()
  ret i32 %vz
}
define i32 @__zluda_ptx_impl_sreg_ntid(i8 zeroext %member) #0 {
  switch i8 %member, label %x [i8 1, label %y
                                i8 2, label %z]
x:
  %vx = call i32 @llvm.agx.threads.per.threadgroup.x()
  ret i32 %vx
y:
  %vy = call i32 @llvm.agx.threads.per.threadgroup.y()
  ret i32 %vy
z:
  %vz = call i32 @llvm.agx.threads.per.threadgroup.z()
  ret i32 %vz
}
define i32 @__zluda_ptx_impl_sreg_ctaid(i8 zeroext %member) #0 {
  switch i8 %member, label %x [i8 1, label %y
                                i8 2, label %z]
x:
  %vx = call i32 @llvm.agx.threadgroup.position.in.grid.x()
  ret i32 %vx
y:
  %vy = call i32 @llvm.agx.threadgroup.position.in.grid.y()
  ret i32 %vy
z:
  %vz = call i32 @llvm.agx.threadgroup.position.in.grid.z()
  ret i32 %vz
}
define i32 @__zluda_ptx_impl_sreg_nctaid(i8 zeroext %member) #0 {
  switch i8 %member, label %x [i8 1, label %y
                                i8 2, label %z]
x:
  %vx = call i32 @llvm.agx.threadgroups.per.grid.x()
  ret i32 %vx
y:
  %vy = call i32 @llvm.agx.threadgroups.per.grid.y()
  ret i32 %vy
z:
  %vz = call i32 @llvm.agx.threadgroups.per.grid.z()
  ret i32 %vz
}
define i32 @__zluda_ptx_impl_sreg_laneid() #0 {
  %l = call i32 @llvm.agx.thread.index.in.simdgroup()
  ret i32 %l
}

define i32 @__zluda_ptx_impl_sreg_lanemask_le() #0 {
  %l = call i32 @llvm.agx.thread.index.in.simdgroup()
  %s = shl i32 2, %l
  %m = sub i32 %s, 1
  ret i32 %m
}

define i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0 {
  %l = call i32 @llvm.agx.thread.index.in.simdgroup()
  %s = shl i32 1, %l
  %m = sub i32 %s, 1
  ret i32 %m
}

define i32 @__zluda_ptx_impl_sreg_lanemask_ge() #0 {
  %l = call i32 @llvm.agx.thread.index.in.simdgroup()
  %m = shl i32 -1, %l
  ret i32 %m
}

; There is no known cycle counter readable by shaders. The clocks count reads
; of a device-wide counter instead: each read of a thread is later than its
; previous one, as PTX requires, but the values do not measure time.
@__zluda_ptx_impl_agx_clock = internal addrspace(1) global i32 0

define i32 @__zluda_ptx_impl_sreg_clock() #3 {
  %c = atomicrmw add ptr addrspace(1) @__zluda_ptx_impl_agx_clock, i32 1 monotonic
  %r = add i32 %c, 1
  ret i32 %r
}

define i64 @__zluda_ptx_impl_sreg_clock64() #3 {
  %c = call i32 @__zluda_ptx_impl_sreg_clock()
  %r = zext i32 %c to i64
  ret i64 %r
}

define i64 @__zluda_ptx_impl_sreg_globaltimer() #3 {
  %c = call i64 @__zluda_ptx_impl_sreg_clock64()
  %r = mul i64 %c, 1000
  ret i64 %r
}

define i32 @__zluda_ptx_impl_sreg_warpid() #0 {
  %w = call i32 @llvm.agx.simdgroup.index.in.threadgroup()
  ret i32 %w
}

; The number of SIMD-groups of the threadgroup.
define i32 @__zluda_ptx_impl_sreg_nwarpid() #0 {
  %x = call i32 @llvm.agx.threads.per.threadgroup.x()
  %y = call i32 @llvm.agx.threads.per.threadgroup.y()
  %z = call i32 @llvm.agx.threads.per.threadgroup.z()
  %xy = mul i32 %x, %y
  %n = mul i32 %xy, %z
  %n31 = add i32 %n, 31
  %r = lshr i32 %n31, 5
  ret i32 %r
}

define i32 @__zluda_ptx_impl_sreg_smid() #0 {
  %c = call i32 @llvm.agx.core.id()
  ret i32 %c
}

define i32 @__zluda_ptx_impl_activemask() #0 {
  %m = call i32 @llvm.agx.simd.ballot(i1 true)
  ret i32 %m
}

; Bit field extract and insert, with PTX's clamping of position and length.
define i32 @__zluda_ptx_impl_bfe_u32(i32 %base, i32 %pos32, i32 %len32) #0 {
  %pos = and i32 %pos32, 255
  %len = and i32 %len32, 255
  %pos.big = icmp uge i32 %pos, 32
  %shifted = lshr i32 %base, %pos
  %len.big = icmp uge i32 %len, 32
  %mask.s = shl i32 1, %len
  %mask = sub i32 %mask.s, 1
  %masked = and i32 %shifted, %mask
  %r0 = select i1 %len.big, i32 %shifted, i32 %masked
  %r = select i1 %pos.big, i32 0, i32 %r0
  ret i32 %r
}

define i64 @__zluda_ptx_impl_bfe_u64(i64 %base, i32 %pos, i32 %len) #0 {
  %pos64 = zext i32 %pos to i64
  %len64 = zext i32 %len to i64
  %pos.big = icmp uge i32 %pos, 64
  %shifted = lshr i64 %base, %pos64
  %len.big = icmp uge i32 %len, 64
  %mask.s = shl i64 1, %len64
  %mask = sub i64 %mask.s, 1
  %masked = and i64 %shifted, %mask
  %r0 = select i1 %len.big, i64 %shifted, i64 %masked
  %r = select i1 %pos.big, i64 0, i64 %r0
  ret i64 %r
}

define i32 @__zluda_ptx_impl_bfe_s32(i32 %base, i32 %pos32, i32 %len32) #0 {
  %pos = and i32 %pos32, 255
  %len = and i32 %len32, 255
  %len.zero = icmp eq i32 %len, 0
  %pos.big = icmp uge i32 %pos, 32
  %sign = ashr i32 %base, 31
  ; Clamp pos + len to 32, then shift the field to the top and back.
  %end = add i32 %pos, %len
  %end.big = icmp ugt i32 %end, 32
  %len.c = sub i32 32, %pos
  %len.f = select i1 %end.big, i32 %len.c, i32 %len
  %up = sub i32 32, %pos
  %up2 = sub i32 %up, %len.f
  %top = shl i32 %base, %up2
  %down = sub i32 32, %len.f
  %field = ashr i32 %top, %down
  %r0 = select i1 %pos.big, i32 %sign, i32 %field
  %r = select i1 %len.zero, i32 0, i32 %r0
  ret i32 %r
}

define i64 @__zluda_ptx_impl_bfe_s64(i64 %base, i32 %pos, i32 %len) #0 {
  %len.zero = icmp eq i32 %len, 0
  %pos.big = icmp uge i32 %pos, 64
  %sign = ashr i64 %base, 63
  %sum = call {i32, i1} @llvm.uadd.with.overflow.i32(i32 %pos, i32 %len)
  %end = extractvalue {i32, i1} %sum, 0
  %ovf = extractvalue {i32, i1} %sum, 1
  %end.big0 = icmp uge i32 %end, 64
  %end.big = or i1 %end.big0, %ovf
  %len.c = sub i32 64, %pos
  %len.f = select i1 %end.big, i32 %len.c, i32 %len
  %pos64 = zext i32 %pos to i64
  %len64 = zext i32 %len.f to i64
  %up0 = sub i64 64, %pos64
  %up = sub i64 %up0, %len64
  %top = shl i64 %base, %up
  %down = sub i64 64, %len64
  %field = ashr i64 %top, %down
  %r0 = select i1 %pos.big, i64 %sign, i64 %field
  %r = select i1 %len.zero, i64 0, i64 %r0
  ret i64 %r
}

define i32 @__zluda_ptx_impl_bfi_b32(i32 %insert, i32 %base, i32 %pos32, i32 %len32) #0 {
  %pos = and i32 %pos32, 255
  %len = and i32 %len32, 255
  %pos.big = icmp uge i32 %pos, 32
  %len.big = icmp uge i32 %len, 32
  %bits.s = shl i32 1, %len
  %bits = sub i32 %bits.s, 1
  %bits.f = select i1 %len.big, i32 -1, i32 %bits
  %mask = shl i32 %bits.f, %pos
  %ins = shl i32 %insert, %pos
  %nmask = xor i32 %mask, -1
  %keep = and i32 %base, %nmask
  %put = and i32 %ins, %mask
  %merged = or i32 %keep, %put
  %r = select i1 %pos.big, i32 %base, i32 %merged
  ret i32 %r
}

define i64 @__zluda_ptx_impl_bfi_b64(i64 %insert, i64 %base, i32 %pos, i32 %len) #0 {
  %pos.big = icmp uge i32 %pos, 64
  %len.big = icmp uge i32 %len, 64
  %pos64 = zext i32 %pos to i64
  %len64 = zext i32 %len to i64
  %bits.s = shl i64 1, %len64
  %bits = sub i64 %bits.s, 1
  %bits.f = select i1 %len.big, i64 -1, i64 %bits
  %mask = shl i64 %bits.f, %pos64
  %ins = shl i64 %insert, %pos64
  %nmask = xor i64 %mask, -1
  %keep = and i64 %base, %nmask
  %put = and i64 %ins, %mask
  %merged = or i64 %keep, %put
  %r = select i1 %pos.big, i64 %base, i64 %merged
  ret i64 %r
}

; bmsk.clamp.b32: the low bytes of position and width, clamped to 32.
define i32 @__zluda_ptx_impl_bmsk_clamp_b32(i32 %position, i32 %width) #0 {
  %p8 = and i32 %position, 255
  %wd8 = and i32 %width, 255
  %p32 = call i32 @llvm.umin.i32(i32 %p8, i32 32)
  %wd32 = call i32 @llvm.umin.i32(i32 %wd8, i32 32)
  %p = zext i32 %p32 to i64
  %wd = zext i32 %wd32 to i64
  %bits.s = shl i64 1, %wd
  %bits = sub i64 %bits.s, 1
  %m64 = shl i64 %bits, %p
  %m = trunc i64 %m64 to i32
  ret i32 %m
}

; Barriers. bar.sync orders threadgroup memory as well.
define void @__zluda_ptx_impl_bar_sync(i32 %barrier) #1 {
  fence syncscope("workgroup") seq_cst
  call void @llvm.agx.threadgroup.barrier()
  ret void
}

; Clusters of several CTAs run as one threadgroup (llvm_zluda's
; LLVMZludaAgxLowerClusters). These placeholders are resolved there for each
; kernel, from its agx-cluster-dims: __zluda_agx_cluster(which) reads
; %cluster_ctaid.{x,y,z} (0-2), %cluster_nctaid (3-5), %cluster_ctarank (6),
; %cluster_nctarank (7), %clusterid (8-10), %nclusterid (11-13) and
; %is_explicit_cluster (14); mapa and rank_of take 32-bit shared addresses.
declare i32 @__zluda_agx_cluster(i32) #4
declare i32 @__zluda_agx_cluster_mapa(i32, i32) #4
declare i32 @__zluda_agx_cluster_rank_of(i32) #4

define i32 @__zluda_ptx_impl_sreg_cluster_ctaid(i8 zeroext %member) #0 {
  %m = zext i8 %member to i32
  %v = call i32 @__zluda_agx_cluster(i32 %m)
  ret i32 %v
}
define i32 @__zluda_ptx_impl_sreg_cluster_nctaid(i8 zeroext %member) #0 {
  %m = zext i8 %member to i32
  %w = add i32 %m, 3
  %v = call i32 @__zluda_agx_cluster(i32 %w)
  ret i32 %v
}
define i32 @__zluda_ptx_impl_sreg_cluster_ctarank() #0 {
  %v = call i32 @__zluda_agx_cluster(i32 6)
  ret i32 %v
}
define i32 @__zluda_ptx_impl_sreg_cluster_nctarank() #0 {
  %v = call i32 @__zluda_agx_cluster(i32 7)
  ret i32 %v
}
define i32 @__zluda_ptx_impl_sreg_clusterid(i8 zeroext %member) #0 {
  %m = zext i8 %member to i32
  %w = add i32 %m, 8
  %v = call i32 @__zluda_agx_cluster(i32 %w)
  ret i32 %v
}
define i32 @__zluda_ptx_impl_sreg_nclusterid(i8 zeroext %member) #0 {
  %m = zext i8 %member to i32
  %w = add i32 %m, 11
  %v = call i32 @__zluda_agx_cluster(i32 %w)
  ret i32 %v
}
define i32 @__zluda_ptx_impl_sreg_is_explicit_cluster() #0 {
  %v = call i32 @__zluda_agx_cluster(i32 14)
  ret i32 %v
}

define i32 @__zluda_ptx_impl_mapa_shared_u32(i32 %addr, i32 %rank) #0 {
  %v = call i32 @__zluda_agx_cluster_mapa(i32 %addr, i32 %rank)
  ret i32 %v
}
define i64 @__zluda_ptx_impl_mapa_shared_u64(i64 %addr, i32 %rank) #0 {
  %a = trunc i64 %addr to i32
  %v = call i32 @__zluda_agx_cluster_mapa(i32 %a, i32 %rank)
  %r = zext i32 %v to i64
  ret i64 %r
}
; A generic address of shared memory, mapped through its shared address
define i64 @__zluda_ptx_impl_mapa_generic_u64(i64 %addr, i32 %rank) #0 {
  %p = inttoptr i64 %addr to ptr
  %s = addrspacecast ptr %p to ptr addrspace(3)
  %a = ptrtoint ptr addrspace(3) %s to i32
  %v = call i32 @__zluda_agx_cluster_mapa(i32 %a, i32 %rank)
  %q = inttoptr i32 %v to ptr addrspace(3)
  %g = addrspacecast ptr addrspace(3) %q to ptr
  %r = ptrtoint ptr %g to i64
  ret i64 %r
}
define i32 @__zluda_ptx_impl_mapa_generic_u32(i32 %addr, i32 %rank) #0 {
  %v = call i32 @__zluda_agx_cluster_mapa(i32 %addr, i32 %rank)
  ret i32 %v
}
define i32 @__zluda_ptx_impl_getctarank_shared_u32(i32 %addr) #0 {
  %v = call i32 @__zluda_agx_cluster_rank_of(i32 %addr)
  ret i32 %v
}
define i32 @__zluda_ptx_impl_getctarank_shared_u64(i64 %addr) #0 {
  %a = trunc i64 %addr to i32
  %v = call i32 @__zluda_agx_cluster_rank_of(i32 %a)
  ret i32 %v
}
define i32 @__zluda_ptx_impl_getctarank_generic_u64(i64 %addr) #0 {
  %p = inttoptr i64 %addr to ptr
  %s = addrspacecast ptr %p to ptr addrspace(3)
  %a = ptrtoint ptr addrspace(3) %s to i32
  %v = call i32 @__zluda_agx_cluster_rank_of(i32 %a)
  ret i32 %v
}
define i32 @__zluda_ptx_impl_getctarank_generic_u32(i32 %addr) #0 {
  %v = call i32 @__zluda_agx_cluster_rank_of(i32 %addr)
  ret i32 %v
}

; Whether a generic address of shared memory is in this CTA's window
define zeroext i1 @__zluda_ptx_impl_agx_in_own_cta(i64 %addr) #0 {
  %r = call i32 @__zluda_ptx_impl_getctarank_generic_u64(i64 %addr)
  %me = call i32 @__zluda_agx_cluster(i32 6)
  %own = icmp eq i32 %r, %me
  ret i1 %own
}

; The cluster is the threadgroup: its barrier is the threadgroup barrier,
; with release and acquire ordering of shared and global memory. An arrive
; is completed by the wait that follows it.
define void @__zluda_ptx_impl_barrier_cluster_arrive() #1 {
  fence syncscope("agent") release
  ret void
}
define void @__zluda_ptx_impl_barrier_cluster_arrive_relaxed() #1 {
  ret void
}
define void @__zluda_ptx_impl_barrier_cluster_wait() #1 {
  fence syncscope("agent") seq_cst
  call void @llvm.agx.threadgroup.barrier()
  fence syncscope("agent") acquire
  ret void
}

; Threadgroup-wide reductions of a predicate, through a threadgroup variable.
@__zluda_ptx_impl_agx_bar_red = internal addrspace(3) global i32 undef

define zeroext i1 @__zluda_ptx_impl_bar_red_and_pred(i32 %barrier, i1 zeroext %pred, i1 zeroext %invert) #1 {
entry:
  %value = xor i1 %pred, %invert
  %v32 = zext i1 %value to i32
  call void @llvm.agx.threadgroup.barrier()
  %tid = call i32 @llvm.agx.thread.index.in.threadgroup()
  %first = icmp eq i32 %tid, 0
  br i1 %first, label %init, label %reduce
init:
  store i32 1, ptr addrspace(3) @__zluda_ptx_impl_agx_bar_red
  br label %reduce
reduce:
  fence syncscope("workgroup") seq_cst
  call void @llvm.agx.threadgroup.barrier()
  %old = atomicrmw and ptr addrspace(3) @__zluda_ptx_impl_agx_bar_red, i32 %v32 monotonic
  fence syncscope("workgroup") seq_cst
  call void @llvm.agx.threadgroup.barrier()
  %all = load i32, ptr addrspace(3) @__zluda_ptx_impl_agx_bar_red
  call void @llvm.agx.threadgroup.barrier()
  %r = icmp ne i32 %all, 0
  ret i1 %r
}
define zeroext i1 @__zluda_ptx_impl_bar_red_or_pred(i32 %barrier, i1 zeroext %pred, i1 zeroext %invert) #1 {
entry:
  %value = xor i1 %pred, %invert
  %v32 = zext i1 %value to i32
  call void @llvm.agx.threadgroup.barrier()
  %tid = call i32 @llvm.agx.thread.index.in.threadgroup()
  %first = icmp eq i32 %tid, 0
  br i1 %first, label %init, label %reduce
init:
  store i32 0, ptr addrspace(3) @__zluda_ptx_impl_agx_bar_red
  br label %reduce
reduce:
  fence syncscope("workgroup") seq_cst
  call void @llvm.agx.threadgroup.barrier()
  %old = atomicrmw or ptr addrspace(3) @__zluda_ptx_impl_agx_bar_red, i32 %v32 monotonic
  fence syncscope("workgroup") seq_cst
  call void @llvm.agx.threadgroup.barrier()
  %all = load i32, ptr addrspace(3) @__zluda_ptx_impl_agx_bar_red
  call void @llvm.agx.threadgroup.barrier()
  %r = icmp ne i32 %all, 0
  ret i1 %r
}
; bar.red.popc.u32: the number of threads whose predicate is true
define i32 @__zluda_ptx_impl_bar_red_popc_u32(i32 %barrier, i1 zeroext %pred, i1 zeroext %invert) #1 {
entry:
  %value = xor i1 %pred, %invert
  %v32 = zext i1 %value to i32
  call void @llvm.agx.threadgroup.barrier()
  %tid = call i32 @llvm.agx.thread.index.in.threadgroup()
  %first = icmp eq i32 %tid, 0
  br i1 %first, label %init, label %reduce
init:
  store i32 0, ptr addrspace(3) @__zluda_ptx_impl_agx_bar_red
  br label %reduce
reduce:
  fence syncscope("workgroup") seq_cst
  call void @llvm.agx.threadgroup.barrier()
  %old = atomicrmw add ptr addrspace(3) @__zluda_ptx_impl_agx_bar_red, i32 %v32 monotonic
  fence syncscope("workgroup") seq_cst
  call void @llvm.agx.threadgroup.barrier()
  %all = load i32, ptr addrspace(3) @__zluda_ptx_impl_agx_bar_red
  call void @llvm.agx.threadgroup.barrier()
  ret i32 %all
}

; bar.red with a thread count. Threads arrive at barriers in whole warps, so
; a count of 32 means that the barrier is the warp's only: the reduction
; is over the SIMD-group. Other counts are taken as the whole threadgroup.
define i32 @__zluda_ptx_impl_bar_red_popc_u32_count(i32 %barrier, i32 %count, i1 zeroext %pred, i1 zeroext %invert) #1 {
entry:
  %warp = icmp eq i32 %count, 32
  br i1 %warp, label %simd, label %cta
simd:
  %value = xor i1 %pred, %invert
  fence syncscope("workgroup") seq_cst
  %ballot = call i32 @llvm.agx.simd.ballot(i1 %value)
  %n = call i32 @llvm.ctpop.i32(i32 %ballot)
  ret i32 %n
cta:
  %r = call i32 @__zluda_ptx_impl_bar_red_popc_u32(i32 %barrier, i1 %pred, i1 %invert)
  ret i32 %r
}
define zeroext i1 @__zluda_ptx_impl_bar_red_and_pred_count(i32 %barrier, i32 %count, i1 zeroext %pred, i1 zeroext %invert) #1 {
entry:
  %warp = icmp eq i32 %count, 32
  br i1 %warp, label %simd, label %cta
simd:
  %value = xor i1 %pred, %invert
  fence syncscope("workgroup") seq_cst
  %ballot = call i32 @llvm.agx.simd.ballot(i1 %value)
  %active = call i32 @llvm.agx.simd.ballot(i1 true)
  %all = icmp eq i32 %ballot, %active
  ret i1 %all
cta:
  %r = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 %barrier, i1 %pred, i1 %invert)
  ret i1 %r
}
define zeroext i1 @__zluda_ptx_impl_bar_red_or_pred_count(i32 %barrier, i32 %count, i1 zeroext %pred, i1 zeroext %invert) #1 {
entry:
  %warp = icmp eq i32 %count, 32
  br i1 %warp, label %simd, label %cta
simd:
  %value = xor i1 %pred, %invert
  fence syncscope("workgroup") seq_cst
  %ballot = call i32 @llvm.agx.simd.ballot(i1 %value)
  %any = icmp ne i32 %ballot, 0
  ret i1 %any
cta:
  %r = call i1 @__zluda_ptx_impl_bar_red_or_pred(i32 %barrier, i1 %pred, i1 %invert)
  ret i1 %r
}

; shfl.sync: opts holds the subsection mask in bits 12..8 and the end lane
; in bits 4..0. Out of range lanes read their own value and clear the
; predicate.
define <2 x i32> @__zluda_ptx_impl_shfl_sync_up_b32_pred(i32 %input, i32 %delta0, i32 %opts, i32 %membermask) #2 {
  %sm0 = lshr i32 %opts, 8
  %section_mask = and i32 %sm0, 31
  %warp_end = and i32 %opts, 31
  %delta = and i32 %delta0, 31
  %self = call i32 @llvm.agx.thread.index.in.simdgroup()
  %subsection = and i32 %section_mask, %self
  %nsec = xor i32 %section_mask, -1
  %endbits = and i32 %nsec, %warp_end
  %subsection_end = or i32 %subsection, %endbits
  %idx = sub i32 %self, %delta
  %oob = icmp slt i32 %idx, %subsection_end
  %lane = select i1 %oob, i32 %self, i32 %idx
  %value = call i32 @llvm.agx.simd.shuffle.i32(i32 %input, i32 %lane)
  %inb = xor i1 %oob, true
  %p = zext i1 %inb to i32
  %r0 = insertelement <2 x i32> poison, i32 %value, i32 0
  %r = insertelement <2 x i32> %r0, i32 %p, i32 1
  ret <2 x i32> %r
}

define i32 @__zluda_ptx_impl_shfl_sync_up_b32(i32 %input, i32 %delta, i32 %opts, i32 %membermask) #2 {
  %r = call <2 x i32> @__zluda_ptx_impl_shfl_sync_up_b32_pred(i32 %input, i32 %delta, i32 %opts, i32 %membermask)
  %v = extractelement <2 x i32> %r, i32 0
  ret i32 %v
}
define <2 x i32> @__zluda_ptx_impl_shfl_sync_down_b32_pred(i32 %input, i32 %delta0, i32 %opts, i32 %membermask) #2 {
  %sm0 = lshr i32 %opts, 8
  %section_mask = and i32 %sm0, 31
  %warp_end = and i32 %opts, 31
  %delta = and i32 %delta0, 31
  %self = call i32 @llvm.agx.thread.index.in.simdgroup()
  %subsection = and i32 %section_mask, %self
  %nsec = xor i32 %section_mask, -1
  %endbits = and i32 %nsec, %warp_end
  %subsection_end = or i32 %subsection, %endbits
  %idx = add i32 %self, %delta
  %oob = icmp sgt i32 %idx, %subsection_end
  %lane = select i1 %oob, i32 %self, i32 %idx
  %value = call i32 @llvm.agx.simd.shuffle.i32(i32 %input, i32 %lane)
  %inb = xor i1 %oob, true
  %p = zext i1 %inb to i32
  %r0 = insertelement <2 x i32> poison, i32 %value, i32 0
  %r = insertelement <2 x i32> %r0, i32 %p, i32 1
  ret <2 x i32> %r
}

define i32 @__zluda_ptx_impl_shfl_sync_down_b32(i32 %input, i32 %delta, i32 %opts, i32 %membermask) #2 {
  %r = call <2 x i32> @__zluda_ptx_impl_shfl_sync_down_b32_pred(i32 %input, i32 %delta, i32 %opts, i32 %membermask)
  %v = extractelement <2 x i32> %r, i32 0
  ret i32 %v
}
define <2 x i32> @__zluda_ptx_impl_shfl_sync_bfly_b32_pred(i32 %input, i32 %delta0, i32 %opts, i32 %membermask) #2 {
  %sm0 = lshr i32 %opts, 8
  %section_mask = and i32 %sm0, 31
  %warp_end = and i32 %opts, 31
  %delta = and i32 %delta0, 31
  %self = call i32 @llvm.agx.thread.index.in.simdgroup()
  %subsection = and i32 %section_mask, %self
  %nsec = xor i32 %section_mask, -1
  %endbits = and i32 %nsec, %warp_end
  %subsection_end = or i32 %subsection, %endbits
  %idx = xor i32 %self, %delta
  %oob = icmp sgt i32 %idx, %subsection_end
  %lane = select i1 %oob, i32 %self, i32 %idx
  %value = call i32 @llvm.agx.simd.shuffle.i32(i32 %input, i32 %lane)
  %inb = xor i1 %oob, true
  %p = zext i1 %inb to i32
  %r0 = insertelement <2 x i32> poison, i32 %value, i32 0
  %r = insertelement <2 x i32> %r0, i32 %p, i32 1
  ret <2 x i32> %r
}

define i32 @__zluda_ptx_impl_shfl_sync_bfly_b32(i32 %input, i32 %delta, i32 %opts, i32 %membermask) #2 {
  %r = call <2 x i32> @__zluda_ptx_impl_shfl_sync_bfly_b32_pred(i32 %input, i32 %delta, i32 %opts, i32 %membermask)
  %v = extractelement <2 x i32> %r, i32 0
  ret i32 %v
}
define <2 x i32> @__zluda_ptx_impl_shfl_sync_idx_b32_pred(i32 %input, i32 %delta0, i32 %opts, i32 %membermask) #2 {
  %sm0 = lshr i32 %opts, 8
  %section_mask = and i32 %sm0, 31
  %warp_end = and i32 %opts, 31
  %delta = and i32 %delta0, 31
  %self = call i32 @llvm.agx.thread.index.in.simdgroup()
  %subsection = and i32 %section_mask, %self
  %nsec = xor i32 %section_mask, -1
  %endbits = and i32 %nsec, %warp_end
  %subsection_end = or i32 %subsection, %endbits
  %nmask = xor i32 %section_mask, -1
  %inside = and i32 %delta, %nmask
  %idx = or i32 %inside, %subsection
  %oob = icmp sgt i32 %idx, %subsection_end
  %lane = select i1 %oob, i32 %self, i32 %idx
  %value = call i32 @llvm.agx.simd.shuffle.i32(i32 %input, i32 %lane)
  %inb = xor i1 %oob, true
  %p = zext i1 %inb to i32
  %r0 = insertelement <2 x i32> poison, i32 %value, i32 0
  %r = insertelement <2 x i32> %r0, i32 %p, i32 1
  ret <2 x i32> %r
}

define i32 @__zluda_ptx_impl_shfl_sync_idx_b32(i32 %input, i32 %delta, i32 %opts, i32 %membermask) #2 {
  %r = call <2 x i32> @__zluda_ptx_impl_shfl_sync_idx_b32_pred(i32 %input, i32 %delta, i32 %opts, i32 %membermask)
  %v = extractelement <2 x i32> %r, i32 0
  ret i32 %v
}
; Warp votes. The member mask is ignored: all active threads take part.
define zeroext i1 @__zluda_ptx_impl_vote_sync_any_pred(i1 zeroext %value, i32 %membermask) #2 {
  %b = call i32 @llvm.agx.simd.ballot(i1 %value)
  %r = icmp ne i32 %b, 0
  ret i1 %r
}

define zeroext i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1 zeroext %value, i32 %membermask) #2 {
  %n = xor i1 %value, true
  %b = call i32 @llvm.agx.simd.ballot(i1 %n)
  %r = icmp ne i32 %b, 0
  ret i1 %r
}

define zeroext i1 @__zluda_ptx_impl_vote_sync_all_pred(i1 zeroext %value, i32 %membermask) #2 {
  %n = xor i1 %value, true
  %b = call i32 @llvm.agx.simd.ballot(i1 %n)
  %r = icmp eq i32 %b, 0
  ret i1 %r
}

define zeroext i1 @__zluda_ptx_impl_vote_sync_all_pred_negate(i1 zeroext %value, i32 %membermask) #2 {
  %b = call i32 @llvm.agx.simd.ballot(i1 %value)
  %r = icmp eq i32 %b, 0
  ret i1 %r
}

; uni: every active thread holds the same value (so negating it changes nothing)
define zeroext i1 @__zluda_ptx_impl_vote_sync_uni_pred(i1 zeroext %value, i32 %membermask) #2 {
  %active = call i32 @llvm.agx.simd.ballot(i1 true)
  %b = call i32 @llvm.agx.simd.ballot(i1 %value)
  %none = icmp eq i32 %b, 0
  %all = icmp eq i32 %b, %active
  %r = or i1 %none, %all
  ret i1 %r
}

define zeroext i1 @__zluda_ptx_impl_vote_sync_uni_pred_negate(i1 zeroext %value, i32 %membermask) #2 {
  %r = call i1 @__zluda_ptx_impl_vote_sync_uni_pred(i1 %value, i32 %membermask)
  ret i1 %r
}

define i32 @__zluda_ptx_impl_vote_sync_ballot_b32(i1 zeroext %value, i32 %membermask) #2 {
  %b = call i32 @llvm.agx.simd.ballot(i1 %value)
  ret i32 %b
}

define i32 @__zluda_ptx_impl_vote_sync_ballot_b32_negate(i1 zeroext %value, i32 %membermask) #2 {
  %n = xor i1 %value, true
  %b = call i32 @llvm.agx.simd.ballot(i1 %n)
  ret i32 %b
}

define i32 @__zluda_ptx_impl_vote_ballot_b32(i1 zeroext %value) #2 {
  %b = call i32 @llvm.agx.simd.ballot(i1 %value)
  ret i32 %b
}

define i32 @__zluda_ptx_impl_vote_ballot_b32_negate(i1 zeroext %value) #2 {
  %n = xor i1 %value, true
  %b = call i32 @llvm.agx.simd.ballot(i1 %n)
  ret i32 %b
}

; redux.sync over the SIMD-group (the member mask is ignored). The reduction
; operation numbers are those of llvm.agx.simd.reduce.
define i32 @__zluda_ptx_impl_redux_sync_add_u32(i32 %src, i32 %membermask) #2 {
  %r = call i32 @llvm.agx.simd.reduce.i32(i32 %src, i32 16)
  ret i32 %r
}
define i32 @__zluda_ptx_impl_redux_sync_add_s32(i32 %src, i32 %membermask) #2 {
  %r = call i32 @llvm.agx.simd.reduce.i32(i32 %src, i32 16)
  ret i32 %r
}
define i32 @__zluda_ptx_impl_redux_sync_min_u32(i32 %src, i32 %membermask) #2 {
  %r = call i32 @llvm.agx.simd.reduce.i32(i32 %src, i32 28)
  ret i32 %r
}
define i32 @__zluda_ptx_impl_redux_sync_max_u32(i32 %src, i32 %membermask) #2 {
  %r = call i32 @llvm.agx.simd.reduce.i32(i32 %src, i32 30)
  ret i32 %r
}
define i32 @__zluda_ptx_impl_redux_sync_min_s32(i32 %src, i32 %membermask) #2 {
  %r = call i32 @llvm.agx.simd.reduce.i32(i32 %src, i32 20)
  ret i32 %r
}
define i32 @__zluda_ptx_impl_redux_sync_max_s32(i32 %src, i32 %membermask) #2 {
  %r = call i32 @llvm.agx.simd.reduce.i32(i32 %src, i32 22)
  ret i32 %r
}
define i32 @__zluda_ptx_impl_redux_sync_and_b32(i32 %src, i32 %membermask) #2 {
  %r = call i32 @llvm.agx.simd.reduce.i32(i32 %src, i32 0)
  ret i32 %r
}
define i32 @__zluda_ptx_impl_redux_sync_or_b32(i32 %src, i32 %membermask) #2 {
  %r = call i32 @llvm.agx.simd.reduce.i32(i32 %src, i32 2)
  ret i32 %r
}
define i32 @__zluda_ptx_impl_redux_sync_xor_b32(i32 %src, i32 %membermask) #2 {
  %r = call i32 @llvm.agx.simd.reduce.i32(i32 %src, i32 4)
  ret i32 %r
}
; Approximate math maps to the hardware functions.
define float @__zluda_ptx_impl_sqrt_approx_f32(float %x) #0 {
  %r = call float @llvm.sqrt.f32(float %x)
  ret float %r
}

define float @__zluda_ptx_impl_rsqrt_approx_f32(float %x) #0 {
  %r = call float @llvm.agx.rsqrt.f32(float %x)
  ret float %r
}

define float @__zluda_ptx_impl_rcp_approx_f32(float %x) #0 {
  %r = call float @llvm.agx.rcp.f32(float %x)
  ret float %r
}

define float @__zluda_ptx_impl_ex2_approx_f32(float %x) #0 {
  %r = call float @llvm.exp2.f32(float %x)
  ret float %r
}

define float @__zluda_ptx_impl_lg2_approx_f32(float %x) #0 {
  %r = call float @llvm.log2.f32(float %x)
  ret float %r
}

; Not correctly rounded: AGX has no correctly rounded square root.
define float @__zluda_ptx_impl_sqrt_rn_f32(float %x) #0 {
  %r = call float @llvm.sqrt.f32(float %x)
  ret float %r
}

define float @__zluda_ptx_impl_sqrt_rn_ftz_f32(float %x) #0 {
  %r = call float @llvm.sqrt.f32(float %x)
  ret float %r
}

define float @__zluda_ptx_impl_div_full_f32(float %a, float %b) #0 {
  %r = fdiv float %a, %b
  ret float %r
}

; prmt: each selector nibble picks a byte of {b, a}; bit 3 replicates its
; sign bit.
define i32 @__zluda_ptx_impl_prmt_b32(i32 %x, i32 %y, i32 %s) #0 {
  %x64 = zext i32 %x to i64
  %y64 = zext i32 %y to i64
  %yh = shl i64 %y64, 32
  %src = or i64 %x64, %yh
  %sel0.s = lshr i32 %s, 0
  %sel0 = and i32 %sel0.s, 15
  %idx0 = and i32 %sel0, 7
  %sh0.b = shl i32 %idx0, 3
  %sh0 = zext i32 %sh0.b to i64
  %byte0.w = lshr i64 %src, %sh0
  %byte0.t = trunc i64 %byte0.w to i32
  %byte0 = and i32 %byte0.t, 255
  %rep0.f = and i32 %sel0, 8
  %rep0 = icmp ne i32 %rep0.f, 0
  %sgn0.s = shl i32 %byte0, 24
  %sgn0.a = ashr i32 %sgn0.s, 31
  %sgn0 = and i32 %sgn0.a, 255
  %b0 = select i1 %rep0, i32 %sgn0, i32 %byte0
  %p0 = shl i32 %b0, 0
  %sel1.s = lshr i32 %s, 4
  %sel1 = and i32 %sel1.s, 15
  %idx1 = and i32 %sel1, 7
  %sh1.b = shl i32 %idx1, 3
  %sh1 = zext i32 %sh1.b to i64
  %byte1.w = lshr i64 %src, %sh1
  %byte1.t = trunc i64 %byte1.w to i32
  %byte1 = and i32 %byte1.t, 255
  %rep1.f = and i32 %sel1, 8
  %rep1 = icmp ne i32 %rep1.f, 0
  %sgn1.s = shl i32 %byte1, 24
  %sgn1.a = ashr i32 %sgn1.s, 31
  %sgn1 = and i32 %sgn1.a, 255
  %b1 = select i1 %rep1, i32 %sgn1, i32 %byte1
  %p1 = shl i32 %b1, 8
  %sel2.s = lshr i32 %s, 8
  %sel2 = and i32 %sel2.s, 15
  %idx2 = and i32 %sel2, 7
  %sh2.b = shl i32 %idx2, 3
  %sh2 = zext i32 %sh2.b to i64
  %byte2.w = lshr i64 %src, %sh2
  %byte2.t = trunc i64 %byte2.w to i32
  %byte2 = and i32 %byte2.t, 255
  %rep2.f = and i32 %sel2, 8
  %rep2 = icmp ne i32 %rep2.f, 0
  %sgn2.s = shl i32 %byte2, 24
  %sgn2.a = ashr i32 %sgn2.s, 31
  %sgn2 = and i32 %sgn2.a, 255
  %b2 = select i1 %rep2, i32 %sgn2, i32 %byte2
  %p2 = shl i32 %b2, 16
  %sel3.s = lshr i32 %s, 12
  %sel3 = and i32 %sel3.s, 15
  %idx3 = and i32 %sel3, 7
  %sh3.b = shl i32 %idx3, 3
  %sh3 = zext i32 %sh3.b to i64
  %byte3.w = lshr i64 %src, %sh3
  %byte3.t = trunc i64 %byte3.w to i32
  %byte3 = and i32 %byte3.t, 255
  %rep3.f = and i32 %sel3, 8
  %rep3 = icmp ne i32 %rep3.f, 0
  %sgn3.s = shl i32 %byte3, 24
  %sgn3.a = ashr i32 %sgn3.s, 31
  %sgn3 = and i32 %sgn3.a, 255
  %b3 = select i1 %rep3, i32 %sgn3, i32 %byte3
  %p3 = shl i32 %b3, 24
  %o01 = or i32 %p0, %p1
  %o012 = or i32 %o01, %p2
  %r = or i32 %o012, %p3
  ret i32 %r
}

; szext.mode.type: the low n bits of a, sign- or zero-extended, n = b mod 32 (wrap) or min(b, 32) (clamp); 0 for
; n = 0.
define i32 @__zluda_ptx_impl_szext_wrap_u32(i32 %a, i32 %b) #0 {
  %n = and i32 %b, 31
  %r = call i32 @__zluda_szext(i32 %a, i32 %n, i1 false)
  ret i32 %r
}

define i32 @__zluda_ptx_impl_szext_wrap_s32(i32 %a, i32 %b) #0 {
  %n = and i32 %b, 31
  %r = call i32 @__zluda_szext(i32 %a, i32 %n, i1 true)
  ret i32 %r
}

define i32 @__zluda_ptx_impl_szext_clamp_u32(i32 %a, i32 %b) #0 {
  %n = call i32 @llvm.umin.i32(i32 %b, i32 32)
  %r = call i32 @__zluda_szext(i32 %a, i32 %n, i1 false)
  ret i32 %r
}

define i32 @__zluda_ptx_impl_szext_clamp_s32(i32 %a, i32 %b) #0 {
  %n = call i32 @llvm.umin.i32(i32 %b, i32 32)
  %r = call i32 @__zluda_szext(i32 %a, i32 %n, i1 true)
  ret i32 %r
}

define internal i32 @__zluda_szext(i32 %a, i32 %n, i1 %signed) #0 {
  %zero = icmp eq i32 %n, 0
  %sh = sub i32 32, %n
  ; n = 32 shifts by 0; n = 0 is handled below (a shift by 32 is poison)
  %shc = and i32 %sh, 31
  %up = shl i32 %a, %shc
  %sx = ashr i32 %up, %shc
  %zx = lshr i32 %up, %shc
  %v = select i1 %signed, i32 %sx, i32 %zx
  %r = select i1 %zero, i32 0, i32 %v
  ret i32 %r
}

; fns.b32: the position of the offset-th set bit of mask counting from base (inclusive) toward the most significant
; bit for a positive offset and toward bit 0 for a negative one; for offset 0, base when its bit is set; 0xffffffff
; when there is no such bit.
define i32 @__zluda_ptx_impl_fns_b32(i32 %mask, i32 %base0, i32 %offset) #0 {
entry:
  %base = and i32 %base0, 31
  %is_zero = icmp eq i32 %offset, 0
  br i1 %is_zero, label %zero, label %search
zero:
  %bit0 = lshr i32 %mask, %base
  %set0 = trunc i32 %bit0 to i1
  %r0 = select i1 %set0, i32 %base, i32 -1
  ret i32 %r0
search:
  %neg = icmp slt i32 %offset, 0
  %noff = sub i32 0, %offset
  %n = select i1 %neg, i32 %noff, i32 %offset
  %step = select i1 %neg, i32 -1, i32 1
  br label %loop
loop:
  %i = phi i32 [ %base, %search ], [ %next, %cont ]
  %left = phi i32 [ %n, %search ], [ %left2, %cont ]
  %bit = lshr i32 %mask, %i
  %set = trunc i32 %bit to i1
  %dec = zext i1 %set to i32
  %left2 = sub i32 %left, %dec
  %found = icmp eq i32 %left2, 0
  br i1 %found, label %done, label %cont
cont:
  %next = add i32 %i, %step
  %inside = icmp ult i32 %next, 32
  br i1 %inside, label %loop, label %missing
done:
  ret i32 %i
missing:
  ret i32 -1
}

; prmt with a mode: c[1:0] picks one of four selectors (PTX's table for the mode, packed 16 bits each),
; which then permutes like the default mode.
define i32 @__zluda_ptx_impl_prmt_f4e_b32(i32 %x, i32 %y, i32 %c) #0 {
  %i = and i32 %c, 3
  %sh = shl i32 %i, 4
  %sh64 = zext i32 %sh to i64
  %t = lshr i64 7296768396121682448, %sh64
  %t32 = trunc i64 %t to i32
  %sel = and i32 %t32, 65535
  %r = call i32 @__zluda_ptx_impl_prmt_b32(i32 %x, i32 %y, i32 %sel)
  ret i32 %r
}
define i32 @__zluda_ptx_impl_prmt_b4e_b32(i32 %x, i32 %y, i32 %c) #0 {
  %i = and i32 %c, 3
  %sh = shl i32 %i, 4
  %sh64 = zext i32 %sh to i64
  %t = lshr i64 82032442562664048, %sh64
  %t32 = trunc i64 %t to i32
  %sel = and i32 %t32, 65535
  %r = call i32 @__zluda_ptx_impl_prmt_b32(i32 %x, i32 %y, i32 %sel)
  ret i32 %r
}
define i32 @__zluda_ptx_impl_prmt_rc8_b32(i32 %x, i32 %y, i32 %c) #0 {
  %i = and i32 %c, 3
  %sh = shl i32 %i, 4
  %sh64 = zext i32 %sh to i64
  %t = lshr i64 3689330049457127424, %sh64
  %t32 = trunc i64 %t to i32
  %sel = and i32 %t32, 65535
  %r = call i32 @__zluda_ptx_impl_prmt_b32(i32 %x, i32 %y, i32 %sel)
  ret i32 %r
}
define i32 @__zluda_ptx_impl_prmt_ecl_b32(i32 %x, i32 %y, i32 %c) #0 {
  %i = and i32 %c, 3
  %sh = shl i32 %i, 4
  %sh64 = zext i32 %sh to i64
  %t = lshr i64 3689347642196832784, %sh64
  %t32 = trunc i64 %t to i32
  %sel = and i32 %t32, 65535
  %r = call i32 @__zluda_ptx_impl_prmt_b32(i32 %x, i32 %y, i32 %sel)
  ret i32 %r
}
define i32 @__zluda_ptx_impl_prmt_ecr_b32(i32 %x, i32 %y, i32 %c) #0 {
  %i = and i32 %c, 3
  %sh = shl i32 %i, 4
  %sh64 = zext i32 %sh to i64
  %t = lshr i64 3607420753924849664, %sh64
  %t32 = trunc i64 %t to i32
  %sel = and i32 %t32, 65535
  %r = call i32 @__zluda_ptx_impl_prmt_b32(i32 %x, i32 %y, i32 %sel)
  ret i32 %r
}
define i32 @__zluda_ptx_impl_prmt_rc16_b32(i32 %x, i32 %y, i32 %c) #0 {
  %i = and i32 %c, 3
  %sh = shl i32 %i, 4
  %sh64 = zext i32 %sh to i64
  %t = lshr i64 3616971112479592464, %sh64
  %t32 = trunc i64 %t to i32
  %sel = and i32 %t32, 65535
  %r = call i32 @__zluda_ptx_impl_prmt_b32(i32 %x, i32 %y, i32 %sel)
  ret i32 %r
}

define void @__zluda_ptx_impl_nanosleep_u32(i32 %nanoseconds) #0 {
  ret void
}

; vprintf: appends a record to @__zluda_agx_printf_buffer for the host to
; print. The buffer starts with the number of bytes of records (updated
; atomically; records whose end would pass the end of the buffer are not
; written), followed by the records from byte 8 on. A record is
;   u32 size     (of the whole record, a multiple of 8)
;   u32 argsize  (bytes of arguments)
;   u64 format   (device address of the format string)
;   argsize bytes of arguments, as laid out by the caller (each argument
;   aligned to its size: 4 bytes for %c, %d, %i, %o, %u, %x, %X without l,
;   8 bytes for the l and ll versions, %p, %s and the floating point ones)
; The argument size is found by scanning the format string.
@__zluda_agx_printf_buffer = internal addrspace(1) global [65536 x i8] zeroinitializer, align 8

define i32 @__zluda_ptx_impl_vprintf(i64 %format, i64 %args) #3 {
entry:
  br label %scan
scan:
  %i = phi i64 [ 0, %entry ], [ %i.next, %next ]
  %in.spec = phi i1 [ false, %entry ], [ %in.spec.next, %next ]
  %long = phi i1 [ false, %entry ], [ %long.next, %next ]
  %size = phi i32 [ 0, %entry ], [ %size.next, %next ]
  %count = phi i32 [ 0, %entry ], [ %count.next, %next ]
  %p = add i64 %format, %i
  %pp = inttoptr i64 %p to ptr
  %ch = load i8, ptr %pp, align 1
  %done = icmp eq i8 %ch, 0
  br i1 %done, label %write, label %char
char:
  %is.percent = icmp eq i8 %ch, 37
  br i1 %in.spec, label %spec, label %plain
plain:
  br label %next
spec:
  ; the argument size of a conversion character, 0 for other characters
  %is.l = icmp eq i8 %ch, 108
  %is.z = icmp eq i8 %ch, 122
  %is.j = icmp eq i8 %ch, 106
  %is.t = icmp eq i8 %ch, 116
  %is.L = icmp eq i8 %ch, 76
  %l1 = or i1 %is.l, %is.z
  %l2 = or i1 %l1, %is.j
  %l3 = or i1 %l2, %is.t
  %is.length8 = or i1 %l3, %is.L
  %c.lower = or i8 %ch, 32
  %is.d = icmp eq i8 %ch, 100
  %is.i = icmp eq i8 %ch, 105
  %is.o = icmp eq i8 %ch, 111
  %is.u = icmp eq i8 %ch, 117
  %is.x = icmp eq i8 %c.lower, 120
  %is.c = icmp eq i8 %ch, 99
  %n1 = or i1 %is.d, %is.i
  %n2 = or i1 %n1, %is.o
  %n3 = or i1 %n2, %is.u
  %n4 = or i1 %n3, %is.x
  %is.integer = or i1 %n4, %is.c
  %is.f = icmp eq i8 %c.lower, 102
  %is.e = icmp eq i8 %c.lower, 101
  %is.g = icmp eq i8 %c.lower, 103
  %is.a = icmp eq i8 %c.lower, 97
  %is.s = icmp eq i8 %ch, 115
  %is.p = icmp eq i8 %ch, 112
  %f1 = or i1 %is.f, %is.e
  %f2 = or i1 %f1, %is.g
  %f3 = or i1 %f2, %is.a
  %f4 = or i1 %f3, %is.s
  %is.wide = or i1 %f4, %is.p
  %is.star = icmp eq i8 %ch, 42
  %integer.size = select i1 %long, i32 8, i32 4
  %s1 = select i1 %is.integer, i32 %integer.size, i32 0
  %s2 = select i1 %is.wide, i32 8, i32 %s1
  %arg.size = select i1 %is.star, i32 4, i32 %s2
  %is.conversion0 = or i1 %is.integer, %is.wide
  %is.conversion = or i1 %is.conversion0, %is.percent
  ; align the arguments to their size
  %align.mask = sub i32 %arg.size, 1
  %size.up = add i32 %size, %align.mask
  %align.not = xor i32 %align.mask, -1
  %aligned = and i32 %size.up, %align.not
  %has.arg = icmp ne i32 %arg.size, 0
  %aligned.size = select i1 %has.arg, i32 %aligned, i32 %size
  %spec.size = add i32 %aligned.size, %arg.size
  %spec.count.inc = zext i1 %is.conversion0 to i32
  %spec.count = add i32 %count, %spec.count.inc
  %spec.long = or i1 %long, %is.length8
  %spec.continues = xor i1 %is.conversion, true
  br label %next
next:
  %in.spec.next = phi i1 [ %is.percent, %plain ], [ %spec.continues, %spec ]
  %long.next = phi i1 [ false, %plain ], [ %spec.long, %spec ]
  %size.next = phi i32 [ %size, %plain ], [ %spec.size, %spec ]
  %count.next = phi i32 [ %count, %plain ], [ %spec.count, %spec ]
  %i.next = add i64 %i, 1
  br label %scan
write:
  ; record: 16 bytes of header and the arguments, rounded up to 8
  %record.args = add i32 %size, 23
  %record.size = and i32 %record.args, -8
  %start = atomicrmw add ptr addrspace(1) @__zluda_agx_printf_buffer, i32 %record.size monotonic
  %end = add i32 %start, %record.size
  %fits = icmp ule i32 %end, 65528
  br i1 %fits, label %header, label %exit
header:
  %start64 = zext i32 %start to i64
  %offset = add i64 %start64, 8
  %record = getelementptr inbounds i8, ptr addrspace(1) @__zluda_agx_printf_buffer, i64 %offset
  store i32 %record.size, ptr addrspace(1) %record, align 8
  %argsize.p = getelementptr inbounds i8, ptr addrspace(1) %record, i64 4
  store i32 %size, ptr addrspace(1) %argsize.p, align 4
  %format.p = getelementptr inbounds i8, ptr addrspace(1) %record, i64 8
  store i64 %format, ptr addrspace(1) %format.p, align 8
  %data = getelementptr inbounds i8, ptr addrspace(1) %record, i64 16
  %words = lshr i32 %size, 2
  br label %copy
copy:
  %w = phi i32 [ 0, %header ], [ %w.next, %copy.word ]
  %copy.done = icmp uge i32 %w, %words
  br i1 %copy.done, label %exit, label %copy.word
copy.word:
  %w64 = zext i32 %w to i64
  %byte = shl i64 %w64, 2
  %src.int = add i64 %args, %byte
  %src = inttoptr i64 %src.int to ptr
  %word = load i32, ptr %src, align 4
  %dst = getelementptr inbounds i8, ptr addrspace(1) %data, i64 %byte
  store i32 %word, ptr addrspace(1) %dst, align 4
  %w.next = add i32 %w, 1
  br label %copy
exit:
  ret i32 %count
}

; __assertfail: prints CUDA's assertion message through the printf buffer,
; sets @__zluda_agx_trap_flag for the host (CUDA_ERROR_ASSERT) and traps. The
; record holds file, line, function, block and thread indices and message,
; laid out as vprintf would for @__zluda_agx_assert_format.
@__zluda_agx_assert_format = internal addrspace(1) constant [73 x i8] c"%s:%u: %s: block: [%u,%u,%u], thread: [%u,%u,%u] Assertion `%s` failed.\0A\00", align 8
@__zluda_agx_trap_flag = internal addrspace(1) global i32 0, align 4

define void @__zluda_ptx_impl___assertfail(i64 %message, i64 %file, i32 %line, i64 %function, i64 %char_size) #1 {
entry:
  store volatile i32 1, ptr addrspace(1) @__zluda_agx_trap_flag, align 4
  %start = atomicrmw add ptr addrspace(1) @__zluda_agx_printf_buffer, i32 72 monotonic
  %end = add i32 %start, 72
  %fits = icmp ule i32 %end, 65528
  br i1 %fits, label %write, label %trap
write:
  %start64 = zext i32 %start to i64
  %offset = add i64 %start64, 8
  %r = getelementptr inbounds i8, ptr addrspace(1) @__zluda_agx_printf_buffer, i64 %offset
  store i32 72, ptr addrspace(1) %r, align 8
  %r4 = getelementptr inbounds i8, ptr addrspace(1) %r, i64 4
  store i32 56, ptr addrspace(1) %r4, align 4
  %r8 = getelementptr inbounds i8, ptr addrspace(1) %r, i64 8
  %fmt = ptrtoint ptr addrspace(1) @__zluda_agx_assert_format to i64
  store i64 %fmt, ptr addrspace(1) %r8, align 8
  %a0 = getelementptr inbounds i8, ptr addrspace(1) %r, i64 16
  store i64 %file, ptr addrspace(1) %a0, align 8
  %a8 = getelementptr inbounds i8, ptr addrspace(1) %r, i64 24
  store i32 %line, ptr addrspace(1) %a8, align 4
  %a16 = getelementptr inbounds i8, ptr addrspace(1) %r, i64 32
  store i64 %function, ptr addrspace(1) %a16, align 8
  %bx = call i32 @llvm.agx.threadgroup.position.in.grid.x()
  %by = call i32 @llvm.agx.threadgroup.position.in.grid.y()
  %bz = call i32 @llvm.agx.threadgroup.position.in.grid.z()
  %tx = call i32 @llvm.agx.thread.position.in.threadgroup.x()
  %ty = call i32 @llvm.agx.thread.position.in.threadgroup.y()
  %tz = call i32 @llvm.agx.thread.position.in.threadgroup.z()
  %a24 = getelementptr inbounds i8, ptr addrspace(1) %r, i64 40
  store i32 %bx, ptr addrspace(1) %a24, align 4
  %a28 = getelementptr inbounds i8, ptr addrspace(1) %r, i64 44
  store i32 %by, ptr addrspace(1) %a28, align 4
  %a32 = getelementptr inbounds i8, ptr addrspace(1) %r, i64 48
  store i32 %bz, ptr addrspace(1) %a32, align 4
  %a36 = getelementptr inbounds i8, ptr addrspace(1) %r, i64 52
  store i32 %tx, ptr addrspace(1) %a36, align 4
  %a40 = getelementptr inbounds i8, ptr addrspace(1) %r, i64 56
  store i32 %ty, ptr addrspace(1) %a40, align 4
  %a44 = getelementptr inbounds i8, ptr addrspace(1) %r, i64 60
  store i32 %tz, ptr addrspace(1) %a44, align 4
  %a48 = getelementptr inbounds i8, ptr addrspace(1) %r, i64 64
  store i64 %message, ptr addrspace(1) %a48, align 8
  br label %trap
trap:
  ; The stores must reach memory before the SIMD-group ends.
  fence release
  call void @llvm.trap()
  unreachable
}


; div.rn: the AMDGPU implementation splits the division into a scaled
; approximation and a fixup. Here the first part does the whole division
; (a reciprocal refined with a Newton-Raphson step) and the second returns it.
%struct.f32.f32.f32.i8 = type { float, float, float, i8 }

define %struct.f32.f32.f32.i8 @__zluda_ptx_impl_div_f32_part1(float %a, float %b) #0 {
  %q = fdiv float %a, %b
  %r = insertvalue %struct.f32.f32.f32.i8 zeroinitializer, float %q, 2
  ret %struct.f32.f32.f32.i8 %r
}

define float @__zluda_ptx_impl_div_f32_part2(float %x, float %y, float %fma4, float %fma1, float %fma3, i8 %flag) #0 {
  ret float %fma3
}

; tanh(x) = sign(x) * (1 - e) / (1 + e) with e = exp(-2|x|), which cannot
; overflow.
define float @__zluda_ptx_impl_tanh_f32(float %x) #0 {
  %ax = call float @llvm.fabs.f32(float %x)
  %s = fmul float %ax, 0xC007154760000000
  %e = call float @llvm.exp2.f32(float %s)
  %num = fsub float 1.0, %e
  %den = fadd float 1.0, %e
  %t = fdiv float %num, %den
  %r = call float @llvm.copysign.f32(float %t, float %x)
  ret float %r
}

define half @__zluda_ptx_impl_tanh_f16(half %x) #0 {
  %f = fpext half %x to float
  %t = call float @__zluda_ptx_impl_tanh_f32(float %f)
  %r = fptrunc float %t to half
  ret half %r
}

; dp2a: the two 16-bit halves of a times two bytes of b (bytes 0-1 for lo,
; 2-3 for hi), plus c.
define i32 @__zluda_ptx_impl_dp2a_lo_u32_u32(i32 %a, i32 %b, i32 %c) #0 {
  %a0.t = shl i32 %a, 16
  %a0 = lshr i32 %a0.t, 16
  %a1 = lshr i32 %a, 16
  %b0.t = shl i32 %b, 24
  %b0 = lshr i32 %b0.t, 24
  %b1.t = shl i32 %b, 16
  %b1 = lshr i32 %b1.t, 24
  %p0 = mul i32 %a0, %b0
  %p1 = mul i32 %a1, %b1
  %s = add i32 %p0, %p1
  %r = add i32 %s, %c
  ret i32 %r
}
define i32 @__zluda_ptx_impl_dp2a_lo_s32_s32(i32 %a, i32 %b, i32 %c) #0 {
  %a0.t = shl i32 %a, 16
  %a0 = ashr i32 %a0.t, 16
  %a1 = ashr i32 %a, 16
  %b0.t = shl i32 %b, 24
  %b0 = ashr i32 %b0.t, 24
  %b1.t = shl i32 %b, 16
  %b1 = ashr i32 %b1.t, 24
  %p0 = mul i32 %a0, %b0
  %p1 = mul i32 %a1, %b1
  %s = add i32 %p0, %p1
  %r = add i32 %s, %c
  ret i32 %r
}
define i32 @__zluda_ptx_impl_dp2a_lo_s32_u32(i32 %a, i32 %b, i32 %c) #0 {
  %a0.t = shl i32 %a, 16
  %a0 = ashr i32 %a0.t, 16
  %a1 = ashr i32 %a, 16
  %b0.t = shl i32 %b, 24
  %b0 = lshr i32 %b0.t, 24
  %b1.t = shl i32 %b, 16
  %b1 = lshr i32 %b1.t, 24
  %p0 = mul i32 %a0, %b0
  %p1 = mul i32 %a1, %b1
  %s = add i32 %p0, %p1
  %r = add i32 %s, %c
  ret i32 %r
}
define i32 @__zluda_ptx_impl_dp2a_lo_u32_s32(i32 %a, i32 %b, i32 %c) #0 {
  %a0.t = shl i32 %a, 16
  %a0 = lshr i32 %a0.t, 16
  %a1 = lshr i32 %a, 16
  %b0.t = shl i32 %b, 24
  %b0 = ashr i32 %b0.t, 24
  %b1.t = shl i32 %b, 16
  %b1 = ashr i32 %b1.t, 24
  %p0 = mul i32 %a0, %b0
  %p1 = mul i32 %a1, %b1
  %s = add i32 %p0, %p1
  %r = add i32 %s, %c
  ret i32 %r
}
define i32 @__zluda_ptx_impl_dp2a_hi_u32_u32(i32 %a, i32 %b, i32 %c) #0 {
  %a0.t = shl i32 %a, 16
  %a0 = lshr i32 %a0.t, 16
  %a1 = lshr i32 %a, 16
  %b0.t = shl i32 %b, 8
  %b0 = lshr i32 %b0.t, 24
  %b1.t = shl i32 %b, 0
  %b1 = lshr i32 %b1.t, 24
  %p0 = mul i32 %a0, %b0
  %p1 = mul i32 %a1, %b1
  %s = add i32 %p0, %p1
  %r = add i32 %s, %c
  ret i32 %r
}
define i32 @__zluda_ptx_impl_dp2a_hi_s32_s32(i32 %a, i32 %b, i32 %c) #0 {
  %a0.t = shl i32 %a, 16
  %a0 = ashr i32 %a0.t, 16
  %a1 = ashr i32 %a, 16
  %b0.t = shl i32 %b, 8
  %b0 = ashr i32 %b0.t, 24
  %b1.t = shl i32 %b, 0
  %b1 = ashr i32 %b1.t, 24
  %p0 = mul i32 %a0, %b0
  %p1 = mul i32 %a1, %b1
  %s = add i32 %p0, %p1
  %r = add i32 %s, %c
  ret i32 %r
}
define i32 @__zluda_ptx_impl_dp2a_hi_s32_u32(i32 %a, i32 %b, i32 %c) #0 {
  %a0.t = shl i32 %a, 16
  %a0 = ashr i32 %a0.t, 16
  %a1 = ashr i32 %a, 16
  %b0.t = shl i32 %b, 8
  %b0 = lshr i32 %b0.t, 24
  %b1.t = shl i32 %b, 0
  %b1 = lshr i32 %b1.t, 24
  %p0 = mul i32 %a0, %b0
  %p1 = mul i32 %a1, %b1
  %s = add i32 %p0, %p1
  %r = add i32 %s, %c
  ret i32 %r
}
define i32 @__zluda_ptx_impl_dp2a_hi_u32_s32(i32 %a, i32 %b, i32 %c) #0 {
  %a0.t = shl i32 %a, 16
  %a0 = lshr i32 %a0.t, 16
  %a1 = lshr i32 %a, 16
  %b0.t = shl i32 %b, 8
  %b0 = ashr i32 %b0.t, 24
  %b1.t = shl i32 %b, 0
  %b1 = ashr i32 %b1.t, 24
  %p0 = mul i32 %a0, %b0
  %p1 = mul i32 %a1, %b1
  %s = add i32 %p0, %p1
  %r = add i32 %s, %c
  ret i32 %r
}
; match.any.sync: the active lanes holding the same value.
define i32 @__zluda_ptx_impl_match_any_sync_b32(i32 %value, i32 %membermask) #2 {
entry:
  %active = call i32 @llvm.agx.simd.ballot(i1 true)
  br label %loop
loop:
  %i = phi i32 [0, %entry], [%i.next, %loop]
  %mask = phi i32 [0, %entry], [%mask.next, %loop]
  %other = call i32 @llvm.agx.simd.shuffle.i32(i32 %value, i32 %i)
  %same = icmp eq i32 %other, %value
  %bit = shl i32 1, %i
  %set = or i32 %mask, %bit
  %mask.next = select i1 %same, i32 %set, i32 %mask
  %i.next = add i32 %i, 1
  %done = icmp eq i32 %i.next, 32
  br i1 %done, label %exit, label %loop
exit:
  %r = and i32 %mask.next, %active
  ret i32 %r
}

define i32 @__zluda_ptx_impl_match_any_sync_b64(i64 %value, i32 %membermask) #2 {
entry:
  %active = call i32 @llvm.agx.simd.ballot(i1 true)
  %lo = trunc i64 %value to i32
  %hi.s = lshr i64 %value, 32
  %hi = trunc i64 %hi.s to i32
  br label %loop
loop:
  %i = phi i32 [0, %entry], [%i.next, %loop]
  %mask = phi i32 [0, %entry], [%mask.next, %loop]
  %olo = call i32 @llvm.agx.simd.shuffle.i32(i32 %lo, i32 %i)
  %ohi = call i32 @llvm.agx.simd.shuffle.i32(i32 %hi, i32 %i)
  %slo = icmp eq i32 %olo, %lo
  %shi = icmp eq i32 %ohi, %hi
  %same = and i1 %slo, %shi
  %bit = shl i32 1, %i
  %set = or i32 %mask, %bit
  %mask.next = select i1 %same, i32 %set, i32 %mask
  %i.next = add i32 %i, 1
  %done = icmp eq i32 %i.next, 32
  br i1 %done, label %exit, label %loop
exit:
  %r = and i32 %mask.next, %active
  ret i32 %r
}

; mbarrier objects (8 bytes of threadgroup memory), updated with 32-bit
; atomics:
;   word 0: bit 31 is the parity of the current phase; bits 0-30 hold, modulo
;           2^31, pending arrivals * 2^19 + pending transaction bytes (a
;           19-bit signed count). They are zero exactly when both counts are.
;   word 1: the expected arrival count.
; The update that brings the counts to zero completes the phase: it flips the
; parity and reloads the pending arrivals with the expected count, in the same
; compare-and-swap. The state an arrive returns is the parity of its phase
; (bit 0) and the pending arrivals before it (bits 32-51).
define void @__zluda_ptx_impl_agx_mbarrier_init(ptr addrspace(3) %mbar, i32 %count) #3 {
  %expected.ptr = getelementptr inbounds i8, ptr addrspace(3) %mbar, i32 4
  %arrivals = shl i32 %count, 19
  %word0 = and i32 %arrivals, 2147483647
  store i32 %word0, ptr addrspace(3) %mbar, align 4
  store i32 %count, ptr addrspace(3) %expected.ptr, align 4
  ret void
}

define void @__zluda_ptx_impl_agx_mbarrier_inval(ptr addrspace(3) %mbar) #3 {
  store i64 0, ptr addrspace(3) %mbar, align 8
  ret void
}

; Adds %delta to the counts of word 0 and returns its previous value.
define i32 @__zluda_ptx_impl_agx_mbarrier_update(ptr addrspace(3) %mbar, i32 %delta) #3 {
entry:
  %expected.ptr = getelementptr inbounds i8, ptr addrspace(3) %mbar, i32 4
  fence syncscope("workgroup") release
  %first = load atomic i32, ptr addrspace(3) %mbar syncscope("workgroup") monotonic, align 4
  br label %loop
loop:
  %old = phi i32 [ %first, %entry ], [ %seen, %loop ]
  %sum = add i32 %old, %delta
  %counts = and i32 %sum, 2147483647
  %phase = and i32 %old, -2147483648
  %stay = or i32 %phase, %counts
  %expected = load atomic i32, ptr addrspace(3) %expected.ptr syncscope("workgroup") monotonic, align 4
  %reload = shl i32 %expected, 19
  %reload.counts = and i32 %reload, 2147483647
  %next.phase = xor i32 %phase, -2147483648
  %next = or i32 %next.phase, %reload.counts
  %complete = icmp eq i32 %counts, 0
  %new = select i1 %complete, i32 %next, i32 %stay
  %pair = cmpxchg ptr addrspace(3) %mbar, i32 %old, i32 %new syncscope("workgroup") acq_rel monotonic, align 4
  %seen = extractvalue { i32, i1 } %pair, 0
  %ok = extractvalue { i32, i1 } %pair, 1
  br i1 %ok, label %exit, label %loop
exit:
  fence syncscope("workgroup") acquire
  ret i32 %old
}

define i64 @__zluda_ptx_impl_agx_mbarrier_arrive(ptr addrspace(3) %mbar, i32 %count, i32 %tx) #3 {
  %arrivals = shl i32 %count, 19
  %delta = sub i32 %tx, %arrivals
  %old = call i32 @__zluda_ptx_impl_agx_mbarrier_update(ptr addrspace(3) %mbar, i32 %delta)
  %phase = lshr i32 %old, 31
  %counts = and i32 %old, 2147483647
  %tx.shl = shl i32 %old, 13
  %pending.tx = ashr i32 %tx.shl, 13
  %pending.shl = sub i32 %counts, %pending.tx
  %pending.arrivals = lshr i32 %pending.shl, 19
  %pending = and i32 %pending.arrivals, 4095
  %pending64 = zext i32 %pending to i64
  %pending.state = shl i64 %pending64, 32
  %phase64 = zext i32 %phase to i64
  %state = or i64 %pending.state, %phase64
  ret i64 %state
}

; arrive_drop also removes the arrivals from the expected count of the next phases
define i64 @__zluda_ptx_impl_agx_mbarrier_arrive_drop(ptr addrspace(3) %mbar, i32 %count, i32 %tx) #3 {
  %expected.ptr = getelementptr inbounds i8, ptr addrspace(3) %mbar, i32 4
  %expected = atomicrmw sub ptr addrspace(3) %expected.ptr, i32 %count syncscope("workgroup") monotonic, align 4
  %state = call i64 @__zluda_ptx_impl_agx_mbarrier_arrive(ptr addrspace(3) %mbar, i32 %count, i32 %tx)
  ret i64 %state
}

define void @__zluda_ptx_impl_agx_mbarrier_expect_tx(ptr addrspace(3) %mbar, i32 %tx) #3 {
  %old = call i32 @__zluda_ptx_impl_agx_mbarrier_update(ptr addrspace(3) %mbar, i32 %tx)
  ret void
}

define void @__zluda_ptx_impl_agx_mbarrier_complete_tx(ptr addrspace(3) %mbar, i32 %tx) #3 {
  %delta = sub i32 0, %tx
  %old = call i32 @__zluda_ptx_impl_agx_mbarrier_update(ptr addrspace(3) %mbar, i32 %delta)
  ret void
}

; Whether the phase of the given parity has completed.
; The .multicast::cluster::32b mbarrier operations: on the mbarrier at the
; same offset of each CTA of the cluster in the mask. op: 0 expect_tx,
; 1 complete_tx, 2 arrive (count arrivals and tx expected bytes)
define void @__zluda_ptx_impl_agx_mbarrier_multicast(ptr addrspace(3) %mbar, i32 %op, i32 %count, i32 %tx, i32 %mask) #3 {
entry:
  %mbar_a = ptrtoint ptr addrspace(3) %mbar to i32
  br label %loop
loop:
  %rank = phi i32 [ 0, %entry ], [ %next, %latch ]
  %bit = lshr i32 %mask, %rank
  %set = trunc i32 %bit to i1
  br i1 %set, label %apply, label %latch
apply:
  %bar_a = call i32 @__zluda_agx_cluster_mapa(i32 %mbar_a, i32 %rank)
  %bar = inttoptr i32 %bar_a to ptr addrspace(3)
  switch i32 %op, label %arrive [i32 0, label %expect
                                  i32 1, label %complete]
expect:
  call void @__zluda_ptx_impl_agx_mbarrier_expect_tx(ptr addrspace(3) %bar, i32 %tx)
  br label %latch
complete:
  call void @__zluda_ptx_impl_agx_mbarrier_complete_tx(ptr addrspace(3) %bar, i32 %tx)
  br label %latch
arrive:
  %state = call i64 @__zluda_ptx_impl_agx_mbarrier_arrive(ptr addrspace(3) %bar, i32 %count, i32 %tx)
  br label %latch
latch:
  %next = add i32 %rank, 1
  %done = icmp eq i32 %next, 16
  br i1 %done, label %exit, label %loop
exit:
  ret void
}

define zeroext i1 @__zluda_ptx_impl_agx_mbarrier_test_wait(ptr addrspace(3) %mbar, i32 %parity) #3 {
  %word0 = load atomic i32, ptr addrspace(3) %mbar syncscope("workgroup") monotonic, align 4
  fence syncscope("workgroup") acquire
  %phase = lshr i32 %word0, 31
  %waited = and i32 %parity, 1
  %done = icmp ne i32 %phase, %waited
  ret i1 %done
}

; try_wait waits at a threadgroup barrier between polls, so that the other
; SIMD-groups run until they wait or exit (the functional simulator only
; switches SIMD-groups at barriers), for a bounded number of polls: PTX
; programs retry while it returns false.
define zeroext i1 @__zluda_ptx_impl_agx_mbarrier_try_wait(ptr addrspace(3) %mbar, i32 %parity) #1 {
entry:
  br label %loop
loop:
  %i = phi i32 [ 0, %entry ], [ %i.next, %wait ]
  %done = call zeroext i1 @__zluda_ptx_impl_agx_mbarrier_test_wait(ptr addrspace(3) %mbar, i32 %parity)
  %i.next = add i32 %i, 1
  %more = icmp ult i32 %i.next, 4096
  %not.done = xor i1 %done, true
  %again = and i1 %not.done, %more
  br i1 %again, label %wait, label %exit
wait:
  call void @llvm.agx.threadgroup.barrier()
  br label %loop
exit:
  ret i1 %done
}

; cp.async.bulk.global.shared::cta.cp_mask: bit i of the mask selects byte i
; of every 16 bytes.
; A multicast bulk copy: the same bytes to the same shared offset of every
; CTA of the cluster in the mask, completing the transaction on each one's
; mbarrier (at the mbarrier's offset in that CTA)
define void @__zluda_ptx_impl_agx_cp_async_bulk_multicast(ptr addrspace(3) %dst, ptr addrspace(1) %src, i32 %size, ptr addrspace(3) %mbar, i32 %mask) #3 {
entry:
  %dst_a = ptrtoint ptr addrspace(3) %dst to i32
  %mbar_a = ptrtoint ptr addrspace(3) %mbar to i32
  br label %loop
loop:
  %rank = phi i32 [ 0, %entry ], [ %next, %latch ]
  %bit = lshr i32 %mask, %rank
  %set = trunc i32 %bit to i1
  br i1 %set, label %copy, label %latch
copy:
  %to_a = call i32 @__zluda_agx_cluster_mapa(i32 %dst_a, i32 %rank)
  %to = inttoptr i32 %to_a to ptr addrspace(3)
  call void @llvm.memcpy.p3.p1.i32(ptr addrspace(3) align 16 %to, ptr addrspace(1) align 16 %src, i32 %size, i1 false)
  %bar_a = call i32 @__zluda_agx_cluster_mapa(i32 %mbar_a, i32 %rank)
  %bar = inttoptr i32 %bar_a to ptr addrspace(3)
  call void @__zluda_ptx_impl_agx_mbarrier_complete_tx(ptr addrspace(3) %bar, i32 %size)
  br label %latch
latch:
  %next = add i32 %rank, 1
  %done = icmp eq i32 %next, 16
  br i1 %done, label %exit, label %loop
exit:
  ret void
}

define void @__zluda_ptx_impl_agx_cp_async_bulk_masked(ptr addrspace(1) %dst, ptr addrspace(3) %src, i32 %size, i32 %mask) #3 {
entry:
  %empty = icmp eq i32 %size, 0
  br i1 %empty, label %exit, label %loop
loop:
  %i = phi i32 [ 0, %entry ], [ %i.next, %next ]
  %lane = and i32 %i, 15
  %bit = lshr i32 %mask, %lane
  %selected = trunc i32 %bit to i1
  br i1 %selected, label %copy, label %next
copy:
  %s = getelementptr inbounds i8, ptr addrspace(3) %src, i32 %i
  %v = load i8, ptr addrspace(3) %s, align 1
  %d = getelementptr inbounds i8, ptr addrspace(1) %dst, i32 %i
  store i8 %v, ptr addrspace(1) %d, align 1
  br label %next
next:
  %i.next = add i32 %i, 1
  %more = icmp ult i32 %i.next, %size
  br i1 %more, label %loop, label %exit
exit:
  ret void
}

; cp.reduce.async.bulk: an atomic (relaxed) reduction per element. %op is the
; operation: 0 add, 1 and, 2 or, 3 xor, 4 inc, 5 dec, 6 min, 7 min.u, 8 max,
; 9 max.u, 10 add.f, 11 min.f, 12 max.f.
define void @__zluda_ptx_impl_agx_cp_reduce_bulk_b32_global(ptr addrspace(1) %dst, ptr addrspace(3) %src, i32 %size, i32 %op) #3 {
entry:
  %n = lshr i32 %size, 2
  %empty = icmp eq i32 %n, 0
  br i1 %empty, label %exit, label %loop
loop:
  %i = phi i32 [ 0, %entry ], [ %i.next, %next ]
  %s = getelementptr inbounds i32, ptr addrspace(3) %src, i32 %i
  %v = load i32, ptr addrspace(3) %s, align 4
  %d = getelementptr inbounds i32, ptr addrspace(1) %dst, i32 %i
  switch i32 %op, label %next [ i32 0, label %add i32 1, label %and i32 2, label %or i32 3, label %xor i32 4, label %inc i32 5, label %dec i32 6, label %min i32 7, label %umin i32 8, label %max i32 9, label %umax ]
add:
  %add.old = atomicrmw add ptr addrspace(1) %d, i32 %v monotonic, align 4
  br label %next
and:
  %and.old = atomicrmw and ptr addrspace(1) %d, i32 %v monotonic, align 4
  br label %next
or:
  %or.old = atomicrmw or ptr addrspace(1) %d, i32 %v monotonic, align 4
  br label %next
xor:
  %xor.old = atomicrmw xor ptr addrspace(1) %d, i32 %v monotonic, align 4
  br label %next
inc:
  %inc.old = atomicrmw uinc_wrap ptr addrspace(1) %d, i32 %v monotonic, align 4
  br label %next
dec:
  %dec.old = atomicrmw udec_wrap ptr addrspace(1) %d, i32 %v monotonic, align 4
  br label %next
min:
  %min.old = atomicrmw min ptr addrspace(1) %d, i32 %v monotonic, align 4
  br label %next
umin:
  %umin.old = atomicrmw umin ptr addrspace(1) %d, i32 %v monotonic, align 4
  br label %next
max:
  %max.old = atomicrmw max ptr addrspace(1) %d, i32 %v monotonic, align 4
  br label %next
umax:
  %umax.old = atomicrmw umax ptr addrspace(1) %d, i32 %v monotonic, align 4
  br label %next
next:
  %i.next = add i32 %i, 1
  %more = icmp ult i32 %i.next, %n
  br i1 %more, label %loop, label %exit
exit:
  ret void
}

define void @__zluda_ptx_impl_agx_cp_reduce_bulk_b32_shared(ptr addrspace(3) %dst, ptr addrspace(3) %src, i32 %size, i32 %op) #3 {
entry:
  %n = lshr i32 %size, 2
  %empty = icmp eq i32 %n, 0
  br i1 %empty, label %exit, label %loop
loop:
  %i = phi i32 [ 0, %entry ], [ %i.next, %next ]
  %s = getelementptr inbounds i32, ptr addrspace(3) %src, i32 %i
  %v = load i32, ptr addrspace(3) %s, align 4
  %d = getelementptr inbounds i32, ptr addrspace(3) %dst, i32 %i
  switch i32 %op, label %next [ i32 0, label %add i32 1, label %and i32 2, label %or i32 3, label %xor i32 4, label %inc i32 5, label %dec i32 6, label %min i32 7, label %umin i32 8, label %max i32 9, label %umax ]
add:
  %add.old = atomicrmw add ptr addrspace(3) %d, i32 %v syncscope("workgroup") monotonic, align 4
  br label %next
and:
  %and.old = atomicrmw and ptr addrspace(3) %d, i32 %v syncscope("workgroup") monotonic, align 4
  br label %next
or:
  %or.old = atomicrmw or ptr addrspace(3) %d, i32 %v syncscope("workgroup") monotonic, align 4
  br label %next
xor:
  %xor.old = atomicrmw xor ptr addrspace(3) %d, i32 %v syncscope("workgroup") monotonic, align 4
  br label %next
inc:
  %inc.old = atomicrmw uinc_wrap ptr addrspace(3) %d, i32 %v syncscope("workgroup") monotonic, align 4
  br label %next
dec:
  %dec.old = atomicrmw udec_wrap ptr addrspace(3) %d, i32 %v syncscope("workgroup") monotonic, align 4
  br label %next
min:
  %min.old = atomicrmw min ptr addrspace(3) %d, i32 %v syncscope("workgroup") monotonic, align 4
  br label %next
umin:
  %umin.old = atomicrmw umin ptr addrspace(3) %d, i32 %v syncscope("workgroup") monotonic, align 4
  br label %next
max:
  %max.old = atomicrmw max ptr addrspace(3) %d, i32 %v syncscope("workgroup") monotonic, align 4
  br label %next
umax:
  %umax.old = atomicrmw umax ptr addrspace(3) %d, i32 %v syncscope("workgroup") monotonic, align 4
  br label %next
next:
  %i.next = add i32 %i, 1
  %more = icmp ult i32 %i.next, %n
  br i1 %more, label %loop, label %exit
exit:
  ret void
}

define void @__zluda_ptx_impl_agx_cp_reduce_bulk_b64_global(ptr addrspace(1) %dst, ptr addrspace(3) %src, i32 %size, i32 %op) #3 {
entry:
  %n = lshr i32 %size, 3
  %empty = icmp eq i32 %n, 0
  br i1 %empty, label %exit, label %loop
loop:
  %i = phi i32 [ 0, %entry ], [ %i.next, %next ]
  %s = getelementptr inbounds i64, ptr addrspace(3) %src, i32 %i
  %v = load i64, ptr addrspace(3) %s, align 8
  %d = getelementptr inbounds i64, ptr addrspace(1) %dst, i32 %i
  switch i32 %op, label %next [ i32 0, label %add i32 1, label %and i32 2, label %or i32 3, label %xor i32 6, label %min i32 7, label %umin i32 8, label %max i32 9, label %umax ]
add:
  %add.old = atomicrmw add ptr addrspace(1) %d, i64 %v monotonic, align 8
  br label %next
and:
  %and.old = atomicrmw and ptr addrspace(1) %d, i64 %v monotonic, align 8
  br label %next
or:
  %or.old = atomicrmw or ptr addrspace(1) %d, i64 %v monotonic, align 8
  br label %next
xor:
  %xor.old = atomicrmw xor ptr addrspace(1) %d, i64 %v monotonic, align 8
  br label %next
min:
  %min.old = atomicrmw min ptr addrspace(1) %d, i64 %v monotonic, align 8
  br label %next
umin:
  %umin.old = atomicrmw umin ptr addrspace(1) %d, i64 %v monotonic, align 8
  br label %next
max:
  %max.old = atomicrmw max ptr addrspace(1) %d, i64 %v monotonic, align 8
  br label %next
umax:
  %umax.old = atomicrmw umax ptr addrspace(1) %d, i64 %v monotonic, align 8
  br label %next
next:
  %i.next = add i32 %i, 1
  %more = icmp ult i32 %i.next, %n
  br i1 %more, label %loop, label %exit
exit:
  ret void
}

define void @__zluda_ptx_impl_agx_cp_reduce_bulk_b64_shared(ptr addrspace(3) %dst, ptr addrspace(3) %src, i32 %size, i32 %op) #3 {
entry:
  %n = lshr i32 %size, 3
  %empty = icmp eq i32 %n, 0
  br i1 %empty, label %exit, label %loop
loop:
  %i = phi i32 [ 0, %entry ], [ %i.next, %next ]
  %s = getelementptr inbounds i64, ptr addrspace(3) %src, i32 %i
  %v = load i64, ptr addrspace(3) %s, align 8
  %d = getelementptr inbounds i64, ptr addrspace(3) %dst, i32 %i
  switch i32 %op, label %next [ i32 0, label %add i32 1, label %and i32 2, label %or i32 3, label %xor i32 6, label %min i32 7, label %umin i32 8, label %max i32 9, label %umax ]
add:
  %add.old = atomicrmw add ptr addrspace(3) %d, i64 %v syncscope("workgroup") monotonic, align 8
  br label %next
and:
  %and.old = atomicrmw and ptr addrspace(3) %d, i64 %v syncscope("workgroup") monotonic, align 8
  br label %next
or:
  %or.old = atomicrmw or ptr addrspace(3) %d, i64 %v syncscope("workgroup") monotonic, align 8
  br label %next
xor:
  %xor.old = atomicrmw xor ptr addrspace(3) %d, i64 %v syncscope("workgroup") monotonic, align 8
  br label %next
min:
  %min.old = atomicrmw min ptr addrspace(3) %d, i64 %v syncscope("workgroup") monotonic, align 8
  br label %next
umin:
  %umin.old = atomicrmw umin ptr addrspace(3) %d, i64 %v syncscope("workgroup") monotonic, align 8
  br label %next
max:
  %max.old = atomicrmw max ptr addrspace(3) %d, i64 %v syncscope("workgroup") monotonic, align 8
  br label %next
umax:
  %umax.old = atomicrmw umax ptr addrspace(3) %d, i64 %v syncscope("workgroup") monotonic, align 8
  br label %next
next:
  %i.next = add i32 %i, 1
  %more = icmp ult i32 %i.next, %n
  br i1 %more, label %loop, label %exit
exit:
  ret void
}

define void @__zluda_ptx_impl_agx_cp_reduce_bulk_f32_global(ptr addrspace(1) %dst, ptr addrspace(3) %src, i32 %size, i32 %op) #3 {
entry:
  %n = lshr i32 %size, 2
  %empty = icmp eq i32 %n, 0
  br i1 %empty, label %exit, label %loop
loop:
  %i = phi i32 [ 0, %entry ], [ %i.next, %next ]
  %s = getelementptr inbounds float, ptr addrspace(3) %src, i32 %i
  %v = load float, ptr addrspace(3) %s, align 4
  %d = getelementptr inbounds float, ptr addrspace(1) %dst, i32 %i
  switch i32 %op, label %next [ i32 10, label %fadd i32 11, label %fmin i32 12, label %fmax ]
fadd:
  %fadd.old = atomicrmw fadd ptr addrspace(1) %d, float %v monotonic, align 4
  br label %next
fmin:
  %fmin.old = atomicrmw fmin ptr addrspace(1) %d, float %v monotonic, align 4
  br label %next
fmax:
  %fmax.old = atomicrmw fmax ptr addrspace(1) %d, float %v monotonic, align 4
  br label %next
next:
  %i.next = add i32 %i, 1
  %more = icmp ult i32 %i.next, %n
  br i1 %more, label %loop, label %exit
exit:
  ret void
}

define void @__zluda_ptx_impl_agx_cp_reduce_bulk_f32_shared(ptr addrspace(3) %dst, ptr addrspace(3) %src, i32 %size, i32 %op) #3 {
entry:
  %n = lshr i32 %size, 2
  %empty = icmp eq i32 %n, 0
  br i1 %empty, label %exit, label %loop
loop:
  %i = phi i32 [ 0, %entry ], [ %i.next, %next ]
  %s = getelementptr inbounds float, ptr addrspace(3) %src, i32 %i
  %v = load float, ptr addrspace(3) %s, align 4
  %d = getelementptr inbounds float, ptr addrspace(3) %dst, i32 %i
  switch i32 %op, label %next [ i32 10, label %fadd i32 11, label %fmin i32 12, label %fmax ]
fadd:
  %fadd.old = atomicrmw fadd ptr addrspace(3) %d, float %v syncscope("workgroup") monotonic, align 4
  br label %next
fmin:
  %fmin.old = atomicrmw fmin ptr addrspace(3) %d, float %v syncscope("workgroup") monotonic, align 4
  br label %next
fmax:
  %fmax.old = atomicrmw fmax ptr addrspace(3) %d, float %v syncscope("workgroup") monotonic, align 4
  br label %next
next:
  %i.next = add i32 %i, 1
  %more = icmp ult i32 %i.next, %n
  br i1 %more, label %loop, label %exit
exit:
  ret void
}

define void @__zluda_ptx_impl_agx_cp_reduce_bulk_f64_global(ptr addrspace(1) %dst, ptr addrspace(3) %src, i32 %size, i32 %op) #3 {
entry:
  %n = lshr i32 %size, 3
  %empty = icmp eq i32 %n, 0
  br i1 %empty, label %exit, label %loop
loop:
  %i = phi i32 [ 0, %entry ], [ %i.next, %next ]
  %s = getelementptr inbounds double, ptr addrspace(3) %src, i32 %i
  %v = load double, ptr addrspace(3) %s, align 8
  %d = getelementptr inbounds double, ptr addrspace(1) %dst, i32 %i
  switch i32 %op, label %next [ i32 10, label %fadd ]
fadd:
  %fadd.old = atomicrmw fadd ptr addrspace(1) %d, double %v monotonic, align 8
  br label %next
next:
  %i.next = add i32 %i, 1
  %more = icmp ult i32 %i.next, %n
  br i1 %more, label %loop, label %exit
exit:
  ret void
}

define void @__zluda_ptx_impl_agx_cp_reduce_bulk_f64_shared(ptr addrspace(3) %dst, ptr addrspace(3) %src, i32 %size, i32 %op) #3 {
entry:
  %n = lshr i32 %size, 3
  %empty = icmp eq i32 %n, 0
  br i1 %empty, label %exit, label %loop
loop:
  %i = phi i32 [ 0, %entry ], [ %i.next, %next ]
  %s = getelementptr inbounds double, ptr addrspace(3) %src, i32 %i
  %v = load double, ptr addrspace(3) %s, align 8
  %d = getelementptr inbounds double, ptr addrspace(3) %dst, i32 %i
  switch i32 %op, label %next [ i32 10, label %fadd ]
fadd:
  %fadd.old = atomicrmw fadd ptr addrspace(3) %d, double %v syncscope("workgroup") monotonic, align 8
  br label %next
next:
  %i.next = add i32 %i, 1
  %more = icmp ult i32 %i.next, %n
  br i1 %more, label %loop, label %exit
exit:
  ret void
}


declare float @llvm.fabs.f32(float)
declare i32 @llvm.umin.i32(i32, i32)
declare i32 @llvm.ctpop.i32(i32)
declare float @llvm.copysign.f32(float, float)

; Rounding of the exact sums of the emulated matrix instructions (see
; ptx/src/pass/agx_matrix.rs): m * 2^e, with m normalized (bit 63 set) or
; zero and the sticky bit of the bits below m, rounded to nearest even into
; the floating point format with %mbits mantissa and %ebits exponent bits
; (denormals kept, overflow to infinity). Returns the bits of the result.
define i64 @__zluda_ptx_impl_agx_round_pack(i1 %neg, i64 %m, i1 %sticky, i32 %e, i32 %mbits, i32 %ebits) #0 {
  %sign_pos = add i32 %mbits, %ebits
  %sign_pos64 = zext i32 %sign_pos to i64
  %neg64 = zext i1 %neg to i64
  %sign = shl i64 %neg64, %sign_pos64
  %is_zero = icmp eq i64 %m, 0
  %ebits1 = sub i32 %ebits, 1
  %bias1 = shl i32 1, %ebits1
  %bias = sub i32 %bias1, 1
  ; biased exponent of the leading bit
  %lead = add i32 %e, 63
  %be = add i32 %lead, %bias
  %normal = icmp sgt i32 %be, 0
  ; bits below the mantissa
  %drop_normal = sub i32 63, %mbits
  %den = sub i32 1, %be
  %drop_den0 = add i32 %drop_normal, %den
  %drop_den = call i32 @llvm.umin.i32(i32 %drop_den0, i32 100)
  %drop = select i1 %normal, i32 %drop_normal, i32 %drop_den
  %mm = zext i64 %m to i128
  %drop128 = zext i32 %drop to i128
  %q = lshr i128 %mm, %drop128
  %unit = shl i128 1, %drop128
  %rmask = sub i128 %unit, 1
  %r = and i128 %mm, %rmask
  %half = lshr i128 %unit, 1
  %above = icmp ugt i128 %r, %half
  %tie = icmp eq i128 %r, %half
  %q64 = trunc i128 %q to i64
  %odd = trunc i64 %q64 to i1
  %tie_up0 = or i1 %sticky, %odd
  %tie_up = and i1 %tie, %tie_up0
  %up = or i1 %above, %tie_up
  %up64 = zext i1 %up to i64
  %qr = add i64 %q64, %up64
  ; ((be - 1) << mbits) + q: a carry out of the mantissa increments the exponent
  %be1 = sub i32 %be, 1
  %field = select i1 %normal, i32 %be1, i32 0
  %field64 = zext i32 %field to i64
  %mbits64 = zext i32 %mbits to i64
  %efield = shl i64 %field64, %mbits64
  %mag0 = add i64 %efield, %qr
  %emax1 = shl i32 1, %ebits
  %emax = sub i32 %emax1, 1
  %emax64 = zext i32 %emax to i64
  %inf = shl i64 %emax64, %mbits64
  %ovf0 = icmp uge i64 %mag0, %inf
  %ovf1 = icmp sge i32 %be, %emax
  %ovf = or i1 %ovf0, %ovf1
  %mag1 = select i1 %ovf, i64 %inf, i64 %mag0
  %mag = select i1 %is_zero, i64 0, i64 %mag1
  %res = or i64 %mag, %sign
  ret i64 %res
}

declare i32 @llvm.agx.thread.position.in.threadgroup.x()
declare i32 @llvm.agx.thread.position.in.threadgroup.y()
declare i32 @llvm.agx.thread.position.in.threadgroup.z()
declare i32 @llvm.agx.threads.per.threadgroup.x()
declare i32 @llvm.agx.threads.per.threadgroup.y()
declare i32 @llvm.agx.threads.per.threadgroup.z()
declare i32 @llvm.agx.threadgroup.position.in.grid.x()
declare i32 @llvm.agx.threadgroup.position.in.grid.y()
declare i32 @llvm.agx.threadgroup.position.in.grid.z()
declare i32 @llvm.agx.threadgroups.per.grid.x()
declare i32 @llvm.agx.threadgroups.per.grid.y()
declare i32 @llvm.agx.threadgroups.per.grid.z()
declare i32 @llvm.agx.thread.index.in.simdgroup()
declare i32 @llvm.agx.thread.index.in.threadgroup()
declare i32 @llvm.agx.simd.ballot(i1)
declare i32 @llvm.agx.simd.shuffle.i32(i32, i32)
declare i32 @llvm.agx.simd.reduce.i32(i32, i32 immarg)
declare void @llvm.agx.threadgroup.barrier()
declare float @llvm.agx.rsqrt.f32(float)
declare float @llvm.agx.rcp.f32(float)
declare float @llvm.sqrt.f32(float)
declare float @llvm.exp2.f32(float)
declare float @llvm.log2.f32(float)
declare {i32, i1} @llvm.uadd.with.overflow.i32(i32, i32)
declare void @llvm.trap()

attributes #0 = { alwaysinline nounwind willreturn memory(none) }
attributes #1 = { alwaysinline convergent nounwind }
attributes #2 = { alwaysinline convergent nounwind willreturn memory(none) }
attributes #3 = { alwaysinline nounwind }
attributes #4 = { nounwind willreturn memory(none) }

declare i32 @llvm.agx.simdgroup.index.in.threadgroup()

declare i32 @llvm.agx.core.id()

declare void @llvm.memcpy.p3.p1.i32(ptr addrspace(3) noalias nocapture writeonly, ptr addrspace(1) noalias nocapture readonly, i32, i1 immarg)
