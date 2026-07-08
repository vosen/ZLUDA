use std::path::PathBuf;

pub fn self_path() -> Option<PathBuf> {
    zluda_windows::get_module_path_for_function(self_path as *const () as usize).map(PathBuf::from)
}
