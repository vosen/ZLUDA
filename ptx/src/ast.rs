use half::f16;
use lalrpop_util::{lexer::Token, ParseError};
use std::alloc::Layout;
use std::{convert::From, mem, num::ParseFloatError, str::FromStr};
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
    ArrayInitializer,
    #[error("")]
    ScalarInitalizer,
    #[error("")]
    NonScalarArray,
    #[error("")]
    InvalidStateSpace,
    #[error("")]
    BlankVariableName,
    #[error("")]
    NonRegPredVariable,
    #[error("")]
    InitializerTypeMismatch,
    #[error("")]
    NonExternPointer,
    #[error("{start}:{end}")]
    UnrecognizedStatement { start: usize, end: usize },
    #[error("{start}:{end}")]
    UnrecognizedDirective { start: usize, end: usize },
    #[error("")]
    NoSmVersion,
    #[error("")]
    UnexpectedMultivariable,
    #[error("")]
    ExternDefinition,
}

// For some weird reson this is illegal:
//   .param .f16x2 foobar;
// but this is legal:
//   .param .f16x2 foobar[1];
// even more interestingly this is legal, but only in .func (not in .entry):
//   .param .b32 foobar[]

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BarDetails {
    SyncAligned,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum ReductionOp {
    And,
    Or,
    Popc,
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
    pub sm_version: u32,
    pub directives: Vec<Directive<'a, ParsedArgParams<'a>>>,
}

pub enum Directive<'a, P: ArgParams> {
    Variable(LinkingDirective, MultiVariableDefinition<P::Id>),
    Method(LinkingDirective, Function<'a, &'a str, Statement<P>>),
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub enum MethodName<'input, ID> {
    Kernel(&'input str),
    Func(ID),
}

impl<'input, ID> MethodName<'input, ID> {
    pub fn is_kernel(&self) -> bool {
        match self {
            MethodName::Kernel(..) => true,
            MethodName::Func(..) => false,
        }
    }
}

#[derive(Clone)]
pub struct MethodDeclaration<'input, ID> {
    pub return_arguments: Vec<VariableDeclaration<ID>>,
    pub name: MethodName<'input, ID>,
    pub input_arguments: Vec<VariableDeclaration<ID>>,
}

pub struct Function<'a, ID, S> {
    pub func_directive: MethodDeclaration<'a, ID>,
    pub tuning: Vec<TuningDirective>,
    pub body: Option<Vec<S>>,
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Type {
    // .param.b32 foo;
    // -> OpTypeInt
    Scalar(ScalarType),
    // .param.v2.b32 foo;
    // -> OpTypeVector
    Vector(ScalarType, u8),
    // .param.b32 foo[4];
    // -> OpTypeArray
    Array(ScalarType, Vec<u32>),
    /*
        Variables of this type almost never exist in the original .ptx and are
        usually artificially created. Some examples below:
        - extern pointers to the .shared memory in the form:
            .extern .shared .b32 shared_mem[];
          which we first parse as
            .extern .shared .b32 shared_mem;
          and then convert to an additional function parameter:
            .param .ptr<.b32.shared> shared_mem;
          and do a load at the start of the function (and renames inside fn):
            .reg .ptr<.b32.shared> temp;
            ld.param.ptr<.b32.shared> temp, [shared_mem];
          note, we don't support non-.shared extern pointers, because there's
          zero use for them in the ptxas
        - artifical pointers created by stateful conversion, which work
          similiarly to the above
        - function parameters:
            foobar(.param .align 4 .b8 numbers[])
          which get parsed to
            foobar(.param .align 4 .b8 numbers)
          and then converted to
            foobar(.reg .align 4 .ptr<.b8.param> numbers)
        - ld/st with offset:
            .reg.b32 x;
            .param.b64 arg0;
            st.param.b32 [arg0+4], x;
          Yes, this code is legal and actually emitted by the NV compiler!
          We convert the st to:
            .reg ptr<.b64.param> temp = ptr_offset(arg0, 4);
            st.param.b32 [temp], x;
    */
    // .reg ptr<.b64.param>
    // -> OpTypePointer Function
    Pointer(ScalarType, StateSpace),
    Texref,
    Surfref,
    // Structs exist only to support certain internal, compiler-generated patterns
    Struct(Vec<StructField>),
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
    pub(crate) fn to_ptx_name(self) -> &'static str {
        match self {
            ScalarType::B8 => "b8",
            ScalarType::B16 => "b16",
            ScalarType::B32 => "b32",
            ScalarType::B64 => "b64",
            ScalarType::U8 => "u8",
            ScalarType::U16 => "u16",
            ScalarType::U32 => "u32",
            ScalarType::U64 => "u64",
            ScalarType::S8 => "s8",
            ScalarType::S16 => "s16",
            ScalarType::S32 => "s32",
            ScalarType::S64 => "s64",
            ScalarType::F16 => "f16",
            ScalarType::F32 => "f32",
            ScalarType::F64 => "f64",
            ScalarType::F16x2 => "f16x2",
            ScalarType::Pred => "pred",
        }
    }
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum StructField {
    Scalar(ScalarType),
    Vector(ScalarType, u8),
}

impl StructField {
    pub fn to_type(self) -> Type {
        match self {
            Self::Scalar(type_) => Type::Scalar(type_),
            Self::Vector(type_, size) => Type::Vector(type_, size),
        }
    }
}

pub enum Statement<P: ArgParams> {
    Label(P::Id),
    Callprototype(Callprototype<P::Id>),
    Variable(Vec<MultiVariableDefinition<P::Id>>),
    Instruction(Option<PredAt<P::Id>>, Instruction<P>),
    Block(Vec<Statement<P>>),
}

#[derive(Clone)]
pub struct Callprototype<ID> {
    pub name: ID,
    pub return_arguments: Vec<(Type, StateSpace)>,
    pub input_arguments: Vec<(Type, StateSpace)>,
}

#[derive(Clone)]
pub struct VariableDeclaration<ID> {
    pub align: Option<u32>,
    pub type_: Type,
    pub state_space: StateSpace,
    pub name: ID,
}

impl<ID> VariableDeclaration<ID> {
    pub fn layout(&self) -> Layout {
        let layout = self.type_.layout();
        match self.align.map(|a| layout.align_to(a as usize)) {
            Some(Ok(aligned_layout)) => aligned_layout,
            _ => layout,
        }
    }
}

#[derive(Clone)]
pub struct MultiVariableDefinition<ID> {
    pub variable: VariableDeclaration<ID>,
    pub suffix: Option<DeclarationSuffix<ID>>,
}

#[derive(Clone)]
pub enum DeclarationSuffix<ID> {
    Count(u32),
    Initializer(Initializer<ID>),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum StateSpace {
    Reg,
    Const,
    Global,
    Local,
    Shared,
    Param,
    Generic,
    Sreg,
}

pub struct PredAt<ID> {
    pub not: bool,
    pub label: ID,
}

pub struct BfindDetails {
    pub shift: bool,
    pub type_: ScalarType,
}

pub enum Instruction<P: ArgParams> {
    Ld(LdDetails, Arg2Ld<P>),
    Mov(MovDetails, Arg2Mov<P>),
    Mul(MulDetails, Arg3<P>),
    Add(ArithDetails, Arg3<P>),
    AddC(CarryInDetails, Arg3<P>),
    AddCC(ScalarType, Arg3<P>),
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
    MadC {
        type_: ScalarType,
        carry_out: bool,
        is_hi: bool,
        arg: Arg4<P>,
    },
    MadCC {
        type_: ScalarType,
        arg: Arg4<P>,
    },
    Fma(ArithFloat, Arg4<P>),
    Or(ScalarType, Arg3<P>),
    Sub(ArithDetails, Arg3<P>),
    SubC(CarryInDetails, Arg3<P>),
    SubCC(ScalarType, Arg3<P>),
    Min(MinMaxDetails, Arg3<P>),
    Max(MinMaxDetails, Arg3<P>),
    Rcp(RcpSqrtDetails, Arg2<P>),
    Sqrt(RcpSqrtDetails, Arg2<P>),
    And(ScalarType, Arg3<P>),
    Selp(ScalarType, Arg4<P>),
    Bar(BarDetails, Arg1Bar<P>),
    BarWarp(BarDetails, Arg1Bar<P>),
    BarRed(ReductionOp, Arg3<P>),
    Atom(AtomDetails, Arg3<P>),
    AtomCas(AtomCasDetails, Arg4<P>),
    Div(DivDetails, Arg3<P>),
    Rsqrt(RsqrtDetails, Arg2<P>),
    Neg(NegDetails, Arg2<P>),
    Sin {
        flush_to_zero: bool,
        arg: Arg2<P>,
    },
    Cos {
        flush_to_zero: bool,
        arg: Arg2<P>,
    },
    Lg2 {
        flush_to_zero: bool,
        arg: Arg2<P>,
    },
    Ex2 {
        flush_to_zero: bool,
        arg: Arg2<P>,
    },
    Clz {
        typ: ScalarType,
        arg: Arg2<P>,
    },
    Brev {
        typ: ScalarType,
        arg: Arg2<P>,
    },
    Popc {
        typ: ScalarType,
        arg: Arg2<P>,
    },
    Xor {
        typ: ScalarType,
        arg: Arg3<P>,
    },
    Bfe {
        typ: ScalarType,
        arg: Arg4<P>,
    },
    Bfi {
        typ: ScalarType,
        arg: Arg5<P>,
    },
    Rem {
        typ: ScalarType,
        arg: Arg3<P>,
    },
    Prmt {
        control: u16,
        arg: Arg3<P>,
    },
    PrmtSlow {
        control: P::Id,
        arg: Arg3<P>,
    },
    Activemask {
        arg: Arg1<P>,
    },
    Membar {
        level: MemScope,
    },
    Tex(TexDetails, Arg5Tex<P>),
    Suld(SurfaceDetails, Arg5Tex<P>),
    Sust(SurfaceDetails, Arg4Sust<P>),
    Shfl(ShflMode, Arg5Shfl<P>),
    Shf(FunnelShift, Arg4<P>),
    Vote(VoteDetails, Arg3<P>),
    Exit,
    Trap,
    Brkpt,
    Vshr(Arg4<P>),
    Bfind(BfindDetails, Arg2<P>),
    Set(SetData, Arg3<P>),
    Dp4a(ScalarType, Arg4<P>),
    MatchAny(Arg3<P>),
    Red(AtomDetails, Arg2St<P>),
    Nanosleep(Arg1<P>),
    Isspacep(StateSpace, Arg2<P>),
}

#[derive(Copy, Clone)]

pub struct CarryInDetails {
    pub type_: ScalarType,
    pub carry_out: bool,
}

#[derive(Copy, Clone)]
pub enum ShflMode {
    Up,
    Down,
    Bfly,
    Idx,
}

#[derive(Copy, Clone)]
pub struct VoteDetails {
    pub mode: VoteMode,
    pub negate_pred: bool,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum VoteMode {
    Ballot,
    All,
    Any,
    Uni,
}

#[derive(Copy, Clone)]
pub struct MadFloatDesc {}

#[derive(Copy, Clone)]
pub struct AbsDetails {
    pub flush_to_zero: Option<bool>,
    pub typ: ScalarType,
}

#[derive(Copy, Clone)]
pub struct RcpSqrtDetails {
    pub kind: RcpSqrtKind,
    pub flush_to_zero: Option<bool>,
    pub type_: ScalarType,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RcpSqrtKind {
    Approx,
    Rounding(RoundingMode),
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FunnelShift {
    pub direction: FunnelDirection,
    pub mode: ShiftNormalization,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FunnelDirection {
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ShiftNormalization {
    Wrap,
    Clamp,
}

pub struct CallInst<P: ArgParams> {
    pub uniform: bool,
    pub ret_params: Vec<P::Id>,
    pub func: P::Id,
    pub param_list: Vec<P::Operand>,
    pub prototype: Option<P::Id>,
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

pub struct Arg4Sust<P: ArgParams> {
    pub image: P::Operand,
    pub coordinates: P::Operand,
    pub layer: Option<P::Operand>,
    pub value: P::Operand,
}

pub struct Arg5<P: ArgParams> {
    pub dst: P::Operand,
    pub src1: P::Operand,
    pub src2: P::Operand,
    pub src3: P::Operand,
    pub src4: P::Operand,
}

pub struct Arg5Tex<P: ArgParams> {
    pub dst: P::Operand,
    pub image: P::Operand,
    pub layer: Option<P::Operand>,
    pub coordinates: P::Operand,
    pub lod: Option<P::Operand>,
}

pub struct Arg5Setp<P: ArgParams> {
    pub dst1: P::Id,
    pub dst2: Option<P::Id>,
    pub src1: P::Operand,
    pub src2: P::Operand,
    pub src3: P::Operand,
}

pub struct Arg5Shfl<P: ArgParams> {
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

impl ImmediateValue {
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            ImmediateValue::U64(x) => x.to_ne_bytes().to_vec(),
            ImmediateValue::S64(x) => x.to_ne_bytes().to_vec(),
            ImmediateValue::F32(x) => x.to_ne_bytes().to_vec(),
            ImmediateValue::F64(x) => x.to_ne_bytes().to_vec(),
        }
    }

    pub fn as_u8(self) -> Option<u8> {
        match self {
            ImmediateValue::U64(x) => Some(x as u8),
            ImmediateValue::S64(x) => Some(x as u8),
            ImmediateValue::F32(_) | ImmediateValue::F64(_) => None,
        }
    }

    pub fn as_i8(self) -> Option<i8> {
        match self {
            ImmediateValue::U64(x) => Some(x as i8),
            ImmediateValue::S64(x) => Some(x as i8),
            ImmediateValue::F32(_) | ImmediateValue::F64(_) => None,
        }
    }

    pub fn as_u16(self) -> Option<u16> {
        match self {
            ImmediateValue::U64(x) => Some(x as u16),
            ImmediateValue::S64(x) => Some(x as u16),
            ImmediateValue::F32(_) | ImmediateValue::F64(_) => None,
        }
    }

    pub fn as_i16(self) -> Option<i16> {
        match self {
            ImmediateValue::U64(x) => Some(x as i16),
            ImmediateValue::S64(x) => Some(x as i16),
            ImmediateValue::F32(_) | ImmediateValue::F64(_) => None,
        }
    }

    pub fn as_u32(self) -> Option<u32> {
        match self {
            ImmediateValue::U64(x) => Some(x as u32),
            ImmediateValue::S64(x) => Some(x as u32),
            ImmediateValue::F32(_) | ImmediateValue::F64(_) => None,
        }
    }

    pub fn as_i32(self) -> Option<i32> {
        match self {
            ImmediateValue::U64(x) => Some(x as i32),
            ImmediateValue::S64(x) => Some(x as i32),
            ImmediateValue::F32(_) | ImmediateValue::F64(_) => None,
        }
    }

    pub fn as_u64(self) -> Option<u64> {
        match self {
            ImmediateValue::U64(x) => Some(x),
            ImmediateValue::S64(x) => Some(x as u64),
            ImmediateValue::F32(_) | ImmediateValue::F64(_) => None,
        }
    }

    pub fn as_i64(self) -> Option<i64> {
        match self {
            ImmediateValue::U64(x) => Some(x as i64),
            ImmediateValue::S64(x) => Some(x),
            ImmediateValue::F32(_) | ImmediateValue::F64(_) => None,
        }
    }

    pub fn as_f32(self) -> Option<f32> {
        match self {
            ImmediateValue::F32(x) => Some(x),
            ImmediateValue::F64(_) | ImmediateValue::U64(_) | ImmediateValue::S64(_) => None,
        }
    }

    pub fn as_f64(self) -> Option<f64> {
        match self {
            ImmediateValue::F32(x) => Some(x as f64),
            ImmediateValue::F64(x) => Some(x),
            ImmediateValue::U64(_) | ImmediateValue::S64(_) => None,
        }
    }
}

#[derive(Clone)]
pub enum Operand<Id> {
    Reg(Id),
    RegOffset(Id, i64),
    Imm(ImmediateValue),
    VecMember(Id, u8),
    VecPack(Vec<RegOrImmediate<Id>>),
}

#[derive(Clone)]
pub enum RegOrImmediate<Id> {
    Reg(Id),
    Imm(ImmediateValue),
}

pub struct LdDetails {
    pub qualifier: LdStQualifier,
    pub state_space: StateSpace,
    pub caching: LdCacheOperator,
    pub typ: Type,
    pub non_coherent: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LdStQualifier {
    Weak,
    Volatile,
    Relaxed(MemScope),
    Acquire(MemScope),
    Release(MemScope),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MemScope {
    Cta,
    Gpu,
    Sys,
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

pub struct SetData {
    pub dst_type: ScalarType,
    pub src_type: ScalarType,
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
    IsAnyNan,
}

pub struct SetpBoolData {
    pub base: SetpData,
    pub bool_op: SetpBoolPostOp,
    pub negate_src3: bool,
}

#[derive(Clone, Copy)]
pub enum SetpBoolPostOp {
    And,
    Or,
    Xor,
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

#[derive(Clone)]
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
    pub to: StateSpace,
    pub from: StateSpace,
    pub size: CvtaSize,
}

#[derive(Clone, Copy)]
pub enum CvtaSize {
    U32,
    U64,
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

#[derive(Copy, Clone)]
pub enum MulDetails {
    Unsigned(MulInt),
    Signed(MulInt),
    Float(ArithFloat),
}

#[derive(Copy, Clone)]
pub struct MulInt {
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
    pub space: StateSpace,
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
    pub space: StateSpace,
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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LinkingDirective {
    None,
    Extern,
    Visible,
    Weak,
    Common,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TuningDirective {
    MaxNReg(u32),
    MaxNtid(u32, u32, u32),
    ReqNtid(u32, u32, u32),
    MinNCtaPerSm(u32),
    Noreturn,
}

#[repr(u8)]
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

pub struct TexDetails {
    pub geometry: TextureGeometry,
    pub channel_type: ScalarType,
    pub coordinate_type: ScalarType,
    // direct = takes .texref, indirect = takes .u64
    pub direct: bool,
}

pub struct SurfaceDetails {
    pub geometry: TextureGeometry,
    pub vector: Option<u8>,
    pub type_: ScalarType,
    // direct = takes .texref, indirect = takes .u64
    pub direct: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextureGeometry {
    OneD,
    TwoD,
    ThreeD,
    Array1D,
    Array2D,
}

#[derive(Clone)]
pub enum Initializer<ID> {
    Constant(ImmediateValue),
    Global(ID, InitializerType),
    GenericGlobal(ID, InitializerType),
    Add(Box<(Initializer<ID>, Initializer<ID>)>),
    Array(Vec<Initializer<ID>>),
}

#[derive(Clone)]
pub enum InitializerType {
    Unknown,
    Value(Type),
    Function(Vec<Type>, Vec<Type>),
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
