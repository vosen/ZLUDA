declare hidden i32 @__zluda_ptx_impl_bfe_u32(i32, i32, i32) #0

define amdgpu_kernel void @bfe(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #1 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"45", align 8
  store i64 %2, ptr addrspace(5) %"47", align 8
  %3 = load i64, ptr addrspace(4) %"46", align 8
  store i64 %3, ptr addrspace(5) %"48", align 8
  %4 = load i64, ptr addrspace(5) %"47", align 8
  %"66" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"66", align 4
  store i32 %5, ptr addrspace(5) %"49", align 4
  %6 = load i64, ptr addrspace(5) %"47", align 8
  %"67" = inttoptr i64 %6 to ptr
  %"41" = getelementptr inbounds i8, ptr %"67", i64 4
  %7 = load i32, ptr %"41", align 4
  store i32 %7, ptr addrspace(5) %"50", align 4
  %8 = load i64, ptr addrspace(5) %"47", align 8
  %"68" = inttoptr i64 %8 to ptr
  %"43" = getelementptr inbounds i8, ptr %"68", i64 8
  %9 = load i32, ptr %"43", align 4
  store i32 %9, ptr addrspace(5) %"51", align 4
  %10 = load i32, ptr addrspace(5) %"49", align 4
  %11 = load i32, ptr addrspace(5) %"50", align 4
  %12 = load i32, ptr addrspace(5) %"51", align 4
  %"60" = call i32 @__zluda_ptx_impl_bfe_u32(i32 %10, i32 %11, i32 %12)
  store i32 %"60", ptr addrspace(5) %"49", align 4
  %13 = load i64, ptr addrspace(5) %"48", align 8
  %14 = load i32, ptr addrspace(5) %"49", align 4
  %"69" = inttoptr i64 %13 to ptr
  store i32 %14, ptr %"69", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
