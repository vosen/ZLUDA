use crate::common::CudaDriverFns;
use cuda_types::*;
use num_traits::{FromPrimitive, Num, WrappingSub};
use rand::{Fill, Rng};
use rand_chacha::rand_core::SeedableRng;
use std::fmt::Debug;
use std::ops::{BitAnd, BitOr, Not, Rem, Shl};
use std::{mem, ptr};

mod common;

static BFI_KERNEL: &'static str = include_str!("bfi.ptx");

cuda_driver_test!(bfi_b32);
unsafe fn bfi_b32<T: CudaDriverFns>(cuda: T) {
    bfi::<_, u32>(cuda, "b32", "4", true)
}

cuda_driver_test!(bfi_b64);
unsafe fn bfi_b64<T: CudaDriverFns>(cuda: T) {
    bfi::<_, u64>(cuda, "b64", "8", false)
}

unsafe fn bfi<
    C: CudaDriverFns,
    T: Copy
        + Default
        + Debug
        + PartialEq
        + Num
        + Shl<Output = T>
        + Not<Output = T>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + Rem<Output = T>
        + WrappingSub<Output = T>
        + FromPrimitive
        + PartialOrd,
>(
    cuda: C,
    type_: &str,
    width: &str,
    limit: bool,
) where
    [T]: Fill,
{
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut kernel = BFI_KERNEL
        .replace("#TYPE#", type_)
        .replace("#WIDTH#", width);
    kernel.push('\0');
    let mut module = ptr::null_mut();
    assert_eq!(
        cuda.cuModuleLoadData(&mut module, kernel.as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut buffer_input = mem::zeroed();
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut buffer_input, mem::size_of::<T>() * 4),
        CUresult::CUDA_SUCCESS
    );
    let mut buffer_output = mem::zeroed();
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut buffer_output, mem::size_of::<T>()),
        CUresult::CUDA_SUCCESS
    );
    let mut result = T::default();
    let mut kernel = mem::zeroed();
    assert_eq!(
        cuda.cuModuleGetFunction(&mut kernel, module, b"kernel_bfi\0".as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0x1905cc2a2c4367e7);
    for i in 0..1024 {
        let mut input = [T::default(); 4];
        rng.fill(&mut input);
        if i == 0 {
            input[2] = T::zero();
            input[3] = T::from_usize(15).unwrap();
        }
        if i == 2 {
            input[2] = T::from_usize(15).unwrap();
            input[3] = T::zero();
        }
        if i % 2 == 1 {
            input[2] = input[2].rem(T::from_usize(32).unwrap());
        }
        assert_eq!(
            cuda.cuMemcpyHtoD_v2(
                buffer_input,
                &mut input as *mut _ as *mut _,
                mem::size_of::<T>() * input.len()
            ),
            CUresult::CUDA_SUCCESS
        );
        let mut params = [&mut buffer_input, &mut buffer_output];
        assert_eq!(
            cuda.cuLaunchKernel(
                kernel,
                1,
                1,
                1,
                1,
                1,
                1,
                0,
                ptr::null_mut(),
                params.as_mut_ptr().cast(),
                ptr::null_mut()
            ),
            CUresult::CUDA_SUCCESS
        );
        assert_eq!(
            cuda.cuMemcpyDtoH_v2(
                &mut result as *mut _ as *mut _,
                buffer_output,
                mem::size_of::<T>()
            ),
            CUresult::CUDA_SUCCESS
        );
        let host_result = bfi_nv(input, limit);
        assert_eq!(result, host_result);
    }
}

fn bfi_nv<
    T: Copy
        + Default
        + Debug
        + PartialEq
        + Num
        + Shl<Output = T>
        + Not<Output = T>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + Rem<Output = T>
        + WrappingSub<Output = T>
        + FromPrimitive
        + PartialOrd,
>(
    input: [T; 4],
    limit: bool,
) -> T {
    let insert = input[0];
    let base = input[1];
    let mut offset = input[2];
    let mut count = input[3];
    if limit {
        offset = offset.rem(T::from_usize(256).unwrap());
        count = count.rem(T::from_usize(256).unwrap());
    }
    let mask = shl_unbound(shl_unbound(T::one(), count).wrapping_sub(&T::one()), offset);
    mask.not()
        .bitand(base)
        .bitor(mask.bitand(shl_unbound(insert, offset)))
}

fn shl_unbound<T>(t: T, amount: T) -> T
where
    T: Num + Shl<Output = T> + FromPrimitive + PartialOrd,
{
    let limit = (mem::size_of::<T>() * 8) - 1;
    if amount > T::from_usize(limit).unwrap() {
        T::zero()
    } else {
        t.shl(amount)
    }
}
