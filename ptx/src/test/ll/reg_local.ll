@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define amdgpu_kernel void @reg_local(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"10" = alloca [8 x i8], align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"42" = load i64, ptr addrspace(4) %"37", align 4
  store i64 %"42", ptr addrspace(5) %"39", align 4
  %"43" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"43", ptr addrspace(5) %"40", align 4
  %"45" = load i64, ptr addrspace(5) %"39", align 4
  %"51" = inttoptr i64 %"45" to ptr addrspace(1)
  %"50" = load i64, ptr addrspace(1) %"51", align 4
  store i64 %"50", ptr addrspace(5) %"41", align 4
  %"46" = load i64, ptr addrspace(5) %"41", align 4
  %"31" = add i64 %"46", 1
  %"52" = addrspacecast ptr addrspace(5) %"10" to ptr
  store i64 %"31", ptr %"52", align 4
  %"54" = addrspacecast ptr addrspace(5) %"10" to ptr
  %"33" = getelementptr inbounds i8, ptr %"54", i64 0
  %"55" = load i64, ptr %"33", align 4
  store i64 %"55", ptr addrspace(5) %"41", align 4
  %"48" = load i64, ptr addrspace(5) %"40", align 4
  %"56" = inttoptr i64 %"48" to ptr addrspace(1)
  %"35" = getelementptr inbounds i8, ptr addrspace(1) %"56", i64 0
  %"49" = load i64, ptr addrspace(5) %"41", align 4
  store i64 %"49", ptr addrspace(1) %"35", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }