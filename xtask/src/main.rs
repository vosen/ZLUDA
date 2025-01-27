use bpaf::{Args, Bpaf, Parser};
use cargo_metadata::{MetadataCommand, Package};
use serde::Deserialize;
use std::{env, ffi::OsString, process::Command};

#[derive(Debug, Clone, Bpaf)]
struct Build {
    #[bpaf(any("CARGO", not_help), many)]
    /// Arguments to pass to cargo, e.g. `--release` for release build
    cargo_arguments: Vec<OsString>,
}

fn not_help(s: OsString) -> Option<OsString> {
    if s == "-h" || s == "--help" {
        None
    } else {
        Some(s)
    }
}

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
enum Options {
    #[bpaf(command)]
    /// Compile ZLUDA (default command)
    Build(#[bpaf(external(build))] Build),
    #[bpaf(command)]
    /// Build ZLUDA package
    Zip,
}

struct Project {
    name: String,
    meta: ZludaMetadata,
}

impl Project {
    fn try_new(p: Package) -> Option<Project> {
        serde_json::from_value::<Option<Metadata>>(p.metadata)
            .unwrap()
            .map(|m| Self {
                name: p.name,
                meta: m.zluda,
            })
    }
}

#[derive(Deserialize)]
struct Metadata {
    zluda: ZludaMetadata,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ZludaMetadata {
    #[serde(default)]
    windows_only: bool,
    #[serde(default)]
    debug_only: bool,
}

fn main() {
    let options = match options().run_inner(Args::current_args()) {
        Ok(b) => b,
        Err(err) => match build().to_options().run_inner(Args::current_args()) {
            Ok(b) => Options::Build(b),
            Err(_) => {
                err.print_message(100);
                std::process::exit(err.exit_code());
            }
        },
    };
    match options {
        Options::Build(b) => compile(b),
        Options::Zip => zip(),
    }
}

fn compile(b: Build) {
    let meta = MetadataCommand::new().no_deps().exec().unwrap();
    let is_release = b
        .cargo_arguments
        .iter()
        .any(|a| a == "-r" || a == "--release");
    let projects = meta.packages.into_iter().filter_map(Project::try_new);
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(&cargo);
    command.arg("build");
    command.arg("--locked");
    for project in projects {
        if project.meta.windows_only && cfg!(not(windows)) {
            continue;
        }
        if project.meta.debug_only && is_release {
            continue;
        }
        command.arg("--package");
        command.arg(&project.name);
    }
    command.args(b.cargo_arguments);
    command.status().unwrap();
}

fn zip() {
    todo!()
}
