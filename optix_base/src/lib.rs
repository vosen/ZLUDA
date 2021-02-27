extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote, ToTokens};
use rustc_hash::{FxHashMap, FxHashSet};
use syn::spanned::Spanned;
use syn::Fields;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    visit::Visit,
    File, ImplItem, Item, Path, Token, Type, TypePath,
};

const OPTIX_RS: &'static str = include_str! {"optix.rs"};
const OPTIX6_RS: &'static str = include_str! {"optix6.rs"};

// This macro copies optix.rs as-is with some changes:
// * All function declarations are filtered out
// * All CUDA types are skipped
#[proc_macro]
pub fn optix_type_declarations(_: TokenStream) -> TokenStream {
    let mut optix_module = syn::parse_str::<File>(OPTIX_RS).unwrap();
    optix_module.items = optix_module
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::ForeignMod(_) => None,
            Item::Type(type_) => {
                if type_.ident.to_string().starts_with("CU") {
                    None
                } else {
                    Some(Item::Type(type_))
                }
            }
            Item::Struct(struct_) => {
                if struct_.ident.to_string().starts_with("CU") {
                    None
                } else {
                    Some(Item::Struct(struct_))
                }
            }
            i => Some(i),
        })
        .collect::<Vec<_>>();
    optix_module.into_token_stream().into()
}

// This macro accepts another macro which accepts a semicolon separated
// list of OptiX functions
//   macro_foo!(
//      "C" fn optixDeviceContextCreate(
//          fromContext: CUcontext,
//          options: *const OptixDeviceContextOptions,
//          context: *mut OptixDeviceContext,
//      ) -> OptixResult;
//      "C" fn optixGetErrorName(result: OptixResult) -> *const ::std::os::raw::c_char
//   )
#[proc_macro]
pub fn optix_function_declarations(tokens: TokenStream) -> TokenStream {
    let macro_ = parse_macro_input!(tokens as Path);
    let optix_module = syn::parse_str::<File>(OPTIX_RS).unwrap();
    let mut all_fns = FxHashMap::default();
    let mut ordered_exports = Vec::new();
    for item in optix_module.items.iter() {
        match item {
            Item::ForeignMod(extern_) => {
                for item in extern_.items.iter() {
                    if let syn::ForeignItem::Fn(fn_) = item {
                        all_fns.insert(&fn_.sig.ident, &fn_.sig);
                    }
                }
            }
            Item::Struct(struct_) if struct_.ident == "OptixFunctionTable" => {
                for fn_ in struct_.fields.iter() {
                    ordered_exports.push(fn_.ident.as_ref().unwrap());
                }
            }
            _ => {}
        }
    }
    let fns_ = ordered_exports
        .into_iter()
        .fold(Vec::new(), |mut vec, export| {
            if export.to_string().starts_with("reserved") {
                vec.push(quote! {
                    "C" fn #export() -> ()
                });
            } else {
                let fn_signature = all_fns[export];
                let name = &fn_signature.ident;
                let inputs = &fn_signature.inputs;
                let output = &fn_signature.output;
                vec.push(quote! {
                    "C" fn #name(#inputs) #output
                });
            }
            vec
        });
    quote! {
        #macro_ ! ( #(#fns_);* );
    }
    .into()
}

// This macro copies optix6.rs as-is with some changes:
// * All function declarations are filtered out
// * For selected enums we generate a safe version of the enum type and a conversion function
#[proc_macro]
pub fn optix6_type_declarations(tokens: TokenStream) -> TokenStream {
    let type_decl = parse_macro_input!(tokens as TypeDeclInput);
    let mut optix_module = syn::parse_str::<File>(OPTIX6_RS).unwrap();
    optix_module.items = optix_module
        .items
        .into_iter()
        .filter_map(|item| match item {
            Item::ForeignMod(_) => None,
            i => Some(i),
        })
        .collect::<Vec<_>>();

    let mut safe_enum_gen = GenerateSafeEnums::new(type_decl.enums.iter());
    syn::visit::visit_file(&mut safe_enum_gen, &optix_module);
    let safe_enums = safe_enum_gen.generate();
    let mut token_stream = optix_module.into_token_stream();
    token_stream.extend(safe_enums);
    token_stream.into()
}

struct GenerateSafeEnums<'ast> {
    enums: FxHashMap<&'ast Ident, (Option<&'ast Type>, Vec<&'ast Ident>)>,
}

impl<'ast> GenerateSafeEnums<'ast> {
    fn new(enums: impl Iterator<Item = &'ast Ident>) -> Self {
        let enums = enums.map(|e| (e, (None, Vec::new()))).collect();
        Self { enums }
    }

    fn generate(self) -> TokenStream2 {
        let mut token_stream = TokenStream2::new();
        for (enum_, (underlying_type, items)) in self.enums {
            let safe_enum = format_ident!("{}Safe", enum_);
            let underlying_type = Self::to_ident(underlying_type.unwrap());
            token_stream.extend(quote! {
                #[repr(#underlying_type)]
                #[derive(Copy, Clone, PartialEq, Eq)]
                pub enum #safe_enum {
                    #(
                        #items = #enum_ :: #items .0,
                    )*
                }

                impl #safe_enum {
                    pub fn new(e: #enum_) -> Option<Self> {
                        match e {
                            #(
                                #enum_ :: #items => Some(#safe_enum :: #items),
                            )*
                            _ => None
                        }
                    }
                }

                impl From<#safe_enum> for #enum_ {
                    fn from(value: #safe_enum) -> #enum_ {
                        match value {
                            #(
                                #safe_enum :: #items => #enum_ :: #items,
                            )*
                        }
                    }
                }
            });
        }
        token_stream
    }

    fn to_ident(ty: &Type) -> Ident {
        if let Type::Path(ty_path) = ty {
            match &*ty_path.path.segments.last().unwrap().ident.to_string() {
                "c_uint" => Ident::new("u32", ty_path.span()),
                "c_int" => Ident::new("i32", ty_path.span()),
                _ => unimplemented!(),
            }
        } else {
            panic!()
        }
    }
}

impl<'ast> Visit<'ast> for GenerateSafeEnums<'ast> {
    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        if let Type::Path(TypePath { path, .. }) = &*item.self_ty {
            if let Some((_, entry)) = self.enums.get_mut(&path.segments[0].ident) {
                for item in item.items.iter() {
                    if let ImplItem::Const(const_) = item {
                        entry.push(&const_.ident)
                    }
                }
            }
        }
    }
    fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
        if let Some((underlying, _)) = self.enums.get_mut(&item.ident) {
            if let Fields::Unnamed(fields) = &item.fields {
                *underlying = Some(&fields.unnamed[0].ty);
            }
        }
    }
}

struct TypeDeclInput {
    enums: Punctuated<Ident, Token![,]>,
}

impl Parse for TypeDeclInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let enums = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(TypeDeclInput { enums })
    }
}

// This macro accepts following arguments:
// * `normal_macro`: ident for a normal macro
// * `override_macro`: ident for an override macro
// * `override_fns`: list of override functions
// Then macro goes through every function in optix6.rs, and for every fn `foo`:
// * if `foo` is contained in `override_fns` then pass it into `override_macro`
// * if `foo` is not contained in `override_fns` pass it to `normal_macro`
// Both macros expected a separated list of OptiX6 functions:
//   macro_foo!(
//      "C" fn rtContextCreate(context: *mut RTcontext) -> RTresult;
//      "C" fn rtContextDestroy(context: RTcontext) -> RTresult
//   )
#[proc_macro]
pub fn optix6_function_declarations(tokens: TokenStream) -> TokenStream {
    let function_decl = parse_macro_input!(tokens as FnDeclInput);
    let override_idents = function_decl
        .override_fns
        .into_iter()
        .collect::<FxHashSet<_>>();
    let optix_module = syn::parse_str::<File>(OPTIX6_RS).unwrap();
    let mut fns = Vec::new();
    let mut override_fns = Vec::new();
    for item in optix_module.items.iter() {
        match item {
            Item::ForeignMod(extern_) => {
                for item in extern_.items.iter() {
                    if let syn::ForeignItem::Fn(fn_) = item {
                        let abi = &extern_.abi.name;
                        let name = &fn_.sig.ident;
                        let inputs = &fn_.sig.inputs;
                        let mut output = &fn_.sig.output;
                        let unit_return = syn::ReturnType::Type(
                            Token![->](Span::call_site()),
                            Box::new(syn::Type::Tuple(syn::TypeTuple {
                                paren_token: syn::token::Paren {
                                    span: Span::call_site(),
                                },
                                elems: Punctuated::new(),
                            })),
                        );
                        if let syn::ReturnType::Default = output {
                            output = &unit_return;
                        }
                        let fn_call = quote! {
                            #abi fn #name(#inputs) #output
                        };
                        let insertion_target = if override_idents.contains(name) {
                            &mut override_fns
                        } else {
                            &mut fns
                        };
                        insertion_target.push(fn_call);
                    }
                }
            }
            _ => {}
        }
    }
    let macro1 = function_decl.normal_macro;
    let macro2 = function_decl.override_macro;
    quote! {
        #macro1 ! ( #(#fns);* );
        #macro2 ! ( #(#override_fns);* );
    }
    .into()
}

struct FnDeclInput {
    normal_macro: Path,
    override_macro: Path,
    override_fns: Punctuated<Ident, Token![,]>,
}

impl Parse for FnDeclInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let normal_macro = input.parse::<Path>()?;
        input.parse::<Token![,]>()?;
        let override_macro = input.parse::<Path>()?;
        input.parse::<Token![,]>()?;
        let override_fns_content;
        bracketed!(override_fns_content in input);
        let override_fns = override_fns_content.parse_terminated(Ident::parse)?;
        Ok(Self {
            normal_macro,
            override_macro,
            override_fns,
        })
    }
}
