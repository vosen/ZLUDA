define amdgpu_kernel void @mov(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
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
  %"51" = inttoptr i64 %4 to ptr
  %5 = load i64, ptr %"51", align 8
  store i64 %5, ptr addrspace(5) %"41", align 8
  %6 = load i64, ptr addrspace(5) %"41", align 8
  store i64 %6, ptr addrspace(5) %"42", align 8
  %7 = load i64, ptr addrspace(5) %"40", align 8
  %8 = load i64, ptr addrspace(5) %"42", align 8
  %"52" = inttoptr i64 %7 to ptr
  store i64 %8, ptr %"52", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }