@0 = addrspace(4) global i8 0

declare hidden i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

define amdgpu_kernel void @ntid(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #1 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"45", ptr addrspace(5) %"41", align 8
  %"46" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"46", ptr addrspace(5) %"42", align 8
  %"48" = load i64, ptr addrspace(5) %"41", align 8
  %"55" = inttoptr i64 %"48" to ptr
  %"47" = load i32, ptr %"55", align 4
  store i32 %"47", ptr addrspace(5) %"43", align 4
  %"34" = load i8, ptr addrspace(4) @0, align 1
  %"35" = call i32 @__zluda_ptx_impl_sreg_ntid(i8 %"34")
  br label %"37"

"37":                                             ; preds = %"36"
  store i32 %"35", ptr addrspace(5) %"44", align 4
  %"51" = load i32, ptr addrspace(5) %"43", align 4
  %"52" = load i32, ptr addrspace(5) %"44", align 4
  %"50" = add i32 %"51", %"52"
  store i32 %"50", ptr addrspace(5) %"43", align 4
  %"53" = load i64, ptr addrspace(5) %"42", align 8
  %"54" = load i32, ptr addrspace(5) %"43", align 4
  %"56" = inttoptr i64 %"53" to ptr
  store i32 %"54", ptr %"56", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }