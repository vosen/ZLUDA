// HACK ALERT
// This buidl script has been copy-pasted from cl-sys to avoid CUDA libraries
// overriding path to OpenCL

 fn main() {
    if cfg!(windows) {
        let known_sdk = [
            // E.g. "c:\Program Files (x86)\Intel\OpenCL SDK\lib\x86\"
            ("INTELOCLSDKROOT", "x64", "x86"),
            // E.g. "c:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v8.0\lib\Win32\"
            ("CUDA_PATH", "x64", "Win32"),
            // E.g. "C:\Program Files (x86)\AMD APP SDK\3.0\lib\x86\"
            ("AMDAPPSDKROOT", "x86_64", "x86"),            
        ];

        for info in known_sdk.iter() {
            if let Ok(sdk) = std::env::var(info.0) {
                let mut path = std::path::PathBuf::from(sdk);
                path.push("lib");
                path.push(if cfg!(target_arch="x86_64") { info.1 } else { info.2 });
                println!("cargo:rustc-link-search=native={}", path.display());
            }
        }

        println!("cargo:rustc-link-search=native=C:\\Program Files (x86)\\OCL_SDK_Light\\lib\\x86_64");        
    }
}