use std::io::Read;

use anyhow::Result;
use blake2::{Blake2b512, Digest};

#[derive(Clone)]
pub struct Source {
    //TODO use type representing url or provided source file, support eg git+https schema
    pub location: String,
    pub alias: Option<String>,
    //TODO choose checksum type
    pub integrity: String,
    pub no_extract: bool,
}

pub fn to_vecs(sources: &Vec<Source>) -> (Vec<String>, Vec<String>, Vec<String>) {
    let (mut source_uris, mut no_extract, mut b2sums) = (Vec::new(), Vec::new(), Vec::new());

    for source in sources {
        let name = format!(
            "{}{}",
            source
                .alias
                .as_ref()
                .map(|a| format!("{a}::"))
                .unwrap_or(String::new()),
            source.location
        );
        source_uris.push(name.clone());
        if source.no_extract {
            no_extract.push(name.clone());
        }
        b2sums.push(source.integrity.clone());
    }

    (source_uris, no_extract, b2sums)
}

impl Source {
    pub fn new<R: Read>(
        location: String,
        alias: Option<String>,
        no_extract: bool,
        reader: &mut R,
    ) -> Result<Self> {
        Ok(Source {
            location,
            alias,
            integrity: blake2_sum(reader)?,
            no_extract,
        })
    }

    pub fn remote_crate_file(url: String, alias: String, checksum: String) -> Self {
        Self {
            location: url,
            alias: Some(alias),
            integrity: checksum,
            no_extract: false,
        }
    }
}

fn blake2_sum<R: Read>(reader: &mut R) -> Result<String> {
    let mut hasher = Blake2b512::new();
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    hasher.update(buf);
    Ok(hasher
        .finalize()
        .to_vec()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect())
}
