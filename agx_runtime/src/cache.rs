//! On-disk cache of compiled AGX modules (object + assembly, the inputs of `Module::load`).
//!
//! Entries are keyed by everything that changes the output: the source text, the chip, the compiler's identity
//! (version and build) and its options. Writes go to a temporary file and are renamed into place, so concurrent
//! processes never see a partial entry. A cache that cannot be read or written is a miss, never an error.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Bumped when the entry layout changes.
const FORMAT: &str = "agx-cache-1";

pub struct Cache {
    dir: PathBuf,
}

/// The parts of a compilation that determine its output.
pub struct Key<'a> {
    pub source: &'a [u8],
    pub chip: &'a str,
    pub compiler: &'a str,
    pub options: &'a [&'a str],
}

impl Key<'_> {
    /// 128-bit FNV-1a over length-prefixed fields.
    pub fn digest(&self) -> String {
        let mut h: u128 = 0x6c62272e07bb014262b821756295c58d;
        let mut feed = |bytes: &[u8]| {
            for &b in (bytes.len() as u64).to_le_bytes().iter().chain(bytes) {
                h ^= b as u128;
                h = h.wrapping_mul(0x0000000001000000000000000000013B);
            }
        };
        feed(FORMAT.as_bytes());
        feed(self.chip.as_bytes());
        feed(self.compiler.as_bytes());
        for o in self.options {
            feed(o.as_bytes());
        }
        feed(self.source);
        format!("{h:032x}")
    }
}

impl Cache {
    /// `$ZLUDA_CACHE_DIR`, else `$XDG_CACHE_HOME/zluda/agx`, else `~/.cache/zluda/agx`. `None` when
    /// `ZLUDA_CACHE_DISABLE` is set or no directory can be found.
    pub fn from_env() -> Option<Cache> {
        if std::env::var_os("ZLUDA_CACHE_DISABLE").is_some() {
            return None;
        }
        let dir = if let Some(d) = std::env::var_os("ZLUDA_CACHE_DIR") {
            PathBuf::from(d)
        } else if let Some(d) = std::env::var_os("XDG_CACHE_HOME") {
            Path::new(&d).join("zluda/agx")
        } else {
            Path::new(&std::env::var_os("HOME")?).join(".cache/zluda/agx")
        };
        Some(Cache::new(dir))
    }

    pub fn new(dir: impl Into<PathBuf>) -> Cache {
        Cache { dir: dir.into() }
    }

    fn paths(&self, key: &Key) -> (PathBuf, PathBuf) {
        let d = key.digest();
        (self.dir.join(format!("{d}.o")), self.dir.join(format!("{d}.s")))
    }

    pub fn get(&self, key: &Key) -> Option<(Vec<u8>, String)> {
        let (o, s) = self.paths(key);
        Some((fs::read(o).ok()?, fs::read_to_string(s).ok()?))
    }

    /// Stores an entry; the assembly is written last, so `get` finds both or neither.
    pub fn put(&self, key: &Key, object: &[u8], assembly: &str) -> io::Result<()> {
        fs::create_dir_all(&self.dir)?;
        let (o, s) = self.paths(key);
        write_atomic(&o, object)?;
        write_atomic(&s, assembly.as_bytes())
    }

    /// Returns the cached entry, or compiles, stores and returns it.
    pub fn get_or_compile<E>(
        &self,
        key: &Key,
        compile: impl FnOnce() -> Result<(Vec<u8>, String), E>,
    ) -> Result<(Vec<u8>, String), E> {
        if let Some(hit) = self.get(key) {
            return Ok(hit);
        }
        let (object, assembly) = compile()?;
        let _ = self.put(key, &object, &assembly);
        Ok((object, assembly))
    }
}

fn write_atomic(path: &Path, data: &[u8]) -> io::Result<()> {
    let tmp = path.with_extension(format!("tmp{}", std::process::id()));
    let mut f = fs::File::create(&tmp)?;
    f.write_all(data)?;
    f.sync_all()?;
    fs::rename(&tmp, path).inspect_err(|_| {
        let _ = fs::remove_file(&tmp);
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key<'a>(source: &'a [u8], chip: &'a str) -> Key<'a> {
        Key { source, chip, compiler: "llvm-agx test", options: &["-O3"] }
    }

    #[test]
    fn digest_depends_on_every_field() {
        let base = key(b"ptx", "agx-g14c").digest();
        assert_eq!(base, key(b"ptx", "agx-g14c").digest());
        assert_ne!(base, key(b"ptx2", "agx-g14c").digest());
        assert_ne!(base, key(b"ptx", "agx-g13g").digest());
        let other = Key { source: b"ptx", chip: "agx-g14c", compiler: "llvm-agx test", options: &["-O2"] };
        assert_ne!(base, other.digest());
        // Length prefixes keep field boundaries apart.
        let a = Key { source: b"", chip: "ab", compiler: "c", options: &[] };
        let b = Key { source: b"", chip: "a", compiler: "bc", options: &[] };
        assert_ne!(a.digest(), b.digest());
    }

    #[test]
    fn compiles_once_then_hits() {
        let dir = std::env::temp_dir().join(format!("agx-cache-test-{}", std::process::id()));
        let cache = Cache::new(&dir);
        let k = key(b"kernel", "agx-g14c");
        let mut calls = 0;
        for _ in 0..2 {
            let r: Result<_, ()> = cache.get_or_compile(&k, || {
                calls += 1;
                Ok((vec![1, 2, 3], "asm".to_owned()))
            });
            assert_eq!(r.unwrap(), (vec![1, 2, 3], "asm".to_owned()));
        }
        assert_eq!(calls, 1);
        assert!(cache.get(&key(b"kernel", "agx-g13g")).is_none());
        fs::remove_dir_all(dir).unwrap();
    }
}
