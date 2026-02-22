declare hidden <4 x float> @__zluda_ptx_impl_mma_sync_aligned_m16n8k16_row_col_f32_bf16_bf16_f32(<4 x i32>, <2 x i32>, <4 x float>) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @mma_m16n8k16_f32_bf16_bf16_f32(ptr addrspace(4) byref(i64) %"78") #1 {
  %"79" = alloca i64, align 8, addrspace(5)
  %"80" = alloca i64, align 8, addrspace(5)
  %"81" = alloca i32, align 4, addrspace(5)
  %"82" = alloca float, align 4, addrspace(5)
  %"83" = alloca float, align 4, addrspace(5)
  %"84" = alloca float, align 4, addrspace(5)
  %"85" = alloca float, align 4, addrspace(5)
  %"86" = alloca float, align 4, addrspace(5)
  %"87" = alloca float, align 4, addrspace(5)
  %"88" = alloca float, align 4, addrspace(5)
  %"89" = alloca float, align 4, addrspace(5)
  %"90" = alloca i32, align 4, addrspace(5)
  %"91" = alloca i32, align 4, addrspace(5)
  %"92" = alloca i32, align 4, addrspace(5)
  %"93" = alloca i32, align 4, addrspace(5)
  %"94" = alloca i32, align 4, addrspace(5)
  %"95" = alloca i32, align 4, addrspace(5)
  %"96" = alloca float, align 4, addrspace(5)
  %"97" = alloca float, align 4, addrspace(5)
  %"98" = alloca float, align 4, addrspace(5)
  %"99" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"76"

"76":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"78", align 8
  store i64 %2, ptr addrspace(5) %"79", align 8
  %"56" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"56", ptr addrspace(5) %"81", align 4
  %3 = load i32, ptr addrspace(5) %"81", align 4
  %"102" = uitofp i32 %3 to float
  store float %"102", ptr addrspace(5) %"82", align 4
  %4 = load float, ptr addrspace(5) %"82", align 4
  %"104" = fmul float %4, 8.000000e+00
  store float %"104", ptr addrspace(5) %"82", align 4
  %5 = load float, ptr addrspace(5) %"82", align 4
  %"106" = fadd float %5, 1.000000e+00
  store float %"106", ptr addrspace(5) %"83", align 4
  %6 = load float, ptr addrspace(5) %"82", align 4
  %"108" = fadd float %6, 2.000000e+00
  store float %"108", ptr addrspace(5) %"84", align 4
  %7 = load float, ptr addrspace(5) %"82", align 4
  %"110" = fadd float %7, 3.000000e+00
  store float %"110", ptr addrspace(5) %"85", align 4
  %8 = load float, ptr addrspace(5) %"82", align 4
  %"112" = fadd float %8, 4.000000e+00
  store float %"112", ptr addrspace(5) %"86", align 4
  %9 = load float, ptr addrspace(5) %"82", align 4
  %"114" = fadd float %9, 5.000000e+00
  store float %"114", ptr addrspace(5) %"87", align 4
  %10 = load float, ptr addrspace(5) %"82", align 4
  %"116" = fadd float %10, 6.000000e+00
  store float %"116", ptr addrspace(5) %"88", align 4
  %11 = load float, ptr addrspace(5) %"82", align 4
  %"118" = fadd float %11, 7.000000e+00
  store float %"118", ptr addrspace(5) %"89", align 4
  %12 = load float, ptr addrspace(5) %"82", align 4
  %13 = load float, ptr addrspace(5) %"83", align 4
  %14 = fptrunc float %12 to bfloat
  %15 = fptrunc float %13 to bfloat
  %16 = insertelement <2 x bfloat> poison, bfloat %14, i32 1
  %"167" = insertelement <2 x bfloat> %16, bfloat %15, i32 0
  %"120" = bitcast <2 x bfloat> %"167" to i32
  store i32 %"120", ptr addrspace(5) %"90", align 4
  %17 = load float, ptr addrspace(5) %"84", align 4
  %18 = load float, ptr addrspace(5) %"85", align 4
  %19 = fptrunc float %17 to bfloat
  %20 = fptrunc float %18 to bfloat
  %21 = insertelement <2 x bfloat> poison, bfloat %19, i32 1
  %"168" = insertelement <2 x bfloat> %21, bfloat %20, i32 0
  %"123" = bitcast <2 x bfloat> %"168" to i32
  store i32 %"123", ptr addrspace(5) %"91", align 4
  %22 = load float, ptr addrspace(5) %"86", align 4
  %23 = load float, ptr addrspace(5) %"87", align 4
  %24 = fptrunc float %22 to bfloat
  %25 = fptrunc float %23 to bfloat
  %26 = insertelement <2 x bfloat> poison, bfloat %24, i32 1
  %"169" = insertelement <2 x bfloat> %26, bfloat %25, i32 0
  %"126" = bitcast <2 x bfloat> %"169" to i32
  store i32 %"126", ptr addrspace(5) %"92", align 4
  %27 = load float, ptr addrspace(5) %"88", align 4
  %28 = load float, ptr addrspace(5) %"89", align 4
  %29 = fptrunc float %27 to bfloat
  %30 = fptrunc float %28 to bfloat
  %31 = insertelement <2 x bfloat> poison, bfloat %29, i32 1
  %"170" = insertelement <2 x bfloat> %31, bfloat %30, i32 0
  %"129" = bitcast <2 x bfloat> %"170" to i32
  store i32 %"129", ptr addrspace(5) %"93", align 4
  %32 = load float, ptr addrspace(5) %"82", align 4
  %33 = load float, ptr addrspace(5) %"83", align 4
  %34 = fptrunc float %32 to bfloat
  %35 = fptrunc float %33 to bfloat
  %36 = insertelement <2 x bfloat> poison, bfloat %34, i32 1
  %"171" = insertelement <2 x bfloat> %36, bfloat %35, i32 0
  %"132" = bitcast <2 x bfloat> %"171" to i32
  store i32 %"132", ptr addrspace(5) %"94", align 4
  %37 = load float, ptr addrspace(5) %"84", align 4
  %38 = load float, ptr addrspace(5) %"85", align 4
  %39 = fptrunc float %37 to bfloat
  %40 = fptrunc float %38 to bfloat
  %41 = insertelement <2 x bfloat> poison, bfloat %39, i32 1
  %"172" = insertelement <2 x bfloat> %41, bfloat %40, i32 0
  %"135" = bitcast <2 x bfloat> %"172" to i32
  store i32 %"135", ptr addrspace(5) %"95", align 4
  %42 = load i32, ptr addrspace(5) %"90", align 4
  %43 = load i32, ptr addrspace(5) %"91", align 4
  %44 = load i32, ptr addrspace(5) %"92", align 4
  %45 = load i32, ptr addrspace(5) %"93", align 4
  %46 = insertelement <4 x i32> undef, i32 %42, i8 0
  %47 = insertelement <4 x i32> %46, i32 %43, i8 1
  %48 = insertelement <4 x i32> %47, i32 %44, i8 2
  %"66" = insertelement <4 x i32> %48, i32 %45, i8 3
  %49 = load i32, ptr addrspace(5) %"94", align 4
  %50 = load i32, ptr addrspace(5) %"95", align 4
  %51 = insertelement <2 x i32> undef, i32 %49, i8 0
  %"67" = insertelement <2 x i32> %51, i32 %50, i8 1
  %52 = load float, ptr addrspace(5) %"82", align 4
  %53 = load float, ptr addrspace(5) %"83", align 4
  %54 = load float, ptr addrspace(5) %"84", align 4
  %55 = load float, ptr addrspace(5) %"85", align 4
  %56 = insertelement <4 x float> undef, float %52, i8 0
  %57 = insertelement <4 x float> %56, float %53, i8 1
  %58 = insertelement <4 x float> %57, float %54, i8 2
  %"68" = insertelement <4 x float> %58, float %55, i8 3
  %"65" = call <4 x float> @__zluda_ptx_impl_mma_sync_aligned_m16n8k16_row_col_f32_bf16_bf16_f32(<4 x i32> %"66", <2 x i32> %"67", <4 x float> %"68")
  %"148" = extractelement <4 x float> %"65", i8 0
  %"149" = extractelement <4 x float> %"65", i8 1
  %"150" = extractelement <4 x float> %"65", i8 2
  %"151" = extractelement <4 x float> %"65", i8 3
  store float %"148", ptr addrspace(5) %"96", align 4
  store float %"149", ptr addrspace(5) %"97", align 4
  store float %"150", ptr addrspace(5) %"98", align 4
  store float %"151", ptr addrspace(5) %"99", align 4
  %59 = load i32, ptr addrspace(5) %"81", align 4
  %60 = zext i32 %59 to i64
  store i64 %60, ptr addrspace(5) %"80", align 8
  %61 = load i64, ptr addrspace(5) %"80", align 8
  %"154" = mul i64 %61, 16
  store i64 %"154", ptr addrspace(5) %"80", align 8
  %62 = load i64, ptr addrspace(5) %"79", align 8
  %63 = load i64, ptr addrspace(5) %"80", align 8
  %"156" = add i64 %62, %63
  store i64 %"156", ptr addrspace(5) %"79", align 8
  %64 = load i64, ptr addrspace(5) %"79", align 8
  %65 = load float, ptr addrspace(5) %"96", align 4
  %"179" = inttoptr i64 %64 to ptr
  store float %65, ptr %"179", align 4
  %66 = load i64, ptr addrspace(5) %"79", align 8
  %"180" = inttoptr i64 %66 to ptr
  %"71" = getelementptr inbounds i8, ptr %"180", i64 4
  %67 = load float, ptr addrspace(5) %"97", align 4
  store float %67, ptr %"71", align 4
  %68 = load i64, ptr addrspace(5) %"79", align 8
  %"181" = inttoptr i64 %68 to ptr
  %"73" = getelementptr inbounds i8, ptr %"181", i64 8
  %69 = load float, ptr addrspace(5) %"98", align 4
  store float %69, ptr %"73", align 4
  %70 = load i64, ptr addrspace(5) %"79", align 8
  %"182" = inttoptr i64 %70 to ptr
  %"75" = getelementptr inbounds i8, ptr %"182", i64 12
  %71 = load float, ptr addrspace(5) %"99", align 4
  store float %71, ptr %"75", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
