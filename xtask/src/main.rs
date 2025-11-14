use bpaf::{Args, Bpaf, Parser};
use cargo_metadata::{MetadataCommand, Package};
use serde::Deserialize;
use std::{env, ffi::OsString, path::PathBuf, process::Command};

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
        match self.target_kind {
            ProjectTarget::Bin => "",
            ProjectTarget::Cdylib => "lib",
        }
    }

    #[cfg(not(unix))]
    fn prefix(&self) -> &'static str {
        ""
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

    // Returns tuple:
    // * symlink file path (relative to the root of build dir)
    // * symlink absolute file path
    // * target actual file (relative to symlink file)
    #[cfg_attr(not(unix), allow(unused))]
    fn linux_symlinks<'a>(
        &'a self,
        target_dir: &'a PathBuf,
        profile: &'a str,
        libname: &'a str,
    ) -> impl Iterator<Item = (&'a str, PathBuf, PathBuf)> + 'a {
        Self::relative_paths(
            self,
            target_dir,
            profile,
            libname,
            self.meta.linux_symlinks.as_slice(),
        )
    }

    #[cfg_attr(unix, allow(unused))]
    fn windows_paths<'a>(
        &'a self,
        target_dir: &'a PathBuf,
        profile: &'a str,
        libname: &'a str,
    ) -> impl Iterator<Item = (&'a str, PathBuf, PathBuf)> + 'a {
        Self::relative_paths(
            self,
            target_dir,
            profile,
            libname,
            self.meta.windows_paths.as_slice(),
        )
    }

    fn relative_paths<'a>(
        &'a self,
        target_dir: &'a PathBuf,
        profile: &'a str,
        libname: &'a str,
        source: &'a [String],
    ) -> impl Iterator<Item = (&'a str, PathBuf, PathBuf)> + 'a {
        source.iter().map(move |source| {
            let mut link = target_dir.clone();
            link.extend([profile, source]);
            let relative_link = PathBuf::from(source);
            let ancestors = relative_link.as_path().ancestors().count();
            let mut target = std::iter::repeat_with(|| "../").take(ancestors - 2).fold(
                PathBuf::new(),
                |mut buff, segment| {
                    buff.push(segment);
                    buff
                },
            );
            target.push(libname);
            (&**source, link, target)
        })
    }

    fn file_name(&self) -> String {
        let target_name = &self.target_name;
        let prefix = self.prefix();
        let suffix = self.suffix();
        format!("{prefix}{target_name}{suffix}")
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
    linux_only: bool,
    #[serde(default)]
    windows_only: bool,
    #[serde(default)]
    debug_only: bool,
    #[cfg_attr(not(unix), allow(unused))]
    #[serde(default)]
    linux_symlinks: Vec<String>,
    #[serde(default)]
    windows_paths: Vec<String>,
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
        .filter(|project| {
            if project.meta.linux_only && cfg!(windows) {
                return false;
            }
            if project.meta.windows_only && cfg!(not(windows)) {
                return false;
            }
            if project.meta.debug_only && profile != "debug" {
                return false;
            }
            true
        })
        .collect::<Vec<_>>();
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(&cargo);
    command.arg("build");
    command.arg("--locked");
    for project in projects.iter() {
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
    let (target_dir, profile, projects) = compile(zip);
    os::zip(target_dir, profile, projects)
}

#[cfg(unix)]
mod os {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::{
        fs::{self, File},
        io::Write,
        path::PathBuf,
    };

    pub fn make_symlinks(
        target_directory: &std::path::PathBuf,
        projects: &[super::Project],
        profile: &str,
    ) {
        use std::os::unix::fs as unix_fs;
        for project in projects.iter() {
            let libname = project.file_name();
            for (_, full_path, target) in project.symlinks(target_directory, profile, &libname) {
                let mut dir = full_path.clone();
                assert!(dir.pop());
                fs::create_dir_all(dir).unwrap();
                fs::remove_file(&full_path).ok();
                unix_fs::symlink(&target, full_path).unwrap();
            }
        }
    }

    pub(crate) fn zip(target_dir: PathBuf, profile: String, projects: Vec<crate::Project>) {
        let mut tar_gz =
            File::create(format!("{}/{profile}/zluda.tar.gz", target_dir.display())).unwrap();
        let enc = GzEncoder::new(&mut tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);
        // Leads to broken tar archives on WSL
        tar.sparse(false);
        tar.follow_symlinks(false);
        for project in projects.iter() {
            let file_name = project.file_name();
            let mut file =
                File::open(format!("{}/{profile}/{file_name}", target_dir.display())).unwrap();
            tar.append_file(format!("zluda/{file_name}"), &mut file)
                .unwrap();
            for (source, full_path, _) in project.symlinks(&target_dir, &profile, &file_name) {
                tar.append_path_with_name(&full_path, format!("zluda/{source}"))
                    .unwrap();
            }
        }
        tar.into_inner().unwrap().finish().unwrap().flush().unwrap();
    }
}

#[cfg(not(unix))]
mod os {
    use std::{fs::File, io, path::PathBuf};
    use zip::{write::SimpleFileOptions, ZipWriter};

    pub fn make_symlinks(
        target_directory: &std::path::PathBuf,
        projects: &[super::Project],
        profile: &str,
    ) {
        for project in projects.iter() {
            let libname = project.file_name();
            for (_, full_path, target) in project.windows_paths(target_directory, profile, &libname)
            {
                let mut dir = full_path.clone();
                assert!(dir.pop());
                std::fs::create_dir_all(&dir).unwrap();
                dir.push(&target);
                std::fs::copy(dir, full_path).unwrap();
            }
        }
    }

    pub(crate) fn zip(target_dir: PathBuf, profile: String, projects: Vec<crate::Project>) {
        let zip_file =
            File::create(format!("{}/{profile}/zluda.zip", target_dir.display())).unwrap();
        let mut zip = ZipWriter::new(zip_file);
        zip.add_directory("zluda", SimpleFileOptions::default())
            .unwrap();
        for project in projects.iter() {
            let file_name = project.file_name();
            let mut file =
                File::open(format!("{}/{profile}/{file_name}", target_dir.display())).unwrap();
            let file_options = file_options_from_time(&file).unwrap_or_default();
            zip.start_file(format!("zluda/{file_name}"), file_options)
                .unwrap();
            io::copy(&mut file, &mut zip).unwrap();
        }
        zip.finish().unwrap();
    }

    fn file_options_from_time(from: &File) -> io::Result<SimpleFileOptions> {
        let metadata = from.metadata()?;
        let modified = metadata.modified()?;
        let modified = time::OffsetDateTime::from(modified);
        Ok(SimpleFileOptions::default().last_modified_time(
            zip::DateTime::try_from(modified).map_err(|err| io::Error::other(err))?,
        ))
    }
}
