use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use rustc_hash::FxHashMap;
use std::fmt::Write;
use syn::bracketed;
use syn::parse::Peek;
use syn::punctuated::Punctuated;
use syn::LitInt;
use syn::Type;
use syn::{braced, parse::Parse, token, Ident, ItemEnum, Token};

pub struct ParseDefinitions {
    pub token_type: ItemEnum,
    pub additional_enums: FxHashMap<Ident, ItemEnum>,
    pub definitions: Vec<OpcodeDefinition>,
}

impl Parse for ParseDefinitions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let token_type = input.parse::<ItemEnum>()?;
        let mut additional_enums = FxHashMap::default();
        while input.peek(Token![#]) {
            let enum_ = input.parse::<ItemEnum>()?;
            additional_enums.insert(enum_.ident.clone(), enum_);
        }
        let mut definitions = Vec::new();
        while !input.is_empty() {
            definitions.push(input.parse::<OpcodeDefinition>()?);
        }
        Ok(Self {
            token_type,
            additional_enums,
            definitions,
        })
    }
}

pub struct OpcodeDefinition(pub Patterns, pub Vec<Rule>);

impl Parse for OpcodeDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let patterns = input.parse::<Patterns>()?;
        let mut rules = Vec::new();
        while Rule::peek(input) {
            rules.push(input.parse::<Rule>()?);
            input.parse::<Token![;]>()?;
        }
        Ok(Self(patterns, rules))
    }
}

pub struct Patterns(pub Vec<(OpcodeDecl, CodeBlock)>);

impl Parse for Patterns {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut result = Vec::new();
        loop {
            if !OpcodeDecl::peek(input) {
                break;
            }
            let decl = input.parse::<OpcodeDecl>()?;
            let code_block = input.parse::<CodeBlock>()?;
            result.push((decl, code_block))
        }
        Ok(Self(result))
    }
}

pub struct OpcodeDecl(pub Instruction, pub Arguments);

impl OpcodeDecl {
    fn peek(input: syn::parse::ParseStream) -> bool {
        Instruction::peek(input)
    }
}

impl Parse for OpcodeDecl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(
            input.parse::<Instruction>()?,
            input.parse::<Arguments>()?,
        ))
    }
}

pub struct CodeBlock(pub proc_macro2::Group);

impl Parse for CodeBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![=>]>()?;
        let group = input.parse::<proc_macro2::Group>()?;
        Ok(Self(group))
    }
}

pub struct Rule {
    pub modifier: DotModifier,
    pub type_: Option<Type>,
    pub alternatives: Vec<DotModifier>,
}

impl Rule {
    fn peek(input: syn::parse::ParseStream) -> bool {
        DotModifier::peek(input)
    }

    fn parse_alternatives(input: syn::parse::ParseStream) -> syn::Result<Vec<DotModifier>> {
        let mut result = Vec::new();
        Self::parse_with_alternative(input, &mut result)?;
        loop {
            if !input.peek(Token![,]) {
                break;
            }
            input.parse::<Token![,]>()?;
            Self::parse_with_alternative(input, &mut result)?;
        }
        Ok(result)
    }

    fn parse_with_alternative(
        input: &syn::parse::ParseBuffer,
        result: &mut Vec<DotModifier>,
    ) -> Result<(), syn::Error> {
        input.parse::<Token![.]>()?;
        let part1 = input.parse::<IdentLike>()?;
        if input.peek(token::Brace) {
            result.push(DotModifier {
                part1: part1.clone(),
                part2: None,
            });
            let suffix_content;
            braced!(suffix_content in input);
            let suffixes = Punctuated::<IdentOrTypeSuffix, Token![,]>::parse_separated_nonempty(
                &suffix_content,
            )?;
            for part2 in suffixes {
                result.push(DotModifier {
                    part1: part1.clone(),
                    part2: Some(part2),
                });
            }
        } else if IdentOrTypeSuffix::peek(input) {
            let part2 = Some(IdentOrTypeSuffix::parse(input)?);
            result.push(DotModifier { part1, part2 });
        } else {
            result.push(DotModifier { part1, part2: None });
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct IdentOrTypeSuffix(IdentLike);

impl IdentOrTypeSuffix {
    fn peek(input: syn::parse::ParseStream) -> bool {
        input.peek(Token![::])
    }
}

impl ToTokens for IdentOrTypeSuffix {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.0;
        quote! { :: #ident }.to_tokens(tokens)
    }
}

impl Parse for IdentOrTypeSuffix {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![::]>()?;
        Ok(Self(input.parse::<IdentLike>()?))
    }
}

impl Parse for Rule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let modifier = input.parse::<DotModifier>()?;
        let type_ = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            Some(input.parse::<Type>()?)
        } else {
            None
        };
        input.parse::<Token![=]>()?;
        let content;
        braced!(content in input);
        let alternatives = Self::parse_alternatives(&content)?;
        Ok(Self {
            modifier,
            type_,
            alternatives,
        })
    }
}

pub struct Instruction {
    pub name: Ident,
    pub modifiers: Vec<MaybeDotModifier>,
}
impl Instruction {
    fn peek(input: syn::parse::ParseStream) -> bool {
        input.peek(Ident)
    }
}

impl Parse for Instruction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let instruction = input.parse::<Ident>()?;
        let mut modifiers = Vec::new();
        loop {
            if !MaybeDotModifier::peek(input) {
                break;
            }
            modifiers.push(MaybeDotModifier::parse(input)?);
        }
        Ok(Self {
            name: instruction,
            modifiers,
        })
    }
}

pub struct MaybeDotModifier {
    pub optional: bool,
    pub modifier: DotModifier,
}

impl MaybeDotModifier {
    fn peek(input: syn::parse::ParseStream) -> bool {
        input.peek(token::Brace) || DotModifier::peek(input)
    }
}

impl Parse for MaybeDotModifier {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(if input.peek(token::Brace) {
            let content;
            braced!(content in input);
            let modifier = DotModifier::parse(&content)?;
            Self {
                modifier,
                optional: true,
            }
        } else {
            let modifier = DotModifier::parse(input)?;
            Self {
                modifier,
                optional: false,
            }
        })
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct DotModifier {
    part1: IdentLike,
    part2: Option<IdentOrTypeSuffix>,
}

impl std::fmt::Display for DotModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ".")?;
        self.part1.fmt(f)?;
        if let Some(ref part2) = self.part2 {
            write!(f, "::")?;
            part2.0.fmt(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for DotModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl DotModifier {
    pub fn ident(&self) -> Ident {
        let mut result = String::new();
        write!(&mut result, "{}", self.part1).unwrap();
        if let Some(ref part2) = self.part2 {
            write!(&mut result, "_{}", part2.0).unwrap();
        } else {
            match self.part1 {
                IdentLike::Type | IdentLike::Const => result.push('_'),
                IdentLike::Ident(_) | IdentLike::Integer(_) => {}
            }
        }
        Ident::new(&result.to_ascii_lowercase(), Span::call_site())
    }

    pub fn variant_capitalized(&self) -> Ident {
        self.capitalized_impl(String::new())
    }

    pub fn dot_capitalized(&self) -> Ident {
        self.capitalized_impl("Dot".to_string())
    }

    fn capitalized_impl(&self, prefix: String) -> Ident {
        let mut temp = String::new();
        write!(&mut temp, "{}", &self.part1).unwrap();
        if let Some(IdentOrTypeSuffix(ref part2)) = self.part2 {
            write!(&mut temp, "_{}", part2).unwrap();
        }
        let mut result = prefix;
        let mut capitalize = true;
        for c in temp.chars() {
            if c == '_' {
                capitalize = true;
                continue;
            }
            let c = if capitalize {
                capitalize = false;
                c.to_ascii_uppercase()
            } else {
                c
            };
            result.push(c);
        }
        Ident::new(&result, Span::call_site())
    }

    pub fn tokens(&self) -> TokenStream {
        let part1 = &self.part1;
        let part2 = &self.part2;
        match self.part2 {
            None => quote! { . #part1 },
            Some(_) => quote! { . #part1 #part2 },
        }
    }

    fn peek(input: syn::parse::ParseStream) -> bool {
        input.peek(Token![.])
    }
}

impl Parse for DotModifier {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![.]>()?;
        let part1 = input.parse::<IdentLike>()?;
        if IdentOrTypeSuffix::peek(input) {
            let part2 = Some(IdentOrTypeSuffix::parse(input)?);
            Ok(Self { part1, part2 })
        } else {
            Ok(Self { part1, part2: None })
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum IdentLike {
    Type,
    Const,
    Ident(Ident),
    Integer(LitInt),
}

impl std::fmt::Display for IdentLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentLike::Type => f.write_str("type"),
            IdentLike::Const => f.write_str("const"),
            IdentLike::Ident(ident) => write!(f, "{}", ident),
            IdentLike::Integer(integer) => write!(f, "{}", integer),
        }
    }
}

impl ToTokens for IdentLike {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            IdentLike::Type => quote! { type }.to_tokens(tokens),
            IdentLike::Const => quote! { const }.to_tokens(tokens),
            IdentLike::Ident(ident) => quote! { #ident }.to_tokens(tokens),
            IdentLike::Integer(int) => quote! { #int }.to_tokens(tokens),
        }
    }
}

impl Parse for IdentLike {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        Ok(if lookahead.peek(Token![const]) {
            input.parse::<Token![const]>()?;
            IdentLike::Const
        } else if lookahead.peek(Token![type]) {
            input.parse::<Token![type]>()?;
            IdentLike::Type
        } else if lookahead.peek(Ident) {
            IdentLike::Ident(input.parse::<Ident>()?)
        } else if lookahead.peek(LitInt) {
            IdentLike::Integer(input.parse::<LitInt>()?)
        } else {
            return Err(lookahead.error());
        })
    }
}

// Arguments decalaration can loook like this:
//  a{, b}
// That's why we don't parse Arguments as Punctuated<Argument, Token![,]>
#[derive(PartialEq, Eq)]
pub struct Arguments(pub Vec<Argument>);

impl Parse for Arguments {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut result = Vec::new();
        loop {
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
            let mut optional = false;
            let mut can_be_negated = false;
            let mut pre_pipe = false;
            let ident;
            let lookahead = input.lookahead1();
            if lookahead.peek(token::Brace) {
                let content;
                braced!(content in input);
                let lookahead = content.lookahead1();
                if lookahead.peek(Token![!]) {
                    content.parse::<Token![!]>()?;
                    can_be_negated = true;
                    ident = input.parse::<Ident>()?;
                } else if lookahead.peek(Token![,]) {
                    optional = true;
                    content.parse::<Token![,]>()?;
                    ident = content.parse::<Ident>()?;
                } else {
                    return Err(lookahead.error());
                }
            } else if lookahead.peek(token::Bracket) {
                let bracketed;
                bracketed!(bracketed in input);
                if bracketed.peek(Token![|]) {
                    optional = true;
                    bracketed.parse::<Token![|]>()?;
                    pre_pipe = true;
                    ident = bracketed.parse::<Ident>()?;
                } else {
                    let mut sub_args = Self::parse(&bracketed)?;
                    sub_args.0.first_mut().unwrap().pre_bracket = true;
                    sub_args.0.last_mut().unwrap().post_bracket = true;
                    if peek_brace_token(input, Token![.]) {
                        let optional_suffix;
                        braced!(optional_suffix in input);
                        optional_suffix.parse::<Token![.]>()?;
                        let unified_ident = optional_suffix.parse::<Ident>()?;
                        if unified_ident.to_string() != "unified" {
                            return Err(syn::Error::new(
                                unified_ident.span(),
                                format!("Exptected `unified`, got `{}`", unified_ident),
                            ));
                        }
                        for a in sub_args.0.iter_mut() {
                            a.unified = true;
                        }
                    }
                    result.extend(sub_args.0);
                    continue;
                }
            } else if lookahead.peek(Ident) {
                ident = input.parse::<Ident>()?;
            } else if lookahead.peek(Token![|]) {
                input.parse::<Token![|]>()?;
                pre_pipe = true;
                ident = input.parse::<Ident>()?;
            } else {
                break;
            }
            result.push(Argument {
                optional,
                pre_pipe,
                can_be_negated,
                pre_bracket: false,
                ident,
                post_bracket: false,
                unified: false,
            });
        }
        Ok(Self(result))
    }
}

// This is effectively input.peek(token::Brace) && input.peek2(Token![.])
// input.peek2 is supposed to skip over next token, but it skips over whole
// braced token group. Not sure if it's a bug
fn peek_brace_token<T: Peek>(input: syn::parse::ParseStream, _t: T) -> bool {
    use syn::token::Token;
    let cursor = input.cursor();
    cursor
        .group(proc_macro2::Delimiter::Brace)
        .map_or(false, |(content, ..)| T::Token::peek(content))
}

#[derive(PartialEq, Eq)]
pub struct Argument {
    pub optional: bool,
    pub pre_bracket: bool,
    pub pre_pipe: bool,
    pub can_be_negated: bool,
    pub ident: Ident,
    pub post_bracket: bool,
    pub unified: bool,
}

#[cfg(test)]
mod tests {
    use super::{Arguments, DotModifier, MaybeDotModifier};
    use quote::{quote, ToTokens};

    #[test]
    fn parse_modifier_complex() {
        let input = quote! {
            .level::eviction_priority
        };
        let modifier = syn::parse2::<DotModifier>(input).unwrap();
        assert_eq!(
            ". level :: eviction_priority",
            modifier.tokens().to_string()
        );
    }

    #[test]
    fn parse_modifier_optional() {
        let input = quote! {
            { .level::eviction_priority }
        };
        let maybe_modifider = syn::parse2::<MaybeDotModifier>(input).unwrap();
        assert_eq!(
            ". level :: eviction_priority",
            maybe_modifider.modifier.tokens().to_string()
        );
        assert!(maybe_modifider.optional);
    }

    #[test]
    fn parse_type_token() {
        let input = quote! {
           . type
        };
        let maybe_modifier = syn::parse2::<MaybeDotModifier>(input).unwrap();
        assert_eq!(". type", maybe_modifier.modifier.tokens().to_string());
        assert!(!maybe_modifier.optional);
    }

    #[test]
    fn arguments_memory() {
        let input = quote! {
            [a], b
        };
        let arguments = syn::parse2::<Arguments>(input).unwrap();
        let a = &arguments.0[0];
        assert!(!a.optional);
        assert_eq!("a", a.ident.to_string());
        assert!(a.pre_bracket);
        assert!(!a.pre_pipe);
        assert!(a.post_bracket);
        assert!(!a.can_be_negated);
        let b = &arguments.0[1];
        assert!(!b.optional);
        assert_eq!("b", b.ident.to_string());
        assert!(!b.pre_bracket);
        assert!(!b.pre_pipe);
        assert!(!b.post_bracket);
        assert!(!b.can_be_negated);
    }

    #[test]
    fn arguments_optional() {
        let input = quote! {
            b{, cache_policy}
        };
        let arguments = syn::parse2::<Arguments>(input).unwrap();
        let b = &arguments.0[0];
        assert!(!b.optional);
        assert_eq!("b", b.ident.to_string());
        assert!(!b.pre_bracket);
        assert!(!b.pre_pipe);
        assert!(!b.post_bracket);
        assert!(!b.can_be_negated);
        let cache_policy = &arguments.0[1];
        assert!(cache_policy.optional);
        assert_eq!("cache_policy", cache_policy.ident.to_string());
        assert!(!cache_policy.pre_bracket);
        assert!(!cache_policy.pre_pipe);
        assert!(!cache_policy.post_bracket);
        assert!(!cache_policy.can_be_negated);
    }

    #[test]
    fn arguments_optional_pred() {
        let input = quote! {
            p[|q], a
        };
        let arguments = syn::parse2::<Arguments>(input).unwrap();
        assert_eq!(arguments.0.len(), 3);
        let p = &arguments.0[0];
        assert!(!p.optional);
        assert_eq!("p", p.ident.to_string());
        assert!(!p.pre_bracket);
        assert!(!p.pre_pipe);
        assert!(!p.post_bracket);
        assert!(!p.can_be_negated);
        let q = &arguments.0[1];
        assert!(q.optional);
        assert_eq!("q", q.ident.to_string());
        assert!(!q.pre_bracket);
        assert!(q.pre_pipe);
        assert!(!q.post_bracket);
        assert!(!q.can_be_negated);
        let a = &arguments.0[2];
        assert!(!a.optional);
        assert_eq!("a", a.ident.to_string());
        assert!(!a.pre_bracket);
        assert!(!a.pre_pipe);
        assert!(!a.post_bracket);
        assert!(!a.can_be_negated);
    }

    #[test]
    fn arguments_optional_with_negate() {
        let input = quote! {
            b, {!}c
        };
        let arguments = syn::parse2::<Arguments>(input).unwrap();
        assert_eq!(arguments.0.len(), 2);
        let b = &arguments.0[0];
        assert!(!b.optional);
        assert_eq!("b", b.ident.to_string());
        assert!(!b.pre_bracket);
        assert!(!b.pre_pipe);
        assert!(!b.post_bracket);
        assert!(!b.can_be_negated);
        let c = &arguments.0[1];
        assert!(!c.optional);
        assert_eq!("c", c.ident.to_string());
        assert!(!c.pre_bracket);
        assert!(!c.pre_pipe);
        assert!(!c.post_bracket);
        assert!(c.can_be_negated);
    }

    #[test]
    fn arguments_tex() {
        let input = quote! {
            d[|p], [a{, b}, c], dpdx, dpdy {, e}
        };
        let arguments = syn::parse2::<Arguments>(input).unwrap();
        assert_eq!(arguments.0.len(), 8);
        {
            let d = &arguments.0[0];
            assert!(!d.optional);
            assert_eq!("d", d.ident.to_string());
            assert!(!d.pre_bracket);
            assert!(!d.pre_pipe);
            assert!(!d.post_bracket);
            assert!(!d.can_be_negated);
        }
        {
            let p = &arguments.0[1];
            assert!(p.optional);
            assert_eq!("p", p.ident.to_string());
            assert!(!p.pre_bracket);
            assert!(p.pre_pipe);
            assert!(!p.post_bracket);
            assert!(!p.can_be_negated);
        }
        {
            let a = &arguments.0[2];
            assert!(!a.optional);
            assert_eq!("a", a.ident.to_string());
            assert!(a.pre_bracket);
            assert!(!a.pre_pipe);
            assert!(!a.post_bracket);
            assert!(!a.can_be_negated);
        }
        {
            let b = &arguments.0[3];
            assert!(b.optional);
            assert_eq!("b", b.ident.to_string());
            assert!(!b.pre_bracket);
            assert!(!b.pre_pipe);
            assert!(!b.post_bracket);
            assert!(!b.can_be_negated);
        }
        {
            let c = &arguments.0[4];
            assert!(!c.optional);
            assert_eq!("c", c.ident.to_string());
            assert!(!c.pre_bracket);
            assert!(!c.pre_pipe);
            assert!(c.post_bracket);
            assert!(!c.can_be_negated);
        }
        {
            let dpdx = &arguments.0[5];
            assert!(!dpdx.optional);
            assert_eq!("dpdx", dpdx.ident.to_string());
            assert!(!dpdx.pre_bracket);
            assert!(!dpdx.pre_pipe);
            assert!(!dpdx.post_bracket);
            assert!(!dpdx.can_be_negated);
        }
        {
            let dpdy = &arguments.0[6];
            assert!(!dpdy.optional);
            assert_eq!("dpdy", dpdy.ident.to_string());
            assert!(!dpdy.pre_bracket);
            assert!(!dpdy.pre_pipe);
            assert!(!dpdy.post_bracket);
            assert!(!dpdy.can_be_negated);
        }
        {
            let e = &arguments.0[7];
            assert!(e.optional);
            assert_eq!("e", e.ident.to_string());
            assert!(!e.pre_bracket);
            assert!(!e.pre_pipe);
            assert!(!e.post_bracket);
            assert!(!e.can_be_negated);
        }
    }

    #[test]
    fn rule_multi() {
        let input = quote! {
            .ss: StateSpace =           { .global, .local, .param{::func}, .shared{::cta, ::cluster} }
        };
        let rule = syn::parse2::<super::Rule>(input).unwrap();
        assert_eq!(". ss", rule.modifier.tokens().to_string());
        assert_eq!(
            "StateSpace",
            rule.type_.unwrap().to_token_stream().to_string()
        );
        let alts = rule
            .alternatives
            .iter()
            .map(|m| m.tokens().to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            vec![
                ". global",
                ". local",
                ". param",
                ". param :: func",
                ". shared",
                ". shared :: cta",
                ". shared :: cluster"
            ],
            alts
        );
    }

    #[test]
    fn rule_multi2() {
        let input = quote! {
            .cop: StCacheOperator =     { .wb, .cg, .cs, .wt }
        };
        let rule = syn::parse2::<super::Rule>(input).unwrap();
        assert_eq!(". cop", rule.modifier.tokens().to_string());
        assert_eq!(
            "StCacheOperator",
            rule.type_.unwrap().to_token_stream().to_string()
        );
        let alts = rule
            .alternatives
            .iter()
            .map(|m| m.tokens().to_string())
            .collect::<Vec<_>>();
        assert_eq!(vec![". wb", ". cg", ". cs", ". wt",], alts);
    }

    #[test]
    fn args_unified() {
        let input = quote! {
            d, [a]{.unified}{, cache_policy}
        };
        let args = syn::parse2::<super::Arguments>(input).unwrap();
        let a = &args.0[1];
        assert!(!a.optional);
        assert_eq!("a", a.ident.to_string());
        assert!(a.pre_bracket);
        assert!(!a.pre_pipe);
        assert!(a.post_bracket);
        assert!(!a.can_be_negated);
        assert!(a.unified);
    }
}
