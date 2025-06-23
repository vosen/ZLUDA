@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define amdgpu_kernel void @warp_sz(ptr addrspace(4) byref(i64) %"29", ptr addrspace(4) byref(i64) %"30") #0 {
  %"31" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"28"

"28":                                             ; preds = %1
  %"32" = load i64, ptr addrspace(4) %"30", align 4
  store i64 %"32", ptr addrspace(5) %"31", align 4
  %"33" = load i64, ptr addrspace(5) %"31", align 4
  %"34" = inttoptr i64 %"33" to ptr
  store i8 32, ptr %"34", align 1
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }