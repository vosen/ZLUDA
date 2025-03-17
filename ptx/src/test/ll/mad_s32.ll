define amdgpu_kernel void @mad_s32(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %"49" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"49", ptr addrspace(5) %"43", align 4
  %"50" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"50", ptr addrspace(5) %"44", align 4
  %"52" = load i64, ptr addrspace(5) %"43", align 4
  %"67" = inttoptr i64 %"52" to ptr
  %"51" = load i32, ptr %"67", align 4
  store i32 %"51", ptr addrspace(5) %"46", align 4
  %"53" = load i64, ptr addrspace(5) %"43", align 4
  %"68" = inttoptr i64 %"53" to ptr
  %"33" = getelementptr inbounds i8, ptr %"68", i64 4
  %"54" = load i32, ptr %"33", align 4
  store i32 %"54", ptr addrspace(5) %"47", align 4
  %"55" = load i64, ptr addrspace(5) %"43", align 4
  %"69" = inttoptr i64 %"55" to ptr
  %"35" = getelementptr inbounds i8, ptr %"69", i64 8
  %"56" = load i32, ptr %"35", align 4
  store i32 %"56", ptr addrspace(5) %"48", align 4
  %"58" = load i32, ptr addrspace(5) %"46", align 4
  %"59" = load i32, ptr addrspace(5) %"47", align 4
  %"60" = load i32, ptr addrspace(5) %"48", align 4
  %2 = mul i32 %"58", %"59"
  %"57" = add i32 %2, %"60"
  store i32 %"57", ptr addrspace(5) %"45", align 4
  %"61" = load i64, ptr addrspace(5) %"44", align 4
  %"62" = load i32, ptr addrspace(5) %"45", align 4
  %"70" = inttoptr i64 %"61" to ptr
  store i32 %"62", ptr %"70", align 4
  %"63" = load i64, ptr addrspace(5) %"44", align 4
  %"71" = inttoptr i64 %"63" to ptr
  %"37" = getelementptr inbounds i8, ptr %"71", i64 4
  %"64" = load i32, ptr addrspace(5) %"45", align 4
  store i32 %"64", ptr %"37", align 4
  %"65" = load i64, ptr addrspace(5) %"44", align 4
  %"72" = inttoptr i64 %"65" to ptr
  %"39" = getelementptr inbounds i8, ptr %"72", i64 8
  %"66" = load i32, ptr addrspace(5) %"45", align 4
  store i32 %"66", ptr %"39", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }