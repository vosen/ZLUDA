declare i32 @__zluda_ptx_impl_cvt_rn_f16x2_e4m3x2(i16) #0

define amdgpu_kernel void @cvt_rn_f16x2_e4m3x2(ptr addrspace(4) byref(i64) %"31", ptr addrspace(4) byref(i64) %"32") #1 {
  %"33" = alloca i64, align 8, addrspace(5)
  %"34" = alloca i64, align 8, addrspace(5)
  %"35" = alloca i16, align 2, addrspace(5)
  %"36" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"30"

"30":                                             ; preds = %1
  %"37" = load i64, ptr addrspace(4) %"31", align 8
  store i64 %"37", ptr addrspace(5) %"33", align 8
  %"38" = load i64, ptr addrspace(4) %"32", align 8
  store i64 %"38", ptr addrspace(5) %"34", align 8
  %"40" = load i64, ptr addrspace(5) %"33", align 8
  %"45" = inttoptr i64 %"40" to ptr
  %"39" = load i16, ptr %"45", align 2
  store i16 %"39", ptr addrspace(5) %"35", align 2
  %"42" = load i16, ptr addrspace(5) %"35", align 2
  %"46" = call i32 @__zluda_ptx_impl_cvt_rn_f16x2_e4m3x2(i16 %"42")
  store i32 %"46", ptr addrspace(5) %"36", align 4
  %"43" = load i64, ptr addrspace(5) %"34", align 8
  %"44" = load i32, ptr addrspace(5) %"36", align 4
  %"48" = inttoptr i64 %"43" to ptr
  store i32 %"44", ptr %"48", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
