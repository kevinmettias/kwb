//! The README's band table against the real workspace, both directions.
//!
//! Mirrors the check `f:/repos/nomos/tests/contract` runs on itself: a crate joining the
//! workspace without joining the README table fails this test, and a README naming a
//! crate the workspace does not have fails it too. Deliberately simple — this repository
//! is new and has one such check, not the five Nomos accumulated over its own history.

use std::collections::{BTreeMap, BTreeSet};
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

// ---- KWB-38: a relation is discoverable from both ends, or it is not discoverable ----

/// Every record's identifier, mapped to its text.
fn Records() -> BTreeMap<String, String>
{
    let directory = Repository_Root().join("docs/records");
    let entries = std::fs::read_dir(&directory).expect("docs/records should be readable");

    let mut records = BTreeMap::new();
    for entry in entries
    {
        let path = entry.expect("a readable directory entry").path();
        if path.extension().is_some_and(|extension| return extension == "md")
        {
            let name = path
                .file_name()
                .and_then(|name| return name.to_str())
                .unwrap_or_default();
            let mut parts = name.split('-');
            let (Some(prefix), Some(number)) = (parts.next(), parts.next())
            else
            {
                continue;
            };
            let text = std::fs::read_to_string(&path).expect("a readable record");
            records.insert(format!("{prefix}-{number}"), text);
        }
    }

    assert!(!records.is_empty(), "no records were scanned, so this test proves nothing");
    return records;
}

/// Every relation a record's frontmatter declares, as `(source, target)`.
fn Declared_Relations(records: &BTreeMap<String, String>) -> Vec<(String, String)>
{
    let mut relations = Vec::new();
    for (identifier, text) in records
    {
        let Some(frontmatter) = text.split("---").nth(1)
        else
        {
            continue;
        };
        for line in frontmatter.lines()
        {
            let trimmed = line.trim();
            let Some(target) = trimmed.strip_prefix("- target:")
            else
            {
                continue;
            };
            relations.push((identifier.clone(), target.trim().to_owned()));
        }
    }
    return relations;
}

/// A record that answers, amends or builds on another must be reachable **from** that other.
///
/// # Why this is a test and not a convention
///
/// Measured before `KWB-38`: of 27 declared relations, **24 had no counterpart in the record
/// they pointed at**. So the record graph was almost entirely backward-linked, and a reader of
/// an older record could not reach the newer one that answered it.
///
/// That is not a tidiness complaint. `D-008` went on asserting that durability was uncovered
/// after `D-014` decided it and `KWB-30` built it; `D-012` said nothing survives the process
/// after `KWB-33` and `KWB-34` made knowledge survive it; `D-013` sent a reader to a crate the
/// evidence vocabulary had already left. Every correction existed and none was reachable from
/// where the false statement was.
///
/// `AGENTS.md` routes every *why* question to `docs/records/` and calls them canonical. A
/// canonical record asserting something the repository stopped doing is the stale-premise
/// defect one level above the code, in the documents that are supposed to be the fix for it.
#[test]
fn Test_Every_Declared_Relation_Should_Be_Reachable_From_Both_Ends()
{
    let records = Records();

    let mut one_way: Vec<String> = Vec::new();
    for (source, target) in Declared_Relations(&records)
    {
        let Some(target_text) = records.get(&target)
        else
        {
            one_way.push(format!("{source} -> {target} (no such record)"));
            continue;
        };
        if !target_text.contains(&source)
        {
            one_way.push(format!("{source} -> {target} (target never names {source})"));
        }
    }

    assert!(
        one_way.is_empty(),
        "these relations are declared in one direction only, so a reader of the target cannot \
         reach the record that relates to it: {one_way:#?}"
    );
}
