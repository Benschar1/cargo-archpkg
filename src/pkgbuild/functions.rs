use crate::string_vec;

pub const CD_INTO_SRCDIR: &str = r#"cd "$srcdir/$pkgname-$pkgver""#;

//TODO add pkgver
pub struct Functions {
    pub prepare: Option<Vec<String>>,
    pub build: Option<Vec<String>>,
    pub check: Option<Vec<String>>,
    pub package: Vec<String>,
}

pub fn show_function(name: &str, lines: Vec<String>, indent: &str) -> String {
    format!(
        "{name}() {{\n{}\n}}",
        lines
            .iter()
            .map(|line| format!("{indent}{line}"))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

impl Default for Functions {
    fn default() -> Self {
        Self {
            prepare: Some(string_vec!(
                CD_INTO_SRCDIR,
                r#"export RUSTUP_TOOLCHAIN=stable"#,
                r#"cargo fetch --locked --target "$CARCH-unknown-linux-gnu""#
            )),
            build: Some(string_vec!(
                CD_INTO_SRCDIR,
                r#"export RUSTUP_TOOLCHAIN=stable"#,
                r#"export CARGO_TARGET_DIR=target"#,
                r#"cargo build --frozen --release --all-features"#
            )),
            check: Some(string_vec!(
                CD_INTO_SRCDIR,
                r#"export RUSTUP_TOOLCHAIN=stable"#,
                r#"cargo test --frozen --all-features"#
            )),
            //TODO search for multiple binaries
            package: string_vec!(
                CD_INTO_SRCDIR,
                r#"install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname""#
            ),
        }
    }
}
