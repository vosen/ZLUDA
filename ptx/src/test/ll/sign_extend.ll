define amdgpu_kernel void @sign_extend(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"36", align 8
  store i64 %2, ptr addrspace(5) %"38", align 8
  %3 = load i64, ptr addrspace(4) %"37", align 8
  store i64 %3, ptr addrspace(5) %"39", align 8
  %4 = load i64, ptr addrspace(5) %"38", align 8
  %"48" = inttoptr i64 %4 to ptr
  %5 = load i16, ptr %"48", align 2
  %"43" = sext i16 %5 to i32
  store i32 %"43", ptr addrspace(5) %"40", align 4
  %6 = load i64, ptr addrspace(5) %"39", align 8
  %7 = load i32, ptr addrspace(5) %"40", align 4
  %"49" = inttoptr i64 %6 to ptr
  store i32 %7, ptr %"49", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }