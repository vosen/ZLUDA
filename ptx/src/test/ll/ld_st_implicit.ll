define amdgpu_kernel void @ld_st_implicit(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"37", align 8
  store i64 %2, ptr addrspace(5) %"39", align 8
  %3 = load i64, ptr addrspace(4) %"38", align 8
  store i64 %3, ptr addrspace(5) %"40", align 8
  store i64 81985529216486895, ptr addrspace(5) %"41", align 8
  %4 = load i64, ptr addrspace(5) %"39", align 8
  %"50" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load float, ptr addrspace(1) %"50", align 4
  %6 = bitcast float %5 to i32
  %"45" = zext i32 %6 to i64
  store i64 %"45", ptr addrspace(5) %"41", align 8
  %7 = load i64, ptr addrspace(5) %"40", align 8
  %8 = load i64, ptr addrspace(5) %"41", align 8
  %"51" = inttoptr i64 %7 to ptr addrspace(1)
  %9 = trunc i64 %8 to i32
  %"52" = bitcast i32 %9 to float
  store float %"52", ptr addrspace(1) %"51", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }