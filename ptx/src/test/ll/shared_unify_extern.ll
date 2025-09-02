@shared_ex = external addrspace(3) global [0 x i32]
@shared_mod = external addrspace(3) global [4 x i32]
@0 = addrspace(4) global i64 8

define hidden i64 @add() #0 {
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %"53" = load i64, ptr addrspace(3) @shared_mod, align 8
  store i64 %"53", ptr addrspace(5) %"51", align 8
  %"54" = load i64, ptr addrspace(3) @shared_ex, align 8
  store i64 %"54", ptr addrspace(5) %"52", align 8
  %"56" = load i64, ptr addrspace(5) %"52", align 8
  %"57" = load i64, ptr addrspace(5) %"51", align 8
  %"79" = add i64 %"56", %"57"
  store i64 %"79", ptr addrspace(5) %"50", align 8
  %2 = load i64, ptr addrspace(5) %"50", align 8
  ret i64 %2
}

define hidden i64 @set_shared_temp1(i64 %"16") #0 {
  %"58" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  store i64 %"16", ptr addrspace(3) @shared_ex, align 8
  %"59" = call i64 @add()
  store i64 %"59", ptr addrspace(5) %"58", align 8
  br label %"47"

"47":                                             ; preds = %"46"
  %2 = load i64, ptr addrspace(5) %"58", align 8
  ret i64 %2
}

define amdgpu_kernel void @shared_unify_extern(ptr addrspace(4) byref(i64) %"60", ptr addrspace(4) byref(i64) %"61") #1 {
  %"62" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i64, align 8, addrspace(5)
  %"64" = alloca i64, align 8, addrspace(5)
  %"65" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %"66" = load i64, ptr addrspace(4) %"60", align 8
  store i64 %"66", ptr addrspace(5) %"62", align 8
  %"67" = load i64, ptr addrspace(4) %"61", align 8
  store i64 %"67", ptr addrspace(5) %"63", align 8
  %"69" = load i64, ptr addrspace(5) %"62", align 8
  %"82" = inttoptr i64 %"69" to ptr addrspace(1)
  %"68" = load i64, ptr addrspace(1) %"82", align 8
  store i64 %"68", ptr addrspace(5) %"64", align 8
  %"43" = load i64, ptr addrspace(4) @0, align 8
  %"70" = load i64, ptr addrspace(5) %"62", align 8
  %"83" = inttoptr i64 %"70" to ptr addrspace(1)
  %"44" = getelementptr inbounds i8, ptr addrspace(1) %"83", i64 %"43"
  %"71" = load i64, ptr addrspace(1) %"44", align 8
  store i64 %"71", ptr addrspace(5) %"65", align 8
  %"72" = load i64, ptr addrspace(5) %"65", align 8
  store i64 %"72", ptr addrspace(3) @shared_mod, align 8
  %"74" = load i64, ptr addrspace(5) %"64", align 8
  %"85" = call i64 @set_shared_temp1(i64 %"74")
  store i64 %"85", ptr addrspace(5) %"65", align 8
  br label %"49"

"49":                                             ; preds = %"48"
  %"75" = load i64, ptr addrspace(5) %"63", align 8
  %"76" = load i64, ptr addrspace(5) %"65", align 8
  %"87" = inttoptr i64 %"75" to ptr
  store i64 %"76", ptr %"87", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }