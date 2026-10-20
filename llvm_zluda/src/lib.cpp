#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm-c/Core.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/Type.h>
#include <llvm/IR/Instructions.h>
#include <llvm/IR/AutoUpgrade.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/Operator.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/ReplaceConstant.h>
#include <llvm/ADT/SetVector.h>
#include <llvm/Support/Alignment.h>
#include <optional>
#include <string>
#include <llvm/IR/CallingConv.h>
#ifdef ZLUDA_WITH_LLD
#include <lld/Common/Driver.h>
#include <lld/Common/CommonLinkerContext.h>
#endif
#include <llvm/Support/raw_ostream.h>
#include "llvm/Support/CommandLine.h"
#include "llvm/Support/raw_ostream.h"
#pragma GCC diagnostic pop

#include <mutex>

#ifdef ZLUDA_WITH_LLD
// Declare that we want to use the ELF driver
LLD_HAS_DRIVER(elf)
#endif

using namespace llvm;

typedef enum
{
    LLVMZludaAtomicRMWBinOpXchg,     /**< Set the new value and return the one old */
    LLVMZludaAtomicRMWBinOpAdd,      /**< Add a value and return the old one */
    LLVMZludaAtomicRMWBinOpSub,      /**< Subtract a value and return the old one */
    LLVMZludaAtomicRMWBinOpAnd,      /**< And a value and return the old one */
    LLVMZludaAtomicRMWBinOpNand,     /**< Not-And a value and return the old one */
    LLVMZludaAtomicRMWBinOpOr,       /**< OR a value and return the old one */
    LLVMZludaAtomicRMWBinOpXor,      /**< Xor a value and return the old one */
    LLVMZludaAtomicRMWBinOpMax,      /**< Sets the value if it's greater than the
                                  original using a signed comparison and return
                                  the old one */
    LLVMZludaAtomicRMWBinOpMin,      /**< Sets the value if it's Smaller than the
                                  original using a signed comparison and return
                                  the old one */
    LLVMZludaAtomicRMWBinOpUMax,     /**< Sets the value if it's greater than the
                                 original using an unsigned comparison and return
                                 the old one */
    LLVMZludaAtomicRMWBinOpUMin,     /**< Sets the value if it's greater than the
                                  original using an unsigned comparison and return
                                  the old one */
    LLVMZludaAtomicRMWBinOpFAdd,     /**< Add a floating point value and return the
                                  old one */
    LLVMZludaAtomicRMWBinOpFSub,     /**< Subtract a floating point value and return the
                                old one */
    LLVMZludaAtomicRMWBinOpFMax,     /**< Sets the value if it's greater than the
                                 original using an floating point comparison and
                                 return the old one */
    LLVMZludaAtomicRMWBinOpFMin,     /**< Sets the value if it's smaller than the
                                 original using an floating point comparison and
                                 return the old one */
    LLVMZludaAtomicRMWBinOpUIncWrap, /**< Increments the value, wrapping back to zero
                                 when incremented above input value */
    LLVMZludaAtomicRMWBinOpUDecWrap, /**< Decrements the value, wrapping back to
                                 the input value when decremented below zero */
} LLVMZludaAtomicRMWBinOp;

static llvm::AtomicRMWInst::BinOp mapFromLLVMRMWBinOp(LLVMZludaAtomicRMWBinOp BinOp)
{
    switch (BinOp)
    {
    case LLVMZludaAtomicRMWBinOpXchg:
        return llvm::AtomicRMWInst::Xchg;
    case LLVMZludaAtomicRMWBinOpAdd:
        return llvm::AtomicRMWInst::Add;
    case LLVMZludaAtomicRMWBinOpSub:
        return llvm::AtomicRMWInst::Sub;
    case LLVMZludaAtomicRMWBinOpAnd:
        return llvm::AtomicRMWInst::And;
    case LLVMZludaAtomicRMWBinOpNand:
        return llvm::AtomicRMWInst::Nand;
    case LLVMZludaAtomicRMWBinOpOr:
        return llvm::AtomicRMWInst::Or;
    case LLVMZludaAtomicRMWBinOpXor:
        return llvm::AtomicRMWInst::Xor;
    case LLVMZludaAtomicRMWBinOpMax:
        return llvm::AtomicRMWInst::Max;
    case LLVMZludaAtomicRMWBinOpMin:
        return llvm::AtomicRMWInst::Min;
    case LLVMZludaAtomicRMWBinOpUMax:
        return llvm::AtomicRMWInst::UMax;
    case LLVMZludaAtomicRMWBinOpUMin:
        return llvm::AtomicRMWInst::UMin;
    case LLVMZludaAtomicRMWBinOpFAdd:
        return llvm::AtomicRMWInst::FAdd;
    case LLVMZludaAtomicRMWBinOpFSub:
        return llvm::AtomicRMWInst::FSub;
    case LLVMZludaAtomicRMWBinOpFMax:
        return llvm::AtomicRMWInst::FMax;
    case LLVMZludaAtomicRMWBinOpFMin:
        return llvm::AtomicRMWInst::FMin;
    case LLVMZludaAtomicRMWBinOpUIncWrap:
        return llvm::AtomicRMWInst::UIncWrap;
    case LLVMZludaAtomicRMWBinOpUDecWrap:
        return llvm::AtomicRMWInst::UDecWrap;
    }

    llvm_unreachable("Invalid LLVMZludaAtomicRMWBinOp value!");
}

static AtomicOrdering mapFromLLVMOrdering(LLVMAtomicOrdering Ordering)
{
    switch (Ordering)
    {
    case LLVMAtomicOrderingNotAtomic:
        return AtomicOrdering::NotAtomic;
    case LLVMAtomicOrderingUnordered:
        return AtomicOrdering::Unordered;
    case LLVMAtomicOrderingMonotonic:
        return AtomicOrdering::Monotonic;
    case LLVMAtomicOrderingAcquire:
        return AtomicOrdering::Acquire;
    case LLVMAtomicOrderingRelease:
        return AtomicOrdering::Release;
    case LLVMAtomicOrderingAcquireRelease:
        return AtomicOrdering::AcquireRelease;
    case LLVMAtomicOrderingSequentiallyConsistent:
        return AtomicOrdering::SequentiallyConsistent;
    }

    llvm_unreachable("Invalid LLVMAtomicOrdering value!");
}

typedef unsigned LLVMFastMathFlags;

static FastMathFlags mapFromLLVMFastMathFlags(LLVMFastMathFlags FMF)
{
    FastMathFlags NewFMF;
    NewFMF.setAllowReassoc((FMF & LLVMFastMathAllowReassoc) != 0);
    NewFMF.setNoNaNs((FMF & LLVMFastMathNoNaNs) != 0);
    NewFMF.setNoInfs((FMF & LLVMFastMathNoInfs) != 0);
    NewFMF.setNoSignedZeros((FMF & LLVMFastMathNoSignedZeros) != 0);
    NewFMF.setAllowReciprocal((FMF & LLVMFastMathAllowReciprocal) != 0);
    NewFMF.setAllowContract((FMF & LLVMFastMathAllowContract) != 0);
    NewFMF.setApproxFunc((FMF & LLVMFastMathApproxFunc) != 0);

    return NewFMF;
}

LLVM_C_EXTERN_C_BEGIN

LLVMValueRef LLVMZludaBuildAlloca(LLVMBuilderRef B, LLVMTypeRef Ty, unsigned AddrSpace,
                                  const char *Name)
{
    return llvm::wrap(llvm::unwrap(B)->CreateAlloca(llvm::unwrap(Ty), AddrSpace, nullptr, Name));
}

LLVMValueRef LLVMZludaBuildArrayAlloca(LLVMBuilderRef B, LLVMTypeRef Ty, unsigned AddrSpace,
                                       LLVMValueRef ArraySize, unsigned Align, const char *Name)
{
    llvm::AllocaInst *Alloca = llvm::unwrap(B)->CreateAlloca(llvm::unwrap(Ty), AddrSpace,
                                                             llvm::unwrap(ArraySize), Name);
    Alloca->setAlignment(llvm::Align(Align));
    return llvm::wrap(Alloca);
}

LLVMValueRef LLVMZludaBuildAtomicRMW(LLVMBuilderRef B, LLVMZludaAtomicRMWBinOp op,
                                     LLVMValueRef PTR, LLVMValueRef Val,
                                     char *scope,
                                     LLVMAtomicOrdering ordering)
{
    auto builder = llvm::unwrap(B);
    LLVMContext &context = builder->getContext();
    llvm::AtomicRMWInst::BinOp intop = mapFromLLVMRMWBinOp(op);
    return llvm::wrap(builder->CreateAtomicRMW(
        intop, llvm::unwrap(PTR), llvm::unwrap(Val), llvm::MaybeAlign(),
        mapFromLLVMOrdering(ordering),
        context.getOrInsertSyncScopeID(scope)));
}

LLVMValueRef LLVMZludaBuildAtomicCmpXchg(LLVMBuilderRef B, LLVMValueRef Ptr,
                                         LLVMValueRef Cmp, LLVMValueRef New,
                                         char *scope,
                                         LLVMAtomicOrdering SuccessOrdering,
                                         LLVMAtomicOrdering FailureOrdering)
{
    auto builder = llvm::unwrap(B);
    LLVMContext &context = builder->getContext();
    return wrap(builder->CreateAtomicCmpXchg(
        unwrap(Ptr), unwrap(Cmp), unwrap(New), MaybeAlign(),
        mapFromLLVMOrdering(SuccessOrdering),
        mapFromLLVMOrdering(FailureOrdering),
        context.getOrInsertSyncScopeID(scope)));
}

void LLVMZludaSetFastMathFlags(LLVMValueRef FPMathInst, LLVMFastMathFlags FMF)
{
    Value *P = unwrap<Value>(FPMathInst);
    cast<Instruction>(P)->setFastMathFlags(mapFromLLVMFastMathFlags(FMF));
}

void LLVMZludaBuildFence(LLVMBuilderRef B, LLVMAtomicOrdering Ordering,
                         char *scope, const char *Name)
{
    auto builder = llvm::unwrap(B);
    LLVMContext &context = builder->getContext();
    builder->CreateFence(mapFromLLVMOrdering(Ordering),
                         context.getOrInsertSyncScopeID(scope),
                         Name);
}

void LLVMZludaSetAtomic(
    LLVMValueRef AtomicInst,
    LLVMAtomicOrdering Ordering,
    char *SSID)
{
    auto inst = unwrap(AtomicInst);
    if (LoadInst *LI = dyn_cast<LoadInst>(inst))
    {
        LI->setAtomic(mapFromLLVMOrdering(Ordering), LI->getContext().getOrInsertSyncScopeID(SSID));
    }
    else if (StoreInst *SI = dyn_cast<StoreInst>(inst))
    {
        SI->setAtomic(mapFromLLVMOrdering(Ordering), SI->getContext().getOrInsertSyncScopeID(SSID));
    }
    else
    {
        llvm_unreachable("Invalid instruction type for LLVMZludaSetAtomic");
    }
}

#ifdef ZLUDA_WITH_LLD
std::mutex lld_mutex;

int LLVMZludaLinkWithLLD(uint32_t inputs, const char **input_paths, const char *output_path, char **ErrorMessage)
{
    std::vector<const char *> args;
    args.push_back("ld.lld");
    args.push_back("-shared");
    for (uint32_t i = 0; i < inputs; ++i)
    {
        args.push_back(input_paths[i]);
    }
    args.push_back("-o");
    args.push_back(output_path);
    args.push_back("--threads=1");
    args.push_back("--no-undefined");

    std::string log_out_str;
    std::string log_err_str;
    llvm::raw_string_ostream log_out(log_out_str);
    llvm::raw_string_ostream log_err(log_err_str);

    // lldMain does not seem to be thread-safe, so we wrap it in a mutex.
    std::lock_guard<std::mutex> guard(lld_mutex);
    lld::Result result = lld::lldMain(args, log_out, log_err,
                                      {{lld::Gnu, &lld::elf::link}});

    if (result.retCode != 0)
    {
        if (ErrorMessage)
        {
            if (!log_err_str.empty())
            {
                *ErrorMessage = strdup(log_err_str.c_str());
            }
            else
            {
                *ErrorMessage = strdup("LLD linking failed with unknown error");
            }
        }
        return result.retCode;
    }

    return 0;
}
#endif

bool LLVMZludaParseCommandLineOptions(int argc, const char *const *argv,
                                      char **ErrorMessage)
{
    std::string log_out_str;
    llvm::raw_string_ostream log_err_str(log_out_str);
    bool result = llvm::cl::ParseCommandLineOptions(argc, argv, StringRef(""), &log_err_str);
    if (!result && ErrorMessage)
    {
        *ErrorMessage = strdup(log_out_str.c_str());
    }
    return result;
}

// Function attributes are emitted in their textual form, which newer LLVM
// versions only understand after upgrading them (for example
// "denormal-fp-math" to denormal_fpenv).
void LLVMZludaUpgradeFunctionAttributes(LLVMModuleRef M)
{
    for (llvm::Function &F : *llvm::unwrap(M))
        llvm::UpgradeFunctionAttributes(F);
}

// The address space a generic address comes from: looks through integer
// casts, constant offsets, address space casts and pointer arithmetic.
// Addresses of unknown origin are device (global) addresses.
static unsigned zludaAgxSourceAddressSpace(Value *V)
{
    for (unsigned depth = 0; depth < 64; depth++)
    {
        if (V->getType()->isPointerTy() && V->getType()->getPointerAddressSpace() != 0)
            return V->getType()->getPointerAddressSpace();
        if (auto *Op = dyn_cast<Operator>(V))
        {
            switch (Op->getOpcode())
            {
            case Instruction::PtrToInt:
            case Instruction::IntToPtr:
            case Instruction::AddrSpaceCast:
            case Instruction::BitCast:
            case Instruction::ZExt:
            case Instruction::SExt:
            case Instruction::Trunc:
            case Instruction::GetElementPtr:
                V = Op->getOperand(0);
                continue;
            case Instruction::Add:
            case Instruction::Sub:
            case Instruction::Or:
            case Instruction::Xor:
                if (isa<Constant>(Op->getOperand(1)))
                {
                    V = Op->getOperand(0);
                    continue;
                }
                if (Op->getOpcode() == Instruction::Add && isa<Constant>(Op->getOperand(0)))
                {
                    V = Op->getOperand(1);
                    continue;
                }
                return 0;
            default:
                return 0;
            }
        }
        return 0;
    }
    return 0;
}

// A generic address of threadgroup or private memory converted to an integer
static Value *zludaAgxLocalBase(Value *V)
{
    auto *P2I = dyn_cast<PtrToIntOperator>(V);
    if (!P2I)
        return nullptr;
    auto *Cast = dyn_cast<AddrSpaceCastOperator>(P2I->getPointerOperand());
    if (!Cast || Cast->getDestAddressSpace() != 0 ||
        (Cast->getSrcAddressSpace() != 3 && Cast->getSrcAddressSpace() != 5))
        return nullptr;
    return Cast->getPointerOperand();
}

// Splits an integer into such an address plus offsets
static Value *zludaAgxSplitLocalAddress(Value *V, SmallVectorImpl<Value *> &Offsets, unsigned Depth = 0)
{
    if (Value *Base = zludaAgxLocalBase(V))
        return Base;
    auto *Add = dyn_cast<BinaryOperator>(V);
    if (!Add || Add->getOpcode() != Instruction::Add || Depth > 8)
        return nullptr;
    for (unsigned i = 0; i < 2; i++)
        if (Value *Base = zludaAgxSplitLocalAddress(Add->getOperand(i), Offsets, Depth + 1))
        {
            Offsets.push_back(Add->getOperand(1 - i));
            return Base;
        }
    return nullptr;
}

// Whether an address converted to an integer is never used as an address
// again: it only flows into comparisons and into operations whose results
// are no addresses (masks of low bits, shifts, ...)
static bool zludaAgxStaysInteger(Value *V, SmallPtrSetImpl<Value *> &Visited)
{
    if (!Visited.insert(V).second)
        return true;
    for (User *U : V->users())
    {
        auto *I = dyn_cast<Instruction>(U);
        if (!I)
            return false;
        bool follow;
        switch (I->getOpcode())
        {
        case Instruction::ICmp:
        case Instruction::LShr:
        case Instruction::AShr:
        case Instruction::Shl:
        case Instruction::Mul:
        case Instruction::UDiv:
        case Instruction::SDiv:
        case Instruction::URem:
        case Instruction::SRem:
            follow = false;
            break;
        case Instruction::And:
        {
            auto *Mask = dyn_cast<ConstantInt>(I->getOperand(1));
            follow = !(Mask && Mask->getValue().ult(0x10000));
            break;
        }
        case Instruction::Trunc:
            follow = I->getType()->getIntegerBitWidth() >= 32;
            break;
        case Instruction::Add:
        case Instruction::Sub:
        case Instruction::Or:
        case Instruction::Xor:
        case Instruction::ZExt:
        case Instruction::SExt:
        case Instruction::Select:
        case Instruction::PHI:
        case Instruction::Freeze:
            follow = true;
            break;
        default:
            return false;
        }
        if (follow && !zludaAgxStaysInteger(I, Visited))
            return false;
    }
    return true;
}

// AGX has no generic addressing of threadgroup or private memory. Generic
// addresses of it that went through integer arithmetic become pointer
// arithmetic on the original pointer (which address space inference then
// follows), and those only used as integers (compared, masked) become
// their 32-bit offsets.
static bool zludaAgxRewriteLocalAddress(Instruction *I)
{
    if (auto *I2P = dyn_cast<IntToPtrInst>(I))
    {
        SmallVector<Value *, 4> Offsets;
        Value *Base = zludaAgxSplitLocalAddress(I2P->getOperand(0), Offsets);
        if (!Base)
            return false;
        IRBuilder<> Builder(I2P);
        Value *Ptr = Base;
        for (Value *Offset : Offsets)
            Ptr = Builder.CreateGEP(Builder.getInt8Ty(), Ptr, Builder.CreateTrunc(Offset, Builder.getInt32Ty()));
        I2P->replaceAllUsesWith(Builder.CreateAddrSpaceCast(Ptr, I2P->getType()));
        I2P->eraseFromParent();
        return true;
    }
    auto *P2I = dyn_cast<PtrToIntInst>(I);
    if (!P2I)
        return false;
    Value *Base = zludaAgxLocalBase(P2I);
    if (!Base)
        return false;
    SmallPtrSet<Value *, 16> Visited;
    if (!zludaAgxStaysInteger(P2I, Visited))
        return false;
    IRBuilder<> Builder(P2I);
    Value *Offset = Builder.CreatePtrToInt(Base, Builder.getInt32Ty());
    P2I->replaceAllUsesWith(Builder.CreateZExtOrTrunc(Offset, P2I->getType()));
    P2I->eraseFromParent();
    return true;
}

// isspacep after optimization: calls to i1 @__zluda_agx_isspacep_<space>(i64)
// become constants when the address space of the argument is known, and
// otherwise test the tag of the generic address. The LLVM AGX backend
// (AGXLowerGenericAddresses) tags generic pointers to threadgroup memory with
// bit 63 and those to private memory with bit 62; device addresses have
// neither (top two bits 0, 2 or 3, 1). Also rewrites the generic addresses of
// those memories that went through integer arithmetic (see
// zludaAgxRewriteLocalAddress), which keeps them out of the tagged form.
// Returns whether anything changed.
bool LLVMZludaAgxLowerAddressSpaces(LLVMModuleRef M)
{
    struct Space
    {
        const char *name;
        bool (*test)(unsigned);
        // the top two bits a generic address of this space has: mask of {0, 1, 2, 3}
        unsigned tags;
    };
    static const Space spaces[] = {
        {"__zluda_agx_isspacep_global", [](unsigned as) { return as == 1; }, 0b0001},
        {"__zluda_agx_isspacep_shared", [](unsigned as) { return as == 3; }, 0b1100},
        {"__zluda_agx_isspacep_local", [](unsigned as) { return as == 5; }, 0b0010},
        {"__zluda_agx_isspacep_const", [](unsigned as) { return as == 4; }, 0},
        {"__zluda_agx_isspacep_param", [](unsigned) { return false; }, 0},
    };
    Module *Mod = unwrap(M);
    bool found = false;
    for (Function &F : *Mod)
        for (BasicBlock &BB : F)
            for (Instruction &I : make_early_inc_range(BB))
                if (zludaAgxRewriteLocalAddress(&I))
                    found = true;
    for (const Space &space : spaces)
    {
        Function *F = Mod->getFunction(space.name);
        if (!F)
            continue;
        for (User *U : make_early_inc_range(F->users()))
        {
            auto *Call = dyn_cast<CallInst>(U);
            if (!Call)
                continue;
            unsigned as = zludaAgxSourceAddressSpace(Call->getArgOperand(0));
            Value *Result;
            if (as != 0 || space.tags == 0)
                Result = ConstantInt::getBool(Call->getContext(), space.test(as));
            else
            {
                IRBuilder<> Builder(Call);
                Value *Tag = Builder.CreateLShr(Call->getArgOperand(0), 62);
                if (space.tags == 0b1100)
                    Result = Builder.CreateICmpUGE(Tag, Builder.getInt64(2));
                else
                    Result = Builder.CreateICmpEQ(Tag, Builder.getInt64(space.tags == 0b0001 ? 0 : 1));
            }
            Call->replaceAllUsesWith(Result);
            Call->eraseFromParent();
            found = true;
        }
        if (F->use_empty())
            F->eraseFromParent();
    }
    return found;
}

LLVM_C_EXTERN_C_END

// The optimizer folds operations on constants to the default NaN of LLVM
// (0x7fc00000 for f32). In the strict floating point mode PTX's canonical NaN
// is wanted (0x7fffffff, 0x7fff for f16 and bf16, 0x7fffffffffffffff for f64),
// which the backend produces for the operations it emits; this rewrites the
// folded ones. A NaN literal with the default payload that is only moved also
// becomes the canonical NaN.
static Constant *zludaCanonicalNaN(Constant *C)
{
    if (auto *FP = dyn_cast<ConstantFP>(C))
    {
        const APFloat &V = FP->getValueAPF();
        if (!V.isNaN())
            return nullptr;
        const fltSemantics &Sem = V.getSemantics();
        if (V.bitcastToAPInt() != APFloat::getQNaN(Sem).bitcastToAPInt())
            return nullptr;
        unsigned Bits = APFloat::getSizeInBits(Sem);
        return ConstantFP::get(C->getType(), APFloat(Sem, APInt::getSignedMaxValue(Bits)));
    }
    if (isa<ConstantDataVector>(C) || isa<ConstantVector>(C))
    {
        auto *VT = cast<FixedVectorType>(C->getType());
        if (!VT->getElementType()->isFloatingPointTy())
            return nullptr;
        SmallVector<Constant *, 8> Elems;
        bool Changed = false;
        for (unsigned i = 0; i < VT->getNumElements(); i++)
        {
            Constant *E = C->getAggregateElement(i);
            Constant *N = E ? zludaCanonicalNaN(E) : nullptr;
            Changed |= N != nullptr;
            Elems.push_back(N ? N : E);
        }
        return Changed ? ConstantVector::get(Elems) : nullptr;
    }
    return nullptr;
}

LLVM_C_EXTERN_C_BEGIN

void LLVMZludaAgxCanonicalizeFoldedNaNs(LLVMModuleRef M)
{
    for (Function &F : *unwrap(M))
        for (BasicBlock &BB : F)
            for (Instruction &I : BB)
                for (unsigned i = 0; i < I.getNumOperands(); i++)
                    if (auto *C = dyn_cast<Constant>(I.getOperand(i)))
                        if (Constant *N = zludaCanonicalNaN(C))
                            I.setOperand(i, N);
}

LLVM_C_EXTERN_C_END

// Clusters of several CTAs on AGX. A kernel with agx-cluster-dims (x, y, z)
// runs a cluster of C = x * y * z CTAs as one threadgroup of C times the CTA's
// threads: the runtime launches the grid divided by the cluster dimensions,
// with the CTA's z multiplied by C. Thread (x, y, z) of CTA r is thread
// (x, y, z * C + r) of the threadgroup: no division by a runtime value, and
// with a CTA of depth 1 (the usual case) each CTA's threads are consecutive
// (whole SIMD-groups when it has a multiple of 32 threads). Here, in each
// function, after inlining:
// * the thread and threadgroup ids become the CTA's (%tid.z, %ntid.z, %ctaid,
//   %nctaid and the linear index in the threadgroup);
// * the shared variables the kernel uses are packed into one array of C
//   windows, CTA r using window r; mapa and getctarank are arithmetic on the
//   window stride;
// * the placeholders of zluda_ptx_impl_agx.ll (__zluda_agx_cluster*) get the
//   cluster registers.
// Kernels without clusters get C = 1: the cluster registers of a single CTA.
namespace
{
    struct ClusterShape
    {
        unsigned dims[3] = {1, 1, 1};
        bool explicit_ = false;
        unsigned count() const { return dims[0] * dims[1] * dims[2]; }
        bool operator==(const ClusterShape &o) const
        {
            return dims[0] == o.dims[0] && dims[1] == o.dims[1] && dims[2] == o.dims[2] &&
                   explicit_ == o.explicit_;
        }
    };

    ClusterShape zludaClusterShape(const Function &F)
    {
        ClusterShape shape;
        Attribute A = F.getFnAttribute("agx-cluster-dims");
        if (!A.isValid())
            return shape;
        SmallVector<StringRef, 3> parts;
        A.getValueAsString().split(parts, ',');
        for (unsigned i = 0; i < parts.size() && i < 3; i++)
        {
            unsigned v = 1;
            if (!parts[i].trim().getAsInteger(10, v) && v > 0)
                shape.dims[i] = v;
        }
        shape.explicit_ = true;
        return shape;
    }

    bool zludaIsCalled(const CallInst *CI, StringRef name)
    {
        const Function *callee = CI->getCalledFunction();
        return callee && callee->getName() == name;
    }

    // with ids: also the thread and threadgroup ids a cluster changes
    bool zludaUsesClusterState(const Function &F, bool ids)
    {
        for (const BasicBlock &BB : F)
            for (const Instruction &I : BB)
            {
                if (auto *CI = dyn_cast<CallInst>(&I))
                    if (const Function *callee = CI->getCalledFunction())
                    {
                        StringRef n = callee->getName();
                        if (n.starts_with("__zluda_agx_cluster"))
                            return true;
                        if (ids && (n.starts_with("llvm.agx.thread.position.in.threadgroup") ||
                            n.starts_with("llvm.agx.threads.per.threadgroup") ||
                            n.starts_with("llvm.agx.threadgroup.position.in.grid") ||
                            n.starts_with("llvm.agx.threadgroups.per.grid") ||
                            n == "llvm.agx.thread.index.in.threadgroup"))
                            return true;
                    }
            }
        return false;
    }

    // The addrspace(3) variables an instruction refers to, through constant
    // expressions
    void zludaSharedVariables(Value *V, SmallSetVector<GlobalVariable *, 8> &out,
                              SmallPtrSetImpl<Value *> &seen)
    {
        if (!seen.insert(V).second)
            return;
        if (auto *GV = dyn_cast<GlobalVariable>(V))
        {
            if (GV->getAddressSpace() == 3)
                out.insert(GV);
            return;
        }
        if (auto *CE = dyn_cast<ConstantExpr>(V))
            for (Value *op : CE->operands())
                zludaSharedVariables(op, out, seen);
    }

    std::string zludaLowerClusters(Function &F, const ClusterShape &shape)
    {
        Module &M = *F.getParent();
        LLVMContext &ctx = M.getContext();
        const DataLayout &DL = M.getDataLayout();
        Type *I32 = Type::getInt32Ty(ctx);
        const unsigned C = shape.count();
        auto intrinsic = [&](const char *name) {
            return M.getOrInsertFunction(name, FunctionType::get(I32, false));
        };

        SmallVector<CallInst *, 16> calls;
        for (BasicBlock &BB : F)
            for (Instruction &I : BB)
                if (auto *CI = dyn_cast<CallInst>(&I))
                    if (CI->getCalledFunction())
                        calls.push_back(CI);

        // the shared variables of a cluster kernel, packed into windows
        SmallSetVector<GlobalVariable *, 8> shared;
        if (C > 1)
        {
            SmallPtrSet<Value *, 32> seen;
            for (BasicBlock &BB : F)
                for (Instruction &I : BB)
                    for (Value *op : I.operands())
                        zludaSharedVariables(op, shared, seen);
        }

        IRBuilder<> B(&*F.getEntryBlock().getFirstInsertionPt());
        Value *hw_tid_z = B.CreateCall(intrinsic("llvm.agx.thread.position.in.threadgroup.z"));
        Value *hw_ntid_z = B.CreateCall(intrinsic("llvm.agx.threads.per.threadgroup.z"));
        Value *ntid_x = B.CreateCall(intrinsic("llvm.agx.threads.per.threadgroup.x"));
        Value *ntid_y = B.CreateCall(intrinsic("llvm.agx.threads.per.threadgroup.y"));
        Value *hw_linear = B.CreateCall(intrinsic("llvm.agx.thread.index.in.threadgroup"));
        Value *hw_ctaid[3] = {B.CreateCall(intrinsic("llvm.agx.threadgroup.position.in.grid.x")),
                              B.CreateCall(intrinsic("llvm.agx.threadgroup.position.in.grid.y")),
                              B.CreateCall(intrinsic("llvm.agx.threadgroup.position.in.grid.z"))};
        Value *hw_nctaid[3] = {B.CreateCall(intrinsic("llvm.agx.threadgroups.per.grid.x")),
                               B.CreateCall(intrinsic("llvm.agx.threadgroups.per.grid.y")),
                               B.CreateCall(intrinsic("llvm.agx.threadgroups.per.grid.z"))};
        auto k = [&](uint64_t v) { return ConstantInt::get(I32, v); };
        Value *ntid_z = B.CreateUDiv(hw_ntid_z, k(C));
        Value *rank = C > 1 ? B.CreateURem(hw_tid_z, k(C)) : static_cast<Value *>(k(0));
        Value *tid_z = C > 1 ? B.CreateUDiv(hw_tid_z, k(C)) : hw_tid_z;
        Value *linear = hw_linear;
        if (C > 1)
        {
            Value *tid_x = B.CreateCall(intrinsic("llvm.agx.thread.position.in.threadgroup.x"));
            Value *tid_y = B.CreateCall(intrinsic("llvm.agx.thread.position.in.threadgroup.y"));
            linear = B.CreateAdd(tid_x, B.CreateMul(ntid_x, B.CreateAdd(tid_y, B.CreateMul(ntid_y, tid_z))));
        }
        const unsigned *d = shape.dims;
        Value *in_cluster[3] = {B.CreateURem(rank, k(d[0])),
                                B.CreateURem(B.CreateUDiv(rank, k(d[0])), k(d[1])),
                                B.CreateUDiv(rank, k(d[0] * d[1]))};
        Value *ctaid[3], *nctaid[3];
        for (int i = 0; i < 3; i++)
        {
            ctaid[i] = B.CreateAdd(B.CreateMul(hw_ctaid[i], k(d[i])), in_cluster[i]);
            nctaid[i] = B.CreateMul(hw_nctaid[i], k(d[i]));
        }

        // the windows: each variable at its offset, aligned, in a window of
        // `stride` bytes
        uint64_t stride = 0;
        Align max_align(1);
        GlobalVariable *windows = nullptr;
        if (!shared.empty())
        {
            SmallVector<uint64_t, 8> offsets;
            for (GlobalVariable *GV : shared)
            {
                if (GV->hasInitializer() && !isa<UndefValue>(GV->getInitializer()))
                    return "the shared variable " + GV->getName().str() + " of a cluster kernel has an initializer";
                uint64_t size = DL.getTypeAllocSize(GV->getValueType());
                // the shared variables are declarations the backend allocates; extern
                // .shared arrays (dynamic shared memory) have no size
                if (size == 0)
                    return "dynamic shared memory (" + GV->getName().str() + ") is not implemented in cluster kernels";
                Align a = GV->getAlign().value_or(DL.getPrefTypeAlign(GV->getValueType()));
                max_align = std::max(max_align, a);
                stride = alignTo(stride, a);
                offsets.push_back(stride);
                stride += size;
            }
            stride = alignTo(stride, max_align);
            if (stride * C > 32768)
                return "the shared memory of a cluster (" + std::to_string(C) + " x " + std::to_string(stride) +
                       " bytes) exceeds the 32 KiB of a threadgroup";
            auto *array = ArrayType::get(Type::getInt8Ty(ctx), stride * C);
            windows = new GlobalVariable(M, array, false, GlobalValue::InternalLinkage, UndefValue::get(array),
                                         F.getName() + ".cluster_shared", nullptr,
                                         GlobalValue::NotThreadLocal, 3);
            windows->setAlignment(max_align);
            Value *window = B.CreateInBoundsGEP(Type::getInt8Ty(ctx), windows, B.CreateMul(rank, k(stride)));
            SmallVector<Constant *, 8> consts(shared.begin(), shared.end());
            convertUsersOfConstantsToInstructions(consts, &F);
            for (unsigned i = 0; i < shared.size(); i++)
            {
                GlobalVariable *GV = shared[i];
                Value *ptr = B.CreateInBoundsGEP(Type::getInt8Ty(ctx), window, k(offsets[i]));
                GV->replaceUsesWithIf(ptr, [&](Use &U) {
                    auto *I = dyn_cast<Instruction>(U.getUser());
                    return I && I->getFunction() == &F;
                });
            }
        }
        Value *base = windows ? B.CreatePtrToInt(windows, I32) : static_cast<Value *>(k(0));

        for (CallInst *CI : calls)
        {
            StringRef n = CI->getCalledFunction()->getName();
            Value *v = nullptr;
            IRBuilder<> At(CI);
            if (n == "__zluda_agx_cluster")
            {
                Value *values[15] = {in_cluster[0], in_cluster[1], in_cluster[2],
                                     k(d[0]), k(d[1]), k(d[2]),
                                     rank, k(C),
                                     hw_ctaid[0], hw_ctaid[1], hw_ctaid[2],
                                     hw_nctaid[0], hw_nctaid[1], hw_nctaid[2],
                                     k(shape.explicit_ ? 1 : 0)};
                Value *which = CI->getArgOperand(0);
                if (auto *c = dyn_cast<ConstantInt>(which))
                    v = c->getZExtValue() < 15 ? values[c->getZExtValue()] : static_cast<Value *>(k(0));
                else
                {
                    v = k(0);
                    for (unsigned i = 0; i < 15; i++)
                        v = At.CreateSelect(At.CreateICmpEQ(which, k(i)), values[i], v);
                }
            }
            else if (n == "__zluda_agx_cluster_mapa")
            {
                Value *addr = CI->getArgOperand(0), *to = CI->getArgOperand(1);
                v = C > 1 ? At.CreateAdd(addr, At.CreateMul(At.CreateSub(to, rank), k(stride))) : addr;
            }
            else if (n == "__zluda_agx_cluster_rank_of")
                v = C > 1 && stride ? At.CreateUDiv(At.CreateSub(CI->getArgOperand(0), base), k(stride))
                                    : static_cast<Value *>(k(0));
            else if (C > 1)
            {
                if (n == "llvm.agx.thread.position.in.threadgroup.z")
                    v = tid_z;
                else if (n == "llvm.agx.threads.per.threadgroup.z")
                    v = ntid_z;
                else if (n == "llvm.agx.thread.index.in.threadgroup")
                    v = linear;
                else
                    for (int i = 0; i < 3; i++)
                    {
                        std::string axis(1, "xyz"[i]);
                        if (n == "llvm.agx.threadgroup.position.in.grid." + axis)
                            v = ctaid[i];
                        else if (n == "llvm.agx.threadgroups.per.grid." + axis)
                            v = nctaid[i];
                    }
            }
            if (v)
            {
                CI->replaceAllUsesWith(v);
                CI->eraseFromParent();
            }
        }
        return "";
    }
}

LLVM_C_EXTERN_C_BEGIN

// Returns 1 if the module changed, 0 if not, and -1 with a message (to
// dispose with LLVMDisposeMessage) if a kernel's cluster cannot be lowered.
int LLVMZludaAgxLowerClusters(LLVMModuleRef M, char **error)
{
    Module *Mod = unwrap(M);
    SmallVector<Function *, 8> kernels, functions;
    for (Function &F : *Mod)
        if (!F.isDeclaration())
            (F.getCallingConv() == CallingConv::AGX_KERNEL ? kernels : functions).push_back(&F);
    std::optional<ClusterShape> common;
    bool mixed = false;
    for (Function *F : kernels)
    {
        ClusterShape s = zludaClusterShape(*F);
        if (common && !(*common == s))
            mixed = true;
        common = s;
    }
    bool changed = false;
    for (Function *F : kernels)
    {
        if (zludaClusterShape(*F).count() == 1 && !zludaUsesClusterState(*F, false))
            continue;
        changed = true;
        std::string e = zludaLowerClusters(*F, zludaClusterShape(*F));
        if (!e.empty())
        {
            *error = LLVMCreateMessage((F->getName().str() + ": " + e).c_str());
            return -1;
        }
    }
    for (Function *F : functions)
    {
        ClusterShape s = common.value_or(ClusterShape());
        if (!zludaUsesClusterState(*F, mixed || s.count() > 1))
            continue;
        if (mixed)
        {
            *error = LLVMCreateMessage((F->getName().str() +
                                        ": a function reading thread ids, not inlined, is called from kernels of "
                                        "different cluster shapes")
                                           .c_str());
            return -1;
        }
        // shared variables are packed per kernel: a cluster kernel needs them inlined
        bool shared = false;
        for (BasicBlock &BB : *F)
            for (Instruction &I : BB)
            {
                SmallSetVector<GlobalVariable *, 8> vars;
                SmallPtrSet<Value *, 8> seen;
                for (Value *op : I.operands())
                    zludaSharedVariables(op, vars, seen);
                shared |= !vars.empty();
            }
        if (s.count() > 1 && shared)
        {
            *error = LLVMCreateMessage((F->getName().str() +
                                        ": shared memory in a function not inlined into a cluster kernel")
                                           .c_str());
            return -1;
        }
        changed = true;
        std::string e = zludaLowerClusters(*F, s);
        if (!e.empty())
        {
            *error = LLVMCreateMessage((F->getName().str() + ": " + e).c_str());
            return -1;
        }
    }
    return changed ? 1 : 0;
}

LLVM_C_EXTERN_C_END
