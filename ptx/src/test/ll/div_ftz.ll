%struct.f32.f32.f32.i8 = type { float, float, float, i8 }

declare hidden %struct.f32.f32.f32.i8 @__zluda_ptx_impl_div_f32_part1(float, float) #0

declare hidden float @__zluda_ptx_impl_div_f32_part2(float, float, float, float, float, i8) #0

define amdgpu_kernel void @div_ftz(ptr addrspace(4) byref(i64) %"66", ptr addrspace(4) byref(i64) %"67") #1 {
  %"68" = alloca i64, align 8, addrspace(5)
  %"69" = alloca i64, align 8, addrspace(5)
  %"70" = alloca float, align 4, addrspace(5)
  %"71" = alloca float, align 4, addrspace(5)
  %"72" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"60"

"60":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"66", align 8
  store i64 %2, ptr addrspace(5) %"68", align 8
  %3 = load i64, ptr addrspace(4) %"67", align 8
  store i64 %3, ptr addrspace(5) %"69", align 8
  %4 = load i64, ptr addrspace(5) %"68", align 8
  %"91" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"91", align 4
  store float %5, ptr addrspace(5) %"70", align 4
  %6 = load i64, ptr addrspace(5) %"68", align 8
  %"92" = inttoptr i64 %6 to ptr
  %"38" = getelementptr inbounds i8, ptr %"92", i64 4
  %7 = load float, ptr %"38", align 4
  store float %7, ptr addrspace(5) %"71", align 4
  %8 = load float, ptr addrspace(5) %"70", align 4
  %9 = load float, ptr addrspace(5) %"71", align 4
  %"79" = fmul float %8, %9
  store float %"79", ptr addrspace(5) %"72", align 4
  %10 = load float, ptr addrspace(5) %"70", align 4
  %11 = load float, ptr addrspace(5) %"71", align 4
  %12 = call %struct.f32.f32.f32.i8 @__zluda_ptx_impl_div_f32_part1(float %10, float %11)
  %"43" = extractvalue %struct.f32.f32.f32.i8 %12, 0
  %"44" = extractvalue %struct.f32.f32.f32.i8 %12, 1
  %"45" = extractvalue %struct.f32.f32.f32.i8 %12, 2
  %"46" = extractvalue %struct.f32.f32.f32.i8 %12, 3
  call void @llvm.amdgcn.s.setreg(i32 6401, i32 0)
  %13 = load float, ptr addrspace(5) %"70", align 4
  %14 = load float, ptr addrspace(5) %"71", align 4
  %"84" = call float @__zluda_ptx_impl_div_f32_part2(float %13, float %14, float %"43", float %"44", float %"45", i8 %"46")
  store float %"84", ptr addrspace(5) %"70", align 4
  %15 = load i64, ptr addrspace(5) %"69", align 8
  %16 = load float, ptr addrspace(5) %"70", align 4
  %"93" = inttoptr i64 %15 to ptr
  store float %16, ptr %"93", align 4
  %17 = load i64, ptr addrspace(5) %"69", align 8
  %"94" = inttoptr i64 %17 to ptr
  %"40" = getelementptr inbounds i8, ptr %"94", i64 4
  %18 = load float, ptr addrspace(5) %"72", align 4
  store float %18, ptr %"40", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }