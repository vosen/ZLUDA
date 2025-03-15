declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @setp_gt(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #1 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca float, align 4, addrspace(5)
  %"50" = alloca float, align 4, addrspace(5)
  %"51" = alloca float, align 4, addrspace(5)
  %"52" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %"53" = load i64, ptr addrspace(4) %"45", align 4
  store i64 %"53", ptr addrspace(5) %"47", align 4
  %"54" = load i64, ptr addrspace(4) %"46", align 4
  store i64 %"54", ptr addrspace(5) %"48", align 4
  %"56" = load i64, ptr addrspace(5) %"47", align 4
  %"70" = inttoptr i64 %"56" to ptr
  %"55" = load float, ptr %"70", align 4
  store float %"55", ptr addrspace(5) %"49", align 4
  %"57" = load i64, ptr addrspace(5) %"47", align 4
  %"71" = inttoptr i64 %"57" to ptr
  %"37" = getelementptr inbounds i8, ptr %"71", i64 4
  %"58" = load float, ptr %"37", align 4
  store float %"58", ptr addrspace(5) %"50", align 4
  %"60" = load float, ptr addrspace(5) %"49", align 4
  %"61" = load float, ptr addrspace(5) %"50", align 4
  %"59" = fcmp ogt float %"60", %"61"
  store i1 %"59", ptr addrspace(5) %"52", align 1
  %"62" = load i1, ptr addrspace(5) %"52", align 1
  br i1 %"62", label %"16", label %"17"

"16":                                             ; preds = %"38"
  %"64" = load float, ptr addrspace(5) %"49", align 4
  store float %"64", ptr addrspace(5) %"51", align 4
  br label %"17"

"17":                                             ; preds = %"16", %"38"
  %"65" = load i1, ptr addrspace(5) %"52", align 1
  br i1 %"65", label %"19", label %"18"

"18":                                             ; preds = %"17"
  %"67" = load float, ptr addrspace(5) %"50", align 4
  store float %"67", ptr addrspace(5) %"51", align 4
  br label %"19"

"19":                                             ; preds = %"18", %"17"
  %"68" = load i64, ptr addrspace(5) %"48", align 4
  %"69" = load float, ptr addrspace(5) %"51", align 4
  %"72" = inttoptr i64 %"68" to ptr
  store float %"69", ptr %"72", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }