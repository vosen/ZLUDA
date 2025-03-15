declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @cvt_sat_s_u(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #1 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"31"

"31":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"45", ptr addrspace(5) %"40", align 4
  %"46" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"46", ptr addrspace(5) %"41", align 4
  %"48" = load i64, ptr addrspace(5) %"40", align 4
  %"55" = inttoptr i64 %"48" to ptr
  %"47" = load i32, ptr %"55", align 4
  store i32 %"47", ptr addrspace(5) %"42", align 4
  %"50" = load i32, ptr addrspace(5) %"42", align 4
  %2 = call i32 @llvm.smax.i32(i32 %"50", i32 0)
  %3 = call i32 @llvm.umin.i32(i32 %2, i32 -1)
  store i32 %3, ptr addrspace(5) %"43", align 4
  %"52" = load i32, ptr addrspace(5) %"43", align 4
  store i32 %"52", ptr addrspace(5) %"44", align 4
  %"53" = load i64, ptr addrspace(5) %"41", align 4
  %"54" = load i32, ptr addrspace(5) %"44", align 4
  %"56" = inttoptr i64 %"53" to ptr
  store i32 %"54", ptr %"56", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.smax.i32(i32, i32) #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.umin.i32(i32, i32) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }