use crate::ptx;
use crate::translate;
use std::error;
use std::ffi::{CStr, CString};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::mem;
use std::slice;
use std::str;

macro_rules! test_ptx {
    ($fn_name:ident, $input:expr, $output:expr) => {
        #[test]
        fn $fn_name() -> Result<(), Box<dyn std::error::Error>> {
            let ptx = include_str!(concat!(stringify!($fn_name), ".ptx"));
            let input = $input;
            let mut output = $output;
            test_ptx_assert(stringify!($fn_name), ptx, &input, &mut output)
        }
    };
}

test_ptx!(ld_st, [1u64], [1u64]);

struct DisplayError<T: Display + Debug> {
    err: T,
}

impl<T: Display + Debug> Display for DisplayError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.err, f)
    }
}

impl<T: Display + Debug> Debug for DisplayError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl<T: Display + Debug> error::Error for DisplayError<T> {}

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

fn run_spirv<T: From<u8> + ze::SafeRepr + Copy>(
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
    let mut drivers = ze::Driver::get()?;
    let drv = drivers.drain(0..1).next().unwrap();
    let mut devices = drv.devices()?;
    let dev = devices.drain(0..1).next().unwrap();
    let queue = ze::CommandQueue::new(&dev)?;
    let module = ze::Module::new_spirv(&dev, byte_il, None)?;
    let kernel = ze::Kernel::new(&module, name)?;
    let mut inp_b = ze::DeviceBuffer::<T>::new(&drv, &dev, input.len())?;
    let mut out_b = ze::DeviceBuffer::<T>::new(&drv, &dev, output.len())?;
    let inp_b_ptr_mut: ze::BufferPtrMut<T> = (&mut inp_b).into();
    let event_pool = ze::EventPool::new(&drv, 3, Some(&[&dev]))?;
    let ev0 = ze::Event::new(&event_pool, 0)?;
    let ev1 = ze::Event::new(&event_pool, 1)?;
    let mut cmd_list = ze::CommandList::new(&dev)?;
    let out_b_ptr: ze::BufferPtrMut<T> = (&mut out_b).into();
    cmd_list.append_memory_copy(inp_b_ptr_mut, input, None, Some(&ev0))?;
    cmd_list.append_memory_fill(out_b_ptr, 0u8.into(), Some(&ev1))?;
    kernel.set_group_size(1, 1, 1)?;
    kernel.set_arg_buffer(0, inp_b_ptr_mut)?;
    kernel.set_arg_buffer(1, out_b_ptr)?;
    cmd_list.append_launch_kernel(&kernel, &[1, 1, 1], None, &[&ev0, &ev1])?;
    cmd_list.append_memory_copy(result.as_mut_slice(), inp_b_ptr_mut, None, Some(&ev0))?;
    queue.execute(cmd_list)?;
    Ok(result)
}
