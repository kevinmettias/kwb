//! Reading a crate's own source, for the guard tests built on it.
//!
//! Two crates assert a structural property of themselves — `kwb-store` that it exposes exactly
//! one mutating method, `kwb-retrieval` that it exposes none — and both assert it the same way:
//! read every `.rs` file under the crate's `src`, then collect the names of the `pub fn`
//! declarations whose parameter list takes `&mut self`.
//!
//! The *property* differs between them and stays with each crate. The *reading* does not, so it
//! lives here once. Written twice it was two copies a fix had to reach, which is what
//! `check-interfile-duplication` names as the hazard rather than the line count.
//!
//! A crate reaches this through a `dev-dependency`, so nothing here is linked into a shipped
//! binary and no production crate gains an edge to it.

#![forbid(unsafe_code)]

use std::path::Path;

/// Every `.rs` file under `<manifest_directory>/src`, read as text.
///
/// The caller passes its own `CARGO_MANIFEST_DIR` rather than this crate reading it directly,
/// because `env!` is expanded where it is written: read here, it would name *this* crate and
/// every guard built on it would scan the wrong tree and pass for the wrong reason.
pub fn Crate_Sources(manifest_directory: &str) -> Vec<String>
{
    let directory = Path::new(manifest_directory).join("src");
    let entries = std::fs::read_dir(&directory).expect("the crate has a src directory");

    let mut sources = Vec::new();
    for entry in entries
    {
        let path = entry.expect("a readable directory entry").path();
        if path.extension().is_some_and(|extension| return extension == "rs")
        {
            sources.push(std::fs::read_to_string(&path).expect("a readable source file"));
        }
    }

    assert!(!sources.is_empty(), "no sources were scanned, so this test proves nothing");
    return sources;
}

/// The name of every `pub fn` in `source` whose parameter list takes `&mut self`.
///
/// Whitespace is collapsed first, so a signature broken across lines cannot slip past, and the
/// parameter list is what is inspected, so a doc comment mentioning `&mut self` is not a
/// finding.
pub fn Mutating_Public_Methods(source: &str) -> Vec<String>
{
    let collapsed = source.split_whitespace().collect::<Vec<&str>>().join(" ");

    let mut found = Vec::new();
    for declaration in collapsed.split("pub fn ").skip(1)
    {
        let Some(signature) = declaration.split(')').next()
        else
        {
            continue;
        };

        if signature.contains("&mut self")
        {
            let name = signature.split('(').next().unwrap_or_default().trim();
            found.push(name.to_owned());
        }
    }

    return found;
}
