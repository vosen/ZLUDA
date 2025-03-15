declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @prmt(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #1 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"45", ptr addrspace(5) %"41", align 4
  %"46" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"46", ptr addrspace(5) %"42", align 4
  %"48" = load i64, ptr addrspace(5) %"41", align 4
  %"56" = inttoptr i64 %"48" to ptr
  %"47" = load i32, ptr %"56", align 4
  store i32 %"47", ptr addrspace(5) %"43", align 4
  %"49" = load i64, ptr addrspace(5) %"41", align 4
  %"57" = inttoptr i64 %"49" to ptr
  %"31" = getelementptr inbounds i8, ptr %"57", i64 4
  %"50" = load i32, ptr %"31", align 4
  store i32 %"50", ptr addrspace(5) %"44", align 4
  %"52" = load i32, ptr addrspace(5) %"43", align 4
  %"53" = load i32, ptr addrspace(5) %"44", align 4
  %2 = bitcast i32 %"52" to <4 x i8>
  %3 = bitcast i32 %"53" to <4 x i8>
  %"58" = shufflevector <4 x i8> %2, <4 x i8> %3, <4 x i32> <i32 4, i32 0, i32 6, i32 7>
  store <4 x i8> %"58", ptr addrspace(5) %"44", align 4
  %"54" = load i64, ptr addrspace(5) %"42", align 4
  %"55" = load i32, ptr addrspace(5) %"44", align 4
  %"61" = inttoptr i64 %"54" to ptr
  store i32 %"55", ptr %"61", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }