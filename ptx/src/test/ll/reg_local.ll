declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @reg_local(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #1 {
  %"10" = alloca [8 x i8], align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"63"

"63":                                             ; preds = %1
  %"47" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"47", ptr addrspace(5) %"44", align 4
  %"48" = load i64, ptr addrspace(4) %"43", align 4
  store i64 %"48", ptr addrspace(5) %"45", align 4
  %"50" = load i64, ptr addrspace(5) %"44", align 4
  %"56" = inttoptr i64 %"50" to ptr addrspace(1)
  %"55" = load i64, ptr addrspace(1) %"56", align 4
  store i64 %"55", ptr addrspace(5) %"46", align 4
  %"51" = load i64, ptr addrspace(5) %"46", align 4
  %"31" = add i64 %"51", 1
  %"57" = addrspacecast ptr addrspace(5) %"10" to ptr
  store i64 %"31", ptr %"57", align 4
  %"59" = addrspacecast ptr addrspace(5) %"10" to ptr
  %"33" = getelementptr inbounds i8, ptr %"59", i64 0
  %"60" = load i64, ptr %"33", align 4
  store i64 %"60", ptr addrspace(5) %"46", align 4
  %"53" = load i64, ptr addrspace(5) %"45", align 4
  %"61" = inttoptr i64 %"53" to ptr addrspace(1)
  %"35" = getelementptr inbounds i8, ptr addrspace(1) %"61", i64 0
  %"54" = load i64, ptr addrspace(5) %"46", align 4
  store i64 %"54", ptr addrspace(1) %"35", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }