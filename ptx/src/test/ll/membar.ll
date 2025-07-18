@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define amdgpu_kernel void @membar(ptr addrspace(4) byref(i64) %"30", ptr addrspace(4) byref(i64) %"31") #0 {
  %"32" = alloca i64, align 8, addrspace(5)
  %"33" = alloca i64, align 8, addrspace(5)
  %"34" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"29"

"29":                                             ; preds = %1
  %"35" = load i64, ptr addrspace(4) %"30", align 4
  store i64 %"35", ptr addrspace(5) %"32", align 4
  %"36" = load i64, ptr addrspace(4) %"31", align 4
  store i64 %"36", ptr addrspace(5) %"33", align 4
  %"38" = load i64, ptr addrspace(5) %"32", align 4
  %"42" = inttoptr i64 %"38" to ptr
  %"41" = load i32, ptr %"42", align 4
  store i32 %"41", ptr addrspace(5) %"34", align 4
  fence seq_cst
  %"39" = load i64, ptr addrspace(5) %"33", align 4
  %"40" = load i32, ptr addrspace(5) %"34", align 4
  %"43" = inttoptr i64 %"39" to ptr
  store i32 %"40", ptr %"43", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }