use proc_macro2::Span;
use quote::{format_ident, quote};
use std::{path::PathBuf, str::FromStr};
use syn::{
    parse_quote, punctuated::Punctuated, visit_mut::VisitMut, Abi, Item, ItemUse, LitStr, UseTree,
};

fn main() {
    let crate_root = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    let cuda_header = bindgen::Builder::default()
        .use_core()
        .header_contents("cuda_wrapper.h", include_str!("../build/cuda_wrapper.h"))
        .no_partialeq("CUDA_HOST_NODE_PARAMS_st")
        .derive_eq(true)
        .allowlist_type("^CU.*")
        .allowlist_function("^cu.*")
        .allowlist_var("^CU.*")
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .layout_tests(false)
        .new_type_alias(r"^CUdevice_v\d+$")
        .new_type_alias(r"^CUdeviceptr_v\d+$")
        .must_use_type("cudaError_enum")
        .constified_enum("cudaError_enum")
        .clang_args(["-I/usr/local/cuda/include"])
        .generate()
        .unwrap()
        .to_string();
    generate_types(
        crate_root,
        &["..", "cuda_types", "src", "lib.rs"],
        cuda_header,
    );
}

fn generate_types(mut output: PathBuf, path: &[&str], cuda_header: String) {
    let mut module: syn::File = syn::parse_str(&cuda_header).unwrap();
    module.attrs.push(parse_quote! {
        #![allow(warnings)]
    });
    let mut converter = ConvertIntoRustResult {
        type_: "CUresult",
        underlying_type: "cudaError_enum",
        new_error_type: "CUerror",
        error_prefix: ("CUDA_ERROR", "ERROR"),
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
            item => Some(item),
        })
        .collect::<Vec<_>>();
    converter.flush(&mut module.items);
    syn::visit_mut::visit_file_mut(&mut FixAbi, &mut module);
    for segment in path {
        output.push(segment);
    }
    std::fs::write(output, prettyplease::unparse(&module)).unwrap();
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
                let old_prefix_len = self.underlying_type.len() + 1 + self.error_prefix.0.len() + 1;
                let variant_ident =
                    format_ident!("{}_{}", self.error_prefix.1, &ident[old_prefix_len..]);
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
}

struct FixAbi;

impl VisitMut for FixAbi {
    fn visit_abi_mut(&mut self, i: &mut Abi) {
        if let Some(ref mut name) = i.name {
            *name = LitStr::new("system", Span::call_site());
        }
    }
}
