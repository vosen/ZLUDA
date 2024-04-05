target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @addc_cc(ptr addrspace(4) byref(i64) %"53", ptr addrspace(4) byref(i64) %"54") #0 {
"68":
  %"13" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"13", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"10" = alloca i32, align 4, addrspace(5)
  %"11" = alloca i32, align 4, addrspace(5)
  %"12" = alloca i32, align 4, addrspace(5)
  %"14" = load i64, ptr addrspace(4) %"53", align 8
  store i64 %"14", ptr addrspace(5) %"4", align 8
  %"15" = load i64, ptr addrspace(4) %"54", align 8
  store i64 %"15", ptr addrspace(5) %"5", align 8
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"56" = inttoptr i64 %"17" to ptr
  %"55" = load i32, ptr %"56", align 4
  store i32 %"55", ptr addrspace(5) %"9", align 4
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %"57" = inttoptr i64 %"19" to ptr
  %"70" = getelementptr inbounds i8, ptr %"57", i64 4
  %"58" = load i32, ptr %"70", align 4
  store i32 %"58", ptr addrspace(5) %"10", align 4
  %"21" = load i64, ptr addrspace(5) %"4", align 8
  %"59" = inttoptr i64 %"21" to ptr
  %"72" = getelementptr inbounds i8, ptr %"59", i64 8
  %"20" = load i32, ptr %"72", align 4
  store i32 %"20", ptr addrspace(5) %"11", align 4
  %"23" = load i64, ptr addrspace(5) %"4", align 8
  %"60" = inttoptr i64 %"23" to ptr
  %"74" = getelementptr inbounds i8, ptr %"60", i64 12
  %"22" = load i32, ptr %"74", align 4
  store i32 %"22", ptr addrspace(5) %"12", align 4
  %"26" = load i32, ptr addrspace(5) %"9", align 4
  %"27" = load i32, ptr addrspace(5) %"10", align 4
  %0 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %"26", i32 %"27")
  %"24" = extractvalue { i32, i1 } %0, 0
  %"25" = extractvalue { i32, i1 } %0, 1
  store i32 %"24", ptr addrspace(5) %"6", align 4
  store i1 %"25", ptr addrspace(5) %"13", align 1
  %"30" = load i1, ptr addrspace(5) %"13", align 1
  %"31" = load i32, ptr addrspace(5) %"6", align 4
  %"32" = load i32, ptr addrspace(5) %"11", align 4
  %1 = zext i1 %"30" to i32
  %2 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %"31", i32 %"32")
  %3 = extractvalue { i32, i1 } %2, 0
  %4 = extractvalue { i32, i1 } %2, 1
  %5 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %3, i32 %1)
  %"28" = extractvalue { i32, i1 } %5, 0
  %6 = extractvalue { i32, i1 } %5, 1
  %"29" = xor i1 %4, %6
  store i32 %"28", ptr addrspace(5) %"7", align 4
  store i1 %"29", ptr addrspace(5) %"13", align 1
  %"34" = load i1, ptr addrspace(5) %"13", align 1
  %"35" = load i32, ptr addrspace(5) %"7", align 4
  %"36" = load i32, ptr addrspace(5) %"12", align 4
  %7 = zext i1 %"34" to i32
  %8 = add i32 %"35", %"36"
  %"33" = add i32 %8, %7
  store i32 %"33", ptr addrspace(5) %"8", align 4
  %"37" = load i64, ptr addrspace(5) %"5", align 8
  %"38" = load i32, ptr addrspace(5) %"6", align 4
  %"65" = inttoptr i64 %"37" to ptr
  store i32 %"38", ptr %"65", align 4
  %"39" = load i64, ptr addrspace(5) %"5", align 8
  %"40" = load i32, ptr addrspace(5) %"7", align 4
  %"66" = inttoptr i64 %"39" to ptr
  %"76" = getelementptr inbounds i8, ptr %"66", i64 4
  store i32 %"40", ptr %"76", align 4
  %"41" = load i64, ptr addrspace(5) %"5", align 8
  %"42" = load i32, ptr addrspace(5) %"8", align 4
  %"67" = inttoptr i64 %"41" to ptr
  %"78" = getelementptr inbounds i8, ptr %"67", i64 8
  store i32 %"42", ptr %"78", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
