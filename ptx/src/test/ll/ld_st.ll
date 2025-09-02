define amdgpu_kernel void @ld_st(ptr addrspace(4) byref(i64) %"33", ptr addrspace(4) byref(i64) %"34") #0 {
  %"35" = alloca i64, align 8, addrspace(5)
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"38" = load i64, ptr addrspace(4) %"33", align 8
  store i64 %"38", ptr addrspace(5) %"35", align 8
  %"39" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"39", ptr addrspace(5) %"36", align 8
  %"41" = load i64, ptr addrspace(5) %"35", align 8
  %"44" = inttoptr i64 %"41" to ptr
  %"40" = load i64, ptr %"44", align 8
  store i64 %"40", ptr addrspace(5) %"37", align 8
  %"42" = load i64, ptr addrspace(5) %"36", align 8
  %"43" = load i64, ptr addrspace(5) %"37", align 8
  %"45" = inttoptr i64 %"42" to ptr
  store i64 %"43", ptr %"45", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }