mod dependencies;
mod functions;
mod metadata;
mod misc;
mod package_relations;
mod source;
pub mod util;

use std::{fmt, io::Read};

use anyhow::Result;
use cargo::core::Manifest;

pub use dependencies::Dependencies;
pub use functions::{Functions, CD_INTO_SRCDIR};
pub use metadata::Metadata;
pub use misc::Misc;
pub use package_relations::PackageRelations;
pub use source::Source;
use util::{bash_arr, bash_val, bash_val_opt};

pub struct Pkgbuild {
    pub metadata: Metadata,
    pub dependencies: Dependencies,
    pub package_relations: PackageRelations,
    pub misc: Misc,
    pub sources: Vec<Source>,
    pub valid_pgp_keys: Option<Vec<String>>,
    pub functions: Functions,
}

impl Pkgbuild {
    pub fn write_authors(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(maintainers) = &self.metadata.maintainers {
            for maintainer in maintainers {
                writeln!(f, "# Maintainer: {}", maintainer)?;
            }
        }
        Ok(())
    }

    pub fn new_provided_crate_file<R: Read>(
        manifest: Manifest,
        name: String,
        crate_file: &mut R,
    ) -> Result<Self> {
        Ok(Self {
            metadata: Metadata::from(manifest),
            dependencies: Dependencies::default(),
            package_relations: PackageRelations::default(),
            misc: Misc::default(),
            sources: vec![Source::new(name, None, false, crate_file)?],
            valid_pgp_keys: None,
            functions: Functions::default(),
        })
    }
}

impl fmt::Display for Pkgbuild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let indent = "    ";
        self.write_authors(f)?;
        bash_val(f, "pkgname", &self.metadata.pkgname)?;
        bash_val(f, "pkgver", &self.metadata.pkgver.to_string())?;
        bash_val(f, "pkgrel", &self.metadata.pkgrel)?;
        bash_val_opt(f, "epoch", self.metadata.epoch.map(|u| u.to_string()))?;
        bash_val_opt(
            f,
            "pkgdesc",
            self.metadata.pkgdesc.as_ref().map(|s| format!("{s}")),
        )?;
        bash_arr(f, "arch", Some(self.metadata.arch.clone()))?;
        bash_val_opt(f, "url", self.metadata.url.clone())?;
        bash_arr(f, "license", self.metadata.license.clone())?;
        bash_arr(f, "groups", self.package_relations.groups.clone())?;
        bash_arr(f, "depends", self.dependencies.depends.clone())?;
        bash_arr(f, "makedepends", self.dependencies.make_depends.clone())?;
        bash_arr(f, "checkdepends", self.dependencies.check_depends.clone())?;
        bash_arr(f, "optdepends", self.dependencies.opt_depends.clone())?;
        bash_arr(f, "provides", self.package_relations.provides.clone())?;
        bash_arr(f, "conflicts", self.package_relations.conflicts.clone())?;
        bash_arr(f, "replaces", self.package_relations.replaces.clone())?;
        bash_arr(f, "backup", self.misc.backup.clone())?;
        bash_arr(f, "options", self.misc.options.clone())?;
        bash_val_opt(f, "install", self.misc.install.clone())?;
        bash_val_opt(f, "changelog", self.misc.changelog.clone())?;
        let (source, no_extract, b2sums) = source::to_vecs(&self.sources);
        bash_arr(f, "source", Some(source))?;
        bash_arr(f, "noextract", Some(no_extract))?;
        bash_arr(f, "b2sums", Some(b2sums))?;

        if let Some(prepare) = &self.functions.prepare {
            writeln!(
                f,
                "\n{}",
                functions::show_function("prepare", prepare.clone(), indent)
            )?;
        }

        if let Some(build) = &self.functions.build {
            writeln!(
                f,
                "\n{}",
                functions::show_function("build", build.clone(), indent)
            )?;
        }

        if let Some(check) = &self.functions.check {
            writeln!(
                f,
                "\n{}",
                functions::show_function("check", check.clone(), indent)
            )?;
        }

        writeln!(
            f,
            "\n{}",
            functions::show_function("package", self.functions.package.clone(), indent)
        )?;

        Ok(())
    }
}
