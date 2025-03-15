declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define i64 @incr(i64 %"43") #0 {
  %"69" = alloca i64, align 8, addrspace(5)
  %"70" = alloca i64, align 8, addrspace(5)
  %"71" = alloca i64, align 8, addrspace(5)
  %"72" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  store i64 %"43", ptr addrspace(5) %"71", align 4
  %"73" = load i64, ptr addrspace(5) %"71", align 4
  store i64 %"73", ptr addrspace(5) %"72", align 4
  %"75" = load i64, ptr addrspace(5) %"72", align 4
  %"74" = add i64 %"75", 1
  store i64 %"74", ptr addrspace(5) %"72", align 4
  %"76" = load i64, ptr addrspace(5) %"72", align 4
  store i64 %"76", ptr addrspace(5) %"70", align 4
  %"77" = load i64, ptr addrspace(5) %"70", align 4
  store i64 %"77", ptr addrspace(5) %"69", align 4
  %2 = load i64, ptr addrspace(5) %"69", align 4
  ret i64 %2
}

define amdgpu_kernel void @call(ptr addrspace(4) byref(i64) %"54", ptr addrspace(4) byref(i64) %"55") #1 {
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i64, align 8, addrspace(5)
  %"64" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %"59" = load i64, ptr addrspace(4) %"54", align 4
  store i64 %"59", ptr addrspace(5) %"56", align 4
  %"60" = load i64, ptr addrspace(4) %"55", align 4
  store i64 %"60", ptr addrspace(5) %"57", align 4
  %"62" = load i64, ptr addrspace(5) %"56", align 4
  %"78" = inttoptr i64 %"62" to ptr addrspace(1)
  %"61" = load i64, ptr addrspace(1) %"78", align 4
  store i64 %"61", ptr addrspace(5) %"58", align 4
  %"65" = load i64, ptr addrspace(5) %"58", align 4
  store i64 %"65", ptr addrspace(5) %"63", align 4
  %"40" = load i64, ptr addrspace(5) %"63", align 4
  %"41" = call i64 @incr(i64 %"40")
  br label %"45"

"45":                                             ; preds = %"44"
  store i64 %"41", ptr addrspace(5) %"64", align 4
  %"66" = load i64, ptr addrspace(5) %"64", align 4
  store i64 %"66", ptr addrspace(5) %"58", align 4
  %"67" = load i64, ptr addrspace(5) %"57", align 4
  %"68" = load i64, ptr addrspace(5) %"58", align 4
  %"81" = inttoptr i64 %"67" to ptr addrspace(1)
  store i64 %"68", ptr addrspace(1) %"81", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }