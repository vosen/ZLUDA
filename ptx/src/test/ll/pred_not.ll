declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @pred_not(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #1 {
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"74"

"74":                                             ; preds = %1
  %"54" = load i64, ptr addrspace(4) %"46", align 4
  store i64 %"54", ptr addrspace(5) %"48", align 4
  %"55" = load i64, ptr addrspace(4) %"47", align 4
  store i64 %"55", ptr addrspace(5) %"49", align 4
  %"57" = load i64, ptr addrspace(5) %"48", align 4
  %"71" = inttoptr i64 %"57" to ptr
  %"56" = load i64, ptr %"71", align 4
  store i64 %"56", ptr addrspace(5) %"50", align 4
  %"58" = load i64, ptr addrspace(5) %"48", align 4
  %"72" = inttoptr i64 %"58" to ptr
  %"37" = getelementptr inbounds i8, ptr %"72", i64 8
  %"59" = load i64, ptr %"37", align 4
  store i64 %"59", ptr addrspace(5) %"51", align 4
  %"61" = load i64, ptr addrspace(5) %"50", align 4
  %"62" = load i64, ptr addrspace(5) %"51", align 4
  %"60" = icmp ult i64 %"61", %"62"
  store i1 %"60", ptr addrspace(5) %"53", align 1
  %"64" = load i1, ptr addrspace(5) %"53", align 1
  %"63" = xor i1 %"64", true
  store i1 %"63", ptr addrspace(5) %"53", align 1
  %"65" = load i1, ptr addrspace(5) %"53", align 1
  br i1 %"65", label %"16", label %"17"

"16":                                             ; preds = %"74"
  store i64 1, ptr addrspace(5) %"52", align 4
  br label %"17"

"17":                                             ; preds = %"16", %"74"
  %"67" = load i1, ptr addrspace(5) %"53", align 1
  br i1 %"67", label %"19", label %"18"

"18":                                             ; preds = %"17"
  store i64 2, ptr addrspace(5) %"52", align 4
  br label %"19"

"19":                                             ; preds = %"18", %"17"
  %"69" = load i64, ptr addrspace(5) %"49", align 4
  %"70" = load i64, ptr addrspace(5) %"52", align 4
  %"73" = inttoptr i64 %"69" to ptr
  store i64 %"70", ptr %"73", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }