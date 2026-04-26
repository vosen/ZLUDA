define amdgpu_kernel void @vshr_clamp(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #0 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"49", align 8
  store i64 %2, ptr addrspace(5) %"51", align 8
  %3 = load i64, ptr addrspace(4) %"50", align 8
  store i64 %3, ptr addrspace(5) %"52", align 8
  %4 = load i64, ptr addrspace(5) %"51", align 8
  %"76" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"76", align 4
  store i32 %5, ptr addrspace(5) %"54", align 4
  %6 = load i64, ptr addrspace(5) %"51", align 8
  %"77" = inttoptr i64 %6 to ptr
  %"42" = getelementptr inbounds i8, ptr %"77", i64 4
  %7 = load i32, ptr %"42", align 4
  store i32 %7, ptr addrspace(5) %"55", align 4
  %8 = load i64, ptr addrspace(5) %"51", align 8
  %"78" = inttoptr i64 %8 to ptr
  %"44" = getelementptr inbounds i8, ptr %"78", i64 8
  %9 = load i32, ptr %"44", align 4
  store i32 %9, ptr addrspace(5) %"56", align 4
  %10 = load i32, ptr addrspace(5) %"54", align 4
  %11 = load i32, ptr addrspace(5) %"55", align 4
  %12 = load i32, ptr addrspace(5) %"56", align 4
  %13 = lshr i32 %10, %11
  %14 = icmp uge i32 %11, 32
  %15 = select i1 %14, i32 0, i32 %13
  %"65" = add i32 %15, %12
  store i32 %"65", ptr addrspace(5) %"53", align 4
  %16 = load i64, ptr addrspace(5) %"52", align 8
  %17 = load i32, ptr addrspace(5) %"53", align 4
  %"79" = inttoptr i64 %16 to ptr
  store i32 %17, ptr %"79", align 4
  %18 = load i32, ptr addrspace(5) %"54", align 4
  %19 = load i32, ptr addrspace(5) %"56", align 4
  %20 = lshr i32 %18, 32
  %21 = select i1 true, i32 0, i32 %20
  %"71" = add i32 %21, %19
  store i32 %"71", ptr addrspace(5) %"53", align 4
  %22 = load i64, ptr addrspace(5) %"52", align 8
  %"80" = inttoptr i64 %22 to ptr
  %"47" = getelementptr inbounds i8, ptr %"80", i64 4
  %23 = load i32, ptr addrspace(5) %"53", align 4
  store i32 %23, ptr %"47", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
