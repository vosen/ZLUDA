target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @madc_cc(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
"54":
  %"11" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"11", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"10" = alloca i32, align 4, addrspace(5)
  %"12" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"12", ptr addrspace(5) %"4", align 8
  %"13" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"13", ptr addrspace(5) %"5", align 8
  %"15" = load i64, ptr addrspace(5) %"4", align 8
  %"43" = inttoptr i64 %"15" to ptr
  %"42" = load i32, ptr %"43", align 4
  store i32 %"42", ptr addrspace(5) %"8", align 4
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"44" = inttoptr i64 %"17" to ptr
  %"56" = getelementptr inbounds i8, ptr %"44", i64 4
  %"45" = load i32, ptr %"56", align 4
  store i32 %"45", ptr addrspace(5) %"9", align 4
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %"46" = inttoptr i64 %"19" to ptr
  %"58" = getelementptr inbounds i8, ptr %"46", i64 8
  %"18" = load i32, ptr %"58", align 4
  store i32 %"18", ptr addrspace(5) %"10", align 4
  %"22" = load i32, ptr addrspace(5) %"8", align 4
  %"23" = load i32, ptr addrspace(5) %"9", align 4
  %"24" = load i32, ptr addrspace(5) %"10", align 4
  %0 = mul i32 %"22", %"23"
  %1 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %0, i32 %"24")
  %"20" = extractvalue { i32, i1 } %1, 0
  %"21" = extractvalue { i32, i1 } %1, 1
  store i32 %"20", ptr addrspace(5) %"6", align 4
  store i1 %"21", ptr addrspace(5) %"11", align 1
  %"26" = load i1, ptr addrspace(5) %"11", align 1
  %"27" = load i32, ptr addrspace(5) %"8", align 4
  %"28" = load i32, ptr addrspace(5) %"9", align 4
  %2 = sext i32 %"27" to i64
  %3 = sext i32 %"28" to i64
  %4 = mul nsw i64 %2, %3
  %5 = lshr i64 %4, 32
  %6 = trunc i64 %5 to i32
  %7 = zext i1 %"26" to i32
  %8 = add i32 %6, 3
  %"25" = add i32 %8, %7
  store i32 %"25", ptr addrspace(5) %"7", align 4
  %"29" = load i64, ptr addrspace(5) %"5", align 8
  %"30" = load i32, ptr addrspace(5) %"6", align 4
  %"52" = inttoptr i64 %"29" to ptr
  store i32 %"30", ptr %"52", align 4
  %"31" = load i64, ptr addrspace(5) %"5", align 8
  %"32" = load i32, ptr addrspace(5) %"7", align 4
  %"53" = inttoptr i64 %"31" to ptr
  %"60" = getelementptr inbounds i8, ptr %"53", i64 4
  store i32 %"32", ptr %"60", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
