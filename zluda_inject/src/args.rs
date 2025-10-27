use bpaf::{choice, construct, pure, Parser};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum ConfigSet {
    Zluda,
    ZludaTrace,
    NvidiaTrace,
    Custom(Vec<CustomLibrary>),
}

struct CustomLibraryList {
    custom_libs: Vec<CustomLibrary>,
}

#[derive(Clone, Debug)]
struct CustomLibrary {
    library: &'static zluda_windows::LibraryInfo,
    path: Option<PathBuf>,
}

pub fn config_sets() -> impl Parser<ConfigSet> {
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

pub fn custom_args() -> Box<dyn Parser<Vec<CustomLibrary>>> {
    zluda_windows::LIBRARIES
        .iter()
        .fold(
            pure(Vec::with_capacity(zluda_windows::LIBRARIES.len())).boxed(),
            |parser, library| {
                let dlls = library
                    .ascii_names
                    .iter()
                    .map(|&name| name.to_lowercase())
                    .collect::<Vec<_>>()
                    .join(" or ");
                let arg = bpaf::long(library.short_name)
                    .help(&*format!("Path to {dlls}"))
                    .argument::<PathBuf>("PATH")
                    .optional()
                    .map(|path| CustomLibrary { library, path });
                construct!(parser, arg)
                    .map(|(mut acc, cur)| {
                        acc.push(cur);
                        acc
                    })
                    .boxed()
            },
        )
        .group_help("Manual configuration of library paths:")
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fail_on_duplicate_config_sets() {
        let args = ["--zluda", "--zluda-trace"];
        let parser = config_sets().to_options();
        parser.run_inner(&args[..]).unwrap_err();
    }

    #[test]
    fn succeed_on_empty_config_set() {
        let args: [&'static str; 0] = [];
        let parser = config_sets().to_options();
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
        let parser = config_sets().to_options();
        let result = parser.run_inner(&args[..]).unwrap();
        assert!(matches!(result, ConfigSet::Custom(_)));
    }

    #[test]
    fn custom_arg_set_fails_on_invalid_arg() {
        let args = [
            "--nvcuda=c:\\path\\to\\nvcuda.dll",
            "--cufoobar=c:\\path\\to\\sparse.dll",
        ];
        let parser = config_sets().to_options();
        let err = parser.run_inner(&args[..]).unwrap_err();
        assert!(format!("{:?}", err).contains("cufoobar"));
    }
}
