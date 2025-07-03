declare { i32, i1 } @__zluda_ptx_impl_shfl_sync_down_b32_pred(i32, i32, i32, i32) #0

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @shfl_sync_down_b32_pred(ptr addrspace(4) byref(i64) %"42") #1 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %"48" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"48", ptr addrspace(5) %"43", align 4
  %"33" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"40"

"40":                                             ; preds = %"39"
  store i32 %"33", ptr addrspace(5) %"45", align 4
  %"52" = load i32, ptr addrspace(5) %"45", align 4
  %2 = call { i32, i1 } @__zluda_ptx_impl_shfl_sync_down_b32_pred(i32 %"52", i32 3, i32 31, i32 -1)
  %"65" = extractvalue { i32, i1 } %2, 0
  %"51" = extractvalue { i32, i1 } %2, 1
  store i32 %"65", ptr addrspace(5) %"46", align 4
  store i1 %"51", ptr addrspace(5) %"47", align 1
  %"53" = load i1, ptr addrspace(5) %"47", align 1
  br i1 %"53", label %"15", label %"14"

"14":                                             ; preds = %"40"
  %"55" = load i32, ptr addrspace(5) %"46", align 4
  %"54" = add i32 %"55", 1000
  store i32 %"54", ptr addrspace(5) %"46", align 4
  br label %"15"

"15":                                             ; preds = %"14", %"40"
  %"57" = load i32, ptr addrspace(5) %"45", align 4
  %"56" = zext i32 %"57" to i64
  store i64 %"56", ptr addrspace(5) %"44", align 4
  %"59" = load i64, ptr addrspace(5) %"44", align 4
  %"58" = mul i64 %"59", 4
  store i64 %"58", ptr addrspace(5) %"44", align 4
  %"61" = load i64, ptr addrspace(5) %"43", align 4
  %"62" = load i64, ptr addrspace(5) %"44", align 4
  %"60" = add i64 %"61", %"62"
  store i64 %"60", ptr addrspace(5) %"43", align 4
  %"63" = load i64, ptr addrspace(5) %"43", align 4
  %"64" = load i32, ptr addrspace(5) %"46", align 4
  %"67" = inttoptr i64 %"63" to ptr
  store i32 %"64", ptr %"67", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }