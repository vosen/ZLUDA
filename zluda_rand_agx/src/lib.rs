//! cuRAND's host API on Apple GPUs for zluda_agx (Asahi Linux), on the kernels of agx_libs
//! (`agx_libs/kernels/rand.mlir`, written once for every AGX generation).
//!
//! Every pseudo-random generator type is Philox4x32-10 (Salmon et al., "Parallel random numbers: as easy as 1, 2,
//! 3"): block `b` of a generator's sequence is `philox(counter (b, 0), key seed)`, four 32-bit words. Each call
//! starts at the next whole block after the words the previous calls used, so the values are reproducible for a
//! seed and a sequence of calls, but they are not NVIDIA's sequences (whose generators and orderings are not
//! documented to the bit). The conversions are cuRAND's documented ones: uniform values in (0, 1] (`x 2^-32 +
//! 2^-33`, and 53 bits for doubles), normal values by the Box-Muller transform of pairs of uniforms (so their count
//! must be even), log-normal ones as their exponential, and Poisson values from exponential arrivals (lambda below
//! 64) or the normal approximation. Device generators run the kernels on the generator's stream; host generators
//! (`curandCreateGeneratorHost`) compute the same words on the CPU. Quasi-random generators and the device API's
//! discrete distributions return `CURAND_STATUS_TYPE_ERROR`; the device API itself (`curand_kernel.h`) is header code
//! compiled into the application's PTX and needs nothing from this library.

#![allow(non_camel_case_types, non_snake_case)]

use cuda_types::cuda::{libraryPropertyType_t, CUstream};
use std::ffi::{c_int, c_void};

/// `curandStatus_t`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct curandStatus_t(pub u32);

impl curandStatus_t {
    pub const SUCCESS: curandStatus_t = curandStatus_t(0);
    pub const VERSION_MISMATCH: curandStatus_t = curandStatus_t(100);
    pub const NOT_INITIALIZED: curandStatus_t = curandStatus_t(101);
    pub const ALLOCATION_FAILED: curandStatus_t = curandStatus_t(102);
    pub const TYPE_ERROR: curandStatus_t = curandStatus_t(103);
    pub const OUT_OF_RANGE: curandStatus_t = curandStatus_t(104);
    pub const LENGTH_NOT_MULTIPLE: curandStatus_t = curandStatus_t(105);
    pub const DOUBLE_PRECISION_REQUIRED: curandStatus_t = curandStatus_t(106);
    pub const LAUNCH_FAILURE: curandStatus_t = curandStatus_t(201);
    pub const PREEXISTING_FAILURE: curandStatus_t = curandStatus_t(202);
    pub const INITIALIZATION_FAILED: curandStatus_t = curandStatus_t(203);
    pub const ARCH_MISMATCH: curandStatus_t = curandStatus_t(204);
    pub const INTERNAL_ERROR: curandStatus_t = curandStatus_t(999);
}

/// `curandRngType_t`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct curandRngType_t(pub u32);

impl curandRngType_t {
    pub const TEST: curandRngType_t = curandRngType_t(0);
    pub const PSEUDO_DEFAULT: curandRngType_t = curandRngType_t(100);
    pub const PSEUDO_XORWOW: curandRngType_t = curandRngType_t(101);
    pub const PSEUDO_MRG32K3A: curandRngType_t = curandRngType_t(121);
    pub const PSEUDO_MTGP32: curandRngType_t = curandRngType_t(141);
    pub const PSEUDO_MT19937: curandRngType_t = curandRngType_t(142);
    pub const PSEUDO_PHILOX4_32_10: curandRngType_t = curandRngType_t(161);
    pub const QUASI_DEFAULT: curandRngType_t = curandRngType_t(200);
    pub const QUASI_SOBOL32: curandRngType_t = curandRngType_t(201);
    pub const QUASI_SCRAMBLED_SOBOL32: curandRngType_t = curandRngType_t(202);
    pub const QUASI_SOBOL64: curandRngType_t = curandRngType_t(203);
    pub const QUASI_SCRAMBLED_SOBOL64: curandRngType_t = curandRngType_t(204);
}

/// `curandOrdering_t`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct curandOrdering_t(pub u32);

impl curandOrdering_t {
    pub const PSEUDO_BEST: curandOrdering_t = curandOrdering_t(100);
    pub const PSEUDO_DEFAULT: curandOrdering_t = curandOrdering_t(101);
    pub const PSEUDO_SEEDED: curandOrdering_t = curandOrdering_t(102);
    pub const PSEUDO_LEGACY: curandOrdering_t = curandOrdering_t(103);
    pub const PSEUDO_DYNAMIC: curandOrdering_t = curandOrdering_t(104);
    pub const QUASI_DEFAULT: curandOrdering_t = curandOrdering_t(201);
}

pub type curandGenerator_t = *mut Generator;
pub type curandDiscreteDistribution_t = *mut c_void;

/// The cuRAND version reported (10.3.6, of CUDA 12.5).
pub const VERSION: (c_int, c_int, c_int) = (10, 3, 6);

/// Philox4x32-10 of the counter `c` with the key `k`.
pub fn philox(mut c: [u32; 4], mut k: [u32; 2]) -> [u32; 4] {
    for _ in 0..10 {
        let p0 = 0xD251_1F53u64 * c[0] as u64;
        let p1 = 0xCD9E_8D57u64 * c[2] as u64;
        c = [(p1 >> 32) as u32 ^ c[1] ^ k[0], p1 as u32, (p0 >> 32) as u32 ^ c[3] ^ k[1], p0 as u32];
        k = [k[0].wrapping_add(0x9E37_79B9), k[1].wrapping_add(0xBB67_AE85)];
    }
    c
}

/// Block `b` of the sequence of `seed`.
pub fn block(b: u64, seed: u64) -> [u32; 4] {
    philox([b as u32, (b >> 32) as u32, 0, 0], [seed as u32, (seed >> 32) as u32])
}

/// `x 2^-32 + 2^-33` in (0, 1].
pub fn u01(x: u32) -> f32 {
    (x as f32).mul_add(2.328_306_4e-10, 1.164_153_2e-10)
}

/// `(x ^ (y << 21)) 2^-53 + 2^-54` in (0, 1].
pub fn u01_f64(x: u32, y: u32) -> f64 {
    ((x as u64 ^ ((y as u64) << 21)) as f64).mul_add(f64::powi(2.0, -53), f64::powi(2.0, -54))
}

/// What a generate call produces.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Bits,
    Uniform,
    Normal { mean: f64, stddev: f64 },
    LogNormal { mean: f64, stddev: f64 },
}

impl Kind {
    fn mode(self) -> i64 {
        match self {
            Kind::Bits => 0,
            Kind::Uniform => 1,
            Kind::Normal { .. } => 2,
            Kind::LogNormal { .. } => 3,
        }
    }

    fn params(self) -> (f64, f64) {
        match self {
            Kind::Normal { mean, stddev } | Kind::LogNormal { mean, stddev } => (mean, stddev),
            _ => (0.0, 1.0),
        }
    }
}

/// The words of `block` as `kind` (agx_rand32 on the host).
pub fn values32(w: [u32; 4], kind: Kind) -> [u32; 4] {
    let (mean, sd) = kind.params();
    let (mean, sd) = (mean as f32, sd as f32);
    let pair = |x: u32, y: u32| {
        let u = u01(x);
        let v = (y as f32).mul_add(1.462_918_1e-9, 7.314_590_4e-10);
        let s = (-2.0 * u.ln()).sqrt();
        [(s * v.sin()).mul_add(sd, mean), (s * v.cos()).mul_add(sd, mean)]
    };
    match kind {
        Kind::Bits => w,
        Kind::Uniform => w.map(|x| u01(x).to_bits()),
        Kind::Normal { .. } | Kind::LogNormal { .. } => {
            let (a, b) = (pair(w[0], w[1]), pair(w[2], w[3]));
            let z = [a[0], a[1], b[0], b[1]];
            z.map(|v| if matches!(kind, Kind::LogNormal { .. }) { v.exp() } else { v }.to_bits())
        }
    }
}

/// The two f64 values of a block (agx_rand64 on the host).
pub fn values64(w: [u32; 4], kind: Kind) -> [f64; 2] {
    let (u0, u1) = (u01_f64(w[0], w[1]), u01_f64(w[2], w[3]));
    let (mean, sd) = kind.params();
    match kind {
        Kind::Bits | Kind::Uniform => [u0, u1],
        Kind::Normal { .. } | Kind::LogNormal { .. } => {
            let s = (-2.0 * u0.ln()).sqrt();
            let (sn, cs) = (2.0 * std::f64::consts::PI * u1).sin_cos();
            let z = [(s * sn).mul_add(sd, mean), (s * cs).mul_add(sd, mean)];
            if matches!(kind, Kind::LogNormal { .. }) {
                z.map(f64::exp)
            } else {
                z
            }
        }
    }
}

/// The Poisson value `g` of the sequence of `seed` (agx_rand_poisson on the host).
pub fn poisson(g: u64, seed: u64, lambda: f32) -> u32 {
    let key = [seed as u32, (seed >> 32) as u32];
    if lambda < 64.0 {
        let (mut sum, mut k) = (0.0f32, 0u32);
        for it in 0u32.. {
            for x in philox([g as u32, (g >> 32) as u32, it, 1], key) {
                sum -= u01(x).ln();
                if sum > lambda {
                    return k;
                }
                k += 1;
            }
        }
        unreachable!()
    }
    let w = philox([g as u32, (g >> 32) as u32, 0, 1], key);
    let v = (w[1] as f32).mul_add(1.462_918_1e-9, 7.314_590_4e-10);
    let z = (-2.0 * u01(w[0]).ln()).sqrt() * v.sin();
    (lambda.sqrt().mul_add(z, lambda) + 0.5).floor().max(0.0) as u32
}

/// A generator: its seed and the number of words its sequence has used.
pub struct Generator {
    pub host: bool,
    pub seed: u64,
    pub words: u64,
    pub stream: CUstream,
}

impl Generator {
    /// The first block of a call using `words` words, and moves past them.
    pub fn take(&mut self, words: u64) -> u64 {
        let b = self.words.div_ceil(4);
        self.words = b * 4 + words;
        b
    }
}

fn status(e: agx_libs::engine::Error) -> curandStatus_t {
    match e {
        agx_libs::engine::Error::NotInitialized => curandStatus_t::INITIALIZATION_FAILED,
        agx_libs::engine::Error::ArchMismatch => curandStatus_t::ARCH_MISMATCH,
        agx_libs::engine::Error::Failed => curandStatus_t::LAUNCH_FAILURE,
    }
}

fn result(r: Result<(), curandStatus_t>) -> curandStatus_t {
    r.err().unwrap_or(curandStatus_t::SUCCESS)
}

unsafe fn gen<'a>(g: curandGenerator_t) -> Result<&'a mut Generator, curandStatus_t> {
    g.as_mut().ok_or(curandStatus_t::NOT_INITIALIZED)
}

fn arg<T>(v: &mut T) -> *mut c_void {
    v as *mut T as *mut c_void
}

unsafe fn launch(g: &Generator, name: &'static std::ffi::CStr, params: &mut [*mut c_void], threads: u64) -> Result<(), curandStatus_t> {
    if threads == 0 {
        return Ok(());
    }
    let groups = u32::try_from(threads.div_ceil(64)).map_err(|_| curandStatus_t::OUT_OF_RANGE)?;
    let f = agx_libs::engine::function(agx_libs::RAND, name).map_err(status)?;
    agx_libs::engine::launch_kernel(f, [groups, 1], [64, 1], params, g.stream).map_err(status)
}

/// `n` 32-bit values (words or f32 bits) into `out`.
unsafe fn generate32(g: curandGenerator_t, out: *mut u32, n: usize, kind: Kind) -> Result<(), curandStatus_t> {
    let g = gen(g)?;
    if n > 0 && out.is_null() {
        return Err(curandStatus_t::OUT_OF_RANGE);
    }
    if matches!(kind, Kind::Normal { .. } | Kind::LogNormal { .. }) && n % 2 != 0 {
        return Err(curandStatus_t::LENGTH_NOT_MULTIPLE);
    }
    let start = g.take(n as u64);
    if g.host {
        let out = std::slice::from_raw_parts_mut(out, n);
        for (i, c) in out.chunks_mut(4).enumerate() {
            let v = values32(block(start + i as u64, g.seed), kind);
            c.copy_from_slice(&v[..c.len()]);
        }
        return Ok(());
    }
    let (mean, sd) = kind.params();
    let (mut p, mut n_, mut start, mut seed, mut mode, mut mean, mut sd) = (out as *mut c_void, n as i64, start as i64, g.seed as i64, kind.mode(), mean as f32, sd as f32);
    launch(
        g,
        c"agx_rand32",
        &mut [arg(&mut p), arg(&mut n_), arg(&mut start), arg(&mut seed), arg(&mut mode), arg(&mut mean), arg(&mut sd)],
        (n as u64).div_ceil(4),
    )
}

/// `n` f64 values into `out`.
unsafe fn generate64(g: curandGenerator_t, out: *mut f64, n: usize, kind: Kind) -> Result<(), curandStatus_t> {
    let g = gen(g)?;
    if n > 0 && out.is_null() {
        return Err(curandStatus_t::OUT_OF_RANGE);
    }
    if matches!(kind, Kind::Normal { .. } | Kind::LogNormal { .. }) && n % 2 != 0 {
        return Err(curandStatus_t::LENGTH_NOT_MULTIPLE);
    }
    let start = g.take(2 * n as u64);
    if g.host {
        let out = std::slice::from_raw_parts_mut(out, n);
        for (i, c) in out.chunks_mut(2).enumerate() {
            let v = values64(block(start + i as u64, g.seed), kind);
            c.copy_from_slice(&v[..c.len()]);
        }
        return Ok(());
    }
    let (mean, sd) = kind.params();
    let (mut p, mut n_, mut start, mut seed, mut mode, mut mean, mut sd) = (out as *mut c_void, n as i64, start as i64, g.seed as i64, kind.mode(), mean, sd);
    launch(
        g,
        c"agx_rand64",
        &mut [arg(&mut p), arg(&mut n_), arg(&mut start), arg(&mut seed), arg(&mut mode), arg(&mut mean), arg(&mut sd)],
        (n as u64).div_ceil(2),
    )
}

unsafe fn create(g: *mut curandGenerator_t, ty: curandRngType_t, host: bool) -> Result<(), curandStatus_t> {
    let g = g.as_mut().ok_or(curandStatus_t::NOT_INITIALIZED)?;
    match ty {
        curandRngType_t::PSEUDO_DEFAULT
        | curandRngType_t::PSEUDO_XORWOW
        | curandRngType_t::PSEUDO_MRG32K3A
        | curandRngType_t::PSEUDO_MTGP32
        | curandRngType_t::PSEUDO_MT19937
        | curandRngType_t::PSEUDO_PHILOX4_32_10 => (),
        _ => return Err(curandStatus_t::TYPE_ERROR),
    }
    if !host && !agx_libs::engine::available() {
        return Err(curandStatus_t::INITIALIZATION_FAILED);
    }
    *g = Box::into_raw(Box::new(Generator { host, seed: 0, words: 0, stream: CUstream(std::ptr::null_mut()) }));
    Ok(())
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandCreateGenerator(g: *mut curandGenerator_t, ty: curandRngType_t) -> curandStatus_t {
    result(create(g, ty, false))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandCreateGeneratorHost(g: *mut curandGenerator_t, ty: curandRngType_t) -> curandStatus_t {
    result(create(g, ty, true))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandDestroyGenerator(g: curandGenerator_t) -> curandStatus_t {
    if g.is_null() {
        return curandStatus_t::NOT_INITIALIZED;
    }
    drop(Box::from_raw(g));
    curandStatus_t::SUCCESS
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandGetVersion(v: *mut c_int) -> curandStatus_t {
    match v.as_mut() {
        Some(v) => {
            *v = VERSION.0 * 1000 + VERSION.1 * 100 + VERSION.2;
            curandStatus_t::SUCCESS
        }
        None => curandStatus_t::OUT_OF_RANGE,
    }
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandGetProperty(ty: libraryPropertyType_t, v: *mut c_int) -> curandStatus_t {
    let Some(v) = v.as_mut() else {
        return curandStatus_t::OUT_OF_RANGE;
    };
    *v = match ty {
        libraryPropertyType_t::MAJOR_VERSION => VERSION.0,
        libraryPropertyType_t::MINOR_VERSION => VERSION.1,
        libraryPropertyType_t::PATCH_LEVEL => VERSION.2,
        _ => return curandStatus_t::OUT_OF_RANGE,
    };
    curandStatus_t::SUCCESS
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandSetStream(g: curandGenerator_t, s: CUstream) -> curandStatus_t {
    result(gen(g).map(|g| g.stream = s))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandSetPseudoRandomGeneratorSeed(g: curandGenerator_t, seed: u64) -> curandStatus_t {
    result(gen(g).map(|g| {
        g.seed = seed;
        g.words = 0;
    }))
}

/// The offset counts 32-bit words of the sequence.
#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandSetGeneratorOffset(g: curandGenerator_t, offset: u64) -> curandStatus_t {
    result(gen(g).map(|g| g.words = offset))
}

/// Every ordering gives the same sequence.
#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandSetGeneratorOrdering(g: curandGenerator_t, o: curandOrdering_t) -> curandStatus_t {
    result(gen(g).and_then(|_| match o {
        curandOrdering_t::PSEUDO_BEST
        | curandOrdering_t::PSEUDO_DEFAULT
        | curandOrdering_t::PSEUDO_SEEDED
        | curandOrdering_t::PSEUDO_LEGACY
        | curandOrdering_t::PSEUDO_DYNAMIC => Ok(()),
        _ => Err(curandStatus_t::OUT_OF_RANGE),
    }))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandSetQuasiRandomGeneratorDimensions(g: curandGenerator_t, _dims: u32) -> curandStatus_t {
    result(gen(g).and(Err(curandStatus_t::TYPE_ERROR)))
}

/// Philox needs no seeding pass.
#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandGenerateSeeds(g: curandGenerator_t) -> curandStatus_t {
    result(gen(g).map(|_| ()))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandGenerate(g: curandGenerator_t, out: *mut u32, n: usize) -> curandStatus_t {
    result(generate32(g, out, n, Kind::Bits))
}

/// Only 64-bit quasi-random generators produce 64-bit values.
#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandGenerateLongLong(g: curandGenerator_t, _out: *mut u64, _n: usize) -> curandStatus_t {
    result(gen(g).and(Err(curandStatus_t::TYPE_ERROR)))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandGenerateUniform(g: curandGenerator_t, out: *mut f32, n: usize) -> curandStatus_t {
    result(generate32(g, out.cast(), n, Kind::Uniform))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandGenerateUniformDouble(g: curandGenerator_t, out: *mut f64, n: usize) -> curandStatus_t {
    result(generate64(g, out, n, Kind::Uniform))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandGenerateNormal(g: curandGenerator_t, out: *mut f32, n: usize, mean: f32, stddev: f32) -> curandStatus_t {
    result(generate32(g, out.cast(), n, Kind::Normal { mean: mean as f64, stddev: stddev as f64 }))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandGenerateNormalDouble(g: curandGenerator_t, out: *mut f64, n: usize, mean: f64, stddev: f64) -> curandStatus_t {
    result(generate64(g, out, n, Kind::Normal { mean, stddev }))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandGenerateLogNormal(g: curandGenerator_t, out: *mut f32, n: usize, mean: f32, stddev: f32) -> curandStatus_t {
    result(generate32(g, out.cast(), n, Kind::LogNormal { mean: mean as f64, stddev: stddev as f64 }))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandGenerateLogNormalDouble(g: curandGenerator_t, out: *mut f64, n: usize, mean: f64, stddev: f64) -> curandStatus_t {
    result(generate64(g, out, n, Kind::LogNormal { mean, stddev }))
}

/// Poisson values use their own streams (value `w + j` of the words counter `w`); each value moves the sequence
/// by one word.
#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandGeneratePoisson(g: curandGenerator_t, out: *mut u32, n: usize, lambda: f64) -> curandStatus_t {
    let r = (|| {
        let g = gen(g)?;
        if !(lambda > 0.0) {
            return Err(curandStatus_t::OUT_OF_RANGE);
        }
        if n > 0 && out.is_null() {
            return Err(curandStatus_t::OUT_OF_RANGE);
        }
        let start = g.words;
        g.words += n as u64;
        if g.host {
            for (j, o) in std::slice::from_raw_parts_mut(out, n).iter_mut().enumerate() {
                *o = poisson(start + j as u64, g.seed, lambda as f32);
            }
            return Ok(());
        }
        let (mut p, mut n_, mut start, mut seed, mut lambda) = (out as *mut c_void, n as i64, start as i64, g.seed as i64, lambda as f32);
        launch(g, c"agx_rand_poisson", &mut [arg(&mut p), arg(&mut n_), arg(&mut start), arg(&mut seed), arg(&mut lambda)], n as u64)
    })();
    result(r)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandCreatePoissonDistribution(_lambda: f64, _d: *mut curandDiscreteDistribution_t) -> curandStatus_t {
    curandStatus_t::TYPE_ERROR
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn curandDestroyDistribution(_d: curandDiscreteDistribution_t) -> curandStatus_t {
    curandStatus_t::TYPE_ERROR
}

#[cfg(test)]
mod tests {
    use super::*;

    fn host() -> curandGenerator_t {
        let mut g = std::ptr::null_mut();
        assert_eq!(unsafe { curandCreateGeneratorHost(&mut g, curandRngType_t::PSEUDO_DEFAULT) }, curandStatus_t::SUCCESS);
        g
    }

    #[test]
    fn philox_known_answer() {
        // Random123's kat_vectors for philox4x32_10
        assert_eq!(philox([0; 4], [0; 2]), [0x6627e8d5, 0xe169c58d, 0xbc57ac4c, 0x9b00dbd8]);
        assert_eq!(philox([u32::MAX; 4], [u32::MAX; 2]), [0x408f276d, 0x41c83b0e, 0xa20bc7c6, 0x6d5451fd]);
        assert_eq!(
            philox([0x243f6a88, 0x85a308d3, 0x13198a2e, 0x03707344], [0xa4093822, 0x299f31d0]),
            [0xd16cfe09, 0x94fdcceb, 0x5001e420, 0x24126ea1]
        );
    }

    #[test]
    fn host_sequences_continue_and_reseed() {
        let g = host();
        unsafe {
            curandSetPseudoRandomGeneratorSeed(g, 1234);
            let mut a = [0u32; 6];
            let mut b = [0u32; 3];
            curandGenerate(g, a.as_mut_ptr(), 6);
            curandGenerate(g, b.as_mut_ptr(), 3);
            let w: Vec<u32> = (0..3).flat_map(|i| block(i, 1234)).collect();
            assert_eq!(a, w[..6]);
            // the second call starts at the next block
            assert_eq!(b, w[8..11]);
            curandSetPseudoRandomGeneratorSeed(g, 1234);
            let mut c = [0u32; 6];
            curandGenerate(g, c.as_mut_ptr(), 6);
            assert_eq!(a, c);
            curandDestroyGenerator(g);
        }
    }

    #[test]
    fn host_distributions() {
        let g = host();
        let n = 20000;
        unsafe {
            let mut u = vec![0f32; n];
            curandGenerateUniform(g, u.as_mut_ptr(), n);
            assert!(u.iter().all(|&x| x > 0.0 && x <= 1.0));
            let mean = u.iter().map(|&x| x as f64).sum::<f64>() / n as f64;
            assert!((mean - 0.5).abs() < 0.01, "{mean}");
            let mut z = vec![0f64; n];
            curandGenerateNormalDouble(g, z.as_mut_ptr(), n, 3.0, 2.0);
            let m = z.iter().sum::<f64>() / n as f64;
            let v = z.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / n as f64;
            assert!((m - 3.0).abs() < 0.05 && (v.sqrt() - 2.0).abs() < 0.05, "{m} {v}");
            assert_eq!(curandGenerateNormal(g, z.as_mut_ptr().cast(), 3, 0.0, 1.0), curandStatus_t::LENGTH_NOT_MULTIPLE);
            for lambda in [0.5, 10.0, 200.0] {
                let mut p = vec![0u32; n];
                curandGeneratePoisson(g, p.as_mut_ptr(), n, lambda);
                let m = p.iter().map(|&x| x as f64).sum::<f64>() / n as f64;
                assert!((m - lambda).abs() < 0.03 * lambda.max(1.0), "lambda {lambda}: {m}");
            }
            let mut q = std::ptr::null_mut();
            assert_eq!(curandCreateGeneratorHost(&mut q, curandRngType_t::QUASI_SOBOL32), curandStatus_t::TYPE_ERROR);
            curandDestroyGenerator(g);
        }
    }

    #[test]
    fn version() {
        let mut v = 0;
        unsafe { curandGetVersion(&mut v) };
        assert_eq!(v, 10306);
    }
}
