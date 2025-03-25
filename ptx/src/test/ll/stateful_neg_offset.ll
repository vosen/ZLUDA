define amdgpu_kernel void @stateful_neg_offset(ptr addrspace(4) byref(i64) %"33", ptr addrspace(4) byref(i64) %"34") #0 {
  %"35" = alloca i64, align 8, addrspace(5)
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"41" = load i64, ptr addrspace(4) %"33", align 4
  store i64 %"41", ptr addrspace(5) %"35", align 4
  %"42" = load i64, ptr addrspace(4) %"34", align 4
  store i64 %"42", ptr addrspace(5) %"36", align 4
  %"44" = load i64, ptr addrspace(5) %"35", align 4
  %2 = inttoptr i64 %"44" to ptr
  %"57" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"57", ptr addrspace(5) %"37", align 8
  %"46" = load i64, ptr addrspace(5) %"36", align 4
  %3 = inttoptr i64 %"46" to ptr
  %"59" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"59", ptr addrspace(5) %"38", align 8
  %"48" = load i64, ptr addrspace(5) %"37", align 4
  %"49" = load i64, ptr addrspace(5) %"38", align 4
  %"47" = add i64 %"48", %"49"
  store i64 %"47", ptr addrspace(5) %"39", align 4
  %"51" = load i64, ptr addrspace(5) %"37", align 4
  %"52" = load i64, ptr addrspace(5) %"38", align 4
  %"50" = sub i64 %"51", %"52"
  store i64 %"50", ptr addrspace(5) %"39", align 4
  %"54" = load i64, ptr addrspace(5) %"37", align 4
  %"61" = inttoptr i64 %"54" to ptr addrspace(1)
  %"53" = load i64, ptr addrspace(1) %"61", align 4
  store i64 %"53", ptr addrspace(5) %"40", align 4
  %"55" = load i64, ptr addrspace(5) %"38", align 4
  %"56" = load i64, ptr addrspace(5) %"40", align 4
  %"62" = inttoptr i64 %"55" to ptr addrspace(1)
  store i64 %"56", ptr addrspace(1) %"62", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }