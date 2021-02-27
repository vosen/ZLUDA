target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @madc_cc2(ptr addrspace(4) byref(i64) %"52", ptr addrspace(4) byref(i64) %"53") #0 {
"66":
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
  %"13" = load i64, ptr addrspace(4) %"53", align 8
  store i64 %"13", ptr addrspace(5) %"5", align 8
  %0 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 0, i32 -1)
  %"14" = extractvalue { i32, i1 } %0, 0
  %"15" = extractvalue { i32, i1 } %0, 1
  store i32 %"14", ptr addrspace(5) %"6", align 4
  store i1 %"15", ptr addrspace(5) %"11", align 1
  %"18" = load i1, ptr addrspace(5) %"11", align 1
  %1 = zext i1 %"18" to i32
  %2 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 1, i32 -1)
  %3 = extractvalue { i32, i1 } %2, 0
  %4 = extractvalue { i32, i1 } %2, 1
  %5 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %3, i32 %1)
  %"54" = extractvalue { i32, i1 } %5, 0
  %6 = extractvalue { i32, i1 } %5, 1
  %"17" = xor i1 %4, %6
  store i32 %"54", ptr addrspace(5) %"7", align 4
  store i1 %"17", ptr addrspace(5) %"11", align 1
  %"20" = load i1, ptr addrspace(5) %"11", align 1
  %7 = zext i1 %"20" to i32
  %"55" = add i32 0, %7
  store i32 %"55", ptr addrspace(5) %"8", align 4
  %"22" = load i1, ptr addrspace(5) %"11", align 1
  %8 = zext i1 %"22" to i32
  %"56" = add i32 0, %8
  store i32 %"56", ptr addrspace(5) %"9", align 4
  %"24" = load i1, ptr addrspace(5) %"12", align 1
  %9 = zext i1 %"24" to i32
  %"57" = sub i32 2, %9
  store i32 %"57", ptr addrspace(5) %"10", align 4
  %"25" = load i64, ptr addrspace(5) %"5", align 8
  %"26" = load i32, ptr addrspace(5) %"7", align 4
  %"58" = inttoptr i64 %"25" to ptr
  store i32 %"26", ptr %"58", align 4
  %"27" = load i64, ptr addrspace(5) %"5", align 8
  %"28" = load i32, ptr addrspace(5) %"8", align 4
  %"60" = inttoptr i64 %"27" to ptr
  %"68" = getelementptr inbounds i8, ptr %"60", i64 4
  store i32 %"28", ptr %"68", align 4
  %"29" = load i64, ptr addrspace(5) %"5", align 8
  %"30" = load i32, ptr addrspace(5) %"9", align 4
  %"62" = inttoptr i64 %"29" to ptr
  %"70" = getelementptr inbounds i8, ptr %"62", i64 8
  store i32 %"30", ptr %"70", align 4
  %"31" = load i64, ptr addrspace(5) %"5", align 8
  %"32" = load i32, ptr addrspace(5) %"10", align 4
  %"64" = inttoptr i64 %"31" to ptr
  %"72" = getelementptr inbounds i8, ptr %"64", i64 12
  store i32 %"32", ptr %"72", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
