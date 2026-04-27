use std::process::ExitCode;

use xtask::Context;
use xtask::{Result, XtaskError};

const HELP: &str = "\
Usage: cargo xtask <command> [args]

Commands:
  setup                    Initialize git submodules and build TF-A.
  build [cargo-args]       Build aarch64-purecap-rt.
  check [cargo-args]       Check aarch64-purecap-rt.
  build-example <name>     Build examples/<name> in release mode. Other
                           arguments are ignored.
  fip <name>               Build <name> and produce a TF-A fip.bin at
                           examples/fvp/output/<name>/fip.bin. Other
                           arguments are ignored.
  fvp <name> [--clean]     Run <name> on the Morello FVP. Rebuilds the
                           firmware image if missing or --clean is passed.
                           Other arguments are ignored.
  help                     Show this message.
";

enum Subcommand {
    Build,
    Check,
    Help,
    FvpExample,
    BuildExample,
    FipExample,
    Setup,
}

impl TryFrom<&str> for Subcommand {
    type Error = XtaskError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "build" => Ok(Subcommand::Build),
            "check" => Ok(Subcommand::Check),
            "build-example" => Ok(Subcommand::BuildExample),
            "fip" => Ok(Subcommand::FipExample),
            "setup" => Ok(Subcommand::Setup),
            "fvp" => Ok(Subcommand::FvpExample),
            "help" | "--help" | "-h" => Ok(Subcommand::Help),
            _ => Err(XtaskError::InvalidSubcommand),
        }
    }
}

fn run(args: Vec<String>) -> Result<()> {
    let name = args.get(1).ok_or(XtaskError::MissingSubcommand)?.as_str();
    let subcommand = Subcommand::try_from(name)?;
    let tail = &args[2..];

    let context = Context::new();
    match subcommand {
        Subcommand::Help => print!("{HELP}"),
        Subcommand::BuildExample => context.build_example(tail)?,
        Subcommand::FvpExample => context.run_example(tail)?,
        Subcommand::FipExample => context.fip_example(tail)?,
        Subcommand::Build => context.build(tail)?,
        Subcommand::Check => context.check(tail)?,
        Subcommand::Setup => context.setup()?,
    }
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    match run(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("xtask: {err:?}");
            eprint!("\n{HELP}");
            ExitCode::FAILURE
        }
    }
}
