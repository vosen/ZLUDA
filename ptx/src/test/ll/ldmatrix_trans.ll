@values_g = addrspace(1) global [256 x i16] [i16 1340, i16 122, i16 527, i16 693, i16 958, i16 394, i16 668, i16 432, i16 646, i16 354, i16 761, i16 449, i16 252, i16 778, i16 218, i16 800, i16 656, i16 493, i16 659, i16 787, i16 672, i16 203, i16 343, i16 845, i16 318, i16 286, i16 206, i16 253, i16 194, i16 489, i16 29, i16 323, i16 7, i16 619, i16 998, i16 930, i16 773, i16 749, i16 172, i16 465, i16 937, i16 96, i16 88, i16 621, i16 909, i16 298, i16 283, i16 286, i16 779, i16 290, i16 429, i16 930, i16 25, i16 687, i16 423, i16 200, i16 918, i16 10, i16 515, i16 248, i16 158, i16 911, i16 270, i16 459, i16 5832, i16 3864, i16 7868, i16 6538, i16 3898, i16 8685, i16 356, i16 3655, i16 3398, i16 8529, i16 2866, i16 1432, i16 4078, i16 1674, i16 498, i16 1124, i16 1576, i16 6490, i16 9895, i16 2152, i16 9668, i16 7349, i16 1948, i16 6239, i16 7944, i16 7630, i16 9699, i16 1957, i16 3360, i16 2291, i16 3832, i16 7370, i16 2683, i16 7465, i16 3107, i16 9822, i16 2510, i16 1642, i16 3240, i16 8860, i16 4935, i16 1935, i16 9328, i16 5164, i16 2759, i16 4816, i16 1049, i16 725, i16 9774, i16 5110, i16 5071, i16 8047, i16 7267, i16 7716, i16 1622, i16 9645, i16 6382, i16 1210, i16 2742, i16 2248, i16 6789, i16 5282, i16 5653, i16 5407, i16 29007, i16 29415, i16 25313, i16 -21396, i16 -15994, i16 21119, i16 -9745, i16 -22804, i16 -1897, i16 13898, i16 -7216, i16 20222, i16 31469, i16 -30937, i16 -676, i16 -4865, i16 4232, i16 -9793, i16 -11737, i16 -21717, i16 14011, i16 12369, i16 -8916, i16 13717, i16 12500, i16 -6672, i16 -31251, i16 -8199, i16 20956, i16 4977, i16 -16240, i16 19215, i16 -18975, i16 -1326, i16 -20663, i16 -29785, i16 15886, i16 14343, i16 966, i16 3529, i16 6132, i16 -8396, i16 -5346, i16 10303, i16 -22494, i16 2064, i16 22282, i16 -3981, i16 25824, i16 31442, i16 -8521, i16 -14400, i16 -24621, i16 30984, i16 -7274, i16 13983, i16 -23474, i16 11128, i16 -18559, i16 4030, i16 -29438, i16 22884, i16 16603, i16 -5437, i16 23344, i16 23968, i16 6079, i16 19797, i16 19404, i16 -30128, i16 12579, i16 13888, i16 -25241, i16 -25296, i16 3729, i16 -22983, i16 24354, i16 14074, i16 -15135, i16 -11424, i16 -28936, i16 -17901, i16 7766, i16 20953, i16 -24581, i16 -18991, i16 3574, i16 -29309, i16 -24581, i16 3027, i16 -14649, i16 -21970, i16 414, i16 8664, i16 -3920, i16 21636, i16 18637, i16 -26803, i16 -23932, i16 -12453, i16 -7462, i16 -3651, i16 22010, i16 -3233, i16 -2100, i16 -20960, i16 5954, i16 30529, i16 -8346, i16 -10708, i16 -8246, i16 -26229, i16 635, i16 28677, i16 29798, i16 13493, i16 14433, i16 16122, i16 6113, i16 29240, i16 22212, i16 16841, i16 -30165, i16 29695, i16 2862, i16 26519, i16 -13825, i16 -26725]
@values_s = external addrspace(3) global [256 x i16], align 16

declare hidden <4 x i32> @__zluda_ptx_impl_ldmatrix_m8n8_x4_trans_b16(ptr addrspace(3)) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @ldmatrix_trans(ptr addrspace(4) byref(i64) %"86") #1 {
  %"87" = alloca i64, align 8, addrspace(5)
  %"88" = alloca i32, align 4, addrspace(5)
  %"89" = alloca i64, align 8, addrspace(5)
  %"90" = alloca i64, align 8, addrspace(5)
  %"91" = alloca i32, align 4, addrspace(5)
  %"92" = alloca i64, align 8, addrspace(5)
  %"93" = alloca i32, align 4, addrspace(5)
  %"94" = alloca i64, align 8, addrspace(5)
  %"95" = alloca i64, align 8, addrspace(5)
  %"96" = alloca i32, align 4, addrspace(5)
  %"97" = alloca i32, align 4, addrspace(5)
  %"98" = alloca i32, align 4, addrspace(5)
  %"99" = alloca i32, align 4, addrspace(5)
  %"100" = alloca <2 x i16>, align 4, addrspace(5)
  %"101" = alloca <2 x i16>, align 4, addrspace(5)
  %"102" = alloca <2 x i16>, align 4, addrspace(5)
  %"103" = alloca <2 x i16>, align 4, addrspace(5)
  %"108" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"83"

"83":                                             ; preds = %1
  %"104" = load i64, ptr addrspace(4) %"86", align 8
  store i64 %"104", ptr addrspace(5) %"87", align 8
  %"52" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"84"

"84":                                             ; preds = %"83"
  store i32 %"52", ptr addrspace(5) %"88", align 4
  %"107" = load i32, ptr addrspace(5) %"88", align 4
  %"106" = zext i32 %"107" to i64
  store i64 %"106", ptr addrspace(5) %"89", align 8
  %"110" = load i32, ptr addrspace(5) %"88", align 4
  %2 = icmp uge i32 %"110", 32
  store i1 %2, ptr addrspace(5) %"108", align 1
  %"111" = load i1, ptr addrspace(5) %"108", align 1
  br i1 %"111", label %"12", label %"32"

"32":                                             ; preds = %"84"
  store i64 ptrtoint (ptr addrspace(1) @values_g to i64), ptr addrspace(5) %"90", align 8
  %"114" = load i64, ptr addrspace(5) %"89", align 8
  %"115" = load i64, ptr addrspace(5) %"90", align 8
  %3 = mul i64 %"114", 16
  %"113" = add i64 %3, %"115"
  store i64 %"113", ptr addrspace(5) %"90", align 8
  %"116" = load i64, ptr addrspace(5) %"90", align 8
  %"166" = inttoptr i64 %"116" to ptr addrspace(1)
  %"55" = load <2 x i64>, ptr addrspace(1) %"166", align 16
  %"167" = extractelement <2 x i64> %"55", i8 0
  %"168" = extractelement <2 x i64> %"55", i8 1
  store i64 %"167", ptr addrspace(5) %"94", align 8
  store i64 %"168", ptr addrspace(5) %"95", align 8
  store i32 ptrtoint (ptr addrspace(3) @values_s to i32), ptr addrspace(5) %"91", align 4
  %"121" = load i32, ptr addrspace(5) %"88", align 4
  %"122" = load i32, ptr addrspace(5) %"91", align 4
  %4 = mul i32 %"121", 16
  %"170" = add i32 %4, %"122"
  store i32 %"170", ptr addrspace(5) %"91", align 4
  %"123" = load i64, ptr addrspace(5) %"94", align 8
  %"124" = load i64, ptr addrspace(5) %"95", align 8
  %5 = insertelement <2 x i64> undef, i64 %"123", i8 0
  %"57" = insertelement <2 x i64> %5, i64 %"124", i8 1
  %"125" = load i32, ptr addrspace(5) %"91", align 4
  %"174" = inttoptr i32 %"125" to ptr addrspace(3)
  store <2 x i64> %"57", ptr addrspace(3) %"174", align 16
  store i32 ptrtoint (ptr addrspace(3) @values_s to i32), ptr addrspace(5) %"91", align 4
  %"128" = load i32, ptr addrspace(5) %"88", align 4
  %"129" = load i32, ptr addrspace(5) %"91", align 4
  %6 = mul i32 %"128", 16
  %"176" = add i32 %6, %"129"
  store i32 %"176", ptr addrspace(5) %"91", align 4
  %"130" = load i32, ptr addrspace(5) %"91", align 4
  %"178" = inttoptr i32 %"130" to ptr addrspace(3)
  %"59" = call <4 x i32> @__zluda_ptx_impl_ldmatrix_m8n8_x4_trans_b16(ptr addrspace(3) %"178")
  %"131" = extractelement <4 x i32> %"59", i8 0
  %"132" = extractelement <4 x i32> %"59", i8 1
  %"133" = extractelement <4 x i32> %"59", i8 2
  %"134" = extractelement <4 x i32> %"59", i8 3
  store i32 %"131", ptr addrspace(5) %"96", align 4
  store i32 %"132", ptr addrspace(5) %"97", align 4
  store i32 %"133", ptr addrspace(5) %"98", align 4
  store i32 %"134", ptr addrspace(5) %"99", align 4
  %"136" = load i64, ptr addrspace(5) %"89", align 8
  %"137" = load i64, ptr addrspace(5) %"87", align 8
  %7 = mul i64 %"136", 32
  %"135" = add i64 %7, %"137"
  store i64 %"135", ptr addrspace(5) %"87", align 8
  %"139" = load i32, ptr addrspace(5) %"96", align 4
  %"138" = bitcast i32 %"139" to <2 x i16>
  store <2 x i16> %"138", ptr addrspace(5) %"100", align 4
  %"140" = load <2 x i16>, ptr addrspace(5) %"100", align 4
  %"61" = extractelement <2 x i16> %"140", i8 0
  %"141" = load i64, ptr addrspace(5) %"87", align 8
  %"180" = inttoptr i64 %"141" to ptr
  store i16 %"61", ptr %"180", align 2
  %"142" = load i64, ptr addrspace(5) %"87", align 8
  %"181" = inttoptr i64 %"142" to ptr
  %"63" = getelementptr inbounds i8, ptr %"181", i64 4
  %"143" = load <2 x i16>, ptr addrspace(5) %"100", align 4
  %"64" = extractelement <2 x i16> %"143", i8 1
  store i16 %"64", ptr %"63", align 2
  %"145" = load i32, ptr addrspace(5) %"97", align 4
  %"144" = bitcast i32 %"145" to <2 x i16>
  store <2 x i16> %"144", ptr addrspace(5) %"101", align 4
  %"146" = load i64, ptr addrspace(5) %"87", align 8
  %"183" = inttoptr i64 %"146" to ptr
  %"66" = getelementptr inbounds i8, ptr %"183", i64 8
  %"147" = load <2 x i16>, ptr addrspace(5) %"101", align 4
  %"67" = extractelement <2 x i16> %"147", i8 0
  store i16 %"67", ptr %"66", align 2
  %"148" = load i64, ptr addrspace(5) %"87", align 8
  %"184" = inttoptr i64 %"148" to ptr
  %"69" = getelementptr inbounds i8, ptr %"184", i64 12
  %"149" = load <2 x i16>, ptr addrspace(5) %"101", align 4
  %"70" = extractelement <2 x i16> %"149", i8 1
  store i16 %"70", ptr %"69", align 2
  %"151" = load i32, ptr addrspace(5) %"98", align 4
  %"150" = bitcast i32 %"151" to <2 x i16>
  store <2 x i16> %"150", ptr addrspace(5) %"102", align 4
  %"152" = load i64, ptr addrspace(5) %"87", align 8
  %"186" = inttoptr i64 %"152" to ptr
  %"72" = getelementptr inbounds i8, ptr %"186", i64 16
  %"153" = load <2 x i16>, ptr addrspace(5) %"102", align 4
  %"73" = extractelement <2 x i16> %"153", i8 0
  store i16 %"73", ptr %"72", align 2
  %"154" = load i64, ptr addrspace(5) %"87", align 8
  %"187" = inttoptr i64 %"154" to ptr
  %"75" = getelementptr inbounds i8, ptr %"187", i64 20
  %"155" = load <2 x i16>, ptr addrspace(5) %"102", align 4
  %"76" = extractelement <2 x i16> %"155", i8 1
  store i16 %"76", ptr %"75", align 2
  %"157" = load i32, ptr addrspace(5) %"99", align 4
  %"156" = bitcast i32 %"157" to <2 x i16>
  store <2 x i16> %"156", ptr addrspace(5) %"103", align 4
  %"158" = load i64, ptr addrspace(5) %"87", align 8
  %"189" = inttoptr i64 %"158" to ptr
  %"78" = getelementptr inbounds i8, ptr %"189", i64 24
  %"159" = load <2 x i16>, ptr addrspace(5) %"103", align 4
  %"79" = extractelement <2 x i16> %"159", i8 0
  store i16 %"79", ptr %"78", align 2
  %"160" = load i64, ptr addrspace(5) %"87", align 8
  %"190" = inttoptr i64 %"160" to ptr
  %"81" = getelementptr inbounds i8, ptr %"190", i64 28
  %"161" = load <2 x i16>, ptr addrspace(5) %"103", align 4
  %"82" = extractelement <2 x i16> %"161", i8 1
  store i16 %"82", ptr %"81", align 2
  br label %"12"

"12":                                             ; preds = %"32", %"84"
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
