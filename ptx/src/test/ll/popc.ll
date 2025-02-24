declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @popc(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"39" = load i64, ptr addrspace(4) %"34", align 4
  store i64 %"39", ptr addrspace(5) %"36", align 4
  %"40" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"40", ptr addrspace(5) %"37", align 4
  %"42" = load i64, ptr addrspace(5) %"36", align 4
  %"47" = inttoptr i64 %"42" to ptr
  %"41" = load i32, ptr %"47", align 4
  store i32 %"41", ptr addrspace(5) %"38", align 4
  %"44" = load i32, ptr addrspace(5) %"38", align 4
  %"48" = call i32 @llvm.ctpop.i32(i32 %"44")
  store i32 %"48", ptr addrspace(5) %"38", align 4
  %"45" = load i64, ptr addrspace(5) %"37", align 4
  %"46" = load i32, ptr addrspace(5) %"38", align 4
  %"49" = inttoptr i64 %"45" to ptr
  store i32 %"46", ptr %"49", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.ctpop.i32(i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
