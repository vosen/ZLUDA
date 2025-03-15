declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @setp_nan(ptr addrspace(4) byref(i64) %"89", ptr addrspace(4) byref(i64) %"90") #1 {
  %"91" = alloca i64, align 8, addrspace(5)
  %"92" = alloca i64, align 8, addrspace(5)
  %"93" = alloca float, align 4, addrspace(5)
  %"94" = alloca float, align 4, addrspace(5)
  %"95" = alloca float, align 4, addrspace(5)
  %"96" = alloca float, align 4, addrspace(5)
  %"97" = alloca float, align 4, addrspace(5)
  %"98" = alloca float, align 4, addrspace(5)
  %"99" = alloca float, align 4, addrspace(5)
  %"100" = alloca float, align 4, addrspace(5)
  %"101" = alloca i32, align 4, addrspace(5)
  %"102" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"82"

"82":                                             ; preds = %1
  %"103" = load i64, ptr addrspace(4) %"89", align 4
  store i64 %"103", ptr addrspace(5) %"91", align 4
  %"104" = load i64, ptr addrspace(4) %"90", align 4
  store i64 %"104", ptr addrspace(5) %"92", align 4
  %"106" = load i64, ptr addrspace(5) %"91", align 4
  %"157" = inttoptr i64 %"106" to ptr
  %"105" = load float, ptr %"157", align 4
  store float %"105", ptr addrspace(5) %"93", align 4
  %"107" = load i64, ptr addrspace(5) %"91", align 4
  %"158" = inttoptr i64 %"107" to ptr
  %"55" = getelementptr inbounds i8, ptr %"158", i64 4
  %"108" = load float, ptr %"55", align 4
  store float %"108", ptr addrspace(5) %"94", align 4
  %"109" = load i64, ptr addrspace(5) %"91", align 4
  %"159" = inttoptr i64 %"109" to ptr
  %"57" = getelementptr inbounds i8, ptr %"159", i64 8
  %"110" = load float, ptr %"57", align 4
  store float %"110", ptr addrspace(5) %"95", align 4
  %"111" = load i64, ptr addrspace(5) %"91", align 4
  %"160" = inttoptr i64 %"111" to ptr
  %"59" = getelementptr inbounds i8, ptr %"160", i64 12
  %"112" = load float, ptr %"59", align 4
  store float %"112", ptr addrspace(5) %"96", align 4
  %"113" = load i64, ptr addrspace(5) %"91", align 4
  %"161" = inttoptr i64 %"113" to ptr
  %"61" = getelementptr inbounds i8, ptr %"161", i64 16
  %"114" = load float, ptr %"61", align 4
  store float %"114", ptr addrspace(5) %"97", align 4
  %"115" = load i64, ptr addrspace(5) %"91", align 4
  %"162" = inttoptr i64 %"115" to ptr
  %"63" = getelementptr inbounds i8, ptr %"162", i64 20
  %"116" = load float, ptr %"63", align 4
  store float %"116", ptr addrspace(5) %"98", align 4
  %"117" = load i64, ptr addrspace(5) %"91", align 4
  %"163" = inttoptr i64 %"117" to ptr
  %"65" = getelementptr inbounds i8, ptr %"163", i64 24
  %"118" = load float, ptr %"65", align 4
  store float %"118", ptr addrspace(5) %"99", align 4
  %"119" = load i64, ptr addrspace(5) %"91", align 4
  %"164" = inttoptr i64 %"119" to ptr
  %"67" = getelementptr inbounds i8, ptr %"164", i64 28
  %"120" = load float, ptr %"67", align 4
  store float %"120", ptr addrspace(5) %"100", align 4
  %"122" = load float, ptr addrspace(5) %"93", align 4
  %"123" = load float, ptr addrspace(5) %"94", align 4
  %"121" = fcmp uno float %"122", %"123"
  store i1 %"121", ptr addrspace(5) %"102", align 1
  %"124" = load i1, ptr addrspace(5) %"102", align 1
  br i1 %"124", label %"22", label %"23"

"22":                                             ; preds = %"82"
  store i32 1, ptr addrspace(5) %"101", align 4
  br label %"23"

"23":                                             ; preds = %"22", %"82"
  %"126" = load i1, ptr addrspace(5) %"102", align 1
  br i1 %"126", label %"25", label %"24"

"24":                                             ; preds = %"23"
  store i32 0, ptr addrspace(5) %"101", align 4
  br label %"25"

"25":                                             ; preds = %"24", %"23"
  %"128" = load i64, ptr addrspace(5) %"92", align 4
  %"129" = load i32, ptr addrspace(5) %"101", align 4
  %"165" = inttoptr i64 %"128" to ptr
  store i32 %"129", ptr %"165", align 4
  %"131" = load float, ptr addrspace(5) %"95", align 4
  %"132" = load float, ptr addrspace(5) %"96", align 4
  %"130" = fcmp uno float %"131", %"132"
  store i1 %"130", ptr addrspace(5) %"102", align 1
  %"133" = load i1, ptr addrspace(5) %"102", align 1
  br i1 %"133", label %"26", label %"27"

"26":                                             ; preds = %"25"
  store i32 1, ptr addrspace(5) %"101", align 4
  br label %"27"

"27":                                             ; preds = %"26", %"25"
  %"135" = load i1, ptr addrspace(5) %"102", align 1
  br i1 %"135", label %"29", label %"28"

"28":                                             ; preds = %"27"
  store i32 0, ptr addrspace(5) %"101", align 4
  br label %"29"

"29":                                             ; preds = %"28", %"27"
  %"137" = load i64, ptr addrspace(5) %"92", align 4
  %"166" = inttoptr i64 %"137" to ptr
  %"73" = getelementptr inbounds i8, ptr %"166", i64 4
  %"138" = load i32, ptr addrspace(5) %"101", align 4
  store i32 %"138", ptr %"73", align 4
  %"140" = load float, ptr addrspace(5) %"97", align 4
  %"141" = load float, ptr addrspace(5) %"98", align 4
  %"139" = fcmp uno float %"140", %"141"
  store i1 %"139", ptr addrspace(5) %"102", align 1
  %"142" = load i1, ptr addrspace(5) %"102", align 1
  br i1 %"142", label %"30", label %"31"

"30":                                             ; preds = %"29"
  store i32 1, ptr addrspace(5) %"101", align 4
  br label %"31"

"31":                                             ; preds = %"30", %"29"
  %"144" = load i1, ptr addrspace(5) %"102", align 1
  br i1 %"144", label %"33", label %"32"

"32":                                             ; preds = %"31"
  store i32 0, ptr addrspace(5) %"101", align 4
  br label %"33"

"33":                                             ; preds = %"32", %"31"
  %"146" = load i64, ptr addrspace(5) %"92", align 4
  %"167" = inttoptr i64 %"146" to ptr
  %"77" = getelementptr inbounds i8, ptr %"167", i64 8
  %"147" = load i32, ptr addrspace(5) %"101", align 4
  store i32 %"147", ptr %"77", align 4
  %"149" = load float, ptr addrspace(5) %"99", align 4
  %"150" = load float, ptr addrspace(5) %"100", align 4
  %"148" = fcmp uno float %"149", %"150"
  store i1 %"148", ptr addrspace(5) %"102", align 1
  %"151" = load i1, ptr addrspace(5) %"102", align 1
  br i1 %"151", label %"34", label %"35"

"34":                                             ; preds = %"33"
  store i32 1, ptr addrspace(5) %"101", align 4
  br label %"35"

"35":                                             ; preds = %"34", %"33"
  %"153" = load i1, ptr addrspace(5) %"102", align 1
  br i1 %"153", label %"37", label %"36"

"36":                                             ; preds = %"35"
  store i32 0, ptr addrspace(5) %"101", align 4
  br label %"37"

"37":                                             ; preds = %"36", %"35"
  %"155" = load i64, ptr addrspace(5) %"92", align 4
  %"168" = inttoptr i64 %"155" to ptr
  %"81" = getelementptr inbounds i8, ptr %"168", i64 12
  %"156" = load i32, ptr addrspace(5) %"101", align 4
  store i32 %"156", ptr %"81", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }