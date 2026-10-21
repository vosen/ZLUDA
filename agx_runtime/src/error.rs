//! Failures of the AGX runtime and the CUDA error codes they map to.
//!
//! Every ioctl error carries its stage and errno (`StageError`). A job that the kernel failed after submission (a GPU
//! fault, or the firmware killing it) carries the fence status (`GpuError`). Which errno the Asahi kernel uses for
//! each case is not measured yet (AGX Performance Simulator `tasks/ptx-re-blockers.md`, B6 and B20), so the mapping
//! below only separates timeouts from other failures.

use std::fmt;
use std::io;

#[derive(Debug)]
pub struct StageError {
    pub stage: String,
    pub source: io::Error,
}

impl fmt::Display for StageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.stage, self.source)
    }
}

impl std::error::Error for StageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

/// A submitted job whose fence the kernel signalled with an error.
#[derive(Debug)]
pub struct GpuError {
    /// Negative errno.
    pub fence_status: i32,
    /// The timeline point of the failed command, when known.
    pub point: Option<u64>,
}

impl fmt::Display for GpuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GPU job failed: {}", io::Error::from_raw_os_error(-self.fence_status))
    }
}

impl std::error::Error for GpuError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failure {
    /// The host stopped waiting (`Queue::timeout`) or the job was killed for running too long.
    Timeout,
    /// The kernel failed the job: a fault, or the firmware or kernel cancelling it.
    JobFailed,
    /// The kernel refused to start the job, e.g. because the device is lost.
    DeviceLost,
    OutOfMemory,
    InvalidValue,
    Other,
}

/// The errno of an ioctl error, or the negated fence status of a failed job.
pub fn errno(e: &io::Error) -> Option<i32> {
    if let Some(inner) = e.get_ref() {
        if let Some(s) = inner.downcast_ref::<StageError>() {
            return s.source.raw_os_error();
        }
        if let Some(g) = inner.downcast_ref::<GpuError>() {
            return Some(-g.fence_status);
        }
    }
    e.raw_os_error()
}

pub fn classify(e: &io::Error) -> Failure {
    let job = e.get_ref().is_some_and(|i| i.is::<GpuError>());
    match errno(e) {
        Some(libc::ETIME) | Some(libc::ETIMEDOUT) => Failure::Timeout,
        _ if job => Failure::JobFailed,
        Some(libc::ENOMEM) | Some(libc::ENOSPC) => Failure::OutOfMemory,
        Some(libc::EINVAL) | Some(libc::E2BIG) | Some(libc::ERANGE) => Failure::InvalidValue,
        Some(libc::ENODEV) | Some(libc::EIO) | Some(libc::ECANCELED) => Failure::DeviceLost,
        _ => Failure::Other,
    }
}

/// The `CUresult` for a failure.
pub fn cuda_error(e: &io::Error) -> u32 {
    match classify(e) {
        Failure::Timeout => 702,     // CUDA_ERROR_LAUNCH_TIMEOUT
        Failure::JobFailed => 719,   // CUDA_ERROR_LAUNCH_FAILED
        Failure::DeviceLost => 719,  // sticky, like CUDA after a failed launch
        Failure::OutOfMemory => 2,   // CUDA_ERROR_OUT_OF_MEMORY
        Failure::InvalidValue => 1,  // CUDA_ERROR_INVALID_VALUE
        Failure::Other => 999,       // CUDA_ERROR_UNKNOWN
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stage(errno: i32) -> io::Error {
        let e = io::Error::from_raw_os_error(errno);
        io::Error::new(e.kind(), StageError { stage: "submit".into(), source: e })
    }

    #[test]
    fn ioctl_errors_keep_their_errno() {
        assert_eq!(errno(&stage(libc::ETIME)), Some(libc::ETIME));
        assert_eq!(cuda_error(&stage(libc::ETIME)), 702);
        assert_eq!(cuda_error(&stage(libc::ENOMEM)), 2);
        assert_eq!(cuda_error(&stage(libc::EINVAL)), 1);
        assert_eq!(classify(&stage(libc::ENODEV)), Failure::DeviceLost);
        assert_eq!(stage(libc::EINVAL).to_string(), format!("submit: {}", io::Error::from_raw_os_error(libc::EINVAL)));
    }

    #[test]
    fn failed_jobs_are_launch_failures_unless_timed_out() {
        let job = |s| io::Error::other(GpuError { fence_status: s, point: None });
        assert_eq!(classify(&job(-libc::EIO)), Failure::JobFailed);
        assert_eq!(cuda_error(&job(-libc::EIO)), 719);
        assert_eq!(cuda_error(&job(-libc::ETIMEDOUT)), 702);
    }
}
