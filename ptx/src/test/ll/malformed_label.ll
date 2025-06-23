@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define amdgpu_kernel void @malformed_label(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(4) %"34", align 4
  store i64 %"40", ptr addrspace(5) %"36", align 4
  %"41" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"41", ptr addrspace(5) %"37", align 4
  br label %"10"

"10":                                             ; preds = %"32"
  %"43" = load i64, ptr addrspace(5) %"36", align 4
  %"48" = inttoptr i64 %"43" to ptr
  %"42" = load i64, ptr %"48", align 4
  store i64 %"42", ptr addrspace(5) %"38", align 4
  %"45" = load i64, ptr addrspace(5) %"38", align 4
  %"44" = add i64 %"45", 1
  store i64 %"44", ptr addrspace(5) %"39", align 4
  %"46" = load i64, ptr addrspace(5) %"37", align 4
  %"47" = load i64, ptr addrspace(5) %"39", align 4
  %"49" = inttoptr i64 %"46" to ptr
  store i64 %"47", ptr %"49", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }