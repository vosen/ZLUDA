target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @fma(ptr addrspace(4) byref(i64) %"28", ptr addrspace(4) byref(i64) %"29") #0 {
  %"9" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca float, align 4, addrspace(5)
  %"7" = alloca float, align 4, addrspace(5)
  %"8" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"9", align 1
  %"10" = load i64, ptr addrspace(4) %"28", align 8
  store i64 %"10", ptr addrspace(5) %"4", align 8
  %"11" = load i64, ptr addrspace(4) %"29", align 8
  store i64 %"11", ptr addrspace(5) %"5", align 8
  %"13" = load i64, ptr addrspace(5) %"4", align 8
  %"30" = inttoptr i64 %"13" to ptr
  %"12" = load float, ptr %"30", align 4
  store float %"12", ptr addrspace(5) %"6", align 4
  %"15" = load i64, ptr addrspace(5) %"4", align 8
  %"31" = inttoptr i64 %"15" to ptr
  %"35" = getelementptr inbounds i8, ptr %"31", i64 4
  %"14" = load float, ptr %"35", align 4
  store float %"14", ptr addrspace(5) %"7", align 4
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"32" = inttoptr i64 %"17" to ptr
  %"37" = getelementptr inbounds i8, ptr %"32", i64 8
  %"16" = load float, ptr %"37", align 4
  store float %"16", ptr addrspace(5) %"8", align 4
  %"19" = load float, ptr addrspace(5) %"6", align 4
  %"20" = load float, ptr addrspace(5) %"7", align 4
  %"21" = load float, ptr addrspace(5) %"8", align 4
  %"18" = call float @llvm.fma.f32(float %"19", float %"20", float %"21")
  store float %"18", ptr addrspace(5) %"6", align 4
  %"22" = load i64, ptr addrspace(5) %"5", align 8
  %"23" = load float, ptr addrspace(5) %"6", align 4
  %"33" = inttoptr i64 %"22" to ptr
  store float %"23", ptr %"33", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.fma.f32(float, float, float) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
