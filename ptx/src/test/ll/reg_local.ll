declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @reg_local(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"9" = alloca [8 x i8], align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"46" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"46", ptr addrspace(5) %"43", align 4
  %"47" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"47", ptr addrspace(5) %"44", align 4
  %"49" = load i64, ptr addrspace(5) %"43", align 4
  %"55" = inttoptr i64 %"49" to ptr addrspace(1)
  %"54" = load i64, ptr addrspace(1) %"55", align 4
  store i64 %"54", ptr addrspace(5) %"45", align 4
  %"50" = load i64, ptr addrspace(5) %"45", align 4
  %"30" = add i64 %"50", 1
  %"56" = addrspacecast ptr addrspace(5) %"9" to ptr
  store i64 %"30", ptr %"56", align 4
  %"58" = addrspacecast ptr addrspace(5) %"9" to ptr
  %"32" = getelementptr inbounds i8, ptr %"58", i64 0
  %"59" = load i64, ptr %"32", align 4
  store i64 %"59", ptr addrspace(5) %"45", align 4
  %"52" = load i64, ptr addrspace(5) %"44", align 4
  %"60" = inttoptr i64 %"52" to ptr addrspace(1)
  %"34" = getelementptr inbounds i8, ptr addrspace(1) %"60", i64 0
  %"53" = load i64, ptr addrspace(5) %"45", align 4
  store i64 %"53", ptr addrspace(1) %"34", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
