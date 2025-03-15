declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @shr(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #1 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"30"

"30":                                             ; preds = %1
  %"42" = load i64, ptr addrspace(4) %"37", align 4
  store i64 %"42", ptr addrspace(5) %"39", align 4
  %"43" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"43", ptr addrspace(5) %"40", align 4
  %"45" = load i64, ptr addrspace(5) %"39", align 4
  %"50" = inttoptr i64 %"45" to ptr
  %"44" = load i32, ptr %"50", align 4
  store i32 %"44", ptr addrspace(5) %"41", align 4
  %"47" = load i32, ptr addrspace(5) %"41", align 4
  %2 = ashr i32 %"47", 1
  %"46" = select i1 false, i32 0, i32 %2
  store i32 %"46", ptr addrspace(5) %"41", align 4
  %"48" = load i64, ptr addrspace(5) %"40", align 4
  %"49" = load i32, ptr addrspace(5) %"41", align 4
  %"51" = inttoptr i64 %"48" to ptr
  store i32 %"49", ptr %"51", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }