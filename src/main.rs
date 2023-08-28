mod config;

use std::fs::{self, DirBuilder, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use cargo::core::resolver::features::CliFeatures;
use cargo::core::EitherManifest;
use cargo::core::{SourceId, Workspace};
use cargo::ops::{package, PackageOpts, Packages};
use cargo::util::command_prelude::root_manifest;
use cargo::util::config::Config as CargoConfig;
use cargo::util::toml::read_manifest;
use cargo::util::FileLock;
use clap::{Args, Parser};

use config::PkgbuildConfig;
use pkgbuild::Pkgbuild;

const TARGET_DIR: &str = "archpkg";
const PKGBUILD_FILENAME: &str = "PKGBUILD";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    //TODO allow naming remote package
    #[arg(short = Some('m'), long = Some("manifest"), value_name = "MANIFEST")]
    manifest_path: Option<PathBuf>,

    #[arg(short = Some('n'), long = Some("no-verify"), default_value = "false")]
    no_verify: bool,

    #[arg(short, long)]
    output: Option<PathBuf>,

    #[command(flatten)]
    packages: PackagesCli,
}

#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = false)]
struct PackagesCli {
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

//TODO clean this
fn main() -> Result<()> {
    //TODO handle parsing when called as cargo subcommand
    let cli_args = CliArgs::parse();

    let cargo_config = CargoConfig::default()?;
    let root_manifest = root_manifest(
        cli_args.manifest_path.as_ref().map(|p| p.as_path()),
        &cargo_config,
    )?;
    let pkgbuild_config = PkgbuildConfig::from_str(&fs::read_to_string(&root_manifest)?)?;

    let source_id = SourceId::for_path(&root_manifest)?;
    let (either_manifest, _) = read_manifest(&root_manifest, source_id, &cargo_config)?;

    match either_manifest {
        EitherManifest::Real(manifest) => {
            let ws = Workspace::new(&root_manifest, &cargo_config)?;

            let out_dir = cli_args
                .output
                .clone()
                .unwrap_or(ws.target_dir().into_path_unlocked().join(TARGET_DIR));

            DirBuilder::new().recursive(true).create(&out_dir)?;

            let flocks = tarball(&cargo_config, &ws, &cli_args)?;

            for flock in flocks {
                let source_alias = format!(
                    "{pkgname}-{pkgver}.tar.gz",
                    pkgname = manifest.name(),
                    pkgver = manifest.version(),
                );

                let source_path = out_dir.join(&source_alias);

                //TODO allow overwriting, control with cli option
                if Path::try_exists(&source_path)? {
                    return Err(anyhow!(
                        "file {source_alias} exists in {out_dir:?}, cannot overwrite it"
                    ));
                } else {
                    let mut source_file = File::create(&source_path)?;
                    io::copy(&mut flock.file(), &mut source_file)?;
                }

                //TODO choose to point to remote source, like github or crates.io
                let mut pkgbuild = Pkgbuild::new_provided_crate_file(
                    manifest.clone(),
                    source_alias,
                    &mut File::open(&source_path)?,
                )?;

                pkgbuild_config.as_ref().map(|config| {
                    if !config.check.unwrap_or(true) {
                        pkgbuild.functions.check = None;
                    }
                });

                let pkgbuild_path = out_dir.join(PKGBUILD_FILENAME);
                if Path::try_exists(&pkgbuild_path)? {
                    return Err(anyhow!(
                        "file {PKGBUILD_FILENAME} exists in {out_dir:?}, cannot overwrite it"
                    ));
                } else {
                    let mut pkgbuild_file = File::create(pkgbuild_path)?;
                    write!(pkgbuild_file, "{pkgbuild}")?;
                }
            }
            Ok(())
        }
        //TODO handle virtual manifests
        EitherManifest::Virtual(_) => Err(anyhow::Error::msg(anyhow!(
            "don't currently support virtual manifests, only local ones"
        ))),
    }
}

fn tarball(
    cargo_config: &CargoConfig,
    ws: &Workspace<'_>,
    cli_args: &CliArgs,
) -> Result<Vec<FileLock>> {
    if ws.root_maybe().is_embedded() {
        return Err(anyhow!(
            "{} is unsupported by `cargo package`, which is used in archpkg",
            ws.root_manifest().display()
        )
        .into());
    }

    package(
        &ws,
        //TODO control with cli options
        &PackageOpts {
            config: cargo_config,
            verify: !cli_args.no_verify,
            list: false,
            check_metadata: true,
            allow_dirty: true,
            to_package: cli_args.packages.clone().into(),
            targets: Vec::new(),
            jobs: None,
            keep_going: true,
            cli_features: CliFeatures::new_all(true),
        },
    )
    .context("failed to package workspace")?
    .ok_or(anyhow::Error::msg(anyhow!(
        "no file locks produced by cargo package"
    )))
}
