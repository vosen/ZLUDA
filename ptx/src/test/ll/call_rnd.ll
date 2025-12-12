define hidden float @add_rm(float %"85", float %"86") #0 {
  %"134" = alloca float, align 4, addrspace(5)
  %"135" = alloca float, align 4, addrspace(5)
  %"136" = alloca float, align 4, addrspace(5)
  %"137" = alloca float, align 4, addrspace(5)
  %"138" = alloca float, align 4, addrspace(5)
  %"139" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"95"

"95":                                             ; preds = %1
  call void @llvm.amdgcn.s.setreg(i32 6145, i32 2)
  br label %"93"

"93":                                             ; preds = %"95"
  store float %"85", ptr addrspace(5) %"136", align 4
  store float %"86", ptr addrspace(5) %"137", align 4
  %2 = load float, ptr addrspace(5) %"136", align 4
  store float %2, ptr addrspace(5) %"138", align 4
  %3 = load float, ptr addrspace(5) %"137", align 4
  store float %3, ptr addrspace(5) %"139", align 4
  %4 = load float, ptr addrspace(5) %"138", align 4
  %5 = load float, ptr addrspace(5) %"139", align 4
  %"142" = fadd float %4, %5
  store float %"142", ptr addrspace(5) %"138", align 4
  %6 = load float, ptr addrspace(5) %"138", align 4
  store float %6, ptr addrspace(5) %"135", align 4
  %7 = load float, ptr addrspace(5) %"135", align 4
  store float %7, ptr addrspace(5) %"134", align 4
  %8 = load float, ptr addrspace(5) %"134", align 4
  ret float %8
}

define hidden float @add_rp(float %"88", float %"89") #0 {
  %"147" = alloca float, align 4, addrspace(5)
  %"148" = alloca float, align 4, addrspace(5)
  %"149" = alloca float, align 4, addrspace(5)
  %"150" = alloca float, align 4, addrspace(5)
  %"151" = alloca float, align 4, addrspace(5)
  %"152" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"94"

"94":                                             ; preds = %1
  store float %"88", ptr addrspace(5) %"149", align 4
  store float %"89", ptr addrspace(5) %"150", align 4
  %2 = load float, ptr addrspace(5) %"149", align 4
  store float %2, ptr addrspace(5) %"151", align 4
  %3 = load float, ptr addrspace(5) %"150", align 4
  store float %3, ptr addrspace(5) %"152", align 4
  %4 = load float, ptr addrspace(5) %"151", align 4
  %5 = load float, ptr addrspace(5) %"152", align 4
  %"155" = fadd float %4, %5
  store float %"155", ptr addrspace(5) %"151", align 4
  %6 = load float, ptr addrspace(5) %"151", align 4
  store float %6, ptr addrspace(5) %"148", align 4
  %7 = load float, ptr addrspace(5) %"148", align 4
  store float %7, ptr addrspace(5) %"147", align 4
  %8 = load float, ptr addrspace(5) %"147", align 4
  ret float %8
}

define amdgpu_kernel void @call_rnd(ptr addrspace(4) byref(i64) %"98", ptr addrspace(4) byref(i64) %"99") #1 {
  %"100" = alloca i64, align 8, addrspace(5)
  %"101" = alloca i64, align 8, addrspace(5)
  %"102" = alloca float, align 4, addrspace(5)
  %"103" = alloca float, align 4, addrspace(5)
  %"104" = alloca float, align 4, addrspace(5)
  %"105" = alloca float, align 4, addrspace(5)
  %"106" = alloca float, align 4, addrspace(5)
  %"107" = alloca float, align 4, addrspace(5)
  %"108" = alloca float, align 4, addrspace(5)
  %"109" = alloca float, align 4, addrspace(5)
  %"110" = alloca float, align 4, addrspace(5)
  %"111" = alloca float, align 4, addrspace(5)
  %"112" = alloca float, align 4, addrspace(5)
  %"113" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"90"

"90":                                             ; preds = %1
  call void @llvm.amdgcn.s.setreg(i32 6145, i32 1)
  %2 = load i64, ptr addrspace(4) %"98", align 8
  store i64 %2, ptr addrspace(5) %"100", align 8
  %3 = load i64, ptr addrspace(4) %"99", align 8
  store i64 %3, ptr addrspace(5) %"101", align 8
  %4 = load i64, ptr addrspace(5) %"100", align 8
  %"160" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"160", align 4
  store float %5, ptr addrspace(5) %"102", align 4
  %6 = load i64, ptr addrspace(5) %"100", align 8
  %"161" = inttoptr i64 %6 to ptr
  %"65" = getelementptr inbounds i8, ptr %"161", i64 4
  %7 = load float, ptr %"65", align 4
  store float %7, ptr addrspace(5) %"103", align 4
  %8 = load i64, ptr addrspace(5) %"100", align 8
  %"162" = inttoptr i64 %8 to ptr
  %"67" = getelementptr inbounds i8, ptr %"162", i64 8
  %9 = load float, ptr %"67", align 4
  store float %9, ptr addrspace(5) %"104", align 4
  %10 = load i64, ptr addrspace(5) %"100", align 8
  %"163" = inttoptr i64 %10 to ptr
  %"69" = getelementptr inbounds i8, ptr %"163", i64 12
  %11 = load float, ptr %"69", align 4
  store float %11, ptr addrspace(5) %"105", align 4
  %12 = load float, ptr addrspace(5) %"102", align 4
  store float %12, ptr addrspace(5) %"108", align 4
  %13 = load float, ptr addrspace(5) %"103", align 4
  store float %13, ptr addrspace(5) %"109", align 4
  %14 = load float, ptr addrspace(5) %"108", align 4
  %15 = load float, ptr addrspace(5) %"109", align 4
  %"80" = call float @add_rp(float %14, float %15)
  br label %"91"

"91":                                             ; preds = %"90"
  store float %"80", ptr addrspace(5) %"110", align 4
  %16 = load float, ptr addrspace(5) %"110", align 4
  store float %16, ptr addrspace(5) %"106", align 4
  %17 = load i64, ptr addrspace(5) %"101", align 8
  %18 = load float, ptr addrspace(5) %"106", align 4
  %"164" = inttoptr i64 %17 to ptr
  store float %18, ptr %"164", align 4
  %19 = load float, ptr addrspace(5) %"104", align 4
  store float %19, ptr addrspace(5) %"111", align 4
  %20 = load float, ptr addrspace(5) %"105", align 4
  store float %20, ptr addrspace(5) %"112", align 4
  %21 = load float, ptr addrspace(5) %"111", align 4
  %22 = load float, ptr addrspace(5) %"112", align 4
  %"83" = call float @add_rm(float %21, float %22)
  br label %"92"

"92":                                             ; preds = %"91"
  store float %"83", ptr addrspace(5) %"113", align 4
  %23 = load float, ptr addrspace(5) %"113", align 4
  store float %23, ptr addrspace(5) %"107", align 4
  %24 = load i64, ptr addrspace(5) %"101", align 8
  %"165" = inttoptr i64 %24 to ptr
  %"71" = getelementptr inbounds i8, ptr %"165", i64 4
  %25 = load float, ptr addrspace(5) %"107", align 4
  store float %25, ptr %"71", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }