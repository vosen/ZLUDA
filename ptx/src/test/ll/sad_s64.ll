define amdgpu_kernel void @sad_s64(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #0 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"49", align 8
  store i64 %2, ptr addrspace(5) %"51", align 8
  %3 = load i64, ptr addrspace(4) %"50", align 8
  store i64 %3, ptr addrspace(5) %"52", align 8
  %4 = load i64, ptr addrspace(5) %"51", align 8
  %"78" = inttoptr i64 %4 to ptr
  %5 = load i64, ptr %"78", align 8
  store i64 %5, ptr addrspace(5) %"53", align 8
  %6 = load i64, ptr addrspace(5) %"51", align 8
  %"79" = inttoptr i64 %6 to ptr
  %"43" = getelementptr inbounds i8, ptr %"79", i64 8
  %7 = load i64, ptr %"43", align 8
  store i64 %7, ptr addrspace(5) %"54", align 8
  %8 = load i64, ptr addrspace(5) %"51", align 8
  %"80" = inttoptr i64 %8 to ptr
  %"45" = getelementptr inbounds i8, ptr %"80", i64 16
  %9 = load i64, ptr %"45", align 8
  store i64 %9, ptr addrspace(5) %"55", align 8
  %10 = load i64, ptr addrspace(5) %"53", align 8
  %11 = load i64, ptr addrspace(5) %"54", align 8
  %12 = load i64, ptr addrspace(5) %"55", align 8
  %13 = icmp slt i64 %10, %11
  %14 = sub i64 %10, %11
  %15 = sub i64 %11, %10
  %16 = select i1 %13, i64 %15, i64 %14
  %"66" = add i64 %16, %12
  store i64 %"66", ptr addrspace(5) %"56", align 8
  %17 = load i64, ptr addrspace(5) %"54", align 8
  %18 = load i64, ptr addrspace(5) %"53", align 8
  %19 = load i64, ptr addrspace(5) %"55", align 8
  %20 = icmp slt i64 %17, %18
  %21 = sub i64 %17, %18
  %22 = sub i64 %18, %17
  %23 = select i1 %20, i64 %22, i64 %21
  %"70" = add i64 %23, %19
  store i64 %"70", ptr addrspace(5) %"57", align 8
  %24 = load i64, ptr addrspace(5) %"52", align 8
  %25 = load i64, ptr addrspace(5) %"56", align 8
  %"81" = inttoptr i64 %24 to ptr
  store i64 %25, ptr %"81", align 8
  %26 = load i64, ptr addrspace(5) %"52", align 8
  %"82" = inttoptr i64 %26 to ptr
  %"47" = getelementptr inbounds i8, ptr %"82", i64 8
  %27 = load i64, ptr addrspace(5) %"57", align 8
  store i64 %27, ptr %"47", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
