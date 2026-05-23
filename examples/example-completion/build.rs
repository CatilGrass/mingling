fn main() {
    build_scripts();
}

/// Generate completion scripts
fn build_scripts() {
    // `env!("CARGO_PKG_NAME")` equals the crate name, which matches the binary name.
    // If your binary name differs from the crate name, specify it explicitly.
    mingling::build::build_comp_scripts(
        // Your binary name:
        env!("CARGO_PKG_NAME"),
    )
    .unwrap();
}
