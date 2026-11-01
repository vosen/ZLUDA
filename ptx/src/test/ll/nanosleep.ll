; Function Attrs: denormal_fpenv(dynamic)
declare hidden void @__zluda_ptx_impl_nanosleep_u32(i32) #0

; Function Attrs: denormal_fpenv(preservesign)
define amdgpu_kernel void @nanosleep(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #1 {
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  call void @llvm.amdgcn.s.dcache.inv()
  call void @__zluda_ptx_impl_nanosleep_u32(i32 1)
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.dcache.inv() #2

attributes #0 = { denormal_fpenv(dynamic) "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { denormal_fpenv(preservesign) "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }
