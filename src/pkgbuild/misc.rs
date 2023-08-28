pub struct Misc {
    pub backup: Option<Vec<String>>,
    pub options: Option<Vec<String>>,
    pub install: Option<String>,
    pub changelog: Option<String>,
}

impl Default for Misc {
    fn default() -> Self {
        Self {
            backup: None,
            options: None,
            install: None,
            changelog: None,
        }
    }
}
