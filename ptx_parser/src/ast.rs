use super::{MemScope, RawSetpCompareOp, ScalarType, SetpBoolPostOp, StateSpace, VectorPrefix};
use crate::{PtxError, PtxParserState};
use bitflags::bitflags;

pub enum Statement<P: Operand> {
    Label(P::Ident),
    Variable(MultiVariable<P::Ident>),
    Instruction(Option<PredAt<P::Ident>>, Instruction<P>),
    Block(Vec<Statement<P>>),
}

gen::generate_instruction_type!(
    pub enum Instruction<T: Operand> {
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
        Setp {
            data: SetpData,
            arguments<T>: {
                dst1: {
                    repr: T,
                    type: ScalarType::Pred.into()
                },
                dst2: {
                    repr: Option<T>,
                    type: ScalarType::Pred.into()
                },
                src1: {
                    repr: T,
                    type: data.type_.into(),
                },
                src2: {
                    repr: T,
                    type: data.type_.into(),
                }
            }
        },
        SetpBool {
            data: SetpBoolData,
            arguments<T>: {
                dst1: {
                    repr: T,
                    type: ScalarType::Pred.into()
                },
                dst2: {
                    repr: Option<T>,
                    type: ScalarType::Pred.into()
                },
                src1: {
                    repr: T,
                    type: data.base.type_.into(),
                },
                src2: {
                    repr: T,
                    type: data.base.type_.into(),
                },
                src3: {
                    repr: T,
                    type: ScalarType::Pred.into()
                }
            }
        },
        Ret {
            data: RetData
        },
        Trap { }
    }
);

pub trait Visitor<T> {
    fn visit(&mut self, args: &T, type_: &Type, space: StateSpace, is_dst: bool);
}

pub trait VisitorMut<T> {
    fn visit(&mut self, args: &mut T, type_: &Type, space: StateSpace, is_dst: bool);
}

pub trait VisitorMap<From, To> {
    fn visit(&mut self, args: From, type_: &Type, space: StateSpace, is_dst: bool) -> To;
}

trait VisitOperand {
    type Operand;
    fn visit(&self, fn_: impl FnOnce(&Self::Operand));
    fn visit_mut(&mut self, fn_: impl FnOnce(&mut Self::Operand));
}

impl<T: Operand> VisitOperand for T {
    type Operand = Self;
    fn visit(&self, fn_: impl FnOnce(&Self::Operand)) {
        fn_(self)
    }
    fn visit_mut(&mut self, fn_: impl FnOnce(&mut Self::Operand)) {
        fn_(self)
    }
}

impl<T: Operand> VisitOperand for Option<T> {
    type Operand = T;
    fn visit(&self, fn_: impl FnOnce(&Self::Operand)) {
        self.as_ref().map(fn_);
    }
    fn visit_mut(&mut self, fn_: impl FnOnce(&mut Self::Operand)) {
        self.as_mut().map(fn_);
    }
}

trait MapOperand: Sized {
    type Input;
    type Output<U>;
    fn map<U>(self, fn_: impl FnOnce(Self::Input) -> U) -> Self::Output<U>;
}

impl<T: Operand> MapOperand for T {
    type Input = Self;
    type Output<U> = U;
    fn map<U>(self, fn_: impl FnOnce(T) -> U) -> U {
        fn_(self)
    }
}

impl<T: Operand> MapOperand for Option<T> {
    type Input = T;
    type Output<U> = Option<U>;
    fn map<U>(self, fn_: impl FnOnce(T) -> U) -> Option<U> {
        self.map(|x| fn_(x))
    }
}

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

impl ScalarType {
    pub fn kind(self) -> ScalarKind {
        match self {
            ScalarType::U8 => ScalarKind::Unsigned,
            ScalarType::U16 => ScalarKind::Unsigned,
            ScalarType::U16x2 => ScalarKind::Unsigned,
            ScalarType::U32 => ScalarKind::Unsigned,
            ScalarType::U64 => ScalarKind::Unsigned,
            ScalarType::S8 => ScalarKind::Signed,
            ScalarType::S16 => ScalarKind::Signed,
            ScalarType::S16x2 => ScalarKind::Signed,
            ScalarType::S32 => ScalarKind::Signed,
            ScalarType::S64 => ScalarKind::Signed,
            ScalarType::B8 => ScalarKind::Bit,
            ScalarType::B16 => ScalarKind::Bit,
            ScalarType::B32 => ScalarKind::Bit,
            ScalarType::B64 => ScalarKind::Bit,
            ScalarType::B128 => ScalarKind::Bit,
            ScalarType::F16 => ScalarKind::Float,
            ScalarType::F16x2 => ScalarKind::Float,
            ScalarType::F32 => ScalarKind::Float,
            ScalarType::F64 => ScalarKind::Float,
            ScalarType::BF16 => ScalarKind::Float,
            ScalarType::BF16x2 => ScalarKind::Float,
            ScalarType::Pred => ScalarKind::Pred,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScalarKind {
    Bit,
    Unsigned,
    Signed,
    Float,
    Pred,
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

pub struct SetpData {
    pub type_: ScalarType,
    pub flush_to_zero: Option<bool>,
    pub cmp_op: SetpCompareOp,
}

impl SetpData {
    pub(crate) fn try_parse(
        errors: &mut PtxParserState,
        cmp_op: super::RawSetpCompareOp,
        ftz: bool,
        type_: ScalarType,
    ) -> Self {
        let flush_to_zero = match (ftz, type_) {
            (_, ScalarType::F32) => Some(ftz),
            _ => {
                errors.push(PtxError::NonF32Ftz);
                None
            }
        };
        let type_kind = type_.kind();
        let cmp_op = if type_kind == ScalarKind::Float {
            SetpCompareOp::Float(SetpCompareFloat::from(cmp_op))
        } else {
            match SetpCompareInt::try_from(cmp_op) {
                Ok(op) => SetpCompareOp::Integer(op),
                Err(err) => {
                    errors.push(err);
                    SetpCompareOp::Integer(SetpCompareInt::Eq)
                }
            }
        };
        Self {
            type_,
            flush_to_zero,
            cmp_op,
        }
    }
}

pub struct SetpBoolData {
    pub base: SetpData,
    pub bool_op: SetpBoolPostOp,
    pub negate_src3: bool
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SetpCompareOp {
    Integer(SetpCompareInt),
    Float(SetpCompareFloat),
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SetpCompareInt {
    Eq,
    NotEq,
    Less,
    LessOrEq,
    Greater,
    GreaterOrEq,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SetpCompareFloat {
    Eq,
    NotEq,
    Less,
    LessOrEq,
    Greater,
    GreaterOrEq,
    NanEq,
    NanNotEq,
    NanLess,
    NanLessOrEq,
    NanGreater,
    NanGreaterOrEq,
    IsNotNan,
    IsAnyNan,
}

impl TryFrom<RawSetpCompareOp> for SetpCompareInt {
    type Error = PtxError;

    fn try_from(value: RawSetpCompareOp) -> Result<Self, PtxError> {
        match value {
            RawSetpCompareOp::Eq => Ok(SetpCompareInt::Eq),
            RawSetpCompareOp::Ne => Ok(SetpCompareInt::NotEq),
            RawSetpCompareOp::Lt => Ok(SetpCompareInt::Less),
            RawSetpCompareOp::Le => Ok(SetpCompareInt::LessOrEq),
            RawSetpCompareOp::Gt => Ok(SetpCompareInt::Greater),
            RawSetpCompareOp::Ge => Ok(SetpCompareInt::GreaterOrEq),
            RawSetpCompareOp::Lo => Ok(SetpCompareInt::Less),
            RawSetpCompareOp::Ls => Ok(SetpCompareInt::LessOrEq),
            RawSetpCompareOp::Hi => Ok(SetpCompareInt::Greater),
            RawSetpCompareOp::Hs => Ok(SetpCompareInt::GreaterOrEq),
            RawSetpCompareOp::Equ => Err(PtxError::WrongType),
            RawSetpCompareOp::Neu => Err(PtxError::WrongType),
            RawSetpCompareOp::Ltu => Err(PtxError::WrongType),
            RawSetpCompareOp::Leu => Err(PtxError::WrongType),
            RawSetpCompareOp::Gtu => Err(PtxError::WrongType),
            RawSetpCompareOp::Geu => Err(PtxError::WrongType),
            RawSetpCompareOp::Num => Err(PtxError::WrongType),
            RawSetpCompareOp::Nan => Err(PtxError::WrongType),
        }
    }
}

impl From<RawSetpCompareOp> for SetpCompareFloat {
    fn from(value: RawSetpCompareOp) -> Self {
        match value {
            RawSetpCompareOp::Eq => SetpCompareFloat::Eq,
            RawSetpCompareOp::Ne => SetpCompareFloat::NotEq,
            RawSetpCompareOp::Lt => SetpCompareFloat::Less,
            RawSetpCompareOp::Le => SetpCompareFloat::LessOrEq,
            RawSetpCompareOp::Gt => SetpCompareFloat::Greater,
            RawSetpCompareOp::Ge => SetpCompareFloat::GreaterOrEq,
            RawSetpCompareOp::Lo => SetpCompareFloat::Less,
            RawSetpCompareOp::Ls => SetpCompareFloat::LessOrEq,
            RawSetpCompareOp::Hi => SetpCompareFloat::Greater,
            RawSetpCompareOp::Hs => SetpCompareFloat::GreaterOrEq,
            RawSetpCompareOp::Equ => SetpCompareFloat::NanEq,
            RawSetpCompareOp::Neu => SetpCompareFloat::NanNotEq,
            RawSetpCompareOp::Ltu => SetpCompareFloat::NanLess,
            RawSetpCompareOp::Leu => SetpCompareFloat::NanLessOrEq,
            RawSetpCompareOp::Gtu => SetpCompareFloat::NanGreater,
            RawSetpCompareOp::Geu => SetpCompareFloat::NanGreaterOrEq,
            RawSetpCompareOp::Num => SetpCompareFloat::IsNotNan,
            RawSetpCompareOp::Nan => SetpCompareFloat::IsAnyNan,
        }
    }
}
