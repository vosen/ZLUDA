define amdgpu_kernel void @atom_inc(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #0 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"43"

"43":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"44", align 8
  store i64 %2, ptr addrspace(5) %"46", align 8
  %3 = load i64, ptr addrspace(4) %"45", align 8
  store i64 %3, ptr addrspace(5) %"47", align 8
  %4 = load i64, ptr addrspace(5) %"46", align 8
  %"65" = inttoptr i64 %4 to ptr
  %5 = atomicrmw uinc_wrap ptr %"65", i32 101 syncscope("agent-one-as") monotonic, align 4
  store i32 %5, ptr addrspace(5) %"48", align 4
  %6 = load i64, ptr addrspace(5) %"46", align 8
  %"66" = inttoptr i64 %6 to ptr addrspace(1)
  %7 = atomicrmw uinc_wrap ptr addrspace(1) %"66", i32 101 syncscope("agent-one-as") monotonic, align 4
  store i32 %7, ptr addrspace(5) %"49", align 4
  %8 = load i64, ptr addrspace(5) %"46", align 8
  %"67" = inttoptr i64 %8 to ptr
  %9 = load i32, ptr %"67", align 4
  store i32 %9, ptr addrspace(5) %"50", align 4
  %10 = load i64, ptr addrspace(5) %"47", align 8
  %11 = load i32, ptr addrspace(5) %"48", align 4
  %"68" = inttoptr i64 %10 to ptr
  store i32 %11, ptr %"68", align 4
  %12 = load i64, ptr addrspace(5) %"47", align 8
  %"69" = inttoptr i64 %12 to ptr
  %"40" = getelementptr inbounds i8, ptr %"69", i64 4
  %13 = load i32, ptr addrspace(5) %"49", align 4
  store i32 %13, ptr %"40", align 4
  %14 = load i64, ptr addrspace(5) %"47", align 8
  %"70" = inttoptr i64 %14 to ptr
  %"42" = getelementptr inbounds i8, ptr %"70", i64 8
  %15 = load i32, ptr addrspace(5) %"50", align 4
  store i32 %15, ptr %"42", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }