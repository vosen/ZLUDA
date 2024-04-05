target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @mad_hi_cc(ptr addrspace(4) byref(i64) %"60", ptr addrspace(4) byref(i64) %"61") #0 {
"77":
  %"14" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"14", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"10" = alloca i32, align 4, addrspace(5)
  %"11" = alloca i32, align 4, addrspace(5)
  %"12" = alloca i32, align 4, addrspace(5)
  %"13" = alloca i32, align 4, addrspace(5)
  %"15" = load i64, ptr addrspace(4) %"60", align 8
  store i64 %"15", ptr addrspace(5) %"4", align 8
  %"16" = load i64, ptr addrspace(4) %"61", align 8
  store i64 %"16", ptr addrspace(5) %"5", align 8
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"63" = inttoptr i64 %"18" to ptr
  %"62" = load i32, ptr %"63", align 4
  store i32 %"62", ptr addrspace(5) %"8", align 4
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"64" = inttoptr i64 %"20" to ptr
  %"79" = getelementptr inbounds i8, ptr %"64", i64 4
  %"65" = load i32, ptr %"79", align 4
  store i32 %"65", ptr addrspace(5) %"9", align 4
  %"22" = load i64, ptr addrspace(5) %"4", align 8
  %"66" = inttoptr i64 %"22" to ptr
  %"81" = getelementptr inbounds i8, ptr %"66", i64 8
  %"21" = load i32, ptr %"81", align 4
  store i32 %"21", ptr addrspace(5) %"10", align 4
  %"25" = load i32, ptr addrspace(5) %"8", align 4
  %"26" = load i32, ptr addrspace(5) %"9", align 4
  %"27" = load i32, ptr addrspace(5) %"10", align 4
  %0 = sext i32 %"25" to i64
  %1 = sext i32 %"26" to i64
  %2 = mul nsw i64 %0, %1
  %3 = lshr i64 %2, 32
  %4 = trunc i64 %3 to i32
  %5 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %4, i32 %"27")
  %"23" = extractvalue { i32, i1 } %5, 0
  %"24" = extractvalue { i32, i1 } %5, 1
  store i32 %"23", ptr addrspace(5) %"7", align 4
  store i1 %"24", ptr addrspace(5) %"14", align 1
  %6 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 1, i32 -2)
  %"28" = extractvalue { i32, i1 } %6, 0
  %"29" = extractvalue { i32, i1 } %6, 1
  store i32 %"28", ptr addrspace(5) %"6", align 4
  store i1 %"29", ptr addrspace(5) %"14", align 1
  %"31" = load i1, ptr addrspace(5) %"14", align 1
  %7 = zext i1 %"31" to i32
  %"70" = add i32 0, %7
  store i32 %"70", ptr addrspace(5) %"12", align 4
  %8 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 1, i32 -1)
  %"32" = extractvalue { i32, i1 } %8, 0
  %"33" = extractvalue { i32, i1 } %8, 1
  store i32 %"32", ptr addrspace(5) %"6", align 4
  store i1 %"33", ptr addrspace(5) %"14", align 1
  %"35" = load i1, ptr addrspace(5) %"14", align 1
  %9 = zext i1 %"35" to i32
  %"71" = add i32 0, %9
  store i32 %"71", ptr addrspace(5) %"13", align 4
  %"36" = load i64, ptr addrspace(5) %"5", align 8
  %"37" = load i32, ptr addrspace(5) %"7", align 4
  %"72" = inttoptr i64 %"36" to ptr
  store i32 %"37", ptr %"72", align 4
  %"38" = load i64, ptr addrspace(5) %"5", align 8
  %"39" = load i32, ptr addrspace(5) %"12", align 4
  %"73" = inttoptr i64 %"38" to ptr
  %"83" = getelementptr inbounds i8, ptr %"73", i64 4
  store i32 %"39", ptr %"83", align 4
  %"40" = load i64, ptr addrspace(5) %"5", align 8
  %"41" = load i32, ptr addrspace(5) %"13", align 4
  %"75" = inttoptr i64 %"40" to ptr
  %"85" = getelementptr inbounds i8, ptr %"75", i64 8
  store i32 %"41", ptr %"85", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
