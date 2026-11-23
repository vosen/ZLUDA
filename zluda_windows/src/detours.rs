use std::{
    ffi::OsStr,
    mem::{self, MaybeUninit},
    os::windows::ffi::OsStrExt,
};

pub const GUID: windows::core::GUID =
    windows::core::GUID::from_u128(0x2f32c748f0f2045f42a9c63ef71c3d27);

pub fn unpack_paths(
    payload: &[u8],
) -> Option<(
    [Option<&[u16]>; crate::LIBRARIES32.len()],
    [Option<&[u16]>; crate::LIBRARIES64.len()],
    &[u8],
)> {
    let payload = bytemuck::try_cast_slice::<_, u16>(payload).ok()?;
    let mut raw_str_iterator =
        payload
            .split_inclusive(|c| *c == 0)
            .map(|slice| if slice.len() == 1 { None } else { Some(slice) });
    let dll_paths32 = array_try_from_fn(|_| raw_str_iterator.next())?;
    let dll_paths64 = array_try_from_fn(|_| raw_str_iterator.next())?;
    let redirect_dll = raw_str_iterator.next()??;
    Some((
        dll_paths32,
        dll_paths64,
        bytemuck::cast_slice::<_, u8>(redirect_dll),
    ))
}

fn array_try_from_fn<T, const N: usize>(mut fn_: impl FnMut(usize) -> Option<T>) -> Option<[T; N]> {
    let mut array = [const { MaybeUninit::uninit() }; N];
    for i in 0..N {
        array[i].write(fn_(i)?);
    }
    unsafe { mem::transmute_copy(&Some(array)) }
}

pub fn pack_paths<'a>(
    paths_32: impl Iterator<Item = Option<&'a OsStr>>,
    paths_64: impl Iterator<Item = Option<&'a OsStr>>,
    redirect_dll: &'a str,
) -> Vec<u8> {
    if cfg!(debug_assertions) {
        assert!(*redirect_dll.as_bytes().last().unwrap() == 0);
    }
    let mut result = Vec::new();
    for path in paths_32.chain(paths_64) {
        if let Some(path) = path {
            for c in path.encode_wide() {
                result.extend_from_slice(&c.to_le_bytes());
            }
        }
        result.extend_from_slice(&[0u8, 0u8]);
    }
    let redirect_dll = redirect_dll.as_bytes();
    result.extend_from_slice(redirect_dll);
    if redirect_dll.len() % 2 == 0 {
        result.extend_from_slice(&[0u8, 0u8]);
    } else {
        result.extend_from_slice(&[0u8]);
    }
    result
}
