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
  br label %1

1:                                                ; preds = %0
  br label %"73"

"73":                                             ; preds = %1
  %"85" = load i64, ptr addrspace(4) %"77", align 8
  store i64 %"85", ptr addrspace(5) %"78", align 8
  %"47" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"74"

"74":                                             ; preds = %"73"
  store i32 %"47", ptr addrspace(5) %"80", align 4
  %"88" = load i32, ptr addrspace(5) %"80", align 4
  %"87" = urem i32 %"88", 2
  store i32 %"87", ptr addrspace(5) %"81", align 4
  %"90" = load i32, ptr addrspace(5) %"81", align 4
  %2 = icmp eq i32 %"90", 0
  store i1 %2, ptr addrspace(5) %"83", align 1
  store i32 0, ptr addrspace(5) %"84", align 4
  %"93" = load i1, ptr addrspace(5) %"83", align 1
  %"92" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 1, i1 %"93", i1 false)
  store i1 %"92", ptr addrspace(5) %"82", align 1
  %"94" = load i1, ptr addrspace(5) %"82", align 1
  br i1 %"94", label %"18", label %"19"

"18":                                             ; preds = %"74"
  %"96" = load i32, ptr addrspace(5) %"84", align 4
  %"95" = add i32 %"96", 1
  store i32 %"95", ptr addrspace(5) %"84", align 4
  br label %"19"

"19":                                             ; preds = %"18", %"74"
  %"98" = load i1, ptr addrspace(5) %"83", align 1
  %"97" = call i1 @__zluda_ptx_impl_bar_red_or_pred(i32 1, i1 %"98", i1 false)
  store i1 %"97", ptr addrspace(5) %"82", align 1
  %"99" = load i1, ptr addrspace(5) %"82", align 1
  br i1 %"99", label %"20", label %"21"

"20":                                             ; preds = %"19"
  %"101" = load i32, ptr addrspace(5) %"84", align 4
  %"100" = add i32 %"101", 1
  store i32 %"100", ptr addrspace(5) %"84", align 4
  br label %"21"

"21":                                             ; preds = %"20", %"19"
  store i1 true, ptr addrspace(5) %"83", align 1
  %"104" = load i1, ptr addrspace(5) %"83", align 1
  %"103" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 1, i1 %"104", i1 false)
  store i1 %"103", ptr addrspace(5) %"82", align 1
  %"105" = load i1, ptr addrspace(5) %"82", align 1
  br i1 %"105", label %"22", label %"23"

"22":                                             ; preds = %"21"
  %"107" = load i32, ptr addrspace(5) %"84", align 4
  %"106" = add i32 %"107", 1
  store i32 %"106", ptr addrspace(5) %"84", align 4
  br label %"23"

"23":                                             ; preds = %"22", %"21"
  store i1 false, ptr addrspace(5) %"83", align 1
  %"110" = load i1, ptr addrspace(5) %"83", align 1
  %"109" = call i1 @__zluda_ptx_impl_bar_red_or_pred(i32 1, i1 %"110", i1 false)
  store i1 %"109", ptr addrspace(5) %"82", align 1
  %"111" = load i1, ptr addrspace(5) %"82", align 1
  br i1 %"111", label %"24", label %"25"

"24":                                             ; preds = %"23"
  %"113" = load i32, ptr addrspace(5) %"84", align 4
  %"112" = add i32 %"113", 1
  store i32 %"112", ptr addrspace(5) %"84", align 4
  br label %"25"

"25":                                             ; preds = %"24", %"23"
  store i1 true, ptr addrspace(5) %"83", align 1
  %"116" = load i1, ptr addrspace(5) %"83", align 1
  %"115" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 1, i1 %"116", i1 true)
  store i1 %"115", ptr addrspace(5) %"82", align 1
  %"117" = load i1, ptr addrspace(5) %"82", align 1
  br i1 %"117", label %"26", label %"27"

"26":                                             ; preds = %"25"
  %"119" = load i32, ptr addrspace(5) %"84", align 4
  %"118" = add i32 %"119", 1
  store i32 %"118", ptr addrspace(5) %"84", align 4
  br label %"27"

"27":                                             ; preds = %"26", %"25"
  %"121" = load i32, ptr addrspace(5) %"80", align 4
  %"120" = zext i32 %"121" to i64
  store i64 %"120", ptr addrspace(5) %"79", align 8
  %"123" = load i64, ptr addrspace(5) %"79", align 8
  %"122" = mul i64 %"123", 4
  store i64 %"122", ptr addrspace(5) %"79", align 8
  %"125" = load i64, ptr addrspace(5) %"78", align 8
  %"126" = load i64, ptr addrspace(5) %"79", align 8
  %"124" = add i64 %"125", %"126"
  store i64 %"124", ptr addrspace(5) %"78", align 8
  %"127" = load i64, ptr addrspace(5) %"78", align 8
  %"128" = load i32, ptr addrspace(5) %"84", align 4
  %"129" = inttoptr i64 %"127" to ptr
  store i32 %"128", ptr %"129", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
