@shared_mem = external addrspace(3) global [0 x i32], align 4

define hidden void @incr_shared_2_global() #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  store i64 2, ptr addrspace(5) %"41", align 4
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %"40", ptr addrspace(5) %"39", align 8
  %"43" = load i64, ptr addrspace(5) %"39", align 8
  %"44" = load i64, ptr addrspace(5) %"41", align 8
  %"42" = add i64 %"43", %"44"
  store i64 %"42", ptr addrspace(5) %"39", align 8
  %"45" = load i64, ptr addrspace(5) %"39", align 8
  store i64 %"45", ptr addrspace(3) @shared_mem, align 8
  ret void
}

define amdgpu_kernel void @extern_shared_call(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #1 {
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"51" = load i64, ptr addrspace(4) %"46", align 8
  store i64 %"51", ptr addrspace(5) %"48", align 8
  %"52" = load i64, ptr addrspace(4) %"47", align 8
  store i64 %"52", ptr addrspace(5) %"49", align 8
  %"54" = load i64, ptr addrspace(5) %"48", align 8
  %"61" = inttoptr i64 %"54" to ptr addrspace(1)
  %"53" = load i64, ptr addrspace(1) %"61", align 8
  store i64 %"53", ptr addrspace(5) %"50", align 8
  %"55" = load i64, ptr addrspace(5) %"50", align 8
  store i64 %"55", ptr addrspace(3) @shared_mem, align 8
  call void @incr_shared_2_global()
  br label %"38"

"38":                                             ; preds = %"37"
  %"56" = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %"56", ptr addrspace(5) %"50", align 8
  %"57" = load i64, ptr addrspace(5) %"49", align 8
  %"58" = load i64, ptr addrspace(5) %"50", align 8
  %"64" = inttoptr i64 %"57" to ptr addrspace(1)
  store i64 %"58", ptr addrspace(1) %"64", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }