define amdgpu_kernel void @mad_wide(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"37", align 4
  store i64 %"45", ptr addrspace(5) %"39", align 4
  %"46" = load i64, ptr addrspace(4) %"38", align 4
  store i64 %"46", ptr addrspace(5) %"40", align 4
  %"48" = load i64, ptr addrspace(5) %"39", align 4
  %"59" = inttoptr i64 %"48" to ptr
  %"47" = load i32, ptr %"59", align 4
  store i32 %"47", ptr addrspace(5) %"42", align 4
  %"49" = load i64, ptr addrspace(5) %"39", align 4
  %"60" = inttoptr i64 %"49" to ptr
  %"33" = getelementptr inbounds i8, ptr %"60", i64 4
  %"50" = load i32, ptr %"33", align 4
  store i32 %"50", ptr addrspace(5) %"43", align 4
  %"51" = load i64, ptr addrspace(5) %"39", align 4
  %"61" = inttoptr i64 %"51" to ptr
  %"35" = getelementptr inbounds i8, ptr %"61", i64 8
  %"52" = load i64, ptr %"35", align 4
  store i64 %"52", ptr addrspace(5) %"44", align 4
  %"54" = load i32, ptr addrspace(5) %"42", align 4
  %"55" = load i32, ptr addrspace(5) %"43", align 4
  %"56" = load i64, ptr addrspace(5) %"44", align 4
  %2 = sext i32 %"54" to i64
  %3 = sext i32 %"55" to i64
  %4 = mul i64 %2, %3
  %"53" = add i64 %4, %"56"
  store i64 %"53", ptr addrspace(5) %"41", align 4
  %"57" = load i64, ptr addrspace(5) %"40", align 4
  %"58" = load i64, ptr addrspace(5) %"41", align 4
  %"62" = inttoptr i64 %"57" to ptr
  store i64 %"58", ptr %"62", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }