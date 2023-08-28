use anyhow::Result;
use serde::Deserialize;

pub const TOML_KEY_NAME: &str = "pkgbuild";

#[derive(Deserialize, Debug)]
pub struct PkgbuildConfig {
    pub check: Option<bool>,
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
}
