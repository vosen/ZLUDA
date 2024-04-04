target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @mad_hi_cc(ptr addrspace(4) byref(i64) %"76", ptr addrspace(4) byref(i64) %"77") #0 {
"99":
  %"15" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"15", align 1
  %"16" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"16", align 1
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
  %"14" = alloca i32, align 4, addrspace(5)
  %"17" = load i64, ptr addrspace(4) %"76", align 8
  store i64 %"17", ptr addrspace(5) %"4", align 8
  %"18" = load i64, ptr addrspace(4) %"77", align 8
  store i64 %"18", ptr addrspace(5) %"5", align 8
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"79" = inttoptr i64 %"20" to ptr
  %"78" = load i32, ptr %"79", align 4
  store i32 %"78", ptr addrspace(5) %"8", align 4
  %"22" = load i64, ptr addrspace(5) %"4", align 8
  %"80" = inttoptr i64 %"22" to ptr
  %"101" = getelementptr inbounds i8, ptr %"80", i64 4
  %"81" = load i32, ptr %"101", align 4
  store i32 %"81", ptr addrspace(5) %"9", align 4
  %"24" = load i64, ptr addrspace(5) %"4", align 8
  %"82" = inttoptr i64 %"24" to ptr
  %"103" = getelementptr inbounds i8, ptr %"82", i64 8
  %"23" = load i32, ptr %"103", align 4
  store i32 %"23", ptr addrspace(5) %"10", align 4
  %"27" = load i32, ptr addrspace(5) %"8", align 4
  %"28" = load i32, ptr addrspace(5) %"9", align 4
  %"29" = load i32, ptr addrspace(5) %"10", align 4
  %0 = sext i32 %"27" to i64
  %1 = sext i32 %"28" to i64
  %2 = mul nsw i64 %0, %1
  %3 = lshr i64 %2, 32
  %4 = trunc i64 %3 to i32
  %5 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %4, i32 %"29")
  %"25" = extractvalue { i32, i1 } %5, 0
  %"26" = extractvalue { i32, i1 } %5, 1
  store i32 %"25", ptr addrspace(5) %"7", align 4
  store i1 %"26", ptr addrspace(5) %"15", align 1
  %"30" = load i64, ptr addrspace(5) %"5", align 8
  %"31" = load i32, ptr addrspace(5) %"7", align 4
  %"86" = inttoptr i64 %"30" to ptr
  store i32 %"31", ptr %"86", align 4
  %6 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 0, i32 -1)
  %"32" = extractvalue { i32, i1 } %6, 0
  %"33" = extractvalue { i32, i1 } %6, 1
  store i32 %"32", ptr addrspace(5) %"6", align 4
  store i1 %"33", ptr addrspace(5) %"15", align 1
  %"36" = load i1, ptr addrspace(5) %"15", align 1
  %7 = zext i1 %"36" to i32
  %8 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 1, i32 -1)
  %9 = extractvalue { i32, i1 } %8, 0
  %10 = extractvalue { i32, i1 } %8, 1
  %11 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %9, i32 %7)
  %"87" = extractvalue { i32, i1 } %11, 0
  %12 = extractvalue { i32, i1 } %11, 1
  %"35" = xor i1 %10, %12
  store i32 %"87", ptr addrspace(5) %"11", align 4
  store i1 %"35", ptr addrspace(5) %"15", align 1
  %"38" = load i1, ptr addrspace(5) %"15", align 1
  %13 = zext i1 %"38" to i32
  %"88" = add i32 0, %13
  store i32 %"88", ptr addrspace(5) %"12", align 4
  %"40" = load i1, ptr addrspace(5) %"15", align 1
  %14 = zext i1 %"40" to i32
  %"89" = add i32 0, %14
  store i32 %"89", ptr addrspace(5) %"13", align 4
  %"42" = load i1, ptr addrspace(5) %"16", align 1
  %15 = zext i1 %"42" to i32
  %"90" = sub i32 2, %15
  store i32 %"90", ptr addrspace(5) %"14", align 4
  %"43" = load i64, ptr addrspace(5) %"5", align 8
  %"44" = load i32, ptr addrspace(5) %"11", align 4
  %"91" = inttoptr i64 %"43" to ptr
  %"105" = getelementptr inbounds i8, ptr %"91", i64 4
  store i32 %"44", ptr %"105", align 4
  %"45" = load i64, ptr addrspace(5) %"5", align 8
  %"46" = load i32, ptr addrspace(5) %"12", align 4
  %"93" = inttoptr i64 %"45" to ptr
  %"107" = getelementptr inbounds i8, ptr %"93", i64 8
  store i32 %"46", ptr %"107", align 4
  %"47" = load i64, ptr addrspace(5) %"5", align 8
  %"48" = load i32, ptr addrspace(5) %"13", align 4
  %"95" = inttoptr i64 %"47" to ptr
  %"109" = getelementptr inbounds i8, ptr %"95", i64 12
  store i32 %"48", ptr %"109", align 4
  %"49" = load i64, ptr addrspace(5) %"5", align 8
  %"50" = load i32, ptr addrspace(5) %"14", align 4
  %"97" = inttoptr i64 %"49" to ptr
  %"111" = getelementptr inbounds i8, ptr %"97", i64 16
  store i32 %"50", ptr %"111", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
