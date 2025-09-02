@0 = addrspace(4) global i64 81985529216486895

define amdgpu_kernel void @ld_st_implicit(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"40", ptr addrspace(5) %"37", align 8
  %"41" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"41", ptr addrspace(5) %"38", align 8
  %"33" = load i64, ptr addrspace(4) @0, align 8
  store i64 %"33", ptr addrspace(5) %"39", align 8
  %"44" = load i64, ptr addrspace(5) %"37", align 8
  %"48" = inttoptr i64 %"44" to ptr addrspace(1)
  %"47" = load float, ptr addrspace(1) %"48", align 4
  %2 = bitcast float %"47" to i32
  %"43" = zext i32 %2 to i64
  store i64 %"43", ptr addrspace(5) %"39", align 8
  %"45" = load i64, ptr addrspace(5) %"38", align 8
  %"46" = load i64, ptr addrspace(5) %"39", align 8
  %"49" = inttoptr i64 %"45" to ptr addrspace(1)
  %3 = trunc i64 %"46" to i32
  %"50" = bitcast i32 %3 to float
  store float %"50", ptr addrspace(1) %"49", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }