@constparams = addrspace(4) global [4 x i16] [i16 10, i16 20, i16 30, i16 40], align 8

define amdgpu_kernel void @const(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #0 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i16, align 2, addrspace(5)
  %"54" = alloca i16, align 2, addrspace(5)
  %"55" = alloca i16, align 2, addrspace(5)
  %"56" = alloca i16, align 2, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  store i64 2, ptr addrspace(5) %"60", align 4
  %"63" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"63", align 4
  %"66" = alloca i64, align 8, addrspace(5)
  store i64 6, ptr addrspace(5) %"66", align 4
  %"71" = alloca i64, align 8, addrspace(5)
  store i64 2, ptr addrspace(5) %"71", align 4
  %"75" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"75", align 4
  %"79" = alloca i64, align 8, addrspace(5)
  store i64 6, ptr addrspace(5) %"79", align 4
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
  %"61" = load i64, ptr addrspace(5) %"60", align 8
  %"37" = getelementptr inbounds i8, ptr addrspace(4) @constparams, i64 %"61"
  %"62" = load i16, ptr addrspace(4) %"37", align 2
  store i16 %"62", ptr addrspace(5) %"54", align 2
  %"64" = load i64, ptr addrspace(5) %"63", align 8
  %"39" = getelementptr inbounds i8, ptr addrspace(4) @constparams, i64 %"64"
  %"65" = load i16, ptr addrspace(4) %"39", align 2
  store i16 %"65", ptr addrspace(5) %"55", align 2
  %"67" = load i64, ptr addrspace(5) %"66", align 8
  %"41" = getelementptr inbounds i8, ptr addrspace(4) @constparams, i64 %"67"
  %"68" = load i16, ptr addrspace(4) %"41", align 2
  store i16 %"68", ptr addrspace(5) %"56", align 2
  %"69" = load i64, ptr addrspace(5) %"52", align 8
  %"70" = load i16, ptr addrspace(5) %"53", align 2
  %"87" = inttoptr i64 %"69" to ptr
  store i16 %"70", ptr %"87", align 2
  %"72" = load i64, ptr addrspace(5) %"52", align 8
  %"73" = load i64, ptr addrspace(5) %"71", align 8
  %"89" = inttoptr i64 %"72" to ptr
  %"43" = getelementptr inbounds i8, ptr %"89", i64 %"73"
  %"74" = load i16, ptr addrspace(5) %"54", align 2
  store i16 %"74", ptr %"43", align 2
  %"76" = load i64, ptr addrspace(5) %"52", align 8
  %"77" = load i64, ptr addrspace(5) %"75", align 8
  %"91" = inttoptr i64 %"76" to ptr
  %"45" = getelementptr inbounds i8, ptr %"91", i64 %"77"
  %"78" = load i16, ptr addrspace(5) %"55", align 2
  store i16 %"78", ptr %"45", align 2
  %"80" = load i64, ptr addrspace(5) %"52", align 8
  %"81" = load i64, ptr addrspace(5) %"79", align 8
  %"93" = inttoptr i64 %"80" to ptr
  %"47" = getelementptr inbounds i8, ptr %"93", i64 %"81"
  %"82" = load i16, ptr addrspace(5) %"56", align 2
  store i16 %"82", ptr %"47", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }