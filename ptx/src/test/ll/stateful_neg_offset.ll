declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @stateful_neg_offset(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"45" = load i64, ptr addrspace(4) %"37", align 4
  store i64 %"45", ptr addrspace(5) %"39", align 4
  %"46" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"46", ptr addrspace(5) %"40", align 4
  %"48" = load i64, ptr addrspace(5) %"39", align 4
  %2 = inttoptr i64 %"48" to ptr
  %"61" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"61", ptr addrspace(5) %"41", align 8
  %"50" = load i64, ptr addrspace(5) %"40", align 4
  %3 = inttoptr i64 %"50" to ptr
  %"63" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"63", ptr addrspace(5) %"42", align 8
  %"52" = load i64, ptr addrspace(5) %"41", align 4
  %"53" = load i64, ptr addrspace(5) %"42", align 4
  %"51" = add i64 %"52", %"53"
  store i64 %"51", ptr addrspace(5) %"43", align 4
  %"55" = load i64, ptr addrspace(5) %"41", align 4
  %"56" = load i64, ptr addrspace(5) %"42", align 4
  %"54" = sub i64 %"55", %"56"
  store i64 %"54", ptr addrspace(5) %"43", align 4
  %"58" = load i64, ptr addrspace(5) %"41", align 4
  %"65" = inttoptr i64 %"58" to ptr addrspace(1)
  %"57" = load i64, ptr addrspace(1) %"65", align 4
  store i64 %"57", ptr addrspace(5) %"44", align 4
  %"59" = load i64, ptr addrspace(5) %"42", align 4
  %"60" = load i64, ptr addrspace(5) %"44", align 4
  %"66" = inttoptr i64 %"59" to ptr addrspace(1)
  store i64 %"60", ptr addrspace(1) %"66", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
