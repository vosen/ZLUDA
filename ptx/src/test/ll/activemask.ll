declare hidden i32 @__zluda_ptx_impl_activemask() #0

define amdgpu_kernel void @activemask(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #1 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"36", align 8
  store i64 %2, ptr addrspace(5) %"37", align 8
  %"40" = call i32 @__zluda_ptx_impl_activemask()
  store i32 %"40", ptr addrspace(5) %"38", align 4
  %3 = load i64, ptr addrspace(5) %"37", align 8
  %4 = load i32, ptr addrspace(5) %"38", align 4
  %"43" = inttoptr i64 %3 to ptr
  store i32 %4, ptr %"43", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }