use derive_more::Display;
use logos::Logos;
use ptx_parser_macros::derive_parser;
use rustc_hash::FxHashMap;
use std::fmt::Debug;
use std::num::{NonZeroU8, ParseFloatError, ParseIntError};
use std::{iter, usize};
use winnow::ascii::dec_uint;
use winnow::combinator::*;
use winnow::error::{ErrMode, ErrorKind};
use winnow::stream::Accumulate;
use winnow::token;
use winnow::token::take_till;
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

impl From<RawCpAsyncCacheOperator> for ast::CpAsyncCacheOperator {
    fn from(value: RawCpAsyncCacheOperator) -> Self {
        match value {
            RawCpAsyncCacheOperator::Ca => ast::CpAsyncCacheOperator::Cached,
            RawCpAsyncCacheOperator::Cg => ast::CpAsyncCacheOperator::L2Only,
        }
    }
}

impl From<RawRoundingMode> for ast::RoundingMode {
    fn from(value: RawRoundingMode) -> Self {
        value.normalize().0
    }
}

impl RawRoundingMode {
    fn normalize(self) -> (ast::RoundingMode, bool) {
        match self {
            RawRoundingMode::Rn => (ast::RoundingMode::NearestEven, false),
            RawRoundingMode::Rz => (ast::RoundingMode::Zero, false),
            RawRoundingMode::Rm => (ast::RoundingMode::NegativeInf, false),
            RawRoundingMode::Rp => (ast::RoundingMode::PositiveInf, false),
            RawRoundingMode::Rni => (ast::RoundingMode::NearestEven, true),
            RawRoundingMode::Rzi => (ast::RoundingMode::Zero, true),
            RawRoundingMode::Rmi => (ast::RoundingMode::NegativeInf, true),
            RawRoundingMode::Rpi => (ast::RoundingMode::PositiveInf, true),
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
    // .callprototype and .calltargets labels: the signature of indirect calls
    call_prototypes:
        FxHashMap<&'input str, (Vec<(ast::Type, StateSpace)>, Vec<(ast::Type, StateSpace)>)>,
    // .branchtargets labels
    branch_targets: FxHashMap<&'input str, Vec<&'input str>>,
    // .file directives: index and name
    files: Vec<(u32, &'input str)>,
}

impl<'a, 'input> PtxParserState<'a, 'input> {
    fn new(text: &'input str, errors: &'a mut Vec<PtxError<'input>>) -> Self {
        Self {
            text,
            errors,
            function_declarations: FxHashMap::default(),
            call_prototypes: FxHashMap::default(),
            branch_targets: FxHashMap::default(),
            files: Vec::new(),
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
            .map(|var| (var.info.v_type.clone(), var.info.state_space))
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
    trace(
        "ident",
        token::any.verify_map(|(t, _)| {
            if let Token::Ident(text) = t {
                if matches!(
                    text,
                    "%envreg1"
                        | "%envreg2"
                        | "%envreg3"
                        | "%envreg4"
                        | "%envreg5"
                        | "%envreg6"
                        | "%envreg7"
                        | "%envreg8"
                        | "%envreg9"
                        | "%envreg10"
                        | "%envreg11"
                        | "%envreg12"
                        | "%envreg13"
                        | "%envreg14"
                        | "%envreg15"
                        | "%envreg16"
                        | "%envreg17"
                        | "%envreg18"
                        | "%envreg19"
                        | "%envreg20"
                        | "%envreg21"
                        | "%envreg22"
                        | "%envreg23"
                        | "%envreg24"
                        | "%envreg25"
                        | "%envreg26"
                        | "%envreg27"
                        | "%envreg28"
                        | "%envreg29"
                        | "%envreg30"
                        | "%envreg31"
                        | "%envreg32"
                ) {
                    stream.state.errors.push(PtxError::UnsupportedSreg(text));
                }
                Some(text)
            } else if let Some(text) = t.opcode_text() {
                Some(text)
            } else {
                None
            }
        }),
    )
    .parse_next(&mut stream.input)
}

fn dot_ident<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<&'input str> {
    trace(
        "dot_ident",
        token::any.verify_map(|(t, _)| {
            if let Token::DotIdent(text) = t {
                Some(text)
            } else {
                None
            }
        }),
    )
    .parse_next(stream)
}

// The next token if its text is `text` (for the words the lexer has no dedicated token for)
fn token_text<'a, 'input>(stream: &mut PtxParser<'a, 'input>, text: &str) -> PResult<()> {
    match stream.input.first() {
        Some((_, span)) if &stream.state.text[span.clone()] == text => {
            stream.input = &stream.input[1..];
            Ok(())
        }
        _ => backtrack(),
    }
}

fn num<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<(&'input str, u32, bool)> {
    trace(
        "num",
        token::any.verify_map(|(t, _)| {
            Some(match t {
                Token::Hex(s) => {
                    if s.ends_with('U') {
                        (&s[2..s.len() - 1], 16, true)
                    } else {
                        (&s[2..], 16, false)
                    }
                }
                Token::Binary(s) => {
                    if s.ends_with('U') {
                        (&s[2..s.len() - 1], 2, true)
                    } else {
                        (&s[2..], 2, false)
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
        }),
    )
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

fn f32<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<f32> {
    take_error(token::any.verify_map(|(t, _)| match t {
        Token::F32Hex(f) => Some(match u32::from_str_radix(&f[2..], 16) {
            Ok(x) => Ok(f32::from_bits(x)),
            Err(err) => Err((0.0, PtxError::from(err))),
        }),
        _ => None,
    }))
    .parse_next(stream)
}

fn f64<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<f64> {
    // A negative decimal literal: -1.5
    let negative = opt(Token::Minus).parse_next(stream)?.is_some();
    let value = f64_unsigned.parse_next(stream)?;
    Ok(if negative { -value } else { value })
}

fn f64_unsigned<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<f64> {
    take_error(token::any.verify_map(|(t, _)| match t {
        Token::F64Hex(f) => Some(match u64::from_str_radix(&f[2..], 16) {
            Ok(x) => Ok(f64::from_bits(x)),
            Err(err) => Err((0.0, PtxError::from(err))),
        }),
        Token::F64(f) => Some(match f.parse::<f64>() {
            Ok(x) => Ok(x),
            Err(err) => Err((0.0, PtxError::from(err))),
        }),
        _ => None,
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
    trace(
        "u32",
        take_error(num.map(|x| {
            let (text, radix, _) = x;
            match u32::from_str_radix(text, radix) {
                Ok(x) => Ok(x),
                Err(err) => Err((0, PtxError::from(err))),
            }
        })),
    )
    .parse_next(stream)
}

fn constant<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<ast::ImmediateValue> {
    // Currently the only built-in constant is WARP_SZ
    // If new ones are added, we can change this to use a Token::Constant(&str) instead
    token::any
        .verify_map(|(t, _)| {
            if t == Token::WarpSz {
                Some(ast::ImmediateValue::U64(32))
            } else {
                None
            }
        })
        .parse_next(stream)
}

fn immediate_value<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<ast::ImmediateValue> {
    trace(
        "immediate_value",
        const_expression.map(|(value, literal)| {
            literal.unwrap_or(match value {
                ConstValue::Int(v, true) => ast::ImmediateValue::U64(v),
                ConstValue::Int(v, false) => ast::ImmediateValue::S64(v as i64),
                ConstValue::Float(f) => ast::ImmediateValue::F64(f),
            })
        }),
    )
    .parse_next(stream)
}

// `.x` `.y` `.z` `.w` or `.r` `.g` `.b` `.a`: the element of a vector register. Matched by text, since some of these
// are also instruction modifiers with their own tokens.
fn vector_member_index(text: &str) -> Option<u8> {
    match text {
        ".x" | ".r" => Some(0),
        ".y" | ".g" => Some(1),
        ".z" | ".b" => Some(2),
        ".w" | ".a" => Some(3),
        _ => None,
    }
}

fn vector_member<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<u8> {
    match stream.input.first() {
        Some((_, span)) => match vector_member_index(&stream.state.text[span.clone()]) {
            Some(index) => {
                stream.input = &stream.input[1..];
                Ok(index)
            }
            None => backtrack(),
        },
        None => backtrack(),
    }
}

fn int_literal_value<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<ast::ImmediateValue> {
    take_error(num.map(|(text, radix, is_unsigned)| {
        if !is_unsigned {
            if let Ok(x) = i64::from_str_radix(text, radix) {
                return Ok(ast::ImmediateValue::S64(x));
            }
        }
        match u64::from_str_radix(text, radix) {
            Ok(x) => Ok(ast::ImmediateValue::U64(x)),
            Err(err) => Err((ast::ImmediateValue::U64(0), PtxError::from(err))),
        }
    }))
    .parse_next(stream)
}

// Constant expressions (PTX ISA 4.5.4 - 4.5.6): C operators and precedence over 64-bit integers, signed (.s64)
// unless a literal has the U suffix or does not fit, with the usual arithmetic conversions, `%` on .u64 operands,
// shifts of the first operand's type (arithmetic for signed), comparisons and logical operators giving .s64 0 / 1 and
// casts `(.s64)` / `(.u64)`; and + - * / over floating point literals.
#[derive(Clone, Copy, Debug)]
enum ConstValue {
    Int(u64, bool),
    Float(f64),
}

impl ConstValue {
    fn bits(self) -> u64 {
        match self {
            ConstValue::Int(v, _) => v,
            ConstValue::Float(f) => f as i64 as u64,
        }
    }
    fn is_true(self) -> bool {
        match self {
            ConstValue::Int(v, _) => v != 0,
            ConstValue::Float(f) => f != 0.0,
        }
    }
    fn as_f64(self) -> f64 {
        match self {
            ConstValue::Int(v, true) => v as f64,
            ConstValue::Int(v, false) => v as i64 as f64,
            ConstValue::Float(f) => f,
        }
    }
    fn bool(b: bool) -> Self {
        ConstValue::Int(b as u64, false)
    }
}

fn const_binary(op: &str, a: ConstValue, b: ConstValue) -> ConstValue {
    use ConstValue::*;
    if let (Float(_), _) | (_, Float(_)) = (a, b) {
        let (x, y) = (a.as_f64(), b.as_f64());
        return match op {
            "+" => Float(x + y),
            "-" => Float(x - y),
            "*" => Float(x * y),
            "/" => Float(x / y),
            "<" => ConstValue::bool(x < y),
            ">" => ConstValue::bool(x > y),
            "<=" => ConstValue::bool(x <= y),
            ">=" => ConstValue::bool(x >= y),
            "==" => ConstValue::bool(x == y),
            "!=" => ConstValue::bool(x != y),
            "&&" => ConstValue::bool(x != 0.0 && y != 0.0),
            "||" => ConstValue::bool(x != 0.0 || y != 0.0),
            _ => Float(f64::NAN),
        };
    }
    let (Int(x, ua), Int(y, ub)) = (a, b) else { unreachable!() };
    let u = ua || ub;
    match op {
        "+" => Int(x.wrapping_add(y), u),
        "-" => Int(x.wrapping_sub(y), u),
        "*" => Int(x.wrapping_mul(y), u),
        "/" if y == 0 => Int(0, u),
        "/" if u => Int(x / y, u),
        "/" => Int((x as i64).wrapping_div(y as i64) as u64, u),
        "%" if y == 0 => Int(0, true),
        "%" => Int(x % y, true),
        "<<" => Int(x.wrapping_shl(y as u32), ua),
        ">>" if ua => Int(x.wrapping_shr(y as u32), ua),
        ">>" => Int((x as i64).wrapping_shr(y as u32) as u64, ua),
        "&" => Int(x & y, u),
        "|" => Int(x | y, u),
        "^" => Int(x ^ y, u),
        "&&" => ConstValue::bool(x != 0 && y != 0),
        "||" => ConstValue::bool(x != 0 || y != 0),
        "==" => ConstValue::bool(x == y),
        "!=" => ConstValue::bool(x != y),
        "<" | ">" | "<=" | ">=" => {
            let ord = if u { x.cmp(&y) } else { (x as i64).cmp(&(y as i64)) };
            ConstValue::bool(match op {
                "<" => ord.is_lt(),
                ">" => ord.is_gt(),
                "<=" => ord.is_le(),
                _ => ord.is_ge(),
            })
        }
        _ => Int(0, u),
    }
}

// The binary operator at the head of the stream, by precedence level (0 = `||` ... 9 = `*`), without consuming it.
fn peek_binary(stream: &PtxParser) -> Option<(&'static str, usize, u8)> {
    let t = |i: usize| stream.input.get(i).map(|(t, _)| *t);
    let two = |a: Token, b: Token| t(0) == Some(a) && t(1) == Some(b);
    Some(match t(0)? {
        Token::Pipe if two(Token::Pipe, Token::Pipe) => ("||", 2, 0),
        Token::Amp if two(Token::Amp, Token::Amp) => ("&&", 2, 1),
        Token::Pipe => ("|", 1, 2),
        Token::Caret => ("^", 1, 3),
        Token::Amp => ("&", 1, 4),
        Token::Eq if two(Token::Eq, Token::Eq) => ("==", 2, 5),
        Token::Exclamation if two(Token::Exclamation, Token::Eq) => ("!=", 2, 5),
        Token::Lt if two(Token::Lt, Token::Lt) => ("<<", 2, 7),
        Token::Gt if two(Token::Gt, Token::Gt) => (">>", 2, 7),
        Token::Lt if two(Token::Lt, Token::Eq) => ("<=", 2, 6),
        Token::Gt if two(Token::Gt, Token::Eq) => (">=", 2, 6),
        Token::Lt => ("<", 1, 6),
        Token::Gt => (">", 1, 6),
        Token::Plus => ("+", 1, 8),
        Token::Minus => ("-", 1, 8),
        Token::Star => ("*", 1, 9),
        Token::Slash => ("/", 1, 9),
        Token::Percent => ("%", 1, 9),
        _ => return None,
    })
}

// A constant expression: its value, and the literal itself when it is a single literal (kept exactly, e.g. 0f...).
fn const_expression<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<(ConstValue, Option<ast::ImmediateValue>)> {
    fn primary<'a, 'input>(
        stream: &mut PtxParser<'a, 'input>,
    ) -> PResult<(ConstValue, Option<ast::ImmediateValue>)> {
        let literal = alt((
            int_literal_value,
            f32.map(ast::ImmediateValue::F32),
            f64_unsigned.map(ast::ImmediateValue::F64),
            constant,
        ))
        .parse_next(stream);
        if let Ok(imm) = literal {
            let value = match imm {
                ast::ImmediateValue::U64(v) => ConstValue::Int(v, true),
                ast::ImmediateValue::S64(v) => ConstValue::Int(v as u64, false),
                ast::ImmediateValue::F32(f) => ConstValue::Float(f as f64),
                ast::ImmediateValue::F64(f) => ConstValue::Float(f),
            };
            return Ok((value, Some(imm)));
        }
        // a cast: (.s64) / (.u64)
        let checkpoint = stream.input;
        if eat(stream, Token::LParen) {
            let cast = match stream.input.first() {
                Some((_, span)) => match &stream.state.text[span.clone()] {
                    ".s64" => Some(false),
                    ".u64" => Some(true),
                    _ => None,
                },
                None => None,
            };
            if let Some(unsigned) = cast {
                stream.input = &stream.input[1..];
                if !eat(stream, Token::RParen) {
                    return backtrack();
                }
                let (v, _) = unary(stream)?;
                return Ok((ConstValue::Int(v.bits(), unsigned), None));
            }
            stream.input = checkpoint;
        }
        let (v, _) = delimited(Token::LParen, const_expression, Token::RParen).parse_next(stream)?;
        Ok((v, None))
    }
    fn unary<'a, 'input>(
        stream: &mut PtxParser<'a, 'input>,
    ) -> PResult<(ConstValue, Option<ast::ImmediateValue>)> {
        let op = match stream.input.first().map(|(t, _)| *t) {
            Some(Token::Minus) => '-',
            Some(Token::Plus) => '+',
            Some(Token::Exclamation) => '!',
            Some(Token::Tilde) => '~',
            _ => return primary(stream),
        };
        stream.input = &stream.input[1..];
        let (v, lit) = unary(stream)?;
        Ok(match (op, v) {
            ('+', _) => (v, lit),
            ('-', ConstValue::Float(f)) => (ConstValue::Float(-f), lit.map(|_| ast::ImmediateValue::F64(-f))),
            ('-', ConstValue::Int(x, u)) => {
                // -literal stays a literal of the same kind, as `-5` always parsed
                let lit = match lit {
                    Some(ast::ImmediateValue::S64(x)) => Some(ast::ImmediateValue::S64(x.wrapping_neg())),
                    _ => None,
                };
                (ConstValue::Int(x.wrapping_neg(), u), lit)
            }
            ('!', v) => (ConstValue::bool(!v.is_true()), None),
            (_, v) => (ConstValue::Int(!v.bits(), matches!(v, ConstValue::Int(_, true))), None),
        })
    }
    fn binary<'a, 'input>(
        stream: &mut PtxParser<'a, 'input>,
        min_level: u8,
    ) -> PResult<(ConstValue, Option<ast::ImmediateValue>)> {
        let (mut lhs, mut lit) = unary(stream)?;
        while let Some((op, len, level)) = peek_binary(stream) {
            if level < min_level {
                break;
            }
            stream.input = &stream.input[len..];
            let (rhs, _) = binary(stream, level + 1)?;
            lhs = const_binary(op, lhs, rhs);
            lit = None;
        }
        Ok((lhs, lit))
    }
    let (cond, lit) = binary(stream, 0)?;
    if eat(stream, Token::Question) {
        let (a, _) = const_expression(stream)?;
        if !eat(stream, Token::Colon) {
            return backtrack();
        }
        let (b, _) = const_expression(stream)?;
        let pick = if cond.is_true() { a } else { b };
        let value = match (pick, a, b) {
            (ConstValue::Int(v, _), ConstValue::Int(_, ua), ConstValue::Int(_, ub)) => ConstValue::Int(v, ua || ub),
            _ => pick,
        };
        return Ok((value, None));
    }
    Ok((cond, lit))
}

fn eat(stream: &mut PtxParser, token: Token) -> bool {
    match stream.input.first() {
        Some((t, _)) if *t == token => {
            stream.input = &stream.input[1..];
            true
        }
        _ => false,
    }
}

// An integer constant expression, as an offset or a size.
fn const_expr<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<i64> {
    const_expression.map(|(v, _)| v.bits() as i64).parse_next(stream)
}

// The constant byte offset of an address `r + offset`: everything up to the end of the operand, so that
// `r + 72 - 8` is r + 64.
fn const_offset<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<i32> {
    const_expr.map(|v| v as i32).parse_next(stream)
}

// The `[index]` of an array element operand: a constant expression (a byte offset) or a register plus or minus one.
fn array_index<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<(Option<&'input str>, i64)> {
    alt((
        (
            ident,
            opt((alt((Token::Plus.value(1i64), Token::Minus.value(-1i64))), const_expr)),
        )
            .map(|(reg, offset)| (Some(reg), offset.map_or(0, |(sign, v)| sign * v))),
        const_expr.map(|offset| (None, offset)),
    ))
    .parse_next(stream)
}

fn reg_or_immediate<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::RegOrImmediate<&'input str>> {
    trace(
        "reg_or_immediate",
        alt((
            immediate_value.map(|imm| ast::RegOrImmediate::Imm(imm)),
            (ident, opt(vector_member)).map(|(id, member)| match member {
                Some(member) => ast::RegOrImmediate::VecMember(id, member),
                None => ast::RegOrImmediate::Reg(id),
            }),
        )),
    )
    .parse_next(stream)
}

// A value in a variable initializer: also `generic(name)` (the address of a
// variable, the same in the generic space here) and `name + offset`
fn initializer_value<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::RegOrImmediate<&'input str>> {
    if token_text(stream, "generic").is_ok() {
        let (name, offset) =
            (delimited(Token::LParen, ident, Token::RParen), opt(preceded(Token::Plus, u32))).parse_next(stream)?;
        return Ok(match offset {
            Some(offset) => ast::RegOrImmediate::RegOffset(name, offset as i64),
            None => ast::RegOrImmediate::Reg(name),
        });
    }
    // mask(expr): the bits of a constant, or of the address of a variable plus an offset, under an 0xFF mask, moved
    // down to bit 0 (PTX ISA 5.4.4)
    let checkpoint = stream.checkpoint();
    if let Ok(mask) = terminated(int_literal_value, Token::LParen).parse_next(stream) {
        let mask = match mask {
            ast::ImmediateValue::U64(x) => x,
            ast::ImmediateValue::S64(x) => x as u64,
            _ => 0,
        };
        if mask == 0 || (mask >> mask.trailing_zeros()) != 0xff {
            return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
        }
        let shift = mask.trailing_zeros();
        let inner = stream.checkpoint();
        if let Ok(value) = terminated(const_expr, Token::RParen).parse_next(stream) {
            return Ok(ast::RegOrImmediate::Imm(ast::ImmediateValue::U64((value as u64 & mask) >> shift)));
        }
        stream.reset(&inner);
        let (name, offset) =
            terminated((ident, opt(preceded(Token::Plus, u32))), Token::RParen).parse_next(stream)?;
        return Ok(ast::RegOrImmediate::MaskedAddress(name, offset.unwrap_or(0) as i64, shift as u8));
    }
    stream.reset(&checkpoint);
    let value = reg_or_immediate.parse_next(stream)?;
    match value {
        ast::RegOrImmediate::Reg(name) => {
            let offset = opt(preceded(Token::Plus, u32)).parse_next(stream)?;
            Ok(match offset {
                Some(offset) => ast::RegOrImmediate::RegOffset(name, offset as i64),
                None => value,
            })
        }
        _ => Ok(value),
    }
}

pub fn parse_for_errors<'input>(text: &'input str) -> Vec<PtxError<'input>> {
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
) -> (Vec<(Token<'input>, logos::Span)>, Vec<PtxError<'input>>) {
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
) -> Result<ast::Module<'input>, Vec<PtxError<'input>>> {
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
        Ok(result) if errors.is_empty() && result.invalid_directives == 0 => Ok(result),
        Ok(_) => Err(errors),
        Err(err) => {
            errors.push(err);
            Err(errors)
        }
    }
}

pub fn parse_module_unchecked<'input>(text: &'input str) -> ast::Module<'input> {
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
        return ast::Module::empty();
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
    parse_result.unwrap_or(ast::Module::empty())
}

fn module<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<ast::Module<'input>> {
    let mut m = module_body.parse_next(stream)?;
    m.files = std::mem::take(&mut stream.state.files);
    Ok(m)
}

fn module_body<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<ast::Module<'input>> {
    trace(
        "module",
        (
            version,
            target,
            opt(address_size),
            repeat_without_none_and_count(directive),
            eof,
        )
            .map(
                |(version, (target, _), address_size, (directives, invalid_directives), _)| {
                    ast::Module {
                        ptx_version: version,
                        sm_version: target,
                        address_size: address_size.unwrap_or(32),
                        directives: directives.into_iter().flatten().collect(),
                        invalid_directives,
                        files: Vec::new(),
                    }
                },
            ),
    )
    .parse_next(stream)
}

fn address_size<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<u8> {
    (
        Token::DotAddressSize,
        alt((u8_literal(64).value(64u8), u8_literal(32).value(32u8))),
    )
        .map(|(_, size)| size)
        .parse_next(stream)
}

fn version<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<(u8, u8)> {
    (Token::DotVersion, f64)
        .map(|(_, version)| {
            // This is a bit imprecise but should work for now
            // A better solution might be to include major and minor in the DotVersion token
            let major = version.floor() as u8;
            let minor = ((version - version.floor()) * 10.0).round() as u8;
            (major, minor)
        })
        .parse_next(stream)
}

fn target<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<(u32, Option<char>)> {
    preceded(
        Token::DotTarget,
        (
            ident.and_then(shader_model),
            // debug, texmode_unified, texmode_independent, map_f64_to_f32
            repeat::<_, _, (), _, _>(0.., (Token::Comma, ident)),
        ),
    )
    .map(|((target, arch_variant), _options)| (target, arch_variant))
    .parse_next(stream)
}

// compute_NN is a synonym of sm_NN
fn shader_model<'a>(stream: &mut &str) -> PResult<(u32, Option<char>)> {
    (
        alt(("sm_", "compute_")),
        dec_uint,
        opt(token::any.verify(|c: &char| c.is_ascii_lowercase())),
        eof,
    )
        .map(|(_, digits, arch_variant, _)| (digits, arch_variant))
        .parse_next(stream)
}

/// Parses a directive from the input stream.
///
/// The return type is a double-Option:
/// - The outer `Option` indicates parse success: `None` means a parse error occurred, `Some` means parsing succeeded.
/// - The inner `Option` indicates whether the directive should be included in the AST:
///     - `None` means the directive should be ignored (e.g., directives like `.file` or `.section`).
///     - `Some` contains the actual directive to include in the AST.
fn directive<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<Option<Vec<ast::Directive<'input, ast::ParsedOperand<&'input str>>>>> {
    let errors = stream.state.errors.len();
    let directive = trace(
        "directive",
        with_recovery(
            alt((
                // When adding a new variant here remember to add its first token into recovery parser down below
                function.map(|(linking, func)| vec![ast::Directive::Method(linking, func)]),
                file.map(|_| Vec::new()),
                section.map(|_| Vec::new()),
                pragma.map(|_| Vec::new()),
                // .alias name, aliasee; (calls go to the aliasee by name)
                (|stream: &mut PtxParser<'a, 'input>| token_text(stream, ".alias"), ident, Token::Comma, ident, Token::Semicolon)
                    .map(|_| Vec::new()),
                (module_variable, Token::Semicolon).map(|((linking, vars), _)| {
                    vars.into_iter().map(|var| ast::Directive::Variable(linking, var)).collect()
                }),
            )),
            (
                token::any,
                take_till(1.., |(token, _)| match token {
            // visibility
            Token::DotExtern | Token::DotVisible | Token::DotWeak
            // methods
            | Token::DotFunc | Token::DotEntry
            // module variables
            | Token::DotGlobal | Token::DotConst | Token::DotShared
            // other sections
            | Token::DotFile | Token::DotSection | Token::DotPragma => true,
            _ => false,
        }),
            )
                .map(|(_, x)| x),
            |text| PtxError::UnrecognizedDirective(text.unwrap_or("")),
        ),
    )
    .parse_next(stream)?;
    if errors != stream.state.errors.len() {
        return Ok(None);
    }
    Ok(directive)
}

fn module_variable<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<(ast::LinkingDirective, Vec<ast::Variable<&'input str>>)> {
    let linking = linking_directives.parse_next(stream)?;
    // .attribute(.managed): managed memory is ordinary device memory here
    let var = terminated(
        global_space,
        opt(|stream: &mut PtxParser<'a, 'input>| {
            token_text(stream, ".attribute")?;
            (Token::LParen, dot_ident, Token::RParen).void().parse_next(stream)
        }),
    )
        .flat_map(|space| multi_variable(linking.contains(LinkingDirective::EXTERN), space))
        .map(|multi_var| {
            // Parameterized names (`g<3>` declares g0, g1, g2) are not in the text: their names are kept for the
            // life of the program, a few bytes per declaration.
            let numbered = |name: &str, count: u32| {
                (0..count).map(move |i| &*Box::leak(format!("{name}{i}").into_boxed_str())).collect::<Vec<&'input str>>()
            };
            match multi_var {
                MultiVariable::Names { info, names } => {
                    names.into_iter().map(|name| ast::Variable { info: info.clone(), name }).collect()
                }
                MultiVariable::Parameterized { info, name, count } => numbered(name, count)
                    .into_iter()
                    .map(|name| ast::Variable { info: info.clone(), name })
                    .collect(),
                MultiVariable::List { info, names } => names
                    .into_iter()
                    .flat_map(|(name, count)| match count {
                        Some(count) => numbered(name, count),
                        None => vec![name],
                    })
                    .map(|name| ast::Variable { info: info.clone(), name })
                    .collect::<Vec<_>>(),
            }
        })
        .parse_next(stream)?;
    Ok((linking, var))
}

// .file index "name" {, timestamp, size}
fn file<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<()> {
    let (index, name) = trace(
        "file",
        (
            Token::DotFile,
            u32,
            token::any.verify_map(|(t, span): (Token<'input>, logos::Span)| matches!(t, Token::String).then_some(span)),
            opt((Token::Comma, u32, Token::Comma, u32)),
        )
            .map(|(_, index, name, _)| (index, name)),
    )
    .parse_next(stream)?;
    let name = stream.state.text.get(name).unwrap_or("").trim_matches('"');
    stream.state.files.push((index, name));
    Ok(())
}

fn section<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<()> {
    trace(
        "section",
        (
            Token::DotSection.void(),
            dot_ident.void(),
            Token::LBrace.void(),
            repeat::<_, _, (), _, _>(0.., section_dwarf_line),
            Token::RBrace.void(),
        )
            .void(),
    )
    .parse_next(stream)
}

fn section_dwarf_line<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<()> {
    alt((
        (section_label, Token::Colon).void(),
        (Token::DotB32, section_label, opt(alt(((Token::Plus, u32).void(), (Token::Minus, section_label).void())))).void(),
        (Token::DotB64, section_label, opt(alt(((Token::Plus, u32).void(), (Token::Minus, section_label).void())))).void(),
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
    trace("section_label", alt((ident, dot_ident)))
        .void()
        .parse_next(stream)
}

fn function<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<(
    ast::LinkingDirective,
    ast::Function<'input, &'input str, ast::Statement<ParsedOperand<&'input str>>>,
)> {
    let (linking, mut func_directive) =
        trace("function", (linking_directives, method_declaration)).parse_next(stream)?;
    // A last unsized `.param .b8 va[]` takes variable arguments, which a call may leave out (PTX ISA 11.2.2). Calls
    // must leave them out here: the parameter is a variable of the function, of one element.
    let variadic = match func_directive.input_arguments.last() {
        Some(ast::Variable {
            info:
                ast::VariableInfo {
                    v_type: ast::Type::Array(_, _, dims),
                    state_space: StateSpace::Param,
                    ..
                },
            ..
        }) if dims[0] == 0 => func_directive.input_arguments.pop().map(|mut var| {
            if let ast::Type::Array(_, _, dims) = &mut var.info.v_type {
                dims[0] = 1;
            }
            var
        }),
        _ => None,
    };
    if func_directive.input_arguments.iter().any(|var| {
        matches!(&var.info.v_type, ast::Type::Array(_, _, dims) if dims[0] == 0)
    }) {
        return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
    }
    // Recorded before the body, which can call the function recursively
    stream.state.record_function(&func_directive);
    prescan_branch_targets(stream);
    // .pragma can sit among the tuning directives
    let (tuning, body): (Vec<Option<ast::TuningDirective>>, _) = (
        // (there without the semicolon too)
        repeat(
            0..,
            alt((
                tuning_directive.map(Some),
                (
                    Token::DotPragma,
                    separated::<_, _, (), _, _, _, _>(1.., Token::String, Token::Comma),
                    opt(Token::Semicolon),
                )
                    .map(|_| None),
            )),
        ),
        function_body,
    )
        .parse_next(stream)?;
    let tuning = tuning.into_iter().flatten().collect();
    let body = match (body, variadic) {
        (Some(mut body), Some(var)) => {
            body.insert(
                0,
                ast::Statement::Variable(ast::MultiVariable::Names {
                    info: var.info,
                    names: vec![var.name],
                }),
            );
            Some(body)
        }
        (body, _) => body,
    };
    Ok((
        linking,
        ast::Function {
            func_directive,
            tuning,
            body,
        },
    ))
}

fn linking_directives<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::LinkingDirective> {
    trace(
        "linking_directives",
        repeat(
            0..,
            dispatch! { token::any;
                (Token::DotExtern, _) => empty.value(ast::LinkingDirective::EXTERN),
                (Token::DotVisible, _) => empty.value(ast::LinkingDirective::VISIBLE),
                (Token::DotWeak, _) => empty.value(ast::LinkingDirective::WEAK),
                _ => fail
            },
        )
        .fold(|| ast::LinkingDirective::NONE, |x, y| x | y),
    )
    .parse_next(stream)
}

fn tuning_directive<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::TuningDirective> {
    dispatch! {token::any;
        (Token::DotMaxnreg, _) => u32.map(ast::TuningDirective::MaxNReg),
        (Token::DotMaxntid, _) => tuple1to3_u32.map(|(nx, ny, nz)| ast::TuningDirective::MaxNtid(nx, ny, nz)),
        (Token::DotReqntid, _) => tuple1to3_u32.map(|(nx, ny, nz)| ast::TuningDirective::ReqNtid(nx, ny, nz)),
        (Token::DotMinnctapersm, _) => u32.map(ast::TuningDirective::MinNCtaPerSm),
        (Token::DotNoreturn, _) => empty.map(|_| ast::TuningDirective::NoReturn),
        (Token::DotReqnctapercluster, _) => tuple1to3_u32.map(|(nx, ny, nz)| ast::TuningDirective::ReqNctaPerCluster(nx, ny, nz)),
        (Token::DotExplicitcluster, _) => empty.map(|_| ast::TuningDirective::ExplicitCluster),
        (Token::DotMaxclusterrank, _) => u32.map(ast::TuningDirective::MaxClusterRank),
        _ => fail
    }
    .parse_next(stream)
}

fn method_declaration<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::MethodDeclaration<'input, &'input str>> {
    dispatch! {token::any;
        (Token::DotEntry, _) => (ident, kernel_arguments).map(|(name, input_arguments)| ast::MethodDeclaration{
            return_arguments: Vec::new(), name: ast::MethodName::Kernel(name), input_arguments, shared_mem: None
        }),
        (Token::DotFunc, _) => (opt(function_attribute), opt(fn_arguments), ident, fn_arguments).map(|(_, r, n, i)| (r, n, i)).map(|(return_arguments, name,input_arguments)| {
            let return_arguments = return_arguments.unwrap_or_else(|| Vec::new());
            let name = ast::MethodName::Func(name);
            ast::MethodDeclaration{ return_arguments, name, input_arguments, shared_mem: None }
        }),
        _ => fail
    }
    .parse_next(stream)
}

// .func .attribute(.unified(0xAB, 0xCD)): ignored, the attributes (.unified) only name the function for unified
// function pointers
fn function_attribute<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<()> {
    token_text(stream, ".attribute")?;
    Token::LParen.parse_next(stream)?;
    let mut depth = 1usize;
    while depth > 0 {
        match token::any.parse_next(stream)?.0 {
            Token::LParen => depth += 1,
            Token::RParen => depth -= 1,
            _ => {}
        }
    }
    Ok(())
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
    preceded(Token::DotParam, method_parameter(StateSpace::Param, true)).parse_next(stream)
}

fn fn_input<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<ast::Variable<&'input str>> {
    dispatch! { token::any;
        (Token::DotParam, _) => method_parameter(StateSpace::Param, false),
        (Token::DotReg, _) => method_parameter(StateSpace::Reg, false),
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
    trace("function_body", dispatch! {token::any;
        (Token::LBrace, _) => terminated(repeat_without_none(statement), Token::RBrace).map(Some),
        (Token::Semicolon, _) => empty.map(|_| None),
        _ => fail
    })
    .parse_next(stream)
}

fn statement<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<Option<Statement<ParsedOperandStr<'input>>>> {
    with_recovery(
        alt((
            call_target_directive.map(|_| None),
            label.map(Some),
            debug_directive.map(|l| Some(Statement::Loc(l))),
            terminated(
                method_space
                    .flat_map(|space| multi_variable(false, space))
                    .map(|var| Some(Statement::Variable(var))),
                Token::Semicolon,
            ),
            // `.pred %p;` declares predicate registers like `.reg .pred %p;`
            terminated(
                winnow::combinator::preceded(
                    winnow::combinator::peek(Token::DotPred),
                    multi_variable(false, StateSpace::Reg),
                )
                .map(|var| Some(Statement::Variable(var))),
                Token::Semicolon,
            ),
            brx_idx.map(Some),
            predicated_instruction.map(Some),
            pragma.map(|_| None),
            block_statement.map(Some),
        )),
        take_till_end_of_statement(),
        |text| PtxError::UnrecognizedStatement(text.unwrap_or("")),
    )
    .map(Option::flatten)
    .parse_next(stream)
}

fn take_till_end_of_statement<
    'a,
    I: Stream<Token = (Token<'a>, std::ops::Range<usize>)>,
    E: ParserError<I>,
>() -> impl Parser<I, <I as Stream>::Slice, E> {
    trace("take_till_end_of_statement", move |stream: &mut I| {
        let mut depth = 0;

        let mut iterator = stream.iter_offsets().peekable();
        while let Some((current_offset, (token, _))) = iterator.next() {
            match token {
                Token::LBrace => {
                    depth += 1;
                }
                Token::RBrace => {
                    if depth == 0 {
                        return Err(ErrMode::from_error_kind(
                            stream,
                            winnow::error::ErrorKind::Token,
                        ));
                    }
                    depth -= 1;
                }
                Token::Semicolon | Token::Colon => {
                    let offset = if let Some((next_offset, _)) = iterator.peek() {
                        *next_offset
                    } else {
                        current_offset
                    };
                    return Ok(stream.next_slice(offset));
                }
                _ => {}
            }
        }

        Err(ParserError::from_error_kind(stream, ErrorKind::Eof))
    })
}

fn with_recovery<'a, 'input: 'a, T>(
    mut parser: impl Parser<PtxParser<'a, 'input>, T, ContextError>,
    recovery: impl Parser<PtxParser<'a, 'input>, &'a [(Token<'input>, logos::Span)], ContextError>,
    mut error: impl FnMut(Option<&'input str>) -> PtxError<'input>,
) -> impl Parser<PtxParser<'a, 'input>, Option<T>, ContextError> {
    let mut recovery = trace("recovery", recovery);
    trace(
        "with_recovery",
        move |stream: &mut PtxParser<'a, 'input>| {
            let input_start = stream.input.first().map(|(_, s)| s).cloned();
            let stream_start = stream.checkpoint();
            match parser.parse_next(stream) {
                Ok(value) => Ok(Some(value)),
                Err(_) => {
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
            }
        },
    )
}

fn pragma<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<()> {
    trace(
        "pragma",
        (
            Token::DotPragma,
            separated::<_, _, (), _, _, _, _>(1.., Token::String, Token::Comma),
            Token::Semicolon,
        )
            .void(),
    )
    .parse_next(stream)
}

fn method_parameter<'a, 'input: 'a>(
    state_space: StateSpace,
    kernel_decl_rules: bool,
) -> impl Parser<PtxParser<'a, 'input>, Variable<&'input str>, ContextError> {
    fn nvptx_kernel_declaration<'a, 'input>(
        stream: &mut PtxParser<'a, 'input>,
    ) -> PResult<((Option<u32>, Option<NonZeroU8>, ScalarType), &'input str)> {
        trace(
            "nvptx_kernel_declaration",
            (
                vector_prefix,
                scalar_type,
                opt((Token::DotPtr, opt(Token::DotGlobal))),
                opt(align.verify(|x| x.count_ones() == 1)),
                ident,
            ),
        )
        // The alignment after .ptr is that of the data pointed to, not of the parameter.
        .map(|(vector, type_, _, _, name)| ((None, vector, type_), name))
        .parse_next(stream)
    }
    trace(
        "method_parameter",
        move |stream: &mut PtxParser<'a, 'input>| {
            if kernel_decl_rules {}
            let ((align, vector, type_), name) =
                alt(((variable_info_non_texref, ident), nvptx_kernel_declaration))
                    .parse_next(stream)?;
            let array_dimensions = if state_space != StateSpace::Reg {
                opt(array_dimensions).parse_next(stream)?
            } else {
                None
            };
            // TODO: push this check into array_dimensions(...)
            // (an unsized .param array of a .func is its variadic parameter, see `function`)
            if let Some(ref dims) = array_dimensions {
                if dims[0] == 0 && (kernel_decl_rules || state_space != StateSpace::Param) {
                    return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
                }
            }
            Ok(Variable {
                info: VariableInfo {
                    align,
                    v_type: Type::maybe_array(vector, type_, array_dimensions),
                    state_space,
                    array_init: Vec::new(),
                },
                name,
            })
        },
    )
}

enum ParsedType {
    Texref,
    Type {
        align: Option<u32>,
        vector: Option<NonZeroU8>,
        scalar: ScalarType,
    },
}

fn texref<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<ParsedType> {
    trace(
        "texref",
        alt((
            Token::DotTexref.void(),
            Token::DotSurfref.void(),
            |stream: &mut PtxParser<'a, 'input>| token_text(stream, ".samplerref"),
        ))
        .map(|_| ParsedType::Texref),
    )
    .parse_next(stream)
}

fn parse_if<Input, Output, Error, ParseNext>(
    cond: bool,
    mut parser: ParseNext,
) -> impl Parser<Input, Output, Error>
where
    Input: Stream,
    ParseNext: Parser<Input, Output, Error>,
    Error: ParserError<Input>,
{
    trace("cond_fail", move |input: &mut Input| {
        if cond {
            parser.parse_next(input)
        } else {
            fail(input)
        }
    })
}

fn variable_info<'a, 'input: 'a>(
    state_space: StateSpace,
) -> impl Parser<PtxParser<'a, 'input>, ParsedType, ContextError> {
    trace(
        "variable_info",
        move |stream: &mut PtxParser<'a, 'input>| {
            alt((
                parse_if(state_space == StateSpace::Global, texref),
                variable_info_non_texref.map(|(align, vector, scalar)| ParsedType::Type {
                    align,
                    vector,
                    scalar,
                }),
            ))
            .parse_next(stream)
        },
    )
}

// TODO: split to a separate type
fn variable_info_non_texref<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<(Option<u32>, Option<NonZeroU8>, ScalarType)> {
    trace(
        "variable_info_non_texref",
        (
            opt(align.verify(|x| x.count_ones() == 1)),
            vector_prefix,
            scalar_type,
        ),
    )
    .parse_next(stream)
}

fn multi_variable<'a, 'input: 'a>(
    extern_: bool,
    state_space: StateSpace,
) -> impl Parser<PtxParser<'a, 'input>, MultiVariable<&'input str>, ContextError> {
    trace(
        "multi_variable",
        move |stream: &mut PtxParser<'a, 'input>| {
            let (parsed_type, entries): (_, Vec<(_, Option<u32>)>) = (
                variable_info(state_space),
                separated(
                    1..,
                    (
                        ident,
                        // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#parameterized-variable-names
                        opt(delimited(Token::Lt, u32.verify(|x| *x != 0), Token::Gt)),
                    ),
                    Token::Comma,
                ),
            )
                .parse_next(stream)?;
            let count = match &entries[..] {
                [(_, count)] => *count,
                _ if entries.iter().all(|(_, count)| count.is_none()) => None,
                _ => {
                    // ptxas accepts parameterized names in a list of
                    // register names.
                    let ParsedType::Type {
                        align,
                        vector,
                        scalar,
                    } = parsed_type
                    else {
                        return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
                    };
                    return Ok(MultiVariable::List {
                        info: VariableInfo {
                            align,
                            v_type: Type::maybe_vector_parsed(vector, scalar),
                            state_space,
                            array_init: zero_init(state_space, scalar, vector),
                        },
                        names: entries,
                    });
                }
            };
            let names = entries.into_iter().map(|(name, _)| name).collect::<Vec<_>>();
            let (align, vector, type_) = match parsed_type {
                ParsedType::Texref => {
                    if count.is_some() {
                        return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
                    }
                    return Ok(MultiVariable::Names {
                        info: VariableInfo {
                            align: None,
                            v_type: Type::Texref,
                            state_space,
                            array_init: Vec::new(),
                        },
                        names,
                    });
                }
                ParsedType::Type {
                    align,
                    vector,
                    scalar,
                } => (align, vector, scalar),
            };
            if let Some(count) = count {
                if names.len() > 1 {
                    // nvcc does not support parameterized variable names in comma-separated lists of names.
                    return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
                }
                let name = names[0];
                return Ok(MultiVariable::Parameterized {
                    info: VariableInfo {
                        align,
                        v_type: Type::maybe_vector_parsed(vector, type_),
                        state_space,
                        array_init: zero_init(state_space, type_, vector),
                    },
                    name,
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
                    Some(ref mut dimensions) => array_initializer(stream, type_, vector, dimensions)?,
                    None => value_initializer(type_, vector).parse_next(stream)?,
                },
                _ => Vec::new(),
            };
            if let Some(ref dims) = array_dimensions {
                if !extern_ && dims[0] == 0 {
                    return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
                }
            }
            Ok(MultiVariable::Names {
                info: VariableInfo {
                    align,
                    v_type: Type::maybe_array(vector, type_, array_dimensions),
                    state_space,
                    array_init: initializer,
                },
                names,
            })
        },
    )
}

// The elements of an array (of vectors) in row-major order, missing ones (and missing sub-arrays) zero. An unsized
// first dimension (`a[] = {...}`) takes its size from the initializer (PTX ISA 5.4.4).
fn array_initializer<'a, 'input: 'a>(
    stream: &mut PtxParser<'a, 'input>,
    type_: ScalarType,
    vector: Option<NonZeroU8>,
    array_dimensions: &mut Vec<u32>,
) -> PResult<Vec<RegOrImmediate<&'input str>>> {
    // a vector is one more (innermost) dimension of the initializer
    let mut dimensions = array_dimensions.clone();
    if let Some(v) = vector {
        dimensions.push(v.get() as u32);
    }
    let default = || ast::RegOrImmediate::Imm(default_immediate_from_type(type_));
    let mut result = Vec::new();
    if opt(Token::Eq).parse_next(stream)?.is_some() {
        if dimensions[1..].iter().any(|d| *d == 0) {
            return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
        }
        let rows = nested_initializer(stream, type_, &dimensions, &mut result)?;
        if dimensions[0] == 0 {
            if array_dimensions.len() > 1 || vector.is_some() {
                // `{ {..}, {..} }` or a flat list: whole rows
                let inner: usize = dimensions[1..].iter().map(|d| *d as usize).product();
                dimensions[0] = match rows {
                    Some(rows) => rows,
                    None => result.len().div_ceil(inner) as u32,
                };
            } else {
                dimensions[0] = result.len() as u32;
            }
            if dimensions[0] == 0 {
                return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
            }
            array_dimensions[0] = dimensions[0];
        }
    } else if dimensions[0] == 0 {
        // an unsized array without initializer: extern only
        return Ok(Vec::new());
    }
    let total: usize = dimensions.iter().map(|d| *d as usize).product();
    if result.len() > total {
        return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
    }
    result.extend(iter::repeat_with(default).take(total - result.len()));
    Ok(result)
}

// One level of an array initializer: `{a, b, ...}` for the last dimension, `{{...}, {...}}` (or a flat list of
// all the elements) for the others
// A first dimension of 0 is unknown (any number of rows). Returns the number of rows when they were braced.
fn nested_initializer<'a, 'input: 'a>(
    stream: &mut PtxParser<'a, 'input>,
    type_: ScalarType,
    dimensions: &[u32],
    result: &mut Vec<RegOrImmediate<&'input str>>,
) -> PResult<Option<u32>> {
    let size: usize = if dimensions[0] == 0 {
        usize::MAX
    } else {
        dimensions.iter().map(|d| *d as usize).product()
    };
    let start = result.len();
    Token::LBrace.parse_next(stream)?;
    let nested = dimensions.len() > 1 && matches!(stream.input.first(), Some((Token::LBrace, _)));
    let mut rows = None;
    if nested {
        let inner: usize = dimensions[1..].iter().map(|d| *d as usize).product();
        let mut count = 0;
        loop {
            if dimensions[0] != 0 && count == dimensions[0] {
                return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
            }
            let at = result.len();
            nested_initializer(stream, type_, &dimensions[1..], result)?;
            let default = default_immediate_from_type(type_);
            result.extend(iter::repeat(ast::RegOrImmediate::Imm(default)).take(inner - (result.len() - at)));
            count += 1;
            if opt(Token::Comma).parse_next(stream)?.is_none() {
                break;
            }
        }
        rows = Some(count);
    } else {
        separated::<_, (), (), _, _, _, _>(0.., single_value_append(result), Token::Comma).parse_next(stream)?;
    }
    Token::RBrace.parse_next(stream)?;
    if result.len() - start > size {
        return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
    }
    Ok(rows)
}

fn default_immediate_from_type(type_: ScalarType) -> ast::ImmediateValue {
    match type_.kind() {
        ScalarKind::Bit | ScalarKind::Unsigned | ScalarKind::Pred => ast::ImmediateValue::U64(0),
        ScalarKind::Signed => ast::ImmediateValue::S64(0),
        ScalarKind::Float => ast::ImmediateValue::F64(0.0),
    }
}

// Uninitialized .global / .const variables are zero, as `value_initializer` makes them when there is no `=`.
fn zero_init<'input>(
    state_space: StateSpace,
    type_: ScalarType,
    vector: Option<NonZeroU8>,
) -> Vec<RegOrImmediate<&'input str>> {
    match state_space {
        StateSpace::Global | StateSpace::Const => {
            let n = vector.map_or(1, |v| v.get() as usize);
            iter::repeat_with(|| ast::RegOrImmediate::Imm(default_immediate_from_type(type_)))
                .take(n)
                .collect()
        }
        _ => Vec::new(),
    }
}

fn value_initializer<'a, 'input: 'a>(
    type_: ScalarType,
    vector: Option<NonZeroU8>,
) -> impl Parser<PtxParser<'a, 'input>, Vec<RegOrImmediate<&'input str>>, ContextError> {
    trace(
        "value_initializer",
        opt(move |stream: &mut PtxParser<'a, 'input>| {
            Token::Eq.parse_next(stream)?;
            let mut result = Vec::new();
            match vector {
                // a vector: `{x, y, ...}`, missing elements zero
                Some(v) => {
                    nested_initializer(stream, type_, &[v.get() as u32], &mut result)?;
                }
                None => single_value_append(&mut result).parse_next(stream)?,
            }
            Ok(result)
        })
        .map(move |maybe_vec| {
            let n = vector.map_or(1, |v| v.get() as usize);
            let mut result = maybe_vec.unwrap_or_default();
            result.extend(
                iter::repeat_with(|| ast::RegOrImmediate::Imm(default_immediate_from_type(type_)))
                    .take(n - result.len().min(n)),
            );
            result
        }),
    )
}

fn single_value_append<'b, 'a: 'b, 'input: 'a>(
    accumulator: &'b mut Vec<RegOrImmediate<&'input str>>,
) -> impl Parser<PtxParser<'a, 'input>, (), ContextError> + 'b {
    trace(
        "single_value_append",
        move |stream: &mut PtxParser<'a, 'input>| {
            let value = initializer_value.parse_next(stream)?;
            accumulator.push(value);
            Ok(())
        },
    )
}

// A size: an integer constant expression (PTX ISA 4.5.4).
fn const_u32<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<u32> {
    const_expr.verify_map(|v| u32::try_from(v).ok()).parse_next(stream)
}

fn array_dimensions<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<Vec<u32>> {
    let dimension = delimited(
        Token::LBracket,
        opt(const_u32).verify(|dim| *dim != Some(0)),
        Token::RBracket,
    )
    .parse_next(stream)?;
    let result = vec![dimension.unwrap_or(0)];
    repeat_fold_0_or_more(
        delimited(
            Token::LBracket,
            const_u32.verify(|dim| *dim != 0),
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
    // the storage of the narrow and tf32 types in declarations
    if let Some((_, span)) = stream.input.first() {
        let storage = match &stream.state.text[span.clone()] {
            ".tf32" => Some(ScalarType::B32),
            ".e2m1x2" => Some(ScalarType::B8),
            ".e4m3x2" | ".e5m2x2" | ".e2m3x2" | ".e3m2x2" | ".ue8m0x2" => Some(ScalarType::B16),
            _ => None,
        };
        if let Some(storage) = storage {
            stream.input = &stream.input[1..];
            return Ok(storage);
        }
    }
    token::any
        .verify_map(|(t, _)| {
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
    let start = stream.input.first().map(|(_, span)| span.start);
    let errors_before = stream.state.errors.len();
    let statement = trace(
        "predicated_instruction",
        (opt(pred_at), alt((tensor_instruction, parse_instruction)), Token::Semicolon)
            .map(|(p, i, _)| ast::Statement::Instruction(p, i)),
    )
    .parse_next(stream)?;
    // name the statement in the "not yet implemented" errors its rule raised, so that they can be traced back to it
    if let Some(start) = start {
        let end = stream.input.first().map(|(_, span)| span.start).unwrap_or(stream.state.text.len());
        let text = stream.state.text[start..end].trim();
        for e in stream.state.errors[errors_before..].iter_mut() {
            if let PtxError::Todo(message) = e {
                if !message.ends_with(';') {
                    *message = format!("{message}: {text}");
                }
            }
        }
    }
    Ok(statement)
}

fn pred_at<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<ast::PredAt<&'input str>> {
    trace(
        "pred_at",
        (Token::At, opt(Token::Exclamation), ident).map(|(_, not, label)| ast::PredAt {
            not: not.is_some(),
            label,
        }),
    )
    .parse_next(stream)
}

// label: .callprototype {(ret)} _ (params); label: .calltargets f, ...;
// label: .branchtargets l, ...;
fn call_target_directive<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<()> {
    fn prototype_parameter<'a, 'input>(
        stream: &mut PtxParser<'a, 'input>,
    ) -> PResult<(ast::Type, StateSpace)> {
        let space = dispatch! { token::any;
            (Token::DotParam, _) => empty.value(StateSpace::Param),
            (Token::DotReg, _) => empty.value(StateSpace::Reg),
            _ => fail
        }
        .parse_next(stream)?;
        let (_, vector, type_) = variable_info_non_texref.parse_next(stream)?;
        alt((Token::Sink.void(), ident.void())).parse_next(stream)?;
        let dimensions = if space != StateSpace::Reg {
            opt(array_dimensions).parse_next(stream)?
        } else {
            None
        };
        Ok((Type::maybe_array(vector, type_, dimensions), space))
    }
    fn prototype_parameters<'a, 'input>(
        stream: &mut PtxParser<'a, 'input>,
    ) -> PResult<Vec<(ast::Type, StateSpace)>> {
        delimited(
            Token::LParen,
            separated(0.., prototype_parameter, Token::Comma),
            Token::RParen,
        )
        .parse_next(stream)
    }
    let (name, _, directive) = (ident, Token::Colon, dot_ident).parse_next(stream)?;
    match directive {
        ".callprototype" => {
            let (returns, _, inputs, _) = (
                opt(prototype_parameters),
                Token::Sink,
                prototype_parameters,
                Token::Semicolon,
            )
                .parse_next(stream)?;
            stream
                .state
                .call_prototypes
                .insert(name, (returns.unwrap_or_default(), inputs));
        }
        ".calltargets" | ".branchtargets" => {
            let (targets, _): (Vec<(&str, Option<u32>)>, _) = (
                separated(1.., (ident, opt(delimited(Token::Lt, u32, Token::Gt))), Token::Comma),
                Token::Semicolon,
            )
                .parse_next(stream)?;
            let targets = targets
                .into_iter()
                .flat_map(|(name, count)| label_range(name, count))
                .collect::<Vec<_>>();
            if directive == ".branchtargets" {
                stream.state.branch_targets.insert(name, targets);
            } else {
                match stream.state.function_declarations.get(targets[0]) {
                    Some(signature) => {
                        let signature = signature.clone();
                        stream.state.call_prototypes.insert(name, signature);
                    }
                    None => stream.state.errors.push(PtxError::UnknownFunction),
                }
            }
        }
        _ => return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify)),
    }
    Ok(())
}

// The labels of `name<count>` (name0 to name{count-1}) or `name`; the generated names are kept for the life of the
// program
fn label_range<'input>(name: &'input str, count: Option<u32>) -> Vec<&'input str> {
    match count {
        Some(count) => (0..count).map(|i| &*Box::leak(format!("{name}{i}").into_boxed_str())).collect(),
        None => vec![name],
    }
}

// The .branchtargets tables of the function body that starts at the stream, which brx.idx
// can name before they are declared
fn prescan_branch_targets<'a, 'input>(stream: &mut PtxParser<'a, 'input>) {
    let tokens = stream.input;
    let text = stream.state.text;
    let mut depth = 0usize;
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].0 {
            Token::LBrace => depth += 1,
            Token::RBrace => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    break;
                }
            }
            Token::Colon if i > 0 && i + 1 < tokens.len() && &text[tokens[i + 1].1.clone()] == ".branchtargets" => {
                let name = &text[tokens[i - 1].1.clone()];
                let mut targets = Vec::new();
                let mut j = i + 2;
                while j < tokens.len() && !matches!(tokens[j].0, Token::Semicolon) {
                    match tokens[j].0 {
                        Token::Comma => {}
                        // N<5>: N0 to N4
                        Token::Lt if j + 2 < tokens.len() && matches!(tokens[j + 2].0, Token::Gt) => {
                            let count = text[tokens[j + 1].1.clone()].parse::<u32>().ok();
                            if let (Some(name), Some(_)) = (targets.pop(), count) {
                                targets.extend(label_range(name, count));
                            }
                            j += 2;
                        }
                        _ => targets.push(&text[tokens[j].1.clone()]),
                    }
                    j += 1;
                }
                stream.state.branch_targets.insert(name, targets);
                i = j;
            }
            _ => {}
        }
        i += 1;
    }
}

// brx.idx{.uni} index, targets; becomes a branch to each target guarded
// by a comparison of the index (an index out of range falls through)
fn brx_idx<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Statement<ParsedOperandStr<'input>>> {
    let (_, idx, _, index, _, table, _) = (
        ident_literal("brx"),
        Token::DotIdx,
        opt(Token::DotUni),
        ParsedOperandStr::parse,
        Token::Comma,
        ident,
        Token::Semicolon,
    )
        .parse_next(stream)?;
    let _ = idx;
    let Some(targets) = stream.state.branch_targets.get(table).cloned() else {
        return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
    };
    const PREDICATE: &str = "%__zluda_brx_idx";
    let mut statements = vec![ast::Statement::Variable(ast::MultiVariable::Names {
        info: ast::VariableInfo {
            align: None,
            v_type: Type::Scalar(ScalarType::Pred),
            state_space: StateSpace::Reg,
            array_init: Vec::new(),
        },
        names: vec![PREDICATE],
    })];
    for (i, target) in targets.into_iter().enumerate() {
        statements.push(ast::Statement::Instruction(
            None,
            ast::Instruction::Setp {
                data: ast::SetpData {
                    type_: ScalarType::U32,
                    flush_to_zero: None,
                    cmp_op: ast::SetpCompareOp::Integer(ast::SetpCompareInt::Eq),
                },
                arguments: SetpArgs {
                    dst1: ParsedOperand::Reg(PREDICATE),
                    dst2: None,
                    src1: index.clone(),
                    src2: ParsedOperand::Imm(ImmediateValue::U64(i as u64)),
                },
            },
        ));
        statements.push(ast::Statement::Instruction(
            Some(ast::PredAt {
                not: false,
                label: PREDICATE,
            }),
            ast::Instruction::Bra {
                arguments: BraArgs { src: target },
            },
        ));
    }
    Ok(ast::Statement::Block(statements))
}

fn label<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Statement<ParsedOperandStr<'input>>> {
    terminated(ident, Token::Colon)
        .map(|l| ast::Statement::Label(l))
        .parse_next(stream)
}

// .loc file line column {prologue_end} {, function_name label{+offset}} {, inlined_at file line column}
fn debug_directive<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<ast::DebugLoc> {
    (
        Token::DotLoc,
        u32,
        u32,
        u32,
        opt(ident_literal("prologue_end")),
        repeat::<_, _, (), _, _>(
            0..,
            preceded(
                Token::Comma,
                alt((
                    (ident_literal("function_name"), ident, opt((Token::Plus, u32))).void(),
                    (ident_literal("inlined_at"), u32, u32, u32).void(),
                )),
            ),
        ),
    )
        .map(|(_, file, line, column, _, _)| ast::DebugLoc { file, line, column })
        .parse_next(stream)
}

fn block_statement<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Statement<ParsedOperandStr<'input>>> {
    trace(
        "block_statement",
        delimited(Token::LBrace, repeat_without_none(statement), Token::RBrace)
            .map(|s| ast::Statement::Block(s)),
    )
    .parse_next(stream)
}

fn repeat_without_none<Input: Stream, Output, Error: ParserError<Input>>(
    parser: impl Parser<Input, Option<Output>, Error>,
) -> impl Parser<Input, Vec<Output>, Error> {
    trace(
        "repeat_without_none",
        repeat(0.., parser).fold(Vec::new, |mut acc: Vec<_>, item| {
            if let Some(item) = item {
                acc.push(item);
            }
            acc
        }),
    )
}

fn repeat_without_none_and_count<Input: Stream, Output, Error: ParserError<Input>>(
    parser: impl Parser<Input, Option<Output>, Error>,
) -> impl Parser<Input, (Vec<Output>, usize), Error> {
    trace(
        "repeat_without_none_and_count",
        repeat(0.., parser).fold(
            || (Vec::new(), 0),
            |(mut accumulator, mut nones): (Vec<_>, usize), item| {
                if let Some(item) = item {
                    accumulator.push(item);
                } else {
                    nones += 1;
                }
                (accumulator, nones)
            },
        ),
    )
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
        token::any
            .verify(|(t, _)| matches!(t, Token::Ident(text) if *text == s))
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
        fn vector_index<'input>(inp: &'input str) -> Result<u8, PtxError<'input>> {
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
                preceded(Token::Plus, const_offset)
                    .map(move |offset| ast::ParsedOperand::RegOffset(main_ident, offset)),
                preceded(Token::Minus, const_offset)
                    .map(move |offset| ast::ParsedOperand::RegOffset(main_ident, offset.wrapping_neg())),
                delimited(Token::LBracket, array_index, Token::RBracket).map(move |(index, offset)| {
                    match index {
                        Some(index) => ast::ParsedOperand::RegIndex(main_ident, index, offset as i32),
                        None => ast::ParsedOperand::RegOffset(main_ident, offset as i32),
                    }
                }),
                vector_member.map(move |index| ast::ParsedOperand::VecMember(main_ident, index)),
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
        ) -> PResult<Vec<Option<ast::RegOrImmediate<&'input str>>>> {
            delimited(
                Token::LBrace,
                separated(
                    1..=256,
                    alt((Token::Sink.value(None), reg_or_immediate.map(Some))),
                    Token::Comma,
                ),
                Token::RBrace,
            )
            .parse_next(stream)
        }
        trace(
            "operand",
            alt((
                trace("ident_operands", ident_operands),
                immediate_value.map(ast::ParsedOperand::Imm),
                trace(
                    "vector_operand",
                    vector_operand.map(ast::ParsedOperand::VecPack),
                ),
                // a lone sink (`mbarrier.arrive.b64 _, [addr]`), see sink_to_none
                Token::Sink.value(ast::ParsedOperand::VecPack(vec![None])),
            )),
        )
        .parse_next(stream)
    }
}

#[derive(Debug, thiserror::Error, PartialEq, strum::AsRefStr)]
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
    #[error("Context error: {0}")]
    Parser(ContextError),
    #[error("Not yet implemented: {0}")]
    Todo(String),
    #[error("Syntax error: {0}")]
    SyntaxError(String),
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
    #[error("Unrecognized statement {0:?}")]
    UnrecognizedStatement(&'input str),
    #[error("Unrecognized directive {0:?}")]
    UnrecognizedDirective(&'input str),
    #[error("Unsupported special register {0:?}")]
    UnsupportedSreg(&'input str),
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
        token::any.verify(|(t, _)| t == self).parse_next(input)
    }
}

fn bra<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    preceded(
        opt(Token::DotUni),
        token::any.verify_map(|(t, _)| match t {
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
    let (uni, return_arguments, name, input_arguments) = trace(
        "call",
        (
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
                separated(0.., ParsedOperand::<&'input str>::parse, Token::Comma)
                    .map(|x: Vec<_>| x),
                Token::RParen.void(),
            )
                .map(|(_, _, arguments, _)| arguments)),
        ),
    )
    .parse_next(stream)?;
    // call (ret), %fp, (args), prototype-or-targets;
    let prototype = opt(preceded(Token::Comma, ident)).parse_next(stream)?;
    let uniform = uni.is_some();
    let (recorded_fn, is_indirect) = match (
        stream.state.function_declarations.get(name),
        prototype.and_then(|p| stream.state.call_prototypes.get(p)),
    ) {
        (_, Some(prototype)) => (prototype, true),
        (Some(decl), None) => (decl, false),
        (None, None) => {
            stream.state.errors.push(PtxError::UnknownFunction);
            return Ok(empty_call(uniform, name));
        }
    };
    // A call may leave out the return values (PTX ISA 9.7.13.4)
    let return_arguments = return_arguments
        .unwrap_or_else(|| (0..recorded_fn.0.len()).map(|_| discarded_register()).collect());
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
        // Indirect calls are not followed by instruction_mode_to_global_mode
        is_external: is_indirect,
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
            is_external: false,
        },
    }
}

// The modifiers of an instruction parsed by hand, without their dots, for
// example `["sync", "aligned", "m16n8k16", "shared::cta"]`. A modifier ends
// at the next dot or at the first token that does not follow it immediately.
fn matrix_modifiers<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> Vec<&'input str> {
    let text = stream.state.text;
    let mut modifiers = Vec::new();
    let mut current: Option<(usize, usize)> = None;
    while let Some((token, span)) = stream.input.first() {
        let piece = &text[span.clone()];
        match current {
            _ if piece.starts_with('.') && piece.len() > 1 => {
                if let Some((start, end)) = current {
                    modifiers.push(&text[start + 1..end]);
                }
                current = Some((span.start, span.end));
            }
            // pieces of a modifier (`::`, digits), not the end of an instruction without operands
            Some((start, end)) if span.start == end && !matches!(token, Token::Semicolon | Token::Comma) => {
                current = Some((start, span.end))
            }
            _ => break,
        }
        stream.input = &stream.input[1..];
    }
    if let Some((start, end)) = current {
        modifiers.push(&text[start + 1..end]);
    }
    modifiers
}

fn backtrack<T>() -> PResult<T> {
    Err(ErrMode::Backtrack(ContextError::new()))
}

// Removes `name` from the modifiers, returns whether it was there.
fn take_modifier(modifiers: &mut Vec<&str>, name: &str) -> bool {
    match modifiers.iter().position(|m| *m == name) {
        Some(i) => {
            modifiers.remove(i);
            true
        }
        None => false,
    }
}

fn take_state_space(modifiers: &mut Vec<&str>) -> Option<StateSpace> {
    for (name, space) in [
        ("global", StateSpace::Global),
        ("shared", StateSpace::Shared),
        ("shared::cta", StateSpace::Shared),
    ] {
        if take_modifier(modifiers, name) {
            return Some(space);
        }
    }
    None
}

fn take_matrix_number(modifiers: &mut Vec<&str>) -> Option<ast::MatrixNumber> {
    for (name, number) in [
        ("x1", ast::MatrixNumber::X1),
        ("x2", ast::MatrixNumber::X2),
        ("x4", ast::MatrixNumber::X4),
    ] {
        if take_modifier(modifiers, name) {
            return Some(number);
        }
    }
    None
}

// `mMnNkK` -> (M, N, K)
fn take_mma_shape(modifiers: &mut Vec<&str>) -> Option<(u32, u32, u32)> {
    let position = modifiers.iter().position(|m| {
        m.starts_with('m') && m.contains('n') && m.contains('k') && m.len() > 5
    })?;
    let shape = modifiers.remove(position);
    let (m, rest) = shape[1..].split_once('n')?;
    let (n, k) = rest.split_once('k')?;
    Some((m.parse().ok()?, n.parse().ok()?, k.parse().ok()?))
}

fn take_layout(modifiers: &mut Vec<&str>) -> Option<ast::MatrixLayout> {
    let position = modifiers.iter().position(|m| *m == "row" || *m == "col")?;
    Some(if modifiers.remove(position) == "row" {
        ast::MatrixLayout::Row
    } else {
        ast::MatrixLayout::Col
    })
}

/// A memory operand: `[address]`, or an array element `a[index]` written without the brackets (PTX ISA 6.4.2).
fn memory_operand<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ParsedOperandStr<'input>> {
    alt((
        delimited(Token::LBracket, ParsedOperandStr::parse, Token::RBracket),
        ParsedOperandStr::parse.verify(|op| {
            matches!(op, ast::ParsedOperand::RegIndex(..))
                || matches!(op, ast::ParsedOperand::RegOffset(..))
        }),
    ))
    .parse_next(stream)
}

fn bracketed_operand<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ParsedOperandStr<'input>> {
    delimited(Token::LBracket, ParsedOperandStr::parse, Token::RBracket).parse_next(stream)
}

// ldmatrix.sync.aligned.m8n8.num{.trans}{.ss}.b16                       r, [p];
// ldmatrix.sync.aligned.m16n16.num.trans{.ss}.{b8, b8x16.b6x16_p32, b8x16.b4x16_p64} r, [p];
// ldmatrix.sync.aligned.m8n16.num{.ss}.{b8x16.b6x16_p32, b8x16.b4x16_p64, s8.s4}  r, [p];
fn ldmatrix<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let data = matrix_move_details(stream, false)?;
    let (dst, _, src) =
        (ParsedOperandStr::parse, Token::Comma, bracketed_operand).parse_next(stream)?;
    Ok(ast::Instruction::LdMatrix {
        data,
        arguments: LdMatrixArgs { dst, src },
    })
}

// stmatrix.sync.aligned.m8n8.num{.trans}{.ss}.b16       [p], r;
// stmatrix.sync.aligned.m16n8.num.trans{.ss}.b8         [p], r;
fn stmatrix<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let data = matrix_move_details(stream, true)?;
    let (src1, _, src2) =
        (bracketed_operand, Token::Comma, ParsedOperandStr::parse).parse_next(stream)?;
    Ok(ast::Instruction::StMatrix {
        data,
        arguments: StMatrixArgs { src1, src2 },
    })
}

fn matrix_move_details<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    store: bool,
) -> PResult<ast::LdMatrixDetails> {
    let mut modifiers = matrix_modifiers(stream);
    if !take_modifier(&mut modifiers, "sync") || !take_modifier(&mut modifiers, "aligned") {
        return backtrack();
    }
    let shapes: &[_] = if store {
        &[("m8n8", ast::MatrixShape::M8n8), ("m16n8", ast::MatrixShape::M16n8)]
    } else {
        &[
            ("m8n8", ast::MatrixShape::M8n8),
            ("m16n16", ast::MatrixShape::M16n16),
            ("m8n16", ast::MatrixShape::M8n16),
        ]
    };
    let Some(shape) = shapes
        .iter()
        .find_map(|(name, shape)| take_modifier(&mut modifiers, name).then_some(*shape))
    else {
        return backtrack();
    };
    let Some(number) = take_matrix_number(&mut modifiers) else {
        return backtrack();
    };
    let transpose = take_modifier(&mut modifiers, "trans");
    let state_space = take_state_space(&mut modifiers).unwrap_or(StateSpace::Shared);
    if state_space != StateSpace::Shared {
        return backtrack();
    }
    let format = match &modifiers[..] {
        ["b16"] => ast::LdMatrixFormat::B16,
        ["b8"] => ast::LdMatrixFormat::B8,
        ["b8x16", "b6x16_p32"] => ast::LdMatrixFormat::B6x16P32,
        ["b8x16", "b4x16_p64"] => ast::LdMatrixFormat::B4x16P64,
        ["s8", "s4"] => ast::LdMatrixFormat::S4,
        _ => return backtrack(),
    };
    use ast::LdMatrixFormat as F;
    let valid = match shape {
        ast::MatrixShape::M8n8 => format == F::B16,
        ast::MatrixShape::M16n8 => format == F::B8 && transpose,
        ast::MatrixShape::M16n16 => {
            transpose
                && number != ast::MatrixNumber::X4
                && matches!(format, F::B8 | F::B6x16P32 | F::B4x16P64)
        }
        ast::MatrixShape::M8n16 => {
            !transpose && matches!(format, F::B6x16P32 | F::B4x16P64 | F::S4)
        }
    };
    if !valid {
        return backtrack();
    }
    Ok(ast::LdMatrixDetails {
        shape,
        number,
        transpose,
        state_space,
        format,
    })
}

// movmatrix.sync.aligned.m8n8.trans.b16 d, a;
fn movmatrix<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let modifiers = matrix_modifiers(stream);
    if modifiers != ["sync", "aligned", "m8n8", "trans", "b16"] {
        return backtrack();
    }
    let (dst, _, src) =
        (ParsedOperandStr::parse, Token::Comma, ParsedOperandStr::parse).parse_next(stream)?;
    Ok(ast::Instruction::MovMatrix {
        arguments: MovMatrixArgs { dst, src },
    })
}

// The element types at the end of an mma or wmma.mma: dtype.atype.btype.ctype
// or (f16 wmma.mma) dtype.ctype.
fn take_mma_types(modifiers: &mut Vec<&str>) -> Option<[ast::MmaType; 4]> {
    let types = modifiers
        .iter()
        .filter_map(|m| ast::MmaType::from_name(m))
        .collect::<Vec<_>>();
    modifiers.retain(|m| ast::MmaType::from_name(m).is_none());
    match types[..] {
        [d, a, b, c] => Some([d, a, b, c]),
        [d, c] => Some([d, ast::MmaType::F16, ast::MmaType::F16, c]),
        _ => None,
    }
}

fn take_rounding(modifiers: &mut Vec<&str>) -> Option<ast::RoundingMode> {
    for (name, mode) in [
        ("rn", ast::RoundingMode::NearestEven),
        ("rz", ast::RoundingMode::Zero),
        ("rm", ast::RoundingMode::NegativeInf),
        ("rp", ast::RoundingMode::PositiveInf),
    ] {
        if take_modifier(modifiers, name) {
            return Some(mode);
        }
    }
    None
}

fn take_b1_op(modifiers: &mut Vec<&str>) -> Option<ast::MmaB1Op> {
    if !take_modifier(modifiers, "popc") {
        return None;
    }
    if take_modifier(modifiers, "xor") {
        Some(ast::MmaB1Op::Xor)
    } else if take_modifier(modifiers, "and") {
        Some(ast::MmaB1Op::And)
    } else {
        None
    }
}

// wgmma.fence / commit_group / wait_group N, and
// The tensor-map instructions, whose `[tensorMap, {coords}]` operand the
// instruction grammar cannot express:
// cp.async.bulk.tensor.dim.dst.src{.tile}.mbarrier::complete_tx::bytes{.multicast::cluster{::32b}}{.cta_group::2}{.L2::cache_hint}
//     [dstMem], [tensorMap, {coords}], [mbar]{, ctaMask}{, cache-policy}
// cp.async.bulk.tensor.dim.global.shared::cta{.tile}.bulk_group{.L2::cache_hint} [tensorMap, {coords}], [srcMem]{, cache-policy}
// cp.reduce.async.bulk.tensor.dim.global.shared::cta.redOp{.tile}.bulk_group{.L2::cache_hint} [tensorMap, {coords}], [srcMem]{, cache-policy}
// tensormap.replace.tile.field{.space}.b1024.type [tensorMap], {ord,} value
// fence.proxy.tensormap::generic.release.scope;
// fence.proxy.tensormap::generic.acquire.scope [addr], size;
// Tensor maps are read with ordinary loads (the generic proxy): a fence of the scope (a cluster is one CTA).
// Also the PTX ISA 9.4 .alias and .async::generic forms with a semantics.
fn fence_tensormap<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    modifiers: Vec<&'input str>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let (acquire, scope) = match modifiers[..] {
        ["proxy", "tensormap::generic", sem @ ("release" | "acquire"), scope] => (sem == "acquire", scope),
        // fence.proxy.alias{.acquire, .release}.scope and
        // fence.proxy.async::generic{.acquire, .release}.sync_restrict::...scope (PTX ISA 9.4): the
        // proxies are all the generic one on AGX
        ["proxy", "alias", "release" | "acquire", scope] => (false, scope),
        ["proxy", "async::generic", "release" | "acquire", restrict, scope] if restrict.starts_with("sync_restrict::") => {
            (false, scope)
        }
        _ => return backtrack(),
    };
    let scope = match scope {
        "cta" => MemScope::Cta,
        "cluster" | "gpu" => MemScope::Gpu,
        "sys" => MemScope::Sys,
        _ => return backtrack(),
    };
    if acquire {
        let _ = (ParsedOperandStr::parse, Token::Comma, ParsedOperandStr::parse).parse_next(stream)?;
    }
    Ok(ast::Instruction::Membar { data: scope })
}

// createpolicy.fractional.L2::primary.L2::secondary.b64 policy{, fraction}: the form with a secondary priority
// (a cache hint, like the primary one), which the instruction grammar cannot tell from the primary.
fn createpolicy_secondary<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    modifiers: Vec<&'input str>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let priority = |m: &str| match m {
        "L2::evict_last" => Some(EvictionPriority::L2EvictLast),
        "L2::evict_normal" => Some(EvictionPriority::L2EvictNormal),
        "L2::evict_first" => Some(EvictionPriority::L2EvictFirst),
        "L2::evict_unchanged" => Some(EvictionPriority::L2EvictUnchanged),
        _ => None,
    };
    let primary = match modifiers[..] {
        ["fractional", primary, "L2::evict_first" | "L2::evict_unchanged", "b64"] => priority(primary),
        _ => None,
    };
    let Some(primary_priority) = primary else {
        return backtrack();
    };
    let dst_policy = ParsedOperandStr::parse.parse_next(stream)?;
    let fraction = opt((Token::Comma, ParsedOperandStr::parse).map(|(_, f)| f)).parse_next(stream)?;
    let fraction = fraction
        .and_then(|op| op.as_immediate())
        .and_then(|imm| imm.as_f64())
        .unwrap_or(1.0) as f32;
    Ok(ast::Instruction::CreatePolicyFractional {
        data: ast::CreatePolicyFractionalDetails { primary_priority, fraction },
        arguments: ast::CreatePolicyFractionalArgs { dst_policy },
    })
}

// mbarrier.test_wait{.parity}.phase_type::{primary, conditional}{.sem}{.scope}{.space}.b64
//     waitComplete{|reportPredicate}{, reportValue}, [addr], state | parity;
// mbarrier.try_wait (the same, then {, suspendTimeHint}): the PTX ISA 9.3 / 9.4 forms with a phase type.
fn mbarrier_phase_type<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    modifiers: Vec<&'input str>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let mut modifiers = modifiers;
    let try_wait = match modifiers.first() {
        Some(&"test_wait") => false,
        Some(&"try_wait") => true,
        _ => return backtrack(),
    };
    modifiers.remove(0);
    let conditional = take_modifier(&mut modifiers, "phase_type::conditional");
    if !conditional && !take_modifier(&mut modifiers, "phase_type::primary") {
        return backtrack();
    }
    let parity = take_modifier(&mut modifiers, "parity");
    for sem in ["acquire", "relaxed", "cta", "cluster"] {
        take_modifier(&mut modifiers, sem);
    }
    let mut shared = false;
    for space in ["shared::cluster", "shared::cta", "shared"] {
        shared |= take_modifier(&mut modifiers, space);
    }
    let space = shared.then_some(StateSpace::Shared);
    if !take_modifier(&mut modifiers, "b64") || !modifiers.is_empty() || (conditional && !parity) {
        return backtrack();
    }
    let dst = ParsedOperandStr::parse.parse_next(stream)?;
    let dst_report_pred = opt((Token::Pipe, ParsedOperandStr::parse).map(|(_, p)| p)).parse_next(stream)?;
    Token::Comma.parse_next(stream)?;
    // reportValue, then [addr]
    let dst_report_value = opt((ParsedOperandStr::parse, Token::Comma).map(|(v, _)| v)).parse_next(stream)?;
    let src_mbarrier = memory_operand(stream)?;
    Token::Comma.parse_next(stream)?;
    let src_state = ParsedOperandStr::parse.parse_next(stream)?;
    let src_time_hint = if try_wait {
        opt((Token::Comma, ParsedOperandStr::parse).map(|(_, t)| t)).parse_next(stream)?
    } else {
        None
    };
    Ok(ast::Instruction::MbarrierWait {
        data: ast::MbarrierWaitDetails { space: shared_space(space), parity, try_wait, conditional },
        arguments: ast::MbarrierWaitArgs {
            dst,
            dst_report_pred,
            dst_report_value,
            src_mbarrier,
            src_state,
            src_time_hint,
        },
    })
}

// atom / red float operations in the orders the instruction grammar does not take: .noftz on .f32
// (atom.global.add.noftz.f32, PTX ISA 9.4) and the vector prefix before the operation
// (atom.global.v4.f32.add{.noftz}).
fn atom_float_any_order<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    opcode: &str,
    modifiers: Vec<&'input str>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let mut modifiers = modifiers;
    let op_position = modifiers.iter().position(|m| matches!(*m, "add" | "min" | "max"));
    let vec_position = modifiers.iter().position(|m| matches!(*m, "v2" | "v4" | "v8"));
    let noftz = take_modifier(&mut modifiers, "noftz");
    let vector_first = matches!((op_position, vec_position), (Some(o), Some(v)) if v < o);
    if !noftz && !vector_first {
        return backtrack();
    }
    let semantics = [
        ("relaxed", AtomSemantics::Relaxed),
        ("acquire", AtomSemantics::Acquire),
        ("release", AtomSemantics::Release),
        ("acq_rel", AtomSemantics::AcqRel),
    ]
    .into_iter()
    .find(|(name, _)| take_modifier(&mut modifiers, name))
    .map_or(AtomSemantics::Relaxed, |(_, s)| s);
    let scope = [("cta", MemScope::Cta), ("cluster", MemScope::Gpu), ("gpu", MemScope::Gpu), ("sys", MemScope::Sys)]
        .into_iter()
        .find(|(name, _)| take_modifier(&mut modifiers, name))
        .map_or(MemScope::Gpu, |(_, s)| s);
    let space = if take_modifier(&mut modifiers, "global") { StateSpace::Global } else { StateSpace::Generic };
    let op = if take_modifier(&mut modifiers, "add") {
        ast::AtomicOp::FloatAdd
    } else if take_modifier(&mut modifiers, "min") {
        ast::AtomicOp::FloatMin
    } else if take_modifier(&mut modifiers, "max") {
        ast::AtomicOp::FloatMax
    } else {
        return backtrack();
    };
    let vector = [("v2", 2u8), ("v4", 4), ("v8", 8)]
        .into_iter()
        .find(|(name, _)| take_modifier(&mut modifiers, name))
        .map(|(_, n)| n);
    let scalar = [
        ("f32", ScalarType::F32),
        ("f16", ScalarType::F16),
        ("bf16", ScalarType::BF16),
        ("f16x2", ScalarType::F16x2),
        ("bf16x2", ScalarType::BF16x2),
    ]
    .into_iter()
    .find(|(name, _)| take_modifier(&mut modifiers, name))
    .map(|(_, t)| t);
    let Some(scalar) = scalar else {
        return backtrack();
    };
    let cache_hint = take_modifier(&mut modifiers, "L2::cache_hint");
    if !modifiers.is_empty() || (vector == Some(8) && scalar == ScalarType::F32) {
        return backtrack();
    }
    let type_ = match vector {
        Some(n) => ast::Type::Vector(n, scalar),
        None => ast::Type::Scalar(scalar),
    };
    let data = AtomDetails { noftz, semantics, scope, space, op, type_ };
    let dst = if opcode == "atom" {
        let d = ParsedOperandStr::parse.parse_next(stream)?;
        Token::Comma.parse_next(stream)?;
        Some(d)
    } else {
        None
    };
    let src1 = memory_operand(stream)?;
    Token::Comma.parse_next(stream)?;
    let src2 = ParsedOperandStr::parse.parse_next(stream)?;
    // the cache policy is a hint
    if cache_hint {
        (Token::Comma, ParsedOperandStr::parse).parse_next(stream)?;
    }
    Ok(match dst {
        Some(dst) => ast::Instruction::Atom { data, arguments: AtomArgs { dst, src1, src2 } },
        None => ast::Instruction::Red { data, arguments: RedArgs { src1, src2 } },
    })
}

// vote.{all,any,uni}.pred d, {!}a: the deprecated forms without .sync (below sm_70), over the active threads.
// vote.ballot.b32 and the .sync forms stay with the instruction grammar.
fn vote_without_sync<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    modifiers: Vec<&'input str>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let mode = match modifiers[..] {
        ["all", "pred"] => VoteMode::All,
        ["any", "pred"] => VoteMode::Any,
        ["uni", "pred"] => VoteMode::Uni,
        _ => return backtrack(),
    };
    let dst = ParsedOperandStr::parse.parse_next(stream)?;
    Token::Comma.parse_next(stream)?;
    let negate = opt(Token::Exclamation).parse_next(stream)?.is_some();
    let src = ParsedOperandStr::parse.parse_next(stream)?;
    Ok(ast::Instruction::Vote {
        data: ast::VoteDetails { mode, negate },
        arguments: ast::VoteArgs { dst, src1: src, src2: None },
    })
}

fn tensor_instruction<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let Some((_, span)) = stream.input.first() else {
        return backtrack();
    };
    let opcode = &stream.state.text[span.clone()];
    let texture = ["tex", "tld4", "txq", "suld", "sust", "sured", "suq", "istypep"].contains(&opcode);
    let arith = ["add", "sub", "mul", "fma"].contains(&opcode);
    let sparse = opcode == "spcompress" || opcode == "spdecompress";
    let tcgen05 = opcode == "tcgen05";
    if opcode != "cp" && opcode != "tensormap" && !texture && !arith && opcode != "cvt" && !sparse && !tcgen05
        && opcode != "vote" && opcode != "fence" && opcode != "createpolicy" && opcode != "mbarrier"
        && opcode != "atom" && opcode != "red" && opcode != "multimem" && opcode != "fabric" && opcode != "ld"
    {
        return backtrack();
    }
    stream.input = &stream.input[1..];
    let mut modifiers = matrix_modifiers(stream);
    if opcode == "vote" {
        return vote_without_sync(stream, modifiers);
    }
    if opcode == "fence" {
        return fence_tensormap(stream, modifiers);
    }
    if opcode == "createpolicy" {
        return createpolicy_secondary(stream, modifiers);
    }
    if opcode == "mbarrier" {
        return mbarrier_phase_type(stream, modifiers);
    }
    if opcode == "atom" || opcode == "red" {
        return atom_float_any_order(stream, opcode, modifiers);
    }
    if opcode == "multimem" {
        return multimem_instruction(stream, modifiers);
    }
    if opcode == "ld" {
        return ld_volatile_relaxed(stream, modifiers);
    }
    if opcode == "fabric" {
        return fabric_instruction(stream, modifiers);
    }
    if opcode == "cvt" {
        return cvt_narrow_instruction(stream, modifiers);
    }
    if sparse {
        return sparse_instruction(stream, opcode, modifiers);
    }
    if tcgen05 {
        return tcgen05_instruction(stream, modifiers);
    }
    if arith {
        return mixed_float_instruction(stream, opcode, modifiers);
    }
    if opcode == "tensormap" {
        return tensormap_replace(stream, modifiers);
    }
    if texture {
        return texture_instruction(stream, opcode, modifiers);
    }
    let reduce = take_modifier(&mut modifiers, "reduce");
    if !take_modifier(&mut modifiers, "async")
        || !take_modifier(&mut modifiers, "bulk")
        || !take_modifier(&mut modifiers, "tensor")
    {
        return backtrack();
    }
    let dims = match modifiers.first().and_then(|m| m.strip_suffix('d')).and_then(|d| d.parse::<u8>().ok()) {
        Some(d @ 1..=5) => d,
        _ => return backtrack(),
    };
    modifiers.remove(0);
    // the destination space comes first
    let load = match modifiers.first() {
        Some(&"shared::cluster") | Some(&"shared::cta") => true,
        Some(&"global") => false,
        _ => return backtrack(),
    };
    // shared::cluster addresses are shared addresses of the cluster's threadgroup
    if !take_modifier(&mut modifiers, "shared::cluster") && !take_modifier(&mut modifiers, "shared::cta") {
        return backtrack();
    }
    let smem_space = StateSpace::Shared;
    if !take_modifier(&mut modifiers, "global") {
        return backtrack();
    }
    take_modifier(&mut modifiers, "tile");
    let im2col = [("im2col", 0u8), ("im2col_no_offs", 0), ("im2col::w", 1), ("im2col::w::128", 2), ("im2col_no_offs::w", 3)]
        .iter()
        .find(|(name, _)| take_modifier(&mut modifiers, name))
        .map(|&(_, mode)| mode);
    // the im2col offsets are an operand of loads only
    let no_offs = !load || im2col.is_none();
    let cache_hint = take_modifier(&mut modifiers, "L2::cache_hint");
    let multicast32 = take_modifier(&mut modifiers, "multicast::cluster::32b");
    let multicast = multicast32 || take_modifier(&mut modifiers, "multicast::cluster");
    let ov_addr = take_modifier(&mut modifiers, "override::global_address");
    let ov_dim_stride = take_modifier(&mut modifiers, "override::global_dim_stride");
    let ov_dim = ov_dim_stride || take_modifier(&mut modifiers, "override::global_dim");
    let cta_group2 = take_modifier(&mut modifiers, "cta_group::2");
    take_modifier(&mut modifiers, "cta_group::1");
    let report = [
        "per_element::ff",
        "per_16bytes::80000000",
        "per_16bytes::8000",
        "per_16bytes::80",
        "per_16bytes::8",
    ]
    .iter()
    .position(|r| {
        let name = format!("mbarrier::report::validity::{r}");
        match modifiers.iter().position(|m| *m == name) {
            Some(i) => {
                modifiers.remove(i);
                true
            }
            None => false,
        }
    })
    .map_or(0, |i| i as u32 + 1);
    let op = if reduce {
        let op = match modifiers.first() {
            Some(&"add") => ast::AtomicOp::Add,
            Some(&"min") => ast::AtomicOp::SignedMin,
            Some(&"max") => ast::AtomicOp::SignedMax,
            Some(&"inc") => ast::AtomicOp::IncrementWrap,
            Some(&"dec") => ast::AtomicOp::DecrementWrap,
            Some(&"and") => ast::AtomicOp::And,
            Some(&"or") => ast::AtomicOp::Or,
            Some(&"xor") => ast::AtomicOp::Xor,
            _ => return backtrack(),
        };
        modifiers.remove(0);
        Some(op)
    } else {
        None
    };
    let completion = if load { "mbarrier::complete_tx::bytes" } else { "bulk_group" };
    if !take_modifier(&mut modifiers, completion) || !modifiers.is_empty() || (reduce && load) || (multicast && !load) {
        return backtrack();
    }
    let address = |stream: &mut PtxParser<'a, 'input>| -> PResult<ParsedOperandStr<'input>> {
        delimited(Token::LBracket, ParsedOperandStr::parse, Token::RBracket).parse_next(stream)
    };
    fn elements<'input>(operand: ParsedOperandStr<'input>) -> PResult<Vec<ParsedOperandStr<'input>>> {
        let ast::ParsedOperand::VecPack(values) = operand else {
            return backtrack();
        };
        values
            .into_iter()
            .map(|c| match c {
                Some(ast::RegOrImmediate::Reg(r)) => Ok(ast::ParsedOperand::Reg(r)),
                Some(ast::RegOrImmediate::Imm(i)) => Ok(ast::ParsedOperand::Imm(i)),
                _ => backtrack(),
            })
            .collect()
    }
    // [tensorMap{, newAddress{, {dims}{, {strides}, upperStrideBits}}}, {coords}]
    struct Tensor<'input> {
        tmap: ParsedOperandStr<'input>,
        address: Option<ParsedOperandStr<'input>>,
        dims: Vec<ParsedOperandStr<'input>>,
        strides: Vec<ParsedOperandStr<'input>>,
        upper: Option<ParsedOperandStr<'input>>,
        coords: Vec<ParsedOperandStr<'input>>,
    }
    let tensor = |stream: &mut PtxParser<'a, 'input>| -> PResult<Tensor<'input>> {
        let (tmap, rest) = delimited(
            Token::LBracket,
            (ParsedOperandStr::parse, repeat(1.., preceded(Token::Comma, ParsedOperandStr::parse))),
            Token::RBracket,
        )
        .parse_next(stream)?;
        let mut rest: Vec<ParsedOperandStr<'input>> = rest;
        let coords = elements(rest.pop().unwrap())?;
        let expected = (ov_addr || ov_dim) as usize + ov_dim as usize + 2 * ov_dim_stride as usize;
        if rest.len() != expected {
            return backtrack();
        }
        let mut rest = rest.into_iter();
        let address = if ov_addr || ov_dim { rest.next() } else { None };
        let dims = if ov_dim { elements(rest.next().unwrap())? } else { Vec::new() };
        let strides = if ov_dim_stride { elements(rest.next().unwrap())? } else { Vec::new() };
        let upper = rest.next();
        if dims.len() > 5 || strides.len() > 4 {
            return backtrack();
        }
        Ok(Tensor { tmap, address, dims, strides, upper, coords })
    };
    let (smem, tensor, mbarrier) = if load {
        let smem = address(stream)?;
        Token::Comma.parse_next(stream)?;
        let tensor = tensor(stream)?;
        Token::Comma.parse_next(stream)?;
        (smem, tensor, Some(address(stream)?))
    } else {
        let tensor = tensor(stream)?;
        Token::Comma.parse_next(stream)?;
        (address(stream)?, tensor, None)
    };
    let Tensor { tmap, address: ov_address, dims: ov_dims, strides: ov_strides, upper: ov_upper, coords } = tensor;
    if coords.len() != dims as usize || (im2col.is_some() && dims < 3) {
        return backtrack();
    }
    let mut info = Vec::new();
    if !no_offs {
        Token::Comma.parse_next(stream)?;
        let ast::ParsedOperand::VecPack(values) = ParsedOperandStr::parse.parse_next(stream)? else {
            return backtrack();
        };
        for v in values {
            info.push(match v {
                Some(ast::RegOrImmediate::Reg(r)) => ast::ParsedOperand::Reg(r),
                Some(ast::RegOrImmediate::Imm(i)) => ast::ParsedOperand::Imm(i),
                _ => return backtrack(),
            });
        }
        if info.len() > 3 {
            return backtrack();
        }
    }
    let mut info = info.into_iter();
    let mut ov_dims = ov_dims.into_iter();
    let mut ov_strides = ov_strides.into_iter();
    let src_cta_mask = if multicast {
        Token::Comma.parse_next(stream)?;
        Some(ParsedOperandStr::parse.parse_next(stream)?)
    } else {
        None
    };
    let src_cache_policy = if cache_hint {
        Token::Comma.parse_next(stream)?;
        Some(ParsedOperandStr::parse.parse_next(stream)?)
    } else {
        None
    };
    let mut coords = coords.into_iter();
    let src_c0 = coords.next().unwrap();
    let kind = match op {
        Some(op) => ast::TensorCopyKind::Reduce(op),
        None if load => ast::TensorCopyKind::Load,
        None => ast::TensorCopyKind::Store,
    };
    Ok(ast::Instruction::CpAsyncBulkTensor {
        data: ast::CpAsyncBulkTensorDetails {
            dims,
            kind,
            smem_space,
            multicast,
            mask32: multicast32,
            cta_group2,
            im2col,
            override_address: ov_addr || ov_dim,
            report,
        },
        arguments: ast::CpAsyncBulkTensorArgs {
            src_smem: smem,
            src_tmap: tmap,
            src_c0,
            src_c1: coords.next(),
            src_c2: coords.next(),
            src_c3: coords.next(),
            src_c4: coords.next(),
            src_mbarrier: mbarrier,
            src_ov_addr: ov_address,
            src_ov_d0: ov_dims.next(),
            src_ov_d1: ov_dims.next(),
            src_ov_d2: ov_dims.next(),
            src_ov_d3: ov_dims.next(),
            src_ov_d4: ov_dims.next(),
            src_ov_s0: ov_strides.next(),
            src_ov_s1: ov_strides.next(),
            src_ov_s2: ov_strides.next(),
            src_ov_s3: ov_strides.next(),
            src_ov_up: ov_upper,
            src_i0: info.next(),
            src_i1: info.next(),
            src_i2: info.next(),
            src_cta_mask,
            src_cache_policy,
        },
    })
}

// The elements of a vector operand `{a, b, ...}` (or a lone operand as one element).
fn operand_elements<'input>(operand: ParsedOperandStr<'input>) -> PResult<Vec<ParsedOperandStr<'input>>> {
    match operand {
        ast::ParsedOperand::VecPack(values) => values
            .into_iter()
            .map(|c| match c {
                Some(ast::RegOrImmediate::Reg(r)) => Ok(ast::ParsedOperand::Reg(r)),
                Some(ast::RegOrImmediate::Imm(i)) => Ok(ast::ParsedOperand::Imm(i)),
                _ => backtrack(),
            })
            .collect(),
        other => Ok(vec![other]),
    }
}

fn take_geometry(modifiers: &mut Vec<&str>) -> Option<ast::TexGeometry> {
    use ast::TexGeometry::*;
    [("1d", D1), ("2d", D2), ("3d", D3), ("a1d", A1d), ("a2d", A2d), ("cube", Cube), ("acube", ACube),
     ("2dms", Ms2d), ("a2dms", AMs2d)]
        .iter()
        .find(|(name, _)| take_modifier(modifiers, name))
        .map(|&(_, g)| g)
}

fn take_type(modifiers: &mut Vec<&str>, types: &[(&str, ScalarType)]) -> Option<ScalarType> {
    types.iter().find(|(name, _)| take_modifier(modifiers, name)).map(|&(_, t)| t)
}

// The narrow conversions of PTX ISA 9.1 / 9.4 that the instruction grammar does
// not have: .pzo, .scaled::n1::ue8m0 / .scaled::n2::ue8m0 and the .ue5m3x2 and
// .s2f6x2 formats (see ast::CvtNarrowDetails). Every other cvt stays with the grammar.
fn cvt_narrow_instruction<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    mut modifiers: Vec<&'input str>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    if take_modifier(&mut modifiers, "rs") {
        return cvt_rs(stream, modifiers);
    }
    if modifiers.contains(&"pzo") {
        let kind = modifiers.iter().find_map(|m| match *m {
            "f16" => Some(0u8),
            "bf16" => Some(1),
            "tf32" => Some(2),
            "f16x2" => Some(3),
            "bf16x2" => Some(4),
            _ => None,
        });
        if let Some(kind) = kind {
            let mut flags = 0u8;
            for (name, bit) in [("relu", 4u8), ("satfinite", 64), ("pzo", 0), ("rn", 0)] {
                if take_modifier(&mut modifiers, name) {
                    flags |= bit;
                }
            }
            if modifiers.len() != 2 || modifiers[1] != "f32" {
                return backtrack();
            }
            let dst = ParsedOperandStr::parse.parse_next(stream)?;
            Token::Comma.parse_next(stream)?;
            let src_a = ParsedOperandStr::parse.parse_next(stream)?;
            let src_b = if kind >= 3 {
                Token::Comma.parse_next(stream)?;
                Some(ParsedOperandStr::parse.parse_next(stream)?)
            } else {
                None
            };
            return Ok(ast::Instruction::CvtPzo {
                data: ast::CvtRsDetails { kind, flags },
                arguments: CvtPzoArgs { dst, src_a, src_b },
            });
        }
    }
    let kind_of = |m: &str| {
        Some(match m {
            "e4m3x2" => 0u8,
            "e5m2x2" => 1,
            "e2m3x2" => 2,
            "e3m2x2" => 3,
            "e2m1x2" => 4,
            "ue5m3x2" => 5,
            "s2f6x2" => 6,
            "ue8m0x2" => 7,
            _ => return None,
        })
    };
    let half_of = |m: &str| match m {
        "f16x2" => Some(0u8),
        "bf16x2" => Some(1u8),
        _ => None,
    };
    let has = |m: &str| modifiers.contains(&m);
    let types: Vec<&str> = modifiers.iter().copied().filter(|m| kind_of(m).is_some() || half_of(m).is_some() || *m == "f32").collect();
    let narrowing = matches!(types.as_slice(), [d, _] if kind_of(d).is_some());
    let widening = matches!(types.as_slice(), [d, s] if half_of(d).is_some() && kind_of(s).is_some());
    // the forms of PTX ISA 9.1 / 9.4 (the older ones are cvt.rn / .rz conversions of the mini formats)
    let new_form = modifiers
        .iter()
        .any(|m| matches!(*m, "pzo" | "scaled::n1::ue8m0" | "scaled::n2::ue8m0" | "ue5m3x2" | "s2f6x2" | "ue8m0x2"))
        || (narrowing && (has("rp") || has("rm")))
        || (widening && (has("relu") || has("satfinite")));
    // the bf16x2 -> ue8m0x2 widening keeps the older form
    if !new_form || (widening && has("ue8m0x2")) {
        return backtrack();
    }
    let rounding = if take_modifier(&mut modifiers, "rn") {
        0u8
    } else if take_modifier(&mut modifiers, "rz") {
        1
    } else if take_modifier(&mut modifiers, "rp") {
        2
    } else if take_modifier(&mut modifiers, "rm") {
        3
    } else {
        return backtrack();
    };
    let mut flags = rounding;
    for (name, bit) in [("relu", 4u8), ("pzo", 8), ("scaled::n1::ue8m0", 16), ("scaled::n2::ue8m0", 32)] {
        if take_modifier(&mut modifiers, name) {
            flags |= bit;
        }
    }
    let satfinite = take_modifier(&mut modifiers, "satfinite");
    let (kind, other, widen) = match modifiers.as_slice() {
        [d, "f32"] if kind_of(d).is_some() => (kind_of(d).unwrap(), 0u8, false),
        [d, s] if kind_of(d).is_some() && half_of(s).is_some() => (kind_of(d).unwrap(), half_of(s).unwrap() + 1, false),
        [d, s] if half_of(d).is_some() && kind_of(s).is_some() => (kind_of(s).unwrap(), half_of(d).unwrap(), true),
        _ => return backtrack(),
    };
    // narrowing needs .satfinite; widening rounds to nearest, with .relu, .satfinite and .scaled::n2::ue8m0
    if (!widen && !satfinite) || (widen && (rounding != 0 || flags & !(32 | 4) != 0)) {
        return backtrack();
    }
    if widen && satfinite {
        flags |= 64;
    }
    let dst = ParsedOperandStr::parse.parse_next(stream)?;
    Token::Comma.parse_next(stream)?;
    let src1 = ParsedOperandStr::parse.parse_next(stream)?;
    let src2 = if !widen && other == 0 {
        Token::Comma.parse_next(stream)?;
        Some(ParsedOperandStr::parse.parse_next(stream)?)
    } else {
        None
    };
    let scale = opt(preceded(Token::Comma, ParsedOperandStr::parse)).parse_next(stream)?;
    Ok(ast::Instruction::CvtNarrow {
        data: ast::CvtNarrowDetails { kind, flags, other, widen },
        arguments: CvtNarrowArgs { dst, src1, src2, src_scale: scale },
    })
}

// cvt.rs{.relu}{.satfinite}{.pzo}.{f16x2,bf16x2}.f32 d, a, b, rbits;
// cvt.rs{.relu}.satfinite{.pzo}.{e4m3x4,e5m2x4,e2m3x4,e3m2x4,e2m1x4}.f32 d, {a, b, e, f}, rbits
fn cvt_rs<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    mut modifiers: Vec<&'input str>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let mut flags = 0u8;
    for (name, bit) in [("relu", 4u8), ("pzo", 8), ("satfinite", 64)] {
        if take_modifier(&mut modifiers, name) {
            flags |= bit;
        }
    }
    let kind = match modifiers.as_slice() {
        ["e4m3x4", "f32"] => 0u8,
        ["e5m2x4", "f32"] => 1,
        ["e2m3x4", "f32"] => 2,
        ["e3m2x4", "f32"] => 3,
        ["e2m1x4", "f32"] => 4,
        ["f16x2", "f32"] => 8,
        ["bf16x2", "f32"] => 9,
        _ => return backtrack(),
    };
    if kind < 8 && flags & 64 == 0 {
        return backtrack();
    }
    let comma = |stream: &mut PtxParser<'a, 'input>| Token::Comma.parse_next(stream).map(|_| ());
    let dst = ParsedOperandStr::parse.parse_next(stream)?;
    comma(stream)?;
    let (src_a, src_b, src_e, src_f) = if kind < 8 {
        let ast::ParsedOperand::VecPack(values) = ParsedOperandStr::parse.parse_next(stream)? else {
            return backtrack();
        };
        let mut regs = Vec::new();
        for v in values {
            match v {
                Some(ast::RegOrImmediate::Reg(r)) => regs.push(ast::ParsedOperand::Reg(r)),
                Some(ast::RegOrImmediate::Imm(i)) => regs.push(ast::ParsedOperand::Imm(i)),
                _ => return backtrack(),
            }
        }
        let [a, b, e, f]: [ParsedOperandStr<'input>; 4] = regs.try_into().map_err(|_| ErrMode::from_error_kind(stream, ErrorKind::Verify))?;
        (a, b, Some(e), Some(f))
    } else {
        let a = ParsedOperandStr::parse.parse_next(stream)?;
        comma(stream)?;
        let b = ParsedOperandStr::parse.parse_next(stream)?;
        (a, b, None, None)
    };
    comma(stream)?;
    let src_rbits = ParsedOperandStr::parse.parse_next(stream)?;
    Ok(ast::Instruction::CvtRs {
        data: ast::CvtRsDetails { kind, flags },
        arguments: CvtRsArgs { dst, src_a, src_b, src_e, src_f, src_rbits },
    })
}

// tcgen05 (the fifth-generation tensor core of sm_100, emulated on AGX with the tensor memory in device memory)
fn tcgen05_instruction<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    mut modifiers: Vec<&'input str>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let Some(&op) = modifiers.first() else {
        return backtrack();
    };
    modifiers.remove(0);
    // `.cta_group::1.128x256b` lexes as one piece (`1.128` is a number)
    modifiers = modifiers
        .into_iter()
        .flat_map(|m| match m.strip_prefix("cta_group::").and_then(|rest| rest.split_once('.')) {
            Some(_) => {
                let (group, shape) = m.split_at("cta_group::".len() + 1);
                vec![group, &shape[1..]]
            }
            None => vec![m],
        })
        .collect();
    let exclusive = take_modifier(&mut modifiers, "exclusive");
    let cta_group = if take_modifier(&mut modifiers, "cta_group::2") {
        2u8
    } else {
        take_modifier(&mut modifiers, "cta_group::1");
        1
    };
    take_modifier(&mut modifiers, "sync");
    take_modifier(&mut modifiers, "aligned");
    let operand = |stream: &mut PtxParser<'a, 'input>| ParsedOperandStr::parse.parse_next(stream);
    let comma = |stream: &mut PtxParser<'a, 'input>| Token::Comma.parse_next(stream).map(|_| ());
    // [taddr] as a value
    let bracketed = |stream: &mut PtxParser<'a, 'input>| -> PResult<ParsedOperandStr<'input>> {
        delimited(Token::LBracket, ParsedOperandStr::parse, Token::RBracket).parse_next(stream)
    };
    let count = |o: &ParsedOperandStr<'input>| match o {
        ast::ParsedOperand::VecPack(values) => values.len() as u8,
        _ => 1,
    };
    let single = |o: ParsedOperandStr<'input>| match o {
        ast::ParsedOperand::VecPack(ref values) if values.len() == 1 => match values[0] {
            Some(ast::RegOrImmediate::Reg(r)) => ast::ParsedOperand::Reg(r),
            _ => o,
        },
        other => other,
    };
    match op {
        "fence::before_thread_sync" | "fence::after_thread_sync" | "wait::ld" | "wait::st" | "relinquish_alloc_permit" => {
            if !modifiers.is_empty() {
                return backtrack();
            }
            Ok(ast::Instruction::Tcgen05Sync {})
        }
        "alloc" => {
            let space = if take_modifier(&mut modifiers, "shared::cta") { StateSpace::Shared } else { StateSpace::Generic };
            if !take_modifier(&mut modifiers, "b32") || !modifiers.is_empty() {
                return backtrack();
            }
            let addr = bracketed(stream)?;
            comma(stream)?;
            let ncols = operand(stream)?;
            Ok(ast::Instruction::Tcgen05Alloc {
                data: ast::Tcgen05AllocDetails { cta_group, space, exclusive },
                arguments: Tcgen05AllocArgs { src_addr: addr, src_ncols: ncols },
            })
        }
        "mma" => {
            // .sp and .ws come before .cta_group
            let sparse_first = take_modifier(&mut modifiers, "sp");
            let ws_first = take_modifier(&mut modifiers, "ws");
            let kind = [
                ("kind::f16", ast::Tcgen05Kind::F16),
                ("kind::tf32", ast::Tcgen05Kind::Tf32),
                ("kind::i8", ast::Tcgen05Kind::I8),
                ("kind::ti16", ast::Tcgen05Kind::Ti16),
                ("kind::f8f6f4", ast::Tcgen05Kind::F8f6f4),
                ("kind::mxf8f6f4", ast::Tcgen05Kind::Mxf8f6f4),
                ("kind::mxf4", ast::Tcgen05Kind::Mxf4),
                ("kind::mxf4nvf4", ast::Tcgen05Kind::Mxf4nvf4),
            ]
            .iter()
            .find(|(name, _)| take_modifier(&mut modifiers, name))
            .map(|&(_, k)| k);
            let Some(kind) = kind else {
                return backtrack();
            };
            let block_scale = take_modifier(&mut modifiers, "block_scale");
            let lut = take_modifier(&mut modifiers, "decompress::lut::b");
            let block = if !block_scale {
                None
            } else if take_modifier(&mut modifiers, "scale_vec::2X") {
                Some(2u8)
            } else if take_modifier(&mut modifiers, "scale_vec::4X") {
                Some(4)
            } else if take_modifier(&mut modifiers, "block16") {
                Some(16)
            } else if take_modifier(&mut modifiers, "block32") {
                Some(32)
            } else {
                take_modifier(&mut modifiers, "scale_vec::1X");
                Some(0)
            };
            let scaled_kind = matches!(kind, ast::Tcgen05Kind::Mxf8f6f4 | ast::Tcgen05Kind::Mxf4 | ast::Tcgen05Kind::Mxf4nvf4);
            if scaled_kind != block_scale {
                return backtrack();
            }
            let ashift = take_modifier(&mut modifiers, "ashift");
            let sparse = take_modifier(&mut modifiers, "sp");
            let ws = take_modifier(&mut modifiers, "ws");
            // the operand collector hints do not change the result
            modifiers.retain(|m| !m.starts_with("collector::"));
            if !modifiers.is_empty() {
                return backtrack();
            }
            let d = bracketed(stream)?;
            comma(stream)?;
            let (a, a_tmem) = if matches!(stream.input.first(), Some((Token::LBracket, _))) {
                (bracketed(stream)?, true)
            } else {
                (operand(stream)?, false)
            };
            comma(stream)?;
            let sparse = sparse || sparse_first;
            let ws = ws || ws_first;
            let b = operand(stream)?;
            comma(stream)?;
            if lut && (sparse || !matches!(kind, ast::Tcgen05Kind::F8f6f4 | ast::Tcgen05Kind::Mxf8f6f4)) {
                return backtrack();
            }
            let meta = if sparse || lut {
                let m = bracketed(stream)?;
                comma(stream)?;
                Some(m)
            } else {
                None
            };
            let idesc = operand(stream)?;
            comma(stream)?;
            // .block_scale: [scale-A], [scale-B], enable
            let (scale_a, scale_b) = if block.is_some() {
                let sa = bracketed(stream)?;
                comma(stream)?;
                let sb = bracketed(stream)?;
                comma(stream)?;
                (Some(sa), Some(sb))
            } else {
                (None, None)
            };
            let first = operand(stream)?;
            let (mask, enable) = if matches!(first, ast::ParsedOperand::VecPack(_)) {
                comma(stream)?;
                (Some(first), operand(stream)?)
            } else {
                (None, first)
            };
            let last = opt(preceded(Token::Comma, ParsedOperandStr::parse)).parse_next(stream)?;
            let (scale, zmask) = if ws { (None, last) } else { (last, None) };
            let data = ast::Tcgen05MmaDetails {
                kind,
                a_tmem,
                ashift,
                mask: mask.is_some(),
                scale: scale.is_some(),
                sparse,
                ws,
                zmask: zmask.is_some(),
                pair: cta_group == 2,
                block,
                lut,
            };
            Ok(ast::Instruction::Tcgen05Mma {
                data,
                arguments: Tcgen05MmaArgs {
                    src_d: d,
                    src_a: a,
                    src_b: b,
                    src_meta: meta,
                    src_idesc: idesc,
                    src_scale_a: scale_a,
                    src_scale_b: scale_b,
                    src_mask: mask,
                    src_enable: enable,
                    src_scale: scale,
                    src_zmask: zmask,
                },
            })
        }
        "cp" => {
            let shape = ["128x256b", "4x256b", "128x128b", "64x128b.warpx2::02_13", "64x128b.warpx2::01_23", "32x128b.warpx4"];
            // the shape is split at the dot into two modifiers for the warpx forms
            let joined = modifiers.join(".");
            let decompress = if joined.ends_with(".b8x16.b6x16_p32") {
                6u8
            } else if joined.ends_with(".b8x16.b4x16_p64") {
                4
            } else {
                0
            };
            let base = joined
                .trim_end_matches(".b8x16.b6x16_p32")
                .trim_end_matches(".b8x16.b4x16_p64");
            let Some(code) = shape.iter().position(|s| *s == base) else {
                return backtrack();
            };
            let taddr = bracketed(stream)?;
            comma(stream)?;
            let desc = operand(stream)?;
            Ok(ast::Instruction::Tcgen05Cp {
                data: (code as u8, decompress, cta_group),
                arguments: Tcgen05CpArgs { src_taddr: taddr, src_desc: desc },
            })
        }
        "shift" => {
            if !take_modifier(&mut modifiers, "down") || !modifiers.is_empty() {
                return backtrack();
            }
            let taddr = bracketed(stream)?;
            Ok(ast::Instruction::Tcgen05Shift { data: cta_group, arguments: Tcgen05ShiftArgs { src_taddr: taddr } })
        }
        "commit" if cta_group == 1 => {
            let space = if take_modifier(&mut modifiers, "shared::cluster") | take_modifier(&mut modifiers, "shared::cta") {
                StateSpace::Shared
            } else {
                StateSpace::Generic
            };
            if !take_modifier(&mut modifiers, "mbarrier::arrive::one") || !take_modifier(&mut modifiers, "b64") || !modifiers.is_empty() {
                return backtrack();
            }
            let mbar = bracketed(stream)?;
            Ok(ast::Instruction::Tcgen05Commit { data: space, arguments: Tcgen05CommitArgs { src_mbar: mbar } })
        }
        "dealloc" => {
            if !take_modifier(&mut modifiers, "b32") || !modifiers.is_empty() {
                return backtrack();
            }
            let taddr = operand(stream)?;
            comma(stream)?;
            let ncols = operand(stream)?;
            Ok(ast::Instruction::Tcgen05Dealloc {
                data: ast::Tcgen05AllocDetails { cta_group, space: StateSpace::Generic, exclusive },
                arguments: Tcgen05DeallocArgs { src_taddr: taddr, src_ncols: ncols } })
        }
        "ld" | "st" => {
            let shape = [
                ("32x32b", ast::Tcgen05Shape::S32x32b),
                ("16x64b", ast::Tcgen05Shape::S16x64b),
                ("16x128b", ast::Tcgen05Shape::S16x128b),
                ("16x256b", ast::Tcgen05Shape::S16x256b),
                ("16x32bx2", ast::Tcgen05Shape::S16x32bx2),
            ]
            .iter()
            .find(|(name, _)| take_modifier(&mut modifiers, name))
            .map(|&(_, s)| s);
            let Some(shape) = shape else {
                return backtrack();
            };
            let num = modifiers
                .iter()
                .position(|m| m.strip_prefix('x').map_or(false, |n| n.parse::<u8>().is_ok()))
                .map(|i| modifiers.remove(i)[1..].parse::<u8>().unwrap());
            let Some(num) = num else {
                return backtrack();
            };
            let red = op == "ld" && take_modifier(&mut modifiers, "red");
            let spcompress = op == "ld" && take_modifier(&mut modifiers, "spcompress");
            if red || spcompress {
                let min = take_modifier(&mut modifiers, "min");
                if !min && !take_modifier(&mut modifiers, "max") {
                    return backtrack();
                }
                let abs = take_modifier(&mut modifiers, "abs");
                let nan = take_modifier(&mut modifiers, "NaN");
                if spcompress && (!take_modifier(&mut modifiers, "sp::2:4") || !take_modifier(&mut modifiers, "b2")) {
                    return backtrack();
                }
                let type_ = if take_modifier(&mut modifiers, "f32") {
                    0u8
                } else if take_modifier(&mut modifiers, "u32") {
                    1
                } else if take_modifier(&mut modifiers, "s32") {
                    2
                } else {
                    return backtrack();
                };
                if !modifiers.is_empty() || (spcompress && type_ != 0) {
                    return backtrack();
                }
                let per = match shape {
                    ast::Tcgen05Shape::S16x256b => 4,
                    ast::Tcgen05Shape::S16x128b => 2,
                    _ => 1,
                };
                let dst = single(operand(stream)?);
                comma(stream)?;
                let dst_c = if spcompress {
                    let c = single(operand(stream)?);
                    comma(stream)?;
                    Some(c)
                } else {
                    None
                };
                let dst_red = if red {
                    let r = operand(stream)?;
                    comma(stream)?;
                    Some(r)
                } else {
                    None
                };
                let taddr = bracketed(stream)?;
                let data = ast::Tcgen05LdRedDetails {
                    shape,
                    loaded: num * per,
                    min,
                    abs,
                    nan,
                    type_,
                    red,
                    spcompress,
                    dst_registers: count(&dst),
                    c_registers: dst_c.as_ref().map_or(1, |c| count(c)),
                };
                return Ok(ast::Instruction::Tcgen05LdRed {
                    data,
                    arguments: Tcgen05LdRedArgs { dst, dst_c, dst_red, src_taddr: taddr },
                });
            }
            let packed = take_modifier(&mut modifiers, if op == "ld" { "pack::16b" } else { "unpack::16b" });
            if !take_modifier(&mut modifiers, "b32") || !modifiers.is_empty() {
                return backtrack();
            }
            if op == "ld" {
                let dst = single(operand(stream)?);
                comma(stream)?;
                let taddr = bracketed(stream)?;
                let offset = opt(preceded(Token::Comma, ParsedOperandStr::parse)).parse_next(stream)?;
                let data = ast::Tcgen05LdStDetails { shape, num, packed, registers: count(&dst) };
                Ok(ast::Instruction::Tcgen05Ld { data, arguments: Tcgen05LdArgs { dst, src_taddr: taddr, src_offset: offset } })
            } else {
                let taddr = bracketed(stream)?;
                comma(stream)?;
                let first = operand(stream)?;
                let (offset, values) = if matches!(first, ast::ParsedOperand::VecPack(_)) {
                    (None, first)
                } else {
                    comma(stream)?;
                    (Some(first), operand(stream)?)
                };
                let values = single(values);
                let data = ast::Tcgen05LdStDetails { shape, num, packed, registers: count(&values) };
                Ok(ast::Instruction::Tcgen05St {
                    data,
                    arguments: Tcgen05StArgs { src_taddr: taddr, src_offset: offset, src_values: values },
                })
            }
        }
        _ => backtrack(),
    }
}

// spcompress.{b8,b16}.{b2,b4}.sp::2:4.xN {mdata}, {cdata}, {data}, spdesc
// spdecompress.{b8,b16}.{b2,b4}.sp::S:T.xN {data}, {mdata}, {cdata}
fn sparse_instruction<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    opcode: &str,
    mut modifiers: Vec<&'input str>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let element_bits = if take_modifier(&mut modifiers, "b8") {
        8u32
    } else if take_modifier(&mut modifiers, "b16") {
        16
    } else {
        return backtrack();
    };
    let index_bits = if take_modifier(&mut modifiers, "b2") {
        2u32
    } else if take_modifier(&mut modifiers, "b4") {
        4
    } else {
        return backtrack();
    };
    let (kept, group, iterations) = match modifiers.as_slice() {
        [ratio, n] => {
            let Some((s, t)) = ratio.strip_prefix("sp::").and_then(|r| r.split_once(':')) else {
                return backtrack();
            };
            let (Ok(s), Ok(t)) = (s.parse::<u32>(), t.parse::<u32>()) else {
                return backtrack();
            };
            let Some(Ok(n)) = n.strip_prefix('x').map(|n| n.parse::<u32>()) else {
                return backtrack();
            };
            (s, t, n)
        }
        _ => return backtrack(),
    };
    let compress = opcode == "spcompress";
    // each iteration of the uncompressed vector is at most 64 bits
    if !(1..=8).contains(&iterations) || (compress && (kept, group) != (2, 4)) || kept >= group || group * element_bits > 64 {
        return backtrack();
    }
    let operand = |stream: &mut PtxParser<'a, 'input>| -> PResult<ParsedOperandStr<'input>> {
        let operand = ParsedOperandStr::parse.parse_next(stream)?;
        Ok(match operand {
            ast::ParsedOperand::VecPack(ref values) if values.len() == 1 => match values[0] {
                Some(ast::RegOrImmediate::Reg(r)) => ast::ParsedOperand::Reg(r),
                _ => return backtrack(),
            },
            other => other,
        })
    };
    let first = operand(stream)?;
    Token::Comma.parse_next(stream)?;
    let second = operand(stream)?;
    Token::Comma.parse_next(stream)?;
    let third = operand(stream)?;
    let count = |o: &ParsedOperandStr<'input>| match o {
        ast::ParsedOperand::VecPack(values) => values.len() as u32,
        _ => 1,
    };
    // the vectors have the registers the instruction names (the model pads with zeros or truncates)
    let (m, c, d) = if compress { (&first, &second, &third) } else { (&second, &third, &first) };
    let data = ast::SparseDetails {
        element_bits: element_bits as u8,
        index_bits: index_bits as u8,
        kept: kept as u8,
        group: group as u8,
        iterations: iterations as u8,
        data: count(d) as u8,
        cdata: count(c) as u8,
        mdata: count(m) as u8,
    };
    if compress {
        Token::Comma.parse_next(stream)?;
        let desc = ParsedOperandStr::parse.parse_next(stream)?;
        Ok(ast::Instruction::Spcompress {
            data,
            arguments: SpcompressArgs { dst_m: first, dst_c: second, src_data: third, src_desc: desc },
        })
    } else {
        Ok(ast::Instruction::Spdecompress {
            data,
            arguments: SpdecompressArgs { dst: first, src_m: second, src_c: third },
        })
    }
}

// The mixed-precision forms with packed operands (PTX ISA 8.6 / 9.4), which
// the instruction grammar cannot express (several types after one another):
// {add,sub,mul}{.rnd}{.ftz}{.sat}.f32x2.{f16x2,bf16x2}.f32x2       d, a, c
// fma.rnd{.ftz}{.sat}.f32x2.{f16x2,bf16x2}.f32x2.f32x2              d, a, b, c
// {add,sub,mul}{.rnd}{.ftz}{.sat}.{f16x2,bf16x2}.f32x2.f32x2       d, a, c   (computed and rounded with rz)
// mul{.rnd}{.ftz}{.sat}.bf16x2.bf16x2.f16x2 (and f16x2.f16x2.bf16x2) d, a, c   (c converted to the type of a)
// and the scalar mul{.rnd}{.ftz}{.sat}.f32.{f16,bf16} d, a, c. Every other
// form stays with the grammar.
fn mixed_float_instruction<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    opcode: &str,
    mut modifiers: Vec<&'input str>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    if let Some(x4) = alt_fp_x4(stream, opcode, &mut modifiers)? {
        return Ok(x4);
    }
    let type_of = |m: &str| match m {
        "f32" => Some(ScalarType::F32),
        "f16" => Some(ScalarType::F16),
        "bf16" => Some(ScalarType::BF16),
        "f32x2" => Some(ScalarType::F32x2),
        "f16x2" => Some(ScalarType::F16x2),
        "bf16x2" => Some(ScalarType::BF16x2),
        _ => None,
    };
    let types: Vec<ScalarType> = modifiers.iter().filter_map(|m| type_of(m)).collect();
    modifiers.retain(|m| type_of(m).is_none());
    let rounding = [
        ("rn", ast::RoundingMode::NearestEven),
        ("rz", ast::RoundingMode::Zero),
        ("rm", ast::RoundingMode::NegativeInf),
        ("rp", ast::RoundingMode::PositiveInf),
    ]
    .iter()
    .find(|(name, _)| take_modifier(&mut modifiers, name))
    .map(|&(_, r)| r);
    let flush_to_zero = take_modifier(&mut modifiers, "ftz");
    // .sat may come after the types, or twice
    let mut saturate = false;
    while take_modifier(&mut modifiers, "sat") {
        saturate = true;
    }
    if opcode == "fma" && take_modifier(&mut modifiers, "oob") {
        let relu = take_modifier(&mut modifiers, "relu");
        let type_ = match types.as_slice() {
            &[t @ (ScalarType::F16 | ScalarType::BF16 | ScalarType::F16x2 | ScalarType::BF16x2)] => t,
            _ => return backtrack(),
        };
        if !modifiers.is_empty() || flush_to_zero || saturate || rounding != Some(ast::RoundingMode::NearestEven) {
            return backtrack();
        }
        let operand = |stream: &mut PtxParser<'a, 'input>| ParsedOperandStr::parse.parse_next(stream);
        let dst = operand(stream)?;
        Token::Comma.parse_next(stream)?;
        let src1 = operand(stream)?;
        Token::Comma.parse_next(stream)?;
        let src2 = operand(stream)?;
        Token::Comma.parse_next(stream)?;
        let src3 = operand(stream)?;
        return Ok(ast::Instruction::FmaOob {
            data: ast::ArithFloat {
                type_,
                rounding: ast::RoundingMode::NearestEven,
                flush_to_zero: if matches!(type_, ScalarType::F16 | ScalarType::F16x2) { Some(false) } else { None },
                saturate: false,
                is_fusable: false,
                relu,
            },
            arguments: FmaOobArgs { dst, src1, src2, src3 },
        });
    }
    // fma.f32 without a rounding mode (which ptxas rejects for sm_20 and later): .rn
    if opcode == "fma" && rounding.is_none() && types == [ScalarType::F32] && modifiers.is_empty() {
        let operand = |stream: &mut PtxParser<'a, 'input>| ParsedOperandStr::parse.parse_next(stream);
        let dst = operand(stream)?;
        Token::Comma.parse_next(stream)?;
        let src1 = operand(stream)?;
        Token::Comma.parse_next(stream)?;
        let src2 = operand(stream)?;
        Token::Comma.parse_next(stream)?;
        let src3 = operand(stream)?;
        return Ok(ast::Instruction::Fma {
            data: ast::ArithFloat {
                type_: ScalarType::F32,
                rounding: ast::RoundingMode::NearestEven,
                flush_to_zero: Some(flush_to_zero),
                saturate,
                is_fusable: false,
                relu: false,
            },
            arguments: ast::FmaArgs { dst, src1, src2, src3 },
        });
    }
    if !modifiers.is_empty() {
        return backtrack();
    }
    let narrow = |t: ScalarType| matches!(t, ScalarType::F16x2 | ScalarType::BF16x2);
    use ScalarType::*;
    let (op, dtype, atype, btype) = match (opcode, types.as_slice()) {
        ("mul", &[F32, a @ (F16 | BF16)]) => (ast::MixedFloatOp::Mul, F32, a, F32),
        ("add" | "sub", &[F32, a @ (F16 | BF16)]) => (op_of(opcode), F32, a, F32),
        ("fma", &[F32, a @ (F16 | BF16)]) => (ast::MixedFloatOp::Fma, F32, a, a),
        ("add" | "sub" | "mul", &[F32x2, a, F32x2]) if narrow(a) => (op_of(opcode), F32x2, a, F32x2),
        ("fma", &[F32x2, a, F32x2, F32x2]) if narrow(a) => (ast::MixedFloatOp::Fma, F32x2, a, a),
        ("add" | "sub" | "mul", &[d, F32x2, F32x2]) if narrow(d) => (op_of(opcode), d, F32x2, F32x2),
        ("mul", &[d, a, c]) if narrow(d) && d == a && narrow(c) && c != d => (ast::MixedFloatOp::Mul, d, a, c),
        _ => return backtrack(),
    };
    let fma = op == ast::MixedFloatOp::Fma;
    if fma && rounding.is_none() {
        return backtrack();
    }
    let dst = ParsedOperandStr::parse.parse_next(stream)?;
    Token::Comma.parse_next(stream)?;
    let src1 = ParsedOperandStr::parse.parse_next(stream)?;
    Token::Comma.parse_next(stream)?;
    let src2 = ParsedOperandStr::parse.parse_next(stream)?;
    let src3 = if fma {
        Token::Comma.parse_next(stream)?;
        Some(ParsedOperandStr::parse.parse_next(stream)?)
    } else {
        None
    };
    Ok(ast::Instruction::MixedFloat {
        data: ast::MixedFloatDetails {
            op,
            dtype,
            atype,
            btype,
            rounding: rounding.unwrap_or(ast::RoundingMode::NearestEven),
            flush_to_zero,
            saturate,
        },
        arguments: MixedFloatArgs { dst, src1, src2, src3 },
    })
}

// ld.volatile.relaxed.scope{.ss}{.vec}.type d, [a] (which the ISA rules out): a volatile load
fn ld_volatile_relaxed<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    mut modifiers: Vec<&'input str>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    if !take_modifier(&mut modifiers, "volatile") || !take_modifier(&mut modifiers, "relaxed") {
        return backtrack();
    }
    if !["cta", "cluster", "gpu", "sys"].iter().any(|s| take_modifier(&mut modifiers, s)) {
        return backtrack();
    }
    let space = [
        ("global", StateSpace::Global),
        ("shared", StateSpace::Shared),
        ("shared::cta", StateSpace::Shared),
        ("shared::cluster", StateSpace::SharedCluster),
        ("local", StateSpace::Local),
    ]
    .iter()
    .find(|(name, _)| take_modifier(&mut modifiers, name))
    .map_or(StateSpace::Generic, |&(_, s)| s);
    let vector = [("v2", 2u8), ("v4", 4), ("v8", 8)].iter().find(|(name, _)| take_modifier(&mut modifiers, name)).map(|&(_, n)| n);
    let type_ = match modifiers.as_slice() {
        [t] => match *t {
            "b8" => ScalarType::B8,
            "b16" => ScalarType::B16,
            "b32" => ScalarType::B32,
            "b64" => ScalarType::B64,
            "u8" => ScalarType::U8,
            "u16" => ScalarType::U16,
            "u32" => ScalarType::U32,
            "u64" => ScalarType::U64,
            "s8" => ScalarType::S8,
            "s16" => ScalarType::S16,
            "s32" => ScalarType::S32,
            "s64" => ScalarType::S64,
            "f16" => ScalarType::F16,
            "f32" => ScalarType::F32,
            "f64" => ScalarType::F64,
            _ => return backtrack(),
        },
        _ => return backtrack(),
    };
    let dst = ParsedOperandStr::parse.parse_next(stream)?;
    Token::Comma.parse_next(stream)?;
    let src = delimited(Token::LBracket, ParsedOperandStr::parse, Token::RBracket).parse_next(stream)?;
    Ok(ast::Instruction::Ld {
        data: ast::LdDetails {
            qualifier: ast::LdStQualifier::Volatile,
            state_space: space,
            caching: ast::LdCacheOperator::Cached,
            typ: match vector {
                Some(n) => ast::Type::Vector(n, type_),
                None => ast::Type::Scalar(type_),
            },
            non_coherent: false,
        },
        arguments: ast::LdArgs { dst, src },
    })
}

// fabric.try_get / try_put / try_red / try_pullred / try_atom (the forms without .tensor), fabric.submit, fabric.wait
fn fabric_instruction<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    mut modifiers: Vec<&'input str>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let none = ast::FabricArgs {
        src_le: None,
        src_off: None,
        src_counter: None,
        src_mem: None,
        src_mem2: None,
        src_size: None,
        src_mbar: None,
        src_mask: None,
        src_tm: None,
        src_d0: None,
        src_d1: None,
        src_d2: None,
        src_d3: None,
        src_d4: None,
        src_c0: None,
        src_c1: None,
        src_c2: None,
        src_c3: None,
        src_c4: None,
    };
    let kind = match modifiers.first().copied() {
        Some("submit" | "wait") => {
            return Ok(ast::Instruction::Fabric { data: ast::FabricDetails { kind: 0, rank: 0, op: 0, type_: 0, flags: 0 }, arguments: none });
        }
        Some("try_get") => 1u8,
        Some("try_put") => 2,
        Some("try_red") => 3,
        Some("try_pullred") => 4,
        Some("try_atom") => 5,
        _ => return backtrack(),
    };
    modifiers.remove(0);
    if take_modifier(&mut modifiers, "tensor") {
        return fabric_tensor(stream, modifiers, kind, none);
    }
    let mut flags = 0u8;
    for (name, bit) in [("multimem", 1u8), ("cp_mask", 2), ("counted::bytes", 4)] {
        if take_modifier(&mut modifiers, name) {
            flags |= bit;
        }
    }
    let op = ["add", "min", "max", "and", "or", "xor", "inc", "dec", "exch", "cas"]
        .iter()
        .position(|o| take_modifier(&mut modifiers, o))
        .map(|p| p as u8);
    const TYPES: [&str; 18] = [
        "b32", "u32", "s32", "b64", "u64", "s64", "f32", "f64", "f16", "bf16", "f16x2", "bf16x2", "e4m3", "e5m2",
        "e4m3x2", "e5m2x2", "e4m3x4", "e5m2x4",
    ];
    let type_ = TYPES.iter().position(|t| take_modifier(&mut modifiers, t)).map(|p| p as u8);
    fabric_drop_modifiers(&mut modifiers);
    if !modifiers.is_empty() || (matches!(kind, 3 | 4 | 5) && (op.is_none() || type_.is_none())) {
        return backtrack();
    }
    let data = ast::FabricDetails { kind, rank: 0, op: op.unwrap_or(0), type_: type_.unwrap_or(0), flags };
    let operand = |stream: &mut PtxParser<'a, 'input>| ParsedOperandStr::parse.parse_next(stream);
    let comma = |stream: &mut PtxParser<'a, 'input>| Token::Comma.parse_next(stream).map(|_| ());
    let memory = |stream: &mut PtxParser<'a, 'input>| memory_operand(stream);
    // [leId, off{, counter}]
    let endpoint = |stream: &mut PtxParser<'a, 'input>| -> PResult<(ParsedOperandStr<'input>, ParsedOperandStr<'input>, Option<ParsedOperandStr<'input>>)> {
        Token::LBracket.parse_next(stream)?;
        let le = ParsedOperandStr::parse.parse_next(stream)?;
        Token::Comma.parse_next(stream)?;
        let off = ParsedOperandStr::parse.parse_next(stream)?;
        let counter = opt(preceded(Token::Comma, ParsedOperandStr::parse)).parse_next(stream)?;
        Token::RBracket.parse_next(stream)?;
        Ok((le, off, counter))
    };
    let mut a = none;
    match kind {
        1 | 4 => {
            a.src_mem = Some(memory(stream)?);
            comma(stream)?;
            let (le, off, _) = endpoint(stream)?;
            a.src_le = Some(le);
            a.src_off = Some(off);
            comma(stream)?;
            a.src_size = Some(operand(stream)?);
            comma(stream)?;
            a.src_mbar = Some(memory(stream)?);
            if kind == 4 {
                comma(stream)?;
                a.src_mask = Some(operand(stream)?);
            }
        }
        2 | 3 => {
            let (le, off, counter) = endpoint(stream)?;
            if counter.is_some() != (flags & 4 != 0) {
                return backtrack();
            }
            a.src_le = Some(le);
            a.src_off = Some(off);
            a.src_counter = counter;
            comma(stream)?;
            a.src_mem = Some(memory(stream)?);
            comma(stream)?;
            a.src_size = Some(operand(stream)?);
            comma(stream)?;
            a.src_mbar = Some(memory(stream)?);
            if flags & 2 != 0 {
                comma(stream)?;
                a.src_mask = Some(operand(stream)?);
            }
        }
        _ => {
            let (le, off, _) = endpoint(stream)?;
            a.src_le = Some(le);
            a.src_off = Some(off);
            comma(stream)?;
            a.src_mem = Some(memory(stream)?);
            comma(stream)?;
            a.src_mem2 = Some(memory(stream)?);
            comma(stream)?;
            a.src_mbar = Some(memory(stream)?);
        }
    }
    Ok(ast::Instruction::Fabric { data, arguments: a })
}

// the completion mechanisms, semantics and scopes do not change the emulation
fn fabric_drop_modifiers(modifiers: &mut Vec<&str>) {
    modifiers.retain(|m| {
        !matches!(
            *m,
            "async" | "shared::cta" | "shared::cluster" | "mbarrier::complete_tx::bytes" | "mbarrier::complete_tx::16B"
                | "mbarrier::report::fabric" | "relaxed" | "release" | "acquire" | "sys" | "gpu" | "cluster" | "b128"
                | "sync" | "aligned"
        )
    });
}

// fabric.try_get.tensor{.multimem}.Nd...tile [smem], [tm, le, off{, {dims}}, {coords}], [mbar];
// fabric.try_put.tensor{.multimem}.Nd...tile [tm, le, off{, {dims}}, {coords}], [smem], size, [mbar]
fn fabric_tensor<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    mut modifiers: Vec<&'input str>,
    kind: u8,
    mut a: ast::FabricArgs<ParsedOperandStr<'input>>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    if kind != 1 && kind != 2 {
        return backtrack();
    }
    let flags = take_modifier(&mut modifiers, "multimem") as u8;
    let rank = match ["1d", "2d", "3d", "4d", "5d"].iter().position(|d| take_modifier(&mut modifiers, d)) {
        Some(r) => r as u8 + 1,
        None => return backtrack(),
    };
    take_modifier(&mut modifiers, "tile");
    take_modifier(&mut modifiers, "override::global_address");
    let dims = take_modifier(&mut modifiers, "override::global_dim");
    fabric_drop_modifiers(&mut modifiers);
    if !modifiers.is_empty() {
        return backtrack();
    }
    let vector = |stream: &mut PtxParser<'a, 'input>| -> PResult<Vec<ParsedOperandStr<'input>>> {
        delimited(Token::LBrace, separated(1..=5, ParsedOperandStr::parse, Token::Comma), Token::RBrace).parse_next(stream)
    };
    let tensor = |stream: &mut PtxParser<'a, 'input>, a: &mut ast::FabricArgs<ParsedOperandStr<'input>>| -> PResult<()> {
        Token::LBracket.parse_next(stream)?;
        a.src_tm = Some(ParsedOperandStr::parse.parse_next(stream)?);
        Token::Comma.parse_next(stream)?;
        a.src_le = Some(ParsedOperandStr::parse.parse_next(stream)?);
        Token::Comma.parse_next(stream)?;
        a.src_off = Some(ParsedOperandStr::parse.parse_next(stream)?);
        Token::Comma.parse_next(stream)?;
        let d = if dims {
            let d = vector(stream)?;
            Token::Comma.parse_next(stream)?;
            d
        } else {
            Vec::new()
        };
        let c = vector(stream)?;
        Token::RBracket.parse_next(stream)?;
        if c.len() != rank as usize || (dims && d.len() != rank as usize) {
            return backtrack();
        }
        let mut d = d.into_iter();
        let mut c = c.into_iter();
        (a.src_d0, a.src_d1, a.src_d2, a.src_d3, a.src_d4) = (d.next(), d.next(), d.next(), d.next(), d.next());
        (a.src_c0, a.src_c1, a.src_c2, a.src_c3, a.src_c4) = (c.next(), c.next(), c.next(), c.next(), c.next());
        Ok(())
    };
    if kind == 1 {
        a.src_mem = Some(memory_operand(stream)?);
        Token::Comma.parse_next(stream)?;
        tensor(stream, &mut a)?;
    } else {
        tensor(stream, &mut a)?;
        Token::Comma.parse_next(stream)?;
        a.src_mem = Some(memory_operand(stream)?);
        Token::Comma.parse_next(stream)?;
        a.src_size = Some(ParsedOperandStr::parse.parse_next(stream)?);
    }
    Token::Comma.parse_next(stream)?;
    a.src_mbar = Some(memory_operand(stream)?);
    let data = ast::FabricDetails { kind: kind + 5, rank, op: 0, type_: 0, flags };
    Ok(ast::Instruction::Fabric { data, arguments: a })
}

// multimem.ld_reduce{.sem}{.scope}{.global}.op{.acc_prec}{.vec}.type d, [a];
// multimem.st{.sem}{.scope}{.global}{.vec}.type [a], b; multimem.red{.sem}{.scope}{.global}.op{.vec}.type [a], b
fn multimem_instruction<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    mut modifiers: Vec<&'input str>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let kind = if take_modifier(&mut modifiers, "ld_reduce") {
        0u8
    } else if take_modifier(&mut modifiers, "st") {
        1
    } else if take_modifier(&mut modifiers, "red") {
        2
    } else {
        return backtrack();
    };
    for m in ["weak", "relaxed", "acquire", "release", "cta", "cluster", "gpu", "sys", "global"] {
        take_modifier(&mut modifiers, m);
    }
    let op = ["add", "min", "max", "and", "or", "xor"]
        .iter()
        .position(|o| take_modifier(&mut modifiers, o))
        .map(|p| p as u8);
    let acc = if take_modifier(&mut modifiers, "acc::f32") {
        1u8
    } else if take_modifier(&mut modifiers, "acc::f16") {
        2
    } else {
        0
    };
    let vector = if take_modifier(&mut modifiers, "v2") {
        2u8
    } else if take_modifier(&mut modifiers, "v4") {
        4
    } else if take_modifier(&mut modifiers, "v8") {
        8
    } else {
        1
    };
    const TYPES: [&str; 18] = [
        "b32", "u32", "s32", "b64", "u64", "s64", "f32", "f64", "f16", "bf16", "f16x2", "bf16x2", "e4m3", "e5m2",
        "e4m3x2", "e5m2x2", "e4m3x4", "e5m2x4",
    ];
    let type_ = match modifiers.as_slice() {
        [t] => match TYPES.iter().position(|x| x == t) {
            Some(p) => p as u8,
            None => return backtrack(),
        },
        _ => return backtrack(),
    };
    if (kind == 1) != op.is_none() {
        return backtrack();
    }
    let data = ast::MultimemDetails { kind, op: op.unwrap_or(0), type_, vector, acc };
    let comma = |stream: &mut PtxParser<'a, 'input>| Token::Comma.parse_next(stream).map(|_| ());
    let address = |stream: &mut PtxParser<'a, 'input>| {
        delimited(Token::LBracket, ParsedOperandStr::parse, Token::RBracket).parse_next(stream)
    };
    let (dst, src_addr, src_value) = if kind == 0 {
        let d = ParsedOperandStr::parse.parse_next(stream)?;
        comma(stream)?;
        (Some(d), address(stream)?, None)
    } else {
        let a = address(stream)?;
        comma(stream)?;
        (None, a, Some(ParsedOperandStr::parse.parse_next(stream)?))
    };
    Ok(ast::Instruction::Multimem { data, arguments: MultimemArgs { dst, src_addr, src_value } })
}

// add / sub{.rn}{.satfinite}.dtype.atype d, a, c; mul{.rn}{.satfinite}.dtype.atype.ctype d, a, c;
// fma{.rn}{.satfinite}.dtype.atype.btype d, a, b, c (PTX ISA 9.4 9.7.6)
fn alt_fp_x4<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    opcode: &str,
    modifiers: &mut Vec<&'input str>,
) -> PResult<Option<ast::Instruction<ParsedOperandStr<'input>>>> {
    use ast::AltFpKind::*;
    let kind_of = |m: &str| match m {
        "e4m3x4" => Some(E4m3),
        "e5m2x4" => Some(E5m2),
        "e2m3x4" => Some(E2m3),
        "e3m2x4" => Some(E3m2),
        "e2m1x4" => Some(E2m1),
        "ue8m0x4" => Some(Ue8m0),
        "e2m1p4x4" => Some(E2m1p4),
        _ => None,
    };
    let kinds: Vec<ast::AltFpKind> = modifiers.iter().filter_map(|m| kind_of(m)).collect();
    if kinds.is_empty() {
        return Ok(None);
    }
    modifiers.retain(|m| kind_of(m).is_none());
    take_modifier(modifiers, "rn");
    let satfinite = take_modifier(modifiers, "satfinite");
    let three = opcode == "mul" || opcode == "fma";
    if !modifiers.is_empty() || kinds.len() != if three { 3 } else { 2 } || !matches!(kinds[0], E4m3 | E5m2) {
        return backtrack();
    }
    let op = match opcode {
        "add" => 0,
        "sub" => 1,
        "mul" => 2,
        _ => 3,
    };
    let (d, a) = (kinds[0], kinds[1]);
    let (b, c) = match op {
        2 => (d, kinds[2]),
        3 => (kinds[2], d),
        _ => (d, d),
    };
    let comma = |stream: &mut PtxParser<'a, 'input>| Token::Comma.parse_next(stream).map(|_| ());
    let dst = ParsedOperandStr::parse.parse_next(stream)?;
    comma(stream)?;
    let src_a = ParsedOperandStr::parse.parse_next(stream)?;
    comma(stream)?;
    let src_b = if op == 3 {
        let b = ParsedOperandStr::parse.parse_next(stream)?;
        comma(stream)?;
        Some(b)
    } else {
        None
    };
    let src_c = ParsedOperandStr::parse.parse_next(stream)?;
    Ok(Some(ast::Instruction::AltFpX4 {
        data: ast::AltFpX4Details { op, d, a, b, c, satfinite },
        arguments: AltFpX4Args { dst, src_a, src_b, src_c },
    }))
}

fn op_of(opcode: &str) -> ast::MixedFloatOp {
    match opcode {
        "add" => ast::MixedFloatOp::Add,
        "sub" => ast::MixedFloatOp::Sub,
        _ => ast::MixedFloatOp::Mul,
    }
}

// The texture and surface instructions in the forms the instruction grammar
// does not have (every geometry, lod and gradient operands, samplers,
// offsets, depth compare, the residency predicate, tld4, queries, surfaces).
// The plain `tex.{1d,2d,3d}.v4.dtype.ctype d, [t, c]` stays with the grammar.
// tex{.base|.level|.grad}.geom.v4.dtype.ctype d{|p}, [t{, s}, c] {, lod | , dPdx, dPdy} {, offsets} {, ref}
// tld4.comp.geom.v4.dtype.f32 d{|p}, [t{, s}, c] {, offsets} {, ref}
// txq.field.b32 d, [t]       suq.field.b32 d, [s]       istypep.kind p, a
// suld.b.geom{.vN}.btype{.cop}.mode d, [s, c]        sust.{b,p}.geom{.vN}.btype{.cop}.mode [s, c], v
// sured.{b,p}.op.geom.type.mode [s, c], v
fn texture_instruction<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    opcode: &str,
    mut modifiers: Vec<&'input str>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let operand = |stream: &mut PtxParser<'a, 'input>| ParsedOperandStr::parse.parse_next(stream);
    let comma = |stream: &mut PtxParser<'a, 'input>| Token::Comma.parse_next(stream).map(|_| ());
    // [handle {, handle}, {coords}] or [handle]
    let bracket = |stream: &mut PtxParser<'a, 'input>| -> PResult<Vec<ParsedOperandStr<'input>>> {
        let (first, rest) = delimited(
            Token::LBracket,
            (ParsedOperandStr::parse, repeat(0.., preceded(Token::Comma, ParsedOperandStr::parse))),
            Token::RBracket,
        )
        .parse_next(stream)?;
        let mut all = vec![first];
        let rest: Vec<_> = rest;
        all.extend(rest);
        Ok(all)
    };
    let rest_operands = |stream: &mut PtxParser<'a, 'input>| -> PResult<Vec<ParsedOperandStr<'input>>> {
        let mut out = Vec::new();
        while opt(Token::Comma).parse_next(stream)?.is_some() {
            out.push(ParsedOperandStr::parse.parse_next(stream)?);
        }
        Ok(out)
    };
    match opcode {
        "tex" | "tld4" => {
            let op = if opcode == "tld4" {
                let comp = ["r", "g", "b", "a"].iter().position(|c| take_modifier(&mut modifiers, c));
                match comp {
                    Some(c) => ast::TexOp::Tld4(c as u8),
                    None => return backtrack(),
                }
            } else {
                ast::TexOp::Tex
            };
            let plain = modifiers.len() == 4;
            let lod_mode = if take_modifier(&mut modifiers, "level") {
                1
            } else if take_modifier(&mut modifiers, "grad") {
                2
            } else {
                take_modifier(&mut modifiers, "base");
                0
            };
            let Some(geometry) = take_geometry(&mut modifiers) else {
                return backtrack();
            };
            let v2 = take_modifier(&mut modifiers, "v2");
            if !v2 && !take_modifier(&mut modifiers, "v4") {
                return backtrack();
            }
            // .dtype then .ctype, in that order; .v4.f16 (four halves) and .v2.f16x2 (pairs of them)
            let type_of = |m: &str| match m {
                "f32" => Some(ScalarType::F32),
                "s32" => Some(ScalarType::S32),
                "u32" => Some(ScalarType::U32),
                "f16" => Some(ScalarType::F16),
                "f16x2" => Some(ScalarType::F16x2),
                _ => None,
            };
            let (Some(dtype), Some(ctype)) = (
                modifiers.first().and_then(|m| type_of(m)),
                modifiers.get(1).and_then(|m| type_of(m)),
            ) else {
                return backtrack();
            };
            if modifiers.len() != 2
                || !matches!(ctype, ScalarType::S32 | ScalarType::F32)
                || v2 != (dtype == ScalarType::F16x2)
            {
                return backtrack();
            }
            let dst = operand(stream)?;
            let dst_pred = if opt(Token::Pipe).parse_next(stream)?.is_some() { Some(operand(stream)?) } else { None };
            comma(stream)?;
            let mut handles = bracket(stream)?;
            let extra = rest_operands(stream)?;
            let simple_geometry = matches!(geometry, ast::TexGeometry::D1 | ast::TexGeometry::D2 | ast::TexGeometry::D3);
            if matches!(op, ast::TexOp::Tex) && plain && lod_mode == 0 && simple_geometry && dst_pred.is_none()
                && handles.len() == 2 && extra.is_empty() && !matches!(dtype, ScalarType::F16 | ScalarType::F16x2)
            {
                // the instruction grammar's form
                return backtrack();
            }
            if handles.len() < 2 || handles.len() > 3 {
                return backtrack();
            }
            let coords = operand_elements(handles.pop().unwrap())?;
            let src_sampler = if handles.len() == 2 { handles.pop() } else { None };
            let src_tex = handles.pop().unwrap();
            let mut dsts = operand_elements(dst)?;
            if dsts.len() != if v2 { 2 } else { 4 } || coords.len() > 4 {
                return backtrack();
            }
            // the two pairs of .v2.f16x2 go to dst0 and dst1
            while dsts.len() < 4 {
                dsts.push(ast::ParsedOperand::Reg(discarded_register()));
            }
            let mut extra = extra.into_iter().peekable();
            let src_lod = if lod_mode == 1 { extra.next() } else { None };
            let (dx, dy) = if lod_mode == 2 {
                match (extra.next(), extra.next()) {
                    (Some(dx), Some(dy)) => (operand_elements(dx)?, operand_elements(dy)?),
                    _ => return backtrack(),
                }
            } else {
                (Vec::new(), Vec::new())
            };
            // a vector next is the offset (a 1d offset is a lone register, which then reads as the reference)
            let offsets = match extra.peek() {
                Some(ast::ParsedOperand::VecPack(_)) => operand_elements(extra.next().unwrap())?,
                _ => Vec::new(),
            };
            let src_ref = extra.next();
            if extra.next().is_some() || dx.len() > 3 || dy.len() > 3 || offsets.len() > 3 {
                return backtrack();
            }
            let mut d = dsts.into_iter();
            let mut c = coords.into_iter();
            let mut dx = dx.into_iter();
            let mut dy = dy.into_iter();
            let mut o = offsets.into_iter();
            Ok(ast::Instruction::TexAgx {
                data: ast::TexAgxDetails { op, geometry, lod_mode, dtype, ctype },
                arguments: ast::TexAgxArgs {
                    dst0: d.next().unwrap(),
                    dst1: d.next().unwrap(),
                    dst2: d.next().unwrap(),
                    dst3: d.next().unwrap(),
                    dst_pred,
                    src_tex,
                    src_sampler,
                    src_c0: c.next(),
                    src_c1: c.next(),
                    src_c2: c.next(),
                    src_c3: c.next(),
                    src_lod,
                    src_dx0: dx.next(),
                    src_dx1: dx.next(),
                    src_dx2: dx.next(),
                    src_dy0: dy.next(),
                    src_dy1: dy.next(),
                    src_dy2: dy.next(),
                    src_o0: o.next(),
                    src_o1: o.next(),
                    src_o2: o.next(),
                    src_ref,
                },
            })
        }
        "txq" | "suq" => {
            let fields: &[&str] = if opcode == "txq" {
                &["width", "height", "depth", "array_size", "normalized_coords", "filter_mode", "addr_mode_0",
                  "num_mipmap_levels", "addr_mode_1", "addr_mode_2", "channel_data_type", "channel_order",
                  "num_samples", "force_unnormalized_coords"]
            } else {
                &["width", "height", "depth", "array_size", "channel_data_type", "channel_order", "memory_layout"]
            };
            let Some(field) = fields.iter().position(|f| take_modifier(&mut modifiers, f)) else {
                return backtrack();
            };
            take_modifier(&mut modifiers, "level");
            if !take_modifier(&mut modifiers, "b32") || !modifiers.is_empty() {
                return backtrack();
            }
            // txq: the addr_mode_* fields read the same word; channel_data_type, channel_order and num_samples 0
            let field = match (opcode, field) {
                ("txq", 8 | 9) => 6,
                ("txq", 10 | 11) => 255,
                (_, f) => f,
            } as u8;
            let dst = operand(stream)?;
            comma(stream)?;
            let mut handles = bracket(stream)?;
            let mut extra = rest_operands(stream)?;
            // txq.level.{width,height,depth} d, [t], lod
            if handles.len() != 1 || extra.len() > 1 || (!extra.is_empty() && (opcode != "txq" || field > 2)) {
                return backtrack();
            }
            Ok(ast::Instruction::TexQuery {
                data: ast::TexQueryDetails { surface: opcode == "suq", field },
                arguments: ast::TexQueryArgs { dst, src: handles.pop().unwrap(), src_level: extra.pop() },
            })
        }
        "istypep" => {
            let Some(kind) = ["texref", "samplerref", "surfref"].iter().position(|k| take_modifier(&mut modifiers, k))
            else {
                return backtrack();
            };
            if !modifiers.is_empty() {
                return backtrack();
            }
            let dst = operand(stream)?;
            comma(stream)?;
            let src = operand(stream)?;
            Ok(ast::Instruction::Istypep { data: kind as u8, arguments: ast::IstypepArgs { dst, src } })
        }
        _ => {
            let formatted = take_modifier(&mut modifiers, "p");
            if !formatted && !take_modifier(&mut modifiers, "b") {
                return backtrack();
            }
            let op = if opcode == "sured" {
                let op = match modifiers.first() {
                    Some(&"add") => ast::AtomicOp::Add,
                    Some(&"and") => ast::AtomicOp::And,
                    Some(&"or") => ast::AtomicOp::Or,
                    Some(&"min") => ast::AtomicOp::UnsignedMin,
                    Some(&"max") => ast::AtomicOp::UnsignedMax,
                    _ => return backtrack(),
                };
                modifiers.remove(0);
                Some(op)
            } else {
                None
            };
            let Some(geometry) = take_geometry(&mut modifiers).filter(|g| !matches!(g, ast::TexGeometry::Ms2d | ast::TexGeometry::AMs2d))
            else {
                return backtrack();
            };
            let vector = if take_modifier(&mut modifiers, "v4") {
                4
            } else if take_modifier(&mut modifiers, "v2") {
                2
            } else {
                take_modifier(&mut modifiers, "v1");
                1
            };
            let types = [
                ("b8", ScalarType::B8),
                ("b16", ScalarType::B16),
                ("b32", ScalarType::B32),
                ("b64", ScalarType::B64),
                ("u32", ScalarType::U32),
                ("s32", ScalarType::S32),
                ("u64", ScalarType::U64),
                ("s64", ScalarType::S64),
            ];
            let Some(type_) = take_type(&mut modifiers, &types) else {
                return backtrack();
            };
            let mode = if take_modifier(&mut modifiers, "zero") {
                ast::SurfaceMode::Zero
            } else if take_modifier(&mut modifiers, "clamp") {
                ast::SurfaceMode::Clamp
            } else {
                take_modifier(&mut modifiers, "trap");
                ast::SurfaceMode::Trap
            };
            // cache operators and .vec are hints
            for hint in ["ca", "cg", "cs", "cv", "wb", "wt"] {
                take_modifier(&mut modifiers, hint);
            }
            if !modifiers.is_empty() {
                return backtrack();
            }
            let surface = |stream: &mut PtxParser<'a, 'input>| -> PResult<(ParsedOperandStr<'input>, Vec<ParsedOperandStr<'input>>)> {
                let mut handles = bracket(stream)?;
                if handles.len() != 2 {
                    return backtrack();
                }
                let coords = operand_elements(handles.pop().unwrap())?;
                if coords.len() > 4 {
                    return backtrack();
                }
                Ok((handles.pop().unwrap(), coords))
            };
            let data = ast::SurfaceDetails { geometry, mode, vector, type_, formatted, op };
            if opcode == "suld" {
                if formatted {
                    return backtrack();
                }
                let dst = operand_elements(operand(stream)?)?;
                comma(stream)?;
                let (src_surf, coords) = surface(stream)?;
                if dst.len() != vector as usize {
                    return backtrack();
                }
                let mut d = dst.into_iter();
                let mut c = coords.into_iter();
                return Ok(ast::Instruction::SuldAgx {
                    data,
                    arguments: ast::SuldAgxArgs {
                        dst0: d.next().unwrap(),
                        dst1: d.next(),
                        dst2: d.next(),
                        dst3: d.next(),
                        src_surf,
                        src_c0: c.next(),
                        src_c1: c.next(),
                        src_c2: c.next(),
                        src_c3: c.next(),
                    },
                });
            }
            let (src_surf, coords) = surface(stream)?;
            comma(stream)?;
            let values = operand_elements(operand(stream)?)?;
            let mut c = coords.into_iter();
            if opcode == "sured" {
                if values.len() != 1 {
                    return backtrack();
                }
                return Ok(ast::Instruction::SuredAgx {
                    data,
                    arguments: ast::SuredAgxArgs {
                        src_surf,
                        src_c0: c.next(),
                        src_c1: c.next(),
                        src_c2: c.next(),
                        src_c3: c.next(),
                        src_v: values.into_iter().next().unwrap(),
                    },
                });
            }
            if values.len() != vector as usize {
                return backtrack();
            }
            let mut v = values.into_iter();
            Ok(ast::Instruction::SustAgx {
                data,
                arguments: ast::SustAgxArgs {
                    src_surf,
                    src_c0: c.next(),
                    src_c1: c.next(),
                    src_c2: c.next(),
                    src_c3: c.next(),
                    src_v0: v.next().unwrap(),
                    src_v1: v.next(),
                    src_v2: v.next(),
                    src_v3: v.next(),
                },
            })
        }
    }
}

fn tensormap_replace<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    mut modifiers: Vec<&'input str>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    use ast::TensormapField::*;
    if !take_modifier(&mut modifiers, "replace") || !take_modifier(&mut modifiers, "tile") || !take_modifier(&mut modifiers, "b1024")
    {
        return backtrack();
    }
    let fields = [
        ("global_address", GlobalAddress),
        ("rank", Rank),
        ("box_dim", BoxDim),
        ("global_dim", GlobalDim),
        ("global_stride", GlobalStride),
        ("element_stride", ElementStride),
        ("elemtype", Elemtype),
        ("interleave_layout", InterleaveLayout),
        ("swizzle_mode", SwizzleMode),
        ("swizzle_atomicity", SwizzleAtomicity),
        ("fill_mode", FillMode),
    ];
    let Some(&(_, field)) = fields.iter().find(|(name, _)| take_modifier(&mut modifiers, name)) else {
        return backtrack();
    };
    let space = if take_modifier(&mut modifiers, "shared::cta") {
        StateSpace::Shared
    } else {
        take_modifier(&mut modifiers, "global");
        StateSpace::Global
    };
    let type_ = if take_modifier(&mut modifiers, "b32") {
        ScalarType::B32
    } else if take_modifier(&mut modifiers, "b64") {
        ScalarType::B64
    } else {
        return backtrack();
    };
    if !modifiers.is_empty() {
        return backtrack();
    }
    let tmap = delimited(Token::LBracket, ParsedOperandStr::parse, Token::RBracket).parse_next(stream)?;
    Token::Comma.parse_next(stream)?;
    let first = ParsedOperandStr::parse.parse_next(stream)?;
    let (src_ord, src_value) = if opt(Token::Comma).parse_next(stream)?.is_some() {
        (Some(first), ParsedOperandStr::parse.parse_next(stream)?)
    } else {
        (None, first)
    };
    if src_ord.is_some() != matches!(field, BoxDim | GlobalDim | GlobalStride | ElementStride) {
        return backtrack();
    }
    Ok(ast::Instruction::TensormapReplace {
        data: ast::TensormapReplaceDetails { field, space, type_ },
        arguments: ast::TensormapReplaceArgs { src_tmap: tmap, src_ord, src_value },
    })
}

// wgmma.mma_async{.sp}.sync.aligned.m64nNkK.dtype.atype.btype{.satfinite}{.bop.popc}
//     d, a | a-desc, b-desc{, sp-meta, sp-sel}, scale-d{, imm-scale-a, imm-scale-b{, imm-trans-a,} imm-trans-b}
fn wgmma<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let mut modifiers = matrix_modifiers(stream);
    if !take_modifier(&mut modifiers, "sync") || !take_modifier(&mut modifiers, "aligned") {
        return backtrack();
    }
    let control = if take_modifier(&mut modifiers, "fence") {
        Some(ast::WgmmaControl::Fence)
    } else if take_modifier(&mut modifiers, "commit_group") {
        Some(ast::WgmmaControl::CommitGroup)
    } else if take_modifier(&mut modifiers, "wait_group") {
        let n = match ParsedOperandStr::parse.parse_next(stream)? {
            ast::ParsedOperand::Imm(value) => match value.as_u64() {
                Some(n) => n as u32,
                None => return backtrack(),
            },
            _ => return backtrack(),
        };
        Some(ast::WgmmaControl::WaitGroup(n))
    } else {
        None
    };
    if let Some(control) = control {
        if !modifiers.is_empty() {
            return backtrack();
        }
        return Ok(ast::Instruction::WgmmaFence { data: control });
    }
    if !take_modifier(&mut modifiers, "mma_async") {
        return backtrack();
    }
    let sparse = take_modifier(&mut modifiers, "sp");
    let Some((m, n, k)) = take_mma_shape(&mut modifiers) else {
        return backtrack();
    };
    let satfinite = take_modifier(&mut modifiers, "satfinite");
    let b1_op = take_b1_op(&mut modifiers);
    let types = modifiers
        .iter()
        .filter_map(|m| ast::MmaType::from_name(m))
        .collect::<Vec<_>>();
    modifiers.retain(|m| ast::MmaType::from_name(m).is_none());
    let [dtype, atype, btype] = types[..] else {
        return backtrack();
    };
    if !modifiers.is_empty() || m != 64 || n == 0 || n > 256 || n % 8 != 0 {
        return backtrack();
    }
    let mut operands = vec![ParsedOperandStr::parse.parse_next(stream)?];
    while opt(Token::Comma).parse_next(stream)?.is_some() {
        operands.push(ParsedOperandStr::parse.parse_next(stream)?);
    }
    let imm = |o: &ParsedOperandStr<'input>| match o {
        ast::ParsedOperand::Imm(value) => value.as_u64().map(|v| v as i64),
        _ => None,
    };
    let mut rest = operands.into_iter();
    let (Some(dst), Some(src_a), Some(src_b)) = (rest.next(), rest.next(), rest.next()) else {
        return backtrack();
    };
    let a_descriptor = !matches!(src_a, ast::ParsedOperand::VecPack(_));
    let (src_metadata, selector) = if sparse {
        let (Some(meta), Some(sel)) = (rest.next(), rest.next()) else {
            return backtrack();
        };
        match imm(&sel) {
            Some(sel) if (0..4).contains(&sel) => (Some(meta), Some(sel as u32)),
            _ => return backtrack(),
        }
    } else {
        (None, None)
    };
    let Some(scale_d) = rest.next() else {
        return backtrack();
    };
    let (scale_d, src_scale_d) = match imm(&scale_d) {
        Some(v) => (Some(v != 0), None),
        None => (None, Some(scale_d)),
    };
    let mut immediates = Vec::new();
    for operand in rest {
        match imm(&operand) {
            Some(v) => immediates.push(v),
            None => return backtrack(),
        }
    }
    // imm-scale-a, imm-scale-b (floating point types), then the transposes of the 16-bit types (only B's when A
    // is in registers)
    let (negate_a, negate_b) = (immediates.first() == Some(&-1), immediates.get(1) == Some(&-1));
    let transposes = immediates.get(2..).unwrap_or(&[]);
    let (transpose_a, transpose_b) = match (a_descriptor, transposes) {
        (true, [a, b]) => (*a != 0, *b != 0),
        (false, [b]) => (false, *b != 0),
        (_, []) => (false, false),
        _ => return backtrack(),
    };
    let mma = ast::MmaDetails {
        m,
        n,
        k,
        alayout: ast::MatrixLayout::Row,
        blayout: ast::MatrixLayout::Col,
        dtype,
        atype,
        btype,
        ctype: dtype,
        satfinite,
        b1_op,
        block_scale: None,
        sparse: selector,
        rounding: None,
    };
    Ok(ast::Instruction::Wgmma {
        data: ast::WgmmaDetails {
            mma,
            a_descriptor,
            negate_a,
            negate_b,
            transpose_a,
            transpose_b,
            scale_d,
        },
        arguments: WgmmaArgs { src_c: dst.clone(), dst, src_a, src_b, src_scale_d, src_metadata },
    })
}

// mma.sync.aligned.shape.row.col{.satfinite}{.kind::f8f6f4}{.bop.popc}.dtype.atype.btype.ctype d, a, b, c;
// Block scaling (.block_scale) and sparse mma (mma.sp) are not supported.
fn mma<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let mut modifiers = matrix_modifiers(stream);
    let sparse =
        take_modifier(&mut modifiers, "sp::ordered_metadata") || take_modifier(&mut modifiers, "sp");
    if !take_modifier(&mut modifiers, "sync") || !take_modifier(&mut modifiers, "aligned") {
        return backtrack();
    }
    let Some((m, n, k)) = take_mma_shape(&mut modifiers) else {
        return backtrack();
    };
    let (Some(alayout), Some(blayout)) = (take_layout(&mut modifiers), take_layout(&mut modifiers))
    else {
        return backtrack();
    };
    let satfinite = take_modifier(&mut modifiers, "satfinite");
    let kind_f8f6f4 = take_modifier(&mut modifiers, "kind::f8f6f4");
    let block_scale = match take_block_scale(&mut modifiers) {
        Ok(block_scale) => block_scale,
        Err(()) => return backtrack(),
    };
    let b1_op = take_b1_op(&mut modifiers);
    let Some([dtype, mut atype, mut btype, ctype]) = take_mma_types(&mut modifiers) else {
        return backtrack();
    };
    // f64: the rounding of the fused multiply-adds
    let rounding = if atype == ast::MmaType::F64 {
        match take_rounding(&mut modifiers) {
            Some(ast::RoundingMode::NearestEven) | None => None,
            rounding => rounding,
        }
    } else {
        None
    };
    // fp4 elements are packed two per byte with kind::mxf4 and kind::mxf4nvf4
    if let Some((_, true)) = block_scale {
        for t in [&mut atype, &mut btype] {
            if *t == ast::MmaType::E2m1 {
                *t = ast::MmaType::E2m1x2;
            }
        }
    }
    let data = ast::MmaDetails {
        m,
        n,
        k,
        alayout,
        blayout,
        dtype,
        atype,
        btype,
        ctype,
        satfinite,
        b1_op,
        block_scale: None,
        sparse: None,
        rounding,
    };
    let supported = match (sparse, block_scale) {
        (true, block_scale) => data.is_supported_sparse(kind_f8f6f4, block_scale.as_ref().map(|(s, fp4)| (s, *fp4))),
        (false, None) => data.is_supported_mma(kind_f8f6f4),
        (false, Some((scale, fp4))) => data.is_supported_block_scale(&scale, fp4),
    };
    // m8n8k4 with .f16 A and B takes every layout
    let any_layout = (m, n, k) == (8, 8, 4) && atype == ast::MmaType::F16;
    if !modifiers.is_empty()
        || (!any_layout && (alayout != ast::MatrixLayout::Row || blayout != ast::MatrixLayout::Col))
        || !supported
    {
        return backtrack();
    }
    let (dst, _, src1, _, src2, _, src3) = (
        ParsedOperandStr::parse,
        Token::Comma,
        ParsedOperandStr::parse,
        Token::Comma,
        ParsedOperandStr::parse,
        Token::Comma,
        ParsedOperandStr::parse,
    )
        .parse_next(stream)?;
    // , metadata, sparsity selector
    let sparse = if sparse {
        let (_, metadata, _, selector) =
            (Token::Comma, ParsedOperandStr::parse, Token::Comma, ParsedOperandStr::parse)
                .parse_next(stream)?;
        let selector = match selector {
            ast::ParsedOperand::Imm(value) => match value.as_u64() {
                Some(selector) if selector < 4 => selector as u32,
                _ => return backtrack(),
            },
            _ => return backtrack(),
        };
        Some((metadata, selector))
    } else {
        None
    };
    let mut scale_operands = None;
    let mut scale = None;
    if let Some((mut s, _)) = block_scale {
        // , scale-a, {byte-id-a, thread-id-a}, scale-b, {byte-id-b, thread-id-b}
        let (_, src_scale_a, _, (a_byte, a_thread), _, src_scale_b, _, (b_byte, b_thread)) = (
            Token::Comma,
            ParsedOperandStr::parse,
            Token::Comma,
            scale_selector,
            Token::Comma,
            ParsedOperandStr::parse,
            Token::Comma,
            scale_selector,
        )
            .parse_next(stream)?;
        (s.a_byte, s.a_thread, s.b_byte, s.b_thread) = (a_byte, a_thread, b_byte, b_thread);
        scale = Some(s);
        scale_operands = Some((src_scale_a, src_scale_b));
    }
    let data = ast::MmaDetails {
        block_scale: scale,
        sparse: sparse.as_ref().map(|(_, selector)| *selector),
        ..data
    };
    if let Some((src_metadata, _)) = sparse {
        let (src_scale_a, src_scale_b) = match scale_operands {
            Some((a, b)) => (Some(a), Some(b)),
            None => (None, None),
        };
        return Ok(ast::Instruction::MmaSparse {
            data,
            arguments: MmaSparseArgs {
                dst,
                src1,
                src2,
                src3,
                src_metadata,
                src_scale_a,
                src_scale_b,
            },
        });
    }
    let Some((src_scale_a, src_scale_b)) = scale_operands else {
        return Ok(ast::Instruction::Mma {
            data,
            arguments: MmaArgs {
                dst,
                src1,
                src2,
                src3,
            },
        });
    };
    Ok(ast::Instruction::MmaBlockScale {
        data,
        arguments: MmaBlockScaleArgs {
            dst,
            src1,
            src2,
            src3,
            src_scale_a,
            src_scale_b,
        },
    })
}

// .kind::{mxf8f6f4, mxf4, mxf4nvf4}.block_scale{.scale_vec::NX} ... .stype:
// the scale factors (with selectors to fill in) and whether the elements are
// packed fp4.
fn take_block_scale(modifiers: &mut Vec<&str>) -> Result<Option<(ast::MmaBlockScale, bool)>, ()> {
    let kinds = [("kind::mxf8f6f4", 1, false), ("kind::mxf4", 2, true), ("kind::mxf4nvf4", 0, true)];
    let Some((_, default_chunks, fp4)) = kinds
        .into_iter()
        .find(|(name, _, _)| take_modifier(modifiers, name))
    else {
        return Ok(None);
    };
    if !take_modifier(modifiers, "block_scale") {
        return Err(());
    }
    let chunks = [("scale_vec::1X", 1), ("scale_vec::2X", 2), ("scale_vec::4X", 4)]
        .into_iter()
        .find_map(|(name, chunks)| take_modifier(modifiers, name).then_some(chunks))
        .unwrap_or(default_chunks);
    let ue4m3 = take_modifier(modifiers, "ue4m3");
    if !ue4m3 && !take_modifier(modifiers, "ue8m0") {
        return Err(());
    }
    if chunks == 0 {
        return Err(());
    }
    Ok(Some((
        ast::MmaBlockScale {
            chunks,
            ue4m3,
            a_byte: 0,
            a_thread: 0,
            b_byte: 0,
            b_thread: 0,
        },
        fp4,
    )))
}

// {byte-id, thread-id}, as immediates
fn scale_selector<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<(u32, u32)> {
    let selector = ParsedOperandStr::parse.parse_next(stream)?;
    let ast::ParsedOperand::VecPack(items) = selector else {
        return backtrack();
    };
    let value = |item: &Option<ast::RegOrImmediate<&str>>| match item {
        Some(ast::RegOrImmediate::Imm(value)) => value.as_u64().map(|v| v as u32),
        _ => None,
    };
    match &items[..] {
        [byte, thread] => match (value(byte), value(thread)) {
            (Some(byte), Some(thread)) if byte < 4 && thread < 4 => Ok((byte, thread)),
            _ => backtrack(),
        },
        _ => backtrack(),
    }
}

// wmma.load.{a,b,c}.sync.aligned.layout.shape{.ss}.type r, [p]{, stride};
// wmma.store.d.sync.aligned.layout.shape{.ss}.type    [p], r{, stride};
// wmma.mma.sync.aligned.alayout.blayout.shape.dtype.ctype d, a, b, c;                (f16)
// wmma.mma{.bop.popc}.sync.aligned.alayout.blayout.shape{.rnd}.dtype.atype.btype.ctype{.satfinite} d, a, b, c;
fn wmma<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let mut modifiers = matrix_modifiers(stream);
    let operation = if modifiers.is_empty() {
        return backtrack();
    } else {
        modifiers.remove(0)
    };
    let b1_op = take_b1_op(&mut modifiers);
    if !take_modifier(&mut modifiers, "sync") || !take_modifier(&mut modifiers, "aligned") {
        return backtrack();
    }
    let Some((m, n, k)) = take_mma_shape(&mut modifiers) else {
        return backtrack();
    };
    if operation == "mma" {
        let (Some(alayout), Some(blayout)) =
            (take_layout(&mut modifiers), take_layout(&mut modifiers))
        else {
            return backtrack();
        };
        let satfinite = take_modifier(&mut modifiers, "satfinite");
        take_modifier(&mut modifiers, "rn");
        let Some([dtype, atype, btype, ctype]) = take_mma_types(&mut modifiers) else {
            return backtrack();
        };
        let data = ast::MmaDetails {
            m,
            n,
            k,
            alayout,
            blayout,
            dtype,
            atype,
            btype,
            ctype,
            satfinite,
            b1_op,
            block_scale: None,
            sparse: None,
            rounding: None,
        };
        if !modifiers.is_empty() || !data.is_supported_wmma() {
            return backtrack();
        }
        let (dst, _, src1, _, src2, _, src3) = (
            ParsedOperandStr::parse,
            Token::Comma,
            ParsedOperandStr::parse,
            Token::Comma,
            ParsedOperandStr::parse,
            Token::Comma,
            ParsedOperandStr::parse,
        )
            .parse_next(stream)?;
        return Ok(ast::Instruction::WmmaMma {
            data,
            arguments: WmmaMmaArgs {
                dst,
                src1,
                src2,
                src3,
            },
        });
    }
    if b1_op.is_some() || modifiers.is_empty() {
        return backtrack();
    }
    let matrix = match (operation, modifiers.remove(0)) {
        ("load", "a") => ast::MmaMatrix::A,
        ("load", "b") => ast::MmaMatrix::B,
        ("load", "c") => ast::MmaMatrix::C,
        ("store", "d") => ast::MmaMatrix::D,
        _ => return backtrack(),
    };
    let Some(layout) = take_layout(&mut modifiers) else {
        return backtrack();
    };
    let state_space = take_state_space(&mut modifiers).unwrap_or(StateSpace::Generic);
    let type_ = match &modifiers[..] {
        [name] => match ast::MmaType::from_name(name) {
            Some(type_) => type_,
            None => return backtrack(),
        },
        _ => return backtrack(),
    };
    let data = ast::WmmaLoadStoreDetails {
        matrix,
        layout,
        m,
        n,
        k,
        state_space,
        type_,
    };
    if !data.is_supported() {
        return backtrack();
    }
    // Without a stride, the matrix is packed: the stride is its number of
    // columns (row major) or rows (column major).
    let (rows, columns) = data.mma().matrix_size(matrix);
    let default_stride = match layout {
        ast::MatrixLayout::Row => columns,
        ast::MatrixLayout::Col => rows,
    };
    let stride = |stream: &mut PtxParser<'a, 'input>| {
        opt(preceded(Token::Comma, ParsedOperandStr::parse))
            .map(|stride| {
                stride.unwrap_or(ast::ParsedOperand::Imm(ast::ImmediateValue::U64(
                    default_stride as u64,
                )))
            })
            .parse_next(stream)
    };
    if matrix == ast::MmaMatrix::D {
        let (src1, _, src2, src_stride) =
            (bracketed_operand, Token::Comma, ParsedOperandStr::parse, stride)
                .parse_next(stream)?;
        Ok(ast::Instruction::WmmaStore {
            data,
            arguments: WmmaStoreArgs {
                src1,
                src2,
                src_stride,
            },
        })
    } else {
        let (dst, _, src, src_stride) =
            (ParsedOperandStr::parse, Token::Comma, bracketed_operand, stride)
                .parse_next(stream)?;
        Ok(ast::Instruction::WmmaLoad {
            data,
            arguments: WmmaLoadArgs {
                dst,
                src,
                src_stride,
            },
        })
    }
}

// `_` in place of an optional destination
fn sink_to_none<'input>(operand: ParsedOperandStr<'input>) -> Option<ParsedOperandStr<'input>> {
    match operand {
        ast::ParsedOperand::VecPack(ref elements) if elements.len() == 1 && elements[0].is_none() => {
            None
        }
        operand => Some(operand),
    }
}

// .shared::cta and .shared::cluster addresses are .shared addresses (on AGX, a cluster
// is one threadgroup with the CTAs' shared memory side by side)
fn shared_space(space: Option<StateSpace>) -> StateSpace {
    match space {
        Some(StateSpace::Shared | StateSpace::SharedCta | StateSpace::SharedCluster) => {
            StateSpace::Shared
        }
        Some(space) => space,
        None => StateSpace::Generic,
    }
}

fn mbarrier_arrive<'input>(
    mbstate: ParsedOperandStr<'input>,
    drop: bool,
    expect_tx: bool,
    no_complete: bool,
    space: Option<StateSpace>,
    addr: ParsedOperandStr<'input>,
    count: Option<ParsedOperandStr<'input>>,
    extra: Option<ParsedOperandStr<'input>>,
    multicast: bool,
) -> ast::Instruction<ParsedOperandStr<'input>> {
    // .noComplete takes a count (it only marks that the arrival must not complete the phase)
    let _ = no_complete;
    // .multicast::cluster::32b: the CTA mask is the last operand
    let (count, cta_mask) = match (multicast, count, extra) {
        (true, Some(count), Some(mask)) => (Some(count), Some(mask)),
        (true, mask, None) => (None, mask),
        (_, count, _) => (count, None),
    };
    let (count, tx_count) = if expect_tx { (None, count) } else { (count, None) };
    ast::Instruction::MbarrierArrive {
        data: ast::MbarrierArriveDetails {
            space: shared_space(space),
            drop,
        },
        arguments: ast::MbarrierArriveArgs {
            dst_state: sink_to_none(mbstate),
            src_mbarrier: addr,
            src_count: count,
            src_tx_count: tx_count,
            src_cta_mask: cta_mask,
        },
    }
}

// A register for a discarded result (a call's left-out return value, the sink `_` of a destination): declared on
// first use (`get_or_declare` in normalize_identifiers), its name kept for the life of the program
fn discarded_register() -> &'static str {
    static DISCARDED: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = DISCARDED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Box::leak(format!("%__discarded_return{n}").into_boxed_str())
}

fn sink_to_register<'input>(operand: ast::ParsedOperand<&'input str>) -> ast::ParsedOperand<&'input str> {
    match operand {
        ast::ParsedOperand::VecPack(ref elements) if matches!(elements[..], [None]) => {
            ast::ParsedOperand::Reg(discarded_register())
        }
        operand => operand,
    }
}

// .xorsign of min/max needs .abs (.abs alone takes the magnitudes of the inputs, PTX ISA 9.3)
fn xorsign_abs(state: &mut PtxParserState, xorsign: bool, abs: bool) -> bool {
    if xorsign && !abs {
        state
            .errors
            .push(PtxError::SyntaxError(".xorsign requires .abs".to_string()));
    }
    xorsign && abs
}

fn packed_cvt<'input, 'e>(
    errors: &mut Vec<PtxError<'e>>,
    rounding: RawRoundingMode,
    relu: bool,
    satfinite: bool,
    dst_type: PackedCvtType,
    src_type: PackedCvtType,
    dst: ParsedOperandStr<'input>,
    src: ParsedOperandStr<'input>,
    src2: Option<ParsedOperandStr<'input>>,
) -> ast::Instruction<ParsedOperandStr<'input>> {
    fn mini(t: PackedCvtType) -> Option<ast::MiniFormat> {
        Some(match t {
            PackedCvtType::E4m3x2 => ast::MiniFormat::E4m3x2,
            PackedCvtType::E5m2x2 => ast::MiniFormat::E5m2x2,
            PackedCvtType::E2m1x2 => ast::MiniFormat::E2m1x2,
            PackedCvtType::E2m3x2 => ast::MiniFormat::E2m3x2,
            PackedCvtType::E3m2x2 => ast::MiniFormat::E3m2x2,
            PackedCvtType::Ue8m0x2 => ast::MiniFormat::Ue8m0x2,
            _ => return None,
        })
    }
    fn scalar(t: PackedCvtType) -> ScalarType {
        match t {
            PackedCvtType::F16x2 => ScalarType::F16x2,
            PackedCvtType::BF16x2 => ScalarType::BF16x2,
            PackedCvtType::E4m3x2 => ScalarType::E4m3x2,
            PackedCvtType::E5m2x2 => ScalarType::E5m2x2,
            _ => ScalarType::F32,
        }
    }
    let is_half = |t| matches!(t, PackedCvtType::F16x2 | PackedCvtType::BF16x2);
    let is_legacy_fp8 = |t| matches!(t, PackedCvtType::E4m3x2 | PackedCvtType::E5m2x2);
    let mini_instruction = |format, direction, errors: &mut Vec<PtxError<'e>>| {
        let rounding = ast::RoundingMode::from(rounding);
        if (rounding == ast::RoundingMode::PositiveInf) != (format == ast::MiniFormat::Ue8m0x2 && direction != ast::MiniDirection::ToBf16x2)
            && rounding != ast::RoundingMode::Zero
        {
            errors.push(PtxError::SyntaxError("invalid rounding of cvt".to_string()));
        }
        ast::Instruction::CvtMini {
            data: ast::CvtMiniDetails {
                format,
                direction,
                rounding,
                relu,
            },
            arguments: CvtMiniArgs {
                dst: dst.clone(),
                src1: src.clone(),
                src2: src2.clone(),
            },
        }
    };
    let pair = src2.is_some();
    match (is_half(dst_type), mini(dst_type), src_type, mini(src_type)) {
        // cvt.frnd2{.relu}{.satfinite}.{f16x2,bf16x2}.f32 d, a, b
        (true, _, PackedCvtType::F32, _) if pair && rounding != RawRoundingMode::Rp => {
            let data = ast::CvtDetails::new(errors, Some(rounding), false, false, relu, satfinite, scalar(dst_type), ScalarType::F32);
            ast::Instruction::Cvt {
                data,
                arguments: ast::CvtArgs { dst, src, src2 }
            }
        }
        // widening of the narrow formats
        (true, _, _, Some(format)) if !pair && rounding == RawRoundingMode::Rn => {
            if is_legacy_fp8(src_type) && !relu && !satfinite {
                let data = ast::CvtDetails::new(errors, Some(rounding), false, false, false, false, scalar(dst_type), scalar(src_type));
                ast::Instruction::Cvt {
                    data,
                    arguments: ast::CvtArgs { dst, src, src2: None }
                }
            } else {
                let direction = if dst_type == PackedCvtType::F16x2 {
                    ast::MiniDirection::ToF16x2
                } else {
                    ast::MiniDirection::ToBf16x2
                };
                mini_instruction(format, direction, errors)
            }
        }
        // narrowing to them
        (false, Some(format), _, None) if pair == (src_type == PackedCvtType::F32) => {
            if is_legacy_fp8(dst_type) && pair && rounding == RawRoundingMode::Rn && !relu && satfinite {
                let data = ast::CvtDetails::new(errors, Some(rounding), false, false, false, false, scalar(dst_type), ScalarType::F32);
                ast::Instruction::Cvt {
                    data,
                    arguments: ast::CvtArgs { dst, src, src2 }
                }
            } else {
                let direction = match src_type {
                    PackedCvtType::F32 => ast::MiniDirection::FromF32,
                    PackedCvtType::F16x2 => ast::MiniDirection::FromF16x2,
                    _ => ast::MiniDirection::FromBf16x2,
                };
                mini_instruction(format, direction, errors)
            }
        }
        _ => {
            errors.push(PtxError::SyntaxError("invalid packed cvt".to_string()));
            mini_instruction(ast::MiniFormat::E4m3x2, ast::MiniDirection::ToF16x2, errors)
        }
    }
}

// The source text of the next token
fn next_token_text<'a, 'input>(stream: &mut PtxParser<'a, 'input>) -> PResult<&'input str> {
    let (_, span) = token::any.parse_next(stream)?;
    Ok(&stream.state.text[span])
}

fn video_selector(text: &str) -> Option<ast::VideoSelector> {
    let half = match text.get(..2)? {
        ".h" => true,
        ".b" => false,
        _ => return None,
    };
    let digits = &text[2..];
    if digits.is_empty() || digits.len() > 4 || !digits.bytes().all(|d| d.is_ascii_digit()) {
        return None;
    }
    let mut result = ast::VideoSelector {
        half,
        digits: [0; 4],
        len: digits.len() as u8,
    };
    for (i, d) in digits.bytes().enumerate() {
        result.digits[i] = d - b'0';
    }
    Some(result)
}

// A video instruction operand: {-}register{.selector} or an immediate
fn video_operand<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
) -> PResult<(
    bool,
    ast::ParsedOperand<&'input str>,
    Option<ast::VideoSelector>,
)> {
    let negate = opt(Token::Minus).parse_next(stream)?.is_some();
    if let Ok(imm) = immediate_value.parse_next(stream) {
        return Ok((negate, ast::ParsedOperand::Imm(imm), None));
    }
    let name = ident.parse_next(stream)?;
    let checkpoint = stream.checkpoint();
    let selector = match next_token_text(stream).map(video_selector) {
        Ok(Some(selector)) => Some(selector),
        _ => {
            stream.reset(&checkpoint);
            None
        }
    };
    Ok((negate, ast::ParsedOperand::Reg(name), selector))
}

// Video instructions: the modifiers start with the operand types, and the
// operands carry selectors (%r1.b0, %r2.h10, ...), so they are parsed here
fn video<'a, 'input>(
    stream: &mut PtxParser<'a, 'input>,
    opcode: &'static str,
) -> PResult<ast::Instruction<ParsedOperandStr<'input>>> {
    let mut modifiers = Vec::new();
    loop {
        let checkpoint = stream.checkpoint();
        match next_token_text(stream) {
            Ok(text) if text.starts_with('.') => modifiers.push(text),
            _ => {
                stream.reset(&checkpoint);
                break;
            }
        }
    }
    let operands: Vec<_> = separated(3..=4, video_operand, Token::Comma).parse_next(stream)?;
    let mut errors = Vec::new();
    let types = modifiers
        .iter()
        .take_while(|m| matches!(**m, ".u32" | ".s32"))
        .map(|m| {
            if *m == ".u32" {
                ScalarType::U32
            } else {
                ScalarType::S32
            }
        })
        .collect::<Vec<_>>();
    let is_set = opcode.starts_with("vset");
    let type_count = if is_set { 2 } else { 3 };
    if types.len() != type_count {
        return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
    }
    let (dtype, atype, btype) = if is_set {
        (ScalarType::U32, types[0], types[1])
    } else {
        (types[0], types[1], types[2])
    };
    let mut saturate = false;
    let mut secondary = None;
    let mut clamp = None;
    let mut shift = 0;
    let mut plus_one = false;
    let mut compare = None;
    for m in &modifiers[type_count..] {
        match *m {
            ".sat" => saturate = true,
            ".add" => secondary = Some(ast::VideoSecondaryOp::Add),
            ".min" => secondary = Some(ast::VideoSecondaryOp::Min),
            ".max" => secondary = Some(ast::VideoSecondaryOp::Max),
            ".clamp" => clamp = Some(true),
            ".wrap" => clamp = Some(false),
            ".shr7" => shift = 7,
            ".shr15" => shift = 15,
            ".po" => plus_one = true,
            ".eq" => compare = Some(ast::VideoCompare::Eq),
            ".ne" => compare = Some(ast::VideoCompare::Ne),
            ".lt" => compare = Some(ast::VideoCompare::Lt),
            ".le" => compare = Some(ast::VideoCompare::Le),
            ".gt" => compare = Some(ast::VideoCompare::Gt),
            ".ge" => compare = Some(ast::VideoCompare::Ge),
            _ => return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify)),
        }
    }
    let base = opcode.trim_end_matches(['2', '4']);
    let lanes = match opcode.as_bytes()[opcode.len() - 1] {
        b'2' => 2,
        b'4' => 4,
        _ => 1,
    };
    let op = match base {
        "vadd" => ast::VideoOp::Add,
        "vsub" => ast::VideoOp::Sub,
        "vabsdiff" => ast::VideoOp::Absdiff,
        "vmin" => ast::VideoOp::Min,
        "vmax" => ast::VideoOp::Max,
        "vshl" => ast::VideoOp::Shl,
        "vshr" => ast::VideoOp::Shr,
        "vavrg" => ast::VideoOp::Avrg,
        "vset" => match compare {
            Some(cmp) => ast::VideoOp::Set(cmp),
            None => return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify)),
        },
        "vmad" => ast::VideoOp::Add,
        _ => return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify)),
    };
    if compare.is_some() && !is_set
        || clamp.is_some() && !matches!(op, ast::VideoOp::Shl | ast::VideoOp::Shr)
        || (shift != 0 || plus_one) && base != "vmad"
    {
        return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
    }
    let mut operands = operands.into_iter();
    let (dst_neg, dst, dsel) = operands.next().unwrap();
    let (neg_a, src1, asel) = operands.next().unwrap();
    let (neg_b, src2, bsel) = operands.next().unwrap();
    let src3 = operands.next();
    let any_negation = dst_neg || neg_a || neg_b || src3.as_ref().map_or(false, |c| c.0);
    if src3.as_ref().map_or(false, |c| c.2.is_some()) || (any_negation && base != "vmad") {
        errors.push(PtxError::SyntaxError(format!("invalid operands of {opcode}")));
    }
    let check_scalar = |sel: &Option<ast::VideoSelector>, errors: &mut Vec<PtxError>| {
        if let Some(sel) = sel {
            if sel.len != 1 || sel.digits[0] >= if sel.half { 2 } else { 4 } {
                errors.push(PtxError::SyntaxError(format!("invalid selector of {opcode}")));
            }
        }
    };
    let instruction = if base == "vmad" {
        check_scalar(&asel, &mut errors);
        check_scalar(&bsel, &mut errors);
        let Some((neg_c, src3, _)) = src3 else {
            return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
        };
        if dsel.is_some() || secondary.is_some() || (plus_one && (neg_a || neg_b || neg_c)) {
            errors.push(PtxError::SyntaxError("invalid vmad".to_string()));
        }
        ast::Instruction::Vmad {
            data: ast::VmadDetails {
                dtype,
                atype,
                btype,
                saturate,
                shift,
                plus_one,
                negate_a: neg_a,
                negate_b: neg_b,
                negate_c: neg_c,
                asel,
                bsel,
            },
            arguments: VmadArgs {
                dst,
                src1,
                src2,
                src3,
            },
        }
    } else if base == "vshr"
        && types == [ScalarType::U32; 3]
        && clamp != Some(false)
        && !saturate
        && secondary == Some(ast::VideoSecondaryOp::Add)
        && asel.is_none()
        && bsel.is_none()
        && dsel.is_none()
        && src3.is_some()
    {
        ast::Instruction::Vshr {
            data: ast::VshData {
                mode: FunnelShiftMode::Clamp,
                op2: VshOp::Add,
            },
            arguments: VshrArgs {
                dst,
                src1,
                src2,
                src3: src3.unwrap().1,
            },
        }
    } else if lanes == 1 {
        check_scalar(&asel, &mut errors);
        check_scalar(&bsel, &mut errors);
        check_scalar(&dsel, &mut errors);
        let src3 = src3.map(|c| c.1);
        if (dsel.is_some() || secondary.is_some()) != src3.is_some()
            || (dsel.is_some() && secondary.is_some())
        {
            errors.push(PtxError::SyntaxError(format!("invalid operands of {opcode}")));
        }
        if matches!(op, ast::VideoOp::Shl | ast::VideoOp::Shr) && btype != ScalarType::U32 {
            errors.push(PtxError::SyntaxError(format!("{opcode} needs .u32 b")));
        }
        ast::Instruction::VideoScalar {
            data: ast::VideoScalarDetails {
                op,
                dtype,
                atype,
                btype,
                saturate: saturate && !is_set,
                clamp: clamp.unwrap_or(true),
                asel,
                bsel,
                dsel,
                secondary,
            },
            arguments: VideoScalarArgs {
                dst,
                src1,
                src2,
                src3,
            },
        }
    } else {
        let Some((_, src3, _)) = src3 else {
            return Err(ErrMode::from_error_kind(stream, ErrorKind::Verify));
        };
        let element_count = if lanes == 2 { 4 } else { 8 };
        let check = |sel: &Option<ast::VideoSelector>, is_mask: bool, errors: &mut Vec<PtxError>| {
            if let Some(sel) = sel {
                let valid = sel.half == (lanes == 2)
                    && if is_mask {
                        sel.digits().iter().all(|d| *d < lanes)
                    } else {
                        sel.len == lanes && sel.digits().iter().all(|d| *d < element_count)
                    };
                if !valid {
                    errors.push(PtxError::SyntaxError(format!("invalid selector of {opcode}")));
                }
            }
        };
        check(&asel, false, &mut errors);
        check(&bsel, false, &mut errors);
        check(&dsel, true, &mut errors);
        if matches!(secondary, Some(ast::VideoSecondaryOp::Min | ast::VideoSecondaryOp::Max)) {
            errors.push(PtxError::SyntaxError(format!("invalid secondary operation of {opcode}")));
        }
        ast::Instruction::VideoSimd {
            data: ast::VideoSimdDetails {
                op,
                lanes,
                dtype,
                atype,
                btype,
                saturate: saturate && !is_set,
                add: secondary.is_some(),
                mask: dsel,
                asel,
                bsel,
            },
            arguments: VideoSimdArgs {
                dst,
                src1,
                src2,
                src3,
            },
        }
    };
    stream.state.errors.extend(errors);
    Ok(instruction)
}

type ParsedOperandStr<'input> = ast::ParsedOperand<&'input str>;

#[derive(Clone, PartialEq, Default, Debug, Display)]
#[display("({}:{})", _0.start, _0.end)]
pub struct TokenError(std::ops::Range<usize>);

impl std::error::Error for TokenError {}

fn first_optional<
    'a,
    'input,
    Input: Stream,
    OptionalOutput,
    RequiredOutput,
    Error,
    ParseOptional,
    ParseRequired,
>(
    mut optional: ParseOptional,
    mut required: ParseRequired,
) -> impl Parser<Input, (Option<OptionalOutput>, RequiredOutput), Error>
where
    ParseOptional: Parser<Input, OptionalOutput, Error>,
    ParseRequired: Parser<Input, RequiredOutput, Error>,
    Error: ParserError<Input>,
{
    trace("first_optional", move |input: &mut Input| -> Result<(Option<OptionalOutput>, RequiredOutput), ErrMode<Error>> {
        let start = input.checkpoint();

        let parsed_optional = match optional.parse_next(input) {
            Ok(v) => Some(v),
            Err(ErrMode::Backtrack(_)) => {
                input.reset(&start);
                None
            }
            Err(e) => return Err(e),
        };

        match required.parse_next(input) {
            Ok(v) => return Ok((parsed_optional, v)),
            Err(ErrMode::Backtrack(_)) => input.reset(&start),
            Err(e) => return Err(e),
        };

        Ok((None, required.parse_next(input)?))
    })
}

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
//   * Code block: => { <code expression> }. Code blocks implictly take all modifiers and arguments
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
        #[token("_")]
        Sink,
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
        F32Hex(&'input str),
        #[regex(r"0[dD][0-9a-zA-Z]{16}", |lex| lex.slice())]
        F64Hex(&'input str),
        #[regex(r"0[xX][0-9a-zA-Z]+U?", |lex| lex.slice())]
        Hex(&'input str),
        #[regex(r"0[bB][01]+U?", |lex| lex.slice())]
        Binary(&'input str),
        // with a fraction, or with only an exponent (1e20)
        #[regex(r"[0-9]*\.[0-9]+([eE][-+]?[0-9]+)?|[0-9]+[eE][-+]?[0-9]+", |lex| lex.slice())]
        F64(&'input str),
        #[regex(r"[0-9]+U?", |lex| lex.slice())]
        Decimal(&'input str),
        #[token("-")]
        Minus,
        #[token("+")]
        Plus,
        #[token("*")]
        Star,
        #[token("/")]
        Slash,
        #[token("%")]
        Percent,
        #[token("&")]
        Amp,
        #[token("^")]
        Caret,
        #[token("~")]
        Tilde,
        #[token("?")]
        Question,
        #[token("=")]
        Eq,
        #[token("WARP_SZ")]
        WarpSz,
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
        #[token(".reqnctapercluster")]
        DotReqnctapercluster,
        #[token(".explicitcluster")]
        DotExplicitcluster,
        #[token(".maxclusterrank")]
        DotMaxclusterrank,
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
        #[token(".section")]
        DotSection,
        #[token(".file")]
        DotFile,
        #[token(".ptr")]
        DotPtr,
        #[token(".noreturn")]
        DotNoreturn,
        #[token(".texref")]
        DotTexref,
        #[token(".surfref")]
        DotSurfref
    }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum StateSpace {
        #[display(".reg")]
        Reg,
        #[display("")]
        Generic,
    }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum MemScope { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum ScalarType {
        #[display(".e4m3x2")]
        E4m3x2,
        #[display(".e5m2x2")]
        E5m2x2
    }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum SetpBoolPostOp { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum AtomSemantics { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum Mul24Control { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum Reduction {
        Popc
    }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum ShuffleMode { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum ShiftDirection { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum FunnelShiftMode { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum VoteMode {
        Ballot
    }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum CacheLevel { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum EvictionPriority { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum ReportMechanism { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum VshOp {
        Add
    }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum BmskMode { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum PrmtMode { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum Tf32Rounding { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum Packed8Type { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum PackedSetType { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash)]
    pub enum TestpOperation { }

    #[derive(Copy, Clone, Display, PartialEq, Eq, Hash, Debug)]
    pub enum PackedCvtType { }


    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#data-movement-and-conversion-instructions-mov
    mov{.vec}.type  d, a => {
        Instruction::Mov {
            data: ast::MovDetails::new(vec, type_),
            arguments: MovArgs { dst: d, src: a },
        }
    }
    .vec: VectorPrefix = { .v2, .v4, .v8 };
    .type: ScalarType =  { .pred,
                           .b16, .b32, .b64, .b128,
                           .u16, .u32, .u64,
                           .s16, .s32, .s64,
                                 .f32, .f64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st
    st{.weak}{.ss}{.cop}{.level::eviction_priority}{.level::cache_hint}{.vec}.type  [a], b{, cache_policy} => {
        // st: the cache and prefetch hints do not change the result
        let _ = (&level_eviction_priority, &level_cache_hint, &cache_policy);
        Instruction::St {
            data: StData {
                qualifier: weak.unwrap_or(RawLdStQualifier::Weak).into(),
                state_space: shared_space(ss),
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
                state_space: shared_space(ss),
                caching: ast::StCacheOperator::Writeback,
                typ: ast::Type::maybe_vector(vec, type_)
            },
            arguments: StArgs { src1:a, src2:b }
        }
    }
    st.relaxed.scope{.ss}{.level::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache_policy} => {
        // st.relaxed: the cache and prefetch hints do not change the result
        let _ = (&level_eviction_priority, &level_cache_hint, &cache_policy);
        Instruction::St {
            data: StData {
                qualifier: ast::LdStQualifier::Relaxed(scope),
                state_space: shared_space(ss),
                caching: ast::StCacheOperator::Writeback,
                typ: ast::Type::maybe_vector(vec, type_)
            },
            arguments: StArgs { src1:a, src2:b }
        }
    }
    st.release.scope{.ss}{.level::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache_policy} => {
        // st.release: the cache and prefetch hints do not change the result
        let _ = (&level_eviction_priority, &level_cache_hint, &cache_policy);
        Instruction::St {
            data: StData {
                qualifier: ast::LdStQualifier::Release(scope),
                state_space: shared_space(ss),
                caching: ast::StCacheOperator::Writeback,
                typ: ast::Type::maybe_vector(vec, type_)
            },
            arguments: StArgs { src1:a, src2:b }
        }
    }
    st.mmio.relaxed.sys{.global}.type                                               [a], b => {
        // an MMIO access happens exactly once, neither merged nor split: as a relaxed system-scope access
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
    .vec: VectorPrefix =        { .v2, .v4, .v8 };
    .type: ScalarType =         { .b8, .b16, .b32, .b64, .b128,
                                  .u8, .u16, .u32, .u64,
                                  .s8, .s16, .s32, .s64,
                                  .f32, .f64 };
    RawLdStQualifier =          { .weak, .volatile };
    StateSpace =                { .global };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-ld
    ld{.weak}{.ss}{.cop}{.level::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type{.proxy}   d, [a]{.unified}{, cache_policy} => {
        let (a, unified) = a;
        // ld: the cache and prefetch hints (and .proxy::readonly) do not change the result
        let _ = (&level_eviction_priority, &level_cache_hint, &level_prefetch_size, &cache_policy, &unified, proxy);
        Instruction::Ld {
            data: LdDetails {
                qualifier: weak.unwrap_or(RawLdStQualifier::Weak).into(),
                state_space: shared_space(ss),
                caching: cop.unwrap_or(RawLdCacheOperator::Ca).into(),
                typ: ast::Type::maybe_vector(vec, type_),
                non_coherent: false
            },
            arguments: LdArgs { dst:d, src:a }
        }
    }
    ld.volatile{.ss}{.level::prefetch_size}{.vec}.type                                                      d, [a] => {
        // ld.volatile: the cache and prefetch hints do not change the result
        let _ = &level_prefetch_size;
        Instruction::Ld {
            data: LdDetails {
                qualifier: volatile.into(),
                state_space: shared_space(ss),
                caching: ast::LdCacheOperator::Cached,
                typ: ast::Type::maybe_vector(vec, type_),
                non_coherent: false
            },
            arguments: LdArgs { dst:d, src:a }
        }
    }
    ld.relaxed.scope{.ss}{.level::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{, cache_policy} => {
        // ld.relaxed: the cache and prefetch hints do not change the result
        let _ = (&level_eviction_priority, &level_cache_hint, &level_prefetch_size, &cache_policy);
        Instruction::Ld {
            data: LdDetails {
                qualifier: ast::LdStQualifier::Relaxed(scope),
                state_space: shared_space(ss),
                caching: ast::LdCacheOperator::Cached,
                typ: ast::Type::maybe_vector(vec, type_),
                non_coherent: false
            },
            arguments: LdArgs { dst:d, src:a }
        }
    }
    ld.acquire.scope{.ss}{.level::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{, cache_policy} => {
        // ld.acquire: the cache and prefetch hints do not change the result
        let _ = (&level_eviction_priority, &level_cache_hint, &level_prefetch_size, &cache_policy);
        Instruction::Ld {
            data: LdDetails {
                qualifier: ast::LdStQualifier::Acquire(scope),
                state_space: shared_space(ss),
                caching: ast::LdCacheOperator::Cached,
                typ: ast::Type::maybe_vector(vec, type_),
                non_coherent: false
            },
            arguments: LdArgs { dst:d, src:a }
        }
    }
    ld.mmio.relaxed.sys{.global}.type                                                                       d, [a] => {
        // an MMIO access happens exactly once, neither merged nor split: as a relaxed system-scope access
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
    .proxy =                                { .proxy::readonly };
    .scope: MemScope =                      { .cta, .cluster, .gpu, .sys };
    .vec: VectorPrefix =                    { .v2, .v4, .v8 };
    .type: ScalarType =                     { .b8, .b16, .b32, .b64, .b128,
                                              .u8, .u16, .u32, .u64,
                                              .s8, .s16, .s32, .s64,
                                              .f32, .f64 };
    RawLdStQualifier =                      { .weak, .volatile };
    StateSpace =                            { .global };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-ldu
    // A load of data that is the same for all threads; read as a normal load
    ldu{.ss}{.vec}.type  d, [a] => {
        Instruction::Ld {
            data: LdDetails {
                qualifier: ast::LdStQualifier::Weak,
                state_space: shared_space(ss),
                caching: ast::LdCacheOperator::Cached,
                typ: Type::maybe_vector(vec, type_),
                non_coherent: false
            },
            arguments: LdArgs { dst:d, src:a }
        }
    }
    .ss: StateSpace =                       { .global };
    .vec: VectorPrefix  =                   { .v2, .v4 };
    .type: ScalarType =                     { .b8, .b16, .b32, .b64, .b128,
                                              .u8, .u16, .u32, .u64,
                                              .s8, .s16, .s32, .s64,
                                              .f32, .f64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#data-movement-and-conversion-instructions-ld-global-nc
    ld.global{.cop}.nc{.level::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{, cache_policy} => {
        if let Some(cop) = cop {
            if let Some(level_eviction_priority) = level_eviction_priority {
                state.errors.push(PtxError::SyntaxError(format!("cannot have both {} and {} in {:?}", cop, level_eviction_priority, state.text)));
            }
        }
        // ld.global.nc: the cache and prefetch hints do not change the result
        let _ = (&level_eviction_priority, &level_cache_hint, &level_prefetch_size, &cache_policy);
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
    .vec: VectorPrefix  =                   { .v2, .v4, .v8 };
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
    // two f32 at once in a 64-bit register (sm_100)
    add{.rnd}{.ftz}{.sat}.f32x2  d, a, b => {
        ast::Instruction::Add {
            data: ast::ArithDetails::Float(ast::ArithFloat {
                    type_: f32x2,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none(),
                    relu: false,
                }),
            arguments: AddArgs { dst: d, src1: a, src2: b }
        }
    }
    // with .atype, the mixed-precision form (sm_100): a is converted to f32
    add{.rnd}{.ftz}{.sat}.f32{.atype}  d, a, b => {
        if let Some(atype) = atype {
            ast::Instruction::MixedFloat {
                data: ast::MixedFloatDetails {
                    op: ast::MixedFloatOp::Add,
                    dtype: f32,
                    atype,
                    btype: f32,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: ftz,
                    saturate: sat,
                },
                arguments: MixedFloatArgs { dst: d, src1: a, src2: b, src3: None }
            }
        } else {
            Instruction::Add {
                data: ast::ArithDetails::Float(
                    ast::ArithFloat {
                        type_: f32,
                        rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                        flush_to_zero: Some(ftz),
                        saturate: sat,
                        is_fusable: rnd.is_none(),
                        relu: false,
                    }
                ),
                arguments: AddArgs {
                    dst: d, src1: a, src2: b
                }
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
                    is_fusable: rnd.is_none(),
                    relu: false,
                }
            ),
            arguments: AddArgs {
                dst: d, src1: a, src2: b
            }
        }
    }
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
    .atype: ScalarType = { .f16, .bf16 };
    ScalarType =        { .f32, .f32x2, .f64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-add
    add{.rnd}{.ftz}{.sat}.f16   d, a, b => {
        Instruction::Add {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: f16,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none(),
                    relu: false,
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
                    is_fusable: rnd.is_none(),
                    relu: false,
                }
            ),
            arguments: AddArgs {
                dst: d, src1: a, src2: b
            }
        }
    }
    add{.rnd}{.ftz}{.sat}.bf16 d, a, b => {
        Instruction::Add {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: bf16,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none(),
                    relu: false,
                }
            ),
            arguments: AddArgs {
                dst: d, src1: a, src2: b
            }
        }
    }
    add{.rnd}{.ftz}{.sat}.bf16x2 d, a, b => {
        Instruction::Add {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: bf16x2,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none(),
                    relu: false,
                }
            ),
            arguments: AddArgs {
                dst: d, src1: a, src2: b
            }
        }
    }
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
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

    // two f32 at once in a 64-bit register (sm_100)
    mul{.rnd}{.ftz}{.sat}.f32x2  d, a, b => {
        ast::Instruction::Mul {
            data: ast::MulDetails::Float(ast::ArithFloat {
                    type_: f32x2,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none(),
                    relu: false,
                }),
            arguments: MulArgs { dst: d, src1: a, src2: b }
        }
    }
    mul{.rnd}{.ftz}{.sat}.f32  d, a, b => {
        ast::Instruction::Mul {
            data: ast::MulDetails::Float (
                ast::ArithFloat {
                    type_: f32,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none(),
                    relu: false,
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
                    is_fusable: rnd.is_none(),
                    relu: false,
                }
            ),
            arguments: MulArgs { dst: d, src1: a, src2: b }
        }
    }
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
    ScalarType = { .f32, .f32x2, .f64 };

    mul{.rnd}{.ftz}{.sat}.f16   d, a, b => {
        ast::Instruction::Mul {
            data: ast::MulDetails::Float (
                ast::ArithFloat {
                    type_: f16,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none(),
                    relu: false,
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
                    is_fusable: rnd.is_none(),
                    relu: false,
                }
            ),
            arguments: MulArgs { dst: d, src1: a, src2: b }
        }
    }
    mul{.rnd}{.ftz}{.sat}.bf16 d, a, b => {
        ast::Instruction::Mul {
            data: ast::MulDetails::Float (
                ast::ArithFloat {
                    type_: bf16,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none(),
                    relu: false,
                }
            ),
            arguments: MulArgs { dst: d, src1: a, src2: b }
        }
    }
    mul{.rnd}{.ftz}{.sat}.bf16x2 d, a, b => {
        ast::Instruction::Mul {
            data: ast::MulDetails::Float (
                ast::ArithFloat {
                    type_: bf16x2,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none(),
                    relu: false,
                }
            ),
            arguments: MulArgs { dst: d, src1: a, src2: b }
        }
    }
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
    ScalarType = { .f16, .f16x2, .bf16, .bf16x2 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#comparison-and-selection-instructions-set
    // https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-comparison-instructions-set
    set.CmpOp{.ftz}.dtype.stype         d, a, b => {
        let base = ast::SetpData::try_parse(state, cmpop, ftz, stype);
        let data = ast::SetData {
            base, dtype
        };
        ast::Instruction::Set {
            data,
            arguments: SetArgs { dst: d, src1: a, src2: b }
        }
    }

    set.CmpOp.BoolOp{.ftz}.dtype.stype  d, a, b, {!}c => {
        let (negate_src3, c) = c;
        let base = ast::SetpData::try_parse(state, cmpop, ftz, stype);
        let base = ast::SetpBoolData {
            base,
            bool_op: boolop,
            negate_src3
        };
        let data = ast::SetBoolData {
            base, dtype
        };
        ast::Instruction::SetBool {
            data,
            arguments: SetBoolArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }

    .CmpOp: RawSetpCompareOp  = { .eq, .ne, .lt, .le, .gt, .ge,
                                  .lo, .ls, .hi, .hs, // signed
                                  .equ, .neu, .ltu, .leu, .gtu, .geu, .num, .nan }; // float-only
    .BoolOp: SetpBoolPostOp = { .and, .or, .xor };
    // (the packed sources, .f16x2 and .bf16x2, take .u32, .s32 or the same packed type)
    .dtype: ScalarType = { .u16, .s16, .u32, .s32, .f16, .bf16, .f32, .f16x2, .bf16x2 };
    .stype: ScalarType = { .b16, .b32, .b64, .u16, .u32, .u64, .s16, .s32, .s64, .f16, .bf16, .f32, .f64,
                           .f16x2, .bf16x2 };


    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#comparison-and-selection-instructions-setp
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#half-precision-comparison-instructions-setp
    setp.CmpOp{.ftz}.type         p[|q], a, b => {
        let data = ast::SetpData::try_parse(state, cmpop, ftz, type_);
        ast::Instruction::Setp {
            data,
            arguments: SetpArgs { dst1: sink_to_register(p), dst2: q.map(sink_to_register), src1: a, src2: b }
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
            arguments: SetpBoolArgs { dst1: sink_to_register(p), dst2: q.map(sink_to_register), src1: a, src2: b, src3: c }
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

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#logic-and-shift-instructions-cnot
    cnot.type d, a => {
        ast::Instruction::Cnot {
            data: type_,
            arguments: CnotArgs { dst: d, src: a }
        }
    }
    .type: ScalarType = { .b16, .b32, .b64 };

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
        let data = ast::CvtDetails::new(&mut state.errors, ifrnd, ftz, sat, false, false, dtype, atype);
        let arguments = ast::CvtArgs { dst: d, src: a, src2: None };
        ast::Instruction::Cvt {
            data, arguments
        }
    }
    // Floating point conversions with .relu or .satfinite (at least one of them)
    cvt{.frnd}.relu{.satfinite}.fdtype.fatype   d, a => {
        let data = ast::CvtDetails::new(&mut state.errors, frnd, false, false, true, satfinite, fdtype, fatype);
        ast::Instruction::Cvt {
            data,
            arguments: ast::CvtArgs { dst: d, src: a, src2: None }
        }
    }
    cvt{.frnd}.satfinite.fdtype.fatype          d, a => {
        let data = ast::CvtDetails::new(&mut state.errors, frnd, false, false, false, true, fdtype, fatype);
        ast::Instruction::Cvt {
            data,
            arguments: ast::CvtArgs { dst: d, src: a, src2: None }
        }
    }
    // cvt.frnd2{.relu}{.satfinite}.f16.f32       d, a;
    // cvt.frnd2{.relu}{.satfinite}.f16x2.f32     d, a, b;
    // cvt.frnd2{.relu}{.satfinite}.bf16.f32      d, a;
    // cvt.rna{.satfinite}.tf32.f32               d, a;
    // cvt.frnd2{.relu}{.satfinite}.tf32.f32       d, a;
    // cvt.rn.satfinite{.relu}.f8x2type.f16x2     d, a;
    /*
    cvt.rn{.relu}.f16x2.f8x2type              d, a => {
        if relu {
            state.errors.push(PtxError::Todo("cvt.rn.f16x2.f8x2 instruction with relu modifier".to_string()));
        }
        let data = ast::CvtDetails::new(&mut state.errors, Some(rn), false, false, ScalarType::F16x2, f8x2type);
        ast::Instruction::Cvt {
            data,
            arguments: ast::CvtArgs { dst: d, src: a, src2: None }
        }
    }
    */

    .ifrnd: RawRoundingMode =   { .rn,  .rz,  .rm,  .rp,  .rni, .rzi, .rmi, .rpi };
    .frnd: RawRoundingMode =    { .rn,  .rz,  .rm,  .rp };
    .fdtype: ScalarType =       { .bf16, .f16, .f32, .f64 };
    .fatype: ScalarType =       { .bf16, .f16, .f32, .f64 };
    RawRoundingMode =           { .rn };
    .dtype: ScalarType =        { .u8,   .u16, .u32, .u64,
                                  .s8,   .s16, .s32, .s64,
                                  .bf16, .f16, .f32, .f64 };
    .atype: ScalarType =        { .u8,   .u16, .u32, .u64,
                                  .s8,   .s16, .s32, .s64,
                                  .bf16, .f16, .f32, .f64 };

    cvt.tfrnd{.relu}{.satfinite}.tf32.f32         d, a => {
        ast::Instruction::CvtTf32 {
            data: ast::CvtTf32Details { rounding: tfrnd, relu, satfinite },
            arguments: CvtTf32Args { dst: d, src: a }
        }
    }
    .tfrnd: Tf32Rounding = { .rn, .rz, .rm, .rp, .rna };

    // Conversions with packed pairs: cvt.frnd2{.relu}{.satfinite}.{f16x2,bf16x2}.f32 d, a, b
    // and the conversions to and from the narrow formats (fp8, fp6, fp4, ue8m0). One
    // pattern, so that the parser can tell it from the others by the destination type
    cvt.prnd{.relu}{.satfinite}.pdst.psrc      d, a {, b} => {
        packed_cvt(&mut state.errors, prnd, relu, satfinite, pdst, psrc, d, a, b)
    }
    .prnd: RawRoundingMode =    { .rn, .rz, .rp };
    .pdst: PackedCvtType =      { .f16x2, .bf16x2, .e4m3x2, .e5m2x2, .e2m1x2, .e2m3x2, .e3m2x2, .ue8m0x2 };
    .psrc: PackedCvtType =      { .f32, .f16x2, .bf16x2, .e4m3x2, .e5m2x2, .e2m1x2, .e2m3x2, .e3m2x2, .ue8m0x2 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cvt-pack
    cvt.pack.sat.convertType.s32.b32        d, a, b, c => {
        ast::Instruction::CvtPack {
            data: converttype,
            arguments: ast::CvtPackArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }

    .convertType: ScalarType = { .u8, .s8 };

    // the 16-bit form: d = sat(a) << 16 | sat(b)
    cvt.pack.sat.packType.s32        d, a, b => {
        ast::Instruction::CvtPack {
            data: packtype,
            arguments: ast::CvtPackArgs { dst: d, src1: a, src2: b, src3: ParsedOperand::Imm(ast::ImmediateValue::U64(0)) }
        }
    }

    .packType: ScalarType = { .u16, .s16 };

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

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#stack-manipulation-instructions-alloca
    alloca.type  ptr, size{, align} => {
        let align = match align.map(|a| a.as_immediate()) {
            None => 8,
            Some(Some(ImmediateValue::U64(a))) if a.is_power_of_two() => a as u32,
            Some(Some(ImmediateValue::S64(a))) if a > 0 && (a as u64).is_power_of_two() => a as u32,
            Some(_) => {
                state.errors.push(PtxError::SyntaxError("alloca alignment must be a power of two".to_string()));
                8
            }
        };
        ast::Instruction::Alloca {
            data: ast::AllocaDetails { type_, align },
            arguments: AllocaArgs { dst: ptr, src: size }
        }
    }
    .type: ScalarType = { .u32, .u64 };
    // https://docs.nvidia.com/cuda/parallel-thread-execution/#stack-manipulation-instructions-stacksave
    stacksave.type  d => {
        ast::Instruction::StackSave {
            data: type_,
            arguments: StackSaveArgs { dst: d }
        }
    }
    .type: ScalarType = { .u32, .u64 };
    // https://docs.nvidia.com/cuda/parallel-thread-execution/#stack-manipulation-instructions-stackrestore
    stackrestore.type  a => {
        ast::Instruction::StackRestore {
            data: type_,
            arguments: StackRestoreArgs { src: a }
        }
    }
    .type: ScalarType = { .u32, .u64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-isspacep
    isspacep.space  p, a => {
        ast::Instruction::Isspacep {
            data: space,
            arguments: IsspacepArgs { dst: p, src: a }
        }
    }
    .space: StateSpace = { .const, .global, .local, .shared{::cta, ::cluster}, .param };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#data-movement-and-conversion-instructions-cvta
    cvta.space.size     p, a => {
        let size = match size {
            ScalarType::U32 => 32,
            ScalarType::U64 => 64,
            _ => unreachable!()
        };
        let data = ast::CvtaDetails {
            state_space: shared_space(Some(space)),
            direction: ast::CvtaDirection::ExplicitToGeneric,
            size,
        };
        let arguments = ast::CvtaArgs {
            dst: p, src: a
        };
        ast::Instruction::Cvta {
            data, arguments
        }
    }
    cvta.to.space.size  p, a => {
        let size = match size {
            ScalarType::U32 => 32,
            ScalarType::U64 => 64,
            _ => unreachable!()
        };
        let data = ast::CvtaDetails {
            state_space: shared_space(Some(space)),
            direction: ast::CvtaDirection::GenericToExplicit,
            size,
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
    .type: ScalarType = { .s16x2, .s16, .s32, .s64 };
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

    // without a rounding mode (is_fusable): mad.rn on sm_20 and later, the product truncated before the addition
    // on sm_1x
    mad{.ftz}{.sat}.f32      d, a, b, c => {
        ast::Instruction::Mad {
            data: ast::MadDetails::Float(
                ast::ArithFloat {
                    type_: f32,
                    rounding: ast::RoundingMode::NearestEven,
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: true,
                    relu: false,
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
                    is_fusable: false,
                    relu: false,
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
                    is_fusable: false,
                    relu: false,
                }
            ),
            arguments: MadArgs { dst: d, src1: a, src2: b, src3: c  }
        }
    }
    .rnd: RawRoundingMode   = { .rn, .rz, .rm, .rp };
    ScalarType =        { .f32, .f64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-fma
    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#half-precision-floating-point-instructions-fma
    // two f32 at once in a 64-bit register (sm_100)
    fma.rnd{.ftz}{.sat}.f32x2  d, a, b, c => {
        ast::Instruction::Fma {
            data: ast::ArithFloat {
                    type_: f32x2,
                    rounding: rnd.into(),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: false,
                    relu: false,
                },
            arguments: FmaArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }
    // with .atype, the mixed-precision form (sm_100): a and b are converted to f32
    fma.rnd{.ftz}{.sat}.f32{.atype}  d, a, b, c => {
        if let Some(atype) = atype {
            ast::Instruction::MixedFloat {
                data: ast::MixedFloatDetails {
                    op: ast::MixedFloatOp::Fma,
                    dtype: f32,
                    atype,
                    btype: atype,
                    rounding: rnd.into(),
                    flush_to_zero: ftz,
                    saturate: sat,
                },
                arguments: MixedFloatArgs { dst: d, src1: a, src2: b, src3: Some(c) }
            }
        } else {
            ast::Instruction::Fma {
                data: ast::ArithFloat {
                    type_: f32,
                    rounding: rnd.into(),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: false,
                    relu: false,
                },
                arguments: FmaArgs { dst: d, src1: a, src2: b, src3: c  }
            }
        }
    }
    fma.rnd.f64              d, a, b, c => {
        ast::Instruction::Fma {
            data: ast::ArithFloat {
                type_: f64,
                rounding: rnd.into(),
                flush_to_zero: None,
                saturate: false,
                is_fusable: false,
                relu: false,
            },
            arguments: FmaArgs { dst: d, src1: a, src2: b, src3: c  }
        }
    }
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
    .atype: ScalarType =    { .f16, .bf16 };
    ScalarType =            { .f32, .f32x2, .f64 };

    fma.rnd{.ftz}{.sat}{.relu}.f16 d, a, b, c => {
        ast::Instruction::Fma {
            data: ast::ArithFloat {
                type_: f16,
                rounding: rnd.into(),
                flush_to_zero: Some(ftz),
                saturate: sat,
                is_fusable: false,
                relu,
            },
            arguments: FmaArgs { dst: d, src1: a, src2: b, src3: c  }
        }
    }
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
    ScalarType =            { .f16 };
    fma.rnd{.ftz}{.sat}{.relu}.type_x2       d, a, b, c => {
        ast::Instruction::Fma {
            data: ast::ArithFloat {
                type_: type_x2,
                rounding: rnd.into(),
                // bf16's .ftz is applied explicitly (insert_post_saturation), its denormal mode is not the f16 one
                flush_to_zero: if type_x2 == ScalarType::F16x2 || ftz { Some(ftz) } else { None },
                saturate: sat,
                is_fusable: false,
                relu,
            },
            arguments: FmaArgs { dst: d, src1: a, src2: b, src3: c  }
        }
    }
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
    .type_x2: ScalarType =            { .bf16x2, .f16x2, .bf16 };
    //fma.rnd.oob.{relu}.type     d, a, b, c;

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
                          .s16, .s32, .s64,
                          .u16x2, .s16x2 };
    ScalarType = { .s32 };

    // two f32 at once in a 64-bit register (sm_100)
    sub{.rnd}{.ftz}{.sat}.f32x2  d, a, b => {
        ast::Instruction::Sub {
            data: ast::ArithDetails::Float(ast::ArithFloat {
                    type_: f32x2,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none(),
                    relu: false,
                }),
            arguments: SubArgs { dst: d, src1: a, src2: b }
        }
    }
    // with .atype, the mixed-precision form (sm_100): a is converted to f32
    sub{.rnd}{.ftz}{.sat}.f32{.atype}  d, a, b => {
        if let Some(atype) = atype {
            ast::Instruction::MixedFloat {
                data: ast::MixedFloatDetails {
                    op: ast::MixedFloatOp::Sub,
                    dtype: f32,
                    atype,
                    btype: f32,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: ftz,
                    saturate: sat,
                },
                arguments: MixedFloatArgs { dst: d, src1: a, src2: b, src3: None }
            }
        } else {
            ast::Instruction::Sub {
                data: ast::ArithDetails::Float(
                    ast::ArithFloat {
                        type_: f32,
                        rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                        flush_to_zero: Some(ftz),
                        saturate: sat,
                        is_fusable: rnd.is_none(),
                        relu: false,
                    }
                ),
                arguments: SubArgs { dst: d, src1: a, src2: b  }
            }
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
                    is_fusable: rnd.is_none(),
                    relu: false,
                }
            ),
            arguments: SubArgs { dst: d, src1: a, src2: b  }
        }
    }
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
    .atype: ScalarType = { .f16, .bf16 };
    ScalarType = { .f32, .f32x2, .f64 };

    sub{.rnd}{.ftz}{.sat}.f16   d, a, b => {
        ast::Instruction::Sub {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: f16,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none(),
                    relu: false,
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
                    is_fusable: rnd.is_none(),
                    relu: false,
                }
            ),
            arguments: SubArgs { dst: d, src1: a, src2: b  }
        }
    }
    sub{.rnd}{.ftz}{.sat}.bf16 d, a, b => {
        ast::Instruction::Sub {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: bf16,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none(),
                    relu: false,
                }
            ),
            arguments: SubArgs { dst: d, src1: a, src2: b  }
        }
    }
    sub{.rnd}{.ftz}{.sat}.bf16x2 d, a, b => {
        ast::Instruction::Sub {
            data: ast::ArithDetails::Float(
                ast::ArithFloat {
                    type_: bf16x2,
                    rounding: rnd.map(Into::into).unwrap_or(ast::RoundingMode::NearestEven),
                    flush_to_zero: Some(ftz),
                    saturate: sat,
                    is_fusable: rnd.is_none(),
                    relu: false,
                }
            ),
            arguments: SubArgs { dst: d, src1: a, src2: b  }
        }
    }
    .rnd: RawRoundingMode = { .rn, .rz, .rm, .rp };
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
            arguments: MinArgs { dst: d, src1: a, src2: b, src3: None }
        }
    }
    min{.relu}.btype  d, a, b => {
        ast::Instruction::Min {
            data: if relu { ast::MinMaxDetails::SignedRelu(btype) } else { ast::MinMaxDetails::Signed(btype) },
            arguments: MinArgs { dst: d, src1: a, src2: b, src3: None }
        }
    }
    .atype: ScalarType = { .u16, .u32, .u64,
                           .u16x2, .s16, .s64 };
    .btype: ScalarType = { .s16x2, .s32 };

    //min{.ftz}{.NaN}{.xorsign.abs}.f32  d, a, b;
    // the third input is sm_100's three-input form
    min{.ftz}{.NaN}{.xorsign}{.abs}.f32   d, a, b{, c} => {
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: Some(ftz),
                    nan,
                    xorsign_abs: xorsign_abs(state, xorsign, abs),
                    abs: abs && !xorsign,
                    type_: f32
                }
            ),
            arguments: MinArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }
    min{.NaN}{.xorsign}{.abs}.f64   d, a, b => {
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: None,
                    nan,
                    xorsign_abs: xorsign_abs(state, xorsign, abs),
                    abs: abs && !xorsign,
                    type_: f64
                }
            ),
            arguments: MinArgs { dst: d, src1: a, src2: b, src3: None }
        }
    }
    ScalarType = { .f32, .f64 };

    //min{.ftz}{.NaN}{.xorsign.abs}.f16      d, a, b;
    //min{.ftz}{.NaN}{.xorsign.abs}.f16x2    d, a, b;
    //min{.NaN}{.xorsign.abs}.bf16           d, a, b;
    //min{.NaN}{.xorsign.abs}.bf16x2         d, a, b;
    min{.ftz}{.NaN}{.xorsign}{.abs}.f16   d, a, b{, c} => {
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: Some(ftz),
                    nan,
                    xorsign_abs: xorsign_abs(state, xorsign, abs),
                    abs: abs && !xorsign,
                    type_: f16
                }
            ),
            arguments: MinArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }
    min{.ftz}{.NaN}{.xorsign}{.abs}.f16x2   d, a, b{, c} => {
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: Some(ftz),
                    nan,
                    xorsign_abs: xorsign_abs(state, xorsign, abs),
                    abs: abs && !xorsign,
                    type_: f16x2
                }
            ),
            arguments: MinArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }
    min{.NaN}{.xorsign}{.abs}.bf16   d, a, b{, c} => {
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: None,
                    nan,
                    xorsign_abs: xorsign_abs(state, xorsign, abs),
                    abs: abs && !xorsign,
                    type_: bf16
                }
            ),
            arguments: MinArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }
    min{.NaN}{.xorsign}{.abs}.bf16x2   d, a, b{, c} => {
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: None,
                    nan,
                    xorsign_abs: xorsign_abs(state, xorsign, abs),
                    abs: abs && !xorsign,
                    type_: bf16x2
                }
            ),
            arguments: MinArgs { dst: d, src1: a, src2: b, src3: c }
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
            arguments: MaxArgs { dst: d, src1: a, src2: b, src3: None }
        }
    }
    max{.relu}.btype  d, a, b => {
        ast::Instruction::Max {
            data: if relu { ast::MinMaxDetails::SignedRelu(btype) } else { ast::MinMaxDetails::Signed(btype) },
            arguments: MaxArgs { dst: d, src1: a, src2: b, src3: None }
        }
    }
    .atype: ScalarType = { .u16, .u32, .u64,
                           .u16x2, .s16, .s64 };
    .btype: ScalarType = { .s16x2, .s32 };

    //max{.ftz}{.NaN}{.xorsign.abs}.f32  d, a, b;
    // the third input is sm_100's three-input form
    max{.ftz}{.NaN}{.xorsign}{.abs}.f32   d, a, b{, c} => {
        ast::Instruction::Max {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: Some(ftz),
                    nan,
                    xorsign_abs: xorsign_abs(state, xorsign, abs),
                    abs: abs && !xorsign,
                    type_: f32
                }
            ),
            arguments: MaxArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }
    max{.NaN}{.xorsign}{.abs}.f64   d, a, b => {
        ast::Instruction::Max {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: None,
                    nan,
                    xorsign_abs: xorsign_abs(state, xorsign, abs),
                    abs: abs && !xorsign,
                    type_: f64
                }
            ),
            arguments: MaxArgs { dst: d, src1: a, src2: b, src3: None }
        }
    }
    ScalarType = { .f32, .f64 };

    //max{.ftz}{.NaN}{.xorsign.abs}.f16      d, a, b;
    //max{.ftz}{.NaN}{.xorsign.abs}.f16x2    d, a, b;
    //max{.NaN}{.xorsign.abs}.bf16           d, a, b;
    //max{.NaN}{.xorsign.abs}.bf16x2         d, a, b;
    max{.ftz}{.NaN}{.xorsign}{.abs}.f16   d, a, b{, c} => {
        ast::Instruction::Max {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: Some(ftz),
                    nan,
                    xorsign_abs: xorsign_abs(state, xorsign, abs),
                    abs: abs && !xorsign,
                    type_: f16
                }
            ),
            arguments: MaxArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }
    max{.ftz}{.NaN}{.xorsign}{.abs}.f16x2   d, a, b{, c} => {
        ast::Instruction::Max {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: Some(ftz),
                    nan,
                    xorsign_abs: xorsign_abs(state, xorsign, abs),
                    abs: abs && !xorsign,
                    type_: f16x2
                }
            ),
            arguments: MaxArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }
    max{.NaN}{.xorsign}{.abs}.bf16   d, a, b{, c} => {
        ast::Instruction::Max {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: None,
                    nan,
                    xorsign_abs: xorsign_abs(state, xorsign, abs),
                    abs: abs && !xorsign,
                    type_: bf16
                }
            ),
            arguments: MaxArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }
    max{.NaN}{.xorsign}{.abs}.bf16x2   d, a, b{, c} => {
        ast::Instruction::Max {
            data: ast::MinMaxDetails::Float(
                MinMaxFloat {
                    flush_to_zero: None,
                    nan,
                    xorsign_abs: xorsign_abs(state, xorsign, abs),
                    abs: abs && !xorsign,
                    type_: bf16x2
                }
            ),
            arguments: MaxArgs { dst: d, src1: a, src2: b, src3: c }
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
                type_,
                saturate: false,
            },
            arguments: RcpArgs { dst: d, src: a  }
        }
    }
    rcp.rnd{.ftz}.f32       d, a => {
        ast::Instruction::Rcp {
            data: ast::RcpData {
                kind: ast::RcpKind::Compliant(rnd.into()),
                flush_to_zero: Some(ftz),
                type_: f32,
                saturate: false,
            },
            arguments: RcpArgs { dst: d, src: a  }
        }
    }
    rcp.rnd.f64             d, a => {
        ast::Instruction::Rcp {
            data: ast::RcpData {
                kind: ast::RcpKind::Compliant(rnd.into()),
                flush_to_zero: None,
                type_: f64,
                saturate: false,
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
                type_: f32,
                saturate: false,
            },
            arguments: SqrtArgs { dst: d, src: a  }
        }
    }
    sqrt.rnd{.ftz}{.sat}.f32     d, a => {
        ast::Instruction::Sqrt {
            data: ast::RcpData {
                kind: ast::RcpKind::Compliant(rnd.into()),
                flush_to_zero: Some(ftz),
                type_: f32,
                saturate: sat,
            },
            arguments: SqrtArgs { dst: d, src: a  }
        }
    }
    sqrt.rnd.f64           d, a => {
        ast::Instruction::Sqrt {
            data: ast::RcpData {
                kind: ast::RcpKind::Compliant(rnd.into()),
                flush_to_zero: None,
                type_: f64,
                saturate: false,
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
            data: ast::BarData {
                aligned,
            },
            arguments: BarArgs { src1: a, src2: b }
        }
    }
    //barrier{.cta}.arrive{.aligned}    a, b;
    //barrier{.cta}.red.popc{.aligned}.u32  d, a{, b}, {!}c;
    //barrier{.cta}.red.op{.aligned}.pred   p, a{, b}, {!}c;
    bar{.cta}.sync                  a{, b} => {
        let _ = cta;
        ast::Instruction::Bar {
            data: ast::BarData {
                aligned: true,
            },
            arguments: BarArgs { src1: a, src2: b }
        }
    }
    //bar{.cta}.arrive    a, b;
    bar{.cta}.red.popc.u32  d, a{, b}, {!}c => {
        let _ = cta;
        let (negate_src3, c) = c;
        ast::Instruction::BarRed {
            data: ast::BarRedData {
                aligned: true,
                pred_reduction: Reduction::Popc,
            },
            arguments: BarRedArgs {
                dst1: d,
                src_barrier: a,
                src_threadcount: b,
                src_predicate: c,
                src_negate_predicate: ParsedOperand::Imm(ImmediateValue::U64(negate_src3 as u64))
            }
        }
    }
    bar{.cta}.red.op.pred   p, a{, b}, {!}c => {
        let _ = cta;
        let (negate_src3, c) = c;
        ast::Instruction::BarRed {
            data: ast::BarRedData {
                aligned: true,
                pred_reduction: op,
            },
            arguments: BarRedArgs {
                dst1: p,
                src_barrier: a,
                src_threadcount: b,
                src_predicate: c,
                src_negate_predicate: ParsedOperand::Imm(ImmediateValue::U64(negate_src3 as u64))
            }
        }
    }
    .op: Reduction = { .and, .or };

    bar.warp.sync membermask => {
        ast::Instruction::BarWarp {
            data: (),
            arguments: BarWarpArgs { src: membermask }
        }
    }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#parallel-synchronization-and-communication-instructions-atom
    atom{.sem}{.scope}{.space}.op{.level::cache_hint}.type                                      d, [a], b{, cache_policy} => {
        if level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo("atom instruction with cache policy/cache hints".to_string()));
        }
        ast::Instruction::Atom {
            data: AtomDetails {
                noftz: false,
                semantics: sem.map(Into::into).unwrap_or(AtomSemantics::Relaxed),
                scope: scope.unwrap_or(MemScope::Gpu),
                space: shared_space(space),
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
                space: shared_space(space),
                type_: cas_type
            },
            arguments: AtomCasArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }
    atom{.sem}{.scope}{.space}.exch{.level::cache_hint}.b128                                    d, [a], b{, cache_policy} => {
        if level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo("atom.exch.b128 instruction with cache policy/cache hints".to_string()));
        }
        ast::Instruction::Atom {
            data: AtomDetails {
                noftz: false,
                semantics: sem.map(Into::into).unwrap_or(AtomSemantics::Relaxed),
                scope: scope.unwrap_or(MemScope::Gpu),
                space: shared_space(space),
                op: ast::AtomicOp::new(exch, b128.kind()),
                type_: b128.into()
            },
            arguments: AtomArgs { dst: d, src1: a, src2: b }
        }
    }
    atom{.sem}{.scope}{.global}.float_op{.level::cache_hint}.vec_32_bit.f32                     d, [a], b{, cache_policy} => {
        if level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo("atom float operation on f32 with cache policy/cache hints".to_string()));
        }
        ast::Instruction::Atom {
            data: AtomDetails {
                noftz: false,
                semantics: sem.map(Into::into).unwrap_or(AtomSemantics::Relaxed),
                scope: scope.unwrap_or(MemScope::Gpu),
                space: global.unwrap_or(StateSpace::Generic),
                op: ast::AtomicOp::new(float_op, f32.kind()),
                type_: ast::Type::Vector(vec_32_bit.len().get(), f32)
            },
            arguments: AtomArgs { dst: d, src1: a, src2: b }
        }
    }
    atom{.sem}{.scope}{.global}.float_op{.noftz}{.level::cache_hint}{.vec_16_bit}.half_word_type  d, [a], b{, cache_policy} => {
        let _ = noftz;
        if level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo("atom float operation on f16/bf16 with cache policy/cache hints".to_string()));
        }
        ast::Instruction::Atom {
            data: AtomDetails {
                noftz: false,
                semantics: sem.map(Into::into).unwrap_or(AtomSemantics::Relaxed),
                scope: scope.unwrap_or(MemScope::Gpu),
                space: global.unwrap_or(StateSpace::Generic),
                op: ast::AtomicOp::new(float_op, half_word_type.kind()),
                type_: ast::Type::maybe_vector(vec_16_bit, half_word_type)
            },
            arguments: AtomArgs { dst: d, src1: a, src2: b }
        }
    }
    atom{.sem}{.scope}{.global}.float_op{.noftz}{.level::cache_hint}{.vec_32_bit}.packed_type     d, [a], b{, cache_policy} => {
        let _ = noftz;
        if level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo("atom float operation on packed types with cache policy/cache hints".to_string()));
        }
        ast::Instruction::Atom {
            data: AtomDetails {
                noftz: false,
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

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-red
    red{.sem}{.scope}{.space}.op{.level::cache_hint}.type                                      [a], b{, cache_policy} => {
        if level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo("red instruction with cache policy/cache hints".to_string()));
        }
        ast::Instruction::Red {
            data: AtomDetails {
                noftz: false,
                semantics: sem.map(Into::into).unwrap_or(AtomSemantics::Relaxed),
                scope: scope.unwrap_or(MemScope::Gpu),
                space: shared_space(space),
                op: ast::AtomicOp::new(op, type_.kind()),
                type_: type_.into()
            },
            arguments: RedArgs { src1: a, src2: b }
        }
    }
    red{.sem}{.scope}{.global}.float_op{.level::cache_hint}.vec_32_bit.f32                     [a], b{, cache_policy} => {
        if level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo("red float operation on f32 with cache policy/cache hints".to_string()));
        }
        ast::Instruction::Red {
            data: AtomDetails {
                noftz: false,
                semantics: sem.map(Into::into).unwrap_or(AtomSemantics::Relaxed),
                scope: scope.unwrap_or(MemScope::Gpu),
                space: global.unwrap_or(StateSpace::Generic),
                op: ast::AtomicOp::new(float_op, f32.kind()),
                type_: ast::Type::Vector(vec_32_bit.len().get(), f32)
            },
            arguments: RedArgs { src1: a, src2: b }
        }
    }
    red{.sem}{.scope}{.global}.float_op{.noftz}{.level::cache_hint}{.vec_16_bit}.half_word_type  [a], b{, cache_policy} => {
        let _ = noftz;
        if level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo("red float operation on f16/bf16 with cache policy/cache hints".to_string()));
        }
        ast::Instruction::Red {
            data: AtomDetails {
                noftz: false,
                semantics: sem.map(Into::into).unwrap_or(AtomSemantics::Relaxed),
                scope: scope.unwrap_or(MemScope::Gpu),
                space: global.unwrap_or(StateSpace::Generic),
                op: ast::AtomicOp::new(float_op, half_word_type.kind()),
                type_: ast::Type::maybe_vector(vec_16_bit, half_word_type)
            },
            arguments: RedArgs { src1: a, src2: b }
        }
    }
    red{.sem}{.scope}{.global}.float_op{.noftz}{.level::cache_hint}{.vec_32_bit}.packed_type     [a], b{, cache_policy} => {
        let _ = noftz;
        if level_cache_hint || cache_policy.is_some() {
            state.errors.push(PtxError::Todo("red float operation on packed types with cache policy/cache hints".to_string()));
        }
        ast::Instruction::Red {
            data: AtomDetails {
                noftz: false,
                semantics: sem.map(Into::into).unwrap_or(AtomSemantics::Relaxed),
                scope: scope.unwrap_or(MemScope::Gpu),
                space: global.unwrap_or(StateSpace::Generic),
                op: ast::AtomicOp::new(float_op, packed_type.kind()),
                type_: ast::Type::maybe_vector(vec_32_bit, packed_type)
            },
            arguments: RedArgs { src1: a, src2: b }
        }
    }
    .space: StateSpace =            { .global, .shared{::cta, ::cluster} };
    .sem: AtomSemantics =           { .relaxed, .release };
    .scope: MemScope =              { .cta, .cluster, .gpu, .sys };
    .op: RawAtomicOp =              { .and, .or, .xor,
                                      .add, .inc, .dec,
                                      .min, .max };
    .level::cache_hint =            { .L2::cache_hint };
    .type: ScalarType =             { .b32, .b64, .u32, .u64, .s32, .s64, .f32, .f64 };
    .half_word_type: ScalarType =   { .f16, .bf16 };
    .packed_type: ScalarType =      { .f16x2, .bf16x2 };
    .vec_16_bit: VectorPrefix =     { .v2, .v4, .v8 };
    .vec_32_bit:  VectorPrefix =    { .v2, .v4 };
    .float_op: RawAtomicOp =        { .add, .min, .max };
    ScalarType =                    { .f32 };
    StateSpace =                    { .global };

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
                kind: ast::DivFloatKind::Approx,
                saturate: false
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
                kind: ast::DivFloatKind::ApproxFull,
                saturate: false
            }),
            arguments: DivArgs {
                dst: d,
                src1: a,
                src2: b,
            },
        }
    }
    div.rnd{.ftz}{.sat}.f32     d, a, b => {
        ast::Instruction::Div {
            data: ast::DivDetails::Float(ast::DivFloatDetails{
                type_: f32,
                flush_to_zero: Some(ftz),
                kind: ast::DivFloatKind::Rounding(rnd.into()),
                saturate: sat,
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
                kind: ast::DivFloatKind::Rounding(rnd.into()),
                saturate: false
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
    .type: ScalarType = { .s16, .s32, .s64, .s16x2 };

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

    // Packed bytes (PTX ISA 9.2): add, sub, min, max, neg and abs on four byte lanes
    add{.sat}.p8type  d, a, b => {
        ast::Instruction::Packed8 {
            data: ast::Packed8Details { op: ast::Packed8Op::Add, signed: p8type == Packed8Type::S8x4, saturate: sat, relu: false },
            arguments: Packed8Args { dst: d, src1: a, src2: Some(b) }
        }
    }
    .p8type: Packed8Type = { .u8x4, .s8x4 };
    sub{.sat}.p8type  d, a, b => {
        ast::Instruction::Packed8 {
            data: ast::Packed8Details { op: ast::Packed8Op::Sub, signed: p8type == Packed8Type::S8x4, saturate: sat, relu: false },
            arguments: Packed8Args { dst: d, src1: a, src2: Some(b) }
        }
    }
    .p8type: Packed8Type = { .u8x4, .s8x4 };
    min{.relu}.p8type  d, a, b => {
        ast::Instruction::Packed8 {
            data: ast::Packed8Details { op: ast::Packed8Op::Min, signed: p8type == Packed8Type::S8x4, saturate: false, relu },
            arguments: Packed8Args { dst: d, src1: a, src2: Some(b) }
        }
    }
    .p8type: Packed8Type = { .u8x4, .s8x4 };
    max{.relu}.p8type  d, a, b => {
        ast::Instruction::Packed8 {
            data: ast::Packed8Details { op: ast::Packed8Op::Max, signed: p8type == Packed8Type::S8x4, saturate: false, relu },
            arguments: Packed8Args { dst: d, src1: a, src2: Some(b) }
        }
    }
    .p8type: Packed8Type = { .u8x4, .s8x4 };
    neg.p8type  d, a => {
        ast::Instruction::Packed8 {
            data: ast::Packed8Details { op: ast::Packed8Op::Neg, signed: p8type == Packed8Type::S8x4, saturate: false, relu: false },
            arguments: Packed8Args { dst: d, src1: a, src2: None }
        }
    }
    .p8type: Packed8Type = { .u8x4, .s8x4 };
    abs.p8type  d, a => {
        ast::Instruction::Packed8 {
            data: ast::Packed8Details { op: ast::Packed8Op::Abs, signed: p8type == Packed8Type::S8x4, saturate: false, relu: false },
            arguments: Packed8Args { dst: d, src1: a, src2: None }
        }
    }
    .p8type: Packed8Type = { .u8x4, .s8x4 };

    // set on packed integer lanes (PTX ISA 9.4)
    set.CmpOp.ptype  d, a, b => {
        use RawSetpCompareOp as C;
        let cmp = match cmpop {
            C::Eq => ast::PackedCmp::Eq,
            C::Ne => ast::PackedCmp::Ne,
            C::Lt | C::Lo => ast::PackedCmp::Lt,
            C::Le | C::Ls => ast::PackedCmp::Le,
            C::Gt | C::Hi => ast::PackedCmp::Gt,
            C::Ge | C::Hs => ast::PackedCmp::Ge,
            _ => {
                state.errors.push(PtxError::SyntaxError("set on packed lanes takes an integer comparison".to_string()));
                ast::PackedCmp::Eq
            }
        };
        let lanes16 = matches!(ptype, PackedSetType::U16x2 | PackedSetType::S16x2);
        ast::Instruction::Packed8 {
            data: ast::Packed8Details {
                op: ast::Packed8Op::Set { cmp, lanes16 },
                signed: matches!(ptype, PackedSetType::S8x4 | PackedSetType::S16x2),
                saturate: false,
                relu: false,
            },
            arguments: Packed8Args { dst: d, src1: a, src2: Some(b) }
        }
    }
    .CmpOp: RawSetpCompareOp  = { .eq, .ne, .lt, .le, .gt, .ge, .lo, .ls, .hi, .hs };
    .ptype: PackedSetType = { .u8x4, .s8x4, .u16x2, .s16x2 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-clmad
    clmad.mode.u64  d, a, b, c => {
        ast::Instruction::Clmad {
            data: mode == RawMulIntControl::Hi,
            arguments: ClmadArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }
    .mode: RawMulIntControl = { .lo, .hi };

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

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-bmsk
    bmsk.mode.b32  d, a, b => {
        ast::Instruction::Bmsk {
            data: mode,
            arguments: BmskArgs { dst: d, src_a: a, src_b: b },
        }
    }
    .mode: BmskMode = { .clamp, .wrap };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#logic-and-shift-instructions-lop3
    lop3.b32  d, a, b, c, lut => {
        ast::Instruction::Lop3 {
            data: None,
            arguments: Lop3Args { dst: d, dst_pred: None, src1: a, src2: b, src3: c, src_lut: lut, src_pred: None },
        }
    }
    // the predicate output form (PTX ISA 8.2): p = (d != 0) BoolOp q
    lop3.BoolOp.b32  d[|p], a, b, c, lut, q => {
        if p.is_none() {
            state.errors.push(PtxError::SyntaxError("lop3.BoolOp needs a predicate destination".to_string()));
        }
        ast::Instruction::Lop3 {
            data: Some(boolop),
            arguments: Lop3Args { dst: d, dst_pred: p, src1: a, src2: b, src3: c, src_lut: lut, src_pred: Some(q) },
        }
    }
    .BoolOp: SetpBoolPostOp = { .and, .or };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-szext
    szext.mode.type  d, a, b => {
        ast::Instruction::Szext {
            data: ast::SzextDetails { clamp: mode == BmskMode::Clamp, type_ },
            arguments: SzextArgs { dst: d, src_a: a, src_b: b },
        }
    }
    .mode: BmskMode = { .clamp, .wrap };
    .type: ScalarType = { .u32, .s32 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-fns
    fns.b32  d, mask, base, offset => {
        ast::Instruction::Fns {
            arguments: FnsArgs { dst: d, src_mask: mask, src_base: base, src_offset: offset },
        }
    }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#data-movement-and-conversion-instructions-prmt
    // prmt.b32{.mode}  d, a, b, c;
    // .mode = { .f4e, .b4e, .rc8, .ecl, .ecr, .rc16 };
    prmt.b32  d, a, b, c => {
        ast::Instruction::Prmt {
            data: None,
            arguments: PrmtArgs {
                dst: d, src1: a, src2: b, src3: c
            }
        }
    }
    prmt.b32.mode  d, a, b, c => {
        ast::Instruction::Prmt {
            data: Some(mode),
            arguments: PrmtArgs {
                dst: d, src1: a, src2: b, src3: c
            }
        }
    }
    .mode: PrmtMode = { .f4e, .b4e, .rc8, .ecl, .ecr, .rc16 };

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

    // Clusters have a single CTA: cluster scope is treated as GPU scope.
    fence{.sem}.scope => {
        let _ = sem;
        ast::Instruction::Membar {
            data: if scope == MemScope::Cluster { MemScope::Gpu } else { scope }
        }
    }
    // Orders a CTA's mbarrier.init before the other CTAs of the cluster use the object
    fence.mbarrier_init.release.cluster => {
        ast::Instruction::Membar { data: MemScope::Cta }
    }
    // Asynchronous copies are emulated with ordinary loads and stores (the generic proxy)
    fence.proxy.async{.space} => {
        let _ = space;
        ast::Instruction::Membar { data: MemScope::Cta }
    }
    .sem: RawFenceSemantics = { .sc, .acq_rel, .acquire, .release };
    .scope: MemScope = { .cta, .cluster, .gpu, .sys };
    .space: StateSpace = { .global, .shared{::cta, ::cluster} };

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

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#integer-arithmetic-instructions-mad24
    mad24.mode.type  d, a, b, c => {
        ast::Instruction::Mad24 {
            data: ast::Mad24Details { control: mode, type_, saturate: false },
            arguments: Mad24Args { dst: d, src1: a, src2: b, src3: c }
        }
    }
    mad24.hi.sat.s32  d, a, b, c => {
        ast::Instruction::Mad24 {
            data: ast::Mad24Details { control: Mul24Control::Hi, type_: s32, saturate: true },
            arguments: Mad24Args { dst: d, src1: a, src2: b, src3: c }
        }
    }
    .mode: Mul24Control = { .hi, .lo };
    .type: ScalarType = { .u32, .s32 };
    ScalarType = { .s32 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#comparison-and-selection-instructions-slct
    slct{.ftz}.dtype.ctype  d, a, b, c => {
        ast::Instruction::Slct {
            data: ast::SlctDetails { type_: dtype, ctype, flush_to_zero: ftz && ctype == ScalarType::F32 },
            arguments: SlctArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }
    .dtype: ScalarType = { .b16, .b32, .b64, .u16, .u32, .u64, .s16, .s32, .s64, .f32, .f64 };
    .ctype: ScalarType = { .s32, .f32 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#floating-point-instructions-testp
    testp.op.type  p, a => {
        let op = match op {
            TestpOperation::Finite => ast::TestpOp::Finite,
            TestpOperation::Infinite => ast::TestpOp::Infinite,
            TestpOperation::Number => ast::TestpOp::Number,
            TestpOperation::Notanumber => ast::TestpOp::NotANumber,
            TestpOperation::Normal => ast::TestpOp::Normal,
            TestpOperation::Subnormal => ast::TestpOp::Subnormal,
        };
        ast::Instruction::Testp {
            data: ast::TestpDetails { op, type_ },
            arguments: TestpArgs { dst: p, src: a }
        }
    }
    .op: TestpOperation = { .finite, .infinite, .number, .notanumber, .normal, .subnormal };
    .type: ScalarType = { .f32, .f64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-shfl-sync
    shfl.mode.b32  d[|p], a, b, c => {
        Instruction::Shfl  {
            data: ast::ShflDetails { mode },
            arguments: ShflArgs { dst: d, dst_pred: p, src: a, src_lane: b, src_opts: c, src_membermask: None }
        }
    }
    shfl.sync.mode.b32  d[|p], a, b, c, membermask => {
        Instruction::Shfl  {
            data: ast::ShflDetails { mode },
            arguments: ShflArgs { dst: d, dst_pred: p, src: a, src_lane: b, src_opts: c, src_membermask: Some(membermask) }
        }
    }
    .mode: ShuffleMode = { .up, .down, .bfly, .idx };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#miscellaneous-instructions-nanosleep
    nanosleep.u32 t => {
        Instruction::Nanosleep {
            arguments: NanosleepArgs { src: t }
        }
    }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-tanh
    // https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-tanh
    tanh.approx.type d, a => {
        Instruction::Tanh {
            data: type_,
            arguments: TanhArgs { dst: d, src: a }
        }
    }
    .type: ScalarType = { .f32, .f16, .f16x2, .bf16, .bf16x2 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async
    cp.async.cop.space.global{.level::cache_hint}{.level::prefetch_size}
                             [dst], [src], cp-size{, src-size}{, cache-policy} => {
        // TODO: handle cache hints and cache policies
        let _ = level_cache_hint;
        let _ = level_prefetch_size;
        let _ = cache_policy;

        let cp_size = cp_size
            .as_immediate()
            .and_then(|imm| imm.as_u64())
            .and_then(|n| CpAsyncCpSize::from_u64(n))
            .unwrap_or_else(|| {
                state.errors.push(PtxError::SyntaxError(format!("invalid cp.async cp-size {} in {:?}", cp_size, state.text)));
                CpAsyncCpSize::Bytes4
            });

        // with .L2::cache_hint, a single optional operand is the cache policy
        let src_size = if level_cache_hint && cache_policy.is_none() { None } else { src_size };
        let (src_size, src_size_reg) = match src_size {
            Some(op) => match op.as_immediate() {
                Some(imm) => (imm.as_u64(), None),
                None => (None, Some(op)),
            },
            None => (None, None),
        };

        Instruction::CpAsync {
            data: CpAsyncDetails {
                caching: cop.into(),
                space,
                cp_size,
                src_size,
                ignore_src: false,
            },
            arguments: CpAsyncArgs {
                src_to: dst,
                src_from: src,
                src_size: src_size_reg,
            }
        }
    }
    // cp.async.ca.shared{::cta}.global{.level::cache_hint}{.level::prefetch_size}
    //                          [dst], [src], cp-size{, ignore-src}{, cache-policy} ;
    // cp.async.cg.shared{::cta}.global{.level::cache_hint}{.level::prefetch_size}
    //                          [dst], [src], 16{, ignore-src}{, cache-policy} ;

    .level::cache_hint = { .L2::cache_hint };
    .level::prefetch_size: PrefetchSize =  { .L2::64B, .L2::128B, .L2::256B };
    // TODO: how to handle this?
    // cp-size =                { 4, 8, 16 }
    .space: StateSpace = { .shared{::cta} };
    .cop: RawCpAsyncCacheOperator = { .ca, .cg };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-commit-group
    // cp.async.bulk.commit_group too: copies complete when they are issued
    cp.async{.bulk}.commit_group => {
        let _ = bulk;
        Instruction::CpAsyncCommitGroup {}
    }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-wait-group
    // cp.async.bulk.wait_group{.read} too
    cp.async{.bulk}.wait_group{.read} n => {
        if read && !bulk {
            state.errors.push(PtxError::SyntaxError(format!("invalid cp.async.wait_group in {:?}", state.text)));
        }
        Instruction::CpAsyncWaitGroup {
            arguments: CpAsyncWaitGroupArgs { src_group: n },
        }
    }
    cp.async.wait_all => {
        Instruction::CpAsyncWaitAll {}
    }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-cp-async-mbarrier-arrive
    cp.async.mbarrier.arrive{.noinc}{.space}.b64 [addr] => {
        Instruction::CpAsyncMbarrierArrive {
            data: CpAsyncMbarrierArriveDetails { space: shared_space(space), noinc },
            arguments: CpAsyncMbarrierArriveArgs { src_mbarrier: addr }
        }
    }
    .space: StateSpace = { .shared{::cta} };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-async-bulk
    // cp.async.bulk.{shared::cta, shared::cluster}.global.mbarrier::complete_tx::bytes{.multicast::cluster}{.level::cache_hint}
    //                                      [dstmem], [srcmem], size, [mbar]{, ctamask}{, cache-policy}
    // cp.async.bulk.shared::cluster.shared::cta.mbarrier::complete_tx::bytes [dstmem], [srcmem], size, [mbar]
    // cp.async.bulk.global.shared::cta.bulk_group{.level::cache_hint}{.cp_mask}
    //                                      [dstmem], [srcmem], size{, cache-policy}{, bytemask}
    // The modifiers are unordered, so the direction follows from the completion mechanism.
    cp.async.bulk{.shared::cluster}{.shared::cta}{.global}{.completion_mechanism}{.report}{.bulk_group}{.multicast}{.level::cache_hint}{.cp_mask}
                                            [dstmem], [srcmem], size{, [mbar]}{, extra1}{, extra2} => {
        let mut extra = [extra1, extra2].into_iter().flatten();
        let to_shared = completion_mechanism && !bulk_group && global as u8 + shared_cta as u8 + shared_cluster as u8 == 2;
        let to_global = bulk_group && !completion_mechanism && global && shared_cta && !shared_cluster;
        let (dst_space, src_space) = if to_global {
            (StateSpace::Global, StateSpace::Shared)
        } else {
            (StateSpace::Shared, if global { StateSpace::Global } else { StateSpace::Shared })
        };
        let cta_mask = if multicast { extra.next() } else { None };
        let cache_policy = if level_cache_hint { extra.next() } else { None };
        let byte_mask = if cp_mask { extra.next() } else { None };
        let valid = (to_shared && mbar.is_some() && !cp_mask && (global || !(multicast || level_cache_hint)))
            || (to_global && mbar.is_none() && !multicast && cp_mask == byte_mask.is_some());
        if !valid || extra.next().is_some() || (multicast && cta_mask.is_none()) || (level_cache_hint && cache_policy.is_none()) {
            state.errors.push(PtxError::SyntaxError(format!("invalid cp.async.bulk in {:?}", state.text)));
        }
        Instruction::CpAsyncBulk {
            data: CpAsyncBulkDetails {
                dst_space,
                src_space,
                multicast,
                report: report.map_or(0, |r| match r {
                    ReportMechanism::MbarrierReportValidityPerElementFf => 1,
                    ReportMechanism::MbarrierReportValidityPer16bytes80000000 => 2,
                    ReportMechanism::MbarrierReportValidityPer16bytes8000 => 3,
                    ReportMechanism::MbarrierReportValidityPer16bytes80 => 4,
                    ReportMechanism::MbarrierReportValidityPer16bytes8 => 5,
                }),
            },
            arguments: CpAsyncBulkArgs {
                src_to: dstmem,
                src_from: srcmem,
                src_size: size,
                src_mbarrier: mbar,
                src_cta_mask: cta_mask,
                src_byte_mask: byte_mask,
                src_cache_policy: cache_policy,
            }
        }
    }
    .completion_mechanism = { .mbarrier::complete_tx::bytes };
    .report: ReportMechanism = { .mbarrier::report::validity::per_element::ff,
        .mbarrier::report::validity::per_16bytes::80000000, .mbarrier::report::validity::per_16bytes::8000,
        .mbarrier::report::validity::per_16bytes::80, .mbarrier::report::validity::per_16bytes::8 };
    .multicast = { .multicast::cluster };
    .level::cache_hint = { .L2::cache_hint };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-mbarrier
    // Clusters have a single CTA, so .shared::cluster addresses are those of .shared::cta.
    mbarrier.init{.layout}{.space}.b64 [addr], count => {
        Instruction::MbarrierInit {
            data: MbarrierInitDetails { space: shared_space(space), v1: layout },
            arguments: MbarrierInitArgs { src_mbarrier: addr, src_count: count }
        }
    }
    mbarrier.inval{.space}.b64 [addr] => {
        Instruction::MbarrierInval {
            data: shared_space(space),
            arguments: MbarrierInvalArgs { src_mbarrier: addr }
        }
    }
    // .multicast::cluster::32b: the operation on the mbarrier at the same offset of each CTA in the mask
    mbarrier.expect_tx{.sem}{.scope}{.space}{.multicast}.b64 [addr], txcount{, ctamask} => {
        let _ = (sem, scope, multicast);
        Instruction::MbarrierTx {
            data: MbarrierTxDetails { space: shared_space(space), complete: false },
            arguments: MbarrierTxArgs { src_mbarrier: addr, src_tx_count: txcount, src_cta_mask: ctamask }
        }
    }
    mbarrier.complete_tx{.sem}{.scope}{.space}{.multicast}.b64 [addr], txcount{, ctamask} => {
        let _ = (sem, scope, multicast);
        Instruction::MbarrierTx {
            data: MbarrierTxDetails { space: shared_space(space), complete: true },
            arguments: MbarrierTxArgs { src_mbarrier: addr, src_tx_count: txcount, src_cta_mask: ctamask }
        }
    }
    // mbarrier.arrive{.sem}{.scope}{.shared{::cta}}.b64 mbstate, [addr]{, count};
    // mbarrier.arrive{.sem}{.scope}{.shared::cluster}.b64 _, [addr]{, count};
    // mbarrier.arrive.expect_tx{.sem}{.scope}{.shared{::cta}}.b64 mbstate, [addr], txCount;
    // mbarrier.arrive.noComplete{.release}{.cta}{.shared{::cta}}.b64 mbstate, [addr], count;
    mbarrier.arrive{.expect_tx}{.nocomplete}{.sem}{.scope}{.space}{.multicast}.b64 mbstate, [addr]{, count}{, extra} => {
        let _ = (sem, scope);
        mbarrier_arrive(mbstate, false, expect_tx, nocomplete, space, addr, count, extra, multicast)
    }
    mbarrier.arrive_drop{.expect_tx}{.nocomplete}{.sem}{.scope}{.space}{.multicast}.b64 mbstate, [addr]{, count}{, extra} => {
        let _ = (sem, scope);
        mbarrier_arrive(mbstate, true, expect_tx, nocomplete, space, addr, count, extra, multicast)
    }
    // mbarrier.test_wait{.parity}{.sem}{.scope}{.shared{::cta}}.b64 waitcomplete, [addr], mbstate|phaseParity;
    mbarrier.test_wait{.parity}{.sem}{.scope}{.space}.b64 waitcomplete, [addr], mbstate => {
        let _ = (sem, scope);
        Instruction::MbarrierWait {
            data: MbarrierWaitDetails { space: shared_space(space), parity, try_wait: false, conditional: false },
            arguments: MbarrierWaitArgs {
                dst: waitcomplete,
                dst_report_pred: None,
                dst_report_value: None,
                src_mbarrier: addr,
                src_state: mbstate,
                src_time_hint: None,
            }
        }
    }
    mbarrier.try_wait{.parity}{.sem}{.scope}{.space}.b64 waitcomplete, [addr], mbstate{, timehint} => {
        let _ = (sem, scope);
        Instruction::MbarrierWait {
            data: MbarrierWaitDetails { space: shared_space(space), parity, try_wait: true, conditional: false },
            arguments: MbarrierWaitArgs {
                dst: waitcomplete,
                dst_report_pred: None,
                dst_report_value: None,
                src_mbarrier: addr,
                src_state: mbstate,
                src_time_hint: timehint,
            }
        }
    }
    mbarrier.pending_count.b64 count, mbstate => {
        Instruction::MbarrierPendingCount {
            arguments: MbarrierPendingCountArgs { dst: count, src_state: mbstate }
        }
    }
    .nocomplete = { .noComplete };
    .layout = { .layout::v1 };
    .sem: RawMbarrierSemantics = { .release, .acquire, .relaxed };
    .scope: MemScope = { .cta, .cluster };
    .space: StateSpace = { .shared{::cta, ::cluster} };
    .multicast = { .multicast::cluster::32b };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-cp-reduce-async-bulk
    // cp.reduce.async.bulk.shared::cluster.shared::cta.mbarrier::complete_tx::bytes.redOp.type [dstmem], [srcmem], size, [mbar]
    // cp.reduce.async.bulk.global.shared::cta.bulk_group{.level::cache_hint}.redOp.type [dstmem], [srcmem], size{, cache-policy}
    cp.reduce.async.bulk{.shared::cluster}{.global}.shared::cta{.completion_mechanism}{.bulk_group}{.level::cache_hint}.redOp{.noftz}.type
                                            [dstmem], [srcmem], size{, [mbar]}{, cache_policy} => {
        // .add.f16 / .bf16 keep subnormals with .noftz and without it
        let _ = noftz;
        let valid = if shared_cluster {
            !global && completion_mechanism && !bulk_group && !level_cache_hint && mbar.is_some() && cache_policy.is_none()
        } else {
            global && bulk_group && !completion_mechanism && mbar.is_none() && level_cache_hint == cache_policy.is_some()
        };
        if !valid {
            state.errors.push(PtxError::SyntaxError(format!("invalid cp.reduce.async.bulk in {:?}", state.text)));
        }
        Instruction::CpReduceAsyncBulk {
            data: CpReduceAsyncBulkDetails {
                dst_space: if global { StateSpace::Global } else { StateSpace::Shared },
                src_space: StateSpace::Shared,
                op: ast::AtomicOp::new(redop, type_.kind()),
                type_
            },
            arguments: CpReduceAsyncBulkArgs {
                src_to: dstmem,
                src_from: srcmem,
                src_size: size,
                src_mbarrier: mbar,
                src_cache_policy: cache_policy,
            }
        }
    }
    .completion_mechanism = { .mbarrier::complete_tx::bytes };
    .level::cache_hint = { .L2::cache_hint };
    .redOp: RawAtomicOp = { .and, .or, .xor, .add, .inc, .dec, .min, .max };
    .type: ScalarType = { .b32, .u32, .s32, .b64, .u64, .s64, .f32, .f64, .f16, .bf16 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#logic-and-shift-instructions-shf
    shf.dir.mode.b32  d, a, b, c => {
        Instruction::Shf {
            data: ShfDetails { direction: dir, mode: mode },
            arguments: ShfArgs { dst: d, src_a: a, src_b: b, src_c: c }
        }
    }

    .dir: ShiftDirection = { .l, .r };
    .mode: FunnelShiftMode = { .clamp, .wrap };

    trap => {
        Instruction::Trap {}
    }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#control-flow-instructions-exit

    exit => {
        Instruction::Exit {}
    }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-dp4a

    dp4a.atype.btype  d, a, b, c => {
        Instruction::Dp4a {
            data: Dp4aDetails {
                atype,
                btype
            },
            arguments: Dp4aArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }

    .atype: ScalarType = { .u32, .s32 };
    .btype: ScalarType = { .u32, .s32 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-vote-sync

    vote.sync.mode.pred  d, {!}a, membermask => {
        let (negate, a) = a;
        Instruction::Vote {
            data: VoteDetails {
                mode,
                negate
            },
            arguments: VoteArgs { dst: d, src1: a, src2: Some(membermask) }
        }
    }
    vote.ballot.b32 d, {!}a => {
        let (negate, a) = a;
        Instruction::Vote {
            data: VoteDetails {
                mode: VoteMode::Ballot,
                negate
            },
            arguments: VoteArgs { dst: d, src1: a, src2: None }
        }
    }
    vote.sync.ballot.b32 d, {!}a, membermask => {
        let (negate, a) = a;
        Instruction::Vote {
            data: VoteDetails {
                mode: VoteMode::Ballot,
                negate
            },
            arguments: VoteArgs { dst: d, src1: a, src2: Some(membermask) }
        }
    }

    .mode: VoteMode = { .all, .any, .uni };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-redux-sync

    // also redux.sync.{min, max}{.abs}{.NaN}.f32
    redux.sync.op{.abs}{.nan}.type dst, src, membermask => {
        let float = type_ == ScalarType::F32;
        if (float && op == Reduction::Add) || (!float && (abs || nan)) {
            state.errors.push(PtxError::SyntaxError(format!("invalid redux.sync in {:?}", state.text)));
        }
        Instruction::ReduxSync {
            data: ReduxSyncData { type_, reduction: op, abs, nan },
            arguments: ReduxSyncArgs { dst, src, src_membermask: membermask }
        }
    }
    .op: Reduction = {.add, .min, .max};
    .abs = { .abs };
    .nan = { .NaN };
    .type: ScalarType = {.u32, .s32, .f32};
    redux.sync.bop.b32 dst, src, membermask => {
        Instruction::ReduxSync {
            data: ReduxSyncData { type_: ScalarType::B32, reduction: bop, abs: false, nan: false },
            arguments: ReduxSyncArgs { dst, src, src_membermask: membermask }
        }
    }
    .bop: Reduction = {.and, .or, .xor};

    // redux.sync.op.b32 dst, src, membermask;
    // .op   = {.and, .or, .xor}


    // https://docs.nvidia.com/cuda/parallel-thread-execution/#warp-level-matrix-instructions-ldmatrix
    // https://docs.nvidia.com/cuda/parallel-thread-execution/#warp-level-matrix-instructions-stmatrix
    // https://docs.nvidia.com/cuda/parallel-thread-execution/#warp-level-matrix-instructions-movmatrix
    // The many combinations of shapes and element types of the warp-level
    // matrix instructions are parsed by hand.
    ldmatrix <= { ldmatrix(stream) }
    stmatrix <= { stmatrix(stream) }
    movmatrix <= { movmatrix(stream) }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st-async
    // st.async and red.async to shared::cluster memory, completing their bytes on an mbarrier there
    st.async{.weak}{.sem}{.scope}{.ss}{.completion_mechanism}{.vec}.type [a], b, [mbar] => {
        let _ = (weak, sem, scope, ss, completion_mechanism);
        Instruction::StAsync {
            data: ast::StAsyncDetails { type_: ast::Type::maybe_vector(vec, type_), red: None, scalar: type_ },
            arguments: StAsyncArgs { src_addr: a, src_value: b, src_mbarrier: mbar }
        }
    }
    .sem: StAsyncSem = { .relaxed };
    .scope: MemScope = { .cluster };
    .ss: StateSpace = { .shared{::cluster} };
    .completion_mechanism = { .mbarrier::complete_tx::bytes };
    .vec: VectorPrefix = { .v2, .v4 };
    .type: ScalarType = { .b32, .b64, .u32, .u64, .s32, .s64, .f32, .f64 };

    red.async{.sem}{.scope}{.ss}{.completion_mechanism}.op.type [a], b, [mbar] => {
        let _ = (sem, scope, ss, completion_mechanism);
        Instruction::StAsync {
            data: ast::StAsyncDetails {
                type_: ast::Type::Scalar(type_),
                red: Some(ast::AtomicOp::new(op, type_.kind())),
                scalar: type_,
            },
            arguments: StAsyncArgs { src_addr: a, src_value: b, src_mbarrier: mbar }
        }
    }
    .sem: StAsyncSem = { .relaxed };
    .scope: MemScope = { .cluster };
    .ss: StateSpace = { .shared{::cluster} };
    .completion_mechanism = { .mbarrier::complete_tx::bytes };
    .op: RawAtomicOp = { .and, .or, .xor, .add, .inc, .dec, .min, .max };
    .type: ScalarType = { .b32, .b64, .u32, .u64, .s32, .s64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st-bulk
    st.bulk{.weak}{.ss} [a], size, initval => {
        let _ = weak;
        Instruction::StBulk {
            data: ast::StBulkDetails { space: shared_space(ss) },
            arguments: StBulkArgs { src_addr: a, src_size: size, src_init: initval }
        }
    }
    .ss: StateSpace = { .shared{::cta} };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#miscellaneous-instructions-setmaxnreg
    // a register count hint: the backend allocates registers per kernel
    setmaxnreg.action.sync.aligned.u32 imm => {
        let _ = (action, imm);
        Instruction::SetMaxNReg { data: 0 }
    }
    .action: SetMaxNRegAction = { .inc, .dec };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-clusterlaunchcontrol
    // No cluster is ever canceled (a valid outcome of try_cancel): the response says so
    clusterlaunchcontrol.try_cancel.async{.space}.completion_mechanism{.multicast}.b128 [addr], [mbar] => {
        let _ = space;
        Instruction::ClusterLaunchTryCancel {
            data: multicast,
            arguments: ClusterLaunchTryCancelArgs { src_addr: addr, src_mbarrier: mbar }
        }
    }
    .space: StateSpace = { .shared{::cta} };
    .completion_mechanism = { .mbarrier::complete_tx::bytes };
    .multicast = { .multicast::cluster::all };

    clusterlaunchcontrol.query_cancel.is_canceled.pred.b128 p, [resp] => {
        Instruction::ClusterLaunchQuery {
            data: ast::ClusterLaunchQueryDetails { ctaid: None, space: StateSpace::Shared },
            arguments: ClusterLaunchQueryArgs { dst: p, src: resp }
        }
    }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-barrier-cluster
    barrier.cluster.arrive{.sem}{.aligned} => {
        let _ = aligned;
        Instruction::BarrierCluster {
            data: ast::BarrierClusterDetails { arrive: true, relaxed: sem == Some(BarrierClusterSem::Relaxed) }
        }
    }
    barrier.cluster.wait{.acquire}{.aligned} => {
        let _ = (acquire, aligned);
        Instruction::BarrierCluster {
            data: ast::BarrierClusterDetails { arrive: false, relaxed: false }
        }
    }
    .sem: BarrierClusterSem = { .release, .relaxed };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-mapa
    mapa{.space}.type d, a, b => {
        Instruction::Mapa {
            data: ast::MapaDetails { shared: space.is_some(), type_ },
            arguments: MapaArgs { dst: d, src_addr: a, src_rank: b }
        }
    }
    .space: StateSpace = { .shared{::cluster} };
    .type: ScalarType = { .u32, .u64 };
    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-getctarank
    getctarank{.space}.type d, a => {
        Instruction::Getctarank {
            data: ast::MapaDetails { shared: space.is_some(), type_ },
            arguments: GetctarankArgs { dst: d, src_addr: a }
        }
    }
    .space: StateSpace = { .shared{::cluster} };
    .type: ScalarType = { .u32, .u64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-griddepcontrol
    griddepcontrol.action => {
        Instruction::GridDepControl {
            data: action
        }
    }
    .action: GridDepControlAction  = { .launch_dependents, .wait };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#warp-level-matrix-instructions-mma
    // https://docs.nvidia.com/cuda/parallel-thread-execution/#warp-level-matrix-instructions-wmma
    mma <= { mma(stream) }
    wmma <= { wmma(stream) }
    // https://docs.nvidia.com/cuda/parallel-thread-execution/#asynchronous-warpgroup-level-matrix-instructions
    wgmma <= { wgmma(stream) }

    copysign.type  d, a, b => {
        ast::Instruction::Copysign {
            data: type_,
            arguments: CopysignArgs { dst: d, src1: a, src2: b, },
        }
    }
    .type: ScalarType = { .f32, .f64 };

    // also prefetch.global.L2::evict_{last,normal}, and the PTX ISA 9.4 hints .L1::32B and .valid_addr
    prefetch{.space}{.level}{.level::eviction_priority}{.l1_size}{.valid_addr} [a] => {
        let _ = (level_eviction_priority, l1_size, valid_addr);
        let space = space.unwrap_or(StateSpace::Generic);
        ast::Instruction::Prefetch {
            data: PrefetchData { space, level: level.unwrap_or(CacheLevel::L2) },
            arguments: PrefetchArgs { src: a }
        }
    }
    .space: StateSpace =    { .global, .local };
    .level: CacheLevel =    { .L1, .L2 };
    .level::eviction_priority: EvictionPriority = { .L2::evict_last, .L2::evict_normal };
    .l1_size = { .L1::32B };
    .valid_addr = { .valid_addr };

    prefetchu.L1 [a] => {
        ast::Instruction::Prefetch {
            data: PrefetchData { space: StateSpace::Generic, level: CacheLevel::L1 },
            arguments: PrefetchArgs { src: a }
        }
    }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-applypriority
    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-discard
    // Cache hints, no-ops like prefetch: the lines discard leaves undefined keep their data
    applypriority{.global}.level::eviction_priority [a], size => {
        let _ = (level_eviction_priority, size);
        ast::Instruction::Prefetch {
            data: PrefetchData { space: if global { StateSpace::Global } else { StateSpace::Generic }, level: CacheLevel::L2 },
            arguments: PrefetchArgs { src: a }
        }
    }
    .level::eviction_priority: EvictionPriority = { .L2::evict_normal };

    discard{.global}.level [a], size => {
        let _ = size;
        ast::Instruction::Prefetch {
            data: PrefetchData { space: if global { StateSpace::Global } else { StateSpace::Generic }, level },
            arguments: PrefetchArgs { src: a }
        }
    }
    .level: CacheLevel = { .L2 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-createpolicy
    // createpolicy.range{.global}.level::primary_priority{.level::secondary_priority}.b64
    //                                 cache-policy, [a], primary-size, total-size;

    // (the form with a secondary priority is parsed by `createpolicy_secondary`)
    createpolicy.fractional.level::primary_priority.b64
                                    cache-policy{, fraction} => {
        let fraction = fraction
            .and_then(|op| op.as_immediate())
            .and_then(|imm| imm.as_f64()).unwrap_or(1.0) as f32;

        Instruction::CreatePolicyFractional {
            data: CreatePolicyFractionalDetails {
                primary_priority: level_primary_priority,
                fraction,
            },
            arguments: CreatePolicyFractionalArgs {
                dst_policy: cache_policy,
            }
        }
    }

    // createpolicy.cvt.L2.b64            cache-policy, access-property;

    .level::primary_priority: EvictionPriority =   { .L2::evict_last, .L2::evict_normal,
                                .L2::evict_first, .L2::evict_unchanged };
    // .level::secondary_priority: EvictionPriority = { .L2::evict_first, .L2::evict_unchanged };

    sad.type  d, a, b, c => {
        Instruction::Sad {
            data: type_,
            arguments: SadArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }

    .type: ScalarType = { .u16, .u32, .u64, .s16, .s32, .s64 };

    dp2a.mode.atype.btype  d, a, b, c => {
        Instruction::Dp2a {
            data: Dp2aData {
                atype: atype,
                btype: btype,
                control: match mode {
                    RawDp2aControl::Lo => Dp2aControl::Low,
                    RawDp2aControl::Hi => Dp2aControl::High
                },
            },
            arguments: Dp2aArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }

    .atype: ScalarType = { .u32, .s32 };
    .btype: ScalarType = { .u32, .s32 };
    .mode: RawDp2aControl = { .lo, .hi };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#extended-precision-arithmetic-instructions-add-cc
    add.cc.type  d, a, b => {
        Instruction::AddExtended {
            data: CarryDetails { kind: CarryKind::CarryOut, type_: type_ },
            arguments: AddExtendedArgs {
                dst: d,
                src1: a,
                src2: b,
            }
        }
    }

    .type: ScalarType = { .u32, .s32, .u64, .s64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#extended-precision-arithmetic-instructions-addc
    addc{.cc}.type  d, a, b => {
        Instruction::AddExtended {
            data: CarryDetails { kind: if cc { CarryKind::CarryInCarryOut } else { CarryKind::CarryIn }, type_: type_ },
            arguments: AddExtendedArgs {
                dst: d,
                src1: a,
                src2: b,
            }
        }
    }

    .type: ScalarType = { .u32, .s32, .u64, .s64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#extended-precision-arithmetic-instructions-sub-cc
    sub.cc.type  d, a, b => {
        Instruction::SubExtended {
            data: CarryDetails { kind: CarryKind::CarryOut, type_: type_ },
            arguments: SubExtendedArgs {
                dst: d,
                src1: a,
                src2: b,
            }
        }
    }

    .type: ScalarType = { .u32, .s32, .u64, .s64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#extended-precision-arithmetic-instructions-subc
    subc{.cc}.type  d, a, b => {
        Instruction::SubExtended {
            data: CarryDetails { kind: if cc { CarryKind::CarryInCarryOut } else { CarryKind::CarryIn }, type_: type_ },
            arguments: SubExtendedArgs {
                dst: d,
                src1: a,
                src2: b,
            }
        }
    }

    .type: ScalarType = { .u32, .s32, .u64, .s64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#extended-precision-arithmetic-instructions-mad-cc
    mad.mode.cc.type  d, a, b, c => {
        Instruction::MadExtended {
            data: MadCarryDetails { kind: CarryKind::CarryOut, control: mode.into(), type_: type_ },
            arguments: MadExtendedArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }

    .mode: RawMulIntControl = { .lo, .hi };
    .type: ScalarType = { .u32, .s32, .u64, .s64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#extended-precision-arithmetic-instructions-madc
    madc.mode{.cc}.type  d, a, b, c => {
        Instruction::MadExtended {
            data: MadCarryDetails {
                kind: if cc { CarryKind::CarryInCarryOut } else { CarryKind::CarryIn },
                control: mode.into(),
                type_: type_
            },
            arguments: MadExtendedArgs { dst: d, src1: a, src2: b, src3: c }
        }
    }

    .mode: RawMulIntControl = { .lo, .hi };
    .type: ScalarType = { .u32, .s32, .u64, .s64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#video-instructions
    // These get custom handling because the type modifiers are at the start
    // of the instruction and the operands have selectors
    vadd <= { video(stream, "vadd") }
    vsub <= { video(stream, "vsub") }
    vabsdiff <= { video(stream, "vabsdiff") }
    vmin <= { video(stream, "vmin") }
    vmax <= { video(stream, "vmax") }
    vshl <= { video(stream, "vshl") }
    vshr <= { video(stream, "vshr") }
    vset <= { video(stream, "vset") }
    vmad <= { video(stream, "vmad") }
    vadd2 <= { video(stream, "vadd2") }
    vsub2 <= { video(stream, "vsub2") }
    vavrg2 <= { video(stream, "vavrg2") }
    vabsdiff2 <= { video(stream, "vabsdiff2") }
    vmin2 <= { video(stream, "vmin2") }
    vmax2 <= { video(stream, "vmax2") }
    vset2 <= { video(stream, "vset2") }
    vadd4 <= { video(stream, "vadd4") }
    vsub4 <= { video(stream, "vsub4") }
    vavrg4 <= { video(stream, "vavrg4") }
    vabsdiff4 <= { video(stream, "vabsdiff4") }
    vmin4 <= { video(stream, "vmin4") }
    vmax4 <= { video(stream, "vmax4") }
    vset4 <= { video(stream, "vset4") }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#texture-instructions-tex
    tex.1d.v4.dtype.ctype  d, [a, c] => {
        Instruction::Tex {
            data: TexData {
                dtype,
                ctype,
                dims: TexDimensions::D1,
                type_: TexType::Texref
            },
            arguments: TexArgs { dst: d, src_ptr: a, src_coord: c }
        }
    }
    tex.2d.v4.dtype.ctype  d, [a, c] => {
        Instruction::Tex {
            data: TexData {
                dtype,
                ctype,
                dims: TexDimensions::D2,
                type_: TexType::Texref
            },
            arguments: TexArgs { dst: d, src_ptr: a, src_coord: c }
        }
    }
    tex.3d.v4.dtype.ctype  d, [a, c] => {
        Instruction::Tex {
            data: TexData {
                dtype,
                ctype,
                dims: TexDimensions::D3,
                type_: TexType::Texref
            },
            arguments: TexArgs { dst: d, src_ptr: a, src_coord: c }
        }
    }

    .dtype: ScalarType = { .u32, .s32, .f32 };
    .ctype: ScalarType = { .s32, .f32 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/
    match.any.sync.type  d, a, membermask => {
        Instruction::MatchSync {
            data: type_,
            arguments: MatchSyncArgs { dst: d, src: a, src_membermask: membermask }
        }
    }
    // d = the member mask when every thread holds the same a (else 0), p = whether they do
    match.all.sync.type  d[|p], a, membermask => {
        Instruction::MatchAllSync {
            data: type_,
            arguments: MatchAllSyncArgs { dst: d, dst_pred: p, src: a, src_membermask: membermask }
        }
    }

    .type: ScalarType = { .b32, .b64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#parallel-synchronization-and-communication-instructions-elect-sync
    // d = the lane of the leader (the lowest active member of membermask), p = whether this is the leader
    elect.sync  d[|p], membermask => {
        Instruction::ElectSync {
            arguments: ElectSyncArgs { dst: d, dst_pred: p, src_membermask: membermask }
        }
    }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-bfind
    bfind{.shiftamt}.type  d, a => {
        Instruction::Bfind {
            data: BfindDetails { type_, shift_amount: shiftamt },
            arguments: BfindArgs { dst: d, src: a }
        }
    }
    .type: ScalarType = { .u32, .u64, .s32, .s64 };
);

#[cfg(test)]
mod tests {
    use crate::first_optional;
    use crate::module;
    use crate::multi_variable;
    use crate::parse_module_checked;
    use crate::section;
    use crate::PtxError;

    use super::target;
    use super::PtxParserState;
    use super::Token;
    use logos::Logos;
    use logos::Span;
    use winnow::prelude::*;

    #[test]
    fn sink_is_not_an_identifier() {
        let tokens = Token::lexer("_ _reg __ %r1")
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Sink,
                Token::Ident("_reg"),
                Token::Ident("__"),
                Token::Ident("%r1")
            ]
        );
    }

    #[test]
    fn vector_operand_preserves_sink_positions() {
        let text = "{_, %r1, _, 7}";
        let tokens = Token::lexer(text)
            .map(|t| t.map(|t| (t, Span::default())))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let mut errors = Vec::new();
        let stream = super::PtxParser {
            input: &tokens[..],
            state: PtxParserState::new(text, &mut errors),
        };
        let operand = crate::ParsedOperand::<&str>::parse.parse(stream).unwrap();
        let crate::ParsedOperand::VecPack(ref elements) = operand else {
            panic!("expected a vector operand");
        };
        assert_eq!(
            elements,
            &vec![
                None,
                Some(crate::RegOrImmediate::Reg("%r1")),
                None,
                Some(crate::RegOrImmediate::Imm(crate::ImmediateValue::S64(7))),
            ]
        );
        assert_eq!(operand.to_string(), text);
        assert!(errors.is_empty());
    }

    fn parse_operand(text: &str) -> crate::ParsedOperand<&str> {
        let tokens = Token::lexer(text)
            .spanned()
            .map(|(t, span)| t.map(|t| (t, span)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let mut errors = Vec::new();
        let stream = super::PtxParser {
            input: &tokens[..],
            state: PtxParserState::new(text, &mut errors),
        };
        let operand = crate::ParsedOperand::<&str>::parse.parse(stream).unwrap();
        assert!(errors.is_empty());
        operand
    }

    #[test]
    fn vector_members_by_colour_and_in_vector_operands() {
        // .r and .a are also modifier tokens
        assert!(matches!(parse_operand("%V.r"), crate::ParsedOperand::VecMember("%V", 0)));
        assert!(matches!(parse_operand("%V.a"), crate::ParsedOperand::VecMember("%V", 3)));
        let crate::ParsedOperand::VecPack(elements) = parse_operand("{%r1, %V.z, %V.x}") else {
            panic!("expected a vector operand");
        };
        assert_eq!(
            elements,
            vec![
                Some(crate::RegOrImmediate::Reg("%r1")),
                Some(crate::RegOrImmediate::VecMember("%V", 2)),
                Some(crate::RegOrImmediate::VecMember("%V", 0)),
            ]
        );
    }

    #[test]
    fn array_elements() {
        assert!(matches!(parse_operand("a[%i]"), crate::ParsedOperand::RegIndex("a", "%i", 0)));
        assert!(matches!(parse_operand("a[%i+8]"), crate::ParsedOperand::RegIndex("a", "%i", 8)));
        assert!(matches!(parse_operand("a[%i - 4]"), crate::ParsedOperand::RegIndex("a", "%i", -4)));
        assert!(matches!(parse_operand("a[3 * 4]"), crate::ParsedOperand::RegOffset("a", 12)));
        assert!(matches!(parse_operand("a[(1 + 2) * 8 / 2 - 4]"), crate::ParsedOperand::RegOffset("a", 8)));
        assert!(matches!(parse_operand("%rd1-4"), crate::ParsedOperand::RegOffset("%rd1", -4)));
    }

    #[test]
    fn first_optional_present() {
        let text = "AB";
        let result = first_optional::<_, _, _, (), _, _>('A', 'B').parse(text);
        assert_eq!(result, Ok((Some('A'), 'B')));
    }

    #[test]
    fn first_optional_absent() {
        let text = "B";
        let result = first_optional::<_, _, _, (), _, _>('A', 'B').parse(text);
        assert_eq!(result, Ok((None, 'B')));
    }

    #[test]
    fn first_optional_repeated_absent() {
        let text = "A";
        let result = first_optional::<_, _, _, (), _, _>('A', 'A').parse(text);
        assert_eq!(result, Ok((None, 'A')));
    }

    #[test]
    fn first_optional_repeated_present() {
        let text = "AA";
        let result = first_optional::<_, _, _, (), _, _>('A', 'A').parse(text);
        assert_eq!(result, Ok((Some('A'), 'A')));
    }

    #[test]
    fn first_optional_sequence_absent() {
        let text = "AA";
        let result = ('A', first_optional::<_, _, _, (), _, _>('A', 'A')).parse(text);
        assert_eq!(result, Ok(('A', (None, 'A'))));
    }

    #[test]
    fn first_optional_sequence_present() {
        let text = "AAA";
        let result = ('A', first_optional::<_, _, _, (), _, _>('A', 'A')).parse(text);
        assert_eq!(result, Ok(('A', (Some('A'), 'A'))));
    }

    #[test]
    fn first_optional_no_match() {
        let text = "C";
        let result = first_optional::<_, _, _, (), _, _>('A', 'B').parse(text);
        assert!(result.is_err());
    }

    #[test]
    fn prmt_modes() {
        for mode in ["", ".f4e", ".b4e", ".rc8", ".ecl", ".ecr", ".rc16"] {
            let text = format!(
                ".version 7.0\n.target sm_70\n.address_size 64\n.visible .entry k() {{ .reg .b32 r; prmt.b32{mode} r, r, r, r; ret; }}"
            );
            assert!(parse_module_checked(&text).is_ok(), "prmt.b32{mode}");
        }
        assert!(parse_module_checked(
            ".version 7.0\n.target sm_70\n.address_size 64\n.visible .entry k() { .reg .b32 r; prmt.b32.f5e r, r, r, r; ret; }"
        )
        .is_err());
    }

    #[test]
    fn szext_fns_min3() {
        let body = "szext.wrap.s32 r, r, r; szext.clamp.u32 r, r, r; fns.b32 r, r, r, r; min.f32 f, f, f, f; max.NaN.f32 f, f, f, f;";
        let text = format!(
            ".version 8.8\n.target sm_100\n.address_size 64\n.visible .entry k() {{ .reg .b32 r; .reg .f32 f; {body} ret; }}"
        );
        assert!(parse_module_checked(&text).is_ok());
    }

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
    fn ptr_param_alignment_is_the_pointees() {
        let text = ".version 7.0\n.target sm_80\n.address_size 64\n.visible .entry k(.param .u8 a, .param .u64 .ptr .global .align 1 p)\n{\n\tret;\n}\n";
        let module = super::parse_module_checked(text).unwrap();
        let [super::ast::Directive::Method(_, method)] = &module.directives[..] else {
            panic!()
        };
        let params = &method.func_directive.input_arguments;
        assert_eq!((params[1].name, params[1].info.align), ("p", None));
    }

    #[test]
    fn report_unknown_instruction() {
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
            PtxError::UnrecognizedStatement("unknown_op1.asdf foobar;")
        ));
        assert!(matches!(
            errors[1],
            PtxError::UnrecognizedStatement("unknown_op2 temp2, temp;")
        ));
    }

    #[test]
    fn report_unknown_instruction_with_braces() {
        let text = "
            .version 6.5
            .target sm_60
            .address_size 64

            .visible .entry unrecognized_braces(
            )
            {
                mov.u32 foo, {} {};
                ret;
            }";
        let errors = parse_module_checked(text).err().unwrap();
        assert_eq!(
            errors,
            vec![PtxError::UnrecognizedStatement("mov.u32 foo, {} {};")]
        );
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
            PtxError::UnrecognizedDirective(".broken_directive_fail; 34; {")
        ));
        assert!(matches!(
            errors[1],
            PtxError::UnrecognizedDirective("section foobar }")
        ));
    }

    #[test]
    fn report_unknown_type_in_directive() {
        let text = "
            .version 4.1
            .target sm_52
            .address_size 64
            .global .bad_type foo;
            .const .b32 bar;
";
        let errors = parse_module_checked(text).err().unwrap();
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            PtxError::UnrecognizedDirective(".global .bad_type foo;")
        ));
    }

    #[test]
    fn dwarf_line() {
        let text = "
        .section        .debug_abbrev
        {
.b8 1                                   // Abbreviation Code
.b8 17                                  // DW_TAG_compile_unit
.b8 1                                   // DW_CHILDREN_yes
.b8 37                                  // DW_AT_producer
.b8 8                                   // DW_FORM_string
.b8 19                                  // DW_AT_language
.b8 5                                   // DW_FORM_data2
.b8 3                                   // DW_AT_name
.b8 8                                   // DW_FORM_string
.b8 16                                  // DW_AT_stmt_list
.b8 6                                   // DW_FORM_data4
.b8 27                                  // DW_AT_comp_dir
.b8 8                                   // DW_FORM_string
.b8 0                                   // EOM(1)
.b8 0                                   // EOM(2)
.b8 2                                   // Abbreviation Code
.b8 46                                  // DW_TAG_subprogram
.b8 0                                   // DW_CHILDREN_no
.b8 3                                   // DW_AT_name
.b8 8                                   // DW_FORM_string
.b8 32                                  // DW_AT_inline
.b8 11                                  // DW_FORM_data1
.b8 0                                   // EOM(1)
.b8 0                                   // EOM(2)
.b8 3                                   // Abbreviation Code
.b8 46                                  // DW_TAG_subprogram
.b8 1                                   // DW_CHILDREN_yes
.b8 17                                  // DW_AT_low_pc
.b8 1                                   // DW_FORM_addr
.b8 18                                  // DW_AT_high_pc
.b8 1                                   // DW_FORM_addr
.b8 49                                  // DW_AT_abstract_origin
.b8 19                                  // DW_FORM_ref4
.b8 0                                   // EOM(1)
.b8 0                                   // EOM(2)
.b8 4                                   // Abbreviation Code
.b8 29                                  // DW_TAG_inlined_subroutine
.b8 0                                   // DW_CHILDREN_no
.b8 49                                  // DW_AT_abstract_origin
.b8 19                                  // DW_FORM_ref4
.b8 17                                  // DW_AT_low_pc
.b8 1                                   // DW_FORM_addr
.b8 18                                  // DW_AT_high_pc
.b8 1                                   // DW_FORM_addr
.b8 88                                  // DW_AT_call_file
.b8 11                                  // DW_FORM_data1
.b8 89                                  // DW_AT_call_line
.b8 11                                  // DW_FORM_data1
.b8 87                                  // DW_AT_call_column
.b8 11                                  // DW_FORM_data1
.b8 0                                   // EOM(1)
.b8 0                                   // EOM(2)
.b8 0                                   // EOM(3)
        }
";
        let tokens = Token::lexer(text)
            .map(|t| t.map(|t| (t, Span::default())))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let mut errors = Vec::new();
        let stream = super::PtxParser {
            input: &tokens[..],
            state: PtxParserState::new(text, &mut errors),
        };
        assert!(section.parse(stream).is_ok());
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn report_unknown_directives() {
        let text = "
            .version 6.5
            .target sm_30
            .address_size 64

            .global .b32 global[4] = {  unknown (1),   2,   3,   4};

            .visible .entry func1()
            {
                st.u64          [out_addr], temp2;
                ret;
            }

            .visible .entry func1()
            {
                broken_instruction;
                ret;
            }";
        let tokens = Token::lexer(text)
            .map(|t| t.map(|t| (t, Span::default())))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let mut errors = Vec::new();
        let stream = super::PtxParser {
            input: &tokens[..],
            state: PtxParserState::new(text, &mut errors),
        };
        let module = module.parse(stream).unwrap();
        assert_eq!(module.directives.len(), 1);
        assert_eq!(module.invalid_directives, 2);
    }

    #[test]
    fn file_and_loc_are_kept() {
        let text = "
            .version 6.5
            .target sm_30
            .address_size 64
            .file 1 \"/src/example.cu\", 1700000000, 1234
            .file 2 \"example.h\"
            .visible .entry func1()
            {
                .loc 2 14 3, function_name $L__info_string0, inlined_at 1 9 5
                ret;
            }";
        let module = parse_module_checked(text).unwrap();
        assert_eq!(module.files, [(1, "/src/example.cu"), (2, "example.h")]);
        let crate::ast::Directive::Method(_, f) = &module.directives[0] else { panic!() };
        assert!(matches!(
            f.body.as_ref().unwrap()[0],
            crate::ast::Statement::Loc(crate::ast::DebugLoc { file: 2, line: 14, column: 3 })
        ));
    }

    #[test]
    fn dont_report_ignored_directives() {
        let text = "
            .version 6.5
            .target sm_30
            .address_size 64

            .file 1 \"example.cu\"

            .visible .entry func1()
            {
                ret;
            }";
        let tokens = Token::lexer(text)
            .map(|t| t.map(|t| (t, Span::default())))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let mut errors = Vec::new();
        let stream = super::PtxParser {
            input: &tokens[..],
            state: PtxParserState::new(text, &mut errors),
        };
        let module = module.parse(stream).unwrap();
        assert_eq!(module.directives.len(), 1);
        assert_eq!(module.invalid_directives, 0);
    }

    #[test]
    fn extern_func_noreturn() {
        let text = ".version 6.4
.target sm_70
.address_size 64

.extern .func __assertfail() .noreturn;
";
        let tokens = Token::lexer(text)
            .map(|t| t.map(|t| (t, Span::default())))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let mut errors = Vec::new();
        let stream = super::PtxParser {
            input: &tokens[..],
            state: PtxParserState::new(text, &mut errors),
        };
        let result = module.parse(stream);
        assert!(result.is_ok(), "Failed to parse extern func with .noreturn");
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn const_zero_pad() {
        let text = ".align 4 .b8 constData[8]";
        let tokens = Token::lexer(text)
            .map(|t| t.map(|t| (t, Span::default())))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let mut errors = Vec::new();
        let stream = super::PtxParser {
            input: &tokens[..],
            state: PtxParserState::new(text, &mut errors),
        };
        let result = multi_variable(false, crate::StateSpace::Const)
            .parse(stream)
            .unwrap();
        let result = match result {
            crate::ast::MultiVariable::Names { info, .. } => info,
            _ => panic!(),
        };
        assert_eq!(
            result.array_init,
            vec![crate::RegOrImmediate::Imm(crate::ImmediateValue::U64(0)); 8]
        );
    }

    #[test]
    fn global_zero_init() {
        let text = ".align 4 .b8 constData";
        let tokens = Token::lexer(text)
            .map(|t| t.map(|t| (t, Span::default())))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let mut errors = Vec::new();
        let stream = super::PtxParser {
            input: &tokens[..],
            state: PtxParserState::new(text, &mut errors),
        };
        let result = multi_variable(false, crate::StateSpace::Global)
            .parse(stream)
            .unwrap();
        let result = match result {
            crate::ast::MultiVariable::Names { info, .. } => info,
            _ => panic!(),
        };
        assert_eq!(
            result.array_init,
            vec![crate::RegOrImmediate::Imm(crate::ImmediateValue::U64(0))]
        );
    }
}
