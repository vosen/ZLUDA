declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @bra(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #1 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"59"

"59":                                             ; preds = %1
  %"47" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"47", ptr addrspace(5) %"43", align 4
  %"48" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"48", ptr addrspace(5) %"44", align 4
  %"50" = load i64, ptr addrspace(5) %"43", align 4
  %"57" = inttoptr i64 %"50" to ptr
  %"49" = load i64, ptr %"57", align 4
  store i64 %"49", ptr addrspace(5) %"45", align 4
  br label %"10"

"10":                                             ; preds = %"59"
  %"52" = load i64, ptr addrspace(5) %"45", align 4
  %"51" = add i64 %"52", 1
  store i64 %"51", ptr addrspace(5) %"46", align 4
  br label %"12"

"11":                                             ; No predecessors!
  %"54" = load i64, ptr addrspace(5) %"45", align 4
  %"53" = add i64 %"54", 2
  store i64 %"53", ptr addrspace(5) %"46", align 4
  br label %"12"

"12":                                             ; preds = %"11", %"10"
  %"55" = load i64, ptr addrspace(5) %"44", align 4
  %"56" = load i64, ptr addrspace(5) %"46", align 4
  %"58" = inttoptr i64 %"55" to ptr
  store i64 %"56", ptr %"58", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }