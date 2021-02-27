pub type LLVMFatalErrorHandler = Option<extern "C" fn(Reason: *const ::libc::c_char)>;

extern "C" {
    /// Install a fatal error handler.
    ///
    /// LLVM will call `exit(1)` if it detects a fatal error. A callback
    /// registered with this function will be invoked before the program is
    /// exited.
    pub fn LLVMInstallFatalErrorHandler(Handler: LLVMFatalErrorHandler);
    /// Reset fatal error handling to the default.
    pub fn LLVMResetFatalErrorHandler();
    /// Enable LLVM's build-in stack trace code.
    ///
    /// This intercepts the OS's crash signals and prints which component
    /// of LLVM you were in at the time of the crash.
    pub fn LLVMEnablePrettyStackTrace();
}
