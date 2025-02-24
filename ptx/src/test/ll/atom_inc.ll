declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @atom_inc(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #1 {
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"70"

"70":                                             ; preds = %1
  %"50" = load i64, ptr addrspace(4) %"43", align 4
  store i64 %"50", ptr addrspace(5) %"45", align 4
  %"51" = load i64, ptr addrspace(4) %"44", align 4
  store i64 %"51", ptr addrspace(5) %"46", align 4
  %"53" = load i64, ptr addrspace(5) %"45", align 4
  %"64" = inttoptr i64 %"53" to ptr
  %2 = atomicrmw uinc_wrap ptr %"64", i32 101 syncscope("agent-one-as") monotonic, align 4
  store i32 %2, ptr addrspace(5) %"47", align 4
  %"55" = load i64, ptr addrspace(5) %"45", align 4
  %"65" = inttoptr i64 %"55" to ptr addrspace(1)
  %3 = atomicrmw uinc_wrap ptr addrspace(1) %"65", i32 101 syncscope("agent-one-as") monotonic, align 4
  store i32 %3, ptr addrspace(5) %"48", align 4
  %"57" = load i64, ptr addrspace(5) %"45", align 4
  %"66" = inttoptr i64 %"57" to ptr
  %"56" = load i32, ptr %"66", align 4
  store i32 %"56", ptr addrspace(5) %"49", align 4
  %"58" = load i64, ptr addrspace(5) %"46", align 4
  %"59" = load i32, ptr addrspace(5) %"47", align 4
  %"67" = inttoptr i64 %"58" to ptr
  store i32 %"59", ptr %"67", align 4
  %"60" = load i64, ptr addrspace(5) %"46", align 4
  %"68" = inttoptr i64 %"60" to ptr
  %"34" = getelementptr inbounds i8, ptr %"68", i64 4
  %"61" = load i32, ptr addrspace(5) %"48", align 4
  store i32 %"61", ptr %"34", align 4
  %"62" = load i64, ptr addrspace(5) %"46", align 4
  %"69" = inttoptr i64 %"62" to ptr
  %"36" = getelementptr inbounds i8, ptr %"69", i64 8
  %"63" = load i32, ptr addrspace(5) %"49", align 4
  store i32 %"63", ptr %"36", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }