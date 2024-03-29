/* automatically generated by rust-bindgen 0.66.1 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUstream_st {
    _unused: [u8; 0],
}
#[doc = " CUDA stream"]
pub type cudaStream_t = *mut CUstream_st;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncclComm {
    _unused: [u8; 0],
}
pub type ncclComm_t = *mut ncclComm;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncclUniqueId {
    pub internal: [::std::os::raw::c_char; 128usize],
}
impl ncclResult_t {
    pub const ncclSuccess: ncclResult_t = ncclResult_t(0);
}
impl ncclResult_t {
    pub const ncclUnhandledCudaError: ncclResult_t = ncclResult_t(1);
}
impl ncclResult_t {
    pub const ncclSystemError: ncclResult_t = ncclResult_t(2);
}
impl ncclResult_t {
    pub const ncclInternalError: ncclResult_t = ncclResult_t(3);
}
impl ncclResult_t {
    pub const ncclInvalidArgument: ncclResult_t = ncclResult_t(4);
}
impl ncclResult_t {
    pub const ncclInvalidUsage: ncclResult_t = ncclResult_t(5);
}
impl ncclResult_t {
    pub const ncclRemoteError: ncclResult_t = ncclResult_t(6);
}
impl ncclResult_t {
    pub const ncclInProgress: ncclResult_t = ncclResult_t(7);
}
impl ncclResult_t {
    pub const ncclNumResults: ncclResult_t = ncclResult_t(8);
}
#[repr(transparent)]
#[must_use]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct ncclResult_t(pub ::std::os::raw::c_uint);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncclConfig_v21700 {
    pub size: usize,
    pub magic: ::std::os::raw::c_uint,
    pub version: ::std::os::raw::c_uint,
    pub blocking: ::std::os::raw::c_int,
    pub cgaClusterSize: ::std::os::raw::c_int,
    pub minCTAs: ::std::os::raw::c_int,
    pub maxCTAs: ::std::os::raw::c_int,
    pub netName: *const ::std::os::raw::c_char,
    pub splitShare: ::std::os::raw::c_int,
}
pub type ncclConfig_t = ncclConfig_v21700;

#[must_use]
#[no_mangle]
pub extern "C" fn ncclGetVersion(version: *mut ::std::os::raw::c_int) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclGetVersion(version: *mut ::std::os::raw::c_int) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclGetUniqueId(uniqueId: *mut ncclUniqueId) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclGetUniqueId(uniqueId: *mut ncclUniqueId) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclCommInitRankConfig(
    comm: *mut ncclComm_t,
    nranks: ::std::os::raw::c_int,
    commId: ncclUniqueId,
    rank: ::std::os::raw::c_int,
    config: *mut ncclConfig_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclCommInitRankConfig(
    comm: *mut ncclComm_t,
    nranks: ::std::os::raw::c_int,
    commId: ncclUniqueId,
    rank: ::std::os::raw::c_int,
    config: *mut ncclConfig_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclCommInitRank(
    comm: *mut ncclComm_t,
    nranks: ::std::os::raw::c_int,
    commId: ncclUniqueId,
    rank: ::std::os::raw::c_int,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclCommInitRank(
    comm: *mut ncclComm_t,
    nranks: ::std::os::raw::c_int,
    commId: ncclUniqueId,
    rank: ::std::os::raw::c_int,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclCommInitAll(
    comm: *mut ncclComm_t,
    ndev: ::std::os::raw::c_int,
    devlist: *const ::std::os::raw::c_int,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclCommInitAll(
    comm: *mut ncclComm_t,
    ndev: ::std::os::raw::c_int,
    devlist: *const ::std::os::raw::c_int,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclCommFinalize(comm: ncclComm_t) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclCommFinalize(comm: ncclComm_t) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclCommDestroy(comm: ncclComm_t) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclCommDestroy(comm: ncclComm_t) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclCommAbort(comm: ncclComm_t) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclCommAbort(comm: ncclComm_t) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclCommSplit(
    comm: ncclComm_t,
    color: ::std::os::raw::c_int,
    key: ::std::os::raw::c_int,
    newcomm: *mut ncclComm_t,
    config: *mut ncclConfig_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclCommSplit(
    comm: ncclComm_t,
    color: ::std::os::raw::c_int,
    key: ::std::os::raw::c_int,
    newcomm: *mut ncclComm_t,
    config: *mut ncclConfig_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[no_mangle]
pub extern "C" fn ncclGetErrorString(result: ncclResult_t) -> *const ::std::os::raw::c_char {
    "\0".as_ptr().cast()
}

#[no_mangle]
pub extern "C" fn pncclGetErrorString(result: ncclResult_t) -> *const ::std::os::raw::c_char {
    "\0".as_ptr().cast()
}

#[no_mangle]
pub extern "C" fn ncclGetLastError(comm: ncclComm_t) -> *const ::std::os::raw::c_char {
    "\0".as_ptr().cast()
}

#[no_mangle]
pub extern "C" fn pncclGetLastError(comm: ncclComm_t) -> *const ::std::os::raw::c_char {
    "\0".as_ptr().cast()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclCommGetAsyncError(
    comm: ncclComm_t,
    asyncError: *mut ncclResult_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclCommGetAsyncError(
    comm: ncclComm_t,
    asyncError: *mut ncclResult_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclCommCount(
    comm: ncclComm_t,
    count: *mut ::std::os::raw::c_int,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclCommCount(
    comm: ncclComm_t,
    count: *mut ::std::os::raw::c_int,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclCommCuDevice(
    comm: ncclComm_t,
    device: *mut ::std::os::raw::c_int,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclCommCuDevice(
    comm: ncclComm_t,
    device: *mut ::std::os::raw::c_int,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclCommUserRank(
    comm: ncclComm_t,
    rank: *mut ::std::os::raw::c_int,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclCommUserRank(
    comm: ncclComm_t,
    rank: *mut ::std::os::raw::c_int,
) -> ncclResult_t {
    crate::unsupported()
}
impl ncclRedOp_t {
    pub const ncclSum: ncclRedOp_t = ncclRedOp_t(0);
}
impl ncclRedOp_t {
    pub const ncclProd: ncclRedOp_t = ncclRedOp_t(1);
}
impl ncclRedOp_t {
    pub const ncclMax: ncclRedOp_t = ncclRedOp_t(2);
}
impl ncclRedOp_t {
    pub const ncclMin: ncclRedOp_t = ncclRedOp_t(3);
}
impl ncclRedOp_t {
    pub const ncclAvg: ncclRedOp_t = ncclRedOp_t(4);
}
impl ncclRedOp_t {
    pub const ncclNumOps: ncclRedOp_t = ncclRedOp_t(5);
}
impl ncclRedOp_t {
    pub const ncclMaxRedOp: ncclRedOp_t = ncclRedOp_t(2147483647);
}
#[repr(transparent)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct ncclRedOp_t(pub ::std::os::raw::c_uint);
impl ncclDataType_t {
    pub const ncclInt8: ncclDataType_t = ncclDataType_t(0);
}
impl ncclDataType_t {
    pub const ncclChar: ncclDataType_t = ncclDataType_t(0);
}
impl ncclDataType_t {
    pub const ncclUint8: ncclDataType_t = ncclDataType_t(1);
}
impl ncclDataType_t {
    pub const ncclInt32: ncclDataType_t = ncclDataType_t(2);
}
impl ncclDataType_t {
    pub const ncclInt: ncclDataType_t = ncclDataType_t(2);
}
impl ncclDataType_t {
    pub const ncclUint32: ncclDataType_t = ncclDataType_t(3);
}
impl ncclDataType_t {
    pub const ncclInt64: ncclDataType_t = ncclDataType_t(4);
}
impl ncclDataType_t {
    pub const ncclUint64: ncclDataType_t = ncclDataType_t(5);
}
impl ncclDataType_t {
    pub const ncclFloat16: ncclDataType_t = ncclDataType_t(6);
}
impl ncclDataType_t {
    pub const ncclHalf: ncclDataType_t = ncclDataType_t(6);
}
impl ncclDataType_t {
    pub const ncclFloat32: ncclDataType_t = ncclDataType_t(7);
}
impl ncclDataType_t {
    pub const ncclFloat: ncclDataType_t = ncclDataType_t(7);
}
impl ncclDataType_t {
    pub const ncclFloat64: ncclDataType_t = ncclDataType_t(8);
}
impl ncclDataType_t {
    pub const ncclDouble: ncclDataType_t = ncclDataType_t(8);
}
impl ncclDataType_t {
    pub const ncclNumTypes: ncclDataType_t = ncclDataType_t(9);
}
#[repr(transparent)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct ncclDataType_t(pub ::std::os::raw::c_uint);
impl ncclScalarResidence_t {
    pub const ncclScalarDevice: ncclScalarResidence_t = ncclScalarResidence_t(0);
}
impl ncclScalarResidence_t {
    pub const ncclScalarHostImmediate: ncclScalarResidence_t = ncclScalarResidence_t(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct ncclScalarResidence_t(pub ::std::os::raw::c_uint);

#[must_use]
#[no_mangle]
pub extern "C" fn ncclRedOpCreatePreMulSum(
    op: *mut ncclRedOp_t,
    scalar: *mut ::std::os::raw::c_void,
    datatype: ncclDataType_t,
    residence: ncclScalarResidence_t,
    comm: ncclComm_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclRedOpCreatePreMulSum(
    op: *mut ncclRedOp_t,
    scalar: *mut ::std::os::raw::c_void,
    datatype: ncclDataType_t,
    residence: ncclScalarResidence_t,
    comm: ncclComm_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclRedOpDestroy(op: ncclRedOp_t, comm: ncclComm_t) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclRedOpDestroy(op: ncclRedOp_t, comm: ncclComm_t) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclReduce(
    sendbuff: *const ::std::os::raw::c_void,
    recvbuff: *mut ::std::os::raw::c_void,
    count: usize,
    datatype: ncclDataType_t,
    op: ncclRedOp_t,
    root: ::std::os::raw::c_int,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclReduce(
    sendbuff: *const ::std::os::raw::c_void,
    recvbuff: *mut ::std::os::raw::c_void,
    count: usize,
    datatype: ncclDataType_t,
    op: ncclRedOp_t,
    root: ::std::os::raw::c_int,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclBcast(
    buff: *mut ::std::os::raw::c_void,
    count: usize,
    datatype: ncclDataType_t,
    root: ::std::os::raw::c_int,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclBcast(
    buff: *mut ::std::os::raw::c_void,
    count: usize,
    datatype: ncclDataType_t,
    root: ::std::os::raw::c_int,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclBroadcast(
    sendbuff: *const ::std::os::raw::c_void,
    recvbuff: *mut ::std::os::raw::c_void,
    count: usize,
    datatype: ncclDataType_t,
    root: ::std::os::raw::c_int,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclBroadcast(
    sendbuff: *const ::std::os::raw::c_void,
    recvbuff: *mut ::std::os::raw::c_void,
    count: usize,
    datatype: ncclDataType_t,
    root: ::std::os::raw::c_int,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclAllReduce(
    sendbuff: *const ::std::os::raw::c_void,
    recvbuff: *mut ::std::os::raw::c_void,
    count: usize,
    datatype: ncclDataType_t,
    op: ncclRedOp_t,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclAllReduce(
    sendbuff: *const ::std::os::raw::c_void,
    recvbuff: *mut ::std::os::raw::c_void,
    count: usize,
    datatype: ncclDataType_t,
    op: ncclRedOp_t,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclReduceScatter(
    sendbuff: *const ::std::os::raw::c_void,
    recvbuff: *mut ::std::os::raw::c_void,
    recvcount: usize,
    datatype: ncclDataType_t,
    op: ncclRedOp_t,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclReduceScatter(
    sendbuff: *const ::std::os::raw::c_void,
    recvbuff: *mut ::std::os::raw::c_void,
    recvcount: usize,
    datatype: ncclDataType_t,
    op: ncclRedOp_t,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclAllGather(
    sendbuff: *const ::std::os::raw::c_void,
    recvbuff: *mut ::std::os::raw::c_void,
    sendcount: usize,
    datatype: ncclDataType_t,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclAllGather(
    sendbuff: *const ::std::os::raw::c_void,
    recvbuff: *mut ::std::os::raw::c_void,
    sendcount: usize,
    datatype: ncclDataType_t,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclSend(
    sendbuff: *const ::std::os::raw::c_void,
    count: usize,
    datatype: ncclDataType_t,
    peer: ::std::os::raw::c_int,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclSend(
    sendbuff: *const ::std::os::raw::c_void,
    count: usize,
    datatype: ncclDataType_t,
    peer: ::std::os::raw::c_int,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclRecv(
    recvbuff: *mut ::std::os::raw::c_void,
    count: usize,
    datatype: ncclDataType_t,
    peer: ::std::os::raw::c_int,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclRecv(
    recvbuff: *mut ::std::os::raw::c_void,
    count: usize,
    datatype: ncclDataType_t,
    peer: ::std::os::raw::c_int,
    comm: ncclComm_t,
    stream: cudaStream_t,
) -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclGroupStart() -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclGroupStart() -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn ncclGroupEnd() -> ncclResult_t {
    crate::unsupported()
}

#[must_use]
#[no_mangle]
pub extern "C" fn pncclGroupEnd() -> ncclResult_t {
    crate::unsupported()
}
