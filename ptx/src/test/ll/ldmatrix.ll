@values_g = addrspace(1) global [64 x i32] [i32 340, i32 122, i32 527, i32 693, i32 958, i32 394, i32 668, i32 432, i32 646, i32 354, i32 761, i32 449, i32 252, i32 778, i32 218, i32 800, i32 656, i32 493, i32 659, i32 787, i32 672, i32 203, i32 343, i32 845, i32 318, i32 286, i32 206, i32 253, i32 194, i32 489, i32 29, i32 323, i32 7, i32 619, i32 998, i32 930, i32 773, i32 749, i32 172, i32 465, i32 937, i32 96, i32 88, i32 621, i32 909, i32 298, i32 283, i32 286, i32 779, i32 290, i32 429, i32 930, i32 25, i32 687, i32 423, i32 200, i32 918, i32 10, i32 515, i32 248, i32 158, i32 911, i32 270, i32 459]
@values_s = external addrspace(3) global [64 x i32], align 16

declare hidden <2 x i32> @__zluda_ptx_impl_ldmatrix_m8n8_x2_b16(ptr addrspace(3)) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @ldmatrix(ptr addrspace(4) byref(i64) %"55") #1 {
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i32, align 4, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i32, align 4, addrspace(5)
  %"63" = alloca i32, align 4, addrspace(5)
  %"64" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"52"

"52":                                             ; preds = %1
  %"65" = load i64, ptr addrspace(4) %"55", align 8
  store i64 %"65", ptr addrspace(5) %"56", align 8
  %"40" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"53"

"53":                                             ; preds = %"52"
  store i32 %"40", ptr addrspace(5) %"57", align 4
  %"68" = load i32, ptr addrspace(5) %"57", align 4
  %"67" = zext i32 %"68" to i64
  store i64 %"67", ptr addrspace(5) %"58", align 8
  store i64 ptrtoint (ptr addrspace(1) @values_g to i64), ptr addrspace(5) %"59", align 8
  %"71" = load i64, ptr addrspace(5) %"58", align 8
  %"72" = load i64, ptr addrspace(5) %"59", align 8
  %2 = mul i64 %"71", 4
  %"70" = add i64 %2, %"72"
  store i64 %"70", ptr addrspace(5) %"59", align 8
  %"74" = load i64, ptr addrspace(5) %"59", align 8
  %"106" = inttoptr i64 %"74" to ptr addrspace(1)
  %"105" = load i32, ptr addrspace(1) %"106", align 4
  store i32 %"105", ptr addrspace(5) %"62", align 4
  store i32 ptrtoint (ptr addrspace(3) @values_s to i32), ptr addrspace(5) %"60", align 4
  %"77" = load i32, ptr addrspace(5) %"57", align 4
  %"78" = load i32, ptr addrspace(5) %"60", align 4
  %3 = mul i32 %"77", 4
  %"108" = add i32 %3, %"78"
  store i32 %"108", ptr addrspace(5) %"60", align 4
  %"79" = load i32, ptr addrspace(5) %"60", align 4
  %"80" = load i32, ptr addrspace(5) %"62", align 4
  %"110" = inttoptr i32 %"79" to ptr addrspace(3)
  store i32 %"80", ptr addrspace(3) %"110", align 4
  %"81" = load i64, ptr addrspace(5) %"59", align 8
  %"112" = inttoptr i64 %"81" to ptr addrspace(1)
  %"44" = getelementptr inbounds i8, ptr addrspace(1) %"112", i64 128
  %"113" = load i32, ptr addrspace(1) %"44", align 4
  store i32 %"113", ptr addrspace(5) %"62", align 4
  %"83" = load i32, ptr addrspace(5) %"60", align 4
  %"114" = inttoptr i32 %"83" to ptr addrspace(3)
  %"46" = getelementptr inbounds i8, ptr addrspace(3) %"114", i64 128
  %"84" = load i32, ptr addrspace(5) %"62", align 4
  store i32 %"84", ptr addrspace(3) %"46", align 4
  store i64 ptrtoint (ptr addrspace(3) @values_s to i64), ptr addrspace(5) %"61", align 8
  %"87" = load i64, ptr addrspace(5) %"61", align 8
  %4 = inttoptr i64 %"87" to ptr addrspace(3)
  %"86" = addrspacecast ptr addrspace(3) %4 to ptr
  store ptr %"86", ptr addrspace(5) %"61", align 8
  %"89" = load i64, ptr addrspace(5) %"58", align 8
  %"90" = load i64, ptr addrspace(5) %"61", align 8
  %5 = mul i64 %"89", 16
  %"117" = add i64 %5, %"90"
  store i64 %"117", ptr addrspace(5) %"61", align 8
  %"91" = load i64, ptr addrspace(5) %"61", align 8
  %"119" = inttoptr i64 %"91" to ptr addrspace(3)
  %"48" = call <2 x i32> @__zluda_ptx_impl_ldmatrix_m8n8_x2_b16(ptr addrspace(3) %"119")
  %"120" = extractelement <2 x i32> %"48", i8 0
  %"121" = extractelement <2 x i32> %"48", i8 1
  store i32 %"120", ptr addrspace(5) %"63", align 4
  store i32 %"121", ptr addrspace(5) %"64", align 4
  %"95" = load i64, ptr addrspace(5) %"58", align 8
  %"96" = load i64, ptr addrspace(5) %"56", align 8
  %6 = mul i64 %"95", 8
  %"94" = add i64 %6, %"96"
  store i64 %"94", ptr addrspace(5) %"56", align 8
  %"97" = load i64, ptr addrspace(5) %"56", align 8
  %"98" = load i32, ptr addrspace(5) %"63", align 4
  %"122" = inttoptr i64 %"97" to ptr
  store i32 %"98", ptr %"122", align 4
  %"99" = load i64, ptr addrspace(5) %"56", align 8
  %"123" = inttoptr i64 %"99" to ptr
  %"51" = getelementptr inbounds i8, ptr %"123", i64 4
  %"100" = load i32, ptr addrspace(5) %"64", align 4
  store i32 %"100", ptr %"51", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
