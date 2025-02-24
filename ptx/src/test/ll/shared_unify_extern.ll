@shared_ex = external addrspace(3) global [0 x i32]
@shared_mod = external addrspace(3) global [4 x i32]

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define i64 @add() #0 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"85"

"85":                                             ; preds = %1
  %"50" = load i64, ptr addrspace(3) @shared_mod, align 4
  store i64 %"50", ptr addrspace(5) %"48", align 4
  %"51" = load i64, ptr addrspace(3) @shared_ex, align 4
  store i64 %"51", ptr addrspace(5) %"49", align 4
  %"53" = load i64, ptr addrspace(5) %"49", align 4
  %"54" = load i64, ptr addrspace(5) %"48", align 4
  %"76" = add i64 %"53", %"54"
  store i64 %"76", ptr addrspace(5) %"47", align 4
  %2 = load i64, ptr addrspace(5) %"47", align 4
  ret i64 %2
}

define i64 @set_shared_temp1(i64 %"15") #0 {
  %"55" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"86"

"86":                                             ; preds = %1
  store i64 %"15", ptr addrspace(3) @shared_ex, align 4
  %"56" = call i64 @add()
  store i64 %"56", ptr addrspace(5) %"55", align 4
  %2 = load i64, ptr addrspace(5) %"55", align 4
  ret i64 %2
}

define amdgpu_kernel void @shared_unify_extern(ptr addrspace(4) byref(i64) %"57", ptr addrspace(4) byref(i64) %"58") #1 {
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"87"

"87":                                             ; preds = %1
  %"63" = load i64, ptr addrspace(4) %"57", align 4
  store i64 %"63", ptr addrspace(5) %"59", align 4
  %"64" = load i64, ptr addrspace(4) %"58", align 4
  store i64 %"64", ptr addrspace(5) %"60", align 4
  %"66" = load i64, ptr addrspace(5) %"59", align 4
  %"79" = inttoptr i64 %"66" to ptr addrspace(1)
  %"65" = load i64, ptr addrspace(1) %"79", align 4
  store i64 %"65", ptr addrspace(5) %"61", align 4
  %"67" = load i64, ptr addrspace(5) %"59", align 4
  %"80" = inttoptr i64 %"67" to ptr addrspace(1)
  %"40" = getelementptr inbounds i8, ptr addrspace(1) %"80", i64 8
  %"68" = load i64, ptr addrspace(1) %"40", align 4
  store i64 %"68", ptr addrspace(5) %"62", align 4
  %"69" = load i64, ptr addrspace(5) %"62", align 4
  store i64 %"69", ptr addrspace(3) @shared_mod, align 4
  %"71" = load i64, ptr addrspace(5) %"61", align 4
  %"82" = call i64 @set_shared_temp1(i64 %"71")
  store i64 %"82", ptr addrspace(5) %"62", align 4
  %"72" = load i64, ptr addrspace(5) %"60", align 4
  %"73" = load i64, ptr addrspace(5) %"62", align 4
  %"84" = inttoptr i64 %"72" to ptr
  store i64 %"73", ptr %"84", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }