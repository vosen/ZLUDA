@shared_ex = external addrspace(3) global [0 x i32]
@shared_mod = external addrspace(3) global i64, align 4

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define i64 @add(i64 %"10") #0 {
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"81"

"81":                                             ; preds = %1
  store i64 %"10", ptr addrspace(3) @shared_mod, align 4
  %"50" = load i64, ptr addrspace(3) @shared_mod, align 4
  store i64 %"50", ptr addrspace(5) %"49", align 4
  %"101" = load i64, ptr addrspace(3) @shared_ex, align 4
  %"52" = load i64, ptr addrspace(5) %"49", align 4
  %"73" = add i64 %"101", %"52"
  store i64 %"73", ptr addrspace(5) %"48", align 4
  %2 = load i64, ptr addrspace(5) %"48", align 4
  ret i64 %2
}

define i64 @set_shared_temp1(i64 %"15", i64 %"16") #0 {
  %"53" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"82"

"82":                                             ; preds = %1
  store i64 %"15", ptr addrspace(3) @shared_ex, align 4
  %"54" = call i64 @add(i64 %"16")
  store i64 %"54", ptr addrspace(5) %"53", align 4
  %2 = load i64, ptr addrspace(5) %"53", align 4
  ret i64 %2
}

define amdgpu_kernel void @shared_unify_local(ptr addrspace(4) byref(i64) %"55", ptr addrspace(4) byref(i64) %"56") #1 {
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"83"

"83":                                             ; preds = %1
  %"61" = load i64, ptr addrspace(4) %"55", align 4
  store i64 %"61", ptr addrspace(5) %"57", align 4
  %"62" = load i64, ptr addrspace(4) %"56", align 4
  store i64 %"62", ptr addrspace(5) %"58", align 4
  %"64" = load i64, ptr addrspace(5) %"57", align 4
  %"76" = inttoptr i64 %"64" to ptr addrspace(1)
  %"63" = load i64, ptr addrspace(1) %"76", align 4
  store i64 %"63", ptr addrspace(5) %"59", align 4
  %"65" = load i64, ptr addrspace(5) %"57", align 4
  %"77" = inttoptr i64 %"65" to ptr addrspace(1)
  %"41" = getelementptr inbounds i8, ptr addrspace(1) %"77", i64 8
  %"66" = load i64, ptr addrspace(1) %"41", align 4
  store i64 %"66", ptr addrspace(5) %"60", align 4
  %"68" = load i64, ptr addrspace(5) %"59", align 4
  %"69" = load i64, ptr addrspace(5) %"60", align 4
  %"78" = call i64 @set_shared_temp1(i64 %"68", i64 %"69")
  store i64 %"78", ptr addrspace(5) %"60", align 4
  %"70" = load i64, ptr addrspace(5) %"58", align 4
  %"71" = load i64, ptr addrspace(5) %"60", align 4
  %"80" = inttoptr i64 %"70" to ptr
  store i64 %"71", ptr %"80", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }