target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @setp_num(ptr addrspace(4) byref(i64) %"116", ptr addrspace(4) byref(i64) %"117") #0 {
"130":
  %"32" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"32", align 1
  %"33" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"33", align 1
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
  %"34" = load i64, ptr addrspace(4) %"116", align 8
  store i64 %"34", ptr addrspace(5) %"4", align 8
  %"35" = load i64, ptr addrspace(4) %"117", align 8
  store i64 %"35", ptr addrspace(5) %"5", align 8
  %"37" = load i64, ptr addrspace(5) %"4", align 8
  %"118" = inttoptr i64 %"37" to ptr
  %"36" = load float, ptr %"118", align 4
  store float %"36", ptr addrspace(5) %"6", align 4
  %"39" = load i64, ptr addrspace(5) %"4", align 8
  %"119" = inttoptr i64 %"39" to ptr
  %"132" = getelementptr inbounds i8, ptr %"119", i64 4
  %"38" = load float, ptr %"132", align 4
  store float %"38", ptr addrspace(5) %"7", align 4
  %"41" = load i64, ptr addrspace(5) %"4", align 8
  %"120" = inttoptr i64 %"41" to ptr
  %"134" = getelementptr inbounds i8, ptr %"120", i64 8
  %"40" = load float, ptr %"134", align 4
  store float %"40", ptr addrspace(5) %"8", align 4
  %"43" = load i64, ptr addrspace(5) %"4", align 8
  %"121" = inttoptr i64 %"43" to ptr
  %"136" = getelementptr inbounds i8, ptr %"121", i64 12
  %"42" = load float, ptr %"136", align 4
  store float %"42", ptr addrspace(5) %"9", align 4
  %"45" = load i64, ptr addrspace(5) %"4", align 8
  %"122" = inttoptr i64 %"45" to ptr
  %"138" = getelementptr inbounds i8, ptr %"122", i64 16
  %"44" = load float, ptr %"138", align 4
  store float %"44", ptr addrspace(5) %"10", align 4
  %"47" = load i64, ptr addrspace(5) %"4", align 8
  %"123" = inttoptr i64 %"47" to ptr
  %"140" = getelementptr inbounds i8, ptr %"123", i64 20
  %"46" = load float, ptr %"140", align 4
  store float %"46", ptr addrspace(5) %"11", align 4
  %"49" = load i64, ptr addrspace(5) %"4", align 8
  %"124" = inttoptr i64 %"49" to ptr
  %"142" = getelementptr inbounds i8, ptr %"124", i64 24
  %"48" = load float, ptr %"142", align 4
  store float %"48", ptr addrspace(5) %"12", align 4
  %"51" = load i64, ptr addrspace(5) %"4", align 8
  %"125" = inttoptr i64 %"51" to ptr
  %"144" = getelementptr inbounds i8, ptr %"125", i64 28
  %"50" = load float, ptr %"144", align 4
  store float %"50", ptr addrspace(5) %"13", align 4
  %"53" = load float, ptr addrspace(5) %"6", align 4
  %"54" = load float, ptr addrspace(5) %"7", align 4
  %"52" = fcmp ord float %"53", %"54"
  store i1 %"52", ptr addrspace(5) %"15", align 1
  %"55" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"55", label %"16", label %"17"

"16":                                             ; preds = %"130"
  %0 = alloca i32, align 4, addrspace(5)
  store i32 2, ptr addrspace(5) %0, align 4
  %"56" = load i32, ptr addrspace(5) %0, align 4
  store i32 %"56", ptr addrspace(5) %"14", align 4
  br label %"17"

"17":                                             ; preds = %"16", %"130"
  %"57" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"57", label %"19", label %"18"

"18":                                             ; preds = %"17"
  %1 = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %1, align 4
  %"58" = load i32, ptr addrspace(5) %1, align 4
  store i32 %"58", ptr addrspace(5) %"14", align 4
  br label %"19"

"19":                                             ; preds = %"18", %"17"
  %"59" = load i64, ptr addrspace(5) %"5", align 8
  %"60" = load i32, ptr addrspace(5) %"14", align 4
  %"126" = inttoptr i64 %"59" to ptr
  store i32 %"60", ptr %"126", align 4
  %"62" = load float, ptr addrspace(5) %"8", align 4
  %"63" = load float, ptr addrspace(5) %"9", align 4
  %"61" = fcmp ord float %"62", %"63"
  store i1 %"61", ptr addrspace(5) %"15", align 1
  %"64" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"64", label %"20", label %"21"

"20":                                             ; preds = %"19"
  %2 = alloca i32, align 4, addrspace(5)
  store i32 2, ptr addrspace(5) %2, align 4
  %"65" = load i32, ptr addrspace(5) %2, align 4
  store i32 %"65", ptr addrspace(5) %"14", align 4
  br label %"21"

"21":                                             ; preds = %"20", %"19"
  %"66" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"66", label %"23", label %"22"

"22":                                             ; preds = %"21"
  %3 = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %3, align 4
  %"67" = load i32, ptr addrspace(5) %3, align 4
  store i32 %"67", ptr addrspace(5) %"14", align 4
  br label %"23"

"23":                                             ; preds = %"22", %"21"
  %"68" = load i64, ptr addrspace(5) %"5", align 8
  %"69" = load i32, ptr addrspace(5) %"14", align 4
  %"127" = inttoptr i64 %"68" to ptr
  %"146" = getelementptr inbounds i8, ptr %"127", i64 4
  store i32 %"69", ptr %"146", align 4
  %"71" = load float, ptr addrspace(5) %"10", align 4
  %"72" = load float, ptr addrspace(5) %"11", align 4
  %"70" = fcmp ord float %"71", %"72"
  store i1 %"70", ptr addrspace(5) %"15", align 1
  %"73" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"73", label %"24", label %"25"

"24":                                             ; preds = %"23"
  %4 = alloca i32, align 4, addrspace(5)
  store i32 2, ptr addrspace(5) %4, align 4
  %"74" = load i32, ptr addrspace(5) %4, align 4
  store i32 %"74", ptr addrspace(5) %"14", align 4
  br label %"25"

"25":                                             ; preds = %"24", %"23"
  %"75" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"75", label %"27", label %"26"

"26":                                             ; preds = %"25"
  %5 = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %5, align 4
  %"76" = load i32, ptr addrspace(5) %5, align 4
  store i32 %"76", ptr addrspace(5) %"14", align 4
  br label %"27"

"27":                                             ; preds = %"26", %"25"
  %"77" = load i64, ptr addrspace(5) %"5", align 8
  %"78" = load i32, ptr addrspace(5) %"14", align 4
  %"128" = inttoptr i64 %"77" to ptr
  %"148" = getelementptr inbounds i8, ptr %"128", i64 8
  store i32 %"78", ptr %"148", align 4
  %"80" = load float, ptr addrspace(5) %"12", align 4
  %"81" = load float, ptr addrspace(5) %"13", align 4
  %"79" = fcmp ord float %"80", %"81"
  store i1 %"79", ptr addrspace(5) %"15", align 1
  %"82" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"82", label %"28", label %"29"

"28":                                             ; preds = %"27"
  %6 = alloca i32, align 4, addrspace(5)
  store i32 2, ptr addrspace(5) %6, align 4
  %"83" = load i32, ptr addrspace(5) %6, align 4
  store i32 %"83", ptr addrspace(5) %"14", align 4
  br label %"29"

"29":                                             ; preds = %"28", %"27"
  %"84" = load i1, ptr addrspace(5) %"15", align 1
  br i1 %"84", label %"31", label %"30"

"30":                                             ; preds = %"29"
  %7 = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %7, align 4
  %"85" = load i32, ptr addrspace(5) %7, align 4
  store i32 %"85", ptr addrspace(5) %"14", align 4
  br label %"31"

"31":                                             ; preds = %"30", %"29"
  %"86" = load i64, ptr addrspace(5) %"5", align 8
  %"87" = load i32, ptr addrspace(5) %"14", align 4
  %"129" = inttoptr i64 %"86" to ptr
  %"150" = getelementptr inbounds i8, ptr %"129", i64 12
  store i32 %"87", ptr %"150", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
