use std::io;
use std::path::PathBuf;
use std::str::Utf8Error;

use amd_comgr_sys::amd_comgr_status_s;
use hip_runtime_sys::hipErrorCode_t;
use ptx::TranslateError;
use ptx_parser::PtxError;

#[derive(Debug, thiserror::Error)]
pub enum CompilerError {
    #[error("HIP error: {0:?}")]
    HipError(hipErrorCode_t),
    #[error("amd_comgr error: {0:?}")]
    ComgrError(amd_comgr_status_s),
    #[error("Not a regular file: {0}")]
    CheckPathError(PathBuf),
    #[error("Invalid output type: {0}")]
    ParseOutputTypeError(String),
    #[error("Error parsing PTX: {0}")]
    PtxParserError(String),
    #[error("Error translating PTX: {0:?}")]
    PtxTranslateError(TranslateError),
    #[error("IO error: {0:?}")]
    IoError(io::Error),
    #[error("Error parsing file: {0:?}")]
    ParseFileError(Utf8Error),
}

impl From<hipErrorCode_t> for CompilerError {
    fn from(error_code: hipErrorCode_t) -> Self {
        CompilerError::HipError(error_code)
    }
}

impl From<amd_comgr_status_s> for CompilerError {
    fn from(error_code: amd_comgr_status_s) -> Self {
        CompilerError::ComgrError(error_code)
    }
}

impl From<Vec<PtxError<'_>>> for CompilerError {
    fn from(causes: Vec<PtxError>) -> Self {
        let errors: Vec<String> = causes.iter().map(PtxError::to_string).collect();
        let msg = errors.join("\n");
        CompilerError::PtxParserError(msg)
    }
}

impl From<io::Error> for CompilerError {
    fn from(cause: io::Error) -> Self {
        CompilerError::IoError(cause)
    }
}

impl From<Utf8Error> for CompilerError {
    fn from(cause: Utf8Error) -> Self {
        CompilerError::ParseFileError(cause)
    }
}

impl From<TranslateError> for CompilerError {
    fn from(cause: TranslateError) -> Self {
        CompilerError::PtxTranslateError(cause)
    }
}