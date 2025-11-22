use bpaf::{Bpaf, Parser};
use cuda_types::{
    cuda::{CUcontext, CUmodule},
    dark_api::FatbinHeader,
};
use goblin::{
    container::{Ctx, Endian},
    elf::SectionHeader,
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
            if &header == goblin::elf::header::ELFMAG {
                let parallel_context = context.parallel_context.clone();
                context.scope.spawn(move |scope| {
                    extract_from_elf(scope, parallel_context, file);
                });
                return true;
            }
        }
    }
    false
}

fn extract_from_elf(scope: &Scope, context: ParallelContext, mut file: File) -> Option<()> {
    let mut compilation = Arc::new(CompilationContext {
        buffer: Vec::new(),
        progress: context.extract_ptx.clone(),
    });
    let buffer = &mut Arc::get_mut(&mut compilation).unwrap().buffer;
    buffer.extend_from_slice(goblin::elf::header::ELFMAG);
    file.read_to_end(buffer).ok()?;
    let goblin_ctx = Ctx::new(goblin::container::Container::Big, Endian::Little);
    let header = goblin::elf64::header::Header::parse(&compilation.buffer).ok()?;
    let section_headers = SectionHeader::parse(
        &compilation.buffer,
        header.e_shoff as usize,
        header.e_shnum as usize,
        goblin_ctx,
    )
    .ok()?;
    let string_table_section = section_headers.get(header.e_shstrndx as usize)?;
    let string_table_start = string_table_section.sh_offset as usize;
    let fatbin_section = section_headers.into_iter().find_map(|section| {
        let section_name = CStr::from_bytes_until_nul(
            &compilation.buffer[string_table_start + section.sh_name as usize..],
        )
        .ok()?;
        if section_name.to_bytes() == b".nv_fatbin" {
            let range = section.sh_offset as usize
                ..(section.sh_offset.saturating_add(section.sh_size)) as usize;
            Some(range)
        } else {
            None
        }
    });
    let mut fatbin_range = unwrap_or::unwrap_some_or!(fatbin_section, return None);
    loop {
        if fatbin_range.len() < mem::size_of::<FatbinHeader>() {
            break;
        }
        let header = unsafe {
            compilation.buffer[fatbin_range.clone()]
                .as_ptr()
                .cast::<FatbinHeader>()
                .read_unaligned()
        };
        if header.magic.to_le_bytes() != FatbinHeader::MAGIC {
            break;
        }
        {
            let compilation = compilation.clone();
            let fatbin_range = fatbin_range.clone();
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
        fatbin_range.start = fatbin_range
            .start
            .saturating_add(header.header_size as usize)
            .saturating_add(header.files_size as usize);
    }
    Some(())
}

macro_rules! do_nothing {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {};
}

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
                current_exe.push("libcuda.so");
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
