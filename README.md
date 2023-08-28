# Cargo-Archpkg

Create PKGBUILD from Rust project

## Usage

To install a cargo package on your system, run `cargo-archpkg` from within the project.
Then `cd` into `target/archpkg` and run `makepkg -si` to install the package.

You can point to a different `Cargo.toml` file with the `--manifest` flag and a different output directory with the `--output` flag.
Unless overridden with `--output`, the pkgbuild is written to the `target/archpkg` of the project it is building.
