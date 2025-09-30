@shared_mem = external addrspace(3) global [1024 x i8], align 4

define amdgpu_kernel void @atom_add_float(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca float, align 4, addrspace(5)
  %"44" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"45", ptr addrspace(5) %"41", align 8
  %"46" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"46", ptr addrspace(5) %"42", align 8
  %"48" = load i64, ptr addrspace(5) %"41", align 8
  %"59" = inttoptr i64 %"48" to ptr
  %"47" = load float, ptr %"59", align 4
  store float %"47", ptr addrspace(5) %"43", align 4
  %"49" = load i64, ptr addrspace(5) %"41", align 8
  %"60" = inttoptr i64 %"49" to ptr
  %"35" = getelementptr inbounds i8, ptr %"60", i64 4
  %"50" = load float, ptr %"35", align 4
  store float %"50", ptr addrspace(5) %"44", align 4
  %"51" = load float, ptr addrspace(5) %"43", align 4
  store float %"51", ptr addrspace(3) @shared_mem, align 4
  %"53" = load float, ptr addrspace(5) %"44", align 4
  %2 = atomicrmw fadd ptr addrspace(3) @shared_mem, float %"53" syncscope("agent-one-as") monotonic, align 4
  store float %2, ptr addrspace(5) %"43", align 4
  %"54" = load float, ptr addrspace(3) @shared_mem, align 4
  store float %"54", ptr addrspace(5) %"44", align 4
  %"55" = load i64, ptr addrspace(5) %"42", align 8
  %"56" = load float, ptr addrspace(5) %"43", align 4
  %"64" = inttoptr i64 %"55" to ptr
  store float %"56", ptr %"64", align 4
  %"57" = load i64, ptr addrspace(5) %"42", align 8
  %"65" = inttoptr i64 %"57" to ptr
  %"37" = getelementptr inbounds i8, ptr %"65", i64 4
  %"58" = load float, ptr addrspace(5) %"44", align 4
  store float %"58", ptr %"37", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
