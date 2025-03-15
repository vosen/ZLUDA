@shared_ex = external addrspace(3) global [0 x i32]
@shared_mod = external addrspace(3) global [4 x i32]

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define i64 @add() #0 {
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %"55" = load i64, ptr addrspace(3) @shared_mod, align 4
  store i64 %"55", ptr addrspace(5) %"53", align 4
  %"56" = load i64, ptr addrspace(3) @shared_ex, align 4
  store i64 %"56", ptr addrspace(5) %"54", align 4
  %"58" = load i64, ptr addrspace(5) %"54", align 4
  %"59" = load i64, ptr addrspace(5) %"53", align 4
  %"81" = add i64 %"58", %"59"
  store i64 %"81", ptr addrspace(5) %"52", align 4
  %2 = load i64, ptr addrspace(5) %"52", align 4
  ret i64 %2
}

define i64 @set_shared_temp1(i64 %"15") #0 {
  %"60" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  store i64 %"15", ptr addrspace(3) @shared_ex, align 4
  %"61" = call i64 @add()
  store i64 %"61", ptr addrspace(5) %"60", align 4
  br label %"43"

"43":                                             ; preds = %"42"
  %2 = load i64, ptr addrspace(5) %"60", align 4
  ret i64 %2
}

define amdgpu_kernel void @shared_unify_extern(ptr addrspace(4) byref(i64) %"62", ptr addrspace(4) byref(i64) %"63") #1 {
  %"64" = alloca i64, align 8, addrspace(5)
  %"65" = alloca i64, align 8, addrspace(5)
  %"66" = alloca i64, align 8, addrspace(5)
  %"67" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %"68" = load i64, ptr addrspace(4) %"62", align 4
  store i64 %"68", ptr addrspace(5) %"64", align 4
  %"69" = load i64, ptr addrspace(4) %"63", align 4
  store i64 %"69", ptr addrspace(5) %"65", align 4
  %"71" = load i64, ptr addrspace(5) %"64", align 4
  %"84" = inttoptr i64 %"71" to ptr addrspace(1)
  %"70" = load i64, ptr addrspace(1) %"84", align 4
  store i64 %"70", ptr addrspace(5) %"66", align 4
  %"72" = load i64, ptr addrspace(5) %"64", align 4
  %"85" = inttoptr i64 %"72" to ptr addrspace(1)
  %"40" = getelementptr inbounds i8, ptr addrspace(1) %"85", i64 8
  %"73" = load i64, ptr addrspace(1) %"40", align 4
  store i64 %"73", ptr addrspace(5) %"67", align 4
  %"74" = load i64, ptr addrspace(5) %"67", align 4
  store i64 %"74", ptr addrspace(3) @shared_mod, align 4
  %"76" = load i64, ptr addrspace(5) %"66", align 4
  %"87" = call i64 @set_shared_temp1(i64 %"76")
  store i64 %"87", ptr addrspace(5) %"67", align 4
  br label %"45"

"45":                                             ; preds = %"44"
  %"77" = load i64, ptr addrspace(5) %"65", align 4
  %"78" = load i64, ptr addrspace(5) %"67", align 4
  %"89" = inttoptr i64 %"77" to ptr
  store i64 %"78", ptr %"89", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }