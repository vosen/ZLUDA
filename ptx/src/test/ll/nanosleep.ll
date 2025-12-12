declare hidden void @__zluda_ptx_impl_nanosleep_u32(i32) #0

define amdgpu_kernel void @nanosleep(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #1 {
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  call void @__zluda_ptx_impl_nanosleep_u32(i32 1)
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }