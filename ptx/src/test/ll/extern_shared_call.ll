@shared_mem = external addrspace(3) global [0 x i32], align 4

define hidden void @incr_shared_2_global() #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %"40", ptr addrspace(5) %"39", align 8
  %"42" = load i64, ptr addrspace(5) %"39", align 8
  %"41" = add i64 %"42", 2
  store i64 %"41", ptr addrspace(5) %"39", align 8
  %"43" = load i64, ptr addrspace(5) %"39", align 8
  store i64 %"43", ptr addrspace(3) @shared_mem, align 8
  ret void
}

define amdgpu_kernel void @extern_shared_call(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #1 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"49" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"49", ptr addrspace(5) %"46", align 8
  %"50" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"50", ptr addrspace(5) %"47", align 8
  %"52" = load i64, ptr addrspace(5) %"46", align 8
  %"59" = inttoptr i64 %"52" to ptr addrspace(1)
  %"51" = load i64, ptr addrspace(1) %"59", align 8
  store i64 %"51", ptr addrspace(5) %"48", align 8
  %"53" = load i64, ptr addrspace(5) %"48", align 8
  store i64 %"53", ptr addrspace(3) @shared_mem, align 8
  call void @incr_shared_2_global()
  br label %"38"

"38":                                             ; preds = %"37"
  %"54" = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %"54", ptr addrspace(5) %"48", align 8
  %"55" = load i64, ptr addrspace(5) %"47", align 8
  %"56" = load i64, ptr addrspace(5) %"48", align 8
  %"62" = inttoptr i64 %"55" to ptr addrspace(1)
  store i64 %"56", ptr addrspace(1) %"62", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }