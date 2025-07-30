use either::Either;
use proc_macro2::{Span, TokenStream};
use ptx_parser_macros_impl::parser;
use quote::{format_ident, quote, ToTokens};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::hash_map, hash::Hash, iter, rc::Rc};
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, Ident, ItemEnum, Token, Type, TypePath,
    Variant,
};

// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#vectors
// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#fundamental-types
// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#alternate-floating-point-data-formats
// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#packed-floating-point-data-types
// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#packed-integer-data-types
#[rustfmt::skip]
static POSTFIX_MODIFIERS: &[&str] = &[
    ".v2", ".v4", ".v8",
    ".s8", ".s16", ".s16x2", ".s32", ".s64",
    ".u8", ".u16", ".u16x2", ".u32", ".u64",
    ".f16", ".f16x2", ".f32", ".f64",
    ".b8", ".b16", ".b32", ".b64", ".b128",
    ".pred",
    ".bf16", ".bf16x2", ".e4m3", ".e5m2", ".tf32",
];

static POSTFIX_TYPES: &[&str] = &["ScalarType", "VectorPrefix"];

struct OpcodeDefinitions {
    definitions: Vec<SingleOpcodeDefinition>,
    block_selection: Vec<Vec<(Option<Vec<parser::DotModifier>>, usize)>>,
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
                let mut candidates = definitions[i]
                    .unordered_modifiers
                    .iter()
                    .chain(definitions[i].ordered_modifiers.iter())
                    .filter(|modifier| match modifier {
                        DotModifierRef::Direct {
                            optional: false, ..
                        }
                        | DotModifierRef::Indirect {
                            optional: false, ..
                        } => true,
                        _ => false,
                    })
                    .collect::<Vec<_>>();
                candidates.sort_by_key(|modifier| match modifier {
                    DotModifierRef::Direct { .. } => 1,
                    DotModifierRef::Indirect { value, .. } => value.alternatives.len(),
                });
                // Attempt every modifier
                'check_candidates: for candidate_modifier in candidates {
                    // check all other unselected patterns
                    for j in unselected.iter().copied() {
                        if i == j {
                            continue;
                        }
                        let candidate_set = match candidate_modifier {
                            DotModifierRef::Direct { value, .. } => Either::Left(iter::once(value)),
                            DotModifierRef::Indirect { value, .. } => {
                                Either::Right(value.alternatives.iter())
                            }
                        };
                        for candidate_value in candidate_set {
                            if definitions[j].possible_modifiers.contains(candidate_value) {
                                continue 'check_candidates;
                            }
                        }
                    }
                    // it's unique
                    let candidate_vec = match candidate_modifier {
                        DotModifierRef::Direct { value, .. } => vec![value.clone()],
                        DotModifierRef::Indirect { value, .. } => {
                            value.alternatives.iter().cloned().collect::<Vec<_>>()
                        }
                    };
                    selections[i] = Some((Some(candidate_vec), generation));
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
                    Some((modifier_set, generation)) => {
                        if *generation == current_generation {
                            current_generation_definitions.push((modifier_set.clone(), idx));
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
                let name = &arg.ident.ident();
                let arg_type = if arg.unified {
                    quote! { (ParsedOperandStr<'input>, bool) }
                } else if arg.can_be_negated {
                    quote! { (bool, ParsedOperandStr<'input>) }
                } else {
                    quote! { ParsedOperandStr<'input> }
                };
                if arg.optional {
                    quote! { #name : Option<#arg_type> }
                } else {
                    quote! { #name : #arg_type }
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
                let name = &arg.ident.ident();
                quote! { #name }
            }))
    }

    fn extract_and_insert(
        definitions: &mut FxHashMap<Ident, Vec<SingleOpcodeDefinition>>,
        special_definitions: &mut FxHashMap<Ident, proc_macro2::Group>,
        parser::OpcodeDefinition(pattern_seq, rules): parser::OpcodeDefinition,
    ) {
        let (mut named_rules, mut unnamed_rules) = gather_rules(rules);
        let mut last_opcode = pattern_seq.0.last().unwrap().0 .0.name.clone();
        for (opcode_decl, code_block) in pattern_seq.0.into_iter().rev() {
            let current_opcode = opcode_decl.0.name.clone();
            if last_opcode != current_opcode {
                named_rules = FxHashMap::default();
                unnamed_rules = FxHashMap::default();
            }
            let parser::OpcodeDecl(instruction, arguments) = opcode_decl;
            if code_block.special {
                if !instruction.modifiers.is_empty() || !arguments.0.is_empty() {
                    panic!(
                        "`{}`: no modifiers or arguments are allowed in parser definition.",
                        instruction.name
                    );
                }
                special_definitions.insert(instruction.name, code_block.code);
                continue;
            }
            let mut possible_modifiers = FxHashSet::default();
            let mut unordered_modifiers = instruction
                .modifiers
                .into_iter()
                .map(|parser::MaybeDotModifier { optional, modifier }| {
                    match named_rules.get(&modifier) {
                        Some(alts) => {
                            possible_modifiers.extend(alts.alternatives.iter().cloned());
                            if alts.alternatives.len() == 1 && alts.type_.is_none() {
                                DotModifierRef::Direct {
                                    optional,
                                    value: alts.alternatives[0].clone(),
                                    name: modifier,
                                    type_: alts.type_.clone(),
                                }
                            } else {
                                DotModifierRef::Indirect {
                                    optional,
                                    value: alts.clone(),
                                    name: modifier,
                                }
                            }
                        }
                        None => {
                            let type_ = unnamed_rules.get(&modifier).cloned();
                            possible_modifiers.insert(modifier.clone());
                            DotModifierRef::Direct {
                                optional,
                                value: modifier.clone(),
                                name: modifier,
                                type_,
                            }
                        }
                    }
                })
                .collect::<Vec<_>>();
            let ordered_modifiers = Self::extract_ordered_modifiers(&mut unordered_modifiers);
            let entry = Self {
                possible_modifiers,
                unordered_modifiers,
                ordered_modifiers,
                arguments,
                code_block,
            };
            multihash_extend(definitions, current_opcode.clone(), entry);
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

fn gather_rules(
    rules: Vec<parser::Rule>,
) -> (
    FxHashMap<parser::DotModifier, Rc<parser::Rule>>,
    FxHashMap<parser::DotModifier, Type>,
) {
    let mut named = FxHashMap::default();
    let mut unnamed = FxHashMap::default();
    for rule in rules {
        match rule.modifier {
            Some(ref modifier) => {
                named.insert(modifier.clone(), Rc::new(rule));
            }
            None => unnamed.extend(
                rule.alternatives
                    .into_iter()
                    .map(|alt| (alt, rule.type_.as_ref().unwrap().clone())),
            ),
        }
    }
    (named, unnamed)
}

#[proc_macro]
pub fn derive_parser(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parse_definitions =
        parse_macro_input!(tokens as ptx_parser_macros_impl::parser::ParseDefinitions);
    let mut definitions = FxHashMap::default();
    let mut special_definitions = FxHashMap::default();
    let types = OpcodeDefinitions::get_enum_types(&parse_definitions.definitions);
    let enum_types_tokens = emit_enum_types(types, parse_definitions.additional_enums);
    for definition in parse_definitions.definitions.into_iter() {
        SingleOpcodeDefinition::extract_and_insert(
            &mut definitions,
            &mut special_definitions,
            definition,
        );
    }
    let definitions = definitions
        .into_iter()
        .map(|(k, v)| {
            let v = OpcodeDefinitions::new(&k, v);
            (k, v)
        })
        .collect::<FxHashMap<_, _>>();
    let mut token_enum = parse_definitions.token_type;
    let (all_opcode, all_modifier) = write_definitions_into_tokens(
        &definitions,
        special_definitions.keys(),
        &mut token_enum.variants,
    );
    let token_impl = emit_parse_function(
        &token_enum.ident,
        &definitions,
        &special_definitions,
        all_opcode,
        all_modifier,
    );
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
    special_defs: &FxHashMap<Ident, proc_macro2::Group>,
    all_opcode: Vec<&Ident>,
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
                let code_block = &def.code_block.code;
                let args = def.function_arguments_declarations();
                quote! {
                    fn #fn_name<'input>(state: &mut PtxParserState, #(#args),* ) -> Instruction<ParsedOperandStr<'input>> #code_block
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
                    Some(selection_keys) => {
                        let selection_keys = selection_keys.iter().map(|k| k.dot_capitalized());
                        quote! {
                            else if false #(|| modifiers.iter().any(|(t, _)| *t == #type_name :: #selection_keys))* {
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
                let modifiers = take_while(0.., |(t,_)| Token::modifier(t)).parse_next(stream)?;
                #selectors
            }
        }
        .to_tokens(&mut result);
        result
    }).chain(special_defs.iter().map(|(opcode, code)| {
        let opcode_variant = Ident::new(&capitalize(&opcode.to_string()), opcode.span());
        quote! {
            #opcode_variant => { #code? }
        }
    }));
    let opcodes = all_opcode.into_iter().map(|op_ident| {
        let op = op_ident.to_string();
        let variant = Ident::new(&capitalize(&op), op_ident.span());
        let value = op;
        quote! {
            #type_name :: #variant => Some(#value),
        }
    });
    let modifier_names = iter::once(Ident::new("DotUnified", Span::call_site()))
        .chain(all_modifier.iter().map(|m| m.dot_capitalized()));
    quote! {
        impl<'input> #type_name<'input> {
            fn opcode_text(self) -> Option<&'static str> {
                match self {
                    #(#opcodes)*
                    _ => None
                }
            }

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

        fn parse_instruction<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> winnow::error::PResult<Instruction<ParsedOperandStr<'input>>>
        {
            use winnow::Parser;
            use winnow::token::*;
            use winnow::combinator::*;
            let opcode = any.parse_next(stream)?.0;
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
        match modifier {
            DotModifierRef::Direct { optional, value, type_: None, .. } => {
                let variant = value.dot_capitalized();
                if *optional {
                    quote! {
                        #arg_name = opt(any.verify(|(t, _)| *t == #token_type :: #variant)).parse_next(&mut stream)?.is_some();
                    }
                } else {
                    quote! {
                        any.verify(|(t, _)| *t == #token_type :: #variant).parse_next(&mut stream)?;
                    }
                }
            }
            DotModifierRef::Direct {  optional: false, type_: Some(type_), name, value } => {
                let variable = name.ident();
                let variant = value.dot_capitalized();
                let parsed_variant = value.variant_capitalized();
                quote! {
                    any.verify(|(t, _)| *t == #token_type :: #variant).parse_next(&mut stream)?;
                    #variable = #type_ :: #parsed_variant;
                }
            }
            DotModifierRef::Direct {  optional: true, type_: Some(_), .. } => { todo!() }
            DotModifierRef::Indirect { optional, value, .. } => {
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
                        #arg_name = opt(any.verify_map(|(tok, _)| {
                            Some(match tok {
                                #(#variants)*
                                _ => return None
                            })
                        })).parse_next(&mut stream)?;
                    }
                } else {
                    quote! {
                        #arg_name = any.verify_map(|(tok, _)| {
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
            DotModifierRef::Direct {
                name,
                value,
                type_: None,
                ..
            } => {
                let name = name.ident();
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
            DotModifierRef::Direct {
                name,
                value,
                type_: Some(type_),
                ..
            } => {
                let variable = name.ident();
                let token_variant = value.dot_capitalized();
                let enum_variant = value.variant_capitalized();
                quote! {
                    #token_type :: #token_variant =>  {
                        if #variable.is_some() {
                            #return_error_ref;
                        }
                        #variable = Some(#type_ :: #enum_variant);
                    }
                }
            }
            DotModifierRef::Indirect { value, name, .. } => {
                let variable = name.ident();
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
                    name,
                    type_: None,
                    ..
                } => {
                    let variable = name.ident();
                    quote! {
                        if !#variable {
                            #return_error;
                        }
                    }
                }
                DotModifierRef::Direct {
                    optional: false,
                    name,
                    type_: Some(_),
                    ..
                } => {
                    let variable = name.ident();
                    quote! {
                        let #variable = match #variable {
                            Some(x) => x,
                            None => #return_error
                        };
                    }
                }
                DotModifierRef::Indirect {
                    optional: false,
                    name,
                    ..
                } => {
                    let variable = name.ident();
                    quote! {
                        let #variable = match #variable {
                            Some(x) => x,
                            None => #return_error
                        };
                    }
                }
                DotModifierRef::Direct { optional: true, .. }
                | DotModifierRef::Indirect { optional: true, .. } => TokenStream::new(),
            });
    let (arguments_pattern, arguments_parser) = definition.arguments.0.iter().enumerate().rfold((quote! { () }, quote! { empty }), |(emitted_pattern, emitted_parser), (idx, arg)| {
        let comma = if idx == 0 || arg.pre_pipe {
            quote! { empty }
        } else {
            quote! { any.verify(|(t, _)| *t == #token_type::Comma).void() }
        };

        let pre_bracket = if arg.pre_bracket {
            quote! {
                any.verify(|(t, _)| *t == #token_type::LBracket).void()
            }
        } else {
            quote! {
                empty
            }
        };
        let pre_pipe = if arg.pre_pipe {
            quote! {
                any.verify(|(t, _)| *t == #token_type::Pipe).void()
            }
        } else {
            quote! {
                empty
            }
        };
        let can_be_negated = if arg.can_be_negated {
            quote! {
                opt(any.verify(|(t, _)| *t == #token_type::Exclamation)).map(|o| o.is_some())
            }
        } else {
            quote! {
                empty
            }
        };
        let operand = {
            quote! {
                ParsedOperandStr::parse
            }
        };
        let post_bracket = if arg.post_bracket {
            quote! {
                any.verify(|(t, _)| *t == #token_type::RBracket).void()
            }
        } else {
            quote! {
                empty
            }
        };
        let unified = if arg.unified {
            quote! {
                opt(any.verify(|(t, _)| *t == #token_type::DotUnified).void()).map(|u| u.is_some())
            }
        } else {
            quote! {
                empty
            }
        };
        let pattern = quote! {
            (#comma, #pre_bracket, #pre_pipe, #can_be_negated, #operand, #post_bracket, #unified)
        };
        let arg_name = &arg.ident.ident();
        if arg.unified && arg.can_be_negated {
            panic!("TODO: argument can't be both prefixed by `!` and suffixed by  `.unified`")
        }
        let inner_parser = if arg.unified {
            quote! {
                #pattern.map(|(_, _, _, _, name, _, unified)| (name, unified))
            }
        } else if arg.can_be_negated {
            quote! {
                #pattern.map(|(_, _, _, negated, name, _, _)| (negated, name))
            }
        } else {
            quote! {
                #pattern.map(|(_, _, _, _, name, _, _)| name)
            }
        };

        let parser = if arg.optional {
            quote! { first_optional(#inner_parser, #emitted_parser) }
        } else {
            quote! { (#inner_parser, #emitted_parser) }
        };

        let pattern = quote! { ( #arg_name, #emitted_pattern ) };

        (pattern, parser)
    });

    let arguments_parse =
        quote! { let #arguments_pattern = ( #arguments_parser ).parse_next(stream)?; };

    let fn_args = definition.function_arguments();
    let fn_name = format_ident!("{}_{}", opcode, fn_idx);
    let fn_call = quote! {
        #fn_name(&mut stream.state,  #(#fn_args),* )
    };
    quote! {
        #(#unordered_parse_declarations)*
        #(#ordered_parse_declarations)*
        {
            let mut stream = ReverseStream(modifiers);
            #(#ordered_parse)*
            let mut stream: &[_] = stream.0;
            for (token, _) in stream.iter().cloned() {
                match token {
                    #(#unordered_parse)*
                    _ => #return_error_ref
                }
            }
        }
        #(#unordered_parse_validations)*
        #arguments_parse
        #fn_call
    }
}

fn write_definitions_into_tokens<'a>(
    defs: &'a FxHashMap<Ident, OpcodeDefinitions>,
    special_definitions: impl Iterator<Item = &'a Ident>,
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
    for opcode in special_definitions {
        all_opcodes.push(opcode);
        let opcode_as_string = opcode.to_string();
        let variant_name = Ident::new(&capitalize(&opcode_as_string), opcode.span());
        let arg: Variant = syn::parse_quote! {
            #[token(#opcode_as_string)]
            #variant_name
        };
        variants.push(arg);
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
    variants.push(parse_quote! {
        #[token(".unified")]
        DotUnified
    });
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
        name: parser::DotModifier,
        type_: Option<Type>,
    },
    Indirect {
        optional: bool,
        name: parser::DotModifier,
        value: Rc<parser::Rule>,
    },
}

impl DotModifierRef {
    fn ident(&self) -> Ident {
        match self {
            DotModifierRef::Direct { name, .. } => name.ident(),
            DotModifierRef::Indirect { name, .. } => name.ident(),
        }
    }

    fn type_of(&self) -> Option<syn::Type> {
        Some(match self {
            DotModifierRef::Direct {
                optional: true,
                type_: None,
                ..
            } => syn::parse_quote! { bool },
            DotModifierRef::Direct {
                optional: false,
                type_: None,
                ..
            } => return None,
            DotModifierRef::Direct {
                optional: true,
                type_: Some(type_),
                ..
            } => syn::parse_quote! { Option<#type_> },
            DotModifierRef::Direct {
                optional: false,
                type_: Some(type_),
                ..
            } => type_.clone(),
            DotModifierRef::Indirect {
                optional, value, ..
            } => {
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
            DotModifierRef::Direct { type_: None, .. } => syn::parse_quote! { bool },
            DotModifierRef::Direct {
                type_: Some(type_), ..
            } => syn::parse_quote! { Option<#type_> },
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

#[proc_macro]
pub fn generate_instruction_type(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as ptx_parser_macros_impl::GenerateInstructionType);
    let mut result = proc_macro2::TokenStream::new();
    input.emit_arg_types(&mut result);
    input.emit_instruction_type(&mut result);
    input.emit_visit(&mut result);
    input.emit_visit_mut(&mut result);
    input.emit_visit_map(&mut result);
    result.into()
}
