@shared_mem = external addrspace(3) global [0 x i8], align 4

define amdgpu_kernel void @shared_ptr_take_address(ptr addrspace(4) byref(i64) %"33", ptr addrspace(4) byref(i64) %"34") #0 {
  %"35" = alloca i64, align 8, addrspace(5)
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(4) %"33", align 4
  store i64 %"40", ptr addrspace(5) %"35", align 4
  %"41" = load i64, ptr addrspace(4) %"34", align 4
  store i64 %"41", ptr addrspace(5) %"36", align 4
  store i64 ptrtoint (ptr addrspace(3) @shared_mem to i64), ptr addrspace(5) %"37", align 4
  %"44" = load i64, ptr addrspace(5) %"35", align 4
  %"52" = inttoptr i64 %"44" to ptr addrspace(1)
  %"43" = load i64, ptr addrspace(1) %"52", align 4
  store i64 %"43", ptr addrspace(5) %"38", align 4
  %"45" = load i64, ptr addrspace(5) %"37", align 4
  %"46" = load i64, ptr addrspace(5) %"38", align 4
  %"53" = inttoptr i64 %"45" to ptr addrspace(3)
  store i64 %"46", ptr addrspace(3) %"53", align 4
  %"48" = load i64, ptr addrspace(5) %"37", align 4
  %"54" = inttoptr i64 %"48" to ptr addrspace(3)
  %"47" = load i64, ptr addrspace(3) %"54", align 4
  store i64 %"47", ptr addrspace(5) %"39", align 4
  %"49" = load i64, ptr addrspace(5) %"36", align 4
  %"50" = load i64, ptr addrspace(5) %"39", align 4
  %"55" = inttoptr i64 %"49" to ptr addrspace(1)
  store i64 %"50", ptr addrspace(1) %"55", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }