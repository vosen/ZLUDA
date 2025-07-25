declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

define amdgpu_kernel void @ntid(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #1 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i32, align 4, addrspace(5)
  %"40" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"41" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"41", ptr addrspace(5) %"37", align 8
  %"42" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"42", ptr addrspace(5) %"38", align 8
  %"44" = load i64, ptr addrspace(5) %"37", align 8
  %"51" = inttoptr i64 %"44" to ptr
  %"43" = load i32, ptr %"51", align 4
  store i32 %"43", ptr addrspace(5) %"39", align 4
  %"31" = call i32 @__zluda_ptx_impl_sreg_ntid(i8 0)
  br label %"33"

"33":                                             ; preds = %"32"
  store i32 %"31", ptr addrspace(5) %"40", align 4
  %"47" = load i32, ptr addrspace(5) %"39", align 4
  %"48" = load i32, ptr addrspace(5) %"40", align 4
  %"46" = add i32 %"47", %"48"
  store i32 %"46", ptr addrspace(5) %"39", align 4
  %"49" = load i64, ptr addrspace(5) %"38", align 8
  %"50" = load i32, ptr addrspace(5) %"39", align 4
  %"52" = inttoptr i64 %"49" to ptr
  store i32 %"50", ptr %"52", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }