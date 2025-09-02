@constparams = addrspace(4) global [4 x i16] [i16 10, i16 20, i16 30, i16 40], align 8
@0 = addrspace(4) global i64 2
@1 = addrspace(4) global i64 4
@2 = addrspace(4) global i64 6
@3 = addrspace(4) global i64 2
@4 = addrspace(4) global i64 4
@5 = addrspace(4) global i64 6

define amdgpu_kernel void @const(ptr addrspace(4) byref(i64) %"55", ptr addrspace(4) byref(i64) %"56") #0 {
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i16, align 2, addrspace(5)
  %"60" = alloca i16, align 2, addrspace(5)
  %"61" = alloca i16, align 2, addrspace(5)
  %"62" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"54"

"54":                                             ; preds = %1
  %"63" = load i64, ptr addrspace(4) %"55", align 8
  store i64 %"63", ptr addrspace(5) %"57", align 8
  %"64" = load i64, ptr addrspace(4) %"56", align 8
  store i64 %"64", ptr addrspace(5) %"58", align 8
  %"65" = load i16, ptr addrspace(4) @constparams, align 2
  store i16 %"65", ptr addrspace(5) %"59", align 2
  %"37" = load i64, ptr addrspace(4) @0, align 8
  %"38" = getelementptr inbounds i8, ptr addrspace(4) @constparams, i64 %"37"
  %"66" = load i16, ptr addrspace(4) %"38", align 2
  store i16 %"66", ptr addrspace(5) %"60", align 2
  %"40" = load i64, ptr addrspace(4) @1, align 8
  %"41" = getelementptr inbounds i8, ptr addrspace(4) @constparams, i64 %"40"
  %"67" = load i16, ptr addrspace(4) %"41", align 2
  store i16 %"67", ptr addrspace(5) %"61", align 2
  %"43" = load i64, ptr addrspace(4) @2, align 8
  %"44" = getelementptr inbounds i8, ptr addrspace(4) @constparams, i64 %"43"
  %"68" = load i16, ptr addrspace(4) %"44", align 2
  store i16 %"68", ptr addrspace(5) %"62", align 2
  %"69" = load i64, ptr addrspace(5) %"58", align 8
  %"70" = load i16, ptr addrspace(5) %"59", align 2
  %"81" = inttoptr i64 %"69" to ptr
  store i16 %"70", ptr %"81", align 2
  %"46" = load i64, ptr addrspace(4) @3, align 8
  %"71" = load i64, ptr addrspace(5) %"58", align 8
  %"83" = inttoptr i64 %"71" to ptr
  %"47" = getelementptr inbounds i8, ptr %"83", i64 %"46"
  %"72" = load i16, ptr addrspace(5) %"60", align 2
  store i16 %"72", ptr %"47", align 2
  %"49" = load i64, ptr addrspace(4) @4, align 8
  %"73" = load i64, ptr addrspace(5) %"58", align 8
  %"85" = inttoptr i64 %"73" to ptr
  %"50" = getelementptr inbounds i8, ptr %"85", i64 %"49"
  %"74" = load i16, ptr addrspace(5) %"61", align 2
  store i16 %"74", ptr %"50", align 2
  %"52" = load i64, ptr addrspace(4) @5, align 8
  %"75" = load i64, ptr addrspace(5) %"58", align 8
  %"87" = inttoptr i64 %"75" to ptr
  %"53" = getelementptr inbounds i8, ptr %"87", i64 %"52"
  %"76" = load i16, ptr addrspace(5) %"62", align 2
  store i16 %"76", ptr %"53", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }