@values_g = addrspace(1) global [256 x i16] [i16 1340, i16 122, i16 527, i16 693, i16 958, i16 394, i16 668, i16 432, i16 646, i16 354, i16 761, i16 449, i16 252, i16 778, i16 218, i16 800, i16 656, i16 493, i16 659, i16 787, i16 672, i16 203, i16 343, i16 845, i16 318, i16 286, i16 206, i16 253, i16 194, i16 489, i16 29, i16 323, i16 7, i16 619, i16 998, i16 930, i16 773, i16 749, i16 172, i16 465, i16 937, i16 96, i16 88, i16 621, i16 909, i16 298, i16 283, i16 286, i16 779, i16 290, i16 429, i16 930, i16 25, i16 687, i16 423, i16 200, i16 918, i16 10, i16 515, i16 248, i16 158, i16 911, i16 270, i16 459, i16 5832, i16 3864, i16 7868, i16 6538, i16 3898, i16 8685, i16 356, i16 3655, i16 3398, i16 8529, i16 2866, i16 1432, i16 4078, i16 1674, i16 498, i16 1124, i16 1576, i16 6490, i16 9895, i16 2152, i16 9668, i16 7349, i16 1948, i16 6239, i16 7944, i16 7630, i16 9699, i16 1957, i16 3360, i16 2291, i16 3832, i16 7370, i16 2683, i16 7465, i16 3107, i16 9822, i16 2510, i16 1642, i16 3240, i16 8860, i16 4935, i16 1935, i16 9328, i16 5164, i16 2759, i16 4816, i16 1049, i16 725, i16 9774, i16 5110, i16 5071, i16 8047, i16 7267, i16 7716, i16 1622, i16 9645, i16 6382, i16 1210, i16 2742, i16 2248, i16 6789, i16 5282, i16 5653, i16 5407, i16 29007, i16 29415, i16 25313, i16 -21396, i16 -15994, i16 21119, i16 -9745, i16 -22804, i16 -1897, i16 13898, i16 -7216, i16 20222, i16 31469, i16 -30937, i16 -676, i16 -4865, i16 4232, i16 -9793, i16 -11737, i16 -21717, i16 14011, i16 12369, i16 -8916, i16 13717, i16 12500, i16 -6672, i16 -31251, i16 -8199, i16 20956, i16 4977, i16 -16240, i16 19215, i16 -18975, i16 -1326, i16 -20663, i16 -29785, i16 15886, i16 14343, i16 966, i16 3529, i16 6132, i16 -8396, i16 -5346, i16 10303, i16 -22494, i16 2064, i16 22282, i16 -3981, i16 25824, i16 31442, i16 -8521, i16 -14400, i16 -24621, i16 30984, i16 -7274, i16 13983, i16 -23474, i16 11128, i16 -18559, i16 4030, i16 -29438, i16 22884, i16 16603, i16 -5437, i16 23344, i16 23968, i16 6079, i16 19797, i16 19404, i16 -30128, i16 12579, i16 13888, i16 -25241, i16 -25296, i16 3729, i16 -22983, i16 24354, i16 14074, i16 -15135, i16 -11424, i16 -28936, i16 -17901, i16 7766, i16 20953, i16 -24581, i16 -18991, i16 3574, i16 -29309, i16 -24581, i16 3027, i16 -14649, i16 -21970, i16 414, i16 8664, i16 -3920, i16 21636, i16 18637, i16 -26803, i16 -23932, i16 -12453, i16 -7462, i16 -3651, i16 22010, i16 -3233, i16 -2100, i16 -20960, i16 5954, i16 30529, i16 -8346, i16 -10708, i16 -8246, i16 -26229, i16 635, i16 28677, i16 29798, i16 13493, i16 14433, i16 16122, i16 6113, i16 29240, i16 22212, i16 16841, i16 -30165, i16 29695, i16 2862, i16 26519, i16 -13825, i16 -26725]
@values_s = external addrspace(3) global [256 x i16], align 16

declare hidden <4 x i32> @__zluda_ptx_impl_ldmatrix_m8n8_x4_trans_b16(ptr addrspace(3)) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @ldmatrix_trans(ptr addrspace(4) byref(i64) %"89") #1 {
  %"90" = alloca i64, align 8, addrspace(5)
  %"91" = alloca i32, align 4, addrspace(5)
  %"92" = alloca i64, align 8, addrspace(5)
  %"93" = alloca i64, align 8, addrspace(5)
  %"94" = alloca i32, align 4, addrspace(5)
  %"95" = alloca i64, align 8, addrspace(5)
  %"96" = alloca i32, align 4, addrspace(5)
  %"97" = alloca i64, align 8, addrspace(5)
  %"98" = alloca i64, align 8, addrspace(5)
  %"99" = alloca i32, align 4, addrspace(5)
  %"100" = alloca i32, align 4, addrspace(5)
  %"101" = alloca i32, align 4, addrspace(5)
  %"102" = alloca i32, align 4, addrspace(5)
  %"103" = alloca <2 x i16>, align 4, addrspace(5)
  %"104" = alloca <2 x i16>, align 4, addrspace(5)
  %"105" = alloca <2 x i16>, align 4, addrspace(5)
  %"106" = alloca <2 x i16>, align 4, addrspace(5)
  %"111" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"86"

"86":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"89", align 8
  store i64 %2, ptr addrspace(5) %"90", align 8
  %"55" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"87"

"87":                                             ; preds = %"86"
  store i32 %"55", ptr addrspace(5) %"91", align 4
  %3 = load i32, ptr addrspace(5) %"91", align 4
  %"109" = zext i32 %3 to i64
  store i64 %"109", ptr addrspace(5) %"92", align 8
  %4 = load i32, ptr addrspace(5) %"91", align 4
  %5 = icmp uge i32 %4, 32
  store i1 %5, ptr addrspace(5) %"111", align 1
  %6 = load i1, ptr addrspace(5) %"111", align 1
  br i1 %6, label %"13", label %"33"

"33":                                             ; preds = %"87"
  store i64 ptrtoint (ptr addrspace(1) @values_g to i64), ptr addrspace(5) %"93", align 8
  %7 = load i64, ptr addrspace(5) %"92", align 8
  %8 = load i64, ptr addrspace(5) %"93", align 8
  %9 = mul i64 %7, 16
  %"116" = add i64 %9, %8
  store i64 %"116", ptr addrspace(5) %"93", align 8
  %10 = load i64, ptr addrspace(5) %"93", align 8
  %"169" = inttoptr i64 %10 to ptr addrspace(1)
  %11 = load <2 x i64>, ptr addrspace(1) %"169", align 16
  %"170" = extractelement <2 x i64> %11, i8 0
  %"171" = extractelement <2 x i64> %11, i8 1
  store i64 %"170", ptr addrspace(5) %"97", align 8
  store i64 %"171", ptr addrspace(5) %"98", align 8
  store i32 ptrtoint (ptr addrspace(3) @values_s to i32), ptr addrspace(5) %"94", align 4
  %12 = load i32, ptr addrspace(5) %"91", align 4
  %13 = load i32, ptr addrspace(5) %"94", align 4
  %14 = mul i32 %12, 16
  %"173" = add i32 %14, %13
  store i32 %"173", ptr addrspace(5) %"94", align 4
  %15 = load i64, ptr addrspace(5) %"97", align 8
  %16 = load i64, ptr addrspace(5) %"98", align 8
  %17 = insertelement <2 x i64> undef, i64 %15, i8 0
  %"60" = insertelement <2 x i64> %17, i64 %16, i8 1
  %18 = load i32, ptr addrspace(5) %"94", align 4
  %"177" = inttoptr i32 %18 to ptr addrspace(3)
  store <2 x i64> %"60", ptr addrspace(3) %"177", align 16
  store i32 ptrtoint (ptr addrspace(3) @values_s to i32), ptr addrspace(5) %"94", align 4
  %19 = load i32, ptr addrspace(5) %"91", align 4
  %20 = load i32, ptr addrspace(5) %"94", align 4
  %21 = mul i32 %19, 16
  %"179" = add i32 %21, %20
  store i32 %"179", ptr addrspace(5) %"94", align 4
  %22 = load i32, ptr addrspace(5) %"94", align 4
  %"181" = inttoptr i32 %22 to ptr addrspace(3)
  %"62" = call <4 x i32> @__zluda_ptx_impl_ldmatrix_m8n8_x4_trans_b16(ptr addrspace(3) %"181")
  %"134" = extractelement <4 x i32> %"62", i8 0
  %"135" = extractelement <4 x i32> %"62", i8 1
  %"136" = extractelement <4 x i32> %"62", i8 2
  %"137" = extractelement <4 x i32> %"62", i8 3
  store i32 %"134", ptr addrspace(5) %"99", align 4
  store i32 %"135", ptr addrspace(5) %"100", align 4
  store i32 %"136", ptr addrspace(5) %"101", align 4
  store i32 %"137", ptr addrspace(5) %"102", align 4
  %23 = load i64, ptr addrspace(5) %"92", align 8
  %24 = load i64, ptr addrspace(5) %"90", align 8
  %25 = mul i64 %23, 32
  %"138" = add i64 %25, %24
  store i64 %"138", ptr addrspace(5) %"90", align 8
  %26 = load i32, ptr addrspace(5) %"99", align 4
  %"141" = bitcast i32 %26 to <2 x i16>
  store <2 x i16> %"141", ptr addrspace(5) %"103", align 4
  %27 = load <2 x i16>, ptr addrspace(5) %"103", align 4
  %"64" = extractelement <2 x i16> %27, i8 0
  %28 = load i64, ptr addrspace(5) %"90", align 8
  %"183" = inttoptr i64 %28 to ptr
  store i16 %"64", ptr %"183", align 2
  %29 = load i64, ptr addrspace(5) %"90", align 8
  %"184" = inttoptr i64 %29 to ptr
  %"66" = getelementptr inbounds i8, ptr %"184", i64 4
  %30 = load <2 x i16>, ptr addrspace(5) %"103", align 4
  %"67" = extractelement <2 x i16> %30, i8 1
  store i16 %"67", ptr %"66", align 2
  %31 = load i32, ptr addrspace(5) %"100", align 4
  %"147" = bitcast i32 %31 to <2 x i16>
  store <2 x i16> %"147", ptr addrspace(5) %"104", align 4
  %32 = load i64, ptr addrspace(5) %"90", align 8
  %"186" = inttoptr i64 %32 to ptr
  %"69" = getelementptr inbounds i8, ptr %"186", i64 8
  %33 = load <2 x i16>, ptr addrspace(5) %"104", align 4
  %"70" = extractelement <2 x i16> %33, i8 0
  store i16 %"70", ptr %"69", align 2
  %34 = load i64, ptr addrspace(5) %"90", align 8
  %"187" = inttoptr i64 %34 to ptr
  %"72" = getelementptr inbounds i8, ptr %"187", i64 12
  %35 = load <2 x i16>, ptr addrspace(5) %"104", align 4
  %"73" = extractelement <2 x i16> %35, i8 1
  store i16 %"73", ptr %"72", align 2
  %36 = load i32, ptr addrspace(5) %"101", align 4
  %"153" = bitcast i32 %36 to <2 x i16>
  store <2 x i16> %"153", ptr addrspace(5) %"105", align 4
  %37 = load i64, ptr addrspace(5) %"90", align 8
  %"189" = inttoptr i64 %37 to ptr
  %"75" = getelementptr inbounds i8, ptr %"189", i64 16
  %38 = load <2 x i16>, ptr addrspace(5) %"105", align 4
  %"76" = extractelement <2 x i16> %38, i8 0
  store i16 %"76", ptr %"75", align 2
  %39 = load i64, ptr addrspace(5) %"90", align 8
  %"190" = inttoptr i64 %39 to ptr
  %"78" = getelementptr inbounds i8, ptr %"190", i64 20
  %40 = load <2 x i16>, ptr addrspace(5) %"105", align 4
  %"79" = extractelement <2 x i16> %40, i8 1
  store i16 %"79", ptr %"78", align 2
  %41 = load i32, ptr addrspace(5) %"102", align 4
  %"159" = bitcast i32 %41 to <2 x i16>
  store <2 x i16> %"159", ptr addrspace(5) %"106", align 4
  %42 = load i64, ptr addrspace(5) %"90", align 8
  %"192" = inttoptr i64 %42 to ptr
  %"81" = getelementptr inbounds i8, ptr %"192", i64 24
  %43 = load <2 x i16>, ptr addrspace(5) %"106", align 4
  %"82" = extractelement <2 x i16> %43, i8 0
  store i16 %"82", ptr %"81", align 2
  %44 = load i64, ptr addrspace(5) %"90", align 8
  %"193" = inttoptr i64 %44 to ptr
  %"84" = getelementptr inbounds i8, ptr %"193", i64 28
  %45 = load <2 x i16>, ptr addrspace(5) %"106", align 4
  %"85" = extractelement <2 x i16> %45, i8 1
  store i16 %"85", ptr %"84", align 2
  br label %"13"

"13":                                             ; preds = %"33", %"87"
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }