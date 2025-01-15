declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @prmt(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"43" = load i64, ptr addrspace(4) %"37", align 4
  store i64 %"43", ptr addrspace(5) %"39", align 4
  %"44" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"44", ptr addrspace(5) %"40", align 4
  %"46" = load i64, ptr addrspace(5) %"39", align 4
  %"54" = inttoptr i64 %"46" to ptr
  %"45" = load i32, ptr %"54", align 4
  store i32 %"45", ptr addrspace(5) %"41", align 4
  %"47" = load i64, ptr addrspace(5) %"39", align 4
  %"55" = inttoptr i64 %"47" to ptr
  %"30" = getelementptr inbounds i8, ptr %"55", i64 4
  %"48" = load i32, ptr %"30", align 4
  store i32 %"48", ptr addrspace(5) %"42", align 4
  %"50" = load i32, ptr addrspace(5) %"41", align 4
  %"51" = load i32, ptr addrspace(5) %"42", align 4
  %2 = bitcast i32 %"50" to <4 x i8>
  %3 = bitcast i32 %"51" to <4 x i8>
  %"56" = shufflevector <4 x i8> %2, <4 x i8> %3, <4 x i32> <i32 4, i32 0, i32 6, i32 7>
  store <4 x i8> %"56", ptr addrspace(5) %"42", align 4
  %"52" = load i64, ptr addrspace(5) %"40", align 4
  %"53" = load i32, ptr addrspace(5) %"42", align 4
  %"59" = inttoptr i64 %"52" to ptr
  store i32 %"53", ptr %"59", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
