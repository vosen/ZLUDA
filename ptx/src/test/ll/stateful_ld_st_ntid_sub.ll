declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @stateful_ld_st_ntid_sub(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #0 {
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"76" = load i64, ptr addrspace(4) %"46", align 4
  store i64 %"76", ptr addrspace(5) %"48", align 4
  %"77" = load i64, ptr addrspace(4) %"47", align 4
  store i64 %"77", ptr addrspace(5) %"51", align 4
  %"60" = load i64, ptr addrspace(5) %"48", align 4
  %2 = inttoptr i64 %"60" to ptr
  %"59" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"59", ptr addrspace(5) %"49", align 8
  %"62" = load i64, ptr addrspace(5) %"51", align 4
  %3 = inttoptr i64 %"62" to ptr
  %"61" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"61", ptr addrspace(5) %"52", align 8
  %"35" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"35", ptr addrspace(5) %"54", align 4
  %"65" = load i32, ptr addrspace(5) %"54", align 4
  %"64" = zext i32 %"65" to i64
  store i64 %"64", ptr addrspace(5) %"55", align 4
  %"67" = load i64, ptr addrspace(5) %"49", align 4
  %"68" = load i64, ptr addrspace(5) %"55", align 4
  %"78" = sub i64 %"67", %"68"
  store i64 %"78", ptr addrspace(5) %"50", align 4
  %"70" = load i64, ptr addrspace(5) %"52", align 4
  %"71" = load i64, ptr addrspace(5) %"55", align 4
  %"81" = sub i64 %"70", %"71"
  store i64 %"81", ptr addrspace(5) %"53", align 4
  %"72" = load i64, ptr addrspace(5) %"50", align 4
  %"84" = inttoptr i64 %"72" to ptr addrspace(1)
  %"37" = getelementptr inbounds i8, ptr addrspace(1) %"84", i64 0
  %"73" = load i64, ptr addrspace(1) %"37", align 4
  store i64 %"73", ptr addrspace(5) %"56", align 4
  %"74" = load i64, ptr addrspace(5) %"53", align 4
  %"85" = inttoptr i64 %"74" to ptr addrspace(1)
  %"39" = getelementptr inbounds i8, ptr addrspace(1) %"85", i64 0
  %"75" = load i64, ptr addrspace(5) %"56", align 4
  store i64 %"75", ptr addrspace(1) %"39", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
