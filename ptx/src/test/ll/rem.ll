define amdgpu_kernel void @rem(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i32, align 4, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
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
  %"44" = load i32, ptr %"53", align 4
  store i32 %"44", ptr addrspace(5) %"40", align 4
  %"46" = load i64, ptr addrspace(5) %"38", align 8
  %"54" = inttoptr i64 %"46" to ptr
  %"34" = getelementptr inbounds i8, ptr %"54", i64 4
  %"47" = load i32, ptr %"34", align 4
  store i32 %"47", ptr addrspace(5) %"41", align 4
  %"49" = load i32, ptr addrspace(5) %"40", align 4
  %"50" = load i32, ptr addrspace(5) %"41", align 4
  %"48" = srem i32 %"49", %"50"
  store i32 %"48", ptr addrspace(5) %"40", align 4
  %"51" = load i64, ptr addrspace(5) %"39", align 8
  %"52" = load i32, ptr addrspace(5) %"40", align 4
  %"55" = inttoptr i64 %"51" to ptr
  store i32 %"52", ptr %"55", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
