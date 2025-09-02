@0 = addrspace(4) global i64 1
@1 = addrspace(4) global i64 0
@2 = addrspace(4) global i64 0

define amdgpu_kernel void @reg_local(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #0 {
  %"11" = alloca [8 x i8], align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  %"48" = load i64, ptr addrspace(4) %"43", align 8
  store i64 %"48", ptr addrspace(5) %"45", align 8
  %"49" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"49", ptr addrspace(5) %"46", align 8
  %"51" = load i64, ptr addrspace(5) %"45", align 8
  %"57" = inttoptr i64 %"51" to ptr addrspace(1)
  %"56" = load i64, ptr addrspace(1) %"57", align 8
  store i64 %"56", ptr addrspace(5) %"47", align 8
  %"34" = load i64, ptr addrspace(4) @0, align 8
  %"52" = load i64, ptr addrspace(5) %"47", align 8
  %"35" = add i64 %"52", %"34"
  %"58" = addrspacecast ptr addrspace(5) %"11" to ptr
  store i64 %"35", ptr %"58", align 8
  %"37" = load i64, ptr addrspace(4) @1, align 8
  %"60" = addrspacecast ptr addrspace(5) %"11" to ptr
  %"38" = getelementptr inbounds i8, ptr %"60", i64 %"37"
  %"61" = load i64, ptr %"38", align 8
  store i64 %"61", ptr addrspace(5) %"47", align 8
  %"40" = load i64, ptr addrspace(4) @2, align 8
  %"54" = load i64, ptr addrspace(5) %"46", align 8
  %"62" = inttoptr i64 %"54" to ptr addrspace(1)
  %"41" = getelementptr inbounds i8, ptr addrspace(1) %"62", i64 %"40"
  %"55" = load i64, ptr addrspace(5) %"47", align 8
  store i64 %"55", ptr addrspace(1) %"41", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }