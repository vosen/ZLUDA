define amdgpu_kernel void @ld_st_offset(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"48", align 4
  %"54" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"54", align 4
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"45" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"45", ptr addrspace(5) %"41", align 8
  %"47" = load i64, ptr addrspace(5) %"40", align 8
  %"58" = inttoptr i64 %"47" to ptr
  %"46" = load i32, ptr %"58", align 4
  store i32 %"46", ptr addrspace(5) %"42", align 4
  %"49" = load i64, ptr addrspace(5) %"40", align 8
  %"50" = load i64, ptr addrspace(5) %"48", align 8
  %"59" = inttoptr i64 %"49" to ptr
  %"34" = getelementptr inbounds i8, ptr %"59", i64 %"50"
  %"51" = load i32, ptr %"34", align 4
  store i32 %"51", ptr addrspace(5) %"43", align 4
  %"52" = load i64, ptr addrspace(5) %"41", align 8
  %"53" = load i32, ptr addrspace(5) %"43", align 4
  %"60" = inttoptr i64 %"52" to ptr
  store i32 %"53", ptr %"60", align 4
  %"55" = load i64, ptr addrspace(5) %"41", align 8
  %"56" = load i64, ptr addrspace(5) %"54", align 8
  %"61" = inttoptr i64 %"55" to ptr
  %"36" = getelementptr inbounds i8, ptr %"61", i64 %"56"
  %"57" = load i32, ptr addrspace(5) %"42", align 4
  store i32 %"57", ptr %"36", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }