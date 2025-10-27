use std::ops::ControlFlow;
use trie_hard::TrieHard;
use uuid::uuid;
use widestring::{u16str, U16Str};

pub static LIBRARIES: &'static [LibraryInfo] =
    &[NVCUDA, NVML, DNN8, DNN9, BLAS, BLAS_LT, SPARSE, FFT];

pub const NVCUDA: LibraryInfo = LibraryInfo {
    short_name: "nvcuda",
    ascii_names: &["nvcuda.dll"],
    utf16_names: &[u16str!("nvcuda.dll")],
    guid: uuid!("9ab28276-52f4-4fe4-bc19-4c966f3aa468"),
};

pub const NVML: LibraryInfo = LibraryInfo {
    short_name: "nvml",
    ascii_names: &["nvml.dll"],
    utf16_names: &[u16str!("nvml.dll")],
    guid: uuid!("4fd0ef13-df98-40a7-b34b-6cf55f34a8e9"),
};

pub const DNN8: LibraryInfo = LibraryInfo {
    short_name: "cudnn8",
    ascii_names: &["cudnn64_8.dll"],
    utf16_names: &[u16str!("cudnn64_8.dll")],
    guid: uuid!("7ed2e46c-82ce-4779-adbd-b39cd6d9da80"),
};

pub const DNN9: LibraryInfo = LibraryInfo {
    short_name: "cudnn9",
    ascii_names: &["cudnn64_9.dll"],
    utf16_names: &[u16str!("cudnn64_9.dll")],
    guid: uuid!("6c258b11-104a-4ab0-b343-4e490e136ddd"),
};

pub const BLAS: LibraryInfo = LibraryInfo {
    short_name: "cublas",
    ascii_names: &["cublas64_13.dll", "cublas64_12.dll"],
    utf16_names: &[u16str!("cublas64_13.dll"), u16str!("cublas64_12.dll")],
    guid: uuid!("b9dcf552-c7a1-4dfe-b27a-58b56695effe"),
};

pub const BLAS_LT: LibraryInfo = LibraryInfo {
    short_name: "cublaslt",
    ascii_names: &["cublaslt64_13.dll", "cublaslt64_12.dll"],
    utf16_names: &[u16str!("cublaslt64_13.dll"), u16str!("cublaslt64_12.dll")],
    guid: uuid!("667b6076-c4c2-44f4-8b81-073dea1d6125"),
};

pub const SPARSE: LibraryInfo = LibraryInfo {
    short_name: "cusparse",
    ascii_names: &["cusparse64_12.dll", "cusparse64_11.dll"],
    utf16_names: &[u16str!("cusparse64_12.dll"), u16str!("cusparse64_11.dll")],
    guid: uuid!("741fb1b7-2e24-4484-8205-4794175ccb78"),
};

pub const FFT: LibraryInfo = LibraryInfo {
    short_name: "cufft",
    ascii_names: &["cufft64_12.dll", "cufft64_11.dll"],
    utf16_names: &[u16str!("cufft64_12.dll"), u16str!("cufft64_11.dll")],
    guid: uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
};

#[derive(Debug)]
pub struct LibraryInfo {
    pub short_name: &'static str,
    pub ascii_names: &'static [&'static str],
    pub utf16_names: &'static [&'static U16Str],
    pub guid: uuid::Uuid,
}

pub struct DllLookup {
    max_dll_len: usize,
    ascii_trie: TrieHard<'static, &'static LibraryInfo>,
    utf16_trie: TrieHard<'static, &'static LibraryInfo>,
}

impl DllLookup {
    pub fn new() -> Self {
        let library_names = LIBRARIES
            .iter()
            .flat_map(|lib| {
                lib.ascii_names.iter().map(move |dll_name| {
                    let ascii_bytes = dll_name.as_bytes();
                    (ascii_bytes, lib)
                })
            })
            .collect::<Vec<_>>();
        let ascii_trie = TrieHard::new(library_names);
        let max_dll_len = ascii_trie
            .iter()
            .map(|(dll_name, _)| dll_name.len())
            .max()
            .unwrap_or(0);
        let library_names_utf16 = LIBRARIES
            .iter()
            .flat_map(|lib| {
                lib.utf16_names.iter().map(move |dll_name| {
                    let utf16_bytes = Self::to_byte_slice(dll_name.as_slice());
                    (utf16_bytes, lib)
                })
            })
            .collect::<Vec<_>>();
        let utf16_trie = TrieHard::new(library_names_utf16);
        Self {
            max_dll_len,
            ascii_trie,
            utf16_trie,
        }
    }

    fn to_byte_slice<'a, T: Copy>(s: &'a [T]) -> &'a [u8] {
        unsafe {
            std::slice::from_raw_parts(s.as_ptr() as *const u8, s.len() * std::mem::size_of::<T>())
        }
    }

    pub fn lookup_impl<T: Copy>(
        max_dll_len: usize,
        trie: &TrieHard<'static, &'static LibraryInfo>,
        dll_name: &[T],
        as_char: impl Fn(T) -> char,
        lower_case: impl Fn(T) -> T,
    ) -> Option<&'static LibraryInfo> {
        let file_name_len = dll_name
            .iter()
            .rev()
            .copied()
            .enumerate()
            .try_fold((), |_, (i, c)| {
                if i > max_dll_len {
                    return ControlFlow::Break(None);
                }
                let c = as_char(c);
                if c == '\\' || c == '/' {
                    return ControlFlow::Break(Some(i));
                }
                ControlFlow::Continue(())
            });
        let file_name_length = match file_name_len {
            ControlFlow::Break(Some(len)) => len,
            ControlFlow::Break(None) => return None,
            ControlFlow::Continue(()) => dll_name.len(),
        };
        let file_name = &dll_name[dll_name.len() - file_name_length..];
        alloca::with_alloca(file_name.len() * std::mem::size_of::<T>(), |uninit_slice| {
            let lower_case_slice = unsafe {
                std::slice::from_raw_parts_mut(
                    uninit_slice.as_mut_ptr().cast::<T>(),
                    file_name.len(),
                )
            };
            for (dst, src) in lower_case_slice.iter_mut().zip(file_name.iter().copied()) {
                *dst = lower_case(src);
            }
            trie.get(Self::to_byte_slice(lower_case_slice))
        })
    }

    pub fn lookup_ascii(&self, dll_name: &[u8]) -> Option<&'static LibraryInfo> {
        Self::lookup_impl(
            self.max_dll_len,
            &self.ascii_trie,
            dll_name,
            |c| c as char,
            |c| c.to_ascii_lowercase(),
        )
    }

    pub fn lookup_utf16(&self, dll_name: &[u16]) -> Option<&'static LibraryInfo> {
        Self::lookup_impl(
            self.max_dll_len,
            &self.utf16_trie,
            dll_name,
            |c| char::from_u32(c as u32).unwrap_or_default(),
            |c| {
                char::from_u32(c as u32)
                    .unwrap_or_default()
                    .to_ascii_lowercase() as u16
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_look_up_from_full_path() {
        let lookup = DllLookup::new();
        let path = r#"C:\Windows\System32\nvml.dll"#;
        assert_eq!(
            NVML.guid,
            lookup.lookup_ascii(path.as_bytes()).unwrap().guid
        );
    }

    #[test]
    fn can_look_up_from_just_file_name() {
        let lookup = DllLookup::new();
        let path = r#"cusparse64_11.dll"#;
        assert_eq!(
            SPARSE.guid,
            lookup.lookup_ascii(path.as_bytes()).unwrap().guid
        );
    }

    #[test]
    fn can_look_up_utf16() {
        let lookup = DllLookup::new();
        let path = u16str!("\\cublaslt64_13.dll");
        assert_eq!(
            BLAS_LT.guid,
            lookup.lookup_utf16(path.as_slice()).unwrap().guid
        );
    }

    #[test]
    fn can_look_up_utf16_case_insensitive() {
        let lookup = DllLookup::new();
        let path = u16str!("\\CUBLASLT64_13.dll");
        assert_eq!(
            BLAS_LT.guid,
            lookup.lookup_utf16(path.as_slice()).unwrap().guid
        );
    }

    #[test]
    fn fails_on_mismatch() {
        let lookup = DllLookup::new();
        let path = u16str!("cusparse64_13.dll");
        assert!(lookup.lookup_utf16(path.as_slice()).is_none());
    }

    #[test]
    fn does_not_explode_on_utf16() {
        let lookup = DllLookup::new();
        let path = u16str!("🇧🇴");
        assert_eq!(path.chars().count(), 2);
        assert_eq!(path.as_slice().len(), 4);
        assert!(lookup.lookup_utf16(path.as_slice()).is_none());
    }
}
