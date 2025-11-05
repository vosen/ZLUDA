declare hidden <4 x i32> @__zluda_ptx_impl_mma_sync_aligned_m16n8k32_row_col_s32_s8_s8_s32(<4 x i32>, <2 x i32>, <4 x i32>) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define hidden i32 @pack_u8(<4 x i16> %"11") #0 {
  %"124" = alloca i32, align 4, addrspace(5)
  %"125" = alloca i32, align 4, addrspace(5)
  %"126" = alloca i32, align 4, addrspace(5)
  %"127" = alloca i32, align 4, addrspace(5)
  %"128" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"114"

"114":                                            ; preds = %1
  %"74" = extractelement <4 x i16> %"11", i8 0
  %"129" = zext i16 %"74" to i32
  store i32 %"129", ptr addrspace(5) %"125", align 4
  %"75" = extractelement <4 x i16> %"11", i8 1
  %"130" = zext i16 %"75" to i32
  store i32 %"130", ptr addrspace(5) %"126", align 4
  %"76" = extractelement <4 x i16> %"11", i8 2
  %"131" = zext i16 %"76" to i32
  store i32 %"131", ptr addrspace(5) %"127", align 4
  %"77" = extractelement <4 x i16> %"11", i8 3
  %"132" = zext i16 %"77" to i32
  store i32 %"132", ptr addrspace(5) %"128", align 4
  %2 = load i32, ptr addrspace(5) %"126", align 4
  %3 = shl i32 %2, 8
  %"300" = select i1 false, i32 0, i32 %3
  store i32 %"300", ptr addrspace(5) %"126", align 4
  %4 = load i32, ptr addrspace(5) %"127", align 4
  %5 = shl i32 %4, 16
  %"302" = select i1 false, i32 0, i32 %5
  store i32 %"302", ptr addrspace(5) %"127", align 4
  %6 = load i32, ptr addrspace(5) %"128", align 4
  %7 = shl i32 %6, 24
  %"304" = select i1 false, i32 0, i32 %7
  store i32 %"304", ptr addrspace(5) %"128", align 4
  %8 = load i32, ptr addrspace(5) %"125", align 4
  store i32 %8, ptr addrspace(5) %"124", align 4
  %9 = load i32, ptr addrspace(5) %"124", align 4
  %10 = load i32, ptr addrspace(5) %"126", align 4
  %"308" = or i32 %9, %10
  store i32 %"308", ptr addrspace(5) %"124", align 4
  %11 = load i32, ptr addrspace(5) %"124", align 4
  %12 = load i32, ptr addrspace(5) %"127", align 4
  %"311" = or i32 %11, %12
  store i32 %"311", ptr addrspace(5) %"124", align 4
  %13 = load i32, ptr addrspace(5) %"124", align 4
  %14 = load i32, ptr addrspace(5) %"128", align 4
  %"314" = or i32 %13, %14
  store i32 %"314", ptr addrspace(5) %"124", align 4
  %15 = load i32, ptr addrspace(5) %"124", align 4
  ret i32 %15
}

define amdgpu_kernel void @mma_m16n8k32_s32_s8_s8_s32(ptr addrspace(4) byref(i64) %"150") #1 {
  %"151" = alloca i64, align 8, addrspace(5)
  %"152" = alloca i64, align 8, addrspace(5)
  %"153" = alloca i32, align 4, addrspace(5)
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
  %"169" = alloca i16, align 2, addrspace(5)
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
  %"183" = alloca i32, align 4, addrspace(5)
  %"220" = alloca <4 x i16>, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"115"

"115":                                            ; preds = %1
  %2 = load i64, ptr addrspace(4) %"150", align 8
  store i64 %2, ptr addrspace(5) %"151", align 8
  %"73" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"116"

"116":                                            ; preds = %"115"
  store i32 %"73", ptr addrspace(5) %"153", align 4
  %3 = load i32, ptr addrspace(5) %"153", align 4
  %"186" = trunc i32 %3 to i16
  store i16 %"186", ptr addrspace(5) %"154", align 2
  %4 = load i16, ptr addrspace(5) %"154", align 2
  %"188" = mul i16 %4, 16
  store i16 %"188", ptr addrspace(5) %"154", align 2
  %5 = load i16, ptr addrspace(5) %"154", align 2
  %"190" = add i16 %5, 1
  store i16 %"190", ptr addrspace(5) %"155", align 2
  %6 = load i16, ptr addrspace(5) %"154", align 2
  %"192" = add i16 %6, 2
  store i16 %"192", ptr addrspace(5) %"156", align 2
  %7 = load i16, ptr addrspace(5) %"154", align 2
  %"194" = add i16 %7, 3
  store i16 %"194", ptr addrspace(5) %"157", align 2
  %8 = load i16, ptr addrspace(5) %"154", align 2
  %"196" = add i16 %8, 4
  store i16 %"196", ptr addrspace(5) %"158", align 2
  %9 = load i16, ptr addrspace(5) %"154", align 2
  %"198" = add i16 %9, 5
  store i16 %"198", ptr addrspace(5) %"159", align 2
  %10 = load i16, ptr addrspace(5) %"154", align 2
  %"200" = add i16 %10, 6
  store i16 %"200", ptr addrspace(5) %"160", align 2
  %11 = load i16, ptr addrspace(5) %"154", align 2
  %"202" = add i16 %11, 7
  store i16 %"202", ptr addrspace(5) %"161", align 2
  %12 = load i16, ptr addrspace(5) %"154", align 2
  %"204" = add i16 %12, 8
  store i16 %"204", ptr addrspace(5) %"162", align 2
  %13 = load i16, ptr addrspace(5) %"154", align 2
  %"206" = add i16 %13, 9
  store i16 %"206", ptr addrspace(5) %"163", align 2
  %14 = load i16, ptr addrspace(5) %"154", align 2
  %"208" = add i16 %14, 10
  store i16 %"208", ptr addrspace(5) %"164", align 2
  %15 = load i16, ptr addrspace(5) %"154", align 2
  %"210" = add i16 %15, 11
  store i16 %"210", ptr addrspace(5) %"165", align 2
  %16 = load i16, ptr addrspace(5) %"154", align 2
  %"212" = add i16 %16, 12
  store i16 %"212", ptr addrspace(5) %"166", align 2
  %17 = load i16, ptr addrspace(5) %"154", align 2
  %"214" = add i16 %17, 13
  store i16 %"214", ptr addrspace(5) %"167", align 2
  %18 = load i16, ptr addrspace(5) %"154", align 2
  %"216" = add i16 %18, 14
  store i16 %"216", ptr addrspace(5) %"168", align 2
  %19 = load i16, ptr addrspace(5) %"154", align 2
  %"218" = add i16 %19, 15
  store i16 %"218", ptr addrspace(5) %"169", align 2
  %20 = load i16, ptr addrspace(5) %"154", align 2
  %21 = load i16, ptr addrspace(5) %"155", align 2
  %22 = load i16, ptr addrspace(5) %"156", align 2
  %23 = load i16, ptr addrspace(5) %"157", align 2
  %24 = insertelement <4 x i16> undef, i16 %20, i8 0
  %25 = insertelement <4 x i16> %24, i16 %21, i8 1
  %26 = insertelement <4 x i16> %25, i16 %22, i8 2
  %"97" = insertelement <4 x i16> %26, i16 %23, i8 3
  store <4 x i16> %"97", ptr addrspace(5) %"220", align 8
  %27 = load <4 x i16>, ptr addrspace(5) %"220", align 8
  %"317" = call i32 @pack_u8(<4 x i16> %27)
  store i32 %"317", ptr addrspace(5) %"174", align 4
  br label %"117"

"117":                                            ; preds = %"116"
  %28 = load i16, ptr addrspace(5) %"158", align 2
  %29 = load i16, ptr addrspace(5) %"159", align 2
  %30 = load i16, ptr addrspace(5) %"160", align 2
  %31 = load i16, ptr addrspace(5) %"161", align 2
  %32 = insertelement <4 x i16> undef, i16 %28, i8 0
  %33 = insertelement <4 x i16> %32, i16 %29, i8 1
  %34 = insertelement <4 x i16> %33, i16 %30, i8 2
  %"98" = insertelement <4 x i16> %34, i16 %31, i8 3
  store <4 x i16> %"98", ptr addrspace(5) %"220", align 8
  %35 = load <4 x i16>, ptr addrspace(5) %"220", align 8
  %"318" = call i32 @pack_u8(<4 x i16> %35)
  store i32 %"318", ptr addrspace(5) %"175", align 4
  br label %"118"

"118":                                            ; preds = %"117"
  %36 = load i16, ptr addrspace(5) %"162", align 2
  %37 = load i16, ptr addrspace(5) %"163", align 2
  %38 = load i16, ptr addrspace(5) %"164", align 2
  %39 = load i16, ptr addrspace(5) %"165", align 2
  %40 = insertelement <4 x i16> undef, i16 %36, i8 0
  %41 = insertelement <4 x i16> %40, i16 %37, i8 1
  %42 = insertelement <4 x i16> %41, i16 %38, i8 2
  %"99" = insertelement <4 x i16> %42, i16 %39, i8 3
  store <4 x i16> %"99", ptr addrspace(5) %"220", align 8
  %43 = load <4 x i16>, ptr addrspace(5) %"220", align 8
  %"319" = call i32 @pack_u8(<4 x i16> %43)
  store i32 %"319", ptr addrspace(5) %"176", align 4
  br label %"119"

"119":                                            ; preds = %"118"
  %44 = load i16, ptr addrspace(5) %"166", align 2
  %45 = load i16, ptr addrspace(5) %"167", align 2
  %46 = load i16, ptr addrspace(5) %"168", align 2
  %47 = load i16, ptr addrspace(5) %"169", align 2
  %48 = insertelement <4 x i16> undef, i16 %44, i8 0
  %49 = insertelement <4 x i16> %48, i16 %45, i8 1
  %50 = insertelement <4 x i16> %49, i16 %46, i8 2
  %"100" = insertelement <4 x i16> %50, i16 %47, i8 3
  store <4 x i16> %"100", ptr addrspace(5) %"220", align 8
  %51 = load <4 x i16>, ptr addrspace(5) %"220", align 8
  %"320" = call i32 @pack_u8(<4 x i16> %51)
  store i32 %"320", ptr addrspace(5) %"177", align 4
  br label %"120"

"120":                                            ; preds = %"119"
  %52 = load i16, ptr addrspace(5) %"154", align 2
  %53 = load i16, ptr addrspace(5) %"155", align 2
  %54 = load i16, ptr addrspace(5) %"156", align 2
  %55 = load i16, ptr addrspace(5) %"157", align 2
  %56 = insertelement <4 x i16> undef, i16 %52, i8 0
  %57 = insertelement <4 x i16> %56, i16 %53, i8 1
  %58 = insertelement <4 x i16> %57, i16 %54, i8 2
  %"101" = insertelement <4 x i16> %58, i16 %55, i8 3
  store <4 x i16> %"101", ptr addrspace(5) %"220", align 8
  %59 = load <4 x i16>, ptr addrspace(5) %"220", align 8
  %"321" = call i32 @pack_u8(<4 x i16> %59)
  store i32 %"321", ptr addrspace(5) %"178", align 4
  br label %"121"

"121":                                            ; preds = %"120"
  %60 = load i16, ptr addrspace(5) %"154", align 2
  %61 = load i16, ptr addrspace(5) %"155", align 2
  %62 = load i16, ptr addrspace(5) %"156", align 2
  %63 = load i16, ptr addrspace(5) %"157", align 2
  %64 = insertelement <4 x i16> undef, i16 %60, i8 0
  %65 = insertelement <4 x i16> %64, i16 %61, i8 1
  %66 = insertelement <4 x i16> %65, i16 %62, i8 2
  %"102" = insertelement <4 x i16> %66, i16 %63, i8 3
  store <4 x i16> %"102", ptr addrspace(5) %"220", align 8
  %67 = load <4 x i16>, ptr addrspace(5) %"220", align 8
  %"322" = call i32 @pack_u8(<4 x i16> %67)
  store i32 %"322", ptr addrspace(5) %"179", align 4
  br label %"122"

"122":                                            ; preds = %"121"
  %68 = load i16, ptr addrspace(5) %"154", align 2
  %"263" = zext i16 %68 to i32
  store i32 %"263", ptr addrspace(5) %"170", align 4
  %69 = load i16, ptr addrspace(5) %"155", align 2
  %"265" = zext i16 %69 to i32
  store i32 %"265", ptr addrspace(5) %"171", align 4
  %70 = load i16, ptr addrspace(5) %"156", align 2
  %"267" = zext i16 %70 to i32
  store i32 %"267", ptr addrspace(5) %"172", align 4
  %71 = load i16, ptr addrspace(5) %"157", align 2
  %"269" = zext i16 %71 to i32
  store i32 %"269", ptr addrspace(5) %"173", align 4
  %72 = load i32, ptr addrspace(5) %"174", align 4
  %73 = load i32, ptr addrspace(5) %"175", align 4
  %74 = load i32, ptr addrspace(5) %"176", align 4
  %75 = load i32, ptr addrspace(5) %"177", align 4
  %76 = insertelement <4 x i32> undef, i32 %72, i8 0
  %77 = insertelement <4 x i32> %76, i32 %73, i8 1
  %78 = insertelement <4 x i32> %77, i32 %74, i8 2
  %"104" = insertelement <4 x i32> %78, i32 %75, i8 3
  %79 = load i32, ptr addrspace(5) %"178", align 4
  %80 = load i32, ptr addrspace(5) %"179", align 4
  %81 = insertelement <2 x i32> undef, i32 %79, i8 0
  %"105" = insertelement <2 x i32> %81, i32 %80, i8 1
  %82 = load i32, ptr addrspace(5) %"170", align 4
  %83 = load i32, ptr addrspace(5) %"171", align 4
  %84 = load i32, ptr addrspace(5) %"172", align 4
  %85 = load i32, ptr addrspace(5) %"173", align 4
  %86 = insertelement <4 x i32> undef, i32 %82, i8 0
  %87 = insertelement <4 x i32> %86, i32 %83, i8 1
  %88 = insertelement <4 x i32> %87, i32 %84, i8 2
  %"106" = insertelement <4 x i32> %88, i32 %85, i8 3
  %"103" = call <4 x i32> @__zluda_ptx_impl_mma_sync_aligned_m16n8k32_row_col_s32_s8_s8_s32(<4 x i32> %"104", <2 x i32> %"105", <4 x i32> %"106")
  %"329" = extractelement <4 x i32> %"103", i8 0
  %"330" = extractelement <4 x i32> %"103", i8 1
  %"331" = extractelement <4 x i32> %"103", i8 2
  %"332" = extractelement <4 x i32> %"103", i8 3
  store i32 %"329", ptr addrspace(5) %"180", align 4
  store i32 %"330", ptr addrspace(5) %"181", align 4
  store i32 %"331", ptr addrspace(5) %"182", align 4
  store i32 %"332", ptr addrspace(5) %"183", align 4
  %89 = load i32, ptr addrspace(5) %"153", align 4
  %"285" = zext i32 %89 to i64
  store i64 %"285", ptr addrspace(5) %"152", align 8
  %90 = load i64, ptr addrspace(5) %"152", align 8
  %"287" = mul i64 %90, 16
  store i64 %"287", ptr addrspace(5) %"152", align 8
  %91 = load i64, ptr addrspace(5) %"151", align 8
  %92 = load i64, ptr addrspace(5) %"152", align 8
  %"289" = add i64 %91, %92
  store i64 %"289", ptr addrspace(5) %"151", align 8
  %93 = load i64, ptr addrspace(5) %"151", align 8
  %94 = load i32, ptr addrspace(5) %"180", align 4
  %"333" = inttoptr i64 %93 to ptr
  %"334" = bitcast i32 %94 to float
  store float %"334", ptr %"333", align 4
  %95 = load i64, ptr addrspace(5) %"151", align 8
  %"335" = inttoptr i64 %95 to ptr
  %"109" = getelementptr inbounds i8, ptr %"335", i64 4
  %96 = load i32, ptr addrspace(5) %"181", align 4
  %"336" = bitcast i32 %96 to float
  store float %"336", ptr %"109", align 4
  %97 = load i64, ptr addrspace(5) %"151", align 8
  %"337" = inttoptr i64 %97 to ptr
  %"111" = getelementptr inbounds i8, ptr %"337", i64 8
  %98 = load i32, ptr addrspace(5) %"182", align 4
  %"338" = bitcast i32 %98 to float
  store float %"338", ptr %"111", align 4
  %99 = load i64, ptr addrspace(5) %"151", align 8
  %"339" = inttoptr i64 %99 to ptr
  %"113" = getelementptr inbounds i8, ptr %"339", i64 12
  %100 = load i32, ptr addrspace(5) %"183", align 4
  %"340" = bitcast i32 %100 to float
  store float %"340", ptr %"113", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }