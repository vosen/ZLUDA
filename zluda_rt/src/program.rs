use crate::context::ContextData;
use crate::geometry::GeometryData;
use crate::geometry_instance::GeometryInstanceData;
use crate::geometry_triangles::GeometryTrianglesData;
use crate::material::MaterialData;
use crate::repr_gpu::TrivialHIPAllocator;
use crate::{
    context::Context,
    null_check, null_unwrap, null_unwrap_mut,
    variable::{Variable, VariableData},
    OptixCell, OptixObjectData, TypeTag,
};
use crate::{div_positive_round_up, eptx, hip, hiprt, repr_gpu, AlignedBuffer, MaybeWeakRefMut};
use comgr::Comgr;
use hip_common::raytracing::VariablesBlock;
use hip_runtime_sys::*;
use hiprt_sys::*;
use optix_types::*;
use ptx::{llvm, raytracing, ModuleParserExt};
use rustc_hash::FxHashMap;
use std::alloc::Layout;
use std::ffi::c_void;
use std::mem::{self, ManuallyDrop};
use std::rc::Weak;
use std::{
    ffi::{CStr, CString},
    rc::Rc,
};
use std::{iter, ptr};

pub(crate) type Program = *const OptixCell<ProgramData>;

pub(crate) struct ProgramData {
    pub(crate) context: Weak<OptixCell<ContextData>>,
    pub(crate) variables: FxHashMap<CString, Rc<OptixCell<VariableData>>>,
    pub(crate) variables_block: VariablesBlock,
    pub(crate) callable_index: Option<u32>,
    // this field is shared between instances created by cloning existing program object
    pub(crate) shared: Rc<ProgramShared>,
}

pub(crate) struct ProgramShared {
    pub(crate) binary: Vec<u8>,
    pub(crate) module: hip::Module,
}

impl OptixObjectData for ProgramData {
    const TYPE: TypeTag = TypeTag::Program;

    fn deregister(&mut self, this: &Rc<OptixCell<Self>>) -> Result<(), RTresult> {
        if let Some(context) = self.context.upgrade() {
            let mut context = (*context).borrow_mut()?;
            context.programs.remove(this);
        }
        Ok(())
    }

    fn context<'a>(&'a mut self) -> crate::MaybeWeakRefMut<'a, ContextData> {
        MaybeWeakRefMut::Weak(&self.context)
    }
}

impl ProgramData {
    pub(crate) const KERNEL_BOUNDING_BOX_NAME: &'static CStr =
        raytracing::Module::KERNEL_BOUNDING_BOX_NAME;
    pub(crate) const KERNEL_NAME: &'static CStr = raytracing::Module::KERNEL_NAME;
    pub(crate) const ATTRIBUTE_FUNCTION_POINTER_NAME: &'static CStr =
        raytracing::Module::ATTRIBUTE_FUNCTION_POINTER_NAME;
    pub(crate) const FUNCTION_POINTER_NAME: &'static CStr =
        raytracing::Module::FUNCTION_POINTER_NAME;

    pub(crate) fn get_program_block_layout(&self) -> Result<Layout, RTresult> {
        Ok(self.variables_block.layout)
    }

    pub(crate) fn try_from_binary(
        context: Weak<OptixCell<ContextData>>,
        binary: Vec<u8>,
    ) -> Option<(Self, VariablesBlock)> {
        let zluda_rt6_section = hip_common::kernel_metadata::get_section(
            hip_common::kernel_metadata::zluda_rt6::SECTION_STR,
            &*binary,
        )?;
        let zluda_rt6_metadata =
            hip_common::kernel_metadata::zluda_rt6::read(&zluda_rt6_section).ok()?;
        let module = hip::Module::load_data(&binary).ok()?;
        Some((
            ProgramData {
                context,
                variables: FxHashMap::default(),
                variables_block: zluda_rt6_metadata.variables,
                callable_index: if zluda_rt6_metadata.is_callable {
                    Some(0)
                } else {
                    None
                },
                shared: Rc::new(ProgramShared { binary, module }),
            },
            zluda_rt6_metadata.attribute_variables,
        ))
    }

    pub(crate) fn copy_program_block<'a>(
        &'a self,
        dst_buffer: &mut [u8],
        get_variable: impl Fn(&CStr) -> Option<&'a Rc<OptixCell<VariableData>>>,
    ) -> Result<(), RTresult> {
        self.copy_program_block_impl(false, dst_buffer, get_variable)
    }

    pub(crate) fn copy_attribute_program_block<'a>(
        &'a self,
        dst_buffer: &mut [u8],
        get_variable: impl Fn(&CStr) -> Option<&'a Rc<OptixCell<VariableData>>>,
    ) -> Result<(), RTresult> {
        self.copy_program_block_impl(true, dst_buffer, get_variable)
    }

    fn copy_program_block_impl<'a>(
        &'a self,
        is_attribute: bool,
        dst_buffer: &mut [u8],
        get_variable: impl Fn(&CStr) -> Option<&'a Rc<OptixCell<VariableData>>>,
    ) -> Result<(), RTresult> {
        let fn_ptr = if is_attribute {
            self.get_attribute_function()
        } else {
            self.get_function()
        }?;
        dst_buffer[..mem::size_of::<hipDeviceptr_t>()]
            .copy_from_slice(&(fn_ptr.0 as usize).to_ne_bytes());
        Self::copy_variable_block(&self.variables_block, dst_buffer, get_variable)
    }

    pub(crate) fn copy_variable_block<'a>(
        variables_block: &VariablesBlock,
        dst_buffer: &mut [u8],
        get_variable: impl Fn(&CStr) -> Option<&'a Rc<OptixCell<VariableData>>>,
    ) -> Result<(), RTresult> {
        for (name, var_details) in variables_block.variables.iter() {
            match get_variable(name) {
                Some(variable) => {
                    let variable = variable.borrow()?;
                    let variable_offset = var_details.offset as usize;
                    variable.copy_into_buffer(
                        &mut dst_buffer
                            [variable_offset..variable_offset + var_details.size as usize],
                    )?;
                }
                None => {
                    if var_details.default_value.len() == 0 {
                        continue;
                    }
                    if var_details.default_value.len() != var_details.size as usize {
                        return Err(RTresult::RT_ERROR_UNKNOWN);
                    }
                    let variable_offset = var_details.offset as usize;
                    dst_buffer[variable_offset..variable_offset + var_details.size as usize]
                        .copy_from_slice(&*var_details.default_value);
                }
            };
        }
        Ok(())
    }

    pub(crate) fn launch_2d(
        &self,
        width: u32,
        height: u32,
        globals: &repr_gpu::GlobalState,
        mut stack: hipDeviceptr_t,
        mut variable_block: hipDeviceptr_t,
        (mut exception, mut exception_var_block): (hipDeviceptr_t, hipDeviceptr_t),
    ) -> Result<(), RTresult> {
        let function = self
            .shared
            .module
            .get_function(Self::KERNEL_NAME)
            .map_err(|_| RTresult::RT_ERROR_INVALID_CONTEXT)?;
        let mut globals = globals.clone();
        globals.width = width;
        globals.height = height;
        let (grid_dim_x, block_dim_x) = get_launch_dimensions_x(width)?;
        let mut params = [
            &mut globals as *mut repr_gpu::GlobalState as *mut c_void,
            &mut stack as *mut _ as *mut c_void,
            &mut variable_block as *mut _ as *mut c_void,
            &mut exception as *mut _ as *mut c_void,
            &mut exception_var_block as *mut _ as *mut c_void,
        ];
        //println!("enter");
        //let mut unused = String::new();
        //std::io::stdin().read_line(&mut unused).unwrap();
        hip! {
            hipModuleLaunchKernel(
                function,
                grid_dim_x,
                height as u32,
                1,
                block_dim_x,
                1,
                1,
                0,
                ptr::null_mut(),
                params.as_mut_ptr() as _,
                ptr::null_mut()
            ),
            RT_ERROR_UNKNOWN
        };
        hip! { hipStreamSynchronize(ptr::null_mut()), RT_ERROR_UNKNOWN };
        Ok(())
    }

    pub(crate) fn get_variable_for_kernel<'a>(
        &'a self,
        context: &'a ContextData,
        name: &CStr,
    ) -> Option<&Rc<OptixCell<VariableData>>> {
        self.variables
            .get(name)
            .or_else(|| context.variables.get(name))
    }

    pub(crate) fn get_variable_for_function<'a>(
        &'a self,
        geometry_instance: &'a GeometryInstanceData,
        material: &'a MaterialData,
        context: &'a ContextData,
        name: &CStr,
    ) -> Option<&Rc<OptixCell<VariableData>>> {
        self.variables
            .get(name)
            .or_else(|| geometry_instance.variables.get(name))
            .or_else(|| material.variables.get(name))
            .or_else(|| context.variables.get(name))
    }

    pub(crate) fn get_variable_for_attribute<'a>(
        &'a self,
        geometry_triangles: &'a GeometryTrianglesData,
        geometry_instance: &'a GeometryInstanceData,
        context: &'a ContextData,
        name: &CStr,
    ) -> Option<&Rc<OptixCell<VariableData>>> {
        self.variables
            .get(name)
            .or_else(|| geometry_triangles.variables.get(name))
            .or_else(|| geometry_instance.variables.get(name))
            .or_else(|| context.variables.get(name))
    }

    pub(crate) fn get_variable_for_function_non_hit<'a>(
        &'a self,
        geometry_instance: &'a GeometryInstanceData,
        geometry: &'a GeometryData,
        context: &'a ContextData,
        name: &CStr,
    ) -> Option<&Rc<OptixCell<VariableData>>> {
        self.variables
            .get(name)
            .or_else(|| geometry_instance.variables.get(name))
            .or_else(|| geometry.variables.get(name))
            .or_else(|| context.variables.get(name))
    }

    pub(crate) fn prepare_variable_block_for_kernel(
        &self,
        allocator: &mut TrivialHIPAllocator,
        context: &ContextData,
    ) -> Result<hipDeviceptr_t, RTresult> {
        let mut staging_buffer = AlignedBuffer::new(self.variables_block.layout);
        Self::copy_variable_block(
            &self.variables_block,
            staging_buffer.as_bytes_mut(),
            |name| self.get_variable_for_kernel(&*context, name),
        )?;
        allocator.copy_to_device(staging_buffer.as_bytes())
    }

    pub(crate) fn prepare_variable_block_for_function_non_hit<'a>(
        &self,
        allocator: &mut TrivialHIPAllocator,
        geometry_instance: &'a GeometryInstanceData,
        geometry: &GeometryData,
        context: &'a ContextData,
    ) -> Result<hipDeviceptr_t, RTresult> {
        let mut staging_buffer = AlignedBuffer::new(self.variables_block.layout);
        ProgramData::copy_variable_block(
            &self.variables_block,
            staging_buffer.as_bytes_mut(),
            |name| {
                self.get_variable_for_function_non_hit(geometry_instance, geometry, &context, name)
            },
        )?;
        allocator.copy_to_device(staging_buffer.as_bytes())
    }

    pub fn get_attribute_function(&self) -> Result<hipDeviceptr_t, RTresult> {
        unsafe {
            self.shared
                .module
                .get_global(Self::ATTRIBUTE_FUNCTION_POINTER_NAME)
                .map_err(|_| RTresult::RT_ERROR_UNKNOWN)
        }
    }

    pub fn get_function(&self) -> Result<hipDeviceptr_t, RTresult> {
        unsafe {
            self.shared
                .module
                .get_global(Self::FUNCTION_POINTER_NAME)
                .map_err(|_| RTresult::RT_ERROR_UNKNOWN)
        }
    }

    unsafe fn create_from(
        &self,
        context_wrapper: &OptixCell<ContextData>,
        context: &mut ContextData,
    ) -> Result<Program, RTresult> {
        let callable_index = self.callable_index.map(|_| {
            context.callable_program_counter += 1;
            context.callable_program_counter
        });
        let context_wrapper = OptixCell::clone_weak(context_wrapper);
        let new_program = Rc::new(OptixCell::new(ProgramData {
            context: context_wrapper,
            variables: FxHashMap::default(),
            variables_block: self.variables_block.clone(),
            callable_index,
            shared: self.shared.clone(),
        }));
        let program_ptr = Rc::as_ptr(&new_program);
        context.programs.insert(new_program);
        Ok(program_ptr)
    }
}

pub(crate) fn get_launch_dimensions_x(width: u32) -> Result<(u32, u32), RTresult> {
    let block_size = 32;
    Ok((
        div_positive_round_up(width as u64, block_size as u64) as u32,
        block_size,
    ))
}

pub(crate) unsafe fn create_from_ptx_file(
    context: Context,
    filename: *const i8,
    program_name: *const i8,
    program: *mut Program,
) -> Result<(), RTresult> {
    null_check(filename)?;
    let path = CStr::from_ptr(filename)
        .to_str()
        .map_err(|_| RTresult::RT_ERROR_INVALID_VALUE)?;
    let file = std::fs::read(path).map_err(|_| RTresult::RT_ERROR_FILE_NOT_FOUND)?;
    create_from_ptx(context, PtxInput::Vec(file), program_name, program)
}

pub(crate) unsafe fn create_from_ptx_string(
    context: Context,
    ptx: *const i8,
    program_name: *const i8,
    program: *mut Program,
) -> Result<(), RTresult> {
    create_from_ptx(
        context,
        PtxInput::CStr(CStr::from_ptr(ptx)),
        program_name,
        program,
    )
}

unsafe fn create_from_ptx(
    raw_context: Context,
    ptx: PtxInput,
    program_name: *const i8,
    program: *mut Program,
) -> Result<(), RTresult> {
    let context = null_unwrap(raw_context)?;
    null_check(program_name)?;
    null_check(program)?;
    let program_name = CStr::from_ptr(program_name);
    let mut context = context.borrow_mut()?;
    let ptx = ptx.decode(
        &context.optix_salt[..],
        &context.vendor_salt[..],
        &context.public_vendor_key[..],
    )?;
    let mut_context = &mut *context;
    let weak_context = Weak::clone(&ManuallyDrop::new(Weak::from_raw(raw_context)));
    let (mut program_object, attribute_block, should_save) =
        build_or_load(mut_context, weak_context, program_name, &ptx)?;
    if let Some(ref mut callable_index) = program_object.callable_index {
        mut_context.callable_program_counter += 1;
        *callable_index = mut_context.callable_program_counter;
    }
    if should_save {
        if let Some(ref mut cache) = mut_context.cache {
            cache.save_program(
                &mut_context.compiler_version,
                &mut_context.hiprt_version,
                &mut_context.isa,
                program_name,
                &ptx,
                &program_object,
                &mut_context.cumulative_attributes,
            );
        }
    }
    mut_context.cumulative_attributes = attribute_block;
    let program_object = Rc::new(OptixCell::new(program_object));
    let result = Rc::as_ptr(&program_object);
    mut_context.programs.insert(program_object);
    *program = result;
    Ok(())
}

fn build_or_load(
    mut_context: &mut ContextData,
    weak_context: Weak<OptixCell<ContextData>>,
    program_name: &CStr,
    ptx: &String,
) -> Result<(ProgramData, VariablesBlock, bool), RTresult> {
    if let Some(ref mut cache) = mut_context.cache {
        if let Some((program, attribute_block)) = cache.try_load_program(
            weak_context.clone(),
            &mut_context.compiler_version,
            &mut_context.hiprt_version,
            &mut_context.isa,
            program_name,
            ptx,
            &mut_context.cumulative_attributes,
        ) {
            return Ok((program, attribute_block, false));
        }
    }
    unsafe {
        build(
            weak_context,
            &mut_context.hiprt,
            &mut_context.comgr,
            &mut_context.isa,
            &mut_context.cumulative_attributes,
            ptx,
            program_name,
            mut_context.context,
        )
    }
}

enum PtxInput<'a> {
    Vec(Vec<u8>),
    CStr(&'a CStr),
}

impl<'a> PtxInput<'a> {
    unsafe fn decode(
        self,
        optix_salt: &[u8],
        vendor_salt: &[u8],
        vendor_key: &[u8],
    ) -> Result<String, RTresult> {
        let is_eptx = &self.to_bytes()[..8] == b"eptx0001";
        let mut this = self.to_owned();
        if is_eptx {
            let new_len = eptx::decode_ptx(&mut this, optix_salt, vendor_salt, vendor_key).len();
            this.truncate(new_len);
        }
        String::from_utf8(this).map_err(|_| RTresult::RT_ERROR_UNKNOWN)
    }

    unsafe fn to_bytes(&self) -> &[u8] {
        match self {
            PtxInput::Vec(vec) => &vec[..],
            PtxInput::CStr(cstr) => cstr.to_bytes(),
        }
    }

    unsafe fn to_owned(self) -> Vec<u8> {
        match self {
            PtxInput::Vec(vec) => vec,
            PtxInput::CStr(cstr) => cstr.to_bytes().to_vec(),
        }
    }
}

// TODO: drop rtc program
unsafe fn build(
    weak_context: Weak<OptixCell<ContextData>>,
    hiprt: &HipRt,
    comgr: &Rc<Comgr>,
    isa: &CStr,
    cumulative_attributes: &VariablesBlock,
    text: &str,
    program_name: &CStr,
    context: hiprtContext,
) -> Result<(ProgramData, VariablesBlock, bool), RTresult> {
    let ast =
        ptx::ModuleParser::parse_checked(text).map_err(|_| RTresult::RT_ERROR_INVALID_SOURCE)?;
    let raytracing_module = ptx::to_llvm_module_for_raytracing(
        ast,
        std::str::from_utf8_unchecked(program_name.to_bytes()),
        cumulative_attributes,
    )
    .map_err(|_| RTresult::RT_ERROR_INVALID_SOURCE)?;
    let debug_level = if cfg!(debug_assertions) {
        b"-g\0".as_ptr()
    } else {
        b"-g0\0".as_ptr()
    };
    let options = [
        debug_level,
        // We just want to emit LLVM, we'd use O0, but somehow IR emitted by O0 prevents inling.
        // Weirdly, -disable-llvm-optzns produces much bigger code
        b"-O1\0".as_ptr(),
        // Stop compilation at LLVM
        b"-fgpu-rdc\0".as_ptr(),
        // hiprtc injects -mcumode which we don't want
        b"-mno-cumode\0".as_ptr(),
        // Internalization makes so that _rt_trace_time_mask_flags_64 is module-private
        // and does not get linked with the code generated by ptx compiler
        b"-mllvm\0".as_ptr(),
        b"-amdgpu-internalize-symbols=0\0".as_ptr(),
    ];
    let mut rt_program = ptr::null_mut::<c_void>();
    let headers = raytracing_module
        .headers
        .iter()
        .map(|s| s.as_ptr())
        .collect::<Vec<_>>();
    let header_names = raytracing_module
        .header_names
        .iter()
        .map(|s| s.as_ptr())
        .collect::<Vec<_>>();
    hiprt! {
        hiprt.hiprtBuildTraceProgram(
            context,
            raytracing::Module::KERNEL_NAME.as_ptr(),
            raytracing_module.kernel_source.as_ptr() as _,
            "zluda_rt_kernel\0".as_ptr() as _,
            headers.len() as i32,
            headers.as_ptr() as _,
            header_names.as_ptr() as _,
            options.as_ptr() as _,
            options.len() as i32,
            (&mut rt_program) as *mut _ as _
        ),
        RT_ERROR_INVALID_SOURCE
    };
    let main_bitcode = get_bitcode(rt_program)?;
    let binary = llvm::MemoryBuffer::create_no_copy(&main_bitcode, false);
    let isa_main = CStr::from_bytes_with_nul_unchecked(b"raytracing_main\0");
    let binary = comgr
        .compile(
            hip_common::CompilationMode::Wave32,
            isa,
            iter::once((binary, isa_main))
                .chain(raytracing_module.compilation_module.get_bitcode_all()),
            &raytracing_module.linker_module,
        )
        .map_err(|_| RTresult::RT_ERROR_UNKNOWN)?;
    let module = hip::Module::load_data(&binary).map_err(|_| RTresult::RT_ERROR_UNKNOWN)?;
    Ok((
        ProgramData {
            context: weak_context,
            variables: FxHashMap::default(),
            variables_block: raytracing_module.variables,
            callable_index: if raytracing_module.is_callable {
                Some(0)
            } else {
                None
            },
            shared: Rc::new(ProgramShared { binary, module }),
        },
        raytracing_module.attribute_variables,
        true,
    ))
}

#[cfg(windows)]
const HIPRTC: &'static str = "hiprtc\0";

#[cfg(not(windows))]
const HIPRTC: &'static str = "libhiprtc.so\0";

unsafe fn get_bitcode(rt_program: *mut c_void) -> Result<Vec<u8>, RTresult> {
    use libloading::{Library, Symbol};
    let hiprtc = Library::new(HIPRTC).map_err(|_| RTresult::RT_ERROR_UNKNOWN)?;
    let hiprtc_get_bitcode_size: Symbol<
        unsafe fn(prog: *mut c_void, bitcode_size: *mut usize) -> u32,
    > = hiprtc
        .get(b"hiprtcGetBitcodeSize\0")
        .map_err(|_| RTresult::RT_ERROR_UNKNOWN)?;
    let hiprtc_get_bitcode: Symbol<unsafe fn(prog: *mut c_void, bitcode: *mut u8) -> u32> = hiprtc
        .get(b"hiprtcGetBitcode\0")
        .map_err(|_| RTresult::RT_ERROR_UNKNOWN)?;
    let mut program_size = 0;
    let error = hiprtc_get_bitcode_size(rt_program, &mut program_size);
    if error != 0 {
        return Err(RTresult::RT_ERROR_UNKNOWN);
    }
    let mut main_bitcode = vec![0u8; program_size];
    let error = hiprtc_get_bitcode(rt_program, main_bitcode.as_mut_ptr());
    if error != 0 {
        return Err(RTresult::RT_ERROR_UNKNOWN);
    }
    Ok(main_bitcode)
}

pub(crate) unsafe fn declare_variable(
    program_ptr: Program,
    name: *const i8,
    v: *mut Variable,
) -> Result<(), RTresult> {
    null_check(name)?;
    let v = null_unwrap_mut(v)?;
    let program = null_unwrap(program_ptr)?;
    let mut program = program.borrow_mut()?;
    let variable = VariableData::new(&mut *program)?;
    let name = CStr::from_ptr(name as _).to_owned();
    let result = Rc::as_ptr(&variable);
    program.variables.insert(name, variable);
    *v = result;
    Ok(())
}

pub(crate) unsafe fn destroy(program: Program) -> Result<(), RTresult> {
    OptixCell::destroy(program)
}

pub(crate) unsafe fn get_id(program: Program, program_id: *mut i32) -> Result<(), RTresult> {
    let program = null_unwrap(program)?;
    let program = program.borrow()?;
    let callable_id = match program.callable_index {
        Some(i) => i,
        None => return Err(RTresult::RT_ERROR_INVALID_VALUE),
    };
    *program_id = callable_id as i32;
    Ok(())
}

pub(crate) unsafe fn query_variable(
    program: Program,
    name: *const i8,
    v: *mut Variable,
) -> Result<(), RTresult> {
    null_check(name)?;
    null_check(v)?;
    let program = null_unwrap(program)?;
    let program = (program).borrow()?;
    *v = program
        .variables
        .get(CStr::from_ptr(name))
        .map(|variable| Rc::as_ptr(variable))
        .unwrap_or(ptr::null_mut());
    Ok(())
}

pub(crate) fn validate(program: Program) -> Result<(), RTresult> {
    null_check(program)?;
    Ok(())
}

pub(crate) unsafe fn create_from_program(
    context: Context,
    program_in: Program,
    program_out: *mut Program,
) -> Result<(), RTresult> {
    null_check(program_out)?;
    let context_wrapper = null_unwrap(context)?;
    let mut context = context_wrapper.borrow_mut()?;
    let program_in = null_unwrap(program_in)?;
    let program_in = program_in.borrow()?;
    *program_out = program_in.create_from(context_wrapper, &mut context)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test_common::OptixFns;
    use crate::{optix_test, tests};
    use std::ptr;

    optix_test!(cloning_program_does_not_clone_variables);

    unsafe fn cloning_program_does_not_clone_variables<Optix: OptixFns>(o: Optix) {
        let mut ctx = ptr::null_mut();
        o.rtContextCreate(&mut ctx);
        let mut program1 = ptr::null_mut();
        let mut ptx = tests::ANY_HIT_INTERSECT_PTX.to_string();
        ptx.push('\0');
        o.rtProgramCreateFromPTXString(
            ctx,
            ptx.as_ptr() as _,
            "set_buffer\0".as_ptr() as _,
            &mut program1,
        );
        let mut var1 = ptr::null_mut();
        o.rtProgramDeclareVariable(program1, b"output_buffer\0".as_ptr() as _, &mut var1);
        let mut program2 = ptr::null_mut();
        o.rtProgramCreateFromProgram(ctx, program1, &mut program2);
        let mut var2 = ptr::null_mut();
        o.rtProgramQueryVariable(program2, b"output_buffer\0".as_ptr() as _, &mut var2);
        assert_ne!(var1, ptr::null_mut());
        assert_eq!(var2, ptr::null_mut());
    }
}

pub(crate) unsafe fn get_context(
    program: *const OptixCell<ProgramData>,
    context: *mut *const OptixCell<ContextData>,
) -> Result<(), RTresult> {
    let program = null_unwrap(program)?;
    let program = program.borrow()?;
    *context = program.context.as_ptr();
    Ok(())
}
