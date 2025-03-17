define amdgpu_kernel void @mul_wide(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i32, align 4, addrspace(5)
  %"39" = alloca i32, align 4, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"41" = load i64, ptr addrspace(4) %"34", align 4
  store i64 %"41", ptr addrspace(5) %"36", align 4
  %"42" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"42", ptr addrspace(5) %"37", align 4
  %"44" = load i64, ptr addrspace(5) %"36", align 4
  %"52" = inttoptr i64 %"44" to ptr addrspace(1)
  %"43" = load i32, ptr addrspace(1) %"52", align 4
  store i32 %"43", ptr addrspace(5) %"38", align 4
  %"45" = load i64, ptr addrspace(5) %"36", align 4
  %"53" = inttoptr i64 %"45" to ptr addrspace(1)
  %"32" = getelementptr inbounds i8, ptr addrspace(1) %"53", i64 4
  %"46" = load i32, ptr addrspace(1) %"32", align 4
  store i32 %"46", ptr addrspace(5) %"39", align 4
  %"48" = load i32, ptr addrspace(5) %"38", align 4
  %"49" = load i32, ptr addrspace(5) %"39", align 4
  %2 = sext i32 %"48" to i64
  %3 = sext i32 %"49" to i64
  %"47" = mul i64 %2, %3
  store i64 %"47", ptr addrspace(5) %"40", align 4
  %"50" = load i64, ptr addrspace(5) %"37", align 4
  %"51" = load i64, ptr addrspace(5) %"40", align 4
  %"54" = inttoptr i64 %"50" to ptr
  store i64 %"51", ptr %"54", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }