//! The README's band table against the real workspace, both directions.
//!
//! Mirrors the check `f:/repos/nomos/tests/contract` runs on itself: a crate joining the
//! workspace without joining the README table fails this test, and a README naming a
//! crate the workspace does not have fails it too. Deliberately simple — this repository
//! is new and has one such check, not the five Nomos accumulated over its own history.

use std::collections::BTreeSet;
use std::path::PathBuf;

fn Repository_Root() -> PathBuf
{
    return PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("tests/contract has a parent directory")
        .parent()
        .expect("the repository root is two levels above tests/contract")
        .to_path_buf();
}

/// Every workspace member's crate name, read from the root `Cargo.toml`'s `members`
/// list. `tests/contract` names itself and is excluded: it asserts the table, it is not
/// a row in it.
fn Workspace_Member_Crate_Names() -> BTreeSet<String>
{
    let manifest = std::fs::read_to_string(Repository_Root().join("Cargo.toml"))
        .expect("the root Cargo.toml should be readable");

    let mut names = BTreeSet::new();
    for line in manifest.lines()
    {
        let trimmed = line.trim();
        let Some(after_quote) = trimmed.strip_prefix('"') else { continue };
        let Some(path) = after_quote.split('"').next() else { continue };
        if path == "tests/contract" || path == "tests/integration"
        {
            continue;
        }
        let Some(name) = path.rsplit('/').next() else { continue };
        names.insert(name.to_owned());
    }

    return names;
}

/// Every crate name named in `README.md`'s Bands table.
fn Readme_Band_Table_Crate_Names() -> BTreeSet<String>
{
    let readme = std::fs::read_to_string(Repository_Root().join("README.md"))
        .expect("README.md should be readable");

    let mut names = BTreeSet::new();
    let mut in_band_table = false;
    for line in readme.lines()
    {
        if line.starts_with("| Band | Crate | Owns |")
        {
            in_band_table = true;
            continue;
        }
        if !in_band_table
        {
            continue;
        }
        if !line.starts_with('|')
        {
            break;
        }
        let columns: Vec<&str> = line.split('|').collect();
        let Some(crate_column) = columns.get(2) else { continue };
        let trimmed = crate_column.trim();
        let Some(name) = trimmed.strip_prefix('`').and_then(|rest| return rest.strip_suffix('`'))
        else
        {
            continue;
        };
        names.insert(name.to_owned());
    }

    return names;
}

#[test]
fn Test_Every_Workspace_Member_Should_Appear_In_The_Readme_Table()
{
    let workspace = Workspace_Member_Crate_Names();
    let readme = Readme_Band_Table_Crate_Names();

    let missing_from_readme: Vec<&String> = workspace.difference(&readme).collect();

    assert!(
        missing_from_readme.is_empty(),
        "these crates are workspace members with no row in README.md's band table: {missing_from_readme:?}"
    );
}

#[test]
fn Test_Every_Readme_Table_Row_Should_Name_A_Real_Workspace_Member()
{
    let workspace = Workspace_Member_Crate_Names();
    let readme = Readme_Band_Table_Crate_Names();

    let missing_from_workspace: Vec<&String> = readme.difference(&workspace).collect();

    assert!(
        missing_from_workspace.is_empty(),
        "README.md's band table names crates the workspace does not have: {missing_from_workspace:?}"
    );
}
