//! The current read: what the graph is now, and what composing the liveness rule excludes.
//!
//! # Why this file is here rather than in `src/tests.rs`
//!
//! This crate's tests used to be one file, `src/tests.rs`, and its stem names no source file. So
//! every function below was exercised and addressed by nothing: the unit `check-test-coverage`
//! reads is the test file's own stem, and `src/versioning/current_knowledge.rs` is covered by a file
//! called `current_knowledge.rs` and by nothing else. A failure in `Claims` arrived under the name
//! of a test about replay.
//!
//! # Why the read is most of the rule
//!
//! `Is_Current` on a value is the simple half. The half that has teeth is that a claim's currency is
//! its own standing **and its concept's**, and an assertion's is its own and its claim's — the same
//! rule applied twice, rather than a second rule that could disagree. A claim about a retired
//! concept that stayed current would keep accumulating citations, and nothing in this workspace
//! would have said the citations were attached to retired knowledge.
//!
//! `Of` is the reader's constructor, and it is `pub(crate)`: a reader assembled against a graph
//! other than the one it was asked for would be a second answer to a question already answered. Its
//! test sits beside it in `src/versioning/current_knowledge.rs`.

use kwb_domain::Assertion;
use kwb_domain::Claim;
use kwb_domain::Concept;
use kwb_domain::KnowledgeGraph;
use kwb_domain::Scope;
use kwb_domain::Standing;
use kwb_domain::Versioned;

/// Why the fixtures below close something.
const BECAUSE: &str = "the concept was withdrawn by its author";

/// The concept every fixture here is about.
fn Entropy() -> Concept
{
    return Concept::Named("entropy");
}

/// The claim every fixture here is about.
fn Its_Claim(concept: &Concept) -> Claim
{
    return Claim::About(concept, "It is non-decreasing in an isolated system.");
}

/// A source's assertion of that claim.
fn Callens_Assertion(claim: &Claim) -> Assertion
{
    return Assertion::By(
        "Callen 1985",
        claim,
        Scope::Named("physical theory").expect("a named scope"),
    );
}

/// A graph holding a concept, the claim about it and an assertion of that claim, and that concept.
///
/// One value with named fields rather than a pair, because a pair says nothing about which of its
/// two positions is the graph and which is the concept inside it.
struct PublishedKnowledge
{
    /// The graph the three publications went into.
    graph: KnowledgeGraph,

    /// The concept the graph holds, which the claim is about.
    concept: Concept,
}

/// A graph holding a concept, the claim about it, and an assertion of that claim -- the three
/// things the current read has to reach before there is anything to exclude.
fn Published_Knowledge() -> PublishedKnowledge
{
    let concept = Entropy();
    let claim = Its_Claim(&concept);
    let assertion = Callens_Assertion(&claim);

    let graph = KnowledgeGraph::Empty()
        .With_Concept(Versioned::Asserted(concept.clone()))
        .With_Claim(Versioned::Asserted(claim))
        .With_Assertion(Versioned::Asserted(assertion));

    return PublishedKnowledge { graph, concept };
}

/// The fixture graph once its concept was merged into a successor.
///
/// A merge is two publications rather than one -- the successor, and the loser closed against the
/// successor's own address -- and naming the pair is what lets the test below assert the one thing
/// it is about: that the current read does not hand the loser back.
fn A_Graph_Whose_Concept_Was_Merged_Into_A_Successor() -> KnowledgeGraph
{
    let PublishedKnowledge { graph, concept } = Published_Knowledge();
    let successor = Concept::Named("C++");

    return graph
        .With_Concept(Versioned::Asserted(successor.clone()))
        .With_Concept(
            Versioned::Asserted(concept).Closed(Standing::Superseded {
                by: successor.Identity(),
                because: BECAUSE.to_owned(),
            }),
        );
}

/// The canonical names the current read answers, in the order it answers them.
///
/// A named read rather than a chain written where it is compared, because what the test below
/// contrasts is a list of names, and the walk that produced them is not part of that contrast.
fn Current_Concept_Names(graph: &KnowledgeGraph) -> Vec<&str>
{
    return graph
        .Current()
        .Concepts()
        .into_iter()
        .map(|held| return held.Canonical_Name())
        .collect();
}

#[test]
fn Test_Concepts_Should_Not_Reach_A_Concept_That_Was_Closed()
{
    // A concept retired on its own, and one merged into a successor, are both not current -- and
    // both stay in the graph. The `Every_Version` half of that is asserted in
    // `tests/every_version.rs`; what is asserted here is that closing one never leaves it in the
    // answer a caller reads by default.
    let after = A_Graph_Whose_Concept_Was_Merged_Into_A_Successor();

    assert_eq!(
        Current_Concept_Names(&after),
        ["C++"],
        "a concept that was closed is still handed out as current, so a merge loser keeps being \
         read as the thing it was merged away from"
    );
}

/// The fixture graph once the concept it is about was retired.
///
/// Retiring is the other closure a concept has, and it names no successor: what makes the claim
/// below not current is the concept's standing alone, which is the half of the rule this test does
/// not share with the merge above.
fn A_Graph_Whose_Concept_Was_Retired(graph: &KnowledgeGraph, concept: Concept) -> KnowledgeGraph
{
    return graph.With_Concept(
        Versioned::Asserted(concept).Closed(Standing::Retired {
            because: BECAUSE.to_owned(),
        }),
    );
}

#[test]
fn Test_Claims_Should_Not_Reach_A_Claim_About_A_Concept_That_Is_Not_Current()
{
    let PublishedKnowledge { graph, concept } = Published_Knowledge();

    assert_eq!(
        graph.Current().Claims().len(),
        1,
        "a published claim about a published concept is not current, so nothing about the graph \
         would ever be readable and this test would pass on a read that answers nothing at all"
    );

    let after = A_Graph_Whose_Concept_Was_Retired(&graph, concept);

    assert!(
        after.Current().Claims().is_empty(),
        "a claim about a concept that is not current is still current, so retired knowledge keeps \
         accumulating citations and nothing says the citations are attached to it"
    );
    assert_eq!(
        after.Every_Version().Claims().len(),
        1,
        "the claim was removed rather than excluded, which is the destruction D17 refuses"
    );
}

/// The fixture graph once the claim about its concept was retired.
///
/// The claim's own standing is what changed here rather than its concept's, so the assertion below
/// is asserting the second application of the rule rather than the first one again.
fn A_Graph_Whose_Claim_Was_Retired(graph: &KnowledgeGraph, concept: &Concept) -> KnowledgeGraph
{
    return graph.With_Claim(
        Versioned::Asserted(Its_Claim(concept)).Closed(Standing::Retired {
            because: BECAUSE.to_owned(),
        }),
    );
}

#[test]
fn Test_Assertions_Should_Not_Reach_An_Assertion_Of_A_Claim_That_Is_Not_Current()
{
    let PublishedKnowledge { graph, concept } = Published_Knowledge();

    assert_eq!(
        graph.Current().Assertions().len(),
        1,
        "a published assertion of a current claim is not current, so this test would pass on a read \
         that answers nothing at all"
    );

    let after = A_Graph_Whose_Claim_Was_Retired(&graph, &concept);

    assert!(
        after.Current().Assertions().is_empty(),
        "an assertion of a claim that is not current is still current, so a citation outlives the \
         claim it cites"
    );
    assert_eq!(
        after.Every_Version().Assertions().len(),
        1,
        "the assertion was removed rather than excluded, which is the destruction D17 refuses"
    );
}
