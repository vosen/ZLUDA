; Function Attrs: denormal_fpenv(preservesign)
define amdgpu_kernel void @trap(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  call void @llvm.amdgcn.s.dcache.inv()
  call void @llvm.trap()
  unreachable
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.dcache.inv() #1

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #2

attributes #0 = { denormal_fpenv(preservesign) "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind willreturn }
attributes #2 = { cold noreturn nounwind memory(inaccessiblemem: write) }
