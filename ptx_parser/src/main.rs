use gen::derive_parser;
use logos::Logos;
use std::mem;
use std::num::{ParseFloatError, ParseIntError};
use winnow::combinator::{alt, empty, fail, opt};
use winnow::stream::SliceLen;
use winnow::token::{any, literal};
use winnow::{
    error::{ContextError, ParserError},
    stream::{Offset, Stream, StreamIsPartial},
    PResult,
};
use winnow::{prelude::*, Stateful};

mod ast;

pub trait Operand {}

pub trait Visitor<T> {
    fn visit(&mut self, args: &T, type_: &Type, space: StateSpace, is_dst: bool);
}

pub trait VisitorMut<T> {
    fn visit(&mut self, args: &mut T, type_: &Type, space: StateSpace, is_dst: bool);
}

pub trait VisitorMap<From, To> {
    fn visit(&mut self, args: From, type_: &Type, space: StateSpace, is_dst: bool) -> To;
}

#[derive(Clone)]
pub struct MovDetails {
    pub typ: Type,
    pub src_is_address: bool,
    // two fields below are in use by member moves
    pub dst_width: u8,
    pub src_width: u8,
    // This is in use by auto-generated movs
    pub relaxed_src2_conv: bool,
}

impl MovDetails {
    pub fn new(typ: Type) -> Self {
        MovDetails {
            typ,
            src_is_address: false,
            dst_width: 0,
            src_width: 0,
            relaxed_src2_conv: false,
        }
    }
}

gen::generate_instruction_type!(
    enum Instruction<T> {
        Mov {
            type: { &data.typ },
            data: MovDetails,
            arguments<T>: {
                dst: T,
                src: T
            }
        },
        Ld {
            type: { &data.typ },
            data: LdDetails,
            arguments<T>: {
                dst: T,
                src: {
                    repr: T,
                    space: { data.state_space },
                }
            }
        },
        Add {
            type: { data.type_().into() },
            data: ArithDetails,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
            }
        },
        St {
            type: { &data.typ },
            data: StData,
            arguments<T>: {
                src1: {
                    repr: T,
                    space: { data.state_space },
                },
                src2: T,
            }
        },
        Ret {
            data: RetData
        },
        Trap { }
    }
);

pub struct LdDetails {
    pub qualifier: LdStQualifier,
    pub state_space: StateSpace,
    pub caching: LdCacheOperator,
    pub typ: Type,
    pub non_coherent: bool,
}

#[derive(Copy, Clone)]
pub enum ArithDetails {
    Unsigned(ScalarType),
    Signed(ArithSInt),
    Float(ArithFloat),
}

impl ArithDetails {
    fn type_(&self) -> ScalarType {
        match self {
            ArithDetails::Unsigned(t) => *t,
            ArithDetails::Signed(arith) => arith.typ,
            ArithDetails::Float(arith) => arith.typ,
        }
    }
}

#[derive(Copy, Clone)]
pub struct ArithSInt {
    pub typ: ScalarType,
    pub saturate: bool,
}

#[derive(Copy, Clone)]
pub struct ArithFloat {
    pub typ: ScalarType,
    pub rounding: Option<RoundingMode>,
    pub flush_to_zero: Option<bool>,
    pub saturate: bool,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum RoundingMode {
    NearestEven,
    Zero,
    NegativeInf,
    PositiveInf,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LdCacheOperator {
    Cached,
    L2Only,
    Streaming,
    LastUse,
    Uncached,
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Type {
    // .param.b32 foo;
    Scalar(ScalarType),
    // .param.v2.b32 foo;
    Vector(ScalarType, u8),
    // .param.b32 foo[4];
    Array(ScalarType, Vec<u32>),
}

impl From<ScalarType> for Type {
    fn from(value: ScalarType) -> Self {
        Type::Scalar(value)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LdStQualifier {
    Weak,
    Volatile,
    Relaxed(MemScope),
    Acquire(MemScope),
    Release(MemScope),
}

pub struct StData {
    pub qualifier: LdStQualifier,
    pub state_space: StateSpace,
    pub caching: StCacheOperator,
    pub typ: Type,
}

#[derive(PartialEq, Eq)]
pub enum StCacheOperator {
    Writeback,
    L2Only,
    Streaming,
    Writethrough,
}

#[derive(Copy, Clone)]
pub struct RetData {
    pub uniform: bool,
}

type ParserState<'a, 'input> = Stateful<&'a [Token<'input>], Vec<PtxError>>;

fn ident<'a, 'input>(stream: &mut ParserState<'a, 'input>) -> PResult<&'input str> {
    any.verify_map(|t| {
        if let Token::Ident(text) = t {
            Some(text)
        } else {
            None
        }
    })
    .parse_next(stream)
}

fn num<'a, 'input>(stream: &mut ParserState<'a, 'input>) -> PResult<(&'input str, u32, bool)> {
    any.verify_map(|t| {
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
    mut parser: impl Parser<ParserState<'a, 'input>, Result<O, (O, PtxError)>, E>,
) -> impl Parser<ParserState<'a, 'input>, O, E> {
    move |input: &mut ParserState<'a, 'input>| {
        Ok(match parser.parse_next(input)? {
            Ok(x) => x,
            Err((x, err)) => {
                input.state.push(err);
                x
            }
        })
    }
}

fn int_immediate<'a, 'input>(input: &mut ParserState<'a, 'input>) -> PResult<ast::ImmediateValue> {
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

fn f32<'a, 'input>(stream: &mut ParserState<'a, 'input>) -> PResult<f32> {
    take_error(any.verify_map(|t| match t {
        Token::F32(f) => Some(match u32::from_str_radix(&f[2..], 16) {
            Ok(x) => Ok(f32::from_bits(x)),
            Err(err) => Err((0.0, PtxError::from(err))),
        }),
        _ => None,
    }))
    .parse_next(stream)
}

fn f64<'a, 'input>(stream: &mut ParserState<'a, 'input>) -> PResult<f64> {
    take_error(any.verify_map(|t| match t {
        Token::F64(f) => Some(match u64::from_str_radix(&f[2..], 16) {
            Ok(x) => Ok(f64::from_bits(x)),
            Err(err) => Err((0.0, PtxError::from(err))),
        }),
        _ => None,
    }))
    .parse_next(stream)
}

fn s32<'a, 'input>(stream: &mut ParserState<'a, 'input>) -> PResult<i32> {
    take_error((opt(Token::Minus), num).map(|(sign, x)| {
        let (text, radix, _) = x;
        match i32::from_str_radix(text, radix) {
            Ok(x) => Ok(if sign.is_some() { -x } else { x }),
            Err(err) => Err((0, PtxError::from(err))),
        }
    }))
    .parse_next(stream)
}

fn immediate_value<'a, 'input>(
    stream: &mut ParserState<'a, 'input>,
) -> PResult<ast::ImmediateValue> {
    alt((
        int_immediate,
        f32.map(ast::ImmediateValue::F32),
        f64.map(ast::ImmediateValue::F64),
    ))
    .parse_next(stream)
}

impl<Ident> ast::ParsedOperand<Ident> {
    fn parse<'a, 'input>(
        stream: &mut ParserState<'a, 'input>,
    ) -> PResult<ast::ParsedOperand<&'input str>> {
        use winnow::combinator::*;
        use winnow::token::any;
        fn vector_index<'input>(inp: &'input str) -> Result<u8, PtxError> {
            match inp {
                "x" | "r" => Ok(0),
                "y" | "g" => Ok(1),
                "z" | "b" => Ok(2),
                "w" | "a" => Ok(3),
                _ => Err(PtxError::WrongVectorElement),
            }
        }
        fn ident_operands<'a, 'input>(
            stream: &mut ParserState<'a, 'input>,
        ) -> PResult<ast::ParsedOperand<&'input str>> {
            let main_ident = ident.parse_next(stream)?;
            alt((
                preceded(Token::Plus, s32)
                    .map(move |offset| ast::ParsedOperand::RegOffset(main_ident, offset)),
                take_error(preceded(Token::Dot, ident).map(move |suffix| {
                    let vector_index = vector_index(suffix)
                        .map_err(move |e| (ast::ParsedOperand::VecMember(main_ident, 0), e))?;
                    Ok(ast::ParsedOperand::VecMember(main_ident, vector_index))
                })),
                empty.value(ast::ParsedOperand::Reg(main_ident)),
            ))
            .parse_next(stream)
        }
        fn vector_operand<'a, 'input>(
            stream: &mut ParserState<'a, 'input>,
        ) -> PResult<Vec<&'input str>> {
            let (_, r1, _, r2) =
                (Token::LBracket, ident, Token::Comma, ident).parse_next(stream)?;
            dispatch! {any;
                Token::LBracket => empty.map(|_| vec![r1, r2]),
                Token::Comma => (ident, Token::Comma, ident, Token::LBracket).map(|(r3, _, r4, _)| vec![r1, r2, r3, r4]),
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
pub enum PtxError {
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
    #[error("")]
    SyntaxError,
    #[error("")]
    NonF32Ftz,
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
    #[error("{start}:{end}")]
    UnrecognizedStatement { start: usize, end: usize },
    #[error("{start}:{end}")]
    UnrecognizedDirective { start: usize, end: usize },
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

impl<'input, I: Stream<Token = Self> + StreamIsPartial, E: ParserError<I>> Parser<I, Self, E>
    for Token<'input>
{
    fn parse_next(&mut self, input: &mut I) -> PResult<Self, E> {
        any.parse_next(input)
    }
}

// Modifiers are turned into arguments to the blocks, with type:
// * If it is an alternative:
//   * If it is mandatory then its type is Foo (as defined by the relevant rule)
//   * If it is optional then its type is Option<Foo>
// * Otherwise:
//   * If it is mandatory then it is skipped
//   * If it is optional then its type is `bool`

type ParsedOperand<'input> = ast::ParsedOperand<&'input str>;

derive_parser!(
    #[derive(Logos, PartialEq, Eq, Debug, Clone, Copy)]
    #[logos(skip r"\s+")]
    enum Token<'input> {
        #[token(",")]
        Comma,
        #[token(".")]
        Dot,
        #[regex(r"[a-zA-Z][a-zA-Z0-9_$]*|[_$%][a-zA-Z0-9_$]+", |lex| lex.slice(), priority = 0)]
        Ident(&'input str),
        #[token("|")]
        Or,
        #[token("!")]
        Not,
        #[token(";")]
        Semicolon,
        #[token("[")]
        LBracket,
        #[token("]")]
        RBracket,
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
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub enum StateSpace {
        Reg
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub enum MemScope { }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub enum ScalarType { }

    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#data-movement-and-conversion-instructions-mov

    mov.type  d, a => {
        Instruction::Mov{
            data: MovDetails::new(type_.into()),
            arguments: MovArgs { dst: d, src: a }
        }
    }
    .type: ScalarType = { .pred,
                          .b16, .b32, .b64,
                          .u16, .u32, .u64,
                          .s16, .s32, .s64,
                                .f32, .f64 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-st
    st{.weak}{.ss}{.cop}{.level::eviction_priority}{.level::cache_hint}{.vec}.type  [a], b{, cache_policy} => {
        todo!()
    }
    st.volatile{.ss}{.vec}.type                                                     [a], b => {
        todo!()
    }
    st.relaxed.scope{.ss}{.level::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache_policy} => {
        todo!()
    }
    st.release.scope{.ss}{.level::eviction_priority}{.level::cache_hint}{.vec}.type [a], b{, cache_policy} => {
        todo!()
    }
    st.mmio.relaxed.sys{.global}.type                                               [a], b => {
        todo!()
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

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#data-movement-and-conversion-instructions-ld
    ld{.weak}{.ss}{.cop}{.level::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type   d, [a]{.unified}{, cache_policy} => {
        todo!()
    }
    ld.volatile{.ss}{.level::prefetch_size}{.vec}.type                                                      d, [a] => {
        todo!()
    }
    ld.relaxed.scope{.ss}{.level::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{, cache_policy} => {
        todo!()
    }
    ld.acquire.scope{.ss}{.level::eviction_priority}{.level::cache_hint}{.level::prefetch_size}{.vec}.type  d, [a]{, cache_policy} => {
        todo!()
    }
    ld.mmio.relaxed.sys{.global}.type                                                                       d, [a] => {
        todo!()
    }

    .ss: StateSpace =                       { .const, .global, .local, .param{::entry, ::func}, .shared{::cta, ::cluster} };
    .cop: RawCacheOp =                      { .ca, .cg, .cs, .lu, .cv };
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

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#integer-arithmetic-instructions-add
    add.type        d, a, b => {
        todo!()
    }
    add{.sat}.s32   d, a, b => {
        todo!()
    }

    .type: ScalarType = { .u16, .u32, .u64,
                          .s16, .s64,
                          .u16x2, .s16x2 };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-add
    add{.rnd}{.ftz}{.sat}.f32  d, a, b => {
        todo!()
    }
    add{.rnd}.f64              d, a, b => {
        todo!()
    }

    .rnd: RawFloatRounding = { .rn, .rz, .rm, .rp };

    // https://docs.nvidia.com/cuda/parallel-thread-execution/#half-precision-floating-point-instructions-add
    add{.rnd}{.ftz}{.sat}.f16   d, a, b => {
        todo!()
    }
    add{.rnd}{.ftz}{.sat}.f16x2 d, a, b => {
        todo!()
    }
    add{.rnd}.bf16              d, a, b => {
        todo!()
    }
    add{.rnd}.bf16x2            d, a, b => {
        todo!()
    }

    .rnd: RawFloatRounding = { .rn };

    ret => {
        todo!()
    }

);

fn main() {
    use winnow::combinator::*;
    use winnow::token::*;
    use winnow::Parser;

    println!("{}", mem::size_of::<Token>());

    let mut input: &[Token] = &[][..];
    let x = opt(any::<_, ContextError>.verify_map(|t| {
        println!("MAP");
        Some(true)
    }))
    .parse_next(&mut input)
    .unwrap();
    dbg!(x);
    let lexer = Token::lexer(
        "
        ld.u64          temp, [in_addr];
        add.u64		    temp2, temp, 1;
        st.u64          [out_addr], temp2;
        ret;
        ",
    );
    let tokens = lexer.map(|t| t.unwrap()).collect::<Vec<_>>();
    println!("{:?}", &tokens);
    let mut stream = ParserState {
        input: &tokens[..],
        state: Vec::new(),
    };
    parse_instruction(&mut stream).unwrap();
    //parse_prefix(&mut lexer);
    let mut parser = &*tokens;
    println!("{}", mem::size_of::<Token>());
}
