@foobar = addrspace(1) global [4 x i32] [i32 1, i32 0, i32 0, i32 0]

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @global_array(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i64 ptrtoint (ptr addrspace(1) @foobar to i64), ptr addrspace(5) %"37", align 4
  %"41" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"41", ptr addrspace(5) %"38", align 4
  %"43" = load i64, ptr addrspace(5) %"37", align 4
  %"47" = inttoptr i64 %"43" to ptr addrspace(1)
  %"42" = load i32, ptr addrspace(1) %"47", align 4
  store i32 %"42", ptr addrspace(5) %"39", align 4
  %"44" = load i64, ptr addrspace(5) %"38", align 4
  %"45" = load i32, ptr addrspace(5) %"39", align 4
  %"48" = inttoptr i64 %"44" to ptr addrspace(1)
  store i32 %"45", ptr addrspace(1) %"48", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
