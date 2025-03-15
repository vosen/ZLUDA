declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @cvt_s32_f32(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #1 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"47" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"47", ptr addrspace(5) %"43", align 4
  %"48" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"48", ptr addrspace(5) %"44", align 4
  %"50" = load i64, ptr addrspace(5) %"43", align 4
  %"62" = inttoptr i64 %"50" to ptr
  %"61" = load float, ptr %"62", align 4
  %"49" = bitcast float %"61" to i32
  store i32 %"49", ptr addrspace(5) %"45", align 4
  %"51" = load i64, ptr addrspace(5) %"43", align 4
  %"63" = inttoptr i64 %"51" to ptr
  %"31" = getelementptr inbounds i8, ptr %"63", i64 4
  %"64" = load float, ptr %"31", align 4
  %"52" = bitcast float %"64" to i32
  store i32 %"52", ptr addrspace(5) %"46", align 4
  %"54" = load i32, ptr addrspace(5) %"45", align 4
  %"66" = bitcast i32 %"54" to float
  %2 = call float @llvm.ceil.f32(float %"66")
  %3 = fptosi float %2 to i32
  %"65" = freeze i32 %3
  store i32 %"65", ptr addrspace(5) %"45", align 4
  %"56" = load i32, ptr addrspace(5) %"46", align 4
  %"68" = bitcast i32 %"56" to float
  %4 = call float @llvm.ceil.f32(float %"68")
  %5 = fptosi float %4 to i32
  %"67" = freeze i32 %5
  store i32 %"67", ptr addrspace(5) %"46", align 4
  %"57" = load i64, ptr addrspace(5) %"44", align 4
  %"58" = load i32, ptr addrspace(5) %"45", align 4
  %"69" = inttoptr i64 %"57" to ptr addrspace(1)
  store i32 %"58", ptr addrspace(1) %"69", align 4
  %"59" = load i64, ptr addrspace(5) %"44", align 4
  %"71" = inttoptr i64 %"59" to ptr addrspace(1)
  %"33" = getelementptr inbounds i8, ptr addrspace(1) %"71", i64 4
  %"60" = load i32, ptr addrspace(5) %"46", align 4
  store i32 %"60", ptr addrspace(1) %"33", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.ceil.f32(float) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }