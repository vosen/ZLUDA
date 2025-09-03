define amdgpu_kernel void @or(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  store i64 8, ptr addrspace(5) %"46", align 4
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"42" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"42", ptr addrspace(5) %"38", align 8
  %"43" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"43", ptr addrspace(5) %"39", align 8
  %"45" = load i64, ptr addrspace(5) %"38", align 8
  %"55" = inttoptr i64 %"45" to ptr
  %"44" = load i64, ptr %"55", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"47" = load i64, ptr addrspace(5) %"38", align 8
  %"48" = load i64, ptr addrspace(5) %"46", align 8
  %"56" = inttoptr i64 %"47" to ptr
  %"34" = getelementptr inbounds i8, ptr %"56", i64 %"48"
  %"49" = load i64, ptr %"34", align 8
  store i64 %"49", ptr addrspace(5) %"41", align 8
  %"51" = load i64, ptr addrspace(5) %"40", align 8
  %"52" = load i64, ptr addrspace(5) %"41", align 8
  %"57" = or i64 %"51", %"52"
  store i64 %"57", ptr addrspace(5) %"40", align 8
  %"53" = load i64, ptr addrspace(5) %"39", align 8
  %"54" = load i64, ptr addrspace(5) %"40", align 8
  %"60" = inttoptr i64 %"53" to ptr
  store i64 %"54", ptr %"60", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }