//! cuFFT on Apple GPUs for zluda_agx (Asahi Linux), on the FFT kernels of agx_libs (`agx_libs/kernels/fft.mlir`,
//! written once for every AGX generation).
//!
//! Single and double precision: complex to complex transforms (C2C, Z2Z) of rank 1, 2 and 3 (strided and batched for
//! rank 1), and real to complex / complex to real transforms (R2C, C2R, D2Z, Z2D) of rank 1. Power-of-two lengths run
//! on radix-2 Stockham passes, others on a direct transform; the twiddles are computed on the host in double precision.
//! Double precision runs the same kernels built for f64 (`<kernel>_f64`, in the backend's software double precision).
//! Like cuFFT, inverse transforms are not normalized. Multi-GPU (Xt) functions return `CUFFT_NOT_SUPPORTED`.

#![allow(non_snake_case)]

use cuda_types::cufft::*;
use std::collections::HashMap;
use std::ffi::{c_int, c_void};
use std::sync::Mutex;

macro_rules! unimplemented_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes_definitions, unused_variables)]
            pub extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                if std::env::var_os("ZLUDA_AGX_LOG").is_some() {
                    eprintln!("[zluda_fft_agx] {} is not implemented", stringify!($fn_name));
                }
                Err(cufftError_t::NOT_SUPPORTED)
            }
        )*
    };
}

macro_rules! implemented_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                imp::$fn_name($($arg_id),*)
            }
        )*
    };
}

cuda_macros::cufft_function_declarations!(
    unimplemented_fn,
    implemented_fn
        <= [
            cufftCreate,
            cufftDestroy,
            cufftEstimate1d,
            cufftEstimate2d,
            cufftEstimate3d,
            cufftEstimateMany,
            cufftExecC2C,
            cufftExecC2R,
            cufftExecD2Z,
            cufftExecR2C,
            cufftExecZ2D,
            cufftExecZ2Z,
            cufftGetProperty,
            cufftGetSize,
            cufftGetSize1d,
            cufftGetSize2d,
            cufftGetSize3d,
            cufftGetSizeMany,
            cufftGetVersion,
            cufftMakePlan1d,
            cufftMakePlan2d,
            cufftMakePlan3d,
            cufftMakePlanMany,
            cufftPlan1d,
            cufftPlan2d,
            cufftPlan3d,
            cufftPlanMany,
            cufftSetAutoAllocation,
            cufftSetStream,
            cufftSetWorkArea,
        ]
);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    C2C,
    R2C,
    C2R,
}

/// A buffer of a transform: the user's input and output, the plan's work buffers, and the twiddles of a length.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Buf {
    In,
    Out,
    W0,
    W1,
    A,
    Tw(usize),
}

/// One kernel launch (fft.mlir), with element offsets (in complex or real elements of the buffer) applied to the
/// pointers.
#[derive(Clone, Debug, PartialEq)]
pub enum Step {
    Radix2 { src: Buf, dst: Buf, tw: usize, n: usize, ns: usize, count: usize, stride: usize, dist: usize, offset: usize },
    Dft { src: Buf, dst: Buf, tw: usize, n: usize, count: usize, stride: usize, dist: usize, offset: usize },
    Copy { src: Buf, dst: Buf, m: usize, count: usize, istride: usize, idist: usize, ostride: usize, odist: usize, ioffset: usize, ooffset: usize },
    RealToComplex { src: Buf, dst: Buf, n: usize, count: usize, idist: usize },
    Hermitian { src: Buf, dst: Buf, n: usize, count: usize, idist: usize },
    RealPart { src: Buf, dst: Buf, n: usize, count: usize, odist: usize },
}

/// A plan's shape.
#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    pub kind: Kind,
    /// cuFFT order: the first dimension varies slowest.
    pub dims: Vec<usize>,
    pub batch: usize,
    /// Double precision (Z2Z, D2Z, Z2D): complex elements of 16 bytes, reals of 8.
    pub double: bool,
    /// Element strides and sequence distances (rank 1; contiguous for higher ranks).
    pub istride: usize,
    pub idist: usize,
    pub ostride: usize,
    pub odist: usize,
}

impl Shape {
    /// The default (contiguous) layout cuFFT uses without embedding arrays.
    pub fn contiguous(kind: Kind, dims: &[usize], batch: usize) -> Shape {
        let total: usize = dims.iter().product();
        let last = *dims.last().unwrap_or(&1);
        let half = total / last * (last / 2 + 1);
        let (idist, odist) = match kind {
            Kind::C2C => (total, total),
            Kind::R2C => (total, half),
            Kind::C2R => (half, total),
        };
        Shape { kind, dims: dims.to_vec(), batch, double: false, istride: 1, idist, ostride: 1, odist }
    }

    /// Bytes of a complex element.
    pub fn complex_bytes(&self) -> usize {
        if self.double { 16 } else { 8 }
    }

    pub fn total(&self) -> usize {
        self.dims.iter().product()
    }

    /// The size of each work buffer, in complex elements.
    pub fn work_elements(&self) -> usize {
        match self.kind {
            Kind::C2C => (self.batch - 1) * self.odist + (self.total() - 1) * self.ostride + 1,
            _ => self.total() * self.batch,
        }
    }

    pub fn check(&self) -> Result<(), cufftError_t> {
        if self.dims.is_empty() || self.dims.len() > 3 || self.dims.iter().any(|&n| n == 0) || self.batch == 0 {
            return Err(cufftError_t::INVALID_SIZE);
        }
        if self.dims.len() > 1 && *self != (Shape { double: self.double, ..Shape::contiguous(self.kind, &self.dims, self.batch) }) {
            return Err(cufftError_t::NOT_SUPPORTED);
        }
        match self.kind {
            // the kernels read and write each sequence with one layout
            Kind::C2C if self.istride != self.ostride || self.idist != self.odist => Err(cufftError_t::NOT_SUPPORTED),
            Kind::R2C | Kind::C2R if self.dims.len() > 1 || self.istride != 1 || self.ostride != 1 => {
                Err(cufftError_t::NOT_SUPPORTED)
            }
            _ => Ok(()),
        }
    }
}

/// A 1-D complex transform of `count` sequences of length n (element `i` of sequence `c` at offset + c dist + i
/// stride) from src to dst, which may be the same buffer. `tw` names the twiddles of length n.
#[allow(clippy::too_many_arguments)]
fn c2c_1d(out: &mut Vec<Step>, src: Buf, dst: Buf, tw: usize, n: usize, count: usize, stride: usize, dist: usize, offset: usize) {
    let copy = |src, dst| Step::Copy {
        src,
        dst,
        m: n,
        count,
        istride: stride,
        idist: dist,
        ostride: stride,
        odist: dist,
        ioffset: offset,
        ooffset: offset,
    };
    if n == 1 {
        if src != dst {
            out.push(copy(src, dst));
        }
        return;
    }
    if !n.is_power_of_two() {
        let to = if src == dst { Buf::W0 } else { dst };
        out.push(Step::Dft { src, dst: to, tw, n, count, stride, dist, offset });
        if to != dst {
            out.push(copy(Buf::W0, dst));
        }
        return;
    }
    let passes = n.trailing_zeros() as usize;
    let mut from = src;
    for p in 0..passes {
        let last = p + 1 == passes;
        // a pass cannot run in place, so a single in-place pass goes through W0
        let to = if last && from != dst { dst } else if p % 2 == 0 { Buf::W0 } else { Buf::W1 };
        out.push(Step::Radix2 { src: from, dst: to, tw, n, ns: 1 << p, count, stride, dist, offset });
        from = to;
    }
    if from != dst {
        out.push(copy(from, dst));
    }
}

/// The launches of a transform; `in_place` when the input and output are the same buffer (the passes then never
/// read and write one buffer in one launch).
pub fn steps(s: &Shape, in_place: bool) -> Vec<Step> {
    let mut out = Vec::new();
    let input = if in_place { Buf::Out } else { Buf::In };
    match s.kind {
        Kind::C2C => {
            let r = s.dims.len();
            if r == 1 {
                c2c_1d(&mut out, input, Buf::Out, 0, s.dims[0], s.batch, s.istride, s.idist, 0);
                return out;
            }
            let total = s.total();
            for k in (0..r).rev() {
                let src = if k == r - 1 { input } else { Buf::Out };
                let n = s.dims[k];
                let inner: usize = s.dims[k + 1..].iter().product();
                let outer: usize = s.dims[..k].iter().product();
                if inner == 1 {
                    c2c_1d(&mut out, src, Buf::Out, k, n, outer * s.batch, 1, n, 0);
                } else {
                    for b in 0..s.batch {
                        for o in 0..outer {
                            c2c_1d(&mut out, src, Buf::Out, k, n, inner, inner, 1, b * total + o * n * inner);
                        }
                    }
                }
            }
        }
        Kind::R2C => {
            let n = s.dims[0];
            out.push(Step::RealToComplex { src: Buf::In, dst: Buf::A, n, count: s.batch, idist: s.idist });
            c2c_1d(&mut out, Buf::A, Buf::A, 0, n, s.batch, 1, n, 0);
            out.push(Step::Copy {
                src: Buf::A,
                dst: Buf::Out,
                m: n / 2 + 1,
                count: s.batch,
                istride: 1,
                idist: n,
                ostride: 1,
                odist: s.odist,
                ioffset: 0,
                ooffset: 0,
            });
        }
        Kind::C2R => {
            let n = s.dims[0];
            out.push(Step::Hermitian { src: Buf::In, dst: Buf::A, n, count: s.batch, idist: s.idist });
            c2c_1d(&mut out, Buf::A, Buf::A, 0, n, s.batch, 1, n, 0);
            out.push(Step::RealPart { src: Buf::A, dst: Buf::Out, n, count: s.batch, odist: s.odist });
        }
    }
    out
}

/// exp(-2 pi i t / n) for t = 0 ... n - 1, interleaved, in double precision.
pub fn twiddles_f64(n: usize) -> Vec<f64> {
    let mut out = Vec::with_capacity(2 * n);
    for t in 0..n {
        let a = -2.0 * std::f64::consts::PI * (t as f64) / (n as f64);
        out.push(a.cos());
        out.push(a.sin());
    }
    out
}

/// exp(-2 pi i t / n) for t = 0 ... n - 1, interleaved, rounded from double precision.
pub fn twiddles(n: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(2 * n);
    for t in 0..n {
        let a = -2.0 * std::f64::consts::PI * (t as f64) / (n as f64);
        out.push(a.cos() as f32);
        out.push(a.sin() as f32);
    }
    out
}

mod imp {
    use super::*;
    use agx_libs::engine;

    struct Plan {
        shape: Option<Shape>,
        stream: cudaStream_t,
        /// Device buffers: W0, W1, A (complex work buffers) and one twiddle table per dimension.
        work: [*mut c_void; 3],
        tw: Vec<*mut c_void>,
    }
    unsafe impl Send for Plan {}

    impl Plan {
        unsafe fn release(&mut self) {
            for p in self.work.iter().chain(&self.tw) {
                let _ = engine::free(*p);
            }
            self.work = [std::ptr::null_mut(); 3];
            self.tw.clear();
            self.shape = None;
        }
    }

    static PLANS: Mutex<Option<(HashMap<c_int, Plan>, c_int)>> = Mutex::new(None);

    fn with<T>(h: cufftHandle, f: impl FnOnce(&mut Plan) -> Result<T, cufftError_t>) -> Result<T, cufftError_t> {
        let mut g = PLANS.lock().unwrap();
        let plans = &mut g.get_or_insert_with(|| (HashMap::new(), 1)).0;
        f(plans.get_mut(&h.0).ok_or(cufftError_t::INVALID_PLAN)?)
    }

    fn status(e: engine::Error) -> cufftError_t {
        match e {
            engine::Error::NotInitialized => cufftError_t::SETUP_FAILED,
            engine::Error::ArchMismatch => cufftError_t::SETUP_FAILED,
            engine::Error::Failed => cufftError_t::EXEC_FAILED,
        }
    }

    unsafe fn out<T>(p: *mut T, v: T) -> cufftResult {
        if let Some(p) = p.as_mut() {
            *p = v;
        }
        Ok(())
    }

    /// The kind of a transform type and whether it is double precision.
    fn kind(t: cufftType) -> Result<(Kind, bool), cufftError_t> {
        match t {
            cufftType::CUFFT_C2C => Ok((Kind::C2C, false)),
            cufftType::CUFFT_R2C => Ok((Kind::R2C, false)),
            cufftType::CUFFT_C2R => Ok((Kind::C2R, false)),
            cufftType::CUFFT_Z2Z => Ok((Kind::C2C, true)),
            cufftType::CUFFT_D2Z => Ok((Kind::R2C, true)),
            cufftType::CUFFT_Z2D => Ok((Kind::C2R, true)),
            _ => Err(cufftError_t::INVALID_TYPE),
        }
    }

    fn contiguous(t: cufftType, dims: &[usize], batch: usize) -> Result<Shape, cufftError_t> {
        let (k, double) = kind(t)?;
        Ok(Shape { double, ..Shape::contiguous(k, dims, batch) })
    }

    pub(crate) unsafe fn cufftCreate(h: *mut cufftHandle) -> cufftResult {
        if !engine::available() {
            return Err(cufftError_t::SETUP_FAILED);
        }
        let mut g = PLANS.lock().unwrap();
        let (plans, next) = g.get_or_insert_with(|| (HashMap::new(), 1));
        let id = *next;
        *next += 1;
        plans.insert(
            id,
            Plan { shape: None, stream: cuda_types::cuda::CUstream(std::ptr::null_mut()), work: [std::ptr::null_mut(); 3], tw: Vec::new() },
        );
        *h.as_mut().ok_or(cufftError_t::INVALID_VALUE)? = cufftHandle(id);
        Ok(())
    }

    pub(crate) unsafe fn cufftDestroy(h: cufftHandle) -> cufftResult {
        let mut g = PLANS.lock().unwrap();
        let plans = &mut g.get_or_insert_with(|| (HashMap::new(), 1)).0;
        let mut p = plans.remove(&h.0).ok_or(cufftError_t::INVALID_PLAN)?;
        p.release();
        Ok(())
    }

    unsafe fn make(h: cufftHandle, s: Shape, work: *mut usize) -> cufftResult {
        s.check()?;
        let bytes = s.complex_bytes() * s.work_elements();
        with(h, |p| {
            p.release();
            for w in &mut p.work {
                *w = engine::alloc(bytes).map_err(|_| cufftError_t::ALLOC_FAILED)?;
            }
            for &n in &s.dims {
                let (t32, t64);
                let (ptr, len) = if s.double {
                    t64 = twiddles_f64(n);
                    (t64.as_ptr() as *const c_void, 8 * t64.len())
                } else {
                    t32 = twiddles(n);
                    (t32.as_ptr() as *const c_void, 4 * t32.len())
                };
                let d = engine::alloc(len).map_err(|_| cufftError_t::ALLOC_FAILED)?;
                p.tw.push(d);
                engine::write(d, ptr, len).map_err(status)?;
            }
            p.shape = Some(s);
            Ok(())
        })?;
        out(work, 3 * bytes)
    }

    fn shape_many(rank: c_int, n: *mut c_int, inembed: *mut c_int, istride: c_int, idist: c_int, onembed: *mut c_int, ostride: c_int, odist: c_int, t: cufftType, batch: c_int) -> Result<Shape, cufftError_t> {
        if !(1..=3).contains(&rank) || n.is_null() || batch < 1 {
            return Err(cufftError_t::INVALID_VALUE);
        }
        let dims: Vec<usize> = unsafe { std::slice::from_raw_parts(n, rank as usize) }.iter().map(|&d| d.max(0) as usize).collect();
        let mut s = contiguous(t, &dims, batch as usize)?;
        // with both embedding arrays NULL the layout is contiguous and the strides and distances are ignored
        if !inembed.is_null() || !onembed.is_null() {
            if istride < 1 || ostride < 1 || idist < 1 || odist < 1 {
                return Err(cufftError_t::INVALID_VALUE);
            }
            s.istride = istride as usize;
            s.ostride = ostride as usize;
            s.idist = idist as usize;
            s.odist = odist as usize;
        }
        Ok(s)
    }

    pub(crate) unsafe fn cufftMakePlan1d(h: cufftHandle, nx: c_int, t: cufftType, batch: c_int, work: *mut usize) -> cufftResult {
        if nx < 1 || batch < 1 {
            return Err(cufftError_t::INVALID_SIZE);
        }
        make(h, contiguous(t, &[nx as usize], batch as usize)?, work)
    }

    pub(crate) unsafe fn cufftMakePlan2d(h: cufftHandle, nx: c_int, ny: c_int, t: cufftType, work: *mut usize) -> cufftResult {
        if nx < 1 || ny < 1 {
            return Err(cufftError_t::INVALID_SIZE);
        }
        make(h, contiguous(t, &[nx as usize, ny as usize], 1)?, work)
    }

    pub(crate) unsafe fn cufftMakePlan3d(h: cufftHandle, nx: c_int, ny: c_int, nz: c_int, t: cufftType, work: *mut usize) -> cufftResult {
        if nx < 1 || ny < 1 || nz < 1 {
            return Err(cufftError_t::INVALID_SIZE);
        }
        make(h, contiguous(t, &[nx as usize, ny as usize, nz as usize], 1)?, work)
    }

    pub(crate) unsafe fn cufftMakePlanMany(
        h: cufftHandle,
        rank: c_int,
        n: *mut c_int,
        inembed: *mut c_int,
        istride: c_int,
        idist: c_int,
        onembed: *mut c_int,
        ostride: c_int,
        odist: c_int,
        t: cufftType,
        batch: c_int,
        work: *mut usize,
    ) -> cufftResult {
        make(h, shape_many(rank, n, inembed, istride, idist, onembed, ostride, odist, t, batch)?, work)
    }

    unsafe fn plan(out_h: *mut cufftHandle, f: impl FnOnce(cufftHandle) -> cufftResult) -> cufftResult {
        cufftCreate(out_h)?;
        let h = *out_h;
        f(h).inspect_err(|_| {
            let _ = cufftDestroy(h);
        })
    }

    pub(crate) unsafe fn cufftPlan1d(h: *mut cufftHandle, nx: c_int, t: cufftType, batch: c_int) -> cufftResult {
        plan(h, |h| cufftMakePlan1d(h, nx, t, batch, std::ptr::null_mut()))
    }

    pub(crate) unsafe fn cufftPlan2d(h: *mut cufftHandle, nx: c_int, ny: c_int, t: cufftType) -> cufftResult {
        plan(h, |h| cufftMakePlan2d(h, nx, ny, t, std::ptr::null_mut()))
    }

    pub(crate) unsafe fn cufftPlan3d(h: *mut cufftHandle, nx: c_int, ny: c_int, nz: c_int, t: cufftType) -> cufftResult {
        plan(h, |h| cufftMakePlan3d(h, nx, ny, nz, t, std::ptr::null_mut()))
    }

    pub(crate) unsafe fn cufftPlanMany(
        h: *mut cufftHandle,
        rank: c_int,
        n: *mut c_int,
        inembed: *mut c_int,
        istride: c_int,
        idist: c_int,
        onembed: *mut c_int,
        ostride: c_int,
        odist: c_int,
        t: cufftType,
        batch: c_int,
    ) -> cufftResult {
        plan(h, |h| cufftMakePlanMany(h, rank, n, inembed, istride, idist, onembed, ostride, odist, t, batch, std::ptr::null_mut()))
    }

    // Workspace sizes: three complex buffers of the output's extent.
    fn work_of(s: Shape, work: *mut usize) -> cufftResult {
        s.check()?;
        unsafe { out(work, 3 * s.complex_bytes() * s.work_elements()) }
    }

    pub(crate) unsafe fn cufftEstimate1d(nx: c_int, t: cufftType, batch: c_int, work: *mut usize) -> cufftResult {
        work_of(contiguous(t, &[nx.max(0) as usize], batch.max(0) as usize)?, work)
    }

    pub(crate) unsafe fn cufftEstimate2d(nx: c_int, ny: c_int, t: cufftType, work: *mut usize) -> cufftResult {
        work_of(contiguous(t, &[nx.max(0) as usize, ny.max(0) as usize], 1)?, work)
    }

    pub(crate) unsafe fn cufftEstimate3d(nx: c_int, ny: c_int, nz: c_int, t: cufftType, work: *mut usize) -> cufftResult {
        work_of(contiguous(t, &[nx.max(0) as usize, ny.max(0) as usize, nz.max(0) as usize], 1)?, work)
    }

    pub(crate) unsafe fn cufftEstimateMany(
        rank: c_int,
        n: *mut c_int,
        inembed: *mut c_int,
        istride: c_int,
        idist: c_int,
        onembed: *mut c_int,
        ostride: c_int,
        odist: c_int,
        t: cufftType,
        batch: c_int,
        work: *mut usize,
    ) -> cufftResult {
        work_of(shape_many(rank, n, inembed, istride, idist, onembed, ostride, odist, t, batch)?, work)
    }

    pub(crate) unsafe fn cufftGetSize1d(_h: cufftHandle, nx: c_int, t: cufftType, batch: c_int, work: *mut usize) -> cufftResult {
        cufftEstimate1d(nx, t, batch, work)
    }

    pub(crate) unsafe fn cufftGetSize2d(_h: cufftHandle, nx: c_int, ny: c_int, t: cufftType, work: *mut usize) -> cufftResult {
        cufftEstimate2d(nx, ny, t, work)
    }

    pub(crate) unsafe fn cufftGetSize3d(_h: cufftHandle, nx: c_int, ny: c_int, nz: c_int, t: cufftType, work: *mut usize) -> cufftResult {
        cufftEstimate3d(nx, ny, nz, t, work)
    }

    pub(crate) unsafe fn cufftGetSizeMany(
        _h: cufftHandle,
        rank: c_int,
        n: *mut c_int,
        inembed: *mut c_int,
        istride: c_int,
        idist: c_int,
        onembed: *mut c_int,
        ostride: c_int,
        odist: c_int,
        t: cufftType,
        batch: c_int,
        work: *mut usize,
    ) -> cufftResult {
        cufftEstimateMany(rank, n, inembed, istride, idist, onembed, ostride, odist, t, batch, work)
    }

    pub(crate) unsafe fn cufftGetSize(h: cufftHandle, work: *mut usize) -> cufftResult {
        let s = with(h, |p| p.shape.clone().ok_or(cufftError_t::INVALID_PLAN))?;
        work_of(s, work)
    }

    /// The plan keeps its own work buffers.
    pub(crate) unsafe fn cufftSetWorkArea(h: cufftHandle, _work: *mut c_void) -> cufftResult {
        with(h, |_| Ok(()))
    }

    pub(crate) unsafe fn cufftSetAutoAllocation(h: cufftHandle, _auto: c_int) -> cufftResult {
        with(h, |_| Ok(()))
    }

    pub(crate) unsafe fn cufftSetStream(h: cufftHandle, s: cudaStream_t) -> cufftResult {
        with(h, |p| {
            p.stream = s;
            Ok(())
        })
    }

    pub(crate) unsafe fn cufftGetVersion(v: *mut c_int) -> cufftResult {
        out(v, 11_300)
    }

    pub(crate) unsafe fn cufftGetProperty(t: libraryPropertyType, v: *mut c_int) -> cufftResult {
        let x = match t {
            libraryPropertyType::MAJOR_VERSION => 11,
            libraryPropertyType::MINOR_VERSION => 3,
            _ => 0,
        };
        out(v, x)
    }

    unsafe fn exec(h: cufftHandle, input: *mut c_void, output: *mut c_void, want: Kind, double: bool, inverse: bool) -> cufftResult {
        let (shape, stream, work, tw) = with(h, |p| {
            Ok((p.shape.clone().ok_or(cufftError_t::INVALID_PLAN)?, p.stream, p.work, p.tw.clone()))
        })?;
        if shape.kind != want || shape.double != double {
            return Err(cufftError_t::INVALID_TYPE);
        }
        let cb = shape.complex_bytes();
        if input.is_null() || output.is_null() {
            return Err(cufftError_t::INVALID_VALUE);
        }
        let ptr = |b: Buf| -> *mut c_void {
            match b {
                Buf::In => input,
                Buf::Out => output,
                Buf::W0 => work[0],
                Buf::W1 => work[1],
                Buf::A => work[2],
                Buf::Tw(k) => tw[k],
            }
        };
        // the single precision kernel, or its double precision copy
        let func = |name: &'static std::ffi::CStr, name64: &'static std::ffi::CStr| {
            engine::function(agx_libs::FFT, if double { name64 } else { name }).map_err(status)
        };
        let groups = |threads: usize, block: u32| (threads.div_ceil(block as usize)) as u32;
        let inv = inverse as i32;
        for step in steps(&shape, input == output) {
            // arguments: pointers, then i32 values
            let (f, ptrs, ints, grid, block): (_, Vec<*mut c_void>, Vec<i32>, [u32; 2], [u32; 2]) = match step {
                Step::Radix2 { src, dst, tw, n, ns, count, stride, dist, offset } => {
                    let bx = (n / 2).clamp(1, 64) as u32;
                    (
                        func(c"agx_fft_radix2", c"agx_fft_radix2_f64")?,
                        vec![ptr(src).byte_add(cb * offset), ptr(dst).byte_add(cb * offset), ptr(Buf::Tw(tw))],
                        vec![n as i32, ns as i32, count as i32, stride as i32, dist as i32, inv],
                        [groups(n / 2, bx), count as u32],
                        [bx, 1],
                    )
                }
                Step::Dft { src, dst, tw, n, count, stride, dist, offset } => (
                    func(c"agx_dft", c"agx_dft_f64")?,
                    vec![ptr(src).byte_add(cb * offset), ptr(dst).byte_add(cb * offset), ptr(Buf::Tw(tw))],
                    vec![n as i32, count as i32, stride as i32, dist as i32, inv],
                    [groups(n, 16), count as u32],
                    [16, 1],
                ),
                Step::Copy { src, dst, m, count, istride, idist, ostride, odist, ioffset, ooffset } => (
                    func(c"agx_fft_copy", c"agx_fft_copy_f64")?,
                    vec![ptr(src).byte_add(cb * ioffset), ptr(dst).byte_add(cb * ooffset)],
                    vec![m as i32, count as i32, istride as i32, idist as i32, ostride as i32, odist as i32],
                    [groups(m, 16), count as u32],
                    [16, 1],
                ),
                Step::RealToComplex { src, dst, n, count, idist } => (
                    func(c"agx_fft_real_to_complex", c"agx_fft_real_to_complex_f64")?,
                    vec![ptr(src), ptr(dst)],
                    vec![n as i32, count as i32, idist as i32],
                    [groups(n, 16), count as u32],
                    [16, 1],
                ),
                Step::Hermitian { src, dst, n, count, idist } => (
                    func(c"agx_fft_hermitian", c"agx_fft_hermitian_f64")?,
                    vec![ptr(src), ptr(dst)],
                    vec![n as i32, count as i32, idist as i32],
                    [groups(n, 16), count as u32],
                    [16, 1],
                ),
                Step::RealPart { src, dst, n, count, odist } => (
                    func(c"agx_fft_real_part", c"agx_fft_real_part_f64")?,
                    vec![ptr(src), ptr(dst)],
                    vec![n as i32, count as i32, odist as i32],
                    [groups(n, 16), count as u32],
                    [16, 1],
                ),
            };
            let mut ptrs = ptrs;
            let mut ints = ints;
            let mut params: Vec<*mut c_void> = ptrs.iter_mut().map(|p| p as *mut *mut c_void as *mut c_void).collect();
            params.extend(ints.iter_mut().map(|v| v as *mut i32 as *mut c_void));
            engine::launch_kernel(f, grid, block, &mut params, stream).map_err(status)?;
        }
        Ok(())
    }

    pub(crate) unsafe fn cufftExecC2C(h: cufftHandle, i: *mut cufftComplex, o: *mut cufftComplex, direction: c_int) -> cufftResult {
        let inverse = match direction {
            CUFFT_FORWARD => false,
            d if d == CUFFT_INVERSE as c_int => true,
            _ => return Err(cufftError_t::INVALID_VALUE),
        };
        exec(h, i.cast(), o.cast(), Kind::C2C, false, inverse)
    }

    pub(crate) unsafe fn cufftExecR2C(h: cufftHandle, i: *mut cufftReal, o: *mut cufftComplex) -> cufftResult {
        exec(h, i.cast(), o.cast(), Kind::R2C, false, false)
    }

    pub(crate) unsafe fn cufftExecC2R(h: cufftHandle, i: *mut cufftComplex, o: *mut cufftReal) -> cufftResult {
        exec(h, i.cast(), o.cast(), Kind::C2R, false, true)
    }

    pub(crate) unsafe fn cufftExecZ2Z(
        h: cufftHandle,
        i: *mut cufftDoubleComplex,
        o: *mut cufftDoubleComplex,
        direction: c_int,
    ) -> cufftResult {
        let inverse = match direction {
            CUFFT_FORWARD => false,
            d if d == CUFFT_INVERSE as c_int => true,
            _ => return Err(cufftError_t::INVALID_VALUE),
        };
        exec(h, i.cast(), o.cast(), Kind::C2C, true, inverse)
    }

    pub(crate) unsafe fn cufftExecD2Z(h: cufftHandle, i: *mut cufftDoubleReal, o: *mut cufftDoubleComplex) -> cufftResult {
        exec(h, i.cast(), o.cast(), Kind::R2C, true, false)
    }

    pub(crate) unsafe fn cufftExecZ2D(h: cufftHandle, i: *mut cufftDoubleComplex, o: *mut cufftDoubleReal) -> cufftResult {
        exec(h, i.cast(), o.cast(), Kind::C2R, true, true)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    type C = (f64, f64);

    /// Runs the steps on the CPU with the semantics the kernels document.
    fn emulate(s: &Shape, inverse: bool, in_place: bool, bufs: &mut HashMap<Buf, Vec<f64>>) {
        let tw: Vec<Vec<f64>> = s.dims.iter().map(|&n| twiddles(n).iter().map(|&v| v as f64).collect()).collect();
        let get = |b: &HashMap<Buf, Vec<f64>>, buf: Buf, i: usize| -> C { (b[&buf][2 * i], b[&buf][2 * i + 1]) };
        for st in steps(s, in_place) {
            match st {
                Step::Radix2 { src, dst, tw: t, n, ns, count, stride, dist, offset } => {
                    let mut writes = Vec::new();
                    for c in 0..count {
                        for j in 0..n / 2 {
                            let base = offset + c * dist;
                            let a = get(bufs, src, base + j * stride);
                            let b = get(bufs, src, base + (j + n / 2) * stride);
                            let k = j % ns;
                            let ti = k * (n / (2 * ns));
                            let (wr, wi) = (tw[t][2 * ti], if inverse { -tw[t][2 * ti + 1] } else { tw[t][2 * ti + 1] });
                            let v = (b.0 * wr - b.1 * wi, b.0 * wi + b.1 * wr);
                            let jd = (j / ns) * 2 * ns + k;
                            writes.push((base + jd * stride, (a.0 + v.0, a.1 + v.1)));
                            writes.push((base + (jd + ns) * stride, (a.0 - v.0, a.1 - v.1)));
                        }
                    }
                    assert_ne!(src, dst, "a radix-2 pass in place");
                    let d = bufs.get_mut(&dst).unwrap();
                    for (i, v) in writes {
                        d[2 * i] = v.0;
                        d[2 * i + 1] = v.1;
                    }
                }
                Step::Dft { src, dst, tw: t, n, count, stride, dist, offset } => {
                    assert_ne!(src, dst);
                    let mut writes = Vec::new();
                    for c in 0..count {
                        let base = offset + c * dist;
                        for k in 0..n {
                            let mut acc = (0.0, 0.0);
                            for j in 0..n {
                                let x = get(bufs, src, base + j * stride);
                                let ti = (j * k) % n;
                                let (wr, wi) = (tw[t][2 * ti], if inverse { -tw[t][2 * ti + 1] } else { tw[t][2 * ti + 1] });
                                acc.0 += x.0 * wr - x.1 * wi;
                                acc.1 += x.0 * wi + x.1 * wr;
                            }
                            writes.push((base + k * stride, acc));
                        }
                    }
                    let d = bufs.get_mut(&dst).unwrap();
                    for (i, v) in writes {
                        d[2 * i] = v.0;
                        d[2 * i + 1] = v.1;
                    }
                }
                Step::Copy { src, dst, m, count, istride, idist, ostride, odist, ioffset, ooffset } => {
                    for c in 0..count {
                        for i in 0..m {
                            let v = get(bufs, src, ioffset + c * idist + i * istride);
                            let d = bufs.get_mut(&dst).unwrap();
                            let o = ooffset + c * odist + i * ostride;
                            d[2 * o] = v.0;
                            d[2 * o + 1] = v.1;
                        }
                    }
                }
                Step::RealToComplex { src, dst, n, count, idist } => {
                    for c in 0..count {
                        for i in 0..n {
                            let v = bufs[&src][c * idist + i];
                            let d = bufs.get_mut(&dst).unwrap();
                            d[2 * (c * n + i)] = v;
                            d[2 * (c * n + i) + 1] = 0.0;
                        }
                    }
                }
                Step::Hermitian { src, dst, n, count, idist } => {
                    for c in 0..count {
                        for k in 0..n {
                            let (j, sign) = if k <= n / 2 { (k, 1.0) } else { (n - k, -1.0) };
                            let v = get(bufs, src, c * idist + j);
                            let d = bufs.get_mut(&dst).unwrap();
                            d[2 * (c * n + k)] = v.0;
                            d[2 * (c * n + k) + 1] = sign * v.1;
                        }
                    }
                }
                Step::RealPart { src, dst, n, count, odist } => {
                    for c in 0..count {
                        for i in 0..n {
                            let v = get(bufs, src, c * n + i);
                            bufs.get_mut(&dst).unwrap()[c * odist + i] = v.0;
                        }
                    }
                }
            }
        }
    }

    fn rnd(seed: &mut u64) -> f64 {
        *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((*seed >> 11) as f64 / (1u64 << 53) as f64) - 0.5
    }

    /// The multi-dimensional DFT of a row-major array.
    fn dft_nd(x: &[C], dims: &[usize], inverse: bool) -> Vec<C> {
        let total: usize = dims.iter().product();
        let sign = if inverse { 1.0 } else { -1.0 };
        let idx = |mut f: usize| {
            let mut v = vec![0; dims.len()];
            for d in (0..dims.len()).rev() {
                v[d] = f % dims[d];
                f /= dims[d];
            }
            v
        };
        (0..total)
            .map(|k| {
                let kv = idx(k);
                let mut acc = (0.0, 0.0);
                for (j, xj) in x.iter().enumerate().take(total) {
                    let jv = idx(j);
                    let a: f64 = (0..dims.len()).map(|d| (jv[d] * kv[d]) as f64 / dims[d] as f64).sum::<f64>()
                        * 2.0
                        * std::f64::consts::PI
                        * sign;
                    acc.0 += xj.0 * a.cos() - xj.1 * a.sin();
                    acc.1 += xj.0 * a.sin() + xj.1 * a.cos();
                }
                acc
            })
            .collect()
    }

    fn run_c2c(dims: &[usize], batch: usize, inverse: bool, in_place: bool) {
        let s = Shape::contiguous(Kind::C2C, dims, batch);
        let mut seed = 1;
        let n = s.total() * batch;
        let input: Vec<f64> = (0..2 * n).map(|_| rnd(&mut seed)).collect();
        let mut bufs: HashMap<Buf, Vec<f64>> = HashMap::new();
        for b in [Buf::In, Buf::Out, Buf::W0, Buf::W1, Buf::A] {
            bufs.insert(b, vec![f64::NAN; 2 * s.work_elements()]);
        }
        bufs.insert(Buf::In, input.clone());
        if in_place {
            bufs.insert(Buf::Out, input.clone());
        }
        emulate(&s, inverse, in_place, &mut bufs);
        let out = &bufs[&Buf::Out];
        for b in 0..batch {
            let x: Vec<C> = (0..s.total()).map(|i| (input[2 * (b * s.total() + i)], input[2 * (b * s.total() + i) + 1])).collect();
            let want = dft_nd(&x, dims, inverse);
            for (i, w) in want.iter().enumerate() {
                let o = b * s.total() + i;
                let g = (out[2 * o], out[2 * o + 1]);
                assert!((g.0 - w.0).abs() < 1e-4 && (g.1 - w.1).abs() < 1e-4, "{dims:?} x{batch} inv={inverse}: {g:?} != {w:?} at {o}");
            }
        }
    }

    #[test]
    fn complex_transforms() {
        for inverse in [false, true] {
            run_c2c(&[8], 3, inverse, false);
            run_c2c(&[2], 2, inverse, false);
            run_c2c(&[12], 2, inverse, false);
            run_c2c(&[1], 4, inverse, false);
            run_c2c(&[4, 8], 2, inverse, false);
            run_c2c(&[3, 5], 1, inverse, false);
            run_c2c(&[2, 3, 4], 1, inverse, false);
            run_c2c(&[2], 3, inverse, true);
            run_c2c(&[5], 2, inverse, true);
            run_c2c(&[4, 8], 2, inverse, true);
            run_c2c(&[3, 2, 2], 1, inverse, true);
        }
    }

    #[test]
    fn in_place_and_strided_rank_1() {
        // in place: In and Out are the same buffer, which the emulation models by steps reading Out after writing it
        for &(n, batch, stride, dist) in &[(8usize, 3usize, 1usize, 8usize), (2, 2, 1, 2), (6, 2, 3, 1), (16, 4, 4, 1)] {
            let s = Shape { kind: Kind::C2C, dims: vec![n], batch, double: false, istride: stride, idist: dist, ostride: stride, odist: dist };
            s.check().unwrap();
            let mut seed = 5;
            let size = s.work_elements();
            let input: Vec<f64> = (0..2 * size).map(|_| rnd(&mut seed)).collect();
            let mut bufs: HashMap<Buf, Vec<f64>> = HashMap::new();
            for b in [Buf::W0, Buf::W1, Buf::A] {
                bufs.insert(b, vec![f64::NAN; 2 * size]);
            }
            bufs.insert(Buf::In, input.clone());
            bufs.insert(Buf::Out, input.clone());
            let st = steps(&s, true);
            assert!(st.iter().all(|x| !matches!(x, Step::Radix2 { src: Buf::In, .. } | Step::Dft { src: Buf::In, .. })));
            // emulate the steps
            let tw = twiddles(n);
            let mut b2 = bufs.clone();
            for x in &st {
                match *x {
                    Step::Radix2 { src, dst, n, ns, count, stride, dist, offset, .. } => {
                        assert_ne!(src, dst);
                        let mut w = Vec::new();
                        for c in 0..count {
                            for j in 0..n / 2 {
                                let base = offset + c * dist;
                                let (ia, ib) = (base + j * stride, base + (j + n / 2) * stride);
                                let a = (b2[&src][2 * ia], b2[&src][2 * ia + 1]);
                                let bb = (b2[&src][2 * ib], b2[&src][2 * ib + 1]);
                                let k = j % ns;
                                let t = k * (n / (2 * ns));
                                let (wr, wi) = (tw[2 * t] as f64, tw[2 * t + 1] as f64);
                                let v = (bb.0 * wr - bb.1 * wi, bb.0 * wi + bb.1 * wr);
                                let jd = (j / ns) * 2 * ns + k;
                                w.push((base + jd * stride, (a.0 + v.0, a.1 + v.1)));
                                w.push((base + (jd + ns) * stride, (a.0 - v.0, a.1 - v.1)));
                            }
                        }
                        let d = b2.get_mut(&dst).unwrap();
                        for (i, v) in w {
                            d[2 * i] = v.0;
                            d[2 * i + 1] = v.1;
                        }
                    }
                    Step::Dft { src, dst, n, count, stride, dist, offset, .. } => {
                        assert_ne!(src, dst);
                        let mut w = Vec::new();
                        for c in 0..count {
                            let base = offset + c * dist;
                            for k in 0..n {
                                let mut acc = (0.0, 0.0);
                                for j in 0..n {
                                    let i = base + j * stride;
                                    let x = (b2[&src][2 * i], b2[&src][2 * i + 1]);
                                    let t = (j * k) % n;
                                    let (wr, wi) = (tw[2 * t] as f64, tw[2 * t + 1] as f64);
                                    acc.0 += x.0 * wr - x.1 * wi;
                                    acc.1 += x.0 * wi + x.1 * wr;
                                }
                                w.push((base + k * stride, acc));
                            }
                        }
                        let d = b2.get_mut(&dst).unwrap();
                        for (i, v) in w {
                            d[2 * i] = v.0;
                            d[2 * i + 1] = v.1;
                        }
                    }
                    Step::Copy { src, dst, m, count, istride, idist, ostride, odist, ioffset, ooffset } => {
                        for c in 0..count {
                            for i in 0..m {
                                let si = ioffset + c * idist + i * istride;
                                let v = (b2[&src][2 * si], b2[&src][2 * si + 1]);
                                let o = ooffset + c * odist + i * ostride;
                                let d = b2.get_mut(&dst).unwrap();
                                d[2 * o] = v.0;
                                d[2 * o + 1] = v.1;
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
            let out = &b2[&Buf::Out];
            for c in 0..batch {
                let x: Vec<C> = (0..n).map(|i| (input[2 * (c * dist + i * stride)], input[2 * (c * dist + i * stride) + 1])).collect();
                let want = dft_nd(&x, &[n], false);
                for (i, w) in want.iter().enumerate() {
                    let o = c * dist + i * stride;
                    assert!((out[2 * o] - w.0).abs() < 1e-4 && (out[2 * o + 1] - w.1).abs() < 1e-4, "n={n} stride={stride}: element {o}");
                }
            }
            // elements outside the sequences are untouched
            let touched: Vec<usize> = (0..batch).flat_map(|c| (0..n).map(move |i| c * dist + i * stride)).collect();
            for e in 0..size {
                if !touched.contains(&e) {
                    assert_eq!(out[2 * e], input[2 * e], "n={n} stride={stride}: element {e} changed");
                }
            }
        }
    }

    #[test]
    fn real_transforms() {
        for &(n, batch) in &[(8usize, 2usize), (9, 1), (1, 3), (16, 1)] {
            let s = Shape::contiguous(Kind::R2C, &[n], batch);
            let mut seed = 9;
            let real: Vec<f64> = (0..n * batch).map(|_| rnd(&mut seed)).collect();
            let mut bufs: HashMap<Buf, Vec<f64>> = HashMap::new();
            for b in [Buf::W0, Buf::W1, Buf::A] {
                bufs.insert(b, vec![f64::NAN; 2 * s.work_elements()]);
            }
            bufs.insert(Buf::In, real.clone());
            bufs.insert(Buf::Out, vec![f64::NAN; 2 * s.odist * batch]);
            emulate(&s, false, false, &mut bufs);
            let spec = bufs[&Buf::Out].clone();
            for c in 0..batch {
                let x: Vec<C> = (0..n).map(|i| (real[c * n + i], 0.0)).collect();
                let want = dft_nd(&x, &[n], false);
                for k in 0..=n / 2 {
                    let o = c * s.odist + k;
                    assert!((spec[2 * o] - want[k].0).abs() < 1e-4 && (spec[2 * o + 1] - want[k].1).abs() < 1e-4, "r2c n={n}");
                }
            }
            // and back: C2R of the half spectrum is n times the input
            let s2 = Shape::contiguous(Kind::C2R, &[n], batch);
            let mut bufs2: HashMap<Buf, Vec<f64>> = HashMap::new();
            for b in [Buf::W0, Buf::W1, Buf::A] {
                bufs2.insert(b, vec![f64::NAN; 2 * s2.work_elements()]);
            }
            bufs2.insert(Buf::In, spec);
            bufs2.insert(Buf::Out, vec![f64::NAN; n * batch]);
            emulate(&s2, true, false, &mut bufs2);
            for (i, r) in real.iter().enumerate() {
                assert!((bufs2[&Buf::Out][i] - n as f64 * r).abs() < 1e-3, "c2r n={n} at {i}");
            }
        }
    }

    #[test]
    fn layouts() {
        assert_eq!(Shape::contiguous(Kind::R2C, &[10], 2).odist, 6);
        assert_eq!(Shape::contiguous(Kind::C2R, &[10], 2).idist, 6);
        assert_eq!(Shape::contiguous(Kind::R2C, &[4, 6], 1).check(), Err(cufftError_t::NOT_SUPPORTED));
        let bad = Shape { istride: 2, ..Shape::contiguous(Kind::C2C, &[8], 1) };
        assert_eq!(bad.check(), Err(cufftError_t::NOT_SUPPORTED));
        // one radix-2 pass per power of two, ending in Out; an in-place single pass goes through W0
        let st = steps(&Shape::contiguous(Kind::C2C, &[8], 1), false);
        assert_eq!(st.iter().filter(|s| matches!(s, Step::Radix2 { .. })).count(), 3);
        assert!(matches!(st.last(), Some(Step::Radix2 { dst: Buf::Out, .. })));
    }

    #[test]
    fn double_precision_plans() {
        let work = |t: cufftType, dims: &[c_int]| {
            let mut w = 0usize;
            let r = unsafe {
                match dims {
                    [x] => cufftEstimate1d(*x, t, 2, &mut w),
                    [x, y] => cufftEstimate2d(*x, *y, t, &mut w),
                    _ => unreachable!(),
                }
            };
            r.map(|_| w)
        };
        // the same plans as single precision, with 16-byte complex elements
        assert_eq!(work(cufftType::CUFFT_Z2Z, &[64]), work(cufftType::CUFFT_C2C, &[64]).map(|w| 2 * w));
        assert_eq!(work(cufftType::CUFFT_D2Z, &[30]), work(cufftType::CUFFT_R2C, &[30]).map(|w| 2 * w));
        assert_eq!(work(cufftType::CUFFT_Z2D, &[30]), work(cufftType::CUFFT_C2R, &[30]).map(|w| 2 * w));
        assert_eq!(work(cufftType::CUFFT_Z2Z, &[8, 6]), work(cufftType::CUFFT_C2C, &[8, 6]).map(|w| 2 * w));
        assert_eq!(work(cufftType::CUFFT_D2Z, &[8, 6]), Err(cufftError_t::NOT_SUPPORTED), "real transforms are rank 1");
        let t = twiddles_f64(12);
        assert_eq!((t[0], t[1]), (1.0, -0.0));
        assert!((t[6] - 0.0).abs() < 1e-15 && (t[7] + 1.0).abs() < 1e-15, "exp(-2 pi i 3 / 12) = -i");
    }
}
