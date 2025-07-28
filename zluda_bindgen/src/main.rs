use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    borrow::Cow, cmp, collections::hash_map, ffi::CString, fs::File, io::Write, iter, mem,
    path::PathBuf, ptr, str::FromStr,
};
use syn::{
    parse_quote, punctuated::Punctuated, visit_mut::VisitMut, Abi, Fields, FieldsUnnamed, FnArg,
    ForeignItem, ForeignItemFn, Ident, Item, ItemConst, ItemForeignMod, ItemUse, LitStr, Path,
    PathArguments, PathSegment, Signature, Type, TypePath, UseTree,
};

// Source: https://developer.nvidia.com/cuda-toolkit-archive
static KNOWN_CUDA_VERSIONS: &[&'static str] = &[
    "12.8.1", "12.8.0", "12.6.3", "12.6.2", "12.6.1", "12.6.0", "12.5.1", "12.5.0", "12.4.1",
    "12.4.0", "12.3.2", "12.3.1", "12.3.0", "12.2.2", "12.2.1", "12.2.0", "12.1.1", "12.1.0",
    "12.0.1", "12.0.0", "11.8.0", "11.7.1", "11.7.0", "11.6.2", "11.6.1", "11.6.0", "11.5.2",
    "11.5.1", "11.5.0", "11.4.4", "11.4.3", "11.4.2", "11.4.1", "11.4.0", "11.3.1", "11.3.0",
    "11.2.2", "11.2.1", "11.2.0", "11.1.1", "11.1.0", "11.0.3", "11.0.2", "11.0.1", "11.0.0",
    "10.2", "10.1", "10.0", "9.2", "9.1", "9.0", "8.0", "7.5", "7.0", "6.5", "6.0", "5.5", "5.0",
    "4.2", "4.1", "4.0", "3.2", "3.1", "3.0", "2.3", "2.2", "2.1", "2.0", "1.1", "1.0",
];

fn main() {
    let crate_root = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    generate_hip_runtime(
        &crate_root,
        &["..", "ext", "hip_runtime-sys", "src", "lib.rs"],
    );
    generate_rocblas(&crate_root, &["..", "ext", "rocblas-sys", "src", "lib.rs"]);
    let cuda_functions = generate_cuda(&crate_root);
    generate_process_address_table(&crate_root, cuda_functions);
    generate_ml(&crate_root);
    generate_cublas(&crate_root);
    generate_cublaslt(&crate_root);
    generate_cufft(&crate_root);
    generate_cusparse(&crate_root);
    generate_cudnn(&crate_root);
}

fn generate_process_address_table(crate_root: &PathBuf, mut cuda_fns: Vec<Ident>) {
    cuda_fns.sort_unstable();
    let mut versions = KNOWN_CUDA_VERSIONS
        .iter()
        .copied()
        .map(cuda_numeric_version)
        .collect::<Vec<_>>();
    versions.sort_unstable();
    let library =
        unsafe { libloading::Library::new("/usr/lib/x86_64-linux-gnu/libcuda.so.1") }.unwrap();
    let cu_get_proc_address = unsafe {
        library.get::<unsafe extern "system" fn(
            symbol: *const ::core::ffi::c_char,
            pfn: *mut *mut ::core::ffi::c_void,
            cudaVersion: ::core::ffi::c_int,
            flags: cuda_types::cuda::cuuint64_t,
            symbolStatus: *mut cuda_types::cuda::CUdriverProcAddressQueryResult,
        ) -> cuda_types::cuda::CUresult>(b"cuGetProcAddress_v2\0")
    }
    .unwrap();
    let mut result = Vec::new();
    for fn_ in cuda_fns {
        let mut known_variants = FxHashMap::default();
        for version in std::iter::successors(Some(1), |x| Some(x + 1)) {
            let map_len = known_variants.len();
            for thread_suffix in ["", "_ptds", "_ptsz"] {
                let version = if version == 1 {
                    "".to_string()
                } else {
                    format!("_v{}", version)
                };
                let fn_ = format!("{}{}{}", fn_, version, thread_suffix);
                match unsafe { library.get::<*mut std::ffi::c_void>(fn_.as_bytes()) } {
                    Ok(symbol) => {
                        known_variants.insert(unsafe { symbol.into_raw() }.as_raw_ptr(), fn_);
                    }
                    Err(_) => {}
                }
            }
            if known_variants.len() == map_len {
                break;
            }
        }
        let fn_ = fn_.to_string();
        let symbol = CString::new(fn_.clone()).unwrap();
        for flag in [
            cuda_types::cuda::CUdriverProcAddress_flags::CU_GET_PROC_ADDRESS_DEFAULT,
            cuda_types::cuda::CUdriverProcAddress_flags::CU_GET_PROC_ADDRESS_LEGACY_STREAM,
            cuda_types::cuda::CUdriverProcAddress_flags::CU_GET_PROC_ADDRESS_PER_THREAD_DEFAULT_STREAM,
            ] {
                let mut breakpoints = Vec::new();
                let mut last_result = None;
                for version in versions.iter().copied() {
                    let mut result = ptr::null_mut();
                    let mut status = unsafe { mem::zeroed() };
                    match unsafe { (cu_get_proc_address)(symbol.as_ptr(), &mut result, version, flag.0 as _, &mut status) } {
                        Ok(()) => {}
                        Err(cuda_types::cuda::CUerror::NOT_FOUND) => {
                            continue;
                        }
                        Err(e) => panic!("{}", e.0)
                    }
                    if status != cuda_types::cuda::CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_SUCCESS {
                        continue;
                    }
                    if Some(result) != last_result {
                        last_result = Some(result);
                        breakpoints.push((version, known_variants.get(&result).unwrap().clone()));
                    }
                }
                breakpoints.sort_unstable_by_key(|(version, _)| cmp::Reverse(*version));
                if !breakpoints.is_empty() {
                    result.push((fn_.clone(), flag.0, breakpoints));
                }
            }
    }
    let mut path = crate_root.clone();
    path.extend(["..", "zluda_bindgen", "src", "process_table.rs"]);
    let mut file = File::create(path).unwrap();
    writeln!(file, "match (name, flag) {{").unwrap();
    for (fn_, version, breakpoints) in result {
        writeln!(file, "    (b\"{fn_}\", {version}) => {{").unwrap();
        for (version, name) in breakpoints {
            writeln!(file, "        if version >= {version} {{").unwrap();
            writeln!(file, "            return {name} as _;").unwrap();
            writeln!(file, "        }}").unwrap();
        }
        writeln!(file, "        usize::MAX as _").unwrap();
        writeln!(file, "    }}").unwrap();
    }
    writeln!(file, "    _ => 0usize as _").unwrap();
    writeln!(file, "}}").unwrap();
}

fn cuda_numeric_version(version: &str) -> i32 {
    let mut version = version.split('.').map(|s| s.parse::<i32>().unwrap());
    let major = version.next().unwrap();
    let minor = version.next().unwrap();
    let patch = version.next().unwrap_or(0);
    major * 1000 + minor * 10 + patch
}

fn generate_cufft(crate_root: &PathBuf) {
    let cufft_header = new_builder()
        .header_contents("cufft_wraper.h", include_str!("../build/cufft_wraper.h"))
        .header("/usr/local/cuda/include/cufftXt.h")
        .allowlist_type("^cufft.*")
        .allowlist_type("^cudaLibXtDesc.*")
        .allowlist_type("^cudaXtDesc.*")
        .allowlist_type("^libFormat.*")
        .allowlist_function("^cufft.*")
        .allowlist_var("^CUFFT_.*")
        .must_use_type("cufftResult_t")
        .allowlist_recursively(false)
        .clang_args(["-I/usr/local/cuda/include"])
        .generate()
        .unwrap()
        .to_string();
    let module: syn::File = syn::parse_str(&cufft_header).unwrap();
    generate_functions(
        &crate_root,
        "cufft",
        &["..", "cuda_macros", "src", "cufft.rs"],
        &module,
    );
    generate_types_library(
        Some(LibraryOverride::CuFft),
        &crate_root,
        &["..", "cuda_types", "src", "cufft.rs"],
        &module,
    );
    generate_display_perflib(
        &crate_root,
        &["..", "format", "src", "format_generated_fft.rs"],
        &["cuda_types", "cufft"],
        &module,
    );
}

fn get_functions(module: syn::File) -> Vec<Ident> {
    module
        .items
        .iter()
        .flat_map(|item| match item {
            Item::ForeignMod(extern_) => {
                extern_
                    .items
                    .iter()
                    .filter_map(|foreign_item| match foreign_item {
                        ForeignItem::Fn(fn_) => Some(fn_.sig.ident.clone()),
                        _ => None,
                    })
            }
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
}

fn generate_cusparse(crate_root: &PathBuf) {
    let cufft_header = new_builder()
        .header("/usr/local/cuda/include/cusparse_v2.h")
        .allowlist_type("^cusparse.*")
        .allowlist_type(".*Info_t$")
        .allowlist_type(".*Info$")
        .blocklist_type("^cudaAsync.*")
        .allowlist_function("^cusparse.*")
        .allowlist_var("^CUSPARSE_.*")
        .must_use_type("cusparseStatus_t")
        .allowlist_recursively(false)
        .clang_args(["-I/usr/local/cuda/include"])
        .generate()
        .unwrap()
        .to_string();
    let module: syn::File = syn::parse_str(&cufft_header).unwrap();
    generate_functions(
        &crate_root,
        "cusparse",
        &["..", "cuda_macros", "src", "cusparse.rs"],
        &module,
    );
    generate_types_library(
        None,
        &crate_root,
        &["..", "cuda_types", "src", "cusparse.rs"],
        &module,
    );
    generate_display_perflib(
        &crate_root,
        &["..", "format", "src", "format_generated_sparse.rs"],
        &["cuda_types", "cusparse"],
        &module,
    );
}

fn generate_cudnn(crate_root: &PathBuf) {
    let cudnn9 = new_builder()
        .header("/usr/include/x86_64-linux-gnu/cudnn_v9.h")
        .allowlist_type("^cudnn.*")
        .allowlist_function("^cudnn.*")
        .allowlist_var("^CUDNN_.*")
        .must_use_type("cudnnStatus_t")
        .allowlist_recursively(false)
        .clang_args(["-I/usr/local/cuda/include"])
        .generate()
        .unwrap()
        .to_string();
    let cudnn9_module: syn::File = syn::parse_str(&cudnn9).unwrap();
    let cudnn9_types = generate_types_library_impl(&cudnn9_module);
    let mut current_dir = PathBuf::from(file!());
    current_dir.pop();
    let cudnn8 = new_builder()
        .header("/usr/include/x86_64-linux-gnu/cudnn_v8.h")
        .allowlist_type("^cudnn.*")
        .allowlist_function("^cudnn.*")
        .allowlist_var("^CUDNN_.*")
        .must_use_type("cudnnStatus_t")
        .allowlist_recursively(false)
        .clang_args([
            "-I/usr/local/cuda/include",
            &format!("-I{}/../build/cudnn_v8", current_dir.display()),
        ])
        .generate()
        .unwrap()
        .to_string();
    let cudnn8_module: syn::File = syn::parse_str(&cudnn8).unwrap();
    let cudnn8_types = generate_types_library_impl(&cudnn8_module);
    merge_types(
        &crate_root,
        &["..", "cuda_types", "src", "cudnn.rs"],
        cudnn9_types,
        &["..", "cuda_types", "src", "cudnn9.rs"],
        cudnn8_types,
        &["..", "cuda_types", "src", "cudnn8.rs"],
    );
    generate_functions(
        &crate_root,
        "cudnn8",
        &["..", "cuda_macros", "src", "cudnn8.rs"],
        &cudnn8_module,
    );
    generate_functions(
        &crate_root,
        "cudnn9",
        &["..", "cuda_macros", "src", "cudnn9.rs"],
        &cudnn9_module,
    );
    generate_display_perflib(
        &crate_root,
        &["..", "format", "src", "format_generated_dnn9.rs"],
        &["cuda_types", "cudnn9"],
        &cudnn9_module,
    );
}

// This code splits types (and constants) into one of:
// - cudnn8-specific
// - cudnn9-specific
// - cudnn shared
// With the rules being:
// - constants go to the version-specific files
// - if there's conflict between types they go to version-specific files
// - if the cudnn9 type is purely additive over cudnn8 then it goes into the
//   shared (and is re-exported by both)
fn merge_types(
    output: &PathBuf,
    cudnn_path: &[&str],
    cudnn9_types: syn::File,
    cudnn9_path: &[&str],
    cudnn8_types: syn::File,
    cudnn8_path: &[&str],
) {
    let cudnn_enums = merge_enums(&cudnn9_types, &cudnn8_types);
    let conflicting_types = get_conflicting_structs(&cudnn9_types, &cudnn8_types, cudnn_enums);
    write_common_cudnn_types(output, cudnn_path, &cudnn9_types, &conflicting_types);
    write_cudnn8_types(output, cudnn8_path, &cudnn8_types, &conflicting_types);
    write_cudnn9_types(output, cudnn9_path, &cudnn9_types, &conflicting_types);
}

fn write_cudnn9_types(
    output: &PathBuf,
    cudnn9_path: &[&str],
    cudnn9_types: &syn::File,
    conflicting_types: &FxHashMap<&Ident, CudnnEnumMergeResult>,
) {
    let items = cudnn9_types.items.iter().filter_map(|item| match item {
        Item::Impl(impl_) => match conflicting_types.get(type_to_ident(&*impl_.self_ty)) {
            Some(CudnnEnumMergeResult::Conflict) | Some(CudnnEnumMergeResult::Cudnn9) | None => {
                Option::<syn::Item>::Some(parse_quote!( #impl_))
            }
            Some(CudnnEnumMergeResult::Same) => None,
        },
        Item::Struct(struct_) => match conflicting_types.get(&struct_.ident) {
            Some(CudnnEnumMergeResult::Conflict) | Some(CudnnEnumMergeResult::Cudnn9) | None => {
                Some(parse_quote!( #struct_))
            }
            Some(CudnnEnumMergeResult::Same) => {
                let type_ = &struct_.ident;
                Some(parse_quote!( pub use super::cudnn:: #type_; ))
            }
        },
        Item::Enum(enum_) => match conflicting_types.get(&enum_.ident) {
            Some(CudnnEnumMergeResult::Conflict) | Some(CudnnEnumMergeResult::Cudnn9) | None => {
                Some(parse_quote!( #enum_))
            }
            Some(CudnnEnumMergeResult::Same) => {
                let type_ = &enum_.ident;
                Some(parse_quote!( pub use super::cudnn:: #type_; ))
            }
        },
        Item::ForeignMod(ItemForeignMod { .. }) => None,
        Item::Const(const_) => Some(parse_quote!(#const_)),
        Item::Union(union_) => match conflicting_types.get(&union_.ident) {
            Some(CudnnEnumMergeResult::Conflict) | Some(CudnnEnumMergeResult::Cudnn9) | None => {
                Some(parse_quote!( #union_))
            }
            Some(CudnnEnumMergeResult::Same) => {
                let type_ = &union_.ident;
                Some(parse_quote!( pub use super::cudnn:: #type_; ))
            }
        },
        Item::Use(use_) => Some(parse_quote!(#use_)),
        Item::Type(type_) => Some(parse_quote!(#type_)),
        _ => unimplemented!(),
    });
    let module: syn::File = parse_quote! {
        #(#items)*
    };
    let mut output = output.clone();
    output.extend(cudnn9_path);
    let text = prettyplease::unparse(&module);
    write_rust_to_file(output, &text)
}

fn write_cudnn8_types(
    output: &PathBuf,
    cudnn8_path: &[&str],
    cudnn8_types: &syn::File,
    conflicting_types: &FxHashMap<&Ident, CudnnEnumMergeResult>,
) {
    let items = cudnn8_types.items.iter().filter_map(|item| match item {
        Item::Impl(impl_) => match conflicting_types.get(type_to_ident(&*impl_.self_ty)) {
            Some(CudnnEnumMergeResult::Conflict) | None => {
                Option::<syn::Item>::Some(parse_quote!( #impl_))
            }
            Some(CudnnEnumMergeResult::Same) => None,
            Some(CudnnEnumMergeResult::Cudnn9) => None,
        },
        Item::Struct(struct_) => match conflicting_types.get(&struct_.ident) {
            Some(CudnnEnumMergeResult::Conflict) | None => Some(parse_quote!( #struct_)),
            Some(CudnnEnumMergeResult::Same) => {
                let type_ = &struct_.ident;
                Some(parse_quote!( pub use super::cudnn:: #type_; ))
            }
            Some(CudnnEnumMergeResult::Cudnn9) => {
                let type_ = &struct_.ident;
                Some(parse_quote!( pub use super::cudnn9:: #type_; ))
            }
        },
        Item::Enum(enum_) => match conflicting_types.get(&enum_.ident) {
            Some(CudnnEnumMergeResult::Conflict) | None => Some(parse_quote!( #enum_)),
            Some(CudnnEnumMergeResult::Same) => {
                let type_ = &enum_.ident;
                Some(parse_quote!( pub use super::cudnn:: #type_; ))
            }
            Some(CudnnEnumMergeResult::Cudnn9) => {
                let type_ = &enum_.ident;
                Some(parse_quote!( pub use super::cudnn9:: #type_; ))
            }
        },
        Item::ForeignMod(ItemForeignMod { .. }) => None,
        Item::Const(const_) => Some(parse_quote!(#const_)),
        Item::Union(union_) => match conflicting_types.get(&union_.ident) {
            Some(CudnnEnumMergeResult::Conflict) | None => Some(parse_quote!( #union_)),
            Some(CudnnEnumMergeResult::Same) => {
                let type_ = &union_.ident;
                Some(parse_quote!( pub use super::cudnn:: #type_; ))
            }
            Some(CudnnEnumMergeResult::Cudnn9) => {
                let type_ = &union_.ident;
                Some(parse_quote!( pub use super::cudnn9:: #type_; ))
            }
        },
        Item::Use(use_) => Some(parse_quote!(#use_)),
        Item::Type(type_) => Some(parse_quote!(#type_)),
        _ => unimplemented!(),
    });
    let module: syn::File = parse_quote! {
        #(#items)*
    };
    let mut output = output.clone();
    output.extend(cudnn8_path);
    let text = prettyplease::unparse(&module);
    write_rust_to_file(output, &text)
}

fn write_common_cudnn_types(
    output: &PathBuf,
    cudnn_path: &[&str],
    cudnn9_types: &syn::File,
    conflicting_types: &FxHashMap<&Ident, CudnnEnumMergeResult>,
) {
    let common_items = cudnn9_types.items.iter().filter_map(|item| match item {
        Item::Impl(ref impl_) => match conflicting_types.get(type_to_ident(&*impl_.self_ty)) {
            Some(CudnnEnumMergeResult::Conflict) => None,
            Some(CudnnEnumMergeResult::Same) => {
                let item: Item = parse_quote! {
                    #impl_
                };
                Some(item)
            }
            Some(CudnnEnumMergeResult::Cudnn9) => None,
            None => None,
        },
        Item::Struct(ref struct_) => match conflicting_types.get(&struct_.ident) {
            Some(CudnnEnumMergeResult::Conflict) => None,
            Some(CudnnEnumMergeResult::Same) => {
                let item: Item = parse_quote! {
                    #struct_
                };
                Some(item)
            }
            Some(CudnnEnumMergeResult::Cudnn9) => None,
            None => None,
        },
        Item::Enum(ref enum_) => match conflicting_types.get(&enum_.ident) {
            Some(CudnnEnumMergeResult::Conflict) => None,
            Some(CudnnEnumMergeResult::Same) => {
                let item: Item = parse_quote! {
                    #enum_
                };
                Some(item)
            }
            Some(CudnnEnumMergeResult::Cudnn9) => None,
            None => None,
        },
        Item::ForeignMod(ItemForeignMod { .. }) => None,
        _ => None,
        //_ => unimplemented!(),
    });
    let cudnn_common: syn::File = parse_quote! {
        #(#common_items)*
    };
    let mut output = output.clone();
    output.extend(cudnn_path);
    let text = prettyplease::unparse(&cudnn_common);
    write_rust_to_file(output, &text)
}

fn get_conflicting_structs<'a>(
    cudnn9_types: &'a syn::File,
    cudnn8_types: &'a syn::File,
    mut enums: FxHashMap<&'a Ident, CudnnEnumMergeResult>,
) -> FxHashMap<&'a Ident, CudnnEnumMergeResult> {
    let structs9 = get_structs(cudnn9_types);
    let structs8 = get_structs(cudnn8_types);
    for (struct_name8, struct8) in structs8 {
        if enums.contains_key(struct_name8) {
            continue;
        }
        match structs9.get(struct_name8) {
            Some(struct9) => {
                if struct8 != *struct9 {
                    panic!("{}", struct_name8.to_string());
                }
                let has_conflicting_field = struct8.iter().any(|field| {
                    let type_ = type_to_ident(&field.ty);
                    enums.get(type_) == Some(&CudnnEnumMergeResult::Conflict)
                });
                let value = if has_conflicting_field {
                    CudnnEnumMergeResult::Conflict
                } else {
                    CudnnEnumMergeResult::Same
                };
                assert!(enums.insert(struct_name8, value).is_none());
            }
            None => {}
        }
    }
    enums
}

fn type_to_ident<'a>(ty: &'a syn::Type) -> &'a syn::Ident {
    match ty {
        Type::Path(path) => &path.path.segments[0].ident,
        Type::Array(array) => type_to_ident(&array.elem),
        _ => unimplemented!("{}", ty.to_token_stream().to_string()),
    }
}

fn merge_enums<'a>(
    cudnn9_types: &'a syn::File,
    cudnn8_types: &'a syn::File,
) -> FxHashMap<&'a Ident, CudnnEnumMergeResult> {
    let result = {
        let enums8 = get_enums(cudnn8_types);
        let enums9 = get_enums(cudnn9_types);
        enums8
            .iter()
            .map(|(enum8_ident, enum8_vars)| {
                let merge_result = match enums9.get(enum8_ident) {
                    Some(enum9_vars) => {
                        let e8_has_extra = enum8_vars.difference(&enum9_vars).any(|_| true);
                        let e9_has_extra = enum9_vars.difference(&enum8_vars).any(|_| true);
                        match (e8_has_extra, e9_has_extra) {
                            (false, false) => CudnnEnumMergeResult::Same,
                            (false, true) => CudnnEnumMergeResult::Cudnn9,
                            (true, true) => CudnnEnumMergeResult::Conflict,
                            (true, false) => unimplemented!(),
                        }
                    }
                    None => {
                        unimplemented!()
                    }
                };
                (*enum8_ident, merge_result)
            })
            .collect::<FxHashMap<_, _>>()
    };
    result
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum CudnnEnumMergeResult {
    // Conflicting definitions
    Conflict,
    // Identical definitions
    Same,
    // Enum present in both, but cudnn9 definition is a strict superset
    Cudnn9,
}

fn get_enums<'a>(
    cudnn_module: &'a syn::File,
) -> FxHashMap<&'a Ident, FxHashSet<&'a syn::ImplItemConst>> {
    let mut enums = FxHashMap::default();
    for item in cudnn_module.items.iter() {
        match item {
            Item::Impl(ref impl_) => match &*impl_.self_ty {
                Type::Path(path) => {
                    let constant = match impl_.items[0] {
                        syn::ImplItem::Const(ref impl_item_const) => impl_item_const,
                        _ => unimplemented!(),
                    };
                    enums
                        .entry(&path.path.segments[0].ident)
                        .or_insert(FxHashSet::default())
                        .insert(constant);
                }
                _ => unimplemented!(),
            },
            _ => {}
        }
    }
    enums
}

fn get_structs<'a>(cudnn_module: &'a syn::File) -> FxHashMap<&'a Ident, Cow<'a, syn::Fields>> {
    let mut structs = FxHashMap::default();
    for item in cudnn_module.items.iter() {
        match item {
            Item::Struct(ref struct_) => {
                assert!(structs
                    .insert(&struct_.ident, Cow::Borrowed(&struct_.fields))
                    .is_none());
            }
            Item::Union(ref union_) => {
                assert!(structs
                    .insert(
                        &union_.ident,
                        Cow::Owned(syn::Fields::Named(union_.fields.clone()))
                    )
                    .is_none());
            }
            _ => {}
        }
    }
    structs
}

fn generate_cublas(crate_root: &PathBuf) {
    let cublas_header = new_builder()
        .header("/usr/local/cuda/include/cublas_v2.h")
        .allowlist_type("^cublas.*")
        .allowlist_function("^cublas.*")
        .allowlist_var("^CUBLAS_.*")
        .must_use_type("cublasStatus_t")
        .allowlist_recursively(false)
        .clang_args(["-I/usr/local/cuda/include", "-x", "c++"])
        .generate()
        .unwrap()
        .to_string();
    let module: syn::File = syn::parse_str(&cublas_header).unwrap();
    generate_functions(
        &crate_root,
        "cublas",
        &["..", "cuda_macros", "src", "cublas.rs"],
        &module,
    );
    generate_types_library(
        None,
        &crate_root,
        &["..", "cuda_types", "src", "cublas.rs"],
        &module,
    );
    generate_display_perflib(
        &crate_root,
        &["..", "format", "src", "format_generated_blas.rs"],
        &["cuda_types", "cublas"],
        &module,
    );
}

fn remove_type(module: &mut syn::File, type_name: &str) {
    let items = std::mem::replace(&mut module.items, Vec::new());
    let items = items
        .into_iter()
        .filter_map(|item| match item {
            Item::Type(type_) if type_.ident == type_name => None,
            Item::Enum(enum_) if enum_.ident == type_name => None,
            Item::Struct(struct_) if struct_.ident == type_name => None,
            Item::Impl(impl_) if impl_.self_ty.to_token_stream().to_string() == type_name => None,
            _ => Some(item),
        })
        .collect();
    module.items = items;
}

fn generate_cublaslt(crate_root: &PathBuf) {
    let cublaslt_header = new_builder()
        .header("/usr/local/cuda/include/cublasLt.h")
        .allowlist_type("^cublas.*")
        .allowlist_function("^cublasLt.*")
        .allowlist_var("^CUBLASLT_.*")
        .must_use_type("cublasStatus_t")
        .allowlist_recursively(false)
        .clang_args(["-I/usr/local/cuda/include", "-x", "c++"])
        .generate()
        .unwrap()
        .to_string();
    let cublaslt_internal_header = new_builder()
        .header_contents(
            "cublasLt_internal.h",
            include_str!("../build/cublasLt_internal.h"),
        )
        .clang_args(["-x", "c++"])
        .override_abi(bindgen::Abi::System, ".*")
        .generate()
        .unwrap()
        .to_string()
        // Simplest and dumbest way to do this
        .replace("pub fn", "fn")
        .replace(");", ") -> ();");
    let module_blaslt_internal: syn::File = syn::parse_str(&cublaslt_internal_header).unwrap();
    std::fs::write(
        crate_root
            .join("..")
            .join("cuda_macros")
            .join("src")
            .join("cublaslt_internal.rs"),
        cublaslt_internal_header,
    )
    .unwrap();
    let mut module_blas: syn::File = syn::parse_str(&cublaslt_header).unwrap();
    remove_type(&mut module_blas, "cublasStatus_t");
    generate_functions(
        &crate_root,
        "cublaslt",
        &["..", "cuda_macros", "src", "cublaslt.rs"],
        &module_blas,
    );
    generate_types_library(
        Some(LibraryOverride::CuBlasLt),
        &crate_root,
        &["..", "cuda_types", "src", "cublaslt.rs"],
        &module_blas,
    );
    generate_display_perflib(
        &crate_root,
        &["..", "format", "src", "format_generated_blaslt.rs"],
        &["cuda_types", "cublaslt"],
        &module_blas,
    );
    generate_display_perflib(
        &crate_root,
        &["..", "format", "src", "format_generated_blaslt_internal.rs"],
        &["cuda_types", "cublaslt"],
        &module_blaslt_internal,
    );
}

fn generate_cuda(crate_root: &PathBuf) -> Vec<Ident> {
    let cuda_header = new_builder()
        .header_contents("cuda_wrapper.h", include_str!("../build/cuda_wrapper.h"))
        .allowlist_type("^CU.*")
        .allowlist_type("^cuda.*")
        .allowlist_type("^cu.*Complex.*")
        .allowlist_type("^libraryPropertyType.*")
        .allowlist_function("^cu.*")
        .allowlist_var("^CU.*")
        .must_use_type("cudaError_enum")
        .constified_enum("cudaError_enum")
        .no_partialeq("CUDA_HOST_NODE_PARAMS_st")
        .new_type_alias(r"^CUdeviceptr_v\d+$")
        .new_type_alias(r"^CUcontext$")
        .new_type_alias(r"^CUstream$")
        .new_type_alias(r"^CUmodule$")
        .new_type_alias(r"^CUfunction$")
        .new_type_alias(r"^CUlibrary$")
        .clang_args(["-I/usr/local/cuda/include"])
        .generate()
        .unwrap()
        .to_string();
    let module: syn::File = syn::parse_str(&cuda_header).unwrap();
    let cuda_functions = get_functions(generate_functions(
        &crate_root,
        "cuda",
        &["..", "cuda_macros", "src", "cuda.rs"],
        &module,
    ));
    generate_types_cuda(
        &crate_root,
        &["..", "cuda_types", "src", "cuda.rs"],
        &module,
    );
    generate_display_cuda(
        &crate_root,
        &["..", "format", "src", "format_generated.rs"],
        &["cuda_types", "cuda"],
        &module,
    );
    cuda_functions
}

fn generate_ml(crate_root: &PathBuf) {
    let ml_header = new_builder()
        .header("/usr/local/cuda/include/nvml.h")
        .allowlist_type("^nvml.*")
        .allowlist_function("^nvml.*")
        .allowlist_var("^NVML.*")
        .must_use_type("nvmlReturn_t")
        .constified_enum("nvmlReturn_enum")
        .clang_args(["-I/usr/local/cuda/include"])
        .generate()
        .unwrap()
        .to_string();
    let mut module: syn::File = syn::parse_str(&ml_header).unwrap();
    let mut converter = ConvertIntoRustResult {
        type_: "nvmlReturn_t",
        underlying_type: "nvmlReturn_enum",
        new_error_type: "nvmlError_t",
        error_prefix: ("NVML_ERROR_", "ERROR_"),
        success: ("NVML_SUCCESS", "SUCCESS"),
        constants: Vec::new(),
    };
    module.items = module
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::Const(const_) => converter.get_const(const_).map(Item::Const),
            Item::Use(use_) => converter.get_use(use_).map(Item::Use),
            Item::Type(type_) => converter.get_type(type_).map(Item::Type),
            item => Some(item),
        })
        .collect::<Vec<_>>();
    converter.flush(&mut module.items);
    generate_functions(
        &crate_root,
        "nvml",
        &["..", "cuda_macros", "src", "nvml.rs"],
        &module,
    );
    generate_types_library(
        None,
        &crate_root,
        &["..", "cuda_types", "src", "nvml.rs"],
        &module,
    );
}

fn generate_types_library(
    override_: Option<LibraryOverride>,
    crate_root: &PathBuf,
    path: &[&str],
    module: &syn::File,
) {
    let module = generate_types_library_impl(module);
    let mut output = crate_root.clone();
    output.extend(path);
    let mut text =
        prettyplease::unparse(&module).replace("self::cudaDataType", "super::cuda::cudaDataType");
    match override_ {
        None => {}
        Some(LibraryOverride::CuBlasLt) => {
            text = text.replace(" cublasStatus_t", " super::cublas::cublasStatus_t");
        }
        Some(LibraryOverride::CuFft) => {
            text = text
                .replace(" cuComplex", " super::cuda::cuComplex")
                .replace(" cuDoubleComplex", " super::cuda::cuDoubleComplex");
        }
    }
    write_rust_to_file(output, &text)
}

#[derive(Clone, Copy)]
enum LibraryOverride {
    CuBlasLt,
    CuFft,
}

fn generate_types_library_impl(module: &syn::File) -> syn::File {
    let known_reexports: Punctuated<syn::Item, syn::parse::Nothing> = parse_quote! {
        pub type __half = u16;
        pub type __nv_bfloat16 = u16;
        pub use super::cuda::cuComplex;
        pub use super::cuda::cuDoubleComplex;
        pub use super::cuda::cudaDataType;
        pub use super::cuda::cudaDataType_t;
        pub type cudaStream_t = super::cuda::CUstream;
        pub use super::cuda::libraryPropertyType;
        pub type cudaGraphExecUpdateResultInfo_st = super::cuda::CUgraphExecUpdateResultInfo_st;
        pub type cudaAsyncNotificationType = super::cuda::CUasyncNotificationType_enum;
        pub type cudaGraph_t = super::cuda::CUgraph;
    };
    let non_fn = module.items.iter().filter_map(|item| match item {
        Item::ForeignMod(_) => None,
        _ => Some(item),
    });
    let items = known_reexports.iter().chain(non_fn);
    parse_quote! {
        #(#items)*
    }
}

fn generate_hip_runtime(output: &PathBuf, path: &[&str]) {
    let hiprt_header = new_builder()
        .header("/opt/rocm/include/hip/hip_runtime_api.h")
        .allowlist_type("^hip.*")
        .allowlist_function("^hip.*")
        .allowlist_var("^hip.*")
        .must_use_type("hipError_t")
        .constified_enum("hipError_t")
        .new_type_alias("^hipDeviceptr_t$")
        .new_type_alias("^hipStream_t$")
        .new_type_alias("^hipModule_t$")
        .new_type_alias("^hipFunction_t$")
        .clang_args(["-I/opt/rocm/include", "-D__HIP_PLATFORM_AMD__"])
        .generate()
        .unwrap()
        .to_string();
    let mut module: syn::File = syn::parse_str(&hiprt_header).unwrap();
    let mut converter = ConvertIntoRustResult {
        type_: "hipError_t",
        underlying_type: "hipError_t",
        new_error_type: "hipErrorCode_t",
        error_prefix: ("hipError", "Error"),
        success: ("hipSuccess", "Success"),
        constants: Vec::new(),
    };
    module.items = module
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::Const(const_) => converter.get_const(const_).map(Item::Const),
            Item::Use(use_) => converter.get_use(use_).map(Item::Use),
            Item::Type(type_) => converter.get_type(type_).map(Item::Type),
            item => Some(item),
        })
        .collect::<Vec<_>>();
    converter.flush(&mut module.items);
    add_send_sync(
        &mut module.items,
        &[
            "hipDeviceptr_t",
            "hipStream_t",
            "hipModule_t",
            "hipFunction_t",
        ],
    );
    let mut output = output.clone();
    output.extend(path);
    write_rust_to_file(output, &prettyplease::unparse(&module))
}

fn generate_rocblas(output: &PathBuf, path: &[&str]) {
    let rocblas_header = new_builder()
        .header("/opt/rocm/include/rocblas/rocblas.h")
        .allowlist_type("^rocblas.*")
        .allowlist_function("^rocblas.*")
        .allowlist_var("^rocblas.*")
        .must_use_type("rocblas_status")
        .constified_enum("rocblas_status_")
        .new_type_alias("^rocblas_handle$")
        .clang_args(["-I/opt/rocm/include", "-D__HIP_PLATFORM_AMD__"])
        .generate()
        .unwrap()
        .to_string();
    let mut module: syn::File = syn::parse_str(&rocblas_header).unwrap();
    remove_type(&mut module, "hipStream_t");
    remove_type(&mut module, "ihipStream_t");
    remove_type(&mut module, "hipEvent_t");
    remove_type(&mut module, "ihipEvent_t");
    let mut converter = ConvertIntoRustResult {
        type_: "rocblas_status",
        underlying_type: "rocblas_status_",
        new_error_type: "rocblas_error",
        error_prefix: ("rocblas_status_", "error_"),
        success: ("rocblas_status_success", "success"),
        constants: Vec::new(),
    };
    module.items = module
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::Const(const_) => converter.get_const(const_).map(Item::Const),
            Item::Use(use_) => converter.get_use(use_).map(Item::Use),
            Item::Type(type_) => converter.get_type(type_).map(Item::Type),
            Item::ForeignMod(mut extern_) => {
                extern_.attrs.push(
                    parse_quote!(#[cfg_attr(windows, link = "rocblas", kind = "raw-dylib")]),
                );
                Some(Item::ForeignMod(extern_))
            }
            item => Some(item),
        })
        .collect::<Vec<_>>();
    converter.flush(&mut module.items);
    add_send_sync(&mut module.items, &["rocblas_handle"]);
    let mut output = output.clone();
    output.extend(path);
    let text = &prettyplease::unparse(&module)
        .replace("hipStream_t", "hip_runtime_sys::hipStream_t")
        .replace("hipEvent_t", "hip_runtime_sys::hipEvent_t");
    write_rust_to_file(output, text)
}

fn add_send_sync(items: &mut Vec<Item>, arg: &[&str]) {
    for type_ in arg {
        let type_ = Ident::new(type_, Span::call_site());
        items.extend([
            parse_quote! {
                unsafe impl Send for #type_ {}
            },
            parse_quote! {
                unsafe impl Sync for #type_ {}
            },
        ]);
    }
}

fn generate_functions(
    output: &PathBuf,
    submodule: &str,
    path: &[&str],
    module: &syn::File,
) -> syn::File {
    let fns_ = module.items.iter().filter_map(|item| match item {
        Item::ForeignMod(extern_) => match &*extern_.items {
            [ForeignItem::Fn(fn_)] => Some(fn_),
            _ => unreachable!(),
        },
        _ => None,
    });
    /*
    let prelude = match submodule {
        "cublaslt" => Some(quote! {
            use cuda_types::cublas::cublasStatus_t;
        }),
        "cublas" => Some(quote! {
            use cuda_types::cublas::cublasStatus_t;
        }),
        _ => None,
    };
    */
    let mut module: syn::File = parse_quote! {
        extern "system" {
            #(#fns_)*
        }
    };
    let submodule = Ident::new(submodule, Span::call_site());
    syn::visit_mut::visit_file_mut(
        &mut PrependCudaPath {
            module: vec![Ident::new("cuda_types", Span::call_site()), submodule],
        },
        &mut module,
    );
    syn::visit_mut::visit_file_mut(&mut RemoveVisibility, &mut module);
    syn::visit_mut::visit_file_mut(&mut ExplicitReturnType, &mut module);
    let mut output = output.clone();
    output.extend(path);
    write_rust_to_file(output, &prettyplease::unparse(&module));
    module
    /*
    module
        .items
        .iter()
        .flat_map(|item| match item {
            Item::ForeignMod(extern_) => {
                extern_
                    .items
                    .iter()
                    .filter_map(|foreign_item| match foreign_item {
                        ForeignItem::Fn(fn_) => Some(fn_.sig.ident.clone()),
                        _ => None,
                    })
            }
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
     */
}

fn generate_types_cuda(output: &PathBuf, path: &[&str], module: &syn::File) {
    let mut module = module.clone();
    let mut converter = ConvertIntoRustResult {
        type_: "CUresult",
        underlying_type: "cudaError_enum",
        new_error_type: "CUerror",
        error_prefix: ("CUDA_ERROR_", "ERROR_"),
        success: ("CUDA_SUCCESS", "SUCCESS"),
        constants: Vec::new(),
    };
    module.items = module
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::ForeignMod(_) => None,
            Item::Const(const_) => converter.get_const(const_).map(Item::Const),
            Item::Use(use_) => converter.get_use(use_).map(Item::Use),
            Item::Type(type_) => converter.get_type(type_).map(Item::Type),
            Item::Struct(mut struct_) => {
                let ident_string = struct_.ident.to_string();
                match &*ident_string {
                    "CUdeviceptr_v2" => {
                        struct_.fields = Fields::Unnamed(parse_quote! {
                            (pub *mut ::core::ffi::c_void)
                        });
                    }
                    "CUuuid_st" => {
                        struct_.fields = Fields::Named(parse_quote! {
                            {pub bytes: [::core::ffi::c_uchar; 16usize]}
                        });
                    }
                    _ => {}
                }
                Some(Item::Struct(struct_))
            }
            item => Some(item),
        })
        .collect::<Vec<_>>();
    converter.flush(&mut module.items);
    module.items.push(parse_quote! {
        impl From<hip_runtime_sys::hipErrorCode_t> for CUerror {
            fn from(error: hip_runtime_sys::hipErrorCode_t) -> Self {
                Self(error.0)
            }
        }
    });
    add_send_sync(
        &mut module.items,
        &[
            "CUdeviceptr",
            "CUcontext",
            "CUstream",
            "CUmodule",
            "CUfunction",
            "CUlibrary",
        ],
    );
    syn::visit_mut::visit_file_mut(&mut FixAbi, &mut module);
    let mut output = output.clone();
    output.extend(path);
    write_rust_to_file(output, &prettyplease::unparse(&module))
}

fn write_rust_to_file(path: impl AsRef<std::path::Path>, content: &str) {
    let mut file = File::create(path).unwrap();
    file.write("// Generated automatically by zluda_bindgen\n// DO NOT EDIT MANUALLY\n#![allow(warnings)]\n".as_bytes())
        .unwrap();
    file.write(content.as_bytes()).unwrap();
}

struct ConvertIntoRustResult {
    type_: &'static str,
    underlying_type: &'static str,
    new_error_type: &'static str,
    error_prefix: (&'static str, &'static str),
    success: (&'static str, &'static str),
    constants: Vec<syn::ItemConst>,
}

impl ConvertIntoRustResult {
    fn get_const(&mut self, const_: syn::ItemConst) -> Option<syn::ItemConst> {
        let name = const_.ident.to_string();
        if name.starts_with(self.underlying_type) {
            self.constants.push(const_);
            None
        } else {
            Some(const_)
        }
    }

    fn get_use(&mut self, use_: ItemUse) -> Option<ItemUse> {
        if let UseTree::Path(ref path) = use_.tree {
            if let UseTree::Rename(ref rename) = &*path.tree {
                if rename.rename == self.type_ {
                    return None;
                }
            }
        }
        Some(use_)
    }

    fn flush(self, items: &mut Vec<Item>) {
        let type_ = format_ident!("{}", self.type_);
        let type_trait = format_ident!("{}Consts", self.type_);
        let new_error_type = format_ident!("{}", self.new_error_type);
        let success = format_ident!("{}", self.success.1);
        let mut result_variants = Vec::new();
        let mut error_variants = Vec::new();
        for const_ in self.constants.iter() {
            let ident = const_.ident.to_string();
            if ident.ends_with(self.success.0) {
                result_variants.push(quote! {
                    const #success: #type_ = #type_::Ok(());
                });
            } else {
                let old_prefix_len = self.underlying_type.len() + 1 + self.error_prefix.0.len();
                let variant_ident =
                    format_ident!("{}{}", self.error_prefix.1, &ident[old_prefix_len..]);
                let error_ident = format_ident!("r#{}", &ident[old_prefix_len..]);
                let expr = &const_.expr;
                result_variants.push(quote! {
                    const #variant_ident: #type_ = #type_::Err(#new_error_type::#error_ident);
                });
                error_variants.push(quote! {
                    pub const #error_ident: #new_error_type = #new_error_type(unsafe { ::core::num::NonZeroU32::new_unchecked(#expr) });
                });
            }
        }
        let extra_items: Punctuated<syn::Item, syn::parse::Nothing> = parse_quote! {
            impl #new_error_type {
                #(#error_variants)*
            }
            #[repr(transparent)]
            #[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
            pub struct #new_error_type(pub ::core::num::NonZeroU32);

            pub trait #type_trait {
                #(#result_variants)*
            }
            impl #type_trait for #type_ {}
            #[must_use]
            pub type #type_ = ::core::result::Result<(), #new_error_type>;
            const _: fn() = || {
                let _ = std::mem::transmute::<#type_, u32>;
            };
        };
        items.extend(extra_items);
    }

    fn get_type(&self, type_: syn::ItemType) -> Option<syn::ItemType> {
        if type_.ident.to_string() == self.type_ {
            None
        } else {
            Some(type_)
        }
    }
}

struct FixAbi;

impl VisitMut for FixAbi {
    fn visit_abi_mut(&mut self, i: &mut Abi) {
        if let Some(ref mut name) = i.name {
            *name = LitStr::new("system", Span::call_site());
        }
    }
}

struct PrependCudaPath {
    module: Vec<Ident>,
}

impl VisitMut for PrependCudaPath {
    fn visit_type_path_mut(&mut self, type_: &mut TypePath) {
        if type_.path.segments.len() == 1 {
            match &*type_.path.segments[0].ident.to_string() {
                "usize" | "u32" | "i32" | "u64" | "i64" | "f64" | "f32" => {}
                "FILE" => {
                    *type_ = parse_quote! { cuda_types :: FILE };
                }
                "cublasStatus_t" => {
                    let module = self.module.iter().rev().skip(1).rev();
                    *type_ = parse_quote! { #(#module :: )* cublas :: #type_ };
                }
                _ => {
                    let module = &self.module;
                    *type_ = parse_quote! { #(#module :: )* #type_ };
                }
            }
        }
    }
}

struct RemoveVisibility;

impl VisitMut for RemoveVisibility {
    fn visit_visibility_mut(&mut self, i: &mut syn::Visibility) {
        *i = syn::Visibility::Inherited;
    }
}

struct ExplicitReturnType;

impl VisitMut for ExplicitReturnType {
    fn visit_return_type_mut(&mut self, i: &mut syn::ReturnType) {
        if let syn::ReturnType::Default = i {
            *i = parse_quote! { -> () };
        }
    }
}

fn generate_display_cuda(
    output: &PathBuf,
    path: &[&str],
    types_crate: &[&'static str],
    module: &syn::File,
) {
    let ignore_types = [
        "CUdevice",
        "CUdeviceptr_v1",
        "CUarrayMapInfo_st",
        "CUDA_RESOURCE_DESC_st",
        "CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st",
        "CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st",
        "CUexecAffinityParam_st",
        "CUstreamBatchMemOpParams_union_CUstreamMemOpWaitValueParams_st",
        "CUstreamBatchMemOpParams_union_CUstreamMemOpWriteValueParams_st",
        "CUuuid_st",
        "HGPUNV",
        "EGLint",
        "EGLSyncKHR",
        "EGLImageKHR",
        "EGLStreamKHR",
        "CUasyncNotificationInfo_st",
        "CUgraphNodeParams_st",
        "CUeglFrame_st",
        "CUdevResource_st",
        "CUlaunchAttribute_st",
        "CUmemcpy3DOperand_st",
        "CUlaunchConfig_st",
    ];
    let ignore_functions = [
        "cuGLGetDevices",
        "cuGLGetDevices_v2",
        "cuStreamSetAttribute",
        "cuStreamSetAttribute_ptsz",
        "cuStreamGetAttribute",
        "cuStreamGetAttribute_ptsz",
        "cuGraphKernelNodeGetAttribute",
        "cuGraphKernelNodeSetAttribute",
    ];
    let count_selectors = [
        ("cuCtxCreate_v3", 1, 2),
        ("cuMemMapArrayAsync", 0, 1),
        ("cuMemMapArrayAsync_ptsz", 0, 1),
        ("cuStreamBatchMemOp", 2, 1),
        ("cuStreamBatchMemOp_ptsz", 2, 1),
        ("cuStreamBatchMemOp_v2", 2, 1),
    ];
    let mut derive_state = DeriveDisplayState::new(
        &ignore_types,
        types_crate,
        &ignore_functions,
        &count_selectors,
    );
    let mut items = module
        .items
        .iter()
        .filter_map(|i| cuda_derive_display_trait_for_item(types_crate, &mut derive_state, i))
        .collect::<Vec<_>>();
    items.push(curesult_display_trait(&derive_state));
    let mut output = output.clone();
    output.extend(path);
    write_rust_to_file(
        output,
        &prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items,
        }),
    );
}

fn generate_display_perflib(
    output: &PathBuf,
    path: &[&str],
    types_crate: &[&'static str],
    module: &syn::File,
) {
    let ignore_types = [
        "cublasLtMatrixLayoutOpaque_t",
        "cublasLtMatmulDescOpaque_t",
        "cublasLtMatrixTransformDescOpaque_t",
        "cublasLtMatmulPreferenceOpaque_t",
        "cublasLogCallback",
        "cudnnBackendDescriptor_t",
        "cublasLtLoggerCallback_t",
        "cusparseLoggerCallback_t",
    ];
    let ignore_functions = [];
    let count_selectors = [
        ("cudnnBackendSetAttribute", 4, 3),
        ("cudnnBackendGetAttribute", 5, 4),
    ];
    let mut derive_state = DeriveDisplayState::new(
        &ignore_types,
        types_crate,
        &ignore_functions,
        &count_selectors,
    );
    let items = module
        .items
        .iter()
        .filter_map(|i| cuda_derive_display_trait_for_item(types_crate, &mut derive_state, i))
        .collect::<Vec<_>>();
    let mut output = output.clone();
    output.extend(path);
    write_rust_to_file(
        output,
        &prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items,
        }),
    );
}

struct DeriveDisplayState<'a> {
    types_crate: Path,
    ignore_types: FxHashSet<Ident>,
    ignore_fns: FxHashSet<Ident>,
    enums: FxHashMap<&'a Ident, Vec<&'a Ident>>,
    array_arguments: FxHashMap<(Ident, usize), usize>,
    result_variants: Vec<&'a ItemConst>,
}

impl<'a> DeriveDisplayState<'a> {
    fn new(
        ignore_types: &[&'static str],
        types_crate: &[&'static str],
        ignore_fns: &[&'static str],
        count_selectors: &[(&'static str, usize, usize)],
    ) -> Self {
        let segments = types_crate
            .iter()
            .map(|seg| PathSegment {
                ident: Ident::new(seg, Span::call_site()),
                arguments: PathArguments::None,
            })
            .collect::<Punctuated<_, _>>();
        DeriveDisplayState {
            types_crate: Path {
                leading_colon: None,
                segments,
            },
            ignore_types: ignore_types
                .into_iter()
                .map(|x| Ident::new(x, Span::call_site()))
                .collect(),
            ignore_fns: ignore_fns
                .into_iter()
                .map(|x| Ident::new(x, Span::call_site()))
                .collect(),
            array_arguments: count_selectors
                .into_iter()
                .map(|(name, val, count)| ((Ident::new(name, Span::call_site()), *val), *count))
                .collect(),
            enums: Default::default(),
            result_variants: Vec::new(),
        }
    }

    fn record_enum_variant(&mut self, enum_: &'a Ident, variant: &'a Ident) {
        match self.enums.entry(enum_) {
            hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().push(variant);
            }
            hash_map::Entry::Vacant(entry) => {
                entry.insert(vec![variant]);
            }
        }
    }
}

fn cuda_derive_display_trait_for_item<'a>(
    path: &[&str],
    state: &mut DeriveDisplayState<'a>,
    item: &'a Item,
) -> Option<syn::Item> {
    let path_prefix = &state.types_crate;
    let path_prefix_iter = iter::repeat(&path_prefix);
    let mut prepend_path = PrependCudaPath {
        module: path
            .iter()
            .map(|segment| Ident::new(segment, Span::call_site()))
            .collect(),
    };
    match item {
        Item::Const(const_) => {
            if const_.ty.to_token_stream().to_string() == "cudaError_enum" {
                state.result_variants.push(const_);
            }
            None
        }
        Item::ForeignMod(ItemForeignMod { items, .. }) => match items.last().unwrap() {
            ForeignItem::Fn(ForeignItemFn {
                sig: Signature { ident, inputs, .. },
                ..
            }) => {
                if state.ignore_fns.contains(ident) {
                    return None;
                }
                let inputs = inputs
                    .iter()
                    .map(|fn_arg| {
                        let mut fn_arg = fn_arg.clone();
                        syn::visit_mut::visit_fn_arg_mut(&mut prepend_path, &mut fn_arg);
                        fn_arg
                    })
                    .collect::<Vec<_>>();
                let inputs_iter = inputs.iter();
                let original_fn_name = ident.to_string();
                let mut write_argument = inputs.iter().enumerate().map(|(index, fn_arg)| {
                    let name = fn_arg_name(fn_arg);
                    if let Some(length_index) = state.array_arguments.get(&(ident.clone(), index)) {
                        let length = fn_arg_name(&inputs[*length_index]);
                        quote! {
                            writer.write_all(concat!(stringify!(#name), ": ").as_bytes())?;
                            writer.write_all(b"[")?;
                            for i in 0..#length {
                                if i != 0 {
                                    writer.write_all(b", ")?;
                                }
                                crate::CudaDisplay::write(unsafe { &*#name.add(i as usize) }, #original_fn_name, arg_idx, writer)?;
                            }
                            writer.write_all(b"]")?;
                        }
                    } else {
                        quote! {
                            writer.write_all(concat!(stringify!(#name), ": ").as_bytes())?;
                            crate::CudaDisplay::write(&#name, #original_fn_name, arg_idx, writer)?;
                        }
                    }
                });
                let fn_name = format_ident!("write_{}", ident);
                Some(match write_argument.next() {
                    Some(first_write_argument) => parse_quote! {
                        pub fn #fn_name(writer: &mut (impl std::io::Write + ?Sized), #(#inputs_iter,)*) -> std::io::Result<()> {
                            let mut arg_idx = 0usize;
                            writer.write_all(b"(")?;
                            #first_write_argument
                            #(
                                arg_idx += 1;
                                writer.write_all(b", ")?;
                                #write_argument
                            )*
                            writer.write_all(b")")
                        }
                    },
                    None => parse_quote! {
                        pub fn #fn_name(writer: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
                            writer.write_all(b"()")
                        }
                    },
                })
            }
            _ => unreachable!(),
        },
        Item::Impl(ref item_impl) => {
            let enum_ = match &*item_impl.self_ty {
                Type::Path(ref path) => &path.path.segments.last().unwrap().ident,
                _ => unreachable!(),
            };
            let variant_ = match item_impl.items.last().unwrap() {
                syn::ImplItem::Const(item_const) => &item_const.ident,
                _ => unreachable!(),
            };
            state.record_enum_variant(enum_, variant_);
            None
        }
        Item::Struct(item_struct) => {
            if state.ignore_types.contains(&item_struct.ident) {
                return None;
            }
            if state.enums.contains_key(&item_struct.ident) {
                let enum_ = &item_struct.ident;
                let enum_iter = iter::repeat(&item_struct.ident);
                let variants = state.enums.get(&item_struct.ident).unwrap().iter();
                Some(parse_quote! {
                    impl crate::CudaDisplay for #path_prefix :: #enum_ {
                        fn write(&self, _fn_name: &'static str, _index: usize, writer: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
                            match self {
                                #(& #path_prefix_iter :: #enum_iter :: #variants => writer.write_all(stringify!(#variants).as_bytes()),)*
                                _ => write!(writer, "{}", self.0)
                            }
                        }
                    }
                })
            } else {
                let struct_ = &item_struct.ident;
                match item_struct.fields {
                    Fields::Named(ref fields) => {
                        let mut rest_of_fields = fields.named.iter().filter_map(|f| {
                            let f_ident = f.ident.as_ref().unwrap();
                            let name = f_ident.to_string();
                            if name.starts_with("reserved") || name == "_unused" {
                                None
                            } else {
                                Some(f_ident)
                            }
                        });
                        let first_field = match rest_of_fields.next() {
                            Some(f) => f,
                            None => return None,
                        };
                        Some(parse_quote! {
                            impl crate::CudaDisplay for #path_prefix :: #struct_ {
                                fn write(&self, _fn_name: &'static str, _index: usize, writer: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
                                    writer.write_all(concat!("{ ", stringify!(#first_field), ": ").as_bytes())?;
                                    crate::CudaDisplay::write(&self.#first_field, "", 0, writer)?;
                                    #(
                                        writer.write_all(concat!(", ", stringify!(#rest_of_fields), ": ").as_bytes())?;
                                        crate::CudaDisplay::write(&self.#rest_of_fields, "", 0, writer)?;
                                    )*
                                    writer.write_all(b" }")
                                }
                            }
                        })
                    }
                    Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) if unnamed.len() == 1 => {
                        Some(parse_quote! {
                            impl crate::CudaDisplay for #path_prefix :: #struct_ {
                                fn write(&self, _fn_name: &'static str, _index: usize, writer: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
                                    write!(writer, "{:p}", self.0)
                                }
                            }
                        })
                    }
                    _ => return None,
                }
            }
        }
        Item::Type(item_type) => {
            if state.ignore_types.contains(&item_type.ident) {
                return None;
            };
            match &*item_type.ty {
                Type::Ptr(_) => {
                    let type_ = &item_type.ident;
                    Some(parse_quote! {
                        impl crate::CudaDisplay for #path_prefix :: #type_ {
                            fn write(&self, _fn_name: &'static str, _index: usize, writer: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
                                if self.is_null() {
                                    writer.write_all(b"NULL")
                                } else {
                                    write!(writer, "{:p}", *self)
                                }
                            }
                        }
                    })
                }
                Type::Path(type_path) => {
                    if type_path.path.leading_colon.is_some() {
                        let option_seg = type_path.path.segments.last().unwrap();
                        if option_seg.ident == "Option" {
                            match &option_seg.arguments {
                                PathArguments::AngleBracketed(generic) => match generic.args[0] {
                                    syn::GenericArgument::Type(Type::BareFn(_)) => {
                                        let type_ = &item_type.ident;
                                        return Some(parse_quote! {
                                            impl crate::CudaDisplay for #path_prefix :: #type_ {
                                                fn write(&self, _fn_name: &'static str, _index: usize, writer: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
                                                    write!(writer, "{:p}", unsafe { std::mem::transmute::<#path_prefix :: #type_, *mut ::std::ffi::c_void>(*self) })
                                                }
                                            }
                                        });
                                    }
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            }
                        }
                    }
                    None
                }
                _ => unreachable!(),
            }
        }
        Item::Union(_) => None,
        Item::Use(_) => None,
        _ => unreachable!(),
    }
}

fn fn_arg_name(fn_arg: &FnArg) -> &Box<syn::Pat> {
    let name = if let FnArg::Typed(t) = fn_arg {
        &t.pat
    } else {
        unreachable!()
    };
    name
}

fn curesult_display_trait(derive_state: &DeriveDisplayState) -> syn::Item {
    let errors = derive_state.result_variants.iter().filter_map(|const_| {
        let prefix = "cudaError_enum_";
        let text = &const_.ident.to_string()[prefix.len()..];
        if text == "CUDA_SUCCESS" {
            return None;
        }
        let expr = &const_.expr;
        Some(quote! {
            #expr => writer.write_all(#text.as_bytes()),
        })
    });
    parse_quote! {
        impl crate::CudaDisplay for cuda_types::cuda::CUresult {
            fn write(&self, _fn_name: &'static str, _index: usize, writer: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
                match self {
                    Ok(()) => writer.write_all(b"CUDA_SUCCESS"),
                    Err(err) => {
                        match err.0.get() {
                            #(#errors)*
                            err => write!(writer, "{}", err)
                        }
                    }
                }
            }
        }
    }
}

fn new_builder() -> bindgen::Builder {
    bindgen::Builder::default()
        .use_core()
        .rust_target(bindgen::RustTarget::Stable_1_77)
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .derive_hash(true)
        .derive_eq(true)
}
