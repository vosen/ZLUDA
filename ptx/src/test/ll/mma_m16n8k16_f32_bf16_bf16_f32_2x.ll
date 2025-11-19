declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @mma_m16n8k16_f32_bf16_bf16_f32_2x(ptr addrspace(4) byref(i64) %"93") #1 {
  %"94" = alloca i64, align 8, addrspace(5)
  %"95" = alloca i64, align 8, addrspace(5)
  %"96" = alloca i32, align 4, addrspace(5)
  %"97" = alloca float, align 4, addrspace(5)
  %"98" = alloca float, align 4, addrspace(5)
  %"99" = alloca float, align 4, addrspace(5)
  %"100" = alloca float, align 4, addrspace(5)
  %"101" = alloca float, align 4, addrspace(5)
  %"102" = alloca float, align 4, addrspace(5)
  %"103" = alloca float, align 4, addrspace(5)
  %"104" = alloca float, align 4, addrspace(5)
  %"105" = alloca i32, align 4, addrspace(5)
  %"106" = alloca i32, align 4, addrspace(5)
  %"107" = alloca i32, align 4, addrspace(5)
  %"108" = alloca i32, align 4, addrspace(5)
  %"109" = alloca i32, align 4, addrspace(5)
  %"110" = alloca i32, align 4, addrspace(5)
  %"111" = alloca i32, align 4, addrspace(5)
  %"112" = alloca i32, align 4, addrspace(5)
  %"113" = alloca float, align 4, addrspace(5)
  %"114" = alloca float, align 4, addrspace(5)
  %"115" = alloca float, align 4, addrspace(5)
  %"116" = alloca float, align 4, addrspace(5)
  %"117" = alloca float, align 4, addrspace(5)
  %"118" = alloca float, align 4, addrspace(5)
  %"119" = alloca float, align 4, addrspace(5)
  %"120" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"91"

"91":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"93", align 8
  store i64 %2, ptr addrspace(5) %"94", align 8
  %"59" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"59", ptr addrspace(5) %"96", align 4
  %3 = load i32, ptr addrspace(5) %"96", align 4
  %"123" = uitofp i32 %3 to float
  store float %"123", ptr addrspace(5) %"97", align 4
  %4 = load float, ptr addrspace(5) %"97", align 4
  %"125" = fmul float %4, 8.000000e+00
  store float %"125", ptr addrspace(5) %"97", align 4
  %5 = load float, ptr addrspace(5) %"97", align 4
  %"127" = fadd float %5, 1.000000e+00
  store float %"127", ptr addrspace(5) %"98", align 4
  %6 = load float, ptr addrspace(5) %"97", align 4
  %"129" = fadd float %6, 2.000000e+00
  store float %"129", ptr addrspace(5) %"99", align 4
  %7 = load float, ptr addrspace(5) %"97", align 4
  %"131" = fadd float %7, 3.000000e+00
  store float %"131", ptr addrspace(5) %"100", align 4
  %8 = load float, ptr addrspace(5) %"97", align 4
  %"133" = fadd float %8, 4.000000e+00
  store float %"133", ptr addrspace(5) %"101", align 4
  %9 = load float, ptr addrspace(5) %"97", align 4
  %"135" = fadd float %9, 5.000000e+00
  store float %"135", ptr addrspace(5) %"102", align 4
  %10 = load float, ptr addrspace(5) %"97", align 4
  %"137" = fadd float %10, 6.000000e+00
  store float %"137", ptr addrspace(5) %"103", align 4
  %11 = load float, ptr addrspace(5) %"97", align 4
  %"139" = fadd float %11, 7.000000e+00
  store float %"139", ptr addrspace(5) %"104", align 4
  %12 = load float, ptr addrspace(5) %"97", align 4
  %13 = load float, ptr addrspace(5) %"98", align 4
  %14 = fptrunc float %12 to bfloat
  %15 = fptrunc float %13 to bfloat
  %16 = insertelement <2 x bfloat> poison, bfloat %14, i32 1
  %"216" = insertelement <2 x bfloat> %16, bfloat %15, i32 0
  %"141" = bitcast <2 x bfloat> %"216" to i32
  store i32 %"141", ptr addrspace(5) %"105", align 4
  %17 = load float, ptr addrspace(5) %"99", align 4
  %18 = load float, ptr addrspace(5) %"100", align 4
  %19 = fptrunc float %17 to bfloat
  %20 = fptrunc float %18 to bfloat
  %21 = insertelement <2 x bfloat> poison, bfloat %19, i32 1
  %"217" = insertelement <2 x bfloat> %21, bfloat %20, i32 0
  %"144" = bitcast <2 x bfloat> %"217" to i32
  store i32 %"144", ptr addrspace(5) %"106", align 4
  %22 = load float, ptr addrspace(5) %"101", align 4
  %23 = load float, ptr addrspace(5) %"102", align 4
  %24 = fptrunc float %22 to bfloat
  %25 = fptrunc float %23 to bfloat
  %26 = insertelement <2 x bfloat> poison, bfloat %24, i32 1
  %"218" = insertelement <2 x bfloat> %26, bfloat %25, i32 0
  %"147" = bitcast <2 x bfloat> %"218" to i32
  store i32 %"147", ptr addrspace(5) %"107", align 4
  %27 = load float, ptr addrspace(5) %"103", align 4
  %28 = load float, ptr addrspace(5) %"104", align 4
  %29 = fptrunc float %27 to bfloat
  %30 = fptrunc float %28 to bfloat
  %31 = insertelement <2 x bfloat> poison, bfloat %29, i32 1
  %"219" = insertelement <2 x bfloat> %31, bfloat %30, i32 0
  %"150" = bitcast <2 x bfloat> %"219" to i32
  store i32 %"150", ptr addrspace(5) %"108", align 4
  %32 = load float, ptr addrspace(5) %"97", align 4
  %33 = load float, ptr addrspace(5) %"98", align 4
  %34 = fptrunc float %32 to bfloat
  %35 = fptrunc float %33 to bfloat
  %36 = insertelement <2 x bfloat> poison, bfloat %34, i32 1
  %"220" = insertelement <2 x bfloat> %36, bfloat %35, i32 0
  %"153" = bitcast <2 x bfloat> %"220" to i32
  store i32 %"153", ptr addrspace(5) %"109", align 4
  %37 = load float, ptr addrspace(5) %"99", align 4
  %38 = load float, ptr addrspace(5) %"100", align 4
  %39 = fptrunc float %37 to bfloat
  %40 = fptrunc float %38 to bfloat
  %41 = insertelement <2 x bfloat> poison, bfloat %39, i32 1
  %"221" = insertelement <2 x bfloat> %41, bfloat %40, i32 0
  %"156" = bitcast <2 x bfloat> %"221" to i32
  store i32 %"156", ptr addrspace(5) %"110", align 4
  %42 = load float, ptr addrspace(5) %"101", align 4
  %43 = load float, ptr addrspace(5) %"102", align 4
  %44 = fptrunc float %42 to bfloat
  %45 = fptrunc float %43 to bfloat
  %46 = insertelement <2 x bfloat> poison, bfloat %44, i32 1
  %"222" = insertelement <2 x bfloat> %46, bfloat %45, i32 0
  %"159" = bitcast <2 x bfloat> %"222" to i32
  store i32 %"159", ptr addrspace(5) %"111", align 4
  %47 = load float, ptr addrspace(5) %"103", align 4
  %48 = load float, ptr addrspace(5) %"104", align 4
  %49 = fptrunc float %47 to bfloat
  %50 = fptrunc float %48 to bfloat
  %51 = insertelement <2 x bfloat> poison, bfloat %49, i32 1
  %"223" = insertelement <2 x bfloat> %51, bfloat %50, i32 0
  %"162" = bitcast <2 x bfloat> %"223" to i32
  store i32 %"162", ptr addrspace(5) %"112", align 4
  %52 = load i32, ptr addrspace(5) %"105", align 4
  %53 = load i32, ptr addrspace(5) %"106", align 4
  %54 = load i32, ptr addrspace(5) %"107", align 4
  %55 = load i32, ptr addrspace(5) %"108", align 4
  %56 = insertelement <4 x i32> undef, i32 %52, i8 0
  %57 = insertelement <4 x i32> %56, i32 %53, i8 1
  %58 = insertelement <4 x i32> %57, i32 %54, i8 2
  %"69" = insertelement <4 x i32> %58, i32 %55, i8 3
  %59 = load i32, ptr addrspace(5) %"109", align 4
  %60 = load i32, ptr addrspace(5) %"110", align 4
  %61 = insertelement <2 x i32> undef, i32 %59, i8 0
  %"70" = insertelement <2 x i32> %61, i32 %60, i8 1
  %62 = load float, ptr addrspace(5) %"97", align 4
  %63 = load float, ptr addrspace(5) %"98", align 4
  %64 = load float, ptr addrspace(5) %"99", align 4
  %65 = load float, ptr addrspace(5) %"100", align 4
  %66 = insertelement <4 x float> undef, float %62, i8 0
  %67 = insertelement <4 x float> %66, float %63, i8 1
  %68 = insertelement <4 x float> %67, float %64, i8 2
  %"71" = insertelement <4 x float> %68, float %65, i8 3
  %"68" = call <4 x float> @llvm.zluda.mma.m16n8k16(<4 x i32> %"69", <2 x i32> %"70", <4 x float> %"71")
  %"175" = extractelement <4 x float> %"68", i8 0
  %"176" = extractelement <4 x float> %"68", i8 1
  %"177" = extractelement <4 x float> %"68", i8 2
  %"178" = extractelement <4 x float> %"68", i8 3
  store float %"175", ptr addrspace(5) %"113", align 4
  store float %"176", ptr addrspace(5) %"114", align 4
  store float %"177", ptr addrspace(5) %"115", align 4
  store float %"178", ptr addrspace(5) %"116", align 4
  %69 = load i32, ptr addrspace(5) %"105", align 4
  %70 = load i32, ptr addrspace(5) %"106", align 4
  %71 = load i32, ptr addrspace(5) %"107", align 4
  %72 = load i32, ptr addrspace(5) %"108", align 4
  %73 = insertelement <4 x i32> undef, i32 %69, i8 0
  %74 = insertelement <4 x i32> %73, i32 %70, i8 1
  %75 = insertelement <4 x i32> %74, i32 %71, i8 2
  %"73" = insertelement <4 x i32> %75, i32 %72, i8 3
  %76 = load i32, ptr addrspace(5) %"111", align 4
  %77 = load i32, ptr addrspace(5) %"112", align 4
  %78 = insertelement <2 x i32> undef, i32 %76, i8 0
  %"74" = insertelement <2 x i32> %78, i32 %77, i8 1
  %79 = load float, ptr addrspace(5) %"101", align 4
  %80 = load float, ptr addrspace(5) %"102", align 4
  %81 = load float, ptr addrspace(5) %"103", align 4
  %82 = load float, ptr addrspace(5) %"104", align 4
  %83 = insertelement <4 x float> undef, float %79, i8 0
  %84 = insertelement <4 x float> %83, float %80, i8 1
  %85 = insertelement <4 x float> %84, float %81, i8 2
  %"75" = insertelement <4 x float> %85, float %82, i8 3
  %"72" = call <4 x float> @llvm.zluda.mma.m16n8k16(<4 x i32> %"73", <2 x i32> %"74", <4 x float> %"75")
  %"189" = extractelement <4 x float> %"72", i8 0
  %"190" = extractelement <4 x float> %"72", i8 1
  %"191" = extractelement <4 x float> %"72", i8 2
  %"192" = extractelement <4 x float> %"72", i8 3
  store float %"189", ptr addrspace(5) %"117", align 4
  store float %"190", ptr addrspace(5) %"118", align 4
  store float %"191", ptr addrspace(5) %"119", align 4
  store float %"192", ptr addrspace(5) %"120", align 4
  %86 = load i32, ptr addrspace(5) %"96", align 4
  %87 = zext i32 %86 to i64
  store i64 %87, ptr addrspace(5) %"95", align 8
  %88 = load i64, ptr addrspace(5) %"95", align 8
  %"195" = mul i64 %88, 32
  store i64 %"195", ptr addrspace(5) %"95", align 8
  %89 = load i64, ptr addrspace(5) %"94", align 8
  %90 = load i64, ptr addrspace(5) %"95", align 8
  %"197" = add i64 %89, %90
  store i64 %"197", ptr addrspace(5) %"94", align 8
  %91 = load i64, ptr addrspace(5) %"94", align 8
  %92 = load float, ptr addrspace(5) %"113", align 4
  %"236" = inttoptr i64 %91 to ptr
  store float %92, ptr %"236", align 4
  %93 = load i64, ptr addrspace(5) %"94", align 8
  %"237" = inttoptr i64 %93 to ptr
  %"78" = getelementptr inbounds i8, ptr %"237", i64 4
  %94 = load float, ptr addrspace(5) %"114", align 4
  store float %94, ptr %"78", align 4
  %95 = load i64, ptr addrspace(5) %"94", align 8
  %"238" = inttoptr i64 %95 to ptr
  %"80" = getelementptr inbounds i8, ptr %"238", i64 8
  %96 = load float, ptr addrspace(5) %"115", align 4
  store float %96, ptr %"80", align 4
  %97 = load i64, ptr addrspace(5) %"94", align 8
  %"239" = inttoptr i64 %97 to ptr
  %"82" = getelementptr inbounds i8, ptr %"239", i64 12
  %98 = load float, ptr addrspace(5) %"116", align 4
  store float %98, ptr %"82", align 4
  %99 = load i64, ptr addrspace(5) %"94", align 8
  %"240" = inttoptr i64 %99 to ptr
  %"84" = getelementptr inbounds i8, ptr %"240", i64 16
  %100 = load float, ptr addrspace(5) %"117", align 4
  store float %100, ptr %"84", align 4
  %101 = load i64, ptr addrspace(5) %"94", align 8
  %"241" = inttoptr i64 %101 to ptr
  %"86" = getelementptr inbounds i8, ptr %"241", i64 20
  %102 = load float, ptr addrspace(5) %"118", align 4
  store float %102, ptr %"86", align 4
  %103 = load i64, ptr addrspace(5) %"94", align 8
  %"242" = inttoptr i64 %103 to ptr
  %"88" = getelementptr inbounds i8, ptr %"242", i64 24
  %104 = load float, ptr addrspace(5) %"119", align 4
  store float %104, ptr %"88", align 4
  %105 = load i64, ptr addrspace(5) %"94", align 8
  %"243" = inttoptr i64 %105 to ptr
  %"90" = getelementptr inbounds i8, ptr %"243", i64 28
  %106 = load float, ptr addrspace(5) %"120", align 4
  store float %106, ptr %"90", align 4
  ret void
}

; Function Attrs: convergent nocallback nofree nosync nounwind willreturn memory(none)
declare <4 x float> @llvm.zluda.mma.m16n8k16(<4 x i32>, <2 x i32>, <4 x float>) #2

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { convergent nocallback nofree nosync nounwind willreturn memory(none) }