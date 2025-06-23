@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define amdgpu_kernel void @atom_cas(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"44", ptr addrspace(5) %"40", align 4
  %"45" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"45", ptr addrspace(5) %"41", align 4
  %"47" = load i64, ptr addrspace(5) %"40", align 4
  %"57" = inttoptr i64 %"47" to ptr
  %"46" = load i32, ptr %"57", align 4
  store i32 %"46", ptr addrspace(5) %"42", align 4
  %"48" = load i64, ptr addrspace(5) %"40", align 4
  %"58" = inttoptr i64 %"48" to ptr
  %"31" = getelementptr inbounds i8, ptr %"58", i64 4
  %"50" = load i32, ptr addrspace(5) %"42", align 4
  %2 = cmpxchg ptr %"31", i32 %"50", i32 100 syncscope("agent-one-as") monotonic monotonic, align 4
  %"59" = extractvalue { i32, i1 } %2, 0
  store i32 %"59", ptr addrspace(5) %"42", align 4
  %"51" = load i64, ptr addrspace(5) %"40", align 4
  %"61" = inttoptr i64 %"51" to ptr
  %"34" = getelementptr inbounds i8, ptr %"61", i64 4
  %"52" = load i32, ptr %"34", align 4
  store i32 %"52", ptr addrspace(5) %"43", align 4
  %"53" = load i64, ptr addrspace(5) %"41", align 4
  %"54" = load i32, ptr addrspace(5) %"42", align 4
  %"62" = inttoptr i64 %"53" to ptr
  store i32 %"54", ptr %"62", align 4
  %"55" = load i64, ptr addrspace(5) %"41", align 4
  %"63" = inttoptr i64 %"55" to ptr
  %"36" = getelementptr inbounds i8, ptr %"63", i64 4
  %"56" = load i32, ptr addrspace(5) %"43", align 4
  store i32 %"56", ptr %"36", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }