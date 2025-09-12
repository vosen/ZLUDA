declare hidden i1 @__zluda_ptx_impl_vote_sync_all_pred(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare hidden i32 @__zluda_ptx_impl_sreg_laneid() #0

define amdgpu_kernel void @vote_all_sub(ptr addrspace(4) byref(i64) %"53") #1 {
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i1, align 1, addrspace(5)
  %"57" = alloca i1, align 1, addrspace(5)
  %"58" = alloca i32, align 4, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  %"70" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %"60" = load i64, ptr addrspace(4) %"53", align 8
  store i64 %"60", ptr addrspace(5) %"59", align 8
  %"38" = call i32 @__zluda_ptx_impl_sreg_laneid()
  br label %"49"

"49":                                             ; preds = %"48"
  store i32 %"38", ptr addrspace(5) %"54", align 4
  %"40" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"50"

"50":                                             ; preds = %"49"
  store i32 %"40", ptr addrspace(5) %"55", align 4
  %"64" = load i32, ptr addrspace(5) %"54", align 4
  %2 = icmp eq i32 %"64", 0
  store i1 %2, ptr addrspace(5) %"56", align 1
  store i1 false, ptr addrspace(5) %"57", align 1
  %"66" = load i1, ptr addrspace(5) %"56", align 1
  br i1 %"66", label %"10", label %"19"

"19":                                             ; preds = %"50"
  %"67" = call i1 @__zluda_ptx_impl_vote_sync_all_pred(i1 true, i32 -1)
  store i1 %"67", ptr addrspace(5) %"57", align 1
  br label %"10"

"10":                                             ; preds = %"19", %"50"
  %"69" = load i1, ptr addrspace(5) %"57", align 1
  %"68" = select i1 %"69", i32 1, i32 0
  store i32 %"68", ptr addrspace(5) %"58", align 4
  %"72" = load i32, ptr addrspace(5) %"55", align 4
  %3 = zext i32 %"72" to i64
  %"71" = mul i64 %3, 4
  store i64 %"71", ptr addrspace(5) %"70", align 8
  %"74" = load i64, ptr addrspace(5) %"59", align 8
  %"75" = load i64, ptr addrspace(5) %"70", align 8
  %"73" = add i64 %"74", %"75"
  store i64 %"73", ptr addrspace(5) %"59", align 8
  %"76" = load i64, ptr addrspace(5) %"59", align 8
  %"77" = load i32, ptr addrspace(5) %"58", align 4
  %"78" = inttoptr i64 %"76" to ptr
  store i32 %"77", ptr %"78", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }