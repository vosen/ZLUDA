define amdgpu_kernel void @setp_gt(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #0 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca float, align 4, addrspace(5)
  %"50" = alloca float, align 4, addrspace(5)
  %"51" = alloca float, align 4, addrspace(5)
  %"52" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"45", align 8
  store i64 %2, ptr addrspace(5) %"47", align 8
  %3 = load i64, ptr addrspace(4) %"46", align 8
  store i64 %3, ptr addrspace(5) %"48", align 8
  %4 = load i64, ptr addrspace(5) %"47", align 8
  %"70" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"70", align 4
  store float %5, ptr addrspace(5) %"49", align 4
  %6 = load i64, ptr addrspace(5) %"47", align 8
  %"71" = inttoptr i64 %6 to ptr
  %"43" = getelementptr inbounds i8, ptr %"71", i64 4
  %7 = load float, ptr %"43", align 4
  store float %7, ptr addrspace(5) %"50", align 4
  %8 = load float, ptr addrspace(5) %"49", align 4
  %9 = load float, ptr addrspace(5) %"50", align 4
  %10 = fcmp ogt float %8, %9
  store i1 %10, ptr addrspace(5) %"52", align 1
  %11 = load i1, ptr addrspace(5) %"52", align 1
  br i1 %11, label %"18", label %"19"

"18":                                             ; preds = %"44"
  %12 = load float, ptr addrspace(5) %"49", align 4
  store float %12, ptr addrspace(5) %"51", align 4
  br label %"19"

"19":                                             ; preds = %"18", %"44"
  %13 = load i1, ptr addrspace(5) %"52", align 1
  br i1 %13, label %"21", label %"20"

"20":                                             ; preds = %"19"
  %14 = load float, ptr addrspace(5) %"50", align 4
  store float %14, ptr addrspace(5) %"51", align 4
  br label %"21"

"21":                                             ; preds = %"20", %"19"
  %15 = load i64, ptr addrspace(5) %"48", align 8
  %16 = load float, ptr addrspace(5) %"51", align 4
  %"72" = inttoptr i64 %15 to ptr
  store float %16, ptr %"72", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }