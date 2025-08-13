use std::ffi::FromBytesUntilNulError;
use std::io;
use std::str::Utf8Error;

use hip_runtime_sys::hipErrorCode_t;
use ptx::TranslateError;
use ptx_parser::PtxError;

#[derive(Debug, thiserror::Error)]
pub enum CompilerError {
    #[error("HIP error code: {0:?}")]
    HipError(hipErrorCode_t),
    #[error(transparent)]
    ComgrError(#[from] comgr::Error),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    #[error(transparent)]
    FromBytesUntilNulError(#[from] FromBytesUntilNulError),
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

impl From<Vec<PtxError<'_>>> for CompilerError {
    fn from(causes: Vec<PtxError>) -> Self {
        let errors: Vec<String> = causes
            .iter()
            .map(|e| {
                let msg = match e {
                    PtxError::UnrecognizedStatement(value)
                    | PtxError::UnrecognizedDirective(value) => value.to_string(),
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
