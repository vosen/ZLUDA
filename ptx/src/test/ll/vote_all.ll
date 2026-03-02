declare hidden i1 @__zluda_ptx_impl_vote_sync_all_pred(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare hidden i32 @__zluda_ptx_impl_sreg_laneid() #0

define amdgpu_kernel void @vote_all(ptr addrspace(4) byref(i64) %"55") #1 {
  %"56" = alloca i32, align 4, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i1, align 1, addrspace(5)
  %"59" = alloca i1, align 1, addrspace(5)
  %"60" = alloca i32, align 4, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  %"73" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"52"

"52":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"55", align 8
  store i64 %2, ptr addrspace(5) %"61", align 8
  %"43" = call i32 @__zluda_ptx_impl_sreg_laneid()
  store i32 %"43", ptr addrspace(5) %"56", align 4
  %"45" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"45", ptr addrspace(5) %"57", align 4
  %3 = load i32, ptr addrspace(5) %"56", align 4
  %4 = icmp ne i32 %3, 0
  store i1 %4, ptr addrspace(5) %"58", align 1
  store i1 false, ptr addrspace(5) %"59", align 1
  %5 = load i1, ptr addrspace(5) %"58", align 1
  br i1 %5, label %"19", label %"20"

"19":                                             ; preds = %"52"
  %6 = load i1, ptr addrspace(5) %"58", align 1
  %"69" = call i1 @__zluda_ptx_impl_vote_sync_all_pred(i1 %6, i32 -2)
  store i1 %"69", ptr addrspace(5) %"59", align 1
  br label %"20"

"20":                                             ; preds = %"19", %"52"
  %7 = load i1, ptr addrspace(5) %"59", align 1
  %"71" = select i1 %7, i32 1, i32 0
  store i32 %"71", ptr addrspace(5) %"60", align 4
  %8 = load i32, ptr addrspace(5) %"57", align 4
  %9 = zext i32 %8 to i64
  %"74" = mul i64 %9, 4
  store i64 %"74", ptr addrspace(5) %"73", align 8
  %10 = load i64, ptr addrspace(5) %"61", align 8
  %11 = load i64, ptr addrspace(5) %"73", align 8
  %"76" = add i64 %10, %11
  store i64 %"76", ptr addrspace(5) %"61", align 8
  %12 = load i64, ptr addrspace(5) %"61", align 8
  %13 = load i32, ptr addrspace(5) %"60", align 4
  %"81" = inttoptr i64 %12 to ptr
  store i32 %13, ptr %"81", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
