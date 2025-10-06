declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid_chain(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #1 {
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"43"

"43":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"46", align 8
  store i64 %2, ptr addrspace(5) %"48", align 8
  %3 = load i64, ptr addrspace(4) %"47", align 8
  store i64 %3, ptr addrspace(5) %"51", align 8
  %4 = load i64, ptr addrspace(5) %"48", align 8
  %5 = inttoptr i64 %4 to ptr
  %"59" = addrspacecast ptr %5 to ptr addrspace(1)
  store ptr addrspace(1) %"59", ptr addrspace(5) %"49", align 8
  %6 = load i64, ptr addrspace(5) %"51", align 8
  %7 = inttoptr i64 %6 to ptr
  %"61" = addrspacecast ptr %7 to ptr addrspace(1)
  store ptr addrspace(1) %"61", ptr addrspace(5) %"52", align 8
  %"42" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"44"

"44":                                             ; preds = %"43"
  store i32 %"42", ptr addrspace(5) %"54", align 4
  %8 = load i32, ptr addrspace(5) %"54", align 4
  %"64" = zext i32 %8 to i64
  store i64 %"64", ptr addrspace(5) %"55", align 8
  %9 = load i64, ptr addrspace(5) %"49", align 8
  %10 = load i64, ptr addrspace(5) %"55", align 8
  %"78" = add i64 %9, %10
  store i64 %"78", ptr addrspace(5) %"50", align 8
  %11 = load i64, ptr addrspace(5) %"52", align 8
  %12 = load i64, ptr addrspace(5) %"55", align 8
  %"80" = add i64 %11, %12
  store i64 %"80", ptr addrspace(5) %"53", align 8
  %13 = load i64, ptr addrspace(5) %"50", align 8
  %"82" = inttoptr i64 %13 to ptr addrspace(1)
  %14 = load i64, ptr addrspace(1) %"82", align 8
  store i64 %14, ptr addrspace(5) %"56", align 8
  %15 = load i64, ptr addrspace(5) %"53", align 8
  %16 = load i64, ptr addrspace(5) %"56", align 8
  %"83" = inttoptr i64 %15 to ptr addrspace(1)
  store i64 %16, ptr addrspace(1) %"83", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }