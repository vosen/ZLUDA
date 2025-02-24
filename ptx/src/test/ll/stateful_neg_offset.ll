declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @stateful_neg_offset(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #1 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"68"

"68":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"46", ptr addrspace(5) %"40", align 4
  %"47" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"47", ptr addrspace(5) %"41", align 4
  %"49" = load i64, ptr addrspace(5) %"40", align 4
  %2 = inttoptr i64 %"49" to ptr
  %"62" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"62", ptr addrspace(5) %"42", align 8
  %"51" = load i64, ptr addrspace(5) %"41", align 4
  %3 = inttoptr i64 %"51" to ptr
  %"64" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"64", ptr addrspace(5) %"43", align 8
  %"53" = load i64, ptr addrspace(5) %"42", align 4
  %"54" = load i64, ptr addrspace(5) %"43", align 4
  %"52" = add i64 %"53", %"54"
  store i64 %"52", ptr addrspace(5) %"44", align 4
  %"56" = load i64, ptr addrspace(5) %"42", align 4
  %"57" = load i64, ptr addrspace(5) %"43", align 4
  %"55" = sub i64 %"56", %"57"
  store i64 %"55", ptr addrspace(5) %"44", align 4
  %"59" = load i64, ptr addrspace(5) %"42", align 4
  %"66" = inttoptr i64 %"59" to ptr addrspace(1)
  %"58" = load i64, ptr addrspace(1) %"66", align 4
  store i64 %"58", ptr addrspace(5) %"45", align 4
  %"60" = load i64, ptr addrspace(5) %"43", align 4
  %"61" = load i64, ptr addrspace(5) %"45", align 4
  %"67" = inttoptr i64 %"60" to ptr addrspace(1)
  store i64 %"61", ptr addrspace(1) %"67", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }