declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @stateful_ld_st_simple(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #1 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"31"

"31":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"45", ptr addrspace(5) %"40", align 4
  %"46" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"46", ptr addrspace(5) %"41", align 4
  %"48" = load i64, ptr addrspace(5) %"40", align 4
  %2 = inttoptr i64 %"48" to ptr
  %"55" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"55", ptr addrspace(5) %"42", align 8
  %"50" = load i64, ptr addrspace(5) %"41", align 4
  %3 = inttoptr i64 %"50" to ptr
  %"57" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"57", ptr addrspace(5) %"43", align 8
  %"52" = load i64, ptr addrspace(5) %"42", align 4
  %"59" = inttoptr i64 %"52" to ptr addrspace(1)
  %"51" = load i64, ptr addrspace(1) %"59", align 4
  store i64 %"51", ptr addrspace(5) %"44", align 4
  %"53" = load i64, ptr addrspace(5) %"43", align 4
  %"54" = load i64, ptr addrspace(5) %"44", align 4
  %"60" = inttoptr i64 %"53" to ptr addrspace(1)
  store i64 %"54", ptr addrspace(1) %"60", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }