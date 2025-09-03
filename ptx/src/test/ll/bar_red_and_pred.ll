declare hidden i1 @__zluda_ptx_impl_bar_red_and_pred(i32, i1, i1) #0

declare hidden i1 @__zluda_ptx_impl_bar_red_or_pred(i32, i1, i1) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @bar_red_and_pred(ptr addrspace(4) byref(i64) %"76", ptr addrspace(4) byref(i64) %"77") #1 {
  %"78" = alloca i64, align 8, addrspace(5)
  %"79" = alloca i64, align 8, addrspace(5)
  %"80" = alloca i32, align 4, addrspace(5)
  %"81" = alloca i32, align 4, addrspace(5)
  %"82" = alloca i1, align 1, addrspace(5)
  %"83" = alloca i1, align 1, addrspace(5)
  %"84" = alloca i32, align 4, addrspace(5)
  %"86" = alloca i8, align 1, addrspace(5)
  store i8 0, ptr addrspace(5) %"86", align 1
  %"89" = alloca i32, align 4, addrspace(5)
  store i32 2, ptr addrspace(5) %"89", align 4
  %"93" = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %"93", align 4
  %"97" = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %"97", align 4
  %"100" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"100", align 4
  %"101" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"101", align 1
  %"107" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"107", align 4
  %"111" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"111", align 4
  %"112" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"112", align 1
  %"118" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"118", align 4
  %"122" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"122", align 4
  %"123" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"123", align 4
  %"127" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"127", align 4
  %"128" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"128", align 1
  %"134" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"134", align 4
  %"138" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"138", align 4
  %"139" = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %"139", align 4
  %"143" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"143", align 4
  %"144" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"144", align 1
  %"150" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"150", align 4
  %"154" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"154", align 4
  %"155" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"155", align 4
  %"159" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"159", align 4
  %"160" = alloca i1, align 1, addrspace(5)
  store i1 true, ptr addrspace(5) %"160", align 1
  %"166" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"166", align 4
  %"172" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"172", align 4
  br label %1

1:                                                ; preds = %0
  br label %"73"

"73":                                             ; preds = %1
  %"85" = load i64, ptr addrspace(4) %"77", align 8
  store i64 %"85", ptr addrspace(5) %"78", align 8
  %"87" = load i8, ptr addrspace(5) %"86", align 1
  %"47" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"87")
  br label %"74"

"74":                                             ; preds = %"73"
  store i32 %"47", ptr addrspace(5) %"80", align 4
  %"91" = load i32, ptr addrspace(5) %"80", align 4
  %"92" = load i32, ptr addrspace(5) %"89", align 4
  %"90" = urem i32 %"91", %"92"
  store i32 %"90", ptr addrspace(5) %"81", align 4
  %"95" = load i32, ptr addrspace(5) %"81", align 4
  %"96" = load i32, ptr addrspace(5) %"93", align 4
  %2 = icmp eq i32 %"95", %"96"
  store i1 %2, ptr addrspace(5) %"83", align 1
  %"99" = load i32, ptr addrspace(5) %"97", align 4
  store i32 %"99", ptr addrspace(5) %"84", align 4
  %"103" = load i32, ptr addrspace(5) %"100", align 4
  %"104" = load i1, ptr addrspace(5) %"83", align 1
  %"105" = load i1, ptr addrspace(5) %"101", align 1
  %"102" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 %"103", i1 %"104", i1 %"105")
  store i1 %"102", ptr addrspace(5) %"82", align 1
  %"106" = load i1, ptr addrspace(5) %"82", align 1
  br i1 %"106", label %"18", label %"19"

"18":                                             ; preds = %"74"
  %"109" = load i32, ptr addrspace(5) %"84", align 4
  %"110" = load i32, ptr addrspace(5) %"107", align 4
  %"108" = add i32 %"109", %"110"
  store i32 %"108", ptr addrspace(5) %"84", align 4
  br label %"19"

"19":                                             ; preds = %"18", %"74"
  %"114" = load i32, ptr addrspace(5) %"111", align 4
  %"115" = load i1, ptr addrspace(5) %"83", align 1
  %"116" = load i1, ptr addrspace(5) %"112", align 1
  %"113" = call i1 @__zluda_ptx_impl_bar_red_or_pred(i32 %"114", i1 %"115", i1 %"116")
  store i1 %"113", ptr addrspace(5) %"82", align 1
  %"117" = load i1, ptr addrspace(5) %"82", align 1
  br i1 %"117", label %"20", label %"21"

"20":                                             ; preds = %"19"
  %"120" = load i32, ptr addrspace(5) %"84", align 4
  %"121" = load i32, ptr addrspace(5) %"118", align 4
  %"119" = add i32 %"120", %"121"
  store i32 %"119", ptr addrspace(5) %"84", align 4
  br label %"21"

"21":                                             ; preds = %"20", %"19"
  %"125" = load i32, ptr addrspace(5) %"122", align 4
  %"126" = load i32, ptr addrspace(5) %"123", align 4
  %3 = icmp eq i32 %"125", %"126"
  store i1 %3, ptr addrspace(5) %"83", align 1
  %"130" = load i32, ptr addrspace(5) %"127", align 4
  %"131" = load i1, ptr addrspace(5) %"83", align 1
  %"132" = load i1, ptr addrspace(5) %"128", align 1
  %"129" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 %"130", i1 %"131", i1 %"132")
  store i1 %"129", ptr addrspace(5) %"82", align 1
  %"133" = load i1, ptr addrspace(5) %"82", align 1
  br i1 %"133", label %"22", label %"23"

"22":                                             ; preds = %"21"
  %"136" = load i32, ptr addrspace(5) %"84", align 4
  %"137" = load i32, ptr addrspace(5) %"134", align 4
  %"135" = add i32 %"136", %"137"
  store i32 %"135", ptr addrspace(5) %"84", align 4
  br label %"23"

"23":                                             ; preds = %"22", %"21"
  %"141" = load i32, ptr addrspace(5) %"138", align 4
  %"142" = load i32, ptr addrspace(5) %"139", align 4
  %4 = icmp eq i32 %"141", %"142"
  store i1 %4, ptr addrspace(5) %"83", align 1
  %"146" = load i32, ptr addrspace(5) %"143", align 4
  %"147" = load i1, ptr addrspace(5) %"83", align 1
  %"148" = load i1, ptr addrspace(5) %"144", align 1
  %"145" = call i1 @__zluda_ptx_impl_bar_red_or_pred(i32 %"146", i1 %"147", i1 %"148")
  store i1 %"145", ptr addrspace(5) %"82", align 1
  %"149" = load i1, ptr addrspace(5) %"82", align 1
  br i1 %"149", label %"24", label %"25"

"24":                                             ; preds = %"23"
  %"152" = load i32, ptr addrspace(5) %"84", align 4
  %"153" = load i32, ptr addrspace(5) %"150", align 4
  %"151" = add i32 %"152", %"153"
  store i32 %"151", ptr addrspace(5) %"84", align 4
  br label %"25"

"25":                                             ; preds = %"24", %"23"
  %"157" = load i32, ptr addrspace(5) %"154", align 4
  %"158" = load i32, ptr addrspace(5) %"155", align 4
  %5 = icmp eq i32 %"157", %"158"
  store i1 %5, ptr addrspace(5) %"83", align 1
  %"162" = load i32, ptr addrspace(5) %"159", align 4
  %"163" = load i1, ptr addrspace(5) %"83", align 1
  %"164" = load i1, ptr addrspace(5) %"160", align 1
  %"161" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 %"162", i1 %"163", i1 %"164")
  store i1 %"161", ptr addrspace(5) %"82", align 1
  %"165" = load i1, ptr addrspace(5) %"82", align 1
  br i1 %"165", label %"26", label %"27"

"26":                                             ; preds = %"25"
  %"168" = load i32, ptr addrspace(5) %"84", align 4
  %"169" = load i32, ptr addrspace(5) %"166", align 4
  %"167" = add i32 %"168", %"169"
  store i32 %"167", ptr addrspace(5) %"84", align 4
  br label %"27"

"27":                                             ; preds = %"26", %"25"
  %"171" = load i32, ptr addrspace(5) %"80", align 4
  %"170" = zext i32 %"171" to i64
  store i64 %"170", ptr addrspace(5) %"79", align 8
  %"174" = load i64, ptr addrspace(5) %"79", align 8
  %"175" = load i64, ptr addrspace(5) %"172", align 8
  %"173" = mul i64 %"174", %"175"
  store i64 %"173", ptr addrspace(5) %"79", align 8
  %"177" = load i64, ptr addrspace(5) %"78", align 8
  %"178" = load i64, ptr addrspace(5) %"79", align 8
  %"176" = add i64 %"177", %"178"
  store i64 %"176", ptr addrspace(5) %"78", align 8
  %"179" = load i64, ptr addrspace(5) %"78", align 8
  %"180" = load i32, ptr addrspace(5) %"84", align 4
  %"181" = inttoptr i64 %"179" to ptr
  store i32 %"180", ptr %"181", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }