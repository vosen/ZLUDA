pub const PROJECT_SUFFIX: &[u8] = b" [ZLUDA]\0";
const COMPUTE_CAPABILITY_MAJOR: i32 = 8;
const COMPUTE_CAPABILITY_MINOR: i32 = 8;

pub const BLAS_HANDLE_COOKIE: usize = 0x57c3fdb0fd72b08e;

pub fn compute_capability() -> (i32, i32) {
    std::env::var("ZLUDA_CC")
        .or_else(|_| std::env::var("ZLUDA_SM"))
        .ok()
        .and_then(|s| {
            let mut parts = s.split('.');
            let major = parts.next()?.parse::<i32>().ok()?;
            let minor = parts.next()?.parse::<i32>().ok()?;
            Some((major, minor))
        })
        .unwrap_or((COMPUTE_CAPABILITY_MAJOR, COMPUTE_CAPABILITY_MINOR))
}
