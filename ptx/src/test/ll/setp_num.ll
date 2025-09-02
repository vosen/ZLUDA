@0 = addrspace(4) global i64 4
@1 = addrspace(4) global i64 8
@2 = addrspace(4) global i64 12
@3 = addrspace(4) global i64 16
@4 = addrspace(4) global i64 20
@5 = addrspace(4) global i64 24
@6 = addrspace(4) global i64 28
@7 = addrspace(4) global i32 2
@8 = addrspace(4) global i32 0
@9 = addrspace(4) global i32 2
@10 = addrspace(4) global i32 0
@11 = addrspace(4) global i64 4
@12 = addrspace(4) global i32 2
@13 = addrspace(4) global i32 0
@14 = addrspace(4) global i64 8
@15 = addrspace(4) global i32 2
@16 = addrspace(4) global i32 0
@17 = addrspace(4) global i64 12

define amdgpu_kernel void @setp_num(ptr addrspace(4) byref(i64) %"104", ptr addrspace(4) byref(i64) %"105") #0 {
  %"106" = alloca i64, align 8, addrspace(5)
  %"107" = alloca i64, align 8, addrspace(5)
  %"108" = alloca float, align 4, addrspace(5)
  %"109" = alloca float, align 4, addrspace(5)
  %"110" = alloca float, align 4, addrspace(5)
  %"111" = alloca float, align 4, addrspace(5)
  %"112" = alloca float, align 4, addrspace(5)
  %"113" = alloca float, align 4, addrspace(5)
  %"114" = alloca float, align 4, addrspace(5)
  %"115" = alloca float, align 4, addrspace(5)
  %"116" = alloca i32, align 4, addrspace(5)
  %"117" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"103"

"103":                                            ; preds = %1
  %"118" = load i64, ptr addrspace(4) %"104", align 8
  store i64 %"118", ptr addrspace(5) %"106", align 8
  %"119" = load i64, ptr addrspace(4) %"105", align 8
  store i64 %"119", ptr addrspace(5) %"107", align 8
  %"121" = load i64, ptr addrspace(5) %"106", align 8
  %"172" = inttoptr i64 %"121" to ptr
  %"120" = load float, ptr %"172", align 4
  store float %"120", ptr addrspace(5) %"108", align 4
  %"58" = load i64, ptr addrspace(4) @0, align 8
  %"122" = load i64, ptr addrspace(5) %"106", align 8
  %"173" = inttoptr i64 %"122" to ptr
  %"59" = getelementptr inbounds i8, ptr %"173", i64 %"58"
  %"123" = load float, ptr %"59", align 4
  store float %"123", ptr addrspace(5) %"109", align 4
  %"61" = load i64, ptr addrspace(4) @1, align 8
  %"124" = load i64, ptr addrspace(5) %"106", align 8
  %"174" = inttoptr i64 %"124" to ptr
  %"62" = getelementptr inbounds i8, ptr %"174", i64 %"61"
  %"125" = load float, ptr %"62", align 4
  store float %"125", ptr addrspace(5) %"110", align 4
  %"64" = load i64, ptr addrspace(4) @2, align 8
  %"126" = load i64, ptr addrspace(5) %"106", align 8
  %"175" = inttoptr i64 %"126" to ptr
  %"65" = getelementptr inbounds i8, ptr %"175", i64 %"64"
  %"127" = load float, ptr %"65", align 4
  store float %"127", ptr addrspace(5) %"111", align 4
  %"67" = load i64, ptr addrspace(4) @3, align 8
  %"128" = load i64, ptr addrspace(5) %"106", align 8
  %"176" = inttoptr i64 %"128" to ptr
  %"68" = getelementptr inbounds i8, ptr %"176", i64 %"67"
  %"129" = load float, ptr %"68", align 4
  store float %"129", ptr addrspace(5) %"112", align 4
  %"70" = load i64, ptr addrspace(4) @4, align 8
  %"130" = load i64, ptr addrspace(5) %"106", align 8
  %"177" = inttoptr i64 %"130" to ptr
  %"71" = getelementptr inbounds i8, ptr %"177", i64 %"70"
  %"131" = load float, ptr %"71", align 4
  store float %"131", ptr addrspace(5) %"113", align 4
  %"73" = load i64, ptr addrspace(4) @5, align 8
  %"132" = load i64, ptr addrspace(5) %"106", align 8
  %"178" = inttoptr i64 %"132" to ptr
  %"74" = getelementptr inbounds i8, ptr %"178", i64 %"73"
  %"133" = load float, ptr %"74", align 4
  store float %"133", ptr addrspace(5) %"114", align 4
  %"76" = load i64, ptr addrspace(4) @6, align 8
  %"134" = load i64, ptr addrspace(5) %"106", align 8
  %"179" = inttoptr i64 %"134" to ptr
  %"77" = getelementptr inbounds i8, ptr %"179", i64 %"76"
  %"135" = load float, ptr %"77", align 4
  store float %"135", ptr addrspace(5) %"115", align 4
  %"137" = load float, ptr addrspace(5) %"108", align 4
  %"138" = load float, ptr addrspace(5) %"109", align 4
  %2 = fcmp ord float %"137", %"138"
  store i1 %2, ptr addrspace(5) %"117", align 1
  %"139" = load i1, ptr addrspace(5) %"117", align 1
  br i1 %"139", label %"23", label %"24"

"23":                                             ; preds = %"103"
  %"79" = load i32, ptr addrspace(4) @7, align 4
  store i32 %"79", ptr addrspace(5) %"116", align 4
  br label %"24"

"24":                                             ; preds = %"23", %"103"
  %"141" = load i1, ptr addrspace(5) %"117", align 1
  br i1 %"141", label %"26", label %"25"

"25":                                             ; preds = %"24"
  %"81" = load i32, ptr addrspace(4) @8, align 4
  store i32 %"81", ptr addrspace(5) %"116", align 4
  br label %"26"

"26":                                             ; preds = %"25", %"24"
  %"143" = load i64, ptr addrspace(5) %"107", align 8
  %"144" = load i32, ptr addrspace(5) %"116", align 4
  %"180" = inttoptr i64 %"143" to ptr
  store i32 %"144", ptr %"180", align 4
  %"146" = load float, ptr addrspace(5) %"110", align 4
  %"147" = load float, ptr addrspace(5) %"111", align 4
  %3 = fcmp ord float %"146", %"147"
  store i1 %3, ptr addrspace(5) %"117", align 1
  %"148" = load i1, ptr addrspace(5) %"117", align 1
  br i1 %"148", label %"27", label %"28"

"27":                                             ; preds = %"26"
  %"83" = load i32, ptr addrspace(4) @9, align 4
  store i32 %"83", ptr addrspace(5) %"116", align 4
  br label %"28"

"28":                                             ; preds = %"27", %"26"
  %"150" = load i1, ptr addrspace(5) %"117", align 1
  br i1 %"150", label %"30", label %"29"

"29":                                             ; preds = %"28"
  %"85" = load i32, ptr addrspace(4) @10, align 4
  store i32 %"85", ptr addrspace(5) %"116", align 4
  br label %"30"

"30":                                             ; preds = %"29", %"28"
  %"87" = load i64, ptr addrspace(4) @11, align 8
  %"152" = load i64, ptr addrspace(5) %"107", align 8
  %"181" = inttoptr i64 %"152" to ptr
  %"88" = getelementptr inbounds i8, ptr %"181", i64 %"87"
  %"153" = load i32, ptr addrspace(5) %"116", align 4
  store i32 %"153", ptr %"88", align 4
  %"155" = load float, ptr addrspace(5) %"112", align 4
  %"156" = load float, ptr addrspace(5) %"113", align 4
  %4 = fcmp ord float %"155", %"156"
  store i1 %4, ptr addrspace(5) %"117", align 1
  %"157" = load i1, ptr addrspace(5) %"117", align 1
  br i1 %"157", label %"31", label %"32"

"31":                                             ; preds = %"30"
  %"90" = load i32, ptr addrspace(4) @12, align 4
  store i32 %"90", ptr addrspace(5) %"116", align 4
  br label %"32"

"32":                                             ; preds = %"31", %"30"
  %"159" = load i1, ptr addrspace(5) %"117", align 1
  br i1 %"159", label %"34", label %"33"

"33":                                             ; preds = %"32"
  %"92" = load i32, ptr addrspace(4) @13, align 4
  store i32 %"92", ptr addrspace(5) %"116", align 4
  br label %"34"

"34":                                             ; preds = %"33", %"32"
  %"94" = load i64, ptr addrspace(4) @14, align 8
  %"161" = load i64, ptr addrspace(5) %"107", align 8
  %"182" = inttoptr i64 %"161" to ptr
  %"95" = getelementptr inbounds i8, ptr %"182", i64 %"94"
  %"162" = load i32, ptr addrspace(5) %"116", align 4
  store i32 %"162", ptr %"95", align 4
  %"164" = load float, ptr addrspace(5) %"114", align 4
  %"165" = load float, ptr addrspace(5) %"115", align 4
  %5 = fcmp ord float %"164", %"165"
  store i1 %5, ptr addrspace(5) %"117", align 1
  %"166" = load i1, ptr addrspace(5) %"117", align 1
  br i1 %"166", label %"35", label %"36"

"35":                                             ; preds = %"34"
  %"97" = load i32, ptr addrspace(4) @15, align 4
  store i32 %"97", ptr addrspace(5) %"116", align 4
  br label %"36"

"36":                                             ; preds = %"35", %"34"
  %"168" = load i1, ptr addrspace(5) %"117", align 1
  br i1 %"168", label %"38", label %"37"

"37":                                             ; preds = %"36"
  %"99" = load i32, ptr addrspace(4) @16, align 4
  store i32 %"99", ptr addrspace(5) %"116", align 4
  br label %"38"

"38":                                             ; preds = %"37", %"36"
  %"101" = load i64, ptr addrspace(4) @17, align 8
  %"170" = load i64, ptr addrspace(5) %"107", align 8
  %"183" = inttoptr i64 %"170" to ptr
  %"102" = getelementptr inbounds i8, ptr %"183", i64 %"101"
  %"171" = load i32, ptr addrspace(5) %"116", align 4
  store i32 %"171", ptr %"102", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }