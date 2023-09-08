use std::str::FromStr;

use anyhow::Result;
use pkgbuild::Pkgbuild;
use serde::Deserialize;

pub const TOML_KEY_NAME: &str = "pkgbuild";

#[derive(Deserialize, Debug)]
pub struct PkgbuildConfig {
    #[serde(default = "default_check")]
    pub check: bool,

    #[serde(default = "default_depends")]
    pub depends: Vec<String>,

    #[serde(default = "default_source")]
    pub source: Source,
}

impl Default for PkgbuildConfig {
    fn default() -> Self {
        Self {
            check: default_check(),
            depends: default_depends(),
            source: default_source(),
        }
    }
}

impl FromStr for PkgbuildConfig {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match toml::from_str::<toml::Value>(s)?.get(TOML_KEY_NAME) {
            Some(v) => v.to_owned().try_into()?,
            None => Self::default(),
        })
    }
}

impl PkgbuildConfig {
    pub fn mod_pkgbuild(&self, pkgbuild: &mut Pkgbuild) {
        if !self.check {
            pkgbuild.functions.check = None;
        }

        pkgbuild.dependencies.depends = Some(self.depends.clone());
    }
}

#[derive(Deserialize, Debug)]
pub enum Source {
    #[serde(rename = "provided")]
    Provided,
    #[serde(untagged)]
    Remote(RemoteSource),
}

#[derive(Deserialize, Debug)]
pub struct RemoteSource {
    pub uri: String,
    pub checksum: String,
}

fn default_check() -> bool {
    true
}

fn default_depends() -> Vec<String> {
    Vec::new()
}

fn default_source() -> Source {
    Source::Provided
}
