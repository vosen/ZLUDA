%struct.f64.f64.f64.i8 = type { double, double, double, i8 }

; Function Attrs: strictfp
declare hidden %struct.f64.f64.f64.i8 @__zluda_ptx_impl_div_f64_part1(double, double) #0

; Function Attrs: strictfp
declare hidden double @__zluda_ptx_impl_div_f64_part2(double, double, double, double, double, i8) #0

; Function Attrs: strictfp
define amdgpu_kernel void @rcp_f64(ptr addrspace(4) byref(i64) %"79", ptr addrspace(4) byref(i64) %"80") #1 {
  %"81" = alloca i64, align 8, addrspace(5)
  %"82" = alloca i64, align 8, addrspace(5)
  %"83" = alloca double, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"73"

"73":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"79", align 8
  store i64 %2, ptr addrspace(5) %"81", align 8
  %3 = load i64, ptr addrspace(4) %"80", align 8
  store i64 %3, ptr addrspace(5) %"82", align 8
  %4 = load i64, ptr addrspace(5) %"81", align 8
  %"93" = inttoptr i64 %4 to ptr
  %5 = load double, ptr %"93", align 8
  store double %5, ptr addrspace(5) %"83", align 8
  %6 = load double, ptr addrspace(5) %"83", align 8
  %7 = call %struct.f64.f64.f64.i8 @__zluda_ptx_impl_div_f64_part1(double 1.000000e+00, double %6)
  %"43" = extractvalue %struct.f64.f64.f64.i8 %7, 0
  %"44" = extractvalue %struct.f64.f64.f64.i8 %7, 1
  %"45" = extractvalue %struct.f64.f64.f64.i8 %7, 2
  %"46" = extractvalue %struct.f64.f64.f64.i8 %7, 3
  call void @llvm.amdgcn.s.setreg(i32 6145, i32 12)
  %8 = load double, ptr addrspace(5) %"83", align 8
  %"89" = call double @__zluda_ptx_impl_div_f64_part2(double 1.000000e+00, double %8, double %"43", double %"44", double %"45", i8 %"46")
  store double %"89", ptr addrspace(5) %"83", align 8
  %9 = load i64, ptr addrspace(5) %"82", align 8
  %10 = load double, ptr addrspace(5) %"83", align 8
  %"94" = inttoptr i64 %9 to ptr
  store double %10, ptr %"94", align 8
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

attributes #0 = { strictfp "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { strictfp "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }
