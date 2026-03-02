@from = addrspace(1) externally_initialized global [4 x i32] [i32 1, i32 2, i32 3, i32 4]
@to = external addrspace(3) global [4 x i32]

define amdgpu_kernel void @cp_async(ptr addrspace(4) byref(i64) %"57", ptr addrspace(4) byref(i64) %"58") #0 {
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i32, align 4, addrspace(5)
  %"62" = alloca i32, align 4, addrspace(5)
  %"63" = alloca i32, align 4, addrspace(5)
  %"64" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"56"

"56":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"57", align 8
  store i64 %2, ptr addrspace(5) %"59", align 8
  %3 = load i64, ptr addrspace(4) %"58", align 8
  store i64 %3, ptr addrspace(5) %"60", align 8
  %4 = load i96, ptr addrspace(1) @from, align 16
  %5 = zext i96 %4 to i128
  store i128 %5, ptr addrspace(3) @to, align 16
  %6 = load i32, ptr addrspacecast (ptr addrspace(3) @to to ptr), align 4
  store i32 %6, ptr addrspace(5) %"61", align 4
  %7 = load i32, ptr getelementptr inbounds (i8, ptr addrspacecast (ptr addrspace(3) @to to ptr), i64 4), align 4
  store i32 %7, ptr addrspace(5) %"62", align 4
  %8 = load i32, ptr getelementptr inbounds (i8, ptr addrspacecast (ptr addrspace(3) @to to ptr), i64 8), align 4
  store i32 %8, ptr addrspace(5) %"63", align 4
  %9 = load i32, ptr getelementptr inbounds (i8, ptr addrspacecast (ptr addrspace(3) @to to ptr), i64 12), align 4
  store i32 %9, ptr addrspace(5) %"64", align 4
  %10 = load i64, ptr addrspace(5) %"60", align 8
  %11 = load i32, ptr addrspace(5) %"61", align 4
  %"85" = inttoptr i64 %10 to ptr
  store i32 %11, ptr %"85", align 4
  %12 = load i64, ptr addrspace(5) %"60", align 8
  %"86" = inttoptr i64 %12 to ptr
  %"51" = getelementptr inbounds i8, ptr %"86", i64 4
  %13 = load i32, ptr addrspace(5) %"62", align 4
  store i32 %13, ptr %"51", align 4
  %14 = load i64, ptr addrspace(5) %"60", align 8
  %"87" = inttoptr i64 %14 to ptr
  %"53" = getelementptr inbounds i8, ptr %"87", i64 8
  %15 = load i32, ptr addrspace(5) %"63", align 4
  store i32 %15, ptr %"53", align 4
  %16 = load i64, ptr addrspace(5) %"60", align 8
  %"88" = inttoptr i64 %16 to ptr
  %"55" = getelementptr inbounds i8, ptr %"88", i64 12
  %17 = load i32, ptr addrspace(5) %"64", align 4
  store i32 %17, ptr %"55", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
