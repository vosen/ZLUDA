use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    braced, parse::Parse, punctuated::Punctuated, token, Expr, Ident, Token, Type, TypeParam,
};

pub mod parser;

pub struct GenerateInstructionType {
    pub name: Ident,
    pub type_parameters: Punctuated<TypeParam, Token![,]>,
    pub short_parameters: Punctuated<Ident, Token![,]>,
    pub variants: Punctuated<InstructionVariant, Token![,]>,
}

impl GenerateInstructionType {
    pub fn emit_arg_types(&self, tokens: &mut TokenStream) {
        for v in self.variants.iter() {
            v.emit_type(&self.type_parameters, tokens);
        }
    }

    pub fn emit_instruction_type(&self, tokens: &mut TokenStream) {
        let type_name = &self.name;
        let type_parameters = &self.type_parameters;
        let variants = self.variants.iter().map(|v| v.emit_variant());
        quote! {
            enum #type_name<#type_parameters> {
                #(#variants),*
            }
        }
        .to_tokens(tokens);
    }

    pub fn emit_visit(&self, tokens: &mut TokenStream) {
        self.emit_visit_impl(VisitKind::Ref, tokens, InstructionVariant::emit_visit)
    }

    pub fn emit_visit_mut(&self, tokens: &mut TokenStream) {
        self.emit_visit_impl(
            VisitKind::RefMut,
            tokens,
            InstructionVariant::emit_visit_mut,
        )
    }

    pub fn emit_visit_map(&self, tokens: &mut TokenStream) {
        self.emit_visit_impl(VisitKind::Map, tokens, InstructionVariant::emit_visit_map)
    }

    fn emit_visit_impl(
        &self,
        kind: VisitKind,
        tokens: &mut TokenStream,
        mut fn_: impl FnMut(&InstructionVariant, &Ident, &mut TokenStream),
    ) {
        let type_name = &self.name;
        let type_parameters = &self.type_parameters;
        let short_parameters = &self.short_parameters;
        let mut inner_tokens = TokenStream::new();
        for v in self.variants.iter() {
            fn_(v, type_name, &mut inner_tokens);
        }
        let visit_ref = kind.reference();
        let visitor_type = format_ident!("Visitor{}", kind.type_suffix());
        let visit_fn = format_ident!("visit{}", kind.fn_suffix());
        let visit_slice_fn = format_ident!("visit{}_slice", kind.fn_suffix());
        let (type_parameters, visitor_parameters, return_type) = if kind == VisitKind::Map {
            (
                quote! { <#type_parameters, To: Operand> },
                quote! { <#short_parameters, To> },
                quote! { #type_name<To> },
            )
        } else {
            (
                quote! { <#type_parameters> },
                quote! { <#short_parameters> },
                quote! { () },
            )
        };
        quote! {
            fn #visit_fn #type_parameters (i: #visit_ref #type_name<#short_parameters>, visitor: &mut impl #visitor_type #visitor_parameters ) -> #return_type {
                match i {
                    #inner_tokens
                }
            }
        }.to_tokens(tokens);
        if kind == VisitKind::Map {
            return;
        }
        quote! {
            fn #visit_slice_fn #type_parameters (instructions: #visit_ref [#type_name<#short_parameters>], visitor: &mut impl #visitor_type #visitor_parameters) {
                for i in instructions {
                    #visit_fn(i, visitor)
                }
            }
        }.to_tokens(tokens);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum VisitKind {
    Ref,
    RefMut,
    Map,
}

impl VisitKind {
    fn fn_suffix(self) -> &'static str {
        match self {
            VisitKind::Ref => "",
            VisitKind::RefMut => "_mut",
            VisitKind::Map => "_map",
        }
    }

    fn type_suffix(self) -> &'static str {
        match self {
            VisitKind::Ref => "",
            VisitKind::RefMut => "Mut",
            VisitKind::Map => "Map",
        }
    }

    fn reference(self) -> Option<proc_macro2::TokenStream> {
        match self {
            VisitKind::Ref => Some(quote! { & }),
            VisitKind::RefMut => Some(quote! { &mut }),
            VisitKind::Map => None,
        }
    }
}

impl Parse for GenerateInstructionType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![enum]>()?;
        let name = input.parse::<Ident>()?;
        input.parse::<Token![<]>()?;
        let type_parameters = Punctuated::parse_separated_nonempty(input)?;
        let short_parameters = type_parameters
            .iter()
            .map(|p: &TypeParam| p.ident.clone())
            .collect();
        input.parse::<Token![>]>()?;
        let variants_buffer;
        braced!(variants_buffer in input);
        let variants = variants_buffer.parse_terminated(InstructionVariant::parse, Token![,])?;
        Ok(Self {
            name,
            type_parameters,
            short_parameters,
            variants,
        })
    }
}

pub struct InstructionVariant {
    pub name: Ident,
    pub type_: Option<Expr>,
    pub space: Option<Expr>,
    pub data: Option<Type>,
    pub arguments: Option<InstructionArguments>,
}

impl InstructionVariant {
    fn args_name(&self) -> Ident {
        format_ident!("{}Args", self.name)
    }

    fn emit_variant(&self) -> TokenStream {
        let name = &self.name;
        let data = match &self.data {
            None => {
                quote! {}
            }
            Some(data_type) => {
                quote! {
                    data: #data_type,
                }
            }
        };
        let arguments = match &self.arguments {
            None => {
                quote! {}
            }
            Some(args) => {
                let args_name = self.args_name();
                match &args.generic {
                    None => {
                        quote! {
                            arguments: #args_name,
                        }
                    }
                    Some(generics) => {
                        quote! {
                            arguments: #args_name <#generics>,
                        }
                    }
                }
            }
        };
        quote! {
            #name { #data #arguments }
        }
    }

    fn emit_visit(&self, enum_: &Ident, tokens: &mut TokenStream) {
        self.emit_visit_impl(enum_, tokens, InstructionArguments::emit_visit)
    }

    fn emit_visit_mut(&self, enum_: &Ident, tokens: &mut TokenStream) {
        self.emit_visit_impl(enum_, tokens, InstructionArguments::emit_visit_mut)
    }

    fn emit_visit_impl(
        &self,
        enum_: &Ident,
        tokens: &mut TokenStream,
        mut fn_: impl FnMut(&InstructionArguments, &Option<Expr>, &Option<Expr>) -> TokenStream,
    ) {
        let name = &self.name;
        let arguments = match &self.arguments {
            None => {
                quote! {
                    #enum_ :: #name { .. } => { }
                }
                .to_tokens(tokens);
                return;
            }
            Some(args) => args,
        };
        let arg_calls = fn_(arguments, &self.type_, &self.space);
        quote! {
            #enum_ :: #name { arguments, data } => {
                #arg_calls
            }
        }
        .to_tokens(tokens);
    }

    fn emit_visit_map(&self, enum_: &Ident, tokens: &mut TokenStream) {
        let name = &self.name;
        let arguments = &self.arguments.as_ref().map(|_| quote! { arguments,});
        let data = &self.data.as_ref().map(|_| quote! { data,});
        let mut arg_calls = None;
        let arguments_init = self.arguments.as_ref().map(|arguments| {
            let arg_type = self.args_name();
            arg_calls = Some(arguments.emit_visit_map(&self.type_, &self.space));
            let arg_names = arguments.fields.iter().map(|arg| &arg.name);
            quote! {
                arguments: #arg_type { #(#arg_names),* }
            }
        });
        quote! {
            #enum_ :: #name { #data #arguments } => {
                #arg_calls
                #enum_ :: #name { #data #arguments_init }
            }
        }
        .to_tokens(tokens);
    }

    fn emit_type(
        &self,
        type_parameters: &Punctuated<TypeParam, Token![,]>,
        tokens: &mut TokenStream,
    ) {
        let arguments = match self.arguments {
            Some(ref a) => a,
            None => return,
        };
        let name = self.args_name();
        let type_parameters = if arguments.generic.is_some() {
            Some(quote! { <#type_parameters> })
        } else {
            None
        };
        let fields = arguments.fields.iter().map(ArgumentField::emit_field);
        quote! {
            struct #name #type_parameters {
                #(#fields),*
            }
        }
        .to_tokens(tokens);
    }
}

impl Parse for InstructionVariant {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let properties_buffer;
        braced!(properties_buffer in input);
        let properties = properties_buffer.parse_terminated(VariantProperty::parse, Token![,])?;
        let mut type_ = None;
        let mut space = None;
        let mut data = None;
        let mut arguments = None;
        for property in properties {
            match property {
                VariantProperty::Type(t) => type_ = Some(t),
                VariantProperty::Space(s) => space = Some(s),
                VariantProperty::Data(d) => data = Some(d),
                VariantProperty::Arguments(a) => arguments = Some(a),
            }
        }
        Ok(Self {
            name,
            type_,
            space,
            data,
            arguments,
        })
    }
}

enum VariantProperty {
    Type(Expr),
    Space(Expr),
    Data(Type),
    Arguments(InstructionArguments),
}

impl VariantProperty {
    pub fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        Ok(if lookahead.peek(Token![type]) {
            input.parse::<Token![type]>()?;
            input.parse::<Token![:]>()?;
            VariantProperty::Type(input.parse::<Expr>()?)
        } else if lookahead.peek(Ident) {
            let key = input.parse::<Ident>()?;
            match &*key.to_string() {
                "data" => {
                    input.parse::<Token![:]>()?;
                    VariantProperty::Data(input.parse::<Type>()?)
                }
                "space" => {
                    input.parse::<Token![:]>()?;
                    VariantProperty::Space(input.parse::<Expr>()?)
                }
                "arguments" => {
                    let generics = if input.peek(Token![<]) {
                        input.parse::<Token![<]>()?;
                        let gen_params =
                            Punctuated::<TypeParam, Token![,]>::parse_separated_nonempty(input)?;
                        input.parse::<Token![>]>()?;
                        Some(gen_params)
                    } else {
                        None
                    };
                    input.parse::<Token![:]>()?;
                    let fields;
                    braced!(fields in input);
                    VariantProperty::Arguments(InstructionArguments::parse(generics, &fields)?)
                }
                x => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!(
                            "Unexpected key `{}`. Expected `type`, `data` or `arguments`.",
                            x
                        ),
                    ))
                }
            }
        } else {
            return Err(lookahead.error());
        })
    }
}

pub struct InstructionArguments {
    pub generic: Option<Punctuated<TypeParam, Token![,]>>,
    pub fields: Punctuated<ArgumentField, Token![,]>,
}

impl InstructionArguments {
    pub fn parse(
        generic: Option<Punctuated<TypeParam, Token![,]>>,
        input: syn::parse::ParseStream,
    ) -> syn::Result<Self> {
        let fields = Punctuated::<ArgumentField, Token![,]>::parse_terminated_with(
            input,
            ArgumentField::parse,
        )?;
        Ok(Self { generic, fields })
    }

    fn emit_visit(&self, parent_type: &Option<Expr>, parent_space: &Option<Expr>) -> TokenStream {
        self.emit_visit_impl(parent_type, parent_space, ArgumentField::emit_visit)
    }

    fn emit_visit_mut(
        &self,
        parent_type: &Option<Expr>,
        parent_space: &Option<Expr>,
    ) -> TokenStream {
        self.emit_visit_impl(parent_type, parent_space, ArgumentField::emit_visit_mut)
    }

    fn emit_visit_map(
        &self,
        parent_type: &Option<Expr>,
        parent_space: &Option<Expr>,
    ) -> TokenStream {
        self.emit_visit_impl(parent_type, parent_space, ArgumentField::emit_visit_map)
    }

    fn emit_visit_impl(
        &self,
        parent_type: &Option<Expr>,
        parent_space: &Option<Expr>,
        mut fn_: impl FnMut(&ArgumentField, &Option<Expr>, &Option<Expr>) -> TokenStream,
    ) -> TokenStream {
        let field_calls = self
            .fields
            .iter()
            .map(|f| fn_(f, parent_type, parent_space));
        quote! {
            #(#field_calls)*
        }
    }
}

pub struct ArgumentField {
    pub name: Ident,
    pub is_dst: bool,
    pub repr: Type,
    pub space: Option<Expr>,
    pub type_: Option<Expr>,
}

impl ArgumentField {
    fn parse_block(
        input: syn::parse::ParseStream,
    ) -> syn::Result<(Type, Option<Expr>, Option<Expr>)> {
        let content;
        braced!(content in input);
        let all_fields =
            Punctuated::<ExprOrPath, Token![,]>::parse_terminated_with(&content, |content| {
                let lookahead = content.lookahead1();
                Ok(if lookahead.peek(Token![type]) {
                    content.parse::<Token![type]>()?;
                    content.parse::<Token![:]>()?;
                    ExprOrPath::Type(content.parse::<Expr>()?)
                } else if lookahead.peek(Ident) {
                    let name_ident = content.parse::<Ident>()?;
                    content.parse::<Token![:]>()?;
                    match &*name_ident.to_string() {
                        "repr" => ExprOrPath::Repr(content.parse::<Type>()?),
                        "space" => ExprOrPath::Space(content.parse::<Expr>()?),
                        name => {
                            return Err(syn::Error::new(
                                name_ident.span(),
                                format!("Unexpected key `{}`, expected `repr` or `space", name),
                            ))
                        }
                    }
                } else {
                    return Err(lookahead.error());
                })
            })?;
        let mut repr = None;
        let mut type_ = None;
        let mut space = None;
        for exp_or_path in all_fields {
            match exp_or_path {
                ExprOrPath::Repr(r) => repr = Some(r),
                ExprOrPath::Type(t) => type_ = Some(t),
                ExprOrPath::Space(s) => space = Some(s),
            }
        }
        Ok((repr.unwrap(), type_, space))
    }

    fn parse_basic(input: &syn::parse::ParseBuffer) -> syn::Result<Type> {
        input.parse::<Type>()
    }

    fn emit_visit(&self, parent_type: &Option<Expr>, parent_space: &Option<Expr>) -> TokenStream {
        self.emit_visit_impl(parent_type, parent_space, false)
    }

    fn emit_visit_mut(
        &self,
        parent_type: &Option<Expr>,
        parent_space: &Option<Expr>,
    ) -> TokenStream {
        self.emit_visit_impl(parent_type, parent_space, true)
    }

    fn emit_visit_impl(
        &self,
        parent_type: &Option<Expr>,
        parent_space: &Option<Expr>,
        is_mut: bool,
    ) -> TokenStream {
        let type_ = self.type_.as_ref().or(parent_type.as_ref()).unwrap();
        let space = self
            .space
            .as_ref()
            .or(parent_space.as_ref())
            .map(|space| quote! { #space })
            .unwrap_or_else(|| quote! { StateSpace::Reg });
        let is_dst = self.is_dst;
        let name = &self.name;
        let arguments_name = if is_mut {
            quote! {
                &mut arguments.#name
            }
        } else {
            quote! {
                & arguments.#name
            }
        };
        quote! {{
            let type_ = #type_;
            let space = #space;
            visitor.visit(#arguments_name, &type_, space, #is_dst);
        }}
    }

    fn emit_visit_map(
        &self,
        parent_type: &Option<Expr>,
        parent_space: &Option<Expr>,
    ) -> TokenStream {
        let type_ = self.type_.as_ref().or(parent_type.as_ref()).unwrap();
        let space = self
            .space
            .as_ref()
            .or(parent_space.as_ref())
            .map(|space| quote! { #space })
            .unwrap_or_else(|| quote! { StateSpace::Reg });
        let is_dst = self.is_dst;
        let name = &self.name;
        quote! {
            let #name = {
                let type_ = #type_;
                let space = #space;
                visitor.visit(arguments.#name, &type_, space, #is_dst)
            };
        }
    }

    fn is_dst(name: &Ident) -> syn::Result<bool> {
        if name.to_string().starts_with("dst") {
            Ok(true)
        } else if name.to_string().starts_with("src") {
            Ok(false)
        } else {
            return Err(syn::Error::new(
                name.span(),
                format!(
                    "Could not guess if `{}` is a read or write argument. Name should start with `dst` or `src`",
                    name
                ),
            ));
        }
    }

    fn emit_field(&self) -> TokenStream {
        let name = &self.name;
        let type_ = &self.repr;
        quote! {
            #name: #type_
        }
    }
}

impl Parse for ArgumentField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let is_dst = Self::is_dst(&name)?;
        input.parse::<Token![:]>()?;
        let lookahead = input.lookahead1();
        let (repr, type_, space) = if lookahead.peek(token::Brace) {
            Self::parse_block(input)?
        } else if lookahead.peek(syn::Ident) {
            (Self::parse_basic(input)?, None, None)
        } else {
            return Err(lookahead.error());
        };
        Ok(Self {
            name,
            is_dst,
            repr,
            type_,
            space,
        })
    }
}

enum ExprOrPath {
    Repr(Type),
    Type(Expr),
    Space(Expr),
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;
    use quote::{quote, ToTokens};

    fn to_string(x: impl ToTokens) -> String {
        quote! { #x }.to_string()
    }

    #[test]
    fn parse_argument_field_basic() {
        let input = quote! {
            dst: P::Operand
        };
        let arg = syn::parse2::<ArgumentField>(input).unwrap();
        assert_eq!("dst", arg.name.to_string());
        assert_eq!("P :: Operand", to_string(arg.repr));
        assert!(matches!(arg.type_, None));
    }

    #[test]
    fn parse_argument_field_block() {
        let input = quote! {
            dst: {
                type: ScalarType::U32,
                space: StateSpace::Global,
                repr: P::Operand,
            }
        };
        let arg = syn::parse2::<ArgumentField>(input).unwrap();
        assert_eq!("dst", arg.name.to_string());
        assert_eq!("ScalarType :: U32", to_string(arg.type_.unwrap()));
        assert_eq!("StateSpace :: Global", to_string(arg.space.unwrap()));
        assert_eq!("P :: Operand", to_string(arg.repr));
    }

    #[test]
    fn parse_argument_field_block_untyped() {
        let input = quote! {
            dst: {
                repr: P::Operand,
            }
        };
        let arg = syn::parse2::<ArgumentField>(input).unwrap();
        assert_eq!("dst", arg.name.to_string());
        assert_eq!("P :: Operand", to_string(arg.repr));
        assert!(matches!(arg.type_, None));
    }

    #[test]
    fn parse_variant_complex() {
        let input = quote! {
            Ld {
                type: ScalarType::U32,
                space: StateSpace::Global,
                data: LdDetails,
                arguments<P>: {
                    dst: {
                        repr: P::Operand,
                        type: ScalarType::U32,
                        space: StateSpace::Shared,
                    },
                    src: P::Operand,
                },
            }
        };
        let variant = syn::parse2::<InstructionVariant>(input).unwrap();
        assert_eq!("Ld", variant.name.to_string());
        assert_eq!("ScalarType :: U32", to_string(variant.type_.unwrap()));
        assert_eq!("StateSpace :: Global", to_string(variant.space.unwrap()));
        assert_eq!("LdDetails", to_string(variant.data.unwrap()));
        let arguments = variant.arguments.unwrap();
        assert_eq!("P", to_string(arguments.generic));
        let mut fields = arguments.fields.into_iter();
        let dst = fields.next().unwrap();
        assert_eq!("P :: Operand", to_string(dst.repr));
        assert_eq!("ScalarType :: U32", to_string(dst.type_));
        assert_eq!("StateSpace :: Shared", to_string(dst.space));
        let src = fields.next().unwrap();
        assert_eq!("P :: Operand", to_string(src.repr));
        assert!(matches!(src.type_, None));
        assert!(matches!(src.space, None));
    }

    #[test]
    fn visit_variant() {
        let input = quote! {
            Ld {
                type: ScalarType::U32,
                data: LdDetails,
                arguments<P>: {
                    dst: {
                        repr: P::Operand,
                        type: ScalarType::U32
                    },
                    src: P::Operand,
                },
            }
        };
        let variant = syn::parse2::<InstructionVariant>(input).unwrap();
        let mut output = TokenStream::new();
        variant.emit_visit(&Ident::new("Instruction", Span::call_site()), &mut output);
        assert_eq!(output.to_string(), "Instruction :: Ld { arguments , data } => { { let type_ = ScalarType :: U32 ; let space = StateSpace :: Reg ; visitor . visit (& arguments . dst , & type_ , space , true) ; } { let type_ = ScalarType :: U32 ; let space = StateSpace :: Reg ; visitor . visit (& arguments . src , & type_ , space , false) ; } }");
    }

    #[test]
    fn visit_variant_empty() {
        let input = quote! {
            Ret {
                data: RetData
            }
        };
        let variant = syn::parse2::<InstructionVariant>(input).unwrap();
        let mut output = TokenStream::new();
        variant.emit_visit(&Ident::new("Instruction", Span::call_site()), &mut output);
        assert_eq!(output.to_string(), "Instruction :: Ret { .. } => { }");
    }
}
