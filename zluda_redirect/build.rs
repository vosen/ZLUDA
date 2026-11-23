fn main() {
    println!("cargo:rustc-cdylib-link-arg=/export:DetourFinishHelperProcess,@1,NONAME");
}
