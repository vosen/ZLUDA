@shared_mem = external addrspace(3) global [1024 x i8], align 4

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @atom_add(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"46" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"46", ptr addrspace(5) %"42", align 4
  %"47" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"47", ptr addrspace(5) %"43", align 4
  %"49" = load i64, ptr addrspace(5) %"42", align 4
  %"60" = inttoptr i64 %"49" to ptr
  %"48" = load i32, ptr %"60", align 4
  store i32 %"48", ptr addrspace(5) %"44", align 4
  %"50" = load i64, ptr addrspace(5) %"42", align 4
  %"61" = inttoptr i64 %"50" to ptr
  %"31" = getelementptr inbounds i8, ptr %"61", i64 4
  %"51" = load i32, ptr %"31", align 4
  store i32 %"51", ptr addrspace(5) %"45", align 4
  %"52" = load i32, ptr addrspace(5) %"44", align 4
  store i32 %"52", ptr addrspace(3) @shared_mem, align 4
  %"54" = load i32, ptr addrspace(5) %"45", align 4
  %2 = atomicrmw add ptr addrspace(3) @shared_mem, i32 %"54" syncscope("agent-one-as") monotonic, align 4
  store i32 %2, ptr addrspace(5) %"44", align 4
  %"55" = load i32, ptr addrspace(3) @shared_mem, align 4
  store i32 %"55", ptr addrspace(5) %"45", align 4
  %"56" = load i64, ptr addrspace(5) %"43", align 4
  %"57" = load i32, ptr addrspace(5) %"44", align 4
  %"65" = inttoptr i64 %"56" to ptr
  store i32 %"57", ptr %"65", align 4
  %"58" = load i64, ptr addrspace(5) %"43", align 4
  %"66" = inttoptr i64 %"58" to ptr
  %"33" = getelementptr inbounds i8, ptr %"66", i64 4
  %"59" = load i32, ptr addrspace(5) %"45", align 4
  store i32 %"59", ptr %"33", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
