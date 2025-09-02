@0 = addrspace(4) global i32 1

declare hidden void @__zluda_ptx_impl_nanosleep_u32(i32) #0

define amdgpu_kernel void @nanosleep(ptr addrspace(4) byref(i64) %"32", ptr addrspace(4) byref(i64) %"33") #1 {
  br label %1

1:                                                ; preds = %0
  br label %"31"

"31":                                             ; preds = %1
  %"30" = load i32, ptr addrspace(4) @0, align 4
  call void @__zluda_ptx_impl_nanosleep_u32(i32 %"30")
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }