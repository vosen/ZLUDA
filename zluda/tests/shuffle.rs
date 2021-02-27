use crate::common::CudaDriverFns;
use cuda_types::*;
use rand::{Rng, SeedableRng};
use std::{ffi::c_void, mem, ptr};

mod common;

cuda_driver_test!(shuffle_down);
cuda_driver_test!(shuffle_up);
cuda_driver_test!(shuffle_bfly);
cuda_driver_test!(shuffle_idx);

const KERNEL: &'static str = include_str!("shuffle.ptx");
const WARP_WIDTH: usize = 32;
const TEST_ITERATIONS: usize = 1000;

unsafe fn shuffle_down<T: CudaDriverFns>(cuda: T) {
    shuffle(cuda, "down", validate_down);
}

unsafe fn shuffle_up<T: CudaDriverFns>(cuda: T) {
    shuffle(cuda, "up", validate_up);
}

unsafe fn shuffle_bfly<T: CudaDriverFns>(cuda: T) {
    shuffle(cuda, "bfly", validate_bfly);
}

unsafe fn shuffle_idx<T: CudaDriverFns>(cuda: T) {
    shuffle(cuda, "idx", validate_idx);
}

unsafe fn shuffle<T: CudaDriverFns>(
    cuda: T,
    shuffle_type: &'static str,
    mut validate: impl FnMut(&[u32; WARP_WIDTH], u32, u32, &[u32; WARP_WIDTH]) -> bool,
) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut kernel_text = KERNEL.replace("#SHUFFLE#", shuffle_type);
    kernel_text.push('\0');
    let mut module = mem::zeroed();
    assert_eq!(
        cuda.cuModuleLoadData(&mut module, kernel_text.as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut kernel = mem::zeroed();
    assert_eq!(
        cuda.cuModuleGetFunction(&mut kernel, module, b"shuffle\0".as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut input_mem = mem::zeroed();
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut input_mem, WARP_WIDTH * mem::size_of::<u32>()),
        CUresult::CUDA_SUCCESS
    );
    let mut output_mem = mem::zeroed();
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut output_mem, WARP_WIDTH * mem::size_of::<u32>()),
        CUresult::CUDA_SUCCESS
    );
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0x7cb9cbc7c2b95f47);
    for _ in 00..TEST_ITERATIONS {
        let input = rng.gen::<[u32; WARP_WIDTH]>();
        assert_eq!(
            cuda.cuMemcpyHtoD_v2(
                input_mem,
                input.as_ptr() as _,
                input.len() * mem::size_of::<u32>(),
            ),
            CUresult::CUDA_SUCCESS
        );
        let mut b = rng.gen::<u32>();
        let mut c = rng.gen::<u32>();
        let mut args = [
            &mut input_mem as *mut _ as *mut c_void,
            &mut output_mem as *mut _ as _,
            &mut b as *mut _ as _,
            &mut c as *mut _ as _,
        ];
        assert_eq!(
            cuda.cuLaunchKernel(
                kernel,
                1,
                1,
                1,
                32,
                1,
                1,
                0,
                0 as _,
                args.as_mut_ptr() as _,
                ptr::null_mut(),
            ),
            CUresult::CUDA_SUCCESS
        );
        let output = [0u32; WARP_WIDTH];
        assert_eq!(
            cuda.cuMemcpyDtoH_v2(
                output.as_ptr() as _,
                output_mem,
                output.len() * mem::size_of::<u32>(),
            ),
            CUresult::CUDA_SUCCESS
        );
        assert_eq!(cuda.cuCtxSynchronize(), CUresult::CUDA_SUCCESS);
        assert!(validate(&input, b, c, &output));
    }
}

fn validate_down(input: &[u32; WARP_WIDTH], b: u32, c: u32, result: &[u32; WARP_WIDTH]) -> bool {
    validate(mode_down, input, b, c, result)
}

fn validate_up(input: &[u32; WARP_WIDTH], b: u32, c: u32, result: &[u32; WARP_WIDTH]) -> bool {
    validate(mode_up, input, b, c, result)
}

fn validate_bfly(input: &[u32; WARP_WIDTH], b: u32, c: u32, result: &[u32; WARP_WIDTH]) -> bool {
    validate(mode_bfly, input, b, c, result)
}

fn validate_idx(input: &[u32; WARP_WIDTH], b: u32, c: u32, result: &[u32; WARP_WIDTH]) -> bool {
    validate(mode_idx, input, b, c, result)
}

fn validate(
    mut mode: impl FnMut(u32, i32, u32, u32, u32) -> (i32, bool),
    input: &[u32; WARP_WIDTH],
    b: u32,
    c: u32,
    result: &[u32; WARP_WIDTH],
) -> bool {
    let bval = (b & 31) as i32;
    let cval = c & 31;
    let mask = (c >> 8) & 31;
    let source = (0u32..WARP_WIDTH as u32)
        .into_iter()
        .map(|lane| input[(lane & 31) as usize])
        .collect::<Vec<_>>();
    let max_lane = (0u32..WARP_WIDTH as u32)
        .into_iter()
        .map(|lane| ((lane & 31) & (mask)) | (cval & !mask))
        .collect::<Vec<_>>();
    let min_lane = (0u32..WARP_WIDTH as u32)
        .into_iter()
        .map(|lane| (lane & 31) & (mask))
        .collect::<Vec<_>>();
    let expected = (0u32..WARP_WIDTH as u32)
        .into_iter()
        .zip(max_lane.iter().copied())
        .zip(min_lane.iter().copied())
        .map(|((lane, max_lane), min_lane)| {
            let (mut j, pval) = mode(lane, bval, mask, max_lane, min_lane);
            if !pval {
                j = lane as i32;
            }
            source[j as usize]
        })
        .collect::<Vec<_>>();
    eprintln!("{:?} {} {} {:?} {:?}", &input, b, c, &result, &expected);
    expected == result
}

fn mode_up(lane: u32, bval: i32, _mask: u32, max_lane: u32, _min_lane: u32) -> (i32, bool) {
    let j = (lane as i32) - bval;
    let pval = j >= max_lane as i32;
    (j, pval)
}

fn mode_down(lane: u32, bval: i32, _mask: u32, max_lane: u32, _min_lane: u32) -> (i32, bool) {
    let j = (lane as i32) + bval;
    let pval = j <= max_lane as i32;
    (j, pval)
}

fn mode_bfly(lane: u32, bval: i32, _mask: u32, max_lane: u32, _min_lane: u32) -> (i32, bool) {
    let j = (lane as i32) ^ bval;
    let pval = j <= max_lane as i32;
    (j, pval)
}

fn mode_idx(_lane: u32, bval: i32, mask: u32, max_lane: u32, min_lane: u32) -> (i32, bool) {
    let j = (min_lane as i32) | (bval & !(mask as i32));
    let pval = j <= max_lane as i32;
    (j, pval)
}
