target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @addc_cc(ptr addrspace(4) byref(i64) %"54", ptr addrspace(4) byref(i64) %"55") #0 {
"69":
  %"13" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"13", align 1
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
  %"15" = load i64, ptr addrspace(4) %"54", align 8
  store i64 %"15", ptr addrspace(5) %"4", align 8
  %"16" = load i64, ptr addrspace(4) %"55", align 8
  store i64 %"16", ptr addrspace(5) %"5", align 8
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"57" = inttoptr i64 %"18" to ptr
  %"56" = load i32, ptr %"57", align 4
  store i32 %"56", ptr addrspace(5) %"9", align 4
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"58" = inttoptr i64 %"20" to ptr
  %"71" = getelementptr inbounds i8, ptr %"58", i64 4
  %"59" = load i32, ptr %"71", align 4
  store i32 %"59", ptr addrspace(5) %"10", align 4
  %"22" = load i64, ptr addrspace(5) %"4", align 8
  %"60" = inttoptr i64 %"22" to ptr
  %"73" = getelementptr inbounds i8, ptr %"60", i64 8
  %"21" = load i32, ptr %"73", align 4
  store i32 %"21", ptr addrspace(5) %"11", align 4
  %"24" = load i64, ptr addrspace(5) %"4", align 8
  %"61" = inttoptr i64 %"24" to ptr
  %"75" = getelementptr inbounds i8, ptr %"61", i64 12
  %"23" = load i32, ptr %"75", align 4
  store i32 %"23", ptr addrspace(5) %"12", align 4
  %"27" = load i32, ptr addrspace(5) %"9", align 4
  %"28" = load i32, ptr addrspace(5) %"10", align 4
  %0 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %"27", i32 %"28")
  %"25" = extractvalue { i32, i1 } %0, 0
  %"26" = extractvalue { i32, i1 } %0, 1
  store i32 %"25", ptr addrspace(5) %"6", align 4
  store i1 %"26", ptr addrspace(5) %"13", align 1
  %"31" = load i1, ptr addrspace(5) %"13", align 1
  %"32" = load i32, ptr addrspace(5) %"6", align 4
  %"33" = load i32, ptr addrspace(5) %"11", align 4
  %1 = zext i1 %"31" to i32
  %2 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %"32", i32 %"33")
  %3 = extractvalue { i32, i1 } %2, 0
  %4 = extractvalue { i32, i1 } %2, 1
  %5 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %3, i32 %1)
  %"29" = extractvalue { i32, i1 } %5, 0
  %6 = extractvalue { i32, i1 } %5, 1
  %"30" = xor i1 %4, %6
  store i32 %"29", ptr addrspace(5) %"7", align 4
  store i1 %"30", ptr addrspace(5) %"13", align 1
  %"35" = load i1, ptr addrspace(5) %"13", align 1
  %"36" = load i32, ptr addrspace(5) %"7", align 4
  %"37" = load i32, ptr addrspace(5) %"12", align 4
  %7 = zext i1 %"35" to i32
  %8 = add i32 %"36", %"37"
  %"34" = add i32 %8, %7
  store i32 %"34", ptr addrspace(5) %"8", align 4
  %"38" = load i64, ptr addrspace(5) %"5", align 8
  %"39" = load i32, ptr addrspace(5) %"6", align 4
  %"66" = inttoptr i64 %"38" to ptr
  store i32 %"39", ptr %"66", align 4
  %"40" = load i64, ptr addrspace(5) %"5", align 8
  %"41" = load i32, ptr addrspace(5) %"7", align 4
  %"67" = inttoptr i64 %"40" to ptr
  %"77" = getelementptr inbounds i8, ptr %"67", i64 4
  store i32 %"41", ptr %"77", align 4
  %"42" = load i64, ptr addrspace(5) %"5", align 8
  %"43" = load i32, ptr addrspace(5) %"8", align 4
  %"68" = inttoptr i64 %"42" to ptr
  %"79" = getelementptr inbounds i8, ptr %"68", i64 8
  store i32 %"43", ptr %"79", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
