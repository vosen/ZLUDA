declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @cvta(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #1 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"56"

"56":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"40", ptr addrspace(5) %"37", align 4
  %"41" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"41", ptr addrspace(5) %"38", align 4
  %"43" = load i64, ptr addrspace(5) %"37", align 4
  %2 = inttoptr i64 %"43" to ptr
  %"50" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"50", ptr addrspace(5) %"37", align 8
  %"45" = load i64, ptr addrspace(5) %"38", align 4
  %3 = inttoptr i64 %"45" to ptr
  %"52" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"52", ptr addrspace(5) %"38", align 8
  %"47" = load i64, ptr addrspace(5) %"37", align 4
  %"54" = inttoptr i64 %"47" to ptr addrspace(1)
  %"46" = load float, ptr addrspace(1) %"54", align 4
  store float %"46", ptr addrspace(5) %"39", align 4
  %"48" = load i64, ptr addrspace(5) %"38", align 4
  %"49" = load float, ptr addrspace(5) %"39", align 4
  %"55" = inttoptr i64 %"48" to ptr addrspace(1)
  store float %"49", ptr addrspace(1) %"55", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }