use bpaf::{Bpaf, Parser};
use indicatif::ProgressBar;
use rayon::Scope;
use std::{
    error::Error,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

const ELF_MAGIC: [u8; 4] = [0x7F, b'E', b'L', b'F'];

#[derive(Clone, Debug, Bpaf)]
struct Arguments {
    /// Follow symbolic links when traversing directories
    follow_links: bool,

    /// Directory or file to recompile CUDA code from
    #[bpaf(positional)]
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = arguments().run();
    let pool = rayon::ThreadPoolBuilder::new().build()?;
    let progress = indicatif::MultiProgress::new();
    let file_scan = progress.insert(0, ProgressBar::no_length());
    file_scan.set_style(
        indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] Building file list... {spinner} {pos} file(s) scanned",
        )?
        .tick_chars("/|\\- "),
    );
    let extract_ptx = progress.insert(1, ProgressBar::new(0));
    extract_ptx.set_style(indicatif::ProgressStyle::with_template(
        "[{elapsed_precise}] {wide_bar} {pos}/{len} file(s) compiled",
    )?);
    pool.scope(|scope| {
        scope.spawn(|scope| {
            read_files_from_path(
                scope,
                &config.input,
                config.follow_links,
                file_scan,
                extract_ptx,
            )
        });
    });
    Ok(())
}

fn read_files_from_path(
    scope: &Scope,
    path: &PathBuf,
    follow_links: bool,
    file_scan: ProgressBar,
    extract_ptx: ProgressBar,
) {
    if path.is_file() {
        file_scan.set_position(1);
        check_if_elf_and_enqueue(extract_ptx, scope, path);
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
                    check_if_elf_and_enqueue(extract_ptx.clone(), scope, entry_path);
                }
            });
    }
    file_scan.finish();
}

fn check_if_elf_and_enqueue(extract_ptx: ProgressBar, scope: &Scope, path: &Path) {
    if let Ok(mut file) = File::open(path) {
        let mut header = [0u8; 4];
        if let Ok(bytes) = file.read(&mut header) {
            if bytes < 4 {
                return;
            }
            if header == ELF_MAGIC {
                extract_ptx.inc_length(1);
                scope.spawn(|_| extract_from_elf(extract_ptx, file));
            }
        }
    }
}

fn extract_from_elf(extract_ptx: ProgressBar, file: File) {
    std::thread::sleep(std::time::Duration::from_secs(1));
    extract_ptx.inc(1);
    //println!("Extracting from ELF file: {:?}", file);
}
