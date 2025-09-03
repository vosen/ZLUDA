@constparams = addrspace(4) global [4 x i16] [i16 10, i16 20, i16 30, i16 40], align 8

define amdgpu_kernel void @const(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #0 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i16, align 2, addrspace(5)
  %"54" = alloca i16, align 2, addrspace(5)
  %"55" = alloca i16, align 2, addrspace(5)
  %"56" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %"57" = load i64, ptr addrspace(4) %"49", align 8
  store i64 %"57", ptr addrspace(5) %"51", align 8
  %"58" = load i64, ptr addrspace(4) %"50", align 8
  store i64 %"58", ptr addrspace(5) %"52", align 8
  %"59" = load i16, ptr addrspace(4) @constparams, align 2
  store i16 %"59", ptr addrspace(5) %"53", align 2
  %"60" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 2), align 2
  store i16 %"60", ptr addrspace(5) %"54", align 2
  %"61" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 4), align 2
  store i16 %"61", ptr addrspace(5) %"55", align 2
  %"62" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 6), align 2
  store i16 %"62", ptr addrspace(5) %"56", align 2
  %"63" = load i64, ptr addrspace(5) %"52", align 8
  %"64" = load i16, ptr addrspace(5) %"53", align 2
  %"75" = inttoptr i64 %"63" to ptr
  store i16 %"64", ptr %"75", align 2
  %"65" = load i64, ptr addrspace(5) %"52", align 8
  %"77" = inttoptr i64 %"65" to ptr
  %"43" = getelementptr inbounds i8, ptr %"77", i64 2
  %"66" = load i16, ptr addrspace(5) %"54", align 2
  store i16 %"66", ptr %"43", align 2
  %"67" = load i64, ptr addrspace(5) %"52", align 8
  %"79" = inttoptr i64 %"67" to ptr
  %"45" = getelementptr inbounds i8, ptr %"79", i64 4
  %"68" = load i16, ptr addrspace(5) %"55", align 2
  store i16 %"68", ptr %"45", align 2
  %"69" = load i64, ptr addrspace(5) %"52", align 8
  %"81" = inttoptr i64 %"69" to ptr
  %"47" = getelementptr inbounds i8, ptr %"81", i64 6
  %"70" = load i16, ptr addrspace(5) %"56", align 2
  store i16 %"70", ptr %"47", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }