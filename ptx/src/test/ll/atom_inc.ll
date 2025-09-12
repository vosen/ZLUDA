define amdgpu_kernel void @atom_inc(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %"48" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"48", ptr addrspace(5) %"43", align 8
  %"49" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"49", ptr addrspace(5) %"44", align 8
  %"51" = load i64, ptr addrspace(5) %"43", align 8
  %"62" = inttoptr i64 %"51" to ptr
  %2 = atomicrmw uinc_wrap ptr %"62", i32 101 syncscope("agent-one-as") monotonic, align 4
  store i32 %2, ptr addrspace(5) %"45", align 4
  %"53" = load i64, ptr addrspace(5) %"43", align 8
  %"63" = inttoptr i64 %"53" to ptr addrspace(1)
  %3 = atomicrmw uinc_wrap ptr addrspace(1) %"63", i32 101 syncscope("agent-one-as") monotonic, align 4
  store i32 %3, ptr addrspace(5) %"46", align 4
  %"55" = load i64, ptr addrspace(5) %"43", align 8
  %"64" = inttoptr i64 %"55" to ptr
  %"54" = load i32, ptr %"64", align 4
  store i32 %"54", ptr addrspace(5) %"47", align 4
  %"56" = load i64, ptr addrspace(5) %"44", align 8
  %"57" = load i32, ptr addrspace(5) %"45", align 4
  %"65" = inttoptr i64 %"56" to ptr
  store i32 %"57", ptr %"65", align 4
  %"58" = load i64, ptr addrspace(5) %"44", align 8
  %"66" = inttoptr i64 %"58" to ptr
  %"37" = getelementptr inbounds i8, ptr %"66", i64 4
  %"59" = load i32, ptr addrspace(5) %"46", align 4
  store i32 %"59", ptr %"37", align 4
  %"60" = load i64, ptr addrspace(5) %"44", align 8
  %"67" = inttoptr i64 %"60" to ptr
  %"39" = getelementptr inbounds i8, ptr %"67", i64 8
  %"61" = load i32, ptr addrspace(5) %"47", align 4
  store i32 %"61", ptr %"39", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }