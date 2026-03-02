@values_g = addrspace(1) externally_initialized global [64 x i32] [i32 340, i32 122, i32 527, i32 693, i32 958, i32 394, i32 668, i32 432, i32 646, i32 354, i32 761, i32 449, i32 252, i32 778, i32 218, i32 800, i32 656, i32 493, i32 659, i32 787, i32 672, i32 203, i32 343, i32 845, i32 318, i32 286, i32 206, i32 253, i32 194, i32 489, i32 29, i32 323, i32 7, i32 619, i32 998, i32 930, i32 773, i32 749, i32 172, i32 465, i32 937, i32 96, i32 88, i32 621, i32 909, i32 298, i32 283, i32 286, i32 779, i32 290, i32 429, i32 930, i32 25, i32 687, i32 423, i32 200, i32 918, i32 10, i32 515, i32 248, i32 158, i32 911, i32 270, i32 459]
@values_s = external addrspace(3) global [64 x i32], align 16

declare hidden <2 x i32> @__zluda_ptx_impl_ldmatrix_m8n8_x2_b16(ptr addrspace(3)) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @ldmatrix(ptr addrspace(4) byref(i64) %"60") #1 {
  %"61" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i32, align 4, addrspace(5)
  %"63" = alloca i64, align 8, addrspace(5)
  %"64" = alloca i64, align 8, addrspace(5)
  %"65" = alloca i32, align 4, addrspace(5)
  %"66" = alloca i64, align 8, addrspace(5)
  %"67" = alloca i32, align 4, addrspace(5)
  %"68" = alloca i32, align 4, addrspace(5)
  %"69" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"58"

"58":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"60", align 8
  store i64 %2, ptr addrspace(5) %"61", align 8
  %"46" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"46", ptr addrspace(5) %"62", align 4
  %3 = load i32, ptr addrspace(5) %"62", align 4
  %4 = zext i32 %3 to i64
  store i64 %4, ptr addrspace(5) %"63", align 8
  store i64 ptrtoint (ptr addrspace(1) @values_g to i64), ptr addrspace(5) %"64", align 8
  %5 = load i64, ptr addrspace(5) %"63", align 8
  %6 = load i64, ptr addrspace(5) %"64", align 8
  %7 = mul i64 %5, 4
  %"75" = add i64 %7, %6
  store i64 %"75", ptr addrspace(5) %"64", align 8
  %8 = load i64, ptr addrspace(5) %"64", align 8
  %"111" = inttoptr i64 %8 to ptr addrspace(1)
  %9 = load i32, ptr addrspace(1) %"111", align 4
  store i32 %9, ptr addrspace(5) %"67", align 4
  store i32 ptrtoint (ptr addrspace(3) @values_s to i32), ptr addrspace(5) %"65", align 4
  %10 = load i32, ptr addrspace(5) %"62", align 4
  %11 = load i32, ptr addrspace(5) %"65", align 4
  %12 = mul i32 %10, 4
  %"113" = add i32 %12, %11
  store i32 %"113", ptr addrspace(5) %"65", align 4
  %13 = load i32, ptr addrspace(5) %"65", align 4
  %14 = load i32, ptr addrspace(5) %"67", align 4
  %"115" = inttoptr i32 %13 to ptr addrspace(3)
  store i32 %14, ptr addrspace(3) %"115", align 4
  %15 = load i64, ptr addrspace(5) %"64", align 8
  %"117" = inttoptr i64 %15 to ptr addrspace(1)
  %"50" = getelementptr inbounds i8, ptr addrspace(1) %"117", i64 128
  %16 = load i32, ptr addrspace(1) %"50", align 4
  store i32 %16, ptr addrspace(5) %"67", align 4
  %17 = load i32, ptr addrspace(5) %"65", align 4
  %"119" = inttoptr i32 %17 to ptr addrspace(3)
  %"52" = getelementptr inbounds i8, ptr addrspace(3) %"119", i64 128
  %18 = load i32, ptr addrspace(5) %"67", align 4
  store i32 %18, ptr addrspace(3) %"52", align 4
  store i64 ptrtoint (ptr addrspace(3) @values_s to i64), ptr addrspace(5) %"66", align 8
  %19 = load i64, ptr addrspace(5) %"66", align 8
  %20 = inttoptr i64 %19 to ptr addrspace(3)
  %"91" = addrspacecast ptr addrspace(3) %20 to ptr
  store ptr %"91", ptr addrspace(5) %"66", align 8
  %21 = load i64, ptr addrspace(5) %"63", align 8
  %22 = load i64, ptr addrspace(5) %"66", align 8
  %23 = mul i64 %21, 16
  %"122" = add i64 %23, %22
  store i64 %"122", ptr addrspace(5) %"66", align 8
  %24 = load i64, ptr addrspace(5) %"66", align 8
  %"124" = inttoptr i64 %24 to ptr addrspace(3)
  %"54" = call <2 x i32> @__zluda_ptx_impl_ldmatrix_m8n8_x2_b16(ptr addrspace(3) %"124")
  %"125" = extractelement <2 x i32> %"54", i8 0
  %"126" = extractelement <2 x i32> %"54", i8 1
  store i32 %"125", ptr addrspace(5) %"68", align 4
  store i32 %"126", ptr addrspace(5) %"69", align 4
  %25 = load i64, ptr addrspace(5) %"63", align 8
  %26 = load i64, ptr addrspace(5) %"61", align 8
  %27 = mul i64 %25, 8
  %"99" = add i64 %27, %26
  store i64 %"99", ptr addrspace(5) %"61", align 8
  %28 = load i64, ptr addrspace(5) %"61", align 8
  %29 = load i32, ptr addrspace(5) %"68", align 4
  %"127" = inttoptr i64 %28 to ptr
  store i32 %29, ptr %"127", align 4
  %30 = load i64, ptr addrspace(5) %"61", align 8
  %"128" = inttoptr i64 %30 to ptr
  %"57" = getelementptr inbounds i8, ptr %"128", i64 4
  %31 = load i32, ptr addrspace(5) %"69", align 4
  store i32 %31, ptr %"57", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
