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
  br label %1

1:                                                ; preds = %0
  br label %"85"

"85":                                             ; preds = %1
  %"100" = load i64, ptr addrspace(4) %"86", align 8
  store i64 %"100", ptr addrspace(5) %"88", align 8
  %"101" = load i64, ptr addrspace(4) %"87", align 8
  store i64 %"101", ptr addrspace(5) %"89", align 8
  %"103" = load i64, ptr addrspace(5) %"88", align 8
  %"154" = inttoptr i64 %"103" to ptr
  %"102" = load float, ptr %"154", align 4
  store float %"102", ptr addrspace(5) %"90", align 4
  %"104" = load i64, ptr addrspace(5) %"88", align 8
  %"155" = inttoptr i64 %"104" to ptr
  %"58" = getelementptr inbounds i8, ptr %"155", i64 4
  %"105" = load float, ptr %"58", align 4
  store float %"105", ptr addrspace(5) %"91", align 4
  %"106" = load i64, ptr addrspace(5) %"88", align 8
  %"156" = inttoptr i64 %"106" to ptr
  %"60" = getelementptr inbounds i8, ptr %"156", i64 8
  %"107" = load float, ptr %"60", align 4
  store float %"107", ptr addrspace(5) %"92", align 4
  %"108" = load i64, ptr addrspace(5) %"88", align 8
  %"157" = inttoptr i64 %"108" to ptr
  %"62" = getelementptr inbounds i8, ptr %"157", i64 12
  %"109" = load float, ptr %"62", align 4
  store float %"109", ptr addrspace(5) %"93", align 4
  %"110" = load i64, ptr addrspace(5) %"88", align 8
  %"158" = inttoptr i64 %"110" to ptr
  %"64" = getelementptr inbounds i8, ptr %"158", i64 16
  %"111" = load float, ptr %"64", align 4
  store float %"111", ptr addrspace(5) %"94", align 4
  %"112" = load i64, ptr addrspace(5) %"88", align 8
  %"159" = inttoptr i64 %"112" to ptr
  %"66" = getelementptr inbounds i8, ptr %"159", i64 20
  %"113" = load float, ptr %"66", align 4
  store float %"113", ptr addrspace(5) %"95", align 4
  %"114" = load i64, ptr addrspace(5) %"88", align 8
  %"160" = inttoptr i64 %"114" to ptr
  %"68" = getelementptr inbounds i8, ptr %"160", i64 24
  %"115" = load float, ptr %"68", align 4
  store float %"115", ptr addrspace(5) %"96", align 4
  %"116" = load i64, ptr addrspace(5) %"88", align 8
  %"161" = inttoptr i64 %"116" to ptr
  %"70" = getelementptr inbounds i8, ptr %"161", i64 28
  %"117" = load float, ptr %"70", align 4
  store float %"117", ptr addrspace(5) %"97", align 4
  %"119" = load float, ptr addrspace(5) %"90", align 4
  %"120" = load float, ptr addrspace(5) %"91", align 4
  %2 = fcmp ord float %"119", %"120"
  store i1 %2, ptr addrspace(5) %"99", align 1
  %"121" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"121", label %"23", label %"24"

"23":                                             ; preds = %"85"
  store i32 2, ptr addrspace(5) %"98", align 4
  br label %"24"

"24":                                             ; preds = %"23", %"85"
  %"123" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"123", label %"26", label %"25"

"25":                                             ; preds = %"24"
  store i32 0, ptr addrspace(5) %"98", align 4
  br label %"26"

"26":                                             ; preds = %"25", %"24"
  %"125" = load i64, ptr addrspace(5) %"89", align 8
  %"126" = load i32, ptr addrspace(5) %"98", align 4
  %"162" = inttoptr i64 %"125" to ptr
  store i32 %"126", ptr %"162", align 4
  %"128" = load float, ptr addrspace(5) %"92", align 4
  %"129" = load float, ptr addrspace(5) %"93", align 4
  %3 = fcmp ord float %"128", %"129"
  store i1 %3, ptr addrspace(5) %"99", align 1
  %"130" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"130", label %"27", label %"28"

"27":                                             ; preds = %"26"
  store i32 2, ptr addrspace(5) %"98", align 4
  br label %"28"

"28":                                             ; preds = %"27", %"26"
  %"132" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"132", label %"30", label %"29"

"29":                                             ; preds = %"28"
  store i32 0, ptr addrspace(5) %"98", align 4
  br label %"30"

"30":                                             ; preds = %"29", %"28"
  %"134" = load i64, ptr addrspace(5) %"89", align 8
  %"163" = inttoptr i64 %"134" to ptr
  %"76" = getelementptr inbounds i8, ptr %"163", i64 4
  %"135" = load i32, ptr addrspace(5) %"98", align 4
  store i32 %"135", ptr %"76", align 4
  %"137" = load float, ptr addrspace(5) %"94", align 4
  %"138" = load float, ptr addrspace(5) %"95", align 4
  %4 = fcmp ord float %"137", %"138"
  store i1 %4, ptr addrspace(5) %"99", align 1
  %"139" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"139", label %"31", label %"32"

"31":                                             ; preds = %"30"
  store i32 2, ptr addrspace(5) %"98", align 4
  br label %"32"

"32":                                             ; preds = %"31", %"30"
  %"141" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"141", label %"34", label %"33"

"33":                                             ; preds = %"32"
  store i32 0, ptr addrspace(5) %"98", align 4
  br label %"34"

"34":                                             ; preds = %"33", %"32"
  %"143" = load i64, ptr addrspace(5) %"89", align 8
  %"164" = inttoptr i64 %"143" to ptr
  %"80" = getelementptr inbounds i8, ptr %"164", i64 8
  %"144" = load i32, ptr addrspace(5) %"98", align 4
  store i32 %"144", ptr %"80", align 4
  %"146" = load float, ptr addrspace(5) %"96", align 4
  %"147" = load float, ptr addrspace(5) %"97", align 4
  %5 = fcmp ord float %"146", %"147"
  store i1 %5, ptr addrspace(5) %"99", align 1
  %"148" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"148", label %"35", label %"36"

"35":                                             ; preds = %"34"
  store i32 2, ptr addrspace(5) %"98", align 4
  br label %"36"

"36":                                             ; preds = %"35", %"34"
  %"150" = load i1, ptr addrspace(5) %"99", align 1
  br i1 %"150", label %"38", label %"37"

"37":                                             ; preds = %"36"
  store i32 0, ptr addrspace(5) %"98", align 4
  br label %"38"

"38":                                             ; preds = %"37", %"36"
  %"152" = load i64, ptr addrspace(5) %"89", align 8
  %"165" = inttoptr i64 %"152" to ptr
  %"84" = getelementptr inbounds i8, ptr %"165", i64 12
  %"153" = load i32, ptr addrspace(5) %"98", align 4
  store i32 %"153", ptr %"84", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
