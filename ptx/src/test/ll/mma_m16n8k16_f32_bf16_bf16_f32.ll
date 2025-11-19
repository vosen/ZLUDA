declare hidden <4 x float> @__zluda_ptx_impl_mma_sync_aligned_m16n8k16_row_col_f32_bf16_bf16_f32(<4 x i32>, <2 x i32>, <4 x float>) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @mma_m16n8k16_f32_bf16_bf16_f32(ptr addrspace(4) byref(i64) %"75") #1 {
  %"76" = alloca i64, align 8, addrspace(5)
  %"77" = alloca i64, align 8, addrspace(5)
  %"78" = alloca i32, align 4, addrspace(5)
  %"79" = alloca float, align 4, addrspace(5)
  %"80" = alloca float, align 4, addrspace(5)
  %"81" = alloca float, align 4, addrspace(5)
  %"82" = alloca float, align 4, addrspace(5)
  %"83" = alloca float, align 4, addrspace(5)
  %"84" = alloca float, align 4, addrspace(5)
  %"85" = alloca float, align 4, addrspace(5)
  %"86" = alloca float, align 4, addrspace(5)
  %"87" = alloca i32, align 4, addrspace(5)
  %"88" = alloca i32, align 4, addrspace(5)
  %"89" = alloca i32, align 4, addrspace(5)
  %"90" = alloca i32, align 4, addrspace(5)
  %"91" = alloca i32, align 4, addrspace(5)
  %"92" = alloca i32, align 4, addrspace(5)
  %"93" = alloca float, align 4, addrspace(5)
  %"94" = alloca float, align 4, addrspace(5)
  %"95" = alloca float, align 4, addrspace(5)
  %"96" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"73"

"73":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"75", align 8
  store i64 %2, ptr addrspace(5) %"76", align 8
  %"53" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"53", ptr addrspace(5) %"78", align 4
  %3 = load i32, ptr addrspace(5) %"78", align 4
  %"99" = uitofp i32 %3 to float
  store float %"99", ptr addrspace(5) %"79", align 4
  %4 = load float, ptr addrspace(5) %"79", align 4
  %"101" = fmul float %4, 8.000000e+00
  store float %"101", ptr addrspace(5) %"79", align 4
  %5 = load float, ptr addrspace(5) %"79", align 4
  %"103" = fadd float %5, 1.000000e+00
  store float %"103", ptr addrspace(5) %"80", align 4
  %6 = load float, ptr addrspace(5) %"79", align 4
  %"105" = fadd float %6, 2.000000e+00
  store float %"105", ptr addrspace(5) %"81", align 4
  %7 = load float, ptr addrspace(5) %"79", align 4
  %"107" = fadd float %7, 3.000000e+00
  store float %"107", ptr addrspace(5) %"82", align 4
  %8 = load float, ptr addrspace(5) %"79", align 4
  %"109" = fadd float %8, 4.000000e+00
  store float %"109", ptr addrspace(5) %"83", align 4
  %9 = load float, ptr addrspace(5) %"79", align 4
  %"111" = fadd float %9, 5.000000e+00
  store float %"111", ptr addrspace(5) %"84", align 4
  %10 = load float, ptr addrspace(5) %"79", align 4
  %"113" = fadd float %10, 6.000000e+00
  store float %"113", ptr addrspace(5) %"85", align 4
  %11 = load float, ptr addrspace(5) %"79", align 4
  %"115" = fadd float %11, 7.000000e+00
  store float %"115", ptr addrspace(5) %"86", align 4
  %12 = load float, ptr addrspace(5) %"79", align 4
  %13 = load float, ptr addrspace(5) %"80", align 4
  %14 = fptrunc float %12 to bfloat
  %15 = fptrunc float %13 to bfloat
  %16 = insertelement <2 x bfloat> poison, bfloat %14, i32 1
  %"164" = insertelement <2 x bfloat> %16, bfloat %15, i32 0
  %"117" = bitcast <2 x bfloat> %"164" to i32
  store i32 %"117", ptr addrspace(5) %"87", align 4
  %17 = load float, ptr addrspace(5) %"81", align 4
  %18 = load float, ptr addrspace(5) %"82", align 4
  %19 = fptrunc float %17 to bfloat
  %20 = fptrunc float %18 to bfloat
  %21 = insertelement <2 x bfloat> poison, bfloat %19, i32 1
  %"165" = insertelement <2 x bfloat> %21, bfloat %20, i32 0
  %"120" = bitcast <2 x bfloat> %"165" to i32
  store i32 %"120", ptr addrspace(5) %"88", align 4
  %22 = load float, ptr addrspace(5) %"83", align 4
  %23 = load float, ptr addrspace(5) %"84", align 4
  %24 = fptrunc float %22 to bfloat
  %25 = fptrunc float %23 to bfloat
  %26 = insertelement <2 x bfloat> poison, bfloat %24, i32 1
  %"166" = insertelement <2 x bfloat> %26, bfloat %25, i32 0
  %"123" = bitcast <2 x bfloat> %"166" to i32
  store i32 %"123", ptr addrspace(5) %"89", align 4
  %27 = load float, ptr addrspace(5) %"85", align 4
  %28 = load float, ptr addrspace(5) %"86", align 4
  %29 = fptrunc float %27 to bfloat
  %30 = fptrunc float %28 to bfloat
  %31 = insertelement <2 x bfloat> poison, bfloat %29, i32 1
  %"167" = insertelement <2 x bfloat> %31, bfloat %30, i32 0
  %"126" = bitcast <2 x bfloat> %"167" to i32
  store i32 %"126", ptr addrspace(5) %"90", align 4
  %32 = load float, ptr addrspace(5) %"79", align 4
  %33 = load float, ptr addrspace(5) %"80", align 4
  %34 = fptrunc float %32 to bfloat
  %35 = fptrunc float %33 to bfloat
  %36 = insertelement <2 x bfloat> poison, bfloat %34, i32 1
  %"168" = insertelement <2 x bfloat> %36, bfloat %35, i32 0
  %"129" = bitcast <2 x bfloat> %"168" to i32
  store i32 %"129", ptr addrspace(5) %"91", align 4
  %37 = load float, ptr addrspace(5) %"81", align 4
  %38 = load float, ptr addrspace(5) %"82", align 4
  %39 = fptrunc float %37 to bfloat
  %40 = fptrunc float %38 to bfloat
  %41 = insertelement <2 x bfloat> poison, bfloat %39, i32 1
  %"169" = insertelement <2 x bfloat> %41, bfloat %40, i32 0
  %"132" = bitcast <2 x bfloat> %"169" to i32
  store i32 %"132", ptr addrspace(5) %"92", align 4
  %42 = load i32, ptr addrspace(5) %"87", align 4
  %43 = load i32, ptr addrspace(5) %"88", align 4
  %44 = load i32, ptr addrspace(5) %"89", align 4
  %45 = load i32, ptr addrspace(5) %"90", align 4
  %46 = insertelement <4 x i32> undef, i32 %42, i8 0
  %47 = insertelement <4 x i32> %46, i32 %43, i8 1
  %48 = insertelement <4 x i32> %47, i32 %44, i8 2
  %"63" = insertelement <4 x i32> %48, i32 %45, i8 3
  %49 = load i32, ptr addrspace(5) %"91", align 4
  %50 = load i32, ptr addrspace(5) %"92", align 4
  %51 = insertelement <2 x i32> undef, i32 %49, i8 0
  %"64" = insertelement <2 x i32> %51, i32 %50, i8 1
  %52 = load float, ptr addrspace(5) %"79", align 4
  %53 = load float, ptr addrspace(5) %"80", align 4
  %54 = load float, ptr addrspace(5) %"81", align 4
  %55 = load float, ptr addrspace(5) %"82", align 4
  %56 = insertelement <4 x float> undef, float %52, i8 0
  %57 = insertelement <4 x float> %56, float %53, i8 1
  %58 = insertelement <4 x float> %57, float %54, i8 2
  %"65" = insertelement <4 x float> %58, float %55, i8 3
  %"62" = call <4 x float> @__zluda_ptx_impl_mma_sync_aligned_m16n8k16_row_col_f32_bf16_bf16_f32(<4 x i32> %"63", <2 x i32> %"64", <4 x float> %"65")
  %"145" = extractelement <4 x float> %"62", i8 0
  %"146" = extractelement <4 x float> %"62", i8 1
  %"147" = extractelement <4 x float> %"62", i8 2
  %"148" = extractelement <4 x float> %"62", i8 3
  store float %"145", ptr addrspace(5) %"93", align 4
  store float %"146", ptr addrspace(5) %"94", align 4
  store float %"147", ptr addrspace(5) %"95", align 4
  store float %"148", ptr addrspace(5) %"96", align 4
  %59 = load i32, ptr addrspace(5) %"78", align 4
  %60 = zext i32 %59 to i64
  store i64 %60, ptr addrspace(5) %"77", align 8
  %61 = load i64, ptr addrspace(5) %"77", align 8
  %"151" = mul i64 %61, 16
  store i64 %"151", ptr addrspace(5) %"77", align 8
  %62 = load i64, ptr addrspace(5) %"76", align 8
  %63 = load i64, ptr addrspace(5) %"77", align 8
  %"153" = add i64 %62, %63
  store i64 %"153", ptr addrspace(5) %"76", align 8
  %64 = load i64, ptr addrspace(5) %"76", align 8
  %65 = load float, ptr addrspace(5) %"93", align 4
  %"176" = inttoptr i64 %64 to ptr
  store float %65, ptr %"176", align 4
  %66 = load i64, ptr addrspace(5) %"76", align 8
  %"177" = inttoptr i64 %66 to ptr
  %"68" = getelementptr inbounds i8, ptr %"177", i64 4
  %67 = load float, ptr addrspace(5) %"94", align 4
  store float %67, ptr %"68", align 4
  %68 = load i64, ptr addrspace(5) %"76", align 8
  %"178" = inttoptr i64 %68 to ptr
  %"70" = getelementptr inbounds i8, ptr %"178", i64 8
  %69 = load float, ptr addrspace(5) %"95", align 4
  store float %69, ptr %"70", align 4
  %70 = load i64, ptr addrspace(5) %"76", align 8
  %"179" = inttoptr i64 %70 to ptr
  %"72" = getelementptr inbounds i8, ptr %"179", i64 12
  %71 = load float, ptr addrspace(5) %"96", align 4
  store float %71, ptr %"72", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }