@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define amdgpu_kernel void @cvt_s16_s8(ptr addrspace(4) byref(i64) %"31", ptr addrspace(4) byref(i64) %"32") #0 {
  %"33" = alloca i64, align 8, addrspace(5)
  %"34" = alloca i64, align 8, addrspace(5)
  %"35" = alloca i32, align 4, addrspace(5)
  %"36" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"30"

"30":                                             ; preds = %1
  %"37" = load i64, ptr addrspace(4) %"31", align 4
  store i64 %"37", ptr addrspace(5) %"33", align 4
  %"38" = load i64, ptr addrspace(4) %"32", align 4
  store i64 %"38", ptr addrspace(5) %"34", align 4
  %"40" = load i64, ptr addrspace(5) %"33", align 4
  %"45" = inttoptr i64 %"40" to ptr addrspace(1)
  %"39" = load i32, ptr addrspace(1) %"45", align 4
  store i32 %"39", ptr addrspace(5) %"36", align 4
  %"42" = load i32, ptr addrspace(5) %"36", align 4
  %2 = trunc i32 %"42" to i8
  %"46" = sext i8 %2 to i16
  %"41" = sext i16 %"46" to i32
  store i32 %"41", ptr addrspace(5) %"35", align 4
  %"43" = load i64, ptr addrspace(5) %"34", align 4
  %"44" = load i32, ptr addrspace(5) %"35", align 4
  %"48" = inttoptr i64 %"43" to ptr
  store i32 %"44", ptr %"48", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }