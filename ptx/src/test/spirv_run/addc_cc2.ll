target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @addc_cc2(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
"50":
  %"9" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"9", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"10" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"10", ptr addrspace(5) %"5", align 8
  %0 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 -1, i32 -1)
  %"41" = extractvalue { i32, i1 } %0, 0
  %"12" = extractvalue { i32, i1 } %0, 1
  store i32 %"41", ptr addrspace(5) %"6", align 4
  store i1 %"12", ptr addrspace(5) %"9", align 1
  %"15" = load i1, ptr addrspace(5) %"9", align 1
  %1 = zext i1 %"15" to i32
  %2 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 -4, i32 -4)
  %3 = extractvalue { i32, i1 } %2, 0
  %4 = extractvalue { i32, i1 } %2, 1
  %5 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %3, i32 %1)
  %"42" = extractvalue { i32, i1 } %5, 0
  %6 = extractvalue { i32, i1 } %5, 1
  %"14" = xor i1 %4, %6
  store i32 %"42", ptr addrspace(5) %"6", align 4
  store i1 %"14", ptr addrspace(5) %"9", align 1
  %"17" = load i1, ptr addrspace(5) %"9", align 1
  %7 = zext i1 %"17" to i32
  %"43" = add i32 0, %7
  store i32 %"43", ptr addrspace(5) %"7", align 4
  %"20" = load i1, ptr addrspace(5) %"9", align 1
  %8 = zext i1 %"20" to i32
  %9 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 0, i32 -1)
  %10 = extractvalue { i32, i1 } %9, 0
  %11 = extractvalue { i32, i1 } %9, 1
  %12 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %10, i32 %8)
  %"44" = extractvalue { i32, i1 } %12, 0
  %13 = extractvalue { i32, i1 } %12, 1
  %"19" = xor i1 %11, %13
  store i32 %"44", ptr addrspace(5) %"6", align 4
  store i1 %"19", ptr addrspace(5) %"9", align 1
  %"22" = load i1, ptr addrspace(5) %"9", align 1
  %14 = zext i1 %"22" to i32
  %"45" = add i32 0, %14
  store i32 %"45", ptr addrspace(5) %"8", align 4
  %"23" = load i64, ptr addrspace(5) %"5", align 8
  %"24" = load i32, ptr addrspace(5) %"7", align 4
  %"46" = inttoptr i64 %"23" to ptr
  store i32 %"24", ptr %"46", align 4
  %"25" = load i64, ptr addrspace(5) %"5", align 8
  %"26" = load i32, ptr addrspace(5) %"8", align 4
  %"48" = inttoptr i64 %"25" to ptr
  %"52" = getelementptr inbounds i8, ptr %"48", i64 4
  store i32 %"26", ptr %"52", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
