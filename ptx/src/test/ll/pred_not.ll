@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define amdgpu_kernel void @pred_not(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %"49" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"49", ptr addrspace(5) %"43", align 4
  %"50" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"50", ptr addrspace(5) %"44", align 4
  %"52" = load i64, ptr addrspace(5) %"43", align 4
  %"66" = inttoptr i64 %"52" to ptr
  %"51" = load i64, ptr %"66", align 4
  store i64 %"51", ptr addrspace(5) %"45", align 4
  %"53" = load i64, ptr addrspace(5) %"43", align 4
  %"67" = inttoptr i64 %"53" to ptr
  %"37" = getelementptr inbounds i8, ptr %"67", i64 8
  %"54" = load i64, ptr %"37", align 4
  store i64 %"54", ptr addrspace(5) %"46", align 4
  %"56" = load i64, ptr addrspace(5) %"45", align 4
  %"57" = load i64, ptr addrspace(5) %"46", align 4
  %"55" = icmp ult i64 %"56", %"57"
  store i1 %"55", ptr addrspace(5) %"48", align 1
  %"59" = load i1, ptr addrspace(5) %"48", align 1
  %"58" = xor i1 %"59", true
  store i1 %"58", ptr addrspace(5) %"48", align 1
  %"60" = load i1, ptr addrspace(5) %"48", align 1
  br i1 %"60", label %"16", label %"17"

"16":                                             ; preds = %"40"
  store i64 1, ptr addrspace(5) %"47", align 4
  br label %"17"

"17":                                             ; preds = %"16", %"40"
  %"62" = load i1, ptr addrspace(5) %"48", align 1
  br i1 %"62", label %"19", label %"18"

"18":                                             ; preds = %"17"
  store i64 2, ptr addrspace(5) %"47", align 4
  br label %"19"

"19":                                             ; preds = %"18", %"17"
  %"64" = load i64, ptr addrspace(5) %"44", align 4
  %"65" = load i64, ptr addrspace(5) %"47", align 4
  %"68" = inttoptr i64 %"64" to ptr
  store i64 %"65", ptr %"68", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }