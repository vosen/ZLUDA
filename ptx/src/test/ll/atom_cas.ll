declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @atom_cas(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #1 {
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"69"

"69":                                             ; preds = %1
  %"49" = load i64, ptr addrspace(4) %"43", align 4
  store i64 %"49", ptr addrspace(5) %"45", align 4
  %"50" = load i64, ptr addrspace(4) %"44", align 4
  store i64 %"50", ptr addrspace(5) %"46", align 4
  %"52" = load i64, ptr addrspace(5) %"45", align 4
  %"62" = inttoptr i64 %"52" to ptr
  %"51" = load i32, ptr %"62", align 4
  store i32 %"51", ptr addrspace(5) %"47", align 4
  %"53" = load i64, ptr addrspace(5) %"45", align 4
  %"63" = inttoptr i64 %"53" to ptr
  %"31" = getelementptr inbounds i8, ptr %"63", i64 4
  %"55" = load i32, ptr addrspace(5) %"47", align 4
  %2 = cmpxchg ptr %"31", i32 %"55", i32 100 syncscope("agent-one-as") monotonic monotonic, align 4
  %"64" = extractvalue { i32, i1 } %2, 0
  store i32 %"64", ptr addrspace(5) %"47", align 4
  %"56" = load i64, ptr addrspace(5) %"45", align 4
  %"66" = inttoptr i64 %"56" to ptr
  %"34" = getelementptr inbounds i8, ptr %"66", i64 4
  %"57" = load i32, ptr %"34", align 4
  store i32 %"57", ptr addrspace(5) %"48", align 4
  %"58" = load i64, ptr addrspace(5) %"46", align 4
  %"59" = load i32, ptr addrspace(5) %"47", align 4
  %"67" = inttoptr i64 %"58" to ptr
  store i32 %"59", ptr %"67", align 4
  %"60" = load i64, ptr addrspace(5) %"46", align 4
  %"68" = inttoptr i64 %"60" to ptr
  %"36" = getelementptr inbounds i8, ptr %"68", i64 4
  %"61" = load i32, ptr addrspace(5) %"48", align 4
  store i32 %"61", ptr %"36", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }