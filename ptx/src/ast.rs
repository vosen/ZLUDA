use half::f16;
use lalrpop_util::{lexer::Token, ParseError};
use std::convert::TryInto;
use std::{convert::From, convert::TryFrom, mem, num::ParseFloatError, str::FromStr};
use std::{marker::PhantomData, num::ParseIntError};

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
}

macro_rules! sub_type {
    ($type_name:ident { $($variant:ident ( $($field_type:ident),+ ) ),+ $(,)? } ) => {
        sub_type! { $type_name : Type {
            $(
                $variant ($($field_type),+),
            )+
         }}
    };
    ($type_name:ident : $base_type:ident { $($variant:ident ( $($field_type:ident),+ ) ),+ $(,)? } ) => {
        #[derive(PartialEq, Eq, Clone)]
        pub enum $type_name {
            $(
                $variant ($($field_type),+),
            )+
        }

        impl From<$type_name> for $base_type {
            #[allow(non_snake_case)]
            fn from(t: $type_name) -> $base_type {
                match t {
                    $(
                        $type_name::$variant ( $($field_type),+ ) => <$base_type>::$variant ( $($field_type.into()),+),
                    )+
                }
            }
        }

        impl std::convert::TryFrom<$base_type> for $type_name {
            type Error = ();

            #[allow(non_snake_case)]
            #[allow(unreachable_patterns)]
            fn try_from(t: $base_type) -> Result<Self, Self::Error> {
                match t {
                    $(
                        $base_type::$variant ( $($field_type),+ )  => Ok($type_name::$variant ( $($field_type.try_into().map_err(|_| ())? ),+ )),
                    )+
                        _ => Err(()),
                }
            }
        }
    };
}

sub_type! {
    VariableRegType {
        Scalar(ScalarType),
        Vector(ScalarType, u8),
        // Array type is used when emiting SSA statements at the start of a method
        Array(ScalarType, VecU32),
        // Pointer variant is used when passing around SLM pointer between
        // function calls for dynamic SLM
        Pointer(ScalarType, LdStateSpace)
    }
}

type VecU32 = Vec<u32>;

sub_type! {
    VariableLocalType {
        Scalar(ScalarType),
        Vector(ScalarType, u8),
        Array(ScalarType, VecU32),
    }
}

impl TryFrom<VariableGlobalType> for VariableLocalType {
    type Error = PtxError;

    fn try_from(value: VariableGlobalType) -> Result<Self, Self::Error> {
        match value {
            VariableGlobalType::Scalar(t) => Ok(VariableLocalType::Scalar(t)),
            VariableGlobalType::Vector(t, len) => Ok(VariableLocalType::Vector(t, len)),
            VariableGlobalType::Array(t, len) => Ok(VariableLocalType::Array(t, len)),
            VariableGlobalType::Pointer(_, _) => Err(PtxError::ZeroDimensionArray),
        }
    }
}

sub_type! {
    VariableGlobalType {
        Scalar(ScalarType),
        Vector(ScalarType, u8),
        Array(ScalarType, VecU32),
        Pointer(ScalarType, LdStateSpace),
    }
}

// For some weird reson this is illegal:
//   .param .f16x2 foobar;
// but this is legal:
//   .param .f16x2 foobar[1];
// even more interestingly this is legal, but only in .func (not in .entry):
//   .param .b32 foobar[]
sub_type! {
    VariableParamType {
        Scalar(ScalarType),
        Array(ScalarType, VecU32),
        Pointer(ScalarType, LdStateSpace),
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BarDetails {
    SyncAligned,
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
    pub directives: Vec<Directive<'a, ParsedArgParams<'a>>>,
}

pub enum Directive<'a, P: ArgParams> {
    Variable(Variable<VariableType, P::Id>),
    Method(Function<'a, &'a str, Statement<P>>),
}

pub enum MethodDecl<'a, ID> {
    Func(Vec<FnArgument<ID>>, ID, Vec<FnArgument<ID>>),
    Kernel {
        name: &'a str,
        in_args: Vec<KernelArgument<ID>>,
    },
}

pub type FnArgument<ID> = Variable<FnArgumentType, ID>;
pub type KernelArgument<ID> = Variable<KernelArgumentType, ID>;

pub struct Function<'a, ID, S> {
    pub func_directive: MethodDecl<'a, ID>,
    pub tuning: Vec<TuningDirective>,
    pub body: Option<Vec<S>>,
}

pub type ParsedFunction<'a> = Function<'a, &'a str, Statement<ParsedArgParams<'a>>>;

#[derive(PartialEq, Eq, Clone)]
pub enum FnArgumentType {
    Reg(VariableRegType),
    Param(VariableParamType),
    Shared,
}
#[derive(PartialEq, Eq, Clone)]
pub enum KernelArgumentType {
    Normal(VariableParamType),
    Shared,
}

impl From<KernelArgumentType> for Type {
    fn from(this: KernelArgumentType) -> Self {
        match this {
            KernelArgumentType::Normal(typ) => typ.into(),
            KernelArgumentType::Shared => {
                Type::Pointer(PointerType::Scalar(ScalarType::B8), LdStateSpace::Shared)
            }
        }
    }
}

impl FnArgumentType {
    pub fn to_type(&self, is_kernel: bool) -> Type {
        if is_kernel {
            self.to_kernel_type()
        } else {
            self.to_func_type()
        }
    }

    pub fn to_kernel_type(&self) -> Type {
        match self {
            FnArgumentType::Reg(x) => x.clone().into(),
            FnArgumentType::Param(x) => x.clone().into(),
            FnArgumentType::Shared => {
                Type::Pointer(PointerType::Scalar(ScalarType::B8), LdStateSpace::Shared)
            }
        }
    }

    pub fn to_func_type(&self) -> Type {
        match self {
            FnArgumentType::Reg(x) => x.clone().into(),
            FnArgumentType::Param(VariableParamType::Scalar(t)) => {
                Type::Pointer(PointerType::Scalar((*t).into()), LdStateSpace::Param)
            }
            FnArgumentType::Param(VariableParamType::Array(t, dims)) => Type::Pointer(
                PointerType::Array((*t).into(), dims.clone()),
                LdStateSpace::Param,
            ),
            FnArgumentType::Param(VariableParamType::Pointer(t, space)) => Type::Pointer(
                PointerType::Pointer((*t).into(), (*space).into()),
                LdStateSpace::Param,
            ),
            FnArgumentType::Shared => {
                Type::Pointer(PointerType::Scalar(ScalarType::B8), LdStateSpace::Shared)
            }
        }
    }

    pub fn is_param(&self) -> bool {
        match self {
            FnArgumentType::Param(_) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum Type {
    Scalar(ScalarType),
    Vector(ScalarType, u8),
    Array(ScalarType, Vec<u32>),
    Pointer(PointerType, LdStateSpace),
}

#[derive(PartialEq, Eq, Clone)]
pub enum PointerType {
    Scalar(ScalarType),
    Vector(ScalarType, u8),
    Array(ScalarType, VecU32),
    // Instances of this variant are generated during stateful conversion
    Pointer(ScalarType, LdStateSpace),
}

impl From<ScalarType> for PointerType {
    fn from(t: ScalarType) -> Self {
        PointerType::Scalar(t.into())
    }
}

impl TryFrom<PointerType> for ScalarType {
    type Error = ();

    fn try_from(value: PointerType) -> Result<Self, Self::Error> {
        match value {
            PointerType::Scalar(t) => Ok(t),
            PointerType::Vector(_, _) => Err(()),
            PointerType::Array(_, _) => Err(()),
            PointerType::Pointer(_, _) => Err(()),
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
    F16x2,
    Pred,
}

impl ScalarType {
    pub fn size_of(self) -> u8 {
        match self {
            ScalarType::U8 => 1,
            ScalarType::S8 => 1,
            ScalarType::B8 => 1,
            ScalarType::U16 => 2,
            ScalarType::S16 => 2,
            ScalarType::B16 => 2,
            ScalarType::F16 => 2,
            ScalarType::U32 => 4,
            ScalarType::S32 => 4,
            ScalarType::B32 => 4,
            ScalarType::F32 => 4,
            ScalarType::U64 => 8,
            ScalarType::S64 => 8,
            ScalarType::B64 => 8,
            ScalarType::F64 => 8,
            ScalarType::F16x2 => 4,
            ScalarType::Pred => 1,
        }
    }
}

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

#[derive(Clone)]
pub struct Variable<T, ID> {
    pub align: Option<u32>,
    pub v_type: T,
    pub name: ID,
    pub array_init: Vec<u8>,
}

#[derive(Eq, PartialEq, Clone)]
pub enum VariableType {
    Reg(VariableRegType),
    Local(VariableLocalType),
    Param(VariableParamType),
    Global(VariableGlobalType),
    Shared(VariableGlobalType),
}

impl VariableType {
    pub fn to_type(&self) -> (StateSpace, Type) {
        match self {
            VariableType::Reg(t) => (StateSpace::Reg, t.clone().into()),
            VariableType::Local(t) => (StateSpace::Local, t.clone().into()),
            VariableType::Param(t) => (StateSpace::Param, t.clone().into()),
            VariableType::Global(t) => (StateSpace::Global, t.clone().into()),
            VariableType::Shared(t) => (StateSpace::Shared, t.clone().into()),
        }
    }
}

impl From<VariableType> for Type {
    fn from(t: VariableType) -> Self {
        match t {
            VariableType::Reg(t) => t.into(),
            VariableType::Local(t) => t.into(),
            VariableType::Param(t) => t.into(),
            VariableType::Global(t) => t.into(),
            VariableType::Shared(t) => t.into(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum StateSpace {
    Reg,
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
    Add(ArithDetails, Arg3<P>),
    Setp(SetpData, Arg4Setp<P>),
    SetpBool(SetpBoolData, Arg5Setp<P>),
    Not(ScalarType, Arg2<P>),
    Bra(BraData, Arg1<P>),
    Cvt(CvtDetails, Arg2<P>),
    Cvta(CvtaDetails, Arg2<P>),
    Shl(ScalarType, Arg3<P>),
    Shr(ScalarType, Arg3<P>),
    St(StData, Arg2St<P>),
    Ret(RetData),
    Call(CallInst<P>),
    Abs(AbsDetails, Arg2<P>),
    Mad(MulDetails, Arg4<P>),
    Or(ScalarType, Arg3<P>),
    Sub(ArithDetails, Arg3<P>),
    Min(MinMaxDetails, Arg3<P>),
    Max(MinMaxDetails, Arg3<P>),
    Rcp(RcpDetails, Arg2<P>),
    And(ScalarType, Arg3<P>),
    Selp(ScalarType, Arg4<P>),
    Bar(BarDetails, Arg1Bar<P>),
    Atom(AtomDetails, Arg3<P>),
    AtomCas(AtomCasDetails, Arg4<P>),
    Div(DivDetails, Arg3<P>),
    Sqrt(SqrtDetails, Arg2<P>),
    Rsqrt(RsqrtDetails, Arg2<P>),
    Neg(NegDetails, Arg2<P>),
    Sin { flush_to_zero: bool, arg: Arg2<P> },
    Cos { flush_to_zero: bool, arg: Arg2<P> },
    Lg2 { flush_to_zero: bool, arg: Arg2<P> },
    Ex2 { flush_to_zero: bool, arg: Arg2<P> },
    Clz { typ: ScalarType, arg: Arg2<P> },
    Brev { typ: ScalarType, arg: Arg2<P> },
    Popc { typ: ScalarType, arg: Arg2<P> },
    Xor { typ: ScalarType, arg: Arg3<P> },
    Bfe { typ: ScalarType, arg: Arg4<P> },
    Bfi { typ: ScalarType, arg: Arg5<P> },
    Rem { typ: ScalarType, arg: Arg3<P> },
}

#[derive(Copy, Clone)]
pub struct MadFloatDesc {}

#[derive(Copy, Clone)]
pub struct AbsDetails {
    pub flush_to_zero: Option<bool>,
    pub typ: ScalarType,
}
#[derive(Copy, Clone)]
pub struct RcpDetails {
    pub rounding: Option<RoundingMode>,
    pub flush_to_zero: Option<bool>,
    pub is_f64: bool,
}

pub struct CallInst<P: ArgParams> {
    pub uniform: bool,
    pub ret_params: Vec<P::Id>,
    pub func: P::Id,
    pub param_list: Vec<P::Operand>,
}

pub trait ArgParams {
    type Id;
    type Operand;
}

pub struct ParsedArgParams<'a> {
    _marker: PhantomData<&'a ()>,
}

impl<'a> ArgParams for ParsedArgParams<'a> {
    type Id = &'a str;
    type Operand = Operand<&'a str>;
}

pub struct Arg1<P: ArgParams> {
    pub src: P::Id, // it is a jump destination, but in terms of operands it is a source operand
}

pub struct Arg1Bar<P: ArgParams> {
    pub src: P::Operand,
}

pub struct Arg2<P: ArgParams> {
    pub dst: P::Operand,
    pub src: P::Operand,
}
pub struct Arg2Ld<P: ArgParams> {
    pub dst: P::Operand,
    pub src: P::Operand,
}

pub struct Arg2St<P: ArgParams> {
    pub src1: P::Operand,
    pub src2: P::Operand,
}

pub struct Arg2Mov<P: ArgParams> {
    pub dst: P::Operand,
    pub src: P::Operand,
}

pub struct Arg3<P: ArgParams> {
    pub dst: P::Operand,
    pub src1: P::Operand,
    pub src2: P::Operand,
}

pub struct Arg4<P: ArgParams> {
    pub dst: P::Operand,
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
    pub dst: P::Operand,
    pub src1: P::Operand,
    pub src2: P::Operand,
    pub src3: P::Operand,
    pub src4: P::Operand,
}

pub struct Arg5Setp<P: ArgParams> {
    pub dst1: P::Id,
    pub dst2: Option<P::Id>,
    pub src1: P::Operand,
    pub src2: P::Operand,
    pub src3: P::Operand,
}

#[derive(Copy, Clone)]
pub enum ImmediateValue {
    U64(u64),
    S64(i64),
    F32(f32),
    F64(f64),
}

#[derive(Clone)]
pub enum Operand<Id> {
    Reg(Id),
    RegOffset(Id, i32),
    Imm(ImmediateValue),
    VecMember(Id, u8),
    VecPack(Vec<Id>),
}

pub enum VectorPrefix {
    V2,
    V4,
}

pub struct LdDetails {
    pub qualifier: LdStQualifier,
    pub state_space: LdStateSpace,
    pub caching: LdCacheOperator,
    pub typ: PointerType,
    pub non_coherent: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LdStQualifier {
    Weak,
    Volatile,
    Relaxed(MemScope),
    Acquire(MemScope),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MemScope {
    Cta,
    Gpu,
    Sys,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
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

#[derive(Copy, Clone)]
pub struct MulIntDesc {
    pub typ: ScalarType,
    pub control: MulIntControl,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MulIntControl {
    Low,
    High,
    Wide,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum RoundingMode {
    NearestEven,
    Zero,
    NegativeInf,
    PositiveInf,
}

pub struct AddIntDesc {
    pub typ: ScalarType,
    pub saturate: bool,
}

pub struct SetpData {
    pub typ: ScalarType,
    pub flush_to_zero: Option<bool>,
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
    IsAnyNan,
}

pub enum SetpBoolPostOp {
    And,
    Or,
    Xor,
}

pub struct SetpBoolData {
    pub typ: ScalarType,
    pub flush_to_zero: Option<bool>,
    pub cmp_op: SetpCompareOp,
    pub bool_op: SetpBoolPostOp,
}

pub struct BraData {
    pub uniform: bool,
}

pub enum CvtDetails {
    IntFromInt(CvtIntToIntDesc),
    FloatFromFloat(CvtDesc),
    IntFromFloat(CvtDesc),
    FloatFromInt(CvtDesc),
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

impl CvtDetails {
    pub fn new_int_from_int_checked<'err, 'input>(
        saturate: bool,
        dst: ScalarType,
        src: ScalarType,
        err: &'err mut Vec<ParseError<usize, Token<'input>, PtxError>>,
    ) -> Self {
        if saturate {
            if src.kind() == ScalarKind::Signed {
                if dst.kind() == ScalarKind::Signed && dst.size_of() >= src.size_of() {
                    err.push(ParseError::from(PtxError::SyntaxError));
                }
            } else {
                if dst == src || dst.size_of() >= src.size_of() {
                    err.push(ParseError::from(PtxError::SyntaxError));
                }
            }
        }
        CvtDetails::IntFromInt(CvtIntToIntDesc { dst, src, saturate })
    }

    pub fn new_float_from_int_checked<'err, 'input>(
        rounding: RoundingMode,
        flush_to_zero: bool,
        saturate: bool,
        dst: ScalarType,
        src: ScalarType,
        err: &'err mut Vec<ParseError<usize, Token<'input>, PtxError>>,
    ) -> Self {
        if flush_to_zero && dst != ScalarType::F32 {
            err.push(ParseError::from(PtxError::NonF32Ftz));
        }
        CvtDetails::FloatFromInt(CvtDesc {
            dst,
            src,
            saturate,
            flush_to_zero: Some(flush_to_zero),
            rounding: Some(rounding),
        })
    }

    pub fn new_int_from_float_checked<'err, 'input>(
        rounding: RoundingMode,
        flush_to_zero: bool,
        saturate: bool,
        dst: ScalarType,
        src: ScalarType,
        err: &'err mut Vec<ParseError<usize, Token<'input>, PtxError>>,
    ) -> Self {
        if flush_to_zero && src != ScalarType::F32 {
            err.push(ParseError::from(PtxError::NonF32Ftz));
        }
        CvtDetails::IntFromFloat(CvtDesc {
            dst,
            src,
            saturate,
            flush_to_zero: Some(flush_to_zero),
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

pub struct StData {
    pub qualifier: LdStQualifier,
    pub state_space: StStateSpace,
    pub caching: StCacheOperator,
    pub typ: PointerType,
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

#[derive(Copy, Clone)]
pub enum MulDetails {
    Unsigned(MulUInt),
    Signed(MulSInt),
    Float(ArithFloat),
}

#[derive(Copy, Clone)]
pub struct MulUInt {
    pub typ: ScalarType,
    pub control: MulIntControl,
}

#[derive(Copy, Clone)]
pub struct MulSInt {
    pub typ: ScalarType,
    pub control: MulIntControl,
}

#[derive(Copy, Clone)]
pub enum ArithDetails {
    Unsigned(ScalarType),
    Signed(ArithSInt),
    Float(ArithFloat),
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

#[derive(Copy, Clone)]
pub enum MinMaxDetails {
    Signed(ScalarType),
    Unsigned(ScalarType),
    Float(MinMaxFloat),
}

#[derive(Copy, Clone)]
pub struct MinMaxFloat {
    pub flush_to_zero: Option<bool>,
    pub nan: bool,
    pub typ: ScalarType,
}

#[derive(Copy, Clone)]
pub struct AtomDetails {
    pub semantics: AtomSemantics,
    pub scope: MemScope,
    pub space: AtomSpace,
    pub inner: AtomInnerDetails,
}

#[derive(Copy, Clone)]
pub enum AtomSemantics {
    Relaxed,
    Acquire,
    Release,
    AcquireRelease,
}

#[derive(Copy, Clone)]
pub enum AtomSpace {
    Generic,
    Global,
    Shared,
}

#[derive(Copy, Clone)]
pub enum AtomInnerDetails {
    Bit { op: AtomBitOp, typ: ScalarType },
    Unsigned { op: AtomUIntOp, typ: ScalarType },
    Signed { op: AtomSIntOp, typ: ScalarType },
    Float { op: AtomFloatOp, typ: ScalarType },
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AtomBitOp {
    And,
    Or,
    Xor,
    Exchange,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AtomUIntOp {
    Add,
    Inc,
    Dec,
    Min,
    Max,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AtomSIntOp {
    Add,
    Min,
    Max,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AtomFloatOp {
    Add,
}

#[derive(Copy, Clone)]
pub struct AtomCasDetails {
    pub semantics: AtomSemantics,
    pub scope: MemScope,
    pub space: AtomSpace,
    pub typ: ScalarType,
}

#[derive(Copy, Clone)]
pub enum DivDetails {
    Unsigned(ScalarType),
    Signed(ScalarType),
    Float(DivFloatDetails),
}

#[derive(Copy, Clone)]
pub struct DivFloatDetails {
    pub typ: ScalarType,
    pub flush_to_zero: Option<bool>,
    pub kind: DivFloatKind,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DivFloatKind {
    Approx,
    Full,
    Rounding(RoundingMode),
}

pub enum NumsOrArrays<'a> {
    Nums(Vec<(&'a str, u32)>),
    Arrays(Vec<NumsOrArrays<'a>>),
}

#[derive(Copy, Clone)]
pub struct SqrtDetails {
    pub typ: ScalarType,
    pub flush_to_zero: Option<bool>,
    pub kind: SqrtKind,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SqrtKind {
    Approx,
    Rounding(RoundingMode),
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RsqrtDetails {
    pub typ: ScalarType,
    pub flush_to_zero: bool,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NegDetails {
    pub typ: ScalarType,
    pub flush_to_zero: Option<bool>,
}

impl<'a> NumsOrArrays<'a> {
    pub fn to_vec(self, typ: ScalarType, dimensions: &mut [u32]) -> Result<Vec<u8>, PtxError> {
        self.normalize_dimensions(dimensions)?;
        let sizeof_t = ScalarType::from(typ).size_of() as usize;
        let result_size = dimensions.iter().fold(sizeof_t, |x, y| x * (*y as usize));
        let mut result = vec![0; result_size];
        self.parse_and_copy(typ, sizeof_t, dimensions, &mut result)?;
        Ok(result)
    }

    fn normalize_dimensions(&self, dimensions: &mut [u32]) -> Result<(), PtxError> {
        match dimensions.first_mut() {
            Some(first) => {
                if *first == 0 {
                    *first = match self {
                        NumsOrArrays::Nums(v) => v.len() as u32,
                        NumsOrArrays::Arrays(v) => v.len() as u32,
                    };
                }
            }
            None => return Err(PtxError::ZeroDimensionArray),
        }
        for dim in dimensions {
            if *dim == 0 {
                return Err(PtxError::ZeroDimensionArray);
            }
        }
        Ok(())
    }

    fn parse_and_copy(
        &self,
        t: ScalarType,
        size_of_t: usize,
        dimensions: &[u32],
        result: &mut [u8],
    ) -> Result<(), PtxError> {
        match dimensions {
            [] => unreachable!(),
            [dim] => match self {
                NumsOrArrays::Nums(vec) => {
                    if vec.len() > *dim as usize {
                        return Err(PtxError::ZeroDimensionArray);
                    }
                    for (idx, (val, radix)) in vec.iter().enumerate() {
                        Self::parse_and_copy_single(t, idx, val, *radix, result)?;
                    }
                }
                NumsOrArrays::Arrays(_) => return Err(PtxError::ZeroDimensionArray),
            },
            [first_dim, rest @ ..] => match self {
                NumsOrArrays::Arrays(vec) => {
                    if vec.len() > *first_dim as usize {
                        return Err(PtxError::ZeroDimensionArray);
                    }
                    let size_of_element = rest.iter().fold(size_of_t, |x, y| x * (*y as usize));
                    for (idx, this) in vec.iter().enumerate() {
                        this.parse_and_copy(
                            t,
                            size_of_t,
                            rest,
                            &mut result[(size_of_element * idx)..],
                        )?;
                    }
                }
                NumsOrArrays::Nums(_) => return Err(PtxError::ZeroDimensionArray),
            },
        }
        Ok(())
    }

    fn parse_and_copy_single(
        t: ScalarType,
        idx: usize,
        str_val: &str,
        radix: u32,
        output: &mut [u8],
    ) -> Result<(), PtxError> {
        match t {
            ScalarType::B8 | ScalarType::U8 => {
                Self::parse_and_copy_single_t::<u8>(idx, str_val, radix, output)?;
            }
            ScalarType::B16 | ScalarType::U16 => {
                Self::parse_and_copy_single_t::<u16>(idx, str_val, radix, output)?;
            }
            ScalarType::B32 | ScalarType::U32 => {
                Self::parse_and_copy_single_t::<u32>(idx, str_val, radix, output)?;
            }
            ScalarType::B64 | ScalarType::U64 => {
                Self::parse_and_copy_single_t::<u64>(idx, str_val, radix, output)?;
            }
            ScalarType::S8 => {
                Self::parse_and_copy_single_t::<i8>(idx, str_val, radix, output)?;
            }
            ScalarType::S16 => {
                Self::parse_and_copy_single_t::<i16>(idx, str_val, radix, output)?;
            }
            ScalarType::S32 => {
                Self::parse_and_copy_single_t::<i32>(idx, str_val, radix, output)?;
            }
            ScalarType::S64 => {
                Self::parse_and_copy_single_t::<i64>(idx, str_val, radix, output)?;
            }
            ScalarType::F16 => {
                Self::parse_and_copy_single_t::<f16>(idx, str_val, radix, output)?;
            }
            ScalarType::F16x2 => todo!(),
            ScalarType::F32 => {
                Self::parse_and_copy_single_t::<f32>(idx, str_val, radix, output)?;
            }
            ScalarType::F64 => {
                Self::parse_and_copy_single_t::<f64>(idx, str_val, radix, output)?;
            }
            ScalarType::Pred => todo!(),
        }
        Ok(())
    }

    fn parse_and_copy_single_t<T: Copy + FromStr>(
        idx: usize,
        str_val: &str,
        _radix: u32, // TODO: use this to properly support hex literals
        output: &mut [u8],
    ) -> Result<(), PtxError>
    where
        T::Err: Into<PtxError>,
    {
        let typed_output = unsafe {
            std::slice::from_raw_parts_mut::<T>(
                output.as_mut_ptr() as *mut _,
                output.len() / mem::size_of::<T>(),
            )
        };
        typed_output[idx] = str_val.parse::<T>().map_err(|e| e.into())?;
        Ok(())
    }
}

pub enum ArrayOrPointer {
    Array { dimensions: Vec<u32>, init: Vec<u8> },
    Pointer,
}

bitflags! {
    pub struct LinkingDirective: u8 {
        const NONE = 0b000;
        const EXTERN = 0b001;
        const VISIBLE = 0b10;
        const WEAK = 0b100;
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TuningDirective {
    MaxNReg(u32),
    MaxNtid(u32, u32, u32),
    ReqNtid(u32, u32, u32),
    MinNCtaPerSm(u32),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScalarKind {
    Bit,
    Unsigned,
    Signed,
    Float,
    Float2,
    Pred,
}

impl ScalarType {
    pub fn kind(self) -> ScalarKind {
        match self {
            ScalarType::U8 => ScalarKind::Unsigned,
            ScalarType::U16 => ScalarKind::Unsigned,
            ScalarType::U32 => ScalarKind::Unsigned,
            ScalarType::U64 => ScalarKind::Unsigned,
            ScalarType::S8 => ScalarKind::Signed,
            ScalarType::S16 => ScalarKind::Signed,
            ScalarType::S32 => ScalarKind::Signed,
            ScalarType::S64 => ScalarKind::Signed,
            ScalarType::B8 => ScalarKind::Bit,
            ScalarType::B16 => ScalarKind::Bit,
            ScalarType::B32 => ScalarKind::Bit,
            ScalarType::B64 => ScalarKind::Bit,
            ScalarType::F16 => ScalarKind::Float,
            ScalarType::F32 => ScalarKind::Float,
            ScalarType::F64 => ScalarKind::Float,
            ScalarType::F16x2 => ScalarKind::Float2,
            ScalarType::Pred => ScalarKind::Pred,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_fails_multiple_0_dmiensions() {
        let inp = NumsOrArrays::Nums(Vec::new());
        assert!(inp.to_vec(ScalarType::B8, &mut vec![0, 0]).is_err());
    }

    #[test]
    fn array_fails_on_empty() {
        let inp = NumsOrArrays::Nums(Vec::new());
        assert!(inp.to_vec(ScalarType::B8, &mut vec![0]).is_err());
    }

    #[test]
    fn array_auto_sizes_0_dimension() {
        let inp = NumsOrArrays::Arrays(vec![
            NumsOrArrays::Nums(vec![("1", 10), ("2", 10)]),
            NumsOrArrays::Nums(vec![("3", 10), ("4", 10)]),
        ]);
        let mut dimensions = vec![0u32, 2];
        assert_eq!(
            vec![1u8, 2, 3, 4],
            inp.to_vec(ScalarType::B8, &mut dimensions).unwrap()
        );
        assert_eq!(dimensions, vec![2u32, 2]);
    }

    #[test]
    fn array_fails_wrong_structure() {
        let inp = NumsOrArrays::Arrays(vec![
            NumsOrArrays::Nums(vec![("1", 10), ("2", 10)]),
            NumsOrArrays::Arrays(vec![NumsOrArrays::Nums(vec![("1", 10)])]),
        ]);
        let mut dimensions = vec![0u32, 2];
        assert!(inp.to_vec(ScalarType::B8, &mut dimensions).is_err());
    }

    #[test]
    fn array_fails_too_long_component() {
        let inp = NumsOrArrays::Arrays(vec![
            NumsOrArrays::Nums(vec![("1", 10), ("2", 10), ("3", 10)]),
            NumsOrArrays::Nums(vec![("4", 10), ("5", 10)]),
        ]);
        let mut dimensions = vec![0u32, 2];
        assert!(inp.to_vec(ScalarType::B8, &mut dimensions).is_err());
    }
}
