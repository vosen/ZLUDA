use hip_common::{
    cache::{KernelExtendedData, KernelRepository},
    unwrap_or_return, CompilationMode,
};
use static_assertions::assert_impl_one;
use std::{borrow::Cow, ffi::CStr, path::Path};

pub(crate) struct KernelCache(KernelRepository<NoExtendedData>);
assert_impl_one!(KernelCache: Sync);

impl KernelCache {
    pub(crate) const CACHE_FILE: &'static str = "zluda.db";

    pub(crate) fn new(cache_dir: &Path) -> Option<Self> {
        let mut file = cache_dir.to_path_buf();
        file.push(Self::CACHE_FILE);
        Some(Self(KernelRepository::new(Some(file)).ok()?))
    }

    pub(crate) fn save_program(
        &self,
        compiler_version: &str,
        device: &CStr,
        ptx_modules: &[Cow<'_, str>],
        compilation_mode: CompilationMode,
        binary: &[u8],
    ) {
        let now = unwrap_or_return!(KernelRepository::<NoExtendedData>::now());
        let mut hasher = blake3::Hasher::new();
        for module in ptx_modules {
            hasher.update(module.as_bytes());
        }
        let hash = hasher.finalize().to_hex();
        let git_hash = env!("VERGEN_GIT_SHA");
        self.0
            .save_program(
                now,
                hash.as_str(),
                compiler_version,
                git_hash,
                device,
                binary,
                rusqlite::params![compilation_mode as u8],
            )
            .ok();
    }

    pub(crate) fn try_load_program(
        &self,
        compiler_version: &str,
        device: &CStr,
        ptx_modules: &[Cow<'_, str>],
        compilation_mode: CompilationMode,
    ) -> Option<Vec<u8>> {
        let now = KernelRepository::<NoExtendedData>::now().ok()?;
        let mut hasher = blake3::Hasher::new();
        for module in ptx_modules {
            hasher.update(module.as_bytes());
        }
        let hash = hasher.finalize().to_hex();
        let git_hash = env!("VERGEN_GIT_SHA");
        Some(
            self.0
                .try_load_program(
                    now,
                    hash.as_str(),
                    compiler_version,
                    git_hash,
                    device,
                    rusqlite::params![compilation_mode as u8],
                )
                .ok()
                .flatten()?,
        )
    }
}

struct NoExtendedData;

impl KernelExtendedData for NoExtendedData {
    const INPUT_COLUMNS: &'static [[&'static str; 2]] = &[["compilation_mode", "INTEGER NOT NULL"]];
}
