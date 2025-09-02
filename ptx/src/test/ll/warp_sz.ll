@0 = addrspace(4) global i8 32

define amdgpu_kernel void @warp_sz(ptr addrspace(4) byref(i64) %"33", ptr addrspace(4) byref(i64) %"34") #0 {
  %"35" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"36" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"36", ptr addrspace(5) %"35", align 8
  %"31" = load i8, ptr addrspace(4) @0, align 1
  %"37" = load i64, ptr addrspace(5) %"35", align 8
  %"38" = inttoptr i64 %"37" to ptr
  store i8 %"31", ptr %"38", align 1
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }