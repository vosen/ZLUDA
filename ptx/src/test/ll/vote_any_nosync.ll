declare hidden i1 @__zluda_ptx_impl_vote_any_pred_negate(i1) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @vote_any_nosync(ptr addrspace(4) byref(i64) %"48") #1 {
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i1, align 1, addrspace(5)
  %"51" = alloca i1, align 1, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"48", align 8
  store i64 %2, ptr addrspace(5) %"53", align 8
  %"41" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"41", ptr addrspace(5) %"49", align 4
  %3 = load i32, ptr addrspace(5) %"49", align 4
  %4 = icmp uge i32 %3, 32
  store i1 %4, ptr addrspace(5) %"50", align 1
  %5 = load i1, ptr addrspace(5) %"50", align 1
  %"58" = call i1 @__zluda_ptx_impl_vote_any_pred_negate(i1 %5)
  store i1 %"58", ptr addrspace(5) %"51", align 1
  %6 = load i1, ptr addrspace(5) %"51", align 1
  %"60" = select i1 %6, i32 1, i32 0
  store i32 %"60", ptr addrspace(5) %"52", align 4
  %7 = load i32, ptr addrspace(5) %"49", align 4
  %8 = zext i32 %7 to i64
  %"63" = mul i64 %8, 4
  store i64 %"63", ptr addrspace(5) %"62", align 8
  %9 = load i64, ptr addrspace(5) %"53", align 8
  %10 = load i64, ptr addrspace(5) %"62", align 8
  %"65" = add i64 %9, %10
  store i64 %"65", ptr addrspace(5) %"53", align 8
  %11 = load i64, ptr addrspace(5) %"53", align 8
  %12 = load i32, ptr addrspace(5) %"52", align 4
  %"70" = inttoptr i64 %11 to ptr
  store i32 %12, ptr %"70", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }
