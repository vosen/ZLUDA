
declare hidden <4 x i32> @__zluda_ptx_impl_mma_sync_aligned_m16n8k32_row_col_s32_s8_s8_s32(<4 x i32>, <2 x i32>, <4 x i32>) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define hidden i32 @pack_u8(<4 x i16> %"11") #0 {
  %"123" = alloca i32, align 4, addrspace(5)
  %"124" = alloca i32, align 4, addrspace(5)
  %"125" = alloca i32, align 4, addrspace(5)
  %"126" = alloca i32, align 4, addrspace(5)
  %"127" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"114"

"114":                                            ; preds = %1
  %"74" = extractelement <4 x i16> %"11", i8 0
  %2 = zext i16 %"74" to i32
  store i32 %2, ptr addrspace(5) %"124", align 4
  %"75" = extractelement <4 x i16> %"11", i8 1
  %3 = zext i16 %"75" to i32
  store i32 %3, ptr addrspace(5) %"125", align 4
  %"76" = extractelement <4 x i16> %"11", i8 2
  %4 = zext i16 %"76" to i32
  store i32 %4, ptr addrspace(5) %"126", align 4
  %"77" = extractelement <4 x i16> %"11", i8 3
  %5 = zext i16 %"77" to i32
  store i32 %5, ptr addrspace(5) %"127", align 4
  %6 = load i32, ptr addrspace(5) %"125", align 4
  %7 = shl i32 %6, 8
  %"299" = select i1 false, i32 0, i32 %7
  store i32 %"299", ptr addrspace(5) %"125", align 4
  %8 = load i32, ptr addrspace(5) %"126", align 4
  %9 = shl i32 %8, 16
  %"301" = select i1 false, i32 0, i32 %9
  store i32 %"301", ptr addrspace(5) %"126", align 4
  %10 = load i32, ptr addrspace(5) %"127", align 4
  %11 = shl i32 %10, 24
  %"303" = select i1 false, i32 0, i32 %11
  store i32 %"303", ptr addrspace(5) %"127", align 4
  %12 = load i32, ptr addrspace(5) %"124", align 4
  store i32 %12, ptr addrspace(5) %"123", align 4
  %13 = load i32, ptr addrspace(5) %"123", align 4
  %14 = load i32, ptr addrspace(5) %"125", align 4
  %"307" = or i32 %13, %14
  store i32 %"307", ptr addrspace(5) %"123", align 4
  %15 = load i32, ptr addrspace(5) %"123", align 4
  %16 = load i32, ptr addrspace(5) %"126", align 4
  %"310" = or i32 %15, %16
  store i32 %"310", ptr addrspace(5) %"123", align 4
  %17 = load i32, ptr addrspace(5) %"123", align 4
  %18 = load i32, ptr addrspace(5) %"127", align 4
  %"313" = or i32 %17, %18
  store i32 %"313", ptr addrspace(5) %"123", align 4
  %19 = load i32, ptr addrspace(5) %"123", align 4
  ret i32 %19
}

define amdgpu_kernel void @mma_m16n8k32_s32_s8_s8_s32(ptr addrspace(4) byref(i64) %"149") #1 {
  %"150" = alloca i64, align 8, addrspace(5)
  %"151" = alloca i64, align 8, addrspace(5)
  %"152" = alloca i32, align 4, addrspace(5)
  %"153" = alloca i16, align 2, addrspace(5)
  %"154" = alloca i16, align 2, addrspace(5)
  %"155" = alloca i16, align 2, addrspace(5)
  %"156" = alloca i16, align 2, addrspace(5)
  %"157" = alloca i16, align 2, addrspace(5)
  %"158" = alloca i16, align 2, addrspace(5)
  %"159" = alloca i16, align 2, addrspace(5)
  %"160" = alloca i16, align 2, addrspace(5)
  %"161" = alloca i16, align 2, addrspace(5)
  %"162" = alloca i16, align 2, addrspace(5)
  %"163" = alloca i16, align 2, addrspace(5)
  %"164" = alloca i16, align 2, addrspace(5)
  %"165" = alloca i16, align 2, addrspace(5)
  %"166" = alloca i16, align 2, addrspace(5)
  %"167" = alloca i16, align 2, addrspace(5)
  %"168" = alloca i16, align 2, addrspace(5)
  %"169" = alloca i32, align 4, addrspace(5)
  %"170" = alloca i32, align 4, addrspace(5)
  %"171" = alloca i32, align 4, addrspace(5)
  %"172" = alloca i32, align 4, addrspace(5)
  %"173" = alloca i32, align 4, addrspace(5)
  %"174" = alloca i32, align 4, addrspace(5)
  %"175" = alloca i32, align 4, addrspace(5)
  %"176" = alloca i32, align 4, addrspace(5)
  %"177" = alloca i32, align 4, addrspace(5)
  %"178" = alloca i32, align 4, addrspace(5)
  %"179" = alloca i32, align 4, addrspace(5)
  %"180" = alloca i32, align 4, addrspace(5)
  %"181" = alloca i32, align 4, addrspace(5)
  %"182" = alloca i32, align 4, addrspace(5)
  %"219" = alloca <4 x i16>, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"115"

"115":                                            ; preds = %1
  %2 = load i64, ptr addrspace(4) %"149", align 8
  store i64 %2, ptr addrspace(5) %"150", align 8
  %"73" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"73", ptr addrspace(5) %"152", align 4
  %3 = load i32, ptr addrspace(5) %"152", align 4
  %4 = trunc i32 %3 to i16
  store i16 %4, ptr addrspace(5) %"153", align 2
  %5 = load i16, ptr addrspace(5) %"153", align 2
  %"187" = mul i16 %5, 16
  store i16 %"187", ptr addrspace(5) %"153", align 2
  %6 = load i16, ptr addrspace(5) %"153", align 2
  %"189" = add i16 %6, 1
  store i16 %"189", ptr addrspace(5) %"154", align 2
  %7 = load i16, ptr addrspace(5) %"153", align 2
  %"191" = add i16 %7, 2
  store i16 %"191", ptr addrspace(5) %"155", align 2
  %8 = load i16, ptr addrspace(5) %"153", align 2
  %"193" = add i16 %8, 3
  store i16 %"193", ptr addrspace(5) %"156", align 2
  %9 = load i16, ptr addrspace(5) %"153", align 2
  %"195" = add i16 %9, 4
  store i16 %"195", ptr addrspace(5) %"157", align 2
  %10 = load i16, ptr addrspace(5) %"153", align 2
  %"197" = add i16 %10, 5
  store i16 %"197", ptr addrspace(5) %"158", align 2
  %11 = load i16, ptr addrspace(5) %"153", align 2
  %"199" = add i16 %11, 6
  store i16 %"199", ptr addrspace(5) %"159", align 2
  %12 = load i16, ptr addrspace(5) %"153", align 2
  %"201" = add i16 %12, 7
  store i16 %"201", ptr addrspace(5) %"160", align 2
  %13 = load i16, ptr addrspace(5) %"153", align 2
  %"203" = add i16 %13, 8
  store i16 %"203", ptr addrspace(5) %"161", align 2
  %14 = load i16, ptr addrspace(5) %"153", align 2
  %"205" = add i16 %14, 9
  store i16 %"205", ptr addrspace(5) %"162", align 2
  %15 = load i16, ptr addrspace(5) %"153", align 2
  %"207" = add i16 %15, 10
  store i16 %"207", ptr addrspace(5) %"163", align 2
  %16 = load i16, ptr addrspace(5) %"153", align 2
  %"209" = add i16 %16, 11
  store i16 %"209", ptr addrspace(5) %"164", align 2
  %17 = load i16, ptr addrspace(5) %"153", align 2
  %"211" = add i16 %17, 12
  store i16 %"211", ptr addrspace(5) %"165", align 2
  %18 = load i16, ptr addrspace(5) %"153", align 2
  %"213" = add i16 %18, 13
  store i16 %"213", ptr addrspace(5) %"166", align 2
  %19 = load i16, ptr addrspace(5) %"153", align 2
  %"215" = add i16 %19, 14
  store i16 %"215", ptr addrspace(5) %"167", align 2
  %20 = load i16, ptr addrspace(5) %"153", align 2
  %"217" = add i16 %20, 15
  store i16 %"217", ptr addrspace(5) %"168", align 2
  %21 = load i16, ptr addrspace(5) %"153", align 2
  %22 = load i16, ptr addrspace(5) %"154", align 2
  %23 = load i16, ptr addrspace(5) %"155", align 2
  %24 = load i16, ptr addrspace(5) %"156", align 2
  %25 = insertelement <4 x i16> undef, i16 %21, i8 0
  %26 = insertelement <4 x i16> %25, i16 %22, i8 1
  %27 = insertelement <4 x i16> %26, i16 %23, i8 2
  %"97" = insertelement <4 x i16> %27, i16 %24, i8 3
  store <4 x i16> %"97", ptr addrspace(5) %"219", align 8
  %28 = load <4 x i16>, ptr addrspace(5) %"219", align 8
  %"316" = call i32 @pack_u8(<4 x i16> %28)
  store i32 %"316", ptr addrspace(5) %"173", align 4
  br label %"116"

"116":                                            ; preds = %"115"
  %29 = load i16, ptr addrspace(5) %"157", align 2
  %30 = load i16, ptr addrspace(5) %"158", align 2
  %31 = load i16, ptr addrspace(5) %"159", align 2
  %32 = load i16, ptr addrspace(5) %"160", align 2
  %33 = insertelement <4 x i16> undef, i16 %29, i8 0
  %34 = insertelement <4 x i16> %33, i16 %30, i8 1
  %35 = insertelement <4 x i16> %34, i16 %31, i8 2
  %"98" = insertelement <4 x i16> %35, i16 %32, i8 3
  store <4 x i16> %"98", ptr addrspace(5) %"219", align 8
  %36 = load <4 x i16>, ptr addrspace(5) %"219", align 8
  %"317" = call i32 @pack_u8(<4 x i16> %36)
  store i32 %"317", ptr addrspace(5) %"174", align 4
  br label %"117"

"117":                                            ; preds = %"116"
  %37 = load i16, ptr addrspace(5) %"161", align 2
  %38 = load i16, ptr addrspace(5) %"162", align 2
  %39 = load i16, ptr addrspace(5) %"163", align 2
  %40 = load i16, ptr addrspace(5) %"164", align 2
  %41 = insertelement <4 x i16> undef, i16 %37, i8 0
  %42 = insertelement <4 x i16> %41, i16 %38, i8 1
  %43 = insertelement <4 x i16> %42, i16 %39, i8 2
  %"99" = insertelement <4 x i16> %43, i16 %40, i8 3
  store <4 x i16> %"99", ptr addrspace(5) %"219", align 8
  %44 = load <4 x i16>, ptr addrspace(5) %"219", align 8
  %"318" = call i32 @pack_u8(<4 x i16> %44)
  store i32 %"318", ptr addrspace(5) %"175", align 4
  br label %"118"

"118":                                            ; preds = %"117"
  %45 = load i16, ptr addrspace(5) %"165", align 2
  %46 = load i16, ptr addrspace(5) %"166", align 2
  %47 = load i16, ptr addrspace(5) %"167", align 2
  %48 = load i16, ptr addrspace(5) %"168", align 2
  %49 = insertelement <4 x i16> undef, i16 %45, i8 0
  %50 = insertelement <4 x i16> %49, i16 %46, i8 1
  %51 = insertelement <4 x i16> %50, i16 %47, i8 2
  %"100" = insertelement <4 x i16> %51, i16 %48, i8 3
  store <4 x i16> %"100", ptr addrspace(5) %"219", align 8
  %52 = load <4 x i16>, ptr addrspace(5) %"219", align 8
  %"319" = call i32 @pack_u8(<4 x i16> %52)
  store i32 %"319", ptr addrspace(5) %"176", align 4
  br label %"119"

"119":                                            ; preds = %"118"
  %53 = load i16, ptr addrspace(5) %"153", align 2
  %54 = load i16, ptr addrspace(5) %"154", align 2
  %55 = load i16, ptr addrspace(5) %"155", align 2
  %56 = load i16, ptr addrspace(5) %"156", align 2
  %57 = insertelement <4 x i16> undef, i16 %53, i8 0
  %58 = insertelement <4 x i16> %57, i16 %54, i8 1
  %59 = insertelement <4 x i16> %58, i16 %55, i8 2
  %"101" = insertelement <4 x i16> %59, i16 %56, i8 3
  store <4 x i16> %"101", ptr addrspace(5) %"219", align 8
  %60 = load <4 x i16>, ptr addrspace(5) %"219", align 8
  %"320" = call i32 @pack_u8(<4 x i16> %60)
  store i32 %"320", ptr addrspace(5) %"177", align 4
  br label %"120"

"120":                                            ; preds = %"119"
  %61 = load i16, ptr addrspace(5) %"153", align 2
  %62 = load i16, ptr addrspace(5) %"154", align 2
  %63 = load i16, ptr addrspace(5) %"155", align 2
  %64 = load i16, ptr addrspace(5) %"156", align 2
  %65 = insertelement <4 x i16> undef, i16 %61, i8 0
  %66 = insertelement <4 x i16> %65, i16 %62, i8 1
  %67 = insertelement <4 x i16> %66, i16 %63, i8 2
  %"102" = insertelement <4 x i16> %67, i16 %64, i8 3
  store <4 x i16> %"102", ptr addrspace(5) %"219", align 8
  %68 = load <4 x i16>, ptr addrspace(5) %"219", align 8
  %"321" = call i32 @pack_u8(<4 x i16> %68)
  store i32 %"321", ptr addrspace(5) %"178", align 4
  br label %"121"

"121":                                            ; preds = %"120"
  %69 = load i16, ptr addrspace(5) %"153", align 2
  %70 = zext i16 %69 to i32
  store i32 %70, ptr addrspace(5) %"169", align 4
  %71 = load i16, ptr addrspace(5) %"154", align 2
  %72 = zext i16 %71 to i32
  store i32 %72, ptr addrspace(5) %"170", align 4
  %73 = load i16, ptr addrspace(5) %"155", align 2
  %74 = zext i16 %73 to i32
  store i32 %74, ptr addrspace(5) %"171", align 4
  %75 = load i16, ptr addrspace(5) %"156", align 2
  %76 = zext i16 %75 to i32
  store i32 %76, ptr addrspace(5) %"172", align 4
  %77 = load i32, ptr addrspace(5) %"173", align 4
  %78 = load i32, ptr addrspace(5) %"174", align 4
  %79 = load i32, ptr addrspace(5) %"175", align 4
  %80 = load i32, ptr addrspace(5) %"176", align 4
  %81 = insertelement <4 x i32> undef, i32 %77, i8 0
  %82 = insertelement <4 x i32> %81, i32 %78, i8 1
  %83 = insertelement <4 x i32> %82, i32 %79, i8 2
  %"104" = insertelement <4 x i32> %83, i32 %80, i8 3
  %84 = load i32, ptr addrspace(5) %"177", align 4
  %85 = load i32, ptr addrspace(5) %"178", align 4
  %86 = insertelement <2 x i32> undef, i32 %84, i8 0
  %"105" = insertelement <2 x i32> %86, i32 %85, i8 1
  %87 = load i32, ptr addrspace(5) %"169", align 4
  %88 = load i32, ptr addrspace(5) %"170", align 4
  %89 = load i32, ptr addrspace(5) %"171", align 4
  %90 = load i32, ptr addrspace(5) %"172", align 4
  %91 = insertelement <4 x i32> undef, i32 %87, i8 0
  %92 = insertelement <4 x i32> %91, i32 %88, i8 1
  %93 = insertelement <4 x i32> %92, i32 %89, i8 2
  %"106" = insertelement <4 x i32> %93, i32 %90, i8 3
  %"103" = call <4 x i32> @__zluda_ptx_impl_mma_sync_aligned_m16n8k32_row_col_s32_s8_s8_s32(<4 x i32> %"104", <2 x i32> %"105", <4 x i32> %"106")
  %"328" = extractelement <4 x i32> %"103", i8 0
  %"329" = extractelement <4 x i32> %"103", i8 1
  %"330" = extractelement <4 x i32> %"103", i8 2
  %"331" = extractelement <4 x i32> %"103", i8 3
  store i32 %"328", ptr addrspace(5) %"179", align 4
  store i32 %"329", ptr addrspace(5) %"180", align 4
  store i32 %"330", ptr addrspace(5) %"181", align 4
  store i32 %"331", ptr addrspace(5) %"182", align 4
  %94 = load i32, ptr addrspace(5) %"152", align 4
  %95 = zext i32 %94 to i64
  store i64 %95, ptr addrspace(5) %"151", align 8
  %96 = load i64, ptr addrspace(5) %"151", align 8
  %"286" = mul i64 %96, 16
  store i64 %"286", ptr addrspace(5) %"151", align 8
  %97 = load i64, ptr addrspace(5) %"150", align 8
  %98 = load i64, ptr addrspace(5) %"151", align 8
  %"288" = add i64 %97, %98
  store i64 %"288", ptr addrspace(5) %"150", align 8
  %99 = load i64, ptr addrspace(5) %"150", align 8
  %100 = load i32, ptr addrspace(5) %"179", align 4
  %"332" = inttoptr i64 %99 to ptr
  %"333" = bitcast i32 %100 to float
  store float %"333", ptr %"332", align 4
  %101 = load i64, ptr addrspace(5) %"150", align 8
  %"334" = inttoptr i64 %101 to ptr
  %"109" = getelementptr inbounds i8, ptr %"334", i64 4
  %102 = load i32, ptr addrspace(5) %"180", align 4
  %"335" = bitcast i32 %102 to float
  store float %"335", ptr %"109", align 4
  %103 = load i64, ptr addrspace(5) %"150", align 8
  %"336" = inttoptr i64 %103 to ptr
  %"111" = getelementptr inbounds i8, ptr %"336", i64 8
  %104 = load i32, ptr addrspace(5) %"181", align 4
  %"337" = bitcast i32 %104 to float
  store float %"337", ptr %"111", align 4
  %105 = load i64, ptr addrspace(5) %"150", align 8
  %"338" = inttoptr i64 %105 to ptr
  %"113" = getelementptr inbounds i8, ptr %"338", i64 12
  %106 = load i32, ptr addrspace(5) %"182", align 4
  %"339" = bitcast i32 %106 to float
  store float %"339", ptr %"113", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
