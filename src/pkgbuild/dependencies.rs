use crate::string_vec;

pub struct Dependencies {
    pub depends: Option<Vec<String>>,
    //TODO get rust edition and version from toml manifest
    pub make_depends: Option<Vec<String>>,
    pub check_depends: Option<Vec<String>>,
    pub opt_depends: Option<Vec<String>>,
}

impl Default for Dependencies {
    fn default() -> Self {
        Self {
            depends: None,
            make_depends: Some(string_vec!["cargo"]),
            check_depends: None,
            opt_depends: None,
        }
    }
}
