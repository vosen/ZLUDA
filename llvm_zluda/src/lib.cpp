#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm-c/Core.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/Type.h>
#include <llvm/IR/Instructions.h>
#include <llvm/IR/AutoUpgrade.h>
#include <llvm/Bitcode/BitcodeReader.h>
#include <llvm/IRReader/IRReader.h>
#include <llvm/Linker/Linker.h>
#include <llvm/Support/MemoryBuffer.h>
#include <llvm/Support/SourceMgr.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/Operator.h>
#include <llvm/IR/Constants.h>
#include <llvm/IR/ReplaceConstant.h>
#include <llvm/Analysis/ConstantFolding.h>
#include <llvm/Analysis/ValueTracking.h>
#include <llvm/Transforms/Utils/ModuleUtils.h>
#include <llvm/Transforms/Utils/BasicBlockUtils.h>
#include <llvm/IR/InstIterator.h>
#include <llvm/IR/IntrinsicInst.h>
#include <llvm/ADT/SetVector.h>
#include <llvm/Support/Alignment.h>
#include <array>
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
// Links into Dest the definitions of the library (bitcode, loaded lazily, or textual IR) that Dest uses, and
// those they use in turn: the AGX helper library is linked into every module, and parsing, verifying and
// optimizing all of it took most of a small kernel's compile time. Returns false with *Error set on failure.
bool LLVMZludaLinkUsedDefinitions(LLVMModuleRef Dest, const char *Data, size_t Size, const char *Name, char **Error)
{
    using namespace llvm;
    Module &D = *unwrap(Dest);
    std::unique_ptr<MemoryBuffer> Buffer =
        MemoryBuffer::getMemBuffer(StringRef(Data, Size), Name, /*RequiresNullTerminator=*/false);
    std::unique_ptr<Module> Lib;
    if (isBitcode(reinterpret_cast<const unsigned char *>(Data), reinterpret_cast<const unsigned char *>(Data) + Size))
    {
        Expected<std::unique_ptr<Module>> Lazy = getOwningLazyBitcodeModule(std::move(Buffer), D.getContext());
        if (!Lazy)
        {
            *Error = strdup(toString(Lazy.takeError()).c_str());
            return false;
        }
        Lib = std::move(*Lazy);
    }
    else
    {
        SMDiagnostic Diag;
        Lib = parseIR(Buffer->getMemBufferRef(), Diag, D.getContext());
        if (!Lib)
        {
            std::string Message;
            raw_string_ostream OS(Message);
            Diag.print(Name, OS);
            *Error = strdup(Message.c_str());
            return false;
        }
    }
    if (Linker::linkModules(D, std::move(Lib), Linker::Flags::LinkOnlyNeeded))
    {
        *Error = strdup("could not link the helper library");
        return false;
    }
    return true;
}

void LLVMZludaUpgradeFunctionAttributes(LLVMModuleRef M)
{
    for (llvm::Function &F : *llvm::unwrap(M))
        llvm::UpgradeFunctionAttributes(F);
}

// The address space a generic address comes from: looks through integer
// casts, constant offsets, address space casts and pointer arithmetic.
// Addresses of unknown origin are device (global) addresses.
static unsigned zludaAgxSourceAddressSpace(Value *V, Value **Source = nullptr)
{
    for (unsigned depth = 0; depth < 64; depth++)
    {
        if (V->getType()->isPointerTy() && V->getType()->getPointerAddressSpace() != 0)
        {
            if (Source)
                *Source = V;
            return V->getType()->getPointerAddressSpace();
        }
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
    // Kernel parameters are constant memory too (address space 4); their window is in the .global window, and
    // not in the .const one (PTX ISA 6.4.1.1)
    auto isParam = [](Value *Source) {
        return Source && isa<Argument>(getUnderlyingObject(Source));
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
            Value *Source = nullptr;
            unsigned as = zludaAgxSourceAddressSpace(Call->getArgOperand(0), &Source);
            Value *Result;
            if (as == 4 && isParam(Source))
            {
                StringRef name = space.name;
                Result = ConstantInt::getBool(Call->getContext(),
                                              name.ends_with("_param") || name.ends_with("_global"));
            }
            else if (as == 0 && StringRef(space.name).ends_with("_const"))
            {
                // a device address of unknown origin: in one of the module's .const variables, which are in the
                // global data buffer
                IRBuilder<> Builder(Call);
                Value *A = Call->getArgOperand(0);
                Result = Builder.getFalse();
                for (GlobalVariable &GV : Mod->globals())
                {
                    uint64_t size = GV.getValueType()->isSized() ? Mod->getDataLayout().getTypeAllocSize(GV.getValueType()) : 0;
                    if (GV.getAddressSpace() != 4 || GV.isDeclaration() || size == 0)
                        continue;
                    Value *Start = Builder.CreatePtrToInt(
                        Builder.CreateAddrSpaceCast(&GV, PointerType::get(Call->getContext(), 0)), A->getType());
                    Value *Offset = Builder.CreateSub(A, Start);
                    Result = Builder.CreateOr(Result, Builder.CreateICmpULT(Offset, ConstantInt::get(A->getType(), size)));
                }
            }
            else if (as != 0 || space.tags == 0)
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

        // the shared variables of a cluster kernel, packed into windows in the order of the module (PTX
        // programs compute shared addresses from that layout)
        SmallSetVector<GlobalVariable *, 8> shared;
        if (C > 1)
        {
            SmallSetVector<GlobalVariable *, 8> used;
            SmallPtrSet<Value *, 32> seen;
            for (BasicBlock &BB : F)
                for (Instruction &I : BB)
                    for (Value *op : I.operands())
                        zludaSharedVariables(op, used, seen);
            // except the shared memory size of the launch and the end of the static shared memory
            // (zluda_ptx_impl_agx.ll's %..._smem_... registers), which stay the threadgroup's
            for (GlobalVariable &GV : M.globals())
                if (used.count(&GV) && GV.getName() != "__zluda_agx_smem_bytes" &&
                    GV.getName() != "__zluda_agx_smem_end")
                    shared.insert(&GV);
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
            // more than the 32 KiB of a threadgroup: the windows move to device memory after the address spaces
            // are lowered (LLVMZludaAgxSpillClusterShared)
            if (stride * C > 32768)
                F.addFnAttr("agx-spill-shared", std::to_string(stride * C));
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

// A cluster kernel whose shared memory (the windows of its CTAs) is more than the 32 KiB of a threadgroup keeps it
// in device memory: a pool of slabs of the windows' size per kernel, the threadgroup claiming a free slab at its
// start (its first thread, the others waiting at a barrier) and freeing it when its last thread returns. Every
// threadgroup memory access of the kernel (after the address spaces are lowered, generic ones included) becomes a
// device memory access at the slab plus its offset in the windows, and threadgroup scope fences and atomics become
// device scope ones. A threadgroup that finds no free slab waits for one, as the tensor memory slabs do.
namespace
{
    const unsigned SPILL_SLABS = 8;

    std::string zludaSpillClusterShared(Function &F, uint64_t size)
    {
        Module &M = *F.getParent();
        LLVMContext &ctx = M.getContext();
        Type *I8 = Type::getInt8Ty(ctx), *I32 = Type::getInt32Ty(ctx), *I64 = Type::getInt64Ty(ctx);
        GlobalVariable *windows = M.getNamedGlobal((F.getName() + ".cluster_shared").str());
        if (!windows)
            return "";
        auto global = [&](Type *T, const Twine &name, unsigned as) {
            return new GlobalVariable(M, T, false, GlobalValue::InternalLinkage,
                                      as == 3 ? static_cast<Constant *>(UndefValue::get(T)) : Constant::getNullValue(T),
                                      F.getName() + name, nullptr, GlobalValue::NotThreadLocal, as);
        };
        GlobalVariable *pool = global(ArrayType::get(I8, size * SPILL_SLABS), ".cluster_pool", 1);
        pool->setAlignment(std::max(windows->getAlign().valueOrOne(), Align(16)));
        GlobalVariable *owner = global(ArrayType::get(I32, SPILL_SLABS), ".cluster_owner", 1);
        GlobalVariable *count = global(ArrayType::get(I32, SPILL_SLABS), ".cluster_count", 1);
        GlobalVariable *slab_var = global(I32, ".cluster_slab", 3);
        slab_var->setAlignment(Align(4));
        auto call = [&](IRBuilder<> &B, const char *name) {
            return B.CreateCall(M.getOrInsertFunction(name, FunctionType::get(I32, false)));
        };
        auto barrier = [&](IRBuilder<> &B) {
            B.CreateFence(AtomicOrdering::SequentiallyConsistent);
            B.CreateCall(M.getOrInsertFunction("llvm.agx.threadgroup.barrier", FunctionType::get(B.getVoidTy(), false)));
            B.CreateFence(AtomicOrdering::SequentiallyConsistent);
        };

        // the claim: the first thread takes a free slab (spinning over the pool) and sets its count of threads
        BasicBlock *entry = &F.getEntryBlock();
        BasicBlock *body = entry->splitBasicBlock(entry->getFirstInsertionPt(), "spill.body");
        entry->getTerminator()->eraseFromParent();
        BasicBlock *claim = BasicBlock::Create(ctx, "spill.claim", &F, body);
        BasicBlock *join = BasicBlock::Create(ctx, "spill.join", &F, body);
        IRBuilder<> B(entry);
        Value *first = B.CreateICmpEQ(call(B, "llvm.agx.thread.index.in.threadgroup"), B.getInt32(0));
        Value *threads = B.CreateMul(call(B, "llvm.agx.threads.per.threadgroup.x"),
                                     B.CreateMul(call(B, "llvm.agx.threads.per.threadgroup.y"),
                                                 call(B, "llvm.agx.threads.per.threadgroup.z")));
        B.CreateCondBr(first, claim, join);
        B.SetInsertPoint(claim);
        PHINode *i = B.CreatePHI(I32, 2);
        i->addIncoming(B.getInt32(0), entry);
        Value *slot = B.CreateInBoundsGEP(I32, owner, i);
        Value *taken = B.CreateExtractValue(
            B.CreateAtomicCmpXchg(slot, B.getInt32(0), B.getInt32(1), MaybeAlign(4), AtomicOrdering::AcquireRelease,
                                  AtomicOrdering::Acquire),
            1);
        Value *next = B.CreateURem(B.CreateAdd(i, B.getInt32(1)), B.getInt32(SPILL_SLABS));
        i->addIncoming(next, claim);
        BasicBlock *got = BasicBlock::Create(ctx, "spill.got", &F, join);
        B.CreateCondBr(taken, got, claim);
        B.SetInsertPoint(got);
        B.CreateAlignedStore(threads, B.CreateInBoundsGEP(I32, count, i), MaybeAlign(4));
        B.CreateAlignedStore(i, slab_var, MaybeAlign(4));
        B.CreateBr(join);
        B.SetInsertPoint(join);
        barrier(B);
        Value *slab = B.CreateAlignedLoad(I32, slab_var, MaybeAlign(4));
        Value *base = B.CreateAdd(B.CreatePtrToInt(pool, I64), B.CreateMul(B.CreateZExt(slab, I64), B.getInt64(size)));
        B.CreateBr(body);

        // the accesses
        Value *windows_address = ConstantExpr::getPtrToInt(windows, I32);
        auto device = [&](Instruction *at, Value *p) -> Value * {
            IRBuilder<> A(at);
            Value *offset = A.CreateSub(A.CreatePtrToInt(p, I32), windows_address);
            Value *address = A.CreateAdd(base, A.CreateZExt(offset, I64));
            return A.CreateIntToPtr(address, PointerType::get(ctx, 1));
        };
        auto shared = [&](Value *p) { return p->getType()->isPointerTy() && p->getType()->getPointerAddressSpace() == 3 && p != slab_var; };
        SmallVector<Instruction *, 64> work;
        for (BasicBlock &BB : F)
            for (Instruction &I : BB)
                work.push_back(&I);
        SmallVector<ReturnInst *, 4> returns;
        for (Instruction *I : work)
        {
            if (auto *L = dyn_cast<LoadInst>(I))
            {
                if (shared(L->getPointerOperand()))
                    L->setOperand(0, device(L, L->getPointerOperand()));
                if (L->isAtomic() && L->getSyncScopeID() != SyncScope::SingleThread)
                    L->setSyncScopeID(SyncScope::System);
            }
            else if (auto *S = dyn_cast<StoreInst>(I))
            {
                if (shared(S->getPointerOperand()))
                    S->setOperand(1, device(S, S->getPointerOperand()));
                if (S->isAtomic() && S->getSyncScopeID() != SyncScope::SingleThread)
                    S->setSyncScopeID(SyncScope::System);
            }
            else if (auto *R = dyn_cast<AtomicRMWInst>(I))
            {
                if (shared(R->getPointerOperand()))
                    R->setOperand(0, device(R, R->getPointerOperand()));
                R->setSyncScopeID(SyncScope::System);
            }
            else if (auto *X = dyn_cast<AtomicCmpXchgInst>(I))
            {
                if (shared(X->getPointerOperand()))
                    X->setOperand(0, device(X, X->getPointerOperand()));
                X->setSyncScopeID(SyncScope::System);
            }
            else if (auto *Fe = dyn_cast<FenceInst>(I))
                Fe->setSyncScopeID(SyncScope::System);
            else if (auto *MI = dyn_cast<MemIntrinsic>(I))
            {
                if (shared(MI->getRawDest()))
                    MI->setDest(device(MI, MI->getRawDest()));
                if (auto *MT = dyn_cast<MemTransferInst>(MI))
                    if (shared(MT->getRawSource()))
                        MT->setSource(device(MT, MT->getRawSource()));
                // the address spaces are part of the intrinsic's name
                SmallVector<Type *, 3> types;
                for (Value *op : MI->args())
                    types.push_back(op->getType());
                if (isa<MemTransferInst>(MI))
                    MI->setCalledFunction(Intrinsic::getOrInsertDeclaration(&M, MI->getIntrinsicID(),
                                                                            {types[0], types[1], types[2]}));
                else
                    MI->setCalledFunction(Intrinsic::getOrInsertDeclaration(&M, MI->getIntrinsicID(),
                                                                            {types[0], types[2]}));
            }
            else if (auto *CI = dyn_cast<CallInst>(I))
            {
                for (Value *op : CI->args())
                    if (shared(op))
                        return "a call with a threadgroup memory pointer (" +
                               CI->getCalledOperand()->getName().str() + ") in a kernel whose cluster shared memory is in device memory";
            }
            else if (auto *Ret = dyn_cast<ReturnInst>(I))
                returns.push_back(Ret);
        }
        // the last thread of the threadgroup frees the slab
        for (ReturnInst *Ret : returns)
        {
            IRBuilder<> A(Ret);
            Value *old = A.CreateAtomicRMW(AtomicRMWInst::Sub, A.CreateInBoundsGEP(I32, count, slab), A.getInt32(1),
                                           MaybeAlign(4), AtomicOrdering::AcquireRelease);
            Instruction *then = SplitBlockAndInsertIfThen(A.CreateICmpEQ(old, A.getInt32(1)), Ret, false);
            IRBuilder<> T(then);
            T.CreateAlignedStore(T.getInt32(0), T.CreateInBoundsGEP(I32, owner, slab), MaybeAlign(4))
                ->setAtomic(AtomicOrdering::Release);
        }
        // the windows' addresses are offsets now
        windows->replaceAllUsesWith(ConstantExpr::getIntToPtr(ConstantInt::get(I32, 0), windows->getType()));
        windows->eraseFromParent();
        F.removeFnAttr("agx-spill-shared");
        return "";
    }
}

LLVM_C_EXTERN_C_BEGIN

// Returns 1 if the module changed, 0 if not, and -1 with a message if a kernel's shared memory cannot be moved.
int LLVMZludaAgxSpillClusterShared(LLVMModuleRef M, char **error)
{
    bool changed = false;
    for (Function &F : *unwrap(M))
    {
        Attribute A = F.getFnAttribute("agx-spill-shared");
        uint64_t size = 0;
        if (!A.isValid() || A.getValueAsString().getAsInteger(10, size))
            continue;
        std::string e = zludaSpillClusterShared(F, size);
        if (!e.empty())
        {
            *error = LLVMCreateMessage((F.getName().str() + ": " + e).c_str());
            return -1;
        }
        changed = true;
    }
    return changed ? 1 : 0;
}

LLVM_C_EXTERN_C_END

LLVM_C_EXTERN_C_BEGIN

// The module's global and constant variables stay in the image even when no
// kernel uses them: cuModuleGetGlobal finds them by name (ZLUDA's own
// helper variables, __zluda_*, may go).
void LLVMZludaAgxKeepGlobals(LLVMModuleRef M)
{
    Module *Mod = unwrap(M);
    SmallVector<GlobalValue *, 16> keep;
    for (GlobalVariable &GV : Mod->globals())
    {
        unsigned as = GV.getAddressSpace();
        if (GV.isDeclaration() || (as != 1 && as != 4) || GV.getName().starts_with("__zluda") ||
            GV.getName().starts_with("llvm."))
            continue;
        keep.push_back(&GV);
    }
    if (!keep.empty())
        appendToUsed(*Mod, keep);
}

LLVM_C_EXTERN_C_END

LLVM_C_EXTERN_C_BEGIN

// ZLUDA_AGX_VIA_AIR, the .air written for Metal: Metal compiles no writable
// program-scope variables (MSL has none; a module with a device global
// crashes its compiler on an M2 Max), so a PTX module's .global variables
// become one device buffer. The functions other than kernels that use them
// are first inlined into the kernels (LLVMZludaAirInlineGlobalUsers and the
// always-inline pass); LLVMZludaAirGlobalsToBuffer then lays the variables
// out in the buffer, gives each kernel using them a last parameter, a
// pointer to it ("zluda-globals"), and describes the buffer in JSON for the
// host, which allocates it and writes its initial contents:
//   {"size": N, "image": "<hex>", "relocs": [[offset, target], ...],
//    "symbols": {"name": offset, ...}, "kernels": ["name", ...]}
// An 8-byte reloc at offset holds the buffer's address plus target (an
// initializer's address of a variable). The AGX compilation is unaffected:
// these run on a copy of the module.

// Whether V's address is used other than to load through it (as an integer,
// in a variable's initializer, stored, passed ...).
static bool zludaAddressEscapes(Value *V)
{
    for (User *U : V->users())
    {
        if (auto *L = dyn_cast<LoadInst>(U); L && L->getPointerOperand() == V)
            continue;
        if (auto *GEP = dyn_cast<GEPOperator>(U); GEP && GEP->getPointerOperand() == V)
        {
            if (zludaAddressEscapes(GEP))
                return true;
            continue;
        }
        return true;
    }
    return false;
}

// .global variables, and .const variables whose address escapes: the host
// knows no address of a Metal program-scope constant, so one held in a
// variable or compared as an integer must be in the buffer too (CUDA's
// generic(cv), mask(foo+4), mov.u64 %r, foo). The AGX table of mask()
// initializers is the init kernel's job, not a variable.
static bool zludaIsBufferGlobal(GlobalVariable &GV)
{
    if (GV.getName().starts_with("llvm.") || GV.getName() == "__zluda_agx_masked_init")
        return false;
    if (GV.getAddressSpace() == 1)
        return true;
    return GV.getAddressSpace() == 2 && zludaAddressEscapes(&GV);
}

// Replaces the uses in F of Old, a variable in address space 1 or 2, with
// New, its address in the buffer (address space 1): GEPs are rebuilt in the
// new address space, casts to it disappear, other users take the device
// pointer. (Each kernel has its own buffer argument: uses in other functions
// stay.)
static void zludaRetarget(Value *Old, Value *New, Function *F)
{
    for (Use &U : make_early_inc_range(Old->uses()))
    {
        auto *I = dyn_cast<Instruction>(U.getUser());
        if (!I || I->getFunction() != F)
            continue;
        if (auto *GEP = dyn_cast<GetElementPtrInst>(I); GEP && GEP->getPointerOperand() == Old &&
                                                         Old->getType() != New->getType())
        {
            IRBuilder<> B(GEP);
            SmallVector<Value *, 4> idx(GEP->indices());
            Value *NG = B.CreateGEP(GEP->getSourceElementType(), New, idx, "", GEP->getNoWrapFlags());
            zludaRetarget(GEP, NG, F);
            NG->takeName(GEP);
            if (GEP->use_empty())
                GEP->eraseFromParent();
            continue;
        }
        if (auto *ASC = dyn_cast<AddrSpaceCastInst>(I); ASC && ASC->getType() == New->getType())
        {
            ASC->replaceAllUsesWith(New);
            ASC->eraseFromParent();
            continue;
        }
        U.set(New);
    }
}

// The functions whose instructions use V, through constant expressions.
static void zludaUsingFunctions(Value *V, SmallPtrSetImpl<Function *> &Out)
{
    for (User *U : V->users())
    {
        if (auto *I = dyn_cast<Instruction>(U))
            Out.insert(I->getFunction());
        else if (isa<ConstantExpr>(U) || isa<ConstantAggregate>(U))
            zludaUsingFunctions(U, Out);
    }
}

// Marks the functions other than kernels that use a buffer global
// alwaysinline; returns how many it marked.
int LLVMZludaAirInlineGlobalUsers(LLVMModuleRef M)
{
    Module *Mod = unwrap(M);
    SmallPtrSet<Function *, 8> users;
    for (GlobalVariable &GV : Mod->globals())
        if (zludaIsBufferGlobal(GV))
            zludaUsingFunctions(&GV, users);
    int marked = 0;
    for (Function *F : users)
    {
        if (F->hasFnAttribute("air-kernel") || F->hasFnAttribute(Attribute::AlwaysInline) || F->use_empty())
            continue;
        F->removeFnAttr(Attribute::NoInline);
        F->addFnAttr(Attribute::AlwaysInline);
        marked++;
    }
    return marked;
}

static std::string zludaHex(const std::vector<uint8_t> &bytes)
{
    static const char digits[] = "0123456789abcdef";
    std::string out;
    out.reserve(bytes.size() * 2);
    for (uint8_t b : bytes)
    {
        out += digits[b >> 4];
        out += digits[b & 15];
    }
    return out;
}

namespace
{
    struct ZludaGlobalsImage
    {
        const DataLayout &DL;
        DenseMap<const GlobalValue *, uint64_t> offsets;
        std::vector<uint8_t> image;
        std::vector<std::pair<uint64_t, uint64_t>> relocs;

        void putInt(const APInt &v, uint64_t at, uint64_t size)
        {
            for (uint64_t i = 0; i < size && i * 8 < v.getBitWidth(); i++)
                image[at + i] = (uint8_t)v.extractBitsAsZExtValue(
                    std::min<unsigned>(8, v.getBitWidth() - i * 8), i * 8);
        }

        // Whether C is a variable's address plus a constant: a GEP or cast
        // of it (IsConstantOffsetFromGlobal), or integer arithmetic on its
        // ptrtoint (PTX's generic(v)+4).
        bool addressOf(Constant *C, GlobalValue *&GV, int64_t &off)
        {
            APInt o;
            if (IsConstantOffsetFromGlobal(C, GV, o, DL))
            {
                off = o.getSExtValue();
                return true;
            }
            auto *CE = dyn_cast<ConstantExpr>(C);
            if (!CE)
                return false;
            if (CE->getOpcode() == Instruction::PtrToInt || CE->getOpcode() == Instruction::IntToPtr ||
                CE->isCast())
                return addressOf(CE->getOperand(0), GV, off);
            if (CE->getOpcode() == Instruction::Add || CE->getOpcode() == Instruction::Sub)
            {
                auto *K = dyn_cast<ConstantInt>(CE->getOperand(1));
                Constant *Base = CE->getOperand(0);
                if (!K && CE->getOpcode() == Instruction::Add)
                {
                    K = dyn_cast<ConstantInt>(CE->getOperand(0));
                    Base = CE->getOperand(1);
                }
                if (!K || !addressOf(Base, GV, off))
                    return false;
                off += CE->getOpcode() == Instruction::Add ? K->getSExtValue() : -K->getSExtValue();
                return true;
            }
            return false;
        }

        // Writes C at offset at; returns an error message or "".
        std::string put(Constant *C, uint64_t at)
        {
            Type *T = C->getType();
            if (isa<UndefValue>(C) || isa<ConstantAggregateZero>(C) || isa<ConstantPointerNull>(C))
                return "";
            if (auto *CI = dyn_cast<ConstantInt>(C))
            {
                putInt(CI->getValue(), at, DL.getTypeStoreSize(T));
                return "";
            }
            if (auto *CF = dyn_cast<ConstantFP>(C))
            {
                putInt(CF->getValueAPF().bitcastToAPInt(), at, DL.getTypeStoreSize(T));
                return "";
            }
            if (auto *ST = dyn_cast<StructType>(T))
            {
                const StructLayout *SL = DL.getStructLayout(ST);
                for (unsigned i = 0; i < ST->getNumElements(); i++)
                    if (std::string e = put(C->getAggregateElement(i), at + SL->getElementOffset(i)); !e.empty())
                        return e;
                return "";
            }
            if (T->isArrayTy() || T->isVectorTy())
            {
                Type *E = T->isArrayTy() ? T->getArrayElementType() : cast<VectorType>(T)->getElementType();
                uint64_t n = T->isArrayTy() ? T->getArrayNumElements() : cast<FixedVectorType>(T)->getNumElements();
                uint64_t stride = T->isArrayTy() ? DL.getTypeAllocSize(E) : DL.getTypeStoreSize(E);
                for (uint64_t i = 0; i < n; i++)
                    if (std::string e = put(C->getAggregateElement(i), at + i * stride); !e.empty())
                        return e;
                return "";
            }
            // A number seen through pointer casts (a function's ID, LLVMZludaAirFunctionPointers:
            // inttoptr ID, and ptrtoint of that).
            {
                Constant *V = C;
                while (auto *CE = dyn_cast<ConstantExpr>(V))
                {
                    if (CE->getOpcode() != Instruction::IntToPtr && CE->getOpcode() != Instruction::PtrToInt)
                        break;
                    V = CE->getOperand(0);
                }
                if (auto *K = dyn_cast<ConstantInt>(V); K && V != C)
                {
                    putInt(K->getValue().zextOrTrunc(8 * DL.getTypeStoreSize(T)), at, DL.getTypeStoreSize(T));
                    return "";
                }
            }
            // The address of a variable (possibly offset, cast to an integer).
            GlobalValue *GV = nullptr;
            int64_t off = 0;
            if (addressOf(C, GV, off))
            {
                auto it = offsets.find(GV);
                if (it == offsets.end())
                    return "an initializer holds the address of " + GV->getName().str() +
                           ", which is not in the globals buffer";
                if (DL.getTypeStoreSize(T) != 8)
                    return "an initializer holds a variable's address in fewer than 8 bytes";
                relocs.push_back({at, it->second + off});
                return "";
            }
            std::string text;
            raw_string_ostream os(text);
            os << *C;
            return "an initializer the globals buffer cannot hold: " + os.str();
        }
    };
}

// Returns 1 with *json set if the module had buffer globals, 0 if not, -1
// with *error.
int LLVMZludaAirGlobalsToBuffer(LLVMModuleRef M, char **json, char **error)
{
    Module *Mod = unwrap(M);
    const DataLayout &DL = Mod->getDataLayout();
    LLVMContext &Ctx = Mod->getContext();
    SmallVector<GlobalVariable *, 8> globals;
    for (GlobalVariable &GV : Mod->globals())
        if (zludaIsBufferGlobal(GV))
            globals.push_back(&GV);
    if (globals.empty())
        return 0;
    auto fail = [&](const std::string &e) {
        *error = LLVMCreateMessage(e.c_str());
        return -1;
    };

    ZludaGlobalsImage img{DL, {}, {}, {}};
    uint64_t size = 0;
    for (GlobalVariable *GV : globals)
    {
        Align a = DL.getValueOrABITypeAlignment(GV->getAlign(), GV->getValueType());
        size = alignTo(size, a);
        img.offsets[GV] = size;
        size += DL.getTypeAllocSize(GV->getValueType());
    }
    img.image.assign(std::max<uint64_t>(size, 1), 0);
    for (GlobalVariable *GV : globals)
        if (GV->hasInitializer())
            // (folded: a function's ID reads ptrtoint (inttoptr ID), which needs the data layout to fold)
            if (std::string e = img.put(ConstantFoldConstant(GV->getInitializer(), DL), img.offsets[GV]); !e.empty())
                return fail(GV->getName().str() + ": " + e);

    // The mask() initializers (ZLUDA's AGX table): bytes of a variable's
    // address, written by the init kernel once the buffer has an address.
    std::vector<std::array<uint64_t, 4>> masked;
    if (GlobalVariable *T = Mod->getGlobalVariable("__zluda_agx_masked_init", true); T && T->hasInitializer())
    {
        auto *Arr = T->getInitializer();
        for (unsigned i = 0, n = Arr->getType()->getArrayNumElements(); i < n; i++)
        {
            Constant *E = Arr->getAggregateElement(i);
            GlobalValue *SG = nullptr, *TG = nullptr;
            int64_t so = 0, to = 0;
            auto *Desc = dyn_cast<ConstantInt>(E->getAggregateElement(2));
            if (!img.addressOf(E->getAggregateElement(0u), SG, so) || !img.addressOf(E->getAggregateElement(1), TG, to) ||
                !Desc || !img.offsets.count(SG) || !img.offsets.count(TG))
                return fail("a mask() initializer the globals buffer cannot hold");
            uint64_t d = Desc->getZExtValue();
            masked.push_back({img.offsets[SG] + so, img.offsets[TG] + to, d & 63, d >> 8});
        }
        T->setInitializer(nullptr);
        T->eraseFromParent();
    }

    SmallPtrSet<Function *, 8> users;
    for (GlobalVariable *GV : globals)
        zludaUsingFunctions(GV, users);
    // Functions inlined into every caller are dead.
    SmallVector<Function *, 4> dead;
    for (Function *F : users)
        if (!F->hasFnAttribute("air-kernel") && F->use_empty())
            dead.push_back(F);
    for (Function *F : dead)
    {
        users.erase(F);
        F->eraseFromParent();
    }
    for (Function *K : users)
        if (!K->hasFnAttribute("air-kernel"))
            return fail(K->getName().str() + " uses a .global variable and could not be inlined into a kernel");
    std::string kernels;
    SmallVector<Constant *, 8> consts(globals.begin(), globals.end());
    for (Function *K : users)
    {
        // K with a last parameter, the buffer's address.
        FunctionType *FT = K->getFunctionType();
        SmallVector<Type *, 8> params(FT->params());
        params.push_back(PointerType::get(Ctx, 1));
        Function *NK = Function::Create(FunctionType::get(FT->getReturnType(), params, false), K->getLinkage(),
                                        K->getAddressSpace(), "", Mod);
        NK->copyAttributesFrom(K);
        NK->splice(NK->begin(), K);
        for (auto [Old, New] : zip(K->args(), NK->args()))
        {
            New.takeName(&Old);
            Old.replaceAllUsesWith(&New);
        }
        Argument *Base = NK->getArg(NK->arg_size() - 1);
        Base->setName("zluda_globals");
        NK->addParamAttr(Base->getArgNo(), Attribute::get(Ctx, "zluda-globals"));
        NK->addParamAttr(Base->getArgNo(), Attribute::NoAlias);
        NK->takeName(K);
        K->replaceAllUsesWith(NK);
        K->eraseFromParent();
        // Each use of a variable in NK becomes the buffer plus its offset.
        convertUsersOfConstantsToInstructions(consts, NK);
        IRBuilder<> B(&*NK->getEntryBlock().getFirstInsertionPt());
        for (GlobalVariable *GV : globals)
        {
            bool used = any_of(GV->users(), [&](User *U) {
                auto *I = dyn_cast<Instruction>(U);
                return I && I->getFunction() == NK;
            });
            if (used)
                zludaRetarget(GV, B.CreateConstInBoundsGEP1_64(B.getInt8Ty(), Base, img.offsets[GV], GV->getName()),
                              NK);
        }
        // Pointers held in the buffer are loaded and stored as i64: Metal's
        // typed-pointer reader rejects a pointer to a pointer ("Unexpected
        // bitcode file", the sregs clock emulation's timer address).
        SmallVector<Value *, 16> work{Base};
        SmallPtrSet<Value *, 16> seen;
        while (!work.empty())
        {
            Value *P = work.pop_back_val();
            if (!seen.insert(P).second)
                continue;
            for (User *U : make_early_inc_range(P->users()))
            {
                if (isa<GetElementPtrInst>(U) || isa<BitCastInst>(U) || isa<PHINode>(U) || isa<SelectInst>(U))
                {
                    if (U->getType()->isPointerTy())
                        work.push_back(U);
                    continue;
                }
                if (auto *L = dyn_cast<LoadInst>(U); L && L->getType()->isPointerTy() && !L->isAtomic())
                {
                    IRBuilder<> LB(L);
                    LoadInst *I = LB.CreateAlignedLoad(LB.getInt64Ty(), P, L->getAlign(), L->isVolatile());
                    Value *R = LB.CreateIntToPtr(I, L->getType());
                    R->takeName(L);
                    L->replaceAllUsesWith(R);
                    L->eraseFromParent();
                }
                else if (auto *S = dyn_cast<StoreInst>(U);
                         S && S->getPointerOperand() == P && S->getValueOperand()->getType()->isPointerTy() &&
                         !S->isAtomic())
                {
                    IRBuilder<> SB(S);
                    SB.CreateAlignedStore(SB.CreatePtrToInt(S->getValueOperand(), SB.getInt64Ty()), P,
                                          S->getAlign(), S->isVolatile());
                    S->eraseFromParent();
                }
            }
        }
        if (!kernels.empty())
            kernels += ", ";
        kernels += "\"" + NK->getName().str() + "\"";
    }

    // The init kernel: each mask() byte group from the buffer's address.
    std::string init;
    if (!masked.empty())
    {
        Function *K = nullptr;
        for (Function &F : *Mod)
            if (F.hasFnAttribute("air-kernel"))
                K = &F;
        Function *IF = Function::Create(FunctionType::get(Type::getVoidTy(Ctx), {PointerType::get(Ctx, 1)}, false),
                                        GlobalValue::ExternalLinkage, "__zluda_air_init", Mod);
        if (K)
            IF->setAttributes(AttributeList::get(Ctx, K->getAttributes().getFnAttrs(), {}, {}));
        IF->addFnAttr("air-kernel");
        Argument *G = IF->getArg(0);
        G->setName("zluda_globals");
        IF->addParamAttr(0, Attribute::get(Ctx, "zluda-globals"));
        IRBuilder<> B(BasicBlock::Create(Ctx, "", IF));
        Value *Addr = B.CreatePtrToInt(G, B.getInt64Ty());
        for (auto &m : masked)
        {
            Value *V = B.CreateLShr(B.CreateAdd(Addr, B.getInt64(m[1])), m[2]);
            Type *ET = B.getIntNTy(8 * m[3]);
            B.CreateStore(B.CreateTrunc(V, ET), B.CreateConstInBoundsGEP1_64(B.getInt8Ty(), G, m[0]));
        }
        B.CreateRetVoid();
        init = IF->getName().str();
    }

    std::string out = "{\"size\": " + std::to_string(size) + ", \"image\": \"" + zludaHex(img.image) +
                      "\", \"relocs\": [";
    for (size_t i = 0; i < img.relocs.size(); i++)
        out += std::string(i ? ", [" : "[") + std::to_string(img.relocs[i].first) + ", " +
               std::to_string(img.relocs[i].second) + "]";
    out += "], \"symbols\": {";
    for (size_t i = 0; i < globals.size(); i++)
        out += std::string(i ? ", \"" : "\"") + globals[i]->getName().str() + "\": " +
               std::to_string(img.offsets[globals[i]]);
    out += "}, \"kernels\": [" + kernels + "], \"masked\": [";
    for (size_t i = 0; i < masked.size(); i++)
        out += std::string(i ? ", [" : "[") + std::to_string(masked[i][0]) + ", " + std::to_string(masked[i][1]) +
               ", " + std::to_string(masked[i][2]) + ", " + std::to_string(masked[i][3]) + "]";
    out += "], \"init_kernel\": \"" + init + "\"}";

    // (their initializers, now in the image, may refer to each other)
    for (GlobalVariable *GV : globals)
        GV->setInitializer(nullptr);
    for (GlobalVariable *GV : globals)
    {
        GV->removeDeadConstantUsers();
        if (!GV->use_empty())
            return fail(GV->getName().str() + " is used outside the kernels (in another variable's initializer?)");
        GV->eraseFromParent();
    }
    *json = LLVMCreateMessage(out.c_str());
    return 1;
}

LLVM_C_EXTERN_C_END

LLVM_C_EXTERN_C_BEGIN

// ZLUDA_AGX_VIA_AIR, the .air for Metal: the shared memory registers in
// Metal's numbering of threadgroup addresses. AGX numbers shared memory from
// 0 (static variables in module order, then the dynamic shared memory), so
// zluda_ptx_impl_agx.ll computes sizes and offsets alike. Metal numbers its
// threadgroup allocations its own way (on an M2 Max static ones from 0x10000,
// reordered, and dynamic threadgroup memory from 0x80000000 after the static
// bytes), so in the .air:
// - ptrtoint(__zluda_agx_smem_end), the end of the static variables, is their
//   size in AGX's layout (%dynamic_smem_size is the launch's bytes minus it);
// - the bias added to the reserved region's offsets places it after the
//   highest threadgroup address the kernel uses (its static variables and its
//   dynamic shared memory), rounded to 128 bytes: every relation PTX states
//   between shared addresses and %reserved_smem_offset_* holds. Sizes
//   (%total_smem_size, %dynamic_smem_size, %aggr_smem_size) are unchanged.
// Returns 1 if the module changed.
int LLVMZludaAirSharedLayout(LLVMModuleRef M)
{
    Module *Mod = unwrap(M);
    const DataLayout &DL = Mod->getDataLayout();
    LLVMContext &Ctx = Mod->getContext();
    GlobalVariable *End = Mod->getGlobalVariable("__zluda_agx_smem_end", true);
    GlobalVariable *Bias = Mod->getGlobalVariable("__zluda_agx_reserved_smem_bias", true);
    GlobalVariable *Bytes = Mod->getGlobalVariable("__zluda_agx_smem_bytes", true);
    if (!End && !Bias)
        return 0;
    // The static variables in AGX's layout, and the dynamic shared memory.
    uint64_t statics = 0;
    SmallVector<std::pair<GlobalVariable *, uint64_t>, 8> sized;
    SmallVector<GlobalVariable *, 2> dynamic;
    for (GlobalVariable &GV : Mod->globals())
    {
        if (GV.getAddressSpace() != 3 || &GV == End)
            continue;
        uint64_t size = DL.getTypeAllocSize(GV.getValueType());
        if (size)
        {
            statics = alignTo(statics, DL.getValueOrABITypeAlignment(GV.getAlign(), GV.getValueType())) + size;
            sized.push_back({&GV, size});
        }
        else
            dynamic.push_back(&GV);
    }
    bool changed = false;
    if (End)
    {
        convertUsersOfConstantsToInstructions({End});
        for (User *U : make_early_inc_range(End->users()))
            if (auto *P = dyn_cast<PtrToIntInst>(U))
            {
                P->replaceAllUsesWith(ConstantInt::get(P->getType(), statics));
                P->eraseFromParent();
                changed = true;
            }
        End->removeDeadConstantUsers();
        if (End->use_empty())
            End->eraseFromParent();
    }
    if (Bias)
    {
        for (User *U : make_early_inc_range(Bias->users()))
        {
            auto *L = dyn_cast<LoadInst>(U);
            if (!L)
                continue;
            IRBuilder<> B(L);
            Type *I64 = B.getInt64Ty();
            Value *Total = Bytes ? B.CreateZExt(B.CreateLoad(B.getInt32Ty(), Bytes), I64)
                                 : static_cast<Value *>(B.getInt64(statics));
            Value *Top = B.getInt64(0);
            auto Max = [&](Value *E) { Top = B.CreateBinaryIntrinsic(Intrinsic::umax, Top, E); };
            for (auto [GV, size] : sized)
                Max(B.CreateAdd(B.CreatePtrToInt(GV, I64), B.getInt64(size)));
            for (GlobalVariable *GV : dynamic)
                Max(B.CreateAdd(B.CreatePtrToInt(GV, I64), B.CreateSub(Total, B.getInt64(statics))));
            auto Round = [&](Value *V) { return B.CreateAnd(B.CreateAdd(V, B.getInt64(127)), B.getInt64(~127ull)); };
            Value *R = B.CreateTrunc(B.CreateSub(Round(Top), Round(Total)), L->getType());
            L->replaceAllUsesWith(R);
            L->eraseFromParent();
            changed = true;
        }
        if (Bias->use_empty())
            Bias->eraseFromParent();
    }
    (void)Ctx;
    return changed ? 1 : 0;
}

LLVM_C_EXTERN_C_END

LLVM_C_EXTERN_C_BEGIN

// ZLUDA_AGX_VIA_AIR, the .air for Metal: function pointers without Metal's
// function pointers. AIR has no call stack and the AIR target inlines every
// call, so an indirect call becomes a choice among the functions it can call:
// those of its type whose address the module takes, which are all a CUDA
// function pointer can hold (ZLUDA compiles the whole linked program as one
// module). A function's address is a small integer ID (1, 2, ...; 0 stays
// null), wherever it appears: in instructions and in variable initializers
// (which the globals buffer then holds as numbers). An indirect call compares
// the callee's ID with each candidate's and calls it directly; with none of
// them matching (an invalid pointer, undefined in CUDA) it calls the last.
// Returns -1 with *error if a call has no candidate, else 1 if the module
// changed.
int LLVMZludaAirFunctionPointers(LLVMModuleRef M, char **error)
{
    Module *Mod = unwrap(M);
    LLVMContext &Ctx = Mod->getContext();
    Type *I64 = Type::getInt64Ty(Ctx);
    auto isCallee = [](const Use &U) {
        auto *CB = dyn_cast<CallBase>(U.getUser());
        return CB && CB->isCallee(&U);
    };
    SmallVector<Function *, 8> taken;
    for (Function &F : *Mod)
        if (!F.isDeclaration() && !F.isIntrinsic() &&
            any_of(F.uses(), [&](const Use &U) { return !isCallee(U) && !isa<BlockAddress>(U.getUser()); }))
            taken.push_back(&F);
    SmallVector<CallBase *, 8> indirect;
    for (Function &F : *Mod)
        for (Instruction &I : instructions(F))
            if (auto *CB = dyn_cast<CallBase>(&I); CB && CB->isIndirectCall() && !CB->isInlineAsm())
                indirect.push_back(CB);
    if (taken.empty() && indirect.empty())
        return 0;

    // Addresses as IDs.
    DenseMap<Function *, uint64_t> ids;
    for (size_t i = 0; i < taken.size(); i++)
    {
        Function *F = taken[i];
        uint64_t id = i + 1;
        ids[F] = id;
        Constant *IdPtr = ConstantExpr::getIntToPtr(ConstantInt::get(I64, id), F->getType());
        SmallVector<Use *, 8> uses;
        for (Use &U : F->uses())
            if (!isCallee(U) && !isa<BlockAddress>(U.getUser()) && !isa<GlobalValue>(U.getUser()))
                uses.push_back(&U);
        for (Use *U : uses)
        {
            if (auto *C = dyn_cast<Constant>(U->getUser()))
                C->handleOperandChange(F, IdPtr);
            else
                U->set(IdPtr);
        }
    }
    // llvm.used and the like may still list a function; they are not addresses the program sees.

    // Indirect calls as a choice among direct ones.
    for (CallBase *CB : indirect)
    {
        auto *CI = dyn_cast<CallInst>(CB);
        if (!CI)
        {
            *error = LLVMCreateMessage("an indirect invoke has no Metal form");
            return -1;
        }
        FunctionType *FT = CI->getFunctionType();
        SmallVector<Function *, 8> cands;
        for (Function *F : taken)
            if (F->getFunctionType() == FT)
                cands.push_back(F);
        if (cands.empty())
        {
            std::string s;
            raw_string_ostream os(s);
            os << "an indirect call of type " << *FT << " in " << CI->getFunction()->getName()
               << " has no function it can call";
            *error = LLVMCreateMessage(os.str().c_str());
            return -1;
        }
        BasicBlock *Head = CI->getParent();
        BasicBlock *Tail = SplitBlock(Head, CI);
        Head->getTerminator()->eraseFromParent();
        IRBuilder<> B(Head);
        Value *Id = B.CreatePtrToInt(CI->getCalledOperand(), I64);
        Function *Fn = Head->getParent();
        PHINode *Phi = nullptr;
        if (!CI->getType()->isVoidTy())
        {
            Phi = PHINode::Create(CI->getType(), cands.size(), "", Tail->begin());
            CI->replaceAllUsesWith(Phi);
        }
        SmallVector<Value *, 8> args(CI->args());
        BasicBlock *Test = Head;
        for (size_t i = 0; i < cands.size(); i++)
        {
            BasicBlock *Arm = BasicBlock::Create(Ctx, "indirect." + cands[i]->getName(), Fn, Tail);
            IRBuilder<> AB(Arm);
            CallInst *Direct = AB.CreateCall(FT, cands[i], args);
            Direct->setCallingConv(cands[i]->getCallingConv());
            Direct->setAttributes(CI->getAttributes());
            AB.CreateBr(Tail);
            if (Phi)
                Phi->addIncoming(Direct, Arm);
            IRBuilder<> TB(Test);
            if (i + 1 == cands.size())
                TB.CreateBr(Arm);
            else
            {
                BasicBlock *Next = BasicBlock::Create(Ctx, "indirect.next", Fn, Tail);
                TB.CreateCondBr(TB.CreateICmpEQ(Id, TB.getInt64(ids[cands[i]])), Arm, Next);
                Test = Next;
            }
        }
        CI->eraseFromParent();
    }
    return 1;
}

LLVM_C_EXTERN_C_END
