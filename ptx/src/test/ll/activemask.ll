declare hidden i32 @__zluda_ptx_impl_activemask() #0

define amdgpu_kernel void @activemask(ptr addrspace(4) byref(i64) %"32", ptr addrspace(4) byref(i64) %"33") #1 {
  %"34" = alloca i64, align 8, addrspace(5)
  %"35" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"31"

"31":                                             ; preds = %1
  %"36" = load i64, ptr addrspace(4) %"33", align 8
  store i64 %"36", ptr addrspace(5) %"34", align 8
  %"37" = call i32 @__zluda_ptx_impl_activemask()
  store i32 %"37", ptr addrspace(5) %"35", align 4
  %"38" = load i64, ptr addrspace(5) %"34", align 8
  %"39" = load i32, ptr addrspace(5) %"35", align 4
  %"40" = inttoptr i64 %"38" to ptr
  store i32 %"39", ptr %"40", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }