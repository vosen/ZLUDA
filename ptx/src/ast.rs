use std::convert::From;
use std::{marker::PhantomData, num::ParseIntError};

quick_error! {
    #[derive(Debug)]
    pub enum PtxError {
        ParseInt (err: ParseIntError) {
            from()
            display("{}", err)
            cause(err)
        }
    }
}

pub trait UnwrapWithVec<E, To> {
    fn unwrap_with(self, errs: &mut Vec<E>) -> To;
}

impl<R: Default, EFrom: std::convert::Into<EInto>, EInto> UnwrapWithVec<EInto, R>
    for Result<R, EFrom>
{
    fn unwrap_with(self, errs: &mut Vec<EInto>) -> R {
        self.unwrap_or_else(|e| {
            errs.push(e.into());
            R::default()
        })
    }
}

impl<
        R1: Default,
        EFrom1: std::convert::Into<EInto>,
        R2: Default,
        EFrom2: std::convert::Into<EInto>,
        EInto,
    > UnwrapWithVec<EInto, (R1, R2)> for (Result<R1, EFrom1>, Result<R2, EFrom2>)
{
    fn unwrap_with(self, errs: &mut Vec<EInto>) -> (R1, R2) {
        let (x, y) = self;
        let r1 = x.unwrap_with(errs);
        let r2 = y.unwrap_with(errs);
        (r1, r2)
    }
}

pub struct Module<'a> {
    pub version: (u8, u8),
    pub functions: Vec<Function<'a>>,
}

pub struct Function<'a> {
    pub kernel: bool,
    pub name: &'a str,
    pub args: Vec<Argument<'a>>,
    pub body: Vec<Statement<ParsedArgParams<'a>>>,
}

#[derive(Default)]
pub struct Argument<'a> {
    pub name: &'a str,
    pub a_type: ScalarType,
    pub length: u32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Type {
    Scalar(ScalarType),
    ExtendedScalar(ExtendedScalarType),
}

impl From<FloatType> for Type {
    fn from(t: FloatType) -> Self {
        match t {
            FloatType::F16 => Type::Scalar(ScalarType::F16),
            FloatType::F16x2 => Type::ExtendedScalar(ExtendedScalarType::F16x2),
            FloatType::F32 => Type::Scalar(ScalarType::F32),
            FloatType::F64 => Type::Scalar(ScalarType::F64),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ScalarType {
    B8,
    B16,
    B32,
    B64,
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    F16,
    F32,
    F64,
}

impl From<IntType> for ScalarType {
    fn from(t: IntType) -> Self {
        match t {
            IntType::S16 => ScalarType::S16,
            IntType::S32 => ScalarType::S32,
            IntType::S64 => ScalarType::S64,
            IntType::U16 => ScalarType::U16,
            IntType::U32 => ScalarType::U32,
            IntType::U64 => ScalarType::U64,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum IntType {
    U16,
    U32,
    U64,
    S16,
    S32,
    S64,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum FloatType {
    F16,
    F16x2,
    F32,
    F64,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ExtendedScalarType {
    F16x2,
    Pred,
}

impl Default for ScalarType {
    fn default() -> Self {
        ScalarType::B8
    }
}

pub enum Statement<P: ArgParams> {
    Label(P::ID),
    Variable(Variable<P>),
    Instruction(Option<PredAt<P::ID>>, Instruction<P>),
}

pub struct Variable<P: ArgParams> {
    pub space: StateSpace,
    pub v_type: Type,
    pub name: P::ID,
    pub count: Option<u32>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum StateSpace {
    Reg,
    Sreg,
    Const,
    Global,
    Local,
    Shared,
}

pub struct PredAt<ID> {
    pub not: bool,
    pub label: ID,
}

pub enum Instruction<P: ArgParams> {
    Ld(LdData, Arg2<P>),
    Mov(MovData, Arg2Mov<P>),
    Mul(MulDetails, Arg3<P>),
    Add(AddDetails, Arg3<P>),
    Setp(SetpData, Arg4<P>),
    SetpBool(SetpBoolData, Arg5<P>),
    Not(NotData, Arg2<P>),
    Bra(BraData, Arg1<P>),
    Cvt(CvtData, Arg2<P>),
    Shl(ShlData, Arg3<P>),
    St(StData, Arg2St<P>),
    Ret(RetData),
}

pub trait ArgParams {
    type ID;
    type Operand;
    type MovOperand;
}

pub struct ParsedArgParams<'a> {
    _marker: PhantomData<&'a ()>,
}

impl<'a> ArgParams for ParsedArgParams<'a> {
    type ID = &'a str;
    type Operand = Operand<&'a str>;
    type MovOperand = MovOperand<&'a str>;
}

pub struct Arg1<P: ArgParams> {
    pub src: P::ID, // it is a jump destination, but in terms of operands it is a source operand
}

pub struct Arg2<P: ArgParams> {
    pub dst: P::ID,
    pub src: P::Operand,
}

pub struct Arg2St<P: ArgParams> {
    pub src1: P::Operand,
    pub src2: P::Operand,
}

pub struct Arg2Mov<P: ArgParams> {
    pub dst: P::ID,
    pub src: P::MovOperand,
}

pub struct Arg3<P: ArgParams> {
    pub dst: P::ID,
    pub src1: P::Operand,
    pub src2: P::Operand,
}

pub struct Arg4<P: ArgParams> {
    pub dst1: P::ID,
    pub dst2: Option<P::ID>,
    pub src1: P::Operand,
    pub src2: P::Operand,
}

pub struct Arg5<P: ArgParams> {
    pub dst1: P::ID,
    pub dst2: Option<P::ID>,
    pub src1: P::Operand,
    pub src2: P::Operand,
    pub src3: P::Operand,
}

pub enum Operand<ID> {
    Reg(ID),
    RegOffset(ID, i32),
    Imm(i128),
}

pub enum MovOperand<ID> {
    Op(Operand<ID>),
    Vec(String, String),
}

pub enum VectorPrefix {
    V2,
    V4,
}

pub struct LdData {
    pub qualifier: LdStQualifier,
    pub state_space: LdStateSpace,
    pub caching: LdCacheOperator,
    pub vector: Option<VectorPrefix>,
    pub typ: ScalarType,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LdStQualifier {
    Weak,
    Volatile,
    Relaxed(LdScope),
    Acquire(LdScope),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LdScope {
    Cta,
    Gpu,
    Sys,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LdStateSpace {
    Generic,
    Const,
    Global,
    Local,
    Param,
    Shared,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LdCacheOperator {
    Cached,
    L2Only,
    Streaming,
    LastUse,
    Uncached,
}

pub struct MovData {
    pub typ: Type,
}

pub enum MulDetails {
    Int(MulIntDesc),
    Float(MulFloatDesc),
}

pub struct MulIntDesc {
    pub typ: IntType,
    pub control: MulIntControl,
}

pub enum MulIntControl {
    Low,
    High,
    Wide,
}

pub struct MulFloatDesc {
    pub typ: FloatType,
    pub rounding: Option<RoundingMode>,
    pub flush_to_zero: bool,
    pub saturate: bool,
}

pub enum RoundingMode {
    NearestEven,
    Zero,
    NegativeInf,
    PositiveInf,
}

pub enum AddDetails {
    Int(AddIntDesc),
    Float(AddFloatDesc),
}

pub struct AddIntDesc {
    pub typ: IntType,
    pub saturate: bool,
}

pub struct AddFloatDesc {
    pub typ: FloatType,
    pub rounding: Option<RoundingMode>,
    pub flush_to_zero: bool,
    pub saturate: bool,
}

pub struct SetpData {
    pub typ: ScalarType,
    pub flush_to_zero: bool,
    pub cmp_op: SetpCompareOp,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SetpCompareOp {
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
    IsNan,
}

pub enum SetpBoolPostOp {
    And,
    Or,
    Xor,
}

pub struct SetpBoolData {
    pub typ: ScalarType,
    pub flush_to_zero: bool,
    pub cmp_op: SetpCompareOp,
    pub bool_op: SetpBoolPostOp,
}

pub struct NotData {}

pub struct BraData {
    pub uniform: bool,
}

pub struct CvtData {}

pub struct ShlData {}

pub struct StData {
    pub qualifier: LdStQualifier,
    pub state_space: StStateSpace,
    pub caching: StCacheOperator,
    pub vector: Option<VectorPrefix>,
    pub typ: ScalarType,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum StStateSpace {
    Generic,
    Global,
    Local,
    Param,
    Shared,
}

#[derive(PartialEq, Eq)]
pub enum StCacheOperator {
    Writeback,
    L2Only,
    Streaming,
    Writethrough,
}

pub struct RetData {
    pub uniform: bool,
}
