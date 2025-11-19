; ModuleID = 'llvm-link'
source_filename = "llvm-link"
target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-p7:160:256:256:32-p8:128:128-p9:192:256:256:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7:8:9"
target triple = "amdgcn-amd-amdhsa"

%0 = type { [16 x %1], [16 x %1], [16 x %1], [16 x %2], [16 x %2], [16 x [256 x %3]], i64, i64, i64, [14 x i64], i64 }
%1 = type { i32, [15 x i64] }
%2 = type { i64, [15 x i64] }
%3 = type { i64, i64, i32 }
%4 = type { i64, i64, i32, i32 }
%5 = type { [64 x [8 x i64]] }

@__ZLUDA_PTX_IMPL_ATTRIBUTE_GFX_VERSION = hidden addrspace(4) constant i32 1011
@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = hidden addrspace(4) constant i32 2124000
@0 = internal addrspace(1) global %0 zeroinitializer, align 8
@__oclc_wavefrontsize64 = hidden addrspace(4) constant i32 0
@__oclc_wavefrontsize_log2 = hidden addrspace(4) constant i32 5
@__oclc_ABI_version = hidden addrspace(4) constant i32 500
@__oclc_unsafe_math_opt = hidden addrspace(4) constant i32 0
@__oclc_correctly_rounded_sqrt32 = hidden addrspace(4) constant i32 1
@__oclc_finite_only_opt = hidden addrspace(4) constant i32 0
@__oclc_ISA_version = hidden addrspace(4) constant i32 10101

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

; Function Attrs: convergent mustprogress nofree nounwind willreturn memory(none)
define linkonce_odr hidden i32 @__zluda_ptx_impl_sreg_tid(i8 noundef zeroext %0) #2 {
  %2 = zext i8 %0 to i32
  %3 = tail call i64 @__ockl_get_local_id(i32 noundef %2) #17
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

; Function Attrs: convergent mustprogress nofree nounwind willreturn memory(none)
define linkonce_odr hidden <4 x i32> @__zluda_ptx_impl_mma_sync_aligned_m16n8k32_row_col_s32_s8_s8_s32(<4 x i32> noundef %0, <2 x i32> noundef %1, <4 x i32> noundef %2) #2 {
  %4 = tail call noundef i32 @llvm.amdgcn.mbcnt.lo(i32 -1, i32 0)
  %5 = extractelement <4 x i32> %2, i64 0
  %6 = extractelement <4 x i32> %2, i64 2
  %7 = shl i32 %4, 3
  %8 = and i32 %7, 24
  %9 = extractelement <4 x i32> %0, i64 0
  %10 = extractelement <4 x i32> %0, i64 1
  %11 = extractelement <2 x i32> %1, i64 0
  %12 = and i32 %4, 3
  %13 = bitcast i32 %9 to <4 x i8>
  %14 = bitcast i32 %10 to <4 x i8>
  %15 = or i32 %8, %12
  %16 = shl nuw nsw i32 %15, 2
  %17 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %16, i32 %11)
  %18 = bitcast i32 %17 to <4 x i8>
  %19 = or i32 %16, 16
  %20 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %19, i32 %11)
  %21 = bitcast i32 %20 to <4 x i8>
  %22 = add i32 %4, 1
  %23 = and i32 %22, 3
  %24 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %9, i32 57, i32 15, i32 15, i1 true)
  %25 = bitcast i32 %24 to <4 x i8>
  %26 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %10, i32 57, i32 15, i32 15, i1 true)
  %27 = bitcast i32 %26 to <4 x i8>
  %28 = or i32 %23, %8
  %29 = shl nuw nsw i32 %28, 2
  %30 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %29, i32 %11)
  %31 = bitcast i32 %30 to <4 x i8>
  %32 = or i32 %29, 16
  %33 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %32, i32 %11)
  %34 = bitcast i32 %33 to <4 x i8>
  %35 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %9, i32 78, i32 15, i32 15, i1 true)
  %36 = bitcast i32 %35 to <4 x i8>
  %37 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %10, i32 78, i32 15, i32 15, i1 true)
  %38 = bitcast i32 %37 to <4 x i8>
  %39 = xor i32 %16, 8
  %40 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %39, i32 %11)
  %41 = bitcast i32 %40 to <4 x i8>
  %42 = or i32 %39, 16
  %43 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %42, i32 %11)
  %44 = bitcast i32 %43 to <4 x i8>
  %45 = add i32 %4, 3
  %46 = and i32 %45, 3
  %47 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %9, i32 147, i32 15, i32 15, i1 true)
  %48 = bitcast i32 %47 to <4 x i8>
  %49 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %10, i32 147, i32 15, i32 15, i1 true)
  %50 = bitcast i32 %49 to <4 x i8>
  %51 = or i32 %46, %8
  %52 = shl nuw nsw i32 %51, 2
  %53 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %52, i32 %11)
  %54 = bitcast i32 %53 to <4 x i8>
  %55 = or i32 %52, 16
  %56 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %55, i32 %11)
  %57 = bitcast i32 %56 to <4 x i8>
  %58 = extractelement <4 x i32> %0, i64 2
  %59 = extractelement <4 x i32> %0, i64 3
  %60 = extractelement <2 x i32> %1, i64 1
  %61 = bitcast i32 %58 to <4 x i8>
  %62 = bitcast i32 %59 to <4 x i8>
  %63 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %16, i32 %60)
  %64 = bitcast i32 %63 to <4 x i8>
  %65 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %19, i32 %60)
  %66 = bitcast i32 %65 to <4 x i8>
  %67 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %58, i32 57, i32 15, i32 15, i1 true)
  %68 = bitcast i32 %67 to <4 x i8>
  %69 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %59, i32 57, i32 15, i32 15, i1 true)
  %70 = bitcast i32 %69 to <4 x i8>
  %71 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %29, i32 %60)
  %72 = bitcast i32 %71 to <4 x i8>
  %73 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %32, i32 %60)
  %74 = bitcast i32 %73 to <4 x i8>
  %75 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %58, i32 78, i32 15, i32 15, i1 true)
  %76 = bitcast i32 %75 to <4 x i8>
  %77 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %59, i32 78, i32 15, i32 15, i1 true)
  %78 = bitcast i32 %77 to <4 x i8>
  %79 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %39, i32 %60)
  %80 = bitcast i32 %79 to <4 x i8>
  %81 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %42, i32 %60)
  %82 = bitcast i32 %81 to <4 x i8>
  %83 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %58, i32 147, i32 15, i32 15, i1 true)
  %84 = bitcast i32 %83 to <4 x i8>
  %85 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %59, i32 147, i32 15, i32 15, i1 true)
  %86 = bitcast i32 %85 to <4 x i8>
  %87 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %52, i32 %60)
  %88 = bitcast i32 %87 to <4 x i8>
  %89 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %55, i32 %60)
  %90 = bitcast i32 %89 to <4 x i8>
  %91 = load i32, ptr addrspace(4) @__oclc_ISA_version, align 4, !tbaa !4
  %92 = icmp eq i32 %91, 10103
  br i1 %92, label %93, label %116

93:                                               ; preds = %3
  %94 = shl i32 %9, 24
  %95 = ashr exact i32 %94, 24
  %96 = shl i32 %17, 24
  %97 = ashr exact i32 %96, 24
  %98 = mul nsw i32 %97, %95
  %99 = shl i32 %9, 16
  %100 = ashr i32 %99, 24
  %101 = shl i32 %17, 16
  %102 = ashr i32 %101, 24
  %103 = mul nsw i32 %102, %100
  %104 = shl i32 %9, 8
  %105 = ashr i32 %104, 24
  %106 = shl i32 %17, 8
  %107 = ashr i32 %106, 24
  %108 = mul nsw i32 %107, %105
  %109 = ashr i32 %9, 24
  %110 = ashr i32 %17, 24
  %111 = mul nsw i32 %110, %109
  %112 = add i32 %111, %5
  %113 = add i32 %112, %98
  %114 = add i32 %113, %103
  %115 = add i32 %114, %108
  br label %118

116:                                              ; preds = %3
  %117 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %13, <4 x i8> noundef %18, i32 noundef %5, i1 noundef zeroext false) #17
  br label %118

118:                                              ; preds = %116, %93
  %119 = phi i32 [ %115, %93 ], [ %117, %116 ]
  br i1 %92, label %122, label %120

120:                                              ; preds = %118
  %121 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %25, <4 x i8> noundef %31, i32 noundef %119, i1 noundef zeroext false) #17
  br label %145

122:                                              ; preds = %118
  %123 = shl i32 %24, 24
  %124 = ashr exact i32 %123, 24
  %125 = shl i32 %30, 24
  %126 = ashr exact i32 %125, 24
  %127 = mul nsw i32 %126, %124
  %128 = shl i32 %24, 16
  %129 = ashr i32 %128, 24
  %130 = shl i32 %30, 16
  %131 = ashr i32 %130, 24
  %132 = mul nsw i32 %131, %129
  %133 = shl i32 %24, 8
  %134 = ashr i32 %133, 24
  %135 = shl i32 %30, 8
  %136 = ashr i32 %135, 24
  %137 = mul nsw i32 %136, %134
  %138 = ashr i32 %24, 24
  %139 = ashr i32 %30, 24
  %140 = mul nsw i32 %139, %138
  %141 = add nsw i32 %127, %140
  %142 = add nsw i32 %141, %132
  %143 = add nsw i32 %142, %137
  %144 = add i32 %143, %119
  br label %145

145:                                              ; preds = %122, %120
  %146 = phi i32 [ %144, %122 ], [ %121, %120 ]
  br i1 %92, label %149, label %147

147:                                              ; preds = %145
  %148 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %36, <4 x i8> noundef %41, i32 noundef %146, i1 noundef zeroext false) #17
  br label %172

149:                                              ; preds = %145
  %150 = shl i32 %35, 24
  %151 = ashr exact i32 %150, 24
  %152 = shl i32 %40, 24
  %153 = ashr exact i32 %152, 24
  %154 = mul nsw i32 %153, %151
  %155 = shl i32 %35, 16
  %156 = ashr i32 %155, 24
  %157 = shl i32 %40, 16
  %158 = ashr i32 %157, 24
  %159 = mul nsw i32 %158, %156
  %160 = shl i32 %35, 8
  %161 = ashr i32 %160, 24
  %162 = shl i32 %40, 8
  %163 = ashr i32 %162, 24
  %164 = mul nsw i32 %163, %161
  %165 = ashr i32 %35, 24
  %166 = ashr i32 %40, 24
  %167 = mul nsw i32 %166, %165
  %168 = add nsw i32 %154, %167
  %169 = add nsw i32 %168, %159
  %170 = add nsw i32 %169, %164
  %171 = add i32 %170, %146
  br label %172

172:                                              ; preds = %149, %147
  %173 = phi i32 [ %171, %149 ], [ %148, %147 ]
  br i1 %92, label %176, label %174

174:                                              ; preds = %172
  %175 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %48, <4 x i8> noundef %54, i32 noundef %173, i1 noundef zeroext false) #17
  br label %199

176:                                              ; preds = %172
  %177 = shl i32 %47, 24
  %178 = ashr exact i32 %177, 24
  %179 = shl i32 %53, 24
  %180 = ashr exact i32 %179, 24
  %181 = mul nsw i32 %180, %178
  %182 = shl i32 %47, 16
  %183 = ashr i32 %182, 24
  %184 = shl i32 %53, 16
  %185 = ashr i32 %184, 24
  %186 = mul nsw i32 %185, %183
  %187 = shl i32 %47, 8
  %188 = ashr i32 %187, 24
  %189 = shl i32 %53, 8
  %190 = ashr i32 %189, 24
  %191 = mul nsw i32 %190, %188
  %192 = ashr i32 %47, 24
  %193 = ashr i32 %53, 24
  %194 = mul nsw i32 %193, %192
  %195 = add nsw i32 %181, %194
  %196 = add nsw i32 %195, %186
  %197 = add nsw i32 %196, %191
  %198 = add i32 %197, %173
  br label %199

199:                                              ; preds = %176, %174
  %200 = phi i32 [ %198, %176 ], [ %175, %174 ]
  br i1 %92, label %203, label %201

201:                                              ; preds = %199
  %202 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %61, <4 x i8> noundef %64, i32 noundef %200, i1 noundef zeroext false) #17
  br label %226

203:                                              ; preds = %199
  %204 = shl i32 %58, 24
  %205 = ashr exact i32 %204, 24
  %206 = shl i32 %63, 24
  %207 = ashr exact i32 %206, 24
  %208 = mul nsw i32 %207, %205
  %209 = shl i32 %58, 16
  %210 = ashr i32 %209, 24
  %211 = shl i32 %63, 16
  %212 = ashr i32 %211, 24
  %213 = mul nsw i32 %212, %210
  %214 = shl i32 %58, 8
  %215 = ashr i32 %214, 24
  %216 = shl i32 %63, 8
  %217 = ashr i32 %216, 24
  %218 = mul nsw i32 %217, %215
  %219 = ashr i32 %58, 24
  %220 = ashr i32 %63, 24
  %221 = mul nsw i32 %220, %219
  %222 = add nsw i32 %208, %221
  %223 = add nsw i32 %222, %213
  %224 = add nsw i32 %223, %218
  %225 = add i32 %224, %200
  br label %226

226:                                              ; preds = %203, %201
  %227 = phi i32 [ %225, %203 ], [ %202, %201 ]
  br i1 %92, label %230, label %228

228:                                              ; preds = %226
  %229 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %68, <4 x i8> noundef %72, i32 noundef %227, i1 noundef zeroext false) #17
  br label %253

230:                                              ; preds = %226
  %231 = shl i32 %67, 24
  %232 = ashr exact i32 %231, 24
  %233 = shl i32 %71, 24
  %234 = ashr exact i32 %233, 24
  %235 = mul nsw i32 %234, %232
  %236 = shl i32 %67, 16
  %237 = ashr i32 %236, 24
  %238 = shl i32 %71, 16
  %239 = ashr i32 %238, 24
  %240 = mul nsw i32 %239, %237
  %241 = shl i32 %67, 8
  %242 = ashr i32 %241, 24
  %243 = shl i32 %71, 8
  %244 = ashr i32 %243, 24
  %245 = mul nsw i32 %244, %242
  %246 = ashr i32 %67, 24
  %247 = ashr i32 %71, 24
  %248 = mul nsw i32 %247, %246
  %249 = add nsw i32 %235, %248
  %250 = add nsw i32 %249, %240
  %251 = add nsw i32 %250, %245
  %252 = add i32 %251, %227
  br label %253

253:                                              ; preds = %230, %228
  %254 = phi i32 [ %252, %230 ], [ %229, %228 ]
  br i1 %92, label %257, label %255

255:                                              ; preds = %253
  %256 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %76, <4 x i8> noundef %80, i32 noundef %254, i1 noundef zeroext false) #17
  br label %280

257:                                              ; preds = %253
  %258 = shl i32 %75, 24
  %259 = ashr exact i32 %258, 24
  %260 = shl i32 %79, 24
  %261 = ashr exact i32 %260, 24
  %262 = mul nsw i32 %261, %259
  %263 = shl i32 %75, 16
  %264 = ashr i32 %263, 24
  %265 = shl i32 %79, 16
  %266 = ashr i32 %265, 24
  %267 = mul nsw i32 %266, %264
  %268 = shl i32 %75, 8
  %269 = ashr i32 %268, 24
  %270 = shl i32 %79, 8
  %271 = ashr i32 %270, 24
  %272 = mul nsw i32 %271, %269
  %273 = ashr i32 %75, 24
  %274 = ashr i32 %79, 24
  %275 = mul nsw i32 %274, %273
  %276 = add nsw i32 %262, %275
  %277 = add nsw i32 %276, %267
  %278 = add nsw i32 %277, %272
  %279 = add i32 %278, %254
  br label %280

280:                                              ; preds = %257, %255
  %281 = phi i32 [ %279, %257 ], [ %256, %255 ]
  br i1 %92, label %284, label %282

282:                                              ; preds = %280
  %283 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %84, <4 x i8> noundef %88, i32 noundef %281, i1 noundef zeroext false) #17
  br label %307

284:                                              ; preds = %280
  %285 = shl i32 %83, 24
  %286 = ashr exact i32 %285, 24
  %287 = shl i32 %87, 24
  %288 = ashr exact i32 %287, 24
  %289 = mul nsw i32 %288, %286
  %290 = shl i32 %83, 16
  %291 = ashr i32 %290, 24
  %292 = shl i32 %87, 16
  %293 = ashr i32 %292, 24
  %294 = mul nsw i32 %293, %291
  %295 = shl i32 %83, 8
  %296 = ashr i32 %295, 24
  %297 = shl i32 %87, 8
  %298 = ashr i32 %297, 24
  %299 = mul nsw i32 %298, %296
  %300 = ashr i32 %83, 24
  %301 = ashr i32 %87, 24
  %302 = mul nsw i32 %301, %300
  %303 = add nsw i32 %289, %302
  %304 = add nsw i32 %303, %294
  %305 = add nsw i32 %304, %299
  %306 = add i32 %305, %281
  br label %307

307:                                              ; preds = %284, %282
  %308 = phi i32 [ %306, %284 ], [ %283, %282 ]
  %309 = extractelement <4 x i32> %2, i64 1
  br i1 %92, label %310, label %333

310:                                              ; preds = %307
  %311 = shl i32 %9, 24
  %312 = ashr exact i32 %311, 24
  %313 = shl i32 %20, 24
  %314 = ashr exact i32 %313, 24
  %315 = mul nsw i32 %314, %312
  %316 = shl i32 %9, 16
  %317 = ashr i32 %316, 24
  %318 = shl i32 %20, 16
  %319 = ashr i32 %318, 24
  %320 = mul nsw i32 %319, %317
  %321 = shl i32 %9, 8
  %322 = ashr i32 %321, 24
  %323 = shl i32 %20, 8
  %324 = ashr i32 %323, 24
  %325 = mul nsw i32 %324, %322
  %326 = ashr i32 %9, 24
  %327 = ashr i32 %20, 24
  %328 = mul nsw i32 %327, %326
  %329 = add i32 %328, %309
  %330 = add i32 %329, %315
  %331 = add i32 %330, %320
  %332 = add i32 %331, %325
  br label %335

333:                                              ; preds = %307
  %334 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %13, <4 x i8> noundef %21, i32 noundef %309, i1 noundef zeroext false) #17
  br label %335

335:                                              ; preds = %333, %310
  %336 = phi i32 [ %332, %310 ], [ %334, %333 ]
  br i1 %92, label %339, label %337

337:                                              ; preds = %335
  %338 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %25, <4 x i8> noundef %34, i32 noundef %336, i1 noundef zeroext false) #17
  br label %362

339:                                              ; preds = %335
  %340 = shl i32 %24, 24
  %341 = ashr exact i32 %340, 24
  %342 = shl i32 %33, 24
  %343 = ashr exact i32 %342, 24
  %344 = mul nsw i32 %343, %341
  %345 = shl i32 %24, 16
  %346 = ashr i32 %345, 24
  %347 = shl i32 %33, 16
  %348 = ashr i32 %347, 24
  %349 = mul nsw i32 %348, %346
  %350 = shl i32 %24, 8
  %351 = ashr i32 %350, 24
  %352 = shl i32 %33, 8
  %353 = ashr i32 %352, 24
  %354 = mul nsw i32 %353, %351
  %355 = ashr i32 %24, 24
  %356 = ashr i32 %33, 24
  %357 = mul nsw i32 %356, %355
  %358 = add nsw i32 %344, %357
  %359 = add nsw i32 %358, %349
  %360 = add nsw i32 %359, %354
  %361 = add i32 %360, %336
  br label %362

362:                                              ; preds = %339, %337
  %363 = phi i32 [ %361, %339 ], [ %338, %337 ]
  br i1 %92, label %366, label %364

364:                                              ; preds = %362
  %365 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %36, <4 x i8> noundef %44, i32 noundef %363, i1 noundef zeroext false) #17
  br label %389

366:                                              ; preds = %362
  %367 = shl i32 %35, 24
  %368 = ashr exact i32 %367, 24
  %369 = shl i32 %43, 24
  %370 = ashr exact i32 %369, 24
  %371 = mul nsw i32 %370, %368
  %372 = shl i32 %35, 16
  %373 = ashr i32 %372, 24
  %374 = shl i32 %43, 16
  %375 = ashr i32 %374, 24
  %376 = mul nsw i32 %375, %373
  %377 = shl i32 %35, 8
  %378 = ashr i32 %377, 24
  %379 = shl i32 %43, 8
  %380 = ashr i32 %379, 24
  %381 = mul nsw i32 %380, %378
  %382 = ashr i32 %35, 24
  %383 = ashr i32 %43, 24
  %384 = mul nsw i32 %383, %382
  %385 = add nsw i32 %371, %384
  %386 = add nsw i32 %385, %376
  %387 = add nsw i32 %386, %381
  %388 = add i32 %387, %363
  br label %389

389:                                              ; preds = %366, %364
  %390 = phi i32 [ %388, %366 ], [ %365, %364 ]
  br i1 %92, label %393, label %391

391:                                              ; preds = %389
  %392 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %48, <4 x i8> noundef %57, i32 noundef %390, i1 noundef zeroext false) #17
  br label %416

393:                                              ; preds = %389
  %394 = shl i32 %47, 24
  %395 = ashr exact i32 %394, 24
  %396 = shl i32 %56, 24
  %397 = ashr exact i32 %396, 24
  %398 = mul nsw i32 %397, %395
  %399 = shl i32 %47, 16
  %400 = ashr i32 %399, 24
  %401 = shl i32 %56, 16
  %402 = ashr i32 %401, 24
  %403 = mul nsw i32 %402, %400
  %404 = shl i32 %47, 8
  %405 = ashr i32 %404, 24
  %406 = shl i32 %56, 8
  %407 = ashr i32 %406, 24
  %408 = mul nsw i32 %407, %405
  %409 = ashr i32 %47, 24
  %410 = ashr i32 %56, 24
  %411 = mul nsw i32 %410, %409
  %412 = add nsw i32 %398, %411
  %413 = add nsw i32 %412, %403
  %414 = add nsw i32 %413, %408
  %415 = add i32 %414, %390
  br label %416

416:                                              ; preds = %393, %391
  %417 = phi i32 [ %415, %393 ], [ %392, %391 ]
  br i1 %92, label %420, label %418

418:                                              ; preds = %416
  %419 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %61, <4 x i8> noundef %66, i32 noundef %417, i1 noundef zeroext false) #17
  br label %443

420:                                              ; preds = %416
  %421 = shl i32 %58, 24
  %422 = ashr exact i32 %421, 24
  %423 = shl i32 %65, 24
  %424 = ashr exact i32 %423, 24
  %425 = mul nsw i32 %424, %422
  %426 = shl i32 %58, 16
  %427 = ashr i32 %426, 24
  %428 = shl i32 %65, 16
  %429 = ashr i32 %428, 24
  %430 = mul nsw i32 %429, %427
  %431 = shl i32 %58, 8
  %432 = ashr i32 %431, 24
  %433 = shl i32 %65, 8
  %434 = ashr i32 %433, 24
  %435 = mul nsw i32 %434, %432
  %436 = ashr i32 %58, 24
  %437 = ashr i32 %65, 24
  %438 = mul nsw i32 %437, %436
  %439 = add nsw i32 %425, %438
  %440 = add nsw i32 %439, %430
  %441 = add nsw i32 %440, %435
  %442 = add i32 %441, %417
  br label %443

443:                                              ; preds = %420, %418
  %444 = phi i32 [ %442, %420 ], [ %419, %418 ]
  br i1 %92, label %447, label %445

445:                                              ; preds = %443
  %446 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %68, <4 x i8> noundef %74, i32 noundef %444, i1 noundef zeroext false) #17
  br label %470

447:                                              ; preds = %443
  %448 = shl i32 %67, 24
  %449 = ashr exact i32 %448, 24
  %450 = shl i32 %73, 24
  %451 = ashr exact i32 %450, 24
  %452 = mul nsw i32 %451, %449
  %453 = shl i32 %67, 16
  %454 = ashr i32 %453, 24
  %455 = shl i32 %73, 16
  %456 = ashr i32 %455, 24
  %457 = mul nsw i32 %456, %454
  %458 = shl i32 %67, 8
  %459 = ashr i32 %458, 24
  %460 = shl i32 %73, 8
  %461 = ashr i32 %460, 24
  %462 = mul nsw i32 %461, %459
  %463 = ashr i32 %67, 24
  %464 = ashr i32 %73, 24
  %465 = mul nsw i32 %464, %463
  %466 = add nsw i32 %452, %465
  %467 = add nsw i32 %466, %457
  %468 = add nsw i32 %467, %462
  %469 = add i32 %468, %444
  br label %470

470:                                              ; preds = %447, %445
  %471 = phi i32 [ %469, %447 ], [ %446, %445 ]
  br i1 %92, label %474, label %472

472:                                              ; preds = %470
  %473 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %76, <4 x i8> noundef %82, i32 noundef %471, i1 noundef zeroext false) #17
  br label %497

474:                                              ; preds = %470
  %475 = shl i32 %75, 24
  %476 = ashr exact i32 %475, 24
  %477 = shl i32 %81, 24
  %478 = ashr exact i32 %477, 24
  %479 = mul nsw i32 %478, %476
  %480 = shl i32 %75, 16
  %481 = ashr i32 %480, 24
  %482 = shl i32 %81, 16
  %483 = ashr i32 %482, 24
  %484 = mul nsw i32 %483, %481
  %485 = shl i32 %75, 8
  %486 = ashr i32 %485, 24
  %487 = shl i32 %81, 8
  %488 = ashr i32 %487, 24
  %489 = mul nsw i32 %488, %486
  %490 = ashr i32 %75, 24
  %491 = ashr i32 %81, 24
  %492 = mul nsw i32 %491, %490
  %493 = add nsw i32 %479, %492
  %494 = add nsw i32 %493, %484
  %495 = add nsw i32 %494, %489
  %496 = add i32 %495, %471
  br label %497

497:                                              ; preds = %474, %472
  %498 = phi i32 [ %496, %474 ], [ %473, %472 ]
  br i1 %92, label %501, label %499

499:                                              ; preds = %497
  %500 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %84, <4 x i8> noundef %90, i32 noundef %498, i1 noundef zeroext false) #17
  br label %524

501:                                              ; preds = %497
  %502 = shl i32 %83, 24
  %503 = ashr exact i32 %502, 24
  %504 = shl i32 %89, 24
  %505 = ashr exact i32 %504, 24
  %506 = mul nsw i32 %505, %503
  %507 = shl i32 %83, 16
  %508 = ashr i32 %507, 24
  %509 = shl i32 %89, 16
  %510 = ashr i32 %509, 24
  %511 = mul nsw i32 %510, %508
  %512 = shl i32 %83, 8
  %513 = ashr i32 %512, 24
  %514 = shl i32 %89, 8
  %515 = ashr i32 %514, 24
  %516 = mul nsw i32 %515, %513
  %517 = ashr i32 %83, 24
  %518 = ashr i32 %89, 24
  %519 = mul nsw i32 %518, %517
  %520 = add nsw i32 %506, %519
  %521 = add nsw i32 %520, %511
  %522 = add nsw i32 %521, %516
  %523 = add i32 %522, %498
  br label %524

524:                                              ; preds = %501, %499
  %525 = phi i32 [ %523, %501 ], [ %500, %499 ]
  br i1 %92, label %526, label %549

526:                                              ; preds = %524
  %527 = shl i32 %10, 24
  %528 = ashr exact i32 %527, 24
  %529 = shl i32 %17, 24
  %530 = ashr exact i32 %529, 24
  %531 = mul nsw i32 %530, %528
  %532 = shl i32 %10, 16
  %533 = ashr i32 %532, 24
  %534 = shl i32 %17, 16
  %535 = ashr i32 %534, 24
  %536 = mul nsw i32 %535, %533
  %537 = shl i32 %10, 8
  %538 = ashr i32 %537, 24
  %539 = shl i32 %17, 8
  %540 = ashr i32 %539, 24
  %541 = mul nsw i32 %540, %538
  %542 = ashr i32 %10, 24
  %543 = ashr i32 %17, 24
  %544 = mul nsw i32 %543, %542
  %545 = add i32 %544, %6
  %546 = add i32 %545, %531
  %547 = add i32 %546, %536
  %548 = add i32 %547, %541
  br label %551

549:                                              ; preds = %524
  %550 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %14, <4 x i8> noundef %18, i32 noundef %6, i1 noundef zeroext false) #17
  br label %551

551:                                              ; preds = %549, %526
  %552 = phi i32 [ %548, %526 ], [ %550, %549 ]
  br i1 %92, label %555, label %553

553:                                              ; preds = %551
  %554 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %27, <4 x i8> noundef %31, i32 noundef %552, i1 noundef zeroext false) #17
  br label %578

555:                                              ; preds = %551
  %556 = shl i32 %26, 24
  %557 = ashr exact i32 %556, 24
  %558 = shl i32 %30, 24
  %559 = ashr exact i32 %558, 24
  %560 = mul nsw i32 %559, %557
  %561 = shl i32 %26, 16
  %562 = ashr i32 %561, 24
  %563 = shl i32 %30, 16
  %564 = ashr i32 %563, 24
  %565 = mul nsw i32 %564, %562
  %566 = shl i32 %26, 8
  %567 = ashr i32 %566, 24
  %568 = shl i32 %30, 8
  %569 = ashr i32 %568, 24
  %570 = mul nsw i32 %569, %567
  %571 = ashr i32 %26, 24
  %572 = ashr i32 %30, 24
  %573 = mul nsw i32 %572, %571
  %574 = add nsw i32 %560, %573
  %575 = add nsw i32 %574, %565
  %576 = add nsw i32 %575, %570
  %577 = add i32 %576, %552
  br label %578

578:                                              ; preds = %555, %553
  %579 = phi i32 [ %577, %555 ], [ %554, %553 ]
  br i1 %92, label %582, label %580

580:                                              ; preds = %578
  %581 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %38, <4 x i8> noundef %41, i32 noundef %579, i1 noundef zeroext false) #17
  br label %605

582:                                              ; preds = %578
  %583 = shl i32 %37, 24
  %584 = ashr exact i32 %583, 24
  %585 = shl i32 %40, 24
  %586 = ashr exact i32 %585, 24
  %587 = mul nsw i32 %586, %584
  %588 = shl i32 %37, 16
  %589 = ashr i32 %588, 24
  %590 = shl i32 %40, 16
  %591 = ashr i32 %590, 24
  %592 = mul nsw i32 %591, %589
  %593 = shl i32 %37, 8
  %594 = ashr i32 %593, 24
  %595 = shl i32 %40, 8
  %596 = ashr i32 %595, 24
  %597 = mul nsw i32 %596, %594
  %598 = ashr i32 %37, 24
  %599 = ashr i32 %40, 24
  %600 = mul nsw i32 %599, %598
  %601 = add nsw i32 %587, %600
  %602 = add nsw i32 %601, %592
  %603 = add nsw i32 %602, %597
  %604 = add i32 %603, %579
  br label %605

605:                                              ; preds = %582, %580
  %606 = phi i32 [ %604, %582 ], [ %581, %580 ]
  br i1 %92, label %609, label %607

607:                                              ; preds = %605
  %608 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %50, <4 x i8> noundef %54, i32 noundef %606, i1 noundef zeroext false) #17
  br label %632

609:                                              ; preds = %605
  %610 = shl i32 %49, 24
  %611 = ashr exact i32 %610, 24
  %612 = shl i32 %53, 24
  %613 = ashr exact i32 %612, 24
  %614 = mul nsw i32 %613, %611
  %615 = shl i32 %49, 16
  %616 = ashr i32 %615, 24
  %617 = shl i32 %53, 16
  %618 = ashr i32 %617, 24
  %619 = mul nsw i32 %618, %616
  %620 = shl i32 %49, 8
  %621 = ashr i32 %620, 24
  %622 = shl i32 %53, 8
  %623 = ashr i32 %622, 24
  %624 = mul nsw i32 %623, %621
  %625 = ashr i32 %49, 24
  %626 = ashr i32 %53, 24
  %627 = mul nsw i32 %626, %625
  %628 = add nsw i32 %614, %627
  %629 = add nsw i32 %628, %619
  %630 = add nsw i32 %629, %624
  %631 = add i32 %630, %606
  br label %632

632:                                              ; preds = %609, %607
  %633 = phi i32 [ %631, %609 ], [ %608, %607 ]
  br i1 %92, label %636, label %634

634:                                              ; preds = %632
  %635 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %62, <4 x i8> noundef %64, i32 noundef %633, i1 noundef zeroext false) #17
  br label %659

636:                                              ; preds = %632
  %637 = shl i32 %59, 24
  %638 = ashr exact i32 %637, 24
  %639 = shl i32 %63, 24
  %640 = ashr exact i32 %639, 24
  %641 = mul nsw i32 %640, %638
  %642 = shl i32 %59, 16
  %643 = ashr i32 %642, 24
  %644 = shl i32 %63, 16
  %645 = ashr i32 %644, 24
  %646 = mul nsw i32 %645, %643
  %647 = shl i32 %59, 8
  %648 = ashr i32 %647, 24
  %649 = shl i32 %63, 8
  %650 = ashr i32 %649, 24
  %651 = mul nsw i32 %650, %648
  %652 = ashr i32 %59, 24
  %653 = ashr i32 %63, 24
  %654 = mul nsw i32 %653, %652
  %655 = add nsw i32 %641, %654
  %656 = add nsw i32 %655, %646
  %657 = add nsw i32 %656, %651
  %658 = add i32 %657, %633
  br label %659

659:                                              ; preds = %636, %634
  %660 = phi i32 [ %658, %636 ], [ %635, %634 ]
  br i1 %92, label %663, label %661

661:                                              ; preds = %659
  %662 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %70, <4 x i8> noundef %72, i32 noundef %660, i1 noundef zeroext false) #17
  br label %686

663:                                              ; preds = %659
  %664 = shl i32 %69, 24
  %665 = ashr exact i32 %664, 24
  %666 = shl i32 %71, 24
  %667 = ashr exact i32 %666, 24
  %668 = mul nsw i32 %667, %665
  %669 = shl i32 %69, 16
  %670 = ashr i32 %669, 24
  %671 = shl i32 %71, 16
  %672 = ashr i32 %671, 24
  %673 = mul nsw i32 %672, %670
  %674 = shl i32 %69, 8
  %675 = ashr i32 %674, 24
  %676 = shl i32 %71, 8
  %677 = ashr i32 %676, 24
  %678 = mul nsw i32 %677, %675
  %679 = ashr i32 %69, 24
  %680 = ashr i32 %71, 24
  %681 = mul nsw i32 %680, %679
  %682 = add nsw i32 %668, %681
  %683 = add nsw i32 %682, %673
  %684 = add nsw i32 %683, %678
  %685 = add i32 %684, %660
  br label %686

686:                                              ; preds = %663, %661
  %687 = phi i32 [ %685, %663 ], [ %662, %661 ]
  br i1 %92, label %690, label %688

688:                                              ; preds = %686
  %689 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %78, <4 x i8> noundef %80, i32 noundef %687, i1 noundef zeroext false) #17
  br label %713

690:                                              ; preds = %686
  %691 = shl i32 %77, 24
  %692 = ashr exact i32 %691, 24
  %693 = shl i32 %79, 24
  %694 = ashr exact i32 %693, 24
  %695 = mul nsw i32 %694, %692
  %696 = shl i32 %77, 16
  %697 = ashr i32 %696, 24
  %698 = shl i32 %79, 16
  %699 = ashr i32 %698, 24
  %700 = mul nsw i32 %699, %697
  %701 = shl i32 %77, 8
  %702 = ashr i32 %701, 24
  %703 = shl i32 %79, 8
  %704 = ashr i32 %703, 24
  %705 = mul nsw i32 %704, %702
  %706 = ashr i32 %77, 24
  %707 = ashr i32 %79, 24
  %708 = mul nsw i32 %707, %706
  %709 = add nsw i32 %695, %708
  %710 = add nsw i32 %709, %700
  %711 = add nsw i32 %710, %705
  %712 = add i32 %711, %687
  br label %713

713:                                              ; preds = %690, %688
  %714 = phi i32 [ %712, %690 ], [ %689, %688 ]
  br i1 %92, label %717, label %715

715:                                              ; preds = %713
  %716 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %86, <4 x i8> noundef %88, i32 noundef %714, i1 noundef zeroext false) #17
  br label %740

717:                                              ; preds = %713
  %718 = shl i32 %85, 24
  %719 = ashr exact i32 %718, 24
  %720 = shl i32 %87, 24
  %721 = ashr exact i32 %720, 24
  %722 = mul nsw i32 %721, %719
  %723 = shl i32 %85, 16
  %724 = ashr i32 %723, 24
  %725 = shl i32 %87, 16
  %726 = ashr i32 %725, 24
  %727 = mul nsw i32 %726, %724
  %728 = shl i32 %85, 8
  %729 = ashr i32 %728, 24
  %730 = shl i32 %87, 8
  %731 = ashr i32 %730, 24
  %732 = mul nsw i32 %731, %729
  %733 = ashr i32 %85, 24
  %734 = ashr i32 %87, 24
  %735 = mul nsw i32 %734, %733
  %736 = add nsw i32 %722, %735
  %737 = add nsw i32 %736, %727
  %738 = add nsw i32 %737, %732
  %739 = add i32 %738, %714
  br label %740

740:                                              ; preds = %717, %715
  %741 = phi i32 [ %739, %717 ], [ %716, %715 ]
  %742 = extractelement <4 x i32> %2, i64 3
  br i1 %92, label %743, label %766

743:                                              ; preds = %740
  %744 = shl i32 %10, 24
  %745 = ashr exact i32 %744, 24
  %746 = shl i32 %20, 24
  %747 = ashr exact i32 %746, 24
  %748 = mul nsw i32 %747, %745
  %749 = shl i32 %10, 16
  %750 = ashr i32 %749, 24
  %751 = shl i32 %20, 16
  %752 = ashr i32 %751, 24
  %753 = mul nsw i32 %752, %750
  %754 = shl i32 %10, 8
  %755 = ashr i32 %754, 24
  %756 = shl i32 %20, 8
  %757 = ashr i32 %756, 24
  %758 = mul nsw i32 %757, %755
  %759 = ashr i32 %10, 24
  %760 = ashr i32 %20, 24
  %761 = mul nsw i32 %760, %759
  %762 = add i32 %761, %742
  %763 = add i32 %762, %748
  %764 = add i32 %763, %753
  %765 = add i32 %764, %758
  br label %768

766:                                              ; preds = %740
  %767 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %14, <4 x i8> noundef %21, i32 noundef %742, i1 noundef zeroext false) #17
  br label %768

768:                                              ; preds = %766, %743
  %769 = phi i32 [ %765, %743 ], [ %767, %766 ]
  br i1 %92, label %772, label %770

770:                                              ; preds = %768
  %771 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %27, <4 x i8> noundef %34, i32 noundef %769, i1 noundef zeroext false) #17
  br label %795

772:                                              ; preds = %768
  %773 = shl i32 %26, 24
  %774 = ashr exact i32 %773, 24
  %775 = shl i32 %33, 24
  %776 = ashr exact i32 %775, 24
  %777 = mul nsw i32 %776, %774
  %778 = shl i32 %26, 16
  %779 = ashr i32 %778, 24
  %780 = shl i32 %33, 16
  %781 = ashr i32 %780, 24
  %782 = mul nsw i32 %781, %779
  %783 = shl i32 %26, 8
  %784 = ashr i32 %783, 24
  %785 = shl i32 %33, 8
  %786 = ashr i32 %785, 24
  %787 = mul nsw i32 %786, %784
  %788 = ashr i32 %26, 24
  %789 = ashr i32 %33, 24
  %790 = mul nsw i32 %789, %788
  %791 = add nsw i32 %777, %790
  %792 = add nsw i32 %791, %782
  %793 = add nsw i32 %792, %787
  %794 = add i32 %793, %769
  br label %795

795:                                              ; preds = %772, %770
  %796 = phi i32 [ %794, %772 ], [ %771, %770 ]
  br i1 %92, label %799, label %797

797:                                              ; preds = %795
  %798 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %38, <4 x i8> noundef %44, i32 noundef %796, i1 noundef zeroext false) #17
  br label %822

799:                                              ; preds = %795
  %800 = shl i32 %37, 24
  %801 = ashr exact i32 %800, 24
  %802 = shl i32 %43, 24
  %803 = ashr exact i32 %802, 24
  %804 = mul nsw i32 %803, %801
  %805 = shl i32 %37, 16
  %806 = ashr i32 %805, 24
  %807 = shl i32 %43, 16
  %808 = ashr i32 %807, 24
  %809 = mul nsw i32 %808, %806
  %810 = shl i32 %37, 8
  %811 = ashr i32 %810, 24
  %812 = shl i32 %43, 8
  %813 = ashr i32 %812, 24
  %814 = mul nsw i32 %813, %811
  %815 = ashr i32 %37, 24
  %816 = ashr i32 %43, 24
  %817 = mul nsw i32 %816, %815
  %818 = add nsw i32 %804, %817
  %819 = add nsw i32 %818, %809
  %820 = add nsw i32 %819, %814
  %821 = add i32 %820, %796
  br label %822

822:                                              ; preds = %799, %797
  %823 = phi i32 [ %821, %799 ], [ %798, %797 ]
  br i1 %92, label %826, label %824

824:                                              ; preds = %822
  %825 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %50, <4 x i8> noundef %57, i32 noundef %823, i1 noundef zeroext false) #17
  br label %849

826:                                              ; preds = %822
  %827 = shl i32 %49, 24
  %828 = ashr exact i32 %827, 24
  %829 = shl i32 %56, 24
  %830 = ashr exact i32 %829, 24
  %831 = mul nsw i32 %830, %828
  %832 = shl i32 %49, 16
  %833 = ashr i32 %832, 24
  %834 = shl i32 %56, 16
  %835 = ashr i32 %834, 24
  %836 = mul nsw i32 %835, %833
  %837 = shl i32 %49, 8
  %838 = ashr i32 %837, 24
  %839 = shl i32 %56, 8
  %840 = ashr i32 %839, 24
  %841 = mul nsw i32 %840, %838
  %842 = ashr i32 %49, 24
  %843 = ashr i32 %56, 24
  %844 = mul nsw i32 %843, %842
  %845 = add nsw i32 %831, %844
  %846 = add nsw i32 %845, %836
  %847 = add nsw i32 %846, %841
  %848 = add i32 %847, %823
  br label %849

849:                                              ; preds = %826, %824
  %850 = phi i32 [ %848, %826 ], [ %825, %824 ]
  br i1 %92, label %853, label %851

851:                                              ; preds = %849
  %852 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %62, <4 x i8> noundef %66, i32 noundef %850, i1 noundef zeroext false) #17
  br label %876

853:                                              ; preds = %849
  %854 = shl i32 %59, 24
  %855 = ashr exact i32 %854, 24
  %856 = shl i32 %65, 24
  %857 = ashr exact i32 %856, 24
  %858 = mul nsw i32 %857, %855
  %859 = shl i32 %59, 16
  %860 = ashr i32 %859, 24
  %861 = shl i32 %65, 16
  %862 = ashr i32 %861, 24
  %863 = mul nsw i32 %862, %860
  %864 = shl i32 %59, 8
  %865 = ashr i32 %864, 24
  %866 = shl i32 %65, 8
  %867 = ashr i32 %866, 24
  %868 = mul nsw i32 %867, %865
  %869 = ashr i32 %59, 24
  %870 = ashr i32 %65, 24
  %871 = mul nsw i32 %870, %869
  %872 = add nsw i32 %858, %871
  %873 = add nsw i32 %872, %863
  %874 = add nsw i32 %873, %868
  %875 = add i32 %874, %850
  br label %876

876:                                              ; preds = %853, %851
  %877 = phi i32 [ %875, %853 ], [ %852, %851 ]
  br i1 %92, label %880, label %878

878:                                              ; preds = %876
  %879 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %70, <4 x i8> noundef %74, i32 noundef %877, i1 noundef zeroext false) #17
  br label %903

880:                                              ; preds = %876
  %881 = shl i32 %69, 24
  %882 = ashr exact i32 %881, 24
  %883 = shl i32 %73, 24
  %884 = ashr exact i32 %883, 24
  %885 = mul nsw i32 %884, %882
  %886 = shl i32 %69, 16
  %887 = ashr i32 %886, 24
  %888 = shl i32 %73, 16
  %889 = ashr i32 %888, 24
  %890 = mul nsw i32 %889, %887
  %891 = shl i32 %69, 8
  %892 = ashr i32 %891, 24
  %893 = shl i32 %73, 8
  %894 = ashr i32 %893, 24
  %895 = mul nsw i32 %894, %892
  %896 = ashr i32 %69, 24
  %897 = ashr i32 %73, 24
  %898 = mul nsw i32 %897, %896
  %899 = add nsw i32 %885, %898
  %900 = add nsw i32 %899, %890
  %901 = add nsw i32 %900, %895
  %902 = add i32 %901, %877
  br label %903

903:                                              ; preds = %880, %878
  %904 = phi i32 [ %902, %880 ], [ %879, %878 ]
  br i1 %92, label %907, label %905

905:                                              ; preds = %903
  %906 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %78, <4 x i8> noundef %82, i32 noundef %904, i1 noundef zeroext false) #17
  br label %930

907:                                              ; preds = %903
  %908 = shl i32 %77, 24
  %909 = ashr exact i32 %908, 24
  %910 = shl i32 %81, 24
  %911 = ashr exact i32 %910, 24
  %912 = mul nsw i32 %911, %909
  %913 = shl i32 %77, 16
  %914 = ashr i32 %913, 24
  %915 = shl i32 %81, 16
  %916 = ashr i32 %915, 24
  %917 = mul nsw i32 %916, %914
  %918 = shl i32 %77, 8
  %919 = ashr i32 %918, 24
  %920 = shl i32 %81, 8
  %921 = ashr i32 %920, 24
  %922 = mul nsw i32 %921, %919
  %923 = ashr i32 %77, 24
  %924 = ashr i32 %81, 24
  %925 = mul nsw i32 %924, %923
  %926 = add nsw i32 %912, %925
  %927 = add nsw i32 %926, %917
  %928 = add nsw i32 %927, %922
  %929 = add i32 %928, %904
  br label %930

930:                                              ; preds = %907, %905
  %931 = phi i32 [ %929, %907 ], [ %906, %905 ]
  br i1 %92, label %934, label %932

932:                                              ; preds = %930
  %933 = tail call i32 @__ockl_sdot4(<4 x i8> noundef %86, <4 x i8> noundef %90, i32 noundef %931, i1 noundef zeroext false) #17
  br label %957

934:                                              ; preds = %930
  %935 = shl i32 %85, 24
  %936 = ashr exact i32 %935, 24
  %937 = shl i32 %89, 24
  %938 = ashr exact i32 %937, 24
  %939 = mul nsw i32 %938, %936
  %940 = shl i32 %85, 16
  %941 = ashr i32 %940, 24
  %942 = shl i32 %89, 16
  %943 = ashr i32 %942, 24
  %944 = mul nsw i32 %943, %941
  %945 = shl i32 %85, 8
  %946 = ashr i32 %945, 24
  %947 = shl i32 %89, 8
  %948 = ashr i32 %947, 24
  %949 = mul nsw i32 %948, %946
  %950 = ashr i32 %85, 24
  %951 = ashr i32 %89, 24
  %952 = mul nsw i32 %951, %950
  %953 = add nsw i32 %939, %952
  %954 = add nsw i32 %953, %944
  %955 = add nsw i32 %954, %949
  %956 = add i32 %955, %931
  br label %957

957:                                              ; preds = %934, %932
  %958 = phi i32 [ %956, %934 ], [ %933, %932 ]
  %959 = insertelement <4 x i32> poison, i32 %308, i64 0
  %960 = insertelement <4 x i32> %959, i32 %525, i64 1
  %961 = insertelement <4 x i32> %960, i32 %741, i64 2
  %962 = insertelement <4 x i32> %961, i32 %958, i64 3
  ret <4 x i32> %962
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i32 @llvm.amdgcn.mbcnt.lo(i32, i32) #3

; Function Attrs: convergent nocallback nofree nounwind willreturn memory(none)
declare i32 @llvm.amdgcn.ds.bpermute(i32, i32) #4

; Function Attrs: convergent nocallback nofree nounwind willreturn memory(none)
declare i32 @llvm.amdgcn.update.dpp.i32(i32, i32, i32 immarg, i32 immarg, i32 immarg, i1 immarg) #4

; Function Attrs: convergent norecurse nounwind
define weak hidden i64 @__ockl_devmem_request(i64 noundef %0, i64 noundef %1) local_unnamed_addr #5 {
  %3 = tail call <2 x i64> @__ockl_hostcall_preview(i32 noundef 3, i64 noundef %0, i64 noundef %1, i64 noundef 0, i64 noundef 0, i64 noundef 0, i64 noundef 0, i64 noundef 0, i64 noundef 0) #18
  %4 = extractelement <2 x i64> %3, i64 0
  ret i64 %4
}

; Function Attrs: convergent norecurse nounwind
define linkonce_odr hidden <2 x i64> @__ockl_hostcall_preview(i32 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4, i64 noundef %5, i64 noundef %6, i64 noundef %7, i64 noundef %8) local_unnamed_addr #5 {
  %10 = load i32, ptr addrspace(4) @__oclc_ABI_version, align 4, !tbaa !8
  %11 = icmp slt i32 %10, 500
  %12 = tail call ptr addrspace(4) @llvm.amdgcn.implicitarg.ptr()
  %13 = select i1 %11, i64 24, i64 80
  %14 = getelementptr inbounds i8, ptr addrspace(4) %12, i64 %13
  %15 = load i64, ptr addrspace(4) %14, align 8, !tbaa !12
  %16 = inttoptr i64 %15 to ptr addrspace(1)
  %17 = addrspacecast ptr addrspace(1) %16 to ptr
  %18 = tail call <2 x i64> @__ockl_hostcall_internal(ptr noundef %17, i32 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4, i64 noundef %5, i64 noundef %6, i64 noundef %7, i64 noundef %8) #19
  ret <2 x i64> %18
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare noundef align 4 ptr addrspace(4) @llvm.amdgcn.implicitarg.ptr() #6

; Function Attrs: convergent norecurse nounwind
define linkonce_odr hidden <2 x i64> @__ockl_hostcall_internal(ptr nocapture noundef %0, i32 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4, i64 noundef %5, i64 noundef %6, i64 noundef %7, i64 noundef %8, i64 noundef %9) local_unnamed_addr #5 {
  %11 = tail call i32 @__ockl_lane_u32() #18
  %12 = tail call i32 @llvm.amdgcn.readfirstlane.i32(i32 %11)
  %13 = addrspacecast ptr %0 to ptr addrspace(1)
  %14 = icmp eq i32 %11, %12
  br i1 %14, label %15, label %37

15:                                               ; preds = %10
  %16 = getelementptr inbounds i8, ptr addrspace(1) %13, i64 24
  %17 = load atomic i64, ptr addrspace(1) %16 syncscope("one-as") acquire, align 8
  %18 = getelementptr i8, ptr addrspace(1) %13, i64 40
  %19 = load ptr addrspace(1), ptr addrspace(1) %13, align 8, !tbaa !14
  %20 = load i64, ptr addrspace(1) %18, align 8, !tbaa !18
  %21 = and i64 %20, %17
  %22 = getelementptr inbounds %4, ptr addrspace(1) %19, i64 %21
  %23 = load atomic i64, ptr addrspace(1) %22 syncscope("one-as") monotonic, align 8
  %24 = cmpxchg ptr addrspace(1) %16, i64 %17, i64 %23 syncscope("one-as") acquire monotonic, align 8
  %25 = extractvalue { i64, i1 } %24, 1
  %26 = extractvalue { i64, i1 } %24, 0
  br i1 %25, label %37, label %27

27:                                               ; preds = %27, %15
  %28 = phi i64 [ %36, %27 ], [ %26, %15 ]
  tail call void @llvm.amdgcn.s.sleep(i32 1)
  %29 = load ptr addrspace(1), ptr addrspace(1) %13, align 8, !tbaa !14
  %30 = load i64, ptr addrspace(1) %18, align 8, !tbaa !18
  %31 = and i64 %30, %28
  %32 = getelementptr inbounds %4, ptr addrspace(1) %29, i64 %31
  %33 = load atomic i64, ptr addrspace(1) %32 syncscope("one-as") monotonic, align 8
  %34 = cmpxchg ptr addrspace(1) %16, i64 %28, i64 %33 syncscope("one-as") acquire monotonic, align 8
  %35 = extractvalue { i64, i1 } %34, 1
  %36 = extractvalue { i64, i1 } %34, 0
  br i1 %35, label %37, label %27

37:                                               ; preds = %27, %15, %10
  %38 = phi i64 [ 0, %10 ], [ %26, %15 ], [ %36, %27 ]
  %39 = trunc i64 %38 to i32
  %40 = lshr i64 %38, 32
  %41 = trunc nuw i64 %40 to i32
  %42 = tail call i32 @llvm.amdgcn.readfirstlane.i32(i32 %39)
  %43 = tail call i32 @llvm.amdgcn.readfirstlane.i32(i32 %41)
  %44 = zext i32 %43 to i64
  %45 = shl nuw i64 %44, 32
  %46 = zext i32 %42 to i64
  %47 = or disjoint i64 %45, %46
  %48 = load ptr addrspace(1), ptr addrspace(1) %13, align 8, !tbaa !14
  %49 = getelementptr i8, ptr addrspace(1) %13, i64 40
  %50 = load i64, ptr addrspace(1) %49, align 8, !tbaa !18
  %51 = and i64 %47, %50
  %52 = getelementptr inbounds %4, ptr addrspace(1) %48, i64 %51
  %53 = getelementptr i8, ptr addrspace(1) %13, i64 8
  %54 = load ptr addrspace(1), ptr addrspace(1) %53, align 8, !tbaa !19
  %55 = getelementptr inbounds %5, ptr addrspace(1) %54, i64 %51
  %56 = tail call i64 @llvm.amdgcn.ballot.i64(i1 true)
  br i1 %14, label %57, label %61

57:                                               ; preds = %37
  %58 = getelementptr inbounds i8, ptr addrspace(1) %52, i64 16
  %59 = getelementptr inbounds i8, ptr addrspace(1) %52, i64 8
  %60 = getelementptr inbounds i8, ptr addrspace(1) %52, i64 20
  store i32 %1, ptr addrspace(1) %58, align 8, !tbaa !20
  store i64 %56, ptr addrspace(1) %59, align 8, !tbaa !22
  store i32 1, ptr addrspace(1) %60, align 4, !tbaa !23
  br label %61

61:                                               ; preds = %57, %37
  %62 = zext i32 %11 to i64
  %63 = getelementptr inbounds [64 x [8 x i64]], ptr addrspace(1) %55, i64 0, i64 %62
  store i64 %2, ptr addrspace(1) %63, align 8, !tbaa !12
  %64 = getelementptr inbounds i8, ptr addrspace(1) %63, i64 8
  store i64 %3, ptr addrspace(1) %64, align 8, !tbaa !12
  %65 = getelementptr inbounds i8, ptr addrspace(1) %63, i64 16
  store i64 %4, ptr addrspace(1) %65, align 8, !tbaa !12
  %66 = getelementptr inbounds i8, ptr addrspace(1) %63, i64 24
  store i64 %5, ptr addrspace(1) %66, align 8, !tbaa !12
  %67 = getelementptr inbounds i8, ptr addrspace(1) %63, i64 32
  store i64 %6, ptr addrspace(1) %67, align 8, !tbaa !12
  %68 = getelementptr inbounds i8, ptr addrspace(1) %63, i64 40
  store i64 %7, ptr addrspace(1) %68, align 8, !tbaa !12
  %69 = getelementptr inbounds i8, ptr addrspace(1) %63, i64 48
  store i64 %8, ptr addrspace(1) %69, align 8, !tbaa !12
  %70 = getelementptr inbounds i8, ptr addrspace(1) %63, i64 56
  store i64 %9, ptr addrspace(1) %70, align 8, !tbaa !12
  br i1 %14, label %71, label %87

71:                                               ; preds = %61
  %72 = getelementptr inbounds i8, ptr addrspace(1) %13, i64 32
  %73 = load atomic i64, ptr addrspace(1) %72 syncscope("one-as") monotonic, align 8
  %74 = load i64, ptr addrspace(1) %49, align 8, !tbaa !18
  %75 = and i64 %74, %47
  %76 = getelementptr inbounds %4, ptr addrspace(1) %48, i64 %75
  store i64 %73, ptr addrspace(1) %76, align 8, !tbaa !24
  %77 = cmpxchg ptr addrspace(1) %72, i64 %73, i64 %47 syncscope("one-as") release monotonic, align 8
  %78 = extractvalue { i64, i1 } %77, 1
  br i1 %78, label %84, label %79

79:                                               ; preds = %79, %71
  %80 = phi { i64, i1 } [ %82, %79 ], [ %77, %71 ]
  %81 = extractvalue { i64, i1 } %80, 0
  tail call void @llvm.amdgcn.s.sleep(i32 1)
  store i64 %81, ptr addrspace(1) %76, align 8, !tbaa !24
  %82 = cmpxchg ptr addrspace(1) %72, i64 %81, i64 %47 syncscope("one-as") release monotonic, align 8
  %83 = extractvalue { i64, i1 } %82, 1
  br i1 %83, label %84, label %79

84:                                               ; preds = %79, %71
  %85 = getelementptr inbounds i8, ptr addrspace(1) %13, i64 16
  %86 = load i64, ptr addrspace(1) %85, align 8
  tail call void @__ockl_hsa_signal_add(i64 %86, i64 noundef 1, i32 noundef 3) #18
  br label %87

87:                                               ; preds = %84, %61
  %88 = getelementptr inbounds i8, ptr addrspace(1) %52, i64 20
  br label %89

89:                                               ; preds = %97, %87
  br i1 %14, label %90, label %93

90:                                               ; preds = %89
  %91 = load atomic i32, ptr addrspace(1) %88 syncscope("one-as") acquire, align 4
  %92 = and i32 %91, 1
  br label %93

93:                                               ; preds = %90, %89
  %94 = phi i32 [ %92, %90 ], [ 1, %89 ]
  %95 = tail call i32 @llvm.amdgcn.readfirstlane.i32(i32 %94)
  %96 = icmp eq i32 %95, 0
  br i1 %96, label %98, label %97

97:                                               ; preds = %93
  tail call void @llvm.amdgcn.s.sleep(i32 1)
  br label %89

98:                                               ; preds = %93
  %99 = load i64, ptr addrspace(1) %63, align 8, !tbaa !12
  %100 = load i64, ptr addrspace(1) %64, align 8, !tbaa !12
  br i1 %14, label %101, label %119

101:                                              ; preds = %98
  %102 = load i64, ptr addrspace(1) %49, align 8, !tbaa !18
  %103 = add i64 %102, 1
  %104 = add i64 %103, %47
  %105 = icmp eq i64 %104, 0
  %106 = select i1 %105, i64 %103, i64 %104
  %107 = getelementptr inbounds i8, ptr addrspace(1) %13, i64 24
  %108 = load atomic i64, ptr addrspace(1) %107 syncscope("one-as") monotonic, align 8
  %109 = load ptr addrspace(1), ptr addrspace(1) %13, align 8, !tbaa !14
  %110 = and i64 %106, %102
  %111 = getelementptr inbounds %4, ptr addrspace(1) %109, i64 %110
  store i64 %108, ptr addrspace(1) %111, align 8, !tbaa !24
  %112 = cmpxchg ptr addrspace(1) %107, i64 %108, i64 %106 syncscope("one-as") release monotonic, align 8
  %113 = extractvalue { i64, i1 } %112, 1
  br i1 %113, label %119, label %114

114:                                              ; preds = %114, %101
  %115 = phi { i64, i1 } [ %117, %114 ], [ %112, %101 ]
  %116 = extractvalue { i64, i1 } %115, 0
  tail call void @llvm.amdgcn.s.sleep(i32 1)
  store i64 %116, ptr addrspace(1) %111, align 8, !tbaa !24
  %117 = cmpxchg ptr addrspace(1) %107, i64 %116, i64 %106 syncscope("one-as") release monotonic, align 8
  %118 = extractvalue { i64, i1 } %117, 1
  br i1 %118, label %119, label %114

119:                                              ; preds = %114, %101, %98
  %120 = insertelement <2 x i64> poison, i64 %99, i64 0
  %121 = insertelement <2 x i64> %120, i64 %100, i64 1
  ret <2 x i64> %121
}

; Function Attrs: alwaysinline mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define linkonce_odr hidden i32 @__ockl_lane_u32() local_unnamed_addr #7 {
  %1 = tail call i32 @llvm.amdgcn.mbcnt.lo(i32 -1, i32 0)
  %2 = tail call i32 @llvm.amdgcn.mbcnt.hi(i32 -1, i32 %1)
  ret i32 %2
}

; Function Attrs: convergent nocallback nofree nounwind willreturn memory(none)
declare i32 @llvm.amdgcn.readfirstlane.i32(i32) #4

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.sleep(i32 immarg) #8

; Function Attrs: convergent nocallback nofree nounwind willreturn memory(none)
declare i64 @llvm.amdgcn.ballot.i64(i1) #4

; Function Attrs: convergent norecurse nounwind
define linkonce_odr hidden void @__ockl_hsa_signal_add(i64 %0, i64 noundef %1, i32 noundef %2) local_unnamed_addr #5 {
  %4 = inttoptr i64 %0 to ptr addrspace(1)
  %5 = getelementptr inbounds i8, ptr addrspace(1) %4, i64 8
  switch i32 %2, label %6 [
    i32 1, label %8
    i32 2, label %8
    i32 3, label %10
    i32 4, label %12
    i32 5, label %14
  ]

6:                                                ; preds = %3
  %7 = atomicrmw add ptr addrspace(1) %5, i64 %1 syncscope("one-as") monotonic, align 8
  br label %16

8:                                                ; preds = %3, %3
  %9 = atomicrmw add ptr addrspace(1) %5, i64 %1 syncscope("one-as") acquire, align 8
  br label %16

10:                                               ; preds = %3
  %11 = atomicrmw add ptr addrspace(1) %5, i64 %1 syncscope("one-as") release, align 8
  br label %16

12:                                               ; preds = %3
  %13 = atomicrmw add ptr addrspace(1) %5, i64 %1 syncscope("one-as") acq_rel, align 8
  br label %16

14:                                               ; preds = %3
  %15 = atomicrmw add ptr addrspace(1) %5, i64 %1 seq_cst, align 8
  br label %16

16:                                               ; preds = %14, %12, %10, %8, %6
  %17 = getelementptr inbounds i8, ptr addrspace(1) %4, i64 16
  %18 = load i64, ptr addrspace(1) %17, align 16, !tbaa !25
  %19 = icmp eq i64 %18, 0
  br i1 %19, label %34, label %20

20:                                               ; preds = %16
  %21 = inttoptr i64 %18 to ptr addrspace(1)
  %22 = getelementptr inbounds i8, ptr addrspace(1) %4, i64 24
  %23 = load i32, ptr addrspace(1) %22, align 8, !tbaa !27
  %24 = zext i32 %23 to i64
  store atomic i64 %24, ptr addrspace(1) %21 syncscope("one-as") release, align 8
  %25 = load i32, ptr addrspace(4) @__oclc_ISA_version, align 4, !tbaa !8
  %26 = icmp slt i32 %25, 9000
  %27 = icmp ult i32 %25, 10000
  %28 = icmp ult i32 %25, 11000
  %29 = select i1 %28, i32 8388607, i32 16777215
  %30 = select i1 %27, i32 16777215, i32 %29
  %31 = select i1 %26, i32 255, i32 %30
  %32 = and i32 %31, %23
  %33 = tail call i32 @llvm.amdgcn.readfirstlane.i32(i32 %32)
  tail call void @llvm.amdgcn.s.sendmsg(i32 1, i32 %33)
  br label %34

34:                                               ; preds = %20, %16
  ret void
}

; Function Attrs: nounwind
declare void @llvm.amdgcn.s.sendmsg(i32 immarg, i32) #9

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i32 @llvm.amdgcn.mbcnt.hi(i32, i32) #3

; Function Attrs: convergent norecurse nounwind
define weak hidden void @__ockl_dm_init_v1(i64 noundef %0, i64 noundef %1, i32 noundef %2, i32 noundef %3) local_unnamed_addr #5 {
  %5 = tail call i64 @__ockl_get_local_id(i32 noundef 0) #17
  %6 = icmp eq i32 %2, 0
  br i1 %6, label %43, label %7

7:                                                ; preds = %4
  %8 = shl i64 %5, 4
  %9 = and i64 %8, 4294967280
  %10 = add i64 %9, %0
  %11 = inttoptr i64 %10 to ptr addrspace(1)
  store <4 x i32> zeroinitializer, ptr addrspace(1) %11, align 16, !tbaa !28
  %12 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 4096
  store <4 x i32> zeroinitializer, ptr addrspace(1) %12, align 16, !tbaa !28
  %13 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 8192
  store <4 x i32> zeroinitializer, ptr addrspace(1) %13, align 16, !tbaa !28
  %14 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 12288
  store <4 x i32> zeroinitializer, ptr addrspace(1) %14, align 16, !tbaa !28
  %15 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 16384
  store <4 x i32> zeroinitializer, ptr addrspace(1) %15, align 16, !tbaa !28
  %16 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 20480
  store <4 x i32> zeroinitializer, ptr addrspace(1) %16, align 16, !tbaa !28
  %17 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 24576
  store <4 x i32> zeroinitializer, ptr addrspace(1) %17, align 16, !tbaa !28
  %18 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 28672
  store <4 x i32> zeroinitializer, ptr addrspace(1) %18, align 16, !tbaa !28
  %19 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 32768
  store <4 x i32> zeroinitializer, ptr addrspace(1) %19, align 16, !tbaa !28
  %20 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 36864
  store <4 x i32> zeroinitializer, ptr addrspace(1) %20, align 16, !tbaa !28
  %21 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 40960
  store <4 x i32> zeroinitializer, ptr addrspace(1) %21, align 16, !tbaa !28
  %22 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 45056
  store <4 x i32> zeroinitializer, ptr addrspace(1) %22, align 16, !tbaa !28
  %23 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 49152
  store <4 x i32> zeroinitializer, ptr addrspace(1) %23, align 16, !tbaa !28
  %24 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 53248
  store <4 x i32> zeroinitializer, ptr addrspace(1) %24, align 16, !tbaa !28
  %25 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 57344
  store <4 x i32> zeroinitializer, ptr addrspace(1) %25, align 16, !tbaa !28
  %26 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 61440
  store <4 x i32> zeroinitializer, ptr addrspace(1) %26, align 16, !tbaa !28
  %27 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 65536
  store <4 x i32> zeroinitializer, ptr addrspace(1) %27, align 16, !tbaa !28
  %28 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 69632
  store <4 x i32> zeroinitializer, ptr addrspace(1) %28, align 16, !tbaa !28
  %29 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 73728
  store <4 x i32> zeroinitializer, ptr addrspace(1) %29, align 16, !tbaa !28
  %30 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 77824
  store <4 x i32> zeroinitializer, ptr addrspace(1) %30, align 16, !tbaa !28
  %31 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 81920
  store <4 x i32> zeroinitializer, ptr addrspace(1) %31, align 16, !tbaa !28
  %32 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 86016
  store <4 x i32> zeroinitializer, ptr addrspace(1) %32, align 16, !tbaa !28
  %33 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 90112
  store <4 x i32> zeroinitializer, ptr addrspace(1) %33, align 16, !tbaa !28
  %34 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 94208
  store <4 x i32> zeroinitializer, ptr addrspace(1) %34, align 16, !tbaa !28
  %35 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 98304
  store <4 x i32> zeroinitializer, ptr addrspace(1) %35, align 16, !tbaa !28
  %36 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 102400
  store <4 x i32> zeroinitializer, ptr addrspace(1) %36, align 16, !tbaa !28
  %37 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 106496
  store <4 x i32> zeroinitializer, ptr addrspace(1) %37, align 16, !tbaa !28
  %38 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 110592
  store <4 x i32> zeroinitializer, ptr addrspace(1) %38, align 16, !tbaa !28
  %39 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 114688
  store <4 x i32> zeroinitializer, ptr addrspace(1) %39, align 16, !tbaa !28
  %40 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 118784
  store <4 x i32> zeroinitializer, ptr addrspace(1) %40, align 16, !tbaa !28
  %41 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 122880
  store <4 x i32> zeroinitializer, ptr addrspace(1) %41, align 16, !tbaa !28
  %42 = getelementptr inbounds i8, ptr addrspace(1) %11, i64 126976
  store <4 x i32> zeroinitializer, ptr addrspace(1) %42, align 16, !tbaa !28
  br label %43

43:                                               ; preds = %7, %4
  fence syncscope("agent") release, !mmra !29
  tail call void @llvm.amdgcn.s.barrier()
  %44 = and i64 %5, 4294967295
  %45 = icmp eq i64 %44, 0
  br i1 %45, label %46, label %54

46:                                               ; preds = %43
  %47 = inttoptr i64 %0 to ptr addrspace(1)
  %48 = getelementptr inbounds i8, ptr addrspace(1) %47, i64 108544
  store atomic i64 %1, ptr addrspace(1) %48 syncscope("agent-one-as") monotonic, align 8
  %49 = zext i32 %3 to i64
  %50 = shl nuw nsw i64 %49, 21
  %51 = add i64 %50, %1
  %52 = getelementptr inbounds i8, ptr addrspace(1) %47, i64 108552
  store i64 %51, ptr addrspace(1) %52, align 8, !tbaa !30
  %53 = getelementptr inbounds i8, ptr addrspace(1) %47, i64 108560
  store i64 %1, ptr addrspace(1) %53, align 8, !tbaa !32
  br label %54

54:                                               ; preds = %46, %43
  ret void
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define linkonce_odr hidden range(i64 0, 1024) i64 @__ockl_get_local_id(i32 noundef %0) local_unnamed_addr #10 {
  switch i32 %0, label %8 [
    i32 0, label %2
    i32 1, label %4
    i32 2, label %6
  ]

2:                                                ; preds = %1
  %3 = tail call noundef range(i32 0, 1024) i32 @llvm.amdgcn.workitem.id.x()
  br label %8

4:                                                ; preds = %1
  %5 = tail call noundef range(i32 0, 1024) i32 @llvm.amdgcn.workitem.id.y()
  br label %8

6:                                                ; preds = %1
  %7 = tail call noundef range(i32 0, 1024) i32 @llvm.amdgcn.workitem.id.z()
  br label %8

8:                                                ; preds = %6, %4, %2, %1
  %9 = phi i32 [ %7, %6 ], [ %5, %4 ], [ %3, %2 ], [ 0, %1 ]
  %10 = zext nneg i32 %9 to i64
  ret i64 %10
}

; Function Attrs: convergent nocallback nofree nounwind willreturn
declare void @llvm.amdgcn.s.barrier() #11

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare noundef i32 @llvm.amdgcn.workitem.id.x() #6

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare noundef i32 @llvm.amdgcn.workitem.id.y() #6

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare noundef i32 @llvm.amdgcn.workitem.id.z() #6

; Function Attrs: cold convergent norecurse nounwind optsize
define weak hidden void @__ockl_dm_trim(ptr noundef %0) local_unnamed_addr #12 {
  %2 = addrspacecast ptr %0 to ptr addrspace(3)
  %3 = load i8, ptr addrspace(4) @__oclc_wavefrontsize64, align 1, !tbaa !33, !range !35, !noundef !36
  %4 = trunc nuw i8 %3 to i1
  %5 = select i1 %4, i32 128, i32 64
  %6 = getelementptr inbounds i32, ptr addrspace(3) %2, i32 %5
  %7 = load i32, ptr addrspace(4) @__oclc_ABI_version, align 4, !tbaa !8
  %8 = icmp slt i32 %7, 500
  br i1 %8, label %14, label %9

9:                                                ; preds = %1
  %10 = tail call ptr addrspace(4) @llvm.amdgcn.implicitarg.ptr()
  %11 = getelementptr inbounds i8, ptr addrspace(4) %10, i64 96
  %12 = load i64, ptr addrspace(4) %11, align 8, !tbaa !12
  %13 = inttoptr i64 %12 to ptr addrspace(1)
  br label %14

14:                                               ; preds = %9, %1
  %15 = phi ptr addrspace(1) [ %13, %9 ], [ @0, %1 ]
  %16 = getelementptr inbounds i8, ptr addrspace(1) %15, i64 108560
  %17 = load i64, ptr addrspace(1) %16, align 8, !tbaa !32
  %18 = getelementptr inbounds i8, ptr addrspace(1) %15, i64 108552
  %19 = load i64, ptr addrspace(1) %18, align 8, !tbaa !30
  %20 = tail call i32 @__ockl_lane_u32() #18
  %21 = icmp eq i32 %20, 0
  %22 = getelementptr inbounds i8, ptr addrspace(1) %15, i64 2048
  %23 = select i1 %4, i32 64, i32 32
  %24 = select i1 %4, i32 -64, i32 -32
  %25 = getelementptr i8, ptr addrspace(3) %2, i32 -4
  %26 = getelementptr inbounds i8, ptr addrspace(1) %15, i64 10240
  %27 = getelementptr i32, ptr addrspace(3) %2, i32 %20
  %28 = getelementptr i32, ptr addrspace(3) %6, i32 %20
  br label %30

29:                                               ; preds = %276
  ret void

30:                                               ; preds = %276, %14
  %31 = phi i32 [ 0, %14 ], [ %277, %276 ]
  br i1 %21, label %32, label %36

32:                                               ; preds = %30
  %33 = zext nneg i32 %31 to i64
  %34 = getelementptr inbounds [16 x %1], ptr addrspace(1) %22, i64 0, i64 %33
  %35 = load atomic i32, ptr addrspace(1) %34 syncscope("agent-one-as") monotonic, align 8
  br label %36

36:                                               ; preds = %32, %30
  %37 = phi i32 [ %35, %32 ], [ 0, %30 ]
  %38 = tail call i32 @llvm.amdgcn.readfirstlane.i32(i32 %37)
  %39 = icmp eq i32 %38, 0
  br i1 %39, label %276, label %40

40:                                               ; preds = %36
  %41 = add i32 %38, -1
  %42 = and i32 %41, %24
  %43 = zext nneg i32 %31 to i64
  br label %44

44:                                               ; preds = %236, %40
  %45 = phi i32 [ 0, %40 ], [ %237, %236 ]
  %46 = phi i32 [ 0, %40 ], [ %226, %236 ]
  %47 = phi i32 [ %42, %40 ], [ %161, %236 ]
  %48 = phi i32 [ 0, %40 ], [ %238, %236 ]
  %49 = phi i32 [ 0, %40 ], [ %103, %236 ]
  %50 = icmp ult i32 %49, %38
  %51 = icmp ult i32 %45, %23
  %52 = select i1 %50, i1 %51, i1 false
  br i1 %52, label %53, label %101

53:                                               ; preds = %91, %44
  %54 = phi i32 [ %97, %91 ], [ %49, %44 ]
  %55 = phi i32 [ %96, %91 ], [ %45, %44 ]
  %56 = add i32 %54, %20
  %57 = icmp ult i32 %56, %38
  br i1 %57, label %58, label %91

58:                                               ; preds = %53
  %59 = icmp ugt i32 %56, 255
  br i1 %59, label %60, label %70

60:                                               ; preds = %58
  %61 = add i32 %56, -256
  %62 = lshr i32 %61, 8
  %63 = zext nneg i32 %62 to i64
  %64 = getelementptr inbounds [16 x [256 x %3]], ptr addrspace(1) %26, i64 0, i64 %43, i64 %63
  %65 = load atomic i64, ptr addrspace(1) %64 syncscope("agent-one-as") monotonic, align 8
  %66 = inttoptr i64 %65 to ptr addrspace(1)
  %67 = and i32 %56, 255
  %68 = zext nneg i32 %67 to i64
  %69 = getelementptr inbounds %3, ptr addrspace(1) %66, i64 %68
  br label %73

70:                                               ; preds = %58
  %71 = zext nneg i32 %56 to i64
  %72 = getelementptr inbounds [16 x [256 x %3]], ptr addrspace(1) %26, i64 0, i64 %43, i64 %71
  br label %73

73:                                               ; preds = %70, %60
  %74 = phi ptr addrspace(1) [ %69, %60 ], [ %72, %70 ]
  %75 = getelementptr inbounds i8, ptr addrspace(1) %74, i64 16
  %76 = load atomic i32, ptr addrspace(1) %75 syncscope("agent-one-as") monotonic, align 8
  %77 = getelementptr inbounds i8, ptr addrspace(1) %74, i64 8
  %78 = load atomic i64, ptr addrspace(1) %77 syncscope("agent-one-as") monotonic, align 8
  %79 = icmp eq i32 %76, 0
  %80 = icmp ne i64 %78, 0
  %81 = icmp ult i64 %78, %17
  %82 = icmp uge i64 %78, %19
  %83 = or i1 %81, %82
  %84 = and i1 %80, %83
  %85 = select i1 %79, i1 %84, i1 false
  br i1 %85, label %86, label %91

86:                                               ; preds = %73
  %87 = tail call i64 @__ockl_devmem_request(i64 noundef %78, i64 noundef 0) #18
  store atomic i64 0, ptr addrspace(1) %77 syncscope("agent-one-as") monotonic, align 8
  store atomic i32 0, ptr addrspace(1) %75 syncscope("agent-one-as") monotonic, align 8
  %88 = tail call i32 @__ockl_activelane_u32() #18
  %89 = getelementptr i32, ptr addrspace(3) %2, i32 %88
  %90 = getelementptr i32, ptr addrspace(3) %89, i32 %55
  store i32 %56, ptr addrspace(3) %90, align 4, !tbaa !8
  br label %91

91:                                               ; preds = %86, %73, %53
  %92 = phi i1 [ false, %53 ], [ true, %86 ], [ false, %73 ]
  %93 = tail call i64 @llvm.amdgcn.ballot.i64(i1 %92)
  %94 = tail call range(i64 0, 65) i64 @llvm.ctpop.i64(i64 %93)
  %95 = trunc nuw nsw i64 %94 to i32
  %96 = add nuw nsw i32 %55, %95
  %97 = add i32 %54, %23
  %98 = icmp ult i32 %97, %38
  %99 = icmp ult i32 %96, %23
  %100 = select i1 %98, i1 %99, i1 false
  br i1 %100, label %53, label %101

101:                                              ; preds = %91, %44
  %102 = phi i32 [ %45, %44 ], [ %96, %91 ]
  %103 = phi i32 [ %49, %44 ], [ %97, %91 ]
  %104 = phi i1 [ %50, %44 ], [ %98, %91 ]
  %105 = icmp eq i32 %102, 0
  br i1 %105, label %239, label %106

106:                                              ; preds = %101
  %107 = icmp ult i32 %47, %38
  %108 = icmp ult i32 %48, %23
  %109 = select i1 %107, i1 %108, i1 false
  br i1 %109, label %110, label %160

110:                                              ; preds = %149, %106
  %111 = phi i32 [ %155, %149 ], [ %48, %106 ]
  %112 = phi i32 [ %156, %149 ], [ %47, %106 ]
  %113 = add i32 %112, %20
  %114 = icmp ult i32 %113, %38
  br i1 %114, label %115, label %149

115:                                              ; preds = %110
  %116 = icmp ugt i32 %113, 255
  br i1 %116, label %117, label %127

117:                                              ; preds = %115
  %118 = add i32 %113, -256
  %119 = lshr i32 %118, 8
  %120 = zext nneg i32 %119 to i64
  %121 = getelementptr inbounds [16 x [256 x %3]], ptr addrspace(1) %26, i64 0, i64 %43, i64 %120
  %122 = load atomic i64, ptr addrspace(1) %121 syncscope("agent-one-as") monotonic, align 8
  %123 = inttoptr i64 %122 to ptr addrspace(1)
  %124 = and i32 %113, 255
  %125 = zext nneg i32 %124 to i64
  %126 = getelementptr inbounds %3, ptr addrspace(1) %123, i64 %125
  br label %130

127:                                              ; preds = %115
  %128 = zext nneg i32 %113 to i64
  %129 = getelementptr inbounds [16 x [256 x %3]], ptr addrspace(1) %26, i64 0, i64 %43, i64 %128
  br label %130

130:                                              ; preds = %127, %117
  %131 = phi ptr addrspace(1) [ %126, %117 ], [ %129, %127 ]
  %132 = getelementptr inbounds i8, ptr addrspace(1) %131, i64 8
  %133 = load atomic i64, ptr addrspace(1) %132 syncscope("agent-one-as") monotonic, align 8
  %134 = getelementptr inbounds i8, ptr addrspace(1) %131, i64 16
  %135 = load atomic i32, ptr addrspace(1) %134 syncscope("agent-one-as") monotonic, align 8
  %136 = icmp ne i32 %135, 0
  %137 = icmp uge i64 %133, %17
  %138 = icmp ult i64 %133, %19
  %139 = and i1 %137, %138
  %140 = select i1 %136, i1 true, i1 %139
  br i1 %140, label %141, label %145

141:                                              ; preds = %130
  %142 = tail call i32 @__ockl_activelane_u32() #18
  %143 = getelementptr i32, ptr addrspace(3) %6, i32 %142
  %144 = getelementptr i32, ptr addrspace(3) %143, i32 %111
  store i32 %113, ptr addrspace(3) %144, align 4, !tbaa !8
  br label %149

145:                                              ; preds = %130
  %146 = icmp eq i64 %133, 0
  br i1 %146, label %149, label %147

147:                                              ; preds = %145
  %148 = tail call i64 @__ockl_devmem_request(i64 noundef %133, i64 noundef 0) #18
  store atomic i64 0, ptr addrspace(1) %132 syncscope("agent-one-as") monotonic, align 8
  store atomic i32 0, ptr addrspace(1) %134 syncscope("agent-one-as") monotonic, align 8
  br label %149

149:                                              ; preds = %147, %145, %141, %110
  %150 = phi i1 [ false, %110 ], [ false, %145 ], [ false, %147 ], [ true, %141 ]
  %151 = tail call i64 @llvm.amdgcn.ballot.i64(i1 %150)
  %152 = tail call range(i64 0, 65) i64 @llvm.ctpop.i64(i64 %151)
  %153 = trunc nuw nsw i64 %152 to i32
  %154 = getelementptr inbounds i32, ptr addrspace(3) %6, i32 %111
  tail call fastcc void @1(ptr addrspace(3) noundef %154, i32 noundef %20, i32 noundef %153) #18
  %155 = add nuw nsw i32 %111, %153
  %156 = sub i32 %112, %23
  %157 = icmp ult i32 %156, %38
  %158 = icmp ult i32 %155, %23
  %159 = select i1 %157, i1 %158, i1 false
  br i1 %159, label %110, label %160

160:                                              ; preds = %149, %106
  %161 = phi i32 [ %47, %106 ], [ %156, %149 ]
  %162 = phi i32 [ %48, %106 ], [ %155, %149 ]
  %163 = icmp eq i32 %162, 0
  br i1 %163, label %239, label %164

164:                                              ; preds = %160
  %165 = tail call i32 @llvm.umin.i32(i32 %102, i32 %162)
  %166 = tail call i32 @llvm.umin.i32(i32 %23, i32 %165)
  %167 = icmp ugt i32 %166, %20
  br i1 %167, label %168, label %172

168:                                              ; preds = %164
  %169 = load i32, ptr addrspace(3) %27, align 4, !tbaa !8
  %170 = load i32, ptr addrspace(3) %28, align 4, !tbaa !8
  %171 = icmp ult i32 %169, %170
  br label %172

172:                                              ; preds = %168, %164
  %173 = phi i1 [ false, %164 ], [ %171, %168 ]
  br i1 %173, label %174, label %217

174:                                              ; preds = %172
  %175 = load i32, ptr addrspace(3) %27, align 4, !tbaa !8
  %176 = icmp ugt i32 %175, 255
  br i1 %176, label %177, label %187

177:                                              ; preds = %174
  %178 = add i32 %175, -256
  %179 = lshr i32 %178, 8
  %180 = zext nneg i32 %179 to i64
  %181 = getelementptr inbounds [16 x [256 x %3]], ptr addrspace(1) %26, i64 0, i64 %43, i64 %180
  %182 = load atomic i64, ptr addrspace(1) %181 syncscope("agent-one-as") monotonic, align 8
  %183 = inttoptr i64 %182 to ptr addrspace(1)
  %184 = and i32 %175, 255
  %185 = zext nneg i32 %184 to i64
  %186 = getelementptr inbounds %3, ptr addrspace(1) %183, i64 %185
  br label %190

187:                                              ; preds = %174
  %188 = zext nneg i32 %175 to i64
  %189 = getelementptr inbounds [16 x [256 x %3]], ptr addrspace(1) %26, i64 0, i64 %43, i64 %188
  br label %190

190:                                              ; preds = %187, %177
  %191 = phi ptr addrspace(1) [ %186, %177 ], [ %189, %187 ]
  %192 = load i32, ptr addrspace(3) %28, align 4, !tbaa !8
  %193 = icmp ugt i32 %192, 255
  br i1 %193, label %194, label %204

194:                                              ; preds = %190
  %195 = add i32 %192, -256
  %196 = lshr i32 %195, 8
  %197 = zext nneg i32 %196 to i64
  %198 = getelementptr inbounds [16 x [256 x %3]], ptr addrspace(1) %26, i64 0, i64 %43, i64 %197
  %199 = load atomic i64, ptr addrspace(1) %198 syncscope("agent-one-as") monotonic, align 8
  %200 = inttoptr i64 %199 to ptr addrspace(1)
  %201 = and i32 %192, 255
  %202 = zext nneg i32 %201 to i64
  %203 = getelementptr inbounds %3, ptr addrspace(1) %200, i64 %202
  br label %207

204:                                              ; preds = %190
  %205 = zext nneg i32 %192 to i64
  %206 = getelementptr inbounds [16 x [256 x %3]], ptr addrspace(1) %26, i64 0, i64 %43, i64 %205
  br label %207

207:                                              ; preds = %204, %194
  %208 = phi ptr addrspace(1) [ %203, %194 ], [ %206, %204 ]
  %209 = getelementptr inbounds i8, ptr addrspace(1) %208, i64 8
  %210 = load atomic i64, ptr addrspace(1) %209 syncscope("agent-one-as") monotonic, align 8
  %211 = inttoptr i64 %210 to ptr addrspace(1)
  %212 = getelementptr inbounds i8, ptr addrspace(1) %211, i64 4
  store i32 %175, ptr addrspace(1) %212, align 4, !tbaa !37
  %213 = getelementptr inbounds i8, ptr addrspace(1) %191, i64 8
  store atomic i64 %210, ptr addrspace(1) %213 syncscope("agent-one-as") monotonic, align 8
  %214 = getelementptr inbounds i8, ptr addrspace(1) %191, i64 16
  %215 = getelementptr inbounds i8, ptr addrspace(1) %208, i64 16
  %216 = load atomic i32, ptr addrspace(1) %215 syncscope("agent-one-as") monotonic, align 8
  store atomic i32 %216, ptr addrspace(1) %214 syncscope("agent-one-as") monotonic, align 8
  store atomic i64 0, ptr addrspace(1) %209 syncscope("agent-one-as") monotonic, align 8
  store atomic i32 0, ptr addrspace(1) %215 syncscope("agent-one-as") monotonic, align 8
  br label %217

217:                                              ; preds = %207, %172
  %218 = tail call i64 @llvm.amdgcn.ballot.i64(i1 %173)
  %219 = tail call range(i64 0, 65) i64 @llvm.ctpop.i64(i64 %218)
  %220 = trunc nuw nsw i64 %219 to i32
  %221 = icmp eq i64 %218, 0
  br i1 %221, label %225, label %222

222:                                              ; preds = %217
  %223 = getelementptr i32, ptr addrspace(3) %25, i32 %220
  %224 = load i32, ptr addrspace(3) %223, align 4, !tbaa !8
  br label %225

225:                                              ; preds = %222, %217
  %226 = phi i32 [ %224, %222 ], [ %46, %217 ]
  %227 = icmp eq i32 %166, %220
  %228 = and i1 %104, %227
  br i1 %228, label %229, label %236

229:                                              ; preds = %225
  %230 = getelementptr i32, ptr addrspace(3) %27, i32 %166
  %231 = getelementptr i32, ptr addrspace(3) %28, i32 %166
  %232 = sub i32 %102, %166
  %233 = sub i32 %162, %166
  %234 = load i32, ptr addrspace(3) %230, align 4, !tbaa !8
  store i32 %234, ptr addrspace(3) %27, align 4, !tbaa !8
  %235 = load i32, ptr addrspace(3) %231, align 4, !tbaa !8
  store i32 %235, ptr addrspace(3) %28, align 4, !tbaa !8
  br label %236

236:                                              ; preds = %229, %225
  %237 = phi i32 [ %232, %229 ], [ %102, %225 ]
  %238 = phi i32 [ %233, %229 ], [ %162, %225 ]
  br i1 %228, label %44, label %239

239:                                              ; preds = %236, %160, %101
  %240 = phi i32 [ %46, %101 ], [ %46, %160 ], [ %226, %236 ]
  %241 = and i32 %240, %24
  br label %242

242:                                              ; preds = %266, %239
  %243 = phi i32 [ %241, %239 ], [ %271, %266 ]
  %244 = add i32 %243, %20
  %245 = icmp ult i32 %244, %38
  br i1 %245, label %246, label %266

