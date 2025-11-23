pub fn try_load_library(name: &str) -> Result<libloading::Library, libloading::Error> {
    unsafe{ libloading::Library::new(name) }
}