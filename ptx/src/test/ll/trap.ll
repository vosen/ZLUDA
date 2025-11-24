define amdgpu_kernel void @trap(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  call void @llvm.trap()
  unreachable
}

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { cold noreturn nounwind memory(inaccessiblemem: write) }