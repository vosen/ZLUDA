use bpaf::{Bpaf, Parser};
use cuda_types::{
    cuda::{CUcontext, CUmodule},
    dark_api::FatbinHeader,
};
use goblin::{
    container::{Ctx, Endian},
    elf::SectionHeader,
    pe::options::ParseOptions,
};
use indicatif::{ProgressBar, ProgressDrawTarget};
use rayon::Scope;
use std::{error::Error, ffi::CStr, fs::File, io::Read, mem, path::PathBuf, ptr, sync::Arc};
use walkdir::DirEntry;

#[derive(Clone, Debug, Bpaf)]
struct Arguments {
    /// CUDA device ID to use for compilation
    #[bpaf(long, short('d'), fallback(0))]
    device: u32,

    /// Follow symbolic links when traversing directories
    follow_links: bool,

    /// Directory or file to scan for CUDA binaries and precompile
    #[bpaf(positional)]
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = arguments().run();
    let (_lib_handle, cuda) = unsafe { CudaContext::new()? };
    unsafe { (cuda.cuInit)(0) }.map_err(|_| "Failed to initialize ZLUDA")?;
    let mut cu_ctx = CUcontext(ptr::null_mut());
    unsafe { (cuda.cuCtxCreate_v2)(&mut cu_ctx, 0, config.device as i32) }
        .map_err(|_| "Failed to create ZLUDA context")?;
    let progress = indicatif::MultiProgress::new();
    let all_files_progress = progress.insert(0, ProgressBar::no_length());
    all_files_progress.set_style(
        indicatif::ProgressStyle::with_template(
            "[1/2] Building file list... {spinner} {pos} file(s) scanned",
        )?
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ "),
    );
    let extract_ptx = progress.insert(1, ProgressBar::new(0));
    extract_ptx.set_style(indicatif::ProgressStyle::with_template(
        "[2/2] {wide_bar} {pos}/{len} file(s) compiled",
    )?);
    let parallel_context = ParallelContext {
        cuda,
        cu_ctx,
        extract_ptx: extract_ptx.clone(),
    };
    parallel_context
        .extract_ptx
        .set_draw_target(ProgressDrawTarget::hidden());
    rayon::scope(|scope| {
        let context = Context {
            parallel_context: &parallel_context,
            all_files_progress: &all_files_progress,
            scope,
        };
        read_and_count_files_from_path(&context, &config.input, config.follow_links);
    });
    extract_ptx.finish();
    Ok(())
}

struct Context<'a, 's> {
    parallel_context: &'a ParallelContext,
    all_files_progress: &'a ProgressBar,
    scope: &'a Scope<'s>,
}

#[derive(Clone)]
struct ParallelContext {
    cuda: CudaContext,
    cu_ctx: CUcontext,
    extract_ptx: ProgressBar,
}

fn read_and_count_files_from_path(context: &Context, path: &PathBuf, follow_links: bool) {
    let elf_count = walkdir::WalkDir::new(path)
        .follow_links(follow_links)
        .into_iter()
        .filter_map(|dir_entry| dir_entry.ok())
        .map(|entry| {
            context.all_files_progress.inc(1);
            check_if_elf_and_enqueue(context, entry) as u64
        })
        .sum();
    context.parallel_context.extract_ptx.set_length(elf_count);
    context
        .parallel_context
        .extract_ptx
        .set_draw_target(ProgressDrawTarget::stdout());
    context.all_files_progress.finish();
}

fn check_if_elf_and_enqueue(context: &Context, entry: DirEntry) -> bool {
    if let Ok(mut file) = File::open(entry.path()) {
        let mut header = [0u8; 4];
        if let Ok(bytes) = file.read(&mut header) {
            if bytes < 4 {
                return false;
            }
            if header[..2] == goblin::pe::header::DOS_MAGIC.to_le_bytes() {
                let parallel_context = context.parallel_context.clone();
                context.scope.spawn(move |scope| {
                    extract_from_binary(
                        scope,
                        parallel_context,
                        file,
                        header,
                        pe_find_fatbin_section,
                    );
                });
                return true;
            } else if &header == goblin::elf::header::ELFMAG {
                let parallel_context = context.parallel_context.clone();
                context.scope.spawn(move |scope| {
                    extract_from_binary(
                        scope,
                        parallel_context,
                        file,
                        header,
                        elf_find_fatbin_section,
                    );
                });
                return true;
            }
        }
    }
    false
}

fn pe_find_fatbin_section(bytes: &[u8]) -> Option<std::ops::Range<usize>> {
    let pe_header = goblin::pe::PE::parse_with_opts(
        bytes,
        &ParseOptions {
            resolve_rva: true,
            parse_attribute_certificates: false,
        },
    )
    .ok()?;
    pe_header.sections.iter().find_map(|section| {
        // PE section name field is limited to 8 chars
        if section.name == *b".nv_fatb" {
            let range = section.pointer_to_raw_data as usize
                ..(section
                    .pointer_to_raw_data
                    .saturating_add(section.size_of_raw_data)) as usize;
            Some(range)
        } else {
            None
        }
    })
}

fn elf_find_fatbin_section(bytes: &[u8]) -> Option<std::ops::Range<usize>> {
    let goblin_ctx = Ctx::new(goblin::container::Container::Big, Endian::Little);
    let header = goblin::elf64::header::Header::parse(bytes).ok()?;
    let section_headers = SectionHeader::parse(
        bytes,
        header.e_shoff as usize,
        header.e_shnum as usize,
        goblin_ctx,
    )
    .ok()?;
    let string_table_section = section_headers.get(header.e_shstrndx as usize)?;
    let string_table_start = usize::try_from(string_table_section.sh_offset).ok()?;
    let string_table_size = usize::try_from(string_table_section.sh_size).ok()?;
    let string_table_end = string_table_start.checked_add(string_table_size)?;
    let string_table = bytes.get(string_table_start..string_table_end)?;
    section_headers.into_iter().find_map(|section| {
        let name_start = section.sh_name as usize;
        let section_name = CStr::from_bytes_until_nul(string_table.get(name_start..)?).ok()?;
        if section_name.to_bytes() == b".nv_fatbin" {
            let section_start = usize::try_from(section.sh_offset).ok()?;
            let section_size = usize::try_from(section.sh_size).ok()?;
            let section_end = section_start.checked_add(section_size)?;
            let range = section_start..section_end;
            bytes.get(range.clone())?;
            Some(range)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{elf_find_fatbin_section, fatbin_frames};

    fn elf_with_sections(
        string_table_offset: u64,
        string_table_size: u64,
        name: u32,
        section_offset: u64,
        section_size: u64,
    ) -> Vec<u8> {
        let mut bytes = vec![0; 400];
        bytes[..4].copy_from_slice(b"\x7fELF");
        bytes[4] = 2;
        bytes[5] = 1;
        bytes[6] = 1;
        bytes[16..18].copy_from_slice(&1u16.to_le_bytes());
        bytes[18..20].copy_from_slice(&62u16.to_le_bytes());
        bytes[20..24].copy_from_slice(&1u32.to_le_bytes());
        bytes[52..54].copy_from_slice(&64u16.to_le_bytes());
        bytes[40..48].copy_from_slice(&64u64.to_le_bytes());
        bytes[58..60].copy_from_slice(&64u16.to_le_bytes());
        bytes[60..62].copy_from_slice(&3u16.to_le_bytes());
        bytes[62..64].copy_from_slice(&1u16.to_le_bytes());
        let string_header = &mut bytes[128..192];
        string_header[4..8].copy_from_slice(&3u32.to_le_bytes());
        string_header[24..32].copy_from_slice(&string_table_offset.to_le_bytes());
        string_header[32..40].copy_from_slice(&string_table_size.to_le_bytes());
        let fatbin_header = &mut bytes[192..256];
        fatbin_header[0..4].copy_from_slice(&name.to_le_bytes());
        fatbin_header[4..8].copy_from_slice(&1u32.to_le_bytes());
        fatbin_header[24..32].copy_from_slice(&section_offset.to_le_bytes());
        fatbin_header[32..40].copy_from_slice(&section_size.to_le_bytes());
        if let Some(end) = (string_table_offset as usize).checked_add(12) {
            if let Some(table) = bytes.get_mut(string_table_offset as usize..end) {
                table.copy_from_slice(b"\0.nv_fatbin\0");
            }
        }
        bytes
    }

    #[test]
    fn finds_fatbin_section_with_bounded_string_table() {
        let bytes = elf_with_sections(256, 12, 1, 380, 4);
        assert_eq!(elf_find_fatbin_section(&bytes), Some(380..384));
    }

    #[test]
    fn rejects_string_table_outside_file() {
        let bytes = elf_with_sections(0x1000, 12, 1, 380, 4);
        assert_eq!(elf_find_fatbin_section(&bytes), None);
    }

    #[test]
    fn rejects_section_name_outside_string_table() {
        let bytes = elf_with_sections(256, 12, 99, 380, 4);
        assert_eq!(elf_find_fatbin_section(&bytes), None);
    }

    #[test]
    fn rejects_unterminated_section_name() {
        let bytes = elf_with_sections(256, 11, 1, 380, 4);
        assert_eq!(elf_find_fatbin_section(&bytes), None);
    }

    #[test]
    fn rejects_fatbin_section_outside_file() {
        let bytes = elf_with_sections(256, 12, 1, 0x1000, 4);
        assert_eq!(elf_find_fatbin_section(&bytes), None);
    }
    #[test]
    fn rejects_offset_overflow() {
        let bytes = elf_with_sections(u64::MAX, 12, 1, 380, 4);
        assert_eq!(elf_find_fatbin_section(&bytes), None);
        let bytes = elf_with_sections(256, 12, 1, u64::MAX, 4);
        assert_eq!(elf_find_fatbin_section(&bytes), None);
        let bytes = elf_with_sections(256, 12, 1, 380, u64::MAX);
        assert_eq!(elf_find_fatbin_section(&bytes), None);
    }
    fn frame(header_size: u16, files_size: u64) -> Vec<u8> {
        let mut bytes = vec![0; usize::from(header_size).max(16) + files_size.min(32) as usize];
        bytes[0..4].copy_from_slice(&cuda_types::dark_api::FatbinHeader::MAGIC);
        bytes[4..6].copy_from_slice(&1u16.to_le_bytes());
        bytes[6..8].copy_from_slice(&header_size.to_le_bytes());
        bytes[8..16].copy_from_slice(&files_size.to_le_bytes());
        bytes
    }

    #[test]
    fn iterates_two_bounded_frames_and_stops_at_trailer() {
        let mut bytes = frame(16, 4);
        bytes.extend(frame(16, 4));
        bytes.extend([0; 16]);
        assert_eq!(
            fatbin_frames(&bytes, 0..bytes.len()).collect::<Vec<_>>(),
            vec![0..20, 20..40]
        );
    }

    #[test]
    fn rejects_zero_or_short_header_without_progress() {
        assert_eq!(fatbin_frames(&frame(0, 0), 0..16).count(), 0);
        assert_eq!(fatbin_frames(&frame(15, 1), 0..16).count(), 0);
    }

    #[test]
    fn accepts_extended_header_in_offset_section() {
        let mut bytes = vec![0; 7];
        bytes.extend(frame(24, 4));
        assert_eq!(
            fatbin_frames(&bytes, 7..bytes.len()).collect::<Vec<_>>(),
            vec![7..35]
        );
    }

    #[test]
    fn rejects_invalid_section_ranges() {
        let bytes = frame(16, 4);
        assert_eq!(fatbin_frames(&bytes, 0..bytes.len() + 1).count(), 0);
        assert_eq!(fatbin_frames(&bytes, 10..5).count(), 0);
        assert_eq!(fatbin_frames(&bytes, 0..15).count(), 0);
    }

    #[test]
    fn stops_after_valid_frame_before_invalid_frame() {
        let mut bytes = frame(16, 4);
        bytes.extend(frame(0, 0));
        assert_eq!(
            fatbin_frames(&bytes, 0..bytes.len()).collect::<Vec<_>>(),
            vec![0..20]
        );
    }

    #[test]
    fn accepts_empty_payload_and_short_trailer() {
        let mut bytes = frame(16, 0);
        bytes.extend([0; 3]);
        assert_eq!(
            fatbin_frames(&bytes, 0..bytes.len()).collect::<Vec<_>>(),
            vec![0..16]
        );
    }

    #[test]
    fn rejects_overflow_and_truncated_frames() {
        assert_eq!(fatbin_frames(&frame(16, u64::MAX), 0..16).count(), 0);
        assert_eq!(fatbin_frames(&frame(16, 4), 0..19).count(), 0);
    }
}

fn fatbin_frames(
    bytes: &[u8],
    section: std::ops::Range<usize>,
) -> impl Iterator<Item = std::ops::Range<usize>> + '_ {
    let mut next_start = section.start;
    let section_end = section.end;
    std::iter::from_fn(move || {
        let remaining = bytes.get(next_start..section_end)?;
        if remaining.len() < mem::size_of::<FatbinHeader>() {
            return None;
        }
        let header = unsafe { remaining.as_ptr().cast::<FatbinHeader>().read_unaligned() };
        if header.magic.to_le_bytes() != FatbinHeader::MAGIC {
            return None;
        }
        let header_size = usize::from(header.header_size);
        if header_size < mem::size_of::<FatbinHeader>() {
            return None;
        }
        let files_size = usize::try_from(header.files_size).ok()?;
        let frame_size = header_size.checked_add(files_size)?;
        if frame_size > remaining.len() {
            return None;
        }
        let frame_end = next_start.checked_add(frame_size)?;
        let frame = next_start..frame_end;
        next_start = frame_end;
        Some(frame)
    })
}
fn extract_from_binary(
    scope: &Scope,
    context: ParallelContext,
    mut file: File,
    header: [u8; 4],
    get_fatbin_section: impl FnOnce(&[u8]) -> Option<std::ops::Range<usize>>,
) -> Option<()> {
    let mut compilation = Arc::new(CompilationContext {
        buffer: Vec::new(),
        progress: context.extract_ptx.clone(),
    });
    let buffer = &mut Arc::get_mut(&mut compilation).unwrap().buffer;
    buffer.extend_from_slice(&header);
    file.read_to_end(buffer).ok()?;
    let fatbin_section = get_fatbin_section(&compilation.buffer)?;
    for fatbin_range in fatbin_frames(&compilation.buffer, fatbin_section) {
        let compilation = compilation.clone();
        let context = context.clone();
        scope.spawn(move |_| {
            (|| {
                unsafe { (context.cuda.cuCtxSetCurrent)(context.cu_ctx) }.ok()?;
                let mut module = CUmodule(ptr::null_mut());
                if unsafe {
                    (context.cuda.cuModuleLoadData)(
                        &mut module,
                        compilation.buffer[fatbin_range].as_ptr().cast(),
                    )
                }
                .is_ok()
                {
                    unsafe { (context.cuda.cuModuleUnload)(module) }.ok()?;
                }
                Some(())
            })();
        });
    }
    Some(())
}

macro_rules! do_nothing {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {};
}

#[cfg(not(windows))]
static LIBCUDA: &str = "libcuda.so";
#[cfg(windows)]
static LIBCUDA: &str = "nvcuda.dll";

macro_rules! dynamic_fns {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        #[derive(Clone)]
        struct CudaContext {
            $(
                #[allow(dead_code)]
                $fn_name: unsafe extern $abi fn ( $($arg_type),* ) -> $ret_type,
            )*
        }

        impl CudaContext {
            unsafe fn new() -> Result<(libloading::Library, Self), Box<dyn Error>> {
                let mut current_exe = std::env::current_exe().map_err(|_| "Current executable not found")?;
                current_exe.pop();
                current_exe.push(LIBCUDA);
                let library = unsafe { libloading::Library::new(current_exe) }?;
                $(
                    let $fn_name = *unsafe { library.get::<unsafe extern $abi fn ($($arg_type),*) -> $ret_type>(concat!(stringify!($fn_name), "\0").as_bytes()) }?;
                )*
                Ok((library, CudaContext { $($fn_name),*}))
            }
        }
    };
}

cuda_macros::cuda_function_declarations! {
    do_nothing,
    dynamic_fns <= [
        cuInit,
        cuCtxCreate_v2,
        cuCtxSetCurrent,
        cuModuleLoadData,
        cuModuleUnload
    ]
}

struct CompilationContext {
    buffer: Vec<u8>,
    progress: ProgressBar,
}

impl Drop for CompilationContext {
    fn drop(&mut self) {
        self.progress.inc(1);
    }
}
