@shared_ex = external addrspace(3) global [0 x i32]
@shared_mod = external addrspace(3) global i64, align 4

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define i64 @add(i64 %"10") #0 {
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  store i64 %"10", ptr addrspace(3) @shared_mod, align 4
  %"55" = load i64, ptr addrspace(3) @shared_mod, align 4
  store i64 %"55", ptr addrspace(5) %"54", align 4
  %"101" = load i64, ptr addrspace(3) @shared_ex, align 4
  %"57" = load i64, ptr addrspace(5) %"54", align 4
  %"78" = add i64 %"101", %"57"
  store i64 %"78", ptr addrspace(5) %"53", align 4
  %2 = load i64, ptr addrspace(5) %"53", align 4
  ret i64 %2
}

define i64 @set_shared_temp1(i64 %"15", i64 %"16") #0 {
  %"58" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"43"

"43":                                             ; preds = %1
  store i64 %"15", ptr addrspace(3) @shared_ex, align 4
  %"59" = call i64 @add(i64 %"16")
  store i64 %"59", ptr addrspace(5) %"58", align 4
  br label %"44"

"44":                                             ; preds = %"43"
  %2 = load i64, ptr addrspace(5) %"58", align 4
  ret i64 %2
}

define amdgpu_kernel void @shared_unify_local(ptr addrspace(4) byref(i64) %"60", ptr addrspace(4) byref(i64) %"61") #1 {
  %"62" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i64, align 8, addrspace(5)
  %"64" = alloca i64, align 8, addrspace(5)
  %"65" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %"66" = load i64, ptr addrspace(4) %"60", align 4
  store i64 %"66", ptr addrspace(5) %"62", align 4
  %"67" = load i64, ptr addrspace(4) %"61", align 4
  store i64 %"67", ptr addrspace(5) %"63", align 4
  %"69" = load i64, ptr addrspace(5) %"62", align 4
  %"81" = inttoptr i64 %"69" to ptr addrspace(1)
  %"68" = load i64, ptr addrspace(1) %"81", align 4
  store i64 %"68", ptr addrspace(5) %"64", align 4
  %"70" = load i64, ptr addrspace(5) %"62", align 4
  %"82" = inttoptr i64 %"70" to ptr addrspace(1)
  %"41" = getelementptr inbounds i8, ptr addrspace(1) %"82", i64 8
  %"71" = load i64, ptr addrspace(1) %"41", align 4
  store i64 %"71", ptr addrspace(5) %"65", align 4
  %"73" = load i64, ptr addrspace(5) %"64", align 4
  %"74" = load i64, ptr addrspace(5) %"65", align 4
  %"83" = call i64 @set_shared_temp1(i64 %"73", i64 %"74")
  store i64 %"83", ptr addrspace(5) %"65", align 4
  br label %"46"

"46":                                             ; preds = %"45"
  %"75" = load i64, ptr addrspace(5) %"63", align 4
  %"76" = load i64, ptr addrspace(5) %"65", align 4
  %"85" = inttoptr i64 %"75" to ptr
  store i64 %"76", ptr %"85", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }