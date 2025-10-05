// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtHandle_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}
pub fn write_cublasLtCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    lightHandle: *mut cuda_types::cublaslt::cublasLtHandle_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(lightHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lightHandle, "cublasLtCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    lightHandle: cuda_types::cublaslt::cublasLtHandle_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(lightHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lightHandle, "cublasLtDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtGetStatusName(
    writer: &mut (impl std::io::Write + ?Sized),
    status: cuda_types::cublas::cublasStatus_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(status), ": ").as_bytes())?;
    crate::CudaDisplay::write(&status, "cublasLtGetStatusName", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtGetStatusString(
    writer: &mut (impl std::io::Write + ?Sized),
    status: cuda_types::cublas::cublasStatus_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(status), ": ").as_bytes())?;
    crate::CudaDisplay::write(&status, "cublasLtGetStatusString", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtGetVersion(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cublasLtGetCudartVersion(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cublasLtGetProperty(
    writer: &mut (impl std::io::Write + ?Sized),
    type_: cuda_types::cublaslt::libraryPropertyType,
    value: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cublasLtGetProperty", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cublasLtGetProperty", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtHeuristicsCacheGetCapacity(
    writer: &mut (impl std::io::Write + ?Sized),
    capacity: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(capacity), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &capacity,
        "cublasLtHeuristicsCacheGetCapacity",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtHeuristicsCacheSetCapacity(
    writer: &mut (impl std::io::Write + ?Sized),
    capacity: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(capacity), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &capacity,
        "cublasLtHeuristicsCacheSetCapacity",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtDisableCpuInstructionsSetMask(
    writer: &mut (impl std::io::Write + ?Sized),
    mask: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(mask), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mask,
        "cublasLtDisableCpuInstructionsSetMask",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatrixLayout_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatmulAlgo_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(data), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.data, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatmulDesc_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatrixTransformDesc_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatmulPreference_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatmulTile_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_UNDEFINED => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_UNDEFINED).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_8x8 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_8x8).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_8x16 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_8x16).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_16x8 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_16x8).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_8x32 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_8x32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_16x16 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_16x16).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_32x8 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_32x8).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_8x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_8x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_16x32 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_16x32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_32x16 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_32x16).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x8 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x8).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_32x32 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_32x32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_32x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_32x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x32 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_32x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_32x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x32 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x512 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x512).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x96 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x96).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_96x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_96x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_96x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_96x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x160 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x160).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_160x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_160x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x96 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x96).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_32x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_32x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x32 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_8x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_8x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_8x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_8x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_8x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_8x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_8x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_8x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_8x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_8x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_8x448 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_8x448).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_8x512 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_8x512).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_8x576 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_8x576).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_8x640 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_8x640).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_8x704 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_8x704).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_8x768 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_8x768).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_16x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_16x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_16x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_16x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_16x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_16x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_16x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_16x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_16x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_16x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_16x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_16x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_16x448 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_16x448).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_16x512 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_16x512).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_16x576 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_16x576).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_16x640 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_16x640).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_16x704 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_16x704).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_16x768 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_16x768).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_24x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_24x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_24x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_24x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_24x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_24x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_24x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_24x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_24x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_24x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_24x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_24x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_24x448 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_24x448).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_24x512 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_24x512).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_24x576 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_24x576).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_24x640 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_24x640).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_24x704 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_24x704).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_24x768 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_24x768).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_32x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_32x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_32x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_32x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_32x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_32x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_32x448 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_32x448).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_32x512 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_32x512).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_32x576 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_32x576).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_32x640 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_32x640).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_32x704 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_32x704).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_32x768 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_32x768).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_40x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_40x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_40x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_40x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_40x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_40x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_40x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_40x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_40x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_40x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_40x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_40x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_40x448 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_40x448).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_40x512 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_40x512).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_40x576 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_40x576).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_40x640 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_40x640).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_40x704 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_40x704).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_40x768 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_40x768).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_48x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_48x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_48x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_48x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_48x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_48x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_48x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_48x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_48x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_48x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_48x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_48x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_48x448 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_48x448).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_48x512 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_48x512).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_48x576 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_48x576).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_48x640 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_48x640).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_48x704 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_48x704).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_48x768 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_48x768).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_56x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_56x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_56x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_56x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_56x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_56x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_56x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_56x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_56x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_56x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_56x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_56x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_56x448 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_56x448).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_56x512 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_56x512).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_56x576 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_56x576).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_56x640 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_56x640).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_56x704 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_56x704).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_56x768 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_56x768).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x448 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x448).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x576 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x576).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x640 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x640).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x704 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x704).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x768 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x768).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_72x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_72x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_72x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_72x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_72x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_72x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_72x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_72x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_72x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_72x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_72x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_72x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_72x448 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_72x448).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_72x512 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_72x512).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_72x576 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_72x576).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_72x640 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_72x640).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_80x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_80x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_80x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_80x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_80x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_80x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_80x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_80x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_80x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_80x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_80x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_80x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_80x448 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_80x448).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_80x512 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_80x512).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_80x576 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_80x576).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_88x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_88x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_88x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_88x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_88x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_88x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_88x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_88x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_88x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_88x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_88x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_88x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_88x448 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_88x448).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_88x512 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_88x512).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_96x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_96x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_96x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_96x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_96x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_96x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_96x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_96x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_96x448 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_96x448).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_96x512 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_96x512).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_104x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_104x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_104x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_104x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_104x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_104x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_104x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_104x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_104x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_104x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_104x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_104x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_104x448 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_104x448).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_112x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_112x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_112x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_112x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_112x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_112x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_112x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_112x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_112x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_112x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_112x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_112x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_120x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_120x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_120x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_120x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_120x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_120x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_120x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_120x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_120x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_120x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_120x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_120x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x384 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x384).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_136x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_136x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_136x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_136x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_136x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_136x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_136x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_136x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_136x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_136x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_144x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_144x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_144x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_144x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_144x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_144x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_144x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_144x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_144x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_144x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_152x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_152x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_152x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_152x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_152x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_152x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_152x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_152x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_152x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_152x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_160x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_160x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_160x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_160x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_160x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_160x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_168x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_168x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_168x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_168x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_168x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_168x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_168x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_168x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_176x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_176x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_176x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_176x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_176x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_176x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_176x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_176x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_184x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_184x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_184x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_184x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_184x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_184x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_184x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_184x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_200x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_200x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_200x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_200x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_200x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_200x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_208x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_208x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_208x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_208x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_208x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_208x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_216x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_216x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_216x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_216x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_216x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_216x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_224x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_224x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_224x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_224x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_224x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_224x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_232x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_232x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_232x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_232x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_232x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_232x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_240x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_240x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_240x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_240x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_240x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_240x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_248x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_248x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_248x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_248x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_248x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_248x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_264x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_264x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_264x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_264x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_272x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_272x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_272x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_272x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_280x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_280x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_280x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_280x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_288x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_288x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_288x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_288x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_296x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_296x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_296x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_296x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_304x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_304x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_304x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_304x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_312x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_312x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_312x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_312x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_328x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_328x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_328x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_328x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_336x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_336x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_336x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_336x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_344x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_344x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_344x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_344x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_352x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_352x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_352x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_352x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_360x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_360x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_360x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_360x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_368x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_368x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_368x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_368x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_376x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_376x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_376x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_376x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_392x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_392x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_400x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_400x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_408x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_408x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_416x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_416x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_424x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_424x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_432x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_432x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_440x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_440x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_456x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_456x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_464x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_464x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_472x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_472x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_480x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_480x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_488x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_488x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_496x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_496x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_504x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_504x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_520x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_520x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_528x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_528x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_536x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_536x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_544x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_544x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_552x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_552x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_560x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_560x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_568x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_568x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_576x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_576x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_584x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_584x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_592x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_592x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_600x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_600x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_608x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_608x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_616x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_616x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_624x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_624x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_632x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_632x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_640x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_640x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_648x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_648x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_656x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_656x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_664x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_664x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_672x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_672x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_680x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_680x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_688x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_688x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_696x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_696x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_704x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_704x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_712x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_712x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_720x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_720x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_728x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_728x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_736x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_736x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_744x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_744x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_752x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_752x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_760x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_760x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_768x64 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_768x64).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x16 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x16).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x24 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x24).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x40 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x40).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x48 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x48).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x56 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x56).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x72 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x72).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x80 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x80).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x88 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x88).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x104 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x104).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x112 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x112).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x120 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x120).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x136 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x136).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x144 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x144).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x152 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x152).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x160 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x160).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x168 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x168).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x176 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x176).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x184 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x184).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x200 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x200).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x208 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x208).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x216 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x216).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x224 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x224).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x232 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x232).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x240 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x240).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x248 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x248).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x264 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x264).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x272 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x272).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x280 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x280).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x288 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x288).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x296 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x296).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x304 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x304).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x312 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x312).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x328 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x328).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x336 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x336).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x344 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x344).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x352 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x352).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x360 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x360).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x368 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x368).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x376 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x376).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x392 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x392).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x400 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x400).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x408 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x408).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x416 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x416).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x424 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x424).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x432 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x432).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x440 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x440).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x456 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x456).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x464 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x464).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x472 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x472).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x480 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x480).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x488 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x488).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x496 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x496).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x504 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x504).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x520 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x520).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x528 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x528).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x536 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x536).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x544 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x544).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x552 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x552).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x560 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x560).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x568 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x568).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x584 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x584).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x592 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x592).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x600 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x600).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x608 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x608).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x616 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x616).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x624 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x624).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x632 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x632).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x648 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x648).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x656 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x656).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x664 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x664).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x672 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x672).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x680 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x680).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x688 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x688).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x696 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x696).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x712 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x712).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x720 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x720).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x728 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x728).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x736 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x736).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x744 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x744).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x752 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x752).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_64x760 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_64x760).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x8 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x8).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x16 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x16).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x24 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x24).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x40 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x40).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x48 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x48).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x56 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x56).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x72 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x72).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x80 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x80).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x88 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x88).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x104 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x104).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x112 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x112).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x120 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x120).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x136 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x136).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x144 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x144).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x152 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x152).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x168 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x168).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x176 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x176).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x184 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x184).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x200 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x200).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x208 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x208).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x216 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x216).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x224 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x224).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x232 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x232).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x240 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x240).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x248 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x248).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x264 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x264).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x272 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x272).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x280 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x280).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x288 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x288).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x296 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x296).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x304 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x304).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x312 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x312).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x328 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x328).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x336 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x336).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x344 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x344).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x352 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x352).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x360 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x360).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x368 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x368).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x376 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x376).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x392 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x392).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x400 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x400).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x408 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x408).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x416 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x416).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x424 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x424).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x432 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x432).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x440 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x440).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x448 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x448).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x456 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x456).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x464 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x464).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x472 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x472).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x480 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x480).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x488 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x488).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x496 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x496).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x504 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x504).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_128x512 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_128x512).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x8 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x8).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x16 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x16).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x24 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x24).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x32 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x40 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x40).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x48 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x48).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x56 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x56).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x72 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x72).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x80 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x80).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x88 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x88).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x96 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x96).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x104 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x104).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x112 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x112).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x120 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x120).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x136 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x136).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x144 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x144).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x152 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x152).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x160 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x160).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x168 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x168).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x176 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x176).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x184 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x184).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x200 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x200).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x208 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x208).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x216 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x216).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x224 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x224).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x232 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x232).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x240 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x240).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x248 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x248).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x264 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x264).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x272 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x272).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x280 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x280).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x288 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x288).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x296 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x296).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x304 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x304).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x312 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x312).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x320 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x320).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x328 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x328).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_192x336 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_192x336).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x8 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x8).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x16 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x16).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x24 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x24).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x40 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x40).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x48 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x48).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x56 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x56).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x72 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x72).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x80 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x80).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x88 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x88).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x96 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x96).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x104 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x104).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x112 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x112).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x120 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x120).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x136 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x136).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x144 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x144).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x152 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x152).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x160 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x160).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x168 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x168).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x176 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x176).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x184 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x184).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x200 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x200).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x208 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x208).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x216 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x216).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x224 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x224).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x232 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x232).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x240 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x240).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x248 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x248).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x256 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x256).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x8 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x8).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x16 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x16).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x24 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x24).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x32 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x40 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x40).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x48 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x48).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x56 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x56).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x72 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x72).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x80 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x80).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x88 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x88).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x96 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x96).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x104 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x104).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x112 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x112).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x120 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x120).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x136 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x136).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x144 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x144).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x152 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x152).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x160 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x160).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x168 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x168).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x176 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x176).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x184 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x184).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x192 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x192).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_320x200 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_320x200).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x8 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x8).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x16 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x16).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x24 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x24).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x32 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x40 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x40).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x48 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x48).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x56 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x56).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x72 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x72).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x80 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x80).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x88 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x88).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x96 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x96).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x104 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x104).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x112 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x112).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x120 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x120).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x136 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x136).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x144 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x144).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x152 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x152).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x160 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x160).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_384x168 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_384x168).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x8 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x8).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x16 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x16).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x24 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x24).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x32 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x40 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x40).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x48 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x48).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x56 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x56).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x72 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x72).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x80 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x80).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x88 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x88).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x96 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x96).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x104 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x104).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x112 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x112).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x120 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x120).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x136 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x136).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_448x144 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_448x144).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x8 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x8).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x16 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x16).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x24 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x24).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x32 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x40 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x40).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x48 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x48).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x56 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x56).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x72 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x72).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x80 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x80).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x88 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x88).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x96 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x96).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x104 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x104).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x112 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x112).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x120 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x120).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x128 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x128).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_576x8 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_576x8).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_576x16 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_576x16).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_576x24 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_576x24).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_576x32 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_576x32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_576x40 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_576x40).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_576x48 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_576x48).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_576x56 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_576x56).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_576x72 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_576x72).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_576x80 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_576x80).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_576x88 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_576x88).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_576x96 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_576x96).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_576x104 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_576x104).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_576x112 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_576x112).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_640x8 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_640x8).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_640x16 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_640x16).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_640x24 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_640x24).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_640x32 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_640x32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_640x40 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_640x40).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_640x48 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_640x48).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_640x56 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_640x56).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_640x72 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_640x72).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_640x80 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_640x80).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_640x88 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_640x88).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_640x96 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_640x96).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_704x8 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_704x8).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_704x16 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_704x16).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_704x24 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_704x24).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_704x32 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_704x32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_704x40 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_704x40).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_704x48 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_704x48).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_704x56 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_704x56).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_704x72 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_704x72).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_704x80 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_704x80).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_704x88 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_704x88).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_768x8 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_768x8).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_768x16 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_768x16).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_768x24 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_768x24).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_768x32 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_768x32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_768x40 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_768x40).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_768x48 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_768x48).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_768x56 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_768x56).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_768x72 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_768x72).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_768x80 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_768x80).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x512 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x512).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_256x1024 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_256x1024).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x512 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x512).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_512x1024 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_512x1024).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulTile_t::CUBLASLT_MATMUL_TILE_END => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_TILE_END).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatmulStages_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_UNDEFINED => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_UNDEFINED).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_16x1 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_16x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_16x2 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_16x2).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_16x3 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_16x3).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_16x4 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_16x4).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_16x5 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_16x5).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_16x6 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_16x6).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_32x1 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_32x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_32x2 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_32x2).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_32x3 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_32x3).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_32x4 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_32x4).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_32x5 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_32x5).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_32x6 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_32x6).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_64x1 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_64x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_64x2 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_64x2).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_64x3 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_64x3).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_64x4 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_64x4).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_64x5 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_64x5).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_64x6 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_64x6).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_128x1 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_128x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_128x2 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_128x2).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_128x3 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_128x3).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_128x4 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_128x4).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_128x5 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_128x5).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_128x6 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_128x6).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_32x10 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_32x10).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_8x4 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_8x4).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_16x10 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_16x10).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_8x5 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_8x5).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_8x3 => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_8x3).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_8xAUTO => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_8xAUTO).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_16xAUTO => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_16xAUTO).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_32xAUTO => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_32xAUTO).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_64xAUTO => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_64xAUTO).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_128xAUTO => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_128xAUTO).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_256xAUTO => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_256xAUTO).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulStages_t::CUBLASLT_MATMUL_STAGES_END => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_STAGES_END).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtClusterShape_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_AUTO => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_AUTO).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_2x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_2x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_4x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_4x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x2x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x2x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_2x2x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_2x2x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_4x2x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_4x2x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x4x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x4x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_2x4x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_2x4x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_4x4x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_4x4x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_8x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_8x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x8x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x8x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_8x2x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_8x2x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_2x8x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_2x8x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_16x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_16x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x16x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x16x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_3x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_3x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_5x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_5x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_6x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_6x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_7x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_7x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_9x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_9x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_10x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_10x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_11x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_11x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_12x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_12x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_13x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_13x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_14x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_14x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_15x1x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_15x1x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_3x2x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_3x2x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_5x2x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_5x2x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_6x2x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_6x2x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_7x2x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_7x2x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x3x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x3x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_2x3x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_2x3x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_3x3x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_3x3x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_4x3x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_4x3x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_5x3x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_5x3x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_3x4x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_3x4x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x5x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x5x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_2x5x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_2x5x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_3x5x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_3x5x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x6x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x6x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_2x6x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_2x6x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x7x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x7x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_2x7x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_2x7x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x9x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x9x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x10x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x10x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x11x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x11x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x12x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x12x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x13x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x13x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x14x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x14x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_1x15x1 => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_1x15x1).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtClusterShape_t::CUBLASLT_CLUSTER_SHAPE_END => {
                writer.write_all(stringify!(CUBLASLT_CLUSTER_SHAPE_END).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatmulInnerShape_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtMatmulInnerShape_t::CUBLASLT_MATMUL_INNER_SHAPE_UNDEFINED => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_INNER_SHAPE_UNDEFINED).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulInnerShape_t::CUBLASLT_MATMUL_INNER_SHAPE_MMA884 => {
                writer
                    .write_all(stringify!(CUBLASLT_MATMUL_INNER_SHAPE_MMA884).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulInnerShape_t::CUBLASLT_MATMUL_INNER_SHAPE_MMA1684 => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_INNER_SHAPE_MMA1684).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulInnerShape_t::CUBLASLT_MATMUL_INNER_SHAPE_MMA1688 => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_INNER_SHAPE_MMA1688).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulInnerShape_t::CUBLASLT_MATMUL_INNER_SHAPE_MMA16816 => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_INNER_SHAPE_MMA16816).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulInnerShape_t::CUBLASLT_MATMUL_INNER_SHAPE_END => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_INNER_SHAPE_END).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatmulMatrixScale_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtMatmulMatrixScale_t::CUBLASLT_MATMUL_MATRIX_SCALE_SCALAR_32F => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_MATRIX_SCALE_SCALAR_32F).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulMatrixScale_t::CUBLASLT_MATMUL_MATRIX_SCALE_VEC16_UE4M3 => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_MATRIX_SCALE_VEC16_UE4M3).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulMatrixScale_t::CUBLASLT_MATMUL_MATRIX_SCALE_VEC32_UE8M0 => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_MATRIX_SCALE_VEC32_UE8M0).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulMatrixScale_t::CUBLASLT_MATMUL_MATRIX_SCALE_OUTER_VEC_32F => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_MATRIX_SCALE_OUTER_VEC_32F).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulMatrixScale_t::CUBLASLT_MATMUL_MATRIX_SCALE_VEC128_32F => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_MATRIX_SCALE_VEC128_32F).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulMatrixScale_t::CUBLASLT_MATMUL_MATRIX_SCALE_BLK128x128_32F => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_MATRIX_SCALE_BLK128x128_32F)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulMatrixScale_t::CUBLASLT_MATMUL_MATRIX_SCALE_END => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_MATRIX_SCALE_END).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtPointerMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtPointerMode_t::CUBLASLT_POINTER_MODE_HOST => {
                writer.write_all(stringify!(CUBLASLT_POINTER_MODE_HOST).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtPointerMode_t::CUBLASLT_POINTER_MODE_DEVICE => {
                writer.write_all(stringify!(CUBLASLT_POINTER_MODE_DEVICE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtPointerMode_t::CUBLASLT_POINTER_MODE_DEVICE_VECTOR => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_POINTER_MODE_DEVICE_VECTOR).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtPointerMode_t::CUBLASLT_POINTER_MODE_ALPHA_DEVICE_VECTOR_BETA_ZERO => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_POINTER_MODE_ALPHA_DEVICE_VECTOR_BETA_ZERO)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtPointerMode_t::CUBLASLT_POINTER_MODE_ALPHA_DEVICE_VECTOR_BETA_HOST => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_POINTER_MODE_ALPHA_DEVICE_VECTOR_BETA_HOST)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtPointerModeMask_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtPointerModeMask_t::CUBLASLT_POINTER_MODE_MASK_HOST => {
                writer.write_all(stringify!(CUBLASLT_POINTER_MODE_MASK_HOST).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtPointerModeMask_t::CUBLASLT_POINTER_MODE_MASK_DEVICE => {
                writer
                    .write_all(stringify!(CUBLASLT_POINTER_MODE_MASK_DEVICE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtPointerModeMask_t::CUBLASLT_POINTER_MODE_MASK_DEVICE_VECTOR => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_POINTER_MODE_MASK_DEVICE_VECTOR).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtPointerModeMask_t::CUBLASLT_POINTER_MODE_MASK_ALPHA_DEVICE_VECTOR_BETA_ZERO => {
                writer
                    .write_all(
                        stringify!(
                            CUBLASLT_POINTER_MODE_MASK_ALPHA_DEVICE_VECTOR_BETA_ZERO
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtPointerModeMask_t::CUBLASLT_POINTER_MODE_MASK_ALPHA_DEVICE_VECTOR_BETA_HOST => {
                writer
                    .write_all(
                        stringify!(
                            CUBLASLT_POINTER_MODE_MASK_ALPHA_DEVICE_VECTOR_BETA_HOST
                        )
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cublasLtMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    lightHandle: cuda_types::cublaslt::cublasLtHandle_t,
    computeDesc: cuda_types::cublaslt::cublasLtMatmulDesc_t,
    alpha: *const ::core::ffi::c_void,
    A: *const ::core::ffi::c_void,
    Adesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    B: *const ::core::ffi::c_void,
    Bdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    beta: *const ::core::ffi::c_void,
    C: *const ::core::ffi::c_void,
    Cdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    D: *mut ::core::ffi::c_void,
    Ddesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    algo: *const cuda_types::cublaslt::cublasLtMatmulAlgo_t,
    workspace: *mut ::core::ffi::c_void,
    workspaceSizeInBytes: usize,
    stream: cuda_types::cublaslt::cudaStream_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(lightHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lightHandle, "cublasLtMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(computeDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&computeDesc, "cublasLtMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cublasLtMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(A), ": ").as_bytes())?;
    crate::CudaDisplay::write(&A, "cublasLtMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Adesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Adesc, "cublasLtMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(B), ": ").as_bytes())?;
    crate::CudaDisplay::write(&B, "cublasLtMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Bdesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Bdesc, "cublasLtMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cublasLtMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(C), ": ").as_bytes())?;
    crate::CudaDisplay::write(&C, "cublasLtMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Cdesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Cdesc, "cublasLtMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(D), ": ").as_bytes())?;
    crate::CudaDisplay::write(&D, "cublasLtMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Ddesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Ddesc, "cublasLtMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cublasLtMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workspace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workspace, "cublasLtMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workspaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workspaceSizeInBytes, "cublasLtMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cublasLtMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatrixTransform(
    writer: &mut (impl std::io::Write + ?Sized),
    lightHandle: cuda_types::cublaslt::cublasLtHandle_t,
    transformDesc: cuda_types::cublaslt::cublasLtMatrixTransformDesc_t,
    alpha: *const ::core::ffi::c_void,
    A: *const ::core::ffi::c_void,
    Adesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    beta: *const ::core::ffi::c_void,
    B: *const ::core::ffi::c_void,
    Bdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    C: *mut ::core::ffi::c_void,
    Cdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    stream: cuda_types::cublaslt::cudaStream_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(lightHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lightHandle, "cublasLtMatrixTransform", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(transformDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &transformDesc,
        "cublasLtMatrixTransform",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cublasLtMatrixTransform", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(A), ": ").as_bytes())?;
    crate::CudaDisplay::write(&A, "cublasLtMatrixTransform", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Adesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Adesc, "cublasLtMatrixTransform", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cublasLtMatrixTransform", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(B), ": ").as_bytes())?;
    crate::CudaDisplay::write(&B, "cublasLtMatrixTransform", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Bdesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Bdesc, "cublasLtMatrixTransform", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(C), ": ").as_bytes())?;
    crate::CudaDisplay::write(&C, "cublasLtMatrixTransform", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Cdesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Cdesc, "cublasLtMatrixTransform", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cublasLtMatrixTransform", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtOrder_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtOrder_t::CUBLASLT_ORDER_COL => {
                writer.write_all(stringify!(CUBLASLT_ORDER_COL).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtOrder_t::CUBLASLT_ORDER_ROW => {
                writer.write_all(stringify!(CUBLASLT_ORDER_ROW).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtOrder_t::CUBLASLT_ORDER_COL32 => {
                writer.write_all(stringify!(CUBLASLT_ORDER_COL32).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtOrder_t::CUBLASLT_ORDER_COL4_4R2_8C => {
                writer.write_all(stringify!(CUBLASLT_ORDER_COL4_4R2_8C).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtOrder_t::CUBLASLT_ORDER_COL32_2R_4R4 => {
                writer.write_all(stringify!(CUBLASLT_ORDER_COL32_2R_4R4).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtBatchMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtBatchMode_t::CUBLASLT_BATCH_MODE_STRIDED => {
                writer.write_all(stringify!(CUBLASLT_BATCH_MODE_STRIDED).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtBatchMode_t::CUBLASLT_BATCH_MODE_POINTER_ARRAY => {
                writer
                    .write_all(stringify!(CUBLASLT_BATCH_MODE_POINTER_ARRAY).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatrixLayoutAttribute_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_TYPE => {
                writer.write_all(stringify!(CUBLASLT_MATRIX_LAYOUT_TYPE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_ORDER => {
                writer.write_all(stringify!(CUBLASLT_MATRIX_LAYOUT_ORDER).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_ROWS => {
                writer.write_all(stringify!(CUBLASLT_MATRIX_LAYOUT_ROWS).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_COLS => {
                writer.write_all(stringify!(CUBLASLT_MATRIX_LAYOUT_COLS).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_LD => {
                writer.write_all(stringify!(CUBLASLT_MATRIX_LAYOUT_LD).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_BATCH_COUNT => {
                writer
                    .write_all(stringify!(CUBLASLT_MATRIX_LAYOUT_BATCH_COUNT).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_PLANE_OFFSET => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATRIX_LAYOUT_PLANE_OFFSET).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_BATCH_MODE => {
                writer
                    .write_all(stringify!(CUBLASLT_MATRIX_LAYOUT_BATCH_MODE).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cublasLtMatrixLayoutInit_internal(
    writer: &mut (impl std::io::Write + ?Sized),
    matLayout: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    size: usize,
    type_: cuda_types::cublaslt::cudaDataType,
    rows: u64,
    cols: u64,
    ld: i64,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(matLayout), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &matLayout,
        "cublasLtMatrixLayoutInit_internal",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &size,
        "cublasLtMatrixLayoutInit_internal",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &type_,
        "cublasLtMatrixLayoutInit_internal",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rows), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rows,
        "cublasLtMatrixLayoutInit_internal",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cols), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cols,
        "cublasLtMatrixLayoutInit_internal",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ld), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ld,
        "cublasLtMatrixLayoutInit_internal",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatrixLayoutCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    matLayout: *mut cuda_types::cublaslt::cublasLtMatrixLayout_t,
    type_: cuda_types::cublaslt::cudaDataType,
    rows: u64,
    cols: u64,
    ld: i64,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(matLayout), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &matLayout,
        "cublasLtMatrixLayoutCreate",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cublasLtMatrixLayoutCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rows), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rows, "cublasLtMatrixLayoutCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cols), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cols, "cublasLtMatrixLayoutCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ld), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ld, "cublasLtMatrixLayoutCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatrixLayoutDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    matLayout: cuda_types::cublaslt::cublasLtMatrixLayout_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(matLayout), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &matLayout,
        "cublasLtMatrixLayoutDestroy",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatrixLayoutSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    matLayout: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    attr: cuda_types::cublaslt::cublasLtMatrixLayoutAttribute_t,
    buf: *const ::core::ffi::c_void,
    sizeInBytes: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(matLayout), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &matLayout,
        "cublasLtMatrixLayoutSetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attr), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attr,
        "cublasLtMatrixLayoutSetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buf), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &buf,
        "cublasLtMatrixLayoutSetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cublasLtMatrixLayoutSetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatrixLayoutGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    matLayout: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    attr: cuda_types::cublaslt::cublasLtMatrixLayoutAttribute_t,
    buf: *mut ::core::ffi::c_void,
    sizeInBytes: usize,
    sizeWritten: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(matLayout), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &matLayout,
        "cublasLtMatrixLayoutGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attr), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attr,
        "cublasLtMatrixLayoutGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buf), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &buf,
        "cublasLtMatrixLayoutGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cublasLtMatrixLayoutGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeWritten), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeWritten,
        "cublasLtMatrixLayoutGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatmulDescAttributes_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_COMPUTE_TYPE => {
                writer
                    .write_all(stringify!(CUBLASLT_MATMUL_DESC_COMPUTE_TYPE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_SCALE_TYPE => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_DESC_SCALE_TYPE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_POINTER_MODE => {
                writer
                    .write_all(stringify!(CUBLASLT_MATMUL_DESC_POINTER_MODE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_TRANSA => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_DESC_TRANSA).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_TRANSB => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_DESC_TRANSB).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_TRANSC => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_DESC_TRANSC).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_FILL_MODE => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_DESC_FILL_MODE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_EPILOGUE => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_DESC_EPILOGUE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_BIAS_POINTER => {
                writer
                    .write_all(stringify!(CUBLASLT_MATMUL_DESC_BIAS_POINTER).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_BIAS_BATCH_STRIDE => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_BIAS_BATCH_STRIDE).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_LD => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_LD).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_BATCH_STRIDE => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_BATCH_STRIDE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_ALPHA_VECTOR_BATCH_STRIDE => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_ALPHA_VECTOR_BATCH_STRIDE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_SM_COUNT_TARGET => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_SM_COUNT_TARGET).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_A_SCALE_POINTER => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_A_SCALE_POINTER).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_B_SCALE_POINTER => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_B_SCALE_POINTER).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_C_SCALE_POINTER => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_C_SCALE_POINTER).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_D_SCALE_POINTER => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_D_SCALE_POINTER).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_AMAX_D_POINTER => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_AMAX_D_POINTER).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_DATA_TYPE => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_DATA_TYPE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_POINTER => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_POINTER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_AMAX_POINTER => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_AMAX_POINTER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_FAST_ACCUM => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_DESC_FAST_ACCUM).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_BIAS_DATA_TYPE => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_BIAS_DATA_TYPE).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_ATOMIC_SYNC_NUM_CHUNKS_D_ROWS => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_ATOMIC_SYNC_NUM_CHUNKS_D_ROWS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_ATOMIC_SYNC_NUM_CHUNKS_D_COLS => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_ATOMIC_SYNC_NUM_CHUNKS_D_COLS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_ATOMIC_SYNC_IN_COUNTERS_POINTER => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_ATOMIC_SYNC_IN_COUNTERS_POINTER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_ATOMIC_SYNC_OUT_COUNTERS_POINTER => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_ATOMIC_SYNC_OUT_COUNTERS_POINTER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_A_SCALE_MODE => {
                writer
                    .write_all(stringify!(CUBLASLT_MATMUL_DESC_A_SCALE_MODE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_B_SCALE_MODE => {
                writer
                    .write_all(stringify!(CUBLASLT_MATMUL_DESC_B_SCALE_MODE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_C_SCALE_MODE => {
                writer
                    .write_all(stringify!(CUBLASLT_MATMUL_DESC_C_SCALE_MODE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_D_SCALE_MODE => {
                writer
                    .write_all(stringify!(CUBLASLT_MATMUL_DESC_D_SCALE_MODE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_MODE => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_MODE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_D_OUT_SCALE_POINTER => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_D_OUT_SCALE_POINTER).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_D_OUT_SCALE_MODE => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_DESC_D_OUT_SCALE_MODE).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cublasLtMatmulDescInit_internal(
    writer: &mut (impl std::io::Write + ?Sized),
    matmulDesc: cuda_types::cublaslt::cublasLtMatmulDesc_t,
    size: usize,
    computeType: cuda_types::cublas::cublasComputeType_t,
    scaleType: cuda_types::cublaslt::cudaDataType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(matmulDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &matmulDesc,
        "cublasLtMatmulDescInit_internal",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &size,
        "cublasLtMatmulDescInit_internal",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(computeType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &computeType,
        "cublasLtMatmulDescInit_internal",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(scaleType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &scaleType,
        "cublasLtMatmulDescInit_internal",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatmulDescCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    matmulDesc: *mut cuda_types::cublaslt::cublasLtMatmulDesc_t,
    computeType: cuda_types::cublas::cublasComputeType_t,
    scaleType: cuda_types::cublaslt::cudaDataType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(matmulDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&matmulDesc, "cublasLtMatmulDescCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(computeType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &computeType,
        "cublasLtMatmulDescCreate",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(scaleType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&scaleType, "cublasLtMatmulDescCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatmulDescDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    matmulDesc: cuda_types::cublaslt::cublasLtMatmulDesc_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(matmulDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &matmulDesc,
        "cublasLtMatmulDescDestroy",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatmulDescSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    matmulDesc: cuda_types::cublaslt::cublasLtMatmulDesc_t,
    attr: cuda_types::cublaslt::cublasLtMatmulDescAttributes_t,
    buf: *const ::core::ffi::c_void,
    sizeInBytes: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(matmulDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &matmulDesc,
        "cublasLtMatmulDescSetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attr, "cublasLtMatmulDescSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buf), ": ").as_bytes())?;
    crate::CudaDisplay::write(&buf, "cublasLtMatmulDescSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cublasLtMatmulDescSetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatmulDescGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    matmulDesc: cuda_types::cublaslt::cublasLtMatmulDesc_t,
    attr: cuda_types::cublaslt::cublasLtMatmulDescAttributes_t,
    buf: *mut ::core::ffi::c_void,
    sizeInBytes: usize,
    sizeWritten: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(matmulDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &matmulDesc,
        "cublasLtMatmulDescGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attr, "cublasLtMatmulDescGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buf), ": ").as_bytes())?;
    crate::CudaDisplay::write(&buf, "cublasLtMatmulDescGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cublasLtMatmulDescGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeWritten), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeWritten,
        "cublasLtMatmulDescGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
impl crate::CudaDisplay
for cuda_types::cublaslt::cublasLtMatrixTransformDescAttributes_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtMatrixTransformDescAttributes_t::CUBLASLT_MATRIX_TRANSFORM_DESC_SCALE_TYPE => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATRIX_TRANSFORM_DESC_SCALE_TYPE).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatrixTransformDescAttributes_t::CUBLASLT_MATRIX_TRANSFORM_DESC_POINTER_MODE => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATRIX_TRANSFORM_DESC_POINTER_MODE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatrixTransformDescAttributes_t::CUBLASLT_MATRIX_TRANSFORM_DESC_TRANSA => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATRIX_TRANSFORM_DESC_TRANSA).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatrixTransformDescAttributes_t::CUBLASLT_MATRIX_TRANSFORM_DESC_TRANSB => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATRIX_TRANSFORM_DESC_TRANSB).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cublasLtMatrixTransformDescInit_internal(
    writer: &mut (impl std::io::Write + ?Sized),
    transformDesc: cuda_types::cublaslt::cublasLtMatrixTransformDesc_t,
    size: usize,
    scaleType: cuda_types::cublaslt::cudaDataType,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(transformDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &transformDesc,
        "cublasLtMatrixTransformDescInit_internal",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &size,
        "cublasLtMatrixTransformDescInit_internal",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(scaleType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &scaleType,
        "cublasLtMatrixTransformDescInit_internal",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatrixTransformDescCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    transformDesc: *mut cuda_types::cublaslt::cublasLtMatrixTransformDesc_t,
    scaleType: cuda_types::cublaslt::cudaDataType,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(transformDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &transformDesc,
        "cublasLtMatrixTransformDescCreate",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(scaleType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &scaleType,
        "cublasLtMatrixTransformDescCreate",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatrixTransformDescDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    transformDesc: cuda_types::cublaslt::cublasLtMatrixTransformDesc_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(transformDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &transformDesc,
        "cublasLtMatrixTransformDescDestroy",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatrixTransformDescSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    transformDesc: cuda_types::cublaslt::cublasLtMatrixTransformDesc_t,
    attr: cuda_types::cublaslt::cublasLtMatrixTransformDescAttributes_t,
    buf: *const ::core::ffi::c_void,
    sizeInBytes: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(transformDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &transformDesc,
        "cublasLtMatrixTransformDescSetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attr), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attr,
        "cublasLtMatrixTransformDescSetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buf), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &buf,
        "cublasLtMatrixTransformDescSetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cublasLtMatrixTransformDescSetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatrixTransformDescGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    transformDesc: cuda_types::cublaslt::cublasLtMatrixTransformDesc_t,
    attr: cuda_types::cublaslt::cublasLtMatrixTransformDescAttributes_t,
    buf: *mut ::core::ffi::c_void,
    sizeInBytes: usize,
    sizeWritten: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(transformDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &transformDesc,
        "cublasLtMatrixTransformDescGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attr), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attr,
        "cublasLtMatrixTransformDescGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buf), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &buf,
        "cublasLtMatrixTransformDescGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cublasLtMatrixTransformDescGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeWritten), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeWritten,
        "cublasLtMatrixTransformDescGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtReductionScheme_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtReductionScheme_t::CUBLASLT_REDUCTION_SCHEME_NONE => {
                writer.write_all(stringify!(CUBLASLT_REDUCTION_SCHEME_NONE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtReductionScheme_t::CUBLASLT_REDUCTION_SCHEME_INPLACE => {
                writer
                    .write_all(stringify!(CUBLASLT_REDUCTION_SCHEME_INPLACE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtReductionScheme_t::CUBLASLT_REDUCTION_SCHEME_COMPUTE_TYPE => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_REDUCTION_SCHEME_COMPUTE_TYPE).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtReductionScheme_t::CUBLASLT_REDUCTION_SCHEME_OUTPUT_TYPE => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_REDUCTION_SCHEME_OUTPUT_TYPE).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtReductionScheme_t::CUBLASLT_REDUCTION_SCHEME_MASK => {
                writer.write_all(stringify!(CUBLASLT_REDUCTION_SCHEME_MASK).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtEpilogue_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_DEFAULT => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_DEFAULT).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_RELU => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_RELU).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_RELU_AUX => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_RELU_AUX).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_BIAS => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_BIAS).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_RELU_BIAS => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_RELU_BIAS).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_RELU_AUX_BIAS => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_RELU_AUX_BIAS).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_DRELU => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_DRELU).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_DRELU_BGRAD => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_DRELU_BGRAD).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_GELU => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_GELU).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_GELU_AUX => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_GELU_AUX).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_GELU_BIAS => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_GELU_BIAS).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_GELU_AUX_BIAS => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_GELU_AUX_BIAS).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_DGELU => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_DGELU).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_DGELU_BGRAD => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_DGELU_BGRAD).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_BGRADA => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_BGRADA).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtEpilogue_t::CUBLASLT_EPILOGUE_BGRADB => {
                writer.write_all(stringify!(CUBLASLT_EPILOGUE_BGRADB).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatmulSearch_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtMatmulSearch_t::CUBLASLT_SEARCH_BEST_FIT => {
                writer.write_all(stringify!(CUBLASLT_SEARCH_BEST_FIT).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulSearch_t::CUBLASLT_SEARCH_LIMITED_BY_ALGO_ID => {
                writer
                    .write_all(stringify!(CUBLASLT_SEARCH_LIMITED_BY_ALGO_ID).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulSearch_t::CUBLASLT_SEARCH_RESERVED_02 => {
                writer.write_all(stringify!(CUBLASLT_SEARCH_RESERVED_02).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulSearch_t::CUBLASLT_SEARCH_RESERVED_03 => {
                writer.write_all(stringify!(CUBLASLT_SEARCH_RESERVED_03).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulSearch_t::CUBLASLT_SEARCH_RESERVED_04 => {
                writer.write_all(stringify!(CUBLASLT_SEARCH_RESERVED_04).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulSearch_t::CUBLASLT_SEARCH_RESERVED_05 => {
                writer.write_all(stringify!(CUBLASLT_SEARCH_RESERVED_05).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulSearch_t::CUBLASLT_SEARCH_RESERVED_06 => {
                writer.write_all(stringify!(CUBLASLT_SEARCH_RESERVED_06).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulSearch_t::CUBLASLT_SEARCH_RESERVED_07 => {
                writer.write_all(stringify!(CUBLASLT_SEARCH_RESERVED_07).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulSearch_t::CUBLASLT_SEARCH_RESERVED_08 => {
                writer.write_all(stringify!(CUBLASLT_SEARCH_RESERVED_08).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulSearch_t::CUBLASLT_SEARCH_RESERVED_09 => {
                writer.write_all(stringify!(CUBLASLT_SEARCH_RESERVED_09).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatmulPreferenceAttributes_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtMatmulPreferenceAttributes_t::CUBLASLT_MATMUL_PREF_SEARCH_MODE => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_PREF_SEARCH_MODE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulPreferenceAttributes_t::CUBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulPreferenceAttributes_t::CUBLASLT_MATMUL_PREF_REDUCTION_SCHEME_MASK => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_PREF_REDUCTION_SCHEME_MASK).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulPreferenceAttributes_t::CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_A_BYTES => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_A_BYTES).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulPreferenceAttributes_t::CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_B_BYTES => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_B_BYTES).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulPreferenceAttributes_t::CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_C_BYTES => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_C_BYTES).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulPreferenceAttributes_t::CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_D_BYTES => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_D_BYTES).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulPreferenceAttributes_t::CUBLASLT_MATMUL_PREF_MAX_WAVES_COUNT => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_MATMUL_PREF_MAX_WAVES_COUNT).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulPreferenceAttributes_t::CUBLASLT_MATMUL_PREF_IMPL_MASK => {
                writer.write_all(stringify!(CUBLASLT_MATMUL_PREF_IMPL_MASK).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cublasLtMatmulPreferenceInit_internal(
    writer: &mut (impl std::io::Write + ?Sized),
    pref: cuda_types::cublaslt::cublasLtMatmulPreference_t,
    size: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pref), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pref,
        "cublasLtMatmulPreferenceInit_internal",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &size,
        "cublasLtMatmulPreferenceInit_internal",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatmulPreferenceCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    pref: *mut cuda_types::cublaslt::cublasLtMatmulPreference_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pref), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pref, "cublasLtMatmulPreferenceCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatmulPreferenceDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    pref: cuda_types::cublaslt::cublasLtMatmulPreference_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pref), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pref,
        "cublasLtMatmulPreferenceDestroy",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatmulPreferenceSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    pref: cuda_types::cublaslt::cublasLtMatmulPreference_t,
    attr: cuda_types::cublaslt::cublasLtMatmulPreferenceAttributes_t,
    buf: *const ::core::ffi::c_void,
    sizeInBytes: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pref), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pref,
        "cublasLtMatmulPreferenceSetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attr), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attr,
        "cublasLtMatmulPreferenceSetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buf), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &buf,
        "cublasLtMatmulPreferenceSetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cublasLtMatmulPreferenceSetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatmulPreferenceGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    pref: cuda_types::cublaslt::cublasLtMatmulPreference_t,
    attr: cuda_types::cublaslt::cublasLtMatmulPreferenceAttributes_t,
    buf: *mut ::core::ffi::c_void,
    sizeInBytes: usize,
    sizeWritten: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pref), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pref,
        "cublasLtMatmulPreferenceGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attr), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attr,
        "cublasLtMatmulPreferenceGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buf), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &buf,
        "cublasLtMatmulPreferenceGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cublasLtMatmulPreferenceGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeWritten), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeWritten,
        "cublasLtMatmulPreferenceGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatmulHeuristicResult_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(algo), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.algo, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(workspaceSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.workspaceSize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(state), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.state, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(wavesCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.wavesCount, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_cublasLtMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    lightHandle: cuda_types::cublaslt::cublasLtHandle_t,
    operationDesc: cuda_types::cublaslt::cublasLtMatmulDesc_t,
    Adesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    Bdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    Cdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    Ddesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    preference: cuda_types::cublaslt::cublasLtMatmulPreference_t,
    requestedAlgoCount: ::core::ffi::c_int,
    heuristicResultsArray: *mut cuda_types::cublaslt::cublasLtMatmulHeuristicResult_t,
    returnAlgoCount: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(lightHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &lightHandle,
        "cublasLtMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(operationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &operationDesc,
        "cublasLtMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Adesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &Adesc,
        "cublasLtMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Bdesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &Bdesc,
        "cublasLtMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Cdesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &Cdesc,
        "cublasLtMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Ddesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &Ddesc,
        "cublasLtMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(preference), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &preference,
        "cublasLtMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedAlgoCount,
        "cublasLtMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(heuristicResultsArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &heuristicResultsArray,
        "cublasLtMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnAlgoCount,
        "cublasLtMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    lightHandle: cuda_types::cublaslt::cublasLtHandle_t,
    computeType: cuda_types::cublas::cublasComputeType_t,
    scaleType: cuda_types::cublaslt::cudaDataType_t,
    Atype: cuda_types::cublaslt::cudaDataType_t,
    Btype: cuda_types::cublaslt::cudaDataType_t,
    Ctype: cuda_types::cublaslt::cudaDataType_t,
    Dtype: cuda_types::cublaslt::cudaDataType_t,
    requestedAlgoCount: ::core::ffi::c_int,
    algoIdsArray: *mut ::core::ffi::c_int,
    returnAlgoCount: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(lightHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &lightHandle,
        "cublasLtMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(computeType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &computeType,
        "cublasLtMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(scaleType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&scaleType, "cublasLtMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Atype), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Atype, "cublasLtMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Btype), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Btype, "cublasLtMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Ctype), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Ctype, "cublasLtMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Dtype), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Dtype, "cublasLtMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedAlgoCount,
        "cublasLtMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algoIdsArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algoIdsArray,
        "cublasLtMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnAlgoCount,
        "cublasLtMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    lightHandle: cuda_types::cublaslt::cublasLtHandle_t,
    computeType: cuda_types::cublas::cublasComputeType_t,
    scaleType: cuda_types::cublaslt::cudaDataType_t,
    Atype: cuda_types::cublaslt::cudaDataType_t,
    Btype: cuda_types::cublaslt::cudaDataType_t,
    Ctype: cuda_types::cublaslt::cudaDataType_t,
    Dtype: cuda_types::cublaslt::cudaDataType_t,
    algoId: ::core::ffi::c_int,
    algo: *mut cuda_types::cublaslt::cublasLtMatmulAlgo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(lightHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lightHandle, "cublasLtMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(computeType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&computeType, "cublasLtMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(scaleType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&scaleType, "cublasLtMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Atype), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Atype, "cublasLtMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Btype), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Btype, "cublasLtMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Ctype), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Ctype, "cublasLtMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Dtype), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Dtype, "cublasLtMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algoId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algoId, "cublasLtMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cublasLtMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    lightHandle: cuda_types::cublaslt::cublasLtHandle_t,
    operationDesc: cuda_types::cublaslt::cublasLtMatmulDesc_t,
    Adesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    Bdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    Cdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    Ddesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    algo: *const cuda_types::cublaslt::cublasLtMatmulAlgo_t,
    result: *mut cuda_types::cublaslt::cublasLtMatmulHeuristicResult_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(lightHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lightHandle, "cublasLtMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(operationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &operationDesc,
        "cublasLtMatmulAlgoCheck",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Adesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Adesc, "cublasLtMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Bdesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Bdesc, "cublasLtMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Cdesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Cdesc, "cublasLtMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Ddesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Ddesc, "cublasLtMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cublasLtMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(result), ": ").as_bytes())?;
    crate::CudaDisplay::write(&result, "cublasLtMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_SPLITK_SUPPORT => {
                writer.write_all(stringify!(CUBLASLT_ALGO_CAP_SPLITK_SUPPORT).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_REDUCTION_SCHEME_MASK => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CAP_REDUCTION_SCHEME_MASK).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_CTA_SWIZZLING_SUPPORT => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CAP_CTA_SWIZZLING_SUPPORT).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_STRIDED_BATCH_SUPPORT => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CAP_STRIDED_BATCH_SUPPORT).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_OUT_OF_PLACE_RESULT_SUPPORT => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CAP_OUT_OF_PLACE_RESULT_SUPPORT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_UPLO_SUPPORT => {
                writer.write_all(stringify!(CUBLASLT_ALGO_CAP_UPLO_SUPPORT).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_TILE_IDS => {
                writer.write_all(stringify!(CUBLASLT_ALGO_CAP_TILE_IDS).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_CUSTOM_OPTION_MAX => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CAP_CUSTOM_OPTION_MAX).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_CUSTOM_MEMORY_ORDER => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CAP_CUSTOM_MEMORY_ORDER).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_POINTER_MODE_MASK => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CAP_POINTER_MODE_MASK).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_EPILOGUE_MASK => {
                writer.write_all(stringify!(CUBLASLT_ALGO_CAP_EPILOGUE_MASK).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_STAGES_IDS => {
                writer.write_all(stringify!(CUBLASLT_ALGO_CAP_STAGES_IDS).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_LD_NEGATIVE => {
                writer.write_all(stringify!(CUBLASLT_ALGO_CAP_LD_NEGATIVE).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_NUMERICAL_IMPL_FLAGS => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CAP_NUMERICAL_IMPL_FLAGS).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_A_BYTES => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_A_BYTES).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_B_BYTES => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_B_BYTES).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_C_BYTES => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_C_BYTES).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_D_BYTES => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_D_BYTES).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_ATOMIC_SYNC => {
                writer.write_all(stringify!(CUBLASLT_ALGO_CAP_ATOMIC_SYNC).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_POINTER_ARRAY_BATCH_SUPPORT => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CAP_POINTER_ARRAY_BATCH_SUPPORT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t::CUBLASLT_ALGO_CAP_FLOATING_POINT_EMULATION_SUPPORT => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CAP_FLOATING_POINT_EMULATION_SUPPORT)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cublasLtMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    algo: *const cuda_types::cublaslt::cublasLtMatmulAlgo_t,
    attr: cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t,
    buf: *mut ::core::ffi::c_void,
    sizeInBytes: usize,
    sizeWritten: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algo,
        "cublasLtMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attr), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attr,
        "cublasLtMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buf), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &buf,
        "cublasLtMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cublasLtMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeWritten), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeWritten,
        "cublasLtMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cublaslt::cublasLtMatmulAlgoConfigAttributes_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cublaslt::cublasLtMatmulAlgoConfigAttributes_t::CUBLASLT_ALGO_CONFIG_ID => {
                writer.write_all(stringify!(CUBLASLT_ALGO_CONFIG_ID).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoConfigAttributes_t::CUBLASLT_ALGO_CONFIG_TILE_ID => {
                writer.write_all(stringify!(CUBLASLT_ALGO_CONFIG_TILE_ID).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoConfigAttributes_t::CUBLASLT_ALGO_CONFIG_SPLITK_NUM => {
                writer.write_all(stringify!(CUBLASLT_ALGO_CONFIG_SPLITK_NUM).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoConfigAttributes_t::CUBLASLT_ALGO_CONFIG_REDUCTION_SCHEME => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CONFIG_REDUCTION_SCHEME).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoConfigAttributes_t::CUBLASLT_ALGO_CONFIG_CTA_SWIZZLING => {
                writer
                    .write_all(stringify!(CUBLASLT_ALGO_CONFIG_CTA_SWIZZLING).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoConfigAttributes_t::CUBLASLT_ALGO_CONFIG_CUSTOM_OPTION => {
                writer
                    .write_all(stringify!(CUBLASLT_ALGO_CONFIG_CUSTOM_OPTION).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoConfigAttributes_t::CUBLASLT_ALGO_CONFIG_STAGES_ID => {
                writer.write_all(stringify!(CUBLASLT_ALGO_CONFIG_STAGES_ID).as_bytes())
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoConfigAttributes_t::CUBLASLT_ALGO_CONFIG_INNER_SHAPE_ID => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CONFIG_INNER_SHAPE_ID).as_bytes(),
                    )
            }
            &cuda_types::cublaslt::cublasLtMatmulAlgoConfigAttributes_t::CUBLASLT_ALGO_CONFIG_CLUSTER_SHAPE_ID => {
                writer
                    .write_all(
                        stringify!(CUBLASLT_ALGO_CONFIG_CLUSTER_SHAPE_ID).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cublasLtMatmulAlgoConfigSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    algo: *mut cuda_types::cublaslt::cublasLtMatmulAlgo_t,
    attr: cuda_types::cublaslt::cublasLtMatmulAlgoConfigAttributes_t,
    buf: *const ::core::ffi::c_void,
    sizeInBytes: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algo,
        "cublasLtMatmulAlgoConfigSetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attr), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attr,
        "cublasLtMatmulAlgoConfigSetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buf), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &buf,
        "cublasLtMatmulAlgoConfigSetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cublasLtMatmulAlgoConfigSetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatmulAlgoConfigGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    algo: *const cuda_types::cublaslt::cublasLtMatmulAlgo_t,
    attr: cuda_types::cublaslt::cublasLtMatmulAlgoConfigAttributes_t,
    buf: *mut ::core::ffi::c_void,
    sizeInBytes: usize,
    sizeWritten: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algo,
        "cublasLtMatmulAlgoConfigGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attr), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attr,
        "cublasLtMatmulAlgoConfigGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buf), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &buf,
        "cublasLtMatmulAlgoConfigGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cublasLtMatmulAlgoConfigGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeWritten), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeWritten,
        "cublasLtMatmulAlgoConfigGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtLoggerSetCallback(
    writer: &mut (impl std::io::Write + ?Sized),
    callback: cuda_types::cublaslt::cublasLtLoggerCallback_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(callback), ": ").as_bytes())?;
    crate::CudaDisplay::write(&callback, "cublasLtLoggerSetCallback", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLoggerSetFile(
    writer: &mut (impl std::io::Write + ?Sized),
    file: *mut cuda_types::FILE,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(file), ": ").as_bytes())?;
    crate::CudaDisplay::write(&file, "cublasLtLoggerSetFile", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLoggerOpenFile(
    writer: &mut (impl std::io::Write + ?Sized),
    logFile: *const ::core::ffi::c_char,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(logFile), ": ").as_bytes())?;
    crate::CudaDisplay::write(&logFile, "cublasLtLoggerOpenFile", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLoggerSetLevel(
    writer: &mut (impl std::io::Write + ?Sized),
    level: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(level), ": ").as_bytes())?;
    crate::CudaDisplay::write(&level, "cublasLtLoggerSetLevel", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLoggerSetMask(
    writer: &mut (impl std::io::Write + ?Sized),
    mask: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(mask), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mask, "cublasLtLoggerSetMask", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLoggerForceDisable(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
