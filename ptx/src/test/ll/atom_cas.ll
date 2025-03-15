declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @atom_cas(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #1 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"50" = load i64, ptr addrspace(4) %"44", align 4
  store i64 %"50", ptr addrspace(5) %"46", align 4
  %"51" = load i64, ptr addrspace(4) %"45", align 4
  store i64 %"51", ptr addrspace(5) %"47", align 4
  %"53" = load i64, ptr addrspace(5) %"46", align 4
  %"63" = inttoptr i64 %"53" to ptr
  %"52" = load i32, ptr %"63", align 4
  store i32 %"52", ptr addrspace(5) %"48", align 4
  %"54" = load i64, ptr addrspace(5) %"46", align 4
  %"64" = inttoptr i64 %"54" to ptr
  %"31" = getelementptr inbounds i8, ptr %"64", i64 4
  %"56" = load i32, ptr addrspace(5) %"48", align 4
  %2 = cmpxchg ptr %"31", i32 %"56", i32 100 syncscope("agent-one-as") monotonic monotonic, align 4
  %"65" = extractvalue { i32, i1 } %2, 0
  store i32 %"65", ptr addrspace(5) %"48", align 4
  %"57" = load i64, ptr addrspace(5) %"46", align 4
  %"67" = inttoptr i64 %"57" to ptr
  %"34" = getelementptr inbounds i8, ptr %"67", i64 4
  %"58" = load i32, ptr %"34", align 4
  store i32 %"58", ptr addrspace(5) %"49", align 4
  %"59" = load i64, ptr addrspace(5) %"47", align 4
  %"60" = load i32, ptr addrspace(5) %"48", align 4
  %"68" = inttoptr i64 %"59" to ptr
  store i32 %"60", ptr %"68", align 4
  %"61" = load i64, ptr addrspace(5) %"47", align 4
  %"69" = inttoptr i64 %"61" to ptr
  %"36" = getelementptr inbounds i8, ptr %"69", i64 4
  %"62" = load i32, ptr addrspace(5) %"49", align 4
  store i32 %"62", ptr %"36", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }