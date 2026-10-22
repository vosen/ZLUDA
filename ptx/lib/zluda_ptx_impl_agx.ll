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

; The clocks. A context whose modules read them has a timer block, which a CPU thread of zluda_agx keeps
; updating with the CPU's generic timer (agx_runtime::TimerBlock): the counter at 0, at 8 its value when the block
; was made and at 16 the nanoseconds since boot then, at 24 the nanoseconds and at 28 the GPU cycles per tick
; (16.16 fixed point). A shader sees a CPU write only through a volatile (coherent) load, which the backend emits
; after the barrier that invalidates the GPU's copy (ptx-re-blockers B35: no special register is a clock; a read
; costs about 1 us). %globaltimer is in nanoseconds, %clock64 in cycles of the clock rate the driver reports.
; Without a timer block (the simulator) the clocks count reads of a device-wide counter: each read of a thread is
; later than its previous one, as PTX requires, but the values do not measure time.
@__zluda_agx_timer = internal addrspace(1) externally_initialized global ptr addrspace(1) null, align 8
@__zluda_ptx_impl_agx_clock = internal addrspace(1) global i32 0

define i32 @__zluda_ptx_impl_agx_clock_count() #3 {
  %c = atomicrmw add ptr addrspace(1) @__zluda_ptx_impl_agx_clock, i32 1 monotonic
  %r = add i32 %c, 1
  ret i32 %r
}

; Ticks of the timer block %t since it was made
define i64 @__zluda_ptx_impl_agx_timer_ticks(ptr addrspace(1) %t) #3 {
  %now = load volatile i64, ptr addrspace(1) %t, align 8
  %base.p = getelementptr inbounds i8, ptr addrspace(1) %t, i64 8
  %base = load i64, ptr addrspace(1) %base.p, align 8
  %d = sub i64 %now, %base
  ret i64 %d
}

; %d times the 16.16 fixed point factor at byte %at of the timer block, without overflow
define i64 @__zluda_ptx_impl_agx_timer_scale(ptr addrspace(1) %t, i64 %d, i64 %at) #3 {
  %q.p = getelementptr inbounds i8, ptr addrspace(1) %t, i64 %at
  %q32 = load i32, ptr addrspace(1) %q.p, align 4
  %q = zext i32 %q32 to i64
  %hi = lshr i64 %d, 16
  %lo = and i64 %d, 65535
  %hi.q = mul i64 %hi, %q
  %lo.q = mul i64 %lo, %q
  %lo.s = lshr i64 %lo.q, 16
  %r = add i64 %hi.q, %lo.s
  ret i64 %r
}

define i64 @__zluda_ptx_impl_sreg_clock64() #3 {
  %t = load ptr addrspace(1), ptr addrspace(1) @__zluda_agx_timer, align 8
  %none = icmp eq ptr addrspace(1) %t, null
  br i1 %none, label %count, label %timer
count:
  %c = call i32 @__zluda_ptx_impl_agx_clock_count()
  %c64 = zext i32 %c to i64
  ret i64 %c64
timer:
  %d = call i64 @__zluda_ptx_impl_agx_timer_ticks(ptr addrspace(1) %t)
  %r = call i64 @__zluda_ptx_impl_agx_timer_scale(ptr addrspace(1) %t, i64 %d, i64 28)
  ret i64 %r
}

define i32 @__zluda_ptx_impl_sreg_clock() #3 {
  %c = call i64 @__zluda_ptx_impl_sreg_clock64()
  %r = trunc i64 %c to i32
  ret i32 %r
}

define i64 @__zluda_ptx_impl_sreg_globaltimer() #3 {
  %t = load ptr addrspace(1), ptr addrspace(1) @__zluda_agx_timer, align 8
  %none = icmp eq ptr addrspace(1) %t, null
  br i1 %none, label %count, label %timer
count:
  %c = call i32 @__zluda_ptx_impl_agx_clock_count()
  %c64 = zext i32 %c to i64
  %c.ns = mul i64 %c64, 1000
  ret i64 %c.ns
timer:
  %d = call i64 @__zluda_ptx_impl_agx_timer_ticks(ptr addrspace(1) %t)
  %ns = call i64 @__zluda_ptx_impl_agx_timer_scale(ptr addrspace(1) %t, i64 %d, i64 24)
  %base.p = getelementptr inbounds i8, ptr addrspace(1) %t, i64 16
  %base = load i64, ptr addrspace(1) %base.p, align 8
  %r = add i64 %base, %ns
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

; convergent, so that the activemask of each side of a branch is not merged into one before it
define i32 @__zluda_ptx_impl_activemask() #2 {
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

; bmsk.clamp.b32: position and width clamped to 32 (the Semantics of PTX ISA
; 9.7.1, on the whole 32-bit operands).
define i32 @__zluda_ptx_impl_bmsk_clamp_b32(i32 %position, i32 %width) #0 {
  %p32 = call i32 @llvm.umin.i32(i32 %position, i32 32)
  %wd32 = call i32 @llvm.umin.i32(i32 %width, i32 32)
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
define i1 @__zluda_ptx_impl_sreg_is_explicit_cluster() #0 {
  %v = call i32 @__zluda_agx_cluster(i32 14)
  %p = icmp ne i32 %v, 0
  ret i1 %p
}

; The shared memory registers. The kernels of a module that reads them store the threadgroup memory of the launch
; (static and dynamic bytes, from their hidden argument "agx-hidden"="threadgroup-bytes") in __zluda_agx_smem_bytes;
; the backend places zero-size threadgroup variables after the others, in module order, so the address of
; __zluda_agx_smem_end, linked after the program's, is where the dynamic shared memory starts. Allocations are in
; units of 128 bytes, as on sm_90, followed by a reserved region of 1 KiB for system software (which AGX has none
; of; no memory is allocated for it). In a cluster kernel they are the threadgroup's, all of the cluster's CTAs.
@__zluda_agx_smem_bytes = addrspace(3) global i32 undef, align 4
@__zluda_agx_smem_end = internal addrspace(3) global [0 x i8] undef, align 1

define i32 @__zluda_ptx_impl_sreg_dynamic_smem_size() #3 {
  %total = load i32, ptr addrspace(3) @__zluda_agx_smem_bytes, align 4
  %static = ptrtoint ptr addrspace(3) @__zluda_agx_smem_end to i32
  %v = sub i32 %total, %static
  ret i32 %v
}
define i32 @__zluda_ptx_impl_sreg_total_smem_size() #3 {
  %total = load i32, ptr addrspace(3) @__zluda_agx_smem_bytes, align 4
  %up = add i32 %total, 127
  %v = and i32 %up, -128
  ret i32 %v
}
define i32 @__zluda_ptx_impl_sreg_reserved_smem_offset_begin() #3 {
  %v = call i32 @__zluda_ptx_impl_sreg_total_smem_size()
  ret i32 %v
}
define i32 @__zluda_ptx_impl_sreg_reserved_smem_offset_end() #3 {
  %b = call i32 @__zluda_ptx_impl_sreg_total_smem_size()
  %v = add i32 %b, 1024
  ret i32 %v
}
; the size of the reserved region
define i32 @__zluda_ptx_impl_sreg_reserved_smem_offset_cap() #3 {
  ret i32 1024
}
define i32 @__zluda_ptx_impl_sreg_aggr_smem_size() #3 {
  %v = call i32 @__zluda_ptx_impl_sreg_reserved_smem_offset_end()
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

; Whether a generic address of shared memory is in the window of a CTA of this cluster
define zeroext i1 @__zluda_ptx_impl_agx_in_cluster(i64 %addr) #0 {
  %r = call i32 @__zluda_ptx_impl_getctarank_generic_u64(i64 %addr)
  %n = call i32 @__zluda_agx_cluster(i32 7)
  %in = icmp ult i32 %r, %n
  ret i1 %in
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
; predicate. Over the whole warp (opts 31 for down and bfly, 0 for up) these
; are the native relative shuffles, whose out of range lanes also keep their
; own value; the general form folds away when opts is that constant.
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
  %general = call i32 @llvm.agx.simd.shuffle.i32(i32 %input, i32 %lane)
  %whole = icmp eq i32 %opts, 0
  %native = call i32 @llvm.agx.simd.shuffle.up.i32(i32 %input, i32 %delta)
  %value = select i1 %whole, i32 %native, i32 %general
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
  %general = call i32 @llvm.agx.simd.shuffle.i32(i32 %input, i32 %lane)
  %whole = icmp eq i32 %opts, 31
  %native = call i32 @llvm.agx.simd.shuffle.down.i32(i32 %input, i32 %delta)
  %value = select i1 %whole, i32 %native, i32 %general
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
  %general = call i32 @llvm.agx.simd.shuffle.i32(i32 %input, i32 %lane)
  %whole = icmp eq i32 %opts, 31
  %native = call i32 @llvm.agx.simd.shuffle.xor.i32(i32 %input, i32 %delta)
  %value = select i1 %whole, i32 %native, i32 %general
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

; the deprecated votes without .sync: over the active threads, as the .sync ones here
define zeroext i1 @__zluda_ptx_impl_vote_any_pred(i1 zeroext %value) #2 {
  %r = call i1 @__zluda_ptx_impl_vote_sync_any_pred(i1 %value, i32 -1)
  ret i1 %r
}

define zeroext i1 @__zluda_ptx_impl_vote_any_pred_negate(i1 zeroext %value) #2 {
  %r = call i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1 %value, i32 -1)
  ret i1 %r
}

define zeroext i1 @__zluda_ptx_impl_vote_all_pred(i1 zeroext %value) #2 {
  %r = call i1 @__zluda_ptx_impl_vote_sync_all_pred(i1 %value, i32 -1)
  ret i1 %r
}

define zeroext i1 @__zluda_ptx_impl_vote_all_pred_negate(i1 zeroext %value) #2 {
  %r = call i1 @__zluda_ptx_impl_vote_sync_all_pred_negate(i1 %value, i32 -1)
  ret i1 %r
}

define zeroext i1 @__zluda_ptx_impl_vote_uni_pred(i1 zeroext %value) #2 {
  %r = call i1 @__zluda_ptx_impl_vote_sync_uni_pred(i1 %value, i32 -1)
  ret i1 %r
}

define zeroext i1 @__zluda_ptx_impl_vote_uni_pred_negate(i1 zeroext %value) #2 {
  %r = call i1 @__zluda_ptx_impl_vote_sync_uni_pred_negate(i1 %value, i32 -1)
  ret i1 %r
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

; nanosleep waits until the timer has advanced by the given nanoseconds (PTX allows up to twice as long; at most
; 2^16 reads, about 70 ms); without a timer block it returns at once
define void @__zluda_ptx_impl_nanosleep_u32(i32 %nanoseconds) #3 {
entry:
  %t = load ptr addrspace(1), ptr addrspace(1) @__zluda_agx_timer, align 8
  %none = icmp eq ptr addrspace(1) %t, null
  br i1 %none, label %done, label %start
start:
  %ns = zext i32 %nanoseconds to i64
  %t0 = call i64 @__zluda_ptx_impl_agx_timer_ticks(ptr addrspace(1) %t)
  br label %loop
loop:
  %i = phi i32 [ 0, %start ], [ %i.next, %loop ]
  %now = call i64 @__zluda_ptx_impl_agx_timer_ticks(ptr addrspace(1) %t)
  %d = sub i64 %now, %t0
  %elapsed = call i64 @__zluda_ptx_impl_agx_timer_scale(ptr addrspace(1) %t, i64 %d, i64 24)
  %i.next = add i32 %i, 1
  %waiting = icmp ult i64 %elapsed, %ns
  %bounded = icmp ult i32 %i.next, 65536
  %again = and i1 %waiting, %bounded
  br i1 %again, label %loop, label %done
done:
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

; The device heap of malloc and free (zluda_ptx_impl_agx_heap.c): zluda_agx
; writes its address here when it loads the module.
@__zluda_agx_heap = internal addrspace(1) externally_initialized global ptr addrspace(1) null, align 8

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
; overflow; below 2^-6, where 1 - e cancels, x - x^3 / 3 (relative error
; under 2^-27).
define float @__zluda_ptx_impl_tanh_f32(float %x) #0 {
  %ax = call float @llvm.fabs.f32(float %x)
  %s = fmul float %ax, 0xC007154760000000
  %e = call float @llvm.exp2.f32(float %s)
  %num = fsub float 1.0, %e
  %den = fadd float 1.0, %e
  %t = fdiv float %num, %den
  %r = call float @llvm.copysign.f32(float %t, float %x)
  %x2 = fmul float %x, %x
  %c = fmul float %x2, 0xBFD5555560000000
  %small.m = call float @llvm.fma.f32(float %x, float %c, float %x)
  %small.r = call float @llvm.copysign.f32(float %small.m, float %x)
  %small = fcmp olt float %ax, 0x3F90000000000000
  %res = select i1 %small, float %small.r, float %r
  ret float %res
}

define half @__zluda_ptx_impl_tanh_f16(half %x) #0 {
  %f = fpext half %x to float
  %t = call float @__zluda_ptx_impl_tanh_f32(float %f)
  %r = fptrunc float %t to half
  ret half %r
}

define <2 x half> @__zluda_ptx_impl_tanh_f16x2(<2 x half> %x) #0 {
  %f = fpext <2 x half> %x to <2 x float>
  %f0 = extractelement <2 x float> %f, i32 0
  %f1 = extractelement <2 x float> %f, i32 1
  %t0 = call float @__zluda_ptx_impl_tanh_f32(float %f0)
  %t1 = call float @__zluda_ptx_impl_tanh_f32(float %f1)
  %v0 = insertelement <2 x float> poison, float %t0, i32 0
  %v1 = insertelement <2 x float> %v0, float %t1, i32 1
  %r = fptrunc <2 x float> %v1 to <2 x half>
  ret <2 x half> %r
}

; Below 2^-4 tanh(x) rounds to x in bfloat, whose subnormals would flush in
; single precision.
define bfloat @__zluda_ptx_impl_tanh_bf16(bfloat %x) #0 {
  %f = fpext bfloat %x to float
  %t = call float @__zluda_ptx_impl_tanh_f32(float %f)
  %r = fptrunc float %t to bfloat
  %bits = bitcast bfloat %x to i16
  %mag = and i16 %bits, 32767
  %small = icmp ult i16 %mag, 15744
  %res = select i1 %small, bfloat %x, bfloat %r
  ret bfloat %res
}

define <2 x bfloat> @__zluda_ptx_impl_tanh_bf16x2(<2 x bfloat> %x) #0 {
  %f = fpext <2 x bfloat> %x to <2 x float>
  %f0 = extractelement <2 x float> %f, i32 0
  %f1 = extractelement <2 x float> %f, i32 1
  %t0 = call float @__zluda_ptx_impl_tanh_f32(float %f0)
  %t1 = call float @__zluda_ptx_impl_tanh_f32(float %f1)
  %v0 = insertelement <2 x float> poison, float %t0, i32 0
  %v1 = insertelement <2 x float> %v0, float %t1, i32 1
  %r = fptrunc <2 x float> %v1 to <2 x bfloat>
  %bits = bitcast <2 x bfloat> %x to <2 x i16>
  %mag = and <2 x i16> %bits, <i16 32767, i16 32767>
  %small = icmp ult <2 x i16> %mag, <i16 15744, i16 15744>
  %res = select <2 x i1> %small, <2 x bfloat> %x, <2 x bfloat> %r
  ret <2 x bfloat> %res
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


; ---------------------------------------------------------------- TMA (cp.async.bulk.tensor)
; A tensor map is 16 little-endian 64-bit words (the encoding of
; PTX-formalization's tmBlob, which the AGX runtime also uses):
;   0 magic | 1 element type (tensormap.replace.elemtype numbering) | 2 rank |
;   3 global address | 4..8 dims | 9..12 byte strides of dims 1..4 |
;   13 box dims 0..3 (16 bits each) | 14 box dim 4, swizzle_mode << 16,
;   fill_mode << 24, interleave << 32, swizzle_atomicity << 48 |
;   15 element strides (8 bits each)
; Copies run element by element in the issuing thread and complete at once.

define i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 %i) #3 {
  %p = getelementptr inbounds i64, ptr %tm, i32 %i
  %v = load i64, ptr %p, align 8
  ret i64 %v
}

; bytes of an element type
define i32 @__zluda_ptx_impl_agx_tma_bytes(i32 %et) #0 {
  switch i32 %et, label %one [ i32 1, label %two i32 6, label %two i32 10, label %two
                               i32 2, label %four i32 3, label %four i32 7, label %four i32 8, label %four i32 11, label %four i32 12, label %four
                               i32 4, label %eight i32 5, label %eight i32 9, label %eight ]
one:
  ret i32 1
two:
  ret i32 2
four:
  ret i32 4
eight:
  ret i32 8
}

; the swizzle of a shared-memory byte address: 1 128B, 2 64B, 3 32B (Swizzle<B,4,3>),
; 4 128B with 32B atomicity, 5 the same with an 8-byte flip on odd 128-byte lines,
; 6 128B with 64B atomicity
define i32 @__zluda_ptx_impl_agx_tma_swizzle(i32 %mode, i32 %a) #0 {
entry:
  %line = lshr i32 %a, 7
  switch i32 %mode, label %none [ i32 1, label %b3 i32 2, label %b2 i32 3, label %b1 i32 4, label %m4 i32 5, label %m5 i32 6, label %m6 ]
b3:
  %x3 = and i32 %line, 7
  br label %chunks
b2:
  %x2 = and i32 %line, 3
  br label %chunks
b1:
  %x1 = and i32 %line, 1
  br label %chunks
chunks:
  %x = phi i32 [ %x3, %b3 ], [ %x2, %b2 ], [ %x1, %b1 ]
  %xs = shl i32 %x, 4
  %r = xor i32 %a, %xs
  ret i32 %r
m4:
  %l4 = and i32 %line, 3
  %s4 = shl i32 %l4, 5
  %r4 = xor i32 %a, %s4
  ret i32 %r4
m5:
  %l5 = and i32 %line, 3
  %s5 = shl i32 %l5, 5
  %t5 = xor i32 %a, %s5
  %o5 = and i32 %line, 1
  %f5 = shl i32 %o5, 3
  %r5 = xor i32 %t5, %f5
  ret i32 %r5
m6:
  %l6 = and i32 %line, 1
  %s6 = shl i32 %l6, 6
  %r6 = xor i32 %a, %s6
  ret i32 %r6
none:
  ret i32 %a
}

define i64 @__zluda_ptx_impl_agx_tma_load_global(i64 %a, i32 %n) #3 {
entry:
  %p = inttoptr i64 %a to ptr addrspace(1)
  switch i32 %n, label %b1 [ i32 2, label %b2 i32 4, label %b4 i32 8, label %b8 ]
b1:
  %v1 = load i8, ptr addrspace(1) %p, align 1
  %r1 = zext i8 %v1 to i64
  ret i64 %r1
b2:
  %v2 = load i16, ptr addrspace(1) %p, align 2
  %r2 = zext i16 %v2 to i64
  ret i64 %r2
b4:
  %v4 = load i32, ptr addrspace(1) %p, align 4
  %r4 = zext i32 %v4 to i64
  ret i64 %r4
b8:
  %v8 = load i64, ptr addrspace(1) %p, align 8
  ret i64 %v8
}

define void @__zluda_ptx_impl_agx_tma_store_global(i64 %a, i32 %n, i64 %v) #3 {
entry:
  %p = inttoptr i64 %a to ptr addrspace(1)
  switch i32 %n, label %b1 [ i32 2, label %b2 i32 4, label %b4 i32 8, label %b8 ]
b1:
  %v1 = trunc i64 %v to i8
  store i8 %v1, ptr addrspace(1) %p, align 1
  ret void
b2:
  %v2 = trunc i64 %v to i16
  store i16 %v2, ptr addrspace(1) %p, align 2
  ret void
b4:
  %v4 = trunc i64 %v to i32
  store i32 %v4, ptr addrspace(1) %p, align 4
  ret void
b8:
  store i64 %v, ptr addrspace(1) %p, align 8
  ret void
}

define i64 @__zluda_ptx_impl_agx_tma_load_shared(i32 %a, i32 %n) #3 {
entry:
  %p = inttoptr i32 %a to ptr addrspace(3)
  switch i32 %n, label %b1 [ i32 2, label %b2 i32 4, label %b4 i32 8, label %b8 ]
b1:
  %v1 = load i8, ptr addrspace(3) %p, align 1
  %r1 = zext i8 %v1 to i64
  ret i64 %r1
b2:
  %v2 = load i16, ptr addrspace(3) %p, align 2
  %r2 = zext i16 %v2 to i64
  ret i64 %r2
b4:
  %v4 = load i32, ptr addrspace(3) %p, align 4
  %r4 = zext i32 %v4 to i64
  ret i64 %r4
b8:
  %v8 = load i64, ptr addrspace(3) %p, align 8
  ret i64 %v8
}

define void @__zluda_ptx_impl_agx_tma_store_shared(i32 %a, i32 %n, i64 %v) #3 {
entry:
  %p = inttoptr i32 %a to ptr addrspace(3)
  switch i32 %n, label %b1 [ i32 2, label %b2 i32 4, label %b4 i32 8, label %b8 ]
b1:
  %v1 = trunc i64 %v to i8
  store i8 %v1, ptr addrspace(3) %p, align 1
  ret void
b2:
  %v2 = trunc i64 %v to i16
  store i16 %v2, ptr addrspace(3) %p, align 2
  ret void
b4:
  %v4 = trunc i64 %v to i32
  store i32 %v4, ptr addrspace(3) %p, align 4
  ret void
b8:
  store i64 %v, ptr addrspace(3) %p, align 8
  ret void
}

; An atomic (relaxed) reduction of one element of type %et into global memory.
; %op: 0 add, 1 and, 2 or, 3 xor, 4 inc, 5 dec, 6 min, 8 max (signedness and
; float from the element type)
define void @__zluda_ptx_impl_agx_tma_red(i64 %a, i32 %et, i32 %op, i64 %v) #3 {
entry:
  %p = inttoptr i64 %a to ptr addrspace(1)
  %v32 = trunc i64 %v to i32
  %v16 = trunc i64 %v to i16
  %h = bitcast i16 %v16 to half
  %bf = bitcast i16 %v16 to bfloat
  %f = bitcast i32 %v32 to float
  %n = call i32 @__zluda_ptx_impl_agx_tma_bytes(i32 %et)
  %wide = icmp eq i32 %n, 8
  %is_f16 = icmp eq i32 %et, 6
  %is_bf16 = icmp eq i32 %et, 10
  %is_f32 = icmp eq i32 %et, 7
  %is_f32ftz = icmp eq i32 %et, 8
  %is_float32 = or i1 %is_f32, %is_f32ftz
  %signed32 = icmp eq i32 %et, 3
  %signed64 = icmp eq i32 %et, 5
  %signed = or i1 %signed32, %signed64
  switch i32 %op, label %exit [ i32 0, label %add i32 1, label %and i32 2, label %or i32 3, label %xor
                                i32 4, label %inc i32 5, label %dec i32 6, label %min i32 8, label %max ]
add:
  br i1 %is_f16, label %add_h, label %add_1
add_1:
  br i1 %is_bf16, label %add_bf, label %add_2
add_2:
  br i1 %is_float32, label %add_f, label %add_3
add_3:
  br i1 %wide, label %add_64, label %add_32
add_h:
  %o0 = atomicrmw fadd ptr addrspace(1) %p, half %h monotonic, align 2
  br label %exit
add_bf:
  %o1 = atomicrmw fadd ptr addrspace(1) %p, bfloat %bf monotonic, align 2
  br label %exit
add_f:
  %o2 = atomicrmw fadd ptr addrspace(1) %p, float %f monotonic, align 4
  br label %exit
add_64:
  %o3 = atomicrmw add ptr addrspace(1) %p, i64 %v monotonic, align 8
  br label %exit
add_32:
  %o4 = atomicrmw add ptr addrspace(1) %p, i32 %v32 monotonic, align 4
  br label %exit
and:
  br i1 %wide, label %and_64, label %and_32
and_64:
  %o5 = atomicrmw and ptr addrspace(1) %p, i64 %v monotonic, align 8
  br label %exit
and_32:
  %o6 = atomicrmw and ptr addrspace(1) %p, i32 %v32 monotonic, align 4
  br label %exit
or:
  br i1 %wide, label %or_64, label %or_32
or_64:
  %o7 = atomicrmw or ptr addrspace(1) %p, i64 %v monotonic, align 8
  br label %exit
or_32:
  %o8 = atomicrmw or ptr addrspace(1) %p, i32 %v32 monotonic, align 4
  br label %exit
xor:
  br i1 %wide, label %xor_64, label %xor_32
xor_64:
  %o9 = atomicrmw xor ptr addrspace(1) %p, i64 %v monotonic, align 8
  br label %exit
xor_32:
  %o10 = atomicrmw xor ptr addrspace(1) %p, i32 %v32 monotonic, align 4
  br label %exit
inc:
  %o11 = atomicrmw uinc_wrap ptr addrspace(1) %p, i32 %v32 monotonic, align 4
  br label %exit
dec:
  %o12 = atomicrmw udec_wrap ptr addrspace(1) %p, i32 %v32 monotonic, align 4
  br label %exit
min:
  br i1 %is_f16, label %min_h, label %min_1
min_1:
  br i1 %is_bf16, label %min_bf, label %min_2
min_2:
  br i1 %wide, label %min_64, label %min_32
min_h:
  %o13 = atomicrmw fmin ptr addrspace(1) %p, half %h monotonic, align 2
  br label %exit
min_bf:
  %o14 = atomicrmw fmin ptr addrspace(1) %p, bfloat %bf monotonic, align 2
  br label %exit
min_64:
  br i1 %signed, label %min_s64, label %min_u64
min_s64:
  %o15 = atomicrmw min ptr addrspace(1) %p, i64 %v monotonic, align 8
  br label %exit
min_u64:
  %o16 = atomicrmw umin ptr addrspace(1) %p, i64 %v monotonic, align 8
  br label %exit
min_32:
  br i1 %signed, label %min_s32, label %min_u32
min_s32:
  %o17 = atomicrmw min ptr addrspace(1) %p, i32 %v32 monotonic, align 4
  br label %exit
min_u32:
  %o18 = atomicrmw umin ptr addrspace(1) %p, i32 %v32 monotonic, align 4
  br label %exit
max:
  br i1 %is_f16, label %max_h, label %max_1
max_1:
  br i1 %is_bf16, label %max_bf, label %max_2
max_2:
  br i1 %wide, label %max_64, label %max_32
max_h:
  %o19 = atomicrmw fmax ptr addrspace(1) %p, half %h monotonic, align 2
  br label %exit
max_bf:
  %o20 = atomicrmw fmax ptr addrspace(1) %p, bfloat %bf monotonic, align 2
  br label %exit
max_64:
  br i1 %signed, label %max_s64, label %max_u64
max_s64:
  %o21 = atomicrmw max ptr addrspace(1) %p, i64 %v monotonic, align 8
  br label %exit
max_u64:
  %o22 = atomicrmw umax ptr addrspace(1) %p, i64 %v monotonic, align 8
  br label %exit
max_32:
  br i1 %signed, label %max_s32, label %max_u32
max_s32:
  %o23 = atomicrmw max ptr addrspace(1) %p, i32 %v32 monotonic, align 4
  br label %exit
max_u32:
  %o24 = atomicrmw umax ptr addrspace(1) %p, i32 %v32 monotonic, align 4
  br label %exit
exit:
  ret void
}

; The box of a tile-mode copy at coordinates c0..c4 (0 beyond the rank),
; element e (dim 0 fastest) at global coordinates c_i + q_i * element stride_i.
; %dir: -1 load (global -> shared image at %smem, out-of-bounds elements
; filled), -2 store (shared -> global, out-of-bounds elements dropped), >= 0
; reduce with that operation. The shared image is dense, swizzled by the
; CTA-relative address. Returns the bytes of the box.
define i32 @__zluda_ptx_impl_agx_tma_tile(ptr %tm, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4, i32 %smem, i32 %dir) #3 {
entry:
  %w1 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 1)
  %w1t = trunc i64 %w1 to i32
  %et = and i32 %w1t, 255
  %packed = icmp uge i32 %et, 13
  br i1 %packed, label %sub, label %elements
sub:
  %sub_bytes = call i32 @__zluda_ptx_impl_agx_tma_tile_sub(ptr %tm, i32 %et, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4, i32 %smem, i32 %dir)
  ret i32 %sub_bytes
elements:
  %eb = call i32 @__zluda_ptx_impl_agx_tma_bytes(i32 %et)
  %eb64 = zext i32 %eb to i64
  %w2 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 2)
  %w2t = trunc i64 %w2 to i32
  %rank = and i32 %w2t, 255
  %base = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 3)
  %d0 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 4)
  %d1 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 5)
  %d2 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 6)
  %d3 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 7)
  %d4 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 8)
  %s1 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 9)
  %s2 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 10)
  %s3 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 11)
  %s4 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 12)
  %w13 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 13)
  %w14 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 14)
  %w15 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 15)
  %r1 = icmp ugt i32 %rank, 1
  %r2 = icmp ugt i32 %rank, 2
  %r3 = icmp ugt i32 %rank, 3
  %r4 = icmp ugt i32 %rank, 4
  %w13t = trunc i64 %w13 to i32
  %w13h = lshr i64 %w13, 32
  %w13ht = trunc i64 %w13h to i32
  %w14t = trunc i64 %w14 to i32
  %bx0 = and i32 %w13t, 65535
  %bx1s = lshr i32 %w13t, 16
  %bx2 = and i32 %w13ht, 65535
  %bx3s = lshr i32 %w13ht, 16
  %bx4 = and i32 %w14t, 65535
  %b1r = select i1 %r1, i32 %bx1s, i32 1
  %b2r = select i1 %r2, i32 %bx2, i32 1
  %b3r = select i1 %r3, i32 %bx3s, i32 1
  %b4r = select i1 %r4, i32 %bx4, i32 1
  %w15t = trunc i64 %w15 to i32
  %e0 = and i32 %w15t, 255
  %e1s = lshr i32 %w15t, 8
  %e1 = and i32 %e1s, 255
  %e2s = lshr i32 %w15t, 16
  %e2 = and i32 %e2s, 255
  %e3 = lshr i32 %w15t, 24
  %w15h = lshr i64 %w15, 32
  %w15ht = trunc i64 %w15h to i32
  %e4 = and i32 %w15ht, 255
  ; swizzle mode from swizzle_mode / swizzle_atomicity
  %swz = lshr i32 %w14t, 16
  %swzm = and i32 %swz, 255
  %w14h = lshr i64 %w14, 48
  %w14ht = trunc i64 %w14h to i32
  %atom = and i32 %w14ht, 255
  %is128 = icmp eq i32 %swzm, 3
  %m128 = select i1 %is128, i32 1, i32 0
  %is64 = icmp eq i32 %swzm, 2
  %is32 = icmp eq i32 %swzm, 1
  %m64 = select i1 %is64, i32 2, i32 %m128
  %m32 = select i1 %is32, i32 3, i32 %m64
  %atomic = add i32 %atom, 3
  %has_atom = icmp ne i32 %atom, 0
  %atom128 = and i1 %is128, %has_atom
  %mode = select i1 %atom128, i32 %atomic, i32 %m32
  ; out-of-bounds fill: zero, or (fill_mode != 0) a NaN of the element type
  %fm = lshr i32 %w14t, 24
  %fm8 = and i32 %fm, 255
  %nan_fill = icmp ne i32 %fm8, 0
  %f4 = icmp eq i32 %eb, 4
  %fh = icmp eq i32 %et, 6
  %fbf = icmp eq i32 %et, 10
  %fd = icmp eq i32 %et, 9
  %n0 = select i1 %fd, i64 9221120237041090560, i64 0
  %n1 = select i1 %fbf, i64 32704, i64 %n0
  %n2 = select i1 %fh, i64 32256, i64 %n1
  %n3 = select i1 %f4, i64 2143289344, i64 %n2
  %fill = select i1 %nan_fill, i64 %n3, i64 0
  ; the shared image is swizzled by its CTA-relative address
  %smem_rel = call i32 @__zluda_agx_cluster_mapa(i32 %smem, i32 0)
  %window = sub i32 %smem, %smem_rel
  %v01 = mul i32 %bx0, %b1r
  %v012 = mul i32 %v01, %b2r
  %v0123 = mul i32 %v012, %b3r
  %count = mul i32 %v0123, %b4r
  %bytes = mul i32 %count, %eb
  %empty = icmp eq i32 %count, 0
  br i1 %empty, label %exit, label %loop
loop:
  %e = phi i32 [ 0, %elements ], [ %e.next, %next ]
  %q0 = phi i32 [ 0, %elements ], [ %q0.n, %next ]
  %q1 = phi i32 [ 0, %elements ], [ %q1.n, %next ]
  %q2 = phi i32 [ 0, %elements ], [ %q2.n, %next ]
  %q3 = phi i32 [ 0, %elements ], [ %q3.n, %next ]
  %q4 = phi i32 [ 0, %elements ], [ %q4.n, %next ]
  %t0 = mul i32 %q0, %e0
  %g0 = add i32 %c0, %t0
  %t1 = mul i32 %q1, %e1
  %g1 = add i32 %c1, %t1
  %t2 = mul i32 %q2, %e2
  %g2 = add i32 %c2, %t2
  %t3 = mul i32 %q3, %e3
  %g3 = add i32 %c3, %t3
  %t4 = mul i32 %q4, %e4
  %g4 = add i32 %c4, %t4
  %g0w = sext i32 %g0 to i64
  %g1w = sext i32 %g1 to i64
  %g2w = sext i32 %g2 to i64
  %g3w = sext i32 %g3 to i64
  %g4w = sext i32 %g4 to i64
  ; unsigned compares also catch negative coordinates
  %o0 = icmp uge i64 %g0w, %d0
  %o1a = icmp uge i64 %g1w, %d1
  %o1 = and i1 %o1a, %r1
  %o2a = icmp uge i64 %g2w, %d2
  %o2 = and i1 %o2a, %r2
  %o3a = icmp uge i64 %g3w, %d3
  %o3 = and i1 %o3a, %r3
  %o4a = icmp uge i64 %g4w, %d4
  %o4 = and i1 %o4a, %r4
  %oo1 = or i1 %o0, %o1
  %oo2 = or i1 %oo1, %o2
  %oo3 = or i1 %oo2, %o3
  %oob = or i1 %oo3, %o4
  %a0 = mul i64 %g0w, %eb64
  %a1 = mul i64 %g1w, %s1
  %a2 = mul i64 %g2w, %s2
  %a3 = mul i64 %g3w, %s3
  %a4 = mul i64 %g4w, %s4
  %a01 = add i64 %a0, %a1
  %a012 = add i64 %a01, %a2
  %a0123 = add i64 %a012, %a3
  %off = add i64 %a0123, %a4
  %gaddr = add i64 %base, %off
  %eoff = mul i32 %e, %eb
  %rel = add i32 %smem_rel, %eoff
  %swizzled = call i32 @__zluda_ptx_impl_agx_tma_swizzle(i32 %mode, i32 %rel)
  %saddr = add i32 %window, %swizzled
  %is_load = icmp eq i32 %dir, -1
  br i1 %is_load, label %load, label %out
load:
  br i1 %oob, label %load_fill, label %load_mem
load_mem:
  %lv = call i64 @__zluda_ptx_impl_agx_tma_load_global(i64 %gaddr, i32 %eb)
  br label %load_fill
load_fill:
  %value = phi i64 [ %fill, %load ], [ %lv, %load_mem ]
  call void @__zluda_ptx_impl_agx_tma_store_shared(i32 %saddr, i32 %eb, i64 %value)
  br label %next
out:
  br i1 %oob, label %next, label %out_elem
out_elem:
  %sv = call i64 @__zluda_ptx_impl_agx_tma_load_shared(i32 %saddr, i32 %eb)
  %is_store = icmp eq i32 %dir, -2
  br i1 %is_store, label %store, label %reduce
store:
  call void @__zluda_ptx_impl_agx_tma_store_global(i64 %gaddr, i32 %eb, i64 %sv)
  br label %next
reduce:
  call void @__zluda_ptx_impl_agx_tma_red(i64 %gaddr, i32 %et, i32 %dir, i64 %sv)
  br label %next
next:
  ; advance (q0, ..., q4) through the box, dim 0 fastest
  %q0.i = add i32 %q0, 1
  %w0 = icmp eq i32 %q0.i, %bx0
  %q0.n = select i1 %w0, i32 0, i32 %q0.i
  %k0 = zext i1 %w0 to i32
  %q1.i = add i32 %q1, %k0
  %w1c = icmp eq i32 %q1.i, %b1r
  %q1.n = select i1 %w1c, i32 0, i32 %q1.i
  %k1 = zext i1 %w1c to i32
  %q2.i = add i32 %q2, %k1
  %w2c = icmp eq i32 %q2.i, %b2r
  %q2.n = select i1 %w2c, i32 0, i32 %q2.i
  %k2 = zext i1 %w2c to i32
  %q3.i = add i32 %q3, %k2
  %w3c = icmp eq i32 %q3.i, %b3r
  %q3.n = select i1 %w3c, i32 0, i32 %q3.i
  %k3 = zext i1 %w3c to i32
  %q4.n = add i32 %q4, %k3
  %e.next = add i32 %e, 1
  %more = icmp ult i32 %e.next, %count
  br i1 %more, label %loop, label %exit
exit:
  ret i32 %bytes
}

; The packed sub-byte element types (elemtype 13 b4x16, 14 b4x16_p64,
; 15 b6x16_p32): dim 0 counts elements, a group of 16 takes 8 (4-bit) or 12
; (6-bit) bytes in global memory and 8 (b4x16) or 16 bytes (data, then zero
; padding) in shared memory; groups are laid out like the elements of a tile,
; and a group outside the tensor reads as zeros. Returns the bytes of the box.
define i32 @__zluda_ptx_impl_agx_tma_tile_sub(ptr %tm, i32 %et, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4, i32 %smem, i32 %dir) #3 {
entry:
  %six = icmp eq i32 %et, 15
  %gb = select i1 %six, i32 12, i32 8
  %nb = select i1 %six, i64 6, i64 4
  %dense = icmp eq i32 %et, 13
  %sb = select i1 %dense, i32 8, i32 16
  %w2 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 2)
  %w2t = trunc i64 %w2 to i32
  %rank = and i32 %w2t, 255
  %base = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 3)
  %w13 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 13)
  %w14 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 14)
  %r1 = icmp ugt i32 %rank, 1
  %r2 = icmp ugt i32 %rank, 2
  %r3 = icmp ugt i32 %rank, 3
  %r4 = icmp ugt i32 %rank, 4
  %w13t = trunc i64 %w13 to i32
  %w13h = lshr i64 %w13, 32
  %w13ht = trunc i64 %w13h to i32
  %w14t = trunc i64 %w14 to i32
  %bx0 = and i32 %w13t, 65535
  %g0n = lshr i32 %bx0, 4
  %bx1 = lshr i32 %w13t, 16
  %bx2 = and i32 %w13ht, 65535
  %bx3 = lshr i32 %w13ht, 16
  %bx4 = and i32 %w14t, 65535
  %b1 = select i1 %r1, i32 %bx1, i32 1
  %b2 = select i1 %r2, i32 %bx2, i32 1
  %b3 = select i1 %r3, i32 %bx3, i32 1
  %b4 = select i1 %r4, i32 %bx4, i32 1
  %e1 = call i32 @__zluda_ptx_impl_agx_tma_est(ptr %tm, i32 1)
  %e2 = call i32 @__zluda_ptx_impl_agx_tma_est(ptr %tm, i32 2)
  %e3 = call i32 @__zluda_ptx_impl_agx_tma_est(ptr %tm, i32 3)
  %e4 = call i32 @__zluda_ptx_impl_agx_tma_est(ptr %tm, i32 4)
  %mode = call i32 @__zluda_ptx_impl_agx_tma_swizzle_mode(i64 %w14)
  %smem_rel = call i32 @__zluda_agx_cluster_mapa(i32 %smem, i32 0)
  %window = sub i32 %smem, %smem_rel
  %v01 = mul i32 %g0n, %b1
  %v012 = mul i32 %v01, %b2
  %v0123 = mul i32 %v012, %b3
  %groups = mul i32 %v0123, %b4
  %bytes = mul i32 %groups, %sb
  %empty = icmp eq i32 %groups, 0
  br i1 %empty, label %exit, label %loop
loop:
  %q = phi i32 [ 0, %entry ], [ %q.next, %next ]
  %q0 = phi i32 [ 0, %entry ], [ %q0.n, %next ]
  %q1 = phi i32 [ 0, %entry ], [ %q1.n, %next ]
  %q2 = phi i32 [ 0, %entry ], [ %q2.n, %next ]
  %q3 = phi i32 [ 0, %entry ], [ %q3.n, %next ]
  %q4 = phi i32 [ 0, %entry ], [ %q4.n, %next ]
  %t0 = shl i32 %q0, 4
  %col = add i32 %c0, %t0
  %o0 = call i64 @__zluda_ptx_impl_agx_tma_offset(ptr %tm, i32 0, i32 %col, i32 1)
  %t1 = mul i32 %q1, %e1
  %g1 = add i32 %c1, %t1
  %o1 = call i64 @__zluda_ptx_impl_agx_tma_offset(ptr %tm, i32 1, i32 %g1, i32 1)
  %t2 = mul i32 %q2, %e2
  %g2 = add i32 %c2, %t2
  %o2 = call i64 @__zluda_ptx_impl_agx_tma_offset(ptr %tm, i32 2, i32 %g2, i32 1)
  %t3 = mul i32 %q3, %e3
  %g3 = add i32 %c3, %t3
  %o3 = call i64 @__zluda_ptx_impl_agx_tma_offset(ptr %tm, i32 3, i32 %g3, i32 1)
  %t4 = mul i32 %q4, %e4
  %g4 = add i32 %c4, %t4
  %o4 = call i64 @__zluda_ptx_impl_agx_tma_offset(ptr %tm, i32 4, i32 %g4, i32 1)
  %bad0 = icmp slt i64 %o0, 0
  %bad1a = icmp slt i64 %o1, 0
  %bad1 = and i1 %bad1a, %r1
  %bad2a = icmp slt i64 %o2, 0
  %bad2 = and i1 %bad2a, %r2
  %bad3a = icmp slt i64 %o3, 0
  %bad3 = and i1 %bad3a, %r3
  %bad4a = icmp slt i64 %o4, 0
  %bad4 = and i1 %bad4a, %r4
  %bb1 = or i1 %bad0, %bad1
  %bb2 = or i1 %bb1, %bad2
  %bb3 = or i1 %bb2, %bad3
  %oob = or i1 %bb3, %bad4
  ; the group's first byte: its first element (col bits from the start), then the strided dims
  %colw = sext i32 %col to i64
  %bits = mul i64 %colw, %nb
  %a0 = lshr i64 %bits, 3
  %s1 = select i1 %r1, i64 %o1, i64 0
  %s2 = select i1 %r2, i64 %o2, i64 0
  %s3 = select i1 %r3, i64 %o3, i64 0
  %s4 = select i1 %r4, i64 %o4, i64 0
  %a01 = add i64 %a0, %s1
  %a012 = add i64 %a01, %s2
  %a0123 = add i64 %a012, %s3
  %a = add i64 %a0123, %s4
  %gaddr = add i64 %base, %a
  %gstart = mul i32 %q, %sb
  br label %byte
byte:
  %j = phi i32 [ 0, %loop ], [ %j.next, %byte ]
  %data = icmp ult i32 %j, %gb
  %jw = zext i32 %j to i64
  %ba = add i64 %gaddr, %jw
  %rel0 = add i32 %smem_rel, %gstart
  %rel = add i32 %rel0, %j
  %swizzled = call i32 @__zluda_ptx_impl_agx_tma_swizzle(i32 %mode, i32 %rel)
  %saddr = add i32 %window, %swizzled
  ; padding bytes: zeros when loading, nothing when storing
  %skip = xor i1 %data, true
  %elem_oob = or i1 %oob, %skip
  call void @__zluda_ptx_impl_agx_tma_elem(i64 %ba, i1 %elem_oob, i32 %saddr, i32 1, i32 0, i64 0, i32 %dir)
  %j.next = add i32 %j, 1
  %jmore = icmp ult i32 %j.next, %sb
  br i1 %jmore, label %byte, label %next
next:
  %q0.i = add i32 %q0, 1
  %w0 = icmp eq i32 %q0.i, %g0n
  %q0.n = select i1 %w0, i32 0, i32 %q0.i
  %k0 = zext i1 %w0 to i32
  %q1.i = add i32 %q1, %k0
  %w1c = icmp eq i32 %q1.i, %b1
  %q1.n = select i1 %w1c, i32 0, i32 %q1.i
  %k1 = zext i1 %w1c to i32
  %q2.i = add i32 %q2, %k1
  %w2c = icmp eq i32 %q2.i, %b2
  %q2.n = select i1 %w2c, i32 0, i32 %q2.i
  %k2 = zext i1 %w2c to i32
  %q3.i = add i32 %q3, %k2
  %w3c = icmp eq i32 %q3.i, %b3
  %q3.n = select i1 %w3c, i32 0, i32 %q3.i
  %k3 = zext i1 %w3c to i32
  %q4.n = add i32 %q4, %k3
  %q.next = add i32 %q, 1
  %more = icmp ult i32 %q.next, %groups
  br i1 %more, label %loop, label %exit
exit:
  ret i32 %bytes
}

; cp.async.bulk.tensor tile loads. %flags: bit 0 .multicast::cluster (to the
; same CTA-relative offset of every CTA in %mask), bit 1 .cta_group::2 (the
; signal for CTA d goes to the CTA of d's pair with the rank parity of the
; CTA holding %mbar)
define void @__zluda_ptx_impl_agx_tma_load(ptr addrspace(3) %dst, ptr %tm, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4, ptr addrspace(3) %mbar, i32 %mask, i32 %flags, i32 %imode, i32 %i0, i32 %i1, i32 %i2) #3 {
entry:
  %dst_a = ptrtoint ptr addrspace(3) %dst to i32
  %mbar_a = ptrtoint ptr addrspace(3) %mbar to i32
  %mc_bit = and i32 %flags, 1
  %mc = icmp ne i32 %mc_bit, 0
  br i1 %mc, label %multicast, label %single
single:
  %bytes = call i32 @__zluda_ptx_impl_agx_tma_copy(ptr %tm, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4, i32 %imode, i32 %i0, i32 %i1, i32 %i2, i32 %dst_a, i32 -1)
  call void @__zluda_ptx_impl_agx_mbarrier_complete_tx(ptr addrspace(3) %mbar, i32 %bytes)
  ret void
multicast:
  %g2_bit = and i32 %flags, 2
  %g2 = icmp ne i32 %g2_bit, 0
  %mbar_rank = call i32 @__zluda_agx_cluster_rank_of(i32 %mbar_a)
  %parity = and i32 %mbar_rank, 1
  br label %loop
loop:
  %rank = phi i32 [ 0, %multicast ], [ %next, %latch ]
  %bit = lshr i32 %mask, %rank
  %set = trunc i32 %bit to i1
  br i1 %set, label %copy, label %latch
copy:
  %to_a = call i32 @__zluda_agx_cluster_mapa(i32 %dst_a, i32 %rank)
  %n = call i32 @__zluda_ptx_impl_agx_tma_copy(ptr %tm, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4, i32 %imode, i32 %i0, i32 %i1, i32 %i2, i32 %to_a, i32 -1)
  %pair = and i32 %rank, -2
  %pair_rank = or i32 %pair, %parity
  %bar_rank = select i1 %g2, i32 %pair_rank, i32 %rank
  %bar_a = call i32 @__zluda_agx_cluster_mapa(i32 %mbar_a, i32 %bar_rank)
  %bar = inttoptr i32 %bar_a to ptr addrspace(3)
  call void @__zluda_ptx_impl_agx_mbarrier_complete_tx(ptr addrspace(3) %bar, i32 %n)
  br label %latch
latch:
  %next = add i32 %rank, 1
  %done = icmp eq i32 %next, 32
  br i1 %done, label %exit, label %loop
exit:
  ret void
}

; cp.async.bulk.tensor tile stores (%op = -2) and cp.reduce.async.bulk.tensor
; (%op = the reduction, see tma_red)
define void @__zluda_ptx_impl_agx_tma_store(ptr %tm, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4, ptr addrspace(3) %src, i32 %op, i32 %imode) #3 {
  %src_a = ptrtoint ptr addrspace(3) %src to i32
  %n = call i32 @__zluda_ptx_impl_agx_tma_copy(ptr %tm, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4, i32 %imode, i32 0, i32 0, i32 0, i32 %src_a, i32 %op)
  ret void
}

; ---------------------------------------------------------------- TMA im2col modes
; (the pixel enumeration of PTX-formalization's ptx-exec-tma.k: tensor dim 0
; = channels, dims 1..S = W, H, D, dim S + 1 = N; the corners of spatial dim k
; are offset-binary in bits 8 + 16 (k - 1) of words 1 (lower) and 2 (upper),
; the traversal strides are the element strides)

define i32 @__zluda_ptx_impl_agx_tma_sel(i32 %i, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4) #0 {
entry:
  switch i32 %i, label %d0 [ i32 1, label %d1 i32 2, label %d2 i32 3, label %d3 i32 4, label %d4 ]
d0:
  ret i32 %c0
d1:
  ret i32 %c1
d2:
  ret i32 %c2
d3:
  ret i32 %c3
d4:
  ret i32 %c4
}

define i32 @__zluda_ptx_impl_agx_tma_est(ptr %tm, i32 %k) #3 {
  %w = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 15)
  %sh = shl i32 %k, 3
  %sh64 = zext i32 %sh to i64
  %v = lshr i64 %w, %sh64
  %t = trunc i64 %v to i32
  %r = and i32 %t, 255
  ret i32 %r
}

; the lower (%word 1) or upper (%word 2) corner of spatial dim %k + 1
define i32 @__zluda_ptx_impl_agx_tma_corner(ptr %tm, i32 %word, i32 %k) #3 {
  %w = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 %word)
  %k16 = shl i32 %k, 4
  %sh = add i32 %k16, 8
  %sh64 = zext i32 %sh to i64
  %v = lshr i64 %w, %sh64
  %t = trunc i64 %v to i32
  %u = and i32 %t, 65535
  %r = sub i32 %u, 32768
  ret i32 %r
}

; floor(a / b) for b > 0
define i32 @__zluda_ptx_impl_agx_tma_floordiv(i32 %a, i32 %b) #0 {
  %q = sdiv i32 %a, %b
  %r = srem i32 %a, %b
  %has_r = icmp ne i32 %r, 0
  %neg = icmp slt i32 %a, 0
  %adj = and i1 %has_r, %neg
  %d = zext i1 %adj to i32
  %f = sub i32 %q, %d
  ret i32 %f
}

; the number of filter-base positions along spatial dim %k (1..S)
define i32 @__zluda_ptx_impl_agx_tma_nk(ptr %tm, i32 %k) #3 {
  %wi = add i32 %k, 4
  %dw = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 %wi)
  %dim = trunc i64 %dw to i32
  %k1 = sub i32 %k, 1
  %lo = call i32 @__zluda_ptx_impl_agx_tma_corner(ptr %tm, i32 1, i32 %k1)
  %up = call i32 @__zluda_ptx_impl_agx_tma_corner(ptr %tm, i32 2, i32 %k1)
  %st = call i32 @__zluda_ptx_impl_agx_tma_est(ptr %tm, i32 %k)
  %a = add i32 %dim, %up
  %b = sub i32 %a, %lo
  %num = sub i32 %b, 1
  %q = sdiv i32 %num, %st
  %n = add i32 %q, 1
  %r = call i32 @llvm.smax.i32(i32 %n, i32 1)
  ret i32 %r
}

; whether coordinate %v is outside dim %i, and its byte offset (dim 0: in elements of %eb bytes)
define i64 @__zluda_ptx_impl_agx_tma_offset(ptr %tm, i32 %i, i32 %v, i32 %eb) #3 {
entry:
  %wi = add i32 %i, 4
  %dim = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 %wi)
  %vw = sext i32 %v to i64
  %oob = icmp uge i64 %vw, %dim
  br i1 %oob, label %out, label %in
out:
  ret i64 -1
in:
  %is0 = icmp eq i32 %i, 0
  br i1 %is0, label %first, label %strided
first:
  %eb64 = zext i32 %eb to i64
  %o0 = mul i64 %vw, %eb64
  ret i64 %o0
strided:
  %si = add i32 %i, 8
  %stride = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 %si)
  %o = mul i64 %vw, %stride
  ret i64 %o
}

; the global address of element %e (pixel %e / channels, channel %e mod channels)
; of an im2col copy, or -1 outside the tensor. %mode: 0 im2col (offsets
; %i0..%i2), 1 im2col::w, 2 im2col::w::128 ({wHalo, wOffset} = {%i0, %i1}),
; 3 im2col_no_offs::w
define i64 @__zluda_ptx_impl_agx_tma_im2col_addr(ptr %tm, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4, i32 %i0, i32 %i1, i32 %i2, i32 %mode, i32 %e, i32 %eb) #3 {
entry:
  %w2 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 2)
  %w2t = trunc i64 %w2 to i32
  %rank = and i32 %w2t, 255
  %last = sub i32 %rank, 1
  %base = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 3)
  %w13 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 13)
  %w13t = trunc i64 %w13 to i32
  %box0 = and i32 %w13t, 65535
  %ch_i = urem i32 %e, %box0
  %pixel = udiv i32 %e, %box0
  %est0 = call i32 @__zluda_ptx_impl_agx_tma_est(ptr %tm, i32 0)
  %cht = mul i32 %ch_i, %est0
  %ch = add i32 %c0, %cht
  %choff = call i64 @__zluda_ptx_impl_agx_tma_offset(ptr %tm, i32 0, i32 %ch, i32 %eb)
  %chbad = icmp slt i64 %choff, 0
  br i1 %chbad, label %outside, label %spatial
spatial:
  %acc0 = add i64 %base, %choff
  %is_w = icmp ne i32 %mode, 0
  br i1 %is_w, label %wmode, label %start
start:
  ; the box index of the first pixel, in (N, D, H, W) order
  %sk = phi i32 [ 1, %spatial ], [ %sk.n, %start_k ]
  %smu = phi i32 [ 1, %spatial ], [ %smu.n, %start_k ]
  %sacc = phi i32 [ 0, %spatial ], [ %sacc.n, %start_k ]
  %smore = icmp slt i32 %sk, %last
  br i1 %smore, label %start_k, label %start_n
start_k:
  %sc = call i32 @__zluda_ptx_impl_agx_tma_sel(i32 %sk, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4)
  %sk1 = sub i32 %sk, 1
  %slo = call i32 @__zluda_ptx_impl_agx_tma_corner(ptr %tm, i32 1, i32 %sk1)
  %sst = call i32 @__zluda_ptx_impl_agx_tma_est(ptr %tm, i32 %sk)
  %sdiff = sub i32 %sc, %slo
  %sq = call i32 @__zluda_ptx_impl_agx_tma_floordiv(i32 %sdiff, i32 %sst)
  %sterm = mul i32 %sq, %smu
  %sacc.n = add i32 %sacc, %sterm
  %snk = call i32 @__zluda_ptx_impl_agx_tma_nk(ptr %tm, i32 %sk)
  %smu.n = mul i32 %smu, %snk
  %sk.n = add i32 %sk, 1
  br label %start
start_n:
  %scn = call i32 @__zluda_ptx_impl_agx_tma_sel(i32 %last, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4)
  %sn = mul i32 %scn, %smu
  %first = add i32 %sacc, %sn
  %idx0 = add i32 %first, %pixel
  br label %pos
pos:
  ; spatial dim k at lower corner + (idx mod nk) * stride + offset
  %k = phi i32 [ 1, %start_n ], [ %k.n, %pos_k ]
  %idx = phi i32 [ %idx0, %start_n ], [ %idx.n, %pos_k ]
  %acc = phi i64 [ %acc0, %start_n ], [ %acc.n, %pos_k ]
  %more = icmp slt i32 %k, %last
  br i1 %more, label %pos_k, label %pos_n
pos_k:
  %nk = call i32 @__zluda_ptx_impl_agx_tma_nk(ptr %tm, i32 %k)
  %k1 = sub i32 %k, 1
  %lo = call i32 @__zluda_ptx_impl_agx_tma_corner(ptr %tm, i32 1, i32 %k1)
  %st = call i32 @__zluda_ptx_impl_agx_tma_est(ptr %tm, i32 %k)
  %im = srem i32 %idx, %nk
  %imt = mul i32 %im, %st
  %pa = add i32 %lo, %imt
  %io = call i32 @__zluda_ptx_impl_agx_tma_sel(i32 %k1, i32 %i0, i32 %i1, i32 %i2, i32 0, i32 0)
  %p = add i32 %pa, %io
  %po = call i64 @__zluda_ptx_impl_agx_tma_offset(ptr %tm, i32 %k, i32 %p, i32 %eb)
  %pbad = icmp slt i64 %po, 0
  %acc.n = add i64 %acc, %po
  %idx.n = sdiv i32 %idx, %nk
  %k.n = add i32 %k, 1
  br i1 %pbad, label %outside, label %pos
pos_n:
  %no = call i64 @__zluda_ptx_impl_agx_tma_offset(ptr %tm, i32 %last, i32 %idx, i32 %eb)
  br label %finish
wmode:
  ; only W varies: c_W + q * stride + wOffset
  %w128 = icmp eq i32 %mode, 2
  %period = add i32 %i0, 32
  %pq = udiv i32 %pixel, %period
  %pr = urem i32 %pixel, %period
  %pq32 = mul i32 %pq, 32
  %q128 = add i32 %pq32, %pr
  %q = select i1 %w128, i32 %q128, i32 %pixel
  %st1 = call i32 @__zluda_ptx_impl_agx_tma_est(ptr %tm, i32 1)
  %qs = mul i32 %q, %st1
  %no_offs = icmp eq i32 %mode, 3
  %woff = select i1 %no_offs, i32 0, i32 %i1
  %shift = add i32 %qs, %woff
  br label %wpos
wpos:
  %wk = phi i32 [ 1, %wmode ], [ %wk.n, %wpos_k ]
  %wacc = phi i64 [ %acc0, %wmode ], [ %wacc.n, %wpos_k ]
  %wmore = icmp slt i32 %wk, %last
  br i1 %wmore, label %wpos_k, label %wpos_n
wpos_k:
  %wc = call i32 @__zluda_ptx_impl_agx_tma_sel(i32 %wk, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4)
  %is_w1 = icmp eq i32 %wk, 1
  %wsh = select i1 %is_w1, i32 %shift, i32 0
  %wp = add i32 %wc, %wsh
  %wo = call i64 @__zluda_ptx_impl_agx_tma_offset(ptr %tm, i32 %wk, i32 %wp, i32 %eb)
  %wbad = icmp slt i64 %wo, 0
  %wacc.n = add i64 %wacc, %wo
  %wk.n = add i32 %wk, 1
  br i1 %wbad, label %outside, label %wpos
wpos_n:
  %wcn = call i32 @__zluda_ptx_impl_agx_tma_sel(i32 %last, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4)
  %wno = call i64 @__zluda_ptx_impl_agx_tma_offset(ptr %tm, i32 %last, i32 %wcn, i32 %eb)
  br label %finish
finish:
  %fo = phi i64 [ %no, %pos_n ], [ %wno, %wpos_n ]
  %facc = phi i64 [ %acc, %pos_n ], [ %wacc, %wpos_n ]
  %fbad = icmp slt i64 %fo, 0
  %fa = add i64 %facc, %fo
  %r = select i1 %fbad, i64 -1, i64 %fa
  ret i64 %r
outside:
  ret i64 -1
}

; An im2col copy (see tma_tile for %dir). The column holds `pixels` pixels of
; box dim 0 channels: box dim 1 pixels, plus wHalo (im2col::w), or 128 plus
; 4 wHalo (::128). im2col_no_offs::w stores stop at the first pixel outside
; the tensor.
define i32 @__zluda_ptx_impl_agx_tma_im2col(ptr %tm, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4, i32 %i0, i32 %i1, i32 %i2, i32 %mode, i32 %smem, i32 %dir) #3 {
entry:
  %w1 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 1)
  %w1t = trunc i64 %w1 to i32
  %et = and i32 %w1t, 255
  %eb = call i32 @__zluda_ptx_impl_agx_tma_bytes(i32 %et)
  %w13 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 13)
  %w13t = trunc i64 %w13 to i32
  %box0 = and i32 %w13t, 65535
  %box1 = lshr i32 %w13t, 16
  %halo4 = shl i32 %i0, 2
  %p128 = add i32 %halo4, 128
  %pw = add i32 %box1, %i0
  %is1 = icmp eq i32 %mode, 1
  %is2 = icmp eq i32 %mode, 2
  %pa = select i1 %is1, i32 %pw, i32 %box1
  %pixels = select i1 %is2, i32 %p128, i32 %pa
  %count = mul i32 %pixels, %box0
  %bytes = mul i32 %count, %eb
  %w14 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 14)
  %mode_s = call i32 @__zluda_ptx_impl_agx_tma_swizzle_mode(i64 %w14)
  %fill = call i64 @__zluda_ptx_impl_agx_tma_fill(i64 %w14, i32 %et, i32 %eb)
  %smem_rel = call i32 @__zluda_agx_cluster_mapa(i32 %smem, i32 0)
  %window = sub i32 %smem, %smem_rel
  ; im2col_no_offs::w: a W start outside the tensor stores nothing
  %is3 = icmp eq i32 %mode, 3
  %neg_w = icmp slt i32 %c1, 0
  %none = and i1 %is3, %neg_w
  %empty = icmp eq i32 %count, 0
  %skip = or i1 %empty, %none
  br i1 %skip, label %exit, label %loop
loop:
  %e = phi i32 [ 0, %entry ], [ %e.next, %loop ]
  %gaddr = call i64 @__zluda_ptx_impl_agx_tma_im2col_addr(ptr %tm, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4, i32 %i0, i32 %i1, i32 %i2, i32 %mode, i32 %e, i32 %eb)
  %oob = icmp slt i64 %gaddr, 0
  %eoff = mul i32 %e, %eb
  %rel = add i32 %smem_rel, %eoff
  %swizzled = call i32 @__zluda_ptx_impl_agx_tma_swizzle(i32 %mode_s, i32 %rel)
  %saddr = add i32 %window, %swizzled
  call void @__zluda_ptx_impl_agx_tma_elem(i64 %gaddr, i1 %oob, i32 %saddr, i32 %eb, i32 %et, i64 %fill, i32 %dir)
  %e.next = add i32 %e, 1
  %more = icmp ult i32 %e.next, %count
  br i1 %more, label %loop, label %exit
exit:
  ret i32 %bytes
}

; the swizzle (see tma_swizzle) of swizzle_mode / swizzle_atomicity in word 14
define i32 @__zluda_ptx_impl_agx_tma_swizzle_mode(i64 %w14) #0 {
  %w14t = trunc i64 %w14 to i32
  %swz = lshr i32 %w14t, 16
  %swzm = and i32 %swz, 255
  %w14h = lshr i64 %w14, 48
  %w14ht = trunc i64 %w14h to i32
  %atom = and i32 %w14ht, 255
  %is128 = icmp eq i32 %swzm, 3
  %m128 = select i1 %is128, i32 1, i32 0
  %is64 = icmp eq i32 %swzm, 2
  %is32 = icmp eq i32 %swzm, 1
  %m64 = select i1 %is64, i32 2, i32 %m128
  %m32 = select i1 %is32, i32 3, i32 %m64
  %atomic = add i32 %atom, 3
  %has_atom = icmp ne i32 %atom, 0
  %atom128 = and i1 %is128, %has_atom
  %mode = select i1 %atom128, i32 %atomic, i32 %m32
  ret i32 %mode
}

; the out-of-bounds fill: zero, or (fill_mode != 0) a NaN of the element type
define i64 @__zluda_ptx_impl_agx_tma_fill(i64 %w14, i32 %et, i32 %eb) #0 {
  %w14t = trunc i64 %w14 to i32
  %fm = lshr i32 %w14t, 24
  %fm8 = and i32 %fm, 255
  %nan_fill = icmp ne i32 %fm8, 0
  %f4 = icmp eq i32 %eb, 4
  %fh = icmp eq i32 %et, 6
  %fbf = icmp eq i32 %et, 10
  %fd = icmp eq i32 %et, 9
  %n0 = select i1 %fd, i64 9221120237041090560, i64 0
  %n1 = select i1 %fbf, i64 32704, i64 %n0
  %n2 = select i1 %fh, i64 32256, i64 %n1
  %n3 = select i1 %f4, i64 2143289344, i64 %n2
  %fill = select i1 %nan_fill, i64 %n3, i64 0
  ret i64 %fill
}

; one element of a copy (see tma_tile for %dir)
define void @__zluda_ptx_impl_agx_tma_elem(i64 %gaddr, i1 %oob, i32 %saddr, i32 %eb, i32 %et, i64 %fill, i32 %dir) #3 {
entry:
  %is_load = icmp eq i32 %dir, -1
  br i1 %is_load, label %load, label %out
load:
  br i1 %oob, label %load_fill, label %load_mem
load_mem:
  %lv = call i64 @__zluda_ptx_impl_agx_tma_load_global(i64 %gaddr, i32 %eb)
  br label %load_fill
load_fill:
  %value = phi i64 [ %fill, %load ], [ %lv, %load_mem ]
  call void @__zluda_ptx_impl_agx_tma_store_shared(i32 %saddr, i32 %eb, i64 %value)
  ret void
out:
  br i1 %oob, label %exit, label %out_elem
out_elem:
  %sv = call i64 @__zluda_ptx_impl_agx_tma_load_shared(i32 %saddr, i32 %eb)
  %is_store = icmp eq i32 %dir, -2
  br i1 %is_store, label %store, label %reduce
store:
  call void @__zluda_ptx_impl_agx_tma_store_global(i64 %gaddr, i32 %eb, i64 %sv)
  ret void
reduce:
  call void @__zluda_ptx_impl_agx_tma_red(i64 %gaddr, i32 %et, i32 %dir, i64 %sv)
  ret void
exit:
  ret void
}

; a tile (%imode = -1) or im2col (%imode = its mode) copy
define i32 @__zluda_ptx_impl_agx_tma_copy(ptr %tm, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4, i32 %imode, i32 %i0, i32 %i1, i32 %i2, i32 %smem, i32 %dir) #3 {
entry:
  %tile = icmp slt i32 %imode, 0
  br i1 %tile, label %t, label %i
t:
  %nt = call i32 @__zluda_ptx_impl_agx_tma_tile(ptr %tm, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4, i32 %smem, i32 %dir)
  ret i32 %nt
i:
  %ni = call i32 @__zluda_ptx_impl_agx_tma_im2col(ptr %tm, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4, i32 %i0, i32 %i1, i32 %i2, i32 %imode, i32 %smem, i32 %dir)
  ret i32 %ni
}

; The tensor map at %tm patched by the .override forms into the 16 words at
; %dst: the global address (word 3, if %has_address), the first %nd dims
; (words 4..) and the first %ns strides (words 9..; stride i is
; (low_i + (((%up >> 4 i) & 15) << 32)) << 4)
define void @__zluda_ptx_impl_agx_tma_override(ptr %dst, ptr %tm, i1 %has_address, i64 %address, i32 %nd, i32 %d0, i32 %d1, i32 %d2, i32 %d3, i32 %d4, i32 %ns, i32 %s0, i32 %s1, i32 %s2, i32 %s3, i32 %up) #3 {
entry:
  call void @llvm.memcpy.p0.p0.i32(ptr align 8 %dst, ptr align 8 %tm, i32 128, i1 false)
  br i1 %has_address, label %address_w, label %dims
address_w:
  %pa = getelementptr inbounds i64, ptr %dst, i32 3
  store i64 %address, ptr %pa, align 8
  br label %dims
dims:
  %i = phi i32 [ 0, %entry ], [ 0, %address_w ], [ %i.n, %dim ]
  %dmore = icmp ult i32 %i, %nd
  br i1 %dmore, label %dim, label %strides
dim:
  %dv = call i32 @__zluda_ptx_impl_agx_tma_sel(i32 %i, i32 %d0, i32 %d1, i32 %d2, i32 %d3, i32 %d4)
  %dw = zext i32 %dv to i64
  %di = add i32 %i, 4
  %pd = getelementptr inbounds i64, ptr %dst, i32 %di
  store i64 %dw, ptr %pd, align 8
  %i.n = add i32 %i, 1
  br label %dims
strides:
  %j = phi i32 [ 0, %dims ], [ %j.n, %stride ]
  %smore = icmp ult i32 %j, %ns
  br i1 %smore, label %stride, label %exit
stride:
  %sv = call i32 @__zluda_ptx_impl_agx_tma_sel(i32 %j, i32 %s0, i32 %s1, i32 %s2, i32 %s3, i32 0)
  %sl = zext i32 %sv to i64
  %ush = shl i32 %j, 2
  %u = lshr i32 %up, %ush
  %u4 = and i32 %u, 15
  %uw = zext i32 %u4 to i64
  %uh = shl i64 %uw, 32
  %sum = add i64 %sl, %uh
  %bytes = shl i64 %sum, 4
  %si = add i32 %j, 9
  %ps = getelementptr inbounds i64, ptr %dst, i32 %si
  store i64 %bytes, ptr %ps, align 8
  %j.n = add i32 %j, 1
  br label %strides
exit:
  ret void
}

; A .trap surface access out of bounds: the host sees CUDA_ERROR_ASSERT, as
; for trap
define void @__zluda_ptx_impl_agx_surface_trap() #1 {
  store volatile i32 1, ptr addrspace(1) @__zluda_agx_trap_flag, align 4
  fence release
  call void @llvm.trap()
  unreachable
}

; tensormap.replace.tile: %field 0 global_address, 1 rank, 2 box_dim,
; 3 global_dim, 4 global_stride, 5 element_stride, 6 elemtype,
; 7 interleave_layout, 8 swizzle_mode, 9 swizzle_atomicity, 10 fill_mode
define void @__zluda_ptx_impl_agx_tensormap_replace(ptr %tm, i32 %field, i32 %ord, i64 %v) #3 {
entry:
  %ord8 = shl i32 %ord, 3
  %ord16 = shl i32 %ord, 4
  %v32 = trunc i64 %v to i32
  switch i32 %field, label %exit [ i32 0, label %address i32 1, label %rank i32 2, label %box i32 3, label %dim
                                   i32 4, label %stride i32 5, label %estride i32 6, label %elemtype
                                   i32 7, label %interleave i32 8, label %swizzle i32 9, label %atomicity i32 10, label %fill ]
address:
  br label %set_word
rank:
  ; the operand holds rank - 1
  %r = add i64 %v, 1
  %r8 = and i64 %r, 255
  br label %low_byte2
low_byte2:
  %w2 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 2)
  %w2c = and i64 %w2, -256
  %w2n = or i64 %w2c, %r8
  br label %set_word_at
elemtype:
  %et8 = and i64 %v, 255
  %w1 = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 1)
  %w1c = and i64 %w1, -256
  %w1n = or i64 %w1c, %et8
  br label %set_word_at
dim:
  %dim_i = add i32 %ord, 4
  br label %set_word_at
stride:
  %stride_i = add i32 %ord, 9
  br label %set_word_at
box:
  %low_box = icmp ult i32 %ord, 4
  %box_i = select i1 %low_box, i32 13, i32 14
  %box_sh = select i1 %low_box, i32 %ord16, i32 0
  br label %field_bits
estride:
  br label %field_bits
interleave:
  br label %field_bits
swizzle:
  br label %field_bits
atomicity:
  br label %field_bits
fill:
  br label %field_bits
field_bits:
  ; word, shift and width of a bit field
  %fw = phi i32 [ %box_i, %box ], [ 15, %estride ], [ 14, %interleave ], [ 14, %swizzle ], [ 14, %atomicity ], [ 14, %fill ]
  %fs = phi i32 [ %box_sh, %box ], [ %ord8, %estride ], [ 32, %interleave ], [ 16, %swizzle ], [ 48, %atomicity ], [ 24, %fill ]
  %fm = phi i64 [ 65535, %box ], [ 255, %estride ], [ 255, %interleave ], [ 255, %swizzle ], [ 255, %atomicity ], [ 255, %fill ]
  %old = call i64 @__zluda_ptx_impl_agx_tma_word(ptr %tm, i32 %fw)
  %fs64 = zext i32 %fs to i64
  %mask = shl i64 %fm, %fs64
  %keep = xor i64 %mask, -1
  %cleared = and i64 %old, %keep
  %vm = and i64 %v, %fm
  %vs = shl i64 %vm, %fs64
  %new = or i64 %cleared, %vs
  br label %set_word_at
set_word:
  br label %set_word_at
set_word_at:
  %wi = phi i32 [ 3, %set_word ], [ 2, %low_byte2 ], [ 1, %elemtype ], [ %dim_i, %dim ], [ %stride_i, %stride ], [ %fw, %field_bits ]
  %wv = phi i64 [ %v, %set_word ], [ %w2n, %low_byte2 ], [ %w1n, %elemtype ], [ %v, %dim ], [ %v, %stride ], [ %new, %field_bits ]
  %p = getelementptr inbounds i64, ptr %tm, i32 %wi
  store i64 %wv, ptr %p, align 8
  br label %exit
exit:
  ret void
}

declare float @llvm.fabs.f32(float)
declare float @llvm.fma.f32(float, float, float)
declare void @llvm.memcpy.p0.p0.i32(ptr, ptr, i32, i1)
declare i32 @llvm.smax.i32(i32, i32)
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
declare i32 @llvm.agx.simd.shuffle.up.i32(i32, i32)
declare i32 @llvm.agx.simd.shuffle.down.i32(i32, i32)
declare i32 @llvm.agx.simd.shuffle.xor.i32(i32, i32)
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
; ---------------------------------------------------------------- BEGIN GENERATED (gen_agx_tex.py)
; regenerate with ptx/lib/gen_agx_tex.py

; from zluda_ptx_impl_agx_tex.c

%struct.sampling = type { i32, i32, i32, i32, double, double }
%struct.q = type { i32, [3 x i32], i32, i32 }

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define dso_local <4 x i32> @__zluda_ptx_impl_agx_tex(ptr addrspace(1) nocapture noundef readonly %0, ptr addrspace(1) noundef readonly %1, i32 noundef %2, i32 noundef %3, i32 noundef %4, i32 noundef %5, i32 noundef %6, i32 noundef %7, i32 noundef %8, i32 noundef %9, float noundef %10, float noundef %11, float noundef %12, float noundef %13, float noundef %14, float noundef %15, float noundef %16, i32 noundef %17, i32 noundef %18, i32 noundef %19, i32 noundef %20, i32 noundef %21) local_unnamed_addr #3 {
  %23 = alloca %struct.sampling, align 8
  %24 = alloca %struct.q, align 4
  %25 = alloca [3 x double], align 8
  %26 = alloca [3 x double], align 8
  %27 = alloca [4 x i32], align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %23)
  %28 = icmp eq ptr addrspace(1) %1, null
  br i1 %28, label %53, label %29

29:                                               ; preds = %22
  %30 = getelementptr inbounds i64, ptr addrspace(1) %1, i64 1
  %31 = load i64, ptr addrspace(1) %30, align 8
  %32 = getelementptr inbounds i64, ptr addrspace(1) %1, i64 2
  %33 = load i64, ptr addrspace(1) %32, align 8
  %34 = getelementptr inbounds i64, ptr addrspace(1) %1, i64 3
  %35 = load i64, ptr addrspace(1) %34, align 8
  %36 = icmp eq i64 %35, 0
  br i1 %36, label %42, label %37

37:                                               ; preds = %29
  %38 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 6
  %39 = load i64, ptr addrspace(1) %38, align 8
  %40 = icmp ne i64 %39, 0
  %41 = zext i1 %40 to i32
  br label %42

42:                                               ; preds = %37, %29
  %43 = phi i32 [ 0, %29 ], [ %41, %37 ]
  %44 = getelementptr inbounds i64, ptr addrspace(1) %1, i64 4
  %45 = getelementptr inbounds i64, ptr addrspace(1) %1, i64 5
  %46 = load i64, ptr addrspace(1) %45, align 8
  %47 = sitofp i64 %46 to double
  %48 = fmul double %47, 3.906250e-03
  %49 = getelementptr inbounds i64, ptr addrspace(1) %1, i64 6
  %50 = load i64, ptr addrspace(1) %49, align 8
  %51 = sitofp i64 %50 to double
  %52 = fmul double %51, 3.906250e-03
  br label %62

53:                                               ; preds = %22
  %54 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 4
  %55 = load i64, ptr addrspace(1) %54, align 8
  %56 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 5
  %57 = load i64, ptr addrspace(1) %56, align 8
  %58 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 6
  %59 = load i64, ptr addrspace(1) %58, align 8
  %60 = trunc i64 %59 to i32
  %61 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 15
  br label %62

62:                                               ; preds = %42, %53
  %63 = phi ptr addrspace(1) [ %44, %42 ], [ %61, %53 ]
  %64 = phi i32 [ %43, %42 ], [ %60, %53 ]
  %65 = phi i64 [ %33, %42 ], [ %57, %53 ]
  %66 = phi i64 [ %31, %42 ], [ %55, %53 ]
  %67 = phi double [ %48, %42 ], [ 0.000000e+00, %53 ]
  %68 = phi double [ %52, %42 ], [ 4.096000e+03, %53 ]
  %69 = load i64, ptr addrspace(1) %63, align 8
  %70 = trunc i64 %66 to i32
  %71 = trunc i64 %65 to i32
  %72 = trunc i64 %69 to i32
  store i32 %70, ptr %23, align 8
  %73 = getelementptr inbounds %struct.sampling, ptr %23, i64 0, i32 1
  store i32 %71, ptr %73, align 4
  %74 = getelementptr inbounds %struct.sampling, ptr %23, i64 0, i32 2
  store i32 %64, ptr %74, align 8
  %75 = getelementptr inbounds %struct.sampling, ptr %23, i64 0, i32 3
  store i32 %72, ptr %75, align 4
  %76 = getelementptr inbounds %struct.sampling, ptr %23, i64 0, i32 4
  store double %67, ptr %76, align 8
  %77 = getelementptr inbounds %struct.sampling, ptr %23, i64 0, i32 5
  store double %68, ptr %77, align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %24)
  store i32 0, ptr %24, align 4
  %78 = getelementptr inbounds %struct.q, ptr %24, i64 0, i32 1
  store i32 %17, ptr %78, align 4
  %79 = getelementptr inbounds %struct.q, ptr %24, i64 0, i32 1, i64 1
  store i32 %18, ptr %79, align 4
  %80 = getelementptr inbounds %struct.q, ptr %24, i64 0, i32 1, i64 2
  store i32 %19, ptr %80, align 4
  %81 = getelementptr inbounds %struct.q, ptr %24, i64 0, i32 2
  store i32 %20, ptr %81, align 4
  %82 = getelementptr inbounds %struct.q, ptr %24, i64 0, i32 3
  store i32 %21, ptr %82, align 4
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %25)
  %83 = fpext float %11 to double
  store double %83, ptr %25, align 8
  %84 = getelementptr inbounds double, ptr %25, i64 1
  %85 = fpext float %12 to double
  store double %85, ptr %84, align 8
  %86 = getelementptr inbounds double, ptr %25, i64 2
  %87 = fpext float %13 to double
  store double %87, ptr %86, align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %26)
  %88 = fpext float %14 to double
  store double %88, ptr %26, align 8
  %89 = getelementptr inbounds double, ptr %26, i64 1
  %90 = fpext float %15 to double
  store double %90, ptr %89, align 8
  %91 = getelementptr inbounds double, ptr %26, i64 2
  %92 = fpext float %16 to double
  store double %92, ptr %91, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %27)
  store i32 %6, ptr %27, align 4
  %93 = getelementptr inbounds i32, ptr %27, i64 1
  store i32 %7, ptr %93, align 4
  %94 = getelementptr inbounds i32, ptr %27, i64 2
  store i32 %8, ptr %94, align 4
  %95 = getelementptr inbounds i32, ptr %27, i64 3
  store i32 %9, ptr %95, align 4
  %96 = add i32 %2, -7
  %97 = icmp ult i32 %96, 2
  br i1 %97, label %98, label %369

98:                                               ; preds = %62
  %99 = icmp eq i32 %2, 8
  %100 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 13
  %101 = load i64, ptr addrspace(1) %100, align 8
  %102 = lshr i64 %101, 8
  %103 = and i64 %102, 255
  %104 = tail call noundef i64 @llvm.smax.i64(i64 %103, i64 1)
  %105 = zext i1 %99 to i64
  %106 = getelementptr inbounds [4 x i32], ptr %27, i64 0, i64 %105
  %107 = load i32, ptr %106, align 4
  %108 = zext i32 %107 to i64
  %109 = add nsw i64 %104, -1
  %110 = tail call noundef i64 @llvm.smin.i64(i64 %108, i64 %109)
  %111 = select i1 %99, i64 2, i64 1
  %112 = getelementptr inbounds [4 x i32], ptr %27, i64 0, i64 %111
  %113 = load i32, ptr %112, align 4
  %114 = sext i32 %113 to i64
  %115 = sext i32 %17 to i64
  %116 = add nsw i64 %114, %115
  %117 = getelementptr i64, ptr addrspace(1) %0, i64 9
  %118 = load i64, ptr addrspace(1) %117, align 8
  %119 = tail call noundef i64 @llvm.smax.i64(i64 %118, i64 1)
  switch i32 %71, label %137 [
    i32 0, label %120
    i32 2, label %124
    i32 3, label %128
  ]

120:                                              ; preds = %98
  %121 = srem i64 %116, %119
  %122 = add nsw i64 %121, %119
  %123 = srem i64 %122, %119
  br label %141

124:                                              ; preds = %98
  %125 = icmp slt i64 %116, %119
  %126 = tail call i64 @llvm.smax.i64(i64 %116, i64 -1)
  %127 = select i1 %125, i64 %126, i64 -1
  br label %141

128:                                              ; preds = %98
  %129 = shl nuw nsw i64 %119, 1
  %130 = srem i64 %116, %129
  %131 = add nsw i64 %130, %129
  %132 = srem i64 %131, %129
  %133 = icmp slt i64 %132, %119
  %134 = xor i64 %132, -1
  %135 = add i64 %129, %134
  %136 = select i1 %133, i64 %132, i64 %135
  br label %141

137:                                              ; preds = %98
  %138 = tail call noundef i64 @llvm.smax.i64(i64 %116, i64 0)
  %139 = add nsw i64 %119, -1
  %140 = tail call noundef i64 @llvm.smin.i64(i64 %138, i64 %139)
  br label %141

141:                                              ; preds = %120, %124, %128, %137
  %142 = phi i64 [ %140, %137 ], [ %136, %128 ], [ %127, %124 ], [ %123, %120 ]
  %143 = select i1 %99, i64 3, i64 2
  %144 = getelementptr inbounds [4 x i32], ptr %27, i64 0, i64 %143
  %145 = load i32, ptr %144, align 4
  %146 = sext i32 %145 to i64
  %147 = sext i32 %18 to i64
  %148 = add nsw i64 %146, %147
  %149 = getelementptr i64, ptr addrspace(1) %0, i64 1
  %150 = getelementptr i64, ptr addrspace(1) %0, i64 10
  %151 = load i64, ptr addrspace(1) %150, align 8
  %152 = tail call noundef i64 @llvm.smax.i64(i64 %151, i64 1)
  switch i32 %71, label %170 [
    i32 0, label %153
    i32 2, label %157
    i32 3, label %161
  ]

153:                                              ; preds = %141
  %154 = srem i64 %148, %152
  %155 = add nsw i64 %154, %152
  %156 = srem i64 %155, %152
  br label %174

157:                                              ; preds = %141
  %158 = icmp slt i64 %148, %152
  %159 = tail call i64 @llvm.smax.i64(i64 %148, i64 -1)
  %160 = select i1 %158, i64 %159, i64 -1
  br label %174

161:                                              ; preds = %141
  %162 = shl nuw nsw i64 %152, 1
  %163 = srem i64 %148, %162
  %164 = add nsw i64 %163, %162
  %165 = srem i64 %164, %162
  %166 = icmp slt i64 %165, %152
  %167 = xor i64 %165, -1
  %168 = add i64 %162, %167
  %169 = select i1 %166, i64 %165, i64 %168
  br label %174

170:                                              ; preds = %141
  %171 = tail call noundef i64 @llvm.smax.i64(i64 %148, i64 0)
  %172 = add nsw i64 %152, -1
  %173 = tail call noundef i64 @llvm.smin.i64(i64 %171, i64 %172)
  br label %174

174:                                              ; preds = %153, %157, %161, %170
  %175 = phi i64 [ %173, %170 ], [ %169, %161 ], [ %160, %157 ], [ %156, %153 ]
  br i1 %99, label %176, label %183

176:                                              ; preds = %174
  %177 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 8
  %178 = zext i32 %6 to i64
  %179 = load i64, ptr addrspace(1) %177, align 8
  %180 = tail call noundef i64 @llvm.smax.i64(i64 %179, i64 1)
  %181 = add nsw i64 %180, -1
  %182 = tail call noundef i64 @llvm.smin.i64(i64 %178, i64 %181)
  br label %183

183:                                              ; preds = %174, %176
  %184 = phi i64 [ %182, %176 ], [ 0, %174 ]
  %185 = icmp sgt i64 %142, -1
  %186 = icmp sgt i64 %175, -1
  %187 = select i1 %185, i1 %186, i1 false
  br i1 %187, label %188, label %727

188:                                              ; preds = %183
  %189 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %190 = load i64, ptr addrspace(1) %189, align 8
  %191 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 2
  %192 = load i64, ptr addrspace(1) %191, align 8
  switch i64 %192, label %194 [
    i64 0, label %195
    i64 20, label %195
    i64 1, label %193
    i64 21, label %193
    i64 6, label %193
  ]

193:                                              ; preds = %188, %188, %188
  br label %195

194:                                              ; preds = %188
  br label %195

195:                                              ; preds = %188, %188, %193, %194
  %196 = phi i32 [ 4, %194 ], [ 2, %193 ], [ 1, %188 ], [ 1, %188 ]
  %197 = mul nsw i64 %184, %152
  %198 = add nsw i64 %197, %175
  %199 = mul nsw i64 %198, %119
  %200 = add nsw i64 %199, %142
  %201 = mul nsw i64 %200, %104
  %202 = add nsw i64 %201, %110
  %203 = load i64, ptr addrspace(1) %149, align 8
  %204 = mul nsw i64 %190, %202
  %205 = zext nneg i32 %196 to i64
  %206 = mul nsw i64 %204, %205
  %207 = add i64 %206, %203
  %208 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 7
  %209 = icmp eq i64 %192, 5
  %210 = sext i1 %209 to i32
  br label %211

211:                                              ; preds = %195, %364
  %212 = phi <4 x double> [ zeroinitializer, %195 ], [ %366, %364 ]
  %213 = phi i32 [ 0, %195 ], [ %367, %364 ]
  %214 = zext nneg i32 %213 to i64
  %215 = icmp sgt i64 %190, %214
  br i1 %215, label %216, label %361

216:                                              ; preds = %211
  %217 = mul nuw nsw i32 %213, %196
  %218 = zext nneg i32 %217 to i64
  %219 = add i64 %207, %218
  switch i64 %192, label %221 [
    i64 0, label %222
    i64 20, label %222
    i64 1, label %220
    i64 21, label %220
    i64 6, label %220
  ]

220:                                              ; preds = %216, %216, %216
  br label %222

221:                                              ; preds = %216
  br label %222

222:                                              ; preds = %221, %220, %216, %216
  %223 = phi i32 [ 4, %221 ], [ 2, %220 ], [ 1, %216 ], [ 1, %216 ]
  %224 = inttoptr i64 %219 to ptr addrspace(1)
  br label %226

225:                                              ; preds = %226
  switch i64 %192, label %329 [
    i64 6, label %238
    i64 7, label %325
    i64 0, label %330
    i64 20, label %330
    i64 1, label %328
    i64 21, label %328
  ]

226:                                              ; preds = %226, %222
  %227 = phi i32 [ 0, %222 ], [ %235, %226 ]
  %228 = phi i32 [ 0, %222 ], [ %236, %226 ]
  %229 = zext nneg i32 %228 to i64
  %230 = getelementptr inbounds i8, ptr addrspace(1) %224, i64 %229
  %231 = load i8, ptr addrspace(1) %230, align 1
  %232 = zext i8 %231 to i32
  %233 = shl nsw i32 %228, 3
  %234 = shl i32 %232, %233
  %235 = or i32 %234, %227
  %236 = add nuw nsw i32 %228, 1
  %237 = icmp eq i32 %236, %223
  br i1 %237, label %225, label %226, !llvm.loop !10016

238:                                              ; preds = %225
  %239 = lshr i32 %235, 10
  %240 = and i32 %239, 31
  %241 = and i32 %235, 1023
  switch i32 %240, label %248 [
    i32 0, label %242
    i32 31, label %245
  ]

242:                                              ; preds = %238
  %243 = uitofp i32 %241 to double
  %244 = fmul double %243, 0x3E70000000000000
  br label %320

245:                                              ; preds = %238
  %246 = icmp eq i32 %241, 0
  %247 = select i1 %246, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %320

248:                                              ; preds = %238
  %249 = or disjoint i32 %241, 1024
  %250 = uitofp i32 %249 to double
  %251 = add nsw i32 %240, -25
  %252 = icmp ugt i32 %240, 25
  br i1 %252, label %253, label %270

253:                                              ; preds = %248
  %254 = add nsw i32 %240, -26
  %255 = and i32 %251, 7
  %256 = icmp eq i32 %255, 0
  br i1 %256, label %265, label %257

257:                                              ; preds = %253, %257
  %258 = phi i32 [ %262, %257 ], [ %251, %253 ]
  %259 = phi double [ %261, %257 ], [ %250, %253 ]
  %260 = phi i32 [ %263, %257 ], [ 0, %253 ]
  %261 = fmul double %259, 2.000000e+00
  %262 = add nsw i32 %258, -1
  %263 = add i32 %260, 1
  %264 = icmp eq i32 %263, %255
  br i1 %264, label %265, label %257, !llvm.loop !10018

265:                                              ; preds = %257, %253
  %266 = phi i32 [ %251, %253 ], [ %262, %257 ]
  %267 = phi double [ %250, %253 ], [ %261, %257 ]
  %268 = phi double [ undef, %253 ], [ %261, %257 ]
  %269 = icmp ult i32 %254, 7
  br i1 %269, label %270, label %293

270:                                              ; preds = %265, %293, %248
  %271 = phi double [ %250, %248 ], [ %268, %265 ], [ %303, %293 ]
  %272 = phi i32 [ %251, %248 ], [ 0, %293 ], [ 0, %265 ]
  %273 = icmp slt i32 %272, 0
  br i1 %273, label %274, label %320

274:                                              ; preds = %270
  %275 = tail call i32 @llvm.umin.i32(i32 %240, i32 25)
  %276 = sub nsw i32 1, %275
  %277 = and i32 %276, 7
  %278 = icmp eq i32 %277, 0
  br i1 %278, label %287, label %279

279:                                              ; preds = %274, %279
  %280 = phi i32 [ %284, %279 ], [ %272, %274 ]
  %281 = phi double [ %283, %279 ], [ %271, %274 ]
  %282 = phi i32 [ %285, %279 ], [ 0, %274 ]
  %283 = fmul double %281, 5.000000e-01
  %284 = add nsw i32 %280, 1
  %285 = add i32 %282, 1
  %286 = icmp eq i32 %285, %277
  br i1 %286, label %287, label %279, !llvm.loop !10020

287:                                              ; preds = %279, %274
  %288 = phi double [ undef, %274 ], [ %283, %279 ]
  %289 = phi i32 [ %272, %274 ], [ %284, %279 ]
  %290 = phi double [ %271, %274 ], [ %283, %279 ]
  %291 = add nsw i32 %275, -18
  %292 = icmp ult i32 %291, 7
  br i1 %292, label %320, label %307

293:                                              ; preds = %265, %293
  %294 = phi i32 [ %304, %293 ], [ %266, %265 ]
  %295 = phi double [ %303, %293 ], [ %267, %265 ]
  %296 = fmul double %295, 2.000000e+00
  %297 = fmul double %296, 2.000000e+00
  %298 = fmul double %297, 2.000000e+00
  %299 = fmul double %298, 2.000000e+00
  %300 = fmul double %299, 2.000000e+00
  %301 = fmul double %300, 2.000000e+00
  %302 = fmul double %301, 2.000000e+00
  %303 = fmul double %302, 2.000000e+00
  %304 = add nsw i32 %294, -8
  %305 = add i32 %294, -9
  %306 = icmp ult i32 %305, -2
  br i1 %306, label %293, label %270, !llvm.loop !10021

307:                                              ; preds = %287, %307
  %308 = phi i32 [ %318, %307 ], [ %289, %287 ]
  %309 = phi double [ %317, %307 ], [ %290, %287 ]
  %310 = fmul double %309, 5.000000e-01
  %311 = fmul double %310, 5.000000e-01
  %312 = fmul double %311, 5.000000e-01
  %313 = fmul double %312, 5.000000e-01
  %314 = fmul double %313, 5.000000e-01
  %315 = fmul double %314, 5.000000e-01
  %316 = fmul double %315, 5.000000e-01
  %317 = fmul double %316, 5.000000e-01
  %318 = add nsw i32 %308, 8
  %319 = icmp eq i32 %318, 0
  br i1 %319, label %320, label %307, !llvm.loop !10022

320:                                              ; preds = %287, %307, %270, %245, %242
  %321 = phi double [ %244, %242 ], [ %247, %245 ], [ %271, %270 ], [ %288, %287 ], [ %317, %307 ]
  %322 = icmp ult i32 %235, 32768
  %323 = fneg double %321
  %324 = select i1 %322, double %321, double %323
  br label %364

325:                                              ; preds = %225
  %326 = bitcast i32 %235 to float
  %327 = fpext float %326 to double
  br label %364

328:                                              ; preds = %225, %225
  br label %330

329:                                              ; preds = %225
  br label %330

330:                                              ; preds = %329, %328, %225, %225
  %331 = phi i32 [ 32, %329 ], [ 16, %328 ], [ 8, %225 ], [ 8, %225 ]
  %332 = zext i32 %235 to i64
  switch i64 %192, label %342 [
    i64 21, label %333
    i64 20, label %333
    i64 3, label %333
    i64 5, label %333
  ]

333:                                              ; preds = %330, %330, %330, %330
  %334 = add nsw i32 %331, -1
  %335 = shl nuw i32 1, %334
  %336 = and i32 %335, %235
  %337 = icmp eq i32 %336, 0
  %338 = zext nneg i32 %331 to i64
  %339 = shl nsw i64 -1, %338
  %340 = select i1 %337, i64 0, i64 %339
  %341 = add nsw i64 %340, %332
  br label %342

342:                                              ; preds = %333, %330
  %343 = phi i64 [ %341, %333 ], [ %332, %330 ]
  %344 = load i64, ptr addrspace(1) %208, align 8
  %345 = icmp eq i64 %344, 0
  br i1 %345, label %346, label %348

346:                                              ; preds = %342
  %347 = sitofp i64 %343 to double
  br label %364

348:                                              ; preds = %342
  switch i64 %192, label %349 [
    i64 21, label %350
    i64 20, label %350
    i64 3, label %350
  ]

349:                                              ; preds = %348
  br label %350

350:                                              ; preds = %349, %348, %348, %348
  %351 = phi i32 [ -1, %348 ], [ %210, %349 ], [ -1, %348 ], [ -1, %348 ]
  %352 = add nsw i32 %351, %331
  %353 = zext nneg i32 %352 to i64
  %354 = shl nsw i64 -1, %353
  %355 = xor i64 %354, -1
  %356 = sitofp i64 %343 to double
  %357 = sitofp i64 %355 to double
  %358 = fdiv double %356, %357
  %359 = fcmp oge double %358, -1.000000e+00
  %360 = select i1 %359, double %358, double -1.000000e+00
  br label %364

361:                                              ; preds = %211
  %362 = icmp eq i32 %213, 3
  %363 = select i1 %362, double 1.000000e+00, double 0.000000e+00
  br label %364

364:                                              ; preds = %350, %346, %325, %320, %361
  %365 = phi double [ %363, %361 ], [ %324, %320 ], [ %327, %325 ], [ %360, %350 ], [ %347, %346 ]
  %366 = insertelement <4 x double> %212, double %365, i32 %213
  %367 = add nuw nsw i32 %213, 1
  %368 = icmp eq i32 %367, 4
  br i1 %368, label %727, label %211, !llvm.loop !10023

369:                                              ; preds = %62
  %370 = icmp ult i32 %96, -2
  br i1 %370, label %590, label %371

371:                                              ; preds = %369
  %372 = icmp eq i32 %2, 6
  %373 = zext i1 %372 to i64
  %374 = getelementptr inbounds [4 x i32], ptr %27, i64 0, i64 %373
  %375 = load float, ptr %374, align 4
  %376 = fpext float %375 to double
  %377 = select i1 %372, i64 2, i64 1
  %378 = getelementptr inbounds [4 x i32], ptr %27, i64 0, i64 %377
  %379 = load float, ptr %378, align 4
  %380 = fpext float %379 to double
  %381 = select i1 %372, i64 3, i64 2
  %382 = getelementptr inbounds [4 x i32], ptr %27, i64 0, i64 %381
  %383 = load float, ptr %382, align 4
  %384 = fpext float %383 to double
  %385 = tail call double @llvm.fabs.f64(double %376)
  %386 = tail call double @llvm.fabs.f64(double %380)
  %387 = tail call double @llvm.fabs.f64(double %384)
  %388 = fcmp ult double %385, %386
  %389 = fcmp ult double %385, %387
  %390 = or i1 %388, %389
  br i1 %390, label %394, label %391

391:                                              ; preds = %371
  %392 = fcmp ult float %375, 0.000000e+00
  %393 = zext i1 %392 to i32
  br label %404

394:                                              ; preds = %371
  %395 = fcmp ule double %386, %385
  %396 = fcmp ult double %386, %387
  %397 = or i1 %395, %396
  br i1 %397, label %401, label %398

398:                                              ; preds = %394
  %399 = fcmp oge float %379, 0.000000e+00
  %400 = select i1 %399, i32 2, i32 3
  br label %404

401:                                              ; preds = %394
  %402 = fcmp oge float %383, 0.000000e+00
  %403 = select i1 %402, i32 4, i32 5
  br label %404

404:                                              ; preds = %391, %398, %401
  %405 = phi i32 [ %393, %391 ], [ %400, %398 ], [ %403, %401 ]
  %406 = icmp ult i32 %405, 2
  %407 = icmp ult i32 %405, 4
  %408 = select i1 %407, double %386, double %387
  %409 = select i1 %406, double %385, double %408
  %410 = icmp eq i32 %3, 1
  br i1 %410, label %411, label %413

411:                                              ; preds = %404
  %412 = fpext float %10 to double
  br label %507

413:                                              ; preds = %404
  %414 = fmul double %409, 2.000000e+00
  %415 = getelementptr i64, ptr addrspace(1) %0, i64 9
  %416 = load i64, ptr addrspace(1) %415, align 8
  %417 = tail call noundef i64 @llvm.smax.i64(i64 %416, i64 1)
  %418 = sitofp i64 %417 to double
  %419 = fdiv double %418, %414
  %420 = fmul double %419, %83
  %421 = fmul double %419, %88
  %422 = fmul double %420, %420
  %423 = fmul double %421, %421
  %424 = getelementptr i64, ptr addrspace(1) %0, i64 10
  %425 = load i64, ptr addrspace(1) %424, align 8
  %426 = tail call noundef i64 @llvm.smax.i64(i64 %425, i64 1)
  %427 = sitofp i64 %426 to double
  %428 = fdiv double %427, %414
  %429 = fmul double %428, %85
  %430 = fmul double %428, %90
  %431 = fmul double %429, %429
  %432 = fadd double %422, %431
  %433 = fmul double %430, %430
  %434 = fadd double %423, %433
  %435 = getelementptr i64, ptr addrspace(1) %0, i64 11
  %436 = load i64, ptr addrspace(1) %435, align 8
  %437 = tail call noundef i64 @llvm.smax.i64(i64 %436, i64 1)
  %438 = sitofp i64 %437 to double
  %439 = fdiv double %438, %414
  %440 = fmul double %439, %87
  %441 = fmul double %439, %92
  %442 = fmul double %440, %440
  %443 = fadd double %432, %442
  %444 = fmul double %441, %441
  %445 = fadd double %434, %444
  %446 = fcmp oge double %443, %445
  %447 = select i1 %446, double %443, double %445
  %448 = fcmp ogt double %447, 0.000000e+00
  br i1 %448, label %449, label %507

449:                                              ; preds = %413
  %450 = bitcast double %447 to i64
  %451 = lshr i64 %450, 52
  %452 = trunc i64 %451 to i32
  %453 = and i32 %452, 2047
  %454 = icmp eq i32 %453, 0
  %455 = fmul double %447, 0x4350000000000000
  %456 = select i1 %454, double %455, double %447
  %457 = bitcast double %456 to i64
  %458 = and i64 %457, 4503599627370495
  %459 = or disjoint i64 %458, 4607182418800017408
  %460 = bitcast i64 %459 to double
  %461 = fcmp ogt double %460, 0x3FF6A09E667F3BCD
  %462 = fmul double %460, 5.000000e-01
  %463 = select i1 %461, double %462, double %460
  %464 = fadd double %463, -1.000000e+00
  %465 = fadd double %463, 1.000000e+00
  %466 = fdiv double %464, %465
  %467 = fmul double %466, %466
  br label %468

468:                                              ; preds = %468, %449
  %469 = phi i32 [ 1, %449 ], [ %491, %468 ]
  %470 = phi double [ 0.000000e+00, %449 ], [ %489, %468 ]
  %471 = phi double [ %466, %449 ], [ %490, %468 ]
  %472 = sitofp i32 %469 to double
  %473 = fdiv double %471, %472
  %474 = fadd double %470, %473
  %475 = fmul double %467, %471
  %476 = add nuw nsw i32 %469, 2
  %477 = sitofp i32 %476 to double
  %478 = fdiv double %475, %477
  %479 = fadd double %474, %478
  %480 = fmul double %467, %475
  %481 = add nuw nsw i32 %469, 4
  %482 = sitofp i32 %481 to double
  %483 = fdiv double %480, %482
  %484 = fadd double %479, %483
  %485 = fmul double %467, %480
  %486 = add nuw nsw i32 %469, 6
  %487 = sitofp i32 %486 to double
  %488 = fdiv double %485, %487
  %489 = fadd double %484, %488
  %490 = fmul double %467, %485
  %491 = add nuw nsw i32 %469, 8
  %492 = icmp ult i32 %469, 32
  br i1 %492, label %468, label %493, !llvm.loop !10024

493:                                              ; preds = %468
  %494 = bitcast double %455 to i64
  %495 = lshr i64 %494, 52
  %496 = trunc i64 %495 to i32
  %497 = and i32 %496, 2047
  %498 = add nsw i32 %497, -54
  %499 = select i1 %454, i32 %498, i32 %453
  %500 = select i1 %461, i32 -1022, i32 -1023
  %501 = add nsw i32 %500, %499
  %502 = sitofp i32 %501 to double
  %503 = fmul double %489, 2.000000e+00
  %504 = fdiv double %503, 0x3FE62E42FEFA39EF
  %505 = fadd double %504, %502
  %506 = fmul double %505, 5.000000e-01
  br label %507

507:                                              ; preds = %493, %413, %411
  %508 = phi double [ %412, %411 ], [ %506, %493 ], [ -1.000000e+03, %413 ]
  %509 = icmp ne i32 %3, 0
  %510 = zext i1 %509 to i32
  br i1 %390, label %514, label %511

511:                                              ; preds = %507
  %512 = fcmp ult float %375, 0.000000e+00
  %513 = zext i1 %512 to i32
  br label %524

514:                                              ; preds = %507
  %515 = fcmp ule double %386, %385
  %516 = fcmp ult double %386, %387
  %517 = or i1 %515, %516
  br i1 %517, label %521, label %518

518:                                              ; preds = %514
  %519 = fcmp oge float %379, 0.000000e+00
  %520 = select i1 %519, i32 2, i32 3
  br label %524

521:                                              ; preds = %514
  %522 = fcmp oge float %383, 0.000000e+00
  %523 = select i1 %522, i32 4, i32 5
  br label %524

524:                                              ; preds = %521, %518, %511
  %525 = phi i32 [ %513, %511 ], [ %520, %518 ], [ %523, %521 ]
  switch i32 %525, label %530 [
    i32 0, label %526
    i32 1, label %531
    i32 5, label %528
  ]

526:                                              ; preds = %524
  %527 = fneg double %384
  br label %531

528:                                              ; preds = %524
  %529 = fneg double %376
  br label %531

530:                                              ; preds = %524
  br label %531

531:                                              ; preds = %524, %526, %528, %530
  %532 = phi double [ %376, %530 ], [ %529, %528 ], [ %527, %526 ], [ %384, %524 ]
  %533 = fcmp ogt double %409, 0.000000e+00
  %534 = fdiv double %532, %409
  %535 = fadd double %534, 1.000000e+00
  %536 = fmul double %535, 5.000000e-01
  %537 = select i1 %533, double %536, double 5.000000e-01
  br i1 %390, label %541, label %538

538:                                              ; preds = %531
  %539 = fcmp ult float %375, 0.000000e+00
  %540 = zext i1 %539 to i32
  br label %551

541:                                              ; preds = %531
  %542 = fcmp ule double %386, %385
  %543 = fcmp ult double %386, %387
  %544 = or i1 %542, %543
  br i1 %544, label %548, label %545

545:                                              ; preds = %541
  %546 = fcmp oge float %379, 0.000000e+00
  %547 = select i1 %546, i32 2, i32 3
  br label %551

548:                                              ; preds = %541
  %549 = fcmp oge float %383, 0.000000e+00
  %550 = select i1 %549, i32 4, i32 5
  br label %551

551:                                              ; preds = %548, %545, %538
  %552 = phi i32 [ %540, %538 ], [ %547, %545 ], [ %550, %548 ]
  switch i32 %552, label %555 [
    i32 2, label %557
    i32 3, label %553
  ]

553:                                              ; preds = %551
  %554 = fneg double %384
  br label %557

555:                                              ; preds = %551
  %556 = fneg double %380
  br label %557

557:                                              ; preds = %551, %553, %555
  %558 = phi double [ %556, %555 ], [ %554, %553 ], [ %384, %551 ]
  %559 = fdiv double %558, %409
  %560 = fadd double %559, 1.000000e+00
  %561 = fmul double %560, 5.000000e-01
  %562 = select i1 %533, double %561, double 5.000000e-01
  %563 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 8
  %564 = load i64, ptr addrspace(1) %563, align 8
  br i1 %390, label %568, label %565

565:                                              ; preds = %557
  %566 = fcmp ult float %375, 0.000000e+00
  %567 = zext i1 %566 to i64
  br label %578

568:                                              ; preds = %557
  %569 = fcmp ule double %386, %385
  %570 = fcmp ult double %386, %387
  %571 = or i1 %569, %570
  br i1 %571, label %575, label %572

572:                                              ; preds = %568
  %573 = fcmp oge float %379, 0.000000e+00
  %574 = select i1 %573, i64 2, i64 3
  br label %578

575:                                              ; preds = %568
  %576 = fcmp oge float %383, 0.000000e+00
  %577 = select i1 %576, i64 4, i64 5
  br label %578

578:                                              ; preds = %565, %572, %575
  %579 = phi i64 [ %567, %565 ], [ %574, %572 ], [ %577, %575 ]
  %580 = select i1 %372, i32 %6, i32 0
  %581 = zext i32 %580 to i64
  %582 = tail call noundef i64 @llvm.smax.i64(i64 %564, i64 1)
  %583 = udiv i64 %582, 6
  %584 = tail call i64 @llvm.smax.i64(i64 %583, i64 1)
  %585 = add nsw i64 %584, -1
  %586 = tail call noundef i64 @llvm.smin.i64(i64 %581, i64 %585)
  %587 = mul nuw nsw i64 %586, 6
  %588 = add nuw nsw i64 %579, %587
  %589 = call fastcc <4 x double> @__zluda_agx_tex_mip_sample(ptr addrspace(1) noundef %0, ptr noundef nonnull %23, i32 noundef %510, double noundef %508, ptr noundef nonnull %24, i32 noundef %70, i32 noundef 1, i32 noundef 1, double noundef %537, double noundef %562, double noundef 0.000000e+00, i64 noundef %588, i32 noundef 2)
  br label %727

590:                                              ; preds = %369
  %591 = add i32 %2, -3
  %592 = icmp ult i32 %591, 2
  %593 = zext i1 %592 to i64
  switch i32 %2, label %594 [
    i32 3, label %597
    i32 0, label %597
  ]

594:                                              ; preds = %590
  %595 = icmp eq i32 %2, 2
  %596 = select i1 %595, i32 3, i32 2
  br label %597

597:                                              ; preds = %590, %590, %594
  %598 = phi i32 [ %596, %594 ], [ 1, %590 ], [ 1, %590 ]
  %599 = icmp eq i32 %3, 1
  br i1 %599, label %600, label %602

600:                                              ; preds = %597
  %601 = fpext float %10 to double
  br label %691

602:                                              ; preds = %597
  %603 = getelementptr i64, ptr addrspace(1) %0, i64 9
  %604 = icmp eq i32 %64, 1
  br label %609

605:                                              ; preds = %619
  %606 = fcmp oge double %628, %630
  %607 = select i1 %606, double %628, double %630
  %608 = fcmp ogt double %607, 0.000000e+00
  br i1 %608, label %633, label %691

609:                                              ; preds = %619, %602
  %610 = phi double [ 0.000000e+00, %602 ], [ %628, %619 ]
  %611 = phi double [ 0.000000e+00, %602 ], [ %630, %619 ]
  %612 = phi i32 [ 0, %602 ], [ %631, %619 ]
  %613 = zext nneg i32 %612 to i64
  br i1 %604, label %614, label %619

614:                                              ; preds = %609
  %615 = getelementptr i64, ptr addrspace(1) %603, i64 %613
  %616 = load i64, ptr addrspace(1) %615, align 8
  %617 = tail call noundef i64 @llvm.smax.i64(i64 %616, i64 1)
  %618 = sitofp i64 %617 to double
  br label %619

619:                                              ; preds = %614, %609
  %620 = phi double [ %618, %614 ], [ 1.000000e+00, %609 ]
  %621 = getelementptr inbounds double, ptr %25, i64 %613
  %622 = load double, ptr %621, align 8
  %623 = fmul double %620, %622
  %624 = getelementptr inbounds double, ptr %26, i64 %613
  %625 = load double, ptr %624, align 8
  %626 = fmul double %620, %625
  %627 = fmul double %623, %623
  %628 = fadd double %610, %627
  %629 = fmul double %626, %626
  %630 = fadd double %611, %629
  %631 = add nuw nsw i32 %612, 1
  %632 = icmp eq i32 %631, %598
  br i1 %632, label %605, label %609, !llvm.loop !10025

633:                                              ; preds = %605
  %634 = bitcast double %607 to i64
  %635 = lshr i64 %634, 52
  %636 = trunc i64 %635 to i32
  %637 = and i32 %636, 2047
  %638 = icmp eq i32 %637, 0
  %639 = fmul double %607, 0x4350000000000000
  %640 = select i1 %638, double %639, double %607
  %641 = bitcast double %640 to i64
  %642 = and i64 %641, 4503599627370495
  %643 = or disjoint i64 %642, 4607182418800017408
  %644 = bitcast i64 %643 to double
  %645 = fcmp ogt double %644, 0x3FF6A09E667F3BCD
  %646 = fmul double %644, 5.000000e-01
  %647 = select i1 %645, double %646, double %644
  %648 = fadd double %647, -1.000000e+00
  %649 = fadd double %647, 1.000000e+00
  %650 = fdiv double %648, %649
  %651 = fmul double %650, %650
  br label %652

652:                                              ; preds = %652, %633
  %653 = phi i32 [ 1, %633 ], [ %675, %652 ]
  %654 = phi double [ 0.000000e+00, %633 ], [ %673, %652 ]
  %655 = phi double [ %650, %633 ], [ %674, %652 ]
  %656 = sitofp i32 %653 to double
  %657 = fdiv double %655, %656
  %658 = fadd double %654, %657
  %659 = fmul double %651, %655
  %660 = add nuw nsw i32 %653, 2
  %661 = sitofp i32 %660 to double
  %662 = fdiv double %659, %661
  %663 = fadd double %658, %662
  %664 = fmul double %651, %659
  %665 = add nuw nsw i32 %653, 4
  %666 = sitofp i32 %665 to double
  %667 = fdiv double %664, %666
  %668 = fadd double %663, %667
  %669 = fmul double %651, %664
  %670 = add nuw nsw i32 %653, 6
  %671 = sitofp i32 %670 to double
  %672 = fdiv double %669, %671
  %673 = fadd double %668, %672
  %674 = fmul double %651, %669
  %675 = add nuw nsw i32 %653, 8
  %676 = icmp ult i32 %653, 32
  br i1 %676, label %652, label %677, !llvm.loop !10024

677:                                              ; preds = %652
  %678 = bitcast double %639 to i64
  %679 = lshr i64 %678, 52
  %680 = trunc i64 %679 to i32
  %681 = and i32 %680, 2047
  %682 = add nsw i32 %681, -54
  %683 = select i1 %638, i32 %682, i32 %637
  %684 = select i1 %645, i32 -1022, i32 -1023
  %685 = add nsw i32 %684, %683
  %686 = sitofp i32 %685 to double
  %687 = fmul double %673, 2.000000e+00
  %688 = fdiv double %687, 0x3FE62E42FEFA39EF
  %689 = fadd double %688, %686
  %690 = fmul double %689, 5.000000e-01
  br label %691

691:                                              ; preds = %677, %605, %600
  %692 = phi double [ %601, %600 ], [ %690, %677 ], [ -1.000000e+03, %605 ]
  %693 = icmp ne i32 %3, 0
  %694 = zext i1 %693 to i32
  %695 = icmp eq i32 %4, 0
  %696 = select i1 %695, i32 0, i32 %70
  %697 = getelementptr inbounds [4 x i32], ptr %27, i64 0, i64 %593
  %698 = load i32, ptr %697, align 4
  %699 = bitcast i32 %698 to float
  %700 = fpext float %699 to double
  %701 = sitofp i32 %698 to double
  %702 = select i1 %695, double %701, double %700
  %703 = select i1 %592, i64 2, i64 1
  %704 = getelementptr inbounds [4 x i32], ptr %27, i64 0, i64 %703
  %705 = load i32, ptr %704, align 4
  %706 = bitcast i32 %705 to float
  %707 = fpext float %706 to double
  %708 = sitofp i32 %705 to double
  %709 = select i1 %695, double %708, double %707
  %710 = or disjoint i64 %593, 2
  %711 = getelementptr inbounds [4 x i32], ptr %27, i64 0, i64 %710
  %712 = load i32, ptr %711, align 4
  %713 = bitcast i32 %712 to float
  %714 = fpext float %713 to double
  %715 = sitofp i32 %712 to double
  %716 = select i1 %695, double %715, double %714
  switch i32 %2, label %724 [
    i32 6, label %717
    i32 4, label %717
    i32 3, label %717
  ]

717:                                              ; preds = %691, %691, %691
  %718 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 8
  %719 = zext i32 %6 to i64
  %720 = load i64, ptr addrspace(1) %718, align 8
  %721 = tail call noundef i64 @llvm.smax.i64(i64 %720, i64 1)
  %722 = add nsw i64 %721, -1
  %723 = tail call noundef i64 @llvm.smin.i64(i64 %719, i64 %722)
  br label %724

724:                                              ; preds = %691, %717
  %725 = phi i64 [ %723, %717 ], [ 0, %691 ]
  %726 = call fastcc <4 x double> @__zluda_agx_tex_mip_sample(ptr addrspace(1) noundef %0, ptr noundef nonnull %23, i32 noundef %694, double noundef %692, ptr noundef nonnull %24, i32 noundef %696, i32 noundef %71, i32 noundef %64, double noundef %702, double noundef %709, double noundef %716, i64 noundef %725, i32 noundef %598)
  br label %727

727:                                              ; preds = %364, %183, %578, %724
  %728 = phi <4 x double> [ %589, %578 ], [ %726, %724 ], [ zeroinitializer, %183 ], [ %366, %364 ]
  %729 = extractelement <4 x double> %728, i64 0
  %730 = icmp eq i32 %5, 0
  br i1 %730, label %731, label %734

731:                                              ; preds = %727
  %732 = fptrunc double %729 to float
  %733 = bitcast float %732 to i32
  br label %739

734:                                              ; preds = %727
  %735 = fcmp uno double %729, 0.000000e+00
  br i1 %735, label %739, label %736

736:                                              ; preds = %734
  %737 = fptosi double %729 to i64
  %738 = trunc i64 %737 to i32
  br label %739

739:                                              ; preds = %731, %734, %736
  %740 = phi i32 [ %733, %731 ], [ %738, %736 ], [ 0, %734 ]
  %741 = extractelement <4 x double> %728, i64 1
  br i1 %730, label %742, label %745

742:                                              ; preds = %739
  %743 = fptrunc double %741 to float
  %744 = bitcast float %743 to i32
  br label %750

745:                                              ; preds = %739
  %746 = fcmp uno double %741, 0.000000e+00
  br i1 %746, label %750, label %747

747:                                              ; preds = %745
  %748 = fptosi double %741 to i64
  %749 = trunc i64 %748 to i32
  br label %750

750:                                              ; preds = %742, %745, %747
  %751 = phi i32 [ %744, %742 ], [ %749, %747 ], [ 0, %745 ]
  %752 = extractelement <4 x double> %728, i64 2
  br i1 %730, label %753, label %756

753:                                              ; preds = %750
  %754 = fptrunc double %752 to float
  %755 = bitcast float %754 to i32
  br label %761

756:                                              ; preds = %750
  %757 = fcmp uno double %752, 0.000000e+00
  br i1 %757, label %761, label %758

758:                                              ; preds = %756
  %759 = fptosi double %752 to i64
  %760 = trunc i64 %759 to i32
  br label %761

761:                                              ; preds = %753, %756, %758
  %762 = phi i32 [ %755, %753 ], [ %760, %758 ], [ 0, %756 ]
  %763 = extractelement <4 x double> %728, i64 3
  br i1 %730, label %764, label %767

764:                                              ; preds = %761
  %765 = fptrunc double %763 to float
  %766 = bitcast float %765 to i32
  br label %772

767:                                              ; preds = %761
  %768 = fcmp uno double %763, 0.000000e+00
  br i1 %768, label %772, label %769

769:                                              ; preds = %767
  %770 = fptosi double %763 to i64
  %771 = trunc i64 %770 to i32
  br label %772

772:                                              ; preds = %764, %767, %769
  %773 = phi i32 [ %766, %764 ], [ %771, %769 ], [ 0, %767 ]
  %774 = insertelement <4 x i32> poison, i32 %740, i64 0
  %775 = insertelement <4 x i32> %774, i32 %751, i64 1
  %776 = insertelement <4 x i32> %775, i32 %762, i64 2
  %777 = insertelement <4 x i32> %776, i32 %773, i64 3
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %27)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %26)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %25)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %24)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %23)
  ret <4 x i32> %777
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture)

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture)

; Function Attrs: inlinehint nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define internal fastcc <4 x double> @__zluda_agx_tex_mip_sample(ptr addrspace(1) nocapture noundef readonly %0, ptr nocapture noundef readonly %1, i32 noundef %2, double noundef %3, ptr nocapture noundef readonly %4, i32 noundef %5, i32 noundef %6, i32 noundef %7, double noundef %8, double noundef %9, double noundef %10, i64 noundef %11, i32 noundef %12) unnamed_addr #3 {
  %14 = icmp eq i32 %2, 0
  br i1 %14, label %15, label %928

15:                                               ; preds = %13
  %16 = icmp eq i32 %5, 1
  %17 = load i32, ptr %4, align 4
  %18 = getelementptr i64, ptr addrspace(1) %0, i64 9
  %19 = load i64, ptr addrspace(1) %18, align 8
  %20 = tail call noundef i64 @llvm.smax.i64(i64 %19, i64 1)
  %21 = zext nneg i32 %17 to i64
  %22 = lshr i64 %20, %21
  %23 = tail call noundef i64 @llvm.smax.i64(i64 %22, i64 1)
  br i1 %16, label %376, label %24

24:                                               ; preds = %15
  %25 = getelementptr i64, ptr addrspace(1) %0, i64 1
  %26 = getelementptr i64, ptr addrspace(1) %0, i64 10
  %27 = load i64, ptr addrspace(1) %26, align 8
  %28 = tail call noundef i64 @llvm.smax.i64(i64 %27, i64 1)
  %29 = lshr i64 %28, %21
  %30 = tail call noundef i64 @llvm.smax.i64(i64 %29, i64 1)
  %31 = getelementptr i64, ptr addrspace(1) %0, i64 2
  %32 = getelementptr i64, ptr addrspace(1) %0, i64 11
  %33 = load i64, ptr addrspace(1) %32, align 8
  %34 = tail call noundef i64 @llvm.smax.i64(i64 %33, i64 1)
  %35 = lshr i64 %34, %21
  %36 = tail call noundef i64 @llvm.smax.i64(i64 %35, i64 1)
  %37 = icmp eq i32 %7, 1
  %38 = sitofp i64 %23 to double
  %39 = select i1 %37, double %38, double 1.000000e+00
  %40 = fmul double %39, %8
  %41 = tail call double @llvm.floor.f64(double %40)
  %42 = fptosi double %41 to i64
  %43 = getelementptr inbounds %struct.q, ptr %4, i64 0, i32 1
  %44 = load i32, ptr %43, align 4
  %45 = sext i32 %44 to i64
  %46 = add nsw i64 %45, %42
  switch i32 %6, label %64 [
    i32 0, label %47
    i32 2, label %51
    i32 3, label %55
  ]

47:                                               ; preds = %24
  %48 = srem i64 %46, %23
  %49 = add nsw i64 %48, %23
  %50 = srem i64 %49, %23
  br label %68

51:                                               ; preds = %24
  %52 = icmp slt i64 %46, %23
  %53 = tail call i64 @llvm.smax.i64(i64 %46, i64 -1)
  %54 = select i1 %52, i64 %53, i64 -1
  br label %68

55:                                               ; preds = %24
  %56 = shl nuw nsw i64 %23, 1
  %57 = srem i64 %46, %56
  %58 = add nsw i64 %57, %56
  %59 = srem i64 %58, %56
  %60 = icmp slt i64 %59, %23
  %61 = xor i64 %59, -1
  %62 = add i64 %56, %61
  %63 = select i1 %60, i64 %59, i64 %62
  br label %68

64:                                               ; preds = %24
  %65 = tail call noundef i64 @llvm.smax.i64(i64 %46, i64 0)
  %66 = add nsw i64 %23, -1
  %67 = tail call noundef i64 @llvm.smin.i64(i64 %65, i64 %66)
  br label %68

68:                                               ; preds = %64, %55, %51, %47
  %69 = phi i64 [ %67, %64 ], [ %63, %55 ], [ %54, %51 ], [ %50, %47 ]
  %70 = icmp sgt i32 %12, 1
  br i1 %70, label %71, label %102

71:                                               ; preds = %68
  %72 = sitofp i64 %30 to double
  %73 = select i1 %37, double %72, double 1.000000e+00
  %74 = fmul double %73, %9
  %75 = tail call double @llvm.floor.f64(double %74)
  %76 = fptosi double %75 to i64
  %77 = getelementptr inbounds %struct.q, ptr %4, i64 0, i32 1, i64 1
  %78 = load i32, ptr %77, align 4
  %79 = sext i32 %78 to i64
  %80 = add nsw i64 %79, %76
  switch i32 %6, label %98 [
    i32 0, label %81
    i32 2, label %85
    i32 3, label %89
  ]

81:                                               ; preds = %71
  %82 = srem i64 %80, %30
  %83 = add nsw i64 %82, %30
  %84 = srem i64 %83, %30
  br label %102

85:                                               ; preds = %71
  %86 = icmp slt i64 %80, %30
  %87 = tail call i64 @llvm.smax.i64(i64 %80, i64 -1)
  %88 = select i1 %86, i64 %87, i64 -1
  br label %102

89:                                               ; preds = %71
  %90 = shl nuw nsw i64 %30, 1
  %91 = srem i64 %80, %90
  %92 = add nsw i64 %91, %90
  %93 = srem i64 %92, %90
  %94 = icmp slt i64 %93, %30
  %95 = xor i64 %93, -1
  %96 = add i64 %90, %95
  %97 = select i1 %94, i64 %93, i64 %96
  br label %102

98:                                               ; preds = %71
  %99 = tail call noundef i64 @llvm.smax.i64(i64 %80, i64 0)
  %100 = add nsw i64 %30, -1
  %101 = tail call noundef i64 @llvm.smin.i64(i64 %99, i64 %100)
  br label %102

102:                                              ; preds = %98, %89, %85, %81, %68
  %103 = phi i64 [ 0, %68 ], [ %101, %98 ], [ %97, %89 ], [ %88, %85 ], [ %84, %81 ]
  %104 = icmp sgt i32 %12, 2
  br i1 %104, label %105, label %136

105:                                              ; preds = %102
  %106 = sitofp i64 %36 to double
  %107 = select i1 %37, double %106, double 1.000000e+00
  %108 = fmul double %107, %10
  %109 = tail call double @llvm.floor.f64(double %108)
  %110 = fptosi double %109 to i64
  %111 = getelementptr inbounds %struct.q, ptr %4, i64 0, i32 1, i64 2
  %112 = load i32, ptr %111, align 4
  %113 = sext i32 %112 to i64
  %114 = add nsw i64 %113, %110
  switch i32 %6, label %132 [
    i32 0, label %115
    i32 2, label %119
    i32 3, label %123
  ]

115:                                              ; preds = %105
  %116 = srem i64 %114, %36
  %117 = add nsw i64 %116, %36
  %118 = srem i64 %117, %36
  br label %136

119:                                              ; preds = %105
  %120 = icmp slt i64 %114, %36
  %121 = tail call i64 @llvm.smax.i64(i64 %114, i64 -1)
  %122 = select i1 %120, i64 %121, i64 -1
  br label %136

123:                                              ; preds = %105
  %124 = shl nuw nsw i64 %36, 1
  %125 = srem i64 %114, %124
  %126 = add nsw i64 %125, %124
  %127 = srem i64 %126, %124
  %128 = icmp slt i64 %127, %36
  %129 = xor i64 %127, -1
  %130 = add i64 %124, %129
  %131 = select i1 %128, i64 %127, i64 %130
  br label %136

132:                                              ; preds = %105
  %133 = tail call noundef i64 @llvm.smax.i64(i64 %114, i64 0)
  %134 = add nsw i64 %36, -1
  %135 = tail call noundef i64 @llvm.smin.i64(i64 %133, i64 %134)
  br label %136

136:                                              ; preds = %132, %123, %119, %115, %102
  %137 = phi i64 [ 0, %102 ], [ %135, %132 ], [ %131, %123 ], [ %122, %119 ], [ %118, %115 ]
  %138 = or i64 %103, %69
  %139 = or i64 %138, %137
  %140 = icmp sgt i64 %139, -1
  br i1 %140, label %141, label %363

141:                                              ; preds = %136
  %142 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %143 = load i64, ptr addrspace(1) %142, align 8
  %144 = load i64, ptr addrspace(1) %31, align 8
  switch i64 %144, label %146 [
    i64 0, label %147
    i64 20, label %147
    i64 1, label %145
    i64 21, label %145
    i64 6, label %145
  ]

145:                                              ; preds = %141, %141, %141
  br label %147

146:                                              ; preds = %141
  br label %147

147:                                              ; preds = %146, %145, %141, %141
  %148 = phi i32 [ 4, %146 ], [ 2, %145 ], [ 1, %141 ], [ 1, %141 ]
  %149 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 14
  %150 = load i64, ptr addrspace(1) %149, align 8
  %151 = icmp sgt i32 %17, 0
  br i1 %151, label %152, label %172

152:                                              ; preds = %147
  %153 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 8
  %154 = load i64, ptr addrspace(1) %153, align 8
  %155 = tail call noundef i64 @llvm.smax.i64(i64 %154, i64 1)
  br label %156

156:                                              ; preds = %156, %152
  %157 = phi i32 [ 0, %152 ], [ %170, %156 ]
  %158 = phi i64 [ 0, %152 ], [ %169, %156 ]
  %159 = zext nneg i32 %157 to i64
  %160 = lshr i64 %20, %159
  %161 = tail call noundef i64 @llvm.smax.i64(i64 %160, i64 1)
  %162 = mul nsw i64 %161, %155
  %163 = lshr i64 %28, %159
  %164 = tail call noundef i64 @llvm.smax.i64(i64 %163, i64 1)
  %165 = mul nsw i64 %162, %164
  %166 = lshr i64 %34, %159
  %167 = tail call noundef i64 @llvm.smax.i64(i64 %166, i64 1)
  %168 = mul nsw i64 %165, %167
  %169 = add nuw nsw i64 %168, %158
  %170 = add nuw nsw i32 %157, 1
  %171 = icmp eq i32 %170, %17
  br i1 %171, label %172, label %156, !llvm.loop !10026

172:                                              ; preds = %156, %147
  %173 = phi i64 [ 0, %147 ], [ %169, %156 ]
  %174 = icmp eq i64 %150, 0
  br i1 %174, label %187, label %175

175:                                              ; preds = %172
  %176 = load i64, ptr addrspace(1) %25, align 8
  %177 = mul nsw i64 %36, %11
  %178 = add nsw i64 %137, %177
  %179 = mul nsw i64 %178, %30
  %180 = add nsw i64 %179, %103
  %181 = mul nsw i64 %150, %180
  %182 = mul nsw i64 %143, %69
  %183 = zext nneg i32 %148 to i64
  %184 = mul nsw i64 %182, %183
  %185 = add nsw i64 %181, %184
  %186 = add i64 %185, %176
  br label %200

187:                                              ; preds = %172
  %188 = mul nsw i64 %36, %11
  %189 = add nsw i64 %137, %188
  %190 = mul nsw i64 %189, %30
  %191 = add nsw i64 %190, %103
  %192 = mul nsw i64 %191, %23
  %193 = add i64 %192, %69
  %194 = add i64 %193, %173
  %195 = load i64, ptr addrspace(1) %25, align 8
  %196 = zext nneg i32 %148 to i64
  %197 = mul i64 %143, %196
  %198 = mul i64 %197, %194
  %199 = add i64 %195, %198
  br label %200

200:                                              ; preds = %187, %175
  %201 = phi i64 [ %186, %175 ], [ %199, %187 ]
  %202 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 7
  %203 = icmp eq i64 %144, 5
  %204 = sext i1 %203 to i32
  br label %205

205:                                              ; preds = %358, %200
  %206 = phi i32 [ 0, %200 ], [ %361, %358 ]
  %207 = phi <4 x double> [ zeroinitializer, %200 ], [ %360, %358 ]
  %208 = zext nneg i32 %206 to i64
  %209 = icmp sgt i64 %143, %208
  br i1 %209, label %210, label %355

210:                                              ; preds = %205
  %211 = mul nuw nsw i32 %206, %148
  %212 = zext nneg i32 %211 to i64
  %213 = add i64 %201, %212
  switch i64 %144, label %215 [
    i64 0, label %216
    i64 20, label %216
    i64 1, label %214
    i64 21, label %214
    i64 6, label %214
  ]

214:                                              ; preds = %210, %210, %210
  br label %216

215:                                              ; preds = %210
  br label %216

216:                                              ; preds = %215, %214, %210, %210
  %217 = phi i32 [ 4, %215 ], [ 2, %214 ], [ 1, %210 ], [ 1, %210 ]
  %218 = inttoptr i64 %213 to ptr addrspace(1)
  br label %220

219:                                              ; preds = %220
  switch i64 %144, label %323 [
    i64 6, label %232
    i64 7, label %319
    i64 0, label %324
    i64 20, label %324
    i64 1, label %322
    i64 21, label %322
  ]

220:                                              ; preds = %220, %216
  %221 = phi i32 [ 0, %216 ], [ %229, %220 ]
  %222 = phi i32 [ 0, %216 ], [ %230, %220 ]
  %223 = zext nneg i32 %222 to i64
  %224 = getelementptr inbounds i8, ptr addrspace(1) %218, i64 %223
  %225 = load i8, ptr addrspace(1) %224, align 1
  %226 = zext i8 %225 to i32
  %227 = shl nsw i32 %222, 3
  %228 = shl i32 %226, %227
  %229 = or i32 %228, %221
  %230 = add nuw nsw i32 %222, 1
  %231 = icmp eq i32 %230, %217
  br i1 %231, label %219, label %220, !llvm.loop !10016

232:                                              ; preds = %219
  %233 = lshr i32 %229, 10
  %234 = and i32 %233, 31
  %235 = and i32 %229, 1023
  switch i32 %234, label %242 [
    i32 0, label %236
    i32 31, label %239
  ]

236:                                              ; preds = %232
  %237 = uitofp i32 %235 to double
  %238 = fmul double %237, 0x3E70000000000000
  br label %314

239:                                              ; preds = %232
  %240 = icmp eq i32 %235, 0
  %241 = select i1 %240, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %314

242:                                              ; preds = %232
  %243 = or disjoint i32 %235, 1024
  %244 = uitofp i32 %243 to double
  %245 = add nsw i32 %234, -25
  %246 = icmp ugt i32 %234, 25
  br i1 %246, label %247, label %264

247:                                              ; preds = %242
  %248 = add nsw i32 %234, -26
  %249 = and i32 %245, 7
  %250 = icmp eq i32 %249, 0
  br i1 %250, label %259, label %251

251:                                              ; preds = %247, %251
  %252 = phi i32 [ %256, %251 ], [ %245, %247 ]
  %253 = phi double [ %255, %251 ], [ %244, %247 ]
  %254 = phi i32 [ %257, %251 ], [ 0, %247 ]
  %255 = fmul double %253, 2.000000e+00
  %256 = add nsw i32 %252, -1
  %257 = add i32 %254, 1
  %258 = icmp eq i32 %257, %249
  br i1 %258, label %259, label %251, !llvm.loop !10027

259:                                              ; preds = %251, %247
  %260 = phi i32 [ %245, %247 ], [ %256, %251 ]
  %261 = phi double [ %244, %247 ], [ %255, %251 ]
  %262 = phi double [ undef, %247 ], [ %255, %251 ]
  %263 = icmp ult i32 %248, 7
  br i1 %263, label %264, label %287

264:                                              ; preds = %259, %287, %242
  %265 = phi double [ %244, %242 ], [ %262, %259 ], [ %297, %287 ]
  %266 = phi i32 [ %245, %242 ], [ 0, %287 ], [ 0, %259 ]
  %267 = icmp slt i32 %266, 0
  br i1 %267, label %268, label %314

268:                                              ; preds = %264
  %269 = tail call i32 @llvm.umin.i32(i32 %234, i32 25)
  %270 = sub nsw i32 1, %269
  %271 = and i32 %270, 7
  %272 = icmp eq i32 %271, 0
  br i1 %272, label %281, label %273

273:                                              ; preds = %268, %273
  %274 = phi i32 [ %278, %273 ], [ %266, %268 ]
  %275 = phi double [ %277, %273 ], [ %265, %268 ]
  %276 = phi i32 [ %279, %273 ], [ 0, %268 ]
  %277 = fmul double %275, 5.000000e-01
  %278 = add nsw i32 %274, 1
  %279 = add i32 %276, 1
  %280 = icmp eq i32 %279, %271
  br i1 %280, label %281, label %273, !llvm.loop !10028

281:                                              ; preds = %273, %268
  %282 = phi double [ undef, %268 ], [ %277, %273 ]
  %283 = phi i32 [ %266, %268 ], [ %278, %273 ]
  %284 = phi double [ %265, %268 ], [ %277, %273 ]
  %285 = add nsw i32 %269, -18
  %286 = icmp ult i32 %285, 7
  br i1 %286, label %314, label %301

287:                                              ; preds = %259, %287
  %288 = phi i32 [ %298, %287 ], [ %260, %259 ]
  %289 = phi double [ %297, %287 ], [ %261, %259 ]
  %290 = fmul double %289, 2.000000e+00
  %291 = fmul double %290, 2.000000e+00
  %292 = fmul double %291, 2.000000e+00
  %293 = fmul double %292, 2.000000e+00
  %294 = fmul double %293, 2.000000e+00
  %295 = fmul double %294, 2.000000e+00
  %296 = fmul double %295, 2.000000e+00
  %297 = fmul double %296, 2.000000e+00
  %298 = add nsw i32 %288, -8
  %299 = add i32 %288, -9
  %300 = icmp ult i32 %299, -2
  br i1 %300, label %287, label %264, !llvm.loop !10021

301:                                              ; preds = %281, %301
  %302 = phi i32 [ %312, %301 ], [ %283, %281 ]
  %303 = phi double [ %311, %301 ], [ %284, %281 ]
  %304 = fmul double %303, 5.000000e-01
  %305 = fmul double %304, 5.000000e-01
  %306 = fmul double %305, 5.000000e-01
  %307 = fmul double %306, 5.000000e-01
  %308 = fmul double %307, 5.000000e-01
  %309 = fmul double %308, 5.000000e-01
  %310 = fmul double %309, 5.000000e-01
  %311 = fmul double %310, 5.000000e-01
  %312 = add nsw i32 %302, 8
  %313 = icmp eq i32 %312, 0
  br i1 %313, label %314, label %301, !llvm.loop !10022

314:                                              ; preds = %281, %301, %264, %239, %236
  %315 = phi double [ %238, %236 ], [ %241, %239 ], [ %265, %264 ], [ %282, %281 ], [ %311, %301 ]
  %316 = icmp ult i32 %229, 32768
  %317 = fneg double %315
  %318 = select i1 %316, double %315, double %317
  br label %358

319:                                              ; preds = %219
  %320 = bitcast i32 %229 to float
  %321 = fpext float %320 to double
  br label %358

322:                                              ; preds = %219, %219
  br label %324

323:                                              ; preds = %219
  br label %324

324:                                              ; preds = %323, %322, %219, %219
  %325 = phi i32 [ 32, %323 ], [ 16, %322 ], [ 8, %219 ], [ 8, %219 ]
  %326 = zext i32 %229 to i64
  switch i64 %144, label %336 [
    i64 21, label %327
    i64 20, label %327
    i64 3, label %327
    i64 5, label %327
  ]

327:                                              ; preds = %324, %324, %324, %324
  %328 = add nsw i32 %325, -1
  %329 = shl nuw i32 1, %328
  %330 = and i32 %329, %229
  %331 = icmp eq i32 %330, 0
  %332 = zext nneg i32 %325 to i64
  %333 = shl nsw i64 -1, %332
  %334 = select i1 %331, i64 0, i64 %333
  %335 = add nsw i64 %334, %326
  br label %336

336:                                              ; preds = %327, %324
  %337 = phi i64 [ %335, %327 ], [ %326, %324 ]
  %338 = load i64, ptr addrspace(1) %202, align 8
  %339 = icmp eq i64 %338, 0
  br i1 %339, label %340, label %342

340:                                              ; preds = %336
  %341 = sitofp i64 %337 to double
  br label %358

342:                                              ; preds = %336
  switch i64 %144, label %343 [
    i64 21, label %344
    i64 20, label %344
    i64 3, label %344
  ]

343:                                              ; preds = %342
  br label %344

344:                                              ; preds = %343, %342, %342, %342
  %345 = phi i32 [ -1, %342 ], [ %204, %343 ], [ -1, %342 ], [ -1, %342 ]
  %346 = add nsw i32 %345, %325
  %347 = zext nneg i32 %346 to i64
  %348 = shl nsw i64 -1, %347
  %349 = xor i64 %348, -1
  %350 = sitofp i64 %337 to double
  %351 = sitofp i64 %349 to double
  %352 = fdiv double %350, %351
  %353 = fcmp oge double %352, -1.000000e+00
  %354 = select i1 %353, double %352, double -1.000000e+00
  br label %358

355:                                              ; preds = %205
  %356 = icmp eq i32 %206, 3
  %357 = select i1 %356, double 1.000000e+00, double 0.000000e+00
  br label %358

358:                                              ; preds = %355, %344, %340, %319, %314
  %359 = phi double [ %357, %355 ], [ %318, %314 ], [ %321, %319 ], [ %354, %344 ], [ %341, %340 ]
  %360 = insertelement <4 x double> %207, double %359, i32 %206
  %361 = add nuw nsw i32 %206, 1
  %362 = icmp eq i32 %361, 4
  br i1 %362, label %363, label %205, !llvm.loop !10029

363:                                              ; preds = %358, %136
  %364 = phi <4 x double> [ zeroinitializer, %136 ], [ %360, %358 ]
  %365 = getelementptr inbounds %struct.q, ptr %4, i64 0, i32 2
  %366 = load i32, ptr %365, align 4
  %367 = icmp eq i32 %366, 0
  br i1 %367, label %1857, label %368

368:                                              ; preds = %363
  %369 = getelementptr inbounds %struct.q, ptr %4, i64 0, i32 3
  %370 = load float, ptr %369, align 4
  %371 = fpext float %370 to double
  %372 = extractelement <4 x double> %364, i64 0
  %373 = fcmp oge double %372, %371
  %374 = select i1 %373, double 1.000000e+00, double 0.000000e+00
  %375 = insertelement <4 x double> <double poison, double 0.000000e+00, double 0.000000e+00, double 1.000000e+00>, double %374, i64 0
  br label %1857

376:                                              ; preds = %15
  %377 = icmp eq i32 %7, 1
  %378 = sitofp i64 %23 to double
  %379 = select i1 %377, double %378, double 1.000000e+00
  %380 = fmul double %379, %8
  %381 = fadd double %380, -5.000000e-01
  %382 = tail call double @llvm.floor.f64(double %381)
  %383 = fptosi double %382 to i64
  %384 = getelementptr inbounds %struct.q, ptr %4, i64 0, i32 1, i64 0
  %385 = load i32, ptr %384, align 4
  %386 = sext i32 %385 to i64
  %387 = add i64 %386, %383
  switch i32 %6, label %405 [
    i32 0, label %388
    i32 2, label %392
    i32 3, label %396
  ]

388:                                              ; preds = %376
  %389 = srem i64 %387, %23
  %390 = add nsw i64 %389, %23
  %391 = srem i64 %390, %23
  br label %409

392:                                              ; preds = %376
  %393 = icmp slt i64 %387, %23
  %394 = tail call i64 @llvm.smax.i64(i64 %387, i64 -1)
  %395 = select i1 %393, i64 %394, i64 -1
  br label %409

396:                                              ; preds = %376
  %397 = shl nuw nsw i64 %23, 1
  %398 = srem i64 %387, %397
  %399 = add nsw i64 %398, %397
  %400 = srem i64 %399, %397
  %401 = icmp slt i64 %400, %23
  %402 = xor i64 %400, -1
  %403 = add i64 %397, %402
  %404 = select i1 %401, i64 %400, i64 %403
  br label %409

405:                                              ; preds = %376
  %406 = tail call noundef i64 @llvm.smax.i64(i64 %387, i64 0)
  %407 = add nsw i64 %23, -1
  %408 = tail call noundef i64 @llvm.smin.i64(i64 %406, i64 %407)
  br label %409

409:                                              ; preds = %405, %396, %392, %388
  %410 = phi i64 [ %408, %405 ], [ %404, %396 ], [ %395, %392 ], [ %391, %388 ]
  %411 = add i64 %387, 1
  switch i32 %6, label %429 [
    i32 0, label %412
    i32 2, label %416
    i32 3, label %420
  ]

412:                                              ; preds = %409
  %413 = srem i64 %411, %23
  %414 = add nsw i64 %413, %23
  %415 = srem i64 %414, %23
  br label %433

416:                                              ; preds = %409
  %417 = icmp slt i64 %411, %23
  %418 = tail call i64 @llvm.smax.i64(i64 %411, i64 -1)
  %419 = select i1 %417, i64 %418, i64 -1
  br label %433

420:                                              ; preds = %409
  %421 = shl nuw nsw i64 %23, 1
  %422 = srem i64 %411, %421
  %423 = add nsw i64 %422, %421
  %424 = srem i64 %423, %421
  %425 = icmp slt i64 %424, %23
  %426 = xor i64 %424, -1
  %427 = add i64 %421, %426
  %428 = select i1 %425, i64 %424, i64 %427
  br label %433

429:                                              ; preds = %409
  %430 = tail call noundef i64 @llvm.smax.i64(i64 %411, i64 0)
  %431 = add nsw i64 %23, -1
  %432 = tail call noundef i64 @llvm.smin.i64(i64 %430, i64 %431)
  br label %433

433:                                              ; preds = %429, %420, %416, %412
  %434 = phi i64 [ %432, %429 ], [ %428, %420 ], [ %419, %416 ], [ %415, %412 ]
  %435 = icmp sgt i32 %12, 1
  br i1 %435, label %436, label %473

436:                                              ; preds = %433
  %437 = getelementptr i64, ptr addrspace(1) %0, i64 10
  %438 = load i64, ptr addrspace(1) %437, align 8
  %439 = tail call noundef i64 @llvm.smax.i64(i64 %438, i64 1)
  %440 = lshr i64 %439, %21
  %441 = tail call noundef i64 @llvm.smax.i64(i64 %440, i64 1)
  %442 = sitofp i64 %441 to double
  %443 = select i1 %377, double %442, double 1.000000e+00
  %444 = fmul double %443, %9
  %445 = fadd double %444, -5.000000e-01
  %446 = tail call double @llvm.floor.f64(double %445)
  %447 = fptosi double %446 to i64
  %448 = getelementptr inbounds %struct.q, ptr %4, i64 0, i32 1, i64 1
  %449 = load i32, ptr %448, align 4
  %450 = sext i32 %449 to i64
  %451 = add i64 %447, %450
  switch i32 %6, label %469 [
    i32 0, label %452
    i32 2, label %456
    i32 3, label %460
  ]

452:                                              ; preds = %436
  %453 = srem i64 %451, %441
  %454 = add nsw i64 %453, %441
  %455 = srem i64 %454, %441
  br label %473

456:                                              ; preds = %436
  %457 = icmp slt i64 %451, %441
  %458 = tail call i64 @llvm.smax.i64(i64 %451, i64 -1)
  %459 = select i1 %457, i64 %458, i64 -1
  br label %473

460:                                              ; preds = %436
  %461 = shl nuw nsw i64 %441, 1
  %462 = srem i64 %451, %461
  %463 = add nsw i64 %462, %461
  %464 = srem i64 %463, %461
  %465 = icmp slt i64 %464, %441
  %466 = xor i64 %464, -1
  %467 = add i64 %461, %466
  %468 = select i1 %465, i64 %464, i64 %467
  br label %473

469:                                              ; preds = %436
  %470 = tail call noundef i64 @llvm.smax.i64(i64 %451, i64 0)
  %471 = add nsw i64 %441, -1
  %472 = tail call noundef i64 @llvm.smin.i64(i64 %470, i64 %471)
  br label %473

473:                                              ; preds = %469, %460, %456, %452, %433
  %474 = phi i64 [ 0, %433 ], [ %472, %469 ], [ %468, %460 ], [ %459, %456 ], [ %455, %452 ]
  br i1 %435, label %475, label %513

475:                                              ; preds = %473
  %476 = getelementptr i64, ptr addrspace(1) %0, i64 10
  %477 = load i64, ptr addrspace(1) %476, align 8
  %478 = tail call noundef i64 @llvm.smax.i64(i64 %477, i64 1)
  %479 = lshr i64 %478, %21
  %480 = tail call noundef i64 @llvm.smax.i64(i64 %479, i64 1)
  %481 = sitofp i64 %480 to double
  %482 = select i1 %377, double %481, double 1.000000e+00
  %483 = fmul double %482, %9
  %484 = fadd double %483, -5.000000e-01
  %485 = tail call double @llvm.floor.f64(double %484)
  %486 = fptosi double %485 to i64
  %487 = getelementptr inbounds %struct.q, ptr %4, i64 0, i32 1, i64 1
  %488 = load i32, ptr %487, align 4
  %489 = sext i32 %488 to i64
  %490 = add nsw i64 %489, 1
  %491 = add i64 %490, %486
  switch i32 %6, label %509 [
    i32 0, label %492
    i32 2, label %496
    i32 3, label %500
  ]

492:                                              ; preds = %475
  %493 = srem i64 %491, %480
  %494 = add nsw i64 %493, %480
  %495 = srem i64 %494, %480
  br label %513

496:                                              ; preds = %475
  %497 = icmp slt i64 %491, %480
  %498 = tail call i64 @llvm.smax.i64(i64 %491, i64 -1)
  %499 = select i1 %497, i64 %498, i64 -1
  br label %513

500:                                              ; preds = %475
  %501 = shl nuw nsw i64 %480, 1
  %502 = srem i64 %491, %501
  %503 = add nsw i64 %502, %501
  %504 = srem i64 %503, %501
  %505 = icmp slt i64 %504, %480
  %506 = xor i64 %504, -1
  %507 = add i64 %501, %506
  %508 = select i1 %505, i64 %504, i64 %507
  br label %513

509:                                              ; preds = %475
  %510 = tail call noundef i64 @llvm.smax.i64(i64 %491, i64 0)
  %511 = add nsw i64 %480, -1
  %512 = tail call noundef i64 @llvm.smin.i64(i64 %510, i64 %511)
  br label %513

513:                                              ; preds = %509, %500, %496, %492, %473
  %514 = phi i64 [ 0, %473 ], [ %512, %509 ], [ %508, %500 ], [ %499, %496 ], [ %495, %492 ]
  %515 = icmp sgt i32 %12, 2
  br i1 %515, label %516, label %553

516:                                              ; preds = %513
  %517 = getelementptr i64, ptr addrspace(1) %0, i64 11
  %518 = load i64, ptr addrspace(1) %517, align 8
  %519 = tail call noundef i64 @llvm.smax.i64(i64 %518, i64 1)
  %520 = lshr i64 %519, %21
  %521 = tail call noundef i64 @llvm.smax.i64(i64 %520, i64 1)
  %522 = sitofp i64 %521 to double
  %523 = select i1 %377, double %522, double 1.000000e+00
  %524 = fmul double %523, %10
  %525 = fadd double %524, -5.000000e-01
  %526 = tail call double @llvm.floor.f64(double %525)
  %527 = fptosi double %526 to i64
  %528 = getelementptr inbounds %struct.q, ptr %4, i64 0, i32 1, i64 2
  %529 = load i32, ptr %528, align 4
  %530 = sext i32 %529 to i64
  %531 = add i64 %527, %530
  switch i32 %6, label %549 [
    i32 0, label %532
    i32 2, label %536
    i32 3, label %540
  ]

532:                                              ; preds = %516
  %533 = srem i64 %531, %521
  %534 = add nsw i64 %533, %521
  %535 = srem i64 %534, %521
  br label %553

536:                                              ; preds = %516
  %537 = icmp slt i64 %531, %521
  %538 = tail call i64 @llvm.smax.i64(i64 %531, i64 -1)
  %539 = select i1 %537, i64 %538, i64 -1
  br label %553

540:                                              ; preds = %516
  %541 = shl nuw nsw i64 %521, 1
  %542 = srem i64 %531, %541
  %543 = add nsw i64 %542, %541
  %544 = srem i64 %543, %541
  %545 = icmp slt i64 %544, %521
  %546 = xor i64 %544, -1
  %547 = add i64 %541, %546
  %548 = select i1 %545, i64 %544, i64 %547
  br label %553

549:                                              ; preds = %516
  %550 = tail call noundef i64 @llvm.smax.i64(i64 %531, i64 0)
  %551 = add nsw i64 %521, -1
  %552 = tail call noundef i64 @llvm.smin.i64(i64 %550, i64 %551)
  br label %553

553:                                              ; preds = %549, %540, %536, %532, %513
  %554 = phi i64 [ 0, %513 ], [ %552, %549 ], [ %548, %540 ], [ %539, %536 ], [ %535, %532 ]
  br i1 %515, label %555, label %593

555:                                              ; preds = %553
  %556 = getelementptr i64, ptr addrspace(1) %0, i64 11
  %557 = load i64, ptr addrspace(1) %556, align 8
  %558 = tail call noundef i64 @llvm.smax.i64(i64 %557, i64 1)
  %559 = lshr i64 %558, %21
  %560 = tail call noundef i64 @llvm.smax.i64(i64 %559, i64 1)
  %561 = sitofp i64 %560 to double
  %562 = select i1 %377, double %561, double 1.000000e+00
  %563 = fmul double %562, %10
  %564 = fadd double %563, -5.000000e-01
  %565 = tail call double @llvm.floor.f64(double %564)
  %566 = fptosi double %565 to i64
  %567 = getelementptr inbounds %struct.q, ptr %4, i64 0, i32 1, i64 2
  %568 = load i32, ptr %567, align 4
  %569 = sext i32 %568 to i64
  %570 = add nsw i64 %569, 1
  %571 = add i64 %570, %566
  switch i32 %6, label %589 [
    i32 0, label %572
    i32 2, label %576
    i32 3, label %580
  ]

572:                                              ; preds = %555
  %573 = srem i64 %571, %560
  %574 = add nsw i64 %573, %560
  %575 = srem i64 %574, %560
  br label %593

576:                                              ; preds = %555
  %577 = icmp slt i64 %571, %560
  %578 = tail call i64 @llvm.smax.i64(i64 %571, i64 -1)
  %579 = select i1 %577, i64 %578, i64 -1
  br label %593

580:                                              ; preds = %555
  %581 = shl nuw nsw i64 %560, 1
  %582 = srem i64 %571, %581
  %583 = add nsw i64 %582, %581
  %584 = srem i64 %583, %581
  %585 = icmp slt i64 %584, %560
  %586 = xor i64 %584, -1
  %587 = add i64 %581, %586
  %588 = select i1 %585, i64 %584, i64 %587
  br label %593

589:                                              ; preds = %555
  %590 = tail call noundef i64 @llvm.smax.i64(i64 %571, i64 0)
  %591 = add nsw i64 %560, -1
  %592 = tail call noundef i64 @llvm.smin.i64(i64 %590, i64 %591)
  br label %593

593:                                              ; preds = %589, %580, %576, %572, %553
  %594 = phi i64 [ 0, %553 ], [ %592, %589 ], [ %588, %580 ], [ %579, %576 ], [ %575, %572 ]
  %595 = fsub double %381, %382
  %596 = fmul double %595, 2.560000e+02
  %597 = fadd double %596, 5.000000e-01
  %598 = tail call double @llvm.floor.f64(double %597)
  %599 = fptosi double %598 to i64
  %600 = sitofp i64 %599 to double
  %601 = fmul double %600, 3.906250e-03
  br i1 %435, label %602, label %620

602:                                              ; preds = %593
  %603 = getelementptr i64, ptr addrspace(1) %0, i64 10
  %604 = load i64, ptr addrspace(1) %603, align 8
  %605 = tail call noundef i64 @llvm.smax.i64(i64 %604, i64 1)
  %606 = lshr i64 %605, %21
  %607 = tail call noundef i64 @llvm.smax.i64(i64 %606, i64 1)
  %608 = sitofp i64 %607 to double
  %609 = select i1 %377, double %608, double 1.000000e+00
  %610 = fmul double %609, %9
  %611 = fadd double %610, -5.000000e-01
  %612 = tail call double @llvm.floor.f64(double %611)
  %613 = fsub double %611, %612
  %614 = fmul double %613, 2.560000e+02
  %615 = fadd double %614, 5.000000e-01
  %616 = tail call double @llvm.floor.f64(double %615)
  %617 = fptosi double %616 to i64
  %618 = sitofp i64 %617 to double
  %619 = fmul double %618, 3.906250e-03
  br label %620

620:                                              ; preds = %602, %593
  %621 = phi double [ %619, %602 ], [ 0.000000e+00, %593 ]
  br i1 %515, label %622, label %640

622:                                              ; preds = %620
  %623 = getelementptr i64, ptr addrspace(1) %0, i64 11
  %624 = load i64, ptr addrspace(1) %623, align 8
  %625 = tail call noundef i64 @llvm.smax.i64(i64 %624, i64 1)
  %626 = lshr i64 %625, %21
  %627 = tail call noundef i64 @llvm.smax.i64(i64 %626, i64 1)
  %628 = sitofp i64 %627 to double
  %629 = select i1 %377, double %628, double 1.000000e+00
  %630 = fmul double %629, %10
  %631 = fadd double %630, -5.000000e-01
  %632 = tail call double @llvm.floor.f64(double %631)
  %633 = fsub double %631, %632
  %634 = fmul double %633, 2.560000e+02
  %635 = fadd double %634, 5.000000e-01
  %636 = tail call double @llvm.floor.f64(double %635)
  %637 = fptosi double %636 to i64
  %638 = sitofp i64 %637 to double
  %639 = fmul double %638, 3.906250e-03
  br label %640

640:                                              ; preds = %622, %620
  %641 = phi double [ %639, %622 ], [ 0.000000e+00, %620 ]
  %642 = fsub double 1.000000e+00, %601
  %643 = fsub double 1.000000e+00, %621
  %644 = fsub double 1.000000e+00, %641
  %645 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %646 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 2
  %647 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 14
  %648 = icmp sgt i32 %17, 0
  %649 = getelementptr i64, ptr addrspace(1) %0, i64 11
  %650 = getelementptr i64, ptr addrspace(1) %0, i64 10
  %651 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 8
  %652 = getelementptr i64, ptr addrspace(1) %0, i64 1
  %653 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 7
  %654 = getelementptr inbounds %struct.q, ptr %4, i64 0, i32 2
  %655 = load i32, ptr %654, align 4
  %656 = icmp eq i32 %655, 0
  %657 = getelementptr inbounds %struct.q, ptr %4, i64 0, i32 3
  br label %658

658:                                              ; preds = %921, %640
  %659 = phi <4 x double> [ zeroinitializer, %640 ], [ %924, %921 ]
  %660 = phi i32 [ 0, %640 ], [ %925, %921 ]
  %661 = and i32 %660, 1
  %662 = icmp eq i32 %661, 0
  %663 = select i1 %662, double %642, double %601
  %664 = and i32 %660, 2
  %665 = icmp eq i32 %664, 0
  %666 = select i1 %665, double %643, double %621
  %667 = fmul double %663, %666
  %668 = and i32 %660, 4
  %669 = icmp eq i32 %668, 0
  %670 = select i1 %669, double %644, double %641
  %671 = fmul double %670, %667
  %672 = insertelement <4 x double> poison, double %671, i64 0
  %673 = shufflevector <4 x double> %672, <4 x double> poison, <4 x i32> zeroinitializer
  %674 = select i1 %662, i64 %410, i64 %434
  %675 = select i1 %665, i64 %474, i64 %514
  %676 = select i1 %669, i64 %554, i64 %594
  %677 = or i64 %675, %674
  %678 = or i64 %677, %676
  %679 = icmp sgt i64 %678, -1
  br i1 %679, label %680, label %912

680:                                              ; preds = %658
  %681 = load i64, ptr addrspace(1) %645, align 8
  %682 = load i64, ptr addrspace(1) %646, align 8
  switch i64 %682, label %684 [
    i64 0, label %685
    i64 20, label %685
    i64 1, label %683
    i64 21, label %683
    i64 6, label %683
  ]

683:                                              ; preds = %680, %680, %680
  br label %685

684:                                              ; preds = %680
  br label %685

685:                                              ; preds = %684, %683, %680, %680
  %686 = phi i32 [ 4, %684 ], [ 2, %683 ], [ 1, %680 ], [ 1, %680 ]
  %687 = load i64, ptr addrspace(1) %647, align 8
  br i1 %648, label %693, label %688

688:                                              ; preds = %685
  %689 = load i64, ptr addrspace(1) %649, align 8
  %690 = load i64, ptr addrspace(1) %650, align 8
  %691 = tail call noundef i64 @llvm.smax.i64(i64 %689, i64 1)
  %692 = tail call noundef i64 @llvm.smax.i64(i64 %690, i64 1)
  br label %716

693:                                              ; preds = %685
  %694 = load i64, ptr addrspace(1) %651, align 8
  %695 = tail call noundef i64 @llvm.smax.i64(i64 %694, i64 1)
  %696 = load i64, ptr addrspace(1) %650, align 8
  %697 = tail call noundef i64 @llvm.smax.i64(i64 %696, i64 1)
  %698 = load i64, ptr addrspace(1) %649, align 8
  %699 = tail call noundef i64 @llvm.smax.i64(i64 %698, i64 1)
  br label %700

700:                                              ; preds = %700, %693
  %701 = phi i32 [ 0, %693 ], [ %714, %700 ]
  %702 = phi i64 [ 0, %693 ], [ %713, %700 ]
  %703 = zext nneg i32 %701 to i64
  %704 = lshr i64 %20, %703
  %705 = tail call noundef i64 @llvm.smax.i64(i64 %704, i64 1)
  %706 = mul nsw i64 %705, %695
  %707 = lshr i64 %697, %703
  %708 = tail call noundef i64 @llvm.smax.i64(i64 %707, i64 1)
  %709 = mul nsw i64 %706, %708
  %710 = lshr i64 %699, %703
  %711 = tail call noundef i64 @llvm.smax.i64(i64 %710, i64 1)
  %712 = mul nsw i64 %709, %711
  %713 = add nuw nsw i64 %712, %702
  %714 = add nuw nsw i32 %701, 1
  %715 = icmp eq i32 %714, %17
  br i1 %715, label %716, label %700, !llvm.loop !10026

716:                                              ; preds = %700, %688
  %717 = phi i64 [ %692, %688 ], [ %697, %700 ]
  %718 = phi i64 [ %691, %688 ], [ %699, %700 ]
  %719 = phi i64 [ 0, %688 ], [ %713, %700 ]
  %720 = lshr i64 %718, %21
  %721 = tail call noundef i64 @llvm.smax.i64(i64 %720, i64 1)
  %722 = lshr i64 %717, %21
  %723 = tail call noundef i64 @llvm.smax.i64(i64 %722, i64 1)
  %724 = icmp eq i64 %687, 0
  br i1 %724, label %737, label %725

725:                                              ; preds = %716
  %726 = load i64, ptr addrspace(1) %652, align 8
  %727 = mul nsw i64 %721, %11
  %728 = add nsw i64 %727, %676
  %729 = mul nsw i64 %728, %723
  %730 = add nsw i64 %729, %675
  %731 = mul nsw i64 %730, %687
  %732 = mul nsw i64 %681, %674
  %733 = zext nneg i32 %686 to i64
  %734 = mul nsw i64 %732, %733
  %735 = add nsw i64 %731, %734
  %736 = add i64 %735, %726
  br label %750

737:                                              ; preds = %716
  %738 = mul nsw i64 %721, %11
  %739 = add nsw i64 %738, %676
  %740 = mul nsw i64 %739, %723
  %741 = add nsw i64 %740, %675
  %742 = mul nsw i64 %741, %23
  %743 = add i64 %719, %674
  %744 = add i64 %743, %742
  %745 = load i64, ptr addrspace(1) %652, align 8
  %746 = zext nneg i32 %686 to i64
  %747 = mul i64 %681, %746
  %748 = mul i64 %747, %744
  %749 = add i64 %745, %748
  br label %750

750:                                              ; preds = %737, %725
  %751 = phi i64 [ %736, %725 ], [ %749, %737 ]
  %752 = icmp eq i64 %682, 5
  %753 = sext i1 %752 to i32
  br label %754

754:                                              ; preds = %907, %750
  %755 = phi i32 [ 0, %750 ], [ %910, %907 ]
  %756 = phi <4 x double> [ zeroinitializer, %750 ], [ %909, %907 ]
  %757 = zext nneg i32 %755 to i64
  %758 = icmp sgt i64 %681, %757
  br i1 %758, label %759, label %904

759:                                              ; preds = %754
  %760 = mul nuw nsw i32 %755, %686
  %761 = zext nneg i32 %760 to i64
  %762 = add i64 %751, %761
  switch i64 %682, label %764 [
    i64 0, label %765
    i64 20, label %765
    i64 1, label %763
    i64 21, label %763
    i64 6, label %763
  ]

763:                                              ; preds = %759, %759, %759
  br label %765

764:                                              ; preds = %759
  br label %765

765:                                              ; preds = %764, %763, %759, %759
  %766 = phi i32 [ 4, %764 ], [ 2, %763 ], [ 1, %759 ], [ 1, %759 ]
  %767 = inttoptr i64 %762 to ptr addrspace(1)
  br label %769

768:                                              ; preds = %769
  switch i64 %682, label %872 [
    i64 6, label %781
    i64 7, label %868
    i64 0, label %873
    i64 20, label %873
    i64 1, label %871
    i64 21, label %871
  ]

769:                                              ; preds = %769, %765
  %770 = phi i32 [ 0, %765 ], [ %778, %769 ]
  %771 = phi i32 [ 0, %765 ], [ %779, %769 ]
  %772 = zext nneg i32 %771 to i64
  %773 = getelementptr inbounds i8, ptr addrspace(1) %767, i64 %772
  %774 = load i8, ptr addrspace(1) %773, align 1
  %775 = zext i8 %774 to i32
  %776 = shl nsw i32 %771, 3
  %777 = shl i32 %775, %776
  %778 = or i32 %777, %770
  %779 = add nuw nsw i32 %771, 1
  %780 = icmp eq i32 %779, %766
  br i1 %780, label %768, label %769, !llvm.loop !10016

781:                                              ; preds = %768
  %782 = lshr i32 %778, 10
  %783 = and i32 %782, 31
  %784 = and i32 %778, 1023
  switch i32 %783, label %791 [
    i32 0, label %785
    i32 31, label %788
  ]

785:                                              ; preds = %781
  %786 = uitofp i32 %784 to double
  %787 = fmul double %786, 0x3E70000000000000
  br label %863

788:                                              ; preds = %781
  %789 = icmp eq i32 %784, 0
  %790 = select i1 %789, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %863

791:                                              ; preds = %781
  %792 = or disjoint i32 %784, 1024
  %793 = uitofp i32 %792 to double
  %794 = add nsw i32 %783, -25
  %795 = icmp ugt i32 %783, 25
  br i1 %795, label %796, label %813

796:                                              ; preds = %791
  %797 = add nsw i32 %783, -26
  %798 = and i32 %794, 7
  %799 = icmp eq i32 %798, 0
  br i1 %799, label %808, label %800

800:                                              ; preds = %796, %800
  %801 = phi i32 [ %805, %800 ], [ %794, %796 ]
  %802 = phi double [ %804, %800 ], [ %793, %796 ]
  %803 = phi i32 [ %806, %800 ], [ 0, %796 ]
  %804 = fmul double %802, 2.000000e+00
  %805 = add nsw i32 %801, -1
  %806 = add i32 %803, 1
  %807 = icmp eq i32 %806, %798
  br i1 %807, label %808, label %800, !llvm.loop !10030

808:                                              ; preds = %800, %796
  %809 = phi i32 [ %794, %796 ], [ %805, %800 ]
  %810 = phi double [ %793, %796 ], [ %804, %800 ]
  %811 = phi double [ undef, %796 ], [ %804, %800 ]
  %812 = icmp ult i32 %797, 7
  br i1 %812, label %813, label %836

813:                                              ; preds = %808, %836, %791
  %814 = phi double [ %793, %791 ], [ %811, %808 ], [ %846, %836 ]
  %815 = phi i32 [ %794, %791 ], [ 0, %836 ], [ 0, %808 ]
  %816 = icmp slt i32 %815, 0
  br i1 %816, label %817, label %863

817:                                              ; preds = %813
  %818 = tail call i32 @llvm.umin.i32(i32 %783, i32 25)
  %819 = sub nsw i32 1, %818
  %820 = and i32 %819, 7
  %821 = icmp eq i32 %820, 0
  br i1 %821, label %830, label %822

822:                                              ; preds = %817, %822
  %823 = phi i32 [ %827, %822 ], [ %815, %817 ]
  %824 = phi double [ %826, %822 ], [ %814, %817 ]
  %825 = phi i32 [ %828, %822 ], [ 0, %817 ]
  %826 = fmul double %824, 5.000000e-01
  %827 = add nsw i32 %823, 1
  %828 = add i32 %825, 1
  %829 = icmp eq i32 %828, %820
  br i1 %829, label %830, label %822, !llvm.loop !10031

830:                                              ; preds = %822, %817
  %831 = phi double [ undef, %817 ], [ %826, %822 ]
  %832 = phi i32 [ %815, %817 ], [ %827, %822 ]
  %833 = phi double [ %814, %817 ], [ %826, %822 ]
  %834 = add nsw i32 %818, -18
  %835 = icmp ult i32 %834, 7
  br i1 %835, label %863, label %850

836:                                              ; preds = %808, %836
  %837 = phi i32 [ %847, %836 ], [ %809, %808 ]
  %838 = phi double [ %846, %836 ], [ %810, %808 ]
  %839 = fmul double %838, 2.000000e+00
  %840 = fmul double %839, 2.000000e+00
  %841 = fmul double %840, 2.000000e+00
  %842 = fmul double %841, 2.000000e+00
  %843 = fmul double %842, 2.000000e+00
  %844 = fmul double %843, 2.000000e+00
  %845 = fmul double %844, 2.000000e+00
  %846 = fmul double %845, 2.000000e+00
  %847 = add nsw i32 %837, -8
  %848 = add i32 %837, -9
  %849 = icmp ult i32 %848, -2
  br i1 %849, label %836, label %813, !llvm.loop !10021

850:                                              ; preds = %830, %850
  %851 = phi i32 [ %861, %850 ], [ %832, %830 ]
  %852 = phi double [ %860, %850 ], [ %833, %830 ]
  %853 = fmul double %852, 5.000000e-01
  %854 = fmul double %853, 5.000000e-01
  %855 = fmul double %854, 5.000000e-01
  %856 = fmul double %855, 5.000000e-01
  %857 = fmul double %856, 5.000000e-01
  %858 = fmul double %857, 5.000000e-01
  %859 = fmul double %858, 5.000000e-01
  %860 = fmul double %859, 5.000000e-01
  %861 = add nsw i32 %851, 8
  %862 = icmp eq i32 %861, 0
  br i1 %862, label %863, label %850, !llvm.loop !10022

863:                                              ; preds = %830, %850, %813, %788, %785
  %864 = phi double [ %787, %785 ], [ %790, %788 ], [ %814, %813 ], [ %831, %830 ], [ %860, %850 ]
  %865 = icmp ult i32 %778, 32768
  %866 = fneg double %864
  %867 = select i1 %865, double %864, double %866
  br label %907

868:                                              ; preds = %768
  %869 = bitcast i32 %778 to float
  %870 = fpext float %869 to double
  br label %907

871:                                              ; preds = %768, %768
  br label %873

872:                                              ; preds = %768
  br label %873

873:                                              ; preds = %872, %871, %768, %768
  %874 = phi i32 [ 32, %872 ], [ 16, %871 ], [ 8, %768 ], [ 8, %768 ]
  %875 = zext i32 %778 to i64
  switch i64 %682, label %885 [
    i64 21, label %876
    i64 20, label %876
    i64 3, label %876
    i64 5, label %876
  ]

876:                                              ; preds = %873, %873, %873, %873
  %877 = add nsw i32 %874, -1
  %878 = shl nuw i32 1, %877
  %879 = and i32 %878, %778
  %880 = icmp eq i32 %879, 0
  %881 = zext nneg i32 %874 to i64
  %882 = shl nsw i64 -1, %881
  %883 = select i1 %880, i64 0, i64 %882
  %884 = add nsw i64 %883, %875
  br label %885

885:                                              ; preds = %876, %873
  %886 = phi i64 [ %884, %876 ], [ %875, %873 ]
  %887 = load i64, ptr addrspace(1) %653, align 8
  %888 = icmp eq i64 %887, 0
  br i1 %888, label %889, label %891

889:                                              ; preds = %885
  %890 = sitofp i64 %886 to double
  br label %907

891:                                              ; preds = %885
  switch i64 %682, label %892 [
    i64 21, label %893
    i64 20, label %893
    i64 3, label %893
  ]

892:                                              ; preds = %891
  br label %893

893:                                              ; preds = %892, %891, %891, %891
  %894 = phi i32 [ -1, %891 ], [ %753, %892 ], [ -1, %891 ], [ -1, %891 ]
  %895 = add nsw i32 %894, %874
  %896 = zext nneg i32 %895 to i64
  %897 = shl nsw i64 -1, %896
  %898 = xor i64 %897, -1
  %899 = sitofp i64 %886 to double
  %900 = sitofp i64 %898 to double
  %901 = fdiv double %899, %900
  %902 = fcmp oge double %901, -1.000000e+00
  %903 = select i1 %902, double %901, double -1.000000e+00
  br label %907

904:                                              ; preds = %754
  %905 = icmp eq i32 %755, 3
  %906 = select i1 %905, double 1.000000e+00, double 0.000000e+00
  br label %907

907:                                              ; preds = %904, %893, %889, %868, %863
  %908 = phi double [ %906, %904 ], [ %867, %863 ], [ %870, %868 ], [ %903, %893 ], [ %890, %889 ]
  %909 = insertelement <4 x double> %756, double %908, i32 %755
  %910 = add nuw nsw i32 %755, 1
  %911 = icmp eq i32 %910, 4
  br i1 %911, label %912, label %754, !llvm.loop !10029

912:                                              ; preds = %907, %658
  %913 = phi <4 x double> [ zeroinitializer, %658 ], [ %909, %907 ]
  br i1 %656, label %921, label %914

914:                                              ; preds = %912
  %915 = load float, ptr %657, align 4
  %916 = fpext float %915 to double
  %917 = extractelement <4 x double> %913, i64 0
  %918 = fcmp oge double %917, %916
  %919 = select i1 %918, double 1.000000e+00, double 0.000000e+00
  %920 = insertelement <4 x double> <double poison, double 0.000000e+00, double 0.000000e+00, double 1.000000e+00>, double %919, i64 0
  br label %921

921:                                              ; preds = %914, %912
  %922 = phi <4 x double> [ %920, %914 ], [ %913, %912 ]
  %923 = fmul <4 x double> %673, %922
  %924 = fadd <4 x double> %659, %923
  %925 = add nuw nsw i32 %660, 1
  %926 = lshr i32 %925, %12
  %927 = icmp eq i32 %926, 0
  br i1 %927, label %658, label %1857, !llvm.loop !10032

928:                                              ; preds = %13
  %929 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 12
  %930 = load i64, ptr addrspace(1) %929, align 8
  %931 = tail call noundef i64 @llvm.smax.i64(i64 %930, i64 1)
  %932 = getelementptr inbounds %struct.sampling, ptr %1, i64 0, i32 4
  %933 = load double, ptr %932, align 8
  %934 = fcmp ole double %933, %3
  %935 = select i1 %934, double %3, double %933
  %936 = fcmp oge double %935, 0.000000e+00
  %937 = select i1 %936, double %935, double 0.000000e+00
  %938 = getelementptr inbounds %struct.sampling, ptr %1, i64 0, i32 5
  %939 = load double, ptr %938, align 8
  %940 = add nsw i64 %931, -1
  %941 = sitofp i64 %940 to double
  %942 = fcmp ole double %939, %941
  %943 = select i1 %942, double %939, double %941
  %944 = fcmp ole double %937, %943
  %945 = select i1 %944, double %937, double %943
  %946 = getelementptr inbounds %struct.sampling, ptr %1, i64 0, i32 3
  %947 = load i32, ptr %946, align 4
  %948 = icmp eq i32 %947, 1
  br i1 %948, label %949, label %952

949:                                              ; preds = %928
  %950 = tail call double @llvm.floor.f64(double %945)
  %951 = fptosi double %950 to i64
  br label %957

952:                                              ; preds = %928
  %953 = fadd double %945, 5.000000e-01
  %954 = tail call double @llvm.floor.f64(double %953)
  %955 = fptosi double %954 to i64
  %956 = tail call noundef i64 @llvm.smin.i64(i64 %955, i64 %940)
  br label %957

957:                                              ; preds = %952, %949
  %958 = phi i64 [ %951, %949 ], [ %956, %952 ]
  br i1 %948, label %959, label %968

959:                                              ; preds = %957
  %960 = tail call double @llvm.floor.f64(double %945)
  %961 = fsub double %945, %960
  %962 = fmul double %961, 2.560000e+02
  %963 = fadd double %962, 5.000000e-01
  %964 = tail call double @llvm.floor.f64(double %963)
  %965 = fptosi double %964 to i64
  %966 = sitofp i64 %965 to double
  %967 = fmul double %966, 3.906250e-03
  br label %968

968:                                              ; preds = %957, %959
  %969 = phi double [ %967, %959 ], [ 0.000000e+00, %957 ]
  %970 = getelementptr inbounds i8, ptr %4, i64 4
  %971 = load i32, ptr %970, align 4
  %972 = getelementptr inbounds i8, ptr %4, i64 8
  %973 = load i32, ptr %972, align 4
  %974 = getelementptr inbounds i8, ptr %4, i64 12
  %975 = load i32, ptr %974, align 4
  %976 = getelementptr inbounds i8, ptr %4, i64 16
  %977 = load i32, ptr %976, align 4
  %978 = getelementptr inbounds i8, ptr %4, i64 20
  %979 = load float, ptr %978, align 4
  %980 = fsub double 1.000000e+00, %969
  %981 = icmp eq i32 %5, 1
  %982 = getelementptr i64, ptr addrspace(1) %0, i64 9
  %983 = load i64, ptr addrspace(1) %982, align 8
  %984 = tail call noundef i64 @llvm.smax.i64(i64 %983, i64 1)
  %985 = getelementptr i64, ptr addrspace(1) %0, i64 1
  %986 = getelementptr i64, ptr addrspace(1) %0, i64 10
  %987 = getelementptr i64, ptr addrspace(1) %0, i64 2
  %988 = getelementptr i64, ptr addrspace(1) %0, i64 11
  %989 = icmp eq i32 %7, 1
  %990 = sext i32 %971 to i64
  %991 = icmp sgt i32 %12, 1
  %992 = sext i32 %973 to i64
  %993 = icmp sgt i32 %12, 2
  %994 = sext i32 %975 to i64
  %995 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %996 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 14
  %997 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 8
  %998 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 7
  %999 = icmp eq i32 %977, 0
  %1000 = fpext float %979 to double
  %1001 = add nsw i64 %992, 1
  %1002 = add nsw i64 %994, 1
  br label %1003

1003:                                             ; preds = %968, %1852
  %1004 = phi i1 [ true, %968 ], [ false, %1852 ]
  %1005 = phi i64 [ 0, %968 ], [ 1, %1852 ]
  %1006 = phi <4 x double> [ zeroinitializer, %968 ], [ %1855, %1852 ]
  %1007 = add nsw i64 %958, %1005
  %1008 = tail call noundef i64 @llvm.smin.i64(i64 %1007, i64 %940)
  %1009 = trunc i64 %1008 to i32
  %1010 = select i1 %1004, double %980, double %969
  %1011 = insertelement <4 x double> poison, double %1010, i64 0
  %1012 = shufflevector <4 x double> %1011, <4 x double> poison, <4 x i32> zeroinitializer
  %1013 = and i64 %1008, 4294967295
  %1014 = lshr i64 %984, %1013
  %1015 = tail call noundef i64 @llvm.smax.i64(i64 %1014, i64 1)
  br i1 %981, label %1342, label %1016

1016:                                             ; preds = %1003
  %1017 = load i64, ptr addrspace(1) %986, align 8
  %1018 = tail call noundef i64 @llvm.smax.i64(i64 %1017, i64 1)
  %1019 = lshr i64 %1018, %1013
  %1020 = tail call noundef i64 @llvm.smax.i64(i64 %1019, i64 1)
  %1021 = load i64, ptr addrspace(1) %988, align 8
  %1022 = tail call noundef i64 @llvm.smax.i64(i64 %1021, i64 1)
  %1023 = lshr i64 %1022, %1013
  %1024 = tail call noundef i64 @llvm.smax.i64(i64 %1023, i64 1)
  %1025 = sitofp i64 %1015 to double
  %1026 = select i1 %989, double %1025, double 1.000000e+00
  %1027 = fmul double %1026, %8
  %1028 = tail call double @llvm.floor.f64(double %1027)
  %1029 = fptosi double %1028 to i64
  %1030 = add nsw i64 %1029, %990
  switch i32 %6, label %1048 [
    i32 0, label %1031
    i32 2, label %1035
    i32 3, label %1039
  ]

1031:                                             ; preds = %1016
  %1032 = srem i64 %1030, %1015
  %1033 = add nsw i64 %1032, %1015
  %1034 = srem i64 %1033, %1015
  br label %1052

1035:                                             ; preds = %1016
  %1036 = icmp slt i64 %1030, %1015
  %1037 = tail call i64 @llvm.smax.i64(i64 %1030, i64 -1)
  %1038 = select i1 %1036, i64 %1037, i64 -1
  br label %1052

1039:                                             ; preds = %1016
  %1040 = shl nuw nsw i64 %1015, 1
  %1041 = srem i64 %1030, %1040
  %1042 = add nsw i64 %1041, %1040
  %1043 = srem i64 %1042, %1040
  %1044 = icmp slt i64 %1043, %1015
  %1045 = xor i64 %1043, -1
  %1046 = add i64 %1040, %1045
  %1047 = select i1 %1044, i64 %1043, i64 %1046
  br label %1052

1048:                                             ; preds = %1016
  %1049 = tail call noundef i64 @llvm.smax.i64(i64 %1030, i64 0)
  %1050 = add nsw i64 %1015, -1
  %1051 = tail call noundef i64 @llvm.smin.i64(i64 %1049, i64 %1050)
  br label %1052

1052:                                             ; preds = %1048, %1039, %1035, %1031
  %1053 = phi i64 [ %1051, %1048 ], [ %1047, %1039 ], [ %1038, %1035 ], [ %1034, %1031 ]
  br i1 %991, label %1054, label %1082

1054:                                             ; preds = %1052
  %1055 = sitofp i64 %1020 to double
  %1056 = select i1 %989, double %1055, double 1.000000e+00
  %1057 = fmul double %1056, %9
  %1058 = tail call double @llvm.floor.f64(double %1057)
  %1059 = fptosi double %1058 to i64
  %1060 = add nsw i64 %1059, %992
  switch i32 %6, label %1078 [
    i32 0, label %1061
    i32 2, label %1065
    i32 3, label %1069
  ]

1061:                                             ; preds = %1054
  %1062 = srem i64 %1060, %1020
  %1063 = add nsw i64 %1062, %1020
  %1064 = srem i64 %1063, %1020
  br label %1082

1065:                                             ; preds = %1054
  %1066 = icmp slt i64 %1060, %1020
  %1067 = tail call i64 @llvm.smax.i64(i64 %1060, i64 -1)
  %1068 = select i1 %1066, i64 %1067, i64 -1
  br label %1082

1069:                                             ; preds = %1054
  %1070 = shl nuw nsw i64 %1020, 1
  %1071 = srem i64 %1060, %1070
  %1072 = add nsw i64 %1071, %1070
  %1073 = srem i64 %1072, %1070
  %1074 = icmp slt i64 %1073, %1020
  %1075 = xor i64 %1073, -1
  %1076 = add i64 %1070, %1075
  %1077 = select i1 %1074, i64 %1073, i64 %1076
  br label %1082

1078:                                             ; preds = %1054
  %1079 = tail call noundef i64 @llvm.smax.i64(i64 %1060, i64 0)
  %1080 = add nsw i64 %1020, -1
  %1081 = tail call noundef i64 @llvm.smin.i64(i64 %1079, i64 %1080)
  br label %1082

1082:                                             ; preds = %1078, %1069, %1065, %1061, %1052
  %1083 = phi i64 [ 0, %1052 ], [ %1081, %1078 ], [ %1077, %1069 ], [ %1068, %1065 ], [ %1064, %1061 ]
  br i1 %993, label %1084, label %1112

1084:                                             ; preds = %1082
  %1085 = sitofp i64 %1024 to double
  %1086 = select i1 %989, double %1085, double 1.000000e+00
  %1087 = fmul double %1086, %10
  %1088 = tail call double @llvm.floor.f64(double %1087)
  %1089 = fptosi double %1088 to i64
  %1090 = add nsw i64 %1089, %994
  switch i32 %6, label %1108 [
    i32 0, label %1091
    i32 2, label %1095
    i32 3, label %1099
  ]

1091:                                             ; preds = %1084
  %1092 = srem i64 %1090, %1024
  %1093 = add nsw i64 %1092, %1024
  %1094 = srem i64 %1093, %1024
  br label %1112

1095:                                             ; preds = %1084
  %1096 = icmp slt i64 %1090, %1024
  %1097 = tail call i64 @llvm.smax.i64(i64 %1090, i64 -1)
  %1098 = select i1 %1096, i64 %1097, i64 -1
  br label %1112

1099:                                             ; preds = %1084
  %1100 = shl nuw nsw i64 %1024, 1
  %1101 = srem i64 %1090, %1100
  %1102 = add nsw i64 %1101, %1100
  %1103 = srem i64 %1102, %1100
  %1104 = icmp slt i64 %1103, %1024
  %1105 = xor i64 %1103, -1
  %1106 = add i64 %1100, %1105
  %1107 = select i1 %1104, i64 %1103, i64 %1106
  br label %1112

1108:                                             ; preds = %1084
  %1109 = tail call noundef i64 @llvm.smax.i64(i64 %1090, i64 0)
  %1110 = add nsw i64 %1024, -1
  %1111 = tail call noundef i64 @llvm.smin.i64(i64 %1109, i64 %1110)
  br label %1112

1112:                                             ; preds = %1108, %1099, %1095, %1091, %1082
  %1113 = phi i64 [ 0, %1082 ], [ %1111, %1108 ], [ %1107, %1099 ], [ %1098, %1095 ], [ %1094, %1091 ]
  %1114 = or i64 %1083, %1053
  %1115 = or i64 %1114, %1113
  %1116 = icmp sgt i64 %1115, -1
  br i1 %1116, label %1117, label %1335

1117:                                             ; preds = %1112
  %1118 = load i64, ptr addrspace(1) %995, align 8
  %1119 = load i64, ptr addrspace(1) %987, align 8
  switch i64 %1119, label %1121 [
    i64 0, label %1122
    i64 20, label %1122
    i64 1, label %1120
    i64 21, label %1120
    i64 6, label %1120
  ]

1120:                                             ; preds = %1117, %1117, %1117
  br label %1122

1121:                                             ; preds = %1117
  br label %1122

1122:                                             ; preds = %1121, %1120, %1117, %1117
  %1123 = phi i32 [ 4, %1121 ], [ 2, %1120 ], [ 1, %1117 ], [ 1, %1117 ]
  %1124 = load i64, ptr addrspace(1) %996, align 8
  %1125 = icmp sgt i32 %1009, 0
  br i1 %1125, label %1126, label %1145

1126:                                             ; preds = %1122
  %1127 = load i64, ptr addrspace(1) %997, align 8
  %1128 = tail call noundef i64 @llvm.smax.i64(i64 %1127, i64 1)
  br label %1129

1129:                                             ; preds = %1129, %1126
  %1130 = phi i32 [ 0, %1126 ], [ %1143, %1129 ]
  %1131 = phi i64 [ 0, %1126 ], [ %1142, %1129 ]
  %1132 = zext nneg i32 %1130 to i64
  %1133 = lshr i64 %984, %1132
  %1134 = tail call noundef i64 @llvm.smax.i64(i64 %1133, i64 1)
  %1135 = mul nsw i64 %1134, %1128
  %1136 = lshr i64 %1018, %1132
  %1137 = tail call noundef i64 @llvm.smax.i64(i64 %1136, i64 1)
  %1138 = mul nsw i64 %1135, %1137
  %1139 = lshr i64 %1022, %1132
  %1140 = tail call noundef i64 @llvm.smax.i64(i64 %1139, i64 1)
  %1141 = mul nsw i64 %1138, %1140
  %1142 = add nuw nsw i64 %1141, %1131
  %1143 = add nuw nsw i32 %1130, 1
  %1144 = icmp eq i32 %1143, %1009
  br i1 %1144, label %1145, label %1129, !llvm.loop !10026

1145:                                             ; preds = %1129, %1122
  %1146 = phi i64 [ 0, %1122 ], [ %1142, %1129 ]
  %1147 = icmp eq i64 %1124, 0
  br i1 %1147, label %1160, label %1148

1148:                                             ; preds = %1145
  %1149 = load i64, ptr addrspace(1) %985, align 8
  %1150 = mul nsw i64 %1024, %11
  %1151 = add nsw i64 %1113, %1150
  %1152 = mul nsw i64 %1151, %1020
  %1153 = add nsw i64 %1152, %1083
  %1154 = mul nsw i64 %1124, %1153
  %1155 = mul nsw i64 %1118, %1053
  %1156 = zext nneg i32 %1123 to i64
  %1157 = mul nsw i64 %1155, %1156
  %1158 = add nsw i64 %1154, %1157
  %1159 = add i64 %1158, %1149
  br label %1173

1160:                                             ; preds = %1145
  %1161 = mul nsw i64 %1024, %11
  %1162 = add nsw i64 %1113, %1161
  %1163 = mul nsw i64 %1162, %1020
  %1164 = add nsw i64 %1163, %1083
  %1165 = mul nsw i64 %1164, %1015
  %1166 = add i64 %1165, %1053
  %1167 = add i64 %1166, %1146
  %1168 = load i64, ptr addrspace(1) %985, align 8
  %1169 = zext nneg i32 %1123 to i64
  %1170 = mul i64 %1118, %1169
  %1171 = mul i64 %1170, %1167
  %1172 = add i64 %1168, %1171
  br label %1173

1173:                                             ; preds = %1160, %1148
  %1174 = phi i64 [ %1159, %1148 ], [ %1172, %1160 ]
  %1175 = icmp eq i64 %1119, 5
  %1176 = sext i1 %1175 to i32
  br label %1177

1177:                                             ; preds = %1330, %1173
  %1178 = phi i32 [ 0, %1173 ], [ %1333, %1330 ]
  %1179 = phi <4 x double> [ zeroinitializer, %1173 ], [ %1332, %1330 ]
  %1180 = zext nneg i32 %1178 to i64
  %1181 = icmp sgt i64 %1118, %1180
  br i1 %1181, label %1182, label %1327

1182:                                             ; preds = %1177
  %1183 = mul nuw nsw i32 %1178, %1123
  %1184 = zext nneg i32 %1183 to i64
  %1185 = add i64 %1174, %1184
  switch i64 %1119, label %1187 [
    i64 0, label %1188
    i64 20, label %1188
    i64 1, label %1186
    i64 21, label %1186
    i64 6, label %1186
  ]

1186:                                             ; preds = %1182, %1182, %1182
  br label %1188

1187:                                             ; preds = %1182
  br label %1188

1188:                                             ; preds = %1187, %1186, %1182, %1182
  %1189 = phi i32 [ 4, %1187 ], [ 2, %1186 ], [ 1, %1182 ], [ 1, %1182 ]
  %1190 = inttoptr i64 %1185 to ptr addrspace(1)
  br label %1192

1191:                                             ; preds = %1192
  switch i64 %1119, label %1295 [
    i64 6, label %1204
    i64 7, label %1291
    i64 0, label %1296
    i64 20, label %1296
    i64 1, label %1294
    i64 21, label %1294
  ]

1192:                                             ; preds = %1192, %1188
  %1193 = phi i32 [ 0, %1188 ], [ %1201, %1192 ]
  %1194 = phi i32 [ 0, %1188 ], [ %1202, %1192 ]
  %1195 = zext nneg i32 %1194 to i64
  %1196 = getelementptr inbounds i8, ptr addrspace(1) %1190, i64 %1195
  %1197 = load i8, ptr addrspace(1) %1196, align 1
  %1198 = zext i8 %1197 to i32
  %1199 = shl nsw i32 %1194, 3
  %1200 = shl i32 %1198, %1199
  %1201 = or i32 %1200, %1193
  %1202 = add nuw nsw i32 %1194, 1
  %1203 = icmp eq i32 %1202, %1189
  br i1 %1203, label %1191, label %1192, !llvm.loop !10016

1204:                                             ; preds = %1191
  %1205 = lshr i32 %1201, 10
  %1206 = and i32 %1205, 31
  %1207 = and i32 %1201, 1023
  switch i32 %1206, label %1214 [
    i32 0, label %1208
    i32 31, label %1211
  ]

1208:                                             ; preds = %1204
  %1209 = uitofp i32 %1207 to double
  %1210 = fmul double %1209, 0x3E70000000000000
  br label %1286

1211:                                             ; preds = %1204
  %1212 = icmp eq i32 %1207, 0
  %1213 = select i1 %1212, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %1286

1214:                                             ; preds = %1204
  %1215 = or disjoint i32 %1207, 1024
  %1216 = uitofp i32 %1215 to double
  %1217 = add nsw i32 %1206, -25
  %1218 = icmp ugt i32 %1206, 25
  br i1 %1218, label %1219, label %1236

1219:                                             ; preds = %1214
  %1220 = add nsw i32 %1206, -26
  %1221 = and i32 %1217, 7
  %1222 = icmp eq i32 %1221, 0
  br i1 %1222, label %1231, label %1223

1223:                                             ; preds = %1219, %1223
  %1224 = phi i32 [ %1228, %1223 ], [ %1217, %1219 ]
  %1225 = phi double [ %1227, %1223 ], [ %1216, %1219 ]
  %1226 = phi i32 [ %1229, %1223 ], [ 0, %1219 ]
  %1227 = fmul double %1225, 2.000000e+00
  %1228 = add nsw i32 %1224, -1
  %1229 = add i32 %1226, 1
  %1230 = icmp eq i32 %1229, %1221
  br i1 %1230, label %1231, label %1223, !llvm.loop !10042

1231:                                             ; preds = %1223, %1219
  %1232 = phi i32 [ %1217, %1219 ], [ %1228, %1223 ]
  %1233 = phi double [ %1216, %1219 ], [ %1227, %1223 ]
  %1234 = phi double [ undef, %1219 ], [ %1227, %1223 ]
  %1235 = icmp ult i32 %1220, 7
  br i1 %1235, label %1236, label %1259

1236:                                             ; preds = %1231, %1259, %1214
  %1237 = phi double [ %1216, %1214 ], [ %1234, %1231 ], [ %1269, %1259 ]
  %1238 = phi i32 [ %1217, %1214 ], [ 0, %1259 ], [ 0, %1231 ]
  %1239 = icmp slt i32 %1238, 0
  br i1 %1239, label %1240, label %1286

1240:                                             ; preds = %1236
  %1241 = tail call i32 @llvm.umin.i32(i32 %1206, i32 25)
  %1242 = sub nsw i32 1, %1241
  %1243 = and i32 %1242, 7
  %1244 = icmp eq i32 %1243, 0
  br i1 %1244, label %1253, label %1245

1245:                                             ; preds = %1240, %1245
  %1246 = phi i32 [ %1250, %1245 ], [ %1238, %1240 ]
  %1247 = phi double [ %1249, %1245 ], [ %1237, %1240 ]
  %1248 = phi i32 [ %1251, %1245 ], [ 0, %1240 ]
  %1249 = fmul double %1247, 5.000000e-01
  %1250 = add nsw i32 %1246, 1
  %1251 = add i32 %1248, 1
  %1252 = icmp eq i32 %1251, %1243
  br i1 %1252, label %1253, label %1245, !llvm.loop !10043

1253:                                             ; preds = %1245, %1240
  %1254 = phi double [ undef, %1240 ], [ %1249, %1245 ]
  %1255 = phi i32 [ %1238, %1240 ], [ %1250, %1245 ]
  %1256 = phi double [ %1237, %1240 ], [ %1249, %1245 ]
  %1257 = add nsw i32 %1241, -18
  %1258 = icmp ult i32 %1257, 7
  br i1 %1258, label %1286, label %1273

1259:                                             ; preds = %1231, %1259
  %1260 = phi i32 [ %1270, %1259 ], [ %1232, %1231 ]
  %1261 = phi double [ %1269, %1259 ], [ %1233, %1231 ]
  %1262 = fmul double %1261, 2.000000e+00
  %1263 = fmul double %1262, 2.000000e+00
  %1264 = fmul double %1263, 2.000000e+00
  %1265 = fmul double %1264, 2.000000e+00
  %1266 = fmul double %1265, 2.000000e+00
  %1267 = fmul double %1266, 2.000000e+00
  %1268 = fmul double %1267, 2.000000e+00
  %1269 = fmul double %1268, 2.000000e+00
  %1270 = add nsw i32 %1260, -8
  %1271 = add i32 %1260, -9
  %1272 = icmp ult i32 %1271, -2
  br i1 %1272, label %1259, label %1236, !llvm.loop !10021

1273:                                             ; preds = %1253, %1273
  %1274 = phi i32 [ %1284, %1273 ], [ %1255, %1253 ]
  %1275 = phi double [ %1283, %1273 ], [ %1256, %1253 ]
  %1276 = fmul double %1275, 5.000000e-01
  %1277 = fmul double %1276, 5.000000e-01
  %1278 = fmul double %1277, 5.000000e-01
  %1279 = fmul double %1278, 5.000000e-01
  %1280 = fmul double %1279, 5.000000e-01
  %1281 = fmul double %1280, 5.000000e-01
  %1282 = fmul double %1281, 5.000000e-01
  %1283 = fmul double %1282, 5.000000e-01
  %1284 = add nsw i32 %1274, 8
  %1285 = icmp eq i32 %1284, 0
  br i1 %1285, label %1286, label %1273, !llvm.loop !10022

1286:                                             ; preds = %1253, %1273, %1236, %1211, %1208
  %1287 = phi double [ %1210, %1208 ], [ %1213, %1211 ], [ %1237, %1236 ], [ %1254, %1253 ], [ %1283, %1273 ]
  %1288 = icmp ult i32 %1201, 32768
  %1289 = fneg double %1287
  %1290 = select i1 %1288, double %1287, double %1289
  br label %1330

1291:                                             ; preds = %1191
  %1292 = bitcast i32 %1201 to float
  %1293 = fpext float %1292 to double
  br label %1330

1294:                                             ; preds = %1191, %1191
  br label %1296

1295:                                             ; preds = %1191
  br label %1296

1296:                                             ; preds = %1295, %1294, %1191, %1191
  %1297 = phi i32 [ 32, %1295 ], [ 16, %1294 ], [ 8, %1191 ], [ 8, %1191 ]
  %1298 = zext i32 %1201 to i64
  switch i64 %1119, label %1308 [
    i64 21, label %1299
    i64 20, label %1299
    i64 3, label %1299
    i64 5, label %1299
  ]

1299:                                             ; preds = %1296, %1296, %1296, %1296
  %1300 = add nsw i32 %1297, -1
  %1301 = shl nuw i32 1, %1300
  %1302 = and i32 %1301, %1201
  %1303 = icmp eq i32 %1302, 0
  %1304 = zext nneg i32 %1297 to i64
  %1305 = shl nsw i64 -1, %1304
  %1306 = select i1 %1303, i64 0, i64 %1305
  %1307 = add nsw i64 %1306, %1298
  br label %1308

1308:                                             ; preds = %1299, %1296
  %1309 = phi i64 [ %1307, %1299 ], [ %1298, %1296 ]
  %1310 = load i64, ptr addrspace(1) %998, align 8
  %1311 = icmp eq i64 %1310, 0
  br i1 %1311, label %1312, label %1314

1312:                                             ; preds = %1308
  %1313 = sitofp i64 %1309 to double
  br label %1330

1314:                                             ; preds = %1308
  switch i64 %1119, label %1315 [
    i64 21, label %1316
    i64 20, label %1316
    i64 3, label %1316
  ]

1315:                                             ; preds = %1314
  br label %1316

1316:                                             ; preds = %1315, %1314, %1314, %1314
  %1317 = phi i32 [ -1, %1314 ], [ %1176, %1315 ], [ -1, %1314 ], [ -1, %1314 ]
  %1318 = add nsw i32 %1317, %1297
  %1319 = zext nneg i32 %1318 to i64
  %1320 = shl nsw i64 -1, %1319
  %1321 = xor i64 %1320, -1
  %1322 = sitofp i64 %1309 to double
  %1323 = sitofp i64 %1321 to double
  %1324 = fdiv double %1322, %1323
  %1325 = fcmp oge double %1324, -1.000000e+00
  %1326 = select i1 %1325, double %1324, double -1.000000e+00
  br label %1330

1327:                                             ; preds = %1177
  %1328 = icmp eq i32 %1178, 3
  %1329 = select i1 %1328, double 1.000000e+00, double 0.000000e+00
  br label %1330

1330:                                             ; preds = %1327, %1316, %1312, %1291, %1286
  %1331 = phi double [ %1329, %1327 ], [ %1290, %1286 ], [ %1293, %1291 ], [ %1326, %1316 ], [ %1313, %1312 ]
  %1332 = insertelement <4 x double> %1179, double %1331, i32 %1178
  %1333 = add nuw nsw i32 %1178, 1
  %1334 = icmp eq i32 %1333, 4
  br i1 %1334, label %1335, label %1177, !llvm.loop !10029

1335:                                             ; preds = %1330, %1112
  %1336 = phi <4 x double> [ zeroinitializer, %1112 ], [ %1332, %1330 ]
  %1337 = extractelement <4 x double> %1336, i64 0
  %1338 = fcmp oge double %1337, %1000
  %1339 = select i1 %1338, double 1.000000e+00, double 0.000000e+00
  %1340 = insertelement <4 x double> <double poison, double 0.000000e+00, double 0.000000e+00, double 1.000000e+00>, double %1339, i64 0
  %1341 = select i1 %999, <4 x double> %1336, <4 x double> %1340
  br label %1852

1342:                                             ; preds = %1003
  %1343 = sitofp i64 %1015 to double
  %1344 = select i1 %989, double %1343, double 1.000000e+00
  %1345 = fmul double %1344, %8
  %1346 = fadd double %1345, -5.000000e-01
  %1347 = tail call double @llvm.floor.f64(double %1346)
  %1348 = fptosi double %1347 to i64
  %1349 = add i64 %1348, %990
  switch i32 %6, label %1367 [
    i32 0, label %1350
    i32 2, label %1354
    i32 3, label %1358
  ]

1350:                                             ; preds = %1342
  %1351 = srem i64 %1349, %1015
  %1352 = add nsw i64 %1351, %1015
  %1353 = srem i64 %1352, %1015
  br label %1371

1354:                                             ; preds = %1342
  %1355 = icmp slt i64 %1349, %1015
  %1356 = tail call i64 @llvm.smax.i64(i64 %1349, i64 -1)
  %1357 = select i1 %1355, i64 %1356, i64 -1
  br label %1371

1358:                                             ; preds = %1342
  %1359 = shl nuw nsw i64 %1015, 1
  %1360 = srem i64 %1349, %1359
  %1361 = add nsw i64 %1360, %1359
  %1362 = srem i64 %1361, %1359
  %1363 = icmp slt i64 %1362, %1015
  %1364 = xor i64 %1362, -1
  %1365 = add i64 %1359, %1364
  %1366 = select i1 %1363, i64 %1362, i64 %1365
  br label %1371

1367:                                             ; preds = %1342
  %1368 = tail call noundef i64 @llvm.smax.i64(i64 %1349, i64 0)
  %1369 = add nsw i64 %1015, -1
  %1370 = tail call noundef i64 @llvm.smin.i64(i64 %1368, i64 %1369)
  br label %1371

1371:                                             ; preds = %1367, %1358, %1354, %1350
  %1372 = phi i64 [ %1370, %1367 ], [ %1366, %1358 ], [ %1357, %1354 ], [ %1353, %1350 ]
  %1373 = add i64 %1349, 1
  switch i32 %6, label %1391 [
    i32 0, label %1374
    i32 2, label %1378
    i32 3, label %1382
  ]

1374:                                             ; preds = %1371
  %1375 = srem i64 %1373, %1015
  %1376 = add nsw i64 %1375, %1015
  %1377 = srem i64 %1376, %1015
  br label %1395

1378:                                             ; preds = %1371
  %1379 = icmp slt i64 %1373, %1015
  %1380 = tail call i64 @llvm.smax.i64(i64 %1373, i64 -1)
  %1381 = select i1 %1379, i64 %1380, i64 -1
  br label %1395

1382:                                             ; preds = %1371
  %1383 = shl nuw nsw i64 %1015, 1
  %1384 = srem i64 %1373, %1383
  %1385 = add nsw i64 %1384, %1383
  %1386 = srem i64 %1385, %1383
  %1387 = icmp slt i64 %1386, %1015
  %1388 = xor i64 %1386, -1
  %1389 = add i64 %1383, %1388
  %1390 = select i1 %1387, i64 %1386, i64 %1389
  br label %1395

1391:                                             ; preds = %1371
  %1392 = tail call noundef i64 @llvm.smax.i64(i64 %1373, i64 0)
  %1393 = add nsw i64 %1015, -1
  %1394 = tail call noundef i64 @llvm.smin.i64(i64 %1392, i64 %1393)
  br label %1395

1395:                                             ; preds = %1391, %1382, %1378, %1374
  %1396 = phi i64 [ %1394, %1391 ], [ %1390, %1382 ], [ %1381, %1378 ], [ %1377, %1374 ]
  br i1 %991, label %1397, label %1430

1397:                                             ; preds = %1395
  %1398 = load i64, ptr addrspace(1) %986, align 8
  %1399 = tail call noundef i64 @llvm.smax.i64(i64 %1398, i64 1)
  %1400 = lshr i64 %1399, %1013
  %1401 = tail call noundef i64 @llvm.smax.i64(i64 %1400, i64 1)
  %1402 = sitofp i64 %1401 to double
  %1403 = select i1 %989, double %1402, double 1.000000e+00
  %1404 = fmul double %1403, %9
  %1405 = fadd double %1404, -5.000000e-01
  %1406 = tail call double @llvm.floor.f64(double %1405)
  %1407 = fptosi double %1406 to i64
  %1408 = add i64 %1407, %992
  switch i32 %6, label %1426 [
    i32 0, label %1409
    i32 2, label %1413
    i32 3, label %1417
  ]

1409:                                             ; preds = %1397
  %1410 = srem i64 %1408, %1401
  %1411 = add nsw i64 %1410, %1401
  %1412 = srem i64 %1411, %1401
  br label %1430

1413:                                             ; preds = %1397
  %1414 = icmp slt i64 %1408, %1401
  %1415 = tail call i64 @llvm.smax.i64(i64 %1408, i64 -1)
  %1416 = select i1 %1414, i64 %1415, i64 -1
  br label %1430

1417:                                             ; preds = %1397
  %1418 = shl nuw nsw i64 %1401, 1
  %1419 = srem i64 %1408, %1418
  %1420 = add nsw i64 %1419, %1418
  %1421 = srem i64 %1420, %1418
  %1422 = icmp slt i64 %1421, %1401
  %1423 = xor i64 %1421, -1
  %1424 = add i64 %1418, %1423
  %1425 = select i1 %1422, i64 %1421, i64 %1424
  br label %1430

1426:                                             ; preds = %1397
  %1427 = tail call noundef i64 @llvm.smax.i64(i64 %1408, i64 0)
  %1428 = add nsw i64 %1401, -1
  %1429 = tail call noundef i64 @llvm.smin.i64(i64 %1427, i64 %1428)
  br label %1430

1430:                                             ; preds = %1426, %1417, %1413, %1409, %1395
  %1431 = phi i64 [ 0, %1395 ], [ %1429, %1426 ], [ %1425, %1417 ], [ %1416, %1413 ], [ %1412, %1409 ]
  br i1 %991, label %1432, label %1465

1432:                                             ; preds = %1430
  %1433 = load i64, ptr addrspace(1) %986, align 8
  %1434 = tail call noundef i64 @llvm.smax.i64(i64 %1433, i64 1)
  %1435 = lshr i64 %1434, %1013
  %1436 = tail call noundef i64 @llvm.smax.i64(i64 %1435, i64 1)
  %1437 = sitofp i64 %1436 to double
  %1438 = select i1 %989, double %1437, double 1.000000e+00
  %1439 = fmul double %1438, %9
  %1440 = fadd double %1439, -5.000000e-01
  %1441 = tail call double @llvm.floor.f64(double %1440)
  %1442 = fptosi double %1441 to i64
  %1443 = add i64 %1001, %1442
  switch i32 %6, label %1461 [
    i32 0, label %1444
    i32 2, label %1448
    i32 3, label %1452
  ]

1444:                                             ; preds = %1432
  %1445 = srem i64 %1443, %1436
  %1446 = add nsw i64 %1445, %1436
  %1447 = srem i64 %1446, %1436
  br label %1465

1448:                                             ; preds = %1432
  %1449 = icmp slt i64 %1443, %1436
  %1450 = tail call i64 @llvm.smax.i64(i64 %1443, i64 -1)
  %1451 = select i1 %1449, i64 %1450, i64 -1
  br label %1465

1452:                                             ; preds = %1432
  %1453 = shl nuw nsw i64 %1436, 1
  %1454 = srem i64 %1443, %1453
  %1455 = add nsw i64 %1454, %1453
  %1456 = srem i64 %1455, %1453
  %1457 = icmp slt i64 %1456, %1436
  %1458 = xor i64 %1456, -1
  %1459 = add i64 %1453, %1458
  %1460 = select i1 %1457, i64 %1456, i64 %1459
  br label %1465

1461:                                             ; preds = %1432
  %1462 = tail call noundef i64 @llvm.smax.i64(i64 %1443, i64 0)
  %1463 = add nsw i64 %1436, -1
  %1464 = tail call noundef i64 @llvm.smin.i64(i64 %1462, i64 %1463)
  br label %1465

1465:                                             ; preds = %1461, %1452, %1448, %1444, %1430
  %1466 = phi i64 [ 0, %1430 ], [ %1464, %1461 ], [ %1460, %1452 ], [ %1451, %1448 ], [ %1447, %1444 ]
  br i1 %993, label %1467, label %1500

1467:                                             ; preds = %1465
  %1468 = load i64, ptr addrspace(1) %988, align 8
  %1469 = tail call noundef i64 @llvm.smax.i64(i64 %1468, i64 1)
  %1470 = lshr i64 %1469, %1013
  %1471 = tail call noundef i64 @llvm.smax.i64(i64 %1470, i64 1)
  %1472 = sitofp i64 %1471 to double
  %1473 = select i1 %989, double %1472, double 1.000000e+00
  %1474 = fmul double %1473, %10
  %1475 = fadd double %1474, -5.000000e-01
  %1476 = tail call double @llvm.floor.f64(double %1475)
  %1477 = fptosi double %1476 to i64
  %1478 = add i64 %1477, %994
  switch i32 %6, label %1496 [
    i32 0, label %1479
    i32 2, label %1483
    i32 3, label %1487
  ]

1479:                                             ; preds = %1467
  %1480 = srem i64 %1478, %1471
  %1481 = add nsw i64 %1480, %1471
  %1482 = srem i64 %1481, %1471
  br label %1500

1483:                                             ; preds = %1467
  %1484 = icmp slt i64 %1478, %1471
  %1485 = tail call i64 @llvm.smax.i64(i64 %1478, i64 -1)
  %1486 = select i1 %1484, i64 %1485, i64 -1
  br label %1500

1487:                                             ; preds = %1467
  %1488 = shl nuw nsw i64 %1471, 1
  %1489 = srem i64 %1478, %1488
  %1490 = add nsw i64 %1489, %1488
  %1491 = srem i64 %1490, %1488
  %1492 = icmp slt i64 %1491, %1471
  %1493 = xor i64 %1491, -1
  %1494 = add i64 %1488, %1493
  %1495 = select i1 %1492, i64 %1491, i64 %1494
  br label %1500

1496:                                             ; preds = %1467
  %1497 = tail call noundef i64 @llvm.smax.i64(i64 %1478, i64 0)
  %1498 = add nsw i64 %1471, -1
  %1499 = tail call noundef i64 @llvm.smin.i64(i64 %1497, i64 %1498)
  br label %1500

1500:                                             ; preds = %1496, %1487, %1483, %1479, %1465
  %1501 = phi i64 [ 0, %1465 ], [ %1499, %1496 ], [ %1495, %1487 ], [ %1486, %1483 ], [ %1482, %1479 ]
  br i1 %993, label %1502, label %1535

1502:                                             ; preds = %1500
  %1503 = load i64, ptr addrspace(1) %988, align 8
  %1504 = tail call noundef i64 @llvm.smax.i64(i64 %1503, i64 1)
  %1505 = lshr i64 %1504, %1013
  %1506 = tail call noundef i64 @llvm.smax.i64(i64 %1505, i64 1)
  %1507 = sitofp i64 %1506 to double
  %1508 = select i1 %989, double %1507, double 1.000000e+00
  %1509 = fmul double %1508, %10
  %1510 = fadd double %1509, -5.000000e-01
  %1511 = tail call double @llvm.floor.f64(double %1510)
  %1512 = fptosi double %1511 to i64
  %1513 = add i64 %1002, %1512
  switch i32 %6, label %1531 [
    i32 0, label %1514
    i32 2, label %1518
    i32 3, label %1522
  ]

1514:                                             ; preds = %1502
  %1515 = srem i64 %1513, %1506
  %1516 = add nsw i64 %1515, %1506
  %1517 = srem i64 %1516, %1506
  br label %1535

1518:                                             ; preds = %1502
  %1519 = icmp slt i64 %1513, %1506
  %1520 = tail call i64 @llvm.smax.i64(i64 %1513, i64 -1)
  %1521 = select i1 %1519, i64 %1520, i64 -1
  br label %1535

1522:                                             ; preds = %1502
  %1523 = shl nuw nsw i64 %1506, 1
  %1524 = srem i64 %1513, %1523
  %1525 = add nsw i64 %1524, %1523
  %1526 = srem i64 %1525, %1523
  %1527 = icmp slt i64 %1526, %1506
  %1528 = xor i64 %1526, -1
  %1529 = add i64 %1523, %1528
  %1530 = select i1 %1527, i64 %1526, i64 %1529
  br label %1535

1531:                                             ; preds = %1502
  %1532 = tail call noundef i64 @llvm.smax.i64(i64 %1513, i64 0)
  %1533 = add nsw i64 %1506, -1
  %1534 = tail call noundef i64 @llvm.smin.i64(i64 %1532, i64 %1533)
  br label %1535

1535:                                             ; preds = %1531, %1522, %1518, %1514, %1500
  %1536 = phi i64 [ 0, %1500 ], [ %1534, %1531 ], [ %1530, %1522 ], [ %1521, %1518 ], [ %1517, %1514 ]
  %1537 = fsub double %1346, %1347
  %1538 = fmul double %1537, 2.560000e+02
  %1539 = fadd double %1538, 5.000000e-01
  %1540 = tail call double @llvm.floor.f64(double %1539)
  %1541 = fptosi double %1540 to i64
  %1542 = sitofp i64 %1541 to double
  %1543 = fmul double %1542, 3.906250e-03
  br i1 %991, label %1544, label %1561

1544:                                             ; preds = %1535
  %1545 = load i64, ptr addrspace(1) %986, align 8
  %1546 = tail call noundef i64 @llvm.smax.i64(i64 %1545, i64 1)
  %1547 = lshr i64 %1546, %1013
  %1548 = tail call noundef i64 @llvm.smax.i64(i64 %1547, i64 1)
  %1549 = sitofp i64 %1548 to double
  %1550 = select i1 %989, double %1549, double 1.000000e+00
  %1551 = fmul double %1550, %9
  %1552 = fadd double %1551, -5.000000e-01
  %1553 = tail call double @llvm.floor.f64(double %1552)
  %1554 = fsub double %1552, %1553
  %1555 = fmul double %1554, 2.560000e+02
  %1556 = fadd double %1555, 5.000000e-01
  %1557 = tail call double @llvm.floor.f64(double %1556)
  %1558 = fptosi double %1557 to i64
  %1559 = sitofp i64 %1558 to double
  %1560 = fmul double %1559, 3.906250e-03
  br label %1561

1561:                                             ; preds = %1544, %1535
  %1562 = phi double [ %1560, %1544 ], [ 0.000000e+00, %1535 ]
  br i1 %993, label %1563, label %1580

1563:                                             ; preds = %1561
  %1564 = load i64, ptr addrspace(1) %988, align 8
  %1565 = tail call noundef i64 @llvm.smax.i64(i64 %1564, i64 1)
  %1566 = lshr i64 %1565, %1013
  %1567 = tail call noundef i64 @llvm.smax.i64(i64 %1566, i64 1)
  %1568 = sitofp i64 %1567 to double
  %1569 = select i1 %989, double %1568, double 1.000000e+00
  %1570 = fmul double %1569, %10
  %1571 = fadd double %1570, -5.000000e-01
  %1572 = tail call double @llvm.floor.f64(double %1571)
  %1573 = fsub double %1571, %1572
  %1574 = fmul double %1573, 2.560000e+02
  %1575 = fadd double %1574, 5.000000e-01
  %1576 = tail call double @llvm.floor.f64(double %1575)
  %1577 = fptosi double %1576 to i64
  %1578 = sitofp i64 %1577 to double
  %1579 = fmul double %1578, 3.906250e-03
  br label %1580

1580:                                             ; preds = %1563, %1561
  %1581 = phi double [ %1579, %1563 ], [ 0.000000e+00, %1561 ]
  %1582 = fsub double 1.000000e+00, %1543
  %1583 = fsub double 1.000000e+00, %1562
  %1584 = fsub double 1.000000e+00, %1581
  %1585 = icmp sgt i32 %1009, 0
  br label %1586

1586:                                             ; preds = %1840, %1580
  %1587 = phi <4 x double> [ zeroinitializer, %1580 ], [ %1848, %1840 ]
  %1588 = phi i32 [ 0, %1580 ], [ %1849, %1840 ]
  %1589 = and i32 %1588, 1
  %1590 = icmp eq i32 %1589, 0
  %1591 = select i1 %1590, double %1582, double %1543
  %1592 = and i32 %1588, 2
  %1593 = icmp eq i32 %1592, 0
  %1594 = select i1 %1593, double %1583, double %1562
  %1595 = fmul double %1591, %1594
  %1596 = and i32 %1588, 4
  %1597 = icmp eq i32 %1596, 0
  %1598 = select i1 %1597, double %1584, double %1581
  %1599 = fmul double %1598, %1595
  %1600 = insertelement <4 x double> poison, double %1599, i64 0
  %1601 = shufflevector <4 x double> %1600, <4 x double> poison, <4 x i32> zeroinitializer
  %1602 = select i1 %1590, i64 %1372, i64 %1396
  %1603 = select i1 %1593, i64 %1431, i64 %1466
  %1604 = select i1 %1597, i64 %1501, i64 %1536
  %1605 = or i64 %1603, %1602
  %1606 = or i64 %1605, %1604
  %1607 = icmp sgt i64 %1606, -1
  br i1 %1607, label %1608, label %1840

1608:                                             ; preds = %1586
  %1609 = load i64, ptr addrspace(1) %995, align 8
  %1610 = load i64, ptr addrspace(1) %987, align 8
  switch i64 %1610, label %1612 [
    i64 0, label %1613
    i64 20, label %1613
    i64 1, label %1611
    i64 21, label %1611
    i64 6, label %1611
  ]

1611:                                             ; preds = %1608, %1608, %1608
  br label %1613

1612:                                             ; preds = %1608
  br label %1613

1613:                                             ; preds = %1612, %1611, %1608, %1608
  %1614 = phi i32 [ 4, %1612 ], [ 2, %1611 ], [ 1, %1608 ], [ 1, %1608 ]
  %1615 = load i64, ptr addrspace(1) %996, align 8
  br i1 %1585, label %1621, label %1616

1616:                                             ; preds = %1613
  %1617 = load i64, ptr addrspace(1) %988, align 8
  %1618 = load i64, ptr addrspace(1) %986, align 8
  %1619 = tail call noundef i64 @llvm.smax.i64(i64 %1617, i64 1)
  %1620 = tail call noundef i64 @llvm.smax.i64(i64 %1618, i64 1)
  br label %1644

1621:                                             ; preds = %1613
  %1622 = load i64, ptr addrspace(1) %997, align 8
  %1623 = tail call noundef i64 @llvm.smax.i64(i64 %1622, i64 1)
  %1624 = load i64, ptr addrspace(1) %986, align 8
  %1625 = tail call noundef i64 @llvm.smax.i64(i64 %1624, i64 1)
  %1626 = load i64, ptr addrspace(1) %988, align 8
  %1627 = tail call noundef i64 @llvm.smax.i64(i64 %1626, i64 1)
  br label %1628

1628:                                             ; preds = %1628, %1621
  %1629 = phi i32 [ 0, %1621 ], [ %1642, %1628 ]
  %1630 = phi i64 [ 0, %1621 ], [ %1641, %1628 ]
  %1631 = zext nneg i32 %1629 to i64
  %1632 = lshr i64 %984, %1631
  %1633 = tail call noundef i64 @llvm.smax.i64(i64 %1632, i64 1)
  %1634 = mul nsw i64 %1633, %1623
  %1635 = lshr i64 %1625, %1631
  %1636 = tail call noundef i64 @llvm.smax.i64(i64 %1635, i64 1)
  %1637 = mul nsw i64 %1634, %1636
  %1638 = lshr i64 %1627, %1631
  %1639 = tail call noundef i64 @llvm.smax.i64(i64 %1638, i64 1)
  %1640 = mul nsw i64 %1637, %1639
  %1641 = add nuw nsw i64 %1640, %1630
  %1642 = add nuw nsw i32 %1629, 1
  %1643 = icmp eq i32 %1642, %1009
  br i1 %1643, label %1644, label %1628, !llvm.loop !10026

1644:                                             ; preds = %1628, %1616
  %1645 = phi i64 [ %1620, %1616 ], [ %1625, %1628 ]
  %1646 = phi i64 [ %1619, %1616 ], [ %1627, %1628 ]
  %1647 = phi i64 [ 0, %1616 ], [ %1641, %1628 ]
  %1648 = lshr i64 %1646, %1013
  %1649 = tail call noundef i64 @llvm.smax.i64(i64 %1648, i64 1)
  %1650 = lshr i64 %1645, %1013
  %1651 = tail call noundef i64 @llvm.smax.i64(i64 %1650, i64 1)
  %1652 = icmp eq i64 %1615, 0
  br i1 %1652, label %1665, label %1653

1653:                                             ; preds = %1644
  %1654 = load i64, ptr addrspace(1) %985, align 8
  %1655 = mul nsw i64 %1649, %11
  %1656 = add nsw i64 %1655, %1604
  %1657 = mul nsw i64 %1656, %1651
  %1658 = add nsw i64 %1657, %1603
  %1659 = mul nsw i64 %1658, %1615
  %1660 = mul nsw i64 %1609, %1602
  %1661 = zext nneg i32 %1614 to i64
  %1662 = mul nsw i64 %1660, %1661
  %1663 = add nsw i64 %1659, %1662
  %1664 = add i64 %1663, %1654
  br label %1678

1665:                                             ; preds = %1644
  %1666 = mul nsw i64 %1649, %11
  %1667 = add nsw i64 %1666, %1604
  %1668 = mul nsw i64 %1667, %1651
  %1669 = add nsw i64 %1668, %1603
  %1670 = mul nsw i64 %1669, %1015
  %1671 = add i64 %1647, %1602
  %1672 = add i64 %1671, %1670
  %1673 = load i64, ptr addrspace(1) %985, align 8
  %1674 = zext nneg i32 %1614 to i64
  %1675 = mul i64 %1609, %1674
  %1676 = mul i64 %1675, %1672
  %1677 = add i64 %1673, %1676
  br label %1678

1678:                                             ; preds = %1665, %1653
  %1679 = phi i64 [ %1664, %1653 ], [ %1677, %1665 ]
  %1680 = icmp eq i64 %1610, 5
  %1681 = sext i1 %1680 to i32
  br label %1682

1682:                                             ; preds = %1835, %1678
  %1683 = phi i32 [ 0, %1678 ], [ %1838, %1835 ]
  %1684 = phi <4 x double> [ zeroinitializer, %1678 ], [ %1837, %1835 ]
  %1685 = zext nneg i32 %1683 to i64
  %1686 = icmp sgt i64 %1609, %1685
  br i1 %1686, label %1687, label %1832

1687:                                             ; preds = %1682
  %1688 = mul nuw nsw i32 %1683, %1614
  %1689 = zext nneg i32 %1688 to i64
  %1690 = add i64 %1679, %1689
  switch i64 %1610, label %1692 [
    i64 0, label %1693
    i64 20, label %1693
    i64 1, label %1691
    i64 21, label %1691
    i64 6, label %1691
  ]

1691:                                             ; preds = %1687, %1687, %1687
  br label %1693

1692:                                             ; preds = %1687
  br label %1693

1693:                                             ; preds = %1692, %1691, %1687, %1687
  %1694 = phi i32 [ 4, %1692 ], [ 2, %1691 ], [ 1, %1687 ], [ 1, %1687 ]
  %1695 = inttoptr i64 %1690 to ptr addrspace(1)
  br label %1697

1696:                                             ; preds = %1697
  switch i64 %1610, label %1800 [
    i64 6, label %1709
    i64 7, label %1796
    i64 0, label %1801
    i64 20, label %1801
    i64 1, label %1799
    i64 21, label %1799
  ]

1697:                                             ; preds = %1697, %1693
  %1698 = phi i32 [ 0, %1693 ], [ %1706, %1697 ]
  %1699 = phi i32 [ 0, %1693 ], [ %1707, %1697 ]
  %1700 = zext nneg i32 %1699 to i64
  %1701 = getelementptr inbounds i8, ptr addrspace(1) %1695, i64 %1700
  %1702 = load i8, ptr addrspace(1) %1701, align 1
  %1703 = zext i8 %1702 to i32
  %1704 = shl nsw i32 %1699, 3
  %1705 = shl i32 %1703, %1704
  %1706 = or i32 %1705, %1698
  %1707 = add nuw nsw i32 %1699, 1
  %1708 = icmp eq i32 %1707, %1694
  br i1 %1708, label %1696, label %1697, !llvm.loop !10016

1709:                                             ; preds = %1696
  %1710 = lshr i32 %1706, 10
  %1711 = and i32 %1710, 31
  %1712 = and i32 %1706, 1023
  switch i32 %1711, label %1719 [
    i32 0, label %1713
    i32 31, label %1716
  ]

1713:                                             ; preds = %1709
  %1714 = uitofp i32 %1712 to double
  %1715 = fmul double %1714, 0x3E70000000000000
  br label %1791

1716:                                             ; preds = %1709
  %1717 = icmp eq i32 %1712, 0
  %1718 = select i1 %1717, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %1791

1719:                                             ; preds = %1709
  %1720 = or disjoint i32 %1712, 1024
  %1721 = uitofp i32 %1720 to double
  %1722 = add nsw i32 %1711, -25
  %1723 = icmp ugt i32 %1711, 25
  br i1 %1723, label %1724, label %1741

1724:                                             ; preds = %1719
  %1725 = add nsw i32 %1711, -26
  %1726 = and i32 %1722, 7
  %1727 = icmp eq i32 %1726, 0
  br i1 %1727, label %1736, label %1728

1728:                                             ; preds = %1724, %1728
  %1729 = phi i32 [ %1733, %1728 ], [ %1722, %1724 ]
  %1730 = phi double [ %1732, %1728 ], [ %1721, %1724 ]
  %1731 = phi i32 [ %1734, %1728 ], [ 0, %1724 ]
  %1732 = fmul double %1730, 2.000000e+00
  %1733 = add nsw i32 %1729, -1
  %1734 = add i32 %1731, 1
  %1735 = icmp eq i32 %1734, %1726
  br i1 %1735, label %1736, label %1728, !llvm.loop !10044

1736:                                             ; preds = %1728, %1724
  %1737 = phi i32 [ %1722, %1724 ], [ %1733, %1728 ]
  %1738 = phi double [ %1721, %1724 ], [ %1732, %1728 ]
  %1739 = phi double [ undef, %1724 ], [ %1732, %1728 ]
  %1740 = icmp ult i32 %1725, 7
  br i1 %1740, label %1741, label %1764

1741:                                             ; preds = %1736, %1764, %1719
  %1742 = phi double [ %1721, %1719 ], [ %1739, %1736 ], [ %1774, %1764 ]
  %1743 = phi i32 [ %1722, %1719 ], [ 0, %1764 ], [ 0, %1736 ]
  %1744 = icmp slt i32 %1743, 0
  br i1 %1744, label %1745, label %1791

1745:                                             ; preds = %1741
  %1746 = tail call i32 @llvm.umin.i32(i32 %1711, i32 25)
  %1747 = sub nsw i32 1, %1746
  %1748 = and i32 %1747, 7
  %1749 = icmp eq i32 %1748, 0
  br i1 %1749, label %1758, label %1750

1750:                                             ; preds = %1745, %1750
  %1751 = phi i32 [ %1755, %1750 ], [ %1743, %1745 ]
  %1752 = phi double [ %1754, %1750 ], [ %1742, %1745 ]
  %1753 = phi i32 [ %1756, %1750 ], [ 0, %1745 ]
  %1754 = fmul double %1752, 5.000000e-01
  %1755 = add nsw i32 %1751, 1
  %1756 = add i32 %1753, 1
  %1757 = icmp eq i32 %1756, %1748
  br i1 %1757, label %1758, label %1750, !llvm.loop !10045

1758:                                             ; preds = %1750, %1745
  %1759 = phi double [ undef, %1745 ], [ %1754, %1750 ]
  %1760 = phi i32 [ %1743, %1745 ], [ %1755, %1750 ]
  %1761 = phi double [ %1742, %1745 ], [ %1754, %1750 ]
  %1762 = add nsw i32 %1746, -18
  %1763 = icmp ult i32 %1762, 7
  br i1 %1763, label %1791, label %1778

1764:                                             ; preds = %1736, %1764
  %1765 = phi i32 [ %1775, %1764 ], [ %1737, %1736 ]
  %1766 = phi double [ %1774, %1764 ], [ %1738, %1736 ]
  %1767 = fmul double %1766, 2.000000e+00
  %1768 = fmul double %1767, 2.000000e+00
  %1769 = fmul double %1768, 2.000000e+00
  %1770 = fmul double %1769, 2.000000e+00
  %1771 = fmul double %1770, 2.000000e+00
  %1772 = fmul double %1771, 2.000000e+00
  %1773 = fmul double %1772, 2.000000e+00
  %1774 = fmul double %1773, 2.000000e+00
  %1775 = add nsw i32 %1765, -8
  %1776 = add i32 %1765, -9
  %1777 = icmp ult i32 %1776, -2
  br i1 %1777, label %1764, label %1741, !llvm.loop !10021

1778:                                             ; preds = %1758, %1778
  %1779 = phi i32 [ %1789, %1778 ], [ %1760, %1758 ]
  %1780 = phi double [ %1788, %1778 ], [ %1761, %1758 ]
  %1781 = fmul double %1780, 5.000000e-01
  %1782 = fmul double %1781, 5.000000e-01
  %1783 = fmul double %1782, 5.000000e-01
  %1784 = fmul double %1783, 5.000000e-01
  %1785 = fmul double %1784, 5.000000e-01
  %1786 = fmul double %1785, 5.000000e-01
  %1787 = fmul double %1786, 5.000000e-01
  %1788 = fmul double %1787, 5.000000e-01
  %1789 = add nsw i32 %1779, 8
  %1790 = icmp eq i32 %1789, 0
  br i1 %1790, label %1791, label %1778, !llvm.loop !10022

1791:                                             ; preds = %1758, %1778, %1741, %1716, %1713
  %1792 = phi double [ %1715, %1713 ], [ %1718, %1716 ], [ %1742, %1741 ], [ %1759, %1758 ], [ %1788, %1778 ]
  %1793 = icmp ult i32 %1706, 32768
  %1794 = fneg double %1792
  %1795 = select i1 %1793, double %1792, double %1794
  br label %1835

1796:                                             ; preds = %1696
  %1797 = bitcast i32 %1706 to float
  %1798 = fpext float %1797 to double
  br label %1835

1799:                                             ; preds = %1696, %1696
  br label %1801

1800:                                             ; preds = %1696
  br label %1801

1801:                                             ; preds = %1800, %1799, %1696, %1696
  %1802 = phi i32 [ 32, %1800 ], [ 16, %1799 ], [ 8, %1696 ], [ 8, %1696 ]
  %1803 = zext i32 %1706 to i64
  switch i64 %1610, label %1813 [
    i64 21, label %1804
    i64 20, label %1804
    i64 3, label %1804
    i64 5, label %1804
  ]

1804:                                             ; preds = %1801, %1801, %1801, %1801
  %1805 = add nsw i32 %1802, -1
  %1806 = shl nuw i32 1, %1805
  %1807 = and i32 %1806, %1706
  %1808 = icmp eq i32 %1807, 0
  %1809 = zext nneg i32 %1802 to i64
  %1810 = shl nsw i64 -1, %1809
  %1811 = select i1 %1808, i64 0, i64 %1810
  %1812 = add nsw i64 %1811, %1803
  br label %1813

1813:                                             ; preds = %1804, %1801
  %1814 = phi i64 [ %1812, %1804 ], [ %1803, %1801 ]
  %1815 = load i64, ptr addrspace(1) %998, align 8
  %1816 = icmp eq i64 %1815, 0
  br i1 %1816, label %1817, label %1819

1817:                                             ; preds = %1813
  %1818 = sitofp i64 %1814 to double
  br label %1835

1819:                                             ; preds = %1813
  switch i64 %1610, label %1820 [
    i64 21, label %1821
    i64 20, label %1821
    i64 3, label %1821
  ]

1820:                                             ; preds = %1819
  br label %1821

1821:                                             ; preds = %1820, %1819, %1819, %1819
  %1822 = phi i32 [ -1, %1819 ], [ %1681, %1820 ], [ -1, %1819 ], [ -1, %1819 ]
  %1823 = add nsw i32 %1822, %1802
  %1824 = zext nneg i32 %1823 to i64
  %1825 = shl nsw i64 -1, %1824
  %1826 = xor i64 %1825, -1
  %1827 = sitofp i64 %1814 to double
  %1828 = sitofp i64 %1826 to double
  %1829 = fdiv double %1827, %1828
  %1830 = fcmp oge double %1829, -1.000000e+00
  %1831 = select i1 %1830, double %1829, double -1.000000e+00
  br label %1835

1832:                                             ; preds = %1682
  %1833 = icmp eq i32 %1683, 3
  %1834 = select i1 %1833, double 1.000000e+00, double 0.000000e+00
  br label %1835

1835:                                             ; preds = %1832, %1821, %1817, %1796, %1791
  %1836 = phi double [ %1834, %1832 ], [ %1795, %1791 ], [ %1798, %1796 ], [ %1831, %1821 ], [ %1818, %1817 ]
  %1837 = insertelement <4 x double> %1684, double %1836, i32 %1683
  %1838 = add nuw nsw i32 %1683, 1
  %1839 = icmp eq i32 %1838, 4
  br i1 %1839, label %1840, label %1682, !llvm.loop !10029

1840:                                             ; preds = %1835, %1586
  %1841 = phi <4 x double> [ zeroinitializer, %1586 ], [ %1837, %1835 ]
  %1842 = extractelement <4 x double> %1841, i64 0
  %1843 = fcmp oge double %1842, %1000
  %1844 = select i1 %1843, double 1.000000e+00, double 0.000000e+00
  %1845 = insertelement <4 x double> <double poison, double 0.000000e+00, double 0.000000e+00, double 1.000000e+00>, double %1844, i64 0
  %1846 = select i1 %999, <4 x double> %1841, <4 x double> %1845
  %1847 = fmul <4 x double> %1601, %1846
  %1848 = fadd <4 x double> %1587, %1847
  %1849 = add nuw nsw i32 %1588, 1
  %1850 = lshr i32 %1849, %12
  %1851 = icmp eq i32 %1850, 0
  br i1 %1851, label %1586, label %1852, !llvm.loop !10032

1852:                                             ; preds = %1840, %1335
  %1853 = phi <4 x double> [ %1341, %1335 ], [ %1848, %1840 ]
  %1854 = fmul <4 x double> %1012, %1853
  %1855 = fadd <4 x double> %1006, %1854
  %1856 = and i1 %1004, %948
  br i1 %1856, label %1003, label %1857, !llvm.loop !10046

1857:                                             ; preds = %1852, %921, %368, %363
  %1858 = phi <4 x double> [ %375, %368 ], [ %364, %363 ], [ %924, %921 ], [ %1855, %1852 ]
  ret <4 x double> %1858
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define dso_local <4 x i32> @__zluda_ptx_impl_agx_tld4(ptr addrspace(1) nocapture noundef readonly %0, ptr addrspace(1) noundef readonly %1, i32 noundef %2, i32 noundef %3, i32 noundef %4, i32 noundef %5, i32 noundef %6, i32 noundef %7, i32 noundef %8, i32 noundef %9, i32 noundef %10, i32 noundef %11, i32 noundef %12) local_unnamed_addr #3 {
  %14 = alloca [4 x i32], align 4
  %15 = icmp eq ptr addrspace(1) %1, null
  br i1 %15, label %27, label %16

16:                                               ; preds = %13
  %17 = getelementptr inbounds i64, ptr addrspace(1) %1, i64 2
  %18 = load i64, ptr addrspace(1) %17, align 8
  %19 = getelementptr inbounds i64, ptr addrspace(1) %1, i64 3
  %20 = load i64, ptr addrspace(1) %19, align 8
  %21 = icmp eq i64 %20, 0
  br i1 %21, label %33, label %22

22:                                               ; preds = %16
  %23 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 6
  %24 = load i64, ptr addrspace(1) %23, align 8
  %25 = icmp ne i64 %24, 0
  %26 = zext i1 %25 to i32
  br label %33

27:                                               ; preds = %13
  %28 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 5
  %29 = load i64, ptr addrspace(1) %28, align 8
  %30 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 6
  %31 = load i64, ptr addrspace(1) %30, align 8
  %32 = trunc i64 %31 to i32
  br label %33

33:                                               ; preds = %16, %22, %27
  %34 = phi i32 [ %32, %27 ], [ 0, %16 ], [ %26, %22 ]
  %35 = phi i64 [ %29, %27 ], [ %18, %16 ], [ %18, %22 ]
  %36 = trunc i64 %35 to i32
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %14)
  store i32 %5, ptr %14, align 4
  %37 = getelementptr inbounds i32, ptr %14, i64 1
  store i32 %6, ptr %37, align 4
  %38 = getelementptr inbounds i32, ptr %14, i64 2
  store i32 %7, ptr %38, align 4
  %39 = getelementptr inbounds i32, ptr %14, i64 3
  store i32 %8, ptr %39, align 4
  %40 = add i32 %2, -7
  %41 = icmp ult i32 %40, -2
  br i1 %41, label %159, label %42

42:                                               ; preds = %33
  %43 = icmp eq i32 %2, 6
  %44 = zext i1 %43 to i64
  %45 = getelementptr inbounds [4 x i32], ptr %14, i64 0, i64 %44
  %46 = load float, ptr %45, align 4
  %47 = fpext float %46 to double
  %48 = select i1 %43, i64 2, i64 1
  %49 = getelementptr inbounds [4 x i32], ptr %14, i64 0, i64 %48
  %50 = load float, ptr %49, align 4
  %51 = fpext float %50 to double
  %52 = select i1 %43, i64 3, i64 2
  %53 = getelementptr inbounds [4 x i32], ptr %14, i64 0, i64 %52
  %54 = load float, ptr %53, align 4
  %55 = fpext float %54 to double
  %56 = tail call double @llvm.fabs.f64(double %47)
  %57 = tail call double @llvm.fabs.f64(double %51)
  %58 = tail call double @llvm.fabs.f64(double %55)
  %59 = fcmp ult double %56, %57
  %60 = fcmp ult double %56, %58
  %61 = or i1 %59, %60
  br i1 %61, label %65, label %62

62:                                               ; preds = %42
  %63 = fcmp ult float %46, 0.000000e+00
  %64 = zext i1 %63 to i32
  br label %75

65:                                               ; preds = %42
  %66 = fcmp ule double %57, %56
  %67 = fcmp ult double %57, %58
  %68 = or i1 %66, %67
  br i1 %68, label %72, label %69

69:                                               ; preds = %65
  %70 = fcmp oge float %50, 0.000000e+00
  %71 = select i1 %70, i32 2, i32 3
  br label %75

72:                                               ; preds = %65
  %73 = fcmp oge float %54, 0.000000e+00
  %74 = select i1 %73, i32 4, i32 5
  br label %75

75:                                               ; preds = %62, %69, %72
  %76 = phi i32 [ %64, %62 ], [ %71, %69 ], [ %74, %72 ]
  %77 = icmp ult i32 %76, 2
  %78 = icmp ult i32 %76, 4
  %79 = select i1 %78, double %57, double %58
  %80 = select i1 %77, double %56, double %79
  br i1 %61, label %84, label %81

81:                                               ; preds = %75
  %82 = fcmp ult float %46, 0.000000e+00
  %83 = zext i1 %82 to i32
  br label %94

84:                                               ; preds = %75
  %85 = fcmp ule double %57, %56
  %86 = fcmp ult double %57, %58
  %87 = or i1 %85, %86
  br i1 %87, label %91, label %88

88:                                               ; preds = %84
  %89 = fcmp oge float %50, 0.000000e+00
  %90 = select i1 %89, i32 2, i32 3
  br label %94

91:                                               ; preds = %84
  %92 = fcmp oge float %54, 0.000000e+00
  %93 = select i1 %92, i32 4, i32 5
  br label %94

94:                                               ; preds = %91, %88, %81
  %95 = phi i32 [ %83, %81 ], [ %90, %88 ], [ %93, %91 ]
  switch i32 %95, label %100 [
    i32 0, label %96
    i32 1, label %101
    i32 5, label %98
  ]

96:                                               ; preds = %94
  %97 = fneg double %55
  br label %101

98:                                               ; preds = %94
  %99 = fneg double %47
  br label %101

100:                                              ; preds = %94
  br label %101

101:                                              ; preds = %94, %96, %98, %100
  %102 = phi double [ %47, %100 ], [ %99, %98 ], [ %97, %96 ], [ %55, %94 ]
  %103 = fcmp ogt double %80, 0.000000e+00
  %104 = fdiv double %102, %80
  %105 = fadd double %104, 1.000000e+00
  %106 = fmul double %105, 5.000000e-01
  %107 = select i1 %103, double %106, double 5.000000e-01
  br i1 %61, label %111, label %108

108:                                              ; preds = %101
  %109 = fcmp ult float %46, 0.000000e+00
  %110 = zext i1 %109 to i32
  br label %121

111:                                              ; preds = %101
  %112 = fcmp ule double %57, %56
  %113 = fcmp ult double %57, %58
  %114 = or i1 %112, %113
  br i1 %114, label %118, label %115

115:                                              ; preds = %111
  %116 = fcmp oge float %50, 0.000000e+00
  %117 = select i1 %116, i32 2, i32 3
  br label %121

118:                                              ; preds = %111
  %119 = fcmp oge float %54, 0.000000e+00
  %120 = select i1 %119, i32 4, i32 5
  br label %121

121:                                              ; preds = %118, %115, %108
  %122 = phi i32 [ %110, %108 ], [ %117, %115 ], [ %120, %118 ]
  switch i32 %122, label %125 [
    i32 2, label %127
    i32 3, label %123
  ]

123:                                              ; preds = %121
  %124 = fneg double %55
  br label %127

125:                                              ; preds = %121
  %126 = fneg double %51
  br label %127

127:                                              ; preds = %121, %123, %125
  %128 = phi double [ %126, %125 ], [ %124, %123 ], [ %55, %121 ]
  %129 = fdiv double %128, %80
  %130 = fadd double %129, 1.000000e+00
  %131 = fmul double %130, 5.000000e-01
  %132 = select i1 %103, double %131, double 5.000000e-01
  %133 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 8
  %134 = load i64, ptr addrspace(1) %133, align 8
  br i1 %61, label %138, label %135

135:                                              ; preds = %127
  %136 = fcmp ult float %46, 0.000000e+00
  %137 = zext i1 %136 to i64
  br label %148

138:                                              ; preds = %127
  %139 = fcmp ule double %57, %56
  %140 = fcmp ult double %57, %58
  %141 = or i1 %139, %140
  br i1 %141, label %145, label %142

142:                                              ; preds = %138
  %143 = fcmp oge float %50, 0.000000e+00
  %144 = select i1 %143, i64 2, i64 3
  br label %148

145:                                              ; preds = %138
  %146 = fcmp oge float %54, 0.000000e+00
  %147 = select i1 %146, i64 4, i64 5
  br label %148

148:                                              ; preds = %135, %142, %145
  %149 = phi i64 [ %137, %135 ], [ %144, %142 ], [ %147, %145 ]
  %150 = select i1 %43, i32 %5, i32 0
  %151 = zext i32 %150 to i64
  %152 = tail call noundef i64 @llvm.smax.i64(i64 %134, i64 1)
  %153 = udiv i64 %152, 6
  %154 = tail call i64 @llvm.smax.i64(i64 %153, i64 1)
  %155 = add nsw i64 %154, -1
  %156 = tail call noundef i64 @llvm.smin.i64(i64 %151, i64 %155)
  %157 = mul nuw nsw i64 %156, 6
  %158 = add nuw nsw i64 %149, %157
  br label %179

159:                                              ; preds = %33
  %160 = add i32 %2, -3
  %161 = icmp ult i32 %160, 2
  %162 = zext i1 %161 to i64
  %163 = getelementptr inbounds [4 x i32], ptr %14, i64 0, i64 %162
  %164 = load float, ptr %163, align 4
  %165 = fpext float %164 to double
  %166 = select i1 %161, i64 2, i64 1
  %167 = getelementptr inbounds [4 x i32], ptr %14, i64 0, i64 %166
  %168 = load float, ptr %167, align 4
  %169 = fpext float %168 to double
  %170 = add i32 %2, -3
  %171 = icmp ult i32 %170, 2
  br i1 %171, label %172, label %179

172:                                              ; preds = %159
  %173 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 8
  %174 = zext i32 %5 to i64
  %175 = load i64, ptr addrspace(1) %173, align 8
  %176 = tail call noundef i64 @llvm.smax.i64(i64 %175, i64 1)
  %177 = add nsw i64 %176, -1
  %178 = tail call noundef i64 @llvm.smin.i64(i64 %174, i64 %177)
  br label %179

179:                                              ; preds = %159, %172, %148
  %180 = phi i32 [ 1, %148 ], [ %34, %159 ], [ %34, %172 ]
  %181 = phi i64 [ %158, %148 ], [ 0, %159 ], [ %178, %172 ]
  %182 = phi double [ %132, %148 ], [ %169, %159 ], [ %169, %172 ]
  %183 = phi double [ %107, %148 ], [ %165, %159 ], [ %165, %172 ]
  %184 = getelementptr i64, ptr addrspace(1) %0, i64 9
  %185 = load i64, ptr addrspace(1) %184, align 8
  %186 = tail call noundef i64 @llvm.smax.i64(i64 %185, i64 1)
  %187 = icmp eq i32 %180, 1
  %188 = sitofp i64 %186 to double
  %189 = select i1 %187, double %188, double 1.000000e+00
  %190 = fmul double %183, %189
  %191 = fadd double %190, -5.000000e-01
  %192 = tail call double @llvm.floor.f64(double %191)
  %193 = fptosi double %192 to i64
  %194 = sext i32 %9 to i64
  %195 = getelementptr i64, ptr addrspace(1) %0, i64 1
  %196 = getelementptr i64, ptr addrspace(1) %0, i64 10
  %197 = load i64, ptr addrspace(1) %196, align 8
  %198 = tail call noundef i64 @llvm.smax.i64(i64 %197, i64 1)
  %199 = sitofp i64 %198 to double
  %200 = select i1 %187, double %199, double 1.000000e+00
  %201 = fmul double %182, %200
  %202 = fadd double %201, -5.000000e-01
  %203 = tail call double @llvm.floor.f64(double %202)
  %204 = fptosi double %203 to i64
  %205 = sext i32 %10 to i64
  %206 = icmp eq i32 %11, 0
  %207 = bitcast i32 %12 to float
  %208 = fpext float %207 to double
  %209 = icmp eq i32 %4, 0
  %210 = shl nuw nsw i64 %186, 1
  %211 = add nsw i64 %186, -1
  %212 = shl nuw nsw i64 %198, 1
  %213 = add nsw i64 %198, -1
  %214 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %215 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 2
  %216 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 14
  %217 = getelementptr i64, ptr addrspace(1) %0, i64 11
  %218 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 7
  br label %220

219:                                              ; preds = %486
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %14)
  ret <4 x i32> %488

220:                                              ; preds = %179, %486
  %221 = phi i32 [ 0, %179 ], [ %489, %486 ]
  %222 = phi <4 x i32> [ undef, %179 ], [ %488, %486 ]
  %223 = add nsw i32 %221, -1
  %224 = icmp ult i32 %223, 2
  %225 = icmp ult i32 %221, 2
  %226 = zext i1 %224 to i64
  %227 = add nsw i64 %226, %194
  %228 = add i64 %227, %193
  switch i32 %36, label %245 [
    i32 0, label %229
    i32 2, label %233
    i32 3, label %237
  ]

229:                                              ; preds = %220
  %230 = srem i64 %228, %186
  %231 = add nsw i64 %230, %186
  %232 = srem i64 %231, %186
  br label %248

233:                                              ; preds = %220
  %234 = icmp slt i64 %228, %186
  %235 = tail call i64 @llvm.smax.i64(i64 %228, i64 -1)
  %236 = select i1 %234, i64 %235, i64 -1
  br label %248

237:                                              ; preds = %220
  %238 = srem i64 %228, %210
  %239 = add nsw i64 %238, %210
  %240 = srem i64 %239, %210
  %241 = icmp slt i64 %240, %186
  %242 = xor i64 %240, -1
  %243 = add i64 %210, %242
  %244 = select i1 %241, i64 %240, i64 %243
  br label %248

245:                                              ; preds = %220
  %246 = tail call noundef i64 @llvm.smax.i64(i64 %228, i64 0)
  %247 = tail call noundef i64 @llvm.smin.i64(i64 %246, i64 %211)
  br label %248

248:                                              ; preds = %229, %233, %237, %245
  %249 = phi i64 [ %247, %245 ], [ %244, %237 ], [ %236, %233 ], [ %232, %229 ]
  %250 = zext i1 %225 to i64
  %251 = add nsw i64 %250, %205
  %252 = add i64 %251, %204
  switch i32 %36, label %269 [
    i32 0, label %253
    i32 2, label %257
    i32 3, label %261
  ]

253:                                              ; preds = %248
  %254 = srem i64 %252, %198
  %255 = add nsw i64 %254, %198
  %256 = srem i64 %255, %198
  br label %272

257:                                              ; preds = %248
  %258 = icmp slt i64 %252, %198
  %259 = tail call i64 @llvm.smax.i64(i64 %252, i64 -1)
  %260 = select i1 %258, i64 %259, i64 -1
  br label %272

261:                                              ; preds = %248
  %262 = srem i64 %252, %212
  %263 = add nsw i64 %262, %212
  %264 = srem i64 %263, %212
  %265 = icmp slt i64 %264, %198
  %266 = xor i64 %264, -1
  %267 = add i64 %212, %266
  %268 = select i1 %265, i64 %264, i64 %267
  br label %272

269:                                              ; preds = %248
  %270 = tail call noundef i64 @llvm.smax.i64(i64 %252, i64 0)
  %271 = tail call noundef i64 @llvm.smin.i64(i64 %270, i64 %213)
  br label %272

272:                                              ; preds = %253, %257, %261, %269
  %273 = phi i64 [ %271, %269 ], [ %268, %261 ], [ %260, %257 ], [ %256, %253 ]
  %274 = or i64 %273, %249
  %275 = icmp sgt i64 %274, -1
  br i1 %275, label %276, label %470

276:                                              ; preds = %272
  %277 = load i64, ptr addrspace(1) %214, align 8
  %278 = load i64, ptr addrspace(1) %215, align 8
  switch i64 %278, label %280 [
    i64 0, label %281
    i64 20, label %281
    i64 1, label %279
    i64 21, label %279
    i64 6, label %279
  ]

279:                                              ; preds = %276, %276, %276
  br label %281

280:                                              ; preds = %276
  br label %281

281:                                              ; preds = %280, %279, %276, %276
  %282 = phi i32 [ 4, %280 ], [ 2, %279 ], [ 1, %276 ], [ 1, %276 ]
  %283 = load i64, ptr addrspace(1) %216, align 8
  %284 = load i64, ptr addrspace(1) %217, align 8
  %285 = tail call noundef i64 @llvm.smax.i64(i64 %284, i64 1)
  %286 = icmp eq i64 %283, 0
  %287 = zext nneg i32 %282 to i64
  br i1 %286, label %298, label %288

288:                                              ; preds = %281
  %289 = load i64, ptr addrspace(1) %195, align 8
  %290 = mul nsw i64 %285, %181
  %291 = mul nsw i64 %290, %198
  %292 = add nsw i64 %291, %273
  %293 = mul nsw i64 %292, %283
  %294 = mul nsw i64 %277, %249
  %295 = mul nsw i64 %294, %287
  %296 = add nsw i64 %293, %295
  %297 = add i64 %296, %289
  br label %308

298:                                              ; preds = %281
  %299 = mul nsw i64 %285, %181
  %300 = mul nsw i64 %299, %198
  %301 = add nsw i64 %300, %273
  %302 = mul nsw i64 %301, %186
  %303 = add i64 %302, %249
  %304 = load i64, ptr addrspace(1) %195, align 8
  %305 = mul i64 %277, %287
  %306 = mul i64 %305, %303
  %307 = add i64 %304, %306
  br label %308

308:                                              ; preds = %298, %288
  %309 = phi i64 [ %297, %288 ], [ %307, %298 ]
  %310 = icmp eq i64 %278, 5
  %311 = sext i1 %310 to i32
  br label %312

312:                                              ; preds = %465, %308
  %313 = phi i32 [ 0, %308 ], [ %468, %465 ]
  %314 = phi <4 x double> [ zeroinitializer, %308 ], [ %467, %465 ]
  %315 = zext nneg i32 %313 to i64
  %316 = icmp sgt i64 %277, %315
  br i1 %316, label %317, label %462

317:                                              ; preds = %312
  %318 = mul nuw nsw i32 %313, %282
  %319 = zext nneg i32 %318 to i64
  %320 = add i64 %309, %319
  switch i64 %278, label %322 [
    i64 0, label %323
    i64 20, label %323
    i64 1, label %321
    i64 21, label %321
    i64 6, label %321
  ]

321:                                              ; preds = %317, %317, %317
  br label %323

322:                                              ; preds = %317
  br label %323

323:                                              ; preds = %322, %321, %317, %317
  %324 = phi i32 [ 4, %322 ], [ 2, %321 ], [ 1, %317 ], [ 1, %317 ]
  %325 = inttoptr i64 %320 to ptr addrspace(1)
  br label %327

326:                                              ; preds = %327
  switch i64 %278, label %430 [
    i64 6, label %339
    i64 7, label %426
    i64 0, label %431
    i64 20, label %431
    i64 1, label %429
    i64 21, label %429
  ]

327:                                              ; preds = %327, %323
  %328 = phi i32 [ 0, %323 ], [ %336, %327 ]
  %329 = phi i32 [ 0, %323 ], [ %337, %327 ]
  %330 = zext nneg i32 %329 to i64
  %331 = getelementptr inbounds i8, ptr addrspace(1) %325, i64 %330
  %332 = load i8, ptr addrspace(1) %331, align 1
  %333 = zext i8 %332 to i32
  %334 = shl nsw i32 %329, 3
  %335 = shl i32 %333, %334
  %336 = or i32 %335, %328
  %337 = add nuw nsw i32 %329, 1
  %338 = icmp eq i32 %337, %324
  br i1 %338, label %326, label %327, !llvm.loop !10016

339:                                              ; preds = %326
  %340 = lshr i32 %336, 10
  %341 = and i32 %340, 31
  %342 = and i32 %336, 1023
  switch i32 %341, label %349 [
    i32 0, label %343
    i32 31, label %346
  ]

343:                                              ; preds = %339
  %344 = uitofp i32 %342 to double
  %345 = fmul double %344, 0x3E70000000000000
  br label %421

346:                                              ; preds = %339
  %347 = icmp eq i32 %342, 0
  %348 = select i1 %347, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %421

349:                                              ; preds = %339
  %350 = or disjoint i32 %342, 1024
  %351 = uitofp i32 %350 to double
  %352 = add nsw i32 %341, -25
  %353 = icmp ugt i32 %341, 25
  br i1 %353, label %354, label %371

354:                                              ; preds = %349
  %355 = add nsw i32 %341, -26
  %356 = and i32 %352, 7
  %357 = icmp eq i32 %356, 0
  br i1 %357, label %366, label %358

358:                                              ; preds = %354, %358
  %359 = phi i32 [ %363, %358 ], [ %352, %354 ]
  %360 = phi double [ %362, %358 ], [ %351, %354 ]
  %361 = phi i32 [ %364, %358 ], [ 0, %354 ]
  %362 = fmul double %360, 2.000000e+00
  %363 = add nsw i32 %359, -1
  %364 = add i32 %361, 1
  %365 = icmp eq i32 %364, %356
  br i1 %365, label %366, label %358, !llvm.loop !10047

366:                                              ; preds = %358, %354
  %367 = phi i32 [ %352, %354 ], [ %363, %358 ]
  %368 = phi double [ %351, %354 ], [ %362, %358 ]
  %369 = phi double [ undef, %354 ], [ %362, %358 ]
  %370 = icmp ult i32 %355, 7
  br i1 %370, label %371, label %394

371:                                              ; preds = %366, %394, %349
  %372 = phi double [ %351, %349 ], [ %369, %366 ], [ %404, %394 ]
  %373 = phi i32 [ %352, %349 ], [ 0, %394 ], [ 0, %366 ]
  %374 = icmp slt i32 %373, 0
  br i1 %374, label %375, label %421

375:                                              ; preds = %371
  %376 = tail call i32 @llvm.umin.i32(i32 %341, i32 25)
  %377 = sub nsw i32 1, %376
  %378 = and i32 %377, 7
  %379 = icmp eq i32 %378, 0
  br i1 %379, label %388, label %380

380:                                              ; preds = %375, %380
  %381 = phi i32 [ %385, %380 ], [ %373, %375 ]
  %382 = phi double [ %384, %380 ], [ %372, %375 ]
  %383 = phi i32 [ %386, %380 ], [ 0, %375 ]
  %384 = fmul double %382, 5.000000e-01
  %385 = add nsw i32 %381, 1
  %386 = add i32 %383, 1
  %387 = icmp eq i32 %386, %378
  br i1 %387, label %388, label %380, !llvm.loop !10048

388:                                              ; preds = %380, %375
  %389 = phi double [ undef, %375 ], [ %384, %380 ]
  %390 = phi i32 [ %373, %375 ], [ %385, %380 ]
  %391 = phi double [ %372, %375 ], [ %384, %380 ]
  %392 = add nsw i32 %376, -18
  %393 = icmp ult i32 %392, 7
  br i1 %393, label %421, label %408

394:                                              ; preds = %366, %394
  %395 = phi i32 [ %405, %394 ], [ %367, %366 ]
  %396 = phi double [ %404, %394 ], [ %368, %366 ]
  %397 = fmul double %396, 2.000000e+00
  %398 = fmul double %397, 2.000000e+00
  %399 = fmul double %398, 2.000000e+00
  %400 = fmul double %399, 2.000000e+00
  %401 = fmul double %400, 2.000000e+00
  %402 = fmul double %401, 2.000000e+00
  %403 = fmul double %402, 2.000000e+00
  %404 = fmul double %403, 2.000000e+00
  %405 = add nsw i32 %395, -8
  %406 = add i32 %395, -9
  %407 = icmp ult i32 %406, -2
  br i1 %407, label %394, label %371, !llvm.loop !10021

408:                                              ; preds = %388, %408
  %409 = phi i32 [ %419, %408 ], [ %390, %388 ]
  %410 = phi double [ %418, %408 ], [ %391, %388 ]
  %411 = fmul double %410, 5.000000e-01
  %412 = fmul double %411, 5.000000e-01
  %413 = fmul double %412, 5.000000e-01
  %414 = fmul double %413, 5.000000e-01
  %415 = fmul double %414, 5.000000e-01
  %416 = fmul double %415, 5.000000e-01
  %417 = fmul double %416, 5.000000e-01
  %418 = fmul double %417, 5.000000e-01
  %419 = add nsw i32 %409, 8
  %420 = icmp eq i32 %419, 0
  br i1 %420, label %421, label %408, !llvm.loop !10022

421:                                              ; preds = %388, %408, %371, %346, %343
  %422 = phi double [ %345, %343 ], [ %348, %346 ], [ %372, %371 ], [ %389, %388 ], [ %418, %408 ]
  %423 = icmp ult i32 %336, 32768
  %424 = fneg double %422
  %425 = select i1 %423, double %422, double %424
  br label %465

426:                                              ; preds = %326
  %427 = bitcast i32 %336 to float
  %428 = fpext float %427 to double
  br label %465

429:                                              ; preds = %326, %326
  br label %431

430:                                              ; preds = %326
  br label %431

431:                                              ; preds = %430, %429, %326, %326
  %432 = phi i32 [ 32, %430 ], [ 16, %429 ], [ 8, %326 ], [ 8, %326 ]
  %433 = zext i32 %336 to i64
  switch i64 %278, label %443 [
    i64 21, label %434
    i64 20, label %434
    i64 3, label %434
    i64 5, label %434
  ]

434:                                              ; preds = %431, %431, %431, %431
  %435 = add nsw i32 %432, -1
  %436 = shl nuw i32 1, %435
  %437 = and i32 %436, %336
  %438 = icmp eq i32 %437, 0
  %439 = zext nneg i32 %432 to i64
  %440 = shl nsw i64 -1, %439
  %441 = select i1 %438, i64 0, i64 %440
  %442 = add nsw i64 %441, %433
  br label %443

443:                                              ; preds = %434, %431
  %444 = phi i64 [ %442, %434 ], [ %433, %431 ]
  %445 = load i64, ptr addrspace(1) %218, align 8
  %446 = icmp eq i64 %445, 0
  br i1 %446, label %447, label %449

447:                                              ; preds = %443
  %448 = sitofp i64 %444 to double
  br label %465

449:                                              ; preds = %443
  switch i64 %278, label %450 [
    i64 21, label %451
    i64 20, label %451
    i64 3, label %451
  ]

450:                                              ; preds = %449
  br label %451

451:                                              ; preds = %450, %449, %449, %449
  %452 = phi i32 [ -1, %449 ], [ %311, %450 ], [ -1, %449 ], [ -1, %449 ]
  %453 = add nsw i32 %452, %432
  %454 = zext nneg i32 %453 to i64
  %455 = shl nsw i64 -1, %454
  %456 = xor i64 %455, -1
  %457 = sitofp i64 %444 to double
  %458 = sitofp i64 %456 to double
  %459 = fdiv double %457, %458
  %460 = fcmp oge double %459, -1.000000e+00
  %461 = select i1 %460, double %459, double -1.000000e+00
  br label %465

462:                                              ; preds = %312
  %463 = icmp eq i32 %313, 3
  %464 = select i1 %463, double 1.000000e+00, double 0.000000e+00
  br label %465

465:                                              ; preds = %462, %451, %447, %426, %421
  %466 = phi double [ %464, %462 ], [ %425, %421 ], [ %428, %426 ], [ %461, %451 ], [ %448, %447 ]
  %467 = insertelement <4 x double> %314, double %466, i32 %313
  %468 = add nuw nsw i32 %313, 1
  %469 = icmp eq i32 %468, 4
  br i1 %469, label %470, label %312, !llvm.loop !10029

470:                                              ; preds = %465, %272
  %471 = phi <4 x double> [ zeroinitializer, %272 ], [ %467, %465 ]
  %472 = extractelement <4 x double> %471, i64 0
  %473 = fcmp oge double %472, %208
  %474 = select i1 %473, double 1.000000e+00, double 0.000000e+00
  %475 = insertelement <4 x double> <double poison, double 0.000000e+00, double 0.000000e+00, double 1.000000e+00>, double %474, i64 0
  %476 = select i1 %206, <4 x double> %471, <4 x double> %475
  %477 = extractelement <4 x double> %476, i32 %3
  br i1 %209, label %478, label %481

478:                                              ; preds = %470
  %479 = fptrunc double %477 to float
  %480 = bitcast float %479 to i32
  br label %486

481:                                              ; preds = %470
  %482 = fcmp uno double %477, 0.000000e+00
  br i1 %482, label %486, label %483

483:                                              ; preds = %481
  %484 = fptosi double %477 to i64
  %485 = trunc i64 %484 to i32
  br label %486

486:                                              ; preds = %478, %481, %483
  %487 = phi i32 [ %480, %478 ], [ %485, %483 ], [ 0, %481 ]
  %488 = insertelement <4 x i32> %222, i32 %487, i32 %221
  %489 = add nuw nsw i32 %221, 1
  %490 = icmp eq i32 %489, 4
  br i1 %490, label %219, label %220, !llvm.loop !10049
}

; Function Attrs: alwaysinline mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read)
define dso_local i32 @__zluda_ptx_impl_agx_txq(ptr addrspace(1) nocapture noundef readonly %0, i32 noundef %1) local_unnamed_addr #3 {
  %3 = load i64, ptr addrspace(1) %0, align 8
  %4 = icmp eq i64 %3, 1396920397
  br i1 %4, label %5, label %25

5:                                                ; preds = %2
  switch i32 %1, label %80 [
    i32 5, label %6
    i32 6, label %10
    i32 4, label %16
    i32 13, label %20
  ]

6:                                                ; preds = %5
  %7 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 1
  %8 = load i64, ptr addrspace(1) %7, align 8
  %9 = trunc i64 %8 to i32
  br label %80

10:                                               ; preds = %5
  %11 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 2
  %12 = load i64, ptr addrspace(1) %11, align 8
  switch i64 %12, label %15 [
    i64 1, label %80
    i64 2, label %13
    i64 3, label %14
  ]

13:                                               ; preds = %10
  br label %80

14:                                               ; preds = %10
  br label %80

15:                                               ; preds = %10
  br label %80

16:                                               ; preds = %5
  %17 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %18 = load i64, ptr addrspace(1) %17, align 8
  %19 = trunc i64 %18 to i32
  br label %80

20:                                               ; preds = %5
  %21 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %22 = load i64, ptr addrspace(1) %21, align 8
  %23 = icmp eq i64 %22, 0
  %24 = zext i1 %23 to i32
  br label %80

25:                                               ; preds = %2
  switch i32 %1, label %80 [
    i32 0, label %26
    i32 1, label %26
    i32 2, label %26
    i32 3, label %33
    i32 4, label %49
    i32 5, label %53
    i32 6, label %57
    i32 7, label %63
    i32 12, label %69
    i32 13, label %75
  ]

26:                                               ; preds = %25, %25, %25
  %27 = zext nneg i32 %1 to i64
  %28 = getelementptr i64, ptr addrspace(1) %0, i64 %27
  %29 = getelementptr i64, ptr addrspace(1) %28, i64 9
  %30 = load i64, ptr addrspace(1) %29, align 8
  %31 = tail call noundef i64 @llvm.smax.i64(i64 %30, i64 1)
  %32 = trunc i64 %31 to i32
  br label %80

33:                                               ; preds = %25
  %34 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 8
  %35 = load i64, ptr addrspace(1) %34, align 8
  %36 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 13
  %37 = load i64, ptr addrspace(1) %36, align 8
  %38 = and i64 %37, 2
  %39 = icmp eq i64 %38, 0
  br i1 %39, label %45, label %40

40:                                               ; preds = %33
  %41 = icmp ugt i64 %35, 6
  %42 = udiv i64 %35, 6
  %43 = trunc i64 %42 to i32
  %44 = select i1 %41, i32 %43, i32 0
  br label %80

45:                                               ; preds = %33
  %46 = icmp ugt i64 %35, 1
  %47 = trunc i64 %35 to i32
  %48 = select i1 %46, i32 %47, i32 0
  br label %80

49:                                               ; preds = %25
  %50 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 6
  %51 = load i64, ptr addrspace(1) %50, align 8
  %52 = trunc i64 %51 to i32
  br label %80

53:                                               ; preds = %25
  %54 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 4
  %55 = load i64, ptr addrspace(1) %54, align 8
  %56 = trunc i64 %55 to i32
  br label %80

57:                                               ; preds = %25
  %58 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 5
  %59 = load i64, ptr addrspace(1) %58, align 8
  switch i64 %59, label %62 [
    i64 1, label %80
    i64 2, label %60
    i64 3, label %61
  ]

60:                                               ; preds = %57
  br label %80

61:                                               ; preds = %57
  br label %80

62:                                               ; preds = %57
  br label %80

63:                                               ; preds = %25
  %64 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 12
  %65 = load i64, ptr addrspace(1) %64, align 8
  %66 = icmp ugt i64 %65, 1
  %67 = trunc i64 %65 to i32
  %68 = select i1 %66, i32 %67, i32 0
  br label %80

69:                                               ; preds = %25
  %70 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 13
  %71 = load i64, ptr addrspace(1) %70, align 8
  %72 = trunc i64 %71 to i32
  %73 = lshr i32 %72, 8
  %74 = and i32 %73, 255
  br label %80

75:                                               ; preds = %25
  %76 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 6
  %77 = load i64, ptr addrspace(1) %76, align 8
  %78 = icmp eq i64 %77, 0
  %79 = zext i1 %78 to i32
  br label %80

80:                                               ; preds = %63, %62, %61, %60, %57, %15, %14, %13, %10, %25, %40, %45, %5, %75, %69, %53, %49, %26, %20, %16, %6
  %81 = phi i32 [ %24, %20 ], [ %19, %16 ], [ %9, %6 ], [ %79, %75 ], [ %74, %69 ], [ %56, %53 ], [ %52, %49 ], [ %32, %26 ], [ 0, %5 ], [ %44, %40 ], [ %48, %45 ], [ 0, %25 ], [ 0, %15 ], [ 1, %14 ], [ 4, %13 ], [ 3, %10 ], [ 0, %62 ], [ 1, %61 ], [ 4, %60 ], [ 3, %57 ], [ %68, %63 ]
  ret i32 %81
}

; Function Attrs: alwaysinline nounwind
define dso_local <4 x i64> @__zluda_ptx_impl_agx_suld_b(ptr addrspace(1) nocapture noundef readonly %0, i32 noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef %4, i32 noundef %5, i32 noundef %6, i32 noundef %7, i32 noundef %8) local_unnamed_addr #3 {
  switch i32 %1, label %14 [
    i32 6, label %10
    i32 4, label %10
    i32 3, label %10
  ]

10:                                               ; preds = %9, %9, %9
  %11 = icmp eq i32 %1, 3
  %12 = select i1 %11, i32 0, i32 %7
  %13 = zext i32 %5 to i64
  br label %21

14:                                               ; preds = %9
  %15 = icmp eq i32 %1, 0
  %16 = select i1 %15, i32 0, i32 %6
  %17 = icmp eq i32 %1, 2
  %18 = select i1 %17, i32 %7, i32 0
  %19 = select i1 %15, i32 0, i32 %18
  %20 = sext i32 %19 to i64
  br label %21

21:                                               ; preds = %10, %14
  %22 = phi i64 [ %20, %14 ], [ 0, %10 ]
  %23 = phi i32 [ %16, %14 ], [ %12, %10 ]
  %24 = phi i32 [ %5, %14 ], [ %6, %10 ]
  %25 = phi i64 [ 0, %14 ], [ %13, %10 ]
  %26 = sext i32 %24 to i64
  %27 = sext i32 %23 to i64
  %28 = sext i32 %3 to i64
  %29 = sext i32 %4 to i64
  %30 = mul nsw i64 %29, %28
  %31 = icmp eq i32 %2, 1
  br i1 %31, label %32, label %61

32:                                               ; preds = %21
  %33 = tail call noundef i64 @llvm.smax.i64(i64 %26, i64 0)
  %34 = getelementptr i64, ptr addrspace(1) %0, i64 4
  %35 = load i64, ptr addrspace(1) %34, align 8
  %36 = tail call noundef i64 @llvm.smax.i64(i64 %35, i64 1)
  %37 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 2
  %38 = load i64, ptr addrspace(1) %37, align 8
  %39 = tail call noundef i64 @llvm.smax.i64(i64 %38, i64 1)
  %40 = mul nsw i64 %39, %36
  %41 = sub nsw i64 %40, %30
  %42 = tail call noundef i64 @llvm.smax.i64(i64 %41, i64 0)
  %43 = tail call noundef i64 @llvm.smin.i64(i64 %33, i64 %42)
  %44 = tail call noundef i64 @llvm.smax.i64(i64 %27, i64 0)
  %45 = getelementptr i64, ptr addrspace(1) %0, i64 5
  %46 = load i64, ptr addrspace(1) %45, align 8
  %47 = tail call noundef i64 @llvm.smax.i64(i64 %46, i64 1)
  %48 = add nsw i64 %47, -1
  %49 = tail call noundef i64 @llvm.smin.i64(i64 %44, i64 %48)
  %50 = tail call noundef i64 @llvm.smax.i64(i64 %22, i64 0)
  %51 = getelementptr i64, ptr addrspace(1) %0, i64 6
  %52 = load i64, ptr addrspace(1) %51, align 8
  %53 = tail call noundef i64 @llvm.smax.i64(i64 %52, i64 1)
  %54 = add nsw i64 %53, -1
  %55 = tail call noundef i64 @llvm.smin.i64(i64 %50, i64 %54)
  %56 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %57 = load i64, ptr addrspace(1) %56, align 8
  %58 = tail call noundef i64 @llvm.smax.i64(i64 %57, i64 1)
  %59 = add nsw i64 %58, -1
  %60 = tail call noundef i64 @llvm.smin.i64(i64 %25, i64 %59)
  br label %61

61:                                               ; preds = %32, %21
  %62 = phi i64 [ %43, %32 ], [ %26, %21 ]
  %63 = phi i64 [ %49, %32 ], [ %27, %21 ]
  %64 = phi i64 [ %55, %32 ], [ %22, %21 ]
  %65 = phi i64 [ %60, %32 ], [ %25, %21 ]
  %66 = icmp slt i64 %62, 0
  br i1 %66, label %106, label %67

67:                                               ; preds = %61
  %68 = add nsw i64 %62, %30
  %69 = getelementptr i64, ptr addrspace(1) %0, i64 4
  %70 = load i64, ptr addrspace(1) %69, align 8
  %71 = tail call noundef i64 @llvm.smax.i64(i64 %70, i64 1)
  %72 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 2
  %73 = load i64, ptr addrspace(1) %72, align 8
  %74 = tail call noundef i64 @llvm.smax.i64(i64 %73, i64 1)
  %75 = mul i64 %74, %71
  %76 = icmp sgt i64 %68, %75
  %77 = icmp slt i64 %63, 0
  %78 = or i1 %77, %76
  br i1 %78, label %106, label %79

79:                                               ; preds = %67
  %80 = getelementptr i64, ptr addrspace(1) %0, i64 1
  %81 = getelementptr i64, ptr addrspace(1) %0, i64 5
  %82 = load i64, ptr addrspace(1) %81, align 8
  %83 = tail call noundef i64 @llvm.smax.i64(i64 %82, i64 1)
  %84 = icmp uge i64 %63, %83
  %85 = icmp slt i64 %64, 0
  %86 = or i1 %85, %84
  br i1 %86, label %106, label %87

87:                                               ; preds = %79
  %88 = getelementptr i64, ptr addrspace(1) %0, i64 6
  %89 = load i64, ptr addrspace(1) %88, align 8
  %90 = tail call noundef i64 @llvm.smax.i64(i64 %89, i64 1)
  %91 = icmp ult i64 %64, %90
  br i1 %91, label %92, label %106

92:                                               ; preds = %87
  %93 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %94 = load i64, ptr addrspace(1) %93, align 8
  %95 = tail call noundef i64 @llvm.smax.i64(i64 %94, i64 1)
  %96 = icmp ult i64 %65, %95
  br i1 %96, label %97, label %106

97:                                               ; preds = %92
  %98 = load i64, ptr addrspace(1) %80, align 8
  %99 = mul nsw i64 %90, %65
  %100 = add nsw i64 %99, %64
  %101 = mul nsw i64 %100, %83
  %102 = add nsw i64 %101, %63
  %103 = mul i64 %102, %75
  %104 = add nsw i64 %103, %62
  %105 = add i64 %104, %98
  br label %106

106:                                              ; preds = %61, %67, %79, %87, %92, %97
  %107 = phi i64 [ %105, %97 ], [ 0, %92 ], [ 0, %87 ], [ 0, %79 ], [ 0, %67 ], [ 0, %61 ]
  %108 = icmp eq i64 %107, 0
  br i1 %108, label %109, label %112

109:                                              ; preds = %106
  %110 = icmp eq i32 %2, 2
  br i1 %110, label %171, label %111

111:                                              ; preds = %109
  tail call void @__zluda_ptx_impl_agx_surface_trap()
  unreachable

112:                                              ; preds = %106
  %113 = inttoptr i64 %107 to ptr addrspace(1)
  %114 = icmp sgt i32 %3, 0
  br i1 %114, label %115, label %171

115:                                              ; preds = %112
  %116 = icmp sgt i32 %4, 0
  %117 = and i32 %4, 1
  %118 = icmp eq i32 %4, 1
  %119 = and i32 %4, 2147483646
  %120 = icmp eq i32 %117, 0
  br label %121

121:                                              ; preds = %115, %140
  %122 = phi i32 [ 0, %115 ], [ %143, %140 ]
  %123 = phi <4 x i64> [ zeroinitializer, %115 ], [ %142, %140 ]
  br i1 %116, label %124, label %140

124:                                              ; preds = %121
  %125 = mul nsw i32 %122, %4
  br i1 %118, label %126, label %145

126:                                              ; preds = %145, %124
  %127 = phi i64 [ undef, %124 ], [ %167, %145 ]
  %128 = phi i32 [ 0, %124 ], [ %168, %145 ]
  %129 = phi i64 [ 0, %124 ], [ %167, %145 ]
  br i1 %120, label %140, label %130

130:                                              ; preds = %126
  %131 = add nuw nsw i32 %128, %125
  %132 = zext nneg i32 %131 to i64
  %133 = getelementptr inbounds i8, ptr addrspace(1) %113, i64 %132
  %134 = load i8, ptr addrspace(1) %133, align 1
  %135 = zext i8 %134 to i64
  %136 = shl nsw i32 %128, 3
  %137 = zext nneg i32 %136 to i64
  %138 = shl i64 %135, %137
  %139 = or i64 %138, %129
  br label %140

140:                                              ; preds = %130, %126, %121
  %141 = phi i64 [ 0, %121 ], [ %127, %126 ], [ %139, %130 ]
  %142 = insertelement <4 x i64> %123, i64 %141, i32 %122
  %143 = add nuw nsw i32 %122, 1
  %144 = icmp eq i32 %143, %3
  br i1 %144, label %171, label %121, !llvm.loop !10050

145:                                              ; preds = %124, %145
  %146 = phi i32 [ %168, %145 ], [ 0, %124 ]
  %147 = phi i64 [ %167, %145 ], [ 0, %124 ]
  %148 = phi i32 [ %169, %145 ], [ 0, %124 ]
  %149 = add nuw nsw i32 %146, %125
  %150 = zext nneg i32 %149 to i64
  %151 = getelementptr inbounds i8, ptr addrspace(1) %113, i64 %150
  %152 = load i8, ptr addrspace(1) %151, align 1
  %153 = zext i8 %152 to i64
  %154 = shl nsw i32 %146, 3
  %155 = zext nneg i32 %154 to i64
  %156 = shl i64 %153, %155
  %157 = or i64 %156, %147
  %158 = or disjoint i32 %146, 1
  %159 = add nuw nsw i32 %158, %125
  %160 = zext nneg i32 %159 to i64
  %161 = getelementptr inbounds i8, ptr addrspace(1) %113, i64 %160
  %162 = load i8, ptr addrspace(1) %161, align 1
  %163 = zext i8 %162 to i64
  %164 = shl nsw i32 %158, 3
  %165 = zext nneg i32 %164 to i64
  %166 = shl i64 %163, %165
  %167 = or i64 %166, %157
  %168 = add nuw nsw i32 %146, 2
  %169 = add i32 %148, 2
  %170 = icmp eq i32 %169, %119
  br i1 %170, label %126, label %145, !llvm.loop !10051

171:                                              ; preds = %140, %112, %109
  %172 = phi <4 x i64> [ zeroinitializer, %109 ], [ zeroinitializer, %112 ], [ %142, %140 ]
  ret <4 x i64> %172
}

; Function Attrs: noreturn

; Function Attrs: alwaysinline nounwind
define dso_local void @__zluda_ptx_impl_agx_sust_b(ptr addrspace(1) nocapture noundef readonly %0, i32 noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef %4, i32 noundef %5, i32 noundef %6, i32 noundef %7, i32 noundef %8, i64 noundef %9, i64 noundef %10, i64 noundef %11, i64 noundef %12) local_unnamed_addr #3 {
  %14 = alloca [4 x i64], align 8
  switch i32 %1, label %19 [
    i32 6, label %15
    i32 4, label %15
    i32 3, label %15
  ]

15:                                               ; preds = %13, %13, %13
  %16 = icmp eq i32 %1, 3
  %17 = select i1 %16, i32 0, i32 %7
  %18 = zext i32 %5 to i64
  br label %26

19:                                               ; preds = %13
  %20 = icmp eq i32 %1, 0
  %21 = select i1 %20, i32 0, i32 %6
  %22 = icmp eq i32 %1, 2
  %23 = select i1 %22, i32 %7, i32 0
  %24 = select i1 %20, i32 0, i32 %23
  %25 = sext i32 %24 to i64
  br label %26

26:                                               ; preds = %15, %19
  %27 = phi i64 [ %25, %19 ], [ 0, %15 ]
  %28 = phi i32 [ %21, %19 ], [ %17, %15 ]
  %29 = phi i32 [ %5, %19 ], [ %6, %15 ]
  %30 = phi i64 [ 0, %19 ], [ %18, %15 ]
  %31 = sext i32 %29 to i64
  %32 = sext i32 %28 to i64
  %33 = sext i32 %3 to i64
  %34 = sext i32 %4 to i64
  %35 = mul nsw i64 %34, %33
  %36 = icmp eq i32 %2, 1
  br i1 %36, label %37, label %66

37:                                               ; preds = %26
  %38 = tail call noundef i64 @llvm.smax.i64(i64 %31, i64 0)
  %39 = getelementptr i64, ptr addrspace(1) %0, i64 4
  %40 = load i64, ptr addrspace(1) %39, align 8
  %41 = tail call noundef i64 @llvm.smax.i64(i64 %40, i64 1)
  %42 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 2
  %43 = load i64, ptr addrspace(1) %42, align 8
  %44 = tail call noundef i64 @llvm.smax.i64(i64 %43, i64 1)
  %45 = mul nsw i64 %44, %41
  %46 = sub nsw i64 %45, %35
  %47 = tail call noundef i64 @llvm.smax.i64(i64 %46, i64 0)
  %48 = tail call noundef i64 @llvm.smin.i64(i64 %38, i64 %47)
  %49 = tail call noundef i64 @llvm.smax.i64(i64 %32, i64 0)
  %50 = getelementptr i64, ptr addrspace(1) %0, i64 5
  %51 = load i64, ptr addrspace(1) %50, align 8
  %52 = tail call noundef i64 @llvm.smax.i64(i64 %51, i64 1)
  %53 = add nsw i64 %52, -1
  %54 = tail call noundef i64 @llvm.smin.i64(i64 %49, i64 %53)
  %55 = tail call noundef i64 @llvm.smax.i64(i64 %27, i64 0)
  %56 = getelementptr i64, ptr addrspace(1) %0, i64 6
  %57 = load i64, ptr addrspace(1) %56, align 8
  %58 = tail call noundef i64 @llvm.smax.i64(i64 %57, i64 1)
  %59 = add nsw i64 %58, -1
  %60 = tail call noundef i64 @llvm.smin.i64(i64 %55, i64 %59)
  %61 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %62 = load i64, ptr addrspace(1) %61, align 8
  %63 = tail call noundef i64 @llvm.smax.i64(i64 %62, i64 1)
  %64 = add nsw i64 %63, -1
  %65 = tail call noundef i64 @llvm.smin.i64(i64 %30, i64 %64)
  br label %66

66:                                               ; preds = %37, %26
  %67 = phi i64 [ %48, %37 ], [ %31, %26 ]
  %68 = phi i64 [ %54, %37 ], [ %32, %26 ]
  %69 = phi i64 [ %60, %37 ], [ %27, %26 ]
  %70 = phi i64 [ %65, %37 ], [ %30, %26 ]
  %71 = icmp slt i64 %67, 0
  br i1 %71, label %111, label %72

72:                                               ; preds = %66
  %73 = add nsw i64 %67, %35
  %74 = getelementptr i64, ptr addrspace(1) %0, i64 4
  %75 = load i64, ptr addrspace(1) %74, align 8
  %76 = tail call noundef i64 @llvm.smax.i64(i64 %75, i64 1)
  %77 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 2
  %78 = load i64, ptr addrspace(1) %77, align 8
  %79 = tail call noundef i64 @llvm.smax.i64(i64 %78, i64 1)
  %80 = mul i64 %79, %76
  %81 = icmp sgt i64 %73, %80
  %82 = icmp slt i64 %68, 0
  %83 = or i1 %82, %81
  br i1 %83, label %111, label %84

84:                                               ; preds = %72
  %85 = getelementptr i64, ptr addrspace(1) %0, i64 1
  %86 = getelementptr i64, ptr addrspace(1) %0, i64 5
  %87 = load i64, ptr addrspace(1) %86, align 8
  %88 = tail call noundef i64 @llvm.smax.i64(i64 %87, i64 1)
  %89 = icmp uge i64 %68, %88
  %90 = icmp slt i64 %69, 0
  %91 = or i1 %90, %89
  br i1 %91, label %111, label %92

92:                                               ; preds = %84
  %93 = getelementptr i64, ptr addrspace(1) %0, i64 6
  %94 = load i64, ptr addrspace(1) %93, align 8
  %95 = tail call noundef i64 @llvm.smax.i64(i64 %94, i64 1)
  %96 = icmp ult i64 %69, %95
  br i1 %96, label %97, label %111

97:                                               ; preds = %92
  %98 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %99 = load i64, ptr addrspace(1) %98, align 8
  %100 = tail call noundef i64 @llvm.smax.i64(i64 %99, i64 1)
  %101 = icmp ult i64 %70, %100
  br i1 %101, label %102, label %111

102:                                              ; preds = %97
  %103 = load i64, ptr addrspace(1) %85, align 8
  %104 = mul nsw i64 %95, %70
  %105 = add nsw i64 %104, %69
  %106 = mul nsw i64 %105, %88
  %107 = add nsw i64 %106, %68
  %108 = mul i64 %107, %80
  %109 = add nsw i64 %108, %67
  %110 = add i64 %109, %103
  br label %111

111:                                              ; preds = %66, %72, %84, %92, %97, %102
  %112 = phi i64 [ %110, %102 ], [ 0, %97 ], [ 0, %92 ], [ 0, %84 ], [ 0, %72 ], [ 0, %66 ]
  %113 = icmp eq i64 %112, 0
  br i1 %113, label %114, label %117

114:                                              ; preds = %111
  %115 = icmp eq i32 %2, 2
  br i1 %115, label %169, label %116

116:                                              ; preds = %114
  tail call void @__zluda_ptx_impl_agx_surface_trap()
  unreachable

117:                                              ; preds = %111
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %14)
  store i64 %9, ptr %14, align 8
  %118 = getelementptr inbounds i64, ptr %14, i64 1
  store i64 %10, ptr %118, align 8
  %119 = getelementptr inbounds i64, ptr %14, i64 2
  store i64 %11, ptr %119, align 8
  %120 = getelementptr inbounds i64, ptr %14, i64 3
  store i64 %12, ptr %120, align 8
  %121 = inttoptr i64 %112 to ptr addrspace(1)
  %122 = icmp sgt i32 %3, 0
  br i1 %122, label %123, label %168

123:                                              ; preds = %117
  %124 = icmp sgt i32 %4, 0
  %125 = icmp ult i32 %4, 4
  %126 = and i32 %4, 2147483644
  %127 = icmp eq i32 %126, %4
  br label %128

128:                                              ; preds = %154, %123
  %129 = phi i32 [ 0, %123 ], [ %155, %154 ]
  br i1 %124, label %130, label %154

130:                                              ; preds = %128
  %131 = zext nneg i32 %129 to i64
  %132 = getelementptr inbounds i64, ptr %14, i64 %131
  %133 = mul nsw i32 %129, %4
  %134 = load i64, ptr %132, align 8
  br i1 %125, label %152, label %135

135:                                              ; preds = %130
  %136 = insertelement <4 x i64> poison, i64 %134, i64 0
  %137 = shufflevector <4 x i64> %136, <4 x i64> poison, <4 x i32> zeroinitializer
  br label %138

138:                                              ; preds = %138, %135
  %139 = phi i32 [ 0, %135 ], [ %148, %138 ]
  %140 = phi <4 x i32> [ <i32 0, i32 1, i32 2, i32 3>, %135 ], [ %149, %138 ]
  %141 = shl nsw <4 x i32> %140, <i32 3, i32 3, i32 3, i32 3>
  %142 = zext nneg <4 x i32> %141 to <4 x i64>
  %143 = lshr <4 x i64> %137, %142
  %144 = trunc <4 x i64> %143 to <4 x i8>
  %145 = add nuw nsw i32 %139, %133
  %146 = zext nneg i32 %145 to i64
  %147 = getelementptr inbounds i8, ptr addrspace(1) %121, i64 %146
  store <4 x i8> %144, ptr addrspace(1) %147, align 1
  %148 = add nuw i32 %139, 4
  %149 = add <4 x i32> %140, <i32 4, i32 4, i32 4, i32 4>
  %150 = icmp eq i32 %148, %126
  br i1 %150, label %151, label %138, !llvm.loop !10052

151:                                              ; preds = %138
  br i1 %127, label %154, label %152

152:                                              ; preds = %130, %151
  %153 = phi i32 [ 0, %130 ], [ %126, %151 ]
  br label %157

154:                                              ; preds = %157, %151, %128
  %155 = add nuw nsw i32 %129, 1
  %156 = icmp eq i32 %155, %3
  br i1 %156, label %168, label %128, !llvm.loop !10055

157:                                              ; preds = %152, %157
  %158 = phi i32 [ %166, %157 ], [ %153, %152 ]
  %159 = shl nsw i32 %158, 3
  %160 = zext nneg i32 %159 to i64
  %161 = lshr i64 %134, %160
  %162 = trunc i64 %161 to i8
  %163 = add nuw nsw i32 %158, %133
  %164 = zext nneg i32 %163 to i64
  %165 = getelementptr inbounds i8, ptr addrspace(1) %121, i64 %164
  store i8 %162, ptr addrspace(1) %165, align 1
  %166 = add nuw nsw i32 %158, 1
  %167 = icmp eq i32 %166, %4
  br i1 %167, label %154, label %157, !llvm.loop !10056

168:                                              ; preds = %154, %117
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %14)
  br label %169

169:                                              ; preds = %114, %168
  ret void
}

; Function Attrs: alwaysinline nounwind
define dso_local void @__zluda_ptx_impl_agx_sust_p(ptr addrspace(1) nocapture noundef readonly %0, i32 noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef %4, i32 noundef %5, i32 noundef %6, i32 noundef %7, i32 noundef %8, i32 noundef %9, i32 noundef %10, i32 noundef %11) local_unnamed_addr #3 {
  switch i32 %1, label %17 [
    i32 6, label %13
    i32 4, label %13
    i32 3, label %13
  ]

13:                                               ; preds = %12, %12, %12
  %14 = icmp eq i32 %1, 3
  %15 = select i1 %14, i32 0, i32 %6
  %16 = zext i32 %4 to i64
  br label %24

17:                                               ; preds = %12
  %18 = icmp eq i32 %1, 0
  %19 = select i1 %18, i32 0, i32 %5
  %20 = icmp eq i32 %1, 2
  %21 = select i1 %20, i32 %6, i32 0
  %22 = select i1 %18, i32 0, i32 %21
  %23 = sext i32 %22 to i64
  br label %24

24:                                               ; preds = %13, %17
  %25 = phi i64 [ %23, %17 ], [ 0, %13 ]
  %26 = phi i32 [ %19, %17 ], [ %15, %13 ]
  %27 = phi i32 [ %4, %17 ], [ %5, %13 ]
  %28 = phi i64 [ 0, %17 ], [ %16, %13 ]
  %29 = sext i32 %27 to i64
  %30 = sext i32 %26 to i64
  %31 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 2
  %32 = load i64, ptr addrspace(1) %31, align 8
  %33 = tail call noundef i64 @llvm.smax.i64(i64 %32, i64 1)
  %34 = mul nsw i64 %33, %29
  %35 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 7
  %36 = load i64, ptr addrspace(1) %35, align 8
  %37 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 8
  %38 = load i64, ptr addrspace(1) %37, align 8
  %39 = tail call noundef i64 @llvm.smax.i64(i64 %38, i64 1)
  %40 = icmp sgt i64 %36, 3
  %41 = icmp ult i64 %36, 9
  %42 = select i1 %41, i32 2, i32 4
  %43 = select i1 %40, i32 %42, i32 1
  %44 = sext i32 %3 to i64
  %45 = tail call noundef i64 @llvm.smin.i64(i64 %44, i64 %39)
  %46 = trunc i64 %45 to i32
  %47 = select i1 %41, i64 1, i64 2
  %48 = select i1 %40, i64 %47, i64 0
  %49 = shl nsw i64 %45, %48
  %50 = icmp eq i32 %2, 1
  br i1 %50, label %51, label %77

51:                                               ; preds = %24
  %52 = tail call noundef i64 @llvm.smax.i64(i64 %34, i64 0)
  %53 = getelementptr i64, ptr addrspace(1) %0, i64 4
  %54 = load i64, ptr addrspace(1) %53, align 8
  %55 = tail call noundef i64 @llvm.smax.i64(i64 %54, i64 1)
  %56 = mul nsw i64 %55, %33
  %57 = sub nsw i64 %56, %49
  %58 = tail call noundef i64 @llvm.smax.i64(i64 %57, i64 0)
  %59 = tail call noundef i64 @llvm.smin.i64(i64 %52, i64 %58)
  %60 = tail call noundef i64 @llvm.smax.i64(i64 %30, i64 0)
  %61 = getelementptr i64, ptr addrspace(1) %0, i64 5
  %62 = load i64, ptr addrspace(1) %61, align 8
  %63 = tail call noundef i64 @llvm.smax.i64(i64 %62, i64 1)
  %64 = add nsw i64 %63, -1
  %65 = tail call noundef i64 @llvm.smin.i64(i64 %60, i64 %64)
  %66 = tail call noundef i64 @llvm.smax.i64(i64 %25, i64 0)
  %67 = getelementptr i64, ptr addrspace(1) %0, i64 6
  %68 = load i64, ptr addrspace(1) %67, align 8
  %69 = tail call noundef i64 @llvm.smax.i64(i64 %68, i64 1)
  %70 = add nsw i64 %69, -1
  %71 = tail call noundef i64 @llvm.smin.i64(i64 %66, i64 %70)
  %72 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %73 = load i64, ptr addrspace(1) %72, align 8
  %74 = tail call noundef i64 @llvm.smax.i64(i64 %73, i64 1)
  %75 = add nsw i64 %74, -1
  %76 = tail call noundef i64 @llvm.smin.i64(i64 %28, i64 %75)
  br label %77

77:                                               ; preds = %51, %24
  %78 = phi i64 [ %59, %51 ], [ %34, %24 ]
  %79 = phi i64 [ %65, %51 ], [ %30, %24 ]
  %80 = phi i64 [ %71, %51 ], [ %25, %24 ]
  %81 = phi i64 [ %76, %51 ], [ %28, %24 ]
  %82 = icmp slt i64 %78, 0
  br i1 %82, label %119, label %83

83:                                               ; preds = %77
  %84 = add nsw i64 %78, %49
  %85 = getelementptr i64, ptr addrspace(1) %0, i64 4
  %86 = load i64, ptr addrspace(1) %85, align 8
  %87 = tail call noundef i64 @llvm.smax.i64(i64 %86, i64 1)
  %88 = mul i64 %87, %33
  %89 = icmp sgt i64 %84, %88
  %90 = icmp slt i64 %79, 0
  %91 = or i1 %90, %89
  br i1 %91, label %119, label %92

92:                                               ; preds = %83
  %93 = getelementptr i64, ptr addrspace(1) %0, i64 1
  %94 = getelementptr i64, ptr addrspace(1) %0, i64 5
  %95 = load i64, ptr addrspace(1) %94, align 8
  %96 = tail call noundef i64 @llvm.smax.i64(i64 %95, i64 1)
  %97 = icmp uge i64 %79, %96
  %98 = icmp slt i64 %80, 0
  %99 = or i1 %98, %97
  br i1 %99, label %119, label %100

100:                                              ; preds = %92
  %101 = getelementptr i64, ptr addrspace(1) %0, i64 6
  %102 = load i64, ptr addrspace(1) %101, align 8
  %103 = tail call noundef i64 @llvm.smax.i64(i64 %102, i64 1)
  %104 = icmp ult i64 %80, %103
  br i1 %104, label %105, label %119

105:                                              ; preds = %100
  %106 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %107 = load i64, ptr addrspace(1) %106, align 8
  %108 = tail call noundef i64 @llvm.smax.i64(i64 %107, i64 1)
  %109 = icmp ult i64 %81, %108
  br i1 %109, label %110, label %119

110:                                              ; preds = %105
  %111 = load i64, ptr addrspace(1) %93, align 8
  %112 = mul nsw i64 %103, %81
  %113 = add nsw i64 %112, %80
  %114 = mul nsw i64 %113, %96
  %115 = add nsw i64 %114, %79
  %116 = mul i64 %115, %88
  %117 = add nsw i64 %116, %78
  %118 = add i64 %117, %111
  br label %119

119:                                              ; preds = %77, %83, %92, %100, %105, %110
  %120 = phi i64 [ %118, %110 ], [ 0, %105 ], [ 0, %100 ], [ 0, %92 ], [ 0, %83 ], [ 0, %77 ]
  %121 = icmp eq i64 %120, 0
  br i1 %121, label %122, label %125

122:                                              ; preds = %119
  %123 = icmp eq i32 %2, 2
  br i1 %123, label %295, label %124

124:                                              ; preds = %122
  tail call void @__zluda_ptx_impl_agx_surface_trap()
  unreachable

125:                                              ; preds = %119
  %126 = inttoptr i64 %120 to ptr addrspace(1)
  %127 = icmp sgt i32 %46, 0
  br i1 %127, label %128, label %295

128:                                              ; preds = %125
  %129 = select i1 %41, i32 16, i32 32
  %130 = select i1 %40, i32 %129, i32 8
  %131 = add nsw i32 %130, -1
  %132 = zext nneg i32 %131 to i64
  %133 = shl nuw nsw i64 1, %132
  %134 = sub nsw i64 0, %133
  %135 = add nsw i64 %133, -1
  %136 = zext nneg i32 %130 to i64
  %137 = shl nsw i64 -1, %136
  %138 = xor i64 %137, -1
  %139 = shl nsw i64 -1, %132
  %140 = xor i64 %139, -1
  %141 = sitofp i64 %140 to double
  %142 = sitofp i64 %138 to double
  %143 = select i1 %41, i32 1, i32 2
  %144 = select i1 %40, i32 %143, i32 0
  %145 = add nsw i32 %46, -1
  %146 = tail call i32 @llvm.umin.i32(i32 %145, i32 3)
  %147 = getelementptr i8, ptr addrspace(1) %126, i64 1
  %148 = icmp ugt i32 %43, 2
  %149 = getelementptr i8, ptr addrspace(1) %126, i64 2
  %150 = icmp ugt i32 %43, 3
  %151 = getelementptr i8, ptr addrspace(1) %126, i64 3
  br label %152

152:                                              ; preds = %128, %292
  %153 = phi i32 [ 0, %128 ], [ %293, %292 ]
  switch i32 %153, label %155 [
    i32 0, label %158
    i32 1, label %154
  ]

154:                                              ; preds = %152
  br label %158

155:                                              ; preds = %152
  %156 = icmp eq i32 %153, 2
  %157 = select i1 %156, i32 %10, i32 %11
  br label %158

158:                                              ; preds = %152, %154, %155
  %159 = phi i32 [ %9, %154 ], [ %157, %155 ], [ %8, %152 ]
  %160 = bitcast i32 %159 to float
  %161 = fpext float %160 to double
  switch i64 %36, label %267 [
    i64 0, label %162
    i64 4, label %162
    i64 1, label %181
    i64 5, label %181
    i64 2, label %203
    i64 6, label %203
    i64 9, label %203
    i64 3, label %206
    i64 7, label %206
    i64 10, label %206
    i64 8, label %211
  ]

162:                                              ; preds = %158, %158
  %163 = fcmp uno float %160, 0.000000e+00
  %164 = fcmp oge float %160, 0.000000e+00
  %165 = select i1 %164, double %161, double 0.000000e+00
  %166 = fcmp ole double %165, 1.000000e+00
  %167 = select i1 %166, double %165, double 1.000000e+00
  %168 = select i1 %163, double 0.000000e+00, double %167
  %169 = fmul double %168, %142
  %170 = tail call double @llvm.floor.f64(double %169)
  %171 = fsub double %169, %170
  %172 = fptosi double %170 to i64
  %173 = fcmp ogt double %171, 5.000000e-01
  br i1 %173, label %179, label %174

174:                                              ; preds = %162
  %175 = fcmp une double %171, 5.000000e-01
  %176 = and i64 %172, 1
  %177 = icmp eq i64 %176, 0
  %178 = select i1 %175, i1 true, i1 %177
  br i1 %178, label %269, label %179

179:                                              ; preds = %174, %162
  %180 = add nsw i64 %172, 1
  br label %269

181:                                              ; preds = %158, %158
  %182 = fcmp uno float %160, 0.000000e+00
  %183 = fcmp oge double %161, -1.000000e+00
  %184 = select i1 %183, double %161, double -1.000000e+00
  %185 = fcmp ole double %184, 1.000000e+00
  %186 = select i1 %185, double %184, double 1.000000e+00
  %187 = select i1 %182, double 0.000000e+00, double %186
  %188 = fmul double %187, %141
  %189 = tail call double @llvm.floor.f64(double %188)
  %190 = fsub double %188, %189
  %191 = fptosi double %189 to i64
  %192 = fcmp ogt double %190, 5.000000e-01
  br i1 %192, label %198, label %193

193:                                              ; preds = %181
  %194 = fcmp une double %190, 5.000000e-01
  %195 = and i64 %191, 1
  %196 = icmp eq i64 %195, 0
  %197 = select i1 %194, i1 true, i1 %196
  br i1 %197, label %200, label %198

198:                                              ; preds = %193, %181
  %199 = add nsw i64 %191, 1
  br label %200

200:                                              ; preds = %198, %193
  %201 = phi i64 [ %199, %198 ], [ %191, %193 ]
  %202 = and i64 %201, %138
  br label %269

203:                                              ; preds = %158, %158, %158
  %204 = zext i32 %159 to i64
  %205 = tail call noundef i64 @llvm.smin.i64(i64 %204, i64 %138)
  br label %269

206:                                              ; preds = %158, %158, %158
  %207 = sext i32 %159 to i64
  %208 = tail call noundef i64 @llvm.smax.i64(i64 %207, i64 %134)
  %209 = tail call noundef i64 @llvm.smin.i64(i64 %208, i64 %135)
  %210 = and i64 %209, %138
  br label %269

211:                                              ; preds = %158
  %212 = lshr i32 %159, 16
  %213 = and i32 %212, 32768
  %214 = lshr i32 %159, 23
  %215 = and i32 %214, 255
  %216 = and i32 %159, 8388607
  %217 = icmp eq i32 %215, 255
  br i1 %217, label %218, label %223

218:                                              ; preds = %211
  %219 = icmp eq i32 %216, 0
  %220 = select i1 %219, i32 0, i32 512
  %221 = or disjoint i32 %213, %220
  %222 = or disjoint i32 %221, 31744
  br label %264

223:                                              ; preds = %211
  %224 = icmp ugt i32 %215, 142
  br i1 %224, label %225, label %227

225:                                              ; preds = %223
  %226 = or disjoint i32 %213, 31744
  br label %264

227:                                              ; preds = %223
  %228 = icmp ult i32 %215, 113
  br i1 %228, label %229, label %251

229:                                              ; preds = %227
  %230 = icmp ult i32 %215, 102
  br i1 %230, label %264, label %231

231:                                              ; preds = %229
  %232 = or disjoint i32 %216, 8388608
  %233 = sub nuw nsw i32 126, %215
  %234 = lshr i32 %232, %233
  %235 = shl nsw i32 -1, %233
  %236 = xor i32 %235, -1
  %237 = and i32 %232, %236
  %238 = sub nuw nsw i32 125, %215
  %239 = shl nuw nsw i32 1, %238
  %240 = icmp ugt i32 %237, %239
  br i1 %240, label %246, label %241

241:                                              ; preds = %231
  %242 = icmp ne i32 %237, %239
  %243 = and i32 %234, 1
  %244 = icmp eq i32 %243, 0
  %245 = select i1 %242, i1 true, i1 %244
  br i1 %245, label %248, label %246

246:                                              ; preds = %241, %231
  %247 = add nuw nsw i32 %234, 1
  br label %248

248:                                              ; preds = %246, %241
  %249 = phi i32 [ %247, %246 ], [ %234, %241 ]
  %250 = or i32 %249, %213
  br label %264

251:                                              ; preds = %227
  %252 = shl nuw nsw i32 %215, 10
  %253 = lshr i32 %216, 13
  %254 = and i32 %159, 8191
  %255 = icmp ugt i32 %254, 4096
  %256 = and i32 %159, 16383
  %257 = icmp eq i32 %256, 12288
  %258 = or i1 %255, %257
  %259 = zext i1 %258 to i32
  %260 = or disjoint i32 %253, -114688
  %261 = add nsw i32 %260, %252
  %262 = add nuw nsw i32 %261, %259
  %263 = or i32 %262, %213
  br label %264

264:                                              ; preds = %251, %248, %229, %225, %218
  %265 = phi i32 [ %222, %218 ], [ %226, %225 ], [ %250, %248 ], [ %263, %251 ], [ %213, %229 ]
  %266 = zext nneg i32 %265 to i64
  br label %269

267:                                              ; preds = %158
  %268 = zext i32 %159 to i64
  br label %269

269:                                              ; preds = %174, %179, %200, %203, %206, %264, %267
  %270 = phi i64 [ %268, %267 ], [ %266, %264 ], [ %210, %206 ], [ %205, %203 ], [ %202, %200 ], [ %180, %179 ], [ %172, %174 ]
  %271 = shl nuw i32 %153, %144
  %272 = trunc i64 %270 to i8
  %273 = zext nneg i32 %271 to i64
  %274 = getelementptr inbounds i8, ptr addrspace(1) %126, i64 %273
  store i8 %272, ptr addrspace(1) %274, align 1
  br i1 %40, label %275, label %280

275:                                              ; preds = %269
  %276 = lshr i64 %270, 8
  %277 = trunc i64 %276 to i8
  %278 = sext i32 %271 to i64
  %279 = getelementptr i8, ptr addrspace(1) %147, i64 %278
  store i8 %277, ptr addrspace(1) %279, align 1
  br label %280

280:                                              ; preds = %275, %269
  br i1 %148, label %281, label %286

281:                                              ; preds = %280
  %282 = lshr i64 %270, 16
  %283 = trunc i64 %282 to i8
  %284 = sext i32 %271 to i64
  %285 = getelementptr i8, ptr addrspace(1) %149, i64 %284
  store i8 %283, ptr addrspace(1) %285, align 1
  br label %286

286:                                              ; preds = %281, %280
  br i1 %150, label %287, label %292

287:                                              ; preds = %286
  %288 = lshr i64 %270, 24
  %289 = trunc i64 %288 to i8
  %290 = sext i32 %271 to i64
  %291 = getelementptr i8, ptr addrspace(1) %151, i64 %290
  store i8 %289, ptr addrspace(1) %291, align 1
  br label %292

292:                                              ; preds = %287, %286
  %293 = add nuw nsw i32 %153, 1
  %294 = icmp eq i32 %153, %146
  br i1 %294, label %295, label %152, !llvm.loop !10057

295:                                              ; preds = %292, %125, %122
  ret void
}

; Function Attrs: alwaysinline nounwind
define dso_local i64 @__zluda_ptx_impl_agx_sured_addr(ptr addrspace(1) nocapture noundef readonly %0, i32 noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef %4, i32 noundef %5, i32 noundef %6, i32 noundef %7, i32 noundef %8) local_unnamed_addr #3 {
  switch i32 %1, label %14 [
    i32 6, label %10
    i32 4, label %10
    i32 3, label %10
  ]

10:                                               ; preds = %9, %9, %9
  %11 = icmp eq i32 %1, 3
  %12 = select i1 %11, i32 0, i32 %7
  %13 = zext i32 %5 to i64
  br label %21

14:                                               ; preds = %9
  %15 = icmp eq i32 %1, 0
  %16 = select i1 %15, i32 0, i32 %6
  %17 = icmp eq i32 %1, 2
  %18 = select i1 %17, i32 %7, i32 0
  %19 = select i1 %15, i32 0, i32 %18
  %20 = sext i32 %19 to i64
  br label %21

21:                                               ; preds = %10, %14
  %22 = phi i64 [ %20, %14 ], [ 0, %10 ]
  %23 = phi i32 [ %16, %14 ], [ %12, %10 ]
  %24 = phi i32 [ %5, %14 ], [ %6, %10 ]
  %25 = phi i64 [ 0, %14 ], [ %13, %10 ]
  %26 = sext i32 %24 to i64
  %27 = sext i32 %23 to i64
  %28 = icmp eq i32 %3, 0
  %29 = select i1 %28, i32 1, i32 %4
  %30 = sext i32 %29 to i64
  %31 = mul nsw i64 %26, %30
  %32 = sext i32 %4 to i64
  %33 = icmp eq i32 %2, 1
  br i1 %33, label %34, label %63

34:                                               ; preds = %21
  %35 = tail call noundef i64 @llvm.smax.i64(i64 %31, i64 0)
  %36 = getelementptr i64, ptr addrspace(1) %0, i64 4
  %37 = load i64, ptr addrspace(1) %36, align 8
  %38 = tail call noundef i64 @llvm.smax.i64(i64 %37, i64 1)
  %39 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 2
  %40 = load i64, ptr addrspace(1) %39, align 8
  %41 = tail call noundef i64 @llvm.smax.i64(i64 %40, i64 1)
  %42 = mul nsw i64 %41, %38
  %43 = sub nsw i64 %42, %32
  %44 = tail call noundef i64 @llvm.smax.i64(i64 %43, i64 0)
  %45 = tail call noundef i64 @llvm.smin.i64(i64 %35, i64 %44)
  %46 = tail call noundef i64 @llvm.smax.i64(i64 %27, i64 0)
  %47 = getelementptr i64, ptr addrspace(1) %0, i64 5
  %48 = load i64, ptr addrspace(1) %47, align 8
  %49 = tail call noundef i64 @llvm.smax.i64(i64 %48, i64 1)
  %50 = add nsw i64 %49, -1
  %51 = tail call noundef i64 @llvm.smin.i64(i64 %46, i64 %50)
  %52 = tail call noundef i64 @llvm.smax.i64(i64 %22, i64 0)
  %53 = getelementptr i64, ptr addrspace(1) %0, i64 6
  %54 = load i64, ptr addrspace(1) %53, align 8
  %55 = tail call noundef i64 @llvm.smax.i64(i64 %54, i64 1)
  %56 = add nsw i64 %55, -1
  %57 = tail call noundef i64 @llvm.smin.i64(i64 %52, i64 %56)
  %58 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %59 = load i64, ptr addrspace(1) %58, align 8
  %60 = tail call noundef i64 @llvm.smax.i64(i64 %59, i64 1)
  %61 = add nsw i64 %60, -1
  %62 = tail call noundef i64 @llvm.smin.i64(i64 %25, i64 %61)
  br label %63

63:                                               ; preds = %34, %21
  %64 = phi i64 [ %45, %34 ], [ %31, %21 ]
  %65 = phi i64 [ %51, %34 ], [ %27, %21 ]
  %66 = phi i64 [ %57, %34 ], [ %22, %21 ]
  %67 = phi i64 [ %62, %34 ], [ %25, %21 ]
  %68 = icmp slt i64 %64, 0
  br i1 %68, label %108, label %69

69:                                               ; preds = %63
  %70 = add nsw i64 %64, %32
  %71 = getelementptr i64, ptr addrspace(1) %0, i64 4
  %72 = load i64, ptr addrspace(1) %71, align 8
  %73 = tail call noundef i64 @llvm.smax.i64(i64 %72, i64 1)
  %74 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 2
  %75 = load i64, ptr addrspace(1) %74, align 8
  %76 = tail call noundef i64 @llvm.smax.i64(i64 %75, i64 1)
  %77 = mul i64 %76, %73
  %78 = icmp sgt i64 %70, %77
  %79 = icmp slt i64 %65, 0
  %80 = or i1 %79, %78
  br i1 %80, label %108, label %81

81:                                               ; preds = %69
  %82 = getelementptr i64, ptr addrspace(1) %0, i64 1
  %83 = getelementptr i64, ptr addrspace(1) %0, i64 5
  %84 = load i64, ptr addrspace(1) %83, align 8
  %85 = tail call noundef i64 @llvm.smax.i64(i64 %84, i64 1)
  %86 = icmp uge i64 %65, %85
  %87 = icmp slt i64 %66, 0
  %88 = or i1 %87, %86
  br i1 %88, label %108, label %89

89:                                               ; preds = %81
  %90 = getelementptr i64, ptr addrspace(1) %0, i64 6
  %91 = load i64, ptr addrspace(1) %90, align 8
  %92 = tail call noundef i64 @llvm.smax.i64(i64 %91, i64 1)
  %93 = icmp ult i64 %66, %92
  br i1 %93, label %94, label %108

94:                                               ; preds = %89
  %95 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %96 = load i64, ptr addrspace(1) %95, align 8
  %97 = tail call noundef i64 @llvm.smax.i64(i64 %96, i64 1)
  %98 = icmp ult i64 %67, %97
  br i1 %98, label %99, label %108

99:                                               ; preds = %94
  %100 = load i64, ptr addrspace(1) %82, align 8
  %101 = mul nsw i64 %92, %67
  %102 = add nsw i64 %101, %66
  %103 = mul nsw i64 %102, %85
  %104 = add nsw i64 %103, %65
  %105 = mul i64 %104, %77
  %106 = add nsw i64 %105, %64
  %107 = add i64 %106, %100
  br label %108

108:                                              ; preds = %63, %69, %81, %89, %94, %99
  %109 = phi i64 [ %107, %99 ], [ 0, %94 ], [ 0, %89 ], [ 0, %81 ], [ 0, %69 ], [ 0, %63 ]
  %110 = icmp eq i64 %109, 0
  %111 = icmp ne i32 %2, 2
  %112 = and i1 %111, %110
  br i1 %112, label %113, label %114

113:                                              ; preds = %108
  tail call void @__zluda_ptx_impl_agx_surface_trap()
  unreachable

114:                                              ; preds = %108
  ret i64 %109
}

; Function Attrs: alwaysinline nounwind
define dso_local void @__zluda_ptx_impl_agx_sured(ptr addrspace(1) nocapture noundef readonly %0, i32 noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef %4, i32 noundef %5, i32 noundef %6, i32 noundef %7, i32 noundef %8, i32 noundef %9, i32 noundef %10, i64 noundef %11) local_unnamed_addr #3 {
  switch i32 %1, label %17 [
    i32 6, label %13
    i32 4, label %13
    i32 3, label %13
  ]

13:                                               ; preds = %12, %12, %12
  %14 = icmp eq i32 %1, 3
  %15 = select i1 %14, i32 0, i32 %7
  %16 = zext i32 %5 to i64
  br label %24

17:                                               ; preds = %12
  %18 = icmp eq i32 %1, 0
  %19 = select i1 %18, i32 0, i32 %6
  %20 = icmp eq i32 %1, 2
  %21 = select i1 %20, i32 %7, i32 0
  %22 = select i1 %18, i32 0, i32 %21
  %23 = sext i32 %22 to i64
  br label %24

24:                                               ; preds = %13, %17
  %25 = phi i64 [ %23, %17 ], [ 0, %13 ]
  %26 = phi i32 [ %19, %17 ], [ %15, %13 ]
  %27 = phi i32 [ %5, %17 ], [ %6, %13 ]
  %28 = phi i64 [ 0, %17 ], [ %16, %13 ]
  %29 = sext i32 %27 to i64
  %30 = sext i32 %26 to i64
  %31 = icmp eq i32 %3, 0
  %32 = select i1 %31, i32 1, i32 %4
  %33 = sext i32 %32 to i64
  %34 = mul nsw i64 %29, %33
  %35 = sext i32 %4 to i64
  %36 = icmp eq i32 %2, 1
  br i1 %36, label %37, label %66

37:                                               ; preds = %24
  %38 = tail call noundef i64 @llvm.smax.i64(i64 %34, i64 0)
  %39 = getelementptr i64, ptr addrspace(1) %0, i64 4
  %40 = load i64, ptr addrspace(1) %39, align 8
  %41 = tail call noundef i64 @llvm.smax.i64(i64 %40, i64 1)
  %42 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 2
  %43 = load i64, ptr addrspace(1) %42, align 8
  %44 = tail call noundef i64 @llvm.smax.i64(i64 %43, i64 1)
  %45 = mul nsw i64 %44, %41
  %46 = sub nsw i64 %45, %35
  %47 = tail call noundef i64 @llvm.smax.i64(i64 %46, i64 0)
  %48 = tail call noundef i64 @llvm.smin.i64(i64 %38, i64 %47)
  %49 = tail call noundef i64 @llvm.smax.i64(i64 %30, i64 0)
  %50 = getelementptr i64, ptr addrspace(1) %0, i64 5
  %51 = load i64, ptr addrspace(1) %50, align 8
  %52 = tail call noundef i64 @llvm.smax.i64(i64 %51, i64 1)
  %53 = add nsw i64 %52, -1
  %54 = tail call noundef i64 @llvm.smin.i64(i64 %49, i64 %53)
  %55 = tail call noundef i64 @llvm.smax.i64(i64 %25, i64 0)
  %56 = getelementptr i64, ptr addrspace(1) %0, i64 6
  %57 = load i64, ptr addrspace(1) %56, align 8
  %58 = tail call noundef i64 @llvm.smax.i64(i64 %57, i64 1)
  %59 = add nsw i64 %58, -1
  %60 = tail call noundef i64 @llvm.smin.i64(i64 %55, i64 %59)
  %61 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %62 = load i64, ptr addrspace(1) %61, align 8
  %63 = tail call noundef i64 @llvm.smax.i64(i64 %62, i64 1)
  %64 = add nsw i64 %63, -1
  %65 = tail call noundef i64 @llvm.smin.i64(i64 %28, i64 %64)
  br label %66

66:                                               ; preds = %37, %24
  %67 = phi i64 [ %48, %37 ], [ %34, %24 ]
  %68 = phi i64 [ %54, %37 ], [ %30, %24 ]
  %69 = phi i64 [ %60, %37 ], [ %25, %24 ]
  %70 = phi i64 [ %65, %37 ], [ %28, %24 ]
  %71 = icmp slt i64 %67, 0
  br i1 %71, label %111, label %72

72:                                               ; preds = %66
  %73 = add nsw i64 %67, %35
  %74 = getelementptr i64, ptr addrspace(1) %0, i64 4
  %75 = load i64, ptr addrspace(1) %74, align 8
  %76 = tail call noundef i64 @llvm.smax.i64(i64 %75, i64 1)
  %77 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 2
  %78 = load i64, ptr addrspace(1) %77, align 8
  %79 = tail call noundef i64 @llvm.smax.i64(i64 %78, i64 1)
  %80 = mul i64 %79, %76
  %81 = icmp sgt i64 %73, %80
  %82 = icmp slt i64 %68, 0
  %83 = or i1 %82, %81
  br i1 %83, label %111, label %84

84:                                               ; preds = %72
  %85 = getelementptr i64, ptr addrspace(1) %0, i64 1
  %86 = getelementptr i64, ptr addrspace(1) %0, i64 5
  %87 = load i64, ptr addrspace(1) %86, align 8
  %88 = tail call noundef i64 @llvm.smax.i64(i64 %87, i64 1)
  %89 = icmp uge i64 %68, %88
  %90 = icmp slt i64 %69, 0
  %91 = or i1 %90, %89
  br i1 %91, label %111, label %92

92:                                               ; preds = %84
  %93 = getelementptr i64, ptr addrspace(1) %0, i64 6
  %94 = load i64, ptr addrspace(1) %93, align 8
  %95 = tail call noundef i64 @llvm.smax.i64(i64 %94, i64 1)
  %96 = icmp ult i64 %69, %95
  br i1 %96, label %97, label %111

97:                                               ; preds = %92
  %98 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %99 = load i64, ptr addrspace(1) %98, align 8
  %100 = tail call noundef i64 @llvm.smax.i64(i64 %99, i64 1)
  %101 = icmp ult i64 %70, %100
  br i1 %101, label %102, label %111

102:                                              ; preds = %97
  %103 = load i64, ptr addrspace(1) %85, align 8
  %104 = mul nsw i64 %95, %70
  %105 = add nsw i64 %104, %69
  %106 = mul nsw i64 %105, %88
  %107 = add nsw i64 %106, %68
  %108 = mul i64 %107, %80
  %109 = add nsw i64 %108, %67
  %110 = add i64 %109, %103
  br label %111

111:                                              ; preds = %66, %72, %84, %92, %97, %102
  %112 = phi i64 [ %110, %102 ], [ 0, %97 ], [ 0, %92 ], [ 0, %84 ], [ 0, %72 ], [ 0, %66 ]
  %113 = icmp eq i64 %112, 0
  %114 = icmp ne i32 %2, 2
  %115 = and i1 %114, %113
  br i1 %115, label %116, label %117

116:                                              ; preds = %111
  tail call void @__zluda_ptx_impl_agx_surface_trap()
  unreachable

117:                                              ; preds = %111
  br i1 %113, label %160, label %118

118:                                              ; preds = %117
  %119 = icmp eq i32 %4, 8
  %120 = inttoptr i64 %112 to ptr addrspace(1)
  br i1 %119, label %121, label %140

121:                                              ; preds = %118
  switch i32 %9, label %134 [
    i32 0, label %122
    i32 1, label %124
    i32 2, label %126
    i32 3, label %128
  ]

122:                                              ; preds = %121
  %123 = atomicrmw add ptr addrspace(1) %120, i64 %11 monotonic, align 8
  br label %160

124:                                              ; preds = %121
  %125 = atomicrmw and ptr addrspace(1) %120, i64 %11 monotonic, align 8
  br label %160

126:                                              ; preds = %121
  %127 = atomicrmw or ptr addrspace(1) %120, i64 %11 monotonic, align 8
  br label %160

128:                                              ; preds = %121
  %129 = icmp eq i32 %10, 0
  br i1 %129, label %132, label %130

130:                                              ; preds = %128
  %131 = atomicrmw min ptr addrspace(1) %120, i64 %11 monotonic, align 8
  br label %160

132:                                              ; preds = %128
  %133 = atomicrmw umin ptr addrspace(1) %120, i64 %11 monotonic, align 8
  br label %160

134:                                              ; preds = %121
  %135 = icmp eq i32 %10, 0
  br i1 %135, label %138, label %136

136:                                              ; preds = %134
  %137 = atomicrmw max ptr addrspace(1) %120, i64 %11 monotonic, align 8
  br label %160

138:                                              ; preds = %134
  %139 = atomicrmw umax ptr addrspace(1) %120, i64 %11 monotonic, align 8
  br label %160

140:                                              ; preds = %118
  %141 = trunc i64 %11 to i32
  switch i32 %9, label %154 [
    i32 0, label %142
    i32 1, label %144
    i32 2, label %146
    i32 3, label %148
  ]

142:                                              ; preds = %140
  %143 = atomicrmw add ptr addrspace(1) %120, i32 %141 monotonic, align 4
  br label %160

144:                                              ; preds = %140
  %145 = atomicrmw and ptr addrspace(1) %120, i32 %141 monotonic, align 4
  br label %160

146:                                              ; preds = %140
  %147 = atomicrmw or ptr addrspace(1) %120, i32 %141 monotonic, align 4
  br label %160

148:                                              ; preds = %140
  %149 = icmp eq i32 %10, 0
  br i1 %149, label %152, label %150

150:                                              ; preds = %148
  %151 = atomicrmw min ptr addrspace(1) %120, i32 %141 monotonic, align 4
  br label %160

152:                                              ; preds = %148
  %153 = atomicrmw umin ptr addrspace(1) %120, i32 %141 monotonic, align 4
  br label %160

154:                                              ; preds = %140
  %155 = icmp eq i32 %10, 0
  br i1 %155, label %158, label %156

156:                                              ; preds = %154
  %157 = atomicrmw max ptr addrspace(1) %120, i32 %141 monotonic, align 4
  br label %160

158:                                              ; preds = %154
  %159 = atomicrmw umax ptr addrspace(1) %120, i32 %141 monotonic, align 4
  br label %160

160:                                              ; preds = %142, %144, %146, %152, %150, %158, %156, %122, %124, %126, %132, %130, %138, %136, %117
  ret void
}

; Function Attrs: alwaysinline mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read)
define dso_local i32 @__zluda_ptx_impl_agx_surface_signed(ptr addrspace(1) nocapture noundef readonly %0) local_unnamed_addr #3 {
  %2 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 7
  %3 = load i64, ptr addrspace(1) %2, align 8
  %4 = and i64 %3, -5
  %5 = icmp eq i64 %4, 3
  %6 = icmp eq i64 %3, 10
  %7 = or i1 %6, %5
  %8 = zext i1 %7 to i32
  ret i32 %8
}

; Function Attrs: alwaysinline mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read)
define dso_local i32 @__zluda_ptx_impl_agx_suq(ptr addrspace(1) nocapture noundef readonly %0, i32 noundef %1) local_unnamed_addr #3 {
  switch i32 %1, label %30 [
    i32 0, label %3
    i32 1, label %3
    i32 2, label %3
    i32 3, label %10
    i32 4, label %18
    i32 5, label %22
    i32 6, label %26
  ]

3:                                                ; preds = %2, %2, %2
  %4 = zext nneg i32 %1 to i64
  %5 = getelementptr i64, ptr addrspace(1) %0, i64 %4
  %6 = getelementptr i64, ptr addrspace(1) %5, i64 4
  %7 = load i64, ptr addrspace(1) %6, align 8
  %8 = tail call noundef i64 @llvm.smax.i64(i64 %7, i64 1)
  %9 = trunc i64 %8 to i32
  br label %30

10:                                               ; preds = %2
  %11 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 12
  %12 = load i64, ptr addrspace(1) %11, align 8
  %13 = icmp eq i64 %12, 0
  br i1 %13, label %30, label %14

14:                                               ; preds = %10
  %15 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 3
  %16 = load i64, ptr addrspace(1) %15, align 8
  %17 = trunc i64 %16 to i32
  br label %30

18:                                               ; preds = %2
  %19 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 9
  %20 = load i64, ptr addrspace(1) %19, align 8
  %21 = trunc i64 %20 to i32
  br label %30

22:                                               ; preds = %2
  %23 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 10
  %24 = load i64, ptr addrspace(1) %23, align 8
  %25 = trunc i64 %24 to i32
  br label %30

26:                                               ; preds = %2
  %27 = getelementptr inbounds i64, ptr addrspace(1) %0, i64 11
  %28 = load i64, ptr addrspace(1) %27, align 8
  %29 = trunc i64 %28 to i32
  br label %30

30:                                               ; preds = %2, %14, %10, %26, %22, %18, %3
  %31 = phi i32 [ %29, %26 ], [ %25, %22 ], [ %21, %18 ], [ %9, %3 ], [ %17, %14 ], [ 0, %10 ], [ 0, %2 ]
  ret i32 %31
}

; Function Attrs: alwaysinline mustprogress nofree norecurse nosync nounwind willreturn memory(read, inaccessiblemem: none)
define dso_local i32 @__zluda_ptx_impl_agx_istypep(i64 noundef %0, i32 noundef %1) local_unnamed_addr #3 {
  %3 = icmp ugt i64 %0, 4095
  %4 = and i64 %0, 7
  %5 = icmp eq i64 %4, 0
  %6 = and i1 %3, %5
  br i1 %6, label %7, label %16

7:                                                ; preds = %2
  %8 = inttoptr i64 %0 to ptr addrspace(1)
  %9 = icmp eq i32 %1, 0
  %10 = icmp eq i32 %1, 1
  %11 = select i1 %10, i64 1396920397, i64 1398035526
  %12 = select i1 %9, i64 1415075924, i64 %11
  %13 = load i64, ptr addrspace(1) %8, align 8
  %14 = icmp eq i64 %13, %12
  %15 = zext i1 %14 to i32
  br label %16

16:                                               ; preds = %2, %7
  %17 = phi i32 [ %15, %7 ], [ 0, %2 ]
  ret i32 %17
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define dso_local <4 x float> @__zluda_ptx_impl_texobj_1d_v4_f32_f32(i64 noundef %0, <1 x float> noundef %1) local_unnamed_addr #3 {
  %3 = inttoptr i64 %0 to ptr addrspace(1)
  %4 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 4
  %5 = load i64, ptr addrspace(1) %4, align 8
  %6 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 5
  %7 = load i64, ptr addrspace(1) %6, align 8
  %8 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 6
  %9 = load i64, ptr addrspace(1) %8, align 8
  %10 = getelementptr i64, ptr addrspace(1) %3, i64 9
  %11 = and i64 %9, 4294967295
  %12 = icmp eq i64 %11, 1
  %13 = trunc i64 %7 to i32
  %14 = extractelement <1 x float> %1, i64 0
  %15 = fpext float %14 to double
  %16 = and i64 %5, 4294967295
  %17 = icmp eq i64 %16, 1
  %18 = load i64, ptr addrspace(1) %10, align 8
  %19 = tail call noundef i64 @llvm.smax.i64(i64 %18, i64 1)
  br i1 %17, label %236, label %20

20:                                               ; preds = %2
  %21 = getelementptr i64, ptr addrspace(1) %3, i64 1
  %22 = getelementptr i64, ptr addrspace(1) %3, i64 2
  %23 = sitofp i64 %19 to double
  %24 = select i1 %12, double %23, double 1.000000e+00
  %25 = fmul double %24, %15
  %26 = tail call double @llvm.floor.f64(double %25)
  %27 = fptosi double %26 to i64
  switch i32 %13, label %45 [
    i32 0, label %28
    i32 2, label %32
    i32 3, label %36
  ]

28:                                               ; preds = %20
  %29 = srem i64 %27, %19
  %30 = add nsw i64 %29, %19
  %31 = srem i64 %30, %19
  br label %49

32:                                               ; preds = %20
  %33 = icmp sgt i64 %19, %27
  %34 = tail call i64 @llvm.smax.i64(i64 %27, i64 -1)
  %35 = select i1 %33, i64 %34, i64 -1
  br label %49

36:                                               ; preds = %20
  %37 = shl nuw nsw i64 %19, 1
  %38 = srem i64 %27, %37
  %39 = add nsw i64 %38, %37
  %40 = srem i64 %39, %37
  %41 = icmp slt i64 %40, %19
  %42 = xor i64 %40, -1
  %43 = add i64 %37, %42
  %44 = select i1 %41, i64 %40, i64 %43
  br label %49

45:                                               ; preds = %20
  %46 = tail call noundef i64 @llvm.smax.i64(i64 %27, i64 0)
  %47 = add nsw i64 %19, -1
  %48 = tail call noundef i64 @llvm.smin.i64(i64 %46, i64 %47)
  br label %49

49:                                               ; preds = %45, %36, %32, %28
  %50 = phi i64 [ %48, %45 ], [ %44, %36 ], [ %35, %32 ], [ %31, %28 ]
  %51 = icmp sgt i64 %50, -1
  br i1 %51, label %52, label %496

52:                                               ; preds = %49
  %53 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 3
  %54 = load i64, ptr addrspace(1) %53, align 8
  %55 = load i64, ptr addrspace(1) %22, align 8
  switch i64 %55, label %57 [
    i64 0, label %58
    i64 20, label %58
    i64 1, label %56
    i64 21, label %56
    i64 6, label %56
  ]

56:                                               ; preds = %52, %52, %52
  br label %58

57:                                               ; preds = %52
  br label %58

58:                                               ; preds = %57, %56, %52, %52
  %59 = phi i32 [ 4, %57 ], [ 2, %56 ], [ 1, %52 ], [ 1, %52 ]
  %60 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 14
  %61 = load i64, ptr addrspace(1) %60, align 8
  %62 = icmp eq i64 %61, 0
  %63 = load i64, ptr addrspace(1) %21, align 8
  br i1 %62, label %68, label %64

64:                                               ; preds = %58
  %65 = mul nsw i64 %54, %50
  %66 = zext nneg i32 %59 to i64
  %67 = mul nsw i64 %65, %66
  br label %72

68:                                               ; preds = %58
  %69 = zext nneg i32 %59 to i64
  %70 = mul i64 %54, %50
  %71 = mul i64 %70, %69
  br label %72

72:                                               ; preds = %68, %64
  %73 = phi i64 [ %71, %68 ], [ %67, %64 ]
  %74 = add i64 %63, %73
  %75 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 7
  %76 = icmp eq i64 %55, 5
  %77 = sext i1 %76 to i32
  br label %78

78:                                               ; preds = %231, %72
  %79 = phi i32 [ 0, %72 ], [ %234, %231 ]
  %80 = phi <4 x double> [ zeroinitializer, %72 ], [ %233, %231 ]
  %81 = zext nneg i32 %79 to i64
  %82 = icmp sgt i64 %54, %81
  br i1 %82, label %83, label %228

83:                                               ; preds = %78
  %84 = mul nuw nsw i32 %79, %59
  %85 = zext nneg i32 %84 to i64
  %86 = add i64 %74, %85
  switch i64 %55, label %88 [
    i64 0, label %89
    i64 20, label %89
    i64 1, label %87
    i64 21, label %87
    i64 6, label %87
  ]

87:                                               ; preds = %83, %83, %83
  br label %89

88:                                               ; preds = %83
  br label %89

89:                                               ; preds = %88, %87, %83, %83
  %90 = phi i32 [ 4, %88 ], [ 2, %87 ], [ 1, %83 ], [ 1, %83 ]
  %91 = inttoptr i64 %86 to ptr addrspace(1)
  br label %93

92:                                               ; preds = %93
  switch i64 %55, label %196 [
    i64 6, label %105
    i64 7, label %192
    i64 0, label %197
    i64 20, label %197
    i64 1, label %195
    i64 21, label %195
  ]

93:                                               ; preds = %93, %89
  %94 = phi i32 [ 0, %89 ], [ %102, %93 ]
  %95 = phi i32 [ 0, %89 ], [ %103, %93 ]
  %96 = zext nneg i32 %95 to i64
  %97 = getelementptr inbounds i8, ptr addrspace(1) %91, i64 %96
  %98 = load i8, ptr addrspace(1) %97, align 1
  %99 = zext i8 %98 to i32
  %100 = shl nsw i32 %95, 3
  %101 = shl i32 %99, %100
  %102 = or i32 %101, %94
  %103 = add nuw nsw i32 %95, 1
  %104 = icmp eq i32 %103, %90
  br i1 %104, label %92, label %93, !llvm.loop !10016

105:                                              ; preds = %92
  %106 = lshr i32 %102, 10
  %107 = and i32 %106, 31
  %108 = and i32 %102, 1023
  switch i32 %107, label %115 [
    i32 0, label %109
    i32 31, label %112
  ]

109:                                              ; preds = %105
  %110 = uitofp i32 %108 to double
  %111 = fmul double %110, 0x3E70000000000000
  br label %187

112:                                              ; preds = %105
  %113 = icmp eq i32 %108, 0
  %114 = select i1 %113, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %187

115:                                              ; preds = %105
  %116 = or disjoint i32 %108, 1024
  %117 = uitofp i32 %116 to double
  %118 = add nsw i32 %107, -25
  %119 = icmp ugt i32 %107, 25
  br i1 %119, label %120, label %137

120:                                              ; preds = %115
  %121 = add nsw i32 %107, -26
  %122 = and i32 %118, 7
  %123 = icmp eq i32 %122, 0
  br i1 %123, label %132, label %124

124:                                              ; preds = %120, %124
  %125 = phi i32 [ %129, %124 ], [ %118, %120 ]
  %126 = phi double [ %128, %124 ], [ %117, %120 ]
  %127 = phi i32 [ %130, %124 ], [ 0, %120 ]
  %128 = fmul double %126, 2.000000e+00
  %129 = add nsw i32 %125, -1
  %130 = add i32 %127, 1
  %131 = icmp eq i32 %130, %122
  br i1 %131, label %132, label %124, !llvm.loop !10058

132:                                              ; preds = %124, %120
  %133 = phi i32 [ %118, %120 ], [ %129, %124 ]
  %134 = phi double [ %117, %120 ], [ %128, %124 ]
  %135 = phi double [ undef, %120 ], [ %128, %124 ]
  %136 = icmp ult i32 %121, 7
  br i1 %136, label %137, label %160

137:                                              ; preds = %132, %160, %115
  %138 = phi double [ %117, %115 ], [ %135, %132 ], [ %170, %160 ]
  %139 = phi i32 [ %118, %115 ], [ 0, %160 ], [ 0, %132 ]
  %140 = icmp slt i32 %139, 0
  br i1 %140, label %141, label %187

141:                                              ; preds = %137
  %142 = tail call i32 @llvm.umin.i32(i32 %107, i32 25)
  %143 = sub nsw i32 1, %142
  %144 = and i32 %143, 7
  %145 = icmp eq i32 %144, 0
  br i1 %145, label %154, label %146

146:                                              ; preds = %141, %146
  %147 = phi i32 [ %151, %146 ], [ %139, %141 ]
  %148 = phi double [ %150, %146 ], [ %138, %141 ]
  %149 = phi i32 [ %152, %146 ], [ 0, %141 ]
  %150 = fmul double %148, 5.000000e-01
  %151 = add nsw i32 %147, 1
  %152 = add i32 %149, 1
  %153 = icmp eq i32 %152, %144
  br i1 %153, label %154, label %146, !llvm.loop !10059

154:                                              ; preds = %146, %141
  %155 = phi double [ undef, %141 ], [ %150, %146 ]
  %156 = phi i32 [ %139, %141 ], [ %151, %146 ]
  %157 = phi double [ %138, %141 ], [ %150, %146 ]
  %158 = add nsw i32 %142, -18
  %159 = icmp ult i32 %158, 7
  br i1 %159, label %187, label %174

160:                                              ; preds = %132, %160
  %161 = phi i32 [ %171, %160 ], [ %133, %132 ]
  %162 = phi double [ %170, %160 ], [ %134, %132 ]
  %163 = fmul double %162, 2.000000e+00
  %164 = fmul double %163, 2.000000e+00
  %165 = fmul double %164, 2.000000e+00
  %166 = fmul double %165, 2.000000e+00
  %167 = fmul double %166, 2.000000e+00
  %168 = fmul double %167, 2.000000e+00
  %169 = fmul double %168, 2.000000e+00
  %170 = fmul double %169, 2.000000e+00
  %171 = add nsw i32 %161, -8
  %172 = add i32 %161, -9
  %173 = icmp ult i32 %172, -2
  br i1 %173, label %160, label %137, !llvm.loop !10021

174:                                              ; preds = %154, %174
  %175 = phi i32 [ %185, %174 ], [ %156, %154 ]
  %176 = phi double [ %184, %174 ], [ %157, %154 ]
  %177 = fmul double %176, 5.000000e-01
  %178 = fmul double %177, 5.000000e-01
  %179 = fmul double %178, 5.000000e-01
  %180 = fmul double %179, 5.000000e-01
  %181 = fmul double %180, 5.000000e-01
  %182 = fmul double %181, 5.000000e-01
  %183 = fmul double %182, 5.000000e-01
  %184 = fmul double %183, 5.000000e-01
  %185 = add nsw i32 %175, 8
  %186 = icmp eq i32 %185, 0
  br i1 %186, label %187, label %174, !llvm.loop !10022

187:                                              ; preds = %154, %174, %137, %112, %109
  %188 = phi double [ %111, %109 ], [ %114, %112 ], [ %138, %137 ], [ %155, %154 ], [ %184, %174 ]
  %189 = icmp ult i32 %102, 32768
  %190 = fneg double %188
  %191 = select i1 %189, double %188, double %190
  br label %231

192:                                              ; preds = %92
  %193 = bitcast i32 %102 to float
  %194 = fpext float %193 to double
  br label %231

195:                                              ; preds = %92, %92
  br label %197

196:                                              ; preds = %92
  br label %197

197:                                              ; preds = %196, %195, %92, %92
  %198 = phi i32 [ 32, %196 ], [ 16, %195 ], [ 8, %92 ], [ 8, %92 ]
  %199 = zext i32 %102 to i64
  switch i64 %55, label %209 [
    i64 21, label %200
    i64 20, label %200
    i64 3, label %200
    i64 5, label %200
  ]

200:                                              ; preds = %197, %197, %197, %197
  %201 = add nsw i32 %198, -1
  %202 = shl nuw i32 1, %201
  %203 = and i32 %202, %102
  %204 = icmp eq i32 %203, 0
  %205 = zext nneg i32 %198 to i64
  %206 = shl nsw i64 -1, %205
  %207 = select i1 %204, i64 0, i64 %206
  %208 = add nsw i64 %207, %199
  br label %209

209:                                              ; preds = %200, %197
  %210 = phi i64 [ %208, %200 ], [ %199, %197 ]
  %211 = load i64, ptr addrspace(1) %75, align 8
  %212 = icmp eq i64 %211, 0
  br i1 %212, label %213, label %215

213:                                              ; preds = %209
  %214 = sitofp i64 %210 to double
  br label %231

215:                                              ; preds = %209
  switch i64 %55, label %216 [
    i64 21, label %217
    i64 20, label %217
    i64 3, label %217
  ]

216:                                              ; preds = %215
  br label %217

217:                                              ; preds = %216, %215, %215, %215
  %218 = phi i32 [ -1, %215 ], [ %77, %216 ], [ -1, %215 ], [ -1, %215 ]
  %219 = add nsw i32 %218, %198
  %220 = zext nneg i32 %219 to i64
  %221 = shl nsw i64 -1, %220
  %222 = xor i64 %221, -1
  %223 = sitofp i64 %210 to double
  %224 = sitofp i64 %222 to double
  %225 = fdiv double %223, %224
  %226 = fcmp oge double %225, -1.000000e+00
  %227 = select i1 %226, double %225, double -1.000000e+00
  br label %231

228:                                              ; preds = %78
  %229 = icmp eq i32 %79, 3
  %230 = select i1 %229, double 1.000000e+00, double 0.000000e+00
  br label %231

231:                                              ; preds = %228, %217, %213, %192, %187
  %232 = phi double [ %230, %228 ], [ %191, %187 ], [ %194, %192 ], [ %227, %217 ], [ %214, %213 ]
  %233 = insertelement <4 x double> %80, double %232, i32 %79
  %234 = add nuw nsw i32 %79, 1
  %235 = icmp eq i32 %234, 4
  br i1 %235, label %496, label %78, !llvm.loop !10029

236:                                              ; preds = %2
  %237 = sitofp i64 %19 to double
  %238 = select i1 %12, double %237, double 1.000000e+00
  %239 = fmul double %238, %15
  %240 = fadd double %239, -5.000000e-01
  %241 = tail call double @llvm.floor.f64(double %240)
  %242 = fptosi double %241 to i64
  switch i32 %13, label %260 [
    i32 0, label %243
    i32 2, label %247
    i32 3, label %251
  ]

243:                                              ; preds = %236
  %244 = srem i64 %242, %19
  %245 = add nsw i64 %244, %19
  %246 = srem i64 %245, %19
  br label %264

247:                                              ; preds = %236
  %248 = icmp sgt i64 %19, %242
  %249 = tail call i64 @llvm.smax.i64(i64 %242, i64 -1)
  %250 = select i1 %248, i64 %249, i64 -1
  br label %264

251:                                              ; preds = %236
  %252 = shl nuw nsw i64 %19, 1
  %253 = srem i64 %242, %252
  %254 = add nsw i64 %253, %252
  %255 = srem i64 %254, %252
  %256 = icmp slt i64 %255, %19
  %257 = xor i64 %255, -1
  %258 = add i64 %252, %257
  %259 = select i1 %256, i64 %255, i64 %258
  br label %264

260:                                              ; preds = %236
  %261 = tail call noundef i64 @llvm.smax.i64(i64 %242, i64 0)
  %262 = add nsw i64 %19, -1
  %263 = tail call noundef i64 @llvm.smin.i64(i64 %261, i64 %262)
  br label %264

264:                                              ; preds = %260, %251, %247, %243
  %265 = phi i64 [ %263, %260 ], [ %259, %251 ], [ %250, %247 ], [ %246, %243 ]
  %266 = add i64 %242, 1
  switch i32 %13, label %284 [
    i32 0, label %267
    i32 2, label %271
    i32 3, label %275
  ]

267:                                              ; preds = %264
  %268 = srem i64 %266, %19
  %269 = add nsw i64 %268, %19
  %270 = srem i64 %269, %19
  br label %288

271:                                              ; preds = %264
  %272 = icmp slt i64 %266, %19
  %273 = tail call i64 @llvm.smax.i64(i64 %266, i64 -1)
  %274 = select i1 %272, i64 %273, i64 -1
  br label %288

275:                                              ; preds = %264
  %276 = shl nuw nsw i64 %19, 1
  %277 = srem i64 %266, %276
  %278 = add nsw i64 %277, %276
  %279 = srem i64 %278, %276
  %280 = icmp slt i64 %279, %19
  %281 = xor i64 %279, -1
  %282 = add i64 %276, %281
  %283 = select i1 %280, i64 %279, i64 %282
  br label %288

284:                                              ; preds = %264
  %285 = tail call noundef i64 @llvm.smax.i64(i64 %266, i64 0)
  %286 = add nsw i64 %19, -1
  %287 = tail call noundef i64 @llvm.smin.i64(i64 %285, i64 %286)
  br label %288

288:                                              ; preds = %284, %275, %271, %267
  %289 = phi i64 [ %287, %284 ], [ %283, %275 ], [ %274, %271 ], [ %270, %267 ]
  %290 = fsub double %240, %241
  %291 = fmul double %290, 2.560000e+02
  %292 = fadd double %291, 5.000000e-01
  %293 = tail call double @llvm.floor.f64(double %292)
  %294 = fptosi double %293 to i64
  %295 = sitofp i64 %294 to double
  %296 = fmul double %295, 3.906250e-03
  %297 = fsub double 1.000000e+00, %296
  %298 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 3
  %299 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 2
  %300 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 14
  %301 = getelementptr i64, ptr addrspace(1) %3, i64 1
  %302 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 7
  br label %303

303:                                              ; preds = %492, %288
  %304 = phi <4 x double> [ zeroinitializer, %288 ], [ %495, %492 ]
  %305 = phi i1 [ true, %288 ], [ false, %492 ]
  %306 = select i1 %305, double %297, double %296
  %307 = insertelement <4 x double> poison, double %306, i64 0
  %308 = shufflevector <4 x double> %307, <4 x double> poison, <4 x i32> zeroinitializer
  %309 = select i1 %305, i64 %265, i64 %289
  %310 = icmp sgt i64 %309, -1
  br i1 %310, label %311, label %492

311:                                              ; preds = %303
  %312 = load i64, ptr addrspace(1) %298, align 8
  %313 = load i64, ptr addrspace(1) %299, align 8
  switch i64 %313, label %315 [
    i64 0, label %316
    i64 20, label %316
    i64 1, label %314
    i64 21, label %314
    i64 6, label %314
  ]

314:                                              ; preds = %311, %311, %311
  br label %316

315:                                              ; preds = %311
  br label %316

316:                                              ; preds = %315, %314, %311, %311
  %317 = phi i32 [ 4, %315 ], [ 2, %314 ], [ 1, %311 ], [ 1, %311 ]
  %318 = load i64, ptr addrspace(1) %300, align 8
  %319 = icmp eq i64 %318, 0
  %320 = load i64, ptr addrspace(1) %301, align 8
  br i1 %319, label %325, label %321

321:                                              ; preds = %316
  %322 = mul nsw i64 %312, %309
  %323 = zext nneg i32 %317 to i64
  %324 = mul nsw i64 %322, %323
  br label %329

325:                                              ; preds = %316
  %326 = zext nneg i32 %317 to i64
  %327 = mul i64 %312, %309
  %328 = mul i64 %327, %326
  br label %329

329:                                              ; preds = %325, %321
  %330 = phi i64 [ %328, %325 ], [ %324, %321 ]
  %331 = add i64 %320, %330
  %332 = icmp eq i64 %313, 5
  %333 = sext i1 %332 to i32
  br label %334

334:                                              ; preds = %487, %329
  %335 = phi i32 [ 0, %329 ], [ %490, %487 ]
  %336 = phi <4 x double> [ zeroinitializer, %329 ], [ %489, %487 ]
  %337 = zext nneg i32 %335 to i64
  %338 = icmp sgt i64 %312, %337
  br i1 %338, label %339, label %484

339:                                              ; preds = %334
  %340 = mul nuw nsw i32 %335, %317
  %341 = zext nneg i32 %340 to i64
  %342 = add i64 %331, %341
  switch i64 %313, label %344 [
    i64 0, label %345
    i64 20, label %345
    i64 1, label %343
    i64 21, label %343
    i64 6, label %343
  ]

343:                                              ; preds = %339, %339, %339
  br label %345

344:                                              ; preds = %339
  br label %345

345:                                              ; preds = %344, %343, %339, %339
  %346 = phi i32 [ 4, %344 ], [ 2, %343 ], [ 1, %339 ], [ 1, %339 ]
  %347 = inttoptr i64 %342 to ptr addrspace(1)
  br label %349

348:                                              ; preds = %349
  switch i64 %313, label %452 [
    i64 6, label %361
    i64 7, label %448
    i64 0, label %453
    i64 20, label %453
    i64 1, label %451
    i64 21, label %451
  ]

349:                                              ; preds = %349, %345
  %350 = phi i32 [ 0, %345 ], [ %358, %349 ]
  %351 = phi i32 [ 0, %345 ], [ %359, %349 ]
  %352 = zext nneg i32 %351 to i64
  %353 = getelementptr inbounds i8, ptr addrspace(1) %347, i64 %352
  %354 = load i8, ptr addrspace(1) %353, align 1
  %355 = zext i8 %354 to i32
  %356 = shl nsw i32 %351, 3
  %357 = shl i32 %355, %356
  %358 = or i32 %357, %350
  %359 = add nuw nsw i32 %351, 1
  %360 = icmp eq i32 %359, %346
  br i1 %360, label %348, label %349, !llvm.loop !10016

361:                                              ; preds = %348
  %362 = lshr i32 %358, 10
  %363 = and i32 %362, 31
  %364 = and i32 %358, 1023
  switch i32 %363, label %371 [
    i32 0, label %365
    i32 31, label %368
  ]

365:                                              ; preds = %361
  %366 = uitofp i32 %364 to double
  %367 = fmul double %366, 0x3E70000000000000
  br label %443

368:                                              ; preds = %361
  %369 = icmp eq i32 %364, 0
  %370 = select i1 %369, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %443

371:                                              ; preds = %361
  %372 = or disjoint i32 %364, 1024
  %373 = uitofp i32 %372 to double
  %374 = add nsw i32 %363, -25
  %375 = icmp ugt i32 %363, 25
  br i1 %375, label %376, label %393

376:                                              ; preds = %371
  %377 = add nsw i32 %363, -26
  %378 = and i32 %374, 7
  %379 = icmp eq i32 %378, 0
  br i1 %379, label %388, label %380

380:                                              ; preds = %376, %380
  %381 = phi i32 [ %385, %380 ], [ %374, %376 ]
  %382 = phi double [ %384, %380 ], [ %373, %376 ]
  %383 = phi i32 [ %386, %380 ], [ 0, %376 ]
  %384 = fmul double %382, 2.000000e+00
  %385 = add nsw i32 %381, -1
  %386 = add i32 %383, 1
  %387 = icmp eq i32 %386, %378
  br i1 %387, label %388, label %380, !llvm.loop !10060

388:                                              ; preds = %380, %376
  %389 = phi i32 [ %374, %376 ], [ %385, %380 ]
  %390 = phi double [ %373, %376 ], [ %384, %380 ]
  %391 = phi double [ undef, %376 ], [ %384, %380 ]
  %392 = icmp ult i32 %377, 7
  br i1 %392, label %393, label %416

393:                                              ; preds = %388, %416, %371
  %394 = phi double [ %373, %371 ], [ %391, %388 ], [ %426, %416 ]
  %395 = phi i32 [ %374, %371 ], [ 0, %416 ], [ 0, %388 ]
  %396 = icmp slt i32 %395, 0
  br i1 %396, label %397, label %443

397:                                              ; preds = %393
  %398 = tail call i32 @llvm.umin.i32(i32 %363, i32 25)
  %399 = sub nsw i32 1, %398
  %400 = and i32 %399, 7
  %401 = icmp eq i32 %400, 0
  br i1 %401, label %410, label %402

402:                                              ; preds = %397, %402
  %403 = phi i32 [ %407, %402 ], [ %395, %397 ]
  %404 = phi double [ %406, %402 ], [ %394, %397 ]
  %405 = phi i32 [ %408, %402 ], [ 0, %397 ]
  %406 = fmul double %404, 5.000000e-01
  %407 = add nsw i32 %403, 1
  %408 = add i32 %405, 1
  %409 = icmp eq i32 %408, %400
  br i1 %409, label %410, label %402, !llvm.loop !10061

410:                                              ; preds = %402, %397
  %411 = phi double [ undef, %397 ], [ %406, %402 ]
  %412 = phi i32 [ %395, %397 ], [ %407, %402 ]
  %413 = phi double [ %394, %397 ], [ %406, %402 ]
  %414 = add nsw i32 %398, -18
  %415 = icmp ult i32 %414, 7
  br i1 %415, label %443, label %430

416:                                              ; preds = %388, %416
  %417 = phi i32 [ %427, %416 ], [ %389, %388 ]
  %418 = phi double [ %426, %416 ], [ %390, %388 ]
  %419 = fmul double %418, 2.000000e+00
  %420 = fmul double %419, 2.000000e+00
  %421 = fmul double %420, 2.000000e+00
  %422 = fmul double %421, 2.000000e+00
  %423 = fmul double %422, 2.000000e+00
  %424 = fmul double %423, 2.000000e+00
  %425 = fmul double %424, 2.000000e+00
  %426 = fmul double %425, 2.000000e+00
  %427 = add nsw i32 %417, -8
  %428 = add i32 %417, -9
  %429 = icmp ult i32 %428, -2
  br i1 %429, label %416, label %393, !llvm.loop !10021

430:                                              ; preds = %410, %430
  %431 = phi i32 [ %441, %430 ], [ %412, %410 ]
  %432 = phi double [ %440, %430 ], [ %413, %410 ]
  %433 = fmul double %432, 5.000000e-01
  %434 = fmul double %433, 5.000000e-01
  %435 = fmul double %434, 5.000000e-01
  %436 = fmul double %435, 5.000000e-01
  %437 = fmul double %436, 5.000000e-01
  %438 = fmul double %437, 5.000000e-01
  %439 = fmul double %438, 5.000000e-01
  %440 = fmul double %439, 5.000000e-01
  %441 = add nsw i32 %431, 8
  %442 = icmp eq i32 %441, 0
  br i1 %442, label %443, label %430, !llvm.loop !10022

443:                                              ; preds = %410, %430, %393, %368, %365
  %444 = phi double [ %367, %365 ], [ %370, %368 ], [ %394, %393 ], [ %411, %410 ], [ %440, %430 ]
  %445 = icmp ult i32 %358, 32768
  %446 = fneg double %444
  %447 = select i1 %445, double %444, double %446
  br label %487

448:                                              ; preds = %348
  %449 = bitcast i32 %358 to float
  %450 = fpext float %449 to double
  br label %487

451:                                              ; preds = %348, %348
  br label %453

452:                                              ; preds = %348
  br label %453

453:                                              ; preds = %452, %451, %348, %348
  %454 = phi i32 [ 32, %452 ], [ 16, %451 ], [ 8, %348 ], [ 8, %348 ]
  %455 = zext i32 %358 to i64
  switch i64 %313, label %465 [
    i64 21, label %456
    i64 20, label %456
    i64 3, label %456
    i64 5, label %456
  ]

456:                                              ; preds = %453, %453, %453, %453
  %457 = add nsw i32 %454, -1
  %458 = shl nuw i32 1, %457
  %459 = and i32 %458, %358
  %460 = icmp eq i32 %459, 0
  %461 = zext nneg i32 %454 to i64
  %462 = shl nsw i64 -1, %461
  %463 = select i1 %460, i64 0, i64 %462
  %464 = add nsw i64 %463, %455
  br label %465

465:                                              ; preds = %456, %453
  %466 = phi i64 [ %464, %456 ], [ %455, %453 ]
  %467 = load i64, ptr addrspace(1) %302, align 8
  %468 = icmp eq i64 %467, 0
  br i1 %468, label %469, label %471

469:                                              ; preds = %465
  %470 = sitofp i64 %466 to double
  br label %487

471:                                              ; preds = %465
  switch i64 %313, label %472 [
    i64 21, label %473
    i64 20, label %473
    i64 3, label %473
  ]

472:                                              ; preds = %471
  br label %473

473:                                              ; preds = %472, %471, %471, %471
  %474 = phi i32 [ -1, %471 ], [ %333, %472 ], [ -1, %471 ], [ -1, %471 ]
  %475 = add nsw i32 %474, %454
  %476 = zext nneg i32 %475 to i64
  %477 = shl nsw i64 -1, %476
  %478 = xor i64 %477, -1
  %479 = sitofp i64 %466 to double
  %480 = sitofp i64 %478 to double
  %481 = fdiv double %479, %480
  %482 = fcmp oge double %481, -1.000000e+00
  %483 = select i1 %482, double %481, double -1.000000e+00
  br label %487

484:                                              ; preds = %334
  %485 = icmp eq i32 %335, 3
  %486 = select i1 %485, double 1.000000e+00, double 0.000000e+00
  br label %487

487:                                              ; preds = %484, %473, %469, %448, %443
  %488 = phi double [ %486, %484 ], [ %447, %443 ], [ %450, %448 ], [ %483, %473 ], [ %470, %469 ]
  %489 = insertelement <4 x double> %336, double %488, i32 %335
  %490 = add nuw nsw i32 %335, 1
  %491 = icmp eq i32 %490, 4
  br i1 %491, label %492, label %334, !llvm.loop !10029

492:                                              ; preds = %487, %303
  %493 = phi <4 x double> [ zeroinitializer, %303 ], [ %489, %487 ]
  %494 = fmul <4 x double> %308, %493
  %495 = fadd <4 x double> %304, %494
  br i1 %305, label %303, label %496, !llvm.loop !10032

496:                                              ; preds = %231, %492, %49
  %497 = phi <4 x double> [ zeroinitializer, %49 ], [ %495, %492 ], [ %233, %231 ]
  %498 = extractelement <4 x double> %497, i64 0
  %499 = fptrunc double %498 to float
  %500 = insertelement <4 x float> poison, float %499, i64 0
  %501 = extractelement <4 x double> %497, i64 1
  %502 = fptrunc double %501 to float
  %503 = insertelement <4 x float> %500, float %502, i64 1
  %504 = extractelement <4 x double> %497, i64 2
  %505 = fptrunc double %504 to float
  %506 = insertelement <4 x float> %503, float %505, i64 2
  %507 = extractelement <4 x double> %497, i64 3
  %508 = fptrunc double %507 to float
  %509 = insertelement <4 x float> %506, float %508, i64 3
  ret <4 x float> %509
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define dso_local <4 x float> @__zluda_ptx_impl_texobj_1d_v4_f32_s32(i64 noundef %0, <1 x i32> noundef %1) local_unnamed_addr #3 {
  %3 = inttoptr i64 %0 to ptr addrspace(1)
  %4 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 5
  %5 = load i64, ptr addrspace(1) %4, align 8
  %6 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 6
  %7 = load i64, ptr addrspace(1) %6, align 8
  %8 = getelementptr i64, ptr addrspace(1) %3, i64 9
  %9 = and i64 %7, 4294967295
  %10 = icmp eq i64 %9, 1
  %11 = extractelement <1 x i32> %1, i64 0
  %12 = trunc i64 %5 to i32
  %13 = sitofp i32 %11 to double
  %14 = load i64, ptr addrspace(1) %8, align 8
  %15 = tail call noundef i64 @llvm.smax.i64(i64 %14, i64 1)
  %16 = getelementptr i64, ptr addrspace(1) %3, i64 1
  %17 = getelementptr i64, ptr addrspace(1) %3, i64 2
  %18 = sitofp i64 %15 to double
  %19 = select i1 %10, double %18, double 1.000000e+00
  %20 = fmul double %19, %13
  %21 = tail call double @llvm.floor.f64(double %20)
  %22 = fptosi double %21 to i64
  switch i32 %12, label %40 [
    i32 0, label %23
    i32 2, label %27
    i32 3, label %31
  ]

23:                                               ; preds = %2
  %24 = srem i64 %22, %15
  %25 = add nsw i64 %24, %15
  %26 = srem i64 %25, %15
  br label %44

27:                                               ; preds = %2
  %28 = icmp sgt i64 %15, %22
  %29 = tail call i64 @llvm.smax.i64(i64 %22, i64 -1)
  %30 = select i1 %28, i64 %29, i64 -1
  br label %44

31:                                               ; preds = %2
  %32 = shl nuw nsw i64 %15, 1
  %33 = srem i64 %22, %32
  %34 = add nsw i64 %33, %32
  %35 = srem i64 %34, %32
  %36 = icmp slt i64 %35, %15
  %37 = xor i64 %35, -1
  %38 = add i64 %32, %37
  %39 = select i1 %36, i64 %35, i64 %38
  br label %44

40:                                               ; preds = %2
  %41 = tail call noundef i64 @llvm.smax.i64(i64 %22, i64 0)
  %42 = add nsw i64 %15, -1
  %43 = tail call noundef i64 @llvm.smin.i64(i64 %41, i64 %42)
  br label %44

44:                                               ; preds = %40, %31, %27, %23
  %45 = phi i64 [ %43, %40 ], [ %39, %31 ], [ %30, %27 ], [ %26, %23 ]
  %46 = icmp sgt i64 %45, -1
  br i1 %46, label %47, label %231

47:                                               ; preds = %44
  %48 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 3
  %49 = load i64, ptr addrspace(1) %48, align 8
  %50 = load i64, ptr addrspace(1) %17, align 8
  switch i64 %50, label %52 [
    i64 0, label %53
    i64 20, label %53
    i64 1, label %51
    i64 21, label %51
    i64 6, label %51
  ]

51:                                               ; preds = %47, %47, %47
  br label %53

52:                                               ; preds = %47
  br label %53

53:                                               ; preds = %52, %51, %47, %47
  %54 = phi i32 [ 4, %52 ], [ 2, %51 ], [ 1, %47 ], [ 1, %47 ]
  %55 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 14
  %56 = load i64, ptr addrspace(1) %55, align 8
  %57 = icmp eq i64 %56, 0
  %58 = load i64, ptr addrspace(1) %16, align 8
  br i1 %57, label %63, label %59

59:                                               ; preds = %53
  %60 = mul nsw i64 %49, %45
  %61 = zext nneg i32 %54 to i64
  %62 = mul nsw i64 %60, %61
  br label %67

63:                                               ; preds = %53
  %64 = zext nneg i32 %54 to i64
  %65 = mul i64 %49, %45
  %66 = mul i64 %65, %64
  br label %67

67:                                               ; preds = %63, %59
  %68 = phi i64 [ %66, %63 ], [ %62, %59 ]
  %69 = add i64 %58, %68
  %70 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 7
  %71 = icmp eq i64 %50, 5
  %72 = sext i1 %71 to i32
  br label %73

73:                                               ; preds = %226, %67
  %74 = phi i32 [ 0, %67 ], [ %229, %226 ]
  %75 = phi <4 x double> [ zeroinitializer, %67 ], [ %228, %226 ]
  %76 = zext nneg i32 %74 to i64
  %77 = icmp sgt i64 %49, %76
  br i1 %77, label %78, label %223

78:                                               ; preds = %73
  %79 = mul nuw nsw i32 %74, %54
  %80 = zext nneg i32 %79 to i64
  %81 = add i64 %69, %80
  switch i64 %50, label %83 [
    i64 0, label %84
    i64 20, label %84
    i64 1, label %82
    i64 21, label %82
    i64 6, label %82
  ]

82:                                               ; preds = %78, %78, %78
  br label %84

83:                                               ; preds = %78
  br label %84

84:                                               ; preds = %83, %82, %78, %78
  %85 = phi i32 [ 4, %83 ], [ 2, %82 ], [ 1, %78 ], [ 1, %78 ]
  %86 = inttoptr i64 %81 to ptr addrspace(1)
  br label %88

87:                                               ; preds = %88
  switch i64 %50, label %191 [
    i64 6, label %100
    i64 7, label %187
    i64 0, label %192
    i64 20, label %192
    i64 1, label %190
    i64 21, label %190
  ]

88:                                               ; preds = %88, %84
  %89 = phi i32 [ 0, %84 ], [ %97, %88 ]
  %90 = phi i32 [ 0, %84 ], [ %98, %88 ]
  %91 = zext nneg i32 %90 to i64
  %92 = getelementptr inbounds i8, ptr addrspace(1) %86, i64 %91
  %93 = load i8, ptr addrspace(1) %92, align 1
  %94 = zext i8 %93 to i32
  %95 = shl nsw i32 %90, 3
  %96 = shl i32 %94, %95
  %97 = or i32 %96, %89
  %98 = add nuw nsw i32 %90, 1
  %99 = icmp eq i32 %98, %85
  br i1 %99, label %87, label %88, !llvm.loop !10016

100:                                              ; preds = %87
  %101 = lshr i32 %97, 10
  %102 = and i32 %101, 31
  %103 = and i32 %97, 1023
  switch i32 %102, label %110 [
    i32 0, label %104
    i32 31, label %107
  ]

104:                                              ; preds = %100
  %105 = uitofp i32 %103 to double
  %106 = fmul double %105, 0x3E70000000000000
  br label %182

107:                                              ; preds = %100
  %108 = icmp eq i32 %103, 0
  %109 = select i1 %108, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %182

110:                                              ; preds = %100
  %111 = or disjoint i32 %103, 1024
  %112 = uitofp i32 %111 to double
  %113 = add nsw i32 %102, -25
  %114 = icmp ugt i32 %102, 25
  br i1 %114, label %115, label %132

115:                                              ; preds = %110
  %116 = add nsw i32 %102, -26
  %117 = and i32 %113, 7
  %118 = icmp eq i32 %117, 0
  br i1 %118, label %127, label %119

119:                                              ; preds = %115, %119
  %120 = phi i32 [ %124, %119 ], [ %113, %115 ]
  %121 = phi double [ %123, %119 ], [ %112, %115 ]
  %122 = phi i32 [ %125, %119 ], [ 0, %115 ]
  %123 = fmul double %121, 2.000000e+00
  %124 = add nsw i32 %120, -1
  %125 = add i32 %122, 1
  %126 = icmp eq i32 %125, %117
  br i1 %126, label %127, label %119, !llvm.loop !10062

127:                                              ; preds = %119, %115
  %128 = phi i32 [ %113, %115 ], [ %124, %119 ]
  %129 = phi double [ %112, %115 ], [ %123, %119 ]
  %130 = phi double [ undef, %115 ], [ %123, %119 ]
  %131 = icmp ult i32 %116, 7
  br i1 %131, label %132, label %155

132:                                              ; preds = %127, %155, %110
  %133 = phi double [ %112, %110 ], [ %130, %127 ], [ %165, %155 ]
  %134 = phi i32 [ %113, %110 ], [ 0, %155 ], [ 0, %127 ]
  %135 = icmp slt i32 %134, 0
  br i1 %135, label %136, label %182

136:                                              ; preds = %132
  %137 = tail call i32 @llvm.umin.i32(i32 %102, i32 25)
  %138 = sub nsw i32 1, %137
  %139 = and i32 %138, 7
  %140 = icmp eq i32 %139, 0
  br i1 %140, label %149, label %141

141:                                              ; preds = %136, %141
  %142 = phi i32 [ %146, %141 ], [ %134, %136 ]
  %143 = phi double [ %145, %141 ], [ %133, %136 ]
  %144 = phi i32 [ %147, %141 ], [ 0, %136 ]
  %145 = fmul double %143, 5.000000e-01
  %146 = add nsw i32 %142, 1
  %147 = add i32 %144, 1
  %148 = icmp eq i32 %147, %139
  br i1 %148, label %149, label %141, !llvm.loop !10063

149:                                              ; preds = %141, %136
  %150 = phi double [ undef, %136 ], [ %145, %141 ]
  %151 = phi i32 [ %134, %136 ], [ %146, %141 ]
  %152 = phi double [ %133, %136 ], [ %145, %141 ]
  %153 = add nsw i32 %137, -18
  %154 = icmp ult i32 %153, 7
  br i1 %154, label %182, label %169

155:                                              ; preds = %127, %155
  %156 = phi i32 [ %166, %155 ], [ %128, %127 ]
  %157 = phi double [ %165, %155 ], [ %129, %127 ]
  %158 = fmul double %157, 2.000000e+00
  %159 = fmul double %158, 2.000000e+00
  %160 = fmul double %159, 2.000000e+00
  %161 = fmul double %160, 2.000000e+00
  %162 = fmul double %161, 2.000000e+00
  %163 = fmul double %162, 2.000000e+00
  %164 = fmul double %163, 2.000000e+00
  %165 = fmul double %164, 2.000000e+00
  %166 = add nsw i32 %156, -8
  %167 = add i32 %156, -9
  %168 = icmp ult i32 %167, -2
  br i1 %168, label %155, label %132, !llvm.loop !10021

169:                                              ; preds = %149, %169
  %170 = phi i32 [ %180, %169 ], [ %151, %149 ]
  %171 = phi double [ %179, %169 ], [ %152, %149 ]
  %172 = fmul double %171, 5.000000e-01
  %173 = fmul double %172, 5.000000e-01
  %174 = fmul double %173, 5.000000e-01
  %175 = fmul double %174, 5.000000e-01
  %176 = fmul double %175, 5.000000e-01
  %177 = fmul double %176, 5.000000e-01
  %178 = fmul double %177, 5.000000e-01
  %179 = fmul double %178, 5.000000e-01
  %180 = add nsw i32 %170, 8
  %181 = icmp eq i32 %180, 0
  br i1 %181, label %182, label %169, !llvm.loop !10022

182:                                              ; preds = %149, %169, %132, %107, %104
  %183 = phi double [ %106, %104 ], [ %109, %107 ], [ %133, %132 ], [ %150, %149 ], [ %179, %169 ]
  %184 = icmp ult i32 %97, 32768
  %185 = fneg double %183
  %186 = select i1 %184, double %183, double %185
  br label %226

187:                                              ; preds = %87
  %188 = bitcast i32 %97 to float
  %189 = fpext float %188 to double
  br label %226

190:                                              ; preds = %87, %87
  br label %192

191:                                              ; preds = %87
  br label %192

192:                                              ; preds = %191, %190, %87, %87
  %193 = phi i32 [ 32, %191 ], [ 16, %190 ], [ 8, %87 ], [ 8, %87 ]
  %194 = zext i32 %97 to i64
  switch i64 %50, label %204 [
    i64 21, label %195
    i64 20, label %195
    i64 3, label %195
    i64 5, label %195
  ]

195:                                              ; preds = %192, %192, %192, %192
  %196 = add nsw i32 %193, -1
  %197 = shl nuw i32 1, %196
  %198 = and i32 %197, %97
  %199 = icmp eq i32 %198, 0
  %200 = zext nneg i32 %193 to i64
  %201 = shl nsw i64 -1, %200
  %202 = select i1 %199, i64 0, i64 %201
  %203 = add nsw i64 %202, %194
  br label %204

204:                                              ; preds = %195, %192
  %205 = phi i64 [ %203, %195 ], [ %194, %192 ]
  %206 = load i64, ptr addrspace(1) %70, align 8
  %207 = icmp eq i64 %206, 0
  br i1 %207, label %208, label %210

208:                                              ; preds = %204
  %209 = sitofp i64 %205 to double
  br label %226

210:                                              ; preds = %204
  switch i64 %50, label %211 [
    i64 21, label %212
    i64 20, label %212
    i64 3, label %212
  ]

211:                                              ; preds = %210
  br label %212

212:                                              ; preds = %211, %210, %210, %210
  %213 = phi i32 [ -1, %210 ], [ %72, %211 ], [ -1, %210 ], [ -1, %210 ]
  %214 = add nsw i32 %213, %193
  %215 = zext nneg i32 %214 to i64
  %216 = shl nsw i64 -1, %215
  %217 = xor i64 %216, -1
  %218 = sitofp i64 %205 to double
  %219 = sitofp i64 %217 to double
  %220 = fdiv double %218, %219
  %221 = fcmp oge double %220, -1.000000e+00
  %222 = select i1 %221, double %220, double -1.000000e+00
  br label %226

223:                                              ; preds = %73
  %224 = icmp eq i32 %74, 3
  %225 = select i1 %224, double 1.000000e+00, double 0.000000e+00
  br label %226

226:                                              ; preds = %223, %212, %208, %187, %182
  %227 = phi double [ %225, %223 ], [ %186, %182 ], [ %189, %187 ], [ %222, %212 ], [ %209, %208 ]
  %228 = insertelement <4 x double> %75, double %227, i32 %74
  %229 = add nuw nsw i32 %74, 1
  %230 = icmp eq i32 %229, 4
  br i1 %230, label %231, label %73, !llvm.loop !10029

231:                                              ; preds = %226, %44
  %232 = phi <4 x double> [ zeroinitializer, %44 ], [ %228, %226 ]
  %233 = extractelement <4 x double> %232, i64 0
  %234 = fptrunc double %233 to float
  %235 = insertelement <4 x float> poison, float %234, i64 0
  %236 = extractelement <4 x double> %232, i64 1
  %237 = fptrunc double %236 to float
  %238 = insertelement <4 x float> %235, float %237, i64 1
  %239 = extractelement <4 x double> %232, i64 2
  %240 = fptrunc double %239 to float
  %241 = insertelement <4 x float> %238, float %240, i64 2
  %242 = extractelement <4 x double> %232, i64 3
  %243 = fptrunc double %242 to float
  %244 = insertelement <4 x float> %241, float %243, i64 3
  ret <4 x float> %244
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define dso_local <4 x float> @__zluda_ptx_impl_texobj_2d_v4_f32_f32(i64 noundef %0, <2 x float> noundef %1) local_unnamed_addr #3 {
  %3 = inttoptr i64 %0 to ptr addrspace(1)
  %4 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 4
  %5 = load i64, ptr addrspace(1) %4, align 8
  %6 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 5
  %7 = load i64, ptr addrspace(1) %6, align 8
  %8 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 6
  %9 = load i64, ptr addrspace(1) %8, align 8
  %10 = getelementptr i64, ptr addrspace(1) %3, i64 9
  %11 = and i64 %9, 4294967295
  %12 = icmp eq i64 %11, 1
  %13 = trunc i64 %7 to i32
  %14 = extractelement <2 x float> %1, i64 0
  %15 = fpext float %14 to double
  %16 = extractelement <2 x float> %1, i64 1
  %17 = fpext float %16 to double
  %18 = and i64 %5, 4294967295
  %19 = icmp eq i64 %18, 1
  %20 = load i64, ptr addrspace(1) %10, align 8
  %21 = tail call noundef i64 @llvm.smax.i64(i64 %20, i64 1)
  br i1 %19, label %275, label %22

22:                                               ; preds = %2
  %23 = getelementptr i64, ptr addrspace(1) %3, i64 1
  %24 = getelementptr i64, ptr addrspace(1) %3, i64 10
  %25 = load i64, ptr addrspace(1) %24, align 8
  %26 = tail call noundef i64 @llvm.smax.i64(i64 %25, i64 1)
  %27 = getelementptr i64, ptr addrspace(1) %3, i64 2
  %28 = sitofp i64 %21 to double
  %29 = select i1 %12, double %28, double 1.000000e+00
  %30 = fmul double %29, %15
  %31 = tail call double @llvm.floor.f64(double %30)
  %32 = fptosi double %31 to i64
  switch i32 %13, label %50 [
    i32 0, label %33
    i32 2, label %37
    i32 3, label %41
  ]

33:                                               ; preds = %22
  %34 = srem i64 %32, %21
  %35 = add nsw i64 %34, %21
  %36 = srem i64 %35, %21
  br label %54

37:                                               ; preds = %22
  %38 = icmp sgt i64 %21, %32
  %39 = tail call i64 @llvm.smax.i64(i64 %32, i64 -1)
  %40 = select i1 %38, i64 %39, i64 -1
  br label %54

41:                                               ; preds = %22
  %42 = shl nuw nsw i64 %21, 1
  %43 = srem i64 %32, %42
  %44 = add nsw i64 %43, %42
  %45 = srem i64 %44, %42
  %46 = icmp slt i64 %45, %21
  %47 = xor i64 %45, -1
  %48 = add i64 %42, %47
  %49 = select i1 %46, i64 %45, i64 %48
  br label %54

50:                                               ; preds = %22
  %51 = tail call noundef i64 @llvm.smax.i64(i64 %32, i64 0)
  %52 = add nsw i64 %21, -1
  %53 = tail call noundef i64 @llvm.smin.i64(i64 %51, i64 %52)
  br label %54

54:                                               ; preds = %50, %41, %37, %33
  %55 = phi i64 [ %53, %50 ], [ %49, %41 ], [ %40, %37 ], [ %36, %33 ]
  %56 = sitofp i64 %26 to double
  %57 = select i1 %12, double %56, double 1.000000e+00
  %58 = fmul double %57, %17
  %59 = tail call double @llvm.floor.f64(double %58)
  %60 = fptosi double %59 to i64
  switch i32 %13, label %78 [
    i32 0, label %61
    i32 2, label %65
    i32 3, label %69
  ]

61:                                               ; preds = %54
  %62 = srem i64 %60, %26
  %63 = add nsw i64 %62, %26
  %64 = srem i64 %63, %26
  br label %82

65:                                               ; preds = %54
  %66 = icmp sgt i64 %26, %60
  %67 = tail call i64 @llvm.smax.i64(i64 %60, i64 -1)
  %68 = select i1 %66, i64 %67, i64 -1
  br label %82

69:                                               ; preds = %54
  %70 = shl nuw nsw i64 %26, 1
  %71 = srem i64 %60, %70
  %72 = add nsw i64 %71, %70
  %73 = srem i64 %72, %70
  %74 = icmp slt i64 %73, %26
  %75 = xor i64 %73, -1
  %76 = add i64 %70, %75
  %77 = select i1 %74, i64 %73, i64 %76
  br label %82

78:                                               ; preds = %54
  %79 = tail call noundef i64 @llvm.smax.i64(i64 %60, i64 0)
  %80 = add nsw i64 %26, -1
  %81 = tail call noundef i64 @llvm.smin.i64(i64 %79, i64 %80)
  br label %82

82:                                               ; preds = %78, %69, %65, %61
  %83 = phi i64 [ %81, %78 ], [ %77, %69 ], [ %68, %65 ], [ %64, %61 ]
  %84 = or i64 %83, %55
  %85 = icmp sgt i64 %84, -1
  br i1 %85, label %86, label %618

86:                                               ; preds = %82
  %87 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 3
  %88 = load i64, ptr addrspace(1) %87, align 8
  %89 = load i64, ptr addrspace(1) %27, align 8
  switch i64 %89, label %91 [
    i64 0, label %92
    i64 20, label %92
    i64 1, label %90
    i64 21, label %90
    i64 6, label %90
  ]

90:                                               ; preds = %86, %86, %86
  br label %92

91:                                               ; preds = %86
  br label %92

92:                                               ; preds = %91, %90, %86, %86
  %93 = phi i32 [ 4, %91 ], [ 2, %90 ], [ 1, %86 ], [ 1, %86 ]
  %94 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 14
  %95 = load i64, ptr addrspace(1) %94, align 8
  %96 = icmp eq i64 %95, 0
  %97 = zext nneg i32 %93 to i64
  br i1 %96, label %105, label %98

98:                                               ; preds = %92
  %99 = load i64, ptr addrspace(1) %23, align 8
  %100 = mul nsw i64 %95, %83
  %101 = mul nsw i64 %88, %55
  %102 = mul nsw i64 %101, %97
  %103 = add nsw i64 %100, %102
  %104 = add i64 %103, %99
  br label %112

105:                                              ; preds = %92
  %106 = mul nsw i64 %83, %21
  %107 = add i64 %106, %55
  %108 = load i64, ptr addrspace(1) %23, align 8
  %109 = mul i64 %88, %107
  %110 = mul i64 %109, %97
  %111 = add i64 %108, %110
  br label %112

112:                                              ; preds = %105, %98
  %113 = phi i64 [ %104, %98 ], [ %111, %105 ]
  %114 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 7
  %115 = icmp eq i64 %89, 5
  %116 = sext i1 %115 to i32
  br label %117

117:                                              ; preds = %270, %112
  %118 = phi i32 [ 0, %112 ], [ %273, %270 ]
  %119 = phi <4 x double> [ zeroinitializer, %112 ], [ %272, %270 ]
  %120 = zext nneg i32 %118 to i64
  %121 = icmp sgt i64 %88, %120
  br i1 %121, label %122, label %267

122:                                              ; preds = %117
  %123 = mul nuw nsw i32 %118, %93
  %124 = zext nneg i32 %123 to i64
  %125 = add i64 %113, %124
  switch i64 %89, label %127 [
    i64 0, label %128
    i64 20, label %128
    i64 1, label %126
    i64 21, label %126
    i64 6, label %126
  ]

126:                                              ; preds = %122, %122, %122
  br label %128

127:                                              ; preds = %122
  br label %128

128:                                              ; preds = %127, %126, %122, %122
  %129 = phi i32 [ 4, %127 ], [ 2, %126 ], [ 1, %122 ], [ 1, %122 ]
  %130 = inttoptr i64 %125 to ptr addrspace(1)
  br label %132

131:                                              ; preds = %132
  switch i64 %89, label %235 [
    i64 6, label %144
    i64 7, label %231
    i64 0, label %236
    i64 20, label %236
    i64 1, label %234
    i64 21, label %234
  ]

132:                                              ; preds = %132, %128
  %133 = phi i32 [ 0, %128 ], [ %141, %132 ]
  %134 = phi i32 [ 0, %128 ], [ %142, %132 ]
  %135 = zext nneg i32 %134 to i64
  %136 = getelementptr inbounds i8, ptr addrspace(1) %130, i64 %135
  %137 = load i8, ptr addrspace(1) %136, align 1
  %138 = zext i8 %137 to i32
  %139 = shl nsw i32 %134, 3
  %140 = shl i32 %138, %139
  %141 = or i32 %140, %133
  %142 = add nuw nsw i32 %134, 1
  %143 = icmp eq i32 %142, %129
  br i1 %143, label %131, label %132, !llvm.loop !10016

144:                                              ; preds = %131
  %145 = lshr i32 %141, 10
  %146 = and i32 %145, 31
  %147 = and i32 %141, 1023
  switch i32 %146, label %154 [
    i32 0, label %148
    i32 31, label %151
  ]

148:                                              ; preds = %144
  %149 = uitofp i32 %147 to double
  %150 = fmul double %149, 0x3E70000000000000
  br label %226

151:                                              ; preds = %144
  %152 = icmp eq i32 %147, 0
  %153 = select i1 %152, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %226

154:                                              ; preds = %144
  %155 = or disjoint i32 %147, 1024
  %156 = uitofp i32 %155 to double
  %157 = add nsw i32 %146, -25
  %158 = icmp ugt i32 %146, 25
  br i1 %158, label %159, label %176

159:                                              ; preds = %154
  %160 = add nsw i32 %146, -26
  %161 = and i32 %157, 7
  %162 = icmp eq i32 %161, 0
  br i1 %162, label %171, label %163

163:                                              ; preds = %159, %163
  %164 = phi i32 [ %168, %163 ], [ %157, %159 ]
  %165 = phi double [ %167, %163 ], [ %156, %159 ]
  %166 = phi i32 [ %169, %163 ], [ 0, %159 ]
  %167 = fmul double %165, 2.000000e+00
  %168 = add nsw i32 %164, -1
  %169 = add i32 %166, 1
  %170 = icmp eq i32 %169, %161
  br i1 %170, label %171, label %163, !llvm.loop !10064

171:                                              ; preds = %163, %159
  %172 = phi i32 [ %157, %159 ], [ %168, %163 ]
  %173 = phi double [ %156, %159 ], [ %167, %163 ]
  %174 = phi double [ undef, %159 ], [ %167, %163 ]
  %175 = icmp ult i32 %160, 7
  br i1 %175, label %176, label %199

176:                                              ; preds = %171, %199, %154
  %177 = phi double [ %156, %154 ], [ %174, %171 ], [ %209, %199 ]
  %178 = phi i32 [ %157, %154 ], [ 0, %199 ], [ 0, %171 ]
  %179 = icmp slt i32 %178, 0
  br i1 %179, label %180, label %226

180:                                              ; preds = %176
  %181 = tail call i32 @llvm.umin.i32(i32 %146, i32 25)
  %182 = sub nsw i32 1, %181
  %183 = and i32 %182, 7
  %184 = icmp eq i32 %183, 0
  br i1 %184, label %193, label %185

185:                                              ; preds = %180, %185
  %186 = phi i32 [ %190, %185 ], [ %178, %180 ]
  %187 = phi double [ %189, %185 ], [ %177, %180 ]
  %188 = phi i32 [ %191, %185 ], [ 0, %180 ]
  %189 = fmul double %187, 5.000000e-01
  %190 = add nsw i32 %186, 1
  %191 = add i32 %188, 1
  %192 = icmp eq i32 %191, %183
  br i1 %192, label %193, label %185, !llvm.loop !10065

193:                                              ; preds = %185, %180
  %194 = phi double [ undef, %180 ], [ %189, %185 ]
  %195 = phi i32 [ %178, %180 ], [ %190, %185 ]
  %196 = phi double [ %177, %180 ], [ %189, %185 ]
  %197 = add nsw i32 %181, -18
  %198 = icmp ult i32 %197, 7
  br i1 %198, label %226, label %213

199:                                              ; preds = %171, %199
  %200 = phi i32 [ %210, %199 ], [ %172, %171 ]
  %201 = phi double [ %209, %199 ], [ %173, %171 ]
  %202 = fmul double %201, 2.000000e+00
  %203 = fmul double %202, 2.000000e+00
  %204 = fmul double %203, 2.000000e+00
  %205 = fmul double %204, 2.000000e+00
  %206 = fmul double %205, 2.000000e+00
  %207 = fmul double %206, 2.000000e+00
  %208 = fmul double %207, 2.000000e+00
  %209 = fmul double %208, 2.000000e+00
  %210 = add nsw i32 %200, -8
  %211 = add i32 %200, -9
  %212 = icmp ult i32 %211, -2
  br i1 %212, label %199, label %176, !llvm.loop !10021

213:                                              ; preds = %193, %213
  %214 = phi i32 [ %224, %213 ], [ %195, %193 ]
  %215 = phi double [ %223, %213 ], [ %196, %193 ]
  %216 = fmul double %215, 5.000000e-01
  %217 = fmul double %216, 5.000000e-01
  %218 = fmul double %217, 5.000000e-01
  %219 = fmul double %218, 5.000000e-01
  %220 = fmul double %219, 5.000000e-01
  %221 = fmul double %220, 5.000000e-01
  %222 = fmul double %221, 5.000000e-01
  %223 = fmul double %222, 5.000000e-01
  %224 = add nsw i32 %214, 8
  %225 = icmp eq i32 %224, 0
  br i1 %225, label %226, label %213, !llvm.loop !10022

226:                                              ; preds = %193, %213, %176, %151, %148
  %227 = phi double [ %150, %148 ], [ %153, %151 ], [ %177, %176 ], [ %194, %193 ], [ %223, %213 ]
  %228 = icmp ult i32 %141, 32768
  %229 = fneg double %227
  %230 = select i1 %228, double %227, double %229
  br label %270

231:                                              ; preds = %131
  %232 = bitcast i32 %141 to float
  %233 = fpext float %232 to double
  br label %270

234:                                              ; preds = %131, %131
  br label %236

235:                                              ; preds = %131
  br label %236

236:                                              ; preds = %235, %234, %131, %131
  %237 = phi i32 [ 32, %235 ], [ 16, %234 ], [ 8, %131 ], [ 8, %131 ]
  %238 = zext i32 %141 to i64
  switch i64 %89, label %248 [
    i64 21, label %239
    i64 20, label %239
    i64 3, label %239
    i64 5, label %239
  ]

239:                                              ; preds = %236, %236, %236, %236
  %240 = add nsw i32 %237, -1
  %241 = shl nuw i32 1, %240
  %242 = and i32 %241, %141
  %243 = icmp eq i32 %242, 0
  %244 = zext nneg i32 %237 to i64
  %245 = shl nsw i64 -1, %244
  %246 = select i1 %243, i64 0, i64 %245
  %247 = add nsw i64 %246, %238
  br label %248

248:                                              ; preds = %239, %236
  %249 = phi i64 [ %247, %239 ], [ %238, %236 ]
  %250 = load i64, ptr addrspace(1) %114, align 8
  %251 = icmp eq i64 %250, 0
  br i1 %251, label %252, label %254

252:                                              ; preds = %248
  %253 = sitofp i64 %249 to double
  br label %270

254:                                              ; preds = %248
  switch i64 %89, label %255 [
    i64 21, label %256
    i64 20, label %256
    i64 3, label %256
  ]

255:                                              ; preds = %254
  br label %256

256:                                              ; preds = %255, %254, %254, %254
  %257 = phi i32 [ -1, %254 ], [ %116, %255 ], [ -1, %254 ], [ -1, %254 ]
  %258 = add nsw i32 %257, %237
  %259 = zext nneg i32 %258 to i64
  %260 = shl nsw i64 -1, %259
  %261 = xor i64 %260, -1
  %262 = sitofp i64 %249 to double
  %263 = sitofp i64 %261 to double
  %264 = fdiv double %262, %263
  %265 = fcmp oge double %264, -1.000000e+00
  %266 = select i1 %265, double %264, double -1.000000e+00
  br label %270

267:                                              ; preds = %117
  %268 = icmp eq i32 %118, 3
  %269 = select i1 %268, double 1.000000e+00, double 0.000000e+00
  br label %270

270:                                              ; preds = %267, %256, %252, %231, %226
  %271 = phi double [ %269, %267 ], [ %230, %226 ], [ %233, %231 ], [ %266, %256 ], [ %253, %252 ]
  %272 = insertelement <4 x double> %119, double %271, i32 %118
  %273 = add nuw nsw i32 %118, 1
  %274 = icmp eq i32 %273, 4
  br i1 %274, label %618, label %117, !llvm.loop !10029

275:                                              ; preds = %2
  %276 = sitofp i64 %21 to double
  %277 = select i1 %12, double %276, double 1.000000e+00
  %278 = fmul double %277, %15
  %279 = fadd double %278, -5.000000e-01
  %280 = tail call double @llvm.floor.f64(double %279)
  %281 = fptosi double %280 to i64
  switch i32 %13, label %299 [
    i32 0, label %282
    i32 2, label %286
    i32 3, label %290
  ]

282:                                              ; preds = %275
  %283 = srem i64 %281, %21
  %284 = add nsw i64 %283, %21
  %285 = srem i64 %284, %21
  br label %303

286:                                              ; preds = %275
  %287 = icmp sgt i64 %21, %281
  %288 = tail call i64 @llvm.smax.i64(i64 %281, i64 -1)
  %289 = select i1 %287, i64 %288, i64 -1
  br label %303

290:                                              ; preds = %275
  %291 = shl nuw nsw i64 %21, 1
  %292 = srem i64 %281, %291
  %293 = add nsw i64 %292, %291
  %294 = srem i64 %293, %291
  %295 = icmp slt i64 %294, %21
  %296 = xor i64 %294, -1
  %297 = add i64 %291, %296
  %298 = select i1 %295, i64 %294, i64 %297
  br label %303

299:                                              ; preds = %275
  %300 = tail call noundef i64 @llvm.smax.i64(i64 %281, i64 0)
  %301 = add nsw i64 %21, -1
  %302 = tail call noundef i64 @llvm.smin.i64(i64 %300, i64 %301)
  br label %303

303:                                              ; preds = %299, %290, %286, %282
  %304 = phi i64 [ %302, %299 ], [ %298, %290 ], [ %289, %286 ], [ %285, %282 ]
  %305 = add i64 %281, 1
  switch i32 %13, label %323 [
    i32 0, label %306
    i32 2, label %310
    i32 3, label %314
  ]

306:                                              ; preds = %303
  %307 = srem i64 %305, %21
  %308 = add nsw i64 %307, %21
  %309 = srem i64 %308, %21
  br label %327

310:                                              ; preds = %303
  %311 = icmp slt i64 %305, %21
  %312 = tail call i64 @llvm.smax.i64(i64 %305, i64 -1)
  %313 = select i1 %311, i64 %312, i64 -1
  br label %327

314:                                              ; preds = %303
  %315 = shl nuw nsw i64 %21, 1
  %316 = srem i64 %305, %315
  %317 = add nsw i64 %316, %315
  %318 = srem i64 %317, %315
  %319 = icmp slt i64 %318, %21
  %320 = xor i64 %318, -1
  %321 = add i64 %315, %320
  %322 = select i1 %319, i64 %318, i64 %321
  br label %327

323:                                              ; preds = %303
  %324 = tail call noundef i64 @llvm.smax.i64(i64 %305, i64 0)
  %325 = add nsw i64 %21, -1
  %326 = tail call noundef i64 @llvm.smin.i64(i64 %324, i64 %325)
  br label %327

327:                                              ; preds = %323, %314, %310, %306
  %328 = phi i64 [ %326, %323 ], [ %322, %314 ], [ %313, %310 ], [ %309, %306 ]
  %329 = getelementptr i64, ptr addrspace(1) %3, i64 10
  %330 = load i64, ptr addrspace(1) %329, align 8
  %331 = tail call noundef i64 @llvm.smax.i64(i64 %330, i64 1)
  %332 = sitofp i64 %331 to double
  %333 = select i1 %12, double %332, double 1.000000e+00
  %334 = fmul double %333, %17
  %335 = fadd double %334, -5.000000e-01
  %336 = tail call double @llvm.floor.f64(double %335)
  %337 = fptosi double %336 to i64
  switch i32 %13, label %355 [
    i32 0, label %338
    i32 2, label %342
    i32 3, label %346
  ]

338:                                              ; preds = %327
  %339 = srem i64 %337, %331
  %340 = add nsw i64 %339, %331
  %341 = srem i64 %340, %331
  br label %359

342:                                              ; preds = %327
  %343 = icmp sgt i64 %331, %337
  %344 = tail call i64 @llvm.smax.i64(i64 %337, i64 -1)
  %345 = select i1 %343, i64 %344, i64 -1
  br label %359

346:                                              ; preds = %327
  %347 = shl nuw nsw i64 %331, 1
  %348 = srem i64 %337, %347
  %349 = add nsw i64 %348, %347
  %350 = srem i64 %349, %347
  %351 = icmp slt i64 %350, %331
  %352 = xor i64 %350, -1
  %353 = add i64 %347, %352
  %354 = select i1 %351, i64 %350, i64 %353
  br label %359

355:                                              ; preds = %327
  %356 = tail call noundef i64 @llvm.smax.i64(i64 %337, i64 0)
  %357 = add nsw i64 %331, -1
  %358 = tail call noundef i64 @llvm.smin.i64(i64 %356, i64 %357)
  br label %359

359:                                              ; preds = %355, %346, %342, %338
  %360 = phi i64 [ %358, %355 ], [ %354, %346 ], [ %345, %342 ], [ %341, %338 ]
  %361 = add i64 %337, 1
  switch i32 %13, label %379 [
    i32 0, label %362
    i32 2, label %366
    i32 3, label %370
  ]

362:                                              ; preds = %359
  %363 = srem i64 %361, %331
  %364 = add nsw i64 %363, %331
  %365 = srem i64 %364, %331
  br label %383

366:                                              ; preds = %359
  %367 = icmp slt i64 %361, %331
  %368 = tail call i64 @llvm.smax.i64(i64 %361, i64 -1)
  %369 = select i1 %367, i64 %368, i64 -1
  br label %383

370:                                              ; preds = %359
  %371 = shl nuw nsw i64 %331, 1
  %372 = srem i64 %361, %371
  %373 = add nsw i64 %372, %371
  %374 = srem i64 %373, %371
  %375 = icmp slt i64 %374, %331
  %376 = xor i64 %374, -1
  %377 = add i64 %371, %376
  %378 = select i1 %375, i64 %374, i64 %377
  br label %383

379:                                              ; preds = %359
  %380 = tail call noundef i64 @llvm.smax.i64(i64 %361, i64 0)
  %381 = add nsw i64 %331, -1
  %382 = tail call noundef i64 @llvm.smin.i64(i64 %380, i64 %381)
  br label %383

383:                                              ; preds = %379, %370, %366, %362
  %384 = phi i64 [ %382, %379 ], [ %378, %370 ], [ %369, %366 ], [ %365, %362 ]
  %385 = fsub double %279, %280
  %386 = fmul double %385, 2.560000e+02
  %387 = fadd double %386, 5.000000e-01
  %388 = tail call double @llvm.floor.f64(double %387)
  %389 = fptosi double %388 to i64
  %390 = sitofp i64 %389 to double
  %391 = fmul double %390, 3.906250e-03
  %392 = fsub double %335, %336
  %393 = fmul double %392, 2.560000e+02
  %394 = fadd double %393, 5.000000e-01
  %395 = tail call double @llvm.floor.f64(double %394)
  %396 = fptosi double %395 to i64
  %397 = sitofp i64 %396 to double
  %398 = fmul double %397, 3.906250e-03
  %399 = fsub double 1.000000e+00, %391
  %400 = fsub double 1.000000e+00, %398
  %401 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 3
  %402 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 2
  %403 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 14
  %404 = getelementptr i64, ptr addrspace(1) %3, i64 1
  %405 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 7
  br label %406

406:                                              ; preds = %612, %383
  %407 = phi <4 x double> [ zeroinitializer, %383 ], [ %615, %612 ]
  %408 = phi i32 [ 0, %383 ], [ %616, %612 ]
  %409 = and i32 %408, 1
  %410 = icmp eq i32 %409, 0
  %411 = select i1 %410, double %399, double %391
  %412 = and i32 %408, 2
  %413 = icmp eq i32 %412, 0
  %414 = select i1 %413, double %400, double %398
  %415 = fmul double %411, %414
  %416 = and i32 %408, 4
  %417 = icmp eq i32 %416, 0
  %418 = select i1 %417, double 1.000000e+00, double 0.000000e+00
  %419 = fmul double %418, %415
  %420 = insertelement <4 x double> poison, double %419, i64 0
  %421 = shufflevector <4 x double> %420, <4 x double> poison, <4 x i32> zeroinitializer
  %422 = select i1 %410, i64 %304, i64 %328
  %423 = select i1 %413, i64 %360, i64 %384
  %424 = or i64 %423, %422
  %425 = icmp sgt i64 %424, -1
  br i1 %425, label %426, label %612

426:                                              ; preds = %406
  %427 = load i64, ptr addrspace(1) %401, align 8
  %428 = load i64, ptr addrspace(1) %402, align 8
  switch i64 %428, label %430 [
    i64 0, label %431
    i64 20, label %431
    i64 1, label %429
    i64 21, label %429
    i64 6, label %429
  ]

429:                                              ; preds = %426, %426, %426
  br label %431

430:                                              ; preds = %426
  br label %431

431:                                              ; preds = %430, %429, %426, %426
  %432 = phi i32 [ 4, %430 ], [ 2, %429 ], [ 1, %426 ], [ 1, %426 ]
  %433 = load i64, ptr addrspace(1) %403, align 8
  %434 = icmp eq i64 %433, 0
  %435 = zext nneg i32 %432 to i64
  br i1 %434, label %443, label %436

436:                                              ; preds = %431
  %437 = load i64, ptr addrspace(1) %404, align 8
  %438 = mul nsw i64 %433, %423
  %439 = mul nsw i64 %427, %422
  %440 = mul nsw i64 %439, %435
  %441 = add nsw i64 %438, %440
  %442 = add i64 %441, %437
  br label %450

443:                                              ; preds = %431
  %444 = mul nsw i64 %423, %21
  %445 = add i64 %444, %422
  %446 = load i64, ptr addrspace(1) %404, align 8
  %447 = mul i64 %427, %445
  %448 = mul i64 %447, %435
  %449 = add i64 %446, %448
  br label %450

450:                                              ; preds = %443, %436
  %451 = phi i64 [ %442, %436 ], [ %449, %443 ]
  %452 = icmp eq i64 %428, 5
  %453 = sext i1 %452 to i32
  br label %454

454:                                              ; preds = %607, %450
  %455 = phi i32 [ 0, %450 ], [ %610, %607 ]
  %456 = phi <4 x double> [ zeroinitializer, %450 ], [ %609, %607 ]
  %457 = zext nneg i32 %455 to i64
  %458 = icmp sgt i64 %427, %457
  br i1 %458, label %459, label %604

459:                                              ; preds = %454
  %460 = mul nuw nsw i32 %455, %432
  %461 = zext nneg i32 %460 to i64
  %462 = add i64 %451, %461
  switch i64 %428, label %464 [
    i64 0, label %465
    i64 20, label %465
    i64 1, label %463
    i64 21, label %463
    i64 6, label %463
  ]

463:                                              ; preds = %459, %459, %459
  br label %465

464:                                              ; preds = %459
  br label %465

465:                                              ; preds = %464, %463, %459, %459
  %466 = phi i32 [ 4, %464 ], [ 2, %463 ], [ 1, %459 ], [ 1, %459 ]
  %467 = inttoptr i64 %462 to ptr addrspace(1)
  br label %469

468:                                              ; preds = %469
  switch i64 %428, label %572 [
    i64 6, label %481
    i64 7, label %568
    i64 0, label %573
    i64 20, label %573
    i64 1, label %571
    i64 21, label %571
  ]

469:                                              ; preds = %469, %465
  %470 = phi i32 [ 0, %465 ], [ %478, %469 ]
  %471 = phi i32 [ 0, %465 ], [ %479, %469 ]
  %472 = zext nneg i32 %471 to i64
  %473 = getelementptr inbounds i8, ptr addrspace(1) %467, i64 %472
  %474 = load i8, ptr addrspace(1) %473, align 1
  %475 = zext i8 %474 to i32
  %476 = shl nsw i32 %471, 3
  %477 = shl i32 %475, %476
  %478 = or i32 %477, %470
  %479 = add nuw nsw i32 %471, 1
  %480 = icmp eq i32 %479, %466
  br i1 %480, label %468, label %469, !llvm.loop !10016

481:                                              ; preds = %468
  %482 = lshr i32 %478, 10
  %483 = and i32 %482, 31
  %484 = and i32 %478, 1023
  switch i32 %483, label %491 [
    i32 0, label %485
    i32 31, label %488
  ]

485:                                              ; preds = %481
  %486 = uitofp i32 %484 to double
  %487 = fmul double %486, 0x3E70000000000000
  br label %563

488:                                              ; preds = %481
  %489 = icmp eq i32 %484, 0
  %490 = select i1 %489, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %563

491:                                              ; preds = %481
  %492 = or disjoint i32 %484, 1024
  %493 = uitofp i32 %492 to double
  %494 = add nsw i32 %483, -25
  %495 = icmp ugt i32 %483, 25
  br i1 %495, label %496, label %513

496:                                              ; preds = %491
  %497 = add nsw i32 %483, -26
  %498 = and i32 %494, 7
  %499 = icmp eq i32 %498, 0
  br i1 %499, label %508, label %500

500:                                              ; preds = %496, %500
  %501 = phi i32 [ %505, %500 ], [ %494, %496 ]
  %502 = phi double [ %504, %500 ], [ %493, %496 ]
  %503 = phi i32 [ %506, %500 ], [ 0, %496 ]
  %504 = fmul double %502, 2.000000e+00
  %505 = add nsw i32 %501, -1
  %506 = add i32 %503, 1
  %507 = icmp eq i32 %506, %498
  br i1 %507, label %508, label %500, !llvm.loop !10066

508:                                              ; preds = %500, %496
  %509 = phi i32 [ %494, %496 ], [ %505, %500 ]
  %510 = phi double [ %493, %496 ], [ %504, %500 ]
  %511 = phi double [ undef, %496 ], [ %504, %500 ]
  %512 = icmp ult i32 %497, 7
  br i1 %512, label %513, label %536

513:                                              ; preds = %508, %536, %491
  %514 = phi double [ %493, %491 ], [ %511, %508 ], [ %546, %536 ]
  %515 = phi i32 [ %494, %491 ], [ 0, %536 ], [ 0, %508 ]
  %516 = icmp slt i32 %515, 0
  br i1 %516, label %517, label %563

517:                                              ; preds = %513
  %518 = tail call i32 @llvm.umin.i32(i32 %483, i32 25)
  %519 = sub nsw i32 1, %518
  %520 = and i32 %519, 7
  %521 = icmp eq i32 %520, 0
  br i1 %521, label %530, label %522

522:                                              ; preds = %517, %522
  %523 = phi i32 [ %527, %522 ], [ %515, %517 ]
  %524 = phi double [ %526, %522 ], [ %514, %517 ]
  %525 = phi i32 [ %528, %522 ], [ 0, %517 ]
  %526 = fmul double %524, 5.000000e-01
  %527 = add nsw i32 %523, 1
  %528 = add i32 %525, 1
  %529 = icmp eq i32 %528, %520
  br i1 %529, label %530, label %522, !llvm.loop !10067

530:                                              ; preds = %522, %517
  %531 = phi double [ undef, %517 ], [ %526, %522 ]
  %532 = phi i32 [ %515, %517 ], [ %527, %522 ]
  %533 = phi double [ %514, %517 ], [ %526, %522 ]
  %534 = add nsw i32 %518, -18
  %535 = icmp ult i32 %534, 7
  br i1 %535, label %563, label %550

536:                                              ; preds = %508, %536
  %537 = phi i32 [ %547, %536 ], [ %509, %508 ]
  %538 = phi double [ %546, %536 ], [ %510, %508 ]
  %539 = fmul double %538, 2.000000e+00
  %540 = fmul double %539, 2.000000e+00
  %541 = fmul double %540, 2.000000e+00
  %542 = fmul double %541, 2.000000e+00
  %543 = fmul double %542, 2.000000e+00
  %544 = fmul double %543, 2.000000e+00
  %545 = fmul double %544, 2.000000e+00
  %546 = fmul double %545, 2.000000e+00
  %547 = add nsw i32 %537, -8
  %548 = add i32 %537, -9
  %549 = icmp ult i32 %548, -2
  br i1 %549, label %536, label %513, !llvm.loop !10021

550:                                              ; preds = %530, %550
  %551 = phi i32 [ %561, %550 ], [ %532, %530 ]
  %552 = phi double [ %560, %550 ], [ %533, %530 ]
  %553 = fmul double %552, 5.000000e-01
  %554 = fmul double %553, 5.000000e-01
  %555 = fmul double %554, 5.000000e-01
  %556 = fmul double %555, 5.000000e-01
  %557 = fmul double %556, 5.000000e-01
  %558 = fmul double %557, 5.000000e-01
  %559 = fmul double %558, 5.000000e-01
  %560 = fmul double %559, 5.000000e-01
  %561 = add nsw i32 %551, 8
  %562 = icmp eq i32 %561, 0
  br i1 %562, label %563, label %550, !llvm.loop !10022

563:                                              ; preds = %530, %550, %513, %488, %485
  %564 = phi double [ %487, %485 ], [ %490, %488 ], [ %514, %513 ], [ %531, %530 ], [ %560, %550 ]
  %565 = icmp ult i32 %478, 32768
  %566 = fneg double %564
  %567 = select i1 %565, double %564, double %566
  br label %607

568:                                              ; preds = %468
  %569 = bitcast i32 %478 to float
  %570 = fpext float %569 to double
  br label %607

571:                                              ; preds = %468, %468
  br label %573

572:                                              ; preds = %468
  br label %573

573:                                              ; preds = %572, %571, %468, %468
  %574 = phi i32 [ 32, %572 ], [ 16, %571 ], [ 8, %468 ], [ 8, %468 ]
  %575 = zext i32 %478 to i64
  switch i64 %428, label %585 [
    i64 21, label %576
    i64 20, label %576
    i64 3, label %576
    i64 5, label %576
  ]

576:                                              ; preds = %573, %573, %573, %573
  %577 = add nsw i32 %574, -1
  %578 = shl nuw i32 1, %577
  %579 = and i32 %578, %478
  %580 = icmp eq i32 %579, 0
  %581 = zext nneg i32 %574 to i64
  %582 = shl nsw i64 -1, %581
  %583 = select i1 %580, i64 0, i64 %582
  %584 = add nsw i64 %583, %575
  br label %585

585:                                              ; preds = %576, %573
  %586 = phi i64 [ %584, %576 ], [ %575, %573 ]
  %587 = load i64, ptr addrspace(1) %405, align 8
  %588 = icmp eq i64 %587, 0
  br i1 %588, label %589, label %591

589:                                              ; preds = %585
  %590 = sitofp i64 %586 to double
  br label %607

591:                                              ; preds = %585
  switch i64 %428, label %592 [
    i64 21, label %593
    i64 20, label %593
    i64 3, label %593
  ]

592:                                              ; preds = %591
  br label %593

593:                                              ; preds = %592, %591, %591, %591
  %594 = phi i32 [ -1, %591 ], [ %453, %592 ], [ -1, %591 ], [ -1, %591 ]
  %595 = add nsw i32 %594, %574
  %596 = zext nneg i32 %595 to i64
  %597 = shl nsw i64 -1, %596
  %598 = xor i64 %597, -1
  %599 = sitofp i64 %586 to double
  %600 = sitofp i64 %598 to double
  %601 = fdiv double %599, %600
  %602 = fcmp oge double %601, -1.000000e+00
  %603 = select i1 %602, double %601, double -1.000000e+00
  br label %607

604:                                              ; preds = %454
  %605 = icmp eq i32 %455, 3
  %606 = select i1 %605, double 1.000000e+00, double 0.000000e+00
  br label %607

607:                                              ; preds = %604, %593, %589, %568, %563
  %608 = phi double [ %606, %604 ], [ %567, %563 ], [ %570, %568 ], [ %603, %593 ], [ %590, %589 ]
  %609 = insertelement <4 x double> %456, double %608, i32 %455
  %610 = add nuw nsw i32 %455, 1
  %611 = icmp eq i32 %610, 4
  br i1 %611, label %612, label %454, !llvm.loop !10029

612:                                              ; preds = %607, %406
  %613 = phi <4 x double> [ zeroinitializer, %406 ], [ %609, %607 ]
  %614 = fmul <4 x double> %421, %613
  %615 = fadd <4 x double> %407, %614
  %616 = add nuw nsw i32 %408, 1
  %617 = icmp eq i32 %616, 4
  br i1 %617, label %618, label %406, !llvm.loop !10032

618:                                              ; preds = %270, %612, %82
  %619 = phi <4 x double> [ zeroinitializer, %82 ], [ %615, %612 ], [ %272, %270 ]
  %620 = extractelement <4 x double> %619, i64 0
  %621 = fptrunc double %620 to float
  %622 = insertelement <4 x float> poison, float %621, i64 0
  %623 = extractelement <4 x double> %619, i64 1
  %624 = fptrunc double %623 to float
  %625 = insertelement <4 x float> %622, float %624, i64 1
  %626 = extractelement <4 x double> %619, i64 2
  %627 = fptrunc double %626 to float
  %628 = insertelement <4 x float> %625, float %627, i64 2
  %629 = extractelement <4 x double> %619, i64 3
  %630 = fptrunc double %629 to float
  %631 = insertelement <4 x float> %628, float %630, i64 3
  ret <4 x float> %631
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define dso_local <4 x float> @__zluda_ptx_impl_texobj_2d_v4_f32_s32(i64 noundef %0, <2 x i32> noundef %1) local_unnamed_addr #3 {
  %3 = extractelement <2 x i32> %1, i64 0
  %4 = inttoptr i64 %0 to ptr addrspace(1)
  %5 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 5
  %6 = load i64, ptr addrspace(1) %5, align 8
  %7 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 6
  %8 = load i64, ptr addrspace(1) %7, align 8
  %9 = getelementptr i64, ptr addrspace(1) %4, i64 9
  %10 = and i64 %8, 4294967295
  %11 = icmp eq i64 %10, 1
  %12 = extractelement <2 x i32> %1, i64 1
  %13 = trunc i64 %6 to i32
  %14 = sitofp i32 %3 to double
  %15 = sitofp i32 %12 to double
  %16 = load i64, ptr addrspace(1) %9, align 8
  %17 = tail call noundef i64 @llvm.smax.i64(i64 %16, i64 1)
  %18 = getelementptr i64, ptr addrspace(1) %4, i64 1
  %19 = getelementptr i64, ptr addrspace(1) %4, i64 10
  %20 = load i64, ptr addrspace(1) %19, align 8
  %21 = tail call noundef i64 @llvm.smax.i64(i64 %20, i64 1)
  %22 = getelementptr i64, ptr addrspace(1) %4, i64 2
  %23 = sitofp i64 %17 to double
  %24 = select i1 %11, double %23, double 1.000000e+00
  %25 = fmul double %24, %14
  %26 = tail call double @llvm.floor.f64(double %25)
  %27 = fptosi double %26 to i64
  switch i32 %13, label %45 [
    i32 0, label %28
    i32 2, label %32
    i32 3, label %36
  ]

28:                                               ; preds = %2
  %29 = srem i64 %27, %17
  %30 = add nsw i64 %29, %17
  %31 = srem i64 %30, %17
  br label %49

32:                                               ; preds = %2
  %33 = icmp sgt i64 %17, %27
  %34 = tail call i64 @llvm.smax.i64(i64 %27, i64 -1)
  %35 = select i1 %33, i64 %34, i64 -1
  br label %49

36:                                               ; preds = %2
  %37 = shl nuw nsw i64 %17, 1
  %38 = srem i64 %27, %37
  %39 = add nsw i64 %38, %37
  %40 = srem i64 %39, %37
  %41 = icmp slt i64 %40, %17
  %42 = xor i64 %40, -1
  %43 = add i64 %37, %42
  %44 = select i1 %41, i64 %40, i64 %43
  br label %49

45:                                               ; preds = %2
  %46 = tail call noundef i64 @llvm.smax.i64(i64 %27, i64 0)
  %47 = add nsw i64 %17, -1
  %48 = tail call noundef i64 @llvm.smin.i64(i64 %46, i64 %47)
  br label %49

49:                                               ; preds = %45, %36, %32, %28
  %50 = phi i64 [ %48, %45 ], [ %44, %36 ], [ %35, %32 ], [ %31, %28 ]
  %51 = sitofp i64 %21 to double
  %52 = select i1 %11, double %51, double 1.000000e+00
  %53 = fmul double %52, %15
  %54 = tail call double @llvm.floor.f64(double %53)
  %55 = fptosi double %54 to i64
  switch i32 %13, label %73 [
    i32 0, label %56
    i32 2, label %60
    i32 3, label %64
  ]

56:                                               ; preds = %49
  %57 = srem i64 %55, %21
  %58 = add nsw i64 %57, %21
  %59 = srem i64 %58, %21
  br label %77

60:                                               ; preds = %49
  %61 = icmp sgt i64 %21, %55
  %62 = tail call i64 @llvm.smax.i64(i64 %55, i64 -1)
  %63 = select i1 %61, i64 %62, i64 -1
  br label %77

64:                                               ; preds = %49
  %65 = shl nuw nsw i64 %21, 1
  %66 = srem i64 %55, %65
  %67 = add nsw i64 %66, %65
  %68 = srem i64 %67, %65
  %69 = icmp slt i64 %68, %21
  %70 = xor i64 %68, -1
  %71 = add i64 %65, %70
  %72 = select i1 %69, i64 %68, i64 %71
  br label %77

73:                                               ; preds = %49
  %74 = tail call noundef i64 @llvm.smax.i64(i64 %55, i64 0)
  %75 = add nsw i64 %21, -1
  %76 = tail call noundef i64 @llvm.smin.i64(i64 %74, i64 %75)
  br label %77

77:                                               ; preds = %73, %64, %60, %56
  %78 = phi i64 [ %76, %73 ], [ %72, %64 ], [ %63, %60 ], [ %59, %56 ]
  %79 = or i64 %78, %50
  %80 = icmp sgt i64 %79, -1
  br i1 %80, label %81, label %270

81:                                               ; preds = %77
  %82 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 3
  %83 = load i64, ptr addrspace(1) %82, align 8
  %84 = load i64, ptr addrspace(1) %22, align 8
  switch i64 %84, label %86 [
    i64 0, label %87
    i64 20, label %87
    i64 1, label %85
    i64 21, label %85
    i64 6, label %85
  ]

85:                                               ; preds = %81, %81, %81
  br label %87

86:                                               ; preds = %81
  br label %87

87:                                               ; preds = %86, %85, %81, %81
  %88 = phi i32 [ 4, %86 ], [ 2, %85 ], [ 1, %81 ], [ 1, %81 ]
  %89 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 14
  %90 = load i64, ptr addrspace(1) %89, align 8
  %91 = icmp eq i64 %90, 0
  %92 = zext nneg i32 %88 to i64
  br i1 %91, label %100, label %93

93:                                               ; preds = %87
  %94 = load i64, ptr addrspace(1) %18, align 8
  %95 = mul nsw i64 %90, %78
  %96 = mul nsw i64 %83, %50
  %97 = mul nsw i64 %96, %92
  %98 = add nsw i64 %95, %97
  %99 = add i64 %98, %94
  br label %107

100:                                              ; preds = %87
  %101 = mul nsw i64 %78, %17
  %102 = add i64 %101, %50
  %103 = load i64, ptr addrspace(1) %18, align 8
  %104 = mul i64 %83, %102
  %105 = mul i64 %104, %92
  %106 = add i64 %103, %105
  br label %107

107:                                              ; preds = %100, %93
  %108 = phi i64 [ %99, %93 ], [ %106, %100 ]
  %109 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 7
  %110 = icmp eq i64 %84, 5
  %111 = sext i1 %110 to i32
  br label %112

112:                                              ; preds = %265, %107
  %113 = phi i32 [ 0, %107 ], [ %268, %265 ]
  %114 = phi <4 x double> [ zeroinitializer, %107 ], [ %267, %265 ]
  %115 = zext nneg i32 %113 to i64
  %116 = icmp sgt i64 %83, %115
  br i1 %116, label %117, label %262

117:                                              ; preds = %112
  %118 = mul nuw nsw i32 %113, %88
  %119 = zext nneg i32 %118 to i64
  %120 = add i64 %108, %119
  switch i64 %84, label %122 [
    i64 0, label %123
    i64 20, label %123
    i64 1, label %121
    i64 21, label %121
    i64 6, label %121
  ]

121:                                              ; preds = %117, %117, %117
  br label %123

122:                                              ; preds = %117
  br label %123

123:                                              ; preds = %122, %121, %117, %117
  %124 = phi i32 [ 4, %122 ], [ 2, %121 ], [ 1, %117 ], [ 1, %117 ]
  %125 = inttoptr i64 %120 to ptr addrspace(1)
  br label %127

126:                                              ; preds = %127
  switch i64 %84, label %230 [
    i64 6, label %139
    i64 7, label %226
    i64 0, label %231
    i64 20, label %231
    i64 1, label %229
    i64 21, label %229
  ]

127:                                              ; preds = %127, %123
  %128 = phi i32 [ 0, %123 ], [ %136, %127 ]
  %129 = phi i32 [ 0, %123 ], [ %137, %127 ]
  %130 = zext nneg i32 %129 to i64
  %131 = getelementptr inbounds i8, ptr addrspace(1) %125, i64 %130
  %132 = load i8, ptr addrspace(1) %131, align 1
  %133 = zext i8 %132 to i32
  %134 = shl nsw i32 %129, 3
  %135 = shl i32 %133, %134
  %136 = or i32 %135, %128
  %137 = add nuw nsw i32 %129, 1
  %138 = icmp eq i32 %137, %124
  br i1 %138, label %126, label %127, !llvm.loop !10016

139:                                              ; preds = %126
  %140 = lshr i32 %136, 10
  %141 = and i32 %140, 31
  %142 = and i32 %136, 1023
  switch i32 %141, label %149 [
    i32 0, label %143
    i32 31, label %146
  ]

143:                                              ; preds = %139
  %144 = uitofp i32 %142 to double
  %145 = fmul double %144, 0x3E70000000000000
  br label %221

146:                                              ; preds = %139
  %147 = icmp eq i32 %142, 0
  %148 = select i1 %147, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %221

149:                                              ; preds = %139
  %150 = or disjoint i32 %142, 1024
  %151 = uitofp i32 %150 to double
  %152 = add nsw i32 %141, -25
  %153 = icmp ugt i32 %141, 25
  br i1 %153, label %154, label %171

154:                                              ; preds = %149
  %155 = add nsw i32 %141, -26
  %156 = and i32 %152, 7
  %157 = icmp eq i32 %156, 0
  br i1 %157, label %166, label %158

158:                                              ; preds = %154, %158
  %159 = phi i32 [ %163, %158 ], [ %152, %154 ]
  %160 = phi double [ %162, %158 ], [ %151, %154 ]
  %161 = phi i32 [ %164, %158 ], [ 0, %154 ]
  %162 = fmul double %160, 2.000000e+00
  %163 = add nsw i32 %159, -1
  %164 = add i32 %161, 1
  %165 = icmp eq i32 %164, %156
  br i1 %165, label %166, label %158, !llvm.loop !10068

166:                                              ; preds = %158, %154
  %167 = phi i32 [ %152, %154 ], [ %163, %158 ]
  %168 = phi double [ %151, %154 ], [ %162, %158 ]
  %169 = phi double [ undef, %154 ], [ %162, %158 ]
  %170 = icmp ult i32 %155, 7
  br i1 %170, label %171, label %194

171:                                              ; preds = %166, %194, %149
  %172 = phi double [ %151, %149 ], [ %169, %166 ], [ %204, %194 ]
  %173 = phi i32 [ %152, %149 ], [ 0, %194 ], [ 0, %166 ]
  %174 = icmp slt i32 %173, 0
  br i1 %174, label %175, label %221

175:                                              ; preds = %171
  %176 = tail call i32 @llvm.umin.i32(i32 %141, i32 25)
  %177 = sub nsw i32 1, %176
  %178 = and i32 %177, 7
  %179 = icmp eq i32 %178, 0
  br i1 %179, label %188, label %180

180:                                              ; preds = %175, %180
  %181 = phi i32 [ %185, %180 ], [ %173, %175 ]
  %182 = phi double [ %184, %180 ], [ %172, %175 ]
  %183 = phi i32 [ %186, %180 ], [ 0, %175 ]
  %184 = fmul double %182, 5.000000e-01
  %185 = add nsw i32 %181, 1
  %186 = add i32 %183, 1
  %187 = icmp eq i32 %186, %178
  br i1 %187, label %188, label %180, !llvm.loop !10069

188:                                              ; preds = %180, %175
  %189 = phi double [ undef, %175 ], [ %184, %180 ]
  %190 = phi i32 [ %173, %175 ], [ %185, %180 ]
  %191 = phi double [ %172, %175 ], [ %184, %180 ]
  %192 = add nsw i32 %176, -18
  %193 = icmp ult i32 %192, 7
  br i1 %193, label %221, label %208

194:                                              ; preds = %166, %194
  %195 = phi i32 [ %205, %194 ], [ %167, %166 ]
  %196 = phi double [ %204, %194 ], [ %168, %166 ]
  %197 = fmul double %196, 2.000000e+00
  %198 = fmul double %197, 2.000000e+00
  %199 = fmul double %198, 2.000000e+00
  %200 = fmul double %199, 2.000000e+00
  %201 = fmul double %200, 2.000000e+00
  %202 = fmul double %201, 2.000000e+00
  %203 = fmul double %202, 2.000000e+00
  %204 = fmul double %203, 2.000000e+00
  %205 = add nsw i32 %195, -8
  %206 = add i32 %195, -9
  %207 = icmp ult i32 %206, -2
  br i1 %207, label %194, label %171, !llvm.loop !10021

208:                                              ; preds = %188, %208
  %209 = phi i32 [ %219, %208 ], [ %190, %188 ]
  %210 = phi double [ %218, %208 ], [ %191, %188 ]
  %211 = fmul double %210, 5.000000e-01
  %212 = fmul double %211, 5.000000e-01
  %213 = fmul double %212, 5.000000e-01
  %214 = fmul double %213, 5.000000e-01
  %215 = fmul double %214, 5.000000e-01
  %216 = fmul double %215, 5.000000e-01
  %217 = fmul double %216, 5.000000e-01
  %218 = fmul double %217, 5.000000e-01
  %219 = add nsw i32 %209, 8
  %220 = icmp eq i32 %219, 0
  br i1 %220, label %221, label %208, !llvm.loop !10022

221:                                              ; preds = %188, %208, %171, %146, %143
  %222 = phi double [ %145, %143 ], [ %148, %146 ], [ %172, %171 ], [ %189, %188 ], [ %218, %208 ]
  %223 = icmp ult i32 %136, 32768
  %224 = fneg double %222
  %225 = select i1 %223, double %222, double %224
  br label %265

226:                                              ; preds = %126
  %227 = bitcast i32 %136 to float
  %228 = fpext float %227 to double
  br label %265

229:                                              ; preds = %126, %126
  br label %231

230:                                              ; preds = %126
  br label %231

231:                                              ; preds = %230, %229, %126, %126
  %232 = phi i32 [ 32, %230 ], [ 16, %229 ], [ 8, %126 ], [ 8, %126 ]
  %233 = zext i32 %136 to i64
  switch i64 %84, label %243 [
    i64 21, label %234
    i64 20, label %234
    i64 3, label %234
    i64 5, label %234
  ]

234:                                              ; preds = %231, %231, %231, %231
  %235 = add nsw i32 %232, -1
  %236 = shl nuw i32 1, %235
  %237 = and i32 %236, %136
  %238 = icmp eq i32 %237, 0
  %239 = zext nneg i32 %232 to i64
  %240 = shl nsw i64 -1, %239
  %241 = select i1 %238, i64 0, i64 %240
  %242 = add nsw i64 %241, %233
  br label %243

243:                                              ; preds = %234, %231
  %244 = phi i64 [ %242, %234 ], [ %233, %231 ]
  %245 = load i64, ptr addrspace(1) %109, align 8
  %246 = icmp eq i64 %245, 0
  br i1 %246, label %247, label %249

247:                                              ; preds = %243
  %248 = sitofp i64 %244 to double
  br label %265

249:                                              ; preds = %243
  switch i64 %84, label %250 [
    i64 21, label %251
    i64 20, label %251
    i64 3, label %251
  ]

250:                                              ; preds = %249
  br label %251

251:                                              ; preds = %250, %249, %249, %249
  %252 = phi i32 [ -1, %249 ], [ %111, %250 ], [ -1, %249 ], [ -1, %249 ]
  %253 = add nsw i32 %252, %232
  %254 = zext nneg i32 %253 to i64
  %255 = shl nsw i64 -1, %254
  %256 = xor i64 %255, -1
  %257 = sitofp i64 %244 to double
  %258 = sitofp i64 %256 to double
  %259 = fdiv double %257, %258
  %260 = fcmp oge double %259, -1.000000e+00
  %261 = select i1 %260, double %259, double -1.000000e+00
  br label %265

262:                                              ; preds = %112
  %263 = icmp eq i32 %113, 3
  %264 = select i1 %263, double 1.000000e+00, double 0.000000e+00
  br label %265

265:                                              ; preds = %262, %251, %247, %226, %221
  %266 = phi double [ %264, %262 ], [ %225, %221 ], [ %228, %226 ], [ %261, %251 ], [ %248, %247 ]
  %267 = insertelement <4 x double> %114, double %266, i32 %113
  %268 = add nuw nsw i32 %113, 1
  %269 = icmp eq i32 %268, 4
  br i1 %269, label %270, label %112, !llvm.loop !10029

270:                                              ; preds = %265, %77
  %271 = phi <4 x double> [ zeroinitializer, %77 ], [ %267, %265 ]
  %272 = extractelement <4 x double> %271, i64 0
  %273 = fptrunc double %272 to float
  %274 = insertelement <4 x float> poison, float %273, i64 0
  %275 = extractelement <4 x double> %271, i64 1
  %276 = fptrunc double %275 to float
  %277 = insertelement <4 x float> %274, float %276, i64 1
  %278 = extractelement <4 x double> %271, i64 2
  %279 = fptrunc double %278 to float
  %280 = insertelement <4 x float> %277, float %279, i64 2
  %281 = extractelement <4 x double> %271, i64 3
  %282 = fptrunc double %281 to float
  %283 = insertelement <4 x float> %280, float %282, i64 3
  ret <4 x float> %283
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define dso_local <4 x float> @__zluda_ptx_impl_texobj_3d_v4_f32_f32(i64 noundef %0, <4 x float> noundef %1) local_unnamed_addr #3 {
  %3 = inttoptr i64 %0 to ptr addrspace(1)
  %4 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 4
  %5 = load i64, ptr addrspace(1) %4, align 8
  %6 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 5
  %7 = load i64, ptr addrspace(1) %6, align 8
  %8 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 6
  %9 = load i64, ptr addrspace(1) %8, align 8
  %10 = getelementptr i64, ptr addrspace(1) %3, i64 9
  %11 = and i64 %9, 4294967295
  %12 = icmp eq i64 %11, 1
  %13 = trunc i64 %7 to i32
  %14 = extractelement <4 x float> %1, i64 0
  %15 = fpext float %14 to double
  %16 = extractelement <4 x float> %1, i64 1
  %17 = fpext float %16 to double
  %18 = extractelement <4 x float> %1, i64 2
  %19 = fpext float %18 to double
  %20 = and i64 %5, 4294967295
  %21 = icmp eq i64 %20, 1
  %22 = load i64, ptr addrspace(1) %10, align 8
  %23 = tail call noundef i64 @llvm.smax.i64(i64 %22, i64 1)
  br i1 %21, label %313, label %24

24:                                               ; preds = %2
  %25 = getelementptr i64, ptr addrspace(1) %3, i64 1
  %26 = getelementptr i64, ptr addrspace(1) %3, i64 10
  %27 = load i64, ptr addrspace(1) %26, align 8
  %28 = tail call noundef i64 @llvm.smax.i64(i64 %27, i64 1)
  %29 = getelementptr i64, ptr addrspace(1) %3, i64 2
  %30 = getelementptr i64, ptr addrspace(1) %3, i64 11
  %31 = load i64, ptr addrspace(1) %30, align 8
  %32 = tail call noundef i64 @llvm.smax.i64(i64 %31, i64 1)
  %33 = sitofp i64 %23 to double
  %34 = select i1 %12, double %33, double 1.000000e+00
  %35 = fmul double %34, %15
  %36 = tail call double @llvm.floor.f64(double %35)
  %37 = fptosi double %36 to i64
  switch i32 %13, label %55 [
    i32 0, label %38
    i32 2, label %42
    i32 3, label %46
  ]

38:                                               ; preds = %24
  %39 = srem i64 %37, %23
  %40 = add nsw i64 %39, %23
  %41 = srem i64 %40, %23
  br label %59

42:                                               ; preds = %24
  %43 = icmp sgt i64 %23, %37
  %44 = tail call i64 @llvm.smax.i64(i64 %37, i64 -1)
  %45 = select i1 %43, i64 %44, i64 -1
  br label %59

46:                                               ; preds = %24
  %47 = shl nuw nsw i64 %23, 1
  %48 = srem i64 %37, %47
  %49 = add nsw i64 %48, %47
  %50 = srem i64 %49, %47
  %51 = icmp slt i64 %50, %23
  %52 = xor i64 %50, -1
  %53 = add i64 %47, %52
  %54 = select i1 %51, i64 %50, i64 %53
  br label %59

55:                                               ; preds = %24
  %56 = tail call noundef i64 @llvm.smax.i64(i64 %37, i64 0)
  %57 = add nsw i64 %23, -1
  %58 = tail call noundef i64 @llvm.smin.i64(i64 %56, i64 %57)
  br label %59

59:                                               ; preds = %55, %46, %42, %38
  %60 = phi i64 [ %58, %55 ], [ %54, %46 ], [ %45, %42 ], [ %41, %38 ]
  %61 = sitofp i64 %28 to double
  %62 = select i1 %12, double %61, double 1.000000e+00
  %63 = fmul double %62, %17
  %64 = tail call double @llvm.floor.f64(double %63)
  %65 = fptosi double %64 to i64
  switch i32 %13, label %83 [
    i32 0, label %66
    i32 2, label %70
    i32 3, label %74
  ]

66:                                               ; preds = %59
  %67 = srem i64 %65, %28
  %68 = add nsw i64 %67, %28
  %69 = srem i64 %68, %28
  br label %87

70:                                               ; preds = %59
  %71 = icmp sgt i64 %28, %65
  %72 = tail call i64 @llvm.smax.i64(i64 %65, i64 -1)
  %73 = select i1 %71, i64 %72, i64 -1
  br label %87

74:                                               ; preds = %59
  %75 = shl nuw nsw i64 %28, 1
  %76 = srem i64 %65, %75
  %77 = add nsw i64 %76, %75
  %78 = srem i64 %77, %75
  %79 = icmp slt i64 %78, %28
  %80 = xor i64 %78, -1
  %81 = add i64 %75, %80
  %82 = select i1 %79, i64 %78, i64 %81
  br label %87

83:                                               ; preds = %59
  %84 = tail call noundef i64 @llvm.smax.i64(i64 %65, i64 0)
  %85 = add nsw i64 %28, -1
  %86 = tail call noundef i64 @llvm.smin.i64(i64 %84, i64 %85)
  br label %87

87:                                               ; preds = %83, %74, %70, %66
  %88 = phi i64 [ %86, %83 ], [ %82, %74 ], [ %73, %70 ], [ %69, %66 ]
  %89 = sitofp i64 %32 to double
  %90 = select i1 %12, double %89, double 1.000000e+00
  %91 = fmul double %90, %19
  %92 = tail call double @llvm.floor.f64(double %91)
  %93 = fptosi double %92 to i64
  switch i32 %13, label %111 [
    i32 0, label %94
    i32 2, label %98
    i32 3, label %102
  ]

94:                                               ; preds = %87
  %95 = srem i64 %93, %32
  %96 = add nsw i64 %95, %32
  %97 = srem i64 %96, %32
  br label %115

98:                                               ; preds = %87
  %99 = icmp sgt i64 %32, %93
  %100 = tail call i64 @llvm.smax.i64(i64 %93, i64 -1)
  %101 = select i1 %99, i64 %100, i64 -1
  br label %115

102:                                              ; preds = %87
  %103 = shl nuw nsw i64 %32, 1
  %104 = srem i64 %93, %103
  %105 = add nsw i64 %104, %103
  %106 = srem i64 %105, %103
  %107 = icmp slt i64 %106, %32
  %108 = xor i64 %106, -1
  %109 = add i64 %103, %108
  %110 = select i1 %107, i64 %106, i64 %109
  br label %115

111:                                              ; preds = %87
  %112 = tail call noundef i64 @llvm.smax.i64(i64 %93, i64 0)
  %113 = add nsw i64 %32, -1
  %114 = tail call noundef i64 @llvm.smin.i64(i64 %112, i64 %113)
  br label %115

115:                                              ; preds = %111, %102, %98, %94
  %116 = phi i64 [ %114, %111 ], [ %110, %102 ], [ %101, %98 ], [ %97, %94 ]
  %117 = or i64 %88, %60
  %118 = or i64 %117, %116
  %119 = icmp sgt i64 %118, -1
  br i1 %119, label %120, label %726

120:                                              ; preds = %115
  %121 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 3
  %122 = load i64, ptr addrspace(1) %121, align 8
  %123 = load i64, ptr addrspace(1) %29, align 8
  switch i64 %123, label %125 [
    i64 0, label %126
    i64 20, label %126
    i64 1, label %124
    i64 21, label %124
    i64 6, label %124
  ]

124:                                              ; preds = %120, %120, %120
  br label %126

125:                                              ; preds = %120
  br label %126

126:                                              ; preds = %125, %124, %120, %120
  %127 = phi i32 [ 4, %125 ], [ 2, %124 ], [ 1, %120 ], [ 1, %120 ]
  %128 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 14
  %129 = load i64, ptr addrspace(1) %128, align 8
  %130 = icmp eq i64 %129, 0
  %131 = zext nneg i32 %127 to i64
  br i1 %130, label %141, label %132

132:                                              ; preds = %126
  %133 = load i64, ptr addrspace(1) %25, align 8
  %134 = mul nsw i64 %116, %28
  %135 = add nsw i64 %134, %88
  %136 = mul nsw i64 %129, %135
  %137 = mul nsw i64 %122, %60
  %138 = mul nsw i64 %137, %131
  %139 = add nsw i64 %136, %138
  %140 = add i64 %139, %133
  br label %150

141:                                              ; preds = %126
  %142 = mul nsw i64 %116, %28
  %143 = add nsw i64 %142, %88
  %144 = mul nsw i64 %143, %23
  %145 = add i64 %144, %60
  %146 = load i64, ptr addrspace(1) %25, align 8
  %147 = mul i64 %122, %145
  %148 = mul i64 %147, %131
  %149 = add i64 %146, %148
  br label %150

150:                                              ; preds = %141, %132
  %151 = phi i64 [ %140, %132 ], [ %149, %141 ]
  %152 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 7
  %153 = icmp eq i64 %123, 5
  %154 = sext i1 %153 to i32
  br label %155

155:                                              ; preds = %308, %150
  %156 = phi i32 [ 0, %150 ], [ %311, %308 ]
  %157 = phi <4 x double> [ zeroinitializer, %150 ], [ %310, %308 ]
  %158 = zext nneg i32 %156 to i64
  %159 = icmp sgt i64 %122, %158
  br i1 %159, label %160, label %305

160:                                              ; preds = %155
  %161 = mul nuw nsw i32 %156, %127
  %162 = zext nneg i32 %161 to i64
  %163 = add i64 %151, %162
  switch i64 %123, label %165 [
    i64 0, label %166
    i64 20, label %166
    i64 1, label %164
    i64 21, label %164
    i64 6, label %164
  ]

164:                                              ; preds = %160, %160, %160
  br label %166

165:                                              ; preds = %160
  br label %166

166:                                              ; preds = %165, %164, %160, %160
  %167 = phi i32 [ 4, %165 ], [ 2, %164 ], [ 1, %160 ], [ 1, %160 ]
  %168 = inttoptr i64 %163 to ptr addrspace(1)
  br label %170

169:                                              ; preds = %170
  switch i64 %123, label %273 [
    i64 6, label %182
    i64 7, label %269
    i64 0, label %274
    i64 20, label %274
    i64 1, label %272
    i64 21, label %272
  ]

170:                                              ; preds = %170, %166
  %171 = phi i32 [ 0, %166 ], [ %179, %170 ]
  %172 = phi i32 [ 0, %166 ], [ %180, %170 ]
  %173 = zext nneg i32 %172 to i64
  %174 = getelementptr inbounds i8, ptr addrspace(1) %168, i64 %173
  %175 = load i8, ptr addrspace(1) %174, align 1
  %176 = zext i8 %175 to i32
  %177 = shl nsw i32 %172, 3
  %178 = shl i32 %176, %177
  %179 = or i32 %178, %171
  %180 = add nuw nsw i32 %172, 1
  %181 = icmp eq i32 %180, %167
  br i1 %181, label %169, label %170, !llvm.loop !10016

182:                                              ; preds = %169
  %183 = lshr i32 %179, 10
  %184 = and i32 %183, 31
  %185 = and i32 %179, 1023
  switch i32 %184, label %192 [
    i32 0, label %186
    i32 31, label %189
  ]

186:                                              ; preds = %182
  %187 = uitofp i32 %185 to double
  %188 = fmul double %187, 0x3E70000000000000
  br label %264

189:                                              ; preds = %182
  %190 = icmp eq i32 %185, 0
  %191 = select i1 %190, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %264

192:                                              ; preds = %182
  %193 = or disjoint i32 %185, 1024
  %194 = uitofp i32 %193 to double
  %195 = add nsw i32 %184, -25
  %196 = icmp ugt i32 %184, 25
  br i1 %196, label %197, label %214

197:                                              ; preds = %192
  %198 = add nsw i32 %184, -26
  %199 = and i32 %195, 7
  %200 = icmp eq i32 %199, 0
  br i1 %200, label %209, label %201

201:                                              ; preds = %197, %201
  %202 = phi i32 [ %206, %201 ], [ %195, %197 ]
  %203 = phi double [ %205, %201 ], [ %194, %197 ]
  %204 = phi i32 [ %207, %201 ], [ 0, %197 ]
  %205 = fmul double %203, 2.000000e+00
  %206 = add nsw i32 %202, -1
  %207 = add i32 %204, 1
  %208 = icmp eq i32 %207, %199
  br i1 %208, label %209, label %201, !llvm.loop !10070

209:                                              ; preds = %201, %197
  %210 = phi i32 [ %195, %197 ], [ %206, %201 ]
  %211 = phi double [ %194, %197 ], [ %205, %201 ]
  %212 = phi double [ undef, %197 ], [ %205, %201 ]
  %213 = icmp ult i32 %198, 7
  br i1 %213, label %214, label %237

214:                                              ; preds = %209, %237, %192
  %215 = phi double [ %194, %192 ], [ %212, %209 ], [ %247, %237 ]
  %216 = phi i32 [ %195, %192 ], [ 0, %237 ], [ 0, %209 ]
  %217 = icmp slt i32 %216, 0
  br i1 %217, label %218, label %264

218:                                              ; preds = %214
  %219 = tail call i32 @llvm.umin.i32(i32 %184, i32 25)
  %220 = sub nsw i32 1, %219
  %221 = and i32 %220, 7
  %222 = icmp eq i32 %221, 0
  br i1 %222, label %231, label %223

223:                                              ; preds = %218, %223
  %224 = phi i32 [ %228, %223 ], [ %216, %218 ]
  %225 = phi double [ %227, %223 ], [ %215, %218 ]
  %226 = phi i32 [ %229, %223 ], [ 0, %218 ]
  %227 = fmul double %225, 5.000000e-01
  %228 = add nsw i32 %224, 1
  %229 = add i32 %226, 1
  %230 = icmp eq i32 %229, %221
  br i1 %230, label %231, label %223, !llvm.loop !10071

231:                                              ; preds = %223, %218
  %232 = phi double [ undef, %218 ], [ %227, %223 ]
  %233 = phi i32 [ %216, %218 ], [ %228, %223 ]
  %234 = phi double [ %215, %218 ], [ %227, %223 ]
  %235 = add nsw i32 %219, -18
  %236 = icmp ult i32 %235, 7
  br i1 %236, label %264, label %251

237:                                              ; preds = %209, %237
  %238 = phi i32 [ %248, %237 ], [ %210, %209 ]
  %239 = phi double [ %247, %237 ], [ %211, %209 ]
  %240 = fmul double %239, 2.000000e+00
  %241 = fmul double %240, 2.000000e+00
  %242 = fmul double %241, 2.000000e+00
  %243 = fmul double %242, 2.000000e+00
  %244 = fmul double %243, 2.000000e+00
  %245 = fmul double %244, 2.000000e+00
  %246 = fmul double %245, 2.000000e+00
  %247 = fmul double %246, 2.000000e+00
  %248 = add nsw i32 %238, -8
  %249 = add i32 %238, -9
  %250 = icmp ult i32 %249, -2
  br i1 %250, label %237, label %214, !llvm.loop !10021

251:                                              ; preds = %231, %251
  %252 = phi i32 [ %262, %251 ], [ %233, %231 ]
  %253 = phi double [ %261, %251 ], [ %234, %231 ]
  %254 = fmul double %253, 5.000000e-01
  %255 = fmul double %254, 5.000000e-01
  %256 = fmul double %255, 5.000000e-01
  %257 = fmul double %256, 5.000000e-01
  %258 = fmul double %257, 5.000000e-01
  %259 = fmul double %258, 5.000000e-01
  %260 = fmul double %259, 5.000000e-01
  %261 = fmul double %260, 5.000000e-01
  %262 = add nsw i32 %252, 8
  %263 = icmp eq i32 %262, 0
  br i1 %263, label %264, label %251, !llvm.loop !10022

264:                                              ; preds = %231, %251, %214, %189, %186
  %265 = phi double [ %188, %186 ], [ %191, %189 ], [ %215, %214 ], [ %232, %231 ], [ %261, %251 ]
  %266 = icmp ult i32 %179, 32768
  %267 = fneg double %265
  %268 = select i1 %266, double %265, double %267
  br label %308

269:                                              ; preds = %169
  %270 = bitcast i32 %179 to float
  %271 = fpext float %270 to double
  br label %308

272:                                              ; preds = %169, %169
  br label %274

273:                                              ; preds = %169
  br label %274

274:                                              ; preds = %273, %272, %169, %169
  %275 = phi i32 [ 32, %273 ], [ 16, %272 ], [ 8, %169 ], [ 8, %169 ]
  %276 = zext i32 %179 to i64
  switch i64 %123, label %286 [
    i64 21, label %277
    i64 20, label %277
    i64 3, label %277
    i64 5, label %277
  ]

277:                                              ; preds = %274, %274, %274, %274
  %278 = add nsw i32 %275, -1
  %279 = shl nuw i32 1, %278
  %280 = and i32 %279, %179
  %281 = icmp eq i32 %280, 0
  %282 = zext nneg i32 %275 to i64
  %283 = shl nsw i64 -1, %282
  %284 = select i1 %281, i64 0, i64 %283
  %285 = add nsw i64 %284, %276
  br label %286

286:                                              ; preds = %277, %274
  %287 = phi i64 [ %285, %277 ], [ %276, %274 ]
  %288 = load i64, ptr addrspace(1) %152, align 8
  %289 = icmp eq i64 %288, 0
  br i1 %289, label %290, label %292

290:                                              ; preds = %286
  %291 = sitofp i64 %287 to double
  br label %308

292:                                              ; preds = %286
  switch i64 %123, label %293 [
    i64 21, label %294
    i64 20, label %294
    i64 3, label %294
  ]

293:                                              ; preds = %292
  br label %294

294:                                              ; preds = %293, %292, %292, %292
  %295 = phi i32 [ -1, %292 ], [ %154, %293 ], [ -1, %292 ], [ -1, %292 ]
  %296 = add nsw i32 %295, %275
  %297 = zext nneg i32 %296 to i64
  %298 = shl nsw i64 -1, %297
  %299 = xor i64 %298, -1
  %300 = sitofp i64 %287 to double
  %301 = sitofp i64 %299 to double
  %302 = fdiv double %300, %301
  %303 = fcmp oge double %302, -1.000000e+00
  %304 = select i1 %303, double %302, double -1.000000e+00
  br label %308

305:                                              ; preds = %155
  %306 = icmp eq i32 %156, 3
  %307 = select i1 %306, double 1.000000e+00, double 0.000000e+00
  br label %308

308:                                              ; preds = %305, %294, %290, %269, %264
  %309 = phi double [ %307, %305 ], [ %268, %264 ], [ %271, %269 ], [ %304, %294 ], [ %291, %290 ]
  %310 = insertelement <4 x double> %157, double %309, i32 %156
  %311 = add nuw nsw i32 %156, 1
  %312 = icmp eq i32 %311, 4
  br i1 %312, label %726, label %155, !llvm.loop !10029

313:                                              ; preds = %2
  %314 = sitofp i64 %23 to double
  %315 = select i1 %12, double %314, double 1.000000e+00
  %316 = fmul double %315, %15
  %317 = fadd double %316, -5.000000e-01
  %318 = tail call double @llvm.floor.f64(double %317)
  %319 = fptosi double %318 to i64
  switch i32 %13, label %337 [
    i32 0, label %320
    i32 2, label %324
    i32 3, label %328
  ]

320:                                              ; preds = %313
  %321 = srem i64 %319, %23
  %322 = add nsw i64 %321, %23
  %323 = srem i64 %322, %23
  br label %341

324:                                              ; preds = %313
  %325 = icmp sgt i64 %23, %319
  %326 = tail call i64 @llvm.smax.i64(i64 %319, i64 -1)
  %327 = select i1 %325, i64 %326, i64 -1
  br label %341

328:                                              ; preds = %313
  %329 = shl nuw nsw i64 %23, 1
  %330 = srem i64 %319, %329
  %331 = add nsw i64 %330, %329
  %332 = srem i64 %331, %329
  %333 = icmp slt i64 %332, %23
  %334 = xor i64 %332, -1
  %335 = add i64 %329, %334
  %336 = select i1 %333, i64 %332, i64 %335
  br label %341

337:                                              ; preds = %313
  %338 = tail call noundef i64 @llvm.smax.i64(i64 %319, i64 0)
  %339 = add nsw i64 %23, -1
  %340 = tail call noundef i64 @llvm.smin.i64(i64 %338, i64 %339)
  br label %341

341:                                              ; preds = %337, %328, %324, %320
  %342 = phi i64 [ %340, %337 ], [ %336, %328 ], [ %327, %324 ], [ %323, %320 ]
  %343 = add i64 %319, 1
  switch i32 %13, label %361 [
    i32 0, label %344
    i32 2, label %348
    i32 3, label %352
  ]

344:                                              ; preds = %341
  %345 = srem i64 %343, %23
  %346 = add nsw i64 %345, %23
  %347 = srem i64 %346, %23
  br label %365

348:                                              ; preds = %341
  %349 = icmp slt i64 %343, %23
  %350 = tail call i64 @llvm.smax.i64(i64 %343, i64 -1)
  %351 = select i1 %349, i64 %350, i64 -1
  br label %365

352:                                              ; preds = %341
  %353 = shl nuw nsw i64 %23, 1
  %354 = srem i64 %343, %353
  %355 = add nsw i64 %354, %353
  %356 = srem i64 %355, %353
  %357 = icmp slt i64 %356, %23
  %358 = xor i64 %356, -1
  %359 = add i64 %353, %358
  %360 = select i1 %357, i64 %356, i64 %359
  br label %365

361:                                              ; preds = %341
  %362 = tail call noundef i64 @llvm.smax.i64(i64 %343, i64 0)
  %363 = add nsw i64 %23, -1
  %364 = tail call noundef i64 @llvm.smin.i64(i64 %362, i64 %363)
  br label %365

365:                                              ; preds = %361, %352, %348, %344
  %366 = phi i64 [ %364, %361 ], [ %360, %352 ], [ %351, %348 ], [ %347, %344 ]
  %367 = getelementptr i64, ptr addrspace(1) %3, i64 10
  %368 = load i64, ptr addrspace(1) %367, align 8
  %369 = tail call noundef i64 @llvm.smax.i64(i64 %368, i64 1)
  %370 = sitofp i64 %369 to double
  %371 = select i1 %12, double %370, double 1.000000e+00
  %372 = fmul double %371, %17
  %373 = fadd double %372, -5.000000e-01
  %374 = tail call double @llvm.floor.f64(double %373)
  %375 = fptosi double %374 to i64
  switch i32 %13, label %393 [
    i32 0, label %376
    i32 2, label %380
    i32 3, label %384
  ]

376:                                              ; preds = %365
  %377 = srem i64 %375, %369
  %378 = add nsw i64 %377, %369
  %379 = srem i64 %378, %369
  br label %397

380:                                              ; preds = %365
  %381 = icmp sgt i64 %369, %375
  %382 = tail call i64 @llvm.smax.i64(i64 %375, i64 -1)
  %383 = select i1 %381, i64 %382, i64 -1
  br label %397

384:                                              ; preds = %365
  %385 = shl nuw nsw i64 %369, 1
  %386 = srem i64 %375, %385
  %387 = add nsw i64 %386, %385
  %388 = srem i64 %387, %385
  %389 = icmp slt i64 %388, %369
  %390 = xor i64 %388, -1
  %391 = add i64 %385, %390
  %392 = select i1 %389, i64 %388, i64 %391
  br label %397

393:                                              ; preds = %365
  %394 = tail call noundef i64 @llvm.smax.i64(i64 %375, i64 0)
  %395 = add nsw i64 %369, -1
  %396 = tail call noundef i64 @llvm.smin.i64(i64 %394, i64 %395)
  br label %397

397:                                              ; preds = %393, %384, %380, %376
  %398 = phi i64 [ %396, %393 ], [ %392, %384 ], [ %383, %380 ], [ %379, %376 ]
  %399 = add i64 %375, 1
  switch i32 %13, label %417 [
    i32 0, label %400
    i32 2, label %404
    i32 3, label %408
  ]

400:                                              ; preds = %397
  %401 = srem i64 %399, %369
  %402 = add nsw i64 %401, %369
  %403 = srem i64 %402, %369
  br label %421

404:                                              ; preds = %397
  %405 = icmp slt i64 %399, %369
  %406 = tail call i64 @llvm.smax.i64(i64 %399, i64 -1)
  %407 = select i1 %405, i64 %406, i64 -1
  br label %421

408:                                              ; preds = %397
  %409 = shl nuw nsw i64 %369, 1
  %410 = srem i64 %399, %409
  %411 = add nsw i64 %410, %409
  %412 = srem i64 %411, %409
  %413 = icmp slt i64 %412, %369
  %414 = xor i64 %412, -1
  %415 = add i64 %409, %414
  %416 = select i1 %413, i64 %412, i64 %415
  br label %421

417:                                              ; preds = %397
  %418 = tail call noundef i64 @llvm.smax.i64(i64 %399, i64 0)
  %419 = add nsw i64 %369, -1
  %420 = tail call noundef i64 @llvm.smin.i64(i64 %418, i64 %419)
  br label %421

421:                                              ; preds = %417, %408, %404, %400
  %422 = phi i64 [ %420, %417 ], [ %416, %408 ], [ %407, %404 ], [ %403, %400 ]
  %423 = getelementptr i64, ptr addrspace(1) %3, i64 11
  %424 = load i64, ptr addrspace(1) %423, align 8
  %425 = tail call noundef i64 @llvm.smax.i64(i64 %424, i64 1)
  %426 = sitofp i64 %425 to double
  %427 = select i1 %12, double %426, double 1.000000e+00
  %428 = fmul double %427, %19
  %429 = fadd double %428, -5.000000e-01
  %430 = tail call double @llvm.floor.f64(double %429)
  %431 = fptosi double %430 to i64
  switch i32 %13, label %449 [
    i32 0, label %432
    i32 2, label %436
    i32 3, label %440
  ]

432:                                              ; preds = %421
  %433 = srem i64 %431, %425
  %434 = add nsw i64 %433, %425
  %435 = srem i64 %434, %425
  br label %453

436:                                              ; preds = %421
  %437 = icmp sgt i64 %425, %431
  %438 = tail call i64 @llvm.smax.i64(i64 %431, i64 -1)
  %439 = select i1 %437, i64 %438, i64 -1
  br label %453

440:                                              ; preds = %421
  %441 = shl nuw nsw i64 %425, 1
  %442 = srem i64 %431, %441
  %443 = add nsw i64 %442, %441
  %444 = srem i64 %443, %441
  %445 = icmp slt i64 %444, %425
  %446 = xor i64 %444, -1
  %447 = add i64 %441, %446
  %448 = select i1 %445, i64 %444, i64 %447
  br label %453

449:                                              ; preds = %421
  %450 = tail call noundef i64 @llvm.smax.i64(i64 %431, i64 0)
  %451 = add nsw i64 %425, -1
  %452 = tail call noundef i64 @llvm.smin.i64(i64 %450, i64 %451)
  br label %453

453:                                              ; preds = %449, %440, %436, %432
  %454 = phi i64 [ %452, %449 ], [ %448, %440 ], [ %439, %436 ], [ %435, %432 ]
  %455 = add i64 %431, 1
  switch i32 %13, label %473 [
    i32 0, label %456
    i32 2, label %460
    i32 3, label %464
  ]

456:                                              ; preds = %453
  %457 = srem i64 %455, %425
  %458 = add nsw i64 %457, %425
  %459 = srem i64 %458, %425
  br label %477

460:                                              ; preds = %453
  %461 = icmp slt i64 %455, %425
  %462 = tail call i64 @llvm.smax.i64(i64 %455, i64 -1)
  %463 = select i1 %461, i64 %462, i64 -1
  br label %477

464:                                              ; preds = %453
  %465 = shl nuw nsw i64 %425, 1
  %466 = srem i64 %455, %465
  %467 = add nsw i64 %466, %465
  %468 = srem i64 %467, %465
  %469 = icmp slt i64 %468, %425
  %470 = xor i64 %468, -1
  %471 = add i64 %465, %470
  %472 = select i1 %469, i64 %468, i64 %471
  br label %477

473:                                              ; preds = %453
  %474 = tail call noundef i64 @llvm.smax.i64(i64 %455, i64 0)
  %475 = add nsw i64 %425, -1
  %476 = tail call noundef i64 @llvm.smin.i64(i64 %474, i64 %475)
  br label %477

477:                                              ; preds = %473, %464, %460, %456
  %478 = phi i64 [ %476, %473 ], [ %472, %464 ], [ %463, %460 ], [ %459, %456 ]
  %479 = fsub double %317, %318
  %480 = fmul double %479, 2.560000e+02
  %481 = fadd double %480, 5.000000e-01
  %482 = tail call double @llvm.floor.f64(double %481)
  %483 = fptosi double %482 to i64
  %484 = sitofp i64 %483 to double
  %485 = fmul double %484, 3.906250e-03
  %486 = fsub double %373, %374
  %487 = fmul double %486, 2.560000e+02
  %488 = fadd double %487, 5.000000e-01
  %489 = tail call double @llvm.floor.f64(double %488)
  %490 = fptosi double %489 to i64
  %491 = sitofp i64 %490 to double
  %492 = fmul double %491, 3.906250e-03
  %493 = fsub double %429, %430
  %494 = fmul double %493, 2.560000e+02
  %495 = fadd double %494, 5.000000e-01
  %496 = tail call double @llvm.floor.f64(double %495)
  %497 = fptosi double %496 to i64
  %498 = sitofp i64 %497 to double
  %499 = fmul double %498, 3.906250e-03
  %500 = fsub double 1.000000e+00, %485
  %501 = fsub double 1.000000e+00, %492
  %502 = fsub double 1.000000e+00, %499
  %503 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 3
  %504 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 2
  %505 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 14
  %506 = getelementptr i64, ptr addrspace(1) %3, i64 1
  %507 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 7
  br label %508

508:                                              ; preds = %720, %477
  %509 = phi <4 x double> [ zeroinitializer, %477 ], [ %723, %720 ]
  %510 = phi i32 [ 0, %477 ], [ %724, %720 ]
  %511 = and i32 %510, 1
  %512 = icmp eq i32 %511, 0
  %513 = select i1 %512, double %500, double %485
  %514 = and i32 %510, 2
  %515 = icmp eq i32 %514, 0
  %516 = select i1 %515, double %501, double %492
  %517 = fmul double %513, %516
  %518 = and i32 %510, 4
  %519 = icmp eq i32 %518, 0
  %520 = select i1 %519, double %502, double %499
  %521 = fmul double %520, %517
  %522 = insertelement <4 x double> poison, double %521, i64 0
  %523 = shufflevector <4 x double> %522, <4 x double> poison, <4 x i32> zeroinitializer
  %524 = select i1 %512, i64 %342, i64 %366
  %525 = select i1 %515, i64 %398, i64 %422
  %526 = select i1 %519, i64 %454, i64 %478
  %527 = or i64 %525, %524
  %528 = or i64 %527, %526
  %529 = icmp sgt i64 %528, -1
  br i1 %529, label %530, label %720

530:                                              ; preds = %508
  %531 = load i64, ptr addrspace(1) %503, align 8
  %532 = load i64, ptr addrspace(1) %504, align 8
  switch i64 %532, label %534 [
    i64 0, label %535
    i64 20, label %535
    i64 1, label %533
    i64 21, label %533
    i64 6, label %533
  ]

533:                                              ; preds = %530, %530, %530
  br label %535

534:                                              ; preds = %530
  br label %535

535:                                              ; preds = %534, %533, %530, %530
  %536 = phi i32 [ 4, %534 ], [ 2, %533 ], [ 1, %530 ], [ 1, %530 ]
  %537 = load i64, ptr addrspace(1) %505, align 8
  %538 = icmp eq i64 %537, 0
  %539 = zext nneg i32 %536 to i64
  br i1 %538, label %549, label %540

540:                                              ; preds = %535
  %541 = load i64, ptr addrspace(1) %506, align 8
  %542 = mul nsw i64 %526, %369
  %543 = add nsw i64 %542, %525
  %544 = mul nsw i64 %537, %543
  %545 = mul nsw i64 %531, %524
  %546 = mul nsw i64 %545, %539
  %547 = add nsw i64 %544, %546
  %548 = add i64 %547, %541
  br label %558

549:                                              ; preds = %535
  %550 = mul nsw i64 %526, %369
  %551 = add nsw i64 %550, %525
  %552 = mul nsw i64 %551, %23
  %553 = add i64 %552, %524
  %554 = load i64, ptr addrspace(1) %506, align 8
  %555 = mul i64 %531, %553
  %556 = mul i64 %555, %539
  %557 = add i64 %554, %556
  br label %558

558:                                              ; preds = %549, %540
  %559 = phi i64 [ %548, %540 ], [ %557, %549 ]
  %560 = icmp eq i64 %532, 5
  %561 = sext i1 %560 to i32
  br label %562

562:                                              ; preds = %715, %558
  %563 = phi i32 [ 0, %558 ], [ %718, %715 ]
  %564 = phi <4 x double> [ zeroinitializer, %558 ], [ %717, %715 ]
  %565 = zext nneg i32 %563 to i64
  %566 = icmp sgt i64 %531, %565
  br i1 %566, label %567, label %712

567:                                              ; preds = %562
  %568 = mul nuw nsw i32 %563, %536
  %569 = zext nneg i32 %568 to i64
  %570 = add i64 %559, %569
  switch i64 %532, label %572 [
    i64 0, label %573
    i64 20, label %573
    i64 1, label %571
    i64 21, label %571
    i64 6, label %571
  ]

571:                                              ; preds = %567, %567, %567
  br label %573

572:                                              ; preds = %567
  br label %573

573:                                              ; preds = %572, %571, %567, %567
  %574 = phi i32 [ 4, %572 ], [ 2, %571 ], [ 1, %567 ], [ 1, %567 ]
  %575 = inttoptr i64 %570 to ptr addrspace(1)
  br label %577

576:                                              ; preds = %577
  switch i64 %532, label %680 [
    i64 6, label %589
    i64 7, label %676
    i64 0, label %681
    i64 20, label %681
    i64 1, label %679
    i64 21, label %679
  ]

577:                                              ; preds = %577, %573
  %578 = phi i32 [ 0, %573 ], [ %586, %577 ]
  %579 = phi i32 [ 0, %573 ], [ %587, %577 ]
  %580 = zext nneg i32 %579 to i64
  %581 = getelementptr inbounds i8, ptr addrspace(1) %575, i64 %580
  %582 = load i8, ptr addrspace(1) %581, align 1
  %583 = zext i8 %582 to i32
  %584 = shl nsw i32 %579, 3
  %585 = shl i32 %583, %584
  %586 = or i32 %585, %578
  %587 = add nuw nsw i32 %579, 1
  %588 = icmp eq i32 %587, %574
  br i1 %588, label %576, label %577, !llvm.loop !10016

589:                                              ; preds = %576
  %590 = lshr i32 %586, 10
  %591 = and i32 %590, 31
  %592 = and i32 %586, 1023
  switch i32 %591, label %599 [
    i32 0, label %593
    i32 31, label %596
  ]

593:                                              ; preds = %589
  %594 = uitofp i32 %592 to double
  %595 = fmul double %594, 0x3E70000000000000
  br label %671

596:                                              ; preds = %589
  %597 = icmp eq i32 %592, 0
  %598 = select i1 %597, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %671

599:                                              ; preds = %589
  %600 = or disjoint i32 %592, 1024
  %601 = uitofp i32 %600 to double
  %602 = add nsw i32 %591, -25
  %603 = icmp ugt i32 %591, 25
  br i1 %603, label %604, label %621

604:                                              ; preds = %599
  %605 = add nsw i32 %591, -26
  %606 = and i32 %602, 7
  %607 = icmp eq i32 %606, 0
  br i1 %607, label %616, label %608

608:                                              ; preds = %604, %608
  %609 = phi i32 [ %613, %608 ], [ %602, %604 ]
  %610 = phi double [ %612, %608 ], [ %601, %604 ]
  %611 = phi i32 [ %614, %608 ], [ 0, %604 ]
  %612 = fmul double %610, 2.000000e+00
  %613 = add nsw i32 %609, -1
  %614 = add i32 %611, 1
  %615 = icmp eq i32 %614, %606
  br i1 %615, label %616, label %608, !llvm.loop !10072

616:                                              ; preds = %608, %604
  %617 = phi i32 [ %602, %604 ], [ %613, %608 ]
  %618 = phi double [ %601, %604 ], [ %612, %608 ]
  %619 = phi double [ undef, %604 ], [ %612, %608 ]
  %620 = icmp ult i32 %605, 7
  br i1 %620, label %621, label %644

621:                                              ; preds = %616, %644, %599
  %622 = phi double [ %601, %599 ], [ %619, %616 ], [ %654, %644 ]
  %623 = phi i32 [ %602, %599 ], [ 0, %644 ], [ 0, %616 ]
  %624 = icmp slt i32 %623, 0
  br i1 %624, label %625, label %671

625:                                              ; preds = %621
  %626 = tail call i32 @llvm.umin.i32(i32 %591, i32 25)
  %627 = sub nsw i32 1, %626
  %628 = and i32 %627, 7
  %629 = icmp eq i32 %628, 0
  br i1 %629, label %638, label %630

630:                                              ; preds = %625, %630
  %631 = phi i32 [ %635, %630 ], [ %623, %625 ]
  %632 = phi double [ %634, %630 ], [ %622, %625 ]
  %633 = phi i32 [ %636, %630 ], [ 0, %625 ]
  %634 = fmul double %632, 5.000000e-01
  %635 = add nsw i32 %631, 1
  %636 = add i32 %633, 1
  %637 = icmp eq i32 %636, %628
  br i1 %637, label %638, label %630, !llvm.loop !10073

638:                                              ; preds = %630, %625
  %639 = phi double [ undef, %625 ], [ %634, %630 ]
  %640 = phi i32 [ %623, %625 ], [ %635, %630 ]
  %641 = phi double [ %622, %625 ], [ %634, %630 ]
  %642 = add nsw i32 %626, -18
  %643 = icmp ult i32 %642, 7
  br i1 %643, label %671, label %658

644:                                              ; preds = %616, %644
  %645 = phi i32 [ %655, %644 ], [ %617, %616 ]
  %646 = phi double [ %654, %644 ], [ %618, %616 ]
  %647 = fmul double %646, 2.000000e+00
  %648 = fmul double %647, 2.000000e+00
  %649 = fmul double %648, 2.000000e+00
  %650 = fmul double %649, 2.000000e+00
  %651 = fmul double %650, 2.000000e+00
  %652 = fmul double %651, 2.000000e+00
  %653 = fmul double %652, 2.000000e+00
  %654 = fmul double %653, 2.000000e+00
  %655 = add nsw i32 %645, -8
  %656 = add i32 %645, -9
  %657 = icmp ult i32 %656, -2
  br i1 %657, label %644, label %621, !llvm.loop !10021

658:                                              ; preds = %638, %658
  %659 = phi i32 [ %669, %658 ], [ %640, %638 ]
  %660 = phi double [ %668, %658 ], [ %641, %638 ]
  %661 = fmul double %660, 5.000000e-01
  %662 = fmul double %661, 5.000000e-01
  %663 = fmul double %662, 5.000000e-01
  %664 = fmul double %663, 5.000000e-01
  %665 = fmul double %664, 5.000000e-01
  %666 = fmul double %665, 5.000000e-01
  %667 = fmul double %666, 5.000000e-01
  %668 = fmul double %667, 5.000000e-01
  %669 = add nsw i32 %659, 8
  %670 = icmp eq i32 %669, 0
  br i1 %670, label %671, label %658, !llvm.loop !10022

671:                                              ; preds = %638, %658, %621, %596, %593
  %672 = phi double [ %595, %593 ], [ %598, %596 ], [ %622, %621 ], [ %639, %638 ], [ %668, %658 ]
  %673 = icmp ult i32 %586, 32768
  %674 = fneg double %672
  %675 = select i1 %673, double %672, double %674
  br label %715

676:                                              ; preds = %576
  %677 = bitcast i32 %586 to float
  %678 = fpext float %677 to double
  br label %715

679:                                              ; preds = %576, %576
  br label %681

680:                                              ; preds = %576
  br label %681

681:                                              ; preds = %680, %679, %576, %576
  %682 = phi i32 [ 32, %680 ], [ 16, %679 ], [ 8, %576 ], [ 8, %576 ]
  %683 = zext i32 %586 to i64
  switch i64 %532, label %693 [
    i64 21, label %684
    i64 20, label %684
    i64 3, label %684
    i64 5, label %684
  ]

684:                                              ; preds = %681, %681, %681, %681
  %685 = add nsw i32 %682, -1
  %686 = shl nuw i32 1, %685
  %687 = and i32 %686, %586
  %688 = icmp eq i32 %687, 0
  %689 = zext nneg i32 %682 to i64
  %690 = shl nsw i64 -1, %689
  %691 = select i1 %688, i64 0, i64 %690
  %692 = add nsw i64 %691, %683
  br label %693

693:                                              ; preds = %684, %681
  %694 = phi i64 [ %692, %684 ], [ %683, %681 ]
  %695 = load i64, ptr addrspace(1) %507, align 8
  %696 = icmp eq i64 %695, 0
  br i1 %696, label %697, label %699

697:                                              ; preds = %693
  %698 = sitofp i64 %694 to double
  br label %715

699:                                              ; preds = %693
  switch i64 %532, label %700 [
    i64 21, label %701
    i64 20, label %701
    i64 3, label %701
  ]

700:                                              ; preds = %699
  br label %701

701:                                              ; preds = %700, %699, %699, %699
  %702 = phi i32 [ -1, %699 ], [ %561, %700 ], [ -1, %699 ], [ -1, %699 ]
  %703 = add nsw i32 %702, %682
  %704 = zext nneg i32 %703 to i64
  %705 = shl nsw i64 -1, %704
  %706 = xor i64 %705, -1
  %707 = sitofp i64 %694 to double
  %708 = sitofp i64 %706 to double
  %709 = fdiv double %707, %708
  %710 = fcmp oge double %709, -1.000000e+00
  %711 = select i1 %710, double %709, double -1.000000e+00
  br label %715

712:                                              ; preds = %562
  %713 = icmp eq i32 %563, 3
  %714 = select i1 %713, double 1.000000e+00, double 0.000000e+00
  br label %715

715:                                              ; preds = %712, %701, %697, %676, %671
  %716 = phi double [ %714, %712 ], [ %675, %671 ], [ %678, %676 ], [ %711, %701 ], [ %698, %697 ]
  %717 = insertelement <4 x double> %564, double %716, i32 %563
  %718 = add nuw nsw i32 %563, 1
  %719 = icmp eq i32 %718, 4
  br i1 %719, label %720, label %562, !llvm.loop !10029

720:                                              ; preds = %715, %508
  %721 = phi <4 x double> [ zeroinitializer, %508 ], [ %717, %715 ]
  %722 = fmul <4 x double> %523, %721
  %723 = fadd <4 x double> %509, %722
  %724 = add nuw nsw i32 %510, 1
  %725 = icmp eq i32 %724, 8
  br i1 %725, label %726, label %508, !llvm.loop !10032

726:                                              ; preds = %308, %720, %115
  %727 = phi <4 x double> [ zeroinitializer, %115 ], [ %723, %720 ], [ %310, %308 ]
  %728 = extractelement <4 x double> %727, i64 0
  %729 = fptrunc double %728 to float
  %730 = insertelement <4 x float> poison, float %729, i64 0
  %731 = extractelement <4 x double> %727, i64 1
  %732 = fptrunc double %731 to float
  %733 = insertelement <4 x float> %730, float %732, i64 1
  %734 = extractelement <4 x double> %727, i64 2
  %735 = fptrunc double %734 to float
  %736 = insertelement <4 x float> %733, float %735, i64 2
  %737 = extractelement <4 x double> %727, i64 3
  %738 = fptrunc double %737 to float
  %739 = insertelement <4 x float> %736, float %738, i64 3
  ret <4 x float> %739
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define dso_local <4 x float> @__zluda_ptx_impl_texobj_3d_v4_f32_s32(i64 noundef %0, <4 x i32> noundef %1) local_unnamed_addr #3 {
  %3 = extractelement <4 x i32> %1, i64 1
  %4 = inttoptr i64 %0 to ptr addrspace(1)
  %5 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 5
  %6 = load i64, ptr addrspace(1) %5, align 8
  %7 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 6
  %8 = load i64, ptr addrspace(1) %7, align 8
  %9 = getelementptr i64, ptr addrspace(1) %4, i64 9
  %10 = and i64 %8, 4294967295
  %11 = icmp eq i64 %10, 1
  %12 = extractelement <4 x i32> %1, i64 0
  %13 = extractelement <4 x i32> %1, i64 2
  %14 = trunc i64 %6 to i32
  %15 = sitofp i32 %12 to double
  %16 = sitofp i32 %3 to double
  %17 = sitofp i32 %13 to double
  %18 = load i64, ptr addrspace(1) %9, align 8
  %19 = tail call noundef i64 @llvm.smax.i64(i64 %18, i64 1)
  %20 = getelementptr i64, ptr addrspace(1) %4, i64 1
  %21 = getelementptr i64, ptr addrspace(1) %4, i64 10
  %22 = load i64, ptr addrspace(1) %21, align 8
  %23 = tail call noundef i64 @llvm.smax.i64(i64 %22, i64 1)
  %24 = getelementptr i64, ptr addrspace(1) %4, i64 2
  %25 = getelementptr i64, ptr addrspace(1) %4, i64 11
  %26 = load i64, ptr addrspace(1) %25, align 8
  %27 = tail call noundef i64 @llvm.smax.i64(i64 %26, i64 1)
  %28 = sitofp i64 %19 to double
  %29 = select i1 %11, double %28, double 1.000000e+00
  %30 = fmul double %29, %15
  %31 = tail call double @llvm.floor.f64(double %30)
  %32 = fptosi double %31 to i64
  switch i32 %14, label %50 [
    i32 0, label %33
    i32 2, label %37
    i32 3, label %41
  ]

33:                                               ; preds = %2
  %34 = srem i64 %32, %19
  %35 = add nsw i64 %34, %19
  %36 = srem i64 %35, %19
  br label %54

37:                                               ; preds = %2
  %38 = icmp sgt i64 %19, %32
  %39 = tail call i64 @llvm.smax.i64(i64 %32, i64 -1)
  %40 = select i1 %38, i64 %39, i64 -1
  br label %54

41:                                               ; preds = %2
  %42 = shl nuw nsw i64 %19, 1
  %43 = srem i64 %32, %42
  %44 = add nsw i64 %43, %42
  %45 = srem i64 %44, %42
  %46 = icmp slt i64 %45, %19
  %47 = xor i64 %45, -1
  %48 = add i64 %42, %47
  %49 = select i1 %46, i64 %45, i64 %48
  br label %54

50:                                               ; preds = %2
  %51 = tail call noundef i64 @llvm.smax.i64(i64 %32, i64 0)
  %52 = add nsw i64 %19, -1
  %53 = tail call noundef i64 @llvm.smin.i64(i64 %51, i64 %52)
  br label %54

54:                                               ; preds = %50, %41, %37, %33
  %55 = phi i64 [ %53, %50 ], [ %49, %41 ], [ %40, %37 ], [ %36, %33 ]
  %56 = sitofp i64 %23 to double
  %57 = select i1 %11, double %56, double 1.000000e+00
  %58 = fmul double %57, %16
  %59 = tail call double @llvm.floor.f64(double %58)
  %60 = fptosi double %59 to i64
  switch i32 %14, label %78 [
    i32 0, label %61
    i32 2, label %65
    i32 3, label %69
  ]

61:                                               ; preds = %54
  %62 = srem i64 %60, %23
  %63 = add nsw i64 %62, %23
  %64 = srem i64 %63, %23
  br label %82

65:                                               ; preds = %54
  %66 = icmp sgt i64 %23, %60
  %67 = tail call i64 @llvm.smax.i64(i64 %60, i64 -1)
  %68 = select i1 %66, i64 %67, i64 -1
  br label %82

69:                                               ; preds = %54
  %70 = shl nuw nsw i64 %23, 1
  %71 = srem i64 %60, %70
  %72 = add nsw i64 %71, %70
  %73 = srem i64 %72, %70
  %74 = icmp slt i64 %73, %23
  %75 = xor i64 %73, -1
  %76 = add i64 %70, %75
  %77 = select i1 %74, i64 %73, i64 %76
  br label %82

78:                                               ; preds = %54
  %79 = tail call noundef i64 @llvm.smax.i64(i64 %60, i64 0)
  %80 = add nsw i64 %23, -1
  %81 = tail call noundef i64 @llvm.smin.i64(i64 %79, i64 %80)
  br label %82

82:                                               ; preds = %78, %69, %65, %61
  %83 = phi i64 [ %81, %78 ], [ %77, %69 ], [ %68, %65 ], [ %64, %61 ]
  %84 = sitofp i64 %27 to double
  %85 = select i1 %11, double %84, double 1.000000e+00
  %86 = fmul double %85, %17
  %87 = tail call double @llvm.floor.f64(double %86)
  %88 = fptosi double %87 to i64
  switch i32 %14, label %106 [
    i32 0, label %89
    i32 2, label %93
    i32 3, label %97
  ]

89:                                               ; preds = %82
  %90 = srem i64 %88, %27
  %91 = add nsw i64 %90, %27
  %92 = srem i64 %91, %27
  br label %110

93:                                               ; preds = %82
  %94 = icmp sgt i64 %27, %88
  %95 = tail call i64 @llvm.smax.i64(i64 %88, i64 -1)
  %96 = select i1 %94, i64 %95, i64 -1
  br label %110

97:                                               ; preds = %82
  %98 = shl nuw nsw i64 %27, 1
  %99 = srem i64 %88, %98
  %100 = add nsw i64 %99, %98
  %101 = srem i64 %100, %98
  %102 = icmp slt i64 %101, %27
  %103 = xor i64 %101, -1
  %104 = add i64 %98, %103
  %105 = select i1 %102, i64 %101, i64 %104
  br label %110

106:                                              ; preds = %82
  %107 = tail call noundef i64 @llvm.smax.i64(i64 %88, i64 0)
  %108 = add nsw i64 %27, -1
  %109 = tail call noundef i64 @llvm.smin.i64(i64 %107, i64 %108)
  br label %110

110:                                              ; preds = %106, %97, %93, %89
  %111 = phi i64 [ %109, %106 ], [ %105, %97 ], [ %96, %93 ], [ %92, %89 ]
  %112 = or i64 %83, %55
  %113 = or i64 %112, %111
  %114 = icmp sgt i64 %113, -1
  br i1 %114, label %115, label %308

115:                                              ; preds = %110
  %116 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 3
  %117 = load i64, ptr addrspace(1) %116, align 8
  %118 = load i64, ptr addrspace(1) %24, align 8
  switch i64 %118, label %120 [
    i64 0, label %121
    i64 20, label %121
    i64 1, label %119
    i64 21, label %119
    i64 6, label %119
  ]

119:                                              ; preds = %115, %115, %115
  br label %121

120:                                              ; preds = %115
  br label %121

121:                                              ; preds = %120, %119, %115, %115
  %122 = phi i32 [ 4, %120 ], [ 2, %119 ], [ 1, %115 ], [ 1, %115 ]
  %123 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 14
  %124 = load i64, ptr addrspace(1) %123, align 8
  %125 = icmp eq i64 %124, 0
  %126 = zext nneg i32 %122 to i64
  br i1 %125, label %136, label %127

127:                                              ; preds = %121
  %128 = load i64, ptr addrspace(1) %20, align 8
  %129 = mul nsw i64 %111, %23
  %130 = add nsw i64 %129, %83
  %131 = mul nsw i64 %124, %130
  %132 = mul nsw i64 %117, %55
  %133 = mul nsw i64 %132, %126
  %134 = add nsw i64 %131, %133
  %135 = add i64 %134, %128
  br label %145

136:                                              ; preds = %121
  %137 = mul nsw i64 %111, %23
  %138 = add nsw i64 %137, %83
  %139 = mul nsw i64 %138, %19
  %140 = add i64 %139, %55
  %141 = load i64, ptr addrspace(1) %20, align 8
  %142 = mul i64 %117, %140
  %143 = mul i64 %142, %126
  %144 = add i64 %141, %143
  br label %145

145:                                              ; preds = %136, %127
  %146 = phi i64 [ %135, %127 ], [ %144, %136 ]
  %147 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 7
  %148 = icmp eq i64 %118, 5
  %149 = sext i1 %148 to i32
  br label %150

150:                                              ; preds = %303, %145
  %151 = phi i32 [ 0, %145 ], [ %306, %303 ]
  %152 = phi <4 x double> [ zeroinitializer, %145 ], [ %305, %303 ]
  %153 = zext nneg i32 %151 to i64
  %154 = icmp sgt i64 %117, %153
  br i1 %154, label %155, label %300

155:                                              ; preds = %150
  %156 = mul nuw nsw i32 %151, %122
  %157 = zext nneg i32 %156 to i64
  %158 = add i64 %146, %157
  switch i64 %118, label %160 [
    i64 0, label %161
    i64 20, label %161
    i64 1, label %159
    i64 21, label %159
    i64 6, label %159
  ]

159:                                              ; preds = %155, %155, %155
  br label %161

160:                                              ; preds = %155
  br label %161

161:                                              ; preds = %160, %159, %155, %155
  %162 = phi i32 [ 4, %160 ], [ 2, %159 ], [ 1, %155 ], [ 1, %155 ]
  %163 = inttoptr i64 %158 to ptr addrspace(1)
  br label %165

164:                                              ; preds = %165
  switch i64 %118, label %268 [
    i64 6, label %177
    i64 7, label %264
    i64 0, label %269
    i64 20, label %269
    i64 1, label %267
    i64 21, label %267
  ]

165:                                              ; preds = %165, %161
  %166 = phi i32 [ 0, %161 ], [ %174, %165 ]
  %167 = phi i32 [ 0, %161 ], [ %175, %165 ]
  %168 = zext nneg i32 %167 to i64
  %169 = getelementptr inbounds i8, ptr addrspace(1) %163, i64 %168
  %170 = load i8, ptr addrspace(1) %169, align 1
  %171 = zext i8 %170 to i32
  %172 = shl nsw i32 %167, 3
  %173 = shl i32 %171, %172
  %174 = or i32 %173, %166
  %175 = add nuw nsw i32 %167, 1
  %176 = icmp eq i32 %175, %162
  br i1 %176, label %164, label %165, !llvm.loop !10016

177:                                              ; preds = %164
  %178 = lshr i32 %174, 10
  %179 = and i32 %178, 31
  %180 = and i32 %174, 1023
  switch i32 %179, label %187 [
    i32 0, label %181
    i32 31, label %184
  ]

181:                                              ; preds = %177
  %182 = uitofp i32 %180 to double
  %183 = fmul double %182, 0x3E70000000000000
  br label %259

184:                                              ; preds = %177
  %185 = icmp eq i32 %180, 0
  %186 = select i1 %185, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %259

187:                                              ; preds = %177
  %188 = or disjoint i32 %180, 1024
  %189 = uitofp i32 %188 to double
  %190 = add nsw i32 %179, -25
  %191 = icmp ugt i32 %179, 25
  br i1 %191, label %192, label %209

192:                                              ; preds = %187
  %193 = add nsw i32 %179, -26
  %194 = and i32 %190, 7
  %195 = icmp eq i32 %194, 0
  br i1 %195, label %204, label %196

196:                                              ; preds = %192, %196
  %197 = phi i32 [ %201, %196 ], [ %190, %192 ]
  %198 = phi double [ %200, %196 ], [ %189, %192 ]
  %199 = phi i32 [ %202, %196 ], [ 0, %192 ]
  %200 = fmul double %198, 2.000000e+00
  %201 = add nsw i32 %197, -1
  %202 = add i32 %199, 1
  %203 = icmp eq i32 %202, %194
  br i1 %203, label %204, label %196, !llvm.loop !10074

204:                                              ; preds = %196, %192
  %205 = phi i32 [ %190, %192 ], [ %201, %196 ]
  %206 = phi double [ %189, %192 ], [ %200, %196 ]
  %207 = phi double [ undef, %192 ], [ %200, %196 ]
  %208 = icmp ult i32 %193, 7
  br i1 %208, label %209, label %232

209:                                              ; preds = %204, %232, %187
  %210 = phi double [ %189, %187 ], [ %207, %204 ], [ %242, %232 ]
  %211 = phi i32 [ %190, %187 ], [ 0, %232 ], [ 0, %204 ]
  %212 = icmp slt i32 %211, 0
  br i1 %212, label %213, label %259

213:                                              ; preds = %209
  %214 = tail call i32 @llvm.umin.i32(i32 %179, i32 25)
  %215 = sub nsw i32 1, %214
  %216 = and i32 %215, 7
  %217 = icmp eq i32 %216, 0
  br i1 %217, label %226, label %218

218:                                              ; preds = %213, %218
  %219 = phi i32 [ %223, %218 ], [ %211, %213 ]
  %220 = phi double [ %222, %218 ], [ %210, %213 ]
  %221 = phi i32 [ %224, %218 ], [ 0, %213 ]
  %222 = fmul double %220, 5.000000e-01
  %223 = add nsw i32 %219, 1
  %224 = add i32 %221, 1
  %225 = icmp eq i32 %224, %216
  br i1 %225, label %226, label %218, !llvm.loop !10075

226:                                              ; preds = %218, %213
  %227 = phi double [ undef, %213 ], [ %222, %218 ]
  %228 = phi i32 [ %211, %213 ], [ %223, %218 ]
  %229 = phi double [ %210, %213 ], [ %222, %218 ]
  %230 = add nsw i32 %214, -18
  %231 = icmp ult i32 %230, 7
  br i1 %231, label %259, label %246

232:                                              ; preds = %204, %232
  %233 = phi i32 [ %243, %232 ], [ %205, %204 ]
  %234 = phi double [ %242, %232 ], [ %206, %204 ]
  %235 = fmul double %234, 2.000000e+00
  %236 = fmul double %235, 2.000000e+00
  %237 = fmul double %236, 2.000000e+00
  %238 = fmul double %237, 2.000000e+00
  %239 = fmul double %238, 2.000000e+00
  %240 = fmul double %239, 2.000000e+00
  %241 = fmul double %240, 2.000000e+00
  %242 = fmul double %241, 2.000000e+00
  %243 = add nsw i32 %233, -8
  %244 = add i32 %233, -9
  %245 = icmp ult i32 %244, -2
  br i1 %245, label %232, label %209, !llvm.loop !10021

246:                                              ; preds = %226, %246
  %247 = phi i32 [ %257, %246 ], [ %228, %226 ]
  %248 = phi double [ %256, %246 ], [ %229, %226 ]
  %249 = fmul double %248, 5.000000e-01
  %250 = fmul double %249, 5.000000e-01
  %251 = fmul double %250, 5.000000e-01
  %252 = fmul double %251, 5.000000e-01
  %253 = fmul double %252, 5.000000e-01
  %254 = fmul double %253, 5.000000e-01
  %255 = fmul double %254, 5.000000e-01
  %256 = fmul double %255, 5.000000e-01
  %257 = add nsw i32 %247, 8
  %258 = icmp eq i32 %257, 0
  br i1 %258, label %259, label %246, !llvm.loop !10022

259:                                              ; preds = %226, %246, %209, %184, %181
  %260 = phi double [ %183, %181 ], [ %186, %184 ], [ %210, %209 ], [ %227, %226 ], [ %256, %246 ]
  %261 = icmp ult i32 %174, 32768
  %262 = fneg double %260
  %263 = select i1 %261, double %260, double %262
  br label %303

264:                                              ; preds = %164
  %265 = bitcast i32 %174 to float
  %266 = fpext float %265 to double
  br label %303

267:                                              ; preds = %164, %164
  br label %269

268:                                              ; preds = %164
  br label %269

269:                                              ; preds = %268, %267, %164, %164
  %270 = phi i32 [ 32, %268 ], [ 16, %267 ], [ 8, %164 ], [ 8, %164 ]
  %271 = zext i32 %174 to i64
  switch i64 %118, label %281 [
    i64 21, label %272
    i64 20, label %272
    i64 3, label %272
    i64 5, label %272
  ]

272:                                              ; preds = %269, %269, %269, %269
  %273 = add nsw i32 %270, -1
  %274 = shl nuw i32 1, %273
  %275 = and i32 %274, %174
  %276 = icmp eq i32 %275, 0
  %277 = zext nneg i32 %270 to i64
  %278 = shl nsw i64 -1, %277
  %279 = select i1 %276, i64 0, i64 %278
  %280 = add nsw i64 %279, %271
  br label %281

281:                                              ; preds = %272, %269
  %282 = phi i64 [ %280, %272 ], [ %271, %269 ]
  %283 = load i64, ptr addrspace(1) %147, align 8
  %284 = icmp eq i64 %283, 0
  br i1 %284, label %285, label %287

285:                                              ; preds = %281
  %286 = sitofp i64 %282 to double
  br label %303

287:                                              ; preds = %281
  switch i64 %118, label %288 [
    i64 21, label %289
    i64 20, label %289
    i64 3, label %289
  ]

288:                                              ; preds = %287
  br label %289

289:                                              ; preds = %288, %287, %287, %287
  %290 = phi i32 [ -1, %287 ], [ %149, %288 ], [ -1, %287 ], [ -1, %287 ]
  %291 = add nsw i32 %290, %270
  %292 = zext nneg i32 %291 to i64
  %293 = shl nsw i64 -1, %292
  %294 = xor i64 %293, -1
  %295 = sitofp i64 %282 to double
  %296 = sitofp i64 %294 to double
  %297 = fdiv double %295, %296
  %298 = fcmp oge double %297, -1.000000e+00
  %299 = select i1 %298, double %297, double -1.000000e+00
  br label %303

300:                                              ; preds = %150
  %301 = icmp eq i32 %151, 3
  %302 = select i1 %301, double 1.000000e+00, double 0.000000e+00
  br label %303

303:                                              ; preds = %300, %289, %285, %264, %259
  %304 = phi double [ %302, %300 ], [ %263, %259 ], [ %266, %264 ], [ %299, %289 ], [ %286, %285 ]
  %305 = insertelement <4 x double> %152, double %304, i32 %151
  %306 = add nuw nsw i32 %151, 1
  %307 = icmp eq i32 %306, 4
  br i1 %307, label %308, label %150, !llvm.loop !10029

308:                                              ; preds = %303, %110
  %309 = phi <4 x double> [ zeroinitializer, %110 ], [ %305, %303 ]
  %310 = extractelement <4 x double> %309, i64 0
  %311 = fptrunc double %310 to float
  %312 = insertelement <4 x float> poison, float %311, i64 0
  %313 = extractelement <4 x double> %309, i64 1
  %314 = fptrunc double %313 to float
  %315 = insertelement <4 x float> %312, float %314, i64 1
  %316 = extractelement <4 x double> %309, i64 2
  %317 = fptrunc double %316 to float
  %318 = insertelement <4 x float> %315, float %317, i64 2
  %319 = extractelement <4 x double> %309, i64 3
  %320 = fptrunc double %319 to float
  %321 = insertelement <4 x float> %318, float %320, i64 3
  ret <4 x float> %321
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define dso_local <4 x i32> @__zluda_ptx_impl_texobj_1d_v4_s32_f32(i64 noundef %0, <1 x float> noundef %1) local_unnamed_addr #3 {
  %3 = inttoptr i64 %0 to ptr addrspace(1)
  %4 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 4
  %5 = load i64, ptr addrspace(1) %4, align 8
  %6 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 5
  %7 = load i64, ptr addrspace(1) %6, align 8
  %8 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 6
  %9 = load i64, ptr addrspace(1) %8, align 8
  %10 = getelementptr i64, ptr addrspace(1) %3, i64 9
  %11 = and i64 %9, 4294967295
  %12 = icmp eq i64 %11, 1
  %13 = trunc i64 %7 to i32
  %14 = extractelement <1 x float> %1, i64 0
  %15 = fpext float %14 to double
  %16 = and i64 %5, 4294967295
  %17 = icmp eq i64 %16, 1
  %18 = load i64, ptr addrspace(1) %10, align 8
  %19 = tail call noundef i64 @llvm.smax.i64(i64 %18, i64 1)
  br i1 %17, label %236, label %20

20:                                               ; preds = %2
  %21 = getelementptr i64, ptr addrspace(1) %3, i64 1
  %22 = getelementptr i64, ptr addrspace(1) %3, i64 2
  %23 = sitofp i64 %19 to double
  %24 = select i1 %12, double %23, double 1.000000e+00
  %25 = fmul double %24, %15
  %26 = tail call double @llvm.floor.f64(double %25)
  %27 = fptosi double %26 to i64
  switch i32 %13, label %45 [
    i32 0, label %28
    i32 2, label %32
    i32 3, label %36
  ]

28:                                               ; preds = %20
  %29 = srem i64 %27, %19
  %30 = add nsw i64 %29, %19
  %31 = srem i64 %30, %19
  br label %49

32:                                               ; preds = %20
  %33 = icmp sgt i64 %19, %27
  %34 = tail call i64 @llvm.smax.i64(i64 %27, i64 -1)
  %35 = select i1 %33, i64 %34, i64 -1
  br label %49

36:                                               ; preds = %20
  %37 = shl nuw nsw i64 %19, 1
  %38 = srem i64 %27, %37
  %39 = add nsw i64 %38, %37
  %40 = srem i64 %39, %37
  %41 = icmp slt i64 %40, %19
  %42 = xor i64 %40, -1
  %43 = add i64 %37, %42
  %44 = select i1 %41, i64 %40, i64 %43
  br label %49

45:                                               ; preds = %20
  %46 = tail call noundef i64 @llvm.smax.i64(i64 %27, i64 0)
  %47 = add nsw i64 %19, -1
  %48 = tail call noundef i64 @llvm.smin.i64(i64 %46, i64 %47)
  br label %49

49:                                               ; preds = %45, %36, %32, %28
  %50 = phi i64 [ %48, %45 ], [ %44, %36 ], [ %35, %32 ], [ %31, %28 ]
  %51 = icmp sgt i64 %50, -1
  br i1 %51, label %52, label %496

52:                                               ; preds = %49
  %53 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 3
  %54 = load i64, ptr addrspace(1) %53, align 8
  %55 = load i64, ptr addrspace(1) %22, align 8
  switch i64 %55, label %57 [
    i64 0, label %58
    i64 20, label %58
    i64 1, label %56
    i64 21, label %56
    i64 6, label %56
  ]

56:                                               ; preds = %52, %52, %52
  br label %58

57:                                               ; preds = %52
  br label %58

58:                                               ; preds = %57, %56, %52, %52
  %59 = phi i32 [ 4, %57 ], [ 2, %56 ], [ 1, %52 ], [ 1, %52 ]
  %60 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 14
  %61 = load i64, ptr addrspace(1) %60, align 8
  %62 = icmp eq i64 %61, 0
  %63 = load i64, ptr addrspace(1) %21, align 8
  br i1 %62, label %68, label %64

64:                                               ; preds = %58
  %65 = mul nsw i64 %54, %50
  %66 = zext nneg i32 %59 to i64
  %67 = mul nsw i64 %65, %66
  br label %72

68:                                               ; preds = %58
  %69 = zext nneg i32 %59 to i64
  %70 = mul i64 %54, %50
  %71 = mul i64 %70, %69
  br label %72

72:                                               ; preds = %68, %64
  %73 = phi i64 [ %71, %68 ], [ %67, %64 ]
  %74 = add i64 %63, %73
  %75 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 7
  %76 = icmp eq i64 %55, 5
  %77 = sext i1 %76 to i32
  br label %78

78:                                               ; preds = %231, %72
  %79 = phi i32 [ 0, %72 ], [ %234, %231 ]
  %80 = phi <4 x double> [ zeroinitializer, %72 ], [ %233, %231 ]
  %81 = zext nneg i32 %79 to i64
  %82 = icmp sgt i64 %54, %81
  br i1 %82, label %83, label %228

83:                                               ; preds = %78
  %84 = mul nuw nsw i32 %79, %59
  %85 = zext nneg i32 %84 to i64
  %86 = add i64 %74, %85
  switch i64 %55, label %88 [
    i64 0, label %89
    i64 20, label %89
    i64 1, label %87
    i64 21, label %87
    i64 6, label %87
  ]

87:                                               ; preds = %83, %83, %83
  br label %89

88:                                               ; preds = %83
  br label %89

89:                                               ; preds = %88, %87, %83, %83
  %90 = phi i32 [ 4, %88 ], [ 2, %87 ], [ 1, %83 ], [ 1, %83 ]
  %91 = inttoptr i64 %86 to ptr addrspace(1)
  br label %93

92:                                               ; preds = %93
  switch i64 %55, label %196 [
    i64 6, label %105
    i64 7, label %192
    i64 0, label %197
    i64 20, label %197
    i64 1, label %195
    i64 21, label %195
  ]

93:                                               ; preds = %93, %89
  %94 = phi i32 [ 0, %89 ], [ %102, %93 ]
  %95 = phi i32 [ 0, %89 ], [ %103, %93 ]
  %96 = zext nneg i32 %95 to i64
  %97 = getelementptr inbounds i8, ptr addrspace(1) %91, i64 %96
  %98 = load i8, ptr addrspace(1) %97, align 1
  %99 = zext i8 %98 to i32
  %100 = shl nsw i32 %95, 3
  %101 = shl i32 %99, %100
  %102 = or i32 %101, %94
  %103 = add nuw nsw i32 %95, 1
  %104 = icmp eq i32 %103, %90
  br i1 %104, label %92, label %93, !llvm.loop !10016

105:                                              ; preds = %92
  %106 = lshr i32 %102, 10
  %107 = and i32 %106, 31
  %108 = and i32 %102, 1023
  switch i32 %107, label %115 [
    i32 0, label %109
    i32 31, label %112
  ]

109:                                              ; preds = %105
  %110 = uitofp i32 %108 to double
  %111 = fmul double %110, 0x3E70000000000000
  br label %187

112:                                              ; preds = %105
  %113 = icmp eq i32 %108, 0
  %114 = select i1 %113, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %187

115:                                              ; preds = %105
  %116 = or disjoint i32 %108, 1024
  %117 = uitofp i32 %116 to double
  %118 = add nsw i32 %107, -25
  %119 = icmp ugt i32 %107, 25
  br i1 %119, label %120, label %137

120:                                              ; preds = %115
  %121 = add nsw i32 %107, -26
  %122 = and i32 %118, 7
  %123 = icmp eq i32 %122, 0
  br i1 %123, label %132, label %124

124:                                              ; preds = %120, %124
  %125 = phi i32 [ %129, %124 ], [ %118, %120 ]
  %126 = phi double [ %128, %124 ], [ %117, %120 ]
  %127 = phi i32 [ %130, %124 ], [ 0, %120 ]
  %128 = fmul double %126, 2.000000e+00
  %129 = add nsw i32 %125, -1
  %130 = add i32 %127, 1
  %131 = icmp eq i32 %130, %122
  br i1 %131, label %132, label %124, !llvm.loop !10076

132:                                              ; preds = %124, %120
  %133 = phi i32 [ %118, %120 ], [ %129, %124 ]
  %134 = phi double [ %117, %120 ], [ %128, %124 ]
  %135 = phi double [ undef, %120 ], [ %128, %124 ]
  %136 = icmp ult i32 %121, 7
  br i1 %136, label %137, label %160

137:                                              ; preds = %132, %160, %115
  %138 = phi double [ %117, %115 ], [ %135, %132 ], [ %170, %160 ]
  %139 = phi i32 [ %118, %115 ], [ 0, %160 ], [ 0, %132 ]
  %140 = icmp slt i32 %139, 0
  br i1 %140, label %141, label %187

141:                                              ; preds = %137
  %142 = tail call i32 @llvm.umin.i32(i32 %107, i32 25)
  %143 = sub nsw i32 1, %142
  %144 = and i32 %143, 7
  %145 = icmp eq i32 %144, 0
  br i1 %145, label %154, label %146

146:                                              ; preds = %141, %146
  %147 = phi i32 [ %151, %146 ], [ %139, %141 ]
  %148 = phi double [ %150, %146 ], [ %138, %141 ]
  %149 = phi i32 [ %152, %146 ], [ 0, %141 ]
  %150 = fmul double %148, 5.000000e-01
  %151 = add nsw i32 %147, 1
  %152 = add i32 %149, 1
  %153 = icmp eq i32 %152, %144
  br i1 %153, label %154, label %146, !llvm.loop !10077

154:                                              ; preds = %146, %141
  %155 = phi double [ undef, %141 ], [ %150, %146 ]
  %156 = phi i32 [ %139, %141 ], [ %151, %146 ]
  %157 = phi double [ %138, %141 ], [ %150, %146 ]
  %158 = add nsw i32 %142, -18
  %159 = icmp ult i32 %158, 7
  br i1 %159, label %187, label %174

160:                                              ; preds = %132, %160
  %161 = phi i32 [ %171, %160 ], [ %133, %132 ]
  %162 = phi double [ %170, %160 ], [ %134, %132 ]
  %163 = fmul double %162, 2.000000e+00
  %164 = fmul double %163, 2.000000e+00
  %165 = fmul double %164, 2.000000e+00
  %166 = fmul double %165, 2.000000e+00
  %167 = fmul double %166, 2.000000e+00
  %168 = fmul double %167, 2.000000e+00
  %169 = fmul double %168, 2.000000e+00
  %170 = fmul double %169, 2.000000e+00
  %171 = add nsw i32 %161, -8
  %172 = add i32 %161, -9
  %173 = icmp ult i32 %172, -2
  br i1 %173, label %160, label %137, !llvm.loop !10021

174:                                              ; preds = %154, %174
  %175 = phi i32 [ %185, %174 ], [ %156, %154 ]
  %176 = phi double [ %184, %174 ], [ %157, %154 ]
  %177 = fmul double %176, 5.000000e-01
  %178 = fmul double %177, 5.000000e-01
  %179 = fmul double %178, 5.000000e-01
  %180 = fmul double %179, 5.000000e-01
  %181 = fmul double %180, 5.000000e-01
  %182 = fmul double %181, 5.000000e-01
  %183 = fmul double %182, 5.000000e-01
  %184 = fmul double %183, 5.000000e-01
  %185 = add nsw i32 %175, 8
  %186 = icmp eq i32 %185, 0
  br i1 %186, label %187, label %174, !llvm.loop !10022

187:                                              ; preds = %154, %174, %137, %112, %109
  %188 = phi double [ %111, %109 ], [ %114, %112 ], [ %138, %137 ], [ %155, %154 ], [ %184, %174 ]
  %189 = icmp ult i32 %102, 32768
  %190 = fneg double %188
  %191 = select i1 %189, double %188, double %190
  br label %231

192:                                              ; preds = %92
  %193 = bitcast i32 %102 to float
  %194 = fpext float %193 to double
  br label %231

195:                                              ; preds = %92, %92
  br label %197

196:                                              ; preds = %92
  br label %197

197:                                              ; preds = %196, %195, %92, %92
  %198 = phi i32 [ 32, %196 ], [ 16, %195 ], [ 8, %92 ], [ 8, %92 ]
  %199 = zext i32 %102 to i64
  switch i64 %55, label %209 [
    i64 21, label %200
    i64 20, label %200
    i64 3, label %200
    i64 5, label %200
  ]

200:                                              ; preds = %197, %197, %197, %197
  %201 = add nsw i32 %198, -1
  %202 = shl nuw i32 1, %201
  %203 = and i32 %202, %102
  %204 = icmp eq i32 %203, 0
  %205 = zext nneg i32 %198 to i64
  %206 = shl nsw i64 -1, %205
  %207 = select i1 %204, i64 0, i64 %206
  %208 = add nsw i64 %207, %199
  br label %209

209:                                              ; preds = %200, %197
  %210 = phi i64 [ %208, %200 ], [ %199, %197 ]
  %211 = load i64, ptr addrspace(1) %75, align 8
  %212 = icmp eq i64 %211, 0
  br i1 %212, label %213, label %215

213:                                              ; preds = %209
  %214 = sitofp i64 %210 to double
  br label %231

215:                                              ; preds = %209
  switch i64 %55, label %216 [
    i64 21, label %217
    i64 20, label %217
    i64 3, label %217
  ]

216:                                              ; preds = %215
  br label %217

217:                                              ; preds = %216, %215, %215, %215
  %218 = phi i32 [ -1, %215 ], [ %77, %216 ], [ -1, %215 ], [ -1, %215 ]
  %219 = add nsw i32 %218, %198
  %220 = zext nneg i32 %219 to i64
  %221 = shl nsw i64 -1, %220
  %222 = xor i64 %221, -1
  %223 = sitofp i64 %210 to double
  %224 = sitofp i64 %222 to double
  %225 = fdiv double %223, %224
  %226 = fcmp oge double %225, -1.000000e+00
  %227 = select i1 %226, double %225, double -1.000000e+00
  br label %231

228:                                              ; preds = %78
  %229 = icmp eq i32 %79, 3
  %230 = select i1 %229, double 1.000000e+00, double 0.000000e+00
  br label %231

231:                                              ; preds = %228, %217, %213, %192, %187
  %232 = phi double [ %230, %228 ], [ %191, %187 ], [ %194, %192 ], [ %227, %217 ], [ %214, %213 ]
  %233 = insertelement <4 x double> %80, double %232, i32 %79
  %234 = add nuw nsw i32 %79, 1
  %235 = icmp eq i32 %234, 4
  br i1 %235, label %496, label %78, !llvm.loop !10029

236:                                              ; preds = %2
  %237 = sitofp i64 %19 to double
  %238 = select i1 %12, double %237, double 1.000000e+00
  %239 = fmul double %238, %15
  %240 = fadd double %239, -5.000000e-01
  %241 = tail call double @llvm.floor.f64(double %240)
  %242 = fptosi double %241 to i64
  switch i32 %13, label %260 [
    i32 0, label %243
    i32 2, label %247
    i32 3, label %251
  ]

243:                                              ; preds = %236
  %244 = srem i64 %242, %19
  %245 = add nsw i64 %244, %19
  %246 = srem i64 %245, %19
  br label %264

247:                                              ; preds = %236
  %248 = icmp sgt i64 %19, %242
  %249 = tail call i64 @llvm.smax.i64(i64 %242, i64 -1)
  %250 = select i1 %248, i64 %249, i64 -1
  br label %264

251:                                              ; preds = %236
  %252 = shl nuw nsw i64 %19, 1
  %253 = srem i64 %242, %252
  %254 = add nsw i64 %253, %252
  %255 = srem i64 %254, %252
  %256 = icmp slt i64 %255, %19
  %257 = xor i64 %255, -1
  %258 = add i64 %252, %257
  %259 = select i1 %256, i64 %255, i64 %258
  br label %264

260:                                              ; preds = %236
  %261 = tail call noundef i64 @llvm.smax.i64(i64 %242, i64 0)
  %262 = add nsw i64 %19, -1
  %263 = tail call noundef i64 @llvm.smin.i64(i64 %261, i64 %262)
  br label %264

264:                                              ; preds = %260, %251, %247, %243
  %265 = phi i64 [ %263, %260 ], [ %259, %251 ], [ %250, %247 ], [ %246, %243 ]
  %266 = add i64 %242, 1
  switch i32 %13, label %284 [
    i32 0, label %267
    i32 2, label %271
    i32 3, label %275
  ]

267:                                              ; preds = %264
  %268 = srem i64 %266, %19
  %269 = add nsw i64 %268, %19
  %270 = srem i64 %269, %19
  br label %288

271:                                              ; preds = %264
  %272 = icmp slt i64 %266, %19
  %273 = tail call i64 @llvm.smax.i64(i64 %266, i64 -1)
  %274 = select i1 %272, i64 %273, i64 -1
  br label %288

275:                                              ; preds = %264
  %276 = shl nuw nsw i64 %19, 1
  %277 = srem i64 %266, %276
  %278 = add nsw i64 %277, %276
  %279 = srem i64 %278, %276
  %280 = icmp slt i64 %279, %19
  %281 = xor i64 %279, -1
  %282 = add i64 %276, %281
  %283 = select i1 %280, i64 %279, i64 %282
  br label %288

284:                                              ; preds = %264
  %285 = tail call noundef i64 @llvm.smax.i64(i64 %266, i64 0)
  %286 = add nsw i64 %19, -1
  %287 = tail call noundef i64 @llvm.smin.i64(i64 %285, i64 %286)
  br label %288

288:                                              ; preds = %284, %275, %271, %267
  %289 = phi i64 [ %287, %284 ], [ %283, %275 ], [ %274, %271 ], [ %270, %267 ]
  %290 = fsub double %240, %241
  %291 = fmul double %290, 2.560000e+02
  %292 = fadd double %291, 5.000000e-01
  %293 = tail call double @llvm.floor.f64(double %292)
  %294 = fptosi double %293 to i64
  %295 = sitofp i64 %294 to double
  %296 = fmul double %295, 3.906250e-03
  %297 = fsub double 1.000000e+00, %296
  %298 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 3
  %299 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 2
  %300 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 14
  %301 = getelementptr i64, ptr addrspace(1) %3, i64 1
  %302 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 7
  br label %303

303:                                              ; preds = %492, %288
  %304 = phi <4 x double> [ zeroinitializer, %288 ], [ %495, %492 ]
  %305 = phi i1 [ true, %288 ], [ false, %492 ]
  %306 = select i1 %305, double %297, double %296
  %307 = insertelement <4 x double> poison, double %306, i64 0
  %308 = shufflevector <4 x double> %307, <4 x double> poison, <4 x i32> zeroinitializer
  %309 = select i1 %305, i64 %265, i64 %289
  %310 = icmp sgt i64 %309, -1
  br i1 %310, label %311, label %492

311:                                              ; preds = %303
  %312 = load i64, ptr addrspace(1) %298, align 8
  %313 = load i64, ptr addrspace(1) %299, align 8
  switch i64 %313, label %315 [
    i64 0, label %316
    i64 20, label %316
    i64 1, label %314
    i64 21, label %314
    i64 6, label %314
  ]

314:                                              ; preds = %311, %311, %311
  br label %316

315:                                              ; preds = %311
  br label %316

316:                                              ; preds = %315, %314, %311, %311
  %317 = phi i32 [ 4, %315 ], [ 2, %314 ], [ 1, %311 ], [ 1, %311 ]
  %318 = load i64, ptr addrspace(1) %300, align 8
  %319 = icmp eq i64 %318, 0
  %320 = load i64, ptr addrspace(1) %301, align 8
  br i1 %319, label %325, label %321

321:                                              ; preds = %316
  %322 = mul nsw i64 %312, %309
  %323 = zext nneg i32 %317 to i64
  %324 = mul nsw i64 %322, %323
  br label %329

325:                                              ; preds = %316
  %326 = zext nneg i32 %317 to i64
  %327 = mul i64 %312, %309
  %328 = mul i64 %327, %326
  br label %329

329:                                              ; preds = %325, %321
  %330 = phi i64 [ %328, %325 ], [ %324, %321 ]
  %331 = add i64 %320, %330
  %332 = icmp eq i64 %313, 5
  %333 = sext i1 %332 to i32
  br label %334

334:                                              ; preds = %487, %329
  %335 = phi i32 [ 0, %329 ], [ %490, %487 ]
  %336 = phi <4 x double> [ zeroinitializer, %329 ], [ %489, %487 ]
  %337 = zext nneg i32 %335 to i64
  %338 = icmp sgt i64 %312, %337
  br i1 %338, label %339, label %484

339:                                              ; preds = %334
  %340 = mul nuw nsw i32 %335, %317
  %341 = zext nneg i32 %340 to i64
  %342 = add i64 %331, %341
  switch i64 %313, label %344 [
    i64 0, label %345
    i64 20, label %345
    i64 1, label %343
    i64 21, label %343
    i64 6, label %343
  ]

343:                                              ; preds = %339, %339, %339
  br label %345

344:                                              ; preds = %339
  br label %345

345:                                              ; preds = %344, %343, %339, %339
  %346 = phi i32 [ 4, %344 ], [ 2, %343 ], [ 1, %339 ], [ 1, %339 ]
  %347 = inttoptr i64 %342 to ptr addrspace(1)
  br label %349

348:                                              ; preds = %349
  switch i64 %313, label %452 [
    i64 6, label %361
    i64 7, label %448
    i64 0, label %453
    i64 20, label %453
    i64 1, label %451
    i64 21, label %451
  ]

349:                                              ; preds = %349, %345
  %350 = phi i32 [ 0, %345 ], [ %358, %349 ]
  %351 = phi i32 [ 0, %345 ], [ %359, %349 ]
  %352 = zext nneg i32 %351 to i64
  %353 = getelementptr inbounds i8, ptr addrspace(1) %347, i64 %352
  %354 = load i8, ptr addrspace(1) %353, align 1
  %355 = zext i8 %354 to i32
  %356 = shl nsw i32 %351, 3
  %357 = shl i32 %355, %356
  %358 = or i32 %357, %350
  %359 = add nuw nsw i32 %351, 1
  %360 = icmp eq i32 %359, %346
  br i1 %360, label %348, label %349, !llvm.loop !10016

361:                                              ; preds = %348
  %362 = lshr i32 %358, 10
  %363 = and i32 %362, 31
  %364 = and i32 %358, 1023
  switch i32 %363, label %371 [
    i32 0, label %365
    i32 31, label %368
  ]

365:                                              ; preds = %361
  %366 = uitofp i32 %364 to double
  %367 = fmul double %366, 0x3E70000000000000
  br label %443

368:                                              ; preds = %361
  %369 = icmp eq i32 %364, 0
  %370 = select i1 %369, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %443

371:                                              ; preds = %361
  %372 = or disjoint i32 %364, 1024
  %373 = uitofp i32 %372 to double
  %374 = add nsw i32 %363, -25
  %375 = icmp ugt i32 %363, 25
  br i1 %375, label %376, label %393

376:                                              ; preds = %371
  %377 = add nsw i32 %363, -26
  %378 = and i32 %374, 7
  %379 = icmp eq i32 %378, 0
  br i1 %379, label %388, label %380

380:                                              ; preds = %376, %380
  %381 = phi i32 [ %385, %380 ], [ %374, %376 ]
  %382 = phi double [ %384, %380 ], [ %373, %376 ]
  %383 = phi i32 [ %386, %380 ], [ 0, %376 ]
  %384 = fmul double %382, 2.000000e+00
  %385 = add nsw i32 %381, -1
  %386 = add i32 %383, 1
  %387 = icmp eq i32 %386, %378
  br i1 %387, label %388, label %380, !llvm.loop !10078

388:                                              ; preds = %380, %376
  %389 = phi i32 [ %374, %376 ], [ %385, %380 ]
  %390 = phi double [ %373, %376 ], [ %384, %380 ]
  %391 = phi double [ undef, %376 ], [ %384, %380 ]
  %392 = icmp ult i32 %377, 7
  br i1 %392, label %393, label %416

393:                                              ; preds = %388, %416, %371
  %394 = phi double [ %373, %371 ], [ %391, %388 ], [ %426, %416 ]
  %395 = phi i32 [ %374, %371 ], [ 0, %416 ], [ 0, %388 ]
  %396 = icmp slt i32 %395, 0
  br i1 %396, label %397, label %443

397:                                              ; preds = %393
  %398 = tail call i32 @llvm.umin.i32(i32 %363, i32 25)
  %399 = sub nsw i32 1, %398
  %400 = and i32 %399, 7
  %401 = icmp eq i32 %400, 0
  br i1 %401, label %410, label %402

402:                                              ; preds = %397, %402
  %403 = phi i32 [ %407, %402 ], [ %395, %397 ]
  %404 = phi double [ %406, %402 ], [ %394, %397 ]
  %405 = phi i32 [ %408, %402 ], [ 0, %397 ]
  %406 = fmul double %404, 5.000000e-01
  %407 = add nsw i32 %403, 1
  %408 = add i32 %405, 1
  %409 = icmp eq i32 %408, %400
  br i1 %409, label %410, label %402, !llvm.loop !10079

410:                                              ; preds = %402, %397
  %411 = phi double [ undef, %397 ], [ %406, %402 ]
  %412 = phi i32 [ %395, %397 ], [ %407, %402 ]
  %413 = phi double [ %394, %397 ], [ %406, %402 ]
  %414 = add nsw i32 %398, -18
  %415 = icmp ult i32 %414, 7
  br i1 %415, label %443, label %430

416:                                              ; preds = %388, %416
  %417 = phi i32 [ %427, %416 ], [ %389, %388 ]
  %418 = phi double [ %426, %416 ], [ %390, %388 ]
  %419 = fmul double %418, 2.000000e+00
  %420 = fmul double %419, 2.000000e+00
  %421 = fmul double %420, 2.000000e+00
  %422 = fmul double %421, 2.000000e+00
  %423 = fmul double %422, 2.000000e+00
  %424 = fmul double %423, 2.000000e+00
  %425 = fmul double %424, 2.000000e+00
  %426 = fmul double %425, 2.000000e+00
  %427 = add nsw i32 %417, -8
  %428 = add i32 %417, -9
  %429 = icmp ult i32 %428, -2
  br i1 %429, label %416, label %393, !llvm.loop !10021

430:                                              ; preds = %410, %430
  %431 = phi i32 [ %441, %430 ], [ %412, %410 ]
  %432 = phi double [ %440, %430 ], [ %413, %410 ]
  %433 = fmul double %432, 5.000000e-01
  %434 = fmul double %433, 5.000000e-01
  %435 = fmul double %434, 5.000000e-01
  %436 = fmul double %435, 5.000000e-01
  %437 = fmul double %436, 5.000000e-01
  %438 = fmul double %437, 5.000000e-01
  %439 = fmul double %438, 5.000000e-01
  %440 = fmul double %439, 5.000000e-01
  %441 = add nsw i32 %431, 8
  %442 = icmp eq i32 %441, 0
  br i1 %442, label %443, label %430, !llvm.loop !10022

443:                                              ; preds = %410, %430, %393, %368, %365
  %444 = phi double [ %367, %365 ], [ %370, %368 ], [ %394, %393 ], [ %411, %410 ], [ %440, %430 ]
  %445 = icmp ult i32 %358, 32768
  %446 = fneg double %444
  %447 = select i1 %445, double %444, double %446
  br label %487

448:                                              ; preds = %348
  %449 = bitcast i32 %358 to float
  %450 = fpext float %449 to double
  br label %487

451:                                              ; preds = %348, %348
  br label %453

452:                                              ; preds = %348
  br label %453

453:                                              ; preds = %452, %451, %348, %348
  %454 = phi i32 [ 32, %452 ], [ 16, %451 ], [ 8, %348 ], [ 8, %348 ]
  %455 = zext i32 %358 to i64
  switch i64 %313, label %465 [
    i64 21, label %456
    i64 20, label %456
    i64 3, label %456
    i64 5, label %456
  ]

456:                                              ; preds = %453, %453, %453, %453
  %457 = add nsw i32 %454, -1
  %458 = shl nuw i32 1, %457
  %459 = and i32 %458, %358
  %460 = icmp eq i32 %459, 0
  %461 = zext nneg i32 %454 to i64
  %462 = shl nsw i64 -1, %461
  %463 = select i1 %460, i64 0, i64 %462
  %464 = add nsw i64 %463, %455
  br label %465

465:                                              ; preds = %456, %453
  %466 = phi i64 [ %464, %456 ], [ %455, %453 ]
  %467 = load i64, ptr addrspace(1) %302, align 8
  %468 = icmp eq i64 %467, 0
  br i1 %468, label %469, label %471

469:                                              ; preds = %465
  %470 = sitofp i64 %466 to double
  br label %487

471:                                              ; preds = %465
  switch i64 %313, label %472 [
    i64 21, label %473
    i64 20, label %473
    i64 3, label %473
  ]

472:                                              ; preds = %471
  br label %473

473:                                              ; preds = %472, %471, %471, %471
  %474 = phi i32 [ -1, %471 ], [ %333, %472 ], [ -1, %471 ], [ -1, %471 ]
  %475 = add nsw i32 %474, %454
  %476 = zext nneg i32 %475 to i64
  %477 = shl nsw i64 -1, %476
  %478 = xor i64 %477, -1
  %479 = sitofp i64 %466 to double
  %480 = sitofp i64 %478 to double
  %481 = fdiv double %479, %480
  %482 = fcmp oge double %481, -1.000000e+00
  %483 = select i1 %482, double %481, double -1.000000e+00
  br label %487

484:                                              ; preds = %334
  %485 = icmp eq i32 %335, 3
  %486 = select i1 %485, double 1.000000e+00, double 0.000000e+00
  br label %487

487:                                              ; preds = %484, %473, %469, %448, %443
  %488 = phi double [ %486, %484 ], [ %447, %443 ], [ %450, %448 ], [ %483, %473 ], [ %470, %469 ]
  %489 = insertelement <4 x double> %336, double %488, i32 %335
  %490 = add nuw nsw i32 %335, 1
  %491 = icmp eq i32 %490, 4
  br i1 %491, label %492, label %334, !llvm.loop !10029

492:                                              ; preds = %487, %303
  %493 = phi <4 x double> [ zeroinitializer, %303 ], [ %489, %487 ]
  %494 = fmul <4 x double> %308, %493
  %495 = fadd <4 x double> %304, %494
  br i1 %305, label %303, label %496, !llvm.loop !10032

496:                                              ; preds = %231, %492, %49
  %497 = phi <4 x double> [ zeroinitializer, %49 ], [ %495, %492 ], [ %233, %231 ]
  %498 = extractelement <4 x double> %497, i64 0
  %499 = fcmp uno double %498, 0.000000e+00
  %500 = fptosi double %498 to i64
  %501 = trunc i64 %500 to i32
  %502 = select i1 %499, i32 0, i32 %501
  %503 = insertelement <4 x i32> poison, i32 %502, i64 0
  %504 = extractelement <4 x double> %497, i64 1
  %505 = fcmp uno double %504, 0.000000e+00
  %506 = fptosi double %504 to i64
  %507 = trunc i64 %506 to i32
  %508 = select i1 %505, i32 0, i32 %507
  %509 = insertelement <4 x i32> %503, i32 %508, i64 1
  %510 = extractelement <4 x double> %497, i64 2
  %511 = fcmp uno double %510, 0.000000e+00
  %512 = fptosi double %510 to i64
  %513 = trunc i64 %512 to i32
  %514 = select i1 %511, i32 0, i32 %513
  %515 = insertelement <4 x i32> %509, i32 %514, i64 2
  %516 = extractelement <4 x double> %497, i64 3
  %517 = fcmp uno double %516, 0.000000e+00
  %518 = fptosi double %516 to i64
  %519 = trunc i64 %518 to i32
  %520 = select i1 %517, i32 0, i32 %519
  %521 = insertelement <4 x i32> %515, i32 %520, i64 3
  ret <4 x i32> %521
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define dso_local <4 x i32> @__zluda_ptx_impl_texobj_1d_v4_s32_s32(i64 noundef %0, <1 x i32> noundef %1) local_unnamed_addr #3 {
  %3 = inttoptr i64 %0 to ptr addrspace(1)
  %4 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 5
  %5 = load i64, ptr addrspace(1) %4, align 8
  %6 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 6
  %7 = load i64, ptr addrspace(1) %6, align 8
  %8 = getelementptr i64, ptr addrspace(1) %3, i64 9
  %9 = and i64 %7, 4294967295
  %10 = icmp eq i64 %9, 1
  %11 = extractelement <1 x i32> %1, i64 0
  %12 = trunc i64 %5 to i32
  %13 = sitofp i32 %11 to double
  %14 = load i64, ptr addrspace(1) %8, align 8
  %15 = tail call noundef i64 @llvm.smax.i64(i64 %14, i64 1)
  %16 = getelementptr i64, ptr addrspace(1) %3, i64 1
  %17 = getelementptr i64, ptr addrspace(1) %3, i64 2
  %18 = sitofp i64 %15 to double
  %19 = select i1 %10, double %18, double 1.000000e+00
  %20 = fmul double %19, %13
  %21 = tail call double @llvm.floor.f64(double %20)
  %22 = fptosi double %21 to i64
  switch i32 %12, label %40 [
    i32 0, label %23
    i32 2, label %27
    i32 3, label %31
  ]

23:                                               ; preds = %2
  %24 = srem i64 %22, %15
  %25 = add nsw i64 %24, %15
  %26 = srem i64 %25, %15
  br label %44

27:                                               ; preds = %2
  %28 = icmp sgt i64 %15, %22
  %29 = tail call i64 @llvm.smax.i64(i64 %22, i64 -1)
  %30 = select i1 %28, i64 %29, i64 -1
  br label %44

31:                                               ; preds = %2
  %32 = shl nuw nsw i64 %15, 1
  %33 = srem i64 %22, %32
  %34 = add nsw i64 %33, %32
  %35 = srem i64 %34, %32
  %36 = icmp slt i64 %35, %15
  %37 = xor i64 %35, -1
  %38 = add i64 %32, %37
  %39 = select i1 %36, i64 %35, i64 %38
  br label %44

40:                                               ; preds = %2
  %41 = tail call noundef i64 @llvm.smax.i64(i64 %22, i64 0)
  %42 = add nsw i64 %15, -1
  %43 = tail call noundef i64 @llvm.smin.i64(i64 %41, i64 %42)
  br label %44

44:                                               ; preds = %40, %31, %27, %23
  %45 = phi i64 [ %43, %40 ], [ %39, %31 ], [ %30, %27 ], [ %26, %23 ]
  %46 = icmp sgt i64 %45, -1
  br i1 %46, label %47, label %231

47:                                               ; preds = %44
  %48 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 3
  %49 = load i64, ptr addrspace(1) %48, align 8
  %50 = load i64, ptr addrspace(1) %17, align 8
  switch i64 %50, label %52 [
    i64 0, label %53
    i64 20, label %53
    i64 1, label %51
    i64 21, label %51
    i64 6, label %51
  ]

51:                                               ; preds = %47, %47, %47
  br label %53

52:                                               ; preds = %47
  br label %53

53:                                               ; preds = %52, %51, %47, %47
  %54 = phi i32 [ 4, %52 ], [ 2, %51 ], [ 1, %47 ], [ 1, %47 ]
  %55 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 14
  %56 = load i64, ptr addrspace(1) %55, align 8
  %57 = icmp eq i64 %56, 0
  %58 = load i64, ptr addrspace(1) %16, align 8
  br i1 %57, label %63, label %59

59:                                               ; preds = %53
  %60 = mul nsw i64 %49, %45
  %61 = zext nneg i32 %54 to i64
  %62 = mul nsw i64 %60, %61
  br label %67

63:                                               ; preds = %53
  %64 = zext nneg i32 %54 to i64
  %65 = mul i64 %49, %45
  %66 = mul i64 %65, %64
  br label %67

67:                                               ; preds = %63, %59
  %68 = phi i64 [ %66, %63 ], [ %62, %59 ]
  %69 = add i64 %58, %68
  %70 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 7
  %71 = icmp eq i64 %50, 5
  %72 = sext i1 %71 to i32
  br label %73

73:                                               ; preds = %226, %67
  %74 = phi i32 [ 0, %67 ], [ %229, %226 ]
  %75 = phi <4 x double> [ zeroinitializer, %67 ], [ %228, %226 ]
  %76 = zext nneg i32 %74 to i64
  %77 = icmp sgt i64 %49, %76
  br i1 %77, label %78, label %223

78:                                               ; preds = %73
  %79 = mul nuw nsw i32 %74, %54
  %80 = zext nneg i32 %79 to i64
  %81 = add i64 %69, %80
  switch i64 %50, label %83 [
    i64 0, label %84
    i64 20, label %84
    i64 1, label %82
    i64 21, label %82
    i64 6, label %82
  ]

82:                                               ; preds = %78, %78, %78
  br label %84

83:                                               ; preds = %78
  br label %84

84:                                               ; preds = %83, %82, %78, %78
  %85 = phi i32 [ 4, %83 ], [ 2, %82 ], [ 1, %78 ], [ 1, %78 ]
  %86 = inttoptr i64 %81 to ptr addrspace(1)
  br label %88

87:                                               ; preds = %88
  switch i64 %50, label %191 [
    i64 6, label %100
    i64 7, label %187
    i64 0, label %192
    i64 20, label %192
    i64 1, label %190
    i64 21, label %190
  ]

88:                                               ; preds = %88, %84
  %89 = phi i32 [ 0, %84 ], [ %97, %88 ]
  %90 = phi i32 [ 0, %84 ], [ %98, %88 ]
  %91 = zext nneg i32 %90 to i64
  %92 = getelementptr inbounds i8, ptr addrspace(1) %86, i64 %91
  %93 = load i8, ptr addrspace(1) %92, align 1
  %94 = zext i8 %93 to i32
  %95 = shl nsw i32 %90, 3
  %96 = shl i32 %94, %95
  %97 = or i32 %96, %89
  %98 = add nuw nsw i32 %90, 1
  %99 = icmp eq i32 %98, %85
  br i1 %99, label %87, label %88, !llvm.loop !10016

100:                                              ; preds = %87
  %101 = lshr i32 %97, 10
  %102 = and i32 %101, 31
  %103 = and i32 %97, 1023
  switch i32 %102, label %110 [
    i32 0, label %104
    i32 31, label %107
  ]

104:                                              ; preds = %100
  %105 = uitofp i32 %103 to double
  %106 = fmul double %105, 0x3E70000000000000
  br label %182

107:                                              ; preds = %100
  %108 = icmp eq i32 %103, 0
  %109 = select i1 %108, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %182

110:                                              ; preds = %100
  %111 = or disjoint i32 %103, 1024
  %112 = uitofp i32 %111 to double
  %113 = add nsw i32 %102, -25
  %114 = icmp ugt i32 %102, 25
  br i1 %114, label %115, label %132

115:                                              ; preds = %110
  %116 = add nsw i32 %102, -26
  %117 = and i32 %113, 7
  %118 = icmp eq i32 %117, 0
  br i1 %118, label %127, label %119

119:                                              ; preds = %115, %119
  %120 = phi i32 [ %124, %119 ], [ %113, %115 ]
  %121 = phi double [ %123, %119 ], [ %112, %115 ]
  %122 = phi i32 [ %125, %119 ], [ 0, %115 ]
  %123 = fmul double %121, 2.000000e+00
  %124 = add nsw i32 %120, -1
  %125 = add i32 %122, 1
  %126 = icmp eq i32 %125, %117
  br i1 %126, label %127, label %119, !llvm.loop !10080

127:                                              ; preds = %119, %115
  %128 = phi i32 [ %113, %115 ], [ %124, %119 ]
  %129 = phi double [ %112, %115 ], [ %123, %119 ]
  %130 = phi double [ undef, %115 ], [ %123, %119 ]
  %131 = icmp ult i32 %116, 7
  br i1 %131, label %132, label %155

132:                                              ; preds = %127, %155, %110
  %133 = phi double [ %112, %110 ], [ %130, %127 ], [ %165, %155 ]
  %134 = phi i32 [ %113, %110 ], [ 0, %155 ], [ 0, %127 ]
  %135 = icmp slt i32 %134, 0
  br i1 %135, label %136, label %182

136:                                              ; preds = %132
  %137 = tail call i32 @llvm.umin.i32(i32 %102, i32 25)
  %138 = sub nsw i32 1, %137
  %139 = and i32 %138, 7
  %140 = icmp eq i32 %139, 0
  br i1 %140, label %149, label %141

141:                                              ; preds = %136, %141
  %142 = phi i32 [ %146, %141 ], [ %134, %136 ]
  %143 = phi double [ %145, %141 ], [ %133, %136 ]
  %144 = phi i32 [ %147, %141 ], [ 0, %136 ]
  %145 = fmul double %143, 5.000000e-01
  %146 = add nsw i32 %142, 1
  %147 = add i32 %144, 1
  %148 = icmp eq i32 %147, %139
  br i1 %148, label %149, label %141, !llvm.loop !10081

149:                                              ; preds = %141, %136
  %150 = phi double [ undef, %136 ], [ %145, %141 ]
  %151 = phi i32 [ %134, %136 ], [ %146, %141 ]
  %152 = phi double [ %133, %136 ], [ %145, %141 ]
  %153 = add nsw i32 %137, -18
  %154 = icmp ult i32 %153, 7
  br i1 %154, label %182, label %169

155:                                              ; preds = %127, %155
  %156 = phi i32 [ %166, %155 ], [ %128, %127 ]
  %157 = phi double [ %165, %155 ], [ %129, %127 ]
  %158 = fmul double %157, 2.000000e+00
  %159 = fmul double %158, 2.000000e+00
  %160 = fmul double %159, 2.000000e+00
  %161 = fmul double %160, 2.000000e+00
  %162 = fmul double %161, 2.000000e+00
  %163 = fmul double %162, 2.000000e+00
  %164 = fmul double %163, 2.000000e+00
  %165 = fmul double %164, 2.000000e+00
  %166 = add nsw i32 %156, -8
  %167 = add i32 %156, -9
  %168 = icmp ult i32 %167, -2
  br i1 %168, label %155, label %132, !llvm.loop !10021

169:                                              ; preds = %149, %169
  %170 = phi i32 [ %180, %169 ], [ %151, %149 ]
  %171 = phi double [ %179, %169 ], [ %152, %149 ]
  %172 = fmul double %171, 5.000000e-01
  %173 = fmul double %172, 5.000000e-01
  %174 = fmul double %173, 5.000000e-01
  %175 = fmul double %174, 5.000000e-01
  %176 = fmul double %175, 5.000000e-01
  %177 = fmul double %176, 5.000000e-01
  %178 = fmul double %177, 5.000000e-01
  %179 = fmul double %178, 5.000000e-01
  %180 = add nsw i32 %170, 8
  %181 = icmp eq i32 %180, 0
  br i1 %181, label %182, label %169, !llvm.loop !10022

182:                                              ; preds = %149, %169, %132, %107, %104
  %183 = phi double [ %106, %104 ], [ %109, %107 ], [ %133, %132 ], [ %150, %149 ], [ %179, %169 ]
  %184 = icmp ult i32 %97, 32768
  %185 = fneg double %183
  %186 = select i1 %184, double %183, double %185
  br label %226

187:                                              ; preds = %87
  %188 = bitcast i32 %97 to float
  %189 = fpext float %188 to double
  br label %226

190:                                              ; preds = %87, %87
  br label %192

191:                                              ; preds = %87
  br label %192

192:                                              ; preds = %191, %190, %87, %87
  %193 = phi i32 [ 32, %191 ], [ 16, %190 ], [ 8, %87 ], [ 8, %87 ]
  %194 = zext i32 %97 to i64
  switch i64 %50, label %204 [
    i64 21, label %195
    i64 20, label %195
    i64 3, label %195
    i64 5, label %195
  ]

195:                                              ; preds = %192, %192, %192, %192
  %196 = add nsw i32 %193, -1
  %197 = shl nuw i32 1, %196
  %198 = and i32 %197, %97
  %199 = icmp eq i32 %198, 0
  %200 = zext nneg i32 %193 to i64
  %201 = shl nsw i64 -1, %200
  %202 = select i1 %199, i64 0, i64 %201
  %203 = add nsw i64 %202, %194
  br label %204

204:                                              ; preds = %195, %192
  %205 = phi i64 [ %203, %195 ], [ %194, %192 ]
  %206 = load i64, ptr addrspace(1) %70, align 8
  %207 = icmp eq i64 %206, 0
  br i1 %207, label %208, label %210

208:                                              ; preds = %204
  %209 = sitofp i64 %205 to double
  br label %226

210:                                              ; preds = %204
  switch i64 %50, label %211 [
    i64 21, label %212
    i64 20, label %212
    i64 3, label %212
  ]

211:                                              ; preds = %210
  br label %212

212:                                              ; preds = %211, %210, %210, %210
  %213 = phi i32 [ -1, %210 ], [ %72, %211 ], [ -1, %210 ], [ -1, %210 ]
  %214 = add nsw i32 %213, %193
  %215 = zext nneg i32 %214 to i64
  %216 = shl nsw i64 -1, %215
  %217 = xor i64 %216, -1
  %218 = sitofp i64 %205 to double
  %219 = sitofp i64 %217 to double
  %220 = fdiv double %218, %219
  %221 = fcmp oge double %220, -1.000000e+00
  %222 = select i1 %221, double %220, double -1.000000e+00
  br label %226

223:                                              ; preds = %73
  %224 = icmp eq i32 %74, 3
  %225 = select i1 %224, double 1.000000e+00, double 0.000000e+00
  br label %226

226:                                              ; preds = %223, %212, %208, %187, %182
  %227 = phi double [ %225, %223 ], [ %186, %182 ], [ %189, %187 ], [ %222, %212 ], [ %209, %208 ]
  %228 = insertelement <4 x double> %75, double %227, i32 %74
  %229 = add nuw nsw i32 %74, 1
  %230 = icmp eq i32 %229, 4
  br i1 %230, label %231, label %73, !llvm.loop !10029

231:                                              ; preds = %226, %44
  %232 = phi <4 x double> [ zeroinitializer, %44 ], [ %228, %226 ]
  %233 = extractelement <4 x double> %232, i64 0
  %234 = fcmp uno double %233, 0.000000e+00
  %235 = fptosi double %233 to i64
  %236 = trunc i64 %235 to i32
  %237 = select i1 %234, i32 0, i32 %236
  %238 = insertelement <4 x i32> poison, i32 %237, i64 0
  %239 = extractelement <4 x double> %232, i64 1
  %240 = fcmp uno double %239, 0.000000e+00
  %241 = fptosi double %239 to i64
  %242 = trunc i64 %241 to i32
  %243 = select i1 %240, i32 0, i32 %242
  %244 = insertelement <4 x i32> %238, i32 %243, i64 1
  %245 = extractelement <4 x double> %232, i64 2
  %246 = fcmp uno double %245, 0.000000e+00
  %247 = fptosi double %245 to i64
  %248 = trunc i64 %247 to i32
  %249 = select i1 %246, i32 0, i32 %248
  %250 = insertelement <4 x i32> %244, i32 %249, i64 2
  %251 = extractelement <4 x double> %232, i64 3
  %252 = fcmp uno double %251, 0.000000e+00
  %253 = fptosi double %251 to i64
  %254 = trunc i64 %253 to i32
  %255 = select i1 %252, i32 0, i32 %254
  %256 = insertelement <4 x i32> %250, i32 %255, i64 3
  ret <4 x i32> %256
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define dso_local <4 x i32> @__zluda_ptx_impl_texobj_2d_v4_s32_f32(i64 noundef %0, <2 x float> noundef %1) local_unnamed_addr #3 {
  %3 = inttoptr i64 %0 to ptr addrspace(1)
  %4 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 4
  %5 = load i64, ptr addrspace(1) %4, align 8
  %6 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 5
  %7 = load i64, ptr addrspace(1) %6, align 8
  %8 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 6
  %9 = load i64, ptr addrspace(1) %8, align 8
  %10 = getelementptr i64, ptr addrspace(1) %3, i64 9
  %11 = and i64 %9, 4294967295
  %12 = icmp eq i64 %11, 1
  %13 = trunc i64 %7 to i32
  %14 = extractelement <2 x float> %1, i64 0
  %15 = fpext float %14 to double
  %16 = extractelement <2 x float> %1, i64 1
  %17 = fpext float %16 to double
  %18 = and i64 %5, 4294967295
  %19 = icmp eq i64 %18, 1
  %20 = load i64, ptr addrspace(1) %10, align 8
  %21 = tail call noundef i64 @llvm.smax.i64(i64 %20, i64 1)
  br i1 %19, label %275, label %22

22:                                               ; preds = %2
  %23 = getelementptr i64, ptr addrspace(1) %3, i64 1
  %24 = getelementptr i64, ptr addrspace(1) %3, i64 10
  %25 = load i64, ptr addrspace(1) %24, align 8
  %26 = tail call noundef i64 @llvm.smax.i64(i64 %25, i64 1)
  %27 = getelementptr i64, ptr addrspace(1) %3, i64 2
  %28 = sitofp i64 %21 to double
  %29 = select i1 %12, double %28, double 1.000000e+00
  %30 = fmul double %29, %15
  %31 = tail call double @llvm.floor.f64(double %30)
  %32 = fptosi double %31 to i64
  switch i32 %13, label %50 [
    i32 0, label %33
    i32 2, label %37
    i32 3, label %41
  ]

33:                                               ; preds = %22
  %34 = srem i64 %32, %21
  %35 = add nsw i64 %34, %21
  %36 = srem i64 %35, %21
  br label %54

37:                                               ; preds = %22
  %38 = icmp sgt i64 %21, %32
  %39 = tail call i64 @llvm.smax.i64(i64 %32, i64 -1)
  %40 = select i1 %38, i64 %39, i64 -1
  br label %54

41:                                               ; preds = %22
  %42 = shl nuw nsw i64 %21, 1
  %43 = srem i64 %32, %42
  %44 = add nsw i64 %43, %42
  %45 = srem i64 %44, %42
  %46 = icmp slt i64 %45, %21
  %47 = xor i64 %45, -1
  %48 = add i64 %42, %47
  %49 = select i1 %46, i64 %45, i64 %48
  br label %54

50:                                               ; preds = %22
  %51 = tail call noundef i64 @llvm.smax.i64(i64 %32, i64 0)
  %52 = add nsw i64 %21, -1
  %53 = tail call noundef i64 @llvm.smin.i64(i64 %51, i64 %52)
  br label %54

54:                                               ; preds = %50, %41, %37, %33
  %55 = phi i64 [ %53, %50 ], [ %49, %41 ], [ %40, %37 ], [ %36, %33 ]
  %56 = sitofp i64 %26 to double
  %57 = select i1 %12, double %56, double 1.000000e+00
  %58 = fmul double %57, %17
  %59 = tail call double @llvm.floor.f64(double %58)
  %60 = fptosi double %59 to i64
  switch i32 %13, label %78 [
    i32 0, label %61
    i32 2, label %65
    i32 3, label %69
  ]

61:                                               ; preds = %54
  %62 = srem i64 %60, %26
  %63 = add nsw i64 %62, %26
  %64 = srem i64 %63, %26
  br label %82

65:                                               ; preds = %54
  %66 = icmp sgt i64 %26, %60
  %67 = tail call i64 @llvm.smax.i64(i64 %60, i64 -1)
  %68 = select i1 %66, i64 %67, i64 -1
  br label %82

69:                                               ; preds = %54
  %70 = shl nuw nsw i64 %26, 1
  %71 = srem i64 %60, %70
  %72 = add nsw i64 %71, %70
  %73 = srem i64 %72, %70
  %74 = icmp slt i64 %73, %26
  %75 = xor i64 %73, -1
  %76 = add i64 %70, %75
  %77 = select i1 %74, i64 %73, i64 %76
  br label %82

78:                                               ; preds = %54
  %79 = tail call noundef i64 @llvm.smax.i64(i64 %60, i64 0)
  %80 = add nsw i64 %26, -1
  %81 = tail call noundef i64 @llvm.smin.i64(i64 %79, i64 %80)
  br label %82

82:                                               ; preds = %78, %69, %65, %61
  %83 = phi i64 [ %81, %78 ], [ %77, %69 ], [ %68, %65 ], [ %64, %61 ]
  %84 = or i64 %83, %55
  %85 = icmp sgt i64 %84, -1
  br i1 %85, label %86, label %618

86:                                               ; preds = %82
  %87 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 3
  %88 = load i64, ptr addrspace(1) %87, align 8
  %89 = load i64, ptr addrspace(1) %27, align 8
  switch i64 %89, label %91 [
    i64 0, label %92
    i64 20, label %92
    i64 1, label %90
    i64 21, label %90
    i64 6, label %90
  ]

90:                                               ; preds = %86, %86, %86
  br label %92

91:                                               ; preds = %86
  br label %92

92:                                               ; preds = %91, %90, %86, %86
  %93 = phi i32 [ 4, %91 ], [ 2, %90 ], [ 1, %86 ], [ 1, %86 ]
  %94 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 14
  %95 = load i64, ptr addrspace(1) %94, align 8
  %96 = icmp eq i64 %95, 0
  %97 = zext nneg i32 %93 to i64
  br i1 %96, label %105, label %98

98:                                               ; preds = %92
  %99 = load i64, ptr addrspace(1) %23, align 8
  %100 = mul nsw i64 %95, %83
  %101 = mul nsw i64 %88, %55
  %102 = mul nsw i64 %101, %97
  %103 = add nsw i64 %100, %102
  %104 = add i64 %103, %99
  br label %112

105:                                              ; preds = %92
  %106 = mul nsw i64 %83, %21
  %107 = add i64 %106, %55
  %108 = load i64, ptr addrspace(1) %23, align 8
  %109 = mul i64 %88, %107
  %110 = mul i64 %109, %97
  %111 = add i64 %108, %110
  br label %112

112:                                              ; preds = %105, %98
  %113 = phi i64 [ %104, %98 ], [ %111, %105 ]
  %114 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 7
  %115 = icmp eq i64 %89, 5
  %116 = sext i1 %115 to i32
  br label %117

117:                                              ; preds = %270, %112
  %118 = phi i32 [ 0, %112 ], [ %273, %270 ]
  %119 = phi <4 x double> [ zeroinitializer, %112 ], [ %272, %270 ]
  %120 = zext nneg i32 %118 to i64
  %121 = icmp sgt i64 %88, %120
  br i1 %121, label %122, label %267

122:                                              ; preds = %117
  %123 = mul nuw nsw i32 %118, %93
  %124 = zext nneg i32 %123 to i64
  %125 = add i64 %113, %124
  switch i64 %89, label %127 [
    i64 0, label %128
    i64 20, label %128
    i64 1, label %126
    i64 21, label %126
    i64 6, label %126
  ]

126:                                              ; preds = %122, %122, %122
  br label %128

127:                                              ; preds = %122
  br label %128

128:                                              ; preds = %127, %126, %122, %122
  %129 = phi i32 [ 4, %127 ], [ 2, %126 ], [ 1, %122 ], [ 1, %122 ]
  %130 = inttoptr i64 %125 to ptr addrspace(1)
  br label %132

131:                                              ; preds = %132
  switch i64 %89, label %235 [
    i64 6, label %144
    i64 7, label %231
    i64 0, label %236
    i64 20, label %236
    i64 1, label %234
    i64 21, label %234
  ]

132:                                              ; preds = %132, %128
  %133 = phi i32 [ 0, %128 ], [ %141, %132 ]
  %134 = phi i32 [ 0, %128 ], [ %142, %132 ]
  %135 = zext nneg i32 %134 to i64
  %136 = getelementptr inbounds i8, ptr addrspace(1) %130, i64 %135
  %137 = load i8, ptr addrspace(1) %136, align 1
  %138 = zext i8 %137 to i32
  %139 = shl nsw i32 %134, 3
  %140 = shl i32 %138, %139
  %141 = or i32 %140, %133
  %142 = add nuw nsw i32 %134, 1
  %143 = icmp eq i32 %142, %129
  br i1 %143, label %131, label %132, !llvm.loop !10016

144:                                              ; preds = %131
  %145 = lshr i32 %141, 10
  %146 = and i32 %145, 31
  %147 = and i32 %141, 1023
  switch i32 %146, label %154 [
    i32 0, label %148
    i32 31, label %151
  ]

148:                                              ; preds = %144
  %149 = uitofp i32 %147 to double
  %150 = fmul double %149, 0x3E70000000000000
  br label %226

151:                                              ; preds = %144
  %152 = icmp eq i32 %147, 0
  %153 = select i1 %152, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %226

154:                                              ; preds = %144
  %155 = or disjoint i32 %147, 1024
  %156 = uitofp i32 %155 to double
  %157 = add nsw i32 %146, -25
  %158 = icmp ugt i32 %146, 25
  br i1 %158, label %159, label %176

159:                                              ; preds = %154
  %160 = add nsw i32 %146, -26
  %161 = and i32 %157, 7
  %162 = icmp eq i32 %161, 0
  br i1 %162, label %171, label %163

163:                                              ; preds = %159, %163
  %164 = phi i32 [ %168, %163 ], [ %157, %159 ]
  %165 = phi double [ %167, %163 ], [ %156, %159 ]
  %166 = phi i32 [ %169, %163 ], [ 0, %159 ]
  %167 = fmul double %165, 2.000000e+00
  %168 = add nsw i32 %164, -1
  %169 = add i32 %166, 1
  %170 = icmp eq i32 %169, %161
  br i1 %170, label %171, label %163, !llvm.loop !10082

171:                                              ; preds = %163, %159
  %172 = phi i32 [ %157, %159 ], [ %168, %163 ]
  %173 = phi double [ %156, %159 ], [ %167, %163 ]
  %174 = phi double [ undef, %159 ], [ %167, %163 ]
  %175 = icmp ult i32 %160, 7
  br i1 %175, label %176, label %199

176:                                              ; preds = %171, %199, %154
  %177 = phi double [ %156, %154 ], [ %174, %171 ], [ %209, %199 ]
  %178 = phi i32 [ %157, %154 ], [ 0, %199 ], [ 0, %171 ]
  %179 = icmp slt i32 %178, 0
  br i1 %179, label %180, label %226

180:                                              ; preds = %176
  %181 = tail call i32 @llvm.umin.i32(i32 %146, i32 25)
  %182 = sub nsw i32 1, %181
  %183 = and i32 %182, 7
  %184 = icmp eq i32 %183, 0
  br i1 %184, label %193, label %185

185:                                              ; preds = %180, %185
  %186 = phi i32 [ %190, %185 ], [ %178, %180 ]
  %187 = phi double [ %189, %185 ], [ %177, %180 ]
  %188 = phi i32 [ %191, %185 ], [ 0, %180 ]
  %189 = fmul double %187, 5.000000e-01
  %190 = add nsw i32 %186, 1
  %191 = add i32 %188, 1
  %192 = icmp eq i32 %191, %183
  br i1 %192, label %193, label %185, !llvm.loop !10083

193:                                              ; preds = %185, %180
  %194 = phi double [ undef, %180 ], [ %189, %185 ]
  %195 = phi i32 [ %178, %180 ], [ %190, %185 ]
  %196 = phi double [ %177, %180 ], [ %189, %185 ]
  %197 = add nsw i32 %181, -18
  %198 = icmp ult i32 %197, 7
  br i1 %198, label %226, label %213

199:                                              ; preds = %171, %199
  %200 = phi i32 [ %210, %199 ], [ %172, %171 ]
  %201 = phi double [ %209, %199 ], [ %173, %171 ]
  %202 = fmul double %201, 2.000000e+00
  %203 = fmul double %202, 2.000000e+00
  %204 = fmul double %203, 2.000000e+00
  %205 = fmul double %204, 2.000000e+00
  %206 = fmul double %205, 2.000000e+00
  %207 = fmul double %206, 2.000000e+00
  %208 = fmul double %207, 2.000000e+00
  %209 = fmul double %208, 2.000000e+00
  %210 = add nsw i32 %200, -8
  %211 = add i32 %200, -9
  %212 = icmp ult i32 %211, -2
  br i1 %212, label %199, label %176, !llvm.loop !10021

213:                                              ; preds = %193, %213
  %214 = phi i32 [ %224, %213 ], [ %195, %193 ]
  %215 = phi double [ %223, %213 ], [ %196, %193 ]
  %216 = fmul double %215, 5.000000e-01
  %217 = fmul double %216, 5.000000e-01
  %218 = fmul double %217, 5.000000e-01
  %219 = fmul double %218, 5.000000e-01
  %220 = fmul double %219, 5.000000e-01
  %221 = fmul double %220, 5.000000e-01
  %222 = fmul double %221, 5.000000e-01
  %223 = fmul double %222, 5.000000e-01
  %224 = add nsw i32 %214, 8
  %225 = icmp eq i32 %224, 0
  br i1 %225, label %226, label %213, !llvm.loop !10022

226:                                              ; preds = %193, %213, %176, %151, %148
  %227 = phi double [ %150, %148 ], [ %153, %151 ], [ %177, %176 ], [ %194, %193 ], [ %223, %213 ]
  %228 = icmp ult i32 %141, 32768
  %229 = fneg double %227
  %230 = select i1 %228, double %227, double %229
  br label %270

231:                                              ; preds = %131
  %232 = bitcast i32 %141 to float
  %233 = fpext float %232 to double
  br label %270

234:                                              ; preds = %131, %131
  br label %236

235:                                              ; preds = %131
  br label %236

236:                                              ; preds = %235, %234, %131, %131
  %237 = phi i32 [ 32, %235 ], [ 16, %234 ], [ 8, %131 ], [ 8, %131 ]
  %238 = zext i32 %141 to i64
  switch i64 %89, label %248 [
    i64 21, label %239
    i64 20, label %239
    i64 3, label %239
    i64 5, label %239
  ]

239:                                              ; preds = %236, %236, %236, %236
  %240 = add nsw i32 %237, -1
  %241 = shl nuw i32 1, %240
  %242 = and i32 %241, %141
  %243 = icmp eq i32 %242, 0
  %244 = zext nneg i32 %237 to i64
  %245 = shl nsw i64 -1, %244
  %246 = select i1 %243, i64 0, i64 %245
  %247 = add nsw i64 %246, %238
  br label %248

248:                                              ; preds = %239, %236
  %249 = phi i64 [ %247, %239 ], [ %238, %236 ]
  %250 = load i64, ptr addrspace(1) %114, align 8
  %251 = icmp eq i64 %250, 0
  br i1 %251, label %252, label %254

252:                                              ; preds = %248
  %253 = sitofp i64 %249 to double
  br label %270

254:                                              ; preds = %248
  switch i64 %89, label %255 [
    i64 21, label %256
    i64 20, label %256
    i64 3, label %256
  ]

255:                                              ; preds = %254
  br label %256

256:                                              ; preds = %255, %254, %254, %254
  %257 = phi i32 [ -1, %254 ], [ %116, %255 ], [ -1, %254 ], [ -1, %254 ]
  %258 = add nsw i32 %257, %237
  %259 = zext nneg i32 %258 to i64
  %260 = shl nsw i64 -1, %259
  %261 = xor i64 %260, -1
  %262 = sitofp i64 %249 to double
  %263 = sitofp i64 %261 to double
  %264 = fdiv double %262, %263
  %265 = fcmp oge double %264, -1.000000e+00
  %266 = select i1 %265, double %264, double -1.000000e+00
  br label %270

267:                                              ; preds = %117
  %268 = icmp eq i32 %118, 3
  %269 = select i1 %268, double 1.000000e+00, double 0.000000e+00
  br label %270

270:                                              ; preds = %267, %256, %252, %231, %226
  %271 = phi double [ %269, %267 ], [ %230, %226 ], [ %233, %231 ], [ %266, %256 ], [ %253, %252 ]
  %272 = insertelement <4 x double> %119, double %271, i32 %118
  %273 = add nuw nsw i32 %118, 1
  %274 = icmp eq i32 %273, 4
  br i1 %274, label %618, label %117, !llvm.loop !10029

275:                                              ; preds = %2
  %276 = sitofp i64 %21 to double
  %277 = select i1 %12, double %276, double 1.000000e+00
  %278 = fmul double %277, %15
  %279 = fadd double %278, -5.000000e-01
  %280 = tail call double @llvm.floor.f64(double %279)
  %281 = fptosi double %280 to i64
  switch i32 %13, label %299 [
    i32 0, label %282
    i32 2, label %286
    i32 3, label %290
  ]

282:                                              ; preds = %275
  %283 = srem i64 %281, %21
  %284 = add nsw i64 %283, %21
  %285 = srem i64 %284, %21
  br label %303

286:                                              ; preds = %275
  %287 = icmp sgt i64 %21, %281
  %288 = tail call i64 @llvm.smax.i64(i64 %281, i64 -1)
  %289 = select i1 %287, i64 %288, i64 -1
  br label %303

290:                                              ; preds = %275
  %291 = shl nuw nsw i64 %21, 1
  %292 = srem i64 %281, %291
  %293 = add nsw i64 %292, %291
  %294 = srem i64 %293, %291
  %295 = icmp slt i64 %294, %21
  %296 = xor i64 %294, -1
  %297 = add i64 %291, %296
  %298 = select i1 %295, i64 %294, i64 %297
  br label %303

299:                                              ; preds = %275
  %300 = tail call noundef i64 @llvm.smax.i64(i64 %281, i64 0)
  %301 = add nsw i64 %21, -1
  %302 = tail call noundef i64 @llvm.smin.i64(i64 %300, i64 %301)
  br label %303

303:                                              ; preds = %299, %290, %286, %282
  %304 = phi i64 [ %302, %299 ], [ %298, %290 ], [ %289, %286 ], [ %285, %282 ]
  %305 = add i64 %281, 1
  switch i32 %13, label %323 [
    i32 0, label %306
    i32 2, label %310
    i32 3, label %314
  ]

306:                                              ; preds = %303
  %307 = srem i64 %305, %21
  %308 = add nsw i64 %307, %21
  %309 = srem i64 %308, %21
  br label %327

310:                                              ; preds = %303
  %311 = icmp slt i64 %305, %21
  %312 = tail call i64 @llvm.smax.i64(i64 %305, i64 -1)
  %313 = select i1 %311, i64 %312, i64 -1
  br label %327

314:                                              ; preds = %303
  %315 = shl nuw nsw i64 %21, 1
  %316 = srem i64 %305, %315
  %317 = add nsw i64 %316, %315
  %318 = srem i64 %317, %315
  %319 = icmp slt i64 %318, %21
  %320 = xor i64 %318, -1
  %321 = add i64 %315, %320
  %322 = select i1 %319, i64 %318, i64 %321
  br label %327

323:                                              ; preds = %303
  %324 = tail call noundef i64 @llvm.smax.i64(i64 %305, i64 0)
  %325 = add nsw i64 %21, -1
  %326 = tail call noundef i64 @llvm.smin.i64(i64 %324, i64 %325)
  br label %327

327:                                              ; preds = %323, %314, %310, %306
  %328 = phi i64 [ %326, %323 ], [ %322, %314 ], [ %313, %310 ], [ %309, %306 ]
  %329 = getelementptr i64, ptr addrspace(1) %3, i64 10
  %330 = load i64, ptr addrspace(1) %329, align 8
  %331 = tail call noundef i64 @llvm.smax.i64(i64 %330, i64 1)
  %332 = sitofp i64 %331 to double
  %333 = select i1 %12, double %332, double 1.000000e+00
  %334 = fmul double %333, %17
  %335 = fadd double %334, -5.000000e-01
  %336 = tail call double @llvm.floor.f64(double %335)
  %337 = fptosi double %336 to i64
  switch i32 %13, label %355 [
    i32 0, label %338
    i32 2, label %342
    i32 3, label %346
  ]

338:                                              ; preds = %327
  %339 = srem i64 %337, %331
  %340 = add nsw i64 %339, %331
  %341 = srem i64 %340, %331
  br label %359

342:                                              ; preds = %327
  %343 = icmp sgt i64 %331, %337
  %344 = tail call i64 @llvm.smax.i64(i64 %337, i64 -1)
  %345 = select i1 %343, i64 %344, i64 -1
  br label %359

346:                                              ; preds = %327
  %347 = shl nuw nsw i64 %331, 1
  %348 = srem i64 %337, %347
  %349 = add nsw i64 %348, %347
  %350 = srem i64 %349, %347
  %351 = icmp slt i64 %350, %331
  %352 = xor i64 %350, -1
  %353 = add i64 %347, %352
  %354 = select i1 %351, i64 %350, i64 %353
  br label %359

355:                                              ; preds = %327
  %356 = tail call noundef i64 @llvm.smax.i64(i64 %337, i64 0)
  %357 = add nsw i64 %331, -1
  %358 = tail call noundef i64 @llvm.smin.i64(i64 %356, i64 %357)
  br label %359

359:                                              ; preds = %355, %346, %342, %338
  %360 = phi i64 [ %358, %355 ], [ %354, %346 ], [ %345, %342 ], [ %341, %338 ]
  %361 = add i64 %337, 1
  switch i32 %13, label %379 [
    i32 0, label %362
    i32 2, label %366
    i32 3, label %370
  ]

362:                                              ; preds = %359
  %363 = srem i64 %361, %331
  %364 = add nsw i64 %363, %331
  %365 = srem i64 %364, %331
  br label %383

366:                                              ; preds = %359
  %367 = icmp slt i64 %361, %331
  %368 = tail call i64 @llvm.smax.i64(i64 %361, i64 -1)
  %369 = select i1 %367, i64 %368, i64 -1
  br label %383

370:                                              ; preds = %359
  %371 = shl nuw nsw i64 %331, 1
  %372 = srem i64 %361, %371
  %373 = add nsw i64 %372, %371
  %374 = srem i64 %373, %371
  %375 = icmp slt i64 %374, %331
  %376 = xor i64 %374, -1
  %377 = add i64 %371, %376
  %378 = select i1 %375, i64 %374, i64 %377
  br label %383

379:                                              ; preds = %359
  %380 = tail call noundef i64 @llvm.smax.i64(i64 %361, i64 0)
  %381 = add nsw i64 %331, -1
  %382 = tail call noundef i64 @llvm.smin.i64(i64 %380, i64 %381)
  br label %383

383:                                              ; preds = %379, %370, %366, %362
  %384 = phi i64 [ %382, %379 ], [ %378, %370 ], [ %369, %366 ], [ %365, %362 ]
  %385 = fsub double %279, %280
  %386 = fmul double %385, 2.560000e+02
  %387 = fadd double %386, 5.000000e-01
  %388 = tail call double @llvm.floor.f64(double %387)
  %389 = fptosi double %388 to i64
  %390 = sitofp i64 %389 to double
  %391 = fmul double %390, 3.906250e-03
  %392 = fsub double %335, %336
  %393 = fmul double %392, 2.560000e+02
  %394 = fadd double %393, 5.000000e-01
  %395 = tail call double @llvm.floor.f64(double %394)
  %396 = fptosi double %395 to i64
  %397 = sitofp i64 %396 to double
  %398 = fmul double %397, 3.906250e-03
  %399 = fsub double 1.000000e+00, %391
  %400 = fsub double 1.000000e+00, %398
  %401 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 3
  %402 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 2
  %403 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 14
  %404 = getelementptr i64, ptr addrspace(1) %3, i64 1
  %405 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 7
  br label %406

406:                                              ; preds = %612, %383
  %407 = phi <4 x double> [ zeroinitializer, %383 ], [ %615, %612 ]
  %408 = phi i32 [ 0, %383 ], [ %616, %612 ]
  %409 = and i32 %408, 1
  %410 = icmp eq i32 %409, 0
  %411 = select i1 %410, double %399, double %391
  %412 = and i32 %408, 2
  %413 = icmp eq i32 %412, 0
  %414 = select i1 %413, double %400, double %398
  %415 = fmul double %411, %414
  %416 = and i32 %408, 4
  %417 = icmp eq i32 %416, 0
  %418 = select i1 %417, double 1.000000e+00, double 0.000000e+00
  %419 = fmul double %418, %415
  %420 = insertelement <4 x double> poison, double %419, i64 0
  %421 = shufflevector <4 x double> %420, <4 x double> poison, <4 x i32> zeroinitializer
  %422 = select i1 %410, i64 %304, i64 %328
  %423 = select i1 %413, i64 %360, i64 %384
  %424 = or i64 %423, %422
  %425 = icmp sgt i64 %424, -1
  br i1 %425, label %426, label %612

426:                                              ; preds = %406
  %427 = load i64, ptr addrspace(1) %401, align 8
  %428 = load i64, ptr addrspace(1) %402, align 8
  switch i64 %428, label %430 [
    i64 0, label %431
    i64 20, label %431
    i64 1, label %429
    i64 21, label %429
    i64 6, label %429
  ]

429:                                              ; preds = %426, %426, %426
  br label %431

430:                                              ; preds = %426
  br label %431

431:                                              ; preds = %430, %429, %426, %426
  %432 = phi i32 [ 4, %430 ], [ 2, %429 ], [ 1, %426 ], [ 1, %426 ]
  %433 = load i64, ptr addrspace(1) %403, align 8
  %434 = icmp eq i64 %433, 0
  %435 = zext nneg i32 %432 to i64
  br i1 %434, label %443, label %436

436:                                              ; preds = %431
  %437 = load i64, ptr addrspace(1) %404, align 8
  %438 = mul nsw i64 %433, %423
  %439 = mul nsw i64 %427, %422
  %440 = mul nsw i64 %439, %435
  %441 = add nsw i64 %438, %440
  %442 = add i64 %441, %437
  br label %450

443:                                              ; preds = %431
  %444 = mul nsw i64 %423, %21
  %445 = add i64 %444, %422
  %446 = load i64, ptr addrspace(1) %404, align 8
  %447 = mul i64 %427, %445
  %448 = mul i64 %447, %435
  %449 = add i64 %446, %448
  br label %450

450:                                              ; preds = %443, %436
  %451 = phi i64 [ %442, %436 ], [ %449, %443 ]
  %452 = icmp eq i64 %428, 5
  %453 = sext i1 %452 to i32
  br label %454

454:                                              ; preds = %607, %450
  %455 = phi i32 [ 0, %450 ], [ %610, %607 ]
  %456 = phi <4 x double> [ zeroinitializer, %450 ], [ %609, %607 ]
  %457 = zext nneg i32 %455 to i64
  %458 = icmp sgt i64 %427, %457
  br i1 %458, label %459, label %604

459:                                              ; preds = %454
  %460 = mul nuw nsw i32 %455, %432
  %461 = zext nneg i32 %460 to i64
  %462 = add i64 %451, %461
  switch i64 %428, label %464 [
    i64 0, label %465
    i64 20, label %465
    i64 1, label %463
    i64 21, label %463
    i64 6, label %463
  ]

463:                                              ; preds = %459, %459, %459
  br label %465

464:                                              ; preds = %459
  br label %465

465:                                              ; preds = %464, %463, %459, %459
  %466 = phi i32 [ 4, %464 ], [ 2, %463 ], [ 1, %459 ], [ 1, %459 ]
  %467 = inttoptr i64 %462 to ptr addrspace(1)
  br label %469

468:                                              ; preds = %469
  switch i64 %428, label %572 [
    i64 6, label %481
    i64 7, label %568
    i64 0, label %573
    i64 20, label %573
    i64 1, label %571
    i64 21, label %571
  ]

469:                                              ; preds = %469, %465
  %470 = phi i32 [ 0, %465 ], [ %478, %469 ]
  %471 = phi i32 [ 0, %465 ], [ %479, %469 ]
  %472 = zext nneg i32 %471 to i64
  %473 = getelementptr inbounds i8, ptr addrspace(1) %467, i64 %472
  %474 = load i8, ptr addrspace(1) %473, align 1
  %475 = zext i8 %474 to i32
  %476 = shl nsw i32 %471, 3
  %477 = shl i32 %475, %476
  %478 = or i32 %477, %470
  %479 = add nuw nsw i32 %471, 1
  %480 = icmp eq i32 %479, %466
  br i1 %480, label %468, label %469, !llvm.loop !10016

481:                                              ; preds = %468
  %482 = lshr i32 %478, 10
  %483 = and i32 %482, 31
  %484 = and i32 %478, 1023
  switch i32 %483, label %491 [
    i32 0, label %485
    i32 31, label %488
  ]

485:                                              ; preds = %481
  %486 = uitofp i32 %484 to double
  %487 = fmul double %486, 0x3E70000000000000
  br label %563

488:                                              ; preds = %481
  %489 = icmp eq i32 %484, 0
  %490 = select i1 %489, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %563

491:                                              ; preds = %481
  %492 = or disjoint i32 %484, 1024
  %493 = uitofp i32 %492 to double
  %494 = add nsw i32 %483, -25
  %495 = icmp ugt i32 %483, 25
  br i1 %495, label %496, label %513

496:                                              ; preds = %491
  %497 = add nsw i32 %483, -26
  %498 = and i32 %494, 7
  %499 = icmp eq i32 %498, 0
  br i1 %499, label %508, label %500

500:                                              ; preds = %496, %500
  %501 = phi i32 [ %505, %500 ], [ %494, %496 ]
  %502 = phi double [ %504, %500 ], [ %493, %496 ]
  %503 = phi i32 [ %506, %500 ], [ 0, %496 ]
  %504 = fmul double %502, 2.000000e+00
  %505 = add nsw i32 %501, -1
  %506 = add i32 %503, 1
  %507 = icmp eq i32 %506, %498
  br i1 %507, label %508, label %500, !llvm.loop !10084

508:                                              ; preds = %500, %496
  %509 = phi i32 [ %494, %496 ], [ %505, %500 ]
  %510 = phi double [ %493, %496 ], [ %504, %500 ]
  %511 = phi double [ undef, %496 ], [ %504, %500 ]
  %512 = icmp ult i32 %497, 7
  br i1 %512, label %513, label %536

513:                                              ; preds = %508, %536, %491
  %514 = phi double [ %493, %491 ], [ %511, %508 ], [ %546, %536 ]
  %515 = phi i32 [ %494, %491 ], [ 0, %536 ], [ 0, %508 ]
  %516 = icmp slt i32 %515, 0
  br i1 %516, label %517, label %563

517:                                              ; preds = %513
  %518 = tail call i32 @llvm.umin.i32(i32 %483, i32 25)
  %519 = sub nsw i32 1, %518
  %520 = and i32 %519, 7
  %521 = icmp eq i32 %520, 0
  br i1 %521, label %530, label %522

522:                                              ; preds = %517, %522
  %523 = phi i32 [ %527, %522 ], [ %515, %517 ]
  %524 = phi double [ %526, %522 ], [ %514, %517 ]
  %525 = phi i32 [ %528, %522 ], [ 0, %517 ]
  %526 = fmul double %524, 5.000000e-01
  %527 = add nsw i32 %523, 1
  %528 = add i32 %525, 1
  %529 = icmp eq i32 %528, %520
  br i1 %529, label %530, label %522, !llvm.loop !10085

530:                                              ; preds = %522, %517
  %531 = phi double [ undef, %517 ], [ %526, %522 ]
  %532 = phi i32 [ %515, %517 ], [ %527, %522 ]
  %533 = phi double [ %514, %517 ], [ %526, %522 ]
  %534 = add nsw i32 %518, -18
  %535 = icmp ult i32 %534, 7
  br i1 %535, label %563, label %550

536:                                              ; preds = %508, %536
  %537 = phi i32 [ %547, %536 ], [ %509, %508 ]
  %538 = phi double [ %546, %536 ], [ %510, %508 ]
  %539 = fmul double %538, 2.000000e+00
  %540 = fmul double %539, 2.000000e+00
  %541 = fmul double %540, 2.000000e+00
  %542 = fmul double %541, 2.000000e+00
  %543 = fmul double %542, 2.000000e+00
  %544 = fmul double %543, 2.000000e+00
  %545 = fmul double %544, 2.000000e+00
  %546 = fmul double %545, 2.000000e+00
  %547 = add nsw i32 %537, -8
  %548 = add i32 %537, -9
  %549 = icmp ult i32 %548, -2
  br i1 %549, label %536, label %513, !llvm.loop !10021

550:                                              ; preds = %530, %550
  %551 = phi i32 [ %561, %550 ], [ %532, %530 ]
  %552 = phi double [ %560, %550 ], [ %533, %530 ]
  %553 = fmul double %552, 5.000000e-01
  %554 = fmul double %553, 5.000000e-01
  %555 = fmul double %554, 5.000000e-01
  %556 = fmul double %555, 5.000000e-01
  %557 = fmul double %556, 5.000000e-01
  %558 = fmul double %557, 5.000000e-01
  %559 = fmul double %558, 5.000000e-01
  %560 = fmul double %559, 5.000000e-01
  %561 = add nsw i32 %551, 8
  %562 = icmp eq i32 %561, 0
  br i1 %562, label %563, label %550, !llvm.loop !10022

563:                                              ; preds = %530, %550, %513, %488, %485
  %564 = phi double [ %487, %485 ], [ %490, %488 ], [ %514, %513 ], [ %531, %530 ], [ %560, %550 ]
  %565 = icmp ult i32 %478, 32768
  %566 = fneg double %564
  %567 = select i1 %565, double %564, double %566
  br label %607

568:                                              ; preds = %468
  %569 = bitcast i32 %478 to float
  %570 = fpext float %569 to double
  br label %607

571:                                              ; preds = %468, %468
  br label %573

572:                                              ; preds = %468
  br label %573

573:                                              ; preds = %572, %571, %468, %468
  %574 = phi i32 [ 32, %572 ], [ 16, %571 ], [ 8, %468 ], [ 8, %468 ]
  %575 = zext i32 %478 to i64
  switch i64 %428, label %585 [
    i64 21, label %576
    i64 20, label %576
    i64 3, label %576
    i64 5, label %576
  ]

576:                                              ; preds = %573, %573, %573, %573
  %577 = add nsw i32 %574, -1
  %578 = shl nuw i32 1, %577
  %579 = and i32 %578, %478
  %580 = icmp eq i32 %579, 0
  %581 = zext nneg i32 %574 to i64
  %582 = shl nsw i64 -1, %581
  %583 = select i1 %580, i64 0, i64 %582
  %584 = add nsw i64 %583, %575
  br label %585

585:                                              ; preds = %576, %573
  %586 = phi i64 [ %584, %576 ], [ %575, %573 ]
  %587 = load i64, ptr addrspace(1) %405, align 8
  %588 = icmp eq i64 %587, 0
  br i1 %588, label %589, label %591

589:                                              ; preds = %585
  %590 = sitofp i64 %586 to double
  br label %607

591:                                              ; preds = %585
  switch i64 %428, label %592 [
    i64 21, label %593
    i64 20, label %593
    i64 3, label %593
  ]

592:                                              ; preds = %591
  br label %593

593:                                              ; preds = %592, %591, %591, %591
  %594 = phi i32 [ -1, %591 ], [ %453, %592 ], [ -1, %591 ], [ -1, %591 ]
  %595 = add nsw i32 %594, %574
  %596 = zext nneg i32 %595 to i64
  %597 = shl nsw i64 -1, %596
  %598 = xor i64 %597, -1
  %599 = sitofp i64 %586 to double
  %600 = sitofp i64 %598 to double
  %601 = fdiv double %599, %600
  %602 = fcmp oge double %601, -1.000000e+00
  %603 = select i1 %602, double %601, double -1.000000e+00
  br label %607

604:                                              ; preds = %454
  %605 = icmp eq i32 %455, 3
  %606 = select i1 %605, double 1.000000e+00, double 0.000000e+00
  br label %607

607:                                              ; preds = %604, %593, %589, %568, %563
  %608 = phi double [ %606, %604 ], [ %567, %563 ], [ %570, %568 ], [ %603, %593 ], [ %590, %589 ]
  %609 = insertelement <4 x double> %456, double %608, i32 %455
  %610 = add nuw nsw i32 %455, 1
  %611 = icmp eq i32 %610, 4
  br i1 %611, label %612, label %454, !llvm.loop !10029

612:                                              ; preds = %607, %406
  %613 = phi <4 x double> [ zeroinitializer, %406 ], [ %609, %607 ]
  %614 = fmul <4 x double> %421, %613
  %615 = fadd <4 x double> %407, %614
  %616 = add nuw nsw i32 %408, 1
  %617 = icmp eq i32 %616, 4
  br i1 %617, label %618, label %406, !llvm.loop !10032

618:                                              ; preds = %270, %612, %82
  %619 = phi <4 x double> [ zeroinitializer, %82 ], [ %615, %612 ], [ %272, %270 ]
  %620 = extractelement <4 x double> %619, i64 0
  %621 = fcmp uno double %620, 0.000000e+00
  %622 = fptosi double %620 to i64
  %623 = trunc i64 %622 to i32
  %624 = select i1 %621, i32 0, i32 %623
  %625 = insertelement <4 x i32> poison, i32 %624, i64 0
  %626 = extractelement <4 x double> %619, i64 1
  %627 = fcmp uno double %626, 0.000000e+00
  %628 = fptosi double %626 to i64
  %629 = trunc i64 %628 to i32
  %630 = select i1 %627, i32 0, i32 %629
  %631 = insertelement <4 x i32> %625, i32 %630, i64 1
  %632 = extractelement <4 x double> %619, i64 2
  %633 = fcmp uno double %632, 0.000000e+00
  %634 = fptosi double %632 to i64
  %635 = trunc i64 %634 to i32
  %636 = select i1 %633, i32 0, i32 %635
  %637 = insertelement <4 x i32> %631, i32 %636, i64 2
  %638 = extractelement <4 x double> %619, i64 3
  %639 = fcmp uno double %638, 0.000000e+00
  %640 = fptosi double %638 to i64
  %641 = trunc i64 %640 to i32
  %642 = select i1 %639, i32 0, i32 %641
  %643 = insertelement <4 x i32> %637, i32 %642, i64 3
  ret <4 x i32> %643
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define dso_local <4 x i32> @__zluda_ptx_impl_texobj_2d_v4_s32_s32(i64 noundef %0, <2 x i32> noundef %1) local_unnamed_addr #3 {
  %3 = extractelement <2 x i32> %1, i64 0
  %4 = inttoptr i64 %0 to ptr addrspace(1)
  %5 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 5
  %6 = load i64, ptr addrspace(1) %5, align 8
  %7 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 6
  %8 = load i64, ptr addrspace(1) %7, align 8
  %9 = getelementptr i64, ptr addrspace(1) %4, i64 9
  %10 = and i64 %8, 4294967295
  %11 = icmp eq i64 %10, 1
  %12 = extractelement <2 x i32> %1, i64 1
  %13 = trunc i64 %6 to i32
  %14 = sitofp i32 %3 to double
  %15 = sitofp i32 %12 to double
  %16 = load i64, ptr addrspace(1) %9, align 8
  %17 = tail call noundef i64 @llvm.smax.i64(i64 %16, i64 1)
  %18 = getelementptr i64, ptr addrspace(1) %4, i64 1
  %19 = getelementptr i64, ptr addrspace(1) %4, i64 10
  %20 = load i64, ptr addrspace(1) %19, align 8
  %21 = tail call noundef i64 @llvm.smax.i64(i64 %20, i64 1)
  %22 = getelementptr i64, ptr addrspace(1) %4, i64 2
  %23 = sitofp i64 %17 to double
  %24 = select i1 %11, double %23, double 1.000000e+00
  %25 = fmul double %24, %14
  %26 = tail call double @llvm.floor.f64(double %25)
  %27 = fptosi double %26 to i64
  switch i32 %13, label %45 [
    i32 0, label %28
    i32 2, label %32
    i32 3, label %36
  ]

28:                                               ; preds = %2
  %29 = srem i64 %27, %17
  %30 = add nsw i64 %29, %17
  %31 = srem i64 %30, %17
  br label %49

32:                                               ; preds = %2
  %33 = icmp sgt i64 %17, %27
  %34 = tail call i64 @llvm.smax.i64(i64 %27, i64 -1)
  %35 = select i1 %33, i64 %34, i64 -1
  br label %49

36:                                               ; preds = %2
  %37 = shl nuw nsw i64 %17, 1
  %38 = srem i64 %27, %37
  %39 = add nsw i64 %38, %37
  %40 = srem i64 %39, %37
  %41 = icmp slt i64 %40, %17
  %42 = xor i64 %40, -1
  %43 = add i64 %37, %42
  %44 = select i1 %41, i64 %40, i64 %43
  br label %49

45:                                               ; preds = %2
  %46 = tail call noundef i64 @llvm.smax.i64(i64 %27, i64 0)
  %47 = add nsw i64 %17, -1
  %48 = tail call noundef i64 @llvm.smin.i64(i64 %46, i64 %47)
  br label %49

49:                                               ; preds = %45, %36, %32, %28
  %50 = phi i64 [ %48, %45 ], [ %44, %36 ], [ %35, %32 ], [ %31, %28 ]
  %51 = sitofp i64 %21 to double
  %52 = select i1 %11, double %51, double 1.000000e+00
  %53 = fmul double %52, %15
  %54 = tail call double @llvm.floor.f64(double %53)
  %55 = fptosi double %54 to i64
  switch i32 %13, label %73 [
    i32 0, label %56
    i32 2, label %60
    i32 3, label %64
  ]

56:                                               ; preds = %49
  %57 = srem i64 %55, %21
  %58 = add nsw i64 %57, %21
  %59 = srem i64 %58, %21
  br label %77

60:                                               ; preds = %49
  %61 = icmp sgt i64 %21, %55
  %62 = tail call i64 @llvm.smax.i64(i64 %55, i64 -1)
  %63 = select i1 %61, i64 %62, i64 -1
  br label %77

64:                                               ; preds = %49
  %65 = shl nuw nsw i64 %21, 1
  %66 = srem i64 %55, %65
  %67 = add nsw i64 %66, %65
  %68 = srem i64 %67, %65
  %69 = icmp slt i64 %68, %21
  %70 = xor i64 %68, -1
  %71 = add i64 %65, %70
  %72 = select i1 %69, i64 %68, i64 %71
  br label %77

73:                                               ; preds = %49
  %74 = tail call noundef i64 @llvm.smax.i64(i64 %55, i64 0)
  %75 = add nsw i64 %21, -1
  %76 = tail call noundef i64 @llvm.smin.i64(i64 %74, i64 %75)
  br label %77

77:                                               ; preds = %73, %64, %60, %56
  %78 = phi i64 [ %76, %73 ], [ %72, %64 ], [ %63, %60 ], [ %59, %56 ]
  %79 = or i64 %78, %50
  %80 = icmp sgt i64 %79, -1
  br i1 %80, label %81, label %270

81:                                               ; preds = %77
  %82 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 3
  %83 = load i64, ptr addrspace(1) %82, align 8
  %84 = load i64, ptr addrspace(1) %22, align 8
  switch i64 %84, label %86 [
    i64 0, label %87
    i64 20, label %87
    i64 1, label %85
    i64 21, label %85
    i64 6, label %85
  ]

85:                                               ; preds = %81, %81, %81
  br label %87

86:                                               ; preds = %81
  br label %87

87:                                               ; preds = %86, %85, %81, %81
  %88 = phi i32 [ 4, %86 ], [ 2, %85 ], [ 1, %81 ], [ 1, %81 ]
  %89 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 14
  %90 = load i64, ptr addrspace(1) %89, align 8
  %91 = icmp eq i64 %90, 0
  %92 = zext nneg i32 %88 to i64
  br i1 %91, label %100, label %93

93:                                               ; preds = %87
  %94 = load i64, ptr addrspace(1) %18, align 8
  %95 = mul nsw i64 %90, %78
  %96 = mul nsw i64 %83, %50
  %97 = mul nsw i64 %96, %92
  %98 = add nsw i64 %95, %97
  %99 = add i64 %98, %94
  br label %107

100:                                              ; preds = %87
  %101 = mul nsw i64 %78, %17
  %102 = add i64 %101, %50
  %103 = load i64, ptr addrspace(1) %18, align 8
  %104 = mul i64 %83, %102
  %105 = mul i64 %104, %92
  %106 = add i64 %103, %105
  br label %107

107:                                              ; preds = %100, %93
  %108 = phi i64 [ %99, %93 ], [ %106, %100 ]
  %109 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 7
  %110 = icmp eq i64 %84, 5
  %111 = sext i1 %110 to i32
  br label %112

112:                                              ; preds = %265, %107
  %113 = phi i32 [ 0, %107 ], [ %268, %265 ]
  %114 = phi <4 x double> [ zeroinitializer, %107 ], [ %267, %265 ]
  %115 = zext nneg i32 %113 to i64
  %116 = icmp sgt i64 %83, %115
  br i1 %116, label %117, label %262

117:                                              ; preds = %112
  %118 = mul nuw nsw i32 %113, %88
  %119 = zext nneg i32 %118 to i64
  %120 = add i64 %108, %119
  switch i64 %84, label %122 [
    i64 0, label %123
    i64 20, label %123
    i64 1, label %121
    i64 21, label %121
    i64 6, label %121
  ]

121:                                              ; preds = %117, %117, %117
  br label %123

122:                                              ; preds = %117
  br label %123

123:                                              ; preds = %122, %121, %117, %117
  %124 = phi i32 [ 4, %122 ], [ 2, %121 ], [ 1, %117 ], [ 1, %117 ]
  %125 = inttoptr i64 %120 to ptr addrspace(1)
  br label %127

126:                                              ; preds = %127
  switch i64 %84, label %230 [
    i64 6, label %139
    i64 7, label %226
    i64 0, label %231
    i64 20, label %231
    i64 1, label %229
    i64 21, label %229
  ]

127:                                              ; preds = %127, %123
  %128 = phi i32 [ 0, %123 ], [ %136, %127 ]
  %129 = phi i32 [ 0, %123 ], [ %137, %127 ]
  %130 = zext nneg i32 %129 to i64
  %131 = getelementptr inbounds i8, ptr addrspace(1) %125, i64 %130
  %132 = load i8, ptr addrspace(1) %131, align 1
  %133 = zext i8 %132 to i32
  %134 = shl nsw i32 %129, 3
  %135 = shl i32 %133, %134
  %136 = or i32 %135, %128
  %137 = add nuw nsw i32 %129, 1
  %138 = icmp eq i32 %137, %124
  br i1 %138, label %126, label %127, !llvm.loop !10016

139:                                              ; preds = %126
  %140 = lshr i32 %136, 10
  %141 = and i32 %140, 31
  %142 = and i32 %136, 1023
  switch i32 %141, label %149 [
    i32 0, label %143
    i32 31, label %146
  ]

143:                                              ; preds = %139
  %144 = uitofp i32 %142 to double
  %145 = fmul double %144, 0x3E70000000000000
  br label %221

146:                                              ; preds = %139
  %147 = icmp eq i32 %142, 0
  %148 = select i1 %147, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %221

149:                                              ; preds = %139
  %150 = or disjoint i32 %142, 1024
  %151 = uitofp i32 %150 to double
  %152 = add nsw i32 %141, -25
  %153 = icmp ugt i32 %141, 25
  br i1 %153, label %154, label %171

154:                                              ; preds = %149
  %155 = add nsw i32 %141, -26
  %156 = and i32 %152, 7
  %157 = icmp eq i32 %156, 0
  br i1 %157, label %166, label %158

158:                                              ; preds = %154, %158
  %159 = phi i32 [ %163, %158 ], [ %152, %154 ]
  %160 = phi double [ %162, %158 ], [ %151, %154 ]
  %161 = phi i32 [ %164, %158 ], [ 0, %154 ]
  %162 = fmul double %160, 2.000000e+00
  %163 = add nsw i32 %159, -1
  %164 = add i32 %161, 1
  %165 = icmp eq i32 %164, %156
  br i1 %165, label %166, label %158, !llvm.loop !10086

166:                                              ; preds = %158, %154
  %167 = phi i32 [ %152, %154 ], [ %163, %158 ]
  %168 = phi double [ %151, %154 ], [ %162, %158 ]
  %169 = phi double [ undef, %154 ], [ %162, %158 ]
  %170 = icmp ult i32 %155, 7
  br i1 %170, label %171, label %194

171:                                              ; preds = %166, %194, %149
  %172 = phi double [ %151, %149 ], [ %169, %166 ], [ %204, %194 ]
  %173 = phi i32 [ %152, %149 ], [ 0, %194 ], [ 0, %166 ]
  %174 = icmp slt i32 %173, 0
  br i1 %174, label %175, label %221

175:                                              ; preds = %171
  %176 = tail call i32 @llvm.umin.i32(i32 %141, i32 25)
  %177 = sub nsw i32 1, %176
  %178 = and i32 %177, 7
  %179 = icmp eq i32 %178, 0
  br i1 %179, label %188, label %180

180:                                              ; preds = %175, %180
  %181 = phi i32 [ %185, %180 ], [ %173, %175 ]
  %182 = phi double [ %184, %180 ], [ %172, %175 ]
  %183 = phi i32 [ %186, %180 ], [ 0, %175 ]
  %184 = fmul double %182, 5.000000e-01
  %185 = add nsw i32 %181, 1
  %186 = add i32 %183, 1
  %187 = icmp eq i32 %186, %178
  br i1 %187, label %188, label %180, !llvm.loop !10087

188:                                              ; preds = %180, %175
  %189 = phi double [ undef, %175 ], [ %184, %180 ]
  %190 = phi i32 [ %173, %175 ], [ %185, %180 ]
  %191 = phi double [ %172, %175 ], [ %184, %180 ]
  %192 = add nsw i32 %176, -18
  %193 = icmp ult i32 %192, 7
  br i1 %193, label %221, label %208

194:                                              ; preds = %166, %194
  %195 = phi i32 [ %205, %194 ], [ %167, %166 ]
  %196 = phi double [ %204, %194 ], [ %168, %166 ]
  %197 = fmul double %196, 2.000000e+00
  %198 = fmul double %197, 2.000000e+00
  %199 = fmul double %198, 2.000000e+00
  %200 = fmul double %199, 2.000000e+00
  %201 = fmul double %200, 2.000000e+00
  %202 = fmul double %201, 2.000000e+00
  %203 = fmul double %202, 2.000000e+00
  %204 = fmul double %203, 2.000000e+00
  %205 = add nsw i32 %195, -8
  %206 = add i32 %195, -9
  %207 = icmp ult i32 %206, -2
  br i1 %207, label %194, label %171, !llvm.loop !10021

208:                                              ; preds = %188, %208
  %209 = phi i32 [ %219, %208 ], [ %190, %188 ]
  %210 = phi double [ %218, %208 ], [ %191, %188 ]
  %211 = fmul double %210, 5.000000e-01
  %212 = fmul double %211, 5.000000e-01
  %213 = fmul double %212, 5.000000e-01
  %214 = fmul double %213, 5.000000e-01
  %215 = fmul double %214, 5.000000e-01
  %216 = fmul double %215, 5.000000e-01
  %217 = fmul double %216, 5.000000e-01
  %218 = fmul double %217, 5.000000e-01
  %219 = add nsw i32 %209, 8
  %220 = icmp eq i32 %219, 0
  br i1 %220, label %221, label %208, !llvm.loop !10022

221:                                              ; preds = %188, %208, %171, %146, %143
  %222 = phi double [ %145, %143 ], [ %148, %146 ], [ %172, %171 ], [ %189, %188 ], [ %218, %208 ]
  %223 = icmp ult i32 %136, 32768
  %224 = fneg double %222
  %225 = select i1 %223, double %222, double %224
  br label %265

226:                                              ; preds = %126
  %227 = bitcast i32 %136 to float
  %228 = fpext float %227 to double
  br label %265

229:                                              ; preds = %126, %126
  br label %231

230:                                              ; preds = %126
  br label %231

231:                                              ; preds = %230, %229, %126, %126
  %232 = phi i32 [ 32, %230 ], [ 16, %229 ], [ 8, %126 ], [ 8, %126 ]
  %233 = zext i32 %136 to i64
  switch i64 %84, label %243 [
    i64 21, label %234
    i64 20, label %234
    i64 3, label %234
    i64 5, label %234
  ]

234:                                              ; preds = %231, %231, %231, %231
  %235 = add nsw i32 %232, -1
  %236 = shl nuw i32 1, %235
  %237 = and i32 %236, %136
  %238 = icmp eq i32 %237, 0
  %239 = zext nneg i32 %232 to i64
  %240 = shl nsw i64 -1, %239
  %241 = select i1 %238, i64 0, i64 %240
  %242 = add nsw i64 %241, %233
  br label %243

243:                                              ; preds = %234, %231
  %244 = phi i64 [ %242, %234 ], [ %233, %231 ]
  %245 = load i64, ptr addrspace(1) %109, align 8
  %246 = icmp eq i64 %245, 0
  br i1 %246, label %247, label %249

247:                                              ; preds = %243
  %248 = sitofp i64 %244 to double
  br label %265

249:                                              ; preds = %243
  switch i64 %84, label %250 [
    i64 21, label %251
    i64 20, label %251
    i64 3, label %251
  ]

250:                                              ; preds = %249
  br label %251

251:                                              ; preds = %250, %249, %249, %249
  %252 = phi i32 [ -1, %249 ], [ %111, %250 ], [ -1, %249 ], [ -1, %249 ]
  %253 = add nsw i32 %252, %232
  %254 = zext nneg i32 %253 to i64
  %255 = shl nsw i64 -1, %254
  %256 = xor i64 %255, -1
  %257 = sitofp i64 %244 to double
  %258 = sitofp i64 %256 to double
  %259 = fdiv double %257, %258
  %260 = fcmp oge double %259, -1.000000e+00
  %261 = select i1 %260, double %259, double -1.000000e+00
  br label %265

262:                                              ; preds = %112
  %263 = icmp eq i32 %113, 3
  %264 = select i1 %263, double 1.000000e+00, double 0.000000e+00
  br label %265

265:                                              ; preds = %262, %251, %247, %226, %221
  %266 = phi double [ %264, %262 ], [ %225, %221 ], [ %228, %226 ], [ %261, %251 ], [ %248, %247 ]
  %267 = insertelement <4 x double> %114, double %266, i32 %113
  %268 = add nuw nsw i32 %113, 1
  %269 = icmp eq i32 %268, 4
  br i1 %269, label %270, label %112, !llvm.loop !10029

270:                                              ; preds = %265, %77
  %271 = phi <4 x double> [ zeroinitializer, %77 ], [ %267, %265 ]
  %272 = extractelement <4 x double> %271, i64 0
  %273 = fcmp uno double %272, 0.000000e+00
  %274 = fptosi double %272 to i64
  %275 = trunc i64 %274 to i32
  %276 = select i1 %273, i32 0, i32 %275
  %277 = insertelement <4 x i32> poison, i32 %276, i64 0
  %278 = extractelement <4 x double> %271, i64 1
  %279 = fcmp uno double %278, 0.000000e+00
  %280 = fptosi double %278 to i64
  %281 = trunc i64 %280 to i32
  %282 = select i1 %279, i32 0, i32 %281
  %283 = insertelement <4 x i32> %277, i32 %282, i64 1
  %284 = extractelement <4 x double> %271, i64 2
  %285 = fcmp uno double %284, 0.000000e+00
  %286 = fptosi double %284 to i64
  %287 = trunc i64 %286 to i32
  %288 = select i1 %285, i32 0, i32 %287
  %289 = insertelement <4 x i32> %283, i32 %288, i64 2
  %290 = extractelement <4 x double> %271, i64 3
  %291 = fcmp uno double %290, 0.000000e+00
  %292 = fptosi double %290 to i64
  %293 = trunc i64 %292 to i32
  %294 = select i1 %291, i32 0, i32 %293
  %295 = insertelement <4 x i32> %289, i32 %294, i64 3
  ret <4 x i32> %295
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define dso_local <4 x i32> @__zluda_ptx_impl_texobj_3d_v4_s32_f32(i64 noundef %0, <4 x float> noundef %1) local_unnamed_addr #3 {
  %3 = inttoptr i64 %0 to ptr addrspace(1)
  %4 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 4
  %5 = load i64, ptr addrspace(1) %4, align 8
  %6 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 5
  %7 = load i64, ptr addrspace(1) %6, align 8
  %8 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 6
  %9 = load i64, ptr addrspace(1) %8, align 8
  %10 = getelementptr i64, ptr addrspace(1) %3, i64 9
  %11 = and i64 %9, 4294967295
  %12 = icmp eq i64 %11, 1
  %13 = trunc i64 %7 to i32
  %14 = extractelement <4 x float> %1, i64 0
  %15 = fpext float %14 to double
  %16 = extractelement <4 x float> %1, i64 1
  %17 = fpext float %16 to double
  %18 = extractelement <4 x float> %1, i64 2
  %19 = fpext float %18 to double
  %20 = and i64 %5, 4294967295
  %21 = icmp eq i64 %20, 1
  %22 = load i64, ptr addrspace(1) %10, align 8
  %23 = tail call noundef i64 @llvm.smax.i64(i64 %22, i64 1)
  br i1 %21, label %313, label %24

24:                                               ; preds = %2
  %25 = getelementptr i64, ptr addrspace(1) %3, i64 1
  %26 = getelementptr i64, ptr addrspace(1) %3, i64 10
  %27 = load i64, ptr addrspace(1) %26, align 8
  %28 = tail call noundef i64 @llvm.smax.i64(i64 %27, i64 1)
  %29 = getelementptr i64, ptr addrspace(1) %3, i64 2
  %30 = getelementptr i64, ptr addrspace(1) %3, i64 11
  %31 = load i64, ptr addrspace(1) %30, align 8
  %32 = tail call noundef i64 @llvm.smax.i64(i64 %31, i64 1)
  %33 = sitofp i64 %23 to double
  %34 = select i1 %12, double %33, double 1.000000e+00
  %35 = fmul double %34, %15
  %36 = tail call double @llvm.floor.f64(double %35)
  %37 = fptosi double %36 to i64
  switch i32 %13, label %55 [
    i32 0, label %38
    i32 2, label %42
    i32 3, label %46
  ]

38:                                               ; preds = %24
  %39 = srem i64 %37, %23
  %40 = add nsw i64 %39, %23
  %41 = srem i64 %40, %23
  br label %59

42:                                               ; preds = %24
  %43 = icmp sgt i64 %23, %37
  %44 = tail call i64 @llvm.smax.i64(i64 %37, i64 -1)
  %45 = select i1 %43, i64 %44, i64 -1
  br label %59

46:                                               ; preds = %24
  %47 = shl nuw nsw i64 %23, 1
  %48 = srem i64 %37, %47
  %49 = add nsw i64 %48, %47
  %50 = srem i64 %49, %47
  %51 = icmp slt i64 %50, %23
  %52 = xor i64 %50, -1
  %53 = add i64 %47, %52
  %54 = select i1 %51, i64 %50, i64 %53
  br label %59

55:                                               ; preds = %24
  %56 = tail call noundef i64 @llvm.smax.i64(i64 %37, i64 0)
  %57 = add nsw i64 %23, -1
  %58 = tail call noundef i64 @llvm.smin.i64(i64 %56, i64 %57)
  br label %59

59:                                               ; preds = %55, %46, %42, %38
  %60 = phi i64 [ %58, %55 ], [ %54, %46 ], [ %45, %42 ], [ %41, %38 ]
  %61 = sitofp i64 %28 to double
  %62 = select i1 %12, double %61, double 1.000000e+00
  %63 = fmul double %62, %17
  %64 = tail call double @llvm.floor.f64(double %63)
  %65 = fptosi double %64 to i64
  switch i32 %13, label %83 [
    i32 0, label %66
    i32 2, label %70
    i32 3, label %74
  ]

66:                                               ; preds = %59
  %67 = srem i64 %65, %28
  %68 = add nsw i64 %67, %28
  %69 = srem i64 %68, %28
  br label %87

70:                                               ; preds = %59
  %71 = icmp sgt i64 %28, %65
  %72 = tail call i64 @llvm.smax.i64(i64 %65, i64 -1)
  %73 = select i1 %71, i64 %72, i64 -1
  br label %87

74:                                               ; preds = %59
  %75 = shl nuw nsw i64 %28, 1
  %76 = srem i64 %65, %75
  %77 = add nsw i64 %76, %75
  %78 = srem i64 %77, %75
  %79 = icmp slt i64 %78, %28
  %80 = xor i64 %78, -1
  %81 = add i64 %75, %80
  %82 = select i1 %79, i64 %78, i64 %81
  br label %87

83:                                               ; preds = %59
  %84 = tail call noundef i64 @llvm.smax.i64(i64 %65, i64 0)
  %85 = add nsw i64 %28, -1
  %86 = tail call noundef i64 @llvm.smin.i64(i64 %84, i64 %85)
  br label %87

87:                                               ; preds = %83, %74, %70, %66
  %88 = phi i64 [ %86, %83 ], [ %82, %74 ], [ %73, %70 ], [ %69, %66 ]
  %89 = sitofp i64 %32 to double
  %90 = select i1 %12, double %89, double 1.000000e+00
  %91 = fmul double %90, %19
  %92 = tail call double @llvm.floor.f64(double %91)
  %93 = fptosi double %92 to i64
  switch i32 %13, label %111 [
    i32 0, label %94
    i32 2, label %98
    i32 3, label %102
  ]

94:                                               ; preds = %87
  %95 = srem i64 %93, %32
  %96 = add nsw i64 %95, %32
  %97 = srem i64 %96, %32
  br label %115

98:                                               ; preds = %87
  %99 = icmp sgt i64 %32, %93
  %100 = tail call i64 @llvm.smax.i64(i64 %93, i64 -1)
  %101 = select i1 %99, i64 %100, i64 -1
  br label %115

102:                                              ; preds = %87
  %103 = shl nuw nsw i64 %32, 1
  %104 = srem i64 %93, %103
  %105 = add nsw i64 %104, %103
  %106 = srem i64 %105, %103
  %107 = icmp slt i64 %106, %32
  %108 = xor i64 %106, -1
  %109 = add i64 %103, %108
  %110 = select i1 %107, i64 %106, i64 %109
  br label %115

111:                                              ; preds = %87
  %112 = tail call noundef i64 @llvm.smax.i64(i64 %93, i64 0)
  %113 = add nsw i64 %32, -1
  %114 = tail call noundef i64 @llvm.smin.i64(i64 %112, i64 %113)
  br label %115

115:                                              ; preds = %111, %102, %98, %94
  %116 = phi i64 [ %114, %111 ], [ %110, %102 ], [ %101, %98 ], [ %97, %94 ]
  %117 = or i64 %88, %60
  %118 = or i64 %117, %116
  %119 = icmp sgt i64 %118, -1
  br i1 %119, label %120, label %726

120:                                              ; preds = %115
  %121 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 3
  %122 = load i64, ptr addrspace(1) %121, align 8
  %123 = load i64, ptr addrspace(1) %29, align 8
  switch i64 %123, label %125 [
    i64 0, label %126
    i64 20, label %126
    i64 1, label %124
    i64 21, label %124
    i64 6, label %124
  ]

124:                                              ; preds = %120, %120, %120
  br label %126

125:                                              ; preds = %120
  br label %126

126:                                              ; preds = %125, %124, %120, %120
  %127 = phi i32 [ 4, %125 ], [ 2, %124 ], [ 1, %120 ], [ 1, %120 ]
  %128 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 14
  %129 = load i64, ptr addrspace(1) %128, align 8
  %130 = icmp eq i64 %129, 0
  %131 = zext nneg i32 %127 to i64
  br i1 %130, label %141, label %132

132:                                              ; preds = %126
  %133 = load i64, ptr addrspace(1) %25, align 8
  %134 = mul nsw i64 %116, %28
  %135 = add nsw i64 %134, %88
  %136 = mul nsw i64 %129, %135
  %137 = mul nsw i64 %122, %60
  %138 = mul nsw i64 %137, %131
  %139 = add nsw i64 %136, %138
  %140 = add i64 %139, %133
  br label %150

141:                                              ; preds = %126
  %142 = mul nsw i64 %116, %28
  %143 = add nsw i64 %142, %88
  %144 = mul nsw i64 %143, %23
  %145 = add i64 %144, %60
  %146 = load i64, ptr addrspace(1) %25, align 8
  %147 = mul i64 %122, %145
  %148 = mul i64 %147, %131
  %149 = add i64 %146, %148
  br label %150

150:                                              ; preds = %141, %132
  %151 = phi i64 [ %140, %132 ], [ %149, %141 ]
  %152 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 7
  %153 = icmp eq i64 %123, 5
  %154 = sext i1 %153 to i32
  br label %155

155:                                              ; preds = %308, %150
  %156 = phi i32 [ 0, %150 ], [ %311, %308 ]
  %157 = phi <4 x double> [ zeroinitializer, %150 ], [ %310, %308 ]
  %158 = zext nneg i32 %156 to i64
  %159 = icmp sgt i64 %122, %158
  br i1 %159, label %160, label %305

160:                                              ; preds = %155
  %161 = mul nuw nsw i32 %156, %127
  %162 = zext nneg i32 %161 to i64
  %163 = add i64 %151, %162
  switch i64 %123, label %165 [
    i64 0, label %166
    i64 20, label %166
    i64 1, label %164
    i64 21, label %164
    i64 6, label %164
  ]

164:                                              ; preds = %160, %160, %160
  br label %166

165:                                              ; preds = %160
  br label %166

166:                                              ; preds = %165, %164, %160, %160
  %167 = phi i32 [ 4, %165 ], [ 2, %164 ], [ 1, %160 ], [ 1, %160 ]
  %168 = inttoptr i64 %163 to ptr addrspace(1)
  br label %170

169:                                              ; preds = %170
  switch i64 %123, label %273 [
    i64 6, label %182
    i64 7, label %269
    i64 0, label %274
    i64 20, label %274
    i64 1, label %272
    i64 21, label %272
  ]

170:                                              ; preds = %170, %166
  %171 = phi i32 [ 0, %166 ], [ %179, %170 ]
  %172 = phi i32 [ 0, %166 ], [ %180, %170 ]
  %173 = zext nneg i32 %172 to i64
  %174 = getelementptr inbounds i8, ptr addrspace(1) %168, i64 %173
  %175 = load i8, ptr addrspace(1) %174, align 1
  %176 = zext i8 %175 to i32
  %177 = shl nsw i32 %172, 3
  %178 = shl i32 %176, %177
  %179 = or i32 %178, %171
  %180 = add nuw nsw i32 %172, 1
  %181 = icmp eq i32 %180, %167
  br i1 %181, label %169, label %170, !llvm.loop !10016

182:                                              ; preds = %169
  %183 = lshr i32 %179, 10
  %184 = and i32 %183, 31
  %185 = and i32 %179, 1023
  switch i32 %184, label %192 [
    i32 0, label %186
    i32 31, label %189
  ]

186:                                              ; preds = %182
  %187 = uitofp i32 %185 to double
  %188 = fmul double %187, 0x3E70000000000000
  br label %264

189:                                              ; preds = %182
  %190 = icmp eq i32 %185, 0
  %191 = select i1 %190, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %264

192:                                              ; preds = %182
  %193 = or disjoint i32 %185, 1024
  %194 = uitofp i32 %193 to double
  %195 = add nsw i32 %184, -25
  %196 = icmp ugt i32 %184, 25
  br i1 %196, label %197, label %214

197:                                              ; preds = %192
  %198 = add nsw i32 %184, -26
  %199 = and i32 %195, 7
  %200 = icmp eq i32 %199, 0
  br i1 %200, label %209, label %201

201:                                              ; preds = %197, %201
  %202 = phi i32 [ %206, %201 ], [ %195, %197 ]
  %203 = phi double [ %205, %201 ], [ %194, %197 ]
  %204 = phi i32 [ %207, %201 ], [ 0, %197 ]
  %205 = fmul double %203, 2.000000e+00
  %206 = add nsw i32 %202, -1
  %207 = add i32 %204, 1
  %208 = icmp eq i32 %207, %199
  br i1 %208, label %209, label %201, !llvm.loop !10088

209:                                              ; preds = %201, %197
  %210 = phi i32 [ %195, %197 ], [ %206, %201 ]
  %211 = phi double [ %194, %197 ], [ %205, %201 ]
  %212 = phi double [ undef, %197 ], [ %205, %201 ]
  %213 = icmp ult i32 %198, 7
  br i1 %213, label %214, label %237

214:                                              ; preds = %209, %237, %192
  %215 = phi double [ %194, %192 ], [ %212, %209 ], [ %247, %237 ]
  %216 = phi i32 [ %195, %192 ], [ 0, %237 ], [ 0, %209 ]
  %217 = icmp slt i32 %216, 0
  br i1 %217, label %218, label %264

218:                                              ; preds = %214
  %219 = tail call i32 @llvm.umin.i32(i32 %184, i32 25)
  %220 = sub nsw i32 1, %219
  %221 = and i32 %220, 7
  %222 = icmp eq i32 %221, 0
  br i1 %222, label %231, label %223

223:                                              ; preds = %218, %223
  %224 = phi i32 [ %228, %223 ], [ %216, %218 ]
  %225 = phi double [ %227, %223 ], [ %215, %218 ]
  %226 = phi i32 [ %229, %223 ], [ 0, %218 ]
  %227 = fmul double %225, 5.000000e-01
  %228 = add nsw i32 %224, 1
  %229 = add i32 %226, 1
  %230 = icmp eq i32 %229, %221
  br i1 %230, label %231, label %223, !llvm.loop !10089

231:                                              ; preds = %223, %218
  %232 = phi double [ undef, %218 ], [ %227, %223 ]
  %233 = phi i32 [ %216, %218 ], [ %228, %223 ]
  %234 = phi double [ %215, %218 ], [ %227, %223 ]
  %235 = add nsw i32 %219, -18
  %236 = icmp ult i32 %235, 7
  br i1 %236, label %264, label %251

237:                                              ; preds = %209, %237
  %238 = phi i32 [ %248, %237 ], [ %210, %209 ]
  %239 = phi double [ %247, %237 ], [ %211, %209 ]
  %240 = fmul double %239, 2.000000e+00
  %241 = fmul double %240, 2.000000e+00
  %242 = fmul double %241, 2.000000e+00
  %243 = fmul double %242, 2.000000e+00
  %244 = fmul double %243, 2.000000e+00
  %245 = fmul double %244, 2.000000e+00
  %246 = fmul double %245, 2.000000e+00
  %247 = fmul double %246, 2.000000e+00
  %248 = add nsw i32 %238, -8
  %249 = add i32 %238, -9
  %250 = icmp ult i32 %249, -2
  br i1 %250, label %237, label %214, !llvm.loop !10021

251:                                              ; preds = %231, %251
  %252 = phi i32 [ %262, %251 ], [ %233, %231 ]
  %253 = phi double [ %261, %251 ], [ %234, %231 ]
  %254 = fmul double %253, 5.000000e-01
  %255 = fmul double %254, 5.000000e-01
  %256 = fmul double %255, 5.000000e-01
  %257 = fmul double %256, 5.000000e-01
  %258 = fmul double %257, 5.000000e-01
  %259 = fmul double %258, 5.000000e-01
  %260 = fmul double %259, 5.000000e-01
  %261 = fmul double %260, 5.000000e-01
  %262 = add nsw i32 %252, 8
  %263 = icmp eq i32 %262, 0
  br i1 %263, label %264, label %251, !llvm.loop !10022

264:                                              ; preds = %231, %251, %214, %189, %186
  %265 = phi double [ %188, %186 ], [ %191, %189 ], [ %215, %214 ], [ %232, %231 ], [ %261, %251 ]
  %266 = icmp ult i32 %179, 32768
  %267 = fneg double %265
  %268 = select i1 %266, double %265, double %267
  br label %308

269:                                              ; preds = %169
  %270 = bitcast i32 %179 to float
  %271 = fpext float %270 to double
  br label %308

272:                                              ; preds = %169, %169
  br label %274

273:                                              ; preds = %169
  br label %274

274:                                              ; preds = %273, %272, %169, %169
  %275 = phi i32 [ 32, %273 ], [ 16, %272 ], [ 8, %169 ], [ 8, %169 ]
  %276 = zext i32 %179 to i64
  switch i64 %123, label %286 [
    i64 21, label %277
    i64 20, label %277
    i64 3, label %277
    i64 5, label %277
  ]

277:                                              ; preds = %274, %274, %274, %274
  %278 = add nsw i32 %275, -1
  %279 = shl nuw i32 1, %278
  %280 = and i32 %279, %179
  %281 = icmp eq i32 %280, 0
  %282 = zext nneg i32 %275 to i64
  %283 = shl nsw i64 -1, %282
  %284 = select i1 %281, i64 0, i64 %283
  %285 = add nsw i64 %284, %276
  br label %286

286:                                              ; preds = %277, %274
  %287 = phi i64 [ %285, %277 ], [ %276, %274 ]
  %288 = load i64, ptr addrspace(1) %152, align 8
  %289 = icmp eq i64 %288, 0
  br i1 %289, label %290, label %292

290:                                              ; preds = %286
  %291 = sitofp i64 %287 to double
  br label %308

292:                                              ; preds = %286
  switch i64 %123, label %293 [
    i64 21, label %294
    i64 20, label %294
    i64 3, label %294
  ]

293:                                              ; preds = %292
  br label %294

294:                                              ; preds = %293, %292, %292, %292
  %295 = phi i32 [ -1, %292 ], [ %154, %293 ], [ -1, %292 ], [ -1, %292 ]
  %296 = add nsw i32 %295, %275
  %297 = zext nneg i32 %296 to i64
  %298 = shl nsw i64 -1, %297
  %299 = xor i64 %298, -1
  %300 = sitofp i64 %287 to double
  %301 = sitofp i64 %299 to double
  %302 = fdiv double %300, %301
  %303 = fcmp oge double %302, -1.000000e+00
  %304 = select i1 %303, double %302, double -1.000000e+00
  br label %308

305:                                              ; preds = %155
  %306 = icmp eq i32 %156, 3
  %307 = select i1 %306, double 1.000000e+00, double 0.000000e+00
  br label %308

308:                                              ; preds = %305, %294, %290, %269, %264
  %309 = phi double [ %307, %305 ], [ %268, %264 ], [ %271, %269 ], [ %304, %294 ], [ %291, %290 ]
  %310 = insertelement <4 x double> %157, double %309, i32 %156
  %311 = add nuw nsw i32 %156, 1
  %312 = icmp eq i32 %311, 4
  br i1 %312, label %726, label %155, !llvm.loop !10029

313:                                              ; preds = %2
  %314 = sitofp i64 %23 to double
  %315 = select i1 %12, double %314, double 1.000000e+00
  %316 = fmul double %315, %15
  %317 = fadd double %316, -5.000000e-01
  %318 = tail call double @llvm.floor.f64(double %317)
  %319 = fptosi double %318 to i64
  switch i32 %13, label %337 [
    i32 0, label %320
    i32 2, label %324
    i32 3, label %328
  ]

320:                                              ; preds = %313
  %321 = srem i64 %319, %23
  %322 = add nsw i64 %321, %23
  %323 = srem i64 %322, %23
  br label %341

324:                                              ; preds = %313
  %325 = icmp sgt i64 %23, %319
  %326 = tail call i64 @llvm.smax.i64(i64 %319, i64 -1)
  %327 = select i1 %325, i64 %326, i64 -1
  br label %341

328:                                              ; preds = %313
  %329 = shl nuw nsw i64 %23, 1
  %330 = srem i64 %319, %329
  %331 = add nsw i64 %330, %329
  %332 = srem i64 %331, %329
  %333 = icmp slt i64 %332, %23
  %334 = xor i64 %332, -1
  %335 = add i64 %329, %334
  %336 = select i1 %333, i64 %332, i64 %335
  br label %341

337:                                              ; preds = %313
  %338 = tail call noundef i64 @llvm.smax.i64(i64 %319, i64 0)
  %339 = add nsw i64 %23, -1
  %340 = tail call noundef i64 @llvm.smin.i64(i64 %338, i64 %339)
  br label %341

341:                                              ; preds = %337, %328, %324, %320
  %342 = phi i64 [ %340, %337 ], [ %336, %328 ], [ %327, %324 ], [ %323, %320 ]
  %343 = add i64 %319, 1
  switch i32 %13, label %361 [
    i32 0, label %344
    i32 2, label %348
    i32 3, label %352
  ]

344:                                              ; preds = %341
  %345 = srem i64 %343, %23
  %346 = add nsw i64 %345, %23
  %347 = srem i64 %346, %23
  br label %365

348:                                              ; preds = %341
  %349 = icmp slt i64 %343, %23
  %350 = tail call i64 @llvm.smax.i64(i64 %343, i64 -1)
  %351 = select i1 %349, i64 %350, i64 -1
  br label %365

352:                                              ; preds = %341
  %353 = shl nuw nsw i64 %23, 1
  %354 = srem i64 %343, %353
  %355 = add nsw i64 %354, %353
  %356 = srem i64 %355, %353
  %357 = icmp slt i64 %356, %23
  %358 = xor i64 %356, -1
  %359 = add i64 %353, %358
  %360 = select i1 %357, i64 %356, i64 %359
  br label %365

361:                                              ; preds = %341
  %362 = tail call noundef i64 @llvm.smax.i64(i64 %343, i64 0)
  %363 = add nsw i64 %23, -1
  %364 = tail call noundef i64 @llvm.smin.i64(i64 %362, i64 %363)
  br label %365

365:                                              ; preds = %361, %352, %348, %344
  %366 = phi i64 [ %364, %361 ], [ %360, %352 ], [ %351, %348 ], [ %347, %344 ]
  %367 = getelementptr i64, ptr addrspace(1) %3, i64 10
  %368 = load i64, ptr addrspace(1) %367, align 8
  %369 = tail call noundef i64 @llvm.smax.i64(i64 %368, i64 1)
  %370 = sitofp i64 %369 to double
  %371 = select i1 %12, double %370, double 1.000000e+00
  %372 = fmul double %371, %17
  %373 = fadd double %372, -5.000000e-01
  %374 = tail call double @llvm.floor.f64(double %373)
  %375 = fptosi double %374 to i64
  switch i32 %13, label %393 [
    i32 0, label %376
    i32 2, label %380
    i32 3, label %384
  ]

376:                                              ; preds = %365
  %377 = srem i64 %375, %369
  %378 = add nsw i64 %377, %369
  %379 = srem i64 %378, %369
  br label %397

380:                                              ; preds = %365
  %381 = icmp sgt i64 %369, %375
  %382 = tail call i64 @llvm.smax.i64(i64 %375, i64 -1)
  %383 = select i1 %381, i64 %382, i64 -1
  br label %397

384:                                              ; preds = %365
  %385 = shl nuw nsw i64 %369, 1
  %386 = srem i64 %375, %385
  %387 = add nsw i64 %386, %385
  %388 = srem i64 %387, %385
  %389 = icmp slt i64 %388, %369
  %390 = xor i64 %388, -1
  %391 = add i64 %385, %390
  %392 = select i1 %389, i64 %388, i64 %391
  br label %397

393:                                              ; preds = %365
  %394 = tail call noundef i64 @llvm.smax.i64(i64 %375, i64 0)
  %395 = add nsw i64 %369, -1
  %396 = tail call noundef i64 @llvm.smin.i64(i64 %394, i64 %395)
  br label %397

397:                                              ; preds = %393, %384, %380, %376
  %398 = phi i64 [ %396, %393 ], [ %392, %384 ], [ %383, %380 ], [ %379, %376 ]
  %399 = add i64 %375, 1
  switch i32 %13, label %417 [
    i32 0, label %400
    i32 2, label %404
    i32 3, label %408
  ]

400:                                              ; preds = %397
  %401 = srem i64 %399, %369
  %402 = add nsw i64 %401, %369
  %403 = srem i64 %402, %369
  br label %421

404:                                              ; preds = %397
  %405 = icmp slt i64 %399, %369
  %406 = tail call i64 @llvm.smax.i64(i64 %399, i64 -1)
  %407 = select i1 %405, i64 %406, i64 -1
  br label %421

408:                                              ; preds = %397
  %409 = shl nuw nsw i64 %369, 1
  %410 = srem i64 %399, %409
  %411 = add nsw i64 %410, %409
  %412 = srem i64 %411, %409
  %413 = icmp slt i64 %412, %369
  %414 = xor i64 %412, -1
  %415 = add i64 %409, %414
  %416 = select i1 %413, i64 %412, i64 %415
  br label %421

417:                                              ; preds = %397
  %418 = tail call noundef i64 @llvm.smax.i64(i64 %399, i64 0)
  %419 = add nsw i64 %369, -1
  %420 = tail call noundef i64 @llvm.smin.i64(i64 %418, i64 %419)
  br label %421

421:                                              ; preds = %417, %408, %404, %400
  %422 = phi i64 [ %420, %417 ], [ %416, %408 ], [ %407, %404 ], [ %403, %400 ]
  %423 = getelementptr i64, ptr addrspace(1) %3, i64 11
  %424 = load i64, ptr addrspace(1) %423, align 8
  %425 = tail call noundef i64 @llvm.smax.i64(i64 %424, i64 1)
  %426 = sitofp i64 %425 to double
  %427 = select i1 %12, double %426, double 1.000000e+00
  %428 = fmul double %427, %19
  %429 = fadd double %428, -5.000000e-01
  %430 = tail call double @llvm.floor.f64(double %429)
  %431 = fptosi double %430 to i64
  switch i32 %13, label %449 [
    i32 0, label %432
    i32 2, label %436
    i32 3, label %440
  ]

432:                                              ; preds = %421
  %433 = srem i64 %431, %425
  %434 = add nsw i64 %433, %425
  %435 = srem i64 %434, %425
  br label %453

436:                                              ; preds = %421
  %437 = icmp sgt i64 %425, %431
  %438 = tail call i64 @llvm.smax.i64(i64 %431, i64 -1)
  %439 = select i1 %437, i64 %438, i64 -1
  br label %453

440:                                              ; preds = %421
  %441 = shl nuw nsw i64 %425, 1
  %442 = srem i64 %431, %441
  %443 = add nsw i64 %442, %441
  %444 = srem i64 %443, %441
  %445 = icmp slt i64 %444, %425
  %446 = xor i64 %444, -1
  %447 = add i64 %441, %446
  %448 = select i1 %445, i64 %444, i64 %447
  br label %453

449:                                              ; preds = %421
  %450 = tail call noundef i64 @llvm.smax.i64(i64 %431, i64 0)
  %451 = add nsw i64 %425, -1
  %452 = tail call noundef i64 @llvm.smin.i64(i64 %450, i64 %451)
  br label %453

453:                                              ; preds = %449, %440, %436, %432
  %454 = phi i64 [ %452, %449 ], [ %448, %440 ], [ %439, %436 ], [ %435, %432 ]
  %455 = add i64 %431, 1
  switch i32 %13, label %473 [
    i32 0, label %456
    i32 2, label %460
    i32 3, label %464
  ]

456:                                              ; preds = %453
  %457 = srem i64 %455, %425
  %458 = add nsw i64 %457, %425
  %459 = srem i64 %458, %425
  br label %477

460:                                              ; preds = %453
  %461 = icmp slt i64 %455, %425
  %462 = tail call i64 @llvm.smax.i64(i64 %455, i64 -1)
  %463 = select i1 %461, i64 %462, i64 -1
  br label %477

464:                                              ; preds = %453
  %465 = shl nuw nsw i64 %425, 1
  %466 = srem i64 %455, %465
  %467 = add nsw i64 %466, %465
  %468 = srem i64 %467, %465
  %469 = icmp slt i64 %468, %425
  %470 = xor i64 %468, -1
  %471 = add i64 %465, %470
  %472 = select i1 %469, i64 %468, i64 %471
  br label %477

473:                                              ; preds = %453
  %474 = tail call noundef i64 @llvm.smax.i64(i64 %455, i64 0)
  %475 = add nsw i64 %425, -1
  %476 = tail call noundef i64 @llvm.smin.i64(i64 %474, i64 %475)
  br label %477

477:                                              ; preds = %473, %464, %460, %456
  %478 = phi i64 [ %476, %473 ], [ %472, %464 ], [ %463, %460 ], [ %459, %456 ]
  %479 = fsub double %317, %318
  %480 = fmul double %479, 2.560000e+02
  %481 = fadd double %480, 5.000000e-01
  %482 = tail call double @llvm.floor.f64(double %481)
  %483 = fptosi double %482 to i64
  %484 = sitofp i64 %483 to double
  %485 = fmul double %484, 3.906250e-03
  %486 = fsub double %373, %374
  %487 = fmul double %486, 2.560000e+02
  %488 = fadd double %487, 5.000000e-01
  %489 = tail call double @llvm.floor.f64(double %488)
  %490 = fptosi double %489 to i64
  %491 = sitofp i64 %490 to double
  %492 = fmul double %491, 3.906250e-03
  %493 = fsub double %429, %430
  %494 = fmul double %493, 2.560000e+02
  %495 = fadd double %494, 5.000000e-01
  %496 = tail call double @llvm.floor.f64(double %495)
  %497 = fptosi double %496 to i64
  %498 = sitofp i64 %497 to double
  %499 = fmul double %498, 3.906250e-03
  %500 = fsub double 1.000000e+00, %485
  %501 = fsub double 1.000000e+00, %492
  %502 = fsub double 1.000000e+00, %499
  %503 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 3
  %504 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 2
  %505 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 14
  %506 = getelementptr i64, ptr addrspace(1) %3, i64 1
  %507 = getelementptr inbounds i64, ptr addrspace(1) %3, i64 7
  br label %508

508:                                              ; preds = %720, %477
  %509 = phi <4 x double> [ zeroinitializer, %477 ], [ %723, %720 ]
  %510 = phi i32 [ 0, %477 ], [ %724, %720 ]
  %511 = and i32 %510, 1
  %512 = icmp eq i32 %511, 0
  %513 = select i1 %512, double %500, double %485
  %514 = and i32 %510, 2
  %515 = icmp eq i32 %514, 0
  %516 = select i1 %515, double %501, double %492
  %517 = fmul double %513, %516
  %518 = and i32 %510, 4
  %519 = icmp eq i32 %518, 0
  %520 = select i1 %519, double %502, double %499
  %521 = fmul double %520, %517
  %522 = insertelement <4 x double> poison, double %521, i64 0
  %523 = shufflevector <4 x double> %522, <4 x double> poison, <4 x i32> zeroinitializer
  %524 = select i1 %512, i64 %342, i64 %366
  %525 = select i1 %515, i64 %398, i64 %422
  %526 = select i1 %519, i64 %454, i64 %478
  %527 = or i64 %525, %524
  %528 = or i64 %527, %526
  %529 = icmp sgt i64 %528, -1
  br i1 %529, label %530, label %720

530:                                              ; preds = %508
  %531 = load i64, ptr addrspace(1) %503, align 8
  %532 = load i64, ptr addrspace(1) %504, align 8
  switch i64 %532, label %534 [
    i64 0, label %535
    i64 20, label %535
    i64 1, label %533
    i64 21, label %533
    i64 6, label %533
  ]

533:                                              ; preds = %530, %530, %530
  br label %535

534:                                              ; preds = %530
  br label %535

535:                                              ; preds = %534, %533, %530, %530
  %536 = phi i32 [ 4, %534 ], [ 2, %533 ], [ 1, %530 ], [ 1, %530 ]
  %537 = load i64, ptr addrspace(1) %505, align 8
  %538 = icmp eq i64 %537, 0
  %539 = zext nneg i32 %536 to i64
  br i1 %538, label %549, label %540

540:                                              ; preds = %535
  %541 = load i64, ptr addrspace(1) %506, align 8
  %542 = mul nsw i64 %526, %369
  %543 = add nsw i64 %542, %525
  %544 = mul nsw i64 %537, %543
  %545 = mul nsw i64 %531, %524
  %546 = mul nsw i64 %545, %539
  %547 = add nsw i64 %544, %546
  %548 = add i64 %547, %541
  br label %558

549:                                              ; preds = %535
  %550 = mul nsw i64 %526, %369
  %551 = add nsw i64 %550, %525
  %552 = mul nsw i64 %551, %23
  %553 = add i64 %552, %524
  %554 = load i64, ptr addrspace(1) %506, align 8
  %555 = mul i64 %531, %553
  %556 = mul i64 %555, %539
  %557 = add i64 %554, %556
  br label %558

558:                                              ; preds = %549, %540
  %559 = phi i64 [ %548, %540 ], [ %557, %549 ]
  %560 = icmp eq i64 %532, 5
  %561 = sext i1 %560 to i32
  br label %562

562:                                              ; preds = %715, %558
  %563 = phi i32 [ 0, %558 ], [ %718, %715 ]
  %564 = phi <4 x double> [ zeroinitializer, %558 ], [ %717, %715 ]
  %565 = zext nneg i32 %563 to i64
  %566 = icmp sgt i64 %531, %565
  br i1 %566, label %567, label %712

567:                                              ; preds = %562
  %568 = mul nuw nsw i32 %563, %536
  %569 = zext nneg i32 %568 to i64
  %570 = add i64 %559, %569
  switch i64 %532, label %572 [
    i64 0, label %573
    i64 20, label %573
    i64 1, label %571
    i64 21, label %571
    i64 6, label %571
  ]

571:                                              ; preds = %567, %567, %567
  br label %573

572:                                              ; preds = %567
  br label %573

573:                                              ; preds = %572, %571, %567, %567
  %574 = phi i32 [ 4, %572 ], [ 2, %571 ], [ 1, %567 ], [ 1, %567 ]
  %575 = inttoptr i64 %570 to ptr addrspace(1)
  br label %577

576:                                              ; preds = %577
  switch i64 %532, label %680 [
    i64 6, label %589
    i64 7, label %676
    i64 0, label %681
    i64 20, label %681
    i64 1, label %679
    i64 21, label %679
  ]

577:                                              ; preds = %577, %573
  %578 = phi i32 [ 0, %573 ], [ %586, %577 ]
  %579 = phi i32 [ 0, %573 ], [ %587, %577 ]
  %580 = zext nneg i32 %579 to i64
  %581 = getelementptr inbounds i8, ptr addrspace(1) %575, i64 %580
  %582 = load i8, ptr addrspace(1) %581, align 1
  %583 = zext i8 %582 to i32
  %584 = shl nsw i32 %579, 3
  %585 = shl i32 %583, %584
  %586 = or i32 %585, %578
  %587 = add nuw nsw i32 %579, 1
  %588 = icmp eq i32 %587, %574
  br i1 %588, label %576, label %577, !llvm.loop !10016

589:                                              ; preds = %576
  %590 = lshr i32 %586, 10
  %591 = and i32 %590, 31
  %592 = and i32 %586, 1023
  switch i32 %591, label %599 [
    i32 0, label %593
    i32 31, label %596
  ]

593:                                              ; preds = %589
  %594 = uitofp i32 %592 to double
  %595 = fmul double %594, 0x3E70000000000000
  br label %671

596:                                              ; preds = %589
  %597 = icmp eq i32 %592, 0
  %598 = select i1 %597, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %671

599:                                              ; preds = %589
  %600 = or disjoint i32 %592, 1024
  %601 = uitofp i32 %600 to double
  %602 = add nsw i32 %591, -25
  %603 = icmp ugt i32 %591, 25
  br i1 %603, label %604, label %621

604:                                              ; preds = %599
  %605 = add nsw i32 %591, -26
  %606 = and i32 %602, 7
  %607 = icmp eq i32 %606, 0
  br i1 %607, label %616, label %608

608:                                              ; preds = %604, %608
  %609 = phi i32 [ %613, %608 ], [ %602, %604 ]
  %610 = phi double [ %612, %608 ], [ %601, %604 ]
  %611 = phi i32 [ %614, %608 ], [ 0, %604 ]
  %612 = fmul double %610, 2.000000e+00
  %613 = add nsw i32 %609, -1
  %614 = add i32 %611, 1
  %615 = icmp eq i32 %614, %606
  br i1 %615, label %616, label %608, !llvm.loop !10090

616:                                              ; preds = %608, %604
  %617 = phi i32 [ %602, %604 ], [ %613, %608 ]
  %618 = phi double [ %601, %604 ], [ %612, %608 ]
  %619 = phi double [ undef, %604 ], [ %612, %608 ]
  %620 = icmp ult i32 %605, 7
  br i1 %620, label %621, label %644

621:                                              ; preds = %616, %644, %599
  %622 = phi double [ %601, %599 ], [ %619, %616 ], [ %654, %644 ]
  %623 = phi i32 [ %602, %599 ], [ 0, %644 ], [ 0, %616 ]
  %624 = icmp slt i32 %623, 0
  br i1 %624, label %625, label %671

625:                                              ; preds = %621
  %626 = tail call i32 @llvm.umin.i32(i32 %591, i32 25)
  %627 = sub nsw i32 1, %626
  %628 = and i32 %627, 7
  %629 = icmp eq i32 %628, 0
  br i1 %629, label %638, label %630

630:                                              ; preds = %625, %630
  %631 = phi i32 [ %635, %630 ], [ %623, %625 ]
  %632 = phi double [ %634, %630 ], [ %622, %625 ]
  %633 = phi i32 [ %636, %630 ], [ 0, %625 ]
  %634 = fmul double %632, 5.000000e-01
  %635 = add nsw i32 %631, 1
  %636 = add i32 %633, 1
  %637 = icmp eq i32 %636, %628
  br i1 %637, label %638, label %630, !llvm.loop !10091

638:                                              ; preds = %630, %625
  %639 = phi double [ undef, %625 ], [ %634, %630 ]
  %640 = phi i32 [ %623, %625 ], [ %635, %630 ]
  %641 = phi double [ %622, %625 ], [ %634, %630 ]
  %642 = add nsw i32 %626, -18
  %643 = icmp ult i32 %642, 7
  br i1 %643, label %671, label %658

644:                                              ; preds = %616, %644
  %645 = phi i32 [ %655, %644 ], [ %617, %616 ]
  %646 = phi double [ %654, %644 ], [ %618, %616 ]
  %647 = fmul double %646, 2.000000e+00
  %648 = fmul double %647, 2.000000e+00
  %649 = fmul double %648, 2.000000e+00
  %650 = fmul double %649, 2.000000e+00
  %651 = fmul double %650, 2.000000e+00
  %652 = fmul double %651, 2.000000e+00
  %653 = fmul double %652, 2.000000e+00
  %654 = fmul double %653, 2.000000e+00
  %655 = add nsw i32 %645, -8
  %656 = add i32 %645, -9
  %657 = icmp ult i32 %656, -2
  br i1 %657, label %644, label %621, !llvm.loop !10021

658:                                              ; preds = %638, %658
  %659 = phi i32 [ %669, %658 ], [ %640, %638 ]
  %660 = phi double [ %668, %658 ], [ %641, %638 ]
  %661 = fmul double %660, 5.000000e-01
  %662 = fmul double %661, 5.000000e-01
  %663 = fmul double %662, 5.000000e-01
  %664 = fmul double %663, 5.000000e-01
  %665 = fmul double %664, 5.000000e-01
  %666 = fmul double %665, 5.000000e-01
  %667 = fmul double %666, 5.000000e-01
  %668 = fmul double %667, 5.000000e-01
  %669 = add nsw i32 %659, 8
  %670 = icmp eq i32 %669, 0
  br i1 %670, label %671, label %658, !llvm.loop !10022

671:                                              ; preds = %638, %658, %621, %596, %593
  %672 = phi double [ %595, %593 ], [ %598, %596 ], [ %622, %621 ], [ %639, %638 ], [ %668, %658 ]
  %673 = icmp ult i32 %586, 32768
  %674 = fneg double %672
  %675 = select i1 %673, double %672, double %674
  br label %715

676:                                              ; preds = %576
  %677 = bitcast i32 %586 to float
  %678 = fpext float %677 to double
  br label %715

679:                                              ; preds = %576, %576
  br label %681

680:                                              ; preds = %576
  br label %681

681:                                              ; preds = %680, %679, %576, %576
  %682 = phi i32 [ 32, %680 ], [ 16, %679 ], [ 8, %576 ], [ 8, %576 ]
  %683 = zext i32 %586 to i64
  switch i64 %532, label %693 [
    i64 21, label %684
    i64 20, label %684
    i64 3, label %684
    i64 5, label %684
  ]

684:                                              ; preds = %681, %681, %681, %681
  %685 = add nsw i32 %682, -1
  %686 = shl nuw i32 1, %685
  %687 = and i32 %686, %586
  %688 = icmp eq i32 %687, 0
  %689 = zext nneg i32 %682 to i64
  %690 = shl nsw i64 -1, %689
  %691 = select i1 %688, i64 0, i64 %690
  %692 = add nsw i64 %691, %683
  br label %693

693:                                              ; preds = %684, %681
  %694 = phi i64 [ %692, %684 ], [ %683, %681 ]
  %695 = load i64, ptr addrspace(1) %507, align 8
  %696 = icmp eq i64 %695, 0
  br i1 %696, label %697, label %699

697:                                              ; preds = %693
  %698 = sitofp i64 %694 to double
  br label %715

699:                                              ; preds = %693
  switch i64 %532, label %700 [
    i64 21, label %701
    i64 20, label %701
    i64 3, label %701
  ]

700:                                              ; preds = %699
  br label %701

701:                                              ; preds = %700, %699, %699, %699
  %702 = phi i32 [ -1, %699 ], [ %561, %700 ], [ -1, %699 ], [ -1, %699 ]
  %703 = add nsw i32 %702, %682
  %704 = zext nneg i32 %703 to i64
  %705 = shl nsw i64 -1, %704
  %706 = xor i64 %705, -1
  %707 = sitofp i64 %694 to double
  %708 = sitofp i64 %706 to double
  %709 = fdiv double %707, %708
  %710 = fcmp oge double %709, -1.000000e+00
  %711 = select i1 %710, double %709, double -1.000000e+00
  br label %715

712:                                              ; preds = %562
  %713 = icmp eq i32 %563, 3
  %714 = select i1 %713, double 1.000000e+00, double 0.000000e+00
  br label %715

715:                                              ; preds = %712, %701, %697, %676, %671
  %716 = phi double [ %714, %712 ], [ %675, %671 ], [ %678, %676 ], [ %711, %701 ], [ %698, %697 ]
  %717 = insertelement <4 x double> %564, double %716, i32 %563
  %718 = add nuw nsw i32 %563, 1
  %719 = icmp eq i32 %718, 4
  br i1 %719, label %720, label %562, !llvm.loop !10029

720:                                              ; preds = %715, %508
  %721 = phi <4 x double> [ zeroinitializer, %508 ], [ %717, %715 ]
  %722 = fmul <4 x double> %523, %721
  %723 = fadd <4 x double> %509, %722
  %724 = add nuw nsw i32 %510, 1
  %725 = icmp eq i32 %724, 8
  br i1 %725, label %726, label %508, !llvm.loop !10032

726:                                              ; preds = %308, %720, %115
  %727 = phi <4 x double> [ zeroinitializer, %115 ], [ %723, %720 ], [ %310, %308 ]
  %728 = extractelement <4 x double> %727, i64 0
  %729 = fcmp uno double %728, 0.000000e+00
  %730 = fptosi double %728 to i64
  %731 = trunc i64 %730 to i32
  %732 = select i1 %729, i32 0, i32 %731
  %733 = insertelement <4 x i32> poison, i32 %732, i64 0
  %734 = extractelement <4 x double> %727, i64 1
  %735 = fcmp uno double %734, 0.000000e+00
  %736 = fptosi double %734 to i64
  %737 = trunc i64 %736 to i32
  %738 = select i1 %735, i32 0, i32 %737
  %739 = insertelement <4 x i32> %733, i32 %738, i64 1
  %740 = extractelement <4 x double> %727, i64 2
  %741 = fcmp uno double %740, 0.000000e+00
  %742 = fptosi double %740 to i64
  %743 = trunc i64 %742 to i32
  %744 = select i1 %741, i32 0, i32 %743
  %745 = insertelement <4 x i32> %739, i32 %744, i64 2
  %746 = extractelement <4 x double> %727, i64 3
  %747 = fcmp uno double %746, 0.000000e+00
  %748 = fptosi double %746 to i64
  %749 = trunc i64 %748 to i32
  %750 = select i1 %747, i32 0, i32 %749
  %751 = insertelement <4 x i32> %745, i32 %750, i64 3
  ret <4 x i32> %751
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define dso_local <4 x i32> @__zluda_ptx_impl_texobj_3d_v4_s32_s32(i64 noundef %0, <4 x i32> noundef %1) local_unnamed_addr #3 {
  %3 = extractelement <4 x i32> %1, i64 1
  %4 = inttoptr i64 %0 to ptr addrspace(1)
  %5 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 5
  %6 = load i64, ptr addrspace(1) %5, align 8
  %7 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 6
  %8 = load i64, ptr addrspace(1) %7, align 8
  %9 = getelementptr i64, ptr addrspace(1) %4, i64 9
  %10 = and i64 %8, 4294967295
  %11 = icmp eq i64 %10, 1
  %12 = extractelement <4 x i32> %1, i64 0
  %13 = extractelement <4 x i32> %1, i64 2
  %14 = trunc i64 %6 to i32
  %15 = sitofp i32 %12 to double
  %16 = sitofp i32 %3 to double
  %17 = sitofp i32 %13 to double
  %18 = load i64, ptr addrspace(1) %9, align 8
  %19 = tail call noundef i64 @llvm.smax.i64(i64 %18, i64 1)
  %20 = getelementptr i64, ptr addrspace(1) %4, i64 1
  %21 = getelementptr i64, ptr addrspace(1) %4, i64 10
  %22 = load i64, ptr addrspace(1) %21, align 8
  %23 = tail call noundef i64 @llvm.smax.i64(i64 %22, i64 1)
  %24 = getelementptr i64, ptr addrspace(1) %4, i64 2
  %25 = getelementptr i64, ptr addrspace(1) %4, i64 11
  %26 = load i64, ptr addrspace(1) %25, align 8
  %27 = tail call noundef i64 @llvm.smax.i64(i64 %26, i64 1)
  %28 = sitofp i64 %19 to double
  %29 = select i1 %11, double %28, double 1.000000e+00
  %30 = fmul double %29, %15
  %31 = tail call double @llvm.floor.f64(double %30)
  %32 = fptosi double %31 to i64
  switch i32 %14, label %50 [
    i32 0, label %33
    i32 2, label %37
    i32 3, label %41
  ]

33:                                               ; preds = %2
  %34 = srem i64 %32, %19
  %35 = add nsw i64 %34, %19
  %36 = srem i64 %35, %19
  br label %54

37:                                               ; preds = %2
  %38 = icmp sgt i64 %19, %32
  %39 = tail call i64 @llvm.smax.i64(i64 %32, i64 -1)
  %40 = select i1 %38, i64 %39, i64 -1
  br label %54

41:                                               ; preds = %2
  %42 = shl nuw nsw i64 %19, 1
  %43 = srem i64 %32, %42
  %44 = add nsw i64 %43, %42
  %45 = srem i64 %44, %42
  %46 = icmp slt i64 %45, %19
  %47 = xor i64 %45, -1
  %48 = add i64 %42, %47
  %49 = select i1 %46, i64 %45, i64 %48
  br label %54

50:                                               ; preds = %2
  %51 = tail call noundef i64 @llvm.smax.i64(i64 %32, i64 0)
  %52 = add nsw i64 %19, -1
  %53 = tail call noundef i64 @llvm.smin.i64(i64 %51, i64 %52)
  br label %54

54:                                               ; preds = %50, %41, %37, %33
  %55 = phi i64 [ %53, %50 ], [ %49, %41 ], [ %40, %37 ], [ %36, %33 ]
  %56 = sitofp i64 %23 to double
  %57 = select i1 %11, double %56, double 1.000000e+00
  %58 = fmul double %57, %16
  %59 = tail call double @llvm.floor.f64(double %58)
  %60 = fptosi double %59 to i64
  switch i32 %14, label %78 [
    i32 0, label %61
    i32 2, label %65
    i32 3, label %69
  ]

61:                                               ; preds = %54
  %62 = srem i64 %60, %23
  %63 = add nsw i64 %62, %23
  %64 = srem i64 %63, %23
  br label %82

65:                                               ; preds = %54
  %66 = icmp sgt i64 %23, %60
  %67 = tail call i64 @llvm.smax.i64(i64 %60, i64 -1)
  %68 = select i1 %66, i64 %67, i64 -1
  br label %82

69:                                               ; preds = %54
  %70 = shl nuw nsw i64 %23, 1
  %71 = srem i64 %60, %70
  %72 = add nsw i64 %71, %70
  %73 = srem i64 %72, %70
  %74 = icmp slt i64 %73, %23
  %75 = xor i64 %73, -1
  %76 = add i64 %70, %75
  %77 = select i1 %74, i64 %73, i64 %76
  br label %82

78:                                               ; preds = %54
  %79 = tail call noundef i64 @llvm.smax.i64(i64 %60, i64 0)
  %80 = add nsw i64 %23, -1
  %81 = tail call noundef i64 @llvm.smin.i64(i64 %79, i64 %80)
  br label %82

82:                                               ; preds = %78, %69, %65, %61
  %83 = phi i64 [ %81, %78 ], [ %77, %69 ], [ %68, %65 ], [ %64, %61 ]
  %84 = sitofp i64 %27 to double
  %85 = select i1 %11, double %84, double 1.000000e+00
  %86 = fmul double %85, %17
  %87 = tail call double @llvm.floor.f64(double %86)
  %88 = fptosi double %87 to i64
  switch i32 %14, label %106 [
    i32 0, label %89
    i32 2, label %93
    i32 3, label %97
  ]

89:                                               ; preds = %82
  %90 = srem i64 %88, %27
  %91 = add nsw i64 %90, %27
  %92 = srem i64 %91, %27
  br label %110

93:                                               ; preds = %82
  %94 = icmp sgt i64 %27, %88
  %95 = tail call i64 @llvm.smax.i64(i64 %88, i64 -1)
  %96 = select i1 %94, i64 %95, i64 -1
  br label %110

97:                                               ; preds = %82
  %98 = shl nuw nsw i64 %27, 1
  %99 = srem i64 %88, %98
  %100 = add nsw i64 %99, %98
  %101 = srem i64 %100, %98
  %102 = icmp slt i64 %101, %27
  %103 = xor i64 %101, -1
  %104 = add i64 %98, %103
  %105 = select i1 %102, i64 %101, i64 %104
  br label %110

106:                                              ; preds = %82
  %107 = tail call noundef i64 @llvm.smax.i64(i64 %88, i64 0)
  %108 = add nsw i64 %27, -1
  %109 = tail call noundef i64 @llvm.smin.i64(i64 %107, i64 %108)
  br label %110

110:                                              ; preds = %106, %97, %93, %89
  %111 = phi i64 [ %109, %106 ], [ %105, %97 ], [ %96, %93 ], [ %92, %89 ]
  %112 = or i64 %83, %55
  %113 = or i64 %112, %111
  %114 = icmp sgt i64 %113, -1
  br i1 %114, label %115, label %308

115:                                              ; preds = %110
  %116 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 3
  %117 = load i64, ptr addrspace(1) %116, align 8
  %118 = load i64, ptr addrspace(1) %24, align 8
  switch i64 %118, label %120 [
    i64 0, label %121
    i64 20, label %121
    i64 1, label %119
    i64 21, label %119
    i64 6, label %119
  ]

119:                                              ; preds = %115, %115, %115
  br label %121

120:                                              ; preds = %115
  br label %121

121:                                              ; preds = %120, %119, %115, %115
  %122 = phi i32 [ 4, %120 ], [ 2, %119 ], [ 1, %115 ], [ 1, %115 ]
  %123 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 14
  %124 = load i64, ptr addrspace(1) %123, align 8
  %125 = icmp eq i64 %124, 0
  %126 = zext nneg i32 %122 to i64
  br i1 %125, label %136, label %127

127:                                              ; preds = %121
  %128 = load i64, ptr addrspace(1) %20, align 8
  %129 = mul nsw i64 %111, %23
  %130 = add nsw i64 %129, %83
  %131 = mul nsw i64 %124, %130
  %132 = mul nsw i64 %117, %55
  %133 = mul nsw i64 %132, %126
  %134 = add nsw i64 %131, %133
  %135 = add i64 %134, %128
  br label %145

136:                                              ; preds = %121
  %137 = mul nsw i64 %111, %23
  %138 = add nsw i64 %137, %83
  %139 = mul nsw i64 %138, %19
  %140 = add i64 %139, %55
  %141 = load i64, ptr addrspace(1) %20, align 8
  %142 = mul i64 %117, %140
  %143 = mul i64 %142, %126
  %144 = add i64 %141, %143
  br label %145

145:                                              ; preds = %136, %127
  %146 = phi i64 [ %135, %127 ], [ %144, %136 ]
  %147 = getelementptr inbounds i64, ptr addrspace(1) %4, i64 7
  %148 = icmp eq i64 %118, 5
  %149 = sext i1 %148 to i32
  br label %150

150:                                              ; preds = %303, %145
  %151 = phi i32 [ 0, %145 ], [ %306, %303 ]
  %152 = phi <4 x double> [ zeroinitializer, %145 ], [ %305, %303 ]
  %153 = zext nneg i32 %151 to i64
  %154 = icmp sgt i64 %117, %153
  br i1 %154, label %155, label %300

155:                                              ; preds = %150
  %156 = mul nuw nsw i32 %151, %122
  %157 = zext nneg i32 %156 to i64
  %158 = add i64 %146, %157
  switch i64 %118, label %160 [
    i64 0, label %161
    i64 20, label %161
    i64 1, label %159
    i64 21, label %159
    i64 6, label %159
  ]

159:                                              ; preds = %155, %155, %155
  br label %161

160:                                              ; preds = %155
  br label %161

161:                                              ; preds = %160, %159, %155, %155
  %162 = phi i32 [ 4, %160 ], [ 2, %159 ], [ 1, %155 ], [ 1, %155 ]
  %163 = inttoptr i64 %158 to ptr addrspace(1)
  br label %165

164:                                              ; preds = %165
  switch i64 %118, label %268 [
    i64 6, label %177
    i64 7, label %264
    i64 0, label %269
    i64 20, label %269
    i64 1, label %267
    i64 21, label %267
  ]

165:                                              ; preds = %165, %161
  %166 = phi i32 [ 0, %161 ], [ %174, %165 ]
  %167 = phi i32 [ 0, %161 ], [ %175, %165 ]
  %168 = zext nneg i32 %167 to i64
  %169 = getelementptr inbounds i8, ptr addrspace(1) %163, i64 %168
  %170 = load i8, ptr addrspace(1) %169, align 1
  %171 = zext i8 %170 to i32
  %172 = shl nsw i32 %167, 3
  %173 = shl i32 %171, %172
  %174 = or i32 %173, %166
  %175 = add nuw nsw i32 %167, 1
  %176 = icmp eq i32 %175, %162
  br i1 %176, label %164, label %165, !llvm.loop !10016

177:                                              ; preds = %164
  %178 = lshr i32 %174, 10
  %179 = and i32 %178, 31
  %180 = and i32 %174, 1023
  switch i32 %179, label %187 [
    i32 0, label %181
    i32 31, label %184
  ]

181:                                              ; preds = %177
  %182 = uitofp i32 %180 to double
  %183 = fmul double %182, 0x3E70000000000000
  br label %259

184:                                              ; preds = %177
  %185 = icmp eq i32 %180, 0
  %186 = select i1 %185, double 0x7FF0000000000000, double 0x7FF8000000000000
  br label %259

187:                                              ; preds = %177
  %188 = or disjoint i32 %180, 1024
  %189 = uitofp i32 %188 to double
  %190 = add nsw i32 %179, -25
  %191 = icmp ugt i32 %179, 25
  br i1 %191, label %192, label %209

192:                                              ; preds = %187
  %193 = add nsw i32 %179, -26
  %194 = and i32 %190, 7
  %195 = icmp eq i32 %194, 0
  br i1 %195, label %204, label %196

196:                                              ; preds = %192, %196
  %197 = phi i32 [ %201, %196 ], [ %190, %192 ]
  %198 = phi double [ %200, %196 ], [ %189, %192 ]
  %199 = phi i32 [ %202, %196 ], [ 0, %192 ]
  %200 = fmul double %198, 2.000000e+00
  %201 = add nsw i32 %197, -1
  %202 = add i32 %199, 1
  %203 = icmp eq i32 %202, %194
  br i1 %203, label %204, label %196, !llvm.loop !10092

204:                                              ; preds = %196, %192
  %205 = phi i32 [ %190, %192 ], [ %201, %196 ]
  %206 = phi double [ %189, %192 ], [ %200, %196 ]
  %207 = phi double [ undef, %192 ], [ %200, %196 ]
  %208 = icmp ult i32 %193, 7
  br i1 %208, label %209, label %232

209:                                              ; preds = %204, %232, %187
  %210 = phi double [ %189, %187 ], [ %207, %204 ], [ %242, %232 ]
  %211 = phi i32 [ %190, %187 ], [ 0, %232 ], [ 0, %204 ]
  %212 = icmp slt i32 %211, 0
  br i1 %212, label %213, label %259

213:                                              ; preds = %209
  %214 = tail call i32 @llvm.umin.i32(i32 %179, i32 25)
  %215 = sub nsw i32 1, %214
  %216 = and i32 %215, 7
  %217 = icmp eq i32 %216, 0
  br i1 %217, label %226, label %218

218:                                              ; preds = %213, %218
  %219 = phi i32 [ %223, %218 ], [ %211, %213 ]
  %220 = phi double [ %222, %218 ], [ %210, %213 ]
  %221 = phi i32 [ %224, %218 ], [ 0, %213 ]
  %222 = fmul double %220, 5.000000e-01
  %223 = add nsw i32 %219, 1
  %224 = add i32 %221, 1
  %225 = icmp eq i32 %224, %216
  br i1 %225, label %226, label %218, !llvm.loop !10093

226:                                              ; preds = %218, %213
  %227 = phi double [ undef, %213 ], [ %222, %218 ]
  %228 = phi i32 [ %211, %213 ], [ %223, %218 ]
  %229 = phi double [ %210, %213 ], [ %222, %218 ]
  %230 = add nsw i32 %214, -18
  %231 = icmp ult i32 %230, 7
  br i1 %231, label %259, label %246

232:                                              ; preds = %204, %232
  %233 = phi i32 [ %243, %232 ], [ %205, %204 ]
  %234 = phi double [ %242, %232 ], [ %206, %204 ]
  %235 = fmul double %234, 2.000000e+00
  %236 = fmul double %235, 2.000000e+00
  %237 = fmul double %236, 2.000000e+00
  %238 = fmul double %237, 2.000000e+00
  %239 = fmul double %238, 2.000000e+00
  %240 = fmul double %239, 2.000000e+00
  %241 = fmul double %240, 2.000000e+00
  %242 = fmul double %241, 2.000000e+00
  %243 = add nsw i32 %233, -8
  %244 = add i32 %233, -9
  %245 = icmp ult i32 %244, -2
  br i1 %245, label %232, label %209, !llvm.loop !10021

246:                                              ; preds = %226, %246
  %247 = phi i32 [ %257, %246 ], [ %228, %226 ]
  %248 = phi double [ %256, %246 ], [ %229, %226 ]
  %249 = fmul double %248, 5.000000e-01
  %250 = fmul double %249, 5.000000e-01
  %251 = fmul double %250, 5.000000e-01
  %252 = fmul double %251, 5.000000e-01
  %253 = fmul double %252, 5.000000e-01
  %254 = fmul double %253, 5.000000e-01
  %255 = fmul double %254, 5.000000e-01
  %256 = fmul double %255, 5.000000e-01
  %257 = add nsw i32 %247, 8
  %258 = icmp eq i32 %257, 0
  br i1 %258, label %259, label %246, !llvm.loop !10022

259:                                              ; preds = %226, %246, %209, %184, %181
  %260 = phi double [ %183, %181 ], [ %186, %184 ], [ %210, %209 ], [ %227, %226 ], [ %256, %246 ]
  %261 = icmp ult i32 %174, 32768
  %262 = fneg double %260
  %263 = select i1 %261, double %260, double %262
  br label %303

264:                                              ; preds = %164
  %265 = bitcast i32 %174 to float
  %266 = fpext float %265 to double
  br label %303

267:                                              ; preds = %164, %164
  br label %269

268:                                              ; preds = %164
  br label %269

269:                                              ; preds = %268, %267, %164, %164
  %270 = phi i32 [ 32, %268 ], [ 16, %267 ], [ 8, %164 ], [ 8, %164 ]
  %271 = zext i32 %174 to i64
  switch i64 %118, label %281 [
    i64 21, label %272
    i64 20, label %272
    i64 3, label %272
    i64 5, label %272
  ]

272:                                              ; preds = %269, %269, %269, %269
  %273 = add nsw i32 %270, -1
  %274 = shl nuw i32 1, %273
  %275 = and i32 %274, %174
  %276 = icmp eq i32 %275, 0
  %277 = zext nneg i32 %270 to i64
  %278 = shl nsw i64 -1, %277
  %279 = select i1 %276, i64 0, i64 %278
  %280 = add nsw i64 %279, %271
  br label %281

281:                                              ; preds = %272, %269
  %282 = phi i64 [ %280, %272 ], [ %271, %269 ]
  %283 = load i64, ptr addrspace(1) %147, align 8
  %284 = icmp eq i64 %283, 0
  br i1 %284, label %285, label %287

285:                                              ; preds = %281
  %286 = sitofp i64 %282 to double
  br label %303

287:                                              ; preds = %281
  switch i64 %118, label %288 [
    i64 21, label %289
    i64 20, label %289
    i64 3, label %289
  ]

288:                                              ; preds = %287
  br label %289

289:                                              ; preds = %288, %287, %287, %287
  %290 = phi i32 [ -1, %287 ], [ %149, %288 ], [ -1, %287 ], [ -1, %287 ]
  %291 = add nsw i32 %290, %270
  %292 = zext nneg i32 %291 to i64
  %293 = shl nsw i64 -1, %292
  %294 = xor i64 %293, -1
  %295 = sitofp i64 %282 to double
  %296 = sitofp i64 %294 to double
  %297 = fdiv double %295, %296
  %298 = fcmp oge double %297, -1.000000e+00
  %299 = select i1 %298, double %297, double -1.000000e+00
  br label %303

300:                                              ; preds = %150
  %301 = icmp eq i32 %151, 3
  %302 = select i1 %301, double 1.000000e+00, double 0.000000e+00
  br label %303

303:                                              ; preds = %300, %289, %285, %264, %259
  %304 = phi double [ %302, %300 ], [ %263, %259 ], [ %266, %264 ], [ %299, %289 ], [ %286, %285 ]
  %305 = insertelement <4 x double> %152, double %304, i32 %151
  %306 = add nuw nsw i32 %151, 1
  %307 = icmp eq i32 %306, 4
  br i1 %307, label %308, label %150, !llvm.loop !10029

308:                                              ; preds = %303, %110
  %309 = phi <4 x double> [ zeroinitializer, %110 ], [ %305, %303 ]
  %310 = extractelement <4 x double> %309, i64 0
  %311 = fcmp uno double %310, 0.000000e+00
  %312 = fptosi double %310 to i64
  %313 = trunc i64 %312 to i32
  %314 = select i1 %311, i32 0, i32 %313
  %315 = insertelement <4 x i32> poison, i32 %314, i64 0
  %316 = extractelement <4 x double> %309, i64 1
  %317 = fcmp uno double %316, 0.000000e+00
  %318 = fptosi double %316 to i64
  %319 = trunc i64 %318 to i32
  %320 = select i1 %317, i32 0, i32 %319
  %321 = insertelement <4 x i32> %315, i32 %320, i64 1
  %322 = extractelement <4 x double> %309, i64 2
  %323 = fcmp uno double %322, 0.000000e+00
  %324 = fptosi double %322 to i64
  %325 = trunc i64 %324 to i32
  %326 = select i1 %323, i32 0, i32 %325
  %327 = insertelement <4 x i32> %321, i32 %326, i64 2
  %328 = extractelement <4 x double> %309, i64 3
  %329 = fcmp uno double %328, 0.000000e+00
  %330 = fptosi double %328 to i64
  %331 = trunc i64 %330 to i32
  %332 = select i1 %329, i32 0, i32 %331
  %333 = insertelement <4 x i32> %327, i32 %332, i64 3
  ret <4 x i32> %333
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fabs.f64(double)

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.floor.f64(double)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.smax.i64(i64, i64)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.smin.i64(i64, i64)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)



!10000 = !{i32 1, !"wchar_size", i32 4}
!10001 = !{i32 7, !"frame-pointer", i32 2}
!10002 = !{!"Ubuntu clang version 18.1.3 (1ubuntu1)"}
!10003 = !{!10004, !10004, i64 0}
!10004 = !{!"long long", !10005, i64 0}
!10005 = !{!"omnipotent char", !10006, i64 0}
!10006 = !{!"Simple C/C++ TBAA"}
!10007 = !{!10008, !10009, i64 0}
!10008 = !{!"q", !10009, i64 0, !10005, i64 4, !10009, i64 16, !10009, i64 20}
!10009 = !{!"int", !10005, i64 0}
!10010 = !{!10009, !10009, i64 0}
!10011 = !{!10008, !10009, i64 16}
!10012 = !{!10008, !10009, i64 20}
!10013 = !{!10014, !10014, i64 0}
!10014 = !{!"double", !10005, i64 0}
!10015 = !{!10005, !10005, i64 0}
!10016 = distinct !{!10016, !10017}
!10017 = !{!"llvm.loop.mustprogress"}
!10018 = distinct !{!10018, !10019}
!10019 = !{!"llvm.loop.unroll.disable"}
!10020 = distinct !{!10020, !10019}
!10021 = distinct !{!10021, !10017}
!10022 = distinct !{!10022, !10017}
!10023 = distinct !{!10023, !10017, !10019}
!10024 = distinct !{!10024, !10017}
!10025 = distinct !{!10025, !10017}
!10026 = distinct !{!10026, !10017}
!10027 = distinct !{!10027, !10019}
!10028 = distinct !{!10028, !10019}
!10029 = distinct !{!10029, !10017, !10019}
!10030 = distinct !{!10030, !10019}
!10031 = distinct !{!10031, !10019}
!10032 = distinct !{!10032, !10017, !10019}
!10033 = !{!10034, !10014, i64 16}
!10034 = !{!"sampling", !10009, i64 0, !10009, i64 4, !10009, i64 8, !10009, i64 12, !10014, i64 16, !10014, i64 24}
!10035 = !{!10034, !10014, i64 24}
!10036 = !{!10034, !10009, i64 12}
!10037 = !{i64 0, i64 12, !10015, i64 12, i64 4, !10010, i64 16, i64 4, !10010}
!10038 = !{i64 0, i64 8, !10015, i64 8, i64 4, !10010, i64 12, i64 4, !10010}
!10039 = !{i64 0, i64 4, !10015, i64 4, i64 4, !10010, i64 8, i64 4, !10010}
!10040 = !{i64 0, i64 4, !10010, i64 4, i64 4, !10010}
!10041 = !{i64 0, i64 4, !10010}
!10042 = distinct !{!10042, !10019}
!10043 = distinct !{!10043, !10019}
!10044 = distinct !{!10044, !10019}
!10045 = distinct !{!10045, !10019}
!10046 = distinct !{!10046, !10017, !10019}
!10047 = distinct !{!10047, !10019}
!10048 = distinct !{!10048, !10019}
!10049 = distinct !{!10049, !10017, !10019}
!10050 = distinct !{!10050, !10017}
!10051 = distinct !{!10051, !10017}
!10052 = distinct !{!10052, !10017, !10053, !10054}
!10053 = !{!"llvm.loop.isvectorized", i32 1}
!10054 = !{!"llvm.loop.unroll.runtime.disable"}
!10055 = distinct !{!10055, !10017}
!10056 = distinct !{!10056, !10017, !10054, !10053}
!10057 = distinct !{!10057, !10017}
!10058 = distinct !{!10058, !10019}
!10059 = distinct !{!10059, !10019}
!10060 = distinct !{!10060, !10019}
!10061 = distinct !{!10061, !10019}
!10062 = distinct !{!10062, !10019}
!10063 = distinct !{!10063, !10019}
!10064 = distinct !{!10064, !10019}
!10065 = distinct !{!10065, !10019}
!10066 = distinct !{!10066, !10019}
!10067 = distinct !{!10067, !10019}
!10068 = distinct !{!10068, !10019}
!10069 = distinct !{!10069, !10019}
!10070 = distinct !{!10070, !10019}
!10071 = distinct !{!10071, !10019}
!10072 = distinct !{!10072, !10019}
!10073 = distinct !{!10073, !10019}
!10074 = distinct !{!10074, !10019}
!10075 = distinct !{!10075, !10019}
!10076 = distinct !{!10076, !10019}
!10077 = distinct !{!10077, !10019}
!10078 = distinct !{!10078, !10019}
!10079 = distinct !{!10079, !10019}
!10080 = distinct !{!10080, !10019}
!10081 = distinct !{!10081, !10019}
!10082 = distinct !{!10082, !10019}
!10083 = distinct !{!10083, !10019}
!10084 = distinct !{!10084, !10019}
!10085 = distinct !{!10085, !10019}
!10086 = distinct !{!10086, !10019}
!10087 = distinct !{!10087, !10019}
!10088 = distinct !{!10088, !10019}
!10089 = distinct !{!10089, !10019}
!10090 = distinct !{!10090, !10019}
!10091 = distinct !{!10091, !10019}
!10092 = distinct !{!10092, !10019}
!10093 = distinct !{!10093, !10019}

; from zluda_ptx_impl_agx_cvt.c

@__zluda_agx_tmem_map = internal addrspace(1) global [4 x i32] zeroinitializer, align 4
@__zluda_agx_tmem_owner = internal addrspace(1) global [4 x i32] zeroinitializer, align 4
@__zluda_agx_tmem_pool = internal addrspace(1) global [262144 x i32] zeroinitializer, align 4

; Function Attrs: nofree norecurse nosync nounwind memory(none)
define dso_local zeroext i16 @__zluda_ptx_impl_agx_cvt_narrow(i32 noundef %0, i32 noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef %4, i32 noundef %5) local_unnamed_addr #3 {
  switch i32 %2, label %79 [
    i32 0, label %7
    i32 1, label %43
  ]

7:                                                ; preds = %6
  %8 = lshr i32 %3, 23
  %9 = and i32 %3, 8388607
  %10 = trunc i32 %8 to i8
  switch i8 %10, label %16 [
    i8 -1, label %11
    i8 0, label %20
  ]

11:                                               ; preds = %7
  %12 = icmp ne i32 %9, 0
  %13 = zext i1 %12 to i32
  %14 = icmp eq i32 %9, 0
  %15 = zext i1 %14 to i32
  br label %20

16:                                               ; preds = %7
  %17 = and i32 %8, 255
  %18 = or disjoint i32 %9, 8388608
  %19 = add nsw i32 %17, -150
  br label %20

20:                                               ; preds = %7, %11, %16
  %21 = phi i32 [ 0, %11 ], [ %19, %16 ], [ -149, %7 ]
  %22 = phi i32 [ 0, %11 ], [ %18, %16 ], [ %9, %7 ]
  %23 = phi i32 [ %15, %11 ], [ 0, %16 ], [ 0, %7 ]
  %24 = phi i32 [ %13, %11 ], [ 0, %16 ], [ 0, %7 ]
  %25 = lshr i32 %4, 23
  %26 = and i32 %4, 8388607
  %27 = trunc i32 %25 to i8
  switch i8 %27, label %33 [
    i8 -1, label %28
    i8 0, label %37
  ]

28:                                               ; preds = %20
  %29 = icmp ne i32 %26, 0
  %30 = zext i1 %29 to i32
  %31 = icmp eq i32 %26, 0
  %32 = zext i1 %31 to i32
  br label %37

33:                                               ; preds = %20
  %34 = and i32 %25, 255
  %35 = or disjoint i32 %26, 8388608
  %36 = add nsw i32 %34, -150
  br label %37

37:                                               ; preds = %20, %28, %33
  %38 = phi i32 [ 0, %28 ], [ %36, %33 ], [ -149, %20 ]
  %39 = phi i32 [ 0, %28 ], [ %35, %33 ], [ %26, %20 ]
  %40 = phi i32 [ %32, %28 ], [ 0, %33 ], [ 0, %20 ]
  %41 = phi i32 [ %30, %28 ], [ 0, %33 ], [ 0, %20 ]
  %42 = lshr i32 %4, 31
  br label %116

43:                                               ; preds = %6
  %44 = lshr i32 %3, 16
  %45 = lshr i32 %3, 26
  %46 = and i32 %45, 31
  %47 = and i32 %44, 1023
  switch i32 %46, label %53 [
    i32 31, label %48
    i32 0, label %56
  ]

48:                                               ; preds = %43
  %49 = icmp ne i32 %47, 0
  %50 = zext i1 %49 to i32
  %51 = icmp eq i32 %47, 0
  %52 = zext i1 %51 to i32
  br label %56

53:                                               ; preds = %43
  %54 = or disjoint i32 %47, 1024
  %55 = add nsw i32 %46, -25
  br label %56

56:                                               ; preds = %43, %48, %53
  %57 = phi i32 [ 0, %48 ], [ %55, %53 ], [ -24, %43 ]
  %58 = phi i32 [ 0, %48 ], [ %54, %53 ], [ %47, %43 ]
  %59 = phi i32 [ %52, %48 ], [ 0, %53 ], [ %46, %43 ]
  %60 = phi i32 [ %50, %48 ], [ 0, %53 ], [ %46, %43 ]
  %61 = lshr i32 %3, 10
  %62 = and i32 %61, 31
  %63 = and i32 %3, 1023
  switch i32 %62, label %69 [
    i32 31, label %64
    i32 0, label %72
  ]

64:                                               ; preds = %56
  %65 = icmp ne i32 %63, 0
  %66 = zext i1 %65 to i32
  %67 = icmp eq i32 %63, 0
  %68 = zext i1 %67 to i32
  br label %72

69:                                               ; preds = %56
  %70 = or disjoint i32 %63, 1024
  %71 = add nsw i32 %62, -25
  br label %72

72:                                               ; preds = %56, %64, %69
  %73 = phi i32 [ 0, %64 ], [ %71, %69 ], [ -24, %56 ]
  %74 = phi i32 [ 0, %64 ], [ %70, %69 ], [ %63, %56 ]
  %75 = phi i32 [ %68, %64 ], [ 0, %69 ], [ %62, %56 ]
  %76 = phi i32 [ %66, %64 ], [ 0, %69 ], [ %62, %56 ]
  %77 = lshr i32 %3, 15
  %78 = and i32 %77, 1
  br label %116

79:                                               ; preds = %6
  %80 = lshr i32 %3, 23
  %81 = and i32 %3, 8323072
  %82 = trunc i32 %80 to i8
  switch i8 %82, label %88 [
    i8 -1, label %83
    i8 0, label %92
  ]

83:                                               ; preds = %79
  %84 = icmp ne i32 %81, 0
  %85 = zext i1 %84 to i32
  %86 = icmp eq i32 %81, 0
  %87 = zext i1 %86 to i32
  br label %92

88:                                               ; preds = %79
  %89 = and i32 %80, 255
  %90 = or disjoint i32 %81, 8388608
  %91 = add nsw i32 %89, -150
  br label %92

92:                                               ; preds = %79, %83, %88
  %93 = phi i32 [ 0, %83 ], [ %91, %88 ], [ -149, %79 ]
  %94 = phi i32 [ 0, %83 ], [ %90, %88 ], [ %81, %79 ]
  %95 = phi i32 [ %87, %83 ], [ 0, %88 ], [ 0, %79 ]
  %96 = phi i32 [ %85, %83 ], [ 0, %88 ], [ 0, %79 ]
  %97 = shl i32 %3, 16
  %98 = lshr i32 %97, 23
  %99 = and i32 %97, 8323072
  %100 = trunc i32 %98 to i8
  switch i8 %100, label %106 [
    i8 -1, label %101
    i8 0, label %110
  ]

101:                                              ; preds = %92
  %102 = icmp ne i32 %99, 0
  %103 = zext i1 %102 to i32
  %104 = icmp eq i32 %99, 0
  %105 = zext i1 %104 to i32
  br label %110

106:                                              ; preds = %92
  %107 = and i32 %98, 255
  %108 = or disjoint i32 %99, 8388608
  %109 = add nsw i32 %107, -150
  br label %110

110:                                              ; preds = %92, %101, %106
  %111 = phi i32 [ 0, %101 ], [ %109, %106 ], [ -149, %92 ]
  %112 = phi i32 [ 0, %101 ], [ %108, %106 ], [ %99, %92 ]
  %113 = phi i32 [ %105, %101 ], [ 0, %106 ], [ 0, %92 ]
  %114 = phi i32 [ %103, %101 ], [ 0, %106 ], [ 0, %92 ]
  %115 = lshr i32 %97, 31
  br label %116

116:                                              ; preds = %72, %110, %37
  %117 = phi i32 [ %114, %110 ], [ %76, %72 ], [ %41, %37 ]
  %118 = phi i32 [ %115, %110 ], [ %78, %72 ], [ %42, %37 ]
  %119 = phi i32 [ %113, %110 ], [ %75, %72 ], [ %40, %37 ]
  %120 = phi i32 [ %112, %110 ], [ %74, %72 ], [ %39, %37 ]
  %121 = phi i32 [ %111, %110 ], [ %73, %72 ], [ %38, %37 ]
  %122 = phi i32 [ %96, %110 ], [ %60, %72 ], [ %24, %37 ]
  %123 = phi i32 [ %95, %110 ], [ %59, %72 ], [ %23, %37 ]
  %124 = phi i32 [ %94, %110 ], [ %58, %72 ], [ %22, %37 ]
  %125 = phi i32 [ %93, %110 ], [ %57, %72 ], [ %21, %37 ]
  %126 = phi i32 [ -14, %110 ], [ -14, %72 ], [ -126, %37 ]
  %127 = zext nneg i32 %124 to i64
  %128 = lshr i32 %3, 31
  %129 = zext nneg i32 %120 to i64
  %130 = lshr i32 %1, 2
  %131 = and i32 %130, 8
  %132 = lshr i32 %5, %131
  %133 = and i32 %132, 255
  %134 = and i32 %5, 255
  %135 = and i32 %1, 16
  %136 = icmp ne i32 %135, 0
  %137 = icmp eq i32 %123, 0
  %138 = or i32 %123, %122
  %139 = icmp eq i32 %138, 0
  %140 = and i1 %136, %139
  %141 = icmp ne i32 %124, 0
  %142 = select i1 %140, i1 %141, i1 false
  br i1 %142, label %143, label %154

143:                                              ; preds = %116, %143
  %144 = phi i32 [ %147, %143 ], [ 0, %116 ]
  %145 = phi i64 [ %146, %143 ], [ %127, %116 ]
  %146 = lshr i64 %145, 1
  %147 = add nuw nsw i32 %144, 1
  %148 = icmp ult i64 %145, 2
  br i1 %148, label %149, label %143, !llvm.loop !20003

149:                                              ; preds = %143
  %150 = add nsw i32 %147, %125
  %151 = icmp sgt i32 %150, %126
  %152 = select i1 %151, i32 %128, i32 0
  %153 = select i1 %151, i64 %127, i64 0
  br label %154

154:                                              ; preds = %149, %116
  %155 = phi i32 [ %128, %116 ], [ %152, %149 ]
  %156 = phi i64 [ %127, %116 ], [ %153, %149 ]
  %157 = icmp ne i32 %133, 255
  %158 = sub nsw i32 %125, %133
  %159 = add nsw i32 %158, 127
  %160 = and i32 %1, 3
  %161 = and i32 %1, 4
  %162 = icmp ne i32 %161, 0
  %163 = icmp eq i32 %0, 6
  %164 = icmp eq i32 %122, 0
  %165 = and i1 %157, %164
  br i1 %163, label %166, label %224

166:                                              ; preds = %154
  br i1 %165, label %167, label %361

167:                                              ; preds = %166
  %168 = icmp ne i32 %155, 0
  %169 = select i1 %162, i1 %168, i1 false
  br i1 %169, label %361, label %170

170:                                              ; preds = %167
  br i1 %137, label %173, label %171

171:                                              ; preds = %170
  %172 = select i1 %168, i32 128, i32 127
  br label %361

173:                                              ; preds = %170
  %174 = icmp eq i64 %156, 0
  br i1 %174, label %213, label %175

175:                                              ; preds = %173
  %176 = add nsw i32 %158, 133
  %177 = icmp sgt i32 %158, -134
  br i1 %177, label %178, label %183

178:                                              ; preds = %175
  %179 = icmp ugt i32 %176, 40
  %180 = zext nneg i32 %176 to i64
  %181 = shl i64 %156, %180
  %182 = select i1 %179, i64 4611686018427387904, i64 %181
  br label %213

183:                                              ; preds = %175
  %184 = sub nuw nsw i32 -133, %158
  %185 = icmp ult i32 %176, -63
  %186 = zext nneg i32 %184 to i64
  %187 = lshr i64 %156, %186
  %188 = select i1 %185, i64 0, i64 %187
  %189 = shl nsw i64 -1, %186
  %190 = xor i64 %189, -1
  %191 = select i1 %185, i64 -1, i64 %190
  %192 = and i64 %156, %191
  %193 = icmp eq i64 %192, 0
  %194 = icmp eq i32 %160, 1
  %195 = or i1 %194, %193
  br i1 %195, label %213, label %196

196:                                              ; preds = %183
  %197 = icmp eq i32 %160, 2
  br i1 %197, label %198, label %200

198:                                              ; preds = %196
  %199 = add nuw nsw i64 %188, 1
  br label %213

200:                                              ; preds = %196
  br i1 %185, label %213, label %201

201:                                              ; preds = %200
  %202 = sub nuw nsw i32 -134, %158
  %203 = zext nneg i32 %202 to i64
  %204 = shl nuw nsw i64 1, %203
  %205 = icmp ugt i64 %192, %204
  %206 = icmp eq i64 %192, %204
  %207 = and i64 %187, 1
  %208 = icmp ne i64 %207, 0
  %209 = select i1 %206, i1 %208, i1 false
  %210 = select i1 %205, i1 true, i1 %209
  %211 = zext i1 %210 to i64
  %212 = add nuw nsw i64 %187, %211
  br label %213

213:                                              ; preds = %201, %200, %198, %183, %178, %173
  %214 = phi i64 [ 0, %173 ], [ %182, %178 ], [ %199, %198 ], [ %212, %201 ], [ %188, %183 ], [ 0, %200 ]
  %215 = icmp eq i32 %155, 0
  br i1 %215, label %221, label %216

216:                                              ; preds = %213
  %217 = tail call i64 @llvm.umin.i64(i64 %214, i64 128)
  %218 = trunc i64 %217 to i32
  %219 = sub nsw i32 0, %218
  %220 = and i32 %219, 255
  br label %361

221:                                              ; preds = %213
  %222 = tail call i64 @llvm.umin.i64(i64 %214, i64 127)
  %223 = trunc i64 %222 to i32
  br label %361

224:                                              ; preds = %154
  br i1 %165, label %232, label %225

225:                                              ; preds = %224
  %226 = icmp ult i32 %0, 2
  %227 = icmp eq i32 %0, 4
  %228 = select i1 %227, i32 7, i32 31
  %229 = icmp eq i32 %0, 5
  %230 = select i1 %229, i32 255, i32 %228
  %231 = select i1 %226, i32 127, i32 %230
  br label %361

232:                                              ; preds = %224
  %233 = icmp eq i32 %0, 5
  %234 = or i1 %233, %162
  %235 = icmp ne i32 %155, 0
  %236 = select i1 %234, i1 %235, i1 false
  br i1 %236, label %361, label %237

237:                                              ; preds = %232
  %238 = icmp ne i32 %0, 5
  %239 = and i1 %238, %235
  %240 = and i32 %0, -2
  %241 = icmp eq i32 %240, 2
  %242 = icmp eq i32 %0, 4
  %243 = select i1 %242, i32 8, i32 128
  %244 = select i1 %241, i32 32, i32 %243
  %245 = select i1 %239, i32 %244, i32 0
  switch i32 %0, label %246 [
    i32 0, label %250
    i32 1, label %248
    i32 4, label %249
  ]

246:                                              ; preds = %237
  %247 = select i1 %233, i64 254, i64 31
  br label %250

248:                                              ; preds = %237
  br label %250

249:                                              ; preds = %237
  br label %250

250:                                              ; preds = %249, %248, %246, %237
  %251 = phi i64 [ 126, %237 ], [ %247, %246 ], [ 123, %248 ], [ 7, %249 ]
  br i1 %137, label %252, label %351

252:                                              ; preds = %250
  %253 = icmp eq i64 %156, 0
  br i1 %253, label %337, label %254

254:                                              ; preds = %252
  switch i32 %0, label %255 [
    i32 0, label %259
    i32 4, label %258
    i32 2, label %258
  ]

255:                                              ; preds = %254
  %256 = icmp eq i32 %0, 3
  %257 = select i1 %256, i32 2, i32 4
  br label %259

258:                                              ; preds = %254, %254
  br label %259

259:                                              ; preds = %258, %255, %254
  %260 = phi i32 [ 3, %254 ], [ %257, %255 ], [ 1, %258 ]
  %261 = and i32 %0, -3
  %262 = shl nsw i32 -1, %260
  br label %263

263:                                              ; preds = %263, %259
  %264 = phi i32 [ %267, %263 ], [ 0, %259 ]
  %265 = phi i64 [ %266, %263 ], [ %156, %259 ]
  %266 = lshr i64 %265, 1
  %267 = add nuw nsw i32 %264, 1
  %268 = icmp ult i64 %265, 2
  br i1 %268, label %269, label %263, !llvm.loop !20003

269:                                              ; preds = %263
  %270 = select i1 %242, i32 1, i32 3
  %271 = xor i32 %262, -1
  %272 = icmp eq i32 %261, 1
  %273 = select i1 %272, i32 2, i32 %270
  %274 = add nsw i32 %262, 2
  %275 = add nsw i32 %267, %159
  %276 = add i32 %264, %159
  %277 = icmp sgt i32 %275, %274
  %278 = select i1 %277, i32 %276, i32 %274
  %279 = icmp sgt i32 %278, 200
  br i1 %279, label %337, label %280

280:                                              ; preds = %269
  %281 = sub i32 %273, %278
  %282 = add i32 %281, %159
  %283 = icmp sgt i32 %282, -1
  br i1 %283, label %284, label %289

284:                                              ; preds = %280
  %285 = icmp ugt i32 %282, 40
  %286 = zext nneg i32 %282 to i64
  %287 = shl i64 %156, %286
  %288 = select i1 %285, i64 4611686018427387904, i64 %287
  br label %319

289:                                              ; preds = %280
  %290 = sub nsw i32 0, %282
  %291 = icmp ult i32 %282, -63
  %292 = zext nneg i32 %290 to i64
  %293 = lshr i64 %156, %292
  %294 = select i1 %291, i64 0, i64 %293
  %295 = shl nsw i64 -1, %292
  %296 = xor i64 %295, -1
  %297 = select i1 %291, i64 -1, i64 %296
  %298 = and i64 %297, %156
  %299 = icmp eq i64 %298, 0
  %300 = icmp eq i32 %160, 1
  %301 = or i1 %300, %299
  br i1 %301, label %319, label %302

302:                                              ; preds = %289
  %303 = icmp eq i32 %160, 2
  br i1 %303, label %304, label %306

304:                                              ; preds = %302
  %305 = add nuw nsw i64 %294, 1
  br label %319

306:                                              ; preds = %302
  br i1 %291, label %319, label %307

307:                                              ; preds = %306
  %308 = xor i32 %282, -1
  %309 = zext nneg i32 %308 to i64
  %310 = shl nuw nsw i64 1, %309
  %311 = icmp ugt i64 %298, %310
  %312 = icmp eq i64 %298, %310
  %313 = and i64 %293, 1
  %314 = icmp ne i64 %313, 0
  %315 = select i1 %312, i1 %314, i1 false
  %316 = select i1 %311, i1 true, i1 %315
  %317 = zext i1 %316 to i64
  %318 = add nuw nsw i64 %293, %317
  br label %319

319:                                              ; preds = %307, %306, %304, %289, %284
  %320 = phi i64 [ %288, %284 ], [ %305, %304 ], [ %318, %307 ], [ %294, %289 ], [ 0, %306 ]
  %321 = zext nneg i32 %273 to i64
  %322 = shl nuw nsw i64 2, %321
  %323 = icmp ult i64 %320, %322
  br i1 %323, label %328, label %324

324:                                              ; preds = %319
  %325 = sub i32 %278, %262
  %326 = sext i32 %325 to i64
  %327 = shl nsw i64 %326, %321
  br label %337

328:                                              ; preds = %319
  %329 = lshr i64 %320, %321
  %330 = icmp eq i64 %329, 0
  %331 = add nsw i32 %278, %271
  %332 = sext i32 %331 to i64
  %333 = add nsw i64 %332, -1
  %334 = shl nsw i64 %333, %321
  %335 = select i1 %330, i64 0, i64 %334
  %336 = add i64 %335, %320
  br label %337

337:                                              ; preds = %328, %324, %269, %252
  %338 = phi i64 [ 0, %252 ], [ 1099511627776, %269 ], [ %327, %324 ], [ %336, %328 ]
  switch i32 %0, label %339 [
    i32 0, label %343
    i32 1, label %341
    i32 4, label %342
  ]

339:                                              ; preds = %337
  %340 = select i1 %233, i64 254, i64 31
  br label %343

341:                                              ; preds = %337
  br label %343

342:                                              ; preds = %337
  br label %343

343:                                              ; preds = %342, %341, %339, %337
  %344 = phi i64 [ 126, %337 ], [ %340, %339 ], [ 123, %341 ], [ 7, %342 ]
  %345 = icmp sgt i64 %338, %344
  br i1 %345, label %346, label %351

346:                                              ; preds = %343
  switch i32 %0, label %347 [
    i32 0, label %351
    i32 1, label %349
    i32 4, label %350
  ]

347:                                              ; preds = %346
  %348 = select i1 %233, i64 254, i64 31
  br label %351

349:                                              ; preds = %346
  br label %351

350:                                              ; preds = %346
  br label %351

351:                                              ; preds = %350, %349, %347, %346, %343, %250
  %352 = phi i64 [ %251, %250 ], [ %338, %343 ], [ 126, %346 ], [ %348, %347 ], [ 123, %349 ], [ 7, %350 ]
  %353 = trunc i64 %352 to i32
  %354 = or i32 %245, %353
  %355 = and i32 %1, 8
  %356 = icmp ne i32 %355, 0
  %357 = and i1 %238, %356
  %358 = icmp eq i32 %354, %244
  %359 = select i1 %357, i1 %358, i1 false
  %360 = select i1 %359, i32 0, i32 %354
  br label %361

361:                                              ; preds = %351, %225, %166, %167, %171, %216, %221, %232
  %362 = phi i32 [ %172, %171 ], [ 127, %166 ], [ 0, %167 ], [ %220, %216 ], [ %223, %221 ], [ 0, %232 ], [ %231, %225 ], [ %360, %351 ]
  %363 = icmp eq i32 %119, 0
  %364 = or i32 %119, %117
  %365 = icmp eq i32 %364, 0
  %366 = and i1 %136, %365
  %367 = icmp ne i32 %120, 0
  %368 = select i1 %366, i1 %367, i1 false
  br i1 %368, label %369, label %380

369:                                              ; preds = %361, %369
  %370 = phi i32 [ %373, %369 ], [ 0, %361 ]
  %371 = phi i64 [ %372, %369 ], [ %129, %361 ]
  %372 = lshr i64 %371, 1
  %373 = add nuw nsw i32 %370, 1
  %374 = icmp ult i64 %371, 2
  br i1 %374, label %375, label %369, !llvm.loop !20003

375:                                              ; preds = %369
  %376 = add nsw i32 %373, %121
  %377 = icmp sgt i32 %376, %126
  %378 = select i1 %377, i32 %118, i32 0
  %379 = select i1 %377, i64 %129, i64 0
  br label %380

380:                                              ; preds = %375, %361
  %381 = phi i32 [ %118, %361 ], [ %378, %375 ]
  %382 = phi i64 [ %129, %361 ], [ %379, %375 ]
  %383 = icmp ne i32 %134, 255
  %384 = sub nsw i32 %121, %134
  %385 = add nsw i32 %384, 127
  %386 = icmp eq i32 %117, 0
  %387 = and i1 %383, %386
  br i1 %163, label %388, label %446

388:                                              ; preds = %380
  br i1 %387, label %389, label %583

389:                                              ; preds = %388
  %390 = icmp ne i32 %381, 0
  %391 = select i1 %162, i1 %390, i1 false
  br i1 %391, label %583, label %392

392:                                              ; preds = %389
  br i1 %363, label %395, label %393

393:                                              ; preds = %392
  %394 = select i1 %390, i32 128, i32 127
  br label %583

395:                                              ; preds = %392
  %396 = icmp eq i64 %382, 0
  br i1 %396, label %435, label %397

397:                                              ; preds = %395
  %398 = add nsw i32 %384, 133
  %399 = icmp sgt i32 %384, -134
  br i1 %399, label %400, label %405

400:                                              ; preds = %397
  %401 = icmp ugt i32 %398, 40
  %402 = zext nneg i32 %398 to i64
  %403 = shl i64 %382, %402
  %404 = select i1 %401, i64 4611686018427387904, i64 %403
  br label %435

405:                                              ; preds = %397
  %406 = sub nuw nsw i32 -133, %384
  %407 = icmp ult i32 %398, -63
  %408 = zext nneg i32 %406 to i64
  %409 = lshr i64 %382, %408
  %410 = select i1 %407, i64 0, i64 %409
  %411 = shl nsw i64 -1, %408
  %412 = xor i64 %411, -1
  %413 = select i1 %407, i64 -1, i64 %412
  %414 = and i64 %382, %413
  %415 = icmp eq i64 %414, 0
  %416 = icmp eq i32 %160, 1
  %417 = or i1 %416, %415
  br i1 %417, label %435, label %418

418:                                              ; preds = %405
  %419 = icmp eq i32 %160, 2
  br i1 %419, label %420, label %422

420:                                              ; preds = %418
  %421 = add nuw nsw i64 %410, 1
  br label %435

422:                                              ; preds = %418
  br i1 %407, label %435, label %423

423:                                              ; preds = %422
  %424 = sub nuw nsw i32 -134, %384
  %425 = zext nneg i32 %424 to i64
  %426 = shl nuw nsw i64 1, %425
  %427 = icmp ugt i64 %414, %426
  %428 = icmp eq i64 %414, %426
  %429 = and i64 %409, 1
  %430 = icmp ne i64 %429, 0
  %431 = select i1 %428, i1 %430, i1 false
  %432 = select i1 %427, i1 true, i1 %431
  %433 = zext i1 %432 to i64
  %434 = add nuw nsw i64 %409, %433
  br label %435

435:                                              ; preds = %423, %422, %420, %405, %400, %395
  %436 = phi i64 [ 0, %395 ], [ %404, %400 ], [ %421, %420 ], [ %434, %423 ], [ %410, %405 ], [ 0, %422 ]
  %437 = icmp eq i32 %381, 0
  br i1 %437, label %443, label %438

438:                                              ; preds = %435
  %439 = tail call i64 @llvm.umin.i64(i64 %436, i64 128)
  %440 = trunc i64 %439 to i32
  %441 = sub nsw i32 0, %440
  %442 = and i32 %441, 255
  br label %583

443:                                              ; preds = %435
  %444 = tail call i64 @llvm.umin.i64(i64 %436, i64 127)
  %445 = trunc i64 %444 to i32
  br label %583

446:                                              ; preds = %380
  br i1 %387, label %454, label %447

447:                                              ; preds = %446
  %448 = icmp ult i32 %0, 2
  %449 = icmp eq i32 %0, 4
  %450 = select i1 %449, i32 7, i32 31
  %451 = icmp eq i32 %0, 5
  %452 = select i1 %451, i32 255, i32 %450
  %453 = select i1 %448, i32 127, i32 %452
  br label %583

454:                                              ; preds = %446
  %455 = icmp eq i32 %0, 5
  %456 = or i1 %455, %162
  %457 = icmp ne i32 %381, 0
  %458 = select i1 %456, i1 %457, i1 false
  br i1 %458, label %583, label %459

459:                                              ; preds = %454
  %460 = icmp ne i32 %0, 5
  %461 = and i1 %460, %457
  %462 = and i32 %0, -2
  %463 = icmp eq i32 %462, 2
  %464 = icmp eq i32 %0, 4
  %465 = select i1 %464, i32 8, i32 128
  %466 = select i1 %463, i32 32, i32 %465
  %467 = select i1 %461, i32 %466, i32 0
  switch i32 %0, label %468 [
    i32 0, label %472
    i32 1, label %470
    i32 4, label %471
  ]

468:                                              ; preds = %459
  %469 = select i1 %455, i64 254, i64 31
  br label %472

470:                                              ; preds = %459
  br label %472

471:                                              ; preds = %459
  br label %472

472:                                              ; preds = %471, %470, %468, %459
  %473 = phi i64 [ 126, %459 ], [ %469, %468 ], [ 123, %470 ], [ 7, %471 ]
  br i1 %363, label %474, label %573

474:                                              ; preds = %472
  %475 = icmp eq i64 %382, 0
  br i1 %475, label %559, label %476

476:                                              ; preds = %474
  switch i32 %0, label %477 [
    i32 0, label %481
    i32 4, label %480
    i32 2, label %480
  ]

477:                                              ; preds = %476
  %478 = icmp eq i32 %0, 3
  %479 = select i1 %478, i32 2, i32 4
  br label %481

480:                                              ; preds = %476, %476
  br label %481

481:                                              ; preds = %480, %477, %476
  %482 = phi i32 [ 3, %476 ], [ %479, %477 ], [ 1, %480 ]
  %483 = and i32 %0, -3
  %484 = shl nsw i32 -1, %482
  br label %485

485:                                              ; preds = %485, %481
  %486 = phi i32 [ %489, %485 ], [ 0, %481 ]
  %487 = phi i64 [ %488, %485 ], [ %382, %481 ]
  %488 = lshr i64 %487, 1
  %489 = add nuw nsw i32 %486, 1
  %490 = icmp ult i64 %487, 2
  br i1 %490, label %491, label %485, !llvm.loop !20003

491:                                              ; preds = %485
  %492 = select i1 %464, i32 1, i32 3
  %493 = xor i32 %484, -1
  %494 = icmp eq i32 %483, 1
  %495 = select i1 %494, i32 2, i32 %492
  %496 = add nsw i32 %484, 2
  %497 = add nsw i32 %489, %385
  %498 = add i32 %486, %385
  %499 = icmp sgt i32 %497, %496
  %500 = select i1 %499, i32 %498, i32 %496
  %501 = icmp sgt i32 %500, 200
  br i1 %501, label %559, label %502

502:                                              ; preds = %491
  %503 = sub i32 %495, %500
  %504 = add i32 %503, %385
  %505 = icmp sgt i32 %504, -1
  br i1 %505, label %506, label %511

506:                                              ; preds = %502
  %507 = icmp ugt i32 %504, 40
  %508 = zext nneg i32 %504 to i64
  %509 = shl i64 %382, %508
  %510 = select i1 %507, i64 4611686018427387904, i64 %509
  br label %541

511:                                              ; preds = %502
  %512 = sub nsw i32 0, %504
  %513 = icmp ult i32 %504, -63
  %514 = zext nneg i32 %512 to i64
  %515 = lshr i64 %382, %514
  %516 = select i1 %513, i64 0, i64 %515
  %517 = shl nsw i64 -1, %514
  %518 = xor i64 %517, -1
  %519 = select i1 %513, i64 -1, i64 %518
  %520 = and i64 %519, %382
  %521 = icmp eq i64 %520, 0
  %522 = icmp eq i32 %160, 1
  %523 = or i1 %522, %521
  br i1 %523, label %541, label %524

524:                                              ; preds = %511
  %525 = icmp eq i32 %160, 2
  br i1 %525, label %526, label %528

526:                                              ; preds = %524
  %527 = add nuw nsw i64 %516, 1
  br label %541

528:                                              ; preds = %524
  br i1 %513, label %541, label %529

529:                                              ; preds = %528
  %530 = xor i32 %504, -1
  %531 = zext nneg i32 %530 to i64
  %532 = shl nuw nsw i64 1, %531
  %533 = icmp ugt i64 %520, %532
  %534 = icmp eq i64 %520, %532
  %535 = and i64 %515, 1
  %536 = icmp ne i64 %535, 0
  %537 = select i1 %534, i1 %536, i1 false
  %538 = select i1 %533, i1 true, i1 %537
  %539 = zext i1 %538 to i64
  %540 = add nuw nsw i64 %515, %539
  br label %541

541:                                              ; preds = %529, %528, %526, %511, %506
  %542 = phi i64 [ %510, %506 ], [ %527, %526 ], [ %540, %529 ], [ %516, %511 ], [ 0, %528 ]
  %543 = zext nneg i32 %495 to i64
  %544 = shl nuw nsw i64 2, %543
  %545 = icmp ult i64 %542, %544
  br i1 %545, label %550, label %546

546:                                              ; preds = %541
  %547 = sub i32 %500, %484
  %548 = sext i32 %547 to i64
  %549 = shl nsw i64 %548, %543
  br label %559

550:                                              ; preds = %541
  %551 = lshr i64 %542, %543
  %552 = icmp eq i64 %551, 0
  %553 = add nsw i32 %500, %493
  %554 = sext i32 %553 to i64
  %555 = add nsw i64 %554, -1
  %556 = shl nsw i64 %555, %543
  %557 = select i1 %552, i64 0, i64 %556
  %558 = add i64 %557, %542
  br label %559

559:                                              ; preds = %550, %546, %491, %474
  %560 = phi i64 [ 0, %474 ], [ 1099511627776, %491 ], [ %549, %546 ], [ %558, %550 ]
  switch i32 %0, label %561 [
    i32 0, label %565
    i32 1, label %563
    i32 4, label %564
  ]

561:                                              ; preds = %559
  %562 = select i1 %455, i64 254, i64 31
  br label %565

563:                                              ; preds = %559
  br label %565

564:                                              ; preds = %559
  br label %565

565:                                              ; preds = %564, %563, %561, %559
  %566 = phi i64 [ 126, %559 ], [ %562, %561 ], [ 123, %563 ], [ 7, %564 ]
  %567 = icmp sgt i64 %560, %566
  br i1 %567, label %568, label %573

568:                                              ; preds = %565
  switch i32 %0, label %569 [
    i32 0, label %573
    i32 1, label %571
    i32 4, label %572
  ]

569:                                              ; preds = %568
  %570 = select i1 %455, i64 254, i64 31
  br label %573

571:                                              ; preds = %568
  br label %573

572:                                              ; preds = %568
  br label %573

573:                                              ; preds = %572, %571, %569, %568, %565, %472
  %574 = phi i64 [ %473, %472 ], [ %560, %565 ], [ 126, %568 ], [ %570, %569 ], [ 123, %571 ], [ 7, %572 ]
  %575 = trunc i64 %574 to i32
  %576 = or i32 %467, %575
  %577 = and i32 %1, 8
  %578 = icmp ne i32 %577, 0
  %579 = and i1 %460, %578
  %580 = icmp eq i32 %576, %466
  %581 = select i1 %579, i1 %580, i1 false
  %582 = select i1 %581, i32 0, i32 %576
  br label %583

583:                                              ; preds = %573, %447, %388, %389, %393, %438, %443, %454
  %584 = phi i32 [ %394, %393 ], [ 127, %388 ], [ 0, %389 ], [ %442, %438 ], [ %445, %443 ], [ 0, %454 ], [ %453, %447 ], [ %582, %573 ]
  %585 = icmp eq i32 %0, 4
  %586 = select i1 %585, i32 4, i32 8
  %587 = shl i32 %362, %586
  %588 = or i32 %584, %587
  %589 = trunc i32 %588 to i16
  ret i16 %589
}

; Function Attrs: nofree norecurse nosync nounwind memory(none)
define dso_local i32 @__zluda_ptx_impl_agx_cvt_widen(i32 noundef %0, i32 noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef %4) local_unnamed_addr #3 {
  %6 = icmp eq i32 %0, 4
  %7 = select i1 %6, i32 2, i32 3
  %8 = select i1 %6, i32 15, i32 255
  %9 = icmp eq i32 %0, 6
  %10 = and i32 %2, 32
  %11 = icmp eq i32 %10, 0
  %12 = lshr i32 %4, 8
  %13 = icmp eq i32 %1, 0
  %14 = select i1 %13, i32 5, i32 8
  %15 = select i1 %13, i32 10, i32 7
  %16 = shl nsw i32 -1, %14
  %17 = xor i32 %16, -1
  %18 = shl nuw nsw i32 %17, %15
  %19 = icmp eq i32 %0, 5
  %20 = and i32 %0, -2
  %21 = icmp eq i32 %20, 2
  %22 = select i1 %6, i32 3, i32 7
  %23 = select i1 %21, i32 5, i32 %22
  %24 = shl nsw i32 -1, %23
  %25 = xor i32 %24, -1
  %26 = icmp eq i32 %0, 0
  %27 = icmp eq i32 %0, 1
  %28 = and i32 %0, -3
  %29 = icmp eq i32 %28, 1
  %30 = select i1 %6, i32 1, i32 3
  %31 = select i1 %29, i32 2, i32 %30
  %32 = icmp eq i32 %0, 3
  %33 = select i1 %32, i32 2, i32 4
  %34 = shl nuw nsw i32 1, %31
  %35 = add nsw i32 %34, -1
  %36 = select i1 %13, i32 32256, i32 32704
  %37 = add nsw i32 %14, -1
  %38 = shl nsw i32 -1, %37
  %39 = xor i32 %38, -1
  %40 = add nuw nsw i32 %38, 2
  %41 = zext nneg i32 %15 to i64
  %42 = shl nuw nsw i64 2, %41
  %43 = zext nneg i32 %18 to i64
  %44 = select i1 %19, i32 -1, i32 %25
  br label %46

45:                                               ; preds = %181
  ret i32 %185

46:                                               ; preds = %5, %181
  %47 = phi i32 [ 0, %5 ], [ %185, %181 ]
  %48 = phi i1 [ true, %5 ], [ false, %181 ]
  %49 = phi i32 [ 0, %5 ], [ 1, %181 ]
  %50 = shl nuw nsw i32 %49, %7
  %51 = lshr i32 %3, %50
  %52 = and i32 %51, %8
  br i1 %9, label %53, label %59

53:                                               ; preds = %46
  %54 = shl nuw nsw i32 %52, 1
  %55 = and i32 %54, 256
  %56 = sub nsw i32 %52, %55
  %57 = lshr i32 %56, 31
  %58 = tail call i32 @llvm.abs.i32(i32 %56, i1 true)
  br label %92

59:                                               ; preds = %46
  %60 = and i32 %52, %44
  %61 = lshr i32 %52, %23
  %62 = and i32 %61, 1
  %63 = select i1 %19, i32 0, i32 %62
  %64 = icmp eq i32 %60, 127
  %65 = select i1 %26, i1 %64, i1 false
  br i1 %65, label %92, label %66

66:                                               ; preds = %59
  %67 = icmp ugt i32 %60, 124
  %68 = select i1 %27, i1 %67, i1 false
  %69 = icmp eq i32 %60, 255
  %70 = select i1 %19, i1 %69, i1 false
  %71 = select i1 %68, i1 true, i1 %70
  br i1 %71, label %92, label %72

72:                                               ; preds = %66
  %73 = icmp eq i32 %60, 124
  %74 = select i1 %27, i1 %73, i1 false
  br i1 %74, label %92, label %75

75:                                               ; preds = %72
  switch i32 %0, label %76 [
    i32 0, label %78
    i32 4, label %77
    i32 2, label %77
  ]

76:                                               ; preds = %75
  br label %78

77:                                               ; preds = %75, %75
  br label %78

78:                                               ; preds = %77, %76, %75
  %79 = phi i32 [ 3, %75 ], [ %33, %76 ], [ 1, %77 ]
  %80 = shl nsw i32 -1, %79
  %81 = lshr i32 %60, %31
  %82 = and i32 %60, %35
  %83 = icmp eq i32 %81, 0
  br i1 %83, label %84, label %87

84:                                               ; preds = %78
  %85 = sub nuw nsw i32 %80, %31
  %86 = add nuw nsw i32 %85, 2
  br label %92

87:                                               ; preds = %78
  %88 = or i32 %82, %34
  %89 = sub nsw i32 %81, %31
  %90 = add nsw i32 %89, 1
  %91 = add nsw i32 %90, %80
  br label %92

92:                                               ; preds = %53, %59, %66, %72, %84, %87
  %93 = phi i32 [ -6, %53 ], [ 0, %66 ], [ 0, %59 ], [ 0, %72 ], [ %86, %84 ], [ %91, %87 ]
  %94 = phi i32 [ %58, %53 ], [ 0, %66 ], [ 0, %59 ], [ 0, %72 ], [ %82, %84 ], [ %88, %87 ]
  %95 = phi i1 [ true, %53 ], [ true, %66 ], [ true, %59 ], [ false, %72 ], [ true, %84 ], [ true, %87 ]
  %96 = phi i1 [ true, %53 ], [ false, %66 ], [ false, %59 ], [ true, %72 ], [ true, %84 ], [ true, %87 ]
  %97 = phi i32 [ %57, %53 ], [ %63, %66 ], [ %63, %59 ], [ %63, %72 ], [ %63, %84 ], [ %63, %87 ]
  %98 = zext nneg i32 %94 to i64
  %99 = or i1 %11, %48
  %100 = select i1 %99, i32 %4, i32 %12
  %101 = and i32 %100, 255
  %102 = icmp ne i32 %101, 255
  %103 = add nsw i32 %101, -127
  %104 = add nsw i32 %103, %93
  %105 = and i1 %102, %96
  br i1 %105, label %106, label %181

106:                                              ; preds = %92
  %107 = icmp eq i32 %97, 0
  %108 = select i1 %107, i32 0, i32 32768
  br i1 %95, label %111, label %109

109:                                              ; preds = %106
  %110 = or i32 %108, %18
  br label %181

111:                                              ; preds = %106
  %112 = icmp eq i32 %94, 0
  br i1 %112, label %181, label %113

113:                                              ; preds = %111, %113
  %114 = phi i32 [ %117, %113 ], [ 0, %111 ]
  %115 = phi i64 [ %116, %113 ], [ %98, %111 ]
  %116 = lshr i64 %115, 1
  %117 = add nuw nsw i32 %114, 1
  %118 = icmp ult i64 %115, 2
  br i1 %118, label %119, label %113, !llvm.loop !20003

119:                                              ; preds = %113
  %120 = add nsw i32 %117, %104
  %121 = add i32 %114, %104
  %122 = icmp sgt i32 %120, %40
  %123 = select i1 %122, i32 %121, i32 %40
  %124 = icmp sgt i32 %123, 200
  br i1 %124, label %175, label %125

125:                                              ; preds = %119
  %126 = sub i32 %15, %123
  %127 = add i32 %126, %104
  %128 = icmp sgt i32 %127, -1
  br i1 %128, label %129, label %134

129:                                              ; preds = %125
  %130 = icmp ugt i32 %127, 40
  %131 = zext nneg i32 %127 to i64
  %132 = shl i64 %98, %131
  %133 = select i1 %130, i64 4611686018427387904, i64 %132
  br label %159

134:                                              ; preds = %125
  %135 = sub nsw i32 0, %127
  %136 = icmp ult i32 %127, -63
  %137 = zext nneg i32 %135 to i64
  %138 = lshr i64 %98, %137
  %139 = select i1 %136, i64 0, i64 %138
  %140 = shl nsw i64 -1, %137
  %141 = xor i64 %140, -1
  %142 = select i1 %136, i64 2147483647, i64 %141
  %143 = and i64 %142, %98
  %144 = icmp eq i64 %143, 0
  %145 = or i1 %136, %144
  %146 = select i1 %144, i64 %139, i64 0
  br i1 %145, label %159, label %147

147:                                              ; preds = %134
  %148 = xor i32 %127, -1
  %149 = zext nneg i32 %148 to i64
  %150 = shl nuw nsw i64 1, %149
  %151 = icmp ugt i64 %143, %150
  %152 = icmp eq i64 %143, %150
  %153 = and i64 %138, 1
  %154 = icmp ne i64 %153, 0
  %155 = select i1 %152, i1 %154, i1 false
  %156 = select i1 %151, i1 true, i1 %155
  %157 = zext i1 %156 to i64
  %158 = add nuw nsw i64 %138, %157
  br label %159

159:                                              ; preds = %147, %134, %129
  %160 = phi i64 [ %133, %129 ], [ %158, %147 ], [ %146, %134 ]
  %161 = icmp ult i64 %160, %42
  br i1 %161, label %166, label %162

162:                                              ; preds = %159
  %163 = sub i32 %123, %38
  %164 = sext i32 %163 to i64
  %165 = shl nsw i64 %164, %41
  br label %175

166:                                              ; preds = %159
  %167 = lshr i64 %160, %41
  %168 = icmp eq i64 %167, 0
  %169 = add nsw i32 %123, %39
  %170 = sext i32 %169 to i64
  %171 = add nsw i64 %170, -1
  %172 = shl nsw i64 %171, %41
  %173 = select i1 %168, i64 0, i64 %172
  %174 = add i64 %173, %160
  br label %175

175:                                              ; preds = %166, %162, %119
  %176 = phi i64 [ 1099511627776, %119 ], [ %165, %162 ], [ %174, %166 ]
  %177 = icmp slt i64 %176, %43
  %178 = trunc i64 %176 to i32
  %179 = select i1 %177, i32 %178, i32 %18
  %180 = or i32 %179, %108
  br label %181

181:                                              ; preds = %92, %109, %111, %175
  %182 = phi i32 [ %110, %109 ], [ %180, %175 ], [ %108, %111 ], [ %36, %92 ]
  %183 = shl nuw nsw i32 %49, 4
  %184 = shl i32 %182, %183
  %185 = or i32 %184, %47
  br i1 %48, label %46, label %45, !llvm.loop !20005
}

; Function Attrs: nofree norecurse nosync nounwind memory(none)
define dso_local i64 @__zluda_ptx_impl_agx_spc_iter(i32 noundef %0, i32 noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef %4) local_unnamed_addr #3 {
  %6 = zext i32 %4 to i64
  %7 = shl nuw i64 %6, 32
  %8 = zext i32 %3 to i64
  %9 = or disjoint i64 %7, %8
  %10 = lshr i32 %0, 2
  %11 = and i32 %10, 7
  %12 = shl i32 %1, 2
  %13 = udiv i32 64, %12
  %14 = icmp ugt i32 %12, 64
  br i1 %14, label %38, label %15

15:                                               ; preds = %5
  %16 = and i32 %0, 2
  %17 = shl nsw i32 -1, %1
  %18 = xor i32 %17, -1
  %19 = shl i32 %1, 1
  %20 = mul i32 %1, 3
  %21 = and i32 %0, 1
  %22 = icmp eq i32 %1, 8
  %23 = icmp eq i32 %11, 0
  %24 = and i1 %22, %23
  %25 = icmp eq i32 %11, 1
  %26 = and i1 %22, %25
  %27 = icmp eq i32 %1, 16
  %28 = and i1 %27, %23
  %29 = and i1 %27, %25
  %30 = and i32 %10, 6
  %31 = icmp eq i32 %11, 2
  %32 = icmp eq i32 %21, 0
  %33 = icmp eq i32 %16, 0
  br label %40

34:                                               ; preds = %454
  %35 = and i64 %458, 4294967295
  %36 = shl i64 %457, 32
  %37 = or disjoint i64 %35, %36
  br label %38

38:                                               ; preds = %34, %5
  %39 = phi i64 [ 0, %5 ], [ %37, %34 ]
  ret i64 %39

40:                                               ; preds = %15, %454
  %41 = phi i64 [ 0, %15 ], [ %458, %454 ]
  %42 = phi i64 [ 0, %15 ], [ %457, %454 ]
  %43 = phi i32 [ 0, %15 ], [ %456, %454 ]
  %44 = phi i32 [ 0, %15 ], [ %455, %454 ]
  %45 = phi i32 [ 0, %15 ], [ %459, %454 ]
  %46 = mul i32 %45, %12
  %47 = zext nneg i32 %46 to i64
  %48 = lshr i64 %9, %47
  %49 = trunc i64 %48 to i32
  %50 = and i32 %49, %18
  %51 = add i32 %46, %1
  %52 = zext nneg i32 %51 to i64
  %53 = lshr i64 %9, %52
  %54 = trunc i64 %53 to i32
  %55 = and i32 %54, %18
  %56 = add i32 %46, %19
  %57 = zext nneg i32 %56 to i64
  %58 = lshr i64 %9, %57
  %59 = trunc i64 %58 to i32
  %60 = and i32 %59, %18
  %61 = add i32 %46, %20
  %62 = zext nneg i32 %61 to i64
  %63 = lshr i64 %9, %62
  %64 = trunc i64 %63 to i32
  %65 = and i32 %64, %18
  br i1 %24, label %66, label %68

66:                                               ; preds = %40
  %67 = and i32 %50, 255
  br label %104

68:                                               ; preds = %40
  br i1 %26, label %69, label %76

69:                                               ; preds = %68
  %70 = and i32 %50, 255
  %71 = shl nuw i32 %50, 1
  %72 = and i32 %71, 256
  %73 = sub nsw i32 %70, %72
  %74 = lshr i32 %73, 31
  %75 = tail call i32 @llvm.abs.i32(i32 %73, i1 true)
  br label %104

76:                                               ; preds = %68
  br i1 %28, label %77, label %82

77:                                               ; preds = %76
  %78 = lshr i32 %50, 15
  %79 = and i32 %78, 1
  %80 = and i32 %50, 32767
  %81 = icmp ugt i32 %80, 31744
  br label %104

82:                                               ; preds = %76
  br i1 %29, label %83, label %88

83:                                               ; preds = %82
  %84 = lshr i32 %50, 15
  %85 = and i32 %84, 1
  %86 = and i32 %50, 32767
  %87 = icmp ugt i32 %86, 32640
  br label %104

88:                                               ; preds = %82
  switch i32 %30, label %100 [
    i32 2, label %89
    i32 4, label %96
  ]

89:                                               ; preds = %88
  %90 = lshr i32 %50, 7
  %91 = and i32 %90, 1
  %92 = and i32 %50, 127
  %93 = icmp ugt i32 %92, 124
  %94 = icmp eq i32 %92, 127
  %95 = select i1 %31, i1 %93, i1 %94
  br label %104

96:                                               ; preds = %88
  %97 = lshr i32 %50, 5
  %98 = and i32 %97, 1
  %99 = and i32 %50, 31
  br label %104

100:                                              ; preds = %88
  switch i32 %11, label %103 [
    i32 6, label %104
    i32 7, label %101
  ]

101:                                              ; preds = %100
  %102 = icmp ugt i32 %50, 2139095040
  br label %104

103:                                              ; preds = %100
  br label %104

104:                                              ; preds = %103, %101, %100, %96, %89, %83, %77, %69, %66
  %105 = phi i1 [ false, %66 ], [ false, %69 ], [ %81, %77 ], [ %87, %83 ], [ false, %103 ], [ %102, %101 ], [ false, %100 ], [ false, %96 ], [ %95, %89 ]
  %106 = phi i32 [ 0, %66 ], [ %74, %69 ], [ %79, %77 ], [ %85, %83 ], [ 0, %103 ], [ 0, %101 ], [ 0, %100 ], [ %98, %96 ], [ %91, %89 ]
  %107 = phi i32 [ %67, %66 ], [ %75, %69 ], [ %80, %77 ], [ %86, %83 ], [ 0, %103 ], [ %50, %101 ], [ %50, %100 ], [ %99, %96 ], [ %92, %89 ]
  br i1 %32, label %110, label %108

108:                                              ; preds = %104
  %109 = zext i32 %107 to i64
  br label %115

110:                                              ; preds = %104
  %111 = icmp eq i32 %106, 0
  %112 = zext i32 %107 to i64
  %113 = sub nsw i64 0, %112
  %114 = select i1 %111, i64 %112, i64 %113
  br label %115

115:                                              ; preds = %108, %110
  %116 = phi i64 [ %109, %108 ], [ %114, %110 ]
  br i1 %24, label %117, label %119

117:                                              ; preds = %115
  %118 = and i32 %55, 255
  br label %155

119:                                              ; preds = %115
  br i1 %26, label %120, label %127

120:                                              ; preds = %119
  %121 = and i32 %55, 255
  %122 = shl nuw i32 %55, 1
  %123 = and i32 %122, 256
  %124 = sub nsw i32 %121, %123
  %125 = lshr i32 %124, 31
  %126 = tail call i32 @llvm.abs.i32(i32 %124, i1 true)
  br label %155

127:                                              ; preds = %119
  br i1 %28, label %128, label %133

128:                                              ; preds = %127
  %129 = lshr i32 %55, 15
  %130 = and i32 %129, 1
  %131 = and i32 %55, 32767
  %132 = icmp ugt i32 %131, 31744
  br label %155

133:                                              ; preds = %127
  br i1 %29, label %134, label %139

134:                                              ; preds = %133
  %135 = lshr i32 %55, 15
  %136 = and i32 %135, 1
  %137 = and i32 %55, 32767
  %138 = icmp ugt i32 %137, 32640
  br label %155

139:                                              ; preds = %133
  switch i32 %30, label %151 [
    i32 2, label %140
    i32 4, label %147
  ]

140:                                              ; preds = %139
  %141 = lshr i32 %55, 7
  %142 = and i32 %141, 1
  %143 = and i32 %55, 127
  %144 = icmp ugt i32 %143, 124
  %145 = icmp eq i32 %143, 127
  %146 = select i1 %31, i1 %144, i1 %145
  br label %155

147:                                              ; preds = %139
  %148 = lshr i32 %55, 5
  %149 = and i32 %148, 1
  %150 = and i32 %55, 31
  br label %155

151:                                              ; preds = %139
  switch i32 %11, label %154 [
    i32 6, label %155
    i32 7, label %152
  ]

152:                                              ; preds = %151
  %153 = icmp ugt i32 %55, 2139095040
  br label %155

154:                                              ; preds = %151
  br label %155

155:                                              ; preds = %154, %152, %151, %147, %140, %134, %128, %120, %117
  %156 = phi i1 [ false, %117 ], [ false, %120 ], [ %132, %128 ], [ %138, %134 ], [ false, %154 ], [ %153, %152 ], [ false, %151 ], [ false, %147 ], [ %146, %140 ]
  %157 = phi i32 [ 0, %117 ], [ %125, %120 ], [ %130, %128 ], [ %136, %134 ], [ 0, %154 ], [ 0, %152 ], [ 0, %151 ], [ %149, %147 ], [ %142, %140 ]
  %158 = phi i32 [ %118, %117 ], [ %126, %120 ], [ %131, %128 ], [ %137, %134 ], [ 0, %154 ], [ %55, %152 ], [ %55, %151 ], [ %150, %147 ], [ %143, %140 ]
  br i1 %32, label %161, label %159

159:                                              ; preds = %155
  %160 = zext i32 %158 to i64
  br label %166

161:                                              ; preds = %155
  %162 = icmp eq i32 %157, 0
  %163 = zext i32 %158 to i64
  %164 = sub nsw i64 0, %163
  %165 = select i1 %162, i64 %163, i64 %164
  br label %166

166:                                              ; preds = %159, %161
  %167 = phi i64 [ %160, %159 ], [ %165, %161 ]
  br i1 %24, label %168, label %170

168:                                              ; preds = %166
  %169 = and i32 %60, 255
  br label %206

170:                                              ; preds = %166
  br i1 %26, label %171, label %178

171:                                              ; preds = %170
  %172 = and i32 %60, 255
  %173 = shl nuw i32 %60, 1
  %174 = and i32 %173, 256
  %175 = sub nsw i32 %172, %174
  %176 = lshr i32 %175, 31
  %177 = tail call i32 @llvm.abs.i32(i32 %175, i1 true)
  br label %206

178:                                              ; preds = %170
  br i1 %28, label %179, label %184

179:                                              ; preds = %178
  %180 = lshr i32 %60, 15
  %181 = and i32 %180, 1
  %182 = and i32 %60, 32767
  %183 = icmp ugt i32 %182, 31744
  br label %206

184:                                              ; preds = %178
  br i1 %29, label %185, label %190

185:                                              ; preds = %184
  %186 = lshr i32 %60, 15
  %187 = and i32 %186, 1
  %188 = and i32 %60, 32767
  %189 = icmp ugt i32 %188, 32640
  br label %206

190:                                              ; preds = %184
  switch i32 %30, label %202 [
    i32 2, label %191
    i32 4, label %198
  ]

191:                                              ; preds = %190
  %192 = lshr i32 %60, 7
  %193 = and i32 %192, 1
  %194 = and i32 %60, 127
  %195 = icmp ugt i32 %194, 124
  %196 = icmp eq i32 %194, 127
  %197 = select i1 %31, i1 %195, i1 %196
  br label %206

198:                                              ; preds = %190
  %199 = lshr i32 %60, 5
  %200 = and i32 %199, 1
  %201 = and i32 %60, 31
  br label %206

202:                                              ; preds = %190
  switch i32 %11, label %205 [
    i32 6, label %206
    i32 7, label %203
  ]

203:                                              ; preds = %202
  %204 = icmp ugt i32 %60, 2139095040
  br label %206

205:                                              ; preds = %202
  br label %206

206:                                              ; preds = %205, %203, %202, %198, %191, %185, %179, %171, %168
  %207 = phi i1 [ false, %168 ], [ false, %171 ], [ %183, %179 ], [ %189, %185 ], [ false, %205 ], [ %204, %203 ], [ false, %202 ], [ false, %198 ], [ %197, %191 ]
  %208 = phi i32 [ 0, %168 ], [ %176, %171 ], [ %181, %179 ], [ %187, %185 ], [ 0, %205 ], [ 0, %203 ], [ 0, %202 ], [ %200, %198 ], [ %193, %191 ]
  %209 = phi i32 [ %169, %168 ], [ %177, %171 ], [ %182, %179 ], [ %188, %185 ], [ 0, %205 ], [ %60, %203 ], [ %60, %202 ], [ %201, %198 ], [ %194, %191 ]
  br i1 %32, label %212, label %210

210:                                              ; preds = %206
  %211 = zext i32 %209 to i64
  br label %217

212:                                              ; preds = %206
  %213 = icmp eq i32 %208, 0
  %214 = zext i32 %209 to i64
  %215 = sub nsw i64 0, %214
  %216 = select i1 %213, i64 %214, i64 %215
  br label %217

217:                                              ; preds = %210, %212
  %218 = phi i64 [ %211, %210 ], [ %216, %212 ]
  br i1 %24, label %219, label %221

219:                                              ; preds = %217
  %220 = and i32 %65, 255
  br label %257

221:                                              ; preds = %217
  br i1 %26, label %222, label %229

222:                                              ; preds = %221
  %223 = and i32 %65, 255
  %224 = shl nuw i32 %65, 1
  %225 = and i32 %224, 256
  %226 = sub nsw i32 %223, %225
  %227 = lshr i32 %226, 31
  %228 = tail call i32 @llvm.abs.i32(i32 %226, i1 true)
  br label %257

229:                                              ; preds = %221
  br i1 %28, label %230, label %235

230:                                              ; preds = %229
  %231 = lshr i32 %65, 15
  %232 = and i32 %231, 1
  %233 = and i32 %65, 32767
  %234 = icmp ugt i32 %233, 31744
  br label %257

235:                                              ; preds = %229
  br i1 %29, label %236, label %241

236:                                              ; preds = %235
  %237 = lshr i32 %65, 15
  %238 = and i32 %237, 1
  %239 = and i32 %65, 32767
  %240 = icmp ugt i32 %239, 32640
  br label %257

241:                                              ; preds = %235
  switch i32 %30, label %253 [
    i32 2, label %242
    i32 4, label %249
  ]

242:                                              ; preds = %241
  %243 = lshr i32 %65, 7
  %244 = and i32 %243, 1
  %245 = and i32 %65, 127
  %246 = icmp ugt i32 %245, 124
  %247 = icmp eq i32 %245, 127
  %248 = select i1 %31, i1 %246, i1 %247
  br label %257

249:                                              ; preds = %241
  %250 = lshr i32 %65, 5
  %251 = and i32 %250, 1
  %252 = and i32 %65, 31
  br label %257

253:                                              ; preds = %241
  switch i32 %11, label %256 [
    i32 6, label %257
    i32 7, label %254
  ]

254:                                              ; preds = %253
  %255 = icmp ugt i32 %65, 2139095040
  br label %257

256:                                              ; preds = %253
  br label %257

257:                                              ; preds = %256, %254, %253, %249, %242, %236, %230, %222, %219
  %258 = phi i1 [ false, %219 ], [ false, %222 ], [ %234, %230 ], [ %240, %236 ], [ false, %256 ], [ %255, %254 ], [ false, %253 ], [ false, %249 ], [ %248, %242 ]
  %259 = phi i32 [ 0, %219 ], [ %227, %222 ], [ %232, %230 ], [ %238, %236 ], [ 0, %256 ], [ 0, %254 ], [ 0, %253 ], [ %251, %249 ], [ %244, %242 ]
  %260 = phi i32 [ %220, %219 ], [ %228, %222 ], [ %233, %230 ], [ %239, %236 ], [ 0, %256 ], [ %65, %254 ], [ %65, %253 ], [ %252, %249 ], [ %245, %242 ]
  br i1 %32, label %263, label %261

261:                                              ; preds = %257
  %262 = zext i32 %260 to i64
  br label %268

263:                                              ; preds = %257
  %264 = icmp eq i32 %259, 0
  %265 = zext i32 %260 to i64
  %266 = sub nsw i64 0, %265
  %267 = select i1 %264, i64 %265, i64 %266
  br label %268

268:                                              ; preds = %261, %263
  %269 = phi i64 [ %262, %261 ], [ %267, %263 ]
  %270 = xor i1 %156, true
  %271 = xor i1 %105, true
  %272 = select i1 %156, i1 true, i1 %105
  %273 = select i1 %156, i1 %271, i1 false
  br i1 %272, label %278, label %274

274:                                              ; preds = %268
  %275 = icmp sgt i64 %167, %116
  %276 = icmp slt i64 %167, %116
  %277 = select i1 %33, i1 %275, i1 %276
  br label %278

278:                                              ; preds = %274, %268
  %279 = phi i1 [ %273, %268 ], [ %277, %274 ]
  %280 = zext i1 %279 to i32
  %281 = xor i1 %207, true
  %282 = select i1 %207, i1 true, i1 %105
  %283 = select i1 %207, i1 %271, i1 false
  br i1 %282, label %288, label %284

284:                                              ; preds = %278
  %285 = icmp sgt i64 %218, %116
  %286 = icmp slt i64 %218, %116
  %287 = select i1 %33, i1 %285, i1 %286
  br label %288

288:                                              ; preds = %284, %278
  %289 = phi i1 [ %283, %278 ], [ %287, %284 ]
  %290 = zext i1 %289 to i32
  %291 = add nuw nsw i32 %290, %280
  %292 = select i1 %258, i1 true, i1 %105
  %293 = select i1 %258, i1 %271, i1 false
  br i1 %292, label %298, label %294

294:                                              ; preds = %288
  %295 = icmp sgt i64 %269, %116
  %296 = icmp slt i64 %269, %116
  %297 = select i1 %33, i1 %295, i1 %296
  br label %298

298:                                              ; preds = %294, %288
  %299 = phi i1 [ %293, %288 ], [ %297, %294 ]
  %300 = zext i1 %299 to i32
  %301 = add nuw nsw i32 %291, %300
  %302 = icmp ult i32 %301, 2
  br i1 %302, label %303, label %311

303:                                              ; preds = %298
  %304 = zext nneg i32 %50 to i64
  %305 = add i32 %43, 1
  %306 = mul i32 %43, %1
  %307 = zext nneg i32 %306 to i64
  %308 = shl i64 %304, %307
  %309 = or i64 %308, %41
  %310 = add i32 %44, 1
  br label %311

311:                                              ; preds = %303, %298
  %312 = phi i32 [ %310, %303 ], [ %44, %298 ]
  %313 = phi i32 [ %305, %303 ], [ %43, %298 ]
  %314 = phi i64 [ %309, %303 ], [ %41, %298 ]
  %315 = or i1 %105, %156
  br i1 %315, label %322, label %316

316:                                              ; preds = %311
  %317 = icmp eq i64 %116, %167
  %318 = icmp sgt i64 %116, %167
  %319 = icmp slt i64 %116, %167
  %320 = select i1 %33, i1 %318, i1 %319
  %321 = or i1 %317, %320
  br label %322

322:                                              ; preds = %316, %311
  %323 = phi i1 [ %105, %311 ], [ %321, %316 ]
  %324 = zext i1 %323 to i32
  %325 = or i1 %156, %207
  %326 = select i1 %207, i1 %270, i1 false
  br i1 %325, label %331, label %327

327:                                              ; preds = %322
  %328 = icmp sgt i64 %218, %167
  %329 = icmp slt i64 %218, %167
  %330 = select i1 %33, i1 %328, i1 %329
  br label %331

331:                                              ; preds = %327, %322
  %332 = phi i1 [ %326, %322 ], [ %330, %327 ]
  %333 = zext i1 %332 to i32
  %334 = add nuw nsw i32 %333, %324
  %335 = or i1 %156, %258
  %336 = select i1 %258, i1 %270, i1 false
  br i1 %335, label %341, label %337

337:                                              ; preds = %331
  %338 = icmp sgt i64 %269, %167
  %339 = icmp slt i64 %269, %167
  %340 = select i1 %33, i1 %338, i1 %339
  br label %341

341:                                              ; preds = %337, %331
  %342 = phi i1 [ %336, %331 ], [ %340, %337 ]
  %343 = zext i1 %342 to i32
  %344 = add nuw nsw i32 %334, %343
  %345 = icmp ult i32 %344, 2
  br i1 %345, label %346, label %358

346:                                              ; preds = %341
  %347 = zext nneg i32 %55 to i64
  %348 = add i32 %313, 1
  %349 = mul i32 %313, %1
  %350 = zext nneg i32 %349 to i64
  %351 = shl i64 %347, %350
  %352 = or i64 %351, %314
  %353 = add i32 %312, 1
  %354 = mul i32 %312, %2
  %355 = zext nneg i32 %354 to i64
  %356 = shl nuw i64 1, %355
  %357 = or i64 %356, %42
  br label %358

358:                                              ; preds = %346, %341
  %359 = phi i32 [ %353, %346 ], [ %312, %341 ]
  %360 = phi i32 [ %348, %346 ], [ %313, %341 ]
  %361 = phi i64 [ %357, %346 ], [ %42, %341 ]
  %362 = phi i64 [ %352, %346 ], [ %314, %341 ]
  %363 = or i1 %105, %207
  br i1 %363, label %370, label %364

364:                                              ; preds = %358
  %365 = icmp eq i64 %116, %218
  %366 = icmp sgt i64 %116, %218
  %367 = icmp slt i64 %116, %218
  %368 = select i1 %33, i1 %366, i1 %367
  %369 = or i1 %365, %368
  br label %370

370:                                              ; preds = %364, %358
  %371 = phi i1 [ %105, %358 ], [ %369, %364 ]
  %372 = zext i1 %371 to i32
  br i1 %325, label %379, label %373

373:                                              ; preds = %370
  %374 = icmp eq i64 %167, %218
  %375 = icmp sgt i64 %167, %218
  %376 = icmp slt i64 %167, %218
  %377 = select i1 %33, i1 %375, i1 %376
  %378 = or i1 %374, %377
  br label %379

379:                                              ; preds = %373, %370
  %380 = phi i1 [ %156, %370 ], [ %378, %373 ]
  %381 = zext i1 %380 to i32
  %382 = add nuw nsw i32 %381, %372
  %383 = or i1 %207, %258
  %384 = select i1 %258, i1 %281, i1 false
  br i1 %383, label %389, label %385

385:                                              ; preds = %379
  %386 = icmp sgt i64 %269, %218
  %387 = icmp slt i64 %269, %218
  %388 = select i1 %33, i1 %386, i1 %387
  br label %389

389:                                              ; preds = %385, %379
  %390 = phi i1 [ %384, %379 ], [ %388, %385 ]
  %391 = zext i1 %390 to i32
  %392 = add nuw nsw i32 %382, %391
  %393 = icmp ult i32 %392, 2
  br i1 %393, label %394, label %406

394:                                              ; preds = %389
  %395 = zext nneg i32 %60 to i64
  %396 = add i32 %360, 1
  %397 = mul i32 %360, %1
  %398 = zext nneg i32 %397 to i64
  %399 = shl i64 %395, %398
  %400 = or i64 %399, %362
  %401 = add i32 %359, 1
  %402 = mul i32 %359, %2
  %403 = zext nneg i32 %402 to i64
  %404 = shl i64 2, %403
  %405 = or i64 %404, %361
  br label %406

406:                                              ; preds = %394, %389
  %407 = phi i32 [ %401, %394 ], [ %359, %389 ]
  %408 = phi i32 [ %396, %394 ], [ %360, %389 ]
  %409 = phi i64 [ %405, %394 ], [ %361, %389 ]
  %410 = phi i64 [ %400, %394 ], [ %362, %389 ]
  %411 = or i1 %105, %258
  br i1 %411, label %418, label %412

412:                                              ; preds = %406
  %413 = icmp eq i64 %116, %269
  %414 = icmp sgt i64 %116, %269
  %415 = icmp slt i64 %116, %269
  %416 = select i1 %33, i1 %414, i1 %415
  %417 = or i1 %413, %416
  br label %418

418:                                              ; preds = %412, %406
  %419 = phi i1 [ %105, %406 ], [ %417, %412 ]
  %420 = zext i1 %419 to i32
  br i1 %335, label %427, label %421

421:                                              ; preds = %418
  %422 = icmp eq i64 %167, %269
  %423 = icmp sgt i64 %167, %269
  %424 = icmp slt i64 %167, %269
  %425 = select i1 %33, i1 %423, i1 %424
  %426 = or i1 %422, %425
  br label %427

427:                                              ; preds = %421, %418
  %428 = phi i1 [ %156, %418 ], [ %426, %421 ]
  %429 = zext i1 %428 to i32
  %430 = add nuw nsw i32 %429, %420
  br i1 %383, label %437, label %431

431:                                              ; preds = %427
  %432 = icmp eq i64 %218, %269
  %433 = icmp sgt i64 %218, %269
  %434 = icmp slt i64 %218, %269
  %435 = select i1 %33, i1 %433, i1 %434
  %436 = or i1 %432, %435
  br label %437

437:                                              ; preds = %431, %427
  %438 = phi i1 [ %207, %427 ], [ %436, %431 ]
  %439 = zext i1 %438 to i32
  %440 = add nuw nsw i32 %430, %439
  %441 = icmp ult i32 %440, 2
  br i1 %441, label %442, label %454

442:                                              ; preds = %437
  %443 = zext nneg i32 %65 to i64
  %444 = add i32 %408, 1
  %445 = mul i32 %408, %1
  %446 = zext nneg i32 %445 to i64
  %447 = shl i64 %443, %446
  %448 = or i64 %447, %410
  %449 = add i32 %407, 1
  %450 = mul i32 %407, %2
  %451 = zext nneg i32 %450 to i64
  %452 = shl i64 3, %451
  %453 = or i64 %452, %409
  br label %454

454:                                              ; preds = %442, %437
  %455 = phi i32 [ %449, %442 ], [ %407, %437 ]
  %456 = phi i32 [ %444, %442 ], [ %408, %437 ]
  %457 = phi i64 [ %453, %442 ], [ %409, %437 ]
  %458 = phi i64 [ %448, %442 ], [ %410, %437 ]
  %459 = add nuw nsw i32 %45, 1
  %460 = icmp ult i32 %459, %13
  br i1 %460, label %40, label %34, !llvm.loop !20006
}

; Function Attrs: nofree norecurse nosync nounwind memory(none)
define dso_local i64 @__zluda_ptx_impl_agx_spd_iter(i32 noundef %0, i32 noundef %1, i32 noundef %2, i32 noundef %3, i64 noundef %4, i32 noundef %5) local_unnamed_addr #3 {
  %7 = icmp eq i32 %0, 64
  %8 = zext nneg i32 %0 to i64
  %9 = shl nsw i64 -1, %8
  %10 = xor i64 %9, -1
  %11 = select i1 %7, i64 -1, i64 %10
  %12 = icmp eq i32 %2, 0
  br i1 %12, label %39, label %13

13:                                               ; preds = %6
  %14 = shl nsw i32 -1, %1
  %15 = xor i32 %14, -1
  %16 = and i32 %2, 1
  %17 = icmp eq i32 %2, 1
  br i1 %17, label %20, label %18

18:                                               ; preds = %13
  %19 = and i32 %2, -2
  br label %41

20:                                               ; preds = %74, %13
  %21 = phi i64 [ undef, %13 ], [ %75, %74 ]
  %22 = phi i64 [ 0, %13 ], [ %75, %74 ]
  %23 = phi i32 [ 0, %13 ], [ %76, %74 ]
  %24 = icmp eq i32 %16, 0
  br i1 %24, label %39, label %25

25:                                               ; preds = %20
  %26 = mul i32 %23, %1
  %27 = lshr i32 %5, %26
  %28 = and i32 %27, %15
  %29 = icmp ult i32 %28, %3
  br i1 %29, label %30, label %39

30:                                               ; preds = %25
  %31 = mul i32 %23, %0
  %32 = zext nneg i32 %31 to i64
  %33 = lshr i64 %4, %32
  %34 = and i64 %33, %11
  %35 = mul i32 %28, %0
  %36 = zext nneg i32 %35 to i64
  %37 = shl i64 %34, %36
  %38 = or i64 %37, %22
  br label %39

39:                                               ; preds = %20, %30, %25, %6
  %40 = phi i64 [ 0, %6 ], [ %21, %20 ], [ %38, %30 ], [ %22, %25 ]
  ret i64 %40

41:                                               ; preds = %74, %18
  %42 = phi i64 [ 0, %18 ], [ %75, %74 ]
  %43 = phi i32 [ 0, %18 ], [ %76, %74 ]
  %44 = phi i32 [ 0, %18 ], [ %77, %74 ]
  %45 = mul i32 %43, %1
  %46 = lshr i32 %5, %45
  %47 = and i32 %46, %15
  %48 = icmp ult i32 %47, %3
  br i1 %48, label %49, label %58

49:                                               ; preds = %41
  %50 = mul i32 %43, %0
  %51 = zext nneg i32 %50 to i64
  %52 = lshr i64 %4, %51
  %53 = and i64 %52, %11
  %54 = mul i32 %47, %0
  %55 = zext nneg i32 %54 to i64
  %56 = shl i64 %53, %55
  %57 = or i64 %56, %42
  br label %58

58:                                               ; preds = %49, %41
  %59 = phi i64 [ %57, %49 ], [ %42, %41 ]
  %60 = or disjoint i32 %43, 1
  %61 = mul i32 %60, %1
  %62 = lshr i32 %5, %61
  %63 = and i32 %62, %15
  %64 = icmp ult i32 %63, %3
  br i1 %64, label %65, label %74

65:                                               ; preds = %58
  %66 = mul i32 %60, %0
  %67 = zext nneg i32 %66 to i64
  %68 = lshr i64 %4, %67
  %69 = and i64 %68, %11
  %70 = mul i32 %63, %0
  %71 = zext nneg i32 %70 to i64
  %72 = shl i64 %69, %71
  %73 = or i64 %72, %59
  br label %74

74:                                               ; preds = %65, %58
  %75 = phi i64 [ %73, %65 ], [ %59, %58 ]
  %76 = add nuw i32 %43, 2
  %77 = add i32 %44, 2
  %78 = icmp eq i32 %77, %19
  br i1 %78, label %20, label %41, !llvm.loop !20007
}

; Function Attrs: nounwind
define dso_local i32 @__zluda_ptx_impl_agx_tmem_alloc(i32 noundef %0, i32 noundef %1) local_unnamed_addr #3 {
  %3 = icmp eq i32 %1, 0
  br i1 %3, label %83, label %4

4:                                                ; preds = %2
  %5 = add i32 %0, -513
  %6 = icmp ult i32 %5, -481
  br i1 %6, label %11, label %7

7:                                                ; preds = %4
  %8 = add nuw nsw i32 %0, 1023
  %9 = and i32 %8, %0
  %10 = icmp eq i32 %9, 0
  br i1 %10, label %12, label %11

11:                                               ; preds = %7, %4
  tail call void @llvm.trap()
  unreachable

12:                                               ; preds = %7
  %13 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 0)
  %14 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 1)
  %15 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 2)
  %16 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 0)
  %17 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 1)
  %18 = mul i32 %17, %15
  %19 = add i32 %18, %14
  %20 = mul i32 %19, %16
  %21 = add i32 %13, 1
  %22 = add i32 %21, %20
  %23 = load atomic i32, ptr addrspace(1) @__zluda_agx_tmem_owner acquire, align 4
  %24 = icmp eq i32 %23, %22
  br i1 %24, label %35, label %25

25:                                               ; preds = %12
  %26 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 1) acquire, align 4
  %27 = icmp eq i32 %26, %22
  br i1 %27, label %35, label %28

28:                                               ; preds = %25
  %29 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 2) acquire, align 4
  %30 = icmp eq i32 %29, %22
  br i1 %30, label %35, label %31

31:                                               ; preds = %28
  %32 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 3) acquire, align 4
  %33 = icmp eq i32 %32, %22
  %34 = select i1 %33, i32 3, i32 -1
  br label %35

35:                                               ; preds = %31, %28, %25, %12
  %36 = phi i32 [ 0, %12 ], [ 1, %25 ], [ 2, %28 ], [ %34, %31 ]
  %37 = icmp slt i32 %36, 0
  br i1 %37, label %38, label %54

38:                                               ; preds = %35, %38
  %39 = phi i32 [ %48, %38 ], [ -1, %35 ]
  %40 = phi i32 [ %53, %38 ], [ 0, %35 ]
  %41 = zext nneg i32 %40 to i64
  %42 = getelementptr inbounds [4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 %41
  %43 = cmpxchg ptr addrspace(1) %42, i32 0, i32 %22 acq_rel acquire, align 4
  %44 = extractvalue { i32, i1 } %43, 1
  %45 = extractvalue { i32, i1 } %43, 0
  %46 = icmp eq i32 %45, %22
  %47 = select i1 %44, i1 true, i1 %46
  %48 = select i1 %47, i32 %40, i32 %39
  %49 = add nuw nsw i32 %40, 1
  %50 = icmp ult i32 %40, 3
  %51 = icmp slt i32 %48, 0
  %52 = and i1 %50, %51
  %53 = select i1 %52, i32 %49, i32 0
  br i1 %51, label %38, label %54, !llvm.loop !20008

54:                                               ; preds = %38, %35
  %55 = phi i32 [ %36, %35 ], [ %48, %38 ]
  %56 = lshr i32 %0, 5
  %57 = icmp eq i32 %56, 16
  %58 = shl nsw i32 -1, %56
  %59 = xor i32 %58, -1
  %60 = select i1 %57, i32 65535, i32 %59
  %61 = zext nneg i32 %55 to i64
  %62 = getelementptr inbounds [4 x i32], ptr addrspace(1) @__zluda_agx_tmem_map, i64 0, i64 %61
  br label %63

63:                                               ; preds = %76, %54
  %64 = load atomic i32, ptr addrspace(1) %62 acquire, align 4
  br label %65

65:                                               ; preds = %69, %63
  %66 = phi i32 [ 0, %63 ], [ %67, %69 ]
  %67 = add nuw nsw i32 %66, %56
  %68 = icmp ult i32 %67, 17
  br i1 %68, label %69, label %73

69:                                               ; preds = %65
  %70 = shl nuw i32 %60, %66
  %71 = and i32 %70, %64
  %72 = icmp eq i32 %71, 0
  br i1 %72, label %73, label %65, !llvm.loop !20009

73:                                               ; preds = %65, %69
  %74 = icmp ugt i32 %67, 16
  br i1 %74, label %75, label %76

75:                                               ; preds = %73
  tail call void @llvm.trap()
  unreachable

76:                                               ; preds = %73
  %77 = shl nuw i32 %60, %66
  %78 = or i32 %77, %64
  %79 = cmpxchg ptr addrspace(1) %62, i32 %64, i32 %78 acq_rel acquire, align 4
  %80 = extractvalue { i32, i1 } %79, 1
  br i1 %80, label %81, label %63

81:                                               ; preds = %76
  %82 = shl nuw nsw i32 %66, 5
  br label %83

83:                                               ; preds = %81, %2
  %84 = phi i32 [ 0, %2 ], [ %82, %81 ]
  ret i32 %84
}

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)

; Function Attrs: nounwind
define dso_local void @__zluda_ptx_impl_agx_tmem_dealloc(i32 noundef %0, i32 noundef %1, i32 noundef %2) local_unnamed_addr #3 {
  %4 = icmp eq i32 %2, 0
  br i1 %4, label %59, label %5

5:                                                ; preds = %3
  %6 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 0)
  %7 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 1)
  %8 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 2)
  %9 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 0)
  %10 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 1)
  %11 = mul i32 %10, %8
  %12 = add i32 %11, %7
  %13 = mul i32 %12, %9
  %14 = add i32 %6, 1
  %15 = add i32 %14, %13
  %16 = load atomic i32, ptr addrspace(1) @__zluda_agx_tmem_owner acquire, align 4
  %17 = icmp eq i32 %16, %15
  br i1 %17, label %28, label %18

18:                                               ; preds = %5
  %19 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 1) acquire, align 4
  %20 = icmp eq i32 %19, %15
  br i1 %20, label %28, label %21

21:                                               ; preds = %18
  %22 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 2) acquire, align 4
  %23 = icmp eq i32 %22, %15
  br i1 %23, label %28, label %24

24:                                               ; preds = %21
  %25 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 3) acquire, align 4
  %26 = icmp eq i32 %25, %15
  %27 = select i1 %26, i32 3, i32 -1
  br label %28

28:                                               ; preds = %5, %18, %21, %24
  %29 = phi i32 [ 0, %5 ], [ 1, %18 ], [ 2, %21 ], [ %27, %24 ]
  %30 = icmp slt i32 %29, 0
  br i1 %30, label %37, label %31

31:                                               ; preds = %28
  %32 = and i32 %0, 31
  %33 = icmp ne i32 %32, 0
  %34 = add i32 %1, -513
  %35 = icmp ult i32 %34, -481
  %36 = or i1 %33, %35
  br i1 %36, label %37, label %38

37:                                               ; preds = %31, %28
  tail call void @llvm.trap()
  unreachable

38:                                               ; preds = %31
  %39 = lshr i32 %1, 5
  %40 = icmp eq i32 %39, 16
  %41 = shl nsw i32 -1, %39
  %42 = xor i32 %41, -1
  %43 = select i1 %40, i32 65535, i32 %42
  %44 = lshr i32 %0, 5
  %45 = and i32 %44, 2047
  %46 = shl i32 %43, %45
  %47 = zext nneg i32 %29 to i64
  %48 = getelementptr inbounds [4 x i32], ptr addrspace(1) @__zluda_agx_tmem_map, i64 0, i64 %47
  %49 = xor i32 %46, -1
  %50 = atomicrmw and ptr addrspace(1) %48, i32 %49 acq_rel, align 4
  %51 = and i32 %50, %46
  %52 = icmp eq i32 %51, %46
  br i1 %52, label %54, label %53

53:                                               ; preds = %38
  tail call void @llvm.trap()
  unreachable

54:                                               ; preds = %38
  %55 = and i32 %50, %49
  %56 = icmp eq i32 %55, 0
  br i1 %56, label %57, label %59

57:                                               ; preds = %54
  %58 = getelementptr inbounds [4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 %47
  store atomic i32 0, ptr addrspace(1) %58 release, align 4
  br label %59

59:                                               ; preds = %54, %57, %3
  ret void
}

; Function Attrs: nounwind
define dso_local i64 @__zluda_ptx_impl_agx_tmem_base() local_unnamed_addr #3 {
  %1 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 0)
  %2 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 1)
  %3 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 2)
  %4 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 0)
  %5 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 1)
  %6 = mul i32 %5, %3
  %7 = add i32 %6, %2
  %8 = mul i32 %7, %4
  %9 = add i32 %1, 1
  %10 = add i32 %9, %8
  %11 = load atomic i32, ptr addrspace(1) @__zluda_agx_tmem_owner acquire, align 4
  %12 = icmp eq i32 %11, %10
  br i1 %12, label %23, label %13

13:                                               ; preds = %0
  %14 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 1) acquire, align 4
  %15 = icmp eq i32 %14, %10
  br i1 %15, label %23, label %16

16:                                               ; preds = %13
  %17 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 2) acquire, align 4
  %18 = icmp eq i32 %17, %10
  br i1 %18, label %23, label %19

19:                                               ; preds = %16
  %20 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 3) acquire, align 4
  %21 = icmp eq i32 %20, %10
  %22 = select i1 %21, i32 3, i32 -1
  br label %23

23:                                               ; preds = %0, %13, %16, %19
  %24 = phi i32 [ 0, %0 ], [ 1, %13 ], [ 2, %16 ], [ %22, %19 ]
  %25 = icmp slt i32 %24, 0
  br i1 %25, label %26, label %27

26:                                               ; preds = %23
  tail call void @llvm.trap()
  unreachable

27:                                               ; preds = %23
  %28 = shl nuw nsw i32 %24, 16
  %29 = zext nneg i32 %28 to i64
  %30 = getelementptr inbounds [262144 x i32], ptr addrspace(1) @__zluda_agx_tmem_pool, i64 0, i64 %29
  %31 = ptrtoint ptr addrspace(1) %30 to i64
  ret i64 %31
}

; Function Attrs: nofree norecurse nosync nounwind memory(none)
define dso_local i32 @__zluda_ptx_impl_agx_mma_elem_f32(i32 noundef %0, i32 noundef %1) local_unnamed_addr #3 {
  switch i32 %0, label %69 [
    i32 0, label %3
    i32 1, label %19
    i32 2, label %21
    i32 3, label %23
    i32 4, label %35
    i32 5, label %49
    i32 6, label %59
  ]

3:                                                ; preds = %2
  %4 = lshr i32 %1, 10
  %5 = and i32 %4, 31
  %6 = and i32 %1, 1023
  switch i32 %5, label %10 [
    i32 31, label %7
    i32 0, label %13
  ]

7:                                                ; preds = %3
  %8 = icmp eq i32 %6, 0
  %9 = icmp ne i32 %6, 0
  br label %13

10:                                               ; preds = %3
  %11 = or disjoint i32 %6, 1024
  %12 = add nsw i32 %5, -25
  br label %13

13:                                               ; preds = %3, %7, %10
  %14 = phi i32 [ 0, %7 ], [ %12, %10 ], [ -24, %3 ]
  %15 = phi i32 [ 0, %7 ], [ %11, %10 ], [ %6, %3 ]
  %16 = phi i1 [ %9, %7 ], [ true, %10 ], [ true, %3 ]
  %17 = phi i1 [ %8, %7 ], [ true, %10 ], [ true, %3 ]
  %18 = lshr i32 %1, 15
  br label %80

19:                                               ; preds = %2
  %20 = shl i32 %1, 16
  br label %157

21:                                               ; preds = %2
  %22 = and i32 %1, -8192
  br label %157

23:                                               ; preds = %2
  %24 = and i32 %1, 127
  %25 = lshr i32 %1, 7
  %26 = icmp eq i32 %24, 127
  br i1 %26, label %80, label %27

27:                                               ; preds = %23
  %28 = lshr i32 %24, 3
  %29 = and i32 %1, 7
  %30 = icmp ult i32 %24, 8
  %31 = or disjoint i32 %29, 8
  %32 = add nsw i32 %28, -10
  %33 = select i1 %30, i32 -9, i32 %32
  %34 = select i1 %30, i32 %29, i32 %31
  br label %80

35:                                               ; preds = %2
  %36 = and i32 %1, 127
  %37 = lshr i32 %1, 7
  %38 = icmp ugt i32 %36, 124
  br i1 %38, label %80, label %39

39:                                               ; preds = %35
  %40 = icmp eq i32 %36, 124
  br i1 %40, label %80, label %41

41:                                               ; preds = %39
  %42 = lshr i32 %36, 2
  %43 = and i32 %1, 3
  %44 = icmp ult i32 %36, 4
  %45 = or disjoint i32 %43, 4
  %46 = add nsw i32 %42, -17
  %47 = select i1 %44, i32 -16, i32 %46
  %48 = select i1 %44, i32 %43, i32 %45
  br label %80

49:                                               ; preds = %2
  %50 = lshr i32 %1, 5
  %51 = lshr i32 %1, 3
  %52 = and i32 %1, 7
  %53 = and i32 %1, 24
  %54 = icmp eq i32 %53, 0
  %55 = or disjoint i32 %52, 8
  %56 = or i32 %51, -4
  %57 = select i1 %54, i32 -3, i32 %56
  %58 = select i1 %54, i32 %52, i32 %55
  br label %80

59:                                               ; preds = %2
  %60 = lshr i32 %1, 5
  %61 = lshr i32 %1, 2
  %62 = and i32 %61, 7
  %63 = and i32 %1, 3
  %64 = icmp eq i32 %62, 0
  %65 = or disjoint i32 %63, 4
  %66 = add nsw i32 %62, -5
  %67 = select i1 %64, i32 -4, i32 %66
  %68 = select i1 %64, i32 %63, i32 %65
  br label %80

69:                                               ; preds = %2
  %70 = lshr i32 %1, 2
  %71 = lshr i32 %1, 5
  %72 = lshr i32 %1, 3
  %73 = and i32 %72, 3
  %74 = and i32 %70, 1
  %75 = icmp eq i32 %73, 0
  %76 = or disjoint i32 %74, 2
  %77 = add nsw i32 %73, -2
  %78 = select i1 %75, i32 -1, i32 %77
  %79 = select i1 %75, i32 %74, i32 %76
  br label %80

80:                                               ; preds = %39, %35, %41, %23, %27, %69, %59, %49, %13
  %81 = phi i32 [ %71, %69 ], [ %60, %59 ], [ %50, %49 ], [ %18, %13 ], [ %25, %27 ], [ %25, %23 ], [ %37, %41 ], [ %37, %35 ], [ %37, %39 ]
  %82 = phi i1 [ true, %69 ], [ true, %59 ], [ true, %49 ], [ %17, %13 ], [ true, %27 ], [ false, %23 ], [ true, %41 ], [ false, %35 ], [ true, %39 ]
  %83 = phi i1 [ true, %69 ], [ true, %59 ], [ true, %49 ], [ %16, %13 ], [ true, %27 ], [ true, %23 ], [ true, %41 ], [ true, %35 ], [ false, %39 ]
  %84 = phi i32 [ %79, %69 ], [ %68, %59 ], [ %58, %49 ], [ %15, %13 ], [ %34, %27 ], [ 0, %23 ], [ %48, %41 ], [ 0, %35 ], [ 0, %39 ]
  %85 = phi i32 [ %78, %69 ], [ %67, %59 ], [ %57, %49 ], [ %14, %13 ], [ %33, %27 ], [ 0, %23 ], [ %47, %41 ], [ 0, %35 ], [ 0, %39 ]
  %86 = zext nneg i32 %84 to i64
  %87 = shl i32 %81, 31
  br i1 %82, label %88, label %157

88:                                               ; preds = %80
  br i1 %83, label %91, label %89

89:                                               ; preds = %88
  %90 = or disjoint i32 %87, 2139095040
  br label %157

91:                                               ; preds = %88
  %92 = icmp eq i32 %84, 0
  br i1 %92, label %157, label %93

93:                                               ; preds = %91, %93
  %94 = phi i32 [ %97, %93 ], [ 0, %91 ]
  %95 = phi i64 [ %96, %93 ], [ %86, %91 ]
  %96 = lshr i64 %95, 1
  %97 = add nuw nsw i32 %94, 1
  %98 = icmp ult i64 %95, 2
  br i1 %98, label %99, label %93, !llvm.loop !20003

99:                                               ; preds = %93
  %100 = add nsw i32 %97, %85
  %101 = add i32 %94, %85
  %102 = icmp sgt i32 %100, -126
  %103 = select i1 %102, i32 %101, i32 -126
  %104 = icmp sgt i32 %103, 200
  br i1 %104, label %153, label %105

105:                                              ; preds = %99
  %106 = sub i32 %85, %103
  %107 = add i32 %106, 23
  %108 = icmp sgt i32 %107, -1
  br i1 %108, label %109, label %114

109:                                              ; preds = %105
  %110 = icmp ugt i32 %107, 40
  %111 = zext nneg i32 %107 to i64
  %112 = shl i64 %86, %111
  %113 = select i1 %110, i64 4611686018427387904, i64 %112
  br label %139

114:                                              ; preds = %105
  %115 = sub nuw i32 -23, %106
  %116 = icmp ult i32 %107, -63
  %117 = zext nneg i32 %115 to i64
  %118 = lshr i64 %86, %117
  %119 = select i1 %116, i64 0, i64 %118
  %120 = shl nsw i64 -1, %117
  %121 = xor i64 %120, -1
  %122 = select i1 %116, i64 2147483647, i64 %121
  %123 = and i64 %122, %86
  %124 = icmp eq i64 %123, 0
  %125 = or i1 %116, %124
  %126 = select i1 %124, i64 %119, i64 0
  br i1 %125, label %139, label %127

127:                                              ; preds = %114
  %128 = sub nuw nsw i32 -24, %106
  %129 = zext nneg i32 %128 to i64
  %130 = shl nuw nsw i64 1, %129
  %131 = icmp ugt i64 %123, %130
  %132 = icmp eq i64 %123, %130
  %133 = and i64 %118, 1
  %134 = icmp ne i64 %133, 0
  %135 = select i1 %132, i1 %134, i1 false
  %136 = select i1 %131, i1 true, i1 %135
  %137 = zext i1 %136 to i64
  %138 = add nuw nsw i64 %118, %137
  br label %139

139:                                              ; preds = %114, %127, %109
  %140 = phi i64 [ %113, %109 ], [ %138, %127 ], [ %126, %114 ]
  %141 = icmp ult i64 %140, 16777216
  br i1 %141, label %146, label %142

142:                                              ; preds = %139
  %143 = add nsw i32 %103, 128
  %144 = sext i32 %143 to i64
  %145 = shl nsw i64 %144, 23
  br label %153

146:                                              ; preds = %139
  %147 = icmp ult i64 %140, 8388608
  %148 = sext i32 %103 to i64
  %149 = shl nsw i64 %148, 23
  %150 = add nsw i64 %149, 1056964608
  %151 = select i1 %147, i64 0, i64 %150
  %152 = add i64 %151, %140
  br label %153

153:                                              ; preds = %99, %142, %146
  %154 = phi i64 [ 1099511627776, %99 ], [ %145, %142 ], [ %152, %146 ]
  %155 = trunc i64 %154 to i32
  %156 = or i32 %87, %155
  br label %157

157:                                              ; preds = %89, %153, %80, %91, %21, %19
  %158 = phi i32 [ %22, %21 ], [ %20, %19 ], [ %90, %89 ], [ %156, %153 ], [ 2147483647, %80 ], [ %87, %91 ]
  ret i32 %158
}

; Function Attrs: nounwind
define dso_local i64 @__zluda_ptx_impl_agx_tmem_base_of(i32 noundef %0) local_unnamed_addr #3 {
  %2 = tail call i32 @__zluda_agx_cluster(i32 noundef 3)
  %3 = tail call i32 @__zluda_agx_cluster(i32 noundef 4)
  %4 = tail call i32 @__zluda_agx_cluster(i32 noundef 5)
  %5 = tail call i32 @__zluda_agx_cluster(i32 noundef 8)
  %6 = freeze i32 %2
  %7 = udiv i32 %0, %6
  %8 = mul i32 %7, %6
  %9 = sub i32 %0, %8
  %10 = tail call i32 @__zluda_agx_cluster(i32 noundef 9)
  %11 = urem i32 %7, %3
  %12 = tail call i32 @__zluda_agx_cluster(i32 noundef 10)
  %13 = mul i32 %12, %4
  %14 = mul i32 %3, %2
  %15 = udiv i32 %0, %14
  %16 = add i32 %13, %15
  %17 = tail call i32 @__zluda_agx_cluster(i32 noundef 11)
  %18 = tail call i32 @__zluda_agx_cluster(i32 noundef 12)
  %19 = mul i32 %18, %16
  %20 = add i32 %19, %10
  %21 = mul i32 %20, %3
  %22 = add i32 %21, %11
  %23 = mul i32 %22, %17
  %24 = add i32 %23, %5
  %25 = mul i32 %24, %2
  %26 = add nuw i32 %9, 1
  %27 = add i32 %26, %25
  %28 = load atomic i32, ptr addrspace(1) @__zluda_agx_tmem_owner acquire, align 4
  %29 = icmp eq i32 %28, %27
  br i1 %29, label %40, label %30

30:                                               ; preds = %1
  %31 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 1) acquire, align 4
  %32 = icmp eq i32 %31, %27
  br i1 %32, label %40, label %33

33:                                               ; preds = %30
  %34 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 2) acquire, align 4
  %35 = icmp eq i32 %34, %27
  br i1 %35, label %40, label %36

36:                                               ; preds = %33
  %37 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 3) acquire, align 4
  %38 = icmp eq i32 %37, %27
  %39 = select i1 %38, i32 3, i32 -1
  br label %40

40:                                               ; preds = %1, %30, %33, %36
  %41 = phi i32 [ 0, %1 ], [ 1, %30 ], [ 2, %33 ], [ %39, %36 ]
  %42 = icmp slt i32 %41, 0
  br i1 %42, label %43, label %44

43:                                               ; preds = %40
  tail call void @llvm.trap()
  unreachable

44:                                               ; preds = %40
  %45 = shl nuw nsw i32 %41, 16
  %46 = zext nneg i32 %45 to i64
  %47 = getelementptr inbounds [262144 x i32], ptr addrspace(1) @__zluda_agx_tmem_pool, i64 0, i64 %46
  %48 = ptrtoint ptr addrspace(1) %47 to i64
  ret i64 %48
}

; Function Attrs: nounwind
define dso_local i32 @__zluda_ptx_impl_agx_cluster_rank() local_unnamed_addr #3 {
  %1 = tail call i32 @__zluda_agx_cluster(i32 noundef 6)
  ret i32 %1
}


; Function Attrs: nounwind
define dso_local i64 @__zluda_ptx_impl_agx_desc_to(i64 noundef %0, i32 noundef %1) local_unnamed_addr #3 {
  %3 = trunc i64 %0 to i32
  %4 = shl i32 %3, 4
  %5 = and i32 %4, 262128
  %6 = tail call i32 @__zluda_agx_cluster_mapa(i32 noundef %5, i32 noundef %1)
  %7 = and i64 %0, -16384
  %8 = lshr i32 %6, 4
  %9 = and i32 %8, 16383
  %10 = zext nneg i32 %9 to i64
  %11 = or disjoint i64 %7, %10
  ret i64 %11
}


; Function Attrs: nounwind
define dso_local void @__zluda_ptx_impl_agx_tcgen05_cp(i32 noundef %0, i64 noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef %4) local_unnamed_addr #3 {
  %6 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 0)
  %7 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 1)
  %8 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 2)
  %9 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 0)
  %10 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 1)
  %11 = mul i32 %10, %8
  %12 = add i32 %11, %7
  %13 = mul i32 %12, %9
  %14 = add i32 %6, 1
  %15 = add i32 %14, %13
  %16 = load atomic i32, ptr addrspace(1) @__zluda_agx_tmem_owner acquire, align 4
  %17 = icmp eq i32 %16, %15
  br i1 %17, label %28, label %18

18:                                               ; preds = %5
  %19 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 1) acquire, align 4
  %20 = icmp eq i32 %19, %15
  br i1 %20, label %28, label %21

21:                                               ; preds = %18
  %22 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 2) acquire, align 4
  %23 = icmp eq i32 %22, %15
  br i1 %23, label %28, label %24

24:                                               ; preds = %21
  %25 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 3) acquire, align 4
  %26 = icmp eq i32 %25, %15
  %27 = select i1 %26, i32 3, i32 -1
  br label %28

28:                                               ; preds = %24, %21, %18, %5
  %29 = phi i32 [ 0, %5 ], [ 1, %18 ], [ 2, %21 ], [ %27, %24 ]
  %30 = icmp slt i32 %29, 0
  br i1 %30, label %31, label %32

31:                                               ; preds = %28
  tail call void @llvm.trap()
  unreachable

32:                                               ; preds = %28
  %33 = shl nuw nsw i32 %29, 16
  %34 = zext nneg i32 %33 to i64
  %35 = getelementptr inbounds [262144 x i32], ptr addrspace(1) @__zluda_agx_tmem_pool, i64 0, i64 %34
  %36 = lshr i32 %0, 16
  %37 = and i32 %0, 65535
  %38 = and i32 %2, -3
  %39 = icmp eq i32 %38, 0
  %40 = icmp eq i32 %2, 1
  %41 = icmp eq i32 %2, 5
  %42 = select i1 %41, i32 32, i32 64
  %43 = select i1 %40, i32 4, i32 %42
  %44 = select i1 %39, i32 128, i32 %43
  %45 = icmp ult i32 %2, 2
  %46 = trunc i64 %1 to i32
  %47 = shl i32 %46, 4
  %48 = and i32 %47, 262128
  %49 = lshr i32 %46, 14
  %50 = and i32 %49, 65532
  %51 = lshr i64 %1, 61
  %52 = trunc i64 %51 to i32
  %53 = lshr i64 %1, 30
  %54 = trunc i64 %53 to i32
  %55 = and i32 %54, 65532
  %56 = icmp eq i32 %3, 0
  %57 = icmp ugt i32 %2, 2
  %58 = select i1 %57, i32 2, i32 1
  %59 = select i1 %41, i32 4, i32 %58
  %60 = icmp eq i32 %52, 1
  %61 = select i1 %60, i32 4, i32 0
  %62 = shl nsw i32 -1, %3
  %63 = xor i32 %62, -1
  %64 = icmp eq i32 %3, 4
  br label %65

65:                                               ; preds = %77, %32
  %66 = phi i32 [ 0, %32 ], [ %78, %77 ]
  %67 = and i32 %66, 7
  %68 = lshr i32 %66, 3
  %69 = mul nuw nsw i32 %68, %55
  %70 = or disjoint i32 %69, 1
  %71 = or disjoint i32 %69, 2
  %72 = or disjoint i32 %69, 3
  %73 = and i32 %66, 31
  %74 = and i32 %66, 2147483616
  %75 = lshr i32 %66, 4
  %76 = and i32 %75, 134217726
  br label %80

77:                                               ; preds = %309
  %78 = add nuw nsw i32 %66, 1
  %79 = icmp eq i32 %78, %44
  br i1 %79, label %344, label %65, !llvm.loop !20010

80:                                               ; preds = %309, %65
  %81 = phi i1 [ true, %65 ], [ false, %309 ]
  %82 = phi i32 [ 0, %65 ], [ 4, %309 ]
  switch i32 %52, label %83 [
    i32 2, label %86
    i32 4, label %84
    i32 6, label %85
  ]

83:                                               ; preds = %80
  br label %86

84:                                               ; preds = %80
  br label %86

85:                                               ; preds = %80
  br label %86

86:                                               ; preds = %85, %84, %83, %80
  %87 = phi i32 [ 1, %80 ], [ %61, %83 ], [ 2, %84 ], [ 3, %85 ]
  switch i32 %87, label %88 [
    i32 0, label %92
    i32 2, label %91
  ]

88:                                               ; preds = %86
  %89 = icmp eq i32 %87, 3
  %90 = select i1 %89, i32 8, i32 32
  br label %92

91:                                               ; preds = %86
  br label %92

92:                                               ; preds = %91, %88, %86
  %93 = phi i32 [ 4, %86 ], [ %90, %88 ], [ 16, %91 ]
  %94 = icmp eq i32 %87, 0
  %95 = select i1 %94, i32 4, i32 %93
  %96 = select i1 %94, i32 %50, i32 4
  %97 = mul nuw nsw i32 %95, %67
  %98 = lshr exact i32 %82, 2
  %99 = mul nuw nsw i32 %96, %98
  %100 = add nuw i32 %99, %69
  %101 = add nuw i32 %100, %97
  %102 = shl nuw nsw i32 %101, 2
  %103 = add nuw nsw i32 %102, %48
  %104 = add nsw i32 %87, -1
  %105 = icmp ult i32 %104, 3
  br i1 %105, label %106, label %113

106:                                              ; preds = %92
  %107 = sub nuw nsw i32 4, %87
  %108 = shl nsw i32 -1, %107
  %109 = xor i32 %108, -1
  %110 = shl nuw nsw i32 %109, 7
  %111 = and i32 %103, %110
  %112 = lshr exact i32 %111, 3
  br label %118

113:                                              ; preds = %92
  %114 = icmp eq i32 %87, 4
  %115 = lshr exact i32 %103, 2
  %116 = and i32 %115, 96
  %117 = select i1 %114, i32 %116, i32 0
  br label %118

118:                                              ; preds = %113, %106
  %119 = phi i32 [ %112, %106 ], [ %117, %113 ]
  %120 = xor i32 %119, %103
  %121 = zext i32 %120 to i64
  %122 = inttoptr i64 %121 to ptr addrspace(3)
  %123 = load i32, ptr addrspace(3) %122, align 4
  switch i32 %52, label %124 [
    i32 2, label %127
    i32 4, label %125
    i32 6, label %126
  ]

124:                                              ; preds = %118
  br label %127

125:                                              ; preds = %118
  br label %127

126:                                              ; preds = %118
  br label %127

127:                                              ; preds = %126, %125, %124, %118
  %128 = phi i32 [ 1, %118 ], [ %61, %124 ], [ 2, %125 ], [ 3, %126 ]
  switch i32 %128, label %129 [
    i32 0, label %133
    i32 2, label %132
  ]

129:                                              ; preds = %127
  %130 = icmp eq i32 %128, 3
  %131 = select i1 %130, i32 8, i32 32
  br label %133

132:                                              ; preds = %127
  br label %133

133:                                              ; preds = %132, %129, %127
  %134 = phi i32 [ 4, %127 ], [ %131, %129 ], [ 16, %132 ]
  %135 = icmp eq i32 %128, 0
  %136 = select i1 %135, i32 4, i32 %134
  %137 = select i1 %135, i32 %50, i32 4
  %138 = mul nuw nsw i32 %136, %67
  %139 = mul nuw nsw i32 %137, %98
  %140 = add nuw i32 %70, %139
  %141 = add nuw i32 %140, %138
  %142 = shl nuw nsw i32 %141, 2
  %143 = add nuw nsw i32 %142, %48
  %144 = add nsw i32 %128, -1
  %145 = icmp ult i32 %144, 3
  br i1 %145, label %146, label %153

146:                                              ; preds = %133
  %147 = sub nuw nsw i32 4, %128
  %148 = shl nsw i32 -1, %147
  %149 = xor i32 %148, -1
  %150 = shl nuw nsw i32 %149, 7
  %151 = and i32 %143, %150
  %152 = lshr exact i32 %151, 3
  br label %158

153:                                              ; preds = %133
  %154 = icmp eq i32 %128, 4
  %155 = lshr exact i32 %143, 2
  %156 = and i32 %155, 96
  %157 = select i1 %154, i32 %156, i32 0
  br label %158

158:                                              ; preds = %153, %146
  %159 = phi i32 [ %152, %146 ], [ %157, %153 ]
  %160 = xor i32 %159, %143
  %161 = zext i32 %160 to i64
  %162 = inttoptr i64 %161 to ptr addrspace(3)
  %163 = load i32, ptr addrspace(3) %162, align 4
  switch i32 %52, label %164 [
    i32 2, label %167
    i32 4, label %165
    i32 6, label %166
  ]

164:                                              ; preds = %158
  br label %167

165:                                              ; preds = %158
  br label %167

166:                                              ; preds = %158
  br label %167

167:                                              ; preds = %166, %165, %164, %158
  %168 = phi i32 [ 1, %158 ], [ %61, %164 ], [ 2, %165 ], [ 3, %166 ]
  switch i32 %168, label %169 [
    i32 0, label %173
    i32 2, label %172
  ]

169:                                              ; preds = %167
  %170 = icmp eq i32 %168, 3
  %171 = select i1 %170, i32 8, i32 32
  br label %173

172:                                              ; preds = %167
  br label %173

173:                                              ; preds = %172, %169, %167
  %174 = phi i32 [ 4, %167 ], [ %171, %169 ], [ 16, %172 ]
  %175 = icmp eq i32 %168, 0
  %176 = select i1 %175, i32 4, i32 %174
  %177 = select i1 %175, i32 %50, i32 4
  %178 = mul nuw nsw i32 %176, %67
  %179 = mul nuw nsw i32 %177, %98
  %180 = add nuw i32 %71, %179
  %181 = add nuw i32 %180, %178
  %182 = shl nuw nsw i32 %181, 2
  %183 = add nuw nsw i32 %182, %48
  %184 = add nsw i32 %168, -1
  %185 = icmp ult i32 %184, 3
  br i1 %185, label %186, label %193

186:                                              ; preds = %173
  %187 = sub nuw nsw i32 4, %168
  %188 = shl nsw i32 -1, %187
  %189 = xor i32 %188, -1
  %190 = shl nuw nsw i32 %189, 7
  %191 = and i32 %183, %190
  %192 = lshr exact i32 %191, 3
  br label %198

193:                                              ; preds = %173
  %194 = icmp eq i32 %168, 4
  %195 = lshr exact i32 %183, 2
  %196 = and i32 %195, 96
  %197 = select i1 %194, i32 %196, i32 0
  br label %198

198:                                              ; preds = %193, %186
  %199 = phi i32 [ %192, %186 ], [ %197, %193 ]
  %200 = xor i32 %199, %183
  %201 = zext i32 %200 to i64
  %202 = inttoptr i64 %201 to ptr addrspace(3)
  %203 = load i32, ptr addrspace(3) %202, align 4
  switch i32 %52, label %204 [
    i32 2, label %207
    i32 4, label %205
    i32 6, label %206
  ]

204:                                              ; preds = %198
  br label %207

205:                                              ; preds = %198
  br label %207

206:                                              ; preds = %198
  br label %207

207:                                              ; preds = %206, %205, %204, %198
  %208 = phi i32 [ 1, %198 ], [ %61, %204 ], [ 2, %205 ], [ 3, %206 ]
  switch i32 %208, label %209 [
    i32 0, label %213
    i32 2, label %212
  ]

209:                                              ; preds = %207
  %210 = icmp eq i32 %208, 3
  %211 = select i1 %210, i32 8, i32 32
  br label %213

212:                                              ; preds = %207
  br label %213

213:                                              ; preds = %212, %209, %207
  %214 = phi i32 [ 4, %207 ], [ %211, %209 ], [ 16, %212 ]
  %215 = icmp eq i32 %208, 0
  %216 = select i1 %215, i32 4, i32 %214
  %217 = select i1 %215, i32 %50, i32 4
  %218 = mul nuw nsw i32 %216, %67
  %219 = mul nuw nsw i32 %217, %98
  %220 = add nuw i32 %72, %219
  %221 = add nuw i32 %220, %218
  %222 = shl nuw nsw i32 %221, 2
  %223 = add nuw nsw i32 %222, %48
  %224 = add nsw i32 %208, -1
  %225 = icmp ult i32 %224, 3
  br i1 %225, label %226, label %233

226:                                              ; preds = %213
  %227 = sub nuw nsw i32 4, %208
  %228 = shl nsw i32 -1, %227
  %229 = xor i32 %228, -1
  %230 = shl nuw nsw i32 %229, 7
  %231 = and i32 %223, %230
  %232 = lshr exact i32 %231, 3
  br label %238

233:                                              ; preds = %213
  %234 = icmp eq i32 %208, 4
  %235 = lshr exact i32 %223, 2
  %236 = and i32 %235, 96
  %237 = select i1 %234, i32 %236, i32 0
  br label %238

238:                                              ; preds = %233, %226
  %239 = phi i32 [ %232, %226 ], [ %237, %233 ]
  %240 = xor i32 %239, %223
  %241 = zext i32 %240 to i64
  %242 = inttoptr i64 %241 to ptr addrspace(3)
  %243 = load i32, ptr addrspace(3) %242, align 4
  br i1 %56, label %296, label %244

244:                                              ; preds = %238
  %245 = zext i32 %123 to i64
  %246 = zext i32 %163 to i64
  %247 = shl nuw i64 %246, 32
  %248 = or disjoint i64 %247, %245
  br label %249

249:                                              ; preds = %289, %244
  %250 = phi i32 [ 0, %244 ], [ %294, %289 ]
  %251 = phi i32 [ 0, %244 ], [ %293, %289 ]
  %252 = phi i32 [ 0, %244 ], [ %292, %289 ]
  %253 = phi i32 [ 0, %244 ], [ %291, %289 ]
  %254 = phi i32 [ 0, %244 ], [ %290, %289 ]
  %255 = mul i32 %250, %3
  %256 = icmp ult i32 %255, 64
  %257 = zext nneg i32 %255 to i64
  %258 = lshr i64 %248, %257
  %259 = trunc i64 %258 to i32
  %260 = select i1 %256, i32 %259, i32 0
  %261 = add i32 %255, %3
  %262 = icmp ugt i32 %261, 64
  %263 = sub nuw nsw i32 64, %255
  %264 = shl i32 %203, %263
  %265 = add i32 %255, -64
  %266 = lshr i32 %203, %265
  %267 = select i1 %256, i32 %264, i32 %266
  %268 = select i1 %262, i32 %267, i32 0
  %269 = or i32 %260, %268
  %270 = and i32 %269, %63
  %271 = shl nuw nsw i32 %270, 2
  %272 = select i1 %64, i32 %271, i32 %270
  %273 = shl nuw nsw i32 %250, 3
  %274 = and i32 %273, 24
  %275 = shl i32 %272, %274
  %276 = icmp ult i32 %250, 4
  br i1 %276, label %277, label %279

277:                                              ; preds = %249
  %278 = or i32 %275, %254
  br label %289

279:                                              ; preds = %249
  %280 = icmp ult i32 %250, 8
  br i1 %280, label %281, label %283

281:                                              ; preds = %279
  %282 = or i32 %275, %253
  br label %289

283:                                              ; preds = %279
  %284 = icmp ult i32 %250, 12
  br i1 %284, label %285, label %287

285:                                              ; preds = %283
  %286 = or i32 %275, %252
  br label %289

287:                                              ; preds = %283
  %288 = or i32 %275, %251
  br label %289

289:                                              ; preds = %287, %285, %281, %277
  %290 = phi i32 [ %278, %277 ], [ %254, %281 ], [ %254, %285 ], [ %254, %287 ]
  %291 = phi i32 [ %253, %277 ], [ %282, %281 ], [ %253, %285 ], [ %253, %287 ]
  %292 = phi i32 [ %252, %277 ], [ %252, %281 ], [ %286, %285 ], [ %252, %287 ]
  %293 = phi i32 [ %251, %277 ], [ %251, %281 ], [ %251, %285 ], [ %288, %287 ]
  %294 = add nuw nsw i32 %250, 1
  %295 = icmp eq i32 %294, 16
  br i1 %295, label %296, label %249, !llvm.loop !20015

296:                                              ; preds = %289, %238
  %297 = phi i32 [ %123, %238 ], [ %290, %289 ]
  %298 = phi i32 [ %163, %238 ], [ %291, %289 ]
  %299 = phi i32 [ %203, %238 ], [ %292, %289 ]
  %300 = phi i32 [ %243, %238 ], [ %293, %289 ]
  %301 = add nuw nsw i32 %82, %37
  %302 = and i32 %301, 511
  %303 = add nuw nsw i32 %301, 1
  %304 = and i32 %303, 511
  %305 = add nuw nsw i32 %301, 2
  %306 = and i32 %305, 511
  %307 = add nuw nsw i32 %301, 3
  %308 = and i32 %307, 511
  br label %311

309:                                              ; preds = %325
  %310 = and i1 %45, %81
  br i1 %310, label %80, label %77, !llvm.loop !20016

311:                                              ; preds = %325, %296
  %312 = phi i32 [ 0, %296 ], [ %342, %325 ]
  br i1 %41, label %313, label %316

313:                                              ; preds = %311
  %314 = shl nuw nsw i32 %312, 5
  %315 = add nuw nsw i32 %314, %66
  br label %325

316:                                              ; preds = %311
  switch i32 %2, label %325 [
    i32 3, label %317
    i32 4, label %321
  ]

317:                                              ; preds = %316
  %318 = add nuw nsw i32 %312, %76
  %319 = shl nuw nsw i32 %318, 5
  %320 = or disjoint i32 %319, %73
  br label %325

321:                                              ; preds = %316
  %322 = shl nuw nsw i32 %312, 6
  %323 = add nuw nsw i32 %322, %74
  %324 = or disjoint i32 %323, %73
  br label %325

325:                                              ; preds = %321, %317, %316, %313
  %326 = phi i32 [ %315, %313 ], [ %320, %317 ], [ %324, %321 ], [ %66, %316 ]
  %327 = add i32 %326, %36
  %328 = shl i32 %327, 9
  %329 = and i32 %328, 65024
  %330 = or disjoint i32 %329, %302
  %331 = zext nneg i32 %330 to i64
  %332 = getelementptr inbounds i32, ptr addrspace(1) %35, i64 %331
  store i32 %297, ptr addrspace(1) %332, align 4
  %333 = or disjoint i32 %329, %304
  %334 = zext nneg i32 %333 to i64
  %335 = getelementptr inbounds i32, ptr addrspace(1) %35, i64 %334
  store i32 %298, ptr addrspace(1) %335, align 4
  %336 = or disjoint i32 %329, %306
  %337 = zext nneg i32 %336 to i64
  %338 = getelementptr inbounds i32, ptr addrspace(1) %35, i64 %337
  store i32 %299, ptr addrspace(1) %338, align 4
  %339 = or disjoint i32 %329, %308
  %340 = zext nneg i32 %339 to i64
  %341 = getelementptr inbounds i32, ptr addrspace(1) %35, i64 %340
  store i32 %300, ptr addrspace(1) %341, align 4
  %342 = add nuw nsw i32 %312, 1
  %343 = icmp eq i32 %342, %59
  br i1 %343, label %309, label %311, !llvm.loop !20017

344:                                              ; preds = %77
  %345 = icmp eq i32 %4, 0
  br i1 %345, label %677, label %346

346:                                              ; preds = %344
  %347 = tail call i32 @__zluda_agx_cluster(i32 noundef 6)
  %348 = xor i32 %347, 1
  %349 = tail call i32 @__zluda_agx_cluster(i32 noundef 3)
  %350 = tail call i32 @__zluda_agx_cluster(i32 noundef 4)
  %351 = tail call i32 @__zluda_agx_cluster(i32 noundef 5)
  %352 = tail call i32 @__zluda_agx_cluster(i32 noundef 8)
  %353 = freeze i32 %348
  %354 = freeze i32 %349
  %355 = udiv i32 %353, %354
  %356 = mul i32 %355, %354
  %357 = sub i32 %353, %356
  %358 = tail call i32 @__zluda_agx_cluster(i32 noundef 9)
  %359 = urem i32 %355, %350
  %360 = tail call i32 @__zluda_agx_cluster(i32 noundef 10)
  %361 = mul i32 %360, %351
  %362 = mul i32 %350, %349
  %363 = udiv i32 %348, %362
  %364 = add i32 %361, %363
  %365 = tail call i32 @__zluda_agx_cluster(i32 noundef 11)
  %366 = tail call i32 @__zluda_agx_cluster(i32 noundef 12)
  %367 = mul i32 %366, %364
  %368 = add i32 %367, %358
  %369 = mul i32 %368, %350
  %370 = add i32 %369, %359
  %371 = mul i32 %370, %365
  %372 = add i32 %371, %352
  %373 = mul i32 %372, %349
  %374 = add nuw i32 %357, 1
  %375 = add i32 %374, %373
  %376 = load atomic i32, ptr addrspace(1) @__zluda_agx_tmem_owner acquire, align 4
  %377 = icmp eq i32 %376, %375
  br i1 %377, label %388, label %378

378:                                              ; preds = %346
  %379 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 1) acquire, align 4
  %380 = icmp eq i32 %379, %375
  br i1 %380, label %388, label %381

381:                                              ; preds = %378
  %382 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 2) acquire, align 4
  %383 = icmp eq i32 %382, %375
  br i1 %383, label %388, label %384

384:                                              ; preds = %381
  %385 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 3) acquire, align 4
  %386 = icmp eq i32 %385, %375
  %387 = select i1 %386, i32 3, i32 -1
  br label %388

388:                                              ; preds = %384, %381, %378, %346
  %389 = phi i32 [ 0, %346 ], [ 1, %378 ], [ 2, %381 ], [ %387, %384 ]
  %390 = icmp slt i32 %389, 0
  br i1 %390, label %391, label %392

391:                                              ; preds = %388
  tail call void @llvm.trap()
  unreachable

392:                                              ; preds = %388
  %393 = shl nuw nsw i32 %389, 16
  %394 = zext nneg i32 %393 to i64
  %395 = getelementptr inbounds [262144 x i32], ptr addrspace(1) @__zluda_agx_tmem_pool, i64 0, i64 %394
  %396 = tail call i32 @__zluda_agx_cluster_mapa(i32 noundef %48, i32 noundef %348)
  %397 = and i32 %396, 262128
  br label %398

398:                                              ; preds = %410, %392
  %399 = phi i32 [ 0, %392 ], [ %411, %410 ]
  %400 = and i32 %399, 7
  %401 = lshr i32 %399, 3
  %402 = mul nuw nsw i32 %401, %55
  %403 = or disjoint i32 %402, 1
  %404 = or disjoint i32 %402, 2
  %405 = or disjoint i32 %402, 3
  %406 = and i32 %399, 31
  %407 = and i32 %399, 2147483616
  %408 = lshr i32 %399, 4
  %409 = and i32 %408, 134217726
  br label %413

410:                                              ; preds = %642
  %411 = add nuw nsw i32 %399, 1
  %412 = icmp eq i32 %411, %44
  br i1 %412, label %677, label %398, !llvm.loop !20010

413:                                              ; preds = %642, %398
  %414 = phi i1 [ true, %398 ], [ false, %642 ]
  %415 = phi i32 [ 0, %398 ], [ 4, %642 ]
  switch i32 %52, label %416 [
    i32 2, label %419
    i32 4, label %417
    i32 6, label %418
  ]

416:                                              ; preds = %413
  br label %419

417:                                              ; preds = %413
  br label %419

418:                                              ; preds = %413
  br label %419

419:                                              ; preds = %418, %417, %416, %413
  %420 = phi i32 [ 1, %413 ], [ %61, %416 ], [ 2, %417 ], [ 3, %418 ]
  switch i32 %420, label %421 [
    i32 0, label %425
    i32 2, label %424
  ]

421:                                              ; preds = %419
  %422 = icmp eq i32 %420, 3
  %423 = select i1 %422, i32 8, i32 32
  br label %425

424:                                              ; preds = %419
  br label %425

425:                                              ; preds = %424, %421, %419
  %426 = phi i32 [ 4, %419 ], [ %423, %421 ], [ 16, %424 ]
  %427 = icmp eq i32 %420, 0
  %428 = select i1 %427, i32 4, i32 %426
  %429 = select i1 %427, i32 %50, i32 4
  %430 = mul nuw nsw i32 %428, %400
  %431 = lshr exact i32 %415, 2
  %432 = mul nuw nsw i32 %429, %431
  %433 = add nuw i32 %432, %402
  %434 = add nuw i32 %433, %430
  %435 = shl nuw nsw i32 %434, 2
  %436 = add nuw nsw i32 %435, %397
  %437 = add nsw i32 %420, -1
  %438 = icmp ult i32 %437, 3
  br i1 %438, label %439, label %446

439:                                              ; preds = %425
  %440 = sub nuw nsw i32 4, %420
  %441 = shl nsw i32 -1, %440
  %442 = xor i32 %441, -1
  %443 = shl nuw nsw i32 %442, 7
  %444 = and i32 %436, %443
  %445 = lshr exact i32 %444, 3
  br label %451

446:                                              ; preds = %425
  %447 = icmp eq i32 %420, 4
  %448 = lshr exact i32 %436, 2
  %449 = and i32 %448, 96
  %450 = select i1 %447, i32 %449, i32 0
  br label %451

451:                                              ; preds = %446, %439
  %452 = phi i32 [ %445, %439 ], [ %450, %446 ]
  %453 = xor i32 %452, %436
  %454 = zext i32 %453 to i64
  %455 = inttoptr i64 %454 to ptr addrspace(3)
  %456 = load i32, ptr addrspace(3) %455, align 4
  switch i32 %52, label %457 [
    i32 2, label %460
    i32 4, label %458
    i32 6, label %459
  ]

457:                                              ; preds = %451
  br label %460

458:                                              ; preds = %451
  br label %460

459:                                              ; preds = %451
  br label %460

460:                                              ; preds = %459, %458, %457, %451
  %461 = phi i32 [ 1, %451 ], [ %61, %457 ], [ 2, %458 ], [ 3, %459 ]
  switch i32 %461, label %462 [
    i32 0, label %466
    i32 2, label %465
  ]

462:                                              ; preds = %460
  %463 = icmp eq i32 %461, 3
  %464 = select i1 %463, i32 8, i32 32
  br label %466

465:                                              ; preds = %460
  br label %466

466:                                              ; preds = %465, %462, %460
  %467 = phi i32 [ 4, %460 ], [ %464, %462 ], [ 16, %465 ]
  %468 = icmp eq i32 %461, 0
  %469 = select i1 %468, i32 4, i32 %467
  %470 = select i1 %468, i32 %50, i32 4
  %471 = mul nuw nsw i32 %469, %400
  %472 = mul nuw nsw i32 %470, %431
  %473 = add nuw i32 %403, %472
  %474 = add nuw i32 %473, %471
  %475 = shl nuw nsw i32 %474, 2
  %476 = add nuw nsw i32 %475, %397
  %477 = add nsw i32 %461, -1
  %478 = icmp ult i32 %477, 3
  br i1 %478, label %479, label %486

479:                                              ; preds = %466
  %480 = sub nuw nsw i32 4, %461
  %481 = shl nsw i32 -1, %480
  %482 = xor i32 %481, -1
  %483 = shl nuw nsw i32 %482, 7
  %484 = and i32 %476, %483
  %485 = lshr exact i32 %484, 3
  br label %491

486:                                              ; preds = %466
  %487 = icmp eq i32 %461, 4
  %488 = lshr exact i32 %476, 2
  %489 = and i32 %488, 96
  %490 = select i1 %487, i32 %489, i32 0
  br label %491

491:                                              ; preds = %486, %479
  %492 = phi i32 [ %485, %479 ], [ %490, %486 ]
  %493 = xor i32 %492, %476
  %494 = zext i32 %493 to i64
  %495 = inttoptr i64 %494 to ptr addrspace(3)
  %496 = load i32, ptr addrspace(3) %495, align 4
  switch i32 %52, label %497 [
    i32 2, label %500
    i32 4, label %498
    i32 6, label %499
  ]

497:                                              ; preds = %491
  br label %500

498:                                              ; preds = %491
  br label %500

499:                                              ; preds = %491
  br label %500

500:                                              ; preds = %499, %498, %497, %491
  %501 = phi i32 [ 1, %491 ], [ %61, %497 ], [ 2, %498 ], [ 3, %499 ]
  switch i32 %501, label %502 [
    i32 0, label %506
    i32 2, label %505
  ]

502:                                              ; preds = %500
  %503 = icmp eq i32 %501, 3
  %504 = select i1 %503, i32 8, i32 32
  br label %506

505:                                              ; preds = %500
  br label %506

506:                                              ; preds = %505, %502, %500
  %507 = phi i32 [ 4, %500 ], [ %504, %502 ], [ 16, %505 ]
  %508 = icmp eq i32 %501, 0
  %509 = select i1 %508, i32 4, i32 %507
  %510 = select i1 %508, i32 %50, i32 4
  %511 = mul nuw nsw i32 %509, %400
  %512 = mul nuw nsw i32 %510, %431
  %513 = add nuw i32 %404, %512
  %514 = add nuw i32 %513, %511
  %515 = shl nuw nsw i32 %514, 2
  %516 = add nuw nsw i32 %515, %397
  %517 = add nsw i32 %501, -1
  %518 = icmp ult i32 %517, 3
  br i1 %518, label %519, label %526

519:                                              ; preds = %506
  %520 = sub nuw nsw i32 4, %501
  %521 = shl nsw i32 -1, %520
  %522 = xor i32 %521, -1
  %523 = shl nuw nsw i32 %522, 7
  %524 = and i32 %516, %523
  %525 = lshr exact i32 %524, 3
  br label %531

526:                                              ; preds = %506
  %527 = icmp eq i32 %501, 4
  %528 = lshr exact i32 %516, 2
  %529 = and i32 %528, 96
  %530 = select i1 %527, i32 %529, i32 0
  br label %531

531:                                              ; preds = %526, %519
  %532 = phi i32 [ %525, %519 ], [ %530, %526 ]
  %533 = xor i32 %532, %516
  %534 = zext i32 %533 to i64
  %535 = inttoptr i64 %534 to ptr addrspace(3)
  %536 = load i32, ptr addrspace(3) %535, align 4
  switch i32 %52, label %537 [
    i32 2, label %540
    i32 4, label %538
    i32 6, label %539
  ]

537:                                              ; preds = %531
  br label %540

538:                                              ; preds = %531
  br label %540

539:                                              ; preds = %531
  br label %540

540:                                              ; preds = %539, %538, %537, %531
  %541 = phi i32 [ 1, %531 ], [ %61, %537 ], [ 2, %538 ], [ 3, %539 ]
  switch i32 %541, label %542 [
    i32 0, label %546
    i32 2, label %545
  ]

542:                                              ; preds = %540
  %543 = icmp eq i32 %541, 3
  %544 = select i1 %543, i32 8, i32 32
  br label %546

545:                                              ; preds = %540
  br label %546

546:                                              ; preds = %545, %542, %540
  %547 = phi i32 [ 4, %540 ], [ %544, %542 ], [ 16, %545 ]
  %548 = icmp eq i32 %541, 0
  %549 = select i1 %548, i32 4, i32 %547
  %550 = select i1 %548, i32 %50, i32 4
  %551 = mul nuw nsw i32 %549, %400
  %552 = mul nuw nsw i32 %550, %431
  %553 = add nuw i32 %405, %552
  %554 = add nuw i32 %553, %551
  %555 = shl nuw nsw i32 %554, 2
  %556 = add nuw nsw i32 %555, %397
  %557 = add nsw i32 %541, -1
  %558 = icmp ult i32 %557, 3
  br i1 %558, label %559, label %566

559:                                              ; preds = %546
  %560 = sub nuw nsw i32 4, %541
  %561 = shl nsw i32 -1, %560
  %562 = xor i32 %561, -1
  %563 = shl nuw nsw i32 %562, 7
  %564 = and i32 %556, %563
  %565 = lshr exact i32 %564, 3
  br label %571

566:                                              ; preds = %546
  %567 = icmp eq i32 %541, 4
  %568 = lshr exact i32 %556, 2
  %569 = and i32 %568, 96
  %570 = select i1 %567, i32 %569, i32 0
  br label %571

571:                                              ; preds = %566, %559
  %572 = phi i32 [ %565, %559 ], [ %570, %566 ]
  %573 = xor i32 %572, %556
  %574 = zext i32 %573 to i64
  %575 = inttoptr i64 %574 to ptr addrspace(3)
  %576 = load i32, ptr addrspace(3) %575, align 4
  br i1 %56, label %629, label %577

577:                                              ; preds = %571
  %578 = zext i32 %456 to i64
  %579 = zext i32 %496 to i64
  %580 = shl nuw i64 %579, 32
  %581 = or disjoint i64 %580, %578
  br label %582

582:                                              ; preds = %622, %577
  %583 = phi i32 [ 0, %577 ], [ %627, %622 ]
  %584 = phi i32 [ 0, %577 ], [ %626, %622 ]
  %585 = phi i32 [ 0, %577 ], [ %625, %622 ]
  %586 = phi i32 [ 0, %577 ], [ %624, %622 ]
  %587 = phi i32 [ 0, %577 ], [ %623, %622 ]
  %588 = mul i32 %583, %3
  %589 = icmp ult i32 %588, 64
  %590 = zext nneg i32 %588 to i64
  %591 = lshr i64 %581, %590
  %592 = trunc i64 %591 to i32
  %593 = select i1 %589, i32 %592, i32 0
  %594 = add i32 %588, %3
  %595 = icmp ugt i32 %594, 64
  %596 = sub nuw nsw i32 64, %588
  %597 = shl i32 %536, %596
  %598 = add i32 %588, -64
  %599 = lshr i32 %536, %598
  %600 = select i1 %589, i32 %597, i32 %599
  %601 = select i1 %595, i32 %600, i32 0
  %602 = or i32 %593, %601
  %603 = and i32 %602, %63
  %604 = shl nuw nsw i32 %603, 2
  %605 = select i1 %64, i32 %604, i32 %603
  %606 = shl nuw nsw i32 %583, 3
  %607 = and i32 %606, 24
  %608 = shl i32 %605, %607
  %609 = icmp ult i32 %583, 4
  br i1 %609, label %610, label %612

610:                                              ; preds = %582
  %611 = or i32 %608, %587
  br label %622

612:                                              ; preds = %582
  %613 = icmp ult i32 %583, 8
  br i1 %613, label %614, label %616

614:                                              ; preds = %612
  %615 = or i32 %608, %586
  br label %622

616:                                              ; preds = %612
  %617 = icmp ult i32 %583, 12
  br i1 %617, label %618, label %620

618:                                              ; preds = %616
  %619 = or i32 %608, %585
  br label %622

620:                                              ; preds = %616
  %621 = or i32 %608, %584
  br label %622

622:                                              ; preds = %620, %618, %614, %610
  %623 = phi i32 [ %611, %610 ], [ %587, %614 ], [ %587, %618 ], [ %587, %620 ]
  %624 = phi i32 [ %586, %610 ], [ %615, %614 ], [ %586, %618 ], [ %586, %620 ]
  %625 = phi i32 [ %585, %610 ], [ %585, %614 ], [ %619, %618 ], [ %585, %620 ]
  %626 = phi i32 [ %584, %610 ], [ %584, %614 ], [ %584, %618 ], [ %621, %620 ]
  %627 = add nuw nsw i32 %583, 1
  %628 = icmp eq i32 %627, 16
  br i1 %628, label %629, label %582, !llvm.loop !20015

629:                                              ; preds = %622, %571
  %630 = phi i32 [ %456, %571 ], [ %623, %622 ]
  %631 = phi i32 [ %496, %571 ], [ %624, %622 ]
  %632 = phi i32 [ %536, %571 ], [ %625, %622 ]
  %633 = phi i32 [ %576, %571 ], [ %626, %622 ]
  %634 = add nuw nsw i32 %415, %37
  %635 = and i32 %634, 511
  %636 = add nuw nsw i32 %634, 1
  %637 = and i32 %636, 511
  %638 = add nuw nsw i32 %634, 2
  %639 = and i32 %638, 511
  %640 = add nuw nsw i32 %634, 3
  %641 = and i32 %640, 511
  br label %644

642:                                              ; preds = %658
  %643 = and i1 %45, %414
  br i1 %643, label %413, label %410, !llvm.loop !20016

644:                                              ; preds = %658, %629
  %645 = phi i32 [ 0, %629 ], [ %675, %658 ]
  br i1 %41, label %646, label %649

646:                                              ; preds = %644
  %647 = shl nuw nsw i32 %645, 5
  %648 = add nuw nsw i32 %647, %399
  br label %658

649:                                              ; preds = %644
  switch i32 %2, label %658 [
    i32 3, label %650
    i32 4, label %654
  ]

650:                                              ; preds = %649
  %651 = add nuw nsw i32 %645, %409
  %652 = shl nuw nsw i32 %651, 5
  %653 = or disjoint i32 %652, %406
  br label %658

654:                                              ; preds = %649
  %655 = shl nuw nsw i32 %645, 6
  %656 = add nuw nsw i32 %655, %407
  %657 = or disjoint i32 %656, %406
  br label %658

658:                                              ; preds = %654, %650, %649, %646
  %659 = phi i32 [ %648, %646 ], [ %653, %650 ], [ %657, %654 ], [ %399, %649 ]
  %660 = add i32 %659, %36
  %661 = shl i32 %660, 9
  %662 = and i32 %661, 65024
  %663 = or disjoint i32 %662, %635
  %664 = zext nneg i32 %663 to i64
  %665 = getelementptr inbounds i32, ptr addrspace(1) %395, i64 %664
  store i32 %630, ptr addrspace(1) %665, align 4
  %666 = or disjoint i32 %662, %637
  %667 = zext nneg i32 %666 to i64
  %668 = getelementptr inbounds i32, ptr addrspace(1) %395, i64 %667
  store i32 %631, ptr addrspace(1) %668, align 4
  %669 = or disjoint i32 %662, %639
  %670 = zext nneg i32 %669 to i64
  %671 = getelementptr inbounds i32, ptr addrspace(1) %395, i64 %670
  store i32 %632, ptr addrspace(1) %671, align 4
  %672 = or disjoint i32 %662, %641
  %673 = zext nneg i32 %672 to i64
  %674 = getelementptr inbounds i32, ptr addrspace(1) %395, i64 %673
  store i32 %633, ptr addrspace(1) %674, align 4
  %675 = add nuw nsw i32 %645, 1
  %676 = icmp eq i32 %675, %59
  br i1 %676, label %642, label %644, !llvm.loop !20017

677:                                              ; preds = %410, %344
  ret void
}

; Function Attrs: nounwind
define dso_local void @__zluda_ptx_impl_agx_tcgen05_shift(i32 noundef %0, i32 noundef %1) local_unnamed_addr #3 {
  %3 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 0)
  %4 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 1)
  %5 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 2)
  %6 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 0)
  %7 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 1)
  %8 = mul i32 %7, %5
  %9 = add i32 %8, %4
  %10 = mul i32 %9, %6
  %11 = add i32 %3, 1
  %12 = add i32 %11, %10
  %13 = load atomic i32, ptr addrspace(1) @__zluda_agx_tmem_owner acquire, align 4
  %14 = icmp eq i32 %13, %12
  br i1 %14, label %25, label %15

15:                                               ; preds = %2
  %16 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 1) acquire, align 4
  %17 = icmp eq i32 %16, %12
  br i1 %17, label %25, label %18

18:                                               ; preds = %15
  %19 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 2) acquire, align 4
  %20 = icmp eq i32 %19, %12
  br i1 %20, label %25, label %21

21:                                               ; preds = %18
  %22 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 3) acquire, align 4
  %23 = icmp eq i32 %22, %12
  %24 = select i1 %23, i32 3, i32 -1
  br label %25

25:                                               ; preds = %21, %18, %15, %2
  %26 = phi i32 [ 0, %2 ], [ 1, %15 ], [ 2, %18 ], [ %24, %21 ]
  %27 = icmp slt i32 %26, 0
  br i1 %27, label %28, label %29

28:                                               ; preds = %25
  tail call void @llvm.trap()
  unreachable

29:                                               ; preds = %25
  %30 = shl nuw nsw i32 %26, 16
  %31 = zext nneg i32 %30 to i64
  %32 = getelementptr inbounds [262144 x i32], ptr addrspace(1) @__zluda_agx_tmem_pool, i64 0, i64 %31
  %33 = lshr i32 %0, 16
  %34 = and i32 %0, 511
  %35 = add i32 %0, 1
  %36 = and i32 %35, 511
  %37 = add i32 %0, 2
  %38 = and i32 %37, 511
  %39 = add i32 %0, 3
  %40 = and i32 %39, 511
  %41 = add i32 %0, 4
  %42 = and i32 %41, 511
  %43 = add i32 %0, 5
  %44 = and i32 %43, 511
  %45 = add i32 %0, 6
  %46 = and i32 %45, 511
  %47 = add i32 %0, 7
  %48 = and i32 %47, 511
  br label %49

49:                                               ; preds = %49, %29
  %50 = phi i32 [ 30, %29 ], [ %112, %49 ]
  %51 = add nuw nsw i32 %50, %33
  %52 = shl i32 %51, 9
  %53 = and i32 %52, 65024
  %54 = add nuw nsw i32 %52, 512
  %55 = and i32 %54, 65024
  %56 = or disjoint i32 %53, %34
  %57 = zext nneg i32 %56 to i64
  %58 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %57
  %59 = load i32, ptr addrspace(1) %58, align 4
  %60 = or disjoint i32 %55, %34
  %61 = zext nneg i32 %60 to i64
  %62 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %61
  store i32 %59, ptr addrspace(1) %62, align 4
  %63 = or disjoint i32 %53, %36
  %64 = zext nneg i32 %63 to i64
  %65 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %64
  %66 = load i32, ptr addrspace(1) %65, align 4
  %67 = or disjoint i32 %55, %36
  %68 = zext nneg i32 %67 to i64
  %69 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %68
  store i32 %66, ptr addrspace(1) %69, align 4
  %70 = or disjoint i32 %53, %38
  %71 = zext nneg i32 %70 to i64
  %72 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %71
  %73 = load i32, ptr addrspace(1) %72, align 4
  %74 = or disjoint i32 %55, %38
  %75 = zext nneg i32 %74 to i64
  %76 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %75
  store i32 %73, ptr addrspace(1) %76, align 4
  %77 = or disjoint i32 %53, %40
  %78 = zext nneg i32 %77 to i64
  %79 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %78
  %80 = load i32, ptr addrspace(1) %79, align 4
  %81 = or disjoint i32 %55, %40
  %82 = zext nneg i32 %81 to i64
  %83 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %82
  store i32 %80, ptr addrspace(1) %83, align 4
  %84 = or disjoint i32 %53, %42
  %85 = zext nneg i32 %84 to i64
  %86 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %85
  %87 = load i32, ptr addrspace(1) %86, align 4
  %88 = or disjoint i32 %55, %42
  %89 = zext nneg i32 %88 to i64
  %90 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %89
  store i32 %87, ptr addrspace(1) %90, align 4
  %91 = or disjoint i32 %53, %44
  %92 = zext nneg i32 %91 to i64
  %93 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %92
  %94 = load i32, ptr addrspace(1) %93, align 4
  %95 = or disjoint i32 %55, %44
  %96 = zext nneg i32 %95 to i64
  %97 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %96
  store i32 %94, ptr addrspace(1) %97, align 4
  %98 = or disjoint i32 %53, %46
  %99 = zext nneg i32 %98 to i64
  %100 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %99
  %101 = load i32, ptr addrspace(1) %100, align 4
  %102 = or disjoint i32 %55, %46
  %103 = zext nneg i32 %102 to i64
  %104 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %103
  store i32 %101, ptr addrspace(1) %104, align 4
  %105 = or disjoint i32 %53, %48
  %106 = zext nneg i32 %105 to i64
  %107 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %106
  %108 = load i32, ptr addrspace(1) %107, align 4
  %109 = or disjoint i32 %55, %48
  %110 = zext nneg i32 %109 to i64
  %111 = getelementptr inbounds i32, ptr addrspace(1) %32, i64 %110
  store i32 %108, ptr addrspace(1) %111, align 4
  %112 = add nsw i32 %50, -1
  %113 = icmp eq i32 %50, 0
  br i1 %113, label %114, label %49, !llvm.loop !20018

114:                                              ; preds = %49
  %115 = icmp eq i32 %1, 0
  br i1 %115, label %231, label %116

116:                                              ; preds = %114
  %117 = tail call i32 @__zluda_agx_cluster(i32 noundef 6)
  %118 = xor i32 %117, 1
  %119 = tail call i32 @__zluda_agx_cluster(i32 noundef 3)
  %120 = tail call i32 @__zluda_agx_cluster(i32 noundef 4)
  %121 = tail call i32 @__zluda_agx_cluster(i32 noundef 5)
  %122 = tail call i32 @__zluda_agx_cluster(i32 noundef 8)
  %123 = freeze i32 %118
  %124 = freeze i32 %119
  %125 = udiv i32 %123, %124
  %126 = mul i32 %125, %124
  %127 = sub i32 %123, %126
  %128 = tail call i32 @__zluda_agx_cluster(i32 noundef 9)
  %129 = urem i32 %125, %120
  %130 = tail call i32 @__zluda_agx_cluster(i32 noundef 10)
  %131 = mul i32 %130, %121
  %132 = mul i32 %120, %119
  %133 = udiv i32 %118, %132
  %134 = add i32 %131, %133
  %135 = tail call i32 @__zluda_agx_cluster(i32 noundef 11)
  %136 = tail call i32 @__zluda_agx_cluster(i32 noundef 12)
  %137 = mul i32 %136, %134
  %138 = add i32 %137, %128
  %139 = mul i32 %138, %120
  %140 = add i32 %139, %129
  %141 = mul i32 %140, %135
  %142 = add i32 %141, %122
  %143 = mul i32 %142, %119
  %144 = add nuw i32 %127, 1
  %145 = add i32 %144, %143
  %146 = load atomic i32, ptr addrspace(1) @__zluda_agx_tmem_owner acquire, align 4
  %147 = icmp eq i32 %146, %145
  br i1 %147, label %158, label %148

148:                                              ; preds = %116
  %149 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 1) acquire, align 4
  %150 = icmp eq i32 %149, %145
  br i1 %150, label %158, label %151

151:                                              ; preds = %148
  %152 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 2) acquire, align 4
  %153 = icmp eq i32 %152, %145
  br i1 %153, label %158, label %154

154:                                              ; preds = %151
  %155 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 3) acquire, align 4
  %156 = icmp eq i32 %155, %145
  %157 = select i1 %156, i32 3, i32 -1
  br label %158

158:                                              ; preds = %154, %151, %148, %116
  %159 = phi i32 [ 0, %116 ], [ 1, %148 ], [ 2, %151 ], [ %157, %154 ]
  %160 = icmp slt i32 %159, 0
  br i1 %160, label %161, label %162

161:                                              ; preds = %158
  tail call void @llvm.trap()
  unreachable

162:                                              ; preds = %158
  %163 = shl nuw nsw i32 %159, 16
  %164 = zext nneg i32 %163 to i64
  %165 = getelementptr inbounds [262144 x i32], ptr addrspace(1) @__zluda_agx_tmem_pool, i64 0, i64 %164
  br label %166

166:                                              ; preds = %166, %162
  %167 = phi i32 [ 30, %162 ], [ %229, %166 ]
  %168 = add nuw nsw i32 %167, %33
  %169 = shl i32 %168, 9
  %170 = and i32 %169, 65024
  %171 = add nuw nsw i32 %169, 512
  %172 = and i32 %171, 65024
  %173 = or disjoint i32 %170, %34
  %174 = zext nneg i32 %173 to i64
  %175 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %174
  %176 = load i32, ptr addrspace(1) %175, align 4
  %177 = or disjoint i32 %172, %34
  %178 = zext nneg i32 %177 to i64
  %179 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %178
  store i32 %176, ptr addrspace(1) %179, align 4
  %180 = or disjoint i32 %170, %36
  %181 = zext nneg i32 %180 to i64
  %182 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %181
  %183 = load i32, ptr addrspace(1) %182, align 4
  %184 = or disjoint i32 %172, %36
  %185 = zext nneg i32 %184 to i64
  %186 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %185
  store i32 %183, ptr addrspace(1) %186, align 4
  %187 = or disjoint i32 %170, %38
  %188 = zext nneg i32 %187 to i64
  %189 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %188
  %190 = load i32, ptr addrspace(1) %189, align 4
  %191 = or disjoint i32 %172, %38
  %192 = zext nneg i32 %191 to i64
  %193 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %192
  store i32 %190, ptr addrspace(1) %193, align 4
  %194 = or disjoint i32 %170, %40
  %195 = zext nneg i32 %194 to i64
  %196 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %195
  %197 = load i32, ptr addrspace(1) %196, align 4
  %198 = or disjoint i32 %172, %40
  %199 = zext nneg i32 %198 to i64
  %200 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %199
  store i32 %197, ptr addrspace(1) %200, align 4
  %201 = or disjoint i32 %170, %42
  %202 = zext nneg i32 %201 to i64
  %203 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %202
  %204 = load i32, ptr addrspace(1) %203, align 4
  %205 = or disjoint i32 %172, %42
  %206 = zext nneg i32 %205 to i64
  %207 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %206
  store i32 %204, ptr addrspace(1) %207, align 4
  %208 = or disjoint i32 %170, %44
  %209 = zext nneg i32 %208 to i64
  %210 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %209
  %211 = load i32, ptr addrspace(1) %210, align 4
  %212 = or disjoint i32 %172, %44
  %213 = zext nneg i32 %212 to i64
  %214 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %213
  store i32 %211, ptr addrspace(1) %214, align 4
  %215 = or disjoint i32 %170, %46
  %216 = zext nneg i32 %215 to i64
  %217 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %216
  %218 = load i32, ptr addrspace(1) %217, align 4
  %219 = or disjoint i32 %172, %46
  %220 = zext nneg i32 %219 to i64
  %221 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %220
  store i32 %218, ptr addrspace(1) %221, align 4
  %222 = or disjoint i32 %170, %48
  %223 = zext nneg i32 %222 to i64
  %224 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %223
  %225 = load i32, ptr addrspace(1) %224, align 4
  %226 = or disjoint i32 %172, %48
  %227 = zext nneg i32 %226 to i64
  %228 = getelementptr inbounds i32, ptr addrspace(1) %165, i64 %227
  store i32 %225, ptr addrspace(1) %228, align 4
  %229 = add nsw i32 %167, -1
  %230 = icmp eq i32 %167, 0
  br i1 %230, label %231, label %166, !llvm.loop !20018

231:                                              ; preds = %166, %114
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define dso_local i32 @__zluda_ptx_impl_agx_tcgen05_red(i32 noundef %0, i32 noundef %1, i32 noundef %2) local_unnamed_addr #3 {
  %4 = and i32 %0, 1
  %5 = lshr i32 %0, 1
  %6 = and i32 %5, 3
  switch i32 %6, label %17 [
    i32 1, label %7
    i32 2, label %12
  ]

7:                                                ; preds = %3
  %8 = icmp eq i32 %4, 0
  %9 = tail call i32 @llvm.umin.i32(i32 %2, i32 %1)
  %10 = tail call i32 @llvm.umax.i32(i32 %2, i32 %1)
  %11 = select i1 %8, i32 %10, i32 %9
  br label %47

12:                                               ; preds = %3
  %13 = icmp eq i32 %4, 0
  %14 = tail call i32 @llvm.smin.i32(i32 %2, i32 %1)
  %15 = tail call i32 @llvm.smax.i32(i32 %2, i32 %1)
  %16 = select i1 %13, i32 %15, i32 %14
  br label %47

17:                                               ; preds = %3
  %18 = and i32 %1, 2147483647
  %19 = icmp ugt i32 %18, 2139095040
  %20 = and i32 %2, 2147483647
  %21 = icmp ugt i32 %20, 2139095040
  %22 = or i1 %19, %21
  br i1 %22, label %23, label %30

23:                                               ; preds = %17
  %24 = and i32 %0, 8
  %25 = icmp ne i32 %24, 0
  %26 = and i1 %19, %21
  %27 = or i1 %25, %26
  %28 = select i1 %19, i32 %2, i32 %1
  %29 = select i1 %27, i32 2147483647, i32 %28
  br label %47

30:                                               ; preds = %17
  %31 = zext nneg i32 %18 to i64
  %32 = xor i64 %31, -1
  %33 = zext nneg i32 %1 to i64
  %34 = icmp slt i32 %1, 0
  %35 = select i1 %34, i64 %32, i64 %33
  %36 = zext nneg i32 %20 to i64
  %37 = xor i64 %36, -1
  %38 = zext nneg i32 %2 to i64
  %39 = icmp slt i32 %2, 0
  %40 = select i1 %39, i64 %37, i64 %38
  %41 = icmp eq i32 %4, 0
  %42 = icmp slt i64 %40, %35
  %43 = select i1 %42, i32 %2, i32 %1
  %44 = icmp sgt i64 %40, %35
  %45 = select i1 %44, i32 %2, i32 %1
  %46 = select i1 %41, i32 %45, i32 %43
  br label %47

47:                                               ; preds = %23, %30, %12, %7
  %48 = phi i32 [ %11, %7 ], [ %16, %12 ], [ %46, %30 ], [ %29, %23 ]
  ret i32 %48
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define dso_local i32 @__zluda_ptx_impl_agx_tcgen05_spc_f32(i32 noundef %0, i32 noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef %4) local_unnamed_addr #3 {
  %6 = and i32 %0, 1
  %7 = and i32 %1, 2147483647
  %8 = icmp ult i32 %7, 2139095041
  %9 = icmp eq i32 %6, 0
  %10 = zext nneg i32 %7 to i64
  %11 = sub nsw i64 0, %10
  %12 = icmp slt i32 %1, 0
  %13 = and i1 %9, %12
  %14 = select i1 %13, i64 %11, i64 %10
  %15 = and i32 %2, 2147483647
  %16 = icmp ult i32 %15, 2139095041
  %17 = zext nneg i32 %15 to i64
  %18 = sub nsw i64 0, %17
  %19 = icmp slt i32 %2, 0
  %20 = and i1 %9, %19
  %21 = select i1 %20, i64 %18, i64 %17
  %22 = and i32 %3, 2147483647
  %23 = icmp ult i32 %22, 2139095041
  %24 = zext nneg i32 %22 to i64
  %25 = sub nsw i64 0, %24
  %26 = icmp slt i32 %3, 0
  %27 = and i1 %9, %26
  %28 = select i1 %27, i64 %25, i64 %24
  %29 = and i32 %4, 2147483647
  %30 = icmp ugt i32 %29, 2139095040
  %31 = zext nneg i32 %29 to i64
  %32 = sub nsw i64 0, %31
  %33 = icmp slt i32 %4, 0
  %34 = and i1 %9, %33
  %35 = select i1 %34, i64 %32, i64 %31
  %36 = xor i1 %16, true
  %37 = and i1 %8, %16
  %38 = and i1 %8, %36
  br i1 %37, label %39, label %44

39:                                               ; preds = %5
  %40 = icmp ult i32 %0, 2
  %41 = icmp sgt i64 %21, %14
  %42 = icmp slt i64 %21, %14
  %43 = select i1 %40, i1 %41, i1 %42
  br label %44

44:                                               ; preds = %39, %5
  %45 = phi i1 [ %38, %5 ], [ %43, %39 ]
  %46 = xor i1 %23, true
  %47 = and i1 %8, %23
  %48 = and i1 %8, %46
  br i1 %47, label %49, label %54

49:                                               ; preds = %44
  %50 = icmp ult i32 %0, 2
  %51 = icmp sgt i64 %28, %14
  %52 = icmp slt i64 %28, %14
  %53 = select i1 %50, i1 %51, i1 %52
  br label %54

54:                                               ; preds = %49, %44
  %55 = phi i1 [ %48, %44 ], [ %53, %49 ]
  %56 = xor i1 %8, true
  %57 = or i1 %30, %56
  %58 = and i1 %8, %30
  br i1 %57, label %64, label %59

59:                                               ; preds = %54
  %60 = icmp ult i32 %0, 2
  %61 = icmp sgt i64 %35, %14
  %62 = icmp slt i64 %35, %14
  %63 = select i1 %60, i1 %61, i1 %62
  br label %64

64:                                               ; preds = %59, %54
  %65 = phi i1 [ %58, %54 ], [ %63, %59 ]
  br i1 %37, label %66, label %73

66:                                               ; preds = %64
  %67 = icmp eq i64 %14, %21
  %68 = icmp ult i32 %0, 2
  %69 = icmp sgt i64 %14, %21
  %70 = icmp slt i64 %14, %21
  %71 = select i1 %68, i1 %69, i1 %70
  %72 = or i1 %67, %71
  br label %73

73:                                               ; preds = %66, %64
  %74 = phi i1 [ %56, %64 ], [ %72, %66 ]
  %75 = and i1 %16, %23
  %76 = and i1 %16, %46
  br i1 %75, label %77, label %82

77:                                               ; preds = %73
  %78 = icmp ult i32 %0, 2
  %79 = icmp sgt i64 %28, %21
  %80 = icmp slt i64 %28, %21
  %81 = select i1 %78, i1 %79, i1 %80
  br label %82

82:                                               ; preds = %77, %73
  %83 = phi i1 [ %76, %73 ], [ %81, %77 ]
  %84 = or i1 %30, %36
  %85 = and i1 %16, %30
  br i1 %84, label %91, label %86

86:                                               ; preds = %82
  %87 = icmp ult i32 %0, 2
  %88 = icmp sgt i64 %35, %21
  %89 = icmp slt i64 %35, %21
  %90 = select i1 %87, i1 %88, i1 %89
  br label %91

91:                                               ; preds = %86, %82
  %92 = phi i1 [ %85, %82 ], [ %90, %86 ]
  br i1 %47, label %93, label %100

93:                                               ; preds = %91
  %94 = icmp eq i64 %14, %28
  %95 = icmp ult i32 %0, 2
  %96 = icmp sgt i64 %14, %28
  %97 = icmp slt i64 %14, %28
  %98 = select i1 %95, i1 %96, i1 %97
  %99 = or i1 %94, %98
  br label %100

100:                                              ; preds = %93, %91
  %101 = phi i1 [ %56, %91 ], [ %99, %93 ]
  br i1 %75, label %102, label %109

102:                                              ; preds = %100
  %103 = icmp eq i64 %21, %28
  %104 = icmp ult i32 %0, 2
  %105 = icmp sgt i64 %21, %28
  %106 = icmp slt i64 %21, %28
  %107 = select i1 %104, i1 %105, i1 %106
  %108 = or i1 %103, %107
  br label %109

109:                                              ; preds = %102, %100
  %110 = phi i1 [ %36, %100 ], [ %108, %102 ]
  %111 = or i1 %30, %46
  %112 = and i1 %23, %30
  br i1 %111, label %118, label %113

113:                                              ; preds = %109
  %114 = icmp ult i32 %0, 2
  %115 = icmp sgt i64 %35, %28
  %116 = icmp slt i64 %35, %28
  %117 = select i1 %114, i1 %115, i1 %116
  br label %118

118:                                              ; preds = %113, %109
  %119 = phi i1 [ %112, %109 ], [ %117, %113 ]
  br i1 %57, label %127, label %120

120:                                              ; preds = %118
  %121 = icmp eq i64 %14, %35
  %122 = icmp ult i32 %0, 2
  %123 = icmp sgt i64 %14, %35
  %124 = icmp slt i64 %14, %35
  %125 = select i1 %122, i1 %123, i1 %124
  %126 = or i1 %121, %125
  br label %127

127:                                              ; preds = %120, %118
  %128 = phi i1 [ %56, %118 ], [ %126, %120 ]
  br i1 %84, label %136, label %129

129:                                              ; preds = %127
  %130 = icmp eq i64 %21, %35
  %131 = icmp ult i32 %0, 2
  %132 = icmp sgt i64 %21, %35
  %133 = icmp slt i64 %21, %35
  %134 = select i1 %131, i1 %132, i1 %133
  %135 = or i1 %130, %134
  br label %136

136:                                              ; preds = %129, %127
  %137 = phi i1 [ %36, %127 ], [ %135, %129 ]
  br i1 %111, label %145, label %138

138:                                              ; preds = %136
  %139 = icmp eq i64 %28, %35
  %140 = icmp ult i32 %0, 2
  %141 = icmp sgt i64 %28, %35
  %142 = icmp slt i64 %28, %35
  %143 = select i1 %140, i1 %141, i1 %142
  %144 = or i1 %139, %143
  br label %145

145:                                              ; preds = %138, %136
  %146 = phi i1 [ %46, %136 ], [ %144, %138 ]
  %147 = zext i1 %128 to i32
  %148 = zext i1 %137 to i32
  %149 = add nuw nsw i32 %148, %147
  %150 = zext i1 %101 to i32
  %151 = zext i1 %110 to i32
  %152 = add nuw nsw i32 %151, %150
  %153 = zext i1 %119 to i32
  %154 = add nuw nsw i32 %152, %153
  %155 = icmp ult i32 %154, 2
  %156 = zext i1 %74 to i32
  %157 = zext i1 %83 to i32
  %158 = add nuw nsw i32 %157, %156
  %159 = zext i1 %92 to i32
  %160 = add nuw nsw i32 %158, %159
  %161 = icmp ult i32 %160, 2
  %162 = zext i1 %45 to i32
  %163 = zext i1 %55 to i32
  %164 = add nuw nsw i32 %163, %162
  %165 = zext i1 %65 to i32
  %166 = add nuw nsw i32 %164, %165
  %167 = icmp ult i32 %166, 2
  %168 = zext i1 %146 to i32
  %169 = add nuw nsw i32 %149, %168
  %170 = icmp ult i32 %169, 2
  %171 = zext i1 %167 to i32
  %172 = select i1 %167, i32 2, i32 1
  %173 = select i1 %167, i32 4, i32 1
  %174 = select i1 %161, i32 %173, i32 0
  %175 = select i1 %161, i32 %172, i32 %171
  %176 = shl nuw nsw i32 %175, 1
  %177 = shl nuw nsw i32 2, %176
  %178 = select i1 %155, i32 %177, i32 0
  %179 = or disjoint i32 %178, %174
  %180 = zext i1 %155 to i32
  %181 = add nuw nsw i32 %175, %180
  %182 = shl nuw nsw i32 %181, 1
  %183 = shl nuw nsw i32 3, %182
  %184 = select i1 %170, i32 %183, i32 0
  %185 = or i32 %179, %184
  ret i32 %185
}



; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umin.i64(i64, i64)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.abs.i32(i32, i1 immarg)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.smin.i32(i32, i32)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.umax.i32(i32, i32)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)



!20000 = !{i32 1, !"wchar_size", i32 4}
!20001 = !{i32 7, !"frame-pointer", i32 2}
!20002 = !{!"Ubuntu clang version 18.1.3 (1ubuntu1)"}
!20003 = distinct !{!20003, !20004}
!20004 = !{!"llvm.loop.mustprogress"}
!20005 = distinct !{!20005, !20004}
!20006 = distinct !{!20006, !20004}
!20007 = distinct !{!20007, !20004}
!20008 = distinct !{!20008, !20004}
!20009 = distinct !{!20009, !20004}
!20010 = distinct !{!20010, !20004}
!20011 = !{!20012, !20012, i64 0}
!20012 = !{!"int", !20013, i64 0}
!20013 = !{!"omnipotent char", !20014, i64 0}
!20014 = !{!"Simple C/C++ TBAA"}
!20015 = distinct !{!20015, !20004}
!20016 = distinct !{!20016, !20004}
!20017 = distinct !{!20017, !20004}
!20018 = distinct !{!20018, !20004}

; from zluda_ptx_impl_agx_heap.c

%struct.heap = type { i32, i32, [2 x i32], [32 x i64] }


; Function Attrs: alwaysinline nofree norecurse nounwind memory(readwrite, inaccessiblemem: none)
define dso_local i64 @__zluda_ptx_impl_malloc(i64 noundef %0) local_unnamed_addr #3 {
  %2 = load ptr addrspace(1), ptr addrspace(1) @__zluda_agx_heap, align 8
  %3 = icmp eq ptr addrspace(1) %2, null
  %4 = icmp ugt i64 %0, 2147483632
  %5 = or i1 %4, %3
  br i1 %5, label %57, label %6

6:                                                ; preds = %1
  %7 = add nuw nsw i64 %0, 16
  br label %8

8:                                                ; preds = %8, %6
  %9 = phi i32 [ %13, %8 ], [ 5, %6 ]
  %10 = zext i32 %9 to i64
  %11 = shl nuw i64 1, %10
  %12 = icmp ult i64 %11, %7
  %13 = add i32 %9, 1
  br i1 %12, label %8, label %14, !llvm.loop !30007

14:                                               ; preds = %8
  %15 = getelementptr inbounds %struct.heap, ptr addrspace(1) %2, i64 0, i32 3, i64 %10
  %16 = load atomic i64, ptr addrspace(1) %15 acquire, align 8
  %17 = getelementptr inbounds i32, ptr addrspace(1) %2, i64 1
  br label %18

18:                                               ; preds = %22, %14
  %19 = phi i64 [ %16, %14 ], [ %32, %22 ]
  %20 = trunc i64 %19 to i32
  %21 = icmp eq i32 %20, 0
  br i1 %21, label %33, label %22

22:                                               ; preds = %18
  %23 = and i64 %19, 4294967295
  %24 = getelementptr inbounds i8, ptr addrspace(1) %17, i64 %23
  %25 = load atomic i32, ptr addrspace(1) %24 monotonic, align 4
  %26 = and i64 %19, -4294967296
  %27 = add i64 %26, 4294967296
  %28 = zext i32 %25 to i64
  %29 = or disjoint i64 %27, %28
  %30 = cmpxchg ptr addrspace(1) %15, i64 %19, i64 %29 acq_rel acquire, align 8
  %31 = extractvalue { i64, i1 } %30, 1
  %32 = extractvalue { i64, i1 } %30, 0
  br i1 %31, label %51, label %18

33:                                               ; preds = %18
  %34 = shl nuw i32 1, %9
  %35 = load atomic i32, ptr addrspace(1) %17 monotonic, align 4
  %36 = add i32 %35, -272
  %37 = load atomic i32, ptr addrspace(1) %2 monotonic, align 8
  br label %38

38:                                               ; preds = %42, %33
  %39 = phi i32 [ %37, %33 ], [ %46, %42 ]
  %40 = sub i32 %36, %39
  %41 = icmp ugt i32 %34, %40
  br i1 %41, label %49, label %42

42:                                               ; preds = %38
  %43 = add i32 %39, %34
  %44 = cmpxchg ptr addrspace(1) %2, i32 %39, i32 %43 monotonic monotonic, align 8
  %45 = extractvalue { i32, i1 } %44, 1
  %46 = extractvalue { i32, i1 } %44, 0
  br i1 %45, label %47, label %38, !llvm.loop !30009

47:                                               ; preds = %42
  %48 = add i32 %39, 272
  br label %49

49:                                               ; preds = %38, %47
  %50 = phi i32 [ %48, %47 ], [ 0, %38 ]
  br i1 %41, label %57, label %51

51:                                               ; preds = %22, %49
  %52 = phi i32 [ %50, %49 ], [ %20, %22 ]
  %53 = zext i32 %52 to i64
  %54 = getelementptr inbounds i8, ptr addrspace(1) %2, i64 %53
  store i32 %9, ptr addrspace(1) %54, align 4
  %55 = getelementptr inbounds i8, ptr addrspace(1) %54, i64 16
  %56 = ptrtoint ptr addrspace(1) %55 to i64
  br label %57

57:                                               ; preds = %51, %49, %1
  %58 = phi i64 [ 0, %1 ], [ %56, %51 ], [ 0, %49 ]
  ret i64 %58
}

; Function Attrs: alwaysinline nofree norecurse nounwind memory(readwrite, inaccessiblemem: none)
define dso_local void @__zluda_ptx_impl_free(i64 noundef %0) local_unnamed_addr #3 {
  %2 = load ptr addrspace(1), ptr addrspace(1) @__zluda_agx_heap, align 8
  %3 = icmp eq i64 %0, 0
  %4 = icmp eq ptr addrspace(1) %2, null
  %5 = select i1 %3, i1 true, i1 %4
  br i1 %5, label %29, label %6

6:                                                ; preds = %1
  %7 = ptrtoint ptr addrspace(1) %2 to i64
  %8 = add i64 %0, 4294967280
  %9 = sub i64 %8, %7
  %10 = and i64 %9, 4294967295
  %11 = getelementptr inbounds i8, ptr addrspace(1) %2, i64 %10
  %12 = load i32, ptr addrspace(1) %11, align 4
  %13 = add i32 %12, -32
  %14 = icmp ult i32 %13, -27
  br i1 %14, label %29, label %15

15:                                               ; preds = %6
  %16 = zext nneg i32 %12 to i64
  %17 = getelementptr inbounds %struct.heap, ptr addrspace(1) %2, i64 0, i32 3, i64 %16
  %18 = load atomic i64, ptr addrspace(1) %17 monotonic, align 8
  %19 = getelementptr inbounds i32, ptr addrspace(1) %11, i64 1
  br label %20

20:                                               ; preds = %20, %15
  %21 = phi i64 [ %18, %15 ], [ %28, %20 ]
  %22 = trunc i64 %21 to i32
  store atomic i32 %22, ptr addrspace(1) %19 monotonic, align 4
  %23 = and i64 %21, -4294967296
  %24 = add i64 %23, 4294967296
  %25 = or disjoint i64 %24, %10
  %26 = cmpxchg ptr addrspace(1) %17, i64 %21, i64 %25 release monotonic, align 8
  %27 = extractvalue { i64, i1 } %26, 1
  %28 = extractvalue { i64, i1 } %26, 0
  br i1 %27, label %29, label %20

29:                                               ; preds = %20, %6, %1
  ret void
}



!30000 = !{i32 1, !"wchar_size", i32 4}
!30001 = !{i32 7, !"frame-pointer", i32 2}
!30002 = !{!"Ubuntu clang version 18.1.3 (1ubuntu1)"}
!30003 = !{!30004, !30004, i64 0}
!30004 = !{!"any pointer", !30005, i64 0}
!30005 = !{!"omnipotent char", !30006, i64 0}
!30006 = !{!"Simple C/C++ TBAA"}
!30007 = distinct !{!30007, !30008}
!30008 = !{!"llvm.loop.mustprogress"}
!30009 = distinct !{!30009, !30008}
!30010 = !{!30011, !30011, i64 0}
!30011 = !{!"int", !30005, i64 0}

; from zluda_ptx_impl_agx_math.c

; Function Attrs: alwaysinline mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define dso_local double @__zluda_ptx_impl_rcp_approx_ftz_f64(double noundef %0) local_unnamed_addr #3 {
  %2 = bitcast double %0 to i64
  %3 = and i64 %2, 9218868437227405312
  %4 = icmp eq i64 %3, 0
  %5 = select i1 %4, i64 -9223372036854775808, i64 -4294967296
  %6 = and i64 %5, %2
  %7 = and i64 %6, 9223372032559808512
  %8 = icmp ugt i64 %7, 9218868437227405312
  br i1 %8, label %32, label %9

9:                                                ; preds = %1
  switch i64 %7, label %16 [
    i64 0, label %10
    i64 9218868437227405312, label %13
  ]

10:                                               ; preds = %9
  %11 = or disjoint i64 %6, 9218868437227405312
  %12 = bitcast i64 %11 to double
  br label %32

13:                                               ; preds = %9
  %14 = and i64 %2, -9223372036854775808
  %15 = bitcast i64 %14 to double
  br label %32

16:                                               ; preds = %9
  %17 = bitcast i64 %6 to double
  %18 = fdiv double 1.000000e+00, %17
  %19 = bitcast double %18 to i64
  %20 = and i64 %19, 9218868437227405312
  %21 = icmp eq i64 %20, 9218868437227405312
  %22 = lshr i64 %19, 32
  %23 = and i64 %22, 1
  %24 = add nuw nsw i64 %23, 2147483647
  %25 = select i1 %21, i64 0, i64 %24
  %26 = add i64 %25, %19
  %27 = and i64 %26, 9218868437227405312
  %28 = icmp eq i64 %27, 0
  %29 = select i1 %28, i64 -9223372036854775808, i64 -4294967296
  %30 = and i64 %29, %26
  %31 = bitcast i64 %30 to double
  br label %32

32:                                               ; preds = %1, %16, %13, %10
  %33 = phi double [ %12, %10 ], [ %15, %13 ], [ %31, %16 ], [ 0x7FFFFFFF00000000, %1 ]
  ret double %33
}

; Function Attrs: alwaysinline mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define dso_local double @__zluda_ptx_impl_rsqrt_approx_ftz_f64(double noundef %0) local_unnamed_addr #3 {
  %2 = bitcast double %0 to i64
  %3 = and i64 %2, 9218868437227405312
  %4 = icmp eq i64 %3, 0
  %5 = select i1 %4, i64 -9223372036854775808, i64 -4294967296
  %6 = and i64 %5, %2
  %7 = and i64 %6, 9223372032559808512
  %8 = icmp ugt i64 %7, 9218868437227405312
  br i1 %8, label %36, label %9

9:                                                ; preds = %1
  %10 = icmp sgt i64 %6, -1
  %11 = icmp eq i64 %7, 0
  %12 = or i1 %10, %11
  br i1 %12, label %13, label %36

13:                                               ; preds = %9
  br i1 %11, label %14, label %17

14:                                               ; preds = %13
  %15 = or disjoint i64 %6, 9218868437227405312
  %16 = bitcast i64 %15 to double
  br label %36

17:                                               ; preds = %13
  %18 = icmp eq i64 %6, 9218868437227405312
  br i1 %18, label %36, label %19

19:                                               ; preds = %17
  %20 = bitcast i64 %6 to double
  %21 = tail call double @llvm.sqrt.f64(double %20)
  %22 = fdiv double 1.000000e+00, %21
  %23 = bitcast double %22 to i64
  %24 = and i64 %23, 9218868437227405312
  %25 = icmp eq i64 %24, 9218868437227405312
  %26 = lshr i64 %23, 32
  %27 = and i64 %26, 1
  %28 = add nuw nsw i64 %27, 2147483647
  %29 = select i1 %25, i64 0, i64 %28
  %30 = add i64 %29, %23
  %31 = and i64 %30, 9218868437227405312
  %32 = icmp eq i64 %31, 0
  %33 = select i1 %32, i64 -9223372036854775808, i64 -4294967296
  %34 = and i64 %33, %30
  %35 = bitcast i64 %34 to double
  br label %36

36:                                               ; preds = %1, %9, %17, %19, %14
  %37 = phi double [ %16, %14 ], [ %35, %19 ], [ 0.000000e+00, %17 ], [ 0x7FFFFFFF00000000, %9 ], [ 0x7FFFFFFF00000000, %1 ]
  ret double %37
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.sqrt.f64(double)



!40000 = !{i32 1, !"wchar_size", i32 4}
!40001 = !{i32 7, !"frame-pointer", i32 2}
!40002 = !{!"Ubuntu clang version 18.1.3 (1ubuntu1)"}
; ---------------------------------------------------------------- END GENERATED (gen_agx_tex.py)
