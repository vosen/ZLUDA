@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define <2 x i32> @impl(<2 x i32> %"9") #0 {
  %"47" = alloca <2 x i32>, align 8, addrspace(5)
  %"48" = alloca <2 x i32>, align 8, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %"38" = extractelement <2 x i32> %"9", i8 0
  store i32 %"38", ptr addrspace(5) %"49", align 4
  %"39" = extractelement <2 x i32> %"9", i8 1
  store i32 %"39", ptr addrspace(5) %"50", align 4
  %"54" = load i32, ptr addrspace(5) %"49", align 4
  %"55" = load i32, ptr addrspace(5) %"50", align 4
  %"53" = add i32 %"54", %"55"
  store i32 %"53", ptr addrspace(5) %"50", align 4
  %"56" = load i32, ptr addrspace(5) %"50", align 4
  %"58" = load <2 x i32>, ptr addrspace(5) %"48", align 8
  %"57" = insertelement <2 x i32> %"58", i32 %"56", i8 0
  store <2 x i32> %"57", ptr addrspace(5) %"48", align 8
  %"59" = load i32, ptr addrspace(5) %"50", align 4
  %"61" = load <2 x i32>, ptr addrspace(5) %"48", align 8
  %"60" = insertelement <2 x i32> %"61", i32 %"59", i8 1
  store <2 x i32> %"60", ptr addrspace(5) %"48", align 8
  %"62" = load <2 x i32>, ptr addrspace(5) %"48", align 8
  %"43" = extractelement <2 x i32> %"62", i8 1
  %"64" = load <2 x i32>, ptr addrspace(5) %"48", align 8
  %"63" = insertelement <2 x i32> %"64", i32 %"43", i8 0
  store <2 x i32> %"63", ptr addrspace(5) %"48", align 8
  %"66" = load <2 x i32>, ptr addrspace(5) %"48", align 8
  store <2 x i32> %"66", ptr addrspace(5) %"47", align 8
  %2 = load <2 x i32>, ptr addrspace(5) %"47", align 8
  ret <2 x i32> %2
}

define amdgpu_kernel void @vector(ptr addrspace(4) byref(i64) %"67", ptr addrspace(4) byref(i64) %"68") #1 {
  %"69" = alloca i64, align 8, addrspace(5)
  %"70" = alloca i64, align 8, addrspace(5)
  %"71" = alloca <2 x i32>, align 8, addrspace(5)
  %"72" = alloca i32, align 4, addrspace(5)
  %"73" = alloca i32, align 4, addrspace(5)
  %"74" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %"75" = load i64, ptr addrspace(4) %"67", align 4
  store i64 %"75", ptr addrspace(5) %"69", align 4
  %"76" = load i64, ptr addrspace(4) %"68", align 4
  store i64 %"76", ptr addrspace(5) %"70", align 4
  %"78" = load i64, ptr addrspace(5) %"69", align 4
  %"85" = inttoptr i64 %"78" to ptr
  %"77" = load <2 x i32>, ptr %"85", align 8
  store <2 x i32> %"77", ptr addrspace(5) %"71", align 8
  %"80" = load <2 x i32>, ptr addrspace(5) %"71", align 8
  %"79" = call <2 x i32> @impl(<2 x i32> %"80")
  store <2 x i32> %"79", ptr addrspace(5) %"71", align 8
  br label %"46"

"46":                                             ; preds = %"45"
  %"82" = load <2 x i32>, ptr addrspace(5) %"71", align 8
  %"86" = bitcast <2 x i32> %"82" to i64
  store i64 %"86", ptr addrspace(5) %"74", align 4
  %"83" = load i64, ptr addrspace(5) %"70", align 4
  %"84" = load <2 x i32>, ptr addrspace(5) %"71", align 8
  %"87" = inttoptr i64 %"83" to ptr
  store <2 x i32> %"84", ptr %"87", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }