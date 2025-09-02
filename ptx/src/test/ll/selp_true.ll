@0 = addrspace(4) global i64 2
@1 = addrspace(4) global i1 true

define amdgpu_kernel void @selp_true(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i16, align 2, addrspace(5)
  %"44" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"45", ptr addrspace(5) %"41", align 8
  %"46" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"46", ptr addrspace(5) %"42", align 8
  %"48" = load i64, ptr addrspace(5) %"41", align 8
  %"56" = inttoptr i64 %"48" to ptr
  %"47" = load i16, ptr %"56", align 2
  store i16 %"47", ptr addrspace(5) %"43", align 2
  %"34" = load i64, ptr addrspace(4) @0, align 8
  %"49" = load i64, ptr addrspace(5) %"41", align 8
  %"57" = inttoptr i64 %"49" to ptr
  %"35" = getelementptr inbounds i8, ptr %"57", i64 %"34"
  %"50" = load i16, ptr %"35", align 2
  store i16 %"50", ptr addrspace(5) %"44", align 2
  %"37" = load i1, ptr addrspace(4) @1, align 1
  %"52" = load i16, ptr addrspace(5) %"43", align 2
  %"53" = load i16, ptr addrspace(5) %"44", align 2
  %"51" = select i1 %"37", i16 %"52", i16 %"53"
  store i16 %"51", ptr addrspace(5) %"43", align 2
  %"54" = load i64, ptr addrspace(5) %"42", align 8
  %"55" = load i16, ptr addrspace(5) %"43", align 2
  %"58" = inttoptr i64 %"54" to ptr
  store i16 %"55", ptr %"58", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }