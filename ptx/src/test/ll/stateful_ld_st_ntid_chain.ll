declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid_chain(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #1 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"43"

"43":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"45", align 8
  store i64 %2, ptr addrspace(5) %"47", align 8
  %3 = load i64, ptr addrspace(4) %"46", align 8
  store i64 %3, ptr addrspace(5) %"50", align 8
  %4 = load i64, ptr addrspace(5) %"47", align 8
  %5 = inttoptr i64 %4 to ptr
  %"58" = addrspacecast ptr %5 to ptr addrspace(1)
  store ptr addrspace(1) %"58", ptr addrspace(5) %"48", align 8
  %6 = load i64, ptr addrspace(5) %"50", align 8
  %7 = inttoptr i64 %6 to ptr
  %"60" = addrspacecast ptr %7 to ptr addrspace(1)
  store ptr addrspace(1) %"60", ptr addrspace(5) %"51", align 8
  %"42" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"42", ptr addrspace(5) %"53", align 4
  %8 = load i32, ptr addrspace(5) %"53", align 4
  %9 = zext i32 %8 to i64
  store i64 %9, ptr addrspace(5) %"54", align 8
  %10 = load i64, ptr addrspace(5) %"48", align 8
  %11 = load i64, ptr addrspace(5) %"54", align 8
  %"77" = add i64 %10, %11
  store i64 %"77", ptr addrspace(5) %"49", align 8
  %12 = load i64, ptr addrspace(5) %"51", align 8
  %13 = load i64, ptr addrspace(5) %"54", align 8
  %"79" = add i64 %12, %13
  store i64 %"79", ptr addrspace(5) %"52", align 8
  %14 = load i64, ptr addrspace(5) %"49", align 8
  %"81" = inttoptr i64 %14 to ptr addrspace(1)
  %15 = load i64, ptr addrspace(1) %"81", align 8
  store i64 %15, ptr addrspace(5) %"55", align 8
  %16 = load i64, ptr addrspace(5) %"52", align 8
  %17 = load i64, ptr addrspace(5) %"55", align 8
  %"82" = inttoptr i64 %16 to ptr addrspace(1)
  store i64 %17, ptr addrspace(1) %"82", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }