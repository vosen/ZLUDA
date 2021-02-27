//! Abstract link time optimization.
//!
//! ## ThinLTO
//!
//! ThinLTO is designed to do LTO while requiring fewer resources than regular
//! LTO. It can run much faster and in less memory (comparable to linking
//! without LTO) than regular LTO, with essentially no loss in optimization.

#![allow(non_camel_case_types)]

pub type lto_bool_t = u8;

// This looks kind of like bitflags but I'm not sure.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum lto_symbol_attributes {
    LTO_SYMBOL_ALIGNMENT_MASK = 31,
    LTO_SYMBOL_PERMISSIONS_MASK = 224,
    LTO_SYMBOL_PERMISSIONS_CODE = 160,
    LTO_SYMBOL_PERMISSIONS_DATA = 192,
    LTO_SYMBOL_PERMISSIONS_RODATA = 128,
    LTO_SYMBOL_DEFINITION_MASK = 1792,
    LTO_SYMBOL_DEFINITION_REGULAR = 256,
    LTO_SYMBOL_DEFINITION_TENTATIVE = 512,
    LTO_SYMBOL_DEFINITION_WEAK = 768,
    LTO_SYMBOL_DEFINITION_UNDEFINED = 1024,
    LTO_SYMBOL_DEFINITION_WEAKUNDEF = 1280,
    LTO_SYMBOL_SCOPE_MASK = 14336,
    LTO_SYMBOL_SCOPE_INTERNAL = 2048,
    LTO_SYMBOL_SCOPE_HIDDEN = 0x1000,
    LTO_SYMBOL_SCOPE_PROTECTED = 0x2000,
    LTO_SYMBOL_SCOPE_DEFAULT = 0x1800,
    LTO_SYMBOL_SCOPE_DEFAULT_CAN_BE_HIDDEN = 0x2800,
    /// Added in LLVM 3.7.
    LTO_SYMBOL_COMDAT = 0x4000,
    /// Added in LLVM 3.7.
    LTO_SYMBOL_ALIAS = 0x8000,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum lto_debug_model {
    LTO_DEBUG_MODEL_NONE = 0,
    LTO_DEBUG_MODEL_DWARF = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum lto_codegen_model {
    LTO_CODEGEN_PIC_MODEL_STATIC = 0,
    LTO_CODEGEN_PIC_MODEL_DYNAMIC = 1,
    LTO_CODEGEN_PIC_MODEL_DYNAMIC_NO_PIC = 2,
    LTO_CODEGEN_PIC_MODEL_DEFAULT = 3,
}

#[derive(Debug)]
pub enum LLVMOpaqueLTOModule {}

pub type lto_module_t = *mut LLVMOpaqueLTOModule;

#[derive(Debug)]
pub enum LLVMOpaqueLTOCodeGenerator {}

pub type lto_code_gen_t = *mut LLVMOpaqueLTOCodeGenerator;

#[derive(Debug)]
pub enum LLVMOpaqueThinLTOCodeGenerator {}

pub type thinlto_code_gen_t = *mut LLVMOpaqueThinLTOCodeGenerator;

#[derive(Debug)]
pub enum LLVMOpaqueLTOInput {}

pub type lto_input_t = *mut LLVMOpaqueLTOInput;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum lto_codegen_diagnostic_severity_t {
    LTO_DS_ERROR = 0,
    LTO_DS_WARNING = 1,
    LTO_DS_REMARK = 3,
    LTO_DS_NOTE = 2,
}

pub type lto_diagnostic_handler_t = Option<
    extern "C" fn(
        severity: lto_codegen_diagnostic_severity_t,
        diag: *const ::libc::c_char,
        ctxt: *mut ::libc::c_void,
    ),
>;

extern "C" {
    pub fn lto_get_version() -> *const ::libc::c_char;
    pub fn lto_get_error_message() -> *const ::libc::c_char;
    pub fn lto_module_is_object_file(path: *const ::libc::c_char) -> lto_bool_t;
    pub fn lto_module_is_object_file_for_target(
        path: *const ::libc::c_char,
        target_triple_prefix: *const ::libc::c_char,
    ) -> lto_bool_t;
    /// Return true if `Buffer` contains a bitcode file with ObjC code
    /// (category or class) in it.
    pub fn lto_module_has_objc_category(
        mem: *const ::libc::c_void,
        length: ::libc::size_t,
    ) -> lto_bool_t;
    /// Checks if a buffer is a loadable object file.
    pub fn lto_module_is_object_file_in_memory(
        mem: *const ::libc::c_void,
        length: ::libc::size_t,
    ) -> lto_bool_t;
    pub fn lto_module_is_object_file_in_memory_for_target(
        mem: *const ::libc::c_void,
        length: ::libc::size_t,
        target_triple_prefix: *const ::libc::c_char,
    ) -> lto_bool_t;
    pub fn lto_module_create(path: *const ::libc::c_char) -> lto_module_t;
    pub fn lto_module_create_from_memory(
        mem: *const ::libc::c_void,
        length: ::libc::size_t,
    ) -> lto_module_t;
    pub fn lto_module_create_from_memory_with_path(
        mem: *const ::libc::c_void,
        length: ::libc::size_t,
        path: *const ::libc::c_char,
    ) -> lto_module_t;
    pub fn lto_module_create_in_local_context(
        mem: *const ::libc::c_void,
        length: ::libc::size_t,
        path: *const ::libc::c_char,
    ) -> lto_module_t;
    pub fn lto_module_create_in_codegen_context(
        mem: *const ::libc::c_void,
        length: ::libc::size_t,
        path: *const ::libc::c_char,
        cg: lto_code_gen_t,
    ) -> lto_module_t;
    pub fn lto_module_create_from_fd(
        fd: ::libc::c_int,
        path: *const ::libc::c_char,
        file_size: ::libc::size_t,
    ) -> lto_module_t;
    pub fn lto_module_create_from_fd_at_offset(
        fd: ::libc::c_int,
        path: *const ::libc::c_char,
        file_size: ::libc::size_t,
        map_size: ::libc::size_t,
        offset: ::libc::off_t,
    ) -> lto_module_t;
    pub fn lto_module_dispose(_mod: lto_module_t);
    pub fn lto_module_get_target_triple(_mod: lto_module_t) -> *const ::libc::c_char;
    pub fn lto_module_set_target_triple(_mod: lto_module_t, triple: *const ::libc::c_char);
    pub fn lto_module_get_num_symbols(_mod: lto_module_t) -> ::libc::c_uint;
    pub fn lto_module_get_symbol_name(
        _mod: lto_module_t,
        index: ::libc::c_uint,
    ) -> *const ::libc::c_char;
    pub fn lto_module_get_symbol_attribute(
        _mod: lto_module_t,
        index: ::libc::c_uint,
    ) -> lto_symbol_attributes;
    /// Returns the module's linker options.
    ///
    /// The linker options may consist of multiple flags. It is the linker's
    /// responsibility to split the flags using a platform-specific mechanism.
    ///
    /// Added in LLVM 3.7.
    pub fn lto_module_get_linkeropts(_mod: lto_module_t) -> *const ::libc::c_char;
    pub fn lto_module_get_macho_cputype(
        _mod: lto_module_t,
        out_cputype: *mut ::libc::c_uint,
        out_cpusubtype: *mut ::libc::c_uint,
    ) -> lto_bool_t;
    /// Return true if the module has either the `@llvm.global_ctors` or the `@llvm.global_dtors`
    /// symbol.
    ///
    /// Added in API version 29 (LLVM 14).
    pub fn lto_module_has_ctor_dtor(mod_: lto_module_t) -> lto_bool_t;
    pub fn lto_codegen_set_diagnostic_handler(
        arg1: lto_code_gen_t,
        arg2: lto_diagnostic_handler_t,
        arg3: *mut ::libc::c_void,
    );
    pub fn lto_codegen_create() -> lto_code_gen_t;
    pub fn lto_codegen_create_in_local_context() -> lto_code_gen_t;
    pub fn lto_codegen_dispose(arg1: lto_code_gen_t);
    pub fn lto_codegen_add_module(cg: lto_code_gen_t, _mod: lto_module_t) -> lto_bool_t;
    /// Sets the object module for code gneeration. This will transfer ownership
    /// of the module to the code generator.
    ///
    /// Added in LLVM 3.7.
    pub fn lto_codegen_set_module(cg: lto_code_gen_t, _mod: lto_module_t);
    pub fn lto_codegen_set_debug_model(cg: lto_code_gen_t, arg1: lto_debug_model) -> lto_bool_t;
    pub fn lto_codegen_set_pic_model(cg: lto_code_gen_t, arg1: lto_codegen_model) -> lto_bool_t;
    pub fn lto_codegen_set_cpu(cg: lto_code_gen_t, cpu: *const ::libc::c_char);
    pub fn lto_codegen_set_assembler_path(cg: lto_code_gen_t, path: *const ::libc::c_char);
    pub fn lto_codegen_set_assembler_args(
        cg: lto_code_gen_t,
        args: *mut *const ::libc::c_char,
        nargs: ::libc::c_int,
    );
    pub fn lto_codegen_add_must_preserve_symbol(cg: lto_code_gen_t, symbol: *const ::libc::c_char);
    pub fn lto_codegen_write_merged_modules(
        cg: lto_code_gen_t,
        path: *const ::libc::c_char,
    ) -> lto_bool_t;
    pub fn lto_codegen_compile(
        cg: lto_code_gen_t,
        length: *mut ::libc::size_t,
    ) -> *const ::libc::c_void;
    pub fn lto_codegen_compile_to_file(
        cg: lto_code_gen_t,
        name: *mut *const ::libc::c_char,
    ) -> lto_bool_t;
    /// Runs optimization for the merged module.
    ///
    /// Returns true on error.
    ///
    /// Added in LLVM 3.7.
    pub fn lto_codegen_optimize(cg: lto_code_gen_t) -> lto_bool_t;
    /// Generates code for the optimized merged module into one native object file.
    ///
    /// Does not run IR optimizations on the merged module.
    ///
    /// Returns a pointer to the generated mach-o/ELF buffer with length
    /// set to the buffer size. This buffer is owned by `cg` and will be
    /// freed when `lto_codegen_dispose` is called or `lto_codegen_compile_optimized`
    /// is called again. Returns null on failure.
    ///
    /// Added in LLVM 3.7.
    pub fn lto_codegen_compile_optimized(
        cg: lto_code_gen_t,
        length: *mut ::libc::size_t,
    ) -> *mut ::libc::c_void;
    /// Returns the runtime API version.
    ///
    /// Added in LLVM 3.7.
    pub fn lto_api_version() -> ::libc::c_uint;
    pub fn lto_set_debug_options(options: *mut *const ::libc::c_char, number: ::libc::c_int);
    pub fn lto_codegen_debug_options(cg: lto_code_gen_t, arg1: *const ::libc::c_char);
    pub fn lto_codegen_debug_options_array(
        cg: lto_code_gen_t,
        arg2: *const *const ::libc::c_char,
        number: ::libc::c_int,
    );
    pub fn lto_initialize_disassembler();
    /// Sets if we should run the internalize pass during optimization and code generation.
    ///
    /// Added in LLVM 3.7.
    pub fn lto_codegen_set_should_internalize(cg: lto_code_gen_t, ShouldInternalize: lto_bool_t);
    /// Set whether to embed uselists in bitcode.
    ///
    /// Sets whether `lto_codegen_write_merged_modules` should embed uselists in
    /// output bitcode. This should be turned on for all -save-temps output.
    ///
    /// Added in LLVM 3.7.
    pub fn lto_codegen_set_should_embed_uselists(
        cg: lto_code_gen_t,
        ShouldEmbedUselists: lto_bool_t,
    );
}

/// Type to wrap a single object returned by ThinLTO.
#[repr(C)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct LTOObjectBuffer {
    Buffer: *const ::libc::c_char,
    Size: ::libc::size_t,
}

extern "C" {
    /// Instantiates a ThinLTO code generator.
    ///
    /// Returns null on error (check `lto_get_error_message` for details).
    ///
    /// The code generator should not be reused.
    pub fn thinlto_create_codegen() -> thinlto_code_gen_t;
    /// Frees a code generator.
    pub fn thinlto_codegen_dispose(cg: thinlto_code_gen_t);
    /// Add a module to a code generator.
    ///
    /// Identifier must be unique among all the modules in the code generator.
    /// The data buffer remains owned by the client, and must live at least
    /// as long as the code generator.
    ///
    /// Returns null on failure.
    pub fn thinlto_codegen_add_module(
        cg: thinlto_code_gen_t,
        identifier: *const ::libc::c_char,
        data: *const ::libc::c_char,
        length: ::libc::c_int,
    );
    /// Optimize and codegen all modules attached to the code generator.
    ///
    /// Resulting objects are accessible with `thinlto_module_get_object`.
    pub fn thinlto_codegen_process(cg: thinlto_code_gen_t);
    /// Return the number of object files produced by the code generator.
    ///
    /// This usually matches the number of input files, but is not guaranteed
    /// to.
    pub fn thinlto_module_get_num_objects(cg: thinlto_code_gen_t) -> ::libc::c_int;
    /// Return a reference to the `index`th object file produced by the
    /// code generator.
    pub fn thinlto_module_get_object(
        cg: thinlto_code_gen_t,
        index: ::libc::c_uint,
    ) -> LTOObjectBuffer;
    /// Return the number of object files produced by the code generator.
    ///
    /// Usually the same as the number of input files, but not guaranteed.
    pub fn thinlto_module_get_num_object_files(cg: thinlto_code_gen_t) -> ::libc::c_uint;
    /// Return the path to the ith output object file.
    ///
    /// Use `thinlto_module_get_num_object_files` to get the number of available objects.
    pub fn thinlto_module_get_object_file(
        cg: thinlto_code_gen_t,
        index: ::libc::c_uint,
    ) -> *const ::libc::c_char;
    /// Set which PIC code model to generate.
    ///
    /// Returns true on error.
    pub fn thinlto_codegen_set_pic_model(
        cg: thinlto_code_gen_t,
        model: lto_codegen_model,
    ) -> lto_bool_t;

    // ThinLTO cache control.
    // Set the path to a directory to use as cache for increment build.
    //
    // Setting this activates caching.
    pub fn thinlto_codegen_set_cache_dir(cg: thinlto_code_gen_t, cache_dir: *const ::libc::c_char);
    /// Set the cache pruning interval, in seconds.
    ///
    /// A negative value disables pruning, and 0 will force pruning to occur.
    pub fn thinlto_codegen_set_cache_pruning_interval(
        cg: thinlto_code_gen_t,
        interval: ::libc::c_int,
    );
    /// Set the maximum cache size to persist across builds.
    ///
    /// This is expressed as a percentage of available disk space. 100 means no limit,
    /// and 50 means no more than half of the available disk space. 0 is ignored, and
    /// values over 100 will be reduced to 100.
    pub fn thinlto_codegen_set_final_cache_size_relative_to_available_space(
        cg: thinlto_code_gen_t,
        percentage: ::libc::c_uint,
    );
    /// Set the expiration (in seconds) for cache entries.
    pub fn thinlto_codegen_set_cache_entry_expiration(
        cg: thinlto_code_gen_t,
        expiration: ::libc::c_uint,
    );
    /// Set the maximum size of the cache directory (in bytes). A value over the
    /// amount of available space on the disk will be reduced to the amount of
    /// available space. An unspecified default value will be applied. A value of 0
    /// will be ignored.
    pub fn thinlto_codegen_set_cache_size_bytes(
        cg: thinlto_code_gen_t,
        max_size_bytes: ::libc::c_uint,
    );
    /// Same as thinlto_codegen_set_cache_size_bytes, except the maximum size is in
    /// megabytes (2^20 bytes).
    pub fn thinlto_codegen_set_cache_size_megabytes(
        cg: thinlto_code_gen_t,
        max_size_megabytes: ::libc::c_uint,
    );
    /// Sets the maximum number of files in the cache directory. An unspecified default value will be applied. A value of 0 will be ignored.
    pub fn thinlto_codegen_set_cache_size_files(
        cg: thinlto_code_gen_t,
        max_size_files: ::libc::c_uint,
    );

    /// Create an LTO input file from a buffer.
    pub fn lto_input_create(
        buffer: *const ::libc::c_void,
        buffer_size: ::libc::size_t,
        path: *const ::libc::c_char,
    ) -> lto_input_t;
    /// Free all memory allocated by the input file.
    pub fn lto_input_dispose(input: lto_input_t);
    /// Get the number of dependent library specifiers for the given input.
    pub fn lto_input_get_num_dependent_libraries(input: lto_input_t) -> ::libc::c_uint;
    /// Get the `i`th dependent library specifier for the given input file.
    ///
    /// The returned string is not null-terminated.
    pub fn lto_input_get_dependent_library(
        input: lto_input_t,
        index: ::libc::size_t,
        size: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    /// Return the list of libcall symbols that can be generated by LTO
    /// that might not be visible from the symbol table of bitcode files.
    pub fn lto_runtime_lib_symbols_list(size: *mut usize) -> *const *const ::libc::c_char;

    /// Set the path to a directory to use as temporary bitcode storage.
    ///
    /// This is meant to make the bitcode files available for debugging.
    pub fn thinlto_codegen_set_savetemps_dir(
        cg: thinlto_code_gen_t,
        save_temps_dir: *const ::libc::c_char,
    );
    /// Set the path to a directory to save generated object files.
    ///
    /// Set this to request on-disk rather than in-memory buffers. When set, use
    /// `thinlto_module_get_object_file` instead of `thinlto_module_get_object`.
    pub fn thinlto_set_generated_objects_dir(
        cg: thinlto_code_gen_t,
        save_temps_dir: *const ::libc::c_char,
    );
    /// Set the CPU to generate code for.
    pub fn thinlto_codegen_set_cpu(cg: thinlto_code_gen_t, cpu: *const ::libc::c_char);
    /// Disable code generation (running all stages until codegen).
    ///
    /// The output with codegen disabled is bitcode.
    pub fn thinlto_codegen_disable_codegen(cg: thinlto_code_gen_t, disable: lto_bool_t);
    /// Perform codegen only; disable all other stages.
    pub fn thinlto_codegen_set_codegen_only(cg: thinlto_code_gen_t, codegen_only: lto_bool_t);
    /// Parse -mllvm style debug options.
    pub fn thinlto_debug_options(options: *const *const ::libc::c_char, number: ::libc::c_int);
    /// Test if a module has ThinLTO linking support.
    pub fn lto_module_is_thinlto(module: lto_module_t) -> lto_bool_t;
    /// Add a symbol to the list of global symbols that must exist in the
    /// final generated code.
    ///
    /// Functions not listed may be inlined in every usage and optimized away.
    pub fn thinlto_codegen_add_must_preserve_symbol(
        cg: thinlto_code_gen_t,
        name: *const ::libc::c_char,
        length: ::libc::c_int,
    );
    /// Add a symbol to the list of global symbols that are cross-referenced
    /// between ThinLTO files.
    ///
    /// Symbols listed can be discarded if every reference from a ThinLTO module
    /// to a symbol is optimized away, then the symbol can be discarded.
    pub fn thinlto_codegen_add_cross_referenced_symbol(
        cg: thinlto_code_gen_t,
        name: *const ::libc::c_char,
        length: ::libc::c_int,
    );
}
