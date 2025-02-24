declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @non_scalar_ptr_offset(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #1 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"57"

"57":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"45", ptr addrspace(5) %"41", align 4
  %"46" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"46", ptr addrspace(5) %"42", align 4
  %"47" = load i64, ptr addrspace(5) %"41", align 4
  %"55" = inttoptr i64 %"47" to ptr addrspace(1)
  %"32" = getelementptr inbounds i8, ptr addrspace(1) %"55", i64 8
  %"30" = load <2 x i32>, ptr addrspace(1) %"32", align 8
  %"48" = extractelement <2 x i32> %"30", i8 0
  %"49" = extractelement <2 x i32> %"30", i8 1
  store i32 %"48", ptr addrspace(5) %"43", align 4
  store i32 %"49", ptr addrspace(5) %"44", align 4
  %"51" = load i32, ptr addrspace(5) %"43", align 4
  %"52" = load i32, ptr addrspace(5) %"44", align 4
  %"50" = add i32 %"51", %"52"
  store i32 %"50", ptr addrspace(5) %"43", align 4
  %"53" = load i64, ptr addrspace(5) %"42", align 4
  %"54" = load i32, ptr addrspace(5) %"43", align 4
  %"56" = inttoptr i64 %"53" to ptr addrspace(1)
  store i32 %"54", ptr addrspace(1) %"56", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }