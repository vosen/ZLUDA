use bpaf::{choice, construct, pure, Bpaf, Parser};
use std::{ffi::OsString, os::windows::ffi::OsStrExt, path::PathBuf};

/// Launch an application and redirect its CUDA calls
#[derive(Debug, Clone, Bpaf)]
pub struct Arguments {
    /// Configuration set defining which CUDA libraries to redirect and their paths
    #[bpaf(external(config_set))]
    pub paths: ConfigSet,

    /// Executable to be injected with custom CUDA libraries
    #[bpaf(positional("EXE"))]
    pub exe: OsString,

    /// Arguments to the executable
    #[bpaf(positional("ARGS"))]
    pub args: Vec<OsString>,
}

impl Arguments {
    pub fn command_line_zero_terminated(&self) -> Vec<u16> {
        let mut cmdline = Vec::with_capacity(1 + self.exe.len());
        cmdline.extend(self.exe.encode_wide());
        cmdline.push(' ' as u16);
        for arg in self.args.iter() {
            cmdline.extend(arg.encode_wide());
        }
        cmdline.push(0);
        cmdline
    }
}

#[derive(Clone, Debug)]
pub enum ConfigSet {
    Zluda,
    ZludaTrace,
    NvidiaTrace,
    Custom(Vec<LibraryWithPath>),
}

#[derive(Clone, Debug)]
pub struct LibraryWithPath {
    pub library: &'static zluda_windows::LibraryInfo,
    pub path: PathBuf,
}

pub fn config_set() -> impl Parser<ConfigSet> {
    let mut temp_dir = std::env::temp_dir();
    temp_dir.push("zluda");
    let temp_dir = temp_dir.display();
    let zluda = bpaf::long("zluda")
        .help("Redirect CUDA calls to ZLUDA (default)")
        .switch()
        .map(|_| ConfigSet::Zluda)
        .boxed();
    let zluda_trace = bpaf::long("zluda-trace")
        .help(&*format!(
            "Redirect CUDA calls to ZLUDA and log to {temp_dir}"
        ))
        .switch()
        .map(|_| ConfigSet::ZludaTrace)
        .boxed();
    let nvidia_trace: Box<dyn Parser<_>> = Box::new(
        bpaf::long("nvidia-trace")
            .help(&*format!("Intercept CUDA calls and log to {temp_dir}"))
            .switch()
            .map(|_| ConfigSet::NvidiaTrace)
            .boxed(),
    );
    choice([
        zluda,
        zluda_trace,
        nvidia_trace,
        custom_args().map(ConfigSet::Custom).boxed(),
    ])
    .fallback(ConfigSet::Zluda)
}

pub fn custom_args() -> impl Parser<Vec<LibraryWithPath>> {
    zluda_windows::LIBRARIES.iter().fold(
        pure(Vec::with_capacity(zluda_windows::LIBRARIES.len())).boxed(),
        |parser, library| {
            let dlls = library.ascii_name;
            let arg = bpaf::long(library.short_name)
                .help(&*format!("Path to file {dlls}"))
                .argument::<PathBuf>("PATH")
                .optional()
                .hide()
                .map(|path| path.map(|path| LibraryWithPath { library, path }));
            construct!(parser, arg)
                .map(|(mut acc, cur)| {
                    if let Some(cur) = cur {
                        acc.push(cur);
                    }
                    acc
                })
                .boxed()
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fail_on_duplicate_config_sets() {
        let args = ["--zluda", "--zluda-trace"];
        let parser = config_set().to_options();
        parser.run_inner(&args[..]).unwrap_err();
    }

    #[test]
    fn succeed_on_empty_config_set() {
        let args: [&'static str; 0] = [];
        let parser = config_set().to_options();
        assert!(matches!(
            parser.run_inner(&args[..]).unwrap(),
            ConfigSet::Zluda
        ));
    }

    #[test]
    fn custom_arg_set_parses_incomplete() {
        let args = [
            "--nvcuda=c:\\path\\to\\nvcuda.dll",
            "--cusparse=c:\\path\\to\\sparse.dll",
        ];
        let parser = config_set().to_options();
        let result = parser.run_inner(&args[..]).unwrap();
        assert!(matches!(result, ConfigSet::Custom(_)));
    }

    #[test]
    fn custom_arg_set_fails_on_invalid_arg() {
        let args = [
            "--nvcuda=c:\\path\\to\\nvcuda.dll",
            "--cufoobar=c:\\path\\to\\sparse.dll",
        ];
        let parser = config_set().to_options();
        let err = parser.run_inner(&args[..]).unwrap_err();
        assert!(format!("{:?}", err).contains("cufoobar"));
    }
}
