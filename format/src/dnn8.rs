#[path = "format_generated_dnn8.rs"]
mod format_generated_dnn8;
pub use format_generated_dnn8::*;

pub use crate::dnn9::write_cudnnBackendSetAttribute;
pub use crate::dnn9::write_cudnnBackendGetAttribute;
pub use crate::dnn9::write_cudnnSetTensorNdDescriptor;
pub use crate::dnn9::write_cudnnSetFilterNdDescriptor;
pub use crate::dnn9::write_cudnnSetConvolutionNdDescriptor;