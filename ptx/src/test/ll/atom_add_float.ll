@shared_mem = external addrspace(3) global [1024 x i8], align 4
@0 = addrspace(4) global i64 4
@1 = addrspace(4) global i64 4

define amdgpu_kernel void @atom_add_float(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca float, align 4, addrspace(5)
  %"46" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %"47" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"47", ptr addrspace(5) %"43", align 8
  %"48" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"48", ptr addrspace(5) %"44", align 8
  %"50" = load i64, ptr addrspace(5) %"43", align 8
  %"61" = inttoptr i64 %"50" to ptr
  %"49" = load float, ptr %"61", align 4
  store float %"49", ptr addrspace(5) %"45", align 4
  %"35" = load i64, ptr addrspace(4) @0, align 8
  %"51" = load i64, ptr addrspace(5) %"43", align 8
  %"62" = inttoptr i64 %"51" to ptr
  %"36" = getelementptr inbounds i8, ptr %"62", i64 %"35"
  %"52" = load float, ptr %"36", align 4
  store float %"52", ptr addrspace(5) %"46", align 4
  %"53" = load float, ptr addrspace(5) %"45", align 4
  store float %"53", ptr addrspace(3) @shared_mem, align 4
  %"55" = load float, ptr addrspace(5) %"46", align 4
  %2 = atomicrmw fadd ptr addrspace(3) @shared_mem, float %"55" syncscope("agent-one-as") monotonic, align 4
  store float %2, ptr addrspace(5) %"45", align 4
  %"56" = load float, ptr addrspace(3) @shared_mem, align 4
  store float %"56", ptr addrspace(5) %"46", align 4
  %"57" = load i64, ptr addrspace(5) %"44", align 8
  %"58" = load float, ptr addrspace(5) %"45", align 4
  %"66" = inttoptr i64 %"57" to ptr
  store float %"58", ptr %"66", align 4
  %"38" = load i64, ptr addrspace(4) @1, align 8
  %"59" = load i64, ptr addrspace(5) %"44", align 8
  %"67" = inttoptr i64 %"59" to ptr
  %"39" = getelementptr inbounds i8, ptr %"67", i64 %"38"
  %"60" = load float, ptr addrspace(5) %"46", align 4
  store float %"60", ptr %"39", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }