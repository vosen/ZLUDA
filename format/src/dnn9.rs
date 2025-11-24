use std::{ffi::c_void, slice};

#[path = "format_generated_dnn9.rs"]
mod format_generated_dnn9;
pub use format_generated_dnn9::*;

#[allow(non_snake_case)]
pub fn write_cudnnBackendGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    descriptor: cuda_types::cudnn9::cudnnBackendDescriptor_t,
    attributeName: cuda_types::cudnn9::cudnnBackendAttributeName_t,
    attributeType: cuda_types::cudnn9::cudnnBackendAttributeType_t,
    requestedElementCount: i64,
    elementCount: *mut i64,
    arrayOfElements: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(descriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(&descriptor, "cudnnBackendGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attributeName), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attributeName, "cudnnBackendGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attributeType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attributeType, "cudnnBackendGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedElementCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedElementCount,
        "cudnnBackendGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(elementCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&elementCount, "cudnnBackendGetAttribute", arg_idx, writer)?;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(arrayOfElements), ": ").as_bytes())?;
    cudnn9_print_elements(
        writer,
        attributeType,
        unsafe { elementCount.as_ref() }
            .copied()
            .unwrap_or(requestedElementCount),
        arrayOfElements,
    )?;
    writer.write_all(b")")
}

pub fn cudnn9_print_elements(
    writer: &mut (impl std::io::Write + ?Sized),
    type_: cuda_types::cudnn9::cudnnBackendAttributeType_t,
    element_count: i64,
    array_of_elements: *const ::core::ffi::c_void,
) -> std::io::Result<()> {
    fn print_typed<T: crate::CudaDisplay>(
        writer: &mut (impl std::io::Write + ?Sized),
        element_count: i64,
        array_of_elements: *const ::core::ffi::c_void,
    ) -> std::io::Result<()> {
        if array_of_elements.is_null() {
            return writer.write_all(b"NULL");
        }
        let elements =
            unsafe { slice::from_raw_parts(array_of_elements as *const T, element_count as usize) };
        crate::CudaDisplay::write(elements, "", 0, writer)
    }
    match type_ {
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_HANDLE => {
            print_typed::<cuda_types::cudnn9::cudnnHandle_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_DATA_TYPE => {
            print_typed::<cuda_types::cudnn9::cudnnDataType_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_BOOLEAN => {
            print_typed::<bool>(writer, element_count, array_of_elements)
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_INT64 => {
            print_typed::<i64>(writer, element_count, array_of_elements)
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_FLOAT => {
            print_typed::<f32>(writer, element_count, array_of_elements)
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_DOUBLE => {
            print_typed::<f64>(writer, element_count, array_of_elements)
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_VOID_PTR => {
            print_typed::<*const c_void>(writer, element_count, array_of_elements)
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_CONVOLUTION_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnConvolutionMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_HEUR_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendHeurMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_KNOB_TYPE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendKnobType_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_NAN_PROPOGATION => {
            print_typed::<cuda_types::cudnn9::cudnnNanPropagation_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_NUMERICAL_NOTE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendNumericalNote_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_LAYOUT_TYPE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendLayoutType_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_ATTRIB_NAME => {
            print_typed::<cuda_types::cudnn9::cudnnBackendAttributeName_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_POINTWISE_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnPointwiseMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_BACKEND_DESCRIPTOR => {
            print_typed::<cuda_types::cudnn9::cudnnBackendDescriptor_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_GENSTATS_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnGenStatsMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_BN_FINALIZE_STATS_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnBnFinalizeStatsMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_REDUCTION_OPERATOR_TYPE => {
            print_typed::<cuda_types::cudnn9::cudnnReduceTensorOp_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_BEHAVIOR_NOTE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendBehaviorNote_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_TENSOR_REORDERING_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendTensorReordering_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_RESAMPLE_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnResampleMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_PADDING_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnPaddingMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_INT32 => {
            print_typed::<i32>(writer, element_count, array_of_elements)
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_CHAR => {
            crate::CudaDisplay::write(&array_of_elements.cast::<i8>(), "", 0, writer)
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_SIGNAL_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnSignalMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_FRACTION => {
            print_typed::<cuda_types::cudnn9::cudnnFraction_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_NORM_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendNormMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_NORM_FWD_PHASE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendNormFwdPhase_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_RNG_DISTRIBUTION => {
            print_typed::<cuda_types::cudnn9::cudnnRngDistribution_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        _ => unimplemented!(),
    }
}

#[allow(non_snake_case)]
pub fn write_cudnnBackendSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    descriptor: cuda_types::cudnn9::cudnnBackendDescriptor_t,
    attributeName: cuda_types::cudnn9::cudnnBackendAttributeName_t,
    attributeType: cuda_types::cudnn9::cudnnBackendAttributeType_t,
    elementCount: i64,
    arrayOfElements: *const ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(descriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(&descriptor, "cudnnBackendSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attributeName), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attributeName, "cudnnBackendSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attributeType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attributeType, "cudnnBackendSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(elementCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&elementCount, "cudnnBackendSetAttribute", arg_idx, writer)?;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(arrayOfElements), ": ").as_bytes())?;
    cudnn9_print_elements(writer, attributeType, elementCount, arrayOfElements)?;
    writer.write_all(b")")
}
