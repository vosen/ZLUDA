; Function Attrs: denormal_fpenv(preservesign)
define amdgpu_kernel void @createpolicy(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  call void @llvm.amdgcn.s.dcache.inv()
  store i64 0, ptr addrspace(5) %"39", align 8
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.dcache.inv() #1

attributes #0 = { denormal_fpenv(preservesign) "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind willreturn }
