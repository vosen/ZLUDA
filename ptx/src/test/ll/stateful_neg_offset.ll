declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @stateful_neg_offset(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #1 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"47" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"47", ptr addrspace(5) %"41", align 4
  %"48" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"48", ptr addrspace(5) %"42", align 4
  %"50" = load i64, ptr addrspace(5) %"41", align 4
  %2 = inttoptr i64 %"50" to ptr
  %"63" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"63", ptr addrspace(5) %"43", align 8
  %"52" = load i64, ptr addrspace(5) %"42", align 4
  %3 = inttoptr i64 %"52" to ptr
  %"65" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"65", ptr addrspace(5) %"44", align 8
  %"54" = load i64, ptr addrspace(5) %"43", align 4
  %"55" = load i64, ptr addrspace(5) %"44", align 4
  %"53" = add i64 %"54", %"55"
  store i64 %"53", ptr addrspace(5) %"45", align 4
  %"57" = load i64, ptr addrspace(5) %"43", align 4
  %"58" = load i64, ptr addrspace(5) %"44", align 4
  %"56" = sub i64 %"57", %"58"
  store i64 %"56", ptr addrspace(5) %"45", align 4
  %"60" = load i64, ptr addrspace(5) %"43", align 4
  %"67" = inttoptr i64 %"60" to ptr addrspace(1)
  %"59" = load i64, ptr addrspace(1) %"67", align 4
  store i64 %"59", ptr addrspace(5) %"46", align 4
  %"61" = load i64, ptr addrspace(5) %"44", align 4
  %"62" = load i64, ptr addrspace(5) %"46", align 4
  %"68" = inttoptr i64 %"61" to ptr addrspace(1)
  store i64 %"62", ptr addrspace(1) %"68", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }