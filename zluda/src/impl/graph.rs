use cuda_types::cuda::CUgraphExecUpdateResult;
use hip_runtime_sys::*;

pub(crate) unsafe fn destroy(graph: hipGraph_t) -> hipError_t {
    hipGraphDestroy(graph)
}

pub(crate) unsafe fn exec_destroy(graph_exec: hipGraphExec_t) -> hipError_t {
    hipGraphExecDestroy(graph_exec)
}

pub(crate) fn exec_update_v2(
    h_graph_exec: hipGraphExec_t,
    h_graph: hipGraph_t,
    result_info: &mut cuda_types::cuda::CUgraphExecUpdateResultInfo,
) -> hipError_t {
    let mut h_error_node: hipGraphNode_t = unsafe { std::mem::zeroed() };
    let mut update_result: hipGraphExecUpdateResult = unsafe { std::mem::zeroed() };
    unsafe { hipGraphExecUpdate(h_graph_exec, h_graph, &mut h_error_node, &mut update_result) }?;

    result_info.errorNode = unsafe { std::mem::transmute(h_error_node) };
    result_info.errorFromNode = unsafe { std::mem::transmute(h_error_node) };
    result_info.result = match update_result {
        hipGraphExecUpdateResult::hipGraphExecUpdateSuccess => {
            CUgraphExecUpdateResult::CU_GRAPH_EXEC_UPDATE_SUCCESS
        }
        hipGraphExecUpdateResult::hipGraphExecUpdateError => {
            CUgraphExecUpdateResult::CU_GRAPH_EXEC_UPDATE_ERROR
        }
        hipGraphExecUpdateResult::hipGraphExecUpdateErrorTopologyChanged => {
            CUgraphExecUpdateResult::CU_GRAPH_EXEC_UPDATE_ERROR_TOPOLOGY_CHANGED
        }
        hipGraphExecUpdateResult::hipGraphExecUpdateErrorNodeTypeChanged => {
            CUgraphExecUpdateResult::CU_GRAPH_EXEC_UPDATE_ERROR_NODE_TYPE_CHANGED
        }
        hipGraphExecUpdateResult::hipGraphExecUpdateErrorFunctionChanged => {
            CUgraphExecUpdateResult::CU_GRAPH_EXEC_UPDATE_ERROR_FUNCTION_CHANGED
        }
        hipGraphExecUpdateResult::hipGraphExecUpdateErrorParametersChanged => {
            CUgraphExecUpdateResult::CU_GRAPH_EXEC_UPDATE_ERROR_PARAMETERS_CHANGED
        }
        hipGraphExecUpdateResult::hipGraphExecUpdateErrorNotSupported => {
            CUgraphExecUpdateResult::CU_GRAPH_EXEC_UPDATE_ERROR_NOT_SUPPORTED
        }
        hipGraphExecUpdateResult::hipGraphExecUpdateErrorUnsupportedFunctionChange => {
            CUgraphExecUpdateResult::CU_GRAPH_EXEC_UPDATE_ERROR_UNSUPPORTED_FUNCTION_CHANGE
        }
        _ => return hipError_t::ErrorNotSupported,
    };

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
