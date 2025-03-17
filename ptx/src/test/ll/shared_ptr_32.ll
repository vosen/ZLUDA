@shared_mem1 = external addrspace(3) global [128 x i8], align 4

define amdgpu_kernel void @shared_ptr_32(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i32, align 4, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"42" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"42", ptr addrspace(5) %"37", align 4
  %"43" = load i64, ptr addrspace(4) %"36", align 4
  store i64 %"43", ptr addrspace(5) %"38", align 4
  store i32 ptrtoint (ptr addrspace(3) @shared_mem1 to i32), ptr addrspace(5) %"39", align 4
  %"46" = load i64, ptr addrspace(5) %"37", align 4
  %"54" = inttoptr i64 %"46" to ptr addrspace(1)
  %"45" = load i64, ptr addrspace(1) %"54", align 4
  store i64 %"45", ptr addrspace(5) %"40", align 4
  %"47" = load i32, ptr addrspace(5) %"39", align 4
  %"48" = load i64, ptr addrspace(5) %"40", align 4
  %"55" = inttoptr i32 %"47" to ptr addrspace(3)
  store i64 %"48", ptr addrspace(3) %"55", align 4
  %"49" = load i32, ptr addrspace(5) %"39", align 4
  %"56" = inttoptr i32 %"49" to ptr addrspace(3)
  %"33" = getelementptr inbounds i8, ptr addrspace(3) %"56", i64 0
  %"50" = load i64, ptr addrspace(3) %"33", align 4
  store i64 %"50", ptr addrspace(5) %"41", align 4
  %"51" = load i64, ptr addrspace(5) %"38", align 4
  %"52" = load i64, ptr addrspace(5) %"41", align 4
  %"57" = inttoptr i64 %"51" to ptr addrspace(1)
  store i64 %"52", ptr addrspace(1) %"57", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }