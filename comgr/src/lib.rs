use amd_comgr_sys::*;
use std::ffi::CStr;
use std::mem;
use std::ptr;

pub struct Data(amd_comgr_data_t);

impl Data {
    fn new(
        kind: amd_comgr_data_kind_t,
        name: &CStr,
        content: &[u8],
    ) -> Result<Self, amd_comgr_status_s> {
        let mut data = unsafe { mem::zeroed() };
        unsafe { amd_comgr_create_data(kind, &mut data) }?;
        unsafe { amd_comgr_set_data_name(data, name.as_ptr()) }?;
        unsafe { amd_comgr_set_data(data, content.len(), content.as_ptr().cast()) }?;
        Ok(Self(data))
    }

    fn get(&self) -> amd_comgr_data_t {
        self.0
    }

    pub fn copy_content(&self) -> Result<Vec<u8>, amd_comgr_status_s> {
        let mut size = unsafe { mem::zeroed() };
        unsafe { amd_comgr_get_data(self.get(), &mut size, ptr::null_mut()) }?;
        let mut result: Vec<u8> = Vec::with_capacity(size);
        unsafe { result.set_len(size) };
        unsafe { amd_comgr_get_data(self.get(), &mut size, result.as_mut_ptr().cast()) }?;
        Ok(result)
    }
}

pub struct DataSet(amd_comgr_data_set_t);

impl DataSet {
    fn new() -> Result<Self, amd_comgr_status_s> {
        let mut data_set = unsafe { mem::zeroed() };
        unsafe { amd_comgr_create_data_set(&mut data_set) }?;
        Ok(Self(data_set))
    }

    fn add(&self, data: &Data) -> Result<(), amd_comgr_status_s> {
        unsafe { amd_comgr_data_set_add(self.get(), data.get()) }
    }

    fn get(&self) -> amd_comgr_data_set_t {
        self.0
    }

    pub fn get_data(
        &self,
        kind: amd_comgr_data_kind_t,
        index: usize,
    ) -> Result<Data, amd_comgr_status_s> {
        let mut data = unsafe { mem::zeroed() };
        unsafe { amd_comgr_action_data_get_data(self.get(), kind, index, &mut data) }?;
        Ok(Data(data))
    }
}

impl Drop for DataSet {
    fn drop(&mut self) {
        unsafe { amd_comgr_destroy_data_set(self.get()).ok() };
    }
}

struct ActionInfo(amd_comgr_action_info_t);

impl ActionInfo {
    fn new() -> Result<Self, amd_comgr_status_s> {
        let mut action = unsafe { mem::zeroed() };
        unsafe { amd_comgr_create_action_info(&mut action) }?;
        Ok(Self(action))
    }

    fn set_isa_name(&self, isa: &CStr) -> Result<(), amd_comgr_status_s> {
        let mut full_isa = "amdgcn-amd-amdhsa--".to_string().into_bytes();
        full_isa.extend(isa.to_bytes_with_nul());
        unsafe { amd_comgr_action_info_set_isa_name(self.get(), full_isa.as_ptr().cast()) }
    }

    fn set_language(&self, language: amd_comgr_language_t) -> Result<(), amd_comgr_status_s> {
        unsafe { amd_comgr_action_info_set_language(self.get(), language) }
    }

    fn set_options<'a>(
        &self,
        options: impl Iterator<Item = &'a CStr>,
    ) -> Result<(), amd_comgr_status_s> {
        let options = options.map(|x| x.as_ptr()).collect::<Vec<_>>();
        unsafe {
            amd_comgr_action_info_set_option_list(
                self.get(),
                options.as_ptr().cast_mut(),
                options.len(),
            )
        }
    }

    fn get(&self) -> amd_comgr_action_info_t {
        self.0
    }
}

impl Drop for ActionInfo {
    fn drop(&mut self) {
        unsafe { amd_comgr_destroy_action_info(self.get()).ok() };
    }
}

fn link_bitcode(main_buffer: &[u8], ptx_impl: &[u8]) -> Result<DataSet, amd_comgr_status_s> {
    use amd_comgr_sys::*;
    let bitcode_data_set = DataSet::new()?;
    let main_bitcode_data = Data::new(
        amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_BC,
        c"zluda.bc",
        main_buffer,
    )?;
    bitcode_data_set.add(&main_bitcode_data)?;
    let stdlib_bitcode_data = Data::new(
        amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_BC,
        c"ptx_impl.bc",
        ptx_impl,
    )?;
    bitcode_data_set.add(&stdlib_bitcode_data)?;
    let linking_info = ActionInfo::new()?;
    do_action(
        &bitcode_data_set,
        &linking_info,
        amd_comgr_action_kind_t::AMD_COMGR_ACTION_LINK_BC_TO_BC,
    )
}

