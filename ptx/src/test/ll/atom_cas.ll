define amdgpu_kernel void @atom_cas(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %"47" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"47", ptr addrspace(5) %"43", align 8
  %"48" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"48", ptr addrspace(5) %"44", align 8
  %"50" = load i64, ptr addrspace(5) %"43", align 8
  %"60" = inttoptr i64 %"50" to ptr
  %"49" = load i32, ptr %"60", align 4
  store i32 %"49", ptr addrspace(5) %"45", align 4
  %"51" = load i64, ptr addrspace(5) %"43", align 8
  %"61" = inttoptr i64 %"51" to ptr
  %"34" = getelementptr inbounds i8, ptr %"61", i64 4
  %"53" = load i32, ptr addrspace(5) %"45", align 4
  %2 = cmpxchg ptr %"34", i32 %"53", i32 100 syncscope("agent-one-as") monotonic monotonic, align 4
  %"62" = extractvalue { i32, i1 } %2, 0
  store i32 %"62", ptr addrspace(5) %"45", align 4
  %"54" = load i64, ptr addrspace(5) %"43", align 8
  %"64" = inttoptr i64 %"54" to ptr
  %"37" = getelementptr inbounds i8, ptr %"64", i64 4
  %"55" = load i32, ptr %"37", align 4
  store i32 %"55", ptr addrspace(5) %"46", align 4
  %"56" = load i64, ptr addrspace(5) %"44", align 8
  %"57" = load i32, ptr addrspace(5) %"45", align 4
  %"65" = inttoptr i64 %"56" to ptr
  store i32 %"57", ptr %"65", align 4
  %"58" = load i64, ptr addrspace(5) %"44", align 8
  %"66" = inttoptr i64 %"58" to ptr
  %"39" = getelementptr inbounds i8, ptr %"66", i64 4
  %"59" = load i32, ptr addrspace(5) %"46", align 4
  store i32 %"59", ptr %"39", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }