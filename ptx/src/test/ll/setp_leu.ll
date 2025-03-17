define amdgpu_kernel void @setp_leu(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca float, align 4, addrspace(5)
  %"44" = alloca float, align 4, addrspace(5)
  %"45" = alloca float, align 4, addrspace(5)
  %"46" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %"47" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"47", ptr addrspace(5) %"41", align 4
  %"48" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"48", ptr addrspace(5) %"42", align 4
  %"50" = load i64, ptr addrspace(5) %"41", align 4
  %"64" = inttoptr i64 %"50" to ptr
  %"49" = load float, ptr %"64", align 4
  store float %"49", ptr addrspace(5) %"43", align 4
  %"51" = load i64, ptr addrspace(5) %"41", align 4
  %"65" = inttoptr i64 %"51" to ptr
  %"37" = getelementptr inbounds i8, ptr %"65", i64 4
  %"52" = load float, ptr %"37", align 4
  store float %"52", ptr addrspace(5) %"44", align 4
  %"54" = load float, ptr addrspace(5) %"43", align 4
  %"55" = load float, ptr addrspace(5) %"44", align 4
  %"53" = fcmp ule float %"54", %"55"
  store i1 %"53", ptr addrspace(5) %"46", align 1
  %"56" = load i1, ptr addrspace(5) %"46", align 1
  br i1 %"56", label %"16", label %"17"

"16":                                             ; preds = %"38"
  %"58" = load float, ptr addrspace(5) %"43", align 4
  store float %"58", ptr addrspace(5) %"45", align 4
  br label %"17"

"17":                                             ; preds = %"16", %"38"
  %"59" = load i1, ptr addrspace(5) %"46", align 1
  br i1 %"59", label %"19", label %"18"

"18":                                             ; preds = %"17"
  %"61" = load float, ptr addrspace(5) %"44", align 4
  store float %"61", ptr addrspace(5) %"45", align 4
  br label %"19"

"19":                                             ; preds = %"18", %"17"
  %"62" = load i64, ptr addrspace(5) %"42", align 4
  %"63" = load float, ptr addrspace(5) %"45", align 4
  %"66" = inttoptr i64 %"62" to ptr
  store float %"63", ptr %"66", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }