target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @madc_cc(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
"55":
  %"11" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"11", align 1
  %"12" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"12", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"10" = alloca i32, align 4, addrspace(5)
  %"13" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"13", ptr addrspace(5) %"4", align 8
  %"14" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"14", ptr addrspace(5) %"5", align 8
  %"16" = load i64, ptr addrspace(5) %"4", align 8
  %"44" = inttoptr i64 %"16" to ptr
  %"43" = load i32, ptr %"44", align 4
  store i32 %"43", ptr addrspace(5) %"8", align 4
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"45" = inttoptr i64 %"18" to ptr
  %"57" = getelementptr inbounds i8, ptr %"45", i64 4
  %"46" = load i32, ptr %"57", align 4
  store i32 %"46", ptr addrspace(5) %"9", align 4
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"47" = inttoptr i64 %"20" to ptr
  %"59" = getelementptr inbounds i8, ptr %"47", i64 8
  %"19" = load i32, ptr %"59", align 4
  store i32 %"19", ptr addrspace(5) %"10", align 4
  %"23" = load i32, ptr addrspace(5) %"8", align 4
  %"24" = load i32, ptr addrspace(5) %"9", align 4
  %"25" = load i32, ptr addrspace(5) %"10", align 4
  %0 = mul i32 %"23", %"24"
  %1 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %0, i32 %"25")
  %"21" = extractvalue { i32, i1 } %1, 0
  %"22" = extractvalue { i32, i1 } %1, 1
  store i32 %"21", ptr addrspace(5) %"6", align 4
  store i1 %"22", ptr addrspace(5) %"11", align 1
  %"27" = load i1, ptr addrspace(5) %"11", align 1
  %"28" = load i32, ptr addrspace(5) %"8", align 4
  %"29" = load i32, ptr addrspace(5) %"9", align 4
  %2 = sext i32 %"28" to i64
  %3 = sext i32 %"29" to i64
  %4 = mul nsw i64 %2, %3
  %5 = lshr i64 %4, 32
  %6 = trunc i64 %5 to i32
  %7 = zext i1 %"27" to i32
  %8 = add i32 %6, 3
  %"26" = add i32 %8, %7
  store i32 %"26", ptr addrspace(5) %"7", align 4
  %"30" = load i64, ptr addrspace(5) %"5", align 8
  %"31" = load i32, ptr addrspace(5) %"6", align 4
  %"53" = inttoptr i64 %"30" to ptr
  store i32 %"31", ptr %"53", align 4
  %"32" = load i64, ptr addrspace(5) %"5", align 8
  %"33" = load i32, ptr addrspace(5) %"7", align 4
  %"54" = inttoptr i64 %"32" to ptr
  %"61" = getelementptr inbounds i8, ptr %"54", i64 4
  store i32 %"33", ptr %"61", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
