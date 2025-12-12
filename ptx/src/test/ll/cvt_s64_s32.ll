define amdgpu_kernel void @cvt_s64_s32(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"37", align 8
  store i64 %2, ptr addrspace(5) %"39", align 8
  %3 = load i64, ptr addrspace(4) %"38", align 8
  store i64 %3, ptr addrspace(5) %"40", align 8
  %4 = load i64, ptr addrspace(5) %"39", align 8
  %"52" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"52", align 4
  store i32 %5, ptr addrspace(5) %"41", align 4
  %6 = load i32, ptr addrspace(5) %"41", align 4
  %7 = sext i32 %6 to i64
  store i64 %7, ptr addrspace(5) %"42", align 8
  %8 = load i64, ptr addrspace(5) %"40", align 8
  %9 = load i64, ptr addrspace(5) %"42", align 8
  %"53" = inttoptr i64 %8 to ptr
  store i64 %9, ptr %"53", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }