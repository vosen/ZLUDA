target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-ni:7"
target triple = "amdgcn-amd-amdhsa"

; Function Attrs: nounwind
define amdgpu_kernel void @implicit_param(i64 %0, i64 %1) #0 !kernel_arg_addr_space !6 !kernel_arg_access_qual !7 !kernel_arg_type !8 !kernel_arg_type_qual !9 !kernel_arg_base_type !8 {
  %3 = alloca i64, align 8, addrspace(5)
  %4 = alloca i64, align 8, addrspace(5)
  %5 = alloca i64, align 8, addrspace(5)
  %6 = alloca i64, align 8, addrspace(5)
  %7 = alloca float, align 4, addrspace(5)
  %8 = alloca i32, align 4, addrspace(5)
  store i64 %0, i64 addrspace(5)* %3, align 8
  store i64 %1, i64 addrspace(5)* %4, align 8
  %9 = load i64, i64 addrspace(5)* %3, align 8
  store i64 %9, i64 addrspace(5)* %5, align 8
  %10 = load i64, i64 addrspace(5)* %4, align 8
  store i64 %10, i64 addrspace(5)* %6, align 8
  %11 = load i64, i64 addrspace(5)* %5, align 8
  %12 = inttoptr i64 %11 to float addrspace(1)*
  %13 = load float, float addrspace(1)* %12, align 4
  store float %13, float addrspace(5)* %7, align 4
  %14 = load float, float addrspace(5)* %7, align 4
  %15 = bitcast i32 addrspace(5)* %8 to float addrspace(5)*
  store float %14, float addrspace(5)* %15, align 4
  %16 = bitcast i32 addrspace(5)* %8 to float addrspace(5)*
  %17 = load float, float addrspace(5)* %16, align 4
  store float %17, float addrspace(5)* %7, align 4
  %18 = load i64, i64 addrspace(5)* %6, align 8
  %19 = load float, float addrspace(5)* %7, align 4
  %20 = inttoptr i64 %18 to float addrspace(1)*
  store float %19, float addrspace(1)* %20, align 4
  ret void
}

attributes #0 = { nounwind }

!spirv.MemoryModel = !{!0}
!opencl.enable.FP_CONTRACT = !{}
!spirv.Source = !{!1}
!opencl.spir.version = !{!2}
!opencl.ocl.version = !{!2}
!opencl.used.extensions = !{!3}
!opencl.used.optional.core.features = !{!4}
!spirv.Generator = !{!5}

!0 = !{i32 2, i32 2}
!1 = !{i32 3, i32 102000}
!2 = !{i32 1, i32 2}
!3 = !{!"cl_khr_fp16"}
!4 = !{!"cl_doubles"}
!5 = !{i16 7, i16 0}
!6 = !{i32 5, i32 5}
!7 = !{!"none", !"none"}
!8 = !{!"long", !"long"}
!9 = !{!"", !""}
