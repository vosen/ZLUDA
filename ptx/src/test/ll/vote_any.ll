declare hidden i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @vote_any(ptr addrspace(4) byref(i64) %"49") #1 {
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i1, align 1, addrspace(5)
  %"52" = alloca i1, align 1, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"49", align 8
  store i64 %2, ptr addrspace(5) %"54", align 8
  %"41" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"41", ptr addrspace(5) %"50", align 4
  %3 = load i32, ptr addrspace(5) %"50", align 4
  %4 = icmp uge i32 %3, 32
  store i1 %4, ptr addrspace(5) %"51", align 1
  %5 = load i1, ptr addrspace(5) %"51", align 1
  %"59" = call i1 @__zluda_ptx_impl_vote_sync_any_pred_negate(i1 %5, i32 -1)
  store i1 %"59", ptr addrspace(5) %"52", align 1
  %6 = load i1, ptr addrspace(5) %"52", align 1
  %"61" = select i1 %6, i32 1, i32 0
  store i32 %"61", ptr addrspace(5) %"53", align 4
  %7 = load i32, ptr addrspace(5) %"50", align 4
  %8 = zext i32 %7 to i64
  %"64" = mul i64 %8, 4
  store i64 %"64", ptr addrspace(5) %"63", align 8
  %9 = load i64, ptr addrspace(5) %"54", align 8
  %10 = load i64, ptr addrspace(5) %"63", align 8
  %"66" = add i64 %9, %10
  store i64 %"66", ptr addrspace(5) %"54", align 8
  %11 = load i64, ptr addrspace(5) %"54", align 8
  %12 = load i32, ptr addrspace(5) %"53", align 4
  %"71" = inttoptr i64 %11 to ptr
  store i32 %12, ptr %"71", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
