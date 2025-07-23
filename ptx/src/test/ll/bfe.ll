declare i32 @__zluda_ptx_impl_bfe_u32(i32, i32, i32) #0

define amdgpu_kernel void @bfe(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #1 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i32, align 4, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"43" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"43", ptr addrspace(5) %"38", align 8
  %"44" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"44", ptr addrspace(5) %"39", align 8
  %"46" = load i64, ptr addrspace(5) %"38", align 8
  %"57" = inttoptr i64 %"46" to ptr
  %"45" = load i32, ptr %"57", align 4
  store i32 %"45", ptr addrspace(5) %"40", align 4
  %"47" = load i64, ptr addrspace(5) %"38", align 8
  %"58" = inttoptr i64 %"47" to ptr
  %"32" = getelementptr inbounds i8, ptr %"58", i64 4
  %"48" = load i32, ptr %"32", align 4
  store i32 %"48", ptr addrspace(5) %"41", align 4
  %"49" = load i64, ptr addrspace(5) %"38", align 8
  %"59" = inttoptr i64 %"49" to ptr
  %"34" = getelementptr inbounds i8, ptr %"59", i64 8
  %"50" = load i32, ptr %"34", align 4
  store i32 %"50", ptr addrspace(5) %"42", align 4
  %"52" = load i32, ptr addrspace(5) %"40", align 4
  %"53" = load i32, ptr addrspace(5) %"41", align 4
  %"54" = load i32, ptr addrspace(5) %"42", align 4
  %"51" = call i32 @__zluda_ptx_impl_bfe_u32(i32 %"52", i32 %"53", i32 %"54")
  store i32 %"51", ptr addrspace(5) %"40", align 4
  %"55" = load i64, ptr addrspace(5) %"39", align 8
  %"56" = load i32, ptr addrspace(5) %"40", align 4
  %"60" = inttoptr i64 %"55" to ptr
  store i32 %"56", ptr %"60", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }