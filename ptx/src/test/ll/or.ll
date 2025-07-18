@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define amdgpu_kernel void @or(ptr addrspace(4) byref(i64) %"33", ptr addrspace(4) byref(i64) %"34") #0 {
  %"35" = alloca i64, align 8, addrspace(5)
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"39" = load i64, ptr addrspace(4) %"33", align 4
  store i64 %"39", ptr addrspace(5) %"35", align 4
  %"40" = load i64, ptr addrspace(4) %"34", align 4
  store i64 %"40", ptr addrspace(5) %"36", align 4
  %"42" = load i64, ptr addrspace(5) %"35", align 4
  %"50" = inttoptr i64 %"42" to ptr
  %"41" = load i64, ptr %"50", align 4
  store i64 %"41", ptr addrspace(5) %"37", align 4
  %"43" = load i64, ptr addrspace(5) %"35", align 4
  %"51" = inttoptr i64 %"43" to ptr
  %"31" = getelementptr inbounds i8, ptr %"51", i64 8
  %"44" = load i64, ptr %"31", align 4
  store i64 %"44", ptr addrspace(5) %"38", align 4
  %"46" = load i64, ptr addrspace(5) %"37", align 4
  %"47" = load i64, ptr addrspace(5) %"38", align 4
  %"52" = or i64 %"46", %"47"
  store i64 %"52", ptr addrspace(5) %"37", align 4
  %"48" = load i64, ptr addrspace(5) %"36", align 4
  %"49" = load i64, ptr addrspace(5) %"37", align 4
  %"55" = inttoptr i64 %"48" to ptr
  store i64 %"49", ptr %"55", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }