define amdgpu_kernel void @pred_not(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #0 {
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"47", align 8
  store i64 %2, ptr addrspace(5) %"49", align 8
  %3 = load i64, ptr addrspace(4) %"48", align 8
  store i64 %3, ptr addrspace(5) %"50", align 8
  %4 = load i64, ptr addrspace(5) %"49", align 8
  %"72" = inttoptr i64 %4 to ptr
  %5 = load i64, ptr %"72", align 8
  store i64 %5, ptr addrspace(5) %"51", align 8
  %6 = load i64, ptr addrspace(5) %"49", align 8
  %"73" = inttoptr i64 %6 to ptr
  %"43" = getelementptr inbounds i8, ptr %"73", i64 8
  %7 = load i64, ptr %"43", align 8
  store i64 %7, ptr addrspace(5) %"52", align 8
  %8 = load i64, ptr addrspace(5) %"51", align 8
  %9 = load i64, ptr addrspace(5) %"52", align 8
  %10 = icmp ult i64 %8, %9
  store i1 %10, ptr addrspace(5) %"54", align 1
  %11 = load i1, ptr addrspace(5) %"54", align 1
  %"64" = xor i1 %11, true
  store i1 %"64", ptr addrspace(5) %"54", align 1
  %12 = load i1, ptr addrspace(5) %"54", align 1
  br i1 %12, label %"18", label %"19"

"18":                                             ; preds = %"46"
  store i64 1, ptr addrspace(5) %"53", align 8
  br label %"19"

"19":                                             ; preds = %"18", %"46"
  %13 = load i1, ptr addrspace(5) %"54", align 1
  br i1 %13, label %"21", label %"20"

"20":                                             ; preds = %"19"
  store i64 2, ptr addrspace(5) %"53", align 8
  br label %"21"

"21":                                             ; preds = %"20", %"19"
  %14 = load i64, ptr addrspace(5) %"50", align 8
  %15 = load i64, ptr addrspace(5) %"53", align 8
  %"74" = inttoptr i64 %14 to ptr
  store i64 %15, ptr %"74", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }