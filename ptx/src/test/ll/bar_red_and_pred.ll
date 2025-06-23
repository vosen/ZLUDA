declare i1 @__zluda_ptx_impl_bar_red_or(i32, i1, i64) #0

declare i1 @__zluda_ptx_impl_bar_red_and(i32, i1, i64) #0

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @bar_red_and_pred(ptr addrspace(4) byref(i64) %"73", ptr addrspace(4) byref(i64) %"74") #1 {
  %"75" = alloca i64, align 8, addrspace(5)
  %"76" = alloca i64, align 8, addrspace(5)
  %"77" = alloca i32, align 4, addrspace(5)
  %"78" = alloca i32, align 4, addrspace(5)
  %"79" = alloca i1, align 1, addrspace(5)
  %"80" = alloca i1, align 1, addrspace(5)
  %"81" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"70"

"70":                                             ; preds = %1
  %"82" = load i64, ptr addrspace(4) %"74", align 4
  store i64 %"82", ptr addrspace(5) %"75", align 4
  %"44" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"71"

"71":                                             ; preds = %"70"
  store i32 %"44", ptr addrspace(5) %"77", align 4
  %"85" = load i32, ptr addrspace(5) %"77", align 4
  %"84" = urem i32 %"85", 2
  store i32 %"84", ptr addrspace(5) %"78", align 4
  %"87" = load i32, ptr addrspace(5) %"78", align 4
  %"86" = icmp eq i32 %"87", 0
  store i1 %"86", ptr addrspace(5) %"80", align 1
  store i32 0, ptr addrspace(5) %"81", align 4
  %"90" = load i1, ptr addrspace(5) %"80", align 1
  %"89" = call i1 @__zluda_ptx_impl_bar_red_and(i32 1, i1 %"90", i64 0)
  store i1 %"89", ptr addrspace(5) %"79", align 1
  %"91" = load i1, ptr addrspace(5) %"79", align 1
  br i1 %"91", label %"17", label %"18"

"17":                                             ; preds = %"71"
  %"93" = load i32, ptr addrspace(5) %"81", align 4
  %"92" = add i32 %"93", 1
  store i32 %"92", ptr addrspace(5) %"81", align 4
  br label %"18"

"18":                                             ; preds = %"17", %"71"
  %"95" = load i1, ptr addrspace(5) %"80", align 1
  %"94" = call i1 @__zluda_ptx_impl_bar_red_or(i32 1, i1 %"95", i64 0)
  store i1 %"94", ptr addrspace(5) %"79", align 1
  %"96" = load i1, ptr addrspace(5) %"79", align 1
  br i1 %"96", label %"19", label %"20"

"19":                                             ; preds = %"18"
  %"98" = load i32, ptr addrspace(5) %"81", align 4
  %"97" = add i32 %"98", 1
  store i32 %"97", ptr addrspace(5) %"81", align 4
  br label %"20"

"20":                                             ; preds = %"19", %"18"
  store i1 true, ptr addrspace(5) %"80", align 1
  %"101" = load i1, ptr addrspace(5) %"80", align 1
  %"100" = call i1 @__zluda_ptx_impl_bar_red_and(i32 1, i1 %"101", i64 0)
  store i1 %"100", ptr addrspace(5) %"79", align 1
  %"102" = load i1, ptr addrspace(5) %"79", align 1
  br i1 %"102", label %"21", label %"22"

"21":                                             ; preds = %"20"
  %"104" = load i32, ptr addrspace(5) %"81", align 4
  %"103" = add i32 %"104", 1
  store i32 %"103", ptr addrspace(5) %"81", align 4
  br label %"22"

"22":                                             ; preds = %"21", %"20"
  store i1 false, ptr addrspace(5) %"80", align 1
  %"107" = load i1, ptr addrspace(5) %"80", align 1
  %"106" = call i1 @__zluda_ptx_impl_bar_red_or(i32 1, i1 %"107", i64 0)
  store i1 %"106", ptr addrspace(5) %"79", align 1
  %"108" = load i1, ptr addrspace(5) %"79", align 1
  br i1 %"108", label %"23", label %"24"

"23":                                             ; preds = %"22"
  %"110" = load i32, ptr addrspace(5) %"81", align 4
  %"109" = add i32 %"110", 1
  store i32 %"109", ptr addrspace(5) %"81", align 4
  br label %"24"

"24":                                             ; preds = %"23", %"22"
  store i1 true, ptr addrspace(5) %"80", align 1
  %"113" = load i1, ptr addrspace(5) %"80", align 1
  %"112" = call i1 @__zluda_ptx_impl_bar_red_and(i32 1, i1 %"113", i64 1)
  store i1 %"112", ptr addrspace(5) %"79", align 1
  %"114" = load i1, ptr addrspace(5) %"79", align 1
  br i1 %"114", label %"25", label %"26"

"25":                                             ; preds = %"24"
  %"116" = load i32, ptr addrspace(5) %"81", align 4
  %"115" = add i32 %"116", 1
  store i32 %"115", ptr addrspace(5) %"81", align 4
  br label %"26"

"26":                                             ; preds = %"25", %"24"
  %"118" = load i32, ptr addrspace(5) %"77", align 4
  %"117" = zext i32 %"118" to i64
  store i64 %"117", ptr addrspace(5) %"76", align 4
  %"120" = load i64, ptr addrspace(5) %"76", align 4
  %"119" = mul i64 %"120", 4
  store i64 %"119", ptr addrspace(5) %"76", align 4
  %"122" = load i64, ptr addrspace(5) %"75", align 4
  %"123" = load i64, ptr addrspace(5) %"76", align 4
  %"121" = add i64 %"122", %"123"
  store i64 %"121", ptr addrspace(5) %"75", align 4
  %"124" = load i64, ptr addrspace(5) %"75", align 4
  %"125" = load i32, ptr addrspace(5) %"81", align 4
  %"126" = inttoptr i64 %"124" to ptr
  store i32 %"125", ptr %"126", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }