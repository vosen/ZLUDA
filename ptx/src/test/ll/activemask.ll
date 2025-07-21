declare i32 @__zluda_ptx_impl_activemask() #0

define amdgpu_kernel void @activemask(ptr addrspace(4) byref(i64) %"29", ptr addrspace(4) byref(i64) %"30") #1 {
  %"31" = alloca i64, align 8, addrspace(5)
  %"32" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"28"

"28":                                             ; preds = %1
  %"33" = load i64, ptr addrspace(4) %"30", align 4
  store i64 %"33", ptr addrspace(5) %"31", align 4
  %"34" = call i32 @__zluda_ptx_impl_activemask()
  store i32 %"34", ptr addrspace(5) %"32", align 4
  %"35" = load i64, ptr addrspace(5) %"31", align 4
  %"36" = load i32, ptr addrspace(5) %"32", align 4
  %"37" = inttoptr i64 %"35" to ptr
  store i32 %"36", ptr %"37", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }