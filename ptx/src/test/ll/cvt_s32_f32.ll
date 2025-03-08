declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @cvt_s32_f32(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #1 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"72"

"72":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"46", ptr addrspace(5) %"42", align 4
  %"47" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"47", ptr addrspace(5) %"43", align 4
  %"49" = load i64, ptr addrspace(5) %"42", align 4
  %"61" = inttoptr i64 %"49" to ptr
  %"60" = load float, ptr %"61", align 4
  %"48" = bitcast float %"60" to i32
  store i32 %"48", ptr addrspace(5) %"44", align 4
  %"50" = load i64, ptr addrspace(5) %"42", align 4
  %"62" = inttoptr i64 %"50" to ptr
  %"31" = getelementptr inbounds i8, ptr %"62", i64 4
  %"63" = load float, ptr %"31", align 4
  %"51" = bitcast float %"63" to i32
  store i32 %"51", ptr addrspace(5) %"45", align 4
  %"53" = load i32, ptr addrspace(5) %"44", align 4
  %"65" = bitcast i32 %"53" to float
  %2 = call float @llvm.ceil.f32(float %"65")
  %3 = fptosi float %2 to i32
  %"64" = freeze i32 %3
  store i32 %"64", ptr addrspace(5) %"44", align 4
  %"55" = load i32, ptr addrspace(5) %"45", align 4
  %"67" = bitcast i32 %"55" to float
  %4 = call float @llvm.ceil.f32(float %"67")
  %5 = fptosi float %4 to i32
  %"66" = freeze i32 %5
  store i32 %"66", ptr addrspace(5) %"45", align 4
  %"56" = load i64, ptr addrspace(5) %"43", align 4
  %"57" = load i32, ptr addrspace(5) %"44", align 4
  %"68" = inttoptr i64 %"56" to ptr addrspace(1)
  store i32 %"57", ptr addrspace(1) %"68", align 4
  %"58" = load i64, ptr addrspace(5) %"43", align 4
  %"70" = inttoptr i64 %"58" to ptr addrspace(1)
  %"33" = getelementptr inbounds i8, ptr addrspace(1) %"70", i64 4
  %"59" = load i32, ptr addrspace(5) %"45", align 4
  store i32 %"59", ptr addrspace(1) %"33", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.ceil.f32(float) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }