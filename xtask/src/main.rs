use bpaf::{Args, Bpaf, Parser};
use cargo_metadata::{MetadataCommand, Package};
use serde::Deserialize;
use std::{env, ffi::OsString, fs::File, path::PathBuf, process::Command};
use zip::{write::SimpleFileOptions, ZipWriter};

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
enum Options {
    #[bpaf(command)]
    /// Compile ZLUDA (default command)
    Build(#[bpaf(external(build))] Build),
    #[bpaf(command)]
    /// Compile ZLUDA and build a package
    Zip(#[bpaf(external(build))] Build),
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
    target_name: String,
    target_kind: ProjectTarget,
    meta: ZludaMetadata,
}

impl Project {
    fn try_new(p: Package) -> Option<Project> {
        let name = p.name;
        serde_json::from_value::<Option<Metadata>>(p.metadata)
            .unwrap()
            .map(|m| {
                let (target_name, target_kind) = p
                    .targets
                    .into_iter()
                    .find_map(|target| {
                        if target.is_cdylib() {
                            Some((target.name, ProjectTarget::Cdylib))
                        } else if target.is_bin() {
                            Some((target.name, ProjectTarget::Bin))
                        } else {
                            None
                        }
                    })
                    .unwrap();
                Self {
                    name,
                    target_name,
                    target_kind,
                    meta: m.zluda,
                }
            })
    }

    #[cfg(unix)]
    fn prefix(&self) -> &'static str {
        match self.clib_name {
            None => "",
            Some(_) => "lib",
        }
    }

    #[cfg(unix)]
    fn suffix(&self) -> &'static str {
        match self.target_kind {
            ProjectTarget::Bin => "",
            ProjectTarget::Cdylib => ".so",
        }
    }

    #[cfg(not(unix))]
    fn suffix(&self) -> &'static str {
        match self.target_kind {
            ProjectTarget::Bin => ".exe",
            ProjectTarget::Cdylib => ".dll",
        }
    }
}

#[derive(Clone, Copy)]
enum ProjectTarget {
    Cdylib,
    Bin,
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
    #[cfg_attr(not(unix), allow(unused))]
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
        Options::Build(b) => {
            compile(b);
        }
        Options::Zip(b) => zip(b),
    }
}

fn compile(b: Build) -> (PathBuf, String, Vec<Project>) {
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
    os::make_symlinks(&target_directory, &*projects, &*profile);
    (target_directory, profile, projects)
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

fn zip(zip: Build) {
    fn file_options_from_time(from: &File) -> std::io::Result<SimpleFileOptions> {
        let metadata = from.metadata()?;
        let modified = metadata.modified()?;
        let modified = time::OffsetDateTime::from(modified);
        Ok(SimpleFileOptions::default().last_modified_time(
            zip::DateTime::try_from(modified).map_err(|err| std::io::Error::other(err))?,
        ))
    }

    let (target_dir, profile, projects) = compile(zip);
    let zip_file = File::create(format!("{}/{profile}/zluda.zip", target_dir.display())).unwrap();
    let mut zip = ZipWriter::new(zip_file);
    zip.add_directory("zluda", SimpleFileOptions::default())
        .unwrap();
    for project in projects.iter() {
        let name = &project.target_name;
        let ext = project.suffix();
        let mut file =
            std::fs::File::open(format!("{}/{profile}/{name}{ext}", target_dir.display())).unwrap();
        let file_options = file_options_from_time(&file).unwrap_or_default();
        zip.start_file(format!("zluda/{name}{ext}"), file_options)
            .unwrap();
        std::io::copy(&mut file, &mut zip).unwrap();
    }
    zip.finish().unwrap();
}

#[cfg(unix)]
mod os {
    use std::path::PathBuf;

    pub fn make_symlinks(
        target_directory: &std::path::PathBuf,
        _projects: &[super::Project],
        profile: &str,
    ) {
        use std::fs;
        use std::os::unix::fs as unix_fs;
        for project in projects.iter() {
            let clib_name = match project.clib_name {
                Some(ref l) => l,
                None => continue,
            };
            let libname = format!("lib{}.so", clib_name);
            for source in project.meta.linux_symlinks.iter() {
                let relative_link = PathBuf::from(source);
                let ancestors = relative_link.as_path().ancestors().count();
                let mut target = std::iter::repeat_with(|| "../").take(ancestors - 2).fold(
                    PathBuf::new(),
                    |mut buff, segment| {
                        buff.push(segment);
                        buff
                    },
                );
                let mut link = target_directory.clone();
                link.extend([profile, source]);
                let mut dir = link.clone();
                assert!(dir.pop());
                fs::create_dir_all(dir).unwrap();
                fs::remove_file(&link).ok();
                target.push(&*libname);
                unix_fs::symlink(&target, link).unwrap();
            }
        }
    }
}

#[cfg(not(unix))]
mod os {
    pub fn make_symlinks(
        _target_directory: &std::path::PathBuf,
        _projects: &[super::Project],
        _profile: &str,
    ) {
    }
}