fn compile_bitcode(
    linked_data_set: &DataSet,
    gcn_arch: &CStr,
) -> Result<DataSet, amd_comgr_status_s> {
    use amd_comgr_sys::*;

    let compile_to_exec = ActionInfo::new()?;
    compile_to_exec.set_isa_name(gcn_arch)?;
    compile_to_exec.set_language(amd_comgr_language_t::AMD_COMGR_LANGUAGE_LLVM_IR)?;
    let common_options = [
        // This makes no sense, but it makes ockl linking work
        c"-Xclang",
        c"-mno-link-builtin-bitcode-postopt",
        // Otherwise LLVM omits dynamic fp mode for ockl functions during linking
        // and then fails to inline them
        c"-Xclang",
        c"-fdenormal-fp-math=dynamic",
        c"-O3",
        c"-mno-wavefrontsize64",
        c"-mcumode",
        // Useful for inlining reports, combined with AMD_COMGR_SAVE_TEMPS=1 AMD_COMGR_EMIT_VERBOSE_LOGS=1 AMD_COMGR_REDIRECT_LOGS=stderr
        // c"-fsave-optimization-record=yaml",
    ]
    .into_iter();
    let opt_options = if cfg!(debug_assertions) {
        //[c"-g", c"-mllvm", c"-print-before-all", c"", c""]
        [c"-g", c"", c"", c"", c""]
    } else {
        [
            c"-g0",
            // default inlining threshold times 10
            c"-mllvm",
            c"-inline-threshold=2250",
            c"-mllvm",
            c"-inlinehint-threshold=3250",
        ]
    };
    compile_to_exec.set_options(common_options.chain(opt_options))?;
    do_action(
        &linked_data_set,
        &compile_to_exec,
        amd_comgr_action_kind_t::AMD_COMGR_ACTION_COMPILE_SOURCE_TO_EXECUTABLE,
    )
}

fn disassemble_exec(
    exec_data_set: &DataSet,
    gcn_arch: &CStr,
) -> Result<DataSet, amd_comgr_status_s> {
    let action_info = ActionInfo::new()?;
    action_info.set_isa_name(gcn_arch)?;
    do_action(
        &exec_data_set,
        &action_info,
        amd_comgr_action_kind_t::AMD_COMGR_ACTION_DISASSEMBLE_EXECUTABLE_TO_SOURCE,
    )
}

pub fn get_linked_bitcode_as_bytes(
    main_buffer: &[u8],
    ptx_impl: &[u8],
) -> Result<Vec<u8>, amd_comgr_status_s> {
    let linked_data_set = link_bitcode(main_buffer, ptx_impl)?;
    let linked_bitcode =
        linked_data_set.get_data(amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_BC, 0)?;
    linked_bitcode.copy_content()
}

fn get_executable(
    gcn_arch: &CStr,
    main_buffer: &[u8],
    ptx_impl: &[u8],
) -> Result<DataSet, amd_comgr_status_s> {
    let linked_data_set = link_bitcode(main_buffer, ptx_impl)?;
    let exec_data_set = compile_bitcode(&linked_data_set, gcn_arch)?;
    Ok(exec_data_set)
}

pub fn get_executable_as_bytes(
    gcn_arch: &CStr,
    main_buffer: &[u8],
    ptx_impl: &[u8],
) -> Result<Vec<u8>, amd_comgr_status_s> {
    let exec_data_set = get_executable(gcn_arch, main_buffer, ptx_impl)?;
    let executable =
        exec_data_set.get_data(amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_EXECUTABLE, 0)?;
    executable.copy_content()
}

pub fn get_assembly_as_bytes(
    gcn_arch: &CStr,
    main_buffer: &[u8],
    ptx_impl: &[u8],
) -> Result<Vec<u8>, amd_comgr_status_s> {
    let exec_data_set = get_executable(gcn_arch, main_buffer, ptx_impl)?;
    let disassembled_data_set = disassemble_exec(&exec_data_set, gcn_arch)?;
    let assembly =
        disassembled_data_set.get_data(amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_SOURCE, 0)?;
    assembly.copy_content()
}

fn do_action(
    data_set: &DataSet,
    action: &ActionInfo,
    kind: amd_comgr_action_kind_t,
) -> Result<DataSet, amd_comgr_status_s> {
    let result = DataSet::new()?;
    unsafe { amd_comgr_do_action(kind, action.get(), data_set.get(), result.get()) }?;
    Ok(result)
}
