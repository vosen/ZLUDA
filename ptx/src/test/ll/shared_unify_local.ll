@shared_ex = external addrspace(3) global [0 x i32]
@shared_mod = external addrspace(3) global i64, align 4
@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define i64 @add(i64 %"10") #0 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  store i64 %"10", ptr addrspace(3) @shared_mod, align 4
  %"49" = load i64, ptr addrspace(3) @shared_mod, align 4
  store i64 %"49", ptr addrspace(5) %"48", align 4
  %"101" = load i64, ptr addrspace(3) @shared_ex, align 4
  %"51" = load i64, ptr addrspace(5) %"48", align 4
  %"72" = add i64 %"101", %"51"
  store i64 %"72", ptr addrspace(5) %"47", align 4
  %2 = load i64, ptr addrspace(5) %"47", align 4
  ret i64 %2
}

define i64 @set_shared_temp1(i64 %"15", i64 %"16") #0 {
  %"52" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"43"

"43":                                             ; preds = %1
  store i64 %"15", ptr addrspace(3) @shared_ex, align 4
  %"53" = call i64 @add(i64 %"16")
  store i64 %"53", ptr addrspace(5) %"52", align 4
  br label %"44"

"44":                                             ; preds = %"43"
  %2 = load i64, ptr addrspace(5) %"52", align 4
  ret i64 %2
}

define amdgpu_kernel void @shared_unify_local(ptr addrspace(4) byref(i64) %"54", ptr addrspace(4) byref(i64) %"55") #1 {
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %"60" = load i64, ptr addrspace(4) %"54", align 4
  store i64 %"60", ptr addrspace(5) %"56", align 4
  %"61" = load i64, ptr addrspace(4) %"55", align 4
  store i64 %"61", ptr addrspace(5) %"57", align 4
  %"63" = load i64, ptr addrspace(5) %"56", align 4
  %"75" = inttoptr i64 %"63" to ptr addrspace(1)
  %"62" = load i64, ptr addrspace(1) %"75", align 4
  store i64 %"62", ptr addrspace(5) %"58", align 4
  %"64" = load i64, ptr addrspace(5) %"56", align 4
  %"76" = inttoptr i64 %"64" to ptr addrspace(1)
  %"41" = getelementptr inbounds i8, ptr addrspace(1) %"76", i64 8
  %"65" = load i64, ptr addrspace(1) %"41", align 4
  store i64 %"65", ptr addrspace(5) %"59", align 4
  %"67" = load i64, ptr addrspace(5) %"58", align 4
  %"68" = load i64, ptr addrspace(5) %"59", align 4
  %"77" = call i64 @set_shared_temp1(i64 %"67", i64 %"68")
  store i64 %"77", ptr addrspace(5) %"59", align 4
  br label %"46"

"46":                                             ; preds = %"45"
  %"69" = load i64, ptr addrspace(5) %"57", align 4
  %"70" = load i64, ptr addrspace(5) %"59", align 4
  %"79" = inttoptr i64 %"69" to ptr
  store i64 %"70", ptr %"79", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
