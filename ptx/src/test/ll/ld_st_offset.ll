define amdgpu_kernel void @ld_st_offset(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"45" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"45", ptr addrspace(5) %"41", align 8
  %"47" = load i64, ptr addrspace(5) %"40", align 8
  %"54" = inttoptr i64 %"47" to ptr
  %"46" = load i32, ptr %"54", align 4
  store i32 %"46", ptr addrspace(5) %"42", align 4
  %"48" = load i64, ptr addrspace(5) %"40", align 8
  %"55" = inttoptr i64 %"48" to ptr
  %"34" = getelementptr inbounds i8, ptr %"55", i64 4
  %"49" = load i32, ptr %"34", align 4
  store i32 %"49", ptr addrspace(5) %"43", align 4
  %"50" = load i64, ptr addrspace(5) %"41", align 8
  %"51" = load i32, ptr addrspace(5) %"43", align 4
  %"56" = inttoptr i64 %"50" to ptr
  store i32 %"51", ptr %"56", align 4
  %"52" = load i64, ptr addrspace(5) %"41", align 8
  %"57" = inttoptr i64 %"52" to ptr
  %"36" = getelementptr inbounds i8, ptr %"57", i64 4
  %"53" = load i32, ptr addrspace(5) %"42", align 4
  store i32 %"53", ptr %"36", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }