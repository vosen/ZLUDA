%struct.f32.f32.f32.i8 = type { float, float, float, i8 }

; Function Attrs: strictfp
declare hidden %struct.f32.f32.f32.i8 @__zluda_ptx_impl_div_f32_part1(float, float) #0

; Function Attrs: strictfp
declare hidden float @__zluda_ptx_impl_div_f32_part2(float, float, float, float, float, i8) #0

; Function Attrs: strictfp
define amdgpu_kernel void @div_noftz(ptr addrspace(4) byref(i64) %"84", ptr addrspace(4) byref(i64) %"85") #1 {
  %"86" = alloca i64, align 8, addrspace(5)
  %"87" = alloca i64, align 8, addrspace(5)
  %"88" = alloca float, align 4, addrspace(5)
  %"89" = alloca float, align 4, addrspace(5)
  %"90" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"78"

"78":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"84", align 8
  store i64 %2, ptr addrspace(5) %"86", align 8
  %3 = load i64, ptr addrspace(4) %"85", align 8
  store i64 %3, ptr addrspace(5) %"87", align 8
  %4 = load i64, ptr addrspace(5) %"86", align 8
  %"109" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"109", align 4
  store float %5, ptr addrspace(5) %"88", align 4
  %6 = load i64, ptr addrspace(5) %"86", align 8
  %"110" = inttoptr i64 %6 to ptr
  %"41" = getelementptr inbounds i8, ptr %"110", i64 4
  %7 = load float, ptr %"41", align 4
  store float %7, ptr addrspace(5) %"89", align 4
  %8 = load float, ptr addrspace(5) %"88", align 4
  %9 = load float, ptr addrspace(5) %"89", align 4
  %"97" = call float @llvm.experimental.constrained.fmul.f32(float %8, float %9, metadata !"round.dynamic", metadata !"fpexcept.ignore")
  store float %"97", ptr addrspace(5) %"90", align 4
  call void @llvm.amdgcn.s.setreg(i32 6401, i32 3)
  %10 = load float, ptr addrspace(5) %"88", align 4
  %11 = load float, ptr addrspace(5) %"89", align 4
  %12 = call %struct.f32.f32.f32.i8 @__zluda_ptx_impl_div_f32_part1(float %10, float %11)
  %"48" = extractvalue %struct.f32.f32.f32.i8 %12, 0
  %"49" = extractvalue %struct.f32.f32.f32.i8 %12, 1
  %"50" = extractvalue %struct.f32.f32.f32.i8 %12, 2
  %"51" = extractvalue %struct.f32.f32.f32.i8 %12, 3
  %13 = load float, ptr addrspace(5) %"88", align 4
  %14 = load float, ptr addrspace(5) %"89", align 4
  %"102" = call float @__zluda_ptx_impl_div_f32_part2(float %13, float %14, float %"48", float %"49", float %"50", i8 %"51")
  store float %"102", ptr addrspace(5) %"88", align 4
  %15 = load i64, ptr addrspace(5) %"87", align 8
  %16 = load float, ptr addrspace(5) %"88", align 4
  %"111" = inttoptr i64 %15 to ptr
  store float %16, ptr %"111", align 4
  %17 = load i64, ptr addrspace(5) %"87", align 8
  %"112" = inttoptr i64 %17 to ptr
  %"43" = getelementptr inbounds i8, ptr %"112", i64 4
  %18 = load float, ptr addrspace(5) %"90", align 4
  store float %18, ptr %"43", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind strictfp willreturn memory(inaccessiblemem: readwrite)
declare float @llvm.experimental.constrained.fmul.f32(float, float, metadata, metadata) #2

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #3

attributes #0 = { strictfp "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { strictfp "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind strictfp willreturn memory(inaccessiblemem: readwrite) }
attributes #3 = { nocallback nofree nosync nounwind willreturn }
