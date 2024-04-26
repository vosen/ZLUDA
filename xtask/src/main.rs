use argh::{EarlyExit, FromArgs, TopLevelCommand};
use cargo_metadata::camino::Utf8PathBuf;
use serde::Deserialize;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

type DynError = Box<dyn std::error::Error>;

struct Arguments {
    command: Subcommand,
}

impl TopLevelCommand for Arguments {}
impl FromArgs for Arguments {
    fn from_args(command_name: &[&str], args: &[&str]) -> Result<Self, argh::EarlyExit> {
        #[derive(FromArgs)]
        /// Run various ZLUDA tasks
        struct TryArguments {
            #[argh(subcommand)]
            command: Option<Subcommand>,
        }
        Ok(
            match <TryArguments as FromArgs>::from_args(command_name, args) {
                Ok(TryArguments { command }) => Arguments {
                    command: command.unwrap_or_default(),
                },
                Err(err @ EarlyExit { status: Ok(()), .. }) => return Err(err),
                Err(EarlyExit {
                    status: Err(()), ..
                }) => Arguments {
                    command: Subcommand::Build(BuildCommand::from_args(command_name, args)?),
                },
            },
        )
    }
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Build(BuildCommand),
    Zip(ZipCommand),
}

impl Default for Subcommand {
    fn default() -> Self {
        Subcommand::Build(BuildCommand { release: false })
    }
}

#[derive(FromArgs)]
/// Compile ZLUDA for the current platform (default command)
#[argh(subcommand, name = "build")]
struct BuildCommand {
    /// build artifacts in release mode, with optimizations
    #[argh(switch, short = 'r')]
    release: bool,
}

#[derive(FromArgs)]
/// Compile ZLUDA and package binaries into an archive (.zip or .tar.gz)
#[argh(subcommand, name = "zip")]
struct ZipCommand {
    /// use artifacts from release mode
    #[argh(switch, short = 'r')]
    #[allow(dead_code)]
    release: bool,
}

fn main() -> Result<(), DynError> {
    let args: Arguments = argh::from_env();
    std::process::exit(match args.command {
        Subcommand::Build(BuildCommand { release }) => build(!release)?,
        Subcommand::Zip(ZipCommand { release }) => build_and_zip(!release),
    })
}

fn build_and_zip(is_debug: bool) -> i32 {
    let workspace = build_impl(is_debug).unwrap();
    os::zip(workspace)
}

#[derive(Deserialize)]
struct ZludaMetadata {
    zluda: Project,
}

#[derive(Deserialize, Default, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
struct Project {
    #[serde(skip_deserializing)]
    name: String,
    #[serde(skip_deserializing)]
    target_name: String,
    #[serde(skip_deserializing)]
    kind: TargetKind,
    #[serde(default)]
    windows_only: bool,
    #[serde(default)]
    linux_only: bool,
    #[serde(default)]
    debug_only: bool,
    #[serde(default)]
    broken: bool,
    #[serde(default)]
    skip_dump_link: bool,
    #[serde(default)]
    skip_zip: bool,
    #[serde(default)]
    linux_names: Vec<String>,
    #[serde(default)]
    dump_names: Vec<String>,
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
enum TargetKind {
    #[default]
    Binary,
    Cdylib,
}

struct Workspace {
    pub cargo: String,
    pub project_root: PathBuf,
    pub projects: Vec<Project>,
    pub target_directory: Utf8PathBuf,
}

impl Workspace {
    fn open(is_debug: bool) -> Result<Self, DynError> {
        let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        let project_root = Self::project_root()?;
        let mut cmd = cargo_metadata::MetadataCommand::new();
        cmd.cargo_path(&cargo).current_dir(&project_root).no_deps();
        let cargo_metadata = cmd.exec()?;
        let projects = cargo_metadata
            .packages
            .into_iter()
            .filter_map(Project::new)
            .filter(|p| !p.skip_build(is_debug))
            .collect::<Vec<_>>();
        let mut target_directory = cargo_metadata.target_directory;
        target_directory.push(if is_debug { "debug" } else { "release" });
        Ok(Workspace {
            cargo,
            project_root,
            projects,
            target_directory,
        })
    }

    fn project_root() -> Result<PathBuf, DynError> {
        Ok(Path::new(&env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .ok_or::<DynError>("CARGO_MANIFEST_DIR".into())?
            .to_path_buf())
    }

    fn cargo_command(&self) -> Command {
        let mut command = Command::new(&self.cargo);
        command.current_dir(&self.project_root);
        command
    }
}

impl Project {
    fn new(json_pkg: cargo_metadata::Package) -> Option<Self> {
        let project_metadata =
            serde_json::from_value::<Option<ZludaMetadata>>(json_pkg.metadata).unwrap()?;
        let mut project = project_metadata.zluda;
        project.name = json_pkg.name;
        if let Some((target_name, kind)) = json_pkg.targets.into_iter().find_map(|t| {
            match t.kind.first().map(std::ops::Deref::deref) {
                Some("cdylib") => Some((t.name, TargetKind::Cdylib)),
                Some("bin") => Some((t.name, TargetKind::Binary)),
                _ => None,
            }
        }) {
            project.target_name = target_name;
            project.kind = kind;
        }
        Some(project)
    }

    fn skip_build(&self, is_debug: bool) -> bool {
        if self.broken {
            return true;
        }
        if cfg!(windows) && self.linux_only {
            return true;
        }
        if !cfg!(windows) && self.windows_only {
            return true;
        }
        if !is_debug && self.debug_only {
            return true;
        }
        false
    }
}

fn build(is_debug: bool) -> Result<i32, DynError> {
    build_impl(is_debug)?;
    Ok(0)
}

fn build_impl(is_debug: bool) -> Result<Workspace, DynError> {
    let workspace = Workspace::open(is_debug)?;
    let mut command = workspace.cargo_command();
    command.arg("build");
    workspace
        .projects
        .iter()
        .fold(&mut command, |command, proj| {
            command.args(["-p", &proj.name])
        });
    if !is_debug {
        command.arg("--release");
    }
    let build_result = command.status()?.code().unwrap();
    if build_result != 0 {
        return Err(format!("{command:?} failed with exit code {build_result}").into());
    }
    os::create_dump_dir_and_symlinks(&workspace);
    Ok(workspace)
}

impl TargetKind {
    #[cfg(unix)]
    fn prefix(self) -> &'static str {
        match self {
            TargetKind::Binary => "",
            TargetKind::Cdylib => "lib",
        }
    }

    #[cfg(unix)]
    fn suffix(self) -> &'static str {
        match self {
            TargetKind::Binary => "",
            TargetKind::Cdylib => ".so",
        }
    }

    #[cfg(windows)]
    fn suffix(self) -> &'static str {
        match self {
            TargetKind::Binary => ".exe",
            TargetKind::Cdylib => ".dll",
        }
    }
}

#[cfg(unix)]
mod os {
    use crate::Workspace;
    use cargo_metadata::camino::Utf8PathBuf;
    use flate2::{write::GzEncoder, Compression};
    use std::fs::File;

    pub(crate) fn create_dump_dir_and_symlinks(workspace: &Workspace) {
        use std::fs;
        let mut dump_dir = workspace.target_directory.clone();
        dump_dir.push("dump");
        fs::create_dir_all(&dump_dir).unwrap();
        for project in workspace.projects.iter() {
            let dst = format!(
                "{}{}{}",
                project.kind.prefix(),
                project.target_name,
                project.kind.suffix()
            );
            let dump_dst = format!("../{}", dst);
            for src_file in project.linux_names.iter() {
                force_symlink(&dst, &workspace.target_directory, src_file);
                if project.skip_dump_link {
                    continue;
                }
                force_symlink(&dump_dst, &dump_dir, src_file);
            }
            for src_file in project.dump_names.iter() {
                force_symlink(&dump_dst, &dump_dir, src_file);
            }
        }
    }

    fn force_symlink(dst: &str, src_dir: &Utf8PathBuf, src_file: &str) {
        use std::io::ErrorKind;
        use std::os::unix::fs as unix_fs;
        let mut src = src_dir.clone();
        src.push(src_file);
        match unix_fs::symlink(dst, &src) {
            Ok(()) => {}
            Err(err) if err.kind() == ErrorKind::AlreadyExists => {
                let current_dst = std::fs::read_link(&src);
                match current_dst {
                    Ok(current_dst) if current_dst.to_str() == Some(dst) => {
                        return;
                    }
                    _ => {
                        std::fs::remove_file(&src).unwrap();
                        unix_fs::symlink(dst, &src).unwrap();
                    }
                }
            }
            Err(err) => panic!("{:?}", err),
        }
    }

    pub fn zip(workspace: Workspace) -> i32 {
        let mut target_file = workspace.target_directory.clone();
        target_file.push("zluda.tar.gz");
        let gz_file = File::create(target_file).unwrap();
        let gz = GzEncoder::new(gz_file, Compression::default());
        let mut tar = tar::Builder::new(gz);
        for project in workspace.projects {
            if project.skip_zip {
                continue;
            }
            let mut src_file = File::open(format!(
                "{}/{}{}{}",
                &workspace.target_directory,
                project.kind.prefix(),
                project.target_name,
                project.kind.suffix()
            ))
            .unwrap();
            let file_in_archive_path = format!(
                "zluda/{}{}{}",
                project.kind.prefix(),
                project.target_name,
                project.kind.suffix()
            );
            tar.append_file(
                format!(
                    "zluda/{}{}{}",
                    project.kind.prefix(),
                    project.target_name,
                    project.kind.suffix()
                ),
                &mut src_file,
            )
            .unwrap();
            for linux_name in project.linux_names.iter() {
                let mut header = tar::Header::new_gnu();
                header.set_entry_type(tar::EntryType::Symlink);
                tar.append_link(
                    &mut header,
                    format!("zluda/{}", linux_name),
                    &file_in_archive_path,
                )
                .unwrap();
                if project.skip_dump_link {
                    continue;
                }
                let mut header = tar::Header::new_gnu();
                header.set_entry_type(tar::EntryType::Symlink);
                tar.append_link(
                    &mut header,
                    format!("zluda/dump/{}", linux_name),
                    &file_in_archive_path,
                )
                .unwrap();
            }
            for dump_name in project.dump_names.iter() {
                let mut header = tar::Header::new_gnu();
                header.set_entry_type(tar::EntryType::Symlink);
                tar.append_link(
                    &mut header,
                    format!("zluda/dump/{}", dump_name),
                    &file_in_archive_path,
                )
                .unwrap();
            }
        }
        tar.finish().unwrap();
        0
    }
}

#[cfg(windows)]
mod os {
    use crate::Workspace;

    // This is 100% intentional, we don't want symlinks on Windows since
    // we use a completely different scheme for injections there
    pub(crate) fn create_dump_dir_and_symlinks(_: &Workspace) {}

    pub fn zip(workspace: Workspace) -> i32 {
        fn get_zip_entry_options(
            f: &File,
            time_offset: time::UtcOffset,
        ) -> Option<zip::write::FileOptions> {
            let time = f.metadata().unwrap().modified().unwrap();
            let time = time::OffsetDateTime::from(time).to_offset(time_offset);
            Some(
                zip::write::FileOptions::default()
                    .last_modified_time(zip::DateTime::try_from(time).unwrap()),
            )
        }
        let mut target_file = workspace.target_directory.clone();
        target_file.push("zluda.zip");
        let zip_archive = File::create(target_file).unwrap();
        let mut zip_writer = zip::write::ZipWriter::new(zip_archive);
        let time_offset = time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
        for p in workspace.projects {
            if p.skip_zip {
                continue;
            }
            let mut src_file = File::open(format!(
                "{}/{}{}",
                &workspace.target_directory,
                p.target_name,
                p.kind.suffix()
            ))
            .unwrap();
            zip_writer
                .start_file(
                    format!("zluda/{}{}", p.target_name, p.kind.suffix()),
                    get_zip_entry_options(&src_file, time_offset)
                        .unwrap_or(zip::write::FileOptions::default()),
                )
                .unwrap();
            std::io::copy(&mut src_file, &mut zip_writer).unwrap();
        }
        zip_writer.finish().unwrap();
        Ok(0)
    }
}