246:                                              ; preds = %242
  %247 = icmp ugt i32 %244, 255
  br i1 %247, label %248, label %258

248:                                              ; preds = %246
  %249 = add i32 %244, -256
  %250 = lshr i32 %249, 8
  %251 = zext nneg i32 %250 to i64
  %252 = getelementptr inbounds [16 x [256 x %3]], ptr addrspace(1) %26, i64 0, i64 %43, i64 %251
  %253 = load atomic i64, ptr addrspace(1) %252 syncscope("agent-one-as") monotonic, align 8
  %254 = inttoptr i64 %253 to ptr addrspace(1)
  %255 = and i32 %244, 255
  %256 = zext nneg i32 %255 to i64
  %257 = getelementptr inbounds %3, ptr addrspace(1) %254, i64 %256
  br label %261

258:                                              ; preds = %246
  %259 = zext nneg i32 %244 to i64
  %260 = getelementptr inbounds [16 x [256 x %3]], ptr addrspace(1) %26, i64 0, i64 %43, i64 %259
  br label %261

261:                                              ; preds = %258, %248
  %262 = phi ptr addrspace(1) [ %257, %248 ], [ %260, %258 ]
  %263 = getelementptr inbounds i8, ptr addrspace(1) %262, i64 8
  %264 = load atomic i64, ptr addrspace(1) %263 syncscope("agent-one-as") monotonic, align 8
  %265 = icmp ne i64 %264, 0
  br label %266

266:                                              ; preds = %261, %242
  %267 = phi i1 [ %265, %261 ], [ false, %242 ]
  %268 = tail call i64 @llvm.amdgcn.ballot.i64(i1 %267)
  %269 = tail call range(i64 0, 65) i64 @llvm.ctpop.i64(i64 %268)
  %270 = trunc nuw nsw i64 %269 to i32
  %271 = add i32 %243, %270
  %272 = icmp eq i32 %23, %270
  br i1 %272, label %242, label %273

273:                                              ; preds = %266
  br i1 %21, label %274, label %276

274:                                              ; preds = %273
  %275 = getelementptr inbounds [16 x %1], ptr addrspace(1) %22, i64 0, i64 %43
  store atomic i32 %271, ptr addrspace(1) %275 syncscope("agent-one-as") monotonic, align 8
  br label %276

276:                                              ; preds = %274, %273, %36
  %277 = add nuw nsw i32 %31, 1
  %278 = icmp eq i32 %277, 16
  br i1 %278, label %29, label %30
}

; Function Attrs: convergent mustprogress nofree norecurse nounwind willreturn memory(none)
define linkonce_odr hidden i32 @__ockl_activelane_u32() local_unnamed_addr #13 {
  %1 = tail call i64 @llvm.amdgcn.ballot.i64(i1 true)
  %2 = lshr i64 %1, 32
  %3 = trunc nuw i64 %2 to i32
  %4 = tail call i32 @llvm.amdgcn.ballot.i32(i1 true)
  %5 = tail call i32 @llvm.amdgcn.mbcnt.lo(i32 %4, i32 0)
  %6 = tail call i32 @llvm.amdgcn.mbcnt.hi(i32 %3, i32 %5)
  ret i32 %6
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.ctpop.i64(i64) #6

; Function Attrs: convergent mustprogress nofree norecurse nounwind willreturn memory(argmem: readwrite)
define internal fastcc void @1(ptr addrspace(3) nocapture noundef %0, i32 noundef %1, i32 noundef %2) unnamed_addr #14 {
  %4 = icmp ult i32 %1, %2
  br i1 %4, label %5, label %12

5:                                                ; preds = %3
  %6 = getelementptr inbounds i32, ptr addrspace(3) %0, i32 %1
  %7 = xor i32 %1, -1
  %8 = add i32 %7, %2
  %9 = shl nsw i32 %8, 2
  %10 = load i32, ptr addrspace(3) %6, align 4, !tbaa !8
  %11 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %9, i32 %10)
  store i32 %11, ptr addrspace(3) %6, align 4, !tbaa !8
  br label %12

12:                                               ; preds = %5, %3
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.umin.i32(i32, i32) #6

; Function Attrs: convergent nocallback nofree nounwind willreturn memory(none)
declare i32 @llvm.amdgcn.ballot.i32(i1) #4

; Function Attrs: convergent mustprogress nofree norecurse nounwind willreturn memory(none)
define linkonce_odr hidden i32 @__ockl_sdot4(<4 x i8> noundef %0, <4 x i8> noundef %1, i32 noundef %2, i1 noundef zeroext %3) local_unnamed_addr #13 {
  %5 = load i32, ptr addrspace(4) @__oclc_ISA_version, align 4
  %6 = freeze i32 %5
  %7 = icmp slt i32 %6, 9006
  br i1 %7, label %9, label %8

8:                                                ; preds = %4
  switch i32 %6, label %37 [
    i32 10100, label %9
    i32 9009, label %9
  ]

9:                                                ; preds = %8, %8, %4
  %10 = extractelement <4 x i8> %0, i64 0
  %11 = sext i8 %10 to i32
  %12 = extractelement <4 x i8> %1, i64 0
  %13 = sext i8 %12 to i32
  %14 = mul nsw i32 %13, %11
  %15 = extractelement <4 x i8> %0, i64 1
  %16 = sext i8 %15 to i32
  %17 = extractelement <4 x i8> %1, i64 1
  %18 = sext i8 %17 to i32
  %19 = mul nsw i32 %18, %16
  %20 = add nsw i32 %14, %19
  %21 = extractelement <4 x i8> %0, i64 2
  %22 = sext i8 %21 to i32
  %23 = extractelement <4 x i8> %1, i64 2
  %24 = sext i8 %23 to i32
  %25 = mul nsw i32 %24, %22
  %26 = add nsw i32 %20, %25
  %27 = extractelement <4 x i8> %0, i64 3
  %28 = sext i8 %27 to i32
  %29 = extractelement <4 x i8> %1, i64 3
  %30 = sext i8 %29 to i32
  %31 = mul nsw i32 %30, %28
  %32 = add nsw i32 %26, %31
  br i1 %3, label %33, label %35

33:                                               ; preds = %9
  %34 = tail call i32 @__ockl_add_sat_i32(i32 noundef %32, i32 noundef %2) #17
  br label %45

35:                                               ; preds = %9
  %36 = add nsw i32 %32, %2
  br label %45

37:                                               ; preds = %8
  %38 = icmp ugt i32 %6, 10999
  %39 = bitcast <4 x i8> %0 to i32
  %40 = bitcast <4 x i8> %1 to i32
  br i1 %38, label %41, label %43

41:                                               ; preds = %37
  %42 = tail call fastcc i32 @2(i32 noundef %39, i32 noundef %40, i32 noundef %2, i1 noundef zeroext %3) #20
  br label %45

43:                                               ; preds = %37
  %44 = tail call fastcc i32 @3(i32 noundef %39, i32 noundef %40, i32 noundef %2, i1 noundef zeroext %3) #20
  br label %45

45:                                               ; preds = %43, %41, %35, %33
  %46 = phi i32 [ %42, %41 ], [ %44, %43 ], [ %34, %33 ], [ %36, %35 ]
  ret i32 %46
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define linkonce_odr hidden noundef i32 @__ockl_add_sat_i32(i32 noundef %0, i32 noundef %1) local_unnamed_addr #10 {
  %3 = tail call i32 @llvm.sadd.sat.i32(i32 %0, i32 %1)
  ret i32 %3
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define internal fastcc i32 @2(i32 noundef %0, i32 noundef %1, i32 noundef %2, i1 noundef zeroext %3) unnamed_addr #15 {
  %5 = tail call i32 @llvm.amdgcn.sudot4(i1 true, i32 %0, i1 true, i32 %1, i32 %2, i1 true)
  %6 = tail call i32 @llvm.amdgcn.sudot4(i1 true, i32 %0, i1 true, i32 %1, i32 %2, i1 false)
  %7 = select i1 %3, i32 %5, i32 %6
  ret i32 %7
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define internal fastcc i32 @3(i32 noundef %0, i32 noundef %1, i32 noundef %2, i1 noundef zeroext %3) unnamed_addr #16 {
  %5 = tail call i32 @llvm.amdgcn.sdot4(i32 %0, i32 %1, i32 %2, i1 true)
  %6 = tail call i32 @llvm.amdgcn.sdot4(i32 %0, i32 %1, i32 %2, i1 false)
  %7 = select i1 %3, i32 %5, i32 %6
  ret i32 %7
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.amdgcn.sdot4(i32, i32, i32, i1 immarg) #6

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.amdgcn.sudot4(i1 immarg, i32, i1 immarg, i32, i32, i1 immarg) #6

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.sadd.sat.i32(i32, i32) #6

; Function Attrs: convergent norecurse nounwind
define weak hidden void @__ockl_sanitizer_report(i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4, i64 noundef %5, i64 noundef %6, i64 noundef %7) local_unnamed_addr #5 {
  %9 = tail call <2 x i64> @__ockl_hostcall_preview(i32 noundef 4, i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i64 noundef %4, i64 noundef %5, i64 noundef %6, i64 noundef %7) #18
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { convergent mustprogress nofree nounwind willreturn memory(none) "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" }
attributes #3 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #4 = { convergent nocallback nofree nounwind willreturn memory(none) }
attributes #5 = { convergent norecurse nounwind "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" }
attributes #6 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #7 = { alwaysinline mustprogress nofree norecurse nosync nounwind willreturn memory(none) "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" }
attributes #8 = { nocallback nofree nosync nounwind willreturn }
attributes #9 = { nounwind }
attributes #10 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" }
attributes #11 = { convergent nocallback nofree nounwind willreturn }
attributes #12 = { cold convergent norecurse nounwind optsize "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" }
attributes #13 = { convergent mustprogress nofree norecurse nounwind willreturn memory(none) "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" }
attributes #14 = { convergent mustprogress nofree norecurse nounwind willreturn memory(argmem: readwrite) "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-features"="+gfx8-insts" }
attributes #15 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-features"="+dot8-insts" }
attributes #16 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-features"="+dot1-insts" }
attributes #17 = { convergent nounwind willreturn memory(none) }
attributes #18 = { convergent nounwind }
attributes #19 = { cold convergent nounwind }
attributes #20 = { nounwind willreturn memory(none) }

!llvm.ident = !{!0, !0, !0}
!opencl.ocl.version = !{!1, !1}
!llvm.module.flags = !{!2, !3}

!0 = !{!"AMD clang version 19.0.0git (https://github.com/RadeonOpenCompute/llvm-project roc-6.4.0 25133 c7fe45cf4b819c5991fe208aaa96edf142730f1d)"}
!1 = !{i32 2, i32 0}
!2 = !{i32 1, !"wchar_size", i32 4}
!3 = !{i32 8, !"PIC Level", i32 2}
!4 = !{!5, !5, i64 0}
!5 = !{!"int", !6, i64 0}
!6 = !{!"omnipotent char", !7, i64 0}
!7 = !{!"Simple C++ TBAA"}
!8 = !{!9, !9, i64 0}
!9 = !{!"int", !10, i64 0}
!10 = !{!"omnipotent char", !11, i64 0}
!11 = !{!"Simple C/C++ TBAA"}
!12 = !{!13, !13, i64 0}
!13 = !{!"long", !10, i64 0}
!14 = !{!15, !16, i64 0}
!15 = !{!"", !16, i64 0, !16, i64 8, !17, i64 16, !13, i64 24, !13, i64 32, !13, i64 40}
!16 = !{!"any pointer", !10, i64 0}
!17 = !{!"hsa_signal_s", !13, i64 0}
!18 = !{!15, !13, i64 40}
!19 = !{!15, !16, i64 8}
!20 = !{!21, !9, i64 16}
!21 = !{!"", !13, i64 0, !13, i64 8, !9, i64 16, !9, i64 20}
!22 = !{!21, !13, i64 8}
!23 = !{!21, !9, i64 20}
!24 = !{!21, !13, i64 0}
!25 = !{!26, !13, i64 16}
!26 = !{!"amd_signal_s", !13, i64 0, !10, i64 8, !13, i64 16, !9, i64 24, !9, i64 28, !13, i64 32, !13, i64 40, !10, i64 48, !10, i64 56}
!27 = !{!26, !9, i64 24}
!28 = !{!10, !10, i64 0}
!29 = !{!"amdgpu-as", !"global"}
!30 = !{!31, !13, i64 108552}
!31 = !{!"heap_s", !10, i64 0, !10, i64 2048, !10, i64 4096, !10, i64 6144, !10, i64 8192, !10, i64 10240, !10, i64 108544, !13, i64 108552, !13, i64 108560, !10, i64 108568, !10, i64 108680}
!32 = !{!31, !13, i64 108560}
!33 = !{!34, !34, i64 0}
!34 = !{!"bool", !10, i64 0}
!35 = !{i8 0, i8 2}
!36 = !{}
!37 = !{!38, !9, i64 4}
!38 = !{!"slab_s", !9, i64 0, !9, i64 4, !10, i64 8, !9, i64 12, !10, i64 16}
