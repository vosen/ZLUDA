@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define amdgpu_kernel void @ld_st_implicit(ptr addrspace(4) byref(i64) %"31", ptr addrspace(4) byref(i64) %"32") #0 {
  %"33" = alloca i64, align 8, addrspace(5)
  %"34" = alloca i64, align 8, addrspace(5)
  %"35" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"30"

"30":                                             ; preds = %1
  %"36" = load i64, ptr addrspace(4) %"31", align 4
  store i64 %"36", ptr addrspace(5) %"33", align 4
  %"37" = load i64, ptr addrspace(4) %"32", align 4
  store i64 %"37", ptr addrspace(5) %"34", align 4
  store i64 81985529216486895, ptr addrspace(5) %"35", align 4
  %"40" = load i64, ptr addrspace(5) %"33", align 4
  %"44" = inttoptr i64 %"40" to ptr addrspace(1)
  %"43" = load float, ptr addrspace(1) %"44", align 4
  %2 = bitcast float %"43" to i32
  %"39" = zext i32 %2 to i64
  store i64 %"39", ptr addrspace(5) %"35", align 4
  %"41" = load i64, ptr addrspace(5) %"34", align 4
  %"42" = load i64, ptr addrspace(5) %"35", align 4
  %"45" = inttoptr i64 %"41" to ptr addrspace(1)
  %3 = trunc i64 %"42" to i32
  %"46" = bitcast i32 %3 to float
  store float %"46", ptr addrspace(1) %"45", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }