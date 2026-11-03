; Function Attrs: denormal_fpenv(float: preservesign)
define amdgpu_kernel void @cvt_u64_f16(ptr addrspace(4) byref(i64) %"52", ptr addrspace(4) byref(i64) %"53") #0 {
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca half, align 2, addrspace(5)
  %"57" = alloca half, align 2, addrspace(5)
  %"58" = alloca half, align 2, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"51"

"51":                                             ; preds = %1
  call void @llvm.amdgcn.s.dcache.inv()
  %2 = load i64, ptr addrspace(4) %"52", align 8
  store i64 %2, ptr addrspace(5) %"54", align 8
  %3 = load i64, ptr addrspace(4) %"53", align 8
  store i64 %3, ptr addrspace(5) %"55", align 8
  %4 = load i64, ptr addrspace(5) %"54", align 8
  %"83" = inttoptr i64 %4 to ptr
  %5 = load i16, ptr %"83", align 2
  %"64" = bitcast i16 %5 to half
  store half %"64", ptr addrspace(5) %"56", align 2
  %6 = load i64, ptr addrspace(5) %"54", align 8
  %"84" = inttoptr i64 %6 to ptr
  %"44" = getelementptr inbounds i8, ptr %"84", i64 2
  %7 = load i16, ptr %"44", align 2
  %"67" = bitcast i16 %7 to half
  store half %"67", ptr addrspace(5) %"57", align 2
  %8 = load i64, ptr addrspace(5) %"54", align 8
  %"86" = inttoptr i64 %8 to ptr
  %"46" = getelementptr inbounds i8, ptr %"86", i64 4
  %9 = load i16, ptr %"46", align 2
  %"69" = bitcast i16 %9 to half
  store half %"69", ptr addrspace(5) %"58", align 2
  %10 = load half, ptr addrspace(5) %"56", align 2
  %11 = call i64 @llvm.fptoui.sat.i64.f16(half %10)
  %12 = call i1 @llvm.is.fpclass.f16(half %10, /* (nan) */ i32 3)
  %13 = select i1 %12, i64 -9223372036854775808, i64 %11
  store i64 %13, ptr addrspace(5) %"59", align 8
  %14 = load half, ptr addrspace(5) %"57", align 2
  %15 = call i64 @llvm.fptoui.sat.i64.f16(half %14)
  %16 = call i1 @llvm.is.fpclass.f16(half %14, /* (nan) */ i32 3)
  %17 = select i1 %16, i64 -9223372036854775808, i64 %15
  store i64 %17, ptr addrspace(5) %"60", align 8
  %18 = load half, ptr addrspace(5) %"58", align 2
  %19 = call i64 @llvm.fptoui.sat.i64.f16(half %18)
  %20 = call i1 @llvm.is.fpclass.f16(half %18, /* (nan) */ i32 3)
  %21 = select i1 %20, i64 -9223372036854775808, i64 %19
  store i64 %21, ptr addrspace(5) %"61", align 8
  %22 = load i64, ptr addrspace(5) %"55", align 8
  %23 = load i64, ptr addrspace(5) %"59", align 8
  %"88" = inttoptr i64 %22 to ptr
  store i64 %23, ptr %"88", align 8
  %24 = load i64, ptr addrspace(5) %"55", align 8
  %"89" = inttoptr i64 %24 to ptr
  %"48" = getelementptr inbounds i8, ptr %"89", i64 8
  %25 = load i64, ptr addrspace(5) %"60", align 8
  store i64 %25, ptr %"48", align 8
  %26 = load i64, ptr addrspace(5) %"55", align 8
  %"90" = inttoptr i64 %26 to ptr
  %"50" = getelementptr inbounds i8, ptr %"90", i64 16
  %27 = load i64, ptr addrspace(5) %"61", align 8
  store i64 %27, ptr %"50", align 8
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.dcache.inv() #1

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.fptoui.sat.i64.f16(half) #2

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare i1 @llvm.is.fpclass.f16(half, i32 immarg) #2

attributes #0 = { denormal_fpenv(float: preservesign) "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind willreturn }
attributes #2 = { nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none) }
