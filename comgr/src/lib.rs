use amd_comgr_sys::*;
use std::{ffi::CStr, iter, mem, ptr};

struct Data(amd_comgr_data_t);

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

    fn copy_content(&self) -> Result<Vec<u8>, amd_comgr_status_s> {
        let mut size = unsafe { mem::zeroed() };
        unsafe { amd_comgr_get_data(self.get(), &mut size, ptr::null_mut()) }?;
        let mut result: Vec<u8> = Vec::with_capacity(size);
        unsafe { result.set_len(size) };
        unsafe { amd_comgr_get_data(self.get(), &mut size, result.as_mut_ptr().cast()) }?;
        Ok(result)
    }
}

struct DataSet(amd_comgr_data_set_t);

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

    fn get_data(
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

pub fn compile_bitcode(
    gcn_arch: &CStr,
    main_buffer: &[u8],
    ptx_impl: &[u8],
) -> Result<Vec<u8>, amd_comgr_status_s> {
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
    let linked_data_set = do_action(
        &bitcode_data_set,
        &linking_info,
        amd_comgr_action_kind_t::AMD_COMGR_ACTION_LINK_BC_TO_BC,
    )?;
    let link_with_device_libs_info = ActionInfo::new()?;
    link_with_device_libs_info.set_isa_name(gcn_arch)?;
    link_with_device_libs_info.set_language(amd_comgr_language_t::AMD_COMGR_LANGUAGE_LLVM_IR)?;
    // This makes no sense, but it makes ockl linking work
    link_with_device_libs_info.set_options([c"-Xclang", c"-mno-link-builtin-bitcode-postopt"].into_iter())?;
    let with_device_libs = do_action(
        &linked_data_set,
        &link_with_device_libs_info,
        amd_comgr_action_kind_t::AMD_COMGR_ACTION_COMPILE_SOURCE_WITH_DEVICE_LIBS_TO_BC,
    )?;
    let compile_action_info = ActionInfo::new()?;
    compile_action_info.set_isa_name(gcn_arch)?;
    compile_action_info.set_options(iter::once(c"-O3"))?;
    let reloc_data_set = do_action(
        &with_device_libs,
        &compile_action_info,
        amd_comgr_action_kind_t::AMD_COMGR_ACTION_CODEGEN_BC_TO_RELOCATABLE,
    )?;
    let exec_data_set = do_action(
        &reloc_data_set,
        &compile_action_info,
        amd_comgr_action_kind_t::AMD_COMGR_ACTION_LINK_RELOCATABLE_TO_EXECUTABLE,
    )?;
    let executable =
        exec_data_set.get_data(amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_EXECUTABLE, 0)?;
    executable.copy_content()
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
