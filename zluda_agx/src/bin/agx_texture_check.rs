//! `agx_texture_check OUT [--float]`: samples a 1024 x 512 RGBA8 array texture (random texels, normalized
//! coordinates, wrap, linear filtering asked for) at 2^20 coordinates, some far outside [0, 1), through the CUDA driver
//! API of zluda_agx, and writes the results to OUT. Without `--float` the texture is read as integers
//! (CU_TRSF_READ_AS_INTEGER, `tex.2d.v4.u32.f32`: point sampled, as Geekbench 7's path tracer reads its environment
//! map), with it as normalized floats (`tex.2d.v4.f32.f32`, filtered). Run it once with ZLUDA_AGX_HARDWARE_TEXTURES=1
//! (the texture unit) and once without (the software sampler) and compare the files.

use cuda_types::cuda::*;
use nvcuda_agx::*;
use std::ffi::c_void;
use std::ptr::null_mut;

fn check(what: &str, r: CUresult) {
    if let Err(e) = r {
        eprintln!("{what} failed: {e:?}");
        std::process::exit(1);
    }
}

fn ptx(float: bool) -> String {
    let (ty, regs) = if float { ("f32", ".f32") } else { ("u32", ".u32") };
    format!(
        ".version 7.0
.target sm_75
.address_size 64
.visible .entry s(.param .u64 tex, .param .u64 pin, .param .u64 pout, .param .u32 n)
{{
    .reg .u32 %i, %n;
    .reg .u64 %t, %a, %b, %o;
    .reg .f32 %x, %y;
    .reg {regs} %r<4>;
    .reg .pred %p;
    mov.u32 %i, %ctaid.x;
    mad.lo.u32 %i, %i, %ntid.x, %tid.x;
    ld.param.u32 %n, [n];
    setp.ge.u32 %p, %i, %n;
    @%p bra DONE;
    ld.param.u64 %t, [tex];
    ld.param.u64 %a, [pin];
    ld.param.u64 %b, [pout];
    cvta.to.global.u64 %a, %a;
    cvta.to.global.u64 %b, %b;
    mul.wide.u32 %o, %i, 8;
    add.u64 %a, %a, %o;
    ld.global.v2.f32 {{%x, %y}}, [%a];
    tex.2d.v4.{ty}.f32 {{%r0, %r1, %r2, %r3}}, [%t, {{%x, %y}}];
    mul.wide.u32 %o, %i, 16;
    add.u64 %b, %b, %o;
    st.global.v4.{ty} [%b], {{%r0, %r1, %r2, %r3}};
DONE:
    ret;
}}
\0"
    )
}

fn main() {
    let out = std::env::args().nth(1).expect("usage: agx_texture_check OUT [--float]");
    let float = std::env::args().any(|a| a == "--float");
    let (w, h, n) = (1024usize, 512usize, 1usize << 20);
    let mut x = 0x2545f4914f6cdd1du64;
    let mut rnd = move || {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x
    };
    unsafe {
        check("cuInit", cuInit(0));
        let mut dev = 0;
        check("cuDeviceGet", cuDeviceGet(&mut dev, 0));
        let mut ctx = CUcontext(null_mut());
        check("cuCtxCreate", cuCtxCreate_v2(&mut ctx, 0, dev));
        let src = ptx(float);
        let mut module = CUmodule(null_mut());
        check("cuModuleLoadData", cuModuleLoadData(&mut module, src.as_ptr() as *const c_void));
        let mut f = CUfunction(null_mut());
        check("cuModuleGetFunction", cuModuleGetFunction(&mut f, module, c"s".as_ptr()));

        // the texels, in a CUDA array
        let texels: Vec<u8> = (0..w * h * 4).map(|_| rnd() as u8).collect();
        let mut ad: CUDA_ARRAY_DESCRIPTOR = std::mem::zeroed();
        ad.Width = w;
        ad.Height = h;
        ad.Format = CUarray_format::CU_AD_FORMAT_UNSIGNED_INT8;
        ad.NumChannels = 4;
        let mut array: CUarray = null_mut();
        check("cuArrayCreate", cuArrayCreate_v2(&mut array, &ad));
        let mut cp: CUDA_MEMCPY2D = std::mem::zeroed();
        cp.srcMemoryType = CUmemorytype::CU_MEMORYTYPE_HOST;
        cp.srcHost = texels.as_ptr() as *const c_void;
        cp.srcPitch = w * 4;
        cp.dstMemoryType = CUmemorytype::CU_MEMORYTYPE_ARRAY;
        cp.dstArray = array;
        cp.WidthInBytes = w * 4;
        cp.Height = h;
        check("cuMemcpy2D", cuMemcpy2D_v2(&cp));
        let mut rd: CUDA_RESOURCE_DESC = std::mem::zeroed();
        rd.resType = CUresourcetype::CU_RESOURCE_TYPE_ARRAY;
        rd.res.array.hArray = array;
        let mut td: CUDA_TEXTURE_DESC = std::mem::zeroed();
        td.addressMode = [CUaddress_mode::CU_TR_ADDRESS_MODE_WRAP; 3];
        td.filterMode = CUfilter_mode::CU_TR_FILTER_MODE_LINEAR;
        td.flags = CU_TRSF_NORMALIZED_COORDINATES | if float { 0 } else { CU_TRSF_READ_AS_INTEGER };
        let mut tex = 0u64;
        check("cuTexObjectCreate", cuTexObjectCreate(&mut tex as *mut u64 as *mut CUtexObject, &rd, &td, null_mut()));

        // coordinates: texel centres, edges, and random values in [-3, 3)
        let coords: Vec<f32> = (0..2 * n)
            .map(|i| match i % 8 {
                0 => ((rnd() % 4096) as f32 + 0.5) / if i % 16 == 0 { w as f32 } else { h as f32 },
                1 => (rnd() % 4097) as f32 / 1024.0,
                _ => (rnd() >> 40) as f32 / (1u64 << 24) as f32 * 6.0 - 3.0,
            })
            .collect();
        let (mut din, mut dout) = (CUdeviceptr_v2(null_mut()), CUdeviceptr_v2(null_mut()));
        check("cuMemAlloc", cuMemAlloc_v2(&mut din, n * 8));
        check("cuMemAlloc", cuMemAlloc_v2(&mut dout, n * 16));
        check("cuMemcpyHtoD", cuMemcpyHtoD_v2(din, coords.as_ptr() as *const c_void, n * 8));
        let nn = n as u32;
        let mut params: [*mut c_void; 4] = [
            &mut tex as *mut _ as *mut c_void,
            &mut din as *mut _ as *mut c_void,
            &mut dout as *mut _ as *mut c_void,
            &nn as *const u32 as *mut c_void,
        ];
        check("cuLaunchKernel", cuLaunchKernel(f, (n / 256) as u32, 1, 1, 256, 1, 1, 0, CUstream(null_mut()), params.as_mut_ptr(), null_mut()));
        let mut result = vec![0u8; n * 16];
        check("cuMemcpyDtoH", cuMemcpyDtoH_v2(result.as_mut_ptr() as *mut c_void, dout, n * 16));
        std::fs::write(&out, &result).expect("write");
        let mut c = Vec::with_capacity(n * 8);
        for v in &coords {
            c.extend_from_slice(&v.to_le_bytes());
        }
        std::fs::write(format!("{out}.coords"), &c).expect("write");
        std::fs::write(format!("{out}.texels"), &texels).expect("write");
        println!("{{\"samples\":{n},\"float\":{float}}}");
    }
}
