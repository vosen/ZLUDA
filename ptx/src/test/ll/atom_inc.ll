declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @atom_inc(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #0 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"49" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"49", ptr addrspace(5) %"44", align 4
  %"50" = load i64, ptr addrspace(4) %"43", align 4
  store i64 %"50", ptr addrspace(5) %"45", align 4
  %"52" = load i64, ptr addrspace(5) %"44", align 4
  %"63" = inttoptr i64 %"52" to ptr
  %2 = atomicrmw uinc_wrap ptr %"63", i32 101 syncscope("agent-one-as") monotonic, align 4
  store i32 %2, ptr addrspace(5) %"46", align 4
  %"54" = load i64, ptr addrspace(5) %"44", align 4
  %"64" = inttoptr i64 %"54" to ptr addrspace(1)
  %3 = atomicrmw uinc_wrap ptr addrspace(1) %"64", i32 101 syncscope("agent-one-as") monotonic, align 4
  store i32 %3, ptr addrspace(5) %"47", align 4
  %"56" = load i64, ptr addrspace(5) %"44", align 4
  %"65" = inttoptr i64 %"56" to ptr
  %"55" = load i32, ptr %"65", align 4
  store i32 %"55", ptr addrspace(5) %"48", align 4
  %"57" = load i64, ptr addrspace(5) %"45", align 4
  %"58" = load i32, ptr addrspace(5) %"46", align 4
  %"66" = inttoptr i64 %"57" to ptr
  store i32 %"58", ptr %"66", align 4
  %"59" = load i64, ptr addrspace(5) %"45", align 4
  %"67" = inttoptr i64 %"59" to ptr
  %"33" = getelementptr inbounds i8, ptr %"67", i64 4
  %"60" = load i32, ptr addrspace(5) %"47", align 4
  store i32 %"60", ptr %"33", align 4
  %"61" = load i64, ptr addrspace(5) %"45", align 4
  %"68" = inttoptr i64 %"61" to ptr
  %"35" = getelementptr inbounds i8, ptr %"68", i64 8
  %"62" = load i32, ptr addrspace(5) %"48", align 4
  store i32 %"62", ptr %"35", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
