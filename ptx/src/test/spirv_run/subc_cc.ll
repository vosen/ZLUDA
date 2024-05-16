target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @subc_cc(ptr addrspace(4) byref(i64) %"57", ptr addrspace(4) byref(i64) %"58") #0 {
  %"13" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"10" = alloca i32, align 4, addrspace(5)
  %"11" = alloca i32, align 4, addrspace(5)
  %"12" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"13", align 1
  %"18" = load i64, ptr addrspace(4) %"57", align 8
  store i64 %"18", ptr addrspace(5) %"4", align 8
  %"19" = load i64, ptr addrspace(4) %"58", align 8
  store i64 %"19", ptr addrspace(5) %"5", align 8
  %"21" = load i64, ptr addrspace(5) %"4", align 8
  %"60" = inttoptr i64 %"21" to ptr
  %"59" = load i32, ptr %"60", align 4
  store i32 %"59", ptr addrspace(5) %"9", align 4
  %"23" = load i64, ptr addrspace(5) %"4", align 8
  %"61" = inttoptr i64 %"23" to ptr
  %"73" = getelementptr inbounds i8, ptr %"61", i64 4
  %"62" = load i32, ptr %"73", align 4
  store i32 %"62", ptr addrspace(5) %"10", align 4
  %"25" = load i64, ptr addrspace(5) %"4", align 8
  %"63" = inttoptr i64 %"25" to ptr
  %"75" = getelementptr inbounds i8, ptr %"63", i64 8
  %"24" = load i32, ptr %"75", align 4
  store i32 %"24", ptr addrspace(5) %"11", align 4
  %"27" = load i64, ptr addrspace(5) %"4", align 8
  %"64" = inttoptr i64 %"27" to ptr
  %"77" = getelementptr inbounds i8, ptr %"64", i64 12
  %"26" = load i32, ptr %"77", align 4
  store i32 %"26", ptr addrspace(5) %"12", align 4
  %"29" = load i32, ptr addrspace(5) %"9", align 4
  %"30" = load i32, ptr addrspace(5) %"10", align 4
  %2 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %"29", i32 %"30")
  %"28" = extractvalue { i32, i1 } %2, 0
  %"14" = extractvalue { i32, i1 } %2, 1
  store i32 %"28", ptr addrspace(5) %"6", align 4
  %"31" = xor i1 %"14", true
  store i1 %"31", ptr addrspace(5) %"13", align 1
  %"32" = load i1, ptr addrspace(5) %"13", align 1
  %"15" = xor i1 %"32", true
  %"34" = load i32, ptr addrspace(5) %"6", align 4
  %"35" = load i32, ptr addrspace(5) %"11", align 4
  %3 = zext i1 %"15" to i32
  %4 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %"34", i32 %"35")
  %5 = extractvalue { i32, i1 } %4, 0
  %6 = extractvalue { i32, i1 } %4, 1
  %7 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %5, i32 %3)
  %"33" = extractvalue { i32, i1 } %7, 0
  %8 = extractvalue { i32, i1 } %7, 1
  %"16" = xor i1 %6, %8
  store i32 %"33", ptr addrspace(5) %"7", align 4
  %"36" = xor i1 %"16", true
  store i1 %"36", ptr addrspace(5) %"13", align 1
  %"37" = load i1, ptr addrspace(5) %"13", align 1
  %"17" = xor i1 %"37", true
  %"39" = load i32, ptr addrspace(5) %"7", align 4
  %"40" = load i32, ptr addrspace(5) %"12", align 4
  %9 = zext i1 %"17" to i32
  %10 = sub i32 %"39", %"40"
  %"38" = sub i32 %10, %9
  store i32 %"38", ptr addrspace(5) %"8", align 4
  %"41" = load i64, ptr addrspace(5) %"5", align 8
  %"42" = load i32, ptr addrspace(5) %"6", align 4
  %"69" = inttoptr i64 %"41" to ptr
  store i32 %"42", ptr %"69", align 4
  %"43" = load i64, ptr addrspace(5) %"5", align 8
  %"44" = load i32, ptr addrspace(5) %"7", align 4
  %"70" = inttoptr i64 %"43" to ptr
  %"79" = getelementptr inbounds i8, ptr %"70", i64 4
  store i32 %"44", ptr %"79", align 4
  %"45" = load i64, ptr addrspace(5) %"5", align 8
  %"46" = load i32, ptr addrspace(5) %"8", align 4
  %"71" = inttoptr i64 %"45" to ptr
  %"81" = getelementptr inbounds i8, ptr %"71", i64 8
  store i32 %"46", ptr %"81", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.usub.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
