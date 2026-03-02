define amdgpu_kernel void @ld_st_implicit(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %3, ptr addrspace(5) %"43", align 8
  store i64 81985529216486895, ptr addrspace(5) %"44", align 8
  %4 = load i64, ptr addrspace(5) %"42", align 8
  %"53" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load float, ptr addrspace(1) %"53", align 4
  %6 = bitcast float %5 to i32
  %"48" = zext i32 %6 to i64
  store i64 %"48", ptr addrspace(5) %"44", align 8
  %7 = load i64, ptr addrspace(5) %"43", align 8
  %8 = load i64, ptr addrspace(5) %"44", align 8
  %"54" = inttoptr i64 %7 to ptr addrspace(1)
  %9 = trunc i64 %8 to i32
  %"55" = bitcast i32 %9 to float
  store float %"55", ptr addrspace(1) %"54", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
