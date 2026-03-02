declare hidden i32 @__zluda_ptx_impl_cvt_rn_f16x2_e5m2x2(i16) #0

define amdgpu_kernel void @cvt_rn_f16x2_e5m2x2(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #1 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i16, align 2, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %3, ptr addrspace(5) %"43", align 8
  %4 = load i64, ptr addrspace(5) %"42", align 8
  %"54" = inttoptr i64 %4 to ptr
  %5 = load i16, ptr %"54", align 2
  store i16 %5, ptr addrspace(5) %"44", align 2
  %6 = load i16, ptr addrspace(5) %"44", align 2
  %"58" = call i32 @__zluda_ptx_impl_cvt_rn_f16x2_e5m2x2(i16 %6)
  %7 = bitcast i32 %"58" to <2 x half>
  %"50" = bitcast <2 x half> %7 to i32
  store i32 %"50", ptr addrspace(5) %"45", align 4
  %8 = load i64, ptr addrspace(5) %"43", align 8
  %9 = load i32, ptr addrspace(5) %"45", align 4
  %"57" = inttoptr i64 %8 to ptr
  store i32 %9, ptr %"57", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
