define amdgpu_kernel void @or(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"42" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"42", ptr addrspace(5) %"38", align 8
  %"43" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"43", ptr addrspace(5) %"39", align 8
  %"45" = load i64, ptr addrspace(5) %"38", align 8
  %"53" = inttoptr i64 %"45" to ptr
  %"44" = load i64, ptr %"53", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"46" = load i64, ptr addrspace(5) %"38", align 8
  %"54" = inttoptr i64 %"46" to ptr
  %"34" = getelementptr inbounds i8, ptr %"54", i64 8
  %"47" = load i64, ptr %"34", align 8
  store i64 %"47", ptr addrspace(5) %"41", align 8
  %"49" = load i64, ptr addrspace(5) %"40", align 8
  %"50" = load i64, ptr addrspace(5) %"41", align 8
  %"55" = or i64 %"49", %"50"
  store i64 %"55", ptr addrspace(5) %"40", align 8
  %"51" = load i64, ptr addrspace(5) %"39", align 8
  %"52" = load i64, ptr addrspace(5) %"40", align 8
  %"58" = inttoptr i64 %"51" to ptr
  store i64 %"52", ptr %"58", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
