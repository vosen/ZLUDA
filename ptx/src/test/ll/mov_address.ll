define amdgpu_kernel void @mov_address(ptr addrspace(4) byref(i64) %"29", ptr addrspace(4) byref(i64) %"30") #0 {
  %"10" = alloca [8 x i8], align 1, addrspace(5)
  %"31" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"28"

"28":                                             ; preds = %1
  %"33" = ptrtoint ptr addrspace(5) %"10" to i64
  store i64 %"33", ptr addrspace(5) %"31", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }