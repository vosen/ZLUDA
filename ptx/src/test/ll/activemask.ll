; Function Attrs: denormal_fpenv(dynamic)
declare hidden i32 @__zluda_ptx_impl_activemask() #0

; Function Attrs: denormal_fpenv(preservesign)
define amdgpu_kernel void @activemask(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #1 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  call void @llvm.amdgcn.s.dcache.inv()
  %2 = load i64, ptr addrspace(4) %"39", align 8
  store i64 %2, ptr addrspace(5) %"40", align 8
  %"43" = call i32 @__zluda_ptx_impl_activemask()
  store i32 %"43", ptr addrspace(5) %"41", align 4
  %3 = load i64, ptr addrspace(5) %"40", align 8
  %4 = load i32, ptr addrspace(5) %"41", align 4
  %"46" = inttoptr i64 %3 to ptr
  store i32 %4, ptr %"46", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.dcache.inv() #2

attributes #0 = { denormal_fpenv(dynamic) "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { denormal_fpenv(preservesign) "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }
