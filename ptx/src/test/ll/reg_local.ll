define amdgpu_kernel void @reg_local(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"11" = alloca [8 x i8], align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"45", ptr addrspace(5) %"42", align 8
  %"46" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"46", ptr addrspace(5) %"43", align 8
  %"48" = load i64, ptr addrspace(5) %"42", align 8
  %"54" = inttoptr i64 %"48" to ptr addrspace(1)
  %"53" = load i64, ptr addrspace(1) %"54", align 8
  store i64 %"53", ptr addrspace(5) %"44", align 8
  %"49" = load i64, ptr addrspace(5) %"44", align 8
  %"34" = add i64 %"49", 1
  %"55" = addrspacecast ptr addrspace(5) %"11" to ptr
  store i64 %"34", ptr %"55", align 8
  %"57" = addrspacecast ptr addrspace(5) %"11" to ptr
  %"36" = getelementptr inbounds i8, ptr %"57", i64 0
  %"58" = load i64, ptr %"36", align 8
  store i64 %"58", ptr addrspace(5) %"44", align 8
  %"51" = load i64, ptr addrspace(5) %"43", align 8
  %"59" = inttoptr i64 %"51" to ptr addrspace(1)
  %"38" = getelementptr inbounds i8, ptr addrspace(1) %"59", i64 0
  %"52" = load i64, ptr addrspace(5) %"44", align 8
  store i64 %"52", ptr addrspace(1) %"38", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
