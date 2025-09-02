define amdgpu_kernel void @setp(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #0 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"43"

"43":                                             ; preds = %1
  %"52" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"52", ptr addrspace(5) %"46", align 8
  %"53" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"53", ptr addrspace(5) %"47", align 8
  %"55" = load i64, ptr addrspace(5) %"46", align 8
  %"67" = inttoptr i64 %"55" to ptr
  %"54" = load i64, ptr %"67", align 8
  store i64 %"54", ptr addrspace(5) %"48", align 8
  %"56" = load i64, ptr addrspace(5) %"46", align 8
  %"68" = inttoptr i64 %"56" to ptr
  %"40" = getelementptr inbounds i8, ptr %"68", i64 8
  %"57" = load i64, ptr %"40", align 8
  store i64 %"57", ptr addrspace(5) %"49", align 8
  %"59" = load i64, ptr addrspace(5) %"48", align 8
  %"60" = load i64, ptr addrspace(5) %"49", align 8
  %2 = icmp ult i64 %"59", %"60"
  store i1 %2, ptr addrspace(5) %"51", align 1
  %"61" = load i1, ptr addrspace(5) %"51", align 1
  br i1 %"61", label %"17", label %"18"

"17":                                             ; preds = %"43"
  store i64 1, ptr addrspace(5) %"50", align 8
  br label %"18"

"18":                                             ; preds = %"17", %"43"
  %"63" = load i1, ptr addrspace(5) %"51", align 1
  br i1 %"63", label %"20", label %"19"

"19":                                             ; preds = %"18"
  store i64 2, ptr addrspace(5) %"50", align 8
  br label %"20"

"20":                                             ; preds = %"19", %"18"
  %"65" = load i64, ptr addrspace(5) %"47", align 8
  %"66" = load i64, ptr addrspace(5) %"50", align 8
  %"69" = inttoptr i64 %"65" to ptr
  store i64 %"66", ptr %"69", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }