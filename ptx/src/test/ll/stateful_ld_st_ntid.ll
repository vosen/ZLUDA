declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #1 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %2, ptr addrspace(5) %"44", align 8
  %3 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %3, ptr addrspace(5) %"45", align 8
  %4 = load i64, ptr addrspace(5) %"44", align 8
  %5 = inttoptr i64 %4 to ptr
  %"51" = addrspacecast ptr %5 to ptr addrspace(1)
  store ptr addrspace(1) %"51", ptr addrspace(5) %"44", align 8
  %6 = load i64, ptr addrspace(5) %"45", align 8
  %7 = inttoptr i64 %6 to ptr
  %"53" = addrspacecast ptr %7 to ptr addrspace(1)
  store ptr addrspace(1) %"53", ptr addrspace(5) %"45", align 8
  %"38" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"40"

"40":                                             ; preds = %"39"
  store i32 %"38", ptr addrspace(5) %"46", align 4
  %8 = load i32, ptr addrspace(5) %"46", align 4
  %"56" = zext i32 %8 to i64
  store i64 %"56", ptr addrspace(5) %"47", align 8
  %9 = load i64, ptr addrspace(5) %"44", align 8
  %10 = load i64, ptr addrspace(5) %"47", align 8
  %"70" = add i64 %9, %10
  store i64 %"70", ptr addrspace(5) %"44", align 8
  %11 = load i64, ptr addrspace(5) %"45", align 8
  %12 = load i64, ptr addrspace(5) %"47", align 8
  %"72" = add i64 %11, %12
  store i64 %"72", ptr addrspace(5) %"45", align 8
  %13 = load i64, ptr addrspace(5) %"44", align 8
  %"74" = inttoptr i64 %13 to ptr addrspace(1)
  %14 = load i64, ptr addrspace(1) %"74", align 8
  store i64 %14, ptr addrspace(5) %"48", align 8
  %15 = load i64, ptr addrspace(5) %"45", align 8
  %16 = load i64, ptr addrspace(5) %"48", align 8
  %"75" = inttoptr i64 %15 to ptr addrspace(1)
  store i64 %16, ptr addrspace(1) %"75", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }