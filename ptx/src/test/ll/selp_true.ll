define amdgpu_kernel void @selp_true(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i16, align 2, addrspace(5)
  %"42" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"43" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"43", ptr addrspace(5) %"39", align 8
  %"44" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"46" = load i64, ptr addrspace(5) %"39", align 8
  %"54" = inttoptr i64 %"46" to ptr
  %"45" = load i16, ptr %"54", align 2
  store i16 %"45", ptr addrspace(5) %"41", align 2
  %"47" = load i64, ptr addrspace(5) %"39", align 8
  %"55" = inttoptr i64 %"47" to ptr
  %"34" = getelementptr inbounds i8, ptr %"55", i64 2
  %"48" = load i16, ptr %"34", align 2
  store i16 %"48", ptr addrspace(5) %"42", align 2
  %"50" = load i16, ptr addrspace(5) %"41", align 2
  %"51" = load i16, ptr addrspace(5) %"42", align 2
  %"49" = select i1 true, i16 %"50", i16 %"51"
  store i16 %"49", ptr addrspace(5) %"41", align 2
  %"52" = load i64, ptr addrspace(5) %"40", align 8
  %"53" = load i16, ptr addrspace(5) %"41", align 2
  %"56" = inttoptr i64 %"52" to ptr
  store i16 %"53", ptr %"56", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }