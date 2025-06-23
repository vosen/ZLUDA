; ModuleID = 'zluda_ptx_impl.bc'
source_filename = "zluda_ptx_impl.cpp"
target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-p7:160:256:256:32-p8:128:128-p9:192:256:256:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7:8:9"
target triple = "amdgcn-amd-amdhsa"

@__const.__assert_fail.fmt = private unnamed_addr addrspace(4) constant [47 x i8] c"%s:%u: %s: Device-side assertion `%s' failed.\0A\00", align 16
@__hip_cuid_be8aa2e6609aaa0 = addrspace(1) global i8 0
@llvm.compiler.used = appending addrspace(1) global [1 x ptr] [ptr addrspacecast (ptr addrspace(1) @__hip_cuid_be8aa2e6609aaa0 to ptr)], section "llvm.metadata"

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #0

; Function Attrs: convergent mustprogress noinline noreturn nounwind
define internal fastcc void @__assert_fail(ptr noundef %0, ptr noundef %1, i32 noundef %2, ptr noundef %3) unnamed_addr #1 {
  %5 = alloca [47 x i8], align 16, addrspace(5)
  call void @llvm.lifetime.start.p5(i64 47, ptr addrspace(5) %5) #13
  call void @llvm.memcpy.p5.p4.i64(ptr addrspace(5) noundef align 16 dereferenceable(47) %5, ptr addrspace(4) noundef align 16 dereferenceable(47) @__const.__assert_fail.fmt, i64 47, i1 false)
  %6 = tail call i64 @__ockl_fprintf_stderr_begin() #14
  br label %7

7:                                                ; preds = %7, %4
  %8 = phi ptr addrspace(5) [ %5, %4 ], [ %9, %7 ]
  %9 = getelementptr inbounds i8, ptr addrspace(5) %8, i32 1
  %10 = load i8, ptr addrspace(5) %8, align 1, !tbaa !1
  %11 = icmp eq i8 %10, 0
  br i1 %11, label %12, label %7, !llvm.loop !4

12:                                               ; preds = %7
  %13 = addrspacecast ptr addrspace(5) %5 to ptr
  %14 = addrspacecast ptr addrspace(5) %9 to ptr
  %15 = ptrtoint ptr %14 to i64
  %16 = ptrtoint ptr %13 to i64
  %17 = sub i64 %15, %16
  %18 = shl i64 %17, 32
  %19 = ashr exact i64 %18, 32
  %20 = call i64 @__ockl_fprintf_append_string_n(i64 noundef %6, ptr noundef %13, i64 noundef %19, i32 noundef 0) #14
  br label %21

21:                                               ; preds = %21, %12
  %22 = phi ptr [ %1, %12 ], [ %23, %21 ]
  %23 = getelementptr inbounds i8, ptr %22, i64 1
  %24 = load i8, ptr %22, align 1, !tbaa !1
  %25 = icmp eq i8 %24, 0
  br i1 %25, label %26, label %21, !llvm.loop !6

26:                                               ; preds = %21
  %27 = ptrtoint ptr %23 to i64
  %28 = ptrtoint ptr %1 to i64
  %29 = sub i64 %27, %28
  %30 = shl i64 %29, 32
  %31 = ashr exact i64 %30, 32
  %32 = call i64 @__ockl_fprintf_append_string_n(i64 noundef %20, ptr noundef %1, i64 noundef %31, i32 noundef 0) #14
  %33 = zext i32 %2 to i64
  %34 = call i64 @__ockl_fprintf_append_args(i64 noundef %32, i32 noundef 1, i64 noundef %33, i64 noundef 0, i64 noundef 0, i64 noundef 0, i64 noundef 0, i64 noundef 0, i64 noundef 0, i32 noundef 0) #14
  br label %35

35:                                               ; preds = %35, %26
  %36 = phi ptr [ %3, %26 ], [ %37, %35 ]
  %37 = getelementptr inbounds i8, ptr %36, i64 1
  %38 = load i8, ptr %36, align 1, !tbaa !1
  %39 = icmp eq i8 %38, 0
  br i1 %39, label %40, label %35, !llvm.loop !7

40:                                               ; preds = %35
  %41 = ptrtoint ptr %37 to i64
  %42 = ptrtoint ptr %3 to i64
  %43 = sub i64 %41, %42
  %44 = shl i64 %43, 32
  %45 = ashr exact i64 %44, 32
  %46 = call i64 @__ockl_fprintf_append_string_n(i64 noundef %34, ptr noundef %3, i64 noundef %45, i32 noundef 0) #14
  br label %47

47:                                               ; preds = %47, %40
  %48 = phi ptr [ %0, %40 ], [ %49, %47 ]
  %49 = getelementptr inbounds i8, ptr %48, i64 1
  %50 = load i8, ptr %48, align 1, !tbaa !1
  %51 = icmp eq i8 %50, 0
  br i1 %51, label %52, label %47, !llvm.loop !8

52:                                               ; preds = %47
  %53 = ptrtoint ptr %49 to i64
  %54 = ptrtoint ptr %0 to i64
  %55 = sub i64 %53, %54
  %56 = shl i64 %55, 32
  %57 = ashr exact i64 %56, 32
  %58 = call i64 @__ockl_fprintf_append_string_n(i64 noundef %46, ptr noundef %0, i64 noundef %57, i32 noundef 1) #14
  call void @llvm.trap()
  unreachable
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p5(i64 immarg, ptr addrspace(5) nocapture) #2

; Function Attrs: convergent nounwind
declare hidden i64 @__ockl_fprintf_stderr_begin() local_unnamed_addr #3

; Function Attrs: convergent nounwind
declare hidden i64 @__ockl_fprintf_append_string_n(i64 noundef, ptr noundef, i64 noundef, i32 noundef) local_unnamed_addr #3

; Function Attrs: convergent nounwind
declare hidden i64 @__ockl_fprintf_append_args(i64 noundef, i32 noundef, i64 noundef, i64 noundef, i64 noundef, i64 noundef, i64 noundef, i64 noundef, i64 noundef, i32 noundef) local_unnamed_addr #3

; Function Attrs: convergent mustprogress nofree norecurse nounwind willreturn memory(none)
define linkonce_odr i32 @__zluda_ptx_impl_activemask() #4 {
  %1 = tail call i32 @llvm.amdgcn.ballot.i32(i1 true)
  ret i32 %1
}

; Function Attrs: convergent nocallback nofree nounwind willreturn memory(none)
declare i32 @llvm.amdgcn.ballot.i32(i1) #5

; Function Attrs: convergent mustprogress nounwind
define linkonce_odr i32 @__zluda_ptx_impl_sreg_tid(i8 noundef zeroext %0) #6 {
  %2 = zext i8 %0 to i32
  %3 = tail call i64 @__ockl_get_local_id(i32 noundef %2) #14
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

; Function Attrs: convergent nounwind
declare hidden i64 @__ockl_get_local_id(i32 noundef) local_unnamed_addr #3

; Function Attrs: convergent mustprogress nounwind
define linkonce_odr i32 @__zluda_ptx_impl_sreg_ntid(i8 noundef zeroext %0) #6 {
  %2 = zext i8 %0 to i32
  %3 = tail call i64 @__ockl_get_local_size(i32 noundef %2) #14
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

; Function Attrs: convergent nounwind
declare hidden i64 @__ockl_get_local_size(i32 noundef) local_unnamed_addr #3

; Function Attrs: convergent mustprogress nounwind
define linkonce_odr i32 @__zluda_ptx_impl_sreg_ctaid(i8 noundef zeroext %0) #6 {
  %2 = zext i8 %0 to i32
  %3 = tail call i64 @__ockl_get_group_id(i32 noundef %2) #14
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

; Function Attrs: convergent nounwind
declare hidden i64 @__ockl_get_group_id(i32 noundef) local_unnamed_addr #3

; Function Attrs: convergent mustprogress nounwind
define linkonce_odr i32 @__zluda_ptx_impl_sreg_nctaid(i8 noundef zeroext %0) #6 {
  %2 = zext i8 %0 to i32
  %3 = tail call i64 @__ockl_get_num_groups(i32 noundef %2) #14
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

; Function Attrs: convergent nounwind
declare hidden i64 @__ockl_get_num_groups(i32 noundef) local_unnamed_addr #3

; Function Attrs: convergent mustprogress nounwind
define linkonce_odr i32 @__zluda_ptx_impl_bfe_u32(i32 noundef %0, i32 noundef %1, i32 noundef %2) #6 {
  %4 = and i32 %1, 255
  %5 = and i32 %2, 255
  %6 = icmp ugt i32 %4, 31
  br i1 %6, label %13, label %7

7:                                                ; preds = %3
  %8 = icmp ugt i32 %5, 31
  br i1 %8, label %9, label %11

9:                                                ; preds = %7
  %10 = lshr i32 %0, %4
  br label %13

11:                                               ; preds = %7
  %12 = tail call i32 @__ockl_bfe_u32(i32 noundef %0, i32 noundef %4, i32 noundef %5) #14
  br label %13

13:                                               ; preds = %11, %9, %3
  %14 = phi i32 [ %10, %9 ], [ %12, %11 ], [ 0, %3 ]
  ret i32 %14
}

; Function Attrs: convergent nounwind
declare hidden i32 @__ockl_bfe_u32(i32 noundef, i32 noundef, i32 noundef) local_unnamed_addr #3

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define linkonce_odr i64 @__zluda_ptx_impl_bfe_u64(i64 noundef %0, i32 noundef %1, i32 noundef %2) #7 {
  %4 = icmp ugt i32 %1, 63
  br i1 %4, label %14, label %5

5:                                                ; preds = %3
  %6 = icmp ugt i32 %2, 63
  %7 = zext i32 %1 to i64
  %8 = lshr i64 %0, %7
  br i1 %6, label %14, label %9

9:                                                ; preds = %5
  %10 = zext i32 %2 to i64
  %11 = shl nsw i64 -1, %10
  %12 = xor i64 %11, -1
  %13 = and i64 %8, %12
  br label %14

14:                                               ; preds = %9, %5, %3
  %15 = phi i64 [ %13, %9 ], [ 0, %3 ], [ %8, %5 ]
  ret i64 %15
}

; Function Attrs: convergent mustprogress nounwind
define linkonce_odr i32 @__zluda_ptx_impl_bfe_s32(i32 noundef %0, i32 noundef %1, i32 noundef %2) #6 {
  %4 = and i32 %1, 255
  %5 = and i32 %2, 255
  %6 = icmp eq i32 %5, 0
  br i1 %6, label %17, label %7

7:                                                ; preds = %3
  %8 = icmp ugt i32 %4, 31
  br i1 %8, label %9, label %11

9:                                                ; preds = %7
  %10 = ashr i32 %0, 31
  br label %17

11:                                               ; preds = %7
  %12 = icmp ugt i32 %5, 31
  br i1 %12, label %13, label %15

13:                                               ; preds = %11
  %14 = ashr i32 %0, %4
  br label %17

15:                                               ; preds = %11
  %16 = tail call i32 @__ockl_bfe_i32(i32 noundef %0, i32 noundef %4, i32 noundef %5) #14
  br label %17

17:                                               ; preds = %15, %13, %9, %3
  %18 = phi i32 [ %10, %9 ], [ %14, %13 ], [ %16, %15 ], [ 0, %3 ]
  ret i32 %18
}

; Function Attrs: convergent nounwind
declare hidden i32 @__ockl_bfe_i32(i32 noundef, i32 noundef, i32 noundef) local_unnamed_addr #3

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define linkonce_odr i64 @__zluda_ptx_impl_bfe_s64(i64 noundef %0, i32 noundef %1, i32 noundef %2) #7 {
  %4 = icmp eq i32 %2, 0
  br i1 %4, label %21, label %5

5:                                                ; preds = %3
  %6 = icmp ugt i32 %1, 63
  br i1 %6, label %7, label %9

7:                                                ; preds = %5
  %8 = ashr i64 %0, 63
  br label %21

9:                                                ; preds = %5
  %10 = tail call noundef i32 @llvm.uadd.sat.i32(i32 %1, i32 %2)
  %11 = icmp ugt i32 %10, 63
  %12 = sub nuw nsw i32 64, %1
  %13 = select i1 %11, i32 %12, i32 %2
  %14 = add i32 %13, %1
  %15 = sub i32 64, %14
  %16 = zext i32 %15 to i64
  %17 = shl i64 %0, %16
  %18 = sub i32 64, %13
  %19 = zext i32 %18 to i64
  %20 = ashr i64 %17, %19
  br label %21

21:                                               ; preds = %9, %7, %3
  %22 = phi i64 [ %8, %7 ], [ %20, %9 ], [ 0, %3 ]
  ret i64 %22
}

; Function Attrs: convergent mustprogress nounwind
define linkonce_odr i32 @__zluda_ptx_impl_bfi_b32(i32 noundef %0, i32 noundef %1, i32 noundef %2, i32 noundef %3) #6 {
  %5 = and i32 %2, 255
  %6 = and i32 %3, 255
  %7 = icmp ugt i32 %5, 31
  br i1 %7, label %21, label %8

8:                                                ; preds = %4
  %9 = icmp ugt i32 %6, 31
  br i1 %9, label %10, label %12

10:                                               ; preds = %8
  %11 = shl nsw i32 -1, %5
  br label %14

12:                                               ; preds = %8
  %13 = tail call i32 @__ockl_bfm_u32(i32 noundef %6, i32 noundef %5) #14
  br label %14

14:                                               ; preds = %12, %10
  %15 = phi i32 [ %11, %10 ], [ %13, %12 ]
  %16 = xor i32 %15, -1
  %17 = and i32 %16, %1
  %18 = shl i32 %0, %5
  %19 = and i32 %15, %18
  %20 = or i32 %17, %19
  br label %21

21:                                               ; preds = %14, %4
  %22 = phi i32 [ %20, %14 ], [ %1, %4 ]
  ret i32 %22
}

; Function Attrs: convergent nounwind
declare hidden i32 @__ockl_bfm_u32(i32 noundef, i32 noundef) local_unnamed_addr #3

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define linkonce_odr i64 @__zluda_ptx_impl_bfi_b64(i64 noundef %0, i64 noundef %1, i32 noundef %2, i32 noundef %3) #7 {
  %5 = icmp ugt i32 %2, 63
  br i1 %5, label %21, label %6

6:                                                ; preds = %4
  %7 = icmp ugt i32 %3, 63
  br i1 %7, label %12, label %8

8:                                                ; preds = %6
  %9 = zext i32 %3 to i64
  %10 = shl nsw i64 -1, %9
  %11 = xor i64 %10, -1
  br label %12

12:                                               ; preds = %8, %6
  %13 = phi i64 [ %11, %8 ], [ -1, %6 ]
  %14 = zext i32 %2 to i64
  %15 = shl i64 %13, %14
  %16 = xor i64 %15, -1
  %17 = and i64 %16, %1
  %18 = and i64 %13, %0
  %19 = shl i64 %18, %14
  %20 = or i64 %17, %19
  br label %21

21:                                               ; preds = %12, %4
  %22 = phi i64 [ %20, %12 ], [ %1, %4 ]
  ret i64 %22
}

; Function Attrs: convergent mustprogress nofree norecurse nounwind willreturn
define linkonce_odr void @__zluda_ptx_impl_bar_sync(i32 %0) #8 {
  fence syncscope("workgroup") seq_cst
  tail call void @llvm.amdgcn.s.barrier()
  ret void
}

; Function Attrs: convergent nocallback nofree nounwind willreturn
declare void @llvm.amdgcn.s.barrier() #9

; Function Attrs: convergent mustprogress nounwind
define linkonce_odr zeroext i1 @__zluda_ptx_impl_bar_red_and(i32 %0, i1 noundef zeroext %1, i64 noundef %2) #6 {
  %4 = icmp ne i64 %2, 0
  %5 = xor i1 %4, %1
  %6 = zext i1 %5 to i32
  %7 = tail call i32 @__ockl_wgred_and_i32(i32 noundef %6) #14
  %8 = icmp ne i32 %7, 0
  ret i1 %8
}

; Function Attrs: convergent nounwind
declare hidden i32 @__ockl_wgred_and_i32(i32 noundef) local_unnamed_addr #3

; Function Attrs: convergent mustprogress nounwind
define linkonce_odr zeroext i1 @__zluda_ptx_impl_bar_red_or(i32 %0, i1 noundef zeroext %1, i64 noundef %2) #6 {
  %4 = icmp ne i64 %2, 0
  %5 = xor i1 %4, %1
  %6 = zext i1 %5 to i32
  %7 = tail call i32 @__ockl_wgred_or_i32(i32 noundef %6) #14
  %8 = icmp ne i32 %7, 0
  ret i1 %8
}

; Function Attrs: convergent nounwind
declare hidden i32 @__ockl_wgred_or_i32(i32 noundef) local_unnamed_addr #3

; Function Attrs: convergent mustprogress noreturn nounwind
define linkonce_odr void @__zluda_ptx_impl___assertfail(i64 noundef %0, i64 noundef %1, i32 noundef %2, i64 noundef %3, i64 %4) #10 {
  %6 = inttoptr i64 %0 to ptr
  %7 = inttoptr i64 %1 to ptr
  %8 = inttoptr i64 %3 to ptr
  tail call fastcc void @__assert_fail(ptr noundef %6, ptr noundef %7, i32 noundef %2, ptr noundef %8) #14
  unreachable
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.uadd.sat.i32(i32, i32) #11

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p5.p4.i64(ptr addrspace(5) noalias nocapture writeonly, ptr addrspace(4) noalias nocapture readonly, i64, i1 immarg) #12

attributes #0 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #1 = { convergent mustprogress noinline noreturn nounwind "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" }
attributes #2 = { nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #3 = { convergent nounwind "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" }
attributes #4 = { convergent mustprogress nofree norecurse nounwind willreturn memory(none) "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" }
attributes #5 = { convergent nocallback nofree nounwind willreturn memory(none) }
attributes #6 = { convergent mustprogress nounwind "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" }
attributes #7 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" }
attributes #8 = { convergent mustprogress nofree norecurse nounwind willreturn "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" }
attributes #9 = { convergent nocallback nofree nounwind willreturn }
attributes #10 = { convergent mustprogress noreturn nounwind "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" "stack-protector-buffer-size"="8" }
attributes #11 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #12 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #13 = { nounwind }
attributes #14 = { convergent nounwind }

!llvm.ident = !{!0}

!0 = !{!"AMD clang version 19.0.0git (https://github.com/RadeonOpenCompute/llvm-project roc-6.4.0 25133 c7fe45cf4b819c5991fe208aaa96edf142730f1d)"}
!1 = !{!2, !2, i64 0}
!2 = !{!"omnipotent char", !3, i64 0}
!3 = !{!"Simple C++ TBAA"}
!4 = distinct !{!4, !5}
!5 = !{!"llvm.loop.mustprogress"}
!6 = distinct !{!6, !5}
!7 = distinct !{!7, !5}
!8 = distinct !{!8, !5}
