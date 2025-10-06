@values_g = addrspace(1) global [64 x i32] [i32 340, i32 122, i32 527, i32 693, i32 958, i32 394, i32 668, i32 432, i32 646, i32 354, i32 761, i32 449, i32 252, i32 778, i32 218, i32 800, i32 656, i32 493, i32 659, i32 787, i32 672, i32 203, i32 343, i32 845, i32 318, i32 286, i32 206, i32 253, i32 194, i32 489, i32 29, i32 323, i32 7, i32 619, i32 998, i32 930, i32 773, i32 749, i32 172, i32 465, i32 937, i32 96, i32 88, i32 621, i32 909, i32 298, i32 283, i32 286, i32 779, i32 290, i32 429, i32 930, i32 25, i32 687, i32 423, i32 200, i32 918, i32 10, i32 515, i32 248, i32 158, i32 911, i32 270, i32 459]
@values_s = external addrspace(3) global [64 x i32], align 16

declare hidden <2 x i32> @__zluda_ptx_impl_ldmatrix_m8n8_x2_b16(ptr addrspace(3)) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @ldmatrix(ptr addrspace(4) byref(i64) %"58") #1 {
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i32, align 4, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i32, align 4, addrspace(5)
  %"64" = alloca i64, align 8, addrspace(5)
  %"65" = alloca i32, align 4, addrspace(5)
  %"66" = alloca i32, align 4, addrspace(5)
  %"67" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"55"

"55":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"58", align 8
  store i64 %2, ptr addrspace(5) %"59", align 8
  %"43" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"56"

"56":                                             ; preds = %"55"
  store i32 %"43", ptr addrspace(5) %"60", align 4
  %3 = load i32, ptr addrspace(5) %"60", align 4
  %"70" = zext i32 %3 to i64
  store i64 %"70", ptr addrspace(5) %"61", align 8
  store i64 ptrtoint (ptr addrspace(1) @values_g to i64), ptr addrspace(5) %"62", align 8
  %4 = load i64, ptr addrspace(5) %"61", align 8
  %5 = load i64, ptr addrspace(5) %"62", align 8
  %6 = mul i64 %4, 4
  %"73" = add i64 %6, %5
  store i64 %"73", ptr addrspace(5) %"62", align 8
  %7 = load i64, ptr addrspace(5) %"62", align 8
  %"109" = inttoptr i64 %7 to ptr addrspace(1)
  %8 = load i32, ptr addrspace(1) %"109", align 4
  store i32 %8, ptr addrspace(5) %"65", align 4
  store i32 ptrtoint (ptr addrspace(3) @values_s to i32), ptr addrspace(5) %"63", align 4
  %9 = load i32, ptr addrspace(5) %"60", align 4
  %10 = load i32, ptr addrspace(5) %"63", align 4
  %11 = mul i32 %9, 4
  %"111" = add i32 %11, %10
  store i32 %"111", ptr addrspace(5) %"63", align 4
  %12 = load i32, ptr addrspace(5) %"63", align 4
  %13 = load i32, ptr addrspace(5) %"65", align 4
  %"113" = inttoptr i32 %12 to ptr addrspace(3)
  store i32 %13, ptr addrspace(3) %"113", align 4
  %14 = load i64, ptr addrspace(5) %"62", align 8
  %"115" = inttoptr i64 %14 to ptr addrspace(1)
  %"47" = getelementptr inbounds i8, ptr addrspace(1) %"115", i64 128
  %15 = load i32, ptr addrspace(1) %"47", align 4
  store i32 %15, ptr addrspace(5) %"65", align 4
  %16 = load i32, ptr addrspace(5) %"63", align 4
  %"117" = inttoptr i32 %16 to ptr addrspace(3)
  %"49" = getelementptr inbounds i8, ptr addrspace(3) %"117", i64 128
  %17 = load i32, ptr addrspace(5) %"65", align 4
  store i32 %17, ptr addrspace(3) %"49", align 4
  store i64 ptrtoint (ptr addrspace(3) @values_s to i64), ptr addrspace(5) %"64", align 8
  %18 = load i64, ptr addrspace(5) %"64", align 8
  %19 = inttoptr i64 %18 to ptr addrspace(3)
  %"89" = addrspacecast ptr addrspace(3) %19 to ptr
  store ptr %"89", ptr addrspace(5) %"64", align 8
  %20 = load i64, ptr addrspace(5) %"61", align 8
  %21 = load i64, ptr addrspace(5) %"64", align 8
  %22 = mul i64 %20, 16
  %"120" = add i64 %22, %21
  store i64 %"120", ptr addrspace(5) %"64", align 8
  %23 = load i64, ptr addrspace(5) %"64", align 8
  %"122" = inttoptr i64 %23 to ptr addrspace(3)
  %"51" = call <2 x i32> @__zluda_ptx_impl_ldmatrix_m8n8_x2_b16(ptr addrspace(3) %"122")
  %"123" = extractelement <2 x i32> %"51", i8 0
  %"124" = extractelement <2 x i32> %"51", i8 1
  store i32 %"123", ptr addrspace(5) %"66", align 4
  store i32 %"124", ptr addrspace(5) %"67", align 4
  %24 = load i64, ptr addrspace(5) %"61", align 8
  %25 = load i64, ptr addrspace(5) %"59", align 8
  %26 = mul i64 %24, 8
  %"97" = add i64 %26, %25
  store i64 %"97", ptr addrspace(5) %"59", align 8
  %27 = load i64, ptr addrspace(5) %"59", align 8
  %28 = load i32, ptr addrspace(5) %"66", align 4
  %"125" = inttoptr i64 %27 to ptr
  store i32 %28, ptr %"125", align 4
  %29 = load i64, ptr addrspace(5) %"59", align 8
  %"126" = inttoptr i64 %29 to ptr
  %"54" = getelementptr inbounds i8, ptr %"126", i64 4
  %30 = load i32, ptr addrspace(5) %"67", align 4
  store i32 %30, ptr %"54", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }