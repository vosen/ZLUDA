use super::{function, stream, LiveCheck};
use crate::hip_call_cuda;
use cuda_types::*;
use hip_runtime_sys::*;

pub(crate) unsafe fn add_kernel_node(
    ph_graph_node: *mut hipGraphNode_t,
    h_graph: hipGraph_t,
    dependencies: *const hipGraphNode_t,
    num_dependencies: usize,
    node_params: *const CUDA_KERNEL_NODE_PARAMS_v1,
) -> Result<(), CUresult> {
    let node_params = node_params
        .as_ref()
        .ok_or(CUresult::CUDA_ERROR_INVALID_VALUE)?;
    let node_params = hip_node_params(node_params)?;
    hip_call_cuda!(hipGraphAddKernelNode(
        ph_graph_node,
        h_graph,
        dependencies,
        num_dependencies,
        &node_params,
    ));
    Ok(())
}

unsafe fn hip_node_params(
    cuda: &CUDA_KERNEL_NODE_PARAMS_v1,
) -> Result<hipKernelNodeParams, CUresult> {
    let zluda_func = cuda.func.cast::<function::Function>();
    let zluda_func = LiveCheck::as_result(zluda_func)?;
    Ok(hipKernelNodeParams {
        blockDim: dim3 {
            x: cuda.blockDimX,
            y: cuda.blockDimY,
            z: cuda.blockDimZ,
        },
        extra: cuda.extra,
        func: zluda_func.base.cast(),
        gridDim: dim3 {
            x: cuda.gridDimX,
            y: cuda.gridDimY,
            z: cuda.gridDimZ,
        },
        kernelParams: cuda.kernelParams,
        sharedMemBytes: cuda.sharedMemBytes,
    })
}

pub(crate) unsafe fn launch(
    graph: hipGraphExec_t,
    stream: *mut stream::Stream,
) -> Result<(), CUresult> {
    let stream = stream::as_hip_stream(stream)?;
    hip_call_cuda!(hipGraphLaunch(graph, stream));
    Ok(())
}
