declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @stateful_ld_st_ntid(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"64" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"64", ptr addrspace(5) %"40", align 4
  %"65" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"65", ptr addrspace(5) %"41", align 4
  %"48" = load i64, ptr addrspace(5) %"40", align 4
  %2 = inttoptr i64 %"48" to ptr
  %"47" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"47", ptr addrspace(5) %"40", align 8
  %"50" = load i64, ptr addrspace(5) %"41", align 4
  %3 = inttoptr i64 %"50" to ptr
  %"49" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"49", ptr addrspace(5) %"41", align 8
  %"31" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"31", ptr addrspace(5) %"42", align 4
  %"53" = load i32, ptr addrspace(5) %"42", align 4
  %"52" = zext i32 %"53" to i64
  store i64 %"52", ptr addrspace(5) %"43", align 4
  %"55" = load i64, ptr addrspace(5) %"40", align 4
  %"56" = load i64, ptr addrspace(5) %"43", align 4
  %"66" = add i64 %"55", %"56"
  store i64 %"66", ptr addrspace(5) %"40", align 4
  %"58" = load i64, ptr addrspace(5) %"41", align 4
  %"59" = load i64, ptr addrspace(5) %"43", align 4
  %"68" = add i64 %"58", %"59"
  store i64 %"68", ptr addrspace(5) %"41", align 4
  %"61" = load i64, ptr addrspace(5) %"40", align 4
  %"70" = inttoptr i64 %"61" to ptr addrspace(1)
  %"60" = load i64, ptr addrspace(1) %"70", align 4
  store i64 %"60", ptr addrspace(5) %"44", align 4
  %"62" = load i64, ptr addrspace(5) %"41", align 4
  %"63" = load i64, ptr addrspace(5) %"44", align 4
  %"71" = inttoptr i64 %"62" to ptr addrspace(1)
  store i64 %"63", ptr addrspace(1) %"71", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
