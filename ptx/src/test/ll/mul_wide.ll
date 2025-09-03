define amdgpu_kernel void @mul_wide(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"48", align 4
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"44", ptr addrspace(5) %"39", align 8
  %"45" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"45", ptr addrspace(5) %"40", align 8
  %"47" = load i64, ptr addrspace(5) %"39", align 8
  %"57" = inttoptr i64 %"47" to ptr addrspace(1)
  %"46" = load i32, ptr addrspace(1) %"57", align 4
  store i32 %"46", ptr addrspace(5) %"41", align 4
  %"49" = load i64, ptr addrspace(5) %"39", align 8
  %"50" = load i64, ptr addrspace(5) %"48", align 8
  %"58" = inttoptr i64 %"49" to ptr addrspace(1)
  %"35" = getelementptr inbounds i8, ptr addrspace(1) %"58", i64 %"50"
  %"51" = load i32, ptr addrspace(1) %"35", align 4
  store i32 %"51", ptr addrspace(5) %"42", align 4
  %"53" = load i32, ptr addrspace(5) %"41", align 4
  %"54" = load i32, ptr addrspace(5) %"42", align 4
  %2 = sext i32 %"53" to i64
  %3 = sext i32 %"54" to i64
  %"52" = mul i64 %2, %3
  store i64 %"52", ptr addrspace(5) %"43", align 8
  %"55" = load i64, ptr addrspace(5) %"40", align 8
  %"56" = load i64, ptr addrspace(5) %"43", align 8
  %"59" = inttoptr i64 %"55" to ptr
  store i64 %"56", ptr %"59", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }