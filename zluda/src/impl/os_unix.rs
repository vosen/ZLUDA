// TODO: remove duplication with zluda_trace
#[link(name = "pthread")]
unsafe extern "C" {
    fn pthread_self() -> std::os::unix::thread::RawPthread;
}

pub(crate) fn current_thread() -> u32 {
    (unsafe { pthread_self() }) as u32
}
