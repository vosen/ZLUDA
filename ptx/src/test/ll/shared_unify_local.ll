@shared_ex = external addrspace(3) global [0 x i32]
@shared_mod = external addrspace(3) global i64, align 4

define hidden i64 @add(i64 %"11") #0 {
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  store i64 %"11", ptr addrspace(3) @shared_mod, align 8
  %"52" = load i64, ptr addrspace(3) @shared_mod, align 8
  store i64 %"52", ptr addrspace(5) %"51", align 8
  %"111" = load i64, ptr addrspace(3) @shared_ex, align 8
  %"54" = load i64, ptr addrspace(5) %"51", align 8
  %"77" = add i64 %"111", %"54"
  store i64 %"77", ptr addrspace(5) %"50", align 8
  %2 = load i64, ptr addrspace(5) %"50", align 8
  ret i64 %2
}

define hidden i64 @set_shared_temp1(i64 %"16", i64 %"17") #0 {
  %"55" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  store i64 %"16", ptr addrspace(3) @shared_ex, align 8
  %"56" = call i64 @add(i64 %"17")
  store i64 %"56", ptr addrspace(5) %"55", align 8
  br label %"47"

"47":                                             ; preds = %"46"
  %2 = load i64, ptr addrspace(5) %"55", align 8
  ret i64 %2
}

define amdgpu_kernel void @shared_unify_local(ptr addrspace(4) byref(i64) %"57", ptr addrspace(4) byref(i64) %"58") #1 {
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i64, align 8, addrspace(5)
  %"67" = alloca i64, align 8, addrspace(5)
  store i64 8, ptr addrspace(5) %"67", align 4
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %"63" = load i64, ptr addrspace(4) %"57", align 8
  store i64 %"63", ptr addrspace(5) %"59", align 8
  %"64" = load i64, ptr addrspace(4) %"58", align 8
  store i64 %"64", ptr addrspace(5) %"60", align 8
  %"66" = load i64, ptr addrspace(5) %"59", align 8
  %"80" = inttoptr i64 %"66" to ptr addrspace(1)
  %"65" = load i64, ptr addrspace(1) %"80", align 8
  store i64 %"65", ptr addrspace(5) %"61", align 8
  %"68" = load i64, ptr addrspace(5) %"59", align 8
  %"69" = load i64, ptr addrspace(5) %"67", align 8
  %"81" = inttoptr i64 %"68" to ptr addrspace(1)
  %"44" = getelementptr inbounds i8, ptr addrspace(1) %"81", i64 %"69"
  %"70" = load i64, ptr addrspace(1) %"44", align 8
  store i64 %"70", ptr addrspace(5) %"62", align 8
  %"72" = load i64, ptr addrspace(5) %"61", align 8
  %"73" = load i64, ptr addrspace(5) %"62", align 8
  %"82" = call i64 @set_shared_temp1(i64 %"72", i64 %"73")
  store i64 %"82", ptr addrspace(5) %"62", align 8
  br label %"49"

"49":                                             ; preds = %"48"
  %"74" = load i64, ptr addrspace(5) %"60", align 8
  %"75" = load i64, ptr addrspace(5) %"62", align 8
  %"84" = inttoptr i64 %"74" to ptr
  store i64 %"75", ptr %"84", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }