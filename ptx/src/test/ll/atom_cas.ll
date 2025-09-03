define amdgpu_kernel void @atom_cas(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"51", align 4
  %"54" = alloca i32, align 4, addrspace(5)
  store i32 100, ptr addrspace(5) %"54", align 4
  %"58" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"58", align 4
  %"64" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"64", align 4
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %"47" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"47", ptr addrspace(5) %"43", align 8
  %"48" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"48", ptr addrspace(5) %"44", align 8
  %"50" = load i64, ptr addrspace(5) %"43", align 8
  %"68" = inttoptr i64 %"50" to ptr
  %"49" = load i32, ptr %"68", align 4
  store i32 %"49", ptr addrspace(5) %"45", align 4
  %"52" = load i64, ptr addrspace(5) %"43", align 8
  %"53" = load i64, ptr addrspace(5) %"51", align 8
  %"69" = inttoptr i64 %"52" to ptr
  %"34" = getelementptr inbounds i8, ptr %"69", i64 %"53"
  %"56" = load i32, ptr addrspace(5) %"45", align 4
  %"57" = load i32, ptr addrspace(5) %"54", align 4
  %2 = cmpxchg ptr %"34", i32 %"56", i32 %"57" syncscope("agent-one-as") monotonic monotonic, align 4
  %"70" = extractvalue { i32, i1 } %2, 0
  store i32 %"70", ptr addrspace(5) %"45", align 4
  %"59" = load i64, ptr addrspace(5) %"43", align 8
  %"60" = load i64, ptr addrspace(5) %"58", align 8
  %"72" = inttoptr i64 %"59" to ptr
  %"37" = getelementptr inbounds i8, ptr %"72", i64 %"60"
  %"61" = load i32, ptr %"37", align 4
  store i32 %"61", ptr addrspace(5) %"46", align 4
  %"62" = load i64, ptr addrspace(5) %"44", align 8
  %"63" = load i32, ptr addrspace(5) %"45", align 4
  %"73" = inttoptr i64 %"62" to ptr
  store i32 %"63", ptr %"73", align 4
  %"65" = load i64, ptr addrspace(5) %"44", align 8
  %"66" = load i64, ptr addrspace(5) %"64", align 8
  %"74" = inttoptr i64 %"65" to ptr
  %"39" = getelementptr inbounds i8, ptr %"74", i64 %"66"
  %"67" = load i32, ptr addrspace(5) %"46", align 4
  store i32 %"67", ptr %"39", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }