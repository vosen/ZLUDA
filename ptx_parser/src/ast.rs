use std::intrinsics::unreachable;

use super::{MemScope, ScalarType, StateSpace, VectorPrefix};
use bitflags::bitflags;

pub enum Statement<P: Operand> {
    Label(P::Ident),
    Variable(MultiVariable<P::Ident>),
    Instruction(Option<PredAt<P::Ident>>, Instruction<P>),
    Block(Vec<Statement<P>>),
}

gen::generate_instruction_type!(
    pub enum Instruction<T> {
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
        Mul {
            type: { data.type_().into() },
            data: MulDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { data.dst_type().into() },
                },
                src1: T,
                src2: T,
            }
        },
        Ret {
            data: RetData
        },
        Trap { }
    }
);

pub struct MultiVariable<ID> {
    pub var: Variable<ID>,
    pub count: Option<u32>,
}

#[derive(Clone)]
pub struct Variable<ID> {
    pub align: Option<u32>,
    pub v_type: Type,
    pub state_space: StateSpace,
    pub name: ID,
    pub array_init: Vec<u8>,
}

pub struct PredAt<ID> {
    pub not: bool,
    pub label: ID,
}

pub trait Visitor<T> {
    fn visit(&mut self, args: &T, type_: &Type, space: StateSpace, is_dst: bool);
}

pub trait VisitorMut<T> {
    fn visit(&mut self, args: &mut T, type_: &Type, space: StateSpace, is_dst: bool);
}

pub trait VisitorMap<From, To> {
    fn visit(&mut self, args: From, type_: &Type, space: StateSpace, is_dst: bool) -> To;
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

impl Type {
    pub(crate) fn maybe_vector(vector: Option<VectorPrefix>, scalar: ScalarType) -> Self {
        match vector {
            Some(VectorPrefix::V2) => Type::Vector(scalar, 2),
            Some(VectorPrefix::V4) => Type::Vector(scalar, 4),
            None => Type::Scalar(scalar),
        }
    }
}

impl From<ScalarType> for Type {
    fn from(value: ScalarType) -> Self {
        Type::Scalar(value)
    }
}

#[derive(Clone)]
pub struct MovDetails {
    pub typ: super::Type,
    pub src_is_address: bool,
    // two fields below are in use by member moves
    pub dst_width: u8,
    pub src_width: u8,
    // This is in use by auto-generated movs
    pub relaxed_src2_conv: bool,
}

impl MovDetails {
    pub(crate) fn new(vector: Option<VectorPrefix>, scalar: ScalarType) -> Self {
        MovDetails {
            typ: Type::maybe_vector(vector, scalar),
            src_is_address: false,
            dst_width: 0,
            src_width: 0,
            relaxed_src2_conv: false,
        }
    }
}

#[derive(Clone)]
pub enum ParsedOperand<Ident> {
    Reg(Ident),
    RegOffset(Ident, i32),
    Imm(ImmediateValue),
    VecMember(Ident, u8),
    VecPack(Vec<Ident>),
}

impl<Ident> Operand for ParsedOperand<Ident> {
    type Ident = Ident;
}

pub trait Operand {
    type Ident;
}

#[derive(Copy, Clone)]
pub enum ImmediateValue {
    U64(u64),
    S64(i64),
    F32(f32),
    F64(f64),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum StCacheOperator {
    Writeback,
    L2Only,
    Streaming,
    Writethrough,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LdCacheOperator {
    Cached,
    L2Only,
    Streaming,
    LastUse,
    Uncached,
}

#[derive(Copy, Clone)]
pub enum ArithDetails {
    Integer(ArithInteger),
    Float(ArithFloat),
}

impl ArithDetails {
    pub fn type_(&self) -> ScalarType {
        match self {
            ArithDetails::Integer(t) => t.type_,
            ArithDetails::Float(arith) => arith.type_,
        }
    }
}

#[derive(Copy, Clone)]
pub struct ArithInteger {
    pub type_: ScalarType,
    pub saturate: bool,
}

#[derive(Copy, Clone)]
pub struct ArithFloat {
    pub type_: ScalarType,
    pub rounding: Option<RoundingMode>,
    pub flush_to_zero: Option<bool>,
    pub saturate: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LdStQualifier {
    Weak,
    Volatile,
    Relaxed(MemScope),
    Acquire(MemScope),
    Release(MemScope),
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum RoundingMode {
    NearestEven,
    Zero,
    NegativeInf,
    PositiveInf,
}

pub struct LdDetails {
    pub qualifier: LdStQualifier,
    pub state_space: StateSpace,
    pub caching: LdCacheOperator,
    pub typ: Type,
    pub non_coherent: bool,
}

pub struct StData {
    pub qualifier: LdStQualifier,
    pub state_space: StateSpace,
    pub caching: StCacheOperator,
    pub typ: Type,
}

#[derive(Copy, Clone)]
pub struct RetData {
    pub uniform: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TuningDirective {
    MaxNReg(u32),
    MaxNtid(u32, u32, u32),
    ReqNtid(u32, u32, u32),
    MinNCtaPerSm(u32),
}

pub struct MethodDeclaration<'input, ID> {
    pub return_arguments: Vec<Variable<ID>>,
    pub name: MethodName<'input, ID>,
    pub input_arguments: Vec<Variable<ID>>,
    pub shared_mem: Option<ID>,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub enum MethodName<'input, ID> {
    Kernel(&'input str),
    Func(ID),
}

bitflags! {
    pub struct LinkingDirective: u8 {
        const NONE = 0b000;
        const EXTERN = 0b001;
        const VISIBLE = 0b10;
        const WEAK = 0b100;
    }
}

pub struct Function<'a, ID, S> {
    pub func_directive: MethodDeclaration<'a, ID>,
    pub tuning: Vec<TuningDirective>,
    pub body: Option<Vec<S>>,
}

pub enum Directive<'input, O: Operand> {
    Variable(LinkingDirective, Variable<O::Ident>),
    Method(
        LinkingDirective,
        Function<'input, &'input str, Statement<O>>,
    ),
}

pub struct Module<'input> {
    pub version: (u8, u8),
    pub directives: Vec<Directive<'input, ParsedOperand<&'input str>>>,
}

#[derive(Copy, Clone)]
pub enum MulDetails {
    Integer {
        type_: ScalarType,
        control: MulIntControl,
    },
    Float(ArithFloat),
}

impl MulDetails {
    fn type_(&self) -> ScalarType {
        match self {
            MulDetails::Integer { type_, .. } => *type_,
            MulDetails::Float(arith) => arith.type_,
        }
    }

    fn dst_type(&self) -> ScalarType {
        match self {
            MulDetails::Integer {
                type_,
                control: MulIntControl::Wide,
            } => match type_ {
                ScalarType::U16 => ScalarType::U32,
                ScalarType::S16 => ScalarType::S32,
                ScalarType::U32 => ScalarType::U64,
                ScalarType::S32 => ScalarType::S64,
                _ => unreachable!(),
            },
            _ => self.type_(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MulIntControl {
    Low,
    High,
    Wide,
}
