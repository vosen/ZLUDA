target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @carry_mixed(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
"44":
  %"9" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"9", align 1
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"11" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"11", ptr addrspace(5) %"5", align 8
  %0 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 0, i32 1)
  %"36" = extractvalue { i32, i1 } %0, 0
  %"13" = extractvalue { i32, i1 } %0, 1
  store i32 %"36", ptr addrspace(5) %"6", align 4
  store i1 %"13", ptr addrspace(5) %"10", align 1
  %"15" = load i1, ptr addrspace(5) %"10", align 1
  %1 = zext i1 %"15" to i32
  %"37" = sub i32 2, %1
  store i32 %"37", ptr addrspace(5) %"7", align 4
  %2 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 0, i32 1)
  %"38" = extractvalue { i32, i1 } %2, 0
  %"17" = extractvalue { i32, i1 } %2, 1
  store i32 %"38", ptr addrspace(5) %"6", align 4
  store i1 %"17", ptr addrspace(5) %"10", align 1
  %"19" = load i1, ptr addrspace(5) %"9", align 1
  %3 = zext i1 %"19" to i32
  %"39" = add i32 1, %3
  store i32 %"39", ptr addrspace(5) %"8", align 4
  %"20" = load i64, ptr addrspace(5) %"5", align 8
  %"21" = load i32, ptr addrspace(5) %"7", align 4
  %"40" = inttoptr i64 %"20" to ptr
  store i32 %"21", ptr %"40", align 4
  %"22" = load i64, ptr addrspace(5) %"5", align 8
  %"23" = load i32, ptr addrspace(5) %"8", align 4
  %"42" = inttoptr i64 %"22" to ptr
  %"46" = getelementptr inbounds i8, ptr %"42", i64 4
  store i32 %"23", ptr %"46", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.usub.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
