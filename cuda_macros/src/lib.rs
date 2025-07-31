extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use rustc_hash::FxHashMap;
use std::iter;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;
use syn::{
    bracketed, parse_macro_input, File, ForeignItem, ForeignItemFn, Ident, Item, Path, Signature,
    Token,
};

const CUDA_RS: &'static str = include_str! {"cuda.rs"};
const NVML_RS: &'static str = include_str! {"nvml.rs"};
const CUBLAS_RS: &'static str = include_str! {"cublas.rs"};
const CUBLASLT_RS: &'static str = include_str! {"cublaslt.rs"};
const CUBLASLT_INTERNAL_RS: &'static str = include_str! {"cublaslt_internal.rs"};
const CUFFT_RS: &'static str = include_str! {"cufft.rs"};
const CUSPARSE_RS: &'static str = include_str! {"cusparse.rs"};
const CUDNN9_RS: &'static str = include_str! {"cudnn9.rs"};

// This macro accepts following arguments:
// * `normal_macro`: ident for a normal macro
// * zero or more:
//   * `override_macro`: ident for an override macro
//   * `override_fns`: list of override functions
// Then macro goes through every function in rust.rs, and for every fn `foo`:
// * if `foo` is contained in `override_fns` then pass it into `override_macro`
// * if `foo` is not contained in `override_fns` pass it to `normal_macro`
// Both `override_macro` and `normal_macro` expect semicolon-separated list:
//   macro_foo!(
//      "system" fn cuCtxDetach(ctx: CUcontext) -> CUresult;
//      "system" fn cuCtxDetach(ctx: CUcontext) -> CUresult
//   )
// Additionally, it does a fixup of CUDA types so they get prefixed with `type_path`
#[proc_macro]
pub fn cuda_function_declarations(tokens: TokenStream) -> TokenStream {
    function_declarations(tokens, CUDA_RS)
}

#[proc_macro]
pub fn cublas_function_declarations(tokens: TokenStream) -> TokenStream {
    function_declarations(tokens, CUBLAS_RS)
}

#[proc_macro]
pub fn cublaslt_function_declarations(tokens: TokenStream) -> TokenStream {
    function_declarations(tokens, CUBLASLT_RS)
}

#[proc_macro]
pub fn cublaslt_internal_function_declarations(tokens: TokenStream) -> TokenStream {
    function_declarations(tokens, CUBLASLT_INTERNAL_RS)
}

#[proc_macro]
pub fn cufft_function_declarations(tokens: TokenStream) -> TokenStream {
    function_declarations(tokens, CUFFT_RS)
}

#[proc_macro]
pub fn cusparse_function_declarations(tokens: TokenStream) -> TokenStream {
    function_declarations(tokens, CUSPARSE_RS)
}

#[proc_macro]
pub fn cudnn9_function_declarations(tokens: TokenStream) -> TokenStream {
    function_declarations(tokens, CUDNN9_RS)
}

fn function_declarations(tokens: TokenStream, module: &str) -> TokenStream {
    let input = parse_macro_input!(tokens as FnDeclInput);
    let mut cuda_module = syn::parse_str::<File>(module).unwrap();
    let mut choose_macro = ChooseMacro::new(input);
    syn::visit_mut::visit_file_mut(&mut FixFnSignatures, &mut cuda_module);
    for item in cuda_module.items {
        let extern_ = if let Item::ForeignMod(extern_) = item {
            extern_
        } else {
            unreachable!()
        };
        let abi = extern_.abi.name;
        for mut item in extern_.items {
            if let ForeignItem::Fn(ForeignItemFn {
                sig: Signature { ref ident, .. },
                ref mut attrs,
                ..
            }) = item
            {
                *attrs = Vec::new();
                choose_macro.add(ident, quote! { #abi #item });
            } else {
                unreachable!()
            }
        }
    }
    let mut result = proc_macro2::TokenStream::new();
    for (path, items) in
        iter::once(choose_macro.default).chain(choose_macro.override_sets.into_iter())
    {
        if items.is_empty() {
            continue;
        }
        quote! {
            #path ! { #(#items)* }
        }
        .to_tokens(&mut result);
    }
    result.into()
}

#[proc_macro]
pub fn nvml_function_declarations(tokens: TokenStream) -> TokenStream {
    function_declarations(tokens, NVML_RS)
}
struct FnDeclInput {
    normal_macro: Path,
    overrides: Punctuated<OverrideMacro, Token![,]>,
}

impl Parse for FnDeclInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let normal_macro = input.parse::<Path>()?;
        let overrides = if input.is_empty() {
            Punctuated::new()
        } else {
            input.parse::<Token![,]>()?;
            input.parse_terminated(OverrideMacro::parse, Token![,])?
        };
        Ok(Self {
            normal_macro,
            overrides,
        })
    }
}
struct OverrideMacro {
    macro_: Path,
    functions: Punctuated<Ident, Token![,]>,
}

impl Parse for OverrideMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let macro_ = input.parse::<Path>()?;
        input.parse::<Token![<=]>()?;
        let functions_content;
        bracketed!(functions_content in input);
        let functions = functions_content.parse_terminated(Ident::parse, Token![,])?;
        Ok(Self { macro_, functions })
    }
}

struct ChooseMacro {
    default: (Path, Vec<proc_macro2::TokenStream>),
    override_lookup: FxHashMap<Ident, Path>,
    override_sets: FxHashMap<Path, Vec<proc_macro2::TokenStream>>,
}

impl ChooseMacro {
    fn new(input: FnDeclInput) -> Self {
        let mut override_lookup = FxHashMap::default();
        let mut override_sets = FxHashMap::default();
        for OverrideMacro { macro_, functions } in input.overrides {
            for ident in functions {
                override_lookup.insert(ident, macro_.clone());
                override_sets.insert(macro_.clone(), Vec::new());
            }
        }
        Self {
            default: (input.normal_macro, Vec::new()),
            override_lookup,
            override_sets,
        }
    }

    fn add(&mut self, ident: &Ident, tokens: proc_macro2::TokenStream) {
        match self.override_lookup.get(ident) {
            Some(override_macro) => {
                self.override_sets
                    .get_mut(override_macro)
                    .unwrap()
                    .push(tokens);
            }
            None => self.default.1.push(tokens),
        }
    }
}

// For some reason prettyplease will append trailing comma *only*
// if there are two or more arguments
struct FixFnSignatures;

impl VisitMut for FixFnSignatures {
    fn visit_signature_mut(&mut self, s: &mut syn::Signature) {
        s.inputs.pop_punct();
    }
}

const MODULES: &[&str] = &[
    "context", "device", "driver", "event", "function", "graph", "kernel",
    "library", "link", "memory", "module", "pointer", "stream"
];

fn normalize_fn_impl(
    prefix: &str,
    default_module: Option<&str>,
    tokens: TokenStream,
) -> TokenStream {
    let mut path = parse_macro_input!(tokens as syn::Path);
    let fn_ = path
        .segments
        .pop()
        .unwrap()
        .into_tuple()
        .0
        .ident
        .to_string();
    let already_has_module = MODULES.contains(&&*path.segments.last().unwrap().ident.to_string());
    let segments: Vec<String> = split(&fn_[prefix.len()..]); // skip "cu"
    let fn_path = join(segments, default_module.filter(|_| !already_has_module));
    quote! {
        #path #fn_path
    }
    .into()
}

#[proc_macro]
pub fn cuda_normalize_fn(tokens: TokenStream) -> TokenStream {
    normalize_fn_impl("cu", Some("driver"), tokens)
}

#[proc_macro]
pub fn cublas_normalize_fn(tokens: TokenStream) -> TokenStream {
    normalize_fn_impl("cublas", None, tokens)
}

#[proc_macro]
pub fn cublaslt_normalize_fn(tokens: TokenStream) -> TokenStream {
    normalize_fn_impl("cublasLt", None, tokens)
}

#[proc_macro]
pub fn cudnn_normalize_fn(tokens: TokenStream) -> TokenStream {
    normalize_fn_impl("cudnn", None, tokens)
}

#[proc_macro]
pub fn cusparse_normalize_fn(tokens: TokenStream) -> TokenStream {
    normalize_fn_impl("cusparse", None, tokens)
}

#[proc_macro]
pub fn nvml_normalize_fn(tokens: TokenStream) -> TokenStream {
    normalize_fn_impl("nvml", None, tokens)
}

fn split(fn_: &str) -> Vec<String> {
    let mut result = Vec::new();
    for c in fn_.chars() {
        if c.is_ascii_uppercase() {
            result.push(c.to_ascii_lowercase().to_string());
        } else {
            result.last_mut().unwrap().push(c);
        }
    }
    result
}

fn join(
    fn_: Vec<String>,
    default_module: Option<&str>,
) -> Punctuated<Ident, Token![::]> {
    fn full_form(segment: &str) -> Option<&[&str]> {
        Some(match segment {
            "ctx" => &["context"],
            "func" => &["function"],
            "mem" => &["memory"],
            "memcpy" => &["memory", "copy"],
            "memset" => &["memory", "set"],
            _ => return None,
        })
    }
    let mut normalized: Vec<&str> = Vec::new();
    for segment in fn_.iter() {
        match full_form(segment) {
            Some(segments) => normalized.extend(segments.into_iter()),
            None => normalized.push(&*segment),
        }
    }
    if let Some(default_module) = default_module {
        if !MODULES.contains(&normalized[0]) {
        let mut globalized = vec![default_module];
        globalized.extend(normalized);
        normalized = globalized;
    }
    let (module, path) = normalized.split_first().unwrap();
    let path = path.join("_");
    [module, &&*path]
        .into_iter()
        .map(|s| Ident::new(s, Span::call_site()))
        .collect()
    } else {
        return [Ident::new(&normalized.join("_"), Span::call_site())]
            .into_iter()
            .collect();
    }
    
}

#[proc_macro]
pub fn generate_api_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ApiMacroInput);
    let ApiMacroInput(macro_name, _, trait_name, _, type_name) = input;
    let expanded = quote! {
        struct #type_name;
        macro_rules! #macro_name {
            ($($abi:literal fn $fn_name:ident( $( $arg_id:ident : $arg_type:ty ),* ) -> $ret_type:ty;)*) => {
                impl #trait_name for #type_name {
                    fn new() -> Self { Self }
                    $(
                        #[inline(always)]
                        fn $fn_name(&self, $( $arg_id : $arg_type ),* ) -> $ret_type {
                            unsafe { super::$fn_name( $( $arg_id ),* ) }
                        }
                    )*
                }
            };
        }
    };
    TokenStream::from(expanded)
}
#[allow(dead_code)]
struct ApiMacroInput(Ident, Token![,], Path, Token![,], Path);
impl syn::parse::Parse for ApiMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(ApiMacroInput(
            input.parse()?, // macro name
            input.parse()?, // comma
            input.parse()?, // trait name
            input.parse()?, // comma
            input.parse()?  // type name
        ))
    }
}
