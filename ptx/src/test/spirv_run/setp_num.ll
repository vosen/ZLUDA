target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @setp_num(ptr addrspace(4) byref(i64) %"115", ptr addrspace(4) byref(i64) %"116") #0 {
  %"32" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca float, align 4, addrspace(5)
  %"7" = alloca float, align 4, addrspace(5)
  %"8" = alloca float, align 4, addrspace(5)
  %"9" = alloca float, align 4, addrspace(5)
  %"10" = alloca float, align 4, addrspace(5)
  %"11" = alloca float, align 4, addrspace(5)
  %"12" = alloca float, align 4, addrspace(5)
  %"13" = alloca float, align 4, addrspace(5)
  %"14" = alloca i32, align 4, addrspace(5)
  %"15" = alloca i1, align 1, addrspace(5)
  %1 = alloca i32, align 4, addrspace(5)
  %2 = alloca i32, align 4, addrspace(5)
  %3 = alloca i32, align 4, addrspace(5)
  %4 = alloca i32, align 4, addrspace(5)
  %5 = alloca i32, align 4, addrspace(5)
  %6 = alloca i32, align 4, addrspace(5)
  %7 = alloca i32, align 4, addrspace(5)
  %8 = alloca i32, align 4, addrspace(5)
  br label %9

9:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"32", align 1
  %"33" = load i64, ptr addrspace(4) %"115", align 8
  store i64 %"33", ptr addrspace(5) %"4", align 8
  %"34" = load i64, ptr addrspace(4) %"116", align 8
  store i64 %"34", ptr addrspace(5) %"5", align 8
  %"36" = load i64, ptr addrspace(5) %"4", align 8
  %"117" = inttoptr i64 %"36" to ptr
  %"35" = load float, ptr %"117", align 4
  store float %"35", ptr addrspace(5) %"6", align 4
  %"38" = load i64, ptr addrspace(5) %"4", align 8
  %"118" = inttoptr i64 %"38" to ptr
  %"130" = getelementptr inbounds i8, ptr %"118", i64 4
  %"37" = load float, ptr %"130", align 4
  store float %"37", ptr addrspace(5) %"7", align 4
  %"40" = load i64, ptr addrspace(5) %"4", align 8
  %"119" = inttoptr i64 %"40" to ptr
  %"132" = getelementptr inbounds i8, ptr %"119", i64 8
  %"39" = load float, ptr %"132", align 4
  store float %"39", ptr addrspace(5) %"8", align 4
  %"42" = load i64, ptr addrspace(5) %"4", align 8
  %"120" = inttoptr i64 %"42" to ptr
  %"134" = getelementptr inbounds i8, ptr %"120", i64 12
  %"41" = load float, ptr %"134", align 4
  store float %"41", ptr addrspace(5) %"9", align 4
  %"44" = load i64, ptr addrspace(5) %"4", align 8
  %"121" = inttoptr i64 %"44" to ptr
  %"136" = getelementptr inbounds i8, ptr %"121", i64 16
  %"43" = load float, ptr %"136", align 4
  store float %"43", ptr addrspace(5) %"10", align 4
  %"46" = load i64, ptr addrspace(5) %"4", align 8
  %"122" = inttoptr i64 %"46" to ptr
  %"138" = getelementptr inbounds i8, ptr %"122", i64 20
  %"45" = load float, ptr %"138", align 4
  store float %"45", ptr addrspace(5) %"11", align 4
  %"48" = load i64, ptr addrspace(5) %"4", align 8
  %"123" = inttoptr i64 %"48" to ptr
  %"140" = getelementptr inbounds i8, ptr %"123", i64 24
  %"47" = load float, ptr %"140", align 4
  store float %"47", ptr addrspace(5) %"12", align 4
  %"50" = load i64, ptr addrspace(5) %"4", align 8
  %"124" = inttoptr i64 %"50" to ptr
  %"142" = getelementptr inbounds i8, ptr %"124", i64 28
  %"49" = load float, ptr %"142", align 4
  store float %"49", ptr addrspace(5) %"13", align 4
  %"52" = load float, ptr addrspace(5) %"6", align 4
  %"53" = load float, ptr addrspace(5) %"7", align 4
  %"51" = fcmp ord float %"52", %"53"
  store i1 %"51", ptr addrspace(5) %"15", align 1
  %"54" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"54", label %"16", label %"17"

"16":                                             ; preds = %9
  store i32 2, ptr addrspace(5) %1, align 4
  %"55" = load i32, ptr addrspace(5) %1, align 4
  store i32 %"55", ptr addrspace(5) %"14", align 4
  br label %"17"

"17":                                             ; preds = %"16", %9
  %"56" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"56", label %"19", label %"18"

"18":                                             ; preds = %"17"
  store i32 0, ptr addrspace(5) %2, align 4
  %"57" = load i32, ptr addrspace(5) %2, align 4
  store i32 %"57", ptr addrspace(5) %"14", align 4
  br label %"19"

"19":                                             ; preds = %"18", %"17"
  %"58" = load i64, ptr addrspace(5) %"5", align 8
  %"59" = load i32, ptr addrspace(5) %"14", align 4
  %"125" = inttoptr i64 %"58" to ptr
  store i32 %"59", ptr %"125", align 4
  %"61" = load float, ptr addrspace(5) %"8", align 4
  %"62" = load float, ptr addrspace(5) %"9", align 4
  %"60" = fcmp ord float %"61", %"62"
  store i1 %"60", ptr addrspace(5) %"15", align 1
  %"63" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"63", label %"20", label %"21"

"20":                                             ; preds = %"19"
  store i32 2, ptr addrspace(5) %3, align 4
  %"64" = load i32, ptr addrspace(5) %3, align 4
  store i32 %"64", ptr addrspace(5) %"14", align 4
  br label %"21"

"21":                                             ; preds = %"20", %"19"
  %"65" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"65", label %"23", label %"22"

"22":                                             ; preds = %"21"
  store i32 0, ptr addrspace(5) %4, align 4
  %"66" = load i32, ptr addrspace(5) %4, align 4
  store i32 %"66", ptr addrspace(5) %"14", align 4
  br label %"23"

"23":                                             ; preds = %"22", %"21"
  %"67" = load i64, ptr addrspace(5) %"5", align 8
  %"68" = load i32, ptr addrspace(5) %"14", align 4
  %"126" = inttoptr i64 %"67" to ptr
  %"144" = getelementptr inbounds i8, ptr %"126", i64 4
  store i32 %"68", ptr %"144", align 4
  %"70" = load float, ptr addrspace(5) %"10", align 4
  %"71" = load float, ptr addrspace(5) %"11", align 4
  %"69" = fcmp ord float %"70", %"71"
  store i1 %"69", ptr addrspace(5) %"15", align 1
  %"72" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"72", label %"24", label %"25"

"24":                                             ; preds = %"23"
  store i32 2, ptr addrspace(5) %5, align 4
  %"73" = load i32, ptr addrspace(5) %5, align 4
  store i32 %"73", ptr addrspace(5) %"14", align 4
  br label %"25"

"25":                                             ; preds = %"24", %"23"
  %"74" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"74", label %"27", label %"26"

"26":                                             ; preds = %"25"
  store i32 0, ptr addrspace(5) %6, align 4
  %"75" = load i32, ptr addrspace(5) %6, align 4
  store i32 %"75", ptr addrspace(5) %"14", align 4
  br label %"27"

"27":                                             ; preds = %"26", %"25"
  %"76" = load i64, ptr addrspace(5) %"5", align 8
  %"77" = load i32, ptr addrspace(5) %"14", align 4
  %"127" = inttoptr i64 %"76" to ptr
  %"146" = getelementptr inbounds i8, ptr %"127", i64 8
  store i32 %"77", ptr %"146", align 4
  %"79" = load float, ptr addrspace(5) %"12", align 4
  %"80" = load float, ptr addrspace(5) %"13", align 4
  %"78" = fcmp ord float %"79", %"80"
  store i1 %"78", ptr addrspace(5) %"15", align 1
  %"81" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"81", label %"28", label %"29"

"28":                                             ; preds = %"27"
  store i32 2, ptr addrspace(5) %7, align 4
  %"82" = load i32, ptr addrspace(5) %7, align 4
  store i32 %"82", ptr addrspace(5) %"14", align 4
  br label %"29"

"29":                                             ; preds = %"28", %"27"
  %"83" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"83", label %"31", label %"30"

"30":                                             ; preds = %"29"
  store i32 0, ptr addrspace(5) %8, align 4
  %"84" = load i32, ptr addrspace(5) %8, align 4
  store i32 %"84", ptr addrspace(5) %"14", align 4
  br label %"31"

"31":                                             ; preds = %"30", %"29"
  %"85" = load i64, ptr addrspace(5) %"5", align 8
  %"86" = load i32, ptr addrspace(5) %"14", align 4
  %"128" = inttoptr i64 %"85" to ptr
  %"148" = getelementptr inbounds i8, ptr %"128", i64 12
  store i32 %"86", ptr %"148", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
