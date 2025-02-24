declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @div_approx(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #1 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca float, align 4, addrspace(5)
  %"43" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"58"

"58":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"44", ptr addrspace(5) %"40", align 4
  %"45" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"45", ptr addrspace(5) %"41", align 4
  %"47" = load i64, ptr addrspace(5) %"40", align 4
  %"55" = inttoptr i64 %"47" to ptr
  %"46" = load float, ptr %"55", align 4
  store float %"46", ptr addrspace(5) %"42", align 4
  %"48" = load i64, ptr addrspace(5) %"40", align 4
  %"56" = inttoptr i64 %"48" to ptr
  %"31" = getelementptr inbounds i8, ptr %"56", i64 4
  %"49" = load float, ptr %"31", align 4
  store float %"49", ptr addrspace(5) %"43", align 4
  %"51" = load float, ptr addrspace(5) %"42", align 4
  %"52" = load float, ptr addrspace(5) %"43", align 4
  %"50" = fdiv arcp afn float %"51", %"52"
  store float %"50", ptr addrspace(5) %"42", align 4
  %"53" = load i64, ptr addrspace(5) %"41", align 4
  %"54" = load float, ptr addrspace(5) %"42", align 4
  %"57" = inttoptr i64 %"53" to ptr
  store float %"54", ptr %"57", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }