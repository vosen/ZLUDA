declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid_sub(ptr addrspace(4) byref(i64) %"52", ptr addrspace(4) byref(i64) %"53") #1 {
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i32, align 4, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"50"

"50":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"52", align 8
  store i64 %2, ptr addrspace(5) %"54", align 8
  %3 = load i64, ptr addrspace(4) %"53", align 8
  store i64 %3, ptr addrspace(5) %"57", align 8
  %4 = load i64, ptr addrspace(5) %"54", align 8
  %5 = inttoptr i64 %4 to ptr
  %6 = addrspacecast ptr %5 to ptr addrspace(1)
  %"65" = ptrtoint ptr addrspace(1) %6 to i64
  store i64 %"65", ptr addrspace(5) %"55", align 8
  %7 = load i64, ptr addrspace(5) %"57", align 8
  %8 = inttoptr i64 %7 to ptr
  %9 = addrspacecast ptr %8 to ptr addrspace(1)
  %"67" = ptrtoint ptr addrspace(1) %9 to i64
  store i64 %"67", ptr addrspace(5) %"58", align 8
  %"45" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"45", ptr addrspace(5) %"60", align 4
  %10 = load i32, ptr addrspace(5) %"60", align 4
  %11 = zext i32 %10 to i64
  store i64 %11, ptr addrspace(5) %"61", align 8
  %12 = load i64, ptr addrspace(5) %"55", align 8
  %13 = load i64, ptr addrspace(5) %"61", align 8
  %"84" = sub i64 %12, %13
  store i64 %"84", ptr addrspace(5) %"56", align 8
  %14 = load i64, ptr addrspace(5) %"58", align 8
  %15 = load i64, ptr addrspace(5) %"61", align 8
  %"87" = sub i64 %14, %15
  store i64 %"87", ptr addrspace(5) %"59", align 8
  %16 = load i64, ptr addrspace(5) %"56", align 8
  %"90" = inttoptr i64 %16 to ptr addrspace(1)
  %"47" = getelementptr inbounds i8, ptr addrspace(1) %"90", i64 0
  %17 = load i64, ptr addrspace(1) %"47", align 8
  store i64 %17, ptr addrspace(5) %"62", align 8
  %18 = load i64, ptr addrspace(5) %"59", align 8
  %"91" = inttoptr i64 %18 to ptr addrspace(1)
  %"49" = getelementptr inbounds i8, ptr addrspace(1) %"91", i64 0
  %19 = load i64, ptr addrspace(5) %"62", align 8
  store i64 %19, ptr addrspace(1) %"49", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
