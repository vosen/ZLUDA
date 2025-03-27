use std::ffi::FromBytesUntilNulError;
use std::io;
use std::str::Utf8Error;

use amd_comgr_sys::amd_comgr_status_s;
use hip_runtime_sys::hipErrorCode_t;
use ptx::TranslateError;
use ptx_parser::PtxError;

#[derive(Debug, thiserror::Error)]
pub enum CompilerError {
    #[error("HIP error code: {0:?}")]
    HipError(hipErrorCode_t),
    #[error("amd_comgr status code: {0:?}")]
    ComgrError(amd_comgr_status_s),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    #[error("{message}")]
    GenericError {
        #[source]
        cause: Option<Box<dyn std::error::Error>>,
        message: String,
    },
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
        let errors: Vec<String> = causes
            .iter()
            .map(|e| {
                let msg = match e {
                    PtxError::UnrecognizedStatement(value)
                    | PtxError::UnrecognizedDirective(value) => value.unwrap_or("").to_string(),
                    other => other.to_string(),
                };
                format!("PtxError::{}: {}", e.as_ref(), msg)
            })
            .collect();
        let message = errors.join("\n");
        CompilerError::GenericError {
            cause: None,
            message,
        }
    }
}

impl From<TranslateError> for CompilerError {
    fn from(cause: TranslateError) -> Self {
        let message = format!("PTX TranslateError::{}", cause.as_ref());
        let cause = Some(Box::new(cause) as Box<dyn std::error::Error>);
        CompilerError::GenericError { cause, message }
    }
}

impl From<FromBytesUntilNulError> for CompilerError {
    fn from(cause: FromBytesUntilNulError) -> Self {
        let message = format!("{}", cause);
        let cause = Some(Box::new(cause) as Box<dyn std::error::Error>);
        CompilerError::GenericError { cause, message }
    }
}
