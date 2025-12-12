@shared_mem = external addrspace(3) global [0 x i32], align 4

define hidden void @incr_shared_2_global() #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(5) %"42", align 8
  %"44" = add i64 %3, 2
  store i64 %"44", ptr addrspace(5) %"42", align 8
  %4 = load i64, ptr addrspace(5) %"42", align 8
  store i64 %4, ptr addrspace(3) @shared_mem, align 8
  ret void
}

define amdgpu_kernel void @extern_shared_call(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #1 {
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"47", align 8
  store i64 %2, ptr addrspace(5) %"49", align 8
  %3 = load i64, ptr addrspace(4) %"48", align 8
  store i64 %3, ptr addrspace(5) %"50", align 8
  %4 = load i64, ptr addrspace(5) %"49", align 8
  %"62" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i64, ptr addrspace(1) %"62", align 8
  store i64 %5, ptr addrspace(5) %"51", align 8
  %6 = load i64, ptr addrspace(5) %"51", align 8
  store i64 %6, ptr addrspace(3) @shared_mem, align 8
  call void @incr_shared_2_global()
  br label %"41"

"41":                                             ; preds = %"40"
  %7 = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %7, ptr addrspace(5) %"51", align 8
  %8 = load i64, ptr addrspace(5) %"50", align 8
  %9 = load i64, ptr addrspace(5) %"51", align 8
  %"65" = inttoptr i64 %8 to ptr addrspace(1)
  store i64 %9, ptr addrspace(1) %"65", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }