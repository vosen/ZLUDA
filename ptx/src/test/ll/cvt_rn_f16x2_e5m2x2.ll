declare hidden i32 @__zluda_ptx_impl_cvt_rn_f16x2_e5m2x2(i16) #0

define amdgpu_kernel void @cvt_rn_f16x2_e5m2x2(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #1 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i16, align 2, addrspace(5)
  %"39" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"40", ptr addrspace(5) %"36", align 8
  %"41" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"41", ptr addrspace(5) %"37", align 8
  %"43" = load i64, ptr addrspace(5) %"36", align 8
  %"48" = inttoptr i64 %"43" to ptr
  %"42" = load i16, ptr %"48", align 2
  store i16 %"42", ptr addrspace(5) %"38", align 2
  %"45" = load i16, ptr addrspace(5) %"38", align 2
  %"52" = call i32 @__zluda_ptx_impl_cvt_rn_f16x2_e5m2x2(i16 %"45")
  %"49" = bitcast i32 %"52" to <2 x half>
  %"44" = bitcast <2 x half> %"49" to i32
  store i32 %"44", ptr addrspace(5) %"39", align 4
  %"46" = load i64, ptr addrspace(5) %"37", align 8
  %"47" = load i32, ptr addrspace(5) %"39", align 4
  %"51" = inttoptr i64 %"46" to ptr
  store i32 %"47", ptr %"51", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }