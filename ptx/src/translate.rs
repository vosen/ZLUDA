use crate::llvm::Message;
use crate::{ast, emit, llvm, raytracing};
use bit_vec::BitVec;
use hip_common::raytracing::VariablesBlock;
use hip_common::{kernel_metadata, CompilationMode};
use paste::paste;
pub use raytracing::Module as RaytracingModule;
use rustc_hash::{FxHashMap, FxHashSet};
use std::alloc::Layout;
use std::cell::RefCell;
use std::collections::{btree_map, hash_map, BTreeMap};
use std::ffi::{CStr, CString};
use std::num::NonZeroU32;
use std::{borrow::Cow, collections::BTreeSet, hash::Hash, iter, mem, rc::Rc};
use zluda_llvm::bit_writer::*;
use zluda_llvm::core::LLVMPrintModuleToString;

static ZLUDA_PTX_IMPL_AMD: &'static [u8] = include_bytes!("../lib/zluda_ptx_impl.bc");
const ZLUDA_PTX_PREFIX: &'static str = "__zluda_ptx_impl__";

macro_rules! derive_error {
    (enum $type_:ident {
        $( $variant:ident $(($underlying:ty))? ),+
    }) => {
        #[derive(Debug)]
        pub enum $type_ {
            $(
                $variant $(($underlying))?  ,
            )+
        }

        impl $type_ {
            $(
                paste! {
                    #[allow(dead_code)]
                    pub(crate) fn [<$variant:snake>] ( $(x: $underlying)? ) -> Self {
                        let result = Self :: $variant $((x as $underlying))?;
                        if cfg!(debug_assertions) {
                            panic!("{:?}", result);
                        } else {
                            result
                        }
                    }
                }
            )+
        }

        impl std::fmt::Display for $type_ {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "{:?}", self)
            }
        }
    }
}

derive_error! {
    enum TranslateError {
        UnknownSymbol,
        UntypedSymbol,
        MismatchedType,
        LLVM(llvm::Message),
        Unreachable,
        Todo,
        UnexpectedPattern,
        SymbolRedefinition
    }
}

impl std::error::Error for TranslateError {}

pub struct Module<'input> {
    pub(crate) llvm_module: llvm::Module,
    pub(crate) _llvm_context: llvm::Context,
    pub kernel_arguments: FxHashMap<String, Vec<Layout>>,
    pub bitcode_modules: Vec<&'static [u8]>,
    pub metadata: Metadata<'input>,
    pub compilation_mode: CompilationMode,
}

impl<'input> Module<'input> {
    pub fn get_bitcode_main(&self) -> llvm::MemoryBuffer {
        unsafe {
            llvm::MemoryBuffer::from_ffi(LLVMWriteBitcodeToMemoryBuffer(self.llvm_module.get()))
        }
    }

    pub fn get_llvm_text(&self) -> Message {
        unsafe { llvm::Message::from_ffi(LLVMPrintModuleToString(self.llvm_module.get())) }
    }

    pub fn get_bitcode_all<'a>(
        &'a self,
    ) -> impl Iterator<Item = (llvm::MemoryBuffer, &'a CStr)> + '_ {
        unsafe {
            let main_bc = llvm::MemoryBuffer::from_ffi(LLVMWriteBitcodeToMemoryBuffer(
                self.llvm_module.get(),
            ));
            let main_name = CStr::from_bytes_with_nul_unchecked(b"main\0");
            iter::once((main_bc, main_name)).chain(self.bitcode_modules.iter().map(|ptx_impl| {
                (
                    llvm::MemoryBuffer::create_no_copy(ptx_impl, false),
                    CStr::from_bytes_with_nul_unchecked(b"ptx_impl\0"),
                )
            }))
        }
    }

    pub fn get_bitcode_multi<'a>(
        mods: impl Iterator<Item = &'a Module<'input>>,
    ) -> Vec<(llvm::MemoryBuffer, CString)>
    where
        'input: 'a,
    {
        unsafe {
            let mut main_bcs = Vec::new();
            let mut bitcode_mods = Vec::new();
            for (idx, mod_) in mods.enumerate() {
                let main_bc = llvm::MemoryBuffer::from_ffi(LLVMWriteBitcodeToMemoryBuffer(
                    mod_.llvm_module.get(),
                ));
                main_bcs.push((
                    main_bc,
                    CString::from_vec_unchecked(format!("main_{}\0", idx).into_bytes()),
                ));
                for (sub_idx, bitcode) in mod_.bitcode_modules.iter().enumerate() {
                    bitcode_mods.push((
                        llvm::MemoryBuffer::create_no_copy(bitcode, false),
                        CString::from_vec_unchecked(
                            format!("ptx_impl_{}_{}\0", idx, sub_idx).into_bytes(),
                        ),
                    ));
                }
            }
            main_bcs.extend(bitcode_mods);
            main_bcs
        }
    }
}

pub struct Metadata<'input> {
    sm_version: u32,
    kernel_metadata: Vec<(Cow<'input, str>, Option<NonZeroU32>, Option<NonZeroU32>)>,
}

impl<'input> Metadata<'input> {
    pub fn empty() -> Self {
        Self {
            sm_version: 0,
            kernel_metadata: Vec::new(),
        }
    }

    pub fn join(self, other: &Self) -> Self {
        let sm_version = self.sm_version.max(other.sm_version);
        let mut kernel_metadata = self.kernel_metadata;
        kernel_metadata.extend(other.kernel_metadata.iter().cloned());
        Self {
            sm_version,
            kernel_metadata,
        }
    }

    pub fn to_elf_section(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let metadata = kernel_metadata::zluda::write(
            self.sm_version,
            self.kernel_metadata
                .iter()
                .map(|(name, min, max)| (&**name, *min, *max)),
        );
        emit::emit_section(
            hip_common::kernel_metadata::zluda::SECTION_STR,
            &metadata,
            &mut result,
        );
        result
    }
}

pub(crate) struct TranslationModule<'input, P: ast::ArgParams> {
    pub(crate) sm_version: u32,
    pub(crate) compilation_mode: CompilationMode,
    pub(crate) id_defs: IdNameMapBuilder<'input>,
    pub(crate) ptx_impl_imports: BTreeMap<String, Rc<RefCell<ast::MethodDeclaration<'input, Id>>>>,
    pub(crate) directives: Vec<TranslationDirective<'input, P>>,
}

impl<'input, P: ast::ArgParams> TranslationModule<'input, P> {
    fn new(compilation_mode: CompilationMode) -> Self {
        let id_defs = IdNameMapBuilder::new(IdGenerator::new());
        let ptx_impl_imports = BTreeMap::new();
        let directives = Vec::new();
        Self {
            compilation_mode,
            sm_version: 0,
            id_defs,
            ptx_impl_imports,
            directives,
        }
    }
}

// https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html#system-calls
// There is a bunch of functions like __assertfail and vprintf that must be
// replaced by imports from ZLUDA PTX implementation library
fn extract_builtin_functions<'input>(
    mut module: TranslationModule<'input, NormalizedArgParams>,
) -> TranslationModule<'input, NormalizedArgParams> {
    for directive in module.directives.iter_mut() {
        if let TranslationDirective::Method(TranslationMethod {
            source_name: Some(name),
            body: None,
            is_kernel: false,
            ..
        }) = directive
        {
            if is_builtin_function_name(&*name) {
                *name = Cow::Owned([ZLUDA_PTX_PREFIX, name].concat());
            }
        }
    }
    module
}

fn is_builtin_function_name(name: &str) -> bool {
    match name {
        "__assertfail" | "malloc" | "free" | "vprintf" => true,
        _ => false,
    }
}

// PTX linking rules are fairly convoluted. Here's my understanding:
// * For normal data (.global, .const) and non-kernel functions (.func)
//     * Symbol occurences must be equivalent under following rules:
//         * For data, symbol occurences must be of the same size. Alignment is ignored
//         * For functions, symbol occurences are strictly type-checked.
//           Number, type and alignmnt of input and return parameters must all match
//     * There are 3 classes of directives:
//         * Declarations. Only valid on functions
//             .func foobar();
//         * Definitions (complete definitions). Either data or functions
//             .func foobar() { ret; }
//             .global foobar .u32;
//           Both .global and .const are *always* initialized. If no explicit
//           initializer is present, they are zero-initialized
//         * Incomplete definitions. Data definitions using incomplete type. Only
//           known incomplete type is an array with at least one zero dimension
//             .extern .global foobar .b8 [];
//           Incomplete definitions *must* have an .extern linking specifier
//     * There can be only one definition (normal or incomplete) of a symbol
//       in a module
//     * There can be multiple declarations of a symbol in a module
//       * Declarations must all be the same: same linking specifier,
//         same argument list, same return list
//     * Data (.global and .const) is alwas accessible by cuModuleGetGlobal(...),
//       no matter the linking specifier. So this definition:
//         .global .u32 foobar1[1] = {1}
//       is not visible to othe modules during linking, but is accessible
//       by CUDA runtime after linking
//     * Non-kernel functions are never accessible by cuModuleGetGlobal(...)
//     * There are are four linking specifiers:
//         * (empty): static linking, a symbol is only visible inside a module
//             * For functions; separate functions with the same name in multiple, separate modules behave as expected
//             * For data, compiler selects the first symbol with the given name for access from cuModuleGetGlobal(...)
//             * This is only allowed linking specifier for local globals (globals defined inside function body),
//               which are not visible through cuModuleGetGlobal(...)
//         * .extern: means that symbol is completely-defined strictly in another module.
//           If the same symbol is completely-defined in the same module it's an error
//           It's legal to not resolve the declaration if it's unused
//           .extern is legal for:
//             * declarations and incomplete definitions
//             * normal definitions if all are true:
//                 * it's a data definition
//                 * it's a non-linking compilation
//                 * initializer is not present
//         * .visible: symbol is strong (overrides .weak) and globally visible.
//           Multiple .visible symbol occurences during linking compilation are illegal
//         * .weak: symbol is weak and globally visible.
//           If there's no strong symbol occurence, first weak symbol occurence gets selected
//         * .common: symbol is strong (overrides .weak) and globally visible with some additional rules:
//             * applies only to .global
//             * selects the first occurence from the largest symbol occurences
//             * explicit initializer is only allowed on symbol occurences with the largest size
fn resolve_linking<'a, 'input>(
    ast_modules: &'a [ast::Module<'input>],
    is_raytracing: bool,
) -> Result<ResolvedLinking<'input>, TranslateError> {
    let mut resolver = LinkingResolver::new(is_raytracing);
    for ast_module in ast_modules {
        resolver.start_module()?;
        for (index, directive) in ast_module.directives.iter().enumerate() {
            match directive {
                ast::Directive::Variable(linking, multivar) => {
                    resolver.on_data(
                        index,
                        *linking,
                        Cow::Borrowed(multivar.variable.name),
                        multivar.variable.state_space,
                        &multivar.variable.type_,
                    )?;
                }
                ast::Directive::Method(linking, method) => {
                    resolver.on_function(
                        index,
                        method.body.is_some(),
                        *linking,
                        &method.func_directive,
                    )?;
                }
            }
        }
    }
    resolver.close()
}

struct LinkingResolver<'a, 'input> {
    explicit_globals: FxHashMap<Cow<'input, str>, SymbolState<'a, 'input>>,
    implicit_globals: FxHashMap<Cow<'input, str>, (usize, usize)>,
    local_definitions: LocalDirectives<'a, 'input>,
    module_index: usize,
    is_raytracing: bool,
}

impl<'a, 'input> LinkingResolver<'a, 'input> {
    fn new(is_raytracing: bool) -> Self {
        Self {
            explicit_globals: FxHashMap::default(),
            implicit_globals: FxHashMap::default(),
            local_definitions: LocalDirectives::new(),
            module_index: 0,
            is_raytracing,
        }
    }

    fn start_module(&mut self) -> Result<(), TranslateError> {
        self.module_index += 1;
        mem::replace(&mut self.local_definitions, LocalDirectives::new()).check()
    }

    fn on_data(
        &mut self,
        location: usize,
        linking: ast::LinkingDirective,
        name: Cow<'input, str>,
        space: ast::StateSpace,
        type_: &ast::Type,
    ) -> Result<(), TranslateError> {
        if linking == ast::LinkingDirective::Common && space != ast::StateSpace::Global {
            return Err(TranslateError::SymbolRedefinition);
        }
        self.local_definitions.on_data(name.clone())?;
        let symbol = GlobalSymbol::Data {
            size: type_.layout().size(),
            space,
            type_: type_.clone(),
        };
        self.update_global_symbol(
            location,
            linking,
            name,
            symbol,
            space != ast::StateSpace::Shared,
        )
    }

    fn on_function(
        &mut self,
        location: usize,
        is_definition: bool,
        linking: ast::LinkingDirective,
        decl: &'a ast::MethodDeclaration<'input, &'input str>,
    ) -> Result<(), TranslateError> {
        // Common is legal only on .global
        if linking == ast::LinkingDirective::Common {
            return Err(TranslateError::SymbolRedefinition);
        }
        if is_definition {
            self.local_definitions
                .on_function_definition(linking, decl)?;
        } else {
            self.local_definitions
                .on_function_declaration(linking, decl)?;
        }
        let symbol = GlobalSymbol::Method {
            kernel: decl.name.is_kernel(),
            declaration: !is_definition,
            return_arguments: &decl.return_arguments,
            input_arguments: &decl.input_arguments,
        };
        self.update_global_symbol(
            location,
            linking,
            Cow::Borrowed(decl.name()),
            symbol,
            decl.name.is_kernel(),
        )
    }

    fn update_global_symbol(
        &mut self,
        new_location: usize,
        new_linking: ast::LinkingDirective,
        name: Cow<'input, str>,
        new_symbol: GlobalSymbol<'a, 'input>,
        implicit_global: bool,
    ) -> Result<(), TranslateError> {
        if new_linking == ast::LinkingDirective::None {
            if implicit_global {
                let will_be_shadowed = if let Some(global) = self.explicit_globals.get(&name) {
                    match global.symbol {
                        GlobalSymbol::Data { .. }
                        | GlobalSymbol::Method {
                            declaration: false, ..
                        } => true,
                        GlobalSymbol::Method {
                            declaration: true, ..
                        } => false,
                    }
                } else {
                    false
                };
                if !will_be_shadowed {
                    if let hash_map::Entry::Vacant(entry) = self.implicit_globals.entry(name) {
                        entry.insert((self.module_index, new_location));
                    }
                }
            }
            return Ok(());
        }
        let is_function_declaration = matches!(
            new_symbol,
            GlobalSymbol::Method {
                declaration: true,
                ..
            }
        );
        if !is_function_declaration {
            self.implicit_globals.remove(&name);
        }
        match self.explicit_globals.entry(name) {
            hash_map::Entry::Occupied(mut entry) => {
                let SymbolState {
                    module,
                    location,
                    linking,
                    symbol,
                } = entry.get_mut();
                let override_global = match (new_linking, *linking) {
                    (ast::LinkingDirective::None, _) | (_, ast::LinkingDirective::None) => {
                        return Err(TranslateError::unreachable())
                    }
                    (ast::LinkingDirective::Extern, _) => false,
                    (ast::LinkingDirective::Common, ast::LinkingDirective::Visible)
                    | (ast::LinkingDirective::Visible, ast::LinkingDirective::Common) => {
                        return Err(TranslateError::SymbolRedefinition);
                    }
                    (ast::LinkingDirective::Visible, ast::LinkingDirective::Visible) => {
                        // If it is in another module
                        if *module != self.module_index {
                            return Err(TranslateError::SymbolRedefinition);
                        } else {
                            !is_function_declaration
                        }
                    }
                    (
                        ast::LinkingDirective::Visible | ast::LinkingDirective::Common,
                        ast::LinkingDirective::Weak | ast::LinkingDirective::Extern,
                    ) => true,
                    (ast::LinkingDirective::Common, ast::LinkingDirective::Common) => {
                        if let (
                            GlobalSymbol::Data {
                                size,
                                space: ast::StateSpace::Global,
                                type_,
                            },
                            GlobalSymbol::Data {
                                size: new_size,
                                space: ast::StateSpace::Global,
                                type_: new_type,
                            },
                        ) = (symbol, new_symbol)
                        {
                            if new_size > *size {
                                *type_ = new_type;
                                *size = new_size;
                                *module = self.module_index;
                                *location = new_location;
                                *linking = new_linking;
                            }
                            return Ok(());
                        } else {
                            return Err(TranslateError::SymbolRedefinition);
                        }
                    }
                    (ast::LinkingDirective::Weak, ast::LinkingDirective::Extern) => true,
                    (ast::LinkingDirective::Weak, ast::LinkingDirective::Visible)
                    | (ast::LinkingDirective::Weak, ast::LinkingDirective::Common) => false,
                    (ast::LinkingDirective::Weak, ast::LinkingDirective::Weak) => match symbol {
                        GlobalSymbol::Method {
                            declaration: true, ..
                        } => {
                            if let GlobalSymbol::Method {
                                declaration: false, ..
                            } = new_symbol
                            {
                                true
                            } else {
                                false
                            }
                        }
                        _ => false,
                    },
                };
                if !new_symbol.is_compatible(symbol) {
                    return Err(TranslateError::SymbolRedefinition);
                }
                if override_global {
                    *symbol = new_symbol;
                    *module = self.module_index;
                    *location = new_location;
                    *linking = new_linking;
                }
            }
            hash_map::Entry::Vacant(entry) => {
                entry.insert(SymbolState {
                    module: self.module_index,
                    location: new_location,
                    linking: new_linking,
                    symbol: new_symbol,
                });
            }
        }
        Ok(())
    }

    fn close(self) -> Result<ResolvedLinking<'input>, TranslateError> {
        self.local_definitions.check()?;
        for (_, state) in self.explicit_globals.iter() {
            if state.linking == ast::LinkingDirective::Extern {
                match state.symbol {
                    GlobalSymbol::Data {
                        space: ast::StateSpace::Shared,
                        ..
                    }
                    | GlobalSymbol::Method { .. } => {}
                    GlobalSymbol::Data { size, .. } if size != 0 && self.module_index == 1 => {}
                    _ => return Err(TranslateError::SymbolRedefinition),
                }
            } else if !self.is_raytracing {
                if matches!(
                    state.symbol,
                    GlobalSymbol::Method {
                        declaration: true,
                        ..
                    }
                ) {
                    return Err(TranslateError::SymbolRedefinition);
                }
            }
        }
        let explicit_globals = self
            .explicit_globals
            .into_iter()
            .map(|(name, symbol)| {
                let type_ = match symbol.symbol {
                    GlobalSymbol::Data { type_, .. } => Some(type_),
                    GlobalSymbol::Method { .. } => None,
                };
                (name, (symbol.module, symbol.location, type_))
            })
            .collect();
        Ok(ResolvedLinking {
            explicit_globals,
            implicit_globals: self.implicit_globals,
        })
    }
}

struct ResolvedLinking<'input> {
    explicit_globals: FxHashMap<Cow<'input, str>, (usize, usize, Option<ast::Type>)>,
    implicit_globals: FxHashMap<Cow<'input, str>, (usize, usize)>,
}

impl<'input> ResolvedLinking<'input> {
    fn get_adjustment(
        &mut self,
        module: usize,
        directive: usize,
        name: Cow<'input, str>,
        linking: ast::LinkingDirective,
        explicit_initializer: bool,
    ) -> Result<VisibilityAdjustment, TranslateError> {
        if linking == ast::LinkingDirective::None {
            if self.implicit_globals.get(&name).copied() == Some((module, directive)) {
                Ok(VisibilityAdjustment::Global)
            } else {
                Ok(VisibilityAdjustment::Module)
            }
        } else {
            if let Some((global_module, global_directive, type_)) = self.explicit_globals.get(&name)
            {
                if module == *global_module && directive == *global_directive {
                    Ok(VisibilityAdjustment::Global)
                } else {
                    match linking {
                        ast::LinkingDirective::Extern
                        | ast::LinkingDirective::Weak
                        // Visible is possible and valid in case of function same-module declarations
                        | ast::LinkingDirective::Visible => {
                            Ok(VisibilityAdjustment::GlobalDeclaration(type_.clone()))
                        }
                        ast::LinkingDirective::Common => {
                            if explicit_initializer {
                                return Err(TranslateError::SymbolRedefinition);
                            }
                            Ok(VisibilityAdjustment::GlobalDeclaration(type_.clone()))
                        }
                        ast::LinkingDirective::None => {
                            Err(TranslateError::unreachable())
                        }
                    }
                }
            } else {
                Err(TranslateError::unreachable())
            }
        }
    }
}

enum VisibilityAdjustment {
    Global,
    Module,
    GlobalDeclaration(Option<ast::Type>),
}

struct LocalDirectives<'a, 'input> {
    directives: FxHashMap<Cow<'input, str>, LocalSymbol<'a, 'input>>,
}

impl<'a, 'input> LocalDirectives<'a, 'input> {
    fn new() -> Self {
        Self {
            directives: FxHashMap::default(),
        }
    }

    fn on_data(&mut self, name: Cow<'input, str>) -> Result<(), TranslateError> {
        match self.directives.entry(name) {
            hash_map::Entry::Occupied(_) => return Err(TranslateError::SymbolRedefinition),
            hash_map::Entry::Vacant(entry) => {
                entry.insert(LocalSymbol::Data);
            }
        }
        Ok(())
    }

    fn on_function_definition(
        &mut self,
        decl_linking: ast::LinkingDirective,
        decl: &'a ast::MethodDeclaration<'input, &'input str>,
    ) -> Result<(), TranslateError> {
        match self.directives.entry(Cow::Borrowed(decl.name())) {
            hash_map::Entry::Occupied(mut entry) => match entry.get_mut() {
                LocalSymbol::Data
                | LocalSymbol::Function {
                    has_definition: true,
                    ..
                } => return Err(TranslateError::SymbolRedefinition),
                LocalSymbol::Function {
                    kernel,
                    ref mut has_definition,
                    return_arguments,
                    input_arguments,
                    linking,
                } => {
                    if *kernel == decl.name.is_kernel() && decl_linking != *linking
                        || !is_variable_list_equivalent(&*decl.return_arguments, return_arguments)
                        || !is_variable_list_equivalent(&*decl.input_arguments, *input_arguments)
                    {
                        return Err(TranslateError::SymbolRedefinition);
                    }
                    *has_definition = true;
                }
            },
            hash_map::Entry::Vacant(entry) => {
                entry.insert(LocalSymbol::Function {
                    kernel: decl.name.is_kernel(),
                    has_definition: true,
                    linking: decl_linking,
                    return_arguments: &decl.return_arguments,
                    input_arguments: &decl.input_arguments,
                });
            }
        }
        Ok(())
    }

    fn on_function_declaration(
        &mut self,
        decl_linking: ast::LinkingDirective,
        decl: &'a ast::MethodDeclaration<'input, &'input str>,
    ) -> Result<(), TranslateError> {
        match self.directives.entry(Cow::Borrowed(decl.name())) {
            hash_map::Entry::Occupied(entry) => match entry.get() {
                LocalSymbol::Data => return Err(TranslateError::SymbolRedefinition),
                LocalSymbol::Function {
                    kernel,
                    has_definition: _,
                    linking,
                    return_arguments,
                    input_arguments,
                } => {
                    if *kernel == decl.name.is_kernel() && *linking != decl_linking
                        || !is_variable_list_equivalent(&*decl.return_arguments, return_arguments)
                        || !is_variable_list_equivalent(&*decl.input_arguments, *input_arguments)
                    {
                        return Err(TranslateError::SymbolRedefinition);
                    }
                }
            },
            hash_map::Entry::Vacant(entry) => {
                entry.insert(LocalSymbol::Function {
                    kernel: decl.name.is_kernel(),
                    has_definition: false,
                    linking: decl_linking,
                    return_arguments: &decl.return_arguments,
                    input_arguments: &decl.input_arguments,
                });
            }
        }
        Ok(())
    }

    // At a first glance this looks incomplete, but:
    // * Unresolved declarations at the global level are checked later,
    //   when we have symbols from all the modules
    // * We don't check unresolved data with incomplete definitions, because
    //   they are invalid anyway, if data is incomplete it must be extern
    //   and hence checked at the global level
    fn check(self) -> Result<(), TranslateError> {
        for (_, symbol) in self.directives {
            match symbol {
                LocalSymbol::Data => {}
                LocalSymbol::Function {
                    has_definition,
                    linking,
                    ..
                } => {
                    if linking == ast::LinkingDirective::None && !has_definition {
                        return Err(TranslateError::SymbolRedefinition);
                    }
                }
            }
        }
        Ok(())
    }
}

// Used to type-check declarations inside a module
enum LocalSymbol<'a, 'input> {
    Data,
    Function {
        kernel: bool,
        has_definition: bool,
        linking: ast::LinkingDirective,
        return_arguments: &'a [ast::VariableDeclaration<&'input str>],
        input_arguments: &'a [ast::VariableDeclaration<&'input str>],
    },
}

struct SymbolState<'a, 'input> {
    module: usize,
    location: usize,
    linking: ast::LinkingDirective,
    symbol: GlobalSymbol<'a, 'input>,
}

enum GlobalSymbol<'a, 'input> {
    Data {
        size: usize,
        space: ast::StateSpace,
        type_: ast::Type,
    },
    Method {
        kernel: bool,
        declaration: bool,
        return_arguments: &'a [ast::VariableDeclaration<&'input str>],
        input_arguments: &'a [ast::VariableDeclaration<&'input str>],
    },
}

impl<'a, 'input> GlobalSymbol<'a, 'input> {
    fn is_compatible(&self, old_symbol: &GlobalSymbol<'a, 'input>) -> bool {
        match (self, old_symbol) {
            (
                GlobalSymbol::Data {
                    size,
                    space,
                    type_: _,
                },
                GlobalSymbol::Data {
                    size: old_size,
                    space: old_space,
                    type_: _,
                },
            ) => (*size == *old_size || *old_size == 0 || *size == 0) && (space == old_space),
            (
                GlobalSymbol::Method {
                    kernel,
                    declaration: _,
                    return_arguments,
                    input_arguments,
                },
                GlobalSymbol::Method {
                    kernel: old_kernel,
                    declaration: _,
                    return_arguments: old_return_arguments,
                    input_arguments: old_input_arguments,
                },
            ) => {
                *kernel == *old_kernel
                    && is_variable_list_equivalent(return_arguments, old_return_arguments)
                    && is_variable_list_equivalent(input_arguments, old_input_arguments)
            }
            _ => false,
        }
    }
}

fn is_variable_list_equivalent<'a>(
    left: &[ast::VariableDeclaration<&'a str>],
    right: &[ast::VariableDeclaration<&'a str>],
) -> bool {
    fn equivalent_arguments<'a>(
        ast::VariableDeclaration {
            type_: l_type_,
            state_space: l_state_space,
            align: _,
            name: _,
        }: &ast::VariableDeclaration<&'a str>,
        ast::VariableDeclaration {
            type_: r_type_,
            state_space: r_state_space,
            align: _,
            name: _,
        }: &ast::VariableDeclaration<&'a str>,
    ) -> bool {
        l_type_ == r_type_ && l_state_space == r_state_space
    }
    let mut left = left.iter();
    let mut right = right.iter();
    loop {
        match (left.next(), right.next()) {
            (None, None) => break,
            (None, Some(_)) => return false,
            (Some(_), None) => return false,
            (Some(left), Some(right)) => {
                if !equivalent_arguments(left, right) {
                    return false;
                }
            }
        }
    }
    true
}

// This is actually three transformations in one:
// * Merge modules (linking-aware)
// * Replace all string identifiers with numeric identifiers
// * Convert predicates to branches
// After those two conversions we can start inserting and removing additional
// instructions freely
fn link_and_normalize_modules<'input>(
    asts: Vec<ast::Module<'input>>,
    module: TranslationModule<'input, NormalizedArgParams>,
    mut linking_resolver: ResolvedLinking<'input>,
) -> Result<
    (
        TranslationModule<'input, NormalizedArgParams>,
        FxHashMap<
            Id,
            (
                Vec<ast::VariableDeclaration<Id>>,
                Vec<ast::VariableDeclaration<Id>>,
            ),
        >,
    ),
    TranslateError,
> {
    let mut functions = get_existing_methods(&*module.directives);
    let mut id_defs = module.id_defs;
    let ptx_impl_imports = module.ptx_impl_imports;
    let mut directives = module.directives;
    let mut sm_version = 0;
    let mut string_resolver = StringIdResolver::new(&mut id_defs, &directives)?;
    for (mut module_index, ast) in asts.into_iter().enumerate() {
        module_index += 1;
        sm_version = sm_version.max(ast.sm_version);
        let mut module_scope = string_resolver.start_module();
        for (directive_index, directive) in ast.directives.into_iter().enumerate() {
            match directive {
                ast::Directive::Method(linking_directive, method) => {
                    directives.push(TranslationDirective::Method(normalize_method(
                        &mut linking_resolver,
                        &mut functions,
                        (module_index, directive_index),
                        &mut module_scope,
                        linking_directive,
                        method,
                    )?));
                }
                ast::Directive::Variable(mut linking_directive, vars) => {
                    expand_multivariable2(
                        &mut module_scope,
                        iter::once(vars),
                        |scope, align, type_, space, name, mut initializer| {
                            let linking_adjustment = linking_resolver.get_adjustment(
                                module_index,
                                directive_index,
                                name.clone(),
                                linking_directive,
                                initializer.is_some(),
                            )?;
                            let (has_global_name, has_body, type_override) =
                                match linking_adjustment {
                                    VisibilityAdjustment::Global => (true, true, None),
                                    VisibilityAdjustment::Module => (false, true, None),
                                    VisibilityAdjustment::GlobalDeclaration(type_override) => {
                                        (true, false, type_override)
                                    }
                                };
                            let type_ = type_override.unwrap_or_else(|| type_.clone());
                            let compiled_name = if has_global_name {
                                Some(name.clone())
                            } else {
                                None
                            };
                            if !has_body {
                                linking_directive = ast::LinkingDirective::Extern;
                                initializer = None;
                            }
                            directives.push(TranslationDirective::Variable(
                                linking_directive,
                                compiled_name,
                                scope.add_or_get_module_variable(
                                    name,
                                    has_global_name,
                                    type_,
                                    space,
                                    align,
                                    initializer,
                                )?,
                            ));
                            Ok(())
                        },
                    )?;
                }
            }
        }
    }
    Ok((
        TranslationModule {
            compilation_mode: module.compilation_mode,
            sm_version,
            id_defs,
            ptx_impl_imports,
            directives,
        },
        functions,
    ))
}

fn get_existing_methods<'input, P: ast::ArgParams<Id = Id>>(
    directives: &[TranslationDirective<'input, P>],
) -> FxHashMap<
    Id,
    (
        Vec<ast::VariableDeclaration<Id>>,
        Vec<ast::VariableDeclaration<Id>>,
    ),
> {
    let mut result = FxHashMap::default();
    for directive in directives {
        match directive {
            TranslationDirective::Variable(..) => continue,
            TranslationDirective::Method(method) => {
                result.insert(
                    method.name,
                    (
                        method.return_arguments.clone(),
                        method.input_arguments.clone(),
                    ),
                );
            }
        }
    }
    result
}

fn normalize_method<'a, 'b, 'input>(
    linking_resolver: &mut ResolvedLinking<'input>,
    function_decls: &mut FxHashMap<
        Id,
        (
            Vec<ast::VariableDeclaration<Id>>,
            Vec<ast::VariableDeclaration<Id>>,
        ),
    >,
    (module, directive): (usize, usize),
    module_scope: &mut StringIdResolverScope<'a, 'b, 'input>,
    linking_directive: ast::LinkingDirective,
    method: ast::Function<'input, &'input str, ast::Statement<ast::ParsedArgParams<'input>>>,
) -> Result<TranslationMethod<'input, NormalizedArgParams>, TranslateError> {
    let is_kernel = method.func_directive.name.is_kernel();
    let linking_adjustment = linking_resolver.get_adjustment(
        module,
        directive,
        Cow::Borrowed(method.func_directive.name()),
        linking_directive,
        false,
    )?;
    let (has_global_name, has_body) = match linking_adjustment {
        VisibilityAdjustment::Global => (true, true),
        VisibilityAdjustment::Module => (false, true),
        VisibilityAdjustment::GlobalDeclaration(_) => (true, false),
    };
    let name =
        module_scope.add_or_get_at_module_level(method.func_directive.name(), has_global_name)?;
    let mut fn_scope = module_scope.start_scope();
    let return_arguments =
        normalize_method_params(&mut fn_scope, &*method.func_directive.return_arguments)?;
    let input_arguments =
        normalize_method_params(&mut fn_scope, &*method.func_directive.input_arguments)?;
    if !is_kernel {
        if let hash_map::Entry::Vacant(entry) = function_decls.entry(name) {
            entry.insert((return_arguments.clone(), input_arguments.clone()));
        }
    }
    let source_name = if has_global_name {
        Some(Cow::Borrowed(method.func_directive.name()))
    } else {
        None
    };
    let body = if has_body {
        method
            .body
            .map(|body| {
                let body = normalize_identifiers2(&mut fn_scope, body)?;
                normalize_predicates2(&mut fn_scope, body)
            })
            .transpose()?
    } else {
        None
    };
    Ok(TranslationMethod {
        return_arguments,
        name,
        input_arguments,
        body,
        tuning: method.tuning,
        is_kernel,
        source_name,
        special_raytracing_linking: false,
    })
}

fn normalize_method_params<'a, 'b, 'input>(
    fn_scope: &mut StringIdResolverScope<'a, 'b, 'input>,
    args: &[ast::VariableDeclaration<&'input str>],
) -> Result<Vec<ast::VariableDeclaration<Id>>, TranslateError> {
    args.iter()
        .map(|a| {
            Ok(ast::VariableDeclaration {
                name: fn_scope.add_variable_checked(
                    a.name,
                    a.type_.clone(),
                    a.state_space,
                    a.align,
                )?,
                type_: a.type_.clone(),
                state_space: a.state_space,
                align: a.align,
            })
        })
        .collect()
}

fn normalize_identifiers2<'a, 'b, 'input>(
    scope: &mut StringIdResolverScope<'a, 'b, 'input>,
    func: Vec<ast::Statement<ast::ParsedArgParams<'input>>>,
) -> Result<Vec<NormalizedStatement>, TranslateError> {
    gather_labels_in_scope(scope, &func)?;
    let mut result = Vec::with_capacity(func.len());
    for statement in func {
        match statement {
            ast::Statement::Block(block) => {
                let mut scope = scope.start_scope();
                result.extend(normalize_identifiers2(&mut scope, block)?);
            }
            ast::Statement::Label(name) => {
                result.push(Statement::Label(scope.get_id_in_function_scopes(name)?))
            }
            ast::Statement::Instruction(p, i) => result.push(Statement::Instruction((
                p.map(|p| p.map_variable(&mut |id| scope.get_id_in_module_scopes(id)))
                    .transpose()?,
                i.map_variable(&mut |id| scope.get_id_in_module_scopes(id))?,
            ))),
            ast::Statement::Variable(vars) => {
                expand_multivariable2(
                    scope,
                    vars.into_iter(),
                    |scope, align, type_, space, name, initializer| {
                        result.push(Statement::Variable(scope.register_variable(
                            name,
                            type_.clone(),
                            space,
                            align,
                            initializer,
                        )?));
                        Ok(())
                    },
                )?;
            }
            ast::Statement::Callprototype(proto) => {
                let name = scope.add_untyped_checked(proto.name)?;
                scope.0.module.globals.function_prototypes.insert(
                    name,
                    Callprototype {
                        return_arguments: proto.return_arguments,
                        input_arguments: proto.input_arguments,
                    },
                );
            }
        }
    }
    Ok(result)
}

fn expand_multivariable2<'a, 'b, 'input>(
    scope: &mut StringIdResolverScope<'a, 'b, 'input>,
    vars: impl Iterator<Item = ast::MultiVariableDefinition<&'input str>>,
    mut inserter: impl FnMut(
        &mut StringIdResolverScope<'a, 'b, 'input>,
        Option<u32>,
        &ast::Type,
        ast::StateSpace,
        Cow<'input, str>,
        Option<ast::Initializer<Id>>,
    ) -> Result<(), TranslateError>,
) -> Result<(), TranslateError> {
    for var in vars {
        let initializer = match var.suffix {
            Some(ast::DeclarationSuffix::Count(count)) => {
                for offset in 0..count {
                    let name = Cow::Owned(format!("{}{}", var.variable.name, offset));
                    inserter(
                        scope,
                        var.variable.align,
                        &var.variable.type_,
                        var.variable.state_space,
                        name,
                        None,
                    )?;
                }
                return Ok(());
            }
            Some(ast::DeclarationSuffix::Initializer(init)) => {
                Some(expand_initializer2(scope, init)?)
            }
            None => None,
        };
        let name = Cow::Borrowed(var.variable.name);
        inserter(
            scope,
            var.variable.align,
            &var.variable.type_,
            var.variable.state_space,
            name,
            initializer,
        )?;
    }
    Ok(())
}

fn expand_initializer2<'a, 'b, 'input>(
    scope: &mut StringIdResolverScope<'a, 'b, 'input>,
    init: ast::Initializer<&'input str>,
) -> Result<ast::Initializer<Id>, TranslateError> {
    Ok(match init {
        ast::Initializer::Constant(c) => ast::Initializer::Constant(c),
        ast::Initializer::Global(g, type_) => {
            ast::Initializer::Global(scope.get_id_in_module_scope(g)?, type_)
        }
        ast::Initializer::GenericGlobal(g, type_) => {
            ast::Initializer::GenericGlobal(scope.get_id_in_module_scope(g)?, type_)
        }
        ast::Initializer::Add(add) => {
            let (init1, init2) = *add;
            ast::Initializer::Add(Box::new((
                expand_initializer2(scope, init1)?,
                expand_initializer2(scope, init2)?,
            )))
        }
        ast::Initializer::Array(array) => ast::Initializer::Array(
            array
                .into_iter()
                .map(|init| expand_initializer2(scope, init))
                .collect::<Result<Vec<_>, _>>()?,
        ),
    })
}

fn normalize_predicates2<'a, 'b, 'input>(
    scope: &mut StringIdResolverScope<'a, 'b, 'input>,
    func: Vec<NormalizedStatement>,
) -> Result<Vec<UnconditionalStatement>, TranslateError> {
    let mut result = Vec::with_capacity(func.len());
    for s in func {
        match s {
            Statement::Label(id) => result.push(Statement::Label(id)),
            Statement::Instruction((pred, inst)) => {
                if let Some(pred) = pred {
                    let if_true = scope.new_untyped();
                    let if_false = scope.new_untyped();
                    let folded_bra = match &inst {
                        ast::Instruction::Bra(_, arg) => Some(arg.src),
                        _ => None,
                    };
                    let mut branch = BrachCondition {
                        predicate: pred.label,
                        if_true: folded_bra.unwrap_or(if_true),
                        if_false,
                    };
                    if pred.not {
                        std::mem::swap(&mut branch.if_true, &mut branch.if_false);
                    }
                    result.push(Statement::Conditional(branch));
                    if folded_bra.is_none() {
                        result.push(Statement::Label(if_true));
                        result.push(Statement::Instruction(inst));
                    }
                    result.push(Statement::Label(if_false));
                } else {
                    result.push(Statement::Instruction(inst));
                }
            }
            Statement::Variable(var) => result.push(Statement::Variable(var)),
            // Blocks are flattened when resolving ids
            _ => return Err(TranslateError::unreachable()),
        }
    }
    Ok(result)
}

// Instructions can reference labels that are declared later on so
// we gather ids of labels ahead of time
fn gather_labels_in_scope<'a, 'b, 'input>(
    scope: &mut StringIdResolverScope<'a, 'b, 'input>,
    func: &[ast::Statement<ast::ParsedArgParams<'input>>],
) -> Result<(), TranslateError> {
    for s in func.iter() {
        // Instructions can reference labels that are declared later so
        // we gather ids of labels ahead of time
        if let ast::Statement::Label(id) = s {
            scope.add_untyped_checked(*id)?;
        }
    }
    Ok(())
}

fn resolve_instruction_types<'input>(
    mut module: TranslationModule<'input, NormalizedArgParams>,
    function_decls: FxHashMap<
        Id,
        (
            Vec<ast::VariableDeclaration<Id>>,
            Vec<ast::VariableDeclaration<Id>>,
        ),
    >,
) -> Result<TranslationModule<'input, TypedArgParams>, TranslateError> {
    let id_defs = &mut module.id_defs;
    let directives = module
        .directives
        .into_iter()
        .map(|directive| {
            Ok(match directive {
                TranslationDirective::Variable(linking, compiled_name, var) => {
                    TranslationDirective::Variable(
                        linking,
                        compiled_name,
                        resolve_initializers(id_defs, var)?,
                    )
                }
                TranslationDirective::Method(method) => {
                    let body = match method.body {
                        Some(body) => Some(resolve_instruction_types_method(
                            id_defs,
                            &function_decls,
                            body,
                        )?),
                        None => None,
                    };
                    TranslationDirective::Method(TranslationMethod {
                        return_arguments: method.return_arguments,
                        name: method.name,
                        input_arguments: method.input_arguments,
                        body,
                        tuning: method.tuning,
                        is_kernel: method.is_kernel,
                        source_name: method.source_name,
                        special_raytracing_linking: method.special_raytracing_linking,
                    })
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TranslationModule {
        compilation_mode: module.compilation_mode,
        sm_version: module.sm_version,
        directives,
        id_defs: module.id_defs,
        ptx_impl_imports: module.ptx_impl_imports,
    })
}

fn resolve_instruction_types_method<'input>(
    id_defs: &mut IdNameMapBuilder<'input>,
    function_decls: &FxHashMap<
        Id,
        (
            Vec<ast::VariableDeclaration<Id>>,
            Vec<ast::VariableDeclaration<Id>>,
        ),
    >,
    fn_body: Vec<UnconditionalStatement>,
) -> Result<Vec<TypedStatement>, TranslateError> {
    let mut result = Vec::<TypedStatement>::with_capacity(fn_body.len());
    let mut constants = KernelConstantsVisitor::new();
    for statement in fn_body {
        match statement {
            Statement::Instruction(inst) => match inst {
                // TODO: Replace this with proper constant propagation
                ast::Instruction::PrmtSlow { control, arg } => {
                    let inst = if let Some(control) = constants.try_get_constant(control) {
                        ast::Instruction::Prmt { control, arg }
                    } else {
                        ast::Instruction::PrmtSlow { control, arg }
                    };
                    let mut visitor =
                        VectorRepackVisitor::new(&mut constants, &mut result, id_defs);
                    let reresolved_call = inst.visit(&mut visitor)?;
                    visitor.func.push(reresolved_call);
                    visitor.func.extend(visitor.post_stmts);
                }
                ast::Instruction::Sust(mut details, args) => {
                    if let ast::Operand::Reg(image) = args.image {
                        let (image_type, _, _, _) = id_defs.get_typed(image)?;
                        if matches!(image_type, ast::Type::Surfref) {
                            details.direct = true;
                        }
                    }
                    let mut visitor =
                        VectorRepackVisitor::new(&mut constants, &mut result, id_defs);
                    let reresolved_call =
                        ast::Instruction::Sust(details, args).visit(&mut visitor)?;
                    visitor.func.push(reresolved_call);
                    visitor.func.extend(visitor.post_stmts);
                }
                ast::Instruction::Suld(mut details, args) => {
                    if let ast::Operand::Reg(image) = args.image {
                        let (image_type, _, _, _) = id_defs.get_typed(image)?;
                        if matches!(image_type, ast::Type::Surfref) {
                            details.direct = true;
                        }
                    }
                    let mut visitor =
                        VectorRepackVisitor::new(&mut constants, &mut result, id_defs);
                    let reresolved_call =
                        ast::Instruction::Suld(details, args).visit(&mut visitor)?;
                    visitor.func.push(reresolved_call);
                    visitor.func.extend(visitor.post_stmts);
                }
                ast::Instruction::Tex(mut details, args) => {
                    if let ast::Operand::Reg(image) = args.image {
                        let (image_type, _, _, _) = id_defs.get_typed(image)?;
                        if matches!(image_type, ast::Type::Texref) {
                            details.direct = true;
                        }
                    }
                    let mut visitor =
                        VectorRepackVisitor::new(&mut constants, &mut result, id_defs);
                    let reresolved_call =
                        ast::Instruction::Tex(details, args).visit(&mut visitor)?;
                    visitor.func.push(reresolved_call);
                    visitor.func.extend(visitor.post_stmts);
                }
                ast::Instruction::Mov(
                    mov,
                    ast::Arg2Mov {
                        dst: ast::Operand::Reg(dst_reg),
                        src: ast::Operand::Reg(src_reg),
                    },
                ) if function_decls.contains_key(&src_reg) => {
                    if mov.typ != ast::Type::Scalar(ast::ScalarType::U64) {
                        return Err(TranslateError::mismatched_type());
                    }
                    result.push(TypedStatement::FunctionPointer(FunctionPointerDetails {
                        dst: dst_reg,
                        src: src_reg,
                    }));
                }
                ast::Instruction::Mov(
                    ast::MovDetails {
                        typ: ast::Type::Scalar(type_),
                        ..
                    },
                    ast::Arg2Mov {
                        dst: ast::Operand::Reg(dst_reg),
                        src: ast::Operand::Imm(src),
                    },
                ) if type_.size_of() >= 2 && type_.is_integer() => {
                    constants.insert(
                        dst_reg,
                        Some(src.as_u16().ok_or_else(TranslateError::unreachable)?),
                    )?;
                    let mut noop_visitor = PassthroughVisitor;
                    let mut visitor =
                        VectorRepackVisitor::new(&mut noop_visitor, &mut result, id_defs);
                    let instruction = Statement::Instruction(inst.map(&mut visitor)?);
                    visitor.func.push(instruction);
                    visitor.func.extend(visitor.post_stmts);
                }
                ast::Instruction::Call(call) => {
                    let resolved_call = match function_decls.get(&call.func) {
                        Some((return_args, input_args)) => {
                            ResolvedCall::from_declaration(call, return_args, input_args)?
                        }
                        None => {
                            let callproto_name =
                                call.prototype.ok_or_else(TranslateError::unreachable)?;
                            let callproto = id_defs
                                .globals
                                .function_prototypes
                                .get(&callproto_name)
                                .ok_or_else(TranslateError::unreachable)?;
                            ResolvedCall::from_callprototype(call, callproto)?
                        }
                    };
                    let mut visitor =
                        VectorRepackVisitor::new(&mut constants, &mut result, id_defs);
                    let reresolved_call = resolved_call.visit(&mut visitor)?;
                    visitor.func.push(reresolved_call);
                    visitor.func.extend(visitor.post_stmts);
                }
                inst => {
                    let mut visitor =
                        VectorRepackVisitor::new(&mut constants, &mut result, id_defs);
                    let instruction = Statement::Instruction(inst.map(&mut visitor)?);
                    visitor.func.push(instruction);
                    visitor.func.extend(visitor.post_stmts);
                }
            },
            Statement::Label(i) => result.push(Statement::Label(i)),
            Statement::Variable(v) => {
                result.push(Statement::Variable(resolve_initializers(id_defs, v)?))
            }
            Statement::Conditional(c) => result.push(Statement::Conditional(c)),
            _ => return Err(TranslateError::unreachable()),
        }
    }
    Ok(result)
}

fn resolve_initializers<'input>(
    id_defs: &mut IdNameMapBuilder<'input>,
    mut v: Variable,
) -> Result<Variable, TranslateError> {
    fn resolve_initializer_impl<'input>(
        id_defs: &mut IdNameMapBuilder<'input>,
        init: &mut ast::Initializer<Id>,
    ) -> Result<(), TranslateError> {
        match init {
            ast::Initializer::Constant(_) => {}
            ast::Initializer::Global(name, type_)
            | ast::Initializer::GenericGlobal(name, type_) => {
                let (src_type, _, _, _) = id_defs.get_typed(*name)?;
                *type_ = src_type;
            }
            ast::Initializer::Add(subinit) => {
                resolve_initializer_impl(id_defs, &mut (*subinit).0)?;
                resolve_initializer_impl(id_defs, &mut (*subinit).1)?;
            }
            ast::Initializer::Array(inits) => {
                for init in inits.iter_mut() {
                    resolve_initializer_impl(id_defs, init)?;
                }
            }
        }
        Ok(())
    }
    if let Some(ref mut init) = v.initializer {
        resolve_initializer_impl(id_defs, init)?;
    }
    Ok(v)
}

// TODO: All this garbage should be replaced with proper constant propagation or
// at least ability to visit statements without moving them
struct KernelConstantsVisitor {
    constant_candidates: FxHashMap<Id, ConstantCandidate>,
}

struct ConstantCandidate {
    value: u16,
    used: bool,
    not_const: bool,
}

impl KernelConstantsVisitor {
    fn new() -> Self {
        Self {
            constant_candidates: FxHashMap::default(),
        }
    }

    fn insert(&mut self, id: Id, value: Option<u16>) -> Result<(), TranslateError> {
        match self.constant_candidates.entry(id) {
            hash_map::Entry::Occupied(mut entry) => {
                let candidate = entry.get_mut();
                if candidate.used {
                    return Err(TranslateError::unexpected_pattern());
                }
                candidate.not_const = true;
            }
            hash_map::Entry::Vacant(entry) => {
                entry.insert(ConstantCandidate {
                    value: value.unwrap_or(u16::MAX),
                    used: false,
                    not_const: value.is_none(),
                });
            }
        }
        Ok(())
    }

    fn try_get_constant(&mut self, id: Id) -> Option<u16> {
        self.constant_candidates.get_mut(&id).and_then(|candidate| {
            if candidate.not_const {
                return None;
            }
            candidate.used = true;
            Some(candidate.value)
        })
    }
}

impl ArgumentMapVisitor<TypedArgParams, TypedArgParams> for KernelConstantsVisitor {
    fn id(
        &mut self,
        desc: ArgumentDescriptor<Id>,
        _: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<Id, TranslateError> {
        if desc.is_dst {
            self.insert(desc.op, None)?;
        }
        Ok(desc.op)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<TypedOperand>,
        _: &ast::Type,
        _: ast::StateSpace,
    ) -> Result<TypedOperand, TranslateError> {
        if desc.is_dst {
            if let TypedOperand::Reg(op) = desc.op {
                self.insert(op, None)?;
            }
        }
        Ok(desc.op)
    }
}

struct PassthroughVisitor;

impl<P: ArgParamsEx> ArgumentMapVisitor<P, P> for PassthroughVisitor {
    fn id(
        &mut self,
        desc: ArgumentDescriptor<P::Id>,
        _: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<P::Id, TranslateError> {
        Ok(desc.op)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<P::Operand>,
        _: &ast::Type,
        _: ast::StateSpace,
    ) -> Result<P::Operand, TranslateError> {
        Ok(desc.op)
    }
}

fn convert_methods<'input, From: ast::ArgParams<Id = Id>, To: ast::ArgParams<Id = Id>>(
    mut module: TranslationModule<'input, From>,
    mut mapper: impl FnMut(
        CompilationMode,
        &mut IdNameMapBuilder<'input>,
        &mut AdditionalFunctionDeclarations,
        &mut [ast::VariableDeclaration<From::Id>],
        &mut [ast::VariableDeclaration<From::Id>],
        bool,
        Vec<Statement<ast::Instruction<From>, From>>,
    ) -> Result<Vec<Statement<ast::Instruction<To>, To>>, TranslateError>,
) -> Result<TranslationModule<'input, To>, TranslateError> {
    let compilation_mode = module.compilation_mode;
    let id_defs = &mut module.id_defs;
    let mut additional_declarations = AdditionalFunctionDeclarations::new();
    let post_declarations_directives = module
        .directives
        .into_iter()
        .map(|directive| {
            Ok(match directive {
                TranslationDirective::Method(mut method) => {
                    let body = match method.body {
                        Some(body) => Some(mapper(
                            compilation_mode,
                            id_defs,
                            &mut additional_declarations,
                            &mut method.return_arguments,
                            &mut method.input_arguments,
                            method.is_kernel,
                            body,
                        )?),
                        None => None,
                    };
                    TranslationDirective::Method(TranslationMethod {
                        return_arguments: method.return_arguments,
                        name: method.name,
                        input_arguments: method.input_arguments,
                        body,
                        tuning: method.tuning,
                        is_kernel: method.is_kernel,
                        source_name: method.source_name,
                        special_raytracing_linking: method.special_raytracing_linking,
                    })
                }
                TranslationDirective::Variable(linking, compiled_name, var) => {
                    TranslationDirective::Variable(linking, compiled_name, var)
                }
            })
        })
        .collect::<Result<Vec<_>, TranslateError>>()?;
    let mut directives = Vec::with_capacity(post_declarations_directives.len());
    additional_declarations.flush(&mut directives);
    directives.extend(post_declarations_directives);
    Ok(TranslationModule {
        compilation_mode: module.compilation_mode,
        sm_version: module.sm_version,
        directives: directives,
        id_defs: module.id_defs,
        ptx_impl_imports: module.ptx_impl_imports,
    })
}

fn convert_methods_simple<'input, From: ast::ArgParams<Id = Id>, To: ast::ArgParams<Id = Id>>(
    module: TranslationModule<'input, From>,
    mut mapper: impl FnMut(
        &mut IdNameMapBuilder<'input>,
        Vec<Statement<ast::Instruction<From>, From>>,
    ) -> Result<Vec<Statement<ast::Instruction<To>, To>>, TranslateError>,
) -> Result<TranslationModule<'input, To>, TranslateError> {
    convert_methods(module, |_, id_defs, _, _, _, _, body| mapper(id_defs, body))
}

// NVIDIA PTX compiler emits methods that are declared like this:
//  .visible .func (.param .b8 retval[12]) foobar(.param .b64 arg1, .param .b64 arg2);
// This pass converts them to a regular form:
//  .visible .func (.reg .b8 retval[12]) foobar(.reg .b64 arg1, .reg .b64 arg2);
// and does appropriate adjustments to function calls
fn deparamize_function_declarations<'input>(
    mut module: TranslationModule<'input, TypedArgParams>,
) -> Result<TranslationModule<'input, TypedArgParams>, TranslateError> {
    let id_defs = &mut module.id_defs;
    let mut delayed_deparamization = FxHashMap::default();
    let directives = module
        .directives
        .into_iter()
        .map(|directive| deparamize_directive(id_defs, directive, &mut delayed_deparamization))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TranslationModule {
        directives,
        ..module
    })
}

fn deparamize_directive<'input>(
    id_defs: &mut IdNameMapBuilder,
    directive: TranslationDirective<'input, TypedArgParams>,
    delayed_deparamization: &mut FxHashMap<Id, (BitVec, BitVec)>,
) -> Result<TranslationDirective<'input, TypedArgParams>, TranslateError> {
    Ok(match directive {
        var @ TranslationDirective::Variable(..) => var,
        TranslationDirective::Method(mut method) => {
            let deparamized_args = get_deparamized_arguments(&method, delayed_deparamization);
            deparamize_function_body(id_defs, &mut method, deparamized_args)?;
            TranslationDirective::Method(method)
        }
    })
}

fn get_deparamized_arguments<'a, 'input, T: ast::ArgParams<Id = Id>>(
    method: &TranslationMethod<'input, T>,
    delayed_deparamization: &'a mut FxHashMap<Id, (BitVec, BitVec)>,
) -> Option<&'a (BitVec, BitVec)> {
    if method.is_kernel {
        return None;
    }
    Some(
        delayed_deparamization
            .entry(method.name)
            .or_insert_with(|| {
                let return_deparams = get_deparamize_arg_list(&method.return_arguments[..]);
                let input_deparams = get_deparamize_arg_list(&method.input_arguments[..]);
                (return_deparams, input_deparams)
            }),
    )
}

fn get_deparamize_arg_list(args: &[ast::VariableDeclaration<Id>]) -> BitVec {
    let mut deparams = BitVec::from_elem(args.len(), false);
    for (index, arg) in args.iter().enumerate() {
        if arg.state_space == ast::StateSpace::Param {
            deparams.set(index, true);
        }
    }
    deparams
}

fn deparamize_function_body<'input>(
    id_defs: &mut IdNameMapBuilder,
    method: &mut TranslationMethod<'input, TypedArgParams>,
    deparams: Option<&(BitVec, BitVec)>,
) -> Result<(), TranslateError> {
    let mut result = Vec::with_capacity(method.body.as_ref().map(|body| body.len()).unwrap_or(0));
    let has_body = method.body.is_some();
    let mut return_args_flush: hash_map::HashMap<
        Id,
        Id,
        std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
    > = FxHashMap::default();
    if let Some((return_deparams, input_deparams)) = deparams {
        for (index, return_arg) in method.return_arguments.iter_mut().enumerate() {
            if !return_deparams
                .get(index)
                .ok_or_else(TranslateError::unreachable)?
            {
                continue;
            }
            if has_body {
                let original_name = return_arg.name;
                *return_arg = id_defs.register_variable_decl(
                    return_arg.align,
                    return_arg.type_.clone(),
                    ast::StateSpace::Reg,
                );
                return_args_flush.insert(return_arg.name, original_name);
                result.push(Statement::Variable(Variable {
                    align: return_arg.align,
                    type_: return_arg.type_.clone(),
                    state_space: ast::StateSpace::Param,
                    name: original_name,
                    initializer: None,
                }));
            } else {
                return_arg.state_space = ast::StateSpace::Reg;
            }
        }
        for (index, input_arg) in method.input_arguments.iter_mut().enumerate() {
            if !input_deparams
                .get(index)
                .ok_or_else(TranslateError::unreachable)?
            {
                continue;
            }
            if has_body {
                let original_name = input_arg.name;
                *input_arg = id_defs.register_variable_decl(
                    input_arg.align,
                    input_arg.type_.clone(),
                    ast::StateSpace::Reg,
                );
                result.push(Statement::Variable(Variable {
                    align: input_arg.align,
                    type_: input_arg.type_.clone(),
                    state_space: ast::StateSpace::Param,
                    name: original_name,
                    initializer: None,
                }));
                result.push(Statement::Instruction(ast::Instruction::St(
                    ast::StData {
                        qualifier: ast::LdStQualifier::Weak,
                        state_space: ast::StateSpace::Param,
                        caching: ast::StCacheOperator::Writeback,
                        typ: input_arg.type_.clone(),
                    },
                    ast::Arg2St {
                        src1: TypedOperand::Reg(original_name),
                        src2: TypedOperand::Reg(input_arg.name),
                    },
                )));
            } else {
                input_arg.state_space = ast::StateSpace::Reg;
            }
        }
    }
    let body = if let Some(ref mut body) = method.body {
        std::mem::replace(body, Vec::new())
    } else {
        return Ok(());
    };
    for statement in body {
        match statement {
            Statement::Instruction(ast::Instruction::Exit) => {
                deparamize_instruction_ret(
                    deparams,
                    &method.return_arguments,
                    &return_args_flush,
                    &mut result,
                    ast::Instruction::Exit,
                )?;
            }
            Statement::Instruction(ast::Instruction::Ret(ret)) => {
                deparamize_instruction_ret(
                    deparams,
                    &method.return_arguments,
                    &return_args_flush,
                    &mut result,
                    ast::Instruction::Ret(ret),
                )?;
            }
            Statement::Call(call) => {
                deparamize_single_function_call(id_defs, &mut result, call)?;
            }
            statement => result.push(statement),
        }
    }
    method.body = Some(result);
    Ok(())
}

fn deparamize_instruction_ret(
    deparams: Option<&(BitVec, BitVec)>,
    return_arguments: &[ast::VariableDeclaration<Id>],
    return_args_flush: &std::collections::HashMap<
        Id,
        Id,
        std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
    >,
    result: &mut Vec<Statement<ast::Instruction<TypedArgParams>, TypedArgParams>>,
    ret: ast::Instruction<TypedArgParams>,
) -> Result<(), TranslateError> {
    if let Some((return_deparams, _)) = deparams {
        for (index, return_arg) in return_arguments.iter().enumerate() {
            if !return_deparams
                .get(index)
                .ok_or_else(TranslateError::unreachable)?
            {
                continue;
            }
            let src = return_args_flush[&return_arg.name];
            result.push(Statement::Instruction(ast::Instruction::Ld(
                ast::LdDetails {
                    qualifier: ast::LdStQualifier::Weak,
                    state_space: ast::StateSpace::Param,
                    caching: ast::LdCacheOperator::Cached,
                    typ: return_arg.type_.clone(),
                    non_coherent: false,
                },
                ast::Arg2Ld {
                    dst: TypedOperand::Reg(return_arg.name),
                    src: TypedOperand::Reg(src),
                },
            )));
        }
    }
    result.push(Statement::Instruction(ret));
    Ok(())
}

fn deparamize_single_function_call(
    id_defs: &mut IdNameMapBuilder,
    result: &mut Vec<TypedStatement>,
    call: ResolvedCall<TypedArgParams>,
) -> Result<(), TranslateError> {
    let input_arguments = call
        .input_arguments
        .into_iter()
        .map(|(operand, type_, space)| match space {
            ast::StateSpace::Param => {
                let arg_id =
                    id_defs.register_intermediate(Some((type_.clone(), ast::StateSpace::Reg)));
                result.push(Statement::Instruction(ast::Instruction::Ld(
                    ast::LdDetails {
                        qualifier: ast::LdStQualifier::Weak,
                        state_space: ast::StateSpace::Param,
                        caching: ast::LdCacheOperator::Cached,
                        typ: type_.clone(),
                        non_coherent: false,
                    },
                    ast::Arg2Ld {
                        dst: TypedOperand::Reg(arg_id),
                        src: operand,
                    },
                )));
                (TypedOperand::Reg(arg_id), type_, ast::StateSpace::Reg)
            }
            space => (operand, type_, space),
        })
        .collect::<Vec<_>>();
    let mut post_statements = Vec::new();
    let return_arguments = call
        .return_arguments
        .into_iter()
        .map(|(operand, type_, state_space)| {
            Ok(match state_space {
                ast::StateSpace::Reg => (operand, type_, state_space),
                ast::StateSpace::Param => {
                    let arg_id =
                        id_defs.register_intermediate(Some((type_.clone(), ast::StateSpace::Reg)));
                    post_statements.push(Statement::Instruction(ast::Instruction::St(
                        ast::StData {
                            qualifier: ast::LdStQualifier::Weak,
                            state_space: ast::StateSpace::Param,
                            caching: ast::StCacheOperator::Writeback,
                            typ: type_.clone(),
                        },
                        ast::Arg2St {
                            src1: TypedOperand::Reg(operand),
                            src2: TypedOperand::Reg(arg_id),
                        },
                    )));
                    (arg_id, type_, ast::StateSpace::Reg)
                }
                _ => return Err(TranslateError::unreachable()),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    result.push(Statement::Call(ResolvedCall {
        input_arguments,
        return_arguments,
        ..call
    }));
    result.extend(post_statements);
    Ok(())
}

fn insert_hardware_registers<'input>(
    module: TranslationModule<'input, TypedArgParams>,
) -> Result<TranslationModule<'input, TypedArgParams>, TranslateError> {
    convert_methods_simple(module, insert_hardware_registers_impl)
}

// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#extended-precision-integer-arithmetic-instructions
// NVIDIA documentation is slightly misleading when it comes to subc and sub.cc.
// They both invert the CC flag. Meaning that for sub:
// * sub.cc x, 0,1 will set CC to 0
// * sub.cc x, 0,0 will set CC to 1
// and for subc:
// * if CC is 1 then subc will compute d = a - b
// * if CC is 0 then subc will compute d = a - b - 1
fn insert_hardware_registers_impl<'input>(
    id_defs: &mut IdNameMapBuilder<'input>,
    typed_statements: Vec<TypedStatement>,
) -> Result<Vec<TypedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(typed_statements.len());
    let carry_flag_variable = id_defs.register_variable_def(
        None,
        ast::Type::Scalar(ast::ScalarType::Pred),
        ast::StateSpace::Reg,
        Some(ast::Initializer::Constant(ast::ImmediateValue::U64(0))),
    );
    let carry_flag = carry_flag_variable.name;
    result.push(Statement::Variable(carry_flag_variable));
    for statement in typed_statements {
        match statement {
            Statement::Instruction(ast::Instruction::MadC {
                type_,
                is_hi,
                arg,
                carry_out,
            }) => result.push(Statement::MadC(MadCDetails {
                type_,
                is_hi,
                arg: Arg4CarryIn::new(arg, carry_out, TypedOperand::Reg(carry_flag)),
            })),
            Statement::Instruction(ast::Instruction::MadCC { type_, is_hi, arg }) => {
                result.push(Statement::MadCC(MadCCDetails {
                    type_,
                    is_hi,
                    arg: Arg4CarryOut::new(arg, TypedOperand::Reg(carry_flag)),
                }))
            }
            Statement::Instruction(ast::Instruction::AddC(details, args)) => {
                result.push(Statement::AddC(
                    details.type_,
                    Arg3CarryIn::new(args, details.carry_out, TypedOperand::Reg(carry_flag)),
                ))
            }
            Statement::Instruction(ast::Instruction::AddCC(details, args)) => {
                result.push(Statement::AddCC(
                    details,
                    Arg3CarryOut::new(args, TypedOperand::Reg(carry_flag)),
                ))
            }
            Statement::Instruction(ast::Instruction::SubC(details, args)) => {
                let inverted_carry_in = id_defs.register_intermediate(Some((
                    ast::Type::Scalar(ast::ScalarType::Pred),
                    ast::StateSpace::Reg,
                )));
                result.push(Statement::Instruction(ast::Instruction::Not(
                    ast::ScalarType::Pred,
                    ast::Arg2 {
                        dst: TypedOperand::Reg(inverted_carry_in),
                        src: TypedOperand::Reg(carry_flag),
                    },
                )));
                let (carry_out_id, carry_out_postprocess) = if details.carry_out {
                    let inverted_carry_out = id_defs.register_intermediate(Some((
                        ast::Type::Scalar(ast::ScalarType::Pred),
                        ast::StateSpace::Reg,
                    )));
                    let invert_statement = Statement::Instruction(ast::Instruction::Not(
                        ast::ScalarType::Pred,
                        ast::Arg2 {
                            dst: TypedOperand::Reg(carry_flag),
                            src: TypedOperand::Reg(inverted_carry_out),
                        },
                    ));
                    (
                        Some(TypedOperand::Reg(inverted_carry_out)),
                        Some(invert_statement),
                    )
                } else {
                    (None, None)
                };
                result.push(Statement::SubC(
                    details.type_,
                    Arg3CarryIn {
                        dst: args.dst,
                        carry_out: carry_out_id,
                        carry_in: TypedOperand::Reg(inverted_carry_in),
                        src1: args.src1,
                        src2: args.src2,
                    },
                ));
                if let Some(carry_out_postprocess) = carry_out_postprocess {
                    result.push(carry_out_postprocess);
                }
            }
            Statement::Instruction(ast::Instruction::SubCC(type_, args)) => {
                let temp = id_defs.register_intermediate(Some((
                    ast::Type::Scalar(ast::ScalarType::Pred),
                    ast::StateSpace::Reg,
                )));
                result.push(Statement::SubCC(
                    type_,
                    Arg3CarryOut::new(args, TypedOperand::Reg(temp)),
                ));
                result.push(Statement::Instruction(ast::Instruction::Not(
                    ast::ScalarType::Pred,
                    ast::Arg2 {
                        dst: TypedOperand::Reg(carry_flag),
                        src: TypedOperand::Reg(temp),
                    },
                )));
            }
            s => result.push(s),
        }
    }
    Ok(result)
}

fn fix_special_registers<'input>(
    module: TranslationModule<'input, TypedArgParams>,
) -> Result<TranslationModule<'input, TypedArgParams>, TranslateError> {
    convert_methods(module, fix_special_registers_impl)
}

fn fix_special_registers_impl<'input>(
    _: CompilationMode,
    id_defs: &mut IdNameMapBuilder<'input>,
    ptx_imports: &mut AdditionalFunctionDeclarations,
    _: &mut [ast::VariableDeclaration<Id>],
    _: &mut [ast::VariableDeclaration<Id>],
    _: bool,
    typed_statements: Vec<TypedStatement>,
) -> Result<Vec<TypedStatement>, TranslateError> {
    let result = Vec::with_capacity(typed_statements.len());
    let mut sreg_sresolver = SpecialRegisterResolver {
        ptx_imports,
        id_defs,
        result,
    };
    for s in typed_statements {
        match s {
            Statement::Call(details) => {
                let new_statement = details.visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::Instruction(details) => {
                let new_statement = details.visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::Conditional(details) => {
                let new_statement = details.visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::Conversion(details) => {
                let new_statement = details.visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::PtrAccess(details) => {
                let new_statement = details.visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::RepackVector(details) => {
                let new_statement = details.visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::MadC(details) => {
                let new_statement = details.visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::MadCC(details) => {
                let new_statement = details.visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::AddC(details, arg) => {
                let new_statement = VisitAddC(details, arg).visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::AddCC(type_, arg) => {
                let new_statement = VisitAddCC(type_, arg).visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::SubC(details, arg) => {
                let new_statement = VisitSubC(details, arg).visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::SubCC(type_, arg) => {
                let new_statement = VisitSubCC(type_, arg).visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            s @ Statement::Variable(_)
            | s @ Statement::Constant(_)
            | s @ Statement::Label(_)
            | s @ Statement::FunctionPointer(_) => sreg_sresolver.result.push(s),
            _ => return Err(TranslateError::unreachable()),
        }
    }
    Ok(sreg_sresolver.result)
}

struct AdditionalFunctionDeclarations(
    BTreeMap<
        String,
        (
            Vec<ast::VariableDeclaration<Id>>,
            Id,
            Vec<ast::VariableDeclaration<Id>>,
        ),
    >,
);

impl AdditionalFunctionDeclarations {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn add_or_get_declaration<'a>(
        &mut self,
        id_defs: &mut IdNameMapBuilder,
        name: String,
        return_arguments: impl Iterator<Item = (&'a ast::Type, ast::StateSpace)>,
        input_arguments: impl Iterator<Item = (&'a ast::Type, ast::StateSpace)>,
    ) -> Result<Id, TranslateError> {
        Ok(match self.0.entry(name) {
            btree_map::Entry::Vacant(entry) => {
                let fn_id = id_defs.register_intermediate(None);
                let return_arguments = fn_arguments_to_variables(id_defs, return_arguments);
                let input_arguments = fn_arguments_to_variables(id_defs, input_arguments);
                entry.insert((return_arguments, fn_id, input_arguments));
                fn_id
            }
            btree_map::Entry::Occupied(entry) => entry.get().1,
        })
    }

    fn flush<'input, P: ast::ArgParams<Id = Id>>(
        self,
        directives: &mut Vec<TranslationDirective<'input, P>>,
    ) {
        for (name, (return_arguments, id, input_arguments)) in self.0 {
            directives.push(TranslationDirective::Method(TranslationMethod {
                return_arguments,
                name: id,
                input_arguments,
                body: None,
                tuning: Vec::new(),
                is_kernel: false,
                source_name: Some(Cow::Owned(name)),
                special_raytracing_linking: false,
            }));
        }
    }
}

fn insert_mem_ssa_statements<'input>(
    module: TranslationModule<'input, TypedArgParams>,
) -> Result<TranslationModule<'input, TypedArgParams>, TranslateError> {
    convert_methods(module, insert_mem_ssa_statements_impl)
}

fn insert_mem_ssa_statements_impl<'input>(
    _: CompilationMode,
    id_def: &mut IdNameMapBuilder<'input>,
    _: &mut AdditionalFunctionDeclarations,
    return_arguments: &mut [ast::VariableDeclaration<Id>],
    input_arguments: &mut [ast::VariableDeclaration<Id>],
    is_kernel: bool,
    typed_statements: Vec<TypedStatement>,
) -> Result<Vec<TypedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(typed_statements.len());
    if !is_kernel {
        for arg in input_arguments.iter_mut() {
            insert_mem_ssa_argument(id_def, &mut result, arg);
        }
    }
    for arg in return_arguments.iter() {
        insert_mem_ssa_argument_reg_return(&mut result, arg);
    }
    for statement in typed_statements {
        match statement {
            Statement::Call(call) => {
                insert_mem_ssa_statement_default(id_def, &mut result, call.cast())?
            }
            Statement::Instruction(inst) => match inst {
                ast::Instruction::Exit => {
                    insert_mma_ssa_statement_ret(
                        return_arguments,
                        &mut result,
                        ast::Instruction::Exit,
                        ast::RetData { uniform: false },
                        id_def,
                    );
                }
                ast::Instruction::Ret(d) => {
                    insert_mma_ssa_statement_ret(
                        return_arguments,
                        &mut result,
                        ast::Instruction::Ret(d),
                        d,
                        id_def,
                    );
                }
                inst => insert_mem_ssa_statement_default(id_def, &mut result, inst)?,
            },
            Statement::Conditional(bra) => {
                insert_mem_ssa_statement_default(id_def, &mut result, bra)?
            }
            Statement::Conversion(conv) => {
                insert_mem_ssa_statement_default(id_def, &mut result, conv)?
            }
            Statement::PtrAccess(ptr_access) => {
                insert_mem_ssa_statement_default(id_def, &mut result, ptr_access)?
            }
            Statement::RepackVector(repack) => {
                insert_mem_ssa_statement_default(id_def, &mut result, repack)?
            }
            Statement::FunctionPointer(func_ptr) => {
                insert_mem_ssa_statement_default(id_def, &mut result, func_ptr)?
            }
            Statement::MadC(madc) => insert_mem_ssa_statement_default(id_def, &mut result, madc)?,
            Statement::MadCC(madcc) => {
                insert_mem_ssa_statement_default(id_def, &mut result, madcc)?
            }
            Statement::AddC(details, arg) => {
                insert_mem_ssa_statement_default(id_def, &mut result, VisitAddC(details, arg))?
            }
            Statement::AddCC(type_, arg) => {
                insert_mem_ssa_statement_default(id_def, &mut result, VisitAddCC(type_, arg))?
            }
            Statement::SubC(details, arg) => {
                insert_mem_ssa_statement_default(id_def, &mut result, VisitSubC(details, arg))?
            }
            Statement::SubCC(type_, arg) => {
                insert_mem_ssa_statement_default(id_def, &mut result, VisitSubCC(type_, arg))?
            }
            s @ Statement::Variable(_) | s @ Statement::Label(_) | s @ Statement::Constant(..) => {
                result.push(s)
            }
            _ => return Err(TranslateError::unreachable()),
        }
    }
    Ok(result)
}

fn insert_mma_ssa_statement_ret(
    return_arguments: &mut [ast::VariableDeclaration<Id>],
    result: &mut Vec<Statement<ast::Instruction<TypedArgParams>, TypedArgParams>>,
    zero_inst: ast::Instruction<TypedArgParams>,
    d: ast::RetData,
    id_def: &mut IdNameMapBuilder<'_>,
) {
    if return_arguments.len() == 0 {
        result.push(Statement::Instruction(zero_inst));
    } else {
        let return_ids = return_arguments
            .iter()
            .map(|return_reg| {
                let new_id = id_def
                    .register_intermediate(Some((return_reg.type_.clone(), ast::StateSpace::Reg)));
                result.push(Statement::LoadVar(LoadVarDetails {
                    arg: ast::Arg2 {
                        dst: new_id,
                        src: return_reg.name,
                    },
                    // TODO: ret with stateful conversion
                    _state_space: ast::StateSpace::Reg,
                    typ: return_reg.type_.clone(),
                    member_index: None,
                }));
                (new_id, return_reg.type_.clone())
            })
            .collect::<Vec<_>>();
        result.push(Statement::RetValue(d, return_ids));
    }
}

fn expand_arguments<'input>(
    module: TranslationModule<'input, TypedArgParams>,
) -> Result<TranslationModule<'input, ExpandedArgParams>, TranslateError> {
    convert_methods_simple(module, expand_arguments2_impl)
}

fn expand_arguments2_impl<'input>(
    id_defs: &mut IdNameMapBuilder<'input>,
    fn_body: Vec<TypedStatement>,
) -> Result<Vec<ExpandedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(fn_body.len());
    for statment in fn_body {
        match statment {
            Statement::Call(call) => {
                let mut visitor = FlattenArguments::new(&mut result, id_defs);
                let (new_call, post_stmts) = (call.map(&mut visitor)?, visitor.post_stmts);
                result.push(Statement::Call(new_call));
                result.extend(post_stmts);
            }
            Statement::Instruction(inst) => {
                let mut visitor = FlattenArguments::new(&mut result, id_defs);
                let (new_inst, post_stmts) = (inst.map(&mut visitor)?, visitor.post_stmts);
                result.push(Statement::Instruction(new_inst));
                result.extend(post_stmts);
            }
            Statement::Variable(Variable {
                align,
                type_,
                state_space,
                name,
                initializer,
            }) => result.push(Statement::Variable(Variable {
                align,
                type_,
                state_space,
                name,
                initializer,
            })),
            Statement::PtrAccess(ptr_access) => {
                let mut visitor = FlattenArguments::new(&mut result, id_defs);
                let (new_inst, post_stmts) = (ptr_access.map(&mut visitor)?, visitor.post_stmts);
                result.push(Statement::PtrAccess(new_inst));
                result.extend(post_stmts);
            }
            Statement::RepackVector(repack) => {
                let mut visitor = FlattenArguments::new(&mut result, id_defs);
                let (new_inst, post_stmts) = (repack.map(&mut visitor)?, visitor.post_stmts);
                result.push(Statement::RepackVector(new_inst));
                result.extend(post_stmts);
            }
            Statement::MadC(madc) => {
                let mut visitor = FlattenArguments::new(&mut result, id_defs);
                let (new_inst, post_stmts) = (madc.visit(&mut visitor)?, visitor.post_stmts);
                result.push(new_inst);
                result.extend(post_stmts);
            }
            Statement::MadCC(madcc) => {
                let mut visitor = FlattenArguments::new(&mut result, id_defs);
                let (new_inst, post_stmts) = (madcc.visit(&mut visitor)?, visitor.post_stmts);
                result.push(new_inst);
                result.extend(post_stmts);
            }
            Statement::AddC(details, arg) => {
                let mut visitor = FlattenArguments::new(&mut result, id_defs);
                let (new_inst, post_stmts) = (
                    VisitAddC(details, arg).visit(&mut visitor)?,
                    visitor.post_stmts,
                );
                result.push(new_inst);
                result.extend(post_stmts);
            }
            Statement::AddCC(type_, arg) => {
                let mut visitor = FlattenArguments::new(&mut result, id_defs);
                let (new_inst, post_stmts) = (
                    VisitAddCC(type_, arg).visit(&mut visitor)?,
                    visitor.post_stmts,
                );
                result.push(new_inst);
                result.extend(post_stmts);
            }
            Statement::SubC(details, arg) => {
                let mut visitor = FlattenArguments::new(&mut result, id_defs);
                let (new_inst, post_stmts) = (
                    VisitSubC(details, arg).visit(&mut visitor)?,
                    visitor.post_stmts,
                );
                result.push(new_inst);
                result.extend(post_stmts);
            }
            Statement::SubCC(type_, arg) => {
                let mut visitor = FlattenArguments::new(&mut result, id_defs);
                let (new_inst, post_stmts) = (
                    VisitSubCC(type_, arg).visit(&mut visitor)?,
                    visitor.post_stmts,
                );
                result.push(new_inst);
                result.extend(post_stmts);
            }
            Statement::Label(id) => result.push(Statement::Label(id)),
            Statement::Conditional(bra) => result.push(Statement::Conditional(bra)),
            Statement::LoadVar(details) => result.push(Statement::LoadVar(details)),
            Statement::StoreVar(details) => result.push(Statement::StoreVar(details)),
            Statement::RetValue(d, id) => result.push(Statement::RetValue(d, id)),
            Statement::Conversion(conv) => result.push(Statement::Conversion(conv)),
            Statement::Constant(c) => result.push(Statement::Constant(c)),
            Statement::FunctionPointer(d) => result.push(Statement::FunctionPointer(d)),
            Statement::AsmVolatile { asm, constraints } => {
                result.push(Statement::AsmVolatile { asm, constraints })
            }
        }
    }
    Ok(result)
}

/*
 There are several kinds of implicit conversions in PTX:
 * auto-bitcast: https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#type-information-for-instructions-and-operands
 * special ld/st/cvt conversion rules: https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#operand-size-exceeding-instruction-type-size
   - ld.param: not documented, but for instruction `ld.param.<type> x, [y]`,
     semantics are to first zext/chop/bitcast `y` as needed and then do
     documented special ld/st/cvt conversion rules for destination operands
   - st.param [x] y (used as function return arguments) same rule as above applies
   - generic/global ld: for instruction `ld x, [y]`, y must be of type
     b64/u64/s64, which is bitcast to a pointer, dereferenced and then
     documented special ld/st/cvt conversion rules are applied to dst
   - generic/global st: for instruction `st [x], y`, x must be of type
     b64/u64/s64, which is bitcast to a pointer
 * and many more
*/
fn insert_implicit_conversions<'input>(
    module: TranslationModule<'input, ExpandedArgParams>,
) -> Result<TranslationModule<'input, ExpandedArgParams>, TranslateError> {
    convert_methods_simple(module, insert_implicit_conversions2_impl)
}

fn insert_implicit_conversions2_impl<'input>(
    id_def: &mut IdNameMapBuilder<'input>,
    fn_body: Vec<ExpandedStatement>,
) -> Result<Vec<ExpandedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(fn_body.len());
    for statement in fn_body {
        match statement {
            Statement::Call(call) => {
                insert_implicit_conversions_impl(&mut result, id_def, call)?;
            }
            Statement::Instruction(inst) => {
                insert_implicit_conversions_impl(&mut result, id_def, inst)?;
            }
            Statement::PtrAccess(access) => {
                insert_implicit_conversions_impl(&mut result, id_def, access)?;
            }
            Statement::RepackVector(repack) => {
                insert_implicit_conversions_impl(&mut result, id_def, repack)?;
            }
            Statement::MadC(madc) => {
                insert_implicit_conversions_impl(&mut result, id_def, madc)?;
            }
            Statement::MadCC(madcc) => {
                insert_implicit_conversions_impl(&mut result, id_def, madcc)?;
            }
            Statement::AddC(details, arg) => {
                insert_implicit_conversions_impl(&mut result, id_def, VisitAddC(details, arg))?;
            }
            Statement::AddCC(type_, arg) => {
                insert_implicit_conversions_impl(&mut result, id_def, VisitAddCC(type_, arg))?;
            }
            Statement::SubC(details, arg) => {
                insert_implicit_conversions_impl(&mut result, id_def, VisitSubC(details, arg))?;
            }
            Statement::SubCC(type_, arg) => {
                insert_implicit_conversions_impl(&mut result, id_def, VisitSubCC(type_, arg))?;
            }
            s @ Statement::Conditional(_)
            | s @ Statement::Conversion(_)
            | s @ Statement::Label(_)
            | s @ Statement::Constant(_)
            | s @ Statement::Variable(_)
            | s @ Statement::LoadVar(..)
            | s @ Statement::StoreVar(..)
            | s @ Statement::RetValue(..)
            | s @ Statement::AsmVolatile { .. }
            | s @ Statement::FunctionPointer(..) => result.push(s),
        }
    }
    Ok(result)
}

fn hoist_globals<'input, P: ast::ArgParams<Id = Id>>(
    module: TranslationModule<'input, P>,
) -> TranslationModule<'input, P> {
    let mut directives = Vec::with_capacity(module.directives.len());
    for directive in module.directives {
        match directive {
            var @ TranslationDirective::Variable(..) => directives.push(var),
            TranslationDirective::Method(method) => {
                let body = method.body.map(|body| {
                    body.into_iter()
                        .filter_map(|statement| match statement {
                            Statement::Variable(
                                var @ Variable {
                                    state_space: ast::StateSpace::Shared,
                                    ..
                                },
                            )
                            | Statement::Variable(
                                var @ Variable {
                                    state_space: ast::StateSpace::Global,
                                    ..
                                },
                            ) => {
                                directives.push(TranslationDirective::Variable(
                                    ast::LinkingDirective::None,
                                    None,
                                    var,
                                ));
                                None
                            }
                            statement => Some(statement),
                        })
                        .collect::<Vec<_>>()
                });
                directives.push(TranslationDirective::Method(TranslationMethod {
                    body,
                    ..method
                }))
            }
        }
    }
    {
        TranslationModule {
            directives,
            ..module
        }
    }
}

fn replace_instructions_with_builtins<'input>(
    module: TranslationModule<'input, ExpandedArgParams>,
) -> Result<TranslationModule<'input, ExpandedArgParams>, TranslateError> {
    convert_methods(module, replace_instructions_with_builtins_impl)
}

fn replace_instructions_with_builtins_impl<'input>(
    compilation_mode: CompilationMode,
    id_def: &mut IdNameMapBuilder<'input>,
    ptx_impl_imports: &mut AdditionalFunctionDeclarations,
    _: &mut [ast::VariableDeclaration<Id>],
    _: &mut [ast::VariableDeclaration<Id>],
    _: bool,
    fn_body: Vec<ExpandedStatement>,
) -> Result<Vec<ExpandedStatement>, TranslateError> {
    let mut statements = Vec::with_capacity(fn_body.len());
    for statement in fn_body {
        match statement {
            Statement::Instruction(ast::Instruction::Nanosleep(arg)) => {
                let fn_name = [ZLUDA_PTX_PREFIX, "nanosleep_u32"].concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Nanosleep(arg),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::MatchAny(arg)) => {
                let fn_name = [ZLUDA_PTX_PREFIX, "match_any_sync_b32"].concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::MatchAny(arg),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Dp4a(type_, arg)) => {
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "dp4a_",
                    type_.to_ptx_name(),
                    "_",
                    type_.to_ptx_name(),
                ]
                .concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Dp4a(type_, arg),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::BarRed(op, arg3)) => {
                let op_name = match op {
                    ast::ReductionOp::And => "and",
                    ast::ReductionOp::Or => "or",
                    ast::ReductionOp::Popc => "popc",
                };
                let dst_type = op.dst_type().to_ptx_name();
                let fn_name = [ZLUDA_PTX_PREFIX, "bar_red_", op_name, "_", dst_type].concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::BarRed(op, arg3),
                    fn_name,
                )?);
            }
            // We dispatch vote_sync_... by compilation mode suffix
            // because LLVM crashes if there are both calls to `llvm.amdgcn.ballot.i32` and
            // `llvm.amdgcn.ballot.i64` inside same function (even if only one of them can be called)
            Statement::Instruction(ast::Instruction::Vote(
                ast::VoteDetails {
                    mode: ast::VoteMode::Any,
                    negate_pred,
                },
                arg,
            )) => {
                let instr_suffix = if negate_pred { "_negate" } else { "" };
                let mode_suffix = compilation_mode_suffix(compilation_mode);
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "vote_sync_any_pred",
                    instr_suffix,
                    mode_suffix,
                ]
                .concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Vote(
                        ast::VoteDetails {
                            mode: ast::VoteMode::Any,
                            negate_pred,
                        },
                        arg,
                    ),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Vote(
                ast::VoteDetails {
                    mode: ast::VoteMode::All,
                    negate_pred,
                },
                arg,
            )) => {
                let instr_suffix = if negate_pred { "_negate" } else { "" };
                let mode_suffix = compilation_mode_suffix(compilation_mode);
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "vote_sync_all_pred",
                    instr_suffix,
                    mode_suffix,
                ]
                .concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Vote(
                        ast::VoteDetails {
                            mode: ast::VoteMode::All,
                            negate_pred,
                        },
                        arg,
                    ),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Vote(
                ast::VoteDetails {
                    mode: ast::VoteMode::Ballot,
                    negate_pred,
                },
                arg,
            )) => {
                let instr_suffix = if negate_pred { "_negate" } else { "" };
                let mode_suffix = compilation_mode_suffix(compilation_mode);
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "vote_sync_ballot_b32",
                    instr_suffix,
                    mode_suffix,
                ]
                .concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Vote(
                        ast::VoteDetails {
                            mode: ast::VoteMode::Ballot,
                            negate_pred,
                        },
                        arg,
                    ),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Bar(details, arg)) => {
                let fn_name = [ZLUDA_PTX_PREFIX, "barrier_sync"].concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Bar(details, arg),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Bfe { typ, arg }) => {
                let fn_name = [ZLUDA_PTX_PREFIX, "bfe_", typ.to_ptx_name()].concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Bfe { typ, arg },
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Bfi { typ, arg }) => {
                let fn_name = [ZLUDA_PTX_PREFIX, "bfi_", typ.to_ptx_name()].concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Bfi { typ, arg },
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Activemask { arg }) => {
                let fn_name = [ZLUDA_PTX_PREFIX, "activemask"].concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Activemask { arg },
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Tex(tex, arg)) => {
                let geometry = tex.geometry.as_ptx();
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "tex",
                    tex.suffix(),
                    "_",
                    geometry,
                    "_v4",
                    "_",
                    tex.channel_type.to_ptx_name(),
                    "_",
                    tex.coordinate_type.to_ptx_name(),
                ]
                .concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Tex(tex, arg),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Shfl(shfl_mode, arg))
                if arg.dst2.is_none() =>
            {
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "shfl_",
                    shfl_mode.to_ptx_name(),
                    "_b32_slow",
                ]
                .concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Shfl(shfl_mode, arg),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Shfl(shfl_mode, arg))
                if arg.dst2.is_some() =>
            {
                replace_shfl_with_pred(id_def, ptx_impl_imports, &mut statements, shfl_mode, arg)?;
            }
            Statement::Instruction(ast::Instruction::Cvt(
                ast::CvtDetails::FloatFromFloat(ast::CvtDesc {
                    saturate: true,
                    src,
                    dst,
                    rounding,
                    flush_to_zero,
                }),
                arg,
            )) if src == dst => {
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "cvt_sat_",
                    dst.to_ptx_name(),
                    "_",
                    dst.to_ptx_name(),
                ]
                .concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Cvt(
                        ast::CvtDetails::FloatFromFloat(ast::CvtDesc {
                            saturate: true,
                            src,
                            dst,
                            rounding,
                            flush_to_zero,
                        }),
                        arg,
                    ),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Shf(
                ast::FunnelShift {
                    direction,
                    mode: ast::ShiftNormalization::Clamp,
                },
                arg,
            )) => {
                let direction_str = match direction {
                    ast::FunnelDirection::Left => "l",
                    ast::FunnelDirection::Right => "r",
                };
                let fn_name = [ZLUDA_PTX_PREFIX, "shf_", direction_str, "_clamp_b32"].concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Shf(
                        ast::FunnelShift {
                            direction,
                            mode: ast::ShiftNormalization::Clamp,
                        },
                        arg,
                    ),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Suld(suld, arg)) => {
                let geometry = suld.geometry.as_ptx();
                let vector = suld.vector_ptx()?;
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "suld_b_",
                    suld.suffix(),
                    geometry,
                    vector,
                    "_",
                    suld.type_.to_ptx_name(),
                    "_zero",
                ]
                .concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Suld(suld, arg),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Sust(sust, arg)) => {
                let geometry = sust.geometry.as_ptx();
                let vector = sust.vector_ptx()?;
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "sust_b_",
                    sust.suffix(),
                    geometry,
                    vector,
                    "_",
                    sust.type_.to_ptx_name(),
                    "_zero",
                ]
                .concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Sust(sust, arg),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Atom(
                details @ ast::AtomDetails {
                    inner:
                        ast::AtomInnerDetails::Unsigned {
                            op: ast::AtomUIntOp::Inc,
                            ..
                        },
                    ..
                },
                args,
            )) => {
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "atom_",
                    details.semantics.to_ptx_name(),
                    "_",
                    details.scope.to_ptx_name(),
                    "_",
                    details.space.to_ptx_name(),
                    "_inc",
                ]
                .concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Atom(details, args),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Atom(
                details @ ast::AtomDetails {
                    inner:
                        ast::AtomInnerDetails::Unsigned {
                            op: ast::AtomUIntOp::Dec,
                            ..
                        },
                    ..
                },
                args,
            )) => {
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "atom_",
                    details.semantics.to_ptx_name(),
                    "_",
                    details.scope.to_ptx_name(),
                    "_",
                    details.space.to_ptx_name(),
                    "_dec",
                ]
                .concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Atom(details, args),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Cvt(
                ast::CvtDetails::FloatFromInt(desc),
                args,
            )) => extract_global_cvt(
                &mut statements,
                ptx_impl_imports,
                id_def,
                desc.clone(),
                ast::Instruction::Cvt(ast::CvtDetails::FloatFromInt(desc), args),
            )?,
            Statement::Instruction(ast::Instruction::Cvt(
                ast::CvtDetails::IntFromFloat(desc),
                args,
            )) => extract_global_cvt(
                &mut statements,
                ptx_impl_imports,
                id_def,
                desc.clone(),
                ast::Instruction::Cvt(ast::CvtDetails::IntFromFloat(desc), args),
            )?,
            Statement::Instruction(ast::Instruction::Cvt(
                ast::CvtDetails::FloatFromFloat(desc),
                args,
            )) if desc.dst.size_of() < desc.src.size_of() => extract_global_cvt(
                &mut statements,
                ptx_impl_imports,
                id_def,
                desc.clone(),
                ast::Instruction::Cvt(ast::CvtDetails::FloatFromFloat(desc), args),
            )?,
            Statement::Instruction(ast::Instruction::Mul(
                ast::MulDetails::Signed(ast::MulInt {
                    control: ast::MulIntControl::High,
                    typ,
                }),
                args,
            ))
            | Statement::Instruction(ast::Instruction::Mul(
                ast::MulDetails::Unsigned(ast::MulInt {
                    control: ast::MulIntControl::High,
                    typ,
                }),
                args,
            )) if typ == ast::ScalarType::U64 || typ == ast::ScalarType::S64 => {
                let fn_name = [ZLUDA_PTX_PREFIX, "mul_hi_", typ.to_ptx_name()].concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Mul(
                        ast::MulDetails::Signed(ast::MulInt {
                            control: ast::MulIntControl::High,
                            typ,
                        }),
                        args,
                    ),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Mad(
                ast::MulDetails::Signed(ast::MulInt {
                    control: ast::MulIntControl::High,
                    typ,
                }),
                args,
            ))
            | Statement::Instruction(ast::Instruction::Mad(
                ast::MulDetails::Unsigned(ast::MulInt {
                    control: ast::MulIntControl::High,
                    typ,
                }),
                args,
            )) if typ == ast::ScalarType::U64 || typ == ast::ScalarType::S64 => {
                let fn_name = [ZLUDA_PTX_PREFIX, "mad_hi_", typ.to_ptx_name()].concat();
                statements.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Mad(
                        ast::MulDetails::Signed(ast::MulInt {
                            control: ast::MulIntControl::High,
                            typ,
                        }),
                        args,
                    ),
                    fn_name,
                )?);
            }
            s => statements.push(s),
        }
    }
    Ok(statements)
}

fn compilation_mode_suffix(compilation_mode: CompilationMode) -> &'static str {
    match compilation_mode {
        CompilationMode::Wave32 => "_32",
        CompilationMode::Wave32OnWave64 => "_32on64",
        CompilationMode::DoubleWave32OnWave64 => "_double32on64",
    }
}

fn replace_shfl_with_pred<'input>(
    id_defs: &mut IdNameMapBuilder<'input>,
    ptx_impl_imports: &mut AdditionalFunctionDeclarations,
    statements: &mut Vec<ExpandedStatement>,
    shfl_mode: ast::ShflMode,
    arg: ast::Arg5Shfl<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let fn_name = [
        ZLUDA_PTX_PREFIX,
        "shfl_",
        shfl_mode.to_ptx_name(),
        "_b32_pred_slow",
    ]
    .concat();
    let inst = ast::Instruction::Shfl(shfl_mode, arg);
    let mut arguments = Vec::new();
    inst.visit(
        &mut |desc: ArgumentDescriptor<Id>, typ: Option<(&ast::Type, ast::StateSpace)>| {
            let (typ, space) = match typ {
                Some((typ, space)) => (typ.clone(), space),
                None => return Err(TranslateError::unreachable()),
            };
            let result = desc.op;
            arguments.push((desc, typ, space));
            Ok(result)
        },
    )?;
    let return_arguments_count = arguments
        .iter()
        .position(|(desc, _, _)| !desc.is_dst)
        .unwrap_or(arguments.len());
    let (original_return_arguments, input_arguments) =
        arguments.split_at_mut(return_arguments_count);
    // Builtin call returns <2 x i32>, we have to unpack the vector and insert
    // conversion for predicate
    let call_return_arguments = [(
        id_defs.register_intermediate(Some((
            ast::Type::Vector(ast::ScalarType::U32, 2),
            ast::StateSpace::Reg,
        ))),
        ast::Type::Vector(ast::ScalarType::U32, 2),
        ast::StateSpace::Reg,
    )];
    let fn_id = ptx_impl_imports.add_or_get_declaration(
        id_defs,
        fn_name,
        call_return_arguments
            .iter()
            .map(|(_, typ, state)| (typ, *state)),
        input_arguments.iter().map(|(_, typ, state)| (typ, *state)),
    )?;
    statements.push(Statement::Call(ResolvedCall {
        uniform: false,
        name: fn_id,
        return_arguments: call_return_arguments.to_vec(),
        input_arguments: arguments_to_resolved_arguments(input_arguments),
        is_indirect: false,
    }));
    let unpacked_elements = [
        original_return_arguments[0].0.op,
        id_defs.register_intermediate(Some((
            ast::Type::Scalar(ast::ScalarType::U32),
            ast::StateSpace::Reg,
        ))),
    ];
    statements.push(Statement::RepackVector(RepackVectorDetails {
        is_extract: true,
        typ: ast::ScalarType::U32,
        packed: call_return_arguments[0].0,
        unpacked: unpacked_elements.to_vec(),
        non_default_implicit_conversion: None,
    }));
    let constant_1 = id_defs.register_intermediate(Some((
        ast::Type::Scalar(ast::ScalarType::U32),
        ast::StateSpace::Reg,
    )));
    statements.push(Statement::Constant(ConstantDefinition {
        dst: constant_1,
        typ: ast::ScalarType::U32,
        value: ast::ImmediateValue::U64(1),
    }));
    statements.push(Statement::Instruction(ast::Instruction::Setp(
        ast::SetpData {
            typ: ast::ScalarType::U32,
            flush_to_zero: None,
            cmp_op: ast::SetpCompareOp::Eq,
        },
        ast::Arg4Setp {
            dst1: original_return_arguments[1].0.op,
            dst2: None,
            src1: unpacked_elements[1],
            src2: constant_1,
        },
    )));
    Ok(())
}

// HACK ALERT!
// This function is a "good enough" heuristic of whetever to mark f16/f32 operations
// in the kernel as flushing denorms to zero or preserving them
// PTX support per-instruction ftz information. Unfortunately LLVM has no
// such capability, so instead we guesstimate which use is more common in a
// method and emit suitable attributes
fn compute_denorm_statistics<'input, P: ast::ArgParams<Id = Id>>(
    module: &TranslationModule<'input, P>,
) -> FxHashMap<Id, DenormSummary> {
    let mut denorm_methods = FxHashMap::default();
    for directive in module.directives.iter() {
        match directive {
            TranslationDirective::Variable(..)
            | TranslationDirective::Method(TranslationMethod { body: None, .. }) => {}
            TranslationDirective::Method(TranslationMethod {
                name,
                body: Some(statements),
                ..
            }) => {
                let mut fp32_flush_count = 0isize;
                let mut nonfp32_flush_count = 0isize;
                for statement in statements {
                    match statement {
                        Statement::Instruction(inst) => match inst.flush_to_zero() {
                            Some((flush, 4)) => {
                                fp32_flush_count += if flush { 1 } else { -1 };
                            }
                            Some((flush, _)) => {
                                nonfp32_flush_count += if flush { 1 } else { -1 };
                            }
                            None => {}
                        },
                        Statement::LoadVar(..) => {}
                        Statement::StoreVar(..) => {}
                        Statement::Call(_) => {}
                        Statement::Conditional(_) => {}
                        Statement::Conversion(_) => {}
                        Statement::Constant(_) => {}
                        Statement::RetValue(_, _) => {}
                        Statement::Label(_) => {}
                        Statement::Variable(_) => {}
                        Statement::PtrAccess { .. } => {}
                        Statement::RepackVector(_) => {}
                        Statement::FunctionPointer(_) => {}
                        Statement::MadC(_) => {}
                        Statement::MadCC(_) => {}
                        Statement::AddC(..) => {}
                        Statement::AddCC(..) => {}
                        Statement::SubC(..) => {}
                        Statement::SubCC(..) => {}
                        Statement::AsmVolatile { .. } => {}
                    }
                }
                let summary = DenormSummary {
                    f32: if fp32_flush_count > 0 {
                        FPDenormMode::FlushToZero
                    } else {
                        FPDenormMode::Preserve
                    },
                    f16f64: if nonfp32_flush_count > 0 {
                        FPDenormMode::FlushToZero
                    } else {
                        FPDenormMode::Preserve
                    },
                };
                denorm_methods.insert(*name, summary);
            }
        }
    }
    denorm_methods
}

#[derive(Copy, Clone)]
pub(crate) struct DenormSummary {
    pub(crate) f32: FPDenormMode,
    pub(crate) f16f64: FPDenormMode,
}

pub fn to_llvm_module<'input>(
    compilation_mode: CompilationMode,
    ast: Vec<ast::Module<'input>>,
) -> Result<Module, TranslateError> {
    to_llvm_module_impl2(compilation_mode, ast, None)
}

pub fn to_llvm_module_for_raytracing<'input>(
    ast: ast::Module<'input>,
    raytracing_fn: &str,
    cumulative_attribute_variables: &VariablesBlock,
) -> Result<RaytracingModule<'input>, TranslateError> {
    let mut raytracing_state =
        RaytracingTranslationState::new(raytracing_fn, cumulative_attribute_variables);
    let compilation_module = to_llvm_module_impl2(
        CompilationMode::Wave32,
        vec![ast],
        Some(&mut raytracing_state),
    )?;
    let entry_point_kind: RaytracingEntryPointKind = raytracing_state.entry_point_kind.unwrap();
    let rt_section = hip_common::kernel_metadata::zluda_rt6::write(
        &raytracing_state.new_attribute_variables,
        &raytracing_state.variables,
        entry_point_kind == RaytracingEntryPointKind::Callable,
    );
    let mut linker_module = Vec::new();
    emit::emit_section(
        hip_common::kernel_metadata::zluda_rt6::SECTION_STR,
        &rt_section,
        &mut linker_module,
    );
    Ok(RaytracingModule::new(
        raytracing_state.kernel_name,
        compilation_module,
        raytracing_state.variables,
        entry_point_kind,
        raytracing_state.new_attribute_variables,
        linker_module,
    ))
}

pub(crate) struct RaytracingTranslationState<'a, 'input> {
    pub(crate) entry_point_str: &'a str,
    pub(crate) entry_point_id: Option<Id>,
    pub(crate) entry_point_kind: Option<RaytracingEntryPointKind>,
    pub(crate) kernel_name: String,
    pub(crate) buffers: FxHashMap<Id, Cow<'input, str>>,
    pub(crate) variables: VariablesBlock,
    pub(crate) old_attribute_variables: &'a VariablesBlock,
    pub(crate) new_attribute_variables: VariablesBlock,
    pub(crate) reachable_user_functions: FxHashSet<Id>,
}

impl<'a, 'input> RaytracingTranslationState<'a, 'input> {
    fn new(entry_point_str: &'a str, cumulative_attribute_variables: &'a VariablesBlock) -> Self {
        Self {
            entry_point_str,
            old_attribute_variables: cumulative_attribute_variables,
            entry_point_id: None,
            entry_point_kind: None,
            kernel_name: String::new(),
            buffers: FxHashMap::default(),
            variables: VariablesBlock::empty(),
            new_attribute_variables: VariablesBlock::empty(),
            reachable_user_functions: FxHashSet::default(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum RaytracingEntryPointKind {
    BoundingBox,
    Intersection,
    Callable,
    // Closest hit, any hit, attribute, ray generation, exception, miss
    Unknown,
}

fn to_llvm_module_impl2<'a, 'input>(
    compilation_mode: CompilationMode,
    asts: Vec<ast::Module<'input>>,
    mut raytracing: Option<&mut RaytracingTranslationState<'a, 'input>>,
) -> Result<Module<'input>, TranslateError> {
    let empty_module = if raytracing.is_some() {
        raytracing::create_module_with_builtins()
    } else {
        TranslationModule::new(compilation_mode)
    };
    let linking = resolve_linking(&*asts, raytracing.is_some())?;
    let (mut translation_module, functions) =
        link_and_normalize_modules(asts, empty_module, linking)?;
    if let Some(ref mut raytracing_state) = raytracing {
        translation_module = raytracing::run_on_normalized(translation_module, raytracing_state)?;
    }
    let translation_module = extract_builtin_functions(translation_module);
    let translation_module = resolve_instruction_types(translation_module, functions)?;
    let mut translation_module = restructure_function_return_types(translation_module)?;
    if let Some(ref mut raytracing_state) = raytracing {
        translation_module = raytracing::run_on_typed(translation_module, raytracing_state)?;
    }
    let translation_module = deparamize_function_declarations(translation_module)?;
    let translation_module = insert_hardware_registers(translation_module)?;
    let translation_module = fix_special_registers(translation_module)?;
    let translation_module = insert_mem_ssa_statements(translation_module)?;
    let translation_module = expand_arguments(translation_module)?;
    let mut translation_module = deparamize_variable_declarations(translation_module)?;
    if let Some(ref mut raytracing_state) = raytracing {
        // raytracing passes rely heavily on particular PTX patterns, they must run before implicit conversions
        translation_module = raytracing::postprocess(translation_module, raytracing_state)?;
    }
    let translation_module = insert_implicit_conversions(translation_module)?;
    let translation_module = insert_compilation_mode_prologue(translation_module);
    let translation_module = hoist_globals(translation_module);
    let mut translation_module = replace_instructions_with_builtins(translation_module)?;
    if raytracing.is_some() {
        translation_module = raytracing::replace_tex_builtins_hack(translation_module)?;
    }
    let call_graph = CallGraph::new(&translation_module.directives);
    let translation_module = convert_dynamic_shared_memory_usage(translation_module, &call_graph)?;
    let denorm_statistics = compute_denorm_statistics(&translation_module);
    let kernel_arguments = get_kernel_arguments(&translation_module.directives)?;
    let mut bitcode_modules = vec![ZLUDA_PTX_IMPL_AMD];
    if raytracing.is_some() {
        bitcode_modules.push(raytracing::bitcode());
    }
    let metadata = create_metadata(&translation_module);
    let (llvm_context, llvm_module) = unsafe {
        emit::emit_llvm_bitcode_and_linker_module(translation_module, denorm_statistics)?
    };
    Ok(Module {
        metadata,
        compilation_mode,
        llvm_module,
        kernel_arguments,
        _llvm_context: llvm_context,
        bitcode_modules,
    })
}

// PTX definition of param state space does not translate cleanly into AMDGPU notion of an address space:
//  .param in kernel arguments matches AMDGPU constant address space
//  .param in function arguments and variables matches AMDGPU private address space
// This pass converts all instances of declarations in .param state space into constant or local state space appropriately
// Previously we used AMDPGU generic address space for params and left it for LLVM to infer the right non-generic space,
// but this made LLVM crash on some inputs (e.g. test alloca_call.ptx)
fn deparamize_variable_declarations<'input>(
    mut module: TranslationModule<'input, ExpandedArgParams>,
) -> Result<TranslationModule<'input, ExpandedArgParams>, TranslateError> {
    let id_def = &mut module.id_defs;
    for directive in module.directives.iter_mut() {
        match directive {
            TranslationDirective::Variable(..) => {}
            TranslationDirective::Method(method) => {
                let mut new_space = FxHashMap::default();
                if method.is_kernel {
                    let input_arguments: Vec<ast::VariableDeclaration<Id>> =
                        mem::replace(&mut method.input_arguments, Vec::new());
                    let input_arguments = input_arguments
                        .into_iter()
                        .map(|arg| {
                            if arg.state_space == ast::StateSpace::Param {
                                let new_arg = id_def.register_variable_decl(
                                    arg.align,
                                    arg.type_.clone(),
                                    ast::StateSpace::Const,
                                );
                                new_space.insert(arg.name, (new_arg.name, ast::StateSpace::Const));
                                new_arg
                            } else {
                                arg
                            }
                        })
                        .collect::<Vec<_>>();
                    method.input_arguments = input_arguments;
                }
                method.body = method
                    .body
                    .take()
                    .map(|old_body| {
                        deparamize_variable_declarations_convert_body(id_def, new_space, old_body)
                    })
                    .transpose()?;
            }
        }
    }
    Ok(module)
}

fn deparamize_variable_declarations_convert_body<'input>(
    id_def: &mut IdNameMapBuilder<'input>,
    mut new_space: FxHashMap<Id, (Id, ast::StateSpace)>,
    fn_body: Vec<ExpandedStatement>,
) -> Result<Vec<ExpandedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(fn_body.len());
    for statement in fn_body {
        match statement {
            Statement::Instruction(ast::Instruction::Mov(details, mut args)) => {
                if let Some((new_name, _)) = new_space.get(&args.src) {
                    args.src = *new_name;
                }
                result.push(Statement::Instruction(ast::Instruction::Mov(details, args)));
            }
            Statement::Variable(
                var @ Variable {
                    state_space: ast::StateSpace::Param,
                    ..
                },
            ) => {
                let old_name = var.name;
                let new_var = id_def.register_variable_def(
                    var.align,
                    var.type_,
                    ast::StateSpace::Local,
                    var.initializer,
                );
                new_space.insert(old_name, (new_var.name, ast::StateSpace::Local));
                result.push(Statement::Variable(new_var));
            }
            Statement::PtrAccess(
                mut ptr @ PtrAccess {
                    state_space: ast::StateSpace::Param,
                    ..
                },
            ) => {
                if let Some((new_name, new_space)) = new_space.get(&ptr.ptr_src) {
                    ptr.state_space = *new_space;
                    ptr.ptr_src = *new_name;
                } else {
                    ptr.state_space = ast::StateSpace::Const;
                }
                let old_name = ptr.dst;
                ptr.dst = id_def
                    .register_intermediate(Some((ptr.underlying_type.clone(), ptr.state_space)));
                new_space.insert(old_name, (ptr.dst, ptr.state_space));
                result.push(Statement::PtrAccess(ptr));
            }
            Statement::Instruction(ast::Instruction::St(
                mut details @ ast::StData {
                    state_space: ast::StateSpace::Param,
                    ..
                },
                mut args,
            )) => {
                if let Some((new_name, new_space)) = new_space.get(&args.src1) {
                    details.state_space = *new_space;
                    args.src1 = *new_name;
                } else {
                    details.state_space = ast::StateSpace::Const;
                }
                result.push(Statement::Instruction(ast::Instruction::St(details, args)));
            }
            Statement::Instruction(ast::Instruction::Ld(
                mut details @ ast::LdDetails {
                    state_space: ast::StateSpace::Param,
                    ..
                },
                mut args,
            )) => {
                if let Some((new_name, new_space)) = new_space.get(&args.src) {
                    details.state_space = *new_space;
                    args.src = *new_name;
                } else {
                    details.state_space = ast::StateSpace::Const;
                }
                result.push(Statement::Instruction(ast::Instruction::Ld(details, args)));
            }
            s => result.push(s),
        }
    }
    Ok(result)
}

fn create_metadata<'input>(
    translation_module: &TranslationModule<'input, ExpandedArgParams>,
) -> Metadata<'input> {
    let mut kernel_metadata = Vec::new();
    for directive in translation_module.directives.iter() {
        match directive {
            TranslationDirective::Method(method) => {
                if method.tuning.is_empty() {
                    continue;
                }
                let name = match method.source_name {
                    Some(ref name) => name.clone(),
                    None => continue,
                };
                for tuning in method.tuning.iter().copied() {
                    match tuning {
                        // TODO: measure
                        ast::TuningDirective::MaxNReg(_)
                        | ast::TuningDirective::MinNCtaPerSm(_) => {}
                        ast::TuningDirective::MaxNtid(x, y, z) => {
                            let size = x as u64 * y as u64 * z as u64;
                            kernel_metadata.push((
                                name.clone(),
                                None,
                                NonZeroU32::new(size as u32),
                            ));
                        }
                        ast::TuningDirective::ReqNtid(x, y, z) => {
                            let size = x as u64 * y as u64 * z as u64;
                            kernel_metadata.push((
                                name.clone(),
                                NonZeroU32::new(size as u32),
                                NonZeroU32::new(size as u32),
                            ));
                        }
                    }
                }
            }
            TranslationDirective::Variable(..) => {}
        }
    }
    Metadata {
        sm_version: translation_module.sm_version,
        kernel_metadata,
    }
}

fn insert_compilation_mode_prologue<'input>(
    mut translation_module: TranslationModule<'input, ExpandedArgParams>,
) -> TranslationModule<'input, ExpandedArgParams> {
    if translation_module.compilation_mode != CompilationMode::Wave32OnWave64 {
        return translation_module;
    }
    for directive in translation_module.directives.iter_mut() {
        match directive {
            TranslationDirective::Method(TranslationMethod {
                is_kernel,
                body: Some(body),
                tuning,
                ..
            }) => {
                for t in tuning.iter_mut() {
                    match t {
                        ast::TuningDirective::MaxNReg(_)
                        | ast::TuningDirective::MinNCtaPerSm(_) => {}
                        ast::TuningDirective::MaxNtid(_, _, z) => {
                            *z *= 2;
                        }
                        ast::TuningDirective::ReqNtid(_, _, z) => {
                            *z *= 2;
                        }
                    }
                }
                if !*is_kernel {
                    continue;
                }
                let old_body = mem::replace(body, Vec::new());
                let mut new_body = Vec::with_capacity(old_body.len() + 1);
                // I'd rather use early exit on laneid like a normal person,
                // but that leads to miscompilations, so here's the next best thing
                let asm = "s_bcnt1_i32_b64 exec_lo, exec\ns_lshr_b32 exec_lo, exec_lo, 1\ns_bfm_b64 exec, exec_lo, 0";
                let constraints = "~{scc}";
                new_body.push(Statement::AsmVolatile { asm, constraints });
                new_body.extend(old_body.into_iter());
                *body = new_body;
            }
            TranslationDirective::Method(..) | TranslationDirective::Variable(..) => {}
        }
    }
    translation_module
}

// THIS PASS IS AN AWFUL HACK TO WORK AROUND LLVM BUG
// In certain situations LLVM will miscompile AMDGPU
// binaries if the return type of a function is  .b8 array.
// For example, if the return of the function is float3 NVIDIA
// frontend compiler will compile it to .b8[12].
// Turns out if the return type is a .b8 array, then LLVM will
// sometimes be unable to remove the alloca.
// Which is fine, but for some reason AMDGPU has a bug
// where it does not deallocate alloca
// Our """solution""" is to convert all b8[] into b32[]
fn restructure_function_return_types(
    mut module: TranslationModule<TypedArgParams>,
) -> Result<TranslationModule<TypedArgParams>, TranslateError> {
    let id_defs = &mut module.id_defs;
    module.directives = module
        .directives
        .into_iter()
        .map(|directive| avoid_byte_array_returns(id_defs, directive))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(module)
}

fn avoid_byte_array_returns<'input>(
    id_defs: &mut IdNameMapBuilder<'input>,
    mut directive: TranslationDirective<'input, TypedArgParams>,
) -> Result<TranslationDirective<'input, TypedArgParams>, TranslateError> {
    match directive {
        TranslationDirective::Method(ref mut method) => {
            if !method.is_kernel {
                let return_arguments = &mut method.return_arguments;
                let input_arguments = &mut method.input_arguments;
                for arg in return_arguments
                    .iter_mut()
                    .chain(input_arguments.iter_mut())
                {
                    if let (
                        ast::Type::Array(
                            ref mut scalar_type @ ast::ScalarType::B8,
                            ref mut dimensions,
                        ),
                        _,
                    ) = (&mut arg.type_, arg.state_space)
                    {
                        if dimensions.len() > 1 {
                            return Err(TranslateError::unexpected_pattern());
                        }
                        *scalar_type = ast::ScalarType::B32;
                        dimensions[0] = div_positive_round_up(dimensions[0], 4);
                        id_defs.change_type(
                            arg.name,
                            ast::Type::Array(ast::ScalarType::B32, dimensions.clone()),
                        )?;
                    }
                }
            }
            for statement in method.body.iter_mut().flatten() {
                if let Statement::Call(call) = statement {
                    let return_arguments = &mut call.return_arguments;
                    let input_arguments = &mut call.input_arguments;
                    for (type_, space) in return_arguments
                        .iter_mut()
                        .map(|(_, t, s)| (t, s))
                        .chain(input_arguments.iter_mut().map(|(_, t, s)| (t, s)))
                    {
                        if let (
                            ast::Type::Array(
                                ref mut scalar_type @ ast::ScalarType::B8,
                                ref mut dimensions,
                            ),
                            _,
                        ) = (type_, space)
                        {
                            if dimensions.len() > 1 {
                                return Err(TranslateError::unexpected_pattern());
                            }
                            *scalar_type = ast::ScalarType::B32;
                            dimensions[0] = div_positive_round_up(dimensions[0], 4);
                        }
                    }
                }
                if let Statement::Variable(Variable {
                    state_space: ast::StateSpace::Param,
                    type_:
                        ast::Type::Array(ref mut scalar_type @ ast::ScalarType::B8, ref mut dimensions),
                    ..
                }) = statement
                {
                    if dimensions.len() > 1 {
                        return Err(TranslateError::unexpected_pattern());
                    }
                    *scalar_type = ast::ScalarType::B32;
                    dimensions[0] = div_positive_round_up(dimensions[0], 4);
                }
            }
            Ok(directive)
        }
        TranslationDirective::Variable(..) => Ok(directive),
    }
}

fn div_positive_round_up(dividend: u32, divisor: u32) -> u32 {
    let mut result = dividend / divisor;
    if (dividend % divisor) != 0 {
        result += 1;
    }
    result
}

fn get_kernel_arguments(
    directives: &[Directive],
) -> Result<FxHashMap<String, Vec<Layout>>, TranslateError> {
    let mut result = FxHashMap::default();
    for directive in directives.iter() {
        match directive {
            Directive::Method(TranslationMethod {
                is_kernel: true,
                source_name,
                input_arguments,
                ..
            }) => {
                let name = match source_name {
                    Some(name) => name,
                    None => continue,
                };
                let layout = input_arguments
                    .iter()
                    .map(|var| var.layout())
                    .collect::<Vec<_>>();
                result.insert(name.to_string(), layout);
            }
            _ => continue,
        }
    }
    Ok(result)
}

pub(crate) struct CallGraph {
    pub(crate) all_callees: FxHashMap<Id, FxHashSet<Id>>,
}

// TODO: resolve declarations
impl CallGraph {
    pub(crate) fn new<'input, P: ast::ArgParams<Id = Id>>(
        module: &[TranslationDirective<'input, P>],
    ) -> Self {
        let mut has_body = FxHashSet::default();
        let mut direct_callees = FxHashMap::default();
        for directive in module {
            match directive {
                TranslationDirective::Method(TranslationMethod {
                    name,
                    body: Some(statements),
                    ..
                }) => {
                    let call_key = *name;
                    has_body.insert(call_key);
                    if let hash_map::Entry::Vacant(entry) = direct_callees.entry(call_key) {
                        entry.insert(Vec::new());
                    }
                    for statement in statements {
                        match statement {
                            Statement::Call(ResolvedCall {
                                name,
                                is_indirect: false,
                                ..
                            }) => {
                                multi_hash_map_append(&mut direct_callees, call_key, *name);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        let mut result = FxHashMap::default();
        for (&method_key, children) in direct_callees.iter() {
            let mut visited = FxHashSet::default();
            for child in children {
                if !has_body.contains(child) {
                    continue;
                }
                Self::add_call_map_single(&has_body, &direct_callees, &mut visited, *child);
            }
            result.insert(method_key, visited);
        }
        CallGraph {
            all_callees: result,
        }
    }

    fn add_call_map_single(
        has_body: &FxHashSet<Id>,
        directly_called_by: &FxHashMap<Id, Vec<Id>>,
        visited: &mut FxHashSet<Id>,
        current: Id,
    ) {
        if !visited.insert(current) {
            return;
        }
        if let Some(children) = directly_called_by.get(&current) {
            for child in children {
                if !has_body.contains(child) {
                    continue;
                }
                Self::add_call_map_single(has_body, directly_called_by, visited, *child);
            }
        }
    }

    fn methods(&self) -> impl Iterator<Item = (Id, &FxHashSet<Id>)> {
        self.all_callees
            .iter()
            .map(|(method, children)| (*method, children))
    }
}

fn multi_hash_map_append<
    K: Eq + std::hash::Hash,
    V,
    Collection: std::iter::Extend<V> + std::default::Default,
>(
    m: &mut FxHashMap<K, Collection>,
    key: K,
    value: V,
) {
    match m.entry(key) {
        hash_map::Entry::Occupied(mut entry) => {
            entry.get_mut().extend(iter::once(value));
        }
        hash_map::Entry::Vacant(entry) => {
            entry.insert(Default::default()).extend(iter::once(value));
        }
    }
}

/*
    PTX represents dynamically allocated shared local memory as
        .extern .shared .b32 shared_mem[];
    In SPIRV/OpenCL world this is expressed as an additional argument to the kernel
    And in AMD compilation
    This pass looks for all uses of .extern .shared and converts them to
    an additional method argument
    The question is how this artificial argument should be expressed. There are
    several options:
    * Straight conversion:
        .shared .b32 shared_mem[]
    * Introduce .param_shared statespace:
        .param_shared .b32 shared_mem
        or
        .param_shared .b32 shared_mem[]
    * Introduce .shared_ptr <SCALAR> type:
        .param .shared_ptr .b32 shared_mem
    * Reuse .ptr hint:
        .param .u64 .ptr shared_mem
      This is the most tempting, but also the most nonsensical, .ptr is just a
      hint, which has no semantical meaning (and the output of our
      transformation has a semantical meaning - we emit additional
      "OpFunctionParameter ..." with type "OpTypePointer Workgroup ...")
*/
fn convert_dynamic_shared_memory_usage<'input>(
    mut module: TranslationModule<'input, ExpandedArgParams>,
    kernels_methods_call_map: &CallGraph,
) -> Result<TranslationModule<'input, ExpandedArgParams>, TranslateError> {
    let mut globals_shared = FxHashMap::default();
    for dir in module.directives.iter() {
        match dir {
            Directive::Variable(
                _,
                _,
                Variable {
                    state_space: ast::StateSpace::Shared,
                    name,
                    type_,
                    ..
                },
            ) => {
                globals_shared.insert(*name, type_.clone());
            }
            _ => {}
        }
    }
    if globals_shared.len() == 0 {
        return Ok(module);
    }
    let mut methods_to_directly_used_shared_globals = FxHashMap::<_, FxHashSet<Id>>::default();
    let remapped_directives = module
        .directives
        .into_iter()
        .map(|directive| match directive {
            Directive::Method(TranslationMethod {
                return_arguments,
                name,
                input_arguments,
                body: Some(statements),
                tuning,
                is_kernel,
                source_name,
                special_raytracing_linking: raytracing_linking,
            }) => {
                let call_key = name;
                let statements = statements
                    .into_iter()
                    .map(|statement| {
                        statement.map_id(&mut |id, _| {
                            if globals_shared.get(&id).is_some() {
                                methods_to_directly_used_shared_globals
                                    .entry(call_key)
                                    .or_insert_with(FxHashSet::default)
                                    .insert(id);
                            }
                            id
                        })
                    })
                    .collect();
                Directive::Method(TranslationMethod {
                    return_arguments,
                    name,
                    input_arguments,
                    body: Some(statements),
                    tuning,
                    is_kernel,
                    source_name,
                    special_raytracing_linking: raytracing_linking,
                })
            }
            directive => directive,
        })
        .collect::<Vec<_>>();
    // If there's a chain `kernel` -> `fn1` -> `fn2`, where only `fn2` uses extern shared,
    // make sure it gets propagated to `fn1` and `kernel`
    let methods_to_indirectly_used_shared_globals = resolve_indirect_uses_of_globals_shared(
        methods_to_directly_used_shared_globals,
        kernels_methods_call_map,
    );
    // now visit every method declaration and inject those additional arguments
    let mut directives = Vec::with_capacity(remapped_directives.len());
    for directive in remapped_directives.into_iter() {
        match directive {
            Directive::Method(TranslationMethod {
                return_arguments,
                name,
                mut input_arguments,
                body,
                tuning,
                is_kernel,
                source_name,
                special_raytracing_linking: raytracing_linking,
            }) => {
                let statements: Option<
                    Vec<Statement<ast::Instruction<ExpandedArgParams>, ExpandedArgParams>>,
                > = {
                    insert_arguments_remap_statements(
                        &mut module.id_defs.id_gen,
                        &globals_shared,
                        &methods_to_indirectly_used_shared_globals,
                        name,
                        is_kernel,
                        &mut input_arguments,
                        body,
                    )?
                };
                directives.push(Directive::Method(TranslationMethod {
                    return_arguments,
                    name,
                    input_arguments,
                    body: statements,
                    tuning,
                    is_kernel,
                    source_name,
                    special_raytracing_linking: raytracing_linking,
                }));
            }
            directive => directives.push(directive),
        }
    }
    Ok(TranslationModule {
        directives,
        ..module
    })
}

fn insert_arguments_remap_statements<'input>(
    new_id: &mut IdGenerator,
    globals_shared: &FxHashMap<Id, ast::Type>,
    methods_to_indirectly_used_shared_globals: &FxHashMap<Id, BTreeSet<Id>>,
    method_name: Id,
    is_kernel: bool,
    input_arguments: &mut Vec<ast::VariableDeclaration<Id>>,
    statements: Option<Vec<Statement<ast::Instruction<ExpandedArgParams>, ExpandedArgParams>>>,
) -> Result<
    Option<Vec<Statement<ast::Instruction<ExpandedArgParams>, ExpandedArgParams>>>,
    TranslateError,
> {
    let method_globals = match methods_to_indirectly_used_shared_globals.get(&method_name) {
        Some(method_globals) => method_globals,
        None => return Ok(statements),
    };
    let remapped_globals_in_method = method_globals
        .iter()
        .map(|global| {
            Ok((
                *global,
                (
                    if is_kernel { *global } else { new_id.next() },
                    globals_shared
                        .get(&global)
                        .ok_or_else(TranslateError::todo)?
                        .clone(),
                ),
            ))
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    if !is_kernel {
        for (_, (new_shared_global_id, shared_global_type)) in remapped_globals_in_method.iter() {
            input_arguments.push(ast::VariableDeclaration {
                align: None,
                type_: shared_global_type.clone(),
                state_space: ast::StateSpace::Shared,
                name: *new_shared_global_id,
            });
        }
    }
    Ok(statements.map(|statements| {
        replace_uses_of_shared_memory(
            methods_to_indirectly_used_shared_globals,
            statements,
            remapped_globals_in_method,
        )
    }))
}

fn replace_uses_of_shared_memory<'input>(
    methods_to_indirectly_used_shared_globals: &FxHashMap<Id, BTreeSet<Id>>,
    statements: Vec<ExpandedStatement>,
    remapped_globals_in_method: BTreeMap<Id, (Id, ast::Type)>,
) -> Vec<ExpandedStatement> {
    let mut result = Vec::with_capacity(statements.len());
    for statement in statements {
        match statement {
            Statement::Call(mut call) => {
                // We can safely skip checking call arguments,
                // because there's simply no way to pass shared ptr
                // without converting it to .b64 first
                if let Some(shared_globals_used_by_callee) =
                    methods_to_indirectly_used_shared_globals.get(&call.name)
                {
                    for &shared_global_used_by_callee in shared_globals_used_by_callee {
                        let (remapped_shared_id, type_) = remapped_globals_in_method
                            .get(&shared_global_used_by_callee)
                            .unwrap_or_else(|| todo!());
                        call.input_arguments.push((
                            *remapped_shared_id,
                            type_.clone(),
                            ast::StateSpace::Shared,
                        ));
                    }
                }
                result.push(Statement::Call(call))
            }
            statement => {
                let new_statement = statement.map_id(&mut |id, _| {
                    if let Some((remapped_shared_id, _)) = remapped_globals_in_method.get(&id) {
                        *remapped_shared_id
                    } else {
                        id
                    }
                });
                result.push(new_statement);
            }
        }
    }
    result
}

// We need to compute two kinds of information:
// * If it's a kernel -> size of .shared globals in use (direct or indirect)
// * If it's a function -> does it use .shared global (directly or indirectly)
fn resolve_indirect_uses_of_globals_shared<'input>(
    methods_use_of_globals_shared: FxHashMap<Id, FxHashSet<Id>>,
    kernels_methods_call_map: &CallGraph,
) -> FxHashMap<Id, BTreeSet<Id>> {
    let mut result = FxHashMap::default();
    for (method, callees) in kernels_methods_call_map.methods() {
        let mut indirect_globals = methods_use_of_globals_shared
            .get(&method)
            .into_iter()
            .flatten()
            .copied()
            .collect::<BTreeSet<_>>();
        for &callee in callees {
            indirect_globals.extend(
                methods_use_of_globals_shared
                    .get(&callee)
                    .into_iter()
                    .flatten()
                    .copied(),
            );
        }
        result.insert(method, indirect_globals);
    }
    result
}

struct SpecialRegisterResolver<'a, 'input> {
    id_defs: &'a mut IdNameMapBuilder<'input>,
    ptx_imports: &'a mut AdditionalFunctionDeclarations,
    result: Vec<TypedStatement>,
}

impl<'a, 'input> SpecialRegisterResolver<'a, 'input> {
    fn replace_sreg(
        &mut self,
        desc: ArgumentDescriptor<Id>,
        vector_index: Option<u8>,
    ) -> Result<Id, TranslateError> {
        if let Some(sreg) = self.id_defs.globals.special_registers.get(desc.op) {
            if desc.is_dst {
                return Err(TranslateError::mismatched_type());
            }
            let input_arguments = match (vector_index, sreg.get_function_input_type()) {
                (Some(idx), Some(inp_type)) => {
                    if inp_type != ast::ScalarType::U8 {
                        return Err(TranslateError::unreachable());
                    }
                    let constant = self.id_defs.register_intermediate(Some((
                        ast::Type::Scalar(inp_type),
                        ast::StateSpace::Reg,
                    )));
                    self.result.push(Statement::Constant(ConstantDefinition {
                        dst: constant,
                        typ: inp_type,
                        value: ast::ImmediateValue::U64(idx as u64),
                    }));
                    vec![(
                        TypedOperand::Reg(constant),
                        ast::Type::Scalar(inp_type),
                        ast::StateSpace::Reg,
                    )]
                }
                (None, None) => Vec::new(),
                _ => return Err(TranslateError::mismatched_type()),
            };
            let ocl_fn_name = [ZLUDA_PTX_PREFIX, sreg.get_unprefixed_function_name()].concat();
            let return_type = sreg.get_function_return_type();
            let fn_result = self.id_defs.register_intermediate(Some((
                ast::Type::Scalar(return_type),
                ast::StateSpace::Reg,
            )));
            let return_arguments = vec![(
                fn_result,
                ast::Type::Scalar(return_type),
                ast::StateSpace::Reg,
            )];
            let fn_call = self.ptx_imports.add_or_get_declaration(
                self.id_defs,
                ocl_fn_name.to_string(),
                return_arguments.iter().map(|(_, typ, space)| (typ, *space)),
                input_arguments.iter().map(|(_, typ, space)| (typ, *space)),
            )?;
            self.result.push(Statement::Call(ResolvedCall {
                uniform: false,
                return_arguments,
                name: fn_call,
                input_arguments,
                is_indirect: false,
            }));
            Ok(fn_result)
        } else {
            Ok(desc.op)
        }
    }
}

impl<'a, 'input> ArgumentMapVisitor<TypedArgParams, TypedArgParams>
    for SpecialRegisterResolver<'a, 'input>
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<Id>,
        _: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<Id, TranslateError> {
        self.replace_sreg(desc, None)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<TypedOperand>,
        _typ: &ast::Type,
        _state_space: ast::StateSpace,
    ) -> Result<TypedOperand, TranslateError> {
        Ok(match desc.op {
            TypedOperand::Reg(reg) => TypedOperand::Reg(self.replace_sreg(desc.new_op(reg), None)?),
            op @ TypedOperand::RegOffset(_, _) => op,
            op @ TypedOperand::Imm(_) => op,
            TypedOperand::VecMember(reg, idx) => {
                TypedOperand::VecMember(self.replace_sreg(desc.new_op(reg), Some(idx))?, idx)
            }
        })
    }
}

fn extract_global_cvt<'input>(
    local: &mut Vec<ExpandedStatement>,
    ptx_impl_imports: &mut AdditionalFunctionDeclarations,
    id_def: &mut IdNameMapBuilder<'input>,
    desc: ast::CvtDesc,
    inst: ast::Instruction<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let fn_name = [
        ZLUDA_PTX_PREFIX,
        "cvt_",
        rounding_to_ptx_name(desc.rounding),
        "_",
        desc.dst.to_ptx_name(),
        "_",
        desc.src.to_ptx_name(),
    ]
    .concat();
    local.push(instruction_to_fn_call(
        id_def,
        ptx_impl_imports,
        inst,
        fn_name,
    )?);
    Ok(())
}

fn rounding_to_ptx_name(this: Option<ast::RoundingMode>) -> &'static str {
    match this {
        None | Some(ast::RoundingMode::NearestEven) => "rn",
        Some(ast::RoundingMode::Zero) => "rz",
        Some(ast::RoundingMode::NegativeInf) => "rm",
        Some(ast::RoundingMode::PositiveInf) => "rp",
    }
}

impl ast::AtomSemantics {
    fn to_ptx_name(self) -> &'static str {
        match self {
            ast::AtomSemantics::Relaxed => "relaxed",
            ast::AtomSemantics::Acquire => "acquire",
            ast::AtomSemantics::Release => "release",
            ast::AtomSemantics::AcquireRelease => "acq_rel",
        }
    }
}

impl ast::MemScope {
    fn to_ptx_name(self) -> &'static str {
        match self {
            ast::MemScope::Cta => "cta",
            ast::MemScope::Gpu => "gpu",
            ast::MemScope::Sys => "sys",
        }
    }
}

impl ast::StateSpace {
    fn to_ptx_name(self) -> &'static str {
        match self {
            ast::StateSpace::Generic => "generic",
            ast::StateSpace::Global => "global",
            ast::StateSpace::Shared => "shared",
            ast::StateSpace::Reg => "reg",
            ast::StateSpace::Const => "const",
            ast::StateSpace::Local => "local",
            ast::StateSpace::Param => "param",
            ast::StateSpace::Sreg => "sreg",
        }
    }
}

impl ast::ShflMode {
    fn to_ptx_name(self) -> &'static str {
        match self {
            ast::ShflMode::Up => "up",
            ast::ShflMode::Down => "down",
            ast::ShflMode::Bfly => "bfly",
            ast::ShflMode::Idx => "idx",
        }
    }
}
struct VectorRepackVisitor<'a, 'input, V> {
    extra_vistor: &'a mut V,
    func: &'a mut Vec<TypedStatement>,
    id_def: &'a mut IdNameMapBuilder<'input>,
    post_stmts: Option<TypedStatement>,
}

impl<'a, 'input, V> VectorRepackVisitor<'a, 'input, V> {
    fn new(
        extra_vistor: &'a mut V,
        func: &'a mut Vec<TypedStatement>,
        id_def: &'a mut IdNameMapBuilder<'input>,
    ) -> Self {
        VectorRepackVisitor {
            extra_vistor,
            func,
            id_def,
            post_stmts: None,
        }
    }

    fn convert_vector(
        &mut self,
        is_dst: bool,
        non_default_implicit_conversion: Option<
            fn(
                (ast::StateSpace, &ast::Type),
                (ast::StateSpace, &ast::Type),
            ) -> Result<Option<ConversionKind>, TranslateError>,
        >,
        typ: &ast::Type,
        state_space: ast::StateSpace,
        idx: Vec<ast::RegOrImmediate<Id>>,
    ) -> Result<Id, TranslateError> {
        let scalar_t = match typ {
            // mov.v2.u32 foobar, {a,b};
            ast::Type::Vector(scalar_t, _) => *scalar_t,
            // mov.b64 foobar, {a,b};
            ast::Type::Scalar(scalar_t) => {
                if scalar_t.kind() == ast::ScalarKind::Bit {
                    let total_size_of = scalar_t.size_of() as usize;
                    let scalar_size_of = total_size_of / idx.len();
                    if idx.len() * scalar_size_of == total_size_of {
                        ast::ScalarType::from_parts(scalar_size_of as u8, ast::ScalarKind::Bit)
                    } else {
                        return Err(TranslateError::mismatched_type());
                    }
                } else {
                    return Err(TranslateError::mismatched_type());
                }
            }
            _ => return Err(TranslateError::mismatched_type()),
        };
        let temp_vec = self
            .id_def
            .register_intermediate(Some((typ.clone(), state_space)));
        let vector_members = idx
            .into_iter()
            .map(|vector_member| match vector_member {
                ast::RegOrImmediate::Reg(reg) => reg,
                ast::RegOrImmediate::Imm(immediate) => {
                    let (id, statement) = FlattenArguments::immediate_impl(
                        self.id_def,
                        immediate,
                        &ast::Type::Scalar(scalar_t),
                        ast::StateSpace::Reg,
                    );
                    self.func.push(statement);
                    id
                }
            })
            .collect::<Vec<_>>();
        let statement = Statement::RepackVector(RepackVectorDetails {
            is_extract: is_dst,
            typ: scalar_t,
            packed: temp_vec,
            unpacked: vector_members,
            non_default_implicit_conversion,
        });
        if is_dst {
            self.post_stmts = Some(statement);
        } else {
            self.func.push(statement);
        }
        Ok(temp_vec)
    }
}

impl<'a, 'b, V: ArgumentMapVisitor<TypedArgParams, TypedArgParams>>
    ArgumentMapVisitor<NormalizedArgParams, TypedArgParams> for VectorRepackVisitor<'a, 'b, V>
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<Id>,
        type_: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<Id, TranslateError> {
        self.extra_vistor.id(desc, type_)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<ast::Operand<Id>>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<TypedOperand, TranslateError> {
        Ok(match desc.op {
            ast::Operand::Reg(reg) => {
                self.extra_vistor
                    .operand(desc.new_op(TypedOperand::Reg(reg)), typ, state_space)?
            }
            ast::Operand::RegOffset(reg, offset) => TypedOperand::RegOffset(reg, offset),
            ast::Operand::Imm(x) => TypedOperand::Imm(x),
            ast::Operand::VecMember(vec, idx) => TypedOperand::VecMember(vec, idx),
            ast::Operand::VecPack(vec) => TypedOperand::Reg(self.convert_vector(
                desc.is_dst,
                desc.non_default_implicit_conversion,
                typ,
                state_space,
                vec,
            )?),
        })
    }
}

fn instruction_to_fn_call<'input>(
    id_defs: &mut IdNameMapBuilder,
    ptx_impl_imports: &mut AdditionalFunctionDeclarations,
    inst: ast::Instruction<ExpandedArgParams>,
    fn_name: String,
) -> Result<ExpandedStatement, TranslateError> {
    let mut arguments = Vec::new();
    inst.visit(
        &mut |desc: ArgumentDescriptor<Id>, typ: Option<(&ast::Type, ast::StateSpace)>| {
            let (typ, space) = match typ {
                Some((typ, space)) => (typ.clone(), space),
                None => return Err(TranslateError::unreachable()),
            };
            let result = desc.op;
            arguments.push((desc, typ, space));
            Ok(result)
        },
    )?;
    let return_arguments_count = arguments
        .iter()
        .position(|(desc, _, _)| !desc.is_dst)
        .unwrap_or(arguments.len());
    let (return_arguments, input_arguments) = arguments.split_at(return_arguments_count);
    let fn_id = ptx_impl_imports.add_or_get_declaration(
        id_defs,
        fn_name,
        return_arguments.iter().map(|(_, typ, state)| (typ, *state)),
        input_arguments.iter().map(|(_, typ, state)| (typ, *state)),
    )?;
    Ok(Statement::Call(ResolvedCall {
        uniform: false,
        name: fn_id,
        return_arguments: arguments_to_resolved_arguments(return_arguments),
        input_arguments: arguments_to_resolved_arguments(input_arguments),
        is_indirect: false,
    }))
}

fn fn_arguments_to_variables<'a>(
    id_defs: &mut IdNameMapBuilder,
    args: impl Iterator<Item = (&'a ast::Type, ast::StateSpace)>,
) -> Vec<ast::VariableDeclaration<Id>> {
    args.map(|(typ, space)| ast::VariableDeclaration {
        align: None,
        type_: typ.clone(),
        state_space: space,
        name: id_defs.register_intermediate(None),
    })
    .collect::<Vec<_>>()
}

fn arguments_to_resolved_arguments(
    args: &[(ArgumentDescriptor<Id>, ast::Type, ast::StateSpace)],
) -> Vec<(Id, ast::Type, ast::StateSpace)> {
    args.iter()
        .map(|(desc, typ, space)| (desc.op, typ.clone(), *space))
        .collect::<Vec<_>>()
}

fn insert_mem_ssa_argument<'input>(
    id_def: &mut IdNameMapBuilder<'input>,
    func: &mut Vec<TypedStatement>,
    arg: &mut ast::VariableDeclaration<Id>,
) {
    let new_id = id_def.register_intermediate(Some((arg.type_.clone(), arg.state_space)));
    func.push(Statement::Variable(Variable {
        align: arg.align,
        type_: arg.type_.clone(),
        state_space: ast::StateSpace::Reg,
        name: arg.name,
        initializer: None,
    }));
    func.push(Statement::StoreVar(StoreVarDetails {
        arg: ast::Arg2St {
            src1: arg.name,
            src2: new_id,
        },
        type_: arg.type_.clone(),
        member_index: None,
    }));
    arg.name = new_id;
}

fn insert_mem_ssa_argument_reg_return(
    func: &mut Vec<TypedStatement>,
    arg: &ast::VariableDeclaration<Id>,
) {
    func.push(Statement::Variable(Variable {
        align: arg.align,
        type_: arg.type_.clone(),
        state_space: arg.state_space,
        name: arg.name,
        initializer: None,
    }));
}

pub(crate) trait Visitable<From: ArgParamsEx, To: ArgParamsEx>: Sized {
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<From, To>,
    ) -> Result<Statement<ast::Instruction<To>, To>, TranslateError>;
}

struct VisitArgumentDescriptor<
    'a,
    Ctor: FnOnce(Id) -> Statement<ast::Instruction<U>, U>,
    U: ArgParamsEx,
> {
    desc: ArgumentDescriptor<Id>,
    typ: &'a ast::Type,
    state_space: ast::StateSpace,
    stmt_ctor: Ctor,
}

impl<
        'a,
        Ctor: FnOnce(Id) -> Statement<ast::Instruction<U>, U>,
        T: ArgParamsEx<Id = Id>,
        U: ArgParamsEx<Id = Id>,
    > Visitable<T, U> for VisitArgumentDescriptor<'a, Ctor, U>
{
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok((self.stmt_ctor)(
            visitor.id(self.desc, Some((self.typ, self.state_space)))?,
        ))
    }
}

struct InsertMemSSAVisitor<'a, 'input> {
    id_def: &'a mut IdNameMapBuilder<'input>,
    func: &'a mut Vec<TypedStatement>,
    post_statements: Vec<TypedStatement>,
}

impl<'a, 'input> InsertMemSSAVisitor<'a, 'input> {
    fn symbol(
        &mut self,
        desc: ArgumentDescriptor<(Id, Option<u8>)>,
        expected: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<Id, TranslateError> {
        let symbol = desc.op.0;
        if expected.is_none() {
            return Ok(symbol);
        };
        let (mut var_type, var_space, _, is_variable) = self.id_def.get_typed(symbol)?;
        if !var_space.is_compatible(ast::StateSpace::Reg) || !is_variable {
            return Ok(symbol);
        };
        let member_index = match desc.op.1 {
            Some(idx) => {
                let vector_width = match var_type {
                    ast::Type::Vector(scalar_t, width) => {
                        var_type = ast::Type::Scalar(scalar_t);
                        width
                    }
                    _ => return Err(TranslateError::mismatched_type()),
                };
                Some((idx, vector_width))
            }
            None => None,
        };
        let generated_id = self
            .id_def
            .register_intermediate(Some((var_type.clone(), ast::StateSpace::Reg)));
        if !desc.is_dst {
            self.func.push(Statement::LoadVar(LoadVarDetails {
                arg: Arg2 {
                    dst: generated_id,
                    src: symbol,
                },
                _state_space: ast::StateSpace::Reg,
                typ: var_type,
                member_index,
            }));
        } else {
            let (type_, member_index) = match member_index {
                None => (var_type, None),
                Some((idx, width)) => {
                    if let ast::Type::Scalar(scalar) = var_type {
                        (ast::Type::Vector(scalar, width), Some(idx))
                    } else {
                        return Err(TranslateError::unreachable());
                    }
                }
            };
            self.post_statements
                .push(Statement::StoreVar(StoreVarDetails {
                    arg: Arg2St {
                        src1: symbol,
                        src2: generated_id,
                    },
                    type_,
                    member_index,
                }));
        }
        Ok(generated_id)
    }
}

impl<'a, 'input> ArgumentMapVisitor<TypedArgParams, TypedArgParams>
    for InsertMemSSAVisitor<'a, 'input>
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<Id>,
        typ: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<Id, TranslateError> {
        self.symbol(desc.new_op((desc.op, None)), typ)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<TypedOperand>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<TypedOperand, TranslateError> {
        Ok(match desc.op {
            TypedOperand::Reg(reg) => {
                TypedOperand::Reg(self.symbol(desc.new_op((reg, None)), Some((typ, state_space)))?)
            }
            TypedOperand::RegOffset(reg, offset) => TypedOperand::RegOffset(
                self.symbol(desc.new_op((reg, None)), Some((typ, state_space)))?,
                offset,
            ),
            op @ TypedOperand::Imm(..) => op,
            TypedOperand::VecMember(symbol, index) => TypedOperand::Reg(
                self.symbol(desc.new_op((symbol, Some(index))), Some((typ, state_space)))?,
            ),
        })
    }
}

fn insert_mem_ssa_statement_default<'a, 'input, S: Visitable<TypedArgParams, TypedArgParams>>(
    id_def: &'a mut IdNameMapBuilder<'input>,
    func: &'a mut Vec<TypedStatement>,
    stmt: S,
) -> Result<(), TranslateError> {
    let mut visitor = InsertMemSSAVisitor {
        id_def,
        func,
        post_statements: Vec::new(),
    };
    let new_stmt = stmt.visit(&mut visitor)?;
    visitor.func.push(new_stmt);
    visitor.func.extend(visitor.post_statements);
    Ok(())
}

struct FlattenArguments<'a, 'input, I, P: ast::ArgParams> {
    func: &'a mut Vec<Statement<I, P>>,
    id_def: &'a mut IdNameMapBuilder<'input>,
    post_stmts: Vec<Statement<I, P>>,
}

impl<'a, 'input, I, P: ast::ArgParams> FlattenArguments<'a, 'input, I, P> {
    fn new(func: &'a mut Vec<Statement<I, P>>, id_def: &'a mut IdNameMapBuilder<'input>) -> Self {
        FlattenArguments {
            func,
            id_def,
            post_stmts: Vec::new(),
        }
    }

    fn reg(
        &mut self,
        desc: ArgumentDescriptor<Id>,
        _: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<Id, TranslateError> {
        Ok(desc.op)
    }

    fn immediate(
        &mut self,
        desc: ArgumentDescriptor<ast::ImmediateValue>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<Id, TranslateError> {
        let (id, statement) = Self::immediate_impl(self.id_def, desc.op, typ, state_space);
        self.func.push(statement);
        Ok(id)
    }

    fn immediate_impl(
        id_def: &mut IdNameMapBuilder<'input>,
        immediate: ast::ImmediateValue,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> (Id, Statement<I, P>) {
        let scalar_t = if let ast::Type::Scalar(scalar) = typ {
            *scalar
        } else {
            todo!()
        };
        let id = id_def.register_intermediate(Some((ast::Type::Scalar(scalar_t), state_space)));
        (
            id,
            Statement::Constant(ConstantDefinition {
                dst: id,
                typ: scalar_t,
                value: immediate,
            }),
        )
    }
}

impl<'a, 'b> FlattenArguments<'a, 'b, ast::Instruction<ExpandedArgParams>, ExpandedArgParams> {
    fn reg_offset(
        &mut self,
        desc: ArgumentDescriptor<(Id, i64)>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<Id, TranslateError> {
        let (reg, offset) = desc.op;
        if !desc.is_memory_access {
            let (reg_type, reg_space, ..) = self.id_def.get_typed(reg)?;
            if !reg_space.is_compatible(ast::StateSpace::Reg) {
                return Err(TranslateError::mismatched_type());
            }
            let reg_scalar_type = match reg_type {
                ast::Type::Scalar(underlying_type) => underlying_type,
                _ => return Err(TranslateError::mismatched_type()),
            };
            let id_constant_stmt = self
                .id_def
                .register_intermediate(Some((reg_type.clone(), ast::StateSpace::Reg)));
            self.func.push(Statement::Constant(ConstantDefinition {
                dst: id_constant_stmt,
                typ: reg_scalar_type,
                value: ast::ImmediateValue::S64(offset),
            }));
            let arith_details = match reg_scalar_type.kind() {
                ast::ScalarKind::Signed => ast::ArithDetails::Signed(ast::ArithSInt {
                    typ: reg_scalar_type,
                    saturate: false,
                }),
                ast::ScalarKind::Unsigned | ast::ScalarKind::Bit => {
                    ast::ArithDetails::Unsigned(reg_scalar_type)
                }
                _ => return Err(TranslateError::unreachable()),
            };
            let id_add_result = self
                .id_def
                .register_intermediate(Some((reg_type, state_space)));
            self.func.push(Statement::Instruction(ast::Instruction::Add(
                arith_details,
                ast::Arg3 {
                    dst: id_add_result,
                    src1: reg,
                    src2: id_constant_stmt,
                },
            )));
            Ok(id_add_result)
        } else {
            let id_constant_stmt = self.id_def.register_intermediate(Some((
                ast::Type::Scalar(ast::ScalarType::S64),
                ast::StateSpace::Reg,
            )));
            self.func.push(Statement::Constant(ConstantDefinition {
                dst: id_constant_stmt,
                typ: ast::ScalarType::S64,
                value: ast::ImmediateValue::S64(offset),
            }));
            let dst = self
                .id_def
                .register_intermediate(Some((typ.clone(), state_space)));
            self.func.push(Statement::PtrAccess(PtrAccess {
                underlying_type: typ.clone(),
                state_space: state_space,
                dst,
                ptr_src: reg,
                offset_src: id_constant_stmt,
            }));
            Ok(dst)
        }
    }
}

impl<'a, 'b> ArgumentMapVisitor<TypedArgParams, ExpandedArgParams>
    for FlattenArguments<'a, 'b, ast::Instruction<ExpandedArgParams>, ExpandedArgParams>
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<Id>,
        t: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<Id, TranslateError> {
        self.reg(desc, t)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<TypedOperand>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<Id, TranslateError> {
        match desc.op {
            TypedOperand::Reg(r) => self.reg(desc.new_op(r), Some((typ, state_space))),
            TypedOperand::Imm(x) => self.immediate(desc.new_op(x), typ, state_space),
            TypedOperand::RegOffset(reg, offset) => {
                self.reg_offset(desc.new_op((reg, offset)), typ, state_space)
            }
            TypedOperand::VecMember(..) => Err(TranslateError::unreachable()),
        }
    }
}

fn insert_implicit_conversions_impl(
    func: &mut Vec<ExpandedStatement>,
    id_def: &mut IdNameMapBuilder,
    stmt: impl Visitable<ExpandedArgParams, ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let mut post_conv = Vec::new();
    let statement =
        stmt.visit(&mut |desc: ArgumentDescriptor<Id>,
                         typ: Option<(&ast::Type, ast::StateSpace)>| {
            let (instr_type, instruction_space) = match typ {
                None => return Ok(desc.op),
                Some(t) => t,
            };
            let (operand_type, operand_space, ..) = id_def.get_typed(desc.op)?;
            let conversion_fn = desc
                .non_default_implicit_conversion
                .unwrap_or(default_implicit_conversion);
            match conversion_fn(
                (operand_space, &operand_type),
                (instruction_space, instr_type),
            )? {
                Some(conv_kind) => {
                    let conv_output = if desc.is_dst {
                        &mut post_conv
                    } else {
                        &mut *func
                    };
                    let mut from_type = instr_type.clone();
                    let mut from_space = instruction_space;
                    let mut to_type = operand_type;
                    let mut to_space = operand_space;
                    let mut src =
                        id_def.register_intermediate(Some((instr_type.clone(), instruction_space)));
                    let mut dst = desc.op;
                    let result = Ok(src);
                    if !desc.is_dst {
                        mem::swap(&mut src, &mut dst);
                        mem::swap(&mut from_type, &mut to_type);
                        mem::swap(&mut from_space, &mut to_space);
                    }
                    conv_output.push(Statement::Conversion(ImplicitConversion {
                        src,
                        dst,
                        from_type,
                        from_space,
                        to_type,
                        to_space,
                        kind: conv_kind,
                    }));
                    result
                }
                None => Ok(desc.op),
            }
        })?;
    func.push(statement);
    func.append(&mut post_conv);
    Ok(())
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum PtxSpecialRegister {
    Tid,
    Ntid,
    Ctaid,
    Nctaid,
    Clock,
    LanemaskLt,
    LanemaskLe,
    LanemaskGe,
    Laneid,
    Clock64,
}

impl PtxSpecialRegister {
    fn try_parse(s: &str) -> Option<Self> {
        match s {
            "%tid" => Some(Self::Tid),
            "%ntid" => Some(Self::Ntid),
            "%ctaid" => Some(Self::Ctaid),
            "%nctaid" => Some(Self::Nctaid),
            "%clock" => Some(Self::Clock),
            "%lanemask_lt" => Some(Self::LanemaskLt),
            "%lanemask_le" => Some(Self::LanemaskLe),
            "%lanemask_ge" => Some(Self::LanemaskGe),
            "%laneid" => Some(Self::Laneid),
            "%clock64" => Some(Self::Clock64),
            _ => None,
        }
    }

    fn get_type(self) -> ast::Type {
        match self {
            PtxSpecialRegister::Tid
            | PtxSpecialRegister::Ntid
            | PtxSpecialRegister::Ctaid
            | PtxSpecialRegister::Nctaid => ast::Type::Vector(self.get_function_return_type(), 4),
            _ => ast::Type::Scalar(self.get_function_return_type()),
        }
    }

    fn get_function_return_type(self) -> ast::ScalarType {
        match self {
            PtxSpecialRegister::Tid
            | PtxSpecialRegister::Ntid
            | PtxSpecialRegister::Ctaid
            | PtxSpecialRegister::Nctaid
            | PtxSpecialRegister::Clock
            | PtxSpecialRegister::LanemaskLt
            | PtxSpecialRegister::LanemaskLe
            | PtxSpecialRegister::LanemaskGe
            | PtxSpecialRegister::Laneid => ast::ScalarType::U32,
            PtxSpecialRegister::Clock64 => ast::ScalarType::U64,
        }
    }

    fn get_function_input_type(self) -> Option<ast::ScalarType> {
        match self {
            PtxSpecialRegister::Tid
            | PtxSpecialRegister::Ntid
            | PtxSpecialRegister::Ctaid
            | PtxSpecialRegister::Nctaid => Some(ast::ScalarType::U8),
            PtxSpecialRegister::Clock
            | PtxSpecialRegister::Clock64
            | PtxSpecialRegister::LanemaskLt
            | PtxSpecialRegister::LanemaskLe
            | PtxSpecialRegister::LanemaskGe
            | PtxSpecialRegister::Laneid => None,
        }
    }

    fn get_unprefixed_function_name(self) -> &'static str {
        match self {
            PtxSpecialRegister::Tid => "sreg_tid",
            PtxSpecialRegister::Ntid => "sreg_ntid",
            PtxSpecialRegister::Ctaid => "sreg_ctaid",
            PtxSpecialRegister::Nctaid => "sreg_nctaid",
            PtxSpecialRegister::Clock => "sreg_clock",
            PtxSpecialRegister::Clock64 => "sreg_clock64",
            PtxSpecialRegister::LanemaskLt => "sreg_lanemask_lt",
            PtxSpecialRegister::LanemaskLe => "sreg_lanemask_le",
            PtxSpecialRegister::LanemaskGe => "sreg_lanemask_ge",
            PtxSpecialRegister::Laneid => "sreg_laneid",
        }
    }
}

struct SpecialRegistersMap {
    reg_to_id: FxHashMap<PtxSpecialRegister, Id>,
    id_to_reg: FxHashMap<Id, PtxSpecialRegister>,
}

impl SpecialRegistersMap {
    fn new() -> Self {
        SpecialRegistersMap {
            reg_to_id: FxHashMap::default(),
            id_to_reg: FxHashMap::default(),
        }
    }

    fn get(&self, id: Id) -> Option<PtxSpecialRegister> {
        self.id_to_reg.get(&id).copied()
    }

    fn get_or_add(&mut self, id_gen: &mut IdGenerator, reg: PtxSpecialRegister) -> Id {
        match self.reg_to_id.entry(reg) {
            hash_map::Entry::Occupied(e) => *e.get(),
            hash_map::Entry::Vacant(e) => {
                let numeric_id = id_gen.next();
                e.insert(numeric_id);
                self.id_to_reg.insert(numeric_id, reg);
                numeric_id
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id(NonZeroU32);

impl Id {
    pub(crate) fn get(self) -> u32 {
        self.0.get()
    }
}

pub(crate) struct IdGenerator {
    pub(crate) next: NonZeroU32,
}

impl IdGenerator {
    pub(crate) fn new() -> Self {
        Self {
            next: unsafe { NonZeroU32::new_unchecked(1) },
        }
    }

    pub(crate) fn next(&mut self) -> Id {
        self.reserve(1).next().unwrap()
    }

    // Returns first reserved id
    pub(crate) fn reserve(&mut self, count: usize) -> impl ExactSizeIterator<Item = Id> + Clone {
        let value = self.next.get();
        unsafe {
            self.next = NonZeroU32::new_unchecked(value + count as u32);
            let start = Id(NonZeroU32::new_unchecked(value));
            let end = Id(self.next);
            (start.0.get()..end.0.get()).map(|i| Id(NonZeroU32::new_unchecked(i)))
        }
    }
}

pub(crate) struct IdNameMapBuilder<'input> {
    pub(crate) id_gen: IdGenerator,
    type_check: FxHashMap<Id, Option<(ast::Type, ast::StateSpace, Option<u32>, bool)>>,
    pub(crate) globals: GlobalsResolver<'input>,
}

impl<'input> IdNameMapBuilder<'input> {
    pub(crate) fn new(id_gen: IdGenerator) -> Self {
        let globals = GlobalsResolver {
            variables: FxHashMap::default(),
            reverse_variables: FxHashMap::default(),
            special_registers: SpecialRegistersMap::new(),
            function_prototypes: FxHashMap::default(),
        };
        Self {
            id_gen,
            globals,
            type_check: FxHashMap::default(),
        }
    }

    pub(crate) fn get_or_add_non_variable<T: Into<Cow<'input, str>>>(&mut self, id: T) -> Id {
        self.get_or_add_impl(id.into(), None)
    }

    fn get_or_add_impl(
        &mut self,
        name: Cow<'input, str>,
        type_: Option<(ast::Type, ast::StateSpace, Option<u32>)>,
    ) -> Id {
        let id = self.globals.get_or_add_impl(&mut self.id_gen, name.clone());
        if cfg!(debug_assertions) {
            eprintln!("{}: {}", id.get(), name.to_owned());
        }
        self.type_check
            .insert(id, type_.map(|(t, ss, align)| (t, ss, align, true)));
        id
    }

    pub(crate) fn register_intermediate(
        &mut self,
        typ: Option<(ast::Type, ast::StateSpace)>,
    ) -> Id {
        let new_id = self.id_gen.next();
        self.type_check
            .insert(new_id, typ.map(|(t, space)| (t, space, None, false)));
        new_id
    }

    // This is for identifiers which will be emitted later as OpVariable
    // They are candidates for insertion of LoadVar/StoreVar
    pub(crate) fn register_variable_decl(
        &mut self,
        align: Option<u32>,
        type_: ast::Type,
        state_space: ast::StateSpace,
    ) -> ast::VariableDeclaration<Id> {
        let name = self.id_gen.next();
        self.type_check
            .insert(name, Some((type_.clone(), state_space, align, true)));
        ast::VariableDeclaration {
            align,
            type_,
            state_space,
            name,
        }
    }
    pub(crate) fn register_variable_def(
        &mut self,
        align: Option<u32>,
        type_: ast::Type,
        state_space: ast::StateSpace,
        initializer: Option<ast::Initializer<Id>>,
    ) -> Variable {
        let name = self.id_gen.next();
        self.type_check
            .insert(name, Some((type_.clone(), state_space, align, true)));
        Variable {
            name,
            align,
            type_,
            state_space,
            initializer,
        }
    }

    pub(crate) fn get_typed(
        &self,
        id: Id,
    ) -> Result<(ast::Type, ast::StateSpace, Option<u32>, bool), TranslateError> {
        match self.type_check.get(&id) {
            Some(Some(x)) => Ok(x.clone()),
            Some(None) => Err(TranslateError::untyped_symbol()),
            None => match self.globals.special_registers.get(id) {
                Some(x) => Ok((x.get_type(), ast::StateSpace::Sreg, None, true)),
                None => match self.type_check.get(&id) {
                    Some(Some(result)) => Ok(result.clone()),
                    Some(None) | None => Err(TranslateError::untyped_symbol()),
                },
            },
        }
    }

    fn change_type(&mut self, id: Id, new_type: ast::Type) -> Result<(), TranslateError> {
        Ok(match self.type_check.get_mut(&id) {
            Some(Some((type_, ..))) => {
                *type_ = new_type;
            }
            _ => return Err(TranslateError::unreachable()),
        })
    }
}

pub(crate) struct GlobalsResolver<'input> {
    // Thos two fields below are only used by raytracing
    // TODO: move to raytracing-specific structures
    pub(crate) variables: FxHashMap<Cow<'input, str>, Id>,
    pub(crate) reverse_variables: FxHashMap<Id, Cow<'input, str>>,
    special_registers: SpecialRegistersMap,
    pub(crate) function_prototypes: FxHashMap<Id, Callprototype>,
}

impl<'input> GlobalsResolver<'input> {
    fn get_or_add_impl(&mut self, id_gen: &mut IdGenerator, id: Cow<'input, str>) -> Id {
        let id = match self.variables.entry(id) {
            hash_map::Entry::Occupied(e) => *(e.get()),
            hash_map::Entry::Vacant(e) => {
                let new_id = id_gen.next();
                self.reverse_variables.insert(new_id, e.key().clone());
                e.insert(new_id);
                new_id
            }
        };
        id
    }
}

pub struct Callprototype {
    pub return_arguments: Vec<(ast::Type, ast::StateSpace)>,
    pub input_arguments: Vec<(ast::Type, ast::StateSpace)>,
}

pub(crate) struct StringIdResolver<'a, 'input> {
    module: &'a mut IdNameMapBuilder<'input>,
    scopes: Vec<FxHashMap<Cow<'input, str>, Id>>,
}

impl<'a, 'input> StringIdResolver<'a, 'input> {
    fn new<P: ast::ArgParams<Id = Id>>(
        module_resolver: &'a mut IdNameMapBuilder<'input>,
        existing_directives: &[TranslationDirective<'input, P>],
    ) -> Result<Self, TranslateError> {
        let mut result = Self {
            module: module_resolver,
            scopes: vec![FxHashMap::default(), FxHashMap::default()],
        };
        for directive in existing_directives {
            match directive {
                TranslationDirective::Variable(..) => return Err(TranslateError::unreachable()),
                TranslationDirective::Method(method) => {
                    let string_name = result
                        .module
                        .globals
                        .reverse_variables
                        .get(&method.name)
                        .ok_or_else(TranslateError::unreachable)?;
                    result.scopes[StringIdResolverScope::IMPLICIT_GLOBALS]
                        .insert(string_name.clone(), method.name);
                }
            }
        }
        Ok(result)
    }

    fn start_module<'b>(&'b mut self) -> StringIdResolverScope<'a, 'b, 'input> {
        self.scopes.push(FxHashMap::default());
        StringIdResolverScope(self)
    }
}

pub(crate) struct StringIdResolverScope<'a, 'b, 'input>(&'b mut StringIdResolver<'a, 'input>);

impl<'a, 'b, 'input> StringIdResolverScope<'a, 'b, 'input> {
    // Items with visible, weak, etc. visibility. Accessible only by items
    // taking part in cross-module linking (so items with visible, weak, etc. visibility)
    const CROSS_MODULE: usize = 0;
    // Some items are not explicitly declared, but are anyway visible inside a module.
    // Currently this is the scope for raytracing function declarations
    // TOOD: refactor special registers (activemask, etc.) to use this scope
    const IMPLICIT_GLOBALS: usize = 1;
    const CURRENT_MODULE: usize = 2;

    fn start_scope<'x>(&'x mut self) -> StringIdResolverScope<'a, 'x, 'input> {
        self.0.scopes.push(FxHashMap::default());
        StringIdResolverScope(self.0)
    }

    fn get_id_in_module_scope(&self, name: &str) -> Result<Id, TranslateError> {
        self.0.scopes[Self::CURRENT_MODULE]
            .get(name)
            .copied()
            .ok_or_else(TranslateError::unknown_symbol)
    }

    fn get_id_in_function_scopes(&self, name: &str) -> Result<Id, TranslateError> {
        let func_scopes_count = self.0.scopes.len() - (Self::CURRENT_MODULE + 1);
        self.0
            .scopes
            .iter()
            .rev()
            .take(func_scopes_count)
            .find_map(|scope| scope.get(name))
            .copied()
            .ok_or_else(TranslateError::unknown_symbol)
    }

    fn get_id_in_module_scopes(&mut self, name: &str) -> Result<Id, TranslateError> {
        // Scope 0 is global scope
        let func_scopes_count = self.0.scopes.len() - (Self::CROSS_MODULE + 1);
        PtxSpecialRegister::try_parse(name)
            .map(|sreg| {
                self.0
                    .module
                    .globals
                    .special_registers
                    .get_or_add(&mut self.0.module.id_gen, sreg)
            })
            .or_else(|| {
                self.0
                    .scopes
                    .iter()
                    .rev()
                    .take(func_scopes_count)
                    .find_map(|scope| scope.get(name))
                    .copied()
            })
            .ok_or_else(TranslateError::unknown_symbol)
    }

    fn add_or_get_at_module_level(
        &mut self,
        name: &'input str,
        use_global_scope: bool,
    ) -> Result<Id, TranslateError> {
        debug_assert!(self.0.scopes.len() == 3);
        if self.0.scopes[Self::IMPLICIT_GLOBALS].get(name).is_some() {
            return Err(TranslateError::symbol_redefinition());
        }
        if use_global_scope {
            let id = Self::get_or_add_untyped_in_scope(
                &mut self.0.module,
                &mut self.0.scopes[Self::CROSS_MODULE],
                Cow::Borrowed(name),
                None,
            );
            match self.0.scopes[Self::CURRENT_MODULE].entry(Cow::Borrowed(name)) {
                hash_map::Entry::Occupied(existing_id) => {
                    if *existing_id.get() != id {
                        return Err(TranslateError::unreachable());
                    }
                }
                hash_map::Entry::Vacant(entry) => {
                    entry.insert(id);
                }
            }
            Ok(id)
        } else {
            Ok(Self::get_or_add_untyped_in_scope(
                &mut self.0.module,
                &mut self.0.scopes[Self::CURRENT_MODULE],
                Cow::Borrowed(name),
                None,
            ))
        }
    }

    fn get_or_add_untyped_in_scope(
        id_def: &mut IdNameMapBuilder<'input>,
        scope: &mut FxHashMap<Cow<'input, str>, Id>,
        name: Cow<'input, str>,
        type_: Option<(ast::Type, ast::StateSpace, Option<u32>, bool)>,
    ) -> Id {
        match scope.entry(name) {
            hash_map::Entry::Occupied(entry) => *entry.get(),
            hash_map::Entry::Vacant(entry) => {
                let id = id_def.id_gen.next();
                if cfg!(debug_assertions) {
                    eprintln!("{}: {}", id.get(), entry.key().to_owned());
                }
                id_def.type_check.insert(id, type_);
                entry.insert(id);
                id
            }
        }
    }

    fn add_untyped_checked(&mut self, name: &'input str) -> Result<Id, TranslateError> {
        let (id, overwrite) = self.add_untyped_impl(name);
        if overwrite {
            Err(TranslateError::SymbolRedefinition)
        } else {
            Ok(id)
        }
    }

    fn add_variable_checked(
        &mut self,
        name: &'input str,
        type_: ast::Type,
        space: ast::StateSpace,
        align: Option<u32>,
    ) -> Result<Id, TranslateError> {
        let id = self.0.module.id_gen.next();
        self.0
            .module
            .type_check
            .insert(id, Some((type_, space, align, true)));
        let old = self
            .0
            .scopes
            .last_mut()
            .unwrap()
            .insert(Cow::Borrowed(name), id);
        if old.is_some() {
            Err(TranslateError::SymbolRedefinition)
        } else {
            Ok(id)
        }
    }

    fn add_untyped_impl(&mut self, name: &'input str) -> (Id, bool) {
        let id = self.0.module.id_gen.next();
        self.0.module.type_check.insert(id, None);
        let old = self
            .0
            .scopes
            .last_mut()
            .unwrap()
            .insert(Cow::Borrowed(name), id);
        (id, old.is_some())
    }

    fn add_or_get_module_variable(
        &mut self,
        name: Cow<'input, str>,
        use_global_scope: bool,
        type_: ast::Type,
        state_space: ast::StateSpace,
        align: Option<u32>,
        initializer: Option<ast::Initializer<Id>>,
    ) -> Result<Variable, TranslateError> {
        let id = if use_global_scope {
            let id = Self::get_or_add_untyped_in_scope(
                &mut self.0.module,
                &mut self.0.scopes[Self::CROSS_MODULE],
                name.clone(),
                Some((type_.clone(), state_space, align, true)),
            );
            if self.0.scopes[Self::CURRENT_MODULE]
                .insert(name.clone(), id)
                .is_some()
            {
                return Err(TranslateError::unreachable());
            }
            id
        } else {
            Self::get_or_add_untyped_in_scope(
                &mut self.0.module,
                &mut self.0.scopes[Self::CURRENT_MODULE],
                name.clone(),
                Some((type_.clone(), state_space, align, true)),
            )
        };
        self.0.module.globals.variables.insert(name.clone(), id);
        self.0.module.globals.reverse_variables.insert(id, name);
        Ok(Variable {
            align,
            type_,
            state_space,
            name: id,
            initializer,
        })
    }

    fn register_variable(
        &mut self,
        name: Cow<'input, str>,
        type_: ast::Type,
        state_space: ast::StateSpace,
        align: Option<u32>,
        initializer: Option<ast::Initializer<Id>>,
    ) -> Result<Variable, TranslateError> {
        let id = self.0.module.id_gen.next();
        self.0
            .module
            .type_check
            .insert(id, Some((type_.clone(), state_space, align, true)));
        let old = self.0.scopes.last_mut().unwrap().insert(name, id);
        if old.is_some() {
            Err(TranslateError::SymbolRedefinition)
        } else {
            Ok(Variable {
                align,
                type_: type_,
                state_space,
                name: id,
                initializer,
            })
        }
    }

    fn new_untyped(&mut self) -> Id {
        let id = self.0.module.id_gen.next();
        self.0.module.type_check.insert(id, None);
        id
    }
}

impl<'a, 'b, 'input> Drop for StringIdResolverScope<'a, 'b, 'input> {
    fn drop(&mut self) {
        self.0.scopes.pop();
    }
}

pub(crate) struct FunctionPointerDetails {
    pub(crate) dst: Id,
    pub(crate) src: Id,
}

impl<T: ArgParamsEx<Id = Id>, U: ArgParamsEx<Id = Id>> Visitable<T, U> for FunctionPointerDetails {
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::FunctionPointer(FunctionPointerDetails {
            dst: visitor.id(
                ArgumentDescriptor {
                    op: self.dst,
                    is_dst: true,
                    is_memory_access: false,
                    non_default_implicit_conversion: None,
                },
                Some((
                    &ast::Type::Scalar(ast::ScalarType::U64),
                    ast::StateSpace::Reg,
                )),
            )?,
            src: visitor.id(
                ArgumentDescriptor {
                    op: self.src,
                    is_dst: false,
                    is_memory_access: false,
                    non_default_implicit_conversion: None,
                },
                None,
            )?,
        }))
    }
}

pub(crate) struct MadCDetails<P: ast::ArgParams> {
    pub(crate) type_: ast::ScalarType,
    pub(crate) is_hi: bool,
    pub(crate) arg: Arg4CarryIn<P>,
}

impl<T: ArgParamsEx<Id = Id>, U: ArgParamsEx<Id = Id>> Visitable<T, U> for MadCDetails<T> {
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::MadC(MadCDetails {
            type_: self.type_,
            is_hi: self.is_hi,
            arg: self.arg.map(visitor, self.type_)?,
        }))
    }
}

pub(crate) struct MadCCDetails<P: ast::ArgParams> {
    pub(crate) type_: ast::ScalarType,
    pub(crate) is_hi: bool,
    pub(crate) arg: Arg4CarryOut<P>,
}

impl<T: ArgParamsEx<Id = Id>, U: ArgParamsEx<Id = Id>> Visitable<T, U> for MadCCDetails<T> {
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::MadCC(MadCCDetails {
            type_: self.type_,
            is_hi: self.is_hi,
            arg: self.arg.map(visitor, self.type_)?,
        }))
    }
}

pub(crate) enum Statement<I, P: ast::ArgParams> {
    Label(Id),
    Variable(Variable),
    Instruction(I),
    // SPIR-V compatible replacement for PTX predicates
    Conditional(BrachCondition),
    Call(ResolvedCall<P>),
    LoadVar(LoadVarDetails),
    StoreVar(StoreVarDetails),
    Conversion(ImplicitConversion),
    Constant(ConstantDefinition),
    RetValue(ast::RetData, Vec<(Id, ast::Type)>),
    PtrAccess(PtrAccess<P>),
    RepackVector(RepackVectorDetails),
    FunctionPointer(FunctionPointerDetails),
    MadC(MadCDetails<P>),
    MadCC(MadCCDetails<P>),
    AddC(ast::ScalarType, Arg3CarryIn<P>),
    AddCC(ast::ScalarType, Arg3CarryOut<P>),
    SubC(ast::ScalarType, Arg3CarryIn<P>),
    SubCC(ast::ScalarType, Arg3CarryOut<P>),
    AsmVolatile {
        asm: &'static str,
        constraints: &'static str,
    },
}

impl ExpandedStatement {
    pub(crate) fn map_id(self, f: &mut impl FnMut(Id, bool) -> Id) -> ExpandedStatement {
        match self {
            Statement::Label(id) => Statement::Label(f(id, false)),
            Statement::Variable(mut var) => {
                var.name = f(var.name, true);
                Statement::Variable(var)
            }
            Statement::Instruction(inst) => inst
                .visit(&mut |arg: ArgumentDescriptor<_>,
                             _: Option<(&ast::Type, ast::StateSpace)>| {
                    Ok(f(arg.op, arg.is_dst))
                })
                .unwrap(),
            Statement::LoadVar(mut details) => {
                details.arg.dst = f(details.arg.dst, true);
                details.arg.src = f(details.arg.src, false);
                Statement::LoadVar(details)
            }
            Statement::StoreVar(mut details) => {
                details.arg.src1 = f(details.arg.src1, false);
                details.arg.src2 = f(details.arg.src2, false);
                Statement::StoreVar(details)
            }
            Statement::Call(mut call) => {
                for (id, _, space) in call.return_arguments.iter_mut() {
                    let is_dst = match space {
                        ast::StateSpace::Reg => true,
                        ast::StateSpace::Param => false,
                        ast::StateSpace::Shared => false,
                        _ => todo!(),
                    };
                    *id = f(*id, is_dst);
                }
                call.name = f(call.name, false);
                for (id, _, _) in call.input_arguments.iter_mut() {
                    *id = f(*id, false);
                }
                Statement::Call(call)
            }
            Statement::Conditional(mut conditional) => {
                conditional.predicate = f(conditional.predicate, false);
                conditional.if_true = f(conditional.if_true, false);
                conditional.if_false = f(conditional.if_false, false);
                Statement::Conditional(conditional)
            }
            Statement::Conversion(mut conv) => {
                conv.dst = f(conv.dst, true);
                conv.src = f(conv.src, false);
                Statement::Conversion(conv)
            }
            Statement::Constant(mut constant) => {
                constant.dst = f(constant.dst, true);
                Statement::Constant(constant)
            }
            Statement::RetValue(data, ids) => {
                let ids = ids
                    .into_iter()
                    .map(|(id, type_)| (f(id, false), type_))
                    .collect();
                Statement::RetValue(data, ids)
            }
            Statement::PtrAccess(PtrAccess {
                underlying_type,
                state_space,
                dst,
                ptr_src,
                offset_src: constant_src,
            }) => {
                let dst = f(dst, true);
                let ptr_src = f(ptr_src, false);
                let constant_src = f(constant_src, false);
                Statement::PtrAccess(PtrAccess {
                    underlying_type,
                    state_space,
                    dst,
                    ptr_src,
                    offset_src: constant_src,
                })
            }
            Statement::RepackVector(repack) => {
                let packed = f(repack.packed, !repack.is_extract);
                let unpacked = repack
                    .unpacked
                    .iter()
                    .map(|id| f(*id, repack.is_extract))
                    .collect();
                Statement::RepackVector(RepackVectorDetails {
                    packed,
                    unpacked,
                    ..repack
                })
            }
            Statement::FunctionPointer(FunctionPointerDetails { dst, src }) => {
                Statement::FunctionPointer(FunctionPointerDetails {
                    dst: f(dst, true),
                    src: f(src, false),
                })
            }
            Statement::MadC(madc) => madc
                .visit(&mut |arg: ArgumentDescriptor<_>,
                             _: Option<(&ast::Type, ast::StateSpace)>| {
                    Ok(f(arg.op, arg.is_dst))
                })
                .unwrap(),
            Statement::MadCC(madcc) => madcc
                .visit(&mut |arg: ArgumentDescriptor<_>,
                             _: Option<(&ast::Type, ast::StateSpace)>| {
                    Ok(f(arg.op, arg.is_dst))
                })
                .unwrap(),
            Statement::AddC(details, arg) => VisitAddC(details, arg)
                .visit(&mut |arg: ArgumentDescriptor<_>,
                             _: Option<(&ast::Type, ast::StateSpace)>| {
                    Ok(f(arg.op, arg.is_dst))
                })
                .unwrap(),
            Statement::AddCC(details, arg) => VisitAddCC(details, arg)
                .visit(&mut |arg: ArgumentDescriptor<_>,
                             _: Option<(&ast::Type, ast::StateSpace)>| {
                    Ok(f(arg.op, arg.is_dst))
                })
                .unwrap(),
            Statement::SubC(details, arg) => VisitSubC(details, arg)
                .visit(&mut |arg: ArgumentDescriptor<_>,
                             _: Option<(&ast::Type, ast::StateSpace)>| {
                    Ok(f(arg.op, arg.is_dst))
                })
                .unwrap(),
            Statement::SubCC(details, arg) => VisitSubCC(details, arg)
                .visit(&mut |arg: ArgumentDescriptor<_>,
                             _: Option<(&ast::Type, ast::StateSpace)>| {
                    Ok(f(arg.op, arg.is_dst))
                })
                .unwrap(),
            Statement::AsmVolatile { asm, constraints } => {
                Statement::AsmVolatile { asm, constraints }
            }
        }
    }
}

pub(crate) struct LoadVarDetails {
    pub(crate) arg: ast::Arg2<ExpandedArgParams>,
    pub(crate) typ: ast::Type,
    pub(crate) _state_space: ast::StateSpace,
    // (index, vector_width)
    pub(crate) member_index: Option<(u8, u8)>,
}

pub(crate) struct StoreVarDetails {
    pub(crate) arg: ast::Arg2St<ExpandedArgParams>,
    pub(crate) type_: ast::Type,
    pub(crate) member_index: Option<u8>,
}

pub(crate) struct RepackVectorDetails {
    pub(crate) is_extract: bool,
    pub(crate) typ: ast::ScalarType,
    pub(crate) packed: Id,
    pub(crate) unpacked: Vec<Id>,
    pub(crate) non_default_implicit_conversion: Option<
        fn(
            (ast::StateSpace, &ast::Type),
            (ast::StateSpace, &ast::Type),
        ) -> Result<Option<ConversionKind>, TranslateError>,
    >,
}

impl RepackVectorDetails {
    fn map<
        From: ArgParamsEx<Id = Id>,
        To: ArgParamsEx<Id = Id>,
        V: ArgumentMapVisitor<From, To>,
    >(
        self,
        visitor: &mut V,
    ) -> Result<RepackVectorDetails, TranslateError> {
        let scalar = visitor.id(
            ArgumentDescriptor {
                op: self.packed,
                is_dst: !self.is_extract,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((
                &ast::Type::Vector(self.typ, self.unpacked.len() as u8),
                ast::StateSpace::Reg,
            )),
        )?;
        let scalar_type = self.typ;
        let is_extract = self.is_extract;
        let non_default_implicit_conversion = self.non_default_implicit_conversion;
        let vector = self
            .unpacked
            .into_iter()
            .map(|id| {
                visitor.id(
                    ArgumentDescriptor {
                        op: id,
                        is_dst: is_extract,
                        is_memory_access: false,
                        non_default_implicit_conversion,
                    },
                    Some((&ast::Type::Scalar(scalar_type), ast::StateSpace::Reg)),
                )
            })
            .collect::<Result<_, _>>()?;
        Ok(RepackVectorDetails {
            is_extract,
            typ: self.typ,
            packed: scalar,
            unpacked: vector,
            non_default_implicit_conversion,
        })
    }
}

impl<T: ArgParamsEx<Id = Id>, U: ArgParamsEx<Id = Id>> Visitable<T, U> for RepackVectorDetails {
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::RepackVector(self.map::<_, _, _>(visitor)?))
    }
}

pub(crate) struct ResolvedCall<P: ast::ArgParams> {
    pub uniform: bool,
    pub return_arguments: Vec<(P::Id, ast::Type, ast::StateSpace)>,
    pub name: P::Id,
    pub input_arguments: Vec<(P::Operand, ast::Type, ast::StateSpace)>,
    pub is_indirect: bool,
}

impl<T: ast::ArgParams> ResolvedCall<T> {
    fn from_declaration<'input>(
        call: ast::CallInst<T>,
        return_arguments: &[ast::VariableDeclaration<Id>],
        input_arguments: &[ast::VariableDeclaration<Id>],
    ) -> Result<Self, TranslateError> {
        if call.ret_params.len() != return_arguments.len()
            || call.param_list.len() != input_arguments.len()
        {
            return Err(TranslateError::mismatched_type());
        }
        let return_arguments = call
            .ret_params
            .into_iter()
            .zip(return_arguments.iter())
            .map(|(id, var_decl)| (id, var_decl.type_.clone(), var_decl.state_space))
            .collect::<Vec<_>>();
        let input_arguments = call
            .param_list
            .into_iter()
            .zip(input_arguments.iter())
            .map(|(id, var_decl)| (id, var_decl.type_.clone(), var_decl.state_space))
            .collect::<Vec<_>>();
        Ok(Self {
            return_arguments,
            input_arguments,
            uniform: call.uniform,
            name: call.func,
            is_indirect: false,
        })
    }

    fn from_callprototype<'input>(
        call: ast::CallInst<T>,
        proto: &Callprototype,
    ) -> Result<Self, TranslateError> {
        if call.ret_params.len() != proto.return_arguments.len()
            || call.param_list.len() != proto.input_arguments.len()
        {
            return Err(TranslateError::mismatched_type());
        }
        let return_arguments = call
            .ret_params
            .into_iter()
            .zip(proto.return_arguments.iter())
            .map(|(id, (type_, state_space))| (id, type_.clone(), *state_space))
            .collect::<Vec<_>>();
        let input_arguments = call
            .param_list
            .into_iter()
            .zip(proto.input_arguments.iter())
            .map(|(id, (type_, state_space))| (id, type_.clone(), *state_space))
            .collect::<Vec<_>>();
        Ok(Self {
            return_arguments,
            input_arguments,
            uniform: call.uniform,
            name: call.func,
            is_indirect: true,
        })
    }

    fn cast<U: ast::ArgParams<Id = T::Id, Operand = T::Operand>>(self) -> ResolvedCall<U> {
        ResolvedCall {
            uniform: self.uniform,
            return_arguments: self.return_arguments,
            name: self.name,
            input_arguments: self.input_arguments,
            is_indirect: self.is_indirect,
        }
    }
}

impl<From: ArgParamsEx<Id = Id>> ResolvedCall<From> {
    fn map<To: ArgParamsEx<Id = Id>, V: ArgumentMapVisitor<From, To>>(
        self,
        visitor: &mut V,
    ) -> Result<ResolvedCall<To>, TranslateError> {
        let return_arguments = self
            .return_arguments
            .into_iter()
            .map::<Result<_, TranslateError>, _>(|(id, typ, space)| {
                let new_id = visitor.id(
                    ArgumentDescriptor {
                        op: id,
                        is_dst: space != ast::StateSpace::Param,
                        is_memory_access: false,
                        non_default_implicit_conversion: None,
                    },
                    Some((&typ, space)),
                )?;
                Ok((new_id, typ, space))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let func = if self.is_indirect {
            visitor.id(
                ArgumentDescriptor {
                    op: self.name,
                    is_dst: false,
                    is_memory_access: false,
                    non_default_implicit_conversion: None,
                },
                Some((
                    &ast::Type::Scalar(ast::ScalarType::B64),
                    ast::StateSpace::Reg,
                )),
            )
        } else {
            visitor.id(
                ArgumentDescriptor {
                    op: self.name,
                    is_dst: false,
                    is_memory_access: false,
                    non_default_implicit_conversion: None,
                },
                None,
            )
        }?;
        let input_arguments = self
            .input_arguments
            .into_iter()
            .map::<Result<_, TranslateError>, _>(|(id, typ, space)| {
                let new_id = visitor.operand(
                    ArgumentDescriptor {
                        op: id,
                        is_dst: false,
                        is_memory_access: false,
                        non_default_implicit_conversion: None,
                    },
                    &typ,
                    space,
                )?;
                Ok((new_id, typ, space))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ResolvedCall {
            uniform: self.uniform,
            return_arguments,
            name: func,
            input_arguments,
            is_indirect: self.is_indirect,
        })
    }
}

impl<T: ArgParamsEx<Id = Id>, U: ArgParamsEx<Id = Id>> Visitable<T, U> for ResolvedCall<T> {
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::Call(self.map(visitor)?))
    }
}

impl<P: ArgParamsEx<Id = Id>> PtrAccess<P> {
    fn map<To: ArgParamsEx<Id = Id>, V: ArgumentMapVisitor<P, To>>(
        self,
        visitor: &mut V,
    ) -> Result<PtrAccess<To>, TranslateError> {
        let new_dst = visitor.id(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((&self.underlying_type, self.state_space)),
        )?;
        let new_ptr_src = visitor.id(
            ArgumentDescriptor {
                op: self.ptr_src,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((&self.underlying_type, self.state_space)),
        )?;
        let new_constant_src = visitor.operand(
            ArgumentDescriptor {
                op: self.offset_src,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::S64),
            ast::StateSpace::Reg,
        )?;
        Ok(PtrAccess {
            underlying_type: self.underlying_type,
            state_space: self.state_space,
            dst: new_dst,
            ptr_src: new_ptr_src,
            offset_src: new_constant_src,
        })
    }
}

impl<T: ArgParamsEx<Id = Id>, U: ArgParamsEx<Id = Id>> Visitable<T, U> for PtrAccess<T> {
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::PtrAccess(self.map(visitor)?))
    }
}

pub trait ArgParamsEx: ast::ArgParams + Sized {}

impl<'input> ArgParamsEx for ast::ParsedArgParams<'input> {}

pub(crate) enum NormalizedArgParams {}

impl ast::ArgParams for NormalizedArgParams {
    type Id = Id;
    type Operand = ast::Operand<Id>;
}

impl ArgParamsEx for NormalizedArgParams {}

type NormalizedStatement = Statement<
    (
        Option<ast::PredAt<Id>>,
        ast::Instruction<NormalizedArgParams>,
    ),
    NormalizedArgParams,
>;

type UnconditionalStatement = Statement<ast::Instruction<NormalizedArgParams>, NormalizedArgParams>;

pub(crate) enum TypedArgParams {}

impl ast::ArgParams for TypedArgParams {
    type Id = Id;
    type Operand = TypedOperand;
}

impl ArgParamsEx for TypedArgParams {}

#[derive(Copy, Clone)]
pub(crate) enum TypedOperand {
    Reg(Id),
    RegOffset(Id, i64),
    Imm(ast::ImmediateValue),
    VecMember(Id, u8),
}

type TypedStatement = Statement<ast::Instruction<TypedArgParams>, TypedArgParams>;

pub(crate) enum ExpandedArgParams {}
pub(crate) type ExpandedStatement =
    Statement<ast::Instruction<ExpandedArgParams>, ExpandedArgParams>;

impl ast::ArgParams for ExpandedArgParams {
    type Id = Id;
    type Operand = Id;
}

impl ArgParamsEx for ExpandedArgParams {}

pub(crate) type Directive<'input> = TranslationDirective<'input, ExpandedArgParams>;

pub(crate) enum TranslationDirective<'input, P: ast::ArgParams> {
    Variable(ast::LinkingDirective, Option<Cow<'input, str>>, Variable),
    Method(TranslationMethod<'input, P>),
}

pub(crate) struct Variable {
    pub align: Option<u32>,
    pub type_: ast::Type,
    pub state_space: ast::StateSpace,
    pub name: Id,
    pub initializer: Option<ast::Initializer<Id>>,
}

pub(crate) type Function<'input> = TranslationMethod<'input, ExpandedArgParams>;

pub(crate) struct TranslationMethod<'input, P: ast::ArgParams> {
    pub(crate) return_arguments: Vec<ast::VariableDeclaration<P::Id>>,
    pub(crate) name: P::Id,
    pub(crate) input_arguments: Vec<ast::VariableDeclaration<P::Id>>,
    pub(crate) body: Option<Vec<Statement<ast::Instruction<P>, P>>>,
    pub(crate) tuning: Vec<ast::TuningDirective>,
    pub(crate) is_kernel: bool,
    pub(crate) source_name: Option<Cow<'input, str>>,
    pub(crate) special_raytracing_linking: bool,
}

pub(crate) trait ArgumentMapVisitor<T: ArgParamsEx, U: ArgParamsEx> {
    fn id(
        &mut self,
        desc: ArgumentDescriptor<T::Id>,
        typ: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<U::Id, TranslateError>;
    fn operand(
        &mut self,
        desc: ArgumentDescriptor<T::Operand>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<U::Operand, TranslateError>;
}

impl<T> ArgumentMapVisitor<ExpandedArgParams, ExpandedArgParams> for T
where
    T: FnMut(
        ArgumentDescriptor<Id>,
        Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<Id, TranslateError>,
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<Id>,
        t: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<Id, TranslateError> {
        self(desc, t)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<Id>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<Id, TranslateError> {
        self(desc, Some((typ, state_space)))
    }
}

impl<'a, T> ArgumentMapVisitor<ast::ParsedArgParams<'a>, NormalizedArgParams> for T
where
    T: FnMut(&str) -> Result<Id, TranslateError>,
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<&str>,
        _: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<Id, TranslateError> {
        self(desc.op)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<ast::Operand<&str>>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<ast::Operand<Id>, TranslateError> {
        Ok(match desc.op {
            ast::Operand::Reg(id) => ast::Operand::Reg(self(id)?),
            ast::Operand::RegOffset(id, imm) => ast::Operand::RegOffset(self(id)?, imm),
            ast::Operand::Imm(imm) => ast::Operand::Imm(imm),
            ast::Operand::VecMember(id, member) => ast::Operand::VecMember(self(id)?, member),
            ast::Operand::VecPack(ref ids) => ast::Operand::VecPack(
                ids.into_iter()
                    .map(|id_or_immediate| {
                        Ok::<_, TranslateError>(match id_or_immediate {
                            ast::RegOrImmediate::Reg(reg) => ast::RegOrImmediate::Reg(
                                self.id(desc.new_op(reg), Some((typ, state_space)))?,
                            ),
                            ast::RegOrImmediate::Imm(imm) => ast::RegOrImmediate::Imm(*imm),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            ),
        })
    }
}

pub(crate) struct ArgumentDescriptor<Op> {
    pub(crate) op: Op,
    pub(crate) is_dst: bool,
    pub(crate) is_memory_access: bool,
    pub(crate) non_default_implicit_conversion: Option<
        fn(
            (ast::StateSpace, &ast::Type),
            (ast::StateSpace, &ast::Type),
        ) -> Result<Option<ConversionKind>, TranslateError>,
    >,
}
pub(crate) struct PtrAccess<P: ast::ArgParams> {
    pub(crate) underlying_type: ast::Type,
    pub(crate) state_space: ast::StateSpace,
    pub(crate) dst: Id,
    pub(crate) ptr_src: Id,
    pub(crate) offset_src: P::Operand,
}

impl<T> ArgumentDescriptor<T> {
    fn new_op<U>(&self, u: U) -> ArgumentDescriptor<U> {
        ArgumentDescriptor {
            op: u,
            is_dst: self.is_dst,
            is_memory_access: self.is_memory_access,
            non_default_implicit_conversion: self.non_default_implicit_conversion,
        }
    }
}

impl<T: ArgParamsEx> ast::Instruction<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
    ) -> Result<ast::Instruction<U>, TranslateError> {
        Ok(match self {
            ast::Instruction::Abs(d, arg) => {
                ast::Instruction::Abs(d, arg.map(visitor, &ast::Type::Scalar(d.typ))?)
            }
            // Call instruction is converted to a call statement early on
            ast::Instruction::Call(_) => return Err(TranslateError::unreachable()),
            ast::Instruction::Ld(d, a) => {
                let new_args = a.map(visitor, &d)?;
                ast::Instruction::Ld(d, new_args)
            }
            ast::Instruction::Mov(d, a) => {
                let mapped = a.map(visitor, &d)?;
                ast::Instruction::Mov(d, mapped)
            }
            ast::Instruction::Mul(d, a) => {
                let inst_type = d.get_type();
                let is_wide = d.is_wide();
                ast::Instruction::Mul(d, a.map_generic(visitor, &inst_type, is_wide)?)
            }
            ast::Instruction::Add(d, a) => {
                let inst_type = ast::Type::Scalar(d.get_type());
                ast::Instruction::Add(d, a.map_generic(visitor, &inst_type, false)?)
            }
            ast::Instruction::Setp(d, a) => {
                let inst_type = d.typ;
                ast::Instruction::Setp(d, a.map(visitor, &ast::Type::Scalar(inst_type))?)
            }
            ast::Instruction::SetpBool(d, a) => {
                let inst_type = d.base.typ;
                ast::Instruction::SetpBool(d, a.map(visitor, &ast::Type::Scalar(inst_type))?)
            }
            ast::Instruction::Not(t, a) => {
                ast::Instruction::Not(t, a.map(visitor, &ast::Type::Scalar(t))?)
            }
            ast::Instruction::Cvt(d, a) => {
                let (dst_t, src_t, int_to_int) = match &d {
                    ast::CvtDetails::FloatFromFloat(desc) => (desc.dst, desc.src, false),
                    ast::CvtDetails::FloatFromInt(desc) => (desc.dst, desc.src, false),
                    ast::CvtDetails::IntFromFloat(desc) => (desc.dst, desc.src, false),
                    ast::CvtDetails::IntFromInt(desc) => (desc.dst, desc.src, true),
                };
                ast::Instruction::Cvt(d, a.map_cvt(visitor, dst_t, src_t, int_to_int)?)
            }
            ast::Instruction::Shl(t, a) => {
                ast::Instruction::Shl(t, a.map_shift(visitor, &ast::Type::Scalar(t))?)
            }
            ast::Instruction::Shr(t, a) => {
                ast::Instruction::Shr(t, a.map_shift(visitor, &ast::Type::Scalar(t.into()))?)
            }
            ast::Instruction::St(d, a) => {
                let new_args = a.map(visitor, &d.typ, d.state_space)?;
                ast::Instruction::St(d, new_args)
            }
            ast::Instruction::Bra(d, a) => ast::Instruction::Bra(d, a.map(visitor, false, None)?),
            ast::Instruction::Exit => ast::Instruction::Exit,
            ast::Instruction::Ret(d) => ast::Instruction::Ret(d),
            ast::Instruction::Cvta(d, a) => {
                let inst_type = ast::Type::Scalar(ast::ScalarType::B64);
                ast::Instruction::Cvta(d, a.map(visitor, &inst_type)?)
            }
            ast::Instruction::Mad(d, a) => {
                let inst_type = d.get_type();
                let is_wide = d.is_wide();
                ast::Instruction::Mad(d, a.map(visitor, &inst_type, is_wide)?)
            }
            ast::Instruction::Fma(d, a) => {
                let inst_type = ast::Type::Scalar(d.typ);
                ast::Instruction::Fma(d, a.map(visitor, &inst_type, false)?)
            }
            ast::Instruction::Or(t, a) => ast::Instruction::Or(
                t,
                a.map_generic(visitor, &ast::Type::Scalar(t.into()), false)?,
            ),
            ast::Instruction::Sub(d, a) => {
                let typ = ast::Type::Scalar(d.get_type());
                ast::Instruction::Sub(d, a.map_generic(visitor, &typ, false)?)
            }
            ast::Instruction::Min(d, a) => {
                let typ = ast::Type::Scalar(d.get_type());
                ast::Instruction::Min(d, a.map_generic(visitor, &typ, false)?)
            }
            ast::Instruction::Max(d, a) => {
                let typ = ast::Type::Scalar(d.get_type());
                ast::Instruction::Max(d, a.map_generic(visitor, &typ, false)?)
            }
            ast::Instruction::Rcp(d, a) => {
                let typ = ast::Type::Scalar(d.type_);
                ast::Instruction::Rcp(d, a.map(visitor, &typ)?)
            }
            ast::Instruction::And(t, a) => ast::Instruction::And(
                t,
                a.map_generic(visitor, &ast::Type::Scalar(t.into()), false)?,
            ),
            ast::Instruction::Selp(t, a) => ast::Instruction::Selp(t, a.map_selp(visitor, t)?),
            ast::Instruction::Bar(d, a) => ast::Instruction::Bar(d, a.map(visitor)?),
            ast::Instruction::BarWarp(d, a) => ast::Instruction::BarWarp(d, a.map(visitor)?),
            ast::Instruction::Atom(d, a) => {
                ast::Instruction::Atom(d, a.map_atom(visitor, d.inner.get_type(), d.space)?)
            }
            ast::Instruction::AtomCas(d, a) => {
                ast::Instruction::AtomCas(d, a.map_atom(visitor, d.typ, d.space)?)
            }
            ast::Instruction::Div(d, a) => {
                ast::Instruction::Div(d, a.map_generic(visitor, &d.get_type(), false)?)
            }
            ast::Instruction::Sqrt(d, a) => {
                let type_ = ast::Type::Scalar(d.type_);
                ast::Instruction::Sqrt(d, a.map(visitor, &type_)?)
            }
            ast::Instruction::Rsqrt(d, a) => {
                ast::Instruction::Rsqrt(d, a.map(visitor, &ast::Type::Scalar(d.typ.into()))?)
            }
            ast::Instruction::Neg(d, a) => {
                ast::Instruction::Neg(d, a.map(visitor, &ast::Type::Scalar(d.typ))?)
            }
            ast::Instruction::Sin { flush_to_zero, arg } => {
                let typ = ast::Type::Scalar(ast::ScalarType::F32);
                ast::Instruction::Sin {
                    flush_to_zero,
                    arg: arg.map(visitor, &typ)?,
                }
            }
            ast::Instruction::Cos { flush_to_zero, arg } => {
                let typ = ast::Type::Scalar(ast::ScalarType::F32);
                ast::Instruction::Cos {
                    flush_to_zero,
                    arg: arg.map(visitor, &typ)?,
                }
            }
            ast::Instruction::Lg2 { flush_to_zero, arg } => {
                let typ = ast::Type::Scalar(ast::ScalarType::F32);
                ast::Instruction::Lg2 {
                    flush_to_zero,
                    arg: arg.map(visitor, &typ)?,
                }
            }
            ast::Instruction::Ex2 { flush_to_zero, arg } => {
                let typ = ast::Type::Scalar(ast::ScalarType::F32);
                ast::Instruction::Ex2 {
                    flush_to_zero,
                    arg: arg.map(visitor, &typ)?,
                }
            }
            ast::Instruction::Clz { typ, arg } => {
                let dst_type = ast::Type::Scalar(ast::ScalarType::B32);
                let src_type = ast::Type::Scalar(typ.into());
                ast::Instruction::Clz {
                    typ,
                    arg: arg.map_different_types(visitor, &dst_type, &src_type)?,
                }
            }
            ast::Instruction::Bfind(details, arg) => {
                let dst_type = ast::Type::Scalar(ast::ScalarType::B32);
                let src_type = ast::Type::Scalar(details.type_);
                ast::Instruction::Bfind(
                    details,
                    arg.map_different_types(visitor, &dst_type, &src_type)?,
                )
            }
            ast::Instruction::Brev { typ, arg } => {
                let full_type = ast::Type::Scalar(typ.into());
                ast::Instruction::Brev {
                    typ,
                    arg: arg.map(visitor, &full_type)?,
                }
            }
            ast::Instruction::Popc { typ, arg } => {
                let dst_type = ast::Type::Scalar(ast::ScalarType::B32);
                let src_type = ast::Type::Scalar(typ.into());
                ast::Instruction::Popc {
                    typ,
                    arg: arg.map_different_types(visitor, &dst_type, &src_type)?,
                }
            }
            ast::Instruction::Xor { typ, arg } => {
                let full_type = ast::Type::Scalar(typ.into());
                ast::Instruction::Xor {
                    typ,
                    arg: arg.map_generic(visitor, &full_type, false)?,
                }
            }
            ast::Instruction::Bfe { typ, arg } => {
                let full_type = ast::Type::Scalar(typ.into());
                ast::Instruction::Bfe {
                    typ,
                    arg: arg.map_bfe(visitor, &full_type)?,
                }
            }
            ast::Instruction::Bfi { typ, arg } => {
                let full_type = ast::Type::Scalar(typ.into());
                ast::Instruction::Bfi {
                    typ,
                    arg: arg.map_bfi(visitor, &full_type)?,
                }
            }
            ast::Instruction::Rem { typ, arg } => {
                let full_type = ast::Type::Scalar(typ.into());
                ast::Instruction::Rem {
                    typ,
                    arg: arg.map_generic(visitor, &full_type, false)?,
                }
            }
            ast::Instruction::Prmt { control, arg } => ast::Instruction::Prmt {
                control,
                arg: arg.map_prmt(visitor)?,
            },
            ast::Instruction::PrmtSlow { control, arg } => ast::Instruction::PrmtSlow {
                arg: arg.map_prmt(visitor)?,
                control: ast::Arg1 { src: control }
                    .map(
                        visitor,
                        false,
                        Some((
                            &ast::Type::Scalar(ast::ScalarType::B32),
                            ast::StateSpace::Reg,
                        )),
                    )?
                    .src,
            },
            ast::Instruction::Activemask { arg } => ast::Instruction::Activemask {
                arg: arg.map(
                    visitor,
                    true,
                    Some((
                        &ast::Type::Scalar(ast::ScalarType::B32),
                        ast::StateSpace::Reg,
                    )),
                )?,
            },
            ast::Instruction::Membar { level } => ast::Instruction::Membar { level },
            ast::Instruction::MadC {
                type_,
                arg,
                is_hi,
                carry_out,
            } => ast::Instruction::MadC {
                type_,
                is_hi,
                carry_out,
                arg: arg.map(visitor, &ast::Type::Scalar(type_), false)?,
            },
            ast::Instruction::MadCC { type_, arg, is_hi } => ast::Instruction::MadCC {
                type_,
                is_hi,
                arg: arg.map(visitor, &ast::Type::Scalar(type_), false)?,
            },
            ast::Instruction::Tex(details, arg) => {
                let image_type_space = if details.direct {
                    (ast::Type::Texref, ast::StateSpace::Global)
                } else {
                    (
                        ast::Type::Scalar(ast::ScalarType::B64),
                        ast::StateSpace::Reg,
                    )
                };
                let arg = arg.map(
                    visitor,
                    image_type_space,
                    details.geometry,
                    ast::Type::Vector(details.channel_type, 4),
                    details.coordinate_type,
                )?;
                ast::Instruction::Tex(details, arg)
            }
            ast::Instruction::Suld(details, arg) => {
                let arg = arg.map(
                    visitor,
                    (ast::Type::Surfref, ast::StateSpace::Global),
                    details.geometry,
                    details.value_type(),
                    ast::ScalarType::B32,
                )?;
                ast::Instruction::Suld(details, arg)
            }
            ast::Instruction::Sust(details, arg) => {
                let arg = arg.map(visitor, &details)?;
                ast::Instruction::Sust(details, arg)
            }
            ast::Instruction::Shfl(mode, arg) => {
                let arg = arg.map(visitor)?;
                ast::Instruction::Shfl(mode, arg)
            }
            ast::Instruction::Shf(details, arg) => {
                let arg = arg.map(visitor, &ast::Type::Scalar(ast::ScalarType::B32), false)?;
                ast::Instruction::Shf(details, arg)
            }
            ast::Instruction::Vote(details, arg) => {
                let arg = arg.map_vote(visitor, details.mode)?;
                ast::Instruction::Vote(details, arg)
            }
            ast::Instruction::BarRed(details, arg) => {
                let arg = arg.map_bar_red(visitor, details)?;
                ast::Instruction::BarRed(details, arg)
            }
            ast::Instruction::Trap => ast::Instruction::Trap,
            ast::Instruction::Brkpt => ast::Instruction::Brkpt,
            ast::Instruction::AddC(details, arg) => {
                let arg = arg.map_generic(visitor, &ast::Type::Scalar(details.type_), false)?;
                ast::Instruction::AddC(details, arg)
            }
            ast::Instruction::AddCC(type_, arg) => {
                let arg = arg.map_generic(visitor, &ast::Type::Scalar(type_), false)?;
                ast::Instruction::AddCC(type_, arg)
            }
            ast::Instruction::SubC(details, arg) => {
                let arg = arg.map_generic(visitor, &ast::Type::Scalar(details.type_), false)?;
                ast::Instruction::SubC(details, arg)
            }
            ast::Instruction::SubCC(type_, arg) => {
                let arg = arg.map_generic(visitor, &ast::Type::Scalar(type_), false)?;
                ast::Instruction::SubCC(type_, arg)
            }
            ast::Instruction::Vshr(arg) => ast::Instruction::Vshr(arg.map(
                visitor,
                &ast::Type::Scalar(ast::ScalarType::U32),
                false,
            )?),
            ast::Instruction::Set(details, arg) => {
                let arg = arg.map_different_types(
                    visitor,
                    &ast::Type::Scalar(details.dst_type),
                    &ast::Type::Scalar(details.src_type),
                )?;
                ast::Instruction::Set(details, arg)
            }
            ast::Instruction::Dp4a(type_, arg) => {
                let arg = arg.map(visitor, &ast::Type::Scalar(type_), false)?;
                ast::Instruction::Dp4a(type_, arg)
            }
            ast::Instruction::MatchAny(arg) => {
                let arg =
                    arg.map_generic(visitor, &ast::Type::Scalar(ast::ScalarType::B32), false)?;
                ast::Instruction::MatchAny(arg)
            }
            ast::Instruction::Red(details, args) => {
                let args = args.map(
                    visitor,
                    &ast::Type::Scalar(details.inner.get_type()),
                    details.space,
                )?;
                ast::Instruction::Red(details, args)
            }
            ast::Instruction::Nanosleep(a) => ast::Instruction::Nanosleep(a.map(
                visitor,
                false,
                Some((
                    &ast::Type::Scalar(ast::ScalarType::U32),
                    ast::StateSpace::Reg,
                )),
            )?),
            ast::Instruction::Sad(type_, a) => {
                ast::Instruction::Sad(type_, a.map(visitor, &ast::Type::Scalar(type_), false)?)
            }
        })
    }
}

impl<T: ArgParamsEx, U: ArgParamsEx> Visitable<T, U> for ast::Instruction<T> {
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::Instruction(self.map(visitor)?))
    }
}

impl ImplicitConversion {
    fn map<T: ArgParamsEx<Id = Id>, U: ArgParamsEx<Id = Id>, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        let new_dst = visitor.id(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((&self.to_type, self.to_space)),
        )?;
        let new_src = visitor.id(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((&self.from_type, self.from_space)),
        )?;
        Ok(Statement::Conversion({
            ImplicitConversion {
                src: new_src,
                dst: new_dst,
                ..self
            }
        }))
    }
}

#[derive(Copy, Clone)]
pub(crate) enum FPDenormMode {
    FlushToZero,
    Preserve,
}

impl<From: ArgParamsEx<Id = Id>, To: ArgParamsEx<Id = Id>> Visitable<From, To>
    for ImplicitConversion
{
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<From, To>,
    ) -> Result<Statement<ast::Instruction<To>, To>, TranslateError> {
        Ok(self.map(visitor)?)
    }
}

impl<T> ArgumentMapVisitor<TypedArgParams, TypedArgParams> for T
where
    T: FnMut(
        ArgumentDescriptor<Id>,
        Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<Id, TranslateError>,
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<Id>,
        t: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<Id, TranslateError> {
        self(desc, t)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<TypedOperand>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<TypedOperand, TranslateError> {
        Ok(match desc.op {
            TypedOperand::Reg(id) => {
                TypedOperand::Reg(self(desc.new_op(id), Some((typ, state_space)))?)
            }
            TypedOperand::Imm(imm) => TypedOperand::Imm(imm),
            TypedOperand::RegOffset(id, imm) => {
                TypedOperand::RegOffset(self(desc.new_op(id), Some((typ, state_space)))?, imm)
            }
            TypedOperand::VecMember(reg, index) => {
                let scalar_type = match typ {
                    ast::Type::Scalar(scalar_t) => *scalar_t,
                    _ => return Err(TranslateError::unreachable()),
                };
                let vec_type = ast::Type::Vector(scalar_type, index + 1);
                TypedOperand::VecMember(
                    self(desc.new_op(reg), Some((&vec_type, state_space)))?,
                    index,
                )
            }
        })
    }
}

impl ast::Type {
    pub(crate) fn widen(self) -> Result<Self, TranslateError> {
        match self {
            ast::Type::Scalar(scalar) => Ok(ast::Type::Scalar(scalar.widen()?)),
            _ => Err(TranslateError::unreachable()),
        }
    }

    pub(crate) fn to_parts(&self) -> TypeParts {
        let width = self.layout().size() as u8;
        match self {
            ast::Type::Scalar(scalar) => TypeParts {
                kind: TypeKind::Scalar,
                state_space: ast::StateSpace::Reg,
                scalar_kind: scalar.kind(),
                components: Vec::new(),
                width,
            },
            ast::Type::Vector(scalar, components) => TypeParts {
                kind: TypeKind::Vector,
                state_space: ast::StateSpace::Reg,
                scalar_kind: scalar.kind(),
                components: vec![*components as u32],
                width,
            },
            ast::Type::Array(scalar, components) => TypeParts {
                kind: TypeKind::Array,
                state_space: ast::StateSpace::Reg,
                scalar_kind: scalar.kind(),
                components: components.clone(),
                width,
            },
            ast::Type::Pointer(scalar, space) => TypeParts {
                kind: TypeKind::Pointer,
                state_space: *space,
                scalar_kind: scalar.kind(),
                components: Vec::new(),
                width,
            },
            ast::Type::Texref => TypeParts {
                kind: TypeKind::Texref,
                state_space: ast::StateSpace::Global,
                scalar_kind: ast::ScalarKind::Bit,
                components: Vec::new(),
                width,
            },
            ast::Type::Surfref => TypeParts {
                kind: TypeKind::Surfref,
                state_space: ast::StateSpace::Global,
                scalar_kind: ast::ScalarKind::Bit,
                components: Vec::new(),
                width,
            },
            ast::Type::Struct(fields) => {
                let components = fields
                    .iter()
                    .map(|field| unsafe { mem::transmute::<_, u16>(*field) as u32 })
                    .collect();
                TypeParts {
                    kind: TypeKind::Struct,
                    state_space: ast::StateSpace::Reg,
                    scalar_kind: ast::ScalarKind::Bit,
                    components,
                    width,
                }
            }
        }
    }

    pub(crate) fn from_parts(t: TypeParts) -> Self {
        match t.kind {
            TypeKind::Scalar => {
                ast::Type::Scalar(ast::ScalarType::from_parts(t.width, t.scalar_kind))
            }
            TypeKind::Vector => ast::Type::Vector(
                ast::ScalarType::from_parts(t.width, t.scalar_kind),
                t.components[0] as u8,
            ),
            TypeKind::Array => ast::Type::Array(
                ast::ScalarType::from_parts(t.width, t.scalar_kind),
                t.components,
            ),
            TypeKind::Pointer => ast::Type::Pointer(
                ast::ScalarType::from_parts(t.width, t.scalar_kind),
                t.state_space,
            ),
            TypeKind::Texref => ast::Type::Texref,
            TypeKind::Surfref => ast::Type::Surfref,
            TypeKind::Struct => ast::Type::Struct(
                t.components
                    .into_iter()
                    .map(|component| unsafe { mem::transmute(component as u16) })
                    .collect(),
            ),
        }
    }

    pub(crate) fn layout(&self) -> Layout {
        match self {
            ast::Type::Scalar(typ) => {
                let size_of = typ.size_of() as usize;
                unsafe { Layout::from_size_align_unchecked(size_of, size_of) }
            }
            ast::Type::Vector(typ, len) => {
                let size_of = typ.size_of() as usize * (*len) as usize;
                unsafe { Layout::from_size_align_unchecked(size_of, size_of) }
            }
            ast::Type::Array(typ, len) => {
                let scalar_size_of = typ.size_of() as usize;
                let len = len
                    .iter()
                    .fold(typ.size_of() as usize, |x, y| (x as usize) * (*y as usize));
                unsafe { Layout::from_size_align_unchecked(scalar_size_of * len, scalar_size_of) }
            }
            ast::Type::Struct(fields) => {
                let mut layout = Layout::new::<()>();
                for field in fields {
                    layout = layout.extend(field.to_type().layout()).unwrap().0
                }
                layout.pad_to_align()
            }
            ast::Type::Pointer(..) => Layout::new::<*const ()>(),
            ast::Type::Texref | ast::Type::Surfref => Layout::new::<usize>(),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
pub(crate) struct TypeParts {
    pub(crate) kind: TypeKind,
    pub(crate) scalar_kind: ast::ScalarKind,
    pub(crate) width: u8,
    pub(crate) state_space: ast::StateSpace,
    pub(crate) components: Vec<u32>,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub(crate) enum TypeKind {
    Scalar,
    Vector,
    Array,
    Pointer,
    Texref,
    Surfref,
    Struct,
}

impl<T: ast::ArgParams> ast::Instruction<T> {
    // .wide instructions don't support ftz, so it's enough to just look at the
    // type declared by the instruction
    fn flush_to_zero(&self) -> Option<(bool, u8)> {
        fn scalar_size_of(type_: ast::ScalarType) -> u8 {
            match type_ {
                ast::ScalarType::U8 => 1,
                ast::ScalarType::S8 => 1,
                ast::ScalarType::B8 => 1,
                ast::ScalarType::U16 => 2,
                ast::ScalarType::S16 => 2,
                ast::ScalarType::B16 => 2,
                ast::ScalarType::F16 => 2,
                ast::ScalarType::U32 => 4,
                ast::ScalarType::S32 => 4,
                ast::ScalarType::B32 => 4,
                ast::ScalarType::F32 => 4,
                ast::ScalarType::U64 => 8,
                ast::ScalarType::S64 => 8,
                ast::ScalarType::B64 => 8,
                ast::ScalarType::F64 => 8,
                ast::ScalarType::F16x2 => 2,
                ast::ScalarType::Pred => 1,
            }
        }

        match self {
            ast::Instruction::Ld(_, _) => None,
            ast::Instruction::St(_, _) => None,
            ast::Instruction::Mov(_, _) => None,
            ast::Instruction::Not(_, _) => None,
            ast::Instruction::Bra(_, _) => None,
            ast::Instruction::Shl(_, _) => None,
            ast::Instruction::Shr(_, _) => None,
            ast::Instruction::Ret(_) => None,
            ast::Instruction::Exit => None,
            ast::Instruction::Trap => None,
            ast::Instruction::Brkpt => None,
            ast::Instruction::Call(_) => None,
            ast::Instruction::Or(_, _) => None,
            ast::Instruction::And(_, _) => None,
            ast::Instruction::Cvta(_, _) => None,
            ast::Instruction::Selp(_, _) => None,
            ast::Instruction::Bar(_, _) => None,
            ast::Instruction::BarWarp(_, _) => None,
            ast::Instruction::Atom(_, _) => None,
            ast::Instruction::Red(_, _) => None,
            ast::Instruction::AtomCas(_, _) => None,
            ast::Instruction::MadC { .. } => None,
            ast::Instruction::MadCC { .. } => None,
            ast::Instruction::BarRed { .. } => None,
            ast::Instruction::AddC { .. } => None,
            ast::Instruction::AddCC { .. } => None,
            ast::Instruction::SubC { .. } => None,
            ast::Instruction::SubCC { .. } => None,
            ast::Instruction::Vshr { .. } => None,
            ast::Instruction::Dp4a { .. } => None,
            ast::Instruction::MatchAny { .. } => None,
            ast::Instruction::Sub(ast::ArithDetails::Signed(_), _) => None,
            ast::Instruction::Sub(ast::ArithDetails::Unsigned(_), _) => None,
            ast::Instruction::Add(ast::ArithDetails::Signed(_), _) => None,
            ast::Instruction::Add(ast::ArithDetails::Unsigned(_), _) => None,
            ast::Instruction::Mul(ast::MulDetails::Unsigned(_), _) => None,
            ast::Instruction::Mul(ast::MulDetails::Signed(_), _) => None,
            ast::Instruction::Mad(ast::MulDetails::Unsigned(_), _) => None,
            ast::Instruction::Mad(ast::MulDetails::Signed(_), _) => None,
            ast::Instruction::Min(ast::MinMaxDetails::Signed(_), _) => None,
            ast::Instruction::Min(ast::MinMaxDetails::Unsigned(_), _) => None,
            ast::Instruction::Max(ast::MinMaxDetails::Signed(_), _) => None,
            ast::Instruction::Max(ast::MinMaxDetails::Unsigned(_), _) => None,
            ast::Instruction::Cvt(ast::CvtDetails::IntFromInt(_), _) => None,
            ast::Instruction::Cvt(ast::CvtDetails::FloatFromInt(_), _) => None,
            ast::Instruction::Div(ast::DivDetails::Unsigned(_), _) => None,
            ast::Instruction::Div(ast::DivDetails::Signed(_), _) => None,
            ast::Instruction::Clz { .. } => None,
            ast::Instruction::Bfind(..) => None,
            ast::Instruction::Brev { .. } => None,
            ast::Instruction::Popc { .. } => None,
            ast::Instruction::Xor { .. } => None,
            ast::Instruction::Bfe { .. } => None,
            ast::Instruction::Bfi { .. } => None,
            ast::Instruction::Rem { .. } => None,
            ast::Instruction::Prmt { .. } => None,
            ast::Instruction::PrmtSlow { .. } => None,
            ast::Instruction::Activemask { .. } => None,
            ast::Instruction::Membar { .. } => None,
            ast::Instruction::Tex(..) => None,
            ast::Instruction::Suld(..) => None,
            ast::Instruction::Sust(..) => None,
            ast::Instruction::Shfl(..) => None,
            ast::Instruction::Shf(..) => None,
            ast::Instruction::Vote(..) => None,
            ast::Instruction::Nanosleep(..) => None,
            ast::Instruction::Sad(_, _) => None,
            ast::Instruction::Sub(ast::ArithDetails::Float(float_control), _)
            | ast::Instruction::Add(ast::ArithDetails::Float(float_control), _)
            | ast::Instruction::Mul(ast::MulDetails::Float(float_control), _)
            | ast::Instruction::Mad(ast::MulDetails::Float(float_control), _) => {
                float_control.flush_to_zero.map(|ftz| {
                    (
                        ftz,
                        scalar_size_of(ast::ScalarType::from(float_control.typ)),
                    )
                })
            }
            ast::Instruction::Fma(d, _) => d.flush_to_zero.map(|ftz| (ftz, scalar_size_of(d.typ))),
            ast::Instruction::Setp(details, _) => details
                .flush_to_zero
                .map(|ftz| (ftz, scalar_size_of(details.typ))),
            ast::Instruction::SetpBool(details, _) => details
                .base
                .flush_to_zero
                .map(|ftz| (ftz, scalar_size_of(details.base.typ))),
            ast::Instruction::Abs(details, _) => details
                .flush_to_zero
                .map(|ftz| (ftz, scalar_size_of(details.typ))),
            ast::Instruction::Min(ast::MinMaxDetails::Float(float_control), _)
            | ast::Instruction::Max(ast::MinMaxDetails::Float(float_control), _) => {
                float_control.flush_to_zero.map(|ftz| {
                    (
                        ftz,
                        scalar_size_of(ast::ScalarType::from(float_control.typ)),
                    )
                })
            }
            ast::Instruction::Rcp(details, _) => details
                .flush_to_zero
                .map(|ftz| (ftz, scalar_size_of(details.type_))),
            // Modifier .ftz can only be specified when either .dtype or .atype
            // is .f32 and applies only to single precision (.f32) inputs and results.
            ast::Instruction::Cvt(
                ast::CvtDetails::FloatFromFloat(ast::CvtDesc { flush_to_zero, .. }),
                _,
            )
            | ast::Instruction::Cvt(
                ast::CvtDetails::IntFromFloat(ast::CvtDesc { flush_to_zero, .. }),
                _,
            ) => flush_to_zero.map(|ftz| (ftz, 4)),
            ast::Instruction::Div(ast::DivDetails::Float(details), _) => details
                .flush_to_zero
                .map(|ftz| (ftz, scalar_size_of(ast::ScalarType::from(details.typ)))),
            ast::Instruction::Sqrt(details, _) => details
                .flush_to_zero
                .map(|ftz| (ftz, scalar_size_of(details.type_))),
            ast::Instruction::Rsqrt(details, _) => Some((
                details.flush_to_zero,
                scalar_size_of(ast::ScalarType::from(details.typ)),
            )),
            ast::Instruction::Neg(details, _) => details
                .flush_to_zero
                .map(|ftz| (ftz, scalar_size_of(details.typ))),
            ast::Instruction::Sin { flush_to_zero, .. }
            | ast::Instruction::Cos { flush_to_zero, .. }
            | ast::Instruction::Lg2 { flush_to_zero, .. }
            | ast::Instruction::Ex2 { flush_to_zero, .. } => {
                Some((*flush_to_zero, mem::size_of::<f32>() as u8))
            }
            ast::Instruction::Set(
                ast::SetData {
                    flush_to_zero,
                    src_type,
                    ..
                },
                _,
            ) => Some((*flush_to_zero, scalar_size_of(*src_type))),
        }
    }
}

type Arg2 = ast::Arg2<ExpandedArgParams>;
type Arg2St = ast::Arg2St<ExpandedArgParams>;

pub(crate) struct ConstantDefinition {
    pub dst: Id,
    pub typ: ast::ScalarType,
    pub value: ast::ImmediateValue,
}

pub(crate) struct BrachCondition {
    pub(crate) predicate: Id,
    pub(crate) if_true: Id,
    pub(crate) if_false: Id,
}

impl<From: ArgParamsEx<Id = Id>, To: ArgParamsEx<Id = Id>> Visitable<From, To> for BrachCondition {
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<From, To>,
    ) -> Result<Statement<ast::Instruction<To>, To>, TranslateError> {
        let predicate = visitor.id(
            ArgumentDescriptor {
                op: self.predicate,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((
                &ast::Type::Scalar(ast::ScalarType::Pred),
                ast::StateSpace::Reg,
            )),
        )?;
        let if_true = self.if_true;
        let if_false = self.if_false;
        Ok(Statement::Conditional(BrachCondition {
            predicate,
            if_true,
            if_false,
        }))
    }
}

#[derive(Clone)]
pub(crate) struct ImplicitConversion {
    pub(crate) src: Id,
    pub(crate) dst: Id,
    pub(crate) from_type: ast::Type,
    pub(crate) to_type: ast::Type,
    pub(crate) from_space: ast::StateSpace,
    pub(crate) to_space: ast::StateSpace,
    pub(crate) kind: ConversionKind,
}

#[derive(PartialEq, Clone)]
pub(crate) enum ConversionKind {
    Default,
    // zero-extend/chop/bitcast depending on types
    SignExtend,
    BitToPtr,
    PtrToPtr,
    AddressOf,
}

impl<T> ast::PredAt<T> {
    fn map_variable<U, F: FnMut(T) -> Result<U, TranslateError>>(
        self,
        f: &mut F,
    ) -> Result<ast::PredAt<U>, TranslateError> {
        let new_label = f(self.label)?;
        Ok(ast::PredAt {
            not: self.not,
            label: new_label,
        })
    }
}

impl<'a> ast::Instruction<ast::ParsedArgParams<'a>> {
    fn map_variable<F>(
        self,
        f: &mut F,
    ) -> Result<ast::Instruction<NormalizedArgParams>, TranslateError>
    where
        F: for<'x> FnMut(&'x str) -> Result<Id, TranslateError>,
    {
        match self {
            ast::Instruction::Call(call) => {
                let call_inst = ast::CallInst {
                    uniform: call.uniform,
                    ret_params: call
                        .ret_params
                        .into_iter()
                        .map(|p| f(p))
                        .collect::<Result<_, _>>()?,
                    func: f(call.func)?,
                    param_list: call
                        .param_list
                        .into_iter()
                        .map(|p| p.map_variable(f))
                        .collect::<Result<_, _>>()?,
                    prototype: call.prototype.map(f).transpose()?,
                };
                Ok(ast::Instruction::Call(call_inst))
            }
            i => i.map(f),
        }
    }
}

pub(crate) struct Arg4CarryOut<P: ast::ArgParams> {
    pub dst: P::Operand,
    pub carry_out: P::Operand,
    pub src1: P::Operand,
    pub src2: P::Operand,
    pub src3: P::Operand,
}

impl<T: ArgParamsEx> Arg4CarryOut<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        type_: ast::ScalarType,
    ) -> Result<Arg4CarryOut<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(type_),
            ast::StateSpace::Reg,
        )?;
        let carry_out = visitor.operand(
            ArgumentDescriptor {
                op: self.carry_out,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::Pred),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(type_),
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(type_),
            ast::StateSpace::Reg,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(type_),
            ast::StateSpace::Reg,
        )?;
        Ok(Arg4CarryOut {
            dst,
            src1,
            src2,
            src3,
            carry_out,
        })
    }

    fn new(arg: ast::Arg4<T>, carry_flag: T::Operand) -> Arg4CarryOut<T> {
        Arg4CarryOut {
            dst: arg.dst,
            src1: arg.src1,
            src2: arg.src2,
            src3: arg.src3,
            carry_out: carry_flag,
        }
    }
}

pub(crate) struct Arg4CarryIn<P: ast::ArgParams> {
    pub dst: P::Operand,
    pub carry_out: Option<P::Operand>,
    pub carry_in: P::Operand,
    pub src1: P::Operand,
    pub src2: P::Operand,
    pub src3: P::Operand,
}

impl<T: ArgParamsEx> Arg4CarryIn<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        type_: ast::ScalarType,
    ) -> Result<Arg4CarryIn<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(type_),
            ast::StateSpace::Reg,
        )?;
        let carry_out = self
            .carry_out
            .map(|carry_out| {
                visitor.operand(
                    ArgumentDescriptor {
                        op: carry_out,
                        is_dst: true,
                        is_memory_access: false,
                        non_default_implicit_conversion: None,
                    },
                    &ast::Type::Scalar(ast::ScalarType::Pred),
                    ast::StateSpace::Reg,
                )
            })
            .transpose()?;
        let carry_in = visitor.operand(
            ArgumentDescriptor {
                op: self.carry_in,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::Pred),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(type_),
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(type_),
            ast::StateSpace::Reg,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(type_),
            ast::StateSpace::Reg,
        )?;
        Ok(Arg4CarryIn {
            dst,
            src1,
            src2,
            src3,
            carry_in,
            carry_out,
        })
    }
}

impl<T: ArgParamsEx> Arg4CarryIn<T>
where
    T::Operand: Copy,
{
    fn new(arg: ast::Arg4<T>, carry_out: bool, carry_flag: T::Operand) -> Arg4CarryIn<T> {
        Arg4CarryIn {
            dst: arg.dst,
            src1: arg.src1,
            src2: arg.src2,
            src3: arg.src3,
            carry_in: carry_flag,
            carry_out: if carry_out { Some(carry_flag) } else { None },
        }
    }
}

pub(crate) struct Arg3CarryOut<P: ast::ArgParams> {
    pub dst: P::Operand,
    pub carry_flag: P::Operand,
    pub src1: P::Operand,
    pub src2: P::Operand,
}

impl<P: ArgParamsEx> Arg3CarryOut<P> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<P, U>>(
        self,
        visitor: &mut V,
        type_: ast::ScalarType,
    ) -> Result<Arg3CarryOut<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(type_),
            ast::StateSpace::Reg,
        )?;
        let carry_flag = visitor.operand(
            ArgumentDescriptor {
                op: self.carry_flag,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::Pred),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(type_),
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(type_),
            ast::StateSpace::Reg,
        )?;
        Ok(Arg3CarryOut {
            dst,
            carry_flag,
            src1,
            src2,
        })
    }

    fn new(args: ast::Arg3<P>, carry_flag: P::Operand) -> Arg3CarryOut<P> {
        Self {
            dst: args.dst,
            carry_flag,
            src1: args.src1,
            src2: args.src2,
        }
    }
}

pub(crate) struct Arg3CarryIn<P: ast::ArgParams> {
    pub dst: P::Operand,
    pub carry_out: Option<P::Operand>,
    pub carry_in: P::Operand,
    pub src1: P::Operand,
    pub src2: P::Operand,
}

impl<P: ArgParamsEx> Arg3CarryIn<P> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<P, U>>(
        self,
        visitor: &mut V,
        type_: ast::ScalarType,
    ) -> Result<Arg3CarryIn<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(type_),
            ast::StateSpace::Reg,
        )?;
        let carry_out = self
            .carry_out
            .map(|carry_out| {
                visitor.operand(
                    ArgumentDescriptor {
                        op: carry_out,
                        is_dst: true,
                        is_memory_access: false,
                        non_default_implicit_conversion: None,
                    },
                    &ast::Type::Scalar(ast::ScalarType::Pred),
                    ast::StateSpace::Reg,
                )
            })
            .transpose()?;
        let carry_in = visitor.operand(
            ArgumentDescriptor {
                op: self.carry_in,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::Pred),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(type_),
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(type_),
            ast::StateSpace::Reg,
        )?;
        Ok(Arg3CarryIn {
            dst,
            carry_in,
            carry_out,
            src1,
            src2,
        })
    }
}

impl<P: ArgParamsEx> Arg3CarryIn<P>
where
    P::Operand: Copy,
{
    fn new(args: ast::Arg3<P>, carry_out: bool, carry_flag: P::Operand) -> Arg3CarryIn<P> {
        Arg3CarryIn {
            dst: args.dst,
            carry_in: carry_flag,
            carry_out: if carry_out { Some(carry_flag) } else { None },
            src1: args.src1,
            src2: args.src2,
        }
    }
}

pub(crate) struct VisitAddC<P: ast::ArgParams>(pub ast::ScalarType, pub Arg3CarryIn<P>);

impl<T: ArgParamsEx<Id = Id>, U: ArgParamsEx<Id = Id>> Visitable<T, U> for VisitAddC<T> {
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::AddC(self.0, self.1.map(visitor, self.0)?))
    }
}

pub(crate) struct VisitAddCC<P: ast::ArgParams>(pub ast::ScalarType, pub Arg3CarryOut<P>);

impl<T: ArgParamsEx<Id = Id>, U: ArgParamsEx<Id = Id>> Visitable<T, U> for VisitAddCC<T> {
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::AddCC(self.0, self.1.map(visitor, self.0)?))
    }
}

pub(crate) struct VisitSubC<P: ast::ArgParams>(pub ast::ScalarType, pub Arg3CarryIn<P>);

impl<T: ArgParamsEx<Id = Id>, U: ArgParamsEx<Id = Id>> Visitable<T, U> for VisitSubC<T> {
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::SubC(self.0, self.1.map(visitor, self.0)?))
    }
}

pub(crate) struct VisitSubCC<P: ast::ArgParams>(pub ast::ScalarType, pub Arg3CarryOut<P>);

impl<T: ArgParamsEx<Id = Id>, U: ArgParamsEx<Id = Id>> Visitable<T, U> for VisitSubCC<T> {
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::SubCC(self.0, self.1.map(visitor, self.0)?))
    }
}

impl<T: ArgParamsEx> ast::Arg1<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        is_dst: bool,
        t: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<ast::Arg1<U>, TranslateError> {
        let new_src = visitor.id(
            ArgumentDescriptor {
                op: self.src,
                is_dst,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
        )?;
        Ok(ast::Arg1 { src: new_src })
    }
}

impl<T: ArgParamsEx> ast::Arg1Bar<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
    ) -> Result<ast::Arg1Bar<U>, TranslateError> {
        let new_src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::U32),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg1Bar { src: new_src })
    }
}

impl<T: ArgParamsEx> ast::Arg2<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: &ast::Type,
    ) -> Result<ast::Arg2<U>, TranslateError> {
        let new_dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let new_src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg2 {
            dst: new_dst,
            src: new_src,
        })
    }

    fn map_cvt<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        dst_t: ast::ScalarType,
        src_t: ast::ScalarType,
        is_int_to_int: bool,
    ) -> Result<ast::Arg2<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: if is_int_to_int {
                    Some(should_convert_relaxed_dst_wrapper)
                } else {
                    None
                },
            },
            &ast::Type::Scalar(dst_t),
            ast::StateSpace::Reg,
        )?;
        let src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: if is_int_to_int {
                    Some(should_convert_relaxed_src_wrapper)
                } else {
                    None
                },
            },
            &ast::Type::Scalar(src_t),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg2 { dst, src })
    }

    fn map_different_types<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        dst_t: &ast::Type,
        src_t: &ast::Type,
    ) -> Result<ast::Arg2<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            dst_t,
            ast::StateSpace::Reg,
        )?;
        let src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            src_t,
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg2 { dst, src })
    }
}

impl<T: ArgParamsEx> ast::Arg2Ld<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        details: &ast::LdDetails,
    ) -> Result<ast::Arg2Ld<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: Some(should_convert_relaxed_dst_wrapper),
            },
            &ast::Type::from(details.typ.clone()),
            ast::StateSpace::Reg,
        )?;
        let src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                is_memory_access: true,
                non_default_implicit_conversion: None,
            },
            &details.typ,
            details.state_space,
        )?;
        Ok(ast::Arg2Ld { dst, src })
    }
}

impl<T: ArgParamsEx> ast::Arg2St<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        type_: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<ast::Arg2St<U>, TranslateError> {
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: true,
                non_default_implicit_conversion: None,
            },
            &type_,
            state_space,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: Some(should_convert_relaxed_src_wrapper),
            },
            &type_.clone().into(),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg2St { src1, src2 })
    }
}

impl<T: ArgParamsEx> ast::Arg2Mov<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        details: &ast::MovDetails,
    ) -> Result<ast::Arg2Mov<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &details.typ.clone().into(),
            ast::StateSpace::Reg,
        )?;
        let src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: Some(implicit_conversion_mov),
            },
            &details.typ.clone().into(),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg2Mov { dst, src })
    }
}

impl<T: ArgParamsEx> ast::Arg3<T> {
    fn map_generic<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        typ: &ast::Type,
        is_wide: bool,
    ) -> Result<ast::Arg3<U>, TranslateError> {
        let wide_type = if is_wide {
            Some(typ.clone().widen()?)
        } else {
            None
        };
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            wide_type.as_ref().unwrap_or(typ),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            typ,
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            typ,
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg3 { dst, src1, src2 })
    }

    fn map_different_types<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        dst_type: &ast::Type,
        src_type: &ast::Type,
    ) -> Result<ast::Arg3<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            dst_type,
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            src_type,
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            src_type,
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg3 { dst, src1, src2 })
    }

    fn map_shift<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: &ast::Type,
    ) -> Result<ast::Arg3<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::U32),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg3 { dst, src1, src2 })
    }

    fn map_atom<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::ScalarType,
        state_space: ast::StateSpace,
    ) -> Result<ast::Arg3<U>, TranslateError> {
        let scalar_type = ast::ScalarType::from(t);
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(scalar_type),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: true,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(scalar_type),
            state_space,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(scalar_type),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg3 { dst, src1, src2 })
    }

    fn map_prmt<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
    ) -> Result<ast::Arg3<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::B32),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::B32),
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::B32),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg3 { dst, src1, src2 })
    }

    fn map_vote<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        mode: ast::VoteMode,
    ) -> Result<ast::Arg3<U>, TranslateError> {
        let dst_type = match mode {
            ast::VoteMode::Ballot => ast::ScalarType::B32,
            ast::VoteMode::All | ast::VoteMode::Any | ast::VoteMode::Uni => ast::ScalarType::Pred,
        };
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(dst_type),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::Pred),
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::B32),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg3 { dst, src1, src2 })
    }

    fn map_bar_red<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        op: ast::ReductionOp,
    ) -> Result<ast::Arg3<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(op.dst_type()),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::B32),
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::Pred),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg3 { dst, src1, src2 })
    }
}

fn texture_geometry_to_vec_length(geometry: ast::TextureGeometry) -> u8 {
    match geometry {
        ast::TextureGeometry::OneD | ast::TextureGeometry::Array1D => 1u8,
        ast::TextureGeometry::TwoD | ast::TextureGeometry::Array2D => 2,
        ast::TextureGeometry::ThreeD => 4,
    }
}

impl<T: ArgParamsEx> ast::Arg4Tex<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        (image_type, image_space): (ast::Type, ast::StateSpace),
        geometry: ast::TextureGeometry,
        value_type: ast::Type,
        coordinate_type: ast::ScalarType,
    ) -> Result<ast::Arg4Tex<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: Some(should_convert_relaxed_dst_wrapper),
            },
            &value_type,
            ast::StateSpace::Reg,
        )?;
        let image = visitor.operand(
            ArgumentDescriptor {
                op: self.image,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &image_type,
            image_space,
        )?;
        let layer = self
            .layer
            .map(|layer| {
                visitor.operand(
                    ArgumentDescriptor {
                        op: layer,
                        is_dst: false,
                        is_memory_access: false,
                        non_default_implicit_conversion: None,
                    },
                    &ast::Type::Scalar(ast::ScalarType::U32),
                    ast::StateSpace::Reg,
                )
            })
            .transpose()?;
        let coord_length = texture_geometry_to_vec_length(geometry);
        let coordinates = visitor.operand(
            ArgumentDescriptor {
                op: self.coordinates,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Vector(coordinate_type, coord_length),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg4Tex {
            dst,
            image,
            layer,
            coordinates,
        })
    }
}

impl<T: ArgParamsEx> ast::Arg4Sust<T> {
    pub(crate) fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        details: &ast::SurfaceDetails,
    ) -> Result<ast::Arg4Sust<U>, TranslateError> {
        let image = visitor.operand(
            ArgumentDescriptor {
                op: self.image,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Surfref,
            ast::StateSpace::Global,
        )?;
        let layer = self
            .layer
            .map(|layer| {
                visitor.operand(
                    ArgumentDescriptor {
                        op: layer,
                        is_dst: false,
                        is_memory_access: false,
                        non_default_implicit_conversion: None,
                    },
                    &ast::Type::Scalar(ast::ScalarType::U32),
                    ast::StateSpace::Reg,
                )
            })
            .transpose()?;
        let coord_length = texture_geometry_to_vec_length(details.geometry);
        let coordinates = visitor.operand(
            ArgumentDescriptor {
                op: self.coordinates,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Vector(ast::ScalarType::B32, coord_length),
            ast::StateSpace::Reg,
        )?;
        let value_type = details.value_type();
        let value = visitor.operand(
            ArgumentDescriptor {
                op: self.value,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: Some(should_convert_relaxed_src_wrapper),
            },
            &value_type,
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg4Sust {
            image,
            coordinates,
            layer,
            value,
        })
    }
}

impl ast::TextureGeometry {
    fn as_ptx(self) -> &'static str {
        match self {
            ast::TextureGeometry::OneD => "1d",
            ast::TextureGeometry::TwoD => "2d",
            ast::TextureGeometry::ThreeD => "3d",
            ast::TextureGeometry::Array1D => "a1d",
            ast::TextureGeometry::Array2D => "a2d",
        }
    }
}

impl ast::SurfaceDetails {
    fn value_type(&self) -> ast::Type {
        match self.vector {
            Some(vec_length) => ast::Type::Vector(self.type_, vec_length),
            None => ast::Type::Scalar(self.type_),
        }
    }

    fn vector_ptx(&self) -> Result<&'static str, TranslateError> {
        Ok(match self.vector {
            Some(2) => "_v2",
            Some(4) => "_v4",
            Some(_) => return Err(TranslateError::unreachable()),
            None => "",
        })
    }
}

impl<T: ArgParamsEx> ast::Arg4<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: &ast::Type,
        is_wide: bool,
    ) -> Result<ast::Arg4<U>, TranslateError> {
        let wide_type = if is_wide {
            t.clone().widen()?
        } else {
            t.clone()
        };
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &wide_type,
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &wide_type,
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg4 {
            dst,
            src1,
            src2,
            src3,
        })
    }

    fn map_selp<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::ScalarType,
    ) -> Result<ast::Arg4<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(t.into()),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(t.into()),
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(t.into()),
            ast::StateSpace::Reg,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::Pred),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg4 {
            dst,
            src1,
            src2,
            src3,
        })
    }

    fn map_atom<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::ScalarType,
        state_space: ast::StateSpace,
    ) -> Result<ast::Arg4<U>, TranslateError> {
        let scalar_type = ast::ScalarType::from(t);
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(scalar_type),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: true,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(scalar_type),
            state_space,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(scalar_type),
            ast::StateSpace::Reg,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(scalar_type),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg4 {
            dst,
            src1,
            src2,
            src3,
        })
    }

    fn map_bfe<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        typ: &ast::Type,
    ) -> Result<ast::Arg4<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            typ,
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            typ,
            ast::StateSpace::Reg,
        )?;
        let u32_type = ast::Type::Scalar(ast::ScalarType::U32);
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &u32_type,
            ast::StateSpace::Reg,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &u32_type,
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg4 {
            dst,
            src1,
            src2,
            src3,
        })
    }
}

impl<T: ArgParamsEx> ast::Arg4Setp<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: &ast::Type,
    ) -> Result<ast::Arg4Setp<U>, TranslateError> {
        let dst1 = visitor.id(
            ArgumentDescriptor {
                op: self.dst1,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((
                &ast::Type::Scalar(ast::ScalarType::Pred),
                ast::StateSpace::Reg,
            )),
        )?;
        let dst2 = self
            .dst2
            .map(|dst2| {
                visitor.id(
                    ArgumentDescriptor {
                        op: dst2,
                        is_dst: true,
                        is_memory_access: false,
                        non_default_implicit_conversion: None,
                    },
                    Some((
                        &ast::Type::Scalar(ast::ScalarType::Pred),
                        ast::StateSpace::Reg,
                    )),
                )
            })
            .transpose()?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg4Setp {
            dst1,
            dst2,
            src1,
            src2,
        })
    }
}

impl<T: ArgParamsEx> ast::Arg5<T> {
    fn map_bfi<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        base_type: &ast::Type,
    ) -> Result<ast::Arg5<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            base_type,
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            base_type,
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            base_type,
            ast::StateSpace::Reg,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::U32),
            ast::StateSpace::Reg,
        )?;
        let src4 = visitor.operand(
            ArgumentDescriptor {
                op: self.src4,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::U32),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg5 {
            dst,
            src1,
            src2,
            src3,
            src4,
        })
    }
}

impl<T: ArgParamsEx> ast::Arg5Setp<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: &ast::Type,
    ) -> Result<ast::Arg5Setp<U>, TranslateError> {
        let dst1 = visitor.id(
            ArgumentDescriptor {
                op: self.dst1,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((
                &ast::Type::Scalar(ast::ScalarType::Pred),
                ast::StateSpace::Reg,
            )),
        )?;
        let dst2 = self
            .dst2
            .map(|dst2| {
                visitor.id(
                    ArgumentDescriptor {
                        op: dst2,
                        is_dst: true,
                        is_memory_access: false,
                        non_default_implicit_conversion: None,
                    },
                    Some((
                        &ast::Type::Scalar(ast::ScalarType::Pred),
                        ast::StateSpace::Reg,
                    )),
                )
            })
            .transpose()?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::Pred),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg5Setp {
            dst1,
            dst2,
            src1,
            src2,
            src3,
        })
    }
}

impl<T: ArgParamsEx> ast::Arg5Shfl<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
    ) -> Result<ast::Arg5Shfl<U>, TranslateError> {
        let dst1 = visitor.id(
            ArgumentDescriptor {
                op: self.dst1,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((
                &ast::Type::Scalar(ast::ScalarType::B32),
                ast::StateSpace::Reg,
            )),
        )?;
        let dst2 = self
            .dst2
            .map(|dst2| {
                visitor.id(
                    ArgumentDescriptor {
                        op: dst2,
                        is_dst: true,
                        is_memory_access: false,
                        non_default_implicit_conversion: None,
                    },
                    Some((
                        &ast::Type::Scalar(ast::ScalarType::Pred),
                        ast::StateSpace::Reg,
                    )),
                )
            })
            .transpose()?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::B32),
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::B32),
            ast::StateSpace::Reg,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::B32),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg5Shfl {
            dst1,
            dst2,
            src1,
            src2,
            src3,
        })
    }
}

impl<T> ast::Operand<T> {
    fn map_variable<U, F: FnMut(T) -> Result<U, TranslateError>>(
        self,
        f: &mut F,
    ) -> Result<ast::Operand<U>, TranslateError> {
        Ok(match self {
            ast::Operand::Reg(reg) => ast::Operand::Reg(f(reg)?),
            ast::Operand::RegOffset(reg, offset) => ast::Operand::RegOffset(f(reg)?, offset),
            ast::Operand::Imm(x) => ast::Operand::Imm(x),
            ast::Operand::VecMember(reg, idx) => ast::Operand::VecMember(f(reg)?, idx),
            ast::Operand::VecPack(vec) => ast::Operand::VecPack(
                vec.into_iter()
                    .map(|reg_or_immediate| {
                        Ok::<_, TranslateError>(match reg_or_immediate {
                            ast::RegOrImmediate::Reg(reg) => ast::RegOrImmediate::Reg(f(reg)?),
                            ast::RegOrImmediate::Imm(imm) => ast::RegOrImmediate::Imm(imm),
                        })
                    })
                    .collect::<Result<_, _>>()?,
            ),
        })
    }
}

impl ast::ScalarType {
    pub(crate) fn widen(self) -> Result<Self, TranslateError> {
        let kind = self.kind();
        let width = self.size_of();
        if (kind != ast::ScalarKind::Signed
            && kind != ast::ScalarKind::Unsigned
            && kind != ast::ScalarKind::Bit)
            || (width == 8)
        {
            return Err(TranslateError::mismatched_type());
        }
        Ok(ast::ScalarType::from_parts(width * 2, kind))
    }

    pub(crate) fn from_parts(width: u8, kind: ast::ScalarKind) -> Self {
        match kind {
            ast::ScalarKind::Float => match width {
                2 => ast::ScalarType::F16,
                4 => ast::ScalarType::F32,
                8 => ast::ScalarType::F64,
                _ => unreachable!(),
            },
            ast::ScalarKind::Bit => match width {
                1 => ast::ScalarType::B8,
                2 => ast::ScalarType::B16,
                4 => ast::ScalarType::B32,
                8 => ast::ScalarType::B64,
                _ => unreachable!(),
            },
            ast::ScalarKind::Signed => match width {
                1 => ast::ScalarType::S8,
                2 => ast::ScalarType::S16,
                4 => ast::ScalarType::S32,
                8 => ast::ScalarType::S64,
                _ => unreachable!(),
            },
            ast::ScalarKind::Unsigned => match width {
                1 => ast::ScalarType::U8,
                2 => ast::ScalarType::U16,
                4 => ast::ScalarType::U32,
                8 => ast::ScalarType::U64,
                _ => unreachable!(),
            },
            ast::ScalarKind::Float2 => match width {
                4 => ast::ScalarType::F16x2,
                _ => unreachable!(),
            },
            ast::ScalarKind::Pred => ast::ScalarType::Pred,
        }
    }
}

impl ast::ArithDetails {
    pub(crate) fn get_type(&self) -> ast::ScalarType {
        match self {
            ast::ArithDetails::Unsigned(t) => *t,
            ast::ArithDetails::Signed(d) => d.typ,
            ast::ArithDetails::Float(d) => d.typ,
        }
    }
}

impl ast::MulDetails {
    fn get_type(&self) -> ast::Type {
        ast::Type::Scalar(match self {
            ast::MulDetails::Unsigned(d) => d.typ.into(),
            ast::MulDetails::Signed(d) => d.typ.into(),
            ast::MulDetails::Float(d) => d.typ.into(),
        })
    }
}

impl ast::MinMaxDetails {
    pub(crate) fn get_type(&self) -> ast::ScalarType {
        match self {
            ast::MinMaxDetails::Signed(t) => *t,
            ast::MinMaxDetails::Unsigned(t) => *t,
            ast::MinMaxDetails::Float(d) => d.typ,
        }
    }
}

impl ast::DivDetails {
    fn get_type(&self) -> ast::Type {
        ast::Type::Scalar(match self {
            ast::DivDetails::Unsigned(t) => (*t).into(),
            ast::DivDetails::Signed(t) => (*t).into(),
            ast::DivDetails::Float(d) => d.typ.into(),
        })
    }
}

impl ast::AtomInnerDetails {
    fn get_type(&self) -> ast::ScalarType {
        match self {
            ast::AtomInnerDetails::Bit { typ, .. } => (*typ).into(),
            ast::AtomInnerDetails::Unsigned { typ, .. } => (*typ).into(),
            ast::AtomInnerDetails::Signed { typ, .. } => (*typ).into(),
            ast::AtomInnerDetails::Float { typ, .. } => (*typ).into(),
        }
    }
}

impl ast::StateSpace {
    fn is_compatible(self, other: ast::StateSpace) -> bool {
        self == other
            || self == ast::StateSpace::Reg && other == ast::StateSpace::Sreg
            || self == ast::StateSpace::Sreg && other == ast::StateSpace::Reg
    }

    fn coerces_to_generic(self) -> bool {
        match self {
            ast::StateSpace::Global
            | ast::StateSpace::Const
            | ast::StateSpace::Local
            | ast::StateSpace::Shared => true,
            ast::StateSpace::Reg
            | ast::StateSpace::Param
            | ast::StateSpace::Generic
            | ast::StateSpace::Sreg => false,
        }
    }

    fn is_addressable(self) -> bool {
        match self {
            ast::StateSpace::Const
            | ast::StateSpace::Generic
            | ast::StateSpace::Global
            | ast::StateSpace::Local
            | ast::StateSpace::Shared
            | ast::StateSpace::Param => true,
            ast::StateSpace::Reg | ast::StateSpace::Sreg => false,
        }
    }
}

impl ast::MulDetails {
    fn is_wide(&self) -> bool {
        match self {
            ast::MulDetails::Unsigned(d) => d.control == ast::MulIntControl::Wide,
            ast::MulDetails::Signed(d) => d.control == ast::MulIntControl::Wide,
            ast::MulDetails::Float(_) => false,
        }
    }
}

impl ast::SurfaceDetails {
    fn suffix(&self) -> &'static str {
        match self.direct {
            true => "",
            false => "indirect_",
        }
    }
}

impl ast::TexDetails {
    fn suffix(&self) -> &'static str {
        match self.direct {
            true => "",
            false => "_indirect",
        }
    }
}

fn default_implicit_conversion(
    (operand_space, operand_type): (ast::StateSpace, &ast::Type),
    (instruction_space, instruction_type): (ast::StateSpace, &ast::Type),
) -> Result<Option<ConversionKind>, TranslateError> {
    if !instruction_space.is_compatible(operand_space) {
        default_implicit_conversion_space(
            (operand_space, operand_type),
            (instruction_space, instruction_type),
        )
    } else if instruction_type != operand_type {
        default_implicit_conversion_type(instruction_space, operand_type, instruction_type)
    } else {
        Ok(None)
    }
}

// Space is different
fn default_implicit_conversion_space(
    (operand_space, operand_type): (ast::StateSpace, &ast::Type),
    (instruction_space, instruction_type): (ast::StateSpace, &ast::Type),
) -> Result<Option<ConversionKind>, TranslateError> {
    if (instruction_space == ast::StateSpace::Generic && operand_space.coerces_to_generic())
        || (operand_space == ast::StateSpace::Generic && instruction_space.coerces_to_generic())
    {
        Ok(Some(ConversionKind::PtrToPtr))
    } else if operand_space.is_compatible(ast::StateSpace::Reg) {
        match operand_type {
            ast::Type::Pointer(operand_ptr_type, operand_ptr_space)
                if *operand_ptr_space == instruction_space =>
            {
                if instruction_type != &ast::Type::Scalar(*operand_ptr_type) {
                    Ok(Some(ConversionKind::PtrToPtr))
                } else {
                    Ok(None)
                }
            }
            // TODO: 32 bit
            ast::Type::Scalar(ast::ScalarType::B64)
            | ast::Type::Scalar(ast::ScalarType::U64)
            | ast::Type::Scalar(ast::ScalarType::S64) => match instruction_space {
                ast::StateSpace::Global
                | ast::StateSpace::Generic
                | ast::StateSpace::Const
                | ast::StateSpace::Local
                | ast::StateSpace::Param
                | ast::StateSpace::Shared => Ok(Some(ConversionKind::BitToPtr)),
                _ => Err(TranslateError::mismatched_type()),
            },
            ast::Type::Scalar(ast::ScalarType::B32)
            | ast::Type::Scalar(ast::ScalarType::U32)
            | ast::Type::Scalar(ast::ScalarType::S32) => match instruction_space {
                ast::StateSpace::Const | ast::StateSpace::Local | ast::StateSpace::Shared => {
                    Ok(Some(ConversionKind::BitToPtr))
                }
                _ => Err(TranslateError::mismatched_type()),
            },
            _ => Err(TranslateError::mismatched_type()),
        }
    } else if instruction_space.is_compatible(ast::StateSpace::Reg) {
        match instruction_type {
            ast::Type::Pointer(instruction_ptr_type, instruction_ptr_space)
                if operand_space == *instruction_ptr_space =>
            {
                if operand_type != &ast::Type::Scalar(*instruction_ptr_type) {
                    Ok(Some(ConversionKind::PtrToPtr))
                } else {
                    Ok(None)
                }
            }
            _ => Err(TranslateError::mismatched_type()),
        }
    } else {
        Err(TranslateError::mismatched_type())
    }
}

// Space is same, but type is different
fn default_implicit_conversion_type(
    space: ast::StateSpace,
    operand_type: &ast::Type,
    instruction_type: &ast::Type,
) -> Result<Option<ConversionKind>, TranslateError> {
    if space.is_compatible(ast::StateSpace::Reg) {
        if should_bitcast(instruction_type, operand_type) {
            Ok(Some(ConversionKind::Default))
        } else {
            Err(TranslateError::mismatched_type())
        }
    } else {
        Ok(Some(ConversionKind::PtrToPtr))
    }
}

fn should_bitcast(instr: &ast::Type, operand: &ast::Type) -> bool {
    match (instr, operand) {
        (ast::Type::Scalar(inst), ast::Type::Scalar(operand)) => {
            if inst.size_of() != operand.size_of() {
                return false;
            }
            match inst.kind() {
                ast::ScalarKind::Bit => operand.kind() != ast::ScalarKind::Bit,
                ast::ScalarKind::Float => operand.kind() == ast::ScalarKind::Bit,
                ast::ScalarKind::Signed => {
                    operand.kind() == ast::ScalarKind::Bit
                        || operand.kind() == ast::ScalarKind::Unsigned
                }
                ast::ScalarKind::Unsigned => {
                    operand.kind() == ast::ScalarKind::Bit
                        || operand.kind() == ast::ScalarKind::Signed
                        || operand.kind() == ast::ScalarKind::Float2
                }
                ast::ScalarKind::Float2 => {
                    operand.kind() == ast::ScalarKind::Bit
                        || operand.kind() == ast::ScalarKind::Unsigned
                }
                ast::ScalarKind::Pred => false,
            }
        }
        (ast::Type::Scalar(scalar), ast::Type::Vector(vector, width))
        | (ast::Type::Vector(vector, width), ast::Type::Scalar(scalar)) => {
            scalar.kind() == ast::ScalarKind::Bit && *width * vector.size_of() == scalar.size_of()
        }
        (ast::Type::Vector(inst, _), ast::Type::Vector(operand, _))
        | (ast::Type::Array(inst, _), ast::Type::Array(operand, _)) => {
            should_bitcast(&ast::Type::Scalar(*inst), &ast::Type::Scalar(*operand))
        }
        _ => false,
    }
}

fn implicit_conversion_mov(
    (operand_space, operand_type): (ast::StateSpace, &ast::Type),
    (instruction_space, instruction_type): (ast::StateSpace, &ast::Type),
) -> Result<Option<ConversionKind>, TranslateError> {
    // instruction_space is always reg
    if operand_space.is_compatible(ast::StateSpace::Reg) {
        if let (ast::Type::Vector(vec_underlying_type, vec_len), ast::Type::Scalar(scalar)) =
            (operand_type, instruction_type)
        {
            if scalar.kind() == ast::ScalarKind::Bit
                && scalar.size_of() == (vec_underlying_type.size_of() * vec_len)
            {
                return Ok(Some(ConversionKind::Default));
            }
        }
        // TODO: verify .params addressability:
        // * kernel arg
        // * func arg
        // * variable
    }
    if is_addressable(operand_type, operand_space) {
        return Ok(Some(ConversionKind::AddressOf));
    }
    default_implicit_conversion(
        (operand_space, operand_type),
        (instruction_space, instruction_type),
    )
}

fn is_addressable(type_: &ast::Type, state_space: ast::StateSpace) -> bool {
    if state_space.is_addressable() {
        return true;
    }
    if !state_space.is_compatible(ast::StateSpace::Reg) {
        return false;
    }
    match type_ {
        ast::Type::Pointer(_, space) => space.is_addressable(),
        _ => false,
    }
}

fn should_convert_relaxed_src_wrapper(
    (operand_space, operand_type): (ast::StateSpace, &ast::Type),
    (instruction_space, instruction_type): (ast::StateSpace, &ast::Type),
) -> Result<Option<ConversionKind>, TranslateError> {
    if !operand_space.is_compatible(instruction_space) {
        return Err(TranslateError::mismatched_type());
    }
    if operand_type == instruction_type {
        return Ok(None);
    }
    match should_convert_relaxed_src(operand_type, instruction_type) {
        conv @ Some(_) => Ok(conv),
        None => Err(TranslateError::mismatched_type()),
    }
}

// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#operand-size-exceeding-instruction-type-size__relaxed-type-checking-rules-source-operands
fn should_convert_relaxed_src(
    src_type: &ast::Type,
    instr_type: &ast::Type,
) -> Option<ConversionKind> {
    if src_type == instr_type {
        return None;
    }
    match (src_type, instr_type) {
        (ast::Type::Scalar(src_type), ast::Type::Scalar(instr_type)) => match instr_type.kind() {
            ast::ScalarKind::Bit => {
                if instr_type.size_of() <= src_type.size_of() {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Signed | ast::ScalarKind::Unsigned => {
                if instr_type.size_of() <= src_type.size_of()
                    && src_type.kind() != ast::ScalarKind::Float
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Float => {
                if instr_type.size_of() <= src_type.size_of()
                    && src_type.kind() == ast::ScalarKind::Bit
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Float2 => todo!(),
            ast::ScalarKind::Pred => None,
        },
        (ast::Type::Vector(dst_type, _), ast::Type::Vector(instr_type, _))
        | (ast::Type::Array(dst_type, _), ast::Type::Array(instr_type, _)) => {
            should_convert_relaxed_src(
                &ast::Type::Scalar(*dst_type),
                &ast::Type::Scalar(*instr_type),
            )
        }
        _ => None,
    }
}

fn should_convert_relaxed_dst_wrapper(
    (operand_space, operand_type): (ast::StateSpace, &ast::Type),
    (instruction_space, instruction_type): (ast::StateSpace, &ast::Type),
) -> Result<Option<ConversionKind>, TranslateError> {
    if !operand_space.is_compatible(instruction_space) {
        return Err(TranslateError::mismatched_type());
    }
    if operand_type == instruction_type {
        return Ok(None);
    }
    match should_convert_relaxed_dst(operand_type, instruction_type) {
        conv @ Some(_) => Ok(conv),
        None => Err(TranslateError::mismatched_type()),
    }
}

// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#operand-size-exceeding-instruction-type-size__relaxed-type-checking-rules-destination-operands
fn should_convert_relaxed_dst(
    dst_type: &ast::Type,
    instr_type: &ast::Type,
) -> Option<ConversionKind> {
    if dst_type == instr_type {
        return None;
    }
    match (dst_type, instr_type) {
        (ast::Type::Scalar(dst_type), ast::Type::Scalar(instr_type)) => match instr_type.kind() {
            ast::ScalarKind::Bit => {
                if instr_type.size_of() <= dst_type.size_of() {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Signed => {
                if dst_type.kind() != ast::ScalarKind::Float {
                    if instr_type.size_of() == dst_type.size_of() {
                        Some(ConversionKind::Default)
                    } else if instr_type.size_of() < dst_type.size_of() {
                        Some(ConversionKind::SignExtend)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            ast::ScalarKind::Unsigned => {
                if instr_type.size_of() <= dst_type.size_of()
                    && dst_type.kind() != ast::ScalarKind::Float
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Float => {
                if instr_type.size_of() <= dst_type.size_of()
                    && dst_type.kind() == ast::ScalarKind::Bit
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Float2 => todo!(),
            ast::ScalarKind::Pred => None,
        },
        (ast::Type::Vector(dst_type, _), ast::Type::Vector(instr_type, _))
        | (ast::Type::Array(dst_type, _), ast::Type::Array(instr_type, _)) => {
            should_convert_relaxed_dst(
                &ast::Type::Scalar(*dst_type),
                &ast::Type::Scalar(*instr_type),
            )
        }
        _ => None,
    }
}

impl<'a> ast::MethodDeclaration<'a, &'a str> {
    fn name(&self) -> &'a str {
        match self.name {
            ast::MethodName::Kernel(name) => name,
            ast::MethodName::Func(name) => name,
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) enum ConstType<'a> {
    Type(&'a ast::Type),
    ArraySubtype(ast::ScalarType, &'a [u32]),
}

impl ast::ScalarType {
    pub fn is_integer(self) -> bool {
        match self.kind() {
            ast::ScalarKind::Unsigned | ast::ScalarKind::Signed | ast::ScalarKind::Bit => true,
            ast::ScalarKind::Float | ast::ScalarKind::Float2 | ast::ScalarKind::Pred => false,
        }
    }
}

impl ast::ReductionOp {
    fn dst_type(self) -> ast::ScalarType {
        match self {
            ast::ReductionOp::And | ast::ReductionOp::Or => ast::ScalarType::Pred,
            ast::ReductionOp::Popc => ast::ScalarType::U32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;

    static SCALAR_TYPES: [ast::ScalarType; 15] = [
        ast::ScalarType::B8,
        ast::ScalarType::B16,
        ast::ScalarType::B32,
        ast::ScalarType::B64,
        ast::ScalarType::S8,
        ast::ScalarType::S16,
        ast::ScalarType::S32,
        ast::ScalarType::S64,
        ast::ScalarType::U8,
        ast::ScalarType::U16,
        ast::ScalarType::U32,
        ast::ScalarType::U64,
        ast::ScalarType::F16,
        ast::ScalarType::F32,
        ast::ScalarType::F64,
    ];

    static RELAXED_SRC_CONVERSION_TABLE: &'static str =
        "b8 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	chop 	chop 	chop
        b16 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	- 	chop 	chop
        b32 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	- 	chop
        b64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	-
        s8 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	inv 	inv 	inv
        s16 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	inv 	inv 	inv
        s32 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	inv 	inv
        s64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv
        u8 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	inv 	inv 	inv
        u16 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	inv 	inv 	inv
        u32 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	inv 	inv
        u64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv
        f16 	inv 	- 	chop 	chop 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	- 	inv 	inv
        f32 	inv 	inv 	- 	chop 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	- 	inv
        f64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	-";

    static RELAXED_DST_CONVERSION_TABLE: &'static str =
        "b8 	- 	zext 	zext 	zext 	- 	zext 	zext 	zext 	- 	zext 	zext 	zext 	zext 	zext 	zext
        b16 	inv 	- 	zext 	zext 	inv 	- 	zext 	zext 	inv 	- 	zext 	zext 	- 	zext 	zext
        b32 	inv 	inv 	- 	zext 	inv 	inv 	- 	zext 	inv 	inv 	- 	zext 	inv 	- 	zext
        b64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	-
        s8 	- 	sext 	sext 	sext 	- 	sext 	sext 	sext 	- 	sext 	sext 	sext 	inv 	inv 	inv
        s16 	inv 	- 	sext 	sext 	inv 	- 	sext 	sext 	inv 	- 	sext 	sext 	inv 	inv 	inv
        s32 	inv 	inv 	- 	sext 	inv 	inv 	- 	sext 	inv 	inv 	- 	sext 	inv 	inv 	inv
        s64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv
        u8 	- 	zext 	zext 	zext 	- 	zext 	zext 	zext 	- 	zext 	zext 	zext 	inv 	inv 	inv
        u16 	inv 	- 	zext 	zext 	inv 	- 	zext 	zext 	inv 	- 	zext 	zext 	inv 	inv 	inv
        u32 	inv 	inv 	- 	zext 	inv 	inv 	- 	zext 	inv 	inv 	- 	zext 	inv 	inv 	inv
        u64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv
        f16 	inv 	- 	zext 	zext 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	- 	inv 	inv
        f32 	inv 	inv 	- 	zext 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	- 	inv
        f64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	-";

    fn table_entry_to_conversion(entry: &'static str) -> Option<ConversionKind> {
        match entry {
            "-" => Some(ConversionKind::Default),
            "inv" => None,
            "zext" => Some(ConversionKind::Default),
            "chop" => Some(ConversionKind::Default),
            "sext" => Some(ConversionKind::SignExtend),
            _ => unreachable!(),
        }
    }

    fn parse_conversion_table(table: &'static str) -> Vec<Vec<Option<ConversionKind>>> {
        table
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .skip(1)
                    .map(table_entry_to_conversion)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn assert_conversion_table<F: Fn(&ast::Type, &ast::Type) -> Option<ConversionKind>>(
        table: &'static str,
        f: F,
    ) {
        let conv_table = parse_conversion_table(table);
        for (instr_idx, instr_type) in SCALAR_TYPES.iter().enumerate() {
            for (op_idx, op_type) in SCALAR_TYPES.iter().enumerate() {
                let conversion = f(
                    &ast::Type::Scalar(*op_type),
                    &ast::Type::Scalar(*instr_type),
                );
                if instr_idx == op_idx {
                    assert!(conversion == None);
                } else {
                    assert!(conversion == conv_table[instr_idx][op_idx]);
                }
            }
        }
    }

    #[test]
    fn should_convert_relaxed_src_all_combinations() {
        assert_conversion_table(RELAXED_SRC_CONVERSION_TABLE, should_convert_relaxed_src);
    }

    #[test]
    fn should_convert_relaxed_dst_all_combinations() {
        assert_conversion_table(RELAXED_DST_CONVERSION_TABLE, should_convert_relaxed_dst);
    }

    #[test]
    fn returns_correct_layout_for_array_params() {
        use crate::{ModuleParser, ModuleParserExt};

        let ptx = r#"
        .version 6.5
        .target sm_30
        .address_size 64
        
        .visible .entry kernel(.param .align 8 .b8 kernel_param[72])
        {
            ret;
        }"#;

        let ast = ModuleParser::parse_checked(ptx).unwrap();
        if let ast::Directive::Method(
            _,
            ast::Function {
                func_directive:
                    ast::MethodDeclaration {
                        input_arguments, ..
                    },
                ..
            },
        ) = &ast.directives[0]
        {
            assert_eq!(input_arguments.len(), 1);
            assert_eq!(input_arguments[0].layout(), unsafe {
                Layout::from_size_align_unchecked(72, 8)
            });
        } else {
            panic!()
        }
    }
}
