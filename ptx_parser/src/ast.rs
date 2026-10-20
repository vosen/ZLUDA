use super::{
    AtomSemantics, MemScope, RawRoundingMode, RawSetpCompareOp, ScalarType, SetpBoolPostOp,
    StateSpace, VectorPrefix,
};
use crate::{
    BmskMode, PrmtMode, CacheLevel, EvictionPriority, FunnelShiftMode, Mul24Control, PtxError,
    PtxParserState, Reduction, ShiftDirection, ShuffleMode, Tf32Rounding, VoteMode, VshOp,
};
use bitflags::bitflags;
use derive_more::Display;
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
            },
            display: write!(f, "add{}", data)?
        },
        AddExtended {
            data: CarryDetails,
            type: { Type::Scalar(data.type_) },
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
            }
        },
        SubExtended {
            data: CarryDetails,
            type: { Type::Scalar(data.type_) },
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
            }
        },
        MadExtended {
            data: MadCarryDetails,
            type: { Type::Scalar(data.type_) },
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
                src3: T,
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
        Red {
            type: &data.type_,
            data: AtomDetails,
            arguments<T>: {
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
                    type: {
                        if data.pred_reduction == Reduction::Popc {
                            Type::from(ScalarType::U32)
                        } else {
                            Type::from(ScalarType::Pred)
                        }
                    }
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
        Lop3 {
            type: Type::Scalar(ScalarType::B32),
            data: Option<SetpBoolPostOp>,
            arguments<T>: {
                dst: T,
                dst_pred: {
                    repr: Option<T>,
                    type: Type::from(ScalarType::Pred)
                },
                src1: T,
                src2: T,
                src3: T,
                src_lut: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U32)
                },
                src_pred: {
                    repr: Option<T>,
                    type: Type::from(ScalarType::Pred)
                },
            }
        },
        Mad24 {
            type: { Type::from(data.type_) },
            data: Mad24Details,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
                src3: T,
            }
        },
        MixedFloat {
            type: { Type::Scalar(data.dtype) },
            data: MixedFloatDetails,
            arguments<T>: {
                dst: T,
                src1: {
                    repr: T,
                    type: Type::Scalar(data.atype)
                },
                src2: {
                    repr: T,
                    type: Type::Scalar(data.btype)
                },
                src3: Option<T>,
            }
        },
        Slct {
            type: { Type::Scalar(data.type_) },
            data: SlctDetails,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
                src3: {
                    repr: T,
                    type: Type::Scalar(data.ctype)
                },
            }
        },
        Testp {
            type: Type::from(ScalarType::Pred),
            data: TestpDetails,
            arguments<T>: {
                dst: T,
                src: {
                    repr: T,
                    type: Type::Scalar(data.type_)
                },
            }
        },
        Szext {
            type: { Type::Scalar(data.type_) },
            data: SzextDetails,
            arguments<T>: {
                dst: T,
                src_a: T,
                src_b: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U32)
                },
            }
        },
        Fns {
            type: Type::Scalar(ScalarType::B32),
            arguments<T>: {
                dst: T,
                src_mask: T,
                src_base: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U32)
                },
                src_offset: {
                    repr: T,
                    type: Type::Scalar(ScalarType::S32)
                },
            }
        },
        Bmsk {
            type: Type::Scalar(ScalarType::B32),
            data: BmskMode,
            arguments<T>: {
                dst: T,
                src_a: T,
                src_b: T,
            }
        },
        Bra {
            type: !,
            arguments<T::Ident>: {
                src: T
            },
            display: write!(f, "bra")?
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
                },
                // src-size or ignore-src in a register (an immediate src-size is in data)
                src_size: {
                    repr: Option<T>,
                    type: { Type::Scalar(if data.ignore_src { ScalarType::Pred } else { ScalarType::U32 }) },
                }
            }
        },
        CpAsyncMbarrierArrive {
            type: Type::Scalar(ScalarType::B64),
            data: CpAsyncMbarrierArriveDetails,
            arguments<T>: {
                src_mbarrier: {
                    repr: T,
                    space: { data.space },
                }
            }
        },
        CpAsyncBulk {
            type: Type::Scalar(ScalarType::B8),
            data: CpAsyncBulkDetails,
            arguments<T>: {
                src_to: {
                    repr: T,
                    space: { data.dst_space },
                },
                src_from: {
                    repr: T,
                    space: { data.src_space },
                },
                src_size: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U32),
                },
                src_mbarrier: {
                    repr: Option<T>,
                    type: Type::Scalar(ScalarType::B64),
                    space: { data.dst_space },
                },
                src_cta_mask: {
                    repr: Option<T>,
                    type: Type::Scalar(ScalarType::B16),
                },
                src_byte_mask: {
                    repr: Option<T>,
                    type: Type::Scalar(ScalarType::B16),
                },
                src_cache_policy: {
                    repr: Option<T>,
                    type: Type::Scalar(ScalarType::B64),
                }
            }
        },
        CpReduceAsyncBulk {
            type: { Type::Scalar(data.type_) },
            data: CpReduceAsyncBulkDetails,
            arguments<T>: {
                src_to: {
                    repr: T,
                    space: { data.dst_space },
                },
                src_from: {
                    repr: T,
                    space: { data.src_space },
                },
                src_size: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U32),
                },
                src_mbarrier: {
                    repr: Option<T>,
                    type: Type::Scalar(ScalarType::B64),
                    space: { data.dst_space },
                },
                src_cache_policy: {
                    repr: Option<T>,
                    type: Type::Scalar(ScalarType::B64),
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
        CreatePolicyFractional {
            type: Type::Scalar(ScalarType::U64),
            data: CreatePolicyFractionalDetails,
            arguments<T>: {
                dst_policy: T
            }
        },
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
                src2: {
                    repr: Option<T>,
                    type: { Type::Scalar(data.from) },
                    relaxed_type_check: true,
                },
            }
        },
        Alloca {
            type: Type::Scalar(data.type_),
            data: AllocaDetails,
            arguments<T>: {
                dst: T,
                src: T,
            }
        },
        StackSave {
            type: Type::Scalar(data.clone()),
            data: ScalarType,
            arguments<T>: {
                dst: T,
            }
        },
        StackRestore {
            type: Type::Scalar(data.clone()),
            data: ScalarType,
            arguments<T>: {
                src: T,
            }
        },
        // clmad: carry-less multiply of a and b, the low (data false) or
        // high (true) 64 bits xor c
        // Scalar video instructions: vadd, vsub, vabsdiff, vmin, vmax, vshl,
        // vshr and vset. src3 is the operand of the secondary operation or
        // of the merge
        VideoScalar {
            type: Type::Scalar(ScalarType::B32),
            data: VideoScalarDetails,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
                src3: Option<T>,
            }
        },
        // SIMD video instructions: vadd2, vsub2, vavrg2, vabsdiff2, vmin2,
        // vmax2, vset2 and their 4-way versions
        VideoSimd {
            type: Type::Scalar(ScalarType::B32),
            data: VideoSimdDetails,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
                src3: T,
            }
        },
        Vmad {
            type: Type::Scalar(ScalarType::B32),
            data: VmadDetails,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
                src3: T,
            }
        },
        Clmad {
            type: Type::Scalar(ScalarType::U64),
            data: bool,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
                src3: T,
            }
        },
        Packed8 {
            type: Type::Scalar(ScalarType::B32),
            data: Packed8Details,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: Option<T>,
            }
        },
        Isspacep {
            type: Type::Scalar(ScalarType::B64),
            data: StateSpace,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: Type::from(ScalarType::Pred)
                },
                src: T,
            }
        },
        // Conversions to and from the narrow floating point formats
        CvtMini {
            data: CvtMiniDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { data.dst_type() },
                    relaxed_type_check: true,
                },
                src1: {
                    repr: T,
                    type: { data.src_type() },
                    relaxed_type_check: true,
                },
                src2: {
                    repr: Option<T>,
                    type: { Type::Scalar(ScalarType::F32) },
                    relaxed_type_check: true,
                },
            }
        },
        CvtTf32 {
            data: CvtTf32Details,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::B32) },
                },
                src: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::F32) },
                },
            }
        },
        CvtPack {
            data: ScalarType,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::U32) },
                },
                src1: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::S32) },
                },
                src2: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::S32) },
                },
                src3: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::B32) },
                }
            }
        },
        Cvta {
            data: CvtaDetails,
            type: { if data.size == 64 { Type::Scalar(ScalarType::B64) } else { Type::Scalar(ScalarType::B32) } },
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
            display: {
                let non_coherent = if data.non_coherent {
                    ".nc"
                } else {
                    ""
                };
                write!(
                    f,
                    "ld{}{}{}{}{}",
                    data.qualifier,
                    data.state_space,
                    non_coherent,
                    data.caching,
                    data.typ,
                )?
            }
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
                src3: Option<T>,
            }
        },
        Membar {
            data: MemScope
        },
        MbarrierInit {
            type: Type::Scalar(ScalarType::B64),
            data: StateSpace,
            arguments<T>: {
                src_mbarrier: {
                    repr: T,
                    space: { data.clone() },
                },
                src_count: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U32),
                }
            }
        },
        MbarrierInval {
            type: Type::Scalar(ScalarType::B64),
            data: StateSpace,
            arguments<T>: {
                src_mbarrier: {
                    repr: T,
                    space: { data.clone() },
                }
            }
        },
        MbarrierArrive {
            type: Type::Scalar(ScalarType::B64),
            data: MbarrierArriveDetails,
            arguments<T>: {
                dst_state: Option<T>,
                src_mbarrier: {
                    repr: T,
                    space: { data.space },
                },
                src_count: {
                    repr: Option<T>,
                    type: Type::Scalar(ScalarType::U32),
                },
                src_tx_count: {
                    repr: Option<T>,
                    type: Type::Scalar(ScalarType::U32),
                },
                src_cta_mask: {
                    repr: Option<T>,
                    type: Type::Scalar(ScalarType::U32),
                }
            }
        },
        MbarrierTx {
            type: Type::Scalar(ScalarType::B64),
            data: MbarrierTxDetails,
            arguments<T>: {
                src_mbarrier: {
                    repr: T,
                    space: { data.space },
                },
                src_tx_count: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U32),
                },
                src_cta_mask: {
                    repr: Option<T>,
                    type: Type::Scalar(ScalarType::U32),
                }
            }
        },
        MbarrierWait {
            type: Type::Scalar(ScalarType::B64),
            data: MbarrierWaitDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: Type::Scalar(ScalarType::Pred),
                },
                src_mbarrier: {
                    repr: T,
                    space: { data.space },
                },
                src_state: {
                    repr: T,
                    type: Type::Scalar(if data.parity { ScalarType::U32 } else { ScalarType::B64 }),
                },
                src_time_hint: {
                    repr: Option<T>,
                    type: Type::Scalar(ScalarType::U32),
                }
            }
        },
        MbarrierPendingCount {
            arguments<T>: {
                dst: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U32),
                },
                src_state: {
                    repr: T,
                    type: Type::Scalar(ScalarType::B64),
                }
            }
        },
        Min {
            type: { Type::from(data.type_()) },
            data: MinMaxDetails,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
                src3: Option<T>,
            }
        },
        Mov {
            type: { &data.typ },
            data: MovDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { &data.typ },
                    relaxed_type_check: true,
                },
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
            },
            display: write!(f, "mul{}", data)?
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
            data: Option<PrmtMode>,
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
        Shfl {
            data: ShflDetails,
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
                src_membermask: Option<T>
            }
        },
        MatchSync {
            data: ScalarType,
            type: Type::Scalar(ScalarType::B32),
            arguments<T>: {
                dst: T,
                src: {
                    repr: T,
                    type: Type::Scalar(data.clone())
                },
                src_membermask: T
            }
        },
        MatchAllSync {
            data: ScalarType,
            type: Type::Scalar(ScalarType::B32),
            arguments<T>: {
                dst: T,
                dst_pred: {
                    repr: Option<T>,
                    type: Type::from(ScalarType::Pred)
                },
                src: {
                    repr: T,
                    type: Type::Scalar(data.clone())
                },
                src_membermask: T
            }
        },
        ElectSync {
            type: Type::Scalar(ScalarType::B32),
            arguments<T>: {
                dst: T,
                dst_pred: {
                    repr: Option<T>,
                    type: Type::from(ScalarType::Pred)
                },
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
            },
            display: {
                write!(
                    f,
                    "st{}{}{}{}",
                    data.qualifier,
                    data.state_space,
                    data.caching,
                    data.typ,
                )?
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
        Exit { },
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
        Vote {
            type: Type::Scalar(data.mode.type_()),
            data: VoteDetails,
            arguments<T>: {
                dst: T,
                src1: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::Pred) },
                },
                src2: {
                    repr: Option<T>,
                    type: { Type::Scalar(ScalarType::U32) },
                }
            }

        },
        ReduxSync {
            type: Type::Scalar(data.type_),
            data: ReduxSyncData,
            arguments<T>: {
                dst: T,
                src: T,
                src_membermask: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::U32) },
                }
            }
        },
        LdMatrix {
            type: data.get_loaded_type(),
            data: LdMatrixDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    relaxed_type_check: true,
                },
                src: {
                    repr: T,
                    space: { data.state_space },
                }
            }
        },
        StMatrix {
            type: data.get_loaded_type(),
            data: LdMatrixDetails,
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
        MovMatrix {
            type: Type::Scalar(ScalarType::B32),
            arguments<T>: {
                dst: T,
                src: T
            }
        },
        MmaSparse {
            data: MmaDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { data.mma_type(MmaMatrix::D) },
                },
                src1: {
                    repr: T,
                    type: { data.mma_type(MmaMatrix::A) },
                },
                src2: {
                    repr: T,
                    type: { data.mma_type(MmaMatrix::B) },
                },
                src3: {
                    repr: T,
                    type: { data.mma_type(MmaMatrix::C) },
                },
                src_metadata: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::B32) },
                },
                src_scale_a: {
                    repr: Option<T>,
                    type: { Type::Scalar(ScalarType::B32) },
                },
                src_scale_b: {
                    repr: Option<T>,
                    type: { Type::Scalar(ScalarType::B32) },
                }
            }
        },
        MmaBlockScale {
            data: MmaDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { data.mma_type(MmaMatrix::D) },
                },
                src1: {
                    repr: T,
                    type: { data.mma_type(MmaMatrix::A) },
                },
                src2: {
                    repr: T,
                    type: { data.mma_type(MmaMatrix::B) },
                },
                src3: {
                    repr: T,
                    type: { data.mma_type(MmaMatrix::C) },
                },
                src_scale_a: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::B32) },
                },
                src_scale_b: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::B32) },
                }
            }
        },
        WmmaLoad {
            type: data.fragment_type(),
            data: WmmaLoadStoreDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    relaxed_type_check: true,
                },
                src: {
                    repr: T,
                    space: { data.state_space },
                },
                src_stride: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::U32) },
                }
            }
        },
        WmmaStore {
            type: data.fragment_type(),
            data: WmmaLoadStoreDetails,
            arguments<T>: {
                src1: {
                    repr: T,
                    space: { data.state_space },
                },
                src2: {
                    repr: T,
                    relaxed_type_check: true,
                },
                src_stride: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::U32) },
                }
            }
        },
        WmmaMma {
            data: MmaDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { data.wmma_type(MmaMatrix::D) },
                    relaxed_type_check: true,
                },
                src1: {
                    repr: T,
                    type: { data.wmma_type(MmaMatrix::A) },
                    relaxed_type_check: true,
                },
                src2: {
                    repr: T,
                    type: { data.wmma_type(MmaMatrix::B) },
                    relaxed_type_check: true,
                },
                src3: {
                    repr: T,
                    type: { data.wmma_type(MmaMatrix::C) },
                    relaxed_type_check: true,
                }
            }
        },
        GridDepControl {
            data: crate::GridDepControlAction,
        },
        BarrierCluster {
            data: BarrierClusterDetails,
        },
        StAsync {
            type: { &data.type_ },
            data: StAsyncDetails,
            arguments<T>: {
                src_addr: {
                    repr: T,
                    space: { StateSpace::Shared },
                },
                src_value: T,
                src_mbarrier: {
                    repr: T,
                    type: Type::Scalar(ScalarType::B64),
                    space: { StateSpace::Shared },
                },
            }
        },
        StBulk {
            type: Type::Scalar(ScalarType::B8),
            data: StBulkDetails,
            arguments<T>: {
                src_addr: {
                    repr: T,
                    space: { data.space },
                },
                src_size: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U64),
                },
                src_init: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U64),
                },
            }
        },
        SetMaxNReg {
            data: u32,
        },
        ClusterLaunchTryCancel {
            type: Type::Scalar(ScalarType::B128),
            arguments<T>: {
                src_addr: {
                    repr: T,
                    space: { StateSpace::Shared },
                },
                src_mbarrier: {
                    repr: T,
                    type: Type::Scalar(ScalarType::B64),
                    space: { StateSpace::Shared },
                },
            }
        },
        ClusterLaunchQuery {
            type: { Type::from(data.dst_type()) },
            data: ClusterLaunchQueryDetails,
            arguments<T>: {
                dst: T,
                src: {
                    repr: T,
                    type: Type::Scalar(ScalarType::B128),
                    space: { data.space },
                },
            }
        },
        Mapa {
            type: Type::Scalar(data.type_),
            data: MapaDetails,
            arguments<T>: {
                dst: T,
                src_addr: T,
                src_rank: {
                    repr: T,
                    type: Type::Scalar(ScalarType::U32)
                },
            }
        },
        Getctarank {
            type: Type::Scalar(ScalarType::U32),
            data: MapaDetails,
            arguments<T>: {
                dst: T,
                src_addr: {
                    repr: T,
                    type: Type::Scalar(data.type_)
                },
            }
        },
        Wgmma {
            data: WgmmaDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { data.d_type() },
                },
                src_a: {
                    repr: T,
                    type: { data.a_type() },
                },
                src_b: {
                    repr: T,
                    type: Type::Scalar(ScalarType::B64),
                },
                // the incoming values of D (the same registers as dst)
                src_c: {
                    repr: T,
                    type: { data.d_type() },
                },
                src_scale_d: {
                    repr: Option<T>,
                    type: Type::Scalar(ScalarType::Pred),
                },
                src_metadata: {
                    repr: Option<T>,
                    type: Type::Scalar(ScalarType::B32),
                },
            }
        },
        WgmmaFence {
            data: WgmmaControl,
        },
        Mma {
            data: MmaDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { data.mma_type(MmaMatrix::D) },
                },
                src1: {
                    repr: T,
                    type: { data.mma_type(MmaMatrix::A) },
                },
                src2: {
                    repr: T,
                    type: { data.mma_type(MmaMatrix::B) },
                },
                src3: {
                    repr: T,
                    type: { data.mma_type(MmaMatrix::C) },
                }
            }
        },
        Copysign {
            type: Type::Scalar(data.clone()),
            data: ScalarType,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
            }
        },
        Prefetch {
            type: !,
            data: PrefetchData,
            arguments<T>: {
                src: {
                    repr: T,
                    type: Type::from(ScalarType::B8),
                    space: { data.space },
                }
            }
        },
        Sad {
            type: Type::Scalar(data.clone()),
            data: ScalarType,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
                src3: T,
            }
        },
        Dp2a {
            type: !,
            data: Dp2aData,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { Type::Scalar(data.ctype()) },

                },
                src1:  {
                    repr: T,
                    type: { Type::Scalar(data.atype) },

                },
                src2:  {
                    repr: T,
                    type: { Type::Scalar(data.btype) },

                },
                src3:  {
                    repr: T,
                    type: { Type::Scalar(data.ctype()) },

                }
            }
        },
        // vshr.u32.u32.u32.clamp.add, kept apart from VideoScalar so that its
        // code does not change
        Vshr {
            type: Type::Scalar(ScalarType::U32),
            data: VshData,
            arguments<T>: {
                dst: T,
                src1: T,
                src2: T,
                src3: T
            }
        },
        Tex {
            type: !,
            data: TexData,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { Type::Vector(4, data.dtype) },

                },
                src_ptr:  {
                    repr: T,
                    type: {
                        if data.type_ == TexType::Texref {
                            Type::Texref
                        } else {
                            Type::Scalar(ScalarType::U64)
                        }
                    },
                    space: {
                        if data.type_ == TexType::Texref {
                            StateSpace::Global
                        } else {
                            StateSpace::Reg
                        }
                    },

                },
                src_coord:  {
                    repr: T,
                    type: { data.coord_type() },

                },
            }
        },
        Bfind {
            type: !,
            data: BfindDetails,
            arguments<T>: {
                dst: {
                    repr: T,
                    type: { Type::Scalar(ScalarType::U32) }

                },
                src:  {
                    repr: T,
                    type: { Type::Scalar(data.type_) }

                }
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
            ParsedOperand::VecPack(vec) => {
                ParsedOperand::VecPack(
                    vec.into_iter()
                        .map(|element| {
                            Ok(match element {
                                Some(RegOrImmediate::Reg(ident)) => Some(RegOrImmediate::Reg(
                                    (self)(ident, type_space, is_dst, relaxed_type_check)?,
                                )),
                                Some(RegOrImmediate::Imm(imm)) => Some(RegOrImmediate::Imm(imm)),
                                None => None,
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                )
            }
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

pub enum MultiVariable<ID> {
    Parameterized {
        info: VariableInfo<ID>,
        name: ID,
        count: u32,
    },
    Names {
        info: VariableInfo<ID>,
        names: Vec<ID>,
    },
    /// A list of register names of which some are parameterized, as in
    /// `.reg .b32 %r<8>, %q<4>, %x;`.
    List {
        info: VariableInfo<ID>,
        names: Vec<(ID, Option<u32>)>,
    },
}

#[derive(Clone)]
pub struct VariableInfo<ID> {
    pub align: Option<u32>,
    pub v_type: Type,
    pub state_space: StateSpace,
    pub array_init: Vec<RegOrImmediate<ID>>,
}

#[derive(Clone)]
pub struct Variable<ID> {
    pub info: VariableInfo<ID>,
    pub name: ID,
}

impl<ID: std::fmt::Display> std::fmt::Display for Variable<ID> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info.state_space)?;

        if let Some(align) = self.info.align {
            write!(f, " .align {}", align)?;
        }

        let (vector_size, scalar_type, array_dims) = match &self.info.v_type {
            Type::Scalar(scalar_type) => (None, *scalar_type, &vec![]),
            Type::Vector(size, scalar_type) => (Some(*size), *scalar_type, &vec![]),
            Type::Array(vector_size, scalar_type, array_dims) => {
                (vector_size.map(|s| s.get()), *scalar_type, array_dims)
            }
            Type::Texref => {
                return write!(f, " .texref");
            }
        };

        if let Some(size) = vector_size {
            write!(f, " .v{}", size)?;
        }

        write!(f, " {} {}", scalar_type, self.name)?;

        for dim in array_dims {
            write!(f, "[{}]", dim)?;
        }

        if self.info.array_init.len() > 0 {
            todo!("Need to interpret the array initializer data as the appropriate type");
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
    Texref,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Scalar(scalar_type) => write!(f, "{}", scalar_type),
            Type::Vector(count, scalar_type) => write!(f, ".v{}{}", count, scalar_type),
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

    pub fn layout(&self) -> Option<Layout> {
        Some(match self {
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
                let element_layout = Type::maybe_vector_parsed(*non_zero, *scalar).layout()?;
                let len = vec.iter().copied().reduce(std::ops::Mul::mul).unwrap_or(0);
                unsafe {
                    Layout::from_size_align_unchecked(
                        element_layout.size() * (len as usize),
                        element_layout.align(),
                    )
                }
            }
            Type::Texref => return None,
        })
    }
}

impl ScalarType {
    pub fn from_size(size: u8) -> Option<Self> {
        Some(match size {
            1 => ScalarType::B8,
            2 => ScalarType::B16,
            4 => ScalarType::B32,
            8 => ScalarType::B64,
            16 => ScalarType::B128,
            _ => return None,
        })
    }
    pub fn size_of(self) -> u8 {
        match self {
            ScalarType::U8 | ScalarType::S8 | ScalarType::B8 => 1,
            ScalarType::U16
            | ScalarType::S16
            | ScalarType::B16
            | ScalarType::F16
            | ScalarType::BF16
            | ScalarType::E4m3x2
            | ScalarType::E5m2x2 => 2,
            ScalarType::U32
            | ScalarType::S32
            | ScalarType::B32
            | ScalarType::F32
            | ScalarType::U16x2
            | ScalarType::S16x2
            | ScalarType::F16x2
            | ScalarType::BF16x2 => 4,
            ScalarType::U64 | ScalarType::S64 | ScalarType::B64 | ScalarType::F64 | ScalarType::F32x2 => 8,
            ScalarType::B128 => 16,
            ScalarType::Pred => 1,
        }
    }

    pub fn bit_size(self) -> u8 {
        self.layout().size() as u8 * 8u8
    }

    pub fn layout(self) -> Layout {
        match self {
            ScalarType::U8 | ScalarType::S8 | ScalarType::B8 => Layout::new::<u8>(),
            ScalarType::U16
            | ScalarType::S16
            | ScalarType::B16
            | ScalarType::F16
            | ScalarType::BF16
            | ScalarType::E4m3x2
            | ScalarType::E5m2x2 => Layout::new::<u16>(),
            ScalarType::U32
            | ScalarType::S32
            | ScalarType::B32
            | ScalarType::F32
            | ScalarType::U16x2
            | ScalarType::S16x2
            | ScalarType::F16x2
            | ScalarType::BF16x2 => Layout::new::<u32>(),
            ScalarType::U64 | ScalarType::S64 | ScalarType::B64 | ScalarType::F64 | ScalarType::F32x2 => {
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
            ScalarType::F32x2 => ScalarKind::Float,
            ScalarType::F64 => ScalarKind::Float,
            ScalarType::BF16 => ScalarKind::Float,
            ScalarType::BF16x2 => ScalarKind::Float,
            ScalarType::E4m3x2 => ScalarKind::Float,
            ScalarType::E5m2x2 => ScalarKind::Float,
            ScalarType::Pred => ScalarKind::Pred,
        }
    }

    pub fn packed_type(&self) -> Option<ScalarType> {
        match self {
            ScalarType::E4m3x2 => Some(ScalarType::B8),
            ScalarType::E5m2x2 => Some(ScalarType::B8),
            ScalarType::F16x2 => Some(ScalarType::F16),
            ScalarType::F32x2 => Some(ScalarType::F32),
            ScalarType::BF16x2 => Some(ScalarType::BF16),
            ScalarType::U16x2 => Some(ScalarType::U16),
            ScalarType::S16x2 => Some(ScalarType::S16),
            ScalarType::S16
            | ScalarType::BF16
            | ScalarType::U32
            | ScalarType::S8
            | ScalarType::S32
            | ScalarType::Pred
            | ScalarType::B8
            | ScalarType::U64
            | ScalarType::B16
            | ScalarType::S64
            | ScalarType::B32
            | ScalarType::U8
            | ScalarType::F32
            | ScalarType::B64
            | ScalarType::B128
            | ScalarType::U16
            | ScalarType::F64
            | ScalarType::F16 => None,
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
pub struct ShflDetails {
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
    // immediate src-size
    pub src_size: Option<u64>,
    // the src_size argument is the ignore-src predicate
    pub ignore_src: bool,
}

pub struct CpAsyncMbarrierArriveDetails {
    pub space: StateSpace,
    pub noinc: bool,
}

pub struct CpAsyncBulkDetails {
    pub dst_space: StateSpace,
    pub src_space: StateSpace,
    pub multicast: bool,
}

pub struct CpReduceAsyncBulkDetails {
    pub dst_space: StateSpace,
    pub src_space: StateSpace,
    pub op: AtomicOp,
    pub type_: ScalarType,
}

pub struct MbarrierArriveDetails {
    pub space: StateSpace,
    // arrive_drop: also decrements the expected arrival count
    pub drop: bool,
}

pub struct MbarrierTxDetails {
    pub space: StateSpace,
    // complete_tx (decrements the transaction count) instead of expect_tx
    pub complete: bool,
}

pub struct MbarrierWaitDetails {
    pub space: StateSpace,
    pub parity: bool,
    // try_wait (potentially blocking) instead of test_wait
    pub try_wait: bool,
}

pub struct ShfDetails {
    pub direction: ShiftDirection,
    pub mode: FunnelShiftMode,
}
#[cfg_attr(debug_assertions, derive(PartialEq))]
#[derive(Clone, Copy, Display, Debug)]
pub enum RegOrImmediate<Ident> {
    Reg(Ident),
    Imm(ImmediateValue),
}

#[derive(Clone)]
pub enum ParsedOperand<Ident> {
    Reg(Ident),
    RegOffset(Ident, i32),
    Imm(ImmediateValue),
    VecMember(Ident, u8),
    // None denotes a sink; retain its position in the vector.
    VecPack(Vec<Option<RegOrImmediate<Ident>>>),
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
                    match item {
                        Some(item) => write!(f, "{}", item)?,
                        None => f.write_char('_')?,
                    }
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

#[cfg_attr(debug_assertions, derive(PartialEq))]
#[derive(Copy, Clone, Debug)]
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

    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            ImmediateValue::F64(n) => Some(n),
            _ => None,
        }
    }

    pub fn get_bits(&self) -> u64 {
        match self {
            ImmediateValue::U64(n) => *n,
            ImmediateValue::S64(n) => *n as u64,
            ImmediateValue::F32(n) => n.to_bits() as u64,
            ImmediateValue::F64(n) => n.to_bits(),
        }
    }
}

impl std::fmt::Display for ImmediateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImmediateValue::U64(n) => write!(f, "{}U", n)?,
            ImmediateValue::S64(n) => write!(f, "{}", n)?,
            ImmediateValue::F32(n) => write!(f, "0f{:x}", n.to_bits())?,
            ImmediateValue::F64(n) => write!(f, "0d{:x}", n.to_bits())?,
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

impl Display for StCacheOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StCacheOperator::Writeback => {}
            StCacheOperator::L2Only => write!(f, ".cg")?,
            StCacheOperator::Streaming => write!(f, ".cs")?,
            StCacheOperator::Writethrough => write!(f, ".wt")?,
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LdCacheOperator {
    Cached,
    L2Only,
    Streaming,
    LastUse,
    Uncached,
}

impl Display for LdCacheOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LdCacheOperator::Cached => {}
            LdCacheOperator::L2Only => write!(f, ".cg")?,
            LdCacheOperator::Streaming => write!(f, ".cs")?,
            LdCacheOperator::LastUse => write!(f, ".lu")?,
            LdCacheOperator::Uncached => write!(f, ".cv")?,
        }
        Ok(())
    }
}

pub enum CpAsyncCacheOperator {
    Cached,
    L2Only,
}

pub enum CarryKind {
    CarryIn,
    CarryOut,
    CarryInCarryOut,
}

pub struct CarryDetails {
    pub kind: CarryKind,
    pub type_: ScalarType,
}

pub struct MadCarryDetails {
    pub kind: CarryKind,
    pub control: MulIntControl,
    pub type_: ScalarType,
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

impl std::fmt::Display for ArithDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArithDetails::Integer(int) => {
                write!(f, "{}", int)?;
            }
            ArithDetails::Float(float) => {
                write!(f, "{}", float)?;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct ArithInteger {
    pub type_: ScalarType,
    pub saturate: bool,
}

impl std::fmt::Display for ArithInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.saturate {
            write!(f, ".sat")?;
        }
        write!(f, "{}", self.type_)?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct ArithFloat {
    pub type_: ScalarType,
    pub rounding: RoundingMode,
    pub flush_to_zero: Option<bool>,
    pub saturate: bool,
    /// `.relu` (fma): negative results become +0, NaN stays NaN
    pub relu: bool,
    // From PTX documentation: https://docs.nvidia.com/cuda/parallel-thread-execution/#mixed-precision-floating-point-instructions-add
    // Note that an add instruction with an explicit rounding modifier is treated conservatively by
    // the code optimizer. An add instruction with no rounding modifier defaults to
    // round-to-nearest-even and may be optimized aggressively by the code optimizer. In particular,
    // mul/add sequences with no rounding modifiers may be optimized to use fused-multiply-add
    // instructions on the target device.
    pub is_fusable: bool,
}

impl std::fmt::Display for ArithFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rounding)?;
        if let Some(true) = self.flush_to_zero {
            write!(f, ".ftz")?;
        }
        if self.saturate {
            write!(f, ".sat")?;
        }
        write!(f, "{}", self.type_)?;
        Ok(())
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

impl Display for LdStQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LdStQualifier::Weak => {}
            LdStQualifier::Volatile => write!(f, ".volatile")?,
            LdStQualifier::Relaxed(scope) => write!(f, ".relaxed{scope}")?,
            LdStQualifier::Acquire(scope) => write!(f, ".acquire{scope}")?,
            LdStQualifier::Release(scope) => write!(f, ".release{scope}")?,
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum RoundingMode {
    NearestEven,
    Zero,
    NegativeInf,
    PositiveInf,
}

impl std::fmt::Display for RoundingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoundingMode::NearestEven => write!(f, ".rn")?,
            RoundingMode::Zero => write!(f, ".rz")?,
            RoundingMode::NegativeInf => write!(f, ".rm")?,
            RoundingMode::PositiveInf => write!(f, ".rp")?,
        }
        Ok(())
    }
}

pub struct LdDetails {
    pub qualifier: LdStQualifier,
    pub state_space: StateSpace,
    pub caching: LdCacheOperator,
    pub typ: Type,
    pub non_coherent: bool,
}

/// Shapes of `ldmatrix` and `stmatrix`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MatrixShape {
    M8n8,
    M16n16,
    M8n16,
    M16n8,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MatrixNumber {
    X1,
    X2,
    X4,
}

impl MatrixNumber {
    pub fn get(&self) -> u8 {
        match self {
            MatrixNumber::X1 => 1,
            MatrixNumber::X2 => 2,
            MatrixNumber::X4 => 4,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MatrixLayout {
    Row,
    Col,
}

/// Element format of `ldmatrix` and `stmatrix`: 16-bit or 8-bit elements, or
/// (sm_100) packed 6-bit, 4-bit and signed 4-bit elements expanded to 8-bit
/// containers (`.b8x16.b6x16_p32`, `.b8x16.b4x16_p64`, `.s8.s4`).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LdMatrixFormat {
    B16,
    B8,
    B6x16P32,
    B4x16P64,
    S4,
}

#[derive(Copy, Clone)]
pub struct LdMatrixDetails {
    pub shape: MatrixShape,
    pub number: MatrixNumber,
    pub transpose: bool,
    pub state_space: StateSpace,
    pub format: LdMatrixFormat,
}

impl LdMatrixDetails {
    /// Number of 32-bit registers of the destination (or source) vector.
    pub fn registers(&self) -> u8 {
        let per_matrix = if self.shape == MatrixShape::M16n16 { 2 } else { 1 };
        self.number.get() * per_matrix
    }

    pub fn get_loaded_type(&self) -> Type {
        Type::Vector(self.registers(), ScalarType::B32)
    }
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
    NoReturn,
    /// .reqnctapercluster: the CTAs of a cluster along x, y and z
    ReqNctaPerCluster(u32, u32, u32),
    ExplicitCluster,
    MaxClusterRank(u32),
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
    pub ptx_version: (u8, u8),
    pub sm_version: u32,
    pub directives: Vec<Directive<'input, ParsedOperand<&'input str>>>,
    pub invalid_directives: usize,
    pub address_size: u8,
}

impl Module<'_> {
    pub fn empty() -> Self {
        Module {
            ptx_version: (1, 0),
            sm_version: 0,
            directives: Vec::new(),
            invalid_directives: usize::MAX,
            address_size: 64,
        }
    }
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

impl std::fmt::Display for MulDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MulDetails::Integer { type_, control } => {
                write!(f, "{}{}", type_, control)
            }
            MulDetails::Float(arith) => {
                write!(f, "{}", arith)
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MulIntControl {
    Low,
    High,
    Wide,
}

impl std::fmt::Display for MulIntControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MulIntControl::Low => write!(f, ".lo"),
            MulIntControl::High => write!(f, ".hi"),
            MulIntControl::Wide => write!(f, ".wide"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Mul24Details {
    pub type_: ScalarType,
    pub control: Mul24Control,
}

/// mad24.mode{.sat}.type: the 24-bit product's low 32 bits (lo) or bits 47..16 (hi), plus c; .sat (hi.s32 only)
/// saturates the addition
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mad24Details {
    pub type_: ScalarType,
    pub control: Mul24Control,
    pub saturate: bool,
}

/// slct.dtype.ctype: a when c >= 0, else b (for f32 c, -0 counts as >= 0 and NaN does not; .ftz flushes c first)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlctDetails {
    pub type_: ScalarType,
    pub ctype: ScalarType,
    pub flush_to_zero: bool,
}

/// st.async (red: None) / red.async: a store, or an atomic operation, on shared::cluster memory that completes
/// its bytes on an mbarrier
#[derive(Clone)]
pub struct StAsyncDetails {
    pub type_: Type,
    pub red: Option<AtomicOp>,
    pub scalar: ScalarType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StBulkDetails {
    pub space: StateSpace,
}

/// clusterlaunchcontrol.query_cancel: is_canceled (a predicate), or get_first_ctaid (x, y, z or all as v4.b32)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ClusterLaunchQueryDetails {
    pub ctaid: Option<Option<u8>>,
    /// where the response is: a .b128 register, or shared memory
    pub space: StateSpace,
}

impl ClusterLaunchQueryDetails {
    pub fn dst_type(&self) -> Type {
        match self.ctaid {
            None => Type::Scalar(ScalarType::Pred),
            Some(Some(_)) => Type::Scalar(ScalarType::B32),
            Some(None) => Type::Vector(4, ScalarType::B32),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BarrierClusterDetails {
    /// arrive (else wait)
    pub arrive: bool,
    /// .relaxed arrive: no release ordering
    pub relaxed: bool,
}

/// The address form of mapa and getctarank: shared::cluster (a 32-bit shared address) or generic (64-bit)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MapaDetails {
    pub shared: bool,
    pub type_: ScalarType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MixedFloatOp {
    Add,
    Sub,
    Mul,
    Fma,
}

/// The mixed-precision forms (PTX ISA 8.6 and 9.x, sm_100 and later): add / sub / fma.f32.{f16,bf16} convert the
/// 16-bit operands to f32 and compute in f32 (the third source of fma, and the second of add / sub, is f32);
/// add / sub / mul / fma.rz.{f16x2,bf16x2}.f32x2.f32x2 compute both lanes in f32 and round the results to the
/// 16-bit type with the given rounding
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MixedFloatDetails {
    pub op: MixedFloatOp,
    pub dtype: ScalarType,
    pub atype: ScalarType,
    pub btype: ScalarType,
    pub rounding: RoundingMode,
    pub flush_to_zero: bool,
    pub saturate: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestpOp {
    Finite,
    Infinite,
    Number,
    NotANumber,
    Normal,
    Subnormal,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TestpDetails {
    pub op: TestpOp,
    pub type_: ScalarType,
}

pub struct SetData {
    pub dtype: ScalarType,
    pub base: SetpData,
}

#[derive(Copy, Clone)]
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
        // comparisons depend on subnormal inputs: without .ftz the f16 / f64 forms keep them
        let flush_to_zero = match (ftz, type_) {
            (_, ScalarType::F32 | ScalarType::F16 | ScalarType::F16x2) => Some(ftz),
            (false, ScalarType::BF16 | ScalarType::BF16x2 | ScalarType::F64) => Some(false),
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
    // This is a sad hack for `instruction_mode_to_global_mode` pass,
    // The problem is that `instruction_mode_to_global_mode` pass right now
    // treats function calls rather suboptimally:
    // If basic blocks A and B call into function F. It models it by first
    // splitting A into sequence [A, F, A'] and B into [B, F, B'], where A'
    // and B' are the continuations after the function call.
    // This would imply that B' is reachable from A (through A -> F -> B'),
    // which is not technically true, but true for the purpose of the analysis.
    // The problem arises when we replace sregs with function calls, then we
    // suddenly have a lot more function calls, which leads to cfg exploding
    //  and quality of `instruction_mode_to_global_mode` pass degrading
    // significantly.
    // One day `instruction_mode_to_global_mode` pass will be rewritten to
    // duplicate and deduplicate function calls properly, but until then,
    // we use this flag to mark function calls that are not actually function
    // calls
    pub is_external: bool,
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
            is_external: self.is_external,
        })
    }
}

pub struct CreatePolicyFractionalDetails {
    pub primary_priority: EvictionPriority,
    pub fraction: f32,
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
        relu: bool,
        satfinite: bool,
    },
    FPTruncate {
        // float rounding
        rounding: RoundingMode,
        is_integer_rounding: bool,
        flush_to_zero: Option<bool>,
        saturate: bool,
        relu: bool,
        satfinite: bool,
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
        relu: bool,
        satfinite: bool,
        dst: ScalarType,
        src: ScalarType,
    ) -> Self {
        if (relu || satfinite) && !(dst.kind() == ScalarKind::Float && src.kind() == ScalarKind::Float)
        {
            errors.push(PtxError::SyntaxError(
                ".relu and .satfinite in cvt need floating point types".to_string(),
            ));
        }
        // Modifier .ftz can only be specified when either .dtype or .atype is .f32 and applies only to single precision (.f32) inputs and results.
        let flush_to_zero = match (dst, src) {
            (ScalarType::F32, _) | (_, ScalarType::F32) => Some(ftz),
            _ => {
                // .ftz needs an f32 source or destination; ptxas rejects it
                // elsewhere, PTX-formalization accepts it and flushes f16
                // and f64 subnormals too. It is ignored here
                let _ = ftz;
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
        let dst_size = if dst.packed_type().is_some() {
            dst.size_of() / 2
        } else {
            dst.size_of()
        };
        let mode = match (dst.kind(), src.kind()) {
            (ScalarKind::Float, ScalarKind::Float) => match dst_size.cmp(&src.size_of()) {
                // Conversions between f16 and bf16 round like truncations
                Ordering::Less | Ordering::Equal
                    if dst_size < src.size_of() || (dst != src && dst.packed_type().is_none()) =>
                {
                    let (rounding, is_integer_rounding) = unwrap_rounding();
                    CvtMode::FPTruncate {
                        rounding,
                        is_integer_rounding,
                        flush_to_zero,
                        saturate,
                        relu,
                        satfinite,
                    }
                }
                Ordering::Less => unreachable!(),
                Ordering::Equal if relu || satfinite => {
                    errors.push(PtxError::SyntaxError(
                        ".relu and .satfinite in cvt need different types".to_string(),
                    ));
                    CvtMode::Bitcast
                }
                Ordering::Equal => CvtMode::FPRound {
                    integer_rounding: rounding.map(|(rnd, _)| rnd),
                    flush_to_zero,
                    saturate,
                },
                Ordering::Greater => {
                    if rounding.is_some()
                        && !(src == ScalarType::E4m3x2 || src == ScalarType::E5m2x2)
                    {
                        errors.push(PtxError::SyntaxError(
                            "should not have rounding mode when dst is larger than src in cvt"
                                .to_string(),
                        ));
                    }
                    CvtMode::FPExtend {
                        flush_to_zero,
                        saturate,
                        relu,
                        satfinite,
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
    pub size: u8,
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
    /// .relu of s32 / s16x2: negative results become 0
    SignedRelu(ScalarType),
    Unsigned(ScalarType),
    Float(MinMaxFloat),
}

impl MinMaxDetails {
    pub fn type_(&self) -> ScalarType {
        match self {
            MinMaxDetails::Signed(t) | MinMaxDetails::SignedRelu(t) => *t,
            MinMaxDetails::Unsigned(t) => *t,
            MinMaxDetails::Float(float) => float.type_,
        }
    }
}

#[derive(Copy, Clone)]
pub struct MinMaxFloat {
    pub flush_to_zero: Option<bool>,
    pub nan: bool,
    // .xorsign.abs: compare the magnitudes, the result has the sign
    // sign(a) xor sign(b)
    pub xorsign_abs: bool,
    pub type_: ScalarType,
}

#[derive(Copy, Clone)]
pub struct RcpData {
    pub kind: RcpKind,
    pub flush_to_zero: Option<bool>,
    pub type_: ScalarType,
    /// .sat of sqrt.rnd.f32 (accepted by PTX-formalization): the result clamped to [0, 1]
    pub saturate: bool,
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
    /// .sat (PTX-formalization accepts it on div.rnd.f32): the result clamped to [0, 1]
    pub saturate: bool,
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

pub struct VoteDetails {
    pub mode: VoteMode,
    pub negate: bool,
}

impl VoteMode {
    fn type_(self) -> ScalarType {
        match self {
            VoteMode::All | VoteMode::Any | VoteMode::Uni => ScalarType::Pred,
            VoteMode::Ballot => ScalarType::B32,
        }
    }
}

pub struct ReduxSyncData {
    pub type_: ScalarType,
    pub reduction: Reduction,
}

/// Element types of the warp-level matrix instructions (`mma`, `wmma`).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MmaType {
    F16,
    Bf16,
    Tf32,
    F32,
    F64,
    S8,
    U8,
    S4,
    U4,
    B1,
    S32,
    E4m3,
    E5m2,
    E3m2,
    E2m3,
    E2m1,
    /// e2m1 packed two per byte (block scaled `kind::mxf4`, `kind::mxf4nvf4`)
    E2m1x2,
}

impl MmaType {
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "f16" => MmaType::F16,
            "bf16" => MmaType::Bf16,
            "tf32" => MmaType::Tf32,
            "f32" => MmaType::F32,
            "f64" => MmaType::F64,
            "s8" => MmaType::S8,
            "u8" => MmaType::U8,
            "s4" => MmaType::S4,
            "u4" => MmaType::U4,
            "b1" => MmaType::B1,
            "s32" => MmaType::S32,
            "e4m3" => MmaType::E4m3,
            "e5m2" => MmaType::E5m2,
            "e3m2" => MmaType::E3m2,
            "e2m3" => MmaType::E2m3,
            "e2m1" => MmaType::E2m1,
            "e2m1x2" => MmaType::E2m1x2,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            MmaType::F16 => "f16",
            MmaType::Bf16 => "bf16",
            MmaType::Tf32 => "tf32",
            MmaType::F32 => "f32",
            MmaType::F64 => "f64",
            MmaType::S8 => "s8",
            MmaType::U8 => "u8",
            MmaType::S4 => "s4",
            MmaType::U4 => "u4",
            MmaType::B1 => "b1",
            MmaType::S32 => "s32",
            MmaType::E4m3 => "e4m3",
            MmaType::E5m2 => "e5m2",
            MmaType::E3m2 => "e3m2",
            MmaType::E2m3 => "e2m3",
            MmaType::E2m1 => "e2m1",
            MmaType::E2m1x2 => "e2m1x2",
        }
    }

    pub fn is_float(self) -> bool {
        !matches!(
            self,
            MmaType::S8 | MmaType::U8 | MmaType::S4 | MmaType::U4 | MmaType::B1 | MmaType::S32
        )
    }

    /// Width of an element in a fragment register, in bits. The 6-bit and
    /// 4-bit floating point elements of `mma .kind::f8f6f4` are in 8-bit
    /// containers.
    pub fn bits(self) -> u32 {
        match self {
            MmaType::F64 => 64,
            MmaType::Tf32 | MmaType::F32 | MmaType::S32 => 32,
            MmaType::F16 | MmaType::Bf16 => 16,
            MmaType::S4 | MmaType::U4 | MmaType::E2m1x2 => 4,
            MmaType::B1 => 1,
            _ => 8,
        }
    }
}

/// The matrices of `D = A * B + C`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MmaMatrix {
    A,
    B,
    C,
    D,
}

/// The operation of single-bit `mma`: `.xor.popc` or `.and.popc`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MmaB1Op {
    Xor,
    And,
}

/// The scale factors of block scaled `mma.sync` (`.block_scale`): every row
/// of A and column of B is cut into `chunks` blocks (`.scale_vec::NX`), each
/// multiplied by a scale factor (`.ue8m0`, or `.ue4m3` with `ue4m3`). The
/// factors of A are byte `a_byte + j` of the scale register of the thread
/// `a_thread` of the selector (the `{byte-id, thread-id}` operands, which
/// must be immediates), and likewise for B.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct MmaBlockScale {
    pub chunks: u32,
    pub ue4m3: bool,
    pub a_byte: u32,
    pub a_thread: u32,
    pub b_byte: u32,
    pub b_thread: u32,
}

/// `mma.sync` and `wmma.mma`: D (m x n) = A (m x k) * B (k x n) + C.
#[derive(Copy, Clone)]
pub struct MmaDetails {
    pub m: u32,
    pub n: u32,
    pub k: u32,
    pub alayout: MatrixLayout,
    pub blayout: MatrixLayout,
    pub dtype: MmaType,
    pub atype: MmaType,
    pub btype: MmaType,
    pub ctype: MmaType,
    pub satfinite: bool,
    pub b1_op: Option<MmaB1Op>,
    pub block_scale: Option<MmaBlockScale>,
    /// Sparse `mma.sp`: A is 2:4 (1:2 for `tf32`, 4:8 for 4-bit elements)
    /// structured sparse, its fragment holds the non-zero elements and the
    /// metadata operand their positions; the value is the sparsity selector
    /// (an immediate).
    pub sparse: Option<u32>,
}

impl MmaDetails {
    /// Whether this is one of the `mma.sync` variants that are supported:
    /// the PTX shapes of the `f16`, `bf16`, `tf32`, `f64`, integer, single
    /// bit and 8-bit floating point (`kind::f8f6f4` included) types.
    pub fn is_supported_mma(&self, kind_f8f6f4: bool) -> bool {
        use MmaType::*;
        let shape = (self.m, self.n, self.k);
        let (a, b, c, d) = (self.atype, self.btype, self.ctype, self.dtype);
        let float_accumulator = matches!(c, F16 | F32) && matches!(d, F16 | F32);
        let integer_accumulator = c == S32 && d == S32;
        let integer = matches!(a, S8 | U8 | S4 | U4);
        if (self.b1_op.is_some() != (a == B1)) || (self.satfinite && !integer) {
            return false;
        }
        if kind_f8f6f4 {
            let f8f6f4 = |t| matches!(t, E4m3 | E5m2 | E3m2 | E2m3 | E2m1);
            return shape == (16, 8, 32) && f8f6f4(a) && f8f6f4(b) && float_accumulator;
        }
        match (a, b) {
            (F16, F16) => matches!(shape, (16, 8, 8) | (16, 8, 16)) && float_accumulator,
            (Bf16, Bf16) => matches!(shape, (16, 8, 8) | (16, 8, 16)) && c == F32 && d == F32,
            (Tf32, Tf32) => matches!(shape, (16, 8, 4) | (16, 8, 8)) && c == F32 && d == F32,
            (F64, F64) => {
                matches!(shape, (8, 8, 4) | (16, 8, 4) | (16, 8, 8) | (16, 8, 16))
                    && c == F64
                    && d == F64
            }
            (S8 | U8, S8 | U8) => {
                matches!(shape, (8, 8, 16) | (16, 8, 16) | (16, 8, 32)) && integer_accumulator
            }
            (S4 | U4, S4 | U4) => {
                matches!(shape, (8, 8, 32) | (16, 8, 32) | (16, 8, 64)) && integer_accumulator
            }
            (B1, B1) => {
                matches!(shape, (8, 8, 128) | (16, 8, 128) | (16, 8, 256)) && integer_accumulator
            }
            (E4m3 | E5m2, E4m3 | E5m2) => {
                matches!(shape, (16, 8, 16) | (16, 8, 32)) && float_accumulator
            }
            _ => false,
        }
    }

    /// Whether this is a supported block scaled `mma.sync` (`fp4`: of kind
    /// `mxf4` or `mxf4nvf4`).
    pub fn is_supported_block_scale(&self, scale: &MmaBlockScale, fp4: bool) -> bool {
        use MmaType::*;
        let shape = (self.m, self.n, self.k);
        if self.ctype != F32 || self.dtype != F32 || self.satfinite || self.b1_op.is_some() {
            return false;
        }
        if fp4 {
            shape == (16, 8, 64)
                && self.atype == E2m1x2
                && self.btype == E2m1x2
                && matches!(scale.chunks, 2 | 4)
        } else {
            let f8f6f4 = |t| matches!(t, E4m3 | E5m2 | E3m2 | E2m3 | E2m1);
            shape == (16, 8, 32)
                && f8f6f4(self.atype)
                && f8f6f4(self.btype)
                && scale.chunks == 1
                && !scale.ue4m3
        }
    }

    /// Whether this is a supported `mma.sp` (with the block scaling of
    /// `scale` if any): m16n8 with the depths of the PTX ISA.
    pub fn is_supported_sparse(
        &self,
        kind_f8f6f4: bool,
        scale: Option<(&MmaBlockScale, bool)>,
    ) -> bool {
        use MmaType::*;
        let (depths, dense_k): (&[u32], u32) = match self.atype {
            F16 | Bf16 => (&[16, 32], 16),
            Tf32 => (&[8, 16], 8),
            S8 | U8 => (&[32, 64], 32),
            S4 | U4 => (&[64, 128], 64),
            E2m1x2 => (&[128], 64),
            E4m3 | E5m2 | E3m2 | E2m3 | E2m1 => (&[64], 32),
            _ => return false,
        };
        // the types are those of a dense mma
        let dense = MmaDetails {
            k: dense_k,
            sparse: None,
            ..*self
        };
        let types = match scale {
            Some((scale, fp4)) => dense.is_supported_block_scale(scale, fp4),
            None => dense.is_supported_mma(kind_f8f6f4),
        };
        self.m == 16 && self.n == 8 && depths.contains(&self.k) && types
    }

    /// Whether this is a supported `wmma.mma`: all the shapes and types of
    /// the PTX ISA (`.rn` only for `f64`).
    pub fn is_supported_wmma(&self) -> bool {
        use MmaType::*;
        let shape = (self.m, self.n, self.k);
        let (a, c, d) = (self.atype, self.ctype, self.dtype);
        let integer = matches!(a, S8 | U8 | S4 | U4);
        if self.btype != a || (self.b1_op.is_some() != (a == B1)) || (self.satfinite && !integer)
        {
            return false;
        }
        let k16 = matches!(shape, (16, 16, 16) | (32, 8, 16) | (8, 32, 16));
        match a {
            F16 => k16 && matches!(c, F16 | F32) && matches!(d, F16 | F32),
            Bf16 => k16 && c == F32 && d == F32,
            S8 | U8 => k16 && c == S32 && d == S32,
            Tf32 => shape == (16, 16, 8) && c == F32 && d == F32,
            F64 => shape == (8, 8, 4) && c == F64 && d == F64,
            S4 | U4 => shape == (8, 8, 32) && c == S32 && d == S32,
            B1 => shape == (8, 8, 128) && c == S32 && d == S32,
            _ => false,
        }
    }

    pub fn element_type(&self, matrix: MmaMatrix) -> MmaType {
        match matrix {
            MmaMatrix::A => self.atype,
            MmaMatrix::B => self.btype,
            MmaMatrix::C => self.ctype,
            MmaMatrix::D => self.dtype,
        }
    }

    /// Rows and columns of a matrix.
    pub fn matrix_size(&self, matrix: MmaMatrix) -> (u32, u32) {
        match matrix {
            MmaMatrix::A => (self.m, self.k),
            MmaMatrix::B => (self.k, self.n),
            MmaMatrix::C | MmaMatrix::D => (self.m, self.n),
        }
    }

    /// Number of registers of a fragment of `mma.sync`: each thread of the
    /// warp holds 1/32 of the matrix, packed in 32-bit registers (64-bit for
    /// `f64`).
    pub fn mma_registers(&self, matrix: MmaMatrix) -> u8 {
        let (rows, cols) = self.matrix_size(matrix);
        let type_ = self.element_type(matrix);
        let register_bits = if type_ == MmaType::F64 { 64 } else { 32 };
        // half of the elements of a sparse A
        let sparsity = if self.sparse.is_some() && matrix == MmaMatrix::A { 2 } else { 1 };
        (rows * cols / 32 / sparsity * type_.bits()).div_ceil(register_bits) as u8
    }

    /// Number of registers of a fragment of `wmma`, as given by the PTX ISA:
    /// like `mma.sync`, except that `f16` A and B fragments have 8 registers
    /// (every element is held by two threads).
    pub fn wmma_registers(&self, matrix: MmaMatrix) -> u8 {
        let type_ = self.element_type(matrix);
        if type_ == MmaType::F16 && matches!(matrix, MmaMatrix::A | MmaMatrix::B) {
            8
        } else {
            self.mma_registers(matrix)
        }
    }

    // The variants of the original AMD GPU implementation keep its operand
    // types: m16n8k16 f32.{f16,bf16} and m16n8k32 s32.s8.s8.
    fn is_legacy(&self) -> bool {
        matches!(
            (self.m, self.n, self.k, self.dtype, self.atype, self.btype, self.ctype),
            (16, 8, 16, MmaType::F32, MmaType::F16, MmaType::F16, MmaType::F32)
                | (16, 8, 16, MmaType::F32, MmaType::Bf16, MmaType::Bf16, MmaType::F32)
                | (16, 8, 32, MmaType::S32, MmaType::S8, MmaType::S8, MmaType::S32)
        ) && !self.satfinite
            && self.sparse.is_none()
    }

    pub fn mma_type(&self, matrix: MmaMatrix) -> Type {
        let registers = self.mma_registers(matrix);
        if self.is_legacy() {
            let scalar = match matrix {
                MmaMatrix::A | MmaMatrix::B => ScalarType::U32,
                MmaMatrix::C | MmaMatrix::D if self.dtype == MmaType::F32 => ScalarType::F32,
                MmaMatrix::C | MmaMatrix::D => ScalarType::U32,
            };
            return Type::Vector(registers, scalar);
        }
        Type::Vector(registers, fragment_scalar(self.element_type(matrix)))
    }

    pub fn wmma_type(&self, matrix: MmaMatrix) -> Type {
        Type::Vector(
            self.wmma_registers(matrix),
            fragment_scalar(self.element_type(matrix)),
        )
    }
}

fn fragment_scalar(type_: MmaType) -> ScalarType {
    if type_ == MmaType::F64 {
        ScalarType::B64
    } else {
        ScalarType::B32
    }
}

/// `wgmma.mma_async{.sp}`: the warpgroup (4 warps) computes a 64 x n x k product. `mma` holds the shape and
/// types (m = 64, ctype = dtype, sparse = the selector of .sp); A is in registers (the m16n8kK fragment of each
/// warp) or in shared memory (a matrix descriptor), B in shared memory.
#[derive(Copy, Clone)]
pub struct WgmmaDetails {
    pub mma: MmaDetails,
    pub a_descriptor: bool,
    /// imm-scale-a / imm-scale-b of -1: the elements are negated
    pub negate_a: bool,
    pub negate_b: bool,
    /// imm-trans-a / imm-trans-b (16-bit types): the descriptor's tile is MN-major instead of K-major
    pub transpose_a: bool,
    pub transpose_b: bool,
    /// scale-d as an immediate (false: D = A * B); None when it is a predicate operand
    pub scale_d: Option<bool>,
}

impl WgmmaDetails {
    /// D: n / 2 elements per thread, 32-bit registers
    pub fn d_type(&self) -> Type {
        let elements = self.mma.n / 2;
        let per = if self.mma.dtype.bits() == 16 { 2 } else { 1 };
        Type::Vector((elements / per) as u8, ScalarType::B32)
    }

    pub fn a_type(&self) -> Type {
        if self.a_descriptor {
            Type::Scalar(ScalarType::B64)
        } else {
            // the m16n8kK A fragment (sparse: of the compressed matrix)
            let k = if self.mma.sparse.is_some() { self.mma.k / 2 } else { self.mma.k };
            Type::Vector((16 * k * self.mma.atype.bits() / 32 / 32) as u8, ScalarType::B32)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WgmmaControl {
    Fence,
    CommitGroup,
    WaitGroup(u32),
}

/// `wmma.load` and `wmma.store`: one fragment of a `wmma.mma` of shape
/// m x n x k, in row or column major order in memory.
#[derive(Copy, Clone)]
pub struct WmmaLoadStoreDetails {
    pub matrix: MmaMatrix,
    pub layout: MatrixLayout,
    pub m: u32,
    pub n: u32,
    pub k: u32,
    pub state_space: StateSpace,
    pub type_: MmaType,
}

impl WmmaLoadStoreDetails {
    /// The `wmma.mma` whose fragment this is (only the shape and the element
    /// type of the matrix matter).
    pub fn mma(&self) -> MmaDetails {
        MmaDetails {
            m: self.m,
            n: self.n,
            k: self.k,
            alayout: self.layout,
            blayout: self.layout,
            dtype: self.type_,
            atype: self.type_,
            btype: self.type_,
            ctype: self.type_,
            satfinite: false,
            b1_op: None,
            block_scale: None,
            sparse: None,
        }
    }

    pub fn fragment_type(&self) -> Type {
        self.mma().wmma_type(self.matrix)
    }

    /// Whether the fragment belongs to a supported `wmma.mma`.
    pub fn is_supported(&self) -> bool {
        use MmaType::*;
        let shape = (self.m, self.n, self.k);
        let k16 = matches!(shape, (16, 16, 16) | (32, 8, 16) | (8, 32, 16));
        match (self.matrix, self.type_) {
            (MmaMatrix::A | MmaMatrix::B, F16 | Bf16 | S8 | U8) => k16,
            (MmaMatrix::A | MmaMatrix::B, Tf32) => shape == (16, 16, 8),
            (MmaMatrix::A | MmaMatrix::B, S4 | U4) => shape == (8, 8, 32),
            (MmaMatrix::A | MmaMatrix::B, B1) => shape == (8, 8, 128),
            (_, F64) => shape == (8, 8, 4),
            (MmaMatrix::C | MmaMatrix::D, F16) => k16,
            (MmaMatrix::C | MmaMatrix::D, F32) => k16 || shape == (16, 16, 8),
            (MmaMatrix::C | MmaMatrix::D, S32) => {
                k16 || matches!(shape, (8, 8, 32) | (8, 8, 128))
            }
            _ => false,
        }
    }
}

pub struct PrefetchData {
    pub space: StateSpace,
    pub level: CacheLevel,
}

#[derive(Clone, Copy)]
pub struct Dp2aData {
    pub atype: ScalarType,
    pub btype: ScalarType,
    pub control: Dp2aControl,
}

impl Dp2aData {
    pub fn ctype(&self) -> ScalarType {
        match (self.atype, self.btype) {
            (ScalarType::U32, ScalarType::U32) => ScalarType::U32,
            _ => ScalarType::S32,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Dp2aControl {
    Low,
    High,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum TexDimensions {
    D1,
    D2,
    D3,
}

pub struct TexData {
    pub type_: TexType,
    pub dtype: ScalarType,
    pub ctype: ScalarType,
    pub dims: TexDimensions,
}

impl TexData {
    pub fn coord_type(&self) -> Type {
        let dims = match self.dims {
            TexDimensions::D1 => 1,
            TexDimensions::D2 => 2,
            TexDimensions::D3 => 4,
        };
        Type::Vector(dims, self.ctype)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum TexType {
    Texref,
    Texobj,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MiniFormat {
    E4m3x2,
    E5m2x2,
    E2m1x2,
    E2m3x2,
    E3m2x2,
    Ue8m0x2,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MiniDirection {
    // two f32 (the first one into the high element)
    FromF32,
    FromF16x2,
    FromBf16x2,
    ToF16x2,
    ToBf16x2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CvtMiniDetails {
    pub format: MiniFormat,
    pub direction: MiniDirection,
    pub rounding: RoundingMode,
    pub relu: bool,
}

impl CvtMiniDetails {
    pub fn dst_type(&self) -> Type {
        match self.direction {
            MiniDirection::ToF16x2 | MiniDirection::ToBf16x2 => Type::Scalar(ScalarType::B32),
            _ => Type::Scalar(ScalarType::B16),
        }
    }

    pub fn src_type(&self) -> Type {
        match self.direction {
            MiniDirection::FromF32 => Type::Scalar(ScalarType::F32),
            MiniDirection::FromF16x2 | MiniDirection::FromBf16x2 => Type::Scalar(ScalarType::B32),
            MiniDirection::ToF16x2 | MiniDirection::ToBf16x2 => Type::Scalar(ScalarType::B16),
        }
    }
}

// cvt.rnd{.relu}{.satfinite}.tf32.f32: rounds an f32 to the 10 fraction bits
// of tf32, the result keeps the binary32 layout with the low 13 bits clear
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CvtTf32Details {
    pub rounding: Tf32Rounding,
    pub relu: bool,
    pub satfinite: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VshData {
    pub mode: FunnelShiftMode,
    pub op2: VshOp,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum VideoOp {
    Add,
    Sub,
    Absdiff,
    Min,
    Max,
    Shl,
    Shr,
    Avrg,
    Set(VideoCompare),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum VideoCompare {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum VideoSecondaryOp {
    Add,
    Min,
    Max,
}

// A byte (.b) or half-word (.h) selector of a video instruction operand:
// one digit for scalar instructions, one per lane (most significant lane
// first, like in the source) for SIMD instructions and their masks
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct VideoSelector {
    pub half: bool,
    pub digits: [u8; 4],
    pub len: u8,
}

impl VideoSelector {
    pub fn width(&self) -> u32 {
        if self.half {
            16
        } else {
            8
        }
    }

    pub fn digits(&self) -> &[u8] {
        &self.digits[..self.len as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VideoScalarDetails {
    pub op: VideoOp,
    // .u32 for vset, which has no destination type
    pub dtype: ScalarType,
    pub atype: ScalarType,
    pub btype: ScalarType,
    pub saturate: bool,
    // shift amount mode of vshl and vshr: .clamp (default) or .wrap
    pub clamp: bool,
    pub asel: Option<VideoSelector>,
    pub bsel: Option<VideoSelector>,
    // merge of the result into src3
    pub dsel: Option<VideoSelector>,
    pub secondary: Option<VideoSecondaryOp>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VideoSimdDetails {
    pub op: VideoOp,
    // 2 or 4
    pub lanes: u8,
    pub dtype: ScalarType,
    pub atype: ScalarType,
    pub btype: ScalarType,
    pub saturate: bool,
    // .add: the sum of the lanes plus src3, otherwise the lanes merged
    // with src3
    pub add: bool,
    pub mask: Option<VideoSelector>,
    pub asel: Option<VideoSelector>,
    pub bsel: Option<VideoSelector>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VmadDetails {
    pub dtype: ScalarType,
    pub atype: ScalarType,
    pub btype: ScalarType,
    pub saturate: bool,
    // .shr7 or .shr15
    pub shift: u8,
    pub plus_one: bool,
    pub negate_a: bool,
    pub negate_b: bool,
    pub negate_c: bool,
    pub asel: Option<VideoSelector>,
    pub bsel: Option<VideoSelector>,
}

// Operations on four byte lanes of a .b32 (.u8x4 and .s8x4 types)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Packed8Details {
    pub op: Packed8Op,
    pub signed: bool,
    pub saturate: bool,
    pub relu: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Packed8Op {
    Add,
    Sub,
    Min,
    Max,
    Neg,
    Abs,
    /// set.CmpOp on packed lanes (PTX ISA 9.4): all ones in each lane that compares true. `lanes16` selects
    /// u16x2 / s16x2 instead of the byte lanes.
    Set { cmp: PackedCmp, lanes16: bool },
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackedCmp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AllocaDetails {
    pub type_: ScalarType,
    pub align: u32,
}

/// `szext.mode.type`: the low N bits of a, sign- (.s32) or zero-extended (.u32), N = b mod 32 (.wrap) or
/// min(b, 32) (.clamp).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SzextDetails {
    pub clamp: bool,
    pub type_: ScalarType,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BfindDetails {
    pub type_: ScalarType,
    pub shift_amount: bool,
}

