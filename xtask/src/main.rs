use argh::{EarlyExit, FromArgs, TopLevelCommand};
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
/// Package build artifacts into an archive (.zip or .tar.gz)
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
        Subcommand::Zip(_) => panic!(),
    })
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
    top_level: bool,
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

impl Project {
    fn new(json_pkg: cargo_metadata::Package) -> Self {
        let mut project = serde_json::from_value::<Option<ZludaMetadata>>(json_pkg.metadata)
            .unwrap()
            .map_or(Default::default(), |x| x.zluda);
        if project != Default::default() {
            project.top_level = true;
        }
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
        project
    }

    fn skip_build(&self, is_debug: bool) -> bool {
        if !self.top_level {
            return true;
        }
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
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let project_root = project_root()?;
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.cargo_path(&cargo).current_dir(&project_root).no_deps();
    let metadata = cmd.exec()?;
    let projects = metadata
        .packages
        .into_iter()
        .map(Project::new)
        .filter(|p| !p.skip_build(is_debug))
        .collect::<Vec<_>>();
    let mut command = Command::new(&cargo);
    command.current_dir(&project_root).arg("build");
    projects.iter().fold(&mut command, |command, proj| {
        command.args(["-p", &proj.name])
    });
    if !is_debug {
        command.arg("--release");
    }
    let build_result = command.status()?.code().unwrap();
    if build_result != 0 {
        return Ok(build_result);
    }
    os::create_dump_dir_and_symlinks(is_debug, metadata.target_directory, projects);
    Ok(0)
}

fn project_root() -> Result<PathBuf, DynError> {
    Ok(Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .ok_or::<DynError>("CARGO_MANIFEST_DIR".into())?
        .to_path_buf())
}

#[cfg(not(unix))]
mod os {
    use super::Project;
    use cargo_metadata::camino::Utf8PathBuf;

    // This is 100% intentional, we don't want symlinks on Windows since
    // we use completely different scheme for injections here
    pub(crate) fn create_dump_dir_and_symlinks(_: bool, _: Utf8PathBuf, _: Vec<Project>) {}
}

#[cfg(unix)]
mod os {
    use super::{Project, TargetKind};
    use cargo_metadata::camino::Utf8PathBuf;

    pub(crate) fn create_dump_dir_and_symlinks(
        is_debug: bool,
        mut target_directory: Utf8PathBuf,
        projects: Vec<Project>,
    ) {
        use std::fs;
        target_directory.push(if is_debug { "debug" } else { "release" });
        let mut dump_dir = target_directory.clone();
        dump_dir.push("dump");
        fs::create_dir_all(&dump_dir).unwrap();
        for project in projects {
            let dst = format!(
                "{}{}{}",
                project.kind.prefix(),
                project.target_name,
                project.kind.suffix()
            );
            let dump_dst = format!("../{}", dst);
            for src_file in project.linux_names {
                force_symlink(&dst, &target_directory, &src_file);
                if project.skip_dump_link {
                    continue;
                }
                force_symlink(&dump_dst, &dump_dir, &src_file);
            }
            for src_file in project.dump_names {
                force_symlink(&dump_dst, &dump_dir, &src_file);
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

    impl TargetKind {
        fn prefix(self) -> &'static str {
            match self {
                TargetKind::Binary => "",
                TargetKind::Cdylib => "lib",
            }
        }

        fn suffix(self) -> &'static str {
            match self {
                TargetKind::Binary => "",
                TargetKind::Cdylib => ".so",
            }
        }
    }
}
