declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @setp_gt(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #1 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca float, align 4, addrspace(5)
  %"49" = alloca float, align 4, addrspace(5)
  %"50" = alloca float, align 4, addrspace(5)
  %"51" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"72"

"72":                                             ; preds = %1
  %"52" = load i64, ptr addrspace(4) %"44", align 4
  store i64 %"52", ptr addrspace(5) %"46", align 4
  %"53" = load i64, ptr addrspace(4) %"45", align 4
  store i64 %"53", ptr addrspace(5) %"47", align 4
  %"55" = load i64, ptr addrspace(5) %"46", align 4
  %"69" = inttoptr i64 %"55" to ptr
  %"54" = load float, ptr %"69", align 4
  store float %"54", ptr addrspace(5) %"48", align 4
  %"56" = load i64, ptr addrspace(5) %"46", align 4
  %"70" = inttoptr i64 %"56" to ptr
  %"37" = getelementptr inbounds i8, ptr %"70", i64 4
  %"57" = load float, ptr %"37", align 4
  store float %"57", ptr addrspace(5) %"49", align 4
  %"59" = load float, ptr addrspace(5) %"48", align 4
  %"60" = load float, ptr addrspace(5) %"49", align 4
  %"58" = fcmp ogt float %"59", %"60"
  store i1 %"58", ptr addrspace(5) %"51", align 1
  %"61" = load i1, ptr addrspace(5) %"51", align 1
  br i1 %"61", label %"16", label %"17"

"16":                                             ; preds = %"72"
  %"63" = load float, ptr addrspace(5) %"48", align 4
  store float %"63", ptr addrspace(5) %"50", align 4
  br label %"17"

"17":                                             ; preds = %"16", %"72"
  %"64" = load i1, ptr addrspace(5) %"51", align 1
  br i1 %"64", label %"19", label %"18"

"18":                                             ; preds = %"17"
  %"66" = load float, ptr addrspace(5) %"49", align 4
  store float %"66", ptr addrspace(5) %"50", align 4
  br label %"19"

"19":                                             ; preds = %"18", %"17"
  %"67" = load i64, ptr addrspace(5) %"47", align 4
  %"68" = load float, ptr addrspace(5) %"50", align 4
  %"71" = inttoptr i64 %"67" to ptr
  store float %"68", ptr %"71", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }