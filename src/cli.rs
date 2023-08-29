use std::path::PathBuf;

use cargo::ops::Packages;
use clap::{Args, Parser, Subcommand};

pub fn parse() -> Opts {
    match TopCmd::parse().executable {
        Executable::Cargo(ArchpkgSubcmd::Archpkg(args)) => args,
        Executable::CargoArchpkg(args) => args,
    }
}

#[derive(Debug, Parser)]
#[command(multicall = true)]
struct TopCmd {
    #[command(subcommand)]
    executable: Executable,
}

#[derive(Debug, Subcommand)]
enum Executable {
    #[command(subcommand)]
    Cargo(ArchpkgSubcmd),
    CargoArchpkg(Opts),
}

#[derive(Clone, Debug, Subcommand)]
#[command(disable_help_subcommand = true)]
enum ArchpkgSubcmd {
    Archpkg(Opts),
}

#[derive(Clone, Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    //TODO allow naming remote package
    #[arg(short = Some('m'), long = Some("manifest"), value_name = "MANIFEST")]
    pub manifest_path: Option<PathBuf>,

    #[arg(short = Some('n'), long = Some("no-verify"), default_value = "false")]
    pub no_verify: bool,

    #[arg(short, long)]
    pub output: Option<PathBuf>,

    #[command(flatten)]
    pub packages: PackagesCli,
}

#[derive(Args, Clone, Debug)]
#[group(required = false, multiple = false)]
pub struct PackagesCli {
    #[arg(short, long)]
    all: bool,

    #[arg(short, long, value_name = "EXCLUDED")]
    exclude: Option<Vec<String>>,

    #[arg(short = Some('p'), long = Some("packages"), value_name = "PACKAGES")]
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
