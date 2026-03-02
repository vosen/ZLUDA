define hidden float @add_rm(float %"88", float %"89") #0 {
  %"137" = alloca float, align 4, addrspace(5)
  %"138" = alloca float, align 4, addrspace(5)
  %"139" = alloca float, align 4, addrspace(5)
  %"140" = alloca float, align 4, addrspace(5)
  %"141" = alloca float, align 4, addrspace(5)
  %"142" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"98"

"98":                                             ; preds = %1
  call void @llvm.amdgcn.s.setreg(i32 6145, i32 2)
  br label %"96"

"96":                                             ; preds = %"98"
  store float %"88", ptr addrspace(5) %"139", align 4
  store float %"89", ptr addrspace(5) %"140", align 4
  %2 = load float, ptr addrspace(5) %"139", align 4
  store float %2, ptr addrspace(5) %"141", align 4
  %3 = load float, ptr addrspace(5) %"140", align 4
  store float %3, ptr addrspace(5) %"142", align 4
  %4 = load float, ptr addrspace(5) %"141", align 4
  %5 = load float, ptr addrspace(5) %"142", align 4
  %"145" = fadd float %4, %5
  store float %"145", ptr addrspace(5) %"141", align 4
  %6 = load float, ptr addrspace(5) %"141", align 4
  store float %6, ptr addrspace(5) %"138", align 4
  %7 = load float, ptr addrspace(5) %"138", align 4
  store float %7, ptr addrspace(5) %"137", align 4
  %8 = load float, ptr addrspace(5) %"137", align 4
  ret float %8
}

define hidden float @add_rp(float %"91", float %"92") #0 {
  %"150" = alloca float, align 4, addrspace(5)
  %"151" = alloca float, align 4, addrspace(5)
  %"152" = alloca float, align 4, addrspace(5)
  %"153" = alloca float, align 4, addrspace(5)
  %"154" = alloca float, align 4, addrspace(5)
  %"155" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"97"

"97":                                             ; preds = %1
  store float %"91", ptr addrspace(5) %"152", align 4
  store float %"92", ptr addrspace(5) %"153", align 4
  %2 = load float, ptr addrspace(5) %"152", align 4
  store float %2, ptr addrspace(5) %"154", align 4
  %3 = load float, ptr addrspace(5) %"153", align 4
  store float %3, ptr addrspace(5) %"155", align 4
  %4 = load float, ptr addrspace(5) %"154", align 4
  %5 = load float, ptr addrspace(5) %"155", align 4
  %"158" = fadd float %4, %5
  store float %"158", ptr addrspace(5) %"154", align 4
  %6 = load float, ptr addrspace(5) %"154", align 4
  store float %6, ptr addrspace(5) %"151", align 4
  %7 = load float, ptr addrspace(5) %"151", align 4
  store float %7, ptr addrspace(5) %"150", align 4
  %8 = load float, ptr addrspace(5) %"150", align 4
  ret float %8
}

define amdgpu_kernel void @call_rnd(ptr addrspace(4) byref(i64) %"101", ptr addrspace(4) byref(i64) %"102") #1 {
  %"103" = alloca i64, align 8, addrspace(5)
  %"104" = alloca i64, align 8, addrspace(5)
  %"105" = alloca float, align 4, addrspace(5)
  %"106" = alloca float, align 4, addrspace(5)
  %"107" = alloca float, align 4, addrspace(5)
  %"108" = alloca float, align 4, addrspace(5)
  %"109" = alloca float, align 4, addrspace(5)
  %"110" = alloca float, align 4, addrspace(5)
  %"111" = alloca float, align 4, addrspace(5)
  %"112" = alloca float, align 4, addrspace(5)
  %"113" = alloca float, align 4, addrspace(5)
  %"114" = alloca float, align 4, addrspace(5)
  %"115" = alloca float, align 4, addrspace(5)
  %"116" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"93"

"93":                                             ; preds = %1
  call void @llvm.amdgcn.s.setreg(i32 6145, i32 1)
  %2 = load i64, ptr addrspace(4) %"101", align 8
  store i64 %2, ptr addrspace(5) %"103", align 8
  %3 = load i64, ptr addrspace(4) %"102", align 8
  store i64 %3, ptr addrspace(5) %"104", align 8
  %4 = load i64, ptr addrspace(5) %"103", align 8
  %"163" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"163", align 4
  store float %5, ptr addrspace(5) %"105", align 4
  %6 = load i64, ptr addrspace(5) %"103", align 8
  %"164" = inttoptr i64 %6 to ptr
  %"68" = getelementptr inbounds i8, ptr %"164", i64 4
  %7 = load float, ptr %"68", align 4
  store float %7, ptr addrspace(5) %"106", align 4
  %8 = load i64, ptr addrspace(5) %"103", align 8
  %"165" = inttoptr i64 %8 to ptr
  %"70" = getelementptr inbounds i8, ptr %"165", i64 8
  %9 = load float, ptr %"70", align 4
  store float %9, ptr addrspace(5) %"107", align 4
  %10 = load i64, ptr addrspace(5) %"103", align 8
  %"166" = inttoptr i64 %10 to ptr
  %"72" = getelementptr inbounds i8, ptr %"166", i64 12
  %11 = load float, ptr %"72", align 4
  store float %11, ptr addrspace(5) %"108", align 4
  %12 = load float, ptr addrspace(5) %"105", align 4
  store float %12, ptr addrspace(5) %"111", align 4
  %13 = load float, ptr addrspace(5) %"106", align 4
  store float %13, ptr addrspace(5) %"112", align 4
  %14 = load float, ptr addrspace(5) %"111", align 4
  %15 = load float, ptr addrspace(5) %"112", align 4
  %"83" = call float @add_rp(float %14, float %15)
  br label %"94"

"94":                                             ; preds = %"93"
  store float %"83", ptr addrspace(5) %"113", align 4
  %16 = load float, ptr addrspace(5) %"113", align 4
  store float %16, ptr addrspace(5) %"109", align 4
  %17 = load i64, ptr addrspace(5) %"104", align 8
  %18 = load float, ptr addrspace(5) %"109", align 4
  %"167" = inttoptr i64 %17 to ptr
  store float %18, ptr %"167", align 4
  %19 = load float, ptr addrspace(5) %"107", align 4
  store float %19, ptr addrspace(5) %"114", align 4
  %20 = load float, ptr addrspace(5) %"108", align 4
  store float %20, ptr addrspace(5) %"115", align 4
  %21 = load float, ptr addrspace(5) %"114", align 4
  %22 = load float, ptr addrspace(5) %"115", align 4
  %"86" = call float @add_rm(float %21, float %22)
  br label %"95"

"95":                                             ; preds = %"94"
  store float %"86", ptr addrspace(5) %"116", align 4
  %23 = load float, ptr addrspace(5) %"116", align 4
  store float %23, ptr addrspace(5) %"110", align 4
  %24 = load i64, ptr addrspace(5) %"104", align 8
  %"168" = inttoptr i64 %24 to ptr
  %"74" = getelementptr inbounds i8, ptr %"168", i64 4
  %25 = load float, ptr addrspace(5) %"110", align 4
  store float %25, ptr %"74", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }
