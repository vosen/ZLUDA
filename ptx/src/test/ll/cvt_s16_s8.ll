define amdgpu_kernel void @cvt_s16_s8(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %3, ptr addrspace(5) %"43", align 8
  %4 = load i64, ptr addrspace(5) %"42", align 8
  %"54" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i32, ptr addrspace(1) %"54", align 4
  store i32 %5, ptr addrspace(5) %"45", align 4
  %6 = load i32, ptr addrspace(5) %"45", align 4
  %7 = trunc i32 %6 to i8
  %8 = sext i8 %7 to i16
  %"50" = sext i16 %8 to i32
  store i32 %"50", ptr addrspace(5) %"44", align 4
  %9 = load i64, ptr addrspace(5) %"43", align 8
  %10 = load i32, ptr addrspace(5) %"44", align 4
  %"57" = inttoptr i64 %9 to ptr
  store i32 %10, ptr %"57", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
