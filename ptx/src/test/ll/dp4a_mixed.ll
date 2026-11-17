; Function Attrs: denormal_fpenv(dynamic)
declare hidden i32 @__zluda_ptx_impl_dp4a_s32_u32(i32, i32, i32) #0

; Function Attrs: denormal_fpenv(dynamic)
declare hidden i32 @__zluda_ptx_impl_dp4a_u32_s32(i32, i32, i32) #0

; Function Attrs: denormal_fpenv(preservesign)
define amdgpu_kernel void @dp4a_mixed(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #1 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  call void @llvm.amdgcn.s.dcache.inv()
  %2 = load i64, ptr addrspace(4) %"49", align 8
  store i64 %2, ptr addrspace(5) %"51", align 8
  %3 = load i64, ptr addrspace(4) %"50", align 8
  store i64 %3, ptr addrspace(5) %"52", align 8
  %4 = load i64, ptr addrspace(5) %"51", align 8
  %"78" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"78", align 4
  store i32 %5, ptr addrspace(5) %"53", align 4
  %6 = load i64, ptr addrspace(5) %"51", align 8
  %"79" = inttoptr i64 %6 to ptr
  %"43" = getelementptr inbounds i8, ptr %"79", i64 4
  %7 = load i32, ptr %"43", align 4
  store i32 %7, ptr addrspace(5) %"54", align 4
  %8 = load i64, ptr addrspace(5) %"51", align 8
  %"80" = inttoptr i64 %8 to ptr
  %"45" = getelementptr inbounds i8, ptr %"80", i64 8
  %9 = load i32, ptr %"45", align 4
  store i32 %9, ptr addrspace(5) %"55", align 4
  %10 = load i32, ptr addrspace(5) %"53", align 4
  %11 = load i32, ptr addrspace(5) %"54", align 4
  %12 = load i32, ptr addrspace(5) %"55", align 4
  %"66" = call i32 @__zluda_ptx_impl_dp4a_s32_u32(i32 %10, i32 %11, i32 %12)
  store i32 %"66", ptr addrspace(5) %"56", align 4
  %13 = load i32, ptr addrspace(5) %"53", align 4
  %14 = load i32, ptr addrspace(5) %"54", align 4
  %15 = load i32, ptr addrspace(5) %"55", align 4
  %"70" = call i32 @__zluda_ptx_impl_dp4a_u32_s32(i32 %13, i32 %14, i32 %15)
  store i32 %"70", ptr addrspace(5) %"57", align 4
  %16 = load i64, ptr addrspace(5) %"52", align 8
  %17 = load i32, ptr addrspace(5) %"56", align 4
  %"87" = inttoptr i64 %16 to ptr
  store i32 %17, ptr %"87", align 4
  %18 = load i64, ptr addrspace(5) %"52", align 8
  %"88" = inttoptr i64 %18 to ptr
  %"47" = getelementptr inbounds i8, ptr %"88", i64 4
  %19 = load i32, ptr addrspace(5) %"57", align 4
  store i32 %19, ptr %"47", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.dcache.inv() #2

attributes #0 = { denormal_fpenv(dynamic) "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { denormal_fpenv(preservesign) "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }
