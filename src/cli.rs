use std::path::PathBuf;

use cargo::ops::Packages;
use clap::{Args, Parser};

pub fn parse() -> Opts {
    let Cmd::Archpkg(opts) = Cmd::parse();
    opts
}

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum Cmd {
    #[command(about = "Create PKGBUILD from Rust project")]
    Archpkg(Opts),
}

#[derive(Clone, Debug, Parser)]
pub struct Opts {
    #[arg(
        short,
        long,
        default_value = "false",
        help = "Don’t verify the contents by building them"
    )]
    pub no_verify: bool,

    //TODO allow naming remote package
    #[arg(short, long, help = "Path to Cargo.toml")]
    pub manifest_path: Option<PathBuf>,

    #[arg(short, long, help = "Directory for PKGBUILD and its provided files")]
    pub output_dir: Option<PathBuf>,

    #[command(flatten)]
    pub packages: PackagesCli,
}

#[derive(Args, Clone, Debug)]
#[group(required = false, multiple = false)]
pub struct PackagesCli {
    #[arg(short, long, help = "Package all crates in workspace")]
    all: bool,

    #[arg(
        short,
        long,
        value_name = "PACKAGE",
        help = "Don't include specified package(s)"
    )]
    exclude: Option<Vec<String>>,

    #[arg(
        short = Some('p'),
        long = Some("package"),
        value_name = "PACKAGE",
        help = "Include specified package(s)"
    )]
    include: Option<Vec<String>>,
}

impl From<PackagesCli> for Packages {
    fn from(value: PackagesCli) -> Self {
        if value.all {
            Packages::All
        } else if let Some(exclude) = value.exclude {
            Packages::OptOut(exclude)
        } else if let Some(include) = value.include {
            Packages::Packages(include)
        } else {
            Packages::Default
        }
    }
}
