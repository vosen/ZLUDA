//! NCCL for one Apple GPU (zluda_agx on Asahi Linux).
//!
//! An Asahi machine has one GPU, so every communicator has one rank (`ncclCommInitRank` with `nranks` 1,
//! `ncclCommInitAll` with one device, `ncclCommSplit`); NCCL itself refuses two ranks on one device. On one rank
//! each collective is a copy from the send to the receive buffer on the stream (none when they are the same),
//! `ncclAvg` divides by one, and a `ncclRedOpCreatePreMulSum` operator scales the copy (f32 and f64). Sends and
//! receives to rank 0 pair up in order, inside or outside a group. Group calls nest; operations run as they are
//! issued.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use agx_libs::engine;
use cuda_types::cuda::CUstream;
use std::collections::VecDeque;
use std::ffi::{c_char, c_int, c_void};
use std::sync::Mutex;

/// `ncclResult_t`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ncclResult_t(pub u32);

impl ncclResult_t {
    pub const Success: ncclResult_t = ncclResult_t(0);
    pub const UnhandledCudaError: ncclResult_t = ncclResult_t(1);
    pub const SystemError: ncclResult_t = ncclResult_t(2);
    pub const InternalError: ncclResult_t = ncclResult_t(3);
    pub const InvalidArgument: ncclResult_t = ncclResult_t(4);
    pub const InvalidUsage: ncclResult_t = ncclResult_t(5);
}

/// `ncclDataType_t`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ncclDataType_t(pub u32);

impl ncclDataType_t {
    pub const Int8: ncclDataType_t = ncclDataType_t(0);
    pub const Uint8: ncclDataType_t = ncclDataType_t(1);
    pub const Int32: ncclDataType_t = ncclDataType_t(2);
    pub const Uint32: ncclDataType_t = ncclDataType_t(3);
    pub const Int64: ncclDataType_t = ncclDataType_t(4);
    pub const Uint64: ncclDataType_t = ncclDataType_t(5);
    pub const Float16: ncclDataType_t = ncclDataType_t(6);
    pub const Float32: ncclDataType_t = ncclDataType_t(7);
    pub const Float64: ncclDataType_t = ncclDataType_t(8);
    pub const Bfloat16: ncclDataType_t = ncclDataType_t(9);
    pub const Float8e4m3: ncclDataType_t = ncclDataType_t(10);
    pub const Float8e5m2: ncclDataType_t = ncclDataType_t(11);
}

/// The size of an element of `ty`.
pub fn type_size(ty: ncclDataType_t) -> Result<usize, ncclResult_t> {
    Ok(match ty.0 {
        0 | 1 | 10 | 11 => 1,
        6 | 9 => 2,
        2 | 3 | 7 => 4,
        4 | 5 | 8 => 8,
        _ => return Err(ncclResult_t::InvalidArgument),
    })
}

/// `ncclRedOp_t`: the built-in operators, then those of `ncclRedOpCreatePreMulSum`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ncclRedOp_t(pub u32);

/// The first operator number `ncclRedOpCreatePreMulSum` hands out (after sum, prod, max, min, avg).
pub const FIRST_CUSTOM_OP: u32 = 5;

/// `ncclScalarResidence_t`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ncclScalarResidence_t(pub u32);

impl ncclScalarResidence_t {
    pub const Device: ncclScalarResidence_t = ncclScalarResidence_t(0);
    pub const HostImmediate: ncclScalarResidence_t = ncclScalarResidence_t(1);
}

/// `ncclUniqueId`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ncclUniqueId {
    pub internal: [c_char; 128],
}

/// The NCCL version reported (2.21.5).
pub const VERSION: (c_int, c_int, c_int) = (2, 21, 5);

/// What an operation does on one rank.
#[derive(Clone, Debug, PartialEq)]
pub enum Step {
    /// Copy `bytes` bytes on `stream` (skipped when `dst == src`).
    Copy { dst: usize, src: usize, bytes: usize },
    /// `dst[i] *= scale` for `n` f32 (`double` false) or f64 values.
    Scale { dst: usize, n: usize, double: bool, scale: f64 },
}

/// A communicator: its custom operators and the sends and receives waiting for their pair.
pub struct Comm {
    /// The scale of each PreMulSum operator, by `op - FIRST_CUSTOM_OP` (None once destroyed).
    ops: Vec<Option<(f64, ncclDataType_t)>>,
    sends: VecDeque<(usize, usize, CUstream)>,
    recvs: VecDeque<(usize, usize, CUstream)>,
}

pub type ncclComm_t = *mut Comm;

impl Comm {
    fn new() -> Comm {
        Comm { ops: Vec::new(), sends: VecDeque::new(), recvs: VecDeque::new() }
    }

    /// The steps of a reduction of `count` elements with `op` from `src` into `dst`.
    pub fn reduce(&self, src: usize, dst: usize, count: usize, ty: ncclDataType_t, op: ncclRedOp_t) -> Result<Vec<Step>, ncclResult_t> {
        let bytes = count * type_size(ty)?;
        let mut steps = vec![Step::Copy { dst, src, bytes }];
        match op.0 {
            0..=4 => {}
            n => {
                let (scale, oty) = self.ops.get((n - FIRST_CUSTOM_OP) as usize).copied().flatten().ok_or(ncclResult_t::InvalidArgument)?;
                if oty != ty {
                    return Err(ncclResult_t::InvalidArgument);
                }
                let double = match ty {
                    ncclDataType_t::Float32 => false,
                    ncclDataType_t::Float64 => true,
                    _ => return Err(ncclResult_t::InvalidArgument),
                };
                if count > 0 {
                    steps.push(Step::Scale { dst, n: count, double, scale });
                }
            }
        }
        Ok(steps)
    }

    /// Pairs a send or receive with a waiting one: the copy of the pair.
    fn pair(&mut self, send: Option<(usize, usize, CUstream)>, recv: Option<(usize, usize, CUstream)>) -> Option<(usize, usize, usize, CUstream, CUstream)> {
        if let Some(s) = send {
            self.sends.push_back(s);
        }
        if let Some(r) = recv {
            self.recvs.push_back(r);
        }
        if self.sends.is_empty() || self.recvs.is_empty() {
            return None;
        }
        let (src, sbytes, sstream) = self.sends.pop_front().unwrap();
        let (dst, rbytes, rstream) = self.recvs.pop_front().unwrap();
        Some((dst, src, sbytes.min(rbytes), sstream, rstream))
    }
}

fn status(_: engine::Error) -> ncclResult_t {
    ncclResult_t::UnhandledCudaError
}

fn done(r: Result<(), ncclResult_t>) -> ncclResult_t {
    r.err().unwrap_or(ncclResult_t::Success)
}

unsafe fn comm<'a>(c: ncclComm_t) -> Result<&'a mut Comm, ncclResult_t> {
    c.as_mut().ok_or(ncclResult_t::InvalidArgument)
}

/// Runs `steps` on `stream`.
unsafe fn run(steps: &[Step], stream: CUstream) -> Result<(), ncclResult_t> {
    for s in steps {
        match *s {
            Step::Copy { dst, src, bytes } => {
                if dst != src && bytes > 0 {
                    engine::copy(dst as *mut c_void, src as *const c_void, bytes, stream).map_err(status)?;
                }
            }
            Step::Scale { dst, n, double, scale } => {
                // the sparse library's y *= beta kernel
                let (mut y, mut n, mut stride) = (dst as *mut c_void, n as i64, 1i64);
                let (mut s32, mut s64) = (scale as f32, scale);
                let beta = if double { &mut s64 as *mut f64 as *mut c_void } else { &mut s32 as *mut f32 as *mut c_void };
                let name = if double { c"agx_sparse_scale_f64" } else { c"agx_sparse_scale" };
                let f = engine::function(agx_libs::SPARSE, name).map_err(status)?;
                let groups = u32::try_from((n as u64).div_ceil(64)).map_err(|_| ncclResult_t::InvalidArgument)?;
                let mut params = [&mut y as *mut _ as *mut c_void, &mut n as *mut _ as *mut c_void, &mut stride as *mut _ as *mut c_void, beta];
                engine::launch_kernel(f, [groups, 1], [64, 1], &mut params, stream).map_err(status)?;
            }
        }
    }
    Ok(())
}

static GROUP: Mutex<u32> = Mutex::new(0);

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclGetVersion(v: *mut c_int) -> ncclResult_t {
    match v.as_mut() {
        Some(v) => {
            *v = VERSION.0 * 10000 + VERSION.1 * 100 + VERSION.2;
            ncclResult_t::Success
        }
        None => ncclResult_t::InvalidArgument,
    }
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclGetErrorString(r: ncclResult_t) -> *const c_char {
    match r.0 {
        0 => c"no error",
        1 => c"unhandled cuda error (run with NCCL_DEBUG=INFO for details)",
        2 => c"unhandled system error (run with NCCL_DEBUG=INFO for details)",
        3 => c"internal error - please report this issue to the NCCL developers",
        4 => c"invalid argument (run with NCCL_DEBUG=WARN for details)",
        5 => c"invalid usage (run with NCCL_DEBUG=WARN for details)",
        6 => c"remote process exited or there was a network error",
        7 => c"NCCL operation in progress",
        _ => c"unknown result code",
    }
    .as_ptr()
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclGetLastError(_c: ncclComm_t) -> *const c_char {
    c"".as_ptr()
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclGetUniqueId(id: *mut ncclUniqueId) -> ncclResult_t {
    match id.as_mut() {
        Some(id) => {
            id.internal = [0; 128];
            for (d, s) in id.internal.iter_mut().zip(b"zluda-agx") {
                *d = *s as c_char;
            }
            ncclResult_t::Success
        }
        None => ncclResult_t::InvalidArgument,
    }
}

unsafe fn init(c: *mut ncclComm_t, nranks: c_int, rank: c_int) -> ncclResult_t {
    let Some(c) = c.as_mut() else {
        return ncclResult_t::InvalidArgument;
    };
    if nranks != 1 || rank != 0 {
        // one GPU: a second rank would share the device, which NCCL does not allow either
        return ncclResult_t::InvalidUsage;
    }
    *c = Box::into_raw(Box::new(Comm::new()));
    ncclResult_t::Success
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclCommInitRank(c: *mut ncclComm_t, nranks: c_int, _id: ncclUniqueId, rank: c_int) -> ncclResult_t {
    init(c, nranks, rank)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclCommInitRankConfig(c: *mut ncclComm_t, nranks: c_int, _id: ncclUniqueId, rank: c_int, _config: *mut c_void) -> ncclResult_t {
    init(c, nranks, rank)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclCommInitAll(c: *mut ncclComm_t, ndev: c_int, devs: *const c_int) -> ncclResult_t {
    if ndev != 1 || (!devs.is_null() && *devs != 0) {
        return ncclResult_t::InvalidUsage;
    }
    init(c, 1, 0)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclCommSplit(c: ncclComm_t, color: c_int, _key: c_int, new: *mut ncclComm_t, _config: *mut c_void) -> ncclResult_t {
    done((|| {
        comm(c)?;
        let new = new.as_mut().ok_or(ncclResult_t::InvalidArgument)?;
        // NCCL_SPLIT_NOCOLOR
        *new = if color == -1 { std::ptr::null_mut() } else { Box::into_raw(Box::new(Comm::new())) };
        Ok(())
    })())
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclCommFinalize(c: ncclComm_t) -> ncclResult_t {
    done(comm(c).map(|_| ()))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclCommDestroy(c: ncclComm_t) -> ncclResult_t {
    if c.is_null() {
        return ncclResult_t::Success;
    }
    drop(Box::from_raw(c));
    ncclResult_t::Success
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclCommAbort(c: ncclComm_t) -> ncclResult_t {
    ncclCommDestroy(c)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclCommGetAsyncError(c: ncclComm_t, r: *mut ncclResult_t) -> ncclResult_t {
    done(comm(c).and_then(|_| {
        *r.as_mut().ok_or(ncclResult_t::InvalidArgument)? = ncclResult_t::Success;
        Ok(())
    }))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclCommCount(c: ncclComm_t, n: *mut c_int) -> ncclResult_t {
    done(comm(c).and_then(|_| {
        *n.as_mut().ok_or(ncclResult_t::InvalidArgument)? = 1;
        Ok(())
    }))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclCommCuDevice(c: ncclComm_t, d: *mut c_int) -> ncclResult_t {
    done(comm(c).and_then(|_| {
        *d.as_mut().ok_or(ncclResult_t::InvalidArgument)? = 0;
        Ok(())
    }))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclCommUserRank(c: ncclComm_t, r: *mut c_int) -> ncclResult_t {
    done(comm(c).and_then(|_| {
        *r.as_mut().ok_or(ncclResult_t::InvalidArgument)? = 0;
        Ok(())
    }))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclCommRegister(c: ncclComm_t, _buf: *mut c_void, _size: usize, handle: *mut *mut c_void) -> ncclResult_t {
    done(comm(c).and_then(|_| {
        *handle.as_mut().ok_or(ncclResult_t::InvalidArgument)? = std::ptr::null_mut();
        Ok(())
    }))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclCommDeregister(c: ncclComm_t, _handle: *mut c_void) -> ncclResult_t {
    done(comm(c).map(|_| ()))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclMemAlloc(p: *mut *mut c_void, size: usize) -> ncclResult_t {
    match p.as_mut() {
        Some(p) => match engine::alloc(size) {
            Ok(m) => {
                *p = m;
                ncclResult_t::Success
            }
            Err(_) => ncclResult_t::UnhandledCudaError,
        },
        None => ncclResult_t::InvalidArgument,
    }
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclMemFree(p: *mut c_void) -> ncclResult_t {
    done(engine::free(p).map_err(status))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclRedOpCreatePreMulSum(
    op: *mut ncclRedOp_t,
    scalar: *mut c_void,
    ty: ncclDataType_t,
    residence: ncclScalarResidence_t,
    c: ncclComm_t,
) -> ncclResult_t {
    done((|| {
        let c = comm(c)?;
        let op = op.as_mut().ok_or(ncclResult_t::InvalidArgument)?;
        if scalar.is_null() {
            return Err(ncclResult_t::InvalidArgument);
        }
        let n = type_size(ty)?;
        let mut v = [0u8; 8];
        match residence {
            ncclScalarResidence_t::HostImmediate => std::ptr::copy_nonoverlapping(scalar as *const u8, v.as_mut_ptr(), n),
            ncclScalarResidence_t::Device => engine::read(v.as_mut_ptr() as *mut c_void, scalar, n).map_err(status)?,
            _ => return Err(ncclResult_t::InvalidArgument),
        }
        let scale = match ty {
            ncclDataType_t::Float32 => f32::from_le_bytes(v[..4].try_into().unwrap()) as f64,
            ncclDataType_t::Float64 => f64::from_le_bytes(v),
            _ => return Err(ncclResult_t::InvalidArgument),
        };
        c.ops.push(Some((scale, ty)));
        *op = ncclRedOp_t(FIRST_CUSTOM_OP + c.ops.len() as u32 - 1);
        Ok(())
    })())
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclRedOpDestroy(op: ncclRedOp_t, c: ncclComm_t) -> ncclResult_t {
    done(comm(c).and_then(|c| {
        let slot = op.0.checked_sub(FIRST_CUSTOM_OP).and_then(|i| c.ops.get_mut(i as usize)).ok_or(ncclResult_t::InvalidArgument)?;
        *slot = None;
        Ok(())
    }))
}

unsafe fn collective(src: *const c_void, dst: *mut c_void, count: usize, ty: ncclDataType_t, op: ncclRedOp_t, c: ncclComm_t, stream: CUstream) -> ncclResult_t {
    done((|| {
        let c = comm(c)?;
        if count > 0 && (src.is_null() || dst.is_null()) {
            return Err(ncclResult_t::InvalidArgument);
        }
        let steps = c.reduce(src as usize, dst as usize, count, ty, op)?;
        run(&steps, stream)
    })())
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclAllReduce(
    send: *const c_void,
    recv: *mut c_void,
    count: usize,
    ty: ncclDataType_t,
    op: ncclRedOp_t,
    c: ncclComm_t,
    stream: CUstream,
) -> ncclResult_t {
    collective(send, recv, count, ty, op, c, stream)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclReduce(
    send: *const c_void,
    recv: *mut c_void,
    count: usize,
    ty: ncclDataType_t,
    op: ncclRedOp_t,
    root: c_int,
    c: ncclComm_t,
    stream: CUstream,
) -> ncclResult_t {
    if root != 0 {
        return ncclResult_t::InvalidArgument;
    }
    collective(send, recv, count, ty, op, c, stream)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclReduceScatter(
    send: *const c_void,
    recv: *mut c_void,
    count: usize,
    ty: ncclDataType_t,
    op: ncclRedOp_t,
    c: ncclComm_t,
    stream: CUstream,
) -> ncclResult_t {
    collective(send, recv, count, ty, op, c, stream)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclBroadcast(
    send: *const c_void,
    recv: *mut c_void,
    count: usize,
    ty: ncclDataType_t,
    root: c_int,
    c: ncclComm_t,
    stream: CUstream,
) -> ncclResult_t {
    if root != 0 {
        return ncclResult_t::InvalidArgument;
    }
    collective(send, recv, count, ty, ncclRedOp_t(0), c, stream)
}

/// In place on the root's buffer: nothing to do on one rank.
#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclBcast(buf: *mut c_void, count: usize, ty: ncclDataType_t, root: c_int, c: ncclComm_t, stream: CUstream) -> ncclResult_t {
    ncclBroadcast(buf, buf, count, ty, root, c, stream)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclAllGather(
    send: *const c_void,
    recv: *mut c_void,
    count: usize,
    ty: ncclDataType_t,
    c: ncclComm_t,
    stream: CUstream,
) -> ncclResult_t {
    collective(send, recv, count, ty, ncclRedOp_t(0), c, stream)
}

unsafe fn p2p(buf: *mut c_void, count: usize, ty: ncclDataType_t, peer: c_int, c: ncclComm_t, stream: CUstream, send: bool) -> ncclResult_t {
    done((|| {
        let c = comm(c)?;
        if peer != 0 {
            return Err(ncclResult_t::InvalidArgument);
        }
        let bytes = count * type_size(ty)?;
        let me = Some((buf as usize, bytes, stream));
        let paired = if send { c.pair(me, None) } else { c.pair(None, me) };
        if let Some((dst, src, bytes, sstream, rstream)) = paired {
            if sstream.0 != rstream.0 {
                // the receive's stream must see the sent data
                engine::synchronize(sstream).map_err(status)?;
            }
            run(&[Step::Copy { dst, src, bytes }], rstream)?;
        }
        Ok(())
    })())
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclSend(send: *const c_void, count: usize, ty: ncclDataType_t, peer: c_int, c: ncclComm_t, stream: CUstream) -> ncclResult_t {
    p2p(send as *mut c_void, count, ty, peer, c, stream, true)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclRecv(recv: *mut c_void, count: usize, ty: ncclDataType_t, peer: c_int, c: ncclComm_t, stream: CUstream) -> ncclResult_t {
    p2p(recv, count, ty, peer, c, stream, false)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclGroupStart() -> ncclResult_t {
    *GROUP.lock().unwrap() += 1;
    ncclResult_t::Success
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclGroupEnd() -> ncclResult_t {
    let mut g = GROUP.lock().unwrap();
    if *g == 0 {
        return ncclResult_t::InvalidUsage;
    }
    *g -= 1;
    ncclResult_t::Success
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn ncclCommInitRankScalable(
    c: *mut ncclComm_t,
    nranks: c_int,
    rank: c_int,
    _nids: c_int,
    _ids: *mut ncclUniqueId,
    _config: *mut c_void,
) -> ncclResult_t {
    init(c, nranks, rank)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(n: usize) -> CUstream {
        CUstream(n as *mut _)
    }

    #[test]
    fn single_rank_communicators() {
        unsafe {
            let mut id = ncclUniqueId { internal: [0; 128] };
            assert_eq!(ncclGetUniqueId(&mut id), ncclResult_t::Success);
            let mut c = std::ptr::null_mut();
            assert_eq!(ncclCommInitRank(&mut c, 2, id, 0), ncclResult_t::InvalidUsage);
            assert_eq!(ncclCommInitRank(&mut c, 1, id, 0), ncclResult_t::Success);
            let mut n = 0;
            ncclCommCount(c, &mut n);
            assert_eq!(n, 1);
            let mut split = std::ptr::null_mut();
            assert_eq!(ncclCommSplit(c, -1, 0, &mut split, std::ptr::null_mut()), ncclResult_t::Success);
            assert!(split.is_null());
            ncclCommSplit(c, 3, 0, &mut split, std::ptr::null_mut());
            assert!(!split.is_null());
            ncclCommDestroy(split);
            assert_eq!(
                ncclReduce(8 as *const c_void, 16 as *mut c_void, 4, ncclDataType_t::Float32, ncclRedOp_t(0), 1, c, s(0)),
                ncclResult_t::InvalidArgument
            );
            ncclCommDestroy(c);
            let mut v = 0;
            ncclGetVersion(&mut v);
            assert_eq!(v, 22105);
        }
    }

    #[test]
    fn reductions_are_copies_and_premulsum_scales() {
        let mut c = Comm::new();
        assert_eq!(c.reduce(100, 200, 10, ncclDataType_t::Float16, ncclRedOp_t(4)), Ok(vec![Step::Copy { dst: 200, src: 100, bytes: 20 }]));
        let mut scale = 0.25f64;
        let cp = &mut c as *mut Comm;
        let mut op = ncclRedOp_t(0);
        unsafe {
            assert_eq!(
                ncclRedOpCreatePreMulSum(&mut op, &mut scale as *mut f64 as *mut c_void, ncclDataType_t::Float64, ncclScalarResidence_t::HostImmediate, cp),
                ncclResult_t::Success
            );
        }
        assert_eq!(op, ncclRedOp_t(FIRST_CUSTOM_OP));
        assert_eq!(
            c.reduce(100, 100, 3, ncclDataType_t::Float64, op),
            Ok(vec![Step::Copy { dst: 100, src: 100, bytes: 24 }, Step::Scale { dst: 100, n: 3, double: true, scale: 0.25 }])
        );
        assert_eq!(c.reduce(100, 100, 3, ncclDataType_t::Float32, op), Err(ncclResult_t::InvalidArgument));
        unsafe { ncclRedOpDestroy(op, cp) };
        assert_eq!(c.reduce(100, 100, 3, ncclDataType_t::Float64, op), Err(ncclResult_t::InvalidArgument));
    }

    #[test]
    fn sends_and_receives_pair_in_order() {
        let mut c = Comm::new();
        assert_eq!(c.pair(Some((1, 8, s(0))), None), None);
        assert_eq!(c.pair(Some((2, 8, s(0))), None), None);
        assert_eq!(c.pair(None, Some((10, 4, s(1)))).map(|p| (p.0, p.1, p.2)), Some((10, 1, 4)));
        assert_eq!(c.pair(None, Some((20, 8, s(1)))).map(|p| (p.0, p.1, p.2)), Some((20, 2, 8)));
        assert_eq!(c.pair(None, Some((30, 8, s(1)))), None);
    }
}
