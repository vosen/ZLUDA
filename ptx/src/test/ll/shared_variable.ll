@shared_mem1 = external addrspace(3) global [128 x i8], align 4

define amdgpu_kernel void @shared_variable(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"41" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"41", ptr addrspace(5) %"37", align 8
  %"42" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"42", ptr addrspace(5) %"38", align 8
  %"44" = load i64, ptr addrspace(5) %"37", align 8
  %"49" = inttoptr i64 %"44" to ptr addrspace(1)
  %"43" = load i64, ptr addrspace(1) %"49", align 8
  store i64 %"43", ptr addrspace(5) %"39", align 8
  %"45" = load i64, ptr addrspace(5) %"39", align 8
  store i64 %"45", ptr addrspace(3) @shared_mem1, align 8
  %"46" = load i64, ptr addrspace(3) @shared_mem1, align 8
  store i64 %"46", ptr addrspace(5) %"40", align 8
  %"47" = load i64, ptr addrspace(5) %"38", align 8
  %"48" = load i64, ptr addrspace(5) %"40", align 8
  %"52" = inttoptr i64 %"47" to ptr addrspace(1)
  store i64 %"48", ptr addrspace(1) %"52", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }