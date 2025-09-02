@shared_ex = external addrspace(3) global [0 x i32]
@shared_mod = external addrspace(3) global [4 x i32]

define hidden i64 @add() #0 {
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %"52" = load i64, ptr addrspace(3) @shared_mod, align 8
  store i64 %"52", ptr addrspace(5) %"50", align 8
  %"53" = load i64, ptr addrspace(3) @shared_ex, align 8
  store i64 %"53", ptr addrspace(5) %"51", align 8
  %"55" = load i64, ptr addrspace(5) %"51", align 8
  %"56" = load i64, ptr addrspace(5) %"50", align 8
  %"78" = add i64 %"55", %"56"
  store i64 %"78", ptr addrspace(5) %"49", align 8
  %2 = load i64, ptr addrspace(5) %"49", align 8
  ret i64 %2
}

define hidden i64 @set_shared_temp1(i64 %"16") #0 {
  %"57" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  store i64 %"16", ptr addrspace(3) @shared_ex, align 8
  %"58" = call i64 @add()
  store i64 %"58", ptr addrspace(5) %"57", align 8
  br label %"46"

"46":                                             ; preds = %"45"
  %2 = load i64, ptr addrspace(5) %"57", align 8
  ret i64 %2
}

define amdgpu_kernel void @shared_unify_extern(ptr addrspace(4) byref(i64) %"59", ptr addrspace(4) byref(i64) %"60") #1 {
  %"61" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i64, align 8, addrspace(5)
  %"64" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %"65" = load i64, ptr addrspace(4) %"59", align 8
  store i64 %"65", ptr addrspace(5) %"61", align 8
  %"66" = load i64, ptr addrspace(4) %"60", align 8
  store i64 %"66", ptr addrspace(5) %"62", align 8
  %"68" = load i64, ptr addrspace(5) %"61", align 8
  %"81" = inttoptr i64 %"68" to ptr addrspace(1)
  %"67" = load i64, ptr addrspace(1) %"81", align 8
  store i64 %"67", ptr addrspace(5) %"63", align 8
  %"69" = load i64, ptr addrspace(5) %"61", align 8
  %"82" = inttoptr i64 %"69" to ptr addrspace(1)
  %"43" = getelementptr inbounds i8, ptr addrspace(1) %"82", i64 8
  %"70" = load i64, ptr addrspace(1) %"43", align 8
  store i64 %"70", ptr addrspace(5) %"64", align 8
  %"71" = load i64, ptr addrspace(5) %"64", align 8
  store i64 %"71", ptr addrspace(3) @shared_mod, align 8
  %"73" = load i64, ptr addrspace(5) %"63", align 8
  %"84" = call i64 @set_shared_temp1(i64 %"73")
  store i64 %"84", ptr addrspace(5) %"64", align 8
  br label %"48"

"48":                                             ; preds = %"47"
  %"74" = load i64, ptr addrspace(5) %"62", align 8
  %"75" = load i64, ptr addrspace(5) %"64", align 8
  %"86" = inttoptr i64 %"74" to ptr
  store i64 %"75", ptr %"86", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }