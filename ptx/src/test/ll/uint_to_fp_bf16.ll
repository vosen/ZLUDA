
define amdgpu_kernel void @uint_to_fp_bf16(ptr addrspace(4) byref(i64) %"59", ptr addrspace(4) byref(i64) %"60") #0 {
  %"61" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i16, align 2, addrspace(5)
  %"64" = alloca i1, align 1, addrspace(5)
  %"65" = alloca i16, align 2, addrspace(5)
  %"66" = alloca i32, align 4, addrspace(5)
  %"67" = alloca float, align 4, addrspace(5)
  %"68" = alloca i32, align 4, addrspace(5)
  %"69" = alloca i32, align 4, addrspace(5)
  %"70" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"58"

"58":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"59", align 8
  store i64 %2, ptr addrspace(5) %"61", align 8
  %3 = load i64, ptr addrspace(4) %"60", align 8
  store i64 %3, ptr addrspace(5) %"62", align 8
  %4 = load i64, ptr addrspace(5) %"61", align 8
  %"97" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"97", align 4
  store i32 %5, ptr addrspace(5) %"68", align 4
  %6 = load i32, ptr addrspace(5) %"68", align 4
  %7 = icmp eq i32 %6, 1
  store i1 %7, ptr addrspace(5) %"64", align 1
  %8 = load i64, ptr addrspace(5) %"61", align 8
  %"99" = inttoptr i64 %8 to ptr
  %"51" = getelementptr inbounds i8, ptr %"99", i64 4
  %9 = load i32, ptr %"51", align 4
  store i32 %9, ptr addrspace(5) %"69", align 4
  %10 = load i32, ptr addrspace(5) %"69", align 4
  %11 = icmp eq i32 %10, 2
  store i1 %11, ptr addrspace(5) %"70", align 1
  %12 = load i1, ptr addrspace(5) %"70", align 1
  br i1 %12, label %"12", label %"26"

"26":                                             ; preds = %"58"
  %13 = load i32, ptr addrspace(5) %"69", align 4
  %14 = icmp eq i32 %13, 3
  store i1 %14, ptr addrspace(5) %"70", align 1
  %15 = load i1, ptr addrspace(5) %"70", align 1
  br i1 %15, label %"13", label %"28"

"28":                                             ; preds = %"26"
  br label %"14"

"12":                                             ; preds = %"58"
  %16 = load i1, ptr addrspace(5) %"64", align 1
  %"85" = select i1 %16, i32 1, i32 0
  store i32 %"85", ptr addrspace(5) %"66", align 4
  %17 = load i64, ptr addrspace(5) %"62", align 8
  %18 = load i32, ptr addrspace(5) %"66", align 4
  %"100" = inttoptr i64 %17 to ptr addrspace(1)
  store i32 %18, ptr addrspace(1) %"100", align 4
  br label %"14"

"13":                                             ; preds = %"26"
  %19 = load i1, ptr addrspace(5) %"64", align 1
  %"89" = select i1 %19, i32 1, i32 0
  store i32 %"89", ptr addrspace(5) %"66", align 4
  %20 = load i32, ptr addrspace(5) %"66", align 4
  %"91" = uitofp i32 %20 to float
  store float %"91", ptr addrspace(5) %"67", align 4
  %21 = load float, ptr addrspace(5) %"67", align 4
  %22 = fptrunc float %21 to bfloat
  %"93" = bitcast bfloat %22 to i16
  store i16 %"93", ptr addrspace(5) %"63", align 2
  %23 = load i64, ptr addrspace(5) %"62", align 8
  %24 = load i16, ptr addrspace(5) %"63", align 2
  %"102" = inttoptr i64 %23 to ptr addrspace(1)
  store i16 %24, ptr addrspace(1) %"102", align 2
  br label %"14"

"14":                                             ; preds = %"13", %"12", %"28"
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
