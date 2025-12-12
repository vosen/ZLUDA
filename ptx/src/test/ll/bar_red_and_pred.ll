declare hidden i1 @__zluda_ptx_impl_bar_red_and_pred(i32, i1, i1) #0

declare hidden i1 @__zluda_ptx_impl_bar_red_or_pred(i32, i1, i1) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @bar_red_and_pred(ptr addrspace(4) byref(i64) %"78", ptr addrspace(4) byref(i64) %"79") #1 {
  %"80" = alloca i64, align 8, addrspace(5)
  %"81" = alloca i64, align 8, addrspace(5)
  %"82" = alloca i32, align 4, addrspace(5)
  %"83" = alloca i32, align 4, addrspace(5)
  %"84" = alloca i1, align 1, addrspace(5)
  %"85" = alloca i1, align 1, addrspace(5)
  %"86" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"76"

"76":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"79", align 8
  store i64 %2, ptr addrspace(5) %"80", align 8
  %"50" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"50", ptr addrspace(5) %"82", align 4
  %3 = load i32, ptr addrspace(5) %"82", align 4
  %"89" = urem i32 %3, 2
  store i32 %"89", ptr addrspace(5) %"83", align 4
  %4 = load i32, ptr addrspace(5) %"83", align 4
  %5 = icmp eq i32 %4, 0
  store i1 %5, ptr addrspace(5) %"85", align 1
  store i32 0, ptr addrspace(5) %"86", align 4
  %6 = load i1, ptr addrspace(5) %"85", align 1
  %"94" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 1, i1 %6, i1 false)
  store i1 %"94", ptr addrspace(5) %"84", align 1
  %7 = load i1, ptr addrspace(5) %"84", align 1
  br i1 %7, label %"19", label %"20"

"19":                                             ; preds = %"76"
  %8 = load i32, ptr addrspace(5) %"86", align 4
  %"97" = add i32 %8, 1
  store i32 %"97", ptr addrspace(5) %"86", align 4
  br label %"20"

"20":                                             ; preds = %"19", %"76"
  %9 = load i1, ptr addrspace(5) %"85", align 1
  %"99" = call i1 @__zluda_ptx_impl_bar_red_or_pred(i32 1, i1 %9, i1 false)
  store i1 %"99", ptr addrspace(5) %"84", align 1
  %10 = load i1, ptr addrspace(5) %"84", align 1
  br i1 %10, label %"21", label %"22"

"21":                                             ; preds = %"20"
  %11 = load i32, ptr addrspace(5) %"86", align 4
  %"102" = add i32 %11, 1
  store i32 %"102", ptr addrspace(5) %"86", align 4
  br label %"22"

"22":                                             ; preds = %"21", %"20"
  store i1 true, ptr addrspace(5) %"85", align 1
  %12 = load i1, ptr addrspace(5) %"85", align 1
  %"105" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 1, i1 %12, i1 false)
  store i1 %"105", ptr addrspace(5) %"84", align 1
  %13 = load i1, ptr addrspace(5) %"84", align 1
  br i1 %13, label %"23", label %"24"

"23":                                             ; preds = %"22"
  %14 = load i32, ptr addrspace(5) %"86", align 4
  %"108" = add i32 %14, 1
  store i32 %"108", ptr addrspace(5) %"86", align 4
  br label %"24"

"24":                                             ; preds = %"23", %"22"
  store i1 false, ptr addrspace(5) %"85", align 1
  %15 = load i1, ptr addrspace(5) %"85", align 1
  %"111" = call i1 @__zluda_ptx_impl_bar_red_or_pred(i32 1, i1 %15, i1 false)
  store i1 %"111", ptr addrspace(5) %"84", align 1
  %16 = load i1, ptr addrspace(5) %"84", align 1
  br i1 %16, label %"25", label %"26"

"25":                                             ; preds = %"24"
  %17 = load i32, ptr addrspace(5) %"86", align 4
  %"114" = add i32 %17, 1
  store i32 %"114", ptr addrspace(5) %"86", align 4
  br label %"26"

"26":                                             ; preds = %"25", %"24"
  store i1 true, ptr addrspace(5) %"85", align 1
  %18 = load i1, ptr addrspace(5) %"85", align 1
  %"117" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 1, i1 %18, i1 true)
  store i1 %"117", ptr addrspace(5) %"84", align 1
  %19 = load i1, ptr addrspace(5) %"84", align 1
  br i1 %19, label %"27", label %"28"

"27":                                             ; preds = %"26"
  %20 = load i32, ptr addrspace(5) %"86", align 4
  %"120" = add i32 %20, 1
  store i32 %"120", ptr addrspace(5) %"86", align 4
  br label %"28"

"28":                                             ; preds = %"27", %"26"
  %21 = load i32, ptr addrspace(5) %"82", align 4
  %22 = zext i32 %21 to i64
  store i64 %22, ptr addrspace(5) %"81", align 8
  %23 = load i64, ptr addrspace(5) %"81", align 8
  %"124" = mul i64 %23, 4
  store i64 %"124", ptr addrspace(5) %"81", align 8
  %24 = load i64, ptr addrspace(5) %"80", align 8
  %25 = load i64, ptr addrspace(5) %"81", align 8
  %"126" = add i64 %24, %25
  store i64 %"126", ptr addrspace(5) %"80", align 8
  %26 = load i64, ptr addrspace(5) %"80", align 8
  %27 = load i32, ptr addrspace(5) %"86", align 4
  %"131" = inttoptr i64 %26 to ptr
  store i32 %27, ptr %"131", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }