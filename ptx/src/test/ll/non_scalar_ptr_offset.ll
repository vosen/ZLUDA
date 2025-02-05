declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @non_scalar_ptr_offset(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"44" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"44", ptr addrspace(5) %"40", align 4
  %"45" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"45", ptr addrspace(5) %"41", align 4
  %"46" = load i64, ptr addrspace(5) %"40", align 4
  %"54" = inttoptr i64 %"46" to ptr addrspace(1)
  %"31" = getelementptr inbounds i8, ptr addrspace(1) %"54", i64 8
  %"29" = load <2 x i32>, ptr addrspace(1) %"31", align 8
  %"47" = extractelement <2 x i32> %"29", i8 0
  %"48" = extractelement <2 x i32> %"29", i8 1
  store i32 %"47", ptr addrspace(5) %"42", align 4
  store i32 %"48", ptr addrspace(5) %"43", align 4
  %"50" = load i32, ptr addrspace(5) %"42", align 4
  %"51" = load i32, ptr addrspace(5) %"43", align 4
  %"49" = add i32 %"50", %"51"
  store i32 %"49", ptr addrspace(5) %"42", align 4
  %"52" = load i64, ptr addrspace(5) %"41", align 4
  %"53" = load i32, ptr addrspace(5) %"42", align 4
  %"55" = inttoptr i64 %"52" to ptr addrspace(1)
  store i32 %"53", ptr addrspace(1) %"55", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
