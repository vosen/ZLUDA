target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @set_f16x2(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
"59":
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
  %"10" = alloca <2 x half>, align 4, addrspace(5)
  %"13" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"13", ptr addrspace(5) %"4", align 8
  %"14" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"14", ptr addrspace(5) %"5", align 8
  %"16" = load i64, ptr addrspace(5) %"4", align 8
  %"44" = inttoptr i64 %"16" to ptr
  %"43" = load i32, ptr %"44", align 4
  store i32 %"43", ptr addrspace(5) %"6", align 4
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"45" = inttoptr i64 %"18" to ptr
  %"61" = getelementptr inbounds i8, ptr %"45", i64 4
  %"46" = load i32, ptr %"61", align 4
  store i32 %"46", ptr addrspace(5) %"7", align 4
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"47" = inttoptr i64 %"20" to ptr
  %"63" = getelementptr inbounds i8, ptr %"47", i64 8
  %"48" = load i32, ptr %"63", align 4
  store i32 %"48", ptr addrspace(5) %"8", align 4
  %"22" = load i64, ptr addrspace(5) %"4", align 8
  %"49" = inttoptr i64 %"22" to ptr
  %"65" = getelementptr inbounds i8, ptr %"49", i64 12
  %"50" = load i32, ptr %"65", align 4
  store i32 %"50", ptr addrspace(5) %"9", align 4
  %"24" = load i32, ptr addrspace(5) %"6", align 4
  %"25" = load i32, ptr addrspace(5) %"7", align 4
  %"52" = bitcast i32 %"24" to <2 x half>
  %"53" = bitcast i32 %"25" to <2 x half>
  %0 = fcmp ugt <2 x half> %"52", %"53"
  %1 = sext <2 x i1> %0 to <2 x i16>
  %"51" = bitcast <2 x i16> %1 to i32
  store i32 %"51", ptr addrspace(5) %"6", align 4
  %"27" = load i32, ptr addrspace(5) %"8", align 4
  %"28" = load i32, ptr addrspace(5) %"9", align 4
  %"55" = bitcast i32 %"27" to <2 x half>
  %"56" = bitcast i32 %"28" to <2 x half>
  %2 = fcmp oeq <2 x half> %"55", %"56"
  %"54" = uitofp <2 x i1> %2 to <2 x half>
  %"26" = bitcast <2 x half> %"54" to i32
  store i32 %"26", ptr addrspace(5) %"8", align 4
  %"29" = load i64, ptr addrspace(5) %"5", align 8
  %"30" = load i32, ptr addrspace(5) %"6", align 4
  %"57" = inttoptr i64 %"29" to ptr
  store i32 %"30", ptr %"57", align 4
  %"31" = load i64, ptr addrspace(5) %"5", align 8
  %"32" = load i32, ptr addrspace(5) %"8", align 4
  %"58" = inttoptr i64 %"31" to ptr
  %"67" = getelementptr inbounds i8, ptr %"58", i64 4
  store i32 %"32", ptr %"67", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
