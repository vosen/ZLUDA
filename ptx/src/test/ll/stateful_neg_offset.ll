define amdgpu_kernel void @stateful_neg_offset(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"44", ptr addrspace(5) %"38", align 8
  %"45" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"45", ptr addrspace(5) %"39", align 8
  %"47" = load i64, ptr addrspace(5) %"38", align 8
  %2 = inttoptr i64 %"47" to ptr
  %"60" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"60", ptr addrspace(5) %"40", align 8
  %"49" = load i64, ptr addrspace(5) %"39", align 8
  %3 = inttoptr i64 %"49" to ptr
  %"62" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"62", ptr addrspace(5) %"41", align 8
  %"51" = load i64, ptr addrspace(5) %"40", align 8
  %"52" = load i64, ptr addrspace(5) %"41", align 8
  %"50" = add i64 %"51", %"52"
  store i64 %"50", ptr addrspace(5) %"42", align 8
  %"54" = load i64, ptr addrspace(5) %"40", align 8
  %"55" = load i64, ptr addrspace(5) %"41", align 8
  %"53" = sub i64 %"54", %"55"
  store i64 %"53", ptr addrspace(5) %"42", align 8
  %"57" = load i64, ptr addrspace(5) %"40", align 8
  %"64" = inttoptr i64 %"57" to ptr addrspace(1)
  %"56" = load i64, ptr addrspace(1) %"64", align 8
  store i64 %"56", ptr addrspace(5) %"43", align 8
  %"58" = load i64, ptr addrspace(5) %"41", align 8
  %"59" = load i64, ptr addrspace(5) %"43", align 8
  %"65" = inttoptr i64 %"58" to ptr addrspace(1)
  store i64 %"59", ptr addrspace(1) %"65", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
