use cargo::core::manifest::Manifest;
use semver::Version;

use crate::string_vec;

pub struct Metadata {
    pub maintainers: Option<Vec<String>>,
    pub pkgname: String,
    pub pkgver: Version,
    pub pkgrel: String, // set to 1 by default
    pub epoch: Option<usize>,
    pub pkgdesc: Option<String>,
    //TODO research compatible architectures
    pub arch: Vec<String>,
    //TODO get git origin url?
    pub url: Option<String>,
    //TODO parse into list, see https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields
    pub license: Option<Vec<String>>,
}

impl Metadata {
    pub fn new(
        maintainers: Option<Vec<String>>,
        pkgname: String,
        pkgver: Version,
        pkgrel: String,
        epoch: Option<usize>,
        pkgdesc: Option<String>,
        arch: Vec<String>,
        url: Option<String>,
        license: Option<Vec<String>>,
    ) -> Self {
        Self {
            maintainers,
            pkgname,
            pkgver,
            pkgrel,
            epoch,
            pkgdesc,
            arch,
            url,
            license,
        }
    }

    pub fn named(pkgname: String) -> Self {
        Self {
            maintainers: None,
            pkgname,
            pkgver: Version::new(0, 1, 0),
            pkgrel: "1".into(),
            epoch: None,
            pkgdesc: None,
            arch: string_vec!["any"],
            url: None,
            license: None,
        }
    }
}

impl From<Manifest> for Metadata {
    fn from(manifest: Manifest) -> Self {
        let metadata = manifest.metadata();
        Self {
            maintainers: Some(metadata.authors.clone()).filter(|v| !v.is_empty()),
            pkgname: manifest.name().to_string(),
            pkgver: manifest.version().clone(),
            pkgrel: "1".into(),
            epoch: None,
            pkgdesc: metadata.description.clone(),
            arch: vec!["any".into()],
            url: metadata.homepage.clone(),
            license: metadata.license.clone().map(|l| vec![l]),
        }
    }
}
