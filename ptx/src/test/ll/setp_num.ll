declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @setp_num(ptr addrspace(4) byref(i64) %"88", ptr addrspace(4) byref(i64) %"89") #1 {
  %"90" = alloca i64, align 8, addrspace(5)
  %"91" = alloca i64, align 8, addrspace(5)
  %"92" = alloca float, align 4, addrspace(5)
  %"93" = alloca float, align 4, addrspace(5)
  %"94" = alloca float, align 4, addrspace(5)
  %"95" = alloca float, align 4, addrspace(5)
  %"96" = alloca float, align 4, addrspace(5)
  %"97" = alloca float, align 4, addrspace(5)
  %"98" = alloca float, align 4, addrspace(5)
  %"99" = alloca float, align 4, addrspace(5)
  %"100" = alloca i32, align 4, addrspace(5)
  %"101" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"168"

"168":                                            ; preds = %1
  %"102" = load i64, ptr addrspace(4) %"88", align 4
  store i64 %"102", ptr addrspace(5) %"90", align 4
  %"103" = load i64, ptr addrspace(4) %"89", align 4
  store i64 %"103", ptr addrspace(5) %"91", align 4
  %"105" = load i64, ptr addrspace(5) %"90", align 4
  %"156" = inttoptr i64 %"105" to ptr
  %"104" = load float, ptr %"156", align 4
  store float %"104", ptr addrspace(5) %"92", align 4
  %"106" = load i64, ptr addrspace(5) %"90", align 4
  %"157" = inttoptr i64 %"106" to ptr
  %"55" = getelementptr inbounds i8, ptr %"157", i64 4
  %"107" = load float, ptr %"55", align 4
  store float %"107", ptr addrspace(5) %"93", align 4
  %"108" = load i64, ptr addrspace(5) %"90", align 4
  %"158" = inttoptr i64 %"108" to ptr
  %"57" = getelementptr inbounds i8, ptr %"158", i64 8
  %"109" = load float, ptr %"57", align 4
  store float %"109", ptr addrspace(5) %"94", align 4
  %"110" = load i64, ptr addrspace(5) %"90", align 4
  %"159" = inttoptr i64 %"110" to ptr
  %"59" = getelementptr inbounds i8, ptr %"159", i64 12
  %"111" = load float, ptr %"59", align 4
  store float %"111", ptr addrspace(5) %"95", align 4
  %"112" = load i64, ptr addrspace(5) %"90", align 4
  %"160" = inttoptr i64 %"112" to ptr
  %"61" = getelementptr inbounds i8, ptr %"160", i64 16
  %"113" = load float, ptr %"61", align 4
  store float %"113", ptr addrspace(5) %"96", align 4
  %"114" = load i64, ptr addrspace(5) %"90", align 4
  %"161" = inttoptr i64 %"114" to ptr
  %"63" = getelementptr inbounds i8, ptr %"161", i64 20
  %"115" = load float, ptr %"63", align 4
  store float %"115", ptr addrspace(5) %"97", align 4
  %"116" = load i64, ptr addrspace(5) %"90", align 4
  %"162" = inttoptr i64 %"116" to ptr
  %"65" = getelementptr inbounds i8, ptr %"162", i64 24
  %"117" = load float, ptr %"65", align 4
  store float %"117", ptr addrspace(5) %"98", align 4
  %"118" = load i64, ptr addrspace(5) %"90", align 4
  %"163" = inttoptr i64 %"118" to ptr
  %"67" = getelementptr inbounds i8, ptr %"163", i64 28
  %"119" = load float, ptr %"67", align 4
  store float %"119", ptr addrspace(5) %"99", align 4
  %"121" = load float, ptr addrspace(5) %"92", align 4
  %"122" = load float, ptr addrspace(5) %"93", align 4
  %"120" = fcmp ord float %"121", %"122"
  store i1 %"120", ptr addrspace(5) %"101", align 1
  %"123" = load i1, ptr addrspace(5) %"101", align 1
  br i1 %"123", label %"22", label %"23"

"22":                                             ; preds = %"168"
  store i32 2, ptr addrspace(5) %"100", align 4
  br label %"23"

"23":                                             ; preds = %"22", %"168"
  %"125" = load i1, ptr addrspace(5) %"101", align 1
  br i1 %"125", label %"25", label %"24"

"24":                                             ; preds = %"23"
  store i32 0, ptr addrspace(5) %"100", align 4
  br label %"25"

"25":                                             ; preds = %"24", %"23"
  %"127" = load i64, ptr addrspace(5) %"91", align 4
  %"128" = load i32, ptr addrspace(5) %"100", align 4
  %"164" = inttoptr i64 %"127" to ptr
  store i32 %"128", ptr %"164", align 4
  %"130" = load float, ptr addrspace(5) %"94", align 4
  %"131" = load float, ptr addrspace(5) %"95", align 4
  %"129" = fcmp ord float %"130", %"131"
  store i1 %"129", ptr addrspace(5) %"101", align 1
  %"132" = load i1, ptr addrspace(5) %"101", align 1
  br i1 %"132", label %"26", label %"27"

"26":                                             ; preds = %"25"
  store i32 2, ptr addrspace(5) %"100", align 4
  br label %"27"

"27":                                             ; preds = %"26", %"25"
  %"134" = load i1, ptr addrspace(5) %"101", align 1
  br i1 %"134", label %"29", label %"28"

"28":                                             ; preds = %"27"
  store i32 0, ptr addrspace(5) %"100", align 4
  br label %"29"

"29":                                             ; preds = %"28", %"27"
  %"136" = load i64, ptr addrspace(5) %"91", align 4
  %"165" = inttoptr i64 %"136" to ptr
  %"73" = getelementptr inbounds i8, ptr %"165", i64 4
  %"137" = load i32, ptr addrspace(5) %"100", align 4
  store i32 %"137", ptr %"73", align 4
  %"139" = load float, ptr addrspace(5) %"96", align 4
  %"140" = load float, ptr addrspace(5) %"97", align 4
  %"138" = fcmp ord float %"139", %"140"
  store i1 %"138", ptr addrspace(5) %"101", align 1
  %"141" = load i1, ptr addrspace(5) %"101", align 1
  br i1 %"141", label %"30", label %"31"

"30":                                             ; preds = %"29"
  store i32 2, ptr addrspace(5) %"100", align 4
  br label %"31"

"31":                                             ; preds = %"30", %"29"
  %"143" = load i1, ptr addrspace(5) %"101", align 1
  br i1 %"143", label %"33", label %"32"

"32":                                             ; preds = %"31"
  store i32 0, ptr addrspace(5) %"100", align 4
  br label %"33"

"33":                                             ; preds = %"32", %"31"
  %"145" = load i64, ptr addrspace(5) %"91", align 4
  %"166" = inttoptr i64 %"145" to ptr
  %"77" = getelementptr inbounds i8, ptr %"166", i64 8
  %"146" = load i32, ptr addrspace(5) %"100", align 4
  store i32 %"146", ptr %"77", align 4
  %"148" = load float, ptr addrspace(5) %"98", align 4
  %"149" = load float, ptr addrspace(5) %"99", align 4
  %"147" = fcmp ord float %"148", %"149"
  store i1 %"147", ptr addrspace(5) %"101", align 1
  %"150" = load i1, ptr addrspace(5) %"101", align 1
  br i1 %"150", label %"34", label %"35"

"34":                                             ; preds = %"33"
  store i32 2, ptr addrspace(5) %"100", align 4
  br label %"35"

"35":                                             ; preds = %"34", %"33"
  %"152" = load i1, ptr addrspace(5) %"101", align 1
  br i1 %"152", label %"37", label %"36"

"36":                                             ; preds = %"35"
  store i32 0, ptr addrspace(5) %"100", align 4
  br label %"37"

"37":                                             ; preds = %"36", %"35"
  %"154" = load i64, ptr addrspace(5) %"91", align 4
  %"167" = inttoptr i64 %"154" to ptr
  %"81" = getelementptr inbounds i8, ptr %"167", i64 12
  %"155" = load i32, ptr addrspace(5) %"100", align 4
  store i32 %"155", ptr %"81", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }