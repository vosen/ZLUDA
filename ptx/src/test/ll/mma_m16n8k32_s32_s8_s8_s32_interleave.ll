declare hidden <4 x i32> @__zluda_ptx_impl_mma_sync_aligned_m16n8k32_row_col_s32_s8_s8_s32(<4 x i32>, <2 x i32>, <4 x i32>) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define hidden i32 @pack_u8(<4 x i16> %"11") #0 {
  %"171" = alloca i32, align 4, addrspace(5)
  %"172" = alloca i32, align 4, addrspace(5)
  %"173" = alloca i32, align 4, addrspace(5)
  %"174" = alloca i32, align 4, addrspace(5)
  %"175" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"162"

"162":                                            ; preds = %1
  %"86" = extractelement <4 x i16> %"11", i8 0
  %2 = zext i16 %"86" to i32
  store i32 %2, ptr addrspace(5) %"172", align 4
  %"87" = extractelement <4 x i16> %"11", i8 1
  %3 = zext i16 %"87" to i32
  store i32 %3, ptr addrspace(5) %"173", align 4
  %"88" = extractelement <4 x i16> %"11", i8 2
  %4 = zext i16 %"88" to i32
  store i32 %4, ptr addrspace(5) %"174", align 4
  %"89" = extractelement <4 x i16> %"11", i8 3
  %5 = zext i16 %"89" to i32
  store i32 %5, ptr addrspace(5) %"175", align 4
  %6 = load i32, ptr addrspace(5) %"173", align 4
  %7 = shl i32 %6, 8
  %"425" = select i1 false, i32 0, i32 %7
  store i32 %"425", ptr addrspace(5) %"173", align 4
  %8 = load i32, ptr addrspace(5) %"174", align 4
  %9 = shl i32 %8, 16
  %"427" = select i1 false, i32 0, i32 %9
  store i32 %"427", ptr addrspace(5) %"174", align 4
  %10 = load i32, ptr addrspace(5) %"175", align 4
  %11 = shl i32 %10, 24
  %"429" = select i1 false, i32 0, i32 %11
  store i32 %"429", ptr addrspace(5) %"175", align 4
  %12 = load i32, ptr addrspace(5) %"172", align 4
  store i32 %12, ptr addrspace(5) %"171", align 4
  %13 = load i32, ptr addrspace(5) %"171", align 4
  %14 = load i32, ptr addrspace(5) %"173", align 4
  %"433" = or i32 %13, %14
  store i32 %"433", ptr addrspace(5) %"171", align 4
  %15 = load i32, ptr addrspace(5) %"171", align 4
  %16 = load i32, ptr addrspace(5) %"174", align 4
  %"436" = or i32 %15, %16
  store i32 %"436", ptr addrspace(5) %"171", align 4
  %17 = load i32, ptr addrspace(5) %"171", align 4
  %18 = load i32, ptr addrspace(5) %"175", align 4
  %"439" = or i32 %17, %18
  store i32 %"439", ptr addrspace(5) %"171", align 4
  %19 = load i32, ptr addrspace(5) %"171", align 4
  ret i32 %19
}

define amdgpu_kernel void @mma_m16n8k32_s32_s8_s8_s32_interleave(ptr addrspace(4) byref(i64) %"197") #1 {
  %"198" = alloca i64, align 8, addrspace(5)
  %"199" = alloca i64, align 8, addrspace(5)
  %"200" = alloca i32, align 4, addrspace(5)
  %"201" = alloca i16, align 2, addrspace(5)
  %"202" = alloca i16, align 2, addrspace(5)
  %"203" = alloca i16, align 2, addrspace(5)
  %"204" = alloca i16, align 2, addrspace(5)
  %"205" = alloca i16, align 2, addrspace(5)
  %"206" = alloca i16, align 2, addrspace(5)
  %"207" = alloca i16, align 2, addrspace(5)
  %"208" = alloca i16, align 2, addrspace(5)
  %"209" = alloca i16, align 2, addrspace(5)
  %"210" = alloca i16, align 2, addrspace(5)
  %"211" = alloca i16, align 2, addrspace(5)
  %"212" = alloca i16, align 2, addrspace(5)
  %"213" = alloca i16, align 2, addrspace(5)
  %"214" = alloca i16, align 2, addrspace(5)
  %"215" = alloca i16, align 2, addrspace(5)
  %"216" = alloca i16, align 2, addrspace(5)
  %"217" = alloca i32, align 4, addrspace(5)
  %"218" = alloca i32, align 4, addrspace(5)
  %"219" = alloca i32, align 4, addrspace(5)
  %"220" = alloca i32, align 4, addrspace(5)
  %"221" = alloca i32, align 4, addrspace(5)
  %"222" = alloca i32, align 4, addrspace(5)
  %"223" = alloca i32, align 4, addrspace(5)
  %"224" = alloca i32, align 4, addrspace(5)
  %"225" = alloca i32, align 4, addrspace(5)
  %"226" = alloca i32, align 4, addrspace(5)
  %"227" = alloca i32, align 4, addrspace(5)
  %"228" = alloca i32, align 4, addrspace(5)
  %"229" = alloca i32, align 4, addrspace(5)
  %"230" = alloca i32, align 4, addrspace(5)
  %"231" = alloca i32, align 4, addrspace(5)
  %"232" = alloca i32, align 4, addrspace(5)
  %"233" = alloca i32, align 4, addrspace(5)
  %"234" = alloca i32, align 4, addrspace(5)
  %"235" = alloca i32, align 4, addrspace(5)
  %"236" = alloca i32, align 4, addrspace(5)
  %"237" = alloca i32, align 4, addrspace(5)
  %"238" = alloca i32, align 4, addrspace(5)
  %"239" = alloca i32, align 4, addrspace(5)
  %"240" = alloca i32, align 4, addrspace(5)
  %"241" = alloca i32, align 4, addrspace(5)
  %"242" = alloca i32, align 4, addrspace(5)
  %"279" = alloca <4 x i16>, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"163"

"163":                                            ; preds = %1
  %2 = load i64, ptr addrspace(4) %"197", align 8
  store i64 %2, ptr addrspace(5) %"198", align 8
  %"85" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"85", ptr addrspace(5) %"200", align 4
  %3 = load i32, ptr addrspace(5) %"200", align 4
  %4 = trunc i32 %3 to i16
  store i16 %4, ptr addrspace(5) %"201", align 2
  %5 = load i16, ptr addrspace(5) %"201", align 2
  %"247" = mul i16 %5, 16
  store i16 %"247", ptr addrspace(5) %"201", align 2
  %6 = load i16, ptr addrspace(5) %"201", align 2
  %"249" = add i16 %6, 1
  store i16 %"249", ptr addrspace(5) %"202", align 2
  %7 = load i16, ptr addrspace(5) %"201", align 2
  %"251" = add i16 %7, 2
  store i16 %"251", ptr addrspace(5) %"203", align 2
  %8 = load i16, ptr addrspace(5) %"201", align 2
  %"253" = add i16 %8, 3
  store i16 %"253", ptr addrspace(5) %"204", align 2
  %9 = load i16, ptr addrspace(5) %"201", align 2
  %"255" = add i16 %9, 4
  store i16 %"255", ptr addrspace(5) %"205", align 2
  %10 = load i16, ptr addrspace(5) %"201", align 2
  %"257" = add i16 %10, 5
  store i16 %"257", ptr addrspace(5) %"206", align 2
  %11 = load i16, ptr addrspace(5) %"201", align 2
  %"259" = add i16 %11, 6
  store i16 %"259", ptr addrspace(5) %"207", align 2
  %12 = load i16, ptr addrspace(5) %"201", align 2
  %"261" = add i16 %12, 7
  store i16 %"261", ptr addrspace(5) %"208", align 2
  %13 = load i16, ptr addrspace(5) %"201", align 2
  %"263" = add i16 %13, 8
  store i16 %"263", ptr addrspace(5) %"209", align 2
  %14 = load i16, ptr addrspace(5) %"201", align 2
  %"265" = add i16 %14, 9
  store i16 %"265", ptr addrspace(5) %"210", align 2
  %15 = load i16, ptr addrspace(5) %"201", align 2
  %"267" = add i16 %15, 10
  store i16 %"267", ptr addrspace(5) %"211", align 2
  %16 = load i16, ptr addrspace(5) %"201", align 2
  %"269" = add i16 %16, 11
  store i16 %"269", ptr addrspace(5) %"212", align 2
  %17 = load i16, ptr addrspace(5) %"201", align 2
  %"271" = add i16 %17, 12
  store i16 %"271", ptr addrspace(5) %"213", align 2
  %18 = load i16, ptr addrspace(5) %"201", align 2
  %"273" = add i16 %18, 13
  store i16 %"273", ptr addrspace(5) %"214", align 2
  %19 = load i16, ptr addrspace(5) %"201", align 2
  %"275" = add i16 %19, 14
  store i16 %"275", ptr addrspace(5) %"215", align 2
  %20 = load i16, ptr addrspace(5) %"201", align 2
  %"277" = add i16 %20, 15
  store i16 %"277", ptr addrspace(5) %"216", align 2
  %21 = load i16, ptr addrspace(5) %"201", align 2
  %22 = load i16, ptr addrspace(5) %"202", align 2
  %23 = load i16, ptr addrspace(5) %"203", align 2
  %24 = load i16, ptr addrspace(5) %"204", align 2
  %25 = insertelement <4 x i16> undef, i16 %21, i8 0
  %26 = insertelement <4 x i16> %25, i16 %22, i8 1
  %27 = insertelement <4 x i16> %26, i16 %23, i8 2
  %"109" = insertelement <4 x i16> %27, i16 %24, i8 3
  store <4 x i16> %"109", ptr addrspace(5) %"279", align 8
  %28 = load <4 x i16>, ptr addrspace(5) %"279", align 8
  %"442" = call i32 @pack_u8(<4 x i16> %28)
  store i32 %"442", ptr addrspace(5) %"221", align 4
  br label %"164"

"164":                                            ; preds = %"163"
  %29 = load i16, ptr addrspace(5) %"205", align 2
  %30 = load i16, ptr addrspace(5) %"206", align 2
  %31 = load i16, ptr addrspace(5) %"207", align 2
  %32 = load i16, ptr addrspace(5) %"208", align 2
  %33 = insertelement <4 x i16> undef, i16 %29, i8 0
  %34 = insertelement <4 x i16> %33, i16 %30, i8 1
  %35 = insertelement <4 x i16> %34, i16 %31, i8 2
  %"110" = insertelement <4 x i16> %35, i16 %32, i8 3
  store <4 x i16> %"110", ptr addrspace(5) %"279", align 8
  %36 = load <4 x i16>, ptr addrspace(5) %"279", align 8
  %"443" = call i32 @pack_u8(<4 x i16> %36)
  store i32 %"443", ptr addrspace(5) %"222", align 4
  br label %"165"

"165":                                            ; preds = %"164"
  %37 = load i16, ptr addrspace(5) %"209", align 2
  %38 = load i16, ptr addrspace(5) %"210", align 2
  %39 = load i16, ptr addrspace(5) %"211", align 2
  %40 = load i16, ptr addrspace(5) %"212", align 2
  %41 = insertelement <4 x i16> undef, i16 %37, i8 0
  %42 = insertelement <4 x i16> %41, i16 %38, i8 1
  %43 = insertelement <4 x i16> %42, i16 %39, i8 2
  %"111" = insertelement <4 x i16> %43, i16 %40, i8 3
  store <4 x i16> %"111", ptr addrspace(5) %"279", align 8
  %44 = load <4 x i16>, ptr addrspace(5) %"279", align 8
  %"444" = call i32 @pack_u8(<4 x i16> %44)
  store i32 %"444", ptr addrspace(5) %"223", align 4
  br label %"166"

"166":                                            ; preds = %"165"
  %45 = load i16, ptr addrspace(5) %"213", align 2
  %46 = load i16, ptr addrspace(5) %"214", align 2
  %47 = load i16, ptr addrspace(5) %"215", align 2
  %48 = load i16, ptr addrspace(5) %"216", align 2
  %49 = insertelement <4 x i16> undef, i16 %45, i8 0
  %50 = insertelement <4 x i16> %49, i16 %46, i8 1
  %51 = insertelement <4 x i16> %50, i16 %47, i8 2
  %"112" = insertelement <4 x i16> %51, i16 %48, i8 3
  store <4 x i16> %"112", ptr addrspace(5) %"279", align 8
  %52 = load <4 x i16>, ptr addrspace(5) %"279", align 8
  %"445" = call i32 @pack_u8(<4 x i16> %52)
  store i32 %"445", ptr addrspace(5) %"224", align 4
  br label %"167"

"167":                                            ; preds = %"166"
  %53 = load i16, ptr addrspace(5) %"201", align 2
  %54 = load i16, ptr addrspace(5) %"202", align 2
  %55 = load i16, ptr addrspace(5) %"203", align 2
  %56 = load i16, ptr addrspace(5) %"204", align 2
  %57 = insertelement <4 x i16> undef, i16 %53, i8 0
  %58 = insertelement <4 x i16> %57, i16 %54, i8 1
  %59 = insertelement <4 x i16> %58, i16 %55, i8 2
  %"113" = insertelement <4 x i16> %59, i16 %56, i8 3
  store <4 x i16> %"113", ptr addrspace(5) %"279", align 8
  %60 = load <4 x i16>, ptr addrspace(5) %"279", align 8
  %"446" = call i32 @pack_u8(<4 x i16> %60)
  store i32 %"446", ptr addrspace(5) %"225", align 4
  br label %"168"

"168":                                            ; preds = %"167"
  %61 = load i16, ptr addrspace(5) %"205", align 2
  %62 = load i16, ptr addrspace(5) %"206", align 2
  %63 = load i16, ptr addrspace(5) %"207", align 2
  %64 = load i16, ptr addrspace(5) %"208", align 2
  %65 = insertelement <4 x i16> undef, i16 %61, i8 0
  %66 = insertelement <4 x i16> %65, i16 %62, i8 1
  %67 = insertelement <4 x i16> %66, i16 %63, i8 2
  %"114" = insertelement <4 x i16> %67, i16 %64, i8 3
  store <4 x i16> %"114", ptr addrspace(5) %"279", align 8
  %68 = load <4 x i16>, ptr addrspace(5) %"279", align 8
  %"447" = call i32 @pack_u8(<4 x i16> %68)
  store i32 %"447", ptr addrspace(5) %"226", align 4
  br label %"169"

