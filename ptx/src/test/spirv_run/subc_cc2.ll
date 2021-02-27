target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @subc_cc2(ptr addrspace(4) byref(i64) %"86", ptr addrspace(4) byref(i64) %"87") #0 {
"112":
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
  %"16" = load i64, ptr addrspace(4) %"87", align 8
  store i64 %"16", ptr addrspace(5) %"5", align 8
  %0 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 0, i32 1)
  %"88" = extractvalue { i32, i1 } %0, 0
  %"18" = extractvalue { i32, i1 } %0, 1
  store i32 %"88", ptr addrspace(5) %"6", align 4
  store i1 %"18", ptr addrspace(5) %"15", align 1
  %"21" = load i1, ptr addrspace(5) %"15", align 1
  %1 = zext i1 %"21" to i32
  %2 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 0, i32 -1)
  %3 = extractvalue { i32, i1 } %2, 0
  %4 = extractvalue { i32, i1 } %2, 1
  %5 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %3, i32 %1)
  %"89" = extractvalue { i32, i1 } %5, 0
  %6 = extractvalue { i32, i1 } %5, 1
  %"20" = xor i1 %4, %6
  store i32 %"89", ptr addrspace(5) %"7", align 4
  store i1 %"20", ptr addrspace(5) %"15", align 1
  %"23" = load i1, ptr addrspace(5) %"15", align 1
  %7 = zext i1 %"23" to i32
  %"90" = sub i32 2, %7
  store i32 %"90", ptr addrspace(5) %"8", align 4
  %"25" = load i1, ptr addrspace(5) %"14", align 1
  %8 = zext i1 %"25" to i32
  %"91" = add i32 0, %8
  store i32 %"91", ptr addrspace(5) %"9", align 4
  %9 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 0, i32 1)
  %"92" = extractvalue { i32, i1 } %9, 0
  %"27" = extractvalue { i32, i1 } %9, 1
  store i32 %"92", ptr addrspace(5) %"6", align 4
  store i1 %"27", ptr addrspace(5) %"15", align 1
  %"30" = load i1, ptr addrspace(5) %"15", align 1
  %10 = zext i1 %"30" to i32
  %11 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 0, i32 0)
  %12 = extractvalue { i32, i1 } %11, 0
  %13 = extractvalue { i32, i1 } %11, 1
  %14 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %12, i32 %10)
  %"93" = extractvalue { i32, i1 } %14, 0
  %15 = extractvalue { i32, i1 } %14, 1
  %"29" = xor i1 %13, %15
  store i32 %"93", ptr addrspace(5) %"10", align 4
  store i1 %"29", ptr addrspace(5) %"15", align 1
  %"32" = load i1, ptr addrspace(5) %"15", align 1
  %16 = zext i1 %"32" to i32
  %"94" = sub i32 2, %16
  store i32 %"94", ptr addrspace(5) %"11", align 4
  %17 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 0, i32 0)
  %"95" = extractvalue { i32, i1 } %17, 0
  %"34" = extractvalue { i32, i1 } %17, 1
  store i32 %"95", ptr addrspace(5) %"6", align 4
  store i1 %"34", ptr addrspace(5) %"15", align 1
  %"37" = load i1, ptr addrspace(5) %"15", align 1
  %18 = zext i1 %"37" to i32
  %19 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 0, i32 1)
  %20 = extractvalue { i32, i1 } %19, 0
  %21 = extractvalue { i32, i1 } %19, 1
  %22 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %20, i32 %18)
  %"96" = extractvalue { i32, i1 } %22, 0
  %23 = extractvalue { i32, i1 } %22, 1
  %"36" = xor i1 %21, %23
  store i32 %"96", ptr addrspace(5) %"12", align 4
  store i1 %"36", ptr addrspace(5) %"15", align 1
  %"39" = load i1, ptr addrspace(5) %"15", align 1
  %24 = zext i1 %"39" to i32
  %"97" = sub i32 2, %24
  store i32 %"97", ptr addrspace(5) %"13", align 4
  %"40" = load i64, ptr addrspace(5) %"5", align 8
  %"41" = load i32, ptr addrspace(5) %"7", align 4
  %"98" = inttoptr i64 %"40" to ptr
  store i32 %"41", ptr %"98", align 4
  %"42" = load i64, ptr addrspace(5) %"5", align 8
  %"43" = load i32, ptr addrspace(5) %"8", align 4
  %"100" = inttoptr i64 %"42" to ptr
  %"114" = getelementptr inbounds i8, ptr %"100", i64 4
  store i32 %"43", ptr %"114", align 4
  %"44" = load i64, ptr addrspace(5) %"5", align 8
  %"45" = load i32, ptr addrspace(5) %"9", align 4
  %"102" = inttoptr i64 %"44" to ptr
  %"116" = getelementptr inbounds i8, ptr %"102", i64 8
  store i32 %"45", ptr %"116", align 4
  %"46" = load i64, ptr addrspace(5) %"5", align 8
  %"47" = load i32, ptr addrspace(5) %"10", align 4
  %"104" = inttoptr i64 %"46" to ptr
  %"118" = getelementptr inbounds i8, ptr %"104", i64 12
  store i32 %"47", ptr %"118", align 4
  %"48" = load i64, ptr addrspace(5) %"5", align 8
  %"49" = load i32, ptr addrspace(5) %"11", align 4
  %"106" = inttoptr i64 %"48" to ptr
  %"120" = getelementptr inbounds i8, ptr %"106", i64 16
  store i32 %"49", ptr %"120", align 4
  %"50" = load i64, ptr addrspace(5) %"5", align 8
  %"51" = load i32, ptr addrspace(5) %"12", align 4
  %"108" = inttoptr i64 %"50" to ptr
  %"122" = getelementptr inbounds i8, ptr %"108", i64 20
  store i32 %"51", ptr %"122", align 4
  %"52" = load i64, ptr addrspace(5) %"5", align 8
  %"53" = load i32, ptr addrspace(5) %"13", align 4
  %"110" = inttoptr i64 %"52" to ptr
  %"124" = getelementptr inbounds i8, ptr %"110", i64 24
  store i32 %"53", ptr %"124", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.usub.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
