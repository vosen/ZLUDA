define hidden <2 x i32> @impl(<2 x i32> %"10") #0 {
  %"50" = alloca <2 x i32>, align 8, addrspace(5)
  %"51" = alloca <2 x i32>, align 8, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %"41" = extractelement <2 x i32> %"10", i8 0
  store i32 %"41", ptr addrspace(5) %"52", align 4
  %"42" = extractelement <2 x i32> %"10", i8 1
  store i32 %"42", ptr addrspace(5) %"53", align 4
  %"57" = load i32, ptr addrspace(5) %"52", align 4
  %"58" = load i32, ptr addrspace(5) %"53", align 4
  %"56" = add i32 %"57", %"58"
  store i32 %"56", ptr addrspace(5) %"53", align 4
  %"59" = load i32, ptr addrspace(5) %"53", align 4
  %"61" = load <2 x i32>, ptr addrspace(5) %"51", align 8
  %"60" = insertelement <2 x i32> %"61", i32 %"59", i8 0
  store <2 x i32> %"60", ptr addrspace(5) %"51", align 8
  %"62" = load i32, ptr addrspace(5) %"53", align 4
  %"64" = load <2 x i32>, ptr addrspace(5) %"51", align 8
  %"63" = insertelement <2 x i32> %"64", i32 %"62", i8 1
  store <2 x i32> %"63", ptr addrspace(5) %"51", align 8
  %"65" = load <2 x i32>, ptr addrspace(5) %"51", align 8
  %"46" = extractelement <2 x i32> %"65", i8 1
  %"67" = load <2 x i32>, ptr addrspace(5) %"51", align 8
  %"66" = insertelement <2 x i32> %"67", i32 %"46", i8 0
  store <2 x i32> %"66", ptr addrspace(5) %"51", align 8
  %"69" = load <2 x i32>, ptr addrspace(5) %"51", align 8
  store <2 x i32> %"69", ptr addrspace(5) %"50", align 8
  %2 = load <2 x i32>, ptr addrspace(5) %"50", align 8
  ret <2 x i32> %2
}

define amdgpu_kernel void @vector(ptr addrspace(4) byref(i64) %"70", ptr addrspace(4) byref(i64) %"71") #1 {
  %"72" = alloca i64, align 8, addrspace(5)
  %"73" = alloca i64, align 8, addrspace(5)
  %"74" = alloca <2 x i32>, align 8, addrspace(5)
  %"75" = alloca i32, align 4, addrspace(5)
  %"76" = alloca i32, align 4, addrspace(5)
  %"77" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %"78" = load i64, ptr addrspace(4) %"70", align 8
  store i64 %"78", ptr addrspace(5) %"72", align 8
  %"79" = load i64, ptr addrspace(4) %"71", align 8
  store i64 %"79", ptr addrspace(5) %"73", align 8
  %"81" = load i64, ptr addrspace(5) %"72", align 8
  %"88" = inttoptr i64 %"81" to ptr
  %"80" = load <2 x i32>, ptr %"88", align 8
  store <2 x i32> %"80", ptr addrspace(5) %"74", align 8
  %"83" = load <2 x i32>, ptr addrspace(5) %"74", align 8
  %"82" = call <2 x i32> @impl(<2 x i32> %"83")
  store <2 x i32> %"82", ptr addrspace(5) %"74", align 8
  br label %"49"

"49":                                             ; preds = %"48"
  %"85" = load <2 x i32>, ptr addrspace(5) %"74", align 8
  %"89" = bitcast <2 x i32> %"85" to i64
  store i64 %"89", ptr addrspace(5) %"77", align 8
  %"86" = load i64, ptr addrspace(5) %"73", align 8
  %"87" = load <2 x i32>, ptr addrspace(5) %"74", align 8
  %"90" = inttoptr i64 %"86" to ptr
  store <2 x i32> %"87", ptr %"90", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }