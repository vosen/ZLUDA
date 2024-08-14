use gen_impl::parser;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::hash_map, hash::Hash, rc::Rc};
use syn::{parse_macro_input, punctuated::Punctuated, Ident, ItemEnum, Token, TypePath, Variant};

// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#vectors
// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#fundamental-types
// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#alternate-floating-point-data-formats
#[rustfmt::skip]
static POSTFIX_MODIFIERS: &[&str] = &[
    ".v2", ".v4",
    ".s8", ".s16", ".s32", ".s64",
    ".u8", ".u16", ".u32", ".u64",
    ".f16", ".f16x2", ".f32", ".f64",
    ".b8", ".b16", ".b32", ".b64", ".b128",
    ".pred",
    ".bf16", ".e4m3", ".e5m2", ".tf32",
];

static POSTFIX_TYPES: &[&str] = &["ScalarType", "VectorPrefix"];

struct OpcodeDefinitions {
    definitions: Vec<SingleOpcodeDefinition>,
    block_selection: Vec<Vec<(Option<parser::DotModifier>, usize)>>,
}

impl OpcodeDefinitions {
    fn new(opcode: &Ident, definitions: Vec<SingleOpcodeDefinition>) -> Self {
        let mut selections = vec![None; definitions.len()];
        let mut generation = 0usize;
        loop {
            let mut selected_something = false;
            let unselected = selections
                .iter()
                .enumerate()
                .filter_map(|(idx, s)| if s.is_none() { Some(idx) } else { None })
                .collect::<Vec<_>>();
            match &*unselected {
                [] => break,
                [remaining] => {
                    selections[*remaining] = Some((None, generation));
                    break;
                }
                _ => {}
            }
            'check_definitions: for i in unselected.iter().copied() {
                // just pick the first alternative and attempt every modifier
                'check_candidates: for candidate in definitions[i]
                    .unordered_modifiers
                    .iter()
                    .chain(definitions[i].ordered_modifiers.iter())
                {
                    let candidate = if let DotModifierRef::Direct {
                        optional: false,
                        value,
                    } = candidate
                    {
                        value
                    } else {
                        continue;
                    };
                    // check all other unselected patterns
                    for j in unselected.iter().copied() {
                        if i == j {
                            continue;
                        }
                        if definitions[j].possible_modifiers.contains(candidate) {
                            continue 'check_candidates;
                        }
                    }
                    // it's unique
                    selections[i] = Some((Some(candidate), generation));
                    selected_something = true;
                    continue 'check_definitions;
                }
            }
            if !selected_something {
                panic!(
                    "Failed to generate pattern selection for `{}`. State: {:?}",
                    opcode,
                    selections.into_iter().rev().collect::<Vec<_>>()
                );
            }
            generation += 1;
        }
        let mut block_selection = Vec::new();
        for current_generation in 0usize.. {
            let mut current_generation_definitions = Vec::new();
            for (idx, selection) in selections.iter_mut().enumerate() {
                match selection {
                    Some((modifier, generation)) => {
                        if *generation == current_generation {
                            current_generation_definitions.push((modifier.cloned(), idx));
                            *selection = None;
                        }
                    }
                    None => {}
                }
            }
            if current_generation_definitions.is_empty() {
                break;
            }
            block_selection.push(current_generation_definitions);
        }
        #[cfg(debug_assertions)]
        {
            let selected = block_selection
                .iter()
                .map(|x| x.len())
                .reduce(|x, y| x + y)
                .unwrap();
            if selected != definitions.len() {
                panic!(
                    "Internal error when generating pattern selection for `{}`: {:?}",
                    opcode, &block_selection
                );
            }
        }
        Self {
            definitions,
            block_selection,
        }
    }

    fn get_enum_types(
        parse_definitions: &[parser::OpcodeDefinition],
    ) -> FxHashMap<syn::Type, FxHashSet<parser::DotModifier>> {
        let mut result = FxHashMap::default();
        for parser::OpcodeDefinition(_, rules) in parse_definitions.iter() {
            for rule in rules {
                let type_ = match rule.type_ {
                    Some(ref type_) => type_.clone(),
                    None => continue,
                };
                let insert_values = |set: &mut FxHashSet<_>| {
                    for value in rule.alternatives.iter().cloned() {
                        set.insert(value);
                    }
                };
                match result.entry(type_) {
                    hash_map::Entry::Occupied(mut entry) => insert_values(entry.get_mut()),
                    hash_map::Entry::Vacant(entry) => {
                        insert_values(entry.insert(FxHashSet::default()))
                    }
                };
            }
        }
        result
    }
}

struct SingleOpcodeDefinition {
    possible_modifiers: FxHashSet<parser::DotModifier>,
    unordered_modifiers: Vec<DotModifierRef>,
    ordered_modifiers: Vec<DotModifierRef>,
    arguments: parser::Arguments,
    code_block: parser::CodeBlock,
}

impl SingleOpcodeDefinition {
    fn function_arguments_declarations(&self) -> impl Iterator<Item = TokenStream> + '_ {
        self.unordered_modifiers
            .iter()
            .chain(self.ordered_modifiers.iter())
            .filter_map(|modf| {
                let type_ = modf.type_of();
                type_.map(|t| {
                    let name = modf.ident();
                    quote! { #name : #t }
                })
            })
            .chain(self.arguments.0.iter().map(|arg| {
                let name = &arg.ident;
                if arg.optional {
                    quote! { #name : Option<&str> }
                } else {
                    quote! { #name : &str }
                }
            }))
    }

    fn function_arguments(&self) -> impl Iterator<Item = TokenStream> + '_ {
        self.unordered_modifiers
            .iter()
            .chain(self.ordered_modifiers.iter())
            .filter_map(|modf| {
                let type_ = modf.type_of();
                type_.map(|_| {
                    let name = modf.ident();
                    quote! { #name }
                })
            })
            .chain(self.arguments.0.iter().map(|arg| {
                let name = &arg.ident;
                quote! { #name }
            }))
    }

    fn extract_and_insert(
        output: &mut FxHashMap<Ident, Vec<SingleOpcodeDefinition>>,
        parser::OpcodeDefinition(pattern_seq, rules): parser::OpcodeDefinition,
    ) {
        let mut rules = rules
            .into_iter()
            .map(|r| (r.modifier.clone(), Rc::new(r)))
            .collect::<FxHashMap<_, _>>();
        let mut last_opcode = pattern_seq.0.last().unwrap().0 .0.name.clone();
        for (opcode_decl, code_block) in pattern_seq.0.into_iter().rev() {
            let current_opcode = opcode_decl.0.name.clone();
            if last_opcode != current_opcode {
                rules = FxHashMap::default();
            }
            let mut possible_modifiers = FxHashSet::default();
            for (_, options) in rules.iter() {
                possible_modifiers.extend(options.alternatives.iter().cloned());
            }
            let parser::OpcodeDecl(instruction, arguments) = opcode_decl;
            let mut unordered_modifiers = instruction
                .modifiers
                .into_iter()
                .map(
                    |parser::MaybeDotModifier { optional, modifier }| match rules.get(&modifier) {
                        Some(alts) => {
                            if alts.alternatives.len() == 1 && alts.type_.is_none() {
                                DotModifierRef::Direct {
                                    optional,
                                    value: alts.alternatives[0].clone(),
                                }
                            } else {
                                DotModifierRef::Indirect {
                                    optional,
                                    value: alts.clone(),
                                }
                            }
                        }
                        None => {
                            possible_modifiers.insert(modifier.clone());
                            DotModifierRef::Direct {
                                optional,
                                value: modifier,
                            }
                        }
                    },
                )
                .collect::<Vec<_>>();
            let ordered_modifiers = Self::extract_ordered_modifiers(&mut unordered_modifiers);
            let entry = Self {
                possible_modifiers,
                unordered_modifiers,
                ordered_modifiers,
                arguments,
                code_block,
            };
            multihash_extend(output, current_opcode.clone(), entry);
            last_opcode = current_opcode;
        }
    }

    fn extract_ordered_modifiers(
        unordered_modifiers: &mut Vec<DotModifierRef>,
    ) -> Vec<DotModifierRef> {
        let mut result = Vec::new();
        loop {
            let is_ordered = match unordered_modifiers.last() {
                Some(DotModifierRef::Direct { value, .. }) => {
                    let name = value.to_string();
                    POSTFIX_MODIFIERS.contains(&&*name)
                }
                Some(DotModifierRef::Indirect { value, .. }) => {
                    let type_ = value.type_.to_token_stream().to_string();
                    //panic!("{} {}", type_, POSTFIX_TYPES.contains(&&*type_));
                    POSTFIX_TYPES.contains(&&*type_)
                }
                None => break,
            };
            if is_ordered {
                result.push(unordered_modifiers.pop().unwrap());
            } else {
                break;
            }
        }
        if unordered_modifiers.len() == 1 {
            result.push(unordered_modifiers.pop().unwrap());
        }
        result.reverse();
        result
    }
}

#[proc_macro]
pub fn derive_parser(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parse_definitions = parse_macro_input!(tokens as gen_impl::parser::ParseDefinitions);
    let mut definitions = FxHashMap::default();
    let types = OpcodeDefinitions::get_enum_types(&parse_definitions.definitions);
    let enum_types_tokens = emit_enum_types(types, parse_definitions.additional_enums);
    for definition in parse_definitions.definitions.into_iter() {
        SingleOpcodeDefinition::extract_and_insert(&mut definitions, definition);
    }
    let definitions = definitions
        .into_iter()
        .map(|(k, v)| {
            let v = OpcodeDefinitions::new(&k, v);
            (k, v)
        })
        .collect::<FxHashMap<_, _>>();
    let mut token_enum = parse_definitions.token_type;
    let (_, all_modifier) = write_definitions_into_tokens(&definitions, &mut token_enum.variants);
    let token_impl = emit_parse_function(&token_enum.ident, &definitions, all_modifier);
    let tokens = quote! {
        #enum_types_tokens

        #token_enum

        #token_impl
    };
    tokens.into()
}

fn emit_enum_types(
    types: FxHashMap<syn::Type, FxHashSet<parser::DotModifier>>,
    mut existing_enums: FxHashMap<Ident, ItemEnum>,
) -> TokenStream {
    let token_types = types.into_iter().filter_map(|(type_, variants)| {
        match type_ {
            syn::Type::Path(TypePath {
                qself: None,
                ref path,
            }) => {
                if let Some(ident) = path.get_ident() {
                    if let Some(enum_) = existing_enums.get_mut(ident) {
                        enum_.variants.extend(variants.into_iter().map(|modifier| {
                            let ident = modifier.variant_capitalized();
                            let variant: syn::Variant = syn::parse_quote! {
                                #ident
                            };
                            variant
                        }));
                        return None;
                    }
                }
            }
            _ => {}
        }
        let variants = variants.iter().map(|v| v.variant_capitalized());
        Some(quote! {
            #[derive(Copy, Clone, PartialEq, Eq, Hash)]
            enum #type_ {
                #(#variants),*
            }
        })
    });
    let mut result = TokenStream::new();
    for tokens in token_types {
        tokens.to_tokens(&mut result);
    }
    for (_, enum_) in existing_enums {
        quote! { #enum_ }.to_tokens(&mut result);
    }
    result
}

fn emit_parse_function(
    type_name: &Ident,
    defs: &FxHashMap<Ident, OpcodeDefinitions>,
    all_modifier: FxHashSet<&parser::DotModifier>,
) -> TokenStream {
    use std::fmt::Write;
    let fns_ = defs
        .iter()
        .map(|(opcode, defs)| {
            defs.definitions.iter().enumerate().map(|(idx, def)| {
                let mut fn_name = opcode.to_string();
                write!(&mut fn_name, "_{}", idx).ok();
                let fn_name = Ident::new(&fn_name, Span::call_site());
                let code_block = &def.code_block.0;
                let args = def.function_arguments_declarations();
                quote! {
                    fn #fn_name( #(#args),* ) -> Instruction<ParsedOperand> #code_block
                }
            })
        })
        .flatten();
    let selectors = defs.iter().map(|(opcode, def)| {
        let opcode_variant = Ident::new(&capitalize(&opcode.to_string()), opcode.span());
        let mut result = TokenStream::new();
        let mut selectors = TokenStream::new();
        quote! {
            if false {
                unsafe { std::hint::unreachable_unchecked() }
            }
        }
        .to_tokens(&mut selectors);
        let mut has_default_selector = false;
        for selection_layer in def.block_selection.iter() {
            for (selection_key, selected_definition) in selection_layer {
                let def_parser = emit_definition_parser(type_name,  (opcode,*selected_definition), &def.definitions[*selected_definition]);
                match selection_key {
                    Some(selection_key) => {
                        let selection_key =
                            selection_key.dot_capitalized();
                        quote! {
                            else if modifiers.contains(& #type_name :: #selection_key) {
                                #def_parser
                            }
                        }
                        .to_tokens(&mut selectors);
                    }
                    None => {
                        has_default_selector = true;
                        quote! {
                            else {
                                #def_parser
                            }
                        }
                        .to_tokens(&mut selectors);
                    }
                }
            }
        }
        if !has_default_selector {
            quote! {
                else {
                    return Err(winnow::error::ErrMode::from_error_kind(stream, winnow::error::ErrorKind::Token))
                }
            }
            .to_tokens(&mut selectors);
        }
        quote! {
            #opcode_variant => {
                let modifers_start = stream.checkpoint();
                let modifiers = take_while(0.., Token::modifier).parse_next(stream)?;
                #selectors
            }
        }
        .to_tokens(&mut result);
        result
    });
    let modifier_names = all_modifier.iter().map(|m| m.dot_capitalized());
    quote! {
        impl<'input> #type_name<'input> {
            fn modifier(self) -> bool {
                match self {
                    #(
                        #type_name :: #modifier_names => true,
                    )*
                    _ => false
                }
            }
        }

        #(#fns_)*

        fn parse_instruction<'input>(stream: &mut (impl winnow::stream::Stream<Token = #type_name<'input>, Slice = &'input [#type_name<'input>]> + winnow::stream::StreamIsPartial)) -> winnow::error::PResult<Instruction<ParsedOperand>> 
        {
            use winnow::Parser;
            use winnow::token::*;
            use winnow::combinator::*;
            let opcode = any.parse_next(stream)?;
            let modifiers_start = stream.checkpoint();
            Ok(match opcode {
                #(
                    #type_name :: #selectors
                )*
                _ => return Err(winnow::error::ErrMode::from_error_kind(stream, winnow::error::ErrorKind::Token))
            })
        }
    }
}

fn emit_definition_parser(
    token_type: &Ident,
    (opcode, fn_idx): (&Ident, usize),
    definition: &SingleOpcodeDefinition,
) -> TokenStream {
    let return_error_ref = quote! {
        return Err(winnow::error::ErrMode::from_error_kind(&stream, winnow::error::ErrorKind::Token))
    };
    let return_error = quote! {
        return Err(winnow::error::ErrMode::from_error_kind(stream, winnow::error::ErrorKind::Token))
    };
    let ordered_parse_declarations = definition.ordered_modifiers.iter().map(|modifier| {
        modifier.type_of().map(|type_| {
            let name = modifier.ident();
            quote! {
                let #name : #type_;
            }
        })
    });
    let ordered_parse = definition.ordered_modifiers.iter().rev().map(|modifier| {
        let arg_name = modifier.ident();
        let arg_type = modifier.type_of();
        match modifier {
            DotModifierRef::Direct { optional, value } => {
                let variant = value.dot_capitalized();
                if *optional {
                    quote! {
                        #arg_name = opt(any.verify(|t| *t == #token_type :: #variant)).parse_next(&mut stream)?.is_some();
                    }
                } else {
                    quote! {
                        any.verify(|t| *t == #token_type :: #variant).parse_next(&mut stream)?;
                    }
                }
            }
            DotModifierRef::Indirect { optional, value } => {
                let variants = value.alternatives.iter().map(|alt| {
                    let type_ = value.type_.as_ref().unwrap();
                    let token_variant = alt.dot_capitalized();
                    let parsed_variant = alt.variant_capitalized();
                    quote! {
                        #token_type :: #token_variant => #type_ :: #parsed_variant,
                    }
                });
                if *optional {
                    quote! {
                        #arg_name = opt(any.verify_map(|tok| {
                            Some(match tok {
                                #(#variants)*
                                _ => return None
                            })
                        })).parse_next(&mut stream)?;
                    }
                } else {
                    quote! {
                        #arg_name = any.verify_map(|tok| {
                            Some(match tok {
                                #(#variants)*
                                _ => return None
                            })
                        }).parse_next(&mut stream)?;
                    }
                }
            }
        }
    });
    let unordered_parse_declarations = definition.unordered_modifiers.iter().map(|modifier| {
        let name = modifier.ident();
        let type_ = modifier.type_of_check();
        quote! {
            let mut #name : #type_ = std::default::Default::default();
        }
    });
    let unordered_parse = definition
        .unordered_modifiers
        .iter()
        .map(|modifier| match modifier {
            DotModifierRef::Direct { value, .. } => {
                let name = value.ident();
                let token_variant = value.dot_capitalized();
                quote! {
                    #token_type :: #token_variant =>  {
                        if #name {
                            #return_error_ref;
                        }
                        #name = true;
                    }
                }
            }
            DotModifierRef::Indirect { value, .. } => {
                let variable = value.modifier.ident();
                let type_ = value.type_.as_ref().unwrap();
                let alternatives = value.alternatives.iter().map(|alt| {
                    let token_variant = alt.dot_capitalized();
                    let enum_variant = alt.variant_capitalized();
                    quote! {
                        #token_type :: #token_variant => {
                            if #variable.is_some() {
                                #return_error_ref;
                            }
                            #variable = Some(#type_ :: #enum_variant);
                        }
                    }
                });
                quote! {
                    #(#alternatives)*
                }
            }
        });
    let unordered_parse_validations =
        definition
            .unordered_modifiers
            .iter()
            .map(|modifier| match modifier {
                DotModifierRef::Direct {
                    optional: false,
                    value,
                } => {
                    let variable = value.ident();
                    quote! {
                        if !#variable {
                            #return_error;
                        }
                    }
                }
                DotModifierRef::Direct { optional: true, .. } => TokenStream::new(),
                DotModifierRef::Indirect {
                    optional: false,
                    value,
                } => {
                    let variable = value.modifier.ident();
                    quote! {
                        let #variable = match #variable {
                            Some(x) => x,
                            None => #return_error
                        };
                    }
                }
                DotModifierRef::Indirect { optional: true, .. } => TokenStream::new(),
            });
    let arguments_parse = definition.arguments.0.iter().enumerate().map(|(idx, arg)| {
        let comma = if idx == 0 {
            quote! { empty }
        } else {
            quote! { any.verify(|t| *t == #token_type::Comma) }
        };
        let pre_bracket = if arg.pre_bracket {
            quote! {
                any.verify(|t| *t == #token_type::LBracket).map(|_| ())
            }
        } else {
            quote! {
                empty
            }
        };
        let pre_pipe = if arg.pre_pipe {
            quote! {
                any.verify(|t| *t == #token_type::Or).map(|_| ())
            }
        } else {
            quote! {
                empty
            }
        };
        let can_be_negated = if arg.can_be_negated {
            quote! {
                opt(any.verify(|t| *t == #token_type::Not)).map(|o| o.is_some())
            }
        } else {
            quote! {
                empty
            }
        };
        let ident = {
            quote! {
                any.verify_map(|t| match t { #token_type::Ident(s) => Some(s), _ => None })
            }
        };
        let post_bracket = if arg.post_bracket {
            quote! {
                any.verify(|t| *t == #token_type::RBracket).map(|_| ())
            }
        } else {
            quote! {
                empty
            }
        };
        let parser = quote! {
            (#comma, #pre_bracket, #pre_pipe, #can_be_negated, #ident, #post_bracket)
        };
        let arg_name = &arg.ident;
        if arg.optional {
            quote! {
                let #arg_name = opt(#parser.map(|(_, _, _, _, name, _)| name)).parse_next(stream)?;
            }
        } else {
            quote! {
                let #arg_name = #parser.map(|(_, _, _, _, name, _)| name).parse_next(stream)?;
            }
        }
    });
    let fn_args = definition.function_arguments();
    let fn_name = format_ident!("{}_{}", opcode, fn_idx);
    let fn_call = quote! {
        #fn_name(  #(#fn_args),* )
    };
    quote! {
        #(#unordered_parse_declarations)*
        #(#ordered_parse_declarations)*
        {
            let mut stream = ReverseStream(modifiers);
            #(#ordered_parse)*
            let mut stream: &[#token_type] = stream.0;
            for token in stream.iter().copied() {
                match token {
                    #(#unordered_parse)*
                    _ => #return_error_ref
                }
            }
        }
        #(#unordered_parse_validations)*
        #(#arguments_parse)*
        #fn_call
    }
}

fn write_definitions_into_tokens<'a>(
    defs: &'a FxHashMap<Ident, OpcodeDefinitions>,
    variants: &mut Punctuated<Variant, Token![,]>,
) -> (Vec<&'a Ident>, FxHashSet<&'a parser::DotModifier>) {
    let mut all_opcodes = Vec::new();
    let mut all_modifiers = FxHashSet::default();
    for (opcode, definitions) in defs.iter() {
        all_opcodes.push(opcode);
        let opcode_as_string = opcode.to_string();
        let variant_name = Ident::new(&capitalize(&opcode_as_string), opcode.span());
        let arg: Variant = syn::parse_quote! {
            #[token(#opcode_as_string)]
            #variant_name
        };
        variants.push(arg);
        for definition in definitions.definitions.iter() {
            for modifier in definition.possible_modifiers.iter() {
                all_modifiers.insert(modifier);
            }
        }
    }
    for modifier in all_modifiers.iter() {
        let modifier_as_string = modifier.to_string();
        let variant_name = modifier.dot_capitalized();
        let arg: Variant = syn::parse_quote! {
            #[token(#modifier_as_string)]
            #variant_name
        };
        variants.push(arg);
    }
    (all_opcodes, all_modifiers)
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn multihash_extend<K: Eq + Hash, V>(multimap: &mut FxHashMap<K, Vec<V>>, k: K, v: V) {
    match multimap.entry(k) {
        hash_map::Entry::Occupied(mut entry) => entry.get_mut().push(v),
        hash_map::Entry::Vacant(entry) => {
            entry.insert(vec![v]);
        }
    }
}

enum DotModifierRef {
    Direct {
        optional: bool,
        value: parser::DotModifier,
    },
    Indirect {
        optional: bool,
        value: Rc<parser::Rule>,
    },
}

impl DotModifierRef {
    fn ident(&self) -> Ident {
        match self {
            DotModifierRef::Direct { value, .. } => value.ident(),
            DotModifierRef::Indirect { value, .. } => value.modifier.ident(),
        }
    }

    fn type_of(&self) -> Option<syn::Type> {
        Some(match self {
            DotModifierRef::Direct { optional: true, .. } => syn::parse_quote! { bool },
            DotModifierRef::Direct {
                optional: false, ..
            } => return None,
            DotModifierRef::Indirect { optional, value } => {
                let type_ = value
                    .type_
                    .as_ref()
                    .expect("Indirect modifer must have a type");
                if *optional {
                    syn::parse_quote! { Option<#type_> }
                } else {
                    type_.clone()
                }
            }
        })
    }

    fn type_of_check(&self) -> syn::Type {
        match self {
            DotModifierRef::Direct { .. } => syn::parse_quote! { bool },
            DotModifierRef::Indirect { value, .. } => {
                let type_ = value
                    .type_
                    .as_ref()
                    .expect("Indirect modifer must have a type");
                syn::parse_quote! { Option<#type_> }
            }
        }
    }
}

impl Hash for DotModifierRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            DotModifierRef::Direct { optional, value } => {
                optional.hash(state);
                value.hash(state);
            }
            DotModifierRef::Indirect { optional, value } => {
                optional.hash(state);
                (value.as_ref() as *const parser::Rule).hash(state);
            }
        }
    }
}

impl PartialEq for DotModifierRef {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Direct {
                    optional: l_optional,
                    value: l_value,
                },
                Self::Direct {
                    optional: r_optional,
                    value: r_value,
                },
            ) => l_optional == r_optional && l_value == r_value,
            (
                Self::Indirect {
                    optional: l_optional,
                    value: l_value,
                },
                Self::Indirect {
                    optional: r_optional,
                    value: r_value,
                },
            ) => {
                l_optional == r_optional
                    && l_value.as_ref() as *const parser::Rule
                        == r_value.as_ref() as *const parser::Rule
            }
            _ => false,
        }
    }
}

impl Eq for DotModifierRef {}

#[proc_macro]
pub fn generate_instruction_type(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as gen_impl::GenerateInstructionType);
    let mut result = proc_macro2::TokenStream::new();
    input.emit_arg_types(&mut result);
    input.emit_instruction_type(&mut result);
    input.emit_visit(&mut result);
    input.emit_visit_mut(&mut result);
    input.emit_visit_map(&mut result);
    result.into()
}
