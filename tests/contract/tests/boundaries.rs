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

/// `README.md`, which is where this repository says what exists and which band may depend on
/// which.
fn Readme() -> String
{
    return std::fs::read_to_string(Repository_Root().join("README.md"))
        .expect("README.md should be readable");
}

/// `AGENTS.md`, which is where this repository routes a reader to the authority for a question.
fn Operating_Contract() -> String
{
    return std::fs::read_to_string(Repository_Root().join("AGENTS.md"))
        .expect("AGENTS.md should be readable");
}

/// The composition root, which is where a verb the binary dispatches is declared.
fn Dispatch() -> String
{
    return std::fs::read_to_string(Repository_Root().join("crates/host/kwb-cli/src/main.rs"))
        .expect("the composition root should be readable");
}

/// The `kwb-mcp` tool surface, which is where a tool an agent can ask for is declared.
fn Tool_Surface() -> String
{
    return std::fs::read_to_string(Repository_Root().join("crates/host/kwb-mcp/src/lib.rs"))
        .expect("the tool surface should be readable");
}

/// The Bands table's header, which is where the table begins.
const BAND_TABLE_HEADER: &str = "| Band | Crate | Owns |";

/// The column a Bands table row puts its crate name in, and the one it puts the description in.
///
/// The header's own order is `Band`, `Crate`, `Owns`, and a row splits on the pipes with the
/// empty cell before the first one kept at 0 — so the crate name is column 2 and the description
/// column 3. Named because two readers index this table, and a column that moved would otherwise
/// move in only one of them.
const CRATE_COLUMN: usize = 2;
const OWNS_COLUMN: usize = 3;

/// The rows of `README.md`'s Bands table, each split into its cells and stripped of padding.
///
/// Two readers want this table — the one asking which crates it names and the one asking what
/// each of them says — and both need the same answer to where it begins and where it ends. That
/// is the part worth writing once: a table that grew a second header row, or a following
/// paragraph that began with a pipe, is a shape the two would otherwise disagree about.
fn Band_Table_Rows() -> Vec<Vec<String>>
{
    let readme = Readme();
    let Some((_, table)) = readme.split_once(BAND_TABLE_HEADER)
    else
    {
        return Vec::new();
    };

    let mut rows = Vec::new();
    for line in table.trim_start().lines()
    {
        if !line.starts_with('|')
        {
            break;
        }
        rows.push(line.split('|').map(|cell| return cell.trim().to_owned()).collect());
    }

    return rows;
}

/// The crate a Bands table row names, when its crate column is backticked as the table writes it.
fn Crate_Named_In(row: &[String]) -> Option<String>
{
    let cell = row.get(CRATE_COLUMN)?;

    return cell.strip_prefix('`')?.strip_suffix('`').map(str::to_owned);
}

/// Every crate name named in `README.md`'s Bands table.
fn Readme_Band_Table_Crate_Names() -> BTreeSet<String>
{
    return Band_Table_Rows()
        .iter()
        .filter_map(|row| return Crate_Named_In(row))
        .collect();
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
        if let Some((identifier, text)) = Record_At(&path)
        {
            records.insert(identifier, text);
        }
    }

    assert!(!records.is_empty(), "no records were scanned, so this test proves nothing");
    return records;
}

/// A record's identifier and text, when the path is a record file.
///
/// `D-004-two-lists-were-not-enough.md` — the identifier is the first two dash-separated segments
/// of the name and everything after them is prose. It is read off the name rather than out of the
/// frontmatter because the name is what a relation's `target` matches.
fn Record_At(path: &std::path::Path) -> Option<(String, String)>
{
    if !path.extension().is_some_and(|extension| return extension == "md")
    {
        return None;
    }
    let name = path.file_name().and_then(|name| return name.to_str())?;
    let mut parts = name.split('-');
    let (Some(prefix), Some(number)) = (parts.next(), parts.next())
    else
    {
        return None;
    };

    let text = std::fs::read_to_string(path).expect("a readable record");

    return Some((format!("{prefix}-{number}"), text));
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
/// `KWB-86` widened the relation checks to cover `docs/observations/`, because an observation
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
        if let Some((name, description)) = Manifest_Description(&path)
        {
            found.insert(name, description);
        }
    }
}

/// A manifest's crate name and description, when the file is one that declares both.
fn Manifest_Description(path: &std::path::Path) -> Option<(String, String)>
{
    if path.file_name().is_none_or(|name| return name != "Cargo.toml")
    {
        return None;
    }

    let text = std::fs::read_to_string(path).expect("a readable manifest");

    return Some((Quoted_After(&text, "name = ")?, Quoted_After(&text, "description = ")?));
}

/// Every crate row in `README.md`'s Bands table, as `crate -> what it owns`.
fn Readme_Band_Descriptions() -> BTreeMap<String, String>
{
    let mut rows = BTreeMap::new();
    for row in Band_Table_Rows()
    {
        let (Some(name), Some(owns)) = (Crate_Named_In(&row), row.get(OWNS_COLUMN).cloned())
        else
        {
            continue;
        };
        rows.insert(name, owns);
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

    let disagreements: Vec<String> = readme
        .iter()
        .filter_map(|(name, owns)| return Description_Disagreement(name, owns, manifests.get(name)))
        .collect();

    assert!(
        disagreements.is_empty(),
        "a crate describes itself differently in two places, so one of them is already wrong \
         and a reader cannot tell which: {}",
        disagreements.join("\n")
    );
}

/// How a crate's two descriptions differ, or `None` when they agree.
///
/// A crate the manifest side does not describe is not a disagreement: these two tables are not
/// required to name the same crates, and `Test_Every_Workspace_Member_Should_Appear_In_The_Readme_Table`
/// is the guard for a crate that is missing from the README rather than from its own manifest.
fn Description_Disagreement(name: &str, owns: &str, described: Option<&String>) -> Option<String>
{
    let description = described?;
    if description == owns
    {
        return None;
    }

    return Some(format!("{name}\n  README:   {owns}\n  manifest: {description}"));
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
    let contract = Operating_Contract();

    let authorities = Authorities();
    let unrouted: Vec<&String> = authorities
        .iter()
        .filter(|name| return !contract.contains(&format!("docs/{name}/")))
        .collect();

    assert!(
        authorities.len() >= MINIMUM_AUTHORITIES,
        "only {} authorities were scanned, so this test proves little",
        authorities.len()
    );
    assert!(
        unrouted.is_empty(),
        "these authorities exist and AGENTS.md routes nobody to them, so a session following \
         the operating contract cannot find them: {unrouted:?}"
    );
}

/// The fewest authorities `docs/` could hold and still be the three kinds the contract routes.
const MINIMUM_AUTHORITIES: usize = 3;

/// Every directory under `docs/`, by name.
fn Authorities() -> Vec<String>
{
    let entries = std::fs::read_dir(Repository_Root().join("docs"))
        .expect("docs/ should be readable");

    let mut names = Vec::new();
    for entry in entries
    {
        let path = entry.expect("a readable directory entry").path();
        if !path.is_dir()
        {
            continue;
        }
        if let Some(name) = path.file_name().and_then(|name| return name.to_str())
        {
            names.push(name.to_owned());
        }
    }

    return names;
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
    let dispatched = Verbs_Dispatched(&Dispatch());

    let missing: Vec<String> = Verbs_Shown(&Readme())
        .into_iter()
        .filter(|verb| return !Dispatch_Answers(verb, &dispatched))
        .collect();

    assert!(
        missing.is_empty(),
        "the README shows commands the binary does not dispatch, so a reader following it would \
         find out the hard way: {missing:?}"
    );
}

/// The fewest verbs the composition root could declare and still be dispatching anything.
const MINIMUM_VERBS: usize = 4;

/// The verbs the binary answers and the dispatch table deliberately does not declare.
///
/// They are how a reader finds the rest rather than capabilities to be listed, so a README that
/// mentions `kwb help` is not naming something undispatched.
const HELP_ALIASES: &[&str] = &["help", "--help", "-h"];

/// Whether the dispatch can answer a verb.
///
/// Asks the table rather than the source text. Until `KWB-74` the verbs *were* the source text —
/// one match arm each — and this looked for the arm; when `KWB-74` moved them into `VERBS` that
/// reading would have reported every verb as undispatched, which is a guard failing for the wrong
/// reason and worse than one passing for the wrong reason, because it teaches people to ignore it.
fn Dispatch_Answers(verb: &str, dispatched: &[String]) -> bool
{
    return HELP_ALIASES.contains(&verb)
        || dispatched.iter().any(|held| return held.as_str() == verb);
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
    let shown = Verbs_Shown(&Readme());
    let undocumented = Verbs_Outside(&Verbs_Dispatched(&Dispatch()), &shown);

    assert!(
        undocumented.is_empty(),
        "the binary answers these and the README never mentions them, so a reader sent to that \
         file for what exists cannot find them: {undocumented:?}"
    );
}

/// The verbs in `shown` that `known` does not name, in the order they appear.
///
/// The two directions of the verb check are this one comparison, and writing it once is what
/// keeps them from parting: they are meant to be the same question asked from either end.
fn Verbs_Outside(shown: &[String], known: &[String]) -> Vec<String>
{
    return shown
        .iter()
        .filter(|verb| return !known.iter().any(|held| return held.as_str() == verb.as_str()))
        .cloned()
        .collect();
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

    let verbs = Names_Declared_In(table);

    // A reader that parsed nothing passes on every repository, so the floor is here rather than
    // at each call site: an empty answer is a broken parse, not a clean workspace.
    assert!(
        verbs.len() >= MINIMUM_VERBS,
        "only {verbs:?} were found in the dispatch table, so this guard covers almost nothing"
    );
    return verbs;
}

/// The names a run of `name: "…"` declarations carries, in order and each kept once.
///
/// The dispatch's `VERBS` table and the tool surface's `TOOLS` table are the same shape, so this
/// is one reader rather than two: both declare a list of records whose first field is a name, and
/// a second parser for the second list would be a second place for the shape to drift.
fn Names_Declared_In(text: &str) -> Vec<String>
{
    let mut names: Vec<String> = Vec::new();
    for fragment in text.split("name: \"").skip(1)
    {
        if let Some(name) = fragment.split('"').next()
        {
            Remember_Once(&mut names, name);
        }
    }

    return names;
}

/// Every `kwb-mcp` tool the README names is one the surface declares.
#[test]
fn Test_Every_Tool_The_Readme_Names_Should_Be_One_The_Surface_Declares()
{
    let surface = Tool_Surface();

    let missing: Vec<String> = Tools_Shown(&Readme())
        .into_iter()
        .filter(|tool| return !surface.contains(&format!("name: \"{tool}\"")))
        .collect();

    assert!(
        missing.is_empty(),
        "the README names tools the surface does not declare: {missing:?}"
    );
}

/// Every `kwb-mcp` tool the README names in a command line, in the order they appear.
///
/// `kwb-mcp <store> <tool> ...` — the tool is the second word, when there is one.
fn Tools_Shown(readme: &str) -> Vec<String>
{
    let mut tools: Vec<String> = Vec::new();
    for line in readme.lines()
    {
        if let Some(tool) = line
            .trim()
            .strip_prefix("$ kwb-mcp ")
            .and_then(|rest| return rest.split_whitespace().nth(1))
        {
            Remember_Once(&mut tools, tool);
        }
    }

    return tools;
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
            Remember_Once(&mut verbs, rest.split_whitespace().next().unwrap_or_default());
        }
    }
    for verb in Verbs_Named_In_Prose(readme)
    {
        Remember_Once(&mut verbs, &verb);
    }

    assert!(
        verbs.len() >= MINIMUM_SHOWN_VERBS,
        "only {verbs:?} were found, so this guard covers almost nothing"
    );
    return verbs;
}

/// Every `kwb` verb a document names in a `kwb …` code span.
fn Verbs_Named_In_Prose(readme: &str) -> Vec<String>
{
    let mut verbs: Vec<String> = Vec::new();
    for fragment in readme.split("`kwb ").skip(1)
    {
        if let Some(inside) = fragment.split('`').next()
        {
            Remember_Once(&mut verbs, inside.split_whitespace().next().unwrap_or_default());
        }
    }

    return verbs;
}

/// The fewest verbs a README could show and still be showing a capability list.
const MINIMUM_SHOWN_VERBS: usize = 3;

/// Keep a word once.
///
/// Used for verbs and for tools, which is why it is named for neither: both are a list of
/// identifiers read out of a document, and a repeat in either one is noise in a failure message
/// rather than a second fact.
fn Remember_Once(seen: &mut Vec<String>, word: &str)
{
    if !word.is_empty() && !seen.iter().any(|held| return held == word)
    {
        seen.push(word.to_owned());
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
    let Some((_, listed)) = record.split_once(REFERENCE_SECTION)
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

/// The heading a document lists its incoming relations under.
const REFERENCE_SECTION: &str = "## Referenced By";

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
    let invented = Invented_References(&documents, &Declared_Relations(&documents));

    assert!(
        invented.is_empty(),
        "these Referenced By entries send a reader to a record that does not relate back, with \
         the authority of a list this repository calls checked: {invented:#?}"
    );
}

/// Every *Referenced By* entry that no document declares a relation to.
fn Invented_References(
    documents: &BTreeMap<String, String>,
    declared: &[(String, String)],
) -> Vec<String>
{
    let mut invented = Vec::new();
    for (identifier, text) in documents
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

    return invented;
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
            let Some(items) = Met_Condition_On(line)
            else
            {
                continue;
            };
            for item in items
            {
                named.push((identifier.clone(), item));
            }
        }
    }

    return named;
}

/// The marker a line states a met condition behind, and the only place it is written.
const CONDITION_MET_MARKER: &str = "**Condition met:**";

/// The items a line names as a met condition, when the line states one.
///
/// A line that merely mentions an item states nothing, which is why the marker is required: of
/// the four passages that name an item near a condition-like phrase, two are conditions and two
/// are not, so a guard reading all four would be wrong half the time — and a guard that is wrong
/// half the time is switched off.
fn Met_Condition_On(line: &str) -> Option<Vec<String>>
{
    let rest = line.trim().strip_prefix(CONDITION_MET_MARKER)?;

    return Some(
        rest.split(',')
            .map(|item| return item.trim().trim_matches('`').trim().to_owned())
            .filter(|item| return !item.is_empty())
            .collect(),
    );
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
    let named = Conditions_Named(&Records());

    assert!(
        !named.is_empty(),
        "no record states a condition in the checkable form, so this guard proves nothing"
    );

    let board = std::fs::read_to_string(Repository_Root().join("work/ledger.json"))
        .expect("the board should be readable");

    let unmet = Unmet_Conditions(&named, &board);
    assert!(
        unmet.is_empty(),
        "a record says a condition is satisfied and the board disagrees, so a subject is \
         declared workable while what it waits on is still open: {unmet:#?}"
    );
}

/// The met conditions the board disagrees with, each as a sentence naming both.
///
/// The state is read anchored on the field rather than on a substring near it. The first attempt
/// looked for `"done"` after the identifier and reported four defects that did not exist, because
/// the board writes `"Done"`. A guard whose parser is wrong does not report nothing — it reports
/// something false, with the authority of a test.
fn Unmet_Conditions(named: &[(String, String)], board: &str) -> Vec<String>
{
    let mut unmet = Vec::new();
    for (record, item) in named
    {
        let state = State_Of(board, item).unwrap_or_default();
        if !Has_Landed(&state)
        {
            unmet.push(format!("{record} calls {item} met and the board says {state:?}"));
        }
    }

    return unmet;
}

/// Whether a board state means the item has landed.
///
/// `Claimed` counts, and `KWB-64` is why. A record amended **by** the item that satisfies its
/// condition writes the marker while that item is still held — the predicate runs before
/// `finish`, so requiring `Done` here makes the only item that can honestly add a marker the one
/// item that cannot. That is a guard fighting the workflow rather than a defect.
///
/// It gives nothing up. An abandoned claim returns the item to `ready` and this fires then, and a
/// declined one fires immediately, so a marker whose item never lands is still caught — just at
/// the moment the board says so rather than before.
fn Has_Landed(state: &str) -> bool
{
    return state.eq_ignore_ascii_case("done") || state.eq_ignore_ascii_case("claimed");
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
    let readme = Readme();
    let declared = Tools_Declared(&Tool_Surface());

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

/// The fewest tools a registry could declare and still be a surface worth checking.
const MINIMUM_TOOLS: usize = 5;

/// Every tool the surface declares, by name.
///
/// The floor is here rather than at a call site, for the reason `Verbs_Dispatched` gives: every
/// tool the surface could lack is a README entry that passes a comparison against nothing.
fn Tools_Declared(surface: &str) -> Vec<String>
{
    let tools = Names_Declared_In(surface);

    assert!(
        tools.len() >= MINIMUM_TOOLS,
        "only {tools:?} were found in the surface, so this guard covers almost nothing"
    );
    return tools;
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
    let found = Versions_And_Amendments(&Records());

    assert!(
        found.len() >= MINIMUM_VERSIONED_RECORDS,
        "only {} records declared a version, so this guard covers almost nothing",
        found.len()
    );
    assert!(
        found.iter().any(|(_, _, amendments)| return *amendments > 0),
        "no record carries an amendment, so this guard would pass on a rule nothing exercises"
    );

    let disagreeing = Disagreeing_Versions(&found);
    assert!(
        disagreeing.is_empty(),
        "these records' frontmatter disagrees with their own bodies about how many times they \
         have been revised: {disagreeing:#?}"
    );
}

/// The fewest records that could declare a version and still exercise the rule.
const MINIMUM_VERSIONED_RECORDS: usize = 10;

/// Every record whose frontmatter version is not one plus its amendment count.
fn Disagreeing_Versions(found: &[(String, usize, usize)]) -> Vec<String>
{
    return found
        .iter()
        .filter(|(_, declared, amendments)| return *declared != amendments.saturating_add(1))
        .map(|(identifier, declared, amendments)| {
            return format!(
                "{identifier} says version {declared} and carries {amendments} amendments"
            );
        })
        .collect();
}

/// The version rule is one an author should meet before writing, which means the route holds.
///
/// The route is part of the rule. A guard nobody is sent to before writing only ever reports the
/// mistake after it is made, so `AGENTS.md` has to still be pointing here — and at *this* file,
/// derived rather than spelled out, so renaming it breaks the pointer loudly.
#[test]
fn Test_The_Operating_Contract_Should_Still_Route_Record_Authors_Here()
{
    let contract = Operating_Contract();
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
        readers.len() >= MINIMUM_READERS,
        "only {} readers were found in the library, so this guard compares against almost \
         nothing",
        readers.len()
    );
    assert!(
        files.len() >= MINIMUM_TEST_FILES,
        "only {} test files were read, so this guard covers almost nothing",
        files.len()
    );

    let duplicated = Duplicated_Readers(&files, &readers);
    assert!(
        duplicated.is_empty(),
        "these are second copies of readers `kwb_contract_tests` already owns -- import them \
         instead, because two bodies behind one name is where a divergence lives unobserved: \
         {duplicated:#?}"
    );
}

/// The fewest readers the library could export and still be the shared one.
const MINIMUM_READERS: usize = 4;

/// The fewest test files those readers could be spread across and still show the walk is walking.
///
/// Separate from the reader floor because the two failures differ: finding too few readers is a
/// library that stopped being shared, and finding them all in one file is a walk that never
/// descended.
const MINIMUM_TEST_FILES: usize = 4;

/// Every test file that declares a reader the library already owns, naming both.
fn Duplicated_Readers(
    files: &BTreeMap<String, String>,
    readers: &BTreeSet<String>,
) -> Vec<String>
{
    let mut duplicated = Vec::new();
    for (name, text) in files
    {
        for reader in readers
        {
            if text.contains(&format!("fn {reader}("))
            {
                duplicated.push(format!("{name} declares its own {reader}"));
            }
        }
    }

    return duplicated;
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
        if let Some((name, one)) = Declared_Crate(&path)
        {
            found.insert(name, one);
        }
    }
}

/// The crate a manifest declares, when the file is one that declares both a name and a band.
fn Declared_Crate(path: &std::path::Path) -> Option<(String, Crate)>
{
    if path.file_name().is_none_or(|name| return name != "Cargo.toml")
    {
        return None;
    }

    let text = std::fs::read_to_string(path).expect("a readable manifest");
    let (Some(name), Some(band)) =
        (Quoted_After(&text, "name = "), Quoted_After(&text, "band = "))
    else
    {
        return None;
    };

    let (depends_on, names_xvpe) = Depends_On(&text, &name);

    return Some((name, Crate { band, depends_on, names_xvpe }));
}

/// The workspace crates a manifest depends on, and whether it names an XVPE crate directly.
///
/// Two answers from one walk because the walk is the same one: a manifest states each dependency
/// once, and both questions are about that list.
fn Depends_On(manifest: &str, name: &str) -> (BTreeSet<String>, bool)
{
    let mut depends_on = BTreeSet::new();
    let mut names_xvpe = false;
    for dependency in Declared_Dependencies(manifest)
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

    return (depends_on, names_xvpe);
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
        let table = trimmed.strip_prefix('[').and_then(|rest| return rest.strip_suffix(']'));
        if let Some(inner) = table
        {
            if let Some(name) = Table_Dependency(inner)
            {
                found.push(name);
            }
            continue;
        }
        if let Some((name, _)) = trimmed.split_once(" = ")
        {
            found.push(name.to_owned());
        }
    }

    return found;
}

/// The crate name a bracketed table declares, when it is a dependencies table.
///
/// `[dependencies.name]`, and the dev, build and target-specific tables that end the same way. The
/// name is the last segment, so the prefix does not need enumerating — which is why a table this
/// does not recognise answers `None` rather than a wrong name.
fn Table_Dependency(table: &str) -> Option<String>
{
    if !table.contains("dependencies.")
    {
        return None;
    }

    return table.rsplit('.').next().map(str::to_owned);
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
/// The fewest crates a bands guard could read and still be reading the workspace.
const MINIMUM_CRATES: usize = 10;

#[test]
fn Test_No_Crate_Should_Depend_On_A_Higher_Band()
{
    let crates = Crates();

    assert!(
        crates.len() >= MINIMUM_CRATES,
        "only {} crates were read, so this guard covers almost nothing",
        crates.len()
    );
    assert!(
        crates.values().any(|one| return !one.depends_on.is_empty()),
        "no crate was found to depend on any other, so this guard would pass on a graph it never \
         read"
    );

    let upward = Upward_Edges(&crates);
    assert!(
        upward.is_empty(),
        "these edges point up the bands, which inverts the order README.md states -- adding one \
         is a decision to record rather than a dependency to add: {upward:#?}"
    );
}

/// The dependency edges that point at a crate in a higher band, as `source -> target`.
fn Upward_Edges(crates: &BTreeMap<String, Crate>) -> Vec<String>
{
    let mut upward = Vec::new();
    for (name, one) in crates
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

    return upward;
}

/// Only `kwb-platform-xvpe` names an XVPE crate.
///
/// # What rests on this
///
/// The bands table calls it *the one crate permitted to name XVPE*, and `D-007` decides adoption
/// happens by git reference and commit SHA into a crate that exists to quarantine it. Both were
/// prose until `KWB-81`. Measured then, the invariant held — and it held while nothing checked
/// it, which is the condition under which it quietly stops holding.
/// The crate permitted to name XVPE, which the bands table and `D-007` both name.
const QUARANTINE_CRATE: &str = "kwb-platform-xvpe";

/// The fewest source files a quarantine guard could read and still be reading the workspace.
const MINIMUM_SOURCES: usize = 20;

#[test]
fn Test_Only_The_Quarantine_Crate_Should_Name_Xvpe()
{
    let breaches = Manifest_Breaches(&Crates());
    assert!(
        breaches.is_empty(),
        "these crates name an XVPE crate directly, so the dependency is no longer quarantined \
         where D-007 put it: {breaches:?}"
    );

    // The manifest is only half of it. A crate could reach an XVPE type through a re-export it
    // did not declare, and the rule is about naming XVPE, not about declaring it.
    let using = Xvpe_Reachers(&Crate_Sources());
    assert!(
        using.is_empty(),
        "these files reach an XVPE crate without going through the quarantine, so what the bands \
         table calls the one crate permitted to name XVPE is no longer the one: {using:#?}"
    );
}

/// The crates whose manifests name an XVPE crate, other than the quarantine itself.
fn Manifest_Breaches(crates: &BTreeMap<String, Crate>) -> Vec<String>
{
    Assert_The_Quarantine_Is_Real(crates);

    return crates
        .iter()
        .filter(|(name, one)| return one.names_xvpe && name.as_str() != QUARANTINE_CRATE)
        .map(|(name, _)| return name.clone())
        .collect();
}

/// The quarantine crate exists and names an XVPE crate, or this guard excepts nothing.
///
/// The floor lives here rather than at the call site: the guard's whole subject is one exception,
/// so a run that cannot find the crate it excepts would pass on every crate in the workspace.
fn Assert_The_Quarantine_Is_Real(crates: &BTreeMap<String, Crate>)
{
    assert!(
        crates.contains_key(QUARANTINE_CRATE),
        "the quarantine crate was not found, so this guard does not know what it is excepting"
    );
    assert!(
        crates
            .get(QUARANTINE_CRATE)
            .is_some_and(|one| return one.names_xvpe),
        "the quarantine crate's manifest names no XVPE crate, so either adoption has been removed \
         or this guard has stopped recognising it -- and either way it would now pass on every \
         crate in the workspace"
    );
}

/// Every Rust source file under `crates/`, with the floor that makes a clean scan a scan.
fn Crate_Sources() -> Vec<(std::path::PathBuf, String)>
{
    let mut sources = Vec::new();
    Rust_Sources(&Repository_Root().join("crates"), &mut sources);

    assert!(
        sources.len() >= MINIMUM_SOURCES,
        "only {} source files were read, so the source half of this guard covers almost nothing",
        sources.len()
    );
    return sources;
}

/// The files outside the quarantine that reach an XVPE crate by name.
fn Xvpe_Reachers(sources: &[(std::path::PathBuf, String)]) -> Vec<String>
{
    let quarantine = format!("/{QUARANTINE_CRATE}/");
    let mut using = Vec::new();
    for (path, text) in sources
    {
        let display = path.display().to_string().replace('\\', "/");
        if display.contains(&quarantine)
        {
            continue;
        }
        for line in text.lines()
        {
            if Names_Xvpe_Directly(line)
            {
                using.push(format!("{display} uses {}", line.trim()));
            }
        }
    }

    return using;
}

/// Whether a line uses an XVPE crate by name, rather than merely containing the letters.
///
/// `kwb_platform_xvpe::` ends in the same letters. What distinguishes a direct use is that `xvpe_`
/// begins the identifier, so the character before it is not one that can sit inside a Rust name.
fn Names_Xvpe_Directly(line: &str) -> bool
{
    let Some(position) = line.find("xvpe_")
    else
    {
        return false;
    };
    let preceding = line.get(..position).and_then(|before| return before.chars().next_back());

    return !preceding.is_some_and(|character| {
        return character.is_alphanumeric() || character == '_';
    });
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

/// How many `-`-separated segments an observation's file name carries.
const OBSERVATION_NAME_PARTS: usize = 4;

/// How many of those segments are the identifier itself.
const OBSERVATION_IDENTIFIER_PARTS: usize = 3;

/// Every observation's identifier, mapped to its text.
fn Observations() -> BTreeMap<String, String>
{
    let directory = Repository_Root().join("docs/observations");
    let entries = std::fs::read_dir(&directory).expect("docs/observations should be readable");

    let mut found = BTreeMap::new();
    for entry in entries
    {
        let path = entry.expect("a readable directory entry").path();
        if let Some((identifier, text)) = Observation_At(&path)
        {
            found.insert(identifier, text);
        }
    }

    return found;
}

/// An observation's identifier and text, when the path is an observation file.
///
/// `OD-LEDGER-001-a-record-cannot-...` — the identifier is the first three segments.
fn Observation_At(path: &std::path::Path) -> Option<(String, String)>
{
    if !path.extension().is_some_and(|extension| return extension == "md")
    {
        return None;
    }
    let name = path.file_name().and_then(|name| return name.to_str())?;
    let parts: Vec<&str> = name.splitn(OBSERVATION_NAME_PARTS, '-').collect();
    let identifier = parts.get(..OBSERVATION_IDENTIFIER_PARTS)?.join("-");

    let text = std::fs::read_to_string(path).expect("a readable observation");
    return Some((identifier, text));
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
        observations.len() >= MINIMUM_OBSERVATIONS,
        "only {} observations were read, so this guard covers almost nothing",
        observations.len()
    );

    let wrong = Unusable_Observations(&observations);
    assert!(
        wrong.is_empty(),
        "an observation's status says whether anything has acted on the finding, and its version \
         counts revisions made in place -- these carry neither usably: {wrong:#?}"
    );

    // The route is part of the rule, for the reason the record guard above gives: a convention an
    // author meets only as a failing test is one they have already broken.
    Assert_The_Contract_Distinguishes_The_Two_Versions();
}

/// The operating contract says an observation's version is revised in place, and says so in terms.
fn Assert_The_Contract_Distinguishes_The_Two_Versions()
{
    let contract = Operating_Contract();

    assert!(
        contract.contains("counts revisions made **in place**"),
        "AGENTS.md no longer says what an observation's version counts, so an author carries the \
         record rule across -- and gets it wrong in the direction no guard catches, because the \
         record guard does not read this directory"
    );
}

/// The fewest observations a field guard could read and still be reading the directory.
const MINIMUM_OBSERVATIONS: usize = 2;

/// The observations whose frontmatter carries no usable version or no meaningful status.
fn Unusable_Observations(observations: &BTreeMap<String, String>) -> Vec<String>
{
    let mut wrong = Vec::new();
    for (identifier, text) in observations
    {
        if !Declares_A_Version(text)
        {
            wrong.push(format!("{identifier} declares no usable version"));
        }
        if let Some(note) = Status_Complaint(identifier, text)
        {
            wrong.push(note);
        }
    }

    return wrong;
}

/// Whether a document's `version` field holds a number of at least one.
fn Declares_A_Version(text: &str) -> bool
{
    return Field_Of(text, "version")
        .and_then(|value| return value.parse::<usize>().ok())
        .is_some_and(|version| return version >= 1);
}

/// Why an observation's `status` is unusable, when it is.
fn Status_Complaint(identifier: &str, text: &str) -> Option<String>
{
    let Some(status) = Field_Of(text, "status")
    else
    {
        return Some(format!("{identifier} declares no status"));
    };
    if OBSERVATION_STATUSES.contains(&status.as_str())
    {
        return None;
    }

    return Some(format!(
        "{identifier} says status {status}, which is not one of {OBSERVATION_STATUSES:?}"
    ));
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

// ---- KWB-87: one note about the relation check, carried by every record that needs it ----

/// What a record's *Referenced By* note says, and the only place it is written.
///
/// # Why this is a constant and not eleven paragraphs
///
/// It was eleven paragraphs. Measured for `KWB-87`: the identical text was copied verbatim into
/// every record that has a *Referenced By* section — eleven of the fifteen; the other four have
/// no section, and the correlation is exact, which the test below asserts rather than assumes.
///
/// Nothing made them agree. They did agree, which is why it went unnoticed for as long as it
/// did, and it is the truth question rather than the agreement question that bit: `KWB-86`
/// widened the relation guards to read observations, and every copy went on saying *no record*
/// where an observation now counts too. Eleven copies of a sentence about a mechanism, and the
/// mechanism had changed underneath all of them.
///
/// So the note is derived from here, the way `README.md`'s tool and bands tables are derived
/// from the registry and the manifests. The copies still exist — a record should carry its own
/// note where a reader meets it, not a pointer — but agreement is no longer somebody's job.
const REFERENCED_BY_NOTE: &str = "*Written by hand, and checked by `tests/contract` in both \
     directions: a declared relation with no entry here fails, and an entry here that nothing \
     declares a relation to fails too. Either end may be a record or an observation, since \
     `KWB-86`. A relation is declared in the frontmatter of the document that makes it; this is \
     the other end, so that a reader of this record can reach the ones that answer, amend or \
     build on it. Before `KWB-38`, 24 of 27 relations were reachable from one side only — which \
     is how three records came to assert things this repository had stopped doing.*";

/// The same text with its line breaks collapsed, so wrapping is not part of the contract.
///
/// A guard that compared line breaks would fail the day somebody reflowed a paragraph, which is
/// a guard failing for the wrong reason — worse than one passing for the wrong reason, because
/// it spends a reader's attention on nothing.
fn Collapsed(text: &str) -> String
{
    return text.split_whitespace().collect::<Vec<&str>>().join(" ");
}

/// The italic note a record carries under its *Referenced By* heading, when it carries one.
fn Referenced_By_Note(record: &str) -> Option<String>
{
    let after = record.split(REFERENCE_SECTION).nth(1)?;
    let (note, _) = after.trim_start().split_once("*\n")?;

    return Some(format!("{note}*"));
}

/// The fewest records carrying a *Referenced By* section before the guard is reading the corpus.
const MINIMUM_NOTED_RECORDS: usize = 10;

/// Every record with a *Referenced By* section carries the note, and it is the note.
#[test]
fn Test_Every_Referenced_By_Section_Should_Carry_The_One_Note()
{
    let records = Records();
    let carrying = With_A_Reference_Section(&records);
    assert!(
        carrying.len() >= MINIMUM_NOTED_RECORDS,
        "only {} records have a Referenced By section, so this guard covers almost nothing",
        carrying.len()
    );

    let wrong = Notes_That_Differ(&records, &carrying);
    assert!(
        wrong.is_empty(),
        "these records describe the relation check differently from every other record -- the \
         note is derived from REFERENCED_BY_NOTE and a record may not carry its own version of \
         it: {wrong:#?}"
    );
}

/// The records that carry a *Referenced By* section.
fn With_A_Reference_Section(records: &BTreeMap<String, String>) -> Vec<String>
{
    return records
        .iter()
        .filter(|(_, text)| return text.contains(REFERENCE_SECTION))
        .map(|(identifier, _)| return identifier.clone())
        .collect();
}

/// The carried notes that are not the one note, as `identifier: note`.
fn Notes_That_Differ(records: &BTreeMap<String, String>, carrying: &[String]) -> Vec<String>
{
    let expected = Collapsed(REFERENCED_BY_NOTE);
    let mut wrong = Vec::new();
    for identifier in carrying
    {
        let text = records.get(identifier).expect("a record just enumerated");
        let Some(note) = Referenced_By_Note(text)
        else
        {
            wrong.push(format!("{identifier} has a Referenced By section and no note"));
            continue;
        };
        if Collapsed(&note) != expected
        {
            wrong.push(format!("{identifier}'s note is not the note: {note}"));
        }
    }

    return wrong;
}

/// A record with no incoming relation has no *Referenced By* section, and that is the whole rule.
///
/// # What this asserts that the test above does not
///
/// The test above checks the records that have a section. This checks the four that do not —
/// `D-009`, `D-011`, `D-013` and `D-015` when `KWB-87` measured it — and says the reason is that
/// nothing declares a relation to them, rather than that somebody forgot the section.
///
/// The distinction matters because the two look identical from outside. If a record acquires an
/// incoming relation and nobody adds the section, the relation guard already fails; this fails
/// earlier and more precisely, at the record that is now missing a section it needs.
#[test]
fn Test_A_Record_Has_A_Referenced_By_Section_Exactly_When_Something_Points_At_It()
{
    let documents = Relating_Documents();
    let records = Records();
    let declared = Declared_Relations(&documents);

    assert!(
        declared.len() >= MINIMUM_RELATIONS,
        "only {} relations were found, so this guard compares against almost nothing",
        declared.len()
    );

    let wrong = Misplaced_Sections(&records, &declared);
    assert!(
        wrong.is_empty(),
        "a Referenced By section means something points here, and its absence means nothing \
         does -- these say otherwise: {wrong:#?}"
    );
}

/// The fewest declared relations before the guard is comparing against something.
const MINIMUM_RELATIONS: usize = 10;

/// The records whose *Referenced By* section disagrees with whether anything points at them.
fn Misplaced_Sections(
    records: &BTreeMap<String, String>,
    declared: &[(String, String)],
) -> Vec<String>
{
    let mut wrong = Vec::new();
    for (identifier, text) in records
    {
        let pointed_at = declared.iter().any(|(_, target)| return target == identifier);
        let has_section = text.contains(REFERENCE_SECTION);

        if pointed_at && !has_section
        {
            wrong.push(format!("{identifier} is related to and has no Referenced By section"));
        }
        if !pointed_at && has_section
        {
            wrong.push(format!(
                "{identifier} has a Referenced By section and nothing declares a relation to it"
            ));
        }
    }

    return wrong;
}

// ---- KWB-96: the XVPE edge adopts one commit, not several ----

/// The manifest carrying this workspace's only XVPE dependency edge.
const XVPE_MANIFEST: &str = "crates/platform/kwb-platform-xvpe/Cargo.toml";

/// Every XVPE dependency the quarantine manifest pins, as `(dependency, rev)`.
///
/// Read from the manifest rather than from `Cargo.lock`, because the lock records what was
/// resolved once and this guard is about what the manifest asks for every time.
fn Xvpe_Pinned_Revisions() -> Vec<(String, String)>
{
    let manifest = std::fs::read_to_string(Repository_Root().join(XVPE_MANIFEST))
        .expect("the XVPE quarantine manifest should be readable");

    return Pinned_From(&manifest);
}

/// Every `[dependencies.x]` table's pinned revision, in the order the manifest declares them.
fn Pinned_From(manifest: &str) -> Vec<(String, String)>
{
    let mut pinned = Vec::new();
    let mut dependency = String::new();
    for line in manifest.lines()
    {
        if let Some(name) = Dependency_Table_On(line)
        {
            name.clone_into(&mut dependency);
            continue;
        }
        if let Some(rev) = Pinned_Revision_On(line)
        {
            if !dependency.is_empty()
            {
                pinned.push((dependency.clone(), rev));
            }
        }
    }

    return pinned;
}

/// The dependency a `[dependencies.x]` line opens, when the line opens one.
fn Dependency_Table_On(line: &str) -> Option<&str>
{
    return line
        .trim()
        .strip_prefix("[dependencies.")
        .and_then(|rest| return rest.strip_suffix(']'));
}

/// The revision a `rev = "…"` line pins, when the line pins one.
fn Pinned_Revision_On(line: &str) -> Option<String>
{
    let (_, value) = line.trim().strip_prefix("rev")?.split_once('=')?;

    return Some(value.trim().trim_matches('"').to_owned());
}

/// The XVPE edge names one commit, so a partial bump fails instead of adopting two.
///
/// # Why this exists
///
/// The manifest states the rule beside the pin: bumping the `rev` is a decision rather than
/// maintenance. `KWB-72` bumped it inside an item about something else and nothing recorded what
/// the move changed. A bump done that way is done by hand, once per dependency, and the failure
/// it invites is the one nobody would notice: three move and one is left behind, so the
/// workspace compiles two states of another repository at once and reports nothing wrong.
///
/// # What this deliberately does not check
///
/// *Which* commit is pinned. That is a decision, `D-007` is where it is recorded, and a test
/// demanding a particular SHA would fail on every deliberate bump -- which is how a guard gets
/// switched off.
/// The fewest pins the manifest must carry before one commit can be said to be the rule.
const MINIMUM_XVPE_PINS: usize = 4;

#[test]
fn Test_Every_Xvpe_Dependency_Should_Pin_The_Same_Commit()
{
    let pinned = Xvpe_Pinned_Revisions();

    assert!(
        pinned.len() >= MINIMUM_XVPE_PINS,
        "only {} XVPE pins were read from {XVPE_MANIFEST}, so this guard is comparing almost \
         nothing and the manifest's shape has moved under it: {pinned:#?}",
        pinned.len()
    );

    let commits: BTreeSet<&str> = pinned.iter().map(|(_, rev)| return rev.as_str()).collect();
    assert!(
        commits.len() == 1,
        "the XVPE edge pins {} different commits, so this workspace adopts more than one state \
         of another repository at once -- which is what D-007 pins a commit to prevent: \
         {pinned:#?}",
        commits.len()
    );
}

// ---- KWB-99: the README's admit transcript shows what the binary prints ----

/// The file the admit report is printed from.
const ADMIT_PRINTER: &str = "crates/host/kwb-cli/src/main.rs";

/// The label a `println!("name   {}", ..)` line writes, if it writes one.
///
/// Requires at least one space between the label and the placeholder, which is what separates a
/// report field from an ordinary formatted line.
fn Printed_Label(line: &str) -> Option<String>
{
    let rest = line.trim().strip_prefix("println!(\"")?;
    let (head, _) = rest.split_once("{}")?;
    let label = head.trim_end();
    if label.is_empty() || label == head
    {
        return None;
    }
    if !label.chars().all(|character| return character.is_ascii_lowercase())
    {
        return None;
    }

    return Some(label.to_owned());
}

/// The admit report's fields, in the order `kwb-cli` prints them.
///
/// Read from the source rather than listed here, so this guard cannot drift from the binary the
/// way the README did. The run is anchored on `source` and walks forward while the lines keep
/// printing a label: that is how the report is written, one `println!` per field, consecutive and
/// all unconditional.
/// The field the admit report opens with, which is where the run of fields starts.
const ADMIT_FIRST_LABEL: &str = "source";

fn Admit_Report_Labels() -> Vec<String>
{
    let printer = std::fs::read_to_string(Repository_Root().join(ADMIT_PRINTER))
        .expect("the admit report's printer should be readable");

    let Some(start) = printer
        .lines()
        .position(|line| return Printed_Label(line).as_deref() == Some(ADMIT_FIRST_LABEL))
    else
    {
        return Vec::new();
    };

    return Labels_From(printer.lines().skip(start));
}

/// The consecutive labels a run of `println!` lines writes, stopping at the first other line.
fn Labels_From<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<String>
{
    let mut labels = Vec::new();
    for line in lines
    {
        let Some(label) = Printed_Label(line)
        else
        {
            break;
        };
        labels.push(label);
    }

    return labels;
}

/// The command a `kwb admit` transcript in `README.md` is introduced by.
const ADMIT_COMMAND: &str = "$ kwb admit";

/// The field labels each `kwb admit` transcript in `README.md` shows, one entry per block.
fn Readme_Admit_Block_Labels() -> Vec<Vec<String>>
{
    return Transcript_Labels(&Readme(), ADMIT_COMMAND);
}

/// The field labels of every transcript a document opens with `command`, one entry per block.
fn Transcript_Labels(document: &str, command: &str) -> Vec<Vec<String>>
{
    let mut blocks = Vec::new();
    let mut collecting: Option<Vec<String>> = None;
    for line in document.lines()
    {
        if line.starts_with(command)
        {
            Push_Block(&mut blocks, collecting.replace(Vec::new()));
            continue;
        }
        let Some(block) = collecting.as_mut()
        else
        {
            continue;
        };
        if !Note_Label(block, line)
        {
            Push_Block(&mut blocks, collecting.take());
        }
    }
    Push_Block(&mut blocks, collecting.take());

    return blocks;
}

/// Collect a transcript that has just ended, if there is one to collect.
fn Push_Block(blocks: &mut Vec<Vec<String>>, collecting: Option<Vec<String>>)
{
    if let Some(block) = collecting
    {
        blocks.push(block);
    }
}

/// Add a transcript line's field label, or report that the line ends the transcript.
fn Note_Label(block: &mut Vec<String>, line: &str) -> bool
{
    let Some(label) = Field_Label_On(line)
    else
    {
        return false;
    };
    block.push(label);

    return true;
}

/// A transcript line's field label, or `None` when the line ends the transcript.
fn Field_Label_On(line: &str) -> Option<String>
{
    if line.trim().is_empty() || line.starts_with("$ ") || line.starts_with("```")
    {
        return None;
    }

    return line.split_whitespace().next().map(str::to_owned);
}

/// Every `kwb admit` transcript in the README shows every field the binary prints, in order.
///
/// # Why this exists
///
/// The section exists so a reader can run the thing, and until `KWB-99` nobody had. Running it
/// found three of four blocks exact — `kwb history` matched count for count — and the second
/// `admit` block showing four of the eight lines the binary prints, with no ellipsis. A reader
/// who runs that command sees four lines the document does not account for, and cannot tell an
/// abridgement from a change in behaviour.
///
/// # Why the labels come from the source
///
/// A test listing the eight fields itself would be a third place the report is written down, and
/// the failure here was already two places disagreeing. Reading the printer means a field added
/// to the binary fails this until the README follows.
///
/// # What it does not check
///
/// The values, and in particular the content addresses. Those are hashes of files the README does
/// not ship, so a reader cannot reproduce them and this guard must not imply otherwise.
/// The fewest report fields the guard could read and still be comparing against the report.
const MINIMUM_REPORT_FIELDS: usize = 8;

/// The fewest `kwb admit` transcripts the README could show and still be showing the section.
const MINIMUM_ADMIT_BLOCKS: usize = 2;

#[test]
fn Test_Every_Readme_Admit_Transcript_Should_Show_Every_Field_The_Binary_Prints()
{
    let labels = Admit_Report_Labels();
    assert!(
        labels.len() >= MINIMUM_REPORT_FIELDS,
        "only {} report fields were read from {ADMIT_PRINTER}, so this guard is comparing against \
         almost nothing and the printer has moved under it: {labels:?}",
        labels.len()
    );

    let blocks = Readme_Admit_Block_Labels();
    assert!(
        blocks.len() >= MINIMUM_ADMIT_BLOCKS,
        "only {} `kwb admit` transcripts were found in README.md, so this guard read almost \
         nothing of the section it exists to hold",
        blocks.len()
    );

    let wrong = Blocks_Not_Showing(&blocks, &labels);
    assert!(
        wrong.is_empty(),
        "a README transcript is what a reader compares their own output against, and these do not \
         show what the binary prints: {wrong:#?}"
    );
}

/// The transcripts that do not show every field, as `block N shows …, the binary prints …`.
fn Blocks_Not_Showing(blocks: &[Vec<String>], labels: &[String]) -> Vec<String>
{
    return blocks
        .iter()
        .enumerate()
        .filter(|(_, block)| return *block != labels)
        .map(|(index, block)| {
            return format!(
                "block {} shows {block:?}, the binary prints {labels:?}",
                index.saturating_add(1)
            );
        })
        .collect();
}
