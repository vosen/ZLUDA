define amdgpu_kernel void @atom_inc(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"45", ptr addrspace(5) %"40", align 8
  %"46" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"46", ptr addrspace(5) %"41", align 8
  %"48" = load i64, ptr addrspace(5) %"40", align 8
  %"59" = inttoptr i64 %"48" to ptr
  %2 = atomicrmw uinc_wrap ptr %"59", i32 101 syncscope("agent-one-as") monotonic, align 4
  store i32 %2, ptr addrspace(5) %"42", align 4
  %"50" = load i64, ptr addrspace(5) %"40", align 8
  %"60" = inttoptr i64 %"50" to ptr addrspace(1)
  %3 = atomicrmw uinc_wrap ptr addrspace(1) %"60", i32 101 syncscope("agent-one-as") monotonic, align 4
  store i32 %3, ptr addrspace(5) %"43", align 4
  %"52" = load i64, ptr addrspace(5) %"40", align 8
  %"61" = inttoptr i64 %"52" to ptr
  %"51" = load i32, ptr %"61", align 4
  store i32 %"51", ptr addrspace(5) %"44", align 4
  %"53" = load i64, ptr addrspace(5) %"41", align 8
  %"54" = load i32, ptr addrspace(5) %"42", align 4
  %"62" = inttoptr i64 %"53" to ptr
  store i32 %"54", ptr %"62", align 4
  %"55" = load i64, ptr addrspace(5) %"41", align 8
  %"63" = inttoptr i64 %"55" to ptr
  %"34" = getelementptr inbounds i8, ptr %"63", i64 4
  %"56" = load i32, ptr addrspace(5) %"43", align 4
  store i32 %"56", ptr %"34", align 4
  %"57" = load i64, ptr addrspace(5) %"41", align 8
  %"64" = inttoptr i64 %"57" to ptr
  %"36" = getelementptr inbounds i8, ptr %"64", i64 8
  %"58" = load i32, ptr addrspace(5) %"44", align 4
  store i32 %"58", ptr %"36", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }