//! Generation of DWARF debug info.
use super::*;

// Debug info flags.
pub type LLVMDIFlags = ::libc::c_int;
pub const LLVMDIFlagZero: LLVMDIFlags = 0;
pub const LLVMDIFlagPrivate: LLVMDIFlags = 1;
pub const LLVMDIFlagProtected: LLVMDIFlags = 2;
pub const LLVMDIFlagPublic: LLVMDIFlags = 3;
pub const LLVMDIFlagFwdDecl: LLVMDIFlags = 1 << 2;
pub const LLVMDIFlagAppleBlock: LLVMDIFlags = 1 << 3;
pub const LLVMDIFlagReservedBit4: LLVMDIFlags = 1 << 4;
pub const LLVMDIFlagVirtual: LLVMDIFlags = 1 << 5;
pub const LLVMDIFlagArtificial: LLVMDIFlags = 1 << 6;
pub const LLVMDIFlagExplicit: LLVMDIFlags = 1 << 7;
pub const LLVMDIFlagPrototyped: LLVMDIFlags = 1 << 8;
pub const LLVMDIFlagObjcClassComplete: LLVMDIFlags = 1 << 9;
pub const LLVMDIFlagObjectPointer: LLVMDIFlags = 1 << 10;
pub const LLVMDIFlagVector: LLVMDIFlags = 1 << 11;
pub const LLVMDIFlagStaticMember: LLVMDIFlags = 1 << 12;
pub const LLVMDIFlagLValueReference: LLVMDIFlags = 1 << 13;
pub const LLVMDIFlagRValueReference: LLVMDIFlags = 1 << 14;
pub const LLVMDIFlagReserved: LLVMDIFlags = 1 << 15;
pub const LLVMDIFlagSingleInheritance: LLVMDIFlags = 1 << 16;
pub const LLVMDIFlagMultipleInheritance: LLVMDIFlags = 2 << 16;
pub const LLVMDIFlagVirtualInheritance: LLVMDIFlags = 3 << 16;
pub const LLVMDIFlagIntroducedVirtual: LLVMDIFlags = 1 << 18;
pub const LLVMDIFlagBitField: LLVMDIFlags = 1 << 19;
pub const LLVMDIFlagNoReturn: LLVMDIFlags = 1 << 20;
pub const LLVMDIFlagTypePassByValue: LLVMDIFlags = 1 << 22;
pub const LLVMDIFlagTypePassByReference: LLVMDIFlags = 1 << 23;
pub const LLVMDIFlagEnumClass: LLVMDIFlags = 1 << 24;
pub const LLVMDIFlagThunk: LLVMDIFlags = 1 << 25;
pub const LLVMDIFlagNonTrivial: LLVMDIFlags = 1 << 26;
pub const LLVMDIFlagBigendian: LLVMDIFlags = 1 << 27;
pub const LLVMDIFlagLittleEndian: LLVMDIFlags = 1 << 28;
pub const LLVMDIFlagIndirectVirtualBase: LLVMDIFlags = (1 << 2) | (1 << 5);
pub const LLVMDIFlagAccessibility: LLVMDIFlags =
    LLVMDIFlagProtected | LLVMDIFlagPrivate | LLVMDIFlagPublic;
pub const LLVMDIFlagPtrToMemberRep: LLVMDIFlags =
    LLVMDIFlagSingleInheritance | LLVMDIFlagMultipleInheritance | LLVMDIFlagVirtualInheritance;

/// Source languages known by DWARF.
#[repr(C)]
#[derive(Debug)]
pub enum LLVMDWARFSourceLanguage {
    LLVMDWARFSourceLanguageC89,
    LLVMDWARFSourceLanguageC,
    LLVMDWARFSourceLanguageAda83,
    LLVMDWARFSourceLanguageC_plus_plus,
    LLVMDWARFSourceLanguageCobol74,
    LLVMDWARFSourceLanguageCobol85,
    LLVMDWARFSourceLanguageFortran77,
    LLVMDWARFSourceLanguageFortran90,
    LLVMDWARFSourceLanguagePascal83,
    LLVMDWARFSourceLanguageModula2,
    // New in DWARF v3:
    LLVMDWARFSourceLanguageJava,
    LLVMDWARFSourceLanguageC99,
    LLVMDWARFSourceLanguageAda95,
    LLVMDWARFSourceLanguageFortran95,
    LLVMDWARFSourceLanguagePLI,
    LLVMDWARFSourceLanguageObjC,
    LLVMDWARFSourceLanguageObjC_plus_plus,
    LLVMDWARFSourceLanguageUPC,
    LLVMDWARFSourceLanguageD,
    // New in DWARF v4:
    LLVMDWARFSourceLanguagePython,
    // New in DWARF v5:
    LLVMDWARFSourceLanguageOpenCL,
    LLVMDWARFSourceLanguageGo,
    LLVMDWARFSourceLanguageModula3,
    LLVMDWARFSourceLanguageHaskell,
    LLVMDWARFSourceLanguageC_plus_plus_03,
    LLVMDWARFSourceLanguageC_plus_plus_11,
    LLVMDWARFSourceLanguageOCaml,
    LLVMDWARFSourceLanguageRust,
    LLVMDWARFSourceLanguageC11,
    LLVMDWARFSourceLanguageSwift,
    LLVMDWARFSourceLanguageJulia,
    LLVMDWARFSourceLanguageDylan,
    LLVMDWARFSourceLanguageC_plus_plus_14,
    LLVMDWARFSourceLanguageFortran03,
    LLVMDWARFSourceLanguageFortran08,
    LLVMDWARFSourceLanguageRenderScript,
    LLVMDWARFSourceLanguageBLISS,
    // Vendor extensions:
    LLVMDWARFSourceLanguageMips_Assembler,
    LLVMDWARFSourceLanguageGOOGLE_RenderScript,
    LLVMDWARFSourceLanguageBORLAND_Delphi,
}

/// The amount of debug information to emit.
#[repr(C)]
#[derive(Debug)]
pub enum LLVMDWARFEmissionKind {
    LLVMDWARFEmissionKindNone = 0,
    LLVMDWARFEmissionKindFull,
    LLVMDWARFEmissionKindLineTablesOnly,
}

#[repr(C)]
#[derive(Debug)]
pub enum LLVMMetadataKind {
    LLVMMDStringMetadataKind,
    LLVMConstantAsMetadataMetadataKind,
    LLVMLocalAsMetadataMetadataKind,
    LLVMDistinctMDOperandPlaceholderMetadataKind,
    LLVMMDTupleMetadataKind,
    LLVMDILocationMetadataKind,
    LLVMDIExpressionMetadataKind,
    LLVMDIGlobalVariableExpressionMetadataKind,
    LLVMGenericDINodeMetadataKind,
    LLVMDISubrangeMetadataKind,
    LLVMDIEnumeratorMetadataKind,
    LLVMDIBasicTypeMetadataKind,
    LLVMDIDerivedTypeMetadataKind,
    LLVMDICompositeTypeMetadataKind,
    LLVMDISubroutineTypeMetadataKind,
    LLVMDIFileMetadataKind,
    LLVMDICompileUnitMetadataKind,
    LLVMDISubprogramMetadataKind,
    LLVMDILexicalBlockMetadataKind,
    LLVMDILexicalBlockFileMetadataKind,
    LLVMDINamespaceMetadataKind,
    LLVMDIModuleMetadataKind,
    LLVMDITemplateTypeParameterMetadataKind,
    LLVMDITemplateValueParameterMetadataKind,
    LLVMDIGlobalVariableMetadataKind,
    LLVMDILocalVariableMetadataKind,
    LLVMDILabelMetadataKind,
    LLVMDIObjCPropertyMetadataKind,
    LLVMDIImportedEntityMetadataKind,
    LLVMDIMacroMetadataKind,
    LLVMDIMacroFileMetadataKind,
    LLVMDICommonBlockMetadataKind,
    LLVMDIStringTypeMetadataKind,
    LLVMDIGenericSubrangeMetadataKind,
    LLVMDIArgListMetadataKind,
}

pub type LLVMDWARFTypeEncoding = ::libc::c_uint;

#[repr(C)]
#[derive(Debug)]
pub enum LLVMDWARFMacinfoRecordType {
    LLVMDWARFMacinfoRecordTypeDefine = 0x01,
    LLVMDWARFMacinfoRecordTypeMacro = 0x02,
    LLVMDWARFMacinfoRecordTypeStartFile = 0x03,
    LLVMDWARFMacinfoRecordTypeEndFile = 0x04,
    LLVMDWARFMacinfoRecordTypeVendorExt = 0xff,
}

extern "C" {
    /// The current debug metadata version number.
    pub fn LLVMDebugMetadataVersion() -> ::libc::c_uint;
    /// The version of debug metadata that's present in the provided Module.
    pub fn LLVMGetModuleDebugMetadataVersion(Module: LLVMModuleRef) -> ::libc::c_uint;
    /// Strip debug info in the module if it exists.
    pub fn LLVMStripModuleDebugInfo(Module: LLVMModuleRef) -> LLVMBool;
    /// Construct a builder for a module, do not allow unresolved nodes.
    pub fn LLVMCreateDIBuilderDisallowUnresolved(M: LLVMModuleRef) -> LLVMDIBuilderRef;
    /// Construct a builder for a module and collect unresolved nodes.
    pub fn LLVMCreateDIBuilder(M: LLVMModuleRef) -> LLVMDIBuilderRef;
    /// Deallocate a builder and everything it owns.
    ///
    /// The builder must be finalized before this.
    pub fn LLVMDisposeDIBuilder(Builder: LLVMDIBuilderRef);
    /// Construct any deferred debug info descriptors.
    pub fn LLVMDIBuilderFinalize(Builder: LLVMDIBuilderRef);
    /// Finalize a specific subprogram.
    /// No new variables may be added to this subprogram afterwards.
    pub fn LLVMDIBuilderFinalizeSubprogram(Builder: LLVMDIBuilderRef, Subprogram: LLVMMetadataRef);
    pub fn LLVMDIBuilderCreateCompileUnit(
        Builder: LLVMDIBuilderRef,
        Lang: LLVMDWARFSourceLanguage,
        FileRef: LLVMMetadataRef,
        Producer: *const ::libc::c_char,
        ProducerLen: ::libc::size_t,
        isOptimized: LLVMBool,
        Flags: *const ::libc::c_char,
        FlagsLen: ::libc::size_t,
        RuntimeVer: ::libc::c_uint,
        SplitName: *const ::libc::c_char,
        SplitNameLen: ::libc::size_t,
        Kind: LLVMDWARFEmissionKind,
        DWOId: ::libc::c_uint,
        SplitDebugInlining: LLVMBool,
        DebugInfoForProfiling: LLVMBool,
        SysRoot: *const ::libc::c_char,
        SysRootLen: ::libc::size_t,
        SDK: *const ::libc::c_char,
        SDKLen: ::libc::size_t,
    ) -> LLVMMetadataRef;
    /// Create a file descriptor to hold debugging information for a file.
    pub fn LLVMDIBuilderCreateFile(
        Builder: LLVMDIBuilderRef,
        Filename: *const ::libc::c_char,
        FilenameLen: ::libc::size_t,
        Directory: *const ::libc::c_char,
        DirectoryLen: ::libc::size_t,
    ) -> LLVMMetadataRef;

    /// Creates a new descriptor for a module with the specified parent scope.
    pub fn LLVMDIBuilderCreateModule(
        Builder: LLVMDIBuilderRef,
        ParentScope: LLVMMetadataRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        ConfigMacros: *const ::libc::c_char,
        ConfigMacrosLen: ::libc::size_t,
        IncludePath: *const ::libc::c_char,
        IncludePathLen: ::libc::size_t,
        APINotesFile: *const ::libc::c_char,
        APINotesFileLen: ::libc::size_t,
    ) -> LLVMMetadataRef;

    /// Creates a new descriptor for a namespace with the specified parent scope.
    pub fn LLVMDIBuilderCreateNameSpace(
        Builder: LLVMDIBuilderRef,
        ParentScope: LLVMMetadataRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        ExportSymbols: LLVMBool,
    ) -> LLVMMetadataRef;

    /// Create a new descriptor for the specified subprogram.
    pub fn LLVMDIBuilderCreateFunction(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        LinkageName: *const ::libc::c_char,
        LinkageNameLen: ::libc::size_t,
        File: LLVMMetadataRef,
        LineNo: ::libc::c_uint,
        Ty: LLVMMetadataRef,
        IsLocalToUnit: LLVMBool,
        IsDefinition: LLVMBool,
        ScopeLine: ::libc::c_uint,
        Flags: LLVMDIFlags,
        IsOptimized: LLVMBool,
    ) -> LLVMMetadataRef;

    /// Create a descriptor for a lexical block with the specified parent context.
    pub fn LLVMDIBuilderCreateLexicalBlock(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        File: LLVMMetadataRef,
        Line: ::libc::c_uint,
        Column: ::libc::c_uint,
    ) -> LLVMMetadataRef;

    /// Create a descriptor for a lexical block with a new file attached.
    pub fn LLVMDIBuilderCreateLexicalBlockFile(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        File: LLVMMetadataRef,
        Discriminator: ::libc::c_uint,
    ) -> LLVMMetadataRef;

    /// Create a descriptor for an imported namespace. Suitable for e.g. C++ using declarations.
    pub fn LLVMDIBuilderCreateImportedModuleFromNamespace(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        NS: LLVMMetadataRef,
        File: LLVMMetadataRef,
        Line: ::libc::c_uint,
    ) -> LLVMMetadataRef;

    /// Create a descriptor for an imported module that aliases another imported entity descriptor.
    pub fn LLVMDIBuilderCreateImportedModuleFromAlias(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        ImportedEntity: LLVMMetadataRef,
        File: LLVMMetadataRef,
        Line: ::libc::c_uint,
        Elements: *mut LLVMMetadataRef,
        NumElements: ::libc::c_uint,
    ) -> LLVMMetadataRef;

    /// Create a descriptor for an imported module.
    pub fn LLVMDIBuilderCreateImportedModuleFromModule(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        M: LLVMMetadataRef,
        File: LLVMMetadataRef,
        Line: ::libc::c_uint,
        Elements: *mut LLVMMetadataRef,
        NumElements: ::libc::c_uint,
    ) -> LLVMMetadataRef;

    /// Create a descriptor for an imported function, type, or variable.
    ///
    /// Suitable for e.g. FORTRAN-style USE declarations.
    pub fn LLVMDIBuilderCreateImportedDeclaration(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        Decl: LLVMMetadataRef,
        File: LLVMMetadataRef,
        Line: ::libc::c_uint,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        Elements: *mut LLVMMetadataRef,
        NumElements: ::libc::c_uint,
    ) -> LLVMMetadataRef;

    /// Creates a new DebugLocation that describes a source location.
    pub fn LLVMDIBuilderCreateDebugLocation(
        Ctx: LLVMContextRef,
        Line: ::libc::c_uint,
        Column: ::libc::c_uint,
        Scope: LLVMMetadataRef,
        InlinedAt: LLVMMetadataRef,
    ) -> LLVMMetadataRef;

    /// Get the line number of this debug location.
    pub fn LLVMDILocationGetLine(Location: LLVMMetadataRef) -> ::libc::c_uint;

    /// Get the column number of this debug location.
    pub fn LLVMDILocationGetColumn(Location: LLVMMetadataRef) -> ::libc::c_uint;

    /// Get the local scope associated with this debug location.
    pub fn LLVMDILocationGetScope(Location: LLVMMetadataRef) -> LLVMMetadataRef;

    /// Get the "inline at" location associated with this debug location.
    pub fn LLVMDILocationGetInlinedAt(Location: LLVMMetadataRef) -> LLVMMetadataRef;

    /// Get the metadata of the file associated with a given scope.
    pub fn LLVMDIScopeGetFile(Scope: LLVMMetadataRef) -> LLVMMetadataRef;

    /// Get the directory of a given file.
    pub fn LLVMDIFileGetDirectory(
        File: LLVMMetadataRef,
        Len: *mut ::libc::c_uint,
    ) -> *const ::libc::c_char;

    /// Get the name of a given file.
    pub fn LLVMDIFileGetFilename(
        File: LLVMMetadataRef,
        Len: *mut ::libc::c_uint,
    ) -> *const ::libc::c_char;

    /// Get the source of a given file.
    pub fn LLVMDIFileGetSource(
        File: LLVMMetadataRef,
        Len: *mut ::libc::c_uint,
    ) -> *const ::libc::c_char;

    /// Create a type array.
    pub fn LLVMDIBuilderGetOrCreateTypeArray(
        Builder: LLVMDIBuilderRef,
        Data: *mut LLVMMetadataRef,
        NumElements: ::libc::size_t,
    ) -> LLVMMetadataRef;

    /// Create subroutine type.
    pub fn LLVMDIBuilderCreateSubroutineType(
        Builder: LLVMDIBuilderRef,
        File: LLVMMetadataRef,
        ParameterTypes: *mut LLVMMetadataRef,
        NumParameterTypes: ::libc::c_uint,
        Flags: LLVMDIFlags,
    ) -> LLVMMetadataRef;

    pub fn LLVMDIBuilderCreateMacro(
        Builder: LLVMDIBuilderRef,
        ParentMacroFile: LLVMMetadataRef,
        Line: ::libc::c_uint,
        RecordType: LLVMDWARFMacinfoRecordType,
        Name: *const ::libc::c_char,
        NameLen: usize,
        Value: *const ::libc::c_char,
        ValueLen: usize,
    ) -> LLVMMetadataRef;

    pub fn LLVMDIBuilderCreateTempMacroFile(
        Builder: LLVMDIBuilderRef,
        ParentMacroFile: LLVMMetadataRef,
        Line: ::libc::c_uint,
        File: LLVMMetadataRef,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for an enumerator.
    pub fn LLVMDIBuilderCreateEnumerator(
        Builder: LLVMDIBuilderRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        Value: i64,
        IsUnsigned: LLVMBool,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for an enumeration.
    pub fn LLVMDIBuilderCreateEnumerationType(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        File: LLVMMetadataRef,
        LineNumber: ::libc::c_uint,
        SizeInBits: u64,
        AlignInBits: u32,
        Elements: *mut LLVMMetadataRef,
        NumElements: ::libc::c_uint,
        ClassTy: LLVMMetadataRef,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for a union.
    pub fn LLVMDIBuilderCreateUnionType(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        File: LLVMMetadataRef,
        LineNumber: ::libc::c_uint,
        SizeInBits: u64,
        AlignInBits: u32,
        Flags: LLVMDIFlags,
        Elements: *mut LLVMMetadataRef,
        NumElements: ::libc::c_uint,
        RunTimeLang: ::libc::c_uint,
        UniqueId: *const ::libc::c_char,
        UniqueIdLen: ::libc::size_t,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for an array.
    pub fn LLVMDIBuilderCreateArrayType(
        Builder: LLVMDIBuilderRef,
        Size: u64,
        AlignInBits: u32,
        Ty: LLVMMetadataRef,
        Subscripts: *mut LLVMMetadataRef,
        NumSubscripts: ::libc::c_uint,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for a vector type.
    pub fn LLVMDIBuilderCreateVectorType(
        Builder: LLVMDIBuilderRef,
        Size: u64,
        AlignInBits: u32,
        Ty: LLVMMetadataRef,
        Subscripts: *mut LLVMMetadataRef,
        NumSubscripts: ::libc::c_uint,
    ) -> LLVMMetadataRef;

    /// Create a DWARF unspecified type.
    pub fn LLVMDIBuilderCreateUnspecifiedType(
        Builder: LLVMDIBuilderRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for a basic type.
    pub fn LLVMDIBuilderCreateBasicType(
        Builder: LLVMDIBuilderRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        SizeInBits: u64,
        Encoding: LLVMDWARFTypeEncoding,
        Flags: LLVMDIFlags,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for a pointer.
    pub fn LLVMDIBuilderCreatePointerType(
        Builder: LLVMDIBuilderRef,
        PointeeTy: LLVMMetadataRef,
        SizeInBits: u64,
        AlignInBits: u32,
        AddressSpace: ::libc::c_uint,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for a struct.
    pub fn LLVMDIBuilderCreateStructType(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        File: LLVMMetadataRef,
        LineNumber: ::libc::c_uint,
        SizeInBits: u64,
        AlignInBits: u32,
        Flags: LLVMDIFlags,
        DerivedFrom: LLVMMetadataRef,
        Elements: *mut LLVMMetadataRef,
        NumElements: ::libc::c_uint,
        RunTimeLang: ::libc::c_uint,
        VTableHolder: LLVMMetadataRef,
        UniqueId: *const ::libc::c_char,
        UniqueIdLen: ::libc::size_t,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for a member.
    pub fn LLVMDIBuilderCreateMemberType(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        File: LLVMMetadataRef,
        LineNo: ::libc::c_uint,
        SizeInBits: u64,
        AlignInBits: u32,
        OffsetInBits: u64,
        Flags: LLVMDIFlags,
        Ty: LLVMMetadataRef,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for a C++ static data member.
    pub fn LLVMDIBuilderCreateStaticMemberType(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        File: LLVMMetadataRef,
        LineNumber: ::libc::c_uint,
        Type: LLVMMetadataRef,
        Flags: LLVMDIFlags,
        ConstantVal: LLVMValueRef,
        AlignInBits: u32,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for a pointer to member.
    pub fn LLVMDIBuilderCreateMemberPointerType(
        Builder: LLVMDIBuilderRef,
        PointeeType: LLVMMetadataRef,
        ClassType: LLVMMetadataRef,
        SizeInBits: u64,
        AlignInBits: u32,
        Flags: LLVMDIFlags,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for Objective-C instance variable.
    pub fn LLVMDIBuilderCreateObjCIVar(
        Builder: LLVMDIBuilderRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        File: LLVMMetadataRef,
        LineNo: ::libc::c_uint,
        SizeInBits: u64,
        AlignInBits: u32,
        OffsetInBits: u64,
        Flags: LLVMDIFlags,
        Ty: LLVMMetadataRef,
        PropertyNode: LLVMMetadataRef,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for Objective-C property.
    pub fn LLVMDIBuilderCreateObjCProperty(
        Builder: LLVMDIBuilderRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        File: LLVMMetadataRef,
        LineNo: ::libc::c_uint,
        GetterName: *const ::libc::c_char,
        GetterNameLen: ::libc::size_t,
        SetterName: *const ::libc::c_char,
        SetterNameLen: ::libc::size_t,
        PropertyAttributes: ::libc::c_uint,
        Ty: LLVMMetadataRef,
    ) -> LLVMMetadataRef;

    /// Create a uniqued DIType* clone with FlagObjectPointer and FlagArtificial set.
    pub fn LLVMDIBuilderCreateObjectPointerType(
        Builder: LLVMDIBuilderRef,
        Type: LLVMMetadataRef,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for a qualified type, e.g. 'const int'.
    pub fn LLVMDIBuilderCreateQualifiedType(
        Builder: LLVMDIBuilderRef,
        Tag: ::libc::c_uint,
        Type: LLVMMetadataRef,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for a c++ style reference or rvalue reference type.
    pub fn LLVMDIBuilderCreateReferenceType(
        Builder: LLVMDIBuilderRef,
        Tag: ::libc::c_uint,
        Type: LLVMMetadataRef,
    ) -> LLVMMetadataRef;

    /// Create C++11 nullptr type.
    pub fn LLVMDIBuilderCreateNullPtrType(Builder: LLVMDIBuilderRef) -> LLVMMetadataRef;

    /// Create debugging information entry for a typedef.
    pub fn LLVMDIBuilderCreateTypedef(
        Builder: LLVMDIBuilderRef,
        Type: LLVMMetadataRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        File: LLVMMetadataRef,
        LineNo: ::libc::c_uint,
        Scope: LLVMMetadataRef,
        AlignInBits: u32,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry to establish inheritance relationship between two types.
    pub fn LLVMDIBuilderCreateInheritance(
        Builder: LLVMDIBuilderRef,
        Ty: LLVMMetadataRef,
        BaseTy: LLVMMetadataRef,
        BaseOffset: u64,
        VBPtrOffset: u32,
        Flags: LLVMDIFlags,
    ) -> LLVMMetadataRef;

    /// Create a permanent forward-declared type.
    pub fn LLVMDIBuilderCreateForwardDecl(
        Builder: LLVMDIBuilderRef,
        Tag: ::libc::c_uint,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        Scope: LLVMMetadataRef,
        File: LLVMMetadataRef,
        Line: ::libc::c_uint,
        RuntimeLang: ::libc::c_uint,
        SizeInBits: u64,
        AlignInBits: u32,
        UniqueIdentifier: *const ::libc::c_char,
        UniqueIdentifierLen: ::libc::size_t,
    ) -> LLVMMetadataRef;

    /// Create a temporary forward-declared type.
    pub fn LLVMDIBuilderCreateReplaceableCompositeType(
        Builder: LLVMDIBuilderRef,
        Tag: ::libc::c_uint,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        Scope: LLVMMetadataRef,
        File: LLVMMetadataRef,
        Line: ::libc::c_uint,
        RuntimeLang: ::libc::c_uint,
        SizeInBits: u64,
        AlignInBits: u32,
        Flags: LLVMDIFlags,
        UniqueIdentifier: *const ::libc::c_char,
        UniqueIdentifierLen: ::libc::size_t,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for a bit field member.
    pub fn LLVMDIBuilderCreateBitFieldMemberType(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        File: LLVMMetadataRef,
        LineNumber: ::libc::c_uint,
        SizeInBits: u64,
        OffsetInBits: u64,
        StorageOffsetInBits: u64,
        Flags: LLVMDIFlags,
        Type: LLVMMetadataRef,
    ) -> LLVMMetadataRef;

    /// Create debugging information entry for a class.
    pub fn LLVMDIBuilderCreateClassType(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        File: LLVMMetadataRef,
        LineNumber: ::libc::c_uint,
        SizeInBits: u64,
        AlignInBits: u32,
        OffsetInBits: u64,
        Flags: LLVMDIFlags,
        DerivedFrom: LLVMMetadataRef,
        Elements: *mut LLVMMetadataRef,
        NumElements: ::libc::c_uint,
        VTableHolder: LLVMMetadataRef,
        TemplateParamsNode: LLVMMetadataRef,
        UniqueIdentifier: *const ::libc::c_char,
        UniqueIdentifierLen: ::libc::size_t,
    ) -> LLVMMetadataRef;

    /// Create a uniqued DIType* clone with FlagArtificial set.
    pub fn LLVMDIBuilderCreateArtificialType(
        Builder: LLVMDIBuilderRef,
        Type: LLVMMetadataRef,
    ) -> LLVMMetadataRef;

    /// Get the name of this DIType.
    pub fn LLVMDITypeGetName(
        DType: LLVMMetadataRef,
        Length: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;

    /// Get the size of this DIType in bits.
    pub fn LLVMDITypeGetSizeInBits(DType: LLVMMetadataRef) -> u64;

    /// Get the offset of this DIType in bits.
    pub fn LLVMDITypeGetOffsetInBits(DType: LLVMMetadataRef) -> u64;

    /// Get the alignment of this DIType in bits.
    pub fn LLVMDITypeGetAlignInBits(DType: LLVMMetadataRef) -> u32;

    /// Get the source line where this DIType is declared.
    pub fn LLVMDITypeGetLine(DType: LLVMMetadataRef) -> ::libc::c_uint;

    /// Get the flags associated with this DIType.
    pub fn LLVMDITypeGetFlags(DType: LLVMMetadataRef) -> LLVMDIFlags;

    /// Create a descriptor for a value range.
    pub fn LLVMDIBuilderGetOrCreateSubrange(
        Builder: LLVMDIBuilderRef,
        LowerBound: i64,
        Count: i64,
    ) -> LLVMMetadataRef;

    /// Create an array of DI Nodes.
    pub fn LLVMDIBuilderGetOrCreateArray(
        Builder: LLVMDIBuilderRef,
        Data: *mut LLVMMetadataRef,
        NumElements: ::libc::size_t,
    ) -> LLVMMetadataRef;

    /// Create a new descriptor for the specified variable which has a complex
    pub fn LLVMDIBuilderCreateExpression(
        Builder: LLVMDIBuilderRef,
        Addr: *mut u64,
        Length: ::libc::size_t,
    ) -> LLVMMetadataRef;

    /// Create a new descriptor for the specified variable that does not have an
    pub fn LLVMDIBuilderCreateConstantValueExpression(
        Builder: LLVMDIBuilderRef,
        Value: u64,
    ) -> LLVMMetadataRef;

    /// Create a new descriptor for the specified variable.
    pub fn LLVMDIBuilderCreateGlobalVariableExpression(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        Linkage: *const ::libc::c_char,
        LinkLen: ::libc::size_t,
        File: LLVMMetadataRef,
        LineNo: ::libc::c_uint,
        Ty: LLVMMetadataRef,
        LocalToUnit: LLVMBool,
        Expr: LLVMMetadataRef,
        Decl: LLVMMetadataRef,
        AlignInBits: u32,
    ) -> LLVMMetadataRef;

    /// Retrieves the DIVariable associated with this global variable expression.
    pub fn LLVMDIGlobalVariableExpressionGetVariable(GVE: LLVMMetadataRef) -> LLVMMetadataRef;

    /// Retrieves the DIExpression associated with this global variable expression.
    pub fn LLVMDIGlobalVariableExpressionGetExpression(GVE: LLVMMetadataRef) -> LLVMMetadataRef;

    ///Get the metadata of the file associated with a given variable.
    pub fn LLVMDIVariableGetFile(Var: LLVMMetadataRef) -> LLVMMetadataRef;

    /// Get the metadata of the scope associated with a given variable.
    pub fn LLVMDIVariableGetScope(Var: LLVMMetadataRef) -> LLVMMetadataRef;

    /// Get the source line where this \c DIVariable is declared.
    pub fn LLVMDIVariableGetLine(Var: LLVMMetadataRef) -> ::libc::c_uint;

    /// Create a new temporary \c MDNode.  Suitable for use in constructing cyclic
    pub fn LLVMTemporaryMDNode(
        Ctx: LLVMContextRef,
        Data: *mut LLVMMetadataRef,
        NumElements: ::libc::size_t,
    ) -> LLVMMetadataRef;

    /// Deallocate a temporary node.
    pub fn LLVMDisposeTemporaryMDNode(TempNode: LLVMMetadataRef);

    /// Replace all uses of temporary metadata.
    pub fn LLVMMetadataReplaceAllUsesWith(
        TempTargetMetadata: LLVMMetadataRef,
        Replacement: LLVMMetadataRef,
    );

    /// Create a new descriptor for the specified global variable that is temporary
    pub fn LLVMDIBuilderCreateTempGlobalVariableFwdDecl(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        Linkage: *const ::libc::c_char,
        LnkLen: ::libc::size_t,
        File: LLVMMetadataRef,
        LineNo: ::libc::c_uint,
        Ty: LLVMMetadataRef,
        LocalToUnit: LLVMBool,
        Decl: LLVMMetadataRef,
        AlignInBits: u32,
    ) -> LLVMMetadataRef;

    /// Insert a new llvm.dbg.declare intrinsic call before the given instruction.
    pub fn LLVMDIBuilderInsertDeclareBefore(
        Builder: LLVMDIBuilderRef,
        Storage: LLVMValueRef,
        VarInfo: LLVMMetadataRef,
        Expr: LLVMMetadataRef,
        DebugLoc: LLVMMetadataRef,
        Instr: LLVMValueRef,
    ) -> LLVMValueRef;

    /// Insert a new llvm.dbg.declare intrinsic call at the end of the given basic block. If the basic block has a terminator instruction, the intrinsic is inserted before that terminator instruction.
    pub fn LLVMDIBuilderInsertDeclareAtEnd(
        Builder: LLVMDIBuilderRef,
        Storage: LLVMValueRef,
        VarInfo: LLVMMetadataRef,
        Expr: LLVMMetadataRef,
        DebugLoc: LLVMMetadataRef,
        Block: LLVMBasicBlockRef,
    ) -> LLVMValueRef;

    /// Insert a new llvm.dbg.value intrinsic call before the given instruction.
    pub fn LLVMDIBuilderInsertDbgValueBefore(
        Builder: LLVMDIBuilderRef,
        Val: LLVMValueRef,
        VarInfo: LLVMMetadataRef,
        Expr: LLVMMetadataRef,
        DebugLoc: LLVMMetadataRef,
        Instr: LLVMValueRef,
    ) -> LLVMValueRef;

    /// Insert a new llvm.dbg.value intrinsic call at the end of the given basic block. If the basic block has a terminator instruction, the intrinsic is inserted before that terminator instruction.
    pub fn LLVMDIBuilderInsertDbgValueAtEnd(
        Builder: LLVMDIBuilderRef,
        Val: LLVMValueRef,
        VarInfo: LLVMMetadataRef,
        Expr: LLVMMetadataRef,
        DebugLoc: LLVMMetadataRef,
        Block: LLVMBasicBlockRef,
    ) -> LLVMValueRef;

    /// Create a new descriptor for a local auto variable.
    pub fn LLVMDIBuilderCreateAutoVariable(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        File: LLVMMetadataRef,
        LineNo: ::libc::c_uint,
        Ty: LLVMMetadataRef,
        AlwaysPreserve: LLVMBool,
        Flags: LLVMDIFlags,
        AlignInBits: u32,
    ) -> LLVMMetadataRef;

    /// Create a new descriptor for a function parameter variable.
    pub fn LLVMDIBuilderCreateParameterVariable(
        Builder: LLVMDIBuilderRef,
        Scope: LLVMMetadataRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        ArgNo: ::libc::c_uint,
        File: LLVMMetadataRef,
        LineNo: ::libc::c_uint,
        Ty: LLVMMetadataRef,
        AlwaysPreserve: LLVMBool,
        Flags: LLVMDIFlags,
    ) -> LLVMMetadataRef;

    /// Get the metadata of the subprogram attached to a function.
    pub fn LLVMGetSubprogram(Func: LLVMValueRef) -> LLVMMetadataRef;

    /// Set the subprogram attached to a function.
    pub fn LLVMSetSubprogram(Func: LLVMValueRef, SP: LLVMMetadataRef);

    /// Get the line associated with a given subprogram.
    pub fn LLVMDISubprogramGetLine(Subprogram: LLVMMetadataRef) -> ::libc::c_uint;

    /// Get the debug location for the given instruction.
    pub fn LLVMInstructionGetDebugLoc(Inst: LLVMValueRef) -> LLVMMetadataRef;

    /// Set the debug location for the given instruction.
    pub fn LLVMInstructionSetDebugLoc(Inst: LLVMValueRef, Loc: LLVMMetadataRef);

    /// Obtain the enumerated type of a metadata instance.
    pub fn LLVMGetMetadataKind(Metadata: LLVMMetadataRef) -> LLVMMetadataKind;
}
