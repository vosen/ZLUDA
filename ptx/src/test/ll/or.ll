declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @or(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #1 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"45", ptr addrspace(5) %"41", align 4
  %"46" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"46", ptr addrspace(5) %"42", align 4
  %"48" = load i64, ptr addrspace(5) %"41", align 4
  %"56" = inttoptr i64 %"48" to ptr
  %"47" = load i64, ptr %"56", align 4
  store i64 %"47", ptr addrspace(5) %"43", align 4
  %"49" = load i64, ptr addrspace(5) %"41", align 4
  %"57" = inttoptr i64 %"49" to ptr
  %"31" = getelementptr inbounds i8, ptr %"57", i64 8
  %"50" = load i64, ptr %"31", align 4
  store i64 %"50", ptr addrspace(5) %"44", align 4
  %"52" = load i64, ptr addrspace(5) %"43", align 4
  %"53" = load i64, ptr addrspace(5) %"44", align 4
  %"58" = or i64 %"52", %"53"
  store i64 %"58", ptr addrspace(5) %"43", align 4
  %"54" = load i64, ptr addrspace(5) %"42", align 4
  %"55" = load i64, ptr addrspace(5) %"43", align 4
  %"61" = inttoptr i64 %"54" to ptr
  store i64 %"55", ptr %"61", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }