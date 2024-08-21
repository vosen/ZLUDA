use std::cmp::Ordering;

use super::{
    MemScope, RawRoundingMode, RawSetpCompareOp, ScalarType, SetpBoolPostOp, StateSpace,
    VectorPrefix,
};
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
            type: { Type::from(data.type_()) },
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
            type: { Type::from(data.type_()) },
            data: MulDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { Type::from(data.dst_type()) },
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
                    type: Type::from(ScalarType::Pred)
                },
                dst2: {
                    repr: Option<T>,
                    type: Type::from(ScalarType::Pred)
                },
                src1: {
                    repr: T,
                    type: Type::from(data.type_),
                },
                src2: {
                    repr: T,
                    type: Type::from(data.type_),
                }
            }
        },
        SetpBool {
            data: SetpBoolData,
            arguments<T>: {
                dst1: {
                    repr: T,
                    type: Type::from(ScalarType::Pred)
                },
                dst2: {
                    repr: Option<T>,
                    type: Type::from(ScalarType::Pred)
                },
                src1: {
                    repr: T,
                    type: Type::from(data.base.type_),
                },
                src2: {
                    repr: T,
                    type: Type::from(data.base.type_),
                },
                src3: {
                    repr: T,
                    type: Type::from(ScalarType::Pred)
                }
            }
        },
        Not {
            data: ScalarType,
            type: { Type::Scalar(data.clone()) },
            arguments<T>: {
                dst: T,
                src: T,
            }
        },
        Or {
            data: ScalarType,
            type: { Type::Scalar(data.clone()) },
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
            }
        },
        And {
            data: ScalarType,
            type: { Type::Scalar(data.clone()) },
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
            }
        },
        Bra {
            type: !,
            arguments<T::Ident>: {
                src: T
            }
        },
        Call {
            data: CallDetails,
            arguments: CallArgs<T>,
            visit: arguments.visit(data, visitor),
            visit_mut: arguments.visit_mut(data, visitor),
            map: Instruction::Call{ arguments: arguments.map(&data, visitor), data }
        },
        Cvt {
            data: CvtDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { Type::Scalar(data.to) },
                },
                src: {
                    repr: T,
                    type: { Type::Scalar(data.from) },
                },
            }
        },
        Shr {
            data: ShrData,
            type: { Type::Scalar(data.type_.clone()) },
            arguments<T>: {
                dst: T,
                src1: T,
                src2: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::U32) },
                },
            }
        },
        Shl {
            data: ScalarType,
            type: { Type::Scalar(data.clone()) },
            arguments<T>: {
                dst: T,
                src1: T,
                src2: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::U32) },
                },
            }
        },
        Ret {
            data: RetData
        },
        Cvta {
            data: CvtaDetails,
            type: { Type::Scalar(ScalarType::B64) },
            arguments<T>: {
                dst: T,
                src: T,
            }
        },
        Abs {
            data: AbsDetails,
            type: { Type::Scalar(data.type_) },
            arguments<T>: {
                dst: T,
                src: T,
            }
        },
        Mad {
            type: { Type::from(data.type_()) },
            data: MadDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { Type::from(data.dst_type()) },
                },
                src1: T,
                src2: T,
                src3: T,
            }
        },
        Fma {
            type: { Type::from(data.type_) },
            data: ArithFloat,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
                src3: T,
            }
        },
        Sub {
            type: { Type::from(data.type_()) },
            data: ArithDetails,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
            }
        },
        Min {
            type: { Type::from(data.type_()) },
            data: MinMaxDetails,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
            }
        },
        Max {
            type: { Type::from(data.type_()) },
            data: MinMaxDetails,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
            }
        },
        Rcp {
            type: { Type::from(data.type_) },
            data: RcpData,
            arguments<T>: {
                dst: T,
                src: T,
            }
        },
        Sqrt {
            type: { Type::from(data.type_) },
            data: RcpData,
            arguments<T>: {
                dst: T,
                src: T,
            }
        },
        Rsqrt {
            type: { Type::from(data.type_) },
            data: RsqrtData,
            arguments<T>: {
                dst: T,
                src: T,
            }
        },
        Trap { }
    }
);

pub trait Visitor<T: Operand> {
    fn visit(&mut self, args: &T, type_space: Option<(&Type, StateSpace)>, is_dst: bool);
    fn visit_ident(&self, args: &T::Ident, type_space: Option<(&Type, StateSpace)>, is_dst: bool);
}

pub trait VisitorMut<T: Operand> {
    fn visit(&mut self, args: &mut T, type_space: Option<(&Type, StateSpace)>, is_dst: bool);
    fn visit_ident(
        &mut self,
        args: &mut T::Ident,
        type_space: Option<(&Type, StateSpace)>,
        is_dst: bool,
    );
}

pub trait VisitorMap<From: Operand, To: Operand> {
    fn visit(&mut self, args: From, type_space: Option<(&Type, StateSpace)>, is_dst: bool) -> To;
    fn visit_ident(
        &mut self,
        args: From::Ident,
        type_space: Option<(&Type, StateSpace)>,
        is_dst: bool,
    ) -> To::Ident;
}

trait VisitOperand {
    type Operand: Operand;
    #[allow(unused)] // Used by generated code
    fn visit(&self, fn_: impl FnMut(&Self::Operand));
    #[allow(unused)] // Used by generated code
    fn visit_mut(&mut self, fn_: impl FnMut(&mut Self::Operand));
}

impl<T: Operand> VisitOperand for T {
    type Operand = Self;
    fn visit(&self, mut fn_: impl FnMut(&Self::Operand)) {
        fn_(self)
    }
    fn visit_mut(&mut self, mut fn_: impl FnMut(&mut Self::Operand)) {
        fn_(self)
    }
}

impl<T: Operand> VisitOperand for Option<T> {
    type Operand = T;
    fn visit(&self, fn_: impl FnMut(&Self::Operand)) {
        self.as_ref().map(fn_);
    }
    fn visit_mut(&mut self, fn_: impl FnMut(&mut Self::Operand)) {
        self.as_mut().map(fn_);
    }
}

impl<T: Operand> VisitOperand for Vec<T> {
    type Operand = T;
    fn visit(&self, mut fn_: impl FnMut(&Self::Operand)) {
        for o in self {
            fn_(o)
        }
    }
    fn visit_mut(&mut self, mut fn_: impl FnMut(&mut Self::Operand)) {
        for o in self {
            fn_(o)
        }
    }
}

trait MapOperand: Sized {
    type Input;
    type Output<U>;
    #[allow(unused)] // Used by generated code
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
    pub fn size_of(self) -> u8 {
        match self {
            ScalarType::U8 | ScalarType::S8 | ScalarType::B8 => 1,
            ScalarType::U16
            | ScalarType::S16
            | ScalarType::B16
            | ScalarType::F16
            | ScalarType::BF16 => 2,
            ScalarType::U32
            | ScalarType::S32
            | ScalarType::B32
            | ScalarType::F32
            | ScalarType::U16x2
            | ScalarType::S16x2
            | ScalarType::F16x2
            | ScalarType::BF16x2 => 4,
            ScalarType::U64 | ScalarType::S64 | ScalarType::B64 | ScalarType::F64 => 8,
            ScalarType::B128 => 16,
            ScalarType::Pred => 1,
        }
    }

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

impl<Ident: Copy> Operand for ParsedOperand<Ident> {
    type Ident = Ident;
}

pub trait Operand {
    type Ident: Copy;
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
    pub fn type_(&self) -> ScalarType {
        match self {
            MulDetails::Integer { type_, .. } => *type_,
            MulDetails::Float(arith) => arith.type_,
        }
    }

    pub fn dst_type(&self) -> ScalarType {
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
        state: &mut PtxParserState,
        cmp_op: super::RawSetpCompareOp,
        ftz: bool,
        type_: ScalarType,
    ) -> Self {
        let flush_to_zero = match (ftz, type_) {
            (_, ScalarType::F32) => Some(ftz),
            _ => {
                state.errors.push(PtxError::NonF32Ftz);
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
                    state.errors.push(err);
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
    pub negate_src3: bool,
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

pub struct CallDetails {
    pub uniform: bool,
    pub return_arguments: Vec<(Type, StateSpace)>,
    pub input_arguments: Vec<(Type, StateSpace)>,
}

pub struct CallArgs<T: Operand> {
    pub return_arguments: Vec<T::Ident>,
    pub func: T::Ident,
    pub input_arguments: Vec<T>,
}

impl<T: Operand> CallArgs<T> {
    #[allow(dead_code)] // Used by generated code
    fn visit(&self, details: &CallDetails, visitor: &mut impl Visitor<T>) {
        for (param, (type_, space)) in self
            .return_arguments
            .iter()
            .zip(details.return_arguments.iter())
        {
            visitor.visit_ident(param, Some((type_, *space)), true);
        }
        visitor.visit_ident(&self.func, None, false);
        for (param, (type_, space)) in self
            .input_arguments
            .iter()
            .zip(details.input_arguments.iter())
        {
            visitor.visit(param, Some((type_, *space)), true);
        }
    }

    #[allow(dead_code)] // Used by generated code
    fn visit_mut(&mut self, details: &CallDetails, visitor: &mut impl VisitorMut<T>) {
        for (param, (type_, space)) in self
            .return_arguments
            .iter_mut()
            .zip(details.return_arguments.iter())
        {
            visitor.visit_ident(param, Some((type_, *space)), true);
        }
        visitor.visit_ident(&mut self.func, None, false);
        for (param, (type_, space)) in self
            .input_arguments
            .iter_mut()
            .zip(details.input_arguments.iter())
        {
            visitor.visit(param, Some((type_, *space)), true);
        }
    }

    #[allow(dead_code)] // Used by generated code
    fn map<U: Operand>(
        self,
        details: &CallDetails,
        visitor: &mut impl VisitorMap<T, U>,
    ) -> CallArgs<U> {
        let return_arguments = self
            .return_arguments
            .into_iter()
            .zip(details.return_arguments.iter())
            .map(|(param, (type_, space))| visitor.visit_ident(param, Some((type_, *space)), true))
            .collect::<Vec<_>>();
        let func = visitor.visit_ident(self.func, None, false);
        let input_arguments = self
            .input_arguments
            .into_iter()
            .zip(details.input_arguments.iter())
            .map(|(param, (type_, space))| visitor.visit(param, Some((type_, *space)), true))
            .collect::<Vec<_>>();
        CallArgs {
            return_arguments,
            func,
            input_arguments,
        }
    }
}

pub struct CvtDetails {
    pub from: ScalarType,
    pub to: ScalarType,
    pub mode: CvtMode,
}

pub enum CvtMode {
    // int from int
    ZeroExtend,
    SignExtend,
    Truncate,
    Bitcast,
    // float from float
    FPExtend {
        flush_to_zero: Option<bool>,
    },
    FPTruncate {
        // float rounding
        rounding: RoundingMode,
        flush_to_zero: Option<bool>,
    },
    FPRound {
        integer_rounding: Option<RoundingMode>,
        flush_to_zero: Option<bool>,
    },
    // int from float
    SignedFromFP {
        rounding: RoundingMode,
        flush_to_zero: Option<bool>,
    }, // integer rounding
    UnsignedFromFP {
        rounding: RoundingMode,
        flush_to_zero: Option<bool>,
    }, // integer rounding
    // float from int, ftz is allowed in the grammar, but clearly nonsensical
    FPFromSigned(RoundingMode),   // float rounding
    FPFromUnsigned(RoundingMode), // float rounding
}

impl CvtDetails {
    pub(crate) fn new(
        errors: &mut Vec<PtxError>,
        rnd: Option<RawRoundingMode>,
        ftz: bool,
        saturate: bool,
        dst: ScalarType,
        src: ScalarType,
    ) -> Self {
        if saturate {
            errors.push(PtxError::Todo);
        }
        // Modifier .ftz can only be specified when either .dtype or .atype is .f32 and applies only to single precision (.f32) inputs and results.
        let flush_to_zero = match (dst, src) {
            (ScalarType::F32, _) | (_, ScalarType::F32) => Some(ftz),
            _ => {
                if ftz {
                    errors.push(PtxError::NonF32Ftz);
                }
                None
            }
        };
        let rounding = rnd.map(Into::into);
        let mut unwrap_rounding = || match rounding {
            Some(rnd) => rnd,
            None => {
                errors.push(PtxError::SyntaxError);
                RoundingMode::NearestEven
            }
        };
        let mode = match (dst.kind(), src.kind()) {
            (ScalarKind::Float, ScalarKind::Float) => match dst.size_of().cmp(&src.size_of()) {
                Ordering::Less => CvtMode::FPTruncate {
                    rounding: unwrap_rounding(),
                    flush_to_zero,
                },
                Ordering::Equal => CvtMode::FPRound {
                    integer_rounding: rounding,
                    flush_to_zero,
                },
                Ordering::Greater => {
                    if rounding.is_some() {
                        errors.push(PtxError::SyntaxError);
                    }
                    CvtMode::FPExtend { flush_to_zero }
                }
            },
            (ScalarKind::Unsigned, ScalarKind::Float) => CvtMode::UnsignedFromFP {
                rounding: unwrap_rounding(),
                flush_to_zero,
            },
            (ScalarKind::Signed, ScalarKind::Float) => CvtMode::SignedFromFP {
                rounding: unwrap_rounding(),
                flush_to_zero,
            },
            (ScalarKind::Float, ScalarKind::Unsigned) => CvtMode::FPFromUnsigned(unwrap_rounding()),
            (ScalarKind::Float, ScalarKind::Signed) => CvtMode::FPFromSigned(unwrap_rounding()),
            (
                ScalarKind::Unsigned | ScalarKind::Signed,
                ScalarKind::Unsigned | ScalarKind::Signed,
            ) => match dst.size_of().cmp(&src.size_of()) {
                Ordering::Less => {
                    if dst.kind() != src.kind() {
                        errors.push(PtxError::Todo);
                    }
                    CvtMode::Truncate
                }
                Ordering::Equal => CvtMode::Bitcast,
                Ordering::Greater => {
                    if dst.kind() != src.kind() {
                        errors.push(PtxError::Todo);
                    }
                    if src.kind() == ScalarKind::Signed {
                        CvtMode::SignExtend
                    } else {
                        CvtMode::ZeroExtend
                    }
                }
            },
            (_, _) => {
                errors.push(PtxError::SyntaxError);
                CvtMode::Bitcast
            }
        };
        CvtDetails {
            mode,
            to: dst,
            from: src,
        }
    }
}

pub struct CvtIntToIntDesc {
    pub dst: ScalarType,
    pub src: ScalarType,
    pub saturate: bool,
}

pub struct CvtDesc {
    pub rounding: Option<RoundingMode>,
    pub flush_to_zero: Option<bool>,
    pub saturate: bool,
    pub dst: ScalarType,
    pub src: ScalarType,
}

pub struct ShrData {
    pub type_: ScalarType,
    pub kind: RightShiftKind,
}

pub enum RightShiftKind {
    Arithmetic,
    Logical,
}

pub struct CvtaDetails {
    pub state_space: StateSpace,
    pub direction: CvtaDirection,
}

pub enum CvtaDirection {
    GenericToExplicit,
    ExplicitToGeneric,
}

#[derive(Copy, Clone)]
pub struct AbsDetails {
    pub flush_to_zero: Option<bool>,
    pub type_: ScalarType,
}

#[derive(Copy, Clone)]
pub enum MadDetails {
    Integer {
        control: MulIntControl,
        saturate: bool,
        type_: ScalarType,
    },
    Float(ArithFloat),
}

impl MadDetails {
    pub fn dst_type(&self) -> ScalarType {
        match self {
            MadDetails::Integer {
                type_,
                control: MulIntControl::Wide,
                ..
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

    fn type_(&self) -> ScalarType {
        match self {
            MadDetails::Integer { type_, .. } => *type_,
            MadDetails::Float(arith) => arith.type_,
        }
    }
}

#[derive(Copy, Clone)]
pub enum MinMaxDetails {
    Signed(ScalarType),
    Unsigned(ScalarType),
    Float(MinMaxFloat),
}

impl MinMaxDetails {
    pub fn type_(&self) -> ScalarType {
        match self {
            MinMaxDetails::Signed(t) => *t,
            MinMaxDetails::Unsigned(t) => *t,
            MinMaxDetails::Float(float) => float.type_,
        }
    }
}

#[derive(Copy, Clone)]
pub struct MinMaxFloat {
    pub flush_to_zero: Option<bool>,
    pub nan: bool,
    pub type_: ScalarType,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DivFloatKind {
    Approx,
    Full,
    Rounding(RoundingMode),
}

#[derive(Copy, Clone)]
pub struct RcpData {
    pub kind: RcpKind,
    pub flush_to_zero: Option<bool>,
    pub type_: ScalarType,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RcpKind {
    Approx,
    Full(RoundingMode),
}

#[derive(Copy, Clone)]
pub struct RsqrtData {
    pub flush_to_zero: Option<bool>,
    pub type_: ScalarType,
}
