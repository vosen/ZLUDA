define amdgpu_kernel void @setp_pred2(ptr addrspace(4) byref(i64) %"71", ptr addrspace(4) byref(i64) %"72") #0 {
  %"73" = alloca i64, align 8, addrspace(5)
  %"74" = alloca i64, align 8, addrspace(5)
  %"75" = alloca i32, align 4, addrspace(5)
  %"76" = alloca i32, align 4, addrspace(5)
  %"77" = alloca i32, align 4, addrspace(5)
  %"78" = alloca i32, align 4, addrspace(5)
  %"79" = alloca i32, align 4, addrspace(5)
  %"80" = alloca i32, align 4, addrspace(5)
  %"81" = alloca i32, align 4, addrspace(5)
  %"82" = alloca i1, align 1, addrspace(5)
  %"83" = alloca i1, align 1, addrspace(5)
  %"84" = alloca i1, align 1, addrspace(5)
  %"85" = alloca i1, align 1, addrspace(5)
  %"86" = alloca i1, align 1, addrspace(5)
  %"87" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"70"

"70":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"71", align 8
  store i64 %2, ptr addrspace(5) %"73", align 8
  %3 = load i64, ptr addrspace(4) %"72", align 8
  store i64 %3, ptr addrspace(5) %"74", align 8
  %4 = load i64, ptr addrspace(5) %"73", align 8
  %"127" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"127", align 4
  store i32 %5, ptr addrspace(5) %"75", align 4
  %6 = load i64, ptr addrspace(5) %"73", align 8
  %"128" = inttoptr i64 %6 to ptr
  %"51" = getelementptr inbounds i8, ptr %"128", i64 4
  %7 = load i32, ptr %"51", align 4
  store i32 %7, ptr addrspace(5) %"76", align 4
  %8 = load i32, ptr addrspace(5) %"75", align 4
  %9 = load i32, ptr addrspace(5) %"76", align 4
  %10 = icmp ult i32 %8, %9
  %11 = xor i1 %10, true
  store i1 %10, ptr addrspace(5) %"82", align 1
  store i1 %11, ptr addrspace(5) %"83", align 1
  %12 = load i32, ptr addrspace(5) %"76", align 4
  %13 = load i32, ptr addrspace(5) %"75", align 4
  %14 = icmp ult i32 %12, %13
  %15 = xor i1 %14, true
  store i1 %14, ptr addrspace(5) %"87", align 1
  store i1 %15, ptr addrspace(5) %"84", align 1
  %16 = load i32, ptr addrspace(5) %"75", align 4
  %17 = load i32, ptr addrspace(5) %"76", align 4
  %18 = load i1, ptr addrspace(5) %"82", align 1
  %19 = icmp ult i32 %16, %17
  %20 = xor i1 %19, %18
  %21 = xor i1 %19, true
  %22 = xor i1 %21, %18
  store i1 %20, ptr addrspace(5) %"85", align 1
  store i1 %22, ptr addrspace(5) %"86", align 1
  %23 = load i1, ptr addrspace(5) %"82", align 1
  %"107" = select i1 %23, i32 1, i32 0
  store i32 %"107", ptr addrspace(5) %"77", align 4
  %24 = load i1, ptr addrspace(5) %"83", align 1
  %"109" = select i1 %24, i32 1, i32 0
  store i32 %"109", ptr addrspace(5) %"78", align 4
  %25 = load i1, ptr addrspace(5) %"84", align 1
  %"111" = select i1 %25, i32 1, i32 0
  store i32 %"111", ptr addrspace(5) %"79", align 4
  %26 = load i1, ptr addrspace(5) %"85", align 1
  %"113" = select i1 %26, i32 1, i32 0
  store i32 %"113", ptr addrspace(5) %"80", align 4
  %27 = load i1, ptr addrspace(5) %"86", align 1
  %"115" = select i1 %27, i32 1, i32 0
  store i32 %"115", ptr addrspace(5) %"81", align 4
  %28 = load i64, ptr addrspace(5) %"74", align 8
  %29 = load i32, ptr addrspace(5) %"77", align 4
  %"129" = inttoptr i64 %28 to ptr
  store i32 %29, ptr %"129", align 4
  %30 = load i64, ptr addrspace(5) %"74", align 8
  %"130" = inttoptr i64 %30 to ptr
  %"63" = getelementptr inbounds i8, ptr %"130", i64 4
  %31 = load i32, ptr addrspace(5) %"78", align 4
  store i32 %31, ptr %"63", align 4
  %32 = load i64, ptr addrspace(5) %"74", align 8
  %"131" = inttoptr i64 %32 to ptr
  %"65" = getelementptr inbounds i8, ptr %"131", i64 8
  %33 = load i32, ptr addrspace(5) %"79", align 4
  store i32 %33, ptr %"65", align 4
  %34 = load i64, ptr addrspace(5) %"74", align 8
  %"132" = inttoptr i64 %34 to ptr
  %"67" = getelementptr inbounds i8, ptr %"132", i64 12
  %35 = load i32, ptr addrspace(5) %"80", align 4
  store i32 %35, ptr %"67", align 4
  %36 = load i64, ptr addrspace(5) %"74", align 8
  %"133" = inttoptr i64 %36 to ptr
  %"69" = getelementptr inbounds i8, ptr %"133", i64 16
  %37 = load i32, ptr addrspace(5) %"81", align 4
  store i32 %37, ptr %"69", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
