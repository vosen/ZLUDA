@shared_mem = external addrspace(3) global [0 x i8], align 4

define amdgpu_kernel void @shared_ptr_take_address(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"43" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"43", ptr addrspace(5) %"38", align 8
  %"44" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"44", ptr addrspace(5) %"39", align 8
  store i64 ptrtoint (ptr addrspace(3) @shared_mem to i64), ptr addrspace(5) %"40", align 8
  %"47" = load i64, ptr addrspace(5) %"38", align 8
  %"55" = inttoptr i64 %"47" to ptr addrspace(1)
  %"46" = load i64, ptr addrspace(1) %"55", align 8
  store i64 %"46", ptr addrspace(5) %"41", align 8
  %"48" = load i64, ptr addrspace(5) %"40", align 8
  %"49" = load i64, ptr addrspace(5) %"41", align 8
  %"56" = inttoptr i64 %"48" to ptr addrspace(3)
  store i64 %"49", ptr addrspace(3) %"56", align 8
  %"51" = load i64, ptr addrspace(5) %"40", align 8
  %"57" = inttoptr i64 %"51" to ptr addrspace(3)
  %"50" = load i64, ptr addrspace(3) %"57", align 8
  store i64 %"50", ptr addrspace(5) %"42", align 8
  %"52" = load i64, ptr addrspace(5) %"39", align 8
  %"53" = load i64, ptr addrspace(5) %"42", align 8
  %"58" = inttoptr i64 %"52" to ptr addrspace(1)
  store i64 %"53", ptr addrspace(1) %"58", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }