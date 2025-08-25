use hip_runtime_sys::*;

pub(crate) unsafe fn destroy(graph: hipGraph_t) -> hipError_t {
    hipGraphDestroy(graph)
}

pub(crate) unsafe fn exec_destroy(graph_exec: hipGraphExec_t) -> hipError_t {
    hipGraphExecDestroy(graph_exec)
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
