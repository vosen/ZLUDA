define amdgpu_kernel void @setp_num(ptr addrspace(4) byref(i64) %"89", ptr addrspace(4) byref(i64) %"90") #0 {
  %"91" = alloca i64, align 8, addrspace(5)
  %"92" = alloca i64, align 8, addrspace(5)
  %"93" = alloca float, align 4, addrspace(5)
  %"94" = alloca float, align 4, addrspace(5)
  %"95" = alloca float, align 4, addrspace(5)
  %"96" = alloca float, align 4, addrspace(5)
  %"97" = alloca float, align 4, addrspace(5)
  %"98" = alloca float, align 4, addrspace(5)
  %"99" = alloca float, align 4, addrspace(5)
  %"100" = alloca float, align 4, addrspace(5)
  %"101" = alloca i32, align 4, addrspace(5)
  %"102" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"88"

"88":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"89", align 8
  store i64 %2, ptr addrspace(5) %"91", align 8
  %3 = load i64, ptr addrspace(4) %"90", align 8
  store i64 %3, ptr addrspace(5) %"92", align 8
  %4 = load i64, ptr addrspace(5) %"91", align 8
  %"157" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"157", align 4
  store float %5, ptr addrspace(5) %"93", align 4
  %6 = load i64, ptr addrspace(5) %"91", align 8
  %"158" = inttoptr i64 %6 to ptr
  %"61" = getelementptr inbounds i8, ptr %"158", i64 4
  %7 = load float, ptr %"61", align 4
  store float %7, ptr addrspace(5) %"94", align 4
  %8 = load i64, ptr addrspace(5) %"91", align 8
  %"159" = inttoptr i64 %8 to ptr
  %"63" = getelementptr inbounds i8, ptr %"159", i64 8
  %9 = load float, ptr %"63", align 4
  store float %9, ptr addrspace(5) %"95", align 4
  %10 = load i64, ptr addrspace(5) %"91", align 8
  %"160" = inttoptr i64 %10 to ptr
  %"65" = getelementptr inbounds i8, ptr %"160", i64 12
  %11 = load float, ptr %"65", align 4
  store float %11, ptr addrspace(5) %"96", align 4
  %12 = load i64, ptr addrspace(5) %"91", align 8
  %"161" = inttoptr i64 %12 to ptr
  %"67" = getelementptr inbounds i8, ptr %"161", i64 16
  %13 = load float, ptr %"67", align 4
  store float %13, ptr addrspace(5) %"97", align 4
  %14 = load i64, ptr addrspace(5) %"91", align 8
  %"162" = inttoptr i64 %14 to ptr
  %"69" = getelementptr inbounds i8, ptr %"162", i64 20
  %15 = load float, ptr %"69", align 4
  store float %15, ptr addrspace(5) %"98", align 4
  %16 = load i64, ptr addrspace(5) %"91", align 8
  %"163" = inttoptr i64 %16 to ptr
  %"71" = getelementptr inbounds i8, ptr %"163", i64 24
  %17 = load float, ptr %"71", align 4
  store float %17, ptr addrspace(5) %"99", align 4
  %18 = load i64, ptr addrspace(5) %"91", align 8
  %"164" = inttoptr i64 %18 to ptr
  %"73" = getelementptr inbounds i8, ptr %"164", i64 28
  %19 = load float, ptr %"73", align 4
  store float %19, ptr addrspace(5) %"100", align 4
  %20 = load float, ptr addrspace(5) %"93", align 4
  %21 = load float, ptr addrspace(5) %"94", align 4
  %22 = fcmp ord float %20, %21
  store i1 %22, ptr addrspace(5) %"102", align 1
  %23 = load i1, ptr addrspace(5) %"102", align 1
  br i1 %23, label %"24", label %"25"

"24":                                             ; preds = %"88"
  store i32 2, ptr addrspace(5) %"101", align 4
  br label %"25"

"25":                                             ; preds = %"24", %"88"
  %24 = load i1, ptr addrspace(5) %"102", align 1
  br i1 %24, label %"27", label %"26"

"26":                                             ; preds = %"25"
  store i32 0, ptr addrspace(5) %"101", align 4
  br label %"27"

"27":                                             ; preds = %"26", %"25"
  %25 = load i64, ptr addrspace(5) %"92", align 8
  %26 = load i32, ptr addrspace(5) %"101", align 4
  %"165" = inttoptr i64 %25 to ptr
  store i32 %26, ptr %"165", align 4
  %27 = load float, ptr addrspace(5) %"95", align 4
  %28 = load float, ptr addrspace(5) %"96", align 4
  %29 = fcmp ord float %27, %28
  store i1 %29, ptr addrspace(5) %"102", align 1
  %30 = load i1, ptr addrspace(5) %"102", align 1
  br i1 %30, label %"28", label %"29"

"28":                                             ; preds = %"27"
  store i32 2, ptr addrspace(5) %"101", align 4
  br label %"29"

"29":                                             ; preds = %"28", %"27"
  %31 = load i1, ptr addrspace(5) %"102", align 1
  br i1 %31, label %"31", label %"30"

"30":                                             ; preds = %"29"
  store i32 0, ptr addrspace(5) %"101", align 4
  br label %"31"

"31":                                             ; preds = %"30", %"29"
  %32 = load i64, ptr addrspace(5) %"92", align 8
  %"166" = inttoptr i64 %32 to ptr
  %"79" = getelementptr inbounds i8, ptr %"166", i64 4
  %33 = load i32, ptr addrspace(5) %"101", align 4
  store i32 %33, ptr %"79", align 4
  %34 = load float, ptr addrspace(5) %"97", align 4
  %35 = load float, ptr addrspace(5) %"98", align 4
  %36 = fcmp ord float %34, %35
  store i1 %36, ptr addrspace(5) %"102", align 1
  %37 = load i1, ptr addrspace(5) %"102", align 1
  br i1 %37, label %"32", label %"33"

"32":                                             ; preds = %"31"
  store i32 2, ptr addrspace(5) %"101", align 4
  br label %"33"

"33":                                             ; preds = %"32", %"31"
  %38 = load i1, ptr addrspace(5) %"102", align 1
  br i1 %38, label %"35", label %"34"

"34":                                             ; preds = %"33"
  store i32 0, ptr addrspace(5) %"101", align 4
  br label %"35"

"35":                                             ; preds = %"34", %"33"
  %39 = load i64, ptr addrspace(5) %"92", align 8
  %"167" = inttoptr i64 %39 to ptr
  %"83" = getelementptr inbounds i8, ptr %"167", i64 8
  %40 = load i32, ptr addrspace(5) %"101", align 4
  store i32 %40, ptr %"83", align 4
  %41 = load float, ptr addrspace(5) %"99", align 4
  %42 = load float, ptr addrspace(5) %"100", align 4
  %43 = fcmp ord float %41, %42
  store i1 %43, ptr addrspace(5) %"102", align 1
  %44 = load i1, ptr addrspace(5) %"102", align 1
  br i1 %44, label %"36", label %"37"

"36":                                             ; preds = %"35"
  store i32 2, ptr addrspace(5) %"101", align 4
  br label %"37"

"37":                                             ; preds = %"36", %"35"
  %45 = load i1, ptr addrspace(5) %"102", align 1
  br i1 %45, label %"39", label %"38"

"38":                                             ; preds = %"37"
  store i32 0, ptr addrspace(5) %"101", align 4
  br label %"39"

"39":                                             ; preds = %"38", %"37"
  %46 = load i64, ptr addrspace(5) %"92", align 8
  %"168" = inttoptr i64 %46 to ptr
  %"87" = getelementptr inbounds i8, ptr %"168", i64 12
  %47 = load i32, ptr addrspace(5) %"101", align 4
  store i32 %47, ptr %"87", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }