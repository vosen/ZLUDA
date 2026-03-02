declare hidden <4 x i32> @__zluda_ptx_impl_mma_sync_aligned_m16n8k32_row_col_s32_s8_s8_s32(<4 x i32>, <2 x i32>, <4 x i32>) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define hidden i32 @pack_u8(<4 x i16> %"12") #0 {
  %"126" = alloca i32, align 4, addrspace(5)
  %"127" = alloca i32, align 4, addrspace(5)
  %"128" = alloca i32, align 4, addrspace(5)
  %"129" = alloca i32, align 4, addrspace(5)
  %"130" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"117"

"117":                                            ; preds = %1
  %"77" = extractelement <4 x i16> %"12", i8 0
  %2 = zext i16 %"77" to i32
  store i32 %2, ptr addrspace(5) %"127", align 4
  %"78" = extractelement <4 x i16> %"12", i8 1
  %3 = zext i16 %"78" to i32
  store i32 %3, ptr addrspace(5) %"128", align 4
  %"79" = extractelement <4 x i16> %"12", i8 2
  %4 = zext i16 %"79" to i32
  store i32 %4, ptr addrspace(5) %"129", align 4
  %"80" = extractelement <4 x i16> %"12", i8 3
  %5 = zext i16 %"80" to i32
  store i32 %5, ptr addrspace(5) %"130", align 4
  %6 = load i32, ptr addrspace(5) %"128", align 4
  %7 = shl i32 %6, 8
  %"302" = select i1 false, i32 0, i32 %7
  store i32 %"302", ptr addrspace(5) %"128", align 4
  %8 = load i32, ptr addrspace(5) %"129", align 4
  %9 = shl i32 %8, 16
  %"304" = select i1 false, i32 0, i32 %9
  store i32 %"304", ptr addrspace(5) %"129", align 4
  %10 = load i32, ptr addrspace(5) %"130", align 4
  %11 = shl i32 %10, 24
  %"306" = select i1 false, i32 0, i32 %11
  store i32 %"306", ptr addrspace(5) %"130", align 4
  %12 = load i32, ptr addrspace(5) %"127", align 4
  store i32 %12, ptr addrspace(5) %"126", align 4
  %13 = load i32, ptr addrspace(5) %"126", align 4
  %14 = load i32, ptr addrspace(5) %"128", align 4
  %"310" = or i32 %13, %14
  store i32 %"310", ptr addrspace(5) %"126", align 4
  %15 = load i32, ptr addrspace(5) %"126", align 4
  %16 = load i32, ptr addrspace(5) %"129", align 4
  %"313" = or i32 %15, %16
  store i32 %"313", ptr addrspace(5) %"126", align 4
  %17 = load i32, ptr addrspace(5) %"126", align 4
  %18 = load i32, ptr addrspace(5) %"130", align 4
  %"316" = or i32 %17, %18
  store i32 %"316", ptr addrspace(5) %"126", align 4
  %19 = load i32, ptr addrspace(5) %"126", align 4
  ret i32 %19
}

define amdgpu_kernel void @mma_m16n8k32_s32_s8_s8_s32(ptr addrspace(4) byref(i64) %"152") #1 {
  %"153" = alloca i64, align 8, addrspace(5)
  %"154" = alloca i64, align 8, addrspace(5)
  %"155" = alloca i32, align 4, addrspace(5)
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
  %"170" = alloca i16, align 2, addrspace(5)
  %"171" = alloca i16, align 2, addrspace(5)
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
  %"184" = alloca i32, align 4, addrspace(5)
  %"185" = alloca i32, align 4, addrspace(5)
  %"222" = alloca <4 x i16>, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"118"

"118":                                            ; preds = %1
  %2 = load i64, ptr addrspace(4) %"152", align 8
  store i64 %2, ptr addrspace(5) %"153", align 8
  %"76" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"76", ptr addrspace(5) %"155", align 4
  %3 = load i32, ptr addrspace(5) %"155", align 4
  %4 = trunc i32 %3 to i16
  store i16 %4, ptr addrspace(5) %"156", align 2
  %5 = load i16, ptr addrspace(5) %"156", align 2
  %"190" = mul i16 %5, 16
  store i16 %"190", ptr addrspace(5) %"156", align 2
  %6 = load i16, ptr addrspace(5) %"156", align 2
  %"192" = add i16 %6, 1
  store i16 %"192", ptr addrspace(5) %"157", align 2
  %7 = load i16, ptr addrspace(5) %"156", align 2
  %"194" = add i16 %7, 2
  store i16 %"194", ptr addrspace(5) %"158", align 2
  %8 = load i16, ptr addrspace(5) %"156", align 2
  %"196" = add i16 %8, 3
  store i16 %"196", ptr addrspace(5) %"159", align 2
  %9 = load i16, ptr addrspace(5) %"156", align 2
  %"198" = add i16 %9, 4
  store i16 %"198", ptr addrspace(5) %"160", align 2
  %10 = load i16, ptr addrspace(5) %"156", align 2
  %"200" = add i16 %10, 5
  store i16 %"200", ptr addrspace(5) %"161", align 2
  %11 = load i16, ptr addrspace(5) %"156", align 2
  %"202" = add i16 %11, 6
  store i16 %"202", ptr addrspace(5) %"162", align 2
  %12 = load i16, ptr addrspace(5) %"156", align 2
  %"204" = add i16 %12, 7
  store i16 %"204", ptr addrspace(5) %"163", align 2
  %13 = load i16, ptr addrspace(5) %"156", align 2
  %"206" = add i16 %13, 8
  store i16 %"206", ptr addrspace(5) %"164", align 2
  %14 = load i16, ptr addrspace(5) %"156", align 2
  %"208" = add i16 %14, 9
  store i16 %"208", ptr addrspace(5) %"165", align 2
  %15 = load i16, ptr addrspace(5) %"156", align 2
  %"210" = add i16 %15, 10
  store i16 %"210", ptr addrspace(5) %"166", align 2
  %16 = load i16, ptr addrspace(5) %"156", align 2
  %"212" = add i16 %16, 11
  store i16 %"212", ptr addrspace(5) %"167", align 2
  %17 = load i16, ptr addrspace(5) %"156", align 2
  %"214" = add i16 %17, 12
  store i16 %"214", ptr addrspace(5) %"168", align 2
  %18 = load i16, ptr addrspace(5) %"156", align 2
  %"216" = add i16 %18, 13
  store i16 %"216", ptr addrspace(5) %"169", align 2
  %19 = load i16, ptr addrspace(5) %"156", align 2
  %"218" = add i16 %19, 14
  store i16 %"218", ptr addrspace(5) %"170", align 2
  %20 = load i16, ptr addrspace(5) %"156", align 2
  %"220" = add i16 %20, 15
  store i16 %"220", ptr addrspace(5) %"171", align 2
  %21 = load i16, ptr addrspace(5) %"156", align 2
  %22 = load i16, ptr addrspace(5) %"157", align 2
  %23 = load i16, ptr addrspace(5) %"158", align 2
  %24 = load i16, ptr addrspace(5) %"159", align 2
  %25 = insertelement <4 x i16> undef, i16 %21, i8 0
  %26 = insertelement <4 x i16> %25, i16 %22, i8 1
  %27 = insertelement <4 x i16> %26, i16 %23, i8 2
  %"100" = insertelement <4 x i16> %27, i16 %24, i8 3
  store <4 x i16> %"100", ptr addrspace(5) %"222", align 8
  %28 = load <4 x i16>, ptr addrspace(5) %"222", align 8
  %"319" = call i32 @pack_u8(<4 x i16> %28)
  store i32 %"319", ptr addrspace(5) %"176", align 4
  br label %"119"

"119":                                            ; preds = %"118"
  %29 = load i16, ptr addrspace(5) %"160", align 2
  %30 = load i16, ptr addrspace(5) %"161", align 2
  %31 = load i16, ptr addrspace(5) %"162", align 2
  %32 = load i16, ptr addrspace(5) %"163", align 2
  %33 = insertelement <4 x i16> undef, i16 %29, i8 0
  %34 = insertelement <4 x i16> %33, i16 %30, i8 1
  %35 = insertelement <4 x i16> %34, i16 %31, i8 2
  %"101" = insertelement <4 x i16> %35, i16 %32, i8 3
  store <4 x i16> %"101", ptr addrspace(5) %"222", align 8
  %36 = load <4 x i16>, ptr addrspace(5) %"222", align 8
  %"320" = call i32 @pack_u8(<4 x i16> %36)
  store i32 %"320", ptr addrspace(5) %"177", align 4
  br label %"120"

"120":                                            ; preds = %"119"
  %37 = load i16, ptr addrspace(5) %"164", align 2
  %38 = load i16, ptr addrspace(5) %"165", align 2
  %39 = load i16, ptr addrspace(5) %"166", align 2
  %40 = load i16, ptr addrspace(5) %"167", align 2
  %41 = insertelement <4 x i16> undef, i16 %37, i8 0
  %42 = insertelement <4 x i16> %41, i16 %38, i8 1
  %43 = insertelement <4 x i16> %42, i16 %39, i8 2
  %"102" = insertelement <4 x i16> %43, i16 %40, i8 3
  store <4 x i16> %"102", ptr addrspace(5) %"222", align 8
  %44 = load <4 x i16>, ptr addrspace(5) %"222", align 8
  %"321" = call i32 @pack_u8(<4 x i16> %44)
  store i32 %"321", ptr addrspace(5) %"178", align 4
  br label %"121"

"121":                                            ; preds = %"120"
  %45 = load i16, ptr addrspace(5) %"168", align 2
  %46 = load i16, ptr addrspace(5) %"169", align 2
  %47 = load i16, ptr addrspace(5) %"170", align 2
  %48 = load i16, ptr addrspace(5) %"171", align 2
  %49 = insertelement <4 x i16> undef, i16 %45, i8 0
  %50 = insertelement <4 x i16> %49, i16 %46, i8 1
  %51 = insertelement <4 x i16> %50, i16 %47, i8 2
  %"103" = insertelement <4 x i16> %51, i16 %48, i8 3
  store <4 x i16> %"103", ptr addrspace(5) %"222", align 8
  %52 = load <4 x i16>, ptr addrspace(5) %"222", align 8
  %"322" = call i32 @pack_u8(<4 x i16> %52)
  store i32 %"322", ptr addrspace(5) %"179", align 4
  br label %"122"

"122":                                            ; preds = %"121"
  %53 = load i16, ptr addrspace(5) %"156", align 2
  %54 = load i16, ptr addrspace(5) %"157", align 2
  %55 = load i16, ptr addrspace(5) %"158", align 2
  %56 = load i16, ptr addrspace(5) %"159", align 2
  %57 = insertelement <4 x i16> undef, i16 %53, i8 0
  %58 = insertelement <4 x i16> %57, i16 %54, i8 1
  %59 = insertelement <4 x i16> %58, i16 %55, i8 2
  %"104" = insertelement <4 x i16> %59, i16 %56, i8 3
  store <4 x i16> %"104", ptr addrspace(5) %"222", align 8
  %60 = load <4 x i16>, ptr addrspace(5) %"222", align 8
  %"323" = call i32 @pack_u8(<4 x i16> %60)
  store i32 %"323", ptr addrspace(5) %"180", align 4
  br label %"123"

"123":                                            ; preds = %"122"
  %61 = load i16, ptr addrspace(5) %"156", align 2
  %62 = load i16, ptr addrspace(5) %"157", align 2
  %63 = load i16, ptr addrspace(5) %"158", align 2
  %64 = load i16, ptr addrspace(5) %"159", align 2
  %65 = insertelement <4 x i16> undef, i16 %61, i8 0
  %66 = insertelement <4 x i16> %65, i16 %62, i8 1
  %67 = insertelement <4 x i16> %66, i16 %63, i8 2
  %"105" = insertelement <4 x i16> %67, i16 %64, i8 3
  store <4 x i16> %"105", ptr addrspace(5) %"222", align 8
  %68 = load <4 x i16>, ptr addrspace(5) %"222", align 8
  %"324" = call i32 @pack_u8(<4 x i16> %68)
  store i32 %"324", ptr addrspace(5) %"181", align 4
  br label %"124"

"124":                                            ; preds = %"123"
  %69 = load i16, ptr addrspace(5) %"156", align 2
  %70 = zext i16 %69 to i32
  store i32 %70, ptr addrspace(5) %"172", align 4
  %71 = load i16, ptr addrspace(5) %"157", align 2
  %72 = zext i16 %71 to i32
  store i32 %72, ptr addrspace(5) %"173", align 4
  %73 = load i16, ptr addrspace(5) %"158", align 2
  %74 = zext i16 %73 to i32
  store i32 %74, ptr addrspace(5) %"174", align 4
  %75 = load i16, ptr addrspace(5) %"159", align 2
  %76 = zext i16 %75 to i32
  store i32 %76, ptr addrspace(5) %"175", align 4
  %77 = load i32, ptr addrspace(5) %"176", align 4
  %78 = load i32, ptr addrspace(5) %"177", align 4
  %79 = load i32, ptr addrspace(5) %"178", align 4
  %80 = load i32, ptr addrspace(5) %"179", align 4
  %81 = insertelement <4 x i32> undef, i32 %77, i8 0
  %82 = insertelement <4 x i32> %81, i32 %78, i8 1
  %83 = insertelement <4 x i32> %82, i32 %79, i8 2
  %"107" = insertelement <4 x i32> %83, i32 %80, i8 3
  %84 = load i32, ptr addrspace(5) %"180", align 4
  %85 = load i32, ptr addrspace(5) %"181", align 4
  %86 = insertelement <2 x i32> undef, i32 %84, i8 0
  %"108" = insertelement <2 x i32> %86, i32 %85, i8 1
  %87 = load i32, ptr addrspace(5) %"172", align 4
  %88 = load i32, ptr addrspace(5) %"173", align 4
  %89 = load i32, ptr addrspace(5) %"174", align 4
  %90 = load i32, ptr addrspace(5) %"175", align 4
  %91 = insertelement <4 x i32> undef, i32 %87, i8 0
  %92 = insertelement <4 x i32> %91, i32 %88, i8 1
  %93 = insertelement <4 x i32> %92, i32 %89, i8 2
  %"109" = insertelement <4 x i32> %93, i32 %90, i8 3
  %"106" = call <4 x i32> @__zluda_ptx_impl_mma_sync_aligned_m16n8k32_row_col_s32_s8_s8_s32(<4 x i32> %"107", <2 x i32> %"108", <4 x i32> %"109")
  %"331" = extractelement <4 x i32> %"106", i8 0
  %"332" = extractelement <4 x i32> %"106", i8 1
  %"333" = extractelement <4 x i32> %"106", i8 2
  %"334" = extractelement <4 x i32> %"106", i8 3
  store i32 %"331", ptr addrspace(5) %"182", align 4
  store i32 %"332", ptr addrspace(5) %"183", align 4
  store i32 %"333", ptr addrspace(5) %"184", align 4
  store i32 %"334", ptr addrspace(5) %"185", align 4
  %94 = load i32, ptr addrspace(5) %"155", align 4
  %95 = zext i32 %94 to i64
  store i64 %95, ptr addrspace(5) %"154", align 8
  %96 = load i64, ptr addrspace(5) %"154", align 8
  %"289" = mul i64 %96, 16
  store i64 %"289", ptr addrspace(5) %"154", align 8
  %97 = load i64, ptr addrspace(5) %"153", align 8
  %98 = load i64, ptr addrspace(5) %"154", align 8
  %"291" = add i64 %97, %98
  store i64 %"291", ptr addrspace(5) %"153", align 8
  %99 = load i64, ptr addrspace(5) %"153", align 8
  %100 = load i32, ptr addrspace(5) %"182", align 4
  %"335" = inttoptr i64 %99 to ptr
  %"336" = bitcast i32 %100 to float
  store float %"336", ptr %"335", align 4
  %101 = load i64, ptr addrspace(5) %"153", align 8
  %"337" = inttoptr i64 %101 to ptr
  %"112" = getelementptr inbounds i8, ptr %"337", i64 4
  %102 = load i32, ptr addrspace(5) %"183", align 4
  %"338" = bitcast i32 %102 to float
  store float %"338", ptr %"112", align 4
  %103 = load i64, ptr addrspace(5) %"153", align 8
  %"339" = inttoptr i64 %103 to ptr
  %"114" = getelementptr inbounds i8, ptr %"339", i64 8
  %104 = load i32, ptr addrspace(5) %"184", align 4
  %"340" = bitcast i32 %104 to float
  store float %"340", ptr %"114", align 4
  %105 = load i64, ptr addrspace(5) %"153", align 8
  %"341" = inttoptr i64 %105 to ptr
  %"116" = getelementptr inbounds i8, ptr %"341", i64 12
  %106 = load i32, ptr addrspace(5) %"185", align 4
  %"342" = bitcast i32 %106 to float
  store float %"342", ptr %"116", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
