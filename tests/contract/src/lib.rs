//! What every check here needs to read the workspace it is checking.
//!
//! # Why this crate has a library at all
//!
//! Each file under `tests/` is its own binary, so nothing in one is visible from another. That
//! left `Repository_Root` written four times over, and a fifth copy was about to be added for
//! `KWB-75`'s bands projection — in an item whose whole subject is that one fact should have one
//! home.
//!
//! So the shared readers live here. `projections.rs` uses them; `boundaries.rs`, `literals.rs`
//! and `commands.rs` still carry their own copies, which is **owed and not done** rather than
//! overlooked — moving them is a mechanical change to three files that have nothing to do with
//! the bands table, and bundling it here would mean one item doing two things.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

/// The repository root, from this crate's own manifest.
///
/// # Panics
///
/// If this crate is not two levels below the root, which would mean the workspace had been
/// rearranged without these checks being moved with it -- a loud failure is right, because every
/// check here reads paths relative to what this returns.
#[must_use]
pub fn Repository_Root() -> PathBuf
{
    return PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("tests/contract sits two levels below the root")
        .to_path_buf();
}

/// The first double-quoted value after a prefix, which is how a manifest states a scalar.
///
/// Deliberately not a TOML parser. This crate reads manifests to check claims about them, and a
/// parser would be a dependency taken to answer a question that a prefix and a quote already
/// answer — `KWB-62` recorded the same reasoning for the board.
#[must_use]
pub fn Quoted_After(text: &str, prefix: &str) -> Option<String>
{
    let after = text.split_once(prefix)?.1;
    let (value, _) = after.trim_start().trim_start_matches('"').split_once('"')?;

    return Some(value.to_owned());
}

/// Every workspace member's crate name, from the root manifest's `members` list.
///
/// `tests/contract` names itself and is excluded: it asserts the tables, it is not a row in one.
///
/// # Panics
///
/// If the root manifest cannot be read. There is no useful empty answer: a check that silently
/// found no members would pass on every table, which is the vacuous result these guards exist
/// to avoid.
#[must_use]
pub fn Workspace_Members() -> BTreeSet<String>
{
    let manifest = std::fs::read_to_string(Repository_Root().join("Cargo.toml"))
        .expect("the root Cargo.toml should be readable");

    let mut names = BTreeSet::new();
    for line in manifest.lines()
    {
        let trimmed = line.trim();
        let Some(after_quote) = trimmed.strip_prefix('"')
        else
        {
            continue;
        };
        let Some(path) = after_quote.split('"').next()
        else
        {
            continue;
        };
        if path == "tests/contract" || path == "tests/integration"
        {
            continue;
        }
        if let Some(name) = path.rsplit('/').next()
        {
            names.insert(name.to_owned());
        }
    }

    return names;
}

/// What a document carries between two markers, when it carries them.
///
/// The markers are how a reader knows a block is generated and not to be edited by hand.
#[must_use]
pub fn Between(text: &str, opens: &str, closes: &str) -> Option<String>
{
    let after = text.split_once(opens)?.1;
    let (block, _) = after.split_once(closes)?;

    return Some(block.trim().to_owned());
}
