declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #1 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %2, ptr addrspace(5) %"43", align 8
  %3 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %3, ptr addrspace(5) %"44", align 8
  %4 = load i64, ptr addrspace(5) %"43", align 8
  %5 = inttoptr i64 %4 to ptr
  %"50" = addrspacecast ptr %5 to ptr addrspace(1)
  store ptr addrspace(1) %"50", ptr addrspace(5) %"43", align 8
  %6 = load i64, ptr addrspace(5) %"44", align 8
  %7 = inttoptr i64 %6 to ptr
  %"52" = addrspacecast ptr %7 to ptr addrspace(1)
  store ptr addrspace(1) %"52", ptr addrspace(5) %"44", align 8
  %"38" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"38", ptr addrspace(5) %"45", align 4
  %8 = load i32, ptr addrspace(5) %"45", align 4
  %9 = zext i32 %8 to i64
  store i64 %9, ptr addrspace(5) %"46", align 8
  %10 = load i64, ptr addrspace(5) %"43", align 8
  %11 = load i64, ptr addrspace(5) %"46", align 8
  %"69" = add i64 %10, %11
  store i64 %"69", ptr addrspace(5) %"43", align 8
  %12 = load i64, ptr addrspace(5) %"44", align 8
  %13 = load i64, ptr addrspace(5) %"46", align 8
  %"71" = add i64 %12, %13
  store i64 %"71", ptr addrspace(5) %"44", align 8
  %14 = load i64, ptr addrspace(5) %"43", align 8
  %"73" = inttoptr i64 %14 to ptr addrspace(1)
  %15 = load i64, ptr addrspace(1) %"73", align 8
  store i64 %15, ptr addrspace(5) %"47", align 8
  %16 = load i64, ptr addrspace(5) %"44", align 8
  %17 = load i64, ptr addrspace(5) %"47", align 8
  %"74" = inttoptr i64 %16 to ptr addrspace(1)
  store i64 %17, ptr addrspace(1) %"74", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }