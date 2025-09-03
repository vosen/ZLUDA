@0 = addrspace(4) global i8 0
@1 = addrspace(4) global i32 2
@2 = addrspace(4) global i32 0
@3 = addrspace(4) global i32 0
@4 = addrspace(4) global i32 1
@5 = addrspace(4) global i1 false
@6 = addrspace(4) global i32 1
@7 = addrspace(4) global i32 1
@8 = addrspace(4) global i1 false
@9 = addrspace(4) global i32 1
@10 = addrspace(4) global i32 1
@11 = addrspace(4) global i32 1
@12 = addrspace(4) global i32 1
@13 = addrspace(4) global i1 false
@14 = addrspace(4) global i32 1
@15 = addrspace(4) global i32 1
@16 = addrspace(4) global i32 0
@17 = addrspace(4) global i32 1
@18 = addrspace(4) global i1 false
@19 = addrspace(4) global i32 1
@20 = addrspace(4) global i32 1
@21 = addrspace(4) global i32 1
@22 = addrspace(4) global i32 1
@23 = addrspace(4) global i1 true
@24 = addrspace(4) global i32 1
@25 = addrspace(4) global i64 4

declare hidden i1 @__zluda_ptx_impl_bar_red_and_pred(i32, i1, i1) #0

declare hidden i1 @__zluda_ptx_impl_bar_red_or_pred(i32, i1, i1) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @bar_red_and_pred(ptr addrspace(4) byref(i64) %"102", ptr addrspace(4) byref(i64) %"103") #1 {
  %"104" = alloca i64, align 8, addrspace(5)
  %"105" = alloca i64, align 8, addrspace(5)
  %"106" = alloca i32, align 4, addrspace(5)
  %"107" = alloca i32, align 4, addrspace(5)
  %"108" = alloca i1, align 1, addrspace(5)
  %"109" = alloca i1, align 1, addrspace(5)
  %"110" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"99"

"99":                                             ; preds = %1
  %"111" = load i64, ptr addrspace(4) %"103", align 8
  store i64 %"111", ptr addrspace(5) %"104", align 8
  %"47" = load i8, ptr addrspace(4) @0, align 1
  %"48" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"47")
  br label %"100"

"100":                                            ; preds = %"99"
  store i32 %"48", ptr addrspace(5) %"106", align 4
  %"50" = load i32, ptr addrspace(4) @1, align 4
  %"114" = load i32, ptr addrspace(5) %"106", align 4
  %"113" = urem i32 %"114", %"50"
  store i32 %"113", ptr addrspace(5) %"107", align 4
  %"52" = load i32, ptr addrspace(4) @2, align 4
  %"116" = load i32, ptr addrspace(5) %"107", align 4
  %2 = icmp eq i32 %"116", %"52"
  store i1 %2, ptr addrspace(5) %"109", align 1
  %"54" = load i32, ptr addrspace(4) @3, align 4
  store i32 %"54", ptr addrspace(5) %"110", align 4
  %"56" = load i32, ptr addrspace(4) @4, align 4
  %"58" = load i1, ptr addrspace(4) @5, align 1
  %"119" = load i1, ptr addrspace(5) %"109", align 1
  %"118" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 %"56", i1 %"119", i1 %"58")
  store i1 %"118", ptr addrspace(5) %"108", align 1
  %"120" = load i1, ptr addrspace(5) %"108", align 1
  br i1 %"120", label %"18", label %"19"

"18":                                             ; preds = %"100"
  %"60" = load i32, ptr addrspace(4) @6, align 4
  %"122" = load i32, ptr addrspace(5) %"110", align 4
  %"121" = add i32 %"122", %"60"
  store i32 %"121", ptr addrspace(5) %"110", align 4
  br label %"19"

"19":                                             ; preds = %"18", %"100"
  %"62" = load i32, ptr addrspace(4) @7, align 4
  %"64" = load i1, ptr addrspace(4) @8, align 1
  %"124" = load i1, ptr addrspace(5) %"109", align 1
  %"123" = call i1 @__zluda_ptx_impl_bar_red_or_pred(i32 %"62", i1 %"124", i1 %"64")
  store i1 %"123", ptr addrspace(5) %"108", align 1
  %"125" = load i1, ptr addrspace(5) %"108", align 1
  br i1 %"125", label %"20", label %"21"

"20":                                             ; preds = %"19"
  %"66" = load i32, ptr addrspace(4) @9, align 4
  %"127" = load i32, ptr addrspace(5) %"110", align 4
  %"126" = add i32 %"127", %"66"
  store i32 %"126", ptr addrspace(5) %"110", align 4
  br label %"21"

"21":                                             ; preds = %"20", %"19"
  %"68" = load i32, ptr addrspace(4) @10, align 4
  %"70" = load i32, ptr addrspace(4) @11, align 4
  %3 = icmp eq i32 %"68", %"70"
  store i1 %3, ptr addrspace(5) %"109", align 1
  %"72" = load i32, ptr addrspace(4) @12, align 4
  %"74" = load i1, ptr addrspace(4) @13, align 1
  %"130" = load i1, ptr addrspace(5) %"109", align 1
  %"129" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 %"72", i1 %"130", i1 %"74")
  store i1 %"129", ptr addrspace(5) %"108", align 1
  %"131" = load i1, ptr addrspace(5) %"108", align 1
  br i1 %"131", label %"22", label %"23"

"22":                                             ; preds = %"21"
  %"76" = load i32, ptr addrspace(4) @14, align 4
  %"133" = load i32, ptr addrspace(5) %"110", align 4
  %"132" = add i32 %"133", %"76"
  store i32 %"132", ptr addrspace(5) %"110", align 4
  br label %"23"

"23":                                             ; preds = %"22", %"21"
  %"78" = load i32, ptr addrspace(4) @15, align 4
  %"80" = load i32, ptr addrspace(4) @16, align 4
  %4 = icmp eq i32 %"78", %"80"
  store i1 %4, ptr addrspace(5) %"109", align 1
  %"82" = load i32, ptr addrspace(4) @17, align 4
  %"84" = load i1, ptr addrspace(4) @18, align 1
  %"136" = load i1, ptr addrspace(5) %"109", align 1
  %"135" = call i1 @__zluda_ptx_impl_bar_red_or_pred(i32 %"82", i1 %"136", i1 %"84")
  store i1 %"135", ptr addrspace(5) %"108", align 1
  %"137" = load i1, ptr addrspace(5) %"108", align 1
  br i1 %"137", label %"24", label %"25"

"24":                                             ; preds = %"23"
  %"86" = load i32, ptr addrspace(4) @19, align 4
  %"139" = load i32, ptr addrspace(5) %"110", align 4
  %"138" = add i32 %"139", %"86"
  store i32 %"138", ptr addrspace(5) %"110", align 4
  br label %"25"

"25":                                             ; preds = %"24", %"23"
  %"88" = load i32, ptr addrspace(4) @20, align 4
  %"90" = load i32, ptr addrspace(4) @21, align 4
  %5 = icmp eq i32 %"88", %"90"
  store i1 %5, ptr addrspace(5) %"109", align 1
  %"92" = load i32, ptr addrspace(4) @22, align 4
  %"94" = load i1, ptr addrspace(4) @23, align 1
  %"142" = load i1, ptr addrspace(5) %"109", align 1
  %"141" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 %"92", i1 %"142", i1 %"94")
  store i1 %"141", ptr addrspace(5) %"108", align 1
  %"143" = load i1, ptr addrspace(5) %"108", align 1
  br i1 %"143", label %"26", label %"27"

"26":                                             ; preds = %"25"
  %"96" = load i32, ptr addrspace(4) @24, align 4
  %"145" = load i32, ptr addrspace(5) %"110", align 4
  %"144" = add i32 %"145", %"96"
  store i32 %"144", ptr addrspace(5) %"110", align 4
  br label %"27"

"27":                                             ; preds = %"26", %"25"
  %"147" = load i32, ptr addrspace(5) %"106", align 4
  %"146" = zext i32 %"147" to i64
  store i64 %"146", ptr addrspace(5) %"105", align 8
  %"98" = load i64, ptr addrspace(4) @25, align 8
  %"149" = load i64, ptr addrspace(5) %"105", align 8
  %"148" = mul i64 %"149", %"98"
  store i64 %"148", ptr addrspace(5) %"105", align 8
  %"151" = load i64, ptr addrspace(5) %"104", align 8
  %"152" = load i64, ptr addrspace(5) %"105", align 8
  %"150" = add i64 %"151", %"152"
  store i64 %"150", ptr addrspace(5) %"104", align 8
  %"153" = load i64, ptr addrspace(5) %"104", align 8
  %"154" = load i32, ptr addrspace(5) %"110", align 4
  %"155" = inttoptr i64 %"153" to ptr
  store i32 %"154", ptr %"155", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }