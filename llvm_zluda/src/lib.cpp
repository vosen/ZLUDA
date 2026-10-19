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

// AGX has no generic addressing of threadgroup or private memory, so
// isspacep is decided at compile time, after optimization: calls to
// i1 @__zluda_agx_isspacep_<space>(i64) become constants. Returns whether
// there were any.
bool LLVMZludaAgxResolveIsspacep(LLVMModuleRef M)
{
    struct Space
    {
        const char *name;
        bool (*test)(unsigned);
    };
    static const Space spaces[] = {
        {"__zluda_agx_isspacep_global", [](unsigned as) { return as == 0 || as == 1; }},
        {"__zluda_agx_isspacep_shared", [](unsigned as) { return as == 3; }},
        {"__zluda_agx_isspacep_local", [](unsigned as) { return as == 5; }},
        {"__zluda_agx_isspacep_const", [](unsigned as) { return as == 4; }},
        {"__zluda_agx_isspacep_param", [](unsigned) { return false; }},
    };
    Module *Mod = unwrap(M);
    bool found = false;
    // Generic addresses of threadgroup or private memory that are only used
    // as integers (compared, masked, stored) are their 32-bit offsets
    for (Function &F : *Mod)
        for (BasicBlock &BB : F)
            for (Instruction &I : make_early_inc_range(BB))
            {
                auto *P2I = dyn_cast<PtrToIntInst>(&I);
                if (!P2I)
                    continue;
                auto *Cast = dyn_cast<AddrSpaceCastOperator>(P2I->getPointerOperand());
                if (!Cast || Cast->getDestAddressSpace() != 0 ||
                    (Cast->getSrcAddressSpace() != 3 && Cast->getSrcAddressSpace() != 5))
                    continue;
                IRBuilder<> Builder(P2I);
                Value *Offset = Builder.CreatePtrToInt(Cast->getPointerOperand(), Builder.getInt32Ty());
                P2I->replaceAllUsesWith(Builder.CreateZExtOrTrunc(Offset, P2I->getType()));
                P2I->eraseFromParent();
                if (auto *CastInst = dyn_cast<Instruction>(Cast))
                    if (CastInst->use_empty())
                        CastInst->eraseFromParent();
                found = true;
            }
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
            Call->replaceAllUsesWith(ConstantInt::getBool(Call->getContext(), space.test(as)));
            Call->eraseFromParent();
            found = true;
        }
        if (F->use_empty())
            F->eraseFromParent();
    }
    return found;
}

LLVM_C_EXTERN_C_END
