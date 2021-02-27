// Some LLVM  headers produce this warning, we'd like to pass them
// with -isystem, but it's currently impossible to do so
#if defined(__GNUC__)
    #pragma GCC diagnostic push
    #pragma GCC diagnostic ignored "-Wunused-parameter"
#endif
#include "llvm-c/Core.h"
#include "llvm/IR/Instructions.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/Support/Alignment.h"
#if defined(__GNUC__)
    #pragma GCC diagnostic pop
#endif

using namespace llvm;

// Taken from Core.cpp
static AtomicRMWInst::BinOp mapFromLLVMRMWBinOp(LLVMAtomicRMWBinOp BinOp)
{
    switch (BinOp)
    {
    case LLVMAtomicRMWBinOpXchg:
        return AtomicRMWInst::Xchg;
    case LLVMAtomicRMWBinOpAdd:
        return AtomicRMWInst::Add;
    case LLVMAtomicRMWBinOpSub:
        return AtomicRMWInst::Sub;
    case LLVMAtomicRMWBinOpAnd:
        return AtomicRMWInst::And;
    case LLVMAtomicRMWBinOpNand:
        return AtomicRMWInst::Nand;
    case LLVMAtomicRMWBinOpOr:
        return AtomicRMWInst::Or;
    case LLVMAtomicRMWBinOpXor:
        return AtomicRMWInst::Xor;
    case LLVMAtomicRMWBinOpMax:
        return AtomicRMWInst::Max;
    case LLVMAtomicRMWBinOpMin:
        return AtomicRMWInst::Min;
    case LLVMAtomicRMWBinOpUMax:
        return AtomicRMWInst::UMax;
    case LLVMAtomicRMWBinOpUMin:
        return AtomicRMWInst::UMin;
    case LLVMAtomicRMWBinOpFAdd:
        return AtomicRMWInst::FAdd;
    case LLVMAtomicRMWBinOpFSub:
        return AtomicRMWInst::FSub;
    case LLVMAtomicRMWBinOpFMax:
        return AtomicRMWInst::FMax;
    case LLVMAtomicRMWBinOpFMin:
        return AtomicRMWInst::FMin;
    }

    llvm_unreachable("Invalid LLVMAtomicRMWBinOp value!");
}

// Taken from Core.cpp
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

LLVM_C_EXTERN_C_BEGIN

LLVMValueRef LLVMZludaBuildAtomicRMW(LLVMBuilderRef B, LLVMAtomicRMWBinOp op,
                                     LLVMValueRef PTR, LLVMValueRef Val,
                                     char *scope,
                                     LLVMAtomicOrdering ordering,
                                     unsigned Align)
{
    AtomicRMWInst::BinOp intop = mapFromLLVMRMWBinOp(op);
    IRBuilder<> *builder = unwrap(B);
    LLVMContext &context = builder->getContext();
    SyncScope::ID syncScope = context.getOrInsertSyncScopeID(scope);
    MaybeAlign maybeAlign = MaybeAlign(Align);
    auto inst = builder->CreateAtomicRMW(intop, unwrap(PTR), unwrap(Val),
                                         maybeAlign,
                                         mapFromLLVMOrdering(ordering), syncScope);
    return wrap(inst);
}

LLVMValueRef LLVMZludaBuildAtomicCmpXchg(LLVMBuilderRef B, LLVMValueRef Ptr,
                                         LLVMValueRef Cmp, LLVMValueRef New,
                                         char *scope,
                                         LLVMAtomicOrdering SuccessOrdering,
                                         LLVMAtomicOrdering FailureOrdering,
                                         unsigned Align)
{
    IRBuilder<> *builder = unwrap(B);
    LLVMContext &context = builder->getContext();
    SyncScope::ID syncScope = context.getOrInsertSyncScopeID(scope);
    Value *newUnwrapped = unwrap(New);
    MaybeAlign maybeAlign = MaybeAlign(Align);
    auto inst = unwrap(B)->CreateAtomicCmpXchg(
        unwrap(Ptr), unwrap(Cmp), newUnwrapped, maybeAlign, mapFromLLVMOrdering(SuccessOrdering),
        mapFromLLVMOrdering(FailureOrdering),
        syncScope);
    return wrap(inst);
}

void LLVMZludaSetFastMathFlags(LLVMValueRef Val, unsigned Flags)
{
    Instruction *I = dyn_cast<Instruction>(unwrap(Val));
    FastMathFlags flags = FastMathFlags();
    flags.setAllowReassoc((Flags & FastMathFlags::AllowReassoc) != 0);
    flags.setNoNaNs((Flags & FastMathFlags::NoNaNs) != 0);
    flags.setNoInfs((Flags & FastMathFlags::NoInfs) != 0);
    flags.setNoSignedZeros((Flags & FastMathFlags::NoSignedZeros) != 0);
    flags.setAllowReciprocal((Flags & FastMathFlags::AllowReciprocal) != 0);
    flags.setAllowContract((Flags & FastMathFlags::AllowContract) != 0);
    flags.setApproxFunc((Flags & FastMathFlags::ApproxFunc) != 0);
    I->setFastMathFlags(flags);
}

LLVMValueRef LLVMZludaBuildFence(LLVMBuilderRef B, LLVMAtomicOrdering Ordering,
                                 char *scope, const char *Name)
{
    IRBuilder<> *builder = unwrap(B);
    SyncScope::ID syncScope = builder->getContext().getOrInsertSyncScopeID(scope);
    return wrap(
        builder->CreateFence(mapFromLLVMOrdering(Ordering),
                             syncScope,
                             Name));
}

LLVMValueRef LLVMZludaBuildAlloca(LLVMBuilderRef B, LLVMTypeRef Ty,
                                  unsigned AddrSpace, const char *Name)
{
    IRBuilder<> *builder = unwrap(B);
    return wrap(builder->CreateAlloca(unwrap(Ty), AddrSpace, nullptr, Name));
}

void LLVMZludaSetAtomic(LLVMContextRef C, LLVMValueRef Val, LLVMAtomicOrdering Ordering, char *scope)
{
    LLVMContext *Ctx = unwrap(C);
    AtomicOrdering O = mapFromLLVMOrdering(Ordering);
    SyncScope::ID syncScope = Ctx->getOrInsertSyncScopeID(scope);
    if (LoadInst *LI = dyn_cast<LoadInst>(unwrap(Val)))
        return LI->setAtomic(O, syncScope);
    return cast<StoreInst>(unwrap(Val))->setAtomic(O, syncScope);
}

LLVM_C_EXTERN_C_END