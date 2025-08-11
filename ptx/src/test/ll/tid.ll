declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @tid(ptr addrspace(4) byref(i64) %"34") #1 {
  %"35" = alloca i64, align 8, addrspace(5)
  %"36" = alloca i32, align 4, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i8, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"31"

"31":                                             ; preds = %1
  %"30" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"32"

"32":                                             ; preds = %"31"
  store i32 %"30", ptr addrspace(5) %"36", align 4
  %"41" = load i32, ptr addrspace(5) %"36", align 4
  %"40" = zext i32 %"41" to i64
  store i64 %"40", ptr addrspace(5) %"37", align 8
  %"43" = load i32, ptr addrspace(5) %"36", align 4
  %"42" = trunc i32 %"43" to i8
  store i8 %"42", ptr addrspace(5) %"38", align 1
  %"44" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"44", ptr addrspace(5) %"35", align 8
  %"46" = load i64, ptr addrspace(5) %"35", align 8
  %"47" = load i64, ptr addrspace(5) %"37", align 8
  %"45" = add i64 %"46", %"47"
  store i64 %"45", ptr addrspace(5) %"35", align 8
  %"48" = load i64, ptr addrspace(5) %"35", align 8
  %"49" = load i8, ptr addrspace(5) %"38", align 1
  %"50" = inttoptr i64 %"48" to ptr
  store i8 %"49", ptr %"50", align 1
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }