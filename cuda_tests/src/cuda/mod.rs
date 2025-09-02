use crate::api_trait;
use cuda_types::cuda::*;

cuda_macros::cuda_function_declarations!(api_trait);

pub unsafe fn init_check<T: Api>(api: T) {
    assert_eq!(api.cuInit(0), CUresult::SUCCESS);
}
