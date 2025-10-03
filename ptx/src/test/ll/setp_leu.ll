define amdgpu_kernel void @setp_leu(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #0 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca float, align 4, addrspace(5)
  %"47" = alloca float, align 4, addrspace(5)
  %"48" = alloca float, align 4, addrspace(5)
  %"49" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %"50" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"50", ptr addrspace(5) %"44", align 8
  %"51" = load i64, ptr addrspace(4) %"43", align 8
  store i64 %"51", ptr addrspace(5) %"45", align 8
  %"53" = load i64, ptr addrspace(5) %"44", align 8
  %"67" = inttoptr i64 %"53" to ptr
  %"52" = load float, ptr %"67", align 4
  store float %"52", ptr addrspace(5) %"46", align 4
  %"54" = load i64, ptr addrspace(5) %"44", align 8
  %"68" = inttoptr i64 %"54" to ptr
  %"40" = getelementptr inbounds i8, ptr %"68", i64 4
  %"55" = load float, ptr %"40", align 4
  store float %"55", ptr addrspace(5) %"47", align 4
  %"57" = load float, ptr addrspace(5) %"46", align 4
  %"58" = load float, ptr addrspace(5) %"47", align 4
  %2 = fcmp ule float %"57", %"58"
  store i1 %2, ptr addrspace(5) %"49", align 1
  %"59" = load i1, ptr addrspace(5) %"49", align 1
  br i1 %"59", label %"17", label %"18"

"17":                                             ; preds = %"41"
  %"61" = load float, ptr addrspace(5) %"46", align 4
  store float %"61", ptr addrspace(5) %"48", align 4
  br label %"18"

"18":                                             ; preds = %"17", %"41"
  %"62" = load i1, ptr addrspace(5) %"49", align 1
  br i1 %"62", label %"20", label %"19"

"19":                                             ; preds = %"18"
  %"64" = load float, ptr addrspace(5) %"47", align 4
  store float %"64", ptr addrspace(5) %"48", align 4
  br label %"20"

"20":                                             ; preds = %"19", %"18"
  %"65" = load i64, ptr addrspace(5) %"45", align 8
  %"66" = load float, ptr addrspace(5) %"48", align 4
  %"69" = inttoptr i64 %"65" to ptr
  store float %"66", ptr %"69", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
