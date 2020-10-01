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
        SyntaxError {}
        NonF32Ftz {}
        WrongArrayType {}
        WrongVectorElement {}
        MultiArrayVariable {}
    }
}

macro_rules! sub_scalar_type {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub enum $name {
            $(
                $variant,
            )+
        }

        impl From<$name> for ScalarType {
            fn from(t: $name) -> ScalarType {
                match t {
                    $(
                        $name::$variant => ScalarType::$variant,
                    )+
                }
            }
        }

        impl std::convert::TryFrom<ScalarType> for $name {
            type Error = ();

            fn try_from(t: ScalarType) -> Result<Self, Self::Error> {
                match t {
                    $(
                        ScalarType::$variant => Ok($name::$variant),
                    )+
                        _ => Err(()),
                }
            }
        }
    };
}

macro_rules! sub_type {
    ($type_name:ident { $($variant:ident ( $($field_type:ident),+ ) ),+ $(,)? } ) => {
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub enum $type_name {
            $(
                $variant ($($field_type),+),
            )+
        }

        impl From<$type_name> for Type {
            #[allow(non_snake_case)]
            fn from(t: $type_name) -> Type {
                match t {
                    $(
                        $type_name::$variant ( $($field_type),+ ) => Type::$variant ( $($field_type.into()),+),
                    )+
                }
            }
        }
    };
}

sub_type! {
    VariableRegType {
        Scalar(ScalarType),
        Vector(SizedScalarType, u8),
    }
}

sub_type! {
    VariableLocalType {
        Scalar(SizedScalarType),
        Vector(SizedScalarType, u8),
        Array(SizedScalarType, u32),
    }
}

// For some weird reson this is illegal:
//   .param .f16x2 foobar;
// but this is legal:
//   .param .f16x2 foobar[1];
sub_type! {
    VariableParamType {
        Scalar(ParamScalarType),
        Array(SizedScalarType, u32),
    }
}

sub_scalar_type!(SizedScalarType {
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
    F16x2,
    F32,
    F64,
});

sub_scalar_type!(ParamScalarType {
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
});

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
    pub functions: Vec<ParsedFunction<'a>>,
}

pub enum MethodDecl<'a, ID> {
    Func(Vec<FnArgument<ID>>, ID, Vec<FnArgument<ID>>),
    Kernel(&'a str, Vec<KernelArgument<ID>>),
}

pub type FnArgument<ID> = Variable<FnArgumentType, ID>;
pub type KernelArgument<ID> = Variable<VariableParamType, ID>;

pub struct Function<'a, ID, S> {
    pub func_directive: MethodDecl<'a, ID>,
    pub body: Option<Vec<S>>,
}

pub type ParsedFunction<'a> = Function<'a, &'a str, Statement<ParsedArgParams<'a>>>;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FnArgumentType {
    Reg(VariableRegType),
    Param(VariableParamType),
}

impl From<FnArgumentType> for Type {
    fn from(t: FnArgumentType) -> Self {
        match t {
            FnArgumentType::Reg(x) => x.into(),
            FnArgumentType::Param(x) => x.into(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Type {
    Scalar(ScalarType),
    Vector(ScalarType, u8),
    Array(ScalarType, u32),
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
    F16x2,
    Pred,
}

sub_scalar_type!(IntType {
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64
});

impl IntType {
    pub fn is_signed(self) -> bool {
        match self {
            IntType::U8 | IntType::U16 | IntType::U32 | IntType::U64 => false,
            IntType::S8 | IntType::S16 | IntType::S32 | IntType::S64 => true,
        }
    }

    pub fn width(self) -> u8 {
        match self {
            IntType::U8 => 1,
            IntType::U16 => 2,
            IntType::U32 => 4,
            IntType::U64 => 8,
            IntType::S8 => 1,
            IntType::S16 => 2,
            IntType::S32 => 4,
            IntType::S64 => 8,
        }
    }
}

sub_scalar_type!(FloatType {
    F16,
    F16x2,
    F32,
    F64
});

impl Default for ScalarType {
    fn default() -> Self {
        ScalarType::B8
    }
}

pub enum Statement<P: ArgParams> {
    Label(P::Id),
    Variable(MultiVariable<P::Id>),
    Instruction(Option<PredAt<P::Id>>, Instruction<P>),
    Block(Vec<Statement<P>>),
}

pub struct MultiVariable<ID> {
    pub var: Variable<VariableType, ID>,
    pub count: Option<u32>,
}

pub struct Variable<T, ID> {
    pub align: Option<u32>,
    pub v_type: T,
    pub name: ID,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum VariableType {
    Reg(VariableRegType),
    Local(VariableLocalType),
    Param(VariableParamType),
}

impl From<VariableType> for Type {
    fn from(t: VariableType) -> Self {
        match t {
            VariableType::Reg(t) => t.into(),
            VariableType::Local(t) => t.into(),
            VariableType::Param(t) => t.into(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum StateSpace {
    Reg,
    Sreg,
    Const,
    Global,
    Local,
    Shared,
    Param,
}

pub struct PredAt<ID> {
    pub not: bool,
    pub label: ID,
}

pub enum Instruction<P: ArgParams> {
    Ld(LdDetails, Arg2Ld<P>),
    Mov(MovDetails, Arg2Mov<P>),
    Mul(MulDetails, Arg3<P>),
    Add(AddDetails, Arg3<P>),
    Setp(SetpData, Arg4Setp<P>),
    SetpBool(SetpBoolData, Arg5<P>),
    Not(NotType, Arg2<P>),
    Bra(BraData, Arg1<P>),
    Cvt(CvtDetails, Arg2<P>),
    Cvta(CvtaDetails, Arg2<P>),
    Shl(ShlType, Arg3<P>),
    Shr(ShrType, Arg3<P>),
    St(StData, Arg2St<P>),
    Ret(RetData),
    Call(CallInst<P>),
    Abs(AbsDetails, Arg2<P>),
    Mad(MulDetails, Arg4<P>),
    Or(OrType, Arg3<P>),
}

#[derive(Copy, Clone)]
pub struct MadFloatDesc {}

#[derive(Copy, Clone)]
pub struct AbsDetails {
    pub flush_to_zero: bool,
    pub typ: ScalarType,
}

pub struct CallInst<P: ArgParams> {
    pub uniform: bool,
    pub ret_params: Vec<P::Id>,
    pub func: P::Id,
    pub param_list: Vec<P::CallOperand>,
}

pub trait ArgParams {
    type Id;
    type Operand;
    type IdOrVector;
    type OperandOrVector;
    type CallOperand;
    type SrcMemberOperand;
}

pub struct ParsedArgParams<'a> {
    _marker: PhantomData<&'a ()>,
}

impl<'a> ArgParams for ParsedArgParams<'a> {
    type Id = &'a str;
    type Operand = Operand<&'a str>;
    type CallOperand = CallOperand<&'a str>;
    type IdOrVector = IdOrVector<&'a str>;
    type OperandOrVector = OperandOrVector<&'a str>;
    type SrcMemberOperand = (&'a str, u8);
}

pub struct Arg1<P: ArgParams> {
    pub src: P::Id, // it is a jump destination, but in terms of operands it is a source operand
}

pub struct Arg2<P: ArgParams> {
    pub dst: P::Id,
    pub src: P::Operand,
}
pub struct Arg2Ld<P: ArgParams> {
    pub dst: P::IdOrVector,
    pub src: P::Operand,
}

pub struct Arg2St<P: ArgParams> {
    pub src1: P::Operand,
    pub src2: P::OperandOrVector,
}

pub enum Arg2Mov<P: ArgParams> {
    Normal(Arg2MovNormal<P>),
    Member(Arg2MovMember<P>),
}

pub struct Arg2MovNormal<P: ArgParams> {
    pub dst: P::IdOrVector,
    pub src: P::OperandOrVector,
}

// We duplicate dst here because during further compilation
// composite dst and composite src will receive different ids
pub enum Arg2MovMember<P: ArgParams> {
    Dst((P::Id, u8), P::Id, P::Id),
    Src(P::Id, P::SrcMemberOperand),
    Both((P::Id, u8), P::Id, P::SrcMemberOperand),
}

pub struct Arg3<P: ArgParams> {
    pub dst: P::Id,
    pub src1: P::Operand,
    pub src2: P::Operand,
}

pub struct Arg4<P: ArgParams> {
    pub dst: P::Id,
    pub src1: P::Operand,
    pub src2: P::Operand,
    pub src3: P::Operand,
}

pub struct Arg4Setp<P: ArgParams> {
    pub dst1: P::Id,
    pub dst2: Option<P::Id>,
    pub src1: P::Operand,
    pub src2: P::Operand,
}

pub struct Arg5<P: ArgParams> {
    pub dst1: P::Id,
    pub dst2: Option<P::Id>,
    pub src1: P::Operand,
    pub src2: P::Operand,
    pub src3: P::Operand,
}

#[derive(Copy, Clone)]
pub enum Operand<ID> {
    Reg(ID),
    RegOffset(ID, i32),
    Imm(u32),
}

#[derive(Copy, Clone)]
pub enum CallOperand<ID> {
    Reg(ID),
    Imm(u32),
}

pub enum IdOrVector<ID> {
    Reg(ID),
    Vec(Vec<ID>),
}

pub enum OperandOrVector<ID> {
    Reg(ID),
    RegOffset(ID, i32),
    Imm(u32),
    Vec(Vec<ID>),
}

impl<T> From<Operand<T>> for OperandOrVector<T> {
    fn from(this: Operand<T>) -> Self {
        match this {
            Operand::Reg(r) => OperandOrVector::Reg(r),
            Operand::RegOffset(r, imm) => OperandOrVector::RegOffset(r, imm),
            Operand::Imm(imm) => OperandOrVector::Imm(imm),
        }
    }
}

pub enum VectorPrefix {
    V2,
    V4,
}

pub struct LdDetails {
    pub qualifier: LdStQualifier,
    pub state_space: LdStateSpace,
    pub caching: LdCacheOperator,
    pub typ: Type,
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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

#[derive(Copy, Clone)]
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

pub enum MulDetails {
    Int(MulIntDesc),
    Float(MulFloatDesc),
}

#[derive(Copy, Clone)]
pub struct MulIntDesc {
    pub typ: IntType,
    pub control: MulIntControl,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MulIntControl {
    Low,
    High,
    Wide,
}

#[derive(Copy, Clone)]
pub struct MulFloatDesc {
    pub typ: FloatType,
    pub rounding: Option<RoundingMode>,
    pub flush_to_zero: bool,
    pub saturate: bool,
}

#[derive(PartialEq, Eq, Copy, Clone)]
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

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum NotType {
    Pred,
    B16,
    B32,
    B64,
}

pub struct BraData {
    pub uniform: bool,
}

pub enum CvtDetails {
    IntFromInt(CvtIntToIntDesc),
    FloatFromFloat(CvtDesc<FloatType, FloatType>),
    IntFromFloat(CvtDesc<IntType, FloatType>),
    FloatFromInt(CvtDesc<FloatType, IntType>),
}

pub struct CvtIntToIntDesc {
    pub dst: IntType,
    pub src: IntType,
    pub saturate: bool,
}

pub struct CvtDesc<Dst, Src> {
    pub rounding: Option<RoundingMode>,
    pub flush_to_zero: bool,
    pub saturate: bool,
    pub dst: Dst,
    pub src: Src,
}

impl CvtDetails {
    pub fn new_int_from_int_checked(
        saturate: bool,
        dst: IntType,
        src: IntType,
        err: &mut Vec<PtxError>,
    ) -> Self {
        if saturate {
            if src.is_signed() {
                if dst.is_signed() && dst.width() >= src.width() {
                    err.push(PtxError::SyntaxError);
                }
            } else {
                if dst == src || dst.width() >= src.width() {
                    err.push(PtxError::SyntaxError);
                }
            }
        }
        CvtDetails::IntFromInt(CvtIntToIntDesc { dst, src, saturate })
    }

    pub fn new_float_from_int_checked(
        rounding: RoundingMode,
        flush_to_zero: bool,
        saturate: bool,
        dst: FloatType,
        src: IntType,
        err: &mut Vec<PtxError>,
    ) -> Self {
        if flush_to_zero && dst != FloatType::F32 {
            err.push(PtxError::NonF32Ftz);
        }
        CvtDetails::FloatFromInt(CvtDesc {
            dst,
            src,
            saturate,
            flush_to_zero,
            rounding: Some(rounding),
        })
    }

    pub fn new_int_from_float_checked(
        rounding: RoundingMode,
        flush_to_zero: bool,
        saturate: bool,
        dst: IntType,
        src: FloatType,
        err: &mut Vec<PtxError>,
    ) -> Self {
        if flush_to_zero && src != FloatType::F32 {
            err.push(PtxError::NonF32Ftz);
        }
        CvtDetails::IntFromFloat(CvtDesc {
            dst,
            src,
            saturate,
            flush_to_zero,
            rounding: Some(rounding),
        })
    }
}

pub struct CvtaDetails {
    pub to: CvtaStateSpace,
    pub from: CvtaStateSpace,
    pub size: CvtaSize,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CvtaStateSpace {
    Generic,
    Const,
    Global,
    Local,
    Shared,
}

pub enum CvtaSize {
    U32,
    U64,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ShlType {
    B16,
    B32,
    B64,
}

sub_scalar_type!(ShrType {
    B16,
    B32,
    B64,
    U16,
    U32,
    U64,
    S16,
    S32,
    S64,
});

pub struct StData {
    pub qualifier: LdStQualifier,
    pub state_space: StStateSpace,
    pub caching: StCacheOperator,
    pub typ: Type,
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

sub_scalar_type!(OrType {
    Pred,
    B16,
    B32,
    B64,
});
