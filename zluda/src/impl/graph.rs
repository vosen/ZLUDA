use cuda_types::cuda::{CUerror, CUresult};
use hip_runtime_sys::*;
use zluda_common::ZludaGraphExecUpdateResultInfo;

pub(crate) unsafe fn destroy(graph: hipGraph_t) -> hipError_t {
    hipGraphDestroy(graph)
}

pub(crate) unsafe fn exec_destroy(graph_exec: hipGraphExec_t) -> hipError_t {
    hipGraphExecDestroy(graph_exec)
}

pub(crate) fn exec_update_v2(
    h_graph_exec: hipGraphExec_t,
    h_graph: hipGraph_t,
    mut result_info: ZludaGraphExecUpdateResultInfo,
) -> CUresult {
    let mut h_error_node: hipGraphNode_t = unsafe { std::mem::zeroed() };
    let mut update_result: hipGraphExecUpdateResult = unsafe { std::mem::zeroed() };
    unsafe { hipGraphExecUpdate(h_graph_exec, h_graph, &mut h_error_node, &mut update_result) }?;

    result_info.set_error_node(h_error_node);
    result_info.set_error_from_node(h_error_node);

    result_info.set_result::<CUerror>(update_result)?;

    Ok(())
}

pub(crate) unsafe fn get_nodes(
    graph: hipGraph_t,
    nodes: *mut hipGraphNode_t,
    num_nodes: *mut usize,
) -> hipError_t {
    hipGraphGetNodes(graph, nodes, num_nodes)
}

pub(crate) unsafe fn instantiate_with_flags(
    graph_exec: *mut hipGraphExec_t,
    graph: hipGraph_t,
    flags: u64,
) -> hipError_t {
    hipGraphInstantiateWithFlags(graph_exec, graph, flags)
}

pub(crate) unsafe fn launch(graph_exec: hipGraphExec_t, stream: hipStream_t) -> hipError_t {
    hipGraphLaunch(graph_exec, stream)
}
