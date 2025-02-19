declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @pred_not(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #0 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"53" = load i64, ptr addrspace(4) %"45", align 4
  store i64 %"53", ptr addrspace(5) %"47", align 4
  %"54" = load i64, ptr addrspace(4) %"46", align 4
  store i64 %"54", ptr addrspace(5) %"48", align 4
  %"56" = load i64, ptr addrspace(5) %"47", align 4
  %"70" = inttoptr i64 %"56" to ptr
  %"55" = load i64, ptr %"70", align 4
  store i64 %"55", ptr addrspace(5) %"49", align 4
  %"57" = load i64, ptr addrspace(5) %"47", align 4
  %"71" = inttoptr i64 %"57" to ptr
  %"36" = getelementptr inbounds i8, ptr %"71", i64 8
  %"58" = load i64, ptr %"36", align 4
  store i64 %"58", ptr addrspace(5) %"50", align 4
  %"60" = load i64, ptr addrspace(5) %"49", align 4
  %"61" = load i64, ptr addrspace(5) %"50", align 4
  %"59" = icmp ult i64 %"60", %"61"
  store i1 %"59", ptr addrspace(5) %"52", align 1
  %"63" = load i1, ptr addrspace(5) %"52", align 1
  %"62" = xor i1 %"63", true
  store i1 %"62", ptr addrspace(5) %"52", align 1
  %"64" = load i1, ptr addrspace(5) %"52", align 1
  br i1 %"64", label %"15", label %"16"

"15":                                             ; preds = %1
  store i64 1, ptr addrspace(5) %"51", align 4
  br label %"16"

"16":                                             ; preds = %"15", %1
  %"66" = load i1, ptr addrspace(5) %"52", align 1
  br i1 %"66", label %"18", label %"17"

"17":                                             ; preds = %"16"
  store i64 2, ptr addrspace(5) %"51", align 4
  br label %"18"

"18":                                             ; preds = %"17", %"16"
  %"68" = load i64, ptr addrspace(5) %"48", align 4
  %"69" = load i64, ptr addrspace(5) %"51", align 4
  %"72" = inttoptr i64 %"68" to ptr
  store i64 %"69", ptr %"72", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
