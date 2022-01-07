extern crate proc_macro;

use std::collections::hash_map;
use std::iter;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use rustc_hash::{FxHashMap, FxHashSet};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Brace;
use syn::visit_mut::VisitMut;
use syn::{
    bracketed, parse_macro_input, Abi, Fields, File, FnArg, ForeignItem, ForeignItemFn, Ident,
    Item, ItemForeignMod, ItemMacro, LitStr, Macro, MacroDelimiter, PatType, Path, PathArguments,
    PathSegment, ReturnType, Signature, Token, Type, TypeArray, TypePath, TypePtr,
};

const CUDA_RS: &'static str = include_str! {"cuda.rs"};

// This macro copies cuda.rs as-is with some changes:
// * All function declarations are filtered out
// * CUdeviceptr_v2 is redefined from `unsigned long long` to `*void`
// * `extern "C"` gets replaced by `extern "system"`
// * CUuuid_st is redefined to use uchar instead of char
#[proc_macro]
pub fn cuda_type_declarations(_: TokenStream) -> TokenStream {
    let mut cuda_module = syn::parse_str::<File>(CUDA_RS).unwrap();
    cuda_module.items = cuda_module
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::ForeignMod(_) => None,
            Item::Struct(mut struct_) => {
                if "CUdeviceptr_v2" == struct_.ident.to_string() {
                    match &mut struct_.fields {
                        Fields::Unnamed(ref mut fields) => {
                            fields.unnamed[0].ty =
                                absolute_path_to_mut_ptr(&["std", "os", "raw", "c_void"])
                        }
                        _ => unreachable!(),
                    }
                } else if "CUuuid_st" == struct_.ident.to_string() {
                    match &mut struct_.fields {
                        Fields::Named(ref mut fields) => match fields.named[0].ty {
                            Type::Array(TypeArray { ref mut elem, .. }) => {
                                *elem = Box::new(Type::Path(TypePath {
                                    qself: None,
                                    path: segments_to_path(&["std", "os", "raw", "c_uchar"]),
                                }))
                            }
                            _ => unreachable!(),
                        },
                        _ => panic!(),
                    }
                }
                Some(Item::Struct(struct_))
            }
            i => Some(i),
        })
        .collect::<Vec<_>>();
    syn::visit_mut::visit_file_mut(&mut FixAbi, &mut cuda_module);
    cuda_module.into_token_stream().into()
}

fn segments_to_path(path: &[&'static str]) -> Path {
    let mut segments = Punctuated::new();
    for ident in path {
        let ident = PathSegment {
            ident: Ident::new(ident, Span::call_site()),
            arguments: PathArguments::None,
        };
        segments.push(ident);
    }
    Path {
        leading_colon: Some(Token![::](Span::call_site())),
        segments,
    }
}

fn absolute_path_to_mut_ptr(path: &[&'static str]) -> Type {
    Type::Ptr(TypePtr {
        star_token: Token![*](Span::call_site()),
        const_token: None,
        mutability: Some(Token![mut](Span::call_site())),
        elem: Box::new(Type::Path(TypePath {
            qself: None,
            path: segments_to_path(path),
        })),
    })
}

struct FixAbi;

impl VisitMut for FixAbi {
    fn visit_abi_mut(&mut self, i: &mut Abi) {
        if let Some(ref mut name) = i.name {
            *name = LitStr::new("system", Span::call_site());
        }
    }
}

// This macro accepts following arguments:
// * `type_path`: path to the module with type definitions (in the module tree)
// * `normal_macro`: ident for a normal macro
// * `override_macro`: ident for an override macro
// * `override_fns`: list of override functions
// Then macro goes through every function in rust.rs, and for every fn `foo`:
// * if `foo` is contained in `override_fns` then pass it into `override_macro`
// * if `foo` is not contained in `override_fns` pass it to `normal_macro`
// Both `override_macro` and `normal_macro` expect this format:
//   macro_foo!("system" fn cuCtxDetach(ctx: CUcontext) -> CUresult)
// Additionally, it does a fixup of CUDA types so they get prefixed with `type_path`
#[proc_macro]
pub fn cuda_function_declarations(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as FnDeclInput);
    let cuda_module = syn::parse_str::<File>(CUDA_RS).unwrap();
    let override_fns = input
        .override_fns
        .iter()
        .map(ToString::to_string)
        .collect::<FxHashSet<_>>();
    cuda_module
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::ForeignMod(ItemForeignMod { mut items, .. }) => match items.pop().unwrap() {
                ForeignItem::Fn(ForeignItemFn {
                    sig:
                        Signature {
                            ident,
                            inputs,
                            output,
                            ..
                        },
                    ..
                }) => {
                    let path = if override_fns.contains(&ident.to_string()) {
                        &input.override_macro
                    } else {
                        &input.normal_macro
                    }
                    .clone();
                    let inputs = inputs
                        .into_iter()
                        .map(|fn_arg| match fn_arg {
                            FnArg::Typed(mut pat_type) => {
                                pat_type.ty =
                                    prepend_cuda_path_to_type(&input.type_path, pat_type.ty);
                                FnArg::Typed(pat_type)
                            }
                            _ => unreachable!(),
                        })
                        .collect::<Punctuated<_, Token![,]>>();
                    let output = match output {
                        ReturnType::Type(_, type_) => type_,
                        ReturnType::Default => unreachable!(),
                    };
                    let type_path = input.type_path.clone();
                    let tokens = quote! {
                        "system" fn #ident(#inputs) -> #type_path :: #output
                    };
                    Some(Item::Macro(ItemMacro {
                        attrs: Vec::new(),
                        ident: None,
                        mac: Macro {
                            path,
                            bang_token: Token![!](Span::call_site()),
                            delimiter: MacroDelimiter::Brace(Brace {
                                span: Span::call_site(),
                            }),
                            tokens,
                        },
                        semi_token: None,
                    }))
                }
                _ => unreachable!(),
            },
            _ => None,
        })
        .map(Item::into_token_stream)
        .collect::<proc_macro2::TokenStream>()
        .into()
}

fn prepend_cuda_path_to_type(base_path: &Path, type_: Box<Type>) -> Box<Type> {
    match *type_ {
        Type::Path(mut type_path) => {
            type_path.path = prepend_cuda_path_to_path(base_path, type_path.path);
            Box::new(Type::Path(type_path))
        }
        Type::Ptr(mut type_ptr) => {
            type_ptr.elem = prepend_cuda_path_to_type(base_path, type_ptr.elem);
            Box::new(Type::Ptr(type_ptr))
        }
        _ => unreachable!(),
    }
}

fn prepend_cuda_path_to_path(base_path: &Path, path: Path) -> Path {
    if path.leading_colon.is_some() {
        return path;
    }
    if path.segments.len() == 1 {
        let ident = path.segments[0].ident.to_string();
        if ident.starts_with("CU") || ident.starts_with("cu") {
            let mut base_path = base_path.clone();
            base_path.segments.extend(path.segments);
            return base_path;
        }
    }
    path
}

struct FnDeclInput {
    type_path: Path,
    normal_macro: Path,
    override_macro: Path,
    override_fns: Punctuated<Ident, Token![,]>,
}

impl Parse for FnDeclInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let type_path = input.parse::<Path>()?;
        input.parse::<Token![,]>()?;
        let normal_macro = input.parse::<Path>()?;
        input.parse::<Token![,]>()?;
        let override_macro = input.parse::<Path>()?;
        input.parse::<Token![,]>()?;
        let override_fns_content;
        bracketed!(override_fns_content in input);
        let override_fns = override_fns_content.parse_terminated(Ident::parse)?;
        Ok(Self {
            type_path,
            normal_macro,
            override_macro,
            override_fns,
        })
    }
}

// This trait accepts following parameters:
// * `type_path`: path to the module with type definitions (in the module tree)
// * `trait_`: name of the trait to be derived
// * `ignore_structs`: bracketed list of types to ignore
// * `ignore_fns`: bracketed list of fns to ignore
#[proc_macro]
pub fn cuda_derive_display_trait(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveDisplayInput);
    let cuda_module = syn::parse_str::<File>(CUDA_RS).unwrap();
    let mut derive_state = DeriveDisplayState::new(input);
    cuda_module
        .items
        .into_iter()
        .filter_map(|i| cuda_derive_display_trait_for_item(&mut derive_state, i))
        .collect::<proc_macro2::TokenStream>()
        .into()
}

fn cuda_derive_display_trait_for_item(
    state: &mut DeriveDisplayState,
    item: Item,
) -> Option<proc_macro2::TokenStream> {
    let path_prefix = &state.type_path;
    let path_prefix_iter = iter::repeat(&path_prefix);
    let trait_ = &state.trait_;
    let trait_iter = iter::repeat(&state.trait_);
    match item {
        Item::Const(_) => None,
        Item::ForeignMod(ItemForeignMod { mut items, .. }) => match items.pop().unwrap() {
            ForeignItem::Fn(ForeignItemFn {
                sig: Signature { ident, inputs, .. },
                ..
            }) => {
                if state.ignore_fns.contains(&ident) {
                    return None;
                }
                let inputs = inputs
                    .into_iter()
                    .map(|fn_arg| match fn_arg {
                        FnArg::Typed(mut pat_type) => {
                            pat_type.ty = prepend_cuda_path_to_type(path_prefix, pat_type.ty);
                            FnArg::Typed(pat_type)
                        }
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>();
                let inputs_iter = inputs.iter();
                let mut arg_name_iter = inputs.iter().map(|fn_arg| match fn_arg {
                    FnArg::Typed(PatType { pat, .. }) => pat,
                    _ => unreachable!(),
                });
                let fn_name = format_ident!("write_{}", ident);
                Some(match arg_name_iter.next() {
                    Some(first_arg_name) => quote! {
                        pub fn #fn_name(writer: &mut (impl std::io::Write + ?Sized), #(#inputs_iter,)*) -> std::io::Result<()> {
                            writer.write_all(concat!("(", stringify!(#first_arg_name), ": ").as_bytes())?;
                            CudaDisplay::write(&#first_arg_name, writer)?;
                            #(
                                writer.write_all(b", ")?;
                                writer.write_all(concat!(stringify!(#arg_name_iter), ": ").as_bytes())?;
                                CudaDisplay::write(&#arg_name_iter, writer)?;
                            )*
                            writer.write_all(b")")
                        }
                    },
                    None => quote! {
                        pub fn #fn_name(writer: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
                            writer.write_all(b"()")
                        }
                    },
                })
            }
            _ => unreachable!(),
        },
        Item::Impl(mut item_impl) => {
            let enum_ = match *(item_impl.self_ty) {
                Type::Path(mut path) => path.path.segments.pop().unwrap().into_value().ident,
                _ => unreachable!(),
            };
            let variant_ = match item_impl.items.pop().unwrap() {
                syn::ImplItem::Const(item_const) => item_const.ident,
                _ => unreachable!(),
            };
            state.record_enum_variant(enum_, variant_);
            None
        }
        Item::Struct(item_struct) => {
            let item_struct_name = item_struct.ident.to_string();
            if state.ignore_structs.contains(&item_struct.ident) {
                return None;
            }
            if item_struct_name.ends_with("_enum") {
                let enum_ = &item_struct.ident;
                let enum_iter = iter::repeat(&item_struct.ident);
                let variants = state.enums.get(&item_struct.ident).unwrap().iter();
                Some(quote! {
                    impl #trait_ for #path_prefix :: #enum_ {
                        fn write(&self, writer: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
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
                    Fields::Named(fields) => {
                        let mut all_idents = fields.named.into_iter().filter_map(|f| {
                            let f_ident = f.ident.unwrap();
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
                Some(quote! {
                    impl #trait_ for #path_prefix :: #struct_ {
                        fn write(&self, writer: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
                            writer.write_all(concat!("{ ", stringify!(#first_field), ": ").as_bytes())?;
                            #trait_::write(&self.#first_field, writer)?;
                            #(
                                writer.write_all(concat!(", ", stringify!(#rest_of_fields), ": ").as_bytes())?;
                                #trait_iter::write(&self.#rest_of_fields, writer)?;
                            )*
                            writer.write_all(b" }")
                        }
                    }
                })
            }
        }
        Item::Type(item_type) => match *(item_type.ty) {
            Type::Ptr(_) => {
                let type_ = item_type.ident;
                Some(quote! {
                    impl #trait_ for #path_prefix :: #type_ {
                        fn write(&self, writer: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
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
                                    return Some(quote! {
                                        impl #trait_ for #path_prefix :: #type_ {
                                            fn write(&self, writer: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
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
        },
        Item::Union(_) => None,
        Item::Use(_) => None,
        _ => unreachable!(),
    }
}

struct DeriveDisplayState {
    type_path: Path,
    trait_: Path,
    ignore_structs: FxHashSet<Ident>,
    ignore_fns: FxHashSet<Ident>,
    enums: FxHashMap<Ident, Vec<Ident>>,
}

impl DeriveDisplayState {
    fn new(input: DeriveDisplayInput) -> Self {
        DeriveDisplayState {
            type_path: input.type_path,
            trait_: input.trait_,
            ignore_structs: input.ignore_structs.into_iter().collect(),
            ignore_fns: input.ignore_fns.into_iter().collect(),
            enums: Default::default(),
        }
    }

    fn record_enum_variant(&mut self, enum_: Ident, variant: Ident) {
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

struct DeriveDisplayInput {
    type_path: Path,
    trait_: Path,
    ignore_structs: Punctuated<Ident, Token![,]>,
    ignore_fns: Punctuated<Ident, Token![,]>,
}

impl Parse for DeriveDisplayInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let type_path = input.parse::<Path>()?;
        input.parse::<Token![,]>()?;
        let trait_ = input.parse::<Path>()?;
        input.parse::<Token![,]>()?;
        let ignore_structs_buffer;
        bracketed!(ignore_structs_buffer in input);
        let ignore_structs = ignore_structs_buffer.parse_terminated(Ident::parse)?;
        input.parse::<Token![,]>()?;
        let ignore_fns_buffer;
        bracketed!(ignore_fns_buffer in input);
        let ignore_fns = ignore_fns_buffer.parse_terminated(Ident::parse)?;
        Ok(Self {
            type_path,
            trait_,
            ignore_structs,
            ignore_fns,
        })
    }
}
