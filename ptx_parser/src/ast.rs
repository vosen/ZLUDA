use super::{
    AtomSemantics, MemScope, RawRoundingMode, RawSetpCompareOp, ScalarType, SetpBoolPostOp,
    StateSpace, VectorPrefix,
};
use crate::{
    FunnelShiftMode, Mul24Control, PtxError, PtxParserState, Reduction, ShiftDirection, ShuffleMode,
};
use bitflags::bitflags;
use std::{alloc::Layout, cmp::Ordering, fmt::Write, num::NonZeroU8};

pub enum Statement<P: Operand> {
    Label(P::Ident),
    Variable(MultiVariable<P::Ident>),
    Instruction(Option<PredAt<P::Ident>>, Instruction<P>),
    Block(Vec<Statement<P>>),
}

// We define the instruction enum through the macro instead of normally, because we have some of how
// we use this type in the compilee. Each instruction can be logically split into two parts:
// properties that define instruction semantics (e.g. is memory load volatile?) that don't change
// during compilation and arguments (e.g. memory load source and destination) that evolve during
// compilation. To support compilation passes we need to be able to visit (and change) every
// argument in a generic way. This macro has visibility over all the fields. Consequently, we use it
// to generate visitor functions. There re three functions to support three different semantics:
// visit-by-ref, visit-by-mutable-ref, visit-and-map. In a previous version of the compiler it was
// done by hand and was very limiting (we supported only visit-and-map).
// The visitor must implement appropriate visitor trait defined below this macro. For convenience,
// we implemented visitors for some corresponding FnMut(...) types.
// Properties in this macro are used to encode information about the instruction arguments (what
// Rust type is used  for it post-parsing, what PTX type does it expect, what PTX address space does
// it expect, etc.).
// This information is then available to a visitor.
ptx_parser_macros::generate_instruction_type!(
    pub enum Instruction<T: Operand> {
        Abs {
            data: TypeFtz,
            type: { Type::Scalar(data.type_) },
            arguments<T>: {
                dst: T,
                src: T,
            }
        },
        Activemask {
            type: Type::Scalar(ScalarType::B32),
            arguments<T>: {
                dst: T
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
        And {
            data: ScalarType,
            type: { Type::Scalar(data.clone()) },
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
            }
        },
        Atom {
            type: &data.type_,
            data: AtomDetails,
            arguments<T>: {
                dst: T,
                src1: {
                    repr: T,
                    space: { data.space },
                },
                src2: T,
            }
        },
        AtomCas {
            type: Type::Scalar(data.type_),
            data: AtomCasDetails,
            arguments<T>: {
                dst: T,
                src1: {
                    repr: T,
                    space: { data.space },
                },
                src2: T,
                src3: T,
            }
        },
        BarWarp {
            type: Type::Scalar(ScalarType::U32),
            data: (),
            arguments<T>: {
                src: T,
            }
        },
        Bar {
            type: Type::Scalar(ScalarType::U32),
            data: BarData,
            arguments<T>: {
                src1: T,
                src2: Option<T>,
            }
        },
        BarRed {
            type: Type::Scalar(ScalarType::U32),
            data: BarRedData,
            arguments<T>: {
                dst1: {
                    repr: T,
                    type: Type::from(ScalarType::Pred)
                },
                src_barrier: T,
                src_threadcount: Option<T>,
                src_predicate: {
                    repr: T,
                    type: Type::from(ScalarType::Pred)
                },
                src_negate_predicate: {
                    repr: T,
                    type: Type::from(ScalarType::Pred)
                },
            }
        },
        Bfe {
            type: Type::Scalar(data.clone()),
            data: ScalarType,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U32)
                },
                src3: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U32)
                },
            }
        },
        Bfi {
            type: Type::Scalar(data.clone()),
            data: ScalarType,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
                src3: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U32)
                },
                src4: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U32)
                },
            }
        },
        Bra {
            type: !,
            arguments<T::Ident>: {
                src: T
            }
        },
        Brev {
            type: Type::Scalar(data.clone()),
            data: ScalarType,
            arguments<T>: {
                dst: T,
                src: T
            }
        },
        Call {
            data: CallDetails,
            arguments: CallArgs<T>,
            visit: arguments.visit(data, visitor)?,
            visit_mut: arguments.visit_mut(data, visitor)?,
            map: Instruction::Call{ arguments: arguments.map(&data, visitor)?, data }
        },
        Clz {
            type: Type::Scalar(data.clone()),
            data: ScalarType,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U32)
                },
                src: T
            }
        },
        Cos {
            type: Type::Scalar(ScalarType::F32),
            data: FlushToZero,
            arguments<T>: {
                dst: T,
                src: T
            }
        },
        CpAsync {
            type: Type::Scalar(ScalarType::U32),
            data: CpAsyncDetails,
            arguments<T>: {
                src_to: {
                    repr: T,
                    space: StateSpace::Shared
                },
                src_from: {
                    repr: T,
                    space: StateSpace::Global
                }
            }
        },
        CpAsyncCommitGroup { },
        CpAsyncWaitGroup {
            type: Type::Scalar(ScalarType::U64),
            arguments<T>: {
                src_group: T
            }
        },
        CpAsyncWaitAll { },
        Cvt {
            data: CvtDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { Type::Scalar(data.to) },
                    // TODO: double check
                    relaxed_type_check: true,
                },
                src: {
                    repr: T,
                    type: { Type::Scalar(data.from) },
                    relaxed_type_check: true,
                },
            }
        },
        Cvta {
            data: CvtaDetails,
            type: { Type::Scalar(ScalarType::B64) },
            arguments<T>: {
                dst: T,
                src: T,
            }
        },
        Div {
            type: Type::Scalar(data.type_()),
            data: DivDetails,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
            }
        },
        Dp4a {
            data: Dp4aDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::B32) },
                },
                src1: {
                    repr: T,
                    type: { Type::Scalar(data.atype) },
                },
                src2: {
                    repr: T,
                    type: { Type::Scalar(data.btype) },
                },
                src3: {
                    repr: T,
                    type: { Type::Scalar(data.ctype()) },
                },
            }
        },
        Ex2 {
            type: Type::Scalar(ScalarType::F32),
            data: TypeFtz,
            arguments<T>: {
                dst: T,
                src: T
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
        Ld {
            type: { &data.typ },
            data: LdDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    relaxed_type_check: true,
                },
                src: {
                    repr: T,
                    space: { data.state_space },
                }
            },
            display: write!(f, "<TODO:finish ld>")?
        },
        Lg2 {
            type: Type::Scalar(ScalarType::F32),
            data: FlushToZero,
            arguments<T>: {
                dst: T,
                src: T
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
                src3: {
                    repr: T,
                    type: { Type::from(data.dst_type()) },
                }
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
        Membar {
            data: MemScope
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
        Mov {
            type: { &data.typ },
            data: MovDetails,
            arguments<T>: {
                dst: T,
                src: T
            },
            display: write!(f, "mov{}", data.typ)?
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
        Mul24 {
            type: { Type::from(data.type_) },
            data: Mul24Details,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
            }
        },
        Nanosleep {
            type: Type::Scalar(ScalarType::U32),
            arguments<T>: {
                src: T
            }
         },
        Neg {
            type: Type::Scalar(data.type_),
            data: TypeFtz,
            arguments<T>: {
                dst: T,
                src: T
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
        Popc {
            type: Type::Scalar(data.clone()),
            data: ScalarType,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U32)
                },
                src: T
            }
        },
        Prmt {
            type: Type::Scalar(ScalarType::B32),
            data: u16,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T
            }
        },
        PrmtSlow {
            type: Type::Scalar(ScalarType::U32),
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
                src3: T
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
        Rem {
            type: Type::Scalar(data.clone()),
            data: ScalarType,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T
            }
        },
        Ret {
            data: RetData,
            display: write!(f, "ret")?
        },
        Rsqrt {
            type: { Type::from(data.type_) },
            data: TypeFtz,
            arguments<T>: {
                dst: T,
                src: T,
            }
        },
        Selp {
            type: { Type::Scalar(data.clone()) },
            data: ScalarType,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
                src3: {
                    repr: T,
                    type: Type::Scalar(ScalarType::Pred)
                },
            }
        },
        Set {
            data: SetData,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: Type::from(data.dtype)
                },
                src1: {
                    repr: T,
                    type: Type::from(data.base.type_),
                },
                src2: {
                    repr: T,
                    type: Type::from(data.base.type_),
                }
            }
        },
        SetBool {
            data: SetBoolData,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: Type::from(data.dtype)
                },
                src1: {
                    repr: T,
                    type: Type::from(data.base.base.type_),
                },
                src2: {
                    repr: T,
                    type: Type::from(data.base.base.type_),
                },
                src3: {
                    repr: T,
                    type: Type::from(ScalarType::Pred)
                }
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
        ShflSync {
            data: ShflSyncDetails,
            type: Type::Scalar(ScalarType::B32),
            arguments<T>: {
                dst: T,
                dst_pred: {
                    repr: Option<T>,
                    type: Type::from(ScalarType::Pred)
                },
                src: T,
                src_lane: T,
                src_opts: T,
                src_membermask: T
            }
        },
        Shf {
            data: ShfDetails,
            type: Type::Scalar(ScalarType::B32),
            arguments<T>: {
                dst: T,
                src_a: T,
                src_b: T,
                src_c: T
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
        Sin {
            type: Type::Scalar(ScalarType::F32),
            data: FlushToZero,
            arguments<T>: {
                dst: T,
                src: T
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
        St {
            type: { &data.typ },
            data: StData,
            arguments<T>: {
                src1: {
                    repr: T,
                    space: { data.state_space },
                },
                src2: {
                    repr: T,
                    relaxed_type_check: true,
                }
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
        Trap { },
        Xor {
            type: Type::Scalar(data.clone()),
            data: ScalarType,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T
            }
        },
        Tanh {
            type: Type::Scalar(data.clone()),
            data: ScalarType,
            arguments<T>: {
                dst: T,
                src: T
            }
        },
    }
);

pub trait Visitor<T: Operand, Err> {
    fn visit(
        &mut self,
        args: &T,
        type_space: Option<(&Type, StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<(), Err>;
    fn visit_ident(
        &mut self,
        args: &T::Ident,
        type_space: Option<(&Type, StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<(), Err>;
}

impl<
        T: Operand,
        Err,
        Fn: FnMut(&T, Option<(&Type, StateSpace)>, bool, bool) -> Result<(), Err>,
    > Visitor<T, Err> for Fn
{
    fn visit(
        &mut self,
        args: &T,
        type_space: Option<(&Type, StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<(), Err> {
        (self)(args, type_space, is_dst, relaxed_type_check)
    }

    fn visit_ident(
        &mut self,
        args: &T::Ident,
        type_space: Option<(&Type, StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<(), Err> {
        (self)(
            &T::from_ident(*args),
            type_space,
            is_dst,
            relaxed_type_check,
        )
    }
}

pub trait VisitorMut<T: Operand, Err> {
    fn visit(
        &mut self,
        args: &mut T,
        type_space: Option<(&Type, StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<(), Err>;
    fn visit_ident(
        &mut self,
        args: &mut T::Ident,
        type_space: Option<(&Type, StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<(), Err>;
}

pub trait VisitorMap<From: Operand, To: Operand, Err> {
    fn visit(
        &mut self,
        args: From,
        type_space: Option<(&Type, StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<To, Err>;
    fn visit_ident(
        &mut self,
        args: From::Ident,
        type_space: Option<(&Type, StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<To::Ident, Err>;
}

impl<T: Copy, U: Copy, Err, Fn> VisitorMap<ParsedOperand<T>, ParsedOperand<U>, Err> for Fn
where
    Fn: FnMut(T, Option<(&Type, StateSpace)>, bool, bool) -> Result<U, Err>,
{
    fn visit(
        &mut self,
        args: ParsedOperand<T>,
        type_space: Option<(&Type, StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<ParsedOperand<U>, Err> {
        Ok(match args {
            ParsedOperand::Reg(ident) => {
                ParsedOperand::Reg((self)(ident, type_space, is_dst, relaxed_type_check)?)
            }
            ParsedOperand::RegOffset(ident, imm) => ParsedOperand::RegOffset(
                (self)(ident, type_space, is_dst, relaxed_type_check)?,
                imm,
            ),
            ParsedOperand::Imm(imm) => ParsedOperand::Imm(imm),
            ParsedOperand::VecMember(ident, index) => ParsedOperand::VecMember(
                (self)(ident, type_space, is_dst, relaxed_type_check)?,
                index,
            ),
            ParsedOperand::VecPack(vec) => ParsedOperand::VecPack(
                vec.into_iter()
                    .map(|ident| (self)(ident, type_space, is_dst, relaxed_type_check))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
        })
    }

    fn visit_ident(
        &mut self,
        args: T,
        type_space: Option<(&Type, StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<U, Err> {
        (self)(args, type_space, is_dst, relaxed_type_check)
    }
}

impl<T: Operand<Ident = T>, U: Operand<Ident = U>, Err, Fn> VisitorMap<T, U, Err> for Fn
where
    Fn: FnMut(T, Option<(&Type, StateSpace)>, bool, bool) -> Result<U, Err>,
{
    fn visit(
        &mut self,
        args: T,
        type_space: Option<(&Type, StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<U, Err> {
        (self)(args, type_space, is_dst, relaxed_type_check)
    }

    fn visit_ident(
        &mut self,
        args: T,
        type_space: Option<(&Type, StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<U, Err> {
        (self)(args, type_space, is_dst, relaxed_type_check)
    }
}

trait VisitOperand<Err> {
    type Operand: Operand;
    #[allow(unused)] // Used by generated code
    fn visit(&self, fn_: impl FnMut(&Self::Operand) -> Result<(), Err>) -> Result<(), Err>;
    #[allow(unused)] // Used by generated code
    fn visit_mut(
        &mut self,
        fn_: impl FnMut(&mut Self::Operand) -> Result<(), Err>,
    ) -> Result<(), Err>;
}

impl<T: Operand, Err> VisitOperand<Err> for T {
    type Operand = Self;
    fn visit(&self, mut fn_: impl FnMut(&Self::Operand) -> Result<(), Err>) -> Result<(), Err> {
        fn_(self)
    }
    fn visit_mut(
        &mut self,
        mut fn_: impl FnMut(&mut Self::Operand) -> Result<(), Err>,
    ) -> Result<(), Err> {
        fn_(self)
    }
}

impl<T: Operand, Err> VisitOperand<Err> for Option<T> {
    type Operand = T;
    fn visit(&self, mut fn_: impl FnMut(&Self::Operand) -> Result<(), Err>) -> Result<(), Err> {
        if let Some(x) = self {
            fn_(x)?;
        }
        Ok(())
    }
    fn visit_mut(
        &mut self,
        mut fn_: impl FnMut(&mut Self::Operand) -> Result<(), Err>,
    ) -> Result<(), Err> {
        if let Some(x) = self {
            fn_(x)?;
        }
        Ok(())
    }
}

impl<T: Operand, Err> VisitOperand<Err> for Vec<T> {
    type Operand = T;
    fn visit(&self, mut fn_: impl FnMut(&Self::Operand) -> Result<(), Err>) -> Result<(), Err> {
        for o in self {
            fn_(o)?;
        }
        Ok(())
    }
    fn visit_mut(
        &mut self,
        mut fn_: impl FnMut(&mut Self::Operand) -> Result<(), Err>,
    ) -> Result<(), Err> {
        for o in self {
            fn_(o)?;
        }
        Ok(())
    }
}

trait MapOperand<Err>: Sized {
    type Input;
    type Output<U>;
    #[allow(unused)] // Used by generated code
    fn map<U>(
        self,
        fn_: impl FnOnce(Self::Input) -> Result<U, Err>,
    ) -> Result<Self::Output<U>, Err>;
}

impl<T: Operand, Err> MapOperand<Err> for T {
    type Input = Self;
    type Output<U> = U;
    fn map<U>(self, fn_: impl FnOnce(T) -> Result<U, Err>) -> Result<U, Err> {
        fn_(self)
    }
}

impl<T: Operand, Err> MapOperand<Err> for Option<T> {
    type Input = T;
    type Output<U> = Option<U>;
    fn map<U>(self, fn_: impl FnOnce(T) -> Result<U, Err>) -> Result<Option<U>, Err> {
        self.map(|x| fn_(x)).transpose()
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

impl<ID: std::fmt::Display> std::fmt::Display for Variable<ID> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.state_space)?;

        if let Some(align) = self.align {
            write!(f, " .align {}", align)?;
        }

        let (vector_size, scalar_type, array_dims) = match &self.v_type {
            Type::Scalar(scalar_type) => (None, *scalar_type, &vec![]),
            Type::Vector(size, scalar_type) => (Some(*size), *scalar_type, &vec![]),
            Type::Array(vector_size, scalar_type, array_dims) => {
                (vector_size.map(|s| s.get()), *scalar_type, array_dims)
            }
        };

        if let Some(size) = vector_size {
            write!(f, " .v{}", size)?;
        }

        write!(f, " {} {}", scalar_type, self.name)?;

        for dim in array_dims {
            write!(f, "[{}]", dim)?;
        }

        if self.array_init.len() > 0 {
            todo!("Need to intepret the array initializer data as the appropriate type");
        }

        Ok(())
    }
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
    Vector(u8, ScalarType),
    // .param.b32 foo[4];
    Array(Option<NonZeroU8>, ScalarType, Vec<u32>),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Scalar(scalar_type) => write!(f, "{}", scalar_type),
            _ => todo!(),
        }
    }
}

impl Type {
    pub(crate) fn maybe_vector(vector: Option<VectorPrefix>, scalar: ScalarType) -> Self {
        match vector {
            Some(prefix) => Type::Vector(prefix.len().get(), scalar),
            None => Type::Scalar(scalar),
        }
    }

    pub(crate) fn maybe_vector_parsed(prefix: Option<NonZeroU8>, scalar: ScalarType) -> Self {
        match prefix {
            Some(prefix) => Type::Vector(prefix.get(), scalar),
            None => Type::Scalar(scalar),
        }
    }

    pub(crate) fn maybe_array(
        prefix: Option<NonZeroU8>,
        scalar: ScalarType,
        array: Option<Vec<u32>>,
    ) -> Self {
        match array {
            Some(dimensions) => Type::Array(prefix, scalar, dimensions),
            None => Self::maybe_vector_parsed(prefix, scalar),
        }
    }

    pub fn layout(&self) -> Layout {
        match self {
            Type::Scalar(type_) => type_.layout(),
            Type::Vector(elements, scalar_type) => {
                let scalar_layout = scalar_type.layout();
                unsafe {
                    Layout::from_size_align_unchecked(
                        scalar_layout.size() * *elements as usize,
                        scalar_layout.align() * *elements as usize,
                    )
                }
            }
            Type::Array(non_zero, scalar, vec) => {
                let element_layout = Type::maybe_vector_parsed(*non_zero, *scalar).layout();
                let len = vec.iter().copied().reduce(std::ops::Mul::mul).unwrap_or(0);
                unsafe {
                    Layout::from_size_align_unchecked(
                        element_layout.size() * (len as usize),
                        element_layout.align(),
                    )
                }
            }
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

    pub fn layout(self) -> Layout {
        match self {
            ScalarType::U8 | ScalarType::S8 | ScalarType::B8 => Layout::new::<u8>(),
            ScalarType::U16
            | ScalarType::S16
            | ScalarType::B16
            | ScalarType::F16
            | ScalarType::BF16 => Layout::new::<u16>(),
            ScalarType::U32
            | ScalarType::S32
            | ScalarType::B32
            | ScalarType::F32
            | ScalarType::U16x2
            | ScalarType::S16x2
            | ScalarType::F16x2
            | ScalarType::BF16x2 => Layout::new::<u32>(),
            ScalarType::U64 | ScalarType::S64 | ScalarType::B64 | ScalarType::F64 => {
                Layout::new::<u64>()
            }
            ScalarType::B128 => unsafe { Layout::from_size_align_unchecked(16, 16) },
            // Close enough
            ScalarType::Pred => Layout::new::<u8>(),
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
}

impl MovDetails {
    pub(crate) fn new(vector: Option<VectorPrefix>, scalar: ScalarType) -> Self {
        MovDetails {
            typ: Type::maybe_vector(vector, scalar),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ShflSyncDetails {
    pub mode: ShuffleMode,
}

pub enum CpAsyncCpSize {
    Bytes4,
    Bytes8,
    Bytes16,
}

impl CpAsyncCpSize {
    pub fn from_u64(n: u64) -> Option<Self> {
        match n {
            4 => Some(Self::Bytes4),
            8 => Some(Self::Bytes8),
            16 => Some(Self::Bytes16),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> u64 {
        match self {
            CpAsyncCpSize::Bytes4 => 4,
            CpAsyncCpSize::Bytes8 => 8,
            CpAsyncCpSize::Bytes16 => 16,
        }
    }
}

pub struct CpAsyncDetails {
    pub caching: CpAsyncCacheOperator,
    pub space: StateSpace,
    pub cp_size: CpAsyncCpSize,
    pub src_size: Option<u64>,
}

pub struct ShfDetails {
    pub direction: ShiftDirection,
    pub mode: FunnelShiftMode,
}

#[derive(Clone)]
pub enum ParsedOperand<Ident> {
    Reg(Ident),
    RegOffset(Ident, i32),
    Imm(ImmediateValue),
    VecMember(Ident, u8),
    VecPack(Vec<Ident>),
}

impl<Ident> ParsedOperand<Ident> {
    pub fn as_immediate(&self) -> Option<ImmediateValue> {
        match self {
            ParsedOperand::Imm(imm) => Some(*imm),
            _ => None,
        }
    }
}

impl<Ident> std::fmt::Display for ParsedOperand<Ident>
where
    Ident: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsedOperand::Reg(id) => write!(f, "{}", id)?,
            ParsedOperand::RegOffset(id, o) => write!(f, "{}+{}", id, o)?,
            ParsedOperand::Imm(imm) => write!(f, "{}", imm)?,
            ParsedOperand::VecMember(id, idx) => {
                let suffix = match idx {
                    0 => "x",
                    1 => "y",
                    2 => "z",
                    3 => "w",
                    _ => "INVALID",
                };
                write!(f, "{}.{}", id, suffix)?
            }
            ParsedOperand::VecPack(items) => {
                f.write_char('{')?;
                for (idx, item) in items.iter().enumerate() {
                    if idx != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                f.write_char('}')?
            }
        }
        Ok(())
    }
}

impl<Ident: Copy> Operand for ParsedOperand<Ident> {
    type Ident = Ident;

    fn from_ident(ident: Self::Ident) -> Self {
        ParsedOperand::Reg(ident)
    }
}

pub trait Operand: Sized {
    type Ident: Copy;

    fn from_ident(ident: Self::Ident) -> Self;
}

#[derive(Copy, Clone)]
pub enum ImmediateValue {
    U64(u64),
    S64(i64),
    F32(f32),
    F64(f64),
}

impl ImmediateValue {
    /// If the value is a U64 or S64, returns the value as a u64, ignoring the sign.
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            ImmediateValue::U64(n) => Some(n),
            ImmediateValue::S64(n) => Some(n as u64),
            ImmediateValue::F32(_) | ImmediateValue::F64(_) => None,
        }
    }
}

impl std::fmt::Display for ImmediateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImmediateValue::U64(n) => write!(f, "{}", n)?,
            ImmediateValue::S64(n) => write!(f, "{}", n)?,
            ImmediateValue::F32(n) => write!(f, "{}", n)?,
            ImmediateValue::F64(n) => write!(f, "{}", n)?,
        }
        Ok(())
    }
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

pub enum CpAsyncCacheOperator {
    Cached,
    L2Only,
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
    pub rounding: RoundingMode,
    pub flush_to_zero: Option<bool>,
    pub saturate: bool,
    // From PTX documentation: https://docs.nvidia.com/cuda/parallel-thread-execution/#mixed-precision-floating-point-instructions-add
    // Note that an add instruction with an explicit rounding modifier is treated conservatively by
    // the code optimizer. An add instruction with no rounding modifier defaults to
    // round-to-nearest-even and may be optimized aggressively by the code optimizer. In particular,
    // mul/add sequences with no rounding modifiers may be optimized to use fused-multiply-add
    // instructions on the target device.
    pub is_fusable: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LdStQualifier {
    Weak,
    Volatile,
    Relaxed(MemScope),
    Acquire(MemScope),
    Release(MemScope),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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

impl<'input> MethodDeclaration<'input, &'input str> {
    pub fn name(&self) -> &'input str {
        match self.name {
            MethodName::Kernel(n) => n,
            MethodName::Func(n) => n,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub enum MethodName<'input, ID> {
    Kernel(&'input str),
    Func(ID),
}

impl<'input, ID> MethodName<'input, ID> {
    pub fn is_kernel(&self) -> bool {
        match self {
            MethodName::Kernel(_) => true,
            MethodName::Func(_) => false,
        }
    }
}

impl<'input> MethodName<'input, &'input str> {
    pub fn text(&self) -> &'input str {
        match self {
            MethodName::Kernel(name) => *name,
            MethodName::Func(name) => *name,
        }
    }
}

bitflags! {
    pub struct LinkingDirective: u8 {
        const NONE = 0b000;
        const EXTERN = 0b001;
        const VISIBLE = 0b10;
        const WEAK = 0b100;
    }
}

impl std::fmt::Display for LinkingDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut directives = vec![];
        if self.contains(LinkingDirective::EXTERN) {
            directives.push(".extern");
        } else if self.contains(LinkingDirective::VISIBLE) {
            directives.push(".visible");
        } else if self.contains(LinkingDirective::WEAK) {
            directives.push(".weak");
        }
        write!(f, "{}", directives.join(" "))
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

#[derive(Copy, Clone)]
pub struct Mul24Details {
    pub type_: ScalarType,
    pub control: Mul24Control,
}

pub struct SetData {
    pub dtype: ScalarType,
    pub base: SetpData,
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
            (true, _) => {
                state.errors.push(PtxError::NonF32Ftz);
                None
            }
            _ => None,
        };
        let type_kind = type_.kind();
        let cmp_op = if type_kind == ScalarKind::Float {
            SetpCompareOp::Float(SetpCompareFloat::from(cmp_op))
        } else {
            match SetpCompareInt::try_from((cmp_op, type_kind)) {
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

pub struct SetBoolData {
    pub dtype: ScalarType,
    pub base: SetpBoolData,
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
    UnsignedLess,
    UnsignedLessOrEq,
    UnsignedGreater,
    UnsignedGreaterOrEq,
    SignedLess,
    SignedLessOrEq,
    SignedGreater,
    SignedGreaterOrEq,
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

impl TryFrom<(RawSetpCompareOp, ScalarKind)> for SetpCompareInt {
    type Error = PtxError<'static>;

    fn try_from((value, kind): (RawSetpCompareOp, ScalarKind)) -> Result<Self, PtxError<'static>> {
        match (value, kind) {
            (RawSetpCompareOp::Eq, _) => Ok(SetpCompareInt::Eq),
            (RawSetpCompareOp::Ne, _) => Ok(SetpCompareInt::NotEq),
            (RawSetpCompareOp::Lt | RawSetpCompareOp::Lo, ScalarKind::Signed) => {
                Ok(SetpCompareInt::SignedLess)
            }
            (RawSetpCompareOp::Lt | RawSetpCompareOp::Lo, _) => Ok(SetpCompareInt::UnsignedLess),
            (RawSetpCompareOp::Le | RawSetpCompareOp::Ls, ScalarKind::Signed) => {
                Ok(SetpCompareInt::SignedLessOrEq)
            }
            (RawSetpCompareOp::Le | RawSetpCompareOp::Ls, _) => {
                Ok(SetpCompareInt::UnsignedLessOrEq)
            }
            (RawSetpCompareOp::Gt | RawSetpCompareOp::Hi, ScalarKind::Signed) => {
                Ok(SetpCompareInt::SignedGreater)
            }
            (RawSetpCompareOp::Gt | RawSetpCompareOp::Hi, _) => Ok(SetpCompareInt::UnsignedGreater),
            (RawSetpCompareOp::Ge | RawSetpCompareOp::Hs, ScalarKind::Signed) => {
                Ok(SetpCompareInt::SignedGreaterOrEq)
            }
            (RawSetpCompareOp::Ge | RawSetpCompareOp::Hs, _) => {
                Ok(SetpCompareInt::UnsignedGreaterOrEq)
            }
            (RawSetpCompareOp::Equ, _) => Err(PtxError::WrongType),
            (RawSetpCompareOp::Neu, _) => Err(PtxError::WrongType),
            (RawSetpCompareOp::Ltu, _) => Err(PtxError::WrongType),
            (RawSetpCompareOp::Leu, _) => Err(PtxError::WrongType),
            (RawSetpCompareOp::Gtu, _) => Err(PtxError::WrongType),
            (RawSetpCompareOp::Geu, _) => Err(PtxError::WrongType),
            (RawSetpCompareOp::Num, _) => Err(PtxError::WrongType),
            (RawSetpCompareOp::Nan, _) => Err(PtxError::WrongType),
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
    fn visit<Err>(
        &self,
        details: &CallDetails,
        visitor: &mut impl Visitor<T, Err>,
    ) -> Result<(), Err> {
        for (param, (type_, space)) in self
            .return_arguments
            .iter()
            .zip(details.return_arguments.iter())
        {
            visitor.visit_ident(
                param,
                Some((type_, *space)),
                *space == StateSpace::Reg,
                false,
            )?;
        }
        visitor.visit_ident(&self.func, None, false, false)?;
        for (param, (type_, space)) in self
            .input_arguments
            .iter()
            .zip(details.input_arguments.iter())
        {
            visitor.visit(param, Some((type_, *space)), false, false)?;
        }
        Ok(())
    }

    #[allow(dead_code)] // Used by generated code
    fn visit_mut<Err>(
        &mut self,
        details: &CallDetails,
        visitor: &mut impl VisitorMut<T, Err>,
    ) -> Result<(), Err> {
        for (param, (type_, space)) in self
            .return_arguments
            .iter_mut()
            .zip(details.return_arguments.iter())
        {
            visitor.visit_ident(
                param,
                Some((type_, *space)),
                *space == StateSpace::Reg,
                false,
            )?;
        }
        visitor.visit_ident(&mut self.func, None, false, false)?;
        for (param, (type_, space)) in self
            .input_arguments
            .iter_mut()
            .zip(details.input_arguments.iter())
        {
            visitor.visit(param, Some((type_, *space)), false, false)?;
        }
        Ok(())
    }

    #[allow(dead_code)] // Used by generated code
    fn map<U: Operand, Err>(
        self,
        details: &CallDetails,
        visitor: &mut impl VisitorMap<T, U, Err>,
    ) -> Result<CallArgs<U>, Err> {
        let return_arguments = self
            .return_arguments
            .into_iter()
            .zip(details.return_arguments.iter())
            .map(|(param, (type_, space))| {
                visitor.visit_ident(
                    param,
                    Some((type_, *space)),
                    *space == StateSpace::Reg,
                    false,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let func = visitor.visit_ident(self.func, None, false, false)?;
        let input_arguments = self
            .input_arguments
            .into_iter()
            .zip(details.input_arguments.iter())
            .map(|(param, (type_, space))| {
                visitor.visit(param, Some((type_, *space)), false, false)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(CallArgs {
            return_arguments,
            func,
            input_arguments,
        })
    }
}

pub struct CvtDetails {
    pub from: ScalarType,
    pub to: ScalarType,
    pub mode: CvtMode,
}

#[derive(Clone, Copy)]
pub enum CvtMode {
    // int from int
    ZeroExtend,
    SignExtend,
    Truncate,
    Bitcast,
    IntSaturateToSigned,
    IntSaturateToUnsigned,
    // float from float
    FPExtend {
        flush_to_zero: Option<bool>,
        saturate: bool,
    },
    FPTruncate {
        // float rounding
        rounding: RoundingMode,
        is_integer_rounding: bool,
        flush_to_zero: Option<bool>,
        saturate: bool,
    },
    FPRound {
        integer_rounding: Option<RoundingMode>,
        flush_to_zero: Option<bool>,
        saturate: bool,
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
    FPFromSigned {
        rounding: RoundingMode,
        saturate: bool,
    }, // float rounding
    FPFromUnsigned {
        rounding: RoundingMode,
        saturate: bool,
    }, // float rounding
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
        let rounding = rnd.map(RawRoundingMode::normalize);
        let mut unwrap_rounding = || match rounding {
            Some((rnd, is_integer)) => (rnd, is_integer),
            None => {
                if let Some(rnd) = rnd {
                    errors.push(PtxError::SyntaxError(format!(
                        "invalid rounding mode {} for cvt",
                        rnd
                    )));
                } else {
                    errors.push(PtxError::SyntaxError(format!(
                        "missing rounding mode for cvt"
                    )));
                }
                (RoundingMode::NearestEven, false)
            }
        };
        let mode = match (dst.kind(), src.kind()) {
            (ScalarKind::Float, ScalarKind::Float) => match dst.size_of().cmp(&src.size_of()) {
                Ordering::Less => {
                    let (rounding, is_integer_rounding) = unwrap_rounding();
                    CvtMode::FPTruncate {
                        rounding,
                        is_integer_rounding,
                        flush_to_zero,
                        saturate,
                    }
                }
                Ordering::Equal => CvtMode::FPRound {
                    integer_rounding: rounding.map(|(rnd, _)| rnd),
                    flush_to_zero,
                    saturate,
                },
                Ordering::Greater => {
                    if rounding.is_some() {
                        errors.push(PtxError::SyntaxError(
                            "should not have rounding mode when dst is larger than src in cvt"
                                .to_string(),
                        ));
                    }
                    CvtMode::FPExtend {
                        flush_to_zero,
                        saturate,
                    }
                }
            },
            (ScalarKind::Unsigned, ScalarKind::Float) => CvtMode::UnsignedFromFP {
                rounding: unwrap_rounding().0,
                flush_to_zero,
            },
            (ScalarKind::Signed, ScalarKind::Float) => CvtMode::SignedFromFP {
                rounding: unwrap_rounding().0,
                flush_to_zero,
            },
            (ScalarKind::Float, ScalarKind::Unsigned) => CvtMode::FPFromUnsigned {
                rounding: unwrap_rounding().0,
                saturate,
            },
            (ScalarKind::Float, ScalarKind::Signed) => CvtMode::FPFromSigned {
                rounding: unwrap_rounding().0,
                saturate,
            },
            (ScalarKind::Signed, ScalarKind::Unsigned)
            | (ScalarKind::Signed, ScalarKind::Signed)
                if saturate =>
            {
                CvtMode::IntSaturateToSigned
            }
            (ScalarKind::Unsigned, ScalarKind::Signed)
            | (ScalarKind::Unsigned, ScalarKind::Unsigned)
                if saturate =>
            {
                CvtMode::IntSaturateToUnsigned
            }
            (ScalarKind::Unsigned, ScalarKind::Unsigned)
            | (ScalarKind::Signed, ScalarKind::Signed)
            | (ScalarKind::Unsigned, ScalarKind::Signed)
            | (ScalarKind::Signed, ScalarKind::Unsigned)
                if dst.size_of() == src.size_of() =>
            {
                CvtMode::Bitcast
            }
            (ScalarKind::Unsigned, ScalarKind::Unsigned)
            | (ScalarKind::Signed, ScalarKind::Signed)
            | (ScalarKind::Unsigned, ScalarKind::Signed)
            | (ScalarKind::Signed, ScalarKind::Unsigned) => match dst.size_of().cmp(&src.size_of())
            {
                Ordering::Less => CvtMode::Truncate,
                Ordering::Equal => CvtMode::Bitcast,
                Ordering::Greater => {
                    if src.kind() == ScalarKind::Signed {
                        CvtMode::SignExtend
                    } else {
                        CvtMode::ZeroExtend
                    }
                }
            },
            (_, _) => {
                errors.push(PtxError::SyntaxError(
                    "unexpected pairing of dst and src types in cvt".to_string(),
                ));
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TypeFtz {
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

#[derive(Copy, Clone)]
pub struct RcpData {
    pub kind: RcpKind,
    pub flush_to_zero: Option<bool>,
    pub type_: ScalarType,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RcpKind {
    Approx,
    Compliant(RoundingMode),
}

pub struct BarData {
    pub aligned: bool,
}

#[derive(Copy, Clone)]
pub struct BarRedData {
    pub aligned: bool,
    pub pred_reduction: Reduction,
}

pub struct AtomDetails {
    pub type_: Type,
    pub semantics: AtomSemantics,
    pub scope: MemScope,
    pub space: StateSpace,
    pub op: AtomicOp,
}

#[derive(Copy, Clone)]
pub enum AtomicOp {
    And,
    Or,
    Xor,
    Exchange,
    Add,
    IncrementWrap,
    DecrementWrap,
    SignedMin,
    UnsignedMin,
    SignedMax,
    UnsignedMax,
    FloatAdd,
    FloatMin,
    FloatMax,
}

impl AtomicOp {
    pub(crate) fn new(op: super::RawAtomicOp, kind: ScalarKind) -> Self {
        use super::RawAtomicOp;
        match (op, kind) {
            (RawAtomicOp::And, _) => Self::And,
            (RawAtomicOp::Or, _) => Self::Or,
            (RawAtomicOp::Xor, _) => Self::Xor,
            (RawAtomicOp::Exch, _) => Self::Exchange,
            (RawAtomicOp::Add, ScalarKind::Float) => Self::FloatAdd,
            (RawAtomicOp::Add, _) => Self::Add,
            (RawAtomicOp::Inc, _) => Self::IncrementWrap,
            (RawAtomicOp::Dec, _) => Self::DecrementWrap,
            (RawAtomicOp::Min, ScalarKind::Signed) => Self::SignedMin,
            (RawAtomicOp::Min, ScalarKind::Float) => Self::FloatMin,
            (RawAtomicOp::Min, _) => Self::UnsignedMin,
            (RawAtomicOp::Max, ScalarKind::Signed) => Self::SignedMax,
            (RawAtomicOp::Max, ScalarKind::Float) => Self::FloatMax,
            (RawAtomicOp::Max, _) => Self::UnsignedMax,
        }
    }
}

pub struct AtomCasDetails {
    pub type_: ScalarType,
    pub semantics: AtomSemantics,
    pub scope: MemScope,
    pub space: StateSpace,
}

#[derive(Copy, Clone)]
pub enum DivDetails {
    Unsigned(ScalarType),
    Signed(ScalarType),
    Float(DivFloatDetails),
}

#[derive(Copy, Clone)]
pub struct Dp4aDetails {
    pub atype: ScalarType,
    pub btype: ScalarType,
}

impl Dp4aDetails {
    pub fn ctype(self) -> ScalarType {
        match (self.atype, self.btype) {
            (ScalarType::U32, ScalarType::U32) => ScalarType::U32,
            _ => ScalarType::S32,
        }
    }
}

impl DivDetails {
    pub fn type_(&self) -> ScalarType {
        match self {
            DivDetails::Unsigned(t) => *t,
            DivDetails::Signed(t) => *t,
            DivDetails::Float(float) => float.type_,
        }
    }
}

#[derive(Copy, Clone)]
pub struct DivFloatDetails {
    pub type_: ScalarType,
    pub flush_to_zero: Option<bool>,
    pub kind: DivFloatKind,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DivFloatKind {
    Approx,
    ApproxFull,
    Rounding(RoundingMode),
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlushToZero {
    pub flush_to_zero: bool,
}
