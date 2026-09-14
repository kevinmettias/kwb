//! What every check here needs to read the workspace it is checking.
//!
//! # Why this crate has a library at all
//!
//! Each file under `tests/` is its own binary, so nothing in one is visible from another. That
//! left `Repository_Root` written four times over, and a fifth copy was about to be added for
//! `KWB-75`'s bands projection — in an item whose whole subject is that one fact should have one
//! home.
//!
//! So the shared readers live here, and `KWB-79` finished the move: every file under `tests/`
//! reads the workspace through this one, and `boundaries.rs` carries a guard that fails when a
//! test file declares a reader this library already owns, so the copies cannot come back
//! quietly.
//!
//! # What the move found, which the debt note had not said
//!
//! It was recorded as mechanical. It was not. `boundaries.rs` and this library each had a
//! `Quoted_After`, and **they were not the same function** — see that function's own note. On
//! the manifests this repository has today they return the same answers, which is exactly why
//! nobody noticed, and two files read manifests believing they called one reader.
//!
//! The general form is this repository's most common defect, one level down from documents: two
//! things claim to be one fact and nothing proves they agree. A duplicate is not only a
//! maintenance cost, it is a place where a divergence can live unobserved — so consolidating is
//! a correctness move and not tidying, and the reconciliation was made deliberately rather than
//! by keeping whichever body happened to survive the merge.

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

/// The double-quoted value on the first line that *is* `prefix`, which is how a manifest states
/// a scalar.
///
/// Deliberately not a TOML parser. This crate reads manifests to check claims about them, and a
/// parser would be a dependency taken to answer a question that a prefix and a quote already
/// answer — `KWB-62` recorded the same reasoning for the board.
///
/// # Which of the two readers this is, and why
///
/// Until `KWB-79` there were two functions with this name. This one searched the whole text for
/// the prefix; `boundaries.rs`'s required a *line* to begin with it. They agreed on every
/// manifest in this repository, which is why the difference went unrecorded for as long as it
/// did, and they part on two inputs a manifest can easily hold:
///
/// - a commented key — `# name = "old"` above `name = "kwb-cli"` — where searching the whole
///   text finds the comment first and answers `old`;
/// - a longer key ending in the prefix — `package-name = "a"` above `name = "b"` — where
///   searching finds the tail of the longer key and answers `a`.
///
/// The line-anchored reading is right in both, so that is what survived. It is anchored to the
/// *trimmed* line rather than to column zero, which the discarded reader was not: a key indented
/// under a table is still that key, and nothing about being indented makes it a comment.
#[must_use]
pub fn Quoted_After(text: &str, prefix: &str) -> Option<String>
{
    return text
        .lines()
        .find(|line| return line.trim_start().starts_with(prefix))
        .and_then(|line| return line.split('"').nth(1))
        .map(str::to_owned);
}

/// Every workspace member's crate name, from the root manifest's `members` list.
///
/// Which lines count and which crates are excluded is `Member_Name_On`'s, stated there.
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
        if let Some(name) = Member_Name_On(line)
        {
            names.insert(name);
        }
    }

    return names;
}

/// The crate name a root-manifest line declares, when that line declares one.
///
/// A `members` list writes one quoted path per line, so a line is either a member or it is not
/// one -- the `members = [` opener, a closing bracket, a blank line, or one of the band comments
/// that say what a group is for. Anchoring to the *trimmed* line is what makes the reading right
/// for a member indented under the list, which every member but the first is.
///
/// The two test crates are refused here rather than by the caller because they are a property of
/// what a member *is* for this reader: `tests/contract` asserts the tables and `tests/integration`
/// is its sibling, so neither is a row in one.
fn Member_Name_On(line: &str) -> Option<String>
{
    let path = line.trim().strip_prefix('"')?.split('"').next()?;
    if path == "tests/contract" || path == "tests/integration"
    {
        return None;
    }

    return path.rsplit('/').next().map(str::to_owned);
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
