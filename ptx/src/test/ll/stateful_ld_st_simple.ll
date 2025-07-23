define amdgpu_kernel void @stateful_ld_st_simple(ptr addrspace(4) byref(i64) %"32", ptr addrspace(4) byref(i64) %"33") #0 {
  %"34" = alloca i64, align 8, addrspace(5)
  %"35" = alloca i64, align 8, addrspace(5)
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"31"

"31":                                             ; preds = %1
  %"39" = load i64, ptr addrspace(4) %"32", align 8
  store i64 %"39", ptr addrspace(5) %"34", align 8
  %"40" = load i64, ptr addrspace(4) %"33", align 8
  store i64 %"40", ptr addrspace(5) %"35", align 8
  %"42" = load i64, ptr addrspace(5) %"34", align 8
  %2 = inttoptr i64 %"42" to ptr
  %"49" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"49", ptr addrspace(5) %"36", align 8
  %"44" = load i64, ptr addrspace(5) %"35", align 8
  %3 = inttoptr i64 %"44" to ptr
  %"51" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"51", ptr addrspace(5) %"37", align 8
  %"46" = load i64, ptr addrspace(5) %"36", align 8
  %"53" = inttoptr i64 %"46" to ptr addrspace(1)
  %"45" = load i64, ptr addrspace(1) %"53", align 8
  store i64 %"45", ptr addrspace(5) %"38", align 8
  %"47" = load i64, ptr addrspace(5) %"37", align 8
  %"48" = load i64, ptr addrspace(5) %"38", align 8
  %"54" = inttoptr i64 %"47" to ptr addrspace(1)
  store i64 %"48", ptr addrspace(1) %"54", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }