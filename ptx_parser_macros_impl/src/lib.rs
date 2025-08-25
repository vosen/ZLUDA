use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    braced, parse::Parse, punctuated::Punctuated, token, Expr, Ident, LitBool, PathSegment, Token,
    Type, TypeParam, Visibility,
};

pub mod parser;

pub struct GenerateInstructionType {
    pub visibility: Option<Visibility>,
    pub name: Ident,
    pub type_parameters: Punctuated<TypeParam, Token![,]>,
    pub short_parameters: Punctuated<Ident, Token![,]>,
    pub variants: Punctuated<InstructionVariant, Token![,]>,
}

impl GenerateInstructionType {
    pub fn emit_arg_types(&self, tokens: &mut TokenStream) {
        for v in self.variants.iter() {
            v.emit_type(&self.visibility, tokens);
        }
    }

    pub fn emit_instruction_type(&self, tokens: &mut TokenStream) {
        let vis = &self.visibility;
        let type_name = &self.name;
        let type_parameters = &self.type_parameters;
        let variants = self.variants.iter().map(|v| v.emit_variant());
        quote! {
            #vis enum #type_name<#type_parameters> {
                #(#variants),*
            }
        }
        .to_tokens(tokens);
    }

    pub fn emit_instruction_display(&self, tokens: &mut TokenStream) {
        let type_name = &self.name;
        let type_parameters = &self.type_parameters;
        let type_arguments = self.type_parameters.iter().map(|p| p.ident.clone());
        let variants = self
            .variants
            .iter()
            .map(|v| v.emit_display(&self.name))
            .filter_map(|v| v);
        quote! {
            impl<#type_parameters> std::fmt::Display for #type_name<#(#type_arguments),*>
            where
                T: std::fmt::Display,
                <T as Operand>::Ident: std::fmt::Display
            {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(#variants),*
                    }
                    Ok(())
                }
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
        let (type_parameters, visitor_parameters, return_type) = if kind == VisitKind::Map {
            (
                quote! { <#type_parameters, To: Operand, Err> },
                quote! { <#short_parameters, To, Err> },
                quote! { std::result::Result<#type_name<To>, Err> },
            )
        } else {
            (
                quote! { <#type_parameters, Err> },
                quote! { <#short_parameters, Err> },
                quote! { std::result::Result<(), Err> },
            )
        };
        quote! {
            pub fn #visit_fn #type_parameters (i: #visit_ref #type_name<#short_parameters>, visitor: &mut impl #visitor_type #visitor_parameters ) -> #return_type {
                Ok(match i {
                    #inner_tokens
                })
            }
        }.to_tokens(tokens);
        if kind == VisitKind::Map {
            return;
        }
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
        let visibility = if !input.peek(Token![enum]) {
            Some(input.parse::<Visibility>()?)
        } else {
            None
        };
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
            visibility,
            name,
            type_parameters,
            short_parameters,
            variants,
        })
    }
}

pub struct InstructionVariant {
    pub name: Ident,
    pub type_: Option<Option<Expr>>,
    pub space: Option<Expr>,
    pub data: Option<Type>,
    pub arguments: Option<Arguments>,
    pub visit: Option<Expr>,
    pub visit_mut: Option<Expr>,
    pub display: Option<Expr>,
    pub map: Option<Expr>,
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
                match &args {
                    Arguments::Def(InstructionArguments { generic: None, .. }) => {
                        quote! {
                            arguments: #args_name,
                        }
                    }
                    Arguments::Def(InstructionArguments {
                        generic: Some(generics),
                        ..
                    }) => {
                        quote! {
                            arguments: #args_name <#generics>,
                        }
                    }
                    Arguments::Decl(type_) => quote! {
                        arguments: #type_,
                    },
                }
            }
        };
        quote! {
            #name { #data #arguments }
        }
    }

    fn emit_display(&self, enum_: &Ident) -> Option<TokenStream> {
        let name = &self.name;
        let enum_ = enum_;
        let arguments = self.arguments.as_ref().map(|_| quote! { arguments });
        let data = &self.data.as_ref().map(|_| quote! { data,});

        let display_op = self
            .display
            .as_ref()
            .map(|d| quote! {#d})
            .unwrap_or(quote! { write!(f, "<{}>", stringify!(#name))? });

        Some(quote! {
            instr @ #enum_ :: #name { #data #arguments } => {
                #display_op;
            }
        })
    }

    fn emit_visit(&self, enum_: &Ident, tokens: &mut TokenStream) {
        self.emit_visit_impl(&self.visit, enum_, tokens, InstructionArguments::emit_visit)
    }

    fn emit_visit_mut(&self, enum_: &Ident, tokens: &mut TokenStream) {
        self.emit_visit_impl(
            &self.visit_mut,
            enum_,
            tokens,
            InstructionArguments::emit_visit_mut,
        )
    }

    fn emit_visit_impl(
        &self,
        visit_fn: &Option<Expr>,
        enum_: &Ident,
        tokens: &mut TokenStream,
        mut fn_: impl FnMut(&InstructionArguments, &Option<Option<Expr>>, &Option<Expr>) -> TokenStream,
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
            Some(Arguments::Decl(_)) => {
                quote! {
                    #enum_ :: #name { data, arguments } => { #visit_fn }
                }
                .to_tokens(tokens);
                return;
            }
            Some(Arguments::Def(args)) => args,
        };
        let data = &self.data.as_ref().map(|_| quote! { data,});
        let arg_calls = fn_(arguments, &self.type_, &self.space);
        quote! {
            #enum_ :: #name { #data arguments } => {
                #arg_calls
            }
        }
        .to_tokens(tokens);
    }

    fn emit_visit_map(&self, enum_: &Ident, tokens: &mut TokenStream) {
        let name = &self.name;
        let data = &self.data.as_ref().map(|_| quote! { data,});
        let arguments = match self.arguments {
            None => None,
            Some(Arguments::Decl(_)) => {
                let map = self.map.as_ref().unwrap();
                quote! {
                    #enum_ :: #name { #data arguments } => {
                        #map
                    }
                }
                .to_tokens(tokens);
                return;
            }
            Some(Arguments::Def(ref def)) => Some(def),
        };
        let arguments_ident = &self.arguments.as_ref().map(|_| quote! { arguments,});
        let mut arg_calls = None;
        let arguments_init = arguments.as_ref().map(|arguments| {
            let arg_type = self.args_name();
            arg_calls = Some(arguments.emit_visit_map(&self.type_, &self.space));
            let arg_names = arguments.fields.iter().map(|arg| &arg.name);
            quote! {
                arguments: #arg_type { #(#arg_names),* }
            }
        });
        quote! {
            #enum_ :: #name { #data #arguments_ident } => {
                #arg_calls
                #enum_ :: #name { #data #arguments_init }
            }
        }
        .to_tokens(tokens);
    }

    fn emit_type(&self, vis: &Option<Visibility>, tokens: &mut TokenStream) {
        let arguments = match self.arguments {
            Some(Arguments::Def(ref a)) => a,
            Some(Arguments::Decl(_)) => return,
            None => return,
        };
        let name = self.args_name();
        let type_parameters = if arguments.generic.is_some() {
            Some(quote! { <T> })
        } else {
            None
        };
        let fields = arguments.fields.iter().map(|f| f.emit_field(vis));
        quote! {
            #vis struct #name #type_parameters {
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
        let mut visit = None;
        let mut visit_mut = None;
        let mut display = None;
        let mut map = None;
        for property in properties {
            match property {
                VariantProperty::Type(t) => type_ = Some(t),
                VariantProperty::Space(s) => space = Some(s),
                VariantProperty::Data(d) => data = Some(d),
                VariantProperty::Arguments(a) => arguments = Some(a),
                VariantProperty::Visit(e) => visit = Some(e),
                VariantProperty::VisitMut(e) => visit_mut = Some(e),
                VariantProperty::Display(e) => display = Some(e),
                VariantProperty::Map(e) => map = Some(e),
            }
        }
        Ok(Self {
            name,
            type_,
            space,
            data,
            arguments,
            visit,
            visit_mut,
            display,
            map,
        })
    }
}

enum VariantProperty {
    Type(Option<Expr>),
    Space(Expr),
    Data(Type),
    Arguments(Arguments),
    Visit(Expr),
    VisitMut(Expr),
    Display(Expr),
    Map(Expr),
}

impl VariantProperty {
    pub fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        Ok(if lookahead.peek(Token![type]) {
            input.parse::<Token![type]>()?;
            input.parse::<Token![:]>()?;
            VariantProperty::Type(if input.peek(Token![!]) {
                input.parse::<Token![!]>()?;
                None
            } else {
                Some(input.parse::<Expr>()?)
            })
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
                            Punctuated::<PathSegment, syn::token::PathSep>::parse_separated_nonempty(input)?;
                        input.parse::<Token![>]>()?;
                        Some(gen_params)
                    } else {
                        None
                    };
                    input.parse::<Token![:]>()?;
                    if input.peek(token::Brace) {
                        let fields;
                        braced!(fields in input);
                        VariantProperty::Arguments(Arguments::Def(InstructionArguments::parse(
                            generics, &fields,
                        )?))
                    } else {
                        VariantProperty::Arguments(Arguments::Decl(input.parse::<Type>()?))
                    }
                }
                "visit" => {
                    input.parse::<Token![:]>()?;
                    VariantProperty::Visit(input.parse::<Expr>()?)
                }
                "visit_mut" => {
                    input.parse::<Token![:]>()?;
                    VariantProperty::VisitMut(input.parse::<Expr>()?)
                }
                "display" => {
                    input.parse::<Token![:]>()?;
                    VariantProperty::Display(input.parse::<Expr>()?)
                }
                "map" => {
                    input.parse::<Token![:]>()?;
                    VariantProperty::Map(input.parse::<Expr>()?)
                }
                x => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!(
                            "Unexpected key `{}`. Expected `type`, `data`, `arguments`, `visit, `visit_mut` or `map`.",
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

pub enum Arguments {
    Decl(Type),
    Def(InstructionArguments),
}

pub struct InstructionArguments {
    pub generic: Option<Punctuated<PathSegment, syn::token::PathSep>>,
    pub fields: Punctuated<ArgumentField, Token![,]>,
}

impl InstructionArguments {
    pub fn parse(
        generic: Option<Punctuated<PathSegment, syn::token::PathSep>>,
        input: syn::parse::ParseStream,
    ) -> syn::Result<Self> {
        let fields = Punctuated::<ArgumentField, Token![,]>::parse_terminated_with(
            input,
            ArgumentField::parse,
        )?;
        Ok(Self { generic, fields })
    }

    fn emit_visit(
        &self,
        parent_type: &Option<Option<Expr>>,
        parent_space: &Option<Expr>,
    ) -> TokenStream {
        self.emit_visit_impl(parent_type, parent_space, ArgumentField::emit_visit)
    }

    fn emit_visit_mut(
        &self,
        parent_type: &Option<Option<Expr>>,
        parent_space: &Option<Expr>,
    ) -> TokenStream {
        self.emit_visit_impl(parent_type, parent_space, ArgumentField::emit_visit_mut)
    }

    fn emit_visit_map(
        &self,
        parent_type: &Option<Option<Expr>>,
        parent_space: &Option<Expr>,
    ) -> TokenStream {
        self.emit_visit_impl(parent_type, parent_space, ArgumentField::emit_visit_map)
    }

    fn emit_visit_impl(
        &self,
        parent_type: &Option<Option<Expr>>,
        parent_space: &Option<Expr>,
        mut fn_: impl FnMut(&ArgumentField, &Option<Option<Expr>>, &Option<Expr>, bool) -> TokenStream,
    ) -> TokenStream {
        let is_ident = if let Some(ref generic) = self.generic {
            generic.len() > 1
        } else {
            false
        };
        let field_calls = self
            .fields
            .iter()
            .map(|f| fn_(f, parent_type, parent_space, is_ident));
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
    pub relaxed_type_check: bool,
}

impl ArgumentField {
    fn parse_block(
        input: syn::parse::ParseStream,
    ) -> syn::Result<(Type, Option<Expr>, Option<Expr>, Option<bool>, bool)> {
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
                        "relaxed_type_check" => {
                            ExprOrPath::RelaxedTypeCheck(content.parse::<LitBool>()?.value)
                        }
                        "repr" => ExprOrPath::Repr(content.parse::<Type>()?),
                        "space" => ExprOrPath::Space(content.parse::<Expr>()?),
                        "dst" => {
                            let ident = content.parse::<LitBool>()?;
                            ExprOrPath::Dst(ident.value)
                        }
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
        let mut is_dst = None;
        let mut relaxed_type_check = false;
        for exp_or_path in all_fields {
            match exp_or_path {
                ExprOrPath::Repr(r) => repr = Some(r),
                ExprOrPath::Type(t) => type_ = Some(t),
                ExprOrPath::Space(s) => space = Some(s),
                ExprOrPath::Dst(x) => is_dst = Some(x),
                ExprOrPath::RelaxedTypeCheck(relaxed) => relaxed_type_check = relaxed,
            }
        }
        Ok((repr.unwrap(), type_, space, is_dst, relaxed_type_check))
    }

    fn parse_basic(input: &syn::parse::ParseBuffer) -> syn::Result<Type> {
        input.parse::<Type>()
    }

    fn emit_visit(
        &self,
        parent_type: &Option<Option<Expr>>,
        parent_space: &Option<Expr>,
        is_ident: bool,
    ) -> TokenStream {
        self.emit_visit_impl(parent_type, parent_space, is_ident, false)
    }

    fn emit_visit_mut(
        &self,
        parent_type: &Option<Option<Expr>>,
        parent_space: &Option<Expr>,
        is_ident: bool,
    ) -> TokenStream {
        self.emit_visit_impl(parent_type, parent_space, is_ident, true)
    }

    fn emit_visit_impl(
        &self,
        parent_type: &Option<Option<Expr>>,
        parent_space: &Option<Expr>,
        is_ident: bool,
        is_mut: bool,
    ) -> TokenStream {
        let (is_typeless, type_) = match (self.type_.as_ref(), parent_type) {
            (Some(type_), _) => (false, Some(type_)),
            (None, None) => panic!("No type set"),
            (None, Some(None)) => (true, None),
            (None, Some(Some(type_))) => (false, Some(type_)),
        };
        let space = self
            .space
            .as_ref()
            .or(parent_space.as_ref())
            .map(|space| quote! { #space })
            .unwrap_or_else(|| quote! { StateSpace::Reg });
        let is_dst = self.is_dst;
        let relaxed_type_check = self.relaxed_type_check;
        let name = &self.name;
        let type_space = if is_typeless {
            quote! {
                let type_space = None;
            }
        } else {
            quote! {
                let type_ = #type_;
                let space = #space;
                let type_space = Some((std::borrow::Borrow::<Type>::borrow(&type_), space));
            }
        };
        if is_ident {
            if is_mut {
                quote! {
                    {
                        #type_space
                        visitor.visit_ident(&mut arguments.#name, type_space, #is_dst, #relaxed_type_check)?;
                    }
                }
            } else {
                quote! {
                    {
                        #type_space
                        visitor.visit_ident(& arguments.#name, type_space, #is_dst, #relaxed_type_check)?;
                    }
                }
            }
        } else {
            let (operand_fn, arguments_name) = if is_mut {
                (
                    quote! {
                        VisitOperand::visit_mut
                    },
                    quote! {
                        &mut arguments.#name
                    },
                )
            } else {
                (
                    quote! {
                        VisitOperand::visit
                    },
                    quote! {
                        & arguments.#name
                    },
                )
            };
            quote! {{
                #type_space
                #operand_fn(#arguments_name, |x| visitor.visit(x, type_space, #is_dst, #relaxed_type_check))?;
            }}
        }
    }

    fn emit_visit_map(
        &self,
        parent_type: &Option<Option<Expr>>,
        parent_space: &Option<Expr>,
        is_ident: bool,
    ) -> TokenStream {
        let (is_typeless, type_) = match (self.type_.as_ref(), parent_type) {
            (Some(type_), _) => (false, Some(type_)),
            (None, None) => panic!("No type set"),
            (None, Some(None)) => (true, None),
            (None, Some(Some(type_))) => (false, Some(type_)),
        };
        let space = self
            .space
            .as_ref()
            .or(parent_space.as_ref())
            .map(|space| quote! { #space })
            .unwrap_or_else(|| quote! { StateSpace::Reg });
        let is_dst = self.is_dst;
        let relaxed_type_check = self.relaxed_type_check;
        let name = &self.name;
        let type_space = if is_typeless {
            quote! {
                let type_space = None;
            }
        } else {
            quote! {
                let type_ = #type_;
                let space = #space;
                let type_space = Some((std::borrow::Borrow::<Type>::borrow(&type_), space));
            }
        };
        let map_call = if is_ident {
            quote! {
                visitor.visit_ident(arguments.#name, type_space, #is_dst, #relaxed_type_check)?
            }
        } else {
            quote! {
                MapOperand::map(arguments.#name, |x| visitor.visit(x, type_space, #is_dst, #relaxed_type_check))?
            }
        };
        quote! {
            let #name = {
                #type_space
                #map_call
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

    fn emit_field(&self, vis: &Option<Visibility>) -> TokenStream {
        let name = &self.name;
        let type_ = &self.repr;
        quote! {
            #vis #name: #type_
        }
    }
}

impl Parse for ArgumentField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;

        input.parse::<Token![:]>()?;
        let lookahead = input.lookahead1();
        let (repr, type_, space, is_dst, relaxed_type_check) = if lookahead.peek(token::Brace) {
            Self::parse_block(input)?
        } else if lookahead.peek(syn::Ident) {
            (Self::parse_basic(input)?, None, None, None, false)
        } else {
            return Err(lookahead.error());
        };
        let is_dst = match is_dst {
            Some(x) => x,
            None => Self::is_dst(&name)?,
        };
        Ok(Self {
            name,
            is_dst,
            repr,
            type_,
            space,
            relaxed_type_check,
        })
    }
}

enum ExprOrPath {
    Repr(Type),
    Type(Expr),
    Space(Expr),
    Dst(bool),
    RelaxedTypeCheck(bool),
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
        let arguments = if let Some(Arguments::Def(a)) = variant.arguments {
            a
        } else {
            panic!()
        };
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
