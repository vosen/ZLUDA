define amdgpu_kernel void @mul_wide(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"44", ptr addrspace(5) %"39", align 8
  %"45" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"45", ptr addrspace(5) %"40", align 8
  %"47" = load i64, ptr addrspace(5) %"39", align 8
  %"55" = inttoptr i64 %"47" to ptr addrspace(1)
  %"46" = load i32, ptr addrspace(1) %"55", align 4
  store i32 %"46", ptr addrspace(5) %"41", align 4
  %"48" = load i64, ptr addrspace(5) %"39", align 8
  %"56" = inttoptr i64 %"48" to ptr addrspace(1)
  %"35" = getelementptr inbounds i8, ptr addrspace(1) %"56", i64 4
  %"49" = load i32, ptr addrspace(1) %"35", align 4
  store i32 %"49", ptr addrspace(5) %"42", align 4
  %"51" = load i32, ptr addrspace(5) %"41", align 4
  %"52" = load i32, ptr addrspace(5) %"42", align 4
  %2 = sext i32 %"51" to i64
  %3 = sext i32 %"52" to i64
  %"50" = mul i64 %2, %3
  store i64 %"50", ptr addrspace(5) %"43", align 8
  %"53" = load i64, ptr addrspace(5) %"40", align 8
  %"54" = load i64, ptr addrspace(5) %"43", align 8
  %"57" = inttoptr i64 %"53" to ptr
  store i64 %"54", ptr %"57", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }