; llc -mtriple=agx -mcpu=agx-g14c -mattr=+agx-checked -filetype=obj chase.ll -o chase.o
;
; A checked kernel that follows pointers loaded from memory: out[i] = *ptrs[i].
; The loaded pointers have no slot; they are checked against the allocation
; table (.agx.kernels extension 3).

define agx_kernel void @chase(ptr addrspace(1) %ptrs, ptr addrspace(1) %out) {
  %i = call i32 @llvm.agx.thread.position.in.threadgroup.x()
  %pp = getelementptr ptr addrspace(1), ptr addrspace(1) %ptrs, i32 %i
  %p = load ptr addrspace(1), ptr addrspace(1) %pp, align 8
  %v = load i32, ptr addrspace(1) %p, align 4
  %po = getelementptr i32, ptr addrspace(1) %out, i32 %i
  store i32 %v, ptr addrspace(1) %po, align 4
  ret void
}

declare i32 @llvm.agx.thread.position.in.threadgroup.x()
