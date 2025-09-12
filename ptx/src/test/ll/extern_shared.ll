@shared_mem = external addrspace(3) global [0 x i32]

define amdgpu_kernel void @extern_shared(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"39" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"39", ptr addrspace(5) %"36", align 8
  %"40" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"40", ptr addrspace(5) %"37", align 8
  %"42" = load i64, ptr addrspace(5) %"36", align 8
  %"47" = inttoptr i64 %"42" to ptr addrspace(1)
  %"41" = load i64, ptr addrspace(1) %"47", align 8
  store i64 %"41", ptr addrspace(5) %"38", align 8
  %"43" = load i64, ptr addrspace(5) %"38", align 8
  store i64 %"43", ptr addrspace(3) @shared_mem, align 8
  %"44" = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %"44", ptr addrspace(5) %"38", align 8
  %"45" = load i64, ptr addrspace(5) %"37", align 8
  %"46" = load i64, ptr addrspace(5) %"38", align 8
  %"50" = inttoptr i64 %"45" to ptr addrspace(1)
  store i64 %"46", ptr addrspace(1) %"50", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }