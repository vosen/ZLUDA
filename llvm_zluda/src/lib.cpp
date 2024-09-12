#include <llvm-c/Core.h>
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/Type.h"

LLVM_C_EXTERN_C_BEGIN

LLVMValueRef LLVMZludaBuildAlloca(LLVMBuilderRef B, LLVMTypeRef Ty, unsigned AddrSpace,
                                  const char *Name)
{
    return llvm::wrap(llvm::unwrap(B)->CreateAlloca(llvm::unwrap(Ty), AddrSpace, nullptr, Name));
}

LLVM_C_EXTERN_C_END