; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define linkonce_odr noundef float @_Z31__zluda_ptx_impl_rcp_approx_f32f(float noundef %0) #0 {
  %2 = tail call i1 @llvm.is.fpclass.f32(float %0, i32 144)
  %3 = select i1 %2, float 0x4170000000000000, float 1.000000e+00
  %4 = tail call contract noundef float @llvm.fabs.f32(float %0)
  %5 = fcmp contract ult float %4, 0x47D0000020000000
  %6 = select i1 %5, float %3, float 0x3F9E17B860000000
  %7 = call float @llvm.experimental.constrained.fmul.f32(float %6, float %0, metadata !"round.dynamic", metadata !"fpexcept.ignore")
  %8 = tail call contract float @llvm.amdgcn.constrained.rcp.f32(float %7)
  %9 = call float @llvm.experimental.constrained.fmul.f32(float %6, float %8, metadata !"round.dynamic", metadata !"fpexcept.ignore")
  ret float %9
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i1 @llvm.is.fpclass.f32(float, i32 immarg) #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.amdgcn.constrained.rcp.f32(float) #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.fabs.f32(float) #1

declare float @llvm.experimental.constrained.fmul.f32(float, float, metadata, metadata)

attributes #0 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) "denormal-fp-math"="dynamic,dynamic" "no-trapping-math"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
