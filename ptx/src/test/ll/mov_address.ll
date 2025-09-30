define amdgpu_kernel void @mov_address(ptr addrspace(4) byref(i64) %"32", ptr addrspace(4) byref(i64) %"33") #0 {
  %"11" = alloca [8 x i8], align 1, addrspace(5)
  %"34" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"31"

"31":                                             ; preds = %1
  %"36" = ptrtoint ptr addrspace(5) %"11" to i64
  store i64 %"36", ptr addrspace(5) %"34", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
