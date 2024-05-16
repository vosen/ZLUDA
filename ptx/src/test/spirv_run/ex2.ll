target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @ex2(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #0 {
  %"7" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"7", align 1
  %"8" = load i64, ptr addrspace(4) %"46", align 8
  store i64 %"8", ptr addrspace(5) %"4", align 8
  %"9" = load i64, ptr addrspace(4) %"47", align 8
  store i64 %"9", ptr addrspace(5) %"5", align 8
  %"11" = load i64, ptr addrspace(5) %"4", align 8
  %"48" = inttoptr i64 %"11" to ptr
  %"10" = load float, ptr %"48", align 4
  store float %"10", ptr addrspace(5) %"6", align 4
  %"13" = load float, ptr addrspace(5) %"6", align 4
  %"12" = call afn float @llvm.exp2.f32(float %"13")
  store float %"12", ptr addrspace(5) %"6", align 4
  %"14" = load i64, ptr addrspace(5) %"5", align 8
  %"15" = load float, ptr addrspace(5) %"6", align 4
  %"49" = inttoptr i64 %"14" to ptr
  store float %"15", ptr %"49", align 4
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"50" = inttoptr i64 %"17" to ptr
  %"57" = getelementptr inbounds i8, ptr %"50", i64 4
  %"16" = load float, ptr %"57", align 4
  store float %"16", ptr addrspace(5) %"6", align 4
  %"19" = load float, ptr addrspace(5) %"6", align 4
  %"18" = call afn float @llvm.exp2.f32(float %"19")
  store float %"18", ptr addrspace(5) %"6", align 4
  %"20" = load i64, ptr addrspace(5) %"5", align 8
  %"21" = load float, ptr addrspace(5) %"6", align 4
  %"51" = inttoptr i64 %"20" to ptr
  %"59" = getelementptr inbounds i8, ptr %"51", i64 4
  store float %"21", ptr %"59", align 4
  %"23" = load i64, ptr addrspace(5) %"4", align 8
  %"52" = inttoptr i64 %"23" to ptr
  %"61" = getelementptr inbounds i8, ptr %"52", i64 8
  %"22" = load float, ptr %"61", align 4
  store float %"22", ptr addrspace(5) %"6", align 4
  %"25" = load float, ptr addrspace(5) %"6", align 4
  %"24" = call afn float @llvm.exp2.f32(float %"25")
  store float %"24", ptr addrspace(5) %"6", align 4
  %"26" = load i64, ptr addrspace(5) %"5", align 8
  %"27" = load float, ptr addrspace(5) %"6", align 4
  %"53" = inttoptr i64 %"26" to ptr
  %"63" = getelementptr inbounds i8, ptr %"53", i64 8
  store float %"27", ptr %"63", align 4
  %"29" = load i64, ptr addrspace(5) %"4", align 8
  %"54" = inttoptr i64 %"29" to ptr
  %"65" = getelementptr inbounds i8, ptr %"54", i64 12
  %"28" = load float, ptr %"65", align 4
  store float %"28", ptr addrspace(5) %"6", align 4
  %"31" = load float, ptr addrspace(5) %"6", align 4
  %"30" = call afn float @llvm.exp2.f32(float %"31")
  store float %"30", ptr addrspace(5) %"6", align 4
  %"32" = load i64, ptr addrspace(5) %"5", align 8
  %"33" = load float, ptr addrspace(5) %"6", align 4
  %"55" = inttoptr i64 %"32" to ptr
  %"67" = getelementptr inbounds i8, ptr %"55", i64 12
  store float %"33", ptr %"67", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare float @llvm.exp2.f32(float) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
