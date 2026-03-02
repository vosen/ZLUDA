define amdgpu_kernel void @shr(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %3, ptr addrspace(5) %"43", align 8
  %4 = load i64, ptr addrspace(5) %"42", align 8
  %"53" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"53", align 4
  store i32 %5, ptr addrspace(5) %"44", align 4
  %6 = load i32, ptr addrspace(5) %"44", align 4
  %7 = ashr i32 %6, 31
  %8 = ashr i32 %6, 1
  %"49" = select i1 false, i32 %7, i32 %8
  store i32 %"49", ptr addrspace(5) %"44", align 4
  %9 = load i64, ptr addrspace(5) %"43", align 8
  %10 = load i32, ptr addrspace(5) %"44", align 4
  %"54" = inttoptr i64 %9 to ptr
  store i32 %10, ptr %"54", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
