define amdgpu_kernel void @setp_num(ptr addrspace(4) byref(i64) %"86", ptr addrspace(4) byref(i64) %"87") #0 {
  %"88" = alloca i64, align 8, addrspace(5)
  %"89" = alloca i64, align 8, addrspace(5)
  %"90" = alloca float, align 4, addrspace(5)
  %"91" = alloca float, align 4, addrspace(5)
  %"92" = alloca float, align 4, addrspace(5)
  %"93" = alloca float, align 4, addrspace(5)
  %"94" = alloca float, align 4, addrspace(5)
  %"95" = alloca float, align 4, addrspace(5)
  %"96" = alloca float, align 4, addrspace(5)
  %"97" = alloca float, align 4, addrspace(5)
  %"98" = alloca i32, align 4, addrspace(5)
  %"99" = alloca i1, align 1, addrspace(5)
  %"104" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"104", align 4
  %"108" = alloca i64, align 8, addrspace(5)
  store i64 8, ptr addrspace(5) %"108", align 4
  %"112" = alloca i64, align 8, addrspace(5)
  store i64 12, ptr addrspace(5) %"112", align 4
  %"116" = alloca i64, align 8, addrspace(5)
  store i64 16, ptr addrspace(5) %"116", align 4
  %"120" = alloca i64, align 8, addrspace(5)
  store i64 20, ptr addrspace(5) %"120", align 4
  %"124" = alloca i64, align 8, addrspace(5)
  store i64 24, ptr addrspace(5) %"124", align 4
  %"128" = alloca i64, align 8, addrspace(5)
  store i64 28, ptr addrspace(5) %"128", align 4
  %"136" = alloca i32, align 4, addrspace(5)
  store i32 2, ptr addrspace(5) %"136", align 4
  %"140" = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %"140", align 4
  %"149" = alloca i32, align 4, addrspace(5)
  store i32 2, ptr addrspace(5) %"149", align 4
  %"153" = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %"153", align 4
  %"156" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"156", align 4
  %"164" = alloca i32, align 4, addrspace(5)
  store i32 2, ptr addrspace(5) %"164", align 4
  %"168" = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %"168", align 4
  %"171" = alloca i64, align 8, addrspace(5)
  store i64 8, ptr addrspace(5) %"171", align 4
  %"179" = alloca i32, align 4, addrspace(5)
  store i32 2, ptr addrspace(5) %"179", align 4
  %"183" = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %"183", align 4
  %"186" = alloca i64, align 8, addrspace(5)
  store i64 12, ptr addrspace(5) %"186", align 4
  br label %1

1:                                                ; preds = %0
  br label %"85"

"85":                                             ; preds = %1
  %"100" = load i64, ptr addrspace(4) %"86", align 8
  store i64 %"100", ptr addrspace(5) %"88", align 8
  %"101" = load i64, ptr addrspace(4) %"87", align 8
  store i64 %"101", ptr addrspace(5) %"89", align 8
  %"103" = load i64, ptr addrspace(5) %"88", align 8
  %"190" = inttoptr i64 %"103" to ptr
  %"102" = load float, ptr %"190", align 4
  store float %"102", ptr addrspace(5) %"90", align 4
  %"105" = load i64, ptr addrspace(5) %"88", align 8
  %"106" = load i64, ptr addrspace(5) %"104", align 8
  %"191" = inttoptr i64 %"105" to ptr
  %"58" = getelementptr inbounds i8, ptr %"191", i64 %"106"
  %"107" = load float, ptr %"58", align 4
  store float %"107", ptr addrspace(5) %"91", align 4
  %"109" = load i64, ptr addrspace(5) %"88", align 8
  %"110" = load i64, ptr addrspace(5) %"108", align 8
  %"192" = inttoptr i64 %"109" to ptr
  %"60" = getelementptr inbounds i8, ptr %"192", i64 %"110"
  %"111" = load float, ptr %"60", align 4
  store float %"111", ptr addrspace(5) %"92", align 4
  %"113" = load i64, ptr addrspace(5) %"88", align 8
  %"114" = load i64, ptr addrspace(5) %"112", align 8
  %"193" = inttoptr i64 %"113" to ptr
  %"62" = getelementptr inbounds i8, ptr %"193", i64 %"114"
  %"115" = load float, ptr %"62", align 4
  store float %"115", ptr addrspace(5) %"93", align 4
  %"117" = load i64, ptr addrspace(5) %"88", align 8
  %"118" = load i64, ptr addrspace(5) %"116", align 8
  %"194" = inttoptr i64 %"117" to ptr
  %"64" = getelementptr inbounds i8, ptr %"194", i64 %"118"
  %"119" = load float, ptr %"64", align 4
  store float %"119", ptr addrspace(5) %"94", align 4
  %"121" = load i64, ptr addrspace(5) %"88", align 8
  %"122" = load i64, ptr addrspace(5) %"120", align 8
  %"195" = inttoptr i64 %"121" to ptr
  %"66" = getelementptr inbounds i8, ptr %"195", i64 %"122"
  %"123" = load float, ptr %"66", align 4
  store float %"123", ptr addrspace(5) %"95", align 4
  %"125" = load i64, ptr addrspace(5) %"88", align 8
  %"126" = load i64, ptr addrspace(5) %"124", align 8
  %"196" = inttoptr i64 %"125" to ptr
  %"68" = getelementptr inbounds i8, ptr %"196", i64 %"126"
  %"127" = load float, ptr %"68", align 4
  store float %"127", ptr addrspace(5) %"96", align 4
  %"129" = load i64, ptr addrspace(5) %"88", align 8
  %"130" = load i64, ptr addrspace(5) %"128", align 8
  %"197" = inttoptr i64 %"129" to ptr
  %"70" = getelementptr inbounds i8, ptr %"197", i64 %"130"
  %"131" = load float, ptr %"70", align 4
  store float %"131", ptr addrspace(5) %"97", align 4
  %"133" = load float, ptr addrspace(5) %"90", align 4
  %"134" = load float, ptr addrspace(5) %"91", align 4
  %2 = fcmp ord float %"133", %"134"
  store i1 %2, ptr addrspace(5) %"99", align 1
  %"135" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"135", label %"23", label %"24"

"23":                                             ; preds = %"85"
  %"138" = load i32, ptr addrspace(5) %"136", align 4
  store i32 %"138", ptr addrspace(5) %"98", align 4
  br label %"24"

"24":                                             ; preds = %"23", %"85"
  %"139" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"139", label %"26", label %"25"

"25":                                             ; preds = %"24"
  %"142" = load i32, ptr addrspace(5) %"140", align 4
  store i32 %"142", ptr addrspace(5) %"98", align 4
  br label %"26"

"26":                                             ; preds = %"25", %"24"
  %"143" = load i64, ptr addrspace(5) %"89", align 8
  %"144" = load i32, ptr addrspace(5) %"98", align 4
  %"198" = inttoptr i64 %"143" to ptr
  store i32 %"144", ptr %"198", align 4
  %"146" = load float, ptr addrspace(5) %"92", align 4
  %"147" = load float, ptr addrspace(5) %"93", align 4
  %3 = fcmp ord float %"146", %"147"
  store i1 %3, ptr addrspace(5) %"99", align 1
  %"148" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"148", label %"27", label %"28"

"27":                                             ; preds = %"26"
  %"151" = load i32, ptr addrspace(5) %"149", align 4
  store i32 %"151", ptr addrspace(5) %"98", align 4
  br label %"28"

"28":                                             ; preds = %"27", %"26"
  %"152" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"152", label %"30", label %"29"

"29":                                             ; preds = %"28"
  %"155" = load i32, ptr addrspace(5) %"153", align 4
  store i32 %"155", ptr addrspace(5) %"98", align 4
  br label %"30"

"30":                                             ; preds = %"29", %"28"
  %"157" = load i64, ptr addrspace(5) %"89", align 8
  %"158" = load i64, ptr addrspace(5) %"156", align 8
  %"199" = inttoptr i64 %"157" to ptr
  %"76" = getelementptr inbounds i8, ptr %"199", i64 %"158"
  %"159" = load i32, ptr addrspace(5) %"98", align 4
  store i32 %"159", ptr %"76", align 4
  %"161" = load float, ptr addrspace(5) %"94", align 4
  %"162" = load float, ptr addrspace(5) %"95", align 4
  %4 = fcmp ord float %"161", %"162"
  store i1 %4, ptr addrspace(5) %"99", align 1
  %"163" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"163", label %"31", label %"32"

"31":                                             ; preds = %"30"
  %"166" = load i32, ptr addrspace(5) %"164", align 4
  store i32 %"166", ptr addrspace(5) %"98", align 4
  br label %"32"

"32":                                             ; preds = %"31", %"30"
  %"167" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"167", label %"34", label %"33"

"33":                                             ; preds = %"32"
  %"170" = load i32, ptr addrspace(5) %"168", align 4
  store i32 %"170", ptr addrspace(5) %"98", align 4
  br label %"34"

"34":                                             ; preds = %"33", %"32"
  %"172" = load i64, ptr addrspace(5) %"89", align 8
  %"173" = load i64, ptr addrspace(5) %"171", align 8
  %"200" = inttoptr i64 %"172" to ptr
  %"80" = getelementptr inbounds i8, ptr %"200", i64 %"173"
  %"174" = load i32, ptr addrspace(5) %"98", align 4
  store i32 %"174", ptr %"80", align 4
  %"176" = load float, ptr addrspace(5) %"96", align 4
  %"177" = load float, ptr addrspace(5) %"97", align 4
  %5 = fcmp ord float %"176", %"177"
  store i1 %5, ptr addrspace(5) %"99", align 1
  %"178" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"178", label %"35", label %"36"

"35":                                             ; preds = %"34"
  %"181" = load i32, ptr addrspace(5) %"179", align 4
  store i32 %"181", ptr addrspace(5) %"98", align 4
  br label %"36"

"36":                                             ; preds = %"35", %"34"
  %"182" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"182", label %"38", label %"37"

"37":                                             ; preds = %"36"
  %"185" = load i32, ptr addrspace(5) %"183", align 4
  store i32 %"185", ptr addrspace(5) %"98", align 4
  br label %"38"

"38":                                             ; preds = %"37", %"36"
  %"187" = load i64, ptr addrspace(5) %"89", align 8
  %"188" = load i64, ptr addrspace(5) %"186", align 8
  %"201" = inttoptr i64 %"187" to ptr
  %"84" = getelementptr inbounds i8, ptr %"201", i64 %"188"
  %"189" = load i32, ptr addrspace(5) %"98", align 4
  store i32 %"189", ptr %"84", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }