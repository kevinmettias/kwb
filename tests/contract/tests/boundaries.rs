//! The README's band table against the real workspace, both directions.
//!
//! Mirrors the check `f:/repos/nomos/tests/contract` runs on itself: a crate joining the
//! workspace without joining the README table fails this test, and a README naming a
//! crate the workspace does not have fails it too. Deliberately simple — this repository
//! is new and has one such check, not the five Nomos accumulated over its own history.

use std::collections::{BTreeMap, BTreeSet};

use kwb_contract_tests::Quoted_After;
use kwb_contract_tests::Repository_Root;
use kwb_contract_tests::Workspace_Members;

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
    let workspace = Workspace_Members();
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
    let workspace = Workspace_Members();
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
    let documents = Relating_Documents();

    let mut one_way: Vec<String> = Vec::new();
    for (source, target) in Declared_Relations(&documents)
    {
        let Some(target_text) = documents.get(&target)
        else
        {
            one_way.push(format!("{source} -> {target} (no such record or observation)"));
            continue;
        };
        if !Referenced_By(target_text).contains(&source)
        {
            one_way.push(format!("{source} -> {target} (not in {target}'s Referenced By)"));
        }
    }

    assert!(
        one_way.is_empty(),
        "these relations are declared in one direction only, so a reader of the target cannot \
         reach the document that relates to it: {one_way:#?}"
    );
}

/// Every document that may declare a relation: the records, and the observations.
///
/// # Why this is not `Records()`, and must not become it
///
/// `KWB-85` widened the relation checks to cover `docs/observations/`, because an observation
/// declaring `relates-to D-005` was invisible to them in both directions — the one relation in
/// this repository unreachable from its target, which is the condition `KWB-38` exists for.
/// `D-005`'s own *Referenced By* note claimed it was checked both ways, and for observations
/// both halves were false, in the paragraph whose job is to say the list is trustworthy.
///
/// It widens the *relation* readers and nothing else. Widening [`Records`] itself would bring
/// observations under the record version rule, and `OD-LEDGER-001` — version 2 with no
/// `## Amendment` section, correctly, because an observation is revised in place — would fail
/// it. `Test_An_Observation_May_Be_Revised_Without_An_Amendment_Section` is the guard that
/// catches anyone doing that, including a later reader of this comment who thinks the two maps
/// ought to be one.
fn Relating_Documents() -> BTreeMap<String, String>
{
    let mut documents = Records();
    documents.extend(Observations());

    return documents;
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
/// # What this can show, and what it cannot
///
/// It proves the two copies **agree**. It cannot prove either is **true**, and `KWB-75` is the
/// example: `kwb-platform-xvpe`'s description said it adopts *the persistent map versioned state
/// is built on* long after `KWB-64`, `KWB-66` and `KWB-72` had added the clock, the chunker and
/// the inference surface. The README said the same thing. Both agreed, both were stale, and this
/// test passed throughout — by construction, because agreement is all it asks about.
///
/// The bands table is now projected from these manifests, so the agreement question is gone:
/// there is one description and the document renders it. **The truth question stays a person's**,
/// and no check here replaces reading what a crate does and what it says it does.
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

    // Against the table, not against the source text. This looked for the match arm's `&"verb"`
    // until `KWB-74` moved the verbs into `VERBS`, at which point it would have reported every
    // verb as undispatched -- a guard that fails for the wrong reason, which is worse than one
    // that passes for the wrong reason because it teaches people to ignore it.
    let dispatched = Verbs_Dispatched(&dispatch);
    assert!(
        dispatched.len() >= 4,
        "only {dispatched:?} were found in the table, so this guard covers almost nothing"
    );

    // `help` and its aliases are answered by the binary and are deliberately not capabilities in
    // the table, so a README that mentions `kwb help` is not naming something undispatched.
    let aliases = ["help", "--help", "-h"];
    let mut missing: Vec<String> = Vec::new();
    for verb in Verbs_Shown(&readme)
    {
        let known = dispatched.iter().any(|held| return *held == verb)
            || aliases.contains(&verb.as_str());
        if !known
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

/// Every verb the binary dispatches is one the README shows.
///
/// # The direction that was missing
///
/// The test above catches a promise the binary cannot keep. This catches a capability nobody is
/// told about, and the repository grew one the day this was written: `KWB-60` added
/// `kwb history` and nothing complained, because the only check ran README-to-binary.
///
/// `AGENTS.md` routes *what exists* to `README.md`, so a verb the binary answers and the README
/// never shows is a capability a reader cannot find. `KWB-56` corrected that file for claiming
/// what the code does not do; this is the same file failing the other way, and the bands table
/// and the record relations are both already checked in both directions.
///
/// `help` is not a capability a reader has to be told about in prose — it is how they find the
/// rest — so it is exempt, and the aliases it answers to with it.
#[test]
fn Test_Every_Verb_The_Binary_Dispatches_Should_Be_One_The_Readme_Shows()
{
    let readme = std::fs::read_to_string(Repository_Root().join("README.md"))
        .expect("README.md should be readable");
    let dispatch =
        std::fs::read_to_string(Repository_Root().join("crates/host/kwb-cli/src/main.rs"))
            .expect("the composition root should be readable");

    let shown = Verbs_Shown(&readme);
    let dispatched = Verbs_Dispatched(&dispatch);

    assert!(
        dispatched.len() >= 4,
        "only {dispatched:?} were found in the dispatch, so this guard covers almost nothing"
    );

    let undocumented: Vec<&String> = dispatched
        .iter()
        .filter(|verb| return !shown.iter().any(|seen| return seen == *verb))
        .collect();

    assert!(
        undocumented.is_empty(),
        "the binary answers these and the README never mentions them, so a reader sent to that \
         file for what exists cannot find them: {undocumented:?}"
    );
}

/// Every verb the composition root's table declares.
///
/// # Why this reads a table and not the match arms
///
/// Until `KWB-74` each verb was written twice in `main.rs` — once as a match arm and once as a
/// usage line — and this parsed the arms. `KWB-74` made `VERBS` the one declaration that the
/// dispatch runs from and the help renders from, so the arms are gone and the table is where
/// the canonical answer lives.
///
/// `help` and its aliases are absent from the table rather than filtered out of it here, which
/// is the better place for that fact: they are how a reader finds the rest rather than a
/// capability to be listed, and a list that had to be filtered would be a list that included
/// them.
fn Verbs_Dispatched(dispatch: &str) -> Vec<String>
{
    let Some((_, table)) = dispatch.split_once("const VERBS:")
    else
    {
        return Vec::new();
    };
    let table = table
        .split_once("\n];")
        .map_or(table, |(inside, _)| return inside);

    let mut verbs: Vec<String> = Vec::new();
    for fragment in table.split("name: \"").skip(1)
    {
        let Some(inside) = fragment.split('"').next()
        else
        {
            continue;
        };
        Remember(&mut verbs, inside);
    }

    return verbs;
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

/// The identifiers a record's `Referenced By` section lists.
///
/// # Why the section and not the whole text
///
/// Until `KWB-55` the reachability test asked whether the target's text contained the source
/// identifier **anywhere**. That passes on a record that mentions another in passing, so a
/// `Referenced By` section could be stale, incomplete, or list records that relate to nothing,
/// and the test would stay green — while eleven records carried a note saying it *"fails if it
/// is wrong"*.
///
/// Reading the section is what makes that sentence true. It is also free: every one of the
/// eleven relation targets already had the section, so no record was restructured to satisfy
/// the stricter check.
fn Referenced_By(record: &str) -> Vec<String>
{
    let Some((_, listed)) = record.split_once("## Referenced By")
    else
    {
        return Vec::new();
    };

    return listed
        .lines()
        .filter_map(|line| return line.trim().strip_prefix("- "))
        .map(|entry| return entry.trim().trim_matches('`').to_owned())
        .collect();
}

/// A `Referenced By` entry that no record declares a relation to.
///
/// # The other direction, and why it is the half that was missing
///
/// A list can be wrong by omission or by invention, and only the first was ever checked. The
/// invented entry is the worse of the two: it sends a reader to a record that does not answer,
/// amend or build on this one, carrying the authority of a section the note called generated.
///
/// A claim about what relates to what is a claim, and this repository checks its claims.
#[test]
fn Test_No_Record_Should_Claim_A_Reference_Nobody_Declared()
{
    let documents = Relating_Documents();
    let declared = Declared_Relations(&documents);

    let mut invented: Vec<String> = Vec::new();
    for (identifier, text) in &documents
    {
        for listed in Referenced_By(text)
        {
            let is_declared = declared
                .iter()
                .any(|(source, target)| return source == &listed && target == identifier);
            if !is_declared
            {
                invented.push(format!("{identifier} lists {listed}, which declares no relation"));
            }
        }
    }

    assert!(
        invented.is_empty(),
        "these Referenced By entries send a reader to a record that does not relate back, with \
         the authority of a list this repository calls checked: {invented:#?}"
    );
}

// ---- KWB-62: a condition stated as items is checked against the board ----

/// Every item a record names as a met condition, as `(record, item)`.
///
/// # The marker, and why there is one
///
/// `D-004`'s amendment says *a list of held subjects goes stale in both directions, and this one
/// went stale silently because nothing re-reads it against the board*, and asks for exactly that
/// mechanism. This is it.
///
/// It reads only a line beginning `**Condition met:**`, because a record that merely *mentions*
/// an item is not stating a condition. Measured across every record when this was written: four
/// passages name an item near a condition-like phrase and only two are conditions —
/// `D-008`'s is prose and was answered by `D-012`, reachable from its own back-link, and
/// `D-004`'s coverage entry names `KWB-4` for something it *owes*, not for what would unstrand
/// it. A guard reading all four would be wrong half the time, and a guard that is wrong half the
/// time is switched off.
fn Conditions_Named(records: &BTreeMap<String, String>) -> Vec<(String, String)>
{
    let mut named = Vec::new();
    for (identifier, text) in records
    {
        for line in text.lines()
        {
            let Some(rest) = line.trim().strip_prefix("**Condition met:**")
            else
            {
                continue;
            };
            for item in rest.split(',')
            {
                let item = item.trim().trim_matches('`').trim();
                if !item.is_empty()
                {
                    named.push((identifier.clone(), item.to_owned()));
                }
            }
        }
    }

    return named;
}

/// Every item named in a met condition is actually done on the board.
///
/// # Why this direction
///
/// The record claims the condition is satisfied. That claim is checkable, and it is the one that
/// rots: `D-004`'s entry was correct when written and false **495 seconds later**, because
/// `KWB-5` closed eight minutes after the amendment holding it was authored. No amount of care
/// re-reads a list inside eight minutes; a test does it on every run.
///
/// It also catches the reverse mistake — a record announcing a condition met before the item
/// closes, which would be a subject declared workable while its predecessor is still open.
#[test]
fn Test_Every_Condition_A_Record_Calls_Met_Should_Be_Met_On_The_Board()
{
    let records = Records();
    let named = Conditions_Named(&records);

    assert!(
        !named.is_empty(),
        "no record states a condition in the checkable form, so this guard proves nothing"
    );

    let board = std::fs::read_to_string(Repository_Root().join("work/ledger.json"))
        .expect("the board should be readable");

    let mut unmet: Vec<String> = Vec::new();
    for (record, item) in named
    {
        // Read from the board, anchored on the field rather than on a substring near it. The
        // first attempt looked for `"done"` after the identifier and reported four defects that
        // did not exist, because the board writes `"Done"`. A guard whose parser is wrong does
        // not report nothing -- it reports something false, with the authority of a test.
        // `Claimed` counts, and `KWB-64` is why. A record amended **by** the item that satisfies
        // its condition writes the marker while that item is still held -- the predicate runs
        // before `finish`, so requiring `Done` here makes the only item that can honestly add a
        // marker the one item that cannot. That is a guard fighting the workflow rather than a
        // defect.
        //
        // It gives nothing up. An abandoned claim returns the item to `ready` and this fires
        // then, and a declined one fires immediately, so a marker whose item never lands is
        // still caught -- just at the moment the board says so rather than before.
        let state = State_Of(&board, &item).unwrap_or_default();
        let landing = state.eq_ignore_ascii_case("done") || state.eq_ignore_ascii_case("claimed");

        if !landing
        {
            unmet.push(format!("{record} calls {item} met and the board says {state:?}"));
        }
    }

    assert!(
        unmet.is_empty(),
        "a record says a condition is satisfied and the board disagrees, so a subject is \
         declared workable while what it waits on is still open: {unmet:#?}"
    );
}

/// An item's `state` on the board, read from the field and not from text near it.
///
/// The board is JSON and this reads it as text, which is a choice: `tests/contract` depends on
/// nothing, and a check that exists to catch a stale claim should not be the reason a
/// serialization crate enters this workspace. What that costs is exactly the mistake made
/// above, so the anchor is the field name and the object boundary rather than a hopeful
/// substring.
fn State_Of(board: &str, item: &str) -> Option<String>
{
    let after_identifier = board.split(&format!("\"id\": \"{item}\"")).nth(1)?;
    // Stop at the next object, so a later item's state cannot be read as this one's.
    let within = after_identifier.split("\"id\":").next().unwrap_or_default();
    let value = within.split("\"state\":").nth(1)?;

    return value
        .split('"')
        .nth(1)
        .map(str::to_owned);
}

/// Every tool the surface declares is one the README names.
///
/// # The direction `KWB-61` added for verbs and `KWB-65` needed for tools
///
/// The test above catches a README naming a tool that does not exist. This catches a tool that
/// exists and nobody is told about — and the repository grew one the day this was written,
/// because `KWB-65` added `held_neighbours` and the only check ran README-to-surface.
///
/// `AGENTS.md` routes *what exists* to `README.md`. An agent reading that file to learn what it
/// can ask would not have found the tool built so that it could ask the question `D19-B` needed.
#[test]
fn Test_Every_Tool_The_Surface_Declares_Should_Be_One_The_Readme_Names()
{
    let readme = std::fs::read_to_string(Repository_Root().join("README.md"))
        .expect("README.md should be readable");
    let surface = std::fs::read_to_string(Repository_Root().join("crates/host/kwb-mcp/src/lib.rs"))
        .expect("the tool surface should be readable");

    let mut declared: Vec<String> = Vec::new();
    for fragment in surface.split("name: \"").skip(1)
    {
        if let Some(name) = fragment.split('"').next()
        {
            Remember(&mut declared, name);
        }
    }

    assert!(
        declared.len() >= 5,
        "only {declared:?} were found in the surface, so this guard covers almost nothing"
    );

    let undocumented: Vec<&String> = declared
        .iter()
        .filter(|tool| return !readme.contains(tool.as_str()))
        .collect();

    assert!(
        undocumented.is_empty(),
        "the surface declares these and the README never names them, so an agent sent to that \
         file for what it can ask cannot find them: {undocumented:?}"
    );
}

// ---- KWB-78: a record's frontmatter says what its body says ----

/// A record's declared version, and how many amendments its body carries.
fn Versions_And_Amendments(records: &BTreeMap<String, String>) -> Vec<(String, usize, usize)>
{
    let mut found = Vec::new();
    for (identifier, text) in records
    {
        let declared = text
            .lines()
            .find_map(|line| return line.trim().strip_prefix("version:"))
            .and_then(|value| return value.trim().parse::<usize>().ok());
        let Some(declared) = declared
        else
        {
            continue;
        };

        let amendments = text
            .lines()
            .filter(|line| return line.starts_with("## Amendment"))
            .count();
        found.push((identifier.clone(), declared, amendments));
    }

    return found;
}

/// A record's version is one plus the number of amendments it carries.
///
/// # Why that is the rule, and why it is written down rather than inferred
///
/// An amendment *is* what a new version of a record is here. `D-004` went to version 2 with its
/// *Two Lists Were Not Enough* amendment, `D-008` and `D-013` likewise — the convention was
/// already practice in three records out of six.
///
/// It was broken in the other three, and `KWB-78` measured that all three breaks were amendments
/// written in one session: `D-009` carried one amendment at version 1, `D-014` two at version 1,
/// `D-012` three at version 2. Nothing read the field and nothing checked it, which is why the
/// drift was invisible — a field that looks like a signal and carries none is worse than an
/// absent field, because a reader takes it for maintained.
///
/// The rule is stated here rather than in each record's frontmatter, because a convention
/// repeated fifteen times is fifteen places to change it. `AGENTS.md` carries a paragraph that
/// routes an author here before they write a record, which is the only place the rule is any
/// use — a guard tells you afterwards. That paragraph is a route and this is the authority, the
/// arrangement `AGENTS.md` uses for everything else it points at.
#[test]
fn Test_A_Records_Version_Should_Be_What_Its_Amendments_Make_It()
{
    let records = Records();
    let found = Versions_And_Amendments(&records);

    assert!(
        found.len() >= 10,
        "only {} records declared a version, so this guard covers almost nothing",
        found.len()
    );
    assert!(
        found.iter().any(|(_, _, amendments)| return *amendments > 0),
        "no record carries an amendment, so this guard would pass on a rule nothing exercises"
    );

    let disagreeing: Vec<String> = found
        .iter()
        .filter(|(_, declared, amendments)| return *declared != amendments.saturating_add(1))
        .map(|(identifier, declared, amendments)| {
            return format!(
                "{identifier} says version {declared} and carries {amendments} amendments"
            );
        })
        .collect();

    assert!(
        disagreeing.is_empty(),
        "these records' frontmatter disagrees with their own bodies about how many times they \
         have been revised: {disagreeing:#?}"
    );

    // The route is part of the rule. A guard nobody is sent to before writing only ever reports
    // the mistake after it is made, so `AGENTS.md` has to still be pointing here -- and at *this*
    // file, derived rather than spelled out, so renaming it breaks the pointer loudly.
    let contract = std::fs::read_to_string(Repository_Root().join("AGENTS.md"))
        .expect("AGENTS.md should be readable");
    let here = file!().replace('\\', "/");

    assert!(
        contract.contains(&here),
        "AGENTS.md does not name {here}, so the paragraph telling a record author what version \
         means either went away or is pointing at a file that has moved"
    );
    assert!(
        contract.contains("one plus the number of amendments"),
        "AGENTS.md no longer states the convention, so an author meets it for the first time as \
         a failing test rather than before writing the record"
    );
}

/// `status` is one value across every record, and that is a fact rather than an oversight.
///
/// # What the field is for
///
/// Every record reads `status: accepted`, because **no record here has been superseded
/// wholesale**. Parts of several have — `D-012`'s durability half, `D-014`'s deferral,
/// `D-009`'s reasoning — and each of those is an amendment inside a record that still stands.
/// That is deliberate: this repository leaves a superseded claim visible beside its correction
/// rather than replacing it, so the unit that goes out of date is a *paragraph*, not a record.
///
/// So the field would change when a record is replaced in full by another — which has not
/// happened and may not. This test exists to say that out loud, because a field with one value
/// and no explanation reads as a maintained signal to anyone who has not counted.
///
/// It asserts the field is *present and uniform* rather than pinning the word: a record that
/// genuinely is superseded should be able to say so without failing a test written before it.
#[test]
fn Test_Every_Record_Should_Declare_A_Status()
{
    let records = Records();

    let silent: Vec<&String> = records
        .iter()
        .filter(|(_, text)| {
            return !text.lines().any(|line| return line.trim().starts_with("status:"));
        })
        .map(|(identifier, _)| return identifier)
        .collect();

    assert!(
        silent.is_empty(),
        "these records declare no status, so a reader cannot tell whether they still stand: \
         {silent:?}"
    );
}

// ---- KWB-79: a reader the library owns is not declared a second time ----

/// Every reader `kwb_contract_tests` exports, by name.
fn Library_Readers() -> BTreeSet<String>
{
    let library = std::fs::read_to_string(Repository_Root().join("tests/contract/src/lib.rs"))
        .expect("tests/contract/src/lib.rs should be readable");

    let mut names = BTreeSet::new();
    for line in library.lines()
    {
        let Some(after) = line.trim_start().strip_prefix("pub fn ")
        else
        {
            continue;
        };
        let Some((name, _)) = after.split_once('(')
        else
        {
            continue;
        };
        names.insert(name.to_owned());
    }

    return names;
}

/// Every test file under `tests/contract/tests`, as `name -> text`.
fn Test_Files() -> BTreeMap<String, String>
{
    let directory = Repository_Root().join("tests/contract/tests");
    let entries = std::fs::read_dir(&directory).expect("tests/contract/tests should be readable");

    let mut files = BTreeMap::new();
    for entry in entries
    {
        let path = entry.expect("a readable directory entry").path();
        if !path.extension().is_some_and(|extension| return extension == "rs")
        {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|name| return name.to_str())
            .unwrap_or_default()
            .to_owned();
        files.insert(name, std::fs::read_to_string(&path).expect("a readable test file"));
    }

    return files;
}

/// No test file declares a reader the library already owns.
///
/// # Why a duplicate reader is a correctness problem and not untidiness
///
/// Each file under `tests/` is its own binary, so nothing stops a second copy compiling, and
/// nothing makes the two agree. `KWB-79` consolidated four `Repository_Root` bodies and two
/// `Quoted_After`s -- and the `Quoted_After` pair **were not the same function**. One searched
/// the whole text for the prefix, the other required a line to begin with it. They returned the
/// same answers on every manifest this repository has, so the divergence was invisible while two
/// files each read manifests believing they called one reader.
///
/// That is the shape this repository keeps finding: two things claim to be one fact and nothing
/// proves they agree. A guard that only removed today's copies would leave the arrangement that
/// produced them, so this one fails when a copy comes back.
///
/// It matches on the declaration rather than on any use, because importing a reader and calling
/// it is exactly what these files should do.
#[test]
fn Test_No_Test_File_Should_Declare_A_Reader_The_Library_Owns()
{
    let readers = Library_Readers();
    let files = Test_Files();

    assert!(
        readers.len() >= 4,
        "only {} readers were found in the library, so this guard compares against almost \
         nothing",
        readers.len()
    );
    assert!(
        files.len() >= 4,
        "only {} test files were read, so this guard covers almost nothing",
        files.len()
    );

    let mut duplicated: Vec<String> = Vec::new();
    for (name, text) in &files
    {
        for reader in &readers
        {
            let declaration = format!("fn {reader}(");
            if text.contains(&declaration)
            {
                duplicated.push(format!("{name} declares its own {reader}"));
            }
        }
    }

    assert!(
        duplicated.is_empty(),
        "these are second copies of readers `kwb_contract_tests` already owns -- import them \
         instead, because two bodies behind one name is where a divergence lives unobserved: \
         {duplicated:#?}"
    );
}

// ---- KWB-81: the band constrains something, and the quarantine holds ----

/// One crate, as its manifest declares it: its band and the crates here it depends on.
struct Crate
{
    band: String,
    depends_on: BTreeSet<String>,
    names_xvpe: bool,
}

/// Where a band sits in the order, from the label rather than from a second field.
///
/// The same derivation `projections.rs` uses for the table's row order, and for the same reason:
/// `1p` is the platform tier beside band 1, so comparing the strings would put `10` before `1p`
/// before `2`. Only the leading digits decide *depends on*, because `1p` and `1` are one tier —
/// a port and its adapters sit beside the crates they serve, not above them.
fn Tier_Of(band: &str) -> u32
{
    let digits: String = band.chars().take_while(char::is_ascii_digit).collect();

    return digits.parse().unwrap_or(u32::MAX);
}

/// Every crate in `crates/`, read from the manifests that declare it.
fn Crates() -> BTreeMap<String, Crate>
{
    let mut found = BTreeMap::new();
    Collect_Crates(&Repository_Root().join("crates"), &mut found);

    return found;
}

/// Walk for manifests, reading the band and the in-workspace dependencies out of each.
fn Collect_Crates(directory: &std::path::Path, found: &mut BTreeMap<String, Crate>)
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
            Collect_Crates(&path, found);
            continue;
        }
        if path.file_name().is_none_or(|name| return name != "Cargo.toml")
        {
            continue;
        }

        let text = std::fs::read_to_string(&path).expect("a readable manifest");
        let (Some(name), Some(band)) =
            (Quoted_After(&text, "name = "), Quoted_After(&text, "band = "))
        else
        {
            continue;
        };

        let mut depends_on = BTreeSet::new();
        let mut names_xvpe = false;
        for dependency in Declared_Dependencies(&text)
        {
            if dependency.starts_with("kwb-") && dependency != name
            {
                depends_on.insert(dependency.clone());
            }
            if Names_An_Xvpe_Crate(&dependency)
            {
                names_xvpe = true;
            }
        }

        found.insert(name, Crate { band, depends_on, names_xvpe });
    }
}

/// Every dependency a manifest declares, in either spelling TOML allows.
///
/// # Why both spellings, found the hard way
///
/// The first version of this read `name = ` lines only, and reported that `kwb-platform-xvpe`
/// depends on no XVPE crate — which would have made the quarantine guard below pass on every
/// crate in the workspace, for no work. It was the guard's own floor that caught it, because
/// `kwb-platform-xvpe` declares each adoption as `[dependencies.xvpe-clock]` with the git
/// reference beneath, which the inline form cannot express.
///
/// So a manifest states a dependency two ways and a reader of manifests must know both, or it
/// reports a clean result for a file it did not understand. `KWB-81`.
fn Declared_Dependencies(manifest: &str) -> Vec<String>
{
    let mut found = Vec::new();
    for line in manifest.lines()
    {
        let trimmed = line.trim();

        // `[dependencies.name]`, and the dev, build and target-specific tables that end the
        // same way. The name is the last segment, so the prefix does not need enumerating.
        if let Some(inner) = trimmed.strip_prefix('[').and_then(|rest| return rest.strip_suffix(']'))
        {
            if inner.contains("dependencies.")
            {
                if let Some(name) = inner.rsplit('.').next()
                {
                    found.push(name.to_owned());
                }
            }
            continue;
        }

        // `name = { … }` or `name = "1.0"`, which is every ordinary entry.
        if let Some((name, _)) = trimmed.split_once(" = ")
        {
            found.push(name.to_owned());
        }
    }

    return found;
}

/// Whether an identifier names an XVPE crate, rather than merely containing the letters.
///
/// `kwb-platform-xvpe` and `kwb_platform_xvpe` contain them and are not XVPE crates — they are
/// the quarantine. A grep that misses this reports every consumer of the quarantine as a breach,
/// which is the probe failing rather than the invariant, and is why this is a function with a
/// name instead of a pattern written at each call site.
fn Names_An_Xvpe_Crate(identifier: &str) -> bool
{
    return identifier.starts_with("xvpe-") || identifier.starts_with("xvpe_");
}

/// A crate depends only on crates in its own tier or below.
///
/// # Why the rule is measured and not invented
///
/// `AGENTS.md` routes *which band may depend on which* to `README.md`, and until `KWB-81` the
/// README did not say. The band had been machine-readable since `KWB-75` and constrained nothing
/// — a field that looks like a signal and carries none, which is `KWB-78`'s finding one tier
/// down, in the architecture rather than in a record's frontmatter.
///
/// So the rule is what the workspace already does rather than a position taken here. Every edge
/// lands in the same tier or below; the two same-tier edges are an adapter on its port
/// (`kwb-platform-std` on `kwb-platform`) and a reader on the seam it implements (`kwb-extract`
/// on `kwb-ingest`). An upward edge is a decision to record, and this test is what makes adding
/// one require that rather than a commit.
#[test]
fn Test_No_Crate_Should_Depend_On_A_Higher_Band()
{
    let crates = Crates();

    assert!(
        crates.len() >= 10,
        "only {} crates were read, so this guard covers almost nothing",
        crates.len()
    );
    assert!(
        crates.values().any(|one| return !one.depends_on.is_empty()),
        "no crate was found to depend on any other, so this guard would pass on a graph it never \
         read"
    );

    let mut upward: Vec<String> = Vec::new();
    for (name, one) in &crates
    {
        for dependency in &one.depends_on
        {
            let Some(target) = crates.get(dependency)
            else
            {
                continue;
            };
            if Tier_Of(&target.band) > Tier_Of(&one.band)
            {
                upward.push(format!(
                    "{name} [band {}] depends on {dependency} [band {}]",
                    one.band, target.band
                ));
            }
        }
    }

    assert!(
        upward.is_empty(),
        "these edges point up the bands, which inverts the order README.md states -- adding one \
         is a decision to record rather than a dependency to add: {upward:#?}"
    );
}

/// Only `kwb-platform-xvpe` names an XVPE crate.
///
/// # What rests on this
///
/// The bands table calls it *the one crate permitted to name XVPE*, and `D-007` decides adoption
/// happens by git reference and commit SHA into a crate that exists to quarantine it. Both were
/// prose until `KWB-81`. Measured then, the invariant held — and it held while nothing checked
/// it, which is the condition under which it quietly stops holding.
#[test]
fn Test_Only_The_Quarantine_Crate_Should_Name_Xvpe()
{
    let crates = Crates();

    assert!(
        crates.contains_key("kwb-platform-xvpe"),
        "the quarantine crate was not found, so this guard does not know what it is excepting"
    );
    assert!(
        crates
            .get("kwb-platform-xvpe")
            .is_some_and(|one| return one.names_xvpe),
        "the quarantine crate's manifest names no XVPE crate, so either adoption has been removed \
         or this guard has stopped recognising it -- and either way it would now pass on every \
         crate in the workspace"
    );

    let breaches: Vec<&String> = crates
        .iter()
        .filter(|(name, one)| return one.names_xvpe && *name != "kwb-platform-xvpe")
        .map(|(name, _)| return name)
        .collect();

    assert!(
        breaches.is_empty(),
        "these crates name an XVPE crate directly, so the dependency is no longer quarantined \
         where D-007 put it: {breaches:?}"
    );

    // The manifest is only half of it. A crate could reach an XVPE type through a re-export it
    // did not declare, and the rule is about naming XVPE, not about declaring it.
    let mut using: Vec<String> = Vec::new();
    let mut sources: Vec<(std::path::PathBuf, String)> = Vec::new();
    Rust_Sources(&Repository_Root().join("crates"), &mut sources);

    assert!(
        sources.len() >= 20,
        "only {} source files were read, so the source half of this guard covers almost nothing",
        sources.len()
    );

    for (path, text) in &sources
    {
        let display = path.display().to_string().replace('\\', "/");
        if display.contains("/kwb-platform-xvpe/")
        {
            continue;
        }
        for line in text.lines()
        {
            let Some(position) = line.find("xvpe_")
            else
            {
                continue;
            };
            // `kwb_platform_xvpe::` ends in the same letters. What distinguishes a direct use is
            // that `xvpe_` begins the identifier, so the character before it is not one that can
            // sit inside a Rust name.
            let preceding = line
                .get(..position)
                .and_then(|before| return before.chars().next_back());
            if preceding.is_some_and(|character| {
                return character.is_alphanumeric() || character == '_';
            })
            {
                continue;
            }
            using.push(format!("{display} uses {}", line.trim()));
        }
    }

    assert!(
        using.is_empty(),
        "these files reach an XVPE crate without going through the quarantine, so what the bands \
         table calls the one crate permitted to name XVPE is no longer the one: {using:#?}"
    );
}

/// Every Rust source file under a directory, as `path -> text`.
fn Rust_Sources(directory: &std::path::Path, into: &mut Vec<(std::path::PathBuf, String)>)
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
            Rust_Sources(&path, into);
            continue;
        }
        if path.extension().is_some_and(|extension| return extension == "rs")
        {
            let text = std::fs::read_to_string(&path).expect("a readable source file");
            into.push((path, text));
        }
    }
}

// ---- KWB-84: an observation's fields are not a record's fields ----

/// What an observation's `status` may say.
///
/// Two values, and both mean something: an observation is `open` until something acts on the
/// finding, and `closed` once something has. This is the reverse of a record's `status`, which
/// reads `accepted` on all fifteen and discriminates nothing — see the record guard above.
const OBSERVATION_STATUSES: [&str; 2] = ["open", "closed"];

/// Every observation's identifier, mapped to its text.
fn Observations() -> BTreeMap<String, String>
{
    let directory = Repository_Root().join("docs/observations");
    let entries = std::fs::read_dir(&directory).expect("docs/observations should be readable");

    let mut found = BTreeMap::new();
    for entry in entries
    {
        let path = entry.expect("a readable directory entry").path();
        if !path.extension().is_some_and(|extension| return extension == "md")
        {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|name| return name.to_str())
            .unwrap_or_default();
        // `OD-LEDGER-001-a-record-cannot-...` — the identifier is the first three segments.
        let parts: Vec<&str> = name.splitn(4, '-').collect();
        let Some(identifier) = parts.get(..3).map(|segments| return segments.join("-"))
        else
        {
            continue;
        };
        let text = std::fs::read_to_string(&path).expect("a readable observation");
        found.insert(identifier, text);
    }

    return found;
}

/// The value of a frontmatter field, from the field and not from text near it.
fn Field_Of(document: &str, field: &str) -> Option<String>
{
    return document
        .lines()
        .find_map(|line| return line.trim().strip_prefix(&format!("{field}:")))
        .map(|value| return value.trim().to_owned());
}

/// Every observation declares a version and a status, and the status is one that means something.
///
/// # Why this is not the record guard pointed at another directory
///
/// `AGENTS.md` routes readers to `docs/observations/` as one of three authorities, and until
/// `KWB-84` nothing here read it at all. The obvious fix — run the record checks over it — is
/// wrong, and measurably so.
///
/// `OD-LEDGER-001` is at version 2 and carries **no** `## Amendment` section. Under the record
/// rule that is a defect. It is not one: the bump was `KWB-43`, whose subject is *my own
/// observation overstated its finding, and now says what was measured*, and which rewrote the
/// text in place. That is what an observation's version counts, because a finding nobody has
/// acted on yet has nothing to keep visible beside its correction, where a record's superseded
/// claim stays precisely because somebody may have acted on it.
///
/// `status` goes the other way. A record's is inert — all fifteen read `accepted` — while an
/// observation's is the whole point of the document, because an observation is open until
/// something acts on it. So the field that carries no signal in one directory is load-bearing in
/// the other, and a guard that treated the two kinds alike would be wrong twice in opposite
/// directions.
#[test]
fn Test_Every_Observation_Should_Declare_A_Version_And_A_Meaningful_Status()
{
    let observations = Observations();

    assert!(
        observations.len() >= 2,
        "only {} observations were read, so this guard covers almost nothing",
        observations.len()
    );

    let mut wrong: Vec<String> = Vec::new();
    for (identifier, text) in &observations
    {
        match Field_Of(text, "version").and_then(|value| return value.parse::<usize>().ok())
        {
            Some(version) if version >= 1 =>
            {}
            _ =>
            {
                wrong.push(format!("{identifier} declares no usable version"));
            }
        }

        let Some(status) = Field_Of(text, "status")
        else
        {
            wrong.push(format!("{identifier} declares no status"));
            continue;
        };
        if !OBSERVATION_STATUSES.contains(&status.as_str())
        {
            wrong.push(format!(
                "{identifier} says status {status}, which is not one of {OBSERVATION_STATUSES:?}"
            ));
        }
    }

    assert!(
        wrong.is_empty(),
        "an observation's status says whether anything has acted on the finding, and its version \
         counts revisions made in place -- these carry neither usably: {wrong:#?}"
    );

    // The route is part of the rule, for the reason the record guard above gives: a convention an
    // author meets only as a failing test is one they have already broken.
    let contract = std::fs::read_to_string(Repository_Root().join("AGENTS.md"))
        .expect("AGENTS.md should be readable");

    assert!(
        contract.contains("counts revisions made **in place**"),
        "AGENTS.md no longer says what an observation's version counts, so an author carries the \
         record rule across -- and gets it wrong in the direction no guard catches, because the \
         record guard does not read this directory"
    );
}

/// The record's version rule does not reach observations, and that is deliberate.
///
/// # Why this test exists rather than a comment
///
/// The record guard reads `docs/records/` and so passes over `docs/observations/` — which looks
/// exactly the same as forgetting to include it. `OD-LEDGER-001` would fail that rule, being at
/// version 2 with no amendments, and it is correct as it stands.
///
/// So this asserts the distinguishing fact directly: an observation at a version above one, with
/// no amendment section, exists and is right. If observations were ever brought under the record
/// rule, this test fails and says why, instead of the repository quietly acquiring a false defect
/// report against a document that was always correct.
#[test]
fn Test_An_Observation_May_Be_Revised_Without_An_Amendment_Section()
{
    let observations = Observations();

    let revised_in_place: Vec<&String> = observations
        .iter()
        .filter(|(_, text)| {
            let version = Field_Of(text, "version")
                .and_then(|value| return value.parse::<usize>().ok())
                .unwrap_or_default();
            let amendments = text.lines().filter(|line| return line.starts_with("## Amendment")).count();

            return version > 1 && amendments == 0;
        })
        .map(|(identifier, _)| return identifier)
        .collect();

    assert!(
        !revised_in_place.is_empty(),
        "no observation is at a version above one without an amendment section, so this test no \
         longer demonstrates what it was written to demonstrate -- that an observation is revised \
         in place. Either the convention changed, in which case AGENTS.md is now wrong, or the \
         one example was edited away and a new one is needed"
    );
}
