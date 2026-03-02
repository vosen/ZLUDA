define amdgpu_kernel void @setp_leu(ptr addrspace(4) byref(i64) %"48", ptr addrspace(4) byref(i64) %"49") #0 {
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca float, align 4, addrspace(5)
  %"53" = alloca float, align 4, addrspace(5)
  %"54" = alloca float, align 4, addrspace(5)
  %"55" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"48", align 8
  store i64 %2, ptr addrspace(5) %"50", align 8
  %3 = load i64, ptr addrspace(4) %"49", align 8
  store i64 %3, ptr addrspace(5) %"51", align 8
  %4 = load i64, ptr addrspace(5) %"50", align 8
  %"73" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"73", align 4
  store float %5, ptr addrspace(5) %"52", align 4
  %6 = load i64, ptr addrspace(5) %"50", align 8
  %"74" = inttoptr i64 %6 to ptr
  %"46" = getelementptr inbounds i8, ptr %"74", i64 4
  %7 = load float, ptr %"46", align 4
  store float %7, ptr addrspace(5) %"53", align 4
  %8 = load float, ptr addrspace(5) %"52", align 4
  %9 = load float, ptr addrspace(5) %"53", align 4
  %10 = fcmp ule float %8, %9
  store i1 %10, ptr addrspace(5) %"55", align 1
  %11 = load i1, ptr addrspace(5) %"55", align 1
  br i1 %11, label %"19", label %"20"

"19":                                             ; preds = %"47"
  %12 = load float, ptr addrspace(5) %"52", align 4
  store float %12, ptr addrspace(5) %"54", align 4
  br label %"20"

"20":                                             ; preds = %"19", %"47"
  %13 = load i1, ptr addrspace(5) %"55", align 1
  br i1 %13, label %"22", label %"21"

"21":                                             ; preds = %"20"
  %14 = load float, ptr addrspace(5) %"53", align 4
  store float %14, ptr addrspace(5) %"54", align 4
  br label %"22"

"22":                                             ; preds = %"21", %"20"
  %15 = load i64, ptr addrspace(5) %"51", align 8
  %16 = load float, ptr addrspace(5) %"54", align 4
  %"75" = inttoptr i64 %15 to ptr
  store float %16, ptr %"75", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
