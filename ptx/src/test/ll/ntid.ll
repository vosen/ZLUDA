declare hidden i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

define amdgpu_kernel void @ntid(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #1 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i8, align 1, addrspace(5)
  store i8 0, ptr addrspace(5) %"48", align 1
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"45" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"45", ptr addrspace(5) %"41", align 8
  %"47" = load i64, ptr addrspace(5) %"40", align 8
  %"56" = inttoptr i64 %"47" to ptr
  %"46" = load i32, ptr %"56", align 4
  store i32 %"46", ptr addrspace(5) %"42", align 4
  %"49" = load i8, ptr addrspace(5) %"48", align 1
  %"34" = call i32 @__zluda_ptx_impl_sreg_ntid(i8 %"49")
  br label %"36"

"36":                                             ; preds = %"35"
  store i32 %"34", ptr addrspace(5) %"43", align 4
  %"52" = load i32, ptr addrspace(5) %"42", align 4
  %"53" = load i32, ptr addrspace(5) %"43", align 4
  %"51" = add i32 %"52", %"53"
  store i32 %"51", ptr addrspace(5) %"42", align 4
  %"54" = load i64, ptr addrspace(5) %"41", align 8
  %"55" = load i32, ptr addrspace(5) %"42", align 4
  %"57" = inttoptr i64 %"54" to ptr
  store i32 %"55", ptr %"57", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }