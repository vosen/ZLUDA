use bpaf::{Bpaf, Parser};
use cuda_types::cuda::{CUcontext, CUmodule, CUresult};
use goblin::{
    container::{Ctx, Endian},
    elf::SectionHeader,
};
use indicatif::ProgressBar;
use rayon::Scope;
use std::{
    error::Error,
    ffi::{c_uint, CStr},
    fs::File,
    io::Read,
    mem,
    path::{Path, PathBuf},
    ptr,
};

#[derive(Clone, Debug, Bpaf)]
struct Arguments {
    /// CUDA device ID to use for compilation
    #[bpaf(long, short('d'), fallback(0))]
    device: u32,

    /// Follow symbolic links when traversing directories
    follow_links: bool,

    /// Directory or file to precompile CUDA code from
    #[bpaf(positional)]
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = arguments().run();
    let mut current_exe = std::env::current_exe()?;
    current_exe.pop();
    current_exe.push("libcuda.so");
    let library = unsafe { libloading::Library::new(current_exe) }?;
    let cu_init =
        unsafe { library.get::<unsafe extern "system" fn(c_uint) -> CUresult>(b"cuInit\0") }?;
    let cu_ctx_create = unsafe {
        library.get::<unsafe extern "system" fn(
            pctx: *mut cuda_types::cuda::CUcontext,
            flags: ::core::ffi::c_uint,
            dev: cuda_types::cuda::CUdevice,
        ) -> CUresult>(b"cuCtxCreate_v2\0")
    }?;
    let cu_ctx_set_current = unsafe {
        library.get::<unsafe extern "system" fn(ctx: cuda_types::cuda::CUcontext) -> CUresult>(
            b"cuCtxSetCurrent\0",
        )
    }?;
    let cu_module_load_data = unsafe {
        library.get::<unsafe extern "system" fn(
            module: *mut cuda_types::cuda::CUmodule,
            image: *const ::core::ffi::c_void,
        ) -> CUresult>(b"cuModuleLoadData\0")
    }?;
    unsafe { cu_init(0) }.map_err(|_| "Failed to initialize CUDA")?;
    let mut ctx = CUcontext(ptr::null_mut());
    unsafe { cu_ctx_create(&mut ctx, 0, config.device as i32) }
        .map_err(|_| "Failed to create CUDA context")?;
    let pool = rayon::ThreadPoolBuilder::new().build()?;
    let progress = indicatif::MultiProgress::new();
    let file_scan = progress.insert(0, ProgressBar::no_length());
    file_scan.set_style(
        indicatif::ProgressStyle::with_template(
            "1/2 Building file list... {spinner} {pos} file(s) scanned",
        )?
        .tick_chars("/|\\- "),
    );
    let extract_ptx = progress.insert(1, ProgressBar::new(0));
    extract_ptx.set_style(indicatif::ProgressStyle::with_template(
        "2/2 {wide_bar} {pos}/{len} file(s) compiled",
    )?);
    pool.scope(|scope| {
        scope.spawn(|scope| {
            read_files_from_path(
                scope,
                &config.input,
                config.follow_links,
                ctx,
                *cu_module_load_data,
                *cu_ctx_set_current,
                file_scan,
                extract_ptx.clone(),
            )
        });
    });
    extract_ptx.finish();
    Ok(())
}

fn read_files_from_path(
    scope: &Scope,
    path: &PathBuf,
    follow_links: bool,
    ctx: CUcontext,
    cu_module_load_data: unsafe extern "system" fn(
        module: *mut cuda_types::cuda::CUmodule,
        image: *const ::core::ffi::c_void,
    ) -> CUresult,
    cu_ctx_set_current: unsafe extern "system" fn(ctx: cuda_types::cuda::CUcontext) -> CUresult,
    file_scan: ProgressBar,
    extract_ptx: ProgressBar,
) {
    let mut extract_counter = 0;
    if path.is_file() {
        file_scan.set_position(1);
        check_if_elf_and_enqueue(
            ctx,
            cu_module_load_data,
            cu_ctx_set_current,
            &mut extract_counter,
            scope,
            path,
        );
    } else if path.is_dir() {
        let mut pos = 0;
        walkdir::WalkDir::new(path)
            .follow_links(follow_links)
            .into_iter()
            .filter_map(|dir_entry| dir_entry.ok())
            .for_each(|entry| {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    pos += 1;
                    file_scan.set_position(pos);
                    check_if_elf_and_enqueue(
                        ctx,
                        cu_module_load_data,
                        cu_ctx_set_current,
                        &mut extract_counter,
                        scope,
                        entry_path,
                    );
                }
            });
    }
    extract_ptx.inc_length(extract_counter);
    file_scan.finish();
}

fn check_if_elf_and_enqueue(
    ctx: CUcontext,
    cu_module_load_data: unsafe extern "system" fn(
        module: *mut cuda_types::cuda::CUmodule,
        image: *const ::core::ffi::c_void,
    ) -> CUresult,
    cu_ctx_set_current: unsafe extern "system" fn(ctx: cuda_types::cuda::CUcontext) -> CUresult,
    extract_count: &mut u64,
    scope: &Scope,
    path: &Path,
) {
    if let Ok(mut file) = File::open(path) {
        let mut header = [0u8; 4];
        if let Ok(bytes) = file.read(&mut header) {
            if bytes < 4 {
                return;
            }
            if &header == goblin::elf::header::ELFMAG {
                *extract_count += 1;
                scope.spawn(move |_| {
                    extract_from_elf(ctx, cu_module_load_data, cu_ctx_set_current, file);
                });
            }
        }
    }
}

fn extract_from_elf(
    cu_ctx: CUcontext,
    cu_module_load_data: unsafe extern "system" fn(
        module: *mut cuda_types::cuda::CUmodule,
        image: *const ::core::ffi::c_void,
    ) -> CUresult,
    cu_ctx_set_current: unsafe extern "system" fn(ctx: cuda_types::cuda::CUcontext) -> CUresult,
    mut file: File,
) -> Option<()> {
    let mut buf = Vec::new();
    buf.extend_from_slice(goblin::elf::header::ELFMAG);
    file.read_to_end(&mut buf).ok()?;
    let ctx = Ctx::new(goblin::container::Container::Big, Endian::Little);
    let header = goblin::elf64::header::Header::parse(&buf).ok()?;
    let section_headers =
        SectionHeader::parse(&buf, header.e_shoff as usize, header.e_shnum as usize, ctx).ok()?;
    let string_table_section = &section_headers[header.e_shstrndx as usize];
    let string_table_start = string_table_section.sh_offset as usize;
    let fatbin_data = section_headers.into_iter().find_map(|section| {
        let section_name = unsafe {
            CStr::from_ptr(
                buf.as_ptr()
                    .add(string_table_start)
                    .add(section.sh_name as usize)
                    .cast(),
            )
        };
        if section_name.to_bytes() == b".nv_fatbin" {
            let section_data =
                &buf[section.sh_offset as usize..(section.sh_offset + section.sh_size) as usize];
            Some(section_data)
        } else {
            None
        }
    });
    let fatbin_data = unwrap_or::unwrap_some_or!(fatbin_data, return None);
    unsafe { cu_ctx_set_current(cu_ctx) }.ok()?;
    let mut module = CUmodule(ptr::null_mut());
    unsafe {
        cu_module_load_data(
            &mut module,
            fatbin_data.as_ptr() as *const ::core::ffi::c_void,
        )
    }
    .ok()?;
    Some(())
}
