@0 = addrspace(4) global i8 0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @tid(ptr addrspace(4) byref(i64) %"38") #1 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i32, align 4, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i8, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"33" = load i8, ptr addrspace(4) @0, align 1
  %"34" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"33")
  br label %"36"

"36":                                             ; preds = %"35"
  store i32 %"34", ptr addrspace(5) %"40", align 4
  %"45" = load i32, ptr addrspace(5) %"40", align 4
  %"44" = zext i32 %"45" to i64
  store i64 %"44", ptr addrspace(5) %"41", align 8
  %"47" = load i32, ptr addrspace(5) %"40", align 4
  %"46" = trunc i32 %"47" to i8
  store i8 %"46", ptr addrspace(5) %"42", align 1
  %"48" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"48", ptr addrspace(5) %"39", align 8
  %"50" = load i64, ptr addrspace(5) %"39", align 8
  %"51" = load i64, ptr addrspace(5) %"41", align 8
  %"49" = add i64 %"50", %"51"
  store i64 %"49", ptr addrspace(5) %"39", align 8
  %"52" = load i64, ptr addrspace(5) %"39", align 8
  %"53" = load i8, ptr addrspace(5) %"42", align 1
  %"54" = inttoptr i64 %"52" to ptr
  store i8 %"53", ptr %"54", align 1
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }