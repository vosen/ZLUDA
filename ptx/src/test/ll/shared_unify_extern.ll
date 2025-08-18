@shared_ex = external addrspace(3) global [0 x i32]
@shared_mod = external addrspace(3) global [4 x i32]

define hidden i64 @add() #0 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %"49" = load i64, ptr addrspace(3) @shared_mod, align 8
  store i64 %"49", ptr addrspace(5) %"47", align 8
  %"50" = load i64, ptr addrspace(3) @shared_ex, align 8
  store i64 %"50", ptr addrspace(5) %"48", align 8
  %"52" = load i64, ptr addrspace(5) %"48", align 8
  %"53" = load i64, ptr addrspace(5) %"47", align 8
  %"75" = add i64 %"52", %"53"
  store i64 %"75", ptr addrspace(5) %"46", align 8
  %2 = load i64, ptr addrspace(5) %"46", align 8
  ret i64 %2
}

define hidden i64 @set_shared_temp1(i64 %"15") #0 {
  %"54" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  store i64 %"15", ptr addrspace(3) @shared_ex, align 8
  %"55" = call i64 @add()
  store i64 %"55", ptr addrspace(5) %"54", align 8
  br label %"43"

"43":                                             ; preds = %"42"
  %2 = load i64, ptr addrspace(5) %"54", align 8
  ret i64 %2
}

define amdgpu_kernel void @shared_unify_extern(ptr addrspace(4) byref(i64) %"56", ptr addrspace(4) byref(i64) %"57") #1 {
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %"62" = load i64, ptr addrspace(4) %"56", align 8
  store i64 %"62", ptr addrspace(5) %"58", align 8
  %"63" = load i64, ptr addrspace(4) %"57", align 8
  store i64 %"63", ptr addrspace(5) %"59", align 8
  %"65" = load i64, ptr addrspace(5) %"58", align 8
  %"78" = inttoptr i64 %"65" to ptr addrspace(1)
  %"64" = load i64, ptr addrspace(1) %"78", align 8
  store i64 %"64", ptr addrspace(5) %"60", align 8
  %"66" = load i64, ptr addrspace(5) %"58", align 8
  %"79" = inttoptr i64 %"66" to ptr addrspace(1)
  %"40" = getelementptr inbounds i8, ptr addrspace(1) %"79", i64 8
  %"67" = load i64, ptr addrspace(1) %"40", align 8
  store i64 %"67", ptr addrspace(5) %"61", align 8
  %"68" = load i64, ptr addrspace(5) %"61", align 8
  store i64 %"68", ptr addrspace(3) @shared_mod, align 8
  %"70" = load i64, ptr addrspace(5) %"60", align 8
  %"81" = call i64 @set_shared_temp1(i64 %"70")
  store i64 %"81", ptr addrspace(5) %"61", align 8
  br label %"45"

"45":                                             ; preds = %"44"
  %"71" = load i64, ptr addrspace(5) %"59", align 8
  %"72" = load i64, ptr addrspace(5) %"61", align 8
  %"83" = inttoptr i64 %"71" to ptr
  store i64 %"72", ptr %"83", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }