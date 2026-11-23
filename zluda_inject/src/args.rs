use bpaf::{choice, construct, pure, Bpaf, Parser};
use std::{ffi::OsString, iter, os::windows::ffi::OsStrExt, path::PathBuf};

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
        let mut result = iter::once(self.exe.encode_wide())
            .chain(self.args.iter().map(|s| s.encode_wide()))
            .fold(Vec::new(), |mut acc, cur| {
                if !acc.is_empty() {
                    acc.push(b' ' as u16);
                }
                acc.extend(cur);
                acc
            });
        result.push(0);
        result
    }
}

#[derive(Clone, Debug)]
pub enum ConfigSet {
    Zluda,
    ZludaTrace,
    NvidiaTrace,
    Custom {
        dll_paths32: Vec<Option<LibraryWithPath>>,
        dll_paths64: Vec<Option<LibraryWithPath>>,
    },
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
        custom_args()
            .map(|(dll_paths32, dll_paths64)| ConfigSet::Custom {
                dll_paths32,
                dll_paths64,
            })
            .boxed(),
    ])
    .fallback(ConfigSet::Zluda)
}

pub fn custom_args() -> impl Parser<(Vec<Option<LibraryWithPath>>, Vec<Option<LibraryWithPath>>)> {
    fn libraries_to_parser(
        libraries: &'static [zluda_windows::LibraryInfo],
    ) -> impl Parser<Vec<Option<LibraryWithPath>>> {
        libraries.iter().fold(
            pure(Vec::with_capacity(libraries.len())).boxed(),
            |parser, library| {
                let short_name = library.short_name;
                let display_name = library.display_name();
                let arg = bpaf::long(short_name)
                    .help(&*format!("Path to file {display_name}"))
                    .argument::<PathBuf>("PATH")
                    .optional()
                    .hide()
                    .map(|path| path.map(|path| LibraryWithPath { library, path }));
                construct!(parser, arg)
                    .map(|(mut acc, cur)| {
                        acc.push(cur);
                        acc
                    })
                    .boxed()
            },
        )
    }
    let parser32 = libraries_to_parser(&zluda_windows::LIBRARIES32);
    let parser64 = libraries_to_parser(&zluda_windows::LIBRARIES64);
    construct!(parser32, parser64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bpaf::ParseFailure;

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
        assert!(matches!(result, ConfigSet::Custom { .. }));
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

    #[test]
    fn empty_args() {
        let args: [&str; 0] = [];
        let parser = arguments().to_options();
        let empty_parse = parser.run_inner(&args[..]);
        assert!(matches!(empty_parse, Err(ParseFailure::Stderr(_))));
    }

    #[test]
    fn parse_positional_args() {
        let args = ["--", "foobar.exe", "--param"];
        let parser = arguments().to_options();
        let parsed_args = parser.run_inner(&args[..]).unwrap();
        assert!(matches!(parsed_args.paths, ConfigSet::Zluda));
        assert_eq!(parsed_args.exe, OsString::from("foobar.exe"));
        assert_eq!(parsed_args.args, [OsString::from("--param")]);
    }
}
