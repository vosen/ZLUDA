declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #1 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"44", align 8
  store i64 %2, ptr addrspace(5) %"46", align 8
  %3 = load i64, ptr addrspace(4) %"45", align 8
  store i64 %3, ptr addrspace(5) %"47", align 8
  %4 = load i64, ptr addrspace(5) %"46", align 8
  %5 = inttoptr i64 %4 to ptr
  %6 = addrspacecast ptr %5 to ptr addrspace(1)
  %"53" = ptrtoint ptr addrspace(1) %6 to i64
  store i64 %"53", ptr addrspace(5) %"46", align 8
  %7 = load i64, ptr addrspace(5) %"47", align 8
  %8 = inttoptr i64 %7 to ptr
  %9 = addrspacecast ptr %8 to ptr addrspace(1)
  %"55" = ptrtoint ptr addrspace(1) %9 to i64
  store i64 %"55", ptr addrspace(5) %"47", align 8
  %"41" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"41", ptr addrspace(5) %"48", align 4
  %10 = load i32, ptr addrspace(5) %"48", align 4
  %11 = zext i32 %10 to i64
  store i64 %11, ptr addrspace(5) %"49", align 8
  %12 = load i64, ptr addrspace(5) %"46", align 8
  %13 = load i64, ptr addrspace(5) %"49", align 8
  %"72" = add i64 %12, %13
  store i64 %"72", ptr addrspace(5) %"46", align 8
  %14 = load i64, ptr addrspace(5) %"47", align 8
  %15 = load i64, ptr addrspace(5) %"49", align 8
  %"74" = add i64 %14, %15
  store i64 %"74", ptr addrspace(5) %"47", align 8
  %16 = load i64, ptr addrspace(5) %"46", align 8
  %"76" = inttoptr i64 %16 to ptr addrspace(1)
  %17 = load i64, ptr addrspace(1) %"76", align 8
  store i64 %17, ptr addrspace(5) %"50", align 8
  %18 = load i64, ptr addrspace(5) %"47", align 8
  %19 = load i64, ptr addrspace(5) %"50", align 8
  %"77" = inttoptr i64 %18 to ptr addrspace(1)
  store i64 %19, ptr addrspace(1) %"77", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