"169":                                            ; preds = %"168"
  %69 = load i16, ptr addrspace(5) %"201", align 2
  %70 = zext i16 %69 to i32
  store i32 %70, ptr addrspace(5) %"217", align 4
  %71 = load i16, ptr addrspace(5) %"202", align 2
  %72 = zext i16 %71 to i32
  store i32 %72, ptr addrspace(5) %"218", align 4
  %73 = load i16, ptr addrspace(5) %"203", align 2
  %74 = zext i16 %73 to i32
  store i32 %74, ptr addrspace(5) %"219", align 4
  %75 = load i16, ptr addrspace(5) %"204", align 2
  %76 = zext i16 %75 to i32
  store i32 %76, ptr addrspace(5) %"220", align 4
  %77 = load i32, ptr addrspace(5) %"221", align 4
  %78 = load i32, ptr addrspace(5) %"222", align 4
  %79 = load i32, ptr addrspace(5) %"223", align 4
  %80 = load i32, ptr addrspace(5) %"224", align 4
  %81 = insertelement <4 x i32> undef, i32 %77, i8 0
  %82 = insertelement <4 x i32> %81, i32 %78, i8 1
  %83 = insertelement <4 x i32> %82, i32 %79, i8 2
  %"116" = insertelement <4 x i32> %83, i32 %80, i8 3
  %84 = load i32, ptr addrspace(5) %"225", align 4
  %85 = load i32, ptr addrspace(5) %"226", align 4
  %86 = insertelement <2 x i32> undef, i32 %84, i8 0
  %"117" = insertelement <2 x i32> %86, i32 %85, i8 1
  %87 = load i32, ptr addrspace(5) %"217", align 4
  %88 = load i32, ptr addrspace(5) %"218", align 4
  %89 = load i32, ptr addrspace(5) %"219", align 4
  %90 = load i32, ptr addrspace(5) %"220", align 4
  %91 = insertelement <4 x i32> undef, i32 %87, i8 0
  %92 = insertelement <4 x i32> %91, i32 %88, i8 1
  %93 = insertelement <4 x i32> %92, i32 %89, i8 2
  %"118" = insertelement <4 x i32> %93, i32 %90, i8 3
  %"115" = call <4 x i32> @__zluda_ptx_impl_mma_sync_aligned_m16n8k32_row_col_s32_s8_s8_s32(<4 x i32> %"116", <2 x i32> %"117", <4 x i32> %"118")
  %"454" = extractelement <4 x i32> %"115", i8 0
  %"455" = extractelement <4 x i32> %"115", i8 1
  %"456" = extractelement <4 x i32> %"115", i8 2
  %"457" = extractelement <4 x i32> %"115", i8 3
  store i32 %"454", ptr addrspace(5) %"227", align 4
  store i32 %"455", ptr addrspace(5) %"228", align 4
  store i32 %"456", ptr addrspace(5) %"229", align 4
  store i32 %"457", ptr addrspace(5) %"230", align 4
  %94 = load i32, ptr addrspace(5) %"224", align 4
  %95 = load i32, ptr addrspace(5) %"223", align 4
  %96 = load i32, ptr addrspace(5) %"222", align 4
  %97 = load i32, ptr addrspace(5) %"221", align 4
  %98 = insertelement <4 x i32> undef, i32 %94, i8 0
  %99 = insertelement <4 x i32> %98, i32 %95, i8 1
  %100 = insertelement <4 x i32> %99, i32 %96, i8 2
  %"120" = insertelement <4 x i32> %100, i32 %97, i8 3
  %101 = load i32, ptr addrspace(5) %"226", align 4
  %102 = load i32, ptr addrspace(5) %"225", align 4
  %103 = insertelement <2 x i32> undef, i32 %101, i8 0
  %"121" = insertelement <2 x i32> %103, i32 %102, i8 1
  %104 = load i32, ptr addrspace(5) %"217", align 4
  %105 = load i32, ptr addrspace(5) %"218", align 4
  %106 = load i32, ptr addrspace(5) %"219", align 4
  %107 = load i32, ptr addrspace(5) %"220", align 4
  %108 = insertelement <4 x i32> undef, i32 %104, i8 0
  %109 = insertelement <4 x i32> %108, i32 %105, i8 1
  %110 = insertelement <4 x i32> %109, i32 %106, i8 2
  %"122" = insertelement <4 x i32> %110, i32 %107, i8 3
  %"119" = call <4 x i32> @__zluda_ptx_impl_mma_sync_aligned_m16n8k32_row_col_s32_s8_s8_s32(<4 x i32> %"120", <2 x i32> %"121", <4 x i32> %"122")
  %"464" = extractelement <4 x i32> %"119", i8 0
  %"465" = extractelement <4 x i32> %"119", i8 1
  %"466" = extractelement <4 x i32> %"119", i8 2
  %"467" = extractelement <4 x i32> %"119", i8 3
  store i32 %"464", ptr addrspace(5) %"231", align 4
  store i32 %"465", ptr addrspace(5) %"232", align 4
  store i32 %"466", ptr addrspace(5) %"233", align 4
  store i32 %"467", ptr addrspace(5) %"234", align 4
  %111 = load i32, ptr addrspace(5) %"221", align 4
  %112 = load i32, ptr addrspace(5) %"222", align 4
  %113 = load i32, ptr addrspace(5) %"223", align 4
  %114 = load i32, ptr addrspace(5) %"224", align 4
  %115 = insertelement <4 x i32> undef, i32 %111, i8 0
  %116 = insertelement <4 x i32> %115, i32 %112, i8 1
  %117 = insertelement <4 x i32> %116, i32 %113, i8 2
  %"124" = insertelement <4 x i32> %117, i32 %114, i8 3
  %118 = load i32, ptr addrspace(5) %"226", align 4
  %119 = load i32, ptr addrspace(5) %"225", align 4
  %120 = insertelement <2 x i32> undef, i32 %118, i8 0
  %"125" = insertelement <2 x i32> %120, i32 %119, i8 1
  %121 = load i32, ptr addrspace(5) %"217", align 4
  %122 = load i32, ptr addrspace(5) %"218", align 4
  %123 = load i32, ptr addrspace(5) %"219", align 4
  %124 = load i32, ptr addrspace(5) %"220", align 4
  %125 = insertelement <4 x i32> undef, i32 %121, i8 0
  %126 = insertelement <4 x i32> %125, i32 %122, i8 1
  %127 = insertelement <4 x i32> %126, i32 %123, i8 2
  %"126" = insertelement <4 x i32> %127, i32 %124, i8 3
  %"123" = call <4 x i32> @__zluda_ptx_impl_mma_sync_aligned_m16n8k32_row_col_s32_s8_s8_s32(<4 x i32> %"124", <2 x i32> %"125", <4 x i32> %"126")
  %"474" = extractelement <4 x i32> %"123", i8 0
  %"475" = extractelement <4 x i32> %"123", i8 1
  %"476" = extractelement <4 x i32> %"123", i8 2
  %"477" = extractelement <4 x i32> %"123", i8 3
  store i32 %"474", ptr addrspace(5) %"235", align 4
  store i32 %"475", ptr addrspace(5) %"236", align 4
  store i32 %"476", ptr addrspace(5) %"237", align 4
  store i32 %"477", ptr addrspace(5) %"238", align 4
  %128 = load i32, ptr addrspace(5) %"224", align 4
  %129 = load i32, ptr addrspace(5) %"223", align 4
  %130 = load i32, ptr addrspace(5) %"222", align 4
  %131 = load i32, ptr addrspace(5) %"221", align 4
  %132 = insertelement <4 x i32> undef, i32 %128, i8 0
  %133 = insertelement <4 x i32> %132, i32 %129, i8 1
  %134 = insertelement <4 x i32> %133, i32 %130, i8 2
  %"128" = insertelement <4 x i32> %134, i32 %131, i8 3
  %135 = load i32, ptr addrspace(5) %"225", align 4
  %136 = load i32, ptr addrspace(5) %"225", align 4
  %137 = insertelement <2 x i32> undef, i32 %135, i8 0
  %"129" = insertelement <2 x i32> %137, i32 %136, i8 1
  %138 = load i32, ptr addrspace(5) %"217", align 4
  %139 = load i32, ptr addrspace(5) %"218", align 4
  %140 = load i32, ptr addrspace(5) %"219", align 4
  %141 = load i32, ptr addrspace(5) %"220", align 4
  %142 = insertelement <4 x i32> undef, i32 %138, i8 0
  %143 = insertelement <4 x i32> %142, i32 %139, i8 1
  %144 = insertelement <4 x i32> %143, i32 %140, i8 2
  %"130" = insertelement <4 x i32> %144, i32 %141, i8 3
  %"127" = call <4 x i32> @__zluda_ptx_impl_mma_sync_aligned_m16n8k32_row_col_s32_s8_s8_s32(<4 x i32> %"128", <2 x i32> %"129", <4 x i32> %"130")
  %"484" = extractelement <4 x i32> %"127", i8 0
  %"485" = extractelement <4 x i32> %"127", i8 1
  %"486" = extractelement <4 x i32> %"127", i8 2
  %"487" = extractelement <4 x i32> %"127", i8 3
  store i32 %"484", ptr addrspace(5) %"239", align 4
  store i32 %"485", ptr addrspace(5) %"240", align 4
  store i32 %"486", ptr addrspace(5) %"241", align 4
  store i32 %"487", ptr addrspace(5) %"242", align 4
  %145 = load i32, ptr addrspace(5) %"200", align 4
  %146 = zext i32 %145 to i64
  store i64 %146, ptr addrspace(5) %"199", align 8
  %147 = load i64, ptr addrspace(5) %"199", align 8
  %"388" = mul i64 %147, 64
  store i64 %"388", ptr addrspace(5) %"199", align 8
  %148 = load i64, ptr addrspace(5) %"198", align 8
  %149 = load i64, ptr addrspace(5) %"199", align 8
  %"390" = add i64 %148, %149
  store i64 %"390", ptr addrspace(5) %"198", align 8
  %150 = load i64, ptr addrspace(5) %"198", align 8
  %151 = load i32, ptr addrspace(5) %"227", align 4
  %"488" = inttoptr i64 %150 to ptr
  store i32 %151, ptr %"488", align 4
  %152 = load i64, ptr addrspace(5) %"198", align 8
  %"489" = inttoptr i64 %152 to ptr
  %"133" = getelementptr inbounds i8, ptr %"489", i64 4
  %153 = load i32, ptr addrspace(5) %"228", align 4
  store i32 %153, ptr %"133", align 4
  %154 = load i64, ptr addrspace(5) %"198", align 8
  %"490" = inttoptr i64 %154 to ptr
  %"135" = getelementptr inbounds i8, ptr %"490", i64 8
  %155 = load i32, ptr addrspace(5) %"229", align 4
  store i32 %155, ptr %"135", align 4
  %156 = load i64, ptr addrspace(5) %"198", align 8
  %"491" = inttoptr i64 %156 to ptr
  %"137" = getelementptr inbounds i8, ptr %"491", i64 12
  %157 = load i32, ptr addrspace(5) %"230", align 4
  store i32 %157, ptr %"137", align 4
  %158 = load i64, ptr addrspace(5) %"198", align 8
  %"492" = inttoptr i64 %158 to ptr
  %"139" = getelementptr inbounds i8, ptr %"492", i64 16
  %159 = load i32, ptr addrspace(5) %"231", align 4
  store i32 %159, ptr %"139", align 4
  %160 = load i64, ptr addrspace(5) %"198", align 8
  %"493" = inttoptr i64 %160 to ptr
  %"141" = getelementptr inbounds i8, ptr %"493", i64 20
  %161 = load i32, ptr addrspace(5) %"232", align 4
  store i32 %161, ptr %"141", align 4
  %162 = load i64, ptr addrspace(5) %"198", align 8
  %"494" = inttoptr i64 %162 to ptr
  %"143" = getelementptr inbounds i8, ptr %"494", i64 24
  %163 = load i32, ptr addrspace(5) %"233", align 4
  store i32 %163, ptr %"143", align 4
  %164 = load i64, ptr addrspace(5) %"198", align 8
  %"495" = inttoptr i64 %164 to ptr
  %"145" = getelementptr inbounds i8, ptr %"495", i64 28
  %165 = load i32, ptr addrspace(5) %"234", align 4
  store i32 %165, ptr %"145", align 4
  %166 = load i64, ptr addrspace(5) %"198", align 8
  %"496" = inttoptr i64 %166 to ptr
  %"147" = getelementptr inbounds i8, ptr %"496", i64 32
  %167 = load i32, ptr addrspace(5) %"235", align 4
  store i32 %167, ptr %"147", align 4
  %168 = load i64, ptr addrspace(5) %"198", align 8
  %"497" = inttoptr i64 %168 to ptr
  %"149" = getelementptr inbounds i8, ptr %"497", i64 36
  %169 = load i32, ptr addrspace(5) %"236", align 4
  store i32 %169, ptr %"149", align 4
  %170 = load i64, ptr addrspace(5) %"198", align 8
  %"498" = inttoptr i64 %170 to ptr
  %"151" = getelementptr inbounds i8, ptr %"498", i64 40
  %171 = load i32, ptr addrspace(5) %"237", align 4
  store i32 %171, ptr %"151", align 4
  %172 = load i64, ptr addrspace(5) %"198", align 8
  %"499" = inttoptr i64 %172 to ptr
  %"153" = getelementptr inbounds i8, ptr %"499", i64 44
  %173 = load i32, ptr addrspace(5) %"238", align 4
  store i32 %173, ptr %"153", align 4
  %174 = load i64, ptr addrspace(5) %"198", align 8
  %"500" = inttoptr i64 %174 to ptr
  %"155" = getelementptr inbounds i8, ptr %"500", i64 48
  %175 = load i32, ptr addrspace(5) %"239", align 4
  store i32 %175, ptr %"155", align 4
  %176 = load i64, ptr addrspace(5) %"198", align 8
  %"501" = inttoptr i64 %176 to ptr
  %"157" = getelementptr inbounds i8, ptr %"501", i64 52
  %177 = load i32, ptr addrspace(5) %"240", align 4
  store i32 %177, ptr %"157", align 4
  %178 = load i64, ptr addrspace(5) %"198", align 8
  %"502" = inttoptr i64 %178 to ptr
  %"159" = getelementptr inbounds i8, ptr %"502", i64 56
  %179 = load i32, ptr addrspace(5) %"241", align 4
  store i32 %179, ptr %"159", align 4
  %180 = load i64, ptr addrspace(5) %"198", align 8
  %"503" = inttoptr i64 %180 to ptr
  %"161" = getelementptr inbounds i8, ptr %"503", i64 60
  %181 = load i32, ptr addrspace(5) %"242", align 4
  store i32 %181, ptr %"161", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }