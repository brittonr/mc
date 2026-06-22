#![doc = include_str!("../README.md")]

use anyhow::Context;

pub fn write_generated_file(
    content: proc_macro2::TokenStream,
    out_file: &str,
) -> anyhow::Result<()> {
    let out_dir = std::env::var_os("OUT_DIR").context("failed to get OUT_DIR env var")?;
    let path = std::path::Path::new(&out_dir).join(out_file);
    let code = content.to_string();

    std::fs::write(&path, code)?;

    // Try to format the output for debugging purposes.
    // Doesn't matter if rustfmt is unavailable.
    if std::process::Command::new("rustfmt")
        .arg(path)
        .output()
        .is_err()
    {
        return Ok(());
    }

    Ok(())
}

/// Parses a [`proc_macro2::Ident`] from a `str`. Rust keywords are prepended
/// with underscores to make them valid identifiers.
pub fn ident<I: AsRef<str>>(s: I) -> proc_macro2::Ident {
    let s = s.as_ref().trim();

    // Parse the ident from a str. If the string is a Rust keyword, stick an
    // underscore in front.
    syn::parse_str::<proc_macro2::Ident>(s).unwrap_or_else(|_| {
        proc_macro2::Ident::new(format!("_{s}").as_str(), proc_macro2::Span::call_site())
    })
}

#[track_caller]
pub fn rerun_if_changed<const N: usize>(files: [&str; N]) {
    for file in files {
        assert!(
            std::path::Path::new(file).exists(),
            "File \"{file}\" does not exist. Did you forget to update the path?"
        );

        println!("cargo:rerun-if-changed={file}");
    }
}
