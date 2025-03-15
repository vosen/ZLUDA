declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @reg_local(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #1 {
  %"10" = alloca [8 x i8], align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"48" = load i64, ptr addrspace(4) %"43", align 4
  store i64 %"48", ptr addrspace(5) %"45", align 4
  %"49" = load i64, ptr addrspace(4) %"44", align 4
  store i64 %"49", ptr addrspace(5) %"46", align 4
  %"51" = load i64, ptr addrspace(5) %"45", align 4
  %"57" = inttoptr i64 %"51" to ptr addrspace(1)
  %"56" = load i64, ptr addrspace(1) %"57", align 4
  store i64 %"56", ptr addrspace(5) %"47", align 4
  %"52" = load i64, ptr addrspace(5) %"47", align 4
  %"31" = add i64 %"52", 1
  %"58" = addrspacecast ptr addrspace(5) %"10" to ptr
  store i64 %"31", ptr %"58", align 4
  %"60" = addrspacecast ptr addrspace(5) %"10" to ptr
  %"33" = getelementptr inbounds i8, ptr %"60", i64 0
  %"61" = load i64, ptr %"33", align 4
  store i64 %"61", ptr addrspace(5) %"47", align 4
  %"54" = load i64, ptr addrspace(5) %"46", align 4
  %"62" = inttoptr i64 %"54" to ptr addrspace(1)
  %"35" = getelementptr inbounds i8, ptr addrspace(1) %"62", i64 0
  %"55" = load i64, ptr addrspace(5) %"47", align 4
  store i64 %"55", ptr addrspace(1) %"35", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }