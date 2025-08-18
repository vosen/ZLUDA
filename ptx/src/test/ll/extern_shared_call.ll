@shared_mem = external addrspace(3) global [0 x i32], align 4

define hidden void @incr_shared_2_global() #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"37" = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %"37", ptr addrspace(5) %"36", align 8
  %"39" = load i64, ptr addrspace(5) %"36", align 8
  %"38" = add i64 %"39", 2
  store i64 %"38", ptr addrspace(5) %"36", align 8
  %"40" = load i64, ptr addrspace(5) %"36", align 8
  store i64 %"40", ptr addrspace(3) @shared_mem, align 8
  ret void
}

define amdgpu_kernel void @extern_shared_call(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #1 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"46", ptr addrspace(5) %"43", align 8
  %"47" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"47", ptr addrspace(5) %"44", align 8
  %"49" = load i64, ptr addrspace(5) %"43", align 8
  %"56" = inttoptr i64 %"49" to ptr addrspace(1)
  %"48" = load i64, ptr addrspace(1) %"56", align 8
  store i64 %"48", ptr addrspace(5) %"45", align 8
  %"50" = load i64, ptr addrspace(5) %"45", align 8
  store i64 %"50", ptr addrspace(3) @shared_mem, align 8
  call void @incr_shared_2_global()
  br label %"35"

"35":                                             ; preds = %"34"
  %"51" = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %"51", ptr addrspace(5) %"45", align 8
  %"52" = load i64, ptr addrspace(5) %"44", align 8
  %"53" = load i64, ptr addrspace(5) %"45", align 8
  %"59" = inttoptr i64 %"52" to ptr addrspace(1)
  store i64 %"53", ptr addrspace(1) %"59", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }