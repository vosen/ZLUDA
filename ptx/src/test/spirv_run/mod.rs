use crate::ptx;
use crate::translate;
use rspirv::{
    binary::{Assemble, Disassemble},
    dr::{Block, Function, Instruction, Loader, Operand},
};
use spirv_headers::Word;
use spirv_tools_sys::{
    spv_binary, spv_endianness_t, spv_parsed_instruction_t, spv_result_t, spv_target_env,
};
use std::error;
use std::ffi::{c_void, CStr, CString};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::mem;
use std::slice;
use std::{borrow::Cow, collections::HashMap, env, fs, path::PathBuf, ptr, str};

macro_rules! test_ptx {
    ($fn_name:ident, $input:expr, $output:expr) => {
        paste::item! {
            #[test]
            fn [<$fn_name _ptx>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx = include_str!(concat!(stringify!($fn_name), ".ptx"));
                let input = $input;
                let mut output = $output;
                test_ptx_assert(stringify!($fn_name), ptx, &input, &mut output)
            }
        }

        paste::item! {
            #[test]
            fn [<$fn_name _spvtxt>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx_txt = include_str!(concat!(stringify!($fn_name), ".ptx"));
                let spirv_file_name = concat!(stringify!($fn_name), ".spvtxt");
                let spirv_txt = include_bytes!(concat!(stringify!($fn_name), ".spvtxt"));
                test_spvtxt_assert(ptx_txt, spirv_txt, spirv_file_name)
            }
        }
    };
}

test_ptx!(ld_st, [1u64], [1u64]);
test_ptx!(mov, [1u64], [1u64]);
test_ptx!(mul_lo, [1u64], [2u64]);
test_ptx!(mul_hi, [u64::max_value()], [1u64]);
test_ptx!(add, [1u64], [2u64]);
test_ptx!(setp, [10u64, 11u64], [1u64, 0u64]);
test_ptx!(bra, [10u64], [11u64]);
test_ptx!(not, [0u64], [u64::max_value()]);
test_ptx!(shl, [11u64], [44u64]);
test_ptx!(cvt_sat_s_u, [-1i32], [0i32]);
test_ptx!(cvta, [3.0f32], [3.0f32]);
test_ptx!(block, [1u64], [2u64]);
test_ptx!(local_align, [1u64], [1u64]);
test_ptx!(call, [1u64], [2u64]);

struct DisplayError<T: Debug> {
    err: T,
}

impl<T: Debug> Display for DisplayError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl<T: Debug> Debug for DisplayError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl<T: Debug> error::Error for DisplayError<T> {}

fn test_ptx_assert<'a, T: From<u8> + ze::SafeRepr + Debug + Copy + PartialEq>(
    name: &str,
    ptx_text: &'a str,
    input: &[T],
    output: &mut [T],
) -> Result<(), Box<dyn error::Error + 'a>> {
    let mut errors = Vec::new();
    let ast = ptx::ModuleParser::new().parse(&mut errors, ptx_text)?;
    assert!(errors.len() == 0);
    let spirv = translate::to_spirv(ast)?;
    let name = CString::new(name)?;
    let result =
        run_spirv(name.as_c_str(), &spirv, input, output).map_err(|err| DisplayError { err })?;
    assert_eq!(output, result.as_slice());
    Ok(())
}

fn run_spirv<T: From<u8> + ze::SafeRepr + Copy + Debug>(
    name: &CStr,
    spirv: &[u32],
    input: &[T],
    output: &mut [T],
) -> ze::Result<Vec<T>> {
    ze::init()?;
    let byte_il = unsafe {
        slice::from_raw_parts::<u8>(
            spirv.as_ptr() as *const _,
            spirv.len() * mem::size_of::<u32>(),
        )
    };
    let mut result = vec![0u8.into(); output.len()];
    {
        let mut drivers = ze::Driver::get()?;
        let drv = drivers.drain(0..1).next().unwrap();
        let mut ctx = ze::Context::new(&drv)?;
        let mut devices = drv.devices()?;
        let dev = devices.drain(0..1).next().unwrap();
        let queue = ze::CommandQueue::new(&mut ctx, &dev)?;
        let module = ze::Module::new_spirv(&mut ctx, &dev, byte_il, None)?;
        let mut kernel = ze::Kernel::new_resident(&module, name)?;
        kernel.set_indirect_access(
            ze::sys::ze_kernel_indirect_access_flags_t::ZE_KERNEL_INDIRECT_ACCESS_FLAG_DEVICE,
        )?;
        let mut inp_b = ze::DeviceBuffer::<T>::new(&mut ctx, &dev, input.len())?;
        let mut out_b = ze::DeviceBuffer::<T>::new(&mut ctx, &dev, output.len())?;
        let inp_b_ptr_mut: ze::BufferPtrMut<T> = (&mut inp_b).into();
        let event_pool = ze::EventPool::new(&mut ctx, 3, Some(&[&dev]))?;
        let ev0 = ze::Event::new(&event_pool, 0)?;
        let ev1 = ze::Event::new(&event_pool, 1)?;
        let mut ev2 = ze::Event::new(&event_pool, 2)?;
        let mut cmd_list = ze::CommandList::new(&mut ctx, &dev)?;
        let out_b_ptr_mut: ze::BufferPtrMut<T> = (&mut out_b).into();
        let mut init_evs = [ev0, ev1];
        cmd_list.append_memory_copy(inp_b_ptr_mut, input, Some(&mut init_evs[0]), &mut [])?;
        cmd_list.append_memory_fill(out_b_ptr_mut, 0, Some(&mut init_evs[1]), &mut [])?;
        kernel.set_group_size(1, 1, 1)?;
        kernel.set_arg_buffer(0, inp_b_ptr_mut)?;
        kernel.set_arg_buffer(1, out_b_ptr_mut)?;
        cmd_list.append_launch_kernel(&kernel, &[1, 1, 1], Some(&mut ev2), &mut init_evs)?;
        cmd_list.append_memory_copy(result.as_mut_slice(), out_b_ptr_mut, None, &mut [ev2])?;
        queue.execute(cmd_list)?;
    }
    Ok(result)
}

fn test_spvtxt_assert<'a>(
    ptx_txt: &'a str,
    spirv_txt: &'a [u8],
    spirv_file_name: &'a str,
) -> Result<(), Box<dyn error::Error + 'a>> {
    let mut errors = Vec::new();
    let ast = ptx::ModuleParser::new().parse(&mut errors, ptx_txt)?;
    assert!(errors.len() == 0);
    let ptx_mod = translate::to_spirv_module(ast)?;
    let spv_context =
        unsafe { spirv_tools::spvContextCreate(spv_target_env::SPV_ENV_UNIVERSAL_1_3) };
    assert!(spv_context != ptr::null_mut());
    let mut spv_binary: spv_binary = ptr::null_mut();
    let result = unsafe {
        spirv_tools::spvTextToBinary(
            spv_context,
            spirv_txt.as_ptr() as *const _,
            spirv_txt.len(),
            &mut spv_binary,
            ptr::null_mut(),
        )
    };
    assert!(result == spv_result_t::SPV_SUCCESS);
    let mut parsed_spirv = Vec::<u32>::new();
    let result = unsafe {
        spirv_tools::spvBinaryParse(
            spv_context,
            &mut parsed_spirv as *mut _ as *mut _,
            (*spv_binary).code,
            (*spv_binary).wordCount,
            Some(parse_header_cb),
            Some(parse_instruction_cb),
            ptr::null_mut(),
        )
    };
    assert!(result == spv_result_t::SPV_SUCCESS);
    let mut loader = Loader::new();
    rspirv::binary::parse_words(&parsed_spirv, &mut loader)?;
    let spvtxt_mod = loader.module();
    unsafe { spirv_tools::spvBinaryDestroy(spv_binary) };
    if !is_spirv_fn_equal(&ptx_mod.functions[0], &spvtxt_mod.functions[0]) {
        // We could simply use ptx_mod.disassemble, but SPIRV-Tools text formattinmg is so much nicer
        let spv_from_ptx_binary = ptx_mod.assemble();
        let mut spv_text: spirv_tools::spv_text = ptr::null_mut();
        let result = unsafe {
            spirv_tools::spvBinaryToText(
                spv_context,
                spv_from_ptx_binary.as_ptr(),
                spv_from_ptx_binary.len(),
                (spirv_tools::spv_binary_to_text_options_t::SPV_BINARY_TO_TEXT_OPTION_INDENT | spirv_tools::spv_binary_to_text_options_t::SPV_BINARY_TO_TEXT_OPTION_NO_HEADER |  spirv_tools::spv_binary_to_text_options_t::SPV_BINARY_TO_TEXT_OPTION_FRIENDLY_NAMES).0,
                &mut spv_text as *mut _,
                ptr::null_mut()
            )
        };
        unsafe { spirv_tools::spvContextDestroy(spv_context) };
        let spirv_text = if result == spv_result_t::SPV_SUCCESS {
            let raw_text = unsafe {
                std::slice::from_raw_parts((*spv_text).str_ as *const u8, (*spv_text).length)
            };
            let spv_from_ptx_text = unsafe { str::from_utf8_unchecked(raw_text) };
            // TODO: stop leaking kernel text
            Cow::Borrowed(spv_from_ptx_text)
        } else {
            Cow::Owned(ptx_mod.disassemble())
        };
        if let Ok(dump_path) = env::var("NOTCUDA_TEST_SPIRV_DUMP_DIR") {
            let mut path = PathBuf::from(dump_path);
            if let Ok(()) = fs::create_dir_all(&path) {
                path.push(spirv_file_name);
                #[allow(unused_must_use)]
                {
                    fs::write(path, spirv_text.as_bytes());
                }
            }
        }
        panic!(spirv_text);
    }
    unsafe { spirv_tools::spvContextDestroy(spv_context) };
    Ok(())
}

fn is_spirv_fn_equal(fn1: &Function, fn2: &Function) -> bool {
    let mut map = HashMap::new();
    if !is_option_equal(&fn1.def, &fn2.def, &mut map, is_instr_equal) {
        return false;
    }
    if !is_option_equal(&fn1.end, &fn2.end, &mut map, is_instr_equal) {
        return false;
    }
    if fn1.parameters.len() != fn2.parameters.len() {
        return false;
    }
    for (inst1, inst2) in fn1.parameters.iter().zip(fn2.parameters.iter()) {
        if !is_instr_equal(inst1, inst2, &mut map) {
            return false;
        }
    }
    if fn1.blocks.len() != fn2.blocks.len() {
        return false;
    }
    for (b1, b2) in fn1.blocks.iter().zip(fn2.blocks.iter()) {
        if !is_block_equal(b1, b2, &mut map) {
            return false;
        }
    }
    true
}

fn is_block_equal(b1: &Block, b2: &Block, map: &mut HashMap<Word, Word>) -> bool {
    if !is_option_equal(&b1.label, &b2.label, map, is_instr_equal) {
        return false;
    }
    if b1.instructions.len() != b2.instructions.len() {
        return false;
    }
    for (inst1, inst2) in b1.instructions.iter().zip(b2.instructions.iter()) {
        if !is_instr_equal(inst1, inst2, map) {
            return false;
        }
    }
    true
}

fn is_instr_equal(
    instr1: &Instruction,
    instr2: &Instruction,
    map: &mut HashMap<Word, Word>,
) -> bool {
    if instr1.class.opcode != instr2.class.opcode {
        return false;
    }
    if !is_option_equal(&instr1.result_type, &instr2.result_type, map, is_word_equal) {
        return false;
    }
    if !is_option_equal(&instr1.result_id, &instr2.result_id, map, is_word_equal) {
        return false;
    }
    if instr1.operands.len() != instr2.operands.len() {
        return false;
    }
    for (o1, o2) in instr1.operands.iter().zip(instr2.operands.iter()) {
        match (o1, o2) {
            (Operand::IdMemorySemantics(w1), Operand::IdMemorySemantics(w2)) => {
                if !is_word_equal(w1, w2, map) {
                    return false;
                }
            }
            (Operand::IdScope(w1), Operand::IdScope(w2)) => {
                if !is_word_equal(w1, w2, map) {
                    return false;
                }
            }
            (Operand::IdRef(w1), Operand::IdRef(w2)) => {
                if !is_word_equal(w1, w2, map) {
                    return false;
                }
            }
            (o1, o2) => {
                if o1 != o2 {
                    return false;
                }
            }
        }
    }
    true
}

fn is_word_equal(w1: &Word, w2: &Word, map: &mut HashMap<Word, Word>) -> bool {
    match map.entry(*w1) {
        std::collections::hash_map::Entry::Occupied(entry) => {
            if entry.get() != w2 {
                return false;
            }
        }
        std::collections::hash_map::Entry::Vacant(entry) => {
            entry.insert(*w2);
        }
    }
    true
}

fn is_option_equal<T, F: FnOnce(&T, &T, &mut HashMap<Word, Word>) -> bool>(
    o1: &Option<T>,
    o2: &Option<T>,
    map: &mut HashMap<Word, Word>,
    f: F,
) -> bool {
    match (o1, o2) {
        (Some(t1), Some(t2)) => f(t1, t2, map),
        (None, None) => true,
        _ => panic!(),
    }
}

unsafe extern "C" fn parse_header_cb(
    user_data: *mut c_void,
    endian: spv_endianness_t,
    magic: u32,
    version: u32,
    generator: u32,
    id_bound: u32,
    reserved: u32,
) -> spv_result_t {
    if endian == spv_endianness_t::SPV_ENDIANNESS_BIG {
        return spv_result_t::SPV_UNSUPPORTED;
    }
    let result_vec: &mut Vec<u32> = std::mem::transmute(user_data);
    result_vec.push(magic);
    result_vec.push(version);
    result_vec.push(generator);
    result_vec.push(id_bound);
    result_vec.push(reserved);
    spv_result_t::SPV_SUCCESS
}

unsafe extern "C" fn parse_instruction_cb(
    user_data: *mut c_void,
    inst: *const spv_parsed_instruction_t,
) -> spv_result_t {
    let inst = &*inst;
    let result_vec: &mut Vec<u32> = std::mem::transmute(user_data);
    for i in 0..inst.num_words {
        result_vec.push(*(inst.words.add(i as usize)));
    }
    spv_result_t::SPV_SUCCESS
}
