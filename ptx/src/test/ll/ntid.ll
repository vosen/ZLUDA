declare hidden i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

define amdgpu_kernel void @ntid(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #1 {
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %2, ptr addrspace(5) %"45", align 8
  %3 = load i64, ptr addrspace(4) %"44", align 8
  store i64 %3, ptr addrspace(5) %"46", align 8
  %4 = load i64, ptr addrspace(5) %"45", align 8
  %"59" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"59", align 4
  store i32 %5, ptr addrspace(5) %"47", align 4
  %"40" = call i32 @__zluda_ptx_impl_sreg_ntid(i8 0)
  store i32 %"40", ptr addrspace(5) %"48", align 4
  %6 = load i32, ptr addrspace(5) %"47", align 4
  %7 = load i32, ptr addrspace(5) %"48", align 4
  %"54" = add i32 %6, %7
  store i32 %"54", ptr addrspace(5) %"47", align 4
  %8 = load i64, ptr addrspace(5) %"46", align 8
  %9 = load i32, ptr addrspace(5) %"47", align 4
  %"60" = inttoptr i64 %8 to ptr
  store i32 %9, ptr %"60", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
