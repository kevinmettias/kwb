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

// ---- KWB-39: one description per crate, in the two places that must each carry one ----

/// Every crate's `description`, from its manifest.
fn Manifest_Descriptions() -> BTreeMap<String, String>
{
    let mut found = BTreeMap::new();
    Collect_Manifests(&Repository_Root().join("crates"), &mut found);
    assert!(!found.is_empty(), "no manifests were scanned, so this test proves nothing");
    return found;
}

/// Walk for `Cargo.toml` files, reading the name and description out of each.
fn Collect_Manifests(directory: &std::path::Path, found: &mut BTreeMap<String, String>)
{
    let Ok(entries) = std::fs::read_dir(directory)
    else
    {
        return;
    };

    for entry in entries
    {
        let path = entry.expect("a readable directory entry").path();
        if path.is_dir()
        {
            Collect_Manifests(&path, found);
            continue;
        }
        if path.file_name().is_none_or(|name| return name != "Cargo.toml")
        {
            continue;
        }

        let text = std::fs::read_to_string(&path).expect("a readable manifest");
        let name = Quoted_After(&text, "name = ");
        let description = Quoted_After(&text, "description = ");
        if let (Some(name), Some(description)) = (name, description)
        {
            found.insert(name, description);
        }
    }
}

/// The double-quoted value on the first line beginning with `prefix`.
fn Quoted_After(text: &str, prefix: &str) -> Option<String>
{
    return text
        .lines()
        .find(|line| return line.starts_with(prefix))
        .and_then(|line| return line.split('"').nth(1))
        .map(str::to_owned);
}

/// Every crate row in `README.md`'s Bands table, as `crate -> what it owns`.
fn Readme_Band_Descriptions() -> BTreeMap<String, String>
{
    let readme = std::fs::read_to_string(Repository_Root().join("README.md"))
        .expect("README.md should be readable");

    let mut rows = BTreeMap::new();
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

        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        let (Some(crate_cell), Some(owns)) = (cells.get(2), cells.get(3))
        else
        {
            continue;
        };
        if let Some(name) = crate_cell
            .strip_prefix('`')
            .and_then(|rest| return rest.strip_suffix('`'))
        {
            rows.insert(name.to_owned(), (*owns).to_owned());
        }
    }

    return rows;
}

/// A crate says one thing about itself, and says it the same way in both places.
///
/// # What this caught, and what it cannot
///
/// `kwb-platform` was described in its manifest **and** in this table as offering port traits
/// for clock, filesystem, lock and process. It exports two traits, neither of them in that
/// list, and its own `lib.rs` says the clock, lock and process ports are not there yet — one
/// fact in three places, two agreeing with each other and both disagreeing with the code.
/// `kwb-retrieval` advertised semantic and hybrid queries, one of which `D-004` holds the
/// ground for and this repository has deliberately refused to stub.
///
/// **This check would not have caught either**, because the two copies agreed. What fixed them
/// was making each description a *charter* — what the crate is for — instead of an inventory of
/// what it contains. An inventory is a copy of the source and drifts whenever the source
/// changes; a charter does not. That is the real remedy and it is not mechanizable.
///
/// What this check does is close the remaining drift mode: two places must still each carry a
/// description, and now they cannot disagree.
#[test]
fn Test_Every_Crate_Should_Describe_Itself_The_Same_Way_In_Both_Places()
{
    let manifests = Manifest_Descriptions();
    let readme = Readme_Band_Descriptions();

    let mut disagreements: Vec<String> = Vec::new();
    for (name, owns) in &readme
    {
        let Some(description) = manifests.get(name)
        else
        {
            continue;
        };
        if description != owns
        {
            disagreements.push(format!("{name}\n  README:   {owns}\n  manifest: {description}"));
        }
    }

    assert!(
        disagreements.is_empty(),
        "a crate describes itself differently in two places, so one of them is already wrong \
         and a reader cannot tell which: {}",
        disagreements.join("\n")
    );
}

// ---- KWB-40: the router routes everything there is ----

/// A reader following the operating contract can reach every authority this repository has.
///
/// # Why this is a test
///
/// `AGENTS.md` opens with *"read the authority, do not infer the architecture from the code
/// nearest to your task"* and then gives a table mapping each question to where it is answered.
/// It omitted two of the three directories under `docs/`: the measured surveys the records cite
/// constantly, and the observations added this session.
///
/// So a session doing exactly what the contract says would never have found the prototype
/// inventory that `KWB-9` spent an item producing. That is the worst place for a routing table
/// to drift, because **a reader who cannot find an authority does not know one is missing** —
/// unlike a stale claim, which at least says something checkable.
#[test]
fn Test_Every_Authority_Under_Docs_Should_Be_Routed_By_The_Operating_Contract()
{
    let contract = std::fs::read_to_string(Repository_Root().join("AGENTS.md"))
        .expect("AGENTS.md should be readable");

    let entries = std::fs::read_dir(Repository_Root().join("docs"))
        .expect("docs/ should be readable");

    let mut unrouted: Vec<String> = Vec::new();
    let mut seen = 0_usize;
    for entry in entries
    {
        let path = entry.expect("a readable directory entry").path();
        if !path.is_dir()
        {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| return name.to_str())
        else
        {
            continue;
        };
        seen = seen.saturating_add(1);
        if !contract.contains(&format!("docs/{name}/"))
        {
            unrouted.push(name.to_owned());
        }
    }

    assert!(seen >= 3, "only {seen} authorities were scanned, so this test proves little");
    assert!(
        unrouted.is_empty(),
        "these authorities exist and AGENTS.md routes nobody to them, so a session following \
         the operating contract cannot find them: {unrouted:?}"
    );
}

// ---- KWB-44: the readers are shown to read ----

/// The relation reader finds what a frontmatter declares, and nothing a body mentions.
///
/// The guard above passes when every relation is reachable both ways. A reader that found **no**
/// relations would pass it just as quietly, having checked nothing — which is why this exists.
#[test]
fn Test_The_Relation_Reader_Should_Find_Declared_Relations()
{
    let mut records = BTreeMap::new();
    records.insert(
        "D-900".to_owned(),
        "---\nid: D-900\nrelations:\n  - target: D-901\n    type: amends\n  - target: D-902\n    \
         type: relates-to\n---\n\nBody text mentioning D-903, which is not a relation.\n"
            .to_owned(),
    );

    let relations = Declared_Relations(&records);

    assert_eq!(
        relations,
        vec![
            ("D-900".to_owned(), "D-901".to_owned()),
            ("D-900".to_owned(), "D-902".to_owned()),
        ],
        "the reader must find every declared relation and nothing the body merely mentions"
    );
}

#[test]
fn Test_The_Relation_Reader_Should_Find_Nothing_Where_Nothing_Is_Declared()
{
    let mut records = BTreeMap::new();
    records.insert(
        "D-900".to_owned(),
        "---\nid: D-900\ntags:\n  - storage\n---\n\nA body citing D-901 in prose.\n".to_owned(),
    );

    assert!(
        Declared_Relations(&records).is_empty(),
        "a citation in prose is not a declared relation"
    );
}

/// The quoted-value reader, which both description checks depend on.
#[test]
fn Test_The_Manifest_Reader_Should_Take_The_Quoted_Value()
{
    let manifest = "[package]\nname = \"kwb-example\"\ndescription = \"What it is for.\"\n";

    assert_eq!(Quoted_After(manifest, "name = "), Some("kwb-example".to_owned()));
    assert_eq!(Quoted_After(manifest, "description = "), Some("What it is for.".to_owned()));
    assert_eq!(
        Quoted_After(manifest, "absent = "),
        None,
        "an absent key must be None rather than an empty string, or a crate with no description \
         would compare equal to one whose description is empty"
    );
}

// ---- KWB-45: the README shows commands that exist ----

/// Every `kwb` verb the README shows is one the composition root dispatches.
///
/// # Why this is a test
///
/// A usage example rots the way every copy does: the code moves and the example does not, and
/// the reader who follows it is the one who finds out. This repository has spent a session
/// fixing five copies of one fact that were corrected in some places and not others, so adding
/// a sixth on the way out — with no check — would be a poor ending.
///
/// It scans the composition root's own dispatch rather than running the binary, so the check
/// costs nothing and cannot leave a store directory behind.
#[test]
fn Test_Every_Command_The_Readme_Shows_Should_Be_One_The_Binary_Dispatches()
{
    let readme = std::fs::read_to_string(Repository_Root().join("README.md"))
        .expect("README.md should be readable");
    let dispatch =
        std::fs::read_to_string(Repository_Root().join("crates/host/kwb-cli/src/main.rs"))
            .expect("the composition root should be readable");

    let mut missing: Vec<String> = Vec::new();
    for verb in Verbs_Shown(&readme)
    {
        if !dispatch.contains(&format!("&\"{verb}\""))
        {
            missing.push(verb);
        }
    }

    assert!(
        missing.is_empty(),
        "the README shows commands the binary does not dispatch, so a reader following it would \
         find out the hard way: {missing:?}"
    );
}

/// Every `kwb-mcp` tool the README names is one the surface declares.
#[test]
fn Test_Every_Tool_The_Readme_Names_Should_Be_One_The_Surface_Declares()
{
    let readme = std::fs::read_to_string(Repository_Root().join("README.md"))
        .expect("README.md should be readable");
    let surface = std::fs::read_to_string(Repository_Root().join("crates/host/kwb-mcp/src/lib.rs"))
        .expect("the tool surface should be readable");

    let mut missing: Vec<String> = Vec::new();
    for line in readme.lines()
    {
        let Some(rest) = line.trim().strip_prefix("$ kwb-mcp ")
        else
        {
            continue;
        };
        // `kwb-mcp <store> <tool> ...` — the tool is the second word, when there is one.
        if let Some(tool) = rest.split_whitespace().nth(1)
        {
            if !surface.contains(&format!("name: \"{tool}\""))
            {
                missing.push(tool.to_owned());
            }
        }
    }

    assert!(
        missing.is_empty(),
        "the README names tools the surface does not declare: {missing:?}"
    );
}

/// Every `kwb` verb the README shows, in a console block or named in prose.
///
/// Both, because a verb mentioned in a sentence is as much a promise to a reader as one in an
/// example. The first version of this collector saw only the console blocks, which would have
/// left `retire` and `supersede` unchecked -- two of the three verbs the README names. A guard
/// covering a third of what it claims to is worse than none, because it reads as coverage.
fn Verbs_Shown(readme: &str) -> Vec<String>
{
    let mut verbs: Vec<String> = Vec::new();

    for line in readme.lines()
    {
        if let Some(rest) = line.trim().strip_prefix("$ kwb ")
        {
            Remember(&mut verbs, rest.split_whitespace().next().unwrap_or_default());
        }
    }

    for fragment in readme.split("`kwb ").skip(1)
    {
        let Some(inside) = fragment.split('`').next()
        else
        {
            continue;
        };
        Remember(&mut verbs, inside.split_whitespace().next().unwrap_or_default());
    }

    assert!(verbs.len() >= 3, "only {verbs:?} were found, so this guard covers almost nothing");
    return verbs;
}

/// Keep a verb once.
fn Remember(verbs: &mut Vec<String>, verb: &str)
{
    if !verb.is_empty() && !verbs.iter().any(|held| return held == verb)
    {
        verbs.push(verb.to_owned());
    }
}

// ---- KWB-49: the ownership route reaches both records that decide it ----

/// Both records that decide ownership are routed, not just the one that is quoted.
///
/// # Why this names identifiers when the test above scans a directory
///
/// The test above enumerates `docs/` and asks whether each authority is routed, which is the
/// stronger shape: it finds an authority nobody thought to add. It cannot be used here, because
/// these two records live in **another repository**. There is nothing under this root to
/// enumerate, and a test that cannot enumerate has to name.
///
/// # What it guards, and what it deliberately does not
///
/// It guards against the rows being deleted or the second one never being noticed. It does
/// **not** check that the sibling's records exist or say what this file claims they say — a
/// test that read across the repository boundary would fail for reasons this repository cannot
/// fix, and a test that fails for reasons you cannot fix gets turned off, taking the part that
/// worked with it.
///
/// # Why the second row is load-bearing
///
/// `ARC-ECOSYSTEM-001` adopts `D-122` and quotes it: a shared mechanism moves to XVPE only
/// after two products have demonstrated materially identical domain-neutral semantics. `D-135`
/// is later and narrows that to code which *started* product-specific. The two give opposite
/// instructions for code designed for sharing from the outset, and `D-135` names this
/// repository as one of the two products it was written for.
///
/// `D-135` declares a relation to `ARC-ECOSYSTEM-001`; `ARC-ECOSYSTEM-001` does not mention
/// `D-135`. So a session that followed the route and stopped at the first record would read the
/// un-narrowed gate — the same one-directional-relation defect `KWB-38` fixed inside this
/// repository, one repository over, where `KWB-38` could not reach.
#[test]
fn Test_Both_Records_That_Decide_Ownership_Should_Be_Routed()
{
    let contract = std::fs::read_to_string(Repository_Root().join("AGENTS.md"))
        .expect("AGENTS.md should be readable");

    let unrouted: Vec<&str> = ["ARC-ECOSYSTEM-001", "D-135"]
        .into_iter()
        .filter(|record| return !contract.contains(record))
        .collect();

    assert!(
        unrouted.is_empty(),
        "AGENTS.md routes ownership to only one of the two records that decide it, so a \
         session following the operating contract reads a burden of proof that a later \
         record narrowed: {unrouted:?}"
    );

    assert!(
        contract.contains("narrow"),
        "both records are named and nothing says D-135 narrows the gate ARC-ECOSYSTEM-001 \
         quotes, so a reader has two rows and no reason to prefer either"
    );
}
