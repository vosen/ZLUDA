//! The LLVM intermediate representation.

use super::*;

// Core
extern "C" {
    pub fn LLVMShutdown();
    pub fn LLVMCreateMessage(Message: *const ::libc::c_char) -> *mut ::libc::c_char;
    pub fn LLVMDisposeMessage(Message: *mut ::libc::c_char);
}

// Core->Contexts
extern "C" {
    pub fn LLVMContextCreate() -> LLVMContextRef;
    //pub fn LLVMGetGlobalContext() -> LLVMContextRef;
    pub fn LLVMContextSetDiagnosticHandler(
        C: LLVMContextRef,
        Handler: LLVMDiagnosticHandler,
        DiagnosticContext: *mut ::libc::c_void,
    );
    /// Get the diagnostic handler of this context.
    pub fn LLVMContextGetDiagnosticHandler(C: LLVMContextRef) -> LLVMDiagnosticHandler;
    /// Get the diagnostic context of this context.
    pub fn LLVMContextGetDiagnosticContext(C: LLVMContextRef) -> *mut ::libc::c_void;
    pub fn LLVMContextSetYieldCallback(
        C: LLVMContextRef,
        Callback: LLVMYieldCallback,
        OpaqueHandle: *mut ::libc::c_void,
    );
    pub fn LLVMContextShouldDiscardValueNames(C: LLVMContextRef) -> LLVMBool;
    pub fn LLVMContextSetDiscardValueNames(C: LLVMContextRef, Discard: LLVMBool);
    /// Set whether the given context is in opaque pointer mode.
    pub fn LLVMContextSetOpaquePointers(C: LLVMContextRef, OpaquePointers: LLVMBool);
    pub fn LLVMContextDispose(C: LLVMContextRef);
    pub fn LLVMGetDiagInfoDescription(DI: LLVMDiagnosticInfoRef) -> *mut ::libc::c_char;
    pub fn LLVMGetDiagInfoSeverity(DI: LLVMDiagnosticInfoRef) -> LLVMDiagnosticSeverity;
    pub fn LLVMGetMDKindIDInContext(
        C: LLVMContextRef,
        Name: *const ::libc::c_char,
        SLen: ::libc::c_uint,
    ) -> ::libc::c_uint;
    //pub fn LLVMGetMDKindID(Name: *const ::libc::c_char, SLen: ::libc::c_uint) -> ::libc::c_uint;

    /// Return a unique id given the name of an enum attribute, or 0 if no attribute
    /// by that name exists.
    ///
    /// See <http://llvm.org/docs/LangRef.html#parameter-attributes>
    /// and <http://llvm.org/docs/LangRef.html#function-attributes>
    /// for the list of available attributes.
    ///
    /// Note that attribute names and IDs are not subject to the same stability
    /// guarantees as this API.
    pub fn LLVMGetEnumAttributeKindForName(
        Name: *const ::libc::c_char,
        SLen: ::libc::size_t,
    ) -> ::libc::c_uint;
    pub fn LLVMGetLastEnumAttributeKind() -> ::libc::c_uint;

    /// Create an enum attribute.
    pub fn LLVMCreateEnumAttribute(
        C: LLVMContextRef,
        KindID: ::libc::c_uint,
        Val: u64,
    ) -> LLVMAttributeRef;
    /// Get the unique id corresponding to the provided enum attribute.
    pub fn LLVMGetEnumAttributeKind(A: LLVMAttributeRef) -> ::libc::c_uint;
    /// Get the value of an enum attribute.
    ///
    /// Returns 0 if none exists.
    pub fn LLVMGetEnumAttributeValue(A: LLVMAttributeRef) -> u64;

    /// Create a type attribute.
    pub fn LLVMCreateTypeAttribute(
        C: LLVMContextRef,
        KindID: ::libc::c_uint,
        type_ref: LLVMTypeRef,
    ) -> LLVMAttributeRef;
    /// Get the type attribute's value.
    pub fn LLVMGetTypeAttributeValue(A: LLVMAttributeRef) -> LLVMTypeRef;

    /// Create a string attribute.
    pub fn LLVMCreateStringAttribute(
        C: LLVMContextRef,
        K: *const ::libc::c_char,
        KLength: ::libc::c_uint,
        V: *const ::libc::c_char,
        VLength: ::libc::c_uint,
    ) -> LLVMAttributeRef;
    /// Get a string attribute's kind.
    pub fn LLVMGetStringAttributeKind(
        A: LLVMAttributeRef,
        Length: *mut ::libc::c_uint,
    ) -> *const ::libc::c_char;
    /// Get a string attribute's value.
    pub fn LLVMGetStringAttributeValue(
        A: LLVMAttributeRef,
        Length: *mut ::libc::c_uint,
    ) -> *const ::libc::c_char;
    pub fn LLVMIsEnumAttribute(A: LLVMAttributeRef) -> LLVMBool;
    pub fn LLVMIsStringAttribute(A: LLVMAttributeRef) -> LLVMBool;
    pub fn LLVMIsTypeAttribute(A: LLVMAttributeRef) -> LLVMBool;

    /// Obtain a Type from a context by its registered name.
    pub fn LLVMGetTypeByName2(C: LLVMContextRef, Name: *const ::libc::c_char) -> LLVMTypeRef;
}

// Core->Modules
extern "C" {
    //pub fn LLVMModuleCreateWithName(ModuleID: *const ::libc::c_char) -> LLVMModuleRef;
    pub fn LLVMModuleCreateWithNameInContext(
        ModuleID: *const ::libc::c_char,
        C: LLVMContextRef,
    ) -> LLVMModuleRef;
    pub fn LLVMCloneModule(M: LLVMModuleRef) -> LLVMModuleRef;
    pub fn LLVMDisposeModule(M: LLVMModuleRef);
    /// Get the identifier of a module.
    ///
    /// `Len` is written to contains the length of the returned string.
    pub fn LLVMGetModuleIdentifier(
        M: LLVMModuleRef,
        Len: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    /// Set the identifier of a module.
    ///
    /// `Len` is the length of the string pointed to by `Ident`.
    pub fn LLVMSetModuleIdentifier(
        M: LLVMModuleRef,
        Ident: *const ::libc::c_char,
        Len: ::libc::size_t,
    );

    /// Obtain the module's original source file name.
    ///
    /// Len holds the length of the returned string, returns the original source file name of M.
    pub fn LLVMGetSourceFileName(
        M: LLVMModuleRef,
        Len: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    /// Set the original source file name of a module to a string Name with length Len.
    pub fn LLVMSetSourceFileName(
        M: LLVMModuleRef,
        Name: *const ::libc::c_char,
        Len: ::libc::size_t,
    );

    #[deprecated(since = "3.9", note = "Confusingly named. Use LLVMGetDataLayoutStr.")]
    pub fn LLVMGetDataLayout(M: LLVMModuleRef) -> *const ::libc::c_char;
    /// Obtain the data layout for a module.
    pub fn LLVMGetDataLayoutStr(M: LLVMModuleRef) -> *const ::libc::c_char;
    pub fn LLVMSetDataLayout(M: LLVMModuleRef, DataLayoutStr: *const ::libc::c_char);
    pub fn LLVMGetTarget(M: LLVMModuleRef) -> *const ::libc::c_char;
    pub fn LLVMSetTarget(M: LLVMModuleRef, Triple: *const ::libc::c_char);

    /// Returns the module flags as an array of flag-key-value triples.  The caller is responsible for freeing this array by calling LLVMDisposeModuleFlagsMetadata.
    pub fn LLVMCopyModuleFlagsMetadata(
        M: LLVMModuleRef,
        Len: *mut ::libc::size_t,
    ) -> *mut LLVMModuleFlagEntry;
    /// Destroys module flags metadata entries.
    pub fn LLVMDisposeModuleFlagsMetadata(Entries: *mut LLVMModuleFlagEntry);
    /// Returns the flag behavior for a module flag entry at a specific index.
    pub fn LLVMModuleFlagEntriesGetFlagBehavior(
        Entries: *mut LLVMModuleFlagEntry,
        Index: ::libc::c_uint,
    ) -> LLVMModuleFlagBehavior;
    /// Returns the key for a module flag entry at a specific index.
    pub fn LLVMModuleFlagEntriesGetKey(
        Entries: *mut LLVMModuleFlagEntry,
        Index: ::libc::c_uint,
        Len: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    /// Returns the metadata for a module flag entry at a specific index.
    pub fn LLVMModuleFlagEntriesGetMetadata(
        Entries: *mut LLVMModuleFlagEntry,
        Index: ::libc::c_uint,
    ) -> LLVMMetadataRef;
    /// Add a module-level flag to the module-level flags metadata if it doesn't already exist.
    pub fn LLVMGetModuleFlag(
        M: LLVMModuleRef,
        Key: *const ::libc::c_char,
        KeyLen: ::libc::size_t,
    ) -> LLVMMetadataRef;
    /// Add a module-level flag to the module-level flags metadata if it doesn't already exist.
    pub fn LLVMAddModuleFlag(
        M: LLVMModuleRef,
        Behavior: LLVMModuleFlagBehavior,
        Key: *const ::libc::c_char,
        KeyLen: ::libc::size_t,
        Val: LLVMMetadataRef,
    );

    pub fn LLVMDumpModule(M: LLVMModuleRef);
    pub fn LLVMPrintModuleToFile(
        M: LLVMModuleRef,
        Filename: *const ::libc::c_char,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMPrintModuleToString(M: LLVMModuleRef) -> *mut ::libc::c_char;

    pub fn LLVMGetModuleInlineAsm(
        M: LLVMModuleRef,
        Len: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    #[deprecated(since = "7.0", note = "Use LLVMSetModuleInlineAsm2 instead")]
    pub fn LLVMSetModuleInlineAsm(M: LLVMModuleRef, Asm: *const ::libc::c_char);
    pub fn LLVMSetModuleInlineAsm2(
        M: LLVMModuleRef,
        Asm: *const ::libc::c_char,
        Len: ::libc::size_t,
    );
    pub fn LLVMAppendModuleInlineAsm(
        M: LLVMModuleRef,
        Asm: *const ::libc::c_char,
        Len: ::libc::size_t,
    );
    pub fn LLVMGetInlineAsm(
        Ty: LLVMTypeRef,
        AsmString: *mut ::libc::c_char,
        AsmStringSize: ::libc::size_t,
        Constraints: *mut ::libc::c_char,
        ConstraintsSize: ::libc::size_t,
        HasSideEffects: LLVMBool,
        IsAlignStack: LLVMBool,
        Dialect: LLVMInlineAsmDialect,
        CanThrow: LLVMBool,
    ) -> LLVMValueRef;

    pub fn LLVMGetModuleContext(M: LLVMModuleRef) -> LLVMContextRef;
    #[deprecated(since = "12.0.0", note = "Use LLVMGetTypeByName2 instead")]
    pub fn LLVMGetTypeByName(M: LLVMModuleRef, Name: *const ::libc::c_char) -> LLVMTypeRef;
    pub fn LLVMGetFirstNamedMetadata(M: LLVMModuleRef) -> LLVMNamedMDNodeRef;
    pub fn LLVMGetLastNamedMetadata(M: LLVMModuleRef) -> LLVMNamedMDNodeRef;
    pub fn LLVMGetNextNamedMetadata(NamedMDNode: LLVMNamedMDNodeRef) -> LLVMNamedMDNodeRef;
    pub fn LLVMGetPreviousNamedMetadata(NamedMDNode: LLVMNamedMDNodeRef) -> LLVMNamedMDNodeRef;
    pub fn LLVMGetNamedMetadata(
        M: LLVMModuleRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
    ) -> LLVMNamedMDNodeRef;
    pub fn LLVMGetOrInsertNamedMetadata(
        M: LLVMModuleRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
    ) -> LLVMNamedMDNodeRef;
    pub fn LLVMGetNamedMetadataName(
        NamedMD: LLVMNamedMDNodeRef,
        NameLen: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    pub fn LLVMGetNamedMetadataNumOperands(
        M: LLVMModuleRef,
        name: *const ::libc::c_char,
    ) -> ::libc::c_uint;
    pub fn LLVMGetNamedMetadataOperands(
        M: LLVMModuleRef,
        name: *const ::libc::c_char,
        Dest: *mut LLVMValueRef,
    );
    pub fn LLVMAddNamedMetadataOperand(
        M: LLVMModuleRef,
        name: *const ::libc::c_char,
        Val: LLVMValueRef,
    );
    pub fn LLVMGetDebugLocDirectory(
        Val: LLVMValueRef,
        Length: *mut ::libc::c_uint,
    ) -> *const ::libc::c_char;
    pub fn LLVMGetDebugLocFilename(
        Val: LLVMValueRef,
        Length: *mut ::libc::c_uint,
    ) -> *const ::libc::c_char;
    pub fn LLVMGetDebugLocLine(Val: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetDebugLocColumn(Val: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMAddFunction(
        M: LLVMModuleRef,
        Name: *const ::libc::c_char,
        FunctionTy: LLVMTypeRef,
    ) -> LLVMValueRef;
    pub fn LLVMGetNamedFunction(M: LLVMModuleRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMGetFirstFunction(M: LLVMModuleRef) -> LLVMValueRef;
    pub fn LLVMGetLastFunction(M: LLVMModuleRef) -> LLVMValueRef;
    pub fn LLVMGetNextFunction(Fn: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetPreviousFunction(Fn: LLVMValueRef) -> LLVMValueRef;
}

// Core->Types
extern "C" {
    pub fn LLVMGetTypeKind(Ty: LLVMTypeRef) -> LLVMTypeKind;
    pub fn LLVMTypeIsSized(Ty: LLVMTypeRef) -> LLVMBool;
    pub fn LLVMGetTypeContext(Ty: LLVMTypeRef) -> LLVMContextRef;
    pub fn LLVMDumpType(Val: LLVMTypeRef);
    pub fn LLVMPrintTypeToString(Val: LLVMTypeRef) -> *mut ::libc::c_char;

    // Core->Types->Integer
    pub fn LLVMInt1TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMInt8TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMInt16TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMInt32TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMInt64TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMInt128TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMIntTypeInContext(C: LLVMContextRef, NumBits: ::libc::c_uint) -> LLVMTypeRef;
    //pub fn LLVMInt1Type() -> LLVMTypeRef;
    //pub fn LLVMInt8Type() -> LLVMTypeRef;
    //pub fn LLVMInt16Type() -> LLVMTypeRef;
    //pub fn LLVMInt32Type() -> LLVMTypeRef;
    //pub fn LLVMInt64Type() -> LLVMTypeRef;
    //pub fn LLVMInt128Type() -> LLVMTypeRef;
    //pub fn LLVMIntType(NumBits: ::libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMGetIntTypeWidth(IntegerTy: LLVMTypeRef) -> ::libc::c_uint;

    // Core->Types->Floating-Point
    pub fn LLVMHalfTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMBFloatTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMFloatTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMDoubleTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMX86FP80TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMFP128TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMPPCFP128TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    //pub fn LLVMHalfType() -> LLVMTypeRef;
    //pub fn LLVMBFloatType() -> LLVMTypeRef;
    //pub fn LLVMFloatType() -> LLVMTypeRef;
    //pub fn LLVMDoubleType() -> LLVMTypeRef;
    //pub fn LLVMX86FP80Type() -> LLVMTypeRef;
    //pub fn LLVMFP128Type() -> LLVMTypeRef;
    //pub fn LLVMPPCFP128Type() -> LLVMTypeRef;

    // Core->Types->Function
    pub fn LLVMFunctionType(
        ReturnType: LLVMTypeRef,
        ParamTypes: *mut LLVMTypeRef,
        ParamCount: ::libc::c_uint,
        IsVarArg: LLVMBool,
    ) -> LLVMTypeRef;
    pub fn LLVMIsFunctionVarArg(FunctionTy: LLVMTypeRef) -> LLVMBool;
    pub fn LLVMGetReturnType(FunctionTy: LLVMTypeRef) -> LLVMTypeRef;
    pub fn LLVMCountParamTypes(FunctionTy: LLVMTypeRef) -> ::libc::c_uint;
    pub fn LLVMGetParamTypes(FunctionTy: LLVMTypeRef, Dest: *mut LLVMTypeRef);

    // Core->Types->Struct
    pub fn LLVMStructTypeInContext(
        C: LLVMContextRef,
        ElementTypes: *mut LLVMTypeRef,
        ElementCount: ::libc::c_uint,
        Packed: LLVMBool,
    ) -> LLVMTypeRef;
    //pub fn LLVMStructType(
    //    ElementTypes: *mut LLVMTypeRef,
    //    ElementCount: ::libc::c_uint,
    //    Packed: LLVMBool,
    //) -> LLVMTypeRef;
    pub fn LLVMStructCreateNamed(C: LLVMContextRef, Name: *const ::libc::c_char) -> LLVMTypeRef;
    pub fn LLVMGetStructName(Ty: LLVMTypeRef) -> *const ::libc::c_char;
    pub fn LLVMStructSetBody(
        StructTy: LLVMTypeRef,
        ElementTypes: *mut LLVMTypeRef,
        ElementCount: ::libc::c_uint,
        Packed: LLVMBool,
    );
    pub fn LLVMCountStructElementTypes(StructTy: LLVMTypeRef) -> ::libc::c_uint;
    pub fn LLVMGetStructElementTypes(StructTy: LLVMTypeRef, Dest: *mut LLVMTypeRef);
    /// Get the type of the element at the given index in a structure.
    ///
    /// Added in LLVM 3.7.
    pub fn LLVMStructGetTypeAtIndex(StructTy: LLVMTypeRef, i: ::libc::c_uint) -> LLVMTypeRef;
    /// Determine whether a structure is packed.
    pub fn LLVMIsPackedStruct(StructTy: LLVMTypeRef) -> LLVMBool;
    pub fn LLVMIsOpaqueStruct(StructTy: LLVMTypeRef) -> LLVMBool;
    pub fn LLVMIsLiteralStruct(StructTy: LLVMTypeRef) -> LLVMBool;

    // Core->Types->Sequential
    pub fn LLVMGetElementType(Ty: LLVMTypeRef) -> LLVMTypeRef;
    /// Get the subtypes of the given type.
    pub fn LLVMGetSubtypes(Tp: LLVMTypeRef, Arr: *mut LLVMTypeRef);
    /// Return the number of types in the derived type.
    pub fn LLVMGetNumContainedTypes(Tp: LLVMTypeRef) -> ::libc::c_uint;
    pub fn LLVMArrayType(ElementType: LLVMTypeRef, ElementCount: ::libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMGetArrayLength(ArrayTy: LLVMTypeRef) -> ::libc::c_uint;
    pub fn LLVMPointerType(ElementType: LLVMTypeRef, AddressSpace: ::libc::c_uint) -> LLVMTypeRef;
    /// Determine whether a pointer is opaque.
    ///
    /// True if this is an instance of an opaque PointerType.
    pub fn LLVMPointerTypeIsOpaque(Ty: LLVMTypeRef) -> LLVMBool;
    /// Create an opaque pointer type in a context.
    pub fn LLVMPointerTypeInContext(C: LLVMContextRef, AddressSpace: ::libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMGetPointerAddressSpace(PointerTy: LLVMTypeRef) -> ::libc::c_uint;
    pub fn LLVMVectorType(ElementType: LLVMTypeRef, ElementCount: ::libc::c_uint) -> LLVMTypeRef;
    /// Create a vector type that contains a defined type and has a scalable
    /// number of elements.
    ///
    /// The created type will exist in the context that its element type
    /// exists in.
    pub fn LLVMScalableVectorType(
        ElementType: LLVMTypeRef,
        ElementCount: ::libc::c_uint,
    ) -> LLVMTypeRef;
    /// Obtain the (possibly scalable) number of elements in a vector type.
    pub fn LLVMGetVectorSize(VectorTy: LLVMTypeRef) -> ::libc::c_uint;

    // Core->Types->Other
    pub fn LLVMVoidTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMLabelTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMX86MMXTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMX86AMXTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMTokenTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMMetadataTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    //pub fn LLVMVoidType() -> LLVMTypeRef;
    //pub fn LLVMLabelType() -> LLVMTypeRef;
    //pub fn LLVMX86MMXType() -> LLVMTypeRef;
    //pub fn LLVMX86AMXType() -> LLVMTypeRef;
}

// Core->Values
extern "C" {
    // Core->Values->General
    // Get the enumerated kind of a Value instance.
    pub fn LLVMGetValueKind(Val: LLVMValueRef) -> LLVMValueKind;
    pub fn LLVMTypeOf(Val: LLVMValueRef) -> LLVMTypeRef;

    #[deprecated(since = "7.0", note = "Use LLVMGetValueName2 instead")]
    pub fn LLVMGetValueName(Val: LLVMValueRef) -> *const ::libc::c_char;
    pub fn LLVMGetValueName2(
        Val: LLVMValueRef,
        Length: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    #[deprecated(since = "7.0", note = "Use LLVMSetValueName2 instead")]
    pub fn LLVMSetValueName(Val: LLVMValueRef, Name: *const ::libc::c_char);
    pub fn LLVMSetValueName2(
        Val: LLVMValueRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
    );

    pub fn LLVMDumpValue(Val: LLVMValueRef);
    pub fn LLVMPrintValueToString(Val: LLVMValueRef) -> *mut ::libc::c_char;
    pub fn LLVMReplaceAllUsesWith(OldVal: LLVMValueRef, NewVal: LLVMValueRef);
    /// Determine whether the specified value instance is constant.
    pub fn LLVMIsConstant(Val: LLVMValueRef) -> LLVMBool;
    pub fn LLVMIsUndef(Val: LLVMValueRef) -> LLVMBool;
    /// Determine whether a value instance is poisonous.
    pub fn LLVMIsPoison(Val: LLVMValueRef) -> LLVMBool;
    pub fn LLVMIsAMDNode(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMDString(Val: LLVMValueRef) -> LLVMValueRef;

    // Core->Values->Usage
    pub fn LLVMGetFirstUse(Val: LLVMValueRef) -> LLVMUseRef;
    pub fn LLVMGetNextUse(U: LLVMUseRef) -> LLVMUseRef;
    pub fn LLVMGetUser(U: LLVMUseRef) -> LLVMValueRef;
    pub fn LLVMGetUsedValue(U: LLVMUseRef) -> LLVMValueRef;

    // Core->Values->User value
    pub fn LLVMGetOperand(Val: LLVMValueRef, Index: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMGetOperandUse(Val: LLVMValueRef, Index: ::libc::c_uint) -> LLVMUseRef;
    pub fn LLVMSetOperand(User: LLVMValueRef, Index: ::libc::c_uint, Val: LLVMValueRef);
    pub fn LLVMGetNumOperands(Val: LLVMValueRef) -> ::libc::c_int;

    // Core->Values->Constants
    pub fn LLVMConstNull(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstAllOnes(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMGetUndef(Ty: LLVMTypeRef) -> LLVMValueRef;
    /// Obtain a constant value referring to a poison value of a type.
    pub fn LLVMGetPoison(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMIsNull(Val: LLVMValueRef) -> LLVMBool;
    pub fn LLVMConstPointerNull(Ty: LLVMTypeRef) -> LLVMValueRef;

    // Core->Values->Constants->Scalar
    pub fn LLVMConstInt(
        IntTy: LLVMTypeRef,
        N: ::libc::c_ulonglong,
        SignExtend: LLVMBool,
    ) -> LLVMValueRef;
    pub fn LLVMConstIntOfArbitraryPrecision(
        IntTy: LLVMTypeRef,
        NumWords: ::libc::c_uint,
        Words: *const u64,
    ) -> LLVMValueRef;
    pub fn LLVMConstIntOfString(
        IntTy: LLVMTypeRef,
        Text: *const ::libc::c_char,
        Radix: u8,
    ) -> LLVMValueRef;
    pub fn LLVMConstIntOfStringAndSize(
        IntTy: LLVMTypeRef,
        Text: *const ::libc::c_char,
        SLen: ::libc::c_uint,
        Radix: u8,
    ) -> LLVMValueRef;
    pub fn LLVMConstReal(RealTy: LLVMTypeRef, N: ::libc::c_double) -> LLVMValueRef;
    pub fn LLVMConstRealOfString(RealTy: LLVMTypeRef, Text: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMConstRealOfStringAndSize(
        RealTy: LLVMTypeRef,
        Text: *const ::libc::c_char,
        SLen: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMConstIntGetZExtValue(ConstantVal: LLVMValueRef) -> ::libc::c_ulonglong;
    pub fn LLVMConstIntGetSExtValue(ConstantVal: LLVMValueRef) -> ::libc::c_longlong;
    pub fn LLVMConstRealGetDouble(
        ConstantVal: LLVMValueRef,
        losesInfo: *mut LLVMBool,
    ) -> ::libc::c_double;

    // Core->Values->Constants->Composite
    pub fn LLVMConstStringInContext(
        C: LLVMContextRef,
        Str: *const ::libc::c_char,
        Length: ::libc::c_uint,
        DontNullTerminate: LLVMBool,
    ) -> LLVMValueRef;
    //pub fn LLVMConstString(
    //    Str: *const ::libc::c_char,
    //    Length: ::libc::c_uint,
    //    DontNullTerminate: LLVMBool,
    //) -> LLVMValueRef;
    pub fn LLVMIsConstantString(c: LLVMValueRef) -> LLVMBool;
    pub fn LLVMGetAsString(C: LLVMValueRef, Length: *mut ::libc::size_t) -> *const ::libc::c_char;
    pub fn LLVMConstStructInContext(
        C: LLVMContextRef,
        ConstantVals: *mut LLVMValueRef,
        Count: ::libc::c_uint,
        Packed: LLVMBool,
    ) -> LLVMValueRef;
    //pub fn LLVMConstStruct(
    //    ConstantVals: *mut LLVMValueRef,
    //    Count: ::libc::c_uint,
    //    Packed: LLVMBool,
    //) -> LLVMValueRef;
    pub fn LLVMConstArray(
        ElementTy: LLVMTypeRef,
        ConstantVals: *mut LLVMValueRef,
        Length: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMConstNamedStruct(
        StructTy: LLVMTypeRef,
        ConstantVals: *mut LLVMValueRef,
        Count: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMGetAggregateElement(C: LLVMValueRef, idx: ::libc::c_uint) -> LLVMValueRef;
    #[deprecated(since = "15.0", note = "Use LLVMGetAggregateElement instead")]
    pub fn LLVMGetElementAsConstant(C: LLVMValueRef, idx: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMConstVector(
        ScalarConstantVals: *mut LLVMValueRef,
        Size: ::libc::c_uint,
    ) -> LLVMValueRef;

    // Core->Values->Constants->Constant expressions
    pub fn LLVMGetConstOpcode(ConstantVal: LLVMValueRef) -> LLVMOpcode;
    pub fn LLVMAlignOf(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMSizeOf(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstNeg(ConstantVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNSWNeg(ConstantVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNUWNeg(ConstantVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstFNeg(ConstantVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNot(ConstantVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstAdd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNSWAdd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNUWAdd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstSub(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNSWSub(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNUWSub(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstMul(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNSWMul(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNUWMul(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstAnd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstOr(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstXor(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstICmp(
        Predicate: LLVMIntPredicate,
        LHSConstant: LLVMValueRef,
        RHSConstant: LLVMValueRef,
    ) -> LLVMValueRef;
    pub fn LLVMConstFCmp(
        Predicate: LLVMRealPredicate,
        LHSConstant: LLVMValueRef,
        RHSConstant: LLVMValueRef,
    ) -> LLVMValueRef;
    pub fn LLVMConstShl(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstLShr(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstAShr(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    #[deprecated(
        since = "14.0",
        note = "Use LLVMConstGEP2 instead to support opaque pointers."
    )]
    pub fn LLVMConstGEP(
        ConstantVal: LLVMValueRef,
        ConstantIndices: *mut LLVMValueRef,
        NumIndices: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMConstGEP2(
        Ty: LLVMTypeRef,
        ConstantVal: LLVMValueRef,
        ConstantIndices: *mut LLVMValueRef,
        NumIndices: ::libc::c_uint,
    ) -> LLVMValueRef;
    #[deprecated(
        since = "14.0",
        note = "Use LLVMConstInBoundsGEP2 instead to support opaque pointers."
    )]
    pub fn LLVMConstInBoundsGEP(
        ConstantVal: LLVMValueRef,
        ConstantIndices: *mut LLVMValueRef,
        NumIndices: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMConstInBoundsGEP2(
        Ty: LLVMTypeRef,
        ConstantVal: LLVMValueRef,
        ConstantIndices: *mut LLVMValueRef,
        NumIndices: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMConstTrunc(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstSExt(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstZExt(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstFPTrunc(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstFPExt(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstUIToFP(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstSIToFP(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstFPToUI(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstFPToSI(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstPtrToInt(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstIntToPtr(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstBitCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstAddrSpaceCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstZExtOrBitCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstSExtOrBitCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstTruncOrBitCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstPointerCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstIntCast(
        ConstantVal: LLVMValueRef,
        ToType: LLVMTypeRef,
        isSigned: LLVMBool,
    ) -> LLVMValueRef;
    pub fn LLVMConstFPCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstSelect(
        ConstantCondition: LLVMValueRef,
        ConstantIfTrue: LLVMValueRef,
        ConstantIfFalse: LLVMValueRef,
    ) -> LLVMValueRef;
    pub fn LLVMConstExtractElement(
        VectorConstant: LLVMValueRef,
        IndexConstant: LLVMValueRef,
    ) -> LLVMValueRef;
    pub fn LLVMConstInsertElement(
        VectorConstant: LLVMValueRef,
        ElementValueConstant: LLVMValueRef,
        IndexConstant: LLVMValueRef,
    ) -> LLVMValueRef;
    pub fn LLVMConstShuffleVector(
        VectorAConstant: LLVMValueRef,
        VectorBConstant: LLVMValueRef,
        MaskConstant: LLVMValueRef,
    ) -> LLVMValueRef;
    #[deprecated(since = "7.0", note = "Use LLVMGetInlineAsm instead")]
    pub fn LLVMConstInlineAsm(
        Ty: LLVMTypeRef,
        AsmString: *const ::libc::c_char,
        Constraints: *const ::libc::c_char,
        HasSideEffects: LLVMBool,
        IsAlignStack: LLVMBool,
    ) -> LLVMValueRef;
    pub fn LLVMBlockAddress(F: LLVMValueRef, BB: LLVMBasicBlockRef) -> LLVMValueRef;

    // Core->Values->Constants->Global Values
    pub fn LLVMGetGlobalParent(Global: LLVMValueRef) -> LLVMModuleRef;
    pub fn LLVMIsDeclaration(Global: LLVMValueRef) -> LLVMBool;
    pub fn LLVMGetLinkage(Global: LLVMValueRef) -> LLVMLinkage;
    pub fn LLVMSetLinkage(Global: LLVMValueRef, Linkage: LLVMLinkage);
    pub fn LLVMGetSection(Global: LLVMValueRef) -> *const ::libc::c_char;
    pub fn LLVMSetSection(Global: LLVMValueRef, Section: *const ::libc::c_char);
    pub fn LLVMGetVisibility(Global: LLVMValueRef) -> LLVMVisibility;
    pub fn LLVMSetVisibility(Global: LLVMValueRef, Viz: LLVMVisibility);
    pub fn LLVMGetDLLStorageClass(Global: LLVMValueRef) -> LLVMDLLStorageClass;
    pub fn LLVMSetDLLStorageClass(Global: LLVMValueRef, Class: LLVMDLLStorageClass);

    pub fn LLVMGetUnnamedAddress(Global: LLVMValueRef) -> LLVMUnnamedAddr;
    pub fn LLVMSetUnnamedAddress(Global: LLVMValueRef, UnnamedAddr: LLVMUnnamedAddr);
    pub fn LLVMGlobalGetValueType(Global: LLVMValueRef) -> LLVMTypeRef;
    #[deprecated(since = "7.0", note = "Use LLVMGetUnnamedAddress instead")]
    pub fn LLVMHasUnnamedAddr(Global: LLVMValueRef) -> LLVMBool;
    #[deprecated(since = "7.0", note = "Use LLVMSetUnnamedAddress instead")]
    pub fn LLVMSetUnnamedAddr(Global: LLVMValueRef, HasUnnamedAddr: LLVMBool);

    pub fn LLVMGetAlignment(V: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMSetAlignment(V: LLVMValueRef, Bytes: ::libc::c_uint);

    pub fn LLVMGlobalSetMetadata(Global: LLVMValueRef, Kind: ::libc::c_uint, MD: LLVMMetadataRef);
    pub fn LLVMGlobalEraseMetadata(Global: LLVMValueRef, Kind: ::libc::c_uint);
    pub fn LLVMGlobalClearMetadata(Global: LLVMValueRef);
    pub fn LLVMGlobalCopyAllMetadata(
        Value: LLVMValueRef,
        NumEntries: *mut ::libc::size_t,
    ) -> *mut LLVMValueMetadataEntry;
    pub fn LLVMDisposeValueMetadataEntries(Entries: *mut LLVMValueMetadataEntry);
    pub fn LLVMValueMetadataEntriesGetKind(
        Entries: *mut LLVMValueMetadataEntry,
        Index: ::libc::c_uint,
    ) -> ::libc::c_uint;
    pub fn LLVMValueMetadataEntriesGetMetadata(
        Entries: *mut LLVMValueMetadataEntry,
        Index: ::libc::c_uint,
    ) -> LLVMMetadataRef;

    // Core->Values->Constants->Global Variables
    pub fn LLVMAddGlobal(
        M: LLVMModuleRef,
        Ty: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMAddGlobalInAddressSpace(
        M: LLVMModuleRef,
        Ty: LLVMTypeRef,
        Name: *const ::libc::c_char,
        AddressSpace: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMGetNamedGlobal(M: LLVMModuleRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMGetFirstGlobal(M: LLVMModuleRef) -> LLVMValueRef;
    pub fn LLVMGetLastGlobal(M: LLVMModuleRef) -> LLVMValueRef;
    pub fn LLVMGetNextGlobal(GlobalVar: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetPreviousGlobal(GlobalVar: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMDeleteGlobal(GlobalVar: LLVMValueRef);
    pub fn LLVMGetInitializer(GlobalVar: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMSetInitializer(GlobalVar: LLVMValueRef, ConstantVal: LLVMValueRef);
    pub fn LLVMIsThreadLocal(GlobalVar: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetThreadLocal(GlobalVar: LLVMValueRef, IsThreadLocal: LLVMBool);
    pub fn LLVMIsGlobalConstant(GlobalVar: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetGlobalConstant(GlobalVar: LLVMValueRef, IsConstant: LLVMBool);
    pub fn LLVMGetThreadLocalMode(GlobalVar: LLVMValueRef) -> LLVMThreadLocalMode;
    pub fn LLVMSetThreadLocalMode(GlobalVar: LLVMValueRef, Mode: LLVMThreadLocalMode);
    pub fn LLVMIsExternallyInitialized(GlobalVar: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetExternallyInitialized(GlobalVar: LLVMValueRef, IsExtInit: LLVMBool);

    // Core->Values->Constants->Global Aliases
    /// Obtain a GlobalAlias value from a Module by its name.
    ///
    /// The returned value corresponds to a llvm::GlobalAlias value.
    pub fn LLVMGetNamedGlobalAlias(
        M: LLVMModuleRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
    ) -> LLVMValueRef;
    /// Obtain an iterator to the first GlobalAlias in a Module.
    pub fn LLVMGetFirstGlobalAlias(M: LLVMModuleRef) -> LLVMValueRef;
    /// Obtain an iterator to the last GlobalAlias in a Module.
    pub fn LLVMGetLastGlobalAlias(M: LLVMModuleRef) -> LLVMValueRef;
    /// Advance a GlobalAlias iterator to the next GlobalAlias.
    ///
    /// Returns NULL if the iterator was already at the end and there are no more global aliases.
    pub fn LLVMGetNextGlobalAlias(GA: LLVMValueRef) -> LLVMValueRef;
    /// Decrement a GlobalAlias iterator to the previous GlobalAlias.
    ///
    /// Returns NULL if the iterator was already at the beginning and there are no previous global aliases.
    pub fn LLVMGetPreviousGlobalAlias(GA: LLVMValueRef) -> LLVMValueRef;
    /// Retrieve the target value of an alias.
    pub fn LLVMAliasGetAliasee(Alias: LLVMValueRef) -> LLVMValueRef;
    /// Set the target value of an alias.
    pub fn LLVMAliasSetAliasee(Alias: LLVMValueRef, Aliasee: LLVMValueRef);

    #[deprecated(
        since = "14.0",
        note = "Use LLVMAddAlias2 instead to support opaque pointers."
    )]
    pub fn LLVMAddAlias(
        M: LLVMModuleRef,
        Ty: LLVMTypeRef,
        Aliasee: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;

    pub fn LLVMAddAlias2(
        M: LLVMModuleRef,
        ValueTy: LLVMTypeRef,
        AddrSpace: ::libc::c_uint,
        Aliasee: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;

    // ..->Function Values
    pub fn LLVMDeleteFunction(Fn: LLVMValueRef);
    /// Check whether the given function has a personality function.
    pub fn LLVMHasPersonalityFn(Fn: LLVMValueRef) -> LLVMBool;
    /// Obtain the personality function attached to the function.
    ///
    /// Added in LLVM 3.7.
    pub fn LLVMGetPersonalityFn(Fn: LLVMValueRef) -> LLVMValueRef;
    /// Set the personality function attached to the function.
    ///
    /// Added in LLVM 3.7.
    pub fn LLVMSetPersonalityFn(Fn: LLVMValueRef, PersonalityFn: LLVMValueRef);
    /// Obtain the intrinsic ID number which matches the given function name.
    pub fn LLVMLookupIntrinsicID(
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
    ) -> ::libc::c_uint;
    /// Obtain the ID number from a function instance.
    pub fn LLVMGetIntrinsicID(Fn: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetIntrinsicDeclaration(
        Mod: LLVMModuleRef,
        ID: ::libc::c_uint,
        ParamTypes: *mut LLVMTypeRef,
        ParamCount: ::libc::size_t,
    ) -> LLVMValueRef;
    pub fn LLVMIntrinsicGetType(
        Ctx: LLVMContextRef,
        ID: ::libc::c_uint,
        ParamTypes: *mut LLVMTypeRef,
        ParamCount: ::libc::size_t,
    ) -> LLVMTypeRef;
    pub fn LLVMIntrinsicGetName(
        ID: ::libc::c_uint,
        NameLength: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    #[deprecated = "Use LLVMIntrinsicCopyOverloadedName2 instead."]
    pub fn LLVMIntrinsicCopyOverloadedName(
        ID: ::libc::c_uint,
        ParamTypes: *mut LLVMTypeRef,
        ParamCount: ::libc::size_t,
        NameLength: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    pub fn LLVMIntrinsicCopyOverloadedName2(
        Mod: LLVMModuleRef,
        ID: ::libc::c_uint,
        ParamTypes: *mut LLVMTypeRef,
        ParamCount: ::libc::size_t,
        NameLength: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    pub fn LLVMIntrinsicIsOverloaded(ID: ::libc::c_uint) -> LLVMBool;
    pub fn LLVMGetFunctionCallConv(Fn: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMSetFunctionCallConv(Fn: LLVMValueRef, CC: ::libc::c_uint);
    pub fn LLVMGetGC(Fn: LLVMValueRef) -> *const ::libc::c_char;
    pub fn LLVMSetGC(Fn: LLVMValueRef, Name: *const ::libc::c_char);
    pub fn LLVMAddAttributeAtIndex(F: LLVMValueRef, Idx: LLVMAttributeIndex, A: LLVMAttributeRef);
    pub fn LLVMGetAttributeCountAtIndex(F: LLVMValueRef, Idx: LLVMAttributeIndex)
        -> ::libc::c_uint;
    pub fn LLVMGetAttributesAtIndex(
        F: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        Attrs: *mut LLVMAttributeRef,
    );
    pub fn LLVMGetEnumAttributeAtIndex(
        F: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        KindID: ::libc::c_uint,
    ) -> LLVMAttributeRef;
    pub fn LLVMGetStringAttributeAtIndex(
        F: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        K: *const ::libc::c_char,
        KLen: ::libc::c_uint,
    ) -> LLVMAttributeRef;
    pub fn LLVMRemoveEnumAttributeAtIndex(
        F: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        KindID: ::libc::c_uint,
    );
    pub fn LLVMRemoveStringAttributeAtIndex(
        F: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        K: *const ::libc::c_char,
        KLen: ::libc::c_uint,
    );
    pub fn LLVMAddTargetDependentFunctionAttr(
        Fn: LLVMValueRef,
        A: *const ::libc::c_char,
        V: *const ::libc::c_char,
    );

    // ..->Function Values->Function Parameters
    pub fn LLVMCountParams(Fn: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetParams(Fn: LLVMValueRef, Params: *mut LLVMValueRef);
    pub fn LLVMGetParam(Fn: LLVMValueRef, Index: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMGetParamParent(Inst: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetFirstParam(Fn: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetLastParam(Fn: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetNextParam(Arg: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetPreviousParam(Arg: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMSetParamAlignment(Arg: LLVMValueRef, Align: ::libc::c_uint);
}

// Core->Metadata
extern "C" {
    #[deprecated(since = "LLVM 9.0", note = "Use LLVMMDStringInContext2 instead.")]
    pub fn LLVMMDStringInContext(
        C: LLVMContextRef,
        Str: *const ::libc::c_char,
        SLen: ::libc::c_uint,
    ) -> LLVMValueRef;
    //#[deprecated(since = "LLVM 9.0", note = "Use LLVMMDStringInContext2 instead.")]
    //pub fn LLVMMDString(Str: *const ::libc::c_char, SLen: ::libc::c_uint) -> LLVMValueRef;
    #[deprecated(since = "LLVM 9.0", note = "Use LLVMMDNodeInContext2 instead.")]
    pub fn LLVMMDNodeInContext(
        C: LLVMContextRef,
        Vals: *mut LLVMValueRef,
        Count: ::libc::c_uint,
    ) -> LLVMValueRef;
    //#[deprecated(since = "LLVM 9.0", note = "Use LLVMMDNodeInContext2 instead.")]
    //pub fn LLVMMDNode(Vals: *mut LLVMValueRef, Count: ::libc::c_uint) -> LLVMValueRef;

    /// Add a global indirect function to a module under a specified name.
    pub fn LLVMAddGlobalIFunc(
        M: LLVMModuleRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        Ty: LLVMTypeRef,
        AddrSpace: ::libc::c_uint,
        Resolver: LLVMValueRef,
    ) -> LLVMValueRef;

    /// Obtain a GlobalIFunc value from a Module by its name.
    pub fn LLVMGetNamedGlobalIFunc(
        M: LLVMModuleRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
    ) -> LLVMValueRef;

    /// Obtain an iterator to the first GlobalIFunc in a Module.
    pub fn LLVMGetFirstGlobalIFunc(M: LLVMModuleRef) -> LLVMValueRef;

    /// Obtain an iterator to the last GlobalIFunc in a Module.
    pub fn LLVMGetLastGlobalIFunc(M: LLVMModuleRef) -> LLVMValueRef;

    /// Advance a GlobalIFunc iterator to the next GlobalIFunc.
    pub fn LLVMGetNextGlobalIFunc(IFunc: LLVMValueRef) -> LLVMValueRef;

    /// Decrement a GlobalIFunc iterator to the previous GlobalIFunc.
    pub fn LLVMGetPreviousGlobalIFunc(IFunc: LLVMValueRef) -> LLVMValueRef;

    /// Retrieves the resolver function associated with this indirect function, or
    /// NULL if it doesn't not exist.
    pub fn LLVMGetGlobalIFuncResolver(IFunc: LLVMValueRef) -> LLVMValueRef;

    /// Sets the resolver function associated with this indirect function.
    pub fn LLVMSetGlobalIFuncResolver(IFunc: LLVMValueRef, Resolver: LLVMValueRef);

    /// Remove a global indirect function from its parent module and delete it.
    pub fn LLVMEraseGlobalIFunc(IFunc: LLVMValueRef);

    /// Remove a global indirect function from its parent module.
    pub fn LLVMRemoveGlobalIFunc(IFunc: LLVMValueRef);

    /// Create an MDString value from a given string value.
    pub fn LLVMMDStringInContext2(
        C: LLVMContextRef,
        Str: *const ::libc::c_char,
        SLen: ::libc::size_t,
    ) -> LLVMMetadataRef;

    /// Create an MDNode value with the given array of operands.
    pub fn LLVMMDNodeInContext2(
        C: LLVMContextRef,
        MDs: *mut LLVMMetadataRef,
        Count: ::libc::size_t,
    ) -> LLVMMetadataRef;

    /// Obtain Metadata as a Value.
    pub fn LLVMMetadataAsValue(C: LLVMContextRef, MD: LLVMMetadataRef) -> LLVMValueRef;
    /// Obtain a Value as Metadata.
    pub fn LLVMValueAsMetadata(Val: LLVMValueRef) -> LLVMMetadataRef;
    /// Obtain the underlying string from a MDString value.
    ///
    /// `Len` is written to contain the length of the returned string.
    pub fn LLVMGetMDString(V: LLVMValueRef, Len: *mut ::libc::c_uint) -> *const ::libc::c_char;
    pub fn LLVMGetMDNodeNumOperands(V: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetMDNodeOperands(V: LLVMValueRef, Dest: *mut LLVMValueRef);
}

// Core->Basic Block
extern "C" {
    pub fn LLVMBasicBlockAsValue(BB: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn LLVMValueIsBasicBlock(Val: LLVMValueRef) -> LLVMBool;
    pub fn LLVMValueAsBasicBlock(Val: LLVMValueRef) -> LLVMBasicBlockRef;
    /// Get the string name of a basic block.
    pub fn LLVMGetBasicBlockName(BB: LLVMBasicBlockRef) -> *const ::libc::c_char;
    pub fn LLVMGetBasicBlockParent(BB: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn LLVMGetBasicBlockTerminator(BB: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn LLVMCountBasicBlocks(Fn: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetBasicBlocks(Fn: LLVMValueRef, BasicBlocks: *mut LLVMBasicBlockRef);
    pub fn LLVMGetFirstBasicBlock(Fn: LLVMValueRef) -> LLVMBasicBlockRef;
    pub fn LLVMGetLastBasicBlock(Fn: LLVMValueRef) -> LLVMBasicBlockRef;
    pub fn LLVMGetNextBasicBlock(BB: LLVMBasicBlockRef) -> LLVMBasicBlockRef;
    pub fn LLVMGetPreviousBasicBlock(BB: LLVMBasicBlockRef) -> LLVMBasicBlockRef;
    pub fn LLVMGetEntryBasicBlock(Fn: LLVMValueRef) -> LLVMBasicBlockRef;
    /// Insert the given basic block after the insertion point of the given builder.
    pub fn LLVMInsertExistingBasicBlockAfterInsertBlock(
        Builder: LLVMBuilderRef,
        BB: LLVMBasicBlockRef,
    );
    /// Append the given basic block to the basic block list of the given function.
    pub fn LLVMAppendExistingBasicBlock(Fn: LLVMValueRef, BB: LLVMBasicBlockRef);
    pub fn LLVMCreateBasicBlockInContext(
        C: LLVMContextRef,
        Name: *const ::libc::c_char,
    ) -> LLVMBasicBlockRef;
    pub fn LLVMAppendBasicBlockInContext(
        C: LLVMContextRef,
        Fn: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMBasicBlockRef;
    //pub fn LLVMAppendBasicBlock(Fn: LLVMValueRef, Name: *const ::libc::c_char)
    //    -> LLVMBasicBlockRef;
    pub fn LLVMInsertBasicBlockInContext(
        C: LLVMContextRef,
        BB: LLVMBasicBlockRef,
        Name: *const ::libc::c_char,
    ) -> LLVMBasicBlockRef;
    //pub fn LLVMInsertBasicBlock(
    //    InsertBeforeBB: LLVMBasicBlockRef,
    //    Name: *const ::libc::c_char,
    //) -> LLVMBasicBlockRef;
    pub fn LLVMDeleteBasicBlock(BB: LLVMBasicBlockRef);
    pub fn LLVMRemoveBasicBlockFromParent(BB: LLVMBasicBlockRef);
    pub fn LLVMMoveBasicBlockBefore(BB: LLVMBasicBlockRef, MovePos: LLVMBasicBlockRef);
    pub fn LLVMMoveBasicBlockAfter(BB: LLVMBasicBlockRef, MovePos: LLVMBasicBlockRef);
    pub fn LLVMGetFirstInstruction(BB: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn LLVMGetLastInstruction(BB: LLVMBasicBlockRef) -> LLVMValueRef;
}

// Core->Instructions
extern "C" {
    pub fn LLVMHasMetadata(Val: LLVMValueRef) -> ::libc::c_int;
    pub fn LLVMGetMetadata(Val: LLVMValueRef, KindID: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMSetMetadata(Val: LLVMValueRef, KindID: ::libc::c_uint, Node: LLVMValueRef);
    pub fn LLVMInstructionGetAllMetadataOtherThanDebugLoc(
        Instr: LLVMValueRef,
        NumEntries: *mut ::libc::size_t,
    ) -> *mut LLVMValueMetadataEntry;
    pub fn LLVMGetInstructionParent(Inst: LLVMValueRef) -> LLVMBasicBlockRef;
    pub fn LLVMGetNextInstruction(Inst: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetPreviousInstruction(Inst: LLVMValueRef) -> LLVMValueRef;
    /// Remove the given instruction from its containing building block but
    /// kept alive.
    pub fn LLVMInstructionRemoveFromParent(Inst: LLVMValueRef);
    /// Remove the given instruction from its containing building block and
    /// delete it.
    pub fn LLVMInstructionEraseFromParent(Inst: LLVMValueRef);
    /// Remove the given instruction that is not inserted into a basic block.
    /// It must have previously been removed from its containing building block.
    pub fn LLVMDeleteInstruction(Inst: LLVMValueRef);
    pub fn LLVMGetInstructionOpcode(Inst: LLVMValueRef) -> LLVMOpcode;
    pub fn LLVMGetICmpPredicate(Inst: LLVMValueRef) -> LLVMIntPredicate;
    pub fn LLVMGetFCmpPredicate(Inst: LLVMValueRef) -> LLVMRealPredicate;
    pub fn LLVMInstructionClone(Inst: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsATerminatorInst(Inst: LLVMValueRef) -> LLVMValueRef;

    // Instructions->Call Sites and Invocations
    // Obtain the argument count for a call instruction.
    //
    // The provided value should be either a CallInst, InvokeInst or FuncletPadInst.
    pub fn LLVMGetNumArgOperands(Instr: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMSetInstructionCallConv(Instr: LLVMValueRef, CC: ::libc::c_uint);
    pub fn LLVMGetInstructionCallConv(Instr: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMSetInstrParamAlignment(
        Instr: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        Align: ::libc::c_uint,
    );
    pub fn LLVMAddCallSiteAttribute(C: LLVMValueRef, Idx: LLVMAttributeIndex, A: LLVMAttributeRef);
    pub fn LLVMGetCallSiteAttributeCount(
        C: LLVMValueRef,
        Idx: LLVMAttributeIndex,
    ) -> ::libc::c_uint;
    pub fn LLVMGetCallSiteAttributes(
        C: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        Attrs: *mut LLVMAttributeRef,
    );
    pub fn LLVMGetCallSiteEnumAttribute(
        C: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        KindID: ::libc::c_uint,
    ) -> LLVMAttributeRef;
    pub fn LLVMGetCallSiteStringAttribute(
        C: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        K: *const ::libc::c_char,
        KLen: ::libc::c_uint,
    ) -> LLVMAttributeRef;
    pub fn LLVMRemoveCallSiteEnumAttribute(
        C: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        KindID: ::libc::c_uint,
    );
    pub fn LLVMRemoveCallSiteStringAttribute(
        C: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        K: *const ::libc::c_char,
        KLen: ::libc::c_uint,
    );
    pub fn LLVMGetCalledFunctionType(C: LLVMValueRef) -> LLVMTypeRef;
    /// Get a pointer to the function invoked by this instruction.
    ///
    /// The provided value should be a CallInst or InvokeInst.
    pub fn LLVMGetCalledValue(Instr: LLVMValueRef) -> LLVMValueRef;
    /// Get whether a call instruction is a tail call.
    pub fn LLVMIsTailCall(CallInst: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetTailCall(CallInst: LLVMValueRef, IsTailCall: LLVMBool);
    /// Return the normal destination basic block of an invoke instruction.
    pub fn LLVMGetNormalDest(InvokeInst: LLVMValueRef) -> LLVMBasicBlockRef;
    /// Return the unwind destination basic block.
    pub fn LLVMGetUnwindDest(InvokeInst: LLVMValueRef) -> LLVMBasicBlockRef;
    /// Set the normal destination basic block.
    pub fn LLVMSetNormalDest(InvokeInst: LLVMValueRef, B: LLVMBasicBlockRef);
    /// Set the unwind destination basic block.
    pub fn LLVMSetUnwindDest(InvokeInst: LLVMValueRef, B: LLVMBasicBlockRef);

    // Instructions->Terminators
    pub fn LLVMGetNumSuccessors(Term: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetSuccessor(Term: LLVMValueRef, i: ::libc::c_uint) -> LLVMBasicBlockRef;
    pub fn LLVMSetSuccessor(Term: LLVMValueRef, i: ::libc::c_uint, block: LLVMBasicBlockRef);
    pub fn LLVMIsConditional(Branch: LLVMValueRef) -> LLVMBool;
    pub fn LLVMGetCondition(Branch: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMSetCondition(Branch: LLVMValueRef, Cond: LLVMValueRef);
    pub fn LLVMGetSwitchDefaultDest(SwitchInstr: LLVMValueRef) -> LLVMBasicBlockRef;

    // Instructions->Allocas
    // Obtain the type being allocated by an alloca instruction.
    pub fn LLVMGetAllocatedType(Alloca: LLVMValueRef) -> LLVMTypeRef;

    // Instructions->GEPs
    // Check whether the given GEP operator is inbounds.
    pub fn LLVMIsInBounds(GEP: LLVMValueRef) -> LLVMBool;
    /// Set the given GEP instruction to be inbounds or not.
    pub fn LLVMSetIsInBounds(GEP: LLVMValueRef, InBounds: LLVMBool);

    /// Get the source element type of the given GEP operator.
    pub fn LLVMGetGEPSourceElementType(GEP: LLVMValueRef) -> LLVMTypeRef;

    // Instruction->PHI Nodes
    pub fn LLVMAddIncoming(
        PhiNode: LLVMValueRef,
        IncomingValues: *mut LLVMValueRef,
        IncomingBlocks: *mut LLVMBasicBlockRef,
        Count: ::libc::c_uint,
    );
    pub fn LLVMCountIncoming(PhiNode: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetIncomingValue(PhiNode: LLVMValueRef, Index: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMGetIncomingBlock(PhiNode: LLVMValueRef, Index: ::libc::c_uint) -> LLVMBasicBlockRef;

}

// Core->Values again; these don't appear in Doxygen because they're macro-generated.
extern "C" {
    pub fn LLVMIsAArgument(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABasicBlock(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInlineAsm(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUser(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstant(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABlockAddress(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantAggregateZero(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantArray(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantDataSequential(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantDataArray(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantDataVector(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantExpr(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantFP(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantInt(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantPointerNull(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantStruct(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantTokenNone(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantVector(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalValue(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalAlias(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalIFunc(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalObject(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFunction(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalVariable(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUndefValue(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAPoisonValue(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInstruction(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUnaryOperator(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABinaryOperator(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACallInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAIntrinsicInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsADbgInfoIntrinsic(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsADbgVariableIntrinsic(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsADbgDeclareInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsADbgLabelInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemIntrinsic(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemCpyInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemMoveInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemSetInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACmpInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFCmpInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAICmpInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAExtractElementInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGetElementPtrInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInsertElementInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInsertValueInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsALandingPadInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAPHINode(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASelectInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAShuffleVectorInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAStoreInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABranchInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAIndirectBrInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInvokeInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAReturnInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASwitchInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUnreachableInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAResumeInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACleanupReturnInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACatchReturnInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACatchSwitchInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACallBrInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFuncletPadInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACatchPadInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACleanupPadInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUnaryInstruction(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAAllocaInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACastInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAAddrSpaceCastInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABitCastInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPExtInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPToSIInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPToUIInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPTruncInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAIntToPtrInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAPtrToIntInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASExtInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASIToFPInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsATruncInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUIToFPInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAZExtInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAExtractValueInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsALoadInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAVAArgInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFreezeInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAAtomicCmpXchgInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAAtomicRMWInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFenceInst(Val: LLVMValueRef) -> LLVMValueRef;
}

// Core->Extract/Insert Value
extern "C" {
    /// Get the number of indices on an ExtractValue, InsertValue or GEP operator.
    pub fn LLVMGetNumIndices(Inst: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetIndices(Inst: LLVMValueRef) -> *const ::libc::c_uint;
}

// Core->Instruction Builders
extern "C" {
    pub fn LLVMCreateBuilderInContext(C: LLVMContextRef) -> LLVMBuilderRef;
    //pub fn LLVMCreateBuilder() -> LLVMBuilderRef;
    pub fn LLVMPositionBuilder(
        Builder: LLVMBuilderRef,
        Block: LLVMBasicBlockRef,
        Instr: LLVMValueRef,
    );
    pub fn LLVMPositionBuilderBefore(Builder: LLVMBuilderRef, Instr: LLVMValueRef);
    pub fn LLVMPositionBuilderAtEnd(Builder: LLVMBuilderRef, Block: LLVMBasicBlockRef);
    pub fn LLVMGetInsertBlock(Builder: LLVMBuilderRef) -> LLVMBasicBlockRef;
    pub fn LLVMClearInsertionPosition(Builder: LLVMBuilderRef);
    pub fn LLVMInsertIntoBuilder(Builder: LLVMBuilderRef, Instr: LLVMValueRef);
    pub fn LLVMInsertIntoBuilderWithName(
        Builder: LLVMBuilderRef,
        Instr: LLVMValueRef,
        Name: *const ::libc::c_char,
    );
    pub fn LLVMDisposeBuilder(Builder: LLVMBuilderRef);

    // Metadata
    /// Get location information used by debugging information.
    pub fn LLVMGetCurrentDebugLocation2(Builder: LLVMBuilderRef) -> LLVMMetadataRef;
    /// Set location information used by debugging information.
    pub fn LLVMSetCurrentDebugLocation2(Builder: LLVMBuilderRef, Loc: LLVMMetadataRef);
    /// Attempts to set the debug location for the given instruction using the
    /// current debug location for the given builder.  If the builder has no current
    /// debug location, this function is a no-op.
    #[deprecated(
        since = "14.0",
        note = "Deprecated in favor of the more general LLVMAddMetadataToInst."
    )]
    pub fn LLVMSetInstDebugLocation(Builder: LLVMBuilderRef, Inst: LLVMValueRef);
    /// Adds the metadata registered with the given builder to the given instruction.
    pub fn LLVMAddMetadataToInst(Builder: LLVMBuilderRef, Inst: LLVMValueRef);
    /// Get the dafult floating-point math metadata for a given builder.
    pub fn LLVMBuilderGetDefaultFPMathTag(Builder: LLVMBuilderRef) -> LLVMMetadataRef;
    /// Set the default floating-point math metadata for the given builder.
    pub fn LLVMBuilderSetDefaultFPMathTag(Builder: LLVMBuilderRef, FPMathTag: LLVMMetadataRef);
    #[deprecated(since = "LLVM 9.0", note = "Use LLVMGetCurrentDebugLocation2 instead.")]
    pub fn LLVMSetCurrentDebugLocation(Builder: LLVMBuilderRef, L: LLVMValueRef);
    pub fn LLVMGetCurrentDebugLocation(Builder: LLVMBuilderRef) -> LLVMValueRef;

    // Terminators
    pub fn LLVMBuildRetVoid(arg1: LLVMBuilderRef) -> LLVMValueRef;
    pub fn LLVMBuildRet(arg1: LLVMBuilderRef, V: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMBuildAggregateRet(
        arg1: LLVMBuilderRef,
        RetVals: *mut LLVMValueRef,
        N: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMBuildBr(arg1: LLVMBuilderRef, Dest: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn LLVMBuildCondBr(
        arg1: LLVMBuilderRef,
        If: LLVMValueRef,
        Then: LLVMBasicBlockRef,
        Else: LLVMBasicBlockRef,
    ) -> LLVMValueRef;
    pub fn LLVMBuildSwitch(
        arg1: LLVMBuilderRef,
        V: LLVMValueRef,
        Else: LLVMBasicBlockRef,
        NumCases: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMBuildIndirectBr(
        B: LLVMBuilderRef,
        Addr: LLVMValueRef,
        NumDests: ::libc::c_uint,
    ) -> LLVMValueRef;
    #[deprecated(
        since = "14.0",
        note = "Use LLVMBuildInvoke2 instead to support opaque pointers."
    )]
    pub fn LLVMBuildInvoke(
        arg1: LLVMBuilderRef,
        Fn: LLVMValueRef,
        Args: *mut LLVMValueRef,
        NumArgs: ::libc::c_uint,
        Then: LLVMBasicBlockRef,
        Catch: LLVMBasicBlockRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildInvoke2(
        arg1: LLVMBuilderRef,
        Ty: LLVMTypeRef,
        Fn: LLVMValueRef,
        Args: *mut LLVMValueRef,
        NumArgs: ::libc::c_uint,
        Then: LLVMBasicBlockRef,
        Catch: LLVMBasicBlockRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildUnreachable(B: LLVMBuilderRef) -> LLVMValueRef;

    pub fn LLVMBuildResume(B: LLVMBuilderRef, Exn: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMBuildLandingPad(
        B: LLVMBuilderRef,
        Ty: LLVMTypeRef,
        PersFn: LLVMValueRef,
        NumClauses: ::libc::c_uint,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildCleanupRet(
        B: LLVMBuilderRef,
        CatchPad: LLVMValueRef,
        BB: LLVMBasicBlockRef,
    ) -> LLVMValueRef;
    pub fn LLVMBuildCatchRet(
        B: LLVMBuilderRef,
        CatchPad: LLVMValueRef,
        BB: LLVMBasicBlockRef,
    ) -> LLVMValueRef;
    pub fn LLVMBuildCatchPad(
        B: LLVMBuilderRef,
        ParentPad: LLVMValueRef,
        Args: *mut LLVMValueRef,
        NumArgs: ::libc::c_uint,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildCleanupPad(
        B: LLVMBuilderRef,
        ParentPad: LLVMValueRef,
        Args: *mut LLVMValueRef,
        NumArgs: ::libc::c_uint,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildCatchSwitch(
        B: LLVMBuilderRef,
        ParentPad: LLVMValueRef,
        UnwindBB: LLVMBasicBlockRef,
        NumHandler: ::libc::c_uint,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;

    /// Add a case to a `switch` instruction
    pub fn LLVMAddCase(Switch: LLVMValueRef, OnVal: LLVMValueRef, Dest: LLVMBasicBlockRef);

    /// Add a destination to an `indirectbr` instruction
    pub fn LLVMAddDestination(IndirectBr: LLVMValueRef, Dest: LLVMBasicBlockRef);

    /// Get the number of clauses on a landingpad instruction.
    pub fn LLVMGetNumClauses(LandingPad: LLVMValueRef) -> ::libc::c_uint;

    /// Get the value of the clause with the given index on a landingpad instruction.
    pub fn LLVMGetClause(LandingPad: LLVMValueRef, Idx: ::libc::c_uint) -> LLVMValueRef;

    /// Add a catch or filter clause to a `landingpad` instruction
    pub fn LLVMAddClause(LandingPad: LLVMValueRef, ClauseVal: LLVMValueRef);

    /// Get the cleanup flag in a landingpad instruction.
    pub fn LLVMIsCleanup(LandingPad: LLVMValueRef) -> LLVMBool;

    /// Set the cleanup flag in a `landingpad` instruction.
    pub fn LLVMSetCleanup(LandingPad: LLVMValueRef, Val: LLVMBool);

    /// Add a destination to the catchswitch instruction
    pub fn LLVMAddHandler(CatchSwitch: LLVMValueRef, Dest: LLVMBasicBlockRef);

    /// Get the number of handlers on the catchswitch instruction
    pub fn LLVMGetNumHandlers(CatchSwitch: LLVMValueRef) -> ::libc::c_uint;

    /// Obtain the basic blocks acting as handlers for a catchswitch instruction.
    ///
    /// The Handlers parameter should point to a pre-allocated array of LLVMBasicBlockRefs at least LLVMGetNumHandlers() large. On return, the first LLVMGetNumHandlers() entries in the array will be populated with LLVMBasicBlockRef instances.
    pub fn LLVMGetHandlers(CatchSwitch: LLVMValueRef, Handlers: *mut LLVMBasicBlockRef);

    // Funclets
    /// Get the number of funcletpad arguments.
    pub fn LLVMGetArgOperand(Funclet: LLVMValueRef, i: ::libc::c_uint) -> LLVMValueRef;

    /// Set a funcletpad argument at the given index.
    pub fn LLVMSetArgOperand(Funclet: LLVMValueRef, i: ::libc::c_uint, value: LLVMValueRef);

    /// Get the parent catchswitch instruction of a catchpad instruction.
    ///
    /// This only works on llvm::CatchPadInst instructions.
    pub fn LLVMGetParentCatchSwitch(CatchPad: LLVMValueRef) -> LLVMValueRef;

    /// Set the parent catchswitch instruction of a catchpad instruction.
    /// This only works on llvm::CatchPadInst instructions.
    pub fn LLVMSetParentCatchSwitch(CatchPad: LLVMValueRef, CatchSwitch: LLVMValueRef);

    // Arithmetic
    pub fn LLVMBuildAdd(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildNSWAdd(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildNUWAdd(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFAdd(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildSub(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildNSWSub(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildNUWSub(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFSub(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildMul(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildNSWMul(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildNUWMul(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFMul(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildUDiv(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildExactUDiv(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildSDiv(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildExactSDiv(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFDiv(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildURem(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildSRem(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFRem(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildShl(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildLShr(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildAShr(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildAnd(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildOr(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildXor(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildBinOp(
        B: LLVMBuilderRef,
        Op: LLVMOpcode,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildNeg(
        arg1: LLVMBuilderRef,
        V: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildNSWNeg(
        B: LLVMBuilderRef,
        V: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildNUWNeg(
        B: LLVMBuilderRef,
        V: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFNeg(
        arg1: LLVMBuilderRef,
        V: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildNot(
        arg1: LLVMBuilderRef,
        V: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;

    // Memory
    pub fn LLVMBuildMalloc(
        arg1: LLVMBuilderRef,
        Ty: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildArrayMalloc(
        arg1: LLVMBuilderRef,
        Ty: LLVMTypeRef,
        Val: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildMemSet(
        B: LLVMBuilderRef,
        Ptr: LLVMValueRef,
        Val: LLVMValueRef,
        Len: LLVMValueRef,
        Align: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMBuildMemCpy(
        B: LLVMBuilderRef,
        Dst: LLVMValueRef,
        DstAlign: ::libc::c_uint,
        Src: LLVMValueRef,
        SrcAlign: ::libc::c_uint,
        Size: LLVMValueRef,
    ) -> LLVMValueRef;
    pub fn LLVMBuildMemMove(
        B: LLVMBuilderRef,
        Dst: LLVMValueRef,
        DstAlign: ::libc::c_uint,
        Src: LLVMValueRef,
        SrcAlign: ::libc::c_uint,
        Size: LLVMValueRef,
    ) -> LLVMValueRef;
    pub fn LLVMBuildAlloca(
        arg1: LLVMBuilderRef,
        Ty: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildArrayAlloca(
        arg1: LLVMBuilderRef,
        Ty: LLVMTypeRef,
        Val: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFree(arg1: LLVMBuilderRef, PointerVal: LLVMValueRef) -> LLVMValueRef;
    #[deprecated(
        since = "14.0",
        note = "Use LLVMBuildLoad2 instead to support opaque pointers."
    )]
    pub fn LLVMBuildLoad(
        arg1: LLVMBuilderRef,
        PointerVal: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildLoad2(
        arg1: LLVMBuilderRef,
        Ty: LLVMTypeRef,
        PointerVal: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildStore(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        Ptr: LLVMValueRef,
    ) -> LLVMValueRef;
    #[deprecated(
        since = "14.0",
        note = "Use LLVMBuildGEP2 instead to support opaque pointers."
    )]
    pub fn LLVMBuildGEP(
        B: LLVMBuilderRef,
        Pointer: LLVMValueRef,
        Indices: *mut LLVMValueRef,
        NumIndices: ::libc::c_uint,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    #[deprecated(
        since = "14.0",
        note = "Use LLVMBuildInBoundsGEP2 instead to support opaque pointers."
    )]
    pub fn LLVMBuildInBoundsGEP(
        B: LLVMBuilderRef,
        Pointer: LLVMValueRef,
        Indices: *mut LLVMValueRef,
        NumIndices: ::libc::c_uint,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    #[deprecated(
        since = "14.0",
        note = "Use LLVMBuildStructGEP2 instead to support opaque pointers."
    )]
    pub fn LLVMBuildStructGEP(
        B: LLVMBuilderRef,
        Pointer: LLVMValueRef,
        Idx: ::libc::c_uint,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildGEP2(
        B: LLVMBuilderRef,
        Ty: LLVMTypeRef,
        Pointer: LLVMValueRef,
        Indices: *mut LLVMValueRef,
        NumIndices: ::libc::c_uint,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildInBoundsGEP2(
        B: LLVMBuilderRef,
        Ty: LLVMTypeRef,
        Pointer: LLVMValueRef,
        Indices: *mut LLVMValueRef,
        NumIndices: ::libc::c_uint,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildStructGEP2(
        B: LLVMBuilderRef,
        Ty: LLVMTypeRef,
        Pointer: LLVMValueRef,
        Idx: ::libc::c_uint,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildGlobalString(
        B: LLVMBuilderRef,
        Str: *const ::libc::c_char,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildGlobalStringPtr(
        B: LLVMBuilderRef,
        Str: *const ::libc::c_char,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMGetVolatile(MemoryAccessInst: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetVolatile(MemoryAccessInst: LLVMValueRef, IsVolatile: LLVMBool);
    pub fn LLVMGetWeak(CmpXchgInst: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetWeak(CmpXchgInst: LLVMValueRef, IsWeak: LLVMBool);
    pub fn LLVMGetOrdering(MemoryAccessInst: LLVMValueRef) -> LLVMAtomicOrdering;
    pub fn LLVMSetOrdering(MemoryAccessInst: LLVMValueRef, Ordering: LLVMAtomicOrdering);
    pub fn LLVMGetAtomicRMWBinOp(AtomicRMWInst: LLVMValueRef) -> LLVMAtomicRMWBinOp;
    pub fn LLVMSetAtomicRMWBinOp(AtomicRMWInst: LLVMValueRef, BinOp: LLVMAtomicRMWBinOp);

    // Casts
    pub fn LLVMBuildTrunc(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildZExt(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildSExt(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFPToUI(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFPToSI(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildUIToFP(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildSIToFP(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFPTrunc(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFPExt(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildPtrToInt(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildIntToPtr(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildBitCast(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildAddrSpaceCast(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildZExtOrBitCast(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildSExtOrBitCast(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildTruncOrBitCast(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildCast(
        B: LLVMBuilderRef,
        Op: LLVMOpcode,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildPointerCast(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildIntCast(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildIntCast2(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        IsSigned: LLVMBool,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFPCast(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        DestTy: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMGetCastOpcode(
        arg1: LLVMValueRef,
        SrcIsSigned: LLVMBool,
        DestTy: LLVMTypeRef,
        DestIsSigned: LLVMBool,
    ) -> LLVMOpcode;

    // Comparisons
    pub fn LLVMBuildICmp(
        arg1: LLVMBuilderRef,
        Op: LLVMIntPredicate,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFCmp(
        arg1: LLVMBuilderRef,
        Op: LLVMRealPredicate,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;

    // Miscellaneous instructions
    pub fn LLVMBuildPhi(
        arg1: LLVMBuilderRef,
        Ty: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    #[deprecated(
        since = "14.0",
        note = "Use LLVMBuildCall2 instead to support opaque pointers."
    )]
    pub fn LLVMBuildCall(
        arg1: LLVMBuilderRef,
        Fn: LLVMValueRef,
        Args: *mut LLVMValueRef,
        NumArgs: ::libc::c_uint,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildCall2(
        arg1: LLVMBuilderRef,
        arg2: LLVMTypeRef,
        Fn: LLVMValueRef,
        Args: *mut LLVMValueRef,
        NumArgs: ::libc::c_uint,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildSelect(
        arg1: LLVMBuilderRef,
        If: LLVMValueRef,
        Then: LLVMValueRef,
        Else: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildVAArg(
        arg1: LLVMBuilderRef,
        List: LLVMValueRef,
        Ty: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildExtractElement(
        arg1: LLVMBuilderRef,
        VecVal: LLVMValueRef,
        Index: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildInsertElement(
        arg1: LLVMBuilderRef,
        VecVal: LLVMValueRef,
        EltVal: LLVMValueRef,
        Index: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildShuffleVector(
        arg1: LLVMBuilderRef,
        V1: LLVMValueRef,
        V2: LLVMValueRef,
        Mask: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildExtractValue(
        arg1: LLVMBuilderRef,
        AggVal: LLVMValueRef,
        Index: ::libc::c_uint,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildInsertValue(
        arg1: LLVMBuilderRef,
        AggVal: LLVMValueRef,
        EltVal: LLVMValueRef,
        Index: ::libc::c_uint,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFreeze(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildIsNull(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildIsNotNull(
        arg1: LLVMBuilderRef,
        Val: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    #[deprecated(
        since = "14.0",
        note = "Use LLVMBuildPtrDiff2 instead to support opaque pointers."
    )]
    pub fn LLVMBuildPtrDiff(
        arg1: LLVMBuilderRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildPtrDiff2(
        arg1: LLVMBuilderRef,
        ElemTy: LLVMTypeRef,
        LHS: LLVMValueRef,
        RHS: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildFence(
        B: LLVMBuilderRef,
        ordering: LLVMAtomicOrdering,
        singleThread: LLVMBool,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMBuildAtomicRMW(
        B: LLVMBuilderRef,
        op: LLVMAtomicRMWBinOp,
        PTR: LLVMValueRef,
        Val: LLVMValueRef,
        ordering: LLVMAtomicOrdering,
        singleThread: LLVMBool,
    ) -> LLVMValueRef;
    pub fn LLVMBuildAtomicCmpXchg(
        B: LLVMBuilderRef,
        Ptr: LLVMValueRef,
        Cmp: LLVMValueRef,
        New: LLVMValueRef,
        SuccessOrdering: LLVMAtomicOrdering,
        FailureOrdering: LLVMAtomicOrdering,
        SingleThread: LLVMBool,
    ) -> LLVMValueRef;
    pub fn LLVMGetNumMaskElements(ShuffleVectorInst: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetUndefMaskElem() -> ::libc::c_int;
    pub fn LLVMGetMaskValue(ShuffleVectorInst: LLVMValueRef, Elt: ::libc::c_uint) -> ::libc::c_int;
    pub fn LLVMIsAtomicSingleThread(AtomicInst: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetAtomicSingleThread(AtomicInst: LLVMValueRef, SingleThread: LLVMBool);
    pub fn LLVMGetCmpXchgSuccessOrdering(CmpXchgInst: LLVMValueRef) -> LLVMAtomicOrdering;
    pub fn LLVMSetCmpXchgSuccessOrdering(CmpXchgInst: LLVMValueRef, Ordering: LLVMAtomicOrdering);
    pub fn LLVMGetCmpXchgFailureOrdering(CmpXchgInst: LLVMValueRef) -> LLVMAtomicOrdering;
    pub fn LLVMSetCmpXchgFailureOrdering(CmpXchgInst: LLVMValueRef, Ordering: LLVMAtomicOrdering);
}

// Core->Module Providers
extern "C" {
    pub fn LLVMCreateModuleProviderForExistingModule(M: LLVMModuleRef) -> LLVMModuleProviderRef;
    pub fn LLVMDisposeModuleProvider(M: LLVMModuleProviderRef);
}

// Core->Memory Buffers
extern "C" {
    pub fn LLVMCreateMemoryBufferWithContentsOfFile(
        Path: *const ::libc::c_char,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        OutMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMCreateMemoryBufferWithSTDIN(
        OutMemBuf: *mut LLVMMemoryBufferRef,
        OutMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMCreateMemoryBufferWithMemoryRange(
        InputData: *const ::libc::c_char,
        InputDataLength: ::libc::size_t,
        BufferName: *const ::libc::c_char,
        RequiresNullTerminator: LLVMBool,
    ) -> LLVMMemoryBufferRef;
    pub fn LLVMCreateMemoryBufferWithMemoryRangeCopy(
        InputData: *const ::libc::c_char,
        InputDataLength: ::libc::size_t,
        BufferName: *const ::libc::c_char,
    ) -> LLVMMemoryBufferRef;
    pub fn LLVMGetBufferStart(MemBuf: LLVMMemoryBufferRef) -> *const ::libc::c_char;
    pub fn LLVMGetBufferSize(MemBuf: LLVMMemoryBufferRef) -> ::libc::size_t;
    pub fn LLVMDisposeMemoryBuffer(MemBuf: LLVMMemoryBufferRef);
}

// Core->pass registry
extern "C" {
    pub fn LLVMGetGlobalPassRegistry() -> LLVMPassRegistryRef;
}

// Core->Pass managers
extern "C" {
    pub fn LLVMCreatePassManager() -> LLVMPassManagerRef;
    pub fn LLVMCreateFunctionPassManagerForModule(M: LLVMModuleRef) -> LLVMPassManagerRef;
    pub fn LLVMCreateFunctionPassManager(MP: LLVMModuleProviderRef) -> LLVMPassManagerRef;
    pub fn LLVMRunPassManager(PM: LLVMPassManagerRef, M: LLVMModuleRef) -> LLVMBool;
    pub fn LLVMInitializeFunctionPassManager(FPM: LLVMPassManagerRef) -> LLVMBool;
    pub fn LLVMRunFunctionPassManager(FPM: LLVMPassManagerRef, F: LLVMValueRef) -> LLVMBool;
    pub fn LLVMFinalizeFunctionPassManager(FPM: LLVMPassManagerRef) -> LLVMBool;
    pub fn LLVMDisposePassManager(PM: LLVMPassManagerRef);
}

// Core->Threading
extern "C" {
    /// Deprecated: LLVM threading is configured at compile-time with `LLVM_ENABLE_THREADS`
    pub fn LLVMStartMultithreaded() -> LLVMBool;
    /// Deprecated: LLVM threading is configured at compile-time with `LLVM_ENABLE_THREADS`
    pub fn LLVMStopMultithreaded();
    pub fn LLVMIsMultithreaded() -> LLVMBool;
}
