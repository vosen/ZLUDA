declare hidden <2 x i32> @__zluda_ptx_impl_shfl_sync_up_b32_pred(i32, i32, i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @shfl_sync_up_dynamic_delta_b32_pred(ptr addrspace(4) byref(i64) %"67") #1 {
  %"68" = alloca i64, align 8, addrspace(5)
  %"69" = alloca i64, align 8, addrspace(5)
  %"70" = alloca i32, align 4, addrspace(5)
  %"71" = alloca i32, align 4, addrspace(5)
  %"72" = alloca i32, align 4, addrspace(5)
  %"73" = alloca i32, align 4, addrspace(5)
  %"74" = alloca i32, align 4, addrspace(5)
  %"75" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"65"

"65":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"67", align 8
  store i64 %2, ptr addrspace(5) %"68", align 8
  store i32 32, ptr addrspace(5) %"71", align 4
  store i32 33, ptr addrspace(5) %"72", align 4
  store i32 63, ptr addrspace(5) %"73", align 4
  %"49" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"49", ptr addrspace(5) %"70", align 4
  %3 = load i32, ptr addrspace(5) %"70", align 4
  %"81" = zext i32 %3 to i64
  store i64 %"81", ptr addrspace(5) %"69", align 8
  %4 = load i64, ptr addrspace(5) %"69", align 8
  %"83" = mul i64 %4, 4
  store i64 %"83", ptr addrspace(5) %"69", align 8
  %5 = load i64, ptr addrspace(5) %"68", align 8
  %6 = load i64, ptr addrspace(5) %"69", align 8
  %"85" = add i64 %5, %6
  store i64 %"85", ptr addrspace(5) %"68", align 8
  %7 = load i32, ptr addrspace(5) %"70", align 4
  %8 = load i32, ptr addrspace(5) %"71", align 4
  %"131" = call <2 x i32> @__zluda_ptx_impl_shfl_sync_up_b32_pred(i32 %7, i32 %8, i32 0, i32 -1)
  %"119" = extractelement <2 x i32> %"131", i8 0
  %"132" = extractelement <2 x i32> %"131", i8 1
  %"89" = trunc i32 %"132" to i1
  store i32 %"119", ptr addrspace(5) %"74", align 4
  store i1 %"89", ptr addrspace(5) %"75", align 1
  %9 = load i1, ptr addrspace(5) %"75", align 1
  br i1 %9, label %"21", label %"20"

"20":                                             ; preds = %"65"
  %10 = load i32, ptr addrspace(5) %"74", align 4
  %"93" = add i32 %10, 1000
  store i32 %"93", ptr addrspace(5) %"74", align 4
  br label %"21"

"21":                                             ; preds = %"20", %"65"
  %11 = load i64, ptr addrspace(5) %"68", align 8
  %12 = load i32, ptr addrspace(5) %"74", align 4
  %"122" = inttoptr i64 %11 to ptr
  store i32 %12, ptr %"122", align 4
  %13 = load i64, ptr addrspace(5) %"68", align 8
  %"97" = add i64 %13, 256
  store i64 %"97", ptr addrspace(5) %"68", align 8
  %14 = load i32, ptr addrspace(5) %"70", align 4
  %15 = load i32, ptr addrspace(5) %"72", align 4
  %"139" = call <2 x i32> @__zluda_ptx_impl_shfl_sync_up_b32_pred(i32 %14, i32 %15, i32 0, i32 -1)
  %"123" = extractelement <2 x i32> %"139", i8 0
  %"140" = extractelement <2 x i32> %"139", i8 1
  %"100" = trunc i32 %"140" to i1
  store i32 %"123", ptr addrspace(5) %"74", align 4
  store i1 %"100", ptr addrspace(5) %"75", align 1
  %16 = load i1, ptr addrspace(5) %"75", align 1
  br i1 %16, label %"23", label %"22"

"22":                                             ; preds = %"21"
  %17 = load i32, ptr addrspace(5) %"74", align 4
  %"104" = add i32 %17, 1000
  store i32 %"104", ptr addrspace(5) %"74", align 4
  br label %"23"

"23":                                             ; preds = %"22", %"21"
  %18 = load i64, ptr addrspace(5) %"68", align 8
  %19 = load i32, ptr addrspace(5) %"74", align 4
  %"126" = inttoptr i64 %18 to ptr
  store i32 %19, ptr %"126", align 4
  %20 = load i64, ptr addrspace(5) %"68", align 8
  %"108" = add i64 %20, 256
  store i64 %"108", ptr addrspace(5) %"68", align 8
  %21 = load i32, ptr addrspace(5) %"70", align 4
  %22 = load i32, ptr addrspace(5) %"73", align 4
  %"141" = call <2 x i32> @__zluda_ptx_impl_shfl_sync_up_b32_pred(i32 %21, i32 %22, i32 0, i32 -1)
  %"127" = extractelement <2 x i32> %"141", i8 0
  %"142" = extractelement <2 x i32> %"141", i8 1
  %"111" = trunc i32 %"142" to i1
  store i32 %"127", ptr addrspace(5) %"74", align 4
  store i1 %"111", ptr addrspace(5) %"75", align 1
  %23 = load i1, ptr addrspace(5) %"75", align 1
  br i1 %23, label %"25", label %"24"

"24":                                             ; preds = %"23"
  %24 = load i32, ptr addrspace(5) %"74", align 4
  %"115" = add i32 %24, 1000
  store i32 %"115", ptr addrspace(5) %"74", align 4
  br label %"25"

"25":                                             ; preds = %"24", %"23"
  %25 = load i64, ptr addrspace(5) %"68", align 8
  %26 = load i32, ptr addrspace(5) %"74", align 4
  %"130" = inttoptr i64 %25 to ptr
  store i32 %26, ptr %"130", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
