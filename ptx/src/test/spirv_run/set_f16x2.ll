target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @set_f16x2(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
"58":
  %"11" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"11", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"10" = alloca <2 x half>, align 4, addrspace(5)
  %"12" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"12", ptr addrspace(5) %"4", align 8
  %"13" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"13", ptr addrspace(5) %"5", align 8
  %"15" = load i64, ptr addrspace(5) %"4", align 8
  %"43" = inttoptr i64 %"15" to ptr
  %"42" = load i32, ptr %"43", align 4
  store i32 %"42", ptr addrspace(5) %"6", align 4
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"44" = inttoptr i64 %"17" to ptr
  %"60" = getelementptr inbounds i8, ptr %"44", i64 4
  %"45" = load i32, ptr %"60", align 4
  store i32 %"45", ptr addrspace(5) %"7", align 4
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %"46" = inttoptr i64 %"19" to ptr
  %"62" = getelementptr inbounds i8, ptr %"46", i64 8
  %"47" = load i32, ptr %"62", align 4
  store i32 %"47", ptr addrspace(5) %"8", align 4
  %"21" = load i64, ptr addrspace(5) %"4", align 8
  %"48" = inttoptr i64 %"21" to ptr
  %"64" = getelementptr inbounds i8, ptr %"48", i64 12
  %"49" = load i32, ptr %"64", align 4
  store i32 %"49", ptr addrspace(5) %"9", align 4
  %"23" = load i32, ptr addrspace(5) %"6", align 4
  %"24" = load i32, ptr addrspace(5) %"7", align 4
  %"51" = bitcast i32 %"23" to <2 x half>
  %"52" = bitcast i32 %"24" to <2 x half>
  %0 = fcmp ugt <2 x half> %"51", %"52"
  %1 = sext <2 x i1> %0 to <2 x i16>
  %"50" = bitcast <2 x i16> %1 to i32
  store i32 %"50", ptr addrspace(5) %"6", align 4
  %"26" = load i32, ptr addrspace(5) %"8", align 4
  %"27" = load i32, ptr addrspace(5) %"9", align 4
  %"54" = bitcast i32 %"26" to <2 x half>
  %"55" = bitcast i32 %"27" to <2 x half>
  %2 = fcmp oeq <2 x half> %"54", %"55"
  %"53" = uitofp <2 x i1> %2 to <2 x half>
  %"25" = bitcast <2 x half> %"53" to i32
  store i32 %"25", ptr addrspace(5) %"8", align 4
  %"28" = load i64, ptr addrspace(5) %"5", align 8
  %"29" = load i32, ptr addrspace(5) %"6", align 4
  %"56" = inttoptr i64 %"28" to ptr
  store i32 %"29", ptr %"56", align 4
  %"30" = load i64, ptr addrspace(5) %"5", align 8
  %"31" = load i32, ptr addrspace(5) %"8", align 4
  %"57" = inttoptr i64 %"30" to ptr
  %"66" = getelementptr inbounds i8, ptr %"57", i64 4
  store i32 %"31", ptr %"66", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
