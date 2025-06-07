// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
pub fn write_cublasLtShutdownCtx(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_long,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtShutdownCtx", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtCtxInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_long,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtCtxInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtHeuristicLutSerializeEntry(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: *mut ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: *mut ::core::ffi::c_ulonglong,
    param_13: *mut ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtHeuristicLutSerializeEntry",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtHeuristicLutSerializeEntry",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtHeuristicLutSerializeEntry",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtHeuristicLutSerializeEntry",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtHeuristicLutSerializeEntry",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtHeuristicLutSerializeEntry",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtHeuristicLutSerializeEntry",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtHeuristicLutSerializeEntry",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtHeuristicLutSerializeEntry",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtHeuristicLutSerializeEntry",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_11,
        "cublasLtHeuristicLutSerializeEntry",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_12,
        "cublasLtHeuristicLutSerializeEntry",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_13,
        "cublasLtHeuristicLutSerializeEntry",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_14,
        "cublasLtHeuristicLutSerializeEntry",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_15,
        "cublasLtHeuristicLutSerializeEntry",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtLegacyGemmACC(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: *mut ::core::ffi::c_int,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_uint,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_uint,
    param_14: ::core::ffi::c_uint,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_uint,
    param_17: ::core::ffi::c_uint,
    param_18: ::core::ffi::c_ulonglong,
    param_19: ::core::ffi::c_uint,
    param_20: ::core::ffi::c_uint,
    param_21: ::core::ffi::c_uint,
    param_22: ::core::ffi::c_uint,
    param_23: ::core::ffi::c_uint,
    param_24: ::core::ffi::c_uchar,
    param_25: ::core::ffi::c_ulonglong,
    param_26: ::core::ffi::c_ulonglong,
    param_27: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_17), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_17, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_18), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_18, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_19), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_19, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_20), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_20, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_21), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_21, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_22), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_22, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_23), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_23, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_24), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_24, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_25), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_25, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_26), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_26, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_27), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_27, "cublasLtLegacyGemmACC", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLegacyGemmBII(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: *mut ::core::ffi::c_int,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_uint,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_uint,
    param_14: ::core::ffi::c_uint,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_uint,
    param_17: ::core::ffi::c_uint,
    param_18: ::core::ffi::c_ulonglong,
    param_19: ::core::ffi::c_uint,
    param_20: ::core::ffi::c_uint,
    param_21: ::core::ffi::c_uint,
    param_22: ::core::ffi::c_uint,
    param_23: ::core::ffi::c_uint,
    param_24: ::core::ffi::c_uchar,
    param_25: ::core::ffi::c_ulonglong,
    param_26: ::core::ffi::c_ulonglong,
    param_27: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_17), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_17, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_18), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_18, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_19), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_19, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_20), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_20, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_21), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_21, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_22), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_22, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_23), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_23, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_24), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_24, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_25), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_25, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_26), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_26, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_27), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_27, "cublasLtLegacyGemmBII", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLegacyGemmBSS(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: *mut ::core::ffi::c_int,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_uint,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_uint,
    param_14: ::core::ffi::c_uint,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_uint,
    param_17: ::core::ffi::c_uint,
    param_18: ::core::ffi::c_ulonglong,
    param_19: ::core::ffi::c_uint,
    param_20: ::core::ffi::c_uint,
    param_21: ::core::ffi::c_uint,
    param_22: ::core::ffi::c_uint,
    param_23: ::core::ffi::c_uint,
    param_24: ::core::ffi::c_uchar,
    param_25: ::core::ffi::c_ulonglong,
    param_26: ::core::ffi::c_ulonglong,
    param_27: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_17), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_17, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_18), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_18, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_19), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_19, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_20), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_20, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_21), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_21, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_22), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_22, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_23), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_23, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_24), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_24, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_25), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_25, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_26), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_26, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_27), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_27, "cublasLtLegacyGemmBSS", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLegacyGemmCCC(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: *mut ::core::ffi::c_int,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_uint,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_uint,
    param_14: ::core::ffi::c_uint,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_uint,
    param_17: ::core::ffi::c_uint,
    param_18: ::core::ffi::c_ulonglong,
    param_19: ::core::ffi::c_uint,
    param_20: ::core::ffi::c_uint,
    param_21: ::core::ffi::c_uint,
    param_22: ::core::ffi::c_uint,
    param_23: ::core::ffi::c_uint,
    param_24: ::core::ffi::c_uchar,
    param_25: ::core::ffi::c_ulonglong,
    param_26: ::core::ffi::c_ulonglong,
    param_27: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_17), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_17, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_18), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_18, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_19), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_19, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_20), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_20, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_21), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_21, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_22), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_22, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_23), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_23, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_24), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_24, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_25), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_25, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_26), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_26, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_27), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_27, "cublasLtLegacyGemmCCC", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLegacyGemmUtilizationCCC(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_uint,
    param_10: ::core::ffi::c_uint,
    param_11: ::core::ffi::c_uint,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtLegacyGemmUtilizationCCC",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtLegacyGemmUtilizationCCC",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtLegacyGemmUtilizationCCC",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtLegacyGemmUtilizationCCC",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtLegacyGemmUtilizationCCC",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtLegacyGemmUtilizationCCC",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtLegacyGemmUtilizationCCC",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtLegacyGemmUtilizationCCC",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtLegacyGemmUtilizationCCC",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtLegacyGemmUtilizationCCC",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_11,
        "cublasLtLegacyGemmUtilizationCCC",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_12,
        "cublasLtLegacyGemmUtilizationCCC",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_13,
        "cublasLtLegacyGemmUtilizationCCC",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtLegacyGemmDDD(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: *mut ::core::ffi::c_int,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_uint,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_uint,
    param_14: ::core::ffi::c_uint,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_uint,
    param_17: ::core::ffi::c_uint,
    param_18: ::core::ffi::c_ulonglong,
    param_19: ::core::ffi::c_uint,
    param_20: ::core::ffi::c_uint,
    param_21: ::core::ffi::c_uint,
    param_22: ::core::ffi::c_uint,
    param_23: ::core::ffi::c_uint,
    param_24: ::core::ffi::c_uchar,
    param_25: ::core::ffi::c_ulonglong,
    param_26: ::core::ffi::c_ulonglong,
    param_27: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_17), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_17, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_18), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_18, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_19), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_19, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_20), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_20, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_21), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_21, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_22), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_22, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_23), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_23, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_24), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_24, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_25), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_25, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_26), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_26, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_27), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_27, "cublasLtLegacyGemmDDD", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLegacyGemmUtilizationDDD(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_uint,
    param_10: ::core::ffi::c_uint,
    param_11: ::core::ffi::c_uint,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtLegacyGemmUtilizationDDD",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtLegacyGemmUtilizationDDD",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtLegacyGemmUtilizationDDD",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtLegacyGemmUtilizationDDD",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtLegacyGemmUtilizationDDD",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtLegacyGemmUtilizationDDD",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtLegacyGemmUtilizationDDD",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtLegacyGemmUtilizationDDD",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtLegacyGemmUtilizationDDD",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtLegacyGemmUtilizationDDD",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_11,
        "cublasLtLegacyGemmUtilizationDDD",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_12,
        "cublasLtLegacyGemmUtilizationDDD",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_13,
        "cublasLtLegacyGemmUtilizationDDD",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtLegacyGemmHHH(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: *mut ::core::ffi::c_int,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_uint,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_uint,
    param_14: ::core::ffi::c_uint,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_uint,
    param_17: ::core::ffi::c_uint,
    param_18: ::core::ffi::c_ulonglong,
    param_19: ::core::ffi::c_uint,
    param_20: ::core::ffi::c_uint,
    param_21: ::core::ffi::c_uint,
    param_22: ::core::ffi::c_uint,
    param_23: ::core::ffi::c_uint,
    param_24: ::core::ffi::c_uchar,
    param_25: ::core::ffi::c_ulonglong,
    param_26: ::core::ffi::c_ulonglong,
    param_27: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_17), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_17, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_18), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_18, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_19), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_19, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_20), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_20, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_21), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_21, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_22), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_22, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_23), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_23, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_24), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_24, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_25), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_25, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_26), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_26, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_27), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_27, "cublasLtLegacyGemmHHH", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLegacyGemmHSS(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: *mut ::core::ffi::c_int,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_uint,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_uint,
    param_14: ::core::ffi::c_uint,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_uint,
    param_17: ::core::ffi::c_uint,
    param_18: ::core::ffi::c_ulonglong,
    param_19: ::core::ffi::c_uint,
    param_20: ::core::ffi::c_uint,
    param_21: ::core::ffi::c_uint,
    param_22: ::core::ffi::c_uint,
    param_23: ::core::ffi::c_uint,
    param_24: ::core::ffi::c_uchar,
    param_25: ::core::ffi::c_ulonglong,
    param_26: ::core::ffi::c_ulonglong,
    param_27: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_17), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_17, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_18), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_18, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_19), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_19, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_20), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_20, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_21), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_21, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_22), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_22, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_23), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_23, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_24), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_24, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_25), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_25, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_26), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_26, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_27), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_27, "cublasLtLegacyGemmHSS", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLegacyGemmHSH(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: *mut ::core::ffi::c_int,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_uint,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_uint,
    param_14: ::core::ffi::c_uint,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_uint,
    param_17: ::core::ffi::c_uint,
    param_18: ::core::ffi::c_ulonglong,
    param_19: ::core::ffi::c_uint,
    param_20: ::core::ffi::c_uint,
    param_21: ::core::ffi::c_uint,
    param_22: ::core::ffi::c_uint,
    param_23: ::core::ffi::c_uint,
    param_24: ::core::ffi::c_uchar,
    param_25: ::core::ffi::c_ulonglong,
    param_26: ::core::ffi::c_ulonglong,
    param_27: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_17), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_17, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_18), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_18, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_19), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_19, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_20), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_20, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_21), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_21, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_22), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_22, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_23), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_23, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_24), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_24, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_25), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_25, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_26), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_26, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_27), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_27, "cublasLtLegacyGemmHSH", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLegacyGemmSSS(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: *mut ::core::ffi::c_int,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_uint,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_uint,
    param_14: ::core::ffi::c_uint,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_uint,
    param_17: ::core::ffi::c_uint,
    param_18: ::core::ffi::c_ulonglong,
    param_19: ::core::ffi::c_uint,
    param_20: ::core::ffi::c_uint,
    param_21: ::core::ffi::c_uint,
    param_22: ::core::ffi::c_uint,
    param_23: ::core::ffi::c_uint,
    param_24: ::core::ffi::c_uchar,
    param_25: ::core::ffi::c_ulonglong,
    param_26: ::core::ffi::c_ulonglong,
    param_27: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_17), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_17, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_18), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_18, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_19), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_19, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_20), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_20, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_21), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_21, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_22), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_22, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_23), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_23, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_24), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_24, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_25), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_25, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_26), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_26, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_27), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_27, "cublasLtLegacyGemmSSS", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLegacyGemmTSS(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: *mut ::core::ffi::c_int,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_uint,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_uint,
    param_14: ::core::ffi::c_uint,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_uint,
    param_17: ::core::ffi::c_uint,
    param_18: ::core::ffi::c_ulonglong,
    param_19: ::core::ffi::c_uint,
    param_20: ::core::ffi::c_uint,
    param_21: ::core::ffi::c_uint,
    param_22: ::core::ffi::c_uint,
    param_23: ::core::ffi::c_uint,
    param_24: ::core::ffi::c_uchar,
    param_25: ::core::ffi::c_ulonglong,
    param_26: ::core::ffi::c_ulonglong,
    param_27: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_17), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_17, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_18), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_18, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_19), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_19, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_20), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_20, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_21), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_21, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_22), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_22, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_23), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_23, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_24), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_24, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_25), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_25, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_26), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_26, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_27), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_27, "cublasLtLegacyGemmTSS", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLegacyGemmTST(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: *mut ::core::ffi::c_int,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_uint,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_uint,
    param_14: ::core::ffi::c_uint,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_uint,
    param_17: ::core::ffi::c_uint,
    param_18: ::core::ffi::c_ulonglong,
    param_19: ::core::ffi::c_uint,
    param_20: ::core::ffi::c_uint,
    param_21: ::core::ffi::c_uint,
    param_22: ::core::ffi::c_uint,
    param_23: ::core::ffi::c_uint,
    param_24: ::core::ffi::c_uchar,
    param_25: ::core::ffi::c_ulonglong,
    param_26: ::core::ffi::c_ulonglong,
    param_27: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_17), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_17, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_18), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_18, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_19), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_19, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_20), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_20, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_21), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_21, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_22), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_22, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_23), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_23, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_24), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_24, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_25), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_25, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_26), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_26, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_27), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_27, "cublasLtLegacyGemmTST", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLegacyGemmZZZ(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: *mut ::core::ffi::c_int,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_uint,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_uint,
    param_14: ::core::ffi::c_uint,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_uint,
    param_17: ::core::ffi::c_uint,
    param_18: ::core::ffi::c_ulonglong,
    param_19: ::core::ffi::c_uint,
    param_20: ::core::ffi::c_uint,
    param_21: ::core::ffi::c_uint,
    param_22: ::core::ffi::c_uint,
    param_23: ::core::ffi::c_uint,
    param_24: ::core::ffi::c_uchar,
    param_25: ::core::ffi::c_ulonglong,
    param_26: ::core::ffi::c_ulonglong,
    param_27: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_17), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_17, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_18), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_18, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_19), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_19, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_20), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_20, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_21), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_21, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_22), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_22, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_23), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_23, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_24), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_24, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_25), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_25, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_26), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_26, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_27), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_27, "cublasLtLegacyGemmZZZ", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtLegacyGemmUtilizationZZZ(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_uint,
    param_10: ::core::ffi::c_uint,
    param_11: ::core::ffi::c_uint,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtLegacyGemmUtilizationZZZ",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtLegacyGemmUtilizationZZZ",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtLegacyGemmUtilizationZZZ",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtLegacyGemmUtilizationZZZ",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtLegacyGemmUtilizationZZZ",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtLegacyGemmUtilizationZZZ",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtLegacyGemmUtilizationZZZ",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtLegacyGemmUtilizationZZZ",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtLegacyGemmUtilizationZZZ",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtLegacyGemmUtilizationZZZ",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_11,
        "cublasLtLegacyGemmUtilizationZZZ",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_12,
        "cublasLtLegacyGemmUtilizationZZZ",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_13,
        "cublasLtLegacyGemmUtilizationZZZ",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtAlgoCharacteristicGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_int,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtAlgoCharacteristicGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtAlgoCharacteristicGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtAlgoCharacteristicGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtAlgoCharacteristicGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtAlgoCharacteristicGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtAlgoCharacteristicGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtAlgoCharacteristicGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtAlgoCharacteristicGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtMatmulAlgoConfigGetAttributeRange(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cublasLtHHHMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtHHHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtHHHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtHHHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtHHHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtHHHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtHHHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtHHHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtHHHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtHHHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtHHHMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtHHHMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtHHHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtHHHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtHHHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtHHHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtHHHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtHHHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtHHHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtHHHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtHHHMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtHHHMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtHHHMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtHHHMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtHHHMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtHHHMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtHHHMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtHHHMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtHHHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtHHHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtHHHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtHHHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtHHHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtHHHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtHHHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtHHHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtHHHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtHHHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtHHHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtHHHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtHHHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtHHHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtHHHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtHHHMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtHHHMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtHHHMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtHHHMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtHHHMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtHHHMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtHHHMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtHHHMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtHHHMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtHHHMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtHHHMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtHHHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtHHHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtHHHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtHHHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtHHHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtHHHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtHHHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtHHHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtHHHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtHHHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtHSHMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtHSHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtHSHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtHSHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtHSHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtHSHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtHSHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtHSHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtHSHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtHSHMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtHSHMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtHSHMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtHSHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtHSHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtHSHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtHSHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtHSHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtHSHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtHSHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtHSHMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtHSHMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtHSHMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtHSHMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtHSHMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtHSHMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtHSHMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtHSHMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtHSHMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtHSHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtHSHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtHSHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtHSHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtHSHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtHSHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtHSHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtHSHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtHSHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtHSHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtHSHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtHSHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtHSHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtHSHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtHSHMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtHSHMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtHSHMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtHSHMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtHSHMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtHSHMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtHSHMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtHSHMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtHSHMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtHSHMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtHSHMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtHSHMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtHSHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtHSHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtHSHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtHSHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtHSHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtHSHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtHSHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtHSHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtHSHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtHSHMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtSSSMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtSSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtSSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtSSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtSSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtSSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtSSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtSSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtSSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtSSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtSSSMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtSSSMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtSSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtSSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtSSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtSSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtSSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtSSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtSSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtSSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtSSSMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtSSSMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtSSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtSSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtSSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtSSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtSSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtSSSMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtSSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtSSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtSSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtSSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtSSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtSSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtSSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtSSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtSSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtSSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtSSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtSSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtSSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtSSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtSSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtSSSMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtSSSMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtSSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtSSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtSSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtSSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtSSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtSSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtSSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtSSSMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtSSSMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtSSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtSSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtSSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtSSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtSSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtSSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtSSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtSSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtSSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtSSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtDDDMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtDDDMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtDDDMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtDDDMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtDDDMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtDDDMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtDDDMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtDDDMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtDDDMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtDDDMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtDDDMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtDDDMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtDDDMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtDDDMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtDDDMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtDDDMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtDDDMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtDDDMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtDDDMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtDDDMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtDDDMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtDDDMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtDDDMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtDDDMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtDDDMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtDDDMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtDDDMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtDDDMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtDDDMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtDDDMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtDDDMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtDDDMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtDDDMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtDDDMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtDDDMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtDDDMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtDDDMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtDDDMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtDDDMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtDDDMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtDDDMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtDDDMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtDDDMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtDDDMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtDDDMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtDDDMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtDDDMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtDDDMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtDDDMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtDDDMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtDDDMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtDDDMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtDDDMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtDDDMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtDDDMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtDDDMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtDDDMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtDDDMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtDDDMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtDDDMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtDDDMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtDDDMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtDDDMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtDDDMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtBSSMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtBSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtBSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtBSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtBSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtBSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtBSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtBSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtBSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtBSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtBSSMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtBSSMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtBSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtBSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtBSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtBSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtBSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtBSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtBSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtBSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtBSSMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtBSSMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtBSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtBSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtBSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtBSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtBSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtBSSMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtBSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtBSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtBSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtBSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtBSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtBSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtBSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtBSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtBSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtBSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtBSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtBSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtBSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtBSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtBSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtBSSMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtBSSMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtBSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtBSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtBSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtBSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtBSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtBSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtBSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtBSSMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtBSSMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtBSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtBSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtBSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtBSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtBSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtBSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtBSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtBSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtBSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtBSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtCCCMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtCCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtCCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtCCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtCCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtCCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtCCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtCCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtCCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtCCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtCCCMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtCCCMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtCCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtCCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtCCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtCCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtCCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtCCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtCCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtCCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtCCCMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtCCCMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtCCCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtCCCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtCCCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtCCCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtCCCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtCCCMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtCCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtCCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtCCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtCCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtCCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtCCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtCCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtCCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtCCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtCCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtCCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtCCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtCCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtCCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtCCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtCCCMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtCCCMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtCCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtCCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtCCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtCCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtCCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtCCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtCCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtCCCMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtCCCMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtCCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtCCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtCCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtCCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtCCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtCCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtCCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtCCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtCCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtCCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtZZZMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtZZZMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtZZZMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtZZZMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtZZZMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtZZZMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtZZZMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtZZZMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtZZZMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtZZZMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtZZZMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtZZZMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtZZZMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtZZZMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtZZZMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtZZZMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtZZZMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtZZZMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtZZZMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtZZZMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtZZZMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtZZZMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtZZZMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtZZZMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtZZZMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtZZZMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtZZZMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtZZZMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtZZZMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtZZZMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtZZZMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtZZZMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtZZZMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtZZZMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtZZZMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtZZZMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtZZZMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtZZZMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtZZZMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtZZZMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtZZZMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtZZZMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtZZZMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtZZZMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtZZZMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtZZZMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtZZZMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtZZZMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtZZZMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtZZZMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtZZZMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtZZZMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtZZZMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtZZZMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtZZZMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtZZZMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtZZZMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtZZZMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtZZZMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtZZZMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtZZZMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtZZZMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtZZZMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtZZZMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtACCMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtACCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtACCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtACCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtACCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtACCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtACCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtACCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtACCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtACCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtACCMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtACCMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtACCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtACCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtACCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtACCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtACCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtACCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtACCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtACCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtACCMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtACCMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtACCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtACCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtACCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtACCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtACCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtACCMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtACCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtACCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtACCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtACCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtACCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtACCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtACCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtACCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtACCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtACCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtACCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtACCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtACCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtACCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtACCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtACCMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtACCMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtACCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtACCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtACCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtACCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtACCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtACCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtACCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtACCMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtACCMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtACCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtACCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtACCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtACCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtACCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtACCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtACCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtACCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtACCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtACCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtBIIMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtBIIMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtBIIMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtBIIMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtBIIMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtBIIMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtBIIMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtBIIMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtBIIMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtBIIMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtBIIMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtBIIMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtBIIMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtBIIMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtBIIMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtBIIMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtBIIMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtBIIMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtBIIMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtBIIMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtBIIMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtBIIMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtBIIMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtBIIMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtBIIMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtBIIMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtBIIMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtBIIMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtBIIMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtBIIMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtBIIMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtBIIMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtBIIMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtBIIMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtBIIMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtBIIMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtBIIMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtBIIMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtBIIMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtBIIMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtBIIMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtBIIMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtBIIMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtBIIMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtBIIMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtBIIMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtBIIMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtBIIMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtBIIMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtBIIMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtBIIMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtBIIMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtBIIMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtBIIMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtBIIMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtBIIMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtBIIMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtBIIMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtBIIMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtBIIMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtBIIMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtBIIMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtBIIMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtBIIMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtBSBMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtBSBMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtBSBMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtBSBMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtBSBMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtBSBMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtBSBMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtBSBMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtBSBMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtBSBMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtBSBMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtBSBMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtBSBMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtBSBMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtBSBMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtBSBMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtBSBMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtBSBMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtBSBMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtBSBMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtBSBMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtBSBMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtBSBMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtBSBMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtBSBMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtBSBMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtBSBMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtBSBMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtBSBMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtBSBMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtBSBMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtBSBMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtBSBMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtBSBMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtBSBMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtBSBMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtBSBMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtBSBMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtBSBMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtBSBMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtBSBMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtBSBMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtBSBMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtBSBMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtBSBMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtBSBMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtBSBMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtBSBMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtBSBMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtBSBMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtBSBMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtBSBMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtBSBMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtBSBMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtBSBMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtBSBMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtBSBMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtBSBMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtBSBMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtBSBMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtBSBMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtBSBMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtBSBMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtBSBMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtHSSMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtHSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtHSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtHSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtHSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtHSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtHSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtHSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtHSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtHSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtHSSMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtHSSMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtHSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtHSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtHSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtHSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtHSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtHSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtHSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtHSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtHSSMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtHSSMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtHSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtHSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtHSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtHSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtHSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtHSSMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtHSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtHSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtHSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtHSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtHSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtHSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtHSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtHSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtHSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtHSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtHSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtHSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtHSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtHSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtHSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtHSSMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtHSSMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtHSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtHSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtHSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtHSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtHSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtHSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtHSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtHSSMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtHSSMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtHSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtHSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtHSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtHSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtHSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtHSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtHSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtHSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtHSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtHSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtKCCMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtKCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtKCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtKCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtKCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtKCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtKCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtKCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtKCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtKCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtKCCMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtKCCMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtKCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtKCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtKCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtKCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtKCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtKCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtKCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtKCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtKCCMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtKCCMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtKCCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtKCCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtKCCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtKCCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtKCCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtKCCMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtKCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtKCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtKCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtKCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtKCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtKCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtKCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtKCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtKCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtKCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtKCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtKCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtKCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtKCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtKCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtKCCMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtKCCMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtKCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtKCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtKCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtKCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtKCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtKCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtKCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtKCCMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtKCCMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtKCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtKCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtKCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtKCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtKCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtKCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtKCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtKCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtKCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtKCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtKCKMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtKCKMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtKCKMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtKCKMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtKCKMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtKCKMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtKCKMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtKCKMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtKCKMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtKCKMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtKCKMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtKCKMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtKCKMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtKCKMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtKCKMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtKCKMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtKCKMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtKCKMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtKCKMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtKCKMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtKCKMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtKCKMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtKCKMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtKCKMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtKCKMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtKCKMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtKCKMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtKCKMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtKCKMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtKCKMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtKCKMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtKCKMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtKCKMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtKCKMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtKCKMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtKCKMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtKCKMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtKCKMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtKCKMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtKCKMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtKCKMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtKCKMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtKCKMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtKCKMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtKCKMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtKCKMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtKCKMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtKCKMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtKCKMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtKCKMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtKCKMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtKCKMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtKCKMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtKCKMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtKCKMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtKCKMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtKCKMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtKCKMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtKCKMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtKCKMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtKCKMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtKCKMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtKCKMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtKCKMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtTSSMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtTSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtTSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtTSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtTSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtTSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtTSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtTSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtTSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtTSSMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtTSSMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtTSSMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtTSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtTSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtTSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtTSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtTSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtTSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtTSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtTSSMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtTSSMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtTSSMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtTSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtTSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtTSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtTSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtTSSMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtTSSMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtTSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtTSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtTSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtTSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtTSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtTSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtTSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtTSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtTSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtTSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtTSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtTSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtTSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtTSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtTSSMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtTSSMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtTSSMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtTSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtTSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtTSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtTSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtTSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtTSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtTSSMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtTSSMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtTSSMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtTSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtTSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtTSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtTSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtTSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtTSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtTSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtTSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtTSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtTSSMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtTSTMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtTSTMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtTSTMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtTSTMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtTSTMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtTSTMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtTSTMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtTSTMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtTSTMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtTSTMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtTSTMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtTSTMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtTSTMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtTSTMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtTSTMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtTSTMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtTSTMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtTSTMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtTSTMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtTSTMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtTSTMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtTSTMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtTSTMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtTSTMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtTSTMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtTSTMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtTSTMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtTSTMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtTSTMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtTSTMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtTSTMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtTSTMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtTSTMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtTSTMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtTSTMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtTSTMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtTSTMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtTSTMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtTSTMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtTSTMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtTSTMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtTSTMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtTSTMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtTSTMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtTSTMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtTSTMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtTSTMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtTSTMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtTSTMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtTSTMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtTSTMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtTSTMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtTSTMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtTSTMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtTSTMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtTSTMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtTSTMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtTSTMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtTSTMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtTSTMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtTSTMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtTSTMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtTSTMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtTSTMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtVCCMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtVCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtVCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtVCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtVCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtVCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtVCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtVCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtVCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtVCCMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtVCCMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtVCCMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtVCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtVCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtVCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtVCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtVCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtVCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtVCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtVCCMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtVCCMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtVCCMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtVCCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtVCCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtVCCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtVCCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtVCCMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtVCCMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtVCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtVCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtVCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtVCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtVCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtVCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtVCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtVCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtVCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtVCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtVCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtVCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtVCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtVCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtVCCMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtVCCMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtVCCMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtVCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtVCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtVCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtVCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtVCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtVCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtVCCMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtVCCMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtVCCMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtVCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtVCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtVCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtVCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtVCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtVCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtVCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtVCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtVCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtVCCMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtVCVMatmulAlgoGetIds(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_int,
    param_5: ::core::ffi::c_int,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_int,
    param_9: ::core::ffi::c_long,
    param_10: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtVCVMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtVCVMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtVCVMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtVCVMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtVCVMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtVCVMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtVCVMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtVCVMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtVCVMatmulAlgoGetIds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtVCVMatmulAlgoGetIds",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtVCVMatmulAlgoInit(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_uint,
    param_4: ::core::ffi::c_uint,
    param_5: ::core::ffi::c_uint,
    param_6: ::core::ffi::c_uint,
    param_7: ::core::ffi::c_uint,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtVCVMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtVCVMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtVCVMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtVCVMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtVCVMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtVCVMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtVCVMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtVCVMatmulAlgoInit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtVCVMatmulAlgoInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtVCVMatmulAlgoCapGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_uint,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtVCVMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtVCVMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtVCVMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtVCVMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtVCVMatmulAlgoCapGetAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLtVCVMatmul(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_ulonglong,
    param_3: ::core::ffi::c_ulonglong,
    param_4: ::core::ffi::c_ulonglong,
    param_5: ::core::ffi::c_ulonglong,
    param_6: ::core::ffi::c_ulonglong,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
    param_11: ::core::ffi::c_ulonglong,
    param_12: ::core::ffi::c_ulonglong,
    param_13: ::core::ffi::c_ulonglong,
    param_14: ::core::ffi::c_ulonglong,
    param_15: ::core::ffi::c_ulonglong,
    param_16: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtVCVMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtVCVMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtVCVMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtVCVMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtVCVMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtVCVMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtVCVMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtVCVMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_9, "cublasLtVCVMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_10, "cublasLtVCVMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_11), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_11, "cublasLtVCVMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_12), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_12, "cublasLtVCVMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_13), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_13, "cublasLtVCVMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_14), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_14, "cublasLtVCVMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_15), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_15, "cublasLtVCVMatmul", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_16), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_16, "cublasLtVCVMatmul", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtVCVMatmulAlgoCheck(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_1, "cublasLtVCVMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_2, "cublasLtVCVMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_3, "cublasLtVCVMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_4, "cublasLtVCVMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_5, "cublasLtVCVMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_6, "cublasLtVCVMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_7, "cublasLtVCVMatmulAlgoCheck", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(&param_8, "cublasLtVCVMatmulAlgoCheck", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cublasLtVCVMatmulAlgoGetHeuristic(
    writer: &mut (impl std::io::Write + ?Sized),
    param_1: ::core::ffi::c_ulonglong,
    param_2: ::core::ffi::c_long,
    param_3: ::core::ffi::c_long,
    param_4: ::core::ffi::c_long,
    param_5: ::core::ffi::c_long,
    param_6: ::core::ffi::c_long,
    param_7: *mut ::core::ffi::c_ulonglong,
    param_8: ::core::ffi::c_uint,
    param_9: ::core::ffi::c_ulonglong,
    param_10: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(param_1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_1,
        "cublasLtVCVMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_2,
        "cublasLtVCVMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_3), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_3,
        "cublasLtVCVMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_4), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_4,
        "cublasLtVCVMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_5), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_5,
        "cublasLtVCVMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_6), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_6,
        "cublasLtVCVMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_7), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_7,
        "cublasLtVCVMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_8), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_8,
        "cublasLtVCVMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_9), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_9,
        "cublasLtVCVMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param_10), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param_10,
        "cublasLtVCVMatmulAlgoGetHeuristic",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cublasLt_for_cublas_BII(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cublasLt_for_cublas_BSS(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cublasLt_for_cublas_CCC(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cublasLt_for_cublas_DDD(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cublasLt_for_cublas_HHH(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cublasLt_for_cublas_HSH(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cublasLt_for_cublas_HSS(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cublasLt_for_cublas_SSS(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cublasLt_for_cublas_TSS(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cublasLt_for_cublas_TST(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cublasLt_for_cublas_ZZZ(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
