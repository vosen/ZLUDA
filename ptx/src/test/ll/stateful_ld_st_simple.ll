define amdgpu_kernel void @stateful_ld_st_simple(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"42" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"42", ptr addrspace(5) %"37", align 8
  %"43" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"43", ptr addrspace(5) %"38", align 8
  %"45" = load i64, ptr addrspace(5) %"37", align 8
  %2 = inttoptr i64 %"45" to ptr
  %"52" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"52", ptr addrspace(5) %"39", align 8
  %"47" = load i64, ptr addrspace(5) %"38", align 8
  %3 = inttoptr i64 %"47" to ptr
  %"54" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"54", ptr addrspace(5) %"40", align 8
  %"49" = load i64, ptr addrspace(5) %"39", align 8
  %"56" = inttoptr i64 %"49" to ptr addrspace(1)
  %"48" = load i64, ptr addrspace(1) %"56", align 8
  store i64 %"48", ptr addrspace(5) %"41", align 8
  %"50" = load i64, ptr addrspace(5) %"40", align 8
  %"51" = load i64, ptr addrspace(5) %"41", align 8
  %"57" = inttoptr i64 %"50" to ptr addrspace(1)
  store i64 %"51", ptr addrspace(1) %"57", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }