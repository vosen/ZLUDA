@shared_mem = external addrspace(3) global [0 x i32], align 4

define hidden void @incr_shared_2_global() #0 {
  %"45" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  %2 = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %2, ptr addrspace(5) %"45", align 8
  %3 = load i64, ptr addrspace(5) %"45", align 8
  %"47" = add i64 %3, 2
  store i64 %"47", ptr addrspace(5) %"45", align 8
  %4 = load i64, ptr addrspace(5) %"45", align 8
  store i64 %4, ptr addrspace(3) @shared_mem, align 8
  ret void
}

define amdgpu_kernel void @extern_shared_call(ptr addrspace(4) byref(i64) %"50", ptr addrspace(4) byref(i64) %"51") #1 {
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"43"

"43":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"50", align 8
  store i64 %2, ptr addrspace(5) %"52", align 8
  %3 = load i64, ptr addrspace(4) %"51", align 8
  store i64 %3, ptr addrspace(5) %"53", align 8
  %4 = load i64, ptr addrspace(5) %"52", align 8
  %"65" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i64, ptr addrspace(1) %"65", align 8
  store i64 %5, ptr addrspace(5) %"54", align 8
  %6 = load i64, ptr addrspace(5) %"54", align 8
  store i64 %6, ptr addrspace(3) @shared_mem, align 8
  call void @incr_shared_2_global()
  br label %"44"

"44":                                             ; preds = %"43"
  %7 = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %7, ptr addrspace(5) %"54", align 8
  %8 = load i64, ptr addrspace(5) %"53", align 8
  %9 = load i64, ptr addrspace(5) %"54", align 8
  %"68" = inttoptr i64 %8 to ptr addrspace(1)
  store i64 %9, ptr addrspace(1) %"68", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
