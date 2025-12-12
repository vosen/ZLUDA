@shared_mem = external addrspace(3) global [0 x i32]

define amdgpu_kernel void @extern_shared(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"37", align 8
  store i64 %2, ptr addrspace(5) %"39", align 8
  %3 = load i64, ptr addrspace(4) %"38", align 8
  store i64 %3, ptr addrspace(5) %"40", align 8
  %4 = load i64, ptr addrspace(5) %"39", align 8
  %"50" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i64, ptr addrspace(1) %"50", align 8
  store i64 %5, ptr addrspace(5) %"41", align 8
  %6 = load i64, ptr addrspace(5) %"41", align 8
  store i64 %6, ptr addrspace(3) @shared_mem, align 8
  %7 = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %7, ptr addrspace(5) %"41", align 8
  %8 = load i64, ptr addrspace(5) %"40", align 8
  %9 = load i64, ptr addrspace(5) %"41", align 8
  %"53" = inttoptr i64 %8 to ptr addrspace(1)
  store i64 %9, ptr addrspace(1) %"53", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }