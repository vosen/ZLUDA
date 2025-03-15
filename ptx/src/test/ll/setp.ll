declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @setp(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #1 {
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %"55" = load i64, ptr addrspace(4) %"47", align 4
  store i64 %"55", ptr addrspace(5) %"49", align 4
  %"56" = load i64, ptr addrspace(4) %"48", align 4
  store i64 %"56", ptr addrspace(5) %"50", align 4
  %"58" = load i64, ptr addrspace(5) %"49", align 4
  %"70" = inttoptr i64 %"58" to ptr
  %"57" = load i64, ptr %"70", align 4
  store i64 %"57", ptr addrspace(5) %"51", align 4
  %"59" = load i64, ptr addrspace(5) %"49", align 4
  %"71" = inttoptr i64 %"59" to ptr
  %"37" = getelementptr inbounds i8, ptr %"71", i64 8
  %"60" = load i64, ptr %"37", align 4
  store i64 %"60", ptr addrspace(5) %"52", align 4
  %"62" = load i64, ptr addrspace(5) %"51", align 4
  %"63" = load i64, ptr addrspace(5) %"52", align 4
  %"61" = icmp ult i64 %"62", %"63"
  store i1 %"61", ptr addrspace(5) %"54", align 1
  %"64" = load i1, ptr addrspace(5) %"54", align 1
  br i1 %"64", label %"16", label %"17"

"16":                                             ; preds = %"40"
  store i64 1, ptr addrspace(5) %"53", align 4
  br label %"17"

"17":                                             ; preds = %"16", %"40"
  %"66" = load i1, ptr addrspace(5) %"54", align 1
  br i1 %"66", label %"19", label %"18"

"18":                                             ; preds = %"17"
  store i64 2, ptr addrspace(5) %"53", align 4
  br label %"19"

"19":                                             ; preds = %"18", %"17"
  %"68" = load i64, ptr addrspace(5) %"50", align 4
  %"69" = load i64, ptr addrspace(5) %"53", align 4
  %"72" = inttoptr i64 %"68" to ptr
  store i64 %"69", ptr %"72", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }