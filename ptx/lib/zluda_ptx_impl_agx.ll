; Implementation of ZLUDA's PTX helper functions (__zluda_ptx_impl_*) for
; Apple GPUs, linked into every module compiled with the AGX backend.
;
; The AMDGPU implementation (zluda_ptx_impl.cpp) is compiled with clang, which
; has no AGX target, so this one is written directly in LLVM IR on top of the
; llvm.agx.* intrinsics. ptx/build.rs assembles it to bitcode (with the llvm-as
; of the LLVM llvm_zluda links) when it can, else it is embedded as text.
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

define i32 @__zluda_ptx_impl_sreg_lanemask_eq() #0 {
  %l = call i32 @llvm.agx.thread.index.in.simdgroup()
  %m = shl i32 1, %l
  ret i32 %m
}

define i32 @__zluda_ptx_impl_sreg_lanemask_gt() #0 {
  %l = call i32 @llvm.agx.thread.index.in.simdgroup()
  %m = shl i32 -2, %l
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
declare void @llvm.memset.p3.i32(ptr addrspace(3) nocapture writeonly, i8, i32, i1 immarg)
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
; The reserved region's offsets are the shared memory size plus a bias, which is 0 here (shared memory starts at 0,
; so sizes and offsets coincide) and which the .air for Metal sets to place the region after Metal's own threadgroup
; addresses (LLVMZludaAirSharedLayout). Externally initialized, so that the optimizer keeps the load for it to find.
@__zluda_agx_reserved_smem_bias = addrspace(4) externally_initialized global i32 0, align 4
define i32 @__zluda_ptx_impl_sreg_reserved_smem_offset_begin() #3 {
  %t = call i32 @__zluda_ptx_impl_sreg_total_smem_size()
  %b = load i32, ptr addrspace(4) @__zluda_agx_reserved_smem_bias, align 4
  %v = add i32 %t, %b
  ret i32 %v
}
define i32 @__zluda_ptx_impl_sreg_reserved_smem_offset_end() #3 {
  %b = call i32 @__zluda_ptx_impl_sreg_reserved_smem_offset_begin()
  %v = add i32 %b, 1024
  ret i32 %v
}
; the size of the reserved region
define i32 @__zluda_ptx_impl_sreg_reserved_smem_offset_cap() #3 {
  ret i32 1024
}
; a size: the shared memory and the reserved region (the reserved region's end where shared memory starts at 0)
define i32 @__zluda_ptx_impl_sreg_aggr_smem_size() #3 {
  %t = call i32 @__zluda_ptx_impl_sreg_total_smem_size()
  %v = add i32 %t, 1024
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
; f32 atomic adds that keep subnormal inputs and results (PTX ISA: on shared
; memory, and .noftz): a compare-and-swap loop around an IEEE addition, the
; sum in (software) f64 rounded once to f32, which is correctly rounded since
; 53 >= 2 * 24 + 2 bits; constrained operations, which are not folded back into
; an f32 addition that flushes.
define float @__zluda_ptx_impl_agx_fadd_ieee(float %a, float %b) #0 {
  %a64 = fpext float %a to double
  %b64 = fpext float %b to double
  %s = call double @llvm.experimental.constrained.fadd.f64(double %a64, double %b64, metadata !"round.tonearest", metadata !"fpexcept.ignore")
  %r = call float @llvm.experimental.constrained.fptrunc.f32.f64(double %s, metadata !"round.tonearest", metadata !"fpexcept.ignore")
  ret float %r
}
declare double @llvm.experimental.constrained.fadd.f64(double, double, metadata, metadata)
declare float @llvm.experimental.constrained.fptrunc.f32.f64(double, metadata, metadata)

define float @__zluda_ptx_impl_agx_atom_fadd_ieee_shared(ptr addrspace(3) %p, float %v) #3 {
entry:
  %first = load atomic i32, ptr addrspace(3) %p syncscope("workgroup") monotonic, align 4
  br label %loop
loop:
  %old = phi i32 [ %first, %entry ], [ %seen, %loop ]
  %old.f = bitcast i32 %old to float
  %sum = call float @__zluda_ptx_impl_agx_fadd_ieee(float %old.f, float %v)
  %new = bitcast float %sum to i32
  %pair = cmpxchg ptr addrspace(3) %p, i32 %old, i32 %new syncscope("workgroup") monotonic monotonic, align 4
  %seen = extractvalue { i32, i1 } %pair, 0
  %ok = extractvalue { i32, i1 } %pair, 1
  br i1 %ok, label %exit, label %loop
exit:
  ret float %old.f
}

define float @__zluda_ptx_impl_agx_atom_fadd_ieee_global(ptr addrspace(1) %p, float %v) #3 {
entry:
  %first = load atomic i32, ptr addrspace(1) %p syncscope("agent") monotonic, align 4
  br label %loop
loop:
  %old = phi i32 [ %first, %entry ], [ %seen, %loop ]
  %old.f = bitcast i32 %old to float
  %sum = call float @__zluda_ptx_impl_agx_fadd_ieee(float %old.f, float %v)
  %new = bitcast float %sum to i32
  %pair = cmpxchg ptr addrspace(1) %p, i32 %old, i32 %new syncscope("agent") monotonic monotonic, align 4
  %seen = extractvalue { i32, i1 } %pair, 0
  %ok = extractvalue { i32, i1 } %pair, 1
  br i1 %ok, label %exit, label %loop
exit:
  ret float %old.f
}

define float @__zluda_ptx_impl_agx_atom_fadd_ieee_generic(ptr %p, float %v) #3 {
entry:
  %first = load atomic i32, ptr %p syncscope("agent") monotonic, align 4
  br label %loop
loop:
  %old = phi i32 [ %first, %entry ], [ %seen, %loop ]
  %old.f = bitcast i32 %old to float
  %sum = call float @__zluda_ptx_impl_agx_fadd_ieee(float %old.f, float %v)
  %new = bitcast float %sum to i32
  %pair = cmpxchg ptr %p, i32 %old, i32 %new syncscope("agent") monotonic monotonic, align 4
  %seen = extractvalue { i32, i1 } %pair, 0
  %ok = extractvalue { i32, i1 } %pair, 1
  br i1 %ok, label %exit, label %loop
exit:
  ret float %old.f
}

; redux.sync.{min, max}{.abs}{.NaN}.f32: an unsigned integer reduction of keys
; ordered like the values, with -0 < +0 (PTX ISA 9.3: "+0.0 > -0.0"). A NaN
; takes the key that loses (all NaN: the canonical NaN), or with .NaN makes the
; result the canonical NaN.
define float @__zluda_ptx_impl_agx_redux_f32(float %src, i1 %max, i1 %abs, i1 %nan) #2 {
  %bits = bitcast float %src to i32
  %magnitude = and i32 %bits, 2147483647
  %x = select i1 %abs, i32 %magnitude, i32 %bits
  %is_nan = icmp ugt i32 %magnitude, 2139095040
  %negative = icmp slt i32 %x, 0
  %flipped = xor i32 %x, -1
  %positive = or i32 %x, -2147483648
  %key = select i1 %negative, i32 %flipped, i32 %positive
  %lose = select i1 %max, i32 0, i32 -1
  %k = select i1 %is_nan, i32 %lose, i32 %key
  %rmax = call i32 @llvm.agx.simd.reduce.i32(i32 %k, i32 30)
  %rmin = call i32 @llvm.agx.simd.reduce.i32(i32 %k, i32 28)
  %r = select i1 %max, i32 %rmax, i32 %rmin
  %r_positive = icmp slt i32 %r, 0
  %r_pos = and i32 %r, 2147483647
  %r_neg = xor i32 %r, -1
  %value = select i1 %r_positive, i32 %r_pos, i32 %r_neg
  %all_nan = icmp eq i32 %r, %lose
  %any_nan_vote = call i32 @llvm.agx.simd.ballot(i1 %is_nan)
  %any_nan = icmp ne i32 %any_nan_vote, 0
  %nan_result = select i1 %nan, i1 %any_nan, i1 %all_nan
  %result = select i1 %nan_result, i32 2147483647, i32 %value
  %f = bitcast i32 %result to float
  ret float %f
}
define float @__zluda_ptx_impl_redux_sync_min_f32(float %src, i32 %membermask) #2 {
  %r = call float @__zluda_ptx_impl_agx_redux_f32(float %src, i1 false, i1 false, i1 false)
  ret float %r
}
define float @__zluda_ptx_impl_redux_sync_min_f32_nan(float %src, i32 %membermask) #2 {
  %r = call float @__zluda_ptx_impl_agx_redux_f32(float %src, i1 false, i1 false, i1 true)
  ret float %r
}
define float @__zluda_ptx_impl_redux_sync_min_f32_abs(float %src, i32 %membermask) #2 {
  %r = call float @__zluda_ptx_impl_agx_redux_f32(float %src, i1 false, i1 true, i1 false)
  ret float %r
}
define float @__zluda_ptx_impl_redux_sync_min_f32_abs_nan(float %src, i32 %membermask) #2 {
  %r = call float @__zluda_ptx_impl_agx_redux_f32(float %src, i1 false, i1 true, i1 true)
  ret float %r
}
define float @__zluda_ptx_impl_redux_sync_max_f32(float %src, i32 %membermask) #2 {
  %r = call float @__zluda_ptx_impl_agx_redux_f32(float %src, i1 true, i1 false, i1 false)
  ret float %r
}
define float @__zluda_ptx_impl_redux_sync_max_f32_nan(float %src, i32 %membermask) #2 {
  %r = call float @__zluda_ptx_impl_agx_redux_f32(float %src, i1 true, i1 false, i1 true)
  ret float %r
}
define float @__zluda_ptx_impl_redux_sync_max_f32_abs(float %src, i32 %membermask) #2 {
  %r = call float @__zluda_ptx_impl_agx_redux_f32(float %src, i1 true, i1 true, i1 false)
  ret float %r
}
define float @__zluda_ptx_impl_redux_sync_max_f32_abs_nan(float %src, i32 %membermask) #2 {
  %r = call float @__zluda_ptx_impl_agx_redux_f32(float %src, i1 true, i1 true, i1 true)
  ret float %r
}
; Approximate math maps to the hardware functions (llvm.agx.exp2 and
; llvm.agx.log2 rather than llvm.exp2 and llvm.log2, which LLVM folds with the
; host's libm for constant arguments: specialized kernels give the same bits).
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
  %r = call float @llvm.agx.exp2.f32(float %x)
  ret float %r
}

define float @__zluda_ptx_impl_lg2_approx_f32(float %x) #0 {
  %r = call float @llvm.agx.log2.f32(float %x)
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
  %e = call float @llvm.agx.exp2.f32(float %s)
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
;   word 1: bits 0-23 the expected arrival count; .layout::v1 (bit 24) objects
;           also hold the payload report of the current primary phase (bit
;           25), that of the last completed one (bit 26) and the parity of the
;           conditional phase (bit 27). Bit 28 is the parity of the primary
;           phase whose completion word 1 reflects.
; The update that brings the counts to zero completes the phase: it flips the
; parity and reloads the pending arrivals with the expected count, in the same
; compare-and-swap; word 1 follows (mbarrier_phase_done). The state an
; arrive returns is the parity of its phase (bit 0) and the pending arrivals
; before it (bits 32-51).
define void @__zluda_ptx_impl_agx_mbarrier_init(ptr addrspace(3) %mbar, i32 %count, i32 %v1) #3 {
  %expected.ptr = getelementptr inbounds i8, ptr addrspace(3) %mbar, i32 4
  %arrivals = shl i32 %count, 19
  %word0 = and i32 %arrivals, 2147483647
  store i32 %word0, ptr addrspace(3) %mbar, align 4
  %layout = shl i32 %v1, 24
  %word1 = or i32 %count, %layout
  store i32 %word1, ptr addrspace(3) %expected.ptr, align 4
  ret void
}

; The completion of a primary phase (the new phase has parity %parity), in
; word 1: for .layout::v1 the report of the completed phase is kept, the next
; one starts at zero and the conditional phase advances if the report was zero
; (PTX ISA 9.4 9.7.15.16.8).
define void @__zluda_ptx_impl_agx_mbarrier_phase_done(ptr addrspace(3) %mbar, i32 %parity) #3 {
entry:
  %ptr = getelementptr inbounds i8, ptr addrspace(3) %mbar, i32 4
  %first = load atomic i32, ptr addrspace(3) %ptr syncscope("workgroup") monotonic, align 4
  %stamp = shl i32 %parity, 28
  br label %loop
loop:
  %old = phi i32 [ %first, %entry ], [ %seen, %loop ]
  %v1b = lshr i32 %old, 24
  %v1 = and i32 %v1b, 1
  %repb = lshr i32 %old, 25
  %rep = and i32 %repb, 1
  ; bits 25-28 cleared
  %base = and i32 %old, -503316481
  %stamped = or i32 %base, %stamp
  %rp = shl i32 %rep, 26
  %advance = xor i32 %rep, 1
  %adv = shl i32 %advance, 27
  %cond = and i32 %old, 134217728
  %cond.next = xor i32 %cond, %adv
  %with.rp = or i32 %stamped, %rp
  %new.v1 = or i32 %with.rp, %cond.next
  %new.v0 = or i32 %stamped, %cond
  %is.v1 = icmp ne i32 %v1, 0
  %new = select i1 %is.v1, i32 %new.v1, i32 %new.v0
  %pair = cmpxchg ptr addrspace(3) %ptr, i32 %old, i32 %new syncscope("workgroup") acq_rel monotonic, align 4
  %seen = extractvalue { i32, i1 } %pair, 0
  %ok = extractvalue { i32, i1 } %pair, 1
  br i1 %ok, label %exit, label %loop
exit:
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
  %word1 = load atomic i32, ptr addrspace(3) %expected.ptr syncscope("workgroup") monotonic, align 4
  %expected = and i32 %word1, 16777215
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
  br i1 %complete, label %completed, label %return
completed:
  %parity = lshr i32 %next, 31
  call void @__zluda_ptx_impl_agx_mbarrier_phase_done(ptr addrspace(3) %mbar, i32 %parity)
  br label %return
return:
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
                                  i32 1, label %complete
                                  i32 3, label %drop]
expect:
  call void @__zluda_ptx_impl_agx_mbarrier_expect_tx(ptr addrspace(3) %bar, i32 %tx)
  br label %latch
complete:
  call void @__zluda_ptx_impl_agx_mbarrier_complete_tx(ptr addrspace(3) %bar, i32 %tx)
  br label %latch
arrive:
  %state = call i64 @__zluda_ptx_impl_agx_mbarrier_arrive(ptr addrspace(3) %bar, i32 %count, i32 %tx)
  br label %latch
drop:
  %dstate = call i64 @__zluda_ptx_impl_agx_mbarrier_arrive_drop(ptr addrspace(3) %bar, i32 %count, i32 %tx)
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

; Whether the conditional phase of the given parity has completed
; (.phase_type::conditional, .layout::v1).
define zeroext i1 @__zluda_ptx_impl_agx_mbarrier_test_wait_conditional(ptr addrspace(3) %mbar, i32 %parity) #3 {
  %ptr = getelementptr inbounds i8, ptr addrspace(3) %mbar, i32 4
  %word1 = load atomic i32, ptr addrspace(3) %ptr syncscope("workgroup") monotonic, align 4
  fence syncscope("workgroup") acquire
  %c = lshr i32 %word1, 27
  %phase = and i32 %c, 1
  %waited = and i32 %parity, 1
  %done = icmp ne i32 %phase, %waited
  ret i1 %done
}

define zeroext i1 @__zluda_ptx_impl_agx_mbarrier_try_wait_conditional(ptr addrspace(3) %mbar, i32 %parity) #1 {
entry:
  br label %loop
loop:
  %i = phi i32 [ 0, %entry ], [ %i.next, %wait ]
  %done = call zeroext i1 @__zluda_ptx_impl_agx_mbarrier_test_wait_conditional(ptr addrspace(3) %mbar, i32 %parity)
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

; The reportPredicate of a wait of .phase_type::primary: the payload report of
; the completed phase of a .layout::v1 object (zero for .layout::v0, and when
; the wait did not complete). Word 1 follows the completion of the phase
; shortly after word 0: wait (boundedly) until it reflects the current phase.
define zeroext i1 @__zluda_ptx_impl_agx_mbarrier_report(ptr addrspace(3) %mbar, i1 %done) #3 {
entry:
  %ptr = getelementptr inbounds i8, ptr addrspace(3) %mbar, i32 4
  br i1 %done, label %loop, label %none
loop:
  %i = phi i32 [ 0, %entry ], [ %i.next, %loop ]
  %word0 = load atomic i32, ptr addrspace(3) %mbar syncscope("workgroup") monotonic, align 4
  %word1 = load atomic i32, ptr addrspace(3) %ptr syncscope("workgroup") monotonic, align 4
  %p = lshr i32 %word0, 31
  %s = lshr i32 %word1, 28
  %stamp = and i32 %s, 1
  %current = icmp eq i32 %stamp, %p
  %i.next = add i32 %i, 1
  %more = icmp ult i32 %i.next, 64
  %not.current = xor i1 %current, true
  %again = and i1 %not.current, %more
  br i1 %again, label %loop, label %read
read:
  fence syncscope("workgroup") acquire
  %v1b = lshr i32 %word1, 24
  %v1 = trunc i32 %v1b to i1
  %rpb = lshr i32 %word1, 26
  %rp = trunc i32 %rpb to i1
  %r = and i1 %v1, %rp
  ret i1 %r
none:
  ret i1 false
}

; The data inspection of .mbarrier::report::validity on the %size bytes copied
; from %src (PTX ISA 9.4 9.7.10.28.1.3, Table 33): %code 1 .per_element::ff
; matches any 0xff byte; 2 to 5 .per_16bytes::80000000, 8000, 80 and 8 match
; the first element (32, 16, 8 or 4 bits) of each 16-byte chunk with the
; pattern. A match is a report-on: the payload report of a .layout::v1 object
; becomes non-zero (a .layout::v0 object has none).
define void @__zluda_ptx_impl_agx_mbarrier_inspect(ptr addrspace(3) %mbar, ptr addrspace(1) %src, i32 %size, i32 %code) #3 {
entry:
  %bytes = icmp eq i32 %code, 1
  %chunks = lshr i32 %size, 4
  %n = select i1 %bytes, i32 %size, i32 %chunks
  %is2 = icmp eq i32 %code, 2
  %is3 = icmp eq i32 %code, 3
  %is4 = icmp eq i32 %code, 4
  %m0 = select i1 %is4, i32 255, i32 15
  %m1 = select i1 %is3, i32 65535, i32 %m0
  %mask = select i1 %is2, i32 -1, i32 %m1
  %p0 = select i1 %is4, i32 128, i32 8
  %p1 = select i1 %is3, i32 32768, i32 %p0
  %pattern = select i1 %is2, i32 -2147483648, i32 %p1
  %empty = icmp eq i32 %n, 0
  br i1 %empty, label %exit, label %loop
loop:
  %i = phi i32 [ 0, %entry ], [ %i.next, %next ]
  br i1 %bytes, label %byte, label %chunk
byte:
  %bp = getelementptr inbounds i8, ptr addrspace(1) %src, i32 %i
  %b = load i8, ptr addrspace(1) %bp, align 1
  %bhit = icmp eq i8 %b, -1
  br label %next
chunk:
  %off = shl i32 %i, 4
  %cp = getelementptr inbounds i8, ptr addrspace(1) %src, i32 %off
  %c = load i32, ptr addrspace(1) %cp, align 4
  %cm = and i32 %c, %mask
  %chit = icmp eq i32 %cm, %pattern
  br label %next
next:
  %hit = phi i1 [ %bhit, %byte ], [ %chit, %chunk ]
  %i.next = add i32 %i, 1
  %more = icmp ult i32 %i.next, %n
  %go = select i1 %hit, i1 false, i1 %more
  br i1 %go, label %loop, label %done
done:
  br i1 %hit, label %report, label %exit
report:
  call void @__zluda_ptx_impl_agx_mbarrier_report_on(ptr addrspace(3) %mbar)
  br label %exit
exit:
  ret void
}

; report-on: the payload report of a .layout::v1 object becomes non-zero (a
; .layout::v0 object has none)
define void @__zluda_ptx_impl_agx_mbarrier_report_on(ptr addrspace(3) %mbar) #3 {
  %ptr = getelementptr inbounds i8, ptr addrspace(3) %mbar, i32 4
  %word1 = load atomic i32, ptr addrspace(3) %ptr syncscope("workgroup") monotonic, align 4
  %v1b = lshr i32 %word1, 24
  %v1 = and i32 %v1b, 1
  %on = shl i32 %v1, 25
  %old = atomicrmw or ptr addrspace(3) %ptr, i32 %on syncscope("workgroup") monotonic, align 4
  ret void
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

; clusterlaunchcontrol.try_cancel.multicast::cluster::all: the failure response (16 zero bytes) at the same offset
; of each CTA of the cluster, and a complete_tx of 16 bytes on each one's mbarrier
define void @__zluda_ptx_impl_agx_clc_try_cancel_multicast(ptr addrspace(3) %resp, ptr addrspace(3) %mbar) #3 {
entry:
  %resp_a = ptrtoint ptr addrspace(3) %resp to i32
  %mbar_a = ptrtoint ptr addrspace(3) %mbar to i32
  %d0 = call i32 @__zluda_agx_cluster(i32 3)
  %d1 = call i32 @__zluda_agx_cluster(i32 4)
  %d2 = call i32 @__zluda_agx_cluster(i32 5)
  %d01 = mul i32 %d0, %d1
  %n = mul i32 %d01, %d2
  br label %loop
loop:
  %rank = phi i32 [ 0, %entry ], [ %next, %loop ]
  %to_a = call i32 @__zluda_agx_cluster_mapa(i32 %resp_a, i32 %rank)
  %to = inttoptr i32 %to_a to ptr addrspace(3)
  call void @llvm.memset.p3.i32(ptr addrspace(3) align 16 %to, i8 0, i32 16, i1 false)
  %bar_a = call i32 @__zluda_agx_cluster_mapa(i32 %mbar_a, i32 %rank)
  %bar = inttoptr i32 %bar_a to ptr addrspace(3)
  call void @__zluda_ptx_impl_agx_mbarrier_complete_tx(ptr addrspace(3) %bar, i32 16)
  %next = add i32 %rank, 1
  %more = icmp ult i32 %next, %n
  br i1 %more, label %loop, label %exit
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
  ; the f32 add of cp.reduce.async.bulk keeps subnormals (PTX ISA 9.4 9.7.10.28.4.2: as .noftz)
  %fadd.old = call float @__zluda_ptx_impl_agx_atom_fadd_ieee_global(ptr addrspace(1) %d, float %v)
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
  %fadd.old = call float @__zluda_ptx_impl_agx_atom_fadd_ieee_shared(ptr addrspace(3) %d, float %v)
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
  br i1 %is_bf16, label %min_bf, label %min_f0
min_f0:
  br i1 %is_float32, label %min_f, label %min_2
min_f:
  %min_fo = atomicrmw fmin ptr addrspace(1) %p, float %f monotonic, align 4
  br label %exit
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
  br i1 %is_bf16, label %max_bf, label %max_f0
max_f0:
  br i1 %is_float32, label %max_f, label %max_2
max_f:
  %max_fo = atomicrmw fmax ptr addrspace(1) %p, float %f monotonic, align 4
  br label %exit
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
  ; %dir < -2: the .report_mechanism inspection (code -3 - %dir) of the elements
  ; read, which returns the number of matches (the sub-byte types are not
  ; inspected)
  %inspecting = icmp slt i32 %dir, -2
  br i1 %packed, label %sub, label %elements
sub:
  br i1 %inspecting, label %sub_none, label %sub_copy
sub_none:
  ret i32 0
sub_copy:
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
  ; dimension i holds ceil(box_i / element stride_i) elements (PTX ISA 5.5.3.2)
  %cnt0 = call i32 @__zluda_ptx_impl_agx_tma_count(i32 %bx0, i32 %e0)
  %cnt1 = call i32 @__zluda_ptx_impl_agx_tma_count(i32 %b1r, i32 %e1)
  %cnt2 = call i32 @__zluda_ptx_impl_agx_tma_count(i32 %b2r, i32 %e2)
  %cnt3 = call i32 @__zluda_ptx_impl_agx_tma_count(i32 %b3r, i32 %e3)
  %cnt4 = call i32 @__zluda_ptx_impl_agx_tma_count(i32 %b4r, i32 %e4)
  ; swizzle mode from swizzle_mode / swizzle_atomicity
  %swz = lshr i32 %w14t, 16
  %swzm = and i32 %swz, 255
  %w14h = lshr i64 %w14, 48
  %w14ht = trunc i64 %w14h to i32
  %atom = and i32 %w14ht, 255
  %is128 = icmp eq i32 %swzm, 3
  %m128 = select i1 %is128, i32 1, i32 0
  %is64 = icmp eq i32 %swzm, 2
  ; 32B, and 96B (4) with the pattern of 32B (PTX ISA 5.5.7, Figure 31)
  %is32b = icmp eq i32 %swzm, 1
  %is96 = icmp eq i32 %swzm, 4
  %is32 = or i1 %is32b, %is96
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
  %v01 = mul i32 %cnt0, %cnt1
  %v012 = mul i32 %v01, %cnt2
  %v0123 = mul i32 %v012, %cnt3
  %count = mul i32 %v0123, %cnt4
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
  %hits = phi i32 [ 0, %elements ], [ %hits.n, %next ]
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
  br i1 %inspecting, label %inspect, label %copy
inspect:
  br i1 %oob, label %next, label %inspect_elem
inspect_elem:
  %code = sub i32 -3, %dir
  %hit = call zeroext i1 @__zluda_ptx_impl_agx_tma_inspect(i64 %gaddr, i32 %eb, i32 %code)
  %hit32 = zext i1 %hit to i32
  %hits.e = add i32 %hits, %hit32
  br label %next
copy:
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
  %hits.n = phi i32 [ %hits, %load_fill ], [ %hits, %out ], [ %hits, %store ], [ %hits, %reduce ], [ %hits, %inspect ], [ %hits.e, %inspect_elem ]
  ; advance (q0, ..., q4) through the box, dim 0 fastest
  %q0.i = add i32 %q0, 1
  %w0 = icmp eq i32 %q0.i, %cnt0
  %q0.n = select i1 %w0, i32 0, i32 %q0.i
  %k0 = zext i1 %w0 to i32
  %q1.i = add i32 %q1, %k0
  %w1c = icmp eq i32 %q1.i, %cnt1
  %q1.n = select i1 %w1c, i32 0, i32 %q1.i
  %k1 = zext i1 %w1c to i32
  %q2.i = add i32 %q2, %k1
  %w2c = icmp eq i32 %q2.i, %cnt2
  %q2.n = select i1 %w2c, i32 0, i32 %q2.i
  %k2 = zext i1 %w2c to i32
  %q3.i = add i32 %q3, %k2
  %w3c = icmp eq i32 %q3.i, %cnt3
  %q3.n = select i1 %w3c, i32 0, i32 %q3.i
  %k3 = zext i1 %w3c to i32
  %q4.n = add i32 %q4, %k3
  %e.next = add i32 %e, 1
  %more = icmp ult i32 %e.next, %count
  br i1 %more, label %loop, label %exit
exit:
  %hits.f = phi i32 [ 0, %elements ], [ %hits.n, %next ]
  %result = select i1 %inspecting, i32 %hits.f, i32 %bytes
  ret i32 %result
}

; The .report_mechanism inspection of an element read from global memory at %a
; (%n bytes): code 1 .per_element::ff matches a 0xff byte; 2 to 5
; .per_16bytes::80000000, 8000, 80 and 8 match the first element (32, 16, 8 or
; 4 bits) of the aligned 16-byte chunk that holds it
define zeroext i1 @__zluda_ptx_impl_agx_tma_inspect(i64 %a, i32 %n, i32 %code) #3 {
entry:
  %bytes = icmp eq i32 %code, 1
  br i1 %bytes, label %loop, label %chunk
loop:
  %j = phi i32 [ 0, %entry ], [ %j.next, %loop ]
  %j64 = zext i32 %j to i64
  %ja = add i64 %a, %j64
  %jp = inttoptr i64 %ja to ptr addrspace(1)
  %b = load i8, ptr addrspace(1) %jp, align 1
  %ff = icmp eq i8 %b, -1
  %j.next = add i32 %j, 1
  %more = icmp ult i32 %j.next, %n
  %go = select i1 %ff, i1 false, i1 %more
  br i1 %go, label %loop, label %bytes_done
bytes_done:
  ret i1 %ff
chunk:
  %c = and i64 %a, -16
  %cp = inttoptr i64 %c to ptr addrspace(1)
  %v = load i32, ptr addrspace(1) %cp, align 16
  %is2 = icmp eq i32 %code, 2
  %is3 = icmp eq i32 %code, 3
  %is4 = icmp eq i32 %code, 4
  %m0 = select i1 %is4, i32 255, i32 15
  %m1 = select i1 %is3, i32 65535, i32 %m0
  %mask = select i1 %is2, i32 -1, i32 %m1
  %p0 = select i1 %is4, i32 128, i32 8
  %p1 = select i1 %is3, i32 32768, i32 %p0
  %pattern = select i1 %is2, i32 -2147483648, i32 %p1
  %vm = and i32 %v, %mask
  %hit = icmp eq i32 %vm, %pattern
  ret i1 %hit
}


; The packed sub-byte element types (elemtype 13 b4x16, 14 b4x16_p64,
; 15 b6x16_p32): dim 0 counts elements, a group of 16 takes 8 (4-bit) or 12
; (6-bit) bytes in global memory and 8 (b4x16) or 16 bytes (data, then zero
; padding) in shared memory; groups are laid out like the elements of a tile,
; and a group outside the tensor reads as zeros. Returns the bytes of the box.
; the elements of a box dimension of `box` elements with element stride `stride`
define i32 @__zluda_ptx_impl_agx_tma_count(i32 %box, i32 %stride) #0 {
  %zero = icmp eq i32 %stride, 0
  %s = select i1 %zero, i32 1, i32 %stride
  %t = add i32 %box, %s
  %t1 = sub i32 %t, 1
  %n = udiv i32 %t1, %s
  ret i32 %n
}

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
  ; .report_mechanism (flags bits 8-15, tile mode): a report-on before the complete-tx
  %code_s = lshr i32 %flags, 8
  %code = and i32 %code_s, 255
  %has_code = icmp ne i32 %code, 0
  %is_tile = icmp slt i32 %imode, 0
  %inspect = and i1 %has_code, %is_tile
  br i1 %inspect, label %inspecting, label %complete
inspecting:
  %dir = sub i32 -3, %code
  %hits = call i32 @__zluda_ptx_impl_agx_tma_tile(ptr %tm, i32 %c0, i32 %c1, i32 %c2, i32 %c3, i32 %c4, i32 %dst_a, i32 %dir)
  %hit = icmp ne i32 %hits, 0
  br i1 %hit, label %report, label %complete
report:
  call void @__zluda_ptx_impl_agx_mbarrier_report_on(ptr addrspace(3) %mbar)
  br label %complete
complete:
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
  br label %start, !llvm.loop !50000
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
  br i1 %pbad, label %outside, label %pos, !llvm.loop !50002
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
  br i1 %wbad, label %outside, label %wpos, !llvm.loop !50003
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
  ; 32B, and 96B (4) with the pattern of 32B (PTX ISA 5.5.7, Figure 31)
  %is32b = icmp eq i32 %swzm, 1
  %is96 = icmp eq i32 %swzm, 4
  %is32 = or i1 %is32b, %is96
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
declare float @llvm.agx.exp2.f32(float)
declare float @llvm.agx.log2.f32(float)
declare {i32, i1} @llvm.uadd.with.overflow.i32(i32, i32)
declare void @llvm.trap()

; The texture unit's sample of (x, y) for tex.1d / tex.2d (zluda_ptx_impl_agx_tex.c): `native` is word 16 of a
; texture descriptor, the sampler index at bit 32 and the texture heap offset in the low 32 bits, bit 61 for 2D.
define <4 x i32> @__zluda_ptx_impl_agx_tex_unit(i64 %native, float %x, float %y) #5 {
  %offset = trunc i64 %native to i32
  %s64 = lshr i64 %native, 32
  %sampler = trunc i64 %s64 to i16
  %d64 = lshr i64 %native, 61
  %d1 = trunc i64 %d64 to i1
  br i1 %d1, label %two, label %one
one:
  %r1 = call <4 x i32> @llvm.agx.texture.sample(i32 %offset, i16 %sampler, float %x, float poison, i32 0)
  ret <4 x i32> %r1
two:
  %r2 = call <4 x i32> @llvm.agx.texture.sample(i32 %offset, i16 %sampler, float %x, float %y, i32 2)
  ret <4 x i32> %r2
}

declare <4 x i32> @llvm.agx.texture.sample(i32, i16, float, float, i32 immarg)

attributes #0 = { alwaysinline nounwind willreturn memory(none) }
attributes #1 = { alwaysinline convergent nounwind }
attributes #2 = { alwaysinline convergent nounwind willreturn memory(none) }
attributes #3 = { alwaysinline nounwind }
attributes #4 = { nounwind willreturn memory(none) }
attributes #5 = { alwaysinline nounwind willreturn memory(read) }

declare i32 @llvm.agx.simdgroup.index.in.threadgroup()

declare i32 @llvm.agx.core.id()

declare void @llvm.memcpy.p3.p1.i32(ptr addrspace(3) noalias nocapture writeonly, ptr addrspace(1) noalias nocapture readonly, i32, i1 immarg)
; The loops over the dimensions of a TMA im2col copy (__zluda_ptx_impl_agx_tma_*): neither unrolled nor peeled. Peeling
; them for compares a constant tensor map decides grew the copy by a third (tma-i2c-4d: 1050 AGX instructions against
; 768) for straight-line code the copy runs once per element.
!50000 = distinct !{!50000, !50001}
!50001 = !{!"llvm.loop.unroll.disable"}
!50002 = distinct !{!50002, !50001}
!50003 = distinct !{!50003, !50001}

; ---------------------------------------------------------------- BEGIN GENERATED (gen_agx_tex.py)
; regenerate with ptx/lib/gen_agx_tex.py

; from zluda_ptx_impl_agx_tex.c

%struct.q = type { i32, [3 x i32], i32, i32 }

; Function Attrs: alwaysinline convergent nounwind
define dso_local <4 x i32> @__zluda_ptx_impl_agx_tex(ptr addrspace(1) nofree noundef readonly captures(none) %t, ptr addrspace(1) nofree noundef readonly captures(address_is_null) %s, i32 noundef %geom, i32 noundef %lod_mode, i32 noundef %ctype_float, i32 noundef %dtype, i32 noundef %c0, i32 noundef %c1, i32 noundef %c2, i32 noundef %c3, float noundef %lod, float noundef %dx0, float noundef %dx1, float noundef %dx2, float noundef %dy0, float noundef %dy1, float noundef %dy2, i32 noundef %o0, i32 noundef %o1, i32 noundef %o2, i32 noundef %has_ref, i32 noundef %ref, i32 noundef %unit) local_unnamed_addr #3 {
entry:
  %q2.i597 = alloca %struct.q, align 4
  %q2.i = alloca %struct.q, align 4
  %q = alloca %struct.q, align 4
  %dx = alloca [3 x double], align 8
  %dy = alloca [3 x double], align 8
  %tobool.not = icmp eq i32 %unit, 0
  br i1 %tobool.not, label %if.end38, label %if.then

if.then:                                          ; preds = %entry
  %arrayidx.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 128
  %0 = load i64, ptr addrspace(1) %arrayidx.i, align 8
  %tobool1.not = icmp sgt i64 %0, -1
  br i1 %tobool1.not, label %if.end, label %land.lhs.true

land.lhs.true:                                    ; preds = %if.then
  %1 = and i64 %0, 4611686018427387904
  %2 = icmp ne i64 %1, 0
  %3 = icmp eq i32 %dtype, 0
  %cmp4 = xor i1 %3, %2
  br i1 %cmp4, label %land.lhs.true6, label %if.end

land.lhs.true6:                                   ; preds = %land.lhs.true
  %4 = and i64 %0, 2305843009213693952
  %5 = icmp ne i64 %4, 0
  %6 = icmp ne i32 %geom, 1
  %cmp12 = xor i1 %6, %5
  br i1 %cmp12, label %if.then14, label %if.end

if.then14:                                        ; preds = %land.lhs.true6
  %7 = bitcast i32 %c0 to float
  %8 = bitcast i32 %c1 to float
  %call19 = tail call <4 x i32> @__zluda_ptx_impl_agx_tex_unit(i64 noundef %0, float noundef %7, float noundef %8)
  br label %return

if.end:                                           ; preds = %land.lhs.true6, %land.lhs.true, %if.then
  %cmp20 = icmp eq i32 %dtype, 0
  br i1 %cmp20, label %if.then22, label %if.end38

if.then22:                                        ; preds = %if.end
  %tobool.not.i = icmp eq ptr addrspace(1) %s, null
  br i1 %tobool.not.i, label %if.else.i, label %if.then.i

if.then.i:                                        ; preds = %if.then22
  %arrayidx.i.i = getelementptr inbounds nuw i8, ptr addrspace(1) %s, i64 8
  %9 = load i64, ptr addrspace(1) %arrayidx.i.i, align 8
  %arrayidx.i38.i = getelementptr inbounds nuw i8, ptr addrspace(1) %s, i64 16
  %10 = load i64, ptr addrspace(1) %arrayidx.i38.i, align 8
  %arrayidx.i39.i = getelementptr inbounds nuw i8, ptr addrspace(1) %s, i64 24
  %11 = load i64, ptr addrspace(1) %arrayidx.i39.i, align 8
  %tobool4.not.i = icmp eq i64 %11, 0
  br i1 %tobool4.not.i, label %sampling_of.exit, label %land.rhs.i

land.rhs.i:                                       ; preds = %if.then.i
  %arrayidx.i40.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 48
  %12 = load i64, ptr addrspace(1) %arrayidx.i40.i, align 8
  %tobool6.i = icmp ne i64 %12, 0
  %13 = zext i1 %tobool6.i to i32
  br label %sampling_of.exit

if.else.i:                                        ; preds = %if.then22
  %arrayidx.i44.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 32
  %14 = load i64, ptr addrspace(1) %arrayidx.i44.i, align 8
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 40
  %15 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 48
  %16 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %16 to i32
  br label %sampling_of.exit

sampling_of.exit:                                 ; preds = %if.then.i, %land.rhs.i, %if.else.i
  %retval.sroa.6.0.i = phi i32 [ %conv21.i, %if.else.i ], [ 0, %if.then.i ], [ %13, %land.rhs.i ]
  %retval.sroa.3.0.in.i = phi i64 [ %15, %if.else.i ], [ %10, %if.then.i ], [ %10, %land.rhs.i ]
  %retval.sroa.0.0.in.i = phi i64 [ %14, %if.else.i ], [ %9, %if.then.i ], [ %9, %land.rhs.i ]
  %retval.sroa.0.0.i = trunc i64 %retval.sroa.0.0.in.i to i32
  %retval.sroa.3.0.i = trunc i64 %retval.sroa.3.0.in.i to i32
  %17 = bitcast i32 %c0 to float
  %18 = bitcast i32 %c1 to float
  %cmp24 = icmp eq i32 %geom, 1
  %cond = select i1 %cmp24, i32 2, i32 1
  %call26 = tail call fastcc <4 x float> @__zluda_agx_tex_sample_f32(ptr addrspace(1) noundef %t, i32 noundef %retval.sroa.0.0.i, i32 noundef %retval.sroa.3.0.i, i32 noundef %retval.sroa.6.0.i, float noundef %17, float noundef %18, i32 noundef %cond)
  %vecinit36 = bitcast <4 x float> %call26 to <4 x i32>
  br label %return

if.end38:                                         ; preds = %if.end, %entry
  %tobool.not.i341 = icmp eq ptr addrspace(1) %s, null
  br i1 %tobool.not.i341, label %if.else.i375, label %if.then.i342

if.then.i342:                                     ; preds = %if.end38
  %arrayidx.i.i343 = getelementptr inbounds nuw i8, ptr addrspace(1) %s, i64 8
  %19 = load i64, ptr addrspace(1) %arrayidx.i.i343, align 8
  %arrayidx.i38.i344 = getelementptr inbounds nuw i8, ptr addrspace(1) %s, i64 16
  %20 = load i64, ptr addrspace(1) %arrayidx.i38.i344, align 8
  %arrayidx.i39.i345 = getelementptr inbounds nuw i8, ptr addrspace(1) %s, i64 24
  %21 = load i64, ptr addrspace(1) %arrayidx.i39.i345, align 8
  %tobool4.not.i346 = icmp eq i64 %21, 0
  br i1 %tobool4.not.i346, label %land.end.i350, label %land.rhs.i347

land.rhs.i347:                                    ; preds = %if.then.i342
  %arrayidx.i40.i348 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 48
  %22 = load i64, ptr addrspace(1) %arrayidx.i40.i348, align 8
  %tobool6.i349 = icmp ne i64 %22, 0
  %23 = zext i1 %tobool6.i349 to i32
  br label %land.end.i350

land.end.i350:                                    ; preds = %land.rhs.i347, %if.then.i342
  %land.ext.i351 = phi i32 [ 0, %if.then.i342 ], [ %23, %land.rhs.i347 ]
  %arrayidx.i41.i352 = getelementptr inbounds nuw i8, ptr addrspace(1) %s, i64 32
  %arrayidx.i42.i353 = getelementptr inbounds nuw i8, ptr addrspace(1) %s, i64 40
  %24 = load <2 x i64>, ptr addrspace(1) %arrayidx.i42.i353, align 8
  %25 = sitofp <2 x i64> %24 to <2 x double>
  %26 = fmul nnan <2 x double> %25, splat (double 3.906250e-03)
  br label %sampling_of.exit381

if.else.i375:                                     ; preds = %if.end38
  %arrayidx.i44.i376 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 32
  %27 = load i64, ptr addrspace(1) %arrayidx.i44.i376, align 8
  %arrayidx.i45.i377 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 40
  %28 = load i64, ptr addrspace(1) %arrayidx.i45.i377, align 8
  %arrayidx.i46.i378 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 48
  %29 = load i64, ptr addrspace(1) %arrayidx.i46.i378, align 8
  %conv21.i379 = trunc i64 %29 to i32
  %arrayidx.i47.i380 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 120
  br label %sampling_of.exit381

sampling_of.exit381:                              ; preds = %land.end.i350, %if.else.i375
  %retval.sroa.9.0.in.in.i359 = phi ptr addrspace(1) [ %arrayidx.i41.i352, %land.end.i350 ], [ %arrayidx.i47.i380, %if.else.i375 ]
  %retval.sroa.6.0.i360 = phi i32 [ %land.ext.i351, %land.end.i350 ], [ %conv21.i379, %if.else.i375 ]
  %retval.sroa.3.0.in.i361 = phi i64 [ %20, %land.end.i350 ], [ %28, %if.else.i375 ]
  %retval.sroa.0.0.in.i362 = phi i64 [ %19, %land.end.i350 ], [ %27, %if.else.i375 ]
  %30 = phi <2 x double> [ %26, %land.end.i350 ], [ <double 0.000000e+00, double 4.096000e+03>, %if.else.i375 ]
  %retval.sroa.9.0.in.i365 = load i64, ptr addrspace(1) %retval.sroa.9.0.in.in.i359, align 8
  %retval.sroa.0.0.i366 = trunc i64 %retval.sroa.0.0.in.i362 to i32
  %retval.sroa.3.0.i367 = trunc i64 %retval.sroa.3.0.in.i361 to i32
  %retval.sroa.9.0.i368 = trunc i64 %retval.sroa.9.0.in.i365 to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q)
  store i32 0, ptr %q, align 4
  %off = getelementptr inbounds nuw i8, ptr %q, i64 4
  store i32 %o0, ptr %off, align 4
  %arrayinit.element = getelementptr inbounds nuw i8, ptr %q, i64 8
  store i32 %o1, ptr %arrayinit.element, align 4
  %arrayinit.element41 = getelementptr inbounds nuw i8, ptr %q, i64 12
  store i32 %o2, ptr %arrayinit.element41, align 4
  %has_ref42 = getelementptr inbounds nuw i8, ptr %q, i64 16
  store i32 %has_ref, ptr %has_ref42, align 4
  %ref43 = getelementptr inbounds nuw i8, ptr %q, i64 20
  store i32 %ref, ptr %ref43, align 4
  call void @llvm.lifetime.start.p0(ptr nonnull %dx)
  %conv44 = fpext float %dx0 to double
  store double %conv44, ptr %dx, align 8
  %arrayinit.element45 = getelementptr inbounds nuw i8, ptr %dx, i64 8
  %conv46 = fpext float %dx1 to double
  store double %conv46, ptr %arrayinit.element45, align 8
  %arrayinit.element47 = getelementptr inbounds nuw i8, ptr %dx, i64 16
  %conv48 = fpext float %dx2 to double
  store double %conv48, ptr %arrayinit.element47, align 8
  call void @llvm.lifetime.start.p0(ptr nonnull %dy)
  %conv49 = fpext float %dy0 to double
  store double %conv49, ptr %dy, align 8
  %arrayinit.element50 = getelementptr inbounds nuw i8, ptr %dy, i64 8
  %conv51 = fpext float %dy1 to double
  store double %conv51, ptr %arrayinit.element50, align 8
  %arrayinit.element52 = getelementptr inbounds nuw i8, ptr %dy, i64 16
  %conv53 = fpext float %dy2 to double
  store double %conv53, ptr %arrayinit.element52, align 8
  %31 = add i32 %geom, -7
  %or.cond = icmp ult i32 %31, 2
  br i1 %or.cond, label %if.then62, label %if.else

if.then62:                                        ; preds = %sampling_of.exit381
  %cmp60 = icmp eq i32 %geom, 8
  %arrayidx.i.i382 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 104
  %32 = load i64, ptr addrspace(1) %arrayidx.i.i382, align 8
  %shr.i = lshr i64 %32, 8
  %and.i = and i64 %shr.i, 255
  %33 = tail call i64 @llvm.umax.i64(i64 %and.i, i64 1)
  %34 = select i1 %cmp60, i32 %c1, i32 %c0
  %conv67 = zext i32 %34 to i64
  %sub = add nsw i64 %33, -1
  %35 = tail call i64 @llvm.umin.i64(i64 %conv67, i64 %sub)
  %36 = select i1 %cmp60, i32 %c2, i32 %c1
  %conv72 = sext i32 %36 to i64
  %conv73 = sext i32 %o0 to i64
  %add74 = add nsw i64 %conv72, %conv73
  %arrayidx.i.i384 = getelementptr i8, ptr addrspace(1) %t, i64 72
  %37 = load i64, ptr addrspace(1) %arrayidx.i.i384, align 8
  %cond.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %37, i64 1)
  switch i32 %retval.sroa.3.0.i367, label %ad_idx.exit [
    i32 0, label %ad_idx.exit.thread
    i32 2, label %ad_idx.exit.thread718
    i32 3, label %ad_idx.exit.thread729
  ]

ad_idx.exit.thread:                               ; preds = %if.then62
  %38 = select i1 %cmp60, i32 %c3, i32 %c2
  %conv81708 = sext i32 %38 to i64
  %conv82709 = sext i32 %o1 to i64
  %add83710 = add nsw i64 %conv81708, %conv82709
  %arrayidx.i.i387711 = getelementptr i8, ptr addrspace(1) %t, i64 80
  %39 = load i64, ptr addrspace(1) %arrayidx.i.i387711, align 8
  %cond.i.i388712 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %39, i64 1)
  %40 = insertelement <2 x i64> poison, i64 %39, i64 0
  %41 = insertelement <2 x i64> %40, i64 %cond.i.i, i64 1
  %42 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %41, <2 x i64> <i64 1, i64 -9223372036854775808>)
  %43 = insertelement <2 x i64> poison, i64 %add83710, i64 0
  %44 = insertelement <2 x i64> %43, i64 %add74, i64 1
  %45 = srem <2 x i64> %44, %42
  %46 = add nsw <2 x i64> %45, %42
  %47 = srem <2 x i64> %46, %42
  br label %ad_idx.exit409

ad_idx.exit.thread718:                            ; preds = %if.then62
  %48 = select i1 %cmp60, i32 %c3, i32 %c2
  %conv81721 = sext i32 %48 to i64
  %conv82722 = sext i32 %o1 to i64
  %add83723 = add nsw i64 %conv81721, %conv82722
  %arrayidx.i.i387724 = getelementptr i8, ptr addrspace(1) %t, i64 80
  %49 = load i64, ptr addrspace(1) %arrayidx.i.i387724, align 8
  %cond.i.i388725 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %49, i64 1)
  %50 = insertelement <2 x i64> poison, i64 %49, i64 0
  %51 = insertelement <2 x i64> %50, i64 %cond.i.i, i64 1
  %52 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %51, <2 x i64> <i64 1, i64 -9223372036854775808>)
  %53 = insertelement <2 x i64> poison, i64 %add83723, i64 0
  %54 = insertelement <2 x i64> %53, i64 %add74, i64 1
  %55 = icmp slt <2 x i64> %54, %52
  %56 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %54, <2 x i64> splat (i64 -1))
  %57 = select <2 x i1> %55, <2 x i64> %56, <2 x i64> splat (i64 -1)
  br label %ad_idx.exit409

ad_idx.exit.thread729:                            ; preds = %if.then62
  %58 = select i1 %cmp60, i32 %c3, i32 %c2
  %conv81732 = sext i32 %58 to i64
  %conv82733 = sext i32 %o1 to i64
  %add83734 = add nsw i64 %conv81732, %conv82733
  %arrayidx.i.i387735 = getelementptr i8, ptr addrspace(1) %t, i64 80
  %59 = load i64, ptr addrspace(1) %arrayidx.i.i387735, align 8
  %cond.i.i388736 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %59, i64 1)
  %60 = insertelement <2 x i64> poison, i64 %59, i64 0
  %61 = insertelement <2 x i64> %60, i64 %cond.i.i, i64 1
  %62 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %61, <2 x i64> <i64 1, i64 -9223372036854775808>)
  %63 = shl nuw nsw <2 x i64> %62, splat (i64 1)
  %64 = insertelement <2 x i64> poison, i64 %add83734, i64 0
  %65 = insertelement <2 x i64> %64, i64 %add74, i64 1
  %66 = srem <2 x i64> %65, %63
  %67 = add nsw <2 x i64> %63, %66
  %68 = srem <2 x i64> %67, %63
  %69 = icmp slt <2 x i64> %68, %62
  %70 = xor <2 x i64> %68, splat (i64 -1)
  %71 = add <2 x i64> %63, %70
  %72 = select <2 x i1> %69, <2 x i64> %68, <2 x i64> %71
  br label %ad_idx.exit409

ad_idx.exit:                                      ; preds = %if.then62
  %73 = select i1 %cmp60, i32 %c3, i32 %c2
  %conv81 = sext i32 %73 to i64
  %conv82 = sext i32 %o1 to i64
  %add83 = add nsw i64 %conv81, %conv82
  %arrayidx.i.i387 = getelementptr i8, ptr addrspace(1) %t, i64 80
  %74 = load i64, ptr addrspace(1) %arrayidx.i.i387, align 8
  %cond.i.i388 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %74, i64 1)
  %75 = insertelement <2 x i64> poison, i64 %74, i64 0
  %76 = insertelement <2 x i64> %75, i64 %cond.i.i, i64 1
  %77 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %76, <2 x i64> <i64 1, i64 -9223372036854775808>)
  %78 = insertelement <2 x i64> poison, i64 %add83, i64 0
  %79 = insertelement <2 x i64> %78, i64 %add74, i64 1
  %80 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %79, <2 x i64> zeroinitializer)
  %81 = add nsw <2 x i64> %77, splat (i64 -1)
  %82 = tail call <2 x i64> @llvm.umin.v2i64(<2 x i64> %80, <2 x i64> %81)
  br label %ad_idx.exit409

ad_idx.exit409:                                   ; preds = %ad_idx.exit.thread, %ad_idx.exit.thread718, %ad_idx.exit.thread729, %ad_idx.exit
  %cond.i.i388717 = phi i64 [ %cond.i.i388, %ad_idx.exit ], [ %cond.i.i388712, %ad_idx.exit.thread ], [ %cond.i.i388725, %ad_idx.exit.thread718 ], [ %cond.i.i388736, %ad_idx.exit.thread729 ]
  %83 = phi <2 x i64> [ %82, %ad_idx.exit ], [ %47, %ad_idx.exit.thread ], [ %57, %ad_idx.exit.thread718 ], [ %72, %ad_idx.exit.thread729 ]
  %84 = getelementptr i8, ptr addrspace(1) %t, i64 8
  br i1 %cmp60, label %cond.true, label %cond.end

cond.true:                                        ; preds = %ad_idx.exit409
  %conv.i410 = zext i32 %c0 to i64
  %arrayidx.i.i.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 64
  %85 = load i64, ptr addrspace(1) %arrayidx.i.i.i, align 8
  %cond.i.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %85, i64 1)
  %sub.i = add nsw i64 %cond.i.i.i, -1
  %86 = tail call range(i64 0, 4294967296) i64 @llvm.umin.i64(i64 %conv.i410, i64 %sub.i)
  %87 = mul nuw nsw i64 %86, %cond.i.i388717
  br label %cond.end

cond.end:                                         ; preds = %ad_idx.exit409, %cond.true
  %cond88 = phi i64 [ %87, %cond.true ], [ 0, %ad_idx.exit409 ]
  %88 = icmp sgt <2 x i64> %83, splat (i64 -1)
  %89 = extractelement <2 x i1> %88, i64 0
  %90 = extractelement <2 x i1> %88, i64 1
  %or.cond247 = select i1 %90, i1 %89, i1 false
  br i1 %or.cond247, label %if.then94, label %if.end226

if.then94:                                        ; preds = %cond.end
  %arrayidx.i411 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 24
  %91 = load i64, ptr addrspace(1) %arrayidx.i411, align 8
  %arrayidx.i412 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 16
  %92 = load i64, ptr addrspace(1) %arrayidx.i412, align 8
  switch i64 %92, label %sw.default.i415 [
    i64 0, label %code_bytes.exit
    i64 20, label %code_bytes.exit
    i64 1, label %sw.bb1.i413
    i64 21, label %sw.bb1.i413
    i64 6, label %sw.bb1.i413
  ]

sw.bb1.i413:                                      ; preds = %if.then94, %if.then94, %if.then94
  br label %code_bytes.exit

sw.default.i415:                                  ; preds = %if.then94
  br label %code_bytes.exit

code_bytes.exit:                                  ; preds = %if.then94, %if.then94, %sw.bb1.i413, %sw.default.i415
  %retval.0.i414 = phi i64 [ 4, %sw.default.i415 ], [ 2, %sw.bb1.i413 ], [ 1, %if.then94 ], [ 1, %if.then94 ]
  %93 = extractelement <2 x i64> %83, i64 0
  %add99 = add nuw nsw i64 %cond88, %93
  %mul101 = mul nsw i64 %add99, %cond.i.i
  %94 = extractelement <2 x i64> %83, i64 1
  %add102 = add nuw nsw i64 %mul101, %94
  %mul103 = mul nsw i64 %add102, %33
  %add104 = add nuw nsw i64 %mul103, %35
  %95 = load i64, ptr addrspace(1) %84, align 8
  %mul106 = mul nsw i64 %91, %add104
  %mul108 = mul nsw i64 %mul106, %retval.0.i414
  %add109 = add i64 %mul108, %95
  br label %for.body

for.body:                                         ; preds = %code_bytes.exit, %cond.end124
  %indvars.iv = phi i64 [ 0, %code_bytes.exit ], [ %indvars.iv.next, %cond.end124 ]
  %r57.0780 = phi <4 x double> [ zeroinitializer, %code_bytes.exit ], [ %vecins, %cond.end124 ]
  %cmp113 = icmp sgt i64 %91, %indvars.iv
  br i1 %cmp113, label %cond.true115, label %cond.false120

cond.true115:                                     ; preds = %for.body
  %96 = mul nuw nsw i64 %indvars.iv, %retval.0.i414
  %add118 = add i64 %add109, %96
  %call119 = tail call fastcc double @__zluda_agx_tex_channel(ptr addrspace(1) noundef %t, i64 noundef %add118)
  br label %cond.end124

cond.false120:                                    ; preds = %for.body
  %cmp121 = icmp eq i64 %indvars.iv, 3
  %cond123 = select i1 %cmp121, double 1.000000e+00, double 0.000000e+00
  br label %cond.end124

cond.end124:                                      ; preds = %cond.false120, %cond.true115
  %cond125 = phi double [ %call119, %cond.true115 ], [ %cond123, %cond.false120 ]
  %97 = trunc nuw nsw i64 %indvars.iv to i32
  %vecins = insertelement <4 x double> %r57.0780, double %cond125, i32 %97
  %indvars.iv.next = add nuw nsw i64 %indvars.iv, 1
  %exitcond.not = icmp eq i64 %indvars.iv.next, 4
  br i1 %exitcond.not, label %if.end226, label %for.body, !llvm.loop !10016

if.else:                                          ; preds = %sampling_of.exit381
  %98 = icmp ult i32 %31, -2
  br i1 %98, label %if.else167, label %if.then130

if.then130:                                       ; preds = %if.else
  %cmp132.not = icmp eq i32 %geom, 6
  %99 = bitcast i32 %c1 to float
  %100 = bitcast i32 %c0 to float
  %101 = select i1 %cmp132.not, float %99, float %100
  %conv.i421 = fpext float %101 to double
  %102 = bitcast i32 %c2 to float
  %103 = select i1 %cmp132.not, float %102, float %99
  %conv.i422 = fpext float %103 to double
  %104 = bitcast i32 %c3 to float
  %105 = select i1 %cmp132.not, float %104, float %102
  %conv.i423 = fpext float %105 to double
  %106 = tail call double @llvm.fabs.f64(double %conv.i421)
  %107 = tail call double @llvm.fabs.f64(double %conv.i422)
  %108 = tail call double @llvm.fabs.f64(double %conv.i423)
  %cmp.i.i = fcmp ult double %106, %107
  %cmp1.i.i = fcmp ult double %106, %108
  %or.cond.i.i = or i1 %cmp.i.i, %cmp1.i.i
  br i1 %or.cond.i.i, label %if.end.i.i, label %if.then.i.i

if.then.i.i:                                      ; preds = %if.then130
  %cmp2.i.i = fcmp ult float %101, 0.000000e+00
  %cond.i.i424 = zext i1 %cmp2.i.i to i32
  br label %cube_ma.exit

if.end.i.i:                                       ; preds = %if.then130
  %cmp3.i.i = fcmp ule double %107, %106
  %cmp5.i.i = fcmp ult double %107, %108
  %or.cond23.i.i = or i1 %cmp3.i.i, %cmp5.i.i
  br i1 %or.cond23.i.i, label %if.end9.i.i, label %if.then6.i.i

if.then6.i.i:                                     ; preds = %if.end.i.i
  %cmp7.i.i = fcmp oge float %103, 0.000000e+00
  %cond8.i.i = select i1 %cmp7.i.i, i32 2, i32 3
  br label %cube_ma.exit

if.end9.i.i:                                      ; preds = %if.end.i.i
  %cmp10.i.i = fcmp oge float %105, 0.000000e+00
  %cond11.i.i = select i1 %cmp10.i.i, i32 4, i32 5
  br label %cube_ma.exit

cube_ma.exit:                                     ; preds = %if.then.i.i, %if.then6.i.i, %if.end9.i.i
  %retval.0.i.i = phi i32 [ %cond.i.i424, %if.then.i.i ], [ %cond8.i.i, %if.then6.i.i ], [ %cond11.i.i, %if.end9.i.i ]
  %cmp.i = icmp samesign ult i32 %retval.0.i.i, 2
  %cmp1.i = icmp samesign ult i32 %retval.0.i.i, 4
  %cond.v.i = select i1 %cmp1.i, double %conv.i422, double %conv.i423
  %cond.i425 = tail call double @llvm.fabs.f64(double %cond.v.i)
  %cond5.i = select i1 %cmp.i, double %106, double %cond.i425
  %cmp148 = icmp eq i32 %lod_mode, 1
  br i1 %cmp148, label %cond.true150, label %cond.false152

cond.true150:                                     ; preds = %cube_ma.exit
  %conv151 = fpext float %lod to double
  br label %cond.end156

cond.false152:                                    ; preds = %cube_ma.exit
  %arrayidx.i.i.i426 = getelementptr i8, ptr addrspace(1) %t, i64 72
  %109 = load <2 x i64>, ptr addrspace(1) %arrayidx.i.i.i426, align 8
  %110 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %109, <2 x i64> splat (i64 1))
  %arrayidx.i.i.i426.2 = getelementptr i8, ptr addrspace(1) %t, i64 88
  %111 = load i64, ptr addrspace(1) %arrayidx.i.i.i426.2, align 8
  %cond.i.i.i427.2 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %111, i64 1)
  %conv.i428.2 = uitofp nneg i64 %cond.i.i.i427.2 to double
  %.scalar = fmul double %cond5.i, 2.000000e+00
  %112 = insertelement <2 x double> <double poison, double 1.000000e+00>, double %.scalar, i64 0
  %113 = shufflevector <2 x double> %112, <2 x double> poison, <4 x i32> <i32 0, i32 1, i32 0, i32 0>
  %114 = shufflevector <2 x i64> %110, <2 x i64> poison, <4 x i32> <i32 0, i32 poison, i32 1, i32 poison>
  %115 = uitofp <4 x i64> %114 to <4 x double>
  %116 = insertelement <4 x double> %115, double 0.000000e+00, i64 1
  %117 = insertelement <4 x double> %116, double %conv.i428.2, i64 3
  %118 = fdiv <4 x double> %117, %113
  %119 = insertelement <4 x double> <double poison, double 1.000000e+00, double poison, double poison>, double %conv44, i64 0
  %120 = insertelement <4 x double> %119, double %conv46, i64 2
  %121 = insertelement <4 x double> %120, double %conv48, i64 3
  %122 = fmul <4 x double> %118, %121
  %123 = insertelement <4 x double> %122, double 1.000000e+00, i64 1
  %124 = fmul <4 x double> %122, %123
  %125 = tail call double @llvm.vector.reduce.fadd.v4f64(double -0.000000e+00, <4 x double> %124)
  %126 = insertelement <4 x double> %118, double 0.000000e+00, i64 1
  %127 = insertelement <4 x double> <double poison, double 1.000000e+00, double poison, double poison>, double %conv49, i64 0
  %128 = insertelement <4 x double> %127, double %conv51, i64 2
  %129 = insertelement <4 x double> %128, double %conv53, i64 3
  %130 = fmul <4 x double> %126, %129
  %131 = insertelement <4 x double> %118, double 1.000000e+00, i64 1
  %132 = fmul <4 x double> %131, %129
  %133 = fmul <4 x double> %130, %132
  %134 = tail call double @llvm.vector.reduce.fadd.v4f64(double -0.000000e+00, <4 x double> %133)
  %cmp.i.i432 = fcmp oge double %125, %134
  %cond.i.i433 = select i1 %cmp.i.i432, double %125, double %134
  %cmp10.i = fcmp ogt double %cond.i.i433, 0.000000e+00
  br i1 %cmp10.i, label %cond.true12.i, label %cond.end156

cond.true12.i:                                    ; preds = %cond.false152
  %135 = bitcast double %cond.i.i433 to i64
  %shr.i.i = lshr i64 %135, 52
  %cmp.i29.i = icmp eq i64 %shr.i.i, 0
  %mul.i.i = fmul double %cond.i.i433, f0x4350000000000000
  %136 = bitcast double %mul.i.i to i64
  %.pre-phi.i.i = select i1 %cmp.i29.i, i64 %136, i64 %135
  %and6.i.i = and i64 %.pre-phi.i.i, 4503599627370495
  %or.i.i = or disjoint i64 %and6.i.i, 4607182418800017408
  %137 = bitcast i64 %or.i.i to double
  %cmp7.i.i434 = fcmp ogt double %137, f0x3FF6A09E667F3BCD
  %mul10.i.i = fmul nnan double %137, 5.000000e-01
  %m.0.i.i = select i1 %cmp7.i.i434, double %mul10.i.i, double %137
  %sub12.i.i = fadd double %m.0.i.i, -1.000000e+00
  %add.i.i435 = fadd double %m.0.i.i, 1.000000e+00
  %div.i.i = fdiv double %sub12.i.i, %add.i.i435
  %mul13.i.i = fmul double %div.i.i, %div.i.i
  br label %for.body.i.i

for.body.i.i:                                     ; preds = %for.body.i.i, %cond.true12.i
  %k.041.i.i = phi i32 [ 1, %cond.true12.i ], [ %add20.i.i, %for.body.i.i ]
  %sum.040.i.i = phi double [ 0.000000e+00, %cond.true12.i ], [ %add18.i.i, %for.body.i.i ]
  %term.039.i.i = phi double [ %div.i.i, %cond.true12.i ], [ %mul19.i.i, %for.body.i.i ]
  %conv16.i.i = uitofp nneg i32 %k.041.i.i to double
  %div17.i.i = fdiv double %term.039.i.i, %conv16.i.i
  %add18.i.i = fadd double %sum.040.i.i, %div17.i.i
  %mul19.i.i = fmul double %mul13.i.i, %term.039.i.i
  %add20.i.i = add nuw nsw i32 %k.041.i.i, 2
  %cmp14.i.i = icmp samesign ult i32 %k.041.i.i, 38
  br i1 %cmp14.i.i, label %for.body.i.i, label %log2_d.exit.i, !llvm.loop !10019

log2_d.exit.i:                                    ; preds = %for.body.i.i
  %138 = trunc nuw nsw i64 %shr.i.i to i32
  %shr2.i.i = lshr i64 %136, 52
  %139 = trunc nuw nsw i64 %shr2.i.i to i32
  %sub.i.i = add nsw i32 %139, -54
  %e.0.i.i = select i1 %cmp.i29.i, i32 %sub.i.i, i32 %138
  %exp.0.v.i.i = select i1 %cmp7.i.i434, i32 -1022, i32 -1023
  %exp.0.i.i = add nsw i32 %exp.0.v.i.i, %e.0.i.i
  %conv21.i.i = sitofp i32 %exp.0.i.i to double
  %mul22.i.i = fmul double %add18.i.i, 2.000000e+00
  %div23.i.i = fdiv double %mul22.i.i, f0x3FE62E42FEFA39EF
  %add24.i.i = fadd double %div23.i.i, %conv21.i.i
  %div14.i = fmul double %add24.i.i, 5.000000e-01
  br label %cond.end156

cond.end156:                                      ; preds = %log2_d.exit.i, %cond.false152, %cond.true150
  %cond157 = phi double [ %conv151, %cond.true150 ], [ %div14.i, %log2_d.exit.i ], [ -1.000000e+03, %cond.false152 ]
  %cmp158.not = icmp eq i32 %lod_mode, 0
  br i1 %or.cond.i.i, label %if.end.i.i441, label %cube_face.exit.i

if.end.i.i441:                                    ; preds = %cond.end156
  %cmp3.i.i442 = fcmp ogt double %107, %106
  %cmp5.i.i443 = fcmp oge double %107, %108
  %or.cond23.i.not13.i = and i1 %cmp3.i.i442, %cmp5.i.i443
  %cmp10.i.i444 = fcmp oge float %105, 0.000000e+00
  %or.cond.i = or i1 %cmp10.i.i444, %or.cond23.i.not13.i
  %fneg3.i = fneg double %conv.i421
  %spec.select12.i = select i1 %or.cond.i, double %conv.i421, double %fneg3.i
  br label %cube_sc.exit

cube_face.exit.i:                                 ; preds = %cond.end156
  %cmp2.i.i439 = fcmp oge float %101, 0.000000e+00
  %fneg.i = fneg double %conv.i423
  %spec.select.i = select i1 %cmp2.i.i439, double %fneg.i, double %conv.i423
  br label %cube_sc.exit

cube_sc.exit:                                     ; preds = %if.end.i.i441, %cube_face.exit.i
  %retval.0.i440 = phi double [ %spec.select12.i, %if.end.i.i441 ], [ %spec.select.i, %cube_face.exit.i ]
  %cmp.i445 = fcmp ogt double %cond5.i, 0.000000e+00
  br i1 %cmp.i445, label %cond.true.i, label %cube_norm.exit

cond.true.i:                                      ; preds = %cube_sc.exit
  %div.i447 = fdiv double %retval.0.i440, %cond5.i
  %add.i448 = fadd double %div.i447, 1.000000e+00
  %mul.i449 = fmul double %add.i448, 5.000000e-01
  br label %cube_norm.exit

cube_norm.exit:                                   ; preds = %cube_sc.exit, %cond.true.i
  %cond.i446 = phi double [ %mul.i449, %cond.true.i ], [ 5.000000e-01, %cube_sc.exit ]
  br i1 %or.cond.i.i, label %if.end.i.i455, label %sw.default.i453

if.end.i.i455:                                    ; preds = %cube_norm.exit
  %cmp3.i.i456 = fcmp ule double %107, %106
  %cmp5.i.i457 = fcmp ult double %107, %108
  %or.cond23.i.i458 = or i1 %cmp3.i.i456, %cmp5.i.i457
  br i1 %or.cond23.i.i458, label %sw.default.i453, label %if.then6.i.i459

if.then6.i.i459:                                  ; preds = %if.end.i.i455
  %cmp7.i.i460 = fcmp oge float %103, 0.000000e+00
  %fneg.i461 = fneg double %conv.i423
  %spec.select.i462 = select i1 %cmp7.i.i460, double %conv.i423, double %fneg.i461
  br label %cube_tc.exit

sw.default.i453:                                  ; preds = %if.end.i.i455, %cube_norm.exit
  %fneg2.i = fneg double %conv.i422
  br label %cube_tc.exit

cube_tc.exit:                                     ; preds = %if.then6.i.i459, %sw.default.i453
  %retval.0.i454 = phi double [ %fneg2.i, %sw.default.i453 ], [ %spec.select.i462, %if.then6.i.i459 ]
  br i1 %cmp.i445, label %cond.true.i465, label %cube_norm.exit469

cond.true.i465:                                   ; preds = %cube_tc.exit
  %div.i466 = fdiv double %retval.0.i454, %cond5.i
  %add.i467 = fadd double %div.i466, 1.000000e+00
  %mul.i468 = fmul double %add.i467, 5.000000e-01
  br label %cube_norm.exit469

cube_norm.exit469:                                ; preds = %cube_tc.exit, %cond.true.i465
  %cond.i464 = phi double [ %mul.i468, %cond.true.i465 ], [ 5.000000e-01, %cube_tc.exit ]
  %arrayidx.i.i.i470 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 64
  %140 = load i64, ptr addrspace(1) %arrayidx.i.i.i470, align 8
  br i1 %or.cond.i.i, label %if.end.i.i484, label %if.then.i.i474

if.then.i.i474:                                   ; preds = %cube_norm.exit469
  %cmp2.i.i475 = fcmp ult float %101, 0.000000e+00
  %cond.i6.i = zext i1 %cmp2.i.i475 to i64
  br label %cube_layer.exit

if.end.i.i484:                                    ; preds = %cube_norm.exit469
  %cmp3.i.i485 = fcmp ule double %107, %106
  %cmp5.i.i486 = fcmp ult double %107, %108
  %or.cond23.i.i487 = or i1 %cmp3.i.i485, %cmp5.i.i486
  br i1 %or.cond23.i.i487, label %if.end9.i.i491, label %if.then6.i.i488

if.then6.i.i488:                                  ; preds = %if.end.i.i484
  %cmp7.i.i489 = fcmp oge float %103, 0.000000e+00
  %cond8.i.i490 = select i1 %cmp7.i.i489, i64 2, i64 3
  br label %cube_layer.exit

if.end9.i.i491:                                   ; preds = %if.end.i.i484
  %cmp10.i.i492 = fcmp oge float %105, 0.000000e+00
  %cond11.i.i493 = select i1 %cmp10.i.i492, i64 4, i64 5
  br label %cube_layer.exit

cube_layer.exit:                                  ; preds = %if.then.i.i474, %if.then6.i.i488, %if.end9.i.i491
  %retval.0.i.i477 = phi i64 [ %cond.i6.i, %if.then.i.i474 ], [ %cond8.i.i490, %if.then6.i.i488 ], [ %cond11.i.i493, %if.end9.i.i491 ]
  %narrow.i = select i1 %cmp132.not, i32 %c0, i32 0
  %cond.i479 = zext i32 %narrow.i to i64
  %cond.i.i.i480 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %140, i64 1)
  %div7.i = udiv i64 %cond.i.i.i480, 6
  %cond.i.i481 = tail call i64 @llvm.usub.sat.i64(i64 %div7.i, i64 1)
  %141 = tail call i64 @llvm.umin.i64(i64 %cond.i479, i64 %cond.i.i481)
  %mul.i482 = mul nuw nsw i64 %141, 6
  %add.i483 = add nuw nsw i64 %retval.0.i.i477, %mul.i482
  br i1 %cmp158.not, label %if.then.i512, label %if.end.i

if.then.i512:                                     ; preds = %cube_layer.exit
  %call.i = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %t, ptr noundef nonnull readonly %q, i32 noundef %retval.sroa.0.0.i366, i32 noundef 1, i32 noundef 1, double noundef %cond.i446, double noundef %cond.i464, double noundef 0.000000e+00, i64 noundef %add.i483, i32 noundef 2)
  br label %if.end226

if.end.i:                                         ; preds = %cube_layer.exit
  %arrayidx.i.i.i495 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 96
  %142 = load i64, ptr addrspace(1) %arrayidx.i.i.i495, align 8
  %cond.i.i.i496 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %142, i64 1)
  %143 = extractelement <2 x double> %30, i64 0
  %cmp.i.i497 = fcmp oge double %cond157, %143
  %cond.i.i498 = select i1 %cmp.i.i497, double %cond157, double %143
  %cmp.i57.i = fcmp oge double %cond.i.i498, 0.000000e+00
  %cond.i58.i = select i1 %cmp.i57.i, double %cond.i.i498, double 0.000000e+00
  %sub.i499 = add nsw i64 %cond.i.i.i496, -1
  %conv.i500 = uitofp nneg i64 %sub.i499 to double
  %144 = extractelement <2 x double> %30, i64 1
  %cmp.i59.i = fcmp ole double %144, %conv.i500
  %cond.i60.i = select i1 %cmp.i59.i, double %144, double %conv.i500
  %cmp.i61.i = fcmp ole double %cond.i58.i, %cond.i60.i
  %cond.i62.i = select i1 %cmp.i61.i, double %cond.i58.i, double %cond.i60.i
  %cmp.i501 = icmp eq i32 %retval.sroa.9.0.i368, 1
  br i1 %cmp.i501, label %cond.true13.i, label %cond.end.i

cond.end.i:                                       ; preds = %if.end.i
  %add.i502 = fadd double %cond.i62.i, 5.000000e-01
  %145 = tail call double @llvm.floor.f64(double %add.i502)
  %conv.i63.i = fptosi double %145 to i64
  %cond.i64.i = tail call noundef i64 @llvm.smin.i64(i64 %conv.i63.i, i64 %sub.i499)
  br label %cond.end16.i

cond.true13.i:                                    ; preds = %if.end.i
  %146 = tail call double @llvm.floor.f64(double %cond.i62.i)
  %conv.i.i = fptosi double %146 to i64
  %sub.i.i508 = fsub double %cond.i62.i, %146
  %mul.i.i509 = fmul double %sub.i.i508, 2.560000e+02
  %add.i.i510 = fadd double %mul.i.i509, 5.000000e-01
  %147 = tail call double @llvm.floor.f64(double %add.i.i510)
  %conv.i.i.i = fptosi double %147 to i64
  %conv.i65.i = sitofp i64 %conv.i.i.i to double
  %div.i.i511 = fmul nnan double %conv.i65.i, 3.906250e-03
  br label %cond.end16.i

cond.end16.i:                                     ; preds = %cond.true13.i, %cond.end.i
  %cond68.i = phi i64 [ %conv.i.i, %cond.true13.i ], [ %cond.i64.i, %cond.end.i ]
  %cond17.i503 = phi double [ %div.i.i511, %cond.true13.i ], [ 0.000000e+00, %cond.end.i ]
  %sub28.i = fsub double 1.000000e+00, %cond17.i503
  br label %for.body.i504

for.body.i504:                                    ; preds = %for.body.i504, %cond.end16.i
  %tobool25.not.i = phi i1 [ true, %cond.end16.i ], [ false, %for.body.i504 ]
  %indvars.iv.i505 = phi i64 [ 0, %cond.end16.i ], [ 1, %for.body.i504 ]
  %acc.070.i = phi <4 x double> [ zeroinitializer, %cond.end16.i ], [ %add32.i, %for.body.i504 ]
  call void @llvm.lifetime.start.p0(ptr nonnull %q2.i)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q2.i, ptr noundef nonnull readonly align 4 dereferenceable(24) %q, i64 24, i1 false)
  %add21.i = add nsw i64 %indvars.iv.i505, %cond68.i
  %cond.i66.i = tail call noundef i64 @llvm.smin.i64(i64 %add21.i, i64 %sub.i499)
  %conv24.i = trunc i64 %cond.i66.i to i32
  store i32 %conv24.i, ptr %q2.i, align 4
  %cond30.i = select i1 %tobool25.not.i, double %sub28.i, double %cond17.i503
  %splat.splatinsert.i = insertelement <4 x double> poison, double %cond30.i, i64 0
  %splat.splat.i = shufflevector <4 x double> %splat.splatinsert.i, <4 x double> poison, <4 x i32> zeroinitializer
  %call31.i = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %t, ptr noundef %q2.i, i32 noundef %retval.sroa.0.0.i366, i32 noundef 1, i32 noundef 1, double noundef %cond.i446, double noundef %cond.i464, double noundef 0.000000e+00, i64 noundef %add.i483, i32 noundef 2)
  %mul.i506 = fmul <4 x double> %splat.splat.i, %call31.i
  %add32.i = fadd <4 x double> %acc.070.i, %mul.i506
  call void @llvm.lifetime.end.p0(ptr nonnull %q2.i)
  %cmp18.not.not.i = and i1 %cmp.i501, %tobool25.not.i
  br i1 %cmp18.not.not.i, label %for.body.i504, label %if.end226, !llvm.loop !10022

if.else167:                                       ; preds = %if.else
  %148 = add i32 %geom, -3
  %or.cond.i513 = icmp ult i32 %148, 2
  switch i32 %geom, label %cond.false.i [
    i32 3, label %geom_rank.exit
    i32 0, label %geom_rank.exit
    i32 2, label %switch.edge.i
  ]

switch.edge.i:                                    ; preds = %if.else167
  br label %geom_rank.exit

cond.false.i:                                     ; preds = %if.else167
  br label %geom_rank.exit

geom_rank.exit:                                   ; preds = %if.else167, %if.else167, %switch.edge.i, %cond.false.i
  %cond3.i = phi i32 [ 2, %cond.false.i ], [ 1, %if.else167 ], [ 1, %if.else167 ], [ 3, %switch.edge.i ]
  %cmp172 = icmp eq i32 %lod_mode, 1
  br i1 %cmp172, label %cond.true174, label %cond.false176

cond.true174:                                     ; preds = %geom_rank.exit
  %conv175 = fpext float %lod to double
  br label %cond.end181

cond.false176:                                    ; preds = %geom_rank.exit
  %cmp1.i518 = icmp eq i32 %retval.sroa.6.0.i360, 1
  %wide.trip.count.i = zext nneg i32 %cond3.i to i64
  br label %for.body.i519

for.cond.cleanup.i535:                            ; preds = %cond.end.i523
  %149 = extractelement <2 x double> %159, i64 0
  %150 = extractelement <2 x double> %159, i64 1
  %cmp.i.i536 = fcmp oge double %149, %150
  %cond.i.i537 = select i1 %cmp.i.i536, double %149, double %150
  %cmp10.i538 = fcmp ogt double %cond.i.i537, 0.000000e+00
  br i1 %cmp10.i538, label %cond.true12.i541, label %cond.end181

for.body.i519:                                    ; preds = %cond.end.i523, %cond.false176
  %indvars.iv.i520 = phi i64 [ 0, %cond.false176 ], [ %indvars.iv.next.i533, %cond.end.i523 ]
  %151 = phi <2 x double> [ zeroinitializer, %cond.false176 ], [ %159, %cond.end.i523 ]
  br i1 %cmp1.i518, label %cond.true.i576, label %cond.end.i523

cond.true.i576:                                   ; preds = %for.body.i519
  %152 = getelementptr [8 x i8], ptr addrspace(1) %t, i64 %indvars.iv.i520
  %arrayidx.i.i.i577 = getelementptr i8, ptr addrspace(1) %152, i64 72
  %153 = load i64, ptr addrspace(1) %arrayidx.i.i.i577, align 8
  %cond.i.i.i578 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %153, i64 1)
  %conv.i579 = uitofp nneg i64 %cond.i.i.i578 to double
  br label %cond.end.i523

cond.end.i523:                                    ; preds = %cond.true.i576, %for.body.i519
  %conv.pn.i = phi double [ %conv.i579, %cond.true.i576 ], [ 1.000000e+00, %for.body.i519 ]
  %arrayidx.i525 = getelementptr inbounds nuw [8 x i8], ptr %dx, i64 %indvars.iv.i520
  %154 = load double, ptr %arrayidx.i525, align 8
  %arrayidx4.i527 = getelementptr inbounds nuw [8 x i8], ptr %dy, i64 %indvars.iv.i520
  %155 = load double, ptr %arrayidx4.i527, align 8
  %mul.i526 = fmul double %conv.pn.i, %154
  %mul5.i528 = fmul double %conv.pn.i, %155
  %156 = insertelement <2 x double> poison, double %mul.i526, i64 0
  %157 = insertelement <2 x double> %156, double %mul5.i528, i64 1
  %158 = fmul <2 x double> %157, %157
  %159 = fadd <2 x double> %151, %158
  %indvars.iv.next.i533 = add nuw nsw i64 %indvars.iv.i520, 1
  %exitcond.not.i534 = icmp eq i64 %indvars.iv.next.i533, %wide.trip.count.i
  br i1 %exitcond.not.i534, label %for.cond.cleanup.i535, label %for.body.i519, !llvm.loop !10023

cond.true12.i541:                                 ; preds = %for.cond.cleanup.i535
  %160 = bitcast double %cond.i.i537 to i64
  %shr.i.i542 = lshr i64 %160, 52
  %cmp.i29.i543 = icmp eq i64 %shr.i.i542, 0
  %mul.i.i544 = fmul double %cond.i.i537, f0x4350000000000000
  %161 = bitcast double %mul.i.i544 to i64
  %.pre-phi.i.i545 = select i1 %cmp.i29.i543, i64 %161, i64 %160
  %and6.i.i546 = and i64 %.pre-phi.i.i545, 4503599627370495
  %or.i.i547 = or disjoint i64 %and6.i.i546, 4607182418800017408
  %162 = bitcast i64 %or.i.i547 to double
  %cmp7.i.i548 = fcmp ogt double %162, f0x3FF6A09E667F3BCD
  %mul10.i.i549 = fmul nnan double %162, 5.000000e-01
  %m.0.i.i550 = select i1 %cmp7.i.i548, double %mul10.i.i549, double %162
  %sub12.i.i551 = fadd double %m.0.i.i550, -1.000000e+00
  %add.i.i552 = fadd double %m.0.i.i550, 1.000000e+00
  %div.i.i553 = fdiv double %sub12.i.i551, %add.i.i552
  %mul13.i.i554 = fmul double %div.i.i553, %div.i.i553
  br label %for.body.i.i555

for.body.i.i555:                                  ; preds = %for.body.i.i555, %cond.true12.i541
  %k.041.i.i556 = phi i32 [ 1, %cond.true12.i541 ], [ %add20.i.i563, %for.body.i.i555 ]
  %sum.040.i.i557 = phi double [ 0.000000e+00, %cond.true12.i541 ], [ %add18.i.i561, %for.body.i.i555 ]
  %term.039.i.i558 = phi double [ %div.i.i553, %cond.true12.i541 ], [ %mul19.i.i562, %for.body.i.i555 ]
  %conv16.i.i559 = uitofp nneg i32 %k.041.i.i556 to double
  %div17.i.i560 = fdiv double %term.039.i.i558, %conv16.i.i559
  %add18.i.i561 = fadd double %sum.040.i.i557, %div17.i.i560
  %mul19.i.i562 = fmul double %mul13.i.i554, %term.039.i.i558
  %add20.i.i563 = add nuw nsw i32 %k.041.i.i556, 2
  %cmp14.i.i564 = icmp samesign ult i32 %k.041.i.i556, 38
  br i1 %cmp14.i.i564, label %for.body.i.i555, label %log2_d.exit.i565, !llvm.loop !10019

log2_d.exit.i565:                                 ; preds = %for.body.i.i555
  %163 = trunc nuw nsw i64 %shr.i.i542 to i32
  %shr2.i.i566 = lshr i64 %161, 52
  %164 = trunc nuw nsw i64 %shr2.i.i566 to i32
  %sub.i.i567 = add nsw i32 %164, -54
  %e.0.i.i568 = select i1 %cmp.i29.i543, i32 %sub.i.i567, i32 %163
  %exp.0.v.i.i569 = select i1 %cmp7.i.i548, i32 -1022, i32 -1023
  %exp.0.i.i570 = add nsw i32 %exp.0.v.i.i569, %e.0.i.i568
  %conv21.i.i571 = sitofp i32 %exp.0.i.i570 to double
  %mul22.i.i572 = fmul double %add18.i.i561, 2.000000e+00
  %div23.i.i573 = fdiv double %mul22.i.i572, f0x3FE62E42FEFA39EF
  %add24.i.i574 = fadd double %div23.i.i573, %conv21.i.i571
  %div14.i575 = fmul double %add24.i.i574, 5.000000e-01
  br label %cond.end181

cond.end181:                                      ; preds = %log2_d.exit.i565, %for.cond.cleanup.i535, %cond.true174
  %cond182 = phi double [ %conv175, %cond.true174 ], [ %div14.i575, %log2_d.exit.i565 ], [ -1.000000e+03, %for.cond.cleanup.i535 ]
  %cmp183.not = icmp eq i32 %lod_mode, 0
  %tobool185.not = icmp eq i32 %ctype_float, 0
  %cond190 = select i1 %tobool185.not, i32 0, i32 %retval.sroa.0.0.i366
  %.sroa.speculated794 = select i1 %or.cond.i513, i32 %c1, i32 %c0
  %165 = bitcast i32 %.sroa.speculated794 to float
  %conv.i.i582 = fpext float %165 to double
  %conv.i583 = sitofp i32 %.sroa.speculated794 to double
  %cond.i584 = select i1 %tobool185.not, double %conv.i583, double %conv.i.i582
  %.sroa.speculated791 = select i1 %or.cond.i513, i32 %c2, i32 %c1
  %166 = bitcast i32 %.sroa.speculated791 to float
  %conv.i.i586 = fpext float %166 to double
  %conv.i587 = sitofp i32 %.sroa.speculated791 to double
  %cond.i588 = select i1 %tobool185.not, double %conv.i587, double %conv.i.i586
  %.sroa.speculated = select i1 %or.cond.i513, i32 %c3, i32 %c2
  %167 = bitcast i32 %.sroa.speculated to float
  %conv.i.i590 = fpext float %167 to double
  %conv.i591 = sitofp i32 %.sroa.speculated to double
  %cond.i592 = select i1 %tobool185.not, double %conv.i591, double %conv.i.i590
  switch i32 %geom, label %cond.end222 [
    i32 6, label %cond.true219
    i32 4, label %cond.true219
    i32 3, label %cond.true219
  ]

cond.true219:                                     ; preds = %cond.end181, %cond.end181, %cond.end181
  %conv.i593 = zext i32 %c0 to i64
  %arrayidx.i.i.i594 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 64
  %168 = load i64, ptr addrspace(1) %arrayidx.i.i.i594, align 8
  %cond.i.i.i595 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %168, i64 1)
  %sub.i596 = add nsw i64 %cond.i.i.i595, -1
  %169 = tail call range(i64 0, 4294967296) i64 @llvm.umin.i64(i64 %conv.i593, i64 %sub.i596)
  br label %cond.end222

cond.end222:                                      ; preds = %cond.end181, %cond.true219
  %cond223 = phi i64 [ %169, %cond.true219 ], [ 0, %cond.end181 ]
  br i1 %cmp183.not, label %if.then.i647, label %if.end.i599

if.then.i647:                                     ; preds = %cond.end222
  %call.i648 = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %t, ptr noundef nonnull readonly %q, i32 noundef %cond190, i32 noundef %retval.sroa.3.0.i367, i32 noundef %retval.sroa.6.0.i360, double noundef %cond.i584, double noundef %cond.i588, double noundef %cond.i592, i64 noundef %cond223, i32 noundef range(i32 1, 4) %cond3.i)
  br label %if.end226

if.end.i599:                                      ; preds = %cond.end222
  %arrayidx.i.i.i600 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 96
  %170 = load i64, ptr addrspace(1) %arrayidx.i.i.i600, align 8
  %cond.i.i.i601 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %170, i64 1)
  %171 = extractelement <2 x double> %30, i64 0
  %cmp.i.i603 = fcmp oge double %cond182, %171
  %cond.i.i604 = select i1 %cmp.i.i603, double %cond182, double %171
  %cmp.i57.i605 = fcmp oge double %cond.i.i604, 0.000000e+00
  %cond.i58.i606 = select i1 %cmp.i57.i605, double %cond.i.i604, double 0.000000e+00
  %sub.i608 = add nsw i64 %cond.i.i.i601, -1
  %conv.i609 = uitofp nneg i64 %sub.i608 to double
  %172 = extractelement <2 x double> %30, i64 1
  %cmp.i59.i610 = fcmp ole double %172, %conv.i609
  %cond.i60.i611 = select i1 %cmp.i59.i610, double %172, double %conv.i609
  %cmp.i61.i612 = fcmp ole double %cond.i58.i606, %cond.i60.i611
  %cond.i62.i613 = select i1 %cmp.i61.i612, double %cond.i58.i606, double %cond.i60.i611
  %cmp.i615 = icmp eq i32 %retval.sroa.9.0.i368, 1
  br i1 %cmp.i615, label %cond.true13.i639, label %cond.end.i616

cond.end.i616:                                    ; preds = %if.end.i599
  %add.i617 = fadd double %cond.i62.i613, 5.000000e-01
  %173 = tail call double @llvm.floor.f64(double %add.i617)
  %conv.i63.i618 = fptosi double %173 to i64
  %cond.i64.i619 = tail call noundef i64 @llvm.smin.i64(i64 %conv.i63.i618, i64 %sub.i608)
  br label %cond.end16.i620

cond.true13.i639:                                 ; preds = %if.end.i599
  %174 = tail call double @llvm.floor.f64(double %cond.i62.i613)
  %conv.i.i640 = fptosi double %174 to i64
  %sub.i.i641 = fsub double %cond.i62.i613, %174
  %mul.i.i642 = fmul double %sub.i.i641, 2.560000e+02
  %add.i.i643 = fadd double %mul.i.i642, 5.000000e-01
  %175 = tail call double @llvm.floor.f64(double %add.i.i643)
  %conv.i.i.i644 = fptosi double %175 to i64
  %conv.i65.i645 = sitofp i64 %conv.i.i.i644 to double
  %div.i.i646 = fmul nnan double %conv.i65.i645, 3.906250e-03
  br label %cond.end16.i620

cond.end16.i620:                                  ; preds = %cond.true13.i639, %cond.end.i616
  %cond68.i621 = phi i64 [ %conv.i.i640, %cond.true13.i639 ], [ %cond.i64.i619, %cond.end.i616 ]
  %cond17.i622 = phi double [ %div.i.i646, %cond.true13.i639 ], [ 0.000000e+00, %cond.end.i616 ]
  %sub28.i623 = fsub double 1.000000e+00, %cond17.i622
  br label %for.body.i624

for.body.i624:                                    ; preds = %for.body.i624, %cond.end16.i620
  %tobool25.not.i625 = phi i1 [ true, %cond.end16.i620 ], [ false, %for.body.i624 ]
  %indvars.iv.i626 = phi i64 [ 0, %cond.end16.i620 ], [ 1, %for.body.i624 ]
  %acc.070.i627 = phi <4 x double> [ zeroinitializer, %cond.end16.i620 ], [ %add32.i636, %for.body.i624 ]
  call void @llvm.lifetime.start.p0(ptr nonnull %q2.i597)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q2.i597, ptr noundef nonnull readonly align 4 dereferenceable(24) %q, i64 24, i1 false)
  %add21.i628 = add nsw i64 %indvars.iv.i626, %cond68.i621
  %cond.i66.i629 = tail call noundef i64 @llvm.smin.i64(i64 %add21.i628, i64 %sub.i608)
  %conv24.i630 = trunc i64 %cond.i66.i629 to i32
  store i32 %conv24.i630, ptr %q2.i597, align 4
  %cond30.i631 = select i1 %tobool25.not.i625, double %sub28.i623, double %cond17.i622
  %splat.splatinsert.i632 = insertelement <4 x double> poison, double %cond30.i631, i64 0
  %splat.splat.i633 = shufflevector <4 x double> %splat.splatinsert.i632, <4 x double> poison, <4 x i32> zeroinitializer
  %call31.i634 = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %t, ptr noundef %q2.i597, i32 noundef %cond190, i32 noundef %retval.sroa.3.0.i367, i32 noundef %retval.sroa.6.0.i360, double noundef %cond.i584, double noundef %cond.i588, double noundef %cond.i592, i64 noundef %cond223, i32 noundef range(i32 1, 4) %cond3.i)
  %mul.i635 = fmul <4 x double> %splat.splat.i633, %call31.i634
  %add32.i636 = fadd <4 x double> %acc.070.i627, %mul.i635
  call void @llvm.lifetime.end.p0(ptr nonnull %q2.i597)
  %cmp18.not.not.i637 = and i1 %cmp.i615, %tobool25.not.i625
  br i1 %cmp18.not.not.i637, label %for.body.i624, label %if.end226, !llvm.loop !10022

if.end226:                                        ; preds = %for.body.i504, %for.body.i624, %cond.end124, %if.then.i647, %if.then.i512, %cond.end
  %176 = phi <4 x double> [ %vecins, %cond.end124 ], [ %add32.i636, %for.body.i624 ], [ zeroinitializer, %cond.end ], [ %call.i, %if.then.i512 ], [ %call.i648, %if.then.i647 ], [ %add32.i, %for.body.i504 ]
  %cmp.i650 = icmp eq i32 %dtype, 0
  br i1 %cmp.i650, label %if.then.i683, label %if.end.i651

if.end.i651:                                      ; preds = %if.end226
  %vecext228 = extractelement <4 x double> %176, i64 0
  %cmp1.i652 = fcmp uno double %vecext228, 0.000000e+00
  %conv5.i = fptosi double %vecext228 to i64
  %conv6.i = trunc i64 %conv5.i to i32
  %retval.0.i653.ph = select i1 %cmp1.i652, i32 0, i32 %conv6.i
  %vecinit230741 = insertelement <4 x i32> poison, i32 %retval.0.i653.ph, i64 0
  %177 = shufflevector <4 x double> %176, <4 x double> poison, <2 x i32> <i32 1, i32 2>
  %178 = fcmp uno <2 x double> %177, zeroinitializer
  %179 = fptosi <2 x double> %177 to <2 x i64>
  %180 = trunc <2 x i64> %179 to <2 x i32>
  %181 = select <2 x i1> %178, <2 x i32> zeroinitializer, <2 x i32> %180
  %182 = shufflevector <2 x i32> %181, <2 x i32> poison, <4 x i32> <i32 0, i32 1, i32 poison, i32 poison>
  %vecinit236755818 = shufflevector <4 x i32> %vecinit230741, <4 x i32> %182, <4 x i32> <i32 0, i32 4, i32 5, i32 poison>
  %vecext237756 = extractelement <4 x double> %176, i64 3
  %cmp1.i678 = fcmp uno double %vecext237756, 0.000000e+00
  br i1 %cmp1.i678, label %out_bits.exit685, label %if.end4.i679

if.then.i683:                                     ; preds = %if.end226
  %183 = fptrunc <4 x double> %176 to <4 x float>
  %vecext234 = extractelement <4 x double> %176, i64 2
  %conv.i674 = fptrunc double %vecext234 to float
  %184 = insertelement <4 x float> %183, float %conv.i674, i64 2
  %vecinit236 = bitcast <4 x float> %184 to <4 x i32>
  %vecext237 = extractelement <4 x double> %176, i64 3
  %conv.i684 = fptrunc double %vecext237 to float
  %185 = bitcast float %conv.i684 to i32
  br label %out_bits.exit685

if.end4.i679:                                     ; preds = %if.end.i651
  %conv5.i680 = fptosi double %vecext237756 to i64
  %conv6.i681 = trunc i64 %conv5.i680 to i32
  br label %out_bits.exit685

out_bits.exit685:                                 ; preds = %if.then.i683, %if.end.i651, %if.end4.i679
  %vecinit236757 = phi <4 x i32> [ %vecinit236, %if.then.i683 ], [ %vecinit236755818, %if.end4.i679 ], [ %vecinit236755818, %if.end.i651 ]
  %retval.0.i682 = phi i32 [ %185, %if.then.i683 ], [ %conv6.i681, %if.end4.i679 ], [ 0, %if.end.i651 ]
  %vecinit239 = insertelement <4 x i32> %vecinit236757, i32 %retval.0.i682, i64 3
  call void @llvm.lifetime.end.p0(ptr nonnull %dy)
  call void @llvm.lifetime.end.p0(ptr nonnull %dx)
  call void @llvm.lifetime.end.p0(ptr nonnull %q)
  br label %return

return:                                           ; preds = %sampling_of.exit, %if.then14, %out_bits.exit685
  %retval.1 = phi <4 x i32> [ %vecinit239, %out_bits.exit685 ], [ %vecinit36, %sampling_of.exit ], [ %call19, %if.then14 ]
  ret <4 x i32> %retval.1
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(ptr captures(none))

; Function Attrs: convergent

; Function Attrs: inlinehint nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define internal fastcc <4 x float> @__zluda_agx_tex_sample_f32(ptr addrspace(1) nofree noundef readonly captures(none) %t, i32 noundef %filter, i32 noundef %addr, i32 noundef %norm, float noundef %x, float noundef %y, i32 noundef range(i32 1, 3) %rank) unnamed_addr #3 {
entry:
  %arrayidx.i.i.i = getelementptr i8, ptr addrspace(1) %t, i64 72
  %0 = load i64, ptr addrspace(1) %arrayidx.i.i.i, align 8
  %cond.i.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %0, i64 1)
  %arrayidx.i.i.i129 = getelementptr i8, ptr addrspace(1) %t, i64 80
  %1 = load i64, ptr addrspace(1) %arrayidx.i.i.i129, align 8
  %cond.i.i.i130 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %1, i64 1)
  %cmp.not = icmp eq i32 %filter, 1
  %cmp.i157 = icmp eq i32 %norm, 1
  %conv.i158 = uitofp nneg i64 %cond.i.i.i to float
  br i1 %cmp.not, label %if.end, label %if.then

if.then:                                          ; preds = %entry
  %2 = tail call float @llvm.fma.f32(float %x, float %conv.i158, float 0.000000e+00)
  %add.i = fadd float %x, 0.000000e+00
  %cond.i = select i1 %cmp.i157, float %2, float %add.i
  %3 = tail call float @llvm.floor.f32(float %cond.i)
  %conv = fptosi float %3 to i64
  switch i32 %addr, label %sw.default.i [
    i32 0, label %sw.bb.i
    i32 2, label %sw.bb1.i
    i32 3, label %sw.bb3.i
  ]

sw.bb.i:                                          ; preds = %if.then
  %rem.i.i = srem i64 %conv, %cond.i.i.i
  %add.i.i = add nsw i64 %rem.i.i, %cond.i.i.i
  %rem1.i.i = srem i64 %add.i.i, %cond.i.i.i
  br label %ad_idx.exit

sw.bb1.i:                                         ; preds = %if.then
  %cmp2.i = icmp sgt i64 %cond.i.i.i, %conv
  %4 = tail call i64 @llvm.smax.i64(i64 %conv, i64 -1)
  %cond.i131 = select i1 %cmp2.i, i64 %4, i64 -1
  br label %ad_idx.exit

sw.bb3.i:                                         ; preds = %if.then
  %mul.i = shl nuw nsw i64 %cond.i.i.i, 1
  %rem.i27.i = srem i64 %conv, %mul.i
  %add.i28.i = add nsw i64 %rem.i27.i, %mul.i
  %rem1.i29.i = srem i64 %add.i28.i, %mul.i
  %cmp5.i = icmp slt i64 %rem1.i29.i, %cond.i.i.i
  %5 = xor i64 %rem1.i29.i, -1
  %sub9.i = add i64 %mul.i, %5
  %cond11.i = select i1 %cmp5.i, i64 %rem1.i29.i, i64 %sub9.i
  br label %ad_idx.exit

sw.default.i:                                     ; preds = %if.then
  %cond.i.i = tail call noundef i64 @llvm.smax.i64(i64 %conv, i64 0)
  %sub13.i = add nsw i64 %cond.i.i.i, -1
  %6 = tail call i64 @llvm.umin.i64(i64 %cond.i.i, i64 %sub13.i)
  br label %ad_idx.exit

ad_idx.exit:                                      ; preds = %sw.bb.i, %sw.bb1.i, %sw.bb3.i, %sw.default.i
  %retval.0.i = phi i64 [ %6, %sw.default.i ], [ %rem1.i.i, %sw.bb.i ], [ %cond.i131, %sw.bb1.i ], [ %cond11.i, %sw.bb3.i ]
  %cmp4 = icmp samesign ugt i32 %rank, 1
  br i1 %cmp4, label %cond.true, label %cond.end

cond.true:                                        ; preds = %ad_idx.exit
  %conv.i133 = uitofp nneg i64 %cond.i.i.i130 to float
  %7 = tail call float @llvm.fma.f32(float %y, float %conv.i133, float 0.000000e+00)
  %add.i134 = fadd float %y, 0.000000e+00
  %cond.i135 = select i1 %cmp.i157, float %7, float %add.i134
  %8 = tail call float @llvm.floor.f32(float %cond.i135)
  %conv7 = fptosi float %8 to i64
  switch i32 %addr, label %sw.default.i152 [
    i32 0, label %sw.bb.i148
    i32 2, label %sw.bb1.i145
    i32 3, label %sw.bb3.i136
  ]

sw.bb.i148:                                       ; preds = %cond.true
  %rem.i.i149 = srem i64 %conv7, %cond.i.i.i130
  %add.i.i150 = add nsw i64 %rem.i.i149, %cond.i.i.i130
  %rem1.i.i151 = srem i64 %add.i.i150, %cond.i.i.i130
  br label %cond.end

sw.bb1.i145:                                      ; preds = %cond.true
  %cmp2.i146 = icmp sgt i64 %cond.i.i.i130, %conv7
  %9 = tail call i64 @llvm.smax.i64(i64 %conv7, i64 -1)
  %cond.i147 = select i1 %cmp2.i146, i64 %9, i64 -1
  br label %cond.end

sw.bb3.i136:                                      ; preds = %cond.true
  %mul.i137 = shl nuw nsw i64 %cond.i.i.i130, 1
  %rem.i27.i138 = srem i64 %conv7, %mul.i137
  %add.i28.i139 = add nsw i64 %rem.i27.i138, %mul.i137
  %rem1.i29.i140 = srem i64 %add.i28.i139, %mul.i137
  %cmp5.i141 = icmp slt i64 %rem1.i29.i140, %cond.i.i.i130
  %10 = xor i64 %rem1.i29.i140, -1
  %sub9.i142 = add i64 %mul.i137, %10
  %cond11.i143 = select i1 %cmp5.i141, i64 %rem1.i29.i140, i64 %sub9.i142
  br label %cond.end

sw.default.i152:                                  ; preds = %cond.true
  %cond.i.i153 = tail call noundef i64 @llvm.smax.i64(i64 %conv7, i64 0)
  %sub13.i154 = add nsw i64 %cond.i.i.i130, -1
  %11 = tail call i64 @llvm.umin.i64(i64 %cond.i.i153, i64 %sub13.i154)
  br label %cond.end

cond.end:                                         ; preds = %sw.default.i152, %sw.bb3.i136, %sw.bb1.i145, %sw.bb.i148, %ad_idx.exit
  %cond = phi i64 [ 0, %ad_idx.exit ], [ %11, %sw.default.i152 ], [ %rem1.i.i151, %sw.bb.i148 ], [ %cond.i147, %sw.bb1.i145 ], [ %cond11.i143, %sw.bb3.i136 ]
  %call9 = tail call fastcc <4 x float> @__zluda_agx_tex_texel_f32(ptr addrspace(1) noundef %t, i64 noundef %retval.0.i, i64 noundef %cond)
  br label %cleanup

if.end:                                           ; preds = %entry
  %12 = tail call float @llvm.fma.f32(float %x, float %conv.i158, float -5.000000e-01)
  %add.i159 = fadd float %x, -5.000000e-01
  %cond.i160 = select i1 %cmp.i157, float %12, float %add.i159
  %cmp11 = icmp samesign ugt i32 %rank, 1
  %conv.i162 = uitofp nneg i64 %cond.i.i.i130 to float
  %13 = tail call float @llvm.fma.f32(float %y, float %conv.i162, float -5.000000e-01)
  %add.i163 = fadd float %y, -5.000000e-01
  %cond.i164 = select i1 %cmp.i157, float %13, float %add.i163
  %14 = tail call float @llvm.floor.f32(float %cond.i160)
  %15 = tail call float @llvm.floor.f32(float %cond.i164)
  %16 = select i1 %cmp11, float %15, float 0.000000e+00
  %conv18 = fptosi float %14 to i64
  switch i32 %addr, label %ad_idx.exit185 [
    i32 0, label %ad_idx.exit185.thread
    i32 2, label %ad_idx.exit185.thread254
    i32 3, label %ad_idx.exit185.thread259
  ]

ad_idx.exit185.thread:                            ; preds = %if.end
  %add250 = add nsw i64 %conv18, 1
  %17 = insertelement <2 x i64> poison, i64 %conv18, i64 0
  %18 = insertelement <2 x i64> %17, i64 %add250, i64 1
  %19 = insertelement <2 x i64> poison, i64 %cond.i.i.i, i64 0
  %20 = shufflevector <2 x i64> %19, <2 x i64> poison, <2 x i32> zeroinitializer
  %21 = srem <2 x i64> %18, %20
  %22 = add nsw <2 x i64> %21, %20
  %23 = srem <2 x i64> %22, %20
  br label %ad_idx.exit206

ad_idx.exit185.thread254:                         ; preds = %if.end
  %add256 = add nsw i64 %conv18, 1
  %24 = insertelement <2 x i64> poison, i64 %cond.i.i.i, i64 0
  %25 = shufflevector <2 x i64> %24, <2 x i64> poison, <2 x i32> zeroinitializer
  %26 = insertelement <2 x i64> poison, i64 %conv18, i64 0
  %27 = insertelement <2 x i64> %26, i64 %add256, i64 1
  %28 = icmp sgt <2 x i64> %25, %27
  %29 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %27, <2 x i64> splat (i64 -1))
  %30 = select <2 x i1> %28, <2 x i64> %29, <2 x i64> splat (i64 -1)
  br label %ad_idx.exit206

ad_idx.exit185.thread259:                         ; preds = %if.end
  %mul.i166 = shl nuw nsw i64 %cond.i.i.i, 1
  %add261 = add nsw i64 %conv18, 1
  %31 = insertelement <2 x i64> poison, i64 %conv18, i64 0
  %32 = insertelement <2 x i64> %31, i64 %add261, i64 1
  %33 = insertelement <2 x i64> poison, i64 %mul.i166, i64 0
  %34 = shufflevector <2 x i64> %33, <2 x i64> poison, <2 x i32> zeroinitializer
  %35 = srem <2 x i64> %32, %34
  %36 = add nsw <2 x i64> %35, %34
  %37 = srem <2 x i64> %36, %34
  %38 = insertelement <2 x i64> poison, i64 %cond.i.i.i, i64 0
  %39 = shufflevector <2 x i64> %38, <2 x i64> poison, <2 x i32> zeroinitializer
  %40 = icmp slt <2 x i64> %37, %39
  %41 = xor <2 x i64> %37, splat (i64 -1)
  %42 = add <2 x i64> %34, %41
  %43 = select <2 x i1> %40, <2 x i64> %37, <2 x i64> %42
  br label %ad_idx.exit206

ad_idx.exit185:                                   ; preds = %if.end
  %sub13.i183 = add nsw i64 %cond.i.i.i, -1
  %44 = tail call i64 @llvm.smax.i64(i64 %conv18, i64 -1)
  %cond.i.i182 = tail call noundef i64 @llvm.smax.i64(i64 %conv18, i64 0)
  %45 = tail call i64 @llvm.umin.i64(i64 %cond.i.i182, i64 %sub13.i183)
  %cond.i.i203 = add nsw i64 %44, 1
  %46 = tail call i64 @llvm.umin.i64(i64 %cond.i.i203, i64 %sub13.i183)
  %47 = insertelement <2 x i64> poison, i64 %45, i64 0
  %48 = insertelement <2 x i64> %47, i64 %46, i64 1
  br label %ad_idx.exit206

ad_idx.exit206:                                   ; preds = %ad_idx.exit185.thread, %ad_idx.exit185.thread254, %ad_idx.exit185.thread259, %ad_idx.exit185
  %49 = phi <2 x i64> [ %48, %ad_idx.exit185 ], [ %23, %ad_idx.exit185.thread ], [ %30, %ad_idx.exit185.thread254 ], [ %43, %ad_idx.exit185.thread259 ]
  br i1 %cmp11, label %cond.true24, label %cond.end37

cond.true24:                                      ; preds = %ad_idx.exit206
  %conv25 = fptosi float %16 to i64
  switch i32 %addr, label %cond.true32 [
    i32 0, label %cond.true32.thread
    i32 2, label %cond.true32.thread273
    i32 3, label %cond.true32.thread279
  ]

cond.true32.thread:                               ; preds = %cond.true24
  %add34269 = add nsw i64 %conv25, 1
  %50 = insertelement <2 x i64> poison, i64 %conv25, i64 0
  %51 = insertelement <2 x i64> %50, i64 %add34269, i64 1
  %52 = insertelement <2 x i64> poison, i64 %cond.i.i.i130, i64 0
  %53 = shufflevector <2 x i64> %52, <2 x i64> poison, <2 x i32> zeroinitializer
  %54 = srem <2 x i64> %51, %53
  %55 = add nsw <2 x i64> %54, %53
  %56 = srem <2 x i64> %55, %53
  br label %cond.true43

cond.true32.thread273:                            ; preds = %cond.true24
  %add34276 = add nsw i64 %conv25, 1
  %57 = insertelement <2 x i64> poison, i64 %conv25, i64 0
  %58 = insertelement <2 x i64> %57, i64 %add34276, i64 1
  %59 = insertelement <2 x i64> poison, i64 %cond.i.i.i130, i64 0
  %60 = shufflevector <2 x i64> %59, <2 x i64> poison, <2 x i32> zeroinitializer
  %61 = icmp slt <2 x i64> %58, %60
  %62 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %58, <2 x i64> splat (i64 -1))
  %63 = select <2 x i1> %61, <2 x i64> %62, <2 x i64> splat (i64 -1)
  br label %cond.true43

cond.true32.thread279:                            ; preds = %cond.true24
  %mul.i208 = shl nuw nsw i64 %cond.i.i.i130, 1
  %add34282 = add nsw i64 %conv25, 1
  %64 = insertelement <2 x i64> poison, i64 %conv25, i64 0
  %65 = insertelement <2 x i64> %64, i64 %add34282, i64 1
  %66 = insertelement <2 x i64> poison, i64 %mul.i208, i64 0
  %67 = shufflevector <2 x i64> %66, <2 x i64> poison, <2 x i32> zeroinitializer
  %68 = srem <2 x i64> %65, %67
  %69 = add nsw <2 x i64> %68, %67
  %70 = srem <2 x i64> %69, %67
  %71 = insertelement <2 x i64> poison, i64 %cond.i.i.i130, i64 0
  %72 = shufflevector <2 x i64> %71, <2 x i64> poison, <2 x i32> zeroinitializer
  %73 = icmp slt <2 x i64> %70, %72
  %74 = xor <2 x i64> %70, splat (i64 -1)
  %75 = add <2 x i64> %67, %74
  %76 = select <2 x i1> %73, <2 x i64> %70, <2 x i64> %75
  br label %cond.true43

cond.true32:                                      ; preds = %cond.true24
  %sub13.i225 = add nsw i64 %cond.i.i.i130, -1
  %77 = tail call i64 @llvm.smax.i64(i64 %conv25, i64 -1)
  %cond.i.i224 = tail call noundef i64 @llvm.smax.i64(i64 %conv25, i64 0)
  %78 = tail call i64 @llvm.umin.i64(i64 %cond.i.i224, i64 %sub13.i225)
  %cond.i.i245 = add nsw i64 %77, 1
  %79 = tail call i64 @llvm.umin.i64(i64 %cond.i.i245, i64 %sub13.i225)
  %80 = insertelement <2 x i64> poison, i64 %78, i64 0
  %81 = insertelement <2 x i64> %80, i64 %79, i64 1
  br label %cond.true43

cond.end37:                                       ; preds = %ad_idx.exit206
  %sub = fsub float %cond.i160, %14
  %mul = fmul float %sub, 2.560000e+02
  %add39 = fadd float %mul, 5.000000e-01
  %82 = tail call float @llvm.floor.f32(float %add39)
  br label %cond.end49

cond.true43:                                      ; preds = %cond.true32, %cond.true32.thread279, %cond.true32.thread273, %cond.true32.thread
  %83 = phi <2 x i64> [ %76, %cond.true32.thread279 ], [ %63, %cond.true32.thread273 ], [ %56, %cond.true32.thread ], [ %81, %cond.true32 ]
  %sub287 = fsub float %cond.i160, %14
  %mul288 = fmul float %sub287, 2.560000e+02
  %add39289 = fadd float %mul288, 5.000000e-01
  %84 = tail call float @llvm.floor.f32(float %add39289)
  %sub44 = fsub float %cond.i164, %16
  %mul45 = fmul float %sub44, 2.560000e+02
  %add46 = fadd float %mul45, 5.000000e-01
  %85 = tail call float @llvm.floor.f32(float %add46)
  %mul47 = fmul float %85, 3.906250e-03
  br label %cond.end49

cond.end49:                                       ; preds = %cond.end37, %cond.true43
  %mul40295.in = phi float [ %84, %cond.true43 ], [ %82, %cond.end37 ]
  %cond50 = phi float [ %mul47, %cond.true43 ], [ 0.000000e+00, %cond.end37 ]
  %86 = phi <2 x i64> [ %83, %cond.true43 ], [ zeroinitializer, %cond.end37 ]
  %mul40295 = fmul float %mul40295.in, 3.906250e-03
  %sub51 = fsub float 1.000000e+00, %mul40295
  %sub52 = fsub float 1.000000e+00, %cond50
  %mul53 = fmul float %sub51, %sub52
  %splat.splatinsert = insertelement <4 x float> poison, float %mul53, i64 0
  %splat.splat = shufflevector <4 x float> %splat.splatinsert, <4 x float> poison, <4 x i32> zeroinitializer
  %87 = extractelement <2 x i64> %86, i64 0
  %88 = extractelement <2 x i64> %49, i64 0
  %call54 = tail call fastcc <4 x float> @__zluda_agx_tex_texel_f32(ptr addrspace(1) noundef %t, i64 noundef %88, i64 noundef %87)
  %mul55 = fmul <4 x float> %call54, %splat.splat
  %mul57 = fmul float %mul40295, %sub52
  %splat.splatinsert58 = insertelement <4 x float> poison, float %mul57, i64 0
  %splat.splat59 = shufflevector <4 x float> %splat.splatinsert58, <4 x float> poison, <4 x i32> zeroinitializer
  %89 = extractelement <2 x i64> %49, i64 1
  %call60 = tail call fastcc <4 x float> @__zluda_agx_tex_texel_f32(ptr addrspace(1) noundef %t, i64 noundef %89, i64 noundef %87)
  %mul61 = fmul <4 x float> %call60, %splat.splat59
  %add62 = fadd <4 x float> %mul55, %mul61
  br i1 %cmp11, label %if.then65, label %cleanup

if.then65:                                        ; preds = %cond.end49
  %mul67 = fmul float %sub51, %cond50
  %splat.splatinsert68 = insertelement <4 x float> poison, float %mul67, i64 0
  %splat.splat69 = shufflevector <4 x float> %splat.splatinsert68, <4 x float> poison, <4 x i32> zeroinitializer
  %90 = extractelement <2 x i64> %86, i64 1
  %call70 = tail call fastcc <4 x float> @__zluda_agx_tex_texel_f32(ptr addrspace(1) noundef %t, i64 noundef %88, i64 noundef %90)
  %mul71 = fmul <4 x float> %splat.splat69, %call70
  %add72 = fadd <4 x float> %add62, %mul71
  %mul73 = fmul float %mul40295, %cond50
  %splat.splatinsert74 = insertelement <4 x float> poison, float %mul73, i64 0
  %splat.splat75 = shufflevector <4 x float> %splat.splatinsert74, <4 x float> poison, <4 x i32> zeroinitializer
  %call76 = tail call fastcc <4 x float> @__zluda_agx_tex_texel_f32(ptr addrspace(1) noundef %t, i64 noundef %89, i64 noundef %90)
  %mul77 = fmul <4 x float> %splat.splat75, %call76
  %add78 = fadd <4 x float> %add72, %mul77
  br label %cleanup

cleanup:                                          ; preds = %cond.end49, %if.then65, %cond.end
  %retval.0 = phi <4 x float> [ %call9, %cond.end ], [ %add78, %if.then65 ], [ %add62, %cond.end49 ]
  ret <4 x float> %retval.0
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(ptr captures(none))

; Function Attrs: inlinehint nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define internal fastcc double @__zluda_agx_tex_channel(ptr addrspace(1) nofree noundef readonly captures(none) %t, i64 noundef %addr) unnamed_addr #3 {
entry:
  %arrayidx.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 16
  %0 = load i64, ptr addrspace(1) %arrayidx.i, align 8
  switch i64 %0, label %sw.default.i [
    i64 0, label %code_bytes.exit
    i64 20, label %code_bytes.exit
    i64 1, label %sw.bb1.i
    i64 21, label %sw.bb1.i
    i64 6, label %sw.bb1.i
  ]

sw.bb1.i:                                         ; preds = %entry, %entry, %entry
  br label %code_bytes.exit

sw.default.i:                                     ; preds = %entry
  br label %code_bytes.exit

code_bytes.exit:                                  ; preds = %entry, %entry, %sw.bb1.i, %sw.default.i
  %retval.0.i = phi i64 [ 4, %sw.default.i ], [ 2, %sw.bb1.i ], [ 1, %entry ], [ 1, %entry ]
  %1 = inttoptr i64 %addr to ptr addrspace(1)
  br label %for.body

for.cond.cleanup:                                 ; preds = %for.body
  switch i64 %0, label %sw.default.i.i [
    i64 6, label %if.then
    i64 7, label %if.then7
    i64 0, label %code_bits.exit
    i64 20, label %code_bits.exit
    i64 1, label %sw.bb1.i.i
    i64 21, label %sw.bb1.i.i
  ]

for.body:                                         ; preds = %code_bytes.exit, %for.body
  %indvars.iv = phi i64 [ 0, %code_bytes.exit ], [ %indvars.iv.next, %for.body ]
  %v.077 = phi i32 [ 0, %code_bytes.exit ], [ %or, %for.body ]
  %arrayidx = getelementptr inbounds nuw i8, ptr addrspace(1) %1, i64 %indvars.iv
  %2 = load i8, ptr addrspace(1) %arrayidx, align 1
  %conv = zext i8 %2 to i32
  %indvars.iv.tr = trunc nuw nsw i64 %indvars.iv to i32
  %3 = shl nuw nsw i32 %indvars.iv.tr, 3
  %shl = shl nuw i32 %conv, %3
  %or = or i32 %shl, %v.077
  %indvars.iv.next = add nuw nsw i64 %indvars.iv, 1
  %exitcond.not = icmp eq i64 %indvars.iv.next, %retval.0.i
  br i1 %exitcond.not, label %for.cond.cleanup, label %for.body, !llvm.loop !10024

if.then:                                          ; preds = %for.cond.cleanup
  %shr1.i = lshr i32 %or, 10
  %and.i = and i32 %shr1.i, 31
  %and2.i = and i32 %or, 1023
  switch i32 %and.i, label %if.else6.i [
    i32 0, label %if.then.i
    i32 31, label %if.then5.i
  ]

if.then.i:                                        ; preds = %if.then
  %conv.i = uitofp nneg i32 %and2.i to double
  %div.i = fmul nnan double %conv.i, f0x3E70000000000000
  br label %half_to_double.exit

if.then5.i:                                       ; preds = %if.then
  %tobool.not.i = icmp eq i32 %and2.i, 0
  %cond.i = select i1 %tobool.not.i, double +inf, double +qnan
  br label %half_to_double.exit

if.else6.i:                                       ; preds = %if.then
  %or.i = or disjoint i32 %and2.i, 1024
  %conv7.i = uitofp nneg i32 %or.i to double
  %sub.i = add nsw i32 %and.i, -25
  %cmp831.i = icmp samesign ugt i32 %and.i, 25
  br i1 %cmp831.i, label %while.body.i, label %while.cond10.preheader.i

while.cond10.preheader.i:                         ; preds = %if.else6.i
  %cmp1135.not.i = icmp eq i32 %and.i, 25
  br i1 %cmp1135.not.i, label %half_to_double.exit, label %while.body13.i

while.body.i:                                     ; preds = %if.else6.i, %while.body.i
  %x.033.i = phi i32 [ %dec.i, %while.body.i ], [ %sub.i, %if.else6.i ]
  %v.032.i = phi double [ %mul.i, %while.body.i ], [ %conv7.i, %if.else6.i ]
  %mul.i = fmul double %v.032.i, 2.000000e+00
  %dec.i = add nsw i32 %x.033.i, -1
  %cmp8.i = icmp samesign ugt i32 %x.033.i, 1
  br i1 %cmp8.i, label %while.body.i, label %half_to_double.exit, !llvm.loop !10025

while.body13.i:                                   ; preds = %while.cond10.preheader.i, %while.body13.i
  %x.137.i = phi i32 [ %inc.i, %while.body13.i ], [ %sub.i, %while.cond10.preheader.i ]
  %v.136.i = phi double [ %mul14.i, %while.body13.i ], [ %conv7.i, %while.cond10.preheader.i ]
  %mul14.i = fmul double %v.136.i, 5.000000e-01
  %inc.i = add nsw i32 %x.137.i, 1
  %exitcond.not.i = icmp eq i32 %inc.i, 0
  br i1 %exitcond.not.i, label %half_to_double.exit, label %while.body13.i, !llvm.loop !10026

half_to_double.exit:                              ; preds = %while.body13.i, %while.body.i, %if.then.i, %if.then5.i, %while.cond10.preheader.i
  %v.2.i = phi double [ %div.i, %if.then.i ], [ %cond.i, %if.then5.i ], [ %conv7.i, %while.cond10.preheader.i ], [ %mul.i, %while.body.i ], [ %mul14.i, %while.body13.i ]
  %tobool17.not.i = icmp ult i32 %or, 32768
  %fneg.i = fneg double %v.2.i
  %cond18.i = select i1 %tobool17.not.i, double %v.2.i, double %fneg.i
  br label %cleanup40

if.then7:                                         ; preds = %for.cond.cleanup
  %4 = bitcast i32 %or to float
  %conv.i64 = fpext float %4 to double
  br label %cleanup40

sw.bb1.i.i:                                       ; preds = %for.cond.cleanup, %for.cond.cleanup
  br label %code_bits.exit

sw.default.i.i:                                   ; preds = %for.cond.cleanup
  br label %code_bits.exit

code_bits.exit:                                   ; preds = %for.cond.cleanup, %for.cond.cleanup, %sw.bb1.i.i, %sw.default.i.i
  %retval.0.i.i = phi i32 [ 32, %sw.default.i.i ], [ 16, %sw.bb1.i.i ], [ 8, %for.cond.cleanup ], [ 8, %for.cond.cleanup ]
  %conv11 = zext i32 %or to i64
  switch i64 %0, label %if.end21 [
    i64 21, label %land.lhs.true15
    i64 20, label %land.lhs.true15
    i64 3, label %land.lhs.true15
    i64 5, label %land.lhs.true15
  ]

land.lhs.true15:                                  ; preds = %code_bits.exit, %code_bits.exit, %code_bits.exit, %code_bits.exit
  %sub = add nsw i32 %retval.0.i.i, -1
  %5 = shl nuw i32 1, %sub
  %6 = and i32 %5, %or
  %tobool16.not = icmp eq i32 %6, 0
  br i1 %tobool16.not, label %if.end21, label %if.then17

if.then17:                                        ; preds = %land.lhs.true15
  %sh_prom = zext nneg i32 %retval.0.i.i to i64
  %shl19.neg = shl nsw i64 -1, %sh_prom
  %sub20 = add nsw i64 %shl19.neg, %conv11
  br label %if.end21

if.end21:                                         ; preds = %code_bits.exit, %if.then17, %land.lhs.true15
  %iv.0 = phi i64 [ %sub20, %if.then17 ], [ %conv11, %land.lhs.true15 ], [ %conv11, %code_bits.exit ]
  %arrayidx.i65 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 56
  %7 = load i64, ptr addrspace(1) %arrayidx.i65, align 8
  %tobool23.not = icmp eq i64 %7, 0
  br i1 %tobool23.not, label %if.then24, label %if.end26

if.then24:                                        ; preds = %if.end21
  %conv25 = sitofp i64 %iv.0 to double
  br label %cleanup40

if.end26:                                         ; preds = %if.end21
  switch i64 %0, label %code_signed.exit69 [
    i64 21, label %code_signed.exit69.thread
    i64 20, label %code_signed.exit69.thread
    i64 3, label %code_signed.exit69.thread
  ]

code_signed.exit69:                               ; preds = %if.end26
  %cmp4.i68.not = icmp eq i64 %0, 5
  %sub29 = sext i1 %cmp4.i68.not to i32
  br label %code_signed.exit69.thread

code_signed.exit69.thread:                        ; preds = %if.end26, %if.end26, %if.end26, %code_signed.exit69
  %sub29.sink = phi i32 [ %sub29, %code_signed.exit69 ], [ -1, %if.end26 ], [ -1, %if.end26 ], [ -1, %if.end26 ]
  %spec.select = add nsw i32 %retval.0.i.i, %sub29.sink
  %sh_prom30.pn = zext nneg i32 %spec.select to i64
  %cond.in = shl nsw i64 -1, %sh_prom30.pn
  %cond = xor i64 %cond.in, -1
  %conv36 = sitofp i64 %iv.0 to double
  %conv37 = uitofp nneg i64 %cond to double
  %div = fdiv double %conv36, %conv37
  %cmp.i = fcmp oge double %div, -1.000000e+00
  %cond.i70 = select i1 %cmp.i, double %div, double -1.000000e+00
  br label %cleanup40

cleanup40:                                        ; preds = %if.then24, %code_signed.exit69.thread, %if.then7, %half_to_double.exit
  %retval.1 = phi double [ %cond18.i, %half_to_double.exit ], [ %conv.i64, %if.then7 ], [ %cond.i70, %code_signed.exit69.thread ], [ %conv25, %if.then24 ]
  ret double %retval.1
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define dso_local <4 x i32> @__zluda_ptx_impl_agx_tld4(ptr addrspace(1) nofree noundef readonly captures(none) %t, ptr addrspace(1) nofree noundef readonly captures(address_is_null) %s, i32 noundef %geom, i32 noundef %comp, i32 noundef %dtype, i32 noundef %c0, i32 noundef %c1, i32 noundef %c2, i32 noundef %c3, i32 noundef %o0, i32 noundef %o1, i32 noundef %has_ref, i32 noundef %ref) local_unnamed_addr #3 {
entry:
  %q = alloca %struct.q, align 4
  %tobool.not.i = icmp eq ptr addrspace(1) %s, null
  br i1 %tobool.not.i, label %if.else.i, label %if.then.i

if.then.i:                                        ; preds = %entry
  %arrayidx.i38.i = getelementptr inbounds nuw i8, ptr addrspace(1) %s, i64 16
  %0 = load i64, ptr addrspace(1) %arrayidx.i38.i, align 8
  %arrayidx.i39.i = getelementptr inbounds nuw i8, ptr addrspace(1) %s, i64 24
  %1 = load i64, ptr addrspace(1) %arrayidx.i39.i, align 8
  %tobool4.not.i = icmp eq i64 %1, 0
  br i1 %tobool4.not.i, label %sampling_of.exit, label %land.rhs.i

land.rhs.i:                                       ; preds = %if.then.i
  %arrayidx.i40.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 48
  %2 = load i64, ptr addrspace(1) %arrayidx.i40.i, align 8
  %tobool6.i = icmp ne i64 %2, 0
  %3 = zext i1 %tobool6.i to i32
  br label %sampling_of.exit

if.else.i:                                        ; preds = %entry
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 40
  %4 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 48
  %5 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %5 to i32
  br label %sampling_of.exit

sampling_of.exit:                                 ; preds = %if.then.i, %land.rhs.i, %if.else.i
  %retval.sroa.6.0.i = phi i32 [ %conv21.i, %if.else.i ], [ 0, %if.then.i ], [ %3, %land.rhs.i ]
  %retval.sroa.3.0.in.i = phi i64 [ %4, %if.else.i ], [ %0, %if.then.i ], [ %0, %land.rhs.i ]
  %retval.sroa.3.0.i = trunc i64 %retval.sroa.3.0.in.i to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q)
  store i32 0, ptr %q, align 4
  %off = getelementptr inbounds nuw i8, ptr %q, i64 4
  store i32 %o0, ptr %off, align 4
  %arrayinit.element = getelementptr inbounds nuw i8, ptr %q, i64 8
  store i32 %o1, ptr %arrayinit.element, align 4
  %arrayinit.element1 = getelementptr inbounds nuw i8, ptr %q, i64 12
  store i32 0, ptr %arrayinit.element1, align 4
  %has_ref2 = getelementptr inbounds nuw i8, ptr %q, i64 16
  store i32 %has_ref, ptr %has_ref2, align 4
  %ref3 = getelementptr inbounds nuw i8, ptr %q, i64 20
  store i32 %ref, ptr %ref3, align 4
  %6 = add i32 %geom, -7
  %7 = icmp ult i32 %6, -2
  br i1 %7, label %if.else, label %if.then

if.then:                                          ; preds = %sampling_of.exit
  %cmp.not = icmp eq i32 %geom, 6
  %8 = bitcast i32 %c1 to float
  %9 = bitcast i32 %c0 to float
  %10 = select i1 %cmp.not, float %8, float %9
  %conv.i = fpext float %10 to double
  %11 = bitcast i32 %c2 to float
  %12 = select i1 %cmp.not, float %11, float %8
  %conv.i79 = fpext float %12 to double
  %13 = bitcast i32 %c3 to float
  %14 = select i1 %cmp.not, float %13, float %11
  %conv.i80 = fpext float %14 to double
  %15 = tail call double @llvm.fabs.f64(double %conv.i)
  %16 = tail call double @llvm.fabs.f64(double %conv.i79)
  %17 = tail call double @llvm.fabs.f64(double %conv.i80)
  %cmp.i.i = fcmp ult double %15, %16
  %cmp1.i.i = fcmp ult double %15, %17
  %or.cond.i.i = or i1 %cmp.i.i, %cmp1.i.i
  br i1 %or.cond.i.i, label %if.end.i.i, label %cube_face.exit.i

if.end.i.i:                                       ; preds = %if.then
  %cmp3.i.i = fcmp ule double %16, %15
  %cmp5.i.i = fcmp ult double %16, %17
  %or.cond23.i.i = or i1 %cmp3.i.i, %cmp5.i.i
  %cond.v.i199 = select i1 %or.cond23.i.i, double %conv.i80, double %conv.i79
  %cond.i200 = tail call double @llvm.fabs.f64(double %cond.v.i199)
  %or.cond23.i.not13.i.demorgan = or i1 %cmp3.i.i, %cmp5.i.i
  %cmp10.i.i88 = fcmp ult float %14, 0.000000e+00
  %or.cond.i.not = and i1 %cmp10.i.i88, %or.cond23.i.not13.i.demorgan
  %fneg3.i = fneg double %conv.i
  %spec.select12.i = select i1 %or.cond.i.not, double %fneg3.i, double %conv.i
  br label %cube_sc.exit

cube_face.exit.i:                                 ; preds = %if.then
  %cmp2.i.i84 = fcmp oge float %10, 0.000000e+00
  %fneg.i = fneg double %conv.i80
  %spec.select.i = select i1 %cmp2.i.i84, double %fneg.i, double %conv.i80
  br label %cube_sc.exit

cube_sc.exit:                                     ; preds = %if.end.i.i, %cube_face.exit.i
  %cond5.i202 = phi double [ %cond.i200, %if.end.i.i ], [ %15, %cube_face.exit.i ]
  %retval.0.i = phi double [ %spec.select12.i, %if.end.i.i ], [ %spec.select.i, %cube_face.exit.i ]
  %cmp.i89 = fcmp ogt double %cond5.i202, 0.000000e+00
  br i1 %cmp.i89, label %cond.true.i, label %cube_norm.exit

cond.true.i:                                      ; preds = %cube_sc.exit
  %div.i91 = fdiv double %retval.0.i, %cond5.i202
  %add.i = fadd double %div.i91, 1.000000e+00
  %mul.i = fmul double %add.i, 5.000000e-01
  br label %cube_norm.exit

cube_norm.exit:                                   ; preds = %cube_sc.exit, %cond.true.i
  %cond.i90 = phi double [ %mul.i, %cond.true.i ], [ 5.000000e-01, %cube_sc.exit ]
  br i1 %or.cond.i.i, label %if.end.i.i96, label %sw.default.i

if.end.i.i96:                                     ; preds = %cube_norm.exit
  %cmp3.i.i97 = fcmp ule double %16, %15
  %cmp5.i.i98 = fcmp ult double %16, %17
  %or.cond23.i.i99 = or i1 %cmp3.i.i97, %cmp5.i.i98
  br i1 %or.cond23.i.i99, label %sw.default.i, label %if.then6.i.i100

if.then6.i.i100:                                  ; preds = %if.end.i.i96
  %cmp7.i.i101 = fcmp oge float %12, 0.000000e+00
  %fneg.i102 = fneg double %conv.i80
  %spec.select.i103 = select i1 %cmp7.i.i101, double %conv.i80, double %fneg.i102
  br label %cube_tc.exit

sw.default.i:                                     ; preds = %if.end.i.i96, %cube_norm.exit
  %fneg2.i = fneg double %conv.i79
  br label %cube_tc.exit

cube_tc.exit:                                     ; preds = %if.then6.i.i100, %sw.default.i
  %retval.0.i95 = phi double [ %fneg2.i, %sw.default.i ], [ %spec.select.i103, %if.then6.i.i100 ]
  br i1 %cmp.i89, label %cond.true.i106, label %cube_norm.exit110

cond.true.i106:                                   ; preds = %cube_tc.exit
  %div.i107 = fdiv double %retval.0.i95, %cond5.i202
  %add.i108 = fadd double %div.i107, 1.000000e+00
  %mul.i109 = fmul double %add.i108, 5.000000e-01
  br label %cube_norm.exit110

cube_norm.exit110:                                ; preds = %cube_tc.exit, %cond.true.i106
  %cond.i105 = phi double [ %mul.i109, %cond.true.i106 ], [ 5.000000e-01, %cube_tc.exit ]
  %arrayidx.i.i.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 64
  %18 = load i64, ptr addrspace(1) %arrayidx.i.i.i, align 8
  br i1 %or.cond.i.i, label %if.end.i.i123, label %if.then.i.i114

if.then.i.i114:                                   ; preds = %cube_norm.exit110
  %cmp2.i.i115 = fcmp ult float %10, 0.000000e+00
  %cond.i6.i = zext i1 %cmp2.i.i115 to i64
  br label %cube_layer.exit

if.end.i.i123:                                    ; preds = %cube_norm.exit110
  %cmp3.i.i124 = fcmp ule double %16, %15
  %cmp5.i.i125 = fcmp ult double %16, %17
  %or.cond23.i.i126 = or i1 %cmp3.i.i124, %cmp5.i.i125
  br i1 %or.cond23.i.i126, label %if.end9.i.i130, label %if.then6.i.i127

if.then6.i.i127:                                  ; preds = %if.end.i.i123
  %cmp7.i.i128 = fcmp oge float %12, 0.000000e+00
  %cond8.i.i129 = select i1 %cmp7.i.i128, i64 2, i64 3
  br label %cube_layer.exit

if.end9.i.i130:                                   ; preds = %if.end.i.i123
  %cmp10.i.i131 = fcmp oge float %14, 0.000000e+00
  %cond11.i.i132 = select i1 %cmp10.i.i131, i64 4, i64 5
  br label %cube_layer.exit

cube_layer.exit:                                  ; preds = %if.then.i.i114, %if.then6.i.i127, %if.end9.i.i130
  %retval.0.i.i117 = phi i64 [ %cond.i6.i, %if.then.i.i114 ], [ %cond8.i.i129, %if.then6.i.i127 ], [ %cond11.i.i132, %if.end9.i.i130 ]
  %narrow.i = select i1 %cmp.not, i32 %c0, i32 0
  %cond.i119 = zext i32 %narrow.i to i64
  %cond.i.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %18, i64 1)
  %div7.i = udiv i64 %cond.i.i.i, 6
  %cond.i.i120 = tail call i64 @llvm.usub.sat.i64(i64 %div7.i, i64 1)
  %19 = tail call i64 @llvm.umin.i64(i64 %cond.i119, i64 %cond.i.i120)
  %mul.i121 = mul nuw nsw i64 %19, 6
  %add.i122 = add nuw nsw i64 %retval.0.i.i117, %mul.i121
  %20 = insertelement <2 x double> poison, double %cond.i90, i64 0
  %21 = insertelement <2 x double> %20, double %cond.i105, i64 1
  br label %if.end

if.else:                                          ; preds = %sampling_of.exit
  %22 = add i32 %geom, -3
  %or.cond.i133 = icmp ult i32 %22, 2
  %23 = insertelement <2 x i32> poison, i32 %c1, i64 0
  %24 = insertelement <2 x i32> %23, i32 %c2, i64 1
  %25 = insertelement <2 x i32> poison, i32 %c0, i64 0
  %26 = insertelement <2 x i32> %25, i32 %c1, i64 1
  %27 = insertelement <2 x i1> poison, i1 %or.cond.i133, i64 0
  %28 = shufflevector <2 x i1> %27, <2 x i1> poison, <2 x i32> zeroinitializer
  %.v = select <2 x i1> %28, <2 x i32> %24, <2 x i32> %26
  %29 = bitcast <2 x i32> %.v to <2 x float>
  %30 = fpext <2 x float> %29 to <2 x double>
  %geom.off = add i32 %geom, -3
  %switch = icmp ult i32 %geom.off, 2
  br i1 %switch, label %cond.true, label %if.end

cond.true:                                        ; preds = %if.else
  %conv.i138 = zext i32 %c0 to i64
  %arrayidx.i.i.i139 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 64
  %31 = load i64, ptr addrspace(1) %arrayidx.i.i.i139, align 8
  %cond.i.i.i140 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %31, i64 1)
  %sub.i = add nsw i64 %cond.i.i.i140, -1
  %32 = tail call range(i64 0, 4294967296) i64 @llvm.umin.i64(i64 %conv.i138, i64 %sub.i)
  br label %if.end

if.end:                                           ; preds = %if.else, %cond.true, %cube_layer.exit
  %layer.0 = phi i64 [ %add.i122, %cube_layer.exit ], [ %32, %cond.true ], [ 0, %if.else ]
  %norm.0 = phi i32 [ 1, %cube_layer.exit ], [ %retval.sroa.6.0.i, %cond.true ], [ %retval.sroa.6.0.i, %if.else ]
  %33 = phi <2 x double> [ %21, %cube_layer.exit ], [ %30, %cond.true ], [ %30, %if.else ]
  %arrayidx.i.i.i.i = getelementptr i8, ptr addrspace(1) %t, i64 72
  %cmp.i.i141 = icmp eq i32 %norm.0, 1
  %conv3.i = sext i32 %o0 to i64
  %34 = load <2 x i64>, ptr addrspace(1) %arrayidx.i.i.i.i, align 8
  %35 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %34, <2 x i64> splat (i64 1))
  %36 = uitofp nneg <2 x i64> %35 to <2 x double>
  %37 = fmul <2 x double> %33, %36
  %38 = insertelement <2 x i1> poison, i1 %cmp.i.i141, i64 0
  %39 = shufflevector <2 x i1> %38, <2 x i1> poison, <2 x i32> zeroinitializer
  %40 = select <2 x i1> %39, <2 x double> %37, <2 x double> %33
  %41 = fadd <2 x double> %40, splat (double -5.000000e-01)
  %42 = tail call <2 x double> @llvm.floor.v2f64(<2 x double> %41)
  %43 = extractelement <2 x double> %42, i64 0
  %conv.i9.i = fptosi double %43 to i64
  %invariant.op = add i64 %conv3.i, %conv.i9.i
  %44 = extractelement <2 x double> %42, i64 1
  %conv.i9.i160 = fptosi double %44 to i64
  %conv3.i163 = sext i32 %o1 to i64
  %invariant.op206 = add i64 %conv3.i163, %conv.i9.i160
  %45 = shl nuw nsw <2 x i64> %35, splat (i64 1)
  %46 = add nsw <2 x i64> %35, splat (i64 -1)
  %cmp.i186 = icmp eq i32 %dtype, 0
  br label %for.body

for.cond.cleanup:                                 ; preds = %out_bits.exit
  call void @llvm.lifetime.end.p0(ptr nonnull %q)
  ret <4 x i32> %vecins

for.body:                                         ; preds = %if.end, %out_bits.exit
  %i.0208 = phi i32 [ 0, %if.end ], [ %inc, %out_bits.exit ]
  %out.0207 = phi <4 x i32> [ undef, %if.end ], [ %vecins, %out_bits.exit ]
  %47 = add nsw i32 %i.0208, -1
  %48 = icmp ult i32 %47, 2
  %cmp41 = icmp samesign ult i32 %i.0208, 2
  %conv.i144 = zext i1 %48 to i64
  %add4.i.reass = add i64 %invariant.op, %conv.i144
  switch i32 %retval.sroa.3.0.i, label %lin_idx.exit [
    i32 0, label %lin_idx.exit.thread
    i32 2, label %lin_idx.exit.thread219
    i32 3, label %lin_idx.exit.thread225
  ]

lin_idx.exit.thread:                              ; preds = %for.body
  %conv.i161214 = zext i1 %cmp41 to i64
  %add4.i165.reass215 = add i64 %invariant.op206, %conv.i161214
  %49 = insertelement <2 x i64> poison, i64 %add4.i.reass, i64 0
  %50 = insertelement <2 x i64> %49, i64 %add4.i165.reass215, i64 1
  %51 = srem <2 x i64> %50, %35
  %52 = add nsw <2 x i64> %51, %35
  %53 = srem <2 x i64> %52, %35
  br label %lin_idx.exit185

lin_idx.exit.thread219:                           ; preds = %for.body
  %conv.i161221 = zext i1 %cmp41 to i64
  %add4.i165.reass222 = add i64 %invariant.op206, %conv.i161221
  %54 = insertelement <2 x i64> poison, i64 %add4.i.reass, i64 0
  %55 = insertelement <2 x i64> %54, i64 %add4.i165.reass222, i64 1
  %56 = icmp slt <2 x i64> %55, %35
  %57 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %55, <2 x i64> splat (i64 -1))
  %58 = select <2 x i1> %56, <2 x i64> %57, <2 x i64> splat (i64 -1)
  br label %lin_idx.exit185

lin_idx.exit.thread225:                           ; preds = %for.body
  %conv.i161227 = zext i1 %cmp41 to i64
  %add4.i165.reass228 = add i64 %invariant.op206, %conv.i161227
  %59 = insertelement <2 x i64> poison, i64 %add4.i.reass, i64 0
  %60 = insertelement <2 x i64> %59, i64 %add4.i165.reass228, i64 1
  %61 = srem <2 x i64> %60, %45
  %62 = add nsw <2 x i64> %61, %45
  %63 = srem <2 x i64> %62, %45
  %64 = icmp slt <2 x i64> %63, %35
  %65 = xor <2 x i64> %63, splat (i64 -1)
  %66 = add <2 x i64> %45, %65
  %67 = select <2 x i1> %64, <2 x i64> %63, <2 x i64> %66
  br label %lin_idx.exit185

lin_idx.exit:                                     ; preds = %for.body
  %conv.i161 = zext i1 %cmp41 to i64
  %add4.i165.reass = add i64 %invariant.op206, %conv.i161
  %68 = insertelement <2 x i64> poison, i64 %add4.i.reass, i64 0
  %69 = insertelement <2 x i64> %68, i64 %add4.i165.reass, i64 1
  %70 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %69, <2 x i64> zeroinitializer)
  %71 = tail call <2 x i64> @llvm.umin.v2i64(<2 x i64> %70, <2 x i64> %46)
  br label %lin_idx.exit185

lin_idx.exit185:                                  ; preds = %lin_idx.exit.thread, %lin_idx.exit.thread219, %lin_idx.exit.thread225, %lin_idx.exit
  %72 = phi <2 x i64> [ %71, %lin_idx.exit ], [ %53, %lin_idx.exit.thread ], [ %58, %lin_idx.exit.thread219 ], [ %67, %lin_idx.exit.thread225 ]
  %73 = extractelement <2 x i64> %72, i64 0
  %74 = extractelement <2 x i64> %72, i64 1
  %call45 = call fastcc <4 x double> @__zluda_agx_tex_texel(ptr addrspace(1) noundef %t, ptr noundef %q, i64 noundef %73, i64 noundef %74, i64 noundef 0, i64 noundef %layer.0)
  %vecext = extractelement <4 x double> %call45, i32 %comp
  br i1 %cmp.i186, label %if.then.i189, label %if.end.i

if.then.i189:                                     ; preds = %lin_idx.exit185
  %conv.i190 = fptrunc double %vecext to float
  %75 = bitcast float %conv.i190 to i32
  br label %out_bits.exit

if.end.i:                                         ; preds = %lin_idx.exit185
  %cmp1.i187 = fcmp uno double %vecext, 0.000000e+00
  br i1 %cmp1.i187, label %out_bits.exit, label %if.end4.i

if.end4.i:                                        ; preds = %if.end.i
  %conv5.i = fptosi double %vecext to i64
  %conv6.i = trunc i64 %conv5.i to i32
  br label %out_bits.exit

out_bits.exit:                                    ; preds = %if.then.i189, %if.end.i, %if.end4.i
  %retval.0.i188 = phi i32 [ %75, %if.then.i189 ], [ %conv6.i, %if.end4.i ], [ 0, %if.end.i ]
  %vecins = insertelement <4 x i32> %out.0207, i32 %retval.0.i188, i32 %i.0208
  %inc = add nuw nsw i32 %i.0208, 1
  %exitcond.not = icmp eq i32 %inc, 4
  br i1 %exitcond.not, label %for.cond.cleanup, label %for.body, !llvm.loop !10027
}

; Function Attrs: inlinehint nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define internal fastcc <4 x double> @__zluda_agx_tex_texel(ptr addrspace(1) nofree noundef readonly captures(none) %t, ptr nofree noundef nonnull readonly captures(none) %q, i64 noundef %i, i64 noundef %j, i64 noundef %k, i64 noundef %l) unnamed_addr #3 {
entry:
  %0 = load i32, ptr %q, align 4
  %1 = or i64 %j, %i
  %2 = or i64 %1, %k
  %or.cond49.not.i = icmp sgt i64 %2, -1
  br i1 %or.cond49.not.i, label %if.end.i, label %texel0.exit

if.end.i:                                         ; preds = %entry
  %arrayidx.i.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 24
  %3 = load i64, ptr addrspace(1) %arrayidx.i.i, align 8
  %arrayidx.i85.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 16
  %4 = load i64, ptr addrspace(1) %arrayidx.i85.i, align 8
  switch i64 %4, label %sw.default.i.i [
    i64 0, label %code_bytes.exit.i
    i64 20, label %code_bytes.exit.i
    i64 1, label %sw.bb1.i.i
    i64 21, label %sw.bb1.i.i
    i64 6, label %sw.bb1.i.i
  ]

sw.bb1.i.i:                                       ; preds = %if.end.i, %if.end.i, %if.end.i
  br label %code_bytes.exit.i

sw.default.i.i:                                   ; preds = %if.end.i
  br label %code_bytes.exit.i

code_bytes.exit.i:                                ; preds = %sw.default.i.i, %sw.bb1.i.i, %if.end.i, %if.end.i
  %retval.0.i.i = phi i64 [ 4, %sw.default.i.i ], [ 2, %sw.bb1.i.i ], [ 1, %if.end.i ], [ 1, %if.end.i ]
  %arrayidx.i86.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 112
  %5 = load i64, ptr addrspace(1) %arrayidx.i86.i, align 8
  %cmp4.i.i = icmp sgt i32 %0, 0
  br i1 %cmp4.i.i, label %for.body.lr.ph.i.i, label %code_bytes.exit.lv_off.exit_crit_edge.i

code_bytes.exit.lv_off.exit_crit_edge.i:          ; preds = %code_bytes.exit.i
  %arrayidx.i.i.i.phi.trans.insert.i = getelementptr i8, ptr addrspace(1) %t, i64 88
  %.pre.i = load i64, ptr addrspace(1) %arrayidx.i.i.i.phi.trans.insert.i, align 8
  %arrayidx.i.i.i87.phi.trans.insert.i = getelementptr i8, ptr addrspace(1) %t, i64 80
  %.pre108.i = load i64, ptr addrspace(1) %arrayidx.i.i.i87.phi.trans.insert.i, align 8
  %.pre109.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %.pre.i, i64 1)
  %.pre110.i = zext nneg i32 %0 to i64
  %.pre111.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %.pre108.i, i64 1)
  br label %lv_off.exit.i

for.body.lr.ph.i.i:                               ; preds = %code_bytes.exit.i
  %arrayidx.i.i.i.i.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 64
  %6 = load <4 x i64>, ptr addrspace(1) %arrayidx.i.i.i.i.i, align 8
  %7 = tail call <4 x i64> @llvm.smax.v4i64(<4 x i64> %6, <4 x i64> splat (i64 1))
  %wide.trip.count.i.i = zext nneg i32 %0 to i64
  %8 = extractelement <4 x i64> %7, i64 2
  %9 = extractelement <4 x i64> %7, i64 3
  br label %for.body.i.i

for.body.i.i:                                     ; preds = %for.body.i.i, %for.body.lr.ph.i.i
  %indvars.iv.i.i = phi i64 [ 0, %for.body.lr.ph.i.i ], [ %indvars.iv.next.i.i, %for.body.i.i ]
  %off.05.i.i = phi i64 [ 0, %for.body.lr.ph.i.i ], [ %add.i.i, %for.body.i.i ]
  %10 = insertelement <4 x i64> <i64 0, i64 poison, i64 poison, i64 poison>, i64 %indvars.iv.i.i, i64 1
  %11 = shufflevector <4 x i64> %10, <4 x i64> poison, <4 x i32> <i32 0, i32 1, i32 1, i32 1>
  %12 = lshr <4 x i64> %7, %11
  %13 = tail call <4 x i64> @llvm.umax.v4i64(<4 x i64> %12, <4 x i64> <i64 0, i64 1, i64 1, i64 1>)
  %14 = tail call i64 @llvm.vector.reduce.mul.v4i64(<4 x i64> %13)
  %add.i.i = add nuw nsw i64 %14, %off.05.i.i
  %indvars.iv.next.i.i = add nuw nsw i64 %indvars.iv.i.i, 1
  %exitcond.not.i.i = icmp eq i64 %indvars.iv.next.i.i, %wide.trip.count.i.i
  br i1 %exitcond.not.i.i, label %lv_off.exit.i, label %for.body.i.i, !llvm.loop !10028

lv_off.exit.i:                                    ; preds = %for.body.i.i, %code_bytes.exit.lv_off.exit_crit_edge.i
  %cond.i.i.i88.pre-phi.i = phi i64 [ %.pre111.i, %code_bytes.exit.lv_off.exit_crit_edge.i ], [ %8, %for.body.i.i ]
  %sh_prom.i.pre-phi.i = phi i64 [ %.pre110.i, %code_bytes.exit.lv_off.exit_crit_edge.i ], [ %wide.trip.count.i.i, %for.body.i.i ]
  %cond.i.i.i.pre-phi.i = phi i64 [ %.pre109.i, %code_bytes.exit.lv_off.exit_crit_edge.i ], [ %9, %for.body.i.i ]
  %off.0.lcssa.i.i = phi i64 [ 0, %code_bytes.exit.lv_off.exit_crit_edge.i ], [ %add.i.i, %for.body.i.i ]
  %shr.i.i = lshr i64 %cond.i.i.i.pre-phi.i, %sh_prom.i.pre-phi.i
  %15 = tail call range(i64 1, -9223372036854775808) i64 @llvm.umax.i64(i64 %shr.i.i, i64 1)
  %16 = getelementptr i8, ptr addrspace(1) %t, i64 8
  %shr.i90.i = lshr i64 %cond.i.i.i88.pre-phi.i, %sh_prom.i.pre-phi.i
  %17 = tail call range(i64 1, -9223372036854775808) i64 @llvm.umax.i64(i64 %shr.i90.i, i64 1)
  %tobool.not.i = icmp eq i64 %5, 0
  br i1 %tobool.not.i, label %cond.false.i, label %cond.true.i

cond.true.i:                                      ; preds = %lv_off.exit.i
  %18 = load i64, ptr addrspace(1) %16, align 8
  %mul18.i = mul nsw i64 %15, %l
  %add19.i = add nsw i64 %mul18.i, %k
  %mul21.i = mul nsw i64 %add19.i, %17
  %add22.i = add nsw i64 %mul21.i, %j
  %mul23.i = mul nsw i64 %add22.i, %5
  %mul24.i = mul nsw i64 %3, %i
  %mul25.i = mul nsw i64 %mul24.i, %retval.0.i.i
  %add26.i = add nsw i64 %mul23.i, %mul25.i
  %add27.i = add i64 %add26.i, %18
  br label %cond.end.i

cond.false.i:                                     ; preds = %lv_off.exit.i
  %arrayidx.i.i.i91.i = getelementptr i8, ptr addrspace(1) %t, i64 72
  %19 = load i64, ptr addrspace(1) %arrayidx.i.i.i91.i, align 8
  %cond.i.i.i92.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %19, i64 1)
  %shr.i94.i = lshr i64 %cond.i.i.i92.i, %sh_prom.i.pre-phi.i
  %20 = tail call range(i64 1, -9223372036854775808) i64 @llvm.umax.i64(i64 %shr.i94.i, i64 1)
  %mul.i = mul nsw i64 %15, %l
  %add.i = add nsw i64 %mul.i, %k
  %mul10.i = mul nsw i64 %add.i, %17
  %add11.i = add nsw i64 %mul10.i, %j
  %mul13.i = mul nsw i64 %20, %add11.i
  %add14.i = add i64 %off.0.lcssa.i.i, %i
  %add15.i = add i64 %add14.i, %mul13.i
  %21 = load i64, ptr addrspace(1) %16, align 8
  %mul29.i = mul i64 %retval.0.i.i, %3
  %mul31.i = mul i64 %mul29.i, %add15.i
  %add32.i = add i64 %mul31.i, %21
  br label %cond.end.i

cond.end.i:                                       ; preds = %cond.false.i, %cond.true.i
  %cond.i = phi i64 [ %add27.i, %cond.true.i ], [ %add32.i, %cond.false.i ]
  br label %for.body.i

for.body.i:                                       ; preds = %cond.end47.i, %cond.end.i
  %indvars.iv.i = phi i64 [ 0, %cond.end.i ], [ %indvars.iv.next.i, %cond.end47.i ]
  %r.0105.i = phi <4 x double> [ zeroinitializer, %cond.end.i ], [ %vecins.i, %cond.end47.i ]
  %cmp36.i = icmp sgt i64 %3, %indvars.iv.i
  br i1 %cmp36.i, label %cond.true38.i, label %cond.false43.i

cond.true38.i:                                    ; preds = %for.body.i
  %22 = mul nuw nsw i64 %indvars.iv.i, %retval.0.i.i
  %add41.i = add i64 %22, %cond.i
  %call42.i = tail call fastcc double @__zluda_agx_tex_channel(ptr addrspace(1) noundef readonly %t, i64 noundef %add41.i)
  br label %cond.end47.i

cond.false43.i:                                   ; preds = %for.body.i
  %cmp44.i = icmp eq i64 %indvars.iv.i, 3
  %cond46.i = select i1 %cmp44.i, double 1.000000e+00, double 0.000000e+00
  br label %cond.end47.i

cond.end47.i:                                     ; preds = %cond.false43.i, %cond.true38.i
  %cond48.i = phi double [ %call42.i, %cond.true38.i ], [ %cond46.i, %cond.false43.i ]
  %23 = trunc nuw nsw i64 %indvars.iv.i to i32
  %vecins.i = insertelement <4 x double> %r.0105.i, double %cond48.i, i32 %23
  %indvars.iv.next.i = add nuw nsw i64 %indvars.iv.i, 1
  %exitcond.not.i = icmp eq i64 %indvars.iv.next.i, 4
  br i1 %exitcond.not.i, label %texel0.exit, label %for.body.i, !llvm.loop !10029

texel0.exit:                                      ; preds = %cond.end47.i, %entry
  %retval.0.i = phi <4 x double> [ zeroinitializer, %entry ], [ %vecins.i, %cond.end47.i ]
  %has_ref = getelementptr inbounds nuw i8, ptr %q, i64 16
  %24 = load i32, ptr %has_ref, align 4
  %tobool.not = icmp eq i32 %24, 0
  br i1 %tobool.not, label %cleanup, label %if.end

if.end:                                           ; preds = %texel0.exit
  %ref = getelementptr inbounds nuw i8, ptr %q, i64 20
  %25 = load float, ptr %ref, align 4
  %conv.i = fpext float %25 to double
  %vecext = extractelement <4 x double> %retval.0.i, i64 0
  %cmp = fcmp oge double %vecext, %conv.i
  %cond = select i1 %cmp, double 1.000000e+00, double 0.000000e+00
  %vecinit4 = insertelement <4 x double> <double poison, double 0.000000e+00, double 0.000000e+00, double 1.000000e+00>, double %cond, i64 0
  br label %cleanup

cleanup:                                          ; preds = %texel0.exit, %if.end
  %retval.0 = phi <4 x double> [ %vecinit4, %if.end ], [ %retval.0.i, %texel0.exit ]
  ret <4 x double> %retval.0
}

; Function Attrs: alwaysinline mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read)
define dso_local i32 @__zluda_ptx_impl_agx_txq(ptr addrspace(1) nofree noundef readonly captures(none) %t, i32 noundef %field) local_unnamed_addr #3 {
entry:
  %0 = load i64, ptr addrspace(1) %t, align 8
  %cmp = icmp eq i64 %0, 1396920397
  br i1 %cmp, label %if.then, label %if.end

if.then:                                          ; preds = %entry
  switch i32 %field, label %return [
    i32 5, label %sw.bb
    i32 6, label %sw.bb2
    i32 4, label %sw.bb5
    i32 13, label %sw.bb8
  ]

sw.bb:                                            ; preds = %if.then
  %arrayidx.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 8
  %1 = load i64, ptr addrspace(1) %arrayidx.i, align 8
  %conv = trunc i64 %1 to i32
  br label %return

sw.bb2:                                           ; preds = %if.then
  %arrayidx.i76 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 16
  %2 = load i64, ptr addrspace(1) %arrayidx.i76, align 8
  switch i64 %2, label %sw.default.i87 [
    i64 1, label %return
    i64 2, label %sw.bb1.i86
    i64 3, label %sw.bb2.i84
  ]

sw.bb5:                                           ; preds = %if.then
  %arrayidx.i77 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 24
  %3 = load i64, ptr addrspace(1) %arrayidx.i77, align 8
  %conv7 = trunc i64 %3 to i32
  br label %return

sw.bb8:                                           ; preds = %if.then
  %arrayidx.i78 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 24
  %4 = load i64, ptr addrspace(1) %arrayidx.i78, align 8
  %tobool.not = icmp eq i64 %4, 0
  %lnot.ext = zext i1 %tobool.not to i32
  br label %return

if.end:                                           ; preds = %entry
  switch i32 %field, label %return [
    i32 0, label %sw.bb10
    i32 1, label %sw.bb10
    i32 2, label %sw.bb10
    i32 3, label %sw.bb13
    i32 4, label %sw.bb29
    i32 5, label %sw.bb32
    i32 6, label %sw.bb35
    i32 7, label %sw.bb38
    i32 12, label %sw.bb48
    i32 13, label %sw.bb51
  ]

sw.bb10:                                          ; preds = %if.end, %if.end, %if.end
  %5 = zext nneg i32 %field to i64
  %6 = getelementptr [8 x i8], ptr addrspace(1) %t, i64 %5
  %arrayidx.i.i = getelementptr i8, ptr addrspace(1) %6, i64 72
  %7 = load i64, ptr addrspace(1) %arrayidx.i.i, align 8
  %cond.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %7, i64 1)
  %conv12 = trunc i64 %cond.i.i to i32
  br label %return

sw.bb13:                                          ; preds = %if.end
  %arrayidx.i79 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 64
  %8 = load i64, ptr addrspace(1) %arrayidx.i79, align 8
  %arrayidx.i80 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 104
  %9 = load i64, ptr addrspace(1) %arrayidx.i80, align 8
  %and = and i64 %9, 2
  %tobool16.not = icmp eq i64 %and, 0
  br i1 %tobool16.not, label %if.end21, label %if.then17

if.then17:                                        ; preds = %sw.bb13
  %cmp18 = icmp ugt i64 %8, 6
  %div = udiv i64 %8, 6
  %conv20 = trunc i64 %div to i32
  %cond = select i1 %cmp18, i32 %conv20, i32 0
  br label %return

if.end21:                                         ; preds = %sw.bb13
  %cmp22 = icmp ugt i64 %8, 1
  %conv25 = trunc i64 %8 to i32
  %cond28 = select i1 %cmp22, i32 %conv25, i32 0
  br label %return

sw.bb29:                                          ; preds = %if.end
  %arrayidx.i81 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 48
  %10 = load i64, ptr addrspace(1) %arrayidx.i81, align 8
  %conv31 = trunc i64 %10 to i32
  br label %return

sw.bb32:                                          ; preds = %if.end
  %arrayidx.i82 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 32
  %11 = load i64, ptr addrspace(1) %arrayidx.i82, align 8
  %conv34 = trunc i64 %11 to i32
  br label %return

sw.bb35:                                          ; preds = %if.end
  %arrayidx.i83 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 40
  %12 = load i64, ptr addrspace(1) %arrayidx.i83, align 8
  switch i64 %12, label %sw.default.i87 [
    i64 1, label %return
    i64 2, label %sw.bb1.i86
    i64 3, label %sw.bb2.i84
  ]

sw.bb1.i86:                                       ; preds = %sw.bb2, %sw.bb35
  br label %return

sw.bb2.i84:                                       ; preds = %sw.bb2, %sw.bb35
  br label %return

sw.default.i87:                                   ; preds = %sw.bb2, %sw.bb35
  br label %return

sw.bb38:                                          ; preds = %if.end
  %arrayidx.i89 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 96
  %13 = load i64, ptr addrspace(1) %arrayidx.i89, align 8
  %cmp40 = icmp ugt i64 %13, 1
  %conv44 = trunc i64 %13 to i32
  %spec.select = select i1 %cmp40, i32 %conv44, i32 0
  br label %return

sw.bb48:                                          ; preds = %if.end
  %arrayidx.i.i91 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 104
  %14 = load i64, ptr addrspace(1) %arrayidx.i.i91, align 8
  %15 = trunc i64 %14 to i32
  %16 = lshr i32 %15, 8
  %conv50 = and i32 %16, 255
  br label %return

sw.bb51:                                          ; preds = %if.end
  %arrayidx.i92 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 48
  %17 = load i64, ptr addrspace(1) %arrayidx.i92, align 8
  %tobool53.not = icmp eq i64 %17, 0
  %lnot.ext55 = zext i1 %tobool53.not to i32
  br label %return

return:                                           ; preds = %sw.bb38, %sw.default.i87, %sw.bb2.i84, %sw.bb1.i86, %sw.bb35, %sw.bb2, %if.end, %if.then17, %if.end21, %if.then, %sw.bb51, %sw.bb48, %sw.bb32, %sw.bb29, %sw.bb10, %sw.bb8, %sw.bb5, %sw.bb
  %retval.1 = phi i32 [ %lnot.ext55, %sw.bb51 ], [ %conv, %sw.bb ], [ 0, %if.end ], [ %conv7, %sw.bb5 ], [ %lnot.ext, %sw.bb8 ], [ %spec.select, %sw.bb38 ], [ %conv12, %sw.bb10 ], [ 0, %if.then ], [ %conv31, %sw.bb29 ], [ %conv34, %sw.bb32 ], [ 3, %sw.bb2 ], [ %cond28, %if.end21 ], [ %conv50, %sw.bb48 ], [ %cond, %if.then17 ], [ 3, %sw.bb35 ], [ 0, %sw.default.i87 ], [ 1, %sw.bb2.i84 ], [ 4, %sw.bb1.i86 ]
  ret i32 %retval.1
}

; Function Attrs: alwaysinline convergent nounwind
define dso_local <4 x i64> @__zluda_ptx_impl_agx_suld_b(ptr addrspace(1) nofree noundef readonly captures(none) %f, i32 noundef %geom, i32 noundef %mode, i32 noundef %n, i32 noundef %b, i32 noundef %c0, i32 noundef %c1, i32 noundef %c2, i32 noundef %c3) local_unnamed_addr #3 {
entry:
  switch i32 %geom, label %cond.false.i52.i [
    i32 6, label %if.then.i
    i32 4, label %if.then.i
    i32 3, label %if.then.i
    i32 2, label %geom_rank.exit53.thread64.i
    i32 0, label %sf_coords.exit
  ]

if.then.i:                                        ; preds = %entry, %entry, %entry
  %cond69.i = icmp eq i32 %geom, 3
  %narrow.i = select i1 %cond69.i, i32 0, i32 %c2
  %spec.select70.i = sext i32 %narrow.i to i64
  %conv11.i = zext i32 %c0 to i64
  %0 = insertelement <2 x i64> <i64 poison, i64 0>, i64 %spec.select70.i, i64 0
  br label %sf_coords.exit

geom_rank.exit53.thread64.i:                      ; preds = %entry
  %1 = insertelement <2 x i32> poison, i32 %c1, i64 0
  %2 = insertelement <2 x i32> %1, i32 %c2, i64 1
  %3 = sext <2 x i32> %2 to <2 x i64>
  br label %sf_coords.exit

cond.false.i52.i:                                 ; preds = %entry
  %4 = sext i32 %c1 to i64
  %5 = insertelement <2 x i64> <i64 poison, i64 0>, i64 %4, i64 0
  br label %sf_coords.exit

sf_coords.exit:                                   ; preds = %geom_rank.exit53.thread64.i, %entry, %cond.false.i52.i, %if.then.i
  %xs.sroa.0.0.in = phi i32 [ %c1, %if.then.i ], [ %c0, %cond.false.i52.i ], [ %c0, %entry ], [ %c0, %geom_rank.exit53.thread64.i ]
  %.sink.i = phi i64 [ %conv11.i, %if.then.i ], [ 0, %cond.false.i52.i ], [ 0, %entry ], [ 0, %geom_rank.exit53.thread64.i ]
  %6 = phi <2 x i64> [ %0, %if.then.i ], [ %5, %cond.false.i52.i ], [ zeroinitializer, %entry ], [ %3, %geom_rank.exit53.thread64.i ]
  %xs.sroa.0.0 = sext i32 %xs.sroa.0.0.in to i64
  %conv = sext i32 %n to i64
  %conv4 = sext i32 %b to i64
  %mul = mul nsw i64 %conv4, %conv
  %cmp.i = icmp eq i32 %mode, 1
  br i1 %cmp.i, label %if.end.thread.i, label %if.end.i

if.end.thread.i:                                  ; preds = %sf_coords.exit
  %cond.i.i = tail call noundef i64 @llvm.smax.i64(i64 %xs.sroa.0.0, i64 0)
  %arrayidx.i.i.i = getelementptr i8, ptr addrspace(1) %f, i64 32
  %7 = load i64, ptr addrspace(1) %arrayidx.i.i.i, align 8
  %cond.i.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %7, i64 1)
  %arrayidx.i.i81.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 16
  %8 = load i64, ptr addrspace(1) %arrayidx.i.i81.i, align 8
  %cond.i.i82.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %8, i64 1)
  %mul.i = mul i64 %cond.i.i82.i, %cond.i.i.i
  %sub.i = sub nsw i64 %mul.i, %mul
  %cond.i83.i = tail call noundef i64 @llvm.smax.i64(i64 %sub.i, i64 0)
  %9 = tail call i64 @llvm.umin.i64(i64 %cond.i.i, i64 %cond.i83.i)
  %10 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %6, <2 x i64> zeroinitializer)
  %arrayidx.i.i86.i = getelementptr i8, ptr addrspace(1) %f, i64 40
  %11 = load <2 x i64>, ptr addrspace(1) %arrayidx.i.i86.i, align 8
  %12 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %11, <2 x i64> splat (i64 1))
  %13 = add nsw <2 x i64> %12, splat (i64 -1)
  %14 = tail call <2 x i64> @llvm.umin.v2i64(<2 x i64> %10, <2 x i64> %13)
  %arrayidx.i.i94.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 24
  %15 = load i64, ptr addrspace(1) %arrayidx.i.i94.i, align 8
  %cond.i.i95.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %15, i64 1)
  %sub15.i = add nsw i64 %cond.i.i95.i, -1
  %16 = tail call i64 @llvm.umin.i64(i64 %.sink.i, i64 %sub15.i)
  br label %lor.lhs.false.i

if.end.i:                                         ; preds = %sf_coords.exit
  %cmp17.i = icmp slt i32 %xs.sroa.0.0.in, 0
  br i1 %cmp17.i, label %if.then, label %if.end.lor.lhs.false_crit_edge.i

if.end.lor.lhs.false_crit_edge.i:                 ; preds = %if.end.i
  %arrayidx.i.i97.phi.trans.insert.i = getelementptr i8, ptr addrspace(1) %f, i64 32
  %.pre.i = load i64, ptr addrspace(1) %arrayidx.i.i97.phi.trans.insert.i, align 8
  %arrayidx.i.i99.phi.trans.insert.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 16
  %.pre124.i = load i64, ptr addrspace(1) %arrayidx.i.i99.phi.trans.insert.i, align 8
  %.pre125.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %.pre.i, i64 1)
  %.pre126.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %.pre124.i, i64 1)
  %.pre127.i = mul i64 %.pre126.i, %.pre125.i
  br label %lor.lhs.false.i

lor.lhs.false.i:                                  ; preds = %if.end.lor.lhs.false_crit_edge.i, %if.end.thread.i
  %mul20.pre-phi.i = phi i64 [ %.pre127.i, %if.end.lor.lhs.false_crit_edge.i ], [ %mul.i, %if.end.thread.i ]
  %l.addr.0123.i = phi i64 [ %.sink.i, %if.end.lor.lhs.false_crit_edge.i ], [ %16, %if.end.thread.i ]
  %x.addr.0120.i = phi i64 [ %xs.sroa.0.0, %if.end.lor.lhs.false_crit_edge.i ], [ %9, %if.end.thread.i ]
  %17 = phi <2 x i64> [ %6, %if.end.lor.lhs.false_crit_edge.i ], [ %14, %if.end.thread.i ]
  %add.i = add nsw i64 %x.addr.0120.i, %mul
  %cmp21.i = icmp sgt i64 %add.i, %mul20.pre-phi.i
  %18 = extractelement <2 x i64> %17, i64 0
  %cmp23.i = icmp slt i64 %18, 0
  %or.cond.i = select i1 %cmp21.i, i1 true, i1 %cmp23.i
  br i1 %or.cond.i, label %if.then, label %lor.lhs.false24.i

lor.lhs.false24.i:                                ; preds = %lor.lhs.false.i
  %19 = getelementptr i8, ptr addrspace(1) %f, i64 8
  %arrayidx.i.i101.i = getelementptr i8, ptr addrspace(1) %f, i64 40
  %20 = load i64, ptr addrspace(1) %arrayidx.i.i101.i, align 8
  %cond.i.i102.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %20, i64 1)
  %cmp26.i = icmp samesign uge i64 %18, %cond.i.i102.i
  %21 = extractelement <2 x i64> %17, i64 1
  %cmp28.i = icmp slt i64 %21, 0
  %or.cond52.i = select i1 %cmp26.i, i1 true, i1 %cmp28.i
  br i1 %or.cond52.i, label %if.then, label %lor.lhs.false29.i

lor.lhs.false29.i:                                ; preds = %lor.lhs.false24.i
  %arrayidx.i.i103.i = getelementptr i8, ptr addrspace(1) %f, i64 48
  %22 = load i64, ptr addrspace(1) %arrayidx.i.i103.i, align 8
  %cond.i.i104.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %22, i64 1)
  %cmp31.i.not = icmp samesign ult i64 %21, %cond.i.i104.i
  br i1 %cmp31.i.not, label %lor.lhs.false34.i, label %if.then

lor.lhs.false34.i:                                ; preds = %lor.lhs.false29.i
  %arrayidx.i.i105.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 24
  %23 = load i64, ptr addrspace(1) %arrayidx.i.i105.i, align 8
  %cond.i.i106.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %23, i64 1)
  %cmp36.not.i = icmp samesign ult i64 %l.addr.0123.i, %cond.i.i106.i
  br i1 %cmp36.not.i, label %sf_addr.exit, label %if.then

sf_addr.exit:                                     ; preds = %lor.lhs.false34.i
  %24 = load i64, ptr addrspace(1) %19, align 8
  %mul41.i = mul nuw nsw i64 %cond.i.i104.i, %l.addr.0123.i
  %add42.i = add nuw nsw i64 %mul41.i, %21
  %mul44.i = mul nuw nsw i64 %add42.i, %cond.i.i102.i
  %add45.i = add nuw nsw i64 %mul44.i, %18
  %mul49.i = mul i64 %add45.i, %mul20.pre-phi.i
  %add50.i = add nuw nsw i64 %mul49.i, %x.addr.0120.i
  %add51.i = add i64 %add50.i, %24
  %tobool.not = icmp eq i64 %add51.i, 0
  br i1 %tobool.not, label %if.then, label %if.end7

if.then:                                          ; preds = %if.end.i, %lor.lhs.false.i, %lor.lhs.false24.i, %lor.lhs.false29.i, %lor.lhs.false34.i, %sf_addr.exit
  %cmp.not = icmp eq i32 %mode, 2
  br i1 %cmp.not, label %cleanup, label %if.then6

if.then6:                                         ; preds = %if.then
  tail call void @__zluda_ptx_impl_agx_surface_trap()
  unreachable

if.end7:                                          ; preds = %sf_addr.exit
  %25 = inttoptr i64 %add51.i to ptr addrspace(1)
  %cmp843 = icmp sgt i32 %n, 0
  br i1 %cmp843, label %for.cond10.preheader.lr.ph, label %cleanup

for.cond10.preheader.lr.ph:                       ; preds = %if.end7
  %cmp1140 = icmp sgt i32 %b, 0
  %26 = zext i32 %b to i64
  %wide.trip.count52 = zext nneg i32 %n to i64
  br label %for.cond10.preheader

for.cond10.preheader:                             ; preds = %for.cond10.preheader.lr.ph, %for.cond.cleanup13
  %indvars.iv49 = phi i64 [ 0, %for.cond10.preheader.lr.ph ], [ %indvars.iv.next50, %for.cond.cleanup13 ]
  %out.044 = phi <4 x i64> [ zeroinitializer, %for.cond10.preheader.lr.ph ], [ %vecins, %for.cond.cleanup13 ]
  br i1 %cmp1140, label %for.body14.lr.ph, label %for.cond.cleanup13

for.body14.lr.ph:                                 ; preds = %for.cond10.preheader
  %27 = mul nuw nsw i64 %indvars.iv49, %26
  %invariant.gep = getelementptr inbounds nuw i8, ptr addrspace(1) %25, i64 %27
  br label %for.body14

for.cond.cleanup13:                               ; preds = %for.body14, %for.cond10.preheader
  %v.0.lcssa = phi i64 [ 0, %for.cond10.preheader ], [ %or, %for.body14 ]
  %28 = trunc nuw nsw i64 %indvars.iv49 to i32
  %vecins = insertelement <4 x i64> %out.044, i64 %v.0.lcssa, i32 %28
  %indvars.iv.next50 = add nuw nsw i64 %indvars.iv49, 1
  %exitcond53.not = icmp eq i64 %indvars.iv.next50, %wide.trip.count52
  br i1 %exitcond53.not, label %cleanup, label %for.cond10.preheader, !llvm.loop !10030

for.body14:                                       ; preds = %for.body14.lr.ph, %for.body14
  %indvars.iv = phi i64 [ 0, %for.body14.lr.ph ], [ %indvars.iv.next, %for.body14 ]
  %v.041 = phi i64 [ 0, %for.body14.lr.ph ], [ %or, %for.body14 ]
  %gep = getelementptr inbounds nuw i8, ptr addrspace(1) %invariant.gep, i64 %indvars.iv
  %29 = load i8, ptr addrspace(1) %gep, align 1
  %conv17 = zext i8 %29 to i64
  %30 = shl nuw nsw i64 %indvars.iv, 3
  %shl = shl i64 %conv17, %30
  %or = or i64 %shl, %v.041
  %indvars.iv.next = add nuw nsw i64 %indvars.iv, 1
  %exitcond.not = icmp eq i64 %indvars.iv.next, %26
  br i1 %exitcond.not, label %for.cond.cleanup13, label %for.body14, !llvm.loop !10031

cleanup:                                          ; preds = %for.cond.cleanup13, %if.end7, %if.then
  %retval.0 = phi <4 x i64> [ zeroinitializer, %if.then ], [ zeroinitializer, %if.end7 ], [ %vecins, %for.cond.cleanup13 ]
  ret <4 x i64> %retval.0
}

; Function Attrs: convergent noreturn

; Function Attrs: alwaysinline convergent nounwind
define dso_local void @__zluda_ptx_impl_agx_sust_b(ptr addrspace(1) nofree noundef readonly captures(none) %f, i32 noundef %geom, i32 noundef %mode, i32 noundef %n, i32 noundef %b, i32 noundef %c0, i32 noundef %c1, i32 noundef %c2, i32 noundef %c3, i64 noundef %v0, i64 noundef %v1, i64 noundef %v2, i64 noundef %v3) local_unnamed_addr #3 {
entry:
  %vs = alloca [4 x i64], align 8
  switch i32 %geom, label %cond.false.i52.i [
    i32 6, label %if.then.i
    i32 4, label %if.then.i
    i32 3, label %if.then.i
    i32 2, label %geom_rank.exit53.thread64.i
    i32 0, label %sf_coords.exit
  ]

if.then.i:                                        ; preds = %entry, %entry, %entry
  %cond69.i = icmp eq i32 %geom, 3
  %narrow.i = select i1 %cond69.i, i32 0, i32 %c2
  %spec.select70.i = sext i32 %narrow.i to i64
  %conv11.i = zext i32 %c0 to i64
  %0 = insertelement <2 x i64> <i64 poison, i64 0>, i64 %spec.select70.i, i64 0
  br label %sf_coords.exit

geom_rank.exit53.thread64.i:                      ; preds = %entry
  %1 = insertelement <2 x i32> poison, i32 %c1, i64 0
  %2 = insertelement <2 x i32> %1, i32 %c2, i64 1
  %3 = sext <2 x i32> %2 to <2 x i64>
  br label %sf_coords.exit

cond.false.i52.i:                                 ; preds = %entry
  %4 = sext i32 %c1 to i64
  %5 = insertelement <2 x i64> <i64 poison, i64 0>, i64 %4, i64 0
  br label %sf_coords.exit

sf_coords.exit:                                   ; preds = %geom_rank.exit53.thread64.i, %entry, %cond.false.i52.i, %if.then.i
  %xs.sroa.0.0.in = phi i32 [ %c1, %if.then.i ], [ %c0, %cond.false.i52.i ], [ %c0, %entry ], [ %c0, %geom_rank.exit53.thread64.i ]
  %.sink.i = phi i64 [ %conv11.i, %if.then.i ], [ 0, %cond.false.i52.i ], [ 0, %entry ], [ 0, %geom_rank.exit53.thread64.i ]
  %6 = phi <2 x i64> [ %0, %if.then.i ], [ %5, %cond.false.i52.i ], [ zeroinitializer, %entry ], [ %3, %geom_rank.exit53.thread64.i ]
  %xs.sroa.0.0 = sext i32 %xs.sroa.0.0.in to i64
  %conv = sext i32 %n to i64
  %conv4 = sext i32 %b to i64
  %mul = mul nsw i64 %conv4, %conv
  %cmp.i = icmp eq i32 %mode, 1
  br i1 %cmp.i, label %if.end.thread.i, label %if.end.i

if.end.thread.i:                                  ; preds = %sf_coords.exit
  %cond.i.i = tail call noundef i64 @llvm.smax.i64(i64 %xs.sroa.0.0, i64 0)
  %arrayidx.i.i.i = getelementptr i8, ptr addrspace(1) %f, i64 32
  %7 = load i64, ptr addrspace(1) %arrayidx.i.i.i, align 8
  %cond.i.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %7, i64 1)
  %arrayidx.i.i81.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 16
  %8 = load i64, ptr addrspace(1) %arrayidx.i.i81.i, align 8
  %cond.i.i82.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %8, i64 1)
  %mul.i = mul i64 %cond.i.i82.i, %cond.i.i.i
  %sub.i = sub nsw i64 %mul.i, %mul
  %cond.i83.i = tail call noundef i64 @llvm.smax.i64(i64 %sub.i, i64 0)
  %9 = tail call i64 @llvm.umin.i64(i64 %cond.i.i, i64 %cond.i83.i)
  %10 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %6, <2 x i64> zeroinitializer)
  %arrayidx.i.i86.i = getelementptr i8, ptr addrspace(1) %f, i64 40
  %11 = load <2 x i64>, ptr addrspace(1) %arrayidx.i.i86.i, align 8
  %12 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %11, <2 x i64> splat (i64 1))
  %13 = add nsw <2 x i64> %12, splat (i64 -1)
  %14 = tail call <2 x i64> @llvm.umin.v2i64(<2 x i64> %10, <2 x i64> %13)
  %arrayidx.i.i94.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 24
  %15 = load i64, ptr addrspace(1) %arrayidx.i.i94.i, align 8
  %cond.i.i95.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %15, i64 1)
  %sub15.i = add nsw i64 %cond.i.i95.i, -1
  %16 = tail call i64 @llvm.umin.i64(i64 %.sink.i, i64 %sub15.i)
  br label %lor.lhs.false.i

if.end.i:                                         ; preds = %sf_coords.exit
  %cmp17.i = icmp slt i32 %xs.sroa.0.0.in, 0
  br i1 %cmp17.i, label %if.then, label %if.end.lor.lhs.false_crit_edge.i

if.end.lor.lhs.false_crit_edge.i:                 ; preds = %if.end.i
  %arrayidx.i.i97.phi.trans.insert.i = getelementptr i8, ptr addrspace(1) %f, i64 32
  %.pre.i = load i64, ptr addrspace(1) %arrayidx.i.i97.phi.trans.insert.i, align 8
  %arrayidx.i.i99.phi.trans.insert.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 16
  %.pre124.i = load i64, ptr addrspace(1) %arrayidx.i.i99.phi.trans.insert.i, align 8
  %.pre125.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %.pre.i, i64 1)
  %.pre126.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %.pre124.i, i64 1)
  %.pre127.i = mul i64 %.pre126.i, %.pre125.i
  br label %lor.lhs.false.i

lor.lhs.false.i:                                  ; preds = %if.end.lor.lhs.false_crit_edge.i, %if.end.thread.i
  %mul20.pre-phi.i = phi i64 [ %.pre127.i, %if.end.lor.lhs.false_crit_edge.i ], [ %mul.i, %if.end.thread.i ]
  %l.addr.0123.i = phi i64 [ %.sink.i, %if.end.lor.lhs.false_crit_edge.i ], [ %16, %if.end.thread.i ]
  %x.addr.0120.i = phi i64 [ %xs.sroa.0.0, %if.end.lor.lhs.false_crit_edge.i ], [ %9, %if.end.thread.i ]
  %17 = phi <2 x i64> [ %6, %if.end.lor.lhs.false_crit_edge.i ], [ %14, %if.end.thread.i ]
  %add.i = add nsw i64 %x.addr.0120.i, %mul
  %cmp21.i = icmp sgt i64 %add.i, %mul20.pre-phi.i
  %18 = extractelement <2 x i64> %17, i64 0
  %cmp23.i = icmp slt i64 %18, 0
  %or.cond.i = select i1 %cmp21.i, i1 true, i1 %cmp23.i
  br i1 %or.cond.i, label %if.then, label %lor.lhs.false24.i

lor.lhs.false24.i:                                ; preds = %lor.lhs.false.i
  %19 = getelementptr i8, ptr addrspace(1) %f, i64 8
  %arrayidx.i.i101.i = getelementptr i8, ptr addrspace(1) %f, i64 40
  %20 = load i64, ptr addrspace(1) %arrayidx.i.i101.i, align 8
  %cond.i.i102.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %20, i64 1)
  %cmp26.i = icmp samesign uge i64 %18, %cond.i.i102.i
  %21 = extractelement <2 x i64> %17, i64 1
  %cmp28.i = icmp slt i64 %21, 0
  %or.cond52.i = select i1 %cmp26.i, i1 true, i1 %cmp28.i
  br i1 %or.cond52.i, label %if.then, label %lor.lhs.false29.i

lor.lhs.false29.i:                                ; preds = %lor.lhs.false24.i
  %arrayidx.i.i103.i = getelementptr i8, ptr addrspace(1) %f, i64 48
  %22 = load i64, ptr addrspace(1) %arrayidx.i.i103.i, align 8
  %cond.i.i104.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %22, i64 1)
  %cmp31.i.not = icmp samesign ult i64 %21, %cond.i.i104.i
  br i1 %cmp31.i.not, label %lor.lhs.false34.i, label %if.then

lor.lhs.false34.i:                                ; preds = %lor.lhs.false29.i
  %arrayidx.i.i105.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 24
  %23 = load i64, ptr addrspace(1) %arrayidx.i.i105.i, align 8
  %cond.i.i106.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %23, i64 1)
  %cmp36.not.i = icmp samesign ult i64 %l.addr.0123.i, %cond.i.i106.i
  br i1 %cmp36.not.i, label %sf_addr.exit, label %if.then

sf_addr.exit:                                     ; preds = %lor.lhs.false34.i
  %24 = load i64, ptr addrspace(1) %19, align 8
  %mul41.i = mul nuw nsw i64 %cond.i.i104.i, %l.addr.0123.i
  %add42.i = add nuw nsw i64 %mul41.i, %21
  %mul44.i = mul nuw nsw i64 %add42.i, %cond.i.i102.i
  %add45.i = add nuw nsw i64 %mul44.i, %18
  %mul49.i = mul i64 %add45.i, %mul20.pre-phi.i
  %add50.i = add nuw nsw i64 %mul49.i, %x.addr.0120.i
  %add51.i = add i64 %add50.i, %24
  %tobool.not = icmp eq i64 %add51.i, 0
  br i1 %tobool.not, label %if.then, label %if.end7

if.then:                                          ; preds = %if.end.i, %lor.lhs.false.i, %lor.lhs.false24.i, %lor.lhs.false29.i, %lor.lhs.false34.i, %sf_addr.exit
  %cmp.not = icmp eq i32 %mode, 2
  br i1 %cmp.not, label %cleanup, label %if.then6

if.then6:                                         ; preds = %if.then
  tail call void @__zluda_ptx_impl_agx_surface_trap()
  unreachable

if.end7:                                          ; preds = %sf_addr.exit
  call void @llvm.lifetime.start.p0(ptr nonnull %vs)
  store i64 %v0, ptr %vs, align 8
  %arrayinit.element = getelementptr inbounds nuw i8, ptr %vs, i64 8
  store i64 %v1, ptr %arrayinit.element, align 8
  %arrayinit.element8 = getelementptr inbounds nuw i8, ptr %vs, i64 16
  store i64 %v2, ptr %arrayinit.element8, align 8
  %arrayinit.element9 = getelementptr inbounds nuw i8, ptr %vs, i64 24
  store i64 %v3, ptr %arrayinit.element9, align 8
  %25 = inttoptr i64 %add51.i to ptr addrspace(1)
  %cmp20.i = icmp sgt i32 %n, 0
  %cmp218.i = icmp sgt i32 %b, 0
  %or.cond.i16 = and i1 %cmp20.i, %cmp218.i
  br i1 %or.cond.i16, label %for.cond1.preheader.preheader.i, label %store_bytes.exit

for.cond1.preheader.preheader.i:                  ; preds = %if.end7
  %26 = zext nneg i32 %b to i64
  %wide.trip.count27.i = zext nneg i32 %n to i64
  %min.iters.check = icmp ult i32 %b, 4
  %n.vec = and i64 %26, 2147483644
  %cmp.n = icmp eq i64 %n.vec, %26
  br label %for.cond1.preheader.i

for.cond1.preheader.i:                            ; preds = %for.cond1.for.cond.cleanup3_crit_edge.i, %for.cond1.preheader.preheader.i
  %indvars.iv24.i = phi i64 [ 0, %for.cond1.preheader.preheader.i ], [ %indvars.iv.next25.i, %for.cond1.for.cond.cleanup3_crit_edge.i ]
  %arrayidx.i = getelementptr inbounds nuw [8 x i8], ptr %vs, i64 %indvars.iv24.i
  %27 = mul nuw nsw i64 %indvars.iv24.i, %26
  %invariant.gep.i = getelementptr inbounds nuw i8, ptr addrspace(1) %25, i64 %27
  %28 = load i64, ptr %arrayidx.i, align 8
  br i1 %min.iters.check, label %for.body4.i.preheader, label %vector.ph

vector.ph:                                        ; preds = %for.cond1.preheader.i
  %broadcast.splatinsert = insertelement <4 x i64> poison, i64 %28, i64 0
  %broadcast.splat = shufflevector <4 x i64> %broadcast.splatinsert, <4 x i64> poison, <4 x i32> zeroinitializer
  br label %vector.body

vector.body:                                      ; preds = %vector.body, %vector.ph
  %index = phi i64 [ 0, %vector.ph ], [ %index.next, %vector.body ]
  %vec.ind = phi <4 x i64> [ <i64 0, i64 1, i64 2, i64 3>, %vector.ph ], [ %vec.ind.next, %vector.body ]
  %29 = shl nuw nsw <4 x i64> %vec.ind, splat (i64 3)
  %30 = lshr <4 x i64> %broadcast.splat, %29
  %31 = trunc <4 x i64> %30 to <4 x i8>
  %32 = getelementptr inbounds nuw i8, ptr addrspace(1) %invariant.gep.i, i64 %index
  store <4 x i8> %31, ptr addrspace(1) %32, align 1
  %index.next = add nuw i64 %index, 4
  %vec.ind.next = add nuw nsw <4 x i64> %vec.ind, splat (i64 4)
  %33 = icmp eq i64 %index.next, %n.vec
  br i1 %33, label %middle.block, label %vector.body, !llvm.loop !10032

middle.block:                                     ; preds = %vector.body
  br i1 %cmp.n, label %for.cond1.for.cond.cleanup3_crit_edge.i, label %for.body4.i.preheader

for.body4.i.preheader:                            ; preds = %for.cond1.preheader.i, %middle.block
  %indvars.iv.i.ph = phi i64 [ 0, %for.cond1.preheader.i ], [ %n.vec, %middle.block ]
  br label %for.body4.i

for.cond1.for.cond.cleanup3_crit_edge.i:          ; preds = %for.body4.i, %middle.block
  %indvars.iv.next25.i = add nuw nsw i64 %indvars.iv24.i, 1
  %exitcond28.not.i = icmp eq i64 %indvars.iv.next25.i, %wide.trip.count27.i
  br i1 %exitcond28.not.i, label %store_bytes.exit, label %for.cond1.preheader.i, !llvm.loop !10035

for.body4.i:                                      ; preds = %for.body4.i.preheader, %for.body4.i
  %indvars.iv.i = phi i64 [ %indvars.iv.next.i, %for.body4.i ], [ %indvars.iv.i.ph, %for.body4.i.preheader ]
  %34 = shl nuw nsw i64 %indvars.iv.i, 3
  %shr.i = lshr i64 %28, %34
  %conv.i17 = trunc i64 %shr.i to i8
  %gep.i = getelementptr inbounds nuw i8, ptr addrspace(1) %invariant.gep.i, i64 %indvars.iv.i
  store i8 %conv.i17, ptr addrspace(1) %gep.i, align 1
  %indvars.iv.next.i = add nuw nsw i64 %indvars.iv.i, 1
  %exitcond.not.i = icmp eq i64 %indvars.iv.next.i, %26
  br i1 %exitcond.not.i, label %for.cond1.for.cond.cleanup3_crit_edge.i, label %for.body4.i, !llvm.loop !10036

store_bytes.exit:                                 ; preds = %for.cond1.for.cond.cleanup3_crit_edge.i, %if.end7
  call void @llvm.lifetime.end.p0(ptr nonnull %vs)
  br label %cleanup

cleanup:                                          ; preds = %if.then, %store_bytes.exit
  ret void
}

; Function Attrs: alwaysinline convergent nounwind
define dso_local void @__zluda_ptx_impl_agx_sust_p(ptr addrspace(1) nofree noundef readonly captures(none) %f, i32 noundef %geom, i32 noundef %mode, i32 noundef %n, i32 noundef %c0, i32 noundef %c1, i32 noundef %c2, i32 noundef %c3, i32 noundef %v0, i32 noundef %v1, i32 noundef %v2, i32 noundef %v3) local_unnamed_addr #3 {
entry:
  switch i32 %geom, label %cond.false.i52.i [
    i32 6, label %if.then.i
    i32 4, label %if.then.i
    i32 3, label %if.then.i
    i32 2, label %geom_rank.exit53.thread64.i
    i32 0, label %sf_coords.exit
  ]

if.then.i:                                        ; preds = %entry, %entry, %entry
  %cond69.i = icmp eq i32 %geom, 3
  %narrow.i = select i1 %cond69.i, i32 0, i32 %c2
  %spec.select70.i = sext i32 %narrow.i to i64
  %conv11.i = zext i32 %c0 to i64
  %0 = insertelement <2 x i64> <i64 poison, i64 0>, i64 %spec.select70.i, i64 0
  br label %sf_coords.exit

geom_rank.exit53.thread64.i:                      ; preds = %entry
  %1 = insertelement <2 x i32> poison, i32 %c1, i64 0
  %2 = insertelement <2 x i32> %1, i32 %c2, i64 1
  %3 = sext <2 x i32> %2 to <2 x i64>
  br label %sf_coords.exit

cond.false.i52.i:                                 ; preds = %entry
  %4 = sext i32 %c1 to i64
  %5 = insertelement <2 x i64> <i64 poison, i64 0>, i64 %4, i64 0
  br label %sf_coords.exit

sf_coords.exit:                                   ; preds = %geom_rank.exit53.thread64.i, %entry, %cond.false.i52.i, %if.then.i
  %xs.sroa.0.0.in = phi i32 [ %c1, %if.then.i ], [ %c0, %cond.false.i52.i ], [ %c0, %entry ], [ %c0, %geom_rank.exit53.thread64.i ]
  %.sink.i = phi i64 [ %conv11.i, %if.then.i ], [ 0, %cond.false.i52.i ], [ 0, %entry ], [ 0, %geom_rank.exit53.thread64.i ]
  %6 = phi <2 x i64> [ %0, %if.then.i ], [ %5, %cond.false.i52.i ], [ zeroinitializer, %entry ], [ %3, %geom_rank.exit53.thread64.i ]
  %xs.sroa.0.0 = sext i32 %xs.sroa.0.0.in to i64
  %arrayidx.i.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 16
  %7 = load i64, ptr addrspace(1) %arrayidx.i.i, align 8
  %cond.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %7, i64 1)
  %mul = mul nsw i64 %cond.i.i, %xs.sroa.0.0
  %arrayidx.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 56
  %8 = load i64, ptr addrspace(1) %arrayidx.i, align 8
  %arrayidx.i80 = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 64
  %9 = load i64, ptr addrspace(1) %arrayidx.i80, align 8
  %cond.i81 = tail call noundef i64 @llvm.smax.i64(i64 %9, i64 1)
  %cmp.i = icmp sgt i64 %8, 3
  %cmp1.i = icmp samesign ult i64 %8, 9
  %conv = sext i32 %n to i64
  %cond.i83 = tail call noundef i64 @llvm.smin.i64(i64 %conv, i64 %cond.i81)
  %conv6 = trunc nsw i64 %cond.i83 to i32
  %10 = select i1 %cmp1.i, i64 1, i64 2
  %11 = select i1 %cmp.i, i64 %10, i64 0
  %mul13 = shl nsw i64 %cond.i83, %11
  %cmp.i84 = icmp eq i32 %mode, 1
  br i1 %cmp.i84, label %if.end.thread.i, label %if.end.i

if.end.thread.i:                                  ; preds = %sf_coords.exit
  %cond.i.i85 = tail call noundef i64 @llvm.smax.i64(i64 %mul, i64 0)
  %arrayidx.i.i.i = getelementptr i8, ptr addrspace(1) %f, i64 32
  %12 = load i64, ptr addrspace(1) %arrayidx.i.i.i, align 8
  %cond.i.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %12, i64 1)
  %mul.i = mul i64 %cond.i.i.i, %cond.i.i
  %sub.i = sub nsw i64 %mul.i, %mul13
  %cond.i83.i = tail call noundef i64 @llvm.smax.i64(i64 %sub.i, i64 0)
  %13 = tail call i64 @llvm.umin.i64(i64 %cond.i.i85, i64 %cond.i83.i)
  %14 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %6, <2 x i64> zeroinitializer)
  %arrayidx.i.i86.i = getelementptr i8, ptr addrspace(1) %f, i64 40
  %15 = load <2 x i64>, ptr addrspace(1) %arrayidx.i.i86.i, align 8
  %16 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %15, <2 x i64> splat (i64 1))
  %17 = add nsw <2 x i64> %16, splat (i64 -1)
  %18 = tail call <2 x i64> @llvm.umin.v2i64(<2 x i64> %14, <2 x i64> %17)
  %arrayidx.i.i94.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 24
  %19 = load i64, ptr addrspace(1) %arrayidx.i.i94.i, align 8
  %cond.i.i95.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %19, i64 1)
  %sub15.i = add nsw i64 %cond.i.i95.i, -1
  %20 = tail call i64 @llvm.umin.i64(i64 %.sink.i, i64 %sub15.i)
  br label %lor.lhs.false.i

if.end.i:                                         ; preds = %sf_coords.exit
  %cmp17.i = icmp slt i64 %mul, 0
  br i1 %cmp17.i, label %if.then, label %if.end.lor.lhs.false_crit_edge.i

if.end.lor.lhs.false_crit_edge.i:                 ; preds = %if.end.i
  %arrayidx.i.i97.phi.trans.insert.i = getelementptr i8, ptr addrspace(1) %f, i64 32
  %.pre.i = load i64, ptr addrspace(1) %arrayidx.i.i97.phi.trans.insert.i, align 8
  %.pre125.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %.pre.i, i64 1)
  %.pre127.i = mul i64 %.pre125.i, %cond.i.i
  br label %lor.lhs.false.i

lor.lhs.false.i:                                  ; preds = %if.end.lor.lhs.false_crit_edge.i, %if.end.thread.i
  %mul20.pre-phi.i = phi i64 [ %.pre127.i, %if.end.lor.lhs.false_crit_edge.i ], [ %mul.i, %if.end.thread.i ]
  %l.addr.0123.i = phi i64 [ %.sink.i, %if.end.lor.lhs.false_crit_edge.i ], [ %20, %if.end.thread.i ]
  %x.addr.0120.i = phi i64 [ %mul, %if.end.lor.lhs.false_crit_edge.i ], [ %13, %if.end.thread.i ]
  %21 = phi <2 x i64> [ %6, %if.end.lor.lhs.false_crit_edge.i ], [ %18, %if.end.thread.i ]
  %add.i = add nsw i64 %x.addr.0120.i, %mul13
  %cmp21.i = icmp sgt i64 %add.i, %mul20.pre-phi.i
  %22 = extractelement <2 x i64> %21, i64 0
  %cmp23.i = icmp slt i64 %22, 0
  %or.cond.i = select i1 %cmp21.i, i1 true, i1 %cmp23.i
  br i1 %or.cond.i, label %if.then, label %lor.lhs.false24.i

lor.lhs.false24.i:                                ; preds = %lor.lhs.false.i
  %23 = getelementptr i8, ptr addrspace(1) %f, i64 8
  %arrayidx.i.i101.i = getelementptr i8, ptr addrspace(1) %f, i64 40
  %24 = load i64, ptr addrspace(1) %arrayidx.i.i101.i, align 8
  %cond.i.i102.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %24, i64 1)
  %cmp26.i = icmp samesign uge i64 %22, %cond.i.i102.i
  %25 = extractelement <2 x i64> %21, i64 1
  %cmp28.i = icmp slt i64 %25, 0
  %or.cond52.i = select i1 %cmp26.i, i1 true, i1 %cmp28.i
  br i1 %or.cond52.i, label %if.then, label %lor.lhs.false29.i

lor.lhs.false29.i:                                ; preds = %lor.lhs.false24.i
  %arrayidx.i.i103.i = getelementptr i8, ptr addrspace(1) %f, i64 48
  %26 = load i64, ptr addrspace(1) %arrayidx.i.i103.i, align 8
  %cond.i.i104.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %26, i64 1)
  %cmp31.i.not = icmp samesign ult i64 %25, %cond.i.i104.i
  br i1 %cmp31.i.not, label %lor.lhs.false34.i, label %if.then

lor.lhs.false34.i:                                ; preds = %lor.lhs.false29.i
  %arrayidx.i.i105.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 24
  %27 = load i64, ptr addrspace(1) %arrayidx.i.i105.i, align 8
  %cond.i.i106.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %27, i64 1)
  %cmp36.not.i = icmp samesign ult i64 %l.addr.0123.i, %cond.i.i106.i
  br i1 %cmp36.not.i, label %sf_addr.exit, label %if.then

sf_addr.exit:                                     ; preds = %lor.lhs.false34.i
  %28 = load i64, ptr addrspace(1) %23, align 8
  %mul41.i = mul nuw nsw i64 %cond.i.i104.i, %l.addr.0123.i
  %add42.i = add nuw nsw i64 %mul41.i, %25
  %mul44.i = mul nuw nsw i64 %add42.i, %cond.i.i102.i
  %add45.i = add nuw nsw i64 %mul44.i, %22
  %mul49.i = mul i64 %add45.i, %mul20.pre-phi.i
  %add50.i = add nuw nsw i64 %mul49.i, %x.addr.0120.i
  %add51.i = add i64 %add50.i, %28
  %tobool.not = icmp eq i64 %add51.i, 0
  br i1 %tobool.not, label %if.then, label %if.end17

if.then:                                          ; preds = %if.end.i, %lor.lhs.false.i, %lor.lhs.false24.i, %lor.lhs.false29.i, %lor.lhs.false34.i, %sf_addr.exit
  %cmp.not = icmp eq i32 %mode, 2
  br i1 %cmp.not, label %cleanup55, label %if.then16

if.then16:                                        ; preds = %if.then
  tail call void @__zluda_ptx_impl_agx_surface_trap()
  unreachable

if.end17:                                         ; preds = %sf_addr.exit
  %29 = inttoptr i64 %add51.i to ptr addrspace(1)
  %invariant.smin = tail call i32 @llvm.smin.i32(i32 %conv6, i32 4)
  %or.cond97 = icmp sgt i32 %n, 0
  br i1 %or.cond97, label %if.end23.lr.ph, label %cleanup55

if.end23.lr.ph:                                   ; preds = %if.end17
  %cond.i.i86 = select i1 %cmp1.i, i32 16, i32 32
  %cond2.i.i = select i1 %cmp.i, i32 %cond.i.i86, i32 8
  %sub33.i = add nsw i32 %cond2.i.i, -1
  %sh_prom34.i = zext nneg i32 %sub33.i to i64
  %shl35.i = shl nuw nsw i64 1, %sh_prom34.i
  %sub36.i = sub nsw i64 0, %shl35.i
  %sub40.i = add nsw i64 %shl35.i, -1
  %sh_prom44.i = zext nneg i32 %cond2.i.i to i64
  %notmask.i = shl nsw i64 -1, %sh_prom44.i
  %sub46.i = xor i64 %notmask.i, -1
  %notmask70.i = shl nsw i64 -1, %sh_prom34.i
  %sub19.i = xor i64 %notmask70.i, -1
  %conv20.i = uitofp nneg i64 %sub19.i to double
  %conv.i92 = uitofp nneg i64 %sub46.i to double
  %30 = select i1 %cmp1.i, i32 1, i32 2
  %31 = select i1 %cmp.i, i32 %30, i32 0
  %32 = select i1 %cmp1.i, i64 2, i64 4
  %33 = select i1 %cmp.i, i64 %32, i64 1
  %cmp44.2 = icmp samesign ugt i64 %33, 2
  %cmp44.3 = icmp samesign ugt i64 %33, 3
  br label %if.end23

if.end23:                                         ; preds = %if.end23.lr.ph, %for.inc.3
  %i.098 = phi i32 [ 0, %if.end23.lr.ph ], [ %inc53, %for.inc.3 ]
  switch i32 %i.098, label %cond.false29 [
    i32 0, label %cond.end36
    i32 1, label %cond.true28
    i32 2, label %switch.edge
  ]

cond.true28:                                      ; preds = %if.end23
  br label %cond.end36

switch.edge:                                      ; preds = %if.end23
  br label %cond.end36

cond.false29:                                     ; preds = %if.end23
  br label %cond.end36

cond.end36:                                       ; preds = %switch.edge, %if.end23, %cond.true28, %cond.false29
  %cond37 = phi i32 [ %v3, %cond.false29 ], [ %v1, %cond.true28 ], [ %v0, %if.end23 ], [ %v2, %switch.edge ]
  %34 = bitcast i32 %cond37 to float
  %conv.i.i = fpext float %34 to double
  switch i64 %8, label %sw.default.i [
    i64 0, label %sw.bb.i
    i64 4, label %sw.bb.i
    i64 1, label %sw.bb6.i
    i64 5, label %sw.bb6.i
    i64 2, label %sw.bb26.i
    i64 6, label %sw.bb26.i
    i64 9, label %sw.bb26.i
    i64 3, label %sw.bb32.i
    i64 7, label %sw.bb32.i
    i64 10, label %sw.bb32.i
    i64 8, label %sw.bb48.i
  ]

sw.bb.i:                                          ; preds = %cond.end36, %cond.end36
  %cmp.i89 = fcmp uno float %34, 0.000000e+00
  %cmp.i73.i = fcmp oge float %34, 0.000000e+00
  %cond.i74.i = select i1 %cmp.i73.i, double %conv.i.i, double 0.000000e+00
  %cmp.i75.i = fcmp ole double %cond.i74.i, 1.000000e+00
  %cond.i76.i = select i1 %cmp.i75.i, double %cond.i74.i, double 1.000000e+00
  %cond.i90 = select i1 %cmp.i89, double 0.000000e+00, double %cond.i76.i
  %mul4.i = fmul double %cond.i90, %conv.i92
  %35 = tail call double @llvm.floor.f64(double %mul4.i)
  %sub.i.i = fsub double %mul4.i, %35
  %conv.i77.i = fptosi double %35 to i64
  %cmp.i78.i = fcmp ogt double %sub.i.i, 5.000000e-01
  br i1 %cmp.i78.i, label %cond.true.i.i, label %lor.lhs.false.i.i

lor.lhs.false.i.i:                                ; preds = %sw.bb.i
  %cmp2.i.i = fcmp une double %sub.i.i, 5.000000e-01
  %and.i.i = and i64 %conv.i77.i, 1
  %tobool.not.i.i = icmp eq i64 %and.i.i, 0
  %or.cond.i.i = select i1 %cmp2.i.i, i1 true, i1 %tobool.not.i.i
  br i1 %or.cond.i.i, label %for.inc, label %cond.true.i.i

cond.true.i.i:                                    ; preds = %lor.lhs.false.i.i, %sw.bb.i
  %add.i.i = add nsw i64 %conv.i77.i, 1
  br label %for.inc

sw.bb6.i:                                         ; preds = %cond.end36, %cond.end36
  %cmp8.i = fcmp uno float %34, 0.000000e+00
  %cmp.i80.i = fcmp oge double %conv.i.i, -1.000000e+00
  %cond.i81.i = select i1 %cmp.i80.i, double %conv.i.i, double -1.000000e+00
  %cmp.i82.i = fcmp ole double %cond.i81.i, 1.000000e+00
  %cond.i83.i88 = select i1 %cmp.i82.i, double %cond.i81.i, double 1.000000e+00
  %cond15.i = select i1 %cmp8.i, double 0.000000e+00, double %cond.i83.i88
  %mul21.i = fmul double %cond15.i, %conv20.i
  %36 = tail call double @llvm.floor.f64(double %mul21.i)
  %sub.i84.i = fsub double %mul21.i, %36
  %conv.i85.i = fptosi double %36 to i64
  %cmp.i86.i = fcmp ogt double %sub.i84.i, 5.000000e-01
  br i1 %cmp.i86.i, label %cond.true.i92.i, label %lor.lhs.false.i87.i

lor.lhs.false.i87.i:                              ; preds = %sw.bb6.i
  %cmp2.i88.i = fcmp une double %sub.i84.i, 5.000000e-01
  %and.i89.i = and i64 %conv.i85.i, 1
  %tobool.not.i90.i = icmp eq i64 %and.i89.i, 0
  %or.cond.i91.i = select i1 %cmp2.i88.i, i1 true, i1 %tobool.not.i90.i
  br i1 %or.cond.i91.i, label %rne.exit95.i, label %cond.true.i92.i

cond.true.i92.i:                                  ; preds = %lor.lhs.false.i87.i, %sw.bb6.i
  %add.i93.i = add nsw i64 %conv.i85.i, 1
  br label %rne.exit95.i

rne.exit95.i:                                     ; preds = %cond.true.i92.i, %lor.lhs.false.i87.i
  %cond.i94.i = phi i64 [ %add.i93.i, %cond.true.i92.i ], [ %conv.i85.i, %lor.lhs.false.i87.i ]
  %and.i = and i64 %cond.i94.i, %sub46.i
  br label %for.inc

sw.bb26.i:                                        ; preds = %cond.end36, %cond.end36, %cond.end36
  %conv27.i = zext i32 %cond37 to i64
  %37 = tail call i64 @llvm.umin.i64(i64 %conv27.i, i64 %sub46.i)
  br label %for.inc

sw.bb32.i:                                        ; preds = %cond.end36, %cond.end36, %cond.end36
  %conv41.i = sext i32 %cond37 to i64
  %cond.i97.i = tail call noundef i64 @llvm.smax.i64(i64 %conv41.i, i64 %sub36.i)
  %cond.i98.i = tail call noundef i64 @llvm.smin.i64(i64 %cond.i97.i, i64 %sub40.i)
  %and47.i = and i64 %cond.i98.i, %sub46.i
  br label %for.inc

sw.bb48.i:                                        ; preds = %cond.end36
  %shr.i.i = lshr i32 %cond37, 16
  %and.i100.i = and i32 %shr.i.i, 32768
  %shr1.i.i = lshr i32 %cond37, 23
  %and2.i.i = and i32 %shr1.i.i, 255
  %and3.i.i = and i32 %cond37, 8388607
  %cmp.i101.i = icmp eq i32 %and2.i.i, 255
  br i1 %cmp.i101.i, label %if.then.i.i, label %if.end.i.i

if.then.i.i:                                      ; preds = %sw.bb48.i
  %tobool.not.i105.i = icmp eq i32 %and3.i.i, 0
  %cond.i106.i = select i1 %tobool.not.i105.i, i32 0, i32 512
  %or.i.i = or disjoint i32 %and.i100.i, %cond.i106.i
  %or7.i.i = or disjoint i32 %or.i.i, 31744
  br label %for.inc.thread

if.end.i.i:                                       ; preds = %sw.bb48.i
  %cmp8.i.i = icmp samesign ugt i32 %and2.i.i, 142
  br i1 %cmp8.i.i, label %if.then10.i.i, label %if.end12.i.i

if.then10.i.i:                                    ; preds = %if.end.i.i
  %or11.i.i = or disjoint i32 %and.i100.i, 31744
  br label %for.inc.thread

if.end12.i.i:                                     ; preds = %if.end.i.i
  %cmp13.i.i = icmp samesign ult i32 %and2.i.i, 113
  br i1 %cmp13.i.i, label %if.then15.i.i, label %if.end36.i.i

if.then15.i.i:                                    ; preds = %if.end12.i.i
  %cmp16.i.i = icmp samesign ult i32 %and2.i.i, 102
  br i1 %cmp16.i.i, label %for.inc.thread, label %if.end19.i.i

if.end19.i.i:                                     ; preds = %if.then15.i.i
  %or20.i.i = or disjoint i32 %and3.i.i, 8388608
  %sub21.i.i = sub nuw nsw i32 126, %and2.i.i
  %shr22.i.i = lshr i32 %or20.i.i, %sub21.i.i
  %notmask.i.i = shl nsw i32 -1, %sub21.i.i
  %sub23.i.i = xor i32 %notmask.i.i, -1
  %and24.i.i = and i32 %or20.i.i, %sub23.i.i
  %sub25.i.i = sub nuw nsw i32 125, %and2.i.i
  %shl26.i.i = shl nuw nsw i32 1, %sub25.i.i
  %cmp27.i.i = icmp samesign ugt i32 %and24.i.i, %shl26.i.i
  br i1 %cmp27.i.i, label %if.then33.i.i, label %lor.lhs.false.i103.i

lor.lhs.false.i103.i:                             ; preds = %if.end19.i.i
  %cmp29.i.i = icmp ne i32 %and24.i.i, %shl26.i.i
  %and31.i.i = and i32 %shr22.i.i, 1
  %tobool32.not.i.i = icmp eq i32 %and31.i.i, 0
  %or.cond.i104.i = select i1 %cmp29.i.i, i1 true, i1 %tobool32.not.i.i
  br i1 %or.cond.i104.i, label %if.end34.i.i, label %if.then33.i.i

if.then33.i.i:                                    ; preds = %lor.lhs.false.i103.i, %if.end19.i.i
  %inc.i.i = add nuw nsw i32 %shr22.i.i, 1
  br label %if.end34.i.i

if.end34.i.i:                                     ; preds = %if.then33.i.i, %lor.lhs.false.i103.i
  %r.0.i.i = phi i32 [ %inc.i.i, %if.then33.i.i ], [ %shr22.i.i, %lor.lhs.false.i103.i ]
  %or35.i.i = or i32 %r.0.i.i, %and.i100.i
  br label %for.inc.thread

if.end36.i.i:                                     ; preds = %if.end12.i.i
  %add.i102.i = shl nuw nsw i32 %and2.i.i, 10
  %shr39.i.i = lshr i32 %and3.i.i, 13
  %and42.i.i = and i32 %cond37, 8191
  %cmp43.i.i = icmp samesign ugt i32 %and42.i.i, 4096
  %38 = and i32 %cond37, 16383
  %or.cond84.not.i.i = icmp eq i32 %38, 12288
  %or.cond85.i.i = or i1 %cmp43.i.i, %or.cond84.not.i.i
  %inc52.i.i = zext i1 %or.cond85.i.i to i32
  %shl38.i.i = or disjoint i32 %shr39.i.i, -114688
  %or40.i.i = add nsw i32 %shl38.i.i, %add.i102.i
  %r37.0.i.i = add nuw nsw i32 %or40.i.i, %inc52.i.i
  %or54.i.i = or i32 %r37.0.i.i, %and.i100.i
  br label %for.inc.thread

for.inc.thread:                                   ; preds = %if.then.i.i, %if.then10.i.i, %if.then15.i.i, %if.end34.i.i, %if.end36.i.i
  %retval.0.i.i = phi i32 [ %or7.i.i, %if.then.i.i ], [ %or11.i.i, %if.then10.i.i ], [ %or54.i.i, %if.end36.i.i ], [ %or35.i.i, %if.end34.i.i ], [ %and.i100.i, %if.then15.i.i ]
  %conv50.i = zext nneg i32 %retval.0.i.i to i64
  %mul49102 = shl nuw nsw i32 %i.098, %30
  %39 = zext nneg i32 %mul49102 to i64
  %conv48103 = trunc i32 %retval.0.i.i to i8
  %arrayidx50104 = getelementptr inbounds nuw i8, ptr addrspace(1) %29, i64 %39
  store i8 %conv48103, ptr addrspace(1) %arrayidx50104, align 1
  br label %if.then46.1

sw.default.i:                                     ; preds = %cond.end36
  %conv51.i = zext i32 %cond37 to i64
  br label %for.inc

for.inc:                                          ; preds = %lor.lhs.false.i.i, %cond.true.i.i, %rne.exit95.i, %sw.bb26.i, %sw.bb32.i, %sw.default.i
  %retval.0.i87 = phi i64 [ %conv51.i, %sw.default.i ], [ %conv.i77.i, %lor.lhs.false.i.i ], [ %and.i, %rne.exit95.i ], [ %37, %sw.bb26.i ], [ %and47.i, %sw.bb32.i ], [ %add.i.i, %cond.true.i.i ]
  %mul49 = shl nuw nsw i32 %i.098, %31
  %40 = zext nneg i32 %mul49 to i64
  %conv48 = trunc i64 %retval.0.i87 to i8
  %arrayidx50 = getelementptr inbounds nuw i8, ptr addrspace(1) %29, i64 %40
  store i8 %conv48, ptr addrspace(1) %arrayidx50, align 1
  br i1 %cmp.i, label %if.then46.1, label %for.inc.1

if.then46.1:                                      ; preds = %for.inc.thread, %for.inc
  %41 = phi i64 [ %39, %for.inc.thread ], [ %40, %for.inc ]
  %retval.0.i87105 = phi i64 [ %conv50.i, %for.inc.thread ], [ %retval.0.i87, %for.inc ]
  %shr.1 = lshr i64 %retval.0.i87105, 8
  %conv48.1 = trunc i64 %shr.1 to i8
  %42 = getelementptr inbounds nuw i8, ptr addrspace(1) %29, i64 %41
  %arrayidx50.1 = getelementptr inbounds nuw i8, ptr addrspace(1) %42, i64 1
  store i8 %conv48.1, ptr addrspace(1) %arrayidx50.1, align 1
  br label %for.inc.1

for.inc.1:                                        ; preds = %if.then46.1, %for.inc
  %43 = phi i64 [ %41, %if.then46.1 ], [ %40, %for.inc ]
  %retval.0.i87106 = phi i64 [ %retval.0.i87105, %if.then46.1 ], [ %retval.0.i87, %for.inc ]
  br i1 %cmp44.2, label %for.inc.2, label %for.inc.3

for.inc.2:                                        ; preds = %for.inc.1
  %shr.2 = lshr i64 %retval.0.i87106, 16
  %conv48.2 = trunc i64 %shr.2 to i8
  %44 = getelementptr inbounds nuw i8, ptr addrspace(1) %29, i64 %43
  %arrayidx50.2 = getelementptr inbounds nuw i8, ptr addrspace(1) %44, i64 2
  store i8 %conv48.2, ptr addrspace(1) %arrayidx50.2, align 1
  br i1 %cmp44.3, label %if.then46.3, label %for.inc.3

if.then46.3:                                      ; preds = %for.inc.2
  %shr.3 = lshr i64 %retval.0.i87106, 24
  %conv48.3 = trunc i64 %shr.3 to i8
  %45 = getelementptr inbounds nuw i8, ptr addrspace(1) %29, i64 %43
  %arrayidx50.3 = getelementptr inbounds nuw i8, ptr addrspace(1) %45, i64 3
  store i8 %conv48.3, ptr addrspace(1) %arrayidx50.3, align 1
  br label %for.inc.3

for.inc.3:                                        ; preds = %for.inc.1, %if.then46.3, %for.inc.2
  %inc53 = add nuw nsw i32 %i.098, 1
  %exitcond.not = icmp eq i32 %inc53, %invariant.smin
  br i1 %exitcond.not, label %cleanup55, label %if.end23, !llvm.loop !10037

cleanup55:                                        ; preds = %for.inc.3, %if.end17, %if.then
  ret void
}

; Function Attrs: alwaysinline convergent nounwind
define dso_local i64 @__zluda_ptx_impl_agx_sured_addr(ptr addrspace(1) nofree noundef readonly captures(none) %f, i32 noundef %geom, i32 noundef %mode, i32 noundef %formatted, i32 noundef %b, i32 noundef %c0, i32 noundef %c1, i32 noundef %c2, i32 noundef %c3) local_unnamed_addr #3 {
entry:
  switch i32 %geom, label %cond.false.i52.i [
    i32 6, label %if.then.i
    i32 4, label %if.then.i
    i32 3, label %if.then.i
    i32 2, label %geom_rank.exit53.thread64.i
    i32 0, label %sf_coords.exit
  ]

if.then.i:                                        ; preds = %entry, %entry, %entry
  %cond69.i = icmp eq i32 %geom, 3
  %narrow.i = select i1 %cond69.i, i32 0, i32 %c2
  %spec.select70.i = sext i32 %narrow.i to i64
  %conv11.i = zext i32 %c0 to i64
  %0 = insertelement <2 x i64> <i64 poison, i64 0>, i64 %spec.select70.i, i64 0
  br label %sf_coords.exit

geom_rank.exit53.thread64.i:                      ; preds = %entry
  %1 = insertelement <2 x i32> poison, i32 %c1, i64 0
  %2 = insertelement <2 x i32> %1, i32 %c2, i64 1
  %3 = sext <2 x i32> %2 to <2 x i64>
  br label %sf_coords.exit

cond.false.i52.i:                                 ; preds = %entry
  %4 = sext i32 %c1 to i64
  %5 = insertelement <2 x i64> <i64 poison, i64 0>, i64 %4, i64 0
  br label %sf_coords.exit

sf_coords.exit:                                   ; preds = %geom_rank.exit53.thread64.i, %entry, %cond.false.i52.i, %if.then.i
  %xs.sroa.0.1.in = phi i32 [ %c1, %if.then.i ], [ %c0, %cond.false.i52.i ], [ %c0, %entry ], [ %c0, %geom_rank.exit53.thread64.i ]
  %.sink.i = phi i64 [ %conv11.i, %if.then.i ], [ 0, %cond.false.i52.i ], [ 0, %entry ], [ 0, %geom_rank.exit53.thread64.i ]
  %6 = phi <2 x i64> [ %0, %if.then.i ], [ %5, %cond.false.i52.i ], [ zeroinitializer, %entry ], [ %3, %geom_rank.exit53.thread64.i ]
  %xs.sroa.0.1 = sext i32 %xs.sroa.0.1.in to i64
  %tobool.not = icmp eq i32 %formatted, 0
  %narrow = select i1 %tobool.not, i32 1, i32 %b
  %mul = sext i32 %narrow to i64
  %xs.sroa.0.0 = mul nsw i64 %xs.sroa.0.1, %mul
  %conv5 = sext i32 %b to i64
  %cmp.i = icmp eq i32 %mode, 1
  br i1 %cmp.i, label %if.end.thread.i, label %if.end.i

if.end.thread.i:                                  ; preds = %sf_coords.exit
  %cond.i.i = tail call noundef i64 @llvm.smax.i64(i64 %xs.sroa.0.0, i64 0)
  %arrayidx.i.i.i = getelementptr i8, ptr addrspace(1) %f, i64 32
  %7 = load i64, ptr addrspace(1) %arrayidx.i.i.i, align 8
  %cond.i.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %7, i64 1)
  %arrayidx.i.i81.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 16
  %8 = load i64, ptr addrspace(1) %arrayidx.i.i81.i, align 8
  %cond.i.i82.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %8, i64 1)
  %mul.i = mul i64 %cond.i.i82.i, %cond.i.i.i
  %sub.i = sub nsw i64 %mul.i, %conv5
  %cond.i83.i = tail call noundef i64 @llvm.smax.i64(i64 %sub.i, i64 0)
  %9 = tail call i64 @llvm.umin.i64(i64 %cond.i.i, i64 %cond.i83.i)
  %10 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %6, <2 x i64> zeroinitializer)
  %arrayidx.i.i86.i = getelementptr i8, ptr addrspace(1) %f, i64 40
  %11 = load <2 x i64>, ptr addrspace(1) %arrayidx.i.i86.i, align 8
  %12 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %11, <2 x i64> splat (i64 1))
  %13 = add nsw <2 x i64> %12, splat (i64 -1)
  %14 = tail call <2 x i64> @llvm.umin.v2i64(<2 x i64> %10, <2 x i64> %13)
  %arrayidx.i.i94.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 24
  %15 = load i64, ptr addrspace(1) %arrayidx.i.i94.i, align 8
  %cond.i.i95.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %15, i64 1)
  %sub15.i = add nsw i64 %cond.i.i95.i, -1
  %16 = tail call i64 @llvm.umin.i64(i64 %.sink.i, i64 %sub15.i)
  br label %lor.lhs.false.i

if.end.i:                                         ; preds = %sf_coords.exit
  %cmp17.i = icmp slt i64 %xs.sroa.0.0, 0
  br i1 %cmp17.i, label %sf_addr.exit, label %if.end.lor.lhs.false_crit_edge.i

if.end.lor.lhs.false_crit_edge.i:                 ; preds = %if.end.i
  %arrayidx.i.i97.phi.trans.insert.i = getelementptr i8, ptr addrspace(1) %f, i64 32
  %.pre.i = load i64, ptr addrspace(1) %arrayidx.i.i97.phi.trans.insert.i, align 8
  %arrayidx.i.i99.phi.trans.insert.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 16
  %.pre124.i = load i64, ptr addrspace(1) %arrayidx.i.i99.phi.trans.insert.i, align 8
  %.pre125.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %.pre.i, i64 1)
  %.pre126.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %.pre124.i, i64 1)
  %.pre127.i = mul i64 %.pre126.i, %.pre125.i
  br label %lor.lhs.false.i

lor.lhs.false.i:                                  ; preds = %if.end.lor.lhs.false_crit_edge.i, %if.end.thread.i
  %mul20.pre-phi.i = phi i64 [ %.pre127.i, %if.end.lor.lhs.false_crit_edge.i ], [ %mul.i, %if.end.thread.i ]
  %l.addr.0123.i = phi i64 [ %.sink.i, %if.end.lor.lhs.false_crit_edge.i ], [ %16, %if.end.thread.i ]
  %x.addr.0120.i = phi i64 [ %xs.sroa.0.0, %if.end.lor.lhs.false_crit_edge.i ], [ %9, %if.end.thread.i ]
  %17 = phi <2 x i64> [ %6, %if.end.lor.lhs.false_crit_edge.i ], [ %14, %if.end.thread.i ]
  %add.i = add nsw i64 %x.addr.0120.i, %conv5
  %cmp21.i = icmp sgt i64 %add.i, %mul20.pre-phi.i
  %18 = extractelement <2 x i64> %17, i64 0
  %cmp23.i = icmp slt i64 %18, 0
  %or.cond.i = select i1 %cmp21.i, i1 true, i1 %cmp23.i
  br i1 %or.cond.i, label %sf_addr.exit, label %lor.lhs.false24.i

lor.lhs.false24.i:                                ; preds = %lor.lhs.false.i
  %19 = getelementptr i8, ptr addrspace(1) %f, i64 8
  %arrayidx.i.i101.i = getelementptr i8, ptr addrspace(1) %f, i64 40
  %20 = load i64, ptr addrspace(1) %arrayidx.i.i101.i, align 8
  %cond.i.i102.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %20, i64 1)
  %cmp26.i = icmp samesign uge i64 %18, %cond.i.i102.i
  %21 = extractelement <2 x i64> %17, i64 1
  %cmp28.i = icmp slt i64 %21, 0
  %or.cond52.i = select i1 %cmp26.i, i1 true, i1 %cmp28.i
  br i1 %or.cond52.i, label %sf_addr.exit, label %lor.lhs.false29.i

lor.lhs.false29.i:                                ; preds = %lor.lhs.false24.i
  %arrayidx.i.i103.i = getelementptr i8, ptr addrspace(1) %f, i64 48
  %22 = load i64, ptr addrspace(1) %arrayidx.i.i103.i, align 8
  %cond.i.i104.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %22, i64 1)
  %cmp31.i.not = icmp samesign ult i64 %21, %cond.i.i104.i
  br i1 %cmp31.i.not, label %lor.lhs.false34.i, label %sf_addr.exit

lor.lhs.false34.i:                                ; preds = %lor.lhs.false29.i
  %arrayidx.i.i105.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 24
  %23 = load i64, ptr addrspace(1) %arrayidx.i.i105.i, align 8
  %cond.i.i106.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %23, i64 1)
  %cmp36.not.i = icmp samesign ult i64 %l.addr.0123.i, %cond.i.i106.i
  br i1 %cmp36.not.i, label %if.end38.i, label %sf_addr.exit

if.end38.i:                                       ; preds = %lor.lhs.false34.i
  %24 = load i64, ptr addrspace(1) %19, align 8
  %mul41.i = mul nuw nsw i64 %cond.i.i104.i, %l.addr.0123.i
  %add42.i = add nuw nsw i64 %mul41.i, %21
  %mul44.i = mul nuw nsw i64 %add42.i, %cond.i.i102.i
  %add45.i = add nuw nsw i64 %mul44.i, %18
  %mul49.i = mul i64 %add45.i, %mul20.pre-phi.i
  %add50.i = add nuw nsw i64 %mul49.i, %x.addr.0120.i
  %add51.i = add i64 %add50.i, %24
  br label %sf_addr.exit

sf_addr.exit:                                     ; preds = %if.end.i, %lor.lhs.false.i, %lor.lhs.false24.i, %lor.lhs.false29.i, %lor.lhs.false34.i, %if.end38.i
  %retval.0.i = phi i64 [ %add51.i, %if.end38.i ], [ 0, %lor.lhs.false34.i ], [ 0, %lor.lhs.false29.i ], [ 0, %lor.lhs.false24.i ], [ 0, %lor.lhs.false.i ], [ 0, %if.end.i ]
  %tobool6 = icmp eq i64 %retval.0.i, 0
  %cmp = icmp ne i32 %mode, 2
  %or.cond = and i1 %cmp, %tobool6
  br i1 %or.cond, label %if.then8, label %if.end9

if.then8:                                         ; preds = %sf_addr.exit
  tail call void @__zluda_ptx_impl_agx_surface_trap()
  unreachable

if.end9:                                          ; preds = %sf_addr.exit
  ret i64 %retval.0.i
}

; Function Attrs: alwaysinline convergent nounwind
define dso_local void @__zluda_ptx_impl_agx_sured(ptr addrspace(1) nofree noundef readonly captures(none) %f, i32 noundef %geom, i32 noundef %mode, i32 noundef %formatted, i32 noundef %b, i32 noundef %c0, i32 noundef %c1, i32 noundef %c2, i32 noundef %c3, i32 noundef %op, i32 noundef %is_signed, i64 noundef %v) local_unnamed_addr #3 {
entry:
  switch i32 %geom, label %cond.false.i52.i [
    i32 6, label %if.then.i87
    i32 4, label %if.then.i87
    i32 3, label %if.then.i87
    i32 2, label %geom_rank.exit53.thread64.i
    i32 0, label %sf_coords.exit
  ]

if.then.i87:                                      ; preds = %entry, %entry, %entry
  %cond69.i = icmp eq i32 %geom, 3
  %narrow.i = select i1 %cond69.i, i32 0, i32 %c2
  %spec.select70.i = sext i32 %narrow.i to i64
  %conv11.i = zext i32 %c0 to i64
  %0 = insertelement <2 x i64> <i64 poison, i64 0>, i64 %spec.select70.i, i64 0
  br label %sf_coords.exit

geom_rank.exit53.thread64.i:                      ; preds = %entry
  %1 = insertelement <2 x i32> poison, i32 %c1, i64 0
  %2 = insertelement <2 x i32> %1, i32 %c2, i64 1
  %3 = sext <2 x i32> %2 to <2 x i64>
  br label %sf_coords.exit

cond.false.i52.i:                                 ; preds = %entry
  %4 = sext i32 %c1 to i64
  %5 = insertelement <2 x i64> <i64 poison, i64 0>, i64 %4, i64 0
  br label %sf_coords.exit

sf_coords.exit:                                   ; preds = %geom_rank.exit53.thread64.i, %entry, %cond.false.i52.i, %if.then.i87
  %xs.i.sroa.0.1.in = phi i32 [ %c1, %if.then.i87 ], [ %c0, %cond.false.i52.i ], [ %c0, %entry ], [ %c0, %geom_rank.exit53.thread64.i ]
  %.sink.i = phi i64 [ %conv11.i, %if.then.i87 ], [ 0, %cond.false.i52.i ], [ 0, %entry ], [ 0, %geom_rank.exit53.thread64.i ]
  %6 = phi <2 x i64> [ %0, %if.then.i87 ], [ %5, %cond.false.i52.i ], [ zeroinitializer, %entry ], [ %3, %geom_rank.exit53.thread64.i ]
  %xs.i.sroa.0.1 = sext i32 %xs.i.sroa.0.1.in to i64
  %tobool.not.i = icmp eq i32 %formatted, 0
  %narrow = select i1 %tobool.not.i, i32 1, i32 %b
  %mul.i = sext i32 %narrow to i64
  %xs.i.sroa.0.0 = mul nsw i64 %xs.i.sroa.0.1, %mul.i
  %conv5.i = sext i32 %b to i64
  %cmp.i90 = icmp eq i32 %mode, 1
  br i1 %cmp.i90, label %if.end.thread.i, label %if.end.i91

if.end.thread.i:                                  ; preds = %sf_coords.exit
  %cond.i.i = tail call noundef i64 @llvm.smax.i64(i64 %xs.i.sroa.0.0, i64 0)
  %arrayidx.i.i.i = getelementptr i8, ptr addrspace(1) %f, i64 32
  %7 = load i64, ptr addrspace(1) %arrayidx.i.i.i, align 8
  %cond.i.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %7, i64 1)
  %arrayidx.i.i81.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 16
  %8 = load i64, ptr addrspace(1) %arrayidx.i.i81.i, align 8
  %cond.i.i82.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %8, i64 1)
  %mul.i93 = mul i64 %cond.i.i82.i, %cond.i.i.i
  %sub.i = sub nsw i64 %mul.i93, %conv5.i
  %cond.i83.i = tail call noundef i64 @llvm.smax.i64(i64 %sub.i, i64 0)
  %9 = tail call i64 @llvm.umin.i64(i64 %cond.i.i, i64 %cond.i83.i)
  %10 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %6, <2 x i64> zeroinitializer)
  %arrayidx.i.i86.i = getelementptr i8, ptr addrspace(1) %f, i64 40
  %11 = load <2 x i64>, ptr addrspace(1) %arrayidx.i.i86.i, align 8
  %12 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %11, <2 x i64> splat (i64 1))
  %13 = add nsw <2 x i64> %12, splat (i64 -1)
  %14 = tail call <2 x i64> @llvm.umin.v2i64(<2 x i64> %10, <2 x i64> %13)
  %arrayidx.i.i94.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 24
  %15 = load i64, ptr addrspace(1) %arrayidx.i.i94.i, align 8
  %cond.i.i95.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %15, i64 1)
  %sub15.i = add nsw i64 %cond.i.i95.i, -1
  %16 = tail call i64 @llvm.umin.i64(i64 %.sink.i, i64 %sub15.i)
  br label %lor.lhs.false.i

if.end.i91:                                       ; preds = %sf_coords.exit
  %cmp17.i = icmp slt i64 %xs.i.sroa.0.0, 0
  br i1 %cmp17.i, label %sf_addr.exit, label %if.end.lor.lhs.false_crit_edge.i

if.end.lor.lhs.false_crit_edge.i:                 ; preds = %if.end.i91
  %arrayidx.i.i97.phi.trans.insert.i = getelementptr i8, ptr addrspace(1) %f, i64 32
  %.pre.i = load i64, ptr addrspace(1) %arrayidx.i.i97.phi.trans.insert.i, align 8
  %arrayidx.i.i99.phi.trans.insert.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 16
  %.pre124.i = load i64, ptr addrspace(1) %arrayidx.i.i99.phi.trans.insert.i, align 8
  %.pre125.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %.pre.i, i64 1)
  %.pre126.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %.pre124.i, i64 1)
  %.pre127.i = mul i64 %.pre126.i, %.pre125.i
  br label %lor.lhs.false.i

lor.lhs.false.i:                                  ; preds = %if.end.lor.lhs.false_crit_edge.i, %if.end.thread.i
  %mul20.pre-phi.i = phi i64 [ %.pre127.i, %if.end.lor.lhs.false_crit_edge.i ], [ %mul.i93, %if.end.thread.i ]
  %l.addr.0123.i = phi i64 [ %.sink.i, %if.end.lor.lhs.false_crit_edge.i ], [ %16, %if.end.thread.i ]
  %x.addr.0120.i = phi i64 [ %xs.i.sroa.0.0, %if.end.lor.lhs.false_crit_edge.i ], [ %9, %if.end.thread.i ]
  %17 = phi <2 x i64> [ %6, %if.end.lor.lhs.false_crit_edge.i ], [ %14, %if.end.thread.i ]
  %add.i = add nsw i64 %x.addr.0120.i, %conv5.i
  %cmp21.i = icmp sgt i64 %add.i, %mul20.pre-phi.i
  %18 = extractelement <2 x i64> %17, i64 0
  %cmp23.i = icmp slt i64 %18, 0
  %or.cond.i92 = select i1 %cmp21.i, i1 true, i1 %cmp23.i
  br i1 %or.cond.i92, label %sf_addr.exit, label %lor.lhs.false24.i

lor.lhs.false24.i:                                ; preds = %lor.lhs.false.i
  %19 = getelementptr i8, ptr addrspace(1) %f, i64 8
  %arrayidx.i.i101.i = getelementptr i8, ptr addrspace(1) %f, i64 40
  %20 = load i64, ptr addrspace(1) %arrayidx.i.i101.i, align 8
  %cond.i.i102.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %20, i64 1)
  %cmp26.i = icmp samesign uge i64 %18, %cond.i.i102.i
  %21 = extractelement <2 x i64> %17, i64 1
  %cmp28.i = icmp slt i64 %21, 0
  %or.cond52.i = select i1 %cmp26.i, i1 true, i1 %cmp28.i
  br i1 %or.cond52.i, label %sf_addr.exit, label %lor.lhs.false29.i

lor.lhs.false29.i:                                ; preds = %lor.lhs.false24.i
  %arrayidx.i.i103.i = getelementptr i8, ptr addrspace(1) %f, i64 48
  %22 = load i64, ptr addrspace(1) %arrayidx.i.i103.i, align 8
  %cond.i.i104.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %22, i64 1)
  %cmp31.i.not = icmp samesign ult i64 %21, %cond.i.i104.i
  br i1 %cmp31.i.not, label %lor.lhs.false34.i, label %sf_addr.exit

lor.lhs.false34.i:                                ; preds = %lor.lhs.false29.i
  %arrayidx.i.i105.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 24
  %23 = load i64, ptr addrspace(1) %arrayidx.i.i105.i, align 8
  %cond.i.i106.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %23, i64 1)
  %cmp36.not.i = icmp samesign ult i64 %l.addr.0123.i, %cond.i.i106.i
  br i1 %cmp36.not.i, label %if.end38.i, label %sf_addr.exit

if.end38.i:                                       ; preds = %lor.lhs.false34.i
  %24 = load i64, ptr addrspace(1) %19, align 8
  %mul41.i = mul nuw nsw i64 %cond.i.i104.i, %l.addr.0123.i
  %add42.i = add nuw nsw i64 %mul41.i, %21
  %mul44.i = mul nuw nsw i64 %add42.i, %cond.i.i102.i
  %add45.i = add nuw nsw i64 %mul44.i, %18
  %mul49.i = mul i64 %add45.i, %mul20.pre-phi.i
  %add50.i = add nuw nsw i64 %mul49.i, %x.addr.0120.i
  %add51.i = add i64 %add50.i, %24
  br label %sf_addr.exit

sf_addr.exit:                                     ; preds = %if.end.i91, %lor.lhs.false.i, %lor.lhs.false24.i, %lor.lhs.false29.i, %lor.lhs.false34.i, %if.end38.i
  %retval.0.i = phi i64 [ %add51.i, %if.end38.i ], [ 0, %lor.lhs.false34.i ], [ 0, %lor.lhs.false29.i ], [ 0, %lor.lhs.false24.i ], [ 0, %lor.lhs.false.i ], [ 0, %if.end.i91 ]
  %tobool6.i = icmp eq i64 %retval.0.i, 0
  %cmp.i = icmp ne i32 %mode, 2
  %or.cond.i = and i1 %cmp.i, %tobool6.i
  br i1 %or.cond.i, label %if.then8.i, label %__zluda_ptx_impl_agx_sured_addr.exit

if.then8.i:                                       ; preds = %sf_addr.exit
  tail call void @__zluda_ptx_impl_agx_surface_trap()
  unreachable

__zluda_ptx_impl_agx_sured_addr.exit:             ; preds = %sf_addr.exit
  br i1 %tobool6.i, label %cleanup, label %if.end

if.end:                                           ; preds = %__zluda_ptx_impl_agx_sured_addr.exit
  %cmp = icmp eq i32 %b, 8
  %25 = inttoptr i64 %retval.0.i to ptr addrspace(1)
  br i1 %cmp, label %if.then1, label %if.end24

if.then1:                                         ; preds = %if.end
  switch i32 %op, label %sw.default [
    i32 0, label %sw.bb
    i32 1, label %sw.bb2
    i32 2, label %sw.bb5
    i32 3, label %sw.bb8
  ]

sw.bb:                                            ; preds = %if.then1
  %26 = atomicrmw add ptr addrspace(1) %25, i64 %v monotonic, align 8
  br label %cleanup

sw.bb2:                                           ; preds = %if.then1
  %27 = atomicrmw and ptr addrspace(1) %25, i64 %v monotonic, align 8
  br label %cleanup

sw.bb5:                                           ; preds = %if.then1
  %28 = atomicrmw or ptr addrspace(1) %25, i64 %v monotonic, align 8
  br label %cleanup

sw.bb8:                                           ; preds = %if.then1
  %tobool9.not = icmp eq i32 %is_signed, 0
  br i1 %tobool9.not, label %if.else, label %if.then10

if.then10:                                        ; preds = %sw.bb8
  %29 = atomicrmw min ptr addrspace(1) %25, i64 %v monotonic, align 8
  br label %cleanup

if.else:                                          ; preds = %sw.bb8
  %30 = atomicrmw umin ptr addrspace(1) %25, i64 %v monotonic, align 8
  br label %cleanup

sw.default:                                       ; preds = %if.then1
  %tobool16.not = icmp eq i32 %is_signed, 0
  br i1 %tobool16.not, label %if.else20, label %if.then17

if.then17:                                        ; preds = %sw.default
  %31 = atomicrmw max ptr addrspace(1) %25, i64 %v monotonic, align 8
  br label %cleanup

if.else20:                                        ; preds = %sw.default
  %32 = atomicrmw umax ptr addrspace(1) %25, i64 %v monotonic, align 8
  br label %cleanup

if.end24:                                         ; preds = %if.end
  %conv = trunc i64 %v to i32
  switch i32 %op, label %sw.default45 [
    i32 0, label %sw.bb27
    i32 1, label %sw.bb30
    i32 2, label %sw.bb33
    i32 3, label %sw.bb36
  ]

sw.bb27:                                          ; preds = %if.end24
  %33 = atomicrmw add ptr addrspace(1) %25, i32 %conv monotonic, align 4
  br label %cleanup

sw.bb30:                                          ; preds = %if.end24
  %34 = atomicrmw and ptr addrspace(1) %25, i32 %conv monotonic, align 4
  br label %cleanup

sw.bb33:                                          ; preds = %if.end24
  %35 = atomicrmw or ptr addrspace(1) %25, i32 %conv monotonic, align 4
  br label %cleanup

sw.bb36:                                          ; preds = %if.end24
  %tobool37.not = icmp eq i32 %is_signed, 0
  br i1 %tobool37.not, label %if.else41, label %if.then38

if.then38:                                        ; preds = %sw.bb36
  %36 = atomicrmw min ptr addrspace(1) %25, i32 %conv monotonic, align 4
  br label %cleanup

if.else41:                                        ; preds = %sw.bb36
  %37 = atomicrmw umin ptr addrspace(1) %25, i32 %conv monotonic, align 4
  br label %cleanup

sw.default45:                                     ; preds = %if.end24
  %tobool46.not = icmp eq i32 %is_signed, 0
  br i1 %tobool46.not, label %if.else50, label %if.then47

if.then47:                                        ; preds = %sw.default45
  %38 = atomicrmw max ptr addrspace(1) %25, i32 %conv monotonic, align 4
  br label %cleanup

if.else50:                                        ; preds = %sw.default45
  %39 = atomicrmw umax ptr addrspace(1) %25, i32 %conv monotonic, align 4
  br label %cleanup

cleanup:                                          ; preds = %sw.bb27, %sw.bb30, %sw.bb33, %if.else41, %if.then38, %if.else50, %if.then47, %sw.bb, %sw.bb2, %sw.bb5, %if.else, %if.then10, %if.else20, %if.then17, %__zluda_ptx_impl_agx_sured_addr.exit
  ret void
}

; Function Attrs: alwaysinline mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read)
define dso_local range(i32 0, 2) i32 @__zluda_ptx_impl_agx_surface_signed(ptr addrspace(1) nofree noundef readonly captures(none) %f) local_unnamed_addr #3 {
entry:
  %arrayidx.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 56
  %0 = load i64, ptr addrspace(1) %arrayidx.i, align 8
  %1 = and i64 %0, -5
  %or.cond.i = icmp eq i64 %1, 3
  %cmp2.i = icmp eq i64 %0, 10
  %narrow.i = or i1 %cmp2.i, %or.cond.i
  %lor.ext.i = zext i1 %narrow.i to i32
  ret i32 %lor.ext.i
}

; Function Attrs: alwaysinline mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read)
define dso_local i32 @__zluda_ptx_impl_agx_suq(ptr addrspace(1) nofree noundef readonly captures(none) %f, i32 noundef %field) local_unnamed_addr #3 {
entry:
  switch i32 %field, label %return [
    i32 0, label %sw.bb
    i32 1, label %sw.bb
    i32 2, label %sw.bb
    i32 3, label %sw.bb1
    i32 4, label %sw.bb5
    i32 5, label %sw.bb8
    i32 6, label %sw.bb11
  ]

sw.bb:                                            ; preds = %entry, %entry, %entry
  %0 = zext nneg i32 %field to i64
  %1 = getelementptr [8 x i8], ptr addrspace(1) %f, i64 %0
  %arrayidx.i.i = getelementptr i8, ptr addrspace(1) %1, i64 32
  %2 = load i64, ptr addrspace(1) %arrayidx.i.i, align 8
  %cond.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %2, i64 1)
  %conv = trunc i64 %cond.i.i to i32
  br label %return

sw.bb1:                                           ; preds = %entry
  %arrayidx.i = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 96
  %3 = load i64, ptr addrspace(1) %arrayidx.i, align 8
  %tobool.not = icmp eq i64 %3, 0
  br i1 %tobool.not, label %return, label %cond.true

cond.true:                                        ; preds = %sw.bb1
  %arrayidx.i20 = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 24
  %4 = load i64, ptr addrspace(1) %arrayidx.i20, align 8
  %5 = trunc i64 %4 to i32
  br label %return

sw.bb5:                                           ; preds = %entry
  %arrayidx.i21 = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 72
  %6 = load i64, ptr addrspace(1) %arrayidx.i21, align 8
  %conv7 = trunc i64 %6 to i32
  br label %return

sw.bb8:                                           ; preds = %entry
  %arrayidx.i22 = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 80
  %7 = load i64, ptr addrspace(1) %arrayidx.i22, align 8
  %conv10 = trunc i64 %7 to i32
  br label %return

sw.bb11:                                          ; preds = %entry
  %arrayidx.i23 = getelementptr inbounds nuw i8, ptr addrspace(1) %f, i64 88
  %8 = load i64, ptr addrspace(1) %arrayidx.i23, align 8
  %conv13 = trunc i64 %8 to i32
  br label %return

return:                                           ; preds = %entry, %cond.true, %sw.bb1, %sw.bb11, %sw.bb8, %sw.bb5, %sw.bb
  %retval.0 = phi i32 [ 0, %sw.bb1 ], [ %conv, %sw.bb ], [ %conv13, %sw.bb11 ], [ %conv7, %sw.bb5 ], [ %conv10, %sw.bb8 ], [ %5, %cond.true ], [ 0, %entry ]
  ret i32 %retval.0
}

; Function Attrs: alwaysinline mustprogress nofree norecurse nosync nounwind willreturn memory(read, inaccessiblemem: none, target_mem: none)
define dso_local range(i32 0, 2) i32 @__zluda_ptx_impl_agx_istypep(i64 noundef %h, i32 noundef %kind) local_unnamed_addr #3 {
entry:
  %cmp = icmp ugt i64 %h, 4095
  %and = and i64 %h, 7
  %tobool.not = icmp eq i64 %and, 0
  %or.cond = and i1 %cmp, %tobool.not
  br i1 %or.cond, label %if.end, label %return

if.end:                                           ; preds = %entry
  %cmp1 = icmp eq i32 %kind, 0
  %cmp2 = icmp eq i32 %kind, 1
  %cond = select i1 %cmp2, i64 1396920397, i64 1398035526
  %cond3 = select i1 %cmp1, i64 1415075924, i64 %cond
  %0 = inttoptr i64 %h to ptr addrspace(1)
  %1 = load i64, ptr addrspace(1) %0, align 8
  %cmp4 = icmp eq i64 %1, %cond3
  %conv5 = zext i1 %cmp4 to i32
  br label %return

return:                                           ; preds = %entry, %if.end
  %retval.0 = phi i32 [ %conv5, %if.end ], [ 0, %entry ]
  ret i32 %retval.0
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define dso_local <4 x float> @__zluda_ptx_impl_texobj_1d_v4_f32_f32(i64 noundef %texobj, <1 x float> noundef %coord) local_unnamed_addr #3 {
if.else167.i:
  %q.i = alloca %struct.q, align 4
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %arrayidx.i44.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 32
  %1 = load i64, ptr addrspace(1) %arrayidx.i44.i, align 8
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %2 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %3 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %3 to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q.i)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q.i, i8 0, i64 24, i1 false)
  %retval.sroa.3.0.i = trunc i64 %2 to i32
  %retval.sroa.0.0.i = trunc i64 %1 to i32
  %4 = extractelement <1 x float> %coord, i64 0
  %conv.i.i = fpext float %4 to double
  %call.i150 = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %0, ptr noundef nonnull readonly %q.i, i32 noundef %retval.sroa.0.0.i, i32 noundef %retval.sroa.3.0.i, i32 noundef %conv21.i, double noundef %conv.i.i, double noundef 0.000000e+00, double noundef 0.000000e+00, i64 noundef 0, i32 noundef 1)
  %5 = fptrunc <4 x double> %call.i150 to <4 x float>
  call void @llvm.lifetime.end.p0(ptr nonnull %q.i)
  ret <4 x float> %5
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define dso_local <4 x float> @__zluda_ptx_impl_texobj_1d_v4_f32_s32(i64 noundef %texobj, <1 x i32> noundef %coord) local_unnamed_addr #3 {
if.else167.i:
  %q.i = alloca %struct.q, align 4
  %vecext = extractelement <1 x i32> %coord, i64 0
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %1 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %2 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %retval.sroa.3.0.i = trunc i64 %1 to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q.i)
  %3 = and i64 %2, 4294967295
  %cmp1.i77 = icmp eq i64 %3, 1
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q.i, i8 0, i64 24, i1 false)
  %conv.i138 = sitofp i32 %vecext to double
  %arrayidx.i.i.i.i.i = getelementptr i8, ptr addrspace(1) %0, i64 72
  %4 = load i64, ptr addrspace(1) %arrayidx.i.i.i.i.i, align 8
  %cond.i.i.i.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %4, i64 1)
  %conv.i.i160 = uitofp nneg i64 %cond.i.i.i.i.i to double
  %mul.i.i161 = select i1 %cmp1.i77, double %conv.i.i160, double 1.000000e+00
  %cond.i.i162 = fmul nnan double %mul.i.i161, %conv.i138
  %5 = tail call double @llvm.floor.f64(double %cond.i.i162)
  %conv.i204.i = fptosi double %5 to i64
  switch i32 %retval.sroa.3.0.i, label %sw.default.i.i [
    i32 0, label %sw.bb.i.i
    i32 2, label %sw.bb1.i.i
    i32 3, label %sw.bb3.i.i
  ]

sw.bb.i.i:                                        ; preds = %if.else167.i
  %rem.i.i.i = srem i64 %conv.i204.i, %cond.i.i.i.i.i
  %add.i.i.i = add nsw i64 %rem.i.i.i, %cond.i.i.i.i.i
  %rem1.i.i.i = srem i64 %add.i.i.i, %cond.i.i.i.i.i
  br label %__zluda_ptx_impl_agx_tex.exit

sw.bb1.i.i:                                       ; preds = %if.else167.i
  %cmp2.i.i170 = icmp sgt i64 %cond.i.i.i.i.i, %conv.i204.i
  %6 = tail call i64 @llvm.smax.i64(i64 %conv.i204.i, i64 -1)
  %cond.i206.i = select i1 %cmp2.i.i170, i64 %6, i64 -1
  br label %__zluda_ptx_impl_agx_tex.exit

sw.bb3.i.i:                                       ; preds = %if.else167.i
  %mul.i205.i = shl nuw nsw i64 %cond.i.i.i.i.i, 1
  %rem.i27.i.i = srem i64 %conv.i204.i, %mul.i205.i
  %add.i28.i.i = add nsw i64 %rem.i27.i.i, %mul.i205.i
  %rem1.i29.i.i = srem i64 %add.i28.i.i, %mul.i205.i
  %cmp5.i.i166 = icmp slt i64 %rem1.i29.i.i, %cond.i.i.i.i.i
  %7 = xor i64 %rem1.i29.i.i, -1
  %sub9.i.i = add i64 %mul.i205.i, %7
  %cond11.i.i167 = select i1 %cmp5.i.i166, i64 %rem1.i29.i.i, i64 %sub9.i.i
  br label %__zluda_ptx_impl_agx_tex.exit

sw.default.i.i:                                   ; preds = %if.else167.i
  %cond.i.i.i171 = tail call noundef i64 @llvm.smax.i64(i64 %conv.i204.i, i64 0)
  %sub13.i.i = add nsw i64 %cond.i.i.i.i.i, -1
  %8 = tail call i64 @llvm.umin.i64(i64 %cond.i.i.i171, i64 %sub13.i.i)
  br label %__zluda_ptx_impl_agx_tex.exit

__zluda_ptx_impl_agx_tex.exit:                    ; preds = %sw.default.i.i, %sw.bb3.i.i, %sw.bb1.i.i, %sw.bb.i.i
  %retval.0.i.i168 = phi i64 [ %8, %sw.default.i.i ], [ %rem1.i.i.i, %sw.bb.i.i ], [ %cond.i206.i, %sw.bb1.i.i ], [ %cond11.i.i167, %sw.bb3.i.i ]
  %call30.i = call fastcc <4 x double> @__zluda_agx_tex_texel(ptr addrspace(1) noundef readonly %0, ptr noundef nonnull readonly %q.i, i64 noundef %retval.0.i.i168, i64 noundef 0, i64 noundef 0, i64 noundef 0)
  %9 = fptrunc <4 x double> %call30.i to <4 x float>
  call void @llvm.lifetime.end.p0(ptr nonnull %q.i)
  ret <4 x float> %9
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define dso_local <4 x float> @__zluda_ptx_impl_texobj_2d_v4_f32_f32(i64 noundef %texobj, <2 x float> noundef %coord) local_unnamed_addr #3 {
if.else167.i:
  %q.i = alloca %struct.q, align 4
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %arrayidx.i44.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 32
  %1 = load i64, ptr addrspace(1) %arrayidx.i44.i, align 8
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %2 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %3 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %3 to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q.i)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q.i, i8 0, i64 24, i1 false)
  %retval.sroa.3.0.i = trunc i64 %2 to i32
  %retval.sroa.0.0.i = trunc i64 %1 to i32
  %4 = extractelement <2 x float> %coord, i64 0
  %conv.i.i = fpext float %4 to double
  %5 = extractelement <2 x float> %coord, i64 1
  %conv.i.i144 = fpext float %5 to double
  %call.i153 = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %0, ptr noundef nonnull readonly %q.i, i32 noundef %retval.sroa.0.0.i, i32 noundef %retval.sroa.3.0.i, i32 noundef %conv21.i, double noundef %conv.i.i, double noundef %conv.i.i144, double noundef 0.000000e+00, i64 noundef 0, i32 noundef 2)
  %6 = fptrunc <4 x double> %call.i153 to <4 x float>
  call void @llvm.lifetime.end.p0(ptr nonnull %q.i)
  ret <4 x float> %6
}

; Function Attrs: alwaysinline convergent nounwind
define dso_local <4 x float> @__zluda_ptx_impl_texobj_1d_v4_f32_f32_unit(i64 noundef %texobj, <1 x float> noundef %coord) local_unnamed_addr #3 {
entry:
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %bc = bitcast <1 x float> %coord to <1 x i32>
  %1 = extractelement <1 x i32> %bc, i64 0
  %arrayidx.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 128
  %2 = load i64, ptr addrspace(1) %arrayidx.i, align 8
  %or.cond22 = icmp slt i64 %2, -6917529027641081856
  br i1 %or.cond22, label %if.then14.i, label %if.end.i

if.then14.i:                                      ; preds = %entry
  %3 = bitcast i32 %1 to float
  %call19.i = tail call <4 x i32> @__zluda_ptx_impl_agx_tex_unit(i64 noundef %2, float noundef %3, float noundef 0.000000e+00)
  %4 = bitcast <4 x i32> %call19.i to <4 x float>
  br label %__zluda_ptx_impl_agx_tex.exit

if.end.i:                                         ; preds = %entry
  %arrayidx.i44.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 32
  %5 = load i64, ptr addrspace(1) %arrayidx.i44.i, align 8
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %6 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %7 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %7 to i32
  %retval.sroa.0.0.i = trunc i64 %5 to i32
  %retval.sroa.3.0.i = trunc i64 %6 to i32
  %8 = bitcast i32 %1 to float
  %call26.i = tail call fastcc <4 x float> @__zluda_agx_tex_sample_f32(ptr addrspace(1) noundef %0, i32 noundef %retval.sroa.0.0.i, i32 noundef %retval.sroa.3.0.i, i32 noundef %conv21.i, float noundef %8, float noundef 0.000000e+00, i32 noundef 1)
  br label %__zluda_ptx_impl_agx_tex.exit

__zluda_ptx_impl_agx_tex.exit:                    ; preds = %if.then14.i, %if.end.i
  %retval.0.i = phi <4 x float> [ %4, %if.then14.i ], [ %call26.i, %if.end.i ]
  ret <4 x float> %retval.0.i
}

; Function Attrs: alwaysinline convergent nounwind
define dso_local <4 x float> @__zluda_ptx_impl_texobj_2d_v4_f32_f32_unit(i64 noundef %texobj, <2 x float> noundef %coord) local_unnamed_addr #3 {
entry:
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %bc = bitcast <2 x float> %coord to <2 x i32>
  %1 = extractelement <2 x i32> %bc, i64 0
  %2 = extractelement <2 x i32> %bc, i64 1
  %arrayidx.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 128
  %3 = load i64, ptr addrspace(1) %arrayidx.i, align 8
  %4 = and i64 %3, -2305843009213693952
  %or.cond26 = icmp eq i64 %4, -6917529027641081856
  br i1 %or.cond26, label %if.then14.i, label %if.end.i

if.then14.i:                                      ; preds = %entry
  %5 = bitcast i32 %1 to float
  %6 = bitcast i32 %2 to float
  %call19.i = tail call <4 x i32> @__zluda_ptx_impl_agx_tex_unit(i64 noundef %3, float noundef %5, float noundef %6)
  %7 = bitcast <4 x i32> %call19.i to <4 x float>
  br label %__zluda_ptx_impl_agx_tex.exit

if.end.i:                                         ; preds = %entry
  %arrayidx.i44.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 32
  %8 = load i64, ptr addrspace(1) %arrayidx.i44.i, align 8
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %9 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %10 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %10 to i32
  %retval.sroa.0.0.i = trunc i64 %8 to i32
  %retval.sroa.3.0.i = trunc i64 %9 to i32
  %11 = bitcast i32 %1 to float
  %12 = bitcast i32 %2 to float
  %call26.i = tail call fastcc <4 x float> @__zluda_agx_tex_sample_f32(ptr addrspace(1) noundef %0, i32 noundef %retval.sroa.0.0.i, i32 noundef %retval.sroa.3.0.i, i32 noundef %conv21.i, float noundef %11, float noundef %12, i32 noundef 2)
  br label %__zluda_ptx_impl_agx_tex.exit

__zluda_ptx_impl_agx_tex.exit:                    ; preds = %if.then14.i, %if.end.i
  %retval.0.i = phi <4 x float> [ %7, %if.then14.i ], [ %call26.i, %if.end.i ]
  ret <4 x float> %retval.0.i
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define dso_local <4 x float> @__zluda_ptx_impl_texobj_2d_v4_f32_s32(i64 noundef %texobj, <2 x i32> noundef %coord) local_unnamed_addr #3 {
if.else167.i:
  %q.i = alloca %struct.q, align 4
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %1 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %2 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %2 to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q.i)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q.i, i8 0, i64 24, i1 false)
  %retval.sroa.3.0.i = trunc i64 %1 to i32
  %vecext1 = extractelement <2 x i32> %coord, i64 1
  %vecext = extractelement <2 x i32> %coord, i64 0
  %conv.i140 = sitofp i32 %vecext to double
  %conv.i143 = sitofp i32 %vecext1 to double
  %call.i151 = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %0, ptr noundef nonnull readonly %q.i, i32 noundef 0, i32 noundef %retval.sroa.3.0.i, i32 noundef %conv21.i, double noundef %conv.i140, double noundef %conv.i143, double noundef 0.000000e+00, i64 noundef 0, i32 noundef 2)
  %3 = fptrunc <4 x double> %call.i151 to <4 x float>
  call void @llvm.lifetime.end.p0(ptr nonnull %q.i)
  ret <4 x float> %3
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define dso_local <4 x float> @__zluda_ptx_impl_texobj_3d_v4_f32_f32(i64 noundef %texobj, <4 x float> noundef %coord) local_unnamed_addr #3 {
if.else167.i:
  %q.i = alloca %struct.q, align 4
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %arrayidx.i44.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 32
  %1 = load i64, ptr addrspace(1) %arrayidx.i44.i, align 8
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %2 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %3 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %3 to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q.i)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q.i, i8 0, i64 24, i1 false)
  %retval.sroa.3.0.i = trunc i64 %2 to i32
  %retval.sroa.0.0.i = trunc i64 %1 to i32
  %4 = extractelement <4 x float> %coord, i64 0
  %conv.i.i = fpext float %4 to double
  %5 = extractelement <4 x float> %coord, i64 1
  %conv.i.i147 = fpext float %5 to double
  %6 = extractelement <4 x float> %coord, i64 2
  %conv.i.i150 = fpext float %6 to double
  %call.i156 = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %0, ptr noundef nonnull readonly %q.i, i32 noundef %retval.sroa.0.0.i, i32 noundef %retval.sroa.3.0.i, i32 noundef %conv21.i, double noundef %conv.i.i, double noundef %conv.i.i147, double noundef %conv.i.i150, i64 noundef 0, i32 noundef 3)
  %7 = fptrunc <4 x double> %call.i156 to <4 x float>
  call void @llvm.lifetime.end.p0(ptr nonnull %q.i)
  ret <4 x float> %7
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define dso_local <4 x float> @__zluda_ptx_impl_texobj_3d_v4_f32_s32(i64 noundef %texobj, <4 x i32> noundef %coord) local_unnamed_addr #3 {
if.else167.i:
  %q.i = alloca %struct.q, align 4
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %1 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %2 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %2 to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q.i)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q.i, i8 0, i64 24, i1 false)
  %retval.sroa.3.0.i = trunc i64 %1 to i32
  %vecext2 = extractelement <4 x i32> %coord, i64 2
  %vecext1 = extractelement <4 x i32> %coord, i64 1
  %vecext = extractelement <4 x i32> %coord, i64 0
  %conv.i142 = sitofp i32 %vecext to double
  %conv.i145 = sitofp i32 %vecext1 to double
  %conv.i148 = sitofp i32 %vecext2 to double
  %call.i153 = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %0, ptr noundef nonnull readonly %q.i, i32 noundef 0, i32 noundef %retval.sroa.3.0.i, i32 noundef %conv21.i, double noundef %conv.i142, double noundef %conv.i145, double noundef %conv.i148, i64 noundef 0, i32 noundef 3)
  %3 = fptrunc <4 x double> %call.i153 to <4 x float>
  call void @llvm.lifetime.end.p0(ptr nonnull %q.i)
  ret <4 x float> %3
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define dso_local <4 x i32> @__zluda_ptx_impl_texobj_1d_v4_s32_f32(i64 noundef %texobj, <1 x float> noundef %coord) local_unnamed_addr #3 {
if.else167.i:
  %q.i = alloca %struct.q, align 4
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %arrayidx.i44.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 32
  %1 = load i64, ptr addrspace(1) %arrayidx.i44.i, align 8
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %2 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %3 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %3 to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q.i)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q.i, i8 0, i64 24, i1 false)
  %retval.sroa.3.0.i = trunc i64 %2 to i32
  %retval.sroa.0.0.i = trunc i64 %1 to i32
  %4 = extractelement <1 x float> %coord, i64 0
  %conv.i.i = fpext float %4 to double
  %call.i145 = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %0, ptr noundef nonnull readonly %q.i, i32 noundef %retval.sroa.0.0.i, i32 noundef %retval.sroa.3.0.i, i32 noundef %conv21.i, double noundef %conv.i.i, double noundef 0.000000e+00, double noundef 0.000000e+00, i64 noundef 0, i32 noundef 1)
  %5 = fcmp uno <4 x double> %call.i145, zeroinitializer
  %6 = fptosi <4 x double> %call.i145 to <4 x i64>
  %7 = trunc <4 x i64> %6 to <4 x i32>
  %8 = select <4 x i1> %5, <4 x i32> zeroinitializer, <4 x i32> %7
  call void @llvm.lifetime.end.p0(ptr nonnull %q.i)
  ret <4 x i32> %8
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define dso_local <4 x i32> @__zluda_ptx_impl_texobj_1d_v4_s32_s32(i64 noundef %texobj, <1 x i32> noundef %coord) local_unnamed_addr #3 {
if.else167.i:
  %q.i = alloca %struct.q, align 4
  %vecext = extractelement <1 x i32> %coord, i64 0
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %1 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %2 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %retval.sroa.3.0.i = trunc i64 %1 to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q.i)
  %3 = and i64 %2, 4294967295
  %cmp1.i73 = icmp eq i64 %3, 1
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q.i, i8 0, i64 24, i1 false)
  %conv.i134 = sitofp i32 %vecext to double
  %arrayidx.i.i.i.i.i = getelementptr i8, ptr addrspace(1) %0, i64 72
  %4 = load i64, ptr addrspace(1) %arrayidx.i.i.i.i.i, align 8
  %cond.i.i.i.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %4, i64 1)
  %conv.i.i168 = uitofp nneg i64 %cond.i.i.i.i.i to double
  %mul.i.i169 = select i1 %cmp1.i73, double %conv.i.i168, double 1.000000e+00
  %cond.i.i170 = fmul nnan double %mul.i.i169, %conv.i134
  %5 = tail call double @llvm.floor.f64(double %cond.i.i170)
  %conv.i204.i = fptosi double %5 to i64
  switch i32 %retval.sroa.3.0.i, label %sw.default.i.i [
    i32 0, label %sw.bb.i.i
    i32 2, label %sw.bb1.i.i
    i32 3, label %sw.bb3.i.i
  ]

sw.bb.i.i:                                        ; preds = %if.else167.i
  %rem.i.i.i = srem i64 %conv.i204.i, %cond.i.i.i.i.i
  %add.i.i.i = add nsw i64 %rem.i.i.i, %cond.i.i.i.i.i
  %rem1.i.i.i = srem i64 %add.i.i.i, %cond.i.i.i.i.i
  br label %__zluda_ptx_impl_agx_tex.exit

sw.bb1.i.i:                                       ; preds = %if.else167.i
  %cmp2.i.i178 = icmp sgt i64 %cond.i.i.i.i.i, %conv.i204.i
  %6 = tail call i64 @llvm.smax.i64(i64 %conv.i204.i, i64 -1)
  %cond.i206.i = select i1 %cmp2.i.i178, i64 %6, i64 -1
  br label %__zluda_ptx_impl_agx_tex.exit

sw.bb3.i.i:                                       ; preds = %if.else167.i
  %mul.i205.i = shl nuw nsw i64 %cond.i.i.i.i.i, 1
  %rem.i27.i.i = srem i64 %conv.i204.i, %mul.i205.i
  %add.i28.i.i = add nsw i64 %rem.i27.i.i, %mul.i205.i
  %rem1.i29.i.i = srem i64 %add.i28.i.i, %mul.i205.i
  %cmp5.i.i174 = icmp slt i64 %rem1.i29.i.i, %cond.i.i.i.i.i
  %7 = xor i64 %rem1.i29.i.i, -1
  %sub9.i.i = add i64 %mul.i205.i, %7
  %cond11.i.i175 = select i1 %cmp5.i.i174, i64 %rem1.i29.i.i, i64 %sub9.i.i
  br label %__zluda_ptx_impl_agx_tex.exit

sw.default.i.i:                                   ; preds = %if.else167.i
  %cond.i.i.i179 = tail call noundef i64 @llvm.smax.i64(i64 %conv.i204.i, i64 0)
  %sub13.i.i = add nsw i64 %cond.i.i.i.i.i, -1
  %8 = tail call i64 @llvm.umin.i64(i64 %cond.i.i.i179, i64 %sub13.i.i)
  br label %__zluda_ptx_impl_agx_tex.exit

__zluda_ptx_impl_agx_tex.exit:                    ; preds = %sw.default.i.i, %sw.bb3.i.i, %sw.bb1.i.i, %sw.bb.i.i
  %retval.0.i.i176 = phi i64 [ %8, %sw.default.i.i ], [ %rem1.i.i.i, %sw.bb.i.i ], [ %cond.i206.i, %sw.bb1.i.i ], [ %cond11.i.i175, %sw.bb3.i.i ]
  %call30.i = call fastcc <4 x double> @__zluda_agx_tex_texel(ptr addrspace(1) noundef readonly %0, ptr noundef nonnull readonly %q.i, i64 noundef %retval.0.i.i176, i64 noundef 0, i64 noundef 0, i64 noundef 0)
  %9 = fcmp uno <4 x double> %call30.i, zeroinitializer
  %10 = fptosi <4 x double> %call30.i to <4 x i64>
  %11 = trunc <4 x i64> %10 to <4 x i32>
  %12 = select <4 x i1> %9, <4 x i32> zeroinitializer, <4 x i32> %11
  call void @llvm.lifetime.end.p0(ptr nonnull %q.i)
  ret <4 x i32> %12
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define dso_local <4 x i32> @__zluda_ptx_impl_texobj_2d_v4_s32_f32(i64 noundef %texobj, <2 x float> noundef %coord) local_unnamed_addr #3 {
if.else167.i:
  %q.i = alloca %struct.q, align 4
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %arrayidx.i44.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 32
  %1 = load i64, ptr addrspace(1) %arrayidx.i44.i, align 8
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %2 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %3 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %3 to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q.i)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q.i, i8 0, i64 24, i1 false)
  %retval.sroa.3.0.i = trunc i64 %2 to i32
  %retval.sroa.0.0.i = trunc i64 %1 to i32
  %4 = extractelement <2 x float> %coord, i64 0
  %conv.i.i = fpext float %4 to double
  %5 = extractelement <2 x float> %coord, i64 1
  %conv.i.i139 = fpext float %5 to double
  %call.i148 = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %0, ptr noundef nonnull readonly %q.i, i32 noundef %retval.sroa.0.0.i, i32 noundef %retval.sroa.3.0.i, i32 noundef %conv21.i, double noundef %conv.i.i, double noundef %conv.i.i139, double noundef 0.000000e+00, i64 noundef 0, i32 noundef 2)
  %6 = fcmp uno <4 x double> %call.i148, zeroinitializer
  %7 = fptosi <4 x double> %call.i148 to <4 x i64>
  %8 = trunc <4 x i64> %7 to <4 x i32>
  %9 = select <4 x i1> %6, <4 x i32> zeroinitializer, <4 x i32> %8
  call void @llvm.lifetime.end.p0(ptr nonnull %q.i)
  ret <4 x i32> %9
}

; Function Attrs: alwaysinline convergent nounwind
define dso_local <4 x i32> @__zluda_ptx_impl_texobj_1d_v4_s32_f32_unit(i64 noundef %texobj, <1 x float> noundef %coord) local_unnamed_addr #3 {
entry:
  %q.i = alloca %struct.q, align 4
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %bc = bitcast <1 x float> %coord to <1 x i32>
  %1 = extractelement <1 x i32> %bc, i64 0
  %arrayidx.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 128
  %2 = load i64, ptr addrspace(1) %arrayidx.i, align 8
  %3 = and i64 %2, -2305843009213693952
  %or.cond180 = icmp eq i64 %3, -4611686018427387904
  br i1 %or.cond180, label %cleanup.i, label %if.else167.i

cleanup.i:                                        ; preds = %entry
  %4 = bitcast i32 %1 to float
  %call19.i = tail call <4 x i32> @__zluda_ptx_impl_agx_tex_unit(i64 noundef %2, float noundef %4, float noundef 0.000000e+00)
  br label %__zluda_ptx_impl_agx_tex.exit

if.else167.i:                                     ; preds = %entry
  %arrayidx.i44.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 32
  %5 = load i64, ptr addrspace(1) %arrayidx.i44.i, align 8
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %6 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %7 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %7 to i32
  %retval.sroa.0.0.i = trunc i64 %5 to i32
  %retval.sroa.3.0.i = trunc i64 %6 to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q.i)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q.i, i8 0, i64 24, i1 false)
  %8 = bitcast i32 %1 to float
  %conv.i.i = fpext float %8 to double
  %call.i149 = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %0, ptr noundef nonnull readonly %q.i, i32 noundef %retval.sroa.0.0.i, i32 noundef %retval.sroa.3.0.i, i32 noundef %conv21.i, double noundef %conv.i.i, double noundef 0.000000e+00, double noundef 0.000000e+00, i64 noundef 0, i32 noundef 1)
  %9 = fcmp uno <4 x double> %call.i149, zeroinitializer
  %10 = fptosi <4 x double> %call.i149 to <4 x i64>
  %11 = trunc <4 x i64> %10 to <4 x i32>
  %12 = select <4 x i1> %9, <4 x i32> zeroinitializer, <4 x i32> %11
  call void @llvm.lifetime.end.p0(ptr nonnull %q.i)
  br label %__zluda_ptx_impl_agx_tex.exit

__zluda_ptx_impl_agx_tex.exit:                    ; preds = %cleanup.i, %if.else167.i
  %retval.1.i = phi <4 x i32> [ %12, %if.else167.i ], [ %call19.i, %cleanup.i ]
  ret <4 x i32> %retval.1.i
}

; Function Attrs: alwaysinline convergent nounwind
define dso_local <4 x i32> @__zluda_ptx_impl_texobj_2d_v4_s32_f32_unit(i64 noundef %texobj, <2 x float> noundef %coord) local_unnamed_addr #3 {
entry:
  %q.i = alloca %struct.q, align 4
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %bc = bitcast <2 x float> %coord to <2 x i32>
  %1 = extractelement <2 x i32> %bc, i64 0
  %2 = extractelement <2 x i32> %bc, i64 1
  %arrayidx.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 128
  %3 = load i64, ptr addrspace(1) %arrayidx.i, align 8
  %or.cond185 = icmp ugt i64 %3, -2305843009213693953
  br i1 %or.cond185, label %cleanup.i, label %if.else167.i

cleanup.i:                                        ; preds = %entry
  %4 = bitcast i32 %1 to float
  %5 = bitcast i32 %2 to float
  %call19.i = tail call <4 x i32> @__zluda_ptx_impl_agx_tex_unit(i64 noundef %3, float noundef %4, float noundef %5)
  br label %__zluda_ptx_impl_agx_tex.exit

if.else167.i:                                     ; preds = %entry
  %arrayidx.i44.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 32
  %6 = load i64, ptr addrspace(1) %arrayidx.i44.i, align 8
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %7 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %8 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %8 to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q.i)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q.i, i8 0, i64 24, i1 false)
  %retval.sroa.3.0.i = trunc i64 %7 to i32
  %retval.sroa.0.0.i = trunc i64 %6 to i32
  %9 = bitcast i32 %1 to float
  %conv.i.i = fpext float %9 to double
  %10 = bitcast i32 %2 to float
  %conv.i.i144 = fpext float %10 to double
  %call.i153 = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %0, ptr noundef nonnull readonly %q.i, i32 noundef %retval.sroa.0.0.i, i32 noundef %retval.sroa.3.0.i, i32 noundef %conv21.i, double noundef %conv.i.i, double noundef %conv.i.i144, double noundef 0.000000e+00, i64 noundef 0, i32 noundef 2)
  %11 = fcmp uno <4 x double> %call.i153, zeroinitializer
  %12 = fptosi <4 x double> %call.i153 to <4 x i64>
  %13 = trunc <4 x i64> %12 to <4 x i32>
  %14 = select <4 x i1> %11, <4 x i32> zeroinitializer, <4 x i32> %13
  call void @llvm.lifetime.end.p0(ptr nonnull %q.i)
  br label %__zluda_ptx_impl_agx_tex.exit

__zluda_ptx_impl_agx_tex.exit:                    ; preds = %cleanup.i, %if.else167.i
  %retval.1.i = phi <4 x i32> [ %14, %if.else167.i ], [ %call19.i, %cleanup.i ]
  ret <4 x i32> %retval.1.i
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define dso_local <4 x i32> @__zluda_ptx_impl_texobj_2d_v4_s32_s32(i64 noundef %texobj, <2 x i32> noundef %coord) local_unnamed_addr #3 {
if.else167.i:
  %q.i = alloca %struct.q, align 4
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %1 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %2 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %2 to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q.i)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q.i, i8 0, i64 24, i1 false)
  %retval.sroa.3.0.i = trunc i64 %1 to i32
  %vecext1 = extractelement <2 x i32> %coord, i64 1
  %vecext = extractelement <2 x i32> %coord, i64 0
  %conv.i136 = sitofp i32 %vecext to double
  %conv.i139 = sitofp i32 %vecext1 to double
  %call.i147 = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %0, ptr noundef nonnull readonly %q.i, i32 noundef 0, i32 noundef %retval.sroa.3.0.i, i32 noundef %conv21.i, double noundef %conv.i136, double noundef %conv.i139, double noundef 0.000000e+00, i64 noundef 0, i32 noundef 2)
  %3 = fcmp uno <4 x double> %call.i147, zeroinitializer
  %4 = fptosi <4 x double> %call.i147 to <4 x i64>
  %5 = trunc <4 x i64> %4 to <4 x i32>
  %6 = select <4 x i1> %3, <4 x i32> zeroinitializer, <4 x i32> %5
  call void @llvm.lifetime.end.p0(ptr nonnull %q.i)
  ret <4 x i32> %6
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define dso_local <4 x i32> @__zluda_ptx_impl_texobj_3d_v4_s32_f32(i64 noundef %texobj, <4 x float> noundef %coord) local_unnamed_addr #3 {
if.else167.i:
  %q.i = alloca %struct.q, align 4
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %arrayidx.i44.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 32
  %1 = load i64, ptr addrspace(1) %arrayidx.i44.i, align 8
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %2 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %3 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %3 to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q.i)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q.i, i8 0, i64 24, i1 false)
  %retval.sroa.3.0.i = trunc i64 %2 to i32
  %retval.sroa.0.0.i = trunc i64 %1 to i32
  %4 = extractelement <4 x float> %coord, i64 0
  %conv.i.i = fpext float %4 to double
  %5 = extractelement <4 x float> %coord, i64 1
  %conv.i.i142 = fpext float %5 to double
  %6 = extractelement <4 x float> %coord, i64 2
  %conv.i.i145 = fpext float %6 to double
  %call.i151 = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %0, ptr noundef nonnull readonly %q.i, i32 noundef %retval.sroa.0.0.i, i32 noundef %retval.sroa.3.0.i, i32 noundef %conv21.i, double noundef %conv.i.i, double noundef %conv.i.i142, double noundef %conv.i.i145, i64 noundef 0, i32 noundef 3)
  %7 = fcmp uno <4 x double> %call.i151, zeroinitializer
  %8 = fptosi <4 x double> %call.i151 to <4 x i64>
  %9 = trunc <4 x i64> %8 to <4 x i32>
  %10 = select <4 x i1> %7, <4 x i32> zeroinitializer, <4 x i32> %9
  call void @llvm.lifetime.end.p0(ptr nonnull %q.i)
  ret <4 x i32> %10
}

; Function Attrs: alwaysinline nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define dso_local <4 x i32> @__zluda_ptx_impl_texobj_3d_v4_s32_s32(i64 noundef %texobj, <4 x i32> noundef %coord) local_unnamed_addr #3 {
if.else167.i:
  %q.i = alloca %struct.q, align 4
  %0 = inttoptr i64 %texobj to ptr addrspace(1)
  %arrayidx.i45.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 40
  %1 = load i64, ptr addrspace(1) %arrayidx.i45.i, align 8
  %arrayidx.i46.i = getelementptr inbounds nuw i8, ptr addrspace(1) %0, i64 48
  %2 = load i64, ptr addrspace(1) %arrayidx.i46.i, align 8
  %conv21.i = trunc i64 %2 to i32
  call void @llvm.lifetime.start.p0(ptr nonnull %q.i)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %q.i, i8 0, i64 24, i1 false)
  %retval.sroa.3.0.i = trunc i64 %1 to i32
  %vecext2 = extractelement <4 x i32> %coord, i64 2
  %vecext1 = extractelement <4 x i32> %coord, i64 1
  %vecext = extractelement <4 x i32> %coord, i64 0
  %conv.i138 = sitofp i32 %vecext to double
  %conv.i141 = sitofp i32 %vecext1 to double
  %conv.i144 = sitofp i32 %vecext2 to double
  %call.i149 = call fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) noundef readonly %0, ptr noundef nonnull readonly %q.i, i32 noundef 0, i32 noundef %retval.sroa.3.0.i, i32 noundef %conv21.i, double noundef %conv.i138, double noundef %conv.i141, double noundef %conv.i144, i64 noundef 0, i32 noundef 3)
  %3 = fcmp uno <4 x double> %call.i149, zeroinitializer
  %4 = fptosi <4 x double> %call.i149 to <4 x i64>
  %5 = trunc <4 x i64> %4 to <4 x i32>
  %6 = select <4 x i1> %3, <4 x i32> zeroinitializer, <4 x i32> %5
  call void @llvm.lifetime.end.p0(ptr nonnull %q.i)
  ret <4 x i32> %6
}

; Function Attrs: mustprogress nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.floor.f32(float)

; Function Attrs: inlinehint nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define internal fastcc <4 x float> @__zluda_agx_tex_texel_f32(ptr addrspace(1) nofree noundef readonly captures(none) %t, i64 noundef %i, i64 noundef %j) unnamed_addr #3 {
entry:
  %0 = or i64 %j, %i
  %or.cond.not = icmp sgt i64 %0, -1
  br i1 %or.cond.not, label %if.end, label %cleanup

if.end:                                           ; preds = %entry
  %arrayidx.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 24
  %1 = load i64, ptr addrspace(1) %arrayidx.i, align 8
  %arrayidx.i55 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 16
  %2 = load i64, ptr addrspace(1) %arrayidx.i55, align 8
  switch i64 %2, label %sw.default.i [
    i64 0, label %code_bytes.exit
    i64 20, label %code_bytes.exit
    i64 1, label %sw.bb1.i
    i64 21, label %sw.bb1.i
    i64 6, label %sw.bb1.i
  ]

sw.bb1.i:                                         ; preds = %if.end, %if.end, %if.end
  br label %code_bytes.exit

sw.default.i:                                     ; preds = %if.end
  br label %code_bytes.exit

code_bytes.exit:                                  ; preds = %if.end, %if.end, %sw.bb1.i, %sw.default.i
  %retval.0.i = phi i64 [ 4, %sw.default.i ], [ 2, %sw.bb1.i ], [ 1, %if.end ], [ 1, %if.end ]
  %arrayidx.i56 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 112
  %3 = load i64, ptr addrspace(1) %arrayidx.i56, align 8
  %tobool.not = icmp eq i64 %3, 0
  %arrayidx.i58 = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 8
  %4 = load i64, ptr addrspace(1) %arrayidx.i58, align 8
  br i1 %tobool.not, label %cond.false, label %cond.true

cond.true:                                        ; preds = %code_bytes.exit
  %mul = mul nsw i64 %3, %j
  %mul6 = mul nsw i64 %1, %i
  %mul7 = mul nsw i64 %mul6, %retval.0.i
  %add = add nsw i64 %mul, %mul7
  br label %cond.end

cond.false:                                       ; preds = %code_bytes.exit
  %arrayidx.i.i.i = getelementptr i8, ptr addrspace(1) %t, i64 72
  %5 = load i64, ptr addrspace(1) %arrayidx.i.i.i, align 8
  %cond.i.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %5, i64 1)
  %mul11 = mul nuw nsw i64 %cond.i.i.i, %j
  %add12 = add nsw i64 %mul11, %i
  %mul13 = mul i64 %1, %retval.0.i
  %mul15 = mul i64 %mul13, %add12
  br label %cond.end

cond.end:                                         ; preds = %cond.false, %cond.true
  %mul15.sink = phi i64 [ %mul15, %cond.false ], [ %add, %cond.true ]
  %add16 = add i64 %mul15.sink, %4
  %arrayidx.i70.i = getelementptr inbounds nuw i8, ptr addrspace(1) %t, i64 56
  %cmp4.i73.not.i = icmp eq i64 %2, 5
  %sub28.i = sext i1 %cmp4.i73.not.i to i32
  br label %for.body

for.body:                                         ; preds = %cond.end, %cond.end31
  %indvars.iv = phi i64 [ 0, %cond.end ], [ %indvars.iv.next, %cond.end31 ]
  %r.060 = phi <4 x float> [ zeroinitializer, %cond.end ], [ %vecins, %cond.end31 ]
  %cmp20 = icmp sgt i64 %1, %indvars.iv
  br i1 %cmp20, label %cond.true22, label %cond.false27

cond.true22:                                      ; preds = %for.body
  %6 = mul nuw nsw i64 %indvars.iv, %retval.0.i
  %add25 = add i64 %add16, %6
  switch i64 %2, label %sw.default.i.i [
    i64 0, label %code_bytes.exit.i
    i64 20, label %code_bytes.exit.i
    i64 1, label %sw.bb1.i.i
    i64 21, label %sw.bb1.i.i
    i64 6, label %sw.bb1.i.i
  ]

sw.bb1.i.i:                                       ; preds = %cond.true22, %cond.true22, %cond.true22
  br label %code_bytes.exit.i

sw.default.i.i:                                   ; preds = %cond.true22
  br label %code_bytes.exit.i

code_bytes.exit.i:                                ; preds = %sw.default.i.i, %sw.bb1.i.i, %cond.true22, %cond.true22
  %retval.0.i.i = phi i64 [ 4, %sw.default.i.i ], [ 2, %sw.bb1.i.i ], [ 1, %cond.true22 ], [ 1, %cond.true22 ]
  %7 = inttoptr i64 %add25 to ptr addrspace(1)
  br label %for.body.i

for.cond.cleanup.i:                               ; preds = %for.body.i
  switch i64 %2, label %sw.default.i.i.i [
    i64 6, label %if.then.i
    i64 7, label %if.then7.i
    i64 0, label %code_bits.exit.i
    i64 20, label %code_bits.exit.i
    i64 1, label %sw.bb1.i.i.i
    i64 21, label %sw.bb1.i.i.i
  ]

for.body.i:                                       ; preds = %for.body.i, %code_bytes.exit.i
  %indvars.iv.i = phi i64 [ 0, %code_bytes.exit.i ], [ %indvars.iv.next.i, %for.body.i ]
  %v.080.i = phi i32 [ 0, %code_bytes.exit.i ], [ %or.i, %for.body.i ]
  %arrayidx.i59 = getelementptr inbounds nuw i8, ptr addrspace(1) %7, i64 %indvars.iv.i
  %8 = load i8, ptr addrspace(1) %arrayidx.i59, align 1
  %conv.i = zext i8 %8 to i32
  %indvars.iv.tr.i = trunc nuw nsw i64 %indvars.iv.i to i32
  %9 = shl nuw nsw i32 %indvars.iv.tr.i, 3
  %shl.i = shl nuw i32 %conv.i, %9
  %or.i = or i32 %shl.i, %v.080.i
  %indvars.iv.next.i = add nuw nsw i64 %indvars.iv.i, 1
  %exitcond.not.i = icmp eq i64 %indvars.iv.next.i, %retval.0.i.i
  br i1 %exitcond.not.i, label %for.cond.cleanup.i, label %for.body.i, !llvm.loop !10038

if.then.i:                                        ; preds = %for.cond.cleanup.i
  %shr.i.i = lshr i32 %or.i, 15
  %shr1.i.i = lshr i32 %or.i, 10
  %and.i.i = and i32 %shr1.i.i, 31
  %and2.i.i = and i32 %or.i, 1023
  switch i32 %and.i.i, label %if.then4.i.i [
    i32 31, label %if.then.i.i
    i32 0, label %if.else8.i.i
  ]

if.then.i.i:                                      ; preds = %if.then.i
  %shl.i.i = shl nuw nsw i32 %and2.i.i, 13
  %or.i.i = or disjoint i32 %shl.i.i, 2139095040
  br label %if.end9.i.i

if.then4.i.i:                                     ; preds = %if.then.i
  %add.i.i = shl nuw nsw i32 %and.i.i, 23
  %shl5.i.i = add nuw nsw i32 %add.i.i, 939524096
  %shl6.i.i = shl nuw nsw i32 %and2.i.i, 13
  %or7.i.i = or disjoint i32 %shl5.i.i, %shl6.i.i
  br label %if.end9.i.i

if.else8.i.i:                                     ; preds = %if.then.i
  %conv.i.i = uitofp nneg i32 %and2.i.i to float
  %mul.i.i = fmul nnan float %conv.i.i, f0x33800000
  %tobool.not.i.i = icmp eq i32 %shr.i.i, 0
  %fneg.i.i = fneg float %mul.i.i
  %cond.i.i = select i1 %tobool.not.i.i, float %mul.i.i, float %fneg.i.i
  br label %cond.end31

if.end9.i.i:                                      ; preds = %if.then4.i.i, %if.then.i.i
  %b.0.i.i = phi i32 [ %or.i.i, %if.then.i.i ], [ %or7.i.i, %if.then4.i.i ]
  %shl10.i.i = shl i32 %shr.i.i, 31
  %or11.i.i = or i32 %b.0.i.i, %shl10.i.i
  %10 = bitcast i32 %or11.i.i to float
  br label %cond.end31

if.then7.i:                                       ; preds = %for.cond.cleanup.i
  %11 = bitcast i32 %or.i to float
  br label %cond.end31

sw.bb1.i.i.i:                                     ; preds = %for.cond.cleanup.i, %for.cond.cleanup.i
  br label %code_bits.exit.i

sw.default.i.i.i:                                 ; preds = %for.cond.cleanup.i
  br label %code_bits.exit.i

code_bits.exit.i:                                 ; preds = %sw.default.i.i.i, %sw.bb1.i.i.i, %for.cond.cleanup.i, %for.cond.cleanup.i
  %retval.0.i.i.i = phi i32 [ 32, %sw.default.i.i.i ], [ 16, %sw.bb1.i.i.i ], [ 8, %for.cond.cleanup.i ], [ 8, %for.cond.cleanup.i ]
  %conv10.i = zext i32 %or.i to i64
  switch i64 %2, label %if.end20.i [
    i64 21, label %land.lhs.true14.i
    i64 20, label %land.lhs.true14.i
    i64 3, label %land.lhs.true14.i
    i64 5, label %land.lhs.true14.i
  ]

land.lhs.true14.i:                                ; preds = %code_bits.exit.i, %code_bits.exit.i, %code_bits.exit.i, %code_bits.exit.i
  %sub.i = add nsw i32 %retval.0.i.i.i, -1
  %12 = shl nuw i32 1, %sub.i
  %13 = and i32 %12, %or.i
  %tobool15.not.i = icmp eq i32 %13, 0
  br i1 %tobool15.not.i, label %if.end20.i, label %if.then16.i

if.then16.i:                                      ; preds = %land.lhs.true14.i
  %sh_prom.i = zext nneg i32 %retval.0.i.i.i to i64
  %shl18.neg.i = shl nsw i64 -1, %sh_prom.i
  %sub19.i = add nsw i64 %shl18.neg.i, %conv10.i
  br label %if.end20.i

if.end20.i:                                       ; preds = %if.then16.i, %land.lhs.true14.i, %code_bits.exit.i
  %iv.0.i = phi i64 [ %sub19.i, %if.then16.i ], [ %conv10.i, %land.lhs.true14.i ], [ %conv10.i, %code_bits.exit.i ]
  %14 = load i64, ptr addrspace(1) %arrayidx.i70.i, align 8
  %tobool22.not.i = icmp eq i64 %14, 0
  br i1 %tobool22.not.i, label %if.then23.i, label %if.end25.i

if.then23.i:                                      ; preds = %if.end20.i
  %conv24.i = sitofp i64 %iv.0.i to float
  br label %cond.end31

if.end25.i:                                       ; preds = %if.end20.i
  switch i64 %2, label %code_signed.exit74.i [
    i64 21, label %code_signed.exit74.thread.i
    i64 20, label %code_signed.exit74.thread.i
    i64 3, label %code_signed.exit74.thread.i
  ]

code_signed.exit74.i:                             ; preds = %if.end25.i
  br label %code_signed.exit74.thread.i

code_signed.exit74.thread.i:                      ; preds = %code_signed.exit74.i, %if.end25.i, %if.end25.i, %if.end25.i
  %sub28.sink.i = phi i32 [ %sub28.i, %code_signed.exit74.i ], [ -1, %if.end25.i ], [ -1, %if.end25.i ], [ -1, %if.end25.i ]
  %spec.select.i = add nsw i32 %sub28.sink.i, %retval.0.i.i.i
  %sh_prom29.pn.i = zext nneg i32 %spec.select.i to i64
  %cond.in.i = shl nsw i64 -1, %sh_prom29.pn.i
  %cond.i = xor i64 %cond.in.i, -1
  %conv35.i = sitofp i64 %iv.0.i to float
  %conv36.i = uitofp nneg i64 %cond.i to float
  %div.i = fdiv float %conv35.i, %conv36.i
  %cmp37.i = fcmp oge float %div.i, -1.000000e+00
  %cond42.i = select i1 %cmp37.i, float %div.i, float -1.000000e+00
  br label %cond.end31

cond.false27:                                     ; preds = %for.body
  %cmp28 = icmp eq i64 %indvars.iv, 3
  %cond30 = select i1 %cmp28, float 1.000000e+00, float 0.000000e+00
  br label %cond.end31

cond.end31:                                       ; preds = %code_signed.exit74.thread.i, %if.then23.i, %if.then7.i, %if.end9.i.i, %if.else8.i.i, %cond.false27
  %cond32 = phi float [ %cond30, %cond.false27 ], [ %conv24.i, %if.then23.i ], [ %11, %if.then7.i ], [ %cond42.i, %code_signed.exit74.thread.i ], [ %10, %if.end9.i.i ], [ %cond.i.i, %if.else8.i.i ]
  %15 = trunc nuw nsw i64 %indvars.iv to i32
  %vecins = insertelement <4 x float> %r.060, float %cond32, i32 %15
  %indvars.iv.next = add nuw nsw i64 %indvars.iv, 1
  %exitcond.not = icmp eq i64 %indvars.iv.next, 4
  br i1 %exitcond.not, label %cleanup, label %for.body, !llvm.loop !10039

cleanup:                                          ; preds = %cond.end31, %entry
  %retval.0 = phi <4 x float> [ zeroinitializer, %entry ], [ %vecins, %cond.end31 ]
  ret <4 x float> %retval.0
}

; Function Attrs: mustprogress nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)

; Function Attrs: mustprogress nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.fabs.f64(double)

; Function Attrs: inlinehint nofree norecurse nosync nounwind memory(read, inaccessiblemem: none, target_mem: none)
define internal fastcc <4 x double> @__zluda_agx_tex_sample(ptr addrspace(1) nofree noundef readonly captures(none) %t, ptr nofree noundef nonnull readonly captures(none) %q, i32 noundef %filter, i32 noundef %addr, i32 noundef %norm, double noundef %x, double noundef %y, double noundef %z, i64 noundef %layer, i32 noundef range(i32 1, 4) %rank) unnamed_addr #3 {
entry:
  %cmp.not = icmp eq i32 %filter, 1
  %0 = load i32, ptr %q, align 4
  %arrayidx.i.i.i.i = getelementptr i8, ptr addrspace(1) %t, i64 72
  %1 = load i64, ptr addrspace(1) %arrayidx.i.i.i.i, align 8
  %cond.i.i.i.i = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %1, i64 1)
  %sh_prom.i.i = zext nneg i32 %0 to i64
  %shr.i.i = lshr i64 %cond.i.i.i.i, %sh_prom.i.i
  %2 = tail call range(i64 1, -9223372036854775808) i64 @llvm.umax.i64(i64 %shr.i.i, i64 1)
  br i1 %cmp.not, label %if.end, label %if.then

if.then:                                          ; preds = %entry
  %arrayidx.i.i.i196 = getelementptr i8, ptr addrspace(1) %t, i64 80
  %3 = load i64, ptr addrspace(1) %arrayidx.i.i.i196, align 8
  %cond.i.i.i197 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %3, i64 1)
  %shr.i199 = lshr i64 %cond.i.i.i197, %sh_prom.i.i
  %4 = tail call range(i64 1, -9223372036854775808) i64 @llvm.umax.i64(i64 %shr.i199, i64 1)
  %arrayidx.i.i.i200 = getelementptr i8, ptr addrspace(1) %t, i64 88
  %5 = load i64, ptr addrspace(1) %arrayidx.i.i.i200, align 8
  %cond.i.i.i201 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %5, i64 1)
  %shr.i203 = lshr i64 %cond.i.i.i201, %sh_prom.i.i
  %6 = tail call range(i64 1, -9223372036854775808) i64 @llvm.umax.i64(i64 %shr.i203, i64 1)
  %cmp.i = icmp eq i32 %norm, 1
  %conv.i = uitofp nneg i64 %2 to double
  %mul.i = fmul double %x, %conv.i
  %cond.i = select i1 %cmp.i, double %mul.i, double %x
  %7 = tail call double @llvm.floor.f64(double %cond.i)
  %conv.i204 = fptosi double %7 to i64
  %off = getelementptr inbounds nuw i8, ptr %q, i64 4
  %8 = load i32, ptr %off, align 4
  %conv = sext i32 %8 to i64
  %add = add nsw i64 %conv.i204, %conv
  switch i32 %addr, label %sw.default.i [
    i32 0, label %sw.bb.i
    i32 2, label %sw.bb1.i
    i32 3, label %sw.bb3.i
  ]

sw.bb.i:                                          ; preds = %if.then
  %rem.i.i = srem i64 %add, %2
  %add.i.i = add nsw i64 %rem.i.i, %2
  %rem1.i.i = srem i64 %add.i.i, %2
  br label %ad_idx.exit

sw.bb1.i:                                         ; preds = %if.then
  %cmp2.i = icmp slt i64 %add, %2
  %9 = tail call i64 @llvm.smax.i64(i64 %add, i64 -1)
  %cond.i206 = select i1 %cmp2.i, i64 %9, i64 -1
  br label %ad_idx.exit

sw.bb3.i:                                         ; preds = %if.then
  %mul.i205 = shl nuw nsw i64 %2, 1
  %rem.i27.i = srem i64 %add, %mul.i205
  %add.i28.i = add nsw i64 %rem.i27.i, %mul.i205
  %rem1.i29.i = srem i64 %add.i28.i, %mul.i205
  %cmp5.i = icmp slt i64 %rem1.i29.i, %2
  %10 = xor i64 %rem1.i29.i, -1
  %sub9.i = add i64 %mul.i205, %10
  %cond11.i = select i1 %cmp5.i, i64 %rem1.i29.i, i64 %sub9.i
  br label %ad_idx.exit

sw.default.i:                                     ; preds = %if.then
  %cond.i.i = tail call noundef i64 @llvm.smax.i64(i64 %add, i64 0)
  %sub13.i = add nsw i64 %2, -1
  %11 = tail call i64 @llvm.umin.i64(i64 %cond.i.i, i64 %sub13.i)
  br label %ad_idx.exit

ad_idx.exit:                                      ; preds = %sw.bb.i, %sw.bb1.i, %sw.bb3.i, %sw.default.i
  %retval.0.i = phi i64 [ %11, %sw.default.i ], [ %rem1.i.i, %sw.bb.i ], [ %cond.i206, %sw.bb1.i ], [ %cond11.i, %sw.bb3.i ]
  %cmp8 = icmp samesign ugt i32 %rank, 1
  br i1 %cmp8, label %cond.true, label %cond.end28

cond.true:                                        ; preds = %ad_idx.exit
  %conv.i208 = uitofp nneg i64 %4 to double
  %mul.i209 = fmul double %y, %conv.i208
  %cond.i210 = select i1 %cmp.i, double %mul.i209, double %y
  %12 = tail call double @llvm.floor.f64(double %cond.i210)
  %conv.i211 = fptosi double %12 to i64
  %arrayidx13 = getelementptr inbounds nuw i8, ptr %q, i64 8
  %13 = load i32, ptr %arrayidx13, align 4
  %conv14 = sext i32 %13 to i64
  %add15 = add nsw i64 %conv14, %conv.i211
  switch i32 %addr, label %sw.default.i228 [
    i32 0, label %sw.bb.i224
    i32 2, label %sw.bb1.i221
    i32 3, label %sw.bb3.i212
  ]

sw.bb.i224:                                       ; preds = %cond.true
  %rem.i.i225 = srem i64 %add15, %4
  %add.i.i226 = add nsw i64 %rem.i.i225, %4
  %rem1.i.i227 = srem i64 %add.i.i226, %4
  br label %cond.end

sw.bb1.i221:                                      ; preds = %cond.true
  %cmp2.i222 = icmp slt i64 %add15, %4
  %14 = tail call i64 @llvm.smax.i64(i64 %add15, i64 -1)
  %cond.i223 = select i1 %cmp2.i222, i64 %14, i64 -1
  br label %cond.end

sw.bb3.i212:                                      ; preds = %cond.true
  %mul.i213 = shl nuw nsw i64 %4, 1
  %rem.i27.i214 = srem i64 %add15, %mul.i213
  %add.i28.i215 = add nsw i64 %rem.i27.i214, %mul.i213
  %rem1.i29.i216 = srem i64 %add.i28.i215, %mul.i213
  %cmp5.i217 = icmp slt i64 %rem1.i29.i216, %4
  %15 = xor i64 %rem1.i29.i216, -1
  %sub9.i218 = add i64 %mul.i213, %15
  %cond11.i219 = select i1 %cmp5.i217, i64 %rem1.i29.i216, i64 %sub9.i218
  br label %cond.end

sw.default.i228:                                  ; preds = %cond.true
  %cond.i.i229 = tail call noundef i64 @llvm.smax.i64(i64 %add15, i64 0)
  %sub13.i230 = add nsw i64 %4, -1
  %16 = tail call i64 @llvm.umin.i64(i64 %cond.i.i229, i64 %sub13.i230)
  br label %cond.end

cond.end:                                         ; preds = %sw.default.i228, %sw.bb3.i212, %sw.bb1.i221, %sw.bb.i224
  %cond = phi i64 [ %cond11.i219, %sw.bb3.i212 ], [ %16, %sw.default.i228 ], [ %rem1.i.i227, %sw.bb.i224 ], [ %cond.i223, %sw.bb1.i221 ]
  %cmp17 = icmp eq i32 %rank, 3
  br i1 %cmp17, label %cond.true19, label %cond.end28

cond.true19:                                      ; preds = %cond.end
  %conv.i234 = uitofp nneg i64 %6 to double
  %mul.i235 = fmul double %z, %conv.i234
  %cond.i236 = select i1 %cmp.i, double %mul.i235, double %z
  %17 = tail call double @llvm.floor.f64(double %cond.i236)
  %conv.i237 = fptosi double %17 to i64
  %arrayidx23 = getelementptr inbounds nuw i8, ptr %q, i64 12
  %18 = load i32, ptr %arrayidx23, align 4
  %conv24 = sext i32 %18 to i64
  %add25 = add nsw i64 %conv24, %conv.i237
  switch i32 %addr, label %sw.default.i254 [
    i32 0, label %sw.bb.i250
    i32 2, label %sw.bb1.i247
    i32 3, label %sw.bb3.i238
  ]

sw.bb.i250:                                       ; preds = %cond.true19
  %rem.i.i251 = srem i64 %add25, %6
  %add.i.i252 = add nsw i64 %rem.i.i251, %6
  %rem1.i.i253 = srem i64 %add.i.i252, %6
  br label %cond.end28

sw.bb1.i247:                                      ; preds = %cond.true19
  %cmp2.i248 = icmp slt i64 %add25, %6
  %19 = tail call i64 @llvm.smax.i64(i64 %add25, i64 -1)
  %cond.i249 = select i1 %cmp2.i248, i64 %19, i64 -1
  br label %cond.end28

sw.bb3.i238:                                      ; preds = %cond.true19
  %mul.i239 = shl nuw nsw i64 %6, 1
  %rem.i27.i240 = srem i64 %add25, %mul.i239
  %add.i28.i241 = add nsw i64 %rem.i27.i240, %mul.i239
  %rem1.i29.i242 = srem i64 %add.i28.i241, %mul.i239
  %cmp5.i243 = icmp slt i64 %rem1.i29.i242, %6
  %20 = xor i64 %rem1.i29.i242, -1
  %sub9.i244 = add i64 %mul.i239, %20
  %cond11.i245 = select i1 %cmp5.i243, i64 %rem1.i29.i242, i64 %sub9.i244
  br label %cond.end28

sw.default.i254:                                  ; preds = %cond.true19
  %cond.i.i255 = tail call noundef i64 @llvm.smax.i64(i64 %add25, i64 0)
  %sub13.i256 = add nsw i64 %6, -1
  %21 = tail call i64 @llvm.umin.i64(i64 %cond.i.i255, i64 %sub13.i256)
  br label %cond.end28

cond.end28:                                       ; preds = %ad_idx.exit, %sw.default.i254, %sw.bb3.i238, %sw.bb1.i247, %sw.bb.i250, %cond.end
  %cond475 = phi i64 [ %cond, %sw.default.i254 ], [ %cond, %cond.end ], [ %cond, %sw.bb.i250 ], [ %cond, %sw.bb1.i247 ], [ %cond, %sw.bb3.i238 ], [ 0, %ad_idx.exit ]
  %cond29 = phi i64 [ %21, %sw.default.i254 ], [ 0, %cond.end ], [ %rem1.i.i253, %sw.bb.i250 ], [ %cond.i249, %sw.bb1.i247 ], [ %cond11.i245, %sw.bb3.i238 ], [ 0, %ad_idx.exit ]
  %call30 = tail call fastcc <4 x double> @__zluda_agx_tex_texel(ptr addrspace(1) noundef %t, ptr noundef %q, i64 noundef %retval.0.i, i64 noundef %cond475, i64 noundef %cond29, i64 noundef %layer)
  br label %return

if.end:                                           ; preds = %entry
  %cmp.i.i = icmp eq i32 %norm, 1
  %conv.i.i = uitofp nneg i64 %2 to double
  %mul.i.i = fmul double %x, %conv.i.i
  %cond.i.i259 = select i1 %cmp.i.i, double %mul.i.i, double %x
  %sub.i = fadd double %cond.i.i259, -5.000000e-01
  %22 = tail call double @llvm.floor.f64(double %sub.i)
  %conv.i9.i = fptosi double %22 to i64
  %off.i = getelementptr inbounds nuw i8, ptr %q, i64 4
  %23 = load i32, ptr %off.i, align 4
  %conv3.i = sext i32 %23 to i64
  %add4.i = add i64 %conv.i9.i, %conv3.i
  switch i32 %addr, label %lin_idx.exit [
    i32 0, label %lin_idx.exit.thread
    i32 2, label %lin_idx.exit.thread482
    i32 3, label %lin_idx.exit.thread488
  ]

lin_idx.exit.thread:                              ; preds = %if.end
  %add4.i273478 = add i64 %add4.i, 1
  %24 = insertelement <2 x i64> poison, i64 %add4.i, i64 0
  %25 = insertelement <2 x i64> %24, i64 %add4.i273478, i64 1
  %26 = insertelement <2 x i64> poison, i64 %2, i64 0
  %27 = shufflevector <2 x i64> %26, <2 x i64> poison, <2 x i32> zeroinitializer
  %28 = srem <2 x i64> %25, %27
  %29 = add nsw <2 x i64> %28, %27
  %30 = srem <2 x i64> %29, %27
  br label %lin_idx.exit293

lin_idx.exit.thread482:                           ; preds = %if.end
  %cmp2.i.i = icmp slt i64 %add4.i, %2
  %add4.i273485 = add i64 %add4.i, 1
  %31 = insertelement <2 x i64> poison, i64 %add4.i, i64 0
  %32 = insertelement <2 x i64> %31, i64 %add4.i273485, i64 1
  %33 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %32, <2 x i64> splat (i64 -1))
  %cmp2.i.i284 = icmp slt i64 %add4.i273485, %2
  %34 = insertelement <2 x i1> poison, i1 %cmp2.i.i, i64 0
  %35 = insertelement <2 x i1> %34, i1 %cmp2.i.i284, i64 1
  %36 = select <2 x i1> %35, <2 x i64> %33, <2 x i64> splat (i64 -1)
  br label %lin_idx.exit293

lin_idx.exit.thread488:                           ; preds = %if.end
  %mul.i10.i = shl nuw nsw i64 %2, 1
  %add4.i273491 = add i64 %add4.i, 1
  %37 = insertelement <2 x i64> poison, i64 %add4.i, i64 0
  %38 = insertelement <2 x i64> %37, i64 %add4.i273491, i64 1
  %39 = insertelement <2 x i64> poison, i64 %mul.i10.i, i64 0
  %40 = shufflevector <2 x i64> %39, <2 x i64> poison, <2 x i32> zeroinitializer
  %41 = srem <2 x i64> %38, %40
  %42 = add nsw <2 x i64> %41, %40
  %43 = srem <2 x i64> %42, %40
  %44 = insertelement <2 x i64> poison, i64 %2, i64 0
  %45 = shufflevector <2 x i64> %44, <2 x i64> poison, <2 x i32> zeroinitializer
  %46 = icmp slt <2 x i64> %43, %45
  %47 = xor <2 x i64> %43, splat (i64 -1)
  %48 = add <2 x i64> %40, %47
  %49 = select <2 x i1> %46, <2 x i64> %43, <2 x i64> %48
  br label %lin_idx.exit293

lin_idx.exit:                                     ; preds = %if.end
  %sub13.i.i = add nsw i64 %2, -1
  %add4.i273 = add i64 %add4.i, 1
  %50 = insertelement <2 x i64> poison, i64 %add4.i, i64 0
  %51 = insertelement <2 x i64> %50, i64 %add4.i273, i64 1
  %52 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %51, <2 x i64> zeroinitializer)
  %53 = insertelement <2 x i64> poison, i64 %sub13.i.i, i64 0
  %54 = shufflevector <2 x i64> %53, <2 x i64> poison, <2 x i32> zeroinitializer
  %55 = tail call <2 x i64> @llvm.umin.v2i64(<2 x i64> %52, <2 x i64> %54)
  br label %lin_idx.exit293

lin_idx.exit293:                                  ; preds = %lin_idx.exit.thread, %lin_idx.exit.thread482, %lin_idx.exit.thread488, %lin_idx.exit
  %56 = phi <2 x i64> [ %55, %lin_idx.exit ], [ %30, %lin_idx.exit.thread ], [ %36, %lin_idx.exit.thread482 ], [ %49, %lin_idx.exit.thread488 ]
  %cmp33 = icmp samesign ugt i32 %rank, 1
  br i1 %cmp33, label %cond.true35, label %cond.end59

cond.true35:                                      ; preds = %lin_idx.exit293
  %arrayidx.i.i.i.i294 = getelementptr i8, ptr addrspace(1) %t, i64 80
  %57 = load i64, ptr addrspace(1) %arrayidx.i.i.i.i294, align 8
  %cond.i.i.i.i295 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %57, i64 1)
  %shr.i.i297 = lshr i64 %cond.i.i.i.i295, %sh_prom.i.i
  %58 = tail call range(i64 1, -9223372036854775808) i64 @llvm.umax.i64(i64 %shr.i.i297, i64 1)
  %conv.i.i299 = uitofp nneg i64 %58 to double
  %mul.i.i300 = fmul double %y, %conv.i.i299
  %cond.i.i301 = select i1 %cmp.i.i, double %mul.i.i300, double %y
  %sub.i302 = fadd double %cond.i.i301, -5.000000e-01
  %59 = tail call double @llvm.floor.f64(double %sub.i302)
  %conv.i9.i303 = fptosi double %59 to i64
  %arrayidx.i = getelementptr inbounds nuw i8, ptr %q, i64 8
  %60 = load i32, ptr %arrayidx.i, align 4
  %conv3.i305 = sext i32 %60 to i64
  %add4.i307 = add i64 %conv.i9.i303, %conv3.i305
  switch i32 %addr, label %cond.true42 [
    i32 0, label %cond.true42.thread
    i32 2, label %cond.true42.thread524
    i32 3, label %cond.true42.thread530
  ]

cond.true42.thread:                               ; preds = %cond.true35
  %add.i341519 = add nsw i64 %conv3.i305, 1
  %add4.i342520 = add i64 %add.i341519, %conv.i9.i303
  %61 = insertelement <2 x i64> poison, i64 %add4.i307, i64 0
  %62 = insertelement <2 x i64> %61, i64 %add4.i342520, i64 1
  %63 = insertelement <2 x i64> poison, i64 %58, i64 0
  %64 = shufflevector <2 x i64> %63, <2 x i64> poison, <2 x i32> zeroinitializer
  %65 = srem <2 x i64> %62, %64
  %66 = add nsw <2 x i64> %65, %64
  %67 = srem <2 x i64> %66, %64
  br label %cond.end45

cond.true42.thread524:                            ; preds = %cond.true35
  %add.i341526 = add nsw i64 %conv3.i305, 1
  %add4.i342527 = add i64 %add.i341526, %conv.i9.i303
  %68 = insertelement <2 x i64> poison, i64 %add4.i307, i64 0
  %69 = insertelement <2 x i64> %68, i64 %add4.i342527, i64 1
  %70 = insertelement <2 x i64> poison, i64 %58, i64 0
  %71 = shufflevector <2 x i64> %70, <2 x i64> poison, <2 x i32> zeroinitializer
  %72 = icmp slt <2 x i64> %69, %71
  %73 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %69, <2 x i64> splat (i64 -1))
  %74 = select <2 x i1> %72, <2 x i64> %73, <2 x i64> splat (i64 -1)
  br label %cond.end45

cond.true42.thread530:                            ; preds = %cond.true35
  %75 = insertelement <2 x i64> poison, i64 %58, i64 0
  %76 = shufflevector <2 x i64> %75, <2 x i64> poison, <2 x i32> zeroinitializer
  %77 = shl nuw nsw <2 x i64> %76, splat (i64 1)
  %add.i341532 = add nsw i64 %conv3.i305, 1
  %add4.i342533 = add i64 %add.i341532, %conv.i9.i303
  %78 = insertelement <2 x i64> poison, i64 %add4.i307, i64 0
  %79 = insertelement <2 x i64> %78, i64 %add4.i342533, i64 1
  %80 = srem <2 x i64> %79, %77
  %81 = add nsw <2 x i64> %80, %77
  %82 = srem <2 x i64> %81, %77
  %83 = icmp slt <2 x i64> %82, %76
  %84 = xor <2 x i64> %82, splat (i64 -1)
  %85 = add <2 x i64> %77, %84
  %86 = select <2 x i1> %83, <2 x i64> %82, <2 x i64> %85
  br label %cond.end45

cond.true42:                                      ; preds = %cond.true35
  %87 = insertelement <2 x i64> poison, i64 %58, i64 0
  %88 = add nsw <2 x i64> %87, <i64 -1, i64 poison>
  %89 = shufflevector <2 x i64> %88, <2 x i64> poison, <2 x i32> zeroinitializer
  %add.i341 = add nsw i64 %conv3.i305, 1
  %add4.i342 = add i64 %add.i341, %conv.i9.i303
  %90 = insertelement <2 x i64> poison, i64 %add4.i307, i64 0
  %91 = insertelement <2 x i64> %90, i64 %add4.i342, i64 1
  %92 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %91, <2 x i64> zeroinitializer)
  %93 = tail call <2 x i64> @llvm.umin.v2i64(<2 x i64> %92, <2 x i64> %89)
  br label %cond.end45

cond.end45:                                       ; preds = %cond.true42, %cond.true42.thread530, %cond.true42.thread524, %cond.true42.thread
  %94 = phi <2 x i64> [ %86, %cond.true42.thread530 ], [ %93, %cond.true42 ], [ %67, %cond.true42.thread ], [ %74, %cond.true42.thread524 ]
  %cmp47 = icmp eq i32 %rank, 3
  br i1 %cmp47, label %cond.true49, label %cond.end59

cond.true49:                                      ; preds = %cond.end45
  %arrayidx.i.i.i.i363 = getelementptr i8, ptr addrspace(1) %t, i64 88
  %95 = load i64, ptr addrspace(1) %arrayidx.i.i.i.i363, align 8
  %cond.i.i.i.i364 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %95, i64 1)
  %shr.i.i366 = lshr i64 %cond.i.i.i.i364, %sh_prom.i.i
  %96 = tail call range(i64 1, -9223372036854775808) i64 @llvm.umax.i64(i64 %shr.i.i366, i64 1)
  %conv.i.i368 = uitofp nneg i64 %96 to double
  %mul.i.i369 = fmul double %z, %conv.i.i368
  %cond.i.i370 = select i1 %cmp.i.i, double %mul.i.i369, double %z
  %sub.i371 = fadd double %cond.i.i370, -5.000000e-01
  %97 = tail call double @llvm.floor.f64(double %sub.i371)
  %conv.i9.i372 = fptosi double %97 to i64
  %arrayidx.i374 = getelementptr inbounds nuw i8, ptr %q, i64 12
  %98 = load i32, ptr %arrayidx.i374, align 4
  %conv3.i375 = sext i32 %98 to i64
  %add4.i377 = add i64 %conv.i9.i372, %conv3.i375
  switch i32 %addr, label %cond.true56 [
    i32 0, label %cond.true56.thread
    i32 2, label %cond.true56.thread541
    i32 3, label %cond.true56.thread547
  ]

cond.true56.thread:                               ; preds = %cond.true49
  %add.i411537 = add nsw i64 %conv3.i375, 1
  %add4.i412538 = add i64 %add.i411537, %conv.i9.i372
  %99 = insertelement <2 x i64> poison, i64 %add4.i377, i64 0
  %100 = insertelement <2 x i64> %99, i64 %add4.i412538, i64 1
  %101 = insertelement <2 x i64> poison, i64 %96, i64 0
  %102 = shufflevector <2 x i64> %101, <2 x i64> poison, <2 x i32> zeroinitializer
  %103 = srem <2 x i64> %100, %102
  %104 = add nsw <2 x i64> %103, %102
  %105 = srem <2 x i64> %104, %102
  br label %cond.end59

cond.true56.thread541:                            ; preds = %cond.true49
  %add.i411543 = add nsw i64 %conv3.i375, 1
  %add4.i412544 = add i64 %add.i411543, %conv.i9.i372
  %106 = insertelement <2 x i64> poison, i64 %add4.i377, i64 0
  %107 = insertelement <2 x i64> %106, i64 %add4.i412544, i64 1
  %108 = insertelement <2 x i64> poison, i64 %96, i64 0
  %109 = shufflevector <2 x i64> %108, <2 x i64> poison, <2 x i32> zeroinitializer
  %110 = icmp slt <2 x i64> %107, %109
  %111 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %107, <2 x i64> splat (i64 -1))
  %112 = select <2 x i1> %110, <2 x i64> %111, <2 x i64> splat (i64 -1)
  br label %cond.end59

cond.true56.thread547:                            ; preds = %cond.true49
  %113 = insertelement <2 x i64> poison, i64 %96, i64 0
  %114 = shufflevector <2 x i64> %113, <2 x i64> poison, <2 x i32> zeroinitializer
  %115 = shl nuw nsw <2 x i64> %114, splat (i64 1)
  %add.i411549 = add nsw i64 %conv3.i375, 1
  %add4.i412550 = add i64 %add.i411549, %conv.i9.i372
  %116 = insertelement <2 x i64> poison, i64 %add4.i377, i64 0
  %117 = insertelement <2 x i64> %116, i64 %add4.i412550, i64 1
  %118 = srem <2 x i64> %117, %115
  %119 = add nsw <2 x i64> %118, %115
  %120 = srem <2 x i64> %119, %115
  %121 = icmp slt <2 x i64> %120, %114
  %122 = xor <2 x i64> %120, splat (i64 -1)
  %123 = add <2 x i64> %115, %122
  %124 = select <2 x i1> %121, <2 x i64> %120, <2 x i64> %123
  br label %cond.end59

cond.true56:                                      ; preds = %cond.true49
  %125 = insertelement <2 x i64> poison, i64 %96, i64 0
  %126 = add nsw <2 x i64> %125, <i64 -1, i64 poison>
  %127 = shufflevector <2 x i64> %126, <2 x i64> poison, <2 x i32> zeroinitializer
  %add.i411 = add nsw i64 %conv3.i375, 1
  %add4.i412 = add i64 %add.i411, %conv.i9.i372
  %128 = insertelement <2 x i64> poison, i64 %add4.i377, i64 0
  %129 = insertelement <2 x i64> %128, i64 %add4.i412, i64 1
  %130 = tail call <2 x i64> @llvm.smax.v2i64(<2 x i64> %129, <2 x i64> zeroinitializer)
  %131 = tail call <2 x i64> @llvm.umin.v2i64(<2 x i64> %130, <2 x i64> %127)
  br label %cond.end59

cond.end59:                                       ; preds = %lin_idx.exit293, %cond.true56, %cond.true56.thread547, %cond.true56.thread541, %cond.true56.thread, %cond.end45
  %cmp47502507 = phi i1 [ false, %cond.end45 ], [ true, %cond.true56 ], [ true, %cond.true56.thread ], [ true, %cond.true56.thread541 ], [ true, %cond.true56.thread547 ], [ false, %lin_idx.exit293 ]
  %132 = phi <2 x i64> [ %94, %cond.end45 ], [ %94, %cond.true56 ], [ %94, %cond.true56.thread ], [ %94, %cond.true56.thread541 ], [ %94, %cond.true56.thread547 ], [ zeroinitializer, %lin_idx.exit293 ]
  %133 = phi <2 x i64> [ zeroinitializer, %cond.end45 ], [ %131, %cond.true56 ], [ %105, %cond.true56.thread ], [ %112, %cond.true56.thread541 ], [ %124, %cond.true56.thread547 ], [ zeroinitializer, %lin_idx.exit293 ]
  %sub.i.i = fsub double %sub.i, %22
  %mul.i3.i = fmul double %sub.i.i, 2.560000e+02
  %add.i.i442 = fadd double %mul.i3.i, 5.000000e-01
  %134 = tail call double @llvm.floor.f64(double %add.i.i442)
  %conv.i.i.i = fptosi double %134 to i64
  %conv.i4.i = sitofp i64 %conv.i.i.i to double
  %div.i.i = fmul nnan double %conv.i4.i, 3.906250e-03
  br i1 %cmp33, label %cond.true64, label %cond.end67

cond.true64:                                      ; preds = %cond.end59
  %arrayidx.i.i.i.i443 = getelementptr i8, ptr addrspace(1) %t, i64 80
  %135 = load i64, ptr addrspace(1) %arrayidx.i.i.i.i443, align 8
  %cond.i.i.i.i444 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %135, i64 1)
  %shr.i.i446 = lshr i64 %cond.i.i.i.i444, %sh_prom.i.i
  %136 = tail call range(i64 1, -9223372036854775808) i64 @llvm.umax.i64(i64 %shr.i.i446, i64 1)
  %conv.i.i448 = uitofp nneg i64 %136 to double
  %mul.i.i449 = fmul double %y, %conv.i.i448
  %cond.i.i450 = select i1 %cmp.i.i, double %mul.i.i449, double %y
  %sub.i451 = fadd double %cond.i.i450, -5.000000e-01
  %137 = tail call double @llvm.floor.f64(double %sub.i451)
  %sub.i.i452 = fsub double %sub.i451, %137
  %mul.i3.i453 = fmul double %sub.i.i452, 2.560000e+02
  %add.i.i454 = fadd double %mul.i3.i453, 5.000000e-01
  %138 = tail call double @llvm.floor.f64(double %add.i.i454)
  %conv.i.i.i455 = fptosi double %138 to i64
  %conv.i4.i456 = sitofp i64 %conv.i.i.i455 to double
  %div.i.i457 = fmul nnan double %conv.i4.i456, 3.906250e-03
  br label %cond.end67

cond.end67:                                       ; preds = %cond.end59, %cond.true64
  %cond68 = phi double [ %div.i.i457, %cond.true64 ], [ 0.000000e+00, %cond.end59 ]
  br i1 %cmp47502507, label %cond.true71, label %cond.end74

cond.true71:                                      ; preds = %cond.end67
  %arrayidx.i.i.i.i458 = getelementptr i8, ptr addrspace(1) %t, i64 88
  %139 = load i64, ptr addrspace(1) %arrayidx.i.i.i.i458, align 8
  %cond.i.i.i.i459 = tail call noundef range(i64 1, -9223372036854775808) i64 @llvm.smax.i64(i64 %139, i64 1)
  %shr.i.i461 = lshr i64 %cond.i.i.i.i459, %sh_prom.i.i
  %140 = tail call range(i64 1, -9223372036854775808) i64 @llvm.umax.i64(i64 %shr.i.i461, i64 1)
  %conv.i.i463 = uitofp nneg i64 %140 to double
  %mul.i.i464 = fmul double %z, %conv.i.i463
  %cond.i.i465 = select i1 %cmp.i.i, double %mul.i.i464, double %z
  %sub.i466 = fadd double %cond.i.i465, -5.000000e-01
  %141 = tail call double @llvm.floor.f64(double %sub.i466)
  %sub.i.i467 = fsub double %sub.i466, %141
  %mul.i3.i468 = fmul double %sub.i.i467, 2.560000e+02
  %add.i.i469 = fadd double %mul.i3.i468, 5.000000e-01
  %142 = tail call double @llvm.floor.f64(double %add.i.i469)
  %conv.i.i.i470 = fptosi double %142 to i64
  %conv.i4.i471 = sitofp i64 %conv.i.i.i470 to double
  %div.i.i472 = fmul nnan double %conv.i4.i471, 3.906250e-03
  br label %cond.end74

cond.end74:                                       ; preds = %cond.end67, %cond.true71
  %cond75 = phi double [ %div.i.i472, %cond.true71 ], [ 0.000000e+00, %cond.end67 ]
  %sub = fsub nnan double 1.000000e+00, %div.i.i
  %sub88 = fsub double 1.000000e+00, %cond68
  %sub94 = fsub double 1.000000e+00, %cond75
  %143 = extractelement <2 x i64> %132, i64 0
  %144 = extractelement <2 x i64> %132, i64 1
  %145 = extractelement <2 x i64> %133, i64 0
  %146 = extractelement <2 x i64> %133, i64 1
  %147 = extractelement <2 x i64> %56, i64 0
  %148 = extractelement <2 x i64> %56, i64 1
  br label %for.body

for.body:                                         ; preds = %cond.end74, %for.body
  %n.0517 = phi i32 [ 0, %cond.end74 ], [ %inc, %for.body ]
  %acc.0516 = phi <4 x double> [ zeroinitializer, %cond.end74 ], [ %add115, %for.body ]
  %and = and i32 %n.0517, 1
  %tobool.not = icmp eq i32 %and, 0
  %cond84 = select i1 %tobool.not, double %sub, double %div.i.i
  %149 = and i32 %n.0517, 2
  %tobool85.not = icmp eq i32 %149, 0
  %cond90 = select i1 %tobool85.not, double %sub88, double %cond68
  %mul = fmul double %cond84, %cond90
  %150 = and i32 %n.0517, 4
  %tobool91.not = icmp eq i32 %150, 0
  %cond96 = select i1 %tobool91.not, double %sub94, double %cond75
  %mul97 = fmul double %cond96, %mul
  %splat.splatinsert = insertelement <4 x double> poison, double %mul97, i64 0
  %splat.splat = shufflevector <4 x double> %splat.splatinsert, <4 x double> poison, <4 x i32> zeroinitializer
  %cond102 = select i1 %tobool.not, i64 %147, i64 %148
  %cond107 = select i1 %tobool85.not, i64 %143, i64 %144
  %cond112 = select i1 %tobool91.not, i64 %145, i64 %146
  %call113 = tail call fastcc <4 x double> @__zluda_agx_tex_texel(ptr addrspace(1) noundef %t, ptr noundef %q, i64 noundef %cond102, i64 noundef %cond107, i64 noundef %cond112, i64 noundef %layer)
  %mul114 = fmul <4 x double> %splat.splat, %call113
  %add115 = fadd <4 x double> %acc.0516, %mul114
  %inc = add nuw nsw i32 %n.0517, 1
  %n.0.highbits = lshr i32 %inc, %rank
  %cmp76 = icmp eq i32 %n.0.highbits, 0
  br i1 %cmp76, label %for.body, label %return, !llvm.loop !10040

return:                                           ; preds = %for.body, %cond.end28
  %retval.0 = phi <4 x double> [ %call30, %cond.end28 ], [ %add115, %for.body ]
  ret <4 x double> %retval.0
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg)

; Function Attrs: mustprogress nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.floor.f64(double)

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.smax.i64(i64, i64)

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.smin.i64(i64, i64)

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64)

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umin.i64(i64, i64)

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.usub.sat.i64(i64, i64)

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.smin.i32(i32, i32)

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg)

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare <2 x i64> @llvm.smax.v2i64(<2 x i64>, <2 x i64>)

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare <2 x i64> @llvm.umin.v2i64(<2 x i64>, <2 x i64>)

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare double @llvm.vector.reduce.fadd.v4f64(double, <4 x double>)

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare <2 x double> @llvm.floor.v2f64(<2 x double>)

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare <4 x i64> @llvm.smax.v4i64(<4 x i64>, <4 x i64>)

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare <4 x i64> @llvm.umax.v4i64(<4 x i64>, <4 x i64>)

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.vector.reduce.mul.v4i64(<4 x i64>)



!10000 = !{i32 7, !"frame-pointer", i32 2}
!10001 = !{!"clang version 24.0.0git (git@github-key-3:Jianwen-Ding/llvm-project.git 4b4d3ad92d2a577f627048842d1477130eaa9799)"}
!10002 = !{!10003, !10004, i64 0}
!10003 = !{!"__libc_errno", !10004, i64 0}
!10004 = !{!"int", !10005, i64 0}
!10005 = !{!"omnipotent char", !10006, i64 0}
!10006 = !{!"Simple C/C++ TBAA"}
!10007 = !{!10008, !10008, i64 0}
!10008 = !{!"long long", !10005, i64 0}
!10009 = !{!10010, !10004, i64 0}
!10010 = !{!"q", !10004, i64 0, !10005, i64 4, !10004, i64 16, !10004, i64 20}
!10011 = !{!10004, !10004, i64 0}
!10012 = !{!10010, !10004, i64 16}
!10013 = !{!10010, !10004, i64 20}
!10014 = !{!10015, !10015, i64 0}
!10015 = !{!"double", !10005, i64 0}
!10016 = distinct !{!10016, !10017, !10018}
!10017 = !{!"llvm.loop.mustprogress"}
!10018 = !{!"llvm.loop.unroll.disable"}
!10019 = distinct !{!10019, !10017}
!10020 = !{i64 0, i64 4, !10011, i64 4, i64 12, !10021, i64 16, i64 4, !10011, i64 20, i64 4, !10011}
!10021 = !{!10005, !10005, i64 0}
!10022 = distinct !{!10022, !10017, !10018}
!10023 = distinct !{!10023, !10017}
!10024 = distinct !{!10024, !10017}
!10025 = distinct !{!10025, !10017}
!10026 = distinct !{!10026, !10017}
!10027 = distinct !{!10027, !10017, !10018}
!10028 = distinct !{!10028, !10017}
!10029 = distinct !{!10029, !10017, !10018}
!10030 = distinct !{!10030, !10017}
!10031 = distinct !{!10031, !10017}
!10032 = distinct !{!10032, !10017, !10033, !10034}
!10033 = !{!"llvm.loop.isvectorized", i32 1}
!10034 = !{!"llvm.loop.unroll.runtime.disable"}
!10035 = distinct !{!10035, !10017}
!10036 = distinct !{!10036, !10017, !10034, !10033}
!10037 = distinct !{!10037, !10017}
!10038 = distinct !{!10038, !10017}
!10039 = distinct !{!10039, !10017, !10018}
!10040 = distinct !{!10040, !10017, !10018}

; from zluda_ptx_impl_agx_cvt.c

@__zluda_agx_tmem_map = internal addrspace(1) global [4 x i32] zeroinitializer, align 4
@__zluda_agx_tmem_owner = internal addrspace(1) global [4 x i32] zeroinitializer, align 4
@__zluda_agx_tmem_pool = internal addrspace(1) global [524288 x i32] zeroinitializer, align 4

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
  %157 = icmp eq i32 %133, 255
  %158 = select i1 %157, i32 1, i32 %122
  %159 = sub nsw i32 %125, %133
  %160 = add nsw i32 %159, 127
  %161 = icmp eq i32 %0, 7
  %162 = and i32 %1, 3
  br i1 %161, label %163, label %227

163:                                              ; preds = %154
  %164 = icmp eq i32 %158, 0
  %165 = or i32 %158, %123
  %166 = icmp eq i32 %165, 0
  %167 = select i1 %164, i32 254, i32 255
  br i1 %166, label %168, label %493

168:                                              ; preds = %163
  %169 = icmp eq i64 %156, 0
  br i1 %169, label %191, label %170

170:                                              ; preds = %168, %170
  %171 = phi i32 [ %174, %170 ], [ 0, %168 ]
  %172 = phi i64 [ %173, %170 ], [ %156, %168 ]
  %173 = lshr i64 %172, 1
  %174 = add nuw nsw i32 %171, 1
  %175 = icmp ult i64 %172, 2
  br i1 %175, label %176, label %170, !llvm.loop !20003

176:                                              ; preds = %170
  %177 = add nsw i32 %174, %160
  %178 = icmp sgt i32 %177, 127
  br i1 %178, label %179, label %191

179:                                              ; preds = %176, %179
  %180 = phi i32 [ %183, %179 ], [ 0, %176 ]
  %181 = phi i64 [ %182, %179 ], [ %156, %176 ]
  %182 = lshr i64 %181, 1
  %183 = add nuw nsw i32 %180, 1
  %184 = icmp ult i64 %181, 2
  br i1 %184, label %185, label %179, !llvm.loop !20003

185:                                              ; preds = %179
  %186 = add nsw i32 %183, %160
  %187 = icmp eq i32 %186, 128
  %188 = tail call i64 @llvm.ctpop.i64(i64 %156)
  %189 = icmp ult i64 %188, 2
  %190 = select i1 %187, i1 %189, i1 false
  br i1 %190, label %191, label %493

191:                                              ; preds = %185, %176, %168
  %192 = icmp eq i32 %155, 0
  %193 = icmp ne i64 %156, 0
  %194 = and i1 %192, %193
  br i1 %194, label %195, label %493

195:                                              ; preds = %191, %195
  %196 = phi i32 [ %199, %195 ], [ 0, %191 ]
  %197 = phi i64 [ %198, %195 ], [ %156, %191 ]
  %198 = lshr i64 %197, 1
  %199 = add nuw nsw i32 %196, 1
  %200 = icmp ult i64 %197, 2
  br i1 %200, label %201, label %195, !llvm.loop !20003

201:                                              ; preds = %195
  %202 = add nsw i32 %199, %160
  %203 = add i32 %196, %160
  %204 = tail call i64 @llvm.ctpop.i64(i64 %156)
  %205 = icmp ult i64 %204, 2
  %206 = icmp ne i32 %162, 2
  %207 = select i1 %206, i1 true, i1 %205
  br i1 %207, label %208, label %222

208:                                              ; preds = %201
  %209 = icmp ne i32 %162, 0
  %210 = select i1 %209, i1 true, i1 %205
  br i1 %210, label %222, label %211

211:                                              ; preds = %208
  %212 = add nsw i32 %196, -1
  %213 = zext nneg i32 %212 to i64
  %214 = shl i64 3, %213
  %215 = icmp ugt i64 %156, %214
  br i1 %215, label %221, label %216

216:                                              ; preds = %211
  %217 = icmp ne i64 %156, %214
  %218 = and i32 %202, 1
  %219 = icmp eq i32 %218, 0
  %220 = select i1 %217, i1 true, i1 %219
  br i1 %220, label %222, label %221

221:                                              ; preds = %216, %211
  br label %222

222:                                              ; preds = %221, %216, %208, %201
  %223 = phi i32 [ %203, %208 ], [ %202, %201 ], [ %202, %221 ], [ %203, %216 ]
  %224 = tail call i32 @llvm.smin.i32(i32 %223, i32 127)
  %225 = tail call i32 @llvm.smax.i32(i32 %224, i32 -127)
  %226 = add nsw i32 %225, 127
  br label %493

227:                                              ; preds = %154
  switch i32 %162, label %234 [
    i32 2, label %228
    i32 3, label %231
  ]

228:                                              ; preds = %227
  %229 = icmp eq i32 %155, 0
  %230 = select i1 %229, i32 2, i32 1
  br label %234

231:                                              ; preds = %227
  %232 = icmp eq i32 %155, 0
  %233 = select i1 %232, i32 1, i32 2
  br label %234

234:                                              ; preds = %231, %228, %227
  %235 = phi i32 [ %230, %228 ], [ %233, %231 ], [ %162, %227 ]
  %236 = and i32 %1, 4
  %237 = icmp ne i32 %236, 0
  %238 = icmp eq i32 %0, 6
  %239 = icmp eq i32 %158, 0
  br i1 %238, label %240, label %298

240:                                              ; preds = %234
  br i1 %239, label %241, label %493

241:                                              ; preds = %240
  %242 = icmp ne i32 %155, 0
  %243 = select i1 %237, i1 %242, i1 false
  br i1 %243, label %493, label %244

244:                                              ; preds = %241
  br i1 %137, label %247, label %245

245:                                              ; preds = %244
  %246 = select i1 %242, i32 128, i32 127
  br label %493

247:                                              ; preds = %244
  %248 = icmp eq i64 %156, 0
  br i1 %248, label %287, label %249

249:                                              ; preds = %247
  %250 = add nsw i32 %159, 133
  %251 = icmp sgt i32 %159, -134
  br i1 %251, label %252, label %257

252:                                              ; preds = %249
  %253 = icmp ugt i32 %250, 40
  %254 = zext nneg i32 %250 to i64
  %255 = shl i64 %156, %254
  %256 = select i1 %253, i64 4611686018427387904, i64 %255
  br label %287

257:                                              ; preds = %249
  %258 = sub nuw nsw i32 -133, %159
  %259 = icmp ult i32 %250, -63
  %260 = zext nneg i32 %258 to i64
  %261 = lshr i64 %156, %260
  %262 = select i1 %259, i64 0, i64 %261
  %263 = shl nsw i64 -1, %260
  %264 = xor i64 %263, -1
  %265 = select i1 %259, i64 -1, i64 %264
  %266 = and i64 %156, %265
  %267 = icmp eq i64 %266, 0
  %268 = icmp eq i32 %235, 1
  %269 = or i1 %267, %268
  br i1 %269, label %287, label %270

270:                                              ; preds = %257
  %271 = icmp eq i32 %235, 2
  br i1 %271, label %272, label %274

272:                                              ; preds = %270
  %273 = add nuw nsw i64 %262, 1
  br label %287

274:                                              ; preds = %270
  br i1 %259, label %287, label %275

275:                                              ; preds = %274
  %276 = sub nuw nsw i32 -134, %159
  %277 = zext nneg i32 %276 to i64
  %278 = shl nuw nsw i64 1, %277
  %279 = icmp ugt i64 %266, %278
  %280 = icmp eq i64 %266, %278
  %281 = and i64 %261, 1
  %282 = icmp ne i64 %281, 0
  %283 = select i1 %280, i1 %282, i1 false
  %284 = select i1 %279, i1 true, i1 %283
  %285 = zext i1 %284 to i64
  %286 = add nuw nsw i64 %261, %285
  br label %287

287:                                              ; preds = %275, %274, %272, %257, %252, %247
  %288 = phi i64 [ 0, %247 ], [ %256, %252 ], [ %273, %272 ], [ %286, %275 ], [ %262, %257 ], [ 0, %274 ]
  %289 = icmp eq i32 %155, 0
  br i1 %289, label %295, label %290

290:                                              ; preds = %287
  %291 = tail call i64 @llvm.umin.i64(i64 %288, i64 128)
  %292 = trunc i64 %291 to i32
  %293 = sub nsw i32 0, %292
  %294 = and i32 %293, 255
  br label %493

295:                                              ; preds = %287
  %296 = tail call i64 @llvm.umin.i64(i64 %288, i64 127)
  %297 = trunc i64 %296 to i32
  br label %493

298:                                              ; preds = %234
  br i1 %239, label %306, label %299

299:                                              ; preds = %298
  %300 = icmp ult i32 %0, 2
  br i1 %300, label %493, label %301

301:                                              ; preds = %299
  switch i32 %0, label %302 [
    i32 5, label %493
    i32 0, label %305
    i32 1, label %303
    i32 4, label %304
  ]

302:                                              ; preds = %301
  br label %493

303:                                              ; preds = %301
  br label %493

304:                                              ; preds = %301
  br label %493

305:                                              ; preds = %301
  br label %493

306:                                              ; preds = %298
  %307 = icmp ne i32 %0, 5
  %308 = or i1 %307, %237
  %309 = icmp eq i32 %155, 0
  %310 = select i1 %308, i1 true, i1 %309
  br i1 %310, label %365, label %311

311:                                              ; preds = %306
  br i1 %137, label %312, label %493

312:                                              ; preds = %311
  %313 = icmp eq i64 %156, 0
  br i1 %313, label %365, label %314

314:                                              ; preds = %312, %314
  %315 = phi i32 [ %318, %314 ], [ 0, %312 ]
  %316 = phi i64 [ %317, %314 ], [ %156, %312 ]
  %317 = lshr i64 %316, 1
  %318 = add nuw nsw i32 %315, 1
  %319 = icmp ult i64 %316, 2
  br i1 %319, label %320, label %314, !llvm.loop !20003

320:                                              ; preds = %314
  %321 = add nsw i32 %318, %160
  %322 = add i32 %315, %160
  %323 = icmp sgt i32 %321, -14
  %324 = select i1 %323, i32 %322, i32 -14
  %325 = icmp sgt i32 %324, 200
  br i1 %325, label %362, label %326

326:                                              ; preds = %320
  %327 = sub i32 %160, %324
  %328 = add i32 %327, 3
  %329 = icmp sgt i32 %328, -1
  br i1 %329, label %330, label %335

330:                                              ; preds = %326
  %331 = icmp ugt i32 %328, 40
  %332 = zext nneg i32 %328 to i64
  %333 = shl i64 %156, %332
  %334 = select i1 %331, i64 4611686018427387904, i64 %333
  br label %348

335:                                              ; preds = %326
  %336 = sub nuw i32 -3, %327
  %337 = icmp ult i32 %328, -63
  %338 = zext nneg i32 %336 to i64
  %339 = lshr i64 %156, %338
  %340 = select i1 %337, i64 0, i64 %339
  %341 = shl nsw i64 -1, %338
  %342 = xor i64 %341, -1
  %343 = select i1 %337, i64 -1, i64 %342
  %344 = and i64 %343, %156
  %345 = icmp ne i64 %344, 0
  %346 = zext i1 %345 to i64
  %347 = add nuw nsw i64 %340, %346
  br label %348

348:                                              ; preds = %335, %330
  %349 = phi i64 [ %334, %330 ], [ %347, %335 ]
  %350 = icmp ult i64 %349, 16
  br i1 %350, label %355, label %351

351:                                              ; preds = %348
  %352 = add nsw i32 %324, 16
  %353 = sext i32 %352 to i64
  %354 = shl nsw i64 %353, 3
  br label %362

355:                                              ; preds = %348
  %356 = icmp ult i64 %349, 8
  %357 = sext i32 %324 to i64
  %358 = shl nsw i64 %357, 3
  %359 = add nsw i64 %358, 112
  %360 = select i1 %356, i64 0, i64 %359
  %361 = add i64 %360, %349
  br label %362

362:                                              ; preds = %355, %351, %320
  %363 = phi i64 [ 1099511627776, %320 ], [ %354, %351 ], [ %361, %355 ]
  %364 = icmp sgt i64 %363, 254
  br i1 %364, label %493, label %365

365:                                              ; preds = %362, %312, %306
  %366 = icmp eq i32 %0, 5
  %367 = or i1 %366, %237
  %368 = icmp ne i32 %155, 0
  %369 = select i1 %367, i1 %368, i1 false
  br i1 %369, label %493, label %370

370:                                              ; preds = %365
  %371 = and i1 %307, %368
  %372 = and i32 %0, -2
  %373 = icmp eq i32 %372, 2
  %374 = icmp eq i32 %0, 4
  %375 = select i1 %374, i32 8, i32 128
  %376 = select i1 %373, i32 32, i32 %375
  %377 = select i1 %371, i32 %376, i32 0
  switch i32 %0, label %378 [
    i32 0, label %382
    i32 1, label %380
    i32 4, label %381
  ]

378:                                              ; preds = %370
  %379 = select i1 %366, i64 254, i64 31
  br label %382

380:                                              ; preds = %370
  br label %382

381:                                              ; preds = %370
  br label %382

382:                                              ; preds = %381, %380, %378, %370
  %383 = phi i64 [ 126, %370 ], [ %379, %378 ], [ 123, %380 ], [ 7, %381 ]
  br i1 %137, label %384, label %483

384:                                              ; preds = %382
  %385 = icmp eq i64 %156, 0
  br i1 %385, label %469, label %386

386:                                              ; preds = %384
  switch i32 %0, label %387 [
    i32 0, label %391
    i32 4, label %390
    i32 2, label %390
  ]

387:                                              ; preds = %386
  %388 = icmp eq i32 %0, 3
  %389 = select i1 %388, i32 2, i32 4
  br label %391

390:                                              ; preds = %386, %386
  br label %391

391:                                              ; preds = %390, %387, %386
  %392 = phi i32 [ 3, %386 ], [ %389, %387 ], [ 1, %390 ]
  %393 = and i32 %0, -3
  %394 = shl nsw i32 -1, %392
  br label %395

395:                                              ; preds = %395, %391
  %396 = phi i32 [ %399, %395 ], [ 0, %391 ]
  %397 = phi i64 [ %398, %395 ], [ %156, %391 ]
  %398 = lshr i64 %397, 1
  %399 = add nuw nsw i32 %396, 1
  %400 = icmp ult i64 %397, 2
  br i1 %400, label %401, label %395, !llvm.loop !20003

401:                                              ; preds = %395
  %402 = select i1 %374, i32 1, i32 3
  %403 = xor i32 %394, -1
  %404 = icmp eq i32 %393, 1
  %405 = select i1 %404, i32 2, i32 %402
  %406 = add nsw i32 %394, 2
  %407 = add nsw i32 %399, %160
  %408 = add i32 %396, %160
  %409 = icmp sgt i32 %407, %406
  %410 = select i1 %409, i32 %408, i32 %406
  %411 = icmp sgt i32 %410, 200
  br i1 %411, label %469, label %412

412:                                              ; preds = %401
  %413 = sub i32 %405, %410
  %414 = add i32 %413, %160
  %415 = icmp sgt i32 %414, -1
  br i1 %415, label %416, label %421

416:                                              ; preds = %412
  %417 = icmp ugt i32 %414, 40
  %418 = zext nneg i32 %414 to i64
  %419 = shl i64 %156, %418
  %420 = select i1 %417, i64 4611686018427387904, i64 %419
  br label %451

421:                                              ; preds = %412
  %422 = sub nsw i32 0, %414
  %423 = icmp ult i32 %414, -63
  %424 = zext nneg i32 %422 to i64
  %425 = lshr i64 %156, %424
  %426 = select i1 %423, i64 0, i64 %425
  %427 = shl nsw i64 -1, %424
  %428 = xor i64 %427, -1
  %429 = select i1 %423, i64 -1, i64 %428
  %430 = and i64 %429, %156
  %431 = icmp eq i64 %430, 0
  %432 = icmp eq i32 %235, 1
  %433 = or i1 %432, %431
  br i1 %433, label %451, label %434

434:                                              ; preds = %421
  %435 = icmp eq i32 %235, 2
  br i1 %435, label %436, label %438

436:                                              ; preds = %434
  %437 = add nuw nsw i64 %426, 1
  br label %451

438:                                              ; preds = %434
  br i1 %423, label %451, label %439

439:                                              ; preds = %438
  %440 = xor i32 %414, -1
  %441 = zext nneg i32 %440 to i64
  %442 = shl nuw nsw i64 1, %441
  %443 = icmp ugt i64 %430, %442
  %444 = icmp eq i64 %430, %442
  %445 = and i64 %425, 1
  %446 = icmp ne i64 %445, 0
  %447 = select i1 %444, i1 %446, i1 false
  %448 = select i1 %443, i1 true, i1 %447
  %449 = zext i1 %448 to i64
  %450 = add nuw nsw i64 %425, %449
  br label %451

451:                                              ; preds = %439, %438, %436, %421, %416
  %452 = phi i64 [ %420, %416 ], [ %437, %436 ], [ %450, %439 ], [ %426, %421 ], [ 0, %438 ]
  %453 = zext nneg i32 %405 to i64
  %454 = shl nuw nsw i64 2, %453
  %455 = icmp ult i64 %452, %454
  br i1 %455, label %460, label %456

456:                                              ; preds = %451
  %457 = sub i32 %410, %394
  %458 = sext i32 %457 to i64
  %459 = shl nsw i64 %458, %453
  br label %469

460:                                              ; preds = %451
  %461 = lshr i64 %452, %453
  %462 = icmp eq i64 %461, 0
  %463 = add nsw i32 %410, %403
  %464 = sext i32 %463 to i64
  %465 = add nsw i64 %464, -1
  %466 = shl nsw i64 %465, %453
  %467 = select i1 %462, i64 0, i64 %466
  %468 = add i64 %467, %452
  br label %469

469:                                              ; preds = %460, %456, %401, %384
  %470 = phi i64 [ 0, %384 ], [ 1099511627776, %401 ], [ %459, %456 ], [ %468, %460 ]
  switch i32 %0, label %471 [
    i32 0, label %475
    i32 1, label %473
    i32 4, label %474
  ]

471:                                              ; preds = %469
  %472 = select i1 %366, i64 254, i64 31
  br label %475

473:                                              ; preds = %469
  br label %475

474:                                              ; preds = %469
  br label %475

475:                                              ; preds = %474, %473, %471, %469
  %476 = phi i64 [ 126, %469 ], [ %472, %471 ], [ 123, %473 ], [ 7, %474 ]
  %477 = icmp sgt i64 %470, %476
  br i1 %477, label %478, label %483

478:                                              ; preds = %475
  switch i32 %0, label %479 [
    i32 0, label %483
    i32 1, label %481
    i32 4, label %482
  ]

479:                                              ; preds = %478
  %480 = select i1 %366, i64 254, i64 31
  br label %483

481:                                              ; preds = %478
  br label %483

482:                                              ; preds = %478
  br label %483

483:                                              ; preds = %482, %481, %479, %478, %475, %382
  %484 = phi i64 [ %383, %382 ], [ %470, %475 ], [ 126, %478 ], [ %480, %479 ], [ 123, %481 ], [ 7, %482 ]
  %485 = trunc i64 %484 to i32
  %486 = or i32 %377, %485
  %487 = and i32 %1, 8
  %488 = icmp ne i32 %487, 0
  %489 = and i1 %307, %488
  %490 = icmp eq i32 %486, %376
  %491 = select i1 %489, i1 %490, i1 false
  %492 = select i1 %491, i32 0, i32 %486
  br label %493

493:                                              ; preds = %483, %163, %185, %191, %222, %240, %241, %245, %290, %295, %299, %301, %302, %303, %304, %305, %311, %362, %365
  %494 = phi i32 [ %246, %245 ], [ 127, %240 ], [ 0, %241 ], [ %294, %290 ], [ %297, %295 ], [ 127, %299 ], [ 255, %301 ], [ 0, %365 ], [ %226, %222 ], [ %167, %163 ], [ 254, %185 ], [ 0, %191 ], [ 31, %302 ], [ 123, %303 ], [ 7, %304 ], [ 126, %305 ], [ 254, %311 ], [ 254, %362 ], [ %492, %483 ]
  %495 = icmp eq i32 %119, 0
  %496 = or i32 %119, %117
  %497 = icmp eq i32 %496, 0
  %498 = and i1 %136, %497
  %499 = icmp ne i32 %120, 0
  %500 = select i1 %498, i1 %499, i1 false
  br i1 %500, label %501, label %512

501:                                              ; preds = %493, %501
  %502 = phi i32 [ %505, %501 ], [ 0, %493 ]
  %503 = phi i64 [ %504, %501 ], [ %129, %493 ]
  %504 = lshr i64 %503, 1
  %505 = add nuw nsw i32 %502, 1
  %506 = icmp ult i64 %503, 2
  br i1 %506, label %507, label %501, !llvm.loop !20003

507:                                              ; preds = %501
  %508 = add nsw i32 %505, %121
  %509 = icmp sgt i32 %508, %126
  %510 = select i1 %509, i32 %118, i32 0
  %511 = select i1 %509, i64 %129, i64 0
  br label %512

512:                                              ; preds = %507, %493
  %513 = phi i32 [ %118, %493 ], [ %510, %507 ]
  %514 = phi i64 [ %129, %493 ], [ %511, %507 ]
  %515 = icmp eq i32 %134, 255
  %516 = select i1 %515, i32 1, i32 %117
  %517 = sub nsw i32 %121, %134
  %518 = add nsw i32 %517, 127
  br i1 %161, label %519, label %583

519:                                              ; preds = %512
  %520 = icmp eq i32 %516, 0
  %521 = or i32 %119, %516
  %522 = icmp eq i32 %521, 0
  %523 = select i1 %520, i32 254, i32 255
  br i1 %522, label %524, label %849

524:                                              ; preds = %519
  %525 = icmp eq i64 %514, 0
  br i1 %525, label %547, label %526

526:                                              ; preds = %524, %526
  %527 = phi i32 [ %530, %526 ], [ 0, %524 ]
  %528 = phi i64 [ %529, %526 ], [ %514, %524 ]
  %529 = lshr i64 %528, 1
  %530 = add nuw nsw i32 %527, 1
  %531 = icmp ult i64 %528, 2
  br i1 %531, label %532, label %526, !llvm.loop !20003

532:                                              ; preds = %526
  %533 = add nsw i32 %530, %518
  %534 = icmp sgt i32 %533, 127
  br i1 %534, label %535, label %547

535:                                              ; preds = %532, %535
  %536 = phi i32 [ %539, %535 ], [ 0, %532 ]
  %537 = phi i64 [ %538, %535 ], [ %514, %532 ]
  %538 = lshr i64 %537, 1
  %539 = add nuw nsw i32 %536, 1
  %540 = icmp ult i64 %537, 2
  br i1 %540, label %541, label %535, !llvm.loop !20003

541:                                              ; preds = %535
  %542 = add nsw i32 %539, %518
  %543 = icmp eq i32 %542, 128
  %544 = tail call i64 @llvm.ctpop.i64(i64 %514)
  %545 = icmp ult i64 %544, 2
  %546 = select i1 %543, i1 %545, i1 false
  br i1 %546, label %547, label %849

547:                                              ; preds = %541, %532, %524
  %548 = icmp eq i32 %513, 0
  %549 = icmp ne i64 %514, 0
  %550 = and i1 %548, %549
  br i1 %550, label %551, label %849

551:                                              ; preds = %547, %551
  %552 = phi i32 [ %555, %551 ], [ 0, %547 ]
  %553 = phi i64 [ %554, %551 ], [ %514, %547 ]
  %554 = lshr i64 %553, 1
  %555 = add nuw nsw i32 %552, 1
  %556 = icmp ult i64 %553, 2
  br i1 %556, label %557, label %551, !llvm.loop !20003

557:                                              ; preds = %551
  %558 = add nsw i32 %555, %518
  %559 = add i32 %552, %518
  %560 = tail call i64 @llvm.ctpop.i64(i64 %514)
  %561 = icmp ult i64 %560, 2
  %562 = icmp ne i32 %162, 2
  %563 = select i1 %562, i1 true, i1 %561
  br i1 %563, label %564, label %578

564:                                              ; preds = %557
  %565 = icmp ne i32 %162, 0
  %566 = select i1 %565, i1 true, i1 %561
  br i1 %566, label %578, label %567

567:                                              ; preds = %564
  %568 = add nsw i32 %552, -1
  %569 = zext nneg i32 %568 to i64
  %570 = shl i64 3, %569
  %571 = icmp ugt i64 %514, %570
  br i1 %571, label %577, label %572

572:                                              ; preds = %567
  %573 = icmp ne i64 %514, %570
  %574 = and i32 %558, 1
  %575 = icmp eq i32 %574, 0
  %576 = select i1 %573, i1 true, i1 %575
  br i1 %576, label %578, label %577

577:                                              ; preds = %572, %567
  br label %578

578:                                              ; preds = %577, %572, %564, %557
  %579 = phi i32 [ %559, %564 ], [ %558, %557 ], [ %558, %577 ], [ %559, %572 ]
  %580 = tail call i32 @llvm.smin.i32(i32 %579, i32 127)
  %581 = tail call i32 @llvm.smax.i32(i32 %580, i32 -127)
  %582 = add nsw i32 %581, 127
  br label %849

583:                                              ; preds = %512
  switch i32 %162, label %590 [
    i32 2, label %584
    i32 3, label %587
  ]

584:                                              ; preds = %583
  %585 = icmp eq i32 %513, 0
  %586 = select i1 %585, i32 2, i32 1
  br label %590

587:                                              ; preds = %583
  %588 = icmp eq i32 %513, 0
  %589 = select i1 %588, i32 1, i32 2
  br label %590

590:                                              ; preds = %587, %584, %583
  %591 = phi i32 [ %586, %584 ], [ %589, %587 ], [ %162, %583 ]
  %592 = and i32 %1, 4
  %593 = icmp ne i32 %592, 0
  %594 = icmp eq i32 %0, 6
  %595 = icmp eq i32 %516, 0
  br i1 %594, label %596, label %654

596:                                              ; preds = %590
  br i1 %595, label %597, label %849

597:                                              ; preds = %596
  %598 = icmp ne i32 %513, 0
  %599 = select i1 %593, i1 %598, i1 false
  br i1 %599, label %849, label %600

600:                                              ; preds = %597
  br i1 %495, label %603, label %601

601:                                              ; preds = %600
  %602 = select i1 %598, i32 128, i32 127
  br label %849

603:                                              ; preds = %600
  %604 = icmp eq i64 %514, 0
  br i1 %604, label %643, label %605

605:                                              ; preds = %603
  %606 = add nsw i32 %517, 133
  %607 = icmp sgt i32 %517, -134
  br i1 %607, label %608, label %613

608:                                              ; preds = %605
  %609 = icmp ugt i32 %606, 40
  %610 = zext nneg i32 %606 to i64
  %611 = shl i64 %514, %610
  %612 = select i1 %609, i64 4611686018427387904, i64 %611
  br label %643

613:                                              ; preds = %605
  %614 = sub nuw nsw i32 -133, %517
  %615 = icmp ult i32 %606, -63
  %616 = zext nneg i32 %614 to i64
  %617 = lshr i64 %514, %616
  %618 = select i1 %615, i64 0, i64 %617
  %619 = shl nsw i64 -1, %616
  %620 = xor i64 %619, -1
  %621 = select i1 %615, i64 -1, i64 %620
  %622 = and i64 %514, %621
  %623 = icmp eq i64 %622, 0
  %624 = icmp eq i32 %591, 1
  %625 = or i1 %623, %624
  br i1 %625, label %643, label %626

626:                                              ; preds = %613
  %627 = icmp eq i32 %591, 2
  br i1 %627, label %628, label %630

628:                                              ; preds = %626
  %629 = add nuw nsw i64 %618, 1
  br label %643

630:                                              ; preds = %626
  br i1 %615, label %643, label %631

631:                                              ; preds = %630
  %632 = sub nuw nsw i32 -134, %517
  %633 = zext nneg i32 %632 to i64
  %634 = shl nuw nsw i64 1, %633
  %635 = icmp ugt i64 %622, %634
  %636 = icmp eq i64 %622, %634
  %637 = and i64 %617, 1
  %638 = icmp ne i64 %637, 0
  %639 = select i1 %636, i1 %638, i1 false
  %640 = select i1 %635, i1 true, i1 %639
  %641 = zext i1 %640 to i64
  %642 = add nuw nsw i64 %617, %641
  br label %643

643:                                              ; preds = %631, %630, %628, %613, %608, %603
  %644 = phi i64 [ 0, %603 ], [ %612, %608 ], [ %629, %628 ], [ %642, %631 ], [ %618, %613 ], [ 0, %630 ]
  %645 = icmp eq i32 %513, 0
  br i1 %645, label %651, label %646

646:                                              ; preds = %643
  %647 = tail call i64 @llvm.umin.i64(i64 %644, i64 128)
  %648 = trunc i64 %647 to i32
  %649 = sub nsw i32 0, %648
  %650 = and i32 %649, 255
  br label %849

651:                                              ; preds = %643
  %652 = tail call i64 @llvm.umin.i64(i64 %644, i64 127)
  %653 = trunc i64 %652 to i32
  br label %849

654:                                              ; preds = %590
  br i1 %595, label %662, label %655

655:                                              ; preds = %654
  %656 = icmp ult i32 %0, 2
  br i1 %656, label %849, label %657

657:                                              ; preds = %655
  switch i32 %0, label %658 [
    i32 5, label %849
    i32 0, label %661
    i32 1, label %659
    i32 4, label %660
  ]

658:                                              ; preds = %657
  br label %849

659:                                              ; preds = %657
  br label %849

660:                                              ; preds = %657
  br label %849

661:                                              ; preds = %657
  br label %849

662:                                              ; preds = %654
  %663 = icmp ne i32 %0, 5
  %664 = or i1 %663, %593
  %665 = icmp eq i32 %513, 0
  %666 = select i1 %664, i1 true, i1 %665
  br i1 %666, label %721, label %667

667:                                              ; preds = %662
  br i1 %495, label %668, label %849

668:                                              ; preds = %667
  %669 = icmp eq i64 %514, 0
  br i1 %669, label %721, label %670

670:                                              ; preds = %668, %670
  %671 = phi i32 [ %674, %670 ], [ 0, %668 ]
  %672 = phi i64 [ %673, %670 ], [ %514, %668 ]
  %673 = lshr i64 %672, 1
  %674 = add nuw nsw i32 %671, 1
  %675 = icmp ult i64 %672, 2
  br i1 %675, label %676, label %670, !llvm.loop !20003

676:                                              ; preds = %670
  %677 = add nsw i32 %674, %518
  %678 = add i32 %671, %518
  %679 = icmp sgt i32 %677, -14
  %680 = select i1 %679, i32 %678, i32 -14
  %681 = icmp sgt i32 %680, 200
  br i1 %681, label %718, label %682

682:                                              ; preds = %676
  %683 = sub i32 %518, %680
  %684 = add i32 %683, 3
  %685 = icmp sgt i32 %684, -1
  br i1 %685, label %686, label %691

686:                                              ; preds = %682
  %687 = icmp ugt i32 %684, 40
  %688 = zext nneg i32 %684 to i64
  %689 = shl i64 %514, %688
  %690 = select i1 %687, i64 4611686018427387904, i64 %689
  br label %704

691:                                              ; preds = %682
  %692 = sub nuw i32 -3, %683
  %693 = icmp ult i32 %684, -63
  %694 = zext nneg i32 %692 to i64
  %695 = lshr i64 %514, %694
  %696 = select i1 %693, i64 0, i64 %695
  %697 = shl nsw i64 -1, %694
  %698 = xor i64 %697, -1
  %699 = select i1 %693, i64 -1, i64 %698
  %700 = and i64 %699, %514
  %701 = icmp ne i64 %700, 0
  %702 = zext i1 %701 to i64
  %703 = add nuw nsw i64 %696, %702
  br label %704

704:                                              ; preds = %691, %686
  %705 = phi i64 [ %690, %686 ], [ %703, %691 ]
  %706 = icmp ult i64 %705, 16
  br i1 %706, label %711, label %707

707:                                              ; preds = %704
  %708 = add nsw i32 %680, 16
  %709 = sext i32 %708 to i64
  %710 = shl nsw i64 %709, 3
  br label %718

711:                                              ; preds = %704
  %712 = icmp ult i64 %705, 8
  %713 = sext i32 %680 to i64
  %714 = shl nsw i64 %713, 3
  %715 = add nsw i64 %714, 112
  %716 = select i1 %712, i64 0, i64 %715
  %717 = add i64 %716, %705
  br label %718

718:                                              ; preds = %711, %707, %676
  %719 = phi i64 [ 1099511627776, %676 ], [ %710, %707 ], [ %717, %711 ]
  %720 = icmp sgt i64 %719, 254
  br i1 %720, label %849, label %721

721:                                              ; preds = %718, %668, %662
  %722 = icmp eq i32 %0, 5
  %723 = or i1 %722, %593
  %724 = icmp ne i32 %513, 0
  %725 = select i1 %723, i1 %724, i1 false
  br i1 %725, label %849, label %726

726:                                              ; preds = %721
  %727 = and i1 %663, %724
  %728 = and i32 %0, -2
  %729 = icmp eq i32 %728, 2
  %730 = icmp eq i32 %0, 4
  %731 = select i1 %730, i32 8, i32 128
  %732 = select i1 %729, i32 32, i32 %731
  %733 = select i1 %727, i32 %732, i32 0
  switch i32 %0, label %734 [
    i32 0, label %738
    i32 1, label %736
    i32 4, label %737
  ]

734:                                              ; preds = %726
  %735 = select i1 %722, i64 254, i64 31
  br label %738

736:                                              ; preds = %726
  br label %738

737:                                              ; preds = %726
  br label %738

738:                                              ; preds = %737, %736, %734, %726
  %739 = phi i64 [ 126, %726 ], [ %735, %734 ], [ 123, %736 ], [ 7, %737 ]
  br i1 %495, label %740, label %839

740:                                              ; preds = %738
  %741 = icmp eq i64 %514, 0
  br i1 %741, label %825, label %742

742:                                              ; preds = %740
  switch i32 %0, label %743 [
    i32 0, label %747
    i32 4, label %746
    i32 2, label %746
  ]

743:                                              ; preds = %742
  %744 = icmp eq i32 %0, 3
  %745 = select i1 %744, i32 2, i32 4
  br label %747

746:                                              ; preds = %742, %742
  br label %747

747:                                              ; preds = %746, %743, %742
  %748 = phi i32 [ 3, %742 ], [ %745, %743 ], [ 1, %746 ]
  %749 = and i32 %0, -3
  %750 = shl nsw i32 -1, %748
  br label %751

751:                                              ; preds = %751, %747
  %752 = phi i32 [ %755, %751 ], [ 0, %747 ]
  %753 = phi i64 [ %754, %751 ], [ %514, %747 ]
  %754 = lshr i64 %753, 1
  %755 = add nuw nsw i32 %752, 1
  %756 = icmp ult i64 %753, 2
  br i1 %756, label %757, label %751, !llvm.loop !20003

757:                                              ; preds = %751
  %758 = select i1 %730, i32 1, i32 3
  %759 = xor i32 %750, -1
  %760 = icmp eq i32 %749, 1
  %761 = select i1 %760, i32 2, i32 %758
  %762 = add nsw i32 %750, 2
  %763 = add nsw i32 %755, %518
  %764 = add i32 %752, %518
  %765 = icmp sgt i32 %763, %762
  %766 = select i1 %765, i32 %764, i32 %762
  %767 = icmp sgt i32 %766, 200
  br i1 %767, label %825, label %768

768:                                              ; preds = %757
  %769 = sub i32 %761, %766
  %770 = add i32 %769, %518
  %771 = icmp sgt i32 %770, -1
  br i1 %771, label %772, label %777

772:                                              ; preds = %768
  %773 = icmp ugt i32 %770, 40
  %774 = zext nneg i32 %770 to i64
  %775 = shl i64 %514, %774
  %776 = select i1 %773, i64 4611686018427387904, i64 %775
  br label %807

777:                                              ; preds = %768
  %778 = sub nsw i32 0, %770
  %779 = icmp ult i32 %770, -63
  %780 = zext nneg i32 %778 to i64
  %781 = lshr i64 %514, %780
  %782 = select i1 %779, i64 0, i64 %781
  %783 = shl nsw i64 -1, %780
  %784 = xor i64 %783, -1
  %785 = select i1 %779, i64 -1, i64 %784
  %786 = and i64 %785, %514
  %787 = icmp eq i64 %786, 0
  %788 = icmp eq i32 %591, 1
  %789 = or i1 %788, %787
  br i1 %789, label %807, label %790

790:                                              ; preds = %777
  %791 = icmp eq i32 %591, 2
  br i1 %791, label %792, label %794

792:                                              ; preds = %790
  %793 = add nuw nsw i64 %782, 1
  br label %807

794:                                              ; preds = %790
  br i1 %779, label %807, label %795

795:                                              ; preds = %794
  %796 = xor i32 %770, -1
  %797 = zext nneg i32 %796 to i64
  %798 = shl nuw nsw i64 1, %797
  %799 = icmp ugt i64 %786, %798
  %800 = icmp eq i64 %786, %798
  %801 = and i64 %781, 1
  %802 = icmp ne i64 %801, 0
  %803 = select i1 %800, i1 %802, i1 false
  %804 = select i1 %799, i1 true, i1 %803
  %805 = zext i1 %804 to i64
  %806 = add nuw nsw i64 %781, %805
  br label %807

807:                                              ; preds = %795, %794, %792, %777, %772
  %808 = phi i64 [ %776, %772 ], [ %793, %792 ], [ %806, %795 ], [ %782, %777 ], [ 0, %794 ]
  %809 = zext nneg i32 %761 to i64
  %810 = shl nuw nsw i64 2, %809
  %811 = icmp ult i64 %808, %810
  br i1 %811, label %816, label %812

812:                                              ; preds = %807
  %813 = sub i32 %766, %750
  %814 = sext i32 %813 to i64
  %815 = shl nsw i64 %814, %809
  br label %825

816:                                              ; preds = %807
  %817 = lshr i64 %808, %809
  %818 = icmp eq i64 %817, 0
  %819 = add nsw i32 %766, %759
  %820 = sext i32 %819 to i64
  %821 = add nsw i64 %820, -1
  %822 = shl nsw i64 %821, %809
  %823 = select i1 %818, i64 0, i64 %822
  %824 = add i64 %823, %808
  br label %825

825:                                              ; preds = %816, %812, %757, %740
  %826 = phi i64 [ 0, %740 ], [ 1099511627776, %757 ], [ %815, %812 ], [ %824, %816 ]
  switch i32 %0, label %827 [
    i32 0, label %831
    i32 1, label %829
    i32 4, label %830
  ]

827:                                              ; preds = %825
  %828 = select i1 %722, i64 254, i64 31
  br label %831

829:                                              ; preds = %825
  br label %831

830:                                              ; preds = %825
  br label %831

831:                                              ; preds = %830, %829, %827, %825
  %832 = phi i64 [ 126, %825 ], [ %828, %827 ], [ 123, %829 ], [ 7, %830 ]
  %833 = icmp sgt i64 %826, %832
  br i1 %833, label %834, label %839

834:                                              ; preds = %831
  switch i32 %0, label %835 [
    i32 0, label %839
    i32 1, label %837
    i32 4, label %838
  ]

835:                                              ; preds = %834
  %836 = select i1 %722, i64 254, i64 31
  br label %839

837:                                              ; preds = %834
  br label %839

838:                                              ; preds = %834
  br label %839

839:                                              ; preds = %838, %837, %835, %834, %831, %738
  %840 = phi i64 [ %739, %738 ], [ %826, %831 ], [ 126, %834 ], [ %836, %835 ], [ 123, %837 ], [ 7, %838 ]
  %841 = trunc i64 %840 to i32
  %842 = or i32 %733, %841
  %843 = and i32 %1, 8
  %844 = icmp ne i32 %843, 0
  %845 = and i1 %663, %844
  %846 = icmp eq i32 %842, %732
  %847 = select i1 %845, i1 %846, i1 false
  %848 = select i1 %847, i32 0, i32 %842
  br label %849

849:                                              ; preds = %839, %519, %541, %547, %578, %596, %597, %601, %646, %651, %655, %657, %658, %659, %660, %661, %667, %718, %721
  %850 = phi i32 [ %602, %601 ], [ 127, %596 ], [ 0, %597 ], [ %650, %646 ], [ %653, %651 ], [ 127, %655 ], [ 255, %657 ], [ 0, %721 ], [ %582, %578 ], [ %523, %519 ], [ 254, %541 ], [ 0, %547 ], [ 31, %658 ], [ 123, %659 ], [ 7, %660 ], [ 126, %661 ], [ 254, %667 ], [ 254, %718 ], [ %848, %839 ]
  %851 = icmp eq i32 %0, 4
  %852 = select i1 %851, i32 4, i32 8
  %853 = shl i32 %494, %852
  %854 = or i32 %850, %853
  %855 = trunc i32 %854 to i16
  ret i16 %855
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)

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
  %19 = and i32 %2, 4
  %20 = icmp eq i32 %0, 5
  %21 = and i32 %0, -2
  %22 = icmp eq i32 %21, 2
  %23 = select i1 %6, i32 3, i32 7
  %24 = select i1 %22, i32 5, i32 %23
  %25 = shl nsw i32 -1, %24
  %26 = xor i32 %25, -1
  %27 = icmp eq i32 %0, 0
  %28 = icmp eq i32 %0, 1
  %29 = and i32 %0, -3
  %30 = icmp eq i32 %29, 1
  %31 = select i1 %6, i32 1, i32 3
  %32 = select i1 %30, i32 2, i32 %31
  %33 = icmp eq i32 %0, 3
  %34 = select i1 %33, i32 2, i32 4
  %35 = shl nuw nsw i32 1, %32
  %36 = add nsw i32 %35, -1
  %37 = icmp eq i32 %19, 0
  %38 = select i1 %13, i32 32256, i32 32704
  %39 = select i1 %37, i32 %38, i32 32767
  %40 = icmp ne i32 %19, 0
  %41 = shl i32 %2, 25
  %42 = ashr i32 %41, 31
  %43 = add nsw i32 %18, %42
  %44 = add nsw i32 %14, -1
  %45 = shl nsw i32 -1, %44
  %46 = xor i32 %45, -1
  %47 = add nuw nsw i32 %45, 2
  %48 = zext nneg i32 %15 to i64
  %49 = shl nuw nsw i64 2, %48
  %50 = zext nneg i32 %18 to i64
  %51 = select i1 %20, i32 -1, i32 %26
  br label %53

52:                                               ; preds = %190
  ret i32 %194

53:                                               ; preds = %5, %190
  %54 = phi i32 [ 0, %5 ], [ %194, %190 ]
  %55 = phi i1 [ true, %5 ], [ false, %190 ]
  %56 = phi i32 [ 0, %5 ], [ 1, %190 ]
  %57 = shl nuw nsw i32 %56, %7
  %58 = lshr i32 %3, %57
  %59 = and i32 %58, %8
  br i1 %9, label %60, label %66

60:                                               ; preds = %53
  %61 = shl nuw nsw i32 %59, 1
  %62 = and i32 %61, 256
  %63 = sub nsw i32 %59, %62
  %64 = lshr i32 %63, 31
  %65 = tail call i32 @llvm.abs.i32(i32 %63, i1 true)
  br label %99

66:                                               ; preds = %53
  %67 = and i32 %59, %51
  %68 = lshr i32 %59, %24
  %69 = and i32 %68, 1
  %70 = select i1 %20, i32 0, i32 %69
  %71 = icmp eq i32 %67, 127
  %72 = select i1 %27, i1 %71, i1 false
  br i1 %72, label %99, label %73

73:                                               ; preds = %66
  %74 = icmp ugt i32 %67, 124
  %75 = select i1 %28, i1 %74, i1 false
  %76 = icmp eq i32 %67, 255
  %77 = select i1 %20, i1 %76, i1 false
  %78 = select i1 %75, i1 true, i1 %77
  br i1 %78, label %99, label %79

79:                                               ; preds = %73
  %80 = icmp eq i32 %67, 124
  %81 = select i1 %28, i1 %80, i1 false
  br i1 %81, label %99, label %82

82:                                               ; preds = %79
  switch i32 %0, label %83 [
    i32 0, label %85
    i32 4, label %84
    i32 2, label %84
  ]

83:                                               ; preds = %82
  br label %85

84:                                               ; preds = %82, %82
  br label %85

85:                                               ; preds = %84, %83, %82
  %86 = phi i32 [ 3, %82 ], [ %34, %83 ], [ 1, %84 ]
  %87 = shl nsw i32 -1, %86
  %88 = lshr i32 %67, %32
  %89 = and i32 %67, %36
  %90 = icmp eq i32 %88, 0
  br i1 %90, label %91, label %94

91:                                               ; preds = %85
  %92 = sub nuw nsw i32 %87, %32
  %93 = add nuw nsw i32 %92, 2
  br label %99

94:                                               ; preds = %85
  %95 = or i32 %89, %35
  %96 = sub nsw i32 %88, %32
  %97 = add nsw i32 %96, 1
  %98 = add nsw i32 %97, %87
  br label %99

99:                                               ; preds = %60, %66, %73, %79, %91, %94
  %100 = phi i32 [ -6, %60 ], [ 0, %73 ], [ 0, %66 ], [ 0, %79 ], [ %93, %91 ], [ %98, %94 ]
  %101 = phi i32 [ %65, %60 ], [ 0, %73 ], [ 0, %66 ], [ 0, %79 ], [ %89, %91 ], [ %95, %94 ]
  %102 = phi i1 [ true, %60 ], [ true, %73 ], [ true, %66 ], [ false, %79 ], [ true, %91 ], [ true, %94 ]
  %103 = phi i1 [ true, %60 ], [ false, %73 ], [ false, %66 ], [ true, %79 ], [ true, %91 ], [ true, %94 ]
  %104 = phi i32 [ %64, %60 ], [ %70, %73 ], [ %70, %66 ], [ %70, %79 ], [ %70, %91 ], [ %70, %94 ]
  %105 = zext nneg i32 %101 to i64
  %106 = or i1 %11, %55
  %107 = select i1 %106, i32 %4, i32 %12
  %108 = and i32 %107, 255
  %109 = icmp ne i32 %108, 255
  %110 = add nsw i32 %108, -127
  %111 = add nsw i32 %110, %100
  %112 = and i1 %109, %103
  br i1 %112, label %113, label %190

113:                                              ; preds = %99
  %114 = icmp ne i32 %104, 0
  %115 = select i1 %40, i1 %114, i1 false
  br i1 %115, label %190, label %116

116:                                              ; preds = %113
  %117 = select i1 %114, i32 32768, i32 0
  br i1 %102, label %120, label %118

118:                                              ; preds = %116
  %119 = or i32 %117, %43
  br label %190

120:                                              ; preds = %116
  %121 = icmp eq i32 %101, 0
  br i1 %121, label %190, label %122

122:                                              ; preds = %120, %122
  %123 = phi i32 [ %126, %122 ], [ 0, %120 ]
  %124 = phi i64 [ %125, %122 ], [ %105, %120 ]
  %125 = lshr i64 %124, 1
  %126 = add nuw nsw i32 %123, 1
  %127 = icmp ult i64 %124, 2
  br i1 %127, label %128, label %122, !llvm.loop !20003

128:                                              ; preds = %122
  %129 = add nsw i32 %126, %111
  %130 = add i32 %123, %111
  %131 = icmp sgt i32 %129, %47
  %132 = select i1 %131, i32 %130, i32 %47
  %133 = icmp sgt i32 %132, 200
  br i1 %133, label %184, label %134

134:                                              ; preds = %128
  %135 = sub i32 %15, %132
  %136 = add i32 %135, %111
  %137 = icmp sgt i32 %136, -1
  br i1 %137, label %138, label %143

138:                                              ; preds = %134
  %139 = icmp ugt i32 %136, 40
  %140 = zext nneg i32 %136 to i64
  %141 = shl i64 %105, %140
  %142 = select i1 %139, i64 4611686018427387904, i64 %141
  br label %168

143:                                              ; preds = %134
  %144 = sub nsw i32 0, %136
  %145 = icmp ult i32 %136, -63
  %146 = zext nneg i32 %144 to i64
  %147 = lshr i64 %105, %146
  %148 = select i1 %145, i64 0, i64 %147
  %149 = shl nsw i64 -1, %146
  %150 = xor i64 %149, -1
  %151 = select i1 %145, i64 2147483647, i64 %150
  %152 = and i64 %151, %105
  %153 = icmp eq i64 %152, 0
  %154 = or i1 %145, %153
  %155 = select i1 %153, i64 %148, i64 0
  br i1 %154, label %168, label %156

156:                                              ; preds = %143
  %157 = xor i32 %136, -1
  %158 = zext nneg i32 %157 to i64
  %159 = shl nuw nsw i64 1, %158
  %160 = icmp ugt i64 %152, %159
  %161 = icmp eq i64 %152, %159
  %162 = and i64 %147, 1
  %163 = icmp ne i64 %162, 0
  %164 = select i1 %161, i1 %163, i1 false
  %165 = select i1 %160, i1 true, i1 %164
  %166 = zext i1 %165 to i64
  %167 = add nuw nsw i64 %147, %166
  br label %168

168:                                              ; preds = %156, %143, %138
  %169 = phi i64 [ %142, %138 ], [ %167, %156 ], [ %155, %143 ]
  %170 = icmp ult i64 %169, %49
  br i1 %170, label %175, label %171

171:                                              ; preds = %168
  %172 = sub i32 %132, %45
  %173 = sext i32 %172 to i64
  %174 = shl nsw i64 %173, %48
  br label %184

175:                                              ; preds = %168
  %176 = lshr i64 %169, %48
  %177 = icmp eq i64 %176, 0
  %178 = add nsw i32 %132, %46
  %179 = sext i32 %178 to i64
  %180 = add nsw i64 %179, -1
  %181 = shl nsw i64 %180, %48
  %182 = select i1 %177, i64 0, i64 %181
  %183 = add i64 %182, %169
  br label %184

184:                                              ; preds = %175, %171, %128
  %185 = phi i64 [ 1099511627776, %128 ], [ %174, %171 ], [ %183, %175 ]
  %186 = icmp slt i64 %185, %50
  %187 = trunc i64 %185 to i32
  %188 = select i1 %186, i32 %187, i32 %43
  %189 = or i32 %188, %117
  br label %190

190:                                              ; preds = %99, %113, %118, %120, %184
  %191 = phi i32 [ 0, %113 ], [ %119, %118 ], [ %189, %184 ], [ %117, %120 ], [ %39, %99 ]
  %192 = shl nuw nsw i32 %56, 4
  %193 = shl i32 %191, %192
  %194 = or i32 %193, %54
  br i1 %55, label %53, label %52, !llvm.loop !20006
}

; Function Attrs: nofree norecurse nosync nounwind memory(none)
define dso_local i32 @__zluda_ptx_impl_agx_cvt_pzo(i32 noundef %0, i32 noundef %1, i32 noundef %2, i32 noundef %3) local_unnamed_addr #3 {
  %5 = lshr i32 %2, 23
  %6 = and i32 %2, 8388607
  %7 = trunc i32 %5 to i8
  switch i8 %7, label %13 [
    i8 -1, label %8
    i8 0, label %17
  ]

8:                                                ; preds = %4
  %9 = icmp ne i32 %6, 0
  %10 = zext i1 %9 to i32
  %11 = icmp eq i32 %6, 0
  %12 = zext i1 %11 to i32
  br label %17

13:                                               ; preds = %4
  %14 = and i32 %5, 255
  %15 = or disjoint i32 %6, 8388608
  %16 = add nsw i32 %14, -150
  br label %17

17:                                               ; preds = %4, %8, %13
  %18 = phi i32 [ 0, %8 ], [ %16, %13 ], [ -149, %4 ]
  %19 = phi i32 [ 0, %8 ], [ %15, %13 ], [ %6, %4 ]
  %20 = phi i32 [ %12, %8 ], [ 0, %13 ], [ 0, %4 ]
  %21 = phi i32 [ %10, %8 ], [ 0, %13 ], [ 0, %4 ]
  %22 = zext nneg i32 %19 to i64
  %23 = icmp eq i32 %0, 2
  br i1 %23, label %24, label %108

24:                                               ; preds = %17
  %25 = icmp eq i32 %21, 0
  br i1 %25, label %26, label %436

26:                                               ; preds = %24
  %27 = and i32 %1, 4
  %28 = icmp ne i32 %27, 0
  %29 = icmp slt i32 %2, 0
  %30 = and i1 %28, %29
  br i1 %30, label %436, label %31

31:                                               ; preds = %26
  %32 = and i32 %2, -2147483648
  %33 = and i32 %1, 64
  %34 = icmp eq i32 %33, 0
  %35 = select i1 %34, i32 261120, i32 261119
  %36 = icmp eq i32 %20, 0
  br i1 %36, label %37, label %99

37:                                               ; preds = %31
  %38 = icmp eq i32 %19, 0
  br i1 %38, label %99, label %39

39:                                               ; preds = %37, %39
  %40 = phi i32 [ %43, %39 ], [ 0, %37 ]
  %41 = phi i64 [ %42, %39 ], [ %22, %37 ]
  %42 = lshr i64 %41, 1
  %43 = add nuw nsw i32 %40, 1
  %44 = icmp ult i64 %41, 2
  br i1 %44, label %45, label %39, !llvm.loop !20003

45:                                               ; preds = %39
  %46 = add nsw i32 %43, %18
  %47 = add i32 %40, %18
  %48 = icmp sgt i32 %46, -126
  %49 = select i1 %48, i32 %47, i32 -126
  %50 = icmp sgt i32 %49, 200
  br i1 %50, label %99, label %51

51:                                               ; preds = %45
  %52 = sub i32 %18, %49
  %53 = add i32 %52, 10
  %54 = icmp sgt i32 %53, -1
  br i1 %54, label %55, label %60

55:                                               ; preds = %51
  %56 = icmp ugt i32 %53, 40
  %57 = zext nneg i32 %53 to i64
  %58 = shl i64 %22, %57
  %59 = select i1 %56, i64 4611686018427387904, i64 %58
  br label %85

60:                                               ; preds = %51
  %61 = sub nuw i32 -10, %52
  %62 = icmp ult i32 %53, -63
  %63 = zext nneg i32 %61 to i64
  %64 = lshr i64 %22, %63
  %65 = select i1 %62, i64 0, i64 %64
  %66 = shl nsw i64 -1, %63
  %67 = xor i64 %66, -1
  %68 = select i1 %62, i64 2147483647, i64 %67
  %69 = and i64 %68, %22
  %70 = icmp eq i64 %69, 0
  %71 = or i1 %62, %70
  %72 = select i1 %70, i64 %65, i64 0
  br i1 %71, label %85, label %73

73:                                               ; preds = %60
  %74 = sub nuw nsw i32 -11, %52
  %75 = zext nneg i32 %74 to i64
  %76 = shl nuw nsw i64 1, %75
  %77 = icmp ugt i64 %69, %76
  %78 = icmp eq i64 %69, %76
  %79 = and i64 %64, 1
  %80 = icmp ne i64 %79, 0
  %81 = select i1 %78, i1 %80, i1 false
  %82 = select i1 %77, i1 true, i1 %81
  %83 = zext i1 %82 to i64
  %84 = add nuw nsw i64 %64, %83
  br label %85

85:                                               ; preds = %60, %73, %55
  %86 = phi i64 [ %59, %55 ], [ %84, %73 ], [ %72, %60 ]
  %87 = icmp ult i64 %86, 2048
  br i1 %87, label %92, label %88

88:                                               ; preds = %85
  %89 = add nsw i32 %49, 128
  %90 = sext i32 %89 to i64
  %91 = shl nsw i64 %90, 10
  br label %99

92:                                               ; preds = %85
  %93 = icmp ult i64 %86, 1024
  %94 = sext i32 %49 to i64
  %95 = shl nsw i64 %94, 10
  %96 = add nsw i64 %95, 129024
  %97 = select i1 %93, i64 0, i64 %96
  %98 = add i64 %97, %86
  br label %99

99:                                               ; preds = %92, %88, %45, %31, %37
  %100 = phi i64 [ 0, %37 ], [ 261120, %31 ], [ 1099511627776, %45 ], [ %91, %88 ], [ %98, %92 ]
  %101 = icmp sgt i64 %100, 261119
  %102 = trunc i64 %100 to i32
  %103 = select i1 %101, i32 %35, i32 %102
  %104 = shl i32 %103, 13
  %105 = or i32 %104, %32
  %106 = icmp eq i32 %105, -2147483648
  %107 = select i1 %106, i32 0, i32 %105
  br label %436

108:                                              ; preds = %17
  %109 = icmp ult i32 %0, 2
  br i1 %109, label %110, label %214

110:                                              ; preds = %108
  %111 = icmp eq i32 %21, 0
  br i1 %111, label %112, label %210

112:                                              ; preds = %110
  %113 = icmp eq i32 %0, 0
  %114 = select i1 %113, i32 5, i32 8
  %115 = select i1 %113, i32 10, i32 7
  %116 = shl nsw i32 -1, %114
  %117 = xor i32 %116, -1
  %118 = shl nuw nsw i32 %117, %115
  %119 = and i32 %1, 4
  %120 = icmp ne i32 %119, 0
  %121 = icmp slt i32 %2, 0
  %122 = and i1 %120, %121
  br i1 %122, label %210, label %123

123:                                              ; preds = %112
  %124 = lshr i32 %2, 16
  %125 = and i32 %124, 32768
  %126 = shl i32 %1, 25
  %127 = ashr i32 %126, 31
  %128 = add nsw i32 %118, %127
  %129 = icmp eq i32 %20, 0
  br i1 %129, label %132, label %130

130:                                              ; preds = %123
  %131 = or i32 %128, %125
  br label %210

132:                                              ; preds = %123
  %133 = icmp eq i32 %19, 0
  br i1 %133, label %210, label %134

134:                                              ; preds = %132
  %135 = add nsw i32 %114, -1
  br label %136

136:                                              ; preds = %136, %134
  %137 = phi i32 [ %140, %136 ], [ 0, %134 ]
  %138 = phi i64 [ %139, %136 ], [ %22, %134 ]
  %139 = lshr i64 %138, 1
  %140 = add nuw nsw i32 %137, 1
  %141 = icmp ult i64 %138, 2
  br i1 %141, label %142, label %136, !llvm.loop !20003

142:                                              ; preds = %136
  %143 = shl nsw i32 -1, %135
  %144 = xor i32 %143, -1
  %145 = add nuw nsw i32 %143, 2
  %146 = add nsw i32 %140, %18
  %147 = add i32 %137, %18
  %148 = icmp sgt i32 %146, %145
  %149 = select i1 %148, i32 %147, i32 %145
  %150 = icmp sgt i32 %149, 200
  br i1 %150, label %203, label %151

151:                                              ; preds = %142
  %152 = sub i32 %115, %149
  %153 = add i32 %152, %18
  %154 = icmp sgt i32 %153, -1
  br i1 %154, label %155, label %160

155:                                              ; preds = %151
  %156 = icmp ugt i32 %153, 40
  %157 = zext nneg i32 %153 to i64
  %158 = shl i64 %22, %157
  %159 = select i1 %156, i64 4611686018427387904, i64 %158
  br label %185

160:                                              ; preds = %151
  %161 = sub nsw i32 0, %153
  %162 = icmp ult i32 %153, -63
  %163 = zext nneg i32 %161 to i64
  %164 = lshr i64 %22, %163
  %165 = select i1 %162, i64 0, i64 %164
  %166 = shl nsw i64 -1, %163
  %167 = xor i64 %166, -1
  %168 = select i1 %162, i64 2147483647, i64 %167
  %169 = and i64 %168, %22
  %170 = icmp eq i64 %169, 0
  %171 = or i1 %162, %170
  %172 = select i1 %170, i64 %165, i64 0
  br i1 %171, label %185, label %173

173:                                              ; preds = %160
  %174 = xor i32 %153, -1
  %175 = zext nneg i32 %174 to i64
  %176 = shl nuw nsw i64 1, %175
  %177 = icmp ugt i64 %169, %176
  %178 = icmp eq i64 %169, %176
  %179 = and i64 %164, 1
  %180 = icmp ne i64 %179, 0
  %181 = select i1 %178, i1 %180, i1 false
  %182 = select i1 %177, i1 true, i1 %181
  %183 = zext i1 %182 to i64
  %184 = add nuw nsw i64 %164, %183
  br label %185

185:                                              ; preds = %173, %160, %155
  %186 = phi i64 [ %159, %155 ], [ %184, %173 ], [ %172, %160 ]
  %187 = zext nneg i32 %115 to i64
  %188 = shl nuw nsw i64 2, %187
  %189 = icmp ult i64 %186, %188
  br i1 %189, label %194, label %190

190:                                              ; preds = %185
  %191 = sub i32 %149, %143
  %192 = sext i32 %191 to i64
  %193 = shl nsw i64 %192, %187
  br label %203

194:                                              ; preds = %185
  %195 = lshr i64 %186, %187
  %196 = icmp eq i64 %195, 0
  %197 = add nsw i32 %149, %144
  %198 = sext i32 %197 to i64
  %199 = add nsw i64 %198, -1
  %200 = shl nsw i64 %199, %187
  %201 = select i1 %196, i64 0, i64 %200
  %202 = add i64 %201, %186
  br label %203

203:                                              ; preds = %194, %190, %142
  %204 = phi i64 [ 1099511627776, %142 ], [ %193, %190 ], [ %202, %194 ]
  %205 = zext nneg i32 %118 to i64
  %206 = icmp slt i64 %204, %205
  %207 = trunc i64 %204 to i32
  %208 = select i1 %206, i32 %207, i32 %128
  %209 = or i32 %208, %125
  br label %210

210:                                              ; preds = %110, %112, %130, %132, %203
  %211 = phi i32 [ 32767, %110 ], [ 0, %112 ], [ %131, %130 ], [ %209, %203 ], [ %125, %132 ]
  %212 = icmp eq i32 %211, 32768
  %213 = select i1 %212, i32 0, i32 %211
  br label %436

214:                                              ; preds = %108
  %215 = icmp eq i32 %0, 4
  %216 = icmp eq i32 %21, 0
  br i1 %216, label %217, label %314

217:                                              ; preds = %214
  %218 = select i1 %215, i32 8, i32 5
  %219 = select i1 %215, i32 7, i32 10
  %220 = shl nsw i32 -1, %218
  %221 = xor i32 %220, -1
  %222 = shl nuw nsw i32 %221, %219
  %223 = and i32 %1, 4
  %224 = icmp ne i32 %223, 0
  %225 = icmp slt i32 %2, 0
  %226 = and i1 %224, %225
  br i1 %226, label %314, label %227

227:                                              ; preds = %217
  %228 = lshr i32 %2, 16
  %229 = and i32 %228, 32768
  %230 = shl i32 %1, 25
  %231 = ashr i32 %230, 31
  %232 = add nsw i32 %222, %231
  %233 = icmp eq i32 %20, 0
  br i1 %233, label %236, label %234

234:                                              ; preds = %227
  %235 = or i32 %232, %229
  br label %314

236:                                              ; preds = %227
  %237 = icmp eq i32 %19, 0
  br i1 %237, label %314, label %238

238:                                              ; preds = %236
  %239 = add nsw i32 %218, -1
  br label %240

240:                                              ; preds = %240, %238
  %241 = phi i32 [ %244, %240 ], [ 0, %238 ]
  %242 = phi i64 [ %243, %240 ], [ %22, %238 ]
  %243 = lshr i64 %242, 1
  %244 = add nuw nsw i32 %241, 1
  %245 = icmp ult i64 %242, 2
  br i1 %245, label %246, label %240, !llvm.loop !20003

246:                                              ; preds = %240
  %247 = shl nsw i32 -1, %239
  %248 = xor i32 %247, -1
  %249 = add nuw nsw i32 %247, 2
  %250 = add nsw i32 %244, %18
  %251 = add i32 %241, %18
  %252 = icmp sgt i32 %250, %249
  %253 = select i1 %252, i32 %251, i32 %249
  %254 = icmp sgt i32 %253, 200
  br i1 %254, label %307, label %255

255:                                              ; preds = %246
  %256 = sub i32 %219, %253
  %257 = add i32 %256, %18
  %258 = icmp sgt i32 %257, -1
  br i1 %258, label %259, label %264

259:                                              ; preds = %255
  %260 = icmp ugt i32 %257, 40
  %261 = zext nneg i32 %257 to i64
  %262 = shl i64 %22, %261
  %263 = select i1 %260, i64 4611686018427387904, i64 %262
  br label %289

264:                                              ; preds = %255
  %265 = sub nsw i32 0, %257
  %266 = icmp ult i32 %257, -63
  %267 = zext nneg i32 %265 to i64
  %268 = lshr i64 %22, %267
  %269 = select i1 %266, i64 0, i64 %268
  %270 = shl nsw i64 -1, %267
  %271 = xor i64 %270, -1
  %272 = select i1 %266, i64 2147483647, i64 %271
  %273 = and i64 %272, %22
  %274 = icmp eq i64 %273, 0
  %275 = or i1 %266, %274
  %276 = select i1 %274, i64 %269, i64 0
  br i1 %275, label %289, label %277

277:                                              ; preds = %264
  %278 = xor i32 %257, -1
  %279 = zext nneg i32 %278 to i64
  %280 = shl nuw nsw i64 1, %279
  %281 = icmp ugt i64 %273, %280
  %282 = icmp eq i64 %273, %280
  %283 = and i64 %268, 1
  %284 = icmp ne i64 %283, 0
  %285 = select i1 %282, i1 %284, i1 false
  %286 = select i1 %281, i1 true, i1 %285
  %287 = zext i1 %286 to i64
  %288 = add nuw nsw i64 %268, %287
  br label %289

289:                                              ; preds = %277, %264, %259
  %290 = phi i64 [ %263, %259 ], [ %288, %277 ], [ %276, %264 ]
  %291 = zext nneg i32 %219 to i64
  %292 = shl nuw nsw i64 2, %291
  %293 = icmp ult i64 %290, %292
  br i1 %293, label %298, label %294

294:                                              ; preds = %289
  %295 = sub i32 %253, %247
  %296 = sext i32 %295 to i64
  %297 = shl nsw i64 %296, %291
  br label %307

298:                                              ; preds = %289
  %299 = lshr i64 %290, %291
  %300 = icmp eq i64 %299, 0
  %301 = add nsw i32 %253, %248
  %302 = sext i32 %301 to i64
  %303 = add nsw i64 %302, -1
  %304 = shl nsw i64 %303, %291
  %305 = select i1 %300, i64 0, i64 %304
  %306 = add i64 %305, %290
  br label %307

307:                                              ; preds = %298, %294, %246
  %308 = phi i64 [ 1099511627776, %246 ], [ %297, %294 ], [ %306, %298 ]
  %309 = zext nneg i32 %222 to i64
  %310 = icmp slt i64 %308, %309
  %311 = trunc i64 %308 to i32
  %312 = select i1 %310, i32 %311, i32 %232
  %313 = or i32 %312, %229
  br label %314

314:                                              ; preds = %214, %217, %234, %236, %307
  %315 = phi i32 [ 32767, %214 ], [ 0, %217 ], [ %235, %234 ], [ %313, %307 ], [ %229, %236 ]
  %316 = icmp eq i32 %315, 32768
  %317 = shl i32 %315, 16
  %318 = select i1 %316, i32 0, i32 %317
  %319 = lshr i32 %3, 23
  %320 = and i32 %3, 8388607
  %321 = trunc i32 %319 to i8
  switch i8 %321, label %325 [
    i8 -1, label %322
    i8 0, label %329
  ]

322:                                              ; preds = %314
  %323 = icmp eq i32 %320, 0
  %324 = icmp ne i32 %320, 0
  br label %329

325:                                              ; preds = %314
  %326 = and i32 %319, 255
  %327 = or disjoint i32 %320, 8388608
  %328 = add nsw i32 %326, -150
  br label %329

329:                                              ; preds = %314, %322, %325
  %330 = phi i32 [ 0, %322 ], [ %328, %325 ], [ -149, %314 ]
  %331 = phi i32 [ 0, %322 ], [ %327, %325 ], [ %320, %314 ]
  %332 = phi i1 [ %324, %322 ], [ true, %325 ], [ true, %314 ]
  %333 = phi i1 [ %323, %322 ], [ true, %325 ], [ true, %314 ]
  %334 = zext nneg i32 %331 to i64
  br i1 %333, label %335, label %431

335:                                              ; preds = %329
  %336 = select i1 %215, i32 8, i32 5
  %337 = select i1 %215, i32 7, i32 10
  %338 = shl nsw i32 -1, %336
  %339 = xor i32 %338, -1
  %340 = shl nuw nsw i32 %339, %337
  %341 = and i32 %1, 4
  %342 = icmp ne i32 %341, 0
  %343 = icmp slt i32 %3, 0
  %344 = and i1 %342, %343
  br i1 %344, label %431, label %345

345:                                              ; preds = %335
  %346 = lshr i32 %3, 16
  %347 = and i32 %346, 32768
  %348 = shl i32 %1, 25
  %349 = ashr i32 %348, 31
  %350 = add nsw i32 %340, %349
  br i1 %332, label %353, label %351

351:                                              ; preds = %345
  %352 = or i32 %347, %350
  br label %431

353:                                              ; preds = %345
  %354 = icmp eq i32 %331, 0
  br i1 %354, label %431, label %355

355:                                              ; preds = %353
  %356 = add nsw i32 %336, -1
  br label %357

357:                                              ; preds = %357, %355
  %358 = phi i32 [ %361, %357 ], [ 0, %355 ]
  %359 = phi i64 [ %360, %357 ], [ %334, %355 ]
  %360 = lshr i64 %359, 1
  %361 = add nuw nsw i32 %358, 1
  %362 = icmp ult i64 %359, 2
  br i1 %362, label %363, label %357, !llvm.loop !20003

363:                                              ; preds = %357
  %364 = shl nsw i32 -1, %356
  %365 = xor i32 %364, -1
  %366 = add nuw nsw i32 %364, 2
  %367 = add nsw i32 %361, %330
  %368 = add i32 %358, %330
  %369 = icmp sgt i32 %367, %366
  %370 = select i1 %369, i32 %368, i32 %366
  %371 = icmp sgt i32 %370, 200
  br i1 %371, label %424, label %372

372:                                              ; preds = %363
  %373 = sub i32 %337, %370
  %374 = add i32 %373, %330
  %375 = icmp sgt i32 %374, -1
  br i1 %375, label %376, label %381

376:                                              ; preds = %372
  %377 = icmp ugt i32 %374, 40
  %378 = zext nneg i32 %374 to i64
  %379 = shl i64 %334, %378
  %380 = select i1 %377, i64 4611686018427387904, i64 %379
  br label %406

381:                                              ; preds = %372
  %382 = sub nsw i32 0, %374
  %383 = icmp ult i32 %374, -63
  %384 = zext nneg i32 %382 to i64
  %385 = lshr i64 %334, %384
  %386 = select i1 %383, i64 0, i64 %385
  %387 = shl nsw i64 -1, %384
  %388 = xor i64 %387, -1
  %389 = select i1 %383, i64 2147483647, i64 %388
  %390 = and i64 %389, %334
  %391 = icmp eq i64 %390, 0
  %392 = or i1 %383, %391
  %393 = select i1 %391, i64 %386, i64 0
  br i1 %392, label %406, label %394

394:                                              ; preds = %381
  %395 = xor i32 %374, -1
  %396 = zext nneg i32 %395 to i64
  %397 = shl nuw nsw i64 1, %396
  %398 = icmp ugt i64 %390, %397
  %399 = icmp eq i64 %390, %397
  %400 = and i64 %385, 1
  %401 = icmp ne i64 %400, 0
  %402 = select i1 %399, i1 %401, i1 false
  %403 = select i1 %398, i1 true, i1 %402
  %404 = zext i1 %403 to i64
  %405 = add nuw nsw i64 %385, %404
  br label %406

406:                                              ; preds = %394, %381, %376
  %407 = phi i64 [ %380, %376 ], [ %405, %394 ], [ %393, %381 ]
  %408 = zext nneg i32 %337 to i64
  %409 = shl nuw nsw i64 2, %408
  %410 = icmp ult i64 %407, %409
  br i1 %410, label %415, label %411

411:                                              ; preds = %406
  %412 = sub i32 %370, %364
  %413 = sext i32 %412 to i64
  %414 = shl nsw i64 %413, %408
  br label %424

415:                                              ; preds = %406
  %416 = lshr i64 %407, %408
  %417 = icmp eq i64 %416, 0
  %418 = add nsw i32 %370, %365
  %419 = sext i32 %418 to i64
  %420 = add nsw i64 %419, -1
  %421 = shl nsw i64 %420, %408
  %422 = select i1 %417, i64 0, i64 %421
  %423 = add i64 %422, %407
  br label %424

424:                                              ; preds = %415, %411, %363
  %425 = phi i64 [ 1099511627776, %363 ], [ %414, %411 ], [ %423, %415 ]
  %426 = zext nneg i32 %340 to i64
  %427 = icmp slt i64 %425, %426
  %428 = trunc i64 %425 to i32
  %429 = select i1 %427, i32 %428, i32 %350
  %430 = or i32 %429, %347
  br label %431

431:                                              ; preds = %329, %335, %351, %353, %424
  %432 = phi i32 [ 32767, %329 ], [ 0, %335 ], [ %352, %351 ], [ %430, %424 ], [ %347, %353 ]
  %433 = icmp eq i32 %432, 32768
  %434 = select i1 %433, i32 0, i32 %432
  %435 = or i32 %434, %318
  br label %436

436:                                              ; preds = %26, %24, %431, %210, %99
  %437 = phi i32 [ %107, %99 ], [ %213, %210 ], [ %435, %431 ], [ 2147483647, %24 ], [ 0, %26 ]
  ret i32 %437
}

; Function Attrs: nofree norecurse nosync nounwind memory(none)
define dso_local i32 @__zluda_ptx_impl_agx_cvt_rs(i32 noundef %0, i32 noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef %4, i32 noundef %5, i32 noundef %6) local_unnamed_addr #3 {
  %8 = icmp ugt i32 %0, 7
  br i1 %8, label %9, label %317

9:                                                ; preds = %7
  %10 = icmp eq i32 %0, 9
  %11 = select i1 %10, i32 65535, i32 8191
  %12 = lshr i32 %2, 23
  %13 = and i32 %2, 8388607
  %14 = trunc i32 %12 to i8
  switch i8 %14, label %20 [
    i8 -1, label %15
    i8 0, label %24
  ]

15:                                               ; preds = %9
  %16 = icmp ne i32 %13, 0
  %17 = zext i1 %16 to i32
  %18 = icmp eq i32 %13, 0
  %19 = zext i1 %18 to i32
  br label %24

20:                                               ; preds = %9
  %21 = and i32 %12, 255
  %22 = or disjoint i32 %13, 8388608
  %23 = add nsw i32 %21, -150
  br label %24

24:                                               ; preds = %9, %15, %20
  %25 = phi i32 [ 0, %15 ], [ %23, %20 ], [ -149, %9 ]
  %26 = phi i32 [ 0, %15 ], [ %22, %20 ], [ %13, %9 ]
  %27 = phi i32 [ %19, %15 ], [ 0, %20 ], [ 0, %9 ]
  %28 = phi i32 [ %17, %15 ], [ 0, %20 ], [ 0, %9 ]
  %29 = zext nneg i32 %26 to i64
  %30 = lshr i32 %6, 16
  %31 = and i32 %30, %11
  %32 = select i1 %10, i32 8, i32 5
  %33 = select i1 %10, i32 -7, i32 -10
  %34 = select i1 %10, i32 7, i32 10
  %35 = shl nsw i32 -1, %32
  %36 = xor i32 %35, -1
  %37 = shl nuw nsw i32 %36, %34
  %38 = icmp eq i32 %28, 0
  br i1 %38, label %39, label %163

39:                                               ; preds = %24
  %40 = and i32 %1, 4
  %41 = icmp ne i32 %40, 0
  %42 = icmp slt i32 %2, 0
  %43 = and i1 %41, %42
  br i1 %43, label %163, label %44

44:                                               ; preds = %39
  %45 = lshr i32 %2, 16
  %46 = and i32 %45, 32768
  %47 = shl i32 %1, 25
  %48 = ashr i32 %47, 31
  %49 = add nsw i32 %37, %48
  %50 = icmp eq i32 %27, 0
  br i1 %50, label %53, label %51

51:                                               ; preds = %44
  %52 = zext nneg i32 %37 to i64
  br label %156

53:                                               ; preds = %44
  %54 = icmp eq i32 %26, 0
  br i1 %54, label %156, label %55

55:                                               ; preds = %53, %55
  %56 = phi i32 [ %59, %55 ], [ 0, %53 ]
  %57 = phi i64 [ %58, %55 ], [ %29, %53 ]
  %58 = lshr i64 %57, 1
  %59 = add nuw nsw i32 %56, 1
  %60 = icmp ult i64 %57, 2
  br i1 %60, label %61, label %55, !llvm.loop !20003

61:                                               ; preds = %55
  %62 = add nsw i32 %32, -1
  %63 = select i1 %10, i32 16, i32 13
  %64 = shl nsw i32 -1, %62
  %65 = add nsw i32 %59, %25
  %66 = add i32 %56, %25
  %67 = add nuw nsw i32 %64, 2
  %68 = icmp sgt i32 %65, %67
  %69 = select i1 %68, i32 %66, i32 %67
  %70 = sub nsw i32 %33, %25
  %71 = add i32 %69, %70
  %72 = icmp slt i32 %71, 1
  br i1 %72, label %93, label %73

73:                                               ; preds = %61
  %74 = icmp ult i32 %71, %63
  br i1 %74, label %85, label %75

75:                                               ; preds = %73
  %76 = sub nsw i32 %71, %63
  %77 = icmp sgt i32 %76, 63
  br i1 %77, label %93, label %78

78:                                               ; preds = %75
  %79 = zext nneg i32 %76 to i64
  %80 = lshr i64 %29, %79
  %81 = zext nneg i32 %63 to i64
  %82 = shl nsw i64 -1, %81
  %83 = xor i64 %82, -1
  %84 = and i64 %80, %83
  br label %93

85:                                               ; preds = %73
  %86 = zext nneg i32 %71 to i64
  %87 = shl nsw i64 -1, %86
  %88 = xor i64 %87, -1
  %89 = and i64 %88, %29
  %90 = sub nsw i32 %63, %71
  %91 = zext nneg i32 %90 to i64
  %92 = shl i64 %89, %91
  br label %93

93:                                               ; preds = %85, %78, %75, %61
  %94 = phi i64 [ %92, %85 ], [ 0, %61 ], [ %84, %78 ], [ 0, %75 ]
  %95 = zext nneg i32 %31 to i64
  %96 = add i64 %94, %95
  %97 = zext nneg i32 %63 to i64
  %98 = lshr i64 %96, %97
  %99 = icmp eq i64 %98, 0
  br label %100

100:                                              ; preds = %100, %93
  %101 = phi i32 [ %104, %100 ], [ 0, %93 ]
  %102 = phi i64 [ %103, %100 ], [ %29, %93 ]
  %103 = lshr i64 %102, 1
  %104 = add nuw nsw i32 %101, 1
  %105 = icmp ult i64 %102, 2
  br i1 %105, label %106, label %100, !llvm.loop !20003

106:                                              ; preds = %100
  %107 = select i1 %72, i1 true, i1 %99
  %108 = xor i32 %64, -1
  %109 = add nsw i32 %104, %25
  %110 = add i32 %101, %25
  %111 = icmp sgt i32 %109, %67
  %112 = select i1 %111, i32 %110, i32 %67
  %113 = icmp sgt i32 %112, 200
  br i1 %113, label %156, label %114

114:                                              ; preds = %106
  %115 = sub i32 %34, %112
  %116 = add i32 %115, %25
  %117 = icmp sgt i32 %116, -1
  br i1 %117, label %118, label %123

118:                                              ; preds = %114
  %119 = icmp ugt i32 %116, 40
  %120 = zext nneg i32 %116 to i64
  %121 = shl i64 %29, %120
  %122 = select i1 %119, i64 4611686018427387904, i64 %121
  br label %138

123:                                              ; preds = %114
  %124 = sub nsw i32 0, %116
  %125 = icmp ult i32 %116, -63
  %126 = zext nneg i32 %124 to i64
  %127 = lshr i64 %29, %126
  %128 = select i1 %125, i64 0, i64 %127
  %129 = shl nsw i64 -1, %126
  %130 = xor i64 %129, -1
  %131 = select i1 %125, i64 2147483647, i64 %130
  %132 = and i64 %131, %29
  %133 = icmp eq i64 %132, 0
  %134 = or i1 %107, %133
  %135 = xor i1 %134, true
  %136 = zext i1 %135 to i64
  %137 = add nuw nsw i64 %128, %136
  br label %138

138:                                              ; preds = %123, %118
  %139 = phi i64 [ %122, %118 ], [ %137, %123 ]
  %140 = zext nneg i32 %34 to i64
  %141 = shl nuw nsw i64 2, %140
  %142 = icmp ult i64 %139, %141
  br i1 %142, label %147, label %143

143:                                              ; preds = %138
  %144 = sub i32 %112, %64
  %145 = sext i32 %144 to i64
  %146 = shl nsw i64 %145, %140
  br label %156

147:                                              ; preds = %138
  %148 = lshr i64 %139, %140
  %149 = icmp eq i64 %148, 0
  %150 = add nsw i32 %112, %108
  %151 = sext i32 %150 to i64
  %152 = add nsw i64 %151, -1
  %153 = shl nsw i64 %152, %140
  %154 = select i1 %149, i64 0, i64 %153
  %155 = add i64 %154, %139
  br label %156

156:                                              ; preds = %147, %143, %106, %53, %51
  %157 = phi i64 [ %52, %51 ], [ 0, %53 ], [ 1099511627776, %106 ], [ %146, %143 ], [ %155, %147 ]
  %158 = zext nneg i32 %37 to i64
  %159 = icmp slt i64 %157, %158
  %160 = trunc i64 %157 to i32
  %161 = select i1 %159, i32 %160, i32 %49
  %162 = or i32 %161, %46
  br label %163

163:                                              ; preds = %24, %39, %156
  %164 = phi i32 [ %162, %156 ], [ 32767, %24 ], [ 0, %39 ]
  %165 = and i32 %1, 8
  %166 = icmp ne i32 %165, 0
  %167 = icmp eq i32 %164, 32768
  %168 = select i1 %166, i1 %167, i1 false
  %169 = shl i32 %164, 16
  %170 = select i1 %168, i32 0, i32 %169
  %171 = lshr i32 %3, 23
  %172 = and i32 %3, 8388607
  %173 = trunc i32 %171 to i8
  switch i8 %173, label %177 [
    i8 -1, label %174
    i8 0, label %181
  ]

174:                                              ; preds = %163
  %175 = icmp eq i32 %172, 0
  %176 = icmp ne i32 %172, 0
  br label %181

177:                                              ; preds = %163
  %178 = and i32 %171, 255
  %179 = or disjoint i32 %172, 8388608
  %180 = add nsw i32 %178, -150
  br label %181

181:                                              ; preds = %163, %174, %177
  %182 = phi i32 [ 0, %174 ], [ %180, %177 ], [ -149, %163 ]
  %183 = phi i32 [ 0, %174 ], [ %179, %177 ], [ %172, %163 ]
  %184 = phi i1 [ %176, %174 ], [ true, %177 ], [ true, %163 ]
  %185 = phi i1 [ %175, %174 ], [ true, %177 ], [ true, %163 ]
  %186 = zext nneg i32 %183 to i64
  %187 = and i32 %11, %6
  br i1 %185, label %188, label %311

188:                                              ; preds = %181
  %189 = and i32 %1, 4
  %190 = icmp ne i32 %189, 0
  %191 = icmp slt i32 %3, 0
  %192 = and i1 %190, %191
  br i1 %192, label %311, label %193

193:                                              ; preds = %188
  %194 = lshr i32 %3, 16
  %195 = and i32 %194, 32768
  %196 = shl i32 %1, 25
  %197 = ashr i32 %196, 31
  %198 = add nsw i32 %37, %197
  br i1 %184, label %201, label %199

199:                                              ; preds = %193
  %200 = zext nneg i32 %37 to i64
  br label %304

201:                                              ; preds = %193
  %202 = icmp eq i32 %183, 0
  br i1 %202, label %304, label %203

203:                                              ; preds = %201, %203
  %204 = phi i32 [ %207, %203 ], [ 0, %201 ]
  %205 = phi i64 [ %206, %203 ], [ %186, %201 ]
  %206 = lshr i64 %205, 1
  %207 = add nuw nsw i32 %204, 1
  %208 = icmp ult i64 %205, 2
  br i1 %208, label %209, label %203, !llvm.loop !20003

209:                                              ; preds = %203
  %210 = add nsw i32 %32, -1
  %211 = select i1 %10, i32 16, i32 13
  %212 = shl nsw i32 -1, %210
  %213 = add nsw i32 %207, %182
  %214 = add i32 %204, %182
  %215 = add nuw nsw i32 %212, 2
  %216 = icmp sgt i32 %213, %215
  %217 = select i1 %216, i32 %214, i32 %215
  %218 = sub nsw i32 %33, %182
  %219 = add i32 %217, %218
  %220 = icmp slt i32 %219, 1
  br i1 %220, label %241, label %221

221:                                              ; preds = %209
  %222 = icmp ult i32 %219, %211
  br i1 %222, label %233, label %223

223:                                              ; preds = %221
  %224 = sub nsw i32 %219, %211
  %225 = icmp sgt i32 %224, 63
  br i1 %225, label %241, label %226

226:                                              ; preds = %223
  %227 = zext nneg i32 %224 to i64
  %228 = lshr i64 %186, %227
  %229 = zext nneg i32 %211 to i64
  %230 = shl nsw i64 -1, %229
  %231 = xor i64 %230, -1
  %232 = and i64 %228, %231
  br label %241

233:                                              ; preds = %221
  %234 = zext nneg i32 %219 to i64
  %235 = shl nsw i64 -1, %234
  %236 = xor i64 %235, -1
  %237 = and i64 %236, %186
  %238 = sub nsw i32 %211, %219
  %239 = zext nneg i32 %238 to i64
  %240 = shl i64 %237, %239
  br label %241

241:                                              ; preds = %233, %226, %223, %209
  %242 = phi i64 [ %240, %233 ], [ 0, %209 ], [ %232, %226 ], [ 0, %223 ]
  %243 = zext nneg i32 %187 to i64
  %244 = add i64 %242, %243
  %245 = zext nneg i32 %211 to i64
  %246 = lshr i64 %244, %245
  %247 = icmp eq i64 %246, 0
  br label %248

248:                                              ; preds = %248, %241
  %249 = phi i32 [ %252, %248 ], [ 0, %241 ]
  %250 = phi i64 [ %251, %248 ], [ %186, %241 ]
  %251 = lshr i64 %250, 1
  %252 = add nuw nsw i32 %249, 1
  %253 = icmp ult i64 %250, 2
  br i1 %253, label %254, label %248, !llvm.loop !20003

254:                                              ; preds = %248
  %255 = select i1 %220, i1 true, i1 %247
  %256 = xor i32 %212, -1
  %257 = add nsw i32 %252, %182
  %258 = add i32 %249, %182
  %259 = icmp sgt i32 %257, %215
  %260 = select i1 %259, i32 %258, i32 %215
  %261 = icmp sgt i32 %260, 200
  br i1 %261, label %304, label %262

262:                                              ; preds = %254
  %263 = sub i32 %34, %260
  %264 = add i32 %263, %182
  %265 = icmp sgt i32 %264, -1
  br i1 %265, label %266, label %271

266:                                              ; preds = %262
  %267 = icmp ugt i32 %264, 40
  %268 = zext nneg i32 %264 to i64
  %269 = shl i64 %186, %268
  %270 = select i1 %267, i64 4611686018427387904, i64 %269
  br label %286

271:                                              ; preds = %262
  %272 = sub nsw i32 0, %264
  %273 = icmp ult i32 %264, -63
  %274 = zext nneg i32 %272 to i64
  %275 = lshr i64 %186, %274
  %276 = select i1 %273, i64 0, i64 %275
  %277 = shl nsw i64 -1, %274
  %278 = xor i64 %277, -1
  %279 = select i1 %273, i64 2147483647, i64 %278
  %280 = and i64 %279, %186
  %281 = icmp eq i64 %280, 0
  %282 = or i1 %255, %281
  %283 = xor i1 %282, true
  %284 = zext i1 %283 to i64
  %285 = add nuw nsw i64 %276, %284
  br label %286

286:                                              ; preds = %271, %266
  %287 = phi i64 [ %270, %266 ], [ %285, %271 ]
  %288 = zext nneg i32 %34 to i64
  %289 = shl nuw nsw i64 2, %288
  %290 = icmp ult i64 %287, %289
  br i1 %290, label %295, label %291

291:                                              ; preds = %286
  %292 = sub i32 %260, %212
  %293 = sext i32 %292 to i64
  %294 = shl nsw i64 %293, %288
  br label %304

295:                                              ; preds = %286
  %296 = lshr i64 %287, %288
  %297 = icmp eq i64 %296, 0
  %298 = add nsw i32 %260, %256
  %299 = sext i32 %298 to i64
  %300 = add nsw i64 %299, -1
  %301 = shl nsw i64 %300, %288
  %302 = select i1 %297, i64 0, i64 %301
  %303 = add i64 %302, %287
  br label %304

304:                                              ; preds = %295, %291, %254, %201, %199
  %305 = phi i64 [ %200, %199 ], [ 0, %201 ], [ 1099511627776, %254 ], [ %294, %291 ], [ %303, %295 ]
  %306 = zext nneg i32 %37 to i64
  %307 = icmp slt i64 %305, %306
  %308 = trunc i64 %305 to i32
  %309 = select i1 %307, i32 %308, i32 %198
  %310 = or i32 %309, %195
  br label %311

311:                                              ; preds = %181, %188, %304
  %312 = phi i32 [ %310, %304 ], [ 32767, %181 ], [ 0, %188 ]
  %313 = icmp eq i32 %312, 32768
  %314 = select i1 %166, i1 %313, i1 false
  %315 = select i1 %314, i32 0, i32 %312
  %316 = or i32 %315, %170
  br label %1525

317:                                              ; preds = %7
  %318 = icmp eq i32 %0, 4
  %319 = lshr i32 %2, 23
  %320 = and i32 %2, 8388607
  %321 = trunc i32 %319 to i8
  br i1 %318, label %322, label %836

322:                                              ; preds = %317
  switch i8 %321, label %328 [
    i8 -1, label %323
    i8 0, label %332
  ]

323:                                              ; preds = %322
  %324 = icmp ne i32 %320, 0
  %325 = zext i1 %324 to i32
  %326 = icmp eq i32 %320, 0
  %327 = zext i1 %326 to i32
  br label %332

328:                                              ; preds = %322
  %329 = and i32 %319, 255
  %330 = or disjoint i32 %320, 8388608
  %331 = add nsw i32 %329, -150
  br label %332

332:                                              ; preds = %322, %323, %328
  %333 = phi i32 [ 0, %323 ], [ %331, %328 ], [ -149, %322 ]
  %334 = phi i32 [ 0, %323 ], [ %330, %328 ], [ %320, %322 ]
  %335 = phi i32 [ %327, %323 ], [ 0, %328 ], [ 0, %322 ]
  %336 = phi i32 [ %325, %323 ], [ 0, %328 ], [ 0, %322 ]
  %337 = zext nneg i32 %334 to i64
  %338 = lshr i32 %6, 24
  %339 = lshr i32 %2, 28
  %340 = and i32 %339, 8
  %341 = icmp eq i32 %336, 0
  br i1 %341, label %342, label %446

342:                                              ; preds = %332
  %343 = and i32 %1, 4
  %344 = icmp ne i32 %343, 0
  %345 = icmp slt i32 %2, 0
  %346 = and i1 %344, %345
  br i1 %346, label %446, label %347

347:                                              ; preds = %342
  %348 = icmp eq i32 %335, 0
  br i1 %348, label %349, label %438

349:                                              ; preds = %347
  %350 = icmp eq i32 %334, 0
  br i1 %350, label %434, label %351

351:                                              ; preds = %349, %351
  %352 = phi i32 [ %355, %351 ], [ 0, %349 ]
  %353 = phi i64 [ %354, %351 ], [ %337, %349 ]
  %354 = lshr i64 %353, 1
  %355 = add nuw nsw i32 %352, 1
  %356 = icmp ult i64 %353, 2
  br i1 %356, label %357, label %351, !llvm.loop !20003

357:                                              ; preds = %351
  %358 = add nsw i32 %355, %333
  %359 = add i32 %352, %333
  %360 = icmp sgt i32 %358, 0
  %361 = select i1 %360, i32 %359, i32 0
  %362 = xor i32 %333, -1
  %363 = add i32 %361, %362
  %364 = icmp slt i32 %363, 1
  br i1 %364, label %379, label %365

365:                                              ; preds = %357
  %366 = icmp ult i32 %363, 8
  br i1 %366, label %374, label %367

367:                                              ; preds = %365
  %368 = add nsw i32 %363, -8
  %369 = icmp ugt i32 %368, 63
  %370 = zext nneg i32 %368 to i64
  %371 = lshr i64 %337, %370
  %372 = and i64 %371, 255
  %373 = select i1 %369, i64 0, i64 %372
  br label %379

374:                                              ; preds = %365
  %375 = sub nuw nsw i32 8, %363
  %376 = zext nneg i32 %375 to i64
  %377 = shl nuw nsw i64 %337, %376
  %378 = and i64 %377, 254
  br label %379

379:                                              ; preds = %367, %374, %357
  %380 = phi i64 [ %378, %374 ], [ 0, %357 ], [ %373, %367 ]
  %381 = zext nneg i32 %338 to i64
  %382 = add nuw nsw i64 %380, %381
  %383 = icmp ult i64 %382, 256
  br label %384

384:                                              ; preds = %384, %379
  %385 = phi i32 [ %388, %384 ], [ 0, %379 ]
  %386 = phi i64 [ %387, %384 ], [ %337, %379 ]
  %387 = lshr i64 %386, 1
  %388 = add nuw nsw i32 %385, 1
  %389 = icmp ult i64 %386, 2
  br i1 %389, label %390, label %384, !llvm.loop !20003

390:                                              ; preds = %384
  %391 = select i1 %364, i1 true, i1 %383
  %392 = add nsw i32 %388, %333
  %393 = add i32 %385, %333
  %394 = icmp sgt i32 %392, 0
  %395 = select i1 %394, i32 %393, i32 0
  %396 = icmp sgt i32 %395, 200
  br i1 %396, label %434, label %397

397:                                              ; preds = %390
  %398 = sub i32 %333, %395
  %399 = add i32 %398, 1
  %400 = icmp sgt i32 %399, -1
  br i1 %400, label %401, label %406

401:                                              ; preds = %397
  %402 = icmp ugt i32 %399, 40
  %403 = zext nneg i32 %399 to i64
  %404 = shl i64 %337, %403
  %405 = select i1 %402, i64 4611686018427387904, i64 %404
  br label %421

406:                                              ; preds = %397
  %407 = xor i32 %398, -1
  %408 = icmp ult i32 %399, -63
  %409 = zext nneg i32 %407 to i64
  %410 = lshr i64 %337, %409
  %411 = select i1 %408, i64 0, i64 %410
  %412 = shl nsw i64 -1, %409
  %413 = xor i64 %412, -1
  %414 = select i1 %408, i64 2147483647, i64 %413
  %415 = and i64 %414, %337
  %416 = icmp eq i64 %415, 0
  %417 = or i1 %391, %416
  %418 = xor i1 %417, true
  %419 = zext i1 %418 to i64
  %420 = add nuw nsw i64 %411, %419
  br label %421

421:                                              ; preds = %406, %401
  %422 = phi i64 [ %405, %401 ], [ %420, %406 ]
  %423 = icmp ult i64 %422, 4
  br i1 %423, label %428, label %424

424:                                              ; preds = %421
  %425 = add nsw i32 %395, 2
  %426 = sext i32 %425 to i64
  %427 = shl nsw i64 %426, 1
  br label %434

428:                                              ; preds = %421
  %429 = icmp ult i64 %422, 2
  %430 = sext i32 %395 to i64
  %431 = shl nsw i64 %430, 1
  %432 = select i1 %429, i64 0, i64 %431
  %433 = add i64 %432, %422
  br label %434

434:                                              ; preds = %428, %424, %390, %349
  %435 = phi i64 [ 0, %349 ], [ 1099511627776, %390 ], [ %427, %424 ], [ %433, %428 ]
  %436 = tail call i64 @llvm.smin.i64(i64 %435, i64 7)
  %437 = trunc i64 %436 to i32
  br label %438

438:                                              ; preds = %434, %347
  %439 = phi i32 [ 7, %347 ], [ %437, %434 ]
  %440 = or i32 %439, %340
  %441 = and i32 %1, 8
  %442 = icmp ne i32 %441, 0
  %443 = icmp eq i32 %440, 8
  %444 = select i1 %442, i1 %443, i1 false
  %445 = select i1 %444, i32 0, i32 %440
  br label %446

446:                                              ; preds = %332, %342, %438
  %447 = phi i32 [ 0, %342 ], [ %445, %438 ], [ 7, %332 ]
  %448 = shl i32 %447, 12
  %449 = lshr i32 %3, 23
  %450 = and i32 %3, 8388607
  %451 = trunc i32 %449 to i8
  switch i8 %451, label %457 [
    i8 -1, label %452
    i8 0, label %461
  ]

452:                                              ; preds = %446
  %453 = icmp ne i32 %450, 0
  %454 = zext i1 %453 to i32
  %455 = icmp eq i32 %450, 0
  %456 = zext i1 %455 to i32
  br label %461

457:                                              ; preds = %446
  %458 = and i32 %449, 255
  %459 = or disjoint i32 %450, 8388608
  %460 = add nsw i32 %458, -150
  br label %461

461:                                              ; preds = %446, %452, %457
  %462 = phi i32 [ 0, %452 ], [ %460, %457 ], [ -149, %446 ]
  %463 = phi i32 [ 0, %452 ], [ %459, %457 ], [ %450, %446 ]
  %464 = phi i32 [ %456, %452 ], [ 0, %457 ], [ 0, %446 ]
  %465 = phi i32 [ %454, %452 ], [ 0, %457 ], [ 0, %446 ]
  %466 = zext nneg i32 %463 to i64
  %467 = lshr i32 %6, 8
  %468 = and i32 %467, 255
  %469 = lshr i32 %3, 28
  %470 = and i32 %469, 8
  %471 = icmp eq i32 %465, 0
  br i1 %471, label %472, label %576

472:                                              ; preds = %461
  %473 = and i32 %1, 4
  %474 = icmp ne i32 %473, 0
  %475 = icmp slt i32 %3, 0
  %476 = and i1 %474, %475
  br i1 %476, label %576, label %477

477:                                              ; preds = %472
  %478 = icmp eq i32 %464, 0
  br i1 %478, label %479, label %568

479:                                              ; preds = %477
  %480 = icmp eq i32 %463, 0
  br i1 %480, label %564, label %481

481:                                              ; preds = %479, %481
  %482 = phi i32 [ %485, %481 ], [ 0, %479 ]
  %483 = phi i64 [ %484, %481 ], [ %466, %479 ]
  %484 = lshr i64 %483, 1
  %485 = add nuw nsw i32 %482, 1
  %486 = icmp ult i64 %483, 2
  br i1 %486, label %487, label %481, !llvm.loop !20003

487:                                              ; preds = %481
  %488 = add nsw i32 %485, %462
  %489 = add i32 %482, %462
  %490 = icmp sgt i32 %488, 0
  %491 = select i1 %490, i32 %489, i32 0
  %492 = xor i32 %462, -1
  %493 = add i32 %491, %492
  %494 = icmp slt i32 %493, 1
  br i1 %494, label %509, label %495

495:                                              ; preds = %487
  %496 = icmp ult i32 %493, 8
  br i1 %496, label %504, label %497

497:                                              ; preds = %495
  %498 = add nsw i32 %493, -8
  %499 = icmp ugt i32 %498, 63
  %500 = zext nneg i32 %498 to i64
  %501 = lshr i64 %466, %500
  %502 = and i64 %501, 255
  %503 = select i1 %499, i64 0, i64 %502
  br label %509

504:                                              ; preds = %495
  %505 = sub nuw nsw i32 8, %493
  %506 = zext nneg i32 %505 to i64
  %507 = shl nuw nsw i64 %466, %506
  %508 = and i64 %507, 254
  br label %509

509:                                              ; preds = %497, %504, %487
  %510 = phi i64 [ %508, %504 ], [ 0, %487 ], [ %503, %497 ]
  %511 = zext nneg i32 %468 to i64
  %512 = add nuw nsw i64 %510, %511
  %513 = icmp ult i64 %512, 256
  br label %514

514:                                              ; preds = %514, %509
  %515 = phi i32 [ %518, %514 ], [ 0, %509 ]
  %516 = phi i64 [ %517, %514 ], [ %466, %509 ]
  %517 = lshr i64 %516, 1
  %518 = add nuw nsw i32 %515, 1
  %519 = icmp ult i64 %516, 2
  br i1 %519, label %520, label %514, !llvm.loop !20003

520:                                              ; preds = %514
  %521 = select i1 %494, i1 true, i1 %513
  %522 = add nsw i32 %518, %462
  %523 = add i32 %515, %462
  %524 = icmp sgt i32 %522, 0
  %525 = select i1 %524, i32 %523, i32 0
  %526 = icmp sgt i32 %525, 200
  br i1 %526, label %564, label %527

527:                                              ; preds = %520
  %528 = sub i32 %462, %525
  %529 = add i32 %528, 1
  %530 = icmp sgt i32 %529, -1
  br i1 %530, label %531, label %536

531:                                              ; preds = %527
  %532 = icmp ugt i32 %529, 40
  %533 = zext nneg i32 %529 to i64
  %534 = shl i64 %466, %533
  %535 = select i1 %532, i64 4611686018427387904, i64 %534
  br label %551

536:                                              ; preds = %527
  %537 = xor i32 %528, -1
  %538 = icmp ult i32 %529, -63
  %539 = zext nneg i32 %537 to i64
  %540 = lshr i64 %466, %539
  %541 = select i1 %538, i64 0, i64 %540
  %542 = shl nsw i64 -1, %539
  %543 = xor i64 %542, -1
  %544 = select i1 %538, i64 2147483647, i64 %543
  %545 = and i64 %544, %466
  %546 = icmp eq i64 %545, 0
  %547 = or i1 %521, %546
  %548 = xor i1 %547, true
  %549 = zext i1 %548 to i64
  %550 = add nuw nsw i64 %541, %549
  br label %551

551:                                              ; preds = %536, %531
  %552 = phi i64 [ %535, %531 ], [ %550, %536 ]
  %553 = icmp ult i64 %552, 4
  br i1 %553, label %558, label %554

554:                                              ; preds = %551
  %555 = add nsw i32 %525, 2
  %556 = sext i32 %555 to i64
  %557 = shl nsw i64 %556, 1
  br label %564

558:                                              ; preds = %551
  %559 = icmp ult i64 %552, 2
  %560 = sext i32 %525 to i64
  %561 = shl nsw i64 %560, 1
  %562 = select i1 %559, i64 0, i64 %561
  %563 = add i64 %562, %552
  br label %564

564:                                              ; preds = %558, %554, %520, %479
  %565 = phi i64 [ 0, %479 ], [ 1099511627776, %520 ], [ %557, %554 ], [ %563, %558 ]
  %566 = tail call i64 @llvm.smin.i64(i64 %565, i64 7)
  %567 = trunc i64 %566 to i32
  br label %568

568:                                              ; preds = %564, %477
  %569 = phi i32 [ 7, %477 ], [ %567, %564 ]
  %570 = or i32 %569, %470
  %571 = and i32 %1, 8
  %572 = icmp ne i32 %571, 0
  %573 = icmp eq i32 %570, 8
  %574 = select i1 %572, i1 %573, i1 false
  %575 = select i1 %574, i32 0, i32 %570
  br label %576

576:                                              ; preds = %461, %472, %568
  %577 = phi i32 [ 0, %472 ], [ %575, %568 ], [ 7, %461 ]
  %578 = shl i32 %577, 8
  %579 = or i32 %578, %448
  %580 = lshr i32 %4, 23
  %581 = and i32 %4, 8388607
  %582 = trunc i32 %580 to i8
  switch i8 %582, label %588 [
    i8 -1, label %583
    i8 0, label %592
  ]

583:                                              ; preds = %576
  %584 = icmp ne i32 %581, 0
  %585 = zext i1 %584 to i32
  %586 = icmp eq i32 %581, 0
  %587 = zext i1 %586 to i32
  br label %592

588:                                              ; preds = %576
  %589 = and i32 %580, 255
  %590 = or disjoint i32 %581, 8388608
  %591 = add nsw i32 %589, -150
  br label %592

592:                                              ; preds = %576, %583, %588
  %593 = phi i32 [ 0, %583 ], [ %591, %588 ], [ -149, %576 ]
  %594 = phi i32 [ 0, %583 ], [ %590, %588 ], [ %581, %576 ]
  %595 = phi i32 [ %587, %583 ], [ 0, %588 ], [ 0, %576 ]
  %596 = phi i32 [ %585, %583 ], [ 0, %588 ], [ 0, %576 ]
  %597 = zext nneg i32 %594 to i64
  %598 = lshr i32 %6, 16
  %599 = and i32 %598, 255
  %600 = lshr i32 %4, 28
  %601 = and i32 %600, 8
  %602 = icmp eq i32 %596, 0
  br i1 %602, label %603, label %708

603:                                              ; preds = %592
  %604 = and i32 %1, 4
  %605 = icmp ne i32 %604, 0
  %606 = icmp slt i32 %4, 0
  %607 = and i1 %605, %606
  br i1 %607, label %708, label %608

608:                                              ; preds = %603
  %609 = icmp eq i32 %595, 0
  br i1 %609, label %610, label %699

610:                                              ; preds = %608
  %611 = icmp eq i32 %594, 0
  br i1 %611, label %695, label %612

612:                                              ; preds = %610, %612
  %613 = phi i32 [ %616, %612 ], [ 0, %610 ]
  %614 = phi i64 [ %615, %612 ], [ %597, %610 ]
  %615 = lshr i64 %614, 1
  %616 = add nuw nsw i32 %613, 1
  %617 = icmp ult i64 %614, 2
  br i1 %617, label %618, label %612, !llvm.loop !20003

618:                                              ; preds = %612
  %619 = add nsw i32 %616, %593
  %620 = add i32 %613, %593
  %621 = icmp sgt i32 %619, 0
  %622 = select i1 %621, i32 %620, i32 0
  %623 = xor i32 %593, -1
  %624 = add i32 %622, %623
  %625 = icmp slt i32 %624, 1
  br i1 %625, label %640, label %626

626:                                              ; preds = %618
  %627 = icmp ult i32 %624, 8
  br i1 %627, label %635, label %628

628:                                              ; preds = %626
  %629 = add nsw i32 %624, -8
  %630 = icmp ugt i32 %629, 63
  %631 = zext nneg i32 %629 to i64
  %632 = lshr i64 %597, %631
  %633 = and i64 %632, 255
  %634 = select i1 %630, i64 0, i64 %633
  br label %640

635:                                              ; preds = %626
  %636 = sub nuw nsw i32 8, %624
  %637 = zext nneg i32 %636 to i64
  %638 = shl nuw nsw i64 %597, %637
  %639 = and i64 %638, 254
  br label %640

640:                                              ; preds = %628, %635, %618
  %641 = phi i64 [ %639, %635 ], [ 0, %618 ], [ %634, %628 ]
  %642 = zext nneg i32 %599 to i64
  %643 = add nuw nsw i64 %641, %642
  %644 = icmp ult i64 %643, 256
  br label %645

645:                                              ; preds = %645, %640
  %646 = phi i32 [ %649, %645 ], [ 0, %640 ]
  %647 = phi i64 [ %648, %645 ], [ %597, %640 ]
  %648 = lshr i64 %647, 1
  %649 = add nuw nsw i32 %646, 1
  %650 = icmp ult i64 %647, 2
  br i1 %650, label %651, label %645, !llvm.loop !20003

651:                                              ; preds = %645
  %652 = select i1 %625, i1 true, i1 %644
  %653 = add nsw i32 %649, %593
  %654 = add i32 %646, %593
  %655 = icmp sgt i32 %653, 0
  %656 = select i1 %655, i32 %654, i32 0
  %657 = icmp sgt i32 %656, 200
  br i1 %657, label %695, label %658

658:                                              ; preds = %651
  %659 = sub i32 %593, %656
  %660 = add i32 %659, 1
  %661 = icmp sgt i32 %660, -1
  br i1 %661, label %662, label %667

662:                                              ; preds = %658
  %663 = icmp ugt i32 %660, 40
  %664 = zext nneg i32 %660 to i64
  %665 = shl i64 %597, %664
  %666 = select i1 %663, i64 4611686018427387904, i64 %665
  br label %682

667:                                              ; preds = %658
  %668 = xor i32 %659, -1
  %669 = icmp ult i32 %660, -63
  %670 = zext nneg i32 %668 to i64
  %671 = lshr i64 %597, %670
  %672 = select i1 %669, i64 0, i64 %671
  %673 = shl nsw i64 -1, %670
  %674 = xor i64 %673, -1
  %675 = select i1 %669, i64 2147483647, i64 %674
  %676 = and i64 %675, %597
  %677 = icmp eq i64 %676, 0
  %678 = or i1 %652, %677
  %679 = xor i1 %678, true
  %680 = zext i1 %679 to i64
  %681 = add nuw nsw i64 %672, %680
  br label %682

682:                                              ; preds = %667, %662
  %683 = phi i64 [ %666, %662 ], [ %681, %667 ]
  %684 = icmp ult i64 %683, 4
  br i1 %684, label %689, label %685

685:                                              ; preds = %682
  %686 = add nsw i32 %656, 2
  %687 = sext i32 %686 to i64
  %688 = shl nsw i64 %687, 1
  br label %695

689:                                              ; preds = %682
  %690 = icmp ult i64 %683, 2
  %691 = sext i32 %656 to i64
  %692 = shl nsw i64 %691, 1
  %693 = select i1 %690, i64 0, i64 %692
  %694 = add i64 %693, %683
  br label %695

695:                                              ; preds = %689, %685, %651, %610
  %696 = phi i64 [ 0, %610 ], [ 1099511627776, %651 ], [ %688, %685 ], [ %694, %689 ]
  %697 = tail call i64 @llvm.smin.i64(i64 %696, i64 7)
  %698 = trunc i64 %697 to i32
  br label %699

699:                                              ; preds = %695, %608
  %700 = phi i32 [ 7, %608 ], [ %698, %695 ]
  %701 = or i32 %700, %601
  %702 = and i32 %1, 8
  %703 = icmp ne i32 %702, 0
  %704 = icmp eq i32 %701, 8
  %705 = select i1 %703, i1 %704, i1 false
  %706 = shl i32 %701, 4
  %707 = select i1 %705, i32 0, i32 %706
  br label %708

708:                                              ; preds = %592, %603, %699
  %709 = phi i32 [ 0, %603 ], [ %707, %699 ], [ 112, %592 ]
  %710 = or i32 %579, %709
  %711 = lshr i32 %5, 23
  %712 = and i32 %5, 8388607
  %713 = trunc i32 %711 to i8
  switch i8 %713, label %717 [
    i8 -1, label %714
    i8 0, label %721
  ]

714:                                              ; preds = %708
  %715 = icmp eq i32 %712, 0
  %716 = icmp ne i32 %712, 0
  br label %721

717:                                              ; preds = %708
  %718 = and i32 %711, 255
  %719 = or disjoint i32 %712, 8388608
  %720 = add nsw i32 %718, -150
  br label %721

721:                                              ; preds = %708, %714, %717
  %722 = phi i32 [ 0, %714 ], [ %720, %717 ], [ -149, %708 ]
  %723 = phi i32 [ 0, %714 ], [ %719, %717 ], [ %712, %708 ]
  %724 = phi i1 [ %716, %714 ], [ true, %717 ], [ true, %708 ]
  %725 = phi i1 [ %715, %714 ], [ true, %717 ], [ true, %708 ]
  %726 = zext nneg i32 %723 to i64
  %727 = and i32 %6, 255
  %728 = lshr i32 %5, 28
  %729 = and i32 %728, 8
  br i1 %725, label %730, label %833

730:                                              ; preds = %721
  %731 = and i32 %1, 4
  %732 = icmp ne i32 %731, 0
  %733 = icmp slt i32 %5, 0
  %734 = and i1 %732, %733
  br i1 %734, label %833, label %735

735:                                              ; preds = %730
  br i1 %724, label %736, label %825

736:                                              ; preds = %735
  %737 = icmp eq i32 %723, 0
  br i1 %737, label %821, label %738

738:                                              ; preds = %736, %738
  %739 = phi i32 [ %742, %738 ], [ 0, %736 ]
  %740 = phi i64 [ %741, %738 ], [ %726, %736 ]
  %741 = lshr i64 %740, 1
  %742 = add nuw nsw i32 %739, 1
  %743 = icmp ult i64 %740, 2
  br i1 %743, label %744, label %738, !llvm.loop !20003

744:                                              ; preds = %738
  %745 = add nsw i32 %742, %722
  %746 = add i32 %739, %722
  %747 = icmp sgt i32 %745, 0
  %748 = select i1 %747, i32 %746, i32 0
  %749 = xor i32 %722, -1
  %750 = add i32 %748, %749
  %751 = icmp slt i32 %750, 1
  br i1 %751, label %766, label %752

752:                                              ; preds = %744
  %753 = icmp ult i32 %750, 8
  br i1 %753, label %761, label %754

754:                                              ; preds = %752
  %755 = add nsw i32 %750, -8
  %756 = icmp ugt i32 %755, 63
  %757 = zext nneg i32 %755 to i64
  %758 = lshr i64 %726, %757
  %759 = and i64 %758, 255
  %760 = select i1 %756, i64 0, i64 %759
  br label %766

761:                                              ; preds = %752
  %762 = sub nuw nsw i32 8, %750
  %763 = zext nneg i32 %762 to i64
  %764 = shl nuw nsw i64 %726, %763
  %765 = and i64 %764, 254
  br label %766

766:                                              ; preds = %754, %761, %744
  %767 = phi i64 [ %765, %761 ], [ 0, %744 ], [ %760, %754 ]
  %768 = zext nneg i32 %727 to i64
  %769 = add nuw nsw i64 %767, %768
  %770 = icmp ult i64 %769, 256
  br label %771

771:                                              ; preds = %771, %766
  %772 = phi i32 [ %775, %771 ], [ 0, %766 ]
  %773 = phi i64 [ %774, %771 ], [ %726, %766 ]
  %774 = lshr i64 %773, 1
  %775 = add nuw nsw i32 %772, 1
  %776 = icmp ult i64 %773, 2
  br i1 %776, label %777, label %771, !llvm.loop !20003

777:                                              ; preds = %771
  %778 = select i1 %751, i1 true, i1 %770
  %779 = add nsw i32 %775, %722
  %780 = add i32 %772, %722
  %781 = icmp sgt i32 %779, 0
  %782 = select i1 %781, i32 %780, i32 0
  %783 = icmp sgt i32 %782, 200
  br i1 %783, label %821, label %784

784:                                              ; preds = %777
  %785 = sub i32 %722, %782
  %786 = add i32 %785, 1
  %787 = icmp sgt i32 %786, -1
  br i1 %787, label %788, label %793

788:                                              ; preds = %784
  %789 = icmp ugt i32 %786, 40
  %790 = zext nneg i32 %786 to i64
  %791 = shl i64 %726, %790
  %792 = select i1 %789, i64 4611686018427387904, i64 %791
  br label %808

793:                                              ; preds = %784
  %794 = xor i32 %785, -1
  %795 = icmp ult i32 %786, -63
  %796 = zext nneg i32 %794 to i64
  %797 = lshr i64 %726, %796
  %798 = select i1 %795, i64 0, i64 %797
  %799 = shl nsw i64 -1, %796
  %800 = xor i64 %799, -1
  %801 = select i1 %795, i64 2147483647, i64 %800
  %802 = and i64 %801, %726
  %803 = icmp eq i64 %802, 0
  %804 = or i1 %778, %803
  %805 = xor i1 %804, true
  %806 = zext i1 %805 to i64
  %807 = add nuw nsw i64 %798, %806
  br label %808

808:                                              ; preds = %793, %788
  %809 = phi i64 [ %792, %788 ], [ %807, %793 ]
  %810 = icmp ult i64 %809, 4
  br i1 %810, label %815, label %811

811:                                              ; preds = %808
  %812 = add nsw i32 %782, 2
  %813 = sext i32 %812 to i64
  %814 = shl nsw i64 %813, 1
  br label %821

815:                                              ; preds = %808
  %816 = icmp ult i64 %809, 2
  %817 = sext i32 %782 to i64
  %818 = shl nsw i64 %817, 1
  %819 = select i1 %816, i64 0, i64 %818
  %820 = add i64 %819, %809
  br label %821

821:                                              ; preds = %815, %811, %777, %736
  %822 = phi i64 [ 0, %736 ], [ 1099511627776, %777 ], [ %814, %811 ], [ %820, %815 ]
  %823 = tail call i64 @llvm.smin.i64(i64 %822, i64 7)
  %824 = trunc i64 %823 to i32
  br label %825

825:                                              ; preds = %821, %735
  %826 = phi i32 [ 7, %735 ], [ %824, %821 ]
  %827 = or i32 %826, %729
  %828 = and i32 %1, 8
  %829 = icmp ne i32 %828, 0
  %830 = icmp eq i32 %827, 8
  %831 = select i1 %829, i1 %830, i1 false
  %832 = select i1 %831, i32 0, i32 %827
  br label %833

833:                                              ; preds = %721, %730, %825
  %834 = phi i32 [ 0, %730 ], [ %832, %825 ], [ 7, %721 ]
  %835 = or i32 %710, %834
  br label %1525

836:                                              ; preds = %317
  switch i8 %321, label %842 [
    i8 -1, label %837
    i8 0, label %846
  ]

837:                                              ; preds = %836
  %838 = icmp ne i32 %320, 0
  %839 = zext i1 %838 to i32
  %840 = icmp eq i32 %320, 0
  %841 = zext i1 %840 to i32
  br label %846

842:                                              ; preds = %836
  %843 = and i32 %319, 255
  %844 = or disjoint i32 %320, 8388608
  %845 = add nsw i32 %843, -150
  br label %846

846:                                              ; preds = %836, %837, %842
  %847 = phi i32 [ 0, %837 ], [ %845, %842 ], [ -149, %836 ]
  %848 = phi i32 [ 0, %837 ], [ %844, %842 ], [ %320, %836 ]
  %849 = phi i32 [ %841, %837 ], [ 0, %842 ], [ 0, %836 ]
  %850 = phi i32 [ %839, %837 ], [ 0, %842 ], [ 0, %836 ]
  %851 = zext nneg i32 %848 to i64
  %852 = lshr i32 %6, 16
  %853 = and i32 %0, 6
  %854 = icmp eq i32 %853, 2
  %855 = select i1 %854, i32 32, i32 128
  %856 = icmp slt i32 %2, 0
  %857 = select i1 %856, i32 %855, i32 0
  %858 = icmp eq i32 %850, 0
  br i1 %858, label %864, label %859

859:                                              ; preds = %846
  %860 = icmp ult i32 %0, 2
  %861 = icmp eq i32 %0, 5
  %862 = select i1 %861, i32 254, i32 31
  %863 = select i1 %860, i32 127, i32 %862
  br label %1006

864:                                              ; preds = %846
  %865 = and i32 %1, 4
  %866 = icmp ne i32 %865, 0
  %867 = and i1 %866, %856
  br i1 %867, label %1006, label %868

868:                                              ; preds = %864
  %869 = icmp eq i32 %849, 0
  br i1 %869, label %876, label %870

870:                                              ; preds = %868
  switch i32 %0, label %871 [
    i32 0, label %998
    i32 1, label %874
    i32 4, label %875
  ]

871:                                              ; preds = %870
  %872 = icmp eq i32 %0, 5
  %873 = select i1 %872, i32 254, i32 31
  br label %998

874:                                              ; preds = %870
  br label %998

875:                                              ; preds = %870
  br label %998

876:                                              ; preds = %868
  %877 = icmp eq i32 %848, 0
  br i1 %877, label %980, label %878

878:                                              ; preds = %876
  switch i32 %0, label %879 [
    i32 0, label %883
    i32 4, label %882
    i32 2, label %882
  ]

879:                                              ; preds = %878
  %880 = icmp eq i32 %0, 3
  %881 = select i1 %880, i32 2, i32 4
  br label %883

882:                                              ; preds = %878, %878
  br label %883

883:                                              ; preds = %882, %879, %878
  %884 = phi i32 [ 3, %878 ], [ %881, %879 ], [ 1, %882 ]
  %885 = and i32 %0, 5
  %886 = icmp eq i32 %885, 1
  %887 = select i1 %886, i32 -2, i32 -3
  br label %888

888:                                              ; preds = %888, %883
  %889 = phi i32 [ %892, %888 ], [ 0, %883 ]
  %890 = phi i64 [ %891, %888 ], [ %851, %883 ]
  %891 = lshr i64 %890, 1
  %892 = add nuw nsw i32 %889, 1
  %893 = icmp ult i64 %890, 2
  br i1 %893, label %894, label %888, !llvm.loop !20003

894:                                              ; preds = %888
  %895 = select i1 %886, i32 2, i32 3
  %896 = shl nsw i32 -1, %884
  %897 = add nsw i32 %892, %847
  %898 = add i32 %889, %847
  %899 = add nsw i32 %896, 2
  %900 = icmp sgt i32 %897, %899
  %901 = select i1 %900, i32 %898, i32 %899
  %902 = sub nsw i32 %887, %847
  %903 = add i32 %902, %901
  %904 = icmp slt i32 %903, 1
  br i1 %904, label %919, label %905

905:                                              ; preds = %894
  %906 = icmp ult i32 %903, 16
  br i1 %906, label %914, label %907

907:                                              ; preds = %905
  %908 = add nsw i32 %903, -16
  %909 = icmp ugt i32 %908, 63
  %910 = zext nneg i32 %908 to i64
  %911 = lshr i64 %851, %910
  %912 = and i64 %911, 65535
  %913 = select i1 %909, i64 0, i64 %912
  br label %919

914:                                              ; preds = %905
  %915 = sub nuw nsw i32 16, %903
  %916 = zext nneg i32 %915 to i64
  %917 = shl nuw nsw i64 %851, %916
  %918 = and i64 %917, 65534
  br label %919

919:                                              ; preds = %907, %914, %894
  %920 = phi i64 [ %918, %914 ], [ 0, %894 ], [ %913, %907 ]
  %921 = zext nneg i32 %852 to i64
  %922 = add nuw nsw i64 %920, %921
  %923 = icmp ult i64 %922, 65536
  br label %924

924:                                              ; preds = %924, %919
  %925 = phi i32 [ %928, %924 ], [ 0, %919 ]
  %926 = phi i64 [ %927, %924 ], [ %851, %919 ]
  %927 = lshr i64 %926, 1
  %928 = add nuw nsw i32 %925, 1
  %929 = icmp ult i64 %926, 2
  br i1 %929, label %930, label %924, !llvm.loop !20003

930:                                              ; preds = %924
  %931 = select i1 %904, i1 true, i1 %923
  %932 = xor i32 %896, -1
  %933 = add nsw i32 %928, %847
  %934 = add i32 %925, %847
  %935 = icmp sgt i32 %933, %899
  %936 = select i1 %935, i32 %934, i32 %899
  %937 = icmp sgt i32 %936, 200
  br i1 %937, label %980, label %938

938:                                              ; preds = %930
  %939 = sub i32 %895, %936
  %940 = add i32 %939, %847
  %941 = icmp sgt i32 %940, -1
  br i1 %941, label %942, label %947

942:                                              ; preds = %938
  %943 = icmp ugt i32 %940, 40
  %944 = zext nneg i32 %940 to i64
  %945 = shl i64 %851, %944
  %946 = select i1 %943, i64 4611686018427387904, i64 %945
  br label %962

947:                                              ; preds = %938
  %948 = sub nsw i32 0, %940
  %949 = icmp ult i32 %940, -63
  %950 = zext nneg i32 %948 to i64
  %951 = lshr i64 %851, %950
  %952 = select i1 %949, i64 0, i64 %951
  %953 = shl nsw i64 -1, %950
  %954 = xor i64 %953, -1
  %955 = select i1 %949, i64 2147483647, i64 %954
  %956 = and i64 %955, %851
  %957 = icmp eq i64 %956, 0
  %958 = or i1 %931, %957
  %959 = xor i1 %958, true
  %960 = zext i1 %959 to i64
  %961 = add nuw nsw i64 %952, %960
  br label %962

962:                                              ; preds = %947, %942
  %963 = phi i64 [ %946, %942 ], [ %961, %947 ]
  %964 = zext nneg i32 %895 to i64
  %965 = shl nuw nsw i64 2, %964
  %966 = icmp ult i64 %963, %965
  br i1 %966, label %971, label %967

967:                                              ; preds = %962
  %968 = sub i32 %936, %896
  %969 = sext i32 %968 to i64
  %970 = shl nsw i64 %969, %964
  br label %980

971:                                              ; preds = %962
  %972 = lshr i64 %963, %964
  %973 = icmp eq i64 %972, 0
  %974 = add nsw i32 %936, %932
  %975 = sext i32 %974 to i64
  %976 = add nsw i64 %975, -1
  %977 = shl nsw i64 %976, %964
  %978 = select i1 %973, i64 0, i64 %977
  %979 = add i64 %978, %963
  br label %980

980:                                              ; preds = %971, %967, %930, %876
  %981 = phi i64 [ 0, %876 ], [ 1099511627776, %930 ], [ %970, %967 ], [ %979, %971 ]
  switch i32 %0, label %982 [
    i32 0, label %987
    i32 1, label %985
    i32 4, label %986
  ]

982:                                              ; preds = %980
  %983 = icmp eq i32 %0, 5
  %984 = select i1 %983, i64 254, i64 31
  br label %987

985:                                              ; preds = %980
  br label %987

986:                                              ; preds = %980
  br label %987

987:                                              ; preds = %986, %985, %982, %980
  %988 = phi i64 [ 126, %980 ], [ %984, %982 ], [ 123, %985 ], [ 7, %986 ]
  %989 = icmp sgt i64 %981, %988
  br i1 %989, label %990, label %996

990:                                              ; preds = %987
  switch i32 %0, label %991 [
    i32 0, label %998
    i32 1, label %994
    i32 4, label %995
  ]

991:                                              ; preds = %990
  %992 = icmp eq i32 %0, 5
  %993 = select i1 %992, i32 254, i32 31
  br label %998

994:                                              ; preds = %990
  br label %998

995:                                              ; preds = %990
  br label %998

996:                                              ; preds = %987
  %997 = trunc i64 %981 to i32
  br label %998

998:                                              ; preds = %996, %995, %994, %991, %990, %875, %874, %871, %870
  %999 = phi i32 [ %997, %996 ], [ 126, %870 ], [ %873, %871 ], [ 123, %874 ], [ 7, %875 ], [ 126, %990 ], [ %993, %991 ], [ 123, %994 ], [ 7, %995 ]
  %1000 = or i32 %999, %857
  %1001 = and i32 %1, 8
  %1002 = icmp ne i32 %1001, 0
  %1003 = icmp eq i32 %1000, %855
  %1004 = select i1 %1002, i1 %1003, i1 false
  %1005 = select i1 %1004, i32 0, i32 %1000
  br label %1006

1006:                                             ; preds = %859, %864, %998
  %1007 = phi i32 [ 0, %864 ], [ %1005, %998 ], [ %863, %859 ]
  %1008 = shl i32 %1007, 24
  %1009 = lshr i32 %3, 23
  %1010 = and i32 %3, 8388607
  %1011 = trunc i32 %1009 to i8
  switch i8 %1011, label %1017 [
    i8 -1, label %1012
    i8 0, label %1021
  ]

1012:                                             ; preds = %1006
  %1013 = icmp ne i32 %1010, 0
  %1014 = zext i1 %1013 to i32
  %1015 = icmp eq i32 %1010, 0
  %1016 = zext i1 %1015 to i32
  br label %1021

1017:                                             ; preds = %1006
  %1018 = and i32 %1009, 255
  %1019 = or disjoint i32 %1010, 8388608
  %1020 = add nsw i32 %1018, -150
  br label %1021

1021:                                             ; preds = %1006, %1012, %1017
  %1022 = phi i32 [ 0, %1012 ], [ %1020, %1017 ], [ -149, %1006 ]
  %1023 = phi i32 [ 0, %1012 ], [ %1019, %1017 ], [ %1010, %1006 ]
  %1024 = phi i32 [ %1016, %1012 ], [ 0, %1017 ], [ 0, %1006 ]
  %1025 = phi i32 [ %1014, %1012 ], [ 0, %1017 ], [ 0, %1006 ]
  %1026 = zext nneg i32 %1023 to i64
  %1027 = icmp slt i32 %3, 0
  %1028 = select i1 %1027, i32 %855, i32 0
  %1029 = icmp eq i32 %1025, 0
  br i1 %1029, label %1035, label %1030

1030:                                             ; preds = %1021
  %1031 = icmp ult i32 %0, 2
  %1032 = icmp eq i32 %0, 5
  %1033 = select i1 %1032, i32 254, i32 31
  %1034 = select i1 %1031, i32 127, i32 %1033
  br label %1177

1035:                                             ; preds = %1021
  %1036 = and i32 %1, 4
  %1037 = icmp ne i32 %1036, 0
  %1038 = and i1 %1037, %1027
  br i1 %1038, label %1177, label %1039

1039:                                             ; preds = %1035
  %1040 = icmp eq i32 %1024, 0
  br i1 %1040, label %1047, label %1041

1041:                                             ; preds = %1039
  switch i32 %0, label %1042 [
    i32 0, label %1169
    i32 1, label %1045
    i32 4, label %1046
  ]

1042:                                             ; preds = %1041
  %1043 = icmp eq i32 %0, 5
  %1044 = select i1 %1043, i32 254, i32 31
  br label %1169

1045:                                             ; preds = %1041
  br label %1169

1046:                                             ; preds = %1041
  br label %1169

1047:                                             ; preds = %1039
  %1048 = icmp eq i32 %1023, 0
  br i1 %1048, label %1151, label %1049

1049:                                             ; preds = %1047
  switch i32 %0, label %1050 [
    i32 0, label %1054
    i32 4, label %1053
    i32 2, label %1053
  ]

1050:                                             ; preds = %1049
  %1051 = icmp eq i32 %0, 3
  %1052 = select i1 %1051, i32 2, i32 4
  br label %1054

1053:                                             ; preds = %1049, %1049
  br label %1054

1054:                                             ; preds = %1053, %1050, %1049
  %1055 = phi i32 [ 3, %1049 ], [ %1052, %1050 ], [ 1, %1053 ]
  %1056 = and i32 %0, 5
  %1057 = icmp eq i32 %1056, 1
  %1058 = select i1 %1057, i32 -2, i32 -3
  br label %1059

1059:                                             ; preds = %1059, %1054
  %1060 = phi i32 [ %1063, %1059 ], [ 0, %1054 ]
  %1061 = phi i64 [ %1062, %1059 ], [ %1026, %1054 ]
  %1062 = lshr i64 %1061, 1
  %1063 = add nuw nsw i32 %1060, 1
  %1064 = icmp ult i64 %1061, 2
  br i1 %1064, label %1065, label %1059, !llvm.loop !20003

1065:                                             ; preds = %1059
  %1066 = select i1 %1057, i32 2, i32 3
  %1067 = shl nsw i32 -1, %1055
  %1068 = add nsw i32 %1063, %1022
  %1069 = add i32 %1060, %1022
  %1070 = add nsw i32 %1067, 2
  %1071 = icmp sgt i32 %1068, %1070
  %1072 = select i1 %1071, i32 %1069, i32 %1070
  %1073 = sub nsw i32 %1058, %1022
  %1074 = add i32 %1073, %1072
  %1075 = icmp slt i32 %1074, 1
  br i1 %1075, label %1090, label %1076

1076:                                             ; preds = %1065
  %1077 = icmp ult i32 %1074, 16
  br i1 %1077, label %1085, label %1078

1078:                                             ; preds = %1076
  %1079 = add nsw i32 %1074, -16
  %1080 = icmp ugt i32 %1079, 63
  %1081 = zext nneg i32 %1079 to i64
  %1082 = lshr i64 %1026, %1081
  %1083 = and i64 %1082, 65535
  %1084 = select i1 %1080, i64 0, i64 %1083
  br label %1090

1085:                                             ; preds = %1076
  %1086 = sub nuw nsw i32 16, %1074
  %1087 = zext nneg i32 %1086 to i64
  %1088 = shl nuw nsw i64 %1026, %1087
  %1089 = and i64 %1088, 65534
  br label %1090

1090:                                             ; preds = %1078, %1085, %1065
  %1091 = phi i64 [ %1089, %1085 ], [ 0, %1065 ], [ %1084, %1078 ]
  %1092 = zext nneg i32 %852 to i64
  %1093 = add nuw nsw i64 %1091, %1092
  %1094 = icmp ult i64 %1093, 65536
  br label %1095

1095:                                             ; preds = %1095, %1090
  %1096 = phi i32 [ %1099, %1095 ], [ 0, %1090 ]
  %1097 = phi i64 [ %1098, %1095 ], [ %1026, %1090 ]
  %1098 = lshr i64 %1097, 1
  %1099 = add nuw nsw i32 %1096, 1
  %1100 = icmp ult i64 %1097, 2
  br i1 %1100, label %1101, label %1095, !llvm.loop !20003

1101:                                             ; preds = %1095
  %1102 = select i1 %1075, i1 true, i1 %1094
  %1103 = xor i32 %1067, -1
  %1104 = add nsw i32 %1099, %1022
  %1105 = add i32 %1096, %1022
  %1106 = icmp sgt i32 %1104, %1070
  %1107 = select i1 %1106, i32 %1105, i32 %1070
  %1108 = icmp sgt i32 %1107, 200
  br i1 %1108, label %1151, label %1109

1109:                                             ; preds = %1101
  %1110 = sub i32 %1066, %1107
  %1111 = add i32 %1110, %1022
  %1112 = icmp sgt i32 %1111, -1
  br i1 %1112, label %1113, label %1118

1113:                                             ; preds = %1109
  %1114 = icmp ugt i32 %1111, 40
  %1115 = zext nneg i32 %1111 to i64
  %1116 = shl i64 %1026, %1115
  %1117 = select i1 %1114, i64 4611686018427387904, i64 %1116
  br label %1133

1118:                                             ; preds = %1109
  %1119 = sub nsw i32 0, %1111
  %1120 = icmp ult i32 %1111, -63
  %1121 = zext nneg i32 %1119 to i64
  %1122 = lshr i64 %1026, %1121
  %1123 = select i1 %1120, i64 0, i64 %1122
  %1124 = shl nsw i64 -1, %1121
  %1125 = xor i64 %1124, -1
  %1126 = select i1 %1120, i64 2147483647, i64 %1125
  %1127 = and i64 %1126, %1026
  %1128 = icmp eq i64 %1127, 0
  %1129 = or i1 %1102, %1128
  %1130 = xor i1 %1129, true
  %1131 = zext i1 %1130 to i64
  %1132 = add nuw nsw i64 %1123, %1131
  br label %1133

1133:                                             ; preds = %1118, %1113
  %1134 = phi i64 [ %1117, %1113 ], [ %1132, %1118 ]
  %1135 = zext nneg i32 %1066 to i64
  %1136 = shl nuw nsw i64 2, %1135
  %1137 = icmp ult i64 %1134, %1136
  br i1 %1137, label %1142, label %1138

1138:                                             ; preds = %1133
  %1139 = sub i32 %1107, %1067
  %1140 = sext i32 %1139 to i64
  %1141 = shl nsw i64 %1140, %1135
  br label %1151

1142:                                             ; preds = %1133
  %1143 = lshr i64 %1134, %1135
  %1144 = icmp eq i64 %1143, 0
  %1145 = add nsw i32 %1107, %1103
  %1146 = sext i32 %1145 to i64
  %1147 = add nsw i64 %1146, -1
  %1148 = shl nsw i64 %1147, %1135
  %1149 = select i1 %1144, i64 0, i64 %1148
  %1150 = add i64 %1149, %1134
  br label %1151

1151:                                             ; preds = %1142, %1138, %1101, %1047
  %1152 = phi i64 [ 0, %1047 ], [ 1099511627776, %1101 ], [ %1141, %1138 ], [ %1150, %1142 ]
  switch i32 %0, label %1153 [
    i32 0, label %1158
    i32 1, label %1156
    i32 4, label %1157
  ]

1153:                                             ; preds = %1151
  %1154 = icmp eq i32 %0, 5
  %1155 = select i1 %1154, i64 254, i64 31
  br label %1158

1156:                                             ; preds = %1151
  br label %1158

1157:                                             ; preds = %1151
  br label %1158

1158:                                             ; preds = %1157, %1156, %1153, %1151
  %1159 = phi i64 [ 126, %1151 ], [ %1155, %1153 ], [ 123, %1156 ], [ 7, %1157 ]
  %1160 = icmp sgt i64 %1152, %1159
  br i1 %1160, label %1161, label %1167

1161:                                             ; preds = %1158
  switch i32 %0, label %1162 [
    i32 0, label %1169
    i32 1, label %1165
    i32 4, label %1166
  ]

1162:                                             ; preds = %1161
  %1163 = icmp eq i32 %0, 5
  %1164 = select i1 %1163, i32 254, i32 31
  br label %1169

1165:                                             ; preds = %1161
  br label %1169

1166:                                             ; preds = %1161
  br label %1169

1167:                                             ; preds = %1158
  %1168 = trunc i64 %1152 to i32
  br label %1169

1169:                                             ; preds = %1167, %1166, %1165, %1162, %1161, %1046, %1045, %1042, %1041
  %1170 = phi i32 [ %1168, %1167 ], [ 126, %1041 ], [ %1044, %1042 ], [ 123, %1045 ], [ 7, %1046 ], [ 126, %1161 ], [ %1164, %1162 ], [ 123, %1165 ], [ 7, %1166 ]
  %1171 = or i32 %1170, %1028
  %1172 = and i32 %1, 8
  %1173 = icmp ne i32 %1172, 0
  %1174 = icmp eq i32 %1171, %855
  %1175 = select i1 %1173, i1 %1174, i1 false
  %1176 = select i1 %1175, i32 0, i32 %1171
  br label %1177

1177:                                             ; preds = %1030, %1035, %1169
  %1178 = phi i32 [ 0, %1035 ], [ %1176, %1169 ], [ %1034, %1030 ]
  %1179 = shl i32 %1178, 16
  %1180 = or i32 %1179, %1008
  %1181 = lshr i32 %4, 23
  %1182 = and i32 %4, 8388607
  %1183 = trunc i32 %1181 to i8
  switch i8 %1183, label %1189 [
    i8 -1, label %1184
    i8 0, label %1193
  ]

1184:                                             ; preds = %1177
  %1185 = icmp ne i32 %1182, 0
  %1186 = zext i1 %1185 to i32
  %1187 = icmp eq i32 %1182, 0
  %1188 = zext i1 %1187 to i32
  br label %1193

1189:                                             ; preds = %1177
  %1190 = and i32 %1181, 255
  %1191 = or disjoint i32 %1182, 8388608
  %1192 = add nsw i32 %1190, -150
  br label %1193

1193:                                             ; preds = %1177, %1184, %1189
  %1194 = phi i32 [ 0, %1184 ], [ %1192, %1189 ], [ -149, %1177 ]
  %1195 = phi i32 [ 0, %1184 ], [ %1191, %1189 ], [ %1182, %1177 ]
  %1196 = phi i32 [ %1188, %1184 ], [ 0, %1189 ], [ 0, %1177 ]
  %1197 = phi i32 [ %1186, %1184 ], [ 0, %1189 ], [ 0, %1177 ]
  %1198 = zext nneg i32 %1195 to i64
  %1199 = and i32 %6, 65535
  %1200 = icmp slt i32 %4, 0
  %1201 = select i1 %1200, i32 %855, i32 0
  %1202 = icmp eq i32 %1197, 0
  br i1 %1202, label %1208, label %1203

1203:                                             ; preds = %1193
  %1204 = icmp ult i32 %0, 2
  %1205 = icmp eq i32 %0, 5
  %1206 = select i1 %1205, i32 254, i32 31
  %1207 = select i1 %1204, i32 127, i32 %1206
  br label %1350

1208:                                             ; preds = %1193
  %1209 = and i32 %1, 4
  %1210 = icmp ne i32 %1209, 0
  %1211 = and i1 %1210, %1200
  br i1 %1211, label %1350, label %1212

1212:                                             ; preds = %1208
  %1213 = icmp eq i32 %1196, 0
  br i1 %1213, label %1220, label %1214

1214:                                             ; preds = %1212
  switch i32 %0, label %1215 [
    i32 0, label %1342
    i32 1, label %1218
    i32 4, label %1219
  ]

1215:                                             ; preds = %1214
  %1216 = icmp eq i32 %0, 5
  %1217 = select i1 %1216, i32 254, i32 31
  br label %1342

1218:                                             ; preds = %1214
  br label %1342

1219:                                             ; preds = %1214
  br label %1342

1220:                                             ; preds = %1212
  %1221 = icmp eq i32 %1195, 0
  br i1 %1221, label %1324, label %1222

1222:                                             ; preds = %1220
  switch i32 %0, label %1223 [
    i32 0, label %1227
    i32 4, label %1226
    i32 2, label %1226
  ]

1223:                                             ; preds = %1222
  %1224 = icmp eq i32 %0, 3
  %1225 = select i1 %1224, i32 2, i32 4
  br label %1227

1226:                                             ; preds = %1222, %1222
  br label %1227

1227:                                             ; preds = %1226, %1223, %1222
  %1228 = phi i32 [ 3, %1222 ], [ %1225, %1223 ], [ 1, %1226 ]
  %1229 = and i32 %0, 5
  %1230 = icmp eq i32 %1229, 1
  %1231 = select i1 %1230, i32 -2, i32 -3
  br label %1232

1232:                                             ; preds = %1232, %1227
  %1233 = phi i32 [ %1236, %1232 ], [ 0, %1227 ]
  %1234 = phi i64 [ %1235, %1232 ], [ %1198, %1227 ]
  %1235 = lshr i64 %1234, 1
  %1236 = add nuw nsw i32 %1233, 1
  %1237 = icmp ult i64 %1234, 2
  br i1 %1237, label %1238, label %1232, !llvm.loop !20003

1238:                                             ; preds = %1232
  %1239 = select i1 %1230, i32 2, i32 3
  %1240 = shl nsw i32 -1, %1228
  %1241 = add nsw i32 %1236, %1194
  %1242 = add i32 %1233, %1194
  %1243 = add nsw i32 %1240, 2
  %1244 = icmp sgt i32 %1241, %1243
  %1245 = select i1 %1244, i32 %1242, i32 %1243
  %1246 = sub nsw i32 %1231, %1194
  %1247 = add i32 %1246, %1245
  %1248 = icmp slt i32 %1247, 1
  br i1 %1248, label %1263, label %1249

1249:                                             ; preds = %1238
  %1250 = icmp ult i32 %1247, 16
  br i1 %1250, label %1258, label %1251

1251:                                             ; preds = %1249
  %1252 = add nsw i32 %1247, -16
  %1253 = icmp ugt i32 %1252, 63
  %1254 = zext nneg i32 %1252 to i64
  %1255 = lshr i64 %1198, %1254
  %1256 = and i64 %1255, 65535
  %1257 = select i1 %1253, i64 0, i64 %1256
  br label %1263

1258:                                             ; preds = %1249
  %1259 = sub nuw nsw i32 16, %1247
  %1260 = zext nneg i32 %1259 to i64
  %1261 = shl nuw nsw i64 %1198, %1260
  %1262 = and i64 %1261, 65534
  br label %1263

1263:                                             ; preds = %1251, %1258, %1238
  %1264 = phi i64 [ %1262, %1258 ], [ 0, %1238 ], [ %1257, %1251 ]
  %1265 = zext nneg i32 %1199 to i64
  %1266 = add nuw nsw i64 %1264, %1265
  %1267 = icmp ult i64 %1266, 65536
  br label %1268

1268:                                             ; preds = %1268, %1263
  %1269 = phi i32 [ %1272, %1268 ], [ 0, %1263 ]
  %1270 = phi i64 [ %1271, %1268 ], [ %1198, %1263 ]
  %1271 = lshr i64 %1270, 1
  %1272 = add nuw nsw i32 %1269, 1
  %1273 = icmp ult i64 %1270, 2
  br i1 %1273, label %1274, label %1268, !llvm.loop !20003

1274:                                             ; preds = %1268
  %1275 = select i1 %1248, i1 true, i1 %1267
  %1276 = xor i32 %1240, -1
  %1277 = add nsw i32 %1272, %1194
  %1278 = add i32 %1269, %1194
  %1279 = icmp sgt i32 %1277, %1243
  %1280 = select i1 %1279, i32 %1278, i32 %1243
  %1281 = icmp sgt i32 %1280, 200
  br i1 %1281, label %1324, label %1282

1282:                                             ; preds = %1274
  %1283 = sub i32 %1239, %1280
  %1284 = add i32 %1283, %1194
  %1285 = icmp sgt i32 %1284, -1
  br i1 %1285, label %1286, label %1291

1286:                                             ; preds = %1282
  %1287 = icmp ugt i32 %1284, 40
  %1288 = zext nneg i32 %1284 to i64
  %1289 = shl i64 %1198, %1288
  %1290 = select i1 %1287, i64 4611686018427387904, i64 %1289
  br label %1306

1291:                                             ; preds = %1282
  %1292 = sub nsw i32 0, %1284
  %1293 = icmp ult i32 %1284, -63
  %1294 = zext nneg i32 %1292 to i64
  %1295 = lshr i64 %1198, %1294
  %1296 = select i1 %1293, i64 0, i64 %1295
  %1297 = shl nsw i64 -1, %1294
  %1298 = xor i64 %1297, -1
  %1299 = select i1 %1293, i64 2147483647, i64 %1298
  %1300 = and i64 %1299, %1198
  %1301 = icmp eq i64 %1300, 0
  %1302 = or i1 %1275, %1301
  %1303 = xor i1 %1302, true
  %1304 = zext i1 %1303 to i64
  %1305 = add nuw nsw i64 %1296, %1304
  br label %1306

1306:                                             ; preds = %1291, %1286
  %1307 = phi i64 [ %1290, %1286 ], [ %1305, %1291 ]
  %1308 = zext nneg i32 %1239 to i64
  %1309 = shl nuw nsw i64 2, %1308
  %1310 = icmp ult i64 %1307, %1309
  br i1 %1310, label %1315, label %1311

1311:                                             ; preds = %1306
  %1312 = sub i32 %1280, %1240
  %1313 = sext i32 %1312 to i64
  %1314 = shl nsw i64 %1313, %1308
  br label %1324

1315:                                             ; preds = %1306
  %1316 = lshr i64 %1307, %1308
  %1317 = icmp eq i64 %1316, 0
  %1318 = add nsw i32 %1280, %1276
  %1319 = sext i32 %1318 to i64
  %1320 = add nsw i64 %1319, -1
  %1321 = shl nsw i64 %1320, %1308
  %1322 = select i1 %1317, i64 0, i64 %1321
  %1323 = add i64 %1322, %1307
  br label %1324

1324:                                             ; preds = %1315, %1311, %1274, %1220
  %1325 = phi i64 [ 0, %1220 ], [ 1099511627776, %1274 ], [ %1314, %1311 ], [ %1323, %1315 ]
  switch i32 %0, label %1326 [
    i32 0, label %1331
    i32 1, label %1329
    i32 4, label %1330
  ]

1326:                                             ; preds = %1324
  %1327 = icmp eq i32 %0, 5
  %1328 = select i1 %1327, i64 254, i64 31
  br label %1331

1329:                                             ; preds = %1324
  br label %1331

1330:                                             ; preds = %1324
  br label %1331

1331:                                             ; preds = %1330, %1329, %1326, %1324
  %1332 = phi i64 [ 126, %1324 ], [ %1328, %1326 ], [ 123, %1329 ], [ 7, %1330 ]
  %1333 = icmp sgt i64 %1325, %1332
  br i1 %1333, label %1334, label %1340

1334:                                             ; preds = %1331
  switch i32 %0, label %1335 [
    i32 0, label %1342
    i32 1, label %1338
    i32 4, label %1339
  ]

1335:                                             ; preds = %1334
  %1336 = icmp eq i32 %0, 5
  %1337 = select i1 %1336, i32 254, i32 31
  br label %1342

1338:                                             ; preds = %1334
  br label %1342

1339:                                             ; preds = %1334
  br label %1342

1340:                                             ; preds = %1331
  %1341 = trunc i64 %1325 to i32
  br label %1342

1342:                                             ; preds = %1340, %1339, %1338, %1335, %1334, %1219, %1218, %1215, %1214
  %1343 = phi i32 [ %1341, %1340 ], [ 126, %1214 ], [ %1217, %1215 ], [ 123, %1218 ], [ 7, %1219 ], [ 126, %1334 ], [ %1337, %1335 ], [ 123, %1338 ], [ 7, %1339 ]
  %1344 = or i32 %1343, %1201
  %1345 = and i32 %1, 8
  %1346 = icmp ne i32 %1345, 0
  %1347 = icmp eq i32 %1344, %855
  %1348 = select i1 %1346, i1 %1347, i1 false
  %1349 = select i1 %1348, i32 0, i32 %1344
  br label %1350

1350:                                             ; preds = %1203, %1208, %1342
  %1351 = phi i32 [ 0, %1208 ], [ %1349, %1342 ], [ %1207, %1203 ]
  %1352 = shl i32 %1351, 8
  %1353 = or i32 %1180, %1352
  %1354 = lshr i32 %5, 23
  %1355 = and i32 %5, 8388607
  %1356 = trunc i32 %1354 to i8
  switch i8 %1356, label %1362 [
    i8 -1, label %1357
    i8 0, label %1366
  ]

1357:                                             ; preds = %1350
  %1358 = icmp ne i32 %1355, 0
  %1359 = zext i1 %1358 to i32
  %1360 = icmp eq i32 %1355, 0
  %1361 = zext i1 %1360 to i32
  br label %1366

1362:                                             ; preds = %1350
  %1363 = and i32 %1354, 255
  %1364 = or disjoint i32 %1355, 8388608
  %1365 = add nsw i32 %1363, -150
  br label %1366

1366:                                             ; preds = %1350, %1357, %1362
  %1367 = phi i32 [ 0, %1357 ], [ %1365, %1362 ], [ -149, %1350 ]
  %1368 = phi i32 [ 0, %1357 ], [ %1364, %1362 ], [ %1355, %1350 ]
  %1369 = phi i32 [ %1361, %1357 ], [ 0, %1362 ], [ 0, %1350 ]
  %1370 = phi i32 [ %1359, %1357 ], [ 0, %1362 ], [ 0, %1350 ]
  %1371 = zext nneg i32 %1368 to i64
  %1372 = icmp slt i32 %5, 0
  %1373 = select i1 %1372, i32 %855, i32 0
  %1374 = icmp eq i32 %1370, 0
  br i1 %1374, label %1380, label %1375

1375:                                             ; preds = %1366
  %1376 = icmp ult i32 %0, 2
  %1377 = icmp eq i32 %0, 5
  %1378 = select i1 %1377, i32 254, i32 31
  %1379 = select i1 %1376, i32 127, i32 %1378
  br label %1522

1380:                                             ; preds = %1366
  %1381 = and i32 %1, 4
  %1382 = icmp ne i32 %1381, 0
  %1383 = and i1 %1382, %1372
  br i1 %1383, label %1522, label %1384

1384:                                             ; preds = %1380
  %1385 = icmp eq i32 %1369, 0
  br i1 %1385, label %1392, label %1386

1386:                                             ; preds = %1384
  switch i32 %0, label %1387 [
    i32 0, label %1514
    i32 1, label %1390
    i32 4, label %1391
  ]

1387:                                             ; preds = %1386
  %1388 = icmp eq i32 %0, 5
  %1389 = select i1 %1388, i32 254, i32 31
  br label %1514

1390:                                             ; preds = %1386
  br label %1514

1391:                                             ; preds = %1386
  br label %1514

1392:                                             ; preds = %1384
  %1393 = icmp eq i32 %1368, 0
  br i1 %1393, label %1496, label %1394

1394:                                             ; preds = %1392
  switch i32 %0, label %1395 [
    i32 0, label %1399
    i32 4, label %1398
    i32 2, label %1398
  ]

1395:                                             ; preds = %1394
  %1396 = icmp eq i32 %0, 3
  %1397 = select i1 %1396, i32 2, i32 4
  br label %1399

1398:                                             ; preds = %1394, %1394
  br label %1399

1399:                                             ; preds = %1398, %1395, %1394
  %1400 = phi i32 [ 3, %1394 ], [ %1397, %1395 ], [ 1, %1398 ]
  %1401 = and i32 %0, 5
  %1402 = icmp eq i32 %1401, 1
  %1403 = select i1 %1402, i32 -2, i32 -3
  br label %1404

1404:                                             ; preds = %1404, %1399
  %1405 = phi i32 [ %1408, %1404 ], [ 0, %1399 ]
  %1406 = phi i64 [ %1407, %1404 ], [ %1371, %1399 ]
  %1407 = lshr i64 %1406, 1
  %1408 = add nuw nsw i32 %1405, 1
  %1409 = icmp ult i64 %1406, 2
  br i1 %1409, label %1410, label %1404, !llvm.loop !20003

1410:                                             ; preds = %1404
  %1411 = select i1 %1402, i32 2, i32 3
  %1412 = shl nsw i32 -1, %1400
  %1413 = add nsw i32 %1408, %1367
  %1414 = add i32 %1405, %1367
  %1415 = add nsw i32 %1412, 2
  %1416 = icmp sgt i32 %1413, %1415
  %1417 = select i1 %1416, i32 %1414, i32 %1415
  %1418 = sub nsw i32 %1403, %1367
  %1419 = add i32 %1418, %1417
  %1420 = icmp slt i32 %1419, 1
  br i1 %1420, label %1435, label %1421

1421:                                             ; preds = %1410
  %1422 = icmp ult i32 %1419, 16
  br i1 %1422, label %1430, label %1423

1423:                                             ; preds = %1421
  %1424 = add nsw i32 %1419, -16
  %1425 = icmp ugt i32 %1424, 63
  %1426 = zext nneg i32 %1424 to i64
  %1427 = lshr i64 %1371, %1426
  %1428 = and i64 %1427, 65535
  %1429 = select i1 %1425, i64 0, i64 %1428
  br label %1435

1430:                                             ; preds = %1421
  %1431 = sub nuw nsw i32 16, %1419
  %1432 = zext nneg i32 %1431 to i64
  %1433 = shl nuw nsw i64 %1371, %1432
  %1434 = and i64 %1433, 65534
  br label %1435

1435:                                             ; preds = %1423, %1430, %1410
  %1436 = phi i64 [ %1434, %1430 ], [ 0, %1410 ], [ %1429, %1423 ]
  %1437 = zext nneg i32 %1199 to i64
  %1438 = add nuw nsw i64 %1436, %1437
  %1439 = icmp ult i64 %1438, 65536
  br label %1440

1440:                                             ; preds = %1440, %1435
  %1441 = phi i32 [ %1444, %1440 ], [ 0, %1435 ]
  %1442 = phi i64 [ %1443, %1440 ], [ %1371, %1435 ]
  %1443 = lshr i64 %1442, 1
  %1444 = add nuw nsw i32 %1441, 1
  %1445 = icmp ult i64 %1442, 2
  br i1 %1445, label %1446, label %1440, !llvm.loop !20003

1446:                                             ; preds = %1440
  %1447 = select i1 %1420, i1 true, i1 %1439
  %1448 = xor i32 %1412, -1
  %1449 = add nsw i32 %1444, %1367
  %1450 = add i32 %1441, %1367
  %1451 = icmp sgt i32 %1449, %1415
  %1452 = select i1 %1451, i32 %1450, i32 %1415
  %1453 = icmp sgt i32 %1452, 200
  br i1 %1453, label %1496, label %1454

1454:                                             ; preds = %1446
  %1455 = sub i32 %1411, %1452
  %1456 = add i32 %1455, %1367
  %1457 = icmp sgt i32 %1456, -1
  br i1 %1457, label %1458, label %1463

1458:                                             ; preds = %1454
  %1459 = icmp ugt i32 %1456, 40
  %1460 = zext nneg i32 %1456 to i64
  %1461 = shl i64 %1371, %1460
  %1462 = select i1 %1459, i64 4611686018427387904, i64 %1461
  br label %1478

1463:                                             ; preds = %1454
  %1464 = sub nsw i32 0, %1456
  %1465 = icmp ult i32 %1456, -63
  %1466 = zext nneg i32 %1464 to i64
  %1467 = lshr i64 %1371, %1466
  %1468 = select i1 %1465, i64 0, i64 %1467
  %1469 = shl nsw i64 -1, %1466
  %1470 = xor i64 %1469, -1
  %1471 = select i1 %1465, i64 2147483647, i64 %1470
  %1472 = and i64 %1471, %1371
  %1473 = icmp eq i64 %1472, 0
  %1474 = or i1 %1447, %1473
  %1475 = xor i1 %1474, true
  %1476 = zext i1 %1475 to i64
  %1477 = add nuw nsw i64 %1468, %1476
  br label %1478

1478:                                             ; preds = %1463, %1458
  %1479 = phi i64 [ %1462, %1458 ], [ %1477, %1463 ]
  %1480 = zext nneg i32 %1411 to i64
  %1481 = shl nuw nsw i64 2, %1480
  %1482 = icmp ult i64 %1479, %1481
  br i1 %1482, label %1487, label %1483

1483:                                             ; preds = %1478
  %1484 = sub i32 %1452, %1412
  %1485 = sext i32 %1484 to i64
  %1486 = shl nsw i64 %1485, %1480
  br label %1496

1487:                                             ; preds = %1478
  %1488 = lshr i64 %1479, %1480
  %1489 = icmp eq i64 %1488, 0
  %1490 = add nsw i32 %1452, %1448
  %1491 = sext i32 %1490 to i64
  %1492 = add nsw i64 %1491, -1
  %1493 = shl nsw i64 %1492, %1480
  %1494 = select i1 %1489, i64 0, i64 %1493
  %1495 = add i64 %1494, %1479
  br label %1496

1496:                                             ; preds = %1487, %1483, %1446, %1392
  %1497 = phi i64 [ 0, %1392 ], [ 1099511627776, %1446 ], [ %1486, %1483 ], [ %1495, %1487 ]
  switch i32 %0, label %1498 [
    i32 0, label %1503
    i32 1, label %1501
    i32 4, label %1502
  ]

1498:                                             ; preds = %1496
  %1499 = icmp eq i32 %0, 5
  %1500 = select i1 %1499, i64 254, i64 31
  br label %1503

1501:                                             ; preds = %1496
  br label %1503

1502:                                             ; preds = %1496
  br label %1503

1503:                                             ; preds = %1502, %1501, %1498, %1496
  %1504 = phi i64 [ 126, %1496 ], [ %1500, %1498 ], [ 123, %1501 ], [ 7, %1502 ]
  %1505 = icmp sgt i64 %1497, %1504
  br i1 %1505, label %1506, label %1512

1506:                                             ; preds = %1503
  switch i32 %0, label %1507 [
    i32 0, label %1514
    i32 1, label %1510
    i32 4, label %1511
  ]

1507:                                             ; preds = %1506
  %1508 = icmp eq i32 %0, 5
  %1509 = select i1 %1508, i32 254, i32 31
  br label %1514

1510:                                             ; preds = %1506
  br label %1514

1511:                                             ; preds = %1506
  br label %1514

1512:                                             ; preds = %1503
  %1513 = trunc i64 %1497 to i32
  br label %1514

1514:                                             ; preds = %1512, %1511, %1510, %1507, %1506, %1391, %1390, %1387, %1386
  %1515 = phi i32 [ %1513, %1512 ], [ 126, %1386 ], [ %1389, %1387 ], [ 123, %1390 ], [ 7, %1391 ], [ 126, %1506 ], [ %1509, %1507 ], [ 123, %1510 ], [ 7, %1511 ]
  %1516 = or i32 %1515, %1373
  %1517 = and i32 %1, 8
  %1518 = icmp ne i32 %1517, 0
  %1519 = icmp eq i32 %1516, %855
  %1520 = select i1 %1518, i1 %1519, i1 false
  %1521 = select i1 %1520, i32 0, i32 %1516
  br label %1522

1522:                                             ; preds = %1375, %1380, %1514
  %1523 = phi i32 [ 0, %1380 ], [ %1521, %1514 ], [ %1379, %1375 ]
  %1524 = or i32 %1353, %1523
  br label %1525

1525:                                             ; preds = %1522, %833, %311
  %1526 = phi i32 [ %316, %311 ], [ %835, %833 ], [ %1524, %1522 ]
  ret i32 %1526
}

; Function Attrs: nofree norecurse nosync nounwind memory(none)
define dso_local i32 @__zluda_ptx_impl_agx_altfp_x4(i32 noundef %0, i32 noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef %4, i32 noundef %5) local_unnamed_addr #3 {
  %7 = and i32 %1, 15
  %8 = lshr i32 %1, 4
  %9 = and i32 %8, 15
  %10 = lshr i32 %1, 8
  %11 = and i32 %10, 15
  %12 = lshr i32 %1, 12
  %13 = and i32 %12, 15
  %14 = icmp eq i32 %7, 1
  %15 = icmp eq i32 %2, 0
  %16 = select i1 %14, i32 123, i32 126
  %17 = icmp eq i32 %9, 2
  %18 = select i1 %17, i32 2, i32 3
  %19 = icmp eq i32 %13, 2
  %20 = select i1 %19, i32 2, i32 3
  %21 = icmp eq i32 %11, 2
  %22 = select i1 %21, i32 2, i32 3
  %23 = select i1 %14, i32 -16, i32 -8
  %24 = xor i32 %23, -1
  %25 = select i1 %14, i32 2, i32 3
  %26 = or disjoint i32 %23, 2
  %27 = zext nneg i32 %25 to i64
  %28 = shl nuw nsw i64 2, %27
  %29 = select i1 %14, i64 123, i64 126
  br label %31

30:                                               ; preds = %586
  ret i32 %590

31:                                               ; preds = %6, %586
  %32 = phi i32 [ 0, %6 ], [ %590, %586 ]
  %33 = phi i32 [ 0, %6 ], [ %591, %586 ]
  switch i32 %9, label %66 [
    i32 5, label %34
    i32 4, label %40
    i32 6, label %53
    i32 0, label %68
    i32 1, label %67
  ]

34:                                               ; preds = %31
  %35 = shl nuw nsw i32 %33, 3
  %36 = lshr i32 %3, %35
  %37 = and i32 %36, 255
  %38 = icmp eq i32 %37, 255
  %39 = add nsw i32 %37, -127
  br label %114

40:                                               ; preds = %31
  %41 = shl nuw nsw i32 %33, 2
  %42 = lshr i32 %3, %41
  %43 = lshr i32 %42, 3
  %44 = and i32 %43, 1
  %45 = lshr i32 %42, 1
  %46 = and i32 %45, 3
  %47 = and i32 %42, 1
  %48 = icmp eq i32 %46, 0
  %49 = or disjoint i32 %47, 2
  %50 = add nsw i32 %46, -2
  %51 = select i1 %48, i32 -1, i32 %50
  %52 = select i1 %48, i32 %47, i32 %49
  br label %114

53:                                               ; preds = %31
  %54 = shl nuw nsw i32 %33, 3
  %55 = lshr i32 %3, %54
  %56 = lshr i32 %55, 3
  %57 = and i32 %56, 1
  %58 = lshr i32 %55, 1
  %59 = and i32 %58, 3
  %60 = and i32 %55, 1
  %61 = icmp eq i32 %59, 0
  %62 = or disjoint i32 %60, 2
  %63 = add nsw i32 %59, -2
  %64 = select i1 %61, i32 -1, i32 %63
  %65 = select i1 %61, i32 %60, i32 %62
  br label %114

66:                                               ; preds = %31
  br label %68

67:                                               ; preds = %31
  br label %68

68:                                               ; preds = %67, %66, %31
  %69 = phi i32 [ %18, %66 ], [ %9, %31 ], [ 1, %67 ]
  %70 = shl nuw nsw i32 %33, 3
  %71 = lshr i32 %3, %70
  %72 = icmp ult i32 %69, 2
  %73 = select i1 %72, i32 255, i32 63
  %74 = and i32 %73, %71
  %75 = select i1 %72, i32 7, i32 5
  %76 = shl nsw i32 -1, %75
  %77 = xor i32 %76, -1
  %78 = and i32 %74, %77
  %79 = lshr i32 %74, %75
  %80 = and i32 %79, 1
  %81 = icmp eq i32 %69, 0
  %82 = icmp eq i32 %78, 127
  %83 = select i1 %81, i1 %82, i1 false
  br i1 %83, label %114, label %84

84:                                               ; preds = %68
  %85 = icmp eq i32 %69, 1
  %86 = icmp ugt i32 %78, 124
  %87 = select i1 %85, i1 %86, i1 false
  br i1 %87, label %114, label %88

88:                                               ; preds = %84
  %89 = icmp eq i32 %78, 124
  %90 = select i1 %85, i1 %89, i1 false
  br i1 %90, label %114, label %91

91:                                               ; preds = %88
  %92 = and i32 %69, 1
  %93 = xor i32 %92, 3
  switch i32 %69, label %94 [
    i32 0, label %98
    i32 2, label %97
  ]

94:                                               ; preds = %91
  %95 = icmp eq i32 %69, 3
  %96 = select i1 %95, i32 2, i32 4
  br label %98

97:                                               ; preds = %91
  br label %98

98:                                               ; preds = %97, %94, %91
  %99 = phi i32 [ 3, %91 ], [ %96, %94 ], [ 1, %97 ]
  %100 = shl nsw i32 -1, %99
  %101 = lshr i32 %78, %93
  %102 = shl nuw nsw i32 1, %93
  %103 = add nsw i32 %102, -1
  %104 = and i32 %78, %103
  %105 = icmp eq i32 %101, 0
  br i1 %105, label %106, label %109

106:                                              ; preds = %98
  %107 = sub nuw nsw i32 %100, %93
  %108 = add nuw nsw i32 %107, 2
  br label %114

109:                                              ; preds = %98
  %110 = or i32 %104, %102
  %111 = sub nsw i32 %101, %93
  %112 = add nsw i32 %111, 1
  %113 = add nsw i32 %112, %100
  br label %114

114:                                              ; preds = %34, %40, %53, %68, %84, %88, %106, %109
  %115 = phi i32 [ 0, %34 ], [ %44, %40 ], [ %57, %53 ], [ %80, %68 ], [ %80, %84 ], [ %80, %88 ], [ %80, %106 ], [ %80, %109 ]
  %116 = phi i1 [ %38, %34 ], [ false, %40 ], [ false, %53 ], [ true, %68 ], [ true, %84 ], [ false, %88 ], [ false, %106 ], [ false, %109 ]
  %117 = phi i1 [ false, %34 ], [ false, %40 ], [ false, %53 ], [ false, %68 ], [ false, %84 ], [ true, %88 ], [ false, %106 ], [ false, %109 ]
  %118 = phi i32 [ 1, %34 ], [ %52, %40 ], [ %65, %53 ], [ 0, %68 ], [ 0, %84 ], [ 0, %88 ], [ %104, %106 ], [ %110, %109 ]
  %119 = phi i32 [ %39, %34 ], [ %51, %40 ], [ %64, %53 ], [ 0, %68 ], [ 0, %84 ], [ 0, %88 ], [ %108, %106 ], [ %113, %109 ]
  %120 = zext nneg i32 %118 to i64
  switch i32 %13, label %153 [
    i32 5, label %121
    i32 4, label %127
    i32 6, label %140
    i32 0, label %155
    i32 1, label %154
  ]

121:                                              ; preds = %114
  %122 = shl nuw nsw i32 %33, 3
  %123 = lshr i32 %5, %122
  %124 = and i32 %123, 255
  %125 = icmp eq i32 %124, 255
  %126 = add nsw i32 %124, -127
  br label %201

127:                                              ; preds = %114
  %128 = shl nuw nsw i32 %33, 2
  %129 = lshr i32 %5, %128
  %130 = lshr i32 %129, 3
  %131 = and i32 %130, 1
  %132 = lshr i32 %129, 1
  %133 = and i32 %132, 3
  %134 = and i32 %129, 1
  %135 = icmp eq i32 %133, 0
  %136 = or disjoint i32 %134, 2
  %137 = add nsw i32 %133, -2
  %138 = select i1 %135, i32 -1, i32 %137
  %139 = select i1 %135, i32 %134, i32 %136
  br label %201

140:                                              ; preds = %114
  %141 = shl nuw nsw i32 %33, 3
  %142 = lshr i32 %5, %141
  %143 = lshr i32 %142, 3
  %144 = and i32 %143, 1
  %145 = lshr i32 %142, 1
  %146 = and i32 %145, 3
  %147 = and i32 %142, 1
  %148 = icmp eq i32 %146, 0
  %149 = or disjoint i32 %147, 2
  %150 = add nsw i32 %146, -2
  %151 = select i1 %148, i32 -1, i32 %150
  %152 = select i1 %148, i32 %147, i32 %149
  br label %201

153:                                              ; preds = %114
  br label %155

154:                                              ; preds = %114
  br label %155

155:                                              ; preds = %154, %153, %114
  %156 = phi i32 [ %20, %153 ], [ %13, %114 ], [ 1, %154 ]
  %157 = shl nuw nsw i32 %33, 3
  %158 = lshr i32 %5, %157
  %159 = icmp ult i32 %156, 2
  %160 = select i1 %159, i32 255, i32 63
  %161 = and i32 %160, %158
  %162 = select i1 %159, i32 7, i32 5
  %163 = shl nsw i32 -1, %162
  %164 = xor i32 %163, -1
  %165 = and i32 %161, %164
  %166 = lshr i32 %161, %162
  %167 = and i32 %166, 1
  %168 = icmp eq i32 %156, 0
  %169 = icmp eq i32 %165, 127
  %170 = select i1 %168, i1 %169, i1 false
  br i1 %170, label %201, label %171

171:                                              ; preds = %155
  %172 = icmp eq i32 %156, 1
  %173 = icmp ugt i32 %165, 124
  %174 = select i1 %172, i1 %173, i1 false
  br i1 %174, label %201, label %175

175:                                              ; preds = %171
  %176 = icmp eq i32 %165, 124
  %177 = select i1 %172, i1 %176, i1 false
  br i1 %177, label %201, label %178

178:                                              ; preds = %175
  %179 = and i32 %156, 1
  %180 = xor i32 %179, 3
  switch i32 %156, label %181 [
    i32 0, label %185
    i32 2, label %184
  ]

181:                                              ; preds = %178
  %182 = icmp eq i32 %156, 3
  %183 = select i1 %182, i32 2, i32 4
  br label %185

184:                                              ; preds = %178
  br label %185

185:                                              ; preds = %184, %181, %178
  %186 = phi i32 [ 3, %178 ], [ %183, %181 ], [ 1, %184 ]
  %187 = shl nsw i32 -1, %186
  %188 = lshr i32 %165, %180
  %189 = shl nuw nsw i32 1, %180
  %190 = add nsw i32 %189, -1
  %191 = and i32 %165, %190
  %192 = icmp eq i32 %188, 0
  br i1 %192, label %193, label %196

193:                                              ; preds = %185
  %194 = sub nuw nsw i32 %187, %180
  %195 = add nuw nsw i32 %194, 2
  br label %201

196:                                              ; preds = %185
  %197 = or i32 %191, %189
  %198 = sub nsw i32 %188, %180
  %199 = add nsw i32 %198, 1
  %200 = add nsw i32 %199, %187
  br label %201

201:                                              ; preds = %121, %127, %140, %155, %171, %175, %193, %196
  %202 = phi i32 [ 0, %121 ], [ %131, %127 ], [ %144, %140 ], [ %167, %155 ], [ %167, %171 ], [ %167, %175 ], [ %167, %193 ], [ %167, %196 ]
  %203 = phi i1 [ %125, %121 ], [ false, %127 ], [ false, %140 ], [ true, %155 ], [ true, %171 ], [ false, %175 ], [ false, %193 ], [ false, %196 ]
  %204 = phi i1 [ false, %121 ], [ false, %127 ], [ false, %140 ], [ false, %155 ], [ false, %171 ], [ true, %175 ], [ false, %193 ], [ false, %196 ]
  %205 = phi i32 [ 1, %121 ], [ %139, %127 ], [ %152, %140 ], [ 0, %155 ], [ 0, %171 ], [ 0, %175 ], [ %191, %193 ], [ %197, %196 ]
  %206 = phi i32 [ %126, %121 ], [ %138, %127 ], [ %151, %140 ], [ 0, %155 ], [ 0, %171 ], [ 0, %175 ], [ %195, %193 ], [ %200, %196 ]
  %207 = zext nneg i32 %205 to i64
  switch i32 %0, label %419 [
    i32 2, label %208
    i32 3, label %230
    i32 1, label %417
  ]

208:                                              ; preds = %201
  %209 = select i1 %116, i1 true, i1 %203
  br i1 %209, label %224, label %210

210:                                              ; preds = %208
  %211 = or i1 %117, %204
  br i1 %211, label %212, label %221

212:                                              ; preds = %210
  %213 = icmp ne i32 %118, 0
  %214 = or i1 %117, %213
  %215 = icmp ne i32 %205, 0
  %216 = or i1 %204, %215
  %217 = select i1 %214, i1 %216, i1 false
  %218 = zext i1 %217 to i32
  %219 = xor i1 %217, true
  %220 = zext i1 %219 to i32
  br label %224

221:                                              ; preds = %210
  %222 = mul nuw nsw i64 %207, %120
  %223 = add nsw i32 %206, %119
  br label %224

224:                                              ; preds = %212, %208, %221
  %225 = phi i32 [ 0, %221 ], [ 0, %208 ], [ %218, %212 ]
  %226 = phi i32 [ 0, %221 ], [ 1, %208 ], [ %220, %212 ]
  %227 = phi i64 [ %222, %221 ], [ 0, %208 ], [ 0, %212 ]
  %228 = phi i32 [ %223, %221 ], [ 0, %208 ], [ 0, %212 ]
  %229 = xor i32 %202, %115
  br label %501

230:                                              ; preds = %201
  switch i32 %11, label %263 [
    i32 5, label %231
    i32 4, label %237
    i32 6, label %250
    i32 0, label %265
    i32 1, label %264
  ]

231:                                              ; preds = %230
  %232 = shl nuw nsw i32 %33, 3
  %233 = lshr i32 %4, %232
  %234 = and i32 %233, 255
  %235 = icmp eq i32 %234, 255
  %236 = add nsw i32 %234, -127
  br label %311

237:                                              ; preds = %230
  %238 = shl nuw nsw i32 %33, 2
  %239 = lshr i32 %4, %238
  %240 = lshr i32 %239, 3
  %241 = and i32 %240, 1
  %242 = lshr i32 %239, 1
  %243 = and i32 %242, 3
  %244 = and i32 %239, 1
  %245 = icmp eq i32 %243, 0
  %246 = or disjoint i32 %244, 2
  %247 = add nsw i32 %243, -2
  %248 = select i1 %245, i32 -1, i32 %247
  %249 = select i1 %245, i32 %244, i32 %246
  br label %311

250:                                              ; preds = %230
  %251 = shl nuw nsw i32 %33, 3
  %252 = lshr i32 %4, %251
  %253 = lshr i32 %252, 3
  %254 = and i32 %253, 1
  %255 = lshr i32 %252, 1
  %256 = and i32 %255, 3
  %257 = and i32 %252, 1
  %258 = icmp eq i32 %256, 0
  %259 = or disjoint i32 %257, 2
  %260 = add nsw i32 %256, -2
  %261 = select i1 %258, i32 -1, i32 %260
  %262 = select i1 %258, i32 %257, i32 %259
  br label %311

263:                                              ; preds = %230
  br label %265

264:                                              ; preds = %230
  br label %265

265:                                              ; preds = %264, %263, %230
  %266 = phi i32 [ %22, %263 ], [ %11, %230 ], [ 1, %264 ]
  %267 = shl nuw nsw i32 %33, 3
  %268 = lshr i32 %4, %267
  %269 = icmp ult i32 %266, 2
  %270 = select i1 %269, i32 255, i32 63
  %271 = and i32 %270, %268
  %272 = select i1 %269, i32 7, i32 5
  %273 = shl nsw i32 -1, %272
  %274 = xor i32 %273, -1
  %275 = and i32 %271, %274
  %276 = lshr i32 %271, %272
  %277 = and i32 %276, 1
  %278 = icmp eq i32 %266, 0
  %279 = icmp eq i32 %275, 127
  %280 = select i1 %278, i1 %279, i1 false
  br i1 %280, label %311, label %281

281:                                              ; preds = %265
  %282 = icmp eq i32 %266, 1
  %283 = icmp ugt i32 %275, 124
  %284 = select i1 %282, i1 %283, i1 false
  br i1 %284, label %311, label %285

285:                                              ; preds = %281
  %286 = icmp eq i32 %275, 124
  %287 = select i1 %282, i1 %286, i1 false
  br i1 %287, label %311, label %288

288:                                              ; preds = %285
  %289 = and i32 %266, 1
  %290 = xor i32 %289, 3
  switch i32 %266, label %291 [
    i32 0, label %295
    i32 2, label %294
  ]

291:                                              ; preds = %288
  %292 = icmp eq i32 %266, 3
  %293 = select i1 %292, i32 2, i32 4
  br label %295

294:                                              ; preds = %288
  br label %295

295:                                              ; preds = %294, %291, %288
  %296 = phi i32 [ 3, %288 ], [ %293, %291 ], [ 1, %294 ]
  %297 = shl nsw i32 -1, %296
  %298 = lshr i32 %275, %290
  %299 = shl nuw nsw i32 1, %290
  %300 = add nsw i32 %299, -1
  %301 = and i32 %275, %300
  %302 = icmp eq i32 %298, 0
  br i1 %302, label %303, label %306

303:                                              ; preds = %295
  %304 = sub nuw nsw i32 %297, %290
  %305 = add nuw nsw i32 %304, 2
  br label %311

306:                                              ; preds = %295
  %307 = or i32 %301, %299
  %308 = sub nsw i32 %298, %290
  %309 = add nsw i32 %308, 1
  %310 = add nsw i32 %309, %297
  br label %311

311:                                              ; preds = %231, %237, %250, %265, %281, %285, %303, %306
  %312 = phi i32 [ 0, %231 ], [ %241, %237 ], [ %254, %250 ], [ %277, %265 ], [ %277, %281 ], [ %277, %285 ], [ %277, %303 ], [ %277, %306 ]
  %313 = phi i1 [ %235, %231 ], [ false, %237 ], [ false, %250 ], [ true, %265 ], [ true, %281 ], [ false, %285 ], [ false, %303 ], [ false, %306 ]
  %314 = phi i1 [ false, %231 ], [ false, %237 ], [ false, %250 ], [ false, %265 ], [ false, %281 ], [ true, %285 ], [ false, %303 ], [ false, %306 ]
  %315 = phi i32 [ 1, %231 ], [ %249, %237 ], [ %262, %250 ], [ 0, %265 ], [ 0, %281 ], [ 0, %285 ], [ %301, %303 ], [ %307, %306 ]
  %316 = phi i32 [ %236, %231 ], [ %248, %237 ], [ %261, %250 ], [ 0, %265 ], [ 0, %281 ], [ 0, %285 ], [ %305, %303 ], [ %310, %306 ]
  %317 = zext nneg i32 %315 to i64
  %318 = select i1 %116, i1 true, i1 %313
  br i1 %318, label %331, label %319

319:                                              ; preds = %311
  %320 = or i1 %117, %314
  br i1 %320, label %321, label %328

321:                                              ; preds = %319
  %322 = icmp ne i32 %118, 0
  %323 = or i1 %117, %322
  %324 = icmp ne i32 %315, 0
  %325 = or i1 %314, %324
  %326 = select i1 %323, i1 %325, i1 false
  %327 = xor i1 %326, true
  br label %331

328:                                              ; preds = %319
  %329 = mul nuw nsw i64 %317, %120
  %330 = add nsw i32 %316, %119
  br label %331

331:                                              ; preds = %321, %311, %328
  %332 = phi i1 [ false, %328 ], [ false, %311 ], [ %326, %321 ]
  %333 = phi i1 [ false, %328 ], [ true, %311 ], [ %327, %321 ]
  %334 = phi i64 [ %329, %328 ], [ 0, %311 ], [ 0, %321 ]
  %335 = phi i32 [ %330, %328 ], [ 0, %311 ], [ 0, %321 ]
  %336 = xor i32 %312, %115
  %337 = select i1 %333, i1 true, i1 %203
  br i1 %337, label %501, label %338

338:                                              ; preds = %331
  %339 = and i1 %204, %332
  %340 = icmp ne i32 %336, %202
  %341 = select i1 %339, i1 %340, i1 false
  br i1 %341, label %501, label %342

342:                                              ; preds = %338
  %343 = or i1 %204, %332
  br i1 %343, label %344, label %346

344:                                              ; preds = %342
  %345 = select i1 %332, i32 %336, i32 %202
  br label %501

346:                                              ; preds = %342
  %347 = icmp ne i64 %334, 0
  %348 = icmp ne i32 %205, 0
  %349 = select i1 %347, i1 %348, i1 false
  br i1 %349, label %361, label %350

350:                                              ; preds = %346
  %351 = select i1 %347, i1 true, i1 %348
  br i1 %351, label %357, label %352

352:                                              ; preds = %350
  %353 = icmp ne i32 %336, 0
  %354 = icmp ne i32 %202, 0
  %355 = select i1 %353, i1 %354, i1 false
  %356 = zext i1 %355 to i32
  br label %501

357:                                              ; preds = %350
  %358 = select i1 %347, i64 %334, i64 %207
  %359 = select i1 %347, i32 %335, i32 %206
  %360 = select i1 %347, i32 %336, i32 %202
  br label %501

361:                                              ; preds = %346, %361
  %362 = phi i32 [ %365, %361 ], [ 0, %346 ]
  %363 = phi i64 [ %364, %361 ], [ %334, %346 ]
  %364 = lshr i64 %363, 1
  %365 = add nuw nsw i32 %362, 1
  %366 = icmp ult i64 %363, 2
  br i1 %366, label %367, label %361, !llvm.loop !20003

367:                                              ; preds = %361, %367
  %368 = phi i32 [ %371, %367 ], [ 0, %361 ]
  %369 = phi i64 [ %370, %367 ], [ %207, %361 ]
  %370 = lshr i64 %369, 1
  %371 = add nuw nsw i32 %368, 1
  %372 = icmp ult i64 %369, 2
  br i1 %372, label %373, label %367, !llvm.loop !20003

373:                                              ; preds = %367
  %374 = add nsw i32 %365, %335
  %375 = add nsw i32 %371, %206
  %376 = icmp slt i32 %374, %375
  br i1 %376, label %381, label %377

377:                                              ; preds = %373
  %378 = icmp eq i32 %374, %375
  %379 = icmp sgt i32 %335, %206
  %380 = and i1 %379, %378
  br i1 %380, label %381, label %382

381:                                              ; preds = %377, %373
  br label %382

382:                                              ; preds = %381, %377
  %383 = phi i64 [ %207, %381 ], [ %334, %377 ]
  %384 = phi i32 [ %206, %381 ], [ %335, %377 ]
  %385 = phi i32 [ %202, %381 ], [ %336, %377 ]
  %386 = phi i32 [ %336, %381 ], [ %202, %377 ]
  %387 = phi i64 [ %334, %381 ], [ %207, %377 ]
  %388 = phi i32 [ %335, %381 ], [ %206, %377 ]
  %389 = phi i32 [ %375, %381 ], [ %374, %377 ]
  %390 = phi i32 [ %374, %381 ], [ %375, %377 ]
  %391 = sub nsw i32 %389, %390
  %392 = icmp sgt i32 %391, 40
  br i1 %392, label %393, label %399

393:                                              ; preds = %382
  %394 = shl nuw nsw i64 %383, 3
  %395 = icmp eq i32 %385, %386
  %396 = select i1 %395, i64 1, i64 -1
  %397 = add nsw i64 %396, %394
  %398 = add nsw i32 %384, -3
  br label %501

399:                                              ; preds = %382
  %400 = tail call i32 @llvm.smin.i32(i32 %206, i32 %335)
  %401 = sub nsw i32 %384, %400
  %402 = zext nneg i32 %401 to i64
  %403 = shl i64 %383, %402
  %404 = sub nsw i32 %388, %400
  %405 = zext nneg i32 %404 to i64
  %406 = shl i64 %387, %405
  %407 = icmp eq i32 %385, 0
  %408 = sub nsw i64 0, %403
  %409 = select i1 %407, i64 %403, i64 %408
  %410 = icmp eq i32 %386, 0
  %411 = sub nsw i64 0, %406
  %412 = select i1 %410, i64 %406, i64 %411
  %413 = add nsw i64 %412, %409
  %414 = lshr i64 %413, 63
  %415 = trunc i64 %414 to i32
  %416 = tail call i64 @llvm.abs.i64(i64 %413, i1 true)
  br label %501

417:                                              ; preds = %201
  %418 = xor i32 %202, 1
  br label %419

419:                                              ; preds = %201, %417
  %420 = phi i32 [ %202, %201 ], [ %418, %417 ]
  %421 = select i1 %116, i1 true, i1 %203
  br i1 %421, label %501, label %422

422:                                              ; preds = %419
  %423 = and i1 %117, %204
  %424 = icmp ne i32 %115, %420
  %425 = select i1 %423, i1 %424, i1 false
  br i1 %425, label %501, label %426

426:                                              ; preds = %422
  %427 = or i1 %117, %204
  br i1 %427, label %428, label %430

428:                                              ; preds = %426
  %429 = select i1 %117, i32 %115, i32 %420
  br label %501

430:                                              ; preds = %426
  %431 = icmp ne i32 %118, 0
  %432 = icmp ne i32 %205, 0
  %433 = select i1 %431, i1 %432, i1 false
  br i1 %433, label %445, label %434

434:                                              ; preds = %430
  %435 = select i1 %431, i1 true, i1 %432
  br i1 %435, label %441, label %436

436:                                              ; preds = %434
  %437 = icmp ne i32 %115, 0
  %438 = icmp ne i32 %420, 0
  %439 = select i1 %437, i1 %438, i1 false
  %440 = zext i1 %439 to i32
  br label %501

441:                                              ; preds = %434
  %442 = select i1 %431, i64 %120, i64 %207
  %443 = select i1 %431, i32 %119, i32 %206
  %444 = select i1 %431, i32 %115, i32 %420
  br label %501

445:                                              ; preds = %430, %445
  %446 = phi i32 [ %449, %445 ], [ 0, %430 ]
  %447 = phi i64 [ %448, %445 ], [ %120, %430 ]
  %448 = lshr i64 %447, 1
  %449 = add nuw nsw i32 %446, 1
  %450 = icmp ult i64 %447, 2
  br i1 %450, label %451, label %445, !llvm.loop !20003

451:                                              ; preds = %445, %451
  %452 = phi i32 [ %455, %451 ], [ 0, %445 ]
  %453 = phi i64 [ %454, %451 ], [ %207, %445 ]
  %454 = lshr i64 %453, 1
  %455 = add nuw nsw i32 %452, 1
  %456 = icmp ult i64 %453, 2
  br i1 %456, label %457, label %451, !llvm.loop !20003

457:                                              ; preds = %451
  %458 = add nsw i32 %449, %119
  %459 = add nsw i32 %455, %206
  %460 = icmp slt i32 %458, %459
  br i1 %460, label %465, label %461

461:                                              ; preds = %457
  %462 = icmp eq i32 %458, %459
  %463 = icmp sgt i32 %119, %206
  %464 = and i1 %463, %462
  br i1 %464, label %465, label %466

465:                                              ; preds = %461, %457
  br label %466

466:                                              ; preds = %465, %461
  %467 = phi i64 [ %207, %465 ], [ %120, %461 ]
  %468 = phi i32 [ %206, %465 ], [ %119, %461 ]
  %469 = phi i32 [ %420, %465 ], [ %115, %461 ]
  %470 = phi i32 [ %115, %465 ], [ %420, %461 ]
  %471 = phi i64 [ %120, %465 ], [ %207, %461 ]
  %472 = phi i32 [ %119, %465 ], [ %206, %461 ]
  %473 = phi i32 [ %459, %465 ], [ %458, %461 ]
  %474 = phi i32 [ %458, %465 ], [ %459, %461 ]
  %475 = sub nsw i32 %473, %474
  %476 = icmp sgt i32 %475, 40
  br i1 %476, label %477, label %483

477:                                              ; preds = %466
  %478 = shl nuw nsw i64 %467, 3
  %479 = icmp eq i32 %469, %470
  %480 = select i1 %479, i64 1, i64 -1
  %481 = add nsw i64 %480, %478
  %482 = add nsw i32 %468, -3
  br label %501

483:                                              ; preds = %466
  %484 = tail call i32 @llvm.smin.i32(i32 %206, i32 %119)
  %485 = sub nsw i32 %468, %484
  %486 = zext nneg i32 %485 to i64
  %487 = shl i64 %467, %486
  %488 = sub nsw i32 %472, %484
  %489 = zext nneg i32 %488 to i64
  %490 = shl i64 %471, %489
  %491 = icmp eq i32 %469, 0
  %492 = sub nsw i64 0, %487
  %493 = select i1 %491, i64 %487, i64 %492
  %494 = icmp eq i32 %470, 0
  %495 = sub nsw i64 0, %490
  %496 = select i1 %494, i64 %490, i64 %495
  %497 = add nsw i64 %496, %493
  %498 = lshr i64 %497, 63
  %499 = trunc i64 %498 to i32
  %500 = tail call i64 @llvm.abs.i64(i64 %497, i1 true)
  br label %501

501:                                              ; preds = %483, %477, %441, %436, %428, %419, %422, %399, %393, %357, %352, %344, %331, %338, %224
  %502 = phi i32 [ %228, %224 ], [ 0, %344 ], [ %359, %357 ], [ 0, %352 ], [ 0, %331 ], [ %398, %393 ], [ %400, %399 ], [ 0, %338 ], [ 0, %428 ], [ %443, %441 ], [ 0, %436 ], [ 0, %419 ], [ %482, %477 ], [ %484, %483 ], [ 0, %422 ]
  %503 = phi i64 [ %227, %224 ], [ 0, %344 ], [ %358, %357 ], [ 0, %352 ], [ 0, %331 ], [ %397, %393 ], [ %416, %399 ], [ 0, %338 ], [ 0, %428 ], [ %442, %441 ], [ 0, %436 ], [ 0, %419 ], [ %481, %477 ], [ %500, %483 ], [ 0, %422 ]
  %504 = phi i32 [ %225, %224 ], [ 1, %344 ], [ 0, %357 ], [ 0, %352 ], [ 0, %331 ], [ 0, %393 ], [ 0, %399 ], [ 0, %338 ], [ 1, %428 ], [ 0, %441 ], [ 0, %436 ], [ 0, %419 ], [ 0, %477 ], [ 0, %483 ], [ 0, %422 ]
  %505 = phi i32 [ %226, %224 ], [ 0, %344 ], [ 0, %357 ], [ 0, %352 ], [ 1, %331 ], [ 0, %393 ], [ 0, %399 ], [ 1, %338 ], [ 0, %428 ], [ 0, %441 ], [ 0, %436 ], [ 1, %419 ], [ 0, %477 ], [ 0, %483 ], [ 1, %422 ]
  %506 = phi i32 [ %229, %224 ], [ %345, %344 ], [ %360, %357 ], [ %356, %352 ], [ 0, %331 ], [ %385, %393 ], [ %415, %399 ], [ 0, %338 ], [ %429, %428 ], [ %444, %441 ], [ %440, %436 ], [ 0, %419 ], [ %469, %477 ], [ %499, %483 ], [ 0, %422 ]
  %507 = icmp eq i32 %506, 0
  %508 = select i1 %507, i32 0, i32 128
  %509 = or disjoint i32 %508, %16
  %510 = or disjoint i32 %508, 124
  %511 = select i1 %14, i32 %510, i32 127
  %512 = select i1 %15, i32 %511, i32 %509
  %513 = icmp eq i32 %505, 0
  br i1 %513, label %514, label %586

514:                                              ; preds = %501
  %515 = icmp eq i32 %504, 0
  br i1 %515, label %516, label %586

516:                                              ; preds = %514
  %517 = icmp eq i64 %503, 0
  br i1 %517, label %586, label %518

518:                                              ; preds = %516, %518
  %519 = phi i32 [ %522, %518 ], [ 0, %516 ]
  %520 = phi i64 [ %521, %518 ], [ %503, %516 ]
  %521 = lshr i64 %520, 1
  %522 = add nuw nsw i32 %519, 1
  %523 = icmp ult i64 %520, 2
  br i1 %523, label %524, label %518, !llvm.loop !20003

524:                                              ; preds = %518
  %525 = add nsw i32 %522, %502
  %526 = add i32 %519, %502
  %527 = icmp sgt i32 %525, %26
  %528 = select i1 %527, i32 %526, i32 %26
  %529 = icmp sgt i32 %528, 200
  br i1 %529, label %580, label %530

530:                                              ; preds = %524
  %531 = sub i32 %25, %528
  %532 = add i32 %531, %502
  %533 = icmp sgt i32 %532, -1
  br i1 %533, label %534, label %539

534:                                              ; preds = %530
  %535 = icmp ugt i32 %532, 40
  %536 = zext nneg i32 %532 to i64
  %537 = shl i64 %503, %536
  %538 = select i1 %535, i64 4611686018427387904, i64 %537
  br label %564

539:                                              ; preds = %530
  %540 = sub nsw i32 0, %532
  %541 = icmp ult i32 %532, -63
  %542 = zext nneg i32 %540 to i64
  %543 = lshr i64 %503, %542
  %544 = select i1 %541, i64 0, i64 %543
  %545 = shl nsw i64 -1, %542
  %546 = xor i64 %545, -1
  %547 = select i1 %541, i64 -1, i64 %546
  %548 = and i64 %547, %503
  %549 = icmp eq i64 %548, 0
  %550 = or i1 %541, %549
  %551 = select i1 %549, i64 %544, i64 0
  br i1 %550, label %564, label %552

552:                                              ; preds = %539
  %553 = xor i32 %532, -1
  %554 = zext nneg i32 %553 to i64
  %555 = shl nuw nsw i64 1, %554
  %556 = icmp ugt i64 %548, %555
  %557 = icmp eq i64 %548, %555
  %558 = and i64 %543, 1
  %559 = icmp ne i64 %558, 0
  %560 = select i1 %557, i1 %559, i1 false
  %561 = select i1 %556, i1 true, i1 %560
  %562 = zext i1 %561 to i64
  %563 = add nuw i64 %543, %562
  br label %564

564:                                              ; preds = %552, %539, %534
  %565 = phi i64 [ %538, %534 ], [ %563, %552 ], [ %551, %539 ]
  %566 = icmp ult i64 %565, %28
  br i1 %566, label %571, label %567

567:                                              ; preds = %564
  %568 = sub i32 %528, %23
  %569 = sext i32 %568 to i64
  %570 = shl nsw i64 %569, %27
  br label %580

571:                                              ; preds = %564
  %572 = lshr i64 %565, %27
  %573 = icmp eq i64 %572, 0
  %574 = add nsw i32 %528, %24
  %575 = sext i32 %574 to i64
  %576 = add nsw i64 %575, -1
  %577 = shl nsw i64 %576, %27
  %578 = select i1 %573, i64 0, i64 %577
  %579 = add i64 %578, %565
  br label %580

580:                                              ; preds = %571, %567, %524
  %581 = phi i64 [ 1099511627776, %524 ], [ %570, %567 ], [ %579, %571 ]
  %582 = icmp sgt i64 %581, %29
  %583 = trunc i64 %581 to i32
  %584 = add i32 %508, %583
  %585 = select i1 %582, i32 %512, i32 %584
  br label %586

586:                                              ; preds = %501, %514, %516, %580
  %587 = phi i32 [ %585, %580 ], [ 127, %501 ], [ %512, %514 ], [ %508, %516 ]
  %588 = shl nuw nsw i32 %33, 3
  %589 = shl i32 %587, %588
  %590 = or i32 %589, %32
  %591 = add nuw nsw i32 %33, 1
  %592 = icmp eq i32 %591, 4
  br i1 %592, label %30, label %31, !llvm.loop !20007
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
  br i1 %460, label %40, label %34, !llvm.loop !20008
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
  br i1 %78, label %20, label %41, !llvm.loop !20009
}

; Function Attrs: nounwind
define dso_local i32 @__zluda_ptx_impl_agx_tmem_alloc(i32 noundef %0, i32 noundef %1, i32 noundef %2) local_unnamed_addr #3 {
  %4 = icmp eq i32 %1, 0
  br i1 %4, label %104, label %5

5:                                                ; preds = %3
  %6 = icmp eq i32 %2, 0
  br i1 %6, label %13, label %7

7:                                                ; preds = %5
  %8 = add i32 %0, -32
  %9 = icmp ult i32 %8, 545
  %10 = and i32 %0, 31
  %11 = icmp eq i32 %10, 0
  %12 = and i1 %9, %11
  br i1 %12, label %21, label %20

13:                                               ; preds = %5
  %14 = add i32 %0, -513
  %15 = icmp ult i32 %14, -481
  br i1 %15, label %20, label %16

16:                                               ; preds = %13
  %17 = add nuw nsw i32 %0, 1023
  %18 = and i32 %17, %0
  %19 = icmp eq i32 %18, 0
  br i1 %19, label %21, label %20

20:                                               ; preds = %16, %13, %7
  tail call void @llvm.trap()
  unreachable

21:                                               ; preds = %7, %16
  %22 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 0)
  %23 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 1)
  %24 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 2)
  %25 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 0)
  %26 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 1)
  %27 = mul i32 %26, %24
  %28 = add i32 %27, %23
  %29 = mul i32 %28, %25
  %30 = add i32 %22, 1
  %31 = add i32 %30, %29
  %32 = load atomic i32, ptr addrspace(1) @__zluda_agx_tmem_owner acquire, align 4
  %33 = icmp eq i32 %32, %31
  br i1 %33, label %44, label %34

34:                                               ; preds = %21
  %35 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 1) acquire, align 4
  %36 = icmp eq i32 %35, %31
  br i1 %36, label %44, label %37

37:                                               ; preds = %34
  %38 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 2) acquire, align 4
  %39 = icmp eq i32 %38, %31
  br i1 %39, label %44, label %40

40:                                               ; preds = %37
  %41 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 3) acquire, align 4
  %42 = icmp eq i32 %41, %31
  %43 = select i1 %42, i32 3, i32 -1
  br label %44

44:                                               ; preds = %40, %37, %34, %21
  %45 = phi i32 [ 0, %21 ], [ 1, %34 ], [ 2, %37 ], [ %43, %40 ]
  %46 = icmp slt i32 %45, 0
  br i1 %46, label %47, label %63

47:                                               ; preds = %44, %47
  %48 = phi i32 [ %57, %47 ], [ -1, %44 ]
  %49 = phi i32 [ %62, %47 ], [ 0, %44 ]
  %50 = zext nneg i32 %49 to i64
  %51 = getelementptr inbounds [4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 %50
  %52 = cmpxchg ptr addrspace(1) %51, i32 0, i32 %31 acq_rel acquire, align 4
  %53 = extractvalue { i32, i1 } %52, 1
  %54 = extractvalue { i32, i1 } %52, 0
  %55 = icmp eq i32 %54, %31
  %56 = select i1 %53, i1 true, i1 %55
  %57 = select i1 %56, i32 %49, i32 %48
  %58 = add nuw nsw i32 %49, 1
  %59 = icmp ult i32 %49, 3
  %60 = icmp slt i32 %57, 0
  %61 = and i1 %59, %60
  %62 = select i1 %61, i32 %58, i32 0
  br i1 %60, label %47, label %63, !llvm.loop !20010

63:                                               ; preds = %47, %44
  %64 = phi i32 [ %45, %44 ], [ %57, %47 ]
  %65 = lshr i32 %0, 5
  %66 = shl nsw i32 -1, %65
  %67 = xor i32 %66, -1
  %68 = zext nneg i32 %64 to i64
  %69 = getelementptr inbounds [4 x i32], ptr addrspace(1) @__zluda_agx_tmem_map, i64 0, i64 %68
  %70 = or disjoint i32 %67, -2147483648
  br label %71

71:                                               ; preds = %103, %63
  %72 = phi i32 [ undef, %63 ], [ %102, %103 ]
  %73 = load atomic i32, ptr addrspace(1) %69 acquire, align 4
  br i1 %6, label %79, label %74

74:                                               ; preds = %71
  %75 = cmpxchg ptr addrspace(1) %69, i32 0, i32 %70 acq_rel acquire, align 4
  %76 = extractvalue { i32, i1 } %75, 1
  %77 = select i1 %76, i32 1, i32 3
  %78 = select i1 %76, i32 0, i32 %72
  br label %100

79:                                               ; preds = %71
  %80 = icmp sgt i32 %73, -1
  br i1 %80, label %81, label %100

81:                                               ; preds = %79, %85
  %82 = phi i32 [ %83, %85 ], [ 0, %79 ]
  %83 = add nuw nsw i32 %82, %65
  %84 = icmp ult i32 %83, 17
  br i1 %84, label %85, label %89

85:                                               ; preds = %81
  %86 = shl i32 %67, %82
  %87 = and i32 %86, %73
  %88 = icmp eq i32 %87, 0
  br i1 %88, label %89, label %81, !llvm.loop !20011

89:                                               ; preds = %81, %85
  %90 = icmp ugt i32 %83, 16
  br i1 %90, label %91, label %92

91:                                               ; preds = %89
  tail call void @llvm.trap()
  unreachable

92:                                               ; preds = %89
  %93 = shl i32 %67, %82
  %94 = or i32 %93, %73
  %95 = cmpxchg ptr addrspace(1) %69, i32 %73, i32 %94 acq_rel acquire, align 4
  %96 = extractvalue { i32, i1 } %95, 1
  %97 = shl nuw nsw i32 %82, 5
  %98 = zext i1 %96 to i32
  %99 = select i1 %96, i32 %97, i32 %72
  br label %100

100:                                              ; preds = %79, %92, %74
  %101 = phi i32 [ %77, %74 ], [ %98, %92 ], [ 3, %79 ]
  %102 = phi i32 [ %78, %74 ], [ %99, %92 ], [ %72, %79 ]
  switch i32 %101, label %104 [
    i32 0, label %103
    i32 3, label %103
  ]

103:                                              ; preds = %100, %100
  br label %71

104:                                              ; preds = %100, %3
  %105 = phi i32 [ 0, %3 ], [ %102, %100 ]
  ret i32 %105
}

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)

; Function Attrs: nounwind
define dso_local void @__zluda_ptx_impl_agx_tmem_zero(i32 noundef %0, i32 noundef %1, i32 noundef %2) local_unnamed_addr #3 {
  %4 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 0)
  %5 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 1)
  %6 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 2)
  %7 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 0)
  %8 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 1)
  %9 = mul i32 %8, %6
  %10 = add i32 %9, %5
  %11 = mul i32 %10, %7
  %12 = add i32 %4, 1
  %13 = add i32 %12, %11
  %14 = load atomic i32, ptr addrspace(1) @__zluda_agx_tmem_owner acquire, align 4
  %15 = icmp eq i32 %14, %13
  br i1 %15, label %26, label %16

16:                                               ; preds = %3
  %17 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 1) acquire, align 4
  %18 = icmp eq i32 %17, %13
  br i1 %18, label %26, label %19

19:                                               ; preds = %16
  %20 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 2) acquire, align 4
  %21 = icmp eq i32 %20, %13
  br i1 %21, label %26, label %22

22:                                               ; preds = %19
  %23 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 3) acquire, align 4
  %24 = icmp eq i32 %23, %13
  %25 = select i1 %24, i32 3, i32 -1
  br label %26

26:                                               ; preds = %22, %19, %16, %3
  %27 = phi i32 [ 0, %3 ], [ 1, %16 ], [ 2, %19 ], [ %25, %22 ]
  %28 = icmp slt i32 %27, 0
  br i1 %28, label %29, label %30

29:                                               ; preds = %26
  tail call void @llvm.trap()
  unreachable

30:                                               ; preds = %26
  %31 = shl nuw nsw i32 %27, 17
  %32 = zext nneg i32 %31 to i64
  %33 = getelementptr inbounds [524288 x i32], ptr addrspace(1) @__zluda_agx_tmem_pool, i64 0, i64 %32
  %34 = shl i32 %1, 7
  %35 = icmp ugt i32 %34, %2
  br i1 %35, label %37, label %36

36:                                               ; preds = %37, %30
  ret void

37:                                               ; preds = %30, %37
  %38 = phi i32 [ %49, %37 ], [ %2, %30 ]
  %39 = freeze i32 %38
  %40 = udiv i32 %39, %1
  %41 = shl i32 %40, 10
  %42 = mul i32 %40, %1
  %43 = sub i32 %39, %42
  %44 = add i32 %43, %0
  %45 = and i32 %44, 1023
  %46 = or disjoint i32 %45, %41
  %47 = zext i32 %46 to i64
  %48 = getelementptr inbounds i32, ptr addrspace(1) %33, i64 %47
  store i32 0, ptr addrspace(1) %48, align 4
  %49 = add nuw i32 %38, 32
  %50 = icmp ult i32 %49, %34
  br i1 %50, label %37, label %36, !llvm.loop !20016
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
  %28 = shl nuw nsw i32 %24, 17
  %29 = zext nneg i32 %28 to i64
  %30 = getelementptr inbounds [524288 x i32], ptr addrspace(1) @__zluda_agx_tmem_pool, i64 0, i64 %29
  %31 = ptrtoint ptr addrspace(1) %30 to i64
  ret i64 %31
}

; Function Attrs: nounwind
define dso_local void @__zluda_ptx_impl_agx_tmem_dealloc(i32 noundef %0, i32 noundef %1, i32 noundef %2, i32 noundef %3) local_unnamed_addr #3 {
  %5 = icmp eq i32 %2, 0
  br i1 %5, label %63, label %6

6:                                                ; preds = %4
  %7 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 0)
  %8 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 1)
  %9 = tail call i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext 2)
  %10 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 0)
  %11 = tail call i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext 1)
  %12 = mul i32 %11, %9
  %13 = add i32 %12, %8
  %14 = mul i32 %13, %10
  %15 = add i32 %7, 1
  %16 = add i32 %15, %14
  %17 = load atomic i32, ptr addrspace(1) @__zluda_agx_tmem_owner acquire, align 4
  %18 = icmp eq i32 %17, %16
  br i1 %18, label %29, label %19

19:                                               ; preds = %6
  %20 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 1) acquire, align 4
  %21 = icmp eq i32 %20, %16
  br i1 %21, label %29, label %22

22:                                               ; preds = %19
  %23 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 2) acquire, align 4
  %24 = icmp eq i32 %23, %16
  br i1 %24, label %29, label %25

25:                                               ; preds = %22
  %26 = load atomic i32, ptr addrspace(1) getelementptr inbounds ([4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 3) acquire, align 4
  %27 = icmp eq i32 %26, %16
  %28 = select i1 %27, i32 3, i32 -1
  br label %29

29:                                               ; preds = %6, %19, %22, %25
  %30 = phi i32 [ 0, %6 ], [ 1, %19 ], [ 2, %22 ], [ %28, %25 ]
  %31 = icmp slt i32 %30, 0
  br i1 %31, label %41, label %32

32:                                               ; preds = %29
  %33 = and i32 %0, 31
  %34 = icmp ne i32 %33, 0
  %35 = icmp ult i32 %1, 32
  %36 = or i1 %34, %35
  br i1 %36, label %41, label %37

37:                                               ; preds = %32
  %38 = icmp eq i32 %3, 0
  %39 = select i1 %38, i32 512, i32 576
  %40 = icmp ult i32 %39, %1
  br i1 %40, label %41, label %42

41:                                               ; preds = %37, %32, %29
  tail call void @llvm.trap()
  unreachable

42:                                               ; preds = %37
  %43 = lshr i32 %1, 5
  %44 = shl nsw i32 -1, %43
  %45 = xor i32 %44, -1
  %46 = lshr i32 %0, 5
  %47 = and i32 %46, 2047
  %48 = shl i32 %45, %47
  %49 = or i32 %48, -2147483648
  %50 = select i1 %38, i32 %48, i32 %49
  %51 = zext nneg i32 %30 to i64
  %52 = getelementptr inbounds [4 x i32], ptr addrspace(1) @__zluda_agx_tmem_map, i64 0, i64 %51
  %53 = xor i32 %50, -1
  %54 = atomicrmw and ptr addrspace(1) %52, i32 %53 acq_rel, align 4
  %55 = and i32 %54, %50
  %56 = icmp eq i32 %55, %50
  br i1 %56, label %58, label %57

57:                                               ; preds = %42
  tail call void @llvm.trap()
  unreachable

58:                                               ; preds = %42
  %59 = and i32 %54, %53
  %60 = icmp eq i32 %59, 0
  br i1 %60, label %61, label %63

61:                                               ; preds = %58
  %62 = getelementptr inbounds [4 x i32], ptr addrspace(1) @__zluda_agx_tmem_owner, i64 0, i64 %51
  store atomic i32 0, ptr addrspace(1) %62 release, align 4
  br label %63

63:                                               ; preds = %58, %61, %4
  ret void
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
  %45 = shl nuw nsw i32 %41, 17
  %46 = zext nneg i32 %45 to i64
  %47 = getelementptr inbounds [524288 x i32], ptr addrspace(1) @__zluda_agx_tmem_pool, i64 0, i64 %46
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
  %33 = shl nuw nsw i32 %29, 17
  %34 = zext nneg i32 %33 to i64
  %35 = getelementptr inbounds [524288 x i32], ptr addrspace(1) @__zluda_agx_tmem_pool, i64 0, i64 %34
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
  br i1 %79, label %344, label %65, !llvm.loop !20017

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
  br i1 %295, label %296, label %249, !llvm.loop !20018

296:                                              ; preds = %289, %238
  %297 = phi i32 [ %123, %238 ], [ %290, %289 ]
  %298 = phi i32 [ %163, %238 ], [ %291, %289 ]
  %299 = phi i32 [ %203, %238 ], [ %292, %289 ]
  %300 = phi i32 [ %243, %238 ], [ %293, %289 ]
  %301 = add nuw nsw i32 %82, %37
  %302 = and i32 %301, 1023
  %303 = add nuw nsw i32 %301, 1
  %304 = and i32 %303, 1023
  %305 = add nuw nsw i32 %301, 2
  %306 = and i32 %305, 1023
  %307 = add nuw nsw i32 %301, 3
  %308 = and i32 %307, 1023
  br label %311

309:                                              ; preds = %325
  %310 = and i1 %45, %81
  br i1 %310, label %80, label %77, !llvm.loop !20019

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
  %328 = shl i32 %327, 10
  %329 = and i32 %328, 130048
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
  br i1 %343, label %309, label %311, !llvm.loop !20020

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
  %393 = shl nuw nsw i32 %389, 17
  %394 = zext nneg i32 %393 to i64
  %395 = getelementptr inbounds [524288 x i32], ptr addrspace(1) @__zluda_agx_tmem_pool, i64 0, i64 %394
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
  br i1 %412, label %677, label %398, !llvm.loop !20017

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
  br i1 %628, label %629, label %582, !llvm.loop !20018

629:                                              ; preds = %622, %571
  %630 = phi i32 [ %456, %571 ], [ %623, %622 ]
  %631 = phi i32 [ %496, %571 ], [ %624, %622 ]
  %632 = phi i32 [ %536, %571 ], [ %625, %622 ]
  %633 = phi i32 [ %576, %571 ], [ %626, %622 ]
  %634 = add nuw nsw i32 %415, %37
  %635 = and i32 %634, 1023
  %636 = add nuw nsw i32 %634, 1
  %637 = and i32 %636, 1023
  %638 = add nuw nsw i32 %634, 2
  %639 = and i32 %638, 1023
  %640 = add nuw nsw i32 %634, 3
  %641 = and i32 %640, 1023
  br label %644

642:                                              ; preds = %658
  %643 = and i1 %45, %414
  br i1 %643, label %413, label %410, !llvm.loop !20019

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
  %661 = shl i32 %660, 10
  %662 = and i32 %661, 130048
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
  br i1 %676, label %642, label %644, !llvm.loop !20020

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
  %30 = shl nuw nsw i32 %26, 17
  %31 = zext nneg i32 %30 to i64
  %32 = getelementptr inbounds [524288 x i32], ptr addrspace(1) @__zluda_agx_tmem_pool, i64 0, i64 %31
  %33 = lshr i32 %0, 16
  %34 = and i32 %0, 1023
  %35 = add i32 %0, 1
  %36 = and i32 %35, 1023
  %37 = add i32 %0, 2
  %38 = and i32 %37, 1023
  %39 = add i32 %0, 3
  %40 = and i32 %39, 1023
  %41 = add i32 %0, 4
  %42 = and i32 %41, 1023
  %43 = add i32 %0, 5
  %44 = and i32 %43, 1023
  %45 = add i32 %0, 6
  %46 = and i32 %45, 1023
  %47 = add i32 %0, 7
  %48 = and i32 %47, 1023
  br label %49

49:                                               ; preds = %49, %29
  %50 = phi i32 [ 30, %29 ], [ %112, %49 ]
  %51 = add nuw nsw i32 %50, %33
  %52 = shl i32 %51, 10
  %53 = and i32 %52, 130048
  %54 = add nuw nsw i32 %52, 1024
  %55 = and i32 %54, 130048
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
  br i1 %113, label %114, label %49, !llvm.loop !20021

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
  %163 = shl nuw nsw i32 %159, 17
  %164 = zext nneg i32 %163 to i64
  %165 = getelementptr inbounds [524288 x i32], ptr addrspace(1) @__zluda_agx_tmem_pool, i64 0, i64 %164
  br label %166

166:                                              ; preds = %166, %162
  %167 = phi i32 [ 30, %162 ], [ %229, %166 ]
  %168 = add nuw nsw i32 %167, %33
  %169 = shl i32 %168, 10
  %170 = and i32 %169, 130048
  %171 = add nuw nsw i32 %169, 1024
  %172 = and i32 %171, 130048
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
  br i1 %230, label %231, label %166, !llvm.loop !20021

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

; Function Attrs: nofree norecurse nosync nounwind memory(read, inaccessiblemem: none)
define dso_local i64 @__zluda_ptx_impl_agx_mm_ld_reduce(i64 noundef %0, i32 noundef %1) local_unnamed_addr #3 {
  %3 = and i32 %1, 7
  %4 = lshr i32 %1, 3
  %5 = and i32 %4, 31
  %6 = lshr i32 %1, 8
  %7 = and i32 %6, 3
  %8 = add i64 %0, -140737488355328
  %9 = icmp ult i64 %8, -17523466567680
  br i1 %9, label %16, label %10

10:                                               ; preds = %2
  %11 = lshr i64 %0, 30
  %12 = and i64 %11, 16320
  %13 = add nuw nsw i64 %12, 123145302310848
  %14 = inttoptr i64 %13 to ptr addrspace(1)
  %15 = load i32, ptr addrspace(1) %14, align 64
  br label %16

16:                                               ; preds = %2, %10
  %17 = phi i32 [ %15, %10 ], [ 1, %2 ]
  %18 = and i32 %4, 30
  %19 = icmp eq i32 %18, 12
  br i1 %19, label %23, label %20

20:                                               ; preds = %16
  switch i32 %5, label %21 [
    i32 15, label %23
    i32 14, label %23
    i32 9, label %23
    i32 8, label %23
    i32 7, label %22
    i32 5, label %22
    i32 4, label %22
    i32 3, label %22
  ]

21:                                               ; preds = %20
  br label %23

22:                                               ; preds = %20, %20, %20, %20
  br label %23

23:                                               ; preds = %16, %20, %20, %20, %20, %21, %22
  %24 = phi i32 [ 4, %21 ], [ 1, %16 ], [ 2, %20 ], [ 2, %20 ], [ 2, %20 ], [ 2, %20 ], [ 8, %22 ]
  %25 = icmp eq i32 %7, 0
  br i1 %25, label %27, label %26

26:                                               ; preds = %23
  switch i32 %5, label %90 [
    i32 7, label %27
    i32 5, label %27
    i32 4, label %27
    i32 3, label %27
    i32 2, label %27
    i32 1, label %27
    i32 0, label %27
    i32 8, label %91
    i32 10, label %91
    i32 9, label %87
    i32 11, label %87
    i32 12, label %88
    i32 14, label %88
    i32 16, label %88
    i32 13, label %89
    i32 15, label %89
    i32 17, label %89
  ]

27:                                               ; preds = %26, %26, %26, %26, %26, %26, %26, %23
  br i1 %9, label %36, label %28

28:                                               ; preds = %27
  %29 = and i64 %0, 68719476735
  %30 = lshr i64 %0, 30
  %31 = and i64 %30, 16320
  %32 = add nuw nsw i64 %31, 123145302310856
  %33 = inttoptr i64 %32 to ptr addrspace(1)
  %34 = load i64, ptr addrspace(1) %33, align 8
  %35 = add i64 %34, %29
  br label %36

36:                                               ; preds = %27, %28
  %37 = phi i64 [ %35, %28 ], [ %0, %27 ]
  %38 = inttoptr i64 %37 to ptr addrspace(1)
  switch i32 %24, label %48 [
    i32 1, label %39
    i32 2, label %42
    i32 4, label %45
  ]

39:                                               ; preds = %36
  %40 = load i8, ptr addrspace(1) %38, align 1
  %41 = zext i8 %40 to i64
  br label %50

42:                                               ; preds = %36
  %43 = load i16, ptr addrspace(1) %38, align 2
  %44 = zext i16 %43 to i64
  br label %50

45:                                               ; preds = %36
  %46 = load i32, ptr addrspace(1) %38, align 4
  %47 = zext i32 %46 to i64
  br label %50

48:                                               ; preds = %36
  %49 = load i64, ptr addrspace(1) %38, align 8
  br label %50

50:                                               ; preds = %39, %42, %45, %48
  %51 = phi i64 [ %41, %39 ], [ %44, %42 ], [ %47, %45 ], [ %49, %48 ]
  %52 = icmp ugt i32 %17, 1
  br i1 %52, label %53, label %919

53:                                               ; preds = %50
  %54 = lshr i64 %0, 30
  %55 = and i64 %54, 16320
  %56 = add nuw nsw i64 %55, 123145302310856
  %57 = and i64 %0, 68719476735
  br label %58

58:                                               ; preds = %53, %82
  %59 = phi i64 [ %51, %53 ], [ %84, %82 ]
  %60 = phi i32 [ 1, %53 ], [ %85, %82 ]
  br i1 %9, label %68, label %61

61:                                               ; preds = %58
  %62 = shl i32 %60, 3
  %63 = zext i32 %62 to i64
  %64 = add nuw nsw i64 %56, %63
  %65 = inttoptr i64 %64 to ptr addrspace(1)
  %66 = load i64, ptr addrspace(1) %65, align 8
  %67 = add i64 %66, %57
  br label %68

68:                                               ; preds = %58, %61
  %69 = phi i64 [ %67, %61 ], [ %0, %58 ]
  %70 = inttoptr i64 %69 to ptr addrspace(1)
  switch i32 %24, label %80 [
    i32 1, label %71
    i32 2, label %74
    i32 4, label %77
  ]

71:                                               ; preds = %68
  %72 = load i8, ptr addrspace(1) %70, align 1
  %73 = zext i8 %72 to i64
  br label %82

74:                                               ; preds = %68
  %75 = load i16, ptr addrspace(1) %70, align 2
  %76 = zext i16 %75 to i64
  br label %82

77:                                               ; preds = %68
  %78 = load i32, ptr addrspace(1) %70, align 4
  %79 = zext i32 %78 to i64
  br label %82

80:                                               ; preds = %68
  %81 = load i64, ptr addrspace(1) %70, align 8
  br label %82

82:                                               ; preds = %71, %74, %77, %80
  %83 = phi i64 [ %73, %71 ], [ %76, %74 ], [ %79, %77 ], [ %81, %80 ]
  %84 = tail call fastcc i64 @__zluda_agx_cvt_mm_combine(i32 noundef %3, i32 noundef %5, i64 noundef %59, i64 noundef %83)
  %85 = add nuw i32 %60, 1
  %86 = icmp eq i32 %85, %17
  br i1 %86, label %919, label %58, !llvm.loop !20027

87:                                               ; preds = %26, %26
  br label %91

88:                                               ; preds = %26, %26, %26
  br label %91

89:                                               ; preds = %26, %26, %26
  br label %91

90:                                               ; preds = %26
  br label %91

91:                                               ; preds = %26, %26, %87, %88, %89, %90
  %92 = phi i1 [ true, %90 ], [ false, %89 ], [ false, %88 ], [ false, %87 ], [ false, %26 ], [ false, %26 ]
  %93 = phi i1 [ false, %90 ], [ false, %89 ], [ false, %88 ], [ true, %87 ], [ false, %26 ], [ false, %26 ]
  %94 = phi i1 [ false, %90 ], [ false, %89 ], [ true, %88 ], [ false, %87 ], [ false, %26 ], [ false, %26 ]
  %95 = phi i32 [ 0, %90 ], [ 4, %89 ], [ 3, %88 ], [ 2, %87 ], [ 1, %26 ], [ 1, %26 ]
  %96 = icmp ne i32 %7, 1
  %97 = zext i1 %96 to i32
  switch i32 %5, label %98 [
    i32 15, label %101
    i32 14, label %101
    i32 11, label %101
    i32 10, label %101
  ]

98:                                               ; preds = %91
  %99 = icmp eq i32 %18, 16
  %100 = select i1 %99, i8 4, i8 1
  br label %101

101:                                              ; preds = %91, %91, %91, %91, %98
  %102 = phi i8 [ %100, %98 ], [ 2, %91 ], [ 2, %91 ], [ 2, %91 ], [ 2, %91 ]
  %103 = trunc i32 %24 to i8
  %104 = shl nuw nsw i8 %103, 3
  %105 = udiv i8 %104, %102
  %106 = zext nneg i8 %105 to i32
  %107 = icmp eq i8 %105, 32
  %108 = shl nsw i32 -1, %106
  %109 = xor i32 %108, -1
  %110 = select i1 %107, i32 -1, i32 %109
  %111 = icmp eq i32 %17, 0
  %112 = icmp eq i32 %18, 16
  %113 = select i1 %112, i32 4, i32 1
  %114 = lshr i64 %0, 30
  %115 = and i64 %114, 16320
  %116 = add nuw nsw i64 %115, 123145302310856
  %117 = and i64 %0, 68719476735
  %118 = add nsw i32 %95, -3
  %119 = icmp ult i32 %118, 2
  %120 = select i1 %93, i32 8, i32 5
  %121 = select i1 %93, i32 7, i32 10
  %122 = shl nsw i32 -1, %120
  %123 = xor i32 %122, -1
  %124 = shl nuw nsw i32 %123, %121
  %125 = add nsw i32 %120, -1
  %126 = shl nsw i32 -1, %125
  %127 = xor i32 %126, -1
  %128 = add nuw nsw i32 %126, 2
  %129 = zext nneg i32 %121 to i64
  %130 = shl nuw nsw i64 2, %129
  %131 = zext nneg i32 %124 to i64
  %132 = select i1 %94, i64 126, i64 123
  %133 = select i1 %94, i32 -8, i32 -16
  %134 = xor i32 %133, -1
  %135 = select i1 %94, i32 3, i32 2
  %136 = or disjoint i32 %133, 2
  %137 = zext nneg i32 %135 to i64
  %138 = shl nuw nsw i64 2, %137
  br label %139

139:                                              ; preds = %400, %101
  %140 = phi i64 [ 0, %101 ], [ %407, %400 ]
  %141 = phi i32 [ 0, %101 ], [ %408, %400 ]
  switch i32 %5, label %142 [
    i32 15, label %143
    i32 14, label %143
    i32 11, label %143
    i32 10, label %143
  ]

142:                                              ; preds = %139
  br label %143

143:                                              ; preds = %139, %139, %139, %139, %142
  %144 = phi i32 [ %113, %142 ], [ 2, %139 ], [ 2, %139 ], [ 2, %139 ], [ 2, %139 ]
  %145 = icmp ult i32 %141, %144
  br i1 %145, label %146, label %919

146:                                              ; preds = %143
  br i1 %111, label %150, label %147

147:                                              ; preds = %146
  %148 = mul nuw nsw i32 %141, %106
  %149 = zext nneg i32 %148 to i64
  br label %409

150:                                              ; preds = %915, %146
  %151 = phi double [ 0.000000e+00, %146 ], [ %916, %915 ]
  %152 = bitcast double %151 to i64
  %153 = lshr i64 %152, 52
  %154 = trunc i64 %153 to i32
  %155 = and i32 %154, 2047
  %156 = and i64 %152, 4503599627370495
  switch i32 %155, label %162 [
    i32 2047, label %157
    i32 0, label %165
  ]

157:                                              ; preds = %150
  %158 = icmp ne i64 %156, 0
  %159 = zext i1 %158 to i32
  %160 = icmp eq i64 %156, 0
  %161 = zext i1 %160 to i32
  br label %165

162:                                              ; preds = %150
  %163 = or disjoint i64 %156, 4503599627370496
  %164 = add nsw i32 %155, -1075
  br label %165

165:                                              ; preds = %162, %157, %150
  %166 = phi i32 [ 0, %157 ], [ %164, %162 ], [ -1074, %150 ]
  %167 = phi i64 [ 0, %157 ], [ %163, %162 ], [ %156, %150 ]
  %168 = phi i32 [ %161, %157 ], [ 0, %162 ], [ %155, %150 ]
  %169 = phi i32 [ %159, %157 ], [ 0, %162 ], [ %155, %150 ]
  %170 = icmp eq i32 %169, 0
  br i1 %119, label %171, label %248

171:                                              ; preds = %165
  br i1 %170, label %172, label %400

172:                                              ; preds = %171
  %173 = lshr i64 %152, 56
  %174 = trunc i64 %173 to i32
  %175 = and i32 %174, 128
  %176 = icmp eq i32 %168, 0
  br i1 %176, label %177, label %244

177:                                              ; preds = %172
  %178 = icmp eq i64 %167, 0
  br i1 %178, label %241, label %179

179:                                              ; preds = %177, %179
  %180 = phi i32 [ %183, %179 ], [ 0, %177 ]
  %181 = phi i64 [ %182, %179 ], [ %167, %177 ]
  %182 = lshr i64 %181, 1
  %183 = add nuw nsw i32 %180, 1
  %184 = icmp ult i64 %181, 2
  br i1 %184, label %185, label %179, !llvm.loop !20003

185:                                              ; preds = %179
  %186 = add nsw i32 %183, %166
  %187 = add i32 %180, %166
  %188 = icmp sgt i32 %186, %136
  %189 = select i1 %188, i32 %187, i32 %136
  %190 = icmp sgt i32 %189, 200
  br i1 %190, label %241, label %191

191:                                              ; preds = %185
  %192 = sub i32 %135, %189
  %193 = add i32 %192, %166
  %194 = icmp sgt i32 %193, -1
  br i1 %194, label %195, label %200

195:                                              ; preds = %191
  %196 = icmp ugt i32 %193, 40
  %197 = zext nneg i32 %193 to i64
  %198 = shl i64 %167, %197
  %199 = select i1 %196, i64 4611686018427387904, i64 %198
  br label %225

200:                                              ; preds = %191
  %201 = sub nsw i32 0, %193
  %202 = icmp ult i32 %193, -63
  %203 = zext nneg i32 %201 to i64
  %204 = lshr i64 %167, %203
  %205 = select i1 %202, i64 0, i64 %204
  %206 = shl nsw i64 -1, %203
  %207 = xor i64 %206, -1
  %208 = select i1 %202, i64 -1, i64 %207
  %209 = and i64 %208, %167
  %210 = icmp eq i64 %209, 0
  %211 = or i1 %202, %210
  %212 = select i1 %210, i64 %205, i64 0
  br i1 %211, label %225, label %213

213:                                              ; preds = %200
  %214 = xor i32 %193, -1
  %215 = zext nneg i32 %214 to i64
  %216 = shl nuw nsw i64 1, %215
  %217 = icmp ugt i64 %209, %216
  %218 = icmp eq i64 %209, %216
  %219 = and i64 %204, 1
  %220 = icmp ne i64 %219, 0
  %221 = select i1 %218, i1 %220, i1 false
  %222 = select i1 %217, i1 true, i1 %221
  %223 = zext i1 %222 to i64
  %224 = add nuw nsw i64 %204, %223
  br label %225

225:                                              ; preds = %213, %200, %195
  %226 = phi i64 [ %199, %195 ], [ %224, %213 ], [ %212, %200 ]
  %227 = icmp ult i64 %226, %138
  br i1 %227, label %232, label %228

228:                                              ; preds = %225
  %229 = sub i32 %189, %133
  %230 = sext i32 %229 to i64
  %231 = shl nsw i64 %230, %137
  br label %241

232:                                              ; preds = %225
  %233 = lshr i64 %226, %137
  %234 = icmp eq i64 %233, 0
  %235 = add nsw i32 %189, %134
  %236 = sext i32 %235 to i64
  %237 = add nsw i64 %236, -1
  %238 = shl nsw i64 %237, %137
  %239 = select i1 %234, i64 0, i64 %238
  %240 = add i64 %239, %226
  br label %241

241:                                              ; preds = %232, %228, %185, %177
  %242 = phi i64 [ 0, %177 ], [ 1099511627776, %185 ], [ %231, %228 ], [ %240, %232 ]
  %243 = tail call i64 @llvm.smin.i64(i64 %242, i64 %132)
  br label %244

244:                                              ; preds = %241, %172
  %245 = phi i64 [ %132, %172 ], [ %243, %241 ]
  %246 = trunc i64 %245 to i32
  %247 = or i32 %175, %246
  br label %400

248:                                              ; preds = %165
  br i1 %92, label %249, label %322

249:                                              ; preds = %248
  br i1 %170, label %250, label %400

250:                                              ; preds = %249
  %251 = lshr i64 %152, 32
  %252 = trunc i64 %251 to i32
  %253 = and i32 %252, -2147483648
  %254 = icmp eq i32 %168, 0
  br i1 %254, label %255, label %317

255:                                              ; preds = %250
  %256 = icmp eq i64 %167, 0
  br i1 %256, label %317, label %257

257:                                              ; preds = %255, %257
  %258 = phi i32 [ %261, %257 ], [ 0, %255 ]
  %259 = phi i64 [ %260, %257 ], [ %167, %255 ]
  %260 = lshr i64 %259, 1
  %261 = add nuw nsw i32 %258, 1
  %262 = icmp ult i64 %259, 2
  br i1 %262, label %263, label %257, !llvm.loop !20003

263:                                              ; preds = %257
  %264 = add nsw i32 %261, %166
  %265 = add i32 %258, %166
  %266 = icmp sgt i32 %264, -126
  %267 = select i1 %266, i32 %265, i32 -126
  %268 = icmp sgt i32 %267, 200
  br i1 %268, label %317, label %269

269:                                              ; preds = %263
  %270 = sub i32 %166, %267
  %271 = add i32 %270, 23
  %272 = icmp sgt i32 %271, -1
  br i1 %272, label %273, label %278

273:                                              ; preds = %269
  %274 = icmp ugt i32 %271, 40
  %275 = zext nneg i32 %271 to i64
  %276 = shl i64 %167, %275
  %277 = select i1 %274, i64 4611686018427387904, i64 %276
  br label %303

278:                                              ; preds = %269
  %279 = sub nuw i32 -23, %270
  %280 = icmp ult i32 %271, -63
  %281 = zext nneg i32 %279 to i64
  %282 = lshr i64 %167, %281
  %283 = select i1 %280, i64 0, i64 %282
  %284 = shl nsw i64 -1, %281
  %285 = xor i64 %284, -1
  %286 = select i1 %280, i64 -1, i64 %285
  %287 = and i64 %286, %167
  %288 = icmp eq i64 %287, 0
  %289 = or i1 %280, %288
  %290 = select i1 %288, i64 %283, i64 0
  br i1 %289, label %303, label %291

291:                                              ; preds = %278
  %292 = sub nuw nsw i32 -24, %270
  %293 = zext nneg i32 %292 to i64
  %294 = shl nuw nsw i64 1, %293
  %295 = icmp ugt i64 %287, %294
  %296 = icmp eq i64 %287, %294
  %297 = and i64 %282, 1
  %298 = icmp ne i64 %297, 0
  %299 = select i1 %296, i1 %298, i1 false
  %300 = select i1 %295, i1 true, i1 %299
  %301 = zext i1 %300 to i64
  %302 = add nuw nsw i64 %282, %301
  br label %303

303:                                              ; preds = %291, %278, %273
  %304 = phi i64 [ %277, %273 ], [ %302, %291 ], [ %290, %278 ]
  %305 = icmp ult i64 %304, 16777216
  br i1 %305, label %310, label %306

306:                                              ; preds = %303
  %307 = add nsw i32 %267, 128
  %308 = sext i32 %307 to i64
  %309 = shl nsw i64 %308, 23
  br label %317

310:                                              ; preds = %303
  %311 = icmp ult i64 %304, 8388608
  %312 = sext i32 %267 to i64
  %313 = shl nsw i64 %312, 23
  %314 = add nsw i64 %313, 1056964608
  %315 = select i1 %311, i64 0, i64 %314
  %316 = add i64 %315, %304
  br label %317

317:                                              ; preds = %310, %306, %263, %255, %250
  %318 = phi i64 [ 2139095040, %250 ], [ 0, %255 ], [ 1099511627776, %263 ], [ %309, %306 ], [ %316, %310 ]
  %319 = tail call i64 @llvm.smin.i64(i64 %318, i64 2139095040)
  %320 = trunc i64 %319 to i32
  %321 = or i32 %253, %320
  br label %400

322:                                              ; preds = %248
  br i1 %170, label %323, label %400

323:                                              ; preds = %322
  %324 = lshr i64 %152, 48
  %325 = trunc i64 %324 to i32
  %326 = and i32 %325, 32768
  %327 = icmp eq i32 %168, 0
  br i1 %327, label %330, label %328

328:                                              ; preds = %323
  %329 = or i32 %326, %124
  br label %400

330:                                              ; preds = %323
  %331 = icmp eq i64 %167, 0
  br i1 %331, label %400, label %332

332:                                              ; preds = %330, %332
  %333 = phi i32 [ %336, %332 ], [ 0, %330 ]
  %334 = phi i64 [ %335, %332 ], [ %167, %330 ]
  %335 = lshr i64 %334, 1
  %336 = add nuw nsw i32 %333, 1
  %337 = icmp ult i64 %334, 2
  br i1 %337, label %338, label %332, !llvm.loop !20003

338:                                              ; preds = %332
  %339 = add nsw i32 %336, %166
  %340 = add i32 %333, %166
  %341 = icmp sgt i32 %339, %128
  %342 = select i1 %341, i32 %340, i32 %128
  %343 = icmp sgt i32 %342, 200
  br i1 %343, label %394, label %344

344:                                              ; preds = %338
  %345 = sub i32 %121, %342
  %346 = add i32 %345, %166
  %347 = icmp sgt i32 %346, -1
  br i1 %347, label %348, label %353

348:                                              ; preds = %344
  %349 = icmp ugt i32 %346, 40
  %350 = zext nneg i32 %346 to i64
  %351 = shl i64 %167, %350
  %352 = select i1 %349, i64 4611686018427387904, i64 %351
  br label %378

353:                                              ; preds = %344
  %354 = sub nsw i32 0, %346
  %355 = icmp ult i32 %346, -63
  %356 = zext nneg i32 %354 to i64
  %357 = lshr i64 %167, %356
  %358 = select i1 %355, i64 0, i64 %357
  %359 = shl nsw i64 -1, %356
  %360 = xor i64 %359, -1
  %361 = select i1 %355, i64 -1, i64 %360
  %362 = and i64 %361, %167
  %363 = icmp eq i64 %362, 0
  %364 = or i1 %355, %363
  %365 = select i1 %363, i64 %358, i64 0
  br i1 %364, label %378, label %366

366:                                              ; preds = %353
  %367 = xor i32 %346, -1
  %368 = zext nneg i32 %367 to i64
  %369 = shl nuw nsw i64 1, %368
  %370 = icmp ugt i64 %362, %369
  %371 = icmp eq i64 %362, %369
  %372 = and i64 %357, 1
  %373 = icmp ne i64 %372, 0
  %374 = select i1 %371, i1 %373, i1 false
  %375 = select i1 %370, i1 true, i1 %374
  %376 = zext i1 %375 to i64
  %377 = add nuw nsw i64 %357, %376
  br label %378

378:                                              ; preds = %366, %353, %348
  %379 = phi i64 [ %352, %348 ], [ %377, %366 ], [ %365, %353 ]
  %380 = icmp ult i64 %379, %130
  br i1 %380, label %385, label %381

381:                                              ; preds = %378
  %382 = sub i32 %342, %126
  %383 = sext i32 %382 to i64
  %384 = shl nsw i64 %383, %129
  br label %394

385:                                              ; preds = %378
  %386 = lshr i64 %379, %129
  %387 = icmp eq i64 %386, 0
  %388 = add nsw i32 %342, %127
  %389 = sext i32 %388 to i64
  %390 = add nsw i64 %389, -1
  %391 = shl nsw i64 %390, %129
  %392 = select i1 %387, i64 0, i64 %391
  %393 = add i64 %392, %379
  br label %394

394:                                              ; preds = %385, %381, %338
  %395 = phi i64 [ 1099511627776, %338 ], [ %384, %381 ], [ %393, %385 ]
  %396 = icmp slt i64 %395, %131
  %397 = trunc i64 %395 to i32
  %398 = select i1 %396, i32 %397, i32 %124
  %399 = or i32 %398, %326
  br label %400

400:                                              ; preds = %171, %244, %249, %317, %322, %328, %330, %394
  %401 = phi i32 [ %321, %317 ], [ 2147483647, %249 ], [ 32767, %322 ], [ %247, %244 ], [ 127, %171 ], [ %329, %328 ], [ %399, %394 ], [ %326, %330 ]
  %402 = and i32 %401, %110
  %403 = zext i32 %402 to i64
  %404 = mul nuw nsw i32 %141, %106
  %405 = zext nneg i32 %404 to i64
  %406 = shl i64 %403, %405
  %407 = or i64 %406, %140
  %408 = add nuw nsw i32 %141, 1
  br label %139, !llvm.loop !20028

409:                                              ; preds = %147, %915
  %410 = phi i32 [ 0, %147 ], [ %917, %915 ]
  %411 = phi double [ 0.000000e+00, %147 ], [ %916, %915 ]
  br i1 %9, label %419, label %412

412:                                              ; preds = %409
  %413 = shl i32 %410, 3
  %414 = zext i32 %413 to i64
  %415 = add nuw nsw i64 %116, %414
  %416 = inttoptr i64 %415 to ptr addrspace(1)
  %417 = load i64, ptr addrspace(1) %416, align 8
  %418 = add i64 %417, %117
  br label %419

419:                                              ; preds = %409, %412
  %420 = phi i64 [ %418, %412 ], [ %0, %409 ]
  %421 = inttoptr i64 %420 to ptr addrspace(1)
  switch i32 %24, label %431 [
    i32 1, label %422
    i32 2, label %425
    i32 4, label %428
  ]

422:                                              ; preds = %419
  %423 = load i8, ptr addrspace(1) %421, align 1
  %424 = zext i8 %423 to i64
  br label %433

425:                                              ; preds = %419
  %426 = load i16, ptr addrspace(1) %421, align 2
  %427 = zext i16 %426 to i64
  br label %433

428:                                              ; preds = %419
  %429 = load i32, ptr addrspace(1) %421, align 4
  %430 = zext i32 %429 to i64
  br label %433

431:                                              ; preds = %419
  %432 = load i64, ptr addrspace(1) %421, align 8
  br label %433

433:                                              ; preds = %422, %425, %428, %431
  %434 = phi i64 [ %424, %422 ], [ %427, %425 ], [ %430, %428 ], [ %432, %431 ]
  %435 = lshr i64 %434, %149
  %436 = trunc i64 %435 to i32
  %437 = and i32 %110, %436
  switch i32 %95, label %608 [
    i32 0, label %438
    i32 1, label %481
    i32 2, label %524
    i32 3, label %568
  ]

438:                                              ; preds = %433
  %439 = lshr i32 %437, 23
  %440 = and i32 %437, 8388607
  %441 = trunc i32 %439 to i8
  switch i8 %441, label %445 [
    i8 -1, label %442
    i8 0, label %449
  ]

442:                                              ; preds = %438
  %443 = icmp ne i32 %440, 0
  %444 = icmp eq i32 %440, 0
  br label %449

445:                                              ; preds = %438
  %446 = and i32 %439, 255
  %447 = or disjoint i32 %440, 8388608
  %448 = add nuw nsw i32 %446, 873
  br label %449

449:                                              ; preds = %445, %442, %438
  %450 = phi i32 [ 1023, %442 ], [ %448, %445 ], [ 874, %438 ]
  %451 = phi i32 [ 0, %442 ], [ %447, %445 ], [ %440, %438 ]
  %452 = phi i1 [ %444, %442 ], [ false, %445 ], [ false, %438 ]
  %453 = phi i1 [ %443, %442 ], [ false, %445 ], [ false, %438 ]
  %454 = zext nneg i32 %451 to i64
  %455 = or i1 %452, %453
  %456 = select i1 %453, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %455, label %475, label %457

457:                                              ; preds = %449
  %458 = icmp eq i32 %451, 0
  br i1 %458, label %475, label %459

459:                                              ; preds = %457, %459
  %460 = phi i32 [ %463, %459 ], [ 0, %457 ]
  %461 = phi i64 [ %462, %459 ], [ %454, %457 ]
  %462 = lshr i64 %461, 1
  %463 = add nuw nsw i32 %460, 1
  %464 = icmp ult i64 %461, 2
  br i1 %464, label %465, label %459, !llvm.loop !20003

465:                                              ; preds = %459
  %466 = add i32 %460, %450
  %467 = zext i32 %466 to i64
  %468 = shl i64 %467, 52
  %469 = sub nsw i32 52, %460
  %470 = zext nneg i32 %469 to i64
  %471 = shl i64 %454, %470
  %472 = and i64 %471, 4503599627370495
  %473 = or disjoint i64 %472, %468
  %474 = bitcast i64 %473 to double
  br label %475

475:                                              ; preds = %465, %457, %449
  %476 = phi double [ %474, %465 ], [ %456, %449 ], [ 0.000000e+00, %457 ]
  %477 = tail call double @llvm.fabs.f64(double %476)
  %478 = fneg double %477
  %479 = icmp slt i32 %437, 0
  %480 = select i1 %479, double %478, double %476
  br label %653

481:                                              ; preds = %433
  %482 = lshr i32 %437, 10
  %483 = and i32 %482, 31
  %484 = and i32 %437, 1023
  switch i32 %483, label %488 [
    i32 31, label %485
    i32 0, label %491
  ]

485:                                              ; preds = %481
  %486 = icmp ne i32 %484, 0
  %487 = icmp eq i32 %484, 0
  br label %491

488:                                              ; preds = %481
  %489 = or disjoint i32 %484, 1024
  %490 = add nuw nsw i32 %483, 998
  br label %491

491:                                              ; preds = %488, %485, %481
  %492 = phi i32 [ 1023, %485 ], [ %490, %488 ], [ 999, %481 ]
  %493 = phi i32 [ 0, %485 ], [ %489, %488 ], [ %484, %481 ]
  %494 = phi i1 [ %487, %485 ], [ false, %488 ], [ false, %481 ]
  %495 = phi i1 [ %486, %485 ], [ false, %488 ], [ false, %481 ]
  %496 = zext nneg i32 %493 to i64
  %497 = or i1 %494, %495
  %498 = select i1 %495, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %497, label %517, label %499

499:                                              ; preds = %491
  %500 = icmp eq i32 %493, 0
  br i1 %500, label %517, label %501

501:                                              ; preds = %499, %501
  %502 = phi i32 [ %505, %501 ], [ 0, %499 ]
  %503 = phi i64 [ %504, %501 ], [ %496, %499 ]
  %504 = lshr i64 %503, 1
  %505 = add nuw nsw i32 %502, 1
  %506 = icmp ult i64 %503, 2
  br i1 %506, label %507, label %501, !llvm.loop !20003

507:                                              ; preds = %501
  %508 = add i32 %502, %492
  %509 = zext i32 %508 to i64
  %510 = shl i64 %509, 52
  %511 = sub nsw i32 52, %502
  %512 = zext nneg i32 %511 to i64
  %513 = shl i64 %496, %512
  %514 = and i64 %513, 4503599627370495
  %515 = or disjoint i64 %514, %510
  %516 = bitcast i64 %515 to double
  br label %517

517:                                              ; preds = %507, %499, %491
  %518 = phi double [ %516, %507 ], [ %498, %491 ], [ 0.000000e+00, %499 ]
  %519 = and i32 %437, 32768
  %520 = icmp eq i32 %519, 0
  %521 = tail call double @llvm.fabs.f64(double %518)
  %522 = fneg double %521
  %523 = select i1 %520, double %518, double %522
  br label %653

524:                                              ; preds = %433
  %525 = shl i32 %437, 16
  %526 = lshr i32 %525, 23
  %527 = and i32 %525, 8323072
  %528 = trunc i32 %526 to i8
  switch i8 %528, label %532 [
    i8 -1, label %529
    i8 0, label %536
  ]

529:                                              ; preds = %524
  %530 = icmp ne i32 %527, 0
  %531 = icmp eq i32 %527, 0
  br label %536

532:                                              ; preds = %524
  %533 = and i32 %526, 255
  %534 = or disjoint i32 %527, 8388608
  %535 = add nuw nsw i32 %533, 873
  br label %536

536:                                              ; preds = %532, %529, %524
  %537 = phi i32 [ 1023, %529 ], [ %535, %532 ], [ 874, %524 ]
  %538 = phi i32 [ 0, %529 ], [ %534, %532 ], [ %527, %524 ]
  %539 = phi i1 [ %531, %529 ], [ false, %532 ], [ false, %524 ]
  %540 = phi i1 [ %530, %529 ], [ false, %532 ], [ false, %524 ]
  %541 = zext nneg i32 %538 to i64
  %542 = or i1 %539, %540
  %543 = select i1 %540, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %542, label %562, label %544

544:                                              ; preds = %536
  %545 = icmp eq i32 %538, 0
  br i1 %545, label %562, label %546

546:                                              ; preds = %544, %546
  %547 = phi i32 [ %550, %546 ], [ 0, %544 ]
  %548 = phi i64 [ %549, %546 ], [ %541, %544 ]
  %549 = lshr i64 %548, 1
  %550 = add nuw nsw i32 %547, 1
  %551 = icmp ult i64 %548, 2
  br i1 %551, label %552, label %546, !llvm.loop !20003

552:                                              ; preds = %546
  %553 = add i32 %547, %537
  %554 = zext i32 %553 to i64
  %555 = shl i64 %554, 52
  %556 = sub nsw i32 52, %547
  %557 = zext nneg i32 %556 to i64
  %558 = shl i64 %541, %557
  %559 = and i64 %558, 4503599627370495
  %560 = or disjoint i64 %559, %555
  %561 = bitcast i64 %560 to double
  br label %562

562:                                              ; preds = %552, %544, %536
  %563 = phi double [ %561, %552 ], [ %543, %536 ], [ 0.000000e+00, %544 ]
  %564 = tail call double @llvm.fabs.f64(double %563)
  %565 = fneg double %564
  %566 = icmp slt i32 %525, 0
  %567 = select i1 %566, double %565, double %563
  br label %653

568:                                              ; preds = %433
  %569 = and i32 %437, 127
  %570 = icmp eq i32 %569, 127
  br i1 %570, label %580, label %571

571:                                              ; preds = %568
  %572 = lshr i32 %569, 3
  %573 = and i32 %437, 7
  %574 = icmp ult i32 %569, 8
  %575 = or disjoint i32 %573, 8
  %576 = add nuw nsw i32 %572, 1013
  %577 = select i1 %574, i32 %573, i32 %575
  %578 = zext nneg i32 %577 to i64
  %579 = select i1 %574, i32 1014, i32 %576
  br label %580

580:                                              ; preds = %571, %568
  %581 = phi i32 [ 1023, %568 ], [ %579, %571 ]
  %582 = phi i64 [ 0, %568 ], [ %578, %571 ]
  br i1 %570, label %601, label %583

583:                                              ; preds = %580
  %584 = icmp eq i64 %582, 0
  br i1 %584, label %601, label %585

585:                                              ; preds = %583, %585
  %586 = phi i32 [ %589, %585 ], [ 0, %583 ]
  %587 = phi i64 [ %588, %585 ], [ %582, %583 ]
  %588 = lshr i64 %587, 1
  %589 = add nuw nsw i32 %586, 1
  %590 = icmp ult i64 %587, 2
  br i1 %590, label %591, label %585, !llvm.loop !20003

591:                                              ; preds = %585
  %592 = add i32 %586, %581
  %593 = zext i32 %592 to i64
  %594 = shl i64 %593, 52
  %595 = sub nsw i32 52, %586
  %596 = zext nneg i32 %595 to i64
  %597 = shl i64 %582, %596
  %598 = and i64 %597, 4503599627370495
  %599 = or disjoint i64 %598, %594
  %600 = bitcast i64 %599 to double
  br label %601

601:                                              ; preds = %591, %583, %580
  %602 = phi double [ %600, %591 ], [ 0x7FF8000000000000, %580 ], [ 0.000000e+00, %583 ]
  %603 = and i32 %437, 128
  %604 = icmp eq i32 %603, 0
  %605 = tail call double @llvm.fabs.f64(double %602)
  %606 = fneg double %605
  %607 = select i1 %604, double %602, double %606
  br label %653

608:                                              ; preds = %433
  %609 = and i32 %437, 127
  %610 = icmp ugt i32 %609, 124
  br i1 %610, label %622, label %611

611:                                              ; preds = %608
  %612 = icmp eq i32 %609, 124
  br i1 %612, label %622, label %613

613:                                              ; preds = %611
  %614 = lshr i32 %609, 2
  %615 = and i32 %437, 3
  %616 = icmp ult i32 %609, 4
  %617 = or disjoint i32 %615, 4
  %618 = add nuw nsw i32 %614, 1006
  %619 = select i1 %616, i32 %615, i32 %617
  %620 = zext nneg i32 %619 to i64
  %621 = select i1 %616, i32 1007, i32 %618
  br label %622

622:                                              ; preds = %613, %611, %608
  %623 = phi double [ 0x7FF8000000000000, %608 ], [ 0x7FF0000000000000, %611 ], [ 0x7FF0000000000000, %613 ]
  %624 = phi i32 [ 1023, %608 ], [ 1023, %611 ], [ %621, %613 ]
  %625 = phi i64 [ 0, %608 ], [ 0, %611 ], [ %620, %613 ]
  %626 = phi i1 [ false, %608 ], [ true, %611 ], [ false, %613 ]
  %627 = or i1 %610, %626
  br i1 %627, label %646, label %628

628:                                              ; preds = %622
  %629 = icmp eq i64 %625, 0
  br i1 %629, label %646, label %630

630:                                              ; preds = %628, %630
  %631 = phi i32 [ %634, %630 ], [ 0, %628 ]
  %632 = phi i64 [ %633, %630 ], [ %625, %628 ]
  %633 = lshr i64 %632, 1
  %634 = add nuw nsw i32 %631, 1
  %635 = icmp ult i64 %632, 2
  br i1 %635, label %636, label %630, !llvm.loop !20003

636:                                              ; preds = %630
  %637 = add i32 %631, %624
  %638 = zext i32 %637 to i64
  %639 = shl i64 %638, 52
  %640 = sub nsw i32 52, %631
  %641 = zext nneg i32 %640 to i64
  %642 = shl i64 %625, %641
  %643 = and i64 %642, 4503599627370495
  %644 = or disjoint i64 %643, %639
  %645 = bitcast i64 %644 to double
  br label %646

646:                                              ; preds = %636, %628, %622
  %647 = phi double [ %645, %636 ], [ %623, %622 ], [ 0.000000e+00, %628 ]
  %648 = and i32 %437, 128
  %649 = icmp eq i32 %648, 0
  %650 = tail call double @llvm.fabs.f64(double %647)
  %651 = fneg double %650
  %652 = select i1 %649, double %647, double %651
  br label %653

653:                                              ; preds = %475, %517, %562, %601, %646
  %654 = phi double [ %652, %646 ], [ %607, %601 ], [ %567, %562 ], [ %523, %517 ], [ %480, %475 ]
  %655 = bitcast double %654 to i64
  %656 = lshr i64 %655, 52
  %657 = trunc i64 %656 to i32
  %658 = and i32 %657, 2047
  %659 = and i64 %655, 4503599627370495
  switch i32 %658, label %665 [
    i32 2047, label %660
    i32 0, label %668
  ]

660:                                              ; preds = %653
  %661 = icmp ne i64 %659, 0
  %662 = zext i1 %661 to i32
  %663 = icmp eq i64 %659, 0
  %664 = zext i1 %663 to i32
  br label %668

665:                                              ; preds = %653
  %666 = or disjoint i64 %659, 4503599627370496
  %667 = add nsw i32 %658, -1075
  br label %668

668:                                              ; preds = %665, %660, %653
  %669 = phi i32 [ 0, %660 ], [ %667, %665 ], [ -1074, %653 ]
  %670 = phi i64 [ 0, %660 ], [ %666, %665 ], [ %659, %653 ]
  %671 = phi i32 [ %664, %660 ], [ 0, %665 ], [ %658, %653 ]
  %672 = phi i32 [ %662, %660 ], [ 0, %665 ], [ %658, %653 ]
  %673 = icmp eq i32 %672, 0
  br i1 %96, label %747, label %674

674:                                              ; preds = %668
  br i1 %673, label %675, label %822

675:                                              ; preds = %674
  %676 = lshr i64 %655, 32
  %677 = trunc i64 %676 to i32
  %678 = and i32 %677, -2147483648
  %679 = icmp eq i32 %671, 0
  br i1 %679, label %680, label %742

680:                                              ; preds = %675
  %681 = icmp eq i64 %670, 0
  br i1 %681, label %742, label %682

682:                                              ; preds = %680, %682
  %683 = phi i32 [ %686, %682 ], [ 0, %680 ]
  %684 = phi i64 [ %685, %682 ], [ %670, %680 ]
  %685 = lshr i64 %684, 1
  %686 = add nuw nsw i32 %683, 1
  %687 = icmp ult i64 %684, 2
  br i1 %687, label %688, label %682, !llvm.loop !20003

688:                                              ; preds = %682
  %689 = add nsw i32 %686, %669
  %690 = add i32 %683, %669
  %691 = icmp sgt i32 %689, -126
  %692 = select i1 %691, i32 %690, i32 -126
  %693 = icmp sgt i32 %692, 200
  br i1 %693, label %742, label %694

694:                                              ; preds = %688
  %695 = sub i32 %669, %692
  %696 = add i32 %695, 23
  %697 = icmp sgt i32 %696, -1
  br i1 %697, label %698, label %703

698:                                              ; preds = %694
  %699 = icmp ugt i32 %696, 40
  %700 = zext nneg i32 %696 to i64
  %701 = shl i64 %670, %700
  %702 = select i1 %699, i64 4611686018427387904, i64 %701
  br label %728

703:                                              ; preds = %694
  %704 = sub nuw i32 -23, %695
  %705 = icmp ult i32 %696, -63
  %706 = zext nneg i32 %704 to i64
  %707 = lshr i64 %670, %706
  %708 = select i1 %705, i64 0, i64 %707
  %709 = shl nsw i64 -1, %706
  %710 = xor i64 %709, -1
  %711 = select i1 %705, i64 -1, i64 %710
  %712 = and i64 %711, %670
  %713 = icmp eq i64 %712, 0
  %714 = or i1 %705, %713
  %715 = select i1 %713, i64 %708, i64 0
  br i1 %714, label %728, label %716

716:                                              ; preds = %703
  %717 = sub nuw nsw i32 -24, %695
  %718 = zext nneg i32 %717 to i64
  %719 = shl nuw nsw i64 1, %718
  %720 = icmp ugt i64 %712, %719
  %721 = icmp eq i64 %712, %719
  %722 = and i64 %707, 1
  %723 = icmp ne i64 %722, 0
  %724 = select i1 %721, i1 %723, i1 false
  %725 = select i1 %720, i1 true, i1 %724
  %726 = zext i1 %725 to i64
  %727 = add nuw nsw i64 %707, %726
  br label %728

728:                                              ; preds = %716, %703, %698
  %729 = phi i64 [ %702, %698 ], [ %727, %716 ], [ %715, %703 ]
  %730 = icmp ult i64 %729, 16777216
  br i1 %730, label %735, label %731

731:                                              ; preds = %728
  %732 = add nsw i32 %692, 128
  %733 = sext i32 %732 to i64
  %734 = shl nsw i64 %733, 23
  br label %742

735:                                              ; preds = %728
  %736 = icmp ult i64 %729, 8388608
  %737 = sext i32 %692 to i64
  %738 = shl nsw i64 %737, 23
  %739 = add nsw i64 %738, 1056964608
  %740 = select i1 %736, i64 0, i64 %739
  %741 = add i64 %740, %729
  br label %742

742:                                              ; preds = %735, %731, %688, %680, %675
  %743 = phi i64 [ 2139095040, %675 ], [ 0, %680 ], [ 1099511627776, %688 ], [ %734, %731 ], [ %741, %735 ]
  %744 = tail call i64 @llvm.smin.i64(i64 %743, i64 2139095040)
  %745 = trunc i64 %744 to i32
  %746 = or i32 %678, %745
  br label %822

747:                                              ; preds = %668
  br i1 %673, label %748, label %822

748:                                              ; preds = %747
  %749 = lshr i64 %655, 48
  %750 = trunc i64 %749 to i32
  %751 = and i32 %750, 32768
  %752 = icmp eq i32 %671, 0
  br i1 %752, label %755, label %753

753:                                              ; preds = %748
  %754 = or disjoint i32 %751, 31744
  br label %822

755:                                              ; preds = %748
  %756 = icmp eq i64 %670, 0
  br i1 %756, label %822, label %757

757:                                              ; preds = %755, %757
  %758 = phi i32 [ %761, %757 ], [ 0, %755 ]
  %759 = phi i64 [ %760, %757 ], [ %670, %755 ]
  %760 = lshr i64 %759, 1
  %761 = add nuw nsw i32 %758, 1
  %762 = icmp ult i64 %759, 2
  br i1 %762, label %763, label %757, !llvm.loop !20003

763:                                              ; preds = %757
  %764 = add nsw i32 %761, %669
  %765 = add i32 %758, %669
  %766 = icmp sgt i32 %764, -14
  %767 = select i1 %766, i32 %765, i32 -14
  %768 = icmp sgt i32 %767, 200
  br i1 %768, label %817, label %769

769:                                              ; preds = %763
  %770 = sub i32 %669, %767
  %771 = add i32 %770, 10
  %772 = icmp sgt i32 %771, -1
  br i1 %772, label %773, label %778

773:                                              ; preds = %769
  %774 = icmp ugt i32 %771, 40
  %775 = zext nneg i32 %771 to i64
  %776 = shl i64 %670, %775
  %777 = select i1 %774, i64 4611686018427387904, i64 %776
  br label %803

778:                                              ; preds = %769
  %779 = sub nuw i32 -10, %770
  %780 = icmp ult i32 %771, -63
  %781 = zext nneg i32 %779 to i64
  %782 = lshr i64 %670, %781
  %783 = select i1 %780, i64 0, i64 %782
  %784 = shl nsw i64 -1, %781
  %785 = xor i64 %784, -1
  %786 = select i1 %780, i64 -1, i64 %785
  %787 = and i64 %786, %670
  %788 = icmp eq i64 %787, 0
  %789 = or i1 %780, %788
  %790 = select i1 %788, i64 %783, i64 0
  br i1 %789, label %803, label %791

791:                                              ; preds = %778
  %792 = sub nuw nsw i32 -11, %770
  %793 = zext nneg i32 %792 to i64
  %794 = shl nuw nsw i64 1, %793
  %795 = icmp ugt i64 %787, %794
  %796 = icmp eq i64 %787, %794
  %797 = and i64 %782, 1
  %798 = icmp ne i64 %797, 0
  %799 = select i1 %796, i1 %798, i1 false
  %800 = select i1 %795, i1 true, i1 %799
  %801 = zext i1 %800 to i64
  %802 = add nuw nsw i64 %782, %801
  br label %803

803:                                              ; preds = %791, %778, %773
  %804 = phi i64 [ %777, %773 ], [ %802, %791 ], [ %790, %778 ]
  %805 = icmp ult i64 %804, 2048
  br i1 %805, label %810, label %806

806:                                              ; preds = %803
  %807 = add nsw i32 %767, 16
  %808 = sext i32 %807 to i64
  %809 = shl nsw i64 %808, 10
  br label %817

810:                                              ; preds = %803
  %811 = icmp ult i64 %804, 1024
  %812 = sext i32 %767 to i64
  %813 = shl nsw i64 %812, 10
  %814 = add nsw i64 %813, 14336
  %815 = select i1 %811, i64 0, i64 %814
  %816 = add i64 %815, %804
  br label %817

817:                                              ; preds = %810, %806, %763
  %818 = phi i64 [ 1099511627776, %763 ], [ %809, %806 ], [ %816, %810 ]
  %819 = tail call i64 @llvm.smin.i64(i64 %818, i64 31744)
  %820 = trunc i64 %819 to i32
  %821 = or i32 %751, %820
  br label %822

822:                                              ; preds = %817, %755, %753, %747, %742, %674
  %823 = phi i32 [ %746, %742 ], [ 2147483647, %674 ], [ 32767, %747 ], [ %754, %753 ], [ %821, %817 ], [ %751, %755 ]
  br i1 %96, label %867, label %824

824:                                              ; preds = %822
  %825 = lshr i32 %823, 23
  %826 = and i32 %823, 8388607
  %827 = trunc i32 %825 to i8
  switch i8 %827, label %831 [
    i8 -1, label %828
    i8 0, label %835
  ]

828:                                              ; preds = %824
  %829 = icmp ne i32 %826, 0
  %830 = icmp eq i32 %826, 0
  br label %835

831:                                              ; preds = %824
  %832 = and i32 %825, 255
  %833 = or disjoint i32 %826, 8388608
  %834 = add nuw nsw i32 %832, 873
  br label %835

835:                                              ; preds = %831, %828, %824
  %836 = phi i32 [ 1023, %828 ], [ %834, %831 ], [ 874, %824 ]
  %837 = phi i32 [ 0, %828 ], [ %833, %831 ], [ %826, %824 ]
  %838 = phi i1 [ %830, %828 ], [ false, %831 ], [ false, %824 ]
  %839 = phi i1 [ %829, %828 ], [ false, %831 ], [ false, %824 ]
  %840 = zext nneg i32 %837 to i64
  %841 = or i1 %838, %839
  %842 = select i1 %839, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %841, label %861, label %843

843:                                              ; preds = %835
  %844 = icmp eq i32 %837, 0
  br i1 %844, label %861, label %845

845:                                              ; preds = %843, %845
  %846 = phi i32 [ %849, %845 ], [ 0, %843 ]
  %847 = phi i64 [ %848, %845 ], [ %840, %843 ]
  %848 = lshr i64 %847, 1
  %849 = add nuw nsw i32 %846, 1
  %850 = icmp ult i64 %847, 2
  br i1 %850, label %851, label %845, !llvm.loop !20003

851:                                              ; preds = %845
  %852 = add i32 %846, %836
  %853 = zext i32 %852 to i64
  %854 = shl i64 %853, 52
  %855 = sub nsw i32 52, %846
  %856 = zext nneg i32 %855 to i64
  %857 = shl i64 %840, %856
  %858 = and i64 %857, 4503599627370495
  %859 = or disjoint i64 %858, %854
  %860 = bitcast i64 %859 to double
  br label %861

861:                                              ; preds = %851, %843, %835
  %862 = phi double [ %860, %851 ], [ %842, %835 ], [ 0.000000e+00, %843 ]
  %863 = tail call double @llvm.fabs.f64(double %862)
  %864 = fneg double %863
  %865 = icmp slt i32 %823, 0
  %866 = select i1 %865, double %864, double %862
  br label %910

867:                                              ; preds = %822
  %868 = lshr i32 %823, 10
  %869 = and i32 %868, 31
  %870 = and i32 %823, 1023
  switch i32 %869, label %874 [
    i32 31, label %871
    i32 0, label %877
  ]

871:                                              ; preds = %867
  %872 = icmp ne i32 %870, 0
  %873 = icmp eq i32 %870, 0
  br label %877

874:                                              ; preds = %867
  %875 = or disjoint i32 %870, 1024
  %876 = add nuw nsw i32 %869, 998
  br label %877

877:                                              ; preds = %874, %871, %867
  %878 = phi i32 [ 1023, %871 ], [ %876, %874 ], [ 999, %867 ]
  %879 = phi i32 [ 0, %871 ], [ %875, %874 ], [ %870, %867 ]
  %880 = phi i1 [ %873, %871 ], [ false, %874 ], [ false, %867 ]
  %881 = phi i1 [ %872, %871 ], [ false, %874 ], [ false, %867 ]
  %882 = zext nneg i32 %879 to i64
  %883 = or i1 %880, %881
  %884 = select i1 %881, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %883, label %903, label %885

885:                                              ; preds = %877
  %886 = icmp eq i32 %879, 0
  br i1 %886, label %903, label %887

887:                                              ; preds = %885, %887
  %888 = phi i32 [ %891, %887 ], [ 0, %885 ]
  %889 = phi i64 [ %890, %887 ], [ %882, %885 ]
  %890 = lshr i64 %889, 1
  %891 = add nuw nsw i32 %888, 1
  %892 = icmp ult i64 %889, 2
  br i1 %892, label %893, label %887, !llvm.loop !20003

893:                                              ; preds = %887
  %894 = add i32 %888, %878
  %895 = zext i32 %894 to i64
  %896 = shl i64 %895, 52
  %897 = sub nsw i32 52, %888
  %898 = zext nneg i32 %897 to i64
  %899 = shl i64 %882, %898
  %900 = and i64 %899, 4503599627370495
  %901 = or disjoint i64 %900, %896
  %902 = bitcast i64 %901 to double
  br label %903

903:                                              ; preds = %893, %885, %877
  %904 = phi double [ %902, %893 ], [ %884, %877 ], [ 0.000000e+00, %885 ]
  %905 = and i32 %823, 32768
  %906 = icmp eq i32 %905, 0
  %907 = tail call double @llvm.fabs.f64(double %904)
  %908 = fneg double %907
  %909 = select i1 %906, double %904, double %908
  br label %910

910:                                              ; preds = %861, %903
  %911 = phi double [ %909, %903 ], [ %866, %861 ]
  %912 = icmp eq i32 %410, 0
  br i1 %912, label %915, label %913

913:                                              ; preds = %910
  %914 = tail call fastcc double @__zluda_agx_cvt_mm_comb(i32 noundef %3, i32 noundef %97, double noundef %411, double noundef %911)
  br label %915

915:                                              ; preds = %910, %913
  %916 = phi double [ %914, %913 ], [ %911, %910 ]
  %917 = add nuw i32 %410, 1
  %918 = icmp eq i32 %917, %17
  br i1 %918, label %150, label %409, !llvm.loop !20029

919:                                              ; preds = %143, %82, %50
  %920 = phi i64 [ %51, %50 ], [ %84, %82 ], [ %140, %143 ]
  ret i64 %920
}

; Function Attrs: nofree norecurse nosync nounwind memory(none)
define internal fastcc i64 @__zluda_agx_cvt_mm_combine(i32 noundef %0, i32 noundef %1, i64 noundef %2, i64 noundef %3) unnamed_addr #3 {
  %5 = icmp ult i32 %1, 6
  br i1 %5, label %6, label %44

6:                                                ; preds = %4
  %7 = icmp ugt i32 %1, 2
  %8 = select i1 %7, i64 -1, i64 4294967295
  %9 = shl i64 %2, 32
  %10 = ashr exact i64 %9, 32
  %11 = select i1 %7, i64 %2, i64 %10
  %12 = shl i64 %3, 32
  %13 = ashr exact i64 %12, 32
  %14 = select i1 %7, i64 %3, i64 %13
  %15 = and i64 %8, %2
  %16 = and i64 %8, %3
  switch i32 %0, label %42 [
    i32 0, label %17
    i32 1, label %20
    i32 2, label %29
    i32 3, label %38
    i32 4, label %40
  ]

17:                                               ; preds = %6
  %18 = add i64 %16, %15
  %19 = and i64 %18, %8
  br label %826

20:                                               ; preds = %6
  switch i32 %1, label %23 [
    i32 5, label %21
    i32 2, label %21
  ]

21:                                               ; preds = %20, %20
  %22 = icmp slt i64 %14, %11
  br label %25

23:                                               ; preds = %20
  %24 = icmp ult i64 %16, %15
  br label %25

25:                                               ; preds = %23, %21
  %26 = phi i1 [ %24, %23 ], [ %22, %21 ]
  %27 = select i1 %26, i64 %3, i64 %2
  %28 = and i64 %27, %8
  br label %826

29:                                               ; preds = %6
  switch i32 %1, label %32 [
    i32 5, label %30
    i32 2, label %30
  ]

30:                                               ; preds = %29, %29
  %31 = icmp sgt i64 %14, %11
  br label %34

32:                                               ; preds = %29
  %33 = icmp ugt i64 %16, %15
  br label %34

34:                                               ; preds = %32, %30
  %35 = phi i1 [ %33, %32 ], [ %31, %30 ]
  %36 = select i1 %35, i64 %3, i64 %2
  %37 = and i64 %36, %8
  br label %826

38:                                               ; preds = %6
  %39 = and i64 %15, %3
  br label %826

40:                                               ; preds = %6
  %41 = or i64 %16, %15
  br label %826

42:                                               ; preds = %6
  %43 = xor i64 %16, %15
  br label %826

44:                                               ; preds = %4
  switch i32 %1, label %73 [
    i32 7, label %45
    i32 8, label %74
    i32 10, label %74
    i32 9, label %70
    i32 11, label %70
    i32 12, label %71
    i32 14, label %71
    i32 16, label %71
    i32 13, label %72
    i32 15, label %72
    i32 17, label %72
  ]

45:                                               ; preds = %44
  %46 = bitcast i64 %2 to double
  %47 = bitcast i64 %3 to double
  %48 = icmp eq i32 %0, 0
  br i1 %48, label %49, label %51

49:                                               ; preds = %45
  %50 = fadd double %46, %47
  br label %67

51:                                               ; preds = %45
  %52 = fcmp uno double %46, 0.000000e+00
  br i1 %52, label %67, label %53

53:                                               ; preds = %51
  %54 = fcmp uno double %47, 0.000000e+00
  br i1 %54, label %67, label %55

55:                                               ; preds = %53
  %56 = fcmp oeq double %46, %47
  %57 = icmp eq i32 %0, 1
  br i1 %56, label %58, label %63

58:                                               ; preds = %55
  %59 = or i64 %3, %2
  %60 = and i64 %3, %2
  %61 = select i1 %57, i64 %59, i64 %60
  %62 = bitcast i64 %61 to double
  br label %67

63:                                               ; preds = %55
  %64 = fcmp uge double %46, %47
  %65 = xor i1 %57, %64
  %66 = select i1 %65, double %46, double %47
  br label %67

67:                                               ; preds = %49, %51, %53, %58, %63
  %68 = phi double [ %50, %49 ], [ %62, %58 ], [ %66, %63 ], [ %47, %51 ], [ %46, %53 ]
  %69 = bitcast double %68 to i64
  br label %826

70:                                               ; preds = %44, %44
  br label %74

71:                                               ; preds = %44, %44, %44
  br label %74

72:                                               ; preds = %44, %44, %44
  br label %74

73:                                               ; preds = %44
  br label %74

74:                                               ; preds = %44, %44, %70, %71, %72, %73
  %75 = phi i1 [ true, %73 ], [ false, %72 ], [ false, %71 ], [ false, %70 ], [ false, %44 ], [ false, %44 ]
  %76 = phi i1 [ false, %73 ], [ false, %72 ], [ false, %71 ], [ true, %70 ], [ false, %44 ], [ false, %44 ]
  %77 = phi i1 [ false, %73 ], [ false, %72 ], [ true, %71 ], [ false, %70 ], [ false, %44 ], [ false, %44 ]
  %78 = phi i32 [ 0, %73 ], [ 4, %72 ], [ 3, %71 ], [ 2, %70 ], [ 1, %44 ], [ 1, %44 ]
  %79 = and i32 %1, -2
  %80 = icmp eq i32 %79, 12
  br i1 %80, label %84, label %81

81:                                               ; preds = %74
  switch i32 %1, label %82 [
    i32 15, label %84
    i32 14, label %84
    i32 9, label %84
    i32 8, label %84
    i32 7, label %83
  ]

82:                                               ; preds = %81
  br label %84

83:                                               ; preds = %81
  br label %84

84:                                               ; preds = %74, %81, %81, %81, %81, %82, %83
  %85 = phi i8 [ 32, %82 ], [ 8, %74 ], [ 16, %81 ], [ 16, %81 ], [ 16, %81 ], [ 16, %81 ], [ 64, %83 ]
  switch i32 %1, label %86 [
    i32 15, label %89
    i32 14, label %89
    i32 11, label %89
    i32 10, label %89
  ]

86:                                               ; preds = %84
  %87 = icmp eq i32 %79, 16
  %88 = select i1 %87, i8 4, i8 1
  br label %89

89:                                               ; preds = %84, %84, %84, %84, %86
  %90 = phi i8 [ %88, %86 ], [ 2, %84 ], [ 2, %84 ], [ 2, %84 ], [ 2, %84 ]
  %91 = udiv i8 %85, %90
  %92 = zext nneg i8 %91 to i32
  %93 = icmp eq i8 %91, 32
  %94 = shl nsw i32 -1, %92
  %95 = xor i32 %94, -1
  %96 = select i1 %93, i32 -1, i32 %95
  %97 = icmp eq i32 %79, 16
  %98 = select i1 %97, i32 4, i32 1
  %99 = add nsw i32 %78, -3
  %100 = icmp ult i32 %99, 2
  %101 = select i1 %76, i32 8, i32 5
  %102 = select i1 %76, i32 7, i32 10
  %103 = shl nsw i32 -1, %101
  %104 = xor i32 %103, -1
  %105 = shl nuw nsw i32 %104, %102
  %106 = add nsw i32 %101, -1
  %107 = shl nsw i32 -1, %106
  %108 = xor i32 %107, -1
  %109 = add nuw nsw i32 %107, 2
  %110 = zext nneg i32 %102 to i64
  %111 = shl nuw nsw i64 2, %110
  %112 = zext nneg i32 %105 to i64
  %113 = select i1 %77, i64 126, i64 123
  %114 = select i1 %77, i32 -8, i32 -16
  %115 = xor i32 %114, -1
  %116 = select i1 %77, i32 3, i32 2
  %117 = or disjoint i32 %114, 2
  %118 = zext nneg i32 %116 to i64
  %119 = shl nuw nsw i64 2, %118
  br label %120

120:                                              ; preds = %819, %89
  %121 = phi i32 [ 0, %89 ], [ %825, %819 ]
  %122 = phi i64 [ 0, %89 ], [ %824, %819 ]
  switch i32 %1, label %123 [
    i32 15, label %124
    i32 14, label %124
    i32 11, label %124
    i32 10, label %124
  ]

123:                                              ; preds = %120
  br label %124

124:                                              ; preds = %120, %120, %120, %120, %123
  %125 = phi i32 [ %98, %123 ], [ 2, %120 ], [ 2, %120 ], [ 2, %120 ], [ 2, %120 ]
  %126 = icmp ult i32 %121, %125
  br i1 %126, label %127, label %826

127:                                              ; preds = %124
  %128 = mul nuw nsw i32 %121, %92
  %129 = zext nneg i32 %128 to i64
  %130 = lshr i64 %2, %129
  %131 = trunc i64 %130 to i32
  %132 = and i32 %96, %131
  switch i32 %78, label %303 [
    i32 0, label %133
    i32 1, label %176
    i32 2, label %219
    i32 3, label %263
  ]

133:                                              ; preds = %127
  %134 = lshr i32 %132, 23
  %135 = and i32 %132, 8388607
  %136 = trunc i32 %134 to i8
  switch i8 %136, label %140 [
    i8 -1, label %137
    i8 0, label %144
  ]

137:                                              ; preds = %133
  %138 = icmp ne i32 %135, 0
  %139 = icmp eq i32 %135, 0
  br label %144

140:                                              ; preds = %133
  %141 = and i32 %134, 255
  %142 = or disjoint i32 %135, 8388608
  %143 = add nuw nsw i32 %141, 873
  br label %144

144:                                              ; preds = %140, %137, %133
  %145 = phi i32 [ 1023, %137 ], [ %143, %140 ], [ 874, %133 ]
  %146 = phi i32 [ 0, %137 ], [ %142, %140 ], [ %135, %133 ]
  %147 = phi i1 [ %139, %137 ], [ false, %140 ], [ false, %133 ]
  %148 = phi i1 [ %138, %137 ], [ false, %140 ], [ false, %133 ]
  %149 = zext nneg i32 %146 to i64
  %150 = or i1 %147, %148
  %151 = select i1 %148, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %150, label %170, label %152

152:                                              ; preds = %144
  %153 = icmp eq i32 %146, 0
  br i1 %153, label %170, label %154

154:                                              ; preds = %152, %154
  %155 = phi i32 [ %158, %154 ], [ 0, %152 ]
  %156 = phi i64 [ %157, %154 ], [ %149, %152 ]
  %157 = lshr i64 %156, 1
  %158 = add nuw nsw i32 %155, 1
  %159 = icmp ult i64 %156, 2
  br i1 %159, label %160, label %154, !llvm.loop !20003

160:                                              ; preds = %154
  %161 = add i32 %155, %145
  %162 = zext i32 %161 to i64
  %163 = shl i64 %162, 52
  %164 = sub nsw i32 52, %155
  %165 = zext nneg i32 %164 to i64
  %166 = shl i64 %149, %165
  %167 = and i64 %166, 4503599627370495
  %168 = or disjoint i64 %167, %163
  %169 = bitcast i64 %168 to double
  br label %170

170:                                              ; preds = %160, %152, %144
  %171 = phi double [ %169, %160 ], [ %151, %144 ], [ 0.000000e+00, %152 ]
  %172 = tail call double @llvm.fabs.f64(double %171)
  %173 = fneg double %172
  %174 = icmp slt i32 %132, 0
  %175 = select i1 %174, double %173, double %171
  br label %348

176:                                              ; preds = %127
  %177 = lshr i32 %132, 10
  %178 = and i32 %177, 31
  %179 = and i32 %132, 1023
  switch i32 %178, label %183 [
    i32 31, label %180
    i32 0, label %186
  ]

180:                                              ; preds = %176
  %181 = icmp ne i32 %179, 0
  %182 = icmp eq i32 %179, 0
  br label %186

183:                                              ; preds = %176
  %184 = or disjoint i32 %179, 1024
  %185 = add nuw nsw i32 %178, 998
  br label %186

186:                                              ; preds = %183, %180, %176
  %187 = phi i32 [ 1023, %180 ], [ %185, %183 ], [ 999, %176 ]
  %188 = phi i32 [ 0, %180 ], [ %184, %183 ], [ %179, %176 ]
  %189 = phi i1 [ %182, %180 ], [ false, %183 ], [ false, %176 ]
  %190 = phi i1 [ %181, %180 ], [ false, %183 ], [ false, %176 ]
  %191 = zext nneg i32 %188 to i64
  %192 = or i1 %189, %190
  %193 = select i1 %190, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %192, label %212, label %194

194:                                              ; preds = %186
  %195 = icmp eq i32 %188, 0
  br i1 %195, label %212, label %196

196:                                              ; preds = %194, %196
  %197 = phi i32 [ %200, %196 ], [ 0, %194 ]
  %198 = phi i64 [ %199, %196 ], [ %191, %194 ]
  %199 = lshr i64 %198, 1
  %200 = add nuw nsw i32 %197, 1
  %201 = icmp ult i64 %198, 2
  br i1 %201, label %202, label %196, !llvm.loop !20003

202:                                              ; preds = %196
  %203 = add i32 %197, %187
  %204 = zext i32 %203 to i64
  %205 = shl i64 %204, 52
  %206 = sub nsw i32 52, %197
  %207 = zext nneg i32 %206 to i64
  %208 = shl i64 %191, %207
  %209 = and i64 %208, 4503599627370495
  %210 = or disjoint i64 %209, %205
  %211 = bitcast i64 %210 to double
  br label %212

212:                                              ; preds = %202, %194, %186
  %213 = phi double [ %211, %202 ], [ %193, %186 ], [ 0.000000e+00, %194 ]
  %214 = and i32 %132, 32768
  %215 = icmp eq i32 %214, 0
  %216 = tail call double @llvm.fabs.f64(double %213)
  %217 = fneg double %216
  %218 = select i1 %215, double %213, double %217
  br label %348

219:                                              ; preds = %127
  %220 = shl i32 %132, 16
  %221 = lshr i32 %220, 23
  %222 = and i32 %220, 8323072
  %223 = trunc i32 %221 to i8
  switch i8 %223, label %227 [
    i8 -1, label %224
    i8 0, label %231
  ]

224:                                              ; preds = %219
  %225 = icmp ne i32 %222, 0
  %226 = icmp eq i32 %222, 0
  br label %231

227:                                              ; preds = %219
  %228 = and i32 %221, 255
  %229 = or disjoint i32 %222, 8388608
  %230 = add nuw nsw i32 %228, 873
  br label %231

231:                                              ; preds = %227, %224, %219
  %232 = phi i32 [ 1023, %224 ], [ %230, %227 ], [ 874, %219 ]
  %233 = phi i32 [ 0, %224 ], [ %229, %227 ], [ %222, %219 ]
  %234 = phi i1 [ %226, %224 ], [ false, %227 ], [ false, %219 ]
  %235 = phi i1 [ %225, %224 ], [ false, %227 ], [ false, %219 ]
  %236 = zext nneg i32 %233 to i64
  %237 = or i1 %234, %235
  %238 = select i1 %235, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %237, label %257, label %239

239:                                              ; preds = %231
  %240 = icmp eq i32 %233, 0
  br i1 %240, label %257, label %241

241:                                              ; preds = %239, %241
  %242 = phi i32 [ %245, %241 ], [ 0, %239 ]
  %243 = phi i64 [ %244, %241 ], [ %236, %239 ]
  %244 = lshr i64 %243, 1
  %245 = add nuw nsw i32 %242, 1
  %246 = icmp ult i64 %243, 2
  br i1 %246, label %247, label %241, !llvm.loop !20003

247:                                              ; preds = %241
  %248 = add i32 %242, %232
  %249 = zext i32 %248 to i64
  %250 = shl i64 %249, 52
  %251 = sub nsw i32 52, %242
  %252 = zext nneg i32 %251 to i64
  %253 = shl i64 %236, %252
  %254 = and i64 %253, 4503599627370495
  %255 = or disjoint i64 %254, %250
  %256 = bitcast i64 %255 to double
  br label %257

257:                                              ; preds = %247, %239, %231
  %258 = phi double [ %256, %247 ], [ %238, %231 ], [ 0.000000e+00, %239 ]
  %259 = tail call double @llvm.fabs.f64(double %258)
  %260 = fneg double %259
  %261 = icmp slt i32 %220, 0
  %262 = select i1 %261, double %260, double %258
  br label %348

263:                                              ; preds = %127
  %264 = and i32 %132, 127
  %265 = icmp eq i32 %264, 127
  br i1 %265, label %275, label %266

266:                                              ; preds = %263
  %267 = lshr i32 %264, 3
  %268 = and i32 %132, 7
  %269 = icmp ult i32 %264, 8
  %270 = or disjoint i32 %268, 8
  %271 = add nuw nsw i32 %267, 1013
  %272 = select i1 %269, i32 %268, i32 %270
  %273 = zext nneg i32 %272 to i64
  %274 = select i1 %269, i32 1014, i32 %271
  br label %275

275:                                              ; preds = %266, %263
  %276 = phi i32 [ 1023, %263 ], [ %274, %266 ]
  %277 = phi i64 [ 0, %263 ], [ %273, %266 ]
  br i1 %265, label %296, label %278

278:                                              ; preds = %275
  %279 = icmp eq i64 %277, 0
  br i1 %279, label %296, label %280

280:                                              ; preds = %278, %280
  %281 = phi i32 [ %284, %280 ], [ 0, %278 ]
  %282 = phi i64 [ %283, %280 ], [ %277, %278 ]
  %283 = lshr i64 %282, 1
  %284 = add nuw nsw i32 %281, 1
  %285 = icmp ult i64 %282, 2
  br i1 %285, label %286, label %280, !llvm.loop !20003

286:                                              ; preds = %280
  %287 = add i32 %281, %276
  %288 = zext i32 %287 to i64
  %289 = shl i64 %288, 52
  %290 = sub nsw i32 52, %281
  %291 = zext nneg i32 %290 to i64
  %292 = shl i64 %277, %291
  %293 = and i64 %292, 4503599627370495
  %294 = or disjoint i64 %293, %289
  %295 = bitcast i64 %294 to double
  br label %296

296:                                              ; preds = %286, %278, %275
  %297 = phi double [ %295, %286 ], [ 0x7FF8000000000000, %275 ], [ 0.000000e+00, %278 ]
  %298 = and i32 %132, 128
  %299 = icmp eq i32 %298, 0
  %300 = tail call double @llvm.fabs.f64(double %297)
  %301 = fneg double %300
  %302 = select i1 %299, double %297, double %301
  br label %348

303:                                              ; preds = %127
  %304 = and i32 %132, 127
  %305 = icmp ugt i32 %304, 124
  br i1 %305, label %317, label %306

306:                                              ; preds = %303
  %307 = icmp eq i32 %304, 124
  br i1 %307, label %317, label %308

308:                                              ; preds = %306
  %309 = lshr i32 %304, 2
  %310 = and i32 %132, 3
  %311 = icmp ult i32 %304, 4
  %312 = or disjoint i32 %310, 4
  %313 = add nuw nsw i32 %309, 1006
  %314 = select i1 %311, i32 %310, i32 %312
  %315 = zext nneg i32 %314 to i64
  %316 = select i1 %311, i32 1007, i32 %313
  br label %317

317:                                              ; preds = %308, %306, %303
  %318 = phi double [ 0x7FF8000000000000, %303 ], [ 0x7FF0000000000000, %306 ], [ 0x7FF0000000000000, %308 ]
  %319 = phi i32 [ 1023, %303 ], [ 1023, %306 ], [ %316, %308 ]
  %320 = phi i64 [ 0, %303 ], [ 0, %306 ], [ %315, %308 ]
  %321 = phi i1 [ false, %303 ], [ true, %306 ], [ false, %308 ]
  %322 = or i1 %305, %321
  br i1 %322, label %341, label %323

323:                                              ; preds = %317
  %324 = icmp eq i64 %320, 0
  br i1 %324, label %341, label %325

325:                                              ; preds = %323, %325
  %326 = phi i32 [ %329, %325 ], [ 0, %323 ]
  %327 = phi i64 [ %328, %325 ], [ %320, %323 ]
  %328 = lshr i64 %327, 1
  %329 = add nuw nsw i32 %326, 1
  %330 = icmp ult i64 %327, 2
  br i1 %330, label %331, label %325, !llvm.loop !20003

331:                                              ; preds = %325
  %332 = add i32 %326, %319
  %333 = zext i32 %332 to i64
  %334 = shl i64 %333, 52
  %335 = sub nsw i32 52, %326
  %336 = zext nneg i32 %335 to i64
  %337 = shl i64 %320, %336
  %338 = and i64 %337, 4503599627370495
  %339 = or disjoint i64 %338, %334
  %340 = bitcast i64 %339 to double
  br label %341

341:                                              ; preds = %331, %323, %317
  %342 = phi double [ %340, %331 ], [ %318, %317 ], [ 0.000000e+00, %323 ]
  %343 = and i32 %132, 128
  %344 = icmp eq i32 %343, 0
  %345 = tail call double @llvm.fabs.f64(double %342)
  %346 = fneg double %345
  %347 = select i1 %344, double %342, double %346
  br label %348

348:                                              ; preds = %170, %212, %257, %296, %341
  %349 = phi double [ %347, %341 ], [ %302, %296 ], [ %262, %257 ], [ %218, %212 ], [ %175, %170 ]
  %350 = lshr i64 %3, %129
  %351 = trunc i64 %350 to i32
  %352 = and i32 %96, %351
  switch i32 %78, label %523 [
    i32 0, label %353
    i32 1, label %396
    i32 2, label %439
    i32 3, label %483
  ]

353:                                              ; preds = %348
  %354 = lshr i32 %352, 23
  %355 = and i32 %352, 8388607
  %356 = trunc i32 %354 to i8
  switch i8 %356, label %360 [
    i8 -1, label %357
    i8 0, label %364
  ]

357:                                              ; preds = %353
  %358 = icmp ne i32 %355, 0
  %359 = icmp eq i32 %355, 0
  br label %364

360:                                              ; preds = %353
  %361 = and i32 %354, 255
  %362 = or disjoint i32 %355, 8388608
  %363 = add nuw nsw i32 %361, 873
  br label %364

364:                                              ; preds = %360, %357, %353
  %365 = phi i32 [ 1023, %357 ], [ %363, %360 ], [ 874, %353 ]
  %366 = phi i32 [ 0, %357 ], [ %362, %360 ], [ %355, %353 ]
  %367 = phi i1 [ %359, %357 ], [ false, %360 ], [ false, %353 ]
  %368 = phi i1 [ %358, %357 ], [ false, %360 ], [ false, %353 ]
  %369 = zext nneg i32 %366 to i64
  %370 = or i1 %367, %368
  %371 = select i1 %368, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %370, label %390, label %372

372:                                              ; preds = %364
  %373 = icmp eq i32 %366, 0
  br i1 %373, label %390, label %374

374:                                              ; preds = %372, %374
  %375 = phi i32 [ %378, %374 ], [ 0, %372 ]
  %376 = phi i64 [ %377, %374 ], [ %369, %372 ]
  %377 = lshr i64 %376, 1
  %378 = add nuw nsw i32 %375, 1
  %379 = icmp ult i64 %376, 2
  br i1 %379, label %380, label %374, !llvm.loop !20003

380:                                              ; preds = %374
  %381 = add i32 %375, %365
  %382 = zext i32 %381 to i64
  %383 = shl i64 %382, 52
  %384 = sub nsw i32 52, %375
  %385 = zext nneg i32 %384 to i64
  %386 = shl i64 %369, %385
  %387 = and i64 %386, 4503599627370495
  %388 = or disjoint i64 %387, %383
  %389 = bitcast i64 %388 to double
  br label %390

390:                                              ; preds = %380, %372, %364
  %391 = phi double [ %389, %380 ], [ %371, %364 ], [ 0.000000e+00, %372 ]
  %392 = tail call double @llvm.fabs.f64(double %391)
  %393 = fneg double %392
  %394 = icmp slt i32 %352, 0
  %395 = select i1 %394, double %393, double %391
  br label %568

396:                                              ; preds = %348
  %397 = lshr i32 %352, 10
  %398 = and i32 %397, 31
  %399 = and i32 %352, 1023
  switch i32 %398, label %403 [
    i32 31, label %400
    i32 0, label %406
  ]

400:                                              ; preds = %396
  %401 = icmp ne i32 %399, 0
  %402 = icmp eq i32 %399, 0
  br label %406

403:                                              ; preds = %396
  %404 = or disjoint i32 %399, 1024
  %405 = add nuw nsw i32 %398, 998
  br label %406

406:                                              ; preds = %403, %400, %396
  %407 = phi i32 [ 1023, %400 ], [ %405, %403 ], [ 999, %396 ]
  %408 = phi i32 [ 0, %400 ], [ %404, %403 ], [ %399, %396 ]
  %409 = phi i1 [ %402, %400 ], [ false, %403 ], [ false, %396 ]
  %410 = phi i1 [ %401, %400 ], [ false, %403 ], [ false, %396 ]
  %411 = zext nneg i32 %408 to i64
  %412 = or i1 %409, %410
  %413 = select i1 %410, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %412, label %432, label %414

414:                                              ; preds = %406
  %415 = icmp eq i32 %408, 0
  br i1 %415, label %432, label %416

416:                                              ; preds = %414, %416
  %417 = phi i32 [ %420, %416 ], [ 0, %414 ]
  %418 = phi i64 [ %419, %416 ], [ %411, %414 ]
  %419 = lshr i64 %418, 1
  %420 = add nuw nsw i32 %417, 1
  %421 = icmp ult i64 %418, 2
  br i1 %421, label %422, label %416, !llvm.loop !20003

422:                                              ; preds = %416
  %423 = add i32 %417, %407
  %424 = zext i32 %423 to i64
  %425 = shl i64 %424, 52
  %426 = sub nsw i32 52, %417
  %427 = zext nneg i32 %426 to i64
  %428 = shl i64 %411, %427
  %429 = and i64 %428, 4503599627370495
  %430 = or disjoint i64 %429, %425
  %431 = bitcast i64 %430 to double
  br label %432

432:                                              ; preds = %422, %414, %406
  %433 = phi double [ %431, %422 ], [ %413, %406 ], [ 0.000000e+00, %414 ]
  %434 = and i32 %352, 32768
  %435 = icmp eq i32 %434, 0
  %436 = tail call double @llvm.fabs.f64(double %433)
  %437 = fneg double %436
  %438 = select i1 %435, double %433, double %437
  br label %568

439:                                              ; preds = %348
  %440 = shl i32 %352, 16
  %441 = lshr i32 %440, 23
  %442 = and i32 %440, 8323072
  %443 = trunc i32 %441 to i8
  switch i8 %443, label %447 [
    i8 -1, label %444
    i8 0, label %451
  ]

444:                                              ; preds = %439
  %445 = icmp ne i32 %442, 0
  %446 = icmp eq i32 %442, 0
  br label %451

447:                                              ; preds = %439
  %448 = and i32 %441, 255
  %449 = or disjoint i32 %442, 8388608
  %450 = add nuw nsw i32 %448, 873
  br label %451

451:                                              ; preds = %447, %444, %439
  %452 = phi i32 [ 1023, %444 ], [ %450, %447 ], [ 874, %439 ]
  %453 = phi i32 [ 0, %444 ], [ %449, %447 ], [ %442, %439 ]
  %454 = phi i1 [ %446, %444 ], [ false, %447 ], [ false, %439 ]
  %455 = phi i1 [ %445, %444 ], [ false, %447 ], [ false, %439 ]
  %456 = zext nneg i32 %453 to i64
  %457 = or i1 %454, %455
  %458 = select i1 %455, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %457, label %477, label %459

459:                                              ; preds = %451
  %460 = icmp eq i32 %453, 0
  br i1 %460, label %477, label %461

461:                                              ; preds = %459, %461
  %462 = phi i32 [ %465, %461 ], [ 0, %459 ]
  %463 = phi i64 [ %464, %461 ], [ %456, %459 ]
  %464 = lshr i64 %463, 1
  %465 = add nuw nsw i32 %462, 1
  %466 = icmp ult i64 %463, 2
  br i1 %466, label %467, label %461, !llvm.loop !20003

467:                                              ; preds = %461
  %468 = add i32 %462, %452
  %469 = zext i32 %468 to i64
  %470 = shl i64 %469, 52
  %471 = sub nsw i32 52, %462
  %472 = zext nneg i32 %471 to i64
  %473 = shl i64 %456, %472
  %474 = and i64 %473, 4503599627370495
  %475 = or disjoint i64 %474, %470
  %476 = bitcast i64 %475 to double
  br label %477

477:                                              ; preds = %467, %459, %451
  %478 = phi double [ %476, %467 ], [ %458, %451 ], [ 0.000000e+00, %459 ]
  %479 = tail call double @llvm.fabs.f64(double %478)
  %480 = fneg double %479
  %481 = icmp slt i32 %440, 0
  %482 = select i1 %481, double %480, double %478
  br label %568

483:                                              ; preds = %348
  %484 = and i32 %352, 127
  %485 = icmp eq i32 %484, 127
  br i1 %485, label %495, label %486

486:                                              ; preds = %483
  %487 = lshr i32 %484, 3
  %488 = and i32 %352, 7
  %489 = icmp ult i32 %484, 8
  %490 = or disjoint i32 %488, 8
  %491 = add nuw nsw i32 %487, 1013
  %492 = select i1 %489, i32 %488, i32 %490
  %493 = zext nneg i32 %492 to i64
  %494 = select i1 %489, i32 1014, i32 %491
  br label %495

495:                                              ; preds = %486, %483
  %496 = phi i32 [ 1023, %483 ], [ %494, %486 ]
  %497 = phi i64 [ 0, %483 ], [ %493, %486 ]
  br i1 %485, label %516, label %498

498:                                              ; preds = %495
  %499 = icmp eq i64 %497, 0
  br i1 %499, label %516, label %500

500:                                              ; preds = %498, %500
  %501 = phi i32 [ %504, %500 ], [ 0, %498 ]
  %502 = phi i64 [ %503, %500 ], [ %497, %498 ]
  %503 = lshr i64 %502, 1
  %504 = add nuw nsw i32 %501, 1
  %505 = icmp ult i64 %502, 2
  br i1 %505, label %506, label %500, !llvm.loop !20003

506:                                              ; preds = %500
  %507 = add i32 %501, %496
  %508 = zext i32 %507 to i64
  %509 = shl i64 %508, 52
  %510 = sub nsw i32 52, %501
  %511 = zext nneg i32 %510 to i64
  %512 = shl i64 %497, %511
  %513 = and i64 %512, 4503599627370495
  %514 = or disjoint i64 %513, %509
  %515 = bitcast i64 %514 to double
  br label %516

516:                                              ; preds = %506, %498, %495
  %517 = phi double [ %515, %506 ], [ 0x7FF8000000000000, %495 ], [ 0.000000e+00, %498 ]
  %518 = and i32 %352, 128
  %519 = icmp eq i32 %518, 0
  %520 = tail call double @llvm.fabs.f64(double %517)
  %521 = fneg double %520
  %522 = select i1 %519, double %517, double %521
  br label %568

523:                                              ; preds = %348
  %524 = and i32 %352, 127
  %525 = icmp ugt i32 %524, 124
  br i1 %525, label %537, label %526

526:                                              ; preds = %523
  %527 = icmp eq i32 %524, 124
  br i1 %527, label %537, label %528

528:                                              ; preds = %526
  %529 = lshr i32 %524, 2
  %530 = and i32 %352, 3
  %531 = icmp ult i32 %524, 4
  %532 = or disjoint i32 %530, 4
  %533 = add nuw nsw i32 %529, 1006
  %534 = select i1 %531, i32 %530, i32 %532
  %535 = zext nneg i32 %534 to i64
  %536 = select i1 %531, i32 1007, i32 %533
  br label %537

537:                                              ; preds = %528, %526, %523
  %538 = phi double [ 0x7FF8000000000000, %523 ], [ 0x7FF0000000000000, %526 ], [ 0x7FF0000000000000, %528 ]
  %539 = phi i32 [ 1023, %523 ], [ 1023, %526 ], [ %536, %528 ]
  %540 = phi i64 [ 0, %523 ], [ 0, %526 ], [ %535, %528 ]
  %541 = phi i1 [ false, %523 ], [ true, %526 ], [ false, %528 ]
  %542 = or i1 %525, %541
  br i1 %542, label %561, label %543

543:                                              ; preds = %537
  %544 = icmp eq i64 %540, 0
  br i1 %544, label %561, label %545

545:                                              ; preds = %543, %545
  %546 = phi i32 [ %549, %545 ], [ 0, %543 ]
  %547 = phi i64 [ %548, %545 ], [ %540, %543 ]
  %548 = lshr i64 %547, 1
  %549 = add nuw nsw i32 %546, 1
  %550 = icmp ult i64 %547, 2
  br i1 %550, label %551, label %545, !llvm.loop !20003

551:                                              ; preds = %545
  %552 = add i32 %546, %539
  %553 = zext i32 %552 to i64
  %554 = shl i64 %553, 52
  %555 = sub nsw i32 52, %546
  %556 = zext nneg i32 %555 to i64
  %557 = shl i64 %540, %556
  %558 = and i64 %557, 4503599627370495
  %559 = or disjoint i64 %558, %554
  %560 = bitcast i64 %559 to double
  br label %561

561:                                              ; preds = %551, %543, %537
  %562 = phi double [ %560, %551 ], [ %538, %537 ], [ 0.000000e+00, %543 ]
  %563 = and i32 %352, 128
  %564 = icmp eq i32 %563, 0
  %565 = tail call double @llvm.fabs.f64(double %562)
  %566 = fneg double %565
  %567 = select i1 %564, double %562, double %566
  br label %568

568:                                              ; preds = %390, %432, %477, %516, %561
  %569 = phi double [ %567, %561 ], [ %522, %516 ], [ %482, %477 ], [ %438, %432 ], [ %395, %390 ]
  %570 = tail call fastcc double @__zluda_agx_cvt_mm_comb(i32 noundef %0, i32 noundef %78, double noundef %349, double noundef %569)
  %571 = bitcast double %570 to i64
  %572 = lshr i64 %571, 52
  %573 = trunc i64 %572 to i32
  %574 = and i32 %573, 2047
  %575 = and i64 %571, 4503599627370495
  switch i32 %574, label %581 [
    i32 2047, label %576
    i32 0, label %584
  ]

576:                                              ; preds = %568
  %577 = icmp ne i64 %575, 0
  %578 = zext i1 %577 to i32
  %579 = icmp eq i64 %575, 0
  %580 = zext i1 %579 to i32
  br label %584

581:                                              ; preds = %568
  %582 = or disjoint i64 %575, 4503599627370496
  %583 = add nsw i32 %574, -1075
  br label %584

584:                                              ; preds = %581, %576, %568
  %585 = phi i32 [ 0, %576 ], [ %583, %581 ], [ -1074, %568 ]
  %586 = phi i64 [ 0, %576 ], [ %582, %581 ], [ %575, %568 ]
  %587 = phi i32 [ %580, %576 ], [ 0, %581 ], [ %574, %568 ]
  %588 = phi i32 [ %578, %576 ], [ 0, %581 ], [ %574, %568 ]
  %589 = icmp eq i32 %588, 0
  br i1 %100, label %590, label %667

590:                                              ; preds = %584
  br i1 %589, label %591, label %819

591:                                              ; preds = %590
  %592 = lshr i64 %571, 56
  %593 = trunc i64 %592 to i32
  %594 = and i32 %593, 128
  %595 = icmp eq i32 %587, 0
  br i1 %595, label %596, label %663

596:                                              ; preds = %591
  %597 = icmp eq i64 %586, 0
  br i1 %597, label %660, label %598

598:                                              ; preds = %596, %598
  %599 = phi i32 [ %602, %598 ], [ 0, %596 ]
  %600 = phi i64 [ %601, %598 ], [ %586, %596 ]
  %601 = lshr i64 %600, 1
  %602 = add nuw nsw i32 %599, 1
  %603 = icmp ult i64 %600, 2
  br i1 %603, label %604, label %598, !llvm.loop !20003

604:                                              ; preds = %598
  %605 = add nsw i32 %602, %585
  %606 = add i32 %599, %585
  %607 = icmp sgt i32 %605, %117
  %608 = select i1 %607, i32 %606, i32 %117
  %609 = icmp sgt i32 %608, 200
  br i1 %609, label %660, label %610

610:                                              ; preds = %604
  %611 = sub i32 %116, %608
  %612 = add i32 %611, %585
  %613 = icmp sgt i32 %612, -1
  br i1 %613, label %614, label %619

614:                                              ; preds = %610
  %615 = icmp ugt i32 %612, 40
  %616 = zext nneg i32 %612 to i64
  %617 = shl i64 %586, %616
  %618 = select i1 %615, i64 4611686018427387904, i64 %617
  br label %644

619:                                              ; preds = %610
  %620 = sub nsw i32 0, %612
  %621 = icmp ult i32 %612, -63
  %622 = zext nneg i32 %620 to i64
  %623 = lshr i64 %586, %622
  %624 = select i1 %621, i64 0, i64 %623
  %625 = shl nsw i64 -1, %622
  %626 = xor i64 %625, -1
  %627 = select i1 %621, i64 -1, i64 %626
  %628 = and i64 %627, %586
  %629 = icmp eq i64 %628, 0
  %630 = or i1 %621, %629
  %631 = select i1 %629, i64 %624, i64 0
  br i1 %630, label %644, label %632

632:                                              ; preds = %619
  %633 = xor i32 %612, -1
  %634 = zext nneg i32 %633 to i64
  %635 = shl nuw nsw i64 1, %634
  %636 = icmp ugt i64 %628, %635
  %637 = icmp eq i64 %628, %635
  %638 = and i64 %623, 1
  %639 = icmp ne i64 %638, 0
  %640 = select i1 %637, i1 %639, i1 false
  %641 = select i1 %636, i1 true, i1 %640
  %642 = zext i1 %641 to i64
  %643 = add nuw nsw i64 %623, %642
  br label %644

644:                                              ; preds = %632, %619, %614
  %645 = phi i64 [ %618, %614 ], [ %643, %632 ], [ %631, %619 ]
  %646 = icmp ult i64 %645, %119
  br i1 %646, label %651, label %647

647:                                              ; preds = %644
  %648 = sub i32 %608, %114
  %649 = sext i32 %648 to i64
  %650 = shl nsw i64 %649, %118
  br label %660

651:                                              ; preds = %644
  %652 = lshr i64 %645, %118
  %653 = icmp eq i64 %652, 0
  %654 = add nsw i32 %608, %115
  %655 = sext i32 %654 to i64
  %656 = add nsw i64 %655, -1
  %657 = shl nsw i64 %656, %118
  %658 = select i1 %653, i64 0, i64 %657
  %659 = add i64 %658, %645
  br label %660

660:                                              ; preds = %651, %647, %604, %596
  %661 = phi i64 [ 0, %596 ], [ 1099511627776, %604 ], [ %650, %647 ], [ %659, %651 ]
  %662 = tail call i64 @llvm.smin.i64(i64 %661, i64 %113)
  br label %663

663:                                              ; preds = %660, %591
  %664 = phi i64 [ %113, %591 ], [ %662, %660 ]
  %665 = trunc i64 %664 to i32
  %666 = or i32 %594, %665
  br label %819

667:                                              ; preds = %584
  br i1 %75, label %668, label %741

668:                                              ; preds = %667
  br i1 %589, label %669, label %819

669:                                              ; preds = %668
  %670 = lshr i64 %571, 32
  %671 = trunc i64 %670 to i32
  %672 = and i32 %671, -2147483648
  %673 = icmp eq i32 %587, 0
  br i1 %673, label %674, label %736

674:                                              ; preds = %669
  %675 = icmp eq i64 %586, 0
  br i1 %675, label %736, label %676

676:                                              ; preds = %674, %676
  %677 = phi i32 [ %680, %676 ], [ 0, %674 ]
  %678 = phi i64 [ %679, %676 ], [ %586, %674 ]
  %679 = lshr i64 %678, 1
  %680 = add nuw nsw i32 %677, 1
  %681 = icmp ult i64 %678, 2
  br i1 %681, label %682, label %676, !llvm.loop !20003

682:                                              ; preds = %676
  %683 = add nsw i32 %680, %585
  %684 = add i32 %677, %585
  %685 = icmp sgt i32 %683, -126
  %686 = select i1 %685, i32 %684, i32 -126
  %687 = icmp sgt i32 %686, 200
  br i1 %687, label %736, label %688

688:                                              ; preds = %682
  %689 = sub i32 %585, %686
  %690 = add i32 %689, 23
  %691 = icmp sgt i32 %690, -1
  br i1 %691, label %692, label %697

692:                                              ; preds = %688
  %693 = icmp ugt i32 %690, 40
  %694 = zext nneg i32 %690 to i64
  %695 = shl i64 %586, %694
  %696 = select i1 %693, i64 4611686018427387904, i64 %695
  br label %722

697:                                              ; preds = %688
  %698 = sub nuw i32 -23, %689
  %699 = icmp ult i32 %690, -63
  %700 = zext nneg i32 %698 to i64
  %701 = lshr i64 %586, %700
  %702 = select i1 %699, i64 0, i64 %701
  %703 = shl nsw i64 -1, %700
  %704 = xor i64 %703, -1
  %705 = select i1 %699, i64 -1, i64 %704
  %706 = and i64 %705, %586
  %707 = icmp eq i64 %706, 0
  %708 = or i1 %699, %707
  %709 = select i1 %707, i64 %702, i64 0
  br i1 %708, label %722, label %710

710:                                              ; preds = %697
  %711 = sub nuw nsw i32 -24, %689
  %712 = zext nneg i32 %711 to i64
  %713 = shl nuw nsw i64 1, %712
  %714 = icmp ugt i64 %706, %713
  %715 = icmp eq i64 %706, %713
  %716 = and i64 %701, 1
  %717 = icmp ne i64 %716, 0
  %718 = select i1 %715, i1 %717, i1 false
  %719 = select i1 %714, i1 true, i1 %718
  %720 = zext i1 %719 to i64
  %721 = add nuw nsw i64 %701, %720
  br label %722

722:                                              ; preds = %710, %697, %692
  %723 = phi i64 [ %696, %692 ], [ %721, %710 ], [ %709, %697 ]
  %724 = icmp ult i64 %723, 16777216
  br i1 %724, label %729, label %725

725:                                              ; preds = %722
  %726 = add nsw i32 %686, 128
  %727 = sext i32 %726 to i64
  %728 = shl nsw i64 %727, 23
  br label %736

729:                                              ; preds = %722
  %730 = icmp ult i64 %723, 8388608
  %731 = sext i32 %686 to i64
  %732 = shl nsw i64 %731, 23
  %733 = add nsw i64 %732, 1056964608
  %734 = select i1 %730, i64 0, i64 %733
  %735 = add i64 %734, %723
  br label %736

736:                                              ; preds = %729, %725, %682, %674, %669
  %737 = phi i64 [ 2139095040, %669 ], [ 0, %674 ], [ 1099511627776, %682 ], [ %728, %725 ], [ %735, %729 ]
  %738 = tail call i64 @llvm.smin.i64(i64 %737, i64 2139095040)
  %739 = trunc i64 %738 to i32
  %740 = or i32 %672, %739
  br label %819

741:                                              ; preds = %667
  br i1 %589, label %742, label %819

742:                                              ; preds = %741
  %743 = lshr i64 %571, 48
  %744 = trunc i64 %743 to i32
  %745 = and i32 %744, 32768
  %746 = icmp eq i32 %587, 0
  br i1 %746, label %749, label %747

747:                                              ; preds = %742
  %748 = or i32 %745, %105
  br label %819

749:                                              ; preds = %742
  %750 = icmp eq i64 %586, 0
  br i1 %750, label %819, label %751

751:                                              ; preds = %749, %751
  %752 = phi i32 [ %755, %751 ], [ 0, %749 ]
  %753 = phi i64 [ %754, %751 ], [ %586, %749 ]
  %754 = lshr i64 %753, 1
  %755 = add nuw nsw i32 %752, 1
  %756 = icmp ult i64 %753, 2
  br i1 %756, label %757, label %751, !llvm.loop !20003

757:                                              ; preds = %751
  %758 = add nsw i32 %755, %585
  %759 = add i32 %752, %585
  %760 = icmp sgt i32 %758, %109
  %761 = select i1 %760, i32 %759, i32 %109
  %762 = icmp sgt i32 %761, 200
  br i1 %762, label %813, label %763

763:                                              ; preds = %757
  %764 = sub i32 %102, %761
  %765 = add i32 %764, %585
  %766 = icmp sgt i32 %765, -1
  br i1 %766, label %767, label %772

767:                                              ; preds = %763
  %768 = icmp ugt i32 %765, 40
  %769 = zext nneg i32 %765 to i64
  %770 = shl i64 %586, %769
  %771 = select i1 %768, i64 4611686018427387904, i64 %770
  br label %797

772:                                              ; preds = %763
  %773 = sub nsw i32 0, %765
  %774 = icmp ult i32 %765, -63
  %775 = zext nneg i32 %773 to i64
  %776 = lshr i64 %586, %775
  %777 = select i1 %774, i64 0, i64 %776
  %778 = shl nsw i64 -1, %775
  %779 = xor i64 %778, -1
  %780 = select i1 %774, i64 -1, i64 %779
  %781 = and i64 %780, %586
  %782 = icmp eq i64 %781, 0
  %783 = or i1 %774, %782
  %784 = select i1 %782, i64 %777, i64 0
  br i1 %783, label %797, label %785

785:                                              ; preds = %772
  %786 = xor i32 %765, -1
  %787 = zext nneg i32 %786 to i64
  %788 = shl nuw nsw i64 1, %787
  %789 = icmp ugt i64 %781, %788
  %790 = icmp eq i64 %781, %788
  %791 = and i64 %776, 1
  %792 = icmp ne i64 %791, 0
  %793 = select i1 %790, i1 %792, i1 false
  %794 = select i1 %789, i1 true, i1 %793
  %795 = zext i1 %794 to i64
  %796 = add nuw nsw i64 %776, %795
  br label %797

797:                                              ; preds = %785, %772, %767
  %798 = phi i64 [ %771, %767 ], [ %796, %785 ], [ %784, %772 ]
  %799 = icmp ult i64 %798, %111
  br i1 %799, label %804, label %800

800:                                              ; preds = %797
  %801 = sub i32 %761, %107
  %802 = sext i32 %801 to i64
  %803 = shl nsw i64 %802, %110
  br label %813

804:                                              ; preds = %797
  %805 = lshr i64 %798, %110
  %806 = icmp eq i64 %805, 0
  %807 = add nsw i32 %761, %108
  %808 = sext i32 %807 to i64
  %809 = add nsw i64 %808, -1
  %810 = shl nsw i64 %809, %110
  %811 = select i1 %806, i64 0, i64 %810
  %812 = add i64 %811, %798
  br label %813

813:                                              ; preds = %804, %800, %757
  %814 = phi i64 [ 1099511627776, %757 ], [ %803, %800 ], [ %812, %804 ]
  %815 = icmp slt i64 %814, %112
  %816 = trunc i64 %814 to i32
  %817 = select i1 %815, i32 %816, i32 %105
  %818 = or i32 %817, %745
  br label %819

819:                                              ; preds = %590, %663, %668, %736, %741, %747, %749, %813
  %820 = phi i32 [ %740, %736 ], [ 2147483647, %668 ], [ 32767, %741 ], [ %666, %663 ], [ 127, %590 ], [ %748, %747 ], [ %818, %813 ], [ %745, %749 ]
  %821 = and i32 %820, %96
  %822 = zext i32 %821 to i64
  %823 = shl i64 %822, %129
  %824 = or i64 %823, %122
  %825 = add nuw nsw i32 %121, 1
  br label %120, !llvm.loop !20030

826:                                              ; preds = %124, %17, %25, %34, %38, %40, %42, %67
  %827 = phi i64 [ %69, %67 ], [ %43, %42 ], [ %41, %40 ], [ %39, %38 ], [ %37, %34 ], [ %28, %25 ], [ %19, %17 ], [ %122, %124 ]
  ret i64 %827
}

; Function Attrs: nofree norecurse nosync nounwind memory(none)
define internal fastcc double @__zluda_agx_cvt_mm_comb(i32 noundef %0, i32 noundef %1, double noundef %2, double noundef %3) unnamed_addr #3 {
  %5 = icmp eq i32 %0, 0
  br i1 %5, label %6, label %8

6:                                                ; preds = %4
  %7 = fadd double %2, %3
  br label %26

8:                                                ; preds = %4
  %9 = fcmp uno double %2, 0.000000e+00
  br i1 %9, label %26, label %10

10:                                               ; preds = %8
  %11 = fcmp uno double %3, 0.000000e+00
  br i1 %11, label %26, label %12

12:                                               ; preds = %10
  %13 = fcmp oeq double %2, %3
  %14 = icmp eq i32 %0, 1
  br i1 %13, label %15, label %22

15:                                               ; preds = %12
  %16 = bitcast double %2 to i64
  %17 = bitcast double %3 to i64
  %18 = or i64 %17, %16
  %19 = and i64 %17, %16
  %20 = select i1 %14, i64 %18, i64 %19
  %21 = bitcast i64 %20 to double
  br label %26

22:                                               ; preds = %12
  %23 = fcmp uge double %2, %3
  %24 = xor i1 %14, %23
  %25 = select i1 %24, double %2, double %3
  br label %26

26:                                               ; preds = %6, %8, %10, %15, %22
  %27 = phi double [ %7, %6 ], [ %21, %15 ], [ %25, %22 ], [ %3, %8 ], [ %2, %10 ]
  %28 = add i32 %1, -3
  %29 = icmp ult i32 %28, 2
  br i1 %29, label %30, label %168

30:                                               ; preds = %26
  %31 = bitcast double %27 to i64
  %32 = lshr i64 %31, 52
  %33 = trunc i64 %32 to i32
  %34 = and i32 %33, 2047
  %35 = and i64 %31, 4503599627370495
  switch i32 %34, label %41 [
    i32 2047, label %36
    i32 0, label %44
  ]

36:                                               ; preds = %30
  %37 = icmp ne i64 %35, 0
  %38 = zext i1 %37 to i32
  %39 = icmp eq i64 %35, 0
  %40 = zext i1 %39 to i32
  br label %44

41:                                               ; preds = %30
  %42 = or disjoint i64 %35, 4503599627370496
  %43 = add nsw i32 %34, -1075
  br label %44

44:                                               ; preds = %41, %36, %30
  %45 = phi i32 [ 0, %36 ], [ %43, %41 ], [ -1074, %30 ]
  %46 = phi i64 [ 0, %36 ], [ %42, %41 ], [ %35, %30 ]
  %47 = phi i32 [ %40, %36 ], [ 0, %41 ], [ %34, %30 ]
  %48 = phi i32 [ %38, %36 ], [ 0, %41 ], [ %34, %30 ]
  %49 = icmp eq i32 %48, 0
  br i1 %49, label %50, label %124

50:                                               ; preds = %44
  %51 = lshr i64 %31, 48
  %52 = trunc i64 %51 to i32
  %53 = and i32 %52, 32768
  %54 = icmp eq i32 %47, 0
  br i1 %54, label %57, label %55

55:                                               ; preds = %50
  %56 = or disjoint i32 %53, 31744
  br label %124

57:                                               ; preds = %50
  %58 = icmp eq i64 %46, 0
  br i1 %58, label %124, label %59

59:                                               ; preds = %57, %59
  %60 = phi i32 [ %63, %59 ], [ 0, %57 ]
  %61 = phi i64 [ %62, %59 ], [ %46, %57 ]
  %62 = lshr i64 %61, 1
  %63 = add nuw nsw i32 %60, 1
  %64 = icmp ult i64 %61, 2
  br i1 %64, label %65, label %59, !llvm.loop !20003

65:                                               ; preds = %59
  %66 = add nsw i32 %63, %45
  %67 = add i32 %60, %45
  %68 = icmp sgt i32 %66, -14
  %69 = select i1 %68, i32 %67, i32 -14
  %70 = icmp sgt i32 %69, 200
  br i1 %70, label %119, label %71

71:                                               ; preds = %65
  %72 = sub i32 %45, %69
  %73 = add i32 %72, 10
  %74 = icmp sgt i32 %73, -1
  br i1 %74, label %75, label %80

75:                                               ; preds = %71
  %76 = icmp ugt i32 %73, 40
  %77 = zext nneg i32 %73 to i64
  %78 = shl i64 %46, %77
  %79 = select i1 %76, i64 4611686018427387904, i64 %78
  br label %105

80:                                               ; preds = %71
  %81 = sub nuw i32 -10, %72
  %82 = icmp ult i32 %73, -63
  %83 = zext nneg i32 %81 to i64
  %84 = lshr i64 %46, %83
  %85 = select i1 %82, i64 0, i64 %84
  %86 = shl nsw i64 -1, %83
  %87 = xor i64 %86, -1
  %88 = select i1 %82, i64 -1, i64 %87
  %89 = and i64 %88, %46
  %90 = icmp eq i64 %89, 0
  %91 = or i1 %82, %90
  %92 = select i1 %90, i64 %85, i64 0
  br i1 %91, label %105, label %93

93:                                               ; preds = %80
  %94 = sub nuw nsw i32 -11, %72
  %95 = zext nneg i32 %94 to i64
  %96 = shl nuw nsw i64 1, %95
  %97 = icmp ugt i64 %89, %96
  %98 = icmp eq i64 %89, %96
  %99 = and i64 %84, 1
  %100 = icmp ne i64 %99, 0
  %101 = select i1 %98, i1 %100, i1 false
  %102 = select i1 %97, i1 true, i1 %101
  %103 = zext i1 %102 to i64
  %104 = add nuw nsw i64 %84, %103
  br label %105

105:                                              ; preds = %93, %80, %75
  %106 = phi i64 [ %79, %75 ], [ %104, %93 ], [ %92, %80 ]
  %107 = icmp ult i64 %106, 2048
  br i1 %107, label %112, label %108

108:                                              ; preds = %105
  %109 = add nsw i32 %69, 16
  %110 = sext i32 %109 to i64
  %111 = shl nsw i64 %110, 10
  br label %119

112:                                              ; preds = %105
  %113 = icmp ult i64 %106, 1024
  %114 = sext i32 %69 to i64
  %115 = shl nsw i64 %114, 10
  %116 = add nsw i64 %115, 14336
  %117 = select i1 %113, i64 0, i64 %116
  %118 = add i64 %117, %106
  br label %119

119:                                              ; preds = %112, %108, %65
  %120 = phi i64 [ 1099511627776, %65 ], [ %111, %108 ], [ %118, %112 ]
  %121 = tail call i64 @llvm.smin.i64(i64 %120, i64 31744)
  %122 = trunc i64 %121 to i32
  %123 = or i32 %53, %122
  br label %124

124:                                              ; preds = %119, %57, %55, %44
  %125 = phi i32 [ 32767, %44 ], [ %56, %55 ], [ %123, %119 ], [ %53, %57 ]
  %126 = lshr i32 %125, 10
  %127 = and i32 %126, 31
  %128 = and i32 %125, 1023
  switch i32 %127, label %132 [
    i32 31, label %129
    i32 0, label %135
  ]

129:                                              ; preds = %124
  %130 = icmp ne i32 %128, 0
  %131 = icmp eq i32 %128, 0
  br label %135

132:                                              ; preds = %124
  %133 = or disjoint i32 %128, 1024
  %134 = add nuw nsw i32 %127, 998
  br label %135

135:                                              ; preds = %132, %129, %124
  %136 = phi i32 [ 1023, %129 ], [ %134, %132 ], [ 999, %124 ]
  %137 = phi i32 [ 0, %129 ], [ %133, %132 ], [ %128, %124 ]
  %138 = phi i1 [ %131, %129 ], [ false, %132 ], [ false, %124 ]
  %139 = phi i1 [ %130, %129 ], [ false, %132 ], [ false, %124 ]
  %140 = zext nneg i32 %137 to i64
  %141 = or i1 %138, %139
  %142 = select i1 %139, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %141, label %161, label %143

143:                                              ; preds = %135
  %144 = icmp eq i32 %137, 0
  br i1 %144, label %161, label %145

145:                                              ; preds = %143, %145
  %146 = phi i32 [ %149, %145 ], [ 0, %143 ]
  %147 = phi i64 [ %148, %145 ], [ %140, %143 ]
  %148 = lshr i64 %147, 1
  %149 = add nuw nsw i32 %146, 1
  %150 = icmp ult i64 %147, 2
  br i1 %150, label %151, label %145, !llvm.loop !20003

151:                                              ; preds = %145
  %152 = add i32 %146, %136
  %153 = zext i32 %152 to i64
  %154 = shl i64 %153, 52
  %155 = sub nsw i32 52, %146
  %156 = zext nneg i32 %155 to i64
  %157 = shl i64 %140, %156
  %158 = and i64 %157, 4503599627370495
  %159 = or disjoint i64 %158, %154
  %160 = bitcast i64 %159 to double
  br label %161

161:                                              ; preds = %135, %143, %151
  %162 = phi double [ %160, %151 ], [ %142, %135 ], [ 0.000000e+00, %143 ]
  %163 = and i32 %125, 32768
  %164 = icmp eq i32 %163, 0
  %165 = tail call double @llvm.fabs.f64(double %162)
  %166 = fneg double %165
  %167 = select i1 %164, double %162, double %166
  br label %168

168:                                              ; preds = %26, %161
  %169 = phi double [ %167, %161 ], [ %27, %26 ]
  %170 = icmp eq i32 %1, 5
  br i1 %170, label %659, label %171

171:                                              ; preds = %168
  %172 = bitcast double %169 to i64
  %173 = lshr i64 %172, 52
  %174 = trunc i64 %173 to i32
  %175 = and i32 %174, 2047
  %176 = and i64 %172, 4503599627370495
  switch i32 %175, label %182 [
    i32 2047, label %177
    i32 0, label %185
  ]

177:                                              ; preds = %171
  %178 = icmp ne i64 %176, 0
  %179 = zext i1 %178 to i32
  %180 = icmp eq i64 %176, 0
  %181 = zext i1 %180 to i32
  br label %185

182:                                              ; preds = %171
  %183 = or disjoint i64 %176, 4503599627370496
  %184 = add nsw i32 %175, -1075
  br label %185

185:                                              ; preds = %182, %177, %171
  %186 = phi i32 [ 0, %177 ], [ %184, %182 ], [ -1074, %171 ]
  %187 = phi i64 [ 0, %177 ], [ %183, %182 ], [ %176, %171 ]
  %188 = phi i32 [ %181, %177 ], [ 0, %182 ], [ %175, %171 ]
  %189 = phi i32 [ %179, %177 ], [ 0, %182 ], [ %175, %171 ]
  %190 = icmp eq i32 %189, 0
  br i1 %29, label %191, label %276

191:                                              ; preds = %185
  %192 = icmp eq i32 %1, 3
  br i1 %190, label %193, label %442

193:                                              ; preds = %191
  %194 = lshr i64 %172, 56
  %195 = trunc i64 %194 to i32
  %196 = and i32 %195, 128
  %197 = select i1 %192, i64 126, i64 123
  %198 = icmp eq i32 %188, 0
  br i1 %198, label %199, label %272

199:                                              ; preds = %193
  %200 = icmp eq i64 %187, 0
  br i1 %200, label %269, label %201

201:                                              ; preds = %199, %201
  %202 = phi i32 [ %205, %201 ], [ 0, %199 ]
  %203 = phi i64 [ %204, %201 ], [ %187, %199 ]
  %204 = lshr i64 %203, 1
  %205 = add nuw nsw i32 %202, 1
  %206 = icmp ult i64 %203, 2
  br i1 %206, label %207, label %201, !llvm.loop !20003

207:                                              ; preds = %201
  %208 = select i1 %192, i32 -8, i32 -16
  %209 = xor i32 %208, -1
  %210 = select i1 %192, i32 3, i32 2
  %211 = or disjoint i32 %208, 2
  %212 = add nsw i32 %205, %186
  %213 = add i32 %202, %186
  %214 = icmp sgt i32 %212, %211
  %215 = select i1 %214, i32 %213, i32 %211
  %216 = icmp sgt i32 %215, 200
  br i1 %216, label %269, label %217

217:                                              ; preds = %207
  %218 = sub i32 %210, %215
  %219 = add i32 %218, %186
  %220 = icmp sgt i32 %219, -1
  br i1 %220, label %221, label %226

221:                                              ; preds = %217
  %222 = icmp ugt i32 %219, 40
  %223 = zext nneg i32 %219 to i64
  %224 = shl i64 %187, %223
  %225 = select i1 %222, i64 4611686018427387904, i64 %224
  br label %251

226:                                              ; preds = %217
  %227 = sub nsw i32 0, %219
  %228 = icmp ult i32 %219, -63
  %229 = zext nneg i32 %227 to i64
  %230 = lshr i64 %187, %229
  %231 = select i1 %228, i64 0, i64 %230
  %232 = shl nsw i64 -1, %229
  %233 = xor i64 %232, -1
  %234 = select i1 %228, i64 -1, i64 %233
  %235 = and i64 %234, %187
  %236 = icmp eq i64 %235, 0
  %237 = or i1 %228, %236
  %238 = select i1 %236, i64 %231, i64 0
  br i1 %237, label %251, label %239

239:                                              ; preds = %226
  %240 = xor i32 %219, -1
  %241 = zext nneg i32 %240 to i64
  %242 = shl nuw nsw i64 1, %241
  %243 = icmp ugt i64 %235, %242
  %244 = icmp eq i64 %235, %242
  %245 = and i64 %230, 1
  %246 = icmp ne i64 %245, 0
  %247 = select i1 %244, i1 %246, i1 false
  %248 = select i1 %243, i1 true, i1 %247
  %249 = zext i1 %248 to i64
  %250 = add nuw nsw i64 %230, %249
  br label %251

251:                                              ; preds = %239, %226, %221
  %252 = phi i64 [ %225, %221 ], [ %250, %239 ], [ %238, %226 ]
  %253 = zext nneg i32 %210 to i64
  %254 = shl nuw nsw i64 2, %253
  %255 = icmp ult i64 %252, %254
  br i1 %255, label %260, label %256

256:                                              ; preds = %251
  %257 = sub nsw i32 %215, %208
  %258 = sext i32 %257 to i64
  %259 = shl nsw i64 %258, %253
  br label %269

260:                                              ; preds = %251
  %261 = lshr i64 %252, %253
  %262 = icmp eq i64 %261, 0
  %263 = add nsw i32 %215, %209
  %264 = sext i32 %263 to i64
  %265 = add nsw i64 %264, -1
  %266 = shl nsw i64 %265, %253
  %267 = select i1 %262, i64 0, i64 %266
  %268 = add i64 %267, %252
  br label %269

269:                                              ; preds = %260, %256, %207, %199
  %270 = phi i64 [ 0, %199 ], [ 1099511627776, %207 ], [ %259, %256 ], [ %268, %260 ]
  %271 = tail call i64 @llvm.smin.i64(i64 %270, i64 %197)
  br label %272

272:                                              ; preds = %269, %193
  %273 = phi i64 [ %197, %193 ], [ %271, %269 ]
  %274 = trunc i64 %273 to i32
  %275 = or i32 %196, %274
  br label %442

276:                                              ; preds = %185
  %277 = icmp eq i32 %1, 0
  br i1 %277, label %278, label %351

278:                                              ; preds = %276
  br i1 %190, label %279, label %442

279:                                              ; preds = %278
  %280 = lshr i64 %172, 32
  %281 = trunc i64 %280 to i32
  %282 = and i32 %281, -2147483648
  %283 = icmp eq i32 %188, 0
  br i1 %283, label %284, label %346

284:                                              ; preds = %279
  %285 = icmp eq i64 %187, 0
  br i1 %285, label %346, label %286

286:                                              ; preds = %284, %286
  %287 = phi i32 [ %290, %286 ], [ 0, %284 ]
  %288 = phi i64 [ %289, %286 ], [ %187, %284 ]
  %289 = lshr i64 %288, 1
  %290 = add nuw nsw i32 %287, 1
  %291 = icmp ult i64 %288, 2
  br i1 %291, label %292, label %286, !llvm.loop !20003

292:                                              ; preds = %286
  %293 = add nsw i32 %290, %186
  %294 = add i32 %287, %186
  %295 = icmp sgt i32 %293, -126
  %296 = select i1 %295, i32 %294, i32 -126
  %297 = icmp sgt i32 %296, 200
  br i1 %297, label %346, label %298

298:                                              ; preds = %292
  %299 = sub i32 %186, %296
  %300 = add i32 %299, 23
  %301 = icmp sgt i32 %300, -1
  br i1 %301, label %302, label %307

302:                                              ; preds = %298
  %303 = icmp ugt i32 %300, 40
  %304 = zext nneg i32 %300 to i64
  %305 = shl i64 %187, %304
  %306 = select i1 %303, i64 4611686018427387904, i64 %305
  br label %332

307:                                              ; preds = %298
  %308 = sub nuw i32 -23, %299
  %309 = icmp ult i32 %300, -63
  %310 = zext nneg i32 %308 to i64
  %311 = lshr i64 %187, %310
  %312 = select i1 %309, i64 0, i64 %311
  %313 = shl nsw i64 -1, %310
  %314 = xor i64 %313, -1
  %315 = select i1 %309, i64 -1, i64 %314
  %316 = and i64 %315, %187
  %317 = icmp eq i64 %316, 0
  %318 = or i1 %309, %317
  %319 = select i1 %317, i64 %312, i64 0
  br i1 %318, label %332, label %320

320:                                              ; preds = %307
  %321 = sub nuw nsw i32 -24, %299
  %322 = zext nneg i32 %321 to i64
  %323 = shl nuw nsw i64 1, %322
  %324 = icmp ugt i64 %316, %323
  %325 = icmp eq i64 %316, %323
  %326 = and i64 %311, 1
  %327 = icmp ne i64 %326, 0
  %328 = select i1 %325, i1 %327, i1 false
  %329 = select i1 %324, i1 true, i1 %328
  %330 = zext i1 %329 to i64
  %331 = add nuw nsw i64 %311, %330
  br label %332

332:                                              ; preds = %320, %307, %302
  %333 = phi i64 [ %306, %302 ], [ %331, %320 ], [ %319, %307 ]
  %334 = icmp ult i64 %333, 16777216
  br i1 %334, label %339, label %335

335:                                              ; preds = %332
  %336 = add nsw i32 %296, 128
  %337 = sext i32 %336 to i64
  %338 = shl nsw i64 %337, 23
  br label %346

339:                                              ; preds = %332
  %340 = icmp ult i64 %333, 8388608
  %341 = sext i32 %296 to i64
  %342 = shl nsw i64 %341, 23
  %343 = add nsw i64 %342, 1056964608
  %344 = select i1 %340, i64 0, i64 %343
  %345 = add i64 %344, %333
  br label %346

346:                                              ; preds = %339, %335, %292, %284, %279
  %347 = phi i64 [ 2139095040, %279 ], [ 0, %284 ], [ 1099511627776, %292 ], [ %338, %335 ], [ %345, %339 ]
  %348 = tail call i64 @llvm.smin.i64(i64 %347, i64 2139095040)
  %349 = trunc i64 %348 to i32
  %350 = or i32 %282, %349
  br label %442

351:                                              ; preds = %276
  br i1 %190, label %352, label %442

352:                                              ; preds = %351
  %353 = icmp eq i32 %1, 2
  %354 = select i1 %353, i32 8, i32 5
  %355 = select i1 %353, i32 7, i32 10
  %356 = shl nsw i32 -1, %354
  %357 = xor i32 %356, -1
  %358 = shl nuw nsw i32 %357, %355
  %359 = lshr i64 %172, 48
  %360 = trunc i64 %359 to i32
  %361 = and i32 %360, 32768
  %362 = icmp eq i32 %188, 0
  br i1 %362, label %365, label %363

363:                                              ; preds = %352
  %364 = or i32 %361, %358
  br label %442

365:                                              ; preds = %352
  %366 = icmp eq i64 %187, 0
  br i1 %366, label %442, label %367

367:                                              ; preds = %365, %367
  %368 = phi i32 [ %371, %367 ], [ 0, %365 ]
  %369 = phi i64 [ %370, %367 ], [ %187, %365 ]
  %370 = lshr i64 %369, 1
  %371 = add nuw nsw i32 %368, 1
  %372 = icmp ult i64 %369, 2
  br i1 %372, label %373, label %367, !llvm.loop !20003

373:                                              ; preds = %367
  %374 = add nsw i32 %354, -1
  %375 = shl nsw i32 -1, %374
  %376 = xor i32 %375, -1
  %377 = add nuw nsw i32 %375, 2
  %378 = add nsw i32 %371, %186
  %379 = add i32 %368, %186
  %380 = icmp sgt i32 %378, %377
  %381 = select i1 %380, i32 %379, i32 %377
  %382 = icmp sgt i32 %381, 200
  br i1 %382, label %435, label %383

383:                                              ; preds = %373
  %384 = sub i32 %355, %381
  %385 = add i32 %384, %186
  %386 = icmp sgt i32 %385, -1
  br i1 %386, label %387, label %392

387:                                              ; preds = %383
  %388 = icmp ugt i32 %385, 40
  %389 = zext nneg i32 %385 to i64
  %390 = shl i64 %187, %389
  %391 = select i1 %388, i64 4611686018427387904, i64 %390
  br label %417

392:                                              ; preds = %383
  %393 = sub nsw i32 0, %385
  %394 = icmp ult i32 %385, -63
  %395 = zext nneg i32 %393 to i64
  %396 = lshr i64 %187, %395
  %397 = select i1 %394, i64 0, i64 %396
  %398 = shl nsw i64 -1, %395
  %399 = xor i64 %398, -1
  %400 = select i1 %394, i64 -1, i64 %399
  %401 = and i64 %400, %187
  %402 = icmp eq i64 %401, 0
  %403 = or i1 %394, %402
  %404 = select i1 %402, i64 %397, i64 0
  br i1 %403, label %417, label %405

405:                                              ; preds = %392
  %406 = xor i32 %385, -1
  %407 = zext nneg i32 %406 to i64
  %408 = shl nuw nsw i64 1, %407
  %409 = icmp ugt i64 %401, %408
  %410 = icmp eq i64 %401, %408
  %411 = and i64 %396, 1
  %412 = icmp ne i64 %411, 0
  %413 = select i1 %410, i1 %412, i1 false
  %414 = select i1 %409, i1 true, i1 %413
  %415 = zext i1 %414 to i64
  %416 = add nuw nsw i64 %396, %415
  br label %417

417:                                              ; preds = %405, %392, %387
  %418 = phi i64 [ %391, %387 ], [ %416, %405 ], [ %404, %392 ]
  %419 = zext nneg i32 %355 to i64
  %420 = shl nuw nsw i64 2, %419
  %421 = icmp ult i64 %418, %420
  br i1 %421, label %426, label %422

422:                                              ; preds = %417
  %423 = sub i32 %381, %375
  %424 = sext i32 %423 to i64
  %425 = shl nsw i64 %424, %419
  br label %435

426:                                              ; preds = %417
  %427 = lshr i64 %418, %419
  %428 = icmp eq i64 %427, 0
  %429 = add nsw i32 %381, %376
  %430 = sext i32 %429 to i64
  %431 = add nsw i64 %430, -1
  %432 = shl nsw i64 %431, %419
  %433 = select i1 %428, i64 0, i64 %432
  %434 = add i64 %433, %418
  br label %435

435:                                              ; preds = %426, %422, %373
  %436 = phi i64 [ 1099511627776, %373 ], [ %425, %422 ], [ %434, %426 ]
  %437 = zext nneg i32 %358 to i64
  %438 = icmp slt i64 %436, %437
  %439 = trunc i64 %436 to i32
  %440 = select i1 %438, i32 %439, i32 %358
  %441 = or i32 %440, %361
  br label %442

442:                                              ; preds = %435, %365, %363, %351, %346, %278, %272, %191
  %443 = phi i32 [ %350, %346 ], [ 2147483647, %278 ], [ 32767, %351 ], [ %275, %272 ], [ 127, %191 ], [ %364, %363 ], [ %441, %435 ], [ %361, %365 ]
  switch i32 %1, label %614 [
    i32 0, label %444
    i32 1, label %487
    i32 2, label %530
    i32 3, label %574
  ]

444:                                              ; preds = %442
  %445 = lshr i32 %443, 23
  %446 = and i32 %443, 8388607
  %447 = trunc i32 %445 to i8
  switch i8 %447, label %451 [
    i8 -1, label %448
    i8 0, label %455
  ]

448:                                              ; preds = %444
  %449 = icmp ne i32 %446, 0
  %450 = icmp eq i32 %446, 0
  br label %455

451:                                              ; preds = %444
  %452 = and i32 %445, 255
  %453 = or disjoint i32 %446, 8388608
  %454 = add nuw nsw i32 %452, 873
  br label %455

455:                                              ; preds = %451, %448, %444
  %456 = phi i32 [ 1023, %448 ], [ %454, %451 ], [ 874, %444 ]
  %457 = phi i32 [ 0, %448 ], [ %453, %451 ], [ %446, %444 ]
  %458 = phi i1 [ %450, %448 ], [ false, %451 ], [ false, %444 ]
  %459 = phi i1 [ %449, %448 ], [ false, %451 ], [ false, %444 ]
  %460 = zext nneg i32 %457 to i64
  %461 = or i1 %458, %459
  %462 = select i1 %459, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %461, label %481, label %463

463:                                              ; preds = %455
  %464 = icmp eq i32 %457, 0
  br i1 %464, label %481, label %465

465:                                              ; preds = %463, %465
  %466 = phi i32 [ %469, %465 ], [ 0, %463 ]
  %467 = phi i64 [ %468, %465 ], [ %460, %463 ]
  %468 = lshr i64 %467, 1
  %469 = add nuw nsw i32 %466, 1
  %470 = icmp ult i64 %467, 2
  br i1 %470, label %471, label %465, !llvm.loop !20003

471:                                              ; preds = %465
  %472 = add i32 %466, %456
  %473 = zext i32 %472 to i64
  %474 = shl i64 %473, 52
  %475 = sub nsw i32 52, %466
  %476 = zext nneg i32 %475 to i64
  %477 = shl i64 %460, %476
  %478 = and i64 %477, 4503599627370495
  %479 = or disjoint i64 %478, %474
  %480 = bitcast i64 %479 to double
  br label %481

481:                                              ; preds = %471, %463, %455
  %482 = phi double [ %480, %471 ], [ %462, %455 ], [ 0.000000e+00, %463 ]
  %483 = tail call double @llvm.fabs.f64(double %482)
  %484 = fneg double %483
  %485 = icmp slt i32 %443, 0
  %486 = select i1 %485, double %484, double %482
  br label %659

487:                                              ; preds = %442
  %488 = lshr i32 %443, 10
  %489 = and i32 %488, 31
  %490 = and i32 %443, 1023
  switch i32 %489, label %494 [
    i32 31, label %491
    i32 0, label %497
  ]

491:                                              ; preds = %487
  %492 = icmp ne i32 %490, 0
  %493 = icmp eq i32 %490, 0
  br label %497

494:                                              ; preds = %487
  %495 = or disjoint i32 %490, 1024
  %496 = add nuw nsw i32 %489, 998
  br label %497

497:                                              ; preds = %494, %491, %487
  %498 = phi i32 [ 1023, %491 ], [ %496, %494 ], [ 999, %487 ]
  %499 = phi i32 [ 0, %491 ], [ %495, %494 ], [ %490, %487 ]
  %500 = phi i1 [ %493, %491 ], [ false, %494 ], [ false, %487 ]
  %501 = phi i1 [ %492, %491 ], [ false, %494 ], [ false, %487 ]
  %502 = zext nneg i32 %499 to i64
  %503 = or i1 %500, %501
  %504 = select i1 %501, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %503, label %523, label %505

505:                                              ; preds = %497
  %506 = icmp eq i32 %499, 0
  br i1 %506, label %523, label %507

507:                                              ; preds = %505, %507
  %508 = phi i32 [ %511, %507 ], [ 0, %505 ]
  %509 = phi i64 [ %510, %507 ], [ %502, %505 ]
  %510 = lshr i64 %509, 1
  %511 = add nuw nsw i32 %508, 1
  %512 = icmp ult i64 %509, 2
  br i1 %512, label %513, label %507, !llvm.loop !20003

513:                                              ; preds = %507
  %514 = add i32 %508, %498
  %515 = zext i32 %514 to i64
  %516 = shl i64 %515, 52
  %517 = sub nsw i32 52, %508
  %518 = zext nneg i32 %517 to i64
  %519 = shl i64 %502, %518
  %520 = and i64 %519, 4503599627370495
  %521 = or disjoint i64 %520, %516
  %522 = bitcast i64 %521 to double
  br label %523

523:                                              ; preds = %513, %505, %497
  %524 = phi double [ %522, %513 ], [ %504, %497 ], [ 0.000000e+00, %505 ]
  %525 = and i32 %443, 32768
  %526 = icmp eq i32 %525, 0
  %527 = tail call double @llvm.fabs.f64(double %524)
  %528 = fneg double %527
  %529 = select i1 %526, double %524, double %528
  br label %659

530:                                              ; preds = %442
  %531 = shl i32 %443, 16
  %532 = lshr i32 %531, 23
  %533 = and i32 %531, 8323072
  %534 = trunc i32 %532 to i8
  switch i8 %534, label %538 [
    i8 -1, label %535
    i8 0, label %542
  ]

535:                                              ; preds = %530
  %536 = icmp ne i32 %533, 0
  %537 = icmp eq i32 %533, 0
  br label %542

538:                                              ; preds = %530
  %539 = and i32 %532, 255
  %540 = or disjoint i32 %533, 8388608
  %541 = add nuw nsw i32 %539, 873
  br label %542

542:                                              ; preds = %538, %535, %530
  %543 = phi i32 [ 1023, %535 ], [ %541, %538 ], [ 874, %530 ]
  %544 = phi i32 [ 0, %535 ], [ %540, %538 ], [ %533, %530 ]
  %545 = phi i1 [ %537, %535 ], [ false, %538 ], [ false, %530 ]
  %546 = phi i1 [ %536, %535 ], [ false, %538 ], [ false, %530 ]
  %547 = zext nneg i32 %544 to i64
  %548 = or i1 %545, %546
  %549 = select i1 %546, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %548, label %568, label %550

550:                                              ; preds = %542
  %551 = icmp eq i32 %544, 0
  br i1 %551, label %568, label %552

552:                                              ; preds = %550, %552
  %553 = phi i32 [ %556, %552 ], [ 0, %550 ]
  %554 = phi i64 [ %555, %552 ], [ %547, %550 ]
  %555 = lshr i64 %554, 1
  %556 = add nuw nsw i32 %553, 1
  %557 = icmp ult i64 %554, 2
  br i1 %557, label %558, label %552, !llvm.loop !20003

558:                                              ; preds = %552
  %559 = add i32 %553, %543
  %560 = zext i32 %559 to i64
  %561 = shl i64 %560, 52
  %562 = sub nsw i32 52, %553
  %563 = zext nneg i32 %562 to i64
  %564 = shl i64 %547, %563
  %565 = and i64 %564, 4503599627370495
  %566 = or disjoint i64 %565, %561
  %567 = bitcast i64 %566 to double
  br label %568

568:                                              ; preds = %558, %550, %542
  %569 = phi double [ %567, %558 ], [ %549, %542 ], [ 0.000000e+00, %550 ]
  %570 = tail call double @llvm.fabs.f64(double %569)
  %571 = fneg double %570
  %572 = icmp slt i32 %531, 0
  %573 = select i1 %572, double %571, double %569
  br label %659

574:                                              ; preds = %442
  %575 = and i32 %443, 127
  %576 = icmp eq i32 %575, 127
  br i1 %576, label %586, label %577

577:                                              ; preds = %574
  %578 = lshr i32 %575, 3
  %579 = and i32 %443, 7
  %580 = icmp ult i32 %575, 8
  %581 = or disjoint i32 %579, 8
  %582 = add nuw nsw i32 %578, 1013
  %583 = select i1 %580, i32 %579, i32 %581
  %584 = zext nneg i32 %583 to i64
  %585 = select i1 %580, i32 1014, i32 %582
  br label %586

586:                                              ; preds = %577, %574
  %587 = phi i32 [ 1023, %574 ], [ %585, %577 ]
  %588 = phi i64 [ 0, %574 ], [ %584, %577 ]
  br i1 %576, label %607, label %589

589:                                              ; preds = %586
  %590 = icmp eq i64 %588, 0
  br i1 %590, label %607, label %591

591:                                              ; preds = %589, %591
  %592 = phi i32 [ %595, %591 ], [ 0, %589 ]
  %593 = phi i64 [ %594, %591 ], [ %588, %589 ]
  %594 = lshr i64 %593, 1
  %595 = add nuw nsw i32 %592, 1
  %596 = icmp ult i64 %593, 2
  br i1 %596, label %597, label %591, !llvm.loop !20003

597:                                              ; preds = %591
  %598 = add i32 %592, %587
  %599 = zext i32 %598 to i64
  %600 = shl i64 %599, 52
  %601 = sub nsw i32 52, %592
  %602 = zext nneg i32 %601 to i64
  %603 = shl i64 %588, %602
  %604 = and i64 %603, 4503599627370495
  %605 = or disjoint i64 %604, %600
  %606 = bitcast i64 %605 to double
  br label %607

607:                                              ; preds = %597, %589, %586
  %608 = phi double [ %606, %597 ], [ 0x7FF8000000000000, %586 ], [ 0.000000e+00, %589 ]
  %609 = and i32 %443, 128
  %610 = icmp eq i32 %609, 0
  %611 = tail call double @llvm.fabs.f64(double %608)
  %612 = fneg double %611
  %613 = select i1 %610, double %608, double %612
  br label %659

614:                                              ; preds = %442
  %615 = and i32 %443, 127
  %616 = icmp ugt i32 %615, 124
  br i1 %616, label %628, label %617

617:                                              ; preds = %614
  %618 = icmp eq i32 %615, 124
  br i1 %618, label %628, label %619

619:                                              ; preds = %617
  %620 = lshr i32 %615, 2
  %621 = and i32 %443, 3
  %622 = icmp ult i32 %615, 4
  %623 = or disjoint i32 %621, 4
  %624 = add nuw nsw i32 %620, 1006
  %625 = select i1 %622, i32 %621, i32 %623
  %626 = zext nneg i32 %625 to i64
  %627 = select i1 %622, i32 1007, i32 %624
  br label %628

628:                                              ; preds = %619, %617, %614
  %629 = phi double [ 0x7FF8000000000000, %614 ], [ 0x7FF0000000000000, %617 ], [ 0x7FF0000000000000, %619 ]
  %630 = phi i32 [ 1023, %614 ], [ 1023, %617 ], [ %627, %619 ]
  %631 = phi i64 [ 0, %614 ], [ 0, %617 ], [ %626, %619 ]
  %632 = phi i1 [ false, %614 ], [ true, %617 ], [ false, %619 ]
  %633 = or i1 %616, %632
  br i1 %633, label %652, label %634

634:                                              ; preds = %628
  %635 = icmp eq i64 %631, 0
  br i1 %635, label %652, label %636

636:                                              ; preds = %634, %636
  %637 = phi i32 [ %640, %636 ], [ 0, %634 ]
  %638 = phi i64 [ %639, %636 ], [ %631, %634 ]
  %639 = lshr i64 %638, 1
  %640 = add nuw nsw i32 %637, 1
  %641 = icmp ult i64 %638, 2
  br i1 %641, label %642, label %636, !llvm.loop !20003

642:                                              ; preds = %636
  %643 = add i32 %637, %630
  %644 = zext i32 %643 to i64
  %645 = shl i64 %644, 52
  %646 = sub nsw i32 52, %637
  %647 = zext nneg i32 %646 to i64
  %648 = shl i64 %631, %647
  %649 = and i64 %648, 4503599627370495
  %650 = or disjoint i64 %649, %645
  %651 = bitcast i64 %650 to double
  br label %652

652:                                              ; preds = %642, %634, %628
  %653 = phi double [ %651, %642 ], [ %629, %628 ], [ 0.000000e+00, %634 ]
  %654 = and i32 %443, 128
  %655 = icmp eq i32 %654, 0
  %656 = tail call double @llvm.fabs.f64(double %653)
  %657 = fneg double %656
  %658 = select i1 %655, double %653, double %657
  br label %659

659:                                              ; preds = %168, %481, %523, %568, %607, %652
  %660 = phi double [ %169, %168 ], [ %658, %652 ], [ %613, %607 ], [ %573, %568 ], [ %529, %523 ], [ %486, %481 ]
  ret double %660
}

; Function Attrs: nofree norecurse nosync nounwind memory(readwrite, inaccessiblemem: none)
define dso_local void @__zluda_ptx_impl_agx_mm_st(i64 noundef %0, i64 noundef %1, i32 noundef %2) local_unnamed_addr #3 {
  %4 = add i64 %0, -140737488355328
  %5 = icmp ult i64 %4, -17523466567680
  %6 = lshr i64 %0, 30
  %7 = and i64 %6, 16320
  %8 = add nuw nsw i64 %7, 123145302310848
  %9 = inttoptr i64 %8 to ptr addrspace(1)
  %10 = add nuw nsw i64 %7, 123145302310856
  %11 = and i64 %0, 68719476735
  %12 = trunc i64 %1 to i32
  %13 = trunc i64 %1 to i16
  %14 = trunc i64 %1 to i8
  br label %15

15:                                               ; preds = %38, %3
  %16 = phi i32 [ 0, %3 ], [ %39, %38 ]
  br i1 %5, label %19, label %17

17:                                               ; preds = %15
  %18 = load i32, ptr addrspace(1) %9, align 64
  br label %19

19:                                               ; preds = %15, %17
  %20 = phi i32 [ %18, %17 ], [ 1, %15 ]
  %21 = icmp ult i32 %16, %20
  br i1 %21, label %23, label %22

22:                                               ; preds = %19
  ret void

23:                                               ; preds = %19
  br i1 %5, label %31, label %24

24:                                               ; preds = %23
  %25 = shl i32 %16, 3
  %26 = zext i32 %25 to i64
  %27 = add nuw nsw i64 %10, %26
  %28 = inttoptr i64 %27 to ptr addrspace(1)
  %29 = load i64, ptr addrspace(1) %28, align 8
  %30 = add i64 %29, %11
  br label %31

31:                                               ; preds = %23, %24
  %32 = phi i64 [ %30, %24 ], [ %0, %23 ]
  %33 = inttoptr i64 %32 to ptr addrspace(1)
  switch i32 %2, label %37 [
    i32 1, label %34
    i32 2, label %35
    i32 4, label %36
  ]

34:                                               ; preds = %31
  store i8 %14, ptr addrspace(1) %33, align 1
  br label %38

35:                                               ; preds = %31
  store i16 %13, ptr addrspace(1) %33, align 2
  br label %38

36:                                               ; preds = %31
  store i32 %12, ptr addrspace(1) %33, align 4
  br label %38

37:                                               ; preds = %31
  store i64 %1, ptr addrspace(1) %33, align 8
  br label %38

38:                                               ; preds = %35, %37, %36, %34
  %39 = add nuw i32 %16, 1
  br label %15, !llvm.loop !20031
}

; Function Attrs: nofree norecurse nounwind memory(readwrite, inaccessiblemem: none)
define dso_local void @__zluda_ptx_impl_agx_mm_red(i64 noundef %0, i64 noundef %1, i32 noundef %2) local_unnamed_addr #3 {
  %4 = and i32 %2, 7
  %5 = lshr i32 %2, 3
  %6 = and i32 %5, 31
  %7 = and i32 %2, 240
  %8 = icmp eq i32 %7, 96
  br i1 %8, label %12, label %9

9:                                                ; preds = %3
  switch i32 %6, label %10 [
    i32 15, label %12
    i32 14, label %12
    i32 9, label %12
    i32 8, label %12
    i32 7, label %11
    i32 5, label %11
    i32 4, label %11
    i32 3, label %11
  ]

10:                                               ; preds = %9
  br label %12

11:                                               ; preds = %9, %9, %9, %9
  br label %12

12:                                               ; preds = %3, %9, %9, %9, %9, %10, %11
  %13 = phi i1 [ false, %10 ], [ false, %3 ], [ false, %9 ], [ false, %9 ], [ false, %9 ], [ false, %9 ], [ true, %11 ]
  %14 = phi i1 [ true, %10 ], [ false, %3 ], [ false, %9 ], [ false, %9 ], [ false, %9 ], [ false, %9 ], [ false, %11 ]
  %15 = phi i32 [ 32, %10 ], [ 8, %3 ], [ 16, %9 ], [ 16, %9 ], [ 16, %9 ], [ 16, %9 ], [ 64, %11 ]
  %16 = add i64 %0, -140737488355328
  %17 = icmp ult i64 %16, -17523466567680
  %18 = lshr i64 %0, 30
  %19 = and i64 %18, 16320
  %20 = add nuw nsw i64 %19, 123145302310848
  %21 = inttoptr i64 %20 to ptr addrspace(1)
  %22 = add nuw nsw i64 %19, 123145302310856
  %23 = and i64 %0, 68719476735
  %24 = shl nsw i32 -1, %15
  %25 = xor i32 %24, -1
  %26 = icmp eq i32 %6, 6
  %27 = icmp eq i32 %4, 0
  %28 = and i1 %27, %26
  %29 = and i64 %1, 4294967295
  %30 = and i64 %1, 2139095040
  %31 = icmp eq i64 %30, 0
  %32 = and i64 %1, 2147483648
  %33 = select i1 %31, i64 %32, i64 %1
  %34 = trunc i64 %33 to i32
  %35 = lshr i32 %34, 23
  %36 = and i32 %34, 8388607
  %37 = trunc i32 %35 to i8
  %38 = icmp ne i32 %36, 0
  %39 = icmp eq i32 %36, 0
  %40 = and i32 %35, 255
  %41 = or disjoint i32 %36, 8388608
  %42 = add nuw nsw i32 %40, 873
  %43 = icmp slt i32 %34, 0
  br label %44

44:                                               ; preds = %410, %12
  %45 = phi i32 [ 0, %12 ], [ %411, %410 ]
  br i1 %17, label %48, label %46

46:                                               ; preds = %44
  %47 = load i32, ptr addrspace(1) %21, align 64
  br label %48

48:                                               ; preds = %44, %46
  %49 = phi i32 [ %47, %46 ], [ 1, %44 ]
  %50 = icmp ult i32 %45, %49
  br i1 %50, label %52, label %51

51:                                               ; preds = %48
  ret void

52:                                               ; preds = %48
  br i1 %17, label %60, label %53

53:                                               ; preds = %52
  %54 = shl i32 %45, 3
  %55 = zext i32 %54 to i64
  %56 = add nuw nsw i64 %22, %55
  %57 = inttoptr i64 %56 to ptr addrspace(1)
  %58 = load i64, ptr addrspace(1) %57, align 8
  %59 = add i64 %58, %23
  br label %60

60:                                               ; preds = %52, %53
  %61 = phi i64 [ %59, %53 ], [ %0, %52 ]
  br i1 %13, label %62, label %71

62:                                               ; preds = %60
  %63 = inttoptr i64 %61 to ptr addrspace(1)
  %64 = load i64, ptr addrspace(1) %63, align 8
  br label %65

65:                                               ; preds = %65, %62
  %66 = phi i64 [ %64, %62 ], [ %70, %65 ]
  %67 = tail call fastcc i64 @__zluda_agx_cvt_mm_combine(i32 noundef %4, i32 noundef %6, i64 noundef %66, i64 noundef %1)
  %68 = cmpxchg ptr addrspace(1) %63, i64 %66, i64 %67 monotonic monotonic, align 8
  %69 = extractvalue { i64, i1 } %68, 1
  %70 = extractvalue { i64, i1 } %68, 0
  br i1 %69, label %410, label %65, !llvm.loop !20032

71:                                               ; preds = %60
  %72 = and i64 %61, -4
  %73 = inttoptr i64 %72 to ptr addrspace(1)
  %74 = trunc i64 %61 to i32
  %75 = shl i32 %74, 3
  %76 = and i32 %75, 24
  %77 = shl i32 %25, %76
  %78 = select i1 %14, i32 -1, i32 %77
  %79 = load i32, ptr addrspace(1) %73, align 4
  %80 = xor i32 %78, -1
  br label %81

81:                                               ; preds = %401, %71
  %82 = phi i32 [ %79, %71 ], [ %409, %401 ]
  %83 = and i32 %82, %78
  %84 = lshr i32 %83, %76
  br i1 %28, label %85, label %397

85:                                               ; preds = %81
  %86 = and i32 %84, 2139095040
  %87 = icmp eq i32 %86, 0
  %88 = and i32 %84, -2147483648
  %89 = select i1 %87, i32 %88, i32 %84
  %90 = lshr i32 %89, 23
  %91 = and i32 %89, 8388607
  %92 = trunc i32 %90 to i8
  %93 = icmp slt i32 %89, 0
  switch i8 %92, label %97 [
    i8 -1, label %94
    i8 0, label %101
  ]

94:                                               ; preds = %85
  %95 = icmp eq i32 %91, 0
  %96 = icmp ne i32 %91, 0
  br label %101

97:                                               ; preds = %85
  %98 = and i32 %90, 255
  %99 = add nuw nsw i32 %98, 873
  %100 = or disjoint i32 %91, 8388608
  br label %101

101:                                              ; preds = %97, %94, %85
  %102 = phi i32 [ 1023, %94 ], [ %99, %97 ], [ 874, %85 ]
  %103 = phi i32 [ 0, %94 ], [ %100, %97 ], [ %91, %85 ]
  %104 = phi i1 [ %95, %94 ], [ false, %97 ], [ false, %85 ]
  %105 = phi i1 [ %96, %94 ], [ false, %97 ], [ false, %85 ]
  %106 = zext nneg i32 %103 to i64
  %107 = or i1 %104, %105
  %108 = select i1 %105, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %107, label %127, label %109

109:                                              ; preds = %101
  %110 = icmp eq i32 %103, 0
  br i1 %110, label %127, label %111

111:                                              ; preds = %109, %111
  %112 = phi i32 [ %115, %111 ], [ 0, %109 ]
  %113 = phi i64 [ %114, %111 ], [ %106, %109 ]
  %114 = lshr i64 %113, 1
  %115 = add nuw nsw i32 %112, 1
  %116 = icmp ult i64 %113, 2
  br i1 %116, label %117, label %111, !llvm.loop !20003

117:                                              ; preds = %111
  %118 = add i32 %112, %102
  %119 = zext i32 %118 to i64
  %120 = shl i64 %119, 52
  %121 = sub nsw i32 52, %112
  %122 = zext nneg i32 %121 to i64
  %123 = shl i64 %106, %122
  %124 = and i64 %123, 4503599627370495
  %125 = or disjoint i64 %124, %120
  %126 = bitcast i64 %125 to double
  br label %127

127:                                              ; preds = %117, %109, %101
  %128 = phi double [ %126, %117 ], [ %108, %101 ], [ 0.000000e+00, %109 ]
  %129 = tail call double @llvm.fabs.f64(double %128)
  %130 = fneg double %129
  %131 = select i1 %93, double %130, double %128
  switch i8 %37, label %133 [
    i8 -1, label %132
    i8 0, label %134
  ]

132:                                              ; preds = %127
  br label %134

133:                                              ; preds = %127
  br label %134

134:                                              ; preds = %133, %132, %127
  %135 = phi i32 [ 1023, %132 ], [ %42, %133 ], [ 874, %127 ]
  %136 = phi i32 [ 0, %132 ], [ %41, %133 ], [ %36, %127 ]
  %137 = phi i1 [ %39, %132 ], [ false, %133 ], [ false, %127 ]
  %138 = phi i1 [ %38, %132 ], [ false, %133 ], [ false, %127 ]
  %139 = zext nneg i32 %136 to i64
  %140 = or i1 %137, %138
  %141 = select i1 %138, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %140, label %160, label %142

142:                                              ; preds = %134
  %143 = icmp eq i32 %136, 0
  br i1 %143, label %160, label %144

144:                                              ; preds = %142, %144
  %145 = phi i32 [ %148, %144 ], [ 0, %142 ]
  %146 = phi i64 [ %147, %144 ], [ %139, %142 ]
  %147 = lshr i64 %146, 1
  %148 = add nuw nsw i32 %145, 1
  %149 = icmp ult i64 %146, 2
  br i1 %149, label %150, label %144, !llvm.loop !20003

150:                                              ; preds = %144
  %151 = add i32 %145, %135
  %152 = zext i32 %151 to i64
  %153 = shl i64 %152, 52
  %154 = sub nsw i32 52, %145
  %155 = zext nneg i32 %154 to i64
  %156 = shl i64 %139, %155
  %157 = and i64 %156, 4503599627370495
  %158 = or disjoint i64 %157, %153
  %159 = bitcast i64 %158 to double
  br label %160

160:                                              ; preds = %150, %142, %134
  %161 = phi double [ %159, %150 ], [ %141, %134 ], [ 0.000000e+00, %142 ]
  %162 = tail call double @llvm.fabs.f64(double %161)
  %163 = fneg double %162
  %164 = select i1 %43, double %163, double %161
  %165 = fadd double %131, %164
  %166 = bitcast double %165 to i64
  %167 = lshr i64 %166, 52
  %168 = trunc i64 %167 to i32
  %169 = and i32 %168, 2047
  %170 = and i64 %166, 4503599627370495
  switch i32 %169, label %176 [
    i32 2047, label %171
    i32 0, label %179
  ]

171:                                              ; preds = %160
  %172 = icmp ne i64 %170, 0
  %173 = zext i1 %172 to i32
  %174 = icmp eq i64 %170, 0
  %175 = zext i1 %174 to i32
  br label %179

176:                                              ; preds = %160
  %177 = or disjoint i64 %170, 4503599627370496
  %178 = add nsw i32 %169, -1075
  br label %179

179:                                              ; preds = %176, %171, %160
  %180 = phi i32 [ 0, %171 ], [ %178, %176 ], [ -1074, %160 ]
  %181 = phi i64 [ 0, %171 ], [ %177, %176 ], [ %170, %160 ]
  %182 = phi i32 [ %175, %171 ], [ 0, %176 ], [ %169, %160 ]
  %183 = phi i32 [ %173, %171 ], [ 0, %176 ], [ %169, %160 ]
  %184 = icmp eq i32 %183, 0
  br i1 %184, label %185, label %257

185:                                              ; preds = %179
  %186 = lshr i64 %166, 32
  %187 = trunc i64 %186 to i32
  %188 = and i32 %187, -2147483648
  %189 = icmp eq i32 %182, 0
  br i1 %189, label %190, label %252

190:                                              ; preds = %185
  %191 = icmp eq i64 %181, 0
  br i1 %191, label %252, label %192

192:                                              ; preds = %190, %192
  %193 = phi i32 [ %196, %192 ], [ 0, %190 ]
  %194 = phi i64 [ %195, %192 ], [ %181, %190 ]
  %195 = lshr i64 %194, 1
  %196 = add nuw nsw i32 %193, 1
  %197 = icmp ult i64 %194, 2
  br i1 %197, label %198, label %192, !llvm.loop !20003

198:                                              ; preds = %192
  %199 = add nsw i32 %196, %180
  %200 = add i32 %193, %180
  %201 = icmp sgt i32 %199, -126
  %202 = select i1 %201, i32 %200, i32 -126
  %203 = icmp sgt i32 %202, 200
  br i1 %203, label %252, label %204

204:                                              ; preds = %198
  %205 = sub i32 %180, %202
  %206 = add i32 %205, 23
  %207 = icmp sgt i32 %206, -1
  br i1 %207, label %208, label %213

208:                                              ; preds = %204
  %209 = icmp ugt i32 %206, 40
  %210 = zext nneg i32 %206 to i64
  %211 = shl i64 %181, %210
  %212 = select i1 %209, i64 4611686018427387904, i64 %211
  br label %238

213:                                              ; preds = %204
  %214 = sub nuw i32 -23, %205
  %215 = icmp ult i32 %206, -63
  %216 = zext nneg i32 %214 to i64
  %217 = lshr i64 %181, %216
  %218 = select i1 %215, i64 0, i64 %217
  %219 = shl nsw i64 -1, %216
  %220 = xor i64 %219, -1
  %221 = select i1 %215, i64 -1, i64 %220
  %222 = and i64 %221, %181
  %223 = icmp eq i64 %222, 0
  %224 = or i1 %215, %223
  %225 = select i1 %223, i64 %218, i64 0
  br i1 %224, label %238, label %226

226:                                              ; preds = %213
  %227 = sub nuw nsw i32 -24, %205
  %228 = zext nneg i32 %227 to i64
  %229 = shl nuw nsw i64 1, %228
  %230 = icmp ugt i64 %222, %229
  %231 = icmp eq i64 %222, %229
  %232 = and i64 %217, 1
  %233 = icmp ne i64 %232, 0
  %234 = select i1 %231, i1 %233, i1 false
  %235 = select i1 %230, i1 true, i1 %234
  %236 = zext i1 %235 to i64
  %237 = add nuw nsw i64 %217, %236
  br label %238

238:                                              ; preds = %226, %213, %208
  %239 = phi i64 [ %212, %208 ], [ %237, %226 ], [ %225, %213 ]
  %240 = icmp ult i64 %239, 16777216
  br i1 %240, label %245, label %241

241:                                              ; preds = %238
  %242 = add nsw i32 %202, 128
  %243 = sext i32 %242 to i64
  %244 = shl nsw i64 %243, 23
  br label %252

245:                                              ; preds = %238
  %246 = icmp ult i64 %239, 8388608
  %247 = sext i32 %202 to i64
  %248 = shl nsw i64 %247, 23
  %249 = add nsw i64 %248, 1056964608
  %250 = select i1 %246, i64 0, i64 %249
  %251 = add i64 %250, %239
  br label %252

252:                                              ; preds = %245, %241, %198, %190, %185
  %253 = phi i64 [ 2139095040, %185 ], [ 0, %190 ], [ 1099511627776, %198 ], [ %244, %241 ], [ %251, %245 ]
  %254 = tail call i64 @llvm.smin.i64(i64 %253, i64 2139095040)
  %255 = trunc i64 %254 to i32
  %256 = or i32 %188, %255
  br label %257

257:                                              ; preds = %252, %179
  %258 = phi i32 [ %256, %252 ], [ 2147483647, %179 ]
  %259 = lshr i32 %258, 23
  %260 = and i32 %258, 8388607
  %261 = trunc i32 %259 to i8
  switch i8 %261, label %265 [
    i8 -1, label %262
    i8 0, label %269
  ]

262:                                              ; preds = %257
  %263 = icmp ne i32 %260, 0
  %264 = icmp eq i32 %260, 0
  br label %269

265:                                              ; preds = %257
  %266 = and i32 %259, 255
  %267 = or disjoint i32 %260, 8388608
  %268 = add nuw nsw i32 %266, 873
  br label %269

269:                                              ; preds = %265, %262, %257
  %270 = phi i32 [ 1023, %262 ], [ %268, %265 ], [ 874, %257 ]
  %271 = phi i32 [ 0, %262 ], [ %267, %265 ], [ %260, %257 ]
  %272 = phi i1 [ %264, %262 ], [ false, %265 ], [ false, %257 ]
  %273 = phi i1 [ %263, %262 ], [ false, %265 ], [ false, %257 ]
  %274 = zext nneg i32 %271 to i64
  %275 = or i1 %272, %273
  %276 = select i1 %273, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %275, label %295, label %277

277:                                              ; preds = %269
  %278 = icmp eq i32 %271, 0
  br i1 %278, label %295, label %279

279:                                              ; preds = %277, %279
  %280 = phi i32 [ %283, %279 ], [ 0, %277 ]
  %281 = phi i64 [ %282, %279 ], [ %274, %277 ]
  %282 = lshr i64 %281, 1
  %283 = add nuw nsw i32 %280, 1
  %284 = icmp ult i64 %281, 2
  br i1 %284, label %285, label %279, !llvm.loop !20003

285:                                              ; preds = %279
  %286 = add i32 %280, %270
  %287 = zext i32 %286 to i64
  %288 = shl i64 %287, 52
  %289 = sub nsw i32 52, %280
  %290 = zext nneg i32 %289 to i64
  %291 = shl i64 %274, %290
  %292 = and i64 %291, 4503599627370495
  %293 = or disjoint i64 %292, %288
  %294 = bitcast i64 %293 to double
  br label %295

295:                                              ; preds = %269, %277, %285
  %296 = phi double [ %294, %285 ], [ %276, %269 ], [ 0.000000e+00, %277 ]
  %297 = tail call double @llvm.fabs.f64(double %296)
  %298 = fneg double %297
  %299 = icmp slt i32 %258, 0
  %300 = select i1 %299, double %298, double %296
  %301 = bitcast double %300 to i64
  %302 = lshr i64 %301, 52
  %303 = trunc i64 %302 to i32
  %304 = and i32 %303, 2047
  %305 = and i64 %301, 4503599627370495
  switch i32 %304, label %311 [
    i32 2047, label %306
    i32 0, label %314
  ]

306:                                              ; preds = %295
  %307 = icmp ne i64 %305, 0
  %308 = zext i1 %307 to i32
  %309 = icmp eq i64 %305, 0
  %310 = zext i1 %309 to i32
  br label %314

311:                                              ; preds = %295
  %312 = or disjoint i64 %305, 4503599627370496
  %313 = add nsw i32 %304, -1075
  br label %314

314:                                              ; preds = %311, %306, %295
  %315 = phi i32 [ 0, %306 ], [ %313, %311 ], [ -1074, %295 ]
  %316 = phi i64 [ 0, %306 ], [ %312, %311 ], [ %305, %295 ]
  %317 = phi i32 [ %310, %306 ], [ 0, %311 ], [ %304, %295 ]
  %318 = phi i32 [ %308, %306 ], [ 0, %311 ], [ %304, %295 ]
  %319 = icmp eq i32 %318, 0
  br i1 %319, label %320, label %390

320:                                              ; preds = %314
  %321 = lshr i64 %301, 32
  %322 = and i64 %321, 2147483648
  %323 = icmp eq i32 %317, 0
  br i1 %323, label %324, label %386

324:                                              ; preds = %320
  %325 = icmp eq i64 %316, 0
  br i1 %325, label %386, label %326

326:                                              ; preds = %324, %326
  %327 = phi i32 [ %330, %326 ], [ 0, %324 ]
  %328 = phi i64 [ %329, %326 ], [ %316, %324 ]
  %329 = lshr i64 %328, 1
  %330 = add nuw nsw i32 %327, 1
  %331 = icmp ult i64 %328, 2
  br i1 %331, label %332, label %326, !llvm.loop !20003

332:                                              ; preds = %326
  %333 = add nsw i32 %330, %315
  %334 = add i32 %327, %315
  %335 = icmp sgt i32 %333, -126
  %336 = select i1 %335, i32 %334, i32 -126
  %337 = icmp sgt i32 %336, 200
  br i1 %337, label %386, label %338

338:                                              ; preds = %332
  %339 = sub i32 %315, %336
  %340 = add i32 %339, 23
  %341 = icmp sgt i32 %340, -1
  br i1 %341, label %342, label %347

342:                                              ; preds = %338
  %343 = icmp ugt i32 %340, 40
  %344 = zext nneg i32 %340 to i64
  %345 = shl i64 %316, %344
  %346 = select i1 %343, i64 4611686018427387904, i64 %345
  br label %372

347:                                              ; preds = %338
  %348 = sub nuw i32 -23, %339
  %349 = icmp ult i32 %340, -63
  %350 = zext nneg i32 %348 to i64
  %351 = lshr i64 %316, %350
  %352 = select i1 %349, i64 0, i64 %351
  %353 = shl nsw i64 -1, %350
  %354 = xor i64 %353, -1
  %355 = select i1 %349, i64 -1, i64 %354
  %356 = and i64 %355, %316
  %357 = icmp eq i64 %356, 0
  %358 = or i1 %349, %357
  %359 = select i1 %357, i64 %352, i64 0
  br i1 %358, label %372, label %360

360:                                              ; preds = %347
  %361 = sub nuw nsw i32 -24, %339
  %362 = zext nneg i32 %361 to i64
  %363 = shl nuw nsw i64 1, %362
  %364 = icmp ugt i64 %356, %363
  %365 = icmp eq i64 %356, %363
  %366 = and i64 %351, 1
  %367 = icmp ne i64 %366, 0
  %368 = select i1 %365, i1 %367, i1 false
  %369 = select i1 %364, i1 true, i1 %368
  %370 = zext i1 %369 to i64
  %371 = add nuw nsw i64 %351, %370
  br label %372

372:                                              ; preds = %360, %347, %342
  %373 = phi i64 [ %346, %342 ], [ %371, %360 ], [ %359, %347 ]
  %374 = icmp ult i64 %373, 16777216
  br i1 %374, label %379, label %375

375:                                              ; preds = %372
  %376 = add nsw i32 %336, 128
  %377 = sext i32 %376 to i64
  %378 = shl nsw i64 %377, 23
  br label %386

379:                                              ; preds = %372
  %380 = icmp ult i64 %373, 8388608
  %381 = sext i32 %336 to i64
  %382 = shl nsw i64 %381, 23
  %383 = add nsw i64 %382, 1056964608
  %384 = select i1 %380, i64 0, i64 %383
  %385 = add i64 %384, %373
  br label %386

386:                                              ; preds = %379, %375, %332, %324, %320
  %387 = phi i64 [ 2139095040, %320 ], [ 0, %324 ], [ 1099511627776, %332 ], [ %378, %375 ], [ %385, %379 ]
  %388 = tail call i64 @llvm.smin.i64(i64 %387, i64 2139095040)
  %389 = or i64 %388, %322
  br label %390

390:                                              ; preds = %386, %314
  %391 = phi i64 [ %389, %386 ], [ 2147483647, %314 ]
  %392 = trunc i64 %391 to i32
  %393 = and i32 %392, 2139095040
  %394 = icmp eq i32 %393, 0
  %395 = and i32 %392, -2147483648
  %396 = select i1 %394, i32 %395, i32 %392
  br label %401

397:                                              ; preds = %81
  %398 = zext i32 %84 to i64
  %399 = tail call fastcc i64 @__zluda_agx_cvt_mm_combine(i32 noundef %4, i32 noundef %6, i64 noundef %398, i64 noundef %29)
  %400 = trunc i64 %399 to i32
  br label %401

401:                                              ; preds = %397, %390
  %402 = phi i32 [ %396, %390 ], [ %400, %397 ]
  %403 = and i32 %82, %80
  %404 = shl i32 %402, %76
  %405 = and i32 %404, %78
  %406 = or i32 %405, %403
  %407 = cmpxchg ptr addrspace(1) %73, i32 %82, i32 %406 monotonic monotonic, align 4
  %408 = extractvalue { i32, i1 } %407, 1
  %409 = extractvalue { i32, i1 } %407, 0
  br i1 %408, label %410, label %81

410:                                              ; preds = %401, %65
  %411 = add i32 %45, 1
  br label %44, !llvm.loop !20033
}

; Function Attrs: nofree norecurse nounwind memory(readwrite, inaccessiblemem: none)
define dso_local void @__zluda_ptx_impl_agx_cp_reduce_bulk_h16_global(ptr addrspace(1) noundef %0, ptr addrspace(3) nocapture noundef readonly %1, i32 noundef %2, i32 noundef %3) local_unnamed_addr #3 {
  %5 = lshr i32 %2, 1
  %6 = icmp ult i32 %2, 2
  br i1 %6, label %25, label %7

7:                                                ; preds = %4
  %8 = and i32 %3, 255
  %9 = icmp eq i32 %8, 10
  %10 = icmp eq i32 %8, 11
  %11 = and i32 %3, 256
  %12 = icmp eq i32 %11, 0
  %13 = select i1 %12, i32 5, i32 8
  %14 = select i1 %12, i32 10, i32 7
  %15 = shl nsw i32 -1, %13
  %16 = xor i32 %15, -1
  %17 = shl nuw nsw i32 %16, %14
  %18 = add nsw i32 %13, -1
  %19 = shl nsw i32 -1, %18
  %20 = xor i32 %19, -1
  %21 = add nuw nsw i32 %19, 2
  %22 = zext nneg i32 %14 to i64
  %23 = shl nuw nsw i64 2, %22
  %24 = zext nneg i32 %17 to i64
  br label %26

25:                                               ; preds = %345, %4
  ret void

26:                                               ; preds = %7, %345
  %27 = phi i32 [ 0, %7 ], [ %346, %345 ]
  %28 = zext nneg i32 %27 to i64
  %29 = getelementptr inbounds i16, ptr addrspace(1) %0, i64 %28
  %30 = ptrtoint ptr addrspace(1) %29 to i64
  %31 = and i64 %30, -4
  %32 = inttoptr i64 %31 to ptr addrspace(1)
  %33 = trunc i64 %30 to i32
  %34 = shl i32 %33, 3
  %35 = and i32 %34, 24
  %36 = getelementptr inbounds i16, ptr addrspace(3) %1, i64 %28
  %37 = load i16, ptr addrspace(3) %36, align 2
  %38 = zext i16 %37 to i32
  %39 = load i32, ptr addrspace(1) %32, align 4
  %40 = shl i32 65535, %35
  %41 = xor i32 %40, -1
  %42 = shl nuw i32 %38, 16
  %43 = lshr i32 %38, 7
  %44 = and i32 %42, 8323072
  %45 = trunc i32 %43 to i8
  %46 = icmp ne i32 %44, 0
  %47 = icmp eq i32 %44, 0
  %48 = and i32 %43, 255
  %49 = or disjoint i32 %44, 8388608
  %50 = add nuw nsw i32 %48, 873
  %51 = icmp slt i32 %42, 0
  %52 = lshr i32 %38, 10
  %53 = and i32 %52, 31
  %54 = and i32 %38, 1023
  %55 = icmp ne i32 %54, 0
  %56 = icmp eq i32 %54, 0
  %57 = or disjoint i32 %54, 1024
  %58 = add nuw nsw i32 %53, 998
  %59 = icmp slt i16 %37, 0
  br label %60

60:                                               ; preds = %337, %26
  %61 = phi i32 [ %39, %26 ], [ %344, %337 ]
  %62 = and i32 %61, %41
  %63 = lshr i32 %61, %35
  br i1 %12, label %64, label %107

64:                                               ; preds = %60
  %65 = lshr i32 %63, 10
  %66 = and i32 %65, 31
  %67 = and i32 %63, 1023
  switch i32 %66, label %71 [
    i32 31, label %68
    i32 0, label %74
  ]

68:                                               ; preds = %64
  %69 = icmp ne i32 %67, 0
  %70 = icmp eq i32 %67, 0
  br label %74

71:                                               ; preds = %64
  %72 = or disjoint i32 %67, 1024
  %73 = add nuw nsw i32 %66, 998
  br label %74

74:                                               ; preds = %71, %68, %64
  %75 = phi i32 [ 1023, %68 ], [ %73, %71 ], [ 999, %64 ]
  %76 = phi i32 [ 0, %68 ], [ %72, %71 ], [ %67, %64 ]
  %77 = phi i1 [ %70, %68 ], [ false, %71 ], [ false, %64 ]
  %78 = phi i1 [ %69, %68 ], [ false, %71 ], [ false, %64 ]
  %79 = zext nneg i32 %76 to i64
  %80 = or i1 %77, %78
  %81 = select i1 %78, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %80, label %100, label %82

82:                                               ; preds = %74
  %83 = icmp eq i32 %76, 0
  br i1 %83, label %100, label %84

84:                                               ; preds = %82, %84
  %85 = phi i32 [ %88, %84 ], [ 0, %82 ]
  %86 = phi i64 [ %87, %84 ], [ %79, %82 ]
  %87 = lshr i64 %86, 1
  %88 = add nuw nsw i32 %85, 1
  %89 = icmp ult i64 %86, 2
  br i1 %89, label %90, label %84, !llvm.loop !20003

90:                                               ; preds = %84
  %91 = add i32 %85, %75
  %92 = zext i32 %91 to i64
  %93 = shl i64 %92, 52
  %94 = sub nsw i32 52, %85
  %95 = zext nneg i32 %94 to i64
  %96 = shl i64 %79, %95
  %97 = and i64 %96, 4503599627370495
  %98 = or disjoint i64 %97, %93
  %99 = bitcast i64 %98 to double
  br label %100

100:                                              ; preds = %90, %82, %74
  %101 = phi double [ %99, %90 ], [ %81, %74 ], [ 0.000000e+00, %82 ]
  %102 = and i32 %63, 32768
  %103 = icmp eq i32 %102, 0
  %104 = tail call double @llvm.fabs.f64(double %101)
  %105 = fneg double %104
  %106 = select i1 %103, double %101, double %105
  br label %152

107:                                              ; preds = %60
  %108 = and i32 %63, 65535
  %109 = shl nuw i32 %108, 16
  %110 = lshr i32 %108, 7
  %111 = and i32 %109, 8323072
  %112 = trunc i32 %110 to i8
  switch i8 %112, label %116 [
    i8 -1, label %113
    i8 0, label %120
  ]

113:                                              ; preds = %107
  %114 = icmp ne i32 %111, 0
  %115 = icmp eq i32 %111, 0
  br label %120

116:                                              ; preds = %107
  %117 = and i32 %110, 255
  %118 = or disjoint i32 %111, 8388608
  %119 = add nuw nsw i32 %117, 873
  br label %120

120:                                              ; preds = %116, %113, %107
  %121 = phi i32 [ 1023, %113 ], [ %119, %116 ], [ 874, %107 ]
  %122 = phi i32 [ 0, %113 ], [ %118, %116 ], [ %111, %107 ]
  %123 = phi i1 [ %115, %113 ], [ false, %116 ], [ false, %107 ]
  %124 = phi i1 [ %114, %113 ], [ false, %116 ], [ false, %107 ]
  %125 = zext nneg i32 %122 to i64
  %126 = or i1 %123, %124
  %127 = select i1 %124, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %126, label %146, label %128

128:                                              ; preds = %120
  %129 = icmp eq i32 %122, 0
  br i1 %129, label %146, label %130

130:                                              ; preds = %128, %130
  %131 = phi i32 [ %134, %130 ], [ 0, %128 ]
  %132 = phi i64 [ %133, %130 ], [ %125, %128 ]
  %133 = lshr i64 %132, 1
  %134 = add nuw nsw i32 %131, 1
  %135 = icmp ult i64 %132, 2
  br i1 %135, label %136, label %130, !llvm.loop !20003

136:                                              ; preds = %130
  %137 = add i32 %131, %121
  %138 = zext i32 %137 to i64
  %139 = shl i64 %138, 52
  %140 = sub nsw i32 52, %131
  %141 = zext nneg i32 %140 to i64
  %142 = shl i64 %125, %141
  %143 = and i64 %142, 4503599627370495
  %144 = or disjoint i64 %143, %139
  %145 = bitcast i64 %144 to double
  br label %146

146:                                              ; preds = %136, %128, %120
  %147 = phi double [ %145, %136 ], [ %127, %120 ], [ 0.000000e+00, %128 ]
  %148 = tail call double @llvm.fabs.f64(double %147)
  %149 = fneg double %148
  %150 = icmp slt i32 %109, 0
  %151 = select i1 %150, double %149, double %147
  br label %152

152:                                              ; preds = %146, %100
  %153 = phi double [ %151, %146 ], [ %106, %100 ]
  br i1 %12, label %154, label %188

154:                                              ; preds = %152
  switch i32 %53, label %156 [
    i32 31, label %155
    i32 0, label %157
  ]

155:                                              ; preds = %154
  br label %157

156:                                              ; preds = %154
  br label %157

157:                                              ; preds = %156, %155, %154
  %158 = phi i32 [ 1023, %155 ], [ %58, %156 ], [ 999, %154 ]
  %159 = phi i32 [ 0, %155 ], [ %57, %156 ], [ %54, %154 ]
  %160 = phi i1 [ %56, %155 ], [ false, %156 ], [ false, %154 ]
  %161 = phi i1 [ %55, %155 ], [ false, %156 ], [ false, %154 ]
  %162 = zext nneg i32 %159 to i64
  %163 = or i1 %160, %161
  %164 = select i1 %161, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %163, label %183, label %165

165:                                              ; preds = %157
  %166 = icmp eq i32 %159, 0
  br i1 %166, label %183, label %167

167:                                              ; preds = %165, %167
  %168 = phi i32 [ %171, %167 ], [ 0, %165 ]
  %169 = phi i64 [ %170, %167 ], [ %162, %165 ]
  %170 = lshr i64 %169, 1
  %171 = add nuw nsw i32 %168, 1
  %172 = icmp ult i64 %169, 2
  br i1 %172, label %173, label %167, !llvm.loop !20003

173:                                              ; preds = %167
  %174 = add i32 %168, %158
  %175 = zext i32 %174 to i64
  %176 = shl i64 %175, 52
  %177 = sub nsw i32 52, %168
  %178 = zext nneg i32 %177 to i64
  %179 = shl i64 %162, %178
  %180 = and i64 %179, 4503599627370495
  %181 = or disjoint i64 %180, %176
  %182 = bitcast i64 %181 to double
  br label %183

183:                                              ; preds = %173, %165, %157
  %184 = phi double [ %182, %173 ], [ %164, %157 ], [ 0.000000e+00, %165 ]
  %185 = tail call double @llvm.fabs.f64(double %184)
  %186 = fneg double %185
  %187 = select i1 %59, double %186, double %184
  br label %222

188:                                              ; preds = %152
  switch i8 %45, label %190 [
    i8 -1, label %189
    i8 0, label %191
  ]

189:                                              ; preds = %188
  br label %191

190:                                              ; preds = %188
  br label %191

191:                                              ; preds = %190, %189, %188
  %192 = phi i32 [ 1023, %189 ], [ %50, %190 ], [ 874, %188 ]
  %193 = phi i32 [ 0, %189 ], [ %49, %190 ], [ %44, %188 ]
  %194 = phi i1 [ %47, %189 ], [ false, %190 ], [ false, %188 ]
  %195 = phi i1 [ %46, %189 ], [ false, %190 ], [ false, %188 ]
  %196 = zext nneg i32 %193 to i64
  %197 = or i1 %194, %195
  %198 = select i1 %195, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %197, label %217, label %199

199:                                              ; preds = %191
  %200 = icmp eq i32 %193, 0
  br i1 %200, label %217, label %201

201:                                              ; preds = %199, %201
  %202 = phi i32 [ %205, %201 ], [ 0, %199 ]
  %203 = phi i64 [ %204, %201 ], [ %196, %199 ]
  %204 = lshr i64 %203, 1
  %205 = add nuw nsw i32 %202, 1
  %206 = icmp ult i64 %203, 2
  br i1 %206, label %207, label %201, !llvm.loop !20003

207:                                              ; preds = %201
  %208 = add i32 %202, %192
  %209 = zext i32 %208 to i64
  %210 = shl i64 %209, 52
  %211 = sub nsw i32 52, %202
  %212 = zext nneg i32 %211 to i64
  %213 = shl i64 %196, %212
  %214 = and i64 %213, 4503599627370495
  %215 = or disjoint i64 %214, %210
  %216 = bitcast i64 %215 to double
  br label %217

217:                                              ; preds = %207, %199, %191
  %218 = phi double [ %216, %207 ], [ %198, %191 ], [ 0.000000e+00, %199 ]
  %219 = tail call double @llvm.fabs.f64(double %218)
  %220 = fneg double %219
  %221 = select i1 %51, double %220, double %218
  br label %222

222:                                              ; preds = %217, %183
  %223 = phi double [ %221, %217 ], [ %187, %183 ]
  br i1 %9, label %224, label %226

224:                                              ; preds = %222
  %225 = fadd double %153, %223
  br label %243

226:                                              ; preds = %222
  %227 = fcmp uno double %153, 0.000000e+00
  br i1 %227, label %243, label %228

228:                                              ; preds = %226
  %229 = fcmp uno double %223, 0.000000e+00
  br i1 %229, label %243, label %230

230:                                              ; preds = %228
  %231 = fcmp oeq double %153, %223
  br i1 %231, label %232, label %239

232:                                              ; preds = %230
  %233 = bitcast double %153 to i64
  %234 = bitcast double %223 to i64
  %235 = or i64 %234, %233
  %236 = and i64 %234, %233
  %237 = select i1 %10, i64 %235, i64 %236
  %238 = bitcast i64 %237 to double
  br label %243

239:                                              ; preds = %230
  %240 = fcmp uge double %153, %223
  %241 = xor i1 %10, %240
  %242 = select i1 %241, double %153, double %223
  br label %243

243:                                              ; preds = %239, %232, %228, %226, %224
  %244 = phi double [ %225, %224 ], [ %238, %232 ], [ %242, %239 ], [ %223, %226 ], [ %153, %228 ]
  %245 = bitcast double %244 to i64
  %246 = lshr i64 %245, 52
  %247 = trunc i64 %246 to i32
  %248 = and i32 %247, 2047
  %249 = and i64 %245, 4503599627370495
  switch i32 %248, label %253 [
    i32 2047, label %250
    i32 0, label %256
  ]

250:                                              ; preds = %243
  %251 = icmp eq i64 %249, 0
  %252 = icmp ne i64 %249, 0
  br label %256

253:                                              ; preds = %243
  %254 = or disjoint i64 %249, 4503599627370496
  %255 = add nsw i32 %248, -1075
  br label %256

256:                                              ; preds = %253, %250, %243
  %257 = phi i32 [ 0, %250 ], [ %255, %253 ], [ -1074, %243 ]
  %258 = phi i64 [ 0, %250 ], [ %254, %253 ], [ %249, %243 ]
  %259 = phi i1 [ %252, %250 ], [ true, %253 ], [ true, %243 ]
  %260 = phi i1 [ %251, %250 ], [ true, %253 ], [ true, %243 ]
  br i1 %260, label %261, label %337

261:                                              ; preds = %256
  %262 = lshr i64 %245, 48
  %263 = trunc i64 %262 to i32
  %264 = and i32 %263, 32768
  br i1 %259, label %267, label %265

265:                                              ; preds = %261
  %266 = or i32 %264, %17
  br label %337

267:                                              ; preds = %261
  %268 = icmp eq i64 %258, 0
  br i1 %268, label %337, label %269

269:                                              ; preds = %267, %269
  %270 = phi i32 [ %273, %269 ], [ 0, %267 ]
  %271 = phi i64 [ %272, %269 ], [ %258, %267 ]
  %272 = lshr i64 %271, 1
  %273 = add nuw nsw i32 %270, 1
  %274 = icmp ult i64 %271, 2
  br i1 %274, label %275, label %269, !llvm.loop !20003

275:                                              ; preds = %269
  %276 = add nsw i32 %273, %257
  %277 = add i32 %270, %257
  %278 = icmp sgt i32 %276, %21
  %279 = select i1 %278, i32 %277, i32 %21
  %280 = icmp sgt i32 %279, 200
  br i1 %280, label %331, label %281

281:                                              ; preds = %275
  %282 = sub i32 %14, %279
  %283 = add i32 %282, %257
  %284 = icmp sgt i32 %283, -1
  br i1 %284, label %285, label %290

285:                                              ; preds = %281
  %286 = icmp ugt i32 %283, 40
  %287 = zext nneg i32 %283 to i64
  %288 = shl i64 %258, %287
  %289 = select i1 %286, i64 4611686018427387904, i64 %288
  br label %315

290:                                              ; preds = %281
  %291 = sub nsw i32 0, %283
  %292 = icmp ult i32 %283, -63
  %293 = zext nneg i32 %291 to i64
  %294 = lshr i64 %258, %293
  %295 = select i1 %292, i64 0, i64 %294
  %296 = shl nsw i64 -1, %293
  %297 = xor i64 %296, -1
  %298 = select i1 %292, i64 -1, i64 %297
  %299 = and i64 %298, %258
  %300 = icmp eq i64 %299, 0
  %301 = or i1 %292, %300
  %302 = select i1 %300, i64 %295, i64 0
  br i1 %301, label %315, label %303

303:                                              ; preds = %290
  %304 = xor i32 %283, -1
  %305 = zext nneg i32 %304 to i64
  %306 = shl nuw nsw i64 1, %305
  %307 = icmp ugt i64 %299, %306
  %308 = icmp eq i64 %299, %306
  %309 = and i64 %294, 1
  %310 = icmp ne i64 %309, 0
  %311 = select i1 %308, i1 %310, i1 false
  %312 = select i1 %307, i1 true, i1 %311
  %313 = zext i1 %312 to i64
  %314 = add nuw nsw i64 %294, %313
  br label %315

315:                                              ; preds = %303, %290, %285
  %316 = phi i64 [ %289, %285 ], [ %314, %303 ], [ %302, %290 ]
  %317 = icmp ult i64 %316, %23
  br i1 %317, label %322, label %318

318:                                              ; preds = %315
  %319 = sub i32 %279, %19
  %320 = sext i32 %319 to i64
  %321 = shl nsw i64 %320, %22
  br label %331

322:                                              ; preds = %315
  %323 = lshr i64 %316, %22
  %324 = icmp eq i64 %323, 0
  %325 = add nsw i32 %279, %20
  %326 = sext i32 %325 to i64
  %327 = add nsw i64 %326, -1
  %328 = shl nsw i64 %327, %22
  %329 = select i1 %324, i64 0, i64 %328
  %330 = add i64 %329, %316
  br label %331

331:                                              ; preds = %322, %318, %275
  %332 = phi i64 [ 1099511627776, %275 ], [ %321, %318 ], [ %330, %322 ]
  %333 = icmp slt i64 %332, %24
  %334 = trunc i64 %332 to i32
  %335 = select i1 %333, i32 %334, i32 %17
  %336 = or i32 %335, %264
  br label %337

337:                                              ; preds = %256, %265, %267, %331
  %338 = phi i32 [ 32767, %256 ], [ %266, %265 ], [ %336, %331 ], [ %264, %267 ]
  %339 = and i32 %338, 65535
  %340 = shl i32 %339, %35
  %341 = or i32 %340, %62
  %342 = cmpxchg ptr addrspace(1) %32, i32 %61, i32 %341 monotonic monotonic, align 4
  %343 = extractvalue { i32, i1 } %342, 1
  %344 = extractvalue { i32, i1 } %342, 0
  br i1 %343, label %345, label %60, !llvm.loop !20034

345:                                              ; preds = %337
  %346 = add nuw nsw i32 %27, 1
  %347 = icmp eq i32 %346, %5
  br i1 %347, label %25, label %26, !llvm.loop !20035
}

; Function Attrs: nofree norecurse nounwind memory(readwrite, inaccessiblemem: none)
define dso_local void @__zluda_ptx_impl_agx_cp_reduce_bulk_h16_shared(ptr addrspace(3) noundef %0, ptr addrspace(3) nocapture noundef readonly %1, i32 noundef %2, i32 noundef %3) local_unnamed_addr #3 {
  %5 = lshr i32 %2, 1
  %6 = icmp ult i32 %2, 2
  br i1 %6, label %25, label %7

7:                                                ; preds = %4
  %8 = and i32 %3, 255
  %9 = icmp eq i32 %8, 10
  %10 = icmp eq i32 %8, 11
  %11 = and i32 %3, 256
  %12 = icmp eq i32 %11, 0
  %13 = select i1 %12, i32 5, i32 8
  %14 = select i1 %12, i32 10, i32 7
  %15 = shl nsw i32 -1, %13
  %16 = xor i32 %15, -1
  %17 = shl nuw nsw i32 %16, %14
  %18 = add nsw i32 %13, -1
  %19 = shl nsw i32 -1, %18
  %20 = xor i32 %19, -1
  %21 = add nuw nsw i32 %19, 2
  %22 = zext nneg i32 %14 to i64
  %23 = shl nuw nsw i64 2, %22
  %24 = zext nneg i32 %17 to i64
  br label %26

25:                                               ; preds = %345, %4
  ret void

26:                                               ; preds = %7, %345
  %27 = phi i32 [ 0, %7 ], [ %346, %345 ]
  %28 = zext nneg i32 %27 to i64
  %29 = getelementptr inbounds i16, ptr addrspace(3) %0, i64 %28
  %30 = ptrtoint ptr addrspace(3) %29 to i64
  %31 = trunc i64 %30 to i32
  %32 = and i64 %30, 4294967292
  %33 = inttoptr i64 %32 to ptr addrspace(3)
  %34 = shl i32 %31, 3
  %35 = and i32 %34, 24
  %36 = getelementptr inbounds i16, ptr addrspace(3) %1, i64 %28
  %37 = load i16, ptr addrspace(3) %36, align 2
  %38 = zext i16 %37 to i32
  %39 = load i32, ptr addrspace(3) %33, align 4
  %40 = shl i32 65535, %35
  %41 = xor i32 %40, -1
  %42 = shl nuw i32 %38, 16
  %43 = lshr i32 %38, 7
  %44 = and i32 %42, 8323072
  %45 = trunc i32 %43 to i8
  %46 = icmp ne i32 %44, 0
  %47 = icmp eq i32 %44, 0
  %48 = and i32 %43, 255
  %49 = or disjoint i32 %44, 8388608
  %50 = add nuw nsw i32 %48, 873
  %51 = icmp slt i32 %42, 0
  %52 = lshr i32 %38, 10
  %53 = and i32 %52, 31
  %54 = and i32 %38, 1023
  %55 = icmp ne i32 %54, 0
  %56 = icmp eq i32 %54, 0
  %57 = or disjoint i32 %54, 1024
  %58 = add nuw nsw i32 %53, 998
  %59 = icmp slt i16 %37, 0
  br label %60

60:                                               ; preds = %337, %26
  %61 = phi i32 [ %39, %26 ], [ %344, %337 ]
  %62 = and i32 %61, %41
  %63 = lshr i32 %61, %35
  br i1 %12, label %64, label %107

64:                                               ; preds = %60
  %65 = lshr i32 %63, 10
  %66 = and i32 %65, 31
  %67 = and i32 %63, 1023
  switch i32 %66, label %71 [
    i32 31, label %68
    i32 0, label %74
  ]

68:                                               ; preds = %64
  %69 = icmp ne i32 %67, 0
  %70 = icmp eq i32 %67, 0
  br label %74

71:                                               ; preds = %64
  %72 = or disjoint i32 %67, 1024
  %73 = add nuw nsw i32 %66, 998
  br label %74

74:                                               ; preds = %71, %68, %64
  %75 = phi i32 [ 1023, %68 ], [ %73, %71 ], [ 999, %64 ]
  %76 = phi i32 [ 0, %68 ], [ %72, %71 ], [ %67, %64 ]
  %77 = phi i1 [ %70, %68 ], [ false, %71 ], [ false, %64 ]
  %78 = phi i1 [ %69, %68 ], [ false, %71 ], [ false, %64 ]
  %79 = zext nneg i32 %76 to i64
  %80 = or i1 %77, %78
  %81 = select i1 %78, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %80, label %100, label %82

82:                                               ; preds = %74
  %83 = icmp eq i32 %76, 0
  br i1 %83, label %100, label %84

84:                                               ; preds = %82, %84
  %85 = phi i32 [ %88, %84 ], [ 0, %82 ]
  %86 = phi i64 [ %87, %84 ], [ %79, %82 ]
  %87 = lshr i64 %86, 1
  %88 = add nuw nsw i32 %85, 1
  %89 = icmp ult i64 %86, 2
  br i1 %89, label %90, label %84, !llvm.loop !20003

90:                                               ; preds = %84
  %91 = add i32 %85, %75
  %92 = zext i32 %91 to i64
  %93 = shl i64 %92, 52
  %94 = sub nsw i32 52, %85
  %95 = zext nneg i32 %94 to i64
  %96 = shl i64 %79, %95
  %97 = and i64 %96, 4503599627370495
  %98 = or disjoint i64 %97, %93
  %99 = bitcast i64 %98 to double
  br label %100

100:                                              ; preds = %90, %82, %74
  %101 = phi double [ %99, %90 ], [ %81, %74 ], [ 0.000000e+00, %82 ]
  %102 = and i32 %63, 32768
  %103 = icmp eq i32 %102, 0
  %104 = tail call double @llvm.fabs.f64(double %101)
  %105 = fneg double %104
  %106 = select i1 %103, double %101, double %105
  br label %152

107:                                              ; preds = %60
  %108 = and i32 %63, 65535
  %109 = shl nuw i32 %108, 16
  %110 = lshr i32 %108, 7
  %111 = and i32 %109, 8323072
  %112 = trunc i32 %110 to i8
  switch i8 %112, label %116 [
    i8 -1, label %113
    i8 0, label %120
  ]

113:                                              ; preds = %107
  %114 = icmp ne i32 %111, 0
  %115 = icmp eq i32 %111, 0
  br label %120

116:                                              ; preds = %107
  %117 = and i32 %110, 255
  %118 = or disjoint i32 %111, 8388608
  %119 = add nuw nsw i32 %117, 873
  br label %120

120:                                              ; preds = %116, %113, %107
  %121 = phi i32 [ 1023, %113 ], [ %119, %116 ], [ 874, %107 ]
  %122 = phi i32 [ 0, %113 ], [ %118, %116 ], [ %111, %107 ]
  %123 = phi i1 [ %115, %113 ], [ false, %116 ], [ false, %107 ]
  %124 = phi i1 [ %114, %113 ], [ false, %116 ], [ false, %107 ]
  %125 = zext nneg i32 %122 to i64
  %126 = or i1 %123, %124
  %127 = select i1 %124, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %126, label %146, label %128

128:                                              ; preds = %120
  %129 = icmp eq i32 %122, 0
  br i1 %129, label %146, label %130

130:                                              ; preds = %128, %130
  %131 = phi i32 [ %134, %130 ], [ 0, %128 ]
  %132 = phi i64 [ %133, %130 ], [ %125, %128 ]
  %133 = lshr i64 %132, 1
  %134 = add nuw nsw i32 %131, 1
  %135 = icmp ult i64 %132, 2
  br i1 %135, label %136, label %130, !llvm.loop !20003

136:                                              ; preds = %130
  %137 = add i32 %131, %121
  %138 = zext i32 %137 to i64
  %139 = shl i64 %138, 52
  %140 = sub nsw i32 52, %131
  %141 = zext nneg i32 %140 to i64
  %142 = shl i64 %125, %141
  %143 = and i64 %142, 4503599627370495
  %144 = or disjoint i64 %143, %139
  %145 = bitcast i64 %144 to double
  br label %146

146:                                              ; preds = %136, %128, %120
  %147 = phi double [ %145, %136 ], [ %127, %120 ], [ 0.000000e+00, %128 ]
  %148 = tail call double @llvm.fabs.f64(double %147)
  %149 = fneg double %148
  %150 = icmp slt i32 %109, 0
  %151 = select i1 %150, double %149, double %147
  br label %152

152:                                              ; preds = %146, %100
  %153 = phi double [ %151, %146 ], [ %106, %100 ]
  br i1 %12, label %154, label %188

154:                                              ; preds = %152
  switch i32 %53, label %156 [
    i32 31, label %155
    i32 0, label %157
  ]

155:                                              ; preds = %154
  br label %157

156:                                              ; preds = %154
  br label %157

157:                                              ; preds = %156, %155, %154
  %158 = phi i32 [ 1023, %155 ], [ %58, %156 ], [ 999, %154 ]
  %159 = phi i32 [ 0, %155 ], [ %57, %156 ], [ %54, %154 ]
  %160 = phi i1 [ %56, %155 ], [ false, %156 ], [ false, %154 ]
  %161 = phi i1 [ %55, %155 ], [ false, %156 ], [ false, %154 ]
  %162 = zext nneg i32 %159 to i64
  %163 = or i1 %160, %161
  %164 = select i1 %161, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %163, label %183, label %165

165:                                              ; preds = %157
  %166 = icmp eq i32 %159, 0
  br i1 %166, label %183, label %167

167:                                              ; preds = %165, %167
  %168 = phi i32 [ %171, %167 ], [ 0, %165 ]
  %169 = phi i64 [ %170, %167 ], [ %162, %165 ]
  %170 = lshr i64 %169, 1
  %171 = add nuw nsw i32 %168, 1
  %172 = icmp ult i64 %169, 2
  br i1 %172, label %173, label %167, !llvm.loop !20003

173:                                              ; preds = %167
  %174 = add i32 %168, %158
  %175 = zext i32 %174 to i64
  %176 = shl i64 %175, 52
  %177 = sub nsw i32 52, %168
  %178 = zext nneg i32 %177 to i64
  %179 = shl i64 %162, %178
  %180 = and i64 %179, 4503599627370495
  %181 = or disjoint i64 %180, %176
  %182 = bitcast i64 %181 to double
  br label %183

183:                                              ; preds = %173, %165, %157
  %184 = phi double [ %182, %173 ], [ %164, %157 ], [ 0.000000e+00, %165 ]
  %185 = tail call double @llvm.fabs.f64(double %184)
  %186 = fneg double %185
  %187 = select i1 %59, double %186, double %184
  br label %222

188:                                              ; preds = %152
  switch i8 %45, label %190 [
    i8 -1, label %189
    i8 0, label %191
  ]

189:                                              ; preds = %188
  br label %191

190:                                              ; preds = %188
  br label %191

191:                                              ; preds = %190, %189, %188
  %192 = phi i32 [ 1023, %189 ], [ %50, %190 ], [ 874, %188 ]
  %193 = phi i32 [ 0, %189 ], [ %49, %190 ], [ %44, %188 ]
  %194 = phi i1 [ %47, %189 ], [ false, %190 ], [ false, %188 ]
  %195 = phi i1 [ %46, %189 ], [ false, %190 ], [ false, %188 ]
  %196 = zext nneg i32 %193 to i64
  %197 = or i1 %194, %195
  %198 = select i1 %195, double 0x7FF8000000000000, double 0x7FF0000000000000
  br i1 %197, label %217, label %199

199:                                              ; preds = %191
  %200 = icmp eq i32 %193, 0
  br i1 %200, label %217, label %201

201:                                              ; preds = %199, %201
  %202 = phi i32 [ %205, %201 ], [ 0, %199 ]
  %203 = phi i64 [ %204, %201 ], [ %196, %199 ]
  %204 = lshr i64 %203, 1
  %205 = add nuw nsw i32 %202, 1
  %206 = icmp ult i64 %203, 2
  br i1 %206, label %207, label %201, !llvm.loop !20003

207:                                              ; preds = %201
  %208 = add i32 %202, %192
  %209 = zext i32 %208 to i64
  %210 = shl i64 %209, 52
  %211 = sub nsw i32 52, %202
  %212 = zext nneg i32 %211 to i64
  %213 = shl i64 %196, %212
  %214 = and i64 %213, 4503599627370495
  %215 = or disjoint i64 %214, %210
  %216 = bitcast i64 %215 to double
  br label %217

217:                                              ; preds = %207, %199, %191
  %218 = phi double [ %216, %207 ], [ %198, %191 ], [ 0.000000e+00, %199 ]
  %219 = tail call double @llvm.fabs.f64(double %218)
  %220 = fneg double %219
  %221 = select i1 %51, double %220, double %218
  br label %222

222:                                              ; preds = %217, %183
  %223 = phi double [ %221, %217 ], [ %187, %183 ]
  br i1 %9, label %224, label %226

224:                                              ; preds = %222
  %225 = fadd double %153, %223
  br label %243

226:                                              ; preds = %222
  %227 = fcmp uno double %153, 0.000000e+00
  br i1 %227, label %243, label %228

228:                                              ; preds = %226
  %229 = fcmp uno double %223, 0.000000e+00
  br i1 %229, label %243, label %230

230:                                              ; preds = %228
  %231 = fcmp oeq double %153, %223
  br i1 %231, label %232, label %239

232:                                              ; preds = %230
  %233 = bitcast double %153 to i64
  %234 = bitcast double %223 to i64
  %235 = or i64 %234, %233
  %236 = and i64 %234, %233
  %237 = select i1 %10, i64 %235, i64 %236
  %238 = bitcast i64 %237 to double
  br label %243

239:                                              ; preds = %230
  %240 = fcmp uge double %153, %223
  %241 = xor i1 %10, %240
  %242 = select i1 %241, double %153, double %223
  br label %243

243:                                              ; preds = %239, %232, %228, %226, %224
  %244 = phi double [ %225, %224 ], [ %238, %232 ], [ %242, %239 ], [ %223, %226 ], [ %153, %228 ]
  %245 = bitcast double %244 to i64
  %246 = lshr i64 %245, 52
  %247 = trunc i64 %246 to i32
  %248 = and i32 %247, 2047
  %249 = and i64 %245, 4503599627370495
  switch i32 %248, label %253 [
    i32 2047, label %250
    i32 0, label %256
  ]

250:                                              ; preds = %243
  %251 = icmp eq i64 %249, 0
  %252 = icmp ne i64 %249, 0
  br label %256

253:                                              ; preds = %243
  %254 = or disjoint i64 %249, 4503599627370496
  %255 = add nsw i32 %248, -1075
  br label %256

256:                                              ; preds = %253, %250, %243
  %257 = phi i32 [ 0, %250 ], [ %255, %253 ], [ -1074, %243 ]
  %258 = phi i64 [ 0, %250 ], [ %254, %253 ], [ %249, %243 ]
  %259 = phi i1 [ %252, %250 ], [ true, %253 ], [ true, %243 ]
  %260 = phi i1 [ %251, %250 ], [ true, %253 ], [ true, %243 ]
  br i1 %260, label %261, label %337

261:                                              ; preds = %256
  %262 = lshr i64 %245, 48
  %263 = trunc i64 %262 to i32
  %264 = and i32 %263, 32768
  br i1 %259, label %267, label %265

265:                                              ; preds = %261
  %266 = or i32 %264, %17
  br label %337

267:                                              ; preds = %261
  %268 = icmp eq i64 %258, 0
  br i1 %268, label %337, label %269

269:                                              ; preds = %267, %269
  %270 = phi i32 [ %273, %269 ], [ 0, %267 ]
  %271 = phi i64 [ %272, %269 ], [ %258, %267 ]
  %272 = lshr i64 %271, 1
  %273 = add nuw nsw i32 %270, 1
  %274 = icmp ult i64 %271, 2
  br i1 %274, label %275, label %269, !llvm.loop !20003

275:                                              ; preds = %269
  %276 = add nsw i32 %273, %257
  %277 = add i32 %270, %257
  %278 = icmp sgt i32 %276, %21
  %279 = select i1 %278, i32 %277, i32 %21
  %280 = icmp sgt i32 %279, 200
  br i1 %280, label %331, label %281

281:                                              ; preds = %275
  %282 = sub i32 %14, %279
  %283 = add i32 %282, %257
  %284 = icmp sgt i32 %283, -1
  br i1 %284, label %285, label %290

285:                                              ; preds = %281
  %286 = icmp ugt i32 %283, 40
  %287 = zext nneg i32 %283 to i64
  %288 = shl i64 %258, %287
  %289 = select i1 %286, i64 4611686018427387904, i64 %288
  br label %315

290:                                              ; preds = %281
  %291 = sub nsw i32 0, %283
  %292 = icmp ult i32 %283, -63
  %293 = zext nneg i32 %291 to i64
  %294 = lshr i64 %258, %293
  %295 = select i1 %292, i64 0, i64 %294
  %296 = shl nsw i64 -1, %293
  %297 = xor i64 %296, -1
  %298 = select i1 %292, i64 -1, i64 %297
  %299 = and i64 %298, %258
  %300 = icmp eq i64 %299, 0
  %301 = or i1 %292, %300
  %302 = select i1 %300, i64 %295, i64 0
  br i1 %301, label %315, label %303

303:                                              ; preds = %290
  %304 = xor i32 %283, -1
  %305 = zext nneg i32 %304 to i64
  %306 = shl nuw nsw i64 1, %305
  %307 = icmp ugt i64 %299, %306
  %308 = icmp eq i64 %299, %306
  %309 = and i64 %294, 1
  %310 = icmp ne i64 %309, 0
  %311 = select i1 %308, i1 %310, i1 false
  %312 = select i1 %307, i1 true, i1 %311
  %313 = zext i1 %312 to i64
  %314 = add nuw nsw i64 %294, %313
  br label %315

315:                                              ; preds = %303, %290, %285
  %316 = phi i64 [ %289, %285 ], [ %314, %303 ], [ %302, %290 ]
  %317 = icmp ult i64 %316, %23
  br i1 %317, label %322, label %318

318:                                              ; preds = %315
  %319 = sub i32 %279, %19
  %320 = sext i32 %319 to i64
  %321 = shl nsw i64 %320, %22
  br label %331

322:                                              ; preds = %315
  %323 = lshr i64 %316, %22
  %324 = icmp eq i64 %323, 0
  %325 = add nsw i32 %279, %20
  %326 = sext i32 %325 to i64
  %327 = add nsw i64 %326, -1
  %328 = shl nsw i64 %327, %22
  %329 = select i1 %324, i64 0, i64 %328
  %330 = add i64 %329, %316
  br label %331

331:                                              ; preds = %322, %318, %275
  %332 = phi i64 [ 1099511627776, %275 ], [ %321, %318 ], [ %330, %322 ]
  %333 = icmp slt i64 %332, %24
  %334 = trunc i64 %332 to i32
  %335 = select i1 %333, i32 %334, i32 %17
  %336 = or i32 %335, %264
  br label %337

337:                                              ; preds = %256, %265, %267, %331
  %338 = phi i32 [ 32767, %256 ], [ %266, %265 ], [ %336, %331 ], [ %264, %267 ]
  %339 = and i32 %338, 65535
  %340 = shl i32 %339, %35
  %341 = or i32 %340, %62
  %342 = cmpxchg ptr addrspace(3) %33, i32 %61, i32 %341 monotonic monotonic, align 4
  %343 = extractvalue { i32, i1 } %342, 1
  %344 = extractvalue { i32, i1 } %342, 0
  br i1 %343, label %345, label %60, !llvm.loop !20036

345:                                              ; preds = %337
  %346 = add nuw nsw i32 %27, 1
  %347 = icmp eq i32 %346, %5
  br i1 %347, label %25, label %26, !llvm.loop !20037
}

; Function Attrs: nounwind
define dso_local void @__zluda_ptx_impl_agx_fabric_get(ptr addrspace(3) nocapture noundef writeonly %0, i32 noundef %1, i64 noundef %2, i32 noundef %3, ptr addrspace(3) noundef %4) local_unnamed_addr #3 {
  %6 = shl i32 %1, 6
  %7 = and i32 %6, 16320
  %8 = zext nneg i32 %7 to i64
  %9 = icmp ugt i32 %1, 255
  br i1 %9, label %23, label %10

10:                                               ; preds = %5
  %11 = zext i32 %3 to i64
  %12 = or disjoint i64 %8, 123076582834180
  %13 = inttoptr i64 %12 to ptr addrspace(1)
  %14 = load i32, ptr addrspace(1) %13, align 4
  %15 = zext i32 %14 to i64
  %16 = or disjoint i64 %8, 123076582834176
  %17 = inttoptr i64 %16 to ptr addrspace(1)
  %18 = load i32, ptr addrspace(1) %17, align 64
  %19 = icmp eq i32 %18, 0
  %20 = add i64 %11, %2
  %21 = icmp ugt i64 %20, %15
  %22 = select i1 %19, i1 true, i1 %21
  br i1 %22, label %23, label %24

23:                                               ; preds = %10, %5
  tail call void @llvm.trap()
  unreachable

24:                                               ; preds = %10
  %25 = or disjoint i64 %8, 123076582834184
  %26 = inttoptr i64 %25 to ptr addrspace(1)
  %27 = load i64, ptr addrspace(1) %26, align 8
  %28 = add i64 %27, %2
  %29 = inttoptr i64 %28 to ptr addrspace(1)
  %30 = icmp eq i32 %3, 0
  br i1 %30, label %47, label %31

31:                                               ; preds = %24
  %32 = icmp ult i32 %3, 4
  br i1 %32, label %45, label %33

33:                                               ; preds = %31
  %34 = and i32 %3, -4
  br label %35

35:                                               ; preds = %35, %33
  %36 = phi i32 [ 0, %33 ], [ %41, %35 ]
  %37 = zext i32 %36 to i64
  %38 = getelementptr inbounds i8, ptr addrspace(1) %29, i64 %37
  %39 = load <4 x i8>, ptr addrspace(1) %38, align 1
  %40 = getelementptr inbounds i8, ptr addrspace(3) %0, i64 %37
  store <4 x i8> %39, ptr addrspace(3) %40, align 1
  %41 = add nuw i32 %36, 4
  %42 = icmp eq i32 %41, %34
  br i1 %42, label %43, label %35, !llvm.loop !20038

43:                                               ; preds = %35
  %44 = icmp eq i32 %34, %3
  br i1 %44, label %47, label %45

45:                                               ; preds = %31, %43
  %46 = phi i32 [ 0, %31 ], [ %34, %43 ]
  br label %48

47:                                               ; preds = %48, %43, %24
  tail call void @__zluda_ptx_impl_agx_mbarrier_complete_tx(ptr addrspace(3) noundef %4, i32 noundef %3)
  ret void

48:                                               ; preds = %45, %48
  %49 = phi i32 [ %54, %48 ], [ %46, %45 ]
  %50 = zext i32 %49 to i64
  %51 = getelementptr inbounds i8, ptr addrspace(1) %29, i64 %50
  %52 = load i8, ptr addrspace(1) %51, align 1
  %53 = getelementptr inbounds i8, ptr addrspace(3) %0, i64 %50
  store i8 %52, ptr addrspace(3) %53, align 1
  %54 = add nuw i32 %49, 1
  %55 = icmp eq i32 %54, %3
  br i1 %55, label %47, label %48, !llvm.loop !20041
}


; Function Attrs: nounwind
define dso_local void @__zluda_ptx_impl_agx_fabric_put(i32 noundef %0, i64 noundef %1, i64 noundef %2, ptr addrspace(3) nocapture noundef readonly %3, i32 noundef %4, i32 noundef %5, i32 noundef %6, ptr addrspace(3) noundef %7) local_unnamed_addr #3 {
  %9 = and i32 %6, 1
  %10 = icmp eq i32 %9, 0
  br i1 %10, label %18, label %11

11:                                               ; preds = %8
  %12 = shl i32 %0, 6
  %13 = and i32 %12, 16320
  %14 = zext nneg i32 %13 to i64
  %15 = or disjoint i64 %14, 123076582834176
  %16 = inttoptr i64 %15 to ptr addrspace(1)
  %17 = load i32, ptr addrspace(1) %16, align 64
  br label %18

18:                                               ; preds = %8, %11
  %19 = phi i32 [ %17, %11 ], [ 1, %8 ]
  %20 = icmp eq i32 %19, 0
  br i1 %20, label %43, label %21

21:                                               ; preds = %18
  %22 = zext i32 %4 to i64
  %23 = shl i32 %0, 6
  %24 = and i32 %23, 16320
  %25 = zext nneg i32 %24 to i64
  %26 = icmp ugt i32 %0, 255
  %27 = or disjoint i64 %25, 123076582834180
  %28 = inttoptr i64 %27 to ptr addrspace(1)
  %29 = or disjoint i64 %25, 123076582834176
  %30 = inttoptr i64 %29 to ptr addrspace(1)
  %31 = add i64 %22, %1
  %32 = or disjoint i64 %25, 123076582834184
  %33 = and i32 %6, 2
  %34 = icmp eq i32 %33, 0
  %35 = and i32 %6, 4
  %36 = icmp eq i32 %35, 0
  %37 = add i64 %2, 8
  br i1 %26, label %53, label %38

38:                                               ; preds = %21
  %39 = and i32 %4, 1
  %40 = icmp eq i32 %4, 1
  %41 = and i32 %4, -2
  %42 = icmp eq i32 %39, 0
  br label %45

43:                                               ; preds = %118, %18
  %44 = lshr i32 %4, 4
  tail call void @__zluda_ptx_impl_agx_mbarrier_complete_tx(ptr addrspace(3) noundef %7, i32 noundef %44)
  ret void

45:                                               ; preds = %38, %118
  %46 = phi i32 [ %119, %118 ], [ 0, %38 ]
  %47 = load i32, ptr addrspace(1) %28, align 4
  %48 = zext i32 %47 to i64
  %49 = load i32, ptr addrspace(1) %30, align 64
  %50 = icmp ule i32 %49, %46
  %51 = icmp ugt i64 %31, %48
  %52 = select i1 %50, i1 true, i1 %51
  br i1 %52, label %53, label %54

53:                                               ; preds = %45, %21
  tail call void @llvm.trap()
  unreachable

54:                                               ; preds = %45
  %55 = shl i32 %46, 3
  %56 = zext i32 %55 to i64
  %57 = add nuw nsw i64 %32, %56
  %58 = inttoptr i64 %57 to ptr addrspace(1)
  %59 = load i64, ptr addrspace(1) %58, align 8
  %60 = add i64 %59, %1
  %61 = inttoptr i64 %60 to ptr addrspace(1)
  switch i32 %4, label %76 [
    i32 0, label %75
    i32 1, label %62
  ]

62:                                               ; preds = %101, %54
  %63 = phi i32 [ 0, %54 ], [ %102, %101 ]
  br i1 %42, label %75, label %64

64:                                               ; preds = %62
  br i1 %34, label %70, label %65

65:                                               ; preds = %64
  %66 = and i32 %63, 15
  %67 = shl nuw nsw i32 1, %66
  %68 = and i32 %67, %5
  %69 = icmp eq i32 %68, 0
  br i1 %69, label %75, label %70

70:                                               ; preds = %65, %64
  %71 = zext i32 %63 to i64
  %72 = getelementptr inbounds i8, ptr addrspace(3) %3, i64 %71
  %73 = load i8, ptr addrspace(3) %72, align 1
  %74 = getelementptr inbounds i8, ptr addrspace(1) %61, i64 %71
  store i8 %73, ptr addrspace(1) %74, align 1
  br label %75

75:                                               ; preds = %62, %70, %65, %54
  br i1 %36, label %118, label %105

76:                                               ; preds = %54, %101
  %77 = phi i32 [ %102, %101 ], [ 0, %54 ]
  %78 = phi i32 [ %103, %101 ], [ 0, %54 ]
  br i1 %34, label %84, label %79

79:                                               ; preds = %76
  %80 = and i32 %77, 14
  %81 = shl nuw nsw i32 1, %80
  %82 = and i32 %81, %5
  %83 = icmp eq i32 %82, 0
  br i1 %83, label %89, label %84

84:                                               ; preds = %79, %76
  %85 = zext i32 %77 to i64
  %86 = getelementptr inbounds i8, ptr addrspace(3) %3, i64 %85
  %87 = load i8, ptr addrspace(3) %86, align 1
  %88 = getelementptr inbounds i8, ptr addrspace(1) %61, i64 %85
  store i8 %87, ptr addrspace(1) %88, align 1
  br label %89

89:                                               ; preds = %79, %84
  %90 = or disjoint i32 %77, 1
  br i1 %34, label %96, label %91

91:                                               ; preds = %89
  %92 = and i32 %90, 15
  %93 = shl nuw nsw i32 1, %92
  %94 = and i32 %93, %5
  %95 = icmp eq i32 %94, 0
  br i1 %95, label %101, label %96

96:                                               ; preds = %91, %89
  %97 = zext i32 %90 to i64
  %98 = getelementptr inbounds i8, ptr addrspace(3) %3, i64 %97
  %99 = load i8, ptr addrspace(3) %98, align 1
  %100 = getelementptr inbounds i8, ptr addrspace(1) %61, i64 %97
  store i8 %99, ptr addrspace(1) %100, align 1
  br label %101

101:                                              ; preds = %96, %91
  %102 = add nuw i32 %77, 2
  %103 = add i32 %78, 2
  %104 = icmp eq i32 %103, %41
  br i1 %104, label %62, label %76, !llvm.loop !20042

105:                                              ; preds = %75
  %106 = load i32, ptr addrspace(1) %28, align 4
  %107 = zext i32 %106 to i64
  %108 = load i32, ptr addrspace(1) %30, align 64
  %109 = icmp ule i32 %108, %46
  %110 = icmp ugt i64 %37, %107
  %111 = select i1 %109, i1 true, i1 %110
  br i1 %111, label %112, label %113

112:                                              ; preds = %105
  tail call void @llvm.trap()
  unreachable

113:                                              ; preds = %105
  %114 = load i64, ptr addrspace(1) %58, align 8
  %115 = add i64 %114, %2
  %116 = inttoptr i64 %115 to ptr addrspace(1)
  %117 = atomicrmw add ptr addrspace(1) %116, i64 %22 monotonic, align 8
  br label %118

118:                                              ; preds = %113, %75
  %119 = add nuw i32 %46, 1
  %120 = icmp eq i32 %119, %19
  br i1 %120, label %43, label %45, !llvm.loop !20043
}

; Function Attrs: nounwind
define dso_local void @__zluda_ptx_impl_agx_fabric_red(i32 noundef %0, i64 noundef %1, i64 noundef %2, ptr addrspace(3) nocapture noundef readonly %3, i32 noundef %4, i32 noundef %5, i32 noundef %6, ptr addrspace(3) noundef %7) local_unnamed_addr #3 {
  %9 = and i32 %5, 15
  %10 = lshr i32 %5, 4
  %11 = and i32 %10, 31
  %12 = and i32 %5, 480
  %13 = icmp eq i32 %12, 192
  br i1 %13, label %17, label %14

14:                                               ; preds = %8
  switch i32 %11, label %15 [
    i32 15, label %17
    i32 14, label %17
    i32 9, label %17
    i32 8, label %17
    i32 7, label %16
    i32 5, label %16
    i32 4, label %16
    i32 3, label %16
  ]

15:                                               ; preds = %14
  br label %17

16:                                               ; preds = %14, %14, %14, %14
  br label %17

17:                                               ; preds = %8, %14, %14, %14, %14, %15, %16
  %18 = phi i32 [ 4, %15 ], [ 1, %8 ], [ 2, %14 ], [ 2, %14 ], [ 2, %14 ], [ 2, %14 ], [ 8, %16 ]
  %19 = and i32 %6, 1
  %20 = icmp eq i32 %19, 0
  br i1 %20, label %28, label %21

21:                                               ; preds = %17
  %22 = shl i32 %0, 6
  %23 = and i32 %22, 16320
  %24 = zext nneg i32 %23 to i64
  %25 = or disjoint i64 %24, 123076582834176
  %26 = inttoptr i64 %25 to ptr addrspace(1)
  %27 = load i32, ptr addrspace(1) %26, align 64
  br label %28

28:                                               ; preds = %17, %21
  %29 = phi i32 [ %27, %21 ], [ 1, %17 ]
  %30 = icmp eq i32 %29, 0
  br i1 %30, label %51, label %31

31:                                               ; preds = %28
  %32 = zext i32 %4 to i64
  %33 = shl i32 %0, 6
  %34 = and i32 %33, 16320
  %35 = zext nneg i32 %34 to i64
  %36 = icmp ugt i32 %0, 255
  %37 = or disjoint i64 %35, 123076582834180
  %38 = inttoptr i64 %37 to ptr addrspace(1)
  %39 = or disjoint i64 %35, 123076582834176
  %40 = inttoptr i64 %39 to ptr addrspace(1)
  %41 = add i64 %32, %1
  %42 = or disjoint i64 %35, 123076582834184
  %43 = icmp ugt i32 %18, %4
  %44 = and i32 %6, 4
  %45 = icmp eq i32 %44, 0
  %46 = add i64 %2, 8
  br i1 %36, label %61, label %47

47:                                               ; preds = %31
  %48 = and i32 %18, 1
  %49 = and i32 %18, 14
  %50 = icmp eq i32 %48, 0
  br label %53

51:                                               ; preds = %194, %28
  %52 = lshr i32 %4, 4
  tail call void @__zluda_ptx_impl_agx_mbarrier_complete_tx(ptr addrspace(3) noundef %7, i32 noundef %52)
  ret void

53:                                               ; preds = %47, %194
  %54 = phi i32 [ %195, %194 ], [ 0, %47 ]
  %55 = load i32, ptr addrspace(1) %38, align 4
  %56 = zext i32 %55 to i64
  %57 = load i32, ptr addrspace(1) %40, align 64
  %58 = icmp ule i32 %57, %54
  %59 = icmp ugt i64 %41, %56
  %60 = select i1 %58, i1 true, i1 %59
  br i1 %60, label %61, label %62

61:                                               ; preds = %53, %31
  tail call void @llvm.trap()
  unreachable

62:                                               ; preds = %53
  %63 = shl i32 %54, 3
  %64 = zext i32 %63 to i64
  %65 = add nuw nsw i64 %42, %64
  %66 = inttoptr i64 %65 to ptr addrspace(1)
  %67 = load i64, ptr addrspace(1) %66, align 8
  %68 = add i64 %67, %1
  br i1 %43, label %69, label %70

69:                                               ; preds = %178, %62
  br i1 %45, label %194, label %181

70:                                               ; preds = %62, %178
  %71 = phi i32 [ %179, %178 ], [ %18, %62 ]
  %72 = phi i32 [ %71, %178 ], [ 0, %62 ]
  %73 = zext i32 %72 to i64
  %74 = getelementptr inbounds i8, ptr addrspace(3) %3, i64 %73
  br i1 %13, label %99, label %75

75:                                               ; preds = %70, %75
  %76 = phi i32 [ %96, %75 ], [ 0, %70 ]
  %77 = phi i64 [ %95, %75 ], [ 0, %70 ]
  %78 = phi i32 [ %97, %75 ], [ 0, %70 ]
  %79 = zext nneg i32 %76 to i64
  %80 = getelementptr inbounds i8, ptr addrspace(3) %74, i64 %79
  %81 = load i8, ptr addrspace(3) %80, align 1
  %82 = zext i8 %81 to i64
  %83 = shl nuw nsw i32 %76, 3
  %84 = zext nneg i32 %83 to i64
  %85 = shl nuw i64 %82, %84
  %86 = or i64 %85, %77
  %87 = or disjoint i32 %76, 1
  %88 = zext nneg i32 %87 to i64
  %89 = getelementptr inbounds i8, ptr addrspace(3) %74, i64 %88
  %90 = load i8, ptr addrspace(3) %89, align 1
  %91 = zext i8 %90 to i64
  %92 = shl nuw nsw i32 %87, 3
  %93 = zext nneg i32 %92 to i64
  %94 = shl nuw i64 %91, %93
  %95 = or i64 %94, %86
  %96 = add nuw nsw i32 %76, 2
  %97 = add i32 %78, 2
  %98 = icmp eq i32 %97, %49
  br i1 %98, label %99, label %75, !llvm.loop !20044

99:                                               ; preds = %75, %70
  %100 = phi i64 [ undef, %70 ], [ %95, %75 ]
  %101 = phi i32 [ 0, %70 ], [ %96, %75 ]
  %102 = phi i64 [ 0, %70 ], [ %95, %75 ]
  br i1 %50, label %112, label %103

103:                                              ; preds = %99
  %104 = zext nneg i32 %101 to i64
  %105 = getelementptr inbounds i8, ptr addrspace(3) %74, i64 %104
  %106 = load i8, ptr addrspace(3) %105, align 1
  %107 = zext i8 %106 to i64
  %108 = shl nuw nsw i32 %101, 3
  %109 = zext nneg i32 %108 to i64
  %110 = shl nuw i64 %107, %109
  %111 = or i64 %110, %102
  br label %112

112:                                              ; preds = %99, %103
  %113 = phi i64 [ %100, %99 ], [ %111, %103 ]
  %114 = add i64 %68, %73
  br i1 %13, label %118, label %115

115:                                              ; preds = %112
  switch i32 %11, label %116 [
    i32 15, label %118
    i32 14, label %118
    i32 9, label %118
    i32 8, label %118
    i32 7, label %117
    i32 5, label %117
    i32 4, label %117
    i32 3, label %117
  ]

116:                                              ; preds = %115
  br label %118

117:                                              ; preds = %115, %115, %115, %115
  br label %118

118:                                              ; preds = %117, %116, %115, %115, %115, %115, %112
  %119 = phi i1 [ false, %116 ], [ false, %112 ], [ false, %115 ], [ false, %115 ], [ false, %115 ], [ false, %115 ], [ true, %117 ]
  %120 = phi i1 [ true, %116 ], [ false, %112 ], [ false, %115 ], [ false, %115 ], [ false, %115 ], [ false, %115 ], [ false, %117 ]
  %121 = phi i32 [ 32, %116 ], [ 8, %112 ], [ 16, %115 ], [ 16, %115 ], [ 16, %115 ], [ 16, %115 ], [ 64, %117 ]
  br i1 %119, label %122, label %137

122:                                              ; preds = %118
  %123 = inttoptr i64 %114 to ptr addrspace(1)
  %124 = load i64, ptr addrspace(1) %123, align 8
  br label %125

125:                                              ; preds = %132, %122
  %126 = phi i64 [ %124, %122 ], [ %136, %132 ]
  switch i32 %9, label %130 [
    i32 8, label %132
    i32 9, label %127
  ]

127:                                              ; preds = %125
  %128 = icmp eq i64 %126, %113
  %129 = select i1 %128, i64 0, i64 %126
  br label %132

130:                                              ; preds = %125
  %131 = tail call fastcc i64 @__zluda_agx_cvt_mm_combine(i32 noundef %9, i32 noundef %11, i64 noundef %126, i64 noundef %113)
  br label %132

132:                                              ; preds = %130, %127, %125
  %133 = phi i64 [ %129, %127 ], [ %131, %130 ], [ %113, %125 ]
  %134 = cmpxchg ptr addrspace(1) %123, i64 %126, i64 %133 monotonic monotonic, align 8
  %135 = extractvalue { i64, i1 } %134, 1
  %136 = extractvalue { i64, i1 } %134, 0
  br i1 %135, label %178, label %125, !llvm.loop !20045

137:                                              ; preds = %118
  %138 = and i64 %114, -4
  %139 = inttoptr i64 %138 to ptr addrspace(1)
  %140 = trunc i64 %114 to i32
  %141 = shl i32 %140, 3
  %142 = and i32 %141, 24
  %143 = shl nsw i32 -1, %121
  %144 = xor i32 %143, -1
  %145 = shl i32 %144, %142
  %146 = select i1 %120, i32 -1, i32 %145
  %147 = load i32, ptr addrspace(1) %139, align 4
  %148 = trunc i64 %113 to i32
  %149 = and i64 %113, 4294967295
  %150 = xor i32 %146, -1
  br label %151

151:                                              ; preds = %169, %137
  %152 = phi i32 [ %147, %137 ], [ %177, %169 ]
  %153 = and i32 %152, %146
  %154 = lshr i32 %153, %142
  switch i32 %9, label %165 [
    i32 6, label %155
    i32 7, label %159
    i32 8, label %169
    i32 9, label %162
  ]

155:                                              ; preds = %151
  %156 = icmp ult i32 %154, %148
  %157 = add i32 %154, 1
  %158 = select i1 %156, i32 %157, i32 0
  br label %169

159:                                              ; preds = %151
  %160 = add i32 %154, -1
  %161 = tail call i32 @llvm.umin.i32(i32 %160, i32 %148)
  br label %169

162:                                              ; preds = %151
  %163 = icmp eq i32 %154, %148
  %164 = select i1 %163, i32 0, i32 %154
  br label %169

165:                                              ; preds = %151
  %166 = zext i32 %154 to i64
  %167 = tail call fastcc i64 @__zluda_agx_cvt_mm_combine(i32 noundef %9, i32 noundef %11, i64 noundef %166, i64 noundef %149)
  %168 = trunc i64 %167 to i32
  br label %169

169:                                              ; preds = %165, %162, %159, %155, %151
  %170 = phi i32 [ %158, %155 ], [ %161, %159 ], [ %164, %162 ], [ %168, %165 ], [ %148, %151 ]
  %171 = and i32 %152, %150
  %172 = shl i32 %170, %142
  %173 = and i32 %172, %146
  %174 = or disjoint i32 %173, %171
  %175 = cmpxchg ptr addrspace(1) %139, i32 %152, i32 %174 monotonic monotonic, align 4
  %176 = extractvalue { i32, i1 } %175, 1
  %177 = extractvalue { i32, i1 } %175, 0
  br i1 %176, label %178, label %151

178:                                              ; preds = %169, %132
  %179 = add i32 %71, %18
  %180 = icmp ugt i32 %179, %4
  br i1 %180, label %69, label %70, !llvm.loop !20046

181:                                              ; preds = %69
  %182 = load i32, ptr addrspace(1) %38, align 4
  %183 = zext i32 %182 to i64
  %184 = load i32, ptr addrspace(1) %40, align 64
  %185 = icmp ule i32 %184, %54
  %186 = icmp ugt i64 %46, %183
  %187 = select i1 %185, i1 true, i1 %186
  br i1 %187, label %188, label %189

188:                                              ; preds = %181
  tail call void @llvm.trap()
  unreachable

189:                                              ; preds = %181
  %190 = load i64, ptr addrspace(1) %66, align 8
  %191 = add i64 %190, %2
  %192 = inttoptr i64 %191 to ptr addrspace(1)
  %193 = atomicrmw add ptr addrspace(1) %192, i64 %32 monotonic, align 8
  br label %194

194:                                              ; preds = %189, %69
  %195 = add nuw i32 %54, 1
  %196 = icmp eq i32 %195, %29
  br i1 %196, label %51, label %53, !llvm.loop !20047
}

; Function Attrs: nounwind
define dso_local void @__zluda_ptx_impl_agx_fabric_pullred(ptr addrspace(3) nocapture noundef writeonly %0, i32 noundef %1, i64 noundef %2, i32 noundef %3, i32 noundef %4, i32 noundef %5, ptr addrspace(3) noundef %6) local_unnamed_addr #3 {
  %8 = icmp eq i32 %5, 0
  br i1 %8, label %9, label %132

9:                                                ; preds = %7
  %10 = and i32 %4, 15
  %11 = lshr i32 %4, 4
  %12 = and i32 %11, 31
  %13 = and i32 %4, 480
  %14 = icmp eq i32 %13, 192
  br i1 %14, label %18, label %15

15:                                               ; preds = %9
  switch i32 %12, label %16 [
    i32 15, label %18
    i32 14, label %18
    i32 9, label %18
    i32 8, label %18
    i32 7, label %17
    i32 5, label %17
    i32 4, label %17
    i32 3, label %17
  ]

16:                                               ; preds = %15
  br label %18

17:                                               ; preds = %15, %15, %15, %15
  br label %18

18:                                               ; preds = %9, %15, %15, %15, %15, %16, %17
  %19 = phi i32 [ 4, %16 ], [ 1, %9 ], [ 2, %15 ], [ 2, %15 ], [ 2, %15 ], [ 2, %15 ], [ 8, %17 ]
  %20 = icmp ugt i32 %19, %3
  br i1 %20, label %57, label %21

21:                                               ; preds = %18
  %22 = shl i32 %1, 6
  %23 = and i32 %22, 16320
  %24 = zext nneg i32 %23 to i64
  %25 = icmp ugt i32 %1, 255
  %26 = zext nneg i32 %19 to i64
  %27 = or disjoint i64 %24, 123076582834184
  %28 = inttoptr i64 %27 to ptr addrspace(1)
  br i1 %25, label %65, label %29

29:                                               ; preds = %21
  %30 = or disjoint i64 %24, 123076582834176
  %31 = inttoptr i64 %30 to ptr addrspace(1)
  %32 = or disjoint i64 %24, 123076582834180
  %33 = inttoptr i64 %32 to ptr addrspace(1)
  %34 = load i32, ptr addrspace(1) %33, align 4
  %35 = zext i32 %34 to i64
  %36 = load i32, ptr addrspace(1) %31, align 64
  %37 = freeze i32 %36
  %38 = icmp eq i32 %37, 0
  %39 = icmp ugt i32 %37, 1
  br i1 %38, label %65, label %40

40:                                               ; preds = %29
  %41 = and i32 %19, 1
  %42 = and i32 %19, 14
  %43 = icmp eq i32 %41, 0
  br label %58

44:                                               ; preds = %111, %83
  %45 = phi i32 [ 0, %83 ], [ %129, %111 ]
  br i1 %43, label %54, label %46

46:                                               ; preds = %44
  %47 = shl nuw nsw i32 %45, 3
  %48 = zext nneg i32 %47 to i64
  %49 = lshr i64 %84, %48
  %50 = trunc i64 %49 to i8
  %51 = add i32 %45, %60
  %52 = zext i32 %51 to i64
  %53 = getelementptr inbounds i8, ptr addrspace(3) %0, i64 %52
  store i8 %50, ptr addrspace(3) %53, align 1
  br label %54

54:                                               ; preds = %44, %46
  %55 = add i32 %59, %19
  %56 = icmp ugt i32 %55, %3
  br i1 %56, label %57, label %58, !llvm.loop !20048

57:                                               ; preds = %54, %18
  tail call void @__zluda_ptx_impl_agx_mbarrier_complete_tx(ptr addrspace(3) noundef %6, i32 noundef %3)
  br label %132

58:                                               ; preds = %40, %54
  %59 = phi i32 [ %55, %54 ], [ %19, %40 ]
  %60 = phi i32 [ %59, %54 ], [ 0, %40 ]
  %61 = zext i32 %60 to i64
  %62 = add i64 %61, %2
  %63 = add i64 %62, %26
  %64 = icmp ugt i64 %63, %35
  br i1 %64, label %65, label %66

65:                                               ; preds = %58, %29, %21
  tail call void @llvm.trap()
  unreachable

66:                                               ; preds = %58
  %67 = load i64, ptr addrspace(1) %28, align 8
  %68 = add i64 %67, %62
  %69 = inttoptr i64 %68 to ptr addrspace(1)
  switch i32 %19, label %79 [
    i32 1, label %70
    i32 2, label %73
    i32 4, label %76
  ]

70:                                               ; preds = %66
  %71 = load i8, ptr addrspace(1) %69, align 1
  %72 = zext i8 %71 to i64
  br label %81

73:                                               ; preds = %66
  %74 = load i16, ptr addrspace(1) %69, align 2
  %75 = zext i16 %74 to i64
  br label %81

76:                                               ; preds = %66
  %77 = load i32, ptr addrspace(1) %69, align 4
  %78 = zext i32 %77 to i64
  br label %81

79:                                               ; preds = %66
  %80 = load i64, ptr addrspace(1) %69, align 8
  br label %81

81:                                               ; preds = %70, %73, %76, %79
  %82 = phi i64 [ %72, %70 ], [ %75, %73 ], [ %78, %76 ], [ %80, %79 ]
  br i1 %39, label %85, label %83

83:                                               ; preds = %106, %81
  %84 = phi i64 [ %82, %81 ], [ %108, %106 ]
  br i1 %14, label %44, label %111

85:                                               ; preds = %81, %106
  %86 = phi i32 [ %109, %106 ], [ 1, %81 ]
  %87 = phi i64 [ %108, %106 ], [ %82, %81 ]
  %88 = shl i32 %86, 3
  %89 = zext i32 %88 to i64
  %90 = add nuw nsw i64 %27, %89
  %91 = inttoptr i64 %90 to ptr addrspace(1)
  %92 = load i64, ptr addrspace(1) %91, align 8
  %93 = add i64 %92, %62
  %94 = inttoptr i64 %93 to ptr addrspace(1)
  switch i32 %19, label %104 [
    i32 1, label %95
    i32 2, label %98
    i32 4, label %101
  ]

95:                                               ; preds = %85
  %96 = load i8, ptr addrspace(1) %94, align 1
  %97 = zext i8 %96 to i64
  br label %106

98:                                               ; preds = %85
  %99 = load i16, ptr addrspace(1) %94, align 2
  %100 = zext i16 %99 to i64
  br label %106

101:                                              ; preds = %85
  %102 = load i32, ptr addrspace(1) %94, align 4
  %103 = zext i32 %102 to i64
  br label %106

104:                                              ; preds = %85
  %105 = load i64, ptr addrspace(1) %94, align 8
  br label %106

106:                                              ; preds = %95, %98, %101, %104
  %107 = phi i64 [ %97, %95 ], [ %100, %98 ], [ %103, %101 ], [ %105, %104 ]
  %108 = tail call fastcc i64 @__zluda_agx_cvt_mm_combine(i32 noundef %10, i32 noundef %12, i64 noundef %87, i64 noundef %107)
  %109 = add nuw i32 %86, 1
  %110 = icmp eq i32 %109, %37
  br i1 %110, label %83, label %85, !llvm.loop !20049

111:                                              ; preds = %83, %111
  %112 = phi i32 [ %129, %111 ], [ 0, %83 ]
  %113 = phi i32 [ %130, %111 ], [ 0, %83 ]
  %114 = shl nuw nsw i32 %112, 3
  %115 = zext nneg i32 %114 to i64
  %116 = lshr i64 %84, %115
  %117 = trunc i64 %116 to i8
  %118 = add i32 %112, %60
  %119 = zext i32 %118 to i64
  %120 = getelementptr inbounds i8, ptr addrspace(3) %0, i64 %119
  store i8 %117, ptr addrspace(3) %120, align 1
  %121 = or disjoint i32 %112, 1
  %122 = shl nuw nsw i32 %121, 3
  %123 = zext nneg i32 %122 to i64
  %124 = lshr i64 %84, %123
  %125 = trunc i64 %124 to i8
  %126 = add i32 %121, %60
  %127 = zext i32 %126 to i64
  %128 = getelementptr inbounds i8, ptr addrspace(3) %0, i64 %127
  store i8 %125, ptr addrspace(3) %128, align 1
  %129 = add nuw nsw i32 %112, 2
  %130 = add i32 %113, 2
  %131 = icmp eq i32 %130, %42
  br i1 %131, label %44, label %111, !llvm.loop !20050

132:                                              ; preds = %7, %57
  ret void
}

; Function Attrs: nounwind
define dso_local void @__zluda_ptx_impl_agx_fabric_atom(i32 noundef %0, i64 noundef %1, ptr addrspace(3) nocapture noundef writeonly %2, ptr addrspace(3) nocapture noundef readonly %3, i32 noundef %4, ptr addrspace(3) noundef %5) local_unnamed_addr #3 {
  %7 = and i32 %4, 15
  %8 = lshr i32 %4, 4
  %9 = and i32 %8, 31
  %10 = and i32 %4, 480
  %11 = icmp eq i32 %10, 192
  br i1 %11, label %15, label %12

12:                                               ; preds = %6
  switch i32 %9, label %13 [
    i32 15, label %15
    i32 14, label %15
    i32 9, label %15
    i32 8, label %15
    i32 7, label %14
    i32 5, label %14
    i32 4, label %14
    i32 3, label %14
  ]

13:                                               ; preds = %12
  br label %15

14:                                               ; preds = %12, %12, %12, %12
  br label %15

15:                                               ; preds = %6, %12, %12, %12, %12, %13, %14
  %16 = phi i32 [ 4, %13 ], [ 1, %6 ], [ 2, %12 ], [ 2, %12 ], [ 2, %12 ], [ 2, %12 ], [ 8, %14 ]
  %17 = trunc i64 %1 to i32
  %18 = and i64 %1, 15
  %19 = getelementptr inbounds i8, ptr addrspace(3) %3, i64 %18
  %20 = add nsw i32 %16, -1
  %21 = and i32 %16, 1
  %22 = icmp eq i32 %20, 0
  br i1 %22, label %49, label %23

23:                                               ; preds = %15
  %24 = and i32 %16, 14
  br label %25

25:                                               ; preds = %25, %23
  %26 = phi i32 [ 0, %23 ], [ %46, %25 ]
  %27 = phi i64 [ 0, %23 ], [ %45, %25 ]
  %28 = phi i32 [ 0, %23 ], [ %47, %25 ]
  %29 = zext nneg i32 %26 to i64
  %30 = getelementptr inbounds i8, ptr addrspace(3) %19, i64 %29
  %31 = load i8, ptr addrspace(3) %30, align 1
  %32 = zext i8 %31 to i64
  %33 = shl nuw nsw i32 %26, 3
  %34 = zext nneg i32 %33 to i64
  %35 = shl nuw i64 %32, %34
  %36 = or i64 %35, %27
  %37 = or disjoint i32 %26, 1
  %38 = zext nneg i32 %37 to i64
  %39 = getelementptr inbounds i8, ptr addrspace(3) %19, i64 %38
  %40 = load i8, ptr addrspace(3) %39, align 1
  %41 = zext i8 %40 to i64
  %42 = shl nuw nsw i32 %37, 3
  %43 = zext nneg i32 %42 to i64
  %44 = shl nuw i64 %41, %43
  %45 = or i64 %44, %36
  %46 = add nuw nsw i32 %26, 2
  %47 = add i32 %28, 2
  %48 = icmp eq i32 %47, %24
  br i1 %48, label %49, label %25, !llvm.loop !20044

49:                                               ; preds = %25, %15
  %50 = phi i64 [ undef, %15 ], [ %45, %25 ]
  %51 = phi i32 [ 0, %15 ], [ %46, %25 ]
  %52 = phi i64 [ 0, %15 ], [ %45, %25 ]
  %53 = icmp eq i32 %21, 0
  br i1 %53, label %63, label %54

54:                                               ; preds = %49
  %55 = zext nneg i32 %51 to i64
  %56 = getelementptr inbounds i8, ptr addrspace(3) %19, i64 %55
  %57 = load i8, ptr addrspace(3) %56, align 1
  %58 = zext i8 %57 to i64
  %59 = shl nuw nsw i32 %51, 3
  %60 = zext nneg i32 %59 to i64
  %61 = shl nuw i64 %58, %60
  %62 = or i64 %61, %52
  br label %63

63:                                               ; preds = %49, %54
  %64 = phi i64 [ %50, %49 ], [ %62, %54 ]
  %65 = and i32 %17, 15
  %66 = icmp eq i32 %7, 9
  br i1 %66, label %67, label %112

67:                                               ; preds = %63
  %68 = getelementptr inbounds i8, ptr addrspace(3) %3, i64 16
  %69 = getelementptr inbounds i8, ptr addrspace(3) %68, i64 %18
  %70 = and i32 %16, 1
  %71 = icmp eq i32 %20, 0
  br i1 %71, label %98, label %72

72:                                               ; preds = %67
  %73 = and i32 %16, 14
  br label %74

74:                                               ; preds = %74, %72
  %75 = phi i32 [ 0, %72 ], [ %95, %74 ]
  %76 = phi i64 [ 0, %72 ], [ %94, %74 ]
  %77 = phi i32 [ 0, %72 ], [ %96, %74 ]
  %78 = zext nneg i32 %75 to i64
  %79 = getelementptr inbounds i8, ptr addrspace(3) %69, i64 %78
  %80 = load i8, ptr addrspace(3) %79, align 1
  %81 = zext i8 %80 to i64
  %82 = shl nuw nsw i32 %75, 3
  %83 = zext nneg i32 %82 to i64
  %84 = shl nuw i64 %81, %83
  %85 = or i64 %84, %76
  %86 = or disjoint i32 %75, 1
  %87 = zext nneg i32 %86 to i64
  %88 = getelementptr inbounds i8, ptr addrspace(3) %69, i64 %87
  %89 = load i8, ptr addrspace(3) %88, align 1
  %90 = zext i8 %89 to i64
  %91 = shl nuw nsw i32 %86, 3
  %92 = zext nneg i32 %91 to i64
  %93 = shl nuw i64 %90, %92
  %94 = or i64 %93, %85
  %95 = add nuw nsw i32 %75, 2
  %96 = add i32 %77, 2
  %97 = icmp eq i32 %96, %73
  br i1 %97, label %98, label %74, !llvm.loop !20044

98:                                               ; preds = %74, %67
  %99 = phi i64 [ undef, %67 ], [ %94, %74 ]
  %100 = phi i32 [ 0, %67 ], [ %95, %74 ]
  %101 = phi i64 [ 0, %67 ], [ %94, %74 ]
  %102 = icmp eq i32 %70, 0
  br i1 %102, label %112, label %103

103:                                              ; preds = %98
  %104 = zext nneg i32 %100 to i64
  %105 = getelementptr inbounds i8, ptr addrspace(3) %69, i64 %104
  %106 = load i8, ptr addrspace(3) %105, align 1
  %107 = zext i8 %106 to i64
  %108 = shl nuw nsw i32 %100, 3
  %109 = zext nneg i32 %108 to i64
  %110 = shl nuw i64 %107, %109
  %111 = or i64 %110, %101
  br label %112

112:                                              ; preds = %103, %98, %63
  %113 = phi i64 [ 0, %63 ], [ %99, %98 ], [ %111, %103 ]
  %114 = shl i32 %0, 6
  %115 = and i32 %114, 16320
  %116 = zext nneg i32 %115 to i64
  %117 = icmp ugt i32 %0, 255
  br i1 %117, label %131, label %118

118:                                              ; preds = %112
  %119 = zext nneg i32 %16 to i64
  %120 = or disjoint i64 %116, 123076582834180
  %121 = inttoptr i64 %120 to ptr addrspace(1)
  %122 = load i32, ptr addrspace(1) %121, align 4
  %123 = zext i32 %122 to i64
  %124 = or disjoint i64 %116, 123076582834176
  %125 = inttoptr i64 %124 to ptr addrspace(1)
  %126 = load i32, ptr addrspace(1) %125, align 64
  %127 = icmp eq i32 %126, 0
  %128 = add i64 %119, %1
  %129 = icmp ugt i64 %128, %123
  %130 = select i1 %127, i1 true, i1 %129
  br i1 %130, label %131, label %132

131:                                              ; preds = %118, %112
  tail call void @llvm.trap()
  unreachable

132:                                              ; preds = %118
  %133 = or disjoint i64 %116, 123076582834184
  %134 = inttoptr i64 %133 to ptr addrspace(1)
  %135 = load i64, ptr addrspace(1) %134, align 8
  %136 = add i64 %135, %1
  br i1 %11, label %140, label %137

137:                                              ; preds = %132
  switch i32 %9, label %138 [
    i32 15, label %140
    i32 14, label %140
    i32 9, label %140
    i32 8, label %140
    i32 7, label %139
    i32 5, label %139
    i32 4, label %139
    i32 3, label %139
  ]

138:                                              ; preds = %137
  br label %140

139:                                              ; preds = %137, %137, %137, %137
  br label %140

140:                                              ; preds = %139, %138, %137, %137, %137, %137, %132
  %141 = phi i1 [ false, %138 ], [ false, %132 ], [ false, %137 ], [ false, %137 ], [ false, %137 ], [ false, %137 ], [ true, %139 ]
  %142 = phi i1 [ true, %138 ], [ false, %132 ], [ false, %137 ], [ false, %137 ], [ false, %137 ], [ false, %137 ], [ false, %139 ]
  %143 = phi i32 [ 32, %138 ], [ 8, %132 ], [ 16, %137 ], [ 16, %137 ], [ 16, %137 ], [ 16, %137 ], [ 64, %139 ]
  br i1 %141, label %144, label %159

144:                                              ; preds = %140
  %145 = inttoptr i64 %136 to ptr addrspace(1)
  %146 = load i64, ptr addrspace(1) %145, align 8
  br label %147

147:                                              ; preds = %154, %144
  %148 = phi i64 [ %146, %144 ], [ %158, %154 ]
  switch i32 %7, label %152 [
    i32 8, label %154
    i32 9, label %149
  ]

149:                                              ; preds = %147
  %150 = icmp eq i64 %148, %64
  %151 = select i1 %150, i64 %113, i64 %148
  br label %154

152:                                              ; preds = %147
  %153 = tail call fastcc i64 @__zluda_agx_cvt_mm_combine(i32 noundef %7, i32 noundef %9, i64 noundef %148, i64 noundef %64)
  br label %154

154:                                              ; preds = %152, %149, %147
  %155 = phi i64 [ %151, %149 ], [ %153, %152 ], [ %64, %147 ]
  %156 = cmpxchg ptr addrspace(1) %145, i64 %148, i64 %155 monotonic monotonic, align 8
  %157 = extractvalue { i64, i1 } %156, 1
  %158 = extractvalue { i64, i1 } %156, 0
  br i1 %157, label %203, label %147, !llvm.loop !20045

159:                                              ; preds = %140
  %160 = and i64 %136, -4
  %161 = inttoptr i64 %160 to ptr addrspace(1)
  %162 = trunc i64 %136 to i32
  %163 = shl i32 %162, 3
  %164 = and i32 %163, 24
  %165 = shl nsw i32 -1, %143
  %166 = xor i32 %165, -1
  %167 = shl i32 %166, %164
  %168 = select i1 %142, i32 -1, i32 %167
  %169 = load i32, ptr addrspace(1) %161, align 4
  %170 = trunc i64 %64 to i32
  %171 = trunc i64 %113 to i32
  %172 = and i64 %64, 4294967295
  %173 = xor i32 %168, -1
  br label %174

174:                                              ; preds = %192, %159
  %175 = phi i32 [ %169, %159 ], [ %200, %192 ]
  %176 = and i32 %175, %168
  %177 = lshr i32 %176, %164
  switch i32 %7, label %188 [
    i32 6, label %178
    i32 7, label %182
    i32 8, label %192
    i32 9, label %185
  ]

178:                                              ; preds = %174
  %179 = icmp ult i32 %177, %170
  %180 = add i32 %177, 1
  %181 = select i1 %179, i32 %180, i32 0
  br label %192

182:                                              ; preds = %174
  %183 = add i32 %177, -1
  %184 = tail call i32 @llvm.umin.i32(i32 %183, i32 %170)
  br label %192

185:                                              ; preds = %174
  %186 = icmp eq i32 %177, %170
  %187 = select i1 %186, i32 %171, i32 %177
  br label %192

188:                                              ; preds = %174
  %189 = zext i32 %177 to i64
  %190 = tail call fastcc i64 @__zluda_agx_cvt_mm_combine(i32 noundef %7, i32 noundef %9, i64 noundef %189, i64 noundef %172)
  %191 = trunc i64 %190 to i32
  br label %192

192:                                              ; preds = %188, %185, %182, %178, %174
  %193 = phi i32 [ %181, %178 ], [ %184, %182 ], [ %187, %185 ], [ %191, %188 ], [ %170, %174 ]
  %194 = and i32 %175, %173
  %195 = shl i32 %193, %164
  %196 = and i32 %195, %168
  %197 = or disjoint i32 %196, %194
  %198 = cmpxchg ptr addrspace(1) %161, i32 %175, i32 %197 monotonic monotonic, align 4
  %199 = extractvalue { i32, i1 } %198, 1
  %200 = extractvalue { i32, i1 } %198, 0
  br i1 %199, label %201, label %174

201:                                              ; preds = %192
  %202 = zext i32 %177 to i64
  br label %203

203:                                              ; preds = %154, %201
  %204 = phi i64 [ %202, %201 ], [ %148, %154 ]
  %205 = and i32 %16, 1
  %206 = icmp eq i32 %20, 0
  br i1 %206, label %209, label %207

207:                                              ; preds = %203
  %208 = and i32 %16, 14
  br label %221

209:                                              ; preds = %221, %203
  %210 = phi i32 [ 0, %203 ], [ %239, %221 ]
  %211 = icmp eq i32 %205, 0
  br i1 %211, label %220, label %212

212:                                              ; preds = %209
  %213 = shl nuw nsw i32 %210, 3
  %214 = zext nneg i32 %213 to i64
  %215 = lshr i64 %204, %214
  %216 = trunc i64 %215 to i8
  %217 = add nuw nsw i32 %210, %65
  %218 = zext nneg i32 %217 to i64
  %219 = getelementptr inbounds i8, ptr addrspace(3) %2, i64 %218
  store i8 %216, ptr addrspace(3) %219, align 1
  br label %220

220:                                              ; preds = %209, %212
  tail call void @__zluda_ptx_impl_agx_mbarrier_complete_tx(ptr addrspace(3) noundef %5, i32 noundef 1)
  ret void

221:                                              ; preds = %221, %207
  %222 = phi i32 [ 0, %207 ], [ %239, %221 ]
  %223 = phi i32 [ 0, %207 ], [ %240, %221 ]
  %224 = shl nuw nsw i32 %222, 3
  %225 = zext nneg i32 %224 to i64
  %226 = lshr i64 %204, %225
  %227 = trunc i64 %226 to i8
  %228 = add nuw nsw i32 %222, %65
  %229 = zext nneg i32 %228 to i64
  %230 = getelementptr inbounds i8, ptr addrspace(3) %2, i64 %229
  store i8 %227, ptr addrspace(3) %230, align 1
  %231 = or disjoint i32 %222, 1
  %232 = shl nuw nsw i32 %231, 3
  %233 = zext nneg i32 %232 to i64
  %234 = lshr i64 %204, %233
  %235 = trunc i64 %234 to i8
  %236 = add nuw nsw i32 %231, %65
  %237 = zext nneg i32 %236 to i64
  %238 = getelementptr inbounds i8, ptr addrspace(3) %2, i64 %237
  store i8 %235, ptr addrspace(3) %238, align 1
  %239 = add nuw nsw i32 %222, 2
  %240 = add i32 %223, 2
  %241 = icmp eq i32 %240, %208
  br i1 %241, label %209, label %221, !llvm.loop !20051
}

; Function Attrs: nounwind
define dso_local void @__zluda_ptx_impl_agx_fabric_tensor(ptr noundef %0, i32 noundef %1, i64 noundef %2, i32 noundef %3, i32 noundef %4, i32 noundef %5, i32 noundef %6, i32 noundef %7, i32 noundef %8, i32 noundef %9, i32 noundef %10, i32 noundef %11, i32 noundef %12, i32 noundef %13, ptr addrspace(3) noundef %14, i32 noundef %15, i32 noundef %16, ptr addrspace(3) noundef %17) local_unnamed_addr #3 {
  %19 = alloca [16 x i64], align 8
  call void @llvm.lifetime.start.p0(ptr nonnull %19)
  %20 = and i32 %16, 2
  %21 = icmp eq i32 %20, 0
  %22 = and i32 %16, 3
  %23 = icmp eq i32 %22, 3
  br i1 %23, label %24, label %31

24:                                               ; preds = %18
  %25 = shl i32 %1, 6
  %26 = and i32 %25, 16320
  %27 = zext nneg i32 %26 to i64
  %28 = or disjoint i64 %27, 123076582834176
  %29 = inttoptr i64 %28 to ptr addrspace(1)
  %30 = load i32, ptr addrspace(1) %29, align 64
  br label %31

31:                                               ; preds = %18, %24
  %32 = phi i32 [ %30, %24 ], [ 1, %18 ]
  %33 = icmp eq i32 %32, 0
  br i1 %33, label %47, label %34

34:                                               ; preds = %31
  %35 = shl i32 %1, 6
  %36 = and i32 %35, 16320
  %37 = zext nneg i32 %36 to i64
  %38 = icmp ugt i32 %1, 255
  %39 = or disjoint i64 %37, 123076582834180
  %40 = inttoptr i64 %39 to ptr addrspace(1)
  %41 = or disjoint i64 %37, 123076582834176
  %42 = inttoptr i64 %41 to ptr addrspace(1)
  %43 = or disjoint i64 %37, 123076582834184
  %44 = ptrtoint ptr addrspace(3) %14 to i64
  %45 = trunc i64 %44 to i32
  %46 = select i1 %21, i32 -1, i32 -2
  br i1 %38, label %59, label %51

47:                                               ; preds = %60, %31
  %48 = phi i32 [ 0, %31 ], [ %67, %60 ]
  %49 = lshr i32 %15, 4
  %50 = select i1 %21, i32 %48, i32 %49
  call void @__zluda_ptx_impl_agx_mbarrier_complete_tx(ptr addrspace(3) noundef %17, i32 noundef %50)
  call void @llvm.lifetime.end.p0(ptr nonnull %19)
  ret void

51:                                               ; preds = %34, %60
  %52 = phi i32 [ %68, %60 ], [ 0, %34 ]
  %53 = load i32, ptr addrspace(1) %40, align 4
  %54 = zext i32 %53 to i64
  %55 = load i32, ptr addrspace(1) %42, align 64
  %56 = icmp ule i32 %55, %52
  %57 = icmp ult i64 %54, %2
  %58 = select i1 %56, i1 true, i1 %57
  br i1 %58, label %59, label %60

59:                                               ; preds = %51, %34
  call void @llvm.trap()
  unreachable

60:                                               ; preds = %51
  %61 = shl i32 %52, 3
  %62 = zext i32 %61 to i64
  %63 = add nuw nsw i64 %43, %62
  %64 = inttoptr i64 %63 to ptr addrspace(1)
  %65 = load i64, ptr addrspace(1) %64, align 8
  %66 = add i64 %65, %2
  call void @__zluda_ptx_impl_agx_tma_override(ptr noundef nonnull %19, ptr noundef %0, i1 noundef zeroext true, i64 noundef %66, i32 noundef %3, i32 noundef %4, i32 noundef %5, i32 noundef %6, i32 noundef %7, i32 noundef %8, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0)
  %67 = call i32 @__zluda_ptx_impl_agx_tma_copy(ptr noundef nonnull %19, i32 noundef %9, i32 noundef %10, i32 noundef %11, i32 noundef %12, i32 noundef %13, i32 noundef -1, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef %45, i32 noundef %46)
  %68 = add nuw i32 %52, 1
  %69 = icmp eq i32 %68, %32
  br i1 %69, label %47, label %51, !llvm.loop !20052
}





; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.abs.i32(i32, i1 immarg)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.abs.i64(i64, i1 immarg)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.umax.i32(i32, i32)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.ctpop.i64(i64)

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)



!20000 = !{i32 1, !"wchar_size", i32 4}
!20001 = !{i32 7, !"frame-pointer", i32 2}
!20002 = !{!"Ubuntu clang version 18.1.3 (1ubuntu1)"}
!20003 = distinct !{!20003, !20004}
!20004 = !{!"llvm.loop.mustprogress"}
!20005 = !{i64 0, i64 65}
!20006 = distinct !{!20006, !20004}
!20007 = distinct !{!20007, !20004}
!20008 = distinct !{!20008, !20004}
!20009 = distinct !{!20009, !20004}
!20010 = distinct !{!20010, !20004}
!20011 = distinct !{!20011, !20004}
!20012 = !{!20013, !20013, i64 0}
!20013 = !{!"int", !20014, i64 0}
!20014 = !{!"omnipotent char", !20015, i64 0}
!20015 = !{!"Simple C/C++ TBAA"}
!20016 = distinct !{!20016, !20004}
!20017 = distinct !{!20017, !20004}
!20018 = distinct !{!20018, !20004}
!20019 = distinct !{!20019, !20004}
!20020 = distinct !{!20020, !20004}
!20021 = distinct !{!20021, !20004}
!20022 = !{!20023, !20023, i64 0}
!20023 = !{!"long long", !20014, i64 0}
!20024 = !{!20014, !20014, i64 0}
!20025 = !{!20026, !20026, i64 0}
!20026 = !{!"short", !20014, i64 0}
!20027 = distinct !{!20027, !20004}
!20028 = distinct !{!20028, !20004}
!20029 = distinct !{!20029, !20004}
!20030 = distinct !{!20030, !20004}
!20031 = distinct !{!20031, !20004}
!20032 = distinct !{!20032, !20004}
!20033 = distinct !{!20033, !20004}
!20034 = distinct !{!20034, !20004}
!20035 = distinct !{!20035, !20004}
!20036 = distinct !{!20036, !20004}
!20037 = distinct !{!20037, !20004}
!20038 = distinct !{!20038, !20004, !20039, !20040}
!20039 = !{!"llvm.loop.isvectorized", i32 1}
!20040 = !{!"llvm.loop.unroll.runtime.disable"}
!20041 = distinct !{!20041, !20004, !20040, !20039}
!20042 = distinct !{!20042, !20004}
!20043 = distinct !{!20043, !20004}
!20044 = distinct !{!20044, !20004}
!20045 = distinct !{!20045, !20004}
!20046 = distinct !{!20046, !20004}
!20047 = distinct !{!20047, !20004}
!20048 = distinct !{!20048, !20004}
!20049 = distinct !{!20049, !20004}
!20050 = distinct !{!20050, !20004}
!20051 = distinct !{!20051, !20004}
!20052 = distinct !{!20052, !20004}

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
