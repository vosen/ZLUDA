define amdgpu_kernel void @sad_s64(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #0 {
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"46", align 8
  store i64 %2, ptr addrspace(5) %"48", align 8
  %3 = load i64, ptr addrspace(4) %"47", align 8
  store i64 %3, ptr addrspace(5) %"49", align 8
  %4 = load i64, ptr addrspace(5) %"48", align 8
  %"75" = inttoptr i64 %4 to ptr
  %5 = load i64, ptr %"75", align 8
  store i64 %5, ptr addrspace(5) %"50", align 8
  %6 = load i64, ptr addrspace(5) %"48", align 8
  %"76" = inttoptr i64 %6 to ptr
  %"40" = getelementptr inbounds i8, ptr %"76", i64 8
  %7 = load i64, ptr %"40", align 8
  store i64 %7, ptr addrspace(5) %"51", align 8
  %8 = load i64, ptr addrspace(5) %"48", align 8
  %"77" = inttoptr i64 %8 to ptr
  %"42" = getelementptr inbounds i8, ptr %"77", i64 16
  %9 = load i64, ptr %"42", align 8
  store i64 %9, ptr addrspace(5) %"52", align 8
  %10 = load i64, ptr addrspace(5) %"50", align 8
  %11 = load i64, ptr addrspace(5) %"51", align 8
  %12 = load i64, ptr addrspace(5) %"52", align 8
  %13 = icmp slt i64 %10, %11
  %14 = sub i64 %10, %11
  %15 = sub i64 %11, %10
  %16 = select i1 %13, i64 %15, i64 %14
  %"63" = add i64 %16, %12
  store i64 %"63", ptr addrspace(5) %"53", align 8
  %17 = load i64, ptr addrspace(5) %"51", align 8
  %18 = load i64, ptr addrspace(5) %"50", align 8
  %19 = load i64, ptr addrspace(5) %"52", align 8
  %20 = icmp slt i64 %17, %18
  %21 = sub i64 %17, %18
  %22 = sub i64 %18, %17
  %23 = select i1 %20, i64 %22, i64 %21
  %"67" = add i64 %23, %19
  store i64 %"67", ptr addrspace(5) %"54", align 8
  %24 = load i64, ptr addrspace(5) %"49", align 8
  %25 = load i64, ptr addrspace(5) %"53", align 8
  %"78" = inttoptr i64 %24 to ptr
  store i64 %25, ptr %"78", align 8
  %26 = load i64, ptr addrspace(5) %"49", align 8
  %"79" = inttoptr i64 %26 to ptr
  %"44" = getelementptr inbounds i8, ptr %"79", i64 8
  %27 = load i64, ptr addrspace(5) %"54", align 8
  store i64 %27, ptr %"44", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
