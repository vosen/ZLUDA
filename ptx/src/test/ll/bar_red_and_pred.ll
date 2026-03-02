declare hidden i1 @__zluda_ptx_impl_bar_red_and_pred(i32, i1, i1) #0

declare hidden i1 @__zluda_ptx_impl_bar_red_or_pred(i32, i1, i1) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @bar_red_and_pred(ptr addrspace(4) byref(i64) %"81", ptr addrspace(4) byref(i64) %"82") #1 {
  %"83" = alloca i64, align 8, addrspace(5)
  %"84" = alloca i64, align 8, addrspace(5)
  %"85" = alloca i32, align 4, addrspace(5)
  %"86" = alloca i32, align 4, addrspace(5)
  %"87" = alloca i1, align 1, addrspace(5)
  %"88" = alloca i1, align 1, addrspace(5)
  %"89" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"79"

"79":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"82", align 8
  store i64 %2, ptr addrspace(5) %"83", align 8
  %"53" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"53", ptr addrspace(5) %"85", align 4
  %3 = load i32, ptr addrspace(5) %"85", align 4
  %"92" = urem i32 %3, 2
  store i32 %"92", ptr addrspace(5) %"86", align 4
  %4 = load i32, ptr addrspace(5) %"86", align 4
  %5 = icmp eq i32 %4, 0
  store i1 %5, ptr addrspace(5) %"88", align 1
  store i32 0, ptr addrspace(5) %"89", align 4
  %6 = load i1, ptr addrspace(5) %"88", align 1
  %"97" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 1, i1 %6, i1 false)
  store i1 %"97", ptr addrspace(5) %"87", align 1
  %7 = load i1, ptr addrspace(5) %"87", align 1
  br i1 %7, label %"20", label %"21"

"20":                                             ; preds = %"79"
  %8 = load i32, ptr addrspace(5) %"89", align 4
  %"100" = add i32 %8, 1
  store i32 %"100", ptr addrspace(5) %"89", align 4
  br label %"21"

"21":                                             ; preds = %"20", %"79"
  %9 = load i1, ptr addrspace(5) %"88", align 1
  %"102" = call i1 @__zluda_ptx_impl_bar_red_or_pred(i32 1, i1 %9, i1 false)
  store i1 %"102", ptr addrspace(5) %"87", align 1
  %10 = load i1, ptr addrspace(5) %"87", align 1
  br i1 %10, label %"22", label %"23"

"22":                                             ; preds = %"21"
  %11 = load i32, ptr addrspace(5) %"89", align 4
  %"105" = add i32 %11, 1
  store i32 %"105", ptr addrspace(5) %"89", align 4
  br label %"23"

"23":                                             ; preds = %"22", %"21"
  store i1 true, ptr addrspace(5) %"88", align 1
  %12 = load i1, ptr addrspace(5) %"88", align 1
  %"108" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 1, i1 %12, i1 false)
  store i1 %"108", ptr addrspace(5) %"87", align 1
  %13 = load i1, ptr addrspace(5) %"87", align 1
  br i1 %13, label %"24", label %"25"

"24":                                             ; preds = %"23"
  %14 = load i32, ptr addrspace(5) %"89", align 4
  %"111" = add i32 %14, 1
  store i32 %"111", ptr addrspace(5) %"89", align 4
  br label %"25"

"25":                                             ; preds = %"24", %"23"
  store i1 false, ptr addrspace(5) %"88", align 1
  %15 = load i1, ptr addrspace(5) %"88", align 1
  %"114" = call i1 @__zluda_ptx_impl_bar_red_or_pred(i32 1, i1 %15, i1 false)
  store i1 %"114", ptr addrspace(5) %"87", align 1
  %16 = load i1, ptr addrspace(5) %"87", align 1
  br i1 %16, label %"26", label %"27"

"26":                                             ; preds = %"25"
  %17 = load i32, ptr addrspace(5) %"89", align 4
  %"117" = add i32 %17, 1
  store i32 %"117", ptr addrspace(5) %"89", align 4
  br label %"27"

"27":                                             ; preds = %"26", %"25"
  store i1 true, ptr addrspace(5) %"88", align 1
  %18 = load i1, ptr addrspace(5) %"88", align 1
  %"120" = call i1 @__zluda_ptx_impl_bar_red_and_pred(i32 1, i1 %18, i1 true)
  store i1 %"120", ptr addrspace(5) %"87", align 1
  %19 = load i1, ptr addrspace(5) %"87", align 1
  br i1 %19, label %"28", label %"29"

"28":                                             ; preds = %"27"
  %20 = load i32, ptr addrspace(5) %"89", align 4
  %"123" = add i32 %20, 1
  store i32 %"123", ptr addrspace(5) %"89", align 4
  br label %"29"

"29":                                             ; preds = %"28", %"27"
  %21 = load i32, ptr addrspace(5) %"85", align 4
  %22 = zext i32 %21 to i64
  store i64 %22, ptr addrspace(5) %"84", align 8
  %23 = load i64, ptr addrspace(5) %"84", align 8
  %"127" = mul i64 %23, 4
  store i64 %"127", ptr addrspace(5) %"84", align 8
  %24 = load i64, ptr addrspace(5) %"83", align 8
  %25 = load i64, ptr addrspace(5) %"84", align 8
  %"129" = add i64 %24, %25
  store i64 %"129", ptr addrspace(5) %"83", align 8
  %26 = load i64, ptr addrspace(5) %"83", align 8
  %27 = load i32, ptr addrspace(5) %"89", align 4
  %"134" = inttoptr i64 %26 to ptr
  store i32 %27, ptr %"134", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
