; Function Attrs: denormal_fpenv(dynamic)
declare hidden i32 @__zluda_ptx_impl_match_any_sync_b32(i32, i32) #0

; Function Attrs: denormal_fpenv(dynamic)
declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

; Function Attrs: denormal_fpenv(preservesign)
define amdgpu_kernel void @match_sync(ptr addrspace(4) byref(i64) %"46") #1 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  call void @llvm.amdgcn.s.dcache.inv()
  %2 = load i64, ptr addrspace(4) %"46", align 8
  store i64 %2, ptr addrspace(5) %"47", align 8
  %"40" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  store i32 %"40", ptr addrspace(5) %"49", align 4
  %3 = load i32, ptr addrspace(5) %"49", align 4
  %"67" = and i32 %3, 1
  store i32 %"67", ptr addrspace(5) %"50", align 4
  %4 = load i32, ptr addrspace(5) %"50", align 4
  %"69" = call i32 @__zluda_ptx_impl_match_any_sync_b32(i32 %4, i32 -1)
  store i32 %"69", ptr addrspace(5) %"51", align 4
  %5 = load i32, ptr addrspace(5) %"49", align 4
  %"58" = zext i32 %5 to i64
  store i64 %"58", ptr addrspace(5) %"48", align 8
  %6 = load i64, ptr addrspace(5) %"48", align 8
  %"60" = mul i64 %6, 4
  store i64 %"60", ptr addrspace(5) %"48", align 8
  %7 = load i64, ptr addrspace(5) %"47", align 8
  %8 = load i64, ptr addrspace(5) %"48", align 8
  %"62" = add i64 %7, %8
  store i64 %"62", ptr addrspace(5) %"47", align 8
  %9 = load i64, ptr addrspace(5) %"47", align 8
  %10 = load i32, ptr addrspace(5) %"51", align 4
  %"71" = inttoptr i64 %9 to ptr
  store i32 %10, ptr %"71", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.dcache.inv() #2

attributes #0 = { denormal_fpenv(dynamic) "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { denormal_fpenv(preservesign) "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }
