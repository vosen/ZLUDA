use derive_more::Display;
use logos::Logos;
use ptx_parser_macros::derive_parser;
use rustc_hash::FxHashMap;
use std::fmt::Debug;
use std::iter;
use std::num::{NonZeroU8, ParseFloatError, ParseIntError};
use winnow::ascii::dec_uint;
use winnow::combinator::*;
use winnow::error::{ErrMode, ErrorKind};
use winnow::stream::Accumulate;
use winnow::token::{any, take_till};
use winnow::{
    error::{ContextError, ParserError},
    stream::{Offset, Stream, StreamIsPartial},
    PResult,
};
use winnow::{prelude::*, Stateful};

mod ast;
pub use ast::*;

impl From<RawMulIntControl> for ast::MulIntControl {
    fn from(value: RawMulIntControl) -> Self {
        match value {
            RawMulIntControl::Lo => ast::MulIntControl::Low,
            RawMulIntControl::Hi => ast::MulIntControl::High,
            RawMulIntControl::Wide => ast::MulIntControl::Wide,
        }
    }
}

impl From<RawStCacheOperator> for ast::StCacheOperator {
    fn from(value: RawStCacheOperator) -> Self {
        match value {
            RawStCacheOperator::Wb => ast::StCacheOperator::Writeback,
            RawStCacheOperator::Cg => ast::StCacheOperator::L2Only,
            RawStCacheOperator::Cs => ast::StCacheOperator::Streaming,
            RawStCacheOperator::Wt => ast::StCacheOperator::Writethrough,
        }
    }
}

impl From<RawLdCacheOperator> for ast::LdCacheOperator {
    fn from(value: RawLdCacheOperator) -> Self {
        match value {
            RawLdCacheOperator::Ca => ast::LdCacheOperator::Cached,
            RawLdCacheOperator::Cg => ast::LdCacheOperator::L2Only,
            RawLdCacheOperator::Cs => ast::LdCacheOperator::Streaming,
            RawLdCacheOperator::Lu => ast::LdCacheOperator::LastUse,
            RawLdCacheOperator::Cv => ast::LdCacheOperator::Uncached,
        }
    }
}

impl From<RawLdStQualifier> for ast::LdStQualifier {
    fn from(value: RawLdStQualifier) -> Self {
        match value {
            RawLdStQualifier::Weak => ast::LdStQualifier::Weak,
            RawLdStQualifier::Volatile => ast::LdStQualifier::Volatile,
        }
    }
}

impl From<RawRoundingMode> for ast::RoundingMode {
    fn from(value: RawRoundingMode) -> Self {
        match value {
            RawRoundingMode::Rn | RawRoundingMode::Rni => ast::RoundingMode::NearestEven,
            RawRoundingMode::Rz | RawRoundingMode::Rzi => ast::RoundingMode::Zero,
            RawRoundingMode::Rm | RawRoundingMode::Rmi => ast::RoundingMode::NegativeInf,
            RawRoundingMode::Rp | RawRoundingMode::Rpi => ast::RoundingMode::PositiveInf,
        }
    }
}

impl VectorPrefix {
    pub(crate) fn len(self) -> NonZeroU8 {
        unsafe {
            match self {
                VectorPrefix::V2 => NonZeroU8::new_unchecked(2),
                VectorPrefix::V4 => NonZeroU8::new_unchecked(4),
                VectorPrefix::V8 => NonZeroU8::new_unchecked(8),
            }
        }
    }
}

struct PtxParserState<'a, 'input> {
    text: &'input str,
    errors: &'a mut Vec<PtxError<'input>>,
    function_declarations:
        FxHashMap<&'input str, (Vec<(ast::Type, StateSpace)>, Vec<(ast::Type, StateSpace)>)>,
}

impl<'a, 'input> PtxParserState<'a, 'input> {
    fn new(text: &'input str, errors: &'a mut Vec<PtxError<'input>>) -> Self {
        Self {
            text,
            errors,
            function_declarations: FxHashMap::default(),
        }
    }

    fn record_function(&mut self, function_decl: &MethodDeclaration<'input, &'input str>) {
        let name = match function_decl.name {
            MethodName::Kernel(name) => name,
            MethodName::Func(name) => name,
        };
        let return_arguments = Self::get_type_space(&*function_decl.return_arguments);
        let input_arguments = Self::get_type_space(&*function_decl.input_arguments);
        // TODO: check if declarations match
        self.function_declarations
            .insert(name, (return_arguments, input_arguments));
    }

    fn get_type_space(input_arguments: &[Variable<&str>]) -> Vec<(Type, StateSpace)> {
        input_arguments
            .iter()
            .map(|var| (var.v_type.clone(), var.state_space))
            .collect::<Vec<_>>()
    }
}

impl<'a, 'input> Debug for PtxParserState<'a, 'input> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PtxParserState")
            .field("errors", &self.errors) /*  .field("function_decl", &self.function_decl) */
            .finish()
    }
}

type PtxParser<'a, 'input> =
    Stateful<&'a [(Token<'input>, logos::Span)], PtxParserState<'a, 'input>>;

fn ident<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<&'input str> {
    any.verify_map(|(t, _)| {
        if let Token::Ident(text) = t {
            Some(text)
        } else if let Some(text) = t.opcode_text() {
            Some(text)
        } else {
            None
        }
    })
    .parse_next(stream)
}

fn dot_ident<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<&'input str> {
    any.verify_map(|(t, _)| {
        if let Token::DotIdent(text) = t {
            Some(text)
        } else {
            None
        }
    })
    .parse_next(stream)
}

fn num<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<(&'input str, u32, bool)> {
    any.verify_map(|(t, _)| {
        Some(match t {
            Token::Hex(s) => {
                if s.ends_with('U') {
                    (&s[2..s.len() - 1], 16, true)
                } else {
                    (&s[2..], 16, false)
                }
            }
            Token::Decimal(s) => {
                let radix = if s.starts_with('0') { 8 } else { 10 };
                if s.ends_with('U') {
                    (&s[..s.len() - 1], radix, true)
                } else {
                    (s, radix, false)
                }
            }
            _ => return None,
        })
    })
    .parse_next(stream)
}

fn take_error<'a, 'input: 'a, O, E>(
    mut parser: impl Parser<PtxParser<'a, 'input>, Result<O, (O, PtxError<'input>)>, E>,
) -> impl Parser<PtxParser<'a, 'input>, O, E> {
    move |input: &mut PtxParser<'a, 'input>| {
        Ok(match parser.parse_next(input)? {
            Ok(x) => x,
            Err((x, err)) => {
                input.state.errors.push(err);
                x
            }
        })
    }
}

fn int_immediate<'a, 'input>(input: &mut PtxParser<'a, 'input>) -> PResult<ast::ImmediateValue> {
    take_error((opt(Token::Minus), num).map(|(neg, x)| {
        let (num, radix, is_unsigned) = x;
        if neg.is_some() {
            match i64::from_str_radix(num, radix) {
                Ok(x) => Ok(ast::ImmediateValue::S64(-x)),
                Err(err) => Err((ast::ImmediateValue::S64(0), PtxError::from(err))),
            }
        } else if is_unsigned {
            match u64::from_str_radix(num, radix) {
                Ok(x) => Ok(ast::ImmediateValue::U64(x)),
                Err(err) => Err((ast::ImmediateValue::U64(0), PtxError::from(err))),
            }
        } else {
            match i64::from_str_radix(num, radix) {
                Ok(x) => Ok(ast::ImmediateValue::S64(x)),
                Err(_) => match u64::from_str_radix(num, radix) {
                    Ok(x) => Ok(ast::ImmediateValue::U64(x)),
                    Err(err) => Err((ast::ImmediateValue::U64(0), PtxError::from(err))),
                },
            }
        }
    }))
    .parse_next(input)
}

fn f32<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<f32> {
    take_error(any.verify_map(|(t, _)| match t {
        Token::F32(f) => Some(match u32::from_str_radix(&f[2..], 16) {
            Ok(x) => Ok(f32::from_bits(x)),
            Err(err) => Err((0.0, PtxError::from(err))),
        }),
        _ => None,
    }))
    .parse_next(stream)
}

fn f64<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<f64> {
    take_error(any.verify_map(|(t, _)| match t {
        Token::F64(f) => Some(match u64::from_str_radix(&f[2..], 16) {
            Ok(x) => Ok(f64::from_bits(x)),
            Err(err) => Err((0.0, PtxError::from(err))),
        }),
        _ => None,
    }))
    .parse_next(stream)
}

fn s32<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<i32> {
    take_error((opt(Token::Minus), num).map(|(sign, x)| {
        let (text, radix, _) = x;
        match i32::from_str_radix(text, radix) {
            Ok(x) => Ok(if sign.is_some() { -x } else { x }),
            Err(err) => Err((0, PtxError::from(err))),
        }
    }))
    .parse_next(stream)
}

fn u8<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<u8> {
    take_error(num.map(|x| {
        let (text, radix, _) = x;
        match u8::from_str_radix(text, radix) {
            Ok(x) => Ok(x),
            Err(err) => Err((0, PtxError::from(err))),
        }
    }))
    .parse_next(stream)
}

fn u32<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<u32> {
    take_error(num.map(|x| {
        let (text, radix, _) = x;
        match u32::from_str_radix(text, radix) {
            Ok(x) => Ok(x),
            Err(err) => Err((0, PtxError::from(err))),
        }
    }))
    .parse_next(stream)
}

fn immediate_value<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<ast::ImmediateValue> {
    alt((
        int_immediate,
        f32.map(ast::ImmediateValue::F32),
        f64.map(ast::ImmediateValue::F64),
    ))
    .parse_next(stream)
}

pub fn parse_for_errors<'input>(text: &'input str) -> Vec<PtxError> {
    let (tokens, mut errors) = lex_with_span_unchecked(text);
    let parse_result = {
        let state = PtxParserState::new(text, &mut errors);
        let parser = PtxParser {
            state,
            input: &tokens[..],
        };
        module
            .parse(parser)
            .map_err(|err| PtxError::Parser(err.into_inner()))
    };
    match parse_result {
        Ok(_) => {}
        Err(err) => {
            errors.push(err);
        }
    }
    errors
}

fn lex_with_span_unchecked<'input>(
    text: &'input str,
) -> (Vec<(Token<'input>, logos::Span)>, Vec<PtxError>) {
    let lexer = Token::lexer(text);
    let mut result = Vec::new();
    let mut errors = Vec::new();
    for (token, span) in lexer.spanned() {
        match token {
            Ok(t) => result.push((t, span)),
            Err(err) => errors.push(PtxError::Lexer { source: err }),
        }
    }
    (result, errors)
}

pub fn parse_module_checked<'input>(
    text: &'input str,
) -> Result<ast::Module<'input>, Vec<PtxError>> {
    let mut lexer = Token::lexer(text);
    let mut errors = Vec::new();
    let mut tokens = Vec::new();
    loop {
        let maybe_token = match lexer.next() {
            Some(maybe_token) => maybe_token,
            None => break,
        };
        match maybe_token {
            Ok(token) => tokens.push((token, lexer.span())),
            Err(mut err) => {
                err.0 = lexer.span();
                errors.push(PtxError::from(err))
            }
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    let parse_result = {
        let state = PtxParserState::new(text, &mut errors);
        let parser = PtxParser {
            state,
            input: &tokens[..],
        };
        module
            .parse(parser)
            .map_err(|err| PtxError::Parser(err.into_inner()))
    };
    match parse_result {
        Ok(result) if errors.is_empty() => Ok(result),
        Ok(_) => Err(errors),
        Err(err) => {
            errors.push(err);
            Err(errors)
        }
    }
}

fn module<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<ast::Module<'input>> {
    (
        version,
        target,
        opt(address_size),
        repeat_without_none(directive),
        eof,
    )
        .map(|(version, _, _, directives, _)| ast::Module {
            version,
            directives,
        })
        .parse_next(stream)
}

fn address_size<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<()> {
    (Token::DotAddressSize, u8_literal(64))
        .void()
        .parse_next(stream)
}

fn version<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<(u8, u8)> {
    (Token::DotVersion, u8, Token::Dot, u8)
        .map(|(_, major, _, minor)| (major, minor))
        .parse_next(stream)
}

fn target<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<(u32, Option<char>)> {
    preceded(Token::DotTarget, ident.and_then(shader_model)).parse_next(stream)
}

fn shader_model<'a>(stream: &mut &str) -> PResult<(u32, Option<char>)> {
    (
        "sm_",
        dec_uint,
        opt(any.verify(|c: &char| c.is_ascii_lowercase())),
        eof,
    )
        .map(|(_, digits, arch_variant, _)| (digits, arch_variant))
        .parse_next(stream)
}

fn directive<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<Option<ast::Directive<'input, ast::ParsedOperand<&'input str>>>> {
    with_recovery(
        alt((
            // When adding a new variant here remember to add its first token into recovery parser down below
            function.map(|(linking, func)| Some(ast::Directive::Method(linking, func))),
            file.map(|_| None),
            section.map(|_| None),
            (module_variable, Token::Semicolon)
                .map(|((linking, var), _)| Some(ast::Directive::Variable(linking, var))),
        )),
        take_till(1.., |(token, _)| match token {
            // visibility
            Token::DotExtern | Token::DotVisible | Token::DotWeak
            // methods
            | Token::DotFunc | Token::DotEntry
            // module variables
            | Token::DotGlobal | Token::DotConst | Token::DotShared
            // other sections
            | Token::DotFile | Token::DotSection => true,
            _ => false,
        }),
        PtxError::UnrecognizedDirective,
    )
    .map(Option::flatten)
    .parse_next(stream)
}

fn module_variable<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<(ast::LinkingDirective, ast::Variable<&'input str>)> {
    let linking = linking_directives.parse_next(stream)?;
    let var = global_space
        .flat_map(|space| multi_variable(linking.contains(LinkingDirective::EXTERN), space))
        // TODO: support multi var in globals
        .map(|multi_var| multi_var.var)
        .parse_next(stream)?;
    Ok((linking, var))
}

fn file<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<()> {
    (
        Token::DotFile,
        u32,
        Token::String,
        opt((Token::Comma, u32, Token::Comma, u32)),
    )
        .void()
        .parse_next(stream)
}

fn section<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<()> {
    (
        Token::DotSection.void(),
        dot_ident.void(),
        Token::LBrace.void(),
        repeat::<_, _, (), _, _>(0.., section_dwarf_line),
        Token::RBrace.void(),
    )
        .void()
        .parse_next(stream)
}

fn section_dwarf_line<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<()> {
    alt((
        (section_label, Token::Colon).void(),
        (Token::DotB32, section_label, opt((Token::Add, u32))).void(),
        (Token::DotB64, section_label, opt((Token::Add, u32))).void(),
        (
            any_bit_type,
            separated::<_, _, (), _, _, _, _>(1.., u32, Token::Comma),
        )
            .void(),
    ))
    .parse_next(stream)
}

fn any_bit_type<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<()> {
    alt((Token::DotB8, Token::DotB16, Token::DotB32, Token::DotB64))
        .void()
        .parse_next(stream)
}

fn section_label<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<()> {
    alt((ident, dot_ident)).void().parse_next(stream)
}

fn function<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<(
    ast::LinkingDirective,
    ast::Function<'input, &'input str, ast::Statement<ParsedOperand<&'input str>>>,
)> {
    let (linking, function) = (
        linking_directives,
        method_declaration,
        repeat(0.., tuning_directive),
        function_body,
    )
        .map(|(linking, func_directive, tuning, body)| {
            (
                linking,
                ast::Function {
                    func_directive,
                    tuning,
                    body,
                },
            )
        })
        .parse_next(stream)?;
    stream.state.record_function(&function.func_directive);
    Ok((linking, function))
}

fn linking_directives<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::LinkingDirective> {
    repeat(
        0..,
        dispatch! { any;
            (Token::DotExtern, _) => empty.value(ast::LinkingDirective::EXTERN),
            (Token::DotVisible, _) => empty.value(ast::LinkingDirective::VISIBLE),
            (Token::DotWeak, _) => empty.value(ast::LinkingDirective::WEAK),
            _ => fail
        },
    )
    .fold(|| ast::LinkingDirective::NONE, |x, y| x | y)
    .parse_next(stream)
}

fn tuning_directive<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::TuningDirective> {
    dispatch! {any;
        (Token::DotMaxnreg, _) => u32.map(ast::TuningDirective::MaxNReg),
        (Token::DotMaxntid, _) => tuple1to3_u32.map(|(nx, ny, nz)| ast::TuningDirective::MaxNtid(nx, ny, nz)),
        (Token::DotReqntid, _) => tuple1to3_u32.map(|(nx, ny, nz)| ast::TuningDirective::ReqNtid(nx, ny, nz)),
        (Token::DotMinnctapersm, _) => u32.map(ast::TuningDirective::MinNCtaPerSm),
        _ => fail
    }
    .parse_next(stream)
}

fn method_declaration<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::MethodDeclaration<'input, &'input str>> {
    dispatch! {any;
        (Token::DotEntry, _) => (ident, kernel_arguments).map(|(name, input_arguments)| ast::MethodDeclaration{
            return_arguments: Vec::new(), name: ast::MethodName::Kernel(name), input_arguments, shared_mem: None
        }),
        (Token::DotFunc, _) => (opt(fn_arguments), ident, fn_arguments).map(|(return_arguments, name,input_arguments)| {
            let return_arguments = return_arguments.unwrap_or_else(|| Vec::new());
            let name = ast::MethodName::Func(name);
            ast::MethodDeclaration{ return_arguments, name, input_arguments, shared_mem: None }
        }),
        _ => fail
    }
    .parse_next(stream)
}

fn fn_arguments<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<Vec<ast::Variable<&'input str>>> {
    delimited(
        Token::LParen,
        separated(0.., fn_input, Token::Comma),
        Token::RParen,
    )
    .parse_next(stream)
}

fn kernel_arguments<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<Vec<ast::Variable<&'input str>>> {
    delimited(
        Token::LParen,
        separated(0.., kernel_input, Token::Comma),
        Token::RParen,
    )
    .parse_next(stream)
}

fn kernel_input<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Variable<&'input str>> {
    preceded(Token::DotParam, method_parameter(StateSpace::Param)).parse_next(stream)
}

fn fn_input<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<ast::Variable<&'input str>> {
    dispatch! { any;
        (Token::DotParam, _) => method_parameter(StateSpace::Param),
        (Token::DotReg, _) => method_parameter(StateSpace::Reg),
        _ => fail
    }
    .parse_next(stream)
}

fn tuple1to3_u32<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<(u32, u32, u32)> {
    struct Tuple3AccumulateU32 {
        index: usize,
        value: (u32, u32, u32),
    }

    impl Accumulate<u32> for Tuple3AccumulateU32 {
        fn initial(_: Option<usize>) -> Self {
            Self {
                index: 0,
                value: (1, 1, 1),
            }
        }

        fn accumulate(&mut self, value: u32) {
            match self.index {
                0 => {
                    self.value = (value, self.value.1, self.value.2);
                    self.index = 1;
                }
                1 => {
                    self.value = (self.value.0, value, self.value.2);
                    self.index = 2;
                }
                2 => {
                    self.value = (self.value.0, self.value.1, value);
                    self.index = 3;
                }
                _ => unreachable!(),
            }
        }
    }

    separated::<_, _, Tuple3AccumulateU32, _, _, _, _>(1..=3, u32, Token::Comma)
        .map(|acc| acc.value)
        .parse_next(stream)
}

fn function_body<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<Option<Vec<ast::Statement<ParsedOperandStr<'input>>>>> {
    dispatch! {any;
        (Token::LBrace, _) => terminated(repeat_without_none(statement), Token::RBrace).map(Some),
        (Token::Semicolon, _) => empty.map(|_| None),
        _ => fail
    }
    .parse_next(stream)
}

fn statement<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<Option<Statement<ParsedOperandStr<'input>>>> {
    with_recovery(
        alt((
            label.map(Some),
            debug_directive.map(|_| None),
            terminated(
                method_space
                    .flat_map(|space| multi_variable(false, space))
                    .map(|var| Some(Statement::Variable(var))),
                Token::Semicolon,
            ),
            predicated_instruction.map(Some),
            pragma.map(|_| None),
            block_statement.map(Some),
        )),
        take_till_inclusive(
            |(t, _)| *t == Token::RBrace,
            |(t, _)| match t {
                Token::Semicolon | Token::Colon => true,
                _ => false,
            },
        ),
        PtxError::UnrecognizedStatement,
    )
    .map(Option::flatten)
    .parse_next(stream)
}

fn take_till_inclusive<I: Stream, E: ParserError<I>>(
    backtrack_token: impl Fn(&I::Token) -> bool,
    end_token: impl Fn(&I::Token) -> bool,
) -> impl Parser<I, <I as Stream>::Slice, E> {
    fn get_offset<I: Stream>(
        input: &mut I,
        backtrack_token: &impl Fn(&I::Token) -> bool,
        end_token: &impl Fn(&I::Token) -> bool,
        should_backtrack: &mut bool,
    ) -> usize {
        *should_backtrack = false;
        let mut hit = false;
        for (offset, token) in input.iter_offsets() {
            if hit {
                return offset;
            } else {
                if backtrack_token(&token) {
                    *should_backtrack = true;
                    return offset;
                }
                if end_token(&token) {
                    hit = true;
                }
            }
        }
        input.eof_offset()
    }
    move |stream: &mut I| {
        let mut should_backtrack = false;
        let offset = get_offset(stream, &backtrack_token, &end_token, &mut should_backtrack);
        let result = stream.next_slice(offset);
        if should_backtrack {
            Err(ErrMode::from_error_kind(
                stream,
                winnow::error::ErrorKind::Token,
            ))
        } else {
            Ok(result)
        }
    }
}

/*
pub fn take_till_or_backtrack_eof<Set, Input, Error>(
    set: Set,
) -> impl Parser<Input, <Input as Stream>::Slice, Error>
where
    Input: StreamIsPartial + Stream,
    Set: winnow::stream::ContainsToken<<Input as Stream>::Token>,
    Error: ParserError<Input>,
{
    move |stream: &mut Input| {
        if stream.eof_offset() == 0 {
            return ;
        }
        take_till(0.., set)
    }
}
     */

fn with_recovery<'a, 'input: 'a, T>(
    mut parser: impl Parser<PtxParser<'a, 'input>, T, ContextError>,
    mut recovery: impl Parser<PtxParser<'a, 'input>, &'a [(Token<'input>, logos::Span)], ContextError>,
    mut error: impl FnMut(Option<&'input str>) -> PtxError<'input>,
) -> impl Parser<PtxParser<'a, 'input>, Option<T>, ContextError> {
    move |stream: &mut PtxParser<'a, 'input>| {
        let input_start = stream.input.first().map(|(_, s)| s).cloned();
        let stream_start = stream.checkpoint();
        match parser.parse_next(stream) {
            Ok(value) => Ok(Some(value)),
            Err(ErrMode::Backtrack(_)) => {
                stream.reset(&stream_start);
                let tokens = recovery.parse_next(stream)?;
                let range = match input_start {
                    Some(start) => {
                        Some(&stream.state.text[start.start..tokens.last().unwrap().1.end])
                    }
                    // We could handle `(Some(start), None)``, but this whole error recovery is to
                    // recover from unknown instructions, so we don't care about early end of stream
                    _ => None,
                };
                stream.state.errors.push(error(range));
                Ok(None)
            }
            Err(err) => Err(err),
        }
    }
}

fn pragma<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<()> {
    (Token::DotPragma, Token::String, Token::Semicolon)
        .void()
        .parse_next(stream)
}

fn method_parameter<'a, 'input: 'a>(
    state_space: StateSpace,
) -> impl Parser<PtxParser<'a, 'input>, Variable<&'input str>, ContextError> {
    move |stream: &mut PtxParser<'a, 'input>| {
        let (align, vector, type_, name) = variable_declaration.parse_next(stream)?;
        let array_dimensions = if state_space != StateSpace::Reg {
            opt(array_dimensions).parse_next(stream)?
        } else {
            None
        };
        // TODO: push this check into array_dimensions(...)
        if let Some(ref dims) = array_dimensions {
            if dims[0] == 0 {
                return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
            }
        }
        Ok(Variable {
            align,
            v_type: Type::maybe_array(vector, type_, array_dimensions),
            state_space,
            name,
            array_init: Vec::new(),
        })
    }
}

// TODO: split to a separate type
fn variable_declaration<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<(Option<u32>, Option<NonZeroU8>, ScalarType, &'input str)> {
    (
        opt(align.verify(|x| x.count_ones() == 1)),
        vector_prefix,
        scalar_type,
        ident,
    )
        .parse_next(stream)
}

fn multi_variable<'a, 'input: 'a>(
    extern_: bool,
    state_space: StateSpace,
) -> impl Parser<PtxParser<'a, 'input>, MultiVariable<&'input str>, ContextError> {
    move |stream: &mut PtxParser<'a, 'input>| {
        let ((align, vector, type_, name), count) = (
            variable_declaration,
            // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#parameterized-variable-names
            opt(delimited(Token::Lt, u32.verify(|x| *x != 0), Token::Gt)),
        )
            .parse_next(stream)?;
        if count.is_some() {
            return Ok(MultiVariable {
                var: Variable {
                    align,
                    v_type: Type::maybe_vector_parsed(vector, type_),
                    state_space,
                    name,
                    array_init: Vec::new(),
                },
                count,
            });
        }
        let mut array_dimensions = if state_space != StateSpace::Reg {
            opt(array_dimensions).parse_next(stream)?
        } else {
            None
        };
        let initializer = match state_space {
            StateSpace::Global | StateSpace::Const => match array_dimensions {
                Some(ref mut dimensions) => {
                    opt(array_initializer(vector, type_, dimensions)).parse_next(stream)?
                }
                None => opt(value_initializer(vector, type_)).parse_next(stream)?,
            },
            _ => None,
        };
        if let Some(ref dims) = array_dimensions {
            if !extern_ && dims[0] == 0 {
                return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
            }
        }
        Ok(MultiVariable {
            var: Variable {
                align,
                v_type: Type::maybe_array(vector, type_, array_dimensions),
                state_space,
                name,
                array_init: initializer.unwrap_or(Vec::new()),
            },
            count,
        })
    }
}

fn array_initializer<'a, 'input: 'a>(
    vector: Option<NonZeroU8>,
    type_: ScalarType,
    array_dimensions: &mut Vec<u32>,
) -> impl Parser<PtxParser<'a, 'input>, Vec<u8>, ContextError> + '_ {
    move |stream: &mut PtxParser<'a, 'input>| {
        Token::Eq.parse_next(stream)?;
        let mut result = Vec::new();
        // TODO: vector constants and multi dim arrays
        if vector.is_some() || array_dimensions[0] == 0 || array_dimensions.len() > 1 {
            return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
        }
        delimited(
            Token::LBrace,
            separated::<_, (), (), _, _, _, _>(
                0..=array_dimensions[0] as usize,
                single_value_append(&mut result, type_),
                Token::Comma,
            ),
            Token::RBrace,
        )
        .parse_next(stream)?;
        // pad with zeros
        let result_size = type_.size_of() as usize * array_dimensions[0] as usize;
        result.extend(iter::repeat(0u8).take(result_size - result.len()));
        Ok(result)
    }
}

fn value_initializer<'a, 'input: 'a>(
    vector: Option<NonZeroU8>,
    type_: ScalarType,
) -> impl Parser<PtxParser<'a, 'input>, Vec<u8>, ContextError> {
    move |stream: &mut PtxParser<'a, 'input>| {
        Token::Eq.parse_next(stream)?;
        let mut result = Vec::new();
        // TODO: vector constants
        if vector.is_some() {
            return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
        }
        single_value_append(&mut result, type_).parse_next(stream)?;
        Ok(result)
    }
}

fn single_value_append<'a, 'input: 'a>(
    accumulator: &mut Vec<u8>,
    type_: ScalarType,
) -> impl Parser<PtxParser<'a, 'input>, (), ContextError> + '_ {
    move |stream: &mut PtxParser<'a, 'input>| {
        let value = immediate_value.parse_next(stream)?;
        match (type_, value) {
            (ScalarType::U8 | ScalarType::B8, ImmediateValue::U64(x)) => {
                accumulator.extend_from_slice(&(x as u8).to_le_bytes())
            }
            (ScalarType::U8 | ScalarType::B8, ImmediateValue::S64(x)) => {
                accumulator.extend_from_slice(&(x as u8).to_le_bytes())
            }
            (ScalarType::U16 | ScalarType::B16, ImmediateValue::U64(x)) => {
                accumulator.extend_from_slice(&(x as u16).to_le_bytes())
            }
            (ScalarType::U16 | ScalarType::B16, ImmediateValue::S64(x)) => {
                accumulator.extend_from_slice(&(x as u16).to_le_bytes())
            }
            (ScalarType::U32 | ScalarType::B32, ImmediateValue::U64(x)) => {
                accumulator.extend_from_slice(&(x as u32).to_le_bytes())
            }
            (ScalarType::U32 | ScalarType::B32, ImmediateValue::S64(x)) => {
                accumulator.extend_from_slice(&(x as u32).to_le_bytes())
            }
            (ScalarType::U64 | ScalarType::B64, ImmediateValue::U64(x)) => {
                accumulator.extend_from_slice(&(x as u64).to_le_bytes())
            }
            (ScalarType::U64 | ScalarType::B64, ImmediateValue::S64(x)) => {
                accumulator.extend_from_slice(&(x as u64).to_le_bytes())
            }
            (ScalarType::S8, ImmediateValue::U64(x)) => {
                accumulator.extend_from_slice(&(x as i8).to_le_bytes())
            }
            (ScalarType::S8, ImmediateValue::S64(x)) => {
                accumulator.extend_from_slice(&(x as i8).to_le_bytes())
            }
            (ScalarType::S16, ImmediateValue::U64(x)) => {
                accumulator.extend_from_slice(&(x as i16).to_le_bytes())
            }
            (ScalarType::S16, ImmediateValue::S64(x)) => {
                accumulator.extend_from_slice(&(x as i16).to_le_bytes())
            }
            (ScalarType::S32, ImmediateValue::U64(x)) => {
                accumulator.extend_from_slice(&(x as i32).to_le_bytes())
            }
            (ScalarType::S32, ImmediateValue::S64(x)) => {
                accumulator.extend_from_slice(&(x as i32).to_le_bytes())
            }
            (ScalarType::S64, ImmediateValue::U64(x)) => {
                accumulator.extend_from_slice(&(x as i64).to_le_bytes())
            }
            (ScalarType::S64, ImmediateValue::S64(x)) => {
                accumulator.extend_from_slice(&(x as i64).to_le_bytes())
            }
            (ScalarType::F32, ImmediateValue::F32(x)) => {
                accumulator.extend_from_slice(&x.to_le_bytes())
            }
            (ScalarType::F64, ImmediateValue::F64(x)) => {
                accumulator.extend_from_slice(&x.to_le_bytes())
            }
            _ => return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify)),
        }
        Ok(())
    }
}

fn array_dimensions<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<Vec<u32>> {
    let dimension = delimited(
        Token::LBracket,
        opt(u32).verify(|dim| *dim != Some(0)),
        Token::RBracket,
    )
    .parse_next(stream)?;
    let result = vec![dimension.unwrap_or(0)];
    repeat_fold_0_or_more(
        delimited(
            Token::LBracket,
            u32.verify(|dim| *dim != 0),
            Token::RBracket,
        ),
        move || result,
        |mut result: Vec<u32>, x| {
            result.push(x);
            result
        },
        stream,
    )
}

// Copied and fixed from Winnow sources (fold_repeat0_)
// Winnow Repeat::fold takes FnMut() -> Result to initalize accumulator,
// this really should be FnOnce() -> Result
fn repeat_fold_0_or_more<I, O, E, F, G, H, R>(
    mut f: F,
    init: H,
    mut g: G,
    input: &mut I,
) -> PResult<R, E>
where
    I: Stream,
    F: Parser<I, O, E>,
    G: FnMut(R, O) -> R,
    H: FnOnce() -> R,
    E: ParserError<I>,
{
    use winnow::error::ErrMode;
    let mut res = init();
    loop {
        let start = input.checkpoint();
        match f.parse_next(input) {
            Ok(o) => {
                res = g(res, o);
            }
            Err(ErrMode::Backtrack(_)) => {
                input.reset(&start);
                return Ok(res);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

fn global_space<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<StateSpace> {
    alt((
        Token::DotGlobal.value(StateSpace::Global),
        Token::DotConst.value(StateSpace::Const),
        Token::DotShared.value(StateSpace::Shared),
    ))
    .parse_next(stream)
}

fn method_space<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<StateSpace> {
    alt((
        Token::DotReg.value(StateSpace::Reg),
        Token::DotLocal.value(StateSpace::Local),
        Token::DotParam.value(StateSpace::Param),
        global_space,
    ))
    .parse_next(stream)
}

fn align<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<u32> {
    preceded(Token::DotAlign, u32).parse_next(stream)
}

fn vector_prefix<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<Option<NonZeroU8>> {
    opt(alt((
        Token::DotV2.value(unsafe { NonZeroU8::new_unchecked(2) }),
        Token::DotV4.value(unsafe { NonZeroU8::new_unchecked(4) }),
        Token::DotV8.value(unsafe { NonZeroU8::new_unchecked(8) }),
    )))
    .parse_next(stream)
}

fn scalar_type<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<ScalarType> {
    any.verify_map(|(t, _)| {
        Some(match t {
            Token::DotS8 => ScalarType::S8,
            Token::DotS16 => ScalarType::S16,
            Token::DotS16x2 => ScalarType::S16x2,
            Token::DotS32 => ScalarType::S32,
            Token::DotS64 => ScalarType::S64,
            Token::DotU8 => ScalarType::U8,
            Token::DotU16 => ScalarType::U16,
            Token::DotU16x2 => ScalarType::U16x2,
            Token::DotU32 => ScalarType::U32,
            Token::DotU64 => ScalarType::U64,
            Token::DotB8 => ScalarType::B8,
            Token::DotB16 => ScalarType::B16,
            Token::DotB32 => ScalarType::B32,
            Token::DotB64 => ScalarType::B64,
            Token::DotB128 => ScalarType::B128,
            Token::DotPred => ScalarType::Pred,
            Token::DotF16 => ScalarType::F16,
            Token::DotF16x2 => ScalarType::F16x2,
            Token::DotF32 => ScalarType::F32,
            Token::DotF64 => ScalarType::F64,
            Token::DotBF16 => ScalarType::BF16,
            Token::DotBF16x2 => ScalarType::BF16x2,
            _ => return None,
        })
    })
    .parse_next(stream)
}

fn predicated_instruction<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Statement<ParsedOperandStr<'input>>> {
    (opt(pred_at), parse_instruction, Token::Semicolon)
        .map(|(p, i, _)| ast::Statement::Instruction(p, i))
        .parse_next(stream)
}

fn pred_at<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<ast::PredAt<&'input str>> {
    (Token::At, opt(Token::Exclamation), ident)
        .map(|(_, not, label)| ast::PredAt {
            not: not.is_some(),
            label,
        })
        .parse_next(stream)
}

fn label<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Statement<ParsedOperandStr<'input>>> {
    terminated(ident, Token::Colon)
        .map(|l| ast::Statement::Label(l))
        .parse_next(stream)
}

fn debug_directive<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<()> {
    (
        Token::DotLoc,
        u32,
        u32,
        u32,
        opt((
            Token::Comma,
            ident_literal("function_name"),
            ident,
            dispatch! { any;
                (Token::Comma, _) => (ident_literal("inlined_at"), u32, u32, u32).void(),
                (Token::Plus, _) => (u32, Token::Comma, ident_literal("inlined_at"), u32, u32, u32).void(),
                _ => fail
            },
        )),
    )
        .void()
        .parse_next(stream)
}

fn block_statement<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Statement<ParsedOperandStr<'input>>> {
    delimited(Token::LBrace, repeat_without_none(statement), Token::RBrace)
        .map(|s| ast::Statement::Block(s))
        .parse_next(stream)
}

fn repeat_without_none<Input: Stream, Output, Error: ParserError<Input>>(
    parser: impl Parser<Input, Option<Output>, Error>,
) -> impl Parser<Input, Vec<Output>, Error> {
    repeat(0.., parser).fold(Vec::new, |mut acc: Vec<_>, item| {
        if let Some(item) = item {
            acc.push(item);
        }
        acc
    })
}

fn ident_literal<
    'a,
    'input,
    X,
    I: Stream<Token = (Token<'input>, X)> + StreamIsPartial,
    E: ParserError<I>,
>(
    s: &'input str,
) -> impl Parser<I, (), E> + 'input {
    move |stream: &mut I| {
        any.verify(|(t, _)| matches!(t, Token::Ident(text) if *text == s))
            .void()
            .parse_next(stream)
    }
}

fn u8_literal<'a, 'input>(x: u8) -> impl Parser<PtxParser<'a, 'input>, (), ContextError> {
    move |stream: &mut PtxParser| u8.verify(|t| *t == x).void().parse_next(stream)
}

impl<Ident> ast::ParsedOperand<Ident> {
    fn parse<'a, 'input>(
        stream: &mut PtxParser<'a, 'input>,
    ) -> PResult<ast::ParsedOperand<&'input str>> {
        use winnow::combinator::*;
        use winnow::token::any;
        fn vector_index<'input>(inp: &'input str) -> Result<u8, PtxError> {
            match inp {
                ".x" | ".r" => Ok(0),
                ".y" | ".g" => Ok(1),
                ".z" | ".b" => Ok(2),
                ".w" | ".a" => Ok(3),
                _ => Err(PtxError::WrongVectorElement),
            }
        }
        fn ident_operands<'a, 'input>(
            stream: &mut PtxParser<'a, 'input>,
        ) -> PResult<ast::ParsedOperand<&'input str>> {
            let main_ident = ident.parse_next(stream)?;
            alt((
                preceded(Token::Plus, s32)
                    .map(move |offset| ast::ParsedOperand::RegOffset(main_ident, offset)),
                take_error(dot_ident.map(move |suffix| {
                    let vector_index = vector_index(suffix)
                        .map_err(move |e| (ast::ParsedOperand::VecMember(main_ident, 0), e))?;
                    Ok(ast::ParsedOperand::VecMember(main_ident, vector_index))
                })),
                empty.value(ast::ParsedOperand::Reg(main_ident)),
            ))
            .parse_next(stream)
        }
        fn vector_operand<'a, 'input>(
            stream: &mut PtxParser<'a, 'input>,
        ) -> PResult<Vec<&'input str>> {
            let (_, r1, _, r2) = (Token::LBrace, ident, Token::Comma, ident).parse_next(stream)?;
            // TODO: parse .v8 literals
            dispatch! {any;
                (Token::RBrace, _) => empty.map(|_| vec![r1, r2]),
                (Token::Comma, _) => (ident, Token::Comma, ident, Token::RBrace).map(|(r3, _, r4, _)| vec![r1, r2, r3, r4]),
                _ => fail
            }
            .parse_next(stream)
        }
        alt((
            ident_operands,
            immediate_value.map(ast::ParsedOperand::Imm),
            vector_operand.map(ast::ParsedOperand::VecPack),
        ))
        .parse_next(stream)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PtxError<'input> {
    #[error("{source}")]
    ParseInt {
        #[from]
        source: ParseIntError,
    },
    #[error("{source}")]
    ParseFloat {
        #[from]
        source: ParseFloatError,
    },
    #[error("{source}")]
    Lexer {
        #[from]
        source: TokenError,
    },
    #[error("")]
    Parser(ContextError),
    #[error("")]
    Todo,
    #[error("")]
    SyntaxError,
    #[error("")]
    NonF32Ftz,
    #[error("")]
    Unsupported32Bit,
    #[error("")]
    WrongType,
    #[error("")]
    UnknownFunction,
    #[error("")]
    MalformedCall,
    #[error("")]
    WrongArrayType,
    #[error("")]
    WrongVectorElement,
    #[error("")]
    MultiArrayVariable,
    #[error("")]
    ZeroDimensionArray,
    #[error("")]
    ArrayInitalizer,
    #[error("")]
    NonExternPointer,
    #[error("{0:?}")]
    UnrecognizedStatement(Option<&'input str>),
    #[error("{0:?}")]
    UnrecognizedDirective(Option<&'input str>),
}

#[derive(Debug)]
struct ReverseStream<'a, T>(pub &'a [T]);

impl<'i, T> Stream for ReverseStream<'i, T>
where
    T: Clone + ::std::fmt::Debug,
{
    type Token = T;
    type Slice = &'i [T];

    type IterOffsets =
        std::iter::Enumerate<std::iter::Cloned<std::iter::Rev<std::slice::Iter<'i, T>>>>;

    type Checkpoint = &'i [T];

    fn iter_offsets(&self) -> Self::IterOffsets {
        self.0.iter().rev().cloned().enumerate()
    }

    fn eof_offset(&self) -> usize {
        self.0.len()
    }

    fn next_token(&mut self) -> Option<Self::Token> {
        let (token, next) = self.0.split_last()?;
        self.0 = next;
        Some(token.clone())
    }

    fn offset_for<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Token) -> bool,
    {
        self.0.iter().rev().position(|b| predicate(b.clone()))
    }

    fn offset_at(&self, tokens: usize) -> Result<usize, winnow::error::Needed> {
        if let Some(needed) = tokens
            .checked_sub(self.0.len())
            .and_then(std::num::NonZeroUsize::new)
        {
            Err(winnow::error::Needed::Size(needed))
        } else {
            Ok(tokens)
        }
    }

    fn next_slice(&mut self, offset: usize) -> Self::Slice {
        let offset = self.0.len() - offset;
        let (next, slice) = self.0.split_at(offset);
        self.0 = next;
        slice
    }

    fn checkpoint(&self) -> Self::Checkpoint {
        self.0
    }

    fn reset(&mut self, checkpoint: &Self::Checkpoint) {
        self.0 = checkpoint;
    }

    fn raw(&self) -> &dyn std::fmt::Debug {
        self
    }
}

impl<'a, T> Offset<&'a [T]> for ReverseStream<'a, T> {
    fn offset_from(&self, start: &&'a [T]) -> usize {
        let fst = start.as_ptr();
        let snd = self.0.as_ptr();

        debug_assert!(
            snd <= fst,
            "`Offset::offset_from({snd:?}, {fst:?})` only accepts slices of `self`"
        );
        (fst as usize - snd as usize) / std::mem::size_of::<T>()
    }
}

impl<'a, T> StreamIsPartial for ReverseStream<'a, T> {
    type PartialState = ();

    fn complete(&mut self) -> Self::PartialState {}

    fn restore_partial(&mut self, _state: Self::PartialState) {}

    fn is_partial_supported() -> bool {
        false
    }
}

impl<'input, X, I: Stream<Token = (Self, X)> + StreamIsPartial, E: ParserError<I>>
    Parser<I, (Self, X), E> for Token<'input>
{
    fn parse_next(&mut self, input: &mut I) -> PResult<(Self, X), E> {
        any.verify(|(t, _)| t == self).parse_next(input)
    }
}

fn bra<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    preceded(
        opt(Token::DotUni),
        any.verify_map(|(t, _)| match t {
            Token::Ident(ident) => Some(ast::Instruction::Bra {
                arguments: BraArgs { src: ident },
            }),
            _ => None,
        }),
    )
    .parse_next(stream)
}

fn call<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let (uni, return_arguments, name, input_arguments) = (
        opt(Token::DotUni),
        opt((
            Token::LParen,
            separated(1.., ident, Token::Comma).map(|x: Vec<_>| x),
            Token::RParen,
            Token::Comma,
        )
            .map(|(_, arguments, _, _)| arguments)),
        ident,
        opt((
            Token::Comma.void(),
            Token::LParen.void(),
            separated(1.., ParsedOperand::<&'input str>::parse, Token::Comma).map(|x: Vec<_>| x),
            Token::RParen.void(),
        )
            .map(|(_, _, arguments, _)| arguments)),
    )
        .parse_next(stream)?;
    let uniform = uni.is_some();
    let recorded_fn = match stream.state.function_declarations.get(name) {
        Some(decl) => decl,
        None => {
            stream.state.errors.push(PtxError::UnknownFunction);
            return Ok(empty_call(uniform, name));
        }
    };
    let return_arguments = return_arguments.unwrap_or(Vec::new());
    let input_arguments = input_arguments.unwrap_or(Vec::new());
    if recorded_fn.0.len() != return_arguments.len() || recorded_fn.1.len() != input_arguments.len()
    {
        stream.state.errors.push(PtxError::MalformedCall);
        return Ok(empty_call(uniform, name));
    }
    let data = CallDetails {
        uniform,
        return_arguments: recorded_fn.0.clone(),
        input_arguments: recorded_fn.1.clone(),
    };
    let arguments = CallArgs {
        return_arguments,
        func: name,
        input_arguments,
    };
    Ok(ast::Instruction::Call { data, arguments })
}

fn empty_call<'input>(
    uniform: bool,
    name: &'input str,
) -> ast::Instruction<ParsedOperandStr<'input>> {
    ast::Instruction::Call {
        data: CallDetails {
            uniform,
            return_arguments: Vec::new(),
            input_arguments: Vec::new(),
        },
        arguments: CallArgs {
            return_arguments: Vec::new(),
            func: name,
            input_arguments: Vec::new(),
        },
    }
}

type ParsedOperandStr<'input> = ast::ParsedOperand<&'input str>;

#[derive(Clone, PartialEq, Default, Debug, Display)]
#[display("({}:{})", _0.start, _0.end)]
pub struct TokenError(std::ops::Range<usize>);

impl std::error::Error for TokenError {}

// This macro is responsible for generating parser code for instruction parser.
// Instruction parsing is by far the most complex part of parsing PTX code:
// * There are tens of instruction kinds, each with slightly different parsing rules
// * After parsing, each instruction needs to do some early validation and generate a specific,
//   strongly-typed object. We want strong-typing because we have a single PTX parser frontend, but
//   there can be multiple different code emitter backends
// * Most importantly, instruction modifiers  can come in aby order, so e.g. both
//   `ld.relaxed.global.u32 a, b` and `ld.global.relaxed.u32 a, b` are equally valid. This makes
//   classic parsing generators fail: if we tried to generate parsing rules that cover every possible
//   ordering we'd need thousands of rules. This is not a purely theoretical problem. NVCC and Clang
//   will always emit modifiers in the correct order, but people who write inline assembly usually
//   get it wrong (even first party developers)
//
// This macro exists purely to generate repetitive code for parsing each instruction. It is
// _not_ self-contained and is _not_ general-purpose: it relies on certain types and functions from
// the enclosing module
//
// derive_parser!(...) input is split into three parts:
// * Token type definition
// * Partial enums
// * Parsing definitions
//
// Token type definition:
// This is the enum type that will be usesby the instruction parser. For every instruction and
// modifier, derive_parser!(...) will add appropriate variant into this type. So e.g. if there is a
// rule for for `bar.sync` then those two variants wil be appended to the Token enum:
// #[token("bar")] Bar,
// #[token(".sync")] DotSync,
//
// Partial enums:
// With proper annotations, derive_parser!(...) parsing definitions are able to interpret
// instruction modifiers as variants of a single enum type. So e.g. for definitions `ld.u32` and
// `ld.u64` the macro can generate `enum ScalarType { U32, U64 }`. The problem is that for some
// (but not all) of those generated enum types we want to add some attributes and additional
// variants. In order to do so, you need to define this enum and derive_parser!(...) will append to
// the type instead of creating a new type. This is sort of replacement for partial classes known
// from C#
//
// Parsing definitions:
// Parsing definitions consist of a list of patterns and rules:
// * Pattern consists of:
//   * Opcode: `ld`
//   * Modifiers, always start with a dot: `.global`, `.relaxed`. Optionals are enclosed in braces
//   * Arguments: `a`, `b`. Optionals are enclosed in braces
//   * Code block: => { <code expression> }. Code blocks implictly take all modifiers ansd arguments
//     as parameters. All modifiers and arguments are passed to the code block:
//     * If it is an alternative (as defined in rules list later):
//       * If it is mandatory then its type is Foo (as defined by the relevant rule)
//       * If it is optional then its type is Option<Foo>
//     * Otherwise:
//       * If it is mandatory then it is skipped
//       * If it is optional then its type is `bool`
// * List of rules. They are associated with the preceding patterns (until different opcode or
//   different rules). Rules are used to resolve modifiers. There are two types of rules:
//   * Normal rule: `.foobar: FoobarEnum => { .a, .b, .c }`. This means that instead of `.foobar` we
//     expecte one of `.a`, `.b`, `.c` and will emit value FoobarEnum::DotA, FoobarEnum::DotB,
//     FoobarEnum::DotC appropriately
//   * Type-only rule: `FoobarEnum => { .a, .b, .c }` this means that all the occurences of `.a` will
//     emit FoobarEnum::DotA to the code block. This helps to avoid copy-paste errors
// Additionally, you can opt out from the usual parsing rule generation with a special `<=` pattern.
// See `call` instruction to see it in action
derive_parser!(
    #[derive(Logos, PartialEq, Eq, Debug, Clone, Copy)]
    #[logos(skip r"(?:\s+)|(?://[^\n\r]*[\n\r]*)|(?:/\*[^*]*\*+(?:[^/*][^*]*\*+)*/)")]
    #[logos(error = TokenError)]
    enum Token<'input> {
        #[token(",")]
        Comma,
        #[token(".")]
        Dot,
        #[token(":")]
        Colon,
        #[token(";")]
        Semicolon,
        #[token("@")]
        At,
        #[regex(r"[a-zA-Z][a-zA-Z0-9_$]*|[_$%][a-zA-Z0-9_$]+", |lex| lex.slice(), priority = 0)]
        Ident(&'input str),
        #[regex(r"\.[a-zA-Z][a-zA-Z0-9_$]*|\.[_$%][a-zA-Z0-9_$]+", |lex| lex.slice(), priority = 0)]
        DotIdent(&'input str),
        #[regex(r#""[^"]*""#)]
        String,
        #[token("|")]
        Pipe,
        #[token("!")]
        Exclamation,
        #[token("(")]
        LParen,
        #[token(")")]
        RParen,
        #[token("[")]
        LBracket,
        #[token("]")]
        RBracket,
        #[token("{")]
        LBrace,
        #[token("}")]
        RBrace,
        #[token("<")]
        Lt,
        #[token(">")]
        Gt,
        #[regex(r"0[fF][0-9a-zA-Z]{8}", |lex| lex.slice())]
        F32(&'input str),
        #[regex(r"0[dD][0-9a-zA-Z]{16}", |lex| lex.slice())]
        F64(&'input str),
        #[regex(r"0[xX][0-9a-zA-Z]+U?", |lex| lex.slice())]
        Hex(&'input str),
        #[regex(r"[0-9]+U?", |lex| lex.slice())]
        Decimal(&'input str),
        #[token("-")]
        Minus,
        #[token("+")]
        Plus,
        #[token("=")]
        Eq,
        #[token(".version")]
        DotVersion,
        #[token(".loc")]
        DotLoc,
        #[token(".reg")]
        DotReg,
        #[token(".align")]
        DotAlign,
        #[token(".pragma")]
        DotPragma,
        #[token(".maxnreg")]
        DotMaxnreg,
        #[token(".maxntid")]
        DotMaxntid,
        #[token(".reqntid")]
        DotReqntid,
        #[token(".minnctapersm")]
        DotMinnctapersm,
        #[token(".entry")]
        DotEntry,
        #[token(".func")]
        DotFunc,
        #[token(".extern")]
        DotExtern,
        #[token(".visible")]
        DotVisible,
        #[token(".target")]
        DotTarget,
        #[token(".address_size")]
        DotAddressSize,
        #[token(".action")]
        DotSection,
        #[token(".file")]
        DotFile
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub enum StateSpace {
        Reg,
        Generic,
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub enum MemScope { }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub enum ScalarType { }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub enum SetpBoolPostOp { }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub enum AtomSemantics { }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub enum Mul24Control { }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#data-movement-and-conversion-instructions-mov
    mov{.vec}.type  d, a => {
        Instruction::Mov {
            data: ast::MovDetails::new(vec, type_),
            arguments: MovArgs { dst: d, src: a },
        }
    }
    .vec: VectorPrefix = { .v2, .v4 };
    .type: ScalarType =  { .pred,
                           .b16, .b32, .b64,
                           .u16, .u32, .u64,
                           .s16, .s32, .s64,
                                 .f32, .f64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st
    st{.weak}{.ss}{.cop}{.level::eviction_priority}{.level::cache_hint}{.vec}.type  [a], b{, cache_policy} => {
        if level_eviction_priority.is_some() || level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo);
        }
        Instruction::St {
            data: StData {
                qualifier: weak.unwrap_or(RawLdStQualifier::Weak).into(),
                state_space: ss.unwrap_or(StateSpace::Generic),
                caching: cop.unwrap_or(RawStCacheOperator::Wb).into(),
                typ: ast::Type::maybe_vector(vec, type_)
            },
            arguments: StArgs { src1:a, src2:b }
        }
    }
    st.volatile{.ss}{.vec}.type                                                     [a], b => {
        Instruction::St {
            data: StData {
                qualifier: volatile.into(),
                state_space: ss.unwrap_or(StateSpace::Generic),
                caching: ast::StCacheOperator::Writeback,
                typ: ast::Type::maybe_vector(vec, type_)
            },
            arguments: StArgs { src1:a, src2:b }
        }
    }
    st.relaxed.scope{.ss}{.level::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache_policy} => {
        if level_eviction_priority.is_some() || level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo);
        }
        Instruction::St {
            data: StData {
                qualifier: ast::LdStQualifier::Relaxed(scope),
                state_space: ss.unwrap_or(StateSpace::Generic),
                caching: ast::StCacheOperator::Writeback,
                typ: ast::Type::maybe_vector(vec, type_)
            },
            arguments: StArgs { src1:a, src2:b }
        }
    }
    st.release.scope{.ss}{.level::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache_policy} => {
        if level_eviction_priority.is_some() || level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo);
        }
        Instruction::St {
            data: StData {
                qualifier: ast::LdStQualifier::Release(scope),
                state_space: ss.unwrap_or(StateSpace::Generic),
                caching: ast::StCacheOperator::Writeback,
                typ: ast::Type::maybe_vector(vec, type_)
            },
            arguments: StArgs { src1:a, src2:b }
        }
    }
    st.mmio.relaxed.sys{.global}.type                                               [a], b => {
        state.errors.push(PtxError::Todo);
        Instruction::St {
            data: ast::StData {
                qualifier: ast::LdStQualifier::Relaxed(MemScope::Sys),
                state_space: global.unwrap_or(StateSpace::Generic),
                caching: ast::StCacheOperator::Writeback,
                typ: type_.into()
            },
            arguments: ast::StArgs { src1:a, src2:b }
        }
    }
    .ss: StateSpace =           { .global, .local, .param{::func}, .shared{::cta, ::cluster} };
    .level::eviction_priority: EvictionPriority =
                                { .L1::evict_normal, .L1::evict_unchanged, .L1::evict_first, .L1::evict_last, .L1::no_allocate };
    .level::cache_hint =        { .L2::cache_hint };
    .cop: RawStCacheOperator =  { .wb, .cg, .cs, .wt };
    .scope: MemScope =          { .cta, .cluster, .gpu, .sys };
    .vec: VectorPrefix =        { .v2, .v4 };
    .type: ScalarType =         { .b8, .b16, .b32, .b64, .b128,
                                  .u8, .u16, .u32, .u64,
                                  .s8, .s16, .s32, .s64,
                                  .f32, .f64 };
    RawLdStQualifier =          { .weak, .volatile };
    StateSpace =                { .global };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-ld
    ld{.weak}{.ss}{.cop}{.level::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type   d, [a]{.unified}{, cache_policy} => {
        let (a, unified) = a;
        if level_eviction_priority.is_some() || level_cache_hint || level_prefetch_size.is_some() || unified || cache_policy.is_some() {
            state.errors.push(PtxError::Todo);
        }
        Instruction::Ld {
            data: LdDetails {
                qualifier: weak.unwrap_or(RawLdStQualifier::Weak).into(),
                state_space: ss.unwrap_or(StateSpace::Generic),
                caching: cop.unwrap_or(RawLdCacheOperator::Ca).into(),
                typ: ast::Type::maybe_vector(vec, type_),
                non_coherent: false
            },
            arguments: LdArgs { dst:d, src:a }
        }
    }
    ld.volatile{.ss}{.level::prefetch_size}{.vec}.type                                                      d, [a] => {
        if level_prefetch_size.is_some() {
            state.errors.push(PtxError::Todo);
        }
        Instruction::Ld {
            data: LdDetails {
                qualifier: volatile.into(),
                state_space: ss.unwrap_or(StateSpace::Generic),
                caching: ast::LdCacheOperator::Cached,
                typ: ast::Type::maybe_vector(vec, type_),
                non_coherent: false
            },
            arguments: LdArgs { dst:d, src:a }
        }
    }
    ld.relaxed.scope{.ss}{.level::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{, cache_policy} => {
        if level_eviction_priority.is_some() || level_cache_hint || level_prefetch_size.is_some() || cache_policy.is_some() {
            state.errors.push(PtxError::Todo);
        }
        Instruction::Ld {
            data: LdDetails {
                qualifier: ast::LdStQualifier::Relaxed(scope),
                state_space: ss.unwrap_or(StateSpace::Generic),
                caching: ast::LdCacheOperator::Cached,
                typ: ast::Type::maybe_vector(vec, type_),
                non_coherent: false
            },
            arguments: LdArgs { dst:d, src:a }
        }
    }
    ld.acquire.scope{.ss}{.level::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{, cache_policy} => {
        if level_eviction_priority.is_some() || level_cache_hint || level_prefetch_size.is_some() || cache_policy.is_some() {
            state.errors.push(PtxError::Todo);
        }
        Instruction::Ld {
            data: LdDetails {
                qualifier: ast::LdStQualifier::Acquire(scope),
                state_space: ss.unwrap_or(StateSpace::Generic),
                caching: ast::LdCacheOperator::Cached,
                typ: ast::Type::maybe_vector(vec, type_),
                non_coherent: false
            },
            arguments: LdArgs { dst:d, src:a }
        }
    }
    ld.mmio.relaxed.sys{.global}.type                                                                       d, [a] => {
        state.errors.push(PtxError::Todo);
        Instruction::Ld {
            data: LdDetails {
                qualifier: ast::LdStQualifier::Relaxed(MemScope::Sys),
                state_space: global.unwrap_or(StateSpace::Generic),
                caching: ast::LdCacheOperator::Cached,
                typ: type_.into(),
                non_coherent: false
            },
            arguments: LdArgs { dst:d, src:a }
        }
    }
    .ss: StateSpace =                       { .const, .global, .local, .param{::entry, ::func}, .shared{::cta, ::cluster} };
    .cop: RawLdCacheOperator =              { .ca, .cg, .cs, .lu, .cv };
    .level::eviction_priority: EvictionPriority =
                                            { .L1::evict_normal, .L1::evict_unchanged, .L1::evict_first, .L1::evict_last, .L1::no_allocate };
    .level::cache_hint =                    { .L2::cache_hint };
    .level::prefetch_size: PrefetchSize =   { .L2::64B, .L2::128B, .L2::256B };
    .scope: MemScope =                      { .cta, .cluster, .gpu, .sys };
    .vec: VectorPrefix =                    { .v2, .v4 };
    .type: ScalarType =                     { .b8, .b16, .b32, .b64, .b128,
                                              .u8, .u16, .u32, .u64,
                                              .s8, .s16, .s32, .s64,
                                              .f32, .f64 };
    RawLdStQualifier =                      { .weak, .volatile };
    StateSpace =                            { .global };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#data-movement-and-conversion-instructions-ld-global-nc
    ld.global{.cop}.nc{.level::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{, cache_policy} => {
        if cop.is_some() && level_eviction_priority.is_some() {
            state.errors.push(PtxError::SyntaxError);
        }
        if level_eviction_priority.is_some() || level_cache_hint || level_prefetch_size.is_some() || cache_policy.is_some() {
            state.errors.push(PtxError::Todo);
        }
        Instruction::Ld {
            data: LdDetails {
                qualifier: ast::LdStQualifier::Weak,
                state_space: global,
                caching: cop.unwrap_or(RawLdCacheOperator::Ca).into(),
                typ: Type::maybe_vector(vec, type_),
                non_coherent: true
            },
            arguments: LdArgs { dst:d, src:a }
        }
    }
    .cop: RawLdCacheOperator  =             { .ca, .cg, .cs };
    .level::eviction_priority: EvictionPriority =
                                            { .L1::evict_normal, .L1::evict_unchanged,
                                              .L1::evict_first, .L1::evict_last, .L1::no_allocate};
    .level::cache_hint =                    { .L2::cache_hint };
    .level::prefetch_size: PrefetchSize =   { .L2::64B, .L2::128B, .L2::256B };
    .vec: VectorPrefix  =                   { .v2, .v4 };
    .type: ScalarType =                     { .b8, .b16, .b32, .b64, .b128,
                                              .u8, .u16, .u32, .u64,
                                              .s8, .s16, .s32, .s64,
                                              .f32, .f64 };
    StateSpace =                { .global };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-add
    add.type        d, a, b => {
        Instruction::Add {
            data: ast::ArithDetails::Integer(
                ast::ArithInteger {
                    type_,
                    saturate: false
                }
            ),
            arguments: AddArgs {
                dst: d, src1: a, src2: b
            }
        }
    }
    add{.sat}.s32   d, a, b => {
        Instruction::Add {
            data: ast::ArithDetails::Integer(
                ast::ArithInteger {
                    type_: s32,
                    saturate: sat
                }
            ),
            arguments: AddArgs {
                dst: d, src1: a, src2: b
            }
        }
    }
    .type: ScalarType = { .u16, .u32, .u64,
                          .s16, .s64,
                          .u16x2, .s16x2 };
    ScalarType =        { .s32 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-add
    add{.rnd}{.ftz}{.sat}.f32  d, a, b => {
        Instruction::Add {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: f32,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: AddArgs {
                dst: d, src1: a, src2: b
            }
        }
    }
    add{.rnd}.f64              d, a, b => {
        Instruction::Add {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: f64,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: None,
                    saturate: false,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: AddArgs {
                dst: d, src1: a, src2: b
            }
        }
    }
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
    ScalarType =        { .f32, .f64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-add
    add{.rnd}{.ftz}{.sat}.f16   d, a, b => {
        Instruction::Add {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: f16,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: AddArgs {
                dst: d, src1: a, src2: b
            }
        }
    }
    add{.rnd}{.ftz}{.sat}.f16x2 d, a, b => {
        Instruction::Add {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: f16x2,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: AddArgs {
                dst: d, src1: a, src2: b
            }
        }
    }
    add{.rnd}.bf16              d, a, b => {
        Instruction::Add {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: bf16,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: None,
                    saturate: false,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: AddArgs {
                dst: d, src1: a, src2: b
            }
        }
    }
    add{.rnd}.bf16x2            d, a, b => {
        Instruction::Add {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: bf16x2,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: None,
                    saturate: false,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: AddArgs {
                dst: d, src1: a, src2: b
            }
        }
    }
    .rnd: RawRoundingMode = { .rn };
    ScalarType =        { .f16, .f16x2, .bf16, .bf16x2 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-mul
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-mul
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#half-precision-floating-point-instructions-mul
    mul.mode.type  d, a, b => {
        ast::Instruction::Mul {
            data: ast::MulDetails::Integer {
                type_,
                control: mode.into()
            },
            arguments: MulArgs { dst: d, src1: a, src2: b }
        }
    }
    .mode: RawMulIntControl =   { .hi, .lo };
    .type: ScalarType =         { .u16, .u32, .u64,
                                  .s16, .s32, .s64 };
    // "The .wide suffix is supported only for 16- and 32-bit integer types"
    mul.wide.type  d, a, b => {
        ast::Instruction::Mul {
            data: ast::MulDetails::Integer {
                type_,
                control: wide.into()
            },
            arguments: MulArgs { dst: d, src1: a, src2: b }
        }
    }
    .type: ScalarType = { .u16, .u32,
                          .s16, .s32 };
    RawMulIntControl =  { .wide };

    mul{.rnd}{.ftz}{.sat}.f32  d, a, b => {
        ast::Instruction::Mul {
            data: ast::MulDetails::Float (
                ast::ArithFloat {
                    type_: f32,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: MulArgs { dst: d, src1: a, src2: b }
        }
    }
    mul{.rnd}.f64              d, a, b => {
        ast::Instruction::Mul {
            data: ast::MulDetails::Float (
                ast::ArithFloat {
                    type_: f64,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: None,
                    saturate: false,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: MulArgs { dst: d, src1: a, src2: b }
        }
    }
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
    ScalarType = { .f32, .f64 };

    mul{.rnd}{.ftz}{.sat}.f16   d, a, b => {
        ast::Instruction::Mul {
            data: ast::MulDetails::Float (
                ast::ArithFloat {
                    type_: f16,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: MulArgs { dst: d, src1: a, src2: b }
        }
    }
    mul{.rnd}{.ftz}{.sat}.f16x2 d, a, b => {
        ast::Instruction::Mul {
            data: ast::MulDetails::Float (
                ast::ArithFloat {
                    type_: f16x2,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: MulArgs { dst: d, src1: a, src2: b }
        }
    }
    mul{.rnd}.bf16   d, a, b => {
        ast::Instruction::Mul {
            data: ast::MulDetails::Float (
                ast::ArithFloat {
                    type_: bf16,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: None,
                    saturate: false,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: MulArgs { dst: d, src1: a, src2: b }
        }
    }
    mul{.rnd}.bf16x2 d, a, b => {
        ast::Instruction::Mul {
            data: ast::MulDetails::Float (
                ast::ArithFloat {
                    type_: bf16x2,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: None,
                    saturate: false,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: MulArgs { dst: d, src1: a, src2: b }
        }
    }
    .rnd: RawRoundingMode = { .rn };
    ScalarType = { .f16, .f16x2, .bf16, .bf16x2 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#comparison-and-selection-instructions-setp
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#half-precision-comparison-instructions-setp
    setp.CmpOp{.ftz}.type         p[|q], a, b => {
        let data = ast::SetpData::try_parse(state, cmpop, ftz, type_);
        ast::Instruction::Setp {
            data,
            arguments: SetpArgs { dst1: p, dst2: q, src1: a, src2: b }
        }
    }
    setp.CmpOp.BoolOp{.ftz}.type  p[|q], a, b, {!}c => {
        let (negate_src3, c) = c;
        let base = ast::SetpData::try_parse(state, cmpop, ftz, type_);
        let data = ast::SetpBoolData {
            base,
            bool_op: boolop,
            negate_src3
        };
        ast::Instruction::SetpBool {
            data,
            arguments: SetpBoolArgs { dst1: p, dst2: q, src1: a, src2: b, src3: c }
        }
    }
    .CmpOp: RawSetpCompareOp  = { .eq, .ne, .lt, .le, .gt, .ge,
                                  .lo, .ls, .hi, .hs, // signed
                                  .equ, .neu, .ltu, .leu, .gtu, .geu, .num, .nan }; // float-only
    .BoolOp: SetpBoolPostOp  =  { .and, .or, .xor };
    .type: ScalarType   =       { .b16, .b32, .b64,
                                  .u16, .u32, .u64,
                                  .s16, .s32, .s64,
                                  .f32, .f64,
                                  .f16, .f16x2, .bf16, .bf16x2 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#logic-and-shift-instructions-not
    not.type d, a => {
        ast::Instruction::Not {
            data: type_,
            arguments: NotArgs { dst: d, src: a }
        }
    }
    .type: ScalarType = { .pred, .b16, .b32, .b64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#logic-and-shift-instructions-or
    or.type d, a, b => {
        ast::Instruction::Or {
            data: type_,
            arguments: OrArgs { dst: d, src1: a, src2: b }
        }
    }
    .type: ScalarType = { .pred, .b16, .b32, .b64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#logic-and-shift-instructions-and
    and.type d, a, b => {
        ast::Instruction::And {
            data: type_,
            arguments: AndArgs { dst: d, src1: a, src2: b }
        }
    }
    .type: ScalarType = { .pred, .b16, .b32, .b64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#control-flow-instructions-bra
    bra <= { bra(stream) }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#control-flow-instructions-call
    call <= { call(stream) }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#data-movement-and-conversion-instructions-cvt
    cvt{.ifrnd}{.ftz}{.sat}.dtype.atype         d, a => {
        let data = ast::CvtDetails::new(&mut state.errors, ifrnd, ftz, sat, dtype, atype);
        let arguments = ast::CvtArgs { dst: d, src: a };
        ast::Instruction::Cvt {
            data, arguments
        }
    }
    // cvt.frnd2{.relu}{.satfinite}.f16.f32       d, a;
    // cvt.frnd2{.relu}{.satfinite}.f16x2.f32     d, a, b;
    // cvt.frnd2{.relu}{.satfinite}.bf16.f32      d, a;
    // cvt.frnd2{.relu}{.satfinite}.bf16x2.f32    d, a, b;
    // cvt.rna{.satfinite}.tf32.f32               d, a;
    // cvt.frnd2{.relu}.tf32.f32                   d, a;
    // cvt.rn.satfinite{.relu}.f8x2type.f32       d, a, b;
    // cvt.rn.satfinite{.relu}.f8x2type.f16x2     d, a;
    // cvt.rn.{.relu}.f16x2.f8x2type              d, a;

    .ifrnd: RawRoundingMode =   { .rn,  .rz,  .rm,  .rp,  .rni, .rzi, .rmi, .rpi };
    .frnd2: RawRoundingMode =   { .rn,  .rz };
    .dtype: ScalarType =        { .u8,   .u16, .u32, .u64,
                                  .s8,   .s16, .s32, .s64,
                                  .bf16, .f16, .f32, .f64 };
    .atype: ScalarType =        { .u8,   .u16, .u32, .u64,
                                  .s8,   .s16, .s32, .s64,
                                  .bf16, .f16, .f32, .f64 };
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#logic-and-shift-instructions-shl
    shl.type d, a, b => {
        ast::Instruction::Shl { data: type_, arguments: ShlArgs { dst: d, src1: a, src2: b } }
    }
    .type: ScalarType = { .b16, .b32, .b64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#logic-and-shift-instructions-shr
    shr.type d, a, b => {
        let kind = if type_.kind() == ast::ScalarKind::Signed { RightShiftKind::Arithmetic} else { RightShiftKind::Logical };
        ast::Instruction::Shr {
            data: ast::ShrData { type_, kind },
            arguments: ShrArgs { dst: d, src1: a, src2: b }
        }
    }
    .type: ScalarType = { .b16, .b32, .b64,
                          .u16, .u32, .u64,
                          .s16, .s32, .s64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#data-movement-and-conversion-instructions-cvta
    cvta.space.size     p, a => {
        if size != ScalarType::U64 {
            state.errors.push(PtxError::Unsupported32Bit);
        }
        let data = ast::CvtaDetails {
            state_space: space,
            direction: ast::CvtaDirection::ExplicitToGeneric
        };
        let arguments = ast::CvtaArgs {
            dst: p, src: a
        };
        ast::Instruction::Cvta {
            data, arguments
        }
    }
    cvta.to.space.size  p, a => {
        if size != ScalarType::U64 {
            state.errors.push(PtxError::Unsupported32Bit);
        }
        let data = ast::CvtaDetails {
            state_space: space,
            direction: ast::CvtaDirection::GenericToExplicit
        };
        let arguments = ast::CvtaArgs {
            dst: p, src: a
        };
        ast::Instruction::Cvta {
            data, arguments
        }
    }
    .space: StateSpace = { .const, .global, .local, .shared{::cta, ::cluster}, .param{::entry} };
    .size: ScalarType  = { .u32, .u64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-abs
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-abs
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#half-precision-floating-point-instructions-abs
    abs.type        d, a => {
        ast::Instruction::Abs {
            data: ast::TypeFtz {
                flush_to_zero: None,
                type_
            },
            arguments: ast::AbsArgs {
                dst: d, src: a
            }
        }
    }
    abs{.ftz}.f32   d, a => {
        ast::Instruction::Abs {
            data: ast::TypeFtz {
                flush_to_zero: Some(ftz),
                type_: f32
            },
            arguments: ast::AbsArgs {
                dst: d, src: a
            }
        }
    }
    abs.f64         d, a => {
        ast::Instruction::Abs {
            data: ast::TypeFtz {
                flush_to_zero: None,
                type_: f64
            },
            arguments: ast::AbsArgs {
                dst: d, src: a
            }
        }
    }
    abs{.ftz}.f16   d, a => {
        ast::Instruction::Abs {
            data: ast::TypeFtz {
                flush_to_zero: Some(ftz),
                type_: f16
            },
            arguments: ast::AbsArgs {
                dst: d, src: a
            }
        }
    }
    abs{.ftz}.f16x2 d, a => {
        ast::Instruction::Abs {
            data: ast::TypeFtz {
                flush_to_zero: Some(ftz),
                type_: f16x2
            },
            arguments: ast::AbsArgs {
                dst: d, src: a
            }
        }
    }
    abs.bf16        d, a => {
        ast::Instruction::Abs {
            data: ast::TypeFtz {
                flush_to_zero: None,
                type_: bf16
            },
            arguments: ast::AbsArgs {
                dst: d, src: a
            }
        }
    }
    abs.bf16x2      d, a => {
        ast::Instruction::Abs {
            data: ast::TypeFtz {
                flush_to_zero: None,
                type_: bf16x2
            },
            arguments: ast::AbsArgs {
                dst: d, src: a
            }
        }
    }
    .type: ScalarType = { .s16, .s32, .s64 };
    ScalarType = { .f32, .f64, .f16, .f16x2, .bf16, .bf16x2 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-mad
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-mad
    mad.mode.type  d, a, b, c => {
        ast::Instruction::Mad {
            data: ast::MadDetails::Integer {
                type_,
                control: mode.into(),
                saturate: false
            },
            arguments: MadArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }
    .type: ScalarType =         { .u16, .u32, .u64,
                                  .s16, .s32, .s64 };
    .mode: RawMulIntControl =   { .hi, .lo };

    // The .wide suffix is supported only for 16-bit and 32-bit integer types.
    mad.wide.type  d, a, b, c => {
        ast::Instruction::Mad {
            data: ast::MadDetails::Integer {
                type_,
                control: wide.into(),
                saturate: false
            },
            arguments: MadArgs { dst: d, src1: a, src2: b, src3: c  }
        }
    }
    .type: ScalarType = { .u16, .u32,
                          .s16, .s32 };
    RawMulIntControl =  { .wide };

    mad.hi.sat.s32 d, a, b, c => {
        ast::Instruction::Mad {
            data: ast::MadDetails::Integer {
                type_: s32,
                control: hi.into(),
                saturate: true
            },
            arguments: MadArgs { dst: d, src1: a, src2: b, src3: c  }
        }
    }
    RawMulIntControl =  { .hi };
    ScalarType =        { .s32 };

    mad{.ftz}{.sat}.f32      d, a, b, c => {
        ast::Instruction::Mad {
            data: ast::MadDetails::Float(
                ast::ArithFloat {
                    type_: f32,
                    rounding: ast::RoundingMode::NearestEven,
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: false
                }
            ),
            arguments: MadArgs { dst: d, src1: a, src2: b, src3: c  }
        }
    }
    mad.rnd{.ftz}{.sat}.f32  d, a, b, c => {
        ast::Instruction::Mad {
            data: ast::MadDetails::Float(
                ast::ArithFloat {
                    type_: f32,
                    rounding: rnd.into(),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: false
                }
            ),
            arguments: MadArgs { dst: d, src1: a, src2: b, src3: c  }
        }
    }
    mad.rnd.f64              d, a, b, c => {
        ast::Instruction::Mad {
            data: ast::MadDetails::Float(
                ast::ArithFloat {
                    type_: f64,
                    rounding: rnd.into(),
                    flush_to_zero: None,
                    saturate: false,
                    is_fusable: false
                }
            ),
            arguments: MadArgs { dst: d, src1: a, src2: b, src3: c  }
        }
    }
    .rnd: RawRoundingMode   = { .rn, .rz, .rm, .rp };
    ScalarType =        { .f32, .f64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-fma
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#half-precision-floating-point-instructions-fma
    fma.rnd{.ftz}{.sat}.f32  d, a, b, c => {
        ast::Instruction::Fma {
            data: ast::ArithFloat {
                type_: f32,
                rounding: rnd.into(),
                flush_to_zero: Some(ftz),
                saturate: sat,
                is_fusable: false
            },
            arguments: FmaArgs { dst: d, src1: a, src2: b, src3: c  }
        }
    }
    fma.rnd.f64              d, a, b, c => {
        ast::Instruction::Fma {
            data: ast::ArithFloat {
                type_: f64,
                rounding: rnd.into(),
                flush_to_zero: None,
                saturate: false,
                is_fusable: false
            },
            arguments: FmaArgs { dst: d, src1: a, src2: b, src3: c  }
        }
    }
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
    ScalarType =            { .f32, .f64 };

    fma.rnd{.ftz}{.sat}.f16 d, a, b, c => {
        ast::Instruction::Fma {
            data: ast::ArithFloat {
                type_: f16,
                rounding: rnd.into(),
                flush_to_zero: Some(ftz),
                saturate: sat,
                is_fusable: false
            },
            arguments: FmaArgs { dst: d, src1: a, src2: b, src3: c  }
        }
    }
    //fma.rnd{.ftz}{.sat}.f16x2   d, a, b, c;
    //fma.rnd{.ftz}.relu.f16      d, a, b, c;
    //fma.rnd{.ftz}.relu.f16x2    d, a, b, c;
    //fma.rnd{.relu}.bf16         d, a, b, c;
    //fma.rnd{.relu}.bf16x2       d, a, b, c;
    //fma.rnd.oob.{relu}.type     d, a, b, c;
    .rnd: RawRoundingMode = { .rn };
    ScalarType =            { .f16 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-sub
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-sub
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#half-precision-floating-point-instructions-sub
    sub.type       d, a, b => {
        ast::Instruction::Sub {
            data: ast::ArithDetails::Integer(
                ArithInteger {
                    type_,
                    saturate: false
                }
            ),
            arguments: SubArgs { dst: d, src1: a, src2: b  }
        }
    }
    sub.sat.s32  d, a, b => {
        ast::Instruction::Sub {
            data: ast::ArithDetails::Integer(
                ArithInteger {
                    type_: s32,
                    saturate: true
                }
            ),
            arguments: SubArgs { dst: d, src1: a, src2: b  }
        }
    }
    .type: ScalarType = { .u16, .u32, .u64,
                          .s16, .s32, .s64 };
    ScalarType = { .s32 };

    sub{.rnd}{.ftz}{.sat}.f32  d, a, b => {
        ast::Instruction::Sub {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: f32,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: SubArgs { dst: d, src1: a, src2: b  }
        }
    }
    sub{.rnd}.f64              d, a, b => {
        ast::Instruction::Sub {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: f64,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: None,
                    saturate: false,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: SubArgs { dst: d, src1: a, src2: b  }
        }
    }
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
    ScalarType = { .f32, .f64 };

    sub{.rnd}{.ftz}{.sat}.f16   d, a, b => {
        ast::Instruction::Sub {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: f16,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: SubArgs { dst: d, src1: a, src2: b  }
        }
    }
    sub{.rnd}{.ftz}{.sat}.f16x2 d, a, b => {
        ast::Instruction::Sub {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: f16x2,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: SubArgs { dst: d, src1: a, src2: b  }
        }
    }
    sub{.rnd}.bf16   d, a, b => {
        ast::Instruction::Sub {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: bf16,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: None,
                    saturate: false,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: SubArgs { dst: d, src1: a, src2: b  }
        }
    }
    sub{.rnd}.bf16x2 d, a, b => {
        ast::Instruction::Sub {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: bf16x2,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: None,
                    saturate: false,
                    is_fusable: rnd.is_none()
                }
            ),
            arguments: SubArgs { dst: d, src1: a, src2: b  }
        }
    }
    .rnd: RawRoundingMode = { .rn };
    ScalarType = { .f16, .f16x2, .bf16, .bf16x2 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-min
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-min
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#half-precision-floating-point-instructions-min
    min.atype         d, a, b => {
        ast::Instruction::Min {
            data: if atype.kind() == ast::ScalarKind::Signed {
                ast::MinMaxDetails::Signed(atype)
            } else {
                ast::MinMaxDetails::Unsigned(atype)
            },
            arguments: MinArgs { dst: d, src1: a, src2: b  }
        }
    }
    //min{.relu}.btype  d, a, b => { todo!() }
    min.btype  d, a, b => {
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Signed(btype),
            arguments: MinArgs { dst: d, src1: a, src2: b  }
        }
    }
    .atype: ScalarType = { .u16, .u32, .u64,
                           .u16x2, .s16, .s64 };
    .btype: ScalarType = { .s16x2, .s32 };

    //min{.ftz}{.NaN}{.xorsign.abs}.f32  d, a, b;
    min{.ftz}{.NaN}.f32   d, a, b => {
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: Some(ftz),
                    nan,
                    type_: f32
                }
            ),
            arguments: MinArgs { dst: d, src1: a, src2: b  }
        }
    }
    min.f64         d, a, b => {
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: None,
                    nan: false,
                    type_: f64
                }
            ),
            arguments: MinArgs { dst: d, src1: a, src2: b  }
        }
    }
    ScalarType = { .f32, .f64 };

    //min{.ftz}{.NaN}{.xorsign.abs}.f16      d, a, b;
    //min{.ftz}{.NaN}{.xorsign.abs}.f16x2    d, a, b;
    //min{.NaN}{.xorsign.abs}.bf16           d, a, b;
    //min{.NaN}{.xorsign.abs}.bf16x2         d, a, b;
    min{.ftz}{.NaN}.f16      d, a, b => {
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: Some(ftz),
                    nan,
                    type_: f16
                }
            ),
            arguments: MinArgs { dst: d, src1: a, src2: b  }
        }
    }
    min{.ftz}{.NaN}.f16x2    d, a, b => {
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: Some(ftz),
                    nan,
                    type_: f16x2
                }
            ),
            arguments: MinArgs { dst: d, src1: a, src2: b  }
        }
    }
    min{.NaN}.bf16           d, a, b => {
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: None,
                    nan,
                    type_: bf16
                }
            ),
            arguments: MinArgs { dst: d, src1: a, src2: b  }
        }
    }
    min{.NaN}.bf16x2         d, a, b => {
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: None,
                    nan,
                    type_: bf16x2
                }
            ),
            arguments: MinArgs { dst: d, src1: a, src2: b  }
        }
    }
    ScalarType = { .f16, .f16x2, .bf16, .bf16x2 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-max
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-max
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#half-precision-floating-point-instructions-max
    max.atype         d, a, b => {
        ast::Instruction::Max {
            data: if atype.kind() == ast::ScalarKind::Signed {
                ast::MinMaxDetails::Signed(atype)
            } else {
                ast::MinMaxDetails::Unsigned(atype)
            },
            arguments: MaxArgs { dst: d, src1: a, src2: b  }
        }
    }
    //max{.relu}.btype  d, a, b => { todo!() }
    max.btype  d, a, b => {
        ast::Instruction::Max {
            data: ast::MinMaxDetails::Signed(btype),
            arguments: MaxArgs { dst: d, src1: a, src2: b  }
        }
    }
    .atype: ScalarType = { .u16, .u32, .u64,
                           .u16x2, .s16, .s64 };
    .btype: ScalarType = { .s16x2, .s32 };

    //max{.ftz}{.NaN}{.xorsign.abs}.f32  d, a, b;
    max{.ftz}{.NaN}.f32   d, a, b => {
        ast::Instruction::Max {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: Some(ftz),
                    nan,
                    type_: f32
                }
            ),
            arguments: MaxArgs { dst: d, src1: a, src2: b  }
        }
    }
    max.f64         d, a, b => {
        ast::Instruction::Max {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: None,
                    nan: false,
                    type_: f64
                }
            ),
            arguments: MaxArgs { dst: d, src1: a, src2: b  }
        }
    }
    ScalarType = { .f32, .f64 };

    //max{.ftz}{.NaN}{.xorsign.abs}.f16      d, a, b;
    //max{.ftz}{.NaN}{.xorsign.abs}.f16x2    d, a, b;
    //max{.NaN}{.xorsign.abs}.bf16           d, a, b;
    //max{.NaN}{.xorsign.abs}.bf16x2         d, a, b;
    max{.ftz}{.NaN}.f16      d, a, b => {
        ast::Instruction::Max {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: Some(ftz),
                    nan,
                    type_: f16
                }
            ),
            arguments: MaxArgs { dst: d, src1: a, src2: b  }
        }
    }
    max{.ftz}{.NaN}.f16x2    d, a, b => {
        ast::Instruction::Max {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: Some(ftz),
                    nan,
                    type_: f16x2
                }
            ),
            arguments: MaxArgs { dst: d, src1: a, src2: b  }
        }
    }
    max{.NaN}.bf16           d, a, b => {
        ast::Instruction::Max {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: None,
                    nan,
                    type_: bf16
                }
            ),
            arguments: MaxArgs { dst: d, src1: a, src2: b  }
        }
    }
    max{.NaN}.bf16x2         d, a, b => {
        ast::Instruction::Max {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: None,
                    nan,
                    type_: bf16x2
                }
            ),
            arguments: MaxArgs { dst: d, src1: a, src2: b  }
        }
    }
    ScalarType = { .f16, .f16x2, .bf16, .bf16x2 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-rcp
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-rcp-approx-ftz-f64
    rcp.approx{.ftz}.type   d, a => {
        ast::Instruction::Rcp {
            data: ast::RcpData {
                kind: ast::RcpKind::Approx,
                flush_to_zero: Some(ftz),
                type_
            },
            arguments: RcpArgs { dst: d, src: a  }
        }
    }
    rcp.rnd{.ftz}.f32       d, a => {
        ast::Instruction::Rcp {
            data: ast::RcpData {
                kind: ast::RcpKind::Compliant(rnd.into()),
                flush_to_zero: Some(ftz),
                type_: f32
            },
            arguments: RcpArgs { dst: d, src: a  }
        }
    }
    rcp.rnd.f64             d, a => {
        ast::Instruction::Rcp {
            data: ast::RcpData {
                kind: ast::RcpKind::Compliant(rnd.into()),
                flush_to_zero: None,
                type_: f64
            },
            arguments: RcpArgs { dst: d, src: a  }
        }
    }
    .type: ScalarType =        { .f32, .f64 };
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
    ScalarType =        { .f32, .f64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-sqrt
    sqrt.approx{.ftz}.f32  d, a => {
        ast::Instruction::Sqrt {
            data: ast::RcpData {
                kind: ast::RcpKind::Approx,
                flush_to_zero: Some(ftz),
                type_: f32
            },
            arguments: SqrtArgs { dst: d, src: a  }
        }
    }
    sqrt.rnd{.ftz}.f32     d, a => {
        ast::Instruction::Sqrt {
            data: ast::RcpData {
                kind: ast::RcpKind::Compliant(rnd.into()),
                flush_to_zero: Some(ftz),
                type_: f32
            },
            arguments: SqrtArgs { dst: d, src: a  }
        }
    }
    sqrt.rnd.f64           d, a => {
        ast::Instruction::Sqrt {
            data: ast::RcpData {
                kind: ast::RcpKind::Compliant(rnd.into()),
                flush_to_zero: None,
                type_: f64
            },
            arguments: SqrtArgs { dst: d, src: a  }
        }
    }
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
    ScalarType =        { .f32, .f64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-rsqrt
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-rsqrt-approx-ftz-f64
    rsqrt.approx{.ftz}.f32  d, a => {
        ast::Instruction::Rsqrt {
            data: ast::TypeFtz {
                flush_to_zero: Some(ftz),
                type_: f32
            },
            arguments: RsqrtArgs { dst: d, src: a  }
        }
    }
    rsqrt.approx.f64        d, a => {
        ast::Instruction::Rsqrt {
            data: ast::TypeFtz {
                flush_to_zero: Some(false),
                type_: f64
            },
            arguments: RsqrtArgs { dst: d, src: a  }
        }
    }
    rsqrt.approx.ftz.f64 d, a => {
        ast::Instruction::Rsqrt {
            data: ast::TypeFtz {
                flush_to_zero: Some(true),
                type_: f64
            },
            arguments: RsqrtArgs { dst: d, src: a  }
        }
    }
    ScalarType =        { .f32, .f64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#comparison-and-selection-instructions-selp
    selp.type d, a, b, c => {
        ast::Instruction::Selp {
            data: type_,
            arguments: SelpArgs { dst: d, src1: a, src2: b, src3: c  }
        }
    }
    .type: ScalarType = { .b16, .b32, .b64,
                          .u16, .u32, .u64,
                          .s16, .s32, .s64,
                          .f32, .f64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#parallel-synchronization-and-communication-instructions-bar
    barrier{.cta}.sync{.aligned}    a{, b} => {
        let _ = cta;
        ast::Instruction::Bar {
            data: ast::BarData { aligned },
            arguments: BarArgs { src1: a, src2: b }
        }
    }
    //barrier{.cta}.arrive{.aligned}    a, b;
    //barrier{.cta}.red.popc{.aligned}.u32  d, a{, b}, {!}c;
    //barrier{.cta}.red.op{.aligned}.pred   p, a{, b}, {!}c;
    bar{.cta}.sync                  a{, b} => {
        let _ = cta;
        ast::Instruction::Bar {
            data: ast::BarData { aligned: true },
            arguments: BarArgs { src1: a, src2: b }
        }
    }
    //bar{.cta}.arrive    a, b;
    //bar{.cta}.red.popc.u32  d, a{, b}, {!}c;
    //bar{.cta}.red.op.pred   p, a{, b}, {!}c;
    //.op = { .and, .or };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#parallel-synchronization-and-communication-instructions-atom
    atom{.sem}{.scope}{.space}.op{.level::cache_hint}.type                                      d, [a], b{, cache_policy} => {
        if level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo);
        }
        ast::Instruction::Atom {
            data: AtomDetails {
                semantics: sem.map(Into::into).unwrap_or(AtomSemantics::Relaxed),
                scope: scope.unwrap_or(MemScope::Gpu),
                space: space.unwrap_or(StateSpace::Generic),
                op: ast::AtomicOp::new(op, type_.kind()),
                type_: type_.into()
            },
            arguments: AtomArgs { dst: d, src1: a, src2: b }
        }
    }
    atom{.sem}{.scope}{.space}.cas.cas_type                                                     d, [a], b, c => {
        ast::Instruction::AtomCas {
            data: AtomCasDetails {
                semantics: sem.map(Into::into).unwrap_or(AtomSemantics::Relaxed),
                scope: scope.unwrap_or(MemScope::Gpu),
                space: space.unwrap_or(StateSpace::Generic),
                type_: cas_type
            },
            arguments: AtomCasArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }
    atom{.sem}{.scope}{.space}.exch{.level::cache_hint}.b128                                    d, [a], b{, cache_policy} => {
        if level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo);
        }
        ast::Instruction::Atom {
            data: AtomDetails {
                semantics: sem.map(Into::into).unwrap_or(AtomSemantics::Relaxed),
                scope: scope.unwrap_or(MemScope::Gpu),
                space: space.unwrap_or(StateSpace::Generic),
                op: ast::AtomicOp::new(exch, b128.kind()),
                type_: b128.into()
            },
            arguments: AtomArgs { dst: d, src1: a, src2: b }
        }
    }
    atom{.sem}{.scope}{.global}.float_op{.level::cache_hint}.vec_32_bit.f32                     d, [a], b{, cache_policy} => {
        if level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo);
        }
        ast::Instruction::Atom {
            data: AtomDetails {
                semantics: sem.map(Into::into).unwrap_or(AtomSemantics::Relaxed),
                scope: scope.unwrap_or(MemScope::Gpu),
                space: global.unwrap_or(StateSpace::Generic),
                op: ast::AtomicOp::new(float_op, f32.kind()),
                type_: ast::Type::Vector(vec_32_bit.len().get(), f32)
            },
            arguments: AtomArgs { dst: d, src1: a, src2: b }
        }
    }
    atom{.sem}{.scope}{.global}.float_op.noftz{.level::cache_hint}{.vec_16_bit}.half_word_type  d, [a], b{, cache_policy} => {
        if level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo);
        }
        ast::Instruction::Atom {
            data: AtomDetails {
                semantics: sem.map(Into::into).unwrap_or(AtomSemantics::Relaxed),
                scope: scope.unwrap_or(MemScope::Gpu),
                space: global.unwrap_or(StateSpace::Generic),
                op: ast::AtomicOp::new(float_op, half_word_type.kind()),
                type_: ast::Type::maybe_vector(vec_16_bit, half_word_type)
            },
            arguments: AtomArgs { dst: d, src1: a, src2: b }
        }
    }
    atom{.sem}{.scope}{.global}.float_op.noftz{.level::cache_hint}{.vec_32_bit}.packed_type     d, [a], b{, cache_policy} => {
        if level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo);
        }
        ast::Instruction::Atom {
            data: AtomDetails {
                semantics: sem.map(Into::into).unwrap_or(AtomSemantics::Relaxed),
                scope: scope.unwrap_or(MemScope::Gpu),
                space: global.unwrap_or(StateSpace::Generic),
                op: ast::AtomicOp::new(float_op, packed_type.kind()),
                type_: ast::Type::maybe_vector(vec_32_bit, packed_type)
            },
            arguments: AtomArgs { dst: d, src1: a, src2: b }
        }
    }
    .space: StateSpace =            { .global, .shared{::cta, ::cluster} };
    .sem: AtomSemantics =           { .relaxed, .acquire, .release, .acq_rel };
    .scope: MemScope =              { .cta, .cluster, .gpu, .sys };
    .op: RawAtomicOp =              { .and, .or, .xor,
                                      .exch,
                                      .add, .inc, .dec,
                                      .min, .max };
    .level::cache_hint =            { .L2::cache_hint };
    .type: ScalarType =             { .b32, .b64, .u32, .u64, .s32, .s64, .f32, .f64 };
    .cas_type: ScalarType =         { .b32, .b64, .u32, .u64, .s32, .s64, .f32, .f64, .b16, .b128 };
    .half_word_type: ScalarType =   { .f16, .bf16 };
    .packed_type: ScalarType =      { .f16x2, .bf16x2 };
    .vec_16_bit: VectorPrefix =     { .v2, .v4, .v8 };
    .vec_32_bit:  VectorPrefix =    { .v2, .v4 };
    .float_op: RawAtomicOp =        { .add, .min, .max };
    ScalarType =                    { .b16, .b128, .f32 };
    StateSpace =                    { .global };
    RawAtomicOp =                   { .exch };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-div
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-div
    div.type  d, a, b => {
        ast::Instruction::Div {
            data: if type_.kind() == ast::ScalarKind::Signed {
                ast::DivDetails::Signed(type_)
            } else {
                ast::DivDetails::Unsigned(type_)
            },
            arguments: DivArgs {
                dst: d,
                src1: a,
                src2: b,
            },
        }
    }
    .type: ScalarType = { .u16, .u32, .u64,
                          .s16, .s32, .s64 };

    div.approx{.ftz}.f32  d, a, b => {
        ast::Instruction::Div {
            data: ast::DivDetails::Float(ast::DivFloatDetails{
                type_: f32,
                flush_to_zero: Some(ftz),
                kind: ast::DivFloatKind::Approx
            }),
            arguments: DivArgs {
                dst: d,
                src1: a,
                src2: b,
            },
        }
    }
    div.full{.ftz}.f32    d, a, b => {
        ast::Instruction::Div {
            data: ast::DivDetails::Float(ast::DivFloatDetails{
                type_: f32,
                flush_to_zero: Some(ftz),
                kind: ast::DivFloatKind::ApproxFull
            }),
            arguments: DivArgs {
                dst: d,
                src1: a,
                src2: b,
            },
        }
    }
    div.rnd{.ftz}.f32     d, a, b => {
        ast::Instruction::Div {
            data: ast::DivDetails::Float(ast::DivFloatDetails{
                type_: f32,
                flush_to_zero: Some(ftz),
                kind: ast::DivFloatKind::Rounding(rnd.into())
            }),
            arguments: DivArgs {
                dst: d,
                src1: a,
                src2: b,
            },
        }
    }
    div.rnd.f64           d, a, b => {
        ast::Instruction::Div {
            data: ast::DivDetails::Float(ast::DivFloatDetails{
                type_: f64,
                flush_to_zero: None,
                kind: ast::DivFloatKind::Rounding(rnd.into())
            }),
            arguments: DivArgs {
                dst: d,
                src1: a,
                src2: b,
            },
        }
    }
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
    ScalarType = { .f32, .f64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-neg
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-neg
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#half-precision-floating-point-instructions-neg
    neg.type  d, a => {
        ast::Instruction::Neg {
            data: TypeFtz {
                type_,
                flush_to_zero: None
            },
            arguments: NegArgs { dst: d, src: a, },
        }
    }
    .type: ScalarType = { .s16, .s32, .s64 };

    neg{.ftz}.f32  d, a => {
        ast::Instruction::Neg {
            data: TypeFtz {
                type_: f32,
                flush_to_zero: Some(ftz)
            },
            arguments: NegArgs { dst: d, src: a, },
        }
    }
    neg.f64        d, a => {
        ast::Instruction::Neg {
            data: TypeFtz {
                type_: f64,
                flush_to_zero: None
            },
            arguments: NegArgs { dst: d, src: a, },
        }
    }
    neg{.ftz}.f16    d, a => {
        ast::Instruction::Neg {
            data: TypeFtz {
                type_: f16,
                flush_to_zero: Some(ftz)
            },
            arguments: NegArgs { dst: d, src: a, },
        }
    }
    neg{.ftz}.f16x2  d, a => {
        ast::Instruction::Neg {
            data: TypeFtz {
                type_: f16x2,
                flush_to_zero: Some(ftz)
            },
            arguments: NegArgs { dst: d, src: a, },
        }
    }
    neg.bf16         d, a => {
        ast::Instruction::Neg {
            data: TypeFtz {
                type_: bf16,
                flush_to_zero: None
            },
            arguments: NegArgs { dst: d, src: a, },
        }
    }
    neg.bf16x2       d, a => {
        ast::Instruction::Neg {
            data: TypeFtz {
                type_: bf16x2,
                flush_to_zero: None
            },
            arguments: NegArgs { dst: d, src: a, },
        }
    }
    ScalarType = { .f32, .f64, .f16, .f16x2, .bf16, .bf16x2 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-sin
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-cos
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-lg2
    sin.approx{.ftz}.f32  d, a => {
        ast::Instruction::Sin {
            data: ast::FlushToZero {
                flush_to_zero: ftz
            },
            arguments: SinArgs { dst: d, src: a, },
        }
    }
    cos.approx{.ftz}.f32  d, a => {
        ast::Instruction::Cos {
            data: ast::FlushToZero {
                flush_to_zero: ftz
            },
            arguments: CosArgs { dst: d, src: a, },
        }
    }
    lg2.approx{.ftz}.f32  d, a => {
        ast::Instruction::Lg2 {
            data: ast::FlushToZero {
                flush_to_zero: ftz
            },
            arguments: Lg2Args { dst: d, src: a, },
        }
    }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-ex2
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#half-precision-floating-point-instructions-ex2
    ex2.approx{.ftz}.f32  d, a => {
        ast::Instruction::Ex2 {
            data: ast::TypeFtz {
                type_: f32,
                flush_to_zero: Some(ftz)
            },
            arguments: Ex2Args { dst: d, src: a, },
        }
    }
    ex2.approx.atype     d, a => {
        ast::Instruction::Ex2 {
            data: ast::TypeFtz {
                type_: atype,
                flush_to_zero: None
            },
            arguments: Ex2Args { dst: d, src: a, },
        }
    }
    ex2.approx.ftz.btype d, a => {
        ast::Instruction::Ex2 {
            data: ast::TypeFtz {
                type_: btype,
                flush_to_zero: Some(true)
            },
            arguments: Ex2Args { dst: d, src: a, },
        }
    }
    .atype: ScalarType = { .f16,  .f16x2 };
    .btype: ScalarType = { .bf16, .bf16x2 };
    ScalarType = { .f32 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-clz
    clz.type  d, a => {
        ast::Instruction::Clz {
            data: type_,
            arguments: ClzArgs { dst: d, src: a, },
        }
    }
    .type: ScalarType = { .b32, .b64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-brev
    brev.type  d, a => {
        ast::Instruction::Brev {
            data: type_,
            arguments: BrevArgs { dst: d, src: a, },
        }
    }
    .type: ScalarType = { .b32, .b64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-popc
    popc.type  d, a => {
        ast::Instruction::Popc {
            data: type_,
            arguments: PopcArgs { dst: d, src: a, },
        }
    }
    .type: ScalarType = { .b32, .b64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#logic-and-shift-instructions-xor
    xor.type d, a, b => {
        ast::Instruction::Xor {
            data: type_,
            arguments: XorArgs { dst: d, src1: a, src2: b, },
        }
    }
    .type: ScalarType = { .pred, .b16, .b32, .b64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-rem
    rem.type  d, a, b => {
        ast::Instruction::Rem {
            data: type_,
            arguments: RemArgs { dst: d, src1: a, src2: b, },
        }
    }
    .type: ScalarType = { .u16, .u32, .u64, .s16, .s32, .s64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-bfe
    bfe.type  d, a, b, c => {
        ast::Instruction::Bfe {
            data: type_,
            arguments: BfeArgs { dst: d, src1: a, src2: b, src3: c },
        }
    }
    .type: ScalarType = { .u32, .u64, .s32, .s64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-bfi
    bfi.type  f, a, b, c, d => {
        ast::Instruction::Bfi {
            data: type_,
            arguments: BfiArgs { dst: f, src1: a, src2: b, src3: c, src4: d },
        }
    }
    .type: ScalarType = { .b32, .b64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#data-movement-and-conversion-instructions-prmt
    // prmt.b32{.mode}  d, a, b, c;
    // .mode = { .f4e, .b4e, .rc8, .ecl, .ecr, .rc16 };
    prmt.b32  d, a, b, c => {
        match c {
            ast::ParsedOperand::Imm(ImmediateValue::S64(control)) => ast::Instruction::Prmt {
                data: control as u16,
                arguments: PrmtArgs {
                    dst: d, src1: a, src2: b
                }
            },
            _ => ast::Instruction::PrmtSlow {
                arguments: PrmtSlowArgs {
                    dst: d, src1: a, src2: b, src3: c
                }
            }
        }
    }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#parallel-synchronization-and-communication-instructions-activemask
    activemask.b32 d => {
        ast::Instruction::Activemask {
            arguments: ActivemaskArgs { dst: d }
        }
    }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#parallel-synchronization-and-communication-instructions-membar
    // fence{.sem}.scope;
    // fence.op_restrict.release.cluster;
    // fence.proxy.proxykind;
    // fence.proxy.to_proxykind::from_proxykind.release.scope;
    // fence.proxy.to_proxykind::from_proxykind.acquire.scope  [addr], size;
    //membar.proxy.proxykind;
    //.sem       = { .sc, .acq_rel };
    //.scope     = { .cta, .cluster, .gpu, .sys };
    //.proxykind = { .alias, .async, async.global, .async.shared::{cta, cluster} };
    //.op_restrict = { .mbarrier_init };
    //.to_proxykind::from_proxykind = {.tensormap::generic};

    membar.level => {
        ast::Instruction::Membar { data: level }
    }
    membar.gl => {
        ast::Instruction::Membar { data: MemScope::Gpu }
    }
    .level: MemScope      = { .cta, .sys };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#control-flow-instructions-ret
    ret{.uni} => {
        Instruction::Ret { data: RetData { uniform: uni } }
    }

    mul24.mode.type  d, a, b => {
        ast::Instruction::Mul24 {
            data: ast::Mul24Details {
                control: mode,
                type_
            },
            arguments: Mul24Args { dst: d, src1: a, src2: b }
        }
    }

    .mode: Mul24Control = { .hi, .lo };
    .type: ScalarType = { .u32, .s32 };

);

#[cfg(test)]
mod tests {
    use crate::parse_module_checked;
    use crate::PtxError;

    use super::target;
    use super::PtxParserState;
    use super::Token;
    use logos::Logos;
    use logos::Span;
    use winnow::prelude::*;

    #[test]
    fn sm_11() {
        let text = ".target sm_11";
        let tokens = Token::lexer(text)
            .map(|t| t.map(|t| (t, Span::default())))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let mut errors = Vec::new();
        let stream = super::PtxParser {
            input: &tokens[..],
            state: PtxParserState::new(text, &mut errors),
        };
        assert_eq!(target.parse(stream).unwrap(), (11, None));
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn sm_90a() {
        let text = ".target sm_90a";
        let tokens = Token::lexer(text)
            .map(|t| t.map(|t| (t, Span::default())))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let mut errors = Vec::new();
        let stream = super::PtxParser {
            input: &tokens[..],
            state: PtxParserState::new(text, &mut errors),
        };
        assert_eq!(target.parse(stream).unwrap(), (90, Some('a')));
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn sm_90ab() {
        let text = ".target sm_90ab";
        let tokens = Token::lexer(text)
            .map(|t| t.map(|t| (t, Span::default())))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let mut errors = Vec::new();
        let stream = super::PtxParser {
            input: &tokens[..],
            state: PtxParserState::new(text, &mut errors),
        };
        assert!(target.parse(stream).is_err());
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn report_unknown_intruction() {
        let text = "
            .version 6.5
            .target sm_30
            .address_size 64

            .visible .entry add(
                .param .u64 input,
                .param .u64 output
            )
            {
                .reg .u64 	    in_addr;
                .reg .u64 	    out_addr;
                .reg .u64 	    temp;
                .reg .u64 	    temp2;

                ld.param.u64 	in_addr, [input];
                ld.param.u64 	out_addr, [output];

                ld.u64          temp, [in_addr];
                unknown_op1.asdf foobar;
                add.u64		    temp2, temp, 1;
                unknown_op2 temp2, temp;
                st.u64          [out_addr], temp2;
                ret;
            }";
        let errors = parse_module_checked(text).err().unwrap();
        assert_eq!(errors.len(), 2);
        assert!(matches!(
            errors[0],
            PtxError::UnrecognizedStatement(Some("unknown_op1.asdf foobar;"))
        ));
        assert!(matches!(
            errors[1],
            PtxError::UnrecognizedStatement(Some("unknown_op2 temp2, temp;"))
        ));
    }

    #[test]
    fn report_unknown_directive() {
        let text = "
            .version 6.5
            .target sm_30
            .address_size 64

            .broken_directive_fail; 34; {

            .visible .entry add(
                .param .u64 input,
                .param .u64 output
            )
            {
                .reg .u64 	    in_addr;
                .reg .u64 	    out_addr;
                .reg .u64 	    temp;
                .reg .u64 	    temp2;

                ld.param.u64 	in_addr, [input];
                ld.param.u64 	out_addr, [output];

                ld.u64          temp, [in_addr];
                add.u64		    temp2, temp, 1;
                st.u64          [out_addr], temp2;
                ret;
            }

            section foobar }";
        let errors = parse_module_checked(text).err().unwrap();
        assert_eq!(errors.len(), 2);
        assert!(matches!(
            errors[0],
            PtxError::UnrecognizedDirective(Some(".broken_directive_fail; 34; {"))
        ));
        assert!(matches!(
            errors[1],
            PtxError::UnrecognizedDirective(Some("section foobar }"))
        ));
    }
}
