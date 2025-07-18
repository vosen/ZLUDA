@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define amdgpu_kernel void @setp_nan(ptr addrspace(4) byref(i64) %"83", ptr addrspace(4) byref(i64) %"84") #0 {
  %"85" = alloca i64, align 8, addrspace(5)
  %"86" = alloca i64, align 8, addrspace(5)
  %"87" = alloca float, align 4, addrspace(5)
  %"88" = alloca float, align 4, addrspace(5)
  %"89" = alloca float, align 4, addrspace(5)
  %"90" = alloca float, align 4, addrspace(5)
  %"91" = alloca float, align 4, addrspace(5)
  %"92" = alloca float, align 4, addrspace(5)
  %"93" = alloca float, align 4, addrspace(5)
  %"94" = alloca float, align 4, addrspace(5)
  %"95" = alloca i32, align 4, addrspace(5)
  %"96" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"82"

"82":                                             ; preds = %1
  %"97" = load i64, ptr addrspace(4) %"83", align 4
  store i64 %"97", ptr addrspace(5) %"85", align 4
  %"98" = load i64, ptr addrspace(4) %"84", align 4
  store i64 %"98", ptr addrspace(5) %"86", align 4
  %"100" = load i64, ptr addrspace(5) %"85", align 4
  %"151" = inttoptr i64 %"100" to ptr
  %"99" = load float, ptr %"151", align 4
  store float %"99", ptr addrspace(5) %"87", align 4
  %"101" = load i64, ptr addrspace(5) %"85", align 4
  %"152" = inttoptr i64 %"101" to ptr
  %"55" = getelementptr inbounds i8, ptr %"152", i64 4
  %"102" = load float, ptr %"55", align 4
  store float %"102", ptr addrspace(5) %"88", align 4
  %"103" = load i64, ptr addrspace(5) %"85", align 4
  %"153" = inttoptr i64 %"103" to ptr
  %"57" = getelementptr inbounds i8, ptr %"153", i64 8
  %"104" = load float, ptr %"57", align 4
  store float %"104", ptr addrspace(5) %"89", align 4
  %"105" = load i64, ptr addrspace(5) %"85", align 4
  %"154" = inttoptr i64 %"105" to ptr
  %"59" = getelementptr inbounds i8, ptr %"154", i64 12
  %"106" = load float, ptr %"59", align 4
  store float %"106", ptr addrspace(5) %"90", align 4
  %"107" = load i64, ptr addrspace(5) %"85", align 4
  %"155" = inttoptr i64 %"107" to ptr
  %"61" = getelementptr inbounds i8, ptr %"155", i64 16
  %"108" = load float, ptr %"61", align 4
  store float %"108", ptr addrspace(5) %"91", align 4
  %"109" = load i64, ptr addrspace(5) %"85", align 4
  %"156" = inttoptr i64 %"109" to ptr
  %"63" = getelementptr inbounds i8, ptr %"156", i64 20
  %"110" = load float, ptr %"63", align 4
  store float %"110", ptr addrspace(5) %"92", align 4
  %"111" = load i64, ptr addrspace(5) %"85", align 4
  %"157" = inttoptr i64 %"111" to ptr
  %"65" = getelementptr inbounds i8, ptr %"157", i64 24
  %"112" = load float, ptr %"65", align 4
  store float %"112", ptr addrspace(5) %"93", align 4
  %"113" = load i64, ptr addrspace(5) %"85", align 4
  %"158" = inttoptr i64 %"113" to ptr
  %"67" = getelementptr inbounds i8, ptr %"158", i64 28
  %"114" = load float, ptr %"67", align 4
  store float %"114", ptr addrspace(5) %"94", align 4
  %"116" = load float, ptr addrspace(5) %"87", align 4
  %"117" = load float, ptr addrspace(5) %"88", align 4
  %"115" = fcmp uno float %"116", %"117"
  store i1 %"115", ptr addrspace(5) %"96", align 1
  %"118" = load i1, ptr addrspace(5) %"96", align 1
  br i1 %"118", label %"22", label %"23"

"22":                                             ; preds = %"82"
  store i32 1, ptr addrspace(5) %"95", align 4
  br label %"23"

"23":                                             ; preds = %"22", %"82"
  %"120" = load i1, ptr addrspace(5) %"96", align 1
  br i1 %"120", label %"25", label %"24"

"24":                                             ; preds = %"23"
  store i32 0, ptr addrspace(5) %"95", align 4
  br label %"25"

"25":                                             ; preds = %"24", %"23"
  %"122" = load i64, ptr addrspace(5) %"86", align 4
  %"123" = load i32, ptr addrspace(5) %"95", align 4
  %"159" = inttoptr i64 %"122" to ptr
  store i32 %"123", ptr %"159", align 4
  %"125" = load float, ptr addrspace(5) %"89", align 4
  %"126" = load float, ptr addrspace(5) %"90", align 4
  %"124" = fcmp uno float %"125", %"126"
  store i1 %"124", ptr addrspace(5) %"96", align 1
  %"127" = load i1, ptr addrspace(5) %"96", align 1
  br i1 %"127", label %"26", label %"27"

"26":                                             ; preds = %"25"
  store i32 1, ptr addrspace(5) %"95", align 4
  br label %"27"

"27":                                             ; preds = %"26", %"25"
  %"129" = load i1, ptr addrspace(5) %"96", align 1
  br i1 %"129", label %"29", label %"28"

"28":                                             ; preds = %"27"
  store i32 0, ptr addrspace(5) %"95", align 4
  br label %"29"

"29":                                             ; preds = %"28", %"27"
  %"131" = load i64, ptr addrspace(5) %"86", align 4
  %"160" = inttoptr i64 %"131" to ptr
  %"73" = getelementptr inbounds i8, ptr %"160", i64 4
  %"132" = load i32, ptr addrspace(5) %"95", align 4
  store i32 %"132", ptr %"73", align 4
  %"134" = load float, ptr addrspace(5) %"91", align 4
  %"135" = load float, ptr addrspace(5) %"92", align 4
  %"133" = fcmp uno float %"134", %"135"
  store i1 %"133", ptr addrspace(5) %"96", align 1
  %"136" = load i1, ptr addrspace(5) %"96", align 1
  br i1 %"136", label %"30", label %"31"

"30":                                             ; preds = %"29"
  store i32 1, ptr addrspace(5) %"95", align 4
  br label %"31"

"31":                                             ; preds = %"30", %"29"
  %"138" = load i1, ptr addrspace(5) %"96", align 1
  br i1 %"138", label %"33", label %"32"

"32":                                             ; preds = %"31"
  store i32 0, ptr addrspace(5) %"95", align 4
  br label %"33"

"33":                                             ; preds = %"32", %"31"
  %"140" = load i64, ptr addrspace(5) %"86", align 4
  %"161" = inttoptr i64 %"140" to ptr
  %"77" = getelementptr inbounds i8, ptr %"161", i64 8
  %"141" = load i32, ptr addrspace(5) %"95", align 4
  store i32 %"141", ptr %"77", align 4
  %"143" = load float, ptr addrspace(5) %"93", align 4
  %"144" = load float, ptr addrspace(5) %"94", align 4
  %"142" = fcmp uno float %"143", %"144"
  store i1 %"142", ptr addrspace(5) %"96", align 1
  %"145" = load i1, ptr addrspace(5) %"96", align 1
  br i1 %"145", label %"34", label %"35"

"34":                                             ; preds = %"33"
  store i32 1, ptr addrspace(5) %"95", align 4
  br label %"35"

"35":                                             ; preds = %"34", %"33"
  %"147" = load i1, ptr addrspace(5) %"96", align 1
  br i1 %"147", label %"37", label %"36"

"36":                                             ; preds = %"35"
  store i32 0, ptr addrspace(5) %"95", align 4
  br label %"37"

"37":                                             ; preds = %"36", %"35"
  %"149" = load i64, ptr addrspace(5) %"86", align 4
  %"162" = inttoptr i64 %"149" to ptr
  %"81" = getelementptr inbounds i8, ptr %"162", i64 12
  %"150" = load i32, ptr addrspace(5) %"95", align 4
  store i32 %"150", ptr %"81", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }