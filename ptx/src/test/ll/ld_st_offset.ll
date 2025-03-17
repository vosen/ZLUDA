define amdgpu_kernel void @ld_st_offset(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i32, align 4, addrspace(5)
  %"40" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"41" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"41", ptr addrspace(5) %"37", align 4
  %"42" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"42", ptr addrspace(5) %"38", align 4
  %"44" = load i64, ptr addrspace(5) %"37", align 4
  %"51" = inttoptr i64 %"44" to ptr
  %"43" = load i32, ptr %"51", align 4
  store i32 %"43", ptr addrspace(5) %"39", align 4
  %"45" = load i64, ptr addrspace(5) %"37", align 4
  %"52" = inttoptr i64 %"45" to ptr
  %"31" = getelementptr inbounds i8, ptr %"52", i64 4
  %"46" = load i32, ptr %"31", align 4
  store i32 %"46", ptr addrspace(5) %"40", align 4
  %"47" = load i64, ptr addrspace(5) %"38", align 4
  %"48" = load i32, ptr addrspace(5) %"40", align 4
  %"53" = inttoptr i64 %"47" to ptr
  store i32 %"48", ptr %"53", align 4
  %"49" = load i64, ptr addrspace(5) %"38", align 4
  %"54" = inttoptr i64 %"49" to ptr
  %"33" = getelementptr inbounds i8, ptr %"54", i64 4
  %"50" = load i32, ptr addrspace(5) %"39", align 4
  store i32 %"50", ptr %"33", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }