//! The properties versioned concept state exists to have, exercised from outside the crate.
//!
//! From outside deliberately: a read is only a separate world if it is separate to a caller.

use kwb_domain::{Concept, KnowledgeGraph, Standing, Versioned};

fn Asserted(name: &str) -> Versioned<Concept>
{
    return Versioned::Asserted(Concept::Named(name));
}

/// Assert that one concept is gone from the current read and still present in the every-version
/// one -- the pair `D-012` is built on.
///
/// The two are asserted together because either alone passes for the wrong reason: a concept that
/// was simply deleted satisfies the first, and a read that never filtered satisfies the second.
/// `D17` is the second, and it is the one that says nothing is destroyed without evidence.
fn Assert_Gone_From_Current_But_Kept(graph: &KnowledgeGraph, concept: &Versioned<Concept>)
{
    let identity = concept.Value().Identity();

    assert!(
        !graph.Current().Concepts().iter().any(|held| return held.Identity() == identity),
        "a merge loser is still counted as a current concept"
    );
    assert!(
        graph.Every_Version().Concepts().iter().any(|held| return held.Value().Identity() == identity),
        "and it is kept, because D17 says destruction requires evidence and a merge is not it"
    );
}

// ---- D-008's first requirement: liveness is one expression ----

/// The claim is structural, so the test is structural: it reads this crate's own source and
/// counts the definitions of the liveness rule.
///
/// # Why this is not a comment
///
/// The prototype had the rule in four places — a Postgres global query filter, an in-memory
/// repository, a JSON repository and a partial unique index — and the providers came to
/// disagree about which concepts exist.
///
/// Its migration history is the proof. `IX_Concepts_CanonicalName_Unique_Active` was created
/// filtering only on `Status <> 'Deprecated'` and corrected three weeks later to also filter
/// on `ValidUntil IS NULL`. **For three weeks the index enforced half the rule while the code
/// documented all of it, and nothing failed**, because a half-rule is a weaker constraint and
/// weaker constraints raise no errors. Nothing could have caught that except something
/// counting the copies.
#[test]
fn Test_The_Crate_Should_Define_Liveness_Exactly_Once()
{
    let mut definitions: Vec<String> = Vec::new();

    for (path, source) in Crate_Sources()
    {
        for name in Liveness_Definitions(&source)
        {
            definitions.push(format!("{path}: {name}"));
        }
    }

    assert_eq!(
        definitions.len(),
        1,
        "liveness must be defined once. A second definition is a second copy of the rule, \
         and a copy drifts silently: {definitions:?}"
    );
    assert!(
        definitions
            .first()
            .is_some_and(|found| return found.contains("standing.rs")),
        "the one definition should be Standing's: {definitions:?}"
    );
}

/// Every `.rs` file in this crate's `src`, with its file name.
fn Crate_Sources() -> Vec<(String, String)>
{
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let entries = std::fs::read_dir(&directory).expect("the crate has a src directory");

    let mut sources = Vec::new();
    for entry in entries
    {
        let path = entry.expect("a readable directory entry").path();
        if path.extension().is_some_and(|extension| return extension == "rs")
        {
            let name = path
                .file_name()
                .and_then(|name| return name.to_str())
                .unwrap_or_default()
                .to_owned();
            sources.push((name, std::fs::read_to_string(&path).expect("a readable source")));
        }
    }

    assert!(!sources.is_empty(), "no sources were scanned, so this test proves nothing");
    return sources;
}

/// Every function in `source` that defines the liveness rule.
///
/// A definition is a `fn Is_Current`. A *use* — calling it — is not, which is the distinction
/// that makes this count the copies of the rule rather than the places that respect it.
fn Liveness_Definitions(source: &str) -> Vec<String>
{
    let collapsed = source.split_whitespace().collect::<Vec<&str>>().join(" ");

    let mut found = Vec::new();
    for declaration in collapsed.split("fn ").skip(1)
    {
        let Some(name) = declaration.split('(').next()
        else
        {
            continue;
        };
        if name.trim() == "Is_Current"
        {
            found.push(name.trim().to_owned());
        }
    }

    return found;
}

// ---- D19-B: the two reads are different worlds, and neither is reachable by forgetting ----

#[test]
fn Test_A_Superseded_Concept_Should_Be_Absent_From_Current_And_Present_In_Every_Version()
{
    let names = ["entropy", "thermodynamic entropy"];
    let entropy = Asserted(names[0]);
    let successor = Asserted(names[1]);

    let graph = KnowledgeGraph::Empty()
        .With_Concept(entropy.clone())
        .With_Concept(successor.clone())
        .With_Concept(entropy.Closed(Standing::Superseded {
            by: successor.Value().Identity(),
            because: "the two names denote one concept".to_owned(),
        }));

    Assert_Gone_From_Current_But_Kept(&graph, &entropy);
    assert_eq!(graph.Current().Concepts().len(), 1);
    assert_eq!(graph.Every_Version().Concepts().len(), names.len());
}

#[test]
fn Test_The_Merge_Log_Should_Be_Answerable_From_The_All_Versions_Read()
{
    // merge-audit resolved the merge log's identifiers against a view that excluded losers,
    // resolved none, printed "nothing has been merged away" and exited 0. Every merge on
    // record could have been wrong and the gate would have passed.
    let loser = Asserted("C");
    let keeper = Asserted("C++");
    let graph = KnowledgeGraph::Empty()
        .With_Concept(keeper.clone())
        .With_Concept(loser.Closed(Standing::Superseded {
            by: keeper.Value().Identity(),
            because: "the two names denote one concept".to_owned(),
        }));

    let losers = graph.Every_Version().Merge_Losers();

    assert_eq!(losers.len(), 1, "the audit must be able to see what was merged away");
    assert_eq!(
        losers.first().and_then(|held| return held.Standing().Superseded_By()),
        Some(keeper.Value().Identity())
    );
    assert!(graph.Current().Concepts().len() == 1, "and the loser is not current");
}

#[test]
fn Test_A_Retired_Concept_Should_Not_Claim_A_Successor()
{
    // Not current and merged-into-something are different facts, and a retired concept has
    // the first without the second.
    let retired = Asserted("phlogiston").Closed(Standing::Retired {
            because: "the concept was withdrawn by its source".to_owned(),
        });

    assert!(!retired.Standing().Is_Current());
    assert_eq!(retired.Standing().Superseded_By(), None);
}

// ---- A version is a value: publishing leaves the previous version valid ----

#[test]
fn Test_Publishing_Should_Leave_The_Previous_Version_Queryable()
{
    let entropy = Asserted("entropy");
    let before = KnowledgeGraph::Empty().With_Concept(entropy.clone());

    let after = before.With_Concept(entropy.Closed(Standing::Retired {
            because: "the concept was withdrawn by its source".to_owned(),
        }));

    assert!(
        !before.Current().Concepts().is_empty(),
        "the earlier graph is a value and is unchanged; this is what makes a temporal read a \
         value you kept rather than a query that opts out of a filter"
    );
    assert!(after.Current().Concepts().is_empty());
}

#[test]
fn Test_The_Two_Reads_Should_Not_Be_One_Type_With_A_Flag()
{
    // Asserted the only way a type-level absence can be: by fixing the call sites. Neither
    // read takes an argument selecting a world, so neither can be pointed at the other by
    // passing the wrong value, and a caller that needs merge losers has had to name
    // Every_Version to get one.
    let graph = KnowledgeGraph::Empty().With_Concept(Asserted("entropy"));

    assert_eq!(graph.Current().Concepts().len(), 1);
    assert_eq!(graph.Every_Version().Concepts().len(), 1);
}

// ---- determinism is enforced at the boundary, not obtained from a hasher ----

/// The addresses every concept a graph holds, in the order the every-version read returns them.
///
/// The graph is built from `names` in the order given, which is the variable under test: two
/// calls that differed in anything else could pass while the listing stayed order-dependent.
fn Listing_Of(names: &[&str]) -> Vec<String>
{
    let mut graph = KnowledgeGraph::Empty();
    for name in names
    {
        graph = graph.With_Concept(Asserted(name));
    }

    let listing: Vec<String> = graph
        .Every_Version()
        .Concepts()
        .iter()
        .map(|held| return held.Value().Identity().Render())
        .collect();

    return listing;
}

#[test]
fn Test_The_Listing_Should_Not_Depend_On_The_Order_Concepts_Arrived_In()
{
    let names = ["entropy", "enthalpy", "free energy", "temperature"];
    let mut reversed = names;
    reversed.reverse();
    let forwards = Listing_Of(&names);
    let backwards = Listing_Of(&reversed);

    assert_eq!(
        forwards, backwards,
        "the map is a hash trie and two graphs carry two hashers, so an unsorted listing \
         would differ here; determinism is enforced by the sort rather than by the hasher"
    );
    assert_eq!(forwards.len(), names.len());
}

// ---- KWB-44: the detector is shown to detect ----

/// A detector that always returned nothing would report one definition of liveness forever.
///
/// The guard above expects exactly one, and this crate has exactly one, so a passing guard is
/// consistent with a parser that has stopped parsing — it would report zero, and zero is not
/// one, so that case would in fact fail. What it could not catch is a parser that stopped
/// finding *second* definitions, which is the failure that matters: these cases show it finds
/// them.
#[test]
fn Test_The_Liveness_Detector_Should_Find_Every_Definition()
{
    assert_eq!(Liveness_Definitions("pub fn Is_Current(&self) -> bool { }"), ["Is_Current"]);
    assert_eq!(
        Liveness_Definitions("fn Is_Current(a: &A) -> bool {} pub fn Is_Current(&self) -> bool {}"),
        ["Is_Current", "Is_Current"],
        "a second definition is the whole defect, so both must be found"
    );
    assert_eq!(
        Liveness_Definitions("pub fn Is_Current(\n    &self,\n) -> bool { }"),
        ["Is_Current"],
        "a signature broken across lines must not slip past"
    );
}

#[test]
fn Test_The_Liveness_Detector_Should_Not_Report_A_Use_As_A_Definition()
{
    // The distinction that makes this count copies of the rule rather than places that respect
    // it: calling Is_Current is not defining it.
    let quiet = [
        "if standing.Is_Current() { }",
        "return held.Standing().Is_Current();",
        "/// See Is_Current for the rule.",
        "fn Is_Currently_Held(&self) -> bool { }",
    ];

    for source in quiet
    {
        assert!(
            Liveness_Definitions(source).is_empty(),
            "counted a use as a definition in: {source:?}"
        );
    }
}
