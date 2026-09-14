//! The properties the query surface exists to have, exercised from outside the crate.

use kwb_domain::{
    Assertion, Claim, Concept, KnowledgeGraph, Scope, Standing, Versioned,
};
use kwb_retrieval::{CurrentQueries, HistoricalQueries};

/// A graph holding one concept, one claim about it, and one assertion of that claim.
fn Corpus() -> (KnowledgeGraph, Concept, Claim)
{
    let entropy = Concept::Named("entropy");
    let claim = Claim::About(&entropy, "It is non-decreasing in an isolated system.");
    let assertion = Assertion::By("Callen 1985", &claim, Scope::Named("physical theory").expect("a named scope"));

    let graph = KnowledgeGraph::Empty()
        .With_Concept(Versioned::Asserted(entropy.clone()))
        .With_Claim(Versioned::Asserted(claim.clone()))
        .With_Assertion(Versioned::Asserted(assertion));

    return (graph, entropy, claim);
}

/// How many concepts the graphs below hold: the one `Corpus` supplies, and the one more that
/// each of the two tests that reach for this writes.
///
/// `With_Concept` re-versions a concept the graph already holds rather than appending a second
/// copy of it, which is why the merge test — which writes `entropy` a second time in order to
/// close it — also holds two.
const CONCEPTS_IN_GRAPH: usize = 2;

// ---- D10: mutation is excluded architecturally, not by convention ----

/// The claim is structural, so the test is structural: it reads this crate's own source and
/// counts the public methods that can change anything.
///
/// # Why this is not a comment
///
/// `KWB-6` puts it exactly: *the type an MCP tool handler is built against should not have a
/// mutating method to call by mistake.* A doc comment saying so is checked by whoever reads
/// it. This is checked by whoever runs the tests, at every future revision, which is the only
/// form of "architecturally excluded" that survives the person who wrote it.
#[test]
fn Test_The_Query_Surface_Should_Have_No_Write_Path()
{
    let mut mutators: Vec<String> = Vec::new();

    for source in Crate_Sources()
    {
        for name in Mutating_Public_Methods(&source)
        {
            mutators.push(name);
        }
    }

    assert!(
        mutators.is_empty(),
        "retrieval answers questions and admission is the only writer. These can change \
         something: {mutators:?}"
    );
}

/// Every `.rs` file in this crate's `src`.
fn Crate_Sources() -> Vec<String>
{
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
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

/// The name of every `pub fn` whose parameter list takes `&mut self`.
///
/// Whitespace is collapsed first, so a signature broken across lines cannot slip past, and the
/// parameter list is what is inspected, so a doc comment mentioning `&mut self` is not a
/// finding.
fn Mutating_Public_Methods(source: &str) -> Vec<String>
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
            found.push(signature.split('(').next().unwrap_or_default().trim().to_owned());
        }
    }

    return found;
}

// ---- keyword ----

#[test]
fn Test_A_Claim_Should_Be_Found_By_Words_It_Contains()
{
    let (graph, _, claim) = Corpus();

    let found = CurrentQueries::Over(&graph).Claims_Matching("isolated system");

    assert_eq!(found.len(), 1);
    assert_eq!(found.first().map(|found| return found.Identity()), Some(claim.Identity()));
}

#[test]
fn Test_Every_Word_Should_Have_To_Match()
{
    let (graph, _, _) = Corpus();

    assert!(
        CurrentQueries::Over(&graph)
            .Claims_Matching("isolated unicorn")
            .is_empty(),
        "one word matching is not a match"
    );
}

#[test]
fn Test_Word_Order_And_Spacing_Should_Not_Decide_A_Match()
{
    let (graph, _, _) = Corpus();
    let queries = CurrentQueries::Over(&graph);

    assert_eq!(queries.Claims_Matching("system isolated").len(), 1);
    assert_eq!(queries.Claims_Matching("  isolated   system  ").len(), 1);
}

#[test]
fn Test_Case_Should_Be_Significant_Here_As_Everywhere_Else()
{
    let (graph, _, _) = Corpus();

    assert!(
        CurrentQueries::Over(&graph).Claims_Matching("Isolated").is_empty(),
        "folding case in one place and not another is how two searches come to disagree"
    );
}

#[test]
fn Test_An_Empty_Query_Should_Match_Nothing_Rather_Than_Everything()
{
    let (graph, _, _) = Corpus();
    let queries = CurrentQueries::Over(&graph);

    assert!(queries.Claims_Matching("").is_empty());
    assert!(queries.Claims_Matching("   ").is_empty());
    assert_eq!(queries.Concept_Count(), 1, "though the corpus is not empty");
}

// ---- neighbourhood ----

#[test]
fn Test_A_Neighbourhood_Should_Reach_A_Concepts_Claims_And_Their_Assertions()
{
    let (graph, entropy, claim) = Corpus();

    let neighbourhood = CurrentQueries::Over(&graph)
        .Neighbourhood_Of(entropy.Identity())
        .expect("the concept is current");

    assert_eq!(neighbourhood.concept.Identity(), entropy.Identity());
    assert_eq!(neighbourhood.claims.len(), 1);
    assert_eq!(neighbourhood.assertions.len(), 1);
    assert_eq!(
        neighbourhood.assertions.first().map(|found| return found.Claim()),
        Some(claim.Identity())
    );
}

#[test]
fn Test_A_Neighbourhood_Should_Not_Reach_Another_Concepts_Claims()
{
    let (graph, entropy, _) = Corpus();
    let other = Concept::Named("enthalpy");
    let unrelated = Claim::About(&other, "It is a thermodynamic potential.");
    let graph = graph
        .With_Concept(Versioned::Asserted(other))
        .With_Claim(Versioned::Asserted(unrelated));

    let neighbourhood = CurrentQueries::Over(&graph)
        .Neighbourhood_Of(entropy.Identity())
        .expect("the concept is current");

    assert_eq!(neighbourhood.claims.len(), 1, "a neighbourhood reached a claim about something else");
    assert_eq!(CurrentQueries::Over(&graph).Concept_Count(), CONCEPTS_IN_GRAPH);
}

// ---- D19-B: the two worlds answer differently, and the caller names which ----

#[test]
fn Test_A_Merge_Loser_Should_Be_Invisible_To_Current_And_Visible_To_Historical()
{
    let (graph, entropy, _) = Corpus();
    let keeper = Concept::Named("thermodynamic entropy");
    let graph = graph
        .With_Concept(Versioned::Asserted(keeper.clone()))
        .With_Concept(Versioned::Asserted(entropy.clone()).Closed(Standing::Superseded {
            by: keeper.Identity(),
            because: "the two names denote one concept".to_owned(),
        }));

    assert!(
        CurrentQueries::Over(&graph)
            .Neighbourhood_Of(entropy.Identity())
            .is_none(),
        "a merge loser is not a current concept"
    );
    assert_eq!(
        HistoricalQueries::Over(&graph).Merge_Losers().len(),
        1,
        "this is the question merge-audit asked of the wrong world, resolved none of, and \
         exited 0 on"
    );
    assert_eq!(HistoricalQueries::Over(&graph).Concept_Count(), CONCEPTS_IN_GRAPH);
}

#[test]
fn Test_A_Claim_Of_A_Retired_Concept_Should_Leave_The_Current_Keyword_Index()
{
    let (graph, entropy, _) = Corpus();

    let retired = graph.With_Concept(
        Versioned::Asserted(entropy).Closed(Standing::Retired {
            because: "the concept was withdrawn by its source".to_owned(),
        }),
    );

    assert!(
        CurrentQueries::Over(&retired)
            .Claims_Matching("isolated system")
            .is_empty(),
        "a claim about a retired concept is not current knowledge, and one liveness rule \
         says so for both the graph and the search"
    );
    assert_eq!(
        HistoricalQueries::Over(&retired)
            .Claims_Matching("isolated system")
            .len(),
        1,
        "and it is still findable by whoever says which world they are asking"
    );
}

// ---- KWB-44: the detector is shown to detect, and the two copies are compared ----

/// The same cases `kwb-store`'s `tests/one_door.rs` runs, deliberately.
///
/// This crate and that one each carry their own copy of this detector, and the two bodies were
/// found to differ. Extracting them into one crate was weighed and not done: a shared
/// test-support crate to hold twenty lines is a real cost too, and the thing that actually goes
/// wrong with a copy is that it drifts unnoticed. Running both against the same cases makes
/// drift a failure, which is what was wanted — if a third copy ever appears, extracting becomes
/// the cheaper answer and this note is the record of where that line is.
#[test]
fn Test_The_Mutating_Method_Detector_Should_Detect()
{
    assert_eq!(Mutating_Public_Methods("pub fn Write(&mut self) -> bool { }"), ["Write"]);
    assert_eq!(
        Mutating_Public_Methods("pub fn Insert(\n    &mut self,\n    value: usize,\n) { }"),
        ["Insert"],
        "a signature broken across lines must not slip past; whitespace is collapsed first"
    );
    assert_eq!(
        Mutating_Public_Methods("pub fn A(&mut self){} pub fn B(&mut self){}"),
        ["A", "B"],
        "it must find every one, not stop at the first"
    );
}

#[test]
fn Test_The_Mutating_Method_Detector_Should_Not_Report_What_Is_Not_One()
{
    let quiet = [
        "pub fn Read(&self) -> bool { }",
        "fn Private(&mut self) { }",
        "/// A doc comment mentioning &mut self, which is not a signature.",
        "pub const fn Empty() -> Self { }",
        "",
    ];

    for source in quiet
    {
        assert!(
            Mutating_Public_Methods(source).is_empty(),
            "reported a mutating method in: {source:?}"
        );
    }
}
