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
}

impl PkgbuildConfig {
    pub fn from_str(s: &str) -> Result<Option<Self>> {
        let toml_value: toml::Value = toml::from_str(s)?;
        if let Some(toml::Value::Table(table)) = toml_value.get(TOML_KEY_NAME) {
            let pkgbuild_config: PkgbuildConfig = table.clone().try_into()?;
            return Ok(Some(pkgbuild_config));
        }

        Ok(None)
    }

    pub fn mod_pkgbuild(&self, pkgbuild: &mut Pkgbuild) {
        if !self.check {
            pkgbuild.functions.check = None;
        }

        pkgbuild.dependencies.depends = Some(self.depends.clone());
    }
}

fn default_check() -> bool {
    true
}

fn default_depends() -> Vec<String> {
    Vec::new()
}
