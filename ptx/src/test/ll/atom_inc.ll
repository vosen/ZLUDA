define amdgpu_kernel void @atom_inc(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  store i32 101, ptr addrspace(5) %"50", align 4
  %"54" = alloca i32, align 4, addrspace(5)
  store i32 101, ptr addrspace(5) %"54", align 4
  %"62" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"62", align 4
  %"66" = alloca i64, align 8, addrspace(5)
  store i64 8, ptr addrspace(5) %"66", align 4
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %"48" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"48", ptr addrspace(5) %"43", align 8
  %"49" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"49", ptr addrspace(5) %"44", align 8
  %"52" = load i64, ptr addrspace(5) %"43", align 8
  %"53" = load i32, ptr addrspace(5) %"50", align 4
  %"70" = inttoptr i64 %"52" to ptr
  %2 = atomicrmw uinc_wrap ptr %"70", i32 %"53" syncscope("agent-one-as") monotonic, align 4
  store i32 %2, ptr addrspace(5) %"45", align 4
  %"56" = load i64, ptr addrspace(5) %"43", align 8
  %"57" = load i32, ptr addrspace(5) %"54", align 4
  %"71" = inttoptr i64 %"56" to ptr addrspace(1)
  %3 = atomicrmw uinc_wrap ptr addrspace(1) %"71", i32 %"57" syncscope("agent-one-as") monotonic, align 4
  store i32 %3, ptr addrspace(5) %"46", align 4
  %"59" = load i64, ptr addrspace(5) %"43", align 8
  %"72" = inttoptr i64 %"59" to ptr
  %"58" = load i32, ptr %"72", align 4
  store i32 %"58", ptr addrspace(5) %"47", align 4
  %"60" = load i64, ptr addrspace(5) %"44", align 8
  %"61" = load i32, ptr addrspace(5) %"45", align 4
  %"73" = inttoptr i64 %"60" to ptr
  store i32 %"61", ptr %"73", align 4
  %"63" = load i64, ptr addrspace(5) %"44", align 8
  %"64" = load i64, ptr addrspace(5) %"62", align 8
  %"74" = inttoptr i64 %"63" to ptr
  %"37" = getelementptr inbounds i8, ptr %"74", i64 %"64"
  %"65" = load i32, ptr addrspace(5) %"46", align 4
  store i32 %"65", ptr %"37", align 4
  %"67" = load i64, ptr addrspace(5) %"44", align 8
  %"68" = load i64, ptr addrspace(5) %"66", align 8
  %"75" = inttoptr i64 %"67" to ptr
  %"39" = getelementptr inbounds i8, ptr %"75", i64 %"68"
  %"69" = load i32, ptr addrspace(5) %"47", align 4
  store i32 %"69", ptr %"39", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }