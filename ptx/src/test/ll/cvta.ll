declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @cvta(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #1 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"29"

"29":                                             ; preds = %1
  %"41" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"41", ptr addrspace(5) %"38", align 4
  %"42" = load i64, ptr addrspace(4) %"37", align 4
  store i64 %"42", ptr addrspace(5) %"39", align 4
  %"44" = load i64, ptr addrspace(5) %"38", align 4
  %2 = inttoptr i64 %"44" to ptr
  %"51" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"51", ptr addrspace(5) %"38", align 8
  %"46" = load i64, ptr addrspace(5) %"39", align 4
  %3 = inttoptr i64 %"46" to ptr
  %"53" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"53", ptr addrspace(5) %"39", align 8
  %"48" = load i64, ptr addrspace(5) %"38", align 4
  %"55" = inttoptr i64 %"48" to ptr addrspace(1)
  %"47" = load float, ptr addrspace(1) %"55", align 4
  store float %"47", ptr addrspace(5) %"40", align 4
  %"49" = load i64, ptr addrspace(5) %"39", align 4
  %"50" = load float, ptr addrspace(5) %"40", align 4
  %"56" = inttoptr i64 %"49" to ptr addrspace(1)
  store float %"50", ptr addrspace(1) %"56", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }