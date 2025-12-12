declare hidden i1 @__zluda_ptx_impl_vote_sync_all_pred(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare hidden i32 @__zluda_ptx_impl_sreg_laneid() #0

define amdgpu_kernel void @vote_all_sub(ptr addrspace(4) byref(i64) %"54") #1 {
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  %"57" = alloca i1, align 1, addrspace(5)
  %"58" = alloca i1, align 1, addrspace(5)
  %"59" = alloca i32, align 4, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  %"71" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"51"

"51":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"54", align 8
  store i64 %2, ptr addrspace(5) %"60", align 8
  %"41" = call i32 @__zluda_ptx_impl_sreg_laneid()
  store i32 %"41", ptr addrspace(5) %"55", align 4
  %"43" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"43", ptr addrspace(5) %"56", align 4
  %3 = load i32, ptr addrspace(5) %"55", align 4
  %4 = icmp eq i32 %3, 0
  store i1 %4, ptr addrspace(5) %"57", align 1
  store i1 false, ptr addrspace(5) %"58", align 1
  %5 = load i1, ptr addrspace(5) %"57", align 1
  br i1 %5, label %"11", label %"20"

"20":                                             ; preds = %"51"
  %"68" = call i1 @__zluda_ptx_impl_vote_sync_all_pred(i1 true, i32 -1)
  store i1 %"68", ptr addrspace(5) %"58", align 1
  br label %"11"

"11":                                             ; preds = %"20", %"51"
  %6 = load i1, ptr addrspace(5) %"58", align 1
  %"69" = select i1 %6, i32 1, i32 0
  store i32 %"69", ptr addrspace(5) %"59", align 4
  %7 = load i32, ptr addrspace(5) %"56", align 4
  %8 = zext i32 %7 to i64
  %"72" = mul i64 %8, 4
  store i64 %"72", ptr addrspace(5) %"71", align 8
  %9 = load i64, ptr addrspace(5) %"60", align 8
  %10 = load i64, ptr addrspace(5) %"71", align 8
  %"74" = add i64 %9, %10
  store i64 %"74", ptr addrspace(5) %"60", align 8
  %11 = load i64, ptr addrspace(5) %"60", align 8
  %12 = load i32, ptr addrspace(5) %"59", align 4
  %"79" = inttoptr i64 %11 to ptr
  store i32 %12, ptr %"79", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }