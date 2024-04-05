target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @cvt_rni(ptr addrspace(4) byref(i64) %"27", ptr addrspace(4) byref(i64) %"28") #0 {
"33":
  %"8" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"8", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca float, align 4, addrspace(5)
  %"7" = alloca float, align 4, addrspace(5)
  %"9" = load i64, ptr addrspace(4) %"27", align 8
  store i64 %"9", ptr addrspace(5) %"4", align 8
  %"10" = load i64, ptr addrspace(4) %"28", align 8
  store i64 %"10", ptr addrspace(5) %"5", align 8
  %"12" = load i64, ptr addrspace(5) %"4", align 8
  %"29" = inttoptr i64 %"12" to ptr
  %"11" = load float, ptr %"29", align 4
  store float %"11", ptr addrspace(5) %"6", align 4
  %"14" = load i64, ptr addrspace(5) %"4", align 8
  %"30" = inttoptr i64 %"14" to ptr
  %"35" = getelementptr inbounds i8, ptr %"30", i64 4
  %"13" = load float, ptr %"35", align 4
  store float %"13", ptr addrspace(5) %"7", align 4
  %"16" = load float, ptr addrspace(5) %"6", align 4
  %"15" = call float @llvm.rint.f32(float %"16")
  store float %"15", ptr addrspace(5) %"6", align 4
  %"18" = load float, ptr addrspace(5) %"7", align 4
  %"17" = call float @llvm.rint.f32(float %"18")
  store float %"17", ptr addrspace(5) %"7", align 4
  %"19" = load i64, ptr addrspace(5) %"5", align 8
  %"20" = load float, ptr addrspace(5) %"6", align 4
  %"31" = inttoptr i64 %"19" to ptr
  store float %"20", ptr %"31", align 4
  %"21" = load i64, ptr addrspace(5) %"5", align 8
  %"22" = load float, ptr addrspace(5) %"7", align 4
  %"32" = inttoptr i64 %"21" to ptr
  %"37" = getelementptr inbounds i8, ptr %"32", i64 4
  store float %"22", ptr %"37", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.rint.f32(float) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
