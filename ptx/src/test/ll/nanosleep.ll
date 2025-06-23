@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

declare void @__zluda_ptx_impl_nanosleep_u32(i32) #0

define amdgpu_kernel void @nanosleep(ptr addrspace(4) byref(i64) %"28", ptr addrspace(4) byref(i64) %"29") #1 {
  br label %1

1:                                                ; preds = %0
  br label %"27"

"27":                                             ; preds = %1
  call void @__zluda_ptx_impl_nanosleep_u32(i32 1)
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }