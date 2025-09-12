define amdgpu_kernel void @warp_sz(ptr addrspace(4) byref(i64) %"32", ptr addrspace(4) byref(i64) %"33") #0 {
  %"34" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"31"

"31":                                             ; preds = %1
  %"35" = load i64, ptr addrspace(4) %"33", align 8
  store i64 %"35", ptr addrspace(5) %"34", align 8
  %"36" = load i64, ptr addrspace(5) %"34", align 8
  %"37" = inttoptr i64 %"36" to ptr
  store i8 32, ptr %"37", align 1
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }