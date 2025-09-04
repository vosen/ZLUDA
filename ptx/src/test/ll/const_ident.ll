@x = addrspace(4) global i64 1
@y = addrspace(4) global [4 x i64] [i64 4, i64 5, i64 6, i64 0]
@constparams = addrspace(4) global [4 x i64] [i64 ptrtoint (ptr addrspace(4) @x to i64), i64 ptrtoint (ptr addrspace(4) @y to i64), i64 0, i64 0]

define amdgpu_kernel void @const_ident(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #0 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %"59" = load i64, ptr addrspace(4) %"49", align 8
  store i64 %"59", ptr addrspace(5) %"51", align 8
  %"60" = load i64, ptr addrspace(4) %"50", align 8
  store i64 %"60", ptr addrspace(5) %"52", align 8
  store i64 ptrtoint (ptr addrspace(4) @x to i64), ptr addrspace(5) %"53", align 8
  store i64 ptrtoint (ptr addrspace(4) @y to i64), ptr addrspace(5) %"54", align 8
  %"63" = load i64, ptr addrspace(4) @constparams, align 8
  store i64 %"63", ptr addrspace(5) %"55", align 8
  %"64" = load i64, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 8), align 8
  store i64 %"64", ptr addrspace(5) %"56", align 8
  %"66" = load i64, ptr addrspace(5) %"53", align 8
  %"67" = load i64, ptr addrspace(5) %"55", align 8
  %"65" = xor i64 %"66", %"67"
  store i64 %"65", ptr addrspace(5) %"57", align 8
  %"69" = load i64, ptr addrspace(5) %"54", align 8
  %"70" = load i64, ptr addrspace(5) %"56", align 8
  %"68" = xor i64 %"69", %"70"
  store i64 %"68", ptr addrspace(5) %"58", align 8
  %"71" = load i64, ptr addrspace(5) %"52", align 8
  %"72" = load i64, ptr addrspace(5) %"57", align 8
  %"85" = inttoptr i64 %"71" to ptr
  store i64 %"72", ptr %"85", align 8
  %"73" = load i64, ptr addrspace(5) %"52", align 8
  %"87" = inttoptr i64 %"73" to ptr
  %"45" = getelementptr inbounds i8, ptr %"87", i64 8
  %"74" = load i64, ptr addrspace(5) %"58", align 8
  store i64 %"74", ptr %"45", align 8
  %"75" = load i64, ptr addrspace(5) %"52", align 8
  %"89" = inttoptr i64 %"75" to ptr
  %"47" = getelementptr inbounds i8, ptr %"89", i64 8
  %"76" = load i64, ptr addrspace(5) %"58", align 8
  store i64 %"76", ptr %"47", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }