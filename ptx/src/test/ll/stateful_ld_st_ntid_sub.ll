declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid_sub(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #1 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"49", align 8
  store i64 %2, ptr addrspace(5) %"51", align 8
  %3 = load i64, ptr addrspace(4) %"50", align 8
  store i64 %3, ptr addrspace(5) %"54", align 8
  %4 = load i64, ptr addrspace(5) %"51", align 8
  %5 = inttoptr i64 %4 to ptr
  %"62" = addrspacecast ptr %5 to ptr addrspace(1)
  store ptr addrspace(1) %"62", ptr addrspace(5) %"52", align 8
  %6 = load i64, ptr addrspace(5) %"54", align 8
  %7 = inttoptr i64 %6 to ptr
  %"64" = addrspacecast ptr %7 to ptr addrspace(1)
  store ptr addrspace(1) %"64", ptr addrspace(5) %"55", align 8
  %"42" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"42", ptr addrspace(5) %"57", align 4
  %8 = load i32, ptr addrspace(5) %"57", align 4
  %9 = zext i32 %8 to i64
  store i64 %9, ptr addrspace(5) %"58", align 8
  %10 = load i64, ptr addrspace(5) %"52", align 8
  %11 = load i64, ptr addrspace(5) %"58", align 8
  %"81" = sub i64 %10, %11
  store i64 %"81", ptr addrspace(5) %"53", align 8
  %12 = load i64, ptr addrspace(5) %"55", align 8
  %13 = load i64, ptr addrspace(5) %"58", align 8
  %"84" = sub i64 %12, %13
  store i64 %"84", ptr addrspace(5) %"56", align 8
  %14 = load i64, ptr addrspace(5) %"53", align 8
  %"87" = inttoptr i64 %14 to ptr addrspace(1)
  %"44" = getelementptr inbounds i8, ptr addrspace(1) %"87", i64 0
  %15 = load i64, ptr addrspace(1) %"44", align 8
  store i64 %15, ptr addrspace(5) %"59", align 8
  %16 = load i64, ptr addrspace(5) %"56", align 8
  %"88" = inttoptr i64 %16 to ptr addrspace(1)
  %"46" = getelementptr inbounds i8, ptr addrspace(1) %"88", i64 0
  %17 = load i64, ptr addrspace(5) %"59", align 8
  store i64 %17, ptr addrspace(1) %"46", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }