define amdgpu_kernel void @setp_gt(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #0 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca float, align 4, addrspace(5)
  %"47" = alloca float, align 4, addrspace(5)
  %"48" = alloca float, align 4, addrspace(5)
  %"49" = alloca i1, align 1, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"54", align 4
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %"50" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"50", ptr addrspace(5) %"44", align 8
  %"51" = load i64, ptr addrspace(4) %"43", align 8
  store i64 %"51", ptr addrspace(5) %"45", align 8
  %"53" = load i64, ptr addrspace(5) %"44", align 8
  %"69" = inttoptr i64 %"53" to ptr
  %"52" = load float, ptr %"69", align 4
  store float %"52", ptr addrspace(5) %"46", align 4
  %"55" = load i64, ptr addrspace(5) %"44", align 8
  %"56" = load i64, ptr addrspace(5) %"54", align 8
  %"70" = inttoptr i64 %"55" to ptr
  %"40" = getelementptr inbounds i8, ptr %"70", i64 %"56"
  %"57" = load float, ptr %"40", align 4
  store float %"57", ptr addrspace(5) %"47", align 4
  %"59" = load float, ptr addrspace(5) %"46", align 4
  %"60" = load float, ptr addrspace(5) %"47", align 4
  %2 = fcmp ogt float %"59", %"60"
  store i1 %2, ptr addrspace(5) %"49", align 1
  %"61" = load i1, ptr addrspace(5) %"49", align 1
  br i1 %"61", label %"17", label %"18"

"17":                                             ; preds = %"41"
  %"63" = load float, ptr addrspace(5) %"46", align 4
  store float %"63", ptr addrspace(5) %"48", align 4
  br label %"18"

"18":                                             ; preds = %"17", %"41"
  %"64" = load i1, ptr addrspace(5) %"49", align 1
  br i1 %"64", label %"20", label %"19"

"19":                                             ; preds = %"18"
  %"66" = load float, ptr addrspace(5) %"47", align 4
  store float %"66", ptr addrspace(5) %"48", align 4
  br label %"20"

"20":                                             ; preds = %"19", %"18"
  %"67" = load i64, ptr addrspace(5) %"45", align 8
  %"68" = load float, ptr addrspace(5) %"48", align 4
  %"71" = inttoptr i64 %"67" to ptr
  store float %"68", ptr %"71", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }