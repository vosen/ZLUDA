define amdgpu_kernel void @ld_st_implicit(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  store i64 81985529216486895, ptr addrspace(5) %"41", align 4
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"39" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"39", ptr addrspace(5) %"36", align 8
  %"40" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"40", ptr addrspace(5) %"37", align 8
  %"43" = load i64, ptr addrspace(5) %"41", align 8
  store i64 %"43", ptr addrspace(5) %"38", align 8
  %"45" = load i64, ptr addrspace(5) %"36", align 8
  %"49" = inttoptr i64 %"45" to ptr addrspace(1)
  %"48" = load float, ptr addrspace(1) %"49", align 4
  %2 = bitcast float %"48" to i32
  %"44" = zext i32 %2 to i64
  store i64 %"44", ptr addrspace(5) %"38", align 8
  %"46" = load i64, ptr addrspace(5) %"37", align 8
  %"47" = load i64, ptr addrspace(5) %"38", align 8
  %"50" = inttoptr i64 %"46" to ptr addrspace(1)
  %3 = trunc i64 %"47" to i32
  %"51" = bitcast i32 %3 to float
  store float %"51", ptr addrspace(1) %"50", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }