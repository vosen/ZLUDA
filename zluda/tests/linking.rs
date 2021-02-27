use common::CudaDriverFns;
use cuda_types::*;
use paste::paste;
use rustc_hash::FxHashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::{mem, os::raw::c_void, ptr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Directive {
    Kernel,
    Method,
    Global,
    Shared,
    Const,
}

impl Directive {
    fn to_str(self, defined: bool) -> &'static str {
        match (self, defined) {
            (Directive::Kernel, false) => ".entry foobar();",
            (Directive::Kernel, true) => ".entry foobar() { ret; }",
            (Directive::Method, false) => ".func foobar();",
            (Directive::Method, true) => ".func foobar() { ret; }",
            (Directive::Global, false) => ".global .b8 foobar[];",
            (Directive::Global, true) => ".global .b8 foobar[1] = {1};",
            (Directive::Shared, false) => ".shared .b8 foobar[];",
            (Directive::Shared, true) => ".shared .b8 foobar[1];",
            (Directive::Const, false) => ".const .b8 foobar[];",
            (Directive::Const, true) => ".const .b8 foobar[1] = {1};",
        }
    }

    fn all() -> [Directive; 5] {
        [
            Directive::Kernel,
            Directive::Method,
            Directive::Global,
            Directive::Shared,
            Directive::Const,
        ]
    }

    unsafe fn try_get<T: CudaDriverFns>(self, cuda: &T, module: CUmodule) -> Option<CUresult> {
        match self {
            Directive::Kernel => {
                let mut unused = ptr::null_mut();
                Some(cuda.cuModuleGetFunction(&mut unused, module, b"foobar\0".as_ptr().cast()))
            }
            Directive::Method | Directive::Shared => None,
            Directive::Global | Directive::Const => {
                let mut unused1: CUdeviceptr_v2 = mem::zeroed();
                let mut unused2 = mem::zeroed();
                Some(cuda.cuModuleGetGlobal_v2(
                    &mut unused1,
                    &mut unused2,
                    module,
                    b"foobar\0".as_ptr().cast(),
                ))
            }
        }
    }

    fn write(self, writer: &mut impl std::fmt::Write, defined: bool, constant: u32) {
        match (self, defined) {
            (Directive::Method, true) => {
                writeln!(
                    writer,
                    ".func (.reg .u32 result) foobar() {{ mov.u32 result, {constant}; ret; }}"
                )
            }
            (Directive::Method, false) => {
                writeln!(writer, ".func (.reg .u32 res) foobar();")
            }
            (Directive::Kernel, true) => {
                writeln!(
                    writer,
                    ".entry foobar(.param .u64 output)
                    {{
                        .reg .u64 out_addr;
                        ld.param.u64 out_addr, [output];
                        st.u32 [out_addr], {constant};
                        ret;
                    }}"
                )
            }
            (Directive::Kernel, false) => {
                writeln!(writer, ".entry foobar(.param .u64 output);")
            }
            (Directive::Global, true) => {
                writeln!(writer, ".global .u32 foobar[1] = {{ {constant} }};")
            }
            (Directive::Global, false) => {
                writeln!(writer, ".global .u32 foobar[];")
            }
            (Directive::Const, true) => {
                writeln!(writer, ".const .u32 foobar[1] = {{ {constant} }};")
            }
            (Directive::Const, false) => {
                writeln!(writer, ".const .u32 foobar[];")
            }
            (Directive::Shared, _) => unimplemented!(),
        }
        .unwrap()
    }

    fn observer_module(self) -> &'static str {
        match self {
            Directive::Kernel => {
                ".version 6.5
                .target sm_60
                .address_size 64
                \0"
            }
            Directive::Method => {
                ".version 6.5
                .target sm_60
                .address_size 64
                .extern .func (.reg .u32 res) foobar();
                .entry observer(.param .u64 output)
                {
                    .reg .u64 out_addr;
                    ld.param.u64 out_addr, [output];
                    .reg .u32 constant;
                    call (constant), foobar, ();
                    st.u32 [out_addr], constant;
                    ret;
                }\0"
            }
            Directive::Global => {
                ".version 6.5
                .target sm_60
                .address_size 64
                .extern .global .u32 foobar[];
                .entry observer(.param .u64 output)
                {
                    .reg .u64 out_addr;
                    ld.param.u64 out_addr, [output];
                    .reg .u32 constant;
                    ld.global.u32 constant, [foobar];
                    st.u32 [out_addr], constant;
                    ret;
                }\0"
            }
            Directive::Const => {
                ".version 6.5
                .target sm_60
                .address_size 64
                .extern .const .u32 foobar[];
                .entry observer(.param .u64 output)
                {
                    .reg .u64 out_addr;
                    ld.param.u64 out_addr, [output];
                    .reg .u32 constant;
                    ld.const.u32 constant, [foobar];
                    st.u32 [out_addr], constant;
                    ret;
                }\0"
            }
            Directive::Shared => unimplemented!(),
        }
    }

    fn observer_name(self) -> &'static str {
        match self {
            Directive::Kernel => "foobar\0",
            _ => "observer\0",
        }
    }

    fn compiled_expected(self) -> &'static [((Linking, bool), (Linking, bool), u32)] {
        match self {
            Directive::Method => &[
                ((Linking::None, true), (Linking::Visible, true), 4),
                ((Linking::Visible, true), (Linking::None, true), 3),
                ((Linking::None, true), (Linking::Weak, true), 4),
                ((Linking::Weak, true), (Linking::None, true), 3),
                ((Linking::Extern, false), (Linking::Visible, true), 4),
                ((Linking::Visible, true), (Linking::Extern, false), 3),
                ((Linking::Extern, false), (Linking::Weak, true), 4),
                ((Linking::Weak, true), (Linking::Extern, false), 3),
                ((Linking::Visible, true), (Linking::Weak, true), 3),
                ((Linking::Weak, true), (Linking::Visible, true), 4),
                ((Linking::Weak, true), (Linking::Weak, true), 3),
            ][..],
            Directive::Kernel => &[
                ((Linking::None, true), (Linking::Extern, false), 3),
                ((Linking::Extern, false), (Linking::None, true), 4),
                ((Linking::Extern, false), (Linking::Visible, true), 4),
                ((Linking::Visible, true), (Linking::Extern, false), 3),
                ((Linking::Extern, false), (Linking::Weak, true), 4),
                ((Linking::Weak, true), (Linking::Extern, false), 3),
                ((Linking::Visible, true), (Linking::Weak, true), 3),
                ((Linking::Weak, true), (Linking::Visible, true), 4),
                ((Linking::Weak, true), (Linking::Weak, true), 3),
            ][..],
            Directive::Global => &[
                ((Linking::None, true), (Linking::Visible, true), 4),
                ((Linking::Visible, true), (Linking::None, true), 3),
                ((Linking::None, true), (Linking::Weak, true), 4),
                ((Linking::Weak, true), (Linking::None, true), 3),
                ((Linking::None, true), (Linking::Common, true), 4),
                ((Linking::Common, true), (Linking::None, true), 3),
                ((Linking::Extern, false), (Linking::Visible, true), 4),
                ((Linking::Visible, true), (Linking::Extern, false), 3),
                ((Linking::Extern, false), (Linking::Weak, true), 4),
                ((Linking::Weak, true), (Linking::Extern, false), 3),
                ((Linking::Extern, false), (Linking::Common, true), 4),
                ((Linking::Common, true), (Linking::Extern, false), 3),
                ((Linking::Visible, true), (Linking::Weak, true), 3),
                ((Linking::Weak, true), (Linking::Visible, true), 4),
                ((Linking::Weak, true), (Linking::Weak, true), 3),
                ((Linking::Weak, true), (Linking::Common, true), 4),
                ((Linking::Common, true), (Linking::Weak, true), 3),
            ][..],
            Directive::Const => &[
                ((Linking::None, true), (Linking::Visible, true), 4),
                ((Linking::Visible, true), (Linking::None, true), 3),
                ((Linking::None, true), (Linking::Weak, true), 4),
                ((Linking::Weak, true), (Linking::None, true), 3),
                ((Linking::Extern, false), (Linking::Visible, true), 4),
                ((Linking::Visible, true), (Linking::Extern, false), 3),
                ((Linking::Extern, false), (Linking::Weak, true), 4),
                ((Linking::Weak, true), (Linking::Extern, false), 3),
                ((Linking::Visible, true), (Linking::Weak, true), 3),
                ((Linking::Weak, true), (Linking::Visible, true), 4),
                ((Linking::Weak, true), (Linking::Weak, true), 3),
            ][..],
            Directive::Shared => unimplemented!(),
        }
    }

    fn assert_exact(self) -> bool {
        match self {
            Directive::Kernel => false,
            Directive::Method => true,
            Directive::Global => false,
            Directive::Const => false,
            Directive::Shared => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Linking {
    None,
    Extern,
    Visible,
    Weak,
    Common,
}

impl Linking {
    fn to_str(self) -> &'static str {
        match self {
            Linking::None => "",
            Linking::Extern => ".extern",
            Linking::Visible => ".visible",
            Linking::Weak => ".weak",
            Linking::Common => ".common",
        }
    }

    fn all() -> [Linking; 5] {
        [
            Linking::None,
            Linking::Extern,
            Linking::Visible,
            Linking::Weak,
            Linking::Common,
        ]
    }
}

mod common;

const KERNEL_PRELUDE: &'static str = "
.version 6.5
.target sm_60
.address_size 64
";

cuda_driver_test!(linking_specifiers_compile);

unsafe fn linking_specifiers_compile<T: CudaDriverFns>(cuda: T) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut results = Vec::new();
    for linking in Linking::all() {
        for directive in Directive::all() {
            for defined in [false, true] {
                let kernel = create_kernel(linking, directive, defined);
                let mut module = ptr::null_mut();
                let error = cuda.cuModuleLoadData(&mut module, kernel.as_ptr().cast());
                let error2 = if error == CUresult::CUDA_SUCCESS {
                    directive.try_get(&cuda, module).map(|x| x.0)
                } else {
                    None
                };
                // we strictly need just return values, other arguments are a debug help
                results.push((linking, directive, defined, error.0, error2));
            }
        }
    }
    let expected = [
        (Linking::None, Directive::Kernel, false, 218, None),
        (Linking::None, Directive::Kernel, true, 0, Some(0)),
        (Linking::None, Directive::Method, false, 218, None),
        (Linking::None, Directive::Method, true, 0, None),
        (Linking::None, Directive::Global, false, 218, None),
        (Linking::None, Directive::Global, true, 0, Some(0)),
        (Linking::None, Directive::Shared, false, 218, None),
        (Linking::None, Directive::Shared, true, 0, None),
        (Linking::None, Directive::Const, false, 218, None),
        (Linking::None, Directive::Const, true, 0, Some(0)),
        (Linking::Extern, Directive::Kernel, false, 0, Some(500)),
        (Linking::Extern, Directive::Kernel, true, 218, None),
        (Linking::Extern, Directive::Method, false, 0, None),
        (Linking::Extern, Directive::Method, true, 218, None),
        (Linking::Extern, Directive::Global, false, 218, None),
        (Linking::Extern, Directive::Global, true, 218, None),
        (Linking::Extern, Directive::Shared, false, 0, None),
        (Linking::Extern, Directive::Shared, true, 0, None),
        (Linking::Extern, Directive::Const, false, 218, None),
        (Linking::Extern, Directive::Const, true, 218, None),
        (Linking::Visible, Directive::Kernel, false, 218, None),
        (Linking::Visible, Directive::Kernel, true, 0, Some(0)),
        (Linking::Visible, Directive::Method, false, 218, None),
        (Linking::Visible, Directive::Method, true, 0, None),
        (Linking::Visible, Directive::Global, false, 218, None),
        (Linking::Visible, Directive::Global, true, 0, Some(0)),
        (Linking::Visible, Directive::Shared, false, 218, None),
        (Linking::Visible, Directive::Shared, true, 0, None),
        (Linking::Visible, Directive::Const, false, 218, None),
        (Linking::Visible, Directive::Const, true, 0, Some(0)),
        (Linking::Weak, Directive::Kernel, false, 218, None),
        (Linking::Weak, Directive::Kernel, true, 0, Some(0)),
        (Linking::Weak, Directive::Method, false, 218, None),
        (Linking::Weak, Directive::Method, true, 0, None),
        (Linking::Weak, Directive::Global, false, 218, None),
        (Linking::Weak, Directive::Global, true, 0, Some(0)),
        (Linking::Weak, Directive::Shared, false, 218, None),
        (Linking::Weak, Directive::Shared, true, 0, None),
        (Linking::Weak, Directive::Const, false, 218, None),
        (Linking::Weak, Directive::Const, true, 0, Some(0)),
        (Linking::Common, Directive::Kernel, false, 218, None),
        (Linking::Common, Directive::Kernel, true, 218, None),
        (Linking::Common, Directive::Method, false, 218, None),
        (Linking::Common, Directive::Method, true, 218, None),
        (Linking::Common, Directive::Global, false, 218, None),
        (Linking::Common, Directive::Global, true, 0, Some(0)),
        (Linking::Common, Directive::Shared, false, 218, None),
        (Linking::Common, Directive::Shared, true, 218, None),
        (Linking::Common, Directive::Const, false, 218, None),
        (Linking::Common, Directive::Const, true, 218, None),
    ];
    assert_eq!(results, expected)
}

fn create_kernel(linking: Linking, directive: Directive, defined: bool) -> String {
    let mut kernel = KERNEL_PRELUDE.to_string();
    kernel.push_str(linking.to_str());
    kernel.push(' ');
    kernel.push_str(directive.to_str(defined));
    kernel.push('\0');
    kernel
}

fn assert_compatible(
    results: Vec<(Linking, Directive, bool, i32, Option<i32>)>,
    expected: [(Linking, Directive, bool, i32, Option<i32>); 50],
) {
    if results.len() != expected.len() {
        panic!();
    }
    let mut broken = Vec::new();
    for (result, expected) in results.into_iter().zip(IntoIterator::into_iter(expected)) {
        let (linking, directive, defined, build_result, load_result) = result;
        let (_, _, _, expected_build, expected_load) = expected;
        if expected_build == 0 {
            if build_result != 0 {
                broken.push((
                    linking,
                    directive,
                    defined,
                    (build_result, load_result),
                    (expected_build, expected_load),
                ));
                continue;
            }
            if expected_load == Some(0) {
                if load_result != Some(0) {
                    broken.push((
                        linking,
                        directive,
                        defined,
                        (build_result, load_result),
                        (expected_build, expected_load),
                    ));
                    continue;
                }
            }
        }
    }
    assert_eq!(broken, []);
}

fn assert_compatible_compile<T: Clone + Hash + Debug + Eq>(
    compiled: &[T],
    compiled_expected: &[T],
) {
    let mut compiled_expected = compiled_expected.iter().cloned().collect::<FxHashSet<_>>();
    for entry in compiled {
        compiled_expected.remove(&entry);
    }
    assert_eq!(compiled_expected, FxHashSet::default());
}

unsafe fn link_and_compile<T: CudaDriverFns>(
    cuda: &T,
    kernels: &[String],
) -> Result<(*mut c_void, usize), CUresult> {
    let mut linker = mem::zeroed();
    assert_eq!(
        cuda.cuLinkCreate_v2(0, ptr::null_mut(), ptr::null_mut(), &mut linker),
        CUresult::CUDA_SUCCESS
    );
    for k in kernels {
        let result = cuda.cuLinkAddData_v2(
            linker,
            CUjitInputType::CU_JIT_INPUT_PTX,
            k.as_ptr().cast_mut().cast(),
            k.len(),
            ptr::null_mut(),
            0,
            ptr::null_mut(),
            ptr::null_mut(),
        );
        if result != CUresult::CUDA_SUCCESS {
            return Err(result);
        }
    }
    let mut binary = mem::zeroed();
    let mut size = 0;
    let result = cuda.cuLinkComplete(linker, &mut binary, &mut size);
    if result != CUresult::CUDA_SUCCESS {
        return Err(result);
    }
    Ok((binary, size))
}

fn all_pairs_ordered<T: Copy + PartialEq>(slice: &[T]) -> Vec<(T, T)> {
    let mut result = Vec::new();
    for i in 0..slice.len() {
        for j in i..slice.len() {
            result.push((slice[i], slice[j]));
            if slice[i] != slice[j] {
                result.push((slice[j], slice[i]));
            }
        }
    }
    result
}

macro_rules! generate_tests2 {
    ([$($directive:expr),+]) => {
        $(
            paste! {
                unsafe fn [<linking_specifiers_link2_ $directive:lower>]<T: CudaDriverFns>(cuda: T) {
                    linking_specifiers_link2::<T>(cuda, Directive:: $directive)
                }
                cuda_driver_test!([<linking_specifiers_link2_ $directive:lower>]);
            }
        )+
    };
}

generate_tests2!([Kernel, Method, Global, Const]);

unsafe fn linking_specifiers_link2<T: CudaDriverFns>(cuda: T, directive: Directive) {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut compiled = Vec::new();
    for (linking_a, linking_b) in all_pairs_ordered(&Linking::all()) {
        for (defined_a, defined_b) in all_pairs_ordered(&[false, true]) {
            if linking_a == Linking::Extern && defined_a
                || linking_b == Linking::Extern && defined_b
                || linking_a != Linking::Extern && !defined_a
                || linking_b != Linking::Extern && !defined_b
            {
                continue;
            }
            let observer = directive.observer_module().to_string();
            let kernel_a = create_kernel2(directive, linking_a, defined_a, 3);
            let kernel_b = create_kernel2(directive, linking_b, defined_b, 4);
            if let Ok((binary, _)) = link_and_compile(&cuda, &[observer, kernel_a, kernel_b][..]) {
                let mut module = mem::zeroed();
                assert_eq!(
                    cuda.cuModuleLoadData(&mut module, binary),
                    CUresult::CUDA_SUCCESS
                );
                let mut function = mem::zeroed();
                if CUresult::CUDA_SUCCESS
                    != cuda.cuModuleGetFunction(
                        &mut function,
                        module,
                        directive.observer_name().as_ptr().cast(),
                    )
                {
                    continue;
                }
                let mut dptr = mem::zeroed();
                assert_eq!(
                    cuda.cuMemAlloc_v2(&mut dptr, mem::size_of::<u32>()),
                    CUresult::CUDA_SUCCESS
                );
                let mut args = [&mut dptr];
                let launch_result = cuda.cuLaunchKernel(
                    function,
                    1,
                    1,
                    1,
                    1,
                    1,
                    1,
                    0,
                    ptr::null_mut(),
                    args.as_mut_ptr().cast(),
                    ptr::null_mut(),
                );
                if launch_result != CUresult::CUDA_SUCCESS {
                    continue;
                }
                let mut result = 0u32;
                assert_eq!(
                    cuda.cuMemcpyDtoH_v2(
                        &mut result as *mut _ as *mut _,
                        dptr,
                        mem::size_of::<u32>()
                    ),
                    CUresult::CUDA_SUCCESS
                );
                compiled.push(((linking_a, defined_a), (linking_b, defined_b), result));
            }
        }
    }
    let compiled_expected = directive.compiled_expected();
    // This is a workaround for NVIDIA bug, see static_kernel_cuda_bug for details
    if !T::is_nvidia() && directive == Directive::Kernel {
        assert_compatible_compile(&compiled, compiled_expected);
    } else {
        assert_eq!(compiled, compiled_expected);
    }
}

fn create_kernel2(directive: Directive, linking: Linking, defined: bool, constant: u32) -> String {
    let mut kernel = KERNEL_PRELUDE.to_string();
    kernel.push_str(linking.to_str());
    kernel.push(' ');
    directive.write(&mut kernel, defined, constant);
    kernel.push('\0');
    kernel
}

cuda_driver_test!(extern_definition_in_non_linking);

unsafe fn extern_definition_in_non_linking<T: CudaDriverFns>(cuda: T) {
    let global_no_init = "
        .version 6.5
        .target sm_60
        .address_size 64
        .extern .global .b32 foobar;\0";
    let global_init = "
        .version 6.5
        .target sm_60
        .address_size 64
        .extern .global .b32 foobar = 0;\0";
    let global_init_incomplete = "
        .version 6.5
        .target sm_60
        .address_size 64
        .extern .global .b32 foobar[];\0";
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut module = ptr::null_mut();
    assert_eq!(
        cuda.cuModuleLoadData(&mut module, global_no_init.as_ptr().cast()),
        CUresult::CUDA_SUCCESS
    );
    assert_ne!(
        cuda.cuModuleLoadData(&mut module, global_init.as_ptr().cast()),
        CUresult::CUDA_SUCCESS
    );
    assert_ne!(
        cuda.cuModuleLoadData(&mut module, global_init_incomplete.as_ptr().cast()),
        CUresult::CUDA_SUCCESS
    );
}

cuda_driver_test!(extern_definition_in_linking);

unsafe fn extern_definition_in_linking<T: CudaDriverFns>(cuda: T) {
    let empty_module = "
        .version 6.5
        .target sm_60
        .address_size 64\0"
        .to_string();
    let global_no_init = "
        .version 6.5
        .target sm_60
        .address_size 64
        .extern .global .b32 foobar;\0"
        .to_string();
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert_ne!(
        link_and_compile(&cuda, &[empty_module, global_no_init]).unwrap_err(),
        CUresult::CUDA_SUCCESS
    );
}

cuda_driver_test!(extern_and_static_illegal);

unsafe fn extern_and_static_illegal<T: CudaDriverFns>(cuda: T) {
    let extern_and_static = "
        .version 6.5
        .target sm_60
        .address_size 64
        .extern .func foobar2();
        .func foobar2() {ret;}\0";
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut module = ptr::null_mut();
    assert_ne!(
        cuda.cuModuleLoadData(&mut module, extern_and_static.as_ptr().cast()),
        CUresult::CUDA_SUCCESS
    );
}

cuda_driver_test!(multiple_common_fail_initializer);

unsafe fn multiple_common_fail_initializer<T: CudaDriverFns>(cuda: T) {
    let common1 = "
        .version 6.5
        .target sm_60
        .address_size 64
         .common .global .u32 foobar = 1;\0"
        .to_string();
    let common2 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .common .global .u32 foobar = 2;\0"
        .to_string();
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert_ne!(
        link_and_compile(&cuda, &[common1, common2]).unwrap_err(),
        CUresult::CUDA_SUCCESS
    );
}

cuda_driver_test!(multiple_common);

unsafe fn multiple_common<T: CudaDriverFns>(cuda: T) {
    let common1 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .common .global .u32 foobar;\0"
        .to_string();
    let common2 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .common .global .u64 foobar = 2;\0"
        .to_string();
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let (binary, _) = link_and_compile(&cuda, &[common1, common2]).unwrap();
    let mut module = mem::zeroed();
    assert_eq!(
        cuda.cuModuleLoadData(&mut module, binary.cast()),
        CUresult::CUDA_SUCCESS
    );
    let mut ptr = mem::zeroed();
    let mut size = mem::zeroed();
    assert_eq!(
        cuda.cuModuleGetGlobal_v2(&mut ptr, &mut size, module, "foobar\0".as_ptr().cast()),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(size, 8);
}

cuda_driver_test!(alignment_and_type_are_ignored_in_globals);

unsafe fn alignment_and_type_are_ignored_in_globals<T: CudaDriverFns>(cuda: T) {
    let common1 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .weak .global .align 8 .u32 foobar;\0"
        .to_string();
    let common2 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .visible .global .align 16 .f32 foobar;\0"
        .to_string();
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let (binary, _) = link_and_compile(&cuda, &[common1, common2]).unwrap();
    let mut module = mem::zeroed();
    assert_eq!(
        cuda.cuModuleLoadData(&mut module, binary.cast()),
        CUresult::CUDA_SUCCESS
    );
    let mut ptr = mem::zeroed();
    let mut size = mem::zeroed();
    assert_eq!(
        cuda.cuModuleGetGlobal_v2(&mut ptr, &mut size, module, "foobar\0".as_ptr().cast()),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(size, 4);
}

cuda_driver_test!(type_check_functions_ignore_align);

unsafe fn type_check_functions_ignore_align<T: CudaDriverFns>(cuda: T) {
    let common1 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .weak .func (.reg .align 8 .u32 x) foobar() { ret; }\0"
        .to_string();
    let common2 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .weak .func (.reg .align 16 .u32 x) foobar() { ret; }\0"
        .to_string();
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert!(link_and_compile(&cuda, &[common1, common2]).is_ok(),);
}

cuda_driver_test!(multiple_static_functions_are_allowed);

unsafe fn multiple_static_functions_are_allowed<T: CudaDriverFns>(cuda: T) {
    let common1 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .func foobar(.param .u32 arg) { ret; }\0"
        .to_string();
    let common2 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .func foobar() { ret; }\0"
        .to_string();
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert!(link_and_compile(&cuda, &[common1, common2]).is_ok());
}

cuda_driver_test!(multiple_static_globals_are_allowed);

unsafe fn multiple_static_globals_are_allowed<T: CudaDriverFns>(cuda: T) {
    let common1 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .global .u64 foobar[1] = {1};\0"
        .to_string();
    let common2 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .global .u32 foobar[1] = {2};\0"
        .to_string();
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let (binary, _) = link_and_compile(&cuda, &[common1, common2]).unwrap();
    let mut module = mem::zeroed();
    assert_eq!(
        cuda.cuModuleLoadData(&mut module, binary.cast()),
        CUresult::CUDA_SUCCESS
    );
    let mut ptr = mem::zeroed();
    let mut size = mem::zeroed();
    assert_eq!(
        cuda.cuModuleGetGlobal_v2(&mut ptr, &mut size, module, "foobar\0".as_ptr().cast()),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(size, 8);
    let mut result = 0u64;
    assert_eq!(
        cuda.cuMemcpyDtoH_v2(&mut result as *mut _ as *mut _, ptr, size),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(result, 1);
}

cuda_driver_test!(local_global_is_not_accessible);

unsafe fn local_global_is_not_accessible<T: CudaDriverFns>(cuda: T) {
    let module_ptx = "
        .version 6.5
        .target sm_60
        .address_size 64
        .entry foo() {
            .global .u32 bar[1] = {2};
            ret;
        }\0"
    .to_string();
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut module = mem::zeroed();
    assert_eq!(
        cuda.cuModuleLoadData(&mut module, module_ptx.as_ptr().cast_mut().cast()),
        CUresult::CUDA_SUCCESS
    );
    let mut ptr = mem::zeroed();
    let mut size = mem::zeroed();
    assert_eq!(
        cuda.cuModuleGetGlobal_v2(&mut ptr, &mut size, module, "bar\0".as_ptr().cast()),
        CUresult::CUDA_ERROR_NOT_FOUND
    );
}

cuda_driver_test!(weak_func);

unsafe fn weak_func<T: CudaDriverFns>(cuda: T) {
    let common1 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .weak .func (.reg .u32 result) foobar() { mov.u32 result, 1; ret; }
        .entry observer1(.param .u64 output)
        {
            .reg .u64 out_addr;
            ld.param.u64 out_addr, [output];
            .reg .u32 constant;
            call (constant), foobar, ();
            st.u32 [out_addr], constant;
            ret;
        }\0"
    .to_string();
    let common2 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .weak .func (.reg .u32 result) foobar() { mov.u32 result, 2; ret; }
        .entry observer2(.param .u64 output)
        {
            .reg .u64 out_addr;
            ld.param.u64 out_addr, [output];
            .reg .u32 constant;
            call (constant), foobar, ();
            st.u32 [out_addr], constant;
            ret;
        }\0"
    .to_string();
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let (binary, _) = link_and_compile(&cuda, &[common1, common2]).unwrap();
    let mut module = mem::zeroed();
    assert_eq!(
        cuda.cuModuleLoadData(&mut module, binary.cast()),
        CUresult::CUDA_SUCCESS
    );
    let mut observer1 = mem::zeroed();
    assert_eq!(
        cuda.cuModuleGetFunction(&mut observer1, module, "observer1\0".as_ptr().cast()),
        CUresult::CUDA_SUCCESS
    );
    let mut observer2 = mem::zeroed();
    assert_eq!(
        cuda.cuModuleGetFunction(&mut observer2, module, "observer2\0".as_ptr().cast()),
        CUresult::CUDA_SUCCESS
    );
    let mut dptr = mem::zeroed();
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut dptr, mem::size_of::<u32>()),
        CUresult::CUDA_SUCCESS
    );
    let mut args = [&mut dptr];
    assert_eq!(
        cuda.cuLaunchKernel(
            observer1,
            1,
            1,
            1,
            1,
            1,
            1,
            0,
            ptr::null_mut(),
            args.as_mut_ptr().cast(),
            ptr::null_mut(),
        ),
        CUresult::CUDA_SUCCESS
    );
    let mut result = 0u32;
    assert_eq!(
        cuda.cuMemcpyDtoH_v2(&mut result as *mut _ as *mut _, dptr, mem::size_of::<u32>()),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(1, result);
    let mut args = [&mut dptr];
    assert_eq!(
        cuda.cuLaunchKernel(
            observer2,
            1,
            1,
            1,
            1,
            1,
            1,
            0,
            ptr::null_mut(),
            args.as_mut_ptr().cast(),
            ptr::null_mut(),
        ),
        CUresult::CUDA_SUCCESS
    );
    let mut result = 0u32;
    assert_eq!(
        cuda.cuMemcpyDtoH_v2(&mut result as *mut _ as *mut _, dptr, mem::size_of::<u32>()),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(1, result);
}

cuda_driver_test!(weak_decl_and_func);

unsafe fn weak_decl_and_func<T: CudaDriverFns>(cuda: T) {
    let common1 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .weak .func foobar();\0"
        .to_string();
    let common2 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .weak .func foobar() { ret; }\0"
        .to_string();
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert_ne!(
        link_and_compile(&cuda, &[common1, common2]).unwrap_err(),
        CUresult::CUDA_SUCCESS
    );
}

// This is a duplicate of a case in mass test `linking_specifiers_link2`
// This is evidently a CUDA bug, so I want to keep it here explicitly
cuda_driver_test!(static_kernel_cuda_bug);

unsafe fn static_kernel_cuda_bug<T: CudaDriverFns>(cuda: T) {
    let input1 = "
        .version 6.5
        .target sm_60
        .address_size 64\0"
        .to_string();
    let input2 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .entry foobar() { ret; }\0"
        .to_string();
    let input3 = "
        .version 6.5
        .target sm_60
        .address_size 64
        .entry foobar() { ret; }\0"
        .to_string();
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let (cubin, _) = link_and_compile(&cuda, &[input1, input2, input3]).unwrap();
    let mut module = mem::zeroed();
    assert_eq!(
        cuda.cuModuleLoadData(&mut module, cubin),
        CUresult::CUDA_SUCCESS
    );
    let mut func = mem::zeroed();
    assert_eq!(
        cuda.cuModuleGetFunction(&mut func, module, b"foobar\0".as_ptr().cast()),
        CUresult::CUDA_SUCCESS
    );
    let mut _unused_arg = 0u64;
    let mut args = [&mut _unused_arg];
    let launch_error = cuda.cuLaunchKernel(
        func,
        1,
        1,
        1,
        1,
        1,
        1,
        0,
        ptr::null_mut(),
        args.as_mut_ptr().cast(),
        ptr::null_mut(),
    );
    if T::is_nvidia() {
        assert_eq!(launch_error, CUresult::CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES);
    } else {
        assert_eq!(launch_error, CUresult::CUDA_SUCCESS);
    }
}

cuda_driver_test!(emit_weak_fn);

unsafe fn emit_weak_fn<T: CudaDriverFns>(cuda: T) {
    let input1 = " 
        .version 6.5
        .target sm_50
        .address_size 64
        
        .weak .func  (.reg .b32 retval) ret0(.reg .b32 input);
        
        .entry observer2(.param .u64 output) {
            .reg .b32 reg32;
            call.uni (reg32), ret0, (reg32);
            ret;
        }
        
        .weak .func  (.reg .b32 retval) ret0(.reg .b32 input)
        {
            mov.b32 retval, 0;
            ret;
        }\0"
    .to_string();
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    assert_eq!(
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    let mut module = mem::zeroed();
    assert_eq!(
        cuda.cuModuleLoadData(&mut module, input1.as_ptr().cast()),
        CUresult::CUDA_SUCCESS
    );
}
