declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @setp_num(ptr addrspace(4) byref(i64) %"87", ptr addrspace(4) byref(i64) %"88") #0 {
  %"89" = alloca i64, align 8, addrspace(5)
  %"90" = alloca i64, align 8, addrspace(5)
  %"91" = alloca float, align 4, addrspace(5)
  %"92" = alloca float, align 4, addrspace(5)
  %"93" = alloca float, align 4, addrspace(5)
  %"94" = alloca float, align 4, addrspace(5)
  %"95" = alloca float, align 4, addrspace(5)
  %"96" = alloca float, align 4, addrspace(5)
  %"97" = alloca float, align 4, addrspace(5)
  %"98" = alloca float, align 4, addrspace(5)
  %"99" = alloca i32, align 4, addrspace(5)
  %"100" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"101" = load i64, ptr addrspace(4) %"87", align 4
  store i64 %"101", ptr addrspace(5) %"89", align 4
  %"102" = load i64, ptr addrspace(4) %"88", align 4
  store i64 %"102", ptr addrspace(5) %"90", align 4
  %"104" = load i64, ptr addrspace(5) %"89", align 4
  %"155" = inttoptr i64 %"104" to ptr
  %"103" = load float, ptr %"155", align 4
  store float %"103", ptr addrspace(5) %"91", align 4
  %"105" = load i64, ptr addrspace(5) %"89", align 4
  %"156" = inttoptr i64 %"105" to ptr
  %"54" = getelementptr inbounds i8, ptr %"156", i64 4
  %"106" = load float, ptr %"54", align 4
  store float %"106", ptr addrspace(5) %"92", align 4
  %"107" = load i64, ptr addrspace(5) %"89", align 4
  %"157" = inttoptr i64 %"107" to ptr
  %"56" = getelementptr inbounds i8, ptr %"157", i64 8
  %"108" = load float, ptr %"56", align 4
  store float %"108", ptr addrspace(5) %"93", align 4
  %"109" = load i64, ptr addrspace(5) %"89", align 4
  %"158" = inttoptr i64 %"109" to ptr
  %"58" = getelementptr inbounds i8, ptr %"158", i64 12
  %"110" = load float, ptr %"58", align 4
  store float %"110", ptr addrspace(5) %"94", align 4
  %"111" = load i64, ptr addrspace(5) %"89", align 4
  %"159" = inttoptr i64 %"111" to ptr
  %"60" = getelementptr inbounds i8, ptr %"159", i64 16
  %"112" = load float, ptr %"60", align 4
  store float %"112", ptr addrspace(5) %"95", align 4
  %"113" = load i64, ptr addrspace(5) %"89", align 4
  %"160" = inttoptr i64 %"113" to ptr
  %"62" = getelementptr inbounds i8, ptr %"160", i64 20
  %"114" = load float, ptr %"62", align 4
  store float %"114", ptr addrspace(5) %"96", align 4
  %"115" = load i64, ptr addrspace(5) %"89", align 4
  %"161" = inttoptr i64 %"115" to ptr
  %"64" = getelementptr inbounds i8, ptr %"161", i64 24
  %"116" = load float, ptr %"64", align 4
  store float %"116", ptr addrspace(5) %"97", align 4
  %"117" = load i64, ptr addrspace(5) %"89", align 4
  %"162" = inttoptr i64 %"117" to ptr
  %"66" = getelementptr inbounds i8, ptr %"162", i64 28
  %"118" = load float, ptr %"66", align 4
  store float %"118", ptr addrspace(5) %"98", align 4
  %"120" = load float, ptr addrspace(5) %"91", align 4
  %"121" = load float, ptr addrspace(5) %"92", align 4
  %"119" = fcmp ord float %"120", %"121"
  store i1 %"119", ptr addrspace(5) %"100", align 1
  %"122" = load i1, ptr addrspace(5) %"100", align 1
  br i1 %"122", label %"21", label %"22"

"21":                                             ; preds = %1
  store i32 2, ptr addrspace(5) %"99", align 4
  br label %"22"

"22":                                             ; preds = %"21", %1
  %"124" = load i1, ptr addrspace(5) %"100", align 1
  br i1 %"124", label %"24", label %"23"

"23":                                             ; preds = %"22"
  store i32 0, ptr addrspace(5) %"99", align 4
  br label %"24"

"24":                                             ; preds = %"23", %"22"
  %"126" = load i64, ptr addrspace(5) %"90", align 4
  %"127" = load i32, ptr addrspace(5) %"99", align 4
  %"163" = inttoptr i64 %"126" to ptr
  store i32 %"127", ptr %"163", align 4
  %"129" = load float, ptr addrspace(5) %"93", align 4
  %"130" = load float, ptr addrspace(5) %"94", align 4
  %"128" = fcmp ord float %"129", %"130"
  store i1 %"128", ptr addrspace(5) %"100", align 1
  %"131" = load i1, ptr addrspace(5) %"100", align 1
  br i1 %"131", label %"25", label %"26"

"25":                                             ; preds = %"24"
  store i32 2, ptr addrspace(5) %"99", align 4
  br label %"26"

"26":                                             ; preds = %"25", %"24"
  %"133" = load i1, ptr addrspace(5) %"100", align 1
  br i1 %"133", label %"28", label %"27"

"27":                                             ; preds = %"26"
  store i32 0, ptr addrspace(5) %"99", align 4
  br label %"28"

"28":                                             ; preds = %"27", %"26"
  %"135" = load i64, ptr addrspace(5) %"90", align 4
  %"164" = inttoptr i64 %"135" to ptr
  %"72" = getelementptr inbounds i8, ptr %"164", i64 4
  %"136" = load i32, ptr addrspace(5) %"99", align 4
  store i32 %"136", ptr %"72", align 4
  %"138" = load float, ptr addrspace(5) %"95", align 4
  %"139" = load float, ptr addrspace(5) %"96", align 4
  %"137" = fcmp ord float %"138", %"139"
  store i1 %"137", ptr addrspace(5) %"100", align 1
  %"140" = load i1, ptr addrspace(5) %"100", align 1
  br i1 %"140", label %"29", label %"30"

"29":                                             ; preds = %"28"
  store i32 2, ptr addrspace(5) %"99", align 4
  br label %"30"

"30":                                             ; preds = %"29", %"28"
  %"142" = load i1, ptr addrspace(5) %"100", align 1
  br i1 %"142", label %"32", label %"31"

"31":                                             ; preds = %"30"
  store i32 0, ptr addrspace(5) %"99", align 4
  br label %"32"

"32":                                             ; preds = %"31", %"30"
  %"144" = load i64, ptr addrspace(5) %"90", align 4
  %"165" = inttoptr i64 %"144" to ptr
  %"76" = getelementptr inbounds i8, ptr %"165", i64 8
  %"145" = load i32, ptr addrspace(5) %"99", align 4
  store i32 %"145", ptr %"76", align 4
  %"147" = load float, ptr addrspace(5) %"97", align 4
  %"148" = load float, ptr addrspace(5) %"98", align 4
  %"146" = fcmp ord float %"147", %"148"
  store i1 %"146", ptr addrspace(5) %"100", align 1
  %"149" = load i1, ptr addrspace(5) %"100", align 1
  br i1 %"149", label %"33", label %"34"

"33":                                             ; preds = %"32"
  store i32 2, ptr addrspace(5) %"99", align 4
  br label %"34"

"34":                                             ; preds = %"33", %"32"
  %"151" = load i1, ptr addrspace(5) %"100", align 1
  br i1 %"151", label %"36", label %"35"

"35":                                             ; preds = %"34"
  store i32 0, ptr addrspace(5) %"99", align 4
  br label %"36"

"36":                                             ; preds = %"35", %"34"
  %"153" = load i64, ptr addrspace(5) %"90", align 4
  %"166" = inttoptr i64 %"153" to ptr
  %"80" = getelementptr inbounds i8, ptr %"166", i64 12
  %"154" = load i32, ptr addrspace(5) %"99", align 4
  store i32 %"154", ptr %"80", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
