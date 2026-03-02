define amdgpu_kernel void @atom_cas(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #0 {
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"47", align 8
  store i64 %2, ptr addrspace(5) %"49", align 8
  %3 = load i64, ptr addrspace(4) %"48", align 8
  store i64 %3, ptr addrspace(5) %"50", align 8
  %4 = load i64, ptr addrspace(5) %"49", align 8
  %"66" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"66", align 4
  store i32 %5, ptr addrspace(5) %"51", align 4
  %6 = load i64, ptr addrspace(5) %"49", align 8
  %"67" = inttoptr i64 %6 to ptr
  %"40" = getelementptr inbounds i8, ptr %"67", i64 4
  %7 = load i32, ptr addrspace(5) %"51", align 4
  %8 = cmpxchg ptr %"40", i32 %7, i32 100 syncscope("agent-one-as") monotonic monotonic, align 4
  %"68" = extractvalue { i32, i1 } %8, 0
  store i32 %"68", ptr addrspace(5) %"51", align 4
  %9 = load i64, ptr addrspace(5) %"49", align 8
  %"70" = inttoptr i64 %9 to ptr
  %"43" = getelementptr inbounds i8, ptr %"70", i64 4
  %10 = load i32, ptr %"43", align 4
  store i32 %10, ptr addrspace(5) %"52", align 4
  %11 = load i64, ptr addrspace(5) %"50", align 8
  %12 = load i32, ptr addrspace(5) %"51", align 4
  %"71" = inttoptr i64 %11 to ptr
  store i32 %12, ptr %"71", align 4
  %13 = load i64, ptr addrspace(5) %"50", align 8
  %"72" = inttoptr i64 %13 to ptr
  %"45" = getelementptr inbounds i8, ptr %"72", i64 4
  %14 = load i32, ptr addrspace(5) %"52", align 4
  store i32 %14, ptr %"45", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
