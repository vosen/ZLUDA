@shared_ex = external addrspace(3) global [0 x i32]
@shared_mod = external addrspace(3) global i64, align 4
@0 = addrspace(4) global i64 8

define hidden i64 @add(i64 %"11") #0 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  store i64 %"11", ptr addrspace(3) @shared_mod, align 8
  %"53" = load i64, ptr addrspace(3) @shared_mod, align 8
  store i64 %"53", ptr addrspace(5) %"52", align 8
  %"111" = load i64, ptr addrspace(3) @shared_ex, align 8
  %"55" = load i64, ptr addrspace(5) %"52", align 8
  %"76" = add i64 %"111", %"55"
  store i64 %"76", ptr addrspace(5) %"51", align 8
  %2 = load i64, ptr addrspace(5) %"51", align 8
  ret i64 %2
}

define hidden i64 @set_shared_temp1(i64 %"16", i64 %"17") #0 {
  %"56" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  store i64 %"16", ptr addrspace(3) @shared_ex, align 8
  %"57" = call i64 @add(i64 %"17")
  store i64 %"57", ptr addrspace(5) %"56", align 8
  br label %"48"

"48":                                             ; preds = %"47"
  %2 = load i64, ptr addrspace(5) %"56", align 8
  ret i64 %2
}

define amdgpu_kernel void @shared_unify_local(ptr addrspace(4) byref(i64) %"58", ptr addrspace(4) byref(i64) %"59") #1 {
  %"60" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"49"

"49":                                             ; preds = %1
  %"64" = load i64, ptr addrspace(4) %"58", align 8
  store i64 %"64", ptr addrspace(5) %"60", align 8
  %"65" = load i64, ptr addrspace(4) %"59", align 8
  store i64 %"65", ptr addrspace(5) %"61", align 8
  %"67" = load i64, ptr addrspace(5) %"60", align 8
  %"79" = inttoptr i64 %"67" to ptr addrspace(1)
  %"66" = load i64, ptr addrspace(1) %"79", align 8
  store i64 %"66", ptr addrspace(5) %"62", align 8
  %"44" = load i64, ptr addrspace(4) @0, align 8
  %"68" = load i64, ptr addrspace(5) %"60", align 8
  %"80" = inttoptr i64 %"68" to ptr addrspace(1)
  %"45" = getelementptr inbounds i8, ptr addrspace(1) %"80", i64 %"44"
  %"69" = load i64, ptr addrspace(1) %"45", align 8
  store i64 %"69", ptr addrspace(5) %"63", align 8
  %"71" = load i64, ptr addrspace(5) %"62", align 8
  %"72" = load i64, ptr addrspace(5) %"63", align 8
  %"81" = call i64 @set_shared_temp1(i64 %"71", i64 %"72")
  store i64 %"81", ptr addrspace(5) %"63", align 8
  br label %"50"

"50":                                             ; preds = %"49"
  %"73" = load i64, ptr addrspace(5) %"61", align 8
  %"74" = load i64, ptr addrspace(5) %"63", align 8
  %"83" = inttoptr i64 %"73" to ptr
  store i64 %"74", ptr %"83", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }