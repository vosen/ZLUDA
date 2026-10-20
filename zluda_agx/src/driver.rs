//! The implemented CUDA driver functions. All state lives behind one mutex: `agx_runtime` objects are not
//! thread-safe, and CUDA calls from several threads are serialised here.

#![allow(non_snake_case)]

use agx_runtime::cache::{Cache, Key};
use agx_runtime::context::Context;
use agx_runtime::{error, module, Fence, LoadedModule, Queue};
use cuda_types::cuda::*;
use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_uint, c_void, CStr};
use std::io;
use std::num::NonZeroU32;
use std::sync::Mutex;
use std::time::Instant;

struct Stream {
    queue: Queue,
    /// Fences of other streams the next command must wait for (cuStreamWaitEvent).
    waits: Vec<Fence>,
}

#[derive(Clone, Copy)]
struct Event {
    /// The stream (0 for the default one) and its fence when recorded, if it had work.
    fence: Option<(usize, Fence)>,
    recorded: Option<Instant>,
}

struct Driver {
    ctx: Context,
    default_waits: Vec<Fence>,
    modules: HashMap<usize, LoadedModule>,
    functions: HashMap<usize, (usize, String)>,
    streams: HashMap<usize, Stream>,
    events: HashMap<usize, Event>,
    next_handle: usize,
}

// The runtime objects use Rc internally; they are only reached through DRIVER's mutex.
unsafe impl Send for Driver {}

static DRIVER: Mutex<Option<Driver>> = Mutex::new(None);

fn err(code: u32) -> CUerror {
    CUerror(NonZeroU32::new(code).unwrap())
}

fn from_io(e: io::Error) -> CUerror {
    err(error::cuda_error(&e))
}

fn with<T>(f: impl FnOnce(&mut Driver) -> Result<T, CUerror>) -> Result<T, CUerror> {
    let mut g = DRIVER.lock().unwrap_or_else(|p| p.into_inner());
    match g.as_mut() {
        Some(d) => f(d),
        None => Err(CUerror::NOT_INITIALIZED),
    }
}

/// The Asahi render node: `$ZLUDA_AGX_NODE`, or the first render node that answers the Asahi parameter query.
fn open_context() -> io::Result<Context> {
    if let Ok(node) = std::env::var("ZLUDA_AGX_NODE") {
        return Context::new(&node);
    }
    let mut last = io::Error::new(io::ErrorKind::NotFound, "no Asahi render node");
    for n in 128..192 {
        match Context::new(&format!("/dev/dri/renderD{n}")) {
            Ok(c) => return Ok(c),
            Err(e) => last = e,
        }
    }
    Err(last)
}

impl Driver {
    fn handle(&mut self) -> usize {
        self.next_handle += 1;
        self.next_handle
    }

    /// The queue of a stream handle; 0 and the legacy / per-thread handles (1, 2) are the default stream.
    fn queue(&mut self, stream: usize) -> Result<(&mut Queue, &mut Vec<Fence>), CUerror> {
        if stream <= 2 {
            return Ok((&mut self.ctx.stream, &mut self.default_waits));
        }
        let s = self.streams.get_mut(&stream).ok_or(CUerror::INVALID_HANDLE)?;
        Ok((&mut s.queue, &mut s.waits))
    }

    /// Prints the device printf output of every module and reports a failed device assert.
    fn after_sync(&mut self) -> Result<(), CUerror> {
        let mut failed = false;
        for m in self.modules.values_mut() {
            if let Some(out) = m.drain_printf(|a| self.ctx.read_c_string(a)) {
                use std::io::Write;
                let mut so = std::io::stdout().lock();
                let _ = so.write_all(&out.text);
                if out.dropped_bytes > 0 {
                    let _ = writeln!(so, "[zluda_agx: {} bytes of printf output did not fit the buffer]", out.dropped_bytes);
                }
                let _ = so.flush();
            }
            failed |= m.take_assert_failure();
        }
        if failed {
            Err(CUerror::ASSERT)
        } else {
            Ok(())
        }
    }

    fn sync_all(&mut self) -> Result<(), CUerror> {
        self.ctx.stream.synchronize().map_err(from_io)?;
        for s in self.streams.values_mut() {
            s.queue.synchronize().map_err(from_io)?;
        }
        self.after_sync()
    }
}

/// `zoc`, ZLUDA's PTX compiler built with the `agx` feature: `$ZLUDA_AGX_ZOC`, else `zoc` next to this library's
/// executable, else on `PATH`.
fn zoc_path() -> std::path::PathBuf {
    if let Some(p) = std::env::var_os("ZLUDA_AGX_ZOC") {
        return p.into();
    }
    if let Some(dir) = std::env::current_exe().ok().and_then(|e| e.parent().map(|p| p.to_owned())) {
        let p = dir.join("zoc");
        if p.exists() {
            return p;
        }
    }
    "zoc".into()
}

/// Compiles PTX for `cpu` with zoc, through the compilation cache.
pub(crate) fn compile_ptx(ptx: &[u8], cpu: &str) -> Result<(Vec<u8>, String), String> {
    let zoc = zoc_path();
    let stamp = std::fs::metadata(&zoc)
        .and_then(|m| m.modified())
        .map(|t| format!("{t:?}"))
        .unwrap_or_default();
    let compiler = format!("zoc {} {stamp}", zoc.display());
    let key = Key { source: ptx, chip: cpu, compiler: &compiler, options: &[] };
    let compile = || -> Result<(Vec<u8>, String), String> {
        let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
        let src = dir.path().join("module.ptx");
        std::fs::write(&src, ptx).map_err(|e| e.to_string())?;
        let out = std::process::Command::new(&zoc)
            .args(["--target", "agx", "--arch", cpu, "--output-dir"])
            .arg(dir.path())
            .arg(&src)
            .output()
            .map_err(|e| format!("{}: {e}", zoc.display()))?;
        let object = std::fs::read(dir.path().join("module.o"))
            .map_err(|_| format!("zoc failed: {}", String::from_utf8_lossy(&out.stderr)))?;
        let asm = std::fs::read_to_string(dir.path().join("module.asm")).unwrap_or_default();
        Ok((object, asm))
    };
    match Cache::from_env() {
        Some(c) => c.get_or_compile(&key, compile),
        None => compile(),
    }
}

/// The module in an image: an AGX object, or PTX text (NUL-terminated).
pub(crate) fn load_image(image: &[u8], cpu: &str) -> Result<module::Module, CUerror> {
    if image.starts_with(b"\x7fELF") {
        return module::Module::load_object(image).map_err(|_| CUerror::INVALID_IMAGE);
    }
    let (object, asm) = compile_ptx(image, cpu).map_err(|e| {
        eprintln!("zluda_agx: {e}");
        CUerror::INVALID_PTX
    })?;
    module::Module::load(&object, &asm).map_err(|_| CUerror::INVALID_IMAGE)
}

/// The PTX of a CUDA fat binary (a `__fatBinC_Wrapper_t` or a bare fatbin header): of the PTX files it holds, the
/// one for the newest architecture. NVIDIA machine code (SASS) cannot run here.
unsafe fn ptx_in_fatbin(image: *const u8) -> Result<Vec<u8>, CUerror> {
    use dark_api::fatbin::{Fatbin, FatbinSubmodule};
    let mut best: Option<(u32, Vec<u8>)> = None;
    let mut take = |sub: FatbinSubmodule| {
        let mut files = sub.get_files();
        while let Some(Ok(f)) = files.next() {
            if f.header.kind != cuda_types::dark_api::FatbinFileHeader::HEADER_KIND_PTX {
                continue;
            }
            let sm = f.header.sm_version;
            if best.as_ref().is_some_and(|(b, _)| *b >= sm) {
                continue;
            }
            if let Ok(text) = f.get_or_decompress_content(true) {
                best = Some((sm, text.into_owned()));
            }
        }
    };
    let magic = std::ptr::read_unaligned(image as *const u32);
    if magic == u32::from_le_bytes(cuda_types::dark_api::FatbincWrapper::MAGIC) {
        let ptr = image as *const c_void;
        let fatbin = Fatbin::new(&ptr).map_err(|_| CUerror::INVALID_IMAGE)?;
        let mut subs = fatbin.get_submodules().map_err(|_| CUerror::INVALID_IMAGE)?;
        while let Some(s) = subs.next() {
            take(s.map_err(|_| CUerror::INVALID_IMAGE)?);
        }
    } else {
        take(FatbinSubmodule::new(&*(image as *const cuda_types::dark_api::FatbinHeader)));
    }
    let (_, mut ptx) = best.ok_or(CUerror::NO_BINARY_FOR_GPU)?;
    ptx.push(0);
    Ok(ptx)
}

unsafe fn out<T>(p: *mut T, v: T) -> CUresult {
    if p.is_null() {
        return Err(CUerror::INVALID_VALUE);
    }
    *p = v;
    Ok(())
}

pub(crate) unsafe fn cuInit(_flags: c_uint) -> CUresult {
    let mut g = DRIVER.lock().unwrap_or_else(|p| p.into_inner());
    if g.is_none() {
        let ctx = open_context().map_err(|_| CUerror::NO_DEVICE)?;
        *g = Some(Driver {
            ctx,
            default_waits: Vec::new(),
            modules: HashMap::new(),
            functions: HashMap::new(),
            streams: HashMap::new(),
            events: HashMap::new(),
            next_handle: 16,
        });
    }
    Ok(())
}

pub(crate) unsafe fn cuDriverGetVersion(v: *mut c_int) -> CUresult {
    out(v, 12040)
}

pub(crate) unsafe fn cuDeviceGetCount(count: *mut c_int) -> CUresult {
    let n = with(|_| Ok(1)).unwrap_or(0);
    out(count, n)
}

pub(crate) unsafe fn cuDeviceGet(device: *mut CUdevice, ordinal: c_int) -> CUresult {
    with(|_| if ordinal == 0 { Ok(()) } else { Err(CUerror::INVALID_DEVICE) })?;
    out(device, 0)
}

fn check_device(dev: CUdevice) -> Result<(), CUerror> {
    if dev == 0 {
        Ok(())
    } else {
        Err(CUerror::INVALID_DEVICE)
    }
}

pub(crate) unsafe fn cuDeviceGetName(name: *mut c_char, len: c_int, dev: CUdevice) -> CUresult {
    check_device(dev)?;
    let n = with(|d| Ok(d.ctx.props.name.clone()))?;
    if name.is_null() || len <= 0 {
        return Err(CUerror::INVALID_VALUE);
    }
    let bytes = n.as_bytes();
    let k = bytes.len().min(len as usize - 1);
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), name as *mut u8, k);
    *name.add(k) = 0;
    Ok(())
}

pub(crate) unsafe fn cuDeviceTotalMem_v2(bytes: *mut usize, dev: CUdevice) -> CUresult {
    check_device(dev)?;
    let t = with(|d| Ok(d.ctx.props.total_memory as usize))?;
    out(bytes, t)
}

/// CUDA compute capability reported for AGX: what ZLUDA's PTX front end accepts.
const CC: (c_int, c_int) = (8, 6);

pub(crate) unsafe fn cuDeviceComputeCapability(major: *mut c_int, minor: *mut c_int, dev: CUdevice) -> CUresult {
    check_device(dev)?;
    out(major, CC.0)?;
    out(minor, CC.1)
}

pub(crate) unsafe fn cuDeviceGetAttribute(pi: *mut c_int, attrib: CUdevice_attribute, dev: CUdevice) -> CUresult {
    check_device(dev)?;
    use CUdevice_attribute_enum as A;
    let v = with(|d| {
        let p = &d.ctx.props;
        Ok(match attrib {
            A::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK => p.max_threads_per_block as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_X => p.max_block_dim[0] as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Y => p.max_block_dim[1] as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Z => p.max_block_dim[2] as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_X => p.max_grid_dim[0].min(i32::MAX as u32) as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Y => p.max_grid_dim[1] as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Z => p.max_grid_dim[2] as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK
            | A::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK_OPTIN
            | A::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_MULTIPROCESSOR => p.shared_memory_per_block as c_int,
            A::CU_DEVICE_ATTRIBUTE_WARP_SIZE => p.warp_size as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_BLOCK | A::CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_MULTIPROCESSOR => 65536,
            A::CU_DEVICE_ATTRIBUTE_CLOCK_RATE => p.clock_khz as c_int,
            A::CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT => p.cores as c_int,
            // a core runs up to 96 SIMD-groups (G14 occupancy at the smallest register allocation)
            A::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_MULTIPROCESSOR => 96 * 32,
            A::CU_DEVICE_ATTRIBUTE_MAX_BLOCKS_PER_MULTIPROCESSOR => 32,
            A::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR => CC.0,
            A::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR => CC.1,
            A::CU_DEVICE_ATTRIBUTE_INTEGRATED | A::CU_DEVICE_ATTRIBUTE_UNIFIED_ADDRESSING => 1,
            A::CU_DEVICE_ATTRIBUTE_CONCURRENT_KERNELS => 1,
            A::CU_DEVICE_ATTRIBUTE_KERNEL_EXEC_TIMEOUT => 1,
            A::CU_DEVICE_ATTRIBUTE_TOTAL_CONSTANT_MEMORY => 65536,
            A::CU_DEVICE_ATTRIBUTE_TEXTURE_ALIGNMENT => 256,
            _ => 0,
        })
    })?;
    out(pi, v)
}

const CTX: usize = 1;

pub(crate) unsafe fn cuDevicePrimaryCtxRetain(pctx: *mut CUcontext, dev: CUdevice) -> CUresult {
    check_device(dev)?;
    with(|_| Ok(()))?;
    out(pctx, CUcontext(CTX as *mut _))
}

pub(crate) unsafe fn cuDevicePrimaryCtxRelease_v2(dev: CUdevice) -> CUresult {
    check_device(dev)
}

pub(crate) unsafe fn cuCtxCreate_v2(pctx: *mut CUcontext, _flags: c_uint, dev: CUdevice) -> CUresult {
    cuDevicePrimaryCtxRetain(pctx, dev)
}

pub(crate) unsafe fn cuCtxDestroy_v2(ctx: CUcontext) -> CUresult {
    if ctx.0 as usize != CTX {
        return Err(CUerror::INVALID_CONTEXT);
    }
    with(|d| d.sync_all())
}

pub(crate) unsafe fn cuCtxSetCurrent(ctx: CUcontext) -> CUresult {
    if !ctx.0.is_null() && ctx.0 as usize != CTX {
        return Err(CUerror::INVALID_CONTEXT);
    }
    Ok(())
}

pub(crate) unsafe fn cuCtxGetCurrent(pctx: *mut CUcontext) -> CUresult {
    let c = with(|_| Ok(CTX)).unwrap_or(0);
    out(pctx, CUcontext(c as *mut _))
}

pub(crate) unsafe fn cuCtxGetDevice(device: *mut CUdevice) -> CUresult {
    out(device, 0)
}

pub(crate) unsafe fn cuCtxSynchronize() -> CUresult {
    with(|d| d.sync_all())
}

pub(crate) unsafe fn cuMemAlloc_v2(dptr: *mut CUdeviceptr, bytesize: usize) -> CUresult {
    let va = with(|d| d.ctx.mem_alloc(bytesize as u64).map_err(from_io))?;
    out(dptr, CUdeviceptr_v2(va as *mut c_void))
}

pub(crate) unsafe fn cuMemFree_v2(dptr: CUdeviceptr) -> CUresult {
    with(|d| {
        d.sync_all()?;
        d.ctx.mem_free(dptr.0 as u64).map_err(from_io)
    })
}

pub(crate) unsafe fn cuMemGetInfo_v2(free: *mut usize, total: *mut usize) -> CUresult {
    let (f, t) = with(|d| Ok(d.ctx.mem_info()))?;
    out(free, f as usize)?;
    out(total, t as usize)
}

pub(crate) unsafe fn cuMemcpyHtoD_v2(dst: CUdeviceptr, src: *const c_void, n: usize) -> CUresult {
    if n == 0 {
        return Ok(());
    }
    let s = std::slice::from_raw_parts(src as *const u8, n);
    with(|d| {
        d.sync_all()?;
        d.ctx.memcpy_htod(dst.0 as u64, s).map_err(from_io)
    })
}

pub(crate) unsafe fn cuMemcpyDtoH_v2(dst: *mut c_void, src: CUdeviceptr, n: usize) -> CUresult {
    if n == 0 {
        return Ok(());
    }
    let s = std::slice::from_raw_parts_mut(dst as *mut u8, n);
    with(|d| {
        d.sync_all()?;
        d.ctx.memcpy_dtoh(s, src.0 as u64).map_err(from_io)
    })
}

pub(crate) unsafe fn cuMemcpyDtoD_v2(dst: CUdeviceptr, src: CUdeviceptr, n: usize) -> CUresult {
    with(|d| {
        d.sync_all()?;
        d.ctx.memcpy_dtod(dst.0 as u64, src.0 as u64, n as u64).map_err(from_io)
    })
}

pub(crate) unsafe fn cuMemsetD8_v2(dst: CUdeviceptr, uc: u8, n: usize) -> CUresult {
    with(|d| {
        d.sync_all()?;
        d.ctx.memset(dst.0 as u64, &[uc], n as u64).map_err(from_io)
    })
}

pub(crate) unsafe fn cuMemsetD32_v2(dst: CUdeviceptr, ui: c_uint, n: usize) -> CUresult {
    with(|d| {
        d.sync_all()?;
        d.ctx.memset(dst.0 as u64, &ui.to_le_bytes(), n as u64).map_err(from_io)
    })
}

pub(crate) unsafe fn cuModuleLoadData(module: *mut CUmodule, image: *const c_void) -> CUresult {
    if image.is_null() {
        return Err(CUerror::INVALID_VALUE);
    }
    let bytes = image as *const u8;
    let h = with(|d| {
        let cpu = d.ctx.props.llvm_cpu.clone();
        // An ELF object's size is in its header; PTX is NUL-terminated.
        let magic = std::ptr::read_unaligned(bytes as *const u32);
        let unpacked;
        let image: &[u8] = if magic == 0x466243b1 || magic == 0xba55ed50 {
            unpacked = ptx_in_fatbin(bytes)?;
            &unpacked[..unpacked.len() - 1]
        } else if std::slice::from_raw_parts(bytes, 4) == b"\x7fELF" {
            let h = std::slice::from_raw_parts(bytes, 64);
            let shoff = u64::from_le_bytes(h[40..48].try_into().unwrap());
            let (shentsize, shnum) = (u16::from_le_bytes([h[58], h[59]]) as u64, u16::from_le_bytes([h[60], h[61]]) as u64);
            std::slice::from_raw_parts(bytes, (shoff + shentsize * shnum) as usize)
        } else {
            CStr::from_ptr(bytes as *const c_char).to_bytes()
        };
        let m = load_image(image, &cpu)?;
        let loaded = LoadedModule::new(&mut d.ctx.vm, m).map_err(from_io)?;
        let h = d.handle();
        d.modules.insert(h, loaded);
        Ok(h)
    })?;
    out(module, CUmodule(h as *mut _))
}

pub(crate) unsafe fn cuModuleUnload(hmod: CUmodule) -> CUresult {
    with(|d| {
        d.sync_all()?;
        let h = hmod.0 as usize;
        d.modules.remove(&h).ok_or(CUerror::INVALID_HANDLE)?;
        d.functions.retain(|_, (m, _)| *m != h);
        Ok(())
    })
}

pub(crate) unsafe fn cuModuleGetFunction(hfunc: *mut CUfunction, hmod: CUmodule, name: *const c_char) -> CUresult {
    let name = CStr::from_ptr(name).to_string_lossy().into_owned();
    let h = with(|d| {
        let m = d.modules.get(&(hmod.0 as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        if !m.module.kernels.contains_key(&name) {
            return Err(CUerror::NOT_FOUND);
        }
        let h = d.handle();
        d.functions.insert(h, (hmod.0 as usize, name));
        Ok(h)
    })?;
    out(hfunc, CUfunction(h as *mut _))
}

pub(crate) unsafe fn cuModuleGetGlobal_v2(dptr: *mut CUdeviceptr, bytes: *mut usize, hmod: CUmodule, name: *const c_char) -> CUresult {
    let name = CStr::from_ptr(name).to_string_lossy().into_owned();
    let (va, size) = with(|d| {
        let m = d.modules.get(&(hmod.0 as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        let &(off, size) = m.module.global_symbols.get(&name).ok_or(CUerror::NOT_FOUND)?;
        Ok((m.globals_va().ok_or(CUerror::NOT_FOUND)? + off, size))
    })?;
    if !dptr.is_null() {
        *dptr = CUdeviceptr_v2(va as *mut c_void);
    }
    if !bytes.is_null() {
        *bytes = size as usize;
    }
    Ok(())
}

pub(crate) unsafe fn cuFuncGetAttribute(pi: *mut c_int, attrib: CUfunction_attribute, hfunc: CUfunction) -> CUresult {
    use CUfunction_attribute_enum as F;
    let v = with(|d| {
        let (m, name) = d.functions.get(&(hfunc.0 as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        let k = &d.modules[m].module.kernels[name];
        Ok(match attrib {
            F::CU_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK => 1024,
            F::CU_FUNC_ATTRIBUTE_NUM_REGS => k.gprs as c_int,
            F::CU_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES => k.threadgroup_bytes as c_int,
            F::CU_FUNC_ATTRIBUTE_LOCAL_SIZE_BYTES => k.stack_bytes as c_int,
            _ => 0,
        })
    })?;
    out(pi, v)
}

pub(crate) unsafe fn cuLaunchKernel(
    f: CUfunction,
    gx: c_uint,
    gy: c_uint,
    gz: c_uint,
    bx: c_uint,
    by: c_uint,
    bz: c_uint,
    shared: c_uint,
    stream: CUstream,
    params: *mut *mut c_void,
    extra: *mut *mut c_void,
) -> CUresult {
    if !extra.is_null() {
        return Err(CUerror::NOT_SUPPORTED);
    }
    with(|d| {
        let (m, name) = d.functions.get(&(f.0 as usize)).cloned().ok_or(CUerror::INVALID_HANDLE)?;
        let g14x = d.ctx.device.g14x();
        let slots = d.modules[&m].module.kernels[&name].args.clone();
        let args: Vec<&[u8]> = slots
            .iter()
            .enumerate()
            .map(|(i, s)| std::slice::from_raw_parts(*params.add(i) as *const u8, s.bytes as usize))
            .collect();
        let sh = stream.0 as usize;
        let Driver { ctx, default_waits, modules, streams, .. } = d;
        let (queue, waits) = if sh <= 2 {
            (&mut ctx.stream, default_waits)
        } else {
            let s = streams.get_mut(&sh).ok_or(CUerror::INVALID_HANDLE)?;
            (&mut s.queue, &mut s.waits)
        };
        let w = std::mem::take(waits);
        modules[&m]
            .launch_params_async(&mut ctx.vm, queue, g14x, &name, &args, [gx, gy, gz], [bx, by, bz], shared, &w)
            .map(|_| ())
            .map_err(from_io)
    })
}

pub(crate) unsafe fn cuStreamCreate(ph: *mut CUstream, _flags: c_uint) -> CUresult {
    let h = with(|d| {
        let queue = Queue::new(&mut d.ctx.vm).map_err(from_io)?;
        let h = d.handle();
        d.streams.insert(h, Stream { queue, waits: Vec::new() });
        Ok(h)
    })?;
    out(ph, CUstream(h as *mut _))
}

pub(crate) unsafe fn cuStreamDestroy_v2(s: CUstream) -> CUresult {
    with(|d| {
        let mut st = d.streams.remove(&(s.0 as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        st.queue.synchronize().map_err(from_io)
    })
}

pub(crate) unsafe fn cuStreamSynchronize(s: CUstream) -> CUresult {
    with(|d| {
        d.queue(s.0 as usize)?.0.synchronize().map_err(from_io)?;
        d.after_sync()
    })
}

pub(crate) unsafe fn cuStreamWaitEvent(s: CUstream, e: CUevent, _flags: c_uint) -> CUresult {
    with(|d| {
        let ev = *d.events.get(&(e as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        if let Some((_, fence)) = ev.fence {
            d.queue(s.0 as usize)?.1.push(fence);
        }
        Ok(())
    })
}

pub(crate) unsafe fn cuEventCreate(ph: *mut CUevent, _flags: c_uint) -> CUresult {
    let h = with(|d| {
        let h = d.handle();
        d.events.insert(h, Event { fence: None, recorded: None });
        Ok(h)
    })?;
    out(ph, h as CUevent)
}

pub(crate) unsafe fn cuEventDestroy_v2(e: CUevent) -> CUresult {
    with(|d| d.events.remove(&(e as usize)).map(|_| ()).ok_or(CUerror::INVALID_HANDLE))
}

pub(crate) unsafe fn cuEventRecord(e: CUevent, s: CUstream) -> CUresult {
    with(|d| {
        let sh = if (s.0 as usize) <= 2 { 0 } else { s.0 as usize };
        let fence = d.queue(sh)?.0.last();
        let ev = d.events.get_mut(&(e as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        ev.fence = (fence.point != 0).then_some((sh, fence));
        ev.recorded = Some(Instant::now());
        Ok(())
    })
}

pub(crate) unsafe fn cuEventQuery(e: CUevent) -> CUresult {
    with(|d| {
        let ev = *d.events.get(&(e as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        let Some((s, fence)) = ev.fence else { return Ok(()) };
        if d.queue(s)?.0.is_done(fence).map_err(from_io)? {
            Ok(())
        } else {
            Err(CUerror::NOT_READY)
        }
    })
}

pub(crate) unsafe fn cuEventSynchronize(e: CUevent) -> CUresult {
    with(|d| {
        let ev = *d.events.get(&(e as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        if let Some((s, fence)) = ev.fence {
            let q = d.queue(s)?.0;
            let timeout = q.timeout;
            q.wait(fence, timeout).map_err(from_io)?;
        }
        d.after_sync()
    })
}

/// Milliseconds between two recorded events: from the firmware timestamps of the commands they follow when both
/// have one, else from the host clock at recording.
pub(crate) unsafe fn cuEventElapsedTime(ms: *mut f32, start: CUevent, end: CUevent) -> CUresult {
    let t = with(|d| {
        let a = *d.events.get(&(start as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        let b = *d.events.get(&(end as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        let freq = d.ctx.device.params.command_timestamp_frequency_hz.max(1) as f64;
        if let (Some((sa, fa)), Some((sb, fb))) = (a.fence, b.fence) {
            let ta = d.queue(sa)?.0.timing(fa);
            let tb = d.queue(sb)?.0.timing(fb);
            if let (Some((_, ea)), Some((_, eb))) = (ta, tb) {
                if ea != 0 && eb != 0 {
                    return Ok(((eb as f64 - ea as f64) / freq * 1e3) as f32);
                }
            }
        }
        match (a.recorded, b.recorded) {
            (Some(x), Some(y)) => Ok(y.saturating_duration_since(x).as_secs_f32() * 1e3),
            _ => Err(CUerror::INVALID_HANDLE),
        }
    })?;
    out(ms, t)
}

fn error_name(code: u32) -> &'static CStr {
    match code {
        0 => c"CUDA_SUCCESS",
        1 => c"CUDA_ERROR_INVALID_VALUE",
        2 => c"CUDA_ERROR_OUT_OF_MEMORY",
        3 => c"CUDA_ERROR_NOT_INITIALIZED",
        100 => c"CUDA_ERROR_NO_DEVICE",
        101 => c"CUDA_ERROR_INVALID_DEVICE",
        200 => c"CUDA_ERROR_INVALID_IMAGE",
        201 => c"CUDA_ERROR_INVALID_CONTEXT",
        218 => c"CUDA_ERROR_INVALID_PTX",
        400 => c"CUDA_ERROR_INVALID_HANDLE",
        500 => c"CUDA_ERROR_NOT_FOUND",
        600 => c"CUDA_ERROR_NOT_READY",
        700 => c"CUDA_ERROR_ILLEGAL_ADDRESS",
        702 => c"CUDA_ERROR_LAUNCH_TIMEOUT",
        710 => c"CUDA_ERROR_ASSERT",
        719 => c"CUDA_ERROR_LAUNCH_FAILED",
        801 => c"CUDA_ERROR_NOT_SUPPORTED",
        _ => c"CUDA_ERROR_UNKNOWN",
    }
}

fn code(e: CUresult) -> u32 {
    match e {
        Ok(()) => 0,
        Err(e) => e.0.get(),
    }
}

pub(crate) unsafe fn cuGetErrorName(e: CUresult, p: *mut *const c_char) -> CUresult {
    out(p, error_name(code(e)).as_ptr())
}

pub(crate) unsafe fn cuGetErrorString(e: CUresult, p: *mut *const c_char) -> CUresult {
    out(p, error_name(code(e)).as_ptr())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Needs zoc built with the agx feature: target/debug/zoc, or ZLUDA_AGX_ZOC.
    #[test]
    fn compiles_ptx_into_a_launchable_module() {
        let zoc = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../target/debug/zoc");
        if std::env::var_os("ZLUDA_AGX_ZOC").is_none() {
            if !zoc.exists() {
                eprintln!("skipped: no zoc");
                return;
            }
            std::env::set_var("ZLUDA_AGX_ZOC", &zoc);
        }
        std::env::set_var("ZLUDA_CACHE_DISABLE", "1");
        let ptx = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../ptx/src/test/_Z9vectorAddPKfS0_Pfi.ptx")).unwrap();
        let m = load_image(&ptx, "agx-g14c").unwrap();
        let k = &m.kernels["_Z9vectorAddPKfS0_Pfi"];
        // three pointers and an int, all passed in the uniforms
        let bytes: Vec<u32> = k.args.iter().map(|a| a.bytes).collect();
        assert_eq!(bytes, [8, 8, 8, 4]);
        assert!(k.args.iter().all(|a| !a.by_pointer));
        let (u, block) = module::pack_params(
            k,
            &[&1u64.to_le_bytes(), &2u64.to_le_bytes(), &3u64.to_le_bytes(), &4u32.to_le_bytes()],
            [1, 1, 1],
            None,
            0,
        )
        .unwrap();
        assert!(block.is_empty());
        assert_eq!(&u[0..14], &[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0]);
    }

    // A fat binary as nvcc lays it out: a wrapper, a fatbin header, then file headers with their payloads.
    #[test]
    fn picks_the_newest_ptx_of_a_fat_binary() {
        fn file(kind: u16, sm: u32, payload: &[u8]) -> Vec<u8> {
            let mut h = vec![0u8; 64];
            h[0..2].copy_from_slice(&kind.to_le_bytes());
            h[2..4].copy_from_slice(&0x101u16.to_le_bytes());
            h[4..8].copy_from_slice(&64u32.to_le_bytes());
            h[8..12].copy_from_slice(&(payload.len() as u32).to_le_bytes());
            h[28..32].copy_from_slice(&sm.to_le_bytes());
            h.extend_from_slice(payload);
            h
        }
        let mut files = file(1, 52, b"old\0\0\0\0\0");
        files.extend(file(2, 86, b"SASS....")); // machine code, ignored
        files.extend(file(1, 86, b"new\0\0\0\0\0"));
        let mut bin = vec![0u8; 16];
        bin[0..4].copy_from_slice(&0xba55ed50u32.to_le_bytes());
        bin[4..6].copy_from_slice(&1u16.to_le_bytes());
        bin[6..8].copy_from_slice(&16u16.to_le_bytes());
        bin[8..16].copy_from_slice(&(files.len() as u64).to_le_bytes());
        bin.extend(files);
        // 8-byte aligned storage
        let mut words = vec![0u64; bin.len().div_ceil(8)];
        unsafe { std::ptr::copy_nonoverlapping(bin.as_ptr(), words.as_mut_ptr() as *mut u8, bin.len()) };
        let header = words.as_ptr() as *const u8;
        assert_eq!(unsafe { ptx_in_fatbin(header) }.unwrap(), b"new\0");
        let wrapper = cuda_types::dark_api::FatbincWrapper {
            magic: 0x466243b1,
            version: 1,
            data: header as *const _,
            filename_or_fatbins: std::ptr::null(),
        };
        assert_eq!(unsafe { ptx_in_fatbin(&wrapper as *const _ as *const u8) }.unwrap(), b"new\0");
    }

    #[test]
    fn error_names() {
        assert_eq!(error_name(code(Err(CUerror::ASSERT))).to_str().unwrap(), "CUDA_ERROR_ASSERT");
        assert_eq!(error_name(code(Ok(()))).to_str().unwrap(), "CUDA_SUCCESS");
    }
}
