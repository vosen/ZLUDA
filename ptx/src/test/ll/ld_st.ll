define amdgpu_kernel void @ld_st(ptr addrspace(4) byref(i64) %"30", ptr addrspace(4) byref(i64) %"31") #0 {
  %"32" = alloca i64, align 8, addrspace(5)
  %"33" = alloca i64, align 8, addrspace(5)
  %"34" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"29"

"29":                                             ; preds = %1
  %"35" = load i64, ptr addrspace(4) %"30", align 4
  store i64 %"35", ptr addrspace(5) %"32", align 4
  %"36" = load i64, ptr addrspace(4) %"31", align 4
  store i64 %"36", ptr addrspace(5) %"33", align 4
  %"38" = load i64, ptr addrspace(5) %"32", align 4
  %"41" = inttoptr i64 %"38" to ptr
  %"37" = load i64, ptr %"41", align 4
  store i64 %"37", ptr addrspace(5) %"34", align 4
  %"39" = load i64, ptr addrspace(5) %"33", align 4
  %"40" = load i64, ptr addrspace(5) %"34", align 4
  %"42" = inttoptr i64 %"39" to ptr
  store i64 %"40", ptr %"42", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }