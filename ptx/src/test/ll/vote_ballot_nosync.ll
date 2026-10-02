declare hidden i32 @__zluda_ptx_impl_vote_ballot_b32(i1) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @vote_ballot_nosync(ptr addrspace(4) byref(i64) %"45") #1 {
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i1, align 1, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"43"

"43":                                             ; preds = %1
  call void @llvm.amdgcn.s.dcache.inv()
  %2 = load i64, ptr addrspace(4) %"45", align 8
  store i64 %2, ptr addrspace(5) %"49", align 8
  %"40" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"40", ptr addrspace(5) %"46", align 4
  %3 = load i32, ptr addrspace(5) %"46", align 4
  %4 = icmp uge i32 %3, 34
  store i1 %4, ptr addrspace(5) %"47", align 1
  %5 = load i1, ptr addrspace(5) %"47", align 1
  %"64" = call i32 @__zluda_ptx_impl_vote_ballot_b32(i1 %5)
  store i32 %"64", ptr addrspace(5) %"48", align 4
  %6 = load i32, ptr addrspace(5) %"46", align 4
  %7 = zext i32 %6 to i64
  %"57" = mul i64 %7, 4
  store i64 %"57", ptr addrspace(5) %"56", align 8
  %8 = load i64, ptr addrspace(5) %"49", align 8
  %9 = load i64, ptr addrspace(5) %"56", align 8
  %"59" = add i64 %8, %9
  store i64 %"59", ptr addrspace(5) %"49", align 8
  %10 = load i64, ptr addrspace(5) %"49", align 8
  %11 = load i32, ptr addrspace(5) %"48", align 4
  %"65" = inttoptr i64 %10 to ptr
  store i32 %11, ptr %"65", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.dcache.inv() #2

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }
