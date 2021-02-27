use crate::common::CudaDriverFns;
use cuda_types::*;
use std::mem;

mod common;

cuda_driver_test!(context_dark_api_primary_is_unretained);

unsafe fn context_dark_api_primary_is_unretained<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let dev = CUdevice_v1(0);
    let mut ctx1 = mem::zeroed();
    let mut export_table = mem::zeroed();
    assert_eq!(
        cuda.cuGetExportTable(
            &mut export_table,
            &CUuuid {
                bytes: [
                    0x6b, 0xd5, 0xfb, 0x6c, 0x5b, 0xf4, 0xe7, 0x4a, 0x89, 0x87, 0xd9, 0x39, 0x12,
                    0xfd, 0x9d, 0xf9
                ]
            }
        ),
        CUresult::CUDA_SUCCESS
    );
    let get_primary_ctx = mem::transmute::<
        _,
        unsafe extern "system" fn(*mut CUcontext, CUdevice) -> CUresult,
    >(*(export_table as *mut usize).add(2));
    assert_eq!(get_primary_ctx(&mut ctx1, dev), CUresult::CUDA_SUCCESS);
    let mut api_version = mem::zeroed();
    assert_eq!(
        cuda.cuCtxGetApiVersion(ctx1, &mut api_version),
        CUresult::CUDA_ERROR_INVALID_CONTEXT
    );
    assert_eq!(cuda.cuCtxSetCurrent(ctx1), CUresult::CUDA_SUCCESS);
    let mut device = mem::zeroed();
    assert_eq!(cuda.cuCtxGetDevice(&mut device), CUresult::CUDA_SUCCESS);
    // TODO: re-enable when adding context getters
    /*
    let mut cache_cfg = mem::zeroed();
    assert_eq!(
        cuda.cuCtxGetCacheConfig(&mut cache_cfg),
        CUresult::CUDA_ERROR_CONTEXT_IS_DESTROYED
    );
    let mut exec_affinity = mem::zeroed();
    assert_eq!(
        cuda.cuCtxGetExecAffinity(
            &mut exec_affinity,
            CUexecAffinityType::CU_EXEC_AFFINITY_TYPE_SM_COUNT
        ),
        CUresult::CUDA_ERROR_CONTEXT_IS_DESTROYED
    );
    let mut flags = mem::zeroed();
    assert_eq!(cuda.cuCtxGetFlags(&mut flags,), CUresult::CUDA_SUCCESS);
    let mut stack = mem::zeroed();
    assert_eq!(
        cuda.cuCtxGetLimit(&mut stack, CUlimit::CU_LIMIT_STACK_SIZE),
        CUresult::CUDA_ERROR_CONTEXT_IS_DESTROYED
    );
    let mut shared_mem_cfg = mem::zeroed();
    assert_eq!(
        cuda.cuCtxGetSharedMemConfig(&mut shared_mem_cfg),
        CUresult::CUDA_ERROR_CONTEXT_IS_DESTROYED
    );
    let mut lowest_priority = mem::zeroed();
    let mut highest_priority = mem::zeroed();
    assert_eq!(
        cuda.cuCtxGetStreamPriorityRange(&mut lowest_priority, &mut highest_priority),
        CUresult::CUDA_ERROR_CONTEXT_IS_DESTROYED
    );
     */
    let mut ctx2 = mem::zeroed();
    assert_eq!(
        cuda.cuDevicePrimaryCtxRetain(&mut ctx2, dev),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(ctx1, ctx2);
    assert_eq!(
        cuda.cuCtxGetApiVersion(ctx1, &mut api_version),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(cuda.cuCtxGetDevice(&mut device), CUresult::CUDA_SUCCESS);
}
