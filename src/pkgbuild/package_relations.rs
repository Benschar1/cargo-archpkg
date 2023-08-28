pub struct PackageRelations {
    pub groups: Option<Vec<String>>,
    pub provides: Option<Vec<String>>,
    pub conflicts: Option<Vec<String>>,
    pub replaces: Option<Vec<String>>,
}

impl Default for PackageRelations {
    fn default() -> Self {
        Self {
            groups: Some(vec![]),
            provides: Some(vec![]),
            conflicts: Some(vec![]),
            replaces: Some(vec![]),
        }
    }
}
