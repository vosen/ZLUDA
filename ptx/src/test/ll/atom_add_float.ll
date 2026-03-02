@shared_mem = external addrspace(3) global [1024 x i8], align 4

define amdgpu_kernel void @atom_add_float(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #0 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca float, align 4, addrspace(5)
  %"50" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"45", align 8
  store i64 %2, ptr addrspace(5) %"47", align 8
  %3 = load i64, ptr addrspace(4) %"46", align 8
  store i64 %3, ptr addrspace(5) %"48", align 8
  %4 = load i64, ptr addrspace(5) %"47", align 8
  %"65" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"65", align 4
  store float %5, ptr addrspace(5) %"49", align 4
  %6 = load i64, ptr addrspace(5) %"47", align 8
  %"66" = inttoptr i64 %6 to ptr
  %"41" = getelementptr inbounds i8, ptr %"66", i64 4
  %7 = load float, ptr %"41", align 4
  store float %7, ptr addrspace(5) %"50", align 4
  %8 = load float, ptr addrspace(5) %"49", align 4
  store float %8, ptr addrspace(3) @shared_mem, align 4
  %9 = load float, ptr addrspace(5) %"50", align 4
  %10 = atomicrmw fadd ptr addrspace(3) @shared_mem, float %9 syncscope("agent-one-as") monotonic, align 4
  store float %10, ptr addrspace(5) %"49", align 4
  %11 = load float, ptr addrspace(3) @shared_mem, align 4
  store float %11, ptr addrspace(5) %"50", align 4
  %12 = load i64, ptr addrspace(5) %"48", align 8
  %13 = load float, ptr addrspace(5) %"49", align 4
  %"70" = inttoptr i64 %12 to ptr
  store float %13, ptr %"70", align 4
  %14 = load i64, ptr addrspace(5) %"48", align 8
  %"71" = inttoptr i64 %14 to ptr
  %"43" = getelementptr inbounds i8, ptr %"71", i64 4
  %15 = load float, ptr addrspace(5) %"50", align 4
  store float %15, ptr %"43", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
