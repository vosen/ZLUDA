// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
impl crate::CudaDisplay for cuda_types::cufft::libFormat_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cufft::libFormat_t::LIB_FORMAT_CUFFT => {
                writer.write_all(stringify!(LIB_FORMAT_CUFFT).as_bytes())
            }
            &cuda_types::cufft::libFormat_t::LIB_FORMAT_UNDEFINED => {
                writer.write_all(stringify!(LIB_FORMAT_UNDEFINED).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cudaXtDesc_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(nGPUs), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.nGPUs, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(GPUs), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.GPUs, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(data), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.data, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(size), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.size, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(cudaXtState), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.cudaXtState, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cudaLibXtDesc_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(descriptor), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.descriptor, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(library), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.library, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(subFormat), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.subFormat, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(libDescriptor), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.libDescriptor, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cufft::cufftType_t::CUFFT_R2C => {
                writer.write_all(stringify!(CUFFT_R2C).as_bytes())
            }
            &cuda_types::cufft::cufftType_t::CUFFT_C2R => {
                writer.write_all(stringify!(CUFFT_C2R).as_bytes())
            }
            &cuda_types::cufft::cufftType_t::CUFFT_C2C => {
                writer.write_all(stringify!(CUFFT_C2C).as_bytes())
            }
            &cuda_types::cufft::cufftType_t::CUFFT_D2Z => {
                writer.write_all(stringify!(CUFFT_D2Z).as_bytes())
            }
            &cuda_types::cufft::cufftType_t::CUFFT_Z2D => {
                writer.write_all(stringify!(CUFFT_Z2D).as_bytes())
            }
            &cuda_types::cufft::cufftType_t::CUFFT_Z2Z => {
                writer.write_all(stringify!(CUFFT_Z2Z).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftCompatibility_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cufft::cufftCompatibility_t::CUFFT_COMPATIBILITY_FFTW_PADDING => {
                writer.write_all(stringify!(CUFFT_COMPATIBILITY_FFTW_PADDING).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cufftPlan1d(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: *mut cuda_types::cufft::cufftHandle,
    nx: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
    batch: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftPlan1d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nx, "cufftPlan1d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftPlan1d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(batch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&batch, "cufftPlan1d", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftPlan2d(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: *mut cuda_types::cufft::cufftHandle,
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftPlan2d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nx, "cufftPlan2d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ny), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ny, "cufftPlan2d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftPlan2d", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftPlan3d(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: *mut cuda_types::cufft::cufftHandle,
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    nz: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftPlan3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nx, "cufftPlan3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ny), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ny, "cufftPlan3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nz), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nz, "cufftPlan3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftPlan3d", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftPlanMany(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: *mut cuda_types::cufft::cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_int,
    inembed: *mut ::core::ffi::c_int,
    istride: ::core::ffi::c_int,
    idist: ::core::ffi::c_int,
    onembed: *mut ::core::ffi::c_int,
    ostride: ::core::ffi::c_int,
    odist: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
    batch: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftPlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rank), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rank, "cufftPlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(n), ": ").as_bytes())?;
    crate::CudaDisplay::write(&n, "cufftPlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inembed, "cufftPlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(istride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&istride, "cufftPlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(idist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&idist, "cufftPlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(onembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&onembed, "cufftPlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ostride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ostride, "cufftPlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(odist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&odist, "cufftPlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftPlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(batch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&batch, "cufftPlanMany", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftMakePlan1d(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    nx: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
    batch: ::core::ffi::c_int,
    workSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftMakePlan1d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nx, "cufftMakePlan1d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftMakePlan1d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(batch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&batch, "cufftMakePlan1d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftMakePlan1d", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftMakePlan2d(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
    workSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftMakePlan2d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nx, "cufftMakePlan2d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ny), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ny, "cufftMakePlan2d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftMakePlan2d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftMakePlan2d", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftMakePlan3d(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    nz: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
    workSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftMakePlan3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nx, "cufftMakePlan3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ny), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ny, "cufftMakePlan3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nz), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nz, "cufftMakePlan3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftMakePlan3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftMakePlan3d", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftMakePlanMany(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_int,
    inembed: *mut ::core::ffi::c_int,
    istride: ::core::ffi::c_int,
    idist: ::core::ffi::c_int,
    onembed: *mut ::core::ffi::c_int,
    ostride: ::core::ffi::c_int,
    odist: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
    batch: ::core::ffi::c_int,
    workSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rank), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rank, "cufftMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(n), ": ").as_bytes())?;
    crate::CudaDisplay::write(&n, "cufftMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inembed, "cufftMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(istride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&istride, "cufftMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(idist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&idist, "cufftMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(onembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&onembed, "cufftMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ostride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ostride, "cufftMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(odist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&odist, "cufftMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(batch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&batch, "cufftMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftMakePlanMany", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftMakePlanMany64(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_longlong,
    inembed: *mut ::core::ffi::c_longlong,
    istride: ::core::ffi::c_longlong,
    idist: ::core::ffi::c_longlong,
    onembed: *mut ::core::ffi::c_longlong,
    ostride: ::core::ffi::c_longlong,
    odist: ::core::ffi::c_longlong,
    type_: cuda_types::cufft::cufftType,
    batch: ::core::ffi::c_longlong,
    workSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftMakePlanMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rank), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rank, "cufftMakePlanMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(n), ": ").as_bytes())?;
    crate::CudaDisplay::write(&n, "cufftMakePlanMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inembed, "cufftMakePlanMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(istride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&istride, "cufftMakePlanMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(idist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&idist, "cufftMakePlanMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(onembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&onembed, "cufftMakePlanMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ostride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ostride, "cufftMakePlanMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(odist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&odist, "cufftMakePlanMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftMakePlanMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(batch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&batch, "cufftMakePlanMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftMakePlanMany64", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftGetSizeMany64(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_longlong,
    inembed: *mut ::core::ffi::c_longlong,
    istride: ::core::ffi::c_longlong,
    idist: ::core::ffi::c_longlong,
    onembed: *mut ::core::ffi::c_longlong,
    ostride: ::core::ffi::c_longlong,
    odist: ::core::ffi::c_longlong,
    type_: cuda_types::cufft::cufftType,
    batch: ::core::ffi::c_longlong,
    workSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftGetSizeMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rank), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rank, "cufftGetSizeMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(n), ": ").as_bytes())?;
    crate::CudaDisplay::write(&n, "cufftGetSizeMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inembed, "cufftGetSizeMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(istride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&istride, "cufftGetSizeMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(idist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&idist, "cufftGetSizeMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(onembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&onembed, "cufftGetSizeMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ostride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ostride, "cufftGetSizeMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(odist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&odist, "cufftGetSizeMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftGetSizeMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(batch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&batch, "cufftGetSizeMany64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftGetSizeMany64", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftEstimate1d(
    writer: &mut (impl std::io::Write + ?Sized),
    nx: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
    batch: ::core::ffi::c_int,
    workSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(nx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nx, "cufftEstimate1d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftEstimate1d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(batch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&batch, "cufftEstimate1d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftEstimate1d", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftEstimate2d(
    writer: &mut (impl std::io::Write + ?Sized),
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
    workSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(nx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nx, "cufftEstimate2d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ny), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ny, "cufftEstimate2d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftEstimate2d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftEstimate2d", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftEstimate3d(
    writer: &mut (impl std::io::Write + ?Sized),
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    nz: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
    workSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(nx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nx, "cufftEstimate3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ny), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ny, "cufftEstimate3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nz), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nz, "cufftEstimate3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftEstimate3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftEstimate3d", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftEstimateMany(
    writer: &mut (impl std::io::Write + ?Sized),
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_int,
    inembed: *mut ::core::ffi::c_int,
    istride: ::core::ffi::c_int,
    idist: ::core::ffi::c_int,
    onembed: *mut ::core::ffi::c_int,
    ostride: ::core::ffi::c_int,
    odist: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
    batch: ::core::ffi::c_int,
    workSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rank), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rank, "cufftEstimateMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(n), ": ").as_bytes())?;
    crate::CudaDisplay::write(&n, "cufftEstimateMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inembed, "cufftEstimateMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(istride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&istride, "cufftEstimateMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(idist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&idist, "cufftEstimateMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(onembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&onembed, "cufftEstimateMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ostride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ostride, "cufftEstimateMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(odist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&odist, "cufftEstimateMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftEstimateMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(batch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&batch, "cufftEstimateMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftEstimateMany", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: *mut cuda_types::cufft::cufftHandle,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cufftCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftGetSize1d(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cufft::cufftHandle,
    nx: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
    batch: ::core::ffi::c_int,
    workSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cufftGetSize1d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nx, "cufftGetSize1d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftGetSize1d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(batch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&batch, "cufftGetSize1d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftGetSize1d", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftGetSize2d(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cufft::cufftHandle,
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
    workSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cufftGetSize2d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nx, "cufftGetSize2d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ny), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ny, "cufftGetSize2d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftGetSize2d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftGetSize2d", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftGetSize3d(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cufft::cufftHandle,
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    nz: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
    workSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cufftGetSize3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nx, "cufftGetSize3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ny), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ny, "cufftGetSize3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nz), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nz, "cufftGetSize3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftGetSize3d", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftGetSize3d", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftGetSizeMany(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cufft::cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_int,
    inembed: *mut ::core::ffi::c_int,
    istride: ::core::ffi::c_int,
    idist: ::core::ffi::c_int,
    onembed: *mut ::core::ffi::c_int,
    ostride: ::core::ffi::c_int,
    odist: ::core::ffi::c_int,
    type_: cuda_types::cufft::cufftType,
    batch: ::core::ffi::c_int,
    workArea: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cufftGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rank), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rank, "cufftGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(n), ": ").as_bytes())?;
    crate::CudaDisplay::write(&n, "cufftGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inembed, "cufftGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(istride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&istride, "cufftGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(idist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&idist, "cufftGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(onembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&onembed, "cufftGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ostride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ostride, "cufftGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(odist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&odist, "cufftGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(batch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&batch, "cufftGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workArea), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workArea, "cufftGetSizeMany", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftGetSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cufft::cufftHandle,
    workSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cufftGetSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftGetSize", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftSetWorkArea(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    workArea: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftSetWorkArea", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workArea), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workArea, "cufftSetWorkArea", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftSetAutoAllocation(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    autoAllocate: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftSetAutoAllocation", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(autoAllocate), ": ").as_bytes())?;
    crate::CudaDisplay::write(&autoAllocate, "cufftSetAutoAllocation", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftExecC2C(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    idata: *mut cuda_types::cufft::cufftComplex,
    odata: *mut cuda_types::cufft::cufftComplex,
    direction: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftExecC2C", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(idata), ": ").as_bytes())?;
    crate::CudaDisplay::write(&idata, "cufftExecC2C", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(odata), ": ").as_bytes())?;
    crate::CudaDisplay::write(&odata, "cufftExecC2C", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(direction), ": ").as_bytes())?;
    crate::CudaDisplay::write(&direction, "cufftExecC2C", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftExecR2C(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    idata: *mut cuda_types::cufft::cufftReal,
    odata: *mut cuda_types::cufft::cufftComplex,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftExecR2C", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(idata), ": ").as_bytes())?;
    crate::CudaDisplay::write(&idata, "cufftExecR2C", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(odata), ": ").as_bytes())?;
    crate::CudaDisplay::write(&odata, "cufftExecR2C", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftExecC2R(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    idata: *mut cuda_types::cufft::cufftComplex,
    odata: *mut cuda_types::cufft::cufftReal,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftExecC2R", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(idata), ": ").as_bytes())?;
    crate::CudaDisplay::write(&idata, "cufftExecC2R", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(odata), ": ").as_bytes())?;
    crate::CudaDisplay::write(&odata, "cufftExecC2R", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftExecZ2Z(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    idata: *mut cuda_types::cufft::cufftDoubleComplex,
    odata: *mut cuda_types::cufft::cufftDoubleComplex,
    direction: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftExecZ2Z", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(idata), ": ").as_bytes())?;
    crate::CudaDisplay::write(&idata, "cufftExecZ2Z", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(odata), ": ").as_bytes())?;
    crate::CudaDisplay::write(&odata, "cufftExecZ2Z", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(direction), ": ").as_bytes())?;
    crate::CudaDisplay::write(&direction, "cufftExecZ2Z", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftExecD2Z(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    idata: *mut cuda_types::cufft::cufftDoubleReal,
    odata: *mut cuda_types::cufft::cufftDoubleComplex,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftExecD2Z", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(idata), ": ").as_bytes())?;
    crate::CudaDisplay::write(&idata, "cufftExecD2Z", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(odata), ": ").as_bytes())?;
    crate::CudaDisplay::write(&odata, "cufftExecD2Z", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftExecZ2D(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    idata: *mut cuda_types::cufft::cufftDoubleComplex,
    odata: *mut cuda_types::cufft::cufftDoubleReal,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftExecZ2D", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(idata), ": ").as_bytes())?;
    crate::CudaDisplay::write(&idata, "cufftExecZ2D", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(odata), ": ").as_bytes())?;
    crate::CudaDisplay::write(&odata, "cufftExecZ2D", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftSetStream(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    stream: cuda_types::cufft::cudaStream_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftSetStream", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cufftSetStream", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftGetVersion(
    writer: &mut (impl std::io::Write + ?Sized),
    version: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(version), ": ").as_bytes())?;
    crate::CudaDisplay::write(&version, "cufftGetVersion", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftGetProperty(
    writer: &mut (impl std::io::Write + ?Sized),
    type_: cuda_types::cufft::libraryPropertyType,
    value: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftGetProperty", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cufftGetProperty", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cufft::cufftProperty_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cufft::cufftProperty_t::NVFFT_PLAN_PROPERTY_INT64_PATIENT_JIT => {
                writer
                    .write_all(
                        stringify!(NVFFT_PLAN_PROPERTY_INT64_PATIENT_JIT).as_bytes(),
                    )
            }
            &cuda_types::cufft::cufftProperty_t::NVFFT_PLAN_PROPERTY_INT64_MAX_NUM_HOST_THREADS => {
                writer
                    .write_all(
                        stringify!(NVFFT_PLAN_PROPERTY_INT64_MAX_NUM_HOST_THREADS)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cufftSetPlanPropertyInt64(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    property: cuda_types::cufft::cufftProperty,
    inputValueInt: ::core::ffi::c_longlong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftSetPlanPropertyInt64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(property), ": ").as_bytes())?;
    crate::CudaDisplay::write(&property, "cufftSetPlanPropertyInt64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inputValueInt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &inputValueInt,
        "cufftSetPlanPropertyInt64",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cufftGetPlanPropertyInt64(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    property: cuda_types::cufft::cufftProperty,
    returnPtrValue: *mut ::core::ffi::c_longlong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftGetPlanPropertyInt64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(property), ": ").as_bytes())?;
    crate::CudaDisplay::write(&property, "cufftGetPlanPropertyInt64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnPtrValue), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnPtrValue,
        "cufftGetPlanPropertyInt64",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cufftResetPlanProperty(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    property: cuda_types::cufft::cufftProperty,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftResetPlanProperty", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(property), ": ").as_bytes())?;
    crate::CudaDisplay::write(&property, "cufftResetPlanProperty", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cufft::cufftXtSubFormat_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cufft::cufftXtSubFormat_t::CUFFT_XT_FORMAT_INPUT => {
                writer.write_all(stringify!(CUFFT_XT_FORMAT_INPUT).as_bytes())
            }
            &cuda_types::cufft::cufftXtSubFormat_t::CUFFT_XT_FORMAT_OUTPUT => {
                writer.write_all(stringify!(CUFFT_XT_FORMAT_OUTPUT).as_bytes())
            }
            &cuda_types::cufft::cufftXtSubFormat_t::CUFFT_XT_FORMAT_INPLACE => {
                writer.write_all(stringify!(CUFFT_XT_FORMAT_INPLACE).as_bytes())
            }
            &cuda_types::cufft::cufftXtSubFormat_t::CUFFT_XT_FORMAT_INPLACE_SHUFFLED => {
                writer.write_all(stringify!(CUFFT_XT_FORMAT_INPLACE_SHUFFLED).as_bytes())
            }
            &cuda_types::cufft::cufftXtSubFormat_t::CUFFT_XT_FORMAT_1D_INPUT_SHUFFLED => {
                writer
                    .write_all(stringify!(CUFFT_XT_FORMAT_1D_INPUT_SHUFFLED).as_bytes())
            }
            &cuda_types::cufft::cufftXtSubFormat_t::CUFFT_XT_FORMAT_DISTRIBUTED_INPUT => {
                writer
                    .write_all(stringify!(CUFFT_XT_FORMAT_DISTRIBUTED_INPUT).as_bytes())
            }
            &cuda_types::cufft::cufftXtSubFormat_t::CUFFT_XT_FORMAT_DISTRIBUTED_OUTPUT => {
                writer
                    .write_all(stringify!(CUFFT_XT_FORMAT_DISTRIBUTED_OUTPUT).as_bytes())
            }
            &cuda_types::cufft::cufftXtSubFormat_t::CUFFT_FORMAT_UNDEFINED => {
                writer.write_all(stringify!(CUFFT_FORMAT_UNDEFINED).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftXtCopyType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cufft::cufftXtCopyType_t::CUFFT_COPY_HOST_TO_DEVICE => {
                writer.write_all(stringify!(CUFFT_COPY_HOST_TO_DEVICE).as_bytes())
            }
            &cuda_types::cufft::cufftXtCopyType_t::CUFFT_COPY_DEVICE_TO_HOST => {
                writer.write_all(stringify!(CUFFT_COPY_DEVICE_TO_HOST).as_bytes())
            }
            &cuda_types::cufft::cufftXtCopyType_t::CUFFT_COPY_DEVICE_TO_DEVICE => {
                writer.write_all(stringify!(CUFFT_COPY_DEVICE_TO_DEVICE).as_bytes())
            }
            &cuda_types::cufft::cufftXtCopyType_t::CUFFT_COPY_UNDEFINED => {
                writer.write_all(stringify!(CUFFT_COPY_UNDEFINED).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftXtQueryType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cufft::cufftXtQueryType_t::CUFFT_QUERY_1D_FACTORS => {
                writer.write_all(stringify!(CUFFT_QUERY_1D_FACTORS).as_bytes())
            }
            &cuda_types::cufft::cufftXtQueryType_t::CUFFT_QUERY_UNDEFINED => {
                writer.write_all(stringify!(CUFFT_QUERY_UNDEFINED).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftXt1dFactors_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(size), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.size, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(stringCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.stringCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(stringLength), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.stringLength, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(substringLength), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.substringLength, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(factor1), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.factor1, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(factor2), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.factor2, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(stringMask), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.stringMask, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(substringMask), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.substringMask, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(factor1Mask), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.factor1Mask, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(factor2Mask), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.factor2Mask, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(stringShift), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.stringShift, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(substringShift), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.substringShift, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(factor1Shift), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.factor1Shift, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(factor2Shift), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.factor2Shift, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftXtWorkAreaPolicy_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cufft::cufftXtWorkAreaPolicy_t::CUFFT_WORKAREA_MINIMAL => {
                writer.write_all(stringify!(CUFFT_WORKAREA_MINIMAL).as_bytes())
            }
            &cuda_types::cufft::cufftXtWorkAreaPolicy_t::CUFFT_WORKAREA_USER => {
                writer.write_all(stringify!(CUFFT_WORKAREA_USER).as_bytes())
            }
            &cuda_types::cufft::cufftXtWorkAreaPolicy_t::CUFFT_WORKAREA_PERFORMANCE => {
                writer.write_all(stringify!(CUFFT_WORKAREA_PERFORMANCE).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cufftXtSetGPUs(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cufft::cufftHandle,
    nGPUs: ::core::ffi::c_int,
    whichGPUs: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cufftXtSetGPUs", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nGPUs), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nGPUs, "cufftXtSetGPUs", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(whichGPUs), ": ").as_bytes())?;
    crate::CudaDisplay::write(&whichGPUs, "cufftXtSetGPUs", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtMalloc(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    descriptor: *mut *mut cuda_types::cufft::cudaLibXtDesc,
    format: cuda_types::cufft::cufftXtSubFormat,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtMalloc", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(descriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(&descriptor, "cufftXtMalloc", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(format), ": ").as_bytes())?;
    crate::CudaDisplay::write(&format, "cufftXtMalloc", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtMemcpy(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    dstPointer: *mut ::core::ffi::c_void,
    srcPointer: *mut ::core::ffi::c_void,
    type_: cuda_types::cufft::cufftXtCopyType,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtMemcpy", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPointer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPointer, "cufftXtMemcpy", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcPointer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcPointer, "cufftXtMemcpy", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftXtMemcpy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtFree(
    writer: &mut (impl std::io::Write + ?Sized),
    descriptor: *mut cuda_types::cufft::cudaLibXtDesc,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(descriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(&descriptor, "cufftXtFree", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtSetWorkArea(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    workArea: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtSetWorkArea", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workArea), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workArea, "cufftXtSetWorkArea", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtExecDescriptorC2C(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    input: *mut cuda_types::cufft::cudaLibXtDesc,
    output: *mut cuda_types::cufft::cudaLibXtDesc,
    direction: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtExecDescriptorC2C", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(input), ": ").as_bytes())?;
    crate::CudaDisplay::write(&input, "cufftXtExecDescriptorC2C", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(output), ": ").as_bytes())?;
    crate::CudaDisplay::write(&output, "cufftXtExecDescriptorC2C", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(direction), ": ").as_bytes())?;
    crate::CudaDisplay::write(&direction, "cufftXtExecDescriptorC2C", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtExecDescriptorR2C(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    input: *mut cuda_types::cufft::cudaLibXtDesc,
    output: *mut cuda_types::cufft::cudaLibXtDesc,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtExecDescriptorR2C", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(input), ": ").as_bytes())?;
    crate::CudaDisplay::write(&input, "cufftXtExecDescriptorR2C", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(output), ": ").as_bytes())?;
    crate::CudaDisplay::write(&output, "cufftXtExecDescriptorR2C", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtExecDescriptorC2R(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    input: *mut cuda_types::cufft::cudaLibXtDesc,
    output: *mut cuda_types::cufft::cudaLibXtDesc,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtExecDescriptorC2R", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(input), ": ").as_bytes())?;
    crate::CudaDisplay::write(&input, "cufftXtExecDescriptorC2R", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(output), ": ").as_bytes())?;
    crate::CudaDisplay::write(&output, "cufftXtExecDescriptorC2R", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtExecDescriptorZ2Z(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    input: *mut cuda_types::cufft::cudaLibXtDesc,
    output: *mut cuda_types::cufft::cudaLibXtDesc,
    direction: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtExecDescriptorZ2Z", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(input), ": ").as_bytes())?;
    crate::CudaDisplay::write(&input, "cufftXtExecDescriptorZ2Z", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(output), ": ").as_bytes())?;
    crate::CudaDisplay::write(&output, "cufftXtExecDescriptorZ2Z", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(direction), ": ").as_bytes())?;
    crate::CudaDisplay::write(&direction, "cufftXtExecDescriptorZ2Z", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtExecDescriptorD2Z(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    input: *mut cuda_types::cufft::cudaLibXtDesc,
    output: *mut cuda_types::cufft::cudaLibXtDesc,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtExecDescriptorD2Z", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(input), ": ").as_bytes())?;
    crate::CudaDisplay::write(&input, "cufftXtExecDescriptorD2Z", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(output), ": ").as_bytes())?;
    crate::CudaDisplay::write(&output, "cufftXtExecDescriptorD2Z", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtExecDescriptorZ2D(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    input: *mut cuda_types::cufft::cudaLibXtDesc,
    output: *mut cuda_types::cufft::cudaLibXtDesc,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtExecDescriptorZ2D", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(input), ": ").as_bytes())?;
    crate::CudaDisplay::write(&input, "cufftXtExecDescriptorZ2D", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(output), ": ").as_bytes())?;
    crate::CudaDisplay::write(&output, "cufftXtExecDescriptorZ2D", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtQueryPlan(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    queryStruct: *mut ::core::ffi::c_void,
    queryType: cuda_types::cufft::cufftXtQueryType,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtQueryPlan", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(queryStruct), ": ").as_bytes())?;
    crate::CudaDisplay::write(&queryStruct, "cufftXtQueryPlan", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(queryType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&queryType, "cufftXtQueryPlan", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cufft::cufftXtCallbackType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cufft::cufftXtCallbackType_t::CUFFT_CB_LD_COMPLEX => {
                writer.write_all(stringify!(CUFFT_CB_LD_COMPLEX).as_bytes())
            }
            &cuda_types::cufft::cufftXtCallbackType_t::CUFFT_CB_LD_COMPLEX_DOUBLE => {
                writer.write_all(stringify!(CUFFT_CB_LD_COMPLEX_DOUBLE).as_bytes())
            }
            &cuda_types::cufft::cufftXtCallbackType_t::CUFFT_CB_LD_REAL => {
                writer.write_all(stringify!(CUFFT_CB_LD_REAL).as_bytes())
            }
            &cuda_types::cufft::cufftXtCallbackType_t::CUFFT_CB_LD_REAL_DOUBLE => {
                writer.write_all(stringify!(CUFFT_CB_LD_REAL_DOUBLE).as_bytes())
            }
            &cuda_types::cufft::cufftXtCallbackType_t::CUFFT_CB_ST_COMPLEX => {
                writer.write_all(stringify!(CUFFT_CB_ST_COMPLEX).as_bytes())
            }
            &cuda_types::cufft::cufftXtCallbackType_t::CUFFT_CB_ST_COMPLEX_DOUBLE => {
                writer.write_all(stringify!(CUFFT_CB_ST_COMPLEX_DOUBLE).as_bytes())
            }
            &cuda_types::cufft::cufftXtCallbackType_t::CUFFT_CB_ST_REAL => {
                writer.write_all(stringify!(CUFFT_CB_ST_REAL).as_bytes())
            }
            &cuda_types::cufft::cufftXtCallbackType_t::CUFFT_CB_ST_REAL_DOUBLE => {
                writer.write_all(stringify!(CUFFT_CB_ST_REAL_DOUBLE).as_bytes())
            }
            &cuda_types::cufft::cufftXtCallbackType_t::CUFFT_CB_UNDEFINED => {
                writer.write_all(stringify!(CUFFT_CB_UNDEFINED).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftCallbackLoadC {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftCallbackLoadC,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftCallbackLoadZ {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftCallbackLoadZ,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftCallbackLoadR {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftCallbackLoadR,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftCallbackLoadD {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftCallbackLoadD,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftCallbackStoreC {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftCallbackStoreC,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftCallbackStoreZ {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftCallbackStoreZ,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftCallbackStoreR {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftCallbackStoreR,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftCallbackStoreD {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftCallbackStoreD,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftJITCallbackLoadC {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftJITCallbackLoadC,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftJITCallbackLoadZ {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftJITCallbackLoadZ,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftJITCallbackLoadR {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftJITCallbackLoadR,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftJITCallbackLoadD {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftJITCallbackLoadD,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftJITCallbackStoreC {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftJITCallbackStoreC,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftJITCallbackStoreZ {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftJITCallbackStoreZ,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftJITCallbackStoreR {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftJITCallbackStoreR,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cufft::cufftJITCallbackStoreD {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cufft::cufftJITCallbackStoreD,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
pub fn write_cufftXtSetCallback(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    callback_routine: *mut *mut ::core::ffi::c_void,
    cbType: cuda_types::cufft::cufftXtCallbackType,
    caller_info: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtSetCallback", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(callback_routine), ": ").as_bytes())?;
    crate::CudaDisplay::write(&callback_routine, "cufftXtSetCallback", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cbType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cbType, "cufftXtSetCallback", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(caller_info), ": ").as_bytes())?;
    crate::CudaDisplay::write(&caller_info, "cufftXtSetCallback", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtClearCallback(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    cbType: cuda_types::cufft::cufftXtCallbackType,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtClearCallback", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cbType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cbType, "cufftXtClearCallback", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtSetCallbackSharedSize(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    cbType: cuda_types::cufft::cufftXtCallbackType,
    sharedSize: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtSetCallbackSharedSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cbType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cbType, "cufftXtSetCallbackSharedSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sharedSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sharedSize,
        "cufftXtSetCallbackSharedSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cufftXtSetJITCallback(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    lto_callback_symbol_name: *const ::core::ffi::c_char,
    lto_callback_fatbin: *const ::core::ffi::c_void,
    lto_callback_fatbin_size: usize,
    type_: cuda_types::cufft::cufftXtCallbackType,
    caller_info: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtSetJITCallback", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lto_callback_symbol_name), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &lto_callback_symbol_name,
        "cufftXtSetJITCallback",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lto_callback_fatbin), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &lto_callback_fatbin,
        "cufftXtSetJITCallback",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lto_callback_fatbin_size), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &lto_callback_fatbin_size,
        "cufftXtSetJITCallback",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cufftXtSetJITCallback", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(caller_info), ": ").as_bytes())?;
    crate::CudaDisplay::write(&caller_info, "cufftXtSetJITCallback", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtMakePlanMany(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_longlong,
    inembed: *mut ::core::ffi::c_longlong,
    istride: ::core::ffi::c_longlong,
    idist: ::core::ffi::c_longlong,
    inputtype: cuda_types::cufft::cudaDataType,
    onembed: *mut ::core::ffi::c_longlong,
    ostride: ::core::ffi::c_longlong,
    odist: ::core::ffi::c_longlong,
    outputtype: cuda_types::cufft::cudaDataType,
    batch: ::core::ffi::c_longlong,
    workSize: *mut usize,
    executiontype: cuda_types::cufft::cudaDataType,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rank), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rank, "cufftXtMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(n), ": ").as_bytes())?;
    crate::CudaDisplay::write(&n, "cufftXtMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inembed, "cufftXtMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(istride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&istride, "cufftXtMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(idist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&idist, "cufftXtMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inputtype), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inputtype, "cufftXtMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(onembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&onembed, "cufftXtMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ostride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ostride, "cufftXtMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(odist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&odist, "cufftXtMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(outputtype), ": ").as_bytes())?;
    crate::CudaDisplay::write(&outputtype, "cufftXtMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(batch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&batch, "cufftXtMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftXtMakePlanMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(executiontype), ": ").as_bytes())?;
    crate::CudaDisplay::write(&executiontype, "cufftXtMakePlanMany", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtGetSizeMany(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_longlong,
    inembed: *mut ::core::ffi::c_longlong,
    istride: ::core::ffi::c_longlong,
    idist: ::core::ffi::c_longlong,
    inputtype: cuda_types::cufft::cudaDataType,
    onembed: *mut ::core::ffi::c_longlong,
    ostride: ::core::ffi::c_longlong,
    odist: ::core::ffi::c_longlong,
    outputtype: cuda_types::cufft::cudaDataType,
    batch: ::core::ffi::c_longlong,
    workSize: *mut usize,
    executiontype: cuda_types::cufft::cudaDataType,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rank), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rank, "cufftXtGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(n), ": ").as_bytes())?;
    crate::CudaDisplay::write(&n, "cufftXtGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inembed, "cufftXtGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(istride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&istride, "cufftXtGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(idist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&idist, "cufftXtGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inputtype), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inputtype, "cufftXtGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(onembed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&onembed, "cufftXtGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ostride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ostride, "cufftXtGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(odist), ": ").as_bytes())?;
    crate::CudaDisplay::write(&odist, "cufftXtGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(outputtype), ": ").as_bytes())?;
    crate::CudaDisplay::write(&outputtype, "cufftXtGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(batch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&batch, "cufftXtGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftXtGetSizeMany", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(executiontype), ": ").as_bytes())?;
    crate::CudaDisplay::write(&executiontype, "cufftXtGetSizeMany", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtExec(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    input: *mut ::core::ffi::c_void,
    output: *mut ::core::ffi::c_void,
    direction: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtExec", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(input), ": ").as_bytes())?;
    crate::CudaDisplay::write(&input, "cufftXtExec", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(output), ": ").as_bytes())?;
    crate::CudaDisplay::write(&output, "cufftXtExec", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(direction), ": ").as_bytes())?;
    crate::CudaDisplay::write(&direction, "cufftXtExec", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtExecDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    input: *mut cuda_types::cufft::cudaLibXtDesc,
    output: *mut cuda_types::cufft::cudaLibXtDesc,
    direction: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtExecDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(input), ": ").as_bytes())?;
    crate::CudaDisplay::write(&input, "cufftXtExecDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(output), ": ").as_bytes())?;
    crate::CudaDisplay::write(&output, "cufftXtExecDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(direction), ": ").as_bytes())?;
    crate::CudaDisplay::write(&direction, "cufftXtExecDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cufftXtSetWorkAreaPolicy(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cufft::cufftHandle,
    policy: cuda_types::cufft::cufftXtWorkAreaPolicy,
    workSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cufftXtSetWorkAreaPolicy", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(policy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&policy, "cufftXtSetWorkAreaPolicy", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSize, "cufftXtSetWorkAreaPolicy", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cufft::cufftResult {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            Ok(()) => writer.write_all(b"CUFFT_SUCCESS"),
            Err(err) => {
                match err.0.get() {
                    1 => writer.write_all("CUFFT_INVALID_PLAN".as_bytes()),
                    2 => writer.write_all("CUFFT_ALLOC_FAILED".as_bytes()),
                    3 => writer.write_all("CUFFT_INVALID_TYPE".as_bytes()),
                    4 => writer.write_all("CUFFT_INVALID_VALUE".as_bytes()),
                    5 => writer.write_all("CUFFT_INTERNAL_ERROR".as_bytes()),
                    6 => writer.write_all("CUFFT_EXEC_FAILED".as_bytes()),
                    7 => writer.write_all("CUFFT_SETUP_FAILED".as_bytes()),
                    8 => writer.write_all("CUFFT_INVALID_SIZE".as_bytes()),
                    9 => writer.write_all("CUFFT_UNALIGNED_DATA".as_bytes()),
                    11 => writer.write_all("CUFFT_INVALID_DEVICE".as_bytes()),
                    13 => writer.write_all("CUFFT_NO_WORKSPACE".as_bytes()),
                    14 => writer.write_all("CUFFT_NOT_IMPLEMENTED".as_bytes()),
                    16 => writer.write_all("CUFFT_NOT_SUPPORTED".as_bytes()),
                    17 => writer.write_all("CUFFT_MISSING_DEPENDENCY".as_bytes()),
                    18 => writer.write_all("CUFFT_NVRTC_FAILURE".as_bytes()),
                    19 => writer.write_all("CUFFT_NVJITLINK_FAILURE".as_bytes()),
                    20 => writer.write_all("CUFFT_NVSHMEM_FAILURE".as_bytes()),
                    err => write!(writer, "{}", err),
                }
            }
        }
    }
}
