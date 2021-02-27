use cuda_base::cuda_type_declarations;

cuda_type_declarations!();

impl From<CUresult> for Result<(), CUresult> {
    fn from(value: CUresult) -> Self {
        match value {
            CUresult::CUDA_SUCCESS => Ok(()),
            err => Err(err),
        }
    }
}
