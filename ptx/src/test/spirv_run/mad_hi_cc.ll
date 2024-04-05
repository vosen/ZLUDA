target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @mad_hi_cc(ptr addrspace(4) byref(i64) %"61", ptr addrspace(4) byref(i64) %"62") #0 {
"78":
  %"14" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"14", align 1
  %"15" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"15", align 1
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
  %"16" = load i64, ptr addrspace(4) %"61", align 8
  store i64 %"16", ptr addrspace(5) %"4", align 8
  %"17" = load i64, ptr addrspace(4) %"62", align 8
  store i64 %"17", ptr addrspace(5) %"5", align 8
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %"64" = inttoptr i64 %"19" to ptr
  %"63" = load i32, ptr %"64", align 4
  store i32 %"63", ptr addrspace(5) %"8", align 4
  %"21" = load i64, ptr addrspace(5) %"4", align 8
  %"65" = inttoptr i64 %"21" to ptr
  %"80" = getelementptr inbounds i8, ptr %"65", i64 4
  %"66" = load i32, ptr %"80", align 4
  store i32 %"66", ptr addrspace(5) %"9", align 4
  %"23" = load i64, ptr addrspace(5) %"4", align 8
  %"67" = inttoptr i64 %"23" to ptr
  %"82" = getelementptr inbounds i8, ptr %"67", i64 8
  %"22" = load i32, ptr %"82", align 4
  store i32 %"22", ptr addrspace(5) %"10", align 4
  %"26" = load i32, ptr addrspace(5) %"8", align 4
  %"27" = load i32, ptr addrspace(5) %"9", align 4
  %"28" = load i32, ptr addrspace(5) %"10", align 4
  %0 = sext i32 %"26" to i64
  %1 = sext i32 %"27" to i64
  %2 = mul nsw i64 %0, %1
  %3 = lshr i64 %2, 32
  %4 = trunc i64 %3 to i32
  %5 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %4, i32 %"28")
  %"24" = extractvalue { i32, i1 } %5, 0
  %"25" = extractvalue { i32, i1 } %5, 1
  store i32 %"24", ptr addrspace(5) %"7", align 4
  store i1 %"25", ptr addrspace(5) %"14", align 1
  %6 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 1, i32 -2)
  %"29" = extractvalue { i32, i1 } %6, 0
  %"30" = extractvalue { i32, i1 } %6, 1
  store i32 %"29", ptr addrspace(5) %"6", align 4
  store i1 %"30", ptr addrspace(5) %"14", align 1
  %"32" = load i1, ptr addrspace(5) %"14", align 1
  %7 = zext i1 %"32" to i32
  %"71" = add i32 0, %7
  store i32 %"71", ptr addrspace(5) %"12", align 4
  %8 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 1, i32 -1)
  %"33" = extractvalue { i32, i1 } %8, 0
  %"34" = extractvalue { i32, i1 } %8, 1
  store i32 %"33", ptr addrspace(5) %"6", align 4
  store i1 %"34", ptr addrspace(5) %"14", align 1
  %"36" = load i1, ptr addrspace(5) %"14", align 1
  %9 = zext i1 %"36" to i32
  %"72" = add i32 0, %9
  store i32 %"72", ptr addrspace(5) %"13", align 4
  %"37" = load i64, ptr addrspace(5) %"5", align 8
  %"38" = load i32, ptr addrspace(5) %"7", align 4
  %"73" = inttoptr i64 %"37" to ptr
  store i32 %"38", ptr %"73", align 4
  %"39" = load i64, ptr addrspace(5) %"5", align 8
  %"40" = load i32, ptr addrspace(5) %"12", align 4
  %"74" = inttoptr i64 %"39" to ptr
  %"84" = getelementptr inbounds i8, ptr %"74", i64 4
  store i32 %"40", ptr %"84", align 4
  %"41" = load i64, ptr addrspace(5) %"5", align 8
  %"42" = load i32, ptr addrspace(5) %"13", align 4
  %"76" = inttoptr i64 %"41" to ptr
  %"86" = getelementptr inbounds i8, ptr %"76", i64 8
  store i32 %"42", ptr %"86", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
