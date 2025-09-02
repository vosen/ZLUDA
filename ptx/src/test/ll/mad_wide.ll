@0 = addrspace(4) global i64 4
@1 = addrspace(4) global i64 8

define amdgpu_kernel void @mad_wide(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #0 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %"50" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"50", ptr addrspace(5) %"44", align 8
  %"51" = load i64, ptr addrspace(4) %"43", align 8
  store i64 %"51", ptr addrspace(5) %"45", align 8
  %"53" = load i64, ptr addrspace(5) %"44", align 8
  %"64" = inttoptr i64 %"53" to ptr
  %"52" = load i32, ptr %"64", align 4
  store i32 %"52", ptr addrspace(5) %"47", align 4
  %"36" = load i64, ptr addrspace(4) @0, align 8
  %"54" = load i64, ptr addrspace(5) %"44", align 8
  %"65" = inttoptr i64 %"54" to ptr
  %"37" = getelementptr inbounds i8, ptr %"65", i64 %"36"
  %"55" = load i32, ptr %"37", align 4
  store i32 %"55", ptr addrspace(5) %"48", align 4
  %"39" = load i64, ptr addrspace(4) @1, align 8
  %"56" = load i64, ptr addrspace(5) %"44", align 8
  %"66" = inttoptr i64 %"56" to ptr
  %"40" = getelementptr inbounds i8, ptr %"66", i64 %"39"
  %"57" = load i64, ptr %"40", align 8
  store i64 %"57", ptr addrspace(5) %"49", align 8
  %"59" = load i32, ptr addrspace(5) %"47", align 4
  %"60" = load i32, ptr addrspace(5) %"48", align 4
  %"61" = load i64, ptr addrspace(5) %"49", align 8
  %2 = sext i32 %"59" to i64
  %3 = sext i32 %"60" to i64
  %4 = mul i64 %2, %3
  %"58" = add i64 %4, %"61"
  store i64 %"58", ptr addrspace(5) %"46", align 8
  %"62" = load i64, ptr addrspace(5) %"45", align 8
  %"63" = load i64, ptr addrspace(5) %"46", align 8
  %"67" = inttoptr i64 %"62" to ptr
  store i64 %"63", ptr %"67", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }