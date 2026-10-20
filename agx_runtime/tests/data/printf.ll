; A kernel that writes one printf record by hand, in the layout of
; __zluda_ptx_impl_vprintf: printf("%d\n", 41 + tid).
@fmt = private unnamed_addr addrspace(1) constant [4 x i8] c"%d\0A\00", align 1
@__zluda_agx_printf_buffer = internal addrspace(1) global [256 x i8] zeroinitializer, align 8

declare i32 @llvm.agx.thread.position.in.grid.x()

define agx_kernel void @say() {
  %t = call i32 @llvm.agx.thread.position.in.grid.x()
  %at = atomicrmw add ptr addrspace(1) @__zluda_agx_printf_buffer, i32 24 monotonic
  %off = add i32 %at, 8
  %off64 = zext i32 %off to i64
  %rec = getelementptr i8, ptr addrspace(1) @__zluda_agx_printf_buffer, i64 %off64
  store i32 24, ptr addrspace(1) %rec, align 8
  %p1 = getelementptr i8, ptr addrspace(1) %rec, i64 4
  store i32 4, ptr addrspace(1) %p1, align 4
  %p2 = getelementptr i8, ptr addrspace(1) %rec, i64 8
  %f = ptrtoint ptr addrspace(1) @fmt to i64
  store i64 %f, ptr addrspace(1) %p2, align 8
  %p3 = getelementptr i8, ptr addrspace(1) %rec, i64 16
  %v = add i32 %t, 41
  store i32 %v, ptr addrspace(1) %p3, align 4
  ret void
}
