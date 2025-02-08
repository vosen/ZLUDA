declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @setp_leu(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #0 {
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca float, align 4, addrspace(5)
  %"48" = alloca float, align 4, addrspace(5)
  %"49" = alloca float, align 4, addrspace(5)
  %"50" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"51" = load i64, ptr addrspace(4) %"43", align 4
  store i64 %"51", ptr addrspace(5) %"45", align 4
  %"52" = load i64, ptr addrspace(4) %"44", align 4
  store i64 %"52", ptr addrspace(5) %"46", align 4
  %"54" = load i64, ptr addrspace(5) %"45", align 4
  %"68" = inttoptr i64 %"54" to ptr
  %"53" = load float, ptr %"68", align 4
  store float %"53", ptr addrspace(5) %"47", align 4
  %"55" = load i64, ptr addrspace(5) %"45", align 4
  %"69" = inttoptr i64 %"55" to ptr
  %"36" = getelementptr inbounds i8, ptr %"69", i64 4
  %"56" = load float, ptr %"36", align 4
  store float %"56", ptr addrspace(5) %"48", align 4
  %"58" = load float, ptr addrspace(5) %"47", align 4
  %"59" = load float, ptr addrspace(5) %"48", align 4
  %"57" = fcmp ule float %"58", %"59"
  store i1 %"57", ptr addrspace(5) %"50", align 1
  %"60" = load i1, ptr addrspace(5) %"50", align 1
  br i1 %"60", label %"15", label %"16"

"15":                                             ; preds = %1
  %"62" = load float, ptr addrspace(5) %"47", align 4
  store float %"62", ptr addrspace(5) %"49", align 4
  br label %"16"

"16":                                             ; preds = %"15", %1
  %"63" = load i1, ptr addrspace(5) %"50", align 1
  br i1 %"63", label %"18", label %"17"

"17":                                             ; preds = %"16"
  %"65" = load float, ptr addrspace(5) %"48", align 4
  store float %"65", ptr addrspace(5) %"49", align 4
  br label %"18"

"18":                                             ; preds = %"17", %"16"
  %"66" = load i64, ptr addrspace(5) %"46", align 4
  %"67" = load float, ptr addrspace(5) %"49", align 4
  %"70" = inttoptr i64 %"66" to ptr
  store float %"67", ptr %"70", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
