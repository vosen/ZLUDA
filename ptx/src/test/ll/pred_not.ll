@0 = addrspace(4) global i64 8
@1 = addrspace(4) global i64 1
@2 = addrspace(4) global i64 2

define amdgpu_kernel void @pred_not(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #0 {
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  %"55" = load i64, ptr addrspace(4) %"47", align 8
  store i64 %"55", ptr addrspace(5) %"49", align 8
  %"56" = load i64, ptr addrspace(4) %"48", align 8
  store i64 %"56", ptr addrspace(5) %"50", align 8
  %"58" = load i64, ptr addrspace(5) %"49", align 8
  %"72" = inttoptr i64 %"58" to ptr
  %"57" = load i64, ptr %"72", align 8
  store i64 %"57", ptr addrspace(5) %"51", align 8
  %"40" = load i64, ptr addrspace(4) @0, align 8
  %"59" = load i64, ptr addrspace(5) %"49", align 8
  %"73" = inttoptr i64 %"59" to ptr
  %"41" = getelementptr inbounds i8, ptr %"73", i64 %"40"
  %"60" = load i64, ptr %"41", align 8
  store i64 %"60", ptr addrspace(5) %"52", align 8
  %"62" = load i64, ptr addrspace(5) %"51", align 8
  %"63" = load i64, ptr addrspace(5) %"52", align 8
  %2 = icmp ult i64 %"62", %"63"
  store i1 %2, ptr addrspace(5) %"54", align 1
  %"65" = load i1, ptr addrspace(5) %"54", align 1
  %"64" = xor i1 %"65", true
  store i1 %"64", ptr addrspace(5) %"54", align 1
  %"66" = load i1, ptr addrspace(5) %"54", align 1
  br i1 %"66", label %"17", label %"18"

"17":                                             ; preds = %"46"
  %"43" = load i64, ptr addrspace(4) @1, align 8
  store i64 %"43", ptr addrspace(5) %"53", align 8
  br label %"18"

"18":                                             ; preds = %"17", %"46"
  %"68" = load i1, ptr addrspace(5) %"54", align 1
  br i1 %"68", label %"20", label %"19"

"19":                                             ; preds = %"18"
  %"45" = load i64, ptr addrspace(4) @2, align 8
  store i64 %"45", ptr addrspace(5) %"53", align 8
  br label %"20"

"20":                                             ; preds = %"19", %"18"
  %"70" = load i64, ptr addrspace(5) %"50", align 8
  %"71" = load i64, ptr addrspace(5) %"53", align 8
  %"74" = inttoptr i64 %"70" to ptr
  store i64 %"71", ptr %"74", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }