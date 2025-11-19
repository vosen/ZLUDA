; ModuleID = 'llvm-link'
source_filename = "llvm-link"
target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-p7:160:256:256:32-p8:128:128-p9:192:256:256:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7:8:9"
target triple = "amdgcn-amd-amdhsa"

; Function Attrs: mustprogress nofree norecurse nounwind willreturn memory(write, inaccessiblemem: none)
define amdgpu_kernel void @mma_m16n8k32_s32_s8_s8_s32(ptr addrspace(4) nocapture readonly byref(i64) %"149") local_unnamed_addr #0 {
"115":
  %0 = load i64, ptr addrspace(4) %"149", align 8, !amdgpu.noclobber !4
  %1 = tail call noundef range(i32 0, 1024) i32 @llvm.amdgcn.workitem.id.x()
  %2 = trunc nuw nsw i32 %1 to i16
  %"187" = shl nuw nsw i16 %2, 4
  %"189" = or disjoint i16 %"187", 1
  %"191" = or disjoint i16 %"187", 2
  %"193" = or disjoint i16 %"187", 3
  %"195" = or disjoint i16 %"187", 4
  %"197" = or disjoint i16 %"187", 5
  %"199" = or disjoint i16 %"187", 6
  %"201" = or disjoint i16 %"187", 7
  %"203" = or disjoint i16 %"187", 8
  %"205" = or disjoint i16 %"187", 9
  %"207" = or disjoint i16 %"187", 10
  %"209" = or disjoint i16 %"187", 11
  %"211" = or disjoint i16 %"187", 12
  %"213" = or disjoint i16 %"187", 13
  %"215" = or disjoint i16 %"187", 14
  %"217" = or disjoint i16 %"187", 15
  %3 = zext nneg i16 %"187" to i32
  %4 = zext nneg i16 %"189" to i32
  %5 = zext nneg i16 %"191" to i32
  %6 = zext nneg i16 %"193" to i32
  %7 = shl nuw nsw i32 %4, 8
  %8 = shl nuw nsw i32 %5, 16
  %9 = shl i32 %6, 24
  %10 = or i32 %8, %7
  %11 = or i32 %10, %9
  %"313.i" = or i32 %11, %3
  %12 = zext nneg i16 %"195" to i32
  %13 = zext nneg i16 %"197" to i32
  %14 = zext nneg i16 %"199" to i32
  %15 = zext nneg i16 %"201" to i32
  %16 = shl nuw nsw i32 %13, 8
  %17 = shl nuw nsw i32 %14, 16
  %18 = shl i32 %15, 24
  %"307.i43" = or i32 %16, %12
  %"310.i44" = or i32 %"307.i43", %17
  %"313.i45" = or i32 %"310.i44", %18
  %19 = zext nneg i16 %"203" to i32
  %20 = zext nneg i16 %"205" to i32
  %21 = zext nneg i16 %"207" to i32
  %22 = zext nneg i16 %"209" to i32
  %23 = shl nuw nsw i32 %20, 8
  %24 = shl nuw nsw i32 %21, 16
  %25 = shl i32 %22, 24
  %"307.i46" = or i32 %23, %19
  %"310.i47" = or i32 %"307.i46", %24
  %"313.i48" = or i32 %"310.i47", %25
  %26 = zext nneg i16 %"211" to i32
  %27 = zext nneg i16 %"213" to i32
  %28 = zext nneg i16 %"215" to i32
  %29 = zext nneg i16 %"217" to i32
  %30 = shl nuw nsw i32 %27, 8
  %31 = shl nuw nsw i32 %28, 16
  %32 = shl i32 %29, 24
  %"307.i49" = or i32 %30, %26
  %"310.i50" = or i32 %"307.i49", %31
  %"313.i51" = or i32 %"310.i50", %32
  %33 = tail call noundef i32 @llvm.amdgcn.mbcnt.lo(i32 -1, i32 0)
  %34 = shl i32 %33, 3
  %35 = and i32 %34, 24
  %36 = and i32 %33, 3
  %37 = or disjoint i32 %35, %36
  %38 = shl nuw nsw i32 %37, 2
  %39 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %38, i32 %"313.i")
  %40 = or disjoint i32 %38, 16
  %41 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %40, i32 %"313.i")
  %42 = add i32 %33, 1
  %43 = and i32 %42, 3
  %44 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %"313.i", i32 57, i32 15, i32 15, i1 true)
  %45 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %"313.i45", i32 57, i32 15, i32 15, i1 true)
  %46 = or disjoint i32 %43, %35
  %47 = shl nuw nsw i32 %46, 2
  %48 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %47, i32 %"313.i")
  %49 = or disjoint i32 %47, 16
  %50 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %49, i32 %"313.i")
  %51 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %"313.i", i32 78, i32 15, i32 15, i1 true)
  %52 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %"313.i45", i32 78, i32 15, i32 15, i1 true)
  %53 = xor i32 %38, 8
  %54 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %53, i32 %"313.i")
  %55 = or disjoint i32 %53, 16
  %56 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %55, i32 %"313.i")
  %57 = add i32 %33, 3
  %58 = and i32 %57, 3
  %59 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %"313.i", i32 147, i32 15, i32 15, i1 true)
  %60 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %"313.i45", i32 147, i32 15, i32 15, i1 true)
  %61 = or disjoint i32 %58, %35
  %62 = shl nuw nsw i32 %61, 2
  %63 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %62, i32 %"313.i")
  %64 = or disjoint i32 %62, 16
  %65 = tail call i32 @llvm.amdgcn.ds.bpermute(i32 %64, i32 %"313.i")
  %66 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %"313.i48", i32 57, i32 15, i32 15, i1 true)
  %67 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %"313.i51", i32 57, i32 15, i32 15, i1 true)
  %68 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %"313.i48", i32 78, i32 15, i32 15, i1 true)
  %69 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %"313.i51", i32 78, i32 15, i32 15, i1 true)
  %70 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %"313.i48", i32 147, i32 15, i32 15, i1 true)
  %71 = tail call i32 @llvm.amdgcn.update.dpp.i32(i32 poison, i32 %"313.i51", i32 147, i32 15, i32 15, i1 true)
  %72 = tail call i32 @llvm.amdgcn.sdot4(i32 %"313.i", i32 %39, i32 %3, i1 false)
  %73 = tail call i32 @llvm.amdgcn.sdot4(i32 %44, i32 %48, i32 %72, i1 false)
  %74 = tail call i32 @llvm.amdgcn.sdot4(i32 %51, i32 %54, i32 %73, i1 false)
  %75 = tail call i32 @llvm.amdgcn.sdot4(i32 %59, i32 %63, i32 %74, i1 false)
  %76 = tail call i32 @llvm.amdgcn.sdot4(i32 %"313.i48", i32 %39, i32 %75, i1 false)
  %77 = tail call i32 @llvm.amdgcn.sdot4(i32 %66, i32 %48, i32 %76, i1 false)
  %78 = tail call i32 @llvm.amdgcn.sdot4(i32 %68, i32 %54, i32 %77, i1 false)
  %79 = tail call i32 @llvm.amdgcn.sdot4(i32 %70, i32 %63, i32 %78, i1 false)
  %80 = tail call i32 @llvm.amdgcn.sdot4(i32 %"313.i", i32 %41, i32 %4, i1 false)
  %81 = tail call i32 @llvm.amdgcn.sdot4(i32 %44, i32 %50, i32 %80, i1 false)
  %82 = tail call i32 @llvm.amdgcn.sdot4(i32 %51, i32 %56, i32 %81, i1 false)
  %83 = tail call i32 @llvm.amdgcn.sdot4(i32 %59, i32 %65, i32 %82, i1 false)
  %84 = tail call i32 @llvm.amdgcn.sdot4(i32 %"313.i48", i32 %41, i32 %83, i1 false)
  %85 = tail call i32 @llvm.amdgcn.sdot4(i32 %66, i32 %50, i32 %84, i1 false)
  %86 = tail call i32 @llvm.amdgcn.sdot4(i32 %68, i32 %56, i32 %85, i1 false)
  %87 = tail call i32 @llvm.amdgcn.sdot4(i32 %70, i32 %65, i32 %86, i1 false)
  %88 = tail call i32 @llvm.amdgcn.sdot4(i32 %"313.i45", i32 %39, i32 %5, i1 false)
  %89 = tail call i32 @llvm.amdgcn.sdot4(i32 %45, i32 %48, i32 %88, i1 false)
  %90 = tail call i32 @llvm.amdgcn.sdot4(i32 %52, i32 %54, i32 %89, i1 false)
  %91 = tail call i32 @llvm.amdgcn.sdot4(i32 %60, i32 %63, i32 %90, i1 false)
  %92 = tail call i32 @llvm.amdgcn.sdot4(i32 %"313.i51", i32 %39, i32 %91, i1 false)
  %93 = tail call i32 @llvm.amdgcn.sdot4(i32 %67, i32 %48, i32 %92, i1 false)
  %94 = tail call i32 @llvm.amdgcn.sdot4(i32 %69, i32 %54, i32 %93, i1 false)
  %95 = tail call i32 @llvm.amdgcn.sdot4(i32 %71, i32 %63, i32 %94, i1 false)
  %96 = tail call i32 @llvm.amdgcn.sdot4(i32 %"313.i45", i32 %41, i32 %6, i1 false)
  %97 = tail call i32 @llvm.amdgcn.sdot4(i32 %45, i32 %50, i32 %96, i1 false)
  %98 = tail call i32 @llvm.amdgcn.sdot4(i32 %52, i32 %56, i32 %97, i1 false)
  %99 = tail call i32 @llvm.amdgcn.sdot4(i32 %60, i32 %65, i32 %98, i1 false)
  %100 = tail call i32 @llvm.amdgcn.sdot4(i32 %"313.i51", i32 %41, i32 %99, i1 false)
  %101 = tail call i32 @llvm.amdgcn.sdot4(i32 %67, i32 %50, i32 %100, i1 false)
  %102 = tail call i32 @llvm.amdgcn.sdot4(i32 %69, i32 %56, i32 %101, i1 false)
  %103 = tail call i32 @llvm.amdgcn.sdot4(i32 %71, i32 %65, i32 %102, i1 false)
  %104 = shl nuw nsw i32 %1, 4
  %"286" = zext nneg i32 %104 to i64
  %"288" = add i64 %0, %"286"
  %"332" = inttoptr i64 %"288" to ptr
  store i32 %79, ptr %"332", align 4
  %"109" = getelementptr inbounds i8, ptr %"332", i64 4
  store i32 %87, ptr %"109", align 4
  %"111" = getelementptr inbounds i8, ptr %"332", i64 8
  store i32 %95, ptr %"111", align 4
  %"113" = getelementptr inbounds i8, ptr %"332", i64 12
  store i32 %103, ptr %"113", align 4
  ret void
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(none)
declare i32 @llvm.amdgcn.mbcnt.lo(i32, i32) #1

; Function Attrs: convergent mustprogress nocallback nofree nounwind willreturn memory(none)
declare i32 @llvm.amdgcn.ds.bpermute(i32, i32) #2

; Function Attrs: convergent mustprogress nocallback nofree nounwind willreturn memory(none)
declare i32 @llvm.amdgcn.update.dpp.i32(i32, i32, i32 immarg, i32 immarg, i32 immarg, i1 immarg) #2

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare noundef i32 @llvm.amdgcn.workitem.id.x() #3

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.amdgcn.sdot4(i32, i32, i32, i1 immarg) #3

attributes #0 = { mustprogress nofree norecurse nounwind willreturn memory(write, inaccessiblemem: none) "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { mustprogress nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { convergent mustprogress nocallback nofree nounwind willreturn memory(none) }
attributes #3 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }

!llvm.module.flags = !{!0, !1}
!opencl.ocl.version = !{!2}
!llvm.ident = !{!3}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 2, i32 0}
!3 = !{!"AMD clang version 19.0.0git (https://github.com/RadeonOpenCompute/llvm-project roc-6.4.0 25133 c7fe45cf4b819c5991fe208aaa96edf142730f1d)"}
!4 = !{}
