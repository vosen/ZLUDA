use gen::derive_parser;
use logos::Logos;
use std::mem;
use winnow::{
    error::{ContextError, ParserError},
    stream::{Offset, Stream, StreamIsPartial},
};

pub trait Operand {}

pub trait Visitor<T: Operand> {
    fn visit(&mut self, args: &T, type_: &Type, space: StateSpace, is_dst: bool);
}

pub trait VisitorMut<T: Operand> {
    fn visit(&mut self, args: &mut T, type_: &Type, space: StateSpace, is_dst: bool);
}

pub trait VisitorMap<From: Operand, To: Operand> {
    fn visit(&mut self, args: From, type_: &Type, space: StateSpace, is_dst: bool) -> To;
}

gen::generate_instruction_type!(
    enum Instruction<T: Operand> {
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

pub struct ParsedOperand {}

impl Operand for ParsedOperand {}

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

    #[inline(always)]
    fn iter_offsets(&self) -> Self::IterOffsets {
        self.0.iter().rev().cloned().enumerate()
    }

    #[inline(always)]
    fn eof_offset(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    fn next_token(&mut self) -> Option<Self::Token> {
        let (token, next) = self.0.split_last()?;
        self.0 = next;
        Some(token.clone())
    }

    #[inline(always)]
    fn offset_for<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Token) -> bool,
    {
        self.0.iter().rev().position(|b| predicate(b.clone()))
    }

    #[inline(always)]
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

    #[inline(always)]
    fn next_slice(&mut self, offset: usize) -> Self::Slice {
        let offset = self.0.len() - offset;
        let (next, slice) = self.0.split_at(offset);
        self.0 = next;
        slice
    }

    #[inline(always)]
    fn checkpoint(&self) -> Self::Checkpoint {
        self.0
    }

    #[inline(always)]
    fn reset(&mut self, checkpoint: &Self::Checkpoint) {
        self.0 = checkpoint;
    }

    #[inline(always)]
    fn raw(&self) -> &dyn std::fmt::Debug {
        self
    }
}

impl<'a, T> Offset<&'a [T]> for ReverseStream<'a, T> {
    #[inline]
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

// Modifiers are turned into arguments to the blocks, with type:
// * If it is an alternative:
//   * If it is mandatory then its type is Foo (as defined by the relevant rule)
//   * If it is optional then its type is Option<Foo>
// * Otherwise:
//   * If it is mandatory then it is skipped
//   * If it is optional then its type is `bool`

derive_parser!(
    #[derive(Logos, PartialEq, Eq, Debug, Clone, Copy)]
    #[logos(skip r"\s+")]
    enum Token<'input> {
        #[token(",")]
        Comma,
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
        #[regex(r"[0-9]+U?")]
        Decimal
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub enum StateSpace {
        Reg
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub enum MemScope { }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub enum ScalarType { }

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

    let mut input: &[Token] = &[][..];
    let x = opt(any::<_, ContextError>.verify_map(|t| { println!("MAP");Some(true) })).parse_next(&mut input).unwrap();
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
    let mut stream = &tokens[..];
    parse_instruction(&mut stream).unwrap();
    //parse_prefix(&mut lexer);
    let mut parser = &*tokens;
    println!("{}", mem::size_of::<Token>());
}
