//! LLVM's BLAKE3 implementation.
//! Original BLAKE3 C API: <https://github.com/BLAKE3-team/BLAKE3/tree/1.3.1/c>

pub const LLVM_BLAKE3_VERSION_STRING: &str = "1.3.1";
pub const LLVM_BLAKE3_KEY_LEN: usize = 32;
pub const LLVM_BLAKE3_OUT_LEN: usize = 32;
pub const LLVM_BLAKE3_BLOCK_LEN: usize = 64;
pub const LLVM_BLAKE3_CHUNK_LEN: usize = 1024;
pub const LLVM_BLAKE3_MAX_DEPTH: usize = 54;

/// This struct is a private implementation detail. It has to be here because
/// it's part of llvm_blake3_hasher below.
#[repr(C)]
struct llvm_blake3_chunk_state {
    cv: [u32; 8],
    chunk_counter: u64,
    buf: [u8; LLVM_BLAKE3_BLOCK_LEN],
    buf_len: u8,
    blocks_compressed: u8,
    flags: u8,
}

#[repr(C)]
pub struct llvm_blake3_hasher {
    key: [u32; 8],
    chunk: llvm_blake3_chunk_state,
    cv_stack_len: u8,
    /// The stack size is MAX_DEPTH + 1 because we do lazy merging. For example,
    /// with 7 chunks, we have 3 entries in the stack. Adding an 8th chunk
    /// requires a 4th entry, rather than merging everything down to 1, because we
    /// don't know whether more input is coming. This is different from how the
    /// reference implementation does things.
    cv_stack: [u8; (LLVM_BLAKE3_MAX_DEPTH + 1) * LLVM_BLAKE3_OUT_LEN],
}

extern "C" {
    pub fn llvm_blake3_version() -> *const ::libc::c_char;
    pub fn llvm_blake3_hasher_init(hasher: *mut llvm_blake3_hasher);
    pub fn llvm_blake3_hasher_init_keyed(hasher: *mut llvm_blake3_hasher, key: *const u8);
    pub fn llvm_blake3_hasher_init_derive_key(
        hasher: *mut llvm_blake3_hasher,
        context: *const ::libc::c_char,
    );
    pub fn llvm_blake3_hasher_init_derive_key_raw(
        hasher: *mut llvm_blake3_hasher,
        context: *const ::libc::c_char,
        context_len: usize,
    );
    pub fn llvm_blake3_hasher_update(
        hasher: *mut llvm_blake3_hasher,
        input: *const ::libc::c_void,
        input_len: usize,
    );
    pub fn llvm_blake3_hasher_finalize(
        hasher: *mut llvm_blake3_hasher,
        out: *mut u8,
        out_len: usize,
    );
    pub fn llvm_blake3_hasher_finalize_seek(
        hasher: *mut llvm_blake3_hasher,
        seek: u64,
        out: *mut u8,
        out_len: usize,
    );
    pub fn llvm_blake3_hasher_reset(hasher: *mut llvm_blake3_hasher);
}
