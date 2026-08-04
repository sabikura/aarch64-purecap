mod error;

use owo_colors::OwoColorize;
use std::path::{Path, PathBuf};
use std::process::Command;

pub use error::XtaskError;
pub type Result<T> = std::result::Result<T, XtaskError>;

/// Wrapper over [`Command::status`] that also prints the executed command in a pretty format.
fn run(command: &mut Command) -> ::std::result::Result<std::process::ExitStatus, std::io::Error> {
    eprintln!(
        "{:>10}: Inside directory {}",
        "Xtask".blue().bold(),
        command
            .get_current_dir()
            .unwrap_or(&std::env::current_dir()?)
            .to_string_lossy()
    );

    eprintln!(
        "{:>10}: Running {}",
        "Xtask".blue().bold(),
        format!(
            "{} {}",
            command.get_program().to_string_lossy(),
            command
                .get_args()
                .map(|oss| oss.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" ")
        )
        .underline()
        .bold()
    );

    command.status()
}

/// Resolved absolute paths to workspace subdirectories. Computed once from
/// CARGO_MANIFEST_DIR so commands work regardless of the caller's cwd.
pub struct Context {
    aarch64_purecap_rt: PathBuf,
    aarch64_purecap_cpu: PathBuf,
    toolchain: PathBuf,
    examples: PathBuf,
    fvp: PathBuf,
}

impl Context {
    pub fn new() -> Self {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let parent = manifest_dir.parent().unwrap();
        let examples = parent.join("examples");

        Self {
            aarch64_purecap_rt: parent.join("aarch64-purecap-rt"),
            aarch64_purecap_cpu: parent.join("aarch64-purecap-cpu"),
            toolchain: parent.join("toolchain"),
            fvp: examples.join("fvp"),
            examples,
        }
    }

    pub fn build(&self, args: &[String]) -> Result<()> {
        self.cargo_command("build", &self.aarch64_purecap_rt, args)?;
        self.cargo_command("build", &self.aarch64_purecap_cpu, args)
    }

    pub fn check(&self, args: &[String]) -> Result<()> {
        self.cargo_command("check", &self.aarch64_purecap_rt, args)?;
        self.cargo_command("build", &self.aarch64_purecap_cpu, args)
    }

    /// Invokes `toolchain/cargo.sh`, the wrapper that points at the CHERI
    /// rustc/cargo fork.
    fn cargo_command(&self, command: &str, path: &Path, args: &[String]) -> Result<()> {
        let cargo = self.toolchain.join("cargo.sh");
        println!(
            "{} {} {}({})",
            cargo.to_string_lossy(),
            command,
            args.join(" "),
            path.to_string_lossy(),
        );

        let status = run(Command::new(cargo)
            .arg(command)
            .args(args)
            .current_dir(path))?;

        if !status.success() {
            eprintln!("command failed with status {}", status);
            return Err(XtaskError::CommandFailed);
        }

        Ok(())
    }

    pub fn build_example(&self, args: &[String]) -> Result<()> {
        let path = self.example_path(args)?;
        self.cargo_command("build", &path, &["--release".into()])?;

        Ok(())
    }

    /// Boots the example on the FVP. The fip.bin is only rebuilt when missing
    /// or when `--clean` is given.
    pub fn run_example(&self, args: &[String]) -> Result<()> {
        let name = self.example_name(args)?;
        let path = self.examples.join(name);
        let fip_bin = self.fip_path(name);

        let clean = args.iter().skip(1).any(|a| a == "--clean");
        if clean || !fip_bin.exists() {
            let pkg_name = read_package_name(&path.join("Cargo.toml"))?;
            self.fip_example_inner(&path, name, &pkg_name)?;
        }

        let run_cmd = self.fvp.join("run.sh");
        let status = run(Command::new(run_cmd).arg(&fip_bin))?;
        if !status.success() {
            eprintln!("run.sh failed with status {}", status);
            return Err(XtaskError::CommandFailed);
        }

        Ok(())
    }

    pub fn fip_example(&self, args: &[String]) -> Result<()> {
        let name = self.example_name(args)?;
        let path = self.examples.join(name);
        let pkg_name = read_package_name(&path.join("Cargo.toml"))?;
        let fip = self.fip_example_inner(&path, name, &pkg_name)?;
        println!("fip: {}", fip.display());
        Ok(())
    }

    fn fip_example_inner(&self, example: &Path, name: &str, pkg_name: &str) -> Result<PathBuf> {
        self.cargo_command("build", example, &["--release".into()])?;

        const TARGET_TRIPLE: &str = "aarch64-unknown-none-purecap";
        let target_dir = example.join("target").join(TARGET_TRIPLE).join("release");

        let elf = target_dir.join(pkg_name);
        let bin = target_dir.join(format!("{pkg_name}.bin"));

        let objcopy = self.toolchain.join("objcopy.sh");
        let status = run(Command::new(&objcopy)
            .args(["-O", "binary"])
            .arg(&elf)
            .arg(&bin))?;
        if !status.success() {
            eprintln!("objcopy failed with status {}", status);
            return Err(XtaskError::CommandFailed);
        }

        let out = self.example_output_dir(name);
        let fip_sh = self.fvp.join("fip.sh");
        let status = run(Command::new(fip_sh).arg(&bin).arg(&out))?;
        if !status.success() {
            eprintln!("fip.sh failed with status {}", status);
            return Err(XtaskError::CommandFailed);
        }

        Ok(out.join("fip.bin"))
    }

    pub fn setup(&self) -> Result<()> {
        let status =
            run(Command::new("git").args(["submodule", "update", "--init", "--recursive"]))?;
        if !status.success() {
            eprintln!("git submodule failed with status {}", status);
            return Err(XtaskError::CommandFailed);
        }

        let maketfa = self.fvp.join("maketfa.sh");
        let status = run(&mut Command::new(maketfa))?;
        if !status.success() {
            eprintln!("maketfa.sh failed with status {}", status);
            return Err(XtaskError::CommandFailed);
        }

        Ok(())
    }

    fn example_path(&self, args: &[String]) -> Result<PathBuf> {
        Ok(self.examples.join(self.example_name(args)?))
    }

    fn example_name<'a>(&self, args: &'a [String]) -> Result<&'a str> {
        args.first()
            .map(String::as_str)
            .ok_or(XtaskError::MissingExampleName)
    }

    fn example_output_dir(&self, name: &str) -> PathBuf {
        self.fvp.join("output").join(name)
    }

    fn fip_path(&self, name: &str) -> PathBuf {
        self.example_output_dir(name).join("fip.bin")
    }
}

fn read_package_name(cargo_toml: &Path) -> Result<String> {
    let content = std::fs::read_to_string(cargo_toml)?;
    let mut in_package = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_package = trimmed == "[package]";
            continue;
        }
        if !in_package {
            continue;
        }
        let Some(rest) = trimmed.strip_prefix("name") else {
            continue;
        };
        let rest = rest.trim_start();
        let Some(rest) = rest.strip_prefix('=') else {
            continue;
        };
        let value = rest.trim().trim_matches('"');
        if !value.is_empty() {
            return Ok(value.to_string());
        }
    }
    Err(XtaskError::MissingPackageName)
}
