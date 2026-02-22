define amdgpu_kernel void @pred_not(ptr addrspace(4) byref(i64) %"50", ptr addrspace(4) byref(i64) %"51") #0 {
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"49"

"49":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"50", align 8
  store i64 %2, ptr addrspace(5) %"52", align 8
  %3 = load i64, ptr addrspace(4) %"51", align 8
  store i64 %3, ptr addrspace(5) %"53", align 8
  %4 = load i64, ptr addrspace(5) %"52", align 8
  %"75" = inttoptr i64 %4 to ptr
  %5 = load i64, ptr %"75", align 8
  store i64 %5, ptr addrspace(5) %"54", align 8
  %6 = load i64, ptr addrspace(5) %"52", align 8
  %"76" = inttoptr i64 %6 to ptr
  %"46" = getelementptr inbounds i8, ptr %"76", i64 8
  %7 = load i64, ptr %"46", align 8
  store i64 %7, ptr addrspace(5) %"55", align 8
  %8 = load i64, ptr addrspace(5) %"54", align 8
  %9 = load i64, ptr addrspace(5) %"55", align 8
  %10 = icmp ult i64 %8, %9
  store i1 %10, ptr addrspace(5) %"57", align 1
  %11 = load i1, ptr addrspace(5) %"57", align 1
  %12 = xor i1 %11, true
  store i1 %12, ptr addrspace(5) %"57", align 1
  %13 = load i1, ptr addrspace(5) %"57", align 1
  br i1 %13, label %"19", label %"20"

"19":                                             ; preds = %"49"
  store i64 1, ptr addrspace(5) %"56", align 8
  br label %"20"

"20":                                             ; preds = %"19", %"49"
  %14 = load i1, ptr addrspace(5) %"57", align 1
  br i1 %14, label %"22", label %"21"

"21":                                             ; preds = %"20"
  store i64 2, ptr addrspace(5) %"56", align 8
  br label %"22"

"22":                                             ; preds = %"21", %"20"
  %15 = load i64, ptr addrspace(5) %"53", align 8
  %16 = load i64, ptr addrspace(5) %"56", align 8
  %"77" = inttoptr i64 %15 to ptr
  store i64 %16, ptr %"77", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
