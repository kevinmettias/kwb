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

/// The crate name a root-manifest line declares, when that line declares a crate the bands table
/// is about.
///
/// A `members` list writes one quoted path per line, so a line is either a member or it is not
/// one -- the `members = [` opener, a closing bracket, a blank line, or one of the band comments
/// that say what a group is for. Anchoring to the *trimmed* line is what makes the reading right
/// for a member indented under the list, which every member but the first is.
///
/// # Which members it refuses, and why that is a property and not a list
///
/// A crate under `tests/` is refused here rather than by the caller, because it is a property of
/// what a member *is* for this reader: the table `Workspace_Members` feeds is the bands table,
/// and a test crate is reached only through a `dev-dependency`, is linked into no shipped binary,
/// and so declares no band and is owed no row.
///
/// Until `KWB-109` that refusal was two path literals -- `tests/contract` and `tests/integration`
/// -- and they had gone stale in opposite directions at once. `tests/integration` is neither a
/// member nor a directory and had not been one for some time, while `tests/guards` is a member and
/// was not named, so two checks failed naming a README that was correct. A rule keyed on where a
/// crate lives cannot fail that way: the directory is refused whole, so a test crate joining the
/// workspace is refused the day it joins and a path that stops existing stops being refused the
/// day it does, with nothing here to edit either time.
fn Member_Name_On(line: &str) -> Option<String>
{
    let path = line.trim().strip_prefix('"')?.split('"').next()?;
    if path.split('/').next() == Some("tests")
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

#[cfg(test)]
mod tests
{
    use std::collections::BTreeSet;

    use super::Member_Name_On;

    /// The member names a fixture `members` list declares, read through the same function the
    /// root manifest is read through.
    fn Members_Of(fixture: &str) -> BTreeSet<String>
    {
        let mut names = BTreeSet::new();
        for line in fixture.lines()
        {
            if let Some(name) = Member_Name_On(line)
            {
                names.insert(name);
            }
        }

        return names;
    }

    /// A member list in the root manifest's shape, carrying a test crate *and* a production one.
    ///
    /// The production crate is not decoration: without it this fixture would be satisfied by a
    /// reader that answered nothing at all, which is the failure the pair below exists to rule out.
    const WITH_A_TEST_CRATE: &str = "members = [\n    # A band.\n    \"crates/kernel/kwb-model\",\n    \"tests/guards\",\n]\n";

    /// The same shape with no test crate in it, so a clean answer can be told from an empty one.
    const WITH_PRODUCTION_CRATES_ONLY: &str = "members = [\n    \"crates/kernel/kwb-model\",\n    \"crates/host/kwb-cli\",\n]\n";

    /// A path under `tests/` that no crate occupies yet, which is the shape of one added later.
    const WITH_A_TEST_CRATE_THAT_DOES_NOT_EXIST_YET: &str = "members = [\n    \"tests/not-yet-a-crate\",\n]\n";

    /// The rule, proved in both directions.
    ///
    /// A one-directional proof is worthless here: a reader that returned `None` unconditionally
    /// would pass the exclusion half, and a reader that returned names unconditionally would pass
    /// the admission half. Only a fixture that carries both kinds, and a result that is checked
    /// against both, shows that the rule is keyed on where a crate lives.
    ///
    /// Both halves of the stale literal are covered too. `tests/guards` is the test crate that
    /// joined the workspace while the literal did not name it, and the last fixture is a path
    /// under `tests/` that no crate has ever occupied -- the direction `tests/integration` failed
    /// from, where a literal went on refusing a path that no longer exists.
    #[test]
    fn Test_A_Test_Crate_Should_Be_Refused_By_Where_It_Lives()
    {
        let mixed = Members_Of(WITH_A_TEST_CRATE);

        assert!(
            !mixed.contains("guards"),
            "a crate under tests/ was read as a member the bands table should carry: {mixed:?}"
        );
        assert!(
            mixed.contains("kwb-model"),
            "the fixture's production crate was not read either, so refusing the test crate \
             proves nothing about the rule: {mixed:?}"
        );

        let production = Members_Of(WITH_PRODUCTION_CRATES_ONLY);
        assert_eq!(
            production,
            BTreeSet::from(["kwb-model".to_owned(), "kwb-cli".to_owned()]),
            "a fixture carrying only production crates did not admit exactly those, so the rule \
             is satisfied by a scan that finds nothing"
        );

        let unoccupied = Members_Of(WITH_A_TEST_CRATE_THAT_DOES_NOT_EXIST_YET);
        assert!(
            unoccupied.is_empty(),
            "a path under tests/ that no crate occupies was read as a member, so the rule is \
             still keyed on a path literal rather than on where a crate lives: {unoccupied:?}"
        );
    }
}
