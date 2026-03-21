define amdgpu_kernel void @setp_bool_and(ptr addrspace(4) byref(i64) %"51", ptr addrspace(4) byref(i64) %"52") #0 {
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i32, align 4, addrspace(5)
  %"59" = alloca i1, align 1, addrspace(5)
  %"60" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"50"

"50":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"51", align 8
  store i64 %2, ptr addrspace(5) %"53", align 8
  %3 = load i64, ptr addrspace(4) %"52", align 8
  store i64 %3, ptr addrspace(5) %"54", align 8
  %4 = load i64, ptr addrspace(5) %"53", align 8
  %"79" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"79", align 4
  store i32 %5, ptr addrspace(5) %"55", align 4
  %6 = load i64, ptr addrspace(5) %"53", align 8
  %"80" = inttoptr i64 %6 to ptr
  %"44" = getelementptr inbounds i8, ptr %"80", i64 4
  %7 = load i32, ptr %"44", align 4
  store i32 %7, ptr addrspace(5) %"56", align 4
  %8 = load i64, ptr addrspace(5) %"53", align 8
  %"81" = inttoptr i64 %8 to ptr
  %"46" = getelementptr inbounds i8, ptr %"81", i64 8
  %9 = load i32, ptr %"46", align 4
  store i32 %9, ptr addrspace(5) %"57", align 4
  %10 = load i32, ptr addrspace(5) %"57", align 4
  %11 = icmp ne i32 %10, 0
  store i1 %11, ptr addrspace(5) %"60", align 1
  %12 = load i32, ptr addrspace(5) %"55", align 4
  %13 = load i32, ptr addrspace(5) %"56", align 4
  %14 = load i1, ptr addrspace(5) %"60", align 1
  %15 = icmp ult i32 %12, %13
  %16 = xor i1 %14, true
  %17 = and i1 %15, %16
  store i1 %17, ptr addrspace(5) %"59", align 1
  %18 = load i1, ptr addrspace(5) %"59", align 1
  %"75" = select i1 %18, i32 1, i32 0
  store i32 %"75", ptr addrspace(5) %"58", align 4
  %19 = load i64, ptr addrspace(5) %"54", align 8
  %20 = load i32, ptr addrspace(5) %"58", align 4
  %"82" = inttoptr i64 %19 to ptr
  store i32 %20, ptr %"82", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
