declare hidden i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

define amdgpu_kernel void @ntid(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #1 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"45" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"45", ptr addrspace(5) %"41", align 8
  %"47" = load i64, ptr addrspace(5) %"40", align 8
  %"54" = inttoptr i64 %"47" to ptr
  %"46" = load i32, ptr %"54", align 4
  store i32 %"46", ptr addrspace(5) %"42", align 4
  %"34" = call i32 @__zluda_ptx_impl_sreg_ntid(i8 0)
  br label %"36"

"36":                                             ; preds = %"35"
  store i32 %"34", ptr addrspace(5) %"43", align 4
  %"50" = load i32, ptr addrspace(5) %"42", align 4
  %"51" = load i32, ptr addrspace(5) %"43", align 4
  %"49" = add i32 %"50", %"51"
  store i32 %"49", ptr addrspace(5) %"42", align 4
  %"52" = load i64, ptr addrspace(5) %"41", align 8
  %"53" = load i32, ptr addrspace(5) %"42", align 4
  %"55" = inttoptr i64 %"52" to ptr
  store i32 %"53", ptr %"55", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }