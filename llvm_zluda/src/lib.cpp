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
