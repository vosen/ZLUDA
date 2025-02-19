declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @cvt_s32_f32(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"45" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"45", ptr addrspace(5) %"41", align 4
  %"46" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"46", ptr addrspace(5) %"42", align 4
  %"48" = load i64, ptr addrspace(5) %"41", align 4
  %"60" = inttoptr i64 %"48" to ptr
  %"59" = load float, ptr %"60", align 4
  %"47" = bitcast float %"59" to i32
  store i32 %"47", ptr addrspace(5) %"43", align 4
  %"49" = load i64, ptr addrspace(5) %"41", align 4
  %"61" = inttoptr i64 %"49" to ptr
  %"30" = getelementptr inbounds i8, ptr %"61", i64 4
  %"62" = load float, ptr %"30", align 4
  %"50" = bitcast float %"62" to i32
  store i32 %"50", ptr addrspace(5) %"44", align 4
  %"52" = load i32, ptr addrspace(5) %"43", align 4
  %"64" = bitcast i32 %"52" to float
  %2 = call float @llvm.ceil.f32(float %"64")
  %3 = fptosi float %2 to i32
  %"63" = freeze i32 %3
  store i32 %"63", ptr addrspace(5) %"43", align 4
  %"54" = load i32, ptr addrspace(5) %"44", align 4
  %"66" = bitcast i32 %"54" to float
  %4 = call float @llvm.ceil.f32(float %"66")
  %5 = fptosi float %4 to i32
  %"65" = freeze i32 %5
  store i32 %"65", ptr addrspace(5) %"44", align 4
  %"55" = load i64, ptr addrspace(5) %"42", align 4
  %"56" = load i32, ptr addrspace(5) %"43", align 4
  %"67" = inttoptr i64 %"55" to ptr addrspace(1)
  store i32 %"56", ptr addrspace(1) %"67", align 4
  %"57" = load i64, ptr addrspace(5) %"42", align 4
  %"69" = inttoptr i64 %"57" to ptr addrspace(1)
  %"32" = getelementptr inbounds i8, ptr addrspace(1) %"69", i64 4
  %"58" = load i32, ptr addrspace(5) %"44", align 4
  store i32 %"58", ptr addrspace(1) %"32", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.ceil.f32(float) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
