//! The properties the query surface exists to have, exercised from outside the crate.

use kwb_domain::{
    Assertion, Claim, Concept, KnowledgeGraph, Scope, Standing, Versioned,
};
use kwb_retrieval::{CurrentQueries, HistoricalQueries};
use kwb_source_guards::Crate_Sources;
use kwb_source_guards::Mutating_Public_Methods;

/// A graph holding one concept, one claim about it, and one assertion of that claim.
///
/// Named rather than returned as a tuple because the callers want different subsets of the three,
/// and a tuple communicates by position alone: a test that wants only the graph would write
/// `let (graph, _, _)`, which tells a reader nothing about what was dropped. A named member says
/// which it is, and a test that wants none of it simply does not mention it.
struct Corpus
{
    /// The graph, holding the concept, the claim and the assertion.
    graph: KnowledgeGraph,

    /// The concept the claim is about, which the tests below close or supersede.
    entropy: Concept,

    /// The claim, whose identity the keyword search is expected to return.
    claim: Claim,
}

/// The corpus above, assembled.
fn Corpus() -> Corpus
{
    let entropy = Concept::Named("entropy");
    let claim = Claim::About(&entropy, "It is non-decreasing in an isolated system.");
    let assertion = Assertion::By("Callen 1985", &claim, Scope::Named("physical theory").expect("a named scope"));

    let graph = KnowledgeGraph::Empty()
        .With_Concept(Versioned::Asserted(entropy.clone()))
        .With_Claim(Versioned::Asserted(claim.clone()))
        .With_Assertion(Versioned::Asserted(assertion));

    return Corpus { graph, entropy, claim };
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

    for source in Crate_Sources(env!("CARGO_MANIFEST_DIR"))
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

// ---- keyword ----

#[test]
fn Test_A_Claim_Should_Be_Found_By_Words_It_Contains()
{
    let corpus = Corpus();

    let found = CurrentQueries::Over(&corpus.graph).Claims_Matching("isolated system");

    assert_eq!(found.len(), 1);
    assert_eq!(found.first().map(|found| return found.Identity()), Some(corpus.claim.Identity()));
}

#[test]
fn Test_Every_Word_Should_Have_To_Match()
{
    let corpus = Corpus();

    assert!(
        CurrentQueries::Over(&corpus.graph)
            .Claims_Matching("isolated unicorn")
            .is_empty(),
        "one word matching is not a match"
    );
}

#[test]
fn Test_Word_Order_And_Spacing_Should_Not_Decide_A_Match()
{
    let corpus = Corpus();
    let queries = CurrentQueries::Over(&corpus.graph);

    assert_eq!(queries.Claims_Matching("system isolated").len(), 1);
    assert_eq!(queries.Claims_Matching("  isolated   system  ").len(), 1);
}

#[test]
fn Test_Case_Should_Be_Significant_Here_As_Everywhere_Else()
{
    let corpus = Corpus();

    assert!(
        CurrentQueries::Over(&corpus.graph).Claims_Matching("Isolated").is_empty(),
        "folding case in one place and not another is how two searches come to disagree"
    );
}

#[test]
fn Test_An_Empty_Query_Should_Match_Nothing_Rather_Than_Everything()
{
    let corpus = Corpus();
    let queries = CurrentQueries::Over(&corpus.graph);

    assert!(queries.Claims_Matching("").is_empty());
    assert!(queries.Claims_Matching("   ").is_empty());
    assert_eq!(queries.Concept_Count(), 1, "though the corpus is not empty");
}

// ---- neighbourhood ----

#[test]
fn Test_A_Neighbourhood_Should_Reach_A_Concepts_Claims_And_Their_Assertions()
{
    let corpus = Corpus();

    let neighbourhood = CurrentQueries::Over(&corpus.graph)
        .Neighbourhood_Of(corpus.entropy.Identity())
        .expect("the concept is current");

    assert_eq!(neighbourhood.concept.Identity(), corpus.entropy.Identity());
    assert_eq!(neighbourhood.claims.len(), 1);
    assert_eq!(neighbourhood.assertions.len(), 1);
    assert_eq!(
        neighbourhood.assertions.first().map(|found| return found.Claim()),
        Some(corpus.claim.Identity())
    );
}

#[test]
fn Test_A_Neighbourhood_Should_Not_Reach_Another_Concepts_Claims()
{
    let corpus = Corpus();
    let other = Concept::Named("enthalpy");
    let unrelated = Claim::About(&other, "It is a thermodynamic potential.");
    let graph = corpus.graph
        .With_Concept(Versioned::Asserted(other))
        .With_Claim(Versioned::Asserted(unrelated));

    let neighbourhood = CurrentQueries::Over(&graph)
        .Neighbourhood_Of(corpus.entropy.Identity())
        .expect("the concept is current");

    assert_eq!(neighbourhood.claims.len(), 1, "a neighbourhood reached a claim about something else");
    assert_eq!(CurrentQueries::Over(&graph).Concept_Count(), CONCEPTS_IN_GRAPH);
}

// ---- D19-B: the two worlds answer differently, and the caller names which ----

#[test]
fn Test_A_Merge_Loser_Should_Be_Invisible_To_Current_And_Visible_To_Historical()
{
    let corpus = Corpus();
    let keeper = Concept::Named("thermodynamic entropy");
    let graph = corpus.graph
        .With_Concept(Versioned::Asserted(keeper.clone()))
        .With_Concept(Versioned::Asserted(corpus.entropy.clone()).Closed(Standing::Superseded {
            by: keeper.Identity(),
            because: "the two names denote one concept".to_owned(),
        }));

    assert!(
        CurrentQueries::Over(&graph)
            .Neighbourhood_Of(corpus.entropy.Identity())
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
    let corpus = Corpus();

    let retired = corpus.graph.With_Concept(
        Versioned::Asserted(corpus.entropy).Closed(Standing::Retired {
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
    for source in NON_MUTATING_SOURCES
    {
        assert!(
            Mutating_Public_Methods(source).is_empty(),
            "reported a mutating method in: {source:?}"
        );
    }
}

/// Sources that must produce no mutating method: a shared reader, a private writer, a doc comment
/// that reads like a signature, a const constructor, and no declaration at all.
///
/// Declared outside the test, and named, for the reason the note above gives about the two copies:
/// this table is the same one `kwb-store`'s `tests/one_door.rs` runs, and the pair is only worth
/// running while the two can be read side by side. A case added to one is visible as a diff against
/// a named thing rather than as an edit buried in a test body.
const NON_MUTATING_SOURCES: &[&str] = &[
    "pub fn Read(&self) -> bool { }",
    "fn Private(&mut self) { }",
    "/// A doc comment mentioning &mut self, which is not a signature.",
    "pub const fn Empty() -> Self { }",
    "",
];
