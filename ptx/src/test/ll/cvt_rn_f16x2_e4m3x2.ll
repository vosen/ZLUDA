declare hidden i32 @__zluda_ptx_impl_cvt_rn_f16x2_e4m3x2(i16) #0

define amdgpu_kernel void @cvt_rn_f16x2_e4m3x2(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #1 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i16, align 2, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"37", align 8
  store i64 %2, ptr addrspace(5) %"39", align 8
  %3 = load i64, ptr addrspace(4) %"38", align 8
  store i64 %3, ptr addrspace(5) %"40", align 8
  %4 = load i64, ptr addrspace(5) %"39", align 8
  %"51" = inttoptr i64 %4 to ptr
  %5 = load i16, ptr %"51", align 2
  store i16 %5, ptr addrspace(5) %"41", align 2
  %6 = load i16, ptr addrspace(5) %"41", align 2
  %"55" = call i32 @__zluda_ptx_impl_cvt_rn_f16x2_e4m3x2(i16 %6)
  %7 = bitcast i32 %"55" to <2 x half>
  %"47" = bitcast <2 x half> %7 to i32
  store i32 %"47", ptr addrspace(5) %"42", align 4
  %8 = load i64, ptr addrspace(5) %"40", align 8
  %9 = load i32, ptr addrspace(5) %"42", align 4
  %"54" = inttoptr i64 %8 to ptr
  store i32 %9, ptr %"54", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }