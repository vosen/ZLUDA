use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::hash_map, fs::File, io::Write, iter, path::PathBuf, str::FromStr};
use syn::{
    parse_quote, punctuated::Punctuated, visit_mut::VisitMut, Abi, Fields, FnArg, ForeignItem,
    ForeignItemFn, Ident, Item, ItemConst, ItemForeignMod, ItemUse, LitStr, Path, PathArguments,
    Signature, Type, TypePath, UseTree,
};

fn main() {
    let crate_root = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    generate_hip_runtime(
        &crate_root,
        &["..", "ext", "hip_runtime-sys", "src", "lib.rs"],
    );
    let cuda_header = bindgen::Builder::default()
        .use_core()
        .rust_target(bindgen::RustTarget::Stable_1_77)
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .derive_eq(true)
        .header_contents("cuda_wrapper.h", include_str!("../build/cuda_wrapper.h"))
        .allowlist_type("^CU.*")
        .allowlist_function("^cu.*")
        .allowlist_var("^CU.*")
        .must_use_type("cudaError_enum")
        .constified_enum("cudaError_enum")
        .no_partialeq("CUDA_HOST_NODE_PARAMS_st")
        .new_type_alias(r"^CUdevice_v\d+$")
        .new_type_alias(r"^CUdeviceptr_v\d+$")
        .clang_args(["-I/usr/local/cuda/include"])
        .generate()
        .unwrap()
        .to_string();
    let module: syn::File = syn::parse_str(&cuda_header).unwrap();
    generate_functions(&crate_root, &["..", "cuda_base", "src", "cuda.rs"], &module);
    generate_types(&crate_root, &["..", "cuda_types", "src", "lib.rs"], &module);
    generate_display(
        &crate_root,
        &["..", "zluda_dump", "src", "format_generated.rs"],
        "cuda_types",
        &module,
    )
}

fn generate_hip_runtime(output: &PathBuf, path: &[&str]) {
    let hiprt_header = bindgen::Builder::default()
        .use_core()
        .rust_target(bindgen::RustTarget::Stable_1_77)
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .derive_eq(true)
        .header("/opt/rocm/include/hip/hip_runtime_api.h")
        .allowlist_type("^hip.*")
        .allowlist_function("^hip.*")
        .allowlist_var("^hip.*")
        .must_use_type("hipError_t")
        .constified_enum("hipError_t")
        .new_type_alias("^hipDeviceptr_t$")
        .new_type_alias("^hipModule_t$")
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
    add_send_sync(&mut module.items, &["hipModule_t"]);
    let mut output = output.clone();
    output.extend(path);
    write_rust_to_file(output, &prettyplease::unparse(&module))
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

fn generate_functions(output: &PathBuf, path: &[&str], module: &syn::File) {
    let fns_ = module.items.iter().filter_map(|item| match item {
        Item::ForeignMod(extern_) => match &*extern_.items {
            [ForeignItem::Fn(fn_)] => Some(fn_),
            _ => unreachable!(),
        },
        _ => None,
    });
    let mut module: syn::File = parse_quote! {
        extern "system" {
            #(#fns_)*
        }
    };
    syn::visit_mut::visit_file_mut(&mut PrependCudaPath, &mut module);
    syn::visit_mut::visit_file_mut(&mut RemoveVisibility, &mut module);
    syn::visit_mut::visit_file_mut(&mut ExplicitReturnType, &mut module);
    let mut output = output.clone();
    output.extend(path);
    write_rust_to_file(output, &prettyplease::unparse(&module))
}

fn generate_types(output: &PathBuf, path: &[&str], module: &syn::File) {
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
                let error_ident = format_ident!("{}", &ident[old_prefix_len..]);
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
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

struct PrependCudaPath;

impl VisitMut for PrependCudaPath {
    fn visit_type_path_mut(&mut self, type_: &mut TypePath) {
        if type_.path.segments.len() == 1 {
            match &*type_.path.segments[0].ident.to_string() {
                "usize" | "f64" | "f32" => {}
                _ => {
                    *type_ = parse_quote! { cuda_types :: #type_ };
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
            *i = parse_quote! { -> {} };
        }
    }
}

fn generate_display(
    output: &PathBuf,
    path: &[&str],
    types_crate: &'static str,
    module: &syn::File,
) {
    let ignore_types = [
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
        .filter_map(|i| cuda_derive_display_trait_for_item(&mut derive_state, i))
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

struct DeriveDisplayState<'a> {
    types_crate: &'static str,
    ignore_types: FxHashSet<Ident>,
    ignore_fns: FxHashSet<Ident>,
    enums: FxHashMap<&'a Ident, Vec<&'a Ident>>,
    array_arguments: FxHashMap<(Ident, usize), usize>,
    result_variants: Vec<&'a ItemConst>,
}

impl<'a> DeriveDisplayState<'a> {
    fn new(
        ignore_types: &[&'static str],
        types_crate: &'static str,
        ignore_fns: &[&'static str],
        count_selectors: &[(&'static str, usize, usize)],
    ) -> Self {
        DeriveDisplayState {
            types_crate,
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
    state: &mut DeriveDisplayState<'a>,
    item: &'a Item,
) -> Option<syn::Item> {
    let path_prefix = Path::from(Ident::new(state.types_crate, Span::call_site()));
    let path_prefix_iter = iter::repeat(&path_prefix);
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
                        syn::visit_mut::visit_fn_arg_mut(&mut PrependCudaPath, &mut fn_arg);
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
                                crate::format::CudaDisplay::write(unsafe { &*#name.add(i as usize) }, #original_fn_name, arg_idx, writer)?;
                            }
                            writer.write_all(b"]")?;
                        }
                    } else {
                        quote! {
                            writer.write_all(concat!(stringify!(#name), ": ").as_bytes())?;
                            crate::format::CudaDisplay::write(&#name, #original_fn_name, arg_idx, writer)?;
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
                    impl crate::format::CudaDisplay for #path_prefix :: #enum_ {
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
                let (first_field, rest_of_fields) = match item_struct.fields {
                    Fields::Named(ref fields) => {
                        let mut all_idents = fields.named.iter().filter_map(|f| {
                            let f_ident = f.ident.as_ref().unwrap();
                            let name = f_ident.to_string();
                            if name.starts_with("reserved") || name == "_unused" {
                                None
                            } else {
                                Some(f_ident)
                            }
                        });
                        let first = match all_idents.next() {
                            Some(f) => f,
                            None => return None,
                        };
                        (first, all_idents)
                    }
                    _ => return None,
                };
                Some(parse_quote! {
                    impl crate::format::CudaDisplay for #path_prefix :: #struct_ {
                        fn write(&self, _fn_name: &'static str, _index: usize, writer: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
                            writer.write_all(concat!("{ ", stringify!(#first_field), ": ").as_bytes())?;
                            crate::format::CudaDisplay::write(&self.#first_field, "", 0, writer)?;
                            #(
                                writer.write_all(concat!(", ", stringify!(#rest_of_fields), ": ").as_bytes())?;
                                crate::format::CudaDisplay::write(&self.#rest_of_fields, "", 0, writer)?;
                            )*
                            writer.write_all(b" }")
                        }
                    }
                })
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
                        impl crate::format::CudaDisplay for #path_prefix :: #type_ {
                            fn write(&self, _fn_name: &'static str, _index: usize, writer: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
                                write!(writer, "{:p}", *self)
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
                                            impl crate::format::CudaDisplay for #path_prefix :: #type_ {
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
        impl crate::format::CudaDisplay for cuda_types::CUresult {
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
