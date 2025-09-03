declare hidden void @__zluda_ptx_impl_nanosleep_u32(i32) #0

define amdgpu_kernel void @nanosleep(ptr addrspace(4) byref(i64) %"31", ptr addrspace(4) byref(i64) %"32") #1 {
  %"33" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"33", align 4
  br label %1

1:                                                ; preds = %0
  br label %"30"

"30":                                             ; preds = %1
  %"34" = load i32, ptr addrspace(5) %"33", align 4
  call void @__zluda_ptx_impl_nanosleep_u32(i32 %"34")
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }