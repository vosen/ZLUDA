declare hidden i1 @__zluda_ptx_impl_vote_sync_all_pred(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare hidden i32 @__zluda_ptx_impl_sreg_laneid() #0

define amdgpu_kernel void @vote_all(ptr addrspace(4) byref(i64) %"51") #1 {
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i1, align 1, addrspace(5)
  %"55" = alloca i1, align 1, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  %"69" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  %"58" = load i64, ptr addrspace(4) %"51", align 8
  store i64 %"58", ptr addrspace(5) %"57", align 8
  %"37" = call i32 @__zluda_ptx_impl_sreg_laneid()
  br label %"47"

"47":                                             ; preds = %"46"
  store i32 %"37", ptr addrspace(5) %"52", align 4
  %"39" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"48"

"48":                                             ; preds = %"47"
  store i32 %"39", ptr addrspace(5) %"53", align 4
  %"62" = load i32, ptr addrspace(5) %"52", align 4
  %2 = icmp ne i32 %"62", 0
  store i1 %2, ptr addrspace(5) %"54", align 1
  store i1 false, ptr addrspace(5) %"55", align 1
  %"64" = load i1, ptr addrspace(5) %"54", align 1
  br i1 %"64", label %"17", label %"18"

"17":                                             ; preds = %"48"
  %"66" = load i1, ptr addrspace(5) %"54", align 1
  %"65" = call i1 @__zluda_ptx_impl_vote_sync_all_pred(i1 %"66", i32 -2)
  store i1 %"65", ptr addrspace(5) %"55", align 1
  br label %"18"

"18":                                             ; preds = %"17", %"48"
  %"68" = load i1, ptr addrspace(5) %"55", align 1
  %"67" = select i1 %"68", i32 1, i32 0
  store i32 %"67", ptr addrspace(5) %"56", align 4
  %"71" = load i32, ptr addrspace(5) %"53", align 4
  %3 = zext i32 %"71" to i64
  %"70" = mul i64 %3, 4
  store i64 %"70", ptr addrspace(5) %"69", align 8
  %"73" = load i64, ptr addrspace(5) %"57", align 8
  %"74" = load i64, ptr addrspace(5) %"69", align 8
  %"72" = add i64 %"73", %"74"
  store i64 %"72", ptr addrspace(5) %"57", align 8
  %"75" = load i64, ptr addrspace(5) %"57", align 8
  %"76" = load i32, ptr addrspace(5) %"56", align 4
  %"77" = inttoptr i64 %"75" to ptr
  store i32 %"76", ptr %"77", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }