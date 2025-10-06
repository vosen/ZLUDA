declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid_sub(ptr addrspace(4) byref(i64) %"50", ptr addrspace(4) byref(i64) %"51") #1 {
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i32, align 4, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"50", align 8
  store i64 %2, ptr addrspace(5) %"52", align 8
  %3 = load i64, ptr addrspace(4) %"51", align 8
  store i64 %3, ptr addrspace(5) %"55", align 8
  %4 = load i64, ptr addrspace(5) %"52", align 8
  %5 = inttoptr i64 %4 to ptr
  %"63" = addrspacecast ptr %5 to ptr addrspace(1)
  store ptr addrspace(1) %"63", ptr addrspace(5) %"53", align 8
  %6 = load i64, ptr addrspace(5) %"55", align 8
  %7 = inttoptr i64 %6 to ptr
  %"65" = addrspacecast ptr %7 to ptr addrspace(1)
  store ptr addrspace(1) %"65", ptr addrspace(5) %"56", align 8
  %"42" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"48"

"48":                                             ; preds = %"47"
  store i32 %"42", ptr addrspace(5) %"58", align 4
  %8 = load i32, ptr addrspace(5) %"58", align 4
  %"68" = zext i32 %8 to i64
  store i64 %"68", ptr addrspace(5) %"59", align 8
  %9 = load i64, ptr addrspace(5) %"53", align 8
  %10 = load i64, ptr addrspace(5) %"59", align 8
  %"82" = sub i64 %9, %10
  store i64 %"82", ptr addrspace(5) %"54", align 8
  %11 = load i64, ptr addrspace(5) %"56", align 8
  %12 = load i64, ptr addrspace(5) %"59", align 8
  %"85" = sub i64 %11, %12
  store i64 %"85", ptr addrspace(5) %"57", align 8
  %13 = load i64, ptr addrspace(5) %"54", align 8
  %"88" = inttoptr i64 %13 to ptr addrspace(1)
  %"44" = getelementptr inbounds i8, ptr addrspace(1) %"88", i64 0
  %14 = load i64, ptr addrspace(1) %"44", align 8
  store i64 %14, ptr addrspace(5) %"60", align 8
  %15 = load i64, ptr addrspace(5) %"57", align 8
  %"89" = inttoptr i64 %15 to ptr addrspace(1)
  %"46" = getelementptr inbounds i8, ptr addrspace(1) %"89", i64 0
  %16 = load i64, ptr addrspace(5) %"60", align 8
  store i64 %16, ptr addrspace(1) %"46", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }