@constparams = addrspace(4) global [4 x i16] [i16 10, i16 20, i16 30, i16 40], align 8

define amdgpu_kernel void @const(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #0 {
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i16, align 2, addrspace(5)
  %"51" = alloca i16, align 2, addrspace(5)
  %"52" = alloca i16, align 2, addrspace(5)
  %"53" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %"54" = load i64, ptr addrspace(4) %"46", align 8
  store i64 %"54", ptr addrspace(5) %"48", align 8
  %"55" = load i64, ptr addrspace(4) %"47", align 8
  store i64 %"55", ptr addrspace(5) %"49", align 8
  %"56" = load i16, ptr addrspace(4) @constparams, align 2
  store i16 %"56", ptr addrspace(5) %"50", align 2
  %"57" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 2), align 2
  store i16 %"57", ptr addrspace(5) %"51", align 2
  %"58" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 4), align 2
  store i16 %"58", ptr addrspace(5) %"52", align 2
  %"59" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 6), align 2
  store i16 %"59", ptr addrspace(5) %"53", align 2
  %"60" = load i64, ptr addrspace(5) %"49", align 8
  %"61" = load i16, ptr addrspace(5) %"50", align 2
  %"72" = inttoptr i64 %"60" to ptr
  store i16 %"61", ptr %"72", align 2
  %"62" = load i64, ptr addrspace(5) %"49", align 8
  %"74" = inttoptr i64 %"62" to ptr
  %"40" = getelementptr inbounds i8, ptr %"74", i64 2
  %"63" = load i16, ptr addrspace(5) %"51", align 2
  store i16 %"63", ptr %"40", align 2
  %"64" = load i64, ptr addrspace(5) %"49", align 8
  %"76" = inttoptr i64 %"64" to ptr
  %"42" = getelementptr inbounds i8, ptr %"76", i64 4
  %"65" = load i16, ptr addrspace(5) %"52", align 2
  store i16 %"65", ptr %"42", align 2
  %"66" = load i64, ptr addrspace(5) %"49", align 8
  %"78" = inttoptr i64 %"66" to ptr
  %"44" = getelementptr inbounds i8, ptr %"78", i64 6
  %"67" = load i16, ptr addrspace(5) %"53", align 2
  store i16 %"67", ptr %"44", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
