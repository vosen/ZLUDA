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

define amdgpu_kernel void @bar_red_and_pred(ptr addrspace(4) byref(i64) %"101", ptr addrspace(4) byref(i64) %"102") #1 {
  %"103" = alloca i64, align 8, addrspace(5)
  %"104" = alloca i64, align 8, addrspace(5)
  %"105" = alloca i32, align 4, addrspace(5)
  %"106" = alloca i32, align 4, addrspace(5)
  %"107" = alloca i1, align 1, addrspace(5)
  %"108" = alloca i1, align 1, addrspace(5)
  %"109" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"98"

"98":                                             ; preds = %1
  %"110" = load i64, ptr addrspace(4) %"102", align 8
  store i64 %"110", ptr addrspace(5) %"103", align 8
  %"47" = call i32 @__zluda_ptx_impl_sreg_tid(i8 ptrtoint (ptr addrspace(4) @0 to i8))
  br label %"99"

"99":                                             ; preds = %"98"
  store i32 %"47", ptr addrspace(5) %"105", align 4
  %"49" = load i32, ptr addrspace(4) @1, align 4
  %"113" = load i32, ptr addrspace(5) %"105", align 4
  %"112" = urem i32 %"113", %"49"
  store i32 %"112", ptr addrspace(5) %"106", align 4
  %"51" = load i32, ptr addrspace(4) @2, align 4
  %"115" = load i32, ptr addrspace(5) %"106", align 4
  %2 = icmp eq i32 %"115", %"51"
  store i1 %2, ptr addrspace(5) %"108", align 1
  %"53" = load i32, ptr addrspace(4) @3, align 4
  store i32 %"53", ptr addrspace(5) %"109", align 4
  %"55" = load i32, ptr addrspace(4) @4, align 4
  %"57" = load i1, ptr addrspace(4) @5, align 1
  %"118" = load i1, ptr addrspace(5) %"108", align 1
  %"117" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 %"55", i1 %"118", i1 %"57")
  store i1 %"117", ptr addrspace(5) %"107", align 1
  %"119" = load i1, ptr addrspace(5) %"107", align 1
  br i1 %"119", label %"18", label %"19"

"18":                                             ; preds = %"99"
  %"59" = load i32, ptr addrspace(4) @6, align 4
  %"121" = load i32, ptr addrspace(5) %"109", align 4
  %"120" = add i32 %"121", %"59"
  store i32 %"120", ptr addrspace(5) %"109", align 4
  br label %"19"

"19":                                             ; preds = %"18", %"99"
  %"61" = load i32, ptr addrspace(4) @7, align 4
  %"63" = load i1, ptr addrspace(4) @8, align 1
  %"123" = load i1, ptr addrspace(5) %"108", align 1
  %"122" = call i1 @__zluda_ptx_impl_bar_red_or_pred(i32 %"61", i1 %"123", i1 %"63")
  store i1 %"122", ptr addrspace(5) %"107", align 1
  %"124" = load i1, ptr addrspace(5) %"107", align 1
  br i1 %"124", label %"20", label %"21"

"20":                                             ; preds = %"19"
  %"65" = load i32, ptr addrspace(4) @9, align 4
  %"126" = load i32, ptr addrspace(5) %"109", align 4
  %"125" = add i32 %"126", %"65"
  store i32 %"125", ptr addrspace(5) %"109", align 4
  br label %"21"

"21":                                             ; preds = %"20", %"19"
  %"67" = load i32, ptr addrspace(4) @10, align 4
  %"69" = load i32, ptr addrspace(4) @11, align 4
  %3 = icmp eq i32 %"67", %"69"
  store i1 %3, ptr addrspace(5) %"108", align 1
  %"71" = load i32, ptr addrspace(4) @12, align 4
  %"73" = load i1, ptr addrspace(4) @13, align 1
  %"129" = load i1, ptr addrspace(5) %"108", align 1
  %"128" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 %"71", i1 %"129", i1 %"73")
  store i1 %"128", ptr addrspace(5) %"107", align 1
  %"130" = load i1, ptr addrspace(5) %"107", align 1
  br i1 %"130", label %"22", label %"23"

"22":                                             ; preds = %"21"
  %"75" = load i32, ptr addrspace(4) @14, align 4
  %"132" = load i32, ptr addrspace(5) %"109", align 4
  %"131" = add i32 %"132", %"75"
  store i32 %"131", ptr addrspace(5) %"109", align 4
  br label %"23"

"23":                                             ; preds = %"22", %"21"
  %"77" = load i32, ptr addrspace(4) @15, align 4
  %"79" = load i32, ptr addrspace(4) @16, align 4
  %4 = icmp eq i32 %"77", %"79"
  store i1 %4, ptr addrspace(5) %"108", align 1
  %"81" = load i32, ptr addrspace(4) @17, align 4
  %"83" = load i1, ptr addrspace(4) @18, align 1
  %"135" = load i1, ptr addrspace(5) %"108", align 1
  %"134" = call i1 @__zluda_ptx_impl_bar_red_or_pred(i32 %"81", i1 %"135", i1 %"83")
  store i1 %"134", ptr addrspace(5) %"107", align 1
  %"136" = load i1, ptr addrspace(5) %"107", align 1
  br i1 %"136", label %"24", label %"25"

"24":                                             ; preds = %"23"
  %"85" = load i32, ptr addrspace(4) @19, align 4
  %"138" = load i32, ptr addrspace(5) %"109", align 4
  %"137" = add i32 %"138", %"85"
  store i32 %"137", ptr addrspace(5) %"109", align 4
  br label %"25"

"25":                                             ; preds = %"24", %"23"
  %"87" = load i32, ptr addrspace(4) @20, align 4
  %"89" = load i32, ptr addrspace(4) @21, align 4
  %5 = icmp eq i32 %"87", %"89"
  store i1 %5, ptr addrspace(5) %"108", align 1
  %"91" = load i32, ptr addrspace(4) @22, align 4
  %"93" = load i1, ptr addrspace(4) @23, align 1
  %"141" = load i1, ptr addrspace(5) %"108", align 1
  %"140" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 %"91", i1 %"141", i1 %"93")
  store i1 %"140", ptr addrspace(5) %"107", align 1
  %"142" = load i1, ptr addrspace(5) %"107", align 1
  br i1 %"142", label %"26", label %"27"

"26":                                             ; preds = %"25"
  %"95" = load i32, ptr addrspace(4) @24, align 4
  %"144" = load i32, ptr addrspace(5) %"109", align 4
  %"143" = add i32 %"144", %"95"
  store i32 %"143", ptr addrspace(5) %"109", align 4
  br label %"27"

"27":                                             ; preds = %"26", %"25"
  %"146" = load i32, ptr addrspace(5) %"105", align 4
  %"145" = zext i32 %"146" to i64
  store i64 %"145", ptr addrspace(5) %"104", align 8
  %"97" = load i64, ptr addrspace(4) @25, align 8
  %"148" = load i64, ptr addrspace(5) %"104", align 8
  %"147" = mul i64 %"148", %"97"
  store i64 %"147", ptr addrspace(5) %"104", align 8
  %"150" = load i64, ptr addrspace(5) %"103", align 8
  %"151" = load i64, ptr addrspace(5) %"104", align 8
  %"149" = add i64 %"150", %"151"
  store i64 %"149", ptr addrspace(5) %"103", align 8
  %"152" = load i64, ptr addrspace(5) %"103", align 8
  %"153" = load i32, ptr addrspace(5) %"109", align 4
  %"155" = inttoptr i64 %"152" to ptr
  store i32 %"153", ptr %"155", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }