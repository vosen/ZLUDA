use bpaf::{Args, Bpaf, Parser};
use cargo_metadata::{MetadataCommand, Package};
use serde::Deserialize;
use std::{env, ffi::OsString, process::Command};

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

// We need to sniff out some args passed to cargo to understand how to create
// symlinks (should they go into `target/debug`, `target/release` or custom)
#[derive(Debug, Clone, Bpaf)]
struct Cargo {
    #[bpaf(switch, long, short)]
    release: Option<bool>,
    #[bpaf(long)]
    profile: Option<String>,
    #[bpaf(any("", Some), many)]
    _unused: Vec<OsString>,
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
    #[serde(default)]
    linux_symlinks: Vec<String>,
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
    let profile = sniff_out_profile_name(&b.cargo_arguments);
    let meta = MetadataCommand::new().no_deps().exec().unwrap();
    let target_directory = meta.target_directory.into_std_path_buf();
    let projects = meta
        .packages
        .into_iter()
        .filter_map(Project::try_new)
        .collect::<Vec<_>>();
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(&cargo);
    command.arg("build");
    command.arg("--locked");
    for project in projects.iter() {
        if project.meta.windows_only && cfg!(not(windows)) {
            continue;
        }
        if project.meta.debug_only && profile != "debug" {
            continue;
        }
        command.arg("--package");
        command.arg(&project.name);
    }
    command.args(b.cargo_arguments);
    assert!(command.status().unwrap().success());
    os::make_symlinks(target_directory, projects, profile);
}

fn sniff_out_profile_name(b: &[OsString]) -> String {
    let parsed_cargo_arguments = cargo().to_options().run_inner(b);
    match parsed_cargo_arguments {
        Ok(Cargo {
            release: Some(true),
            ..
        }) => "release".to_string(),
        Ok(Cargo {
            profile: Some(profile),
            ..
        }) => profile,
        _ => "debug".to_string(),
    }
}

fn zip() {
    todo!()
}

#[cfg(unix)]
mod os {
    pub fn make_symlinks(
        target_directory: std::path::PathBuf,
        projects: Vec<super::Project>,
        profile: String,
    ) {
        use std::fs;
        use std::os::unix::fs as unix_fs;
        for project in projects.iter() {
            let mut target = target_directory.clone();
            target.extend([&*profile, &format!("lib{}.so", project.name)]);
            for source in project.meta.linux_symlinks.iter() {
                let mut link = target_directory.clone();
                link.extend([&*profile, source]);
                let mut dir = link.clone();
                assert!(dir.pop());
                fs::create_dir_all(dir).unwrap();
                fs::remove_file(&link).ok();
                unix_fs::symlink(&target, link).unwrap();
            }
        }
    }
}

#[cfg(not(unix))]
mod os {
    pub fn make_symlinks(
        target_directory: std::path::PathBuf,
        projects: Vec<super::Project>,
        profile: String,
    ) {
    }
}
